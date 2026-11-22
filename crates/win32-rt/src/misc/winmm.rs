//! winmm.dll: multimedia timers.

use crate::types::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

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
    *tc = TIMECAPS { period_min: 1, period_max: 1_000_000 };
    TIMERR_NOERROR
}

type TimeCb = unsafe extern "win64" fn(u32, u32, usize, usize, usize);

static NEXT_ID: AtomicU32 = AtomicU32::new(1);
static TIMERS: Mutex<Option<HashMap<u32, Arc<AtomicBool>>>> = Mutex::new(None);

/// `timeSetEvent(delay, resolution, callback_or_event, user, flags)`.
/// Runs on a dedicated Windows thread. Flags: TIME_PERIODIC 1,
/// TIME_CALLBACK_EVENT_SET 0x10, TIME_CALLBACK_EVENT_PULSE 0x20.
pub unsafe extern "win64" fn timeSetEvent(delay: u32, _res: u32, cb: usize, user: usize, flags: u32) -> u32 {
    if delay == 0 || cb == 0 {
        return 0;
    }
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let killed = Arc::new(AtomicBool::new(false));
    TIMERS.lock().unwrap().get_or_insert_with(HashMap::new).insert(id, killed.clone());
    let periodic = flags & 1 != 0;
    let body = Box::new(move || {
        let period = std::time::Duration::from_millis(delay as u64);
        let mut next = std::time::Instant::now() + period;
        loop {
            let now = std::time::Instant::now();
            if next > now {
                std::thread::sleep(next - now);
            }
            if killed.load(Ordering::Acquire) {
                break;
            }
            unsafe {
                if flags & 0x10 != 0 {
                    crate::kernel32::threads::SetEvent(cb as HANDLE);
                } else if flags & 0x20 != 0 {
                    crate::kernel32::threads::SetEvent(cb as HANDLE);
                    crate::kernel32::threads::ResetEvent(cb as HANDLE);
                } else {
                    let f: TimeCb = std::mem::transmute(cb);
                    f(id, 0, user, 0, 0);
                }
            }
            if !periodic {
                break;
            }
            next += period;
        }
        0
    });
    match crate::thread::spawn(body, 256 * 1024, false, false) {
        Ok(_) => id,
        Err(e) => {
            log::error!("timeSetEvent: thread spawn failed: {e}");
            0
        }
    }
}

pub unsafe extern "win64" fn timeKillEvent(id: u32) -> u32 {
    match TIMERS.lock().unwrap().get_or_insert_with(HashMap::new).remove(&id) {
        Some(k) => {
            k.store(true, Ordering::Release);
            TIMERR_NOERROR
        }
        None => 96, // MMSYSERR_INVALPARAM-ish (TIMERR_BASE+? ) -> invalid id
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
