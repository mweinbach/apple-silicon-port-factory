//! Corlib initialization, O(N) ECMA-335 assembly loader with `HashMap` metadata indices,
//! O(1) class lookup by `(namespace, name)`, and `MonoMethodDesc` parsing/matching.

use super::super::metadata::{
    elem_type, field_attr, method_attr, method_semantics, table, AssemblyImage, SigType,
};
use super::heap::MONO_OBJECT_HEADER_SIZE;
use super::value::{default_val_for_sig, CilVal, StorageKind};
use super::{
    ClassInfo, ClrVm, CoreClasses, FieldInfo, LoadedAssembly, MethodInfo, MonoAssembly, MonoClass,
    MonoClassField, MonoDomain, MonoImage, MonoMethod, MonoMethodDesc, MonoProperty, MonoVTable,
    PropertyInfo, VmError,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

impl ClrVm {
    pub(crate) fn init_corlib(&mut self) {
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
            classes_by_short_name: HashMap::new(),
            field_rvas: HashMap::new(),
            user_strings: HashMap::new(),
            type_cache: HashMap::new(),
            field_cache: HashMap::new(),
            call_cache: HashMap::new(),
            newobj_cache: HashMap::new(),
            standalone_sig_cache: HashMap::new(),
            vdispatch_cache: HashMap::new(),
        });

        let obj = self.add_corlib_class("System", "Object", None, false, false, false, None, 8);
        let vt = self.add_corlib_class("System", "ValueType", Some(obj), false, false, false, None, 8);
        let _en = self.add_corlib_class("System", "Enum", Some(vt), true, false, false, None, 4);
        let str_c = self.add_corlib_class(
            "System",
            "String",
            Some(obj),
            false,
            false,
            false,
            Some(SigType::String),
            8,
        );
        let arr_c = self.add_corlib_class("System", "Array", Some(obj), false, false, false, None, 8);
        let exc = self.add_corlib_class("System", "Exception", Some(obj), false, false, true, None, 8);
        let sys_exc = self.add_corlib_class(
            "System",
            "SystemException",
            Some(exc),
            false,
            false,
            true,
            None,
            8,
        );
        let inv_cast = self.add_corlib_class(
            "System",
            "InvalidCastException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let null_ref = self.add_corlib_class(
            "System",
            "NullReferenceException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let idx_oor = self.add_corlib_class(
            "System",
            "IndexOutOfRangeException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let div0 = self.add_corlib_class(
            "System",
            "DivideByZeroException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let ovf = self.add_corlib_class(
            "System",
            "OverflowException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let _arg_exc = self.add_corlib_class(
            "System",
            "ArgumentException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let _arg_null = self.add_corlib_class(
            "System",
            "ArgumentNullException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let _not_impl = self.add_corlib_class(
            "System",
            "NotImplementedException",
            Some(sys_exc),
            false,
            false,
            true,
            None,
            8,
        );
        let type_c = self.add_corlib_class("System", "Type", Some(obj), false, false, false, None, 8);
        let del = self.add_corlib_class("System", "Delegate", Some(obj), false, true, false, None, 8);
        let _mdel = self.add_corlib_class(
            "System",
            "MulticastDelegate",
            Some(del),
            false,
            true,
            false,
            None,
            8,
        );
        let sb = self.add_corlib_class(
            "System.Text",
            "StringBuilder",
            Some(obj),
            false,
            false,
            false,
            None,
            8,
        );
        let list_cls = self.add_corlib_class(
            "System.Collections.Generic",
            "List`1",
            Some(obj),
            false,
            false,
            false,
            None,
            8,
        );
        let dict_cls = self.add_corlib_class(
            "System.Collections.Generic",
            "Dictionary`2",
            Some(obj),
            false,
            false,
            false,
            None,
            8,
        );

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
            list_cls,
            dict_cls,
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
        self.corlib_class_count = self.classes.len();
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
            cctor_method_id: None,
            cctor_ran: true,
            mono_class: c_ptr,
            mono_vtable: vt_ptr,
        });
        self.assemblies[0].class_ids.push(class_id);
        self.assemblies[0]
            .classes_by_short_name
            .entry(name.to_string())
            .or_default()
            .push(class_id);
        self.classes_by_short_name
            .entry(name.to_string())
            .or_default()
            .push(class_id);
        class_id
    }

    pub fn load_assembly_file(&mut self, path: &Path) -> Result<usize, VmError> {
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
        let parsed = Arc::new(AssemblyImage::parse(data).map_err(|e| VmError::BadImage(e.0))?);
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
        let mut asm_classes_by_short_name: HashMap<String, Vec<usize>> =
            HashMap::with_capacity(parsed.type_defs.len());

        // Pre-index sparse metadata tables in O(N) to avoid O(N^2) scans during loading.
        let field_layout_map: HashMap<u32, u32> = if parsed.field_layouts.is_empty() {
            HashMap::new()
        } else {
            parsed
                .field_layouts
                .iter()
                .map(|fl| (fl.field, fl.offset))
                .collect()
        };
        let constant_map: HashMap<u32, usize> = if parsed.constants.is_empty() {
            HashMap::new()
        } else {
            parsed
                .constants
                .iter()
                .enumerate()
                .map(|(idx, c)| (c.parent, idx))
                .collect()
        };
        let impl_map_by_method: HashMap<u32, usize> = if parsed.impl_maps.is_empty() {
            HashMap::new()
        } else {
            parsed
                .impl_maps
                .iter()
                .enumerate()
                .map(|(idx, r)| (r.member_forwarded, idx))
                .collect()
        };
        let field_rvas: HashMap<u32, u32> = if parsed.field_rvas.is_empty() {
            HashMap::new()
        } else {
            parsed
                .field_rvas
                .iter()
                .map(|r| (r.field, r.rva))
                .collect()
        };

        // Pass 1: allocate ClassInfo for every TypeDef and register in O(1) name indices.
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

            asm_classes_by_short_name
                .entry(name.clone())
                .or_default()
                .push(class_id);
            self.classes_by_short_name
                .entry(name.clone())
                .or_default()
                .push(class_id);

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
                cctor_method_id: None,
                cctor_ran: true,
                mono_class: c_ptr,
                mono_vtable: vt_ptr,
            });
            class_ids.push(class_id);
        }

        // Pass 2: link parent_class_id (O(1) via classes_by_short_name index).
        for (i, td) in parsed.type_defs.iter().enumerate() {
            let class_id = class_ids[i];
            if td.extends != 0 {
                let ext_tbl = (td.extends >> 24) as u8;
                let ext_rid = (td.extends & 0x00FF_FFFF) as usize;
                let parent =
                    if ext_tbl == table::TYPE_DEF && ext_rid >= 1 && ext_rid <= class_ids.len() {
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

        // Pass 3: fields and methods per TypeDef (all lookups O(1)).
        for (i, &class_id) in class_ids.iter().enumerate() {
            let td_rid = (i + 1) as u32;
            let mut inst_off = 0usize;

            for f_rid in parsed.type_field_range(td_rid) {
                let f_row = parsed.fields[(f_rid - 1) as usize];
                let fname = parsed.get_string(f_row.name).to_string();
                let fsig = parsed.parse_field_sig(f_row.signature);
                let fkind = StorageKind::from_sig(&fsig);
                let is_static = (f_row.flags & field_attr::STATIC) != 0
                    || (f_row.flags & field_attr::LITERAL) != 0;
                let fsize = fsig.elem_size().max(1);
                let align = fsize.min(8).next_power_of_two();
                let struct_offset = if is_static {
                    0
                } else if let Some(&explicit_off) = field_layout_map.get(&f_rid) {
                    let o = explicit_off as usize;
                    inst_off = inst_off.max(o + fsize);
                    o
                } else {
                    let aligned = (inst_off + align - 1) & !(align - 1);
                    inst_off = aligned + fsize;
                    aligned
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
                let init_val = if let Some(&c_idx) = constant_map.get(&field_tok) {
                    let c = parsed.constants[c_idx];
                    let blob = parsed.get_blob(c.value);
                    match c.elem_type {
                        elem_type::I4 | elem_type::U4 if blob.len() >= 4 => {
                            CilVal::I32(i32::from_le_bytes([blob[0], blob[1], blob[2], blob[3]]))
                        }
                        elem_type::I8 | elem_type::U8 if blob.len() >= 8 => {
                            CilVal::I64(i64::from_le_bytes([
                                blob[0], blob[1], blob[2], blob[3], blob[4], blob[5], blob[6],
                                blob[7],
                            ]))
                        }
                        elem_type::BOOLEAN | elem_type::U1 if !blob.is_empty() => {
                            CilVal::I32(blob[0] as i32)
                        }
                        elem_type::STRING => {
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
                    kind: fkind,
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
                    let m_tok = ((table::METHOD_DEF as u32) << 24) | m_rid;
                    impl_map_by_method.get(&m_tok).and_then(|&im_idx| {
                        let row = parsed.impl_maps[im_idx];
                        let mr = parsed
                            .module_refs
                            .get(row.import_scope.saturating_sub(1) as usize)?;
                        let dll = parsed.get_string(mr.name).to_string();
                        let imp = parsed.get_string(row.import_name);
                        let entry = if imp.is_empty() {
                            mname.clone()
                        } else {
                            imp.to_string()
                        };
                        Some((dll, entry, row.mapping_flags))
                    })
                } else {
                    None
                };

                let method_id = self.methods.len();
                let is_cctor = mname == ".cctor" && msig.params.is_empty();
                let argc = msig.params.len() + usize::from(msig.has_this && !msig.explicit_this);
                let is_virtual = (m_row.flags & method_attr::VIRTUAL) != 0;
                let is_void_ret = msig.ret == SigType::Void;

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
                    argc,
                    is_virtual,
                    is_void_ret,
                    body,
                    locals_sig,
                    local_init_plan: None,
                    pinvoke,
                    pinvoke_fn_ptr: None,
                    cached_target: None,
                    mono_method: m_ptr,
                });
                method_ids[(m_rid - 1) as usize] = method_id;
                self.classes[class_id].methods.push(method_id);
                if is_cctor {
                    self.classes[class_id].cctor_method_id = Some(method_id);
                    self.classes[class_id].cctor_ran = false;
                }
            }
        }

        // Pass 4: properties in O(PM + P + MS) using direct iteration & pre-indexed MethodSemantics.
        if !parsed.property_maps.is_empty() {
            let mut prop_accessors: HashMap<u32, (Option<u32>, Option<u32>)> =
                HashMap::with_capacity(parsed.properties.len());
            for ms in &parsed.method_semantics {
                if ((ms.association >> 24) as u8) == table::PROPERTY {
                    let p_rid = ms.association & 0x00FF_FFFF;
                    let entry = prop_accessors.entry(p_rid).or_insert((None, None));
                    if (ms.semantics & method_semantics::GETTER) != 0 {
                        entry.0 = Some(ms.method);
                    }
                    if (ms.semantics & method_semantics::SETTER) != 0 {
                        entry.1 = Some(ms.method);
                    }
                }
            }

            let total_props = parsed.properties.len() as u32 + 1;
            for (pm_idx, pm) in parsed.property_maps.iter().enumerate() {
                let td_idx = pm.parent.saturating_sub(1) as usize;
                let Some(&class_id) = class_ids.get(td_idx) else {
                    continue;
                };
                let start = pm.property_list.max(1);
                let end = parsed
                    .property_maps
                    .get(pm_idx + 1)
                    .map(|n| n.property_list)
                    .unwrap_or(total_props)
                    .min(total_props);
                if start >= end {
                    continue;
                }
                for p_rid in start..end {
                    let p_row = parsed.properties[(p_rid - 1) as usize];
                    let pname = parsed.get_string(p_row.name).to_string();
                    let (g_rid, s_rid) = prop_accessors.get(&p_rid).copied().unwrap_or((None, None));
                    let getter_id = g_rid
                        .and_then(|r| method_ids.get(r.saturating_sub(1) as usize).copied())
                        .filter(|&id| id < self.methods.len());
                    let setter_id = s_rid
                        .and_then(|r| method_ids.get(r.saturating_sub(1) as usize).copied())
                        .filter(|&id| id < self.methods.len());
                    let getter_ptr = getter_id.map_or(std::ptr::null_mut(), |id| {
                        self.methods[id].mono_method as *mut MonoMethod
                    });
                    let setter_ptr = setter_id.map_or(std::ptr::null_mut(), |id| {
                        self.methods[id].mono_method as *mut MonoMethod
                    });

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
            classes_by_short_name: asm_classes_by_short_name,
            field_rvas,
            user_strings: HashMap::new(),
            type_cache: HashMap::new(),
            field_cache: HashMap::new(),
            call_cache: HashMap::new(),
            newobj_cache: HashMap::new(),
            standalone_sig_cache: HashMap::new(),
            vdispatch_cache: HashMap::new(),
        });

        // New types/methods may satisfy previously unresolved cross-assembly refs.
        self.invalidate_resolution_caches();

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

    #[inline]
    pub fn find_class_in_assembly(&self, asm_id: usize, ns: &str, name: &str) -> Option<usize> {
        let asm = self.assemblies.get(asm_id)?;
        let bucket = asm.classes_by_short_name.get(name)?;
        bucket
            .iter()
            .copied()
            .find(|&cid| self.classes[cid].namespace == ns)
    }

    pub fn find_class_by_full_name_in_assembly(&self, asm_id: usize, full: &str) -> Option<usize> {
        let clean = full.split(',').next().unwrap_or(full).trim();
        let (ns, name) = split_full_type_name(clean);
        self.find_class_in_assembly(asm_id, ns, name)
    }

    #[inline]
    pub fn find_class_by_name_any(&self, ns: &str, name: &str) -> Option<usize> {
        // Search user assemblies first (newest loaded first), then corlib (id 0).
        let bucket = self.classes_by_short_name.get(name)?;
        bucket
            .iter()
            .rev()
            .copied()
            .find(|&cid| self.classes[cid].namespace == ns)
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
        rid.checked_sub(1)
            .and_then(|i| asm.method_ids.get(i).copied())
    }

    pub fn register_internal_call(&mut self, name: &str, fn_ptr: usize) {
        self.internal_calls.insert(name.to_string(), fn_ptr);
        if let Some((base, _)) = name.split_once('(') {
            self.internal_calls.insert(base.to_string(), fn_ptr);
        }
        self.invalidate_resolution_caches();
    }

    pub fn lookup_internal_call(&self, class_id: usize, method_name: &str) -> Option<usize> {
        if self.internal_calls.is_empty() {
            return None;
        }
        let cls = self.classes.get(class_id)?;
        let full = format!("{}::{method_name}", cls.full_name());
        if let Some(&p) = self.internal_calls.get(&full) {
            return Some(p);
        }
        let short = format!("{}::{method_name}", cls.name);
        self.internal_calls.get(&short).copied()
    }
}

pub(crate) fn split_full_type_name(full: &str) -> (&str, &str) {
    match full.rfind('.') {
        Some(pos) => (&full[..pos], &full[pos + 1..]),
        None => ("", full),
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
            if ns.is_empty() {
                None
            } else {
                Some(ns.to_string())
            },
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
