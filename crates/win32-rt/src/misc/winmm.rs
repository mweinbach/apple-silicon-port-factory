//! winmm.dll: multimedia timers.

use crate::types::*;
use std::collections::{BinaryHeap, HashMap};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const TIMERR_NOERROR: u32 = 0;
const TIMERR_NOCANDO: u32 = 97;

#[inline(never)]
pub unsafe extern "win64" fn timeGetTime() -> u32 {
    (crate::kernel32::time::interrupt_time_100ns() / 10_000) as u32
}

/// The host timer is already high resolution; the period is accepted but
/// nothing changes (sleep granularity on macOS is ~sub-millisecond).
pub unsafe extern "win64" fn timeBeginPeriod(p: u32) -> u32 {
    if p == 0 {
        return TIMERR_NOCANDO;
    }
    log::debug!("timeBeginPeriod({p}) (no-op: host timers are high resolution)");
    TIMERR_NOERROR
}
pub unsafe extern "win64" fn timeEndPeriod(p: u32) -> u32 {
    if p == 0 {
        return TIMERR_NOCANDO;
    }
    TIMERR_NOERROR
}

#[repr(C)]
pub struct TIMECAPS {
    period_min: u32,
    period_max: u32,
}

pub unsafe extern "win64" fn timeGetDevCaps(tc: *mut TIMECAPS, size: u32) -> u32 {
    if tc.is_null() || (size as usize) < std::mem::size_of::<TIMECAPS>() {
        return TIMERR_NOCANDO;
    }
    std::ptr::write_unaligned(tc, TIMECAPS { period_min: 1, period_max: 1_000_000 });
    TIMERR_NOERROR
}

struct WinmmTimer {
    cb: usize,
    user: usize,
    flags: u32,
    period: Duration,
}

struct WinmmHeapEntry {
    due: Instant,
    id: u32,
}

impl PartialEq for WinmmHeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.due == other.due && self.id == other.id
    }
}
impl Eq for WinmmHeapEntry {}
impl PartialOrd for WinmmHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for WinmmHeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.due.cmp(&self.due) // min-heap by due time
    }
}

struct WinmmScheduler {
    mu: Mutex<WinmmSchedState>,
    cv: Condvar,
}

struct WinmmSchedState {
    active: HashMap<u32, WinmmTimer>,
    heap: BinaryHeap<WinmmHeapEntry>,
    in_flight: Option<(u32, std::thread::ThreadId)>,
    started: bool,
}

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

fn scheduler() -> &'static WinmmScheduler {
    static S: OnceLock<WinmmScheduler> = OnceLock::new();
    S.get_or_init(|| WinmmScheduler {
        mu: Mutex::new(WinmmSchedState {
            active: HashMap::new(),
            heap: BinaryHeap::new(),
            in_flight: None,
            started: false,
        }),
        cv: Condvar::new(),
    })
}

fn ensure_timer_thread() -> Result<(), std::io::Error> {
    let s = scheduler();
    let mut st = s.mu.lock().unwrap();
    if st.started {
        return Ok(());
    }
    let body = Box::new(move || {
        let s = scheduler();
        loop {
            let fired = {
                let mut st = s.mu.lock().unwrap();
                loop {
                    let now = Instant::now();
                    match st.heap.peek() {
                        None => st = s.cv.wait(st).unwrap(),
                        Some(e) if e.due > now => {
                            let d = e.due - now;
                            st = s.cv.wait_timeout(st, d).unwrap().0;
                        }
                        Some(_) => {
                            let e = st.heap.pop().unwrap();
                            let Some(t) = st.active.get(&e.id) else {
                                continue;
                            };
                            let (cb, user, flags, period) = (t.cb, t.user, t.flags, t.period);
                            let periodic = flags & 1 != 0;
                            if periodic {
                                let next_due = (e.due + period).max(now);
                                st.heap.push(WinmmHeapEntry { due: next_due, id: e.id });
                            } else {
                                st.active.remove(&e.id);
                            }
                            st.in_flight = Some((e.id, std::thread::current().id()));
                            break (e.id, cb, user, flags);
                        }
                    }
                }
            };
            let (id, cb, user, flags) = fired;
            unsafe {
                if flags & 0x10 != 0 {
                    crate::kernel32::threads::SetEvent(cb as HANDLE);
                } else if flags & 0x20 != 0 {
                    crate::kernel32::threads::SetEvent(cb as HANDLE);
                    crate::kernel32::threads::ResetEvent(cb as HANDLE);
                } else {
                    crate::call_guest8(cb, id as u64, 0, user as u64, 0, 0, 0, 0, 0);
                }
            }
            {
                let mut st = s.mu.lock().unwrap();
                st.in_flight = None;
                s.cv.notify_all();
            }
        }
    });
    crate::thread::spawn(body, 256 * 1024, false, false)?;
    st.started = true;
    Ok(())
}

/// `timeSetEvent(delay, resolution, callback_or_event, user, flags)`.
/// Dispatches via a shared Windows timer thread. Flags: TIME_PERIODIC 1,
/// TIME_CALLBACK_EVENT_SET 0x10, TIME_CALLBACK_EVENT_PULSE 0x20.
pub unsafe extern "win64" fn timeSetEvent(delay: u32, _res: u32, cb: usize, user: usize, flags: u32) -> u32 {
    if delay == 0 || cb == 0 {
        return 0;
    }
    if let Err(e) = ensure_timer_thread() {
        log::error!("timeSetEvent: timer thread spawn failed: {e}");
        return 0;
    }
    let mut id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    if id == 0 {
        id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    }
    let period = Duration::from_millis(delay as u64);
    let due = Instant::now() + period;
    let s = scheduler();
    let mut st = s.mu.lock().unwrap();
    st.active.insert(id, WinmmTimer { cb, user, flags, period });
    st.heap.push(WinmmHeapEntry { due, id });
    s.cv.notify_all();
    id
}

pub unsafe extern "win64" fn timeKillEvent(id: u32) -> u32 {
    let s = scheduler();
    let mut st = s.mu.lock().unwrap();
    match st.active.remove(&id) {
        Some(t) => {
            st.heap.retain(|e| e.id != id);
            // TIME_KILL_SYNCHRONOUS (0x100): wait if this timer's callback is currently running
            // on the timer thread (avoiding self-deadlock if called from inside the callback).
            let me = std::thread::current().id();
            if t.flags & 0x100 != 0 {
                while matches!(st.in_flight, Some((fid, tid)) if fid == id && tid != me) {
                    st = s.cv.wait(st).unwrap();
                }
            }
            TIMERR_NOERROR
        }
        None => 96, // MMSYSERR_INVALPARAM -> invalid id
    }
}

const MMSYSERR_NODRIVER: u32 = 6;
const MMSYSERR_INVALHANDLE: u32 = 5;

pub unsafe extern "win64" fn waveOutGetNumDevs() -> u32 {
    0
}

pub unsafe extern "win64" fn waveOutOpen(
    phwo: *mut usize,
    _device_id: u32,
    _pwfx: *const u8,
    _callback: usize,
    _instance: usize,
    _fdw_open: u32,
) -> u32 {
    if !phwo.is_null() {
        *phwo = 0;
    }
    MMSYSERR_NODRIVER
}

pub unsafe extern "win64" fn waveOutClose(_hwo: usize) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutPrepareHeader(_hwo: usize, _pwh: *mut u8, _cbwh: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutUnprepareHeader(_hwo: usize, _pwh: *mut u8, _cbwh: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutWrite(_hwo: usize, _pwh: *mut u8, _cbwh: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutPause(_hwo: usize) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutRestart(_hwo: usize) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutReset(_hwo: usize) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutSetVolume(_hwo: usize, _dw_volume: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutGetVolume(_hwo: usize, _pdw_volume: *mut u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutGetPosition(_hwo: usize, _pmmt: *mut u8, _cbmmt: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveOutGetDevCapsW(_device_id: usize, _pwoc: *mut u8, _cbwoc: u32) -> u32 {
    MMSYSERR_NODRIVER
}

pub unsafe extern "win64" fn waveOutGetDevCapsA(_device_id: usize, _pwoc: *mut u8, _cbwoc: u32) -> u32 {
    MMSYSERR_NODRIVER
}

pub unsafe extern "win64" fn waveOutMessage(_hwo: usize, _msg: u32, _dw1: usize, _dw2: usize) -> u32 {
    MMSYSERR_NODRIVER
}

pub unsafe extern "win64" fn waveInGetNumDevs() -> u32 {
    0
}

pub unsafe extern "win64" fn waveInGetDevCapsA(_device_id: usize, _pwic: *mut u8, _cbwic: u32) -> u32 {
    MMSYSERR_NODRIVER
}

pub unsafe extern "win64" fn waveInOpen(
    phwi: *mut usize,
    _device_id: u32,
    _pwfx: *const u8,
    _callback: usize,
    _instance: usize,
    _fdw_open: u32,
) -> u32 {
    if !phwi.is_null() {
        *phwi = 0;
    }
    MMSYSERR_NODRIVER
}

pub unsafe extern "win64" fn waveInClose(_hwi: usize) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveInPrepareHeader(_hwi: usize, _pwh: *mut u8, _cbwh: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveInUnprepareHeader(_hwi: usize, _pwh: *mut u8, _cbwh: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveInAddBuffer(_hwi: usize, _pwh: *mut u8, _cbwh: u32) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveInStart(_hwi: usize) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveInReset(_hwi: usize) -> u32 {
    MMSYSERR_INVALHANDLE
}

pub unsafe extern "win64" fn waveInMessage(_hwi: usize, _msg: u32, _dw1: usize, _dw2: usize) -> u32 {
    MMSYSERR_NODRIVER
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};

    #[test]
    fn winmm_shared_timer_event_and_callback() {
        unsafe extern "win64" fn timer_cb(_id: u32, _msg: u32, user: usize, _dw1: usize, _dw2: usize) {
            let hits = &*(user as *const AtomicU32);
            hits.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            // 1. One-shot event-set timer.
            let ev = CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
            assert!(!ev.is_null());
            let id1 = timeSetEvent(15, 1, ev as usize, 0, 0x10 /* TIME_CALLBACK_EVENT_SET */);
            assert_ne!(id1, 0);
            assert_eq!(WaitForSingleObject(ev, 5_000), WAIT_OBJECT_0);
            assert_eq!(CloseHandle(ev), TRUE);

            // 2. Periodic callback timer routed through call_guest8 + TIME_KILL_SYNCHRONOUS.
            let hits = AtomicU32::new(0);
            let id2 = timeSetEvent(
                10,
                1,
                timer_cb as *const () as usize,
                &hits as *const AtomicU32 as usize,
                1 | 0x100, /* TIME_PERIODIC | TIME_KILL_SYNCHRONOUS */
            );
            assert_ne!(id2, 0);
            let deadline = Instant::now() + Duration::from_secs(3);
            while hits.load(Ordering::Acquire) < 2 {
                assert!(Instant::now() < deadline, "periodic winmm timer did not fire twice");
                std::thread::sleep(Duration::from_millis(5));
            }
            assert_eq!(timeKillEvent(id2), TIMERR_NOERROR);
            // Killing an already-killed id returns MMSYSERR_INVALPARAM (96).
            assert_eq!(timeKillEvent(id2), 96);
        }
    }
}
