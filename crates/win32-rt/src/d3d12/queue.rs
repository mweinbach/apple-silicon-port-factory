//! ID3D12CommandQueue1 and ID3D12CommandAllocator.

use super::cmdlist;
use super::com::*;
use super::cs;
use super::tr::tr;
use super::failed;
use super::fence;
use super::vtables::*;
use crate::types::*;
use d3d12_metal::metal::cmdlist::CommandAllocator as MtlAllocator;
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::types::CommandQueueDesc;
use std::ffi::c_void;
use std::sync::{Arc, OnceLock};

// ---------------------------------------------------------------------------
// Queue
// ---------------------------------------------------------------------------

pub struct CommandQueue {
    pub q: Arc<Queue>,
    pub desc: CommandQueueDesc,
}

static QCLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for CommandQueue {
    fn class() -> &'static Class {
        class_once(&QCLASS, || Class {
            name: "ID3D12CommandQueue1",
            vtbl: build_named::<CommandQueue>(
                "ID3D12CommandQueue1",
                ID3D12CommandQueue1,
                &[
                    m!("ExecuteCommandLists" => execute_command_lists),
                    m!("SetMarker" => marker),
                    m!("BeginEvent" => marker),
                    m!("EndEvent" => end_event),
                    m!("Signal" => signal),
                    m!("Wait" => wait),
                    m!("GetTimestampFrequency" => get_timestamp_frequency),
                    m!("GetDesc" => get_desc),
                ],
            ),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12CommandQueue, IID_ID3D12CommandQueue1],
        })
    }

    fn final_release(&self) {
        // Outstanding GPU work keeps Metal objects alive by itself; wait so
        // deferred releases run and the app's "release after idle" holds.
        cs::flush();
        self.q.wait_idle();
    }
}

pub type PQ = *mut Obj<CommandQueue>;

pub unsafe fn from_ptr<'a>(p: *mut c_void) -> Option<&'a CommandQueue> {
    Obj::<CommandQueue>::from_iface(p).map(|o| &(*o).inner)
}

pub unsafe fn create(desc: *const CommandQueueDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreateCommandQueue: null desc");
    }
    let d = *desc;
    let Some(kind) = ListType::from_raw(d.Type).filter(|k| *k != ListType::Bundle) else {
        return failed!(E_INVALIDARG, "CreateCommandQueue: type {}", d.Type);
    };
    if out.is_null() {
        return S_FALSE;
    }
    *out = std::ptr::null_mut();
    let Some(q) = Queue::new(kind, &format!("D3D12 {kind:?} queue")) else {
        return failed!(E_OUTOFMEMORY, "CreateCommandQueue({d:?}): MTL4CommandQueue creation failed");
    };
    let o = Obj::new(CommandQueue { q: Arc::new(q), desc: d });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    log::info!(target: "d3d", "CreateCommandQueue({d:?}) -> {hr:#x} ({:?})", *out);
    hr
}

unsafe extern "win64" fn execute_command_lists(this: PQ, n: u32, lists: *const *mut c_void) {
    const INLINE_CAP: usize = 8;
    crate::shadow::guard_host("d3d12 ExecuteCommandLists");
    super::tr::sync_uploads();
    tr!(
        let ids: Vec<u64> = (0..n as usize).map(|i| *lists.add(i) as u64).collect();
        Call::ExecuteCommandLists { queue: this as u64, lists: U64s::from_slice(&ids) }
    );
    let q = Obj::get(this).q.clone();
    let t_ecl = d3d12_metal::metal::queue::capture_submit_timestamp();
    if (n as usize) <= INLINE_CAP {
        let mut inline: [cmdlist::PL; INLINE_CAP] = [std::ptr::null_mut(); INLINE_CAP];
        let mut len = 0usize;
        for i in 0..n as usize {
            let p = *lists.add(i);
            let Some(l) = Obj::<cmdlist::CommandList>::from_iface(p) else {
                log::error!(target: "d3d", "ExecuteCommandLists: list {i} ({p:?}) is not one of ours; skipped");
                continue;
            };
            inline[len] = l;
            len += 1;
        }
        crate::hitch::queue_op(crate::hitch::QOP_ECL_CALL, q.kind as u32, len as u64, 0);
        if !cs::enabled() {
            submit_lists(&q, &inline[..len], t_ecl);
            return;
        }
        // The lists' recorded commands were handed to the CS thread at Close;
        // submission follows them in order. Lists stay alive until submitted.
        let mut lp = [cs::SendPtr(std::ptr::null::<Obj<cmdlist::CommandList>>()); INLINE_CAP];
        for i in 0..len {
            Obj::add_ref(inline[i]);
            lp[i] = cs::SendPtr(inline[i] as *const _);
        }
        cs::submit(move || {
            let mut ls: [cmdlist::PL; INLINE_CAP] = [std::ptr::null_mut(); INLINE_CAP];
            for i in 0..len {
                ls[i] = lp[i].get() as cmdlist::PL;
            }
            submit_lists(&q, &ls[..len], t_ecl);
            for &l in &ls[..len] {
                Obj::release(l);
            }
        });
        return;
    }
    let mut ls: Vec<cmdlist::PL> = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        let p = *lists.add(i);
        let Some(l) = Obj::<cmdlist::CommandList>::from_iface(p) else {
            log::error!(target: "d3d", "ExecuteCommandLists: list {i} ({p:?}) is not one of ours; skipped");
            continue;
        };
        ls.push(l);
    }
    crate::hitch::queue_op(crate::hitch::QOP_ECL_CALL, q.kind as u32, ls.len() as u64, 0);
    if !cs::enabled() {
        submit_lists(&q, &ls, t_ecl);
        return;
    }
    // The lists' recorded commands were handed to the CS thread at Close;
    // submission follows them in order. Lists stay alive until submitted.
    for &l in &ls {
        Obj::add_ref(l);
    }
    let lp: Vec<cs::SendPtr<Obj<cmdlist::CommandList>>> = ls.iter().map(|&l| cs::SendPtr(l as *const _)).collect();
    cs::submit(move || {
        let ls: Vec<cmdlist::PL> = lp.iter().map(|p| p.get() as cmdlist::PL).collect();
        submit_lists(&q, &ls, t_ecl);
        for l in ls {
            Obj::release(l);
        }
    });
}

unsafe fn submit_lists(q: &Queue, ls: &[cmdlist::PL], t_ecl: u64) {
    let n = ls.len();
    let scope = crate::hitch::Scope::enter(crate::hitch::ActKind::CmdSubmit, n as u64);
    let mut batch = q.begin_prepare(t_ecl);
    for (i, &l) in ls.iter().enumerate() {
        let mut g = Obj::get(l).state.lock().unwrap();
        q.prepare_list(&mut batch, &mut g.l, i);
        // Release each `CommandList` lock immediately before committing to Metal.
    }
    let res = q.submit_prepared(batch);
    let elapsed_ns = scope.elapsed_ns();
    drop(scope);
    crate::hitch::record_cmd_submit(res.submitted_cbs as u64, elapsed_ns);
    log::trace!(
        target: "d3d",
        "ExecuteCommandLists({n}) -> {} command buffers, timeline {}",
        res.submitted_cbs,
        res.timeline_value
    );
}

/// PIX markers: no semantics (Metal debug groups would need an open encoder).
unsafe extern "win64" fn marker(_this: PQ, _meta: u32, _data: *const c_void, _size: u32) {}
unsafe extern "win64" fn end_event(_this: PQ) {}

unsafe extern "win64" fn signal(this: PQ, f: *mut c_void, v: u64) -> HRESULT {
    crate::shadow::guard_host("d3d12 CommandQueue::Signal");
    tr!(Call::QueueSignal { queue: this as u64, fence: f as u64, value: v });
    let fp = f;
    let Some(f) = fence::from_ptr(f) else { return failed!(E_INVALIDARG, "ID3D12CommandQueue::Signal: foreign fence {f:?}") };
    let kind = Obj::get(this).q.kind as u32;
    crate::hitch::queue_op(crate::hitch::QOP_SIGNAL_CALL, kind, fp as u64, v);
    if !cs::enabled() {
        Obj::get(this).q.signal(&f.f, v);
        return S_OK;
    }
    let q = Obj::get(this).q.clone();
    let mf = cs::SendPtr(&f.f as *const d3d12_metal::metal::fence::Fence);
    let fp = cs::SendPtr(fp as *const c_void);
    com_add_ref(fp.0 as *mut c_void);
    cs::submit(move || {
        crate::hitch::queue_op(crate::hitch::QOP_SIGNAL_EXEC, kind, fp.get() as u64, v);
        q.signal(&*mf.get(), v);
        com_release(fp.get() as *mut c_void);
    });
    S_OK
}

unsafe extern "win64" fn wait(this: PQ, f: *mut c_void, v: u64) -> HRESULT {
    tr!(Call::QueueWait { queue: this as u64, fence: f as u64, value: v });
    let fp = f;
    let Some(f) = fence::from_ptr(f) else { return failed!(E_INVALIDARG, "ID3D12CommandQueue::Wait: foreign fence {f:?}") };
    let kind = Obj::get(this).q.kind as u32;
    crate::hitch::queue_op(crate::hitch::QOP_WAIT_CALL, kind, fp as u64, v);
    if !cs::enabled() {
        Obj::get(this).q.wait(&f.f, v);
        return S_OK;
    }
    let q = Obj::get(this).q.clone();
    let mf = cs::SendPtr(&f.f as *const d3d12_metal::metal::fence::Fence);
    let fp = cs::SendPtr(fp as *const c_void);
    com_add_ref(fp.0 as *mut c_void);
    cs::submit(move || {
        crate::hitch::queue_op(crate::hitch::QOP_WAIT_EXEC, kind, fp.get() as u64, v);
        q.wait(&*mf.get(), v);
        com_release(fp.get() as *mut c_void);
    });
    S_OK
}

unsafe extern "win64" fn get_timestamp_frequency(_this: PQ, out: *mut u64) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = d3d12_metal::metal::query::timestamp_frequency();
    S_OK
}

unsafe extern "win64" fn get_desc(this: PQ, ret: *mut CommandQueueDesc) -> *mut CommandQueueDesc {
    *ret = Obj::get(this).desc;
    ret
}

// ---------------------------------------------------------------------------
// Allocator
// ---------------------------------------------------------------------------

pub struct CommandAllocator {
    pub a: MtlAllocator,
}

static ACLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for CommandAllocator {
    fn class() -> &'static Class {
        class_once(&ACLASS, || Class {
            name: "ID3D12CommandAllocator",
            vtbl: build_named::<CommandAllocator>("ID3D12CommandAllocator", ID3D12CommandAllocator, &[m!("Reset" => allocator_reset)]),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12CommandAllocator],
        })
    }
}

pub unsafe fn allocator_from_ptr<'a>(p: *mut c_void) -> Option<&'a CommandAllocator> {
    Obj::<CommandAllocator>::from_iface(p).map(|o| &(*o).inner)
}

pub unsafe fn create_allocator(ty: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let Some(kind) = ListType::from_raw(ty) else { return failed!(E_INVALIDARG, "CreateCommandAllocator: type {ty}") };
    if out.is_null() {
        return S_FALSE;
    }
    *out = std::ptr::null_mut();
    let Some(a) = MtlAllocator::new(kind) else { return failed!(E_OUTOFMEMORY, "CreateCommandAllocator({kind:?}): newCommandAllocator failed") };
    let o = Obj::new(CommandAllocator { a });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

/// Ordered on the CS thread after the commands recorded with the allocator.
unsafe extern "win64" fn allocator_reset(this: *mut Obj<CommandAllocator>) -> HRESULT {
    if !cs::enabled() {
        Obj::get(this).a.reset();
        return S_OK;
    }
    Obj::add_ref(this);
    let a = cs::SendPtr(this as *const Obj<CommandAllocator>);
    cs::submit(move || {
        let this = a.get() as *mut Obj<CommandAllocator>;
        Obj::get(this).a.reset();
        Obj::release(this);
    });
    S_OK
}

pub(crate) fn plain_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        vec![
            execute_command_lists as *const () as usize,
            marker as *const () as usize,
            end_event as *const () as usize,
            signal as *const () as usize,
            wait as *const () as usize,
            get_timestamp_frequency as *const () as usize,
            get_desc as *const () as usize,
            allocator_reset as *const () as usize,
        ]
        .into_boxed_slice()
    })
}
