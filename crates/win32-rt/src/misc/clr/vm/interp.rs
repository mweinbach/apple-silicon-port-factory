//! Hot-loop ECMA-335 CIL bytecode interpreter.
//!
//! Key performance properties:
//! - Outer `'frame` loop clones `Arc<MethodBody>` once per frame switch and hoists
//!   `code: &[u8]`, `ip`, `asm_id`, `arg_base`, and `local_base` into locals for the
//!   inner `'op` loop (zero per-opcode `Arc::clone`).
//! - Single contiguous `val_stack` across frames (zero `Vec` allocations per managed call)
//!   with frame-scoped `FrameArena` reclamation for value-type locals and `localloc`.
//! - Per-assembly token caches for `call`/`callvirt`/`newobj`/`ldfld`/`stfld`/`ldstr`/`calli`,
//!   `BclIntrinsic` integer dispatch, direct unaligned pointer reads/writes, and O(1)
//!   header-tagged `ManagedHeap` slab lookups for arrays/strings/objects.

use super::super::metadata::table;
use super::bcl;
use super::eh::{dispatch_exception, dispatch_exception_from, find_exited_finally, ExecState};
use super::heap::{HeapObjKind, MONO_ARRAY_DATA_OFFSET, MONO_ARRAY_LEN_OFFSET, MONO_OBJECT_HEADER_SIZE};
use super::pinvoke;
use super::resolve::{ResolvedField, ResolvedNewObj, ResolvedTarget};
use super::value::{
    arith_bin, arith_checked, arith_div, arith_rem, bitwise_bin, conv_checked, eval_branch_cmp,
    eval_cmp_op, field_base_ptr, raw_ptr_from_val, read_kind_from_ptr, shift_bin,
    write_kind_to_ptr, CilVal, ManagedAddr, SmallArgs, StorageKind,
};
use super::{lock_vm, ClrVm, VmError};
use std::sync::MutexGuard;

/// Pop `argc` arguments from `val_stack` into a stack-allocated `SmallArgs` buffer,
/// never popping below `stack_base` even on malformed IL.
#[inline]
fn pop_small_args(val_stack: &mut Vec<CilVal>, stack_base: usize, argc: usize) -> SmallArgs {
    let avail = val_stack.len().saturating_sub(stack_base);
    if avail >= argc {
        SmallArgs::pop_from(val_stack, argc)
    } else {
        // Pad on the left if fewer operands are present than the signature expects.
        let mut padded = Vec::with_capacity(argc);
        padded.resize(argc - avail, CilVal::I32(0));
        padded.extend_from_slice(&val_stack[stack_base..]);
        val_stack.truncate(stack_base);
        let mut tmp = padded;
        SmallArgs::pop_from(&mut tmp, argc)
    }
}

#[inline]
fn array_elem_ptr(vm: &ClrVm, arr: usize, idx: isize) -> Result<(usize, usize), bool> {
    if arr == 0 {
        return Err(true);
    }
    let Some((len, elem_sz)) = vm.heap.array_info(arr) else {
        return Err(true);
    };
    if idx < 0 || (idx as usize) >= len {
        return Err(false);
    }
    Ok((
        arr + MONO_ARRAY_DATA_OFFSET + (idx as usize) * elem_sz,
        elem_sz,
    ))
}

/// Execute a managed method (by global `method_id`) with the given CIL arguments.
/// Releases the VM mutex before any P/Invoke, `LoadLibrary`, or internal-call native callback
/// so re-entrant `mono_*` / CLR calls never deadlock.
pub fn execute_method(method_id: usize, args: Vec<CilVal>) -> Result<Option<CilVal>, VmError> {
    let mut vm: MutexGuard<'static, ClrVm> = lock_vm();
    if let Some(target) = vm.classify_method_cached(method_id) {
        match target {
            ResolvedTarget::PInvoke(mid) => {
                let (_vm, res) = pinvoke::invoke_pinvoke(vm, None, mid, &args);
                return res;
            }
            ResolvedTarget::InternalCall { fn_ptr, ret_sig } => {
                let (_vm, res) = pinvoke::invoke_icall(vm, fn_ptr, &ret_sig, &args);
                return Ok(res);
            }
            ResolvedTarget::Managed(_)
            | ResolvedTarget::Bcl(_)
            | ResolvedTarget::DelegateInvoke
            | ResolvedTarget::UnsupportedBcl(_) => {}
        }
    }

    let mut exec = ExecState::new();
    let initial_argc = args.len();
    exec.val_stack.extend(args);
    exec.push_frame_in_place(&mut vm, method_id, 0, initial_argc, None)?;

    'frame: loop {
        let Some(f_idx) = exec.frames.len().checked_sub(1) else {
            return Ok(None);
        };

        // Hoist frame metadata and `Arc<MethodBody>` ONCE per frame activation.
        let body = exec.frames[f_idx].body.clone();
        let code: &[u8] = &body.code;
        let asm_id = exec.frames[f_idx].asm_id;
        let arg_base = exec.frames[f_idx].arg_base;
        let mut local_base = exec.frames[f_idx].local_base;
        let mut ip = exec.frames[f_idx].ip;
        let mut constrained_tok = exec.frames[f_idx].constrained_token.take();

        macro_rules! pop {
            () => {{
                let sb = exec.frames[f_idx].stack_base;
                if exec.val_stack.len() > sb {
                    exec.val_stack.pop().unwrap()
                } else {
                    CilVal::I32(0)
                }
            }};
        }
        macro_rules! push {
            ($v:expr) => {
                exec.val_stack.push($v)
            };
        }
        macro_rules! peek_top {
            ($default:expr) => {{
                let sb = exec.frames[f_idx].stack_base;
                if exec.val_stack.len() > sb {
                    *exec.val_stack.last().unwrap()
                } else {
                    $default
                }
            }};
        }
        macro_rules! read_u8 {
            () => {{
                let v = *code.get(ip).unwrap_or(&0);
                ip += 1;
                v
            }};
        }
        macro_rules! read_i8 {
            () => {{
                let v = *code.get(ip).unwrap_or(&0) as i8;
                ip += 1;
                v
            }};
        }
        macro_rules! read_u16 {
            () => {{
                let v = code
                    .get(ip..ip + 2)
                    .map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
                ip += 2;
                v
            }};
        }
        macro_rules! read_i32 {
            () => {{
                let v = code
                    .get(ip..ip + 4)
                    .map_or(0, |b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]));
                ip += 4;
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
                let v = code.get(ip..ip + 8).map_or(0, |b| {
                    i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
                });
                ip += 8;
                v
            }};
        }
        macro_rules! throw_exc {
            ($instr_ip:expr, $cls:expr, $msg:expr) => {{
                exec.frames[f_idx].ip = ip;
                let exc_cls = $cls;
                let exc_obj = vm.make_exception(exc_cls, $msg);
                if !dispatch_exception(&mut vm, &mut exec, exc_obj, $instr_ip) {
                    let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                    return Err(VmError::ManagedException {
                        exc_obj,
                        message: msg,
                    });
                }
                continue 'frame;
            }};
        }

        loop {
            if ip >= code.len() {
                // Fell off end of method body -> treat as void return.
                let out = exec.pop_frame(&mut vm, None);
                if exec.frames.is_empty() {
                    return Ok(out);
                }
                continue 'frame;
            }

            let instr_ip = ip;
            let op = code[ip];
            ip += 1;

            let constrained = if op != 0xFE {
                constrained_tok.take()
            } else {
                None
            };

            match op {
                // nop, break
                0x00 | 0x01 => {}

                // ldarg.0..3
                0x02..=0x05 => {
                    let idx = (op - 0x02) as usize;
                    let v = if idx < exec.frames[f_idx].arg_count {
                        exec.val_stack[arg_base + idx]
                    } else {
                        CilVal::I32(0)
                    };
                    push!(v);
                }
                // ldloc.0..3
                0x06..=0x09 => {
                    let idx = (op - 0x06) as usize;
                    let v = if idx < exec.frames[f_idx].local_count {
                        exec.val_stack[local_base + idx]
                    } else {
                        CilVal::I32(0)
                    };
                    push!(v);
                }
                // stloc.0..3
                0x0A..=0x0D => {
                    let idx = (op - 0x0A) as usize;
                    let v = pop!();
                    exec.store_local(&vm, f_idx, idx, v);
                }
                // ldarg.s
                0x0E => {
                    let idx = read_u8!() as usize;
                    let v = if idx < exec.frames[f_idx].arg_count {
                        exec.val_stack[arg_base + idx]
                    } else {
                        CilVal::I32(0)
                    };
                    push!(v);
                }
                // ldarga.s
                0x0F => {
                    let idx = read_u8!() as usize;
                    let ptr_val = match exec.read_arg(f_idx, idx) {
                        CilVal::ValueType { ptr, .. } => CilVal::RawPtr(ptr),
                        _ => CilVal::ArgPtr {
                            frame: f_idx as u32,
                            idx: idx as u32,
                        },
                    };
                    push!(ptr_val);
                }
                // starg.s
                0x10 => {
                    let idx = read_u8!() as usize;
                    let v = pop!();
                    exec.store_arg(&vm, f_idx, idx, v);
                    local_base = exec.frames[f_idx].local_base;
                }
                // ldloc.s
                0x11 => {
                    let idx = read_u8!() as usize;
                    let v = if idx < exec.frames[f_idx].local_count {
                        exec.val_stack[local_base + idx]
                    } else {
                        CilVal::I32(0)
                    };
                    push!(v);
                }
                // ldloca.s
                0x12 => {
                    let idx = read_u8!() as usize;
                    let ptr_val = match exec.read_local(f_idx, idx) {
                        CilVal::ValueType { ptr, .. } => CilVal::RawPtr(ptr),
                        _ => CilVal::LocalPtr {
                            frame: f_idx as u32,
                            idx: idx as u32,
                        },
                    };
                    push!(ptr_val);
                }
                // stloc.s
                0x13 => {
                    let idx = read_u8!() as usize;
                    let v = pop!();
                    exec.store_local(&vm, f_idx, idx, v);
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
                    let v = peek_top!(CilVal::I32(0));
                    push!(v);
                }
                // pop
                0x26 => {
                    let _ = pop!();
                }
                // jmp
                0x27 => {
                    let tok = read_u32!();
                    let target_mid = if ((tok >> 24) as u8) == table::METHOD_DEF {
                        let rid = (tok & 0x00FF_FFFF) as usize;
                        rid.checked_sub(1)
                            .and_then(|i| vm.assemblies[asm_id].method_ids.get(i).copied())
                    } else {
                        match vm.resolve_call_entry_cached(asm_id, tok).map(|e| e.target) {
                            Some(ResolvedTarget::Managed(mid)) => Some(mid),
                            _ => None,
                        }
                    };
                    if let Some(mid) = target_mid {
                        exec.replace_top_frame(&mut vm, mid)?;
                        continue 'frame;
                    }
                }
                // call / callvirt
                0x28 | 0x6F => {
                    let is_virt = op == 0x6F;
                    let tok = read_u32!();
                    exec.frames[f_idx].ip = ip;
                    exec.frames[f_idx].call_ip = instr_ip;

                    let sb = exec.frames[f_idx].stack_base;
                    let tbl = (tok >> 24) as u8;

                    let (argc, resolved_target) = if tbl == table::METHOD_DEF {
                        let rid = (tok & 0x00FF_FFFF) as usize;
                        let Some(&base_mid) = rid
                            .checked_sub(1)
                            .and_then(|i| vm.assemblies[asm_id].method_ids.get(i))
                        else {
                            return Err(VmError::MethodNotFound(format!("token 0x{tok:08x}")));
                        };
                        let argc = vm.methods[base_mid].argc;
                        let has_this = vm.methods[base_mid].sig.has_this;
                        let is_virtual = vm.methods[base_mid].is_virtual;
                        let base_cls = vm.methods[base_mid].class_id;

                        let avail = exec.val_stack.len().saturating_sub(sb);
                        let arg_start = if avail >= argc {
                            exec.val_stack.len() - argc
                        } else {
                            sb
                        };

                        let mut mid = base_mid;
                        if is_virt && has_this && constrained.is_none() {
                            let recv = exec
                                .val_stack
                                .get(arg_start)
                                .copied()
                                .unwrap_or(CilVal::Obj(0));
                            match recv {
                                CilVal::Obj(0) => {
                                    throw_exc!(
                                        instr_ip,
                                        vm.core.null_ref,
                                        "NullReferenceException in callvirt"
                                    );
                                }
                                CilVal::Obj(p) if is_virtual => {
                                    if let Some(rcid) = vm.object_class_id(p) {
                                        if rcid != base_cls {
                                            let mname = vm.methods[base_mid].name.clone();
                                            let pcnt = vm.methods[base_mid].sig.params.len() as i32;
                                            if let Some(ov) = vm.resolve_virtual_override_cached(
                                                asm_id, tok, rcid, &mname, pcnt,
                                            ) {
                                                mid = ov;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        let Some(target) = vm.classify_method_cached(mid) else {
                            return Err(VmError::MethodNotFound(format!("token 0x{tok:08x}")));
                        };
                        (argc, target)
                    } else {
                        let Some(entry) = vm.resolve_call_entry_cached(asm_id, tok) else {
                            return Err(VmError::MethodNotFound(format!("token 0x{tok:08x}")));
                        };
                        let argc = entry.argc;
                        let avail = exec.val_stack.len().saturating_sub(sb);
                        let arg_start = if avail >= argc {
                            exec.val_stack.len() - argc
                        } else {
                            sb
                        };

                        let mut target_override = None;
                        if let Some(ctok) = constrained {
                            if let Some(cid) = vm.resolve_type_token_cached(asm_id, ctok) {
                                if cid >= vm.corlib_class_count {
                                    if let Some(ov) = vm.resolve_virtual_override_cached(
                                        asm_id,
                                        tok,
                                        cid,
                                        &entry.method_name,
                                        entry.param_count,
                                    ) {
                                        target_override = vm.classify_method_cached(ov);
                                    }
                                }
                            }
                        } else if is_virt && entry.has_this {
                            let recv = exec
                                .val_stack
                                .get(arg_start)
                                .copied()
                                .unwrap_or(CilVal::Obj(0));
                            match recv {
                                CilVal::Obj(0) => {
                                    throw_exc!(
                                        instr_ip,
                                        vm.core.null_ref,
                                        "NullReferenceException in callvirt"
                                    );
                                }
                                CilVal::Obj(p) => {
                                    if let Some(rcid) = vm.object_class_id(p) {
                                        if rcid >= vm.corlib_class_count {
                                            if let Some(ov) = vm.resolve_virtual_override_cached(
                                                asm_id,
                                                tok,
                                                rcid,
                                                &entry.method_name,
                                                entry.param_count,
                                            ) {
                                                target_override = vm.classify_method_cached(ov);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        (argc, target_override.unwrap_or(entry.target))
                    };

                    match resolved_target {
                        ResolvedTarget::Managed(mid) => {
                            let callee_cls = vm.methods[mid].class_id;
                            if !vm.classes[callee_cls].cctor_ran {
                                vm.classes[callee_cls].cctor_ran = true;
                                if let Some(cctor_id) = vm.classes[callee_cls].cctor_method_id {
                                    // Args remain intact on val_stack; rewind IP and preserve constrained prefix so call retries after .cctor.
                                    exec.frames[f_idx].ip = instr_ip;
                                    exec.frames[f_idx].constrained_token = constrained;
                                    let cctor_arg_base = exec.val_stack.len();
                                    exec.push_frame_in_place(&mut vm, cctor_id, cctor_arg_base, 0, None)?;
                                    continue 'frame;
                                }
                            }
                            let avail = exec.val_stack.len().saturating_sub(sb);
                            let (arg_start, provided) = if avail >= argc {
                                (exec.val_stack.len() - argc, argc)
                            } else {
                                (sb, avail)
                            };
                            exec.push_frame_in_place(&mut vm, mid, arg_start, provided, None)?;
                            continue 'frame;
                        }
                        ResolvedTarget::Bcl(intr) => {
                            let call_args = pop_small_args(&mut exec.val_stack, sb, argc);
                            match bcl::dispatch_intrinsic(&mut vm, &mut exec, intr, call_args.as_slice()) {
                                Ok(Some(ret_v)) => push!(ret_v),
                                Ok(None) => {}
                                Err(VmError::ManagedException { exc_obj, message }) => {
                                    let exc = if exc_obj != 0 {
                                        exc_obj
                                    } else {
                                        let ec = vm.core.exception;
                                        vm.make_exception(ec, message)
                                    };
                                    if !dispatch_exception(&mut vm, &mut exec, exc, instr_ip) {
                                        let msg = vm.format_val_to_string(CilVal::Obj(exc));
                                        return Err(VmError::ManagedException {
                                            exc_obj: exc,
                                            message: msg,
                                        });
                                    }
                                    continue 'frame;
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        ResolvedTarget::PInvoke(mid) => {
                            let call_args = pop_small_args(&mut exec.val_stack, sb, argc);
                            let (next_vm, res) =
                                pinvoke::invoke_pinvoke(vm, Some(&mut exec), mid, call_args.as_slice());
                            vm = next_vm;
                            if let Some(r) = res? {
                                push!(r);
                            }
                        }
                        ResolvedTarget::InternalCall { fn_ptr, ret_sig } => {
                            let call_args = pop_small_args(&mut exec.val_stack, sb, argc);
                            let (next_vm, res) =
                                pinvoke::invoke_icall(vm, fn_ptr, &ret_sig, call_args.as_slice());
                            vm = next_vm;
                            if let Some(r) = res {
                                push!(r);
                            }
                        }
                        ResolvedTarget::DelegateInvoke => {
                            let avail = exec.val_stack.len().saturating_sub(sb);
                            let arg_start = if avail >= argc {
                                exec.val_stack.len() - argc
                            } else {
                                sb
                            };
                            let del_obj = exec
                                .val_stack
                                .get(arg_start)
                                .map_or(0, |v| v.as_obj());
                            let Some(HeapObjKind::Delegate {
                                method_id: target_mid,
                                target_obj,
                                ..
                            }) = vm.heap.get_managed(del_obj).cloned()
                            else {
                                throw_exc!(
                                    instr_ip,
                                    vm.core.null_ref,
                                    "NullReferenceException invoking delegate"
                                );
                            };
                            let total_avail = exec.val_stack.len() - arg_start;
                            let provided = if target_obj != 0 {
                                if total_avail > 0 {
                                    exec.val_stack[arg_start] = CilVal::Obj(target_obj);
                                    total_avail
                                } else {
                                    exec.val_stack.push(CilVal::Obj(target_obj));
                                    1
                                }
                            } else if total_avail > 0 {
                                exec.val_stack
                                    .copy_within(arg_start + 1..arg_start + total_avail, arg_start);
                                exec.val_stack.pop();
                                total_avail - 1
                            } else {
                                0
                            };
                            exec.push_frame_in_place(&mut vm, target_mid, arg_start, provided, None)?;
                            continue 'frame;
                        }
                        ResolvedTarget::UnsupportedBcl(name) => {
                            return Err(VmError::Unsupported(format!(
                                "unimplemented BCL method {name}"
                            )));
                        }
                    }
                }
                // calli
                0x29 => {
                    let sig_tok = read_u32!();
                    let fn_ptr = pop!().as_isize() as usize;
                    exec.frames[f_idx].ip = ip;
                    let sig = vm.resolve_standalone_sig_cached(asm_id, sig_tok);
                    let argc = sig.params.len();
                    let mut raw_args = [0u64; 8];
                    for i in (0..argc).rev() {
                        let v = pop!();
                        if i < 8 {
                            raw_args[i] = v.as_u64_bits();
                        }
                    }
                    if fn_ptr == 0 {
                        throw_exc!(instr_ip, vm.core.null_ref, "NullReferenceException in calli");
                    }
                    let (next_vm, res) = pinvoke::invoke_calli(vm, fn_ptr, &sig.ret, raw_args);
                    vm = next_vm;
                    if let Some(r) = res {
                        push!(r);
                    }
                }
                // ret
                0x2A => {
                    let ret_v = if exec.frames[f_idx].newobj_result.is_some()
                        || vm.methods[exec.frames[f_idx].method_id].is_void_ret
                    {
                        None
                    } else {
                        Some(peek_top!(CilVal::I32(0)))
                    };
                    let out = exec.pop_frame(&mut vm, ret_v);
                    if exec.frames.is_empty() {
                        return Ok(out);
                    }
                    continue 'frame;
                }

                // Short & long branches (0x2B..0x44)
                0x2B => {
                    let d = read_i8!() as isize;
                    ip = (ip as isize + d) as usize;
                }
                0x38 => {
                    let d = read_i32!() as isize;
                    ip = (ip as isize + d) as usize;
                }
                0x2C | 0x39 => {
                    let d = if op == 0x2C {
                        read_i8!() as isize
                    } else {
                        read_i32!() as isize
                    };
                    if !pop!().is_truthy() {
                        ip = (ip as isize + d) as usize;
                    }
                }
                0x2D | 0x3A => {
                    let d = if op == 0x2D {
                        read_i8!() as isize
                    } else {
                        read_i32!() as isize
                    };
                    if pop!().is_truthy() {
                        ip = (ip as isize + d) as usize;
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
                        ip = (ip as isize + d) as usize;
                    }
                }
                // switch
                0x45 => {
                    let count = read_u32!() as usize;
                    let base_table = ip;
                    let next_ip = base_table + count * 4;
                    ip = next_ip;
                    let idx = pop!().as_i32() as u32 as usize;
                    if idx < count {
                        let o = base_table + idx * 4;
                        if let Some(bytes) = code.get(o..o + 4) {
                            let rel =
                                i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as isize;
                            ip = (next_ip as isize + rel) as usize;
                        }
                    }
                }

                // ldind.* (0x46..0x50)
                0x46..=0x50 => {
                    let ptr_val = pop!();
                    let kind = match op {
                        0x46 => StorageKind::I1,
                        0x47 => StorageKind::U1,
                        0x48 => StorageKind::I2,
                        0x49 => StorageKind::U2,
                        0x4A => StorageKind::I4,
                        0x4B => StorageKind::U4,
                        0x4C => StorageKind::I8,
                        0x4D => StorageKind::NativeInt,
                        0x4E => StorageKind::R4,
                        0x4F => StorageKind::R8,
                        _ => StorageKind::Ref,
                    };
                    push!(exec.read_ptr_kind(&vm, ptr_val, kind));
                }
                // stind.* (0x51..0x57, 0xDF)
                0x51..=0x57 | 0xDF => {
                    let val = pop!();
                    let addr_val = pop!();
                    let kind = match op {
                        0x51 => StorageKind::Ref,
                        0x52 => StorageKind::I1,
                        0x53 => StorageKind::I2,
                        0x54 => StorageKind::I4,
                        0x55 => StorageKind::I8,
                        0x56 => StorageKind::R4,
                        0x57 => StorageKind::R8,
                        _ => StorageKind::NativeInt,
                    };
                    exec.write_ptr_kind(&mut vm, addr_val, kind, val);
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
                        Err(true) => throw_exc!(instr_ip, vm.core.div_by_zero, "DivideByZeroException"),
                        Err(false) => {
                            throw_exc!(instr_ip, vm.core.overflow, "OverflowException in div")
                        }
                    }
                }
                0x5C => {
                    let b = pop!();
                    let a = pop!();
                    match arith_div(a, b, true) {
                        Ok(v) => push!(v),
                        Err(_) => throw_exc!(instr_ip, vm.core.div_by_zero, "DivideByZeroException"),
                    }
                }
                0x5D => {
                    let b = pop!();
                    let a = pop!();
                    match arith_rem(a, b, false) {
                        Ok(v) => push!(v),
                        Err(true) => throw_exc!(instr_ip, vm.core.div_by_zero, "DivideByZeroException"),
                        Err(false) => {
                            throw_exc!(instr_ip, vm.core.overflow, "OverflowException in rem")
                        }
                    }
                }
                0x5E => {
                    let b = pop!();
                    let a = pop!();
                    match arith_rem(a, b, true) {
                        Ok(v) => push!(v),
                        Err(_) => throw_exc!(instr_ip, vm.core.div_by_zero, "DivideByZeroException"),
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
                        Err(()) => {
                            throw_exc!(instr_ip, vm.core.overflow, "OverflowException in conv.ovf")
                        }
                    }
                }
                // Overflow-checked arithmetic (0xD6..0xDB)
                0xD6..=0xDB => {
                    let b = pop!();
                    let a = pop!();
                    match arith_checked(op, a, b) {
                        Ok(r) => push!(r),
                        Err(()) => throw_exc!(
                            instr_ip,
                            vm.core.overflow,
                            "OverflowException in arithmetic"
                        ),
                    }
                }

                // cpobj, ldobj, stobj
                0x70 => {
                    let tok = read_u32!();
                    let src = pop!();
                    let dst = pop!();
                    let sz = vm
                        .resolve_type_token_cached(asm_id, tok)
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
                    let cid = vm
                        .resolve_type_token_cached(asm_id, tok)
                        .unwrap_or(vm.core.object);
                    if let Some(ps) = vm.classes[cid].primitive_sig.as_ref() {
                        let kind = StorageKind::from_sig(ps);
                        push!(exec.read_ptr_kind(&vm, src, kind));
                    } else if !vm.classes[cid].is_value_type {
                        push!(exec.read_ptr_kind(&vm, src, StorageKind::Ref));
                    } else {
                        let sz = vm.value_type_size(cid);
                        let dst_buf = exec.arena.alloc(sz);
                        let src_resolved = match src {
                            CilVal::StaticFieldPtr(fid) => vm
                                .fields
                                .get(fid as usize)
                                .map_or(CilVal::Obj(0), |f| f.static_value),
                            CilVal::LocalPtr { frame, idx } => {
                                exec.read_local(frame as usize, idx as usize)
                            }
                            CilVal::ArgPtr { frame, idx } => {
                                exec.read_arg(frame as usize, idx as usize)
                            }
                            other => other,
                        };
                        if let Some(sp) = raw_ptr_from_val(src_resolved) {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    sp as *const u8,
                                    dst_buf as *mut u8,
                                    sz,
                                );
                            }
                        }
                        push!(CilVal::ValueType {
                            class_id: cid as u32,
                            ptr: dst_buf,
                        });
                    }
                }
                0x81 => {
                    let tok = read_u32!();
                    let val = pop!();
                    let dst = pop!();
                    let cid = vm
                        .resolve_type_token_cached(asm_id, tok)
                        .unwrap_or(vm.core.object);
                    if let CilVal::ValueType {
                        class_id: vcid,
                        ptr: sp,
                    } = val
                    {
                        if let Some(dp) = raw_ptr_from_val(dst) {
                            let sz = vm.value_type_size(vcid as usize);
                            unsafe {
                                std::ptr::copy_nonoverlapping(sp as *const u8, dp as *mut u8, sz);
                            }
                        } else if let Some(addr) = dst.as_managed_addr() {
                            exec.write_managed_addr_kind(&mut vm, addr, StorageKind::Ref, val);
                        }
                    } else {
                        let kind = vm.classes[cid]
                            .primitive_sig
                            .as_ref()
                            .map_or(StorageKind::Ref, StorageKind::from_sig);
                        exec.write_ptr_kind(&mut vm, dst, kind, val);
                    }
                }

                // ldstr (interned per-assembly `#US` cache)
                0x72 => {
                    let tok = read_u32!();
                    let obj = vm.intern_user_string(asm_id, tok);
                    push!(CilVal::Obj(obj));
                }

                // newobj
                0x73 => {
                    let tok = read_u32!();
                    exec.frames[f_idx].ip = ip;
                    exec.frames[f_idx].call_ip = instr_ip;
                    let sb = exec.frames[f_idx].stack_base;

                    let Some(entry) = vm.resolve_newobj_cached(asm_id, tok) else {
                        return Err(VmError::MethodNotFound(format!(
                            "newobj token 0x{tok:08x}"
                        )));
                    };
                    let argc = entry.argc;

                    match entry.kind {
                        ResolvedNewObj::Delegate { class_id } => {
                            let ctor_args = pop_small_args(&mut exec.val_stack, sb, argc);
                            let s = ctor_args.as_slice();
                            let target_obj = s.first().map_or(0, |v| v.as_obj());
                            let method_id = s.get(1).map_or(0, |v| v.as_isize() as usize);
                            let vt = vm.classes[class_id].mono_vtable;
                            let addr =
                                vm.heap
                                    .alloc_delegate(vt, class_id, method_id, target_obj);
                            push!(CilVal::Obj(addr));
                        }
                        ResolvedNewObj::ManagedValueType {
                            ctor_mid,
                            class_id,
                            size,
                        } => {
                            if !vm.classes[class_id].cctor_ran {
                                vm.classes[class_id].cctor_ran = true;
                                if let Some(cctor_id) = vm.classes[class_id].cctor_method_id {
                                    exec.frames[f_idx].ip = instr_ip;
                                    let cbase = exec.val_stack.len();
                                    exec.push_frame_in_place(&mut vm, cctor_id, cbase, 0, None)?;
                                    continue 'frame;
                                }
                            }
                            // Allocate value-type backing in caller's arena scope so it outlives `.ctor`.
                            let buf = exec.arena.alloc(size);
                            let avail = exec.val_stack.len().saturating_sub(sb);
                            let arg_start = if avail >= argc {
                                exec.val_stack.len() - argc
                            } else {
                                sb
                            };
                            let moved = exec.val_stack.len() - arg_start;
                            exec.val_stack.push(CilVal::I32(0));
                            exec.val_stack
                                .copy_within(arg_start..arg_start + moved, arg_start + 1);
                            exec.val_stack[arg_start] = CilVal::RawPtr(buf);
                            let result_val = CilVal::ValueType {
                                class_id: class_id as u32,
                                ptr: buf,
                            };
                            exec.push_frame_in_place(
                                &mut vm,
                                ctor_mid,
                                arg_start,
                                moved + 1,
                                Some(result_val),
                            )?;
                            continue 'frame;
                        }
                        ResolvedNewObj::ManagedClass { ctor_mid, class_id } => {
                            if !vm.classes[class_id].cctor_ran {
                                vm.classes[class_id].cctor_ran = true;
                                if let Some(cctor_id) = vm.classes[class_id].cctor_method_id {
                                    exec.frames[f_idx].ip = instr_ip;
                                    let cbase = exec.val_stack.len();
                                    exec.push_frame_in_place(&mut vm, cctor_id, cbase, 0, None)?;
                                    continue 'frame;
                                }
                            }
                            let obj = vm.alloc_object_for_class(class_id);
                            let avail = exec.val_stack.len().saturating_sub(sb);
                            let arg_start = if avail >= argc {
                                exec.val_stack.len() - argc
                            } else {
                                sb
                            };
                            let moved = exec.val_stack.len() - arg_start;
                            exec.val_stack.push(CilVal::I32(0));
                            exec.val_stack
                                .copy_within(arg_start..arg_start + moved, arg_start + 1);
                            exec.val_stack[arg_start] = CilVal::Obj(obj);
                            exec.push_frame_in_place(
                                &mut vm,
                                ctor_mid,
                                arg_start,
                                moved + 1,
                                Some(CilVal::Obj(obj)),
                            )?;
                            continue 'frame;
                        }
                        ResolvedNewObj::Bcl(intr) => {
                            let ctor_args = pop_small_args(&mut exec.val_stack, sb, argc);
                            let obj = bcl::dispatch_newobj(&mut vm, intr, ctor_args.as_slice())?;
                            push!(obj);
                        }
                    }
                }

                // castclass / isinst
                0x74 | 0x75 => {
                    let tok = read_u32!();
                    let val = pop!();
                    let obj = val.as_obj();
                    let target_cid = vm
                        .resolve_type_token_cached(asm_id, tok)
                        .unwrap_or(vm.core.object);
                    if obj == 0 {
                        push!(CilVal::Obj(0));
                    } else if vm.is_instance_of(obj, target_cid) {
                        push!(CilVal::Obj(obj));
                    } else if op == 0x75 {
                        push!(CilVal::Obj(0));
                    } else {
                        let tname = vm.classes[target_cid].full_name();
                        throw_exc!(
                            instr_ip,
                            vm.core.invalid_cast,
                            format!("InvalidCastException to {tname}")
                        );
                    }
                }

                // unbox
                0x79 => {
                    let _tok = read_u32!();
                    let obj = pop!().as_obj();
                    if obj == 0 {
                        throw_exc!(instr_ip, vm.core.null_ref, "NullReferenceException in unbox");
                    }
                    push!(CilVal::RawPtr(obj + MONO_OBJECT_HEADER_SIZE));
                }

                // throw
                0x7A => {
                    exec.frames[f_idx].ip = ip;
                    let exc = pop!().as_obj();
                    let exc_obj = if exc == 0 {
                        let nr = vm.core.null_ref;
                        vm.make_exception(nr, "NullReferenceException (throw null)")
                    } else {
                        exc
                    };
                    if !dispatch_exception(&mut vm, &mut exec, exc_obj, instr_ip) {
                        let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                        return Err(VmError::ManagedException {
                            exc_obj,
                            message: msg,
                        });
                    }
                    continue 'frame;
                }

                // ldfld / ldflda / stfld
                0x7B | 0x7C | 0x7D => {
                    let tok = read_u32!();
                    let Some(ResolvedField::Field {
                        field_id,
                        struct_offset,
                        size,
                        is_static,
                        kind,
                        ..
                    }) = vm.resolve_field_cached(asm_id, tok)
                    else {
                        return Err(VmError::Unsupported(format!(
                            "field token 0x{tok:08x} not found"
                        )));
                    };

                    if op == 0x7D {
                        let val = pop!();
                        let target = pop!();
                        if is_static {
                            let promoted = exec.maybe_promote_static_val(&mut vm, val);
                            vm.fields[field_id].static_value = promoted;
                        } else {
                            let Some(base) = field_base_ptr(target) else {
                                throw_exc!(
                                    instr_ip,
                                    vm.core.null_ref,
                                    "NullReferenceException in stfld"
                                );
                            };
                            if let CilVal::ValueType {
                                class_id: vcid,
                                ptr: sp,
                            } = val
                            {
                                let copy_sz = vm.value_type_size(vcid as usize).min(size);
                                unsafe {
                                    std::ptr::copy_nonoverlapping(
                                        sp as *const u8,
                                        (base + struct_offset) as *mut u8,
                                        copy_sz,
                                    );
                                }
                            } else {
                                write_kind_to_ptr(
                                    kind,
                                    (base + struct_offset) as *mut u8,
                                    val,
                                );
                            }
                        }
                    } else {
                        let target = pop!();
                        if is_static {
                            if op == 0x7C {
                                push!(CilVal::StaticFieldPtr(field_id as u32));
                            } else {
                                push!(vm.fields[field_id].static_value);
                            }
                        } else {
                            let Some(base) = field_base_ptr(target) else {
                                throw_exc!(
                                    instr_ip,
                                    vm.core.null_ref,
                                    "NullReferenceException in ldfld"
                                );
                            };
                            if op == 0x7C {
                                push!(CilVal::RawPtr(base + struct_offset));
                            } else {
                                push!(read_kind_from_ptr(
                                    kind,
                                    (base + struct_offset) as *const u8,
                                ));
                            }
                        }
                    }
                }

                // ldsfld / ldsflda / stsfld
                0x7E | 0x7F | 0x80 => {
                    let tok = read_u32!();
                    match vm.resolve_field_cached(asm_id, tok) {
                        Some(ResolvedField::Field {
                            field_id, class_id, ..
                        }) => {
                            if !vm.classes[class_id].cctor_ran {
                                vm.classes[class_id].cctor_ran = true;
                                if let Some(cctor_id) = vm.classes[class_id].cctor_method_id {
                                    exec.frames[f_idx].ip = instr_ip;
                                    let cbase = exec.val_stack.len();
                                    exec.push_frame_in_place(&mut vm, cctor_id, cbase, 0, None)?;
                                    continue 'frame;
                                }
                            }
                            match op {
                                0x7E => push!(vm.fields[field_id].static_value),
                                0x7F => push!(CilVal::StaticFieldPtr(field_id as u32)),
                                _ => {
                                    let val = pop!();
                                    let promoted = exec.maybe_promote_static_val(&mut vm, val);
                                    vm.fields[field_id].static_value = promoted;
                                }
                            }
                        }
                        Some(ResolvedField::BclConst(bcl_val)) => {
                            if op == 0x7E {
                                push!(bcl_val);
                            } else if op == 0x80 {
                                let _ = pop!();
                            } else {
                                push!(CilVal::RawPtr(0));
                            }
                        }
                        None => {
                            return Err(VmError::Unsupported(format!(
                                "static field token 0x{tok:08x} not found"
                            )));
                        }
                    }
                }

                // box
                0x8C => {
                    let tok = read_u32!();
                    let val = pop!();
                    let cid = vm
                        .resolve_type_token_cached(asm_id, tok)
                        .unwrap_or(vm.core.object);
                    let boxed = vm.box_value(cid, val);
                    push!(CilVal::Obj(boxed));
                }

                // newarr
                0x8D => {
                    let tok = read_u32!();
                    let len = pop!().as_i32();
                    if len < 0 {
                        throw_exc!(
                            instr_ip,
                            vm.core.overflow,
                            "OverflowException: negative array length"
                        );
                    }
                    let elem_cid = vm
                        .resolve_type_token_cached(asm_id, tok)
                        .unwrap_or(vm.core.object);
                    let arr = vm.alloc_array_for_class(elem_cid, len as usize);
                    push!(CilVal::Obj(arr));
                }

                // ldlen
                0x8E => {
                    let arr = pop!().as_obj();
                    if arr == 0 {
                        throw_exc!(instr_ip, vm.core.null_ref, "NullReferenceException in ldlen");
                    }
                    let len = vm.heap.array_info(arr).map_or_else(
                        || unsafe {
                            std::ptr::read_unaligned((arr + MONO_ARRAY_LEN_OFFSET) as *const usize)
                        },
                        |(l, _)| l,
                    );
                    push!(CilVal::NativeInt(len as isize));
                }

                // ldelema
                0x8F => {
                    let _tok = read_u32!();
                    let idx = pop!().as_isize();
                    let arr = pop!().as_obj();
                    match array_elem_ptr(&vm, arr, idx) {
                        Ok((ep, _)) => push!(CilVal::RawPtr(ep)),
                        Err(true) => {
                            throw_exc!(
                                instr_ip,
                                vm.core.null_ref,
                                "NullReferenceException in ldelema"
                            )
                        }
                        Err(false) => throw_exc!(
                            instr_ip,
                            vm.core.index_out_of_range,
                            "IndexOutOfRangeException in ldelema"
                        ),
                    }
                }

                // ldelem.* (0x90..0x9A) and ldelem <tok> (0xA3)
                0x90..=0x9A | 0xA3 => {
                    let tok = if op == 0xA3 { Some(read_u32!()) } else { None };
                    let idx = pop!().as_isize();
                    let arr = pop!().as_obj();
                    match array_elem_ptr(&vm, arr, idx) {
                        Ok((ep, elem_sz)) => {
                            if let Some(t) = tok {
                                let cid = vm
                                    .resolve_type_token_cached(asm_id, t)
                                    .unwrap_or(vm.core.object);
                                if let Some(ps) = vm.classes[cid].primitive_sig.as_ref() {
                                    push!(read_kind_from_ptr(
                                        StorageKind::from_sig(ps),
                                        ep as *const u8,
                                    ));
                                } else if vm.classes[cid].is_value_type {
                                    let sz = vm.value_type_size(cid).min(elem_sz);
                                    let buf = exec.arena.alloc(sz);
                                    unsafe {
                                        std::ptr::copy_nonoverlapping(
                                            ep as *const u8,
                                            buf as *mut u8,
                                            sz,
                                        );
                                    }
                                    push!(CilVal::ValueType {
                                        class_id: cid as u32,
                                        ptr: buf,
                                    });
                                } else {
                                    push!(read_kind_from_ptr(StorageKind::Ref, ep as *const u8));
                                }
                            } else {
                                let kind = match op {
                                    0x90 => StorageKind::I1,
                                    0x91 => StorageKind::U1,
                                    0x92 => StorageKind::I2,
                                    0x93 => StorageKind::U2,
                                    0x94 => StorageKind::I4,
                                    0x95 => StorageKind::U4,
                                    0x96 => StorageKind::I8,
                                    0x97 => StorageKind::NativeInt,
                                    0x98 => StorageKind::R4,
                                    0x99 => StorageKind::R8,
                                    _ => StorageKind::Ref,
                                };
                                push!(read_kind_from_ptr(kind, ep as *const u8));
                            }
                        }
                        Err(true) => {
                            throw_exc!(
                                instr_ip,
                                vm.core.null_ref,
                                "NullReferenceException in ldelem"
                            )
                        }
                        Err(false) => throw_exc!(
                            instr_ip,
                            vm.core.index_out_of_range,
                            "IndexOutOfRangeException in ldelem"
                        ),
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
                            if let CilVal::ValueType {
                                class_id: vcid,
                                ptr: sp,
                            } = val
                            {
                                let sz = vm.value_type_size(vcid as usize).min(elem_sz);
                                unsafe {
                                    std::ptr::copy_nonoverlapping(
                                        sp as *const u8,
                                        ep as *mut u8,
                                        sz,
                                    );
                                }
                            } else {
                                let kind = match op {
                                    0x9B => StorageKind::NativeInt,
                                    0x9C => StorageKind::I1,
                                    0x9D => StorageKind::I2,
                                    0x9E => StorageKind::I4,
                                    0x9F => StorageKind::I8,
                                    0xA0 => StorageKind::R4,
                                    0xA1 => StorageKind::R8,
                                    0xA2 => StorageKind::Ref,
                                    _ => {
                                        let cid = tok
                                            .and_then(|t| vm.resolve_type_token_cached(asm_id, t))
                                            .unwrap_or(vm.core.object);
                                        vm.classes[cid]
                                            .primitive_sig
                                            .as_ref()
                                            .map_or(StorageKind::Ref, StorageKind::from_sig)
                                    }
                                };
                                write_kind_to_ptr(kind, ep as *mut u8, val);
                            }
                        }
                        Err(true) => {
                            throw_exc!(
                                instr_ip,
                                vm.core.null_ref,
                                "NullReferenceException in stelem"
                            )
                        }
                        Err(false) => throw_exc!(
                            instr_ip,
                            vm.core.index_out_of_range,
                            "IndexOutOfRangeException in stelem"
                        ),
                    }
                }

                // unbox.any
                0xA5 => {
                    let tok = read_u32!();
                    let obj = pop!().as_obj();
                    let cid = vm
                        .resolve_type_token_cached(asm_id, tok)
                        .unwrap_or(vm.core.object);
                    match vm.unbox_any(obj, cid) {
                        Ok(v) => push!(v),
                        Err(VmError::ManagedException { message, .. }) => {
                            let e_cls = if obj == 0 {
                                vm.core.null_ref
                            } else {
                                vm.core.invalid_cast
                            };
                            throw_exc!(instr_ip, e_cls, message);
                        }
                        Err(e) => return Err(e),
                    }
                }

                // ckfinite
                0xC3 => {
                    let v = peek_top!(CilVal::F64(0.0)).as_f64();
                    if !v.is_finite() {
                        throw_exc!(
                            instr_ip,
                            vm.core.overflow,
                            "ArithmeticException: non-finite float"
                        );
                    }
                }

                // ldtoken
                0xD0 => {
                    let tok = read_u32!();
                    let packed = (((asm_id as u64) << 32) | (tok as u64)) as isize;
                    push!(CilVal::NativeInt(packed));
                }

                // endfinally / endfault
                0xDC => {
                    exec.val_stack.truncate(exec.frames[f_idx].stack_base);
                    if let Some((target, leave_ip, next_idx)) =
                        exec.frames[f_idx].pending_leave.take()
                    {
                        if let Some((handler, idx_after)) =
                            find_exited_finally(&body.eh_clauses, leave_ip, target, next_idx)
                        {
                            exec.frames[f_idx].pending_leave = Some((target, leave_ip, idx_after));
                            ip = handler;
                        } else {
                            ip = target;
                        }
                    } else if let Some((exc_obj, throw_ip, next_idx)) =
                        exec.frames[f_idx].pending_unwind.take()
                    {
                        exec.frames[f_idx].ip = ip;
                        if !dispatch_exception_from(&mut vm, &mut exec, exc_obj, throw_ip, next_idx)
                        {
                            let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                            return Err(VmError::ManagedException {
                                exc_obj,
                                message: msg,
                            });
                        }
                        continue 'frame;
                    }
                }

                // leave / leave.s
                0xDD | 0xDE => {
                    let d = if op == 0xDE {
                        read_i8!() as isize
                    } else {
                        read_i32!() as isize
                    };
                    let target = (ip as isize + d) as usize;
                    exec.val_stack.truncate(exec.frames[f_idx].stack_base);
                    exec.frames[f_idx].caught_exc = None;
                    if let Some((handler, idx_after)) =
                        find_exited_finally(&body.eh_clauses, instr_ip, target, 0)
                    {
                        exec.frames[f_idx].pending_leave = Some((target, instr_ip, idx_after));
                        ip = handler;
                    } else {
                        ip = target;
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
                            push!(CilVal::I32(i32::from(eval_cmp_op(op2, a, b))));
                        }
                        // ldftn
                        0x06 => {
                            let tok = read_u32!();
                            let mid = vm.resolve_method_id_for_ftn(asm_id, tok, None);
                            push!(CilVal::NativeInt(mid as isize));
                        }
                        // ldvirtftn
                        0x07 => {
                            let tok = read_u32!();
                            let obj = pop!().as_obj();
                            let mid = vm.resolve_method_id_for_ftn(asm_id, tok, Some(obj));
                            push!(CilVal::NativeInt(mid as isize));
                        }
                        // ldarg
                        0x09 => {
                            let idx = read_u16!() as usize;
                            let v = if idx < exec.frames[f_idx].arg_count {
                                exec.val_stack[arg_base + idx]
                            } else {
                                CilVal::I32(0)
                            };
                            push!(v);
                        }
                        // ldarga
                        0x0A => {
                            let idx = read_u16!() as usize;
                            let ptr_val = match exec.read_arg(f_idx, idx) {
                                CilVal::ValueType { ptr, .. } => CilVal::RawPtr(ptr),
                                _ => CilVal::ArgPtr {
                                    frame: f_idx as u32,
                                    idx: idx as u32,
                                },
                            };
                            push!(ptr_val);
                        }
                        // starg
                        0x0B => {
                            let idx = read_u16!() as usize;
                            let v = pop!();
                            exec.store_arg(&vm, f_idx, idx, v);
                            local_base = exec.frames[f_idx].local_base;
                        }
                        // ldloc
                        0x0C => {
                            let idx = read_u16!() as usize;
                            let v = if idx < exec.frames[f_idx].local_count {
                                exec.val_stack[local_base + idx]
                            } else {
                                CilVal::I32(0)
                            };
                            push!(v);
                        }
                        // ldloca
                        0x0D => {
                            let idx = read_u16!() as usize;
                            let ptr_val = match exec.read_local(f_idx, idx) {
                                CilVal::ValueType { ptr, .. } => CilVal::RawPtr(ptr),
                                _ => CilVal::LocalPtr {
                                    frame: f_idx as u32,
                                    idx: idx as u32,
                                },
                            };
                            push!(ptr_val);
                        }
                        // stloc
                        0x0E => {
                            let idx = read_u16!() as usize;
                            let v = pop!();
                            exec.store_local(&vm, f_idx, idx, v);
                        }
                        // localloc (frame-scoped; reclaimed on frame exit!)
                        0x0F => {
                            let sz = pop!().as_isize().max(0) as usize;
                            let ptr = exec.arena.alloc(sz.max(8));
                            push!(CilVal::NativeInt(ptr as isize));
                        }
                        // endfilter
                        0x11 => {
                            let decision = pop!().as_i32();
                            if let Some((exc_obj, throw_ip, clause_idx)) =
                                exec.frames[f_idx].active_filter.take()
                            {
                                if decision != 0 {
                                    let handler = body.eh_clauses[clause_idx].handler_offset;
                                    exec.val_stack.truncate(exec.frames[f_idx].stack_base);
                                    exec.val_stack.push(CilVal::Obj(exc_obj));
                                    exec.frames[f_idx].caught_exc = Some(exc_obj);
                                    ip = handler;
                                } else {
                                    exec.frames[f_idx].ip = ip;
                                    if !dispatch_exception_from(
                                        &mut vm,
                                        &mut exec,
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
                                    continue 'frame;
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
                            let cid = vm
                                .resolve_type_token_cached(asm_id, tok)
                                .unwrap_or(vm.core.object);
                            let sz = vm.value_type_size(cid);
                            match dst.as_managed_addr() {
                                Some(ManagedAddr::Raw(p)) if p != 0 => unsafe {
                                    std::ptr::write_bytes(p as *mut u8, 0, sz);
                                },
                                Some(addr) => {
                                    let kind = vm.classes[cid]
                                        .primitive_sig
                                        .as_ref()
                                        .map_or(StorageKind::Ref, StorageKind::from_sig);
                                    exec.write_managed_addr_kind(
                                        &mut vm,
                                        addr,
                                        kind,
                                        kind.default_val(),
                                    );
                                }
                                None => {}
                            }
                        }
                        // constrained.
                        0x16 => {
                            let tok = read_u32!();
                            constrained_tok = Some(tok);
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
                            exec.frames[f_idx].ip = ip;
                            let exc_obj = exec.frames[f_idx].caught_exc.unwrap_or_else(|| {
                                let ec = vm.core.exception;
                                vm.make_exception(ec, "rethrow")
                            });
                            if !dispatch_exception(&mut vm, &mut exec, exc_obj, instr_ip) {
                                let msg = vm.format_val_to_string(CilVal::Obj(exc_obj));
                                return Err(VmError::ManagedException {
                                    exc_obj,
                                    message: msg,
                                });
                            }
                            continue 'frame;
                        }
                        // sizeof
                        0x1C => {
                            let tok = read_u32!();
                            let sz = vm
                                .resolve_type_token_cached(asm_id, tok)
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
}
