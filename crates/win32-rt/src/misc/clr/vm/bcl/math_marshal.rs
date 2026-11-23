//! `System.Math`, `System.MathF`, and `System.Runtime.InteropServices.Marshal` BCL intrinsics.

use super::super::super::{alloc_delegate_trampoline, DelegateBinding, DelegateCallMode};
use super::super::heap::{HeapObjKind, MONO_ARRAY_DATA_OFFSET};
use super::super::resolve::BclIntrinsic;
use super::super::value::CilVal;
use super::super::{ClrVm, VmError};
use std::ffi::c_void;

pub(super) fn dispatch_math(intr: BclIntrinsic, args: &[CilVal]) -> CilVal {
    let a = args.first().copied().unwrap_or(CilVal::I32(0));
    let b = args.get(1).copied().unwrap_or(CilVal::I32(0));
    let c = args.get(2).copied().unwrap_or(CilVal::I32(0));

    match intr {
        BclIntrinsic::MathAbsF64 => CilVal::F64(a.as_f64().abs()),
        BclIntrinsic::MathAbsI64 => CilVal::I64(a.as_i64().wrapping_abs()),
        BclIntrinsic::MathAbsI32 => CilVal::I32(a.as_i32().wrapping_abs()),
        BclIntrinsic::MathMinF64 => CilVal::F64(a.as_f64().min(b.as_f64())),
        BclIntrinsic::MathMinI64 => CilVal::I64(a.as_i64().min(b.as_i64())),
        BclIntrinsic::MathMinI32 => CilVal::I32(a.as_i32().min(b.as_i32())),
        BclIntrinsic::MathMaxF64 => CilVal::F64(a.as_f64().max(b.as_f64())),
        BclIntrinsic::MathMaxI64 => CilVal::I64(a.as_i64().max(b.as_i64())),
        BclIntrinsic::MathMaxI32 => CilVal::I32(a.as_i32().max(b.as_i32())),
        BclIntrinsic::MathClampF64 => CilVal::F64(a.as_f64().clamp(b.as_f64(), c.as_f64())),
        BclIntrinsic::MathClampI64 => CilVal::I64(a.as_i64().clamp(b.as_i64(), c.as_i64())),
        BclIntrinsic::MathClampI32 => CilVal::I32(a.as_i32().clamp(b.as_i32(), c.as_i32())),
        BclIntrinsic::MathSignF64 => {
            let x = a.as_f64();
            let s = if x > 0.0 {
                1
            } else if x < 0.0 {
                -1
            } else {
                0
            };
            CilVal::I32(s)
        }
        BclIntrinsic::MathSignInt => {
            let x = a.as_i64();
            let s = if x > 0 {
                1
            } else if x < 0 {
                -1
            } else {
                0
            };
            CilVal::I32(s)
        }
        BclIntrinsic::MathSqrt => CilVal::F64(a.as_f64().sqrt()),
        BclIntrinsic::MathPow => CilVal::F64(a.as_f64().powf(b.as_f64())),
        BclIntrinsic::MathExp => CilVal::F64(a.as_f64().exp()),
        BclIntrinsic::MathLog => {
            if args.len() >= 2 {
                CilVal::F64(a.as_f64().log(b.as_f64()))
            } else {
                CilVal::F64(a.as_f64().ln())
            }
        }
        BclIntrinsic::MathLog10 => CilVal::F64(a.as_f64().log10()),
        BclIntrinsic::MathLog2 => CilVal::F64(a.as_f64().log2()),
        BclIntrinsic::MathSin => CilVal::F64(a.as_f64().sin()),
        BclIntrinsic::MathCos => CilVal::F64(a.as_f64().cos()),
        BclIntrinsic::MathTan => CilVal::F64(a.as_f64().tan()),
        BclIntrinsic::MathAsin => CilVal::F64(a.as_f64().asin()),
        BclIntrinsic::MathAcos => CilVal::F64(a.as_f64().acos()),
        BclIntrinsic::MathAtan => CilVal::F64(a.as_f64().atan()),
        BclIntrinsic::MathAtan2 => CilVal::F64(a.as_f64().atan2(b.as_f64())),
        BclIntrinsic::MathSinh => CilVal::F64(a.as_f64().sinh()),
        BclIntrinsic::MathCosh => CilVal::F64(a.as_f64().cosh()),
        BclIntrinsic::MathTanh => CilVal::F64(a.as_f64().tanh()),
        BclIntrinsic::MathFloor => CilVal::F64(a.as_f64().floor()),
        BclIntrinsic::MathCeiling => CilVal::F64(a.as_f64().ceil()),
        BclIntrinsic::MathRound => CilVal::F64(a.as_f64().round()),
        BclIntrinsic::MathTruncate => CilVal::F64(a.as_f64().trunc()),
        BclIntrinsic::MathFma => CilVal::F64(a.as_f64().mul_add(b.as_f64(), c.as_f64())),
        BclIntrinsic::MathUnknown => CilVal::F64(0.0),
        _ => unreachable!(),
    }
}

#[inline]
fn ptr_with_ofs(args: &[CilVal], ofs_idx: usize) -> usize {
    let base = args.first().map_or(0, |v| v.as_isize() as usize);
    let ofs = args.get(ofs_idx).map_or(0, |v| v.as_i32() as isize);
    (base as isize).wrapping_add(ofs) as usize
}

pub(super) fn dispatch_marshal(
    vm: &mut ClrVm,
    intr: BclIntrinsic,
    args: &[CilVal],
) -> Result<Option<CilVal>, VmError> {
    match intr {
        BclIntrinsic::MarshalAllocHGlobal => {
            let sz = args.first().map_or(0, |v| v.as_isize()).max(1) as usize;
            let p = unsafe { libc::calloc(1, sz) };
            Ok(Some(CilVal::NativeInt(p as usize as isize)))
        }
        BclIntrinsic::MarshalReAllocHGlobal => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *mut c_void;
            let sz = args.get(1).map_or(0, |v| v.as_isize()).max(1) as usize;
            let np = unsafe { libc::realloc(p, sz) };
            Ok(Some(CilVal::NativeInt(np as usize as isize)))
        }
        BclIntrinsic::MarshalFreeHGlobal => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *mut c_void;
            if !p.is_null() {
                unsafe { libc::free(p) };
            }
            Ok(None)
        }
        BclIntrinsic::MarshalReadByte => {
            let p = if args.len() >= 2 {
                ptr_with_ofs(args, 1)
            } else {
                args.first().map_or(0, |v| v.as_isize() as usize)
            };
            let v = if p != 0 {
                unsafe { std::ptr::read_unaligned(p as *const u8) }
            } else {
                0
            };
            Ok(Some(CilVal::I32(v as i32)))
        }
        BclIntrinsic::MarshalReadInt16 => {
            let p = if args.len() >= 2 {
                ptr_with_ofs(args, 1)
            } else {
                args.first().map_or(0, |v| v.as_isize() as usize)
            };
            let v = if p != 0 {
                unsafe { std::ptr::read_unaligned(p as *const i16) }
            } else {
                0
            };
            Ok(Some(CilVal::I32(v as i32)))
        }
        BclIntrinsic::MarshalReadInt32 => {
            let p = if args.len() >= 2 {
                ptr_with_ofs(args, 1)
            } else {
                args.first().map_or(0, |v| v.as_isize() as usize)
            };
            let v = if p != 0 {
                unsafe { std::ptr::read_unaligned(p as *const i32) }
            } else {
                0
            };
            Ok(Some(CilVal::I32(v)))
        }
        BclIntrinsic::MarshalReadInt64 => {
            let p = if args.len() >= 2 {
                ptr_with_ofs(args, 1)
            } else {
                args.first().map_or(0, |v| v.as_isize() as usize)
            };
            let v = if p != 0 {
                unsafe { std::ptr::read_unaligned(p as *const i64) }
            } else {
                0
            };
            Ok(Some(CilVal::I64(v)))
        }
        BclIntrinsic::MarshalReadIntPtr => {
            let p = if args.len() >= 2 {
                ptr_with_ofs(args, 1)
            } else {
                args.first().map_or(0, |v| v.as_isize() as usize)
            };
            let v = if p != 0 {
                unsafe { std::ptr::read_unaligned(p as *const isize) }
            } else {
                0
            };
            Ok(Some(CilVal::NativeInt(v)))
        }
        BclIntrinsic::MarshalWriteByte => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i32() as u8)
            } else {
                (
                    args.first().map_or(0, |v| v.as_isize() as usize),
                    args.get(1).map_or(0, |v| v.as_i32()) as u8,
                )
            };
            if p != 0 {
                unsafe { std::ptr::write_unaligned(p as *mut u8, val) };
            }
            Ok(None)
        }
        BclIntrinsic::MarshalWriteInt16 => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i32() as i16)
            } else {
                (
                    args.first().map_or(0, |v| v.as_isize() as usize),
                    args.get(1).map_or(0, |v| v.as_i32()) as i16,
                )
            };
            if p != 0 {
                unsafe { std::ptr::write_unaligned(p as *mut i16, val) };
            }
            Ok(None)
        }
        BclIntrinsic::MarshalWriteInt32 => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i32())
            } else {
                (
                    args.first().map_or(0, |v| v.as_isize() as usize),
                    args.get(1).map_or(0, |v| v.as_i32()),
                )
            };
            if p != 0 {
                unsafe { std::ptr::write_unaligned(p as *mut i32, val) };
            }
            Ok(None)
        }
        BclIntrinsic::MarshalWriteInt64 => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_i64())
            } else {
                (
                    args.first().map_or(0, |v| v.as_isize() as usize),
                    args.get(1).map_or(0, |v| v.as_i64()),
                )
            };
            if p != 0 {
                unsafe { std::ptr::write_unaligned(p as *mut i64, val) };
            }
            Ok(None)
        }
        BclIntrinsic::MarshalWriteIntPtr => {
            let (p, val) = if args.len() >= 3 {
                (ptr_with_ofs(args, 1), args[2].as_isize())
            } else {
                (
                    args.first().map_or(0, |v| v.as_isize() as usize),
                    args.get(1).map_or(0, |v| v.as_isize()),
                )
            };
            if p != 0 {
                unsafe { std::ptr::write_unaligned(p as *mut isize, val) };
            }
            Ok(None)
        }
        BclIntrinsic::MarshalCopyFromArray => {
            if args.len() >= 4 {
                let arr = args[0].as_obj();
                let start = args[1].as_i32().max(0) as usize;
                let dst = args[2].as_isize() as usize;
                let cnt = args[3].as_i32().max(0) as usize;
                if let Some((length, elem_size)) = vm.heap.array_info(arr) {
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
            }
            Ok(None)
        }
        BclIntrinsic::MarshalCopyToArray => {
            if args.len() >= 4 {
                let src = args[0].as_isize() as usize;
                let arr = args[1].as_obj();
                let start = args[2].as_i32().max(0) as usize;
                let cnt = args[3].as_i32().max(0) as usize;
                if let Some((length, elem_size)) = vm.heap.array_info(arr) {
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
            Ok(None)
        }
        BclIntrinsic::MarshalStringToAnsi => {
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
        BclIntrinsic::MarshalStringToUni => {
            let u16s = vm
                .heap
                .read_string_utf16(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let p = unsafe { libc::malloc((u16s.len() + 1) * 2) } as *mut u8;
            if !p.is_null() {
                unsafe {
                    if !u16s.is_empty() {
                        std::ptr::copy_nonoverlapping(
                            u16s.as_ptr() as *const u8,
                            p,
                            u16s.len() * 2,
                        );
                    }
                    std::ptr::write_unaligned(p.add(u16s.len() * 2) as *mut u16, 0u16);
                }
            }
            Ok(Some(CilVal::NativeInt(p as usize as isize)))
        }
        BclIntrinsic::MarshalPtrToStringAnsi => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *const u8;
            if p.is_null() {
                Ok(Some(CilVal::Obj(0)))
            } else {
                let s = unsafe { crate::strings::from_ansi(p) };
                Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
            }
        }
        BclIntrinsic::MarshalPtrToStringUni => {
            let p = args.first().map_or(0, |v| v.as_isize() as usize) as *const u16;
            if p.is_null() {
                Ok(Some(CilVal::Obj(0)))
            } else {
                let s = unsafe { crate::strings::from_wide(p) };
                Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
            }
        }
        BclIntrinsic::MarshalGetLastWin32Error => {
            Ok(Some(CilVal::I32(crate::teb::last_error() as i32)))
        }
        BclIntrinsic::MarshalSizeOf => {
            let sz = match args.first().copied() {
                Some(CilVal::Obj(obj)) => match vm.heap.get_managed(obj) {
                    Some(HeapObjKind::TypeObj {
                        represented_class_id,
                    }) => vm.classes[*represented_class_id].elem_size,
                    _ => vm
                        .object_class_id(obj)
                        .map_or(8, |cid| vm.classes[cid].elem_size),
                },
                Some(CilVal::ValueType { class_id, .. }) => {
                    vm.value_type_size(class_id as usize)
                }
                Some(CilVal::I32(_)) => 4,
                _ => 8,
            };
            Ok(Some(CilVal::I32(sz as i32)))
        }
        BclIntrinsic::MarshalGetFnPtrForDelegate => {
            let del_obj = args.first().map_or(0, |v| v.as_obj());
            if let Some(HeapObjKind::Delegate {
                method_id,
                target_obj,
                ..
            }) = vm.heap.get_managed(del_obj).cloned()
            {
                let asm_id = vm.methods[method_id].asm_id;
                let fp = alloc_delegate_trampoline(DelegateBinding {
                    asm_id,
                    method_id,
                    target_obj,
                    mode: DelegateCallMode::DirectAnsi,
                })?;
                Ok(Some(CilVal::NativeInt(fp as isize)))
            } else {
                Ok(Some(CilVal::NativeInt(0)))
            }
        }
        _ => unreachable!(),
    }
}
