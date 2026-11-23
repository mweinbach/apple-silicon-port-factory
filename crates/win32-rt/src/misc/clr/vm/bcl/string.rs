//! `System.String` and `System.Text.StringBuilder` BCL intrinsics with zero-allocation
//! fast paths for `get_Length`, `get_Chars`, `IsNullOrEmpty`, `Equals`/`op_Equality`, and hashing.

use super::super::heap::{
    HeapObjKind, MONO_ARRAY_DATA_OFFSET, MONO_STRING_CHARS_OFFSET, MONO_STRING_LEN_OFFSET,
};
use super::super::resolve::BclIntrinsic;
use super::super::value::CilVal;
use super::super::{ClrVm, VmError};

pub(super) fn dispatch_string(
    vm: &mut ClrVm,
    intr: BclIntrinsic,
    args: &[CilVal],
) -> Result<Option<CilVal>, VmError> {
    match intr {
        BclIntrinsic::StringGetLength => {
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
        BclIntrinsic::StringGetChars => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32());
            let Some(len) = vm.heap.string_len(obj) else {
                return Err(VmError::ManagedException {
                    exc_obj: vm.make_exception(vm.core.null_ref, "NullReferenceException"),
                    message: "NullReferenceException".into(),
                });
            };
            if idx < 0 || (idx as usize) >= len {
                return Err(VmError::ManagedException {
                    exc_obj: vm
                        .make_exception(vm.core.index_out_of_range, "IndexOutOfRangeException"),
                    message: "IndexOutOfRangeException".into(),
                });
            }
            let ch = unsafe {
                std::ptr::read_unaligned(
                    (obj + MONO_STRING_CHARS_OFFSET + (idx as usize) * 2) as *const u16,
                )
            };
            Ok(Some(CilVal::I32(ch as i32)))
        }
        BclIntrinsic::StringConcatArray => {
            let mut out = String::new();
            let arr = args.first().map_or(0, |v| v.as_obj());
            if let Some((length, _)) = vm.heap.array_info(arr) {
                for i in 0..length {
                    let elem = unsafe {
                        std::ptr::read_unaligned(
                            (arr + MONO_ARRAY_DATA_OFFSET + i * 8) as *const usize,
                        )
                    };
                    out.push_str(&vm.format_val_to_string(CilVal::Obj(elem)));
                }
            }
            Ok(Some(CilVal::Obj(vm.alloc_string(&out))))
        }
        BclIntrinsic::StringConcat => {
            let mut out = String::new();
            for &a in args {
                out.push_str(&vm.format_val_to_string(a));
            }
            Ok(Some(CilVal::Obj(vm.alloc_string(&out))))
        }
        BclIntrinsic::StringOpEquality | BclIntrinsic::StringEquals => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let b = args.get(1).map_or(0, |v| v.as_obj());
            let eq = if a == b {
                true
            } else if a == 0 || b == 0 {
                false
            } else {
                vm.heap.strings_equal(a, b)
            };
            Ok(Some(CilVal::I32(i32::from(eq))))
        }
        BclIntrinsic::StringOpInequality => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let b = args.get(1).map_or(0, |v| v.as_obj());
            let neq = if a == b {
                false
            } else if a == 0 || b == 0 {
                true
            } else {
                !vm.heap.strings_equal(a, b)
            };
            Ok(Some(CilVal::I32(i32::from(neq))))
        }
        BclIntrinsic::StringCompare => {
            let a = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let b = vm
                .heap
                .read_string(args.get(1).map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let c = match a.cmp(&b) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            Ok(Some(CilVal::I32(c)))
        }
        BclIntrinsic::StringIsNullOrEmpty => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let empty = a == 0 || vm.heap.string_len(a).is_none_or(|n| n == 0);
            Ok(Some(CilVal::I32(i32::from(empty))))
        }
        BclIntrinsic::StringIsNullOrWhiteSpace => {
            let a = args.first().map_or(0, |v| v.as_obj());
            let empty = a == 0
                || vm
                    .heap
                    .read_string(a)
                    .is_none_or(|s| s.trim().is_empty());
            Ok(Some(CilVal::I32(i32::from(empty))))
        }
        BclIntrinsic::StringSubstring => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let u16s = vm.heap.read_string_utf16(obj).unwrap_or_default();
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
        BclIntrinsic::StringContains => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let needle = vm
                .heap
                .read_string(args.get(1).map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::I32(i32::from(s.contains(&needle)))))
        }
        BclIntrinsic::StringStartsWith => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let needle = vm
                .heap
                .read_string(args.get(1).map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::I32(i32::from(s.starts_with(&needle)))))
        }
        BclIntrinsic::StringEndsWith => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let needle = vm
                .heap
                .read_string(args.get(1).map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::I32(i32::from(s.ends_with(&needle)))))
        }
        BclIntrinsic::StringIndexOfChar => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let target = args.get(1).map_or(0, |v| v.as_i32()) as u16;
            let start = args.get(2).map_or(0, |v| v.as_i32()).max(0) as usize;
            let mut found = -1i32;
            if let Some(len) = vm.heap.string_len(obj) {
                for i in start..len {
                    if vm.heap.string_char_at(obj, i) == Some(target) {
                        found = i as i32;
                        break;
                    }
                }
            }
            Ok(Some(CilVal::I32(found)))
        }
        BclIntrinsic::StringIndexOfStr => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let needle = vm
                .heap
                .read_string(args.get(1).map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let idx = s
                .find(&needle)
                .map_or(-1, |b| s[..b].encode_utf16().count() as i32);
            Ok(Some(CilVal::I32(idx)))
        }
        BclIntrinsic::StringToUpper => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s.to_uppercase()))))
        }
        BclIntrinsic::StringToLower => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::Obj(vm.alloc_string(&s.to_lowercase()))))
        }
        BclIntrinsic::StringTrim => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            Ok(Some(CilVal::Obj(vm.alloc_string(s.trim()))))
        }
        BclIntrinsic::StringReplaceChar => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let old_c =
                char::from_u32(args.get(1).map_or(0, |v| v.as_i32()) as u32).unwrap_or('\0');
            let new_c =
                char::from_u32(args.get(2).map_or(0, |v| v.as_i32()) as u32).unwrap_or('\0');
            let mut buf = [0u8; 4];
            let new_s = new_c.encode_utf8(&mut buf);
            let out = s.replace(old_c, new_s);
            Ok(Some(CilVal::Obj(vm.alloc_string(&out))))
        }
        BclIntrinsic::StringReplaceStr => {
            let s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let old_s = vm
                .heap
                .read_string(args.get(1).map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let new_s = vm
                .heap
                .read_string(args.get(2).map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let out = s.replace(&old_s, &new_s);
            Ok(Some(CilVal::Obj(vm.alloc_string(&out))))
        }
        BclIntrinsic::StringToCharArray => {
            let u16s = vm
                .heap
                .read_string_utf16(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            let arr = vm.alloc_array_for_class(vm.core.char_cls, u16s.len());
            if !u16s.is_empty() {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        u16s.as_ptr() as *const u8,
                        (arr + MONO_ARRAY_DATA_OFFSET) as *mut u8,
                        u16s.len() * 2,
                    );
                }
            }
            Ok(Some(CilVal::Obj(arr)))
        }
        BclIntrinsic::StringFormat => {
            let mut fmt_s = vm
                .heap
                .read_string(args.first().map_or(0, |v| v.as_obj()))
                .unwrap_or_default();
            for (i, &a) in args.iter().skip(1).enumerate() {
                let rep = vm.format_val_to_string(a);
                fmt_s = fmt_s.replace(&format!("{{{i}}}"), &rep);
            }
            Ok(Some(CilVal::Obj(vm.alloc_string(&fmt_s))))
        }
        BclIntrinsic::StringToString => {
            Ok(Some(args.first().copied().unwrap_or(CilVal::Obj(0))))
        }
        BclIntrinsic::StringGetHashCode => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let h = vm.heap.string_hash_u64(obj).unwrap_or(5381) as i32;
            Ok(Some(CilVal::I32(h)))
        }
        BclIntrinsic::SbAppendChar => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            let ch = args
                .get(1)
                .and_then(|v| char::from_u32(v.as_i32() as u32));
            if let Some(ch) = ch {
                if let Some(HeapObjKind::StringBuilder { buf }) = vm.heap.get_managed_mut(sb) {
                    buf.push(ch);
                }
            }
            Ok(Some(CilVal::Obj(sb)))
        }
        BclIntrinsic::SbAppendVal => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            let piece = args
                .get(1)
                .map_or_else(String::new, |&v| vm.format_val_to_string(v));
            if let Some(HeapObjKind::StringBuilder { buf }) = vm.heap.get_managed_mut(sb) {
                buf.push_str(&piece);
            }
            Ok(Some(CilVal::Obj(sb)))
        }
        BclIntrinsic::SbToString => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            let s = match vm.heap.get_managed(sb) {
                Some(HeapObjKind::StringBuilder { buf }) => buf.clone(),
                _ => String::new(),
            };
            Ok(Some(CilVal::Obj(vm.alloc_string(&s))))
        }
        BclIntrinsic::SbGetLength => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            let l = match vm.heap.get_managed(sb) {
                Some(HeapObjKind::StringBuilder { buf }) => buf.encode_utf16().count() as i32,
                _ => 0,
            };
            Ok(Some(CilVal::I32(l)))
        }
        BclIntrinsic::SbClear => {
            let sb = args.first().map_or(0, |v| v.as_obj());
            if let Some(HeapObjKind::StringBuilder { buf }) = vm.heap.get_managed_mut(sb) {
                buf.clear();
            }
            Ok(Some(CilVal::Obj(sb)))
        }
        _ => unreachable!(),
    }
}
