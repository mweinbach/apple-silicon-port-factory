//! Pure-Rust ECMA-335 Managed Heap, Type System, CIL Bytecode Interpreter,
//! BCL Intrinsics, P/Invoke Dispatcher, and Mono Internal-Call Bridge.

use super::metadata::{
    eh_flags, field_attr, method_attr, method_impl_attr, pinvoke_attr, table, AssemblyImage,
    MethodBody, MethodSig, SigType,
};
use std::collections::HashMap;
use std::ffi::{c_char, c_void, CString};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

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

/// `MonoObject` header (16 bytes): matches 64-bit Mono ABI.
pub const MONO_OBJECT_HEADER_SIZE: usize = 16;
/// `MonoString`: 16-byte header + 4-byte `length` + inline UTF-16 `chars` (+ NUL).
pub const MONO_STRING_LEN_OFFSET: usize = 16;
pub const MONO_STRING_CHARS_OFFSET: usize = 20;
/// `MonoArray`: 16-byte header + 8-byte `bounds` + 8-byte `max_length` + inline `vector`.
pub const MONO_ARRAY_LEN_OFFSET: usize = 24;
pub const MONO_ARRAY_DATA_OFFSET: usize = 32;

#[repr(C)]
pub struct MonoVTable {
    pub klass: *mut MonoClass,
    pub domain: *mut MonoDomain,
    pub class_id: usize,
}

#[repr(C)]
pub struct MonoDomain {
    pub id: u32,
    pub name_ptr: *const c_char,
}

#[repr(C)]
pub struct MonoAssembly {
    pub asm_id: usize,
    pub image: *mut MonoImage,
}

#[repr(C)]
pub struct MonoImage {
    pub asm_id: usize,
    pub name_ptr: *const c_char,
}

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

#[repr(C)]
pub struct MonoMethod {
    pub method_id: usize,
    pub asm_id: usize,
    pub class_id: usize,
    pub method_rid: u32,
    pub name_ptr: *const c_char,
}

#[repr(C)]
pub struct MonoClassField {
    pub field_id: usize,
    pub class_id: usize,
    pub offset: usize,
    pub size: usize,
    pub is_static: bool,
    pub name_ptr: *const c_char,
}

#[repr(C)]
pub struct MonoProperty {
    pub prop_id: usize,
    pub class_id: usize,
    pub getter: *mut MonoMethod,
    pub setter: *mut MonoMethod,
    pub name_ptr: *const c_char,
}

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
// CIL Value & Managed Address types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum ManagedAddr {
    Raw(usize),
    Local { frame: usize, idx: usize },
    Arg { frame: usize, idx: usize },
    StaticField(usize),
}

#[derive(Clone, Copy, Debug)]
pub enum CilVal {
    I32(i32),
    I64(i64),
    NativeInt(isize),
    F64(f64),
    Obj(usize),
    ManagedPtr(ManagedAddr),
    ValueType { class_id: usize, ptr: usize, size: usize },
}

impl CilVal {
    pub fn as_i32(self) -> i32 {
        match self {
            CilVal::I32(v) => v,
            CilVal::I64(v) => v as i32,
            CilVal::NativeInt(v) => v as i32,
            CilVal::F64(v) => v as i32,
            CilVal::Obj(p) => p as i32,
            CilVal::ManagedPtr(ManagedAddr::Raw(p)) => p as i32,
            _ => 0,
        }
    }

    pub fn as_i64(self) -> i64 {
        match self {
            CilVal::I32(v) => v as i64,
            CilVal::I64(v) => v,
            CilVal::NativeInt(v) => v as i64,
            CilVal::F64(v) => v as i64,
            CilVal::Obj(p) => p as i64,
            CilVal::ManagedPtr(ManagedAddr::Raw(p)) => p as i64,
            _ => 0,
        }
    }

    pub fn as_u64_bits(self) -> u64 {
        match self {
            CilVal::I32(v) => v as u32 as u64,
            CilVal::I64(v) => v as u64,
            CilVal::NativeInt(v) => v as usize as u64,
            CilVal::F64(v) => v.to_bits(),
            CilVal::Obj(p) => p as u64,
            CilVal::ManagedPtr(ManagedAddr::Raw(p)) => p as u64,
            CilVal::ValueType { ptr, .. } => ptr as u64,
            CilVal::ManagedPtr(_) => 0,
        }
    }

    pub fn as_isize(self) -> isize {
        match self {
            CilVal::I32(v) => v as isize,
            CilVal::I64(v) => v as isize,
            CilVal::NativeInt(v) => v,
            CilVal::Obj(p) => p as isize,
            CilVal::ManagedPtr(ManagedAddr::Raw(p)) => p as isize,
            CilVal::F64(v) => v as isize,
            _ => 0,
        }
    }

    pub fn as_f64(self) -> f64 {
        match self {
            CilVal::F64(v) => v,
            CilVal::I32(v) => v as f64,
            CilVal::I64(v) => v as f64,
            CilVal::NativeInt(v) => v as f64,
            _ => 0.0,
        }
    }

    pub fn as_obj(self) -> usize {
        match self {
            CilVal::Obj(p) => p,
            CilVal::NativeInt(p) => p as usize,
            CilVal::I64(p) => p as usize,
            CilVal::I32(p) => p as u32 as usize,
            _ => 0,
        }
    }

    pub fn is_truthy(self) -> bool {
        match self {
            CilVal::I32(v) => v != 0,
            CilVal::I64(v) => v != 0,
            CilVal::NativeInt(v) => v != 0,
            CilVal::Obj(p) => p != 0,
            CilVal::F64(v) => v != 0.0,
            CilVal::ManagedPtr(ManagedAddr::Raw(p)) => p != 0,
            CilVal::ManagedPtr(_) => true,
            CilVal::ValueType { ptr, .. } => ptr != 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Managed Heap & Metadata Models
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum HeapObjKind {
    Object {
        class_id: usize,
    },
    String,
    Array {
        elem_class_id: usize,
        elem_size: usize,
        length: usize,
    },
    Boxed {
        class_id: usize,
        sig: SigType,
    },
    Delegate {
        class_id: usize,
        method_id: usize,
        target_obj: usize,
    },
    TypeObj {
        represented_class_id: usize,
    },
    StringBuilder {
        buf: String,
    },
    Exception {
        class_id: usize,
        message: String,
    },
    RawStruct {
        class_id: usize,
    },
}

pub struct GcHandleEntry {
    pub target: usize,
    pub _pinned: bool,
    pub _weak: bool,
}

pub struct ManagedHeap {
    /// Pinned 8-byte-aligned backing blocks so object pointers remain valid for VM lifetime.
    blocks: Vec<Box<[u64]>>,
    pub objects: HashMap<usize, HeapObjKind>,
    pub gc_handles: HashMap<u32, GcHandleEntry>,
    next_gc_handle: u32,
}

impl ManagedHeap {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            objects: HashMap::new(),
            gc_handles: HashMap::new(),
            next_gc_handle: 1,
        }
    }

    fn alloc_raw(&mut self, byte_len: usize) -> *mut u8 {
        let words = byte_len.div_ceil(8).max(2);
        let mut blk = vec![0u64; words].into_boxed_slice();
        let ptr = blk.as_mut_ptr() as *mut u8;
        self.blocks.push(blk);
        ptr
    }

    pub fn alloc_object(&mut self, vtable: usize, class_id: usize, payload_size: usize) -> usize {
        let total = MONO_OBJECT_HEADER_SIZE + payload_size;
        let ptr = self.alloc_raw(total);
        unsafe {
            std::ptr::write_unaligned(ptr as *mut usize, vtable);
        }
        let addr = ptr as usize;
        self.objects.insert(addr, HeapObjKind::Object { class_id });
        addr
    }

    pub fn alloc_string_utf16(&mut self, string_vtable: usize, utf16: &[u16]) -> usize {
        let total = MONO_STRING_CHARS_OFFSET + (utf16.len() + 1) * 2;
        let ptr = self.alloc_raw(total);
        unsafe {
            std::ptr::write_unaligned(ptr as *mut usize, string_vtable);
            std::ptr::write_unaligned(ptr.add(MONO_STRING_LEN_OFFSET) as *mut i32, utf16.len() as i32);
            let dst = ptr.add(MONO_STRING_CHARS_OFFSET) as *mut u16;
            for (i, &ch) in utf16.iter().enumerate() {
                std::ptr::write_unaligned(dst.add(i), ch);
            }
            std::ptr::write_unaligned(dst.add(utf16.len()), 0u16);
        }
        let addr = ptr as usize;
        self.objects.insert(addr, HeapObjKind::String);
        addr
    }

    pub fn alloc_string(&mut self, string_vtable: usize, s: &str) -> usize {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        self.alloc_string_utf16(string_vtable, &utf16)
    }

    pub fn read_string(&self, obj: usize) -> Option<String> {
        if obj == 0 {
            return None;
        }
        if !matches!(self.objects.get(&obj), Some(HeapObjKind::String)) {
            return None;
        }
        unsafe {
            let ptr = obj as *const u8;
            let len = std::ptr::read_unaligned(ptr.add(MONO_STRING_LEN_OFFSET) as *const i32);
            if len < 0 {
                return Some(String::new());
            }
            let n = len as usize;
            let chars_ptr = ptr.add(MONO_STRING_CHARS_OFFSET) as *const u16;
            let mut u16s = Vec::with_capacity(n);
            for i in 0..n {
                u16s.push(std::ptr::read_unaligned(chars_ptr.add(i)));
            }
            Some(String::from_utf16_lossy(&u16s))
        }
    }

    pub fn read_string_utf16(&self, obj: usize) -> Option<Vec<u16>> {
        if obj == 0 || !matches!(self.objects.get(&obj), Some(HeapObjKind::String)) {
            return None;
        }
        unsafe {
            let ptr = obj as *const u8;
            let len = std::ptr::read_unaligned(ptr.add(MONO_STRING_LEN_OFFSET) as *const i32);
            if len < 0 {
                return Some(Vec::new());
            }
            let n = len as usize;
            let chars_ptr = ptr.add(MONO_STRING_CHARS_OFFSET) as *const u16;
            let mut u16s = Vec::with_capacity(n);
            for i in 0..n {
                u16s.push(std::ptr::read_unaligned(chars_ptr.add(i)));
            }
            Some(u16s)
        }
    }

    pub fn alloc_array(
        &mut self,
        array_vtable: usize,
        elem_class_id: usize,
        elem_size: usize,
        length: usize,
    ) -> usize {
        let elem_sz = elem_size.max(1);
        let total = MONO_ARRAY_DATA_OFFSET + length.saturating_mul(elem_sz);
        let ptr = self.alloc_raw(total);
        unsafe {
            std::ptr::write_unaligned(ptr as *mut usize, array_vtable);
            std::ptr::write_unaligned(ptr.add(MONO_ARRAY_LEN_OFFSET) as *mut usize, length);
        }
        let addr = ptr as usize;
        self.objects.insert(
            addr,
            HeapObjKind::Array {
                elem_class_id,
                elem_size: elem_sz,
                length,
            },
        );
        addr
    }

    pub fn alloc_boxed(&mut self, vtable: usize, class_id: usize, sig: SigType, bytes: &[u8]) -> usize {
        let size = bytes.len().max(1);
        let total = MONO_OBJECT_HEADER_SIZE + size;
        let ptr = self.alloc_raw(total);
        unsafe {
            std::ptr::write_unaligned(ptr as *mut usize, vtable);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.add(MONO_OBJECT_HEADER_SIZE), bytes.len());
        }
        let addr = ptr as usize;
        self.objects
            .insert(addr, HeapObjKind::Boxed { class_id, sig });
        addr
    }

    pub fn alloc_raw_struct(&mut self, class_id: usize, size: usize) -> usize {
        let sz = size.max(8);
        let ptr = self.alloc_raw(sz);
        let addr = ptr as usize;
        self.objects.insert(addr, HeapObjKind::RawStruct { class_id });
        addr
    }

    pub fn alloc_exception(&mut self, vtable: usize, class_id: usize, message: String) -> usize {
        let ptr = self.alloc_raw(MONO_OBJECT_HEADER_SIZE + 16);
        unsafe {
            std::ptr::write_unaligned(ptr as *mut usize, vtable);
        }
        let addr = ptr as usize;
        self.objects
            .insert(addr, HeapObjKind::Exception { class_id, message });
        addr
    }

    pub fn gchandle_new(&mut self, target: usize, pinned: bool, weak: bool) -> u32 {
        let h = self.next_gc_handle;
        self.next_gc_handle = self.next_gc_handle.wrapping_add(1).max(1);
        self.gc_handles.insert(
            h,
            GcHandleEntry {
                target,
                _pinned: pinned,
                _weak: weak,
            },
        );
        h
    }

    pub fn gchandle_get(&self, handle: u32) -> usize {
        self.gc_handles.get(&handle).map_or(0, |e| e.target)
    }

    pub fn gchandle_free(&mut self, handle: u32) {
        self.gc_handles.remove(&handle);
    }
}

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
    pub body: Option<Arc<MethodBody>>,
    pub locals_sig: Vec<SigType>,
    pub pinvoke: Option<(String, String, u16)>,
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

pub struct ClrVm {
    pub heap: ManagedHeap,
    pub assemblies: Vec<LoadedAssembly>,
    pub classes: Vec<ClassInfo>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub properties: Vec<PropertyInfo>,
    pub core: CoreClasses,

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

    fn intern_cstr(&mut self, s: &str) -> *const c_char {
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

    fn init_corlib(&mut self) {
        let name_ptr = self.intern_cstr("mscorlib");
        let img_box = Box::new(MonoImage {
            asm_id: 0,
            name_ptr,
        });
        let img_ptr = (&*img_box as *const MonoImage) as usize;
        let asm_box = Box::new(MonoAssembly {
            asm_id: 0,
            image: img_ptr as *mut MonoImage,
        });
        let asm_ptr = (&*asm_box as *const MonoAssembly) as usize;
        self.owned_images.push(img_box);
        self.owned_assemblies.push(asm_box);
        self.image_by_ptr.insert(img_ptr, 0);
        self.assembly_by_ptr.insert(asm_ptr, 0);

        self.assemblies.push(LoadedAssembly {
            id: 0,
            name: "mscorlib".to_string(),
            path: None,
            image: None,
            mono_assembly: asm_ptr,
            mono_image: img_ptr,
            class_ids: Vec::new(),
            method_ids: Vec::new(),
            field_ids: Vec::new(),
        });

        let obj = self.add_corlib_class("System", "Object", None, false, false, false, None, 8);
        let vt = self.add_corlib_class("System", "ValueType", Some(obj), false, false, false, None, 8);
        let _en = self.add_corlib_class("System", "Enum", Some(vt), true, false, false, None, 4);
        let str_c = self.add_corlib_class("System", "String", Some(obj), false, false, false, Some(SigType::String), 8);
        let arr_c = self.add_corlib_class("System", "Array", Some(obj), false, false, false, None, 8);
        let exc = self.add_corlib_class("System", "Exception", Some(obj), false, false, true, None, 8);
        let sys_exc = self.add_corlib_class("System", "SystemException", Some(exc), false, false, true, None, 8);
        let inv_cast = self.add_corlib_class("System", "InvalidCastException", Some(sys_exc), false, false, true, None, 8);
        let null_ref = self.add_corlib_class("System", "NullReferenceException", Some(sys_exc), false, false, true, None, 8);
        let idx_oor = self.add_corlib_class("System", "IndexOutOfRangeException", Some(sys_exc), false, false, true, None, 8);
        let div0 = self.add_corlib_class("System", "DivideByZeroException", Some(sys_exc), false, false, true, None, 8);
        let ovf = self.add_corlib_class("System", "OverflowException", Some(sys_exc), false, false, true, None, 8);
        let _arg_exc = self.add_corlib_class("System", "ArgumentException", Some(sys_exc), false, false, true, None, 8);
        let _arg_null = self.add_corlib_class("System", "ArgumentNullException", Some(sys_exc), false, false, true, None, 8);
        let _not_impl = self.add_corlib_class("System", "NotImplementedException", Some(sys_exc), false, false, true, None, 8);
        let type_c = self.add_corlib_class("System", "Type", Some(obj), false, false, false, None, 8);
        let del = self.add_corlib_class("System", "Delegate", Some(obj), false, true, false, None, 8);
        let _mdel = self.add_corlib_class("System", "MulticastDelegate", Some(del), false, true, false, None, 8);
        let sb = self.add_corlib_class("System.Text", "StringBuilder", Some(obj), false, false, false, None, 8);

        let void_c = self.add_corlib_class("System", "Void", Some(vt), true, false, false, Some(SigType::Void), 0);
        let bool_c = self.add_corlib_class("System", "Boolean", Some(vt), true, false, false, Some(SigType::Boolean), 1);
        let char_c = self.add_corlib_class("System", "Char", Some(vt), true, false, false, Some(SigType::Char), 2);
        let i1_c = self.add_corlib_class("System", "SByte", Some(vt), true, false, false, Some(SigType::I1), 1);
        let u1_c = self.add_corlib_class("System", "Byte", Some(vt), true, false, false, Some(SigType::U1), 1);
        let i2_c = self.add_corlib_class("System", "Int16", Some(vt), true, false, false, Some(SigType::I2), 2);
        let u2_c = self.add_corlib_class("System", "UInt16", Some(vt), true, false, false, Some(SigType::U2), 2);
        let i4_c = self.add_corlib_class("System", "Int32", Some(vt), true, false, false, Some(SigType::I4), 4);
        let u4_c = self.add_corlib_class("System", "UInt32", Some(vt), true, false, false, Some(SigType::U4), 4);
        let i8_c = self.add_corlib_class("System", "Int64", Some(vt), true, false, false, Some(SigType::I8), 8);
        let u8_c = self.add_corlib_class("System", "UInt64", Some(vt), true, false, false, Some(SigType::U8), 8);
        let r4_c = self.add_corlib_class("System", "Single", Some(vt), true, false, false, Some(SigType::R4), 4);
        let r8_c = self.add_corlib_class("System", "Double", Some(vt), true, false, false, Some(SigType::R8), 8);
        let ip_c = self.add_corlib_class("System", "IntPtr", Some(vt), true, false, false, Some(SigType::IntPtr), 8);
        let up_c = self.add_corlib_class("System", "UIntPtr", Some(vt), true, false, false, Some(SigType::UIntPtr), 8);

        for (ns, name) in [
            ("System", "Math"),
            ("System", "MathF"),
            ("System", "Buffer"),
            ("System", "Environment"),
            ("System", "Console"),
            ("System", "Convert"),
            ("System", "BitConverter"),
            ("System", "GC"),
            ("System.Runtime.InteropServices", "Marshal"),
            ("System.Runtime.CompilerServices", "RuntimeHelpers"),
        ] {
            let _ = self.add_corlib_class(ns, name, Some(obj), false, false, false, None, 8);
        }

        self.core = CoreClasses {
            object: obj,
            string: str_c,
            array: arr_c,
            exception: exc,
            invalid_cast: inv_cast,
            null_ref,
            index_out_of_range: idx_oor,
            div_by_zero: div0,
            overflow: ovf,
            type_cls: type_c,
            string_builder: sb,
            void_cls: void_c,
            boolean: bool_c,
            char_cls: char_c,
            sbyte: i1_c,
            byte: u1_c,
            int16: i2_c,
            uint16: u2_c,
            int32: i4_c,
            uint32: u4_c,
            int64: i8_c,
            uint64: u8_c,
            single: r4_c,
            double: r8_c,
            intptr: ip_c,
            uintptr: up_c,
        };
    }

    #[allow(clippy::too_many_arguments)]
    fn add_corlib_class(
        &mut self,
        ns: &str,
        name: &str,
        parent: Option<usize>,
        is_value_type: bool,
        is_delegate: bool,
        is_exception: bool,
        primitive_sig: Option<SigType>,
        elem_size: usize,
    ) -> usize {
        let class_id = self.classes.len();
        let name_ptr = self.intern_cstr(name);
        let ns_ptr = self.intern_cstr(ns);
        let c_box = Box::new(MonoClass {
            class_id,
            asm_id: 0,
            typedef_rid: 0,
            name_ptr,
            namespace_ptr: ns_ptr,
            instance_size: MONO_OBJECT_HEADER_SIZE + elem_size,
            elem_size,
            is_value_type,
        });
        let c_ptr = (&*c_box as *const MonoClass) as usize;
        let vt_box = Box::new(MonoVTable {
            klass: c_ptr as *mut MonoClass,
            domain: self.root_domain as *mut MonoDomain,
            class_id,
        });
        let vt_ptr = (&*vt_box as *const MonoVTable) as usize;
        self.owned_classes.push(c_box);
        self.owned_vtables.push(vt_box);
        self.class_by_ptr.insert(c_ptr, class_id);
        self.vtable_by_ptr.insert(vt_ptr, class_id);
        self.classes.push(ClassInfo {
            namespace: ns.to_string(),
            name: name.to_string(),
            parent_class_id: parent,
            is_value_type,
            is_delegate,
            is_exception,
            primitive_sig,
            elem_size,
            instance_payload_size: elem_size,
            fields: Vec::new(),
            methods: Vec::new(),
            properties: Vec::new(),
            cctor_ran: true,
            mono_class: c_ptr,
            mono_vtable: vt_ptr,
        });
        self.assemblies[0].class_ids.push(class_id);
        class_id
    }

    pub fn corlib_image(&self) -> usize {
        self.assemblies[0].mono_image
    }

    pub fn load_assembly_file(&mut self, path: &Path) -> Result<usize, VmError> {
        // Resolve via direct path or search_dirs
        let resolved = if path.is_file() {
            path.to_path_buf()
        } else {
            let mut found = None;
            let file_name = path.file_name().map(|f| f.to_os_string());
            if let Some(fname) = file_name {
                for dir in &self.search_dirs {
                    let cand = dir.join(&fname);
                    if cand.is_file() {
                        found = Some(cand);
                        break;
                    }
                    let with_dll = dir.join(format!("{}.dll", fname.to_string_lossy()));
                    if with_dll.is_file() {
                        found = Some(with_dll);
                        break;
                    }
                }
            }
            found.ok_or_else(|| VmError::FileNotFound(path.display().to_string()))?
        };

        let canon = std::fs::canonicalize(&resolved).unwrap_or_else(|_| resolved.clone());
        for a in &self.assemblies {
            if a.path.as_ref() == Some(&canon) {
                return Ok(a.id);
            }
        }
        let bytes = std::fs::read(&canon)
            .map_err(|e| VmError::FileNotFound(format!("{}: {e}", canon.display())))?;
        self.load_assembly_bytes(&bytes, Some(canon))
    }

    pub fn load_assembly_bytes(
        &mut self,
        data: &[u8],
        path: Option<PathBuf>,
    ) -> Result<usize, VmError> {
        let parsed = Arc::new(
            AssemblyImage::parse(data).map_err(|e| VmError::BadImage(e.0))?,
        );
        let asm_id = self.assemblies.len();
        let asm_name = parsed.assembly_name();

        let name_ptr = self.intern_cstr(&asm_name);
        let img_box = Box::new(MonoImage { asm_id, name_ptr });
        let img_ptr = (&*img_box as *const MonoImage) as usize;
        let asm_box = Box::new(MonoAssembly {
            asm_id,
            image: img_ptr as *mut MonoImage,
        });
        let asm_ptr = (&*asm_box as *const MonoAssembly) as usize;
        self.owned_images.push(img_box);
        self.owned_assemblies.push(asm_box);
        self.image_by_ptr.insert(img_ptr, asm_id);
        self.assembly_by_ptr.insert(asm_ptr, asm_id);

        let mut class_ids = Vec::with_capacity(parsed.type_defs.len());
        let mut method_ids = vec![0usize; parsed.method_defs.len()];
        let mut field_ids = vec![0usize; parsed.fields.len()];

        // Pass 1: allocate ClassInfo for every TypeDef
        for (i, td) in parsed.type_defs.iter().enumerate() {
            let rid = (i + 1) as u32;
            let ns = parsed.get_string(td.namespace).to_string();
            let name = parsed.get_string(td.name).to_string();
            let class_id = self.classes.len();

            let ext_name = parsed.resolve_type_token_name(td.extends);
            let is_value_type = matches!(
                ext_name.as_ref().map(|(ns, n)| (ns.as_str(), n.as_str())),
                Some(("System", "ValueType")) | Some(("System", "Enum"))
            );
            let is_delegate = matches!(
                ext_name.as_ref().map(|(ns, n)| (ns.as_str(), n.as_str())),
                Some(("System", "Delegate")) | Some(("System", "MulticastDelegate"))
            );
            let is_exception = ext_name
                .as_ref()
                .is_some_and(|(_, n)| n.ends_with("Exception"));

            let name_c = self.intern_cstr(&name);
            let ns_c = self.intern_cstr(&ns);
            let c_box = Box::new(MonoClass {
                class_id,
                asm_id,
                typedef_rid: rid,
                name_ptr: name_c,
                namespace_ptr: ns_c,
                instance_size: MONO_OBJECT_HEADER_SIZE,
                elem_size: 8,
                is_value_type,
            });
            let c_ptr = (&*c_box as *const MonoClass) as usize;
            let vt_box = Box::new(MonoVTable {
                klass: c_ptr as *mut MonoClass,
                domain: self.root_domain as *mut MonoDomain,
                class_id,
            });
            let vt_ptr = (&*vt_box as *const MonoVTable) as usize;
            self.owned_classes.push(c_box);
            self.owned_vtables.push(vt_box);
            self.class_by_ptr.insert(c_ptr, class_id);
            self.vtable_by_ptr.insert(vt_ptr, class_id);

            self.classes.push(ClassInfo {
                namespace: ns,
                name,
                parent_class_id: None,
                is_value_type,
                is_delegate,
                is_exception,
                primitive_sig: None,
                elem_size: 8,
                instance_payload_size: 0,
                fields: Vec::new(),
                methods: Vec::new(),
                properties: Vec::new(),
                cctor_ran: false,
                mono_class: c_ptr,
                mono_vtable: vt_ptr,
            });
            class_ids.push(class_id);
        }

        // Pass 2: link parent_class_id
        for (i, td) in parsed.type_defs.iter().enumerate() {
            let class_id = class_ids[i];
            if td.extends != 0 {
                let ext_tbl = (td.extends >> 24) as u8;
                let ext_rid = (td.extends & 0x00FF_FFFF) as usize;
                let parent = if ext_tbl == table::TYPE_DEF && ext_rid >= 1 && ext_rid <= class_ids.len() {
                    Some(class_ids[ext_rid - 1])
                } else if let Some((pns, pn)) = parsed.resolve_type_token_name(td.extends) {
                    self.find_class_by_name_any(&pns, &pn)
                } else {
                    None
                };
                self.classes[class_id].parent_class_id = parent;
                if let Some(pid) = parent {
                    if self.classes[pid].is_exception {
                        self.classes[class_id].is_exception = true;
                    }
                    if self.classes[pid].is_delegate {
                        self.classes[class_id].is_delegate = true;
                    }
                }
            }
        }

        // Pass 3: fields, methods, properties per TypeDef
        for (i, &class_id) in class_ids.iter().enumerate() {
            let td_rid = (i + 1) as u32;
            let mut inst_off = 0usize;

            for f_rid in parsed.type_field_range(td_rid) {
                let f_row = parsed.fields[(f_rid - 1) as usize];
                let fname = parsed.get_string(f_row.name).to_string();
                let fsig = parsed.parse_field_sig(f_row.signature);
                let is_static = (f_row.flags & field_attr::STATIC) != 0
                    || (f_row.flags & field_attr::LITERAL) != 0;
                let fsize = fsig.elem_size().max(1);
                let align = fsize.min(8).next_power_of_two();
                let struct_offset = if is_static {
                    0
                } else {
                    if let Some(fl) = parsed.field_layouts.iter().find(|fl| fl.field == f_rid) {
                        let o = fl.offset as usize;
                        inst_off = inst_off.max(o + fsize);
                        o
                    } else {
                        let aligned = (inst_off + align - 1) & !(align - 1);
                        inst_off = aligned + fsize;
                        aligned
                    }
                };
                let field_id = self.fields.len();
                let fname_c = self.intern_cstr(&fname);
                let f_box = Box::new(MonoClassField {
                    field_id,
                    class_id,
                    offset: if is_static {
                        0
                    } else {
                        MONO_OBJECT_HEADER_SIZE + struct_offset
                    },
                    size: fsize,
                    is_static,
                    name_ptr: fname_c,
                });
                let f_ptr = (&*f_box as *const MonoClassField) as usize;
                self.owned_fields.push(f_box);
                self.field_by_ptr.insert(f_ptr, field_id);

                let field_tok = ((table::FIELD as u32) << 24) | f_rid;
                let init_val = if let Some(c) = parsed.constants.iter().find(|c| c.parent == field_tok) {
                    let blob = parsed.get_blob(c.value);
                    match c.elem_type {
                        super::metadata::elem_type::I4 | super::metadata::elem_type::U4 if blob.len() >= 4 => {
                            CilVal::I32(i32::from_le_bytes([blob[0], blob[1], blob[2], blob[3]]))
                        }
                        super::metadata::elem_type::I8 | super::metadata::elem_type::U8 if blob.len() >= 8 => {
                            CilVal::I64(i64::from_le_bytes([
                                blob[0], blob[1], blob[2], blob[3], blob[4], blob[5], blob[6], blob[7],
                            ]))
                        }
                        super::metadata::elem_type::BOOLEAN | super::metadata::elem_type::U1 if !blob.is_empty() => {
                            CilVal::I32(blob[0] as i32)
                        }
                        super::metadata::elem_type::STRING => {
                            let u16s: Vec<u16> = blob
                                .chunks_exact(2)
                                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                                .collect();
                            CilVal::Obj(self.alloc_string_utf16(&u16s))
                        }
                        _ => default_val_for_sig(&fsig),
                    }
                } else {
                    default_val_for_sig(&fsig)
                };
                self.fields.push(FieldInfo {
                    class_id,
                    name: fname,
                    sig: fsig,
                    is_static,
                    struct_offset,
                    size: fsize,
                    static_value: init_val,
                    mono_field: f_ptr,
                });
                field_ids[(f_rid - 1) as usize] = field_id;
                self.classes[class_id].fields.push(field_id);
            }

            let payload = (inst_off + 7) & !7;
            self.classes[class_id].instance_payload_size = payload;
            if self.classes[class_id].is_value_type {
                self.classes[class_id].elem_size = payload.max(1);
            }
            unsafe {
                let mc = self.classes[class_id].mono_class as *mut MonoClass;
                (*mc).instance_size = MONO_OBJECT_HEADER_SIZE + payload;
                (*mc).elem_size = self.classes[class_id].elem_size;
            }

            // Methods
            for m_rid in parsed.type_method_range(td_rid) {
                let m_row = parsed.method_defs[(m_rid - 1) as usize];
                let mname = parsed.get_string(m_row.name).to_string();
                let msig = parsed.parse_method_sig(m_row.signature);
                let body = if m_row.rva != 0 {
                    parsed.parse_method_body(m_row.rva).ok().map(Arc::new)
                } else {
                    None
                };
                let locals_sig = body
                    .as_ref()
                    .map(|b| parsed.parse_local_var_sig(b.local_var_sig_tok))
                    .unwrap_or_default();
                let pinvoke = if (m_row.flags & method_attr::PINVOKE_IMPL) != 0 {
                    parsed.method_pinvoke(m_rid)
                } else {
                    None
                };

                let method_id = self.methods.len();
                let mname_c = self.intern_cstr(&mname);
                let m_box = Box::new(MonoMethod {
                    method_id,
                    asm_id,
                    class_id,
                    method_rid: m_rid,
                    name_ptr: mname_c,
                });
                let m_ptr = (&*m_box as *const MonoMethod) as usize;
                self.owned_methods.push(m_box);
                self.method_by_ptr.insert(m_ptr, method_id);

                self.methods.push(MethodInfo {
                    asm_id,
                    class_id,
                    name: mname,
                    flags: m_row.flags,
                    impl_flags: m_row.impl_flags,
                    sig: msig,
                    body,
                    locals_sig,
                    pinvoke,
                    mono_method: m_ptr,
                });
                method_ids[(m_rid - 1) as usize] = method_id;
                self.classes[class_id].methods.push(method_id);
            }
        }

        // Pass 4: properties (needs method_ids populated first)
        for (i, &class_id) in class_ids.iter().enumerate() {
            let td_rid = (i + 1) as u32;
            for p_rid in parsed.type_property_range(td_rid) {
                let p_row = parsed.properties[(p_rid - 1) as usize];
                let pname = parsed.get_string(p_row.name).to_string();
                let (g_rid, s_rid) = parsed.property_accessors(p_rid);
                let getter_id = g_rid
                    .and_then(|r| method_ids.get(r.saturating_sub(1) as usize).copied())
                    .filter(|&id| id < self.methods.len());
                let setter_id = s_rid
                    .and_then(|r| method_ids.get(r.saturating_sub(1) as usize).copied())
                    .filter(|&id| id < self.methods.len());
                let getter_ptr = getter_id
                    .map_or(std::ptr::null_mut(), |id| self.methods[id].mono_method as *mut MonoMethod);
                let setter_ptr = setter_id
                    .map_or(std::ptr::null_mut(), |id| self.methods[id].mono_method as *mut MonoMethod);

                let prop_id = self.properties.len();
                let pname_c = self.intern_cstr(&pname);
                let p_box = Box::new(MonoProperty {
                    prop_id,
                    class_id,
                    getter: getter_ptr,
                    setter: setter_ptr,
                    name_ptr: pname_c,
                });
                let p_ptr = (&*p_box as *const MonoProperty) as usize;
                self.owned_props.push(p_box);
                self.prop_by_ptr.insert(p_ptr, prop_id);

                self.properties.push(PropertyInfo {
                    name: pname,
                    getter_method_id: getter_id,
                    setter_method_id: setter_id,
                    mono_prop: p_ptr,
                });
                self.classes[class_id].properties.push(prop_id);
            }
        }

        // Mark classes without a `.cctor` as already initialized
        for &cid in &class_ids {
            let has_cctor = self.classes[cid]
                .methods
                .iter()
                .any(|&mid| self.methods[mid].name == ".cctor");
            if !has_cctor {
                self.classes[cid].cctor_ran = true;
            }
        }

        self.assemblies.push(LoadedAssembly {
            id: asm_id,
            name: asm_name,
            path,
            image: Some(parsed),
            mono_assembly: asm_ptr,
            mono_image: img_ptr,
            class_ids,
            method_ids,
            field_ids,
        });

        Ok(asm_id)
    }

    pub fn find_assembly_by_name(&self, name: &str) -> Option<usize> {
        let short = name.split(',').next().unwrap_or(name).trim();
        let stripped = short
            .strip_suffix(".dll")
            .or_else(|| short.strip_suffix(".exe"))
            .unwrap_or(short);
        self.assemblies
            .iter()
            .rev()
            .find(|a| a.name.eq_ignore_ascii_case(stripped))
            .map(|a| a.id)
    }

    pub fn find_class_in_assembly(&self, asm_id: usize, ns: &str, name: &str) -> Option<usize> {
        let asm = self.assemblies.get(asm_id)?;
        asm.class_ids
            .iter()
            .copied()
            .find(|&cid| self.classes[cid].namespace == ns && self.classes[cid].name == name)
    }

    pub fn find_class_by_full_name_in_assembly(&self, asm_id: usize, full: &str) -> Option<usize> {
        let clean = full.split(',').next().unwrap_or(full).trim();
        let (ns, name) = split_full_type_name(clean);
        self.find_class_in_assembly(asm_id, ns, name)
    }

    pub fn find_class_by_name_any(&self, ns: &str, name: &str) -> Option<usize> {
        // Search user assemblies first (newest first), then corlib (id 0).
        for asm in self.assemblies.iter().rev() {
            if let Some(cid) = self.find_class_in_assembly(asm.id, ns, name) {
                return Some(cid);
            }
        }
        None
    }

    pub fn find_class_by_full_name_any(&self, full: &str) -> Option<usize> {
        if let Some((ty_part, asm_part)) = full.split_once(',') {
            let asm_name = asm_part.split(',').next().unwrap_or(asm_part).trim();
            if let Some(aid) = self.find_assembly_by_name(asm_name) {
                if let Some(cid) = self.find_class_by_full_name_in_assembly(aid, ty_part.trim()) {
                    return Some(cid);
                }
            }
        }
        let clean = full.split(',').next().unwrap_or(full).trim();
        let (ns, name) = split_full_type_name(clean);
        self.find_class_by_name_any(ns, name)
    }

    pub fn find_method_in_class(
        &self,
        class_id: usize,
        name: &str,
        param_count: i32,
    ) -> Option<usize> {
        let mut cur = Some(class_id);
        while let Some(cid) = cur {
            let cls = self.classes.get(cid)?;
            for &mid in &cls.methods {
                let m = &self.methods[mid];
                if m.name == name && (param_count < 0 || m.sig.params.len() as i32 == param_count) {
                    return Some(mid);
                }
            }
            cur = cls.parent_class_id;
        }
        None
    }

    pub fn find_field_in_class(&self, class_id: usize, name: &str) -> Option<usize> {
        let mut cur = Some(class_id);
        while let Some(cid) = cur {
            let cls = self.classes.get(cid)?;
            for &fid in &cls.fields {
                if self.fields[fid].name == name {
                    return Some(fid);
                }
            }
            cur = cls.parent_class_id;
        }
        None
    }

    pub fn find_property_in_class(&self, class_id: usize, name: &str) -> Option<usize> {
        let mut cur = Some(class_id);
        while let Some(cid) = cur {
            let cls = self.classes.get(cid)?;
            for &pid in &cls.properties {
                if self.properties[pid].name == name {
                    return Some(pid);
                }
            }
            cur = cls.parent_class_id;
        }
        None
    }

    pub fn entry_point_method(&self, asm_id: usize) -> Option<usize> {
        let asm = self.assemblies.get(asm_id)?;
        let img = asm.image.as_ref()?;
        // Native entrypoint flag (COMIMAGE_FLAGS_NATIVE_ENTRYPOINT = 0x10) has an RVA, not a MethodDef token.
        if (img.cor20.flags & 0x10) != 0 {
            return None;
        }
        let tok = img.cor20.entry_point_token;
        if (tok >> 24) as u8 != table::METHOD_DEF {
            return None;
        }
        let rid = (tok & 0x00FF_FFFF) as usize;
        rid.checked_sub(1).and_then(|i| asm.method_ids.get(i).copied())
    }

    pub fn register_internal_call(&mut self, name: &str, fn_ptr: usize) {
        self.internal_calls.insert(name.to_string(), fn_ptr);
        if let Some((base, _)) = name.split_once('(') {
            self.internal_calls.insert(base.to_string(), fn_ptr);
        }
    }

    pub fn lookup_internal_call(&self, class_id: usize, method_name: &str) -> Option<usize> {
        let cls = self.classes.get(class_id)?;
        let full = format!("{}::{method_name}", cls.full_name());
        if let Some(&p) = self.internal_calls.get(&full) {
            return Some(p);
        }
        let short = format!("{}::{method_name}", cls.name);
        self.internal_calls.get(&short).copied()
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
            table::TYPE_REF => {
                let img = asm.image.as_ref()?;
                let (ns, name) = img.resolve_type_token_name(token)?;
                self.find_class_by_name_any(&ns, &name)
            }
            table::TYPE_SPEC => {
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

    pub fn object_class_id(&self, obj: usize) -> Option<usize> {
        match self.heap.objects.get(&obj)? {
            HeapObjKind::Object { class_id }
            | HeapObjKind::Boxed { class_id, .. }
            | HeapObjKind::Delegate { class_id, .. }
            | HeapObjKind::Exception { class_id, .. }
            | HeapObjKind::RawStruct { class_id, .. } => Some(*class_id),
            HeapObjKind::String => Some(self.core.string),
            HeapObjKind::Array { .. } => Some(self.core.array),
            HeapObjKind::TypeObj { .. } => Some(self.core.type_cls),
            HeapObjKind::StringBuilder { .. } => Some(self.core.string_builder),
        }
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
        if let Some(cls) = self.classes.get(class_id) {
            if !cls.is_value_type {
                return val.as_obj();
            }
            let vt = cls.mono_vtable;
            let sig = cls.primitive_sig.clone().unwrap_or(SigType::ValueType(0));
            match val {
                CilVal::ValueType { ptr, size, .. } => {
                    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, size).to_vec() };
                    return self.heap.alloc_boxed(vt, class_id, sig, &bytes);
                }
                _ => {
                    let bytes = cil_val_to_bytes(&sig, val);
                    return self.heap.alloc_boxed(vt, class_id, sig, &bytes);
                }
            }
        }
        val.as_obj()
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
            class_id,
            ptr: payload_ptr as usize,
            size: cls.instance_payload_size.max(cls.elem_size),
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
            CilVal::Obj(p) => match self.heap.objects.get(&p) {
                Some(HeapObjKind::String) => self.heap.read_string(p).unwrap_or_default(),
                Some(HeapObjKind::Boxed { sig, .. }) => {
                    let v = read_sig_from_ptr(sig, (p + MONO_OBJECT_HEADER_SIZE) as *const u8);
                    self.format_val_to_string(v)
                }
                Some(HeapObjKind::Exception { class_id, message }) => {
                    format!("{}: {message}", self.classes[*class_id].full_name())
                }
                Some(HeapObjKind::StringBuilder { buf }) => buf.clone(),
                Some(HeapObjKind::TypeObj { represented_class_id }) => {
                    self.classes[*represented_class_id].full_name()
                }
                Some(HeapObjKind::Object { class_id }) => self.classes[*class_id].full_name(),
                Some(HeapObjKind::Array { elem_class_id, .. }) => {
                    format!("{}[]", self.classes[*elem_class_id].full_name())
                }
                _ => format!("Object@0x{p:x}"),
            },
            CilVal::ValueType { class_id, .. } => self
                .classes
                .get(class_id)
                .map_or_else(|| "ValueType".to_string(), |c| c.full_name()),
            CilVal::ManagedPtr(_) => String::new(),
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

fn split_full_type_name(full: &str) -> (&str, &str) {
    match full.rfind('.') {
        Some(pos) => (&full[..pos], &full[pos + 1..]),
        None => ("", full),
    }
}

pub fn default_val_for_sig(sig: &SigType) -> CilVal {
    match sig {
        SigType::Boolean
        | SigType::Char
        | SigType::I1
        | SigType::U1
        | SigType::I2
        | SigType::U2
        | SigType::I4
        | SigType::U4 => CilVal::I32(0),
        SigType::I8 | SigType::U8 => CilVal::I64(0),
        SigType::R4 | SigType::R8 => CilVal::F64(0.0),
        SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => CilVal::NativeInt(0),
        _ => CilVal::Obj(0),
    }
}

pub fn cil_val_to_bytes(sig: &SigType, val: CilVal) -> Vec<u8> {
    match sig {
        SigType::Boolean => vec![if val.is_truthy() { 1u8 } else { 0u8 }],
        SigType::I1 | SigType::U1 => vec![val.as_i32() as u8],
        SigType::Char | SigType::I2 | SigType::U2 => (val.as_i32() as u16).to_le_bytes().to_vec(),
        SigType::I4 | SigType::U4 => val.as_i32().to_le_bytes().to_vec(),
        SigType::I8 | SigType::U8 => val.as_i64().to_le_bytes().to_vec(),
        SigType::R4 => (val.as_f64() as f32).to_le_bytes().to_vec(),
        SigType::R8 => val.as_f64().to_le_bytes().to_vec(),
        SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
            (val.as_isize() as usize).to_le_bytes().to_vec()
        }
        _ => val.as_obj().to_le_bytes().to_vec(),
    }
}

pub fn read_sig_from_ptr(sig: &SigType, ptr: *const u8) -> CilVal {
    unsafe {
        match sig {
            SigType::Boolean | SigType::U1 => CilVal::I32(std::ptr::read_unaligned(ptr) as i32),
            SigType::I1 => CilVal::I32(std::ptr::read_unaligned(ptr as *const i8) as i32),
            SigType::Char | SigType::U2 => {
                CilVal::I32(std::ptr::read_unaligned(ptr as *const u16) as i32)
            }
            SigType::I2 => CilVal::I32(std::ptr::read_unaligned(ptr as *const i16) as i32),
            SigType::I4 | SigType::U4 => CilVal::I32(std::ptr::read_unaligned(ptr as *const i32)),
            SigType::I8 | SigType::U8 => CilVal::I64(std::ptr::read_unaligned(ptr as *const i64)),
            SigType::R4 => CilVal::F64(std::ptr::read_unaligned(ptr as *const f32) as f64),
            SigType::R8 => CilVal::F64(std::ptr::read_unaligned(ptr as *const f64)),
            SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
                CilVal::NativeInt(std::ptr::read_unaligned(ptr as *const isize))
            }
            _ => CilVal::Obj(std::ptr::read_unaligned(ptr as *const usize)),
        }
    }
}

pub fn write_sig_to_ptr(sig: &SigType, ptr: *mut u8, val: CilVal) {
    let bytes = cil_val_to_bytes(sig, val);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// CIL Bytecode Execution Engine (Iterative Stack Machine)
// ---------------------------------------------------------------------------

struct ExecFrame {
    method_id: usize,
    asm_id: usize,
    body: Arc<MethodBody>,
    ip: usize,
    call_ip: usize,
    args: Vec<CilVal>,
    locals: Vec<CilVal>,
    stack: Vec<CilVal>,
    /// If `Some`, `ret` pushes this value onto the caller stack (used by `newobj`).
    newobj_result: Option<CilVal>,
    /// Pending `leave` after `finally` finishes: `(target_ip, leave_ip, next_eh_idx)`.
    pending_leave: Option<(usize, usize, usize)>,
    /// Pending exception unwinding through `finally`/`fault`: `(exc_obj, throw_ip, next_eh_idx)`.
    pending_unwind: Option<(usize, usize, usize)>,
    /// Exception currently being handled inside a `catch` block (for `rethrow`).
    caught_exc: Option<usize>,
    /// Active filter clause index (for `endfilter`).
    active_filter: Option<(usize, usize, usize)>,
    /// Constrained prefix type token for the next `callvirt`.
    constrained_token: Option<u32>,
}

enum CallTarget {
    Managed(usize),
    Bcl {
        namespace: String,
        type_name: String,
        method_name: String,
        sig: MethodSig,
    },
    PInvoke {
        dll: String,
        entry: String,
        mapping_flags: u16,
        sig: MethodSig,
    },
    InternalCall {
        fn_ptr: usize,
        sig: MethodSig,
    },
    DelegateInvoke {
        sig: MethodSig,
    },
}

/// Execute a managed method (by global `method_id`) with the given CIL arguments.
/// Releases the VM mutex before any P/Invoke or internal-call native callback so
/// re-entrant `mono_*` calls never deadlock.
pub fn execute_method(method_id: usize, args: Vec<CilVal>) -> Result<Option<CilVal>, VmError> {
    let mut vm = lock_vm();
    if let Some(target) = classify_method_id(&vm, method_id) {
        match target {
            CallTarget::PInvoke {
                dll,
                entry,
                mapping_flags,
                sig,
            } => {
                let fn_ptr = resolve_pinvoke_target(&dll, &entry, mapping_flags)
                    .ok_or_else(|| VmError::MethodNotFound(format!("P/Invoke {dll}!{entry} not found")))?;
                let mut keep_ansi = Vec::new();
                let mut keep_wide = Vec::new();
                let mut byref_slots = Vec::new();
                let unicode = (mapping_flags & pinvoke_attr::CHAR_SET_MASK)
                    == pinvoke_attr::CHAR_SET_UNICODE
                    || (mapping_flags & pinvoke_attr::CHAR_SET_MASK) == pinvoke_attr::CHAR_SET_AUTO;
                let mut raw_args = [0u64; 8];
                for (i, val) in args.iter().enumerate().take(8) {
                    let pty = sig.params.get(i).unwrap_or(&SigType::IntPtr);
                    raw_args[i] = marshal_pinvoke_arg(
                        &vm,
                        &[],
                        pty,
                        *val,
                        unicode,
                        &mut keep_ansi,
                        &mut keep_wide,
                        &mut byref_slots,
                    );
                }
                drop(vm);
                let raw_ret = unsafe {
                    crate::call_guest8(
                        fn_ptr,
                        raw_args[0],
                        raw_args[1],
                        raw_args[2],
                        raw_args[3],
                        raw_args[4],
                        raw_args[5],
                        raw_args[6],
                        raw_args[7],
                    )
                };
                if sig.ret == SigType::Void {
                    return Ok(None);
                }
                let mut vm = lock_vm();
                return Ok(Some(unmarshal_native_return(&mut vm, &sig.ret, raw_ret, unicode)));
            }
            CallTarget::InternalCall { fn_ptr, sig } => {
                let mut raw_args = [0u64; 8];
                for (i, val) in args.iter().enumerate().take(8) {
                    raw_args[i] = val.as_u64_bits();
                }
                drop(vm);
                let raw_ret = unsafe {
                    crate::call_guest8(
                        fn_ptr,
                        raw_args[0],
                        raw_args[1],
                        raw_args[2],
                        raw_args[3],
                        raw_args[4],
                        raw_args[5],
                        raw_args[6],
                        raw_args[7],
                    )
                };
                return Ok(if sig.ret == SigType::Void {
                    None
                } else {
                    Some(unmarshal_icall_return(&sig.ret, raw_ret))
                });
            }
            CallTarget::Managed(_) | CallTarget::Bcl { .. } | CallTarget::DelegateInvoke { .. } => {}
        }
    }
    let initial_frame = prepare_frame(&mut vm, method_id, args, None)?;
    let mut frames = vec![initial_frame];

    'interp: loop {
        let Some(f_idx) = frames.len().checked_sub(1) else {
            return Ok(None);
        };

        // Clone lightweight instruction context
        let body = frames[f_idx].body.clone();
        let code = &body.code;
        let ip = frames[f_idx].ip;
        if ip >= code.len() {
            // Fell off end of method body -> treat as void return
            let done = frames.pop().unwrap();
            let ret_v = done.newobj_result;
            if frames.is_empty() {
                return Ok(ret_v);
            }
            if let Some(v) = ret_v {
                frames.last_mut().unwrap().stack.push(v);
            }
            continue;
        }

        let instr_ip = ip;
        let op = code[ip];
        frames[f_idx].ip = ip + 1;

        macro_rules! pop {
            () => {
                frames[f_idx].stack.pop().unwrap_or(CilVal::I32(0))
            };
        }
        macro_rules! push {
            ($v:expr) => {
                frames[f_idx].stack.push($v)
            };
        }
        macro_rules! read_i8 {
            () => {{
                let p = frames[f_idx].ip;
                let v = *code.get(p).unwrap_or(&0) as i8;
                frames[f_idx].ip = p + 1;
                v
            }};
        }
        macro_rules! read_u8 {
            () => {{
                let p = frames[f_idx].ip;
                let v = *code.get(p).unwrap_or(&0);
                frames[f_idx].ip = p + 1;
                v
            }};
        }
        macro_rules! read_u16 {
            () => {{
                let p = frames[f_idx].ip;
                let v = code
                    .get(p..p + 2)
                    .map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
                frames[f_idx].ip = p + 2;
                v
            }};
        }
        macro_rules! read_i32 {
            () => {{
                let p = frames[f_idx].ip;
                let v = code
                    .get(p..p + 4)
                    .map_or(0, |b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]));
                frames[f_idx].ip = p + 4;
                v
            }};
        }
        macro_rules! read_u32 {
            () => {
                read_i32!() as u32
            };
        }
        macro_rules! read_i64 {
            () => {{
                let p = frames[f_idx].ip;
                let v = code
                    .get(p..p + 8)
                    .map_or(0, |b| i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]));
                frames[f_idx].ip = p + 8;
                v
            }};
        }
        macro_rules! throw_exc {
            ($cls:expr, $msg:expr) => {{
                let exc_cls = $cls;
                let exc_obj = vm.make_exception(exc_cls, $msg);
                if !dispatch_exception(&vm, &mut frames, exc_obj, instr_ip) {
                    let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                    return Err(VmError::ManagedException { exc_obj, message: msg });
                }
                continue 'interp;
            }};
        }

        let constrained = if op != 0xFE {
            frames[f_idx].constrained_token.take()
        } else {
            None
        };

        match op {
            // nop, break
            0x00 | 0x01 => {}

            // ldarg.0..3
            0x02..=0x05 => {
                let idx = (op - 0x02) as usize;
                let v = frames[f_idx].args.get(idx).copied().unwrap_or(CilVal::I32(0));
                push!(v);
            }
            // ldloc.0..3
            0x06..=0x09 => {
                let idx = (op - 0x06) as usize;
                let v = frames[f_idx].locals.get(idx).copied().unwrap_or(CilVal::I32(0));
                push!(v);
            }
            // stloc.0..3
            0x0A..=0x0D => {
                let idx = (op - 0x0A) as usize;
                let v = pop!();
                store_slot(&mut frames[f_idx].locals, idx, v);
            }
            // ldarg.s
            0x0E => {
                let idx = read_u8!() as usize;
                let v = frames[f_idx].args.get(idx).copied().unwrap_or(CilVal::I32(0));
                push!(v);
            }
            // ldarga.s
            0x0F => {
                let idx = read_u8!() as usize;
                let addr = match frames[f_idx].args.get(idx) {
                    Some(CilVal::ValueType { ptr, .. }) => ManagedAddr::Raw(*ptr),
                    _ => ManagedAddr::Arg { frame: f_idx, idx },
                };
                push!(CilVal::ManagedPtr(addr));
            }
            // starg.s
            0x10 => {
                let idx = read_u8!() as usize;
                let v = pop!();
                store_slot(&mut frames[f_idx].args, idx, v);
            }
            // ldloc.s
            0x11 => {
                let idx = read_u8!() as usize;
                let v = frames[f_idx].locals.get(idx).copied().unwrap_or(CilVal::I32(0));
                push!(v);
            }
            // ldloca.s
            0x12 => {
                let idx = read_u8!() as usize;
                let addr = match frames[f_idx].locals.get(idx) {
                    Some(CilVal::ValueType { ptr, .. }) => ManagedAddr::Raw(*ptr),
                    _ => ManagedAddr::Local { frame: f_idx, idx },
                };
                push!(CilVal::ManagedPtr(addr));
            }
            // stloc.s
            0x13 => {
                let idx = read_u8!() as usize;
                let v = pop!();
                store_slot(&mut frames[f_idx].locals, idx, v);
            }
            // ldnull
            0x14 => push!(CilVal::Obj(0)),
            // ldc.i4.m1..8
            0x15..=0x1E => push!(CilVal::I32((op as i32) - 0x16)),
            // ldc.i4.s
            0x1F => {
                let v = read_i8!() as i32;
                push!(CilVal::I32(v));
            }
            // ldc.i4
            0x20 => {
                let v = read_i32!();
                push!(CilVal::I32(v));
            }
            // ldc.i8
            0x21 => {
                let v = read_i64!();
                push!(CilVal::I64(v));
            }
            // ldc.r4
            0x22 => {
                let bits = read_u32!();
                push!(CilVal::F64(f32::from_bits(bits) as f64));
            }
            // ldc.r8
            0x23 => {
                let bits = read_i64!() as u64;
                push!(CilVal::F64(f64::from_bits(bits)));
            }
            // dup
            0x25 => {
                let v = frames[f_idx].stack.last().copied().unwrap_or(CilVal::I32(0));
                push!(v);
            }
            // pop
            0x26 => {
                let _ = pop!();
            }
            // jmp
            0x27 => {
                let tok = read_u32!();
                let asm_id = frames[f_idx].asm_id;
                let args = frames[f_idx].args.clone();
                if let Some(CallTarget::Managed(mid)) = resolve_call_target(&vm, asm_id, tok, None, None) {
                    let nf = prepare_frame(&mut vm, mid, args, frames[f_idx].newobj_result)?;
                    frames[f_idx] = nf;
                }
            }
            // call / callvirt
            0x28 | 0x6F => {
                let is_virt = op == 0x6F;
                let tok = read_u32!();
                let asm_id = frames[f_idx].asm_id;
                frames[f_idx].call_ip = instr_ip;

                // Determine argument count from target signature
                let Some(sig) = resolve_method_sig_for_token(&vm, asm_id, tok) else {
                    return Err(VmError::MethodNotFound(format!("token 0x{tok:08x}")));
                };
                let argc = sig.params.len() + usize::from(sig.has_this && !sig.explicit_this);
                let mut call_args = vec![CilVal::I32(0); argc];
                for i in (0..argc).rev() {
                    call_args[i] = pop!();
                }

                // Null check on callvirt receiver (unless constrained value-type pointer)
                let recv_obj = if is_virt && sig.has_this && constrained.is_none() {
                    let r = call_args.first().copied().unwrap_or(CilVal::Obj(0));
                    match r {
                        CilVal::Obj(0) => {
                            throw_exc!(vm.core.null_ref, "NullReferenceException in callvirt");
                        }
                        CilVal::Obj(p) => Some(p),
                        _ => None,
                    }
                } else {
                    call_args.first().and_then(|v| match *v {
                        CilVal::Obj(p) if p != 0 => Some(p),
                        _ => None,
                    })
                };

                let Some(target) = resolve_call_target(&vm, asm_id, tok, if is_virt { recv_obj } else { None }, constrained) else {
                    return Err(VmError::MethodNotFound(format!("token 0x{tok:08x}")));
                };

                match target {
                    CallTarget::Managed(mid) => {
                        let callee_cls = vm.methods[mid].class_id;
                        if !vm.classes[callee_cls].cctor_ran {
                            if let Some(cctor_id) = vm.find_method_in_class(callee_cls, ".cctor", 0) {
                                vm.classes[callee_cls].cctor_ran = true;
                                // Restore args & rewind IP so this call runs right after .cctor
                                for a in call_args {
                                    frames[f_idx].stack.push(a);
                                }
                                frames[f_idx].ip = instr_ip;
                                let cf = prepare_frame(&mut vm, cctor_id, Vec::new(), None)?;
                                frames.push(cf);
                                continue 'interp;
                            }
                            vm.classes[callee_cls].cctor_ran = true;
                        }
                        let nf = prepare_frame(&mut vm, mid, call_args, None)?;
                        frames.push(nf);
                    }
                    CallTarget::Bcl {
                        namespace,
                        type_name,
                        method_name,
                        sig: bcl_sig,
                    } => {
                        match dispatch_bcl(
                            &mut vm,
                            &mut frames,
                            &namespace,
                            &type_name,
                            &method_name,
                            &bcl_sig,
                            &call_args,
                        ) {
                            Ok(Some(ret_v)) => push!(ret_v),
                            Ok(None) => {}
                            Err(VmError::ManagedException { exc_obj, message }) => {
                                let exc = if exc_obj != 0 {
                                    exc_obj
                                } else {
                                    let ec = vm.core.exception;
                                    vm.make_exception(ec, message)
                                };
                                if !dispatch_exception(&vm, &mut frames, exc, instr_ip) {
                                    let msg = vm.format_val_to_string(CilVal::Obj(exc));
                                    return Err(VmError::ManagedException {
                                        exc_obj: exc,
                                        message: msg,
                                    });
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    CallTarget::PInvoke {
                        dll,
                        entry,
                        mapping_flags,
                        sig: pi_sig,
                    } => {
                        let fn_ptr = resolve_pinvoke_target(&dll, &entry, mapping_flags)
                            .ok_or_else(|| {
                                VmError::MethodNotFound(format!("P/Invoke {dll}!{entry} not found"))
                            })?;
                        let mut keep_alive_ansi: Vec<CString> = Vec::new();
                        let mut keep_alive_wide: Vec<Vec<u16>> = Vec::new();
                        let mut byref_slots: Vec<(ManagedAddr, SigType, Box<u64>)> = Vec::new();
                        let unicode = (mapping_flags & pinvoke_attr::CHAR_SET_MASK)
                            == pinvoke_attr::CHAR_SET_UNICODE
                            || (mapping_flags & pinvoke_attr::CHAR_SET_MASK)
                                == pinvoke_attr::CHAR_SET_AUTO;
                        let mut raw_args = [0u64; 8];
                        for (i, val) in call_args.iter().enumerate().take(8) {
                            let pty = pi_sig.params.get(i).unwrap_or(&SigType::IntPtr);
                            raw_args[i] = marshal_pinvoke_arg(
                                &vm,
                                &frames,
                                pty,
                                *val,
                                unicode,
                                &mut keep_alive_ansi,
                                &mut keep_alive_wide,
                                &mut byref_slots,
                            );
                        }
                        // Release VM lock before calling native/guest code!
                        drop(vm);
                        let raw_ret = unsafe {
                            crate::call_guest8(
                                fn_ptr,
                                raw_args[0],
                                raw_args[1],
                                raw_args[2],
                                raw_args[3],
                                raw_args[4],
                                raw_args[5],
                                raw_args[6],
                                raw_args[7],
                            )
                        };
                        vm = lock_vm();
                        // Write back any byref out/ref args
                        for (addr, inner_sig, slot) in byref_slots {
                            let v = read_sig_from_ptr(&inner_sig, (&*slot as *const u64) as *const u8);
                            write_managed_addr(&mut vm, &mut frames, addr, &inner_sig, v);
                        }
                        if pi_sig.ret != SigType::Void {
                            let r = unmarshal_native_return(&mut vm, &pi_sig.ret, raw_ret, unicode);
                            push!(r);
                        }
                    }
                    CallTarget::InternalCall {
                        fn_ptr,
                        sig: ic_sig,
                    } => {
                        let mut raw_args = [0u64; 8];
                        for (i, val) in call_args.iter().enumerate().take(8) {
                            raw_args[i] = val.as_u64_bits();
                        }
                        drop(vm);
                        let raw_ret = unsafe {
                            crate::call_guest8(
                                fn_ptr,
                                raw_args[0],
                                raw_args[1],
                                raw_args[2],
                                raw_args[3],
                                raw_args[4],
                                raw_args[5],
                                raw_args[6],
                                raw_args[7],
                            )
                        };
                        vm = lock_vm();
                        if ic_sig.ret != SigType::Void {
                            let r = unmarshal_icall_return(&ic_sig.ret, raw_ret);
                            push!(r);
                        }
                    }
                    CallTarget::DelegateInvoke { sig: del_sig } => {
                        let del_obj = call_args.first().map_or(0, |v| v.as_obj());
                        let Some(HeapObjKind::Delegate {
                            method_id: target_mid,
                            target_obj,
                            ..
                        }) = vm.heap.objects.get(&del_obj).cloned()
                        else {
                            throw_exc!(vm.core.null_ref, "NullReferenceException invoking delegate");
                        };
                        let mut invoke_args = Vec::new();
                        if target_obj != 0 {
                            invoke_args.push(CilVal::Obj(target_obj));
                        }
                        invoke_args.extend_from_slice(&call_args[1..]);
                        let _ = del_sig;
                        let nf = prepare_frame(&mut vm, target_mid, invoke_args, None)?;
                        frames.push(nf);
                    }
                }
            }
            // calli
            0x29 => {
                let sig_tok = read_u32!();
                let fn_ptr = pop!().as_isize() as usize;
                let asm_id = frames[f_idx].asm_id;
                let sig = vm.assemblies[asm_id]
                    .image
                    .as_ref()
                    .and_then(|img| img.parse_standalone_method_sig(sig_tok))
                    .unwrap_or(MethodSig {
                        has_this: false,
                        explicit_this: false,
                        ret: SigType::Void,
                        params: Vec::new(),
                    });
                let argc = sig.params.len();
                let mut raw_args = [0u64; 8];
                for i in (0..argc).rev() {
                    let v = pop!();
                    if i < 8 {
                        raw_args[i] = v.as_u64_bits();
                    }
                }
                if fn_ptr == 0 {
                    throw_exc!(vm.core.null_ref, "NullReferenceException in calli");
                }
                drop(vm);
                let raw_ret = unsafe {
                    crate::call_guest8(
                        fn_ptr,
                        raw_args[0],
                        raw_args[1],
                        raw_args[2],
                        raw_args[3],
                        raw_args[4],
                        raw_args[5],
                        raw_args[6],
                        raw_args[7],
                    )
                };
                vm = lock_vm();
                if sig.ret != SigType::Void {
                    push!(unmarshal_icall_return(&sig.ret, raw_ret));
                }
            }
            // ret
            0x2A => {
                let finished = frames.pop().unwrap();
                let ret_v = if let Some(newobj) = finished.newobj_result {
                    Some(newobj)
                } else {
                    let ret_sig = &vm.methods[finished.method_id].sig.ret;
                    if *ret_sig == SigType::Void {
                        None
                    } else {
                        Some(finished.stack.last().copied().unwrap_or(CilVal::I32(0)))
                    }
                };
                if frames.is_empty() {
                    return Ok(ret_v);
                }
                if let Some(v) = ret_v {
                    frames.last_mut().unwrap().stack.push(v);
                }
            }

            // Short & long branches (0x2B..0x44)
            0x2B => {
                let d = read_i8!() as isize;
                frames[f_idx].ip = (frames[f_idx].ip as isize + d) as usize;
            }
            0x38 => {
                let d = read_i32!() as isize;
                frames[f_idx].ip = (frames[f_idx].ip as isize + d) as usize;
            }
            0x2C | 0x39 => {
                let d = if op == 0x2C { read_i8!() as isize } else { read_i32!() as isize };
                if !pop!().is_truthy() {
                    frames[f_idx].ip = (frames[f_idx].ip as isize + d) as usize;
                }
            }
            0x2D | 0x3A => {
                let d = if op == 0x2D { read_i8!() as isize } else { read_i32!() as isize };
                if pop!().is_truthy() {
                    frames[f_idx].ip = (frames[f_idx].ip as isize + d) as usize;
                }
            }
            0x2E..=0x37 | 0x3B..=0x44 => {
                let (kind, d) = if op <= 0x37 {
                    (op - 0x2E, read_i8!() as isize)
                } else {
                    (op - 0x3B, read_i32!() as isize)
                };
                let b = pop!();
                let a = pop!();
                if eval_branch_cmp(kind, a, b) {
                    frames[f_idx].ip = (frames[f_idx].ip as isize + d) as usize;
                }
            }
            // switch
            0x45 => {
                let count = read_u32!() as usize;
                let base_table = frames[f_idx].ip;
                let next_ip = base_table + count * 4;
                frames[f_idx].ip = next_ip;
                let idx = pop!().as_i32() as u32 as usize;
                if idx < count {
                    let o = base_table + idx * 4;
                    if let Some(bytes) = code.get(o..o + 4) {
                        let rel = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as isize;
                        frames[f_idx].ip = (next_ip as isize + rel) as usize;
                    }
                }
            }

            // ldind.* (0x46..0x50)
            0x46..=0x50 => {
                let ptr_val = pop!();
                let sig = match op {
                    0x46 => SigType::I1,
                    0x47 => SigType::U1,
                    0x48 => SigType::I2,
                    0x49 => SigType::U2,
                    0x4A => SigType::I4,
                    0x4B => SigType::U4,
                    0x4C => SigType::I8,
                    0x4D => SigType::IntPtr,
                    0x4E => SigType::R4,
                    0x4F => SigType::R8,
                    _ => SigType::Object,
                };
                let v = read_ptr_val(&vm, &frames, ptr_val, &sig);
                push!(v);
            }
            // stind.* (0x51..0x57, 0xDF)
            0x51..=0x57 | 0xDF => {
                let val = pop!();
                let addr_val = pop!();
                let sig = match op {
                    0x51 => SigType::Object,
                    0x52 => SigType::I1,
                    0x53 => SigType::I2,
                    0x54 => SigType::I4,
                    0x55 => SigType::I8,
                    0x56 => SigType::R4,
                    0x57 => SigType::R8,
                    _ => SigType::IntPtr,
                };
                write_ptr_val(&mut vm, &mut frames, addr_val, &sig, val);
            }

            // Arithmetic & bitwise (0x58..0x66)
            0x58 => {
                let b = pop!();
                let a = pop!();
                push!(arith_bin(a, b, 0));
            }
            0x59 => {
                let b = pop!();
                let a = pop!();
                push!(arith_bin(a, b, 1));
            }
            0x5A => {
                let b = pop!();
                let a = pop!();
                push!(arith_bin(a, b, 2));
            }
            0x5B => {
                let b = pop!();
                let a = pop!();
                match arith_div(a, b, false) {
                    Ok(v) => push!(v),
                    Err(true) => throw_exc!(vm.core.div_by_zero, "DivideByZeroException"),
                    Err(false) => throw_exc!(vm.core.overflow, "OverflowException in div"),
                }
            }
            0x5C => {
                let b = pop!();
                let a = pop!();
                match arith_div(a, b, true) {
                    Ok(v) => push!(v),
                    Err(_) => throw_exc!(vm.core.div_by_zero, "DivideByZeroException"),
                }
            }
            0x5D => {
                let b = pop!();
                let a = pop!();
                match arith_rem(a, b, false) {
                    Ok(v) => push!(v),
                    Err(true) => throw_exc!(vm.core.div_by_zero, "DivideByZeroException"),
                    Err(false) => throw_exc!(vm.core.overflow, "OverflowException in rem"),
                }
            }
            0x5E => {
                let b = pop!();
                let a = pop!();
                match arith_rem(a, b, true) {
                    Ok(v) => push!(v),
                    Err(_) => throw_exc!(vm.core.div_by_zero, "DivideByZeroException"),
                }
            }
            0x5F => {
                let b = pop!();
                let a = pop!();
                push!(bitwise_bin(a, b, 0));
            }
            0x60 => {
                let b = pop!();
                let a = pop!();
                push!(bitwise_bin(a, b, 1));
            }
            0x61 => {
                let b = pop!();
                let a = pop!();
                push!(bitwise_bin(a, b, 2));
            }
            0x62 => {
                let b = pop!();
                let a = pop!();
                push!(shift_bin(a, b, 0));
            }
            0x63 => {
                let b = pop!();
                let a = pop!();
                push!(shift_bin(a, b, 1));
            }
            0x64 => {
                let b = pop!();
                let a = pop!();
                push!(shift_bin(a, b, 2));
            }
            0x65 => {
                let a = pop!();
                let r = match a {
                    CilVal::I32(v) => CilVal::I32(v.wrapping_neg()),
                    CilVal::I64(v) => CilVal::I64(v.wrapping_neg()),
                    CilVal::NativeInt(v) => CilVal::NativeInt(v.wrapping_neg()),
                    CilVal::F64(v) => CilVal::F64(-v),
                    _ => CilVal::I32(0),
                };
                push!(r);
            }
            0x66 => {
                let a = pop!();
                let r = match a {
                    CilVal::I32(v) => CilVal::I32(!v),
                    CilVal::I64(v) => CilVal::I64(!v),
                    CilVal::NativeInt(v) => CilVal::NativeInt(!v),
                    _ => CilVal::I32(0),
                };
                push!(r);
            }

            // Conversions (unchecked)
            0x67 => {
                let v = pop!();
                push!(CilVal::I32((v.as_i64() as i8) as i32));
            }
            0x68 => {
                let v = pop!();
                push!(CilVal::I32((v.as_i64() as i16) as i32));
            }
            0x69 => {
                let v = pop!();
                push!(CilVal::I32(v.as_i32()));
            }
            0x6A => {
                let v = pop!();
                push!(CilVal::I64(v.as_i64()));
            }
            0x6B => {
                let v = pop!();
                push!(CilVal::F64((v.as_f64() as f32) as f64));
            }
            0x6C => {
                let v = pop!();
                push!(CilVal::F64(v.as_f64()));
            }
            0x6D => {
                let v = pop!();
                push!(CilVal::I32(v.as_u64_bits() as u32 as i32));
            }
            0x6E => {
                let v = pop!();
                let r = match v {
                    CilVal::I32(x) => (x as u32) as i64,
                    CilVal::I64(x) => x,
                    CilVal::NativeInt(x) => (x as usize) as i64,
                    CilVal::F64(x) => (x as u64) as i64,
                    _ => v.as_u64_bits() as i64,
                };
                push!(CilVal::I64(r));
            }
            0x76 => {
                // conv.r.un
                let v = pop!();
                let f = match v {
                    CilVal::I32(x) => (x as u32) as f64,
                    CilVal::I64(x) => (x as u64) as f64,
                    CilVal::NativeInt(x) => (x as usize) as f64,
                    CilVal::F64(x) => x,
                    _ => v.as_u64_bits() as f64,
                };
                push!(CilVal::F64(f));
            }
            0xD1 => {
                let v = pop!();
                push!(CilVal::I32((v.as_u64_bits() as u16) as i32));
            }
            0xD2 => {
                let v = pop!();
                push!(CilVal::I32((v.as_u64_bits() as u8) as i32));
            }
            0xD3 => {
                let v = pop!();
                push!(CilVal::NativeInt(v.as_isize()));
            }
            0xE0 => {
                let v = pop!();
                push!(CilVal::NativeInt(v.as_u64_bits() as usize as isize));
            }

            // Overflow-checked conversions (0x82..0x8A, 0xB3..0xBC, 0xD4..0xD5)
            0x82..=0x8A | 0xB3..=0xBC | 0xD4 | 0xD5 => {
                let v = pop!();
                match conv_checked(op, v) {
                    Ok(r) => push!(r),
                    Err(()) => throw_exc!(vm.core.overflow, "OverflowException in conv.ovf"),
                }
            }
            // Overflow-checked arithmetic (0xD6..0xDB)
            0xD6..=0xDB => {
                let b = pop!();
                let a = pop!();
                match arith_checked(op, a, b) {
                    Ok(r) => push!(r),
                    Err(()) => throw_exc!(vm.core.overflow, "OverflowException in arithmetic"),
                }
            }

            // cpobj, ldobj, stobj
            0x70 => {
                let tok = read_u32!();
                let src = pop!();
                let dst = pop!();
                let asm_id = frames[f_idx].asm_id;
                let sz = vm
                    .resolve_type_token(asm_id, tok)
                    .map_or(8, |cid| vm.classes[cid].elem_size);
                if let (Some(sp), Some(dp)) = (raw_ptr_from_val(src), raw_ptr_from_val(dst)) {
                    unsafe {
                        std::ptr::copy(sp as *const u8, dp as *mut u8, sz);
                    }
                }
            }
            0x71 => {
                let tok = read_u32!();
                let src = pop!();
                let asm_id = frames[f_idx].asm_id;
                let cid = vm.resolve_type_token(asm_id, tok).unwrap_or(vm.core.object);
                let sig = vm.classes[cid]
                    .primitive_sig
                    .clone()
                    .unwrap_or(SigType::ValueType(tok));
                if vm.classes[cid].primitive_sig.is_some() || !vm.classes[cid].is_value_type {
                    let v = read_ptr_val(&vm, &frames, src, &sig);
                    push!(v);
                } else {
                    let sz = vm.classes[cid].instance_payload_size.max(vm.classes[cid].elem_size);
                    let dst_buf = vm.heap.alloc_raw_struct(cid, sz);
                    if let Some(sp) = raw_ptr_from_val(src) {
                        unsafe {
                            std::ptr::copy_nonoverlapping(sp as *const u8, dst_buf as *mut u8, sz);
                        }
                    }
                    push!(CilVal::ValueType {
                        class_id: cid,
                        ptr: dst_buf,
                        size: sz,
                    });
                }
            }
            0x81 => {
                let tok = read_u32!();
                let val = pop!();
                let dst = pop!();
                let asm_id = frames[f_idx].asm_id;
                let cid = vm.resolve_type_token(asm_id, tok).unwrap_or(vm.core.object);
                if let CilVal::ValueType { ptr: sp, size, .. } = val {
                    if let Some(dp) = raw_ptr_from_val(dst) {
                        unsafe {
                            std::ptr::copy_nonoverlapping(sp as *const u8, dp as *mut u8, size);
                        }
                    }
                } else {
                    let sig = vm.classes[cid].primitive_sig.clone().unwrap_or(SigType::Object);
                    write_ptr_val(&mut vm, &mut frames, dst, &sig, val);
                }
            }

            // ldstr
            0x72 => {
                let tok = read_u32!();
                let asm_id = frames[f_idx].asm_id;
                let u16s = vm.assemblies[asm_id]
                    .image
                    .as_ref()
                    .map(|img| img.get_user_string(tok))
                    .unwrap_or_default();
                let obj = vm.alloc_string_utf16(&u16s);
                push!(CilVal::Obj(obj));
            }

            // newobj
            0x73 => {
                let tok = read_u32!();
                let asm_id = frames[f_idx].asm_id;
                frames[f_idx].call_ip = instr_ip;
                let Some(sig) = resolve_method_sig_for_token(&vm, asm_id, tok) else {
                    return Err(VmError::MethodNotFound(format!("newobj token 0x{tok:08x}")));
                };
                let argc = sig.params.len();
                let mut ctor_args = vec![CilVal::I32(0); argc];
                for i in (0..argc).rev() {
                    ctor_args[i] = pop!();
                }

                let Some(target) = resolve_call_target(&vm, asm_id, tok, None, None) else {
                    return Err(VmError::MethodNotFound(format!("newobj token 0x{tok:08x}")));
                };

                match target {
                    CallTarget::Managed(ctor_mid) => {
                        let cls_id = vm.methods[ctor_mid].class_id;
                        if vm.classes[cls_id].is_delegate {
                            let target_obj = ctor_args.first().map_or(0, |v| v.as_obj());
                            let method_id = ctor_args.get(1).map_or(0, |v| v.as_isize() as usize);
                            let vt = vm.classes[cls_id].mono_vtable;
                            let ptr = vm.heap.alloc_raw(MONO_OBJECT_HEADER_SIZE);
                            unsafe {
                                std::ptr::write_unaligned(ptr as *mut usize, vt);
                            }
                            let addr = ptr as usize;
                            vm.heap.objects.insert(
                                addr,
                                HeapObjKind::Delegate {
                                    class_id: cls_id,
                                    method_id,
                                    target_obj,
                                },
                            );
                            push!(CilVal::Obj(addr));
                        } else if vm.classes[cls_id].is_value_type {
                            let sz = vm.classes[cls_id]
                                .instance_payload_size
                                .max(vm.classes[cls_id].elem_size);
                            let buf = vm.heap.alloc_raw_struct(cls_id, sz);
                            let mut all = Vec::with_capacity(argc + 1);
                            all.push(CilVal::ManagedPtr(ManagedAddr::Raw(buf)));
                            all.extend(ctor_args);
                            let result_val = CilVal::ValueType {
                                class_id: cls_id,
                                ptr: buf,
                                size: sz,
                            };
                            let nf = prepare_frame(&mut vm, ctor_mid, all, Some(result_val))?;
                            frames.push(nf);
                        } else {
                            let obj = vm.alloc_object_for_class(cls_id);
                            let mut all = Vec::with_capacity(argc + 1);
                            all.push(CilVal::Obj(obj));
                            all.extend(ctor_args);
                            let nf = prepare_frame(&mut vm, ctor_mid, all, Some(CilVal::Obj(obj)))?;
                            frames.push(nf);
                        }
                    }
                    CallTarget::Bcl {
                        namespace,
                        type_name,
                        method_name: _,
                        sig: bcl_sig,
                    } => {
                        let obj = dispatch_bcl_newobj(&mut vm, &namespace, &type_name, &bcl_sig, &ctor_args)?;
                        push!(obj);
                    }
                    _ => {
                        return Err(VmError::Unsupported(format!(
                            "newobj target 0x{tok:08x} is not a constructor"
                        )));
                    }
                }
            }

            // castclass / isinst
            0x74 | 0x75 => {
                let tok = read_u32!();
                let val = pop!();
                let obj = val.as_obj();
                let asm_id = frames[f_idx].asm_id;
                let target_cid = vm.resolve_type_token(asm_id, tok).unwrap_or(vm.core.object);
                if obj == 0 {
                    push!(CilVal::Obj(0));
                } else if vm.is_instance_of(obj, target_cid) {
                    push!(CilVal::Obj(obj));
                } else if op == 0x75 {
                    push!(CilVal::Obj(0));
                } else {
                    let tname = vm.classes[target_cid].full_name();
                    throw_exc!(vm.core.invalid_cast, format!("InvalidCastException to {tname}"));
                }
            }

            // unbox
            0x79 => {
                let _tok = read_u32!();
                let obj = pop!().as_obj();
                if obj == 0 {
                    throw_exc!(vm.core.null_ref, "NullReferenceException in unbox");
                }
                push!(CilVal::ManagedPtr(ManagedAddr::Raw(
                    obj + MONO_OBJECT_HEADER_SIZE
                )));
            }

            // throw
            0x7A => {
                let exc = pop!().as_obj();
                let exc_obj = if exc == 0 {
                    let nr = vm.core.null_ref;
                    vm.make_exception(nr, "NullReferenceException (throw null)")
                } else {
                    exc
                };
                if !dispatch_exception(&vm, &mut frames, exc_obj, instr_ip) {
                    let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                    return Err(VmError::ManagedException {
                        exc_obj,
                        message: msg,
                    });
                }
            }

            // ldfld / ldflda / stfld
            0x7B | 0x7C | 0x7D => {
                let tok = read_u32!();
                let asm_id = frames[f_idx].asm_id;
                let Some(fid) = resolve_field_token(&vm, asm_id, tok) else {
                    return Err(VmError::Unsupported(format!("field token 0x{tok:08x} not found")));
                };
                let is_static = vm.fields[fid].is_static;
                let f_off = vm.fields[fid].struct_offset;
                let f_sig = vm.fields[fid].sig.clone();

                if op == 0x7D {
                    let val = pop!();
                    let target = pop!();
                    if is_static {
                        vm.fields[fid].static_value = val;
                    } else {
                        let Some(base) = field_base_ptr(target) else {
                            throw_exc!(vm.core.null_ref, "NullReferenceException in stfld");
                        };
                        write_sig_to_ptr(&f_sig, (base + f_off) as *mut u8, val);
                    }
                } else {
                    let target = pop!();
                    if is_static {
                        if op == 0x7C {
                            push!(CilVal::ManagedPtr(ManagedAddr::StaticField(fid)));
                        } else {
                            push!(vm.fields[fid].static_value);
                        }
                    } else {
                        let Some(base) = field_base_ptr(target) else {
                            throw_exc!(vm.core.null_ref, "NullReferenceException in ldfld");
                        };
                        if op == 0x7C {
                            push!(CilVal::ManagedPtr(ManagedAddr::Raw(base + f_off)));
                        } else {
                            push!(read_sig_from_ptr(&f_sig, (base + f_off) as *const u8));
                        }
                    }
                }
            }

            // ldsfld / ldsflda / stsfld
            0x7E | 0x7F | 0x80 => {
                let tok = read_u32!();
                let asm_id = frames[f_idx].asm_id;
                if let Some(fid) = resolve_field_token(&vm, asm_id, tok) {
                    let cid = vm.fields[fid].class_id;
                    if !vm.classes[cid].cctor_ran {
                        if let Some(cctor_id) = vm.find_method_in_class(cid, ".cctor", 0) {
                            vm.classes[cid].cctor_ran = true;
                            frames[f_idx].ip = instr_ip;
                            let cf = prepare_frame(&mut vm, cctor_id, Vec::new(), None)?;
                            frames.push(cf);
                            continue 'interp;
                        }
                        vm.classes[cid].cctor_ran = true;
                    }
                    match op {
                        0x7E => push!(vm.fields[fid].static_value),
                        0x7F => push!(CilVal::ManagedPtr(ManagedAddr::StaticField(fid))),
                        _ => {
                            let val = pop!();
                            vm.fields[fid].static_value = val;
                        }
                    }
                } else if let Some(bcl_val) = resolve_bcl_static_field(&mut vm, asm_id, tok) {
                    if op == 0x7E {
                        push!(bcl_val);
                    } else if op == 0x80 {
                        let _ = pop!();
                    } else {
                        push!(CilVal::ManagedPtr(ManagedAddr::Raw(0)));
                    }
                } else {
                    return Err(VmError::Unsupported(format!(
                        "static field token 0x{tok:08x} not found"
                    )));
                }
            }

            // box
            0x8C => {
                let tok = read_u32!();
                let val = pop!();
                let asm_id = frames[f_idx].asm_id;
                let cid = vm.resolve_type_token(asm_id, tok).unwrap_or(vm.core.object);
                let boxed = vm.box_value(cid, val);
                push!(CilVal::Obj(boxed));
            }

            // newarr
            0x8D => {
                let tok = read_u32!();
                let len = pop!().as_i32();
                if len < 0 {
                    throw_exc!(vm.core.overflow, "OverflowException: negative array length");
                }
                let asm_id = frames[f_idx].asm_id;
                let elem_cid = vm.resolve_type_token(asm_id, tok).unwrap_or(vm.core.object);
                let arr = vm.alloc_array_for_class(elem_cid, len as usize);
                push!(CilVal::Obj(arr));
            }

            // ldlen
            0x8E => {
                let arr = pop!().as_obj();
                if arr == 0 {
                    throw_exc!(vm.core.null_ref, "NullReferenceException in ldlen");
                }
                let len = match vm.heap.objects.get(&arr) {
                    Some(HeapObjKind::Array { length, .. }) => *length,
                    _ => unsafe {
                        std::ptr::read_unaligned((arr + MONO_ARRAY_LEN_OFFSET) as *const usize)
                    },
                };
                push!(CilVal::NativeInt(len as isize));
            }

            // ldelema
            0x8F => {
                let _tok = read_u32!();
                let idx = pop!().as_isize();
                let arr = pop!().as_obj();
                match array_elem_ptr(&vm, arr, idx) {
                    Ok((ep, _)) => push!(CilVal::ManagedPtr(ManagedAddr::Raw(ep))),
                    Err(true) => throw_exc!(vm.core.null_ref, "NullReferenceException in ldelema"),
                    Err(false) => {
                        throw_exc!(vm.core.index_out_of_range, "IndexOutOfRangeException in ldelema")
                    }
                }
            }

            // ldelem.* (0x90..0x9A) and ldelem <tok> (0xA3)
            0x90..=0x9A | 0xA3 => {
                let tok = if op == 0xA3 { Some(read_u32!()) } else { None };
                let idx = pop!().as_isize();
                let arr = pop!().as_obj();
                match array_elem_ptr(&vm, arr, idx) {
                    Ok((ep, _)) => {
                        let sig = match op {
                            0x90 => SigType::I1,
                            0x91 => SigType::U1,
                            0x92 => SigType::I2,
                            0x93 => SigType::U2,
                            0x94 => SigType::I4,
                            0x95 => SigType::U4,
                            0x96 => SigType::I8,
                            0x97 => SigType::IntPtr,
                            0x98 => SigType::R4,
                            0x99 => SigType::R8,
                            0x9A => SigType::Object,
                            _ => {
                                let asm_id = frames[f_idx].asm_id;
                                let cid = tok
                                    .and_then(|t| vm.resolve_type_token(asm_id, t))
                                    .unwrap_or(vm.core.object);
                                vm.classes[cid].primitive_sig.clone().unwrap_or(SigType::Object)
                            }
                        };
                        push!(read_sig_from_ptr(&sig, ep as *const u8));
                    }
                    Err(true) => throw_exc!(vm.core.null_ref, "NullReferenceException in ldelem"),
                    Err(false) => {
                        throw_exc!(vm.core.index_out_of_range, "IndexOutOfRangeException in ldelem")
                    }
                }
            }

            // stelem.* (0x9B..0xA2) and stelem <tok> (0xA4)
            0x9B..=0xA2 | 0xA4 => {
                let tok = if op == 0xA4 { Some(read_u32!()) } else { None };
                let val = pop!();
                let idx = pop!().as_isize();
                let arr = pop!().as_obj();
                match array_elem_ptr(&vm, arr, idx) {
                    Ok((ep, elem_sz)) => {
                        if let CilVal::ValueType { ptr: sp, size, .. } = val {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    sp as *const u8,
                                    ep as *mut u8,
                                    size.min(elem_sz),
                                );
                            }
                        } else {
                            let sig = match op {
                                0x9B => SigType::IntPtr,
                                0x9C => SigType::I1,
                                0x9D => SigType::I2,
                                0x9E => SigType::I4,
                                0x9F => SigType::I8,
                                0xA0 => SigType::R4,
                                0xA1 => SigType::R8,
                                0xA2 => SigType::Object,
                                _ => {
                                    let asm_id = frames[f_idx].asm_id;
                                    let cid = tok
                                        .and_then(|t| vm.resolve_type_token(asm_id, t))
                                        .unwrap_or(vm.core.object);
                                    vm.classes[cid].primitive_sig.clone().unwrap_or(SigType::Object)
                                }
                            };
                            write_sig_to_ptr(&sig, ep as *mut u8, val);
                        }
                    }
                    Err(true) => throw_exc!(vm.core.null_ref, "NullReferenceException in stelem"),
                    Err(false) => {
                        throw_exc!(vm.core.index_out_of_range, "IndexOutOfRangeException in stelem")
                    }
                }
            }

            // unbox.any
            0xA5 => {
                let tok = read_u32!();
                let obj = pop!().as_obj();
                let asm_id = frames[f_idx].asm_id;
                let cid = vm.resolve_type_token(asm_id, tok).unwrap_or(vm.core.object);
                match vm.unbox_any(obj, cid) {
                    Ok(v) => push!(v),
                    Err(VmError::ManagedException { message, .. }) => {
                        let e_cls = if obj == 0 {
                            vm.core.null_ref
                        } else {
                            vm.core.invalid_cast
                        };
                        throw_exc!(e_cls, message);
                    }
                    Err(e) => return Err(e),
                }
            }

            // ckfinite
            0xC3 => {
                let v = frames[f_idx].stack.last().copied().unwrap_or(CilVal::F64(0.0)).as_f64();
                if !v.is_finite() {
                    throw_exc!(vm.core.overflow, "ArithmeticException: non-finite float");
                }
            }

            // ldtoken
            0xD0 => {
                let tok = read_u32!();
                let asm_id = frames[f_idx].asm_id;
                // Pack (asm_id << 32) | tok into NativeInt so RuntimeHelpers/Type.GetTypeFromHandle resolve cleanly.
                let packed = (((asm_id as u64) << 32) | (tok as u64)) as isize;
                push!(CilVal::NativeInt(packed));
            }

            // endfinally / endfault
            0xDC => {
                frames[f_idx].stack.clear();
                if let Some((target, leave_ip, next_idx)) = frames[f_idx].pending_leave.take() {
                    if let Some((handler, idx_after)) =
                        find_exited_finally(&body.eh_clauses, leave_ip, target, next_idx)
                    {
                        frames[f_idx].pending_leave = Some((target, leave_ip, idx_after));
                        frames[f_idx].ip = handler;
                    } else {
                        frames[f_idx].ip = target;
                    }
                } else if let Some((exc_obj, throw_ip, next_idx)) = frames[f_idx].pending_unwind.take() {
                    if !dispatch_exception_from(&vm, &mut frames, exc_obj, throw_ip, next_idx) {
                        let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                        return Err(VmError::ManagedException {
                            exc_obj,
                            message: msg,
                        });
                    }
                }
            }

            // leave / leave.s
            0xDD | 0xDE => {
                let d = if op == 0xDE {
                    read_i8!() as isize
                } else {
                    read_i32!() as isize
                };
                let target = (frames[f_idx].ip as isize + d) as usize;
                frames[f_idx].stack.clear();
                frames[f_idx].caught_exc = None;
                if let Some((handler, idx_after)) =
                    find_exited_finally(&body.eh_clauses, instr_ip, target, 0)
                {
                    frames[f_idx].pending_leave = Some((target, instr_ip, idx_after));
                    frames[f_idx].ip = handler;
                } else {
                    frames[f_idx].ip = target;
                }
            }

            // Extended 0xFE opcodes
            0xFE => {
                let op2 = read_u8!();
                match op2 {
                    // ceq, cgt, cgt.un, clt, clt.un
                    0x01..=0x05 => {
                        let b = pop!();
                        let a = pop!();
                        let res = eval_cmp_op(op2, a, b);
                        push!(CilVal::I32(i32::from(res)));
                    }
                    // ldftn
                    0x06 => {
                        let tok = read_u32!();
                        let asm_id = frames[f_idx].asm_id;
                        let mid = match resolve_call_target(&vm, asm_id, tok, None, None) {
                            Some(CallTarget::Managed(id)) => id,
                            _ => 0,
                        };
                        push!(CilVal::NativeInt(mid as isize));
                    }
                    // ldvirtftn
                    0x07 => {
                        let tok = read_u32!();
                        let obj = pop!().as_obj();
                        let asm_id = frames[f_idx].asm_id;
                        let mid = match resolve_call_target(&vm, asm_id, tok, Some(obj), None) {
                            Some(CallTarget::Managed(id)) => id,
                            _ => 0,
                        };
                        push!(CilVal::NativeInt(mid as isize));
                    }
                    // ldarg
                    0x09 => {
                        let idx = read_u16!() as usize;
                        let v = frames[f_idx].args.get(idx).copied().unwrap_or(CilVal::I32(0));
                        push!(v);
                    }
                    // ldarga
                    0x0A => {
                        let idx = read_u16!() as usize;
                        let addr = match frames[f_idx].args.get(idx) {
                            Some(CilVal::ValueType { ptr, .. }) => ManagedAddr::Raw(*ptr),
                            _ => ManagedAddr::Arg { frame: f_idx, idx },
                        };
                        push!(CilVal::ManagedPtr(addr));
                    }
                    // starg
                    0x0B => {
                        let idx = read_u16!() as usize;
                        let v = pop!();
                        store_slot(&mut frames[f_idx].args, idx, v);
                    }
                    // ldloc
                    0x0C => {
                        let idx = read_u16!() as usize;
                        let v = frames[f_idx].locals.get(idx).copied().unwrap_or(CilVal::I32(0));
                        push!(v);
                    }
                    // ldloca
                    0x0D => {
                        let idx = read_u16!() as usize;
                        let addr = match frames[f_idx].locals.get(idx) {
                            Some(CilVal::ValueType { ptr, .. }) => ManagedAddr::Raw(*ptr),
                            _ => ManagedAddr::Local { frame: f_idx, idx },
                        };
                        push!(CilVal::ManagedPtr(addr));
                    }
                    // stloc
                    0x0E => {
                        let idx = read_u16!() as usize;
                        let v = pop!();
                        store_slot(&mut frames[f_idx].locals, idx, v);
                    }
                    // localloc
                    0x0F => {
                        let sz = pop!().as_isize().max(0) as usize;
                        let ptr = vm.heap.alloc_raw(sz.max(8));
                        push!(CilVal::NativeInt(ptr as usize as isize));
                    }
                    // endfilter
                    0x11 => {
                        let decision = pop!().as_i32();
                        if let Some((exc_obj, throw_ip, clause_idx)) = frames[f_idx].active_filter.take() {
                            if decision != 0 {
                                let handler = body.eh_clauses[clause_idx].handler_offset;
                                frames[f_idx].stack.clear();
                                frames[f_idx].stack.push(CilVal::Obj(exc_obj));
                                frames[f_idx].caught_exc = Some(exc_obj);
                                frames[f_idx].ip = handler;
                            } else if !dispatch_exception_from(
                                &vm,
                                &mut frames,
                                exc_obj,
                                throw_ip,
                                clause_idx + 1,
                            ) {
                                let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                                return Err(VmError::ManagedException {
                                    exc_obj,
                                    message: msg,
                                });
                            }
                        }
                    }
                    // unaligned., no. (1-byte immediate prefix)
                    0x12 | 0x19 => {
                        let _ = read_u8!();
                    }
                    // volatile., tail., readonly. (no-op prefixes)
                    0x13 | 0x14 | 0x1E => {}
                    // initobj
                    0x15 => {
                        let tok = read_u32!();
                        let dst = pop!();
                        let asm_id = frames[f_idx].asm_id;
                        let cid = vm.resolve_type_token(asm_id, tok).unwrap_or(vm.core.object);
                        let sz = vm.classes[cid]
                            .instance_payload_size
                            .max(vm.classes[cid].elem_size);
                        match dst {
                            CilVal::ManagedPtr(ManagedAddr::Raw(p)) if p != 0 => unsafe {
                                std::ptr::write_bytes(p as *mut u8, 0, sz);
                            },
                            CilVal::ManagedPtr(addr) => {
                                let sig = vm.classes[cid]
                                    .primitive_sig
                                    .clone()
                                    .unwrap_or(SigType::Object);
                                write_managed_addr(&mut vm, &mut frames, addr, &sig, default_val_for_sig(&sig));
                            }
                            _ => {}
                        }
                    }
                    // constrained.
                    0x16 => {
                        let tok = read_u32!();
                        frames[f_idx].constrained_token = Some(tok);
                    }
                    // cpblk
                    0x17 => {
                        let len = pop!().as_i32() as u32 as usize;
                        let src = pop!().as_isize() as usize;
                        let dst = pop!().as_isize() as usize;
                        if len > 0 && src != 0 && dst != 0 {
                            unsafe {
                                std::ptr::copy(src as *const u8, dst as *mut u8, len);
                            }
                        }
                    }
                    // initblk
                    0x18 => {
                        let len = pop!().as_i32() as u32 as usize;
                        let val = pop!().as_i32() as u8;
                        let dst = pop!().as_isize() as usize;
                        if len > 0 && dst != 0 {
                            unsafe {
                                std::ptr::write_bytes(dst as *mut u8, val, len);
                            }
                        }
                    }
                    // rethrow
                    0x1A => {
                        let exc_obj = frames[f_idx].caught_exc.unwrap_or_else(|| {
                            let ec = vm.core.exception;
                            vm.make_exception(ec, "rethrow")
                        });
                        if !dispatch_exception(&vm, &mut frames, exc_obj, instr_ip) {
                            let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                            return Err(VmError::ManagedException {
                                exc_obj,
                                message: msg,
                            });
                        }
                    }
                    // sizeof
                    0x1C => {
                        let tok = read_u32!();
                        let asm_id = frames[f_idx].asm_id;
                        let sz = vm
                            .resolve_type_token(asm_id, tok)
                            .map_or(8, |cid| vm.classes[cid].elem_size);
                        push!(CilVal::I32(sz as i32));
                    }
                    other => {
                        return Err(VmError::Unsupported(format!(
                            "unimplemented CIL opcode 0xFE 0x{other:02x} at ip=0x{instr_ip:x}"
                        )));
                    }
                }
            }

            other => {
                return Err(VmError::Unsupported(format!(
                    "unimplemented CIL opcode 0x{other:02x} at ip=0x{instr_ip:x}"
                )));
            }
        }
    }
}

fn prepare_frame(
    vm: &mut ClrVm,
    method_id: usize,
    args: Vec<CilVal>,
    newobj_result: Option<CilVal>,
) -> Result<ExecFrame, VmError> {
    let m = vm
        .methods
        .get(method_id)
        .ok_or_else(|| VmError::MethodNotFound(format!("method_id {method_id}")))?;
    let asm_id = m.asm_id;
    let body = m.body.clone().ok_or_else(|| {
        VmError::MethodNotFound(format!("method {} has no IL body", m.name))
    })?;
    let local_sigs = m.locals_sig.clone();
    let mut locals = Vec::with_capacity(local_sigs.len());
    for lsig in &local_sigs {
        let v = match lsig {
            SigType::ValueType(tok) | SigType::GenericInst(true, tok, _) => {
                if let Some(cid) = vm.resolve_type_token(asm_id, *tok) {
                    if let Some(ps) = &vm.classes[cid].primitive_sig {
                        default_val_for_sig(ps)
                    } else {
                        let sz = vm.classes[cid]
                            .instance_payload_size
                            .max(vm.classes[cid].elem_size);
                        let ptr = vm.heap.alloc_raw_struct(cid, sz);
                        CilVal::ValueType {
                            class_id: cid,
                            ptr,
                            size: sz,
                        }
                    }
                } else {
                    CilVal::I32(0)
                }
            }
            other => default_val_for_sig(other),
        };
        locals.push(v);
    }
    Ok(ExecFrame {
        method_id,
        asm_id,
        body,
        ip: 0,
        call_ip: 0,
        args,
        locals,
        stack: Vec::with_capacity(16),
        newobj_result,
        pending_leave: None,
        pending_unwind: None,
        caught_exc: None,
        active_filter: None,
        constrained_token: None,
    })
}

fn store_slot(slots: &mut Vec<CilVal>, idx: usize, val: CilVal) {
    if idx >= slots.len() {
        slots.resize(idx + 1, CilVal::I32(0));
    }
    if let (
        CilVal::ValueType { ptr: dp, size: dsz, .. },
        CilVal::ValueType { ptr: sp, size: ssz, .. },
    ) = (slots[idx], val)
    {
        if dp != 0 && sp != 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(sp as *const u8, dp as *mut u8, dsz.min(ssz));
            }
            return;
        }
    }
    slots[idx] = val;
}

fn field_base_ptr(target: CilVal) -> Option<usize> {
    match target {
        CilVal::Obj(0) => None,
        CilVal::Obj(p) => Some(p + MONO_OBJECT_HEADER_SIZE),
        CilVal::ManagedPtr(ManagedAddr::Raw(p)) if p != 0 => Some(p),
        CilVal::ValueType { ptr, .. } if ptr != 0 => Some(ptr),
        CilVal::NativeInt(p) if p != 0 => Some(p as usize),
        _ => None,
    }
}

fn raw_ptr_from_val(v: CilVal) -> Option<usize> {
    match v {
        CilVal::ManagedPtr(ManagedAddr::Raw(p)) if p != 0 => Some(p),
        CilVal::NativeInt(p) if p != 0 => Some(p as usize),
        CilVal::I64(p) if p != 0 => Some(p as usize),
        CilVal::Obj(p) if p != 0 => Some(p),
        CilVal::ValueType { ptr, .. } if ptr != 0 => Some(ptr),
        _ => None,
    }
}

fn read_ptr_val(vm: &ClrVm, frames: &[ExecFrame], ptr_val: CilVal, sig: &SigType) -> CilVal {
    match ptr_val {
        CilVal::ManagedPtr(ManagedAddr::Local { frame, idx }) => frames
            .get(frame)
            .and_then(|f| f.locals.get(idx))
            .copied()
            .unwrap_or_else(|| default_val_for_sig(sig)),
        CilVal::ManagedPtr(ManagedAddr::Arg { frame, idx }) => frames
            .get(frame)
            .and_then(|f| f.args.get(idx))
            .copied()
            .unwrap_or_else(|| default_val_for_sig(sig)),
        CilVal::ManagedPtr(ManagedAddr::StaticField(fid)) => vm
            .fields
            .get(fid)
            .map_or_else(|| default_val_for_sig(sig), |f| f.static_value),
        _ => {
            if let Some(p) = raw_ptr_from_val(ptr_val) {
                read_sig_from_ptr(sig, p as *const u8)
            } else {
                default_val_for_sig(sig)
            }
        }
    }
}

fn write_ptr_val(
    vm: &mut ClrVm,
    frames: &mut [ExecFrame],
    addr_val: CilVal,
    sig: &SigType,
    val: CilVal,
) {
    match addr_val {
        CilVal::ManagedPtr(addr) => write_managed_addr(vm, frames, addr, sig, val),
        _ => {
            if let Some(p) = raw_ptr_from_val(addr_val) {
                write_sig_to_ptr(sig, p as *mut u8, val);
            }
        }
    }
}

fn write_managed_addr(
    vm: &mut ClrVm,
    frames: &mut [ExecFrame],
    addr: ManagedAddr,
    sig: &SigType,
    val: CilVal,
) {
    match addr {
        ManagedAddr::Local { frame, idx } => {
            if let Some(f) = frames.get_mut(frame) {
                store_slot(&mut f.locals, idx, val);
            }
        }
        ManagedAddr::Arg { frame, idx } => {
            if let Some(f) = frames.get_mut(frame) {
                store_slot(&mut f.args, idx, val);
            }
        }
        ManagedAddr::StaticField(fid) => {
            if let Some(f) = vm.fields.get_mut(fid) {
                f.static_value = val;
            }
        }
        ManagedAddr::Raw(p) => {
            if p != 0 {
                write_sig_to_ptr(sig, p as *mut u8, val);
            }
        }
    }
}

fn array_elem_ptr(vm: &ClrVm, arr: usize, idx: isize) -> Result<(usize, usize), bool> {
    if arr == 0 {
        return Err(true);
    }
    let (len, elem_sz) = match vm.heap.objects.get(&arr) {
        Some(HeapObjKind::Array {
            length, elem_size, ..
        }) => (*length, *elem_size),
        _ => return Err(true),
    };
    if idx < 0 || (idx as usize) >= len {
        return Err(false);
    }
    Ok((arr + MONO_ARRAY_DATA_OFFSET + (idx as usize) * elem_sz, elem_sz))
}

fn find_exited_finally(
    clauses: &[super::metadata::ExceptionClause],
    leave_ip: usize,
    target_ip: usize,
    start_idx: usize,
) -> Option<(usize, usize)> {
    for (i, c) in clauses.iter().enumerate().skip(start_idx) {
        if c.flags == eh_flags::FINALLY {
            let try_end = c.try_offset + c.try_length;
            let in_try = leave_ip >= c.try_offset && leave_ip < try_end;
            let target_in_try = target_ip >= c.try_offset && target_ip < try_end;
            if in_try && !target_in_try {
                return Some((c.handler_offset, i + 1));
            }
        }
    }
    None
}

fn dispatch_exception(
    vm: &ClrVm,
    frames: &mut Vec<ExecFrame>,
    exc_obj: usize,
    throw_ip: usize,
) -> bool {
    dispatch_exception_from(vm, frames, exc_obj, throw_ip, 0)
}

fn dispatch_exception_from(
    vm: &ClrVm,
    frames: &mut Vec<ExecFrame>,
    exc_obj: usize,
    mut ip: usize,
    mut start_clause: usize,
) -> bool {
    while let Some(frame) = frames.last_mut() {
        let asm_id = frame.asm_id;
        let clauses = frame.body.eh_clauses.clone();
        for (i, c) in clauses.iter().enumerate().skip(start_clause) {
            let try_end = c.try_offset + c.try_length;
            if ip < c.try_offset || ip >= try_end {
                continue;
            }
            if c.flags == eh_flags::FINALLY || c.flags == eh_flags::FAULT {
                frame.stack.clear();
                frame.pending_unwind = Some((exc_obj, ip, i + 1));
                frame.ip = c.handler_offset;
                return true;
            }
            if c.flags == eh_flags::FILTER {
                frame.stack.clear();
                frame.stack.push(CilVal::Obj(exc_obj));
                frame.active_filter = Some((exc_obj, ip, i));
                frame.ip = c.class_token_or_filter_offset as usize;
                return true;
            }
            if c.flags == eh_flags::EXCEPTION {
                let matches = if c.class_token_or_filter_offset == 0 {
                    true
                } else if let Some(cid) = vm.resolve_type_token(asm_id, c.class_token_or_filter_offset) {
                    vm.is_instance_of(exc_obj, cid)
                } else {
                    true
                };
                if matches {
                    frame.stack.clear();
                    frame.stack.push(CilVal::Obj(exc_obj));
                    frame.caught_exc = Some(exc_obj);
                    frame.ip = c.handler_offset;
                    return true;
                }
            }
        }
        frames.pop();
        if let Some(caller) = frames.last() {
            ip = caller.call_ip;
            start_clause = 0;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Arithmetic, Comparison, Conversion Helpers
// ---------------------------------------------------------------------------

fn arith_bin(a: CilVal, b: CilVal, op: u8) -> CilVal {
    match (a, b) {
        (CilVal::F64(_), _) | (_, CilVal::F64(_)) => {
            let (x, y) = (a.as_f64(), b.as_f64());
            CilVal::F64(match op {
                0 => x + y,
                1 => x - y,
                _ => x * y,
            })
        }
        (CilVal::I64(_), _) | (_, CilVal::I64(_)) => {
            let (x, y) = (a.as_i64(), b.as_i64());
            CilVal::I64(match op {
                0 => x.wrapping_add(y),
                1 => x.wrapping_sub(y),
                _ => x.wrapping_mul(y),
            })
        }
        (CilVal::NativeInt(_), _)
        | (_, CilVal::NativeInt(_))
        | (CilVal::ManagedPtr(_), _)
        | (_, CilVal::ManagedPtr(_)) => {
            let (x, y) = (a.as_isize(), b.as_isize());
            CilVal::NativeInt(match op {
                0 => x.wrapping_add(y),
                1 => x.wrapping_sub(y),
                _ => x.wrapping_mul(y),
            })
        }
        _ => {
            let (x, y) = (a.as_i32(), b.as_i32());
            CilVal::I32(match op {
                0 => x.wrapping_add(y),
                1 => x.wrapping_sub(y),
                _ => x.wrapping_mul(y),
            })
        }
    }
}

fn arith_div(a: CilVal, b: CilVal, unsigned: bool) -> Result<CilVal, bool> {
    if matches!(a, CilVal::F64(_)) || matches!(b, CilVal::F64(_)) {
        return Ok(CilVal::F64(a.as_f64() / b.as_f64()));
    }
    if matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_)) {
        let (x, y) = (a.as_i64(), b.as_i64());
        if y == 0 {
            return Err(true);
        }
        if unsigned {
            return Ok(CilVal::I64(((x as u64) / (y as u64)) as i64));
        }
        if x == i64::MIN && y == -1 {
            return Err(false);
        }
        return Ok(CilVal::I64(x / y));
    }
    let (x, y) = (a.as_i32(), b.as_i32());
    if y == 0 {
        return Err(true);
    }
    if unsigned {
        return Ok(CilVal::I32(((x as u32) / (y as u32)) as i32));
    }
    if x == i32::MIN && y == -1 {
        return Err(false);
    }
    Ok(CilVal::I32(x / y))
}

fn arith_rem(a: CilVal, b: CilVal, unsigned: bool) -> Result<CilVal, bool> {
    if matches!(a, CilVal::F64(_)) || matches!(b, CilVal::F64(_)) {
        return Ok(CilVal::F64(a.as_f64() % b.as_f64()));
    }
    if matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_)) {
        let (x, y) = (a.as_i64(), b.as_i64());
        if y == 0 {
            return Err(true);
        }
        if unsigned {
            return Ok(CilVal::I64(((x as u64) % (y as u64)) as i64));
        }
        if x == i64::MIN && y == -1 {
            return Err(false);
        }
        return Ok(CilVal::I64(x % y));
    }
    let (x, y) = (a.as_i32(), b.as_i32());
    if y == 0 {
        return Err(true);
    }
    if unsigned {
        return Ok(CilVal::I32(((x as u32) % (y as u32)) as i32));
    }
    if x == i32::MIN && y == -1 {
        return Err(false);
    }
    Ok(CilVal::I32(x % y))
}

fn arith_checked(op: u8, a: CilVal, b: CilVal) -> Result<CilVal, ()> {
    let is64 = matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_));
    if is64 {
        let (x, y) = (a.as_i64(), b.as_i64());
        let r = match op {
            0xD6 => x.checked_add(y),
            0xD7 => (x as u64).checked_add(y as u64).map(|v| v as i64),
            0xD8 => x.checked_mul(y),
            0xD9 => (x as u64).checked_mul(y as u64).map(|v| v as i64),
            0xDA => x.checked_sub(y),
            0xDB => (x as u64).checked_sub(y as u64).map(|v| v as i64),
            _ => None,
        };
        r.map(CilVal::I64).ok_or(())
    } else {
        let (x, y) = (a.as_i32(), b.as_i32());
        let r = match op {
            0xD6 => x.checked_add(y),
            0xD7 => (x as u32).checked_add(y as u32).map(|v| v as i32),
            0xD8 => x.checked_mul(y),
            0xD9 => (x as u32).checked_mul(y as u32).map(|v| v as i32),
            0xDA => x.checked_sub(y),
            0xDB => (x as u32).checked_sub(y as u32).map(|v| v as i32),
            _ => None,
        };
        r.map(CilVal::I32).ok_or(())
    }
}

fn bitwise_bin(a: CilVal, b: CilVal, op: u8) -> CilVal {
    if matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_)) {
        let (x, y) = (a.as_i64(), b.as_i64());
        CilVal::I64(match op {
            0 => x & y,
            1 => x | y,
            _ => x ^ y,
        })
    } else if matches!(a, CilVal::NativeInt(_)) || matches!(b, CilVal::NativeInt(_)) {
        let (x, y) = (a.as_isize(), b.as_isize());
        CilVal::NativeInt(match op {
            0 => x & y,
            1 => x | y,
            _ => x ^ y,
        })
    } else {
        let (x, y) = (a.as_i32(), b.as_i32());
        CilVal::I32(match op {
            0 => x & y,
            1 => x | y,
            _ => x ^ y,
        })
    }
}

fn shift_bin(a: CilVal, b: CilVal, op: u8) -> CilVal {
    let sh = b.as_i32() as u32;
    match a {
        CilVal::I64(x) => CilVal::I64(match op {
            0 => x.wrapping_shl(sh & 63),
            1 => x.wrapping_shr(sh & 63),
            _ => (x as u64).wrapping_shr(sh & 63) as i64,
        }),
        CilVal::NativeInt(x) => CilVal::NativeInt(match op {
            0 => x.wrapping_shl(sh & 63),
            1 => x.wrapping_shr(sh & 63),
            _ => (x as usize).wrapping_shr(sh & 63) as isize,
        }),
        _ => {
            let x = a.as_i32();
            CilVal::I32(match op {
                0 => x.wrapping_shl(sh & 31),
                1 => x.wrapping_shr(sh & 31),
                _ => (x as u32).wrapping_shr(sh & 31) as i32,
            })
        }
    }
}

fn eval_branch_cmp(kind: u8, a: CilVal, b: CilVal) -> bool {
    if matches!(a, CilVal::F64(_)) || matches!(b, CilVal::F64(_)) {
        let (x, y) = (a.as_f64(), b.as_f64());
        return match kind {
            0 => x == y,               // beq
            1 => x >= y,               // bge
            2 => x > y,                // bgt
            3 => x <= y,               // ble
            4 => x < y,                // blt
            5 => !(x == y),            // bne.un
            6 => !(x < y),             // bge.un
            7 => !(x <= y),            // bgt.un
            8 => !(x > y),             // ble.un
            _ => !(x >= y),            // blt.un
        };
    }
    let wide = matches!(
        (a, b),
        (CilVal::I64(_), _)
            | (_, CilVal::I64(_))
            | (CilVal::NativeInt(_), _)
            | (_, CilVal::NativeInt(_))
            | (CilVal::Obj(_), _)
            | (_, CilVal::Obj(_))
    );
    if wide {
        let (sx, sy) = (a.as_i64(), b.as_i64());
        let (ux, uy) = (a.as_u64_bits(), b.as_u64_bits());
        match kind {
            0 => ux == uy,
            1 => sx >= sy,
            2 => sx > sy,
            3 => sx <= sy,
            4 => sx < sy,
            5 => ux != uy,
            6 => ux >= uy,
            7 => ux > uy,
            8 => ux <= uy,
            _ => ux < uy,
        }
    } else {
        let (sx, sy) = (a.as_i32(), b.as_i32());
        let (ux, uy) = (sx as u32, sy as u32);
        match kind {
            0 => sx == sy,
            1 => sx >= sy,
            2 => sx > sy,
            3 => sx <= sy,
            4 => sx < sy,
            5 => sx != sy,
            6 => ux >= uy,
            7 => ux > uy,
            8 => ux <= uy,
            _ => ux < uy,
        }
    }
}

fn eval_cmp_op(op2: u8, a: CilVal, b: CilVal) -> bool {
    match op2 {
        0x01 => eval_branch_cmp(0, a, b), // ceq
        0x02 => eval_branch_cmp(2, a, b), // cgt
        0x03 => eval_branch_cmp(7, a, b), // cgt.un
        0x04 => eval_branch_cmp(4, a, b), // clt
        0x05 => eval_branch_cmp(9, a, b), // clt.un
        _ => false,
    }
}

fn conv_checked(op: u8, v: CilVal) -> Result<CilVal, ()> {
    let unsigned_in = (0x82..=0x8A).contains(&op);
    if unsigned_in {
        let u = match v {
            CilVal::I32(x) => x as u32 as u64,
            CilVal::I64(x) => x as u64,
            CilVal::NativeInt(x) => x as usize as u64,
            CilVal::F64(x) if x >= 0.0 && x.is_finite() => x as u64,
            _ => return Err(()),
        };
        match op {
            0x82 => i8::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x83 => i16::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x84 => i32::try_from(u).map(CilVal::I32).map_err(|_| ()),
            0x85 => i64::try_from(u).map(CilVal::I64).map_err(|_| ()),
            0x86 => u8::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x87 => u16::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x88 => u32::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x89 => Ok(CilVal::I64(u as i64)),
            0x8A => isize::try_from(u).map(CilVal::NativeInt).map_err(|_| ()),
            _ => Err(()),
        }
    } else {
        let s: i64 = match v {
            CilVal::I32(x) => x as i64,
            CilVal::I64(x) => x,
            CilVal::NativeInt(x) => x as i64,
            CilVal::F64(x) if x.is_finite() && x >= (i64::MIN as f64) && x <= (i64::MAX as f64) => x as i64,
            _ => return Err(()),
        };
        match op {
            0xB3 => i8::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB4 => u8::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB5 => i16::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB6 => u16::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB7 => i32::try_from(s).map(CilVal::I32).map_err(|_| ()),
            0xB8 => u32::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB9 => Ok(CilVal::I64(s)),
            0xBA => u64::try_from(s).map(|x| CilVal::I64(x as i64)).map_err(|_| ()),
            0xD4 => isize::try_from(s).map(CilVal::NativeInt).map_err(|_| ()),
            0xD5 => usize::try_from(s).map(|x| CilVal::NativeInt(x as isize)).map_err(|_| ()),
            _ => Err(()),
        }
    }
}

// ---------------------------------------------------------------------------
// Token Resolution & Virtual Dispatch
// ---------------------------------------------------------------------------

fn resolve_method_sig_for_token(vm: &ClrVm, asm_id: usize, token: u32) -> Option<MethodSig> {
    let tbl = (token >> 24) as u8;
    let rid = (token & 0x00FF_FFFF) as usize;
    if rid == 0 {
        return None;
    }
    let asm = vm.assemblies.get(asm_id)?;
    match tbl {
        table::METHOD_DEF => {
            let mid = *asm.method_ids.get(rid - 1)?;
            Some(vm.methods[mid].sig.clone())
        }
        table::MEMBER_REF => {
            let img = asm.image.as_ref()?;
            let mr = img.member_refs.get(rid - 1)?;
            Some(img.parse_method_sig(mr.signature))
        }
        table::METHOD_SPEC => {
            let img = asm.image.as_ref()?;
            let ms = img.method_specs.get(rid - 1)?;
            resolve_method_sig_for_token(vm, asm_id, ms.method)
        }
        _ => None,
    }
}

fn resolve_call_target(
    vm: &ClrVm,
    asm_id: usize,
    token: u32,
    virt_recv_obj: Option<usize>,
    constrained_token: Option<u32>,
) -> Option<CallTarget> {
    let tbl = (token >> 24) as u8;
    let rid = (token & 0x00FF_FFFF) as usize;
    if rid == 0 {
        return None;
    }
    let asm = vm.assemblies.get(asm_id)?;
    match tbl {
        table::METHOD_SPEC => {
            let img = asm.image.as_ref()?;
            let ms = img.method_specs.get(rid - 1)?;
            resolve_call_target(vm, asm_id, ms.method, virt_recv_obj, constrained_token)
        }
        table::METHOD_DEF => {
            let mut mid = *asm.method_ids.get(rid - 1)?;
            // Virtual dispatch override lookup on runtime class of receiver
            if let Some(recv) = virt_recv_obj {
                if (vm.methods[mid].flags & method_attr::VIRTUAL) != 0 {
                    if let Some(rcid) = vm.object_class_id(recv) {
                        let mname = &vm.methods[mid].name;
                        let pcnt = vm.methods[mid].sig.params.len() as i32;
                        if let Some(ov) = vm.find_method_in_class(rcid, mname, pcnt) {
                            mid = ov;
                        }
                    }
                }
            }
            classify_method_id(vm, mid)
        }
        table::MEMBER_REF => {
            let img = asm.image.as_ref()?;
            let mr = img.member_refs.get(rid - 1)?;
            let mname = img.get_string(mr.name).to_string();
            let msig = img.parse_method_sig(mr.signature);

            // Check if constrained or virtual receiver resolves to a user method
            if let Some(ctok) = constrained_token {
                if let Some(cid) = vm.resolve_type_token(asm_id, ctok) {
                    if let Some(mid) = vm.find_method_in_class(cid, &mname, msig.params.len() as i32) {
                        if let Some(ct) = classify_method_id(vm, mid) {
                            return Some(ct);
                        }
                    }
                }
            }
            if let Some(recv) = virt_recv_obj {
                if let Some(rcid) = vm.object_class_id(recv) {
                    if let Some(mid) = vm.find_method_in_class(rcid, &mname, msig.params.len() as i32) {
                        if let Some(ct) = classify_method_id(vm, mid) {
                            return Some(ct);
                        }
                    }
                }
            }

            let (pns, pname) = img
                .resolve_type_token_name(mr.class)
                .unwrap_or_else(|| (String::new(), String::new()));

            // Check loaded user assemblies first
            if let Some(cid) = vm.find_class_by_name_any(&pns, &pname) {
                if vm.classes[cid].is_delegate && mname == "Invoke" {
                    return Some(CallTarget::DelegateInvoke { sig: msig });
                }
                if let Some(mid) = vm.find_method_in_class(cid, &mname, msig.params.len() as i32) {
                    if let Some(ct) = classify_method_id(vm, mid) {
                        return Some(ct);
                    }
                }
                if let Some(fn_ptr) = vm.lookup_internal_call(cid, &mname) {
                    return Some(CallTarget::InternalCall { fn_ptr, sig: msig });
                }
            }

            Some(CallTarget::Bcl {
                namespace: pns,
                type_name: pname,
                method_name: mname,
                sig: msig,
            })
        }
        _ => None,
    }
}

fn classify_method_id(vm: &ClrVm, mid: usize) -> Option<CallTarget> {
    let m = vm.methods.get(mid)?;
    let cls = &vm.classes[m.class_id];
    if cls.is_delegate && m.name == "Invoke" {
        return Some(CallTarget::DelegateInvoke { sig: m.sig.clone() });
    }
    if (m.flags & method_attr::PINVOKE_IMPL) != 0 {
        if let Some((dll, entry, mapping_flags)) = &m.pinvoke {
            return Some(CallTarget::PInvoke {
                dll: dll.clone(),
                entry: entry.clone(),
                mapping_flags: *mapping_flags,
                sig: m.sig.clone(),
            });
        }
    }
    if (m.impl_flags & method_impl_attr::INTERNAL_CALL) != 0 || m.body.is_none() {
        if let Some(fn_ptr) = vm.lookup_internal_call(m.class_id, &m.name) {
            return Some(CallTarget::InternalCall {
                fn_ptr,
                sig: m.sig.clone(),
            });
        }
        if m.body.is_none() {
            return Some(CallTarget::Bcl {
                namespace: cls.namespace.clone(),
                type_name: cls.name.clone(),
                method_name: m.name.clone(),
                sig: m.sig.clone(),
            });
        }
    }
    Some(CallTarget::Managed(mid))
}

fn resolve_field_token(vm: &ClrVm, asm_id: usize, token: u32) -> Option<usize> {
    let tbl = (token >> 24) as u8;
    let rid = (token & 0x00FF_FFFF) as usize;
    if rid == 0 {
        return None;
    }
    let asm = vm.assemblies.get(asm_id)?;
    match tbl {
        table::FIELD => asm.field_ids.get(rid - 1).copied(),
        table::MEMBER_REF => {
            let img = asm.image.as_ref()?;
            let mr = img.member_refs.get(rid - 1)?;
            let fname = img.get_string(mr.name);
            let (pns, pname) = img.resolve_type_token_name(mr.class)?;
            let cid = vm.find_class_by_name_any(&pns, &pname)?;
            vm.find_field_in_class(cid, fname)
        }
        _ => None,
    }
}

fn resolve_bcl_static_field(vm: &mut ClrVm, asm_id: usize, token: u32) -> Option<CilVal> {
    if (token >> 24) as u8 != table::MEMBER_REF {
        return None;
    }
    let rid = (token & 0x00FF_FFFF) as usize;
    let img = vm.assemblies.get(asm_id)?.image.as_ref()?.clone();
    let mr = img.member_refs.get(rid.checked_sub(1)?)?;
    let fname = img.get_string(mr.name);
    let (pns, pname) = img.resolve_type_token_name(mr.class)?;
    match (pns.as_str(), pname.as_str(), fname) {
        ("System", "String", "Empty") => Some(CilVal::Obj(vm.alloc_string(""))),
        ("System", "IntPtr", "Zero") | ("System", "UIntPtr", "Zero") => Some(CilVal::NativeInt(0)),
        ("System", "Math", "PI") => Some(CilVal::F64(std::f64::consts::PI)),
        ("System", "Math", "E") => Some(CilVal::F64(std::f64::consts::E)),
        ("System", "MathF", "PI") => Some(CilVal::F64(std::f32::consts::PI as f64)),
        ("System", "MathF", "E") => Some(CilVal::F64(std::f32::consts::E as f64)),
        ("System", "BitConverter", "IsLittleEndian") => Some(CilVal::I32(1)),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// P/Invoke Marshalling
// ---------------------------------------------------------------------------

fn resolve_pinvoke_target(dll: &str, entry: &str, mapping_flags: u16) -> Option<usize> {
    if let Some(p) = crate::lookup(dll, entry) {
        return Some(p as usize);
    }
    let no_mangle = (mapping_flags & pinvoke_attr::NO_MANGLE) != 0;
    if !no_mangle {
        let cs = mapping_flags & pinvoke_attr::CHAR_SET_MASK;
        let suffixes = if cs == pinvoke_attr::CHAR_SET_UNICODE || cs == pinvoke_attr::CHAR_SET_AUTO {
            ["W", "A"]
        } else {
            ["A", "W"]
        };
        for suf in suffixes {
            let cand = format!("{entry}{suf}");
            if let Some(p) = crate::lookup(dll, &cand) {
                return Some(p as usize);
            }
        }
    }
    if let Some(loader) = crate::try_loader() {
        if let Ok(hmod) = loader.load_library(dll, 0) {
            if let Some(p) = loader.get_proc_address(hmod, crate::ProcName::Name(entry)) {
                return Some(p);
            }
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn marshal_pinvoke_arg(
    vm: &ClrVm,
    frames: &[ExecFrame],
    pty: &SigType,
    val: CilVal,
    unicode: bool,
    keep_ansi: &mut Vec<CString>,
    keep_wide: &mut Vec<Vec<u16>>,
    byref_slots: &mut Vec<(ManagedAddr, SigType, Box<u64>)>,
) -> u64 {
    match pty {
        SigType::String => {
            let obj = val.as_obj();
            if obj == 0 {
                return 0;
            }
            let s = vm.heap.read_string(obj).unwrap_or_default();
            if unicode {
                let w = crate::strings::to_wide_z(&s);
                let ptr = w.as_ptr() as u64;
                keep_wide.push(w);
                ptr
            } else {
                let c = CString::new(s.as_bytes()).unwrap_or_default();
                let ptr = c.as_ptr() as u64;
                keep_ansi.push(c);
                ptr
            }
        }
        SigType::SzArray(_) | SigType::Array(_, _) => {
            let obj = val.as_obj();
            if obj == 0 { 0 } else { (obj + MONO_ARRAY_DATA_OFFSET) as u64 }
        }
        SigType::ByRef(inner) => match val {
            CilVal::ManagedPtr(ManagedAddr::Raw(p)) => p as u64,
            CilVal::ManagedPtr(addr) => {
                let cur = read_ptr_val(vm, frames, val, inner);
                let mut slot = Box::new(0u64);
                write_sig_to_ptr(inner, (&mut *slot as *mut u64) as *mut u8, cur);
                let ptr = (&mut *slot as *mut u64) as u64;
                byref_slots.push((addr, (**inner).clone(), slot));
                ptr
            }
            other => other.as_u64_bits(),
        },
        SigType::R4 => (val.as_f64() as f32).to_bits() as u64,
        _ => val.as_u64_bits(),
    }
}

fn unmarshal_native_return(vm: &mut ClrVm, ret: &SigType, raw: u64, unicode: bool) -> CilVal {
    match ret {
        SigType::Boolean => CilVal::I32(i32::from((raw as u32) != 0)),
        SigType::I1 => CilVal::I32(raw as i8 as i32),
        SigType::U1 => CilVal::I32(raw as u8 as i32),
        SigType::I2 => CilVal::I32(raw as i16 as i32),
        SigType::U2 | SigType::Char => CilVal::I32(raw as u16 as i32),
        SigType::I4 | SigType::U4 => CilVal::I32(raw as i32),
        SigType::I8 | SigType::U8 => CilVal::I64(raw as i64),
        SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
            CilVal::NativeInt(raw as isize)
        }
        SigType::R4 => CilVal::F64(f32::from_bits(raw as u32) as f64),
        SigType::R8 => CilVal::F64(f64::from_bits(raw)),
        SigType::String => {
            if raw == 0 {
                CilVal::Obj(0)
            } else {
                let s = unsafe {
                    if unicode {
                        crate::strings::from_wide(raw as *const u16)
                    } else {
                        crate::strings::from_ansi(raw as *const u8)
                    }
                };
                CilVal::Obj(vm.alloc_string(&s))
            }
        }
        _ => CilVal::Obj(raw as usize),
    }
}

fn unmarshal_icall_return(ret: &SigType, raw: u64) -> CilVal {
    match ret {
        SigType::Boolean => CilVal::I32(i32::from((raw as u8) != 0)),
        SigType::I1 | SigType::U1 | SigType::I2 | SigType::U2 | SigType::Char | SigType::I4 | SigType::U4 => {
            CilVal::I32(raw as i32)
        }
        SigType::I8 | SigType::U8 => CilVal::I64(raw as i64),
        SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
            CilVal::NativeInt(raw as isize)
        }
        SigType::R4 => CilVal::F64(f32::from_bits(raw as u32) as f64),
        SigType::R8 => CilVal::F64(f64::from_bits(raw)),
        _ => CilVal::Obj(raw as usize),
    }
}

// ---------------------------------------------------------------------------
// Essential BCL Intrinsics
// ---------------------------------------------------------------------------

fn dispatch_bcl_newobj(
    vm: &mut ClrVm,
    ns: &str,
    ty: &str,
    sig: &MethodSig,
    args: &[CilVal],
) -> Result<CilVal, VmError> {
    match (ns, ty) {
        ("System", "Object") => Ok(CilVal::Obj(vm.alloc_object_for_class(vm.core.object))),
        ("System", "String") => {
            let s = match args.len() {
                1 => {
                    let p = args[0];
                    match sig.params.first() {
                        Some(SigType::SzArray(_)) => {
                            let arr = p.as_obj();
                            if arr == 0 {
                                String::new()
                            } else {
                                let len = match vm.heap.objects.get(&arr) {
                                    Some(HeapObjKind::Array { length, .. }) => *length,
                                    _ => 0,
                                };
                                let mut u16s = Vec::with_capacity(len);
                                unsafe {
                                    let base = (arr + MONO_ARRAY_DATA_OFFSET) as *const u16;
                                    for i in 0..len {
                                        u16s.push(std::ptr::read_unaligned(base.add(i)));
                                    }
                                }
                                return Ok(CilVal::Obj(vm.alloc_string_utf16(&u16s)));
                            }
                        }
                        Some(SigType::Ptr(_)) => {
                            let ptr = p.as_isize() as *const u16;
                            if ptr.is_null() {
                                String::new()
                            } else {
                                unsafe { crate::strings::from_wide(ptr) }
                            }
                        }
                        _ => String::new(),
                    }
                }
                2 => {
                    let ch = args[0].as_i32() as u16;
                    let cnt = args[1].as_i32().max(0) as usize;
                    let u16s = vec![ch; cnt];
                    return Ok(CilVal::Obj(vm.alloc_string_utf16(&u16s)));
                }
                3 => {
                    let arr = args[0].as_obj();
                    let start = args[1].as_i32().max(0) as usize;
                    let cnt = args[2].as_i32().max(0) as usize;
                    let len = match vm.heap.objects.get(&arr) {
                        Some(HeapObjKind::Array { length, .. }) => *length,
                        _ => 0,
                    };
                    let end = start.saturating_add(cnt).min(len);
                    let mut u16s = Vec::with_capacity(end.saturating_sub(start));
                    unsafe {
                        let base = (arr + MONO_ARRAY_DATA_OFFSET) as *const u16;
                        for i in start..end {
                            u16s.push(std::ptr::read_unaligned(base.add(i)));
                        }
                    }
                    return Ok(CilVal::Obj(vm.alloc_string_utf16(&u16s)));
                }
                _ => String::new(),
            };
            Ok(CilVal::Obj(vm.alloc_string(&s)))
        }
        ("System", "IntPtr") | ("System", "UIntPtr") => {
            Ok(CilVal::NativeInt(args.first().map_or(0, |v| v.as_isize())))
        }
        ("System.Text", "StringBuilder") => {
            let init = if args.len() == 1 && matches!(sig.params.first(), Some(SigType::String)) {
                vm.heap.read_string(args[0].as_obj()).unwrap_or_default()
            } else {
                String::new()
            };
            let vt = vm.classes[vm.core.string_builder].mono_vtable;
            let ptr = vm.heap.alloc_raw(MONO_OBJECT_HEADER_SIZE);
            unsafe {
                std::ptr::write_unaligned(ptr as *mut usize, vt);
            }
            let addr = ptr as usize;
            vm.heap
                .objects
                .insert(addr, HeapObjKind::StringBuilder { buf: init });
            Ok(CilVal::Obj(addr))
        }
        (_, name) if name.ends_with("Exception") => {
            let cid = vm
                .find_class_by_name_any(ns, ty)
                .unwrap_or(vm.core.exception);
            let msg = args
                .first()
                .and_then(|v| vm.heap.read_string(v.as_obj()))
                .unwrap_or_else(|| format!("{ns}.{ty}"));
            Ok(CilVal::Obj(vm.make_exception(cid, msg)))
        }
        _ => {
            let cid = vm
                .find_class_by_name_any(ns, ty)
                .unwrap_or(vm.core.object);
            Ok(CilVal::Obj(vm.alloc_object_for_class(cid)))
        }
    }
}

fn dispatch_bcl(
    vm: &mut ClrVm,
    frames: &mut [ExecFrame],
    ns: &str,
    ty: &str,
    method: &str,
    sig: &MethodSig,
    args: &[CilVal],
) -> Result<Option<CilVal>, VmError> {
    match (ns, ty, method) {
        // ---- System.Object ----
        ("System", "Object", ".ctor") => Ok(None),
        ("System", "Object", "ReferenceEquals") => {
            let eq = args.first().map_or(0, |v| v.as_obj()) == args.get(1).map_or(0, |v| v.as_obj());
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        ("System", "Object", "Equals") => {
            let a = args.first().copied().unwrap_or(CilVal::Obj(0));
            let b = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let eq = if let (Some(sa), Some(sb)) =
                (vm.heap.read_string(a.as_obj()), vm.heap.read_string(b.as_obj()))
            {
                sa == sb
            } else {
                a.as_u64_bits() == b.as_u64_bits()
            };
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        ("System", "Object", "GetHashCode") => {
            let v = args.first().map_or(0, |v| v.as_u64_bits());
            Ok(Some(CilVal::I32(((v >> 32) ^ v) as i32)))
        }
        ("System", "Object", "ToString") => {
            let s = vm.format_val_to_string(args.first().copied().unwrap_or(CilVal::Obj(0)));
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        ("System", "Object", "GetType") => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let cid = vm.object_class_id(obj).unwrap_or(vm.core.object);
            Ok(Some(CilVal::Obj(alloc_type_obj(vm, cid))))
        }

        // ---- System.Type ----
        ("System", "Type", "GetTypeFromHandle") => {
            let packed = args.first().map_or(0, |v| v.as_u64_bits());
            let aid = (packed >> 32) as usize;
            let tok = packed as u32;
            let cid = vm.resolve_type_token(aid, tok).unwrap_or(vm.core.object);
            Ok(Some(CilVal::Obj(alloc_type_obj(vm, cid))))
        }
        ("System", "Type", "get_Name") => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let name = match vm.heap.objects.get(&obj) {
                Some(HeapObjKind::TypeObj { represented_class_id }) => {
                    vm.classes[*represented_class_id].name.clone()
                }
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&name))))
        }
        ("System", "Type", "get_FullName") => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let name = match vm.heap.objects.get(&obj) {
                Some(HeapObjKind::TypeObj { represented_class_id }) => {
                    vm.classes[*represented_class_id].full_name()
                }
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&name))))
        }
        ("System", "Type", "get_Namespace") => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let name = match vm.heap.objects.get(&obj) {
                Some(HeapObjKind::TypeObj { represented_class_id }) => {
                    vm.classes[*represented_class_id].namespace.clone()
                }
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&name))))
        }

        // ---- System.String ----
        ("System", "String", "get_Length") => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            if obj == 0 {
                return Err(VmError::ManagedException {
                    exc_obj: vm.make_exception(vm.core.null_ref, "NullReferenceException"),
                    message: "NullReferenceException".into(),
                });
            }
            let len = unsafe {
                std::ptr::read_unaligned((obj + MONO_STRING_LEN_OFFSET) as *const i32)
            };
            Ok(Some(CilVal::I32(len)))
        }
        ("System", "String", "get_Chars") => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32());
            let Some(u16s) = vm.heap.read_string_utf16(obj) else {
                return Err(VmError::ManagedException {
                    exc_obj: vm.make_exception(vm.core.null_ref, "NullReferenceException"),
                    message: "NullReferenceException".into(),
                });
            };
            if idx < 0 || (idx as usize) >= u16s.len() {
                return Err(VmError::ManagedException {
                    exc_obj: vm.make_exception(vm.core.index_out_of_range, "IndexOutOfRangeException"),
                    message: "IndexOutOfRangeException".into(),
                });
            }
            Ok(Some(CilVal::I32(u16s[idx as usize] as i32)))
        }
        ("System", "String", "Concat") => {
            let mut out = String::new();
            if args.len() == 1 && matches!(sig.params.first(), Some(SigType::SzArray(_))) {
                let arr = args[0].as_obj();
                if let Some(HeapObjKind::Array { length, .. }) = vm.heap.objects.get(&arr).cloned() {
                    for i in 0..length {
                        let elem = unsafe {
                            std::ptr::read_unaligned(
                                (arr + MONO_ARRAY_DATA_OFFSET + i * 8) as *const usize,
                            )
                        };
                        out.push_str(&vm.format_val_to_string(CilVal::Obj(elem)));
                    }
                }
            } else {
                for &a in args {
                    out.push_str(&vm.format_val_to_string(a));
                }
            }
            Ok(Some(CilVal::Obj(vm.alloc_string(&out))))
        }
        ("System", "String", "op_Equality") => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let b = args.get(1).map_or(0, |v| v.as_obj());
            let eq = if a == b {
                true
            } else if a == 0 || b == 0 {
                false
            } else {
                vm.heap.read_string(a) == vm.heap.read_string(b)
            };
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        ("System", "String", "op_Inequality") => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let b = args.get(1).map_or(0, |v| v.as_obj());
            let neq = if a == b {
                false
            } else if a == 0 || b == 0 {
                true
            } else {
                vm.heap.read_string(a) != vm.heap.read_string(b)
            };
            Ok(Some(CilVal::I32(i32::from(neq))))
        }
        ("System", "String", "Equals") => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let b = args.get(1).map_or(0, |v| v.as_obj());
            let eq = if a == b {
                true
            } else if a == 0 || b == 0 {
                false
            } else {
                vm.heap.read_string(a) == vm.heap.read_string(b)
            };
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        ("System", "String", "Compare" | "CompareOrdinal") => {
            let a = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let b = vm.heap.read_string(args.get(1).map_or(0, |v| v.as_obj())).unwrap_or_default();
            let c = match a.cmp(&b) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            Ok(Some(CilVal::I32(c)))
        }
        ("System", "String", "IsNullOrEmpty") => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let empty = a == 0 || vm.heap.read_string(a).is_none_or(|s| s.is_empty());
            Ok(Some(CilVal::I32(i32::from(empty))))
        }
        ("System", "String", "IsNullOrWhiteSpace") => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let empty = a == 0 || vm.heap.read_string(a).is_none_or(|s| s.trim().is_empty());
            Ok(Some(CilVal::I32(i32::from(empty))))
        }
        ("System", "String", "Substring") => {
            let u16s = vm
                .heap
                .read_string_utf16(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let start = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let len = if args.len() >= 3 {
                args[2].as_i32().max(0) as usize
            } else {
                u16s.len().saturating_sub(start)
            };
            let end = start.saturating_add(len).min(u16s.len());
            let sub = if start <= end { &u16s[start..end] } else { &[] };
            Ok(Some(CilVal::Obj(vm.alloc_string_utf16(sub))))
        }
        ("System", "String", "Contains") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let needle = vm.heap.read_string(args.get(1).map_or(0, |v| v.as_obj())).unwrap_or_default();
            Ok(Some(CilVal::I32(i32::from(s.contains(&needle)))))
        }
        ("System", "String", "StartsWith") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let needle = vm.heap.read_string(args.get(1).map_or(0, |v| v.as_obj())).unwrap_or_default();
            Ok(Some(CilVal::I32(i32::from(s.starts_with(&needle)))))
        }
        ("System", "String", "EndsWith") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let needle = vm.heap.read_string(args.get(1).map_or(0, |v| v.as_obj())).unwrap_or_default();
            Ok(Some(CilVal::I32(i32::from(s.ends_with(&needle)))))
        }
        ("System", "String", "IndexOf") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let idx = if matches!(sig.params.first(), Some(SigType::Char)) {
                let ch = char::from_u32(args.get(1).map_or(0, |v| v.as_i32()) as u32).unwrap_or('\0');
                let start = args.get(2).map_or(0, |v| v.as_i32()).max(0) as usize;
                s.chars().skip(start).position(|c| c == ch).map_or(-1, |i| (start + i) as i32)
            } else {
                let needle = vm.heap.read_string(args.get(1).map_or(0, |v| v.as_obj())).unwrap_or_default();
                s.find(&needle).map_or(-1, |b| s[..b].encode_utf16().count() as i32)
            };
            Ok(Some(CilVal::I32(idx)))
        }
        ("System", "String", "ToUpper" | "ToUpperInvariant") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s.to_uppercase()))))
        }
        ("System", "String", "ToLower" | "ToLowerInvariant") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s.to_lowercase()))))
        }
        ("System", "String", "Trim") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            Ok(Some(CilVal::Obj(vm.alloc_string(s.trim()))))
        }
        ("System", "String", "Replace") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let out = if matches!(sig.params.first(), Some(SigType::Char)) {
                let old_c = char::from_u32(args.get(1).map_or(0, |v| v.as_i32()) as u32).unwrap_or('\0');
                let new_c = char::from_u32(args.get(2).map_or(0, |v| v.as_i32()) as u32).unwrap_or('\0');
                s.replace(old_c, &new_c.to_string())
            } else {
                let old_s = vm.heap.read_string(args.get(1).map_or(0, |v| v.as_obj())).unwrap_or_default();
                let new_s = vm.heap.read_string(args.get(2).map_or(0, |v| v.as_obj())).unwrap_or_default();
                s.replace(&old_s, &new_s)
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&out))))
        }
        ("System", "String", "ToCharArray") => {
            let u16s = vm
                .heap
                .read_string_utf16(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let arr = vm.alloc_array_for_class(vm.core.char_cls, u16s.len());
            unsafe {
                let dst = (arr + MONO_ARRAY_DATA_OFFSET) as *mut u16;
                for (i, &ch) in u16s.iter().enumerate() {
                    std::ptr::write_unaligned(dst.add(i), ch);
                }
            }
            Ok(Some(CilVal::Obj(arr)))
        }
        ("System", "String", "Format") => {
            let mut fmt_s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            for (i, &a) in args.iter().skip(1).enumerate() {
                let rep = vm.format_val_to_string(a);
                fmt_s = fmt_s.replace(&format!("{{{i}}}"), &rep);
            }
            Ok(Some(CilVal::Obj(vm.alloc_string(&fmt_s))))
        }
        ("System", "String", "ToString") => Ok(Some(args.first().copied().unwrap_or(CilVal::Obj(0)))),
        ("System", "String", "GetHashCode") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let mut h = 5381u32;
            for b in s.bytes() {
                h = h.wrapping_mul(33).wrapping_add(b as u32);
            }
            Ok(Some(CilVal::I32(h as i32)))
        }

        // ---- System.Math / System.MathF ----
        ("System", "Math" | "MathF", mname) => Ok(Some(dispatch_math(mname, sig, args))),

        // ---- System.Array ----
        ("System", "Array", "get_Length" | "get_LongLength") => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let len = match vm.heap.objects.get(&arr) {
                Some(HeapObjKind::Array { length, .. }) => *length,
                _ => 0,
            };
            if method == "get_LongLength" {
                Ok(Some(CilVal::I64(len as i64)))
            } else {
                Ok(Some(CilVal::I32(len as i32)))
            }
        }
        ("System", "Array", "get_Rank") => Ok(Some(CilVal::I32(1))),
        ("System", "Array", "GetLength") => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let len = match vm.heap.objects.get(&arr) {
                Some(HeapObjKind::Array { length, .. }) => *length,
                _ => 0,
            };
            Ok(Some(CilVal::I32(len as i32)))
        }
        ("System", "Array", "Copy") => {
            let (src, si, dst, di, cnt) = if args.len() >= 5 {
                (
                    args[0].as_obj(),
                    args[1].as_i32().max(0) as usize,
                    args[2].as_obj(),
                    args[3].as_i32().max(0) as usize,
                    args[4].as_i32().max(0) as usize,
                )
            } else {
                (
                    args.first().map_or(0, |v| v.as_obj()),
                    0,
                    args.get(1).map_or(0, |v| v.as_obj()),
                    0,
                    args.get(2).map_or(0, |v| v.as_i32()).max(0) as usize,
                )
            };
            if let (
                Some(HeapObjKind::Array { elem_size: es1, length: l1, .. }),
                Some(HeapObjKind::Array { elem_size: es2, length: l2, .. }),
            ) = (vm.heap.objects.get(&src), vm.heap.objects.get(&dst))
            {
                let es = (*es1).min(*es2);
                let n = cnt.min(l1.saturating_sub(si)).min(l2.saturating_sub(di));
                unsafe {
                    std::ptr::copy(
                        (src + MONO_ARRAY_DATA_OFFSET + si * es) as *const u8,
                        (dst + MONO_ARRAY_DATA_OFFSET + di * es) as *mut u8,
                        n * es,
                    );
                }
            }
            Ok(None)
        }
        ("System", "Array", "Clear") => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            if let Some(HeapObjKind::Array { elem_size, length, .. }) = vm.heap.objects.get(&arr) {
                let idx = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
                let cnt = args
                    .get(2)
                    .map_or(*length, |v| v.as_i32().max(0) as usize)
                    .min(length.saturating_sub(idx));
                unsafe {
                    std::ptr::write_bytes(
                        (arr + MONO_ARRAY_DATA_OFFSET + idx * elem_size) as *mut u8,
                        0,
                        cnt * elem_size,
                    );
                }
            }
            Ok(None)
        }
        ("System", "Array", "Empty") => {
            let arr = vm.alloc_array_for_class(vm.core.object, 0);
            Ok(Some(CilVal::Obj(arr)))
        }

        // ---- System.Buffer ----
        ("System", "Buffer", "BlockCopy") => {
            let src = args.first().map_or(0, |v| v.as_obj());
            let soff = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let dst = args.get(2).map_or(0, |v| v.as_obj());
            let doff = args.get(3).map_or(0, |v| v.as_i32()).max(0) as usize;
            let cnt = args.get(4).map_or(0, |v| v.as_i32()).max(0) as usize;
            if let (
                Some(HeapObjKind::Array { elem_size: es1, length: l1, .. }),
                Some(HeapObjKind::Array { elem_size: es2, length: l2, .. }),
            ) = (vm.heap.objects.get(&src), vm.heap.objects.get(&dst))
            {
                let max1 = l1 * es1;
                let max2 = l2 * es2;
                let n = cnt.min(max1.saturating_sub(soff)).min(max2.saturating_sub(doff));
                unsafe {
                    std::ptr::copy(
                        (src + MONO_ARRAY_DATA_OFFSET + soff) as *const u8,
                        (dst + MONO_ARRAY_DATA_OFFSET + doff) as *mut u8,
                        n,
                    );
                }
            }
            Ok(None)
        }
        ("System", "Buffer", "ByteLength") => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let bl = match vm.heap.objects.get(&arr) {
                Some(HeapObjKind::Array { elem_size, length, .. }) => (elem_size * length) as i32,
                _ => 0,
            };
            Ok(Some(CilVal::I32(bl)))
        }
        ("System", "Buffer", "GetByte") => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let b = unsafe { std::ptr::read_unaligned((arr + MONO_ARRAY_DATA_OFFSET + idx) as *const u8) };
            Ok(Some(CilVal::I32(b as i32)))
        }
        ("System", "Buffer", "SetByte") => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let b = args.get(2).map_or(0, |v| v.as_i32()) as u8;
            unsafe {
                std::ptr::write_unaligned((arr + MONO_ARRAY_DATA_OFFSET + idx) as *mut u8, b);
            }
            Ok(None)
        }
        ("System", "Buffer", "MemoryCopy") => {
            let src = args.first().map_or(0, |v| v.as_isize() as usize);
            let dst = args.get(1).map_or(0, |v| v.as_isize() as usize);
            let dst_len = args.get(2).map_or(0, |v| v.as_i64()).max(0) as usize;
            let cnt = args.get(3).map_or(0, |v| v.as_i64()).max(0) as usize;
            let n = cnt.min(dst_len);
            if n > 0 && src != 0 && dst != 0 {
                unsafe {
                    std::ptr::copy(src as *const u8, dst as *mut u8, n);
                }
            }
            Ok(None)
        }

        // ---- System.Environment ----
        ("System", "Environment", "get_TickCount") => {
            let ms = unsafe { crate::kernel32::time::GetTickCount() } as i32;
            Ok(Some(CilVal::I32(ms)))
        }
        ("System", "Environment", "get_TickCount64") => {
            let ms = unsafe { crate::kernel32::time::GetTickCount64() } as i64;
            Ok(Some(CilVal::I64(ms)))
        }
        ("System", "Environment", "get_ProcessorCount") => {
            let n = std::thread::available_parallelism().map_or(4, |p| p.get()) as i32;
            Ok(Some(CilVal::I32(n)))
        }
        ("System", "Environment", "get_Is64BitProcess" | "get_Is64BitOperatingSystem") => {
            Ok(Some(CilVal::I32(1)))
        }
        ("System", "Environment", "get_NewLine") => Ok(Some(CilVal::Obj(vm.alloc_string("\r\n")))),
        ("System", "Environment", "get_CurrentDirectory") => {
            let cwd = crate::path::cwd();
            Ok(Some(CilVal::Obj(vm.alloc_string(&cwd))))
        }
        ("System", "Environment", "get_MachineName") => {
            Ok(Some(CilVal::Obj(vm.alloc_string("APPLE-SILICON"))))
        }
        ("System", "Environment", "get_UserName") => {
            let u = std::env::var("USER").unwrap_or_else(|_| "player".into());
            Ok(Some(CilVal::Obj(vm.alloc_string(&u))))
        }
        ("System", "Environment", "GetEnvironmentVariable") => {
            let key = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            match std::env::var(&key) {
                Ok(val) => Ok(Some(CilVal::Obj(vm.alloc_string(&val)))),
                Err(_) => Ok(Some(CilVal::Obj(0))),
            }
        }
        ("System", "Environment", "SetEnvironmentVariable") => {
            let key = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let val = vm.heap.read_string(args.get(1).map_or(0, |v| v.as_obj()));
            if !key.is_empty() {
                if let Some(v) = val {
                    unsafe { std::env::set_var(&key, &v) };
                } else {
                    unsafe { std::env::remove_var(&key) };
                }
            }
            Ok(None)
        }
        ("System", "Environment", "GetCommandLineArgs") => {
            let items = vm.command_line_args.clone();
            let arr = vm.alloc_array_for_class(vm.core.string, items.len());
            for (i, s) in items.iter().enumerate() {
                let so = vm.alloc_string(s);
                unsafe {
                    std::ptr::write_unaligned(
                        (arr + MONO_ARRAY_DATA_OFFSET + i * 8) as *mut usize,
                        so,
                    );
                }
            }
            Ok(Some(CilVal::Obj(arr)))
        }
        ("System", "Environment", "get_ExitCode") => Ok(Some(CilVal::I32(vm.env_exit_code))),
        ("System", "Environment", "set_ExitCode") => {
            vm.env_exit_code = args.first().map_or(0, |v| v.as_i32());
            Ok(None)
        }
        ("System", "Environment", "Exit") => {
            let code = args.first().map_or(0, |v| v.as_i32());
            vm.env_exit_code = code;
            std::process::exit(code);
        }

        // ---- System.Runtime.InteropServices.Marshal ----
        ("System.Runtime.InteropServices", "Marshal", mname) => {
            dispatch_marshal(vm, mname, sig, args)
        }

        // ---- System.Runtime.CompilerServices.RuntimeHelpers ----
        ("System.Runtime.CompilerServices", "RuntimeHelpers", "InitializeArray") => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let packed = args.get(1).map_or(0, |v| v.as_u64_bits());
            let aid = (packed >> 32) as usize;
            let ftok = packed as u32;
            let frid = ftok & 0x00FF_FFFF;
            if let Some(HeapObjKind::Array { elem_size, length, .. }) = vm.heap.objects.get(&arr) {
                let byte_len = elem_size * length;
                if let Some(img) = vm.assemblies.get(aid).and_then(|a| a.image.as_ref()) {
                    if let Some(frva) = img.field_rvas.iter().find(|r| r.field == frid) {
                        if let Some(src) = img.bytes_at_rva(frva.rva, byte_len) {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    src.as_ptr(),
                                    (arr + MONO_ARRAY_DATA_OFFSET) as *mut u8,
                                    byte_len,
                                );
                            }
                        }
                    }
                }
            }
            Ok(None)
        }
        ("System.Runtime.CompilerServices", "RuntimeHelpers", "GetHashCode") => {
            let p = args.first().map_or(0, |v| v.as_u64_bits());
            Ok(Some(CilVal::I32(((p >> 32) ^ p) as i32)))
        }

        // ---- System.Console ----
        ("System", "Console", "Write" | "WriteLine") => {
            let msg = if args.is_empty() {
                String::new()
            } else if args.len() == 1 {
                vm.format_val_to_string(args[0])
            } else {
                let mut fmt_s = vm.format_val_to_string(args[0]);
                for (i, &a) in args.iter().skip(1).enumerate() {
                    fmt_s = fmt_s.replace(&format!("{{{i}}}"), &vm.format_val_to_string(a));
                }
                fmt_s
            };
            log::info!("clr[Console]: {msg}");
            Ok(None)
        }

        // ---- System.Text.StringBuilder ----
        ("System.Text", "StringBuilder", "Append") => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            let piece = if let Some(&v) = args.get(1) {
                if matches!(sig.params.first(), Some(SigType::Char)) {
                    char::from_u32(v.as_i32() as u32)
                        .map_or_else(String::new, |c| c.to_string())
                } else {
                    vm.format_val_to_string(v)
                }
            } else {
                String::new()
            };
            if let Some(HeapObjKind::StringBuilder { buf }) = vm.heap.objects.get_mut(&sb) {
                buf.push_str(&piece);
            }
            Ok(Some(CilVal::Obj(sb)))
        }
        ("System.Text", "StringBuilder", "ToString") => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            let s = match vm.heap.objects.get(&sb) {
                Some(HeapObjKind::StringBuilder { buf }) => buf.clone(),
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        ("System.Text", "StringBuilder", "get_Length") => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            let l = match vm.heap.objects.get(&sb) {
                Some(HeapObjKind::StringBuilder { buf }) => buf.encode_utf16().count() as i32,
                _ => 0,
            };
            Ok(Some(CilVal::I32(l)))
        }
        ("System.Text", "StringBuilder", "Clear") => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            if let Some(HeapObjKind::StringBuilder { buf }) = vm.heap.objects.get_mut(&sb) {
                buf.clear();
            }
            Ok(Some(CilVal::Obj(sb)))
        }

        // ---- System.Exception ----
        ("System", _, "get_Message") => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let msg = match vm.heap.objects.get(&obj) {
                Some(HeapObjKind::Exception { message, .. }) => message.clone(),
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&msg))))
        }

        // ---- System.BitConverter ----
        ("System", "BitConverter", "SingleToInt32Bits") => {
            Ok(Some(CilVal::I32((args.first().map_or(0.0, |v| v.as_f64()) as f32).to_bits() as i32)))
        }
        ("System", "BitConverter", "Int32BitsToSingle") => {
            Ok(Some(CilVal::F64(f32::from_bits(args.first().map_or(0, |v| v.as_i32()) as u32) as f64)))
        }
        ("System", "BitConverter", "DoubleToInt64Bits") => {
            Ok(Some(CilVal::I64(args.first().map_or(0.0, |v| v.as_f64()).to_bits() as i64)))
        }
        ("System", "BitConverter", "Int64BitsToDouble") => {
            Ok(Some(CilVal::F64(f64::from_bits(args.first().map_or(0, |v| v.as_i64()) as u64))))
        }

        // ---- System.IntPtr / UIntPtr ----
        ("System", "IntPtr" | "UIntPtr", "ToInt32") => {
            let v = read_ptr_val(vm, frames, args.first().copied().unwrap_or(CilVal::NativeInt(0)), &SigType::IntPtr);
            Ok(Some(CilVal::I32(v.as_i32())))
        }
        ("System", "IntPtr" | "UIntPtr", "ToInt64" | "ToPointer") => {
            let v = read_ptr_val(vm, frames, args.first().copied().unwrap_or(CilVal::NativeInt(0)), &SigType::IntPtr);
            Ok(Some(if method == "ToInt64" {
                CilVal::I64(v.as_i64())
            } else {
                CilVal::NativeInt(v.as_isize())
            }))
        }
        ("System", "IntPtr" | "UIntPtr", "op_Explicit") => {
            let a = args.first().copied().unwrap_or(CilVal::NativeInt(0));
            Ok(Some(match sig.ret {
                SigType::I4 | SigType::U4 => CilVal::I32(a.as_i32()),
                SigType::I8 | SigType::U8 => CilVal::I64(a.as_i64()),
                _ => CilVal::NativeInt(a.as_isize()),
            }))
        }
        ("System", "IntPtr" | "UIntPtr", "op_Equality") => {
            let eq = args.first().map_or(0, |v| v.as_isize()) == args.get(1).map_or(0, |v| v.as_isize());
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        ("System", "IntPtr" | "UIntPtr", "op_Inequality") => {
            let neq = args.first().map_or(0, |v| v.as_isize()) != args.get(1).map_or(0, |v| v.as_isize());
            Ok(Some(CilVal::I32(i32::from(neq))))
        }

        // ---- Primitives Parse / ToString / Convert ----
        ("System", "Int32", "Parse") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let v = s.trim().parse::<i32>().unwrap_or(0);
            Ok(Some(CilVal::I32(v)))
        }
        ("System", "Int64", "Parse") => {
            let s = vm.heap.read_string(args.first().map_or(0, |v| v.as_obj())).unwrap_or_default();
            let v = s.trim().parse::<i64>().unwrap_or(0);
            Ok(Some(CilVal::I64(v)))
        }
        ("System", "Int32" | "UInt32" | "Int64" | "UInt64" | "Single" | "Double" | "Boolean" | "Byte", "ToString") => {
            let p = args.first().copied().unwrap_or(CilVal::I32(0));
            let s = match ty {
                "Int32" => read_ptr_val(vm, frames, p, &SigType::I4).as_i32().to_string(),
                "UInt32" => (read_ptr_val(vm, frames, p, &SigType::U4).as_i32() as u32).to_string(),
                "Int64" => read_ptr_val(vm, frames, p, &SigType::I8).as_i64().to_string(),
                "UInt64" => (read_ptr_val(vm, frames, p, &SigType::U8).as_i64() as u64).to_string(),
                "Single" => format!("{}", read_ptr_val(vm, frames, p, &SigType::R4).as_f64() as f32),
                "Double" => format!("{}", read_ptr_val(vm, frames, p, &SigType::R8).as_f64()),
                "Boolean" => {
                    if read_ptr_val(vm, frames, p, &SigType::Boolean).is_truthy() {
                        "True".to_string()
                    } else {
                        "False".to_string()
                    }
                }
                _ => (read_ptr_val(vm, frames, p, &SigType::U1).as_i32() as u8).to_string(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        ("System", "Convert", "ToInt32") => {
            let a = args.first().copied().unwrap_or(CilVal::I32(0));
            let v = if let Some(s) = vm.heap.read_string(a.as_obj()) {
                s.trim().parse::<i32>().unwrap_or(0)
            } else {
                a.as_i32()
            };
            Ok(Some(CilVal::I32(v)))
        }
        ("System", "Convert", "ToInt64") => {
            let a = args.first().copied().unwrap_or(CilVal::I64(0));
            let v = if let Some(s) = vm.heap.read_string(a.as_obj()) {
                s.trim().parse::<i64>().unwrap_or(0)
            } else {
                a.as_i64()
            };
            Ok(Some(CilVal::I64(v)))
        }
        ("System", "Convert", "ToDouble") => {
            let a = args.first().copied().unwrap_or(CilVal::F64(0.0));
            let v = if let Some(s) = vm.heap.read_string(a.as_obj()) {
                s.trim().parse::<f64>().unwrap_or(0.0)
            } else {
                a.as_f64()
            };
            Ok(Some(CilVal::F64(v)))
        }
        ("System", "Convert", "ToString") => {
            let s = vm.format_val_to_string(args.first().copied().unwrap_or(CilVal::Obj(0)));
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }

        // ---- System.GC ----
        ("System", "GC", "Collect" | "SuppressFinalize" | "KeepAlive" | "WaitForPendingFinalizers") => {
            Ok(None)
        }

        // ---- Base .ctor fallback ----
        (_, _, ".ctor") => Ok(None),

        _ => Err(VmError::Unsupported(format!(
            "unimplemented BCL method {ns}.{ty}::{method}"
        ))),
    }
}

fn alloc_type_obj(vm: &mut ClrVm, represented_class_id: usize) -> usize {
    let vt = vm.classes[vm.core.type_cls].mono_vtable;
    let ptr = vm.heap.alloc_raw(MONO_OBJECT_HEADER_SIZE);
    unsafe {
        std::ptr::write_unaligned(ptr as *mut usize, vt);
    }
    let addr = ptr as usize;
    vm.heap.objects.insert(
        addr,
        HeapObjKind::TypeObj {
            represented_class_id,
        },
    );
    addr
}

fn dispatch_math(method: &str, sig: &MethodSig, args: &[CilVal]) -> CilVal {
    let a = args.first().copied().unwrap_or(CilVal::I32(0));
    let b = args.get(1).copied().unwrap_or(CilVal::I32(0));
    let c = args.get(2).copied().unwrap_or(CilVal::I32(0));
    let is_f = matches!(sig.ret, SigType::R4 | SigType::R8);
    let is_i64 = matches!(sig.ret, SigType::I8 | SigType::U8);

    match method {
        "Abs" => {
            if is_f {
                CilVal::F64(a.as_f64().abs())
            } else if is_i64 {
                CilVal::I64(a.as_i64().wrapping_abs())
            } else {
                CilVal::I32(a.as_i32().wrapping_abs())
            }
        }
        "Min" => {
            if is_f {
                CilVal::F64(a.as_f64().min(b.as_f64()))
            } else if is_i64 {
                CilVal::I64(a.as_i64().min(b.as_i64()))
            } else {
                CilVal::I32(a.as_i32().min(b.as_i32()))
            }
        }
        "Max" => {
            if is_f {
                CilVal::F64(a.as_f64().max(b.as_f64()))
            } else if is_i64 {
                CilVal::I64(a.as_i64().max(b.as_i64()))
            } else {
                CilVal::I32(a.as_i32().max(b.as_i32()))
            }
        }
        "Clamp" => {
            if is_f {
                CilVal::F64(a.as_f64().clamp(b.as_f64(), c.as_f64()))
            } else if is_i64 {
                CilVal::I64(a.as_i64().clamp(b.as_i64(), c.as_i64()))
            } else {
                CilVal::I32(a.as_i32().clamp(b.as_i32(), c.as_i32()))
            }
        }
        "Sign" => {
            let s = if matches!(sig.params.first(), Some(SigType::R4 | SigType::R8)) {
                let x = a.as_f64();
                if x > 0.0 { 1 } else if x < 0.0 { -1 } else { 0 }
            } else {
                let x = a.as_i64();
                if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
            };
            CilVal::I32(s)
        }
        "Sqrt" => CilVal::F64(a.as_f64().sqrt()),
        "Pow" => CilVal::F64(a.as_f64().powf(b.as_f64())),
        "Exp" => CilVal::F64(a.as_f64().exp()),
        "Log" => {
            if args.len() >= 2 {
                CilVal::F64(a.as_f64().log(b.as_f64()))
            } else {
                CilVal::F64(a.as_f64().ln())
            }
        }
        "Log10" => CilVal::F64(a.as_f64().log10()),
        "Log2" => CilVal::F64(a.as_f64().log2()),
        "Sin" => CilVal::F64(a.as_f64().sin()),
        "Cos" => CilVal::F64(a.as_f64().cos()),
        "Tan" => CilVal::F64(a.as_f64().tan()),
        "Asin" => CilVal::F64(a.as_f64().asin()),
        "Acos" => CilVal::F64(a.as_f64().acos()),
        "Atan" => CilVal::F64(a.as_f64().atan()),
        "Atan2" => CilVal::F64(a.as_f64().atan2(b.as_f64())),
        "Sinh" => CilVal::F64(a.as_f64().sinh()),
        "Cosh" => CilVal::F64(a.as_f64().cosh()),
        "Tanh" => CilVal::F64(a.as_f64().tanh()),
        "Floor" => CilVal::F64(a.as_f64().floor()),
        "Ceiling" => CilVal::F64(a.as_f64().ceil()),
        "Round" => CilVal::F64(a.as_f64().round()),
        "Truncate" => CilVal::F64(a.as_f64().trunc()),
        "FusedMultiplyAdd" => CilVal::F64(a.as_f64().mul_add(b.as_f64(), c.as_f64())),
        _ => CilVal::F64(0.0),
    }
}

fn dispatch_marshal(
    vm: &mut ClrVm,
    method: &str,
    sig: &MethodSig,
    args: &[CilVal],
) -> Result<Option<CilVal>, VmError> {
    let ptr_with_ofs = |args: &[CilVal], ofs_idx: usize| -> usize {
        let base = args.first().map_or(0, |v| v.as_isize() as usize);
        let ofs = args.get(ofs_idx).map_or(0, |v| v.as_i32() as isize);
        (base as isize).wrapping_add(ofs) as usize
    };
    match method {
        "AllocHGlobal" | "AllocCoTaskMem" => {
            let sz = args.first().map_or(0, |v| v.as_isize()).max(1) as usize;
            let p = unsafe { libc::calloc(1, sz) };
            Ok(Some(CilVal::NativeInt(p as usize as isize)))
        }
        "ReAllocHGlobal" => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *mut c_void;
            let sz = args.get(1).map_or(0, |v| v.as_isize()).max(1) as usize;
            let np = unsafe { libc::realloc(p, sz) };
            Ok(Some(CilVal::NativeInt(np as usize as isize)))
        }
        "FreeHGlobal" | "FreeCoTaskMem" => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *mut c_void;
            if !p.is_null() {
                unsafe { libc::free(p) };
            }
            Ok(None)
        }
        "ReadByte" => {
            let p = if args.len() >= 2 { ptr_with_ofs(args, 1) } else { args[0].as_isize() as usize };
            let v = unsafe { std::ptr::read_unaligned(p as *const u8) };
            Ok(Some(CilVal::I32(v as i32)))
        }
        "ReadInt16" => {
            let p = if args.len() >= 2 { ptr_with_ofs(args, 1) } else { args[0].as_isize() as usize };
            let v = unsafe { std::ptr::read_unaligned(p as *const i16) };
            Ok(Some(CilVal::I32(v as i32)))
        }
        "ReadInt32" => {
            let p = if args.len() >= 2 { ptr_with_ofs(args, 1) } else { args[0].as_isize() as usize };
            let v = unsafe { std::ptr::read_unaligned(p as *const i32) };
            Ok(Some(CilVal::I32(v)))
        }
        "ReadInt64" => {
            let p = if args.len() >= 2 { ptr_with_ofs(args, 1) } else { args[0].as_isize() as usize };
            let v = unsafe { std::ptr::read_unaligned(p as *const i64) };
            Ok(Some(CilVal::I64(v)))
        }
        "ReadIntPtr" => {
            let p = if args.len() >= 2 { ptr_with_ofs(args, 1) } else { args[0].as_isize() as usize };
            let v = unsafe { std::ptr::read_unaligned(p as *const isize) };
            Ok(Some(CilVal::NativeInt(v)))
        }
        "WriteByte" => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i32() as u8)
            } else {
                (args[0].as_isize() as usize, args[1].as_i32() as u8)
            };
            unsafe { std::ptr::write_unaligned(p as *mut u8, val) };
            Ok(None)
        }
        "WriteInt16" => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i32() as i16)
            } else {
                (args[0].as_isize() as usize, args[1].as_i32() as i16)
            };
            unsafe { std::ptr::write_unaligned(p as *mut i16, val) };
            Ok(None)
        }
        "WriteInt32" => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i32())
            } else {
                (args[0].as_isize() as usize, args[1].as_i32())
            };
            unsafe { std::ptr::write_unaligned(p as *mut i32, val) };
            Ok(None)
        }
        "WriteInt64" => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i64())
            } else {
                (args[0].as_isize() as usize, args[1].as_i64())
            };
            unsafe { std::ptr::write_unaligned(p as *mut i64, val) };
            Ok(None)
        }
        "WriteIntPtr" => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_isize())
            } else {
                (args[0].as_isize() as usize, args[1].as_isize())
            };
            unsafe { std::ptr::write_unaligned(p as *mut isize, val) };
            Ok(None)
        }
        "Copy" => {
            if args.len() >= 4 {
                if matches!(sig.params.first(), Some(SigType::SzArray(_))) {
                    // Copy(T[] source, int startIndex, IntPtr destination, int length)
                    let arr = args[0].as_obj();
                    let start = args[1].as_i32().max(0) as usize;
                    let dst = args[2].as_isize() as usize;
                    let cnt = args[3].as_i32().max(0) as usize;
                    if let Some(HeapObjKind::Array { elem_size, length, .. }) = vm.heap.objects.get(&arr) {
                        let n = cnt.min(length.saturating_sub(start));
                        if dst != 0 && n > 0 {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    (arr + MONO_ARRAY_DATA_OFFSET + start * elem_size) as *const u8,
                                    dst as *mut u8,
                                    n * elem_size,
                                );
                            }
                        }
                    }
                } else {
                    // Copy(IntPtr source, T[] destination, int startIndex, int length)
                    let src = args[0].as_isize() as usize;
                    let arr = args[1].as_obj();
                    let start = args[2].as_i32().max(0) as usize;
                    let cnt = args[3].as_i32().max(0) as usize;
                    if let Some(HeapObjKind::Array { elem_size, length, .. }) = vm.heap.objects.get(&arr) {
                        let n = cnt.min(length.saturating_sub(start));
                        if src != 0 && n > 0 {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    src as *const u8,
                                    (arr + MONO_ARRAY_DATA_OFFSET + start * elem_size) as *mut u8,
                                    n * elem_size,
                                );
                            }
                        }
                    }
                }
            }
            Ok(None)
        }
        "StringToHGlobalAnsi" | "StringToCoTaskMemAnsi" | "StringToCoTaskMemUTF8" => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let bytes = s.as_bytes();
            let p = unsafe { libc::malloc(bytes.len() + 1) } as *mut u8;
            if !p.is_null() {
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
                    std::ptr::write_unaligned(p.add(bytes.len()), 0u8);
                }
            }
            Ok(Some(CilVal::NativeInt(p as usize as isize)))
        }
        "StringToHGlobalUni" | "StringToCoTaskMemUni" => {
            let u16s = vm
                .heap
                .read_string_utf16(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let p = unsafe { libc::malloc((u16s.len() + 1) * 2) } as *mut u16;
            if !p.is_null() {
                unsafe {
                    for (i, &c) in u16s.iter().enumerate() {
                        std::ptr::write_unaligned(p.add(i), c);
                    }
                    std::ptr::write_unaligned(p.add(u16s.len()), 0u16);
                }
            }
            Ok(Some(CilVal::NativeInt(p as usize as isize)))
        }
        "PtrToStringAnsi" | "PtrToStringUTF8" => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *const u8;
            if p.is_null() {
                Ok(Some(CilVal::Obj(0)))
            } else {
                let s = unsafe { crate::strings::from_ansi(p) };
                Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
            }
        }
        "PtrToStringUni" => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *const u16;
            if p.is_null() {
                Ok(Some(CilVal::Obj(0)))
            } else {
                let s = unsafe { crate::strings::from_wide(p) };
                Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
            }
        }
        "GetLastWin32Error" => Ok(Some(CilVal::I32(crate::teb::last_error() as i32))),
        "SizeOf" => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let sz = match vm.heap.objects.get(&obj) {
                Some(HeapObjKind::TypeObj { represented_class_id }) => {
                    vm.classes[*represented_class_id].elem_size
                }
                _ => vm
                    .object_class_id(obj)
                    .map_or(8, |cid| vm.classes[cid].elem_size),
            };
            Ok(Some(CilVal::I32(sz as i32)))
        }
        "GetFunctionPointerForDelegate" => {
            let del_obj = args.first().map_or(0, |v| v.as_obj());
            if let Some(HeapObjKind::Delegate {
                method_id,
                target_obj,
                ..
            }) = vm.heap.objects.get(&del_obj).cloned()
            {
                let asm_id = vm.methods[method_id].asm_id;
                let fp = super::alloc_delegate_trampoline(super::DelegateBinding {
                    asm_id,
                    method_id,
                    target_obj,
                    mode: super::DelegateCallMode::DirectAnsi,
                })?;
                Ok(Some(CilVal::NativeInt(fp as isize)))
            } else {
                Ok(Some(CilVal::NativeInt(0)))
            }
        }
        _ => Err(VmError::Unsupported(format!(
            "unimplemented System.Runtime.InteropServices.Marshal::{method}"
        ))),
    }
}

// ---------------------------------------------------------------------------
// Mono Method Descriptor Parser
// ---------------------------------------------------------------------------

pub fn parse_mono_method_desc(spec: &str, include_namespace: bool) -> MonoMethodDesc {
    let spec = spec.trim();
    let (before_params, param_count) = if let Some(lp) = spec.find('(') {
        let rp = spec.rfind(')').unwrap_or(spec.len());
        let inside = spec[lp + 1..rp].trim();
        let cnt = if inside.is_empty() {
            0
        } else {
            inside.split(',').count()
        };
        (&spec[..lp], Some(cnt))
    } else {
        (spec, None)
    };

    let (class_part, method_name) = match before_params.rfind(':') {
        Some(pos) => (&before_params[..pos], before_params[pos + 1..].trim()),
        None => ("", before_params.trim()),
    };
    let class_part = class_part.trim_end_matches(':').trim();

    let (namespace, class_name) = if class_part.is_empty() {
        (None, None)
    } else if include_namespace {
        let (ns, cls) = split_full_type_name(class_part);
        (
            if ns.is_empty() { None } else { Some(ns.to_string()) },
            Some(cls.to_string()),
        )
    } else {
        (None, Some(class_part.to_string()))
    };

    MonoMethodDesc {
        namespace,
        class_name,
        method_name: method_name.to_string(),
        param_count,
        include_namespace,
    }
}

pub fn match_method_desc_in_class(
    vm: &ClrVm,
    desc: &MonoMethodDesc,
    class_id: usize,
) -> Option<usize> {
    let cls = vm.classes.get(class_id)?;
    if let Some(req_cls) = &desc.class_name {
        if cls.name != *req_cls {
            return None;
        }
    }
    if desc.include_namespace {
        if let Some(req_ns) = &desc.namespace {
            if cls.namespace != *req_ns {
                return None;
            }
        }
    }
    let pcnt = desc.param_count.map_or(-1, |c| c as i32);
    vm.find_method_in_class(class_id, &desc.method_name, pcnt)
}

#[cfg(test)]
mod tests {
    use super::super::metadata::elem_type;
    use super::super::metadata::test_asm::{TestAsmBuilder, TestMethodBody};
    use super::super::metadata::ExceptionClause;
    use super::*;

    #[test]
    fn vm_executes_arithmetic_loops_arrays_strings_and_exceptions() {
        let mut b = TestAsmBuilder::new("VmCoreTest");
        let corlib = b.add_assembly_ref("mscorlib");
        let obj_ref = b.add_type_ref(corlib, "System", "Object");
        let str_ref = b.add_type_ref(corlib, "System", "String");
        let math_ref = b.add_type_ref(corlib, "System", "Math");
        let exc_ref = b.add_type_ref(corlib, "System", "Exception");
        let i4_ref = b.add_type_ref(corlib, "System", "Int32");

        let concat_mr = b.add_member_ref_on_typeref(
            str_ref,
            "Concat",
            &[0x00, 0x02, elem_type::STRING, elem_type::STRING, elem_type::STRING],
        );
        let strlen_mr = b.add_member_ref_on_typeref(
            str_ref,
            "get_Length",
            &[0x20, 0x00, elem_type::I4],
        );
        let math_max_mr = b.add_member_ref_on_typeref(
            math_ref,
            "Max",
            &[0x00, 0x02, elem_type::I4, elem_type::I4, elem_type::I4],
        );
        let exc_ctor_mr = b.add_member_ref_on_typeref(
            exc_ref,
            ".ctor",
            &[0x20, 0x01, elem_type::VOID, elem_type::STRING],
        );

        let _td = b.add_type_def("Tests", "Kernel", Some(obj_ref));
        let locals_loop = b.add_standalone_locals(&[elem_type::I4, elem_type::I4]);

        // Method 1: SumTo(int n) -> sum 1..=n using loop + locals
        // local0 = sum = 0; local1 = i = 1;
        // loop: if i > n goto done; sum += i; i += 1; br loop; done: ret sum
        let sum_code = vec![
            0x16, 0x0A, // ldc.i4.0; stloc.0
            0x17, 0x0B, // ldc.i4.1; stloc.1
            // loop @ 4:
            0x07, 0x02, 0x30, 0x0C, // ldloc.1; ldarg.0; bgt.s +12 (-> offset 20)
            0x06, 0x07, 0x58, 0x0A, // ldloc.0; ldloc.1; add; stloc.0
            0x07, 0x17, 0x58, 0x0B, // ldloc.1; ldc.i4.1; add; stloc.1
            0x2B, (-14i8) as u8, // br.s -14 (-> offset 4)
            // done @ 18? Wait: 4 + 4 = 8; 8 + 4 = 12; 12 + 4 = 16; 16 + 2 = 18!
            // Let's make offsets exact: bgt.s at 6..8 -> next_ip = 8; target = 18 => +10.
            // br.s at 16..18 -> next_ip = 18; target = 4 => -14.
            0x06, 0x2A, // ldloc.0; ret
        ];
        let mut sum_code = sum_code;
        sum_code[7] = 10;
        let _m_sum = b.add_method(
            0x0016,
            0,
            "SumTo",
            &[0x00, 0x01, elem_type::I4, elem_type::I4],
            Some(TestMethodBody::fat(sum_code, 8, locals_loop, Vec::new())),
        );

        // Method 2: StringAndMath() -> String.Concat("Hi ", "CLR").Length + Math.Max(10, 32)
        let s1 = b.add_user_string("Hi ");
        let s2 = b.add_user_string("CLR");
        let mut sm_code = Vec::new();
        sm_code.push(0x72);
        sm_code.extend_from_slice(&s1.to_le_bytes());
        sm_code.push(0x72);
        sm_code.extend_from_slice(&s2.to_le_bytes());
        sm_code.push(0x28);
        sm_code.extend_from_slice(&concat_mr.to_le_bytes());
        sm_code.push(0x6F);
        sm_code.extend_from_slice(&strlen_mr.to_le_bytes()); // 6
        sm_code.extend_from_slice(&[0x1F, 10, 0x1F, 32, 0x28]);
        sm_code.extend_from_slice(&math_max_mr.to_le_bytes()); // 32
        sm_code.push(0x58); // add -> 38
        sm_code.push(0x2A); // ret
        let _m_sm = b.add_method(
            0x0016,
            0,
            "StringAndMath",
            &[0x00, 0x00, elem_type::I4],
            Some(TestMethodBody::tiny(sm_code)),
        );

        // Method 3: ArrayAndTryCatch() -> allocates int[3], writes 7, catches thrown Exception, adds 100 in finally -> 107
        let locals_arr = b.add_standalone_locals(&[elem_type::I4]);
        let boom_str = b.add_user_string("boom");
        let i4_tok = ((table::TYPE_REF as u32) << 24) | i4_ref;
        let exc_tok = ((table::TYPE_REF as u32) << 24) | exc_ref;
        let mut tc_code = Vec::new();
        // newarr Int32 (len 3), store [1] = 7, load [1] -> local0
        tc_code.push(0x19); // ldc.i4.3
        tc_code.push(0x8D);
        tc_code.extend_from_slice(&i4_tok.to_le_bytes());
        tc_code.extend_from_slice(&[0x25, 0x17, 0x1D, 0x9E]); // dup; ldc.i4.1; ldc.i4.7; stelem.i4
        tc_code.extend_from_slice(&[0x17, 0x94, 0x0A]); // ldc.i4.1; ldelem.i4; stloc.0
        let try_outer_start = tc_code.len();
        let try_inner_start = tc_code.len();
        // throw new Exception("boom")
        tc_code.push(0x72);
        tc_code.extend_from_slice(&boom_str.to_le_bytes());
        tc_code.push(0x73);
        tc_code.extend_from_slice(&exc_ctor_mr.to_le_bytes());
        tc_code.push(0x7A); // throw
        let try_inner_len = tc_code.len() - try_inner_start;
        // catch handler: pop exc; local0 += 5; leave.s after_finally
        let catch_start = tc_code.len();
        tc_code.extend_from_slice(&[0x26, 0x06, 0x1B, 0x58, 0x0A]); // pop; ldloc.0; ldc.i4.5; add; stloc.0
        let leave_pos = tc_code.len();
        tc_code.extend_from_slice(&[0xDE, 0x00]); // leave.s (patched below)
        let try_outer_len = tc_code.len() - try_outer_start;
        // finally handler: local0 += 100; endfinally
        let fin_start = tc_code.len();
        tc_code.extend_from_slice(&[0x06, 0x1F, 100, 0x58, 0x0A, 0xDC]);
        let after_pos = tc_code.len();
        tc_code[leave_pos + 1] = (after_pos - (leave_pos + 2)) as u8;
        tc_code.extend_from_slice(&[0x06, 0x2A]); // ldloc.0; ret

        let eh = vec![
            ExceptionClause {
                flags: eh_flags::EXCEPTION,
                try_offset: try_inner_start,
                try_length: try_inner_len,
                handler_offset: catch_start,
                class_token_or_filter_offset: exc_tok,
            },
            ExceptionClause {
                flags: eh_flags::FINALLY,
                try_offset: try_outer_start,
                try_length: try_outer_len,
                handler_offset: fin_start,
                class_token_or_filter_offset: 0,
            },
        ];
        let _m_tc = b.add_method(
            0x0016,
            0,
            "ArrayAndTryCatch",
            &[0x00, 0x00, elem_type::I4],
            Some(TestMethodBody::fat(tc_code, 8, locals_arr, eh)),
        );

        let pe = b.build();
        let (mid_sum, mid_sm, mid_tc) = {
            let mut vm = lock_vm();
            let aid = vm.load_assembly_bytes(&pe, None).expect("load assembly");
            let cid = vm.find_class_in_assembly(aid, "Tests", "Kernel").unwrap();
            (
                vm.find_method_in_class(cid, "SumTo", 1).unwrap(),
                vm.find_method_in_class(cid, "StringAndMath", 0).unwrap(),
                vm.find_method_in_class(cid, "ArrayAndTryCatch", 0).unwrap(),
            )
        };

        let r1 = execute_method(mid_sum, vec![CilVal::I32(10)]).unwrap().unwrap();
        assert_eq!(r1.as_i32(), 55);

        let r2 = execute_method(mid_sm, Vec::new()).unwrap().unwrap();
        assert_eq!(r2.as_i32(), 38);

        let r3 = execute_method(mid_tc, Vec::new()).unwrap().unwrap();
        assert_eq!(r3.as_i32(), 112); // 7 + 5 + 100
    }

    #[test]
    fn vm_executes_cctor_boxing_switch_buffer_marshal_and_environment() {
        let mut b = TestAsmBuilder::new("VmAdvancedTest");
        let corlib = b.add_assembly_ref("mscorlib");
        let obj_ref = b.add_type_ref(corlib, "System", "Object");
        let i4_ref = b.add_type_ref(corlib, "System", "Int32");
        let byte_ref = b.add_type_ref(corlib, "System", "Byte");
        let buf_ref = b.add_type_ref(corlib, "System", "Buffer");
        let env_ref = b.add_type_ref(corlib, "System", "Environment");
        let marshal_ref = b.add_type_ref(corlib, "System.Runtime.InteropServices", "Marshal");

        let block_copy_mr = b.add_member_ref_on_typeref(
            buf_ref,
            "BlockCopy",
            &[
                0x00, 0x05, elem_type::VOID, elem_type::OBJECT, elem_type::I4,
                elem_type::OBJECT, elem_type::I4, elem_type::I4,
            ],
        );
        let is64_mr = b.add_member_ref_on_typeref(
            env_ref,
            "get_Is64BitProcess",
            &[0x00, 0x00, elem_type::BOOLEAN],
        );
        let alloc_hg_mr = b.add_member_ref_on_typeref(
            marshal_ref,
            "AllocHGlobal",
            &[0x00, 0x01, elem_type::I, elem_type::I4],
        );
        let write_i32_mr = b.add_member_ref_on_typeref(
            marshal_ref,
            "WriteInt32",
            &[0x00, 0x02, elem_type::VOID, elem_type::I, elem_type::I4],
        );
        let read_i32_mr = b.add_member_ref_on_typeref(
            marshal_ref,
            "ReadInt32",
            &[0x00, 0x01, elem_type::I4, elem_type::I],
        );
        let free_hg_mr = b.add_member_ref_on_typeref(
            marshal_ref,
            "FreeHGlobal",
            &[0x00, 0x01, elem_type::VOID, elem_type::I],
        );

        let _td = b.add_type_def("Tests", "Advanced", Some(obj_ref));
        let f_seed = b.add_field(0x0016, "Seed", elem_type::I4); // Public | Static

        // .cctor sets Seed = 20
        let mut cctor_code = vec![0x1F, 20, 0x80];
        cctor_code.extend_from_slice(&f_seed.to_le_bytes());
        cctor_code.push(0x2A);
        let _m_cctor = b.add_method(
            0x1816,
            0,
            ".cctor",
            &[0x00, 0x00, elem_type::VOID],
            Some(TestMethodBody::tiny(cctor_code)),
        );

        // BoxSwitchAndBcl(int sel):
        // 1) switch(sel): case 0 -> 10, case 1 -> 20, default -> 30
        // 2) box/unbox.any Int32
        // 3) add Seed (triggers .cctor -> +20)
        // 4) Buffer.BlockCopy byte[4]{0x05,0,0,0} -> byte[4], add dst[0] (+5)
        // 5) Marshal.AllocHGlobal(8) / WriteInt32(7) / ReadInt32 / FreeHGlobal (+7)
        // 6) Environment.Is64BitProcess (+1)
        let locals = b.add_standalone_locals(&[
            elem_type::I4,     // loc0 = acc
            elem_type::OBJECT, // loc1 = src byte[]
            elem_type::OBJECT, // loc2 = dst byte[]
            elem_type::I,      // loc3 = hglobal ptr
        ]);
        let i4_tok = ((table::TYPE_REF as u32) << 24) | i4_ref;
        let byte_tok = ((table::TYPE_REF as u32) << 24) | byte_ref;
        let mut code = Vec::new();
        // switch(arg0) with 2 cases (offsets relative to end of switch table)
        // case 0: ldc.i4.s 10 (2 bytes), br.s +7 (2 bytes) = 4 bytes
        // case 1: ldc.i4.s 20 (2 bytes), br.s +3 (2 bytes) = 4 bytes
        // default: ldc.i4.s 30 (2 bytes) -> falls through to after_sw
        // layout after switch:
        //  +0: default: 1F 1E (2B), br.s +8 (2B) -> +12
        //  +4: case0:   1F 0A (2B), br.s +4 (2B) -> +12
        //  +8: case1:   1F 14 (2B)               -> +10... wait let's put default right after switch:
        //  +0: default: 1F 1E (2B), br.s +8 -> +12
        //  +4: case0:   1F 0A (2B), br.s +4 -> +12
        //  +8: case1:   1F 14 (2B), br.s +0 -> +12 (4B)
        code.push(0x02); // ldarg.0
        code.push(0x45); // switch
        code.extend_from_slice(&2u32.to_le_bytes());
        code.extend_from_slice(&4i32.to_le_bytes()); // case 0 -> +4
        code.extend_from_slice(&8i32.to_le_bytes()); // case 1 -> +8
        code.extend_from_slice(&[0x1F, 30, 0x2B, 8]); // default
        code.extend_from_slice(&[0x1F, 10, 0x2B, 4]); // case 0
        code.extend_from_slice(&[0x1F, 20, 0x2B, 0]); // case 1
        // box Int32; unbox.any Int32
        code.push(0x8C);
        code.extend_from_slice(&i4_tok.to_le_bytes());
        code.push(0xA5);
        code.extend_from_slice(&i4_tok.to_le_bytes());
        // + ldsfld Seed (+20)
        code.push(0x7E);
        code.extend_from_slice(&f_seed.to_le_bytes());
        code.extend_from_slice(&[0x58, 0x0A]); // add; stloc.0
        // loc1 = new byte[4]; loc1[0] = 5; loc2 = new byte[4]
        code.push(0x1A); // ldc.i4.4
        code.push(0x8D);
        code.extend_from_slice(&byte_tok.to_le_bytes());
        code.extend_from_slice(&[0x25, 0x16, 0x1B, 0x9C, 0x0B]); // dup; 0; 5; stelem.i1; stloc.1
        code.push(0x1A); // ldc.i4.4
        code.push(0x8D);
        code.extend_from_slice(&byte_tok.to_le_bytes());
        code.push(0x0C); // stloc.2
        // Buffer.BlockCopy(loc1, 0, loc2, 0, 4)
        code.extend_from_slice(&[0x07, 0x16, 0x08, 0x16, 0x1A, 0x28]);
        code.extend_from_slice(&block_copy_mr.to_le_bytes());
        // loc0 += loc2[0]
        code.extend_from_slice(&[0x06, 0x08, 0x16, 0x91, 0x58, 0x0A]); // ldloc.0; ldloc.2; 0; ldelem.u1; add; stloc.0
        // loc3 = Marshal.AllocHGlobal(8); WriteInt32(loc3, 7); loc0 += ReadInt32(loc3); FreeHGlobal(loc3)
        code.extend_from_slice(&[0x1E, 0x28]); // ldc.i4.8; call AllocHGlobal
        code.extend_from_slice(&alloc_hg_mr.to_le_bytes());
        code.extend_from_slice(&[0x0D, 0x09, 0x1D, 0x28]); // stloc.3; ldloc.3; ldc.i4.7; call WriteInt32
        code.extend_from_slice(&write_i32_mr.to_le_bytes());
        code.extend_from_slice(&[0x06, 0x09, 0x28]); // ldloc.0; ldloc.3; call ReadInt32
        code.extend_from_slice(&read_i32_mr.to_le_bytes());
        code.extend_from_slice(&[0x58, 0x0A, 0x09, 0x28]); // add; stloc.0; ldloc.3; call FreeHGlobal
        code.extend_from_slice(&free_hg_mr.to_le_bytes());
        // loc0 += Environment.Is64BitProcess (+1)
        code.extend_from_slice(&[0x06, 0x28]);
        code.extend_from_slice(&is64_mr.to_le_bytes());
        code.extend_from_slice(&[0x58, 0x2A]); // add; ret

        let _m_adv = b.add_method(
            0x0016,
            0,
            "BoxSwitchAndBcl",
            &[0x00, 0x01, elem_type::I4, elem_type::I4],
            Some(TestMethodBody::fat(code, 8, locals, Vec::new())),
        );

        let pe = b.build();
        let mid = {
            let mut vm = lock_vm();
            let aid = vm.load_assembly_bytes(&pe, None).unwrap();
            let cid = vm.find_class_in_assembly(aid, "Tests", "Advanced").unwrap();
            vm.find_method_in_class(cid, "BoxSwitchAndBcl", 1).unwrap()
        };

        // case 0: 10 + 20 + 5 + 7 + 1 = 43
        assert_eq!(execute_method(mid, vec![CilVal::I32(0)]).unwrap().unwrap().as_i32(), 43);
        // case 1: 20 + 20 + 5 + 7 + 1 = 53
        assert_eq!(execute_method(mid, vec![CilVal::I32(1)]).unwrap().unwrap().as_i32(), 53);
        // default: 30 + 20 + 5 + 7 + 1 = 63
        assert_eq!(execute_method(mid, vec![CilVal::I32(9)]).unwrap().unwrap().as_i32(), 63);
    }
}
