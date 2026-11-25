//! Ownership of the single public reference to each grouped threadpool object.
//! Individual close and group drain take that reference by removing its entry.

use super::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub(super) enum Kind {
    Work,
    Simple,
    Timer,
    Wait,
    Io,
}

struct Member {
    kind: Kind,
    context: usize,
    cancel_cb: usize,
}

#[derive(Default)]
pub(super) struct Group {
    members: Mutex<HashMap<usize, Member>>,
}

#[derive(Default)]
pub(super) struct Membership {
    group: Option<Arc<Group>>,
    cancel_cb: usize,
}

impl Membership {
    pub(super) unsafe fn from_env(env: *mut c_void) -> Self {
        if env.is_null() {
            return Self::default();
        }
        // V1 and V3 share the Win64 prefix: Version, Pool, CleanupGroup, CancelCallback.
        let env = env.cast::<u8>();
        let group = env.add(16).cast::<*const Group>().read_unaligned();
        Self {
            group: (!group.is_null()).then(|| clone_arc_from_raw(group)),
            cancel_cb: env.add(24).cast::<usize>().read_unaligned(),
        }
    }

    pub(super) fn register(&self, addr: usize, kind: Kind, ctx: SendPtr) {
        if let Some(group) = &self.group {
            group.members.lock().unwrap().insert(
                addr,
                Member {
                    kind,
                    context: ctx.0 as usize,
                    cancel_cb: self.cancel_cb,
                },
            );
        }
    }

    pub(super) fn take(&self, addr: usize) -> bool {
        self.group
            .as_ref()
            .is_none_or(|g| g.members.lock().unwrap().remove(&addr).is_some())
    }
}

impl Group {
    pub(super) unsafe fn drain(&self, cancel: BOOL, cleanup_context: *mut c_void) {
        let mut members = std::mem::take(&mut *self.members.lock().unwrap());
        // First disarm producers and cancel queued jobs for the entire group. Waiting for
        // one callback here could let pending callbacks on another member start.
        for (&addr, member) in &mut members {
            match member.kind {
                Kind::Work | Kind::Simple if cancel != 0 => {
                    let removed = cancel_queued(addr, &(*(addr as *const TpWork)).pending);
                    if matches!(member.kind, Kind::Simple) && removed == 0 {
                        // A one-shot callback that already started closes itself on return.
                        member.cancel_cb = 0;
                    }
                }
                Kind::Timer => {
                    let t = &*(addr as *const TpTimer);
                    let mut state = t.state.lock().unwrap();
                    state.1 = false;
                    if cancel != 0 {
                        t.generation.fetch_add(1, Ordering::AcqRel);
                    }
                    drop(state);
                    timers()
                        .heap
                        .lock()
                        .unwrap()
                        .retain(|e| Arc::as_ptr(&e.timer) as usize != addr);
                    if cancel != 0 {
                        cancel_queued(addr, &t.pending);
                    }
                }
                Kind::Wait => {
                    let w = &*(addr as *const TpWait);
                    if cancel != 0 {
                        w.generation.fetch_add(1, Ordering::AcqRel);
                    }
                    disarm_wait(wait_dispatcher(), w);
                    if cancel != 0 {
                        cancel_queued(addr, &w.pending);
                    }
                }
                Kind::Io if cancel != 0 => {
                    let io = &*(addr as *const TpIo);
                    let mut st = io.state.lock().unwrap();
                    st.cancel_waiters += 1;
                    st.generation = st.generation.wrapping_add(1);
                    st.queued_callbacks = 0;
                    if st.is_idle() {
                        io.cv.notify_all();
                    }
                }
                _ => {}
            }
        }

        for (addr, member) in members {
            match member.kind {
                Kind::Work | Kind::Simple => {
                    Arc::from_raw(addr as *const TpWork).pending.wait_zero()
                }
                Kind::Wait => Arc::from_raw(addr as *const TpWait).pending.wait_zero(),
                Kind::Timer => Arc::from_raw(addr as *const TpTimer).pending.wait_zero(),
                Kind::Io => {
                    let io = Arc::from_raw(addr as *const TpIo);
                    WaitForThreadpoolIoCallbacks(addr as *mut TpIo, FALSE);
                    crate::kernel32::file::unbind_threadpool_io(io.file.0, &io);
                }
            }
            if cancel != 0 && member.cancel_cb != 0 {
                crate::call_guest8(
                    member.cancel_cb,
                    member.context as u64,
                    cleanup_context as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                );
            }
        }
    }
}
