//! D3D12 command stream (CS) thread.
//!
//! Command-list methods only capture their arguments (CPU descriptors are
//! still read at call time, as D3D12 specifies) into a per-list [`CmdBuf`];
//! batches are replayed into the Metal command list, and queue operations
//! (`ExecuteCommandLists`, `Signal`, `Wait`, allocator resets) run, in call
//! order on one thread. Metal encoding thus overlaps the game's recording
//! thread instead of running inline on it. `Present` and other operations
//! that need submitted work call [`flush`] first.
//!
//! `AW2_D3D_CS=0` encodes inline at call time (A/B switch).

use std::collections::VecDeque;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::{Condvar, Mutex, OnceLock};

/// Encoding on the CS thread (default) vs. inline at call time.
#[inline]
pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        let on = std::env::var_os("AW2_D3D_CS").is_none_or(|v| v != "0");
        log::info!(target: "d3d", "D3D12 command stream thread: {}", if on { "on" } else { "off (AW2_D3D_CS=0)" });
        on
    })
}

/// Recorded commands replayed per batch once a list holds this many.
pub const BATCH: usize = 256;

#[repr(C)]
struct Hdr<S> {
    /// Runs (`s` non-null) or drops (`s` null) the closure at `p`.
    call: unsafe fn(*mut u8, *mut S),
    /// 16-byte units of closure storage following the header.
    units: usize,
}

unsafe fn shim<S, F: FnOnce(&mut S)>(p: *mut u8, s: *mut S) {
    let f = (p as *mut F).read();
    if !s.is_null() {
        f(&mut *s);
    }
}

/// Inline arena of type-erased `FnOnce(&mut S)` closures (no allocation per
/// command once the buffer has grown).
pub struct CmdBuf<S> {
    buf: Vec<MaybeUninit<u128>>,
    n: usize,
    _s: PhantomData<fn(&mut S)>,
}

// SAFETY: closures capture plain data, `Arc`s and raw pointers to D3D12
// objects the application keeps alive until the work executes (D3D12 object
// lifetime rules); they are run on exactly one thread at a time.
unsafe impl<S> Send for CmdBuf<S> {}

impl<S> Default for CmdBuf<S> {
    fn default() -> Self {
        CmdBuf { buf: Vec::new(), n: 0, _s: PhantomData }
    }
}

impl<S> CmdBuf<S> {
    #[inline]
    pub fn len(&self) -> usize {
        self.n
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    #[inline]
    pub fn push<F: FnOnce(&mut S)>(&mut self, f: F) {
        const { assert!(std::mem::size_of::<Hdr<S>>() == 16) };
        const { assert!(std::mem::align_of::<F>() <= 16) };
        let units = std::mem::size_of::<F>().div_ceil(16);
        let len = self.buf.len();
        self.buf.reserve(1 + units);
        unsafe {
            let p = self.buf.as_mut_ptr().add(len);
            (p as *mut Hdr<S>).write(Hdr { call: shim::<S, F>, units });
            (p.add(1) as *mut F).write(f);
            self.buf.set_len(len + 1 + units);
        }
        self.n += 1;
    }

    unsafe fn drain(&mut self, s: *mut S) {
        let base = self.buf.as_mut_ptr();
        let end = self.buf.len();
        // Consumed closures must not be run/dropped again if one panics.
        self.buf.set_len(0);
        self.n = 0;
        let mut i = 0;
        while i < end {
            let h = (base.add(i) as *const Hdr<S>).read();
            (h.call)(base.add(i + 1) as *mut u8, s);
            i += 1 + h.units;
        }
    }

    /// Run every closure in order against `s`, leaving the buffer empty.
    pub fn run(&mut self, s: &mut S) {
        unsafe { self.drain(s) }
    }

    /// Drop every closure without running it.
    pub fn clear(&mut self) {
        unsafe { self.drain(std::ptr::null_mut()) }
    }
}

impl<S> Drop for CmdBuf<S> {
    fn drop(&mut self) {
        self.clear();
    }
}

type Job = Box<dyn FnOnce() + Send>;

struct Cs {
    q: Mutex<State>,
    work: Condvar,
    idle: Condvar,
}

struct State {
    jobs: VecDeque<Job>,
    pushed: u64,
    done: u64,
    waiters: usize,
}

static CS_ENCODE_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static CS_FLUSH_WAIT_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Return and reset `(cs_encode_ns, cs_flush_wait_ns)` accumulated since the previous call.
pub fn take_perf_stats() -> (u64, u64) {
    (
        CS_ENCODE_NS.swap(0, std::sync::atomic::Ordering::Relaxed),
        CS_FLUSH_WAIT_NS.swap(0, std::sync::atomic::Ordering::Relaxed),
    )
}

thread_local! {
    static ON_CS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn cs() -> &'static Cs {
    static CS: OnceLock<Cs> = OnceLock::new();
    CS.get_or_init(|| {
        std::thread::Builder::new()
            .name("d3d12-metal-cs".into())
            .spawn(run_thread)
            .expect("spawn D3D12 command stream thread");
        Cs { q: Mutex::new(State { jobs: VecDeque::new(), pushed: 0, done: 0, waiters: 0 }), work: Condvar::new(), idle: Condvar::new() }
    })
}

fn run_thread() {
    ON_CS.with(|c| c.set(true));
    unsafe {
        // Encoding is latency-critical: same QoS as the game's render thread.
        libc::pthread_set_qos_class_self_np(libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, 0);
    }
    let cs = cs();
    loop {
        let job = {
            let mut st = cs.q.lock().unwrap();
            loop {
                if let Some(j) = st.jobs.pop_front() {
                    break j;
                }
                if d3d12_metal::metal::queue::has_pending_cbs() {
                    let (new_st, timeout) = cs
                        .work
                        .wait_timeout(st, std::time::Duration::from_millis(2))
                        .unwrap();
                    st = new_st;
                    if timeout.timed_out() && st.jobs.is_empty() {
                        drop(st);
                        d3d12_metal::metal::queue::Queue::flush_all_pending_cbs();
                        st = cs.q.lock().unwrap();
                    }
                } else {
                    st = cs.work.wait(st).unwrap();
                }
            }
        };
        let t0 = crate::hitch::now_ns();
        objc2::rc::autoreleasepool(|_| job());
        CS_ENCODE_NS.fetch_add(crate::hitch::now_ns().saturating_sub(t0), std::sync::atomic::Ordering::Relaxed);
        let mut st = cs.q.lock().unwrap();
        st.done += 1;
        if st.waiters > 0 {
            cs.idle.notify_all();
        }
    }
}

/// Run `job` on the CS thread after everything submitted before it.
pub fn submit(job: impl FnOnce() + Send + 'static) {
    let cs = cs();
    let mut st = cs.q.lock().unwrap();
    st.jobs.push_back(Box::new(job));
    st.pushed += 1;
    cs.work.notify_one();
}

/// Block until every job submitted so far has run (no-op on the CS thread
/// itself or when the CS thread is disabled).
pub fn flush() {
    if !enabled() || ON_CS.with(|c| c.get()) {
        return;
    }
    let cs = cs();
    let mut st = cs.q.lock().unwrap();
    let target = st.pushed;
    if st.done < target {
        let t0 = crate::hitch::now_ns();
        st.waiters += 1;
        while st.done < target {
            st = cs.idle.wait(st).unwrap();
        }
        st.waiters -= 1;
        CS_FLUSH_WAIT_NS.fetch_add(crate::hitch::now_ns().saturating_sub(t0), std::sync::atomic::Ordering::Relaxed);
    }
    drop(st);
    if d3d12_metal::metal::queue::has_pending_cbs() {
        d3d12_metal::metal::queue::Queue::flush_all_pending_cbs();
    }
}

/// `Send` wrapper for raw pointers captured by CS jobs.
pub struct SendPtr<T>(pub *const T);
unsafe impl<T> Send for SendPtr<T> {}
impl<T> Clone for SendPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for SendPtr<T> {}

impl<T> SendPtr<T> {
    /// By-value accessor: closures then capture the whole (`Send`) wrapper
    /// rather than the raw-pointer field.
    #[inline(always)]
    pub fn get(self) -> *const T {
        self.0
    }
}
