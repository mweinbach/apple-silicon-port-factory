//! winmm.dll: multimedia timers.

use crate::types::*;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const TIMERR_NOERROR: u32 = 0;
const TIMERR_NOCANDO: u32 = 97;
const TIME_PERIOD_MIN_MS: u32 = 1;
const TIME_PERIOD_MAX_MS: u32 = 1_000_000;

/// Active minimum requested multimedia timer period in ms (`0` = none active).
/// Read lock-free by `kernel32::time::sleep_ms` to engage precise `mach_wait_until` waits.
static ACTIVE_MIN_PERIOD_MS: AtomicU32 = AtomicU32::new(0);
static PERIOD_COUNTS: Mutex<BTreeMap<u32, u32>> = Mutex::new(BTreeMap::new());

/// Smallest currently active period requested via `timeBeginPeriod`, if any.
#[inline]
pub fn requested_timer_period_ms() -> Option<u32> {
    let p = ACTIVE_MIN_PERIOD_MS.load(Ordering::Acquire);
    (p > 0).then_some(p)
}

/// Returns `true` when any guest component holds an active `timeBeginPeriod(<= 15 ms)`
/// request (finer than Windows' 15.6 ms default system timer tick).
#[inline]
pub fn high_res_timer_active() -> bool {
    let p = ACTIVE_MIN_PERIOD_MS.load(Ordering::Acquire);
    p > 0 && p <= 15
}

#[inline(never)]
pub unsafe extern "win64" fn timeGetTime() -> u32 {
    (crate::kernel32::time::interrupt_time_100ns() / 10_000) as u32
}

/// Request a minimum timer resolution of `p` milliseconds (`1..=1_000_000`).
/// Tracked with reference counts so `kernel32::time::sleep_ms` engages precise
/// `mach_wait_until` deadlines whenever high-resolution timing (`<= 15 ms`) is active.
pub unsafe extern "win64" fn timeBeginPeriod(p: u32) -> u32 {
    if !(TIME_PERIOD_MIN_MS..=TIME_PERIOD_MAX_MS).contains(&p) {
        return TIMERR_NOCANDO;
    }
    let mut counts = crate::thread::defer_lock(&PERIOD_COUNTS);
    let entry = counts.entry(p).or_insert(0);
    *entry = entry.saturating_add(1);
    let min_p = counts.keys().next().copied().unwrap_or(0);
    ACTIVE_MIN_PERIOD_MS.store(min_p, Ordering::Release);
    log::debug!("timeBeginPeriod({p}) -> active min period = {min_p} ms");
    TIMERR_NOERROR
}

/// Release a previously requested timer resolution of `p` milliseconds.
pub unsafe extern "win64" fn timeEndPeriod(p: u32) -> u32 {
    if !(TIME_PERIOD_MIN_MS..=TIME_PERIOD_MAX_MS).contains(&p) {
        return TIMERR_NOCANDO;
    }
    let mut counts = crate::thread::defer_lock(&PERIOD_COUNTS);
    let Some(entry) = counts.get_mut(&p) else {
        return TIMERR_NOCANDO;
    };
    if *entry <= 1 {
        counts.remove(&p);
    } else {
        *entry -= 1;
    }
    let min_p = counts.keys().next().copied().unwrap_or(0);
    ACTIVE_MIN_PERIOD_MS.store(min_p, Ordering::Release);
    log::debug!("timeEndPeriod({p}) -> active min period = {min_p} ms");
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

// ---------------------------------------------------------------------------
// Multimedia I/O (`mmio*` RIFF/file/memory stream API)
// ---------------------------------------------------------------------------

use std::io::{Read, Seek, SeekFrom};

const MMIO_WRITE: u32 = 0x0000_0001;
const MMIO_READWRITE: u32 = 0x0000_0002;
const MMIO_DELETE: u32 = 0x0000_0200;
const MMIO_CREATE: u32 = 0x0000_1000;
const MMIO_EXIST: u32 = 0x0000_4000;

const MMIO_FINDCHUNK: u32 = 0x0010;
const MMIO_FINDRIFF: u32 = 0x0020;
const MMIO_FINDLIST: u32 = 0x0040;

const FOURCC_RIFF: u32 = u32::from_le_bytes(*b"RIFF");
const FOURCC_LIST: u32 = u32::from_le_bytes(*b"LIST");
const FOURCC_MEM: u32 = u32::from_le_bytes(*b"MEM ");

const MMIOERR_FILENOTFOUND: u32 = 257;
const MMIOERR_CANNOTOPEN: u32 = 259;
const MMIOERR_CANNOTSEEK: u32 = 263;
const MMIOERR_CHUNKNOTFOUND: u32 = 265;

const MMIO_SEEK_SET: i32 = 0;
const MMIO_SEEK_CUR: i32 = 1;
const MMIO_SEEK_END: i32 = 2;

/// Win64 `MMCKINFO` (`mmsystem.h` under `#include <pshpack1.h>`, 20 bytes).
#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
pub struct MmckInfo {
    pub ckid: u32,
    pub cksize: u32,
    pub fcc_type: u32,
    pub dw_data_offset: u32,
    pub dw_flags: u32,
}
const _: () = assert!(std::mem::size_of::<MmckInfo>() == 20);
const _: () = assert!(std::mem::offset_of!(MmckInfo, dw_data_offset) == 12);

/// Win64 `MMIOINFO` (`mmsystem.h` under `#include <pshpack1.h>`, 104 bytes).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MmioInfo {
    pub dw_flags: u32,
    pub fcc_io_proc: u32,
    pub p_io_proc: usize,
    pub w_error_ret: u32,
    pub h_task: usize,
    pub cch_buffer: i32,
    pub pch_buffer: *mut u8,
    pub pch_next: *mut u8,
    pub pch_end_read: *mut u8,
    pub pch_end_write: *mut u8,
    pub l_buf_offset: i32,
    pub l_disk_offset: i32,
    pub adw_info: [u32; 4],
    pub dw_reserved1: u32,
    pub dw_reserved2: u32,
    pub hmmio: usize,
}
const _: () = assert!(std::mem::size_of::<MmioInfo>() == 104);
const _: () = assert!(std::mem::offset_of!(MmioInfo, cch_buffer) == 28);
const _: () = assert!(std::mem::offset_of!(MmioInfo, pch_buffer) == 32);
const _: () = assert!(std::mem::offset_of!(MmioInfo, l_disk_offset) == 68);
const _: () = assert!(std::mem::offset_of!(MmioInfo, hmmio) == 96);

enum MmioBacking {
    File(std::fs::File),
    /// Caller-owned memory buffer (`pchBuffer`, `cchBuffer`), never freed by `mmioClose`.
    ExternalMem { ptr: usize, len: usize, pos: usize },
    /// Internally allocated memory buffer.
    OwnedMem { data: Vec<u8>, pos: usize },
}

impl MmioBacking {
    fn pos(&mut self) -> Result<u64, std::io::Error> {
        match self {
            Self::File(f) => f.stream_position(),
            Self::ExternalMem { pos, .. } => Ok(*pos as u64),
            Self::OwnedMem { pos, .. } => Ok(*pos as u64),
        }
    }

    fn len(&mut self) -> Result<u64, std::io::Error> {
        match self {
            Self::File(f) => Ok(f.metadata()?.len()),
            Self::ExternalMem { len, .. } => Ok(*len as u64),
            Self::OwnedMem { data, .. } => Ok(data.len() as u64),
        }
    }

    fn seek_to(&mut self, target: u64) -> Result<u64, std::io::Error> {
        match self {
            Self::File(f) => f.seek(SeekFrom::Start(target)),
            Self::ExternalMem { len, pos, .. } => {
                if target > *len as u64 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "seek past memory end"));
                }
                *pos = target as usize;
                Ok(target)
            }
            Self::OwnedMem { data, pos } => {
                if target > data.len() as u64 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "seek past memory end"));
                }
                *pos = target as usize;
                Ok(target)
            }
        }
    }

    unsafe fn read_into(&mut self, dst: *mut u8, count: usize) -> Result<usize, std::io::Error> {
        if count == 0 {
            return Ok(0);
        }
        match self {
            Self::File(f) => {
                let mut tmp = vec![0u8; count];
                let mut total = 0usize;
                while total < count {
                    match f.read(&mut tmp[total..]) {
                        Ok(0) => break,
                        Ok(n) => total += n,
                        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(e) => return Err(e),
                    }
                }
                if total > 0 {
                    std::ptr::copy_nonoverlapping(tmp.as_ptr(), dst, total);
                }
                Ok(total)
            }
            Self::ExternalMem { ptr, len, pos } => {
                let avail = len.saturating_sub(*pos);
                let n = count.min(avail);
                if n > 0 {
                    std::ptr::copy_nonoverlapping((*ptr as *const u8).add(*pos), dst, n);
                    *pos += n;
                }
                Ok(n)
            }
            Self::OwnedMem { data, pos } => {
                let avail = data.len().saturating_sub(*pos);
                let n = count.min(avail);
                if n > 0 {
                    std::ptr::copy_nonoverlapping(data.as_ptr().add(*pos), dst, n);
                    *pos += n;
                }
                Ok(n)
            }
        }
    }

    unsafe fn read_exact_bytes<const N: usize>(&mut self) -> Option<[u8; N]> {
        let mut buf = [0u8; N];
        let got = self.read_into(buf.as_mut_ptr(), N).ok()?;
        (got == N).then_some(buf)
    }
}

static NEXT_MMIO: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x004D_4D10);

fn mmio_table() -> &'static Mutex<HashMap<usize, MmioBacking>> {
    static T: OnceLock<Mutex<HashMap<usize, MmioBacking>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn set_mmio_error(pmmioinfo: *mut MmioInfo, err: u32) {
    if !pmmioinfo.is_null() {
        let mut info = pmmioinfo.read_unaligned();
        info.w_error_ret = err;
        pmmioinfo.write_unaligned(info);
    }
}

pub unsafe extern "win64" fn mmioOpenA(
    psz_file_name: LPCSTR,
    pmmioinfo: *mut MmioInfo,
    fdw_open: u32,
) -> usize {
    let info_opt = (!pmmioinfo.is_null()).then(|| pmmioinfo.read_unaligned());
    let is_mem = info_opt.is_some_and(|info| {
        info.fcc_io_proc == FOURCC_MEM || (psz_file_name.is_null() && !info.pch_buffer.is_null())
    });

    if is_mem {
        let info = info_opt.unwrap();
        let len = info.cch_buffer.max(0) as usize;
        let backing = if !info.pch_buffer.is_null() {
            MmioBacking::ExternalMem { ptr: info.pch_buffer as usize, len, pos: 0 }
        } else {
            MmioBacking::OwnedMem { data: vec![0u8; len], pos: 0 }
        };
        let handle = NEXT_MMIO.fetch_add(4, Ordering::Relaxed);
        mmio_table().lock().unwrap().insert(handle, backing);
        if !pmmioinfo.is_null() {
            let mut out = info;
            out.w_error_ret = 0;
            out.hmmio = handle;
            pmmioinfo.write_unaligned(out);
        }
        return handle;
    }

    if psz_file_name.is_null() {
        set_mmio_error(pmmioinfo, MMIOERR_CANNOTOPEN);
        return 0;
    }

    let win_path = crate::strings::from_ansi(psz_file_name);
    let Some(host_path) = crate::path::to_host(&win_path) else {
        set_mmio_error(pmmioinfo, MMIOERR_FILENOTFOUND);
        return 0;
    };

    if (fdw_open & MMIO_EXIST) != 0 {
        return usize::from(host_path.exists());
    }
    if (fdw_open & MMIO_DELETE) != 0 {
        return match std::fs::remove_file(&host_path) {
            Ok(()) => 1,
            Err(_) => {
                set_mmio_error(pmmioinfo, MMIOERR_FILENOTFOUND);
                0
            }
        };
    }

    let mut opts = std::fs::OpenOptions::new();
    let mode = fdw_open & 0x3;
    match mode {
        MMIO_WRITE => {
            opts.write(true);
        }
        MMIO_READWRITE => {
            opts.read(true).write(true);
        }
        _ => {
            opts.read(true);
        }
    }
    if (fdw_open & MMIO_CREATE) != 0 {
        opts.write(true).create(true).truncate(true);
    }

    match opts.open(&host_path) {
        Ok(file) => {
            let handle = NEXT_MMIO.fetch_add(4, Ordering::Relaxed);
            mmio_table().lock().unwrap().insert(handle, MmioBacking::File(file));
            if let Some(mut out) = info_opt {
                out.w_error_ret = 0;
                out.hmmio = handle;
                pmmioinfo.write_unaligned(out);
            }
            handle
        }
        Err(e) => {
            let code = if e.kind() == std::io::ErrorKind::NotFound {
                MMIOERR_FILENOTFOUND
            } else {
                MMIOERR_CANNOTOPEN
            };
            set_mmio_error(pmmioinfo, code);
            0
        }
    }
}

pub unsafe extern "win64" fn mmioDescend(
    hmmio: usize,
    pmmcki: *mut MmckInfo,
    pmmcki_parent: *const MmckInfo,
    fu_descend: u32,
) -> u32 {
    if hmmio == 0 || pmmcki.is_null() {
        return MMSYSERR_INVALHANDLE;
    }
    let want = pmmcki.read_unaligned();
    let want_ckid = want.ckid;
    let want_fcc = want.fcc_type;
    let parent_end: Option<u64> = if pmmcki_parent.is_null() {
        None
    } else {
        let p = pmmcki_parent.read_unaligned();
        Some((p.dw_data_offset as u64).saturating_add(p.cksize as u64))
    };

    let mut table = mmio_table().lock().unwrap();
    let Some(stream) = table.get_mut(&hmmio) else {
        return MMSYSERR_INVALHANDLE;
    };

    let find_flags = fu_descend & (MMIO_FINDCHUNK | MMIO_FINDRIFF | MMIO_FINDLIST);
    loop {
        let Ok(hdr_pos) = stream.pos() else {
            return MMIOERR_CHUNKNOTFOUND;
        };
        if parent_end.is_some_and(|end| hdr_pos.saturating_add(8) > end) {
            return MMIOERR_CHUNKNOTFOUND;
        }
        let Some(hdr) = stream.read_exact_bytes::<8>() else {
            return MMIOERR_CHUNKNOTFOUND;
        };
        let ckid = u32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
        let cksize = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]);
        let data_offset = hdr_pos.saturating_add(8);

        let is_container = ckid == FOURCC_RIFF || ckid == FOURCC_LIST;
        let fcc_type = if is_container {
            if parent_end.is_some_and(|end| hdr_pos.saturating_add(12) > end) {
                return MMIOERR_CHUNKNOTFOUND;
            }
            let Some(fcc_bytes) = stream.read_exact_bytes::<4>() else {
                return MMIOERR_CHUNKNOTFOUND;
            };
            u32::from_le_bytes(fcc_bytes)
        } else {
            0
        };

        let matched = match find_flags {
            0 => true,
            f if (f & MMIO_FINDRIFF) != 0 => {
                ckid == FOURCC_RIFF && (want_fcc == 0 || fcc_type == want_fcc)
            }
            f if (f & MMIO_FINDLIST) != 0 => {
                ckid == FOURCC_LIST && (want_fcc == 0 || fcc_type == want_fcc)
            }
            f if (f & MMIO_FINDCHUNK) != 0 => ckid == want_ckid,
            _ => true,
        };

        if matched {
            pmmcki.write_unaligned(MmckInfo {
                ckid,
                cksize,
                fcc_type,
                dw_data_offset: data_offset as u32,
                dw_flags: 0,
            });
            return 0;
        }

        // Skip over this chunk's payload plus optional word-alignment pad byte.
        let next_pos = data_offset
            .saturating_add(cksize as u64)
            .saturating_add((cksize & 1) as u64);
        if parent_end.is_some_and(|end| next_pos > end) {
            return MMIOERR_CHUNKNOTFOUND;
        }
        if stream.seek_to(next_pos).is_err() {
            return MMIOERR_CHUNKNOTFOUND;
        }
    }
}

pub unsafe extern "win64" fn mmioRead(hmmio: usize, pch: *mut u8, cch: i32) -> i32 {
    if hmmio == 0 || pch.is_null() || cch < 0 {
        return -1;
    }
    if cch == 0 {
        return 0;
    }
    let mut table = mmio_table().lock().unwrap();
    let Some(stream) = table.get_mut(&hmmio) else {
        return -1;
    };
    match stream.read_into(pch, cch as usize) {
        Ok(n) => n as i32,
        Err(_) => -1,
    }
}

pub unsafe extern "win64" fn mmioAscend(
    hmmio: usize,
    pmmcki: *mut MmckInfo,
    _fu_ascend: u32,
) -> u32 {
    if hmmio == 0 || pmmcki.is_null() {
        return MMSYSERR_INVALHANDLE;
    }
    let ck = pmmcki.read_unaligned();
    let end_pos = (ck.dw_data_offset as u64)
        .saturating_add(ck.cksize as u64)
        .saturating_add((ck.cksize & 1) as u64);
    let mut table = mmio_table().lock().unwrap();
    let Some(stream) = table.get_mut(&hmmio) else {
        return MMSYSERR_INVALHANDLE;
    };
    if stream.seek_to(end_pos).is_err() {
        return MMIOERR_CANNOTSEEK;
    }
    0
}

pub unsafe extern "win64" fn mmioSeek(hmmio: usize, l_offset: i32, i_origin: i32) -> i32 {
    if hmmio == 0 {
        return -1;
    }
    let mut table = mmio_table().lock().unwrap();
    let Some(stream) = table.get_mut(&hmmio) else {
        return -1;
    };
    let base: i64 = match i_origin {
        MMIO_SEEK_SET => 0,
        MMIO_SEEK_CUR => match stream.pos() {
            Ok(p) => p as i64,
            Err(_) => return -1,
        },
        MMIO_SEEK_END => match stream.len() {
            Ok(l) => l as i64,
            Err(_) => return -1,
        },
        _ => return -1,
    };
    let target = base + (l_offset as i64);
    if target < 0 {
        return -1;
    }
    match stream.seek_to(target as u64) {
        Ok(pos) => pos as i32,
        Err(_) => -1,
    }
}

pub unsafe extern "win64" fn mmioClose(hmmio: usize, _fu_close: u32) -> u32 {
    if hmmio == 0 {
        return MMSYSERR_INVALHANDLE;
    }
    if mmio_table().lock().unwrap().remove(&hmmio).is_some() {
        0
    } else {
        MMSYSERR_INVALHANDLE
    }
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

    #[test]
    fn time_begin_and_end_period_track_min_period_and_precise_sleep() {
        unsafe {
            assert_eq!(timeBeginPeriod(0), TIMERR_NOCANDO);
            assert_eq!(timeBeginPeriod(1_000_001), TIMERR_NOCANDO);
            assert_eq!(timeEndPeriod(0), TIMERR_NOCANDO);
            assert_eq!(timeEndPeriod(1), TIMERR_NOCANDO);
            assert_eq!(requested_timer_period_ms(), None);
            assert!(!high_res_timer_active());

            assert_eq!(timeBeginPeriod(5), TIMERR_NOERROR);
            assert_eq!(requested_timer_period_ms(), Some(5));
            assert!(high_res_timer_active());
            if std::env::var_os("PORT_PRECISE_SLEEP").is_none()
                && std::env::var_os("AW2_PRECISE_SLEEP").is_none()
            {
                assert!(crate::kernel32::time::should_use_precise_sleep(1));
                assert!(crate::kernel32::time::should_use_precise_sleep(15));
                assert!(!crate::kernel32::time::should_use_precise_sleep(16));
                assert!(!crate::kernel32::time::should_use_precise_sleep(0));
            }

            // Finer period lowers the active minimum; nested refs must unwind cleanly.
            assert_eq!(timeBeginPeriod(1), TIMERR_NOERROR);
            assert_eq!(timeBeginPeriod(1), TIMERR_NOERROR);
            assert_eq!(requested_timer_period_ms(), Some(1));
            assert_eq!(timeEndPeriod(1), TIMERR_NOERROR);
            assert_eq!(requested_timer_period_ms(), Some(1));
            assert_eq!(timeEndPeriod(1), TIMERR_NOERROR);
            assert_eq!(requested_timer_period_ms(), Some(5));

            // Coarser period (>15ms) does not engage high-res sleep once finer periods end.
            assert_eq!(timeBeginPeriod(20), TIMERR_NOERROR);
            assert_eq!(timeEndPeriod(5), TIMERR_NOERROR);
            assert_eq!(requested_timer_period_ms(), Some(20));
            assert!(!high_res_timer_active());

            assert_eq!(timeEndPeriod(20), TIMERR_NOERROR);
            assert_eq!(requested_timer_period_ms(), None);
            assert!(!high_res_timer_active());
            assert_eq!(timeEndPeriod(20), TIMERR_NOCANDO);
        }
    }

    fn build_sample_riff_wave() -> Vec<u8> {
        // Construct RIFF('WAVE', chunk('JUNK', 3 bytes + 1 pad), chunk('fmt ', 4 bytes), chunk('data', 5 bytes + 1 pad))
        let mut body = Vec::new();
        body.extend_from_slice(b"WAVE");
        body.extend_from_slice(b"JUNK");
        body.extend_from_slice(&3u32.to_le_bytes());
        body.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0x00]); // odd size 3 + 1 pad byte
        body.extend_from_slice(b"fmt ");
        body.extend_from_slice(&4u32.to_le_bytes());
        body.extend_from_slice(&[1, 0, 2, 0]);
        body.extend_from_slice(b"data");
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&[10, 20, 30, 40, 50, 0]);

        let mut riff = Vec::new();
        riff.extend_from_slice(b"RIFF");
        riff.extend_from_slice(&(body.len() as u32).to_le_bytes());
        riff.extend_from_slice(&body);
        riff
    }

    #[test]
    fn mmio_memory_and_file_riff_parsing_with_odd_chunk_padding() {
        let mut wav = build_sample_riff_wave();
        let fcc_wave = u32::from_le_bytes(*b"WAVE");
        let fcc_fmt = u32::from_le_bytes(*b"fmt ");
        let fcc_data = u32::from_le_bytes(*b"data");

        unsafe {
            // 1. In-memory FOURCC_MEM stream.
            let mut info: MmioInfo = std::mem::zeroed();
            info.fcc_io_proc = FOURCC_MEM;
            info.pch_buffer = wav.as_mut_ptr();
            info.cch_buffer = wav.len() as i32;
            let h = mmioOpenA(std::ptr::null(), &mut info, 0);
            assert_ne!(h, 0);

            let mut parent = MmckInfo { fcc_type: fcc_wave, ..Default::default() };
            assert_eq!(mmioDescend(h, &mut parent, std::ptr::null(), MMIO_FINDRIFF), 0);
            let p_ckid = parent.ckid;
            let p_fcc = parent.fcc_type;
            let p_off = parent.dw_data_offset;
            assert_eq!(p_ckid, FOURCC_RIFF);
            assert_eq!(p_fcc, fcc_wave);
            assert_eq!(p_off, 8);

            // Skipping odd-sized JUNK chunk must respect the 1-byte word-alignment pad.
            let mut sub = MmckInfo { ckid: fcc_fmt, ..Default::default() };
            assert_eq!(mmioDescend(h, &mut sub, &parent, MMIO_FINDCHUNK), 0);
            let fmt_size = sub.cksize;
            assert_eq!(fmt_size, 4);
            let mut fmt_buf = [0u8; 4];
            assert_eq!(mmioRead(h, fmt_buf.as_mut_ptr(), 4), 4);
            assert_eq!(fmt_buf, [1, 0, 2, 0]);
            assert_eq!(mmioAscend(h, &mut sub, 0), 0);

            // Descend into 'data', read, seek back, re-read.
            let mut data_ck = MmckInfo { ckid: fcc_data, ..Default::default() };
            assert_eq!(mmioDescend(h, &mut data_ck, &parent, MMIO_FINDCHUNK), 0);
            let data_size = data_ck.cksize;
            let data_off = data_ck.dw_data_offset;
            assert_eq!(data_size, 5);
            let mut data_buf = [0u8; 5];
            assert_eq!(mmioRead(h, data_buf.as_mut_ptr(), 5), 5);
            assert_eq!(data_buf, [10, 20, 30, 40, 50]);
            assert_eq!(mmioSeek(h, data_off as i32 + 2, MMIO_SEEK_SET), data_off as i32 + 2);
            let mut b1 = [0u8; 2];
            assert_eq!(mmioRead(h, b1.as_mut_ptr(), 2), 2);
            assert_eq!(b1, [30, 40]);
            assert_eq!(mmioAscend(h, &mut data_ck, 0), 0);
            assert_eq!(mmioSeek(h, 0, MMIO_SEEK_CUR), wav.len() as i32);
            assert_eq!(mmioClose(h, 0), 0);
            assert_eq!(mmioClose(h, 0), MMSYSERR_INVALHANDLE);

            // 2. File-backed stream via virtual Windows mount.
            let tmp_dir = std::env::temp_dir().join(format!("winmm_mmio_test_{}", std::process::id()));
            std::fs::create_dir_all(&tmp_dir).unwrap();
            let tmp_file = tmp_dir.join("sample.wav");
            std::fs::write(&tmp_file, &wav).unwrap();
            crate::path::add_mount("M:\\MmioTest", &tmp_dir);
            let cpath = std::ffi::CString::new("M:\\MmioTest\\sample.wav").unwrap();
            assert_eq!(mmioOpenA(cpath.as_ptr().cast(), std::ptr::null_mut(), MMIO_EXIST), 1);
            let hf = mmioOpenA(cpath.as_ptr().cast(), std::ptr::null_mut(), 0);
            assert_ne!(hf, 0);
            let mut fp = MmckInfo { fcc_type: fcc_wave, ..Default::default() };
            assert_eq!(mmioDescend(hf, &mut fp, std::ptr::null(), MMIO_FINDRIFF), 0);
            let mut fd = MmckInfo { ckid: fcc_data, ..Default::default() };
            assert_eq!(mmioDescend(hf, &mut fd, &fp, MMIO_FINDCHUNK), 0);
            let mut fbuf = [0u8; 5];
            assert_eq!(mmioRead(hf, fbuf.as_mut_ptr(), 5), 5);
            assert_eq!(fbuf, [10, 20, 30, 40, 50]);
            assert_eq!(mmioClose(hf, 0), 0);
            let _ = std::fs::remove_dir_all(&tmp_dir);
        }
    }
}
