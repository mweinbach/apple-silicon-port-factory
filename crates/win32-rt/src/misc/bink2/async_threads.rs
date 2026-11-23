//! Global configuration, utility allocator, reentrant utility mutexes,
//! error buffer, timer, and async thread pool coordination for `bink2w64.dll`.

use crate::handles::HandleTable;
use crate::strings::read_ansi_bytes_z;
use crate::sync::ReentrantLock;
use std::collections::HashSet;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

static MEM_ALLOC: AtomicUsize = AtomicUsize::new(0);
static MEM_FREE: AtomicUsize = AtomicUsize::new(0);

pub(crate) static OS_OPEN: AtomicUsize = AtomicUsize::new(0);
pub(crate) static OS_READ: AtomicUsize = AtomicUsize::new(0);
pub(crate) static OS_SEEK: AtomicUsize = AtomicUsize::new(0);
pub(crate) static OS_CLOSE: AtomicUsize = AtomicUsize::new(0);

static SOUND_OPEN: AtomicUsize = AtomicUsize::new(0);
static SOUND_PARAM1: AtomicU64 = AtomicU64::new(0);
static SOUND_PARAM2: AtomicU64 = AtomicU64::new(0);
static SOUND_PARAM3: AtomicU64 = AtomicU64::new(0);
static SOUND_PARAM4: AtomicU64 = AtomicU64::new(0);
pub(crate) static SOUND_DISABLED: AtomicBool = AtomicBool::new(false);
pub(crate) static SOUND_TRACK_SELECTION: Mutex<Option<Vec<u32>>> = Mutex::new(None);

// Pre-open global options set via `BinkSetFileOffset`, `BinkSetFrameRate`, `BinkSetIO`,
// `BinkSetIOSize`, and `BinkSetSimulate` (honored by `BinkOpen` / `BinkOpenWithOptions`).
pub(crate) static PREOPEN_FILE_OFFSET: AtomicU64 = AtomicU64::new(0);
pub(crate) static PREOPEN_FRAME_RATE_NUM: AtomicU32 = AtomicU32::new(0);
pub(crate) static PREOPEN_FRAME_RATE_DEN: AtomicU32 = AtomicU32::new(0);
pub(crate) static PREOPEN_USER_IO: AtomicUsize = AtomicUsize::new(0);
pub(crate) static PREOPEN_IO_SIZE: AtomicU32 = AtomicU32::new(0);
pub(crate) static PREOPEN_SIMULATE: AtomicU32 = AtomicU32::new(0);

// Static NUL-terminated error message buffer for `BinkGetError` / `BinkSetError`.
static LAST_ERROR_LOCK: Mutex<()> = Mutex::new(());
static LAST_ERROR: [AtomicU8; 512] = [const { AtomicU8::new(0) }; 512];

// Monotonic millisecond clock epoch for `RADTimerRead`.
static RAD_TIMER_EPOCH: OnceLock<Instant> = OnceLock::new();

// Track host-allocated `BinkUtilMalloc` blocks so `BinkUtilFree` always matches the allocator
// even if `BinkSetMemory` is changed between alloc and free.
static HOST_UTIL_ALLOCS: OnceLock<Mutex<HashSet<usize>>> = OnceLock::new();

fn host_util_allocs() -> &'static Mutex<HashSet<usize>> {
    HOST_UTIL_ALLOCS.get_or_init(|| Mutex::new(HashSet::new()))
}

// Reentrant utility mutex table and global sound mutex (backed by shared `crate::sync::ReentrantLock`).
static UTIL_MUTEX_TABLE: HandleTable<Arc<ReentrantLock>> = HandleTable::new(0xB140_0004, 4);
static SOUND_GLOBAL_MUTEX: ReentrantLock = ReentrantLock::new();

pub(crate) const MAX_ASYNC_THREADS: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum AsyncThreadState {
    Stopped,
    Running,
    StopRequested,
}

pub(crate) struct AsyncThreadPool {
    pub(crate) states: [AsyncThreadState; MAX_ASYNC_THREADS],
    pub(crate) in_flight: [u32; MAX_ASYNC_THREADS],
}

impl AsyncThreadPool {
    const fn new() -> Self {
        Self {
            states: [AsyncThreadState::Stopped; MAX_ASYNC_THREADS],
            in_flight: [0; MAX_ASYNC_THREADS],
        }
    }
}

static ASYNC_POOL: OnceLock<(Mutex<AsyncThreadPool>, Condvar)> = OnceLock::new();

pub(crate) fn async_pool() -> &'static (Mutex<AsyncThreadPool>, Condvar) {
    ASYNC_POOL.get_or_init(|| (Mutex::new(AsyncThreadPool::new()), Condvar::new()))
}

pub(crate) struct AsyncDecodeGuard {
    slots: Vec<usize>,
}

impl AsyncDecodeGuard {
    pub(crate) fn acquire(thread_ids: &[u32]) -> Self {
        let mut slots = Vec::with_capacity(thread_ids.len());
        let (lock, _) = async_pool();
        let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
        for &tid in thread_ids {
            let idx = tid as usize;
            if idx < MAX_ASYNC_THREADS && !slots.contains(&idx) {
                pool.in_flight[idx] = pool.in_flight[idx].saturating_add(1);
                slots.push(idx);
            }
        }
        Self { slots }
    }
}

impl Drop for AsyncDecodeGuard {
    fn drop(&mut self) {
        if self.slots.is_empty() {
            return;
        }
        let (lock, cvar) = async_pool();
        let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
        for &idx in &self.slots {
            pool.in_flight[idx] = pool.in_flight[idx].saturating_sub(1);
        }
        cvar.notify_all();
    }
}

// ============================================================================
// Global Configuration & Error Helpers
// ============================================================================

pub(crate) fn set_memory(alloc_fn: usize, free_fn: usize) {
    MEM_ALLOC.store(alloc_fn, Ordering::Release);
    MEM_FREE.store(free_fn, Ordering::Release);
}

pub(crate) fn set_os_file_callbacks(
    open_fn: usize,
    read_fn: usize,
    seek_fn: usize,
    close_fn: usize,
) {
    OS_OPEN.store(open_fn, Ordering::Release);
    OS_READ.store(read_fn, Ordering::Release);
    OS_SEEK.store(seek_fn, Ordering::Release);
    OS_CLOSE.store(close_fn, Ordering::Release);
}

pub(crate) fn mark_sound_enabled() {
    SOUND_DISABLED.store(false, Ordering::Release);
}

pub(crate) fn set_sound_system4(
    open_fn: usize,
    param1: u64,
    param2: u64,
    param3: u64,
    param4: u64,
) -> i32 {
    SOUND_OPEN.store(open_fn, Ordering::Release);
    SOUND_PARAM1.store(param1, Ordering::Release);
    SOUND_PARAM2.store(param2, Ordering::Release);
    SOUND_PARAM3.store(param3, Ordering::Release);
    SOUND_PARAM4.store(param4, Ordering::Release);
    SOUND_DISABLED.store(open_fn == 0, Ordering::Release);
    1
}

pub(crate) unsafe fn set_sound_track(total_tracks: u32, tracks: *const u32) {
    let mut ids = Vec::new();
    if total_tracks > 0 && !tracks.is_null() {
        let count = (total_tracks as usize).min(32);
        ids.reserve(count);
        for i in 0..count {
            ids.push(std::ptr::read_unaligned(tracks.add(i)));
        }
    }
    *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner()) = Some(ids);
}

pub(crate) fn set_bink_error(msg: &str) {
    let _guard = LAST_ERROR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let bytes = msg.as_bytes();
    let max_copy = bytes.len().min(LAST_ERROR.len() - 1);
    for (i, &b) in bytes[..max_copy].iter().enumerate() {
        LAST_ERROR[i].store(if b == 0 { b' ' } else { b }, Ordering::Release);
    }
    LAST_ERROR[max_copy].store(0, Ordering::Release);
}

pub(crate) unsafe fn set_bink_error_ptr(err: *const u8) {
    let _guard = LAST_ERROR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if err.is_null() {
        LAST_ERROR[0].store(0, Ordering::Release);
        return;
    }
    let bytes = read_ansi_bytes_z(err, LAST_ERROR.len() - 1);
    for (i, &b) in bytes.iter().enumerate() {
        LAST_ERROR[i].store(b, Ordering::Release);
    }
    LAST_ERROR[bytes.len()].store(0, Ordering::Release);
}

pub(crate) fn get_bink_error_ptr() -> *const u8 {
    LAST_ERROR.as_ptr() as *const u8
}

pub(crate) fn rad_timer_read() -> u32 {
    let epoch = RAD_TIMER_EPOCH.get_or_init(Instant::now);
    epoch.elapsed().as_millis() as u32
}

pub(crate) unsafe fn free_globals() {
    let _ = wait_stop_async_threads_multi(std::ptr::null(), 0);
    PREOPEN_FILE_OFFSET.store(0, Ordering::Release);
    PREOPEN_FRAME_RATE_NUM.store(0, Ordering::Release);
    PREOPEN_FRAME_RATE_DEN.store(0, Ordering::Release);
    PREOPEN_USER_IO.store(0, Ordering::Release);
    PREOPEN_IO_SIZE.store(0, Ordering::Release);
    PREOPEN_SIMULATE.store(0, Ordering::Release);
    *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner()) = None;
    set_bink_error("");
}

pub(crate) fn util_cpus() -> i32 {
    std::thread::available_parallelism()
        .map(|n| n.get().clamp(1, 256) as i32)
        .unwrap_or(4)
}

// ============================================================================
// Utility Memory Allocator & Reentrant Mutex API
// ============================================================================

pub(crate) unsafe fn util_malloc(bytes: u64) -> *mut c_void {
    let alloc_fn = MEM_ALLOC.load(Ordering::Acquire);
    if alloc_fn != 0 {
        return crate::call_guest8(alloc_fn, bytes, 0, 0, 0, 0, 0, 0, 0) as *mut c_void;
    }
    let size = (bytes as usize).max(1);
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let rc = libc::posix_memalign(&mut ptr, 32, size);
    if rc != 0 || ptr.is_null() {
        return std::ptr::null_mut();
    }
    host_util_allocs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(ptr as usize);
    ptr
}

pub(crate) unsafe fn util_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let is_host = host_util_allocs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&(ptr as usize));
    if is_host {
        libc::free(ptr);
        return;
    }
    let free_fn = MEM_FREE.load(Ordering::Acquire);
    if free_fn != 0 {
        crate::call_guest8(free_fn, ptr as u64, 0, 0, 0, 0, 0, 0, 0);
    } else {
        libc::free(ptr);
    }
}

pub(crate) fn util_mutex_create() -> *mut c_void {
    UTIL_MUTEX_TABLE.insert(Arc::new(ReentrantLock::new())) as *mut c_void
}

pub(crate) fn util_mutex_destroy(mutex: *mut c_void) {
    if mutex.is_null() {
        return;
    }
    let _ = UTIL_MUTEX_TABLE.remove(mutex as usize);
}

pub(crate) fn util_mutex_lock(mutex: *mut c_void) {
    if mutex.is_null() {
        return;
    }
    if let Some(m) = UTIL_MUTEX_TABLE.get_cloned(mutex as usize) {
        m.lock();
    }
}

pub(crate) fn util_mutex_lock_timeout(mutex: *mut c_void, ms: i32) -> i32 {
    if mutex.is_null() {
        return 0;
    }
    let Some(m) = UTIL_MUTEX_TABLE.get_cloned(mutex as usize) else {
        return 0;
    };
    if ms < 0 {
        m.lock();
        1
    } else if ms == 0 {
        i32::from(m.try_lock())
    } else {
        i32::from(m.lock_timeout(Duration::from_millis(ms as u64)))
    }
}

pub(crate) fn util_mutex_unlock(mutex: *mut c_void) {
    if mutex.is_null() {
        return;
    }
    if let Some(m) = UTIL_MUTEX_TABLE.get_cloned(mutex as usize) {
        let _ = m.unlock();
    }
}

pub(crate) fn sound_global_lock() {
    SOUND_GLOBAL_MUTEX.lock();
}

pub(crate) fn sound_global_unlock() {
    let _ = SOUND_GLOBAL_MUTEX.unlock();
}

// ============================================================================
// Async Thread Pool Lifecycle & Synchronization
// ============================================================================

pub(crate) fn start_async_thread(thread_num: i32) -> i32 {
    if thread_num < 0 || (thread_num as usize) >= MAX_ASYNC_THREADS {
        return 0;
    }
    let (lock, _) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    pool.states[thread_num as usize] = AsyncThreadState::Running;
    1
}

pub(crate) fn request_stop_async_thread(thread_num: i32) -> i32 {
    if thread_num < 0 || (thread_num as usize) >= MAX_ASYNC_THREADS {
        return 0;
    }
    let (lock, _) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    let slot = &mut pool.states[thread_num as usize];
    if *slot == AsyncThreadState::Running {
        *slot = AsyncThreadState::StopRequested;
    }
    1
}

pub(crate) fn wait_stop_async_thread(thread_num: i32) -> i32 {
    if thread_num < 0 || (thread_num as usize) >= MAX_ASYNC_THREADS {
        return 0;
    }
    let idx = thread_num as usize;
    let (lock, cvar) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    while pool.in_flight[idx] > 0 {
        pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
    }
    pool.states[idx] = AsyncThreadState::Stopped;
    1
}

pub(crate) unsafe fn request_stop_async_threads_multi(
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    let (lock, _) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    if threads.is_null() {
        if thread_count > 0 {
            return 0;
        }
        for slot in &mut pool.states {
            if *slot == AsyncThreadState::Running {
                *slot = AsyncThreadState::StopRequested;
            }
        }
        return 1;
    }
    if thread_count <= 0 {
        return 1;
    }
    let count = (thread_count as usize).min(MAX_ASYNC_THREADS);
    for i in 0..count {
        let tid = std::ptr::read_unaligned(threads.add(i)) as usize;
        if tid < MAX_ASYNC_THREADS && pool.states[tid] == AsyncThreadState::Running {
            pool.states[tid] = AsyncThreadState::StopRequested;
        }
    }
    1
}

pub(crate) unsafe fn wait_stop_async_threads_multi(
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    let (lock, cvar) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    if threads.is_null() {
        if thread_count > 0 {
            return 0;
        }
        while pool.in_flight.iter().any(|&n| n > 0) {
            pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
        }
        for slot in &mut pool.states {
            *slot = AsyncThreadState::Stopped;
        }
        return 1;
    }
    if thread_count <= 0 {
        return 1;
    }
    let count = (thread_count as usize).min(MAX_ASYNC_THREADS);
    let mut targets = Vec::with_capacity(count);
    for i in 0..count {
        let tid = std::ptr::read_unaligned(threads.add(i)) as usize;
        if tid < MAX_ASYNC_THREADS && !targets.contains(&tid) {
            targets.push(tid);
        }
    }
    while targets.iter().any(|&tid| pool.in_flight[tid] > 0) {
        pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
    }
    for tid in targets {
        pool.states[tid] = AsyncThreadState::Stopped;
    }
    1
}

pub(crate) fn wait_async_decode(bink: *mut c_void, us: i32) -> i32 {
    if bink.is_null() {
        return 0;
    }
    let (lock, cvar) = async_pool();
    let mut pool = lock.lock().unwrap_or_else(|e| e.into_inner());
    if !pool.in_flight.iter().any(|&n| n > 0) {
        return 1;
    }
    if us == 0 {
        return 0;
    }
    if us < 0 {
        while pool.in_flight.iter().any(|&n| n > 0) {
            pool = cvar.wait(pool).unwrap_or_else(|e| e.into_inner());
        }
        return 1;
    }
    let deadline = Instant::now() + Duration::from_micros(us as u64);
    while pool.in_flight.iter().any(|&n| n > 0) {
        let Some(rem) = deadline.checked_duration_since(Instant::now()) else {
            return 0;
        };
        let (next_pool, res) = cvar
            .wait_timeout(pool, rem)
            .unwrap_or_else(|e| e.into_inner());
        pool = next_pool;
        if !pool.in_flight.iter().any(|&n| n > 0) {
            return 1;
        }
        if res.timed_out() {
            return 0;
        }
    }
    1
}
