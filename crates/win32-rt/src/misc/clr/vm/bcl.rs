//! Essential BCL intrinsics dispatcher (zero string-comparison hot path via `BclIntrinsic`
//! and `BclNewObjIntrinsic` enums cached per assembly token).

mod collections;
mod math_marshal;
mod string;

use super::eh::ExecState;
use super::heap::{HeapObjKind, MONO_ARRAY_DATA_OFFSET};
use super::resolve::{BclIntrinsic, BclNewObjIntrinsic};
use super::value::{CilVal, StorageKind};
use super::{ClrVm, VmError};

pub(crate) fn dispatch_newobj(
    vm: &mut ClrVm,
    intr: BclNewObjIntrinsic,
    args: &[CilVal],
) -> Result<CilVal, VmError> {
    match intr {
        BclNewObjIntrinsic::Object => Ok(CilVal::Obj(vm.alloc_object_for_class(vm.core.object))),
        BclNewObjIntrinsic::StringEmpty => Ok(CilVal::Obj(vm.empty_string())),
        BclNewObjIntrinsic::StringFromCharArray => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            if arr == 0 {
                return Ok(CilVal::Obj(vm.empty_string()));
            }
            let len = vm.heap.array_info(arr).map_or(0, |(l, _)| l);
            let mut u16s = vec![0u16; len];
            if len > 0 {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (arr + MONO_ARRAY_DATA_OFFSET) as *const u8,
                        u16s.as_mut_ptr() as *mut u8,
                        len * 2,
                    );
                }
            }
            Ok(CilVal::Obj(vm.alloc_string_utf16(&u16s)))
        }
        BclNewObjIntrinsic::StringFromCharPtr => {
            let ptr = args.first().map_or(0, |v| v.as_isize() as usize) as *const u16;
            if ptr.is_null() {
                Ok(CilVal::Obj(vm.empty_string()))
            } else {
                let s = unsafe { crate::strings::from_wide(ptr) };
                Ok(CilVal::Obj(vm.alloc_string(&s)))
            }
        }
        BclNewObjIntrinsic::StringFromCharCount => {
            let ch = args.first().map_or(0, |v| v.as_i32()) as u16;
            let cnt = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let u16s = vec![ch; cnt];
            Ok(CilVal::Obj(vm.alloc_string_utf16(&u16s)))
        }
        BclNewObjIntrinsic::StringFromCharArraySlice => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let start = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let cnt = args.get(2).map_or(0, |v| v.as_i32()).max(0) as usize;
            let len = vm.heap.array_info(arr).map_or(0, |(l, _)| l);
            let end = start.saturating_add(cnt).min(len);
            let n = end.saturating_sub(start);
            let mut u16s = vec![0u16; n];
            if n > 0 {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (arr + MONO_ARRAY_DATA_OFFSET + start * 2) as *const u8,
                        u16s.as_mut_ptr() as *mut u8,
                        n * 2,
                    );
                }
            }
            Ok(CilVal::Obj(vm.alloc_string_utf16(&u16s)))
        }
        BclNewObjIntrinsic::IntPtr => {
            Ok(CilVal::NativeInt(args.first().map_or(0, |v| v.as_isize())))
        }
        BclNewObjIntrinsic::StringBuilderEmpty => {
            let vt = vm.classes[vm.core.string_builder].mono_vtable;
            Ok(CilVal::Obj(vm.heap.alloc_string_builder(vt, String::new())))
        }
        BclNewObjIntrinsic::StringBuilderFromString => {
            let init = args
                .first()
                .and_then(|v| vm.heap.read_string(v.as_obj()))
                .unwrap_or_default();
            let vt = vm.classes[vm.core.string_builder].mono_vtable;
            Ok(CilVal::Obj(vm.heap.alloc_string_builder(vt, init)))
        }
        BclNewObjIntrinsic::ListEmpty => {
            let vt = vm.classes[vm.core.list_cls].mono_vtable;
            Ok(CilVal::Obj(vm.heap.alloc_list(vt, 0)))
        }
        BclNewObjIntrinsic::ListWithCapacity => {
            let cap = args.first().map_or(0, |v| v.as_i32()).max(0) as usize;
            let vt = vm.classes[vm.core.list_cls].mono_vtable;
            Ok(CilVal::Obj(vm.heap.alloc_list(vt, cap)))
        }
        BclNewObjIntrinsic::DictionaryEmpty => {
            let vt = vm.classes[vm.core.dict_cls].mono_vtable;
            Ok(CilVal::Obj(vm.heap.alloc_dictionary(vt, 0)))
        }
        BclNewObjIntrinsic::DictionaryWithCapacity => {
            let cap = args.first().map_or(0, |v| v.as_i32()).max(0) as usize;
            let vt = vm.classes[vm.core.dict_cls].mono_vtable;
            Ok(CilVal::Obj(vm.heap.alloc_dictionary(vt, cap)))
        }
        BclNewObjIntrinsic::Exception(cid) => {
            let msg = args
                .first()
                .and_then(|v| vm.heap.read_string(v.as_obj()))
                .unwrap_or_else(|| vm.classes[cid].full_name());
            Ok(CilVal::Obj(vm.make_exception(cid, msg)))
        }
        BclNewObjIntrinsic::GenericObject(cid) => {
            Ok(CilVal::Obj(vm.alloc_object_for_class(cid)))
        }
    }
}

pub(crate) fn dispatch_intrinsic(
    vm: &mut ClrVm,
    exec: &mut ExecState,
    intr: BclIntrinsic,
    args: &[CilVal],
) -> Result<Option<CilVal>, VmError> {
    match intr {
        // ---- System.String & System.Text.StringBuilder ----
        BclIntrinsic::StringGetLength
        | BclIntrinsic::StringGetChars
        | BclIntrinsic::StringConcat
        | BclIntrinsic::StringConcatArray
        | BclIntrinsic::StringOpEquality
        | BclIntrinsic::StringOpInequality
        | BclIntrinsic::StringEquals
        | BclIntrinsic::StringCompare
        | BclIntrinsic::StringIsNullOrEmpty
        | BclIntrinsic::StringIsNullOrWhiteSpace
        | BclIntrinsic::StringSubstring
        | BclIntrinsic::StringContains
        | BclIntrinsic::StringStartsWith
        | BclIntrinsic::StringEndsWith
        | BclIntrinsic::StringIndexOfChar
        | BclIntrinsic::StringIndexOfStr
        | BclIntrinsic::StringToUpper
        | BclIntrinsic::StringToLower
        | BclIntrinsic::StringTrim
        | BclIntrinsic::StringReplaceChar
        | BclIntrinsic::StringReplaceStr
        | BclIntrinsic::StringToCharArray
        | BclIntrinsic::StringFormat
        | BclIntrinsic::StringToString
        | BclIntrinsic::StringGetHashCode
        | BclIntrinsic::SbAppendChar
        | BclIntrinsic::SbAppendVal
        | BclIntrinsic::SbToString
        | BclIntrinsic::SbGetLength
        | BclIntrinsic::SbClear => string::dispatch_string(vm, intr, args),

        // ---- System.Array, System.Buffer, List<T>, Dictionary<K,V> ----
        BclIntrinsic::ArrayGetLength
        | BclIntrinsic::ArrayGetLongLength
        | BclIntrinsic::ArrayGetRank
        | BclIntrinsic::ArrayCopy
        | BclIntrinsic::ArrayClear
        | BclIntrinsic::ArrayEmpty
        | BclIntrinsic::BufferBlockCopy
        | BclIntrinsic::BufferByteLength
        | BclIntrinsic::BufferGetByte
        | BclIntrinsic::BufferSetByte
        | BclIntrinsic::BufferMemoryCopy
        | BclIntrinsic::ListAdd
        | BclIntrinsic::ListGetCount
        | BclIntrinsic::ListGetItem
        | BclIntrinsic::ListSetItem
        | BclIntrinsic::ListClear
        | BclIntrinsic::ListContains
        | BclIntrinsic::ListIndexOf
        | BclIntrinsic::ListRemoveAt
        | BclIntrinsic::ListRemove
        | BclIntrinsic::ListToArray
        | BclIntrinsic::DictAdd
        | BclIntrinsic::DictGetCount
        | BclIntrinsic::DictGetItem
        | BclIntrinsic::DictSetItem
        | BclIntrinsic::DictContainsKey
        | BclIntrinsic::DictTryGetValue
        | BclIntrinsic::DictRemove
        | BclIntrinsic::DictClear => collections::dispatch_collections(vm, exec, intr, args),

        // ---- System.Math / System.MathF ----
        BclIntrinsic::MathAbsF64
        | BclIntrinsic::MathAbsI64
        | BclIntrinsic::MathAbsI32
        | BclIntrinsic::MathMinF64
        | BclIntrinsic::MathMinI64
        | BclIntrinsic::MathMinI32
        | BclIntrinsic::MathMaxF64
        | BclIntrinsic::MathMaxI64
        | BclIntrinsic::MathMaxI32
        | BclIntrinsic::MathClampF64
        | BclIntrinsic::MathClampI64
        | BclIntrinsic::MathClampI32
        | BclIntrinsic::MathSignF64
        | BclIntrinsic::MathSignInt
        | BclIntrinsic::MathSqrt
        | BclIntrinsic::MathPow
        | BclIntrinsic::MathExp
        | BclIntrinsic::MathLog
        | BclIntrinsic::MathLog10
        | BclIntrinsic::MathLog2
        | BclIntrinsic::MathSin
        | BclIntrinsic::MathCos
        | BclIntrinsic::MathTan
        | BclIntrinsic::MathAsin
        | BclIntrinsic::MathAcos
        | BclIntrinsic::MathAtan
        | BclIntrinsic::MathAtan2
        | BclIntrinsic::MathSinh
        | BclIntrinsic::MathCosh
        | BclIntrinsic::MathTanh
        | BclIntrinsic::MathFloor
        | BclIntrinsic::MathCeiling
        | BclIntrinsic::MathRound
        | BclIntrinsic::MathTruncate
        | BclIntrinsic::MathFma
        | BclIntrinsic::MathUnknown => Ok(Some(math_marshal::dispatch_math(intr, args))),

        // ---- System.Runtime.InteropServices.Marshal ----
        BclIntrinsic::MarshalAllocHGlobal
        | BclIntrinsic::MarshalReAllocHGlobal
        | BclIntrinsic::MarshalFreeHGlobal
        | BclIntrinsic::MarshalReadByte
        | BclIntrinsic::MarshalReadInt16
        | BclIntrinsic::MarshalReadInt32
        | BclIntrinsic::MarshalReadInt64
        | BclIntrinsic::MarshalReadIntPtr
        | BclIntrinsic::MarshalWriteByte
        | BclIntrinsic::MarshalWriteInt16
        | BclIntrinsic::MarshalWriteInt32
        | BclIntrinsic::MarshalWriteInt64
        | BclIntrinsic::MarshalWriteIntPtr
        | BclIntrinsic::MarshalCopyFromArray
        | BclIntrinsic::MarshalCopyToArray
        | BclIntrinsic::MarshalStringToAnsi
        | BclIntrinsic::MarshalStringToUni
        | BclIntrinsic::MarshalPtrToStringAnsi
        | BclIntrinsic::MarshalPtrToStringUni
        | BclIntrinsic::MarshalGetLastWin32Error
        | BclIntrinsic::MarshalSizeOf
        | BclIntrinsic::MarshalGetFnPtrForDelegate => {
            math_marshal::dispatch_marshal(vm, intr, args)
        }

        // ---- System.Object ----
        BclIntrinsic::ObjectCtor | BclIntrinsic::BaseCtorNoop | BclIntrinsic::GcNoop => Ok(None),
        BclIntrinsic::ObjectReferenceEquals => {
            let eq =
                args.first().map_or(0, |v| v.as_obj()) == args.get(1).map_or(0, |v| v.as_obj());
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        BclIntrinsic::ObjectEquals => {
            let a = args.first().copied().unwrap_or(CilVal::Obj(0));
            let b = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let eq = match (a, b) {
                (CilVal::Obj(pa), CilVal::Obj(pb))
                    if vm.heap.is_string(pa) && vm.heap.is_string(pb) =>
                {
                    vm.heap.strings_equal(pa, pb)
                }
                _ => a.as_u64_bits() == b.as_u64_bits(),
            };
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        BclIntrinsic::ObjectGetHashCode | BclIntrinsic::RuntimeHelpersGetHashCode => {
            let v = args.first().map_or(0, |v| v.as_u64_bits());
            Ok(Some(CilVal::I32(((v >> 32) ^ v) as i32)))
        }
        BclIntrinsic::ObjectToString | BclIntrinsic::ConvertToString => {
            let s = vm.format_val_to_string(args.first().copied().unwrap_or(CilVal::Obj(0)));
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::ObjectGetType => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let cid = vm.object_class_id(obj).unwrap_or(vm.core.object);
            let vt = vm.classes[vm.core.type_cls].mono_vtable;
            Ok(Some(CilVal::Obj(vm.heap.alloc_type_obj(vt, cid))))
        }

        // ---- System.Type ----
        BclIntrinsic::TypeGetTypeFromHandle => {
            let packed = args.first().map_or(0, |v| v.as_u64_bits());
            let aid = (packed >> 32) as usize;
            let tok = packed as u32;
            let cid = vm
                .resolve_type_token_cached(aid, tok)
                .unwrap_or(vm.core.object);
            let vt = vm.classes[vm.core.type_cls].mono_vtable;
            Ok(Some(CilVal::Obj(vm.heap.alloc_type_obj(vt, cid))))
        }
        BclIntrinsic::TypeGetName => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let name = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::TypeObj {
                    represented_class_id,
                }) => vm.classes[*represented_class_id].name.clone(),
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&name))))
        }
        BclIntrinsic::TypeGetFullName => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let name = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::TypeObj {
                    represented_class_id,
                }) => vm.classes[*represented_class_id].full_name(),
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&name))))
        }
        BclIntrinsic::TypeGetNamespace => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let name = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::TypeObj {
                    represented_class_id,
                }) => vm.classes[*represented_class_id].namespace.clone(),
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&name))))
        }

        // ---- System.Environment ----
        BclIntrinsic::EnvGetTickCount => {
            let ms = unsafe { crate::kernel32::time::GetTickCount() } as i32;
            Ok(Some(CilVal::I32(ms)))
        }
        BclIntrinsic::EnvGetTickCount64 => {
            let ms = unsafe { crate::kernel32::time::GetTickCount64() } as i64;
            Ok(Some(CilVal::I64(ms)))
        }
        BclIntrinsic::EnvGetProcessorCount => {
            let n = std::thread::available_parallelism().map_or(4, |p| p.get()) as i32;
            Ok(Some(CilVal::I32(n)))
        }
        BclIntrinsic::EnvGetIs64Bit => Ok(Some(CilVal::I32(1))),
        BclIntrinsic::EnvGetNewLine => Ok(Some(CilVal::Obj(vm.alloc_string("\r\n")))),
        BclIntrinsic::EnvGetCurrentDirectory => {
            let cwd = crate::path::cwd();
            Ok(Some(CilVal::Obj(vm.alloc_string(&cwd))))
        }
        BclIntrinsic::EnvGetMachineName => Ok(Some(CilVal::Obj(vm.alloc_string("APPLE-SILICON")))),
        BclIntrinsic::EnvGetUserName => {
            let u = std::env::var("USER").unwrap_or_else(|_| "player".into());
            Ok(Some(CilVal::Obj(vm.alloc_string(&u))))
        }
        BclIntrinsic::EnvGetEnvVar => {
            let key = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            match std::env::var(&key) {
                Ok(val) => Ok(Some(CilVal::Obj(vm.alloc_string(&val)))),
                Err(_) => Ok(Some(CilVal::Obj(0))),
            }
        }
        BclIntrinsic::EnvSetEnvVar => {
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
        BclIntrinsic::EnvGetCommandLineArgs => {
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
        BclIntrinsic::EnvGetExitCode => Ok(Some(CilVal::I32(vm.env_exit_code))),
        BclIntrinsic::EnvSetExitCode => {
            vm.env_exit_code = args.first().map_or(0, |v| v.as_i32());
            Ok(None)
        }
        BclIntrinsic::EnvExit => {
            let code = args.first().map_or(0, |v| v.as_i32());
            vm.env_exit_code = code;
            std::process::exit(code);
        }

        // ---- System.Runtime.CompilerServices.RuntimeHelpers ----
        BclIntrinsic::RuntimeHelpersInitArray => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let packed = args.get(1).map_or(0, |v| v.as_u64_bits());
            let aid = (packed >> 32) as usize;
            let frid = (packed as u32) & 0x00FF_FFFF;
            if let Some((length, elem_size)) = vm.heap.array_info(arr) {
                let byte_len = elem_size * length;
                if let Some(asm) = vm.assemblies.get(aid) {
                    if let (Some(&rva), Some(img)) = (asm.field_rvas.get(&frid), asm.image.as_ref())
                    {
                        if let Some(src) = img.bytes_at_rva(rva, byte_len) {
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

        // ---- System.Console ----
        BclIntrinsic::ConsoleWrite => {
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

        // ---- System.Exception ----
        BclIntrinsic::ExceptionGetMessage => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let msg = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::Exception { message, .. }) => message.clone(),
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&msg))))
        }

        // ---- System.BitConverter ----
        BclIntrinsic::BitConvSingleToI32 => Ok(Some(CilVal::I32(
            (args.first().map_or(0.0, |v| v.as_f64()) as f32).to_bits() as i32,
        ))),
        BclIntrinsic::BitConvI32ToSingle => Ok(Some(CilVal::F64(f32::from_bits(
            args.first().map_or(0, |v| v.as_i32()) as u32,
        ) as f64))),
        BclIntrinsic::BitConvDoubleToI64 => Ok(Some(CilVal::I64(
            args.first().map_or(0.0, |v| v.as_f64()).to_bits() as i64,
        ))),
        BclIntrinsic::BitConvI64ToDouble => Ok(Some(CilVal::F64(f64::from_bits(
            args.first().map_or(0, |v| v.as_i64()) as u64,
        )))),

        // ---- System.IntPtr / UIntPtr ----
        BclIntrinsic::IntPtrToI32 => {
            let v = exec.read_ptr_kind(
                vm,
                args.first().copied().unwrap_or(CilVal::NativeInt(0)),
                StorageKind::NativeInt,
            );
            Ok(Some(CilVal::I32(v.as_i32())))
        }
        BclIntrinsic::IntPtrToI64 => {
            let v = exec.read_ptr_kind(
                vm,
                args.first().copied().unwrap_or(CilVal::NativeInt(0)),
                StorageKind::NativeInt,
            );
            Ok(Some(CilVal::I64(v.as_i64())))
        }
        BclIntrinsic::IntPtrToPtr => {
            let v = exec.read_ptr_kind(
                vm,
                args.first().copied().unwrap_or(CilVal::NativeInt(0)),
                StorageKind::NativeInt,
            );
            Ok(Some(CilVal::NativeInt(v.as_isize())))
        }
        BclIntrinsic::IntPtrExplicitToI32 => Ok(Some(CilVal::I32(
            args.first().map_or(0, |v| v.as_i32()),
        ))),
        BclIntrinsic::IntPtrExplicitToI64 => Ok(Some(CilVal::I64(
            args.first().map_or(0, |v| v.as_i64()),
        ))),
        BclIntrinsic::IntPtrExplicitToNative => Ok(Some(CilVal::NativeInt(
            args.first().map_or(0, |v| v.as_isize()),
        ))),
        BclIntrinsic::IntPtrOpEq => {
            let eq =
                args.first().map_or(0, |v| v.as_isize()) == args.get(1).map_or(0, |v| v.as_isize());
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        BclIntrinsic::IntPtrOpNeq => {
            let neq =
                args.first().map_or(0, |v| v.as_isize()) != args.get(1).map_or(0, |v| v.as_isize());
            Ok(Some(CilVal::I32(i32::from(neq))))
        }

        // ---- Primitives & System.Convert ----
        BclIntrinsic::Int32Parse => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::I32(s.trim().parse::<i32>().unwrap_or(0))))
        }
        BclIntrinsic::Int64Parse => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::I64(s.trim().parse::<i64>().unwrap_or(0))))
        }
        BclIntrinsic::PrimToStringI32 => {
            let p = args.first().copied().unwrap_or(CilVal::I32(0));
            let s = exec.read_ptr_kind(vm, p, StorageKind::I4).as_i32().to_string();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::PrimToStringU32 => {
            let p = args.first().copied().unwrap_or(CilVal::I32(0));
            let s = (exec.read_ptr_kind(vm, p, StorageKind::U4).as_i32() as u32).to_string();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::PrimToStringI64 => {
            let p = args.first().copied().unwrap_or(CilVal::I64(0));
            let s = exec.read_ptr_kind(vm, p, StorageKind::I8).as_i64().to_string();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::PrimToStringU64 => {
            let p = args.first().copied().unwrap_or(CilVal::I64(0));
            let s = (exec.read_ptr_kind(vm, p, StorageKind::U8).as_i64() as u64).to_string();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::PrimToStringF32 => {
            let p = args.first().copied().unwrap_or(CilVal::F64(0.0));
            let s = format!("{}", exec.read_ptr_kind(vm, p, StorageKind::R4).as_f64() as f32);
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::PrimToStringF64 => {
            let p = args.first().copied().unwrap_or(CilVal::F64(0.0));
            let s = format!("{}", exec.read_ptr_kind(vm, p, StorageKind::R8).as_f64());
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::PrimToStringBool => {
            let p = args.first().copied().unwrap_or(CilVal::I32(0));
            let s = if exec.read_ptr_kind(vm, p, StorageKind::Bool).is_truthy() {
                "True"
            } else {
                "False"
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(s))))
        }
        BclIntrinsic::PrimToStringU8 => {
            let p = args.first().copied().unwrap_or(CilVal::I32(0));
            let s = (exec.read_ptr_kind(vm, p, StorageKind::U1).as_i32() as u8).to_string();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::ConvertToI32 => {
            let a = args.first().copied().unwrap_or(CilVal::I32(0));
            let v = match a {
                CilVal::Obj(p) => vm
                    .heap
                    .read_string(p)
                    .map_or_else(|| a.as_i32(), |s| s.trim().parse::<i32>().unwrap_or(0)),
                _ => a.as_i32(),
            };
            Ok(Some(CilVal::I32(v)))
        }
        BclIntrinsic::ConvertToI64 => {
            let a = args.first().copied().unwrap_or(CilVal::I64(0));
            let v = match a {
                CilVal::Obj(p) => vm
                    .heap
                    .read_string(p)
                    .map_or_else(|| a.as_i64(), |s| s.trim().parse::<i64>().unwrap_or(0)),
                _ => a.as_i64(),
            };
            Ok(Some(CilVal::I64(v)))
        }
        BclIntrinsic::ConvertToF64 => {
            let a = args.first().copied().unwrap_or(CilVal::F64(0.0));
            let v = match a {
                CilVal::Obj(p) => vm
                    .heap
                    .read_string(p)
                    .map_or_else(|| a.as_f64(), |s| s.trim().parse::<f64>().unwrap_or(0.0)),
                _ => a.as_f64(),
            };
            Ok(Some(CilVal::F64(v)))
        }
    }
}
