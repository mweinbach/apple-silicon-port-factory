//! Execution stack state (`ExecState` / `ExecFrame`), slot & managed-pointer access,
//! frame-scoped value-type reclamation, and ECMA-335 exception handling/unwinding.

use super::super::metadata::{eh_flags, ExceptionClause, MethodBody, SigType};
use super::heap::FrameArena;
use super::value::{
    default_val_for_sig, raw_ptr_from_val, read_kind_from_ptr, read_sig_from_ptr,
    write_kind_to_ptr, write_sig_to_ptr, CilVal, ManagedAddr, StorageKind,
};
use super::{ClrVm, VmError};
use std::sync::Arc;

/// Pre-computed per-local initialization plan cached on `MethodInfo`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum LocalInit {
    Val(CilVal),
    Struct { class_id: u32, size: u32 },
}

pub(crate) struct ExecFrame {
    pub method_id: usize,
    pub asm_id: usize,
    pub body: Arc<MethodBody>,
    pub ip: usize,
    pub call_ip: usize,
    pub arg_base: usize,
    pub arg_count: usize,
    pub local_base: usize,
    pub local_count: usize,
    pub stack_base: usize,
    pub arena_mark: usize,
    /// If `Some`, `ret` pushes this value onto the caller stack (used by `newobj`).
    pub newobj_result: Option<CilVal>,
    /// Pending `leave` after `finally` finishes: `(target_ip, leave_ip, next_eh_idx)`.
    pub pending_leave: Option<(usize, usize, usize)>,
    /// Pending exception unwinding through `finally`/`fault`: `(exc_obj, throw_ip, next_eh_idx)`.
    pub pending_unwind: Option<(usize, usize, usize)>,
    /// Exception currently being handled inside a `catch` block (for `rethrow`).
    pub caught_exc: Option<usize>,
    /// Active filter clause state (for `endfilter`): `(exc_obj, throw_ip, clause_idx)`.
    pub active_filter: Option<(usize, usize, usize)>,
    /// Constrained prefix type token for the next `callvirt`.
    pub constrained_token: Option<u32>,
}

/// Contiguous value stack + call frames + frame-scoped value-type arena for one
/// `execute_method` invocation.
pub(crate) struct ExecState {
    pub frames: Vec<ExecFrame>,
    pub val_stack: Vec<CilVal>,
    pub arena: FrameArena,
}

impl ExecState {
    pub fn new() -> Self {
        Self {
            frames: Vec::with_capacity(16),
            val_stack: Vec::with_capacity(128),
            arena: FrameArena::new(),
        }
    }

    /// Push a new frame whose `arg_count` arguments are ALREADY placed at
    /// `val_stack[arg_base .. arg_base + arg_count]`. Zero `Vec` allocations per call.
    pub fn push_frame_in_place(
        &mut self,
        vm: &mut ClrVm,
        method_id: usize,
        arg_base: usize,
        provided_args: usize,
        newobj_result: Option<CilVal>,
    ) -> Result<(), VmError> {
        let (asm_id, body, expected_argc, local_plan) = {
            let m = vm
                .methods
                .get(method_id)
                .ok_or_else(|| VmError::MethodNotFound(format!("method_id {method_id}")))?;
            let body = m
                .body
                .clone()
                .ok_or_else(|| VmError::MethodNotFound(format!("method {} has no IL body", m.name)))?;
            let plan = m.local_init_plan.clone();
            (m.asm_id, body, m.argc, plan)
        };

        let local_plan = match local_plan {
            Some(p) => p,
            None => build_and_cache_local_plan(vm, method_id),
        };

        let arena_mark = self.arena.mark();
        let arg_count = provided_args.max(expected_argc);
        if provided_args < arg_count {
            self.val_stack.resize(arg_base + arg_count, CilVal::I32(0));
        }
        let local_base = arg_base + arg_count;
        let local_count = local_plan.len();
        self.val_stack.reserve(local_count + 8);
        for &init in local_plan.iter() {
            let v = match init {
                LocalInit::Val(cv) => cv,
                LocalInit::Struct { class_id, size } => {
                    let ptr = self.arena.alloc(size as usize);
                    CilVal::ValueType { class_id, ptr }
                }
            };
            self.val_stack.push(v);
        }
        let stack_base = local_base + local_count;

        self.frames.push(ExecFrame {
            method_id,
            asm_id,
            body,
            ip: 0,
            call_ip: 0,
            arg_base,
            arg_count,
            local_base,
            local_count,
            stack_base,
            arena_mark,
            newobj_result,
            pending_leave: None,
            pending_unwind: None,
            caught_exc: None,
            active_filter: None,
            constrained_token: None,
        });
        Ok(())
    }

    /// Replace the top frame in place (used by `jmp`), reusing its `arg_base..local_base`.
    pub fn replace_top_frame(
        &mut self,
        vm: &mut ClrVm,
        method_id: usize,
    ) -> Result<(), VmError> {
        let Some(top) = self.frames.pop() else {
            return Ok(());
        };
        self.val_stack.truncate(top.arg_base + top.arg_count);
        self.arena.reclaim(top.arena_mark);
        self.push_frame_in_place(vm, method_id, top.arg_base, top.arg_count, top.newobj_result)
    }

    /// Pop the top frame, reclaiming its stack slots and frame-scoped value-type allocations.
    /// If the return value is a scratch-allocated `ValueType`, it is adopted by the caller frame
    /// (zero-copy) or promoted to the managed heap when returning out of `execute_method`.
    pub fn pop_frame(
        &mut self,
        vm: &mut ClrVm,
        mut ret_val: Option<CilVal>,
    ) -> Option<CilVal> {
        let finished = self.frames.pop()?;
        if let Some(newobj) = finished.newobj_result {
            ret_val = Some(newobj);
        }
        self.val_stack.truncate(finished.arg_base);

        if let Some(CilVal::ValueType { class_id, ptr }) = ret_val {
            if self.frames.is_empty() {
                if self.arena.contains_ptr(ptr) {
                    let sz = vm.value_type_size(class_id as usize);
                    let heap_ptr = vm.heap.alloc_raw_struct(class_id as usize, sz);
                    unsafe {
                        std::ptr::copy_nonoverlapping(ptr as *const u8, heap_ptr as *mut u8, sz);
                    }
                    ret_val = Some(CilVal::ValueType {
                        class_id,
                        ptr: heap_ptr,
                    });
                }
                self.arena.reclaim(finished.arena_mark);
            } else {
                let _ = self.arena.reclaim_keeping(finished.arena_mark, ptr);
            }
        } else {
            self.arena.reclaim(finished.arena_mark);
        }

        if !self.frames.is_empty() {
            if let Some(v) = ret_val {
                self.val_stack.push(v);
            }
        }
        ret_val
    }

    /// Promote a scratch-allocated `ValueType` to the permanent heap if stored into a static field.
    #[inline]
    pub fn maybe_promote_static_val(&self, vm: &mut ClrVm, val: CilVal) -> CilVal {
        if let CilVal::ValueType { class_id, ptr } = val {
            if self.arena.contains_ptr(ptr) {
                let sz = vm.value_type_size(class_id as usize);
                let heap_ptr = vm.heap.alloc_raw_struct(class_id as usize, sz);
                unsafe {
                    std::ptr::copy_nonoverlapping(ptr as *const u8, heap_ptr as *mut u8, sz);
                }
                return CilVal::ValueType {
                    class_id,
                    ptr: heap_ptr,
                };
            }
        }
        val
    }

    #[inline]
    pub fn read_local(&self, frame_idx: usize, idx: usize) -> CilVal {
        let Some(f) = self.frames.get(frame_idx) else {
            return CilVal::I32(0);
        };
        if idx < f.local_count {
            self.val_stack[f.local_base + idx]
        } else {
            CilVal::I32(0)
        }
    }

    #[inline]
    pub fn read_arg(&self, frame_idx: usize, idx: usize) -> CilVal {
        let Some(f) = self.frames.get(frame_idx) else {
            return CilVal::I32(0);
        };
        if idx < f.arg_count {
            self.val_stack[f.arg_base + idx]
        } else {
            CilVal::I32(0)
        }
    }

    #[inline]
    pub fn store_local(&mut self, vm: &ClrVm, frame_idx: usize, idx: usize, val: CilVal) {
        let Some(f) = self.frames.get(frame_idx) else {
            return;
        };
        if idx < f.local_count {
            let slot_pos = f.local_base + idx;
            store_val_slot(vm, &mut self.val_stack[slot_pos], val);
            return;
        }
        if frame_idx + 1 == self.frames.len() {
            self.grow_locals_and_store(vm, frame_idx, idx, val);
        }
    }

    #[inline]
    pub fn store_arg(&mut self, vm: &ClrVm, frame_idx: usize, idx: usize, val: CilVal) {
        let Some(f) = self.frames.get(frame_idx) else {
            return;
        };
        if idx < f.arg_count {
            let slot_pos = f.arg_base + idx;
            store_val_slot(vm, &mut self.val_stack[slot_pos], val);
            return;
        }
        if frame_idx + 1 == self.frames.len() {
            self.grow_args_and_store(vm, frame_idx, idx, val);
        }
    }

    #[cold]
    fn grow_locals_and_store(&mut self, vm: &ClrVm, frame_idx: usize, idx: usize, val: CilVal) {
        let old_count = self.frames[frame_idx].local_count;
        let extra = idx + 1 - old_count;
        let insert_at = self.frames[frame_idx].stack_base;
        self.val_stack
            .splice(insert_at..insert_at, std::iter::repeat_n(CilVal::I32(0), extra));
        self.frames[frame_idx].local_count += extra;
        self.frames[frame_idx].stack_base += extra;
        for f in &mut self.frames[frame_idx + 1..] {
            f.arg_base += extra;
            f.local_base += extra;
            f.stack_base += extra;
        }
        let slot_pos = self.frames[frame_idx].local_base + idx;
        store_val_slot(vm, &mut self.val_stack[slot_pos], val);
    }

    #[cold]
    fn grow_args_and_store(&mut self, vm: &ClrVm, frame_idx: usize, idx: usize, val: CilVal) {
        let old_count = self.frames[frame_idx].arg_count;
        let extra = idx + 1 - old_count;
        let insert_at = self.frames[frame_idx].local_base;
        self.val_stack
            .splice(insert_at..insert_at, std::iter::repeat_n(CilVal::I32(0), extra));
        self.frames[frame_idx].arg_count += extra;
        self.frames[frame_idx].local_base += extra;
        self.frames[frame_idx].stack_base += extra;
        for f in &mut self.frames[frame_idx + 1..] {
            f.arg_base += extra;
            f.local_base += extra;
            f.stack_base += extra;
        }
        let slot_pos = self.frames[frame_idx].arg_base + idx;
        store_val_slot(vm, &mut self.val_stack[slot_pos], val);
    }

    #[inline]
    pub fn read_ptr_kind(&self, vm: &ClrVm, ptr_val: CilVal, kind: StorageKind) -> CilVal {
        match ptr_val {
            CilVal::LocalPtr { frame, idx } => self.read_local(frame as usize, idx as usize),
            CilVal::ArgPtr { frame, idx } => self.read_arg(frame as usize, idx as usize),
            CilVal::StaticFieldPtr(fid) => vm
                .fields
                .get(fid as usize)
                .map_or_else(|| kind.default_val(), |f| f.static_value),
            _ => {
                if let Some(p) = raw_ptr_from_val(ptr_val) {
                    read_kind_from_ptr(kind, p as *const u8)
                } else {
                    kind.default_val()
                }
            }
        }
    }

    #[inline]
    pub fn read_ptr_val(&self, vm: &ClrVm, ptr_val: CilVal, sig: &SigType) -> CilVal {
        match ptr_val {
            CilVal::LocalPtr { frame, idx } => self.read_local(frame as usize, idx as usize),
            CilVal::ArgPtr { frame, idx } => self.read_arg(frame as usize, idx as usize),
            CilVal::StaticFieldPtr(fid) => vm
                .fields
                .get(fid as usize)
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

    #[inline]
    pub fn write_ptr_kind(
        &mut self,
        vm: &mut ClrVm,
        addr_val: CilVal,
        kind: StorageKind,
        val: CilVal,
    ) {
        if let Some(addr) = addr_val.as_managed_addr() {
            self.write_managed_addr_kind(vm, addr, kind, val);
        } else if let Some(p) = raw_ptr_from_val(addr_val) {
            write_kind_to_ptr(kind, p as *mut u8, val);
        }
    }

    #[inline]
    pub fn write_managed_addr_kind(
        &mut self,
        vm: &mut ClrVm,
        addr: ManagedAddr,
        kind: StorageKind,
        val: CilVal,
    ) {
        match addr {
            ManagedAddr::Local { frame, idx } => {
                self.store_local(vm, frame as usize, idx as usize, val);
            }
            ManagedAddr::Arg { frame, idx } => {
                self.store_arg(vm, frame as usize, idx as usize, val);
            }
            ManagedAddr::StaticField(fid) => {
                let promoted = self.maybe_promote_static_val(vm, val);
                if let Some(f) = vm.fields.get_mut(fid as usize) {
                    f.static_value = promoted;
                }
            }
            ManagedAddr::Raw(p) => {
                if p != 0 {
                    write_kind_to_ptr(kind, p as *mut u8, val);
                }
            }
        }
    }

    #[inline]
    pub fn write_managed_addr(
        &mut self,
        vm: &mut ClrVm,
        addr: ManagedAddr,
        sig: &SigType,
        val: CilVal,
    ) {
        match addr {
            ManagedAddr::Local { frame, idx } => {
                self.store_local(vm, frame as usize, idx as usize, val);
            }
            ManagedAddr::Arg { frame, idx } => {
                self.store_arg(vm, frame as usize, idx as usize, val);
            }
            ManagedAddr::StaticField(fid) => {
                let promoted = self.maybe_promote_static_val(vm, val);
                if let Some(f) = vm.fields.get_mut(fid as usize) {
                    f.static_value = promoted;
                }
            }
            ManagedAddr::Raw(p) => {
                if p != 0 {
                    write_sig_to_ptr(sig, p as *mut u8, val);
                }
            }
        }
    }
}

#[inline]
fn store_val_slot(vm: &ClrVm, slot: &mut CilVal, val: CilVal) {
    if let (
        CilVal::ValueType {
            class_id: dcid,
            ptr: dp,
        },
        CilVal::ValueType {
            class_id: scid,
            ptr: sp,
        },
    ) = (*slot, val)
    {
        if dp != 0 && sp != 0 {
            let sz = vm
                .value_type_size(dcid as usize)
                .min(vm.value_type_size(scid as usize));
            unsafe {
                std::ptr::copy_nonoverlapping(sp as *const u8, dp as *mut u8, sz);
            }
            return;
        }
    }
    *slot = val;
}

fn build_and_cache_local_plan(vm: &mut ClrVm, method_id: usize) -> Arc<[LocalInit]> {
    let asm_id = vm.methods[method_id].asm_id;
    let local_sigs = vm.methods[method_id].locals_sig.clone();
    let mut plan = Vec::with_capacity(local_sigs.len());
    for lsig in &local_sigs {
        let entry = match lsig {
            SigType::ValueType(tok) | SigType::GenericInst(true, tok, _) => {
                if let Some(cid) = vm.resolve_type_token_cached(asm_id, *tok) {
                    if let Some(ps) = &vm.classes[cid].primitive_sig {
                        LocalInit::Val(default_val_for_sig(ps))
                    } else {
                        let sz = vm.value_type_size(cid) as u32;
                        LocalInit::Struct {
                            class_id: cid as u32,
                            size: sz,
                        }
                    }
                } else {
                    LocalInit::Val(CilVal::I32(0))
                }
            }
            other => LocalInit::Val(default_val_for_sig(other)),
        };
        plan.push(entry);
    }
    let arc: Arc<[LocalInit]> = plan.into();
    vm.methods[method_id].local_init_plan = Some(arc.clone());
    arc
}

// ---------------------------------------------------------------------------
// Exception Handling Clause Search & Unwinding
// ---------------------------------------------------------------------------

pub(crate) fn find_exited_finally(
    clauses: &[ExceptionClause],
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

pub(crate) fn dispatch_exception(
    vm: &mut ClrVm,
    exec: &mut ExecState,
    exc_obj: usize,
    throw_ip: usize,
) -> bool {
    dispatch_exception_from(vm, exec, exc_obj, throw_ip, 0)
}

pub(crate) fn dispatch_exception_from(
    vm: &mut ClrVm,
    exec: &mut ExecState,
    exc_obj: usize,
    mut ip: usize,
    mut start_clause: usize,
) -> bool {
    while let Some(f_idx) = exec.frames.len().checked_sub(1) {
        let asm_id = exec.frames[f_idx].asm_id;
        let n_clauses = exec.frames[f_idx].body.eh_clauses.len();
        for i in start_clause..n_clauses {
            let c = exec.frames[f_idx].body.eh_clauses[i];
            let try_end = c.try_offset + c.try_length;
            if ip < c.try_offset || ip >= try_end {
                continue;
            }
            if c.flags == eh_flags::FINALLY || c.flags == eh_flags::FAULT {
                let sb = exec.frames[f_idx].stack_base;
                exec.val_stack.truncate(sb);
                exec.frames[f_idx].pending_unwind = Some((exc_obj, ip, i + 1));
                exec.frames[f_idx].ip = c.handler_offset;
                return true;
            }
            if c.flags == eh_flags::FILTER {
                let sb = exec.frames[f_idx].stack_base;
                exec.val_stack.truncate(sb);
                exec.val_stack.push(CilVal::Obj(exc_obj));
                exec.frames[f_idx].active_filter = Some((exc_obj, ip, i));
                exec.frames[f_idx].ip = c.class_token_or_filter_offset as usize;
                return true;
            }
            if c.flags == eh_flags::EXCEPTION {
                let matches = if c.class_token_or_filter_offset == 0 {
                    true
                } else if let Some(cid) =
                    vm.resolve_type_token_cached(asm_id, c.class_token_or_filter_offset)
                {
                    vm.is_instance_of(exc_obj, cid)
                } else {
                    true
                };
                if matches {
                    let sb = exec.frames[f_idx].stack_base;
                    exec.val_stack.truncate(sb);
                    exec.val_stack.push(CilVal::Obj(exc_obj));
                    exec.frames[f_idx].caught_exc = Some(exc_obj);
                    exec.frames[f_idx].ip = c.handler_offset;
                    return true;
                }
            }
        }
        let popped = exec.frames.pop().unwrap();
        exec.val_stack.truncate(popped.arg_base);
        exec.arena.reclaim(popped.arena_mark);
        if let Some(caller) = exec.frames.last() {
            ip = caller.call_ip;
            start_clause = 0;
        }
    }
    false
}
