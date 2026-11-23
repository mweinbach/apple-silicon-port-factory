//! P/Invoke resolution & marshalling, Mono internal-call dispatch, and `calli`
//! native invocation — always releasing `GLOBAL_VM` before executing guest/native code
//! (including `LoadLibrary` / `DllMain`) to prevent re-entrancy deadlocks.

use super::super::metadata::{pinvoke_attr, MethodSig, SigType};
use super::eh::ExecState;
use super::heap::MONO_ARRAY_DATA_OFFSET;
use super::value::{read_sig_from_ptr, write_sig_to_ptr, CilVal, ManagedAddr};
use super::{lock_vm, ClrVm, VmError};
use std::ffi::CString;
use std::sync::MutexGuard;

/// Fast in-process lookup against builtin `win32-rt` tables (never runs guest code).
fn lookup_builtin_pinvoke(dll: &str, entry: &str, mapping_flags: u16) -> Option<usize> {
    if let Some(p) = crate::lookup(dll, entry) {
        return Some(p as usize);
    }
    let no_mangle = (mapping_flags & pinvoke_attr::NO_MANGLE) != 0;
    if !no_mangle {
        let cs = mapping_flags & pinvoke_attr::CHAR_SET_MASK;
        let suffixes = if cs == pinvoke_attr::CHAR_SET_UNICODE || cs == pinvoke_attr::CHAR_SET_AUTO
        {
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
    None
}

/// Resolve a P/Invoke target, caching the resolved address on `MethodInfo`.
/// Releases `GLOBAL_VM` before calling `loader.load_library` so guest `DllMain` can
/// safely call back into CLR/Mono APIs without deadlocking.
fn resolve_pinvoke_for_method(
    mut vm: MutexGuard<'static, ClrVm>,
    method_id: usize,
) -> (MutexGuard<'static, ClrVm>, Result<(usize, bool, MethodSig), VmError>) {
    let (cached, dll, entry, mapping_flags, sig) = {
        let m = &vm.methods[method_id];
        let Some((dll, entry, flags)) = m.pinvoke.as_ref() else {
            let msg = format!("method {} has no P/Invoke metadata", m.name);
            return (vm, Err(VmError::MethodNotFound(msg)));
        };
        (
            m.pinvoke_fn_ptr,
            dll.clone(),
            entry.clone(),
            *flags,
            m.sig.clone(),
        )
    };
    let unicode = (mapping_flags & pinvoke_attr::CHAR_SET_MASK) == pinvoke_attr::CHAR_SET_UNICODE
        || (mapping_flags & pinvoke_attr::CHAR_SET_MASK) == pinvoke_attr::CHAR_SET_AUTO;

    if let Some(fp) = cached {
        return (vm, Ok((fp, unicode, sig)));
    }

    if let Some(fp) = lookup_builtin_pinvoke(&dll, &entry, mapping_flags) {
        vm.methods[method_id].pinvoke_fn_ptr = Some(fp);
        return (vm, Ok((fp, unicode, sig)));
    }

    // Dynamic guest DLL load may execute DllMain (guest x86-64 code): release GLOBAL_VM first!
    if let Some(loader) = crate::try_loader() {
        drop(vm);
        let found = loader
            .load_library(&dll, 0)
            .ok()
            .and_then(|hmod| loader.get_proc_address(hmod, crate::ProcName::Name(&entry)));
        vm = lock_vm();
        if let Some(fp) = found {
            vm.methods[method_id].pinvoke_fn_ptr = Some(fp);
            return (vm, Ok((fp, unicode, sig)));
        }
    }

    (
        vm,
        Err(VmError::MethodNotFound(format!(
            "P/Invoke {dll}!{entry} not found"
        ))),
    )
}

/// Invoke a P/Invoke `MethodInfo`, releasing `GLOBAL_VM` around the native/guest call
/// and re-acquiring it before writing back `byref` arguments and unmarshalling the return.
pub(crate) fn invoke_pinvoke(
    vm: MutexGuard<'static, ClrVm>,
    exec: Option<&mut ExecState>,
    method_id: usize,
    args: &[CilVal],
) -> (MutexGuard<'static, ClrVm>, Result<Option<CilVal>, VmError>) {
    let (vm, resolved) = resolve_pinvoke_for_method(vm, method_id);
    let (fn_ptr, unicode, sig) = match resolved {
        Ok(v) => v,
        Err(e) => return (vm, Err(e)),
    };

    let mut keep_ansi: Vec<CString> = Vec::new();
    let mut keep_wide: Vec<Vec<u16>> = Vec::new();
    let mut byref_slots: Vec<(ManagedAddr, SigType, Box<u64>)> = Vec::new();
    let mut raw_args = [0u64; 8];
    for (i, &val) in args.iter().enumerate().take(8) {
        let pty = sig.params.get(i).unwrap_or(&SigType::IntPtr);
        raw_args[i] = marshal_pinvoke_arg(
            &vm,
            exec.as_deref(),
            pty,
            val,
            unicode,
            &mut keep_ansi,
            &mut keep_wide,
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
    let mut vm = lock_vm();

    if !byref_slots.is_empty() {
        if let Some(ex) = exec {
            for (addr, inner_sig, slot) in byref_slots {
                let v = read_sig_from_ptr(&inner_sig, (&*slot as *const u64) as *const u8);
                ex.write_managed_addr(&mut vm, addr, &inner_sig, v);
            }
        } else {
            for (addr, inner_sig, slot) in byref_slots {
                let v = read_sig_from_ptr(&inner_sig, (&*slot as *const u64) as *const u8);
                match addr {
                    ManagedAddr::StaticField(fid) => {
                        if let Some(f) = vm.fields.get_mut(fid as usize) {
                            f.static_value = v;
                        }
                    }
                    ManagedAddr::Raw(p) if p != 0 => {
                        write_sig_to_ptr(&inner_sig, p as *mut u8, v);
                    }
                    _ => {}
                }
            }
        }
    }

    let ret = if sig.ret == SigType::Void {
        None
    } else {
        Some(unmarshal_native_return(&mut vm, &sig.ret, raw_ret, unicode))
    };
    (vm, Ok(ret))
}

/// Invoke a Mono internal call (`mono_add_internal_call`), releasing `GLOBAL_VM` around the call.
pub(crate) fn invoke_icall(
    vm: MutexGuard<'static, ClrVm>,
    fn_ptr: usize,
    ret_sig: &SigType,
    args: &[CilVal],
) -> (MutexGuard<'static, ClrVm>, Option<CilVal>) {
    let mut raw_args = [0u64; 8];
    for (i, &val) in args.iter().enumerate().take(8) {
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
    let vm = lock_vm();
    let ret = if *ret_sig == SigType::Void {
        None
    } else {
        Some(unmarshal_icall_return(ret_sig, raw_ret))
    };
    (vm, ret)
}

/// Invoke an indirect native function pointer (`calli`), releasing `GLOBAL_VM` around the call.
pub(crate) fn invoke_calli(
    vm: MutexGuard<'static, ClrVm>,
    fn_ptr: usize,
    ret_sig: &SigType,
    raw_args: [u64; 8],
) -> (MutexGuard<'static, ClrVm>, Option<CilVal>) {
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
    let vm = lock_vm();
    let ret = if *ret_sig == SigType::Void {
        None
    } else {
        Some(unmarshal_icall_return(ret_sig, raw_ret))
    };
    (vm, ret)
}

fn marshal_pinvoke_arg(
    vm: &ClrVm,
    exec: Option<&ExecState>,
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
            if obj == 0 {
                0
            } else {
                (obj + MONO_ARRAY_DATA_OFFSET) as u64
            }
        }
        SigType::ByRef(inner) => match val.as_managed_addr() {
            Some(ManagedAddr::Raw(p)) => p as u64,
            Some(addr) => {
                let cur = match exec {
                    Some(ex) => ex.read_ptr_val(vm, val, inner),
                    None => match addr {
                        ManagedAddr::StaticField(fid) => vm
                            .fields
                            .get(fid as usize)
                            .map_or_else(|| super::value::default_val_for_sig(inner), |f| f.static_value),
                        _ => super::value::default_val_for_sig(inner),
                    },
                };
                let mut slot = Box::new(0u64);
                write_sig_to_ptr(inner, (&mut *slot as *mut u64) as *mut u8, cur);
                let ptr = (&mut *slot as *mut u64) as u64;
                byref_slots.push((addr, (**inner).clone(), slot));
                ptr
            }
            None => val.as_u64_bits(),
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
        SigType::I1
        | SigType::U1
        | SigType::I2
        | SigType::U2
        | SigType::Char
        | SigType::I4
        | SigType::U4 => CilVal::I32(raw as i32),
        SigType::I8 | SigType::U8 => CilVal::I64(raw as i64),
        SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
            CilVal::NativeInt(raw as isize)
        }
        SigType::R4 => CilVal::F64(f32::from_bits(raw as u32) as f64),
        SigType::R8 => CilVal::F64(f64::from_bits(raw)),
        _ => CilVal::Obj(raw as usize),
    }
}
