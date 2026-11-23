//! `System.Array`, `System.Buffer`, `System.Collections.Generic.List<T>`, and
//! `System.Collections.Generic.Dictionary<TKey, TValue>` BCL intrinsics.

use super::super::eh::ExecState;
use super::super::heap::{HeapObjKind, MONO_ARRAY_DATA_OFFSET};
use super::super::resolve::BclIntrinsic;
use super::super::value::{CilVal, StorageKind};
use super::super::{ClrVm, VmError};

/// Compute a stable key hash supporting value equality on managed `System.String`.
fn cil_key_hash(vm: &ClrVm, key: CilVal) -> u64 {
    if let CilVal::Obj(p) = key {
        if let Some(h) = vm.heap.string_hash_u64(p) {
            return h ^ 0x517C_C1B7_2722_0A95;
        }
    }
    key.as_u64_bits()
}

/// Compare two `CilVal` values using string content equality for `System.String` and bit equality otherwise.
fn cil_vals_equal(vm: &ClrVm, a: CilVal, b: CilVal) -> bool {
    if let (CilVal::Obj(pa), CilVal::Obj(pb)) = (a, b) {
        if pa == pb {
            return true;
        }
        if vm.heap.is_string(pa) && vm.heap.is_string(pb) {
            return vm.heap.strings_equal(pa, pb);
        }
        return false;
    }
    a.as_u64_bits() == b.as_u64_bits()
}

fn find_dict_entry(vm: &ClrVm, dict_obj: usize, key: CilVal) -> Option<(u64, usize)> {
    let hash = cil_key_hash(vm, key);
    let Some(HeapObjKind::Dictionary { entries, index }) = vm.heap.get_managed(dict_obj) else {
        return None;
    };
    let bucket = index.get(&hash)?;
    for &ei in bucket {
        if let Some((ek, _)) = entries.get(ei) {
            if cil_vals_equal(vm, *ek, key) {
                return Some((hash, ei));
            }
        }
    }
    None
}

pub(super) fn dispatch_collections(
    vm: &mut ClrVm,
    exec: &mut ExecState,
    intr: BclIntrinsic,
    args: &[CilVal],
) -> Result<Option<CilVal>, VmError> {
    match intr {
        // ---- System.Array ----
        BclIntrinsic::ArrayGetLength => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let len = vm.heap.array_info(arr).map_or(0, |(l, _)| l);
            Ok(Some(CilVal::I32(len as i32)))
        }
        BclIntrinsic::ArrayGetLongLength => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let len = vm.heap.array_info(arr).map_or(0, |(l, _)| l);
            Ok(Some(CilVal::I64(len as i64)))
        }
        BclIntrinsic::ArrayGetRank => Ok(Some(CilVal::I32(1))),
        BclIntrinsic::ArrayCopy => {
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
            if let (Some((l1, es1)), Some((l2, es2))) =
                (vm.heap.array_info(src), vm.heap.array_info(dst))
            {
                let es = es1.min(es2);
                let n = cnt.min(l1.saturating_sub(si)).min(l2.saturating_sub(di));
                if n > 0 {
                    unsafe {
                        std::ptr::copy(
                            (src + MONO_ARRAY_DATA_OFFSET + si * es) as *const u8,
                            (dst + MONO_ARRAY_DATA_OFFSET + di * es) as *mut u8,
                            n * es,
                        );
                    }
                }
            }
            Ok(None)
        }
        BclIntrinsic::ArrayClear => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            if let Some((length, elem_size)) = vm.heap.array_info(arr) {
                let idx = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
                let cnt = args
                    .get(2)
                    .map_or(length, |v| v.as_i32().max(0) as usize)
                    .min(length.saturating_sub(idx));
                if cnt > 0 {
                    unsafe {
                        std::ptr::write_bytes(
                            (arr + MONO_ARRAY_DATA_OFFSET + idx * elem_size) as *mut u8,
                            0,
                            cnt * elem_size,
                        );
                    }
                }
            }
            Ok(None)
        }
        BclIntrinsic::ArrayEmpty => {
            let arr = vm.alloc_array_for_class(vm.core.object, 0);
            Ok(Some(CilVal::Obj(arr)))
        }

        // ---- System.Buffer ----
        BclIntrinsic::BufferBlockCopy => {
            let src = args.first().map_or(0, |v| v.as_obj());
            let soff = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let dst = args.get(2).map_or(0, |v| v.as_obj());
            let doff = args.get(3).map_or(0, |v| v.as_i32()).max(0) as usize;
            let cnt = args.get(4).map_or(0, |v| v.as_i32()).max(0) as usize;
            if let (Some((l1, es1)), Some((l2, es2))) =
                (vm.heap.array_info(src), vm.heap.array_info(dst))
            {
                let max1 = l1 * es1;
                let max2 = l2 * es2;
                let n = cnt
                    .min(max1.saturating_sub(soff))
                    .min(max2.saturating_sub(doff));
                if n > 0 {
                    unsafe {
                        std::ptr::copy(
                            (src + MONO_ARRAY_DATA_OFFSET + soff) as *const u8,
                            (dst + MONO_ARRAY_DATA_OFFSET + doff) as *mut u8,
                            n,
                        );
                    }
                }
            }
            Ok(None)
        }
        BclIntrinsic::BufferByteLength => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let bl = vm
                .heap
                .array_info(arr)
                .map_or(0, |(len, es)| (len * es) as i32);
            Ok(Some(CilVal::I32(bl)))
        }
        BclIntrinsic::BufferGetByte => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let b = unsafe {
                std::ptr::read_unaligned((arr + MONO_ARRAY_DATA_OFFSET + idx) as *const u8)
            };
            Ok(Some(CilVal::I32(b as i32)))
        }
        BclIntrinsic::BufferSetByte => {
            let arr = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32()).max(0) as usize;
            let b = args.get(2).map_or(0, |v| v.as_i32()) as u8;
            unsafe {
                std::ptr::write_unaligned((arr + MONO_ARRAY_DATA_OFFSET + idx) as *mut u8, b);
            }
            Ok(None)
        }
        BclIntrinsic::BufferMemoryCopy => {
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

        // ---- System.Collections.Generic.List<T> ----
        BclIntrinsic::ListAdd => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let val = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let val = exec.maybe_promote_static_val(vm, val);
            if let Some(HeapObjKind::List { items }) = vm.heap.get_managed_mut(obj) {
                items.push(val);
            }
            Ok(None)
        }
        BclIntrinsic::ListGetCount => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let n = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::List { items }) => items.len() as i32,
                _ => 0,
            };
            Ok(Some(CilVal::I32(n)))
        }
        BclIntrinsic::ListGetItem => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32());
            let item = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::List { items }) if idx >= 0 && (idx as usize) < items.len() => {
                    items[idx as usize]
                }
                _ => {
                    return Err(VmError::ManagedException {
                        exc_obj: vm.make_exception(
                            vm.core.index_out_of_range,
                            "ArgumentOutOfRangeException in List.get_Item",
                        ),
                        message: "ArgumentOutOfRangeException in List.get_Item".into(),
                    });
                }
            };
            Ok(Some(item))
        }
        BclIntrinsic::ListSetItem => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32());
            let val = args.get(2).copied().unwrap_or(CilVal::Obj(0));
            let val = exec.maybe_promote_static_val(vm, val);
            match vm.heap.get_managed_mut(obj) {
                Some(HeapObjKind::List { items }) if idx >= 0 && (idx as usize) < items.len() => {
                    items[idx as usize] = val;
                    Ok(None)
                }
                _ => Err(VmError::ManagedException {
                    exc_obj: vm.make_exception(
                        vm.core.index_out_of_range,
                        "ArgumentOutOfRangeException in List.set_Item",
                    ),
                    message: "ArgumentOutOfRangeException in List.set_Item".into(),
                }),
            }
        }
        BclIntrinsic::ListClear => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            if let Some(HeapObjKind::List { items }) = vm.heap.get_managed_mut(obj) {
                items.clear();
            }
            Ok(None)
        }
        BclIntrinsic::ListContains | BclIntrinsic::ListIndexOf => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let needle = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let mut found: i32 = -1;
            if let Some(HeapObjKind::List { items }) = vm.heap.get_managed(obj) {
                for (i, &it) in items.iter().enumerate() {
                    if cil_vals_equal(vm, it, needle) {
                        found = i as i32;
                        break;
                    }
                }
            }
            if intr == BclIntrinsic::ListContains {
                Ok(Some(CilVal::I32(i32::from(found >= 0))))
            } else {
                Ok(Some(CilVal::I32(found)))
            }
        }
        BclIntrinsic::ListRemoveAt => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let idx = args.get(1).map_or(0, |v| v.as_i32());
            if let Some(HeapObjKind::List { items }) = vm.heap.get_managed_mut(obj) {
                if idx >= 0 && (idx as usize) < items.len() {
                    items.remove(idx as usize);
                }
            }
            Ok(None)
        }
        BclIntrinsic::ListRemove => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let needle = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let pos = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::List { items }) => items
                    .iter()
                    .position(|&it| cil_vals_equal(vm, it, needle)),
                _ => None,
            };
            if let Some(i) = pos {
                if let Some(HeapObjKind::List { items }) = vm.heap.get_managed_mut(obj) {
                    items.remove(i);
                }
                Ok(Some(CilVal::I32(1)))
            } else {
                Ok(Some(CilVal::I32(0)))
            }
        }
        BclIntrinsic::ListToArray => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let snapshot: Vec<CilVal> = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::List { items }) => items.clone(),
                _ => Vec::new(),
            };
            let arr = vm.alloc_array_for_class(vm.core.object, snapshot.len());
            for (i, v) in snapshot.iter().enumerate() {
                unsafe {
                    std::ptr::write_unaligned(
                        (arr + MONO_ARRAY_DATA_OFFSET + i * 8) as *mut u64,
                        v.as_u64_bits(),
                    );
                }
            }
            Ok(Some(CilVal::Obj(arr)))
        }

        // ---- System.Collections.Generic.Dictionary<K, V> ----
        BclIntrinsic::DictAdd | BclIntrinsic::DictSetItem => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let key = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let val = args.get(2).copied().unwrap_or(CilVal::Obj(0));
            let key = exec.maybe_promote_static_val(vm, key);
            let val = exec.maybe_promote_static_val(vm, val);
            let existing = find_dict_entry(vm, obj, key);
            let hash = cil_key_hash(vm, key);
            if let Some(HeapObjKind::Dictionary { entries, index }) = vm.heap.get_managed_mut(obj)
            {
                if let Some((_, ei)) = existing {
                    entries[ei].1 = val;
                } else {
                    let ei = entries.len();
                    entries.push((key, val));
                    index.entry(hash).or_default().push(ei);
                }
            }
            Ok(None)
        }
        BclIntrinsic::DictGetCount => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let n = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::Dictionary { entries, .. }) => entries.len() as i32,
                _ => 0,
            };
            Ok(Some(CilVal::I32(n)))
        }
        BclIntrinsic::DictGetItem => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let key = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            if let Some((_, ei)) = find_dict_entry(vm, obj, key) {
                if let Some(HeapObjKind::Dictionary { entries, .. }) = vm.heap.get_managed(obj) {
                    return Ok(Some(entries[ei].1));
                }
            }
            Err(VmError::ManagedException {
                exc_obj: vm.make_exception(vm.core.exception, "KeyNotFoundException"),
                message: "KeyNotFoundException".into(),
            })
        }
        BclIntrinsic::DictContainsKey => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let key = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let found = find_dict_entry(vm, obj, key).is_some();
            Ok(Some(CilVal::I32(i32::from(found))))
        }
        BclIntrinsic::DictTryGetValue => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let key = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let out_ptr = args.get(2).copied().unwrap_or(CilVal::RawPtr(0));
            if let Some((_, ei)) = find_dict_entry(vm, obj, key) {
                let val = match vm.heap.get_managed(obj) {
                    Some(HeapObjKind::Dictionary { entries, .. }) => entries[ei].1,
                    _ => CilVal::Obj(0),
                };
                exec.write_ptr_kind(vm, out_ptr, StorageKind::Ref, val);
                Ok(Some(CilVal::I32(1)))
            } else {
                exec.write_ptr_kind(vm, out_ptr, StorageKind::Ref, CilVal::I32(0));
                Ok(Some(CilVal::I32(0)))
            }
        }
        BclIntrinsic::DictRemove => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            let key = args.get(1).copied().unwrap_or(CilVal::Obj(0));
            let Some((_, ei)) = find_dict_entry(vm, obj, key) else {
                return Ok(Some(CilVal::I32(0)));
            };
            // Swap-remove and rebuild affected index entries cleanly.
            let swapped_key = match vm.heap.get_managed(obj) {
                Some(HeapObjKind::Dictionary { entries, .. }) => {
                    let last = entries.len() - 1;
                    (last != ei).then(|| entries[last].0)
                }
                _ => None,
            };
            let removed_hash = cil_key_hash(vm, key);
            let swapped_hash = swapped_key.map(|k| cil_key_hash(vm, k));
            if let Some(HeapObjKind::Dictionary { entries, index }) = vm.heap.get_managed_mut(obj)
            {
                let last = entries.len() - 1;
                entries.swap_remove(ei);
                if let Some(b) = index.get_mut(&removed_hash) {
                    b.retain(|&idx| idx != ei);
                }
                if let Some(sh) = swapped_hash {
                    if let Some(b) = index.get_mut(&sh) {
                        for idx in b.iter_mut() {
                            if *idx == last {
                                *idx = ei;
                            }
                        }
                    }
                }
            }
            Ok(Some(CilVal::I32(1)))
        }
        BclIntrinsic::DictClear => {
            let obj = args.first().map_or(0, |v| v.as_obj());
            if let Some(HeapObjKind::Dictionary { entries, index }) = vm.heap.get_managed_mut(obj)
            {
                entries.clear();
                index.clear();
            }
            Ok(None)
        }
        _ => unreachable!(),
    }
}
