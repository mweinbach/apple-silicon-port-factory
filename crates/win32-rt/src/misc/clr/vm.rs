//! Pure-Rust ECMA-335 Managed Heap, Type System, CIL Bytecode Interpreter,
//! BCL Intrinsics, P/Invoke Dispatcher, and Mono Internal-Call Bridge.

mod bcl;
mod eh;
mod heap;
mod interp;
mod loader;
mod pinvoke;
mod resolve;
mod value;

#[cfg(test)]
mod tests;

pub use heap::{
    HeapObjKind, ManagedHeap, MONO_ARRAY_DATA_OFFSET, MONO_OBJECT_HEADER_SIZE,
    MONO_STRING_CHARS_OFFSET, MONO_STRING_LEN_OFFSET,
};
pub use interp::execute_method;
pub use loader::{match_method_desc_in_class, parse_mono_method_desc};
pub use value::{default_val_for_sig, read_sig_from_ptr, write_sig_to_ptr, CilVal};

use super::metadata::{table, AssemblyImage, MethodBody, MethodSig, SigType};
use eh::LocalInit;
use resolve::{CachedCallEntry, CachedNewObjEntry, ResolvedField, ResolvedTarget};
use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use value::{write_sig_to_stack_buf, StorageKind};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum VmError {
    FileNotFound(String),
    BadImage(String),
    MethodNotFound(String),
    Unsupported(String),
    ManagedException { exc_obj: usize, message: String },
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound(s) => write!(f, "file not found: {s}"),
            Self::BadImage(s) => write!(f, "bad image: {s}"),
            Self::MethodNotFound(s) => write!(f, "method not found: {s}"),
            Self::Unsupported(s) => write!(f, "unsupported: {s}"),
            Self::ManagedException { message, .. } => write!(f, "managed exception: {message}"),
        }
    }
}

// ---------------------------------------------------------------------------
// ABI-compatible Mono structs (64-bit layout)
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[repr(C)]
pub struct MonoVTable {
    pub klass: *mut MonoClass,
    pub domain: *mut MonoDomain,
    pub class_id: usize,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoDomain {
    pub id: u32,
    pub name_ptr: *const c_char,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoAssembly {
    pub asm_id: usize,
    pub image: *mut MonoImage,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoImage {
    pub asm_id: usize,
    pub name_ptr: *const c_char,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoClass {
    pub class_id: usize,
    pub asm_id: usize,
    pub typedef_rid: u32,
    pub name_ptr: *const c_char,
    pub namespace_ptr: *const c_char,
    pub instance_size: usize,
    pub elem_size: usize,
    pub is_value_type: bool,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoMethod {
    pub method_id: usize,
    pub asm_id: usize,
    pub class_id: usize,
    pub method_rid: u32,
    pub name_ptr: *const c_char,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoClassField {
    pub field_id: usize,
    pub class_id: usize,
    pub offset: usize,
    pub size: usize,
    pub is_static: bool,
    pub name_ptr: *const c_char,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoProperty {
    pub prop_id: usize,
    pub class_id: usize,
    pub getter: *mut MonoMethod,
    pub setter: *mut MonoMethod,
    pub name_ptr: *const c_char,
}

#[allow(dead_code)]
#[repr(C)]
pub struct MonoThread {
    pub os_tid: u64,
    pub domain: *mut MonoDomain,
}

#[derive(Clone, Debug)]
pub struct MonoMethodDesc {
    pub namespace: Option<String>,
    pub class_name: Option<String>,
    pub method_name: String,
    pub param_count: Option<usize>,
    pub include_namespace: bool,
}

// ---------------------------------------------------------------------------
// Metadata Models
// ---------------------------------------------------------------------------

pub struct LoadedAssembly {
    pub id: usize,
    pub name: String,
    pub path: Option<PathBuf>,
    pub image: Option<Arc<AssemblyImage>>,
    pub mono_assembly: usize,
    pub mono_image: usize,
    /// Indexed by `typedef_rid - 1`
    pub class_ids: Vec<usize>,
    /// Indexed by `method_rid - 1`
    pub method_ids: Vec<usize>,
    /// Indexed by `field_rid - 1`
    pub field_ids: Vec<usize>,

    // Per-assembly O(1) lookup indices & resolved-token caches
    pub(crate) classes_by_short_name: HashMap<String, Vec<usize>>,
    pub(crate) field_rvas: HashMap<u32, u32>,
    pub(crate) user_strings: HashMap<u32, usize>,
    pub(crate) type_cache: HashMap<u32, usize>,
    pub(crate) field_cache: HashMap<u32, ResolvedField>,
    pub(crate) call_cache: HashMap<u32, CachedCallEntry>,
    pub(crate) newobj_cache: HashMap<u32, CachedNewObjEntry>,
    pub(crate) standalone_sig_cache: HashMap<u32, Arc<MethodSig>>,
    pub(crate) vdispatch_cache: HashMap<(u32, usize), usize>,
}

pub struct ClassInfo {
    pub namespace: String,
    pub name: String,
    pub parent_class_id: Option<usize>,
    pub is_value_type: bool,
    pub is_delegate: bool,
    pub is_exception: bool,
    pub primitive_sig: Option<SigType>,
    pub elem_size: usize,
    pub instance_payload_size: usize,
    pub fields: Vec<usize>,
    pub methods: Vec<usize>,
    pub properties: Vec<usize>,
    pub(crate) cctor_method_id: Option<usize>,
    pub cctor_ran: bool,
    pub mono_class: usize,
    pub mono_vtable: usize,
}

impl ClassInfo {
    pub fn full_name(&self) -> String {
        if self.namespace.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.namespace, self.name)
        }
    }
}

pub struct FieldInfo {
    pub class_id: usize,
    pub name: String,
    pub sig: SigType,
    pub(crate) kind: StorageKind,
    pub is_static: bool,
    pub struct_offset: usize,
    pub size: usize,
    pub static_value: CilVal,
    pub mono_field: usize,
}

pub struct MethodInfo {
    pub asm_id: usize,
    pub class_id: usize,
    pub name: String,
    pub flags: u16,
    pub impl_flags: u16,
    pub sig: MethodSig,
    pub(crate) argc: usize,
    pub(crate) is_virtual: bool,
    pub(crate) is_void_ret: bool,
    pub body: Option<Arc<MethodBody>>,
    pub locals_sig: Vec<SigType>,
    pub(crate) local_init_plan: Option<Arc<[LocalInit]>>,
    pub pinvoke: Option<(String, String, u16)>,
    pub(crate) pinvoke_fn_ptr: Option<usize>,
    pub(crate) cached_target: Option<ResolvedTarget>,
    pub mono_method: usize,
}

pub struct PropertyInfo {
    pub name: String,
    pub getter_method_id: Option<usize>,
    pub setter_method_id: Option<usize>,
    pub mono_prop: usize,
}

#[derive(Clone, Copy)]
pub struct CoreClasses {
    pub object: usize,
    pub string: usize,
    pub array: usize,
    pub exception: usize,
    pub invalid_cast: usize,
    pub null_ref: usize,
    pub index_out_of_range: usize,
    pub div_by_zero: usize,
    pub overflow: usize,
    pub type_cls: usize,
    pub string_builder: usize,
    pub list_cls: usize,
    pub dict_cls: usize,
    pub void_cls: usize,
    pub boolean: usize,
    pub char_cls: usize,
    pub sbyte: usize,
    pub byte: usize,
    pub int16: usize,
    pub uint16: usize,
    pub int32: usize,
    pub uint32: usize,
    pub int64: usize,
    pub uint64: usize,
    pub single: usize,
    pub double: usize,
    pub intptr: usize,
    pub uintptr: usize,
}

// ---------------------------------------------------------------------------
// Global VM State
// ---------------------------------------------------------------------------

pub struct ClrVm {
    pub heap: ManagedHeap,
    pub assemblies: Vec<LoadedAssembly>,
    pub classes: Vec<ClassInfo>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub properties: Vec<PropertyInfo>,
    pub core: CoreClasses,
    pub(crate) corlib_class_count: usize,
    pub(crate) classes_by_short_name: HashMap<String, Vec<usize>>,
    empty_string_obj: usize,

    pub root_domain: usize,
    pub current_domain: usize,
    pub domains: HashMap<usize, u32>,
    pub domains_by_id: HashMap<u32, usize>,
    next_domain_id: u32,

    pub threads: HashMap<u64, usize>,
    pub assembly_by_ptr: HashMap<usize, usize>,
    pub image_by_ptr: HashMap<usize, usize>,
    pub class_by_ptr: HashMap<usize, usize>,
    pub vtable_by_ptr: HashMap<usize, usize>,
    pub method_by_ptr: HashMap<usize, usize>,
    pub field_by_ptr: HashMap<usize, usize>,
    pub prop_by_ptr: HashMap<usize, usize>,
    pub method_descs: HashMap<usize, MonoMethodDesc>,

    pub internal_calls: HashMap<String, usize>,
    pub search_dirs: Vec<PathBuf>,
    pub env_exit_code: i32,
    pub command_line_args: Vec<String>,

    /// Owned C strings & boxed handle structs so pointers never dangle.
    c_strings: Vec<CString>,
    owned_domains: Vec<Box<MonoDomain>>,
    owned_assemblies: Vec<Box<MonoAssembly>>,
    owned_images: Vec<Box<MonoImage>>,
    owned_classes: Vec<Box<MonoClass>>,
    owned_vtables: Vec<Box<MonoVTable>>,
    owned_methods: Vec<Box<MonoMethod>>,
    owned_fields: Vec<Box<MonoClassField>>,
    owned_props: Vec<Box<MonoProperty>>,
    owned_threads: Vec<Box<MonoThread>>,
}

unsafe impl Send for ClrVm {}

static GLOBAL_VM: OnceLock<Mutex<ClrVm>> = OnceLock::new();

pub fn lock_vm() -> MutexGuard<'static, ClrVm> {
    GLOBAL_VM
        .get_or_init(|| Mutex::new(ClrVm::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

impl ClrVm {
    fn new() -> Self {
        let mut vm = Self {
            heap: ManagedHeap::new(),
            assemblies: Vec::new(),
            classes: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            properties: Vec::new(),
            core: CoreClasses {
                object: 0,
                string: 0,
                array: 0,
                exception: 0,
                invalid_cast: 0,
                null_ref: 0,
                index_out_of_range: 0,
                div_by_zero: 0,
                overflow: 0,
                type_cls: 0,
                string_builder: 0,
                list_cls: 0,
                dict_cls: 0,
                void_cls: 0,
                boolean: 0,
                char_cls: 0,
                sbyte: 0,
                byte: 0,
                int16: 0,
                uint16: 0,
                int32: 0,
                uint32: 0,
                int64: 0,
                uint64: 0,
                single: 0,
                double: 0,
                intptr: 0,
                uintptr: 0,
            },
            corlib_class_count: 0,
            classes_by_short_name: HashMap::new(),
            empty_string_obj: 0,
            root_domain: 0,
            current_domain: 0,
            domains: HashMap::new(),
            domains_by_id: HashMap::new(),
            next_domain_id: 1,
            threads: HashMap::new(),
            assembly_by_ptr: HashMap::new(),
            image_by_ptr: HashMap::new(),
            class_by_ptr: HashMap::new(),
            vtable_by_ptr: HashMap::new(),
            method_by_ptr: HashMap::new(),
            field_by_ptr: HashMap::new(),
            prop_by_ptr: HashMap::new(),
            method_descs: HashMap::new(),
            internal_calls: HashMap::new(),
            search_dirs: Vec::new(),
            env_exit_code: 0,
            command_line_args: Vec::new(),
            c_strings: Vec::new(),
            owned_domains: Vec::new(),
            owned_assemblies: Vec::new(),
            owned_images: Vec::new(),
            owned_classes: Vec::new(),
            owned_vtables: Vec::new(),
            owned_methods: Vec::new(),
            owned_fields: Vec::new(),
            owned_props: Vec::new(),
            owned_threads: Vec::new(),
        };
        let dom = vm.create_domain("DefaultDomain");
        vm.root_domain = dom;
        vm.current_domain = dom;
        vm.init_corlib();
        vm
    }

    pub(crate) fn intern_cstr(&mut self, s: &str) -> *const c_char {
        let cs = CString::new(s.as_bytes()).unwrap_or_else(|_| CString::new("").unwrap());
        let ptr = cs.as_ptr();
        self.c_strings.push(cs);
        ptr
    }

    pub fn create_domain(&mut self, name: &str) -> usize {
        let id = self.next_domain_id;
        self.next_domain_id += 1;
        let name_ptr = self.intern_cstr(name);
        let boxed = Box::new(MonoDomain { id, name_ptr });
        let ptr = (&*boxed as *const MonoDomain) as usize;
        self.owned_domains.push(boxed);
        self.domains.insert(ptr, id);
        self.domains_by_id.insert(id, ptr);
        ptr
    }

    pub fn is_valid_domain(&self, dom: usize) -> bool {
        dom != 0 && self.domains.contains_key(&dom)
    }

    pub fn attach_current_thread(&mut self, domain: usize) -> usize {
        let dom = if self.is_valid_domain(domain) {
            domain
        } else {
            self.root_domain
        };
        let tid = current_os_tid();
        if let Some(&existing) = self.threads.get(&tid) {
            return existing;
        }
        let boxed = Box::new(MonoThread {
            os_tid: tid,
            domain: dom as *mut MonoDomain,
        });
        let ptr = (&*boxed as *const MonoThread) as usize;
        self.owned_threads.push(boxed);
        self.threads.insert(tid, ptr);
        ptr
    }

    pub fn detach_thread(&mut self, thread: usize) {
        self.threads.retain(|_, &mut v| v != thread);
    }

    pub fn current_thread(&self) -> usize {
        let tid = current_os_tid();
        self.threads.get(&tid).copied().unwrap_or(0)
    }

    pub fn corlib_image(&self) -> usize {
        self.assemblies[0].mono_image
    }

    #[inline]
    pub(crate) fn value_type_size(&self, class_id: usize) -> usize {
        self.classes
            .get(class_id)
            .map_or(8, |c| c.instance_payload_size.max(c.elem_size).max(1))
    }

    #[inline]
    pub(crate) fn empty_string(&mut self) -> usize {
        if self.empty_string_obj == 0 {
            self.empty_string_obj = self.alloc_string_utf16(&[]);
        }
        self.empty_string_obj
    }

    pub fn class_id_for_sig(&self, asm_id: usize, sig: &SigType) -> usize {
        match sig {
            SigType::Void => self.core.void_cls,
            SigType::Boolean => self.core.boolean,
            SigType::Char => self.core.char_cls,
            SigType::I1 => self.core.sbyte,
            SigType::U1 => self.core.byte,
            SigType::I2 => self.core.int16,
            SigType::U2 => self.core.uint16,
            SigType::I4 => self.core.int32,
            SigType::U4 => self.core.uint32,
            SigType::I8 => self.core.int64,
            SigType::U8 => self.core.uint64,
            SigType::R4 => self.core.single,
            SigType::R8 => self.core.double,
            SigType::String => self.core.string,
            SigType::IntPtr => self.core.intptr,
            SigType::UIntPtr => self.core.uintptr,
            SigType::Object => self.core.object,
            SigType::SzArray(_) | SigType::Array(_, _) => self.core.array,
            SigType::ValueType(tok) | SigType::Class(tok) | SigType::GenericInst(_, tok, _) => {
                self.resolve_type_token(asm_id, *tok)
                    .unwrap_or(self.core.object)
            }
            _ => self.core.object,
        }
    }

    pub fn resolve_type_token(&self, asm_id: usize, token: u32) -> Option<usize> {
        let tbl = (token >> 24) as u8;
        let rid = (token & 0x00FF_FFFF) as usize;
        if rid == 0 {
            return None;
        }
        let asm = self.assemblies.get(asm_id)?;
        match tbl {
            table::TYPE_DEF => asm.class_ids.get(rid - 1).copied(),
            table::TYPE_REF | table::TYPE_SPEC => {
                if let Some(&cid) = asm.type_cache.get(&token) {
                    return Some(cid);
                }
                let img = asm.image.as_ref()?;
                let (ns, name) = img.resolve_type_token_name(token)?;
                self.find_class_by_name_any(&ns, &name)
            }
            _ => None,
        }
    }

    pub fn is_instance_of(&self, obj: usize, target_class_id: usize) -> bool {
        if obj == 0 {
            return false;
        }
        let Some(obj_cid) = self.object_class_id(obj) else {
            return false;
        };
        self.is_subclass_of(obj_cid, target_class_id)
    }

    pub fn is_subclass_of(&self, sub: usize, sup: usize) -> bool {
        if sub == sup || sup == self.core.object {
            return true;
        }
        let mut cur = self.classes.get(sub).and_then(|c| c.parent_class_id);
        let mut guard = 0;
        while let Some(c) = cur {
            if c == sup {
                return true;
            }
            cur = self.classes.get(c).and_then(|ci| ci.parent_class_id);
            guard += 1;
            if guard > 64 {
                break;
            }
        }
        false
    }

    #[inline]
    fn class_id_for_heap_kind(&self, kind: &HeapObjKind) -> usize {
        match kind {
            HeapObjKind::Object { class_id }
            | HeapObjKind::Boxed { class_id, .. }
            | HeapObjKind::Delegate { class_id, .. }
            | HeapObjKind::Exception { class_id, .. }
            | HeapObjKind::RawStruct { class_id } => *class_id,
            HeapObjKind::String => self.core.string,
            HeapObjKind::Array { .. } => self.core.array,
            HeapObjKind::TypeObj { .. } => self.core.type_cls,
            HeapObjKind::StringBuilder { .. } => self.core.string_builder,
            HeapObjKind::List { .. } => self.core.list_cls,
            HeapObjKind::Dictionary { .. } => self.core.dict_cls,
        }
    }

    #[inline]
    pub fn object_class_id(&self, obj: usize) -> Option<usize> {
        Some(self.class_id_for_heap_kind(self.heap.get_managed(obj)?))
    }

    /// Safe class lookup for untrusted native C pointers (never dereferences `obj`).
    #[inline]
    pub fn object_class_id_untrusted(&self, obj: usize) -> Option<usize> {
        Some(self.class_id_for_heap_kind(self.heap.get_untrusted(obj)?))
    }

    pub fn alloc_string(&mut self, s: &str) -> usize {
        let vt = self.classes[self.core.string].mono_vtable;
        self.heap.alloc_string(vt, s)
    }

    pub fn alloc_string_utf16(&mut self, u16s: &[u16]) -> usize {
        let vt = self.classes[self.core.string].mono_vtable;
        self.heap.alloc_string_utf16(vt, u16s)
    }

    pub fn alloc_object_for_class(&mut self, class_id: usize) -> usize {
        let vt = self.classes[class_id].mono_vtable;
        let payload = self.classes[class_id].instance_payload_size;
        self.heap.alloc_object(vt, class_id, payload)
    }

    pub fn alloc_array_for_class(&mut self, elem_class_id: usize, length: usize) -> usize {
        let vt = self.classes[self.core.array].mono_vtable;
        let elem_sz = self
            .classes
            .get(elem_class_id)
            .map_or(8, |c| c.elem_size)
            .max(1);
        self.heap.alloc_array(vt, elem_class_id, elem_sz, length)
    }

    pub fn box_value(&mut self, class_id: usize, val: CilVal) -> usize {
        let Some((is_vt, vt, sig, vt_size)) = self.classes.get(class_id).map(|cls| {
            (
                cls.is_value_type,
                cls.mono_vtable,
                cls.primitive_sig.clone().unwrap_or(SigType::ValueType(0)),
                cls.instance_payload_size.max(cls.elem_size).max(1),
            )
        }) else {
            return val.as_obj();
        };
        if !is_vt {
            return val.as_obj();
        }
        match val {
            CilVal::ValueType { class_id: vcid, ptr } => {
                let sz = self.value_type_size(vcid as usize).min(vt_size);
                let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, sz) };
                self.heap.alloc_boxed(vt, class_id, sig, slice)
            }
            _ => {
                let (buf, len) = write_sig_to_stack_buf(&sig, val);
                self.heap.alloc_boxed(vt, class_id, sig, &buf[..len])
            }
        }
    }

    pub fn unbox_any(&self, obj: usize, class_id: usize) -> Result<CilVal, VmError> {
        let Some(cls) = self.classes.get(class_id) else {
            return Ok(CilVal::Obj(obj));
        };
        if !cls.is_value_type {
            if obj == 0 || self.is_instance_of(obj, class_id) {
                return Ok(CilVal::Obj(obj));
            }
            return Err(VmError::ManagedException {
                exc_obj: 0,
                message: format!("InvalidCastException to {}", cls.full_name()),
            });
        }
        if obj == 0 {
            return Err(VmError::ManagedException {
                exc_obj: 0,
                message: "NullReferenceException in unbox.any".into(),
            });
        }
        let payload_ptr = (obj + MONO_OBJECT_HEADER_SIZE) as *const u8;
        if let Some(sig) = &cls.primitive_sig {
            return Ok(read_sig_from_ptr(sig, payload_ptr));
        }
        Ok(CilVal::ValueType {
            class_id: class_id as u32,
            ptr: payload_ptr as usize,
        })
    }

    pub fn make_exception(&mut self, class_id: usize, msg: impl Into<String>) -> usize {
        let vt = self.classes[class_id].mono_vtable;
        self.heap.alloc_exception(vt, class_id, msg.into())
    }

    pub fn format_val_to_string(&self, val: CilVal) -> String {
        match val {
            CilVal::I32(v) => v.to_string(),
            CilVal::I64(v) => v.to_string(),
            CilVal::NativeInt(v) => v.to_string(),
            CilVal::F64(v) => format!("{v}"),
            CilVal::Obj(0) => String::new(),
            CilVal::Obj(p) => match self.heap.get_managed(p) {
                Some(HeapObjKind::String) => self.heap.read_string(p).unwrap_or_default(),
                Some(HeapObjKind::Boxed { sig, .. }) => {
                    let v = read_sig_from_ptr(sig, (p + MONO_OBJECT_HEADER_SIZE) as *const u8);
                    self.format_val_to_string(v)
                }
                Some(HeapObjKind::Exception { class_id, message }) => {
                    format!("{}: {message}", self.classes[*class_id].full_name())
                }
                Some(HeapObjKind::StringBuilder { buf }) => buf.clone(),
                Some(HeapObjKind::TypeObj {
                    represented_class_id,
                }) => self.classes[*represented_class_id].full_name(),
                Some(HeapObjKind::Object { class_id }) => self.classes[*class_id].full_name(),
                Some(HeapObjKind::Array { elem_class_id, .. }) => {
                    format!("{}[]", self.classes[*elem_class_id].full_name())
                }
                Some(HeapObjKind::List { .. }) => "System.Collections.Generic.List`1".to_string(),
                Some(HeapObjKind::Dictionary { .. }) => {
                    "System.Collections.Generic.Dictionary`2".to_string()
                }
                _ => format!("Object@0x{p:x}"),
            },
            CilVal::ValueType { class_id, .. } => self
                .classes
                .get(class_id as usize)
                .map_or_else(|| "ValueType".to_string(), |c| c.full_name()),
            CilVal::RawPtr(_)
            | CilVal::LocalPtr { .. }
            | CilVal::ArgPtr { .. }
            | CilVal::StaticFieldPtr(_) => String::new(),
        }
    }
}

fn current_os_tid() -> u64 {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    unsafe {
        let mut tid = 0u64;
        libc::pthread_threadid_np(0, &mut tid);
        tid
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        0
    }
}
