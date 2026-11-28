//! hid.dll and the HID device layer (`\\?\hid#...` device handles).
//!
//! Devices come from a platform backend (macOS: IOHIDManager in
//! `macos::hidmgr`, game controllers only) and are listed by setupapi under
//! `GUID_DEVINTERFACE_HID`. Opening a device path with `CreateFileW` gives a
//! [`HidHandle`] kernel object; kernel32's `ReadFile`/`WriteFile`/`CancelIo[Ex]`
//! dispatch to [`read_file`]/[`write_file`]/[`cancel_io`] for it.
//!
//! Semantics follow hidclass.sys:
//! * every handle has its own ring of the 32 most recent input reports
//!   (the default `HidD_SetNumInputBuffers` size); `ReadFile` returns one
//!   report of exactly `InputReportByteLength` bytes, report ID first (0 when
//!   the device has no report IDs);
//! * an overlapped `ReadFile` with a queued report completes synchronously
//!   (TRUE, OVERLAPPED filled, event set); otherwise it returns
//!   `ERROR_IO_PENDING` with `Internal = STATUS_PENDING` and is completed by
//!   the next input report, `CancelIo[Ex]` (`STATUS_CANCELLED`), device
//!   removal (`STATUS_DEVICE_NOT_CONNECTED`) or closing the handle;
//! * an overlapped `WriteFile` (output report) is queued to the backend and
//!   completes asynchronously the same way;
//! * `HidD_GetFeature`/`HidD_SetFeature` are synchronous.
//!
//! Completions may run on a native backend thread: they write the guest
//! buffer, `InternalHigh`, then `Internal` (release), then set the event.
//! The guest buffer stays owned by the request until it is completed or
//! cancelled, as on Windows.
//!
//! Preparsed data is built from the device's real report descriptor
//! ([`parse`]), so `HidP_GetCaps`/`HidP_GetValueCaps` give what Windows
//! would for the same descriptor.

pub mod parse;

use crate::handles::{self, Event, Guarded, KObject, WaitGuard, Waitable};
use crate::kernel32::file::OVERLAPPED;
use crate::types::*;
use parse::{Caps, HIDP_FEATURE, HIDP_INPUT, HIDP_OUTPUT};
use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, Weak};
use std::time::Duration;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const HIDP_STATUS_SUCCESS: NTSTATUS = 0x0011_0000;
const HIDP_STATUS_INVALID_REPORT_TYPE: NTSTATUS = 0xC011_0002u32 as i32;
const HIDP_STATUS_INVALID_PREPARSED_DATA: NTSTATUS = 0xC011_0001u32 as i32;
const HIDP_STATUS_BUFFER_TOO_SMALL: NTSTATUS = 0xC011_0007u32 as i32;
const HIDP_STATUS_USAGE_NOT_FOUND: NTSTATUS = 0xC011_0004u32 as i32;

pub const STATUS_SUCCESS: usize = 0;
pub const STATUS_PENDING: usize = 0x103;
pub const STATUS_CANCELLED: usize = 0xC000_0120;
pub const STATUS_DEVICE_NOT_CONNECTED: usize = 0xC000_009D;
pub const STATUS_IO_DEVICE_ERROR: usize = 0xC000_0185;

const ERROR_INVALID_USER_BUFFER: u32 = 1784;
const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;
const ERROR_GEN_FAILURE: u32 = 31;
const ERROR_SEM_TIMEOUT: u32 = 121;
const ERROR_IO_DEVICE: u32 = 1117;

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const GENERIC_ALL: u32 = 0x1000_0000;
const FILE_READ_DATA: u32 = 1;
const FILE_WRITE_DATA: u32 = 2;
const FILE_FLAG_OVERLAPPED: u32 = 0x4000_0000;

/// Input reports buffered per handle (hidclass default).
const INPUT_BUFFERS: usize = 32;
/// Upper bound for synchronous feature / non-overlapped requests.
const SYNC_TIMEOUT: Duration = Duration::from_secs(5);

/// GUID_DEVINTERFACE_HID {4D1E55B2-F16F-11CF-88CB-001111000030}.
pub const GUID_DEVINTERFACE_HID: [u8; 16] = [0xB2, 0x55, 0x1E, 0x4D, 0x6F, 0xF1, 0xCF, 0x11, 0x88, 0xCB, 0x00, 0x11, 0x11, 0x00, 0x00, 0x30];

// ---------------------------------------------------------------------------
// Backend interface and device registry
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportType {
    Input = 0,
    Output = 1,
    Feature = 2,
}

/// Completion callback: Ok(bytes) or Err(NTSTATUS).
pub type GetDone = Box<dyn FnOnce(Result<Vec<u8>, usize>) + Send>;
pub type SetDone = Box<dyn FnOnce(Result<usize, usize>) + Send>;

/// A platform HID transport. Callbacks run exactly once, on any thread.
pub trait Backend: Send + Sync {
    /// Read a report. `id` is the report ID (0 = unnumbered); the result may
    /// or may not start with the report ID ([`windows_layout`] decides).
    fn get_report(&self, dev: u64, ty: ReportType, id: u8, len: usize, done: GetDone);
    /// Send a report given in Windows layout (`data[0]` = report ID).
    fn set_report(&self, dev: u64, ty: ReportType, data: Vec<u8>, done: SetDone);
}

/// What a backend knows about a device.
#[derive(Clone, Debug, Default)]
pub struct DeviceDesc {
    /// Backend-unique, stable while connected (macOS: registry entry ID).
    pub id: u64,
    pub vendor_id: u16,
    pub product_id: u16,
    pub version: u16,
    pub bluetooth: bool,
    /// Port / location, used for the instance path and container ID.
    pub location: u32,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial: Option<String>,
    pub report_descriptor: Vec<u8>,
}

pub struct Device {
    pub desc: DeviceDesc,
    pub caps: Arc<Caps>,
    /// `\\?\hid#...#{4d1e55b2-...}` (lowercase).
    pub path: String,
    /// Device instance number for setupapi (`DevInst`).
    pub inst: u32,
    connected: AtomicBool,
    readers: Mutex<Vec<Weak<HidHandle>>>,
}

impl Device {
    pub fn connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    /// `{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}` stable for this device and
    /// port (SPDRP_BASE_CONTAINERID).
    pub fn container_id(&self) -> String {
        let d = &self.desc;
        let a = 0xa2c0_0000u32 ^ d.location;
        format!(
            "{{{:08x}-{:04x}-{:04x}-{:04x}-{:04x}{:08x}}}",
            a,
            d.vendor_id,
            d.product_id,
            0x8000 | (d.bluetooth as u16) << 12 | (d.version & 0x0fff),
            (d.id >> 32) as u16,
            d.id as u32
        )
    }
}

struct Registry {
    devices: Vec<Arc<Device>>,
    next_inst: u32,
}

fn registry() -> &'static Mutex<Registry> {
    static R: OnceLock<Mutex<Registry>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(Registry { devices: Vec::new(), next_inst: 1 }))
}

static BACKEND: Mutex<Option<Arc<dyn Backend>>> = Mutex::new(None);

pub fn set_backend(b: Arc<dyn Backend>) {
    *BACKEND.lock().unwrap() = Some(b);
}

fn backend() -> Option<Arc<dyn Backend>> {
    BACKEND.lock().unwrap().clone()
}

fn make_path(d: &DeviceDesc) -> String {
    const IFACE: &str = "{4d1e55b2-f16f-11cf-88cb-001111000030}";
    if d.bluetooth {
        format!(
            "\\\\?\\hid#{{00001124-0000-1000-8000-00805f9b34fb}}_vid&0002{:04x}_pid&{:04x}#9&{:08x}&0&0000#{IFACE}",
            d.vendor_id, d.product_id, d.id as u32 ^ d.location
        )
    } else {
        format!("\\\\?\\hid#vid_{:04x}&pid_{:04x}#8&{:08x}&0&0000#{IFACE}", d.vendor_id, d.product_id, d.id as u32 ^ d.location)
    }
}

/// Backend: a device appeared. Returns false if its descriptor is unusable.
pub fn device_added(desc: DeviceDesc) -> bool {
    let caps = match parse::parse(&desc.report_descriptor) {
        Ok(c) if c.report_len.iter().any(|&l| l != 0) => c,
        Ok(_) => {
            log::warn!("hid: {:04x}:{:04x} has no reports in its descriptor; not exposed", desc.vendor_id, desc.product_id);
            return false;
        }
        Err(e) => {
            log::warn!("hid: {:04x}:{:04x} report descriptor unparsable ({e}); not exposed", desc.vendor_id, desc.product_id);
            return false;
        }
    };
    let mut r = registry().lock().unwrap();
    if r.devices.iter().any(|d| d.desc.id == desc.id) {
        return true;
    }
    let inst = r.next_inst;
    r.next_inst += 1;
    let path = make_path(&desc);
    log::info!(
        "hid: device {:04x}:{:04x} {:?} ({}) in {} out {} feat {} -> {path}",
        desc.vendor_id,
        desc.product_id,
        desc.product.as_deref().unwrap_or("?"),
        if desc.bluetooth { "Bluetooth" } else { "USB" },
        caps.report_len[0],
        caps.report_len[1],
        caps.report_len[2]
    );
    let usage_page = caps.usage_page;
    let usage = caps.usage;
    r.devices.push(Arc::new(Device { desc, caps: Arc::new(caps), path, inst, connected: AtomicBool::new(true), readers: Mutex::new(Vec::new()) }));
    drop(r);
    crate::user32::misc::notify_device_change(true);
    crate::user32::input::notify_raw_hid_device_change(inst, usage_page, usage, true);
    true
}

/// Backend: a device went away. Pending reads fail with
/// STATUS_DEVICE_NOT_CONNECTED; open handles stay valid but every request
/// fails with ERROR_DEVICE_NOT_CONNECTED.
pub fn device_removed(id: u64) {
    let dev = {
        let mut r = registry().lock().unwrap();
        let Some(pos) = r.devices.iter().position(|d| d.desc.id == id) else { return };
        r.devices.remove(pos)
    };
    log::info!("hid: device {:04x}:{:04x} removed ({})", dev.desc.vendor_id, dev.desc.product_id, dev.path);
    dev.connected.store(false, Ordering::Release);
    let readers: Vec<_> = dev.readers.lock().unwrap().drain(..).filter_map(|w| w.upgrade()).collect();
    for h in readers {
        h.fail_all(STATUS_DEVICE_NOT_CONNECTED);
    }
    crate::user32::misc::notify_device_change(false);
    crate::user32::input::notify_raw_hid_device_change(dev.inst, dev.caps.usage_page, dev.caps.usage, false);
}

/// Windows layout of a report: `[report ID (0 if unnumbered), payload..]`,
/// exactly `len` bytes (zero padded). `data` may or may not start with the
/// report ID; the descriptor's report length decides.
pub fn windows_layout(caps: &Caps, t: usize, id: u8, data: &[u8], len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    if len == 0 {
        return out;
    }
    let expect = caps.report_bits.get(&(t, id)).map(|b| b.div_ceil(8) as usize);
    let has_id = caps.uses_report_ids && data.first() == Some(&id) && expect.map_or(true, |e| data.len() > e);
    let payload = if has_id { &data[1..] } else { data };
    out[0] = if caps.uses_report_ids { id } else { 0 };
    let n = payload.len().min(len - 1);
    out[1..1 + n].copy_from_slice(&payload[..n]);
    out
}

/// Backend: an input report arrived (`data` with or without the leading
/// report ID byte).
pub fn input_report(id: u64, report_id: u8, data: &[u8]) {
    let dev = registry().lock().unwrap().devices.iter().find(|d| d.desc.id == id).cloned();
    let Some(dev) = dev else { return };
    let len = dev.caps.report_len[HIDP_INPUT] as usize;
    if len == 0 {
        return;
    }
    let buf = windows_layout(&dev.caps, HIDP_INPUT, report_id, data, len);
    let readers: Vec<Arc<HidHandle>> = {
        let mut rs = dev.readers.lock().unwrap();
        rs.retain(|w| w.strong_count() > 0);
        rs.iter().filter_map(|w| w.upgrade()).collect()
    };
    for h in readers {
        h.deliver(&buf);
    }
    crate::user32::input::native_raw_hid(dev.inst, dev.caps.usage_page, dev.caps.usage, &buf);
}

/// Connected devices, in arrival order.
pub fn devices() -> Vec<Arc<Device>> {
    registry().lock().unwrap().devices.clone()
}

/// Look up a currently connected HID device by its stable instance number (`Device::inst`).
pub fn device_by_inst(inst: u32) -> Option<Arc<Device>> {
    registry().lock().unwrap().devices.iter().find(|d| d.inst == inst && d.connected()).cloned()
}

/// Is a Sony (054C) game controller exposed through the HID path? Then the
/// game's own pad library drives it, and XInput must not report it too.
pub fn exposes_sony_pad() -> bool {
    registry().lock().unwrap().devices.iter().any(|d| d.desc.vendor_id == 0x054c)
}

/// Start the platform backend (idempotent; waits briefly for the initial
/// device scan). Disabled with `AW2_HID=0`.
pub fn ensure_started() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        if std::env::var("AW2_HID").is_ok_and(|v| v == "0") {
            log::info!("hid: disabled by AW2_HID=0");
            return;
        }
        #[cfg(not(test))]
        crate::macos::hidmgr::start();
    });
}

// ---------------------------------------------------------------------------
// Device handles
// ---------------------------------------------------------------------------

enum PendingKind {
    Read { buf: usize, len: u32 },
    Write,
}

struct Pending {
    token: u64,
    ov: usize,
    kind: PendingKind,
    event: Option<Arc<Event>>,
}

struct HState {
    queue: VecDeque<Vec<u8>>,
    max_buffers: usize,
    pending: VecDeque<Pending>,
    next_token: u64,
}

impl Default for HState {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            max_buffers: INPUT_BUFFERS,
            pending: VecDeque::new(),
            next_token: 0,
        }
    }
}

pub struct HidHandle {
    pub dev: Arc<Device>,
    access: u32,
    overlapped: bool,
    st: Mutex<HState>,
    cv: Condvar,
    /// File-object signal state (used when an OVERLAPPED has no event).
    signaled: Guarded<bool>,
}

impl KObject for HidHandle {
    fn type_name(&self) -> &'static str {
        "HidDevice"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}

impl Waitable for HidHandle {
    fn is_signaled(&self, g: &WaitGuard, _tid: u32) -> bool {
        *self.signaled.get(g)
    }
    fn acquire(&self, _g: &WaitGuard, _tid: u32) -> bool {
        false
    }
}

impl Drop for HidHandle {
    fn drop(&mut self) {
        // Closing the last handle cancels its outstanding I/O.
        let pend: Vec<Pending> = self.st.get_mut().map(|s| s.pending.drain(..).collect()).unwrap_or_default();
        for p in pend {
            unsafe { finish(p.ov, STATUS_CANCELLED, 0, p.event.as_ref(), None) };
        }
    }
}

/// Complete an OVERLAPPED: buffer already written by the caller.
unsafe fn finish(ov: usize, status: usize, n: usize, event: Option<&Arc<Event>>, file: Option<&HidHandle>) {
    let o = ov as *mut OVERLAPPED;
    std::ptr::write_volatile(&mut (*o).internal_high, n);
    AtomicUsize::from_ptr(&mut (*o).internal).store(status, Ordering::Release);
    match event {
        Some(e) => e.set(),
        None => {
            if let Some(h) = file {
                let g = handles::wait_lock();
                *h.signaled.get(&g) = true;
                drop(g);
                handles::notify_waiters();
            }
        }
    }
}

/// Resolve and reset the OVERLAPPED's event (Windows resets it when the
/// operation starts) and mark the request pending.
unsafe fn start_overlapped(ov: *mut OVERLAPPED, h: &HidHandle) -> Option<Arc<Event>> {
    let ev = (*ov).h_event;
    let event = if ev.is_null() { None } else { handles::get_as::<Event>((ev as usize & !1) as HANDLE) };
    match &event {
        Some(e) => e.reset(),
        None => {
            let g = handles::wait_lock();
            *h.signaled.get(&g) = false;
        }
    }
    (*ov).internal_high = 0;
    AtomicUsize::from_ptr(&mut (*ov).internal).store(STATUS_PENDING, Ordering::Release);
    event
}

impl HidHandle {
    fn can_read(&self) -> bool {
        self.access & (GENERIC_READ | GENERIC_ALL | FILE_READ_DATA) != 0
    }
    fn can_write(&self) -> bool {
        self.access & (GENERIC_WRITE | GENERIC_ALL | FILE_WRITE_DATA) != 0
    }

    /// A normalized input report (exactly InputReportByteLength bytes).
    fn deliver(&self, rep: &[u8]) {
        let mut st = self.st.lock().unwrap();
        if let Some(i) = st.pending.iter().position(|p| matches!(p.kind, PendingKind::Read { .. })) {
            let p = st.pending.remove(i).unwrap();
            drop(st);
            if let PendingKind::Read { buf, len } = p.kind {
                let n = rep.len().min(len as usize);
                unsafe {
                    std::ptr::copy_nonoverlapping(rep.as_ptr(), buf as *mut u8, n);
                    finish(p.ov, STATUS_SUCCESS, n, p.event.as_ref(), Some(self));
                }
            }
            return;
        }
        let cap = st.max_buffers.max(2);
        while st.queue.len() >= cap {
            st.queue.pop_front();
        }
        st.queue.push_back(rep.to_vec());
        drop(st);
        self.cv.notify_all();
    }

    fn fail_all(&self, status: usize) {
        let pend: Vec<Pending> = self.st.lock().unwrap().pending.drain(..).collect();
        for p in pend {
            unsafe { finish(p.ov, status, 0, p.event.as_ref(), Some(self)) };
        }
        self.cv.notify_all();
    }

    fn complete_token(&self, token: u64, status: usize, n: usize) {
        let mut st = self.st.lock().unwrap();
        let Some(i) = st.pending.iter().position(|p| p.token == token) else { return };
        let p = st.pending.remove(i).unwrap();
        drop(st);
        unsafe { finish(p.ov, status, n, p.event.as_ref(), Some(self)) };
    }
}

fn hid_handle(h: HANDLE) -> Option<Arc<HidHandle>> {
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return None;
    }
    handles::get_as::<HidHandle>(h)
}

fn set_err(e: u32) {
    crate::teb::set_last_error(e);
}

/// Wait for a callback-style result, bounded by [`SYNC_TIMEOUT`].
fn wait_result<T: Send + 'static>(start: impl FnOnce(Box<dyn FnOnce(T) + Send>)) -> Option<T> {
    let slot = Arc::new((Mutex::new(None::<T>), Condvar::new()));
    let s2 = slot.clone();
    start(Box::new(move |v| {
        *s2.0.lock().unwrap() = Some(v);
        s2.1.notify_all();
    }));
    let (m, cv) = &*slot;
    let g = m.lock().unwrap();
    let (mut g, _) = cv.wait_timeout_while(g, SYNC_TIMEOUT, |v| v.is_none()).unwrap();
    g.take()
}

fn status_to_error(s: usize) -> u32 {
    match s {
        STATUS_CANCELLED => 995,
        STATUS_DEVICE_NOT_CONNECTED => ERROR_DEVICE_NOT_CONNECTED,
        STATUS_IO_DEVICE_ERROR => ERROR_IO_DEVICE,
        _ => ERROR_GEN_FAILURE,
    }
}

// ---------------------------------------------------------------------------
// kernel32 hooks
// ---------------------------------------------------------------------------

/// `CreateFileW` on a HID device path. None = not a HID path.
pub fn create_file(name: &str, access: u32, flags: u32) -> Option<HANDLE> {
    let lower = name.to_ascii_lowercase();
    if !(lower.starts_with("\\\\?\\hid#") || lower.starts_with("\\\\.\\hid#")) {
        return None;
    }
    let key = lower.replacen("\\\\.\\", "\\\\?\\", 1);
    let dev = registry().lock().unwrap().devices.iter().find(|d| d.path == key).cloned();
    let Some(dev) = dev else {
        log::debug!("CreateFileW({name:?}): no such HID device");
        set_err(err::ERROR_FILE_NOT_FOUND);
        return Some(INVALID_HANDLE_VALUE);
    };
    let h = Arc::new(HidHandle {
        dev: dev.clone(),
        access,
        overlapped: flags & FILE_FLAG_OVERLAPPED != 0,
        st: Mutex::new(HState::default()),
        cv: Condvar::new(),
        signaled: Guarded::new(false),
    });
    if h.can_read() {
        dev.readers.lock().unwrap().push(Arc::downgrade(&h));
    }
    let handle = handles::insert(h);
    log::debug!("CreateFileW({name:?}, {access:#x}, flags {flags:#x}) -> HID handle {handle:p}");
    set_err(0);
    Some(handle)
}

/// `ReadFile` on a HID handle: one input report.
pub unsafe fn read_file(h: HANDLE, buf: *mut std::ffi::c_void, n: u32, read: *mut u32, ov: *mut OVERLAPPED) -> Option<BOOL> {
    let hh = hid_handle(h)?;
    if !read.is_null() {
        *read = 0;
    }
    if !hh.can_read() {
        set_err(err::ERROR_ACCESS_DENIED);
        return Some(FALSE);
    }
    if !hh.dev.connected() {
        set_err(ERROR_DEVICE_NOT_CONNECTED);
        return Some(FALSE);
    }
    let need = hh.dev.caps.report_len[HIDP_INPUT] as u32;
    if need == 0 {
        set_err(err::ERROR_INVALID_FUNCTION);
        return Some(FALSE);
    }
    if n < need || buf.is_null() {
        set_err(ERROR_INVALID_USER_BUFFER);
        return Some(FALSE);
    }
    let mut st = hh.st.lock().unwrap();
    if ov.is_null() {
        // Synchronous read: wait for the next report (or removal).
        loop {
            if let Some(rep) = st.queue.pop_front() {
                std::ptr::copy_nonoverlapping(rep.as_ptr(), buf as *mut u8, rep.len());
                if !read.is_null() {
                    *read = rep.len() as u32;
                }
                return Some(TRUE);
            }
            if !hh.dev.connected() {
                set_err(ERROR_DEVICE_NOT_CONNECTED);
                return Some(FALSE);
            }
            st = hh.cv.wait_timeout(st, Duration::from_millis(250)).unwrap().0;
        }
    }
    if let Some(rep) = st.queue.pop_front() {
        drop(st);
        std::ptr::copy_nonoverlapping(rep.as_ptr(), buf as *mut u8, rep.len());
        if !read.is_null() {
            *read = rep.len() as u32;
        }
        let event = start_overlapped(ov, &hh);
        finish(ov as usize, STATUS_SUCCESS, rep.len(), event.as_ref(), Some(&hh));
        return Some(TRUE);
    }
    let event = start_overlapped(ov, &hh);
    let token = st.next_token;
    st.next_token += 1;
    st.pending.push_back(Pending { token, ov: ov as usize, kind: PendingKind::Read { buf: buf as usize, len: n }, event });
    drop(st);
    let _ = hh.overlapped;
    set_err(err::ERROR_IO_PENDING);
    Some(FALSE)
}

/// `WriteFile` on a HID handle: one output report (`buf[0]` = report ID).
pub unsafe fn write_file(h: HANDLE, buf: *const std::ffi::c_void, n: u32, written: *mut u32, ov: *mut OVERLAPPED) -> Option<BOOL> {
    let hh = hid_handle(h)?;
    if !written.is_null() {
        *written = 0;
    }
    if !hh.can_write() {
        set_err(err::ERROR_ACCESS_DENIED);
        return Some(FALSE);
    }
    if !hh.dev.connected() {
        set_err(ERROR_DEVICE_NOT_CONNECTED);
        return Some(FALSE);
    }
    if hh.dev.caps.report_len[HIDP_OUTPUT] == 0 {
        set_err(err::ERROR_INVALID_FUNCTION);
        return Some(FALSE);
    }
    if n == 0 || buf.is_null() {
        set_err(ERROR_INVALID_USER_BUFFER);
        return Some(FALSE);
    }
    let Some(be) = backend() else {
        set_err(ERROR_DEVICE_NOT_CONNECTED);
        return Some(FALSE);
    };
    let data = std::slice::from_raw_parts(buf as *const u8, n as usize).to_vec();
    let dev_id = hh.dev.desc.id;
    if ov.is_null() {
        let r = wait_result(|done| be.set_report(dev_id, ReportType::Output, data, done));
        return Some(match r {
            Some(Ok(k)) => {
                if !written.is_null() {
                    *written = k as u32;
                }
                TRUE
            }
            Some(Err(s)) => {
                set_err(status_to_error(s));
                FALSE
            }
            None => {
                set_err(ERROR_SEM_TIMEOUT);
                FALSE
            }
        });
    }
    let event = start_overlapped(ov, &hh);
    let token = {
        let mut st = hh.st.lock().unwrap();
        let t = st.next_token;
        st.next_token += 1;
        st.pending.push_back(Pending { token: t, ov: ov as usize, kind: PendingKind::Write, event });
        t
    };
    let weak = Arc::downgrade(&hh);
    be.set_report(
        dev_id,
        ReportType::Output,
        data,
        Box::new(move |r| {
            if let Some(h) = weak.upgrade() {
                match r {
                    Ok(k) => h.complete_token(token, STATUS_SUCCESS, k),
                    Err(s) => h.complete_token(token, s, 0),
                }
            }
        }),
    );
    set_err(err::ERROR_IO_PENDING);
    Some(FALSE)
}

/// `CancelIo` (`ex == false`, cancels everything on the handle) /
/// `CancelIoEx` (`ov` NULL = everything).
pub unsafe fn cancel_io(h: HANDLE, ov: *mut OVERLAPPED, ex: bool) -> Option<BOOL> {
    let hh = hid_handle(h)?;
    let matched: Vec<Pending> = {
        let mut st = hh.st.lock().unwrap();
        if ov.is_null() {
            st.pending.drain(..).collect()
        } else {
            let (m, keep): (Vec<_>, Vec<_>) = st.pending.drain(..).partition(|p| p.ov == ov as usize);
            st.pending = keep.into();
            m
        }
    };
    if matched.is_empty() {
        if ex {
            set_err(err::ERROR_NOT_FOUND);
            return Some(FALSE);
        }
        return Some(TRUE);
    }
    for p in matched {
        finish(p.ov, STATUS_CANCELLED, 0, p.event.as_ref(), Some(&hh));
    }
    Some(TRUE)
}

// ---------------------------------------------------------------------------
// HidD_*
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn HidD_GetHidGuid(guid: *mut u8) {
    if !guid.is_null() {
        std::ptr::copy_nonoverlapping(GUID_DEVINTERFACE_HID.as_ptr(), guid, 16);
    }
}

/// HIDD_ATTRIBUTES { ULONG Size; USHORT VendorID, ProductID, VersionNumber; }
pub unsafe extern "win64" fn HidD_GetAttributes(h: HANDLE, attr: *mut u8) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    if attr.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let d = &hh.dev.desc;
    (attr as *mut u32).write_unaligned(12);
    (attr.add(4) as *mut u16).write_unaligned(d.vendor_id);
    (attr.add(6) as *mut u16).write_unaligned(d.product_id);
    (attr.add(8) as *mut u16).write_unaligned(d.version);
    1
}

/// Preparsed data blobs handed out by `HidD_GetPreparsedData` (validated on use).
fn preparsed() -> &'static Mutex<HashSet<usize>> {
    static P: OnceLock<Mutex<HashSet<usize>>> = OnceLock::new();
    P.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Self-describing 16-byte header written into caller-allocated buffers by
/// `GetRawInputDeviceInfo(RIDI_PREPARSEDDATA)` so `HidP_*` can inspect them
/// without requiring `HidD_FreePreparsedData`.
const RAW_PREPARSED_MAGIC: [u8; 8] = *b"HidP RAW";
pub const RAW_PREPARSED_SIZE: usize = 16;

fn raw_preparsed() -> &'static Mutex<HashMap<u64, Arc<Caps>>> {
    static RP: OnceLock<Mutex<HashMap<u64, Arc<Caps>>>> = OnceLock::new();
    RP.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Build the flat `RIDI_PREPARSEDDATA` blob for a connected HID device.
pub fn raw_preparsed_bytes(dev: &Device) -> [u8; RAW_PREPARSED_SIZE] {
    let token = dev.inst as u64;
    raw_preparsed().lock().unwrap().insert(token, dev.caps.clone());
    let mut out = [0u8; RAW_PREPARSED_SIZE];
    out[0..8].copy_from_slice(&RAW_PREPARSED_MAGIC);
    out[8..16].copy_from_slice(&token.to_le_bytes());
    out
}

struct Preparsed {
    _magic: [u8; 8],
    caps: Arc<Caps>,
}

fn with_preparsed<R>(pp: *const u8, f: impl FnOnce(&Caps) -> R) -> Option<R> {
    if pp.is_null() {
        return None;
    }
    if preparsed().lock().unwrap().contains(&(pp as usize)) {
        let p = unsafe { &*(pp as *const Preparsed) };
        return Some(f(&p.caps));
    }
    if (pp as usize) < 0x10000 {
        return None;
    }
    let caps = {
        let map = raw_preparsed().lock().unwrap();
        if map.is_empty() {
            return None;
        }
        let hdr = unsafe { std::ptr::read_unaligned(pp as *const [u8; RAW_PREPARSED_SIZE]) };
        if hdr[0..8] != RAW_PREPARSED_MAGIC {
            return None;
        }
        let token = u64::from_le_bytes(hdr[8..16].try_into().unwrap());
        map.get(&token).cloned()?
    };
    Some(f(&caps))
}

pub unsafe extern "win64" fn HidD_GetPreparsedData(h: HANDLE, out: *mut *mut u8) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    if out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let p = Box::into_raw(Box::new(Preparsed { _magic: *b"HidP KDR", caps: hh.dev.caps.clone() })) as *mut u8;
    preparsed().lock().unwrap().insert(p as usize);
    *out = p;
    1
}

pub unsafe extern "win64" fn HidD_FreePreparsedData(pp: *mut u8) -> BOOLEAN {
    if preparsed().lock().unwrap().remove(&(pp as usize)) {
        drop(Box::from_raw(pp as *mut Preparsed));
        1
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        0
    }
}

pub unsafe extern "win64" fn HidD_GetFeature(h: HANDLE, buf: *mut u8, len: u32) -> BOOLEAN {
    sync_get(h, ReportType::Feature, buf, len)
}

pub unsafe extern "win64" fn HidD_GetInputReport(h: HANDLE, buf: *mut u8, len: u32) -> BOOLEAN {
    sync_get(h, ReportType::Input, buf, len)
}

unsafe fn sync_get(h: HANDLE, ty: ReportType, buf: *mut u8, len: u32) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    if buf.is_null() || len == 0 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return 0;
    }
    if !hh.dev.connected() {
        set_err(ERROR_DEVICE_NOT_CONNECTED);
        return 0;
    }
    let Some(be) = backend() else {
        set_err(ERROR_DEVICE_NOT_CONNECTED);
        return 0;
    };
    let id = *buf;
    let dev = hh.dev.desc.id;
    match wait_result(|done| be.get_report(dev, ty, id, len as usize, done)) {
        Some(Ok(data)) => {
            let out = windows_layout(&hh.dev.caps, ty as usize, id, &data, len as usize);
            std::ptr::copy_nonoverlapping(out.as_ptr(), buf, out.len());
            1
        }
        Some(Err(s)) => {
            log::debug!("hid: get {ty:?} report {id:#x} failed: status {s:#x}");
            set_err(status_to_error(s));
            0
        }
        None => {
            set_err(ERROR_SEM_TIMEOUT);
            0
        }
    }
}

pub unsafe extern "win64" fn HidD_SetFeature(h: HANDLE, buf: *const u8, len: u32) -> BOOLEAN {
    sync_set(h, ReportType::Feature, buf, len)
}

pub unsafe extern "win64" fn HidD_SetOutputReport(h: HANDLE, buf: *const u8, len: u32) -> BOOLEAN {
    sync_set(h, ReportType::Output, buf, len)
}

unsafe fn sync_set(h: HANDLE, ty: ReportType, buf: *const u8, len: u32) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    if buf.is_null() || len == 0 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return 0;
    }
    if !hh.dev.connected() {
        set_err(ERROR_DEVICE_NOT_CONNECTED);
        return 0;
    }
    let Some(be) = backend() else {
        set_err(ERROR_DEVICE_NOT_CONNECTED);
        return 0;
    };
    let data = std::slice::from_raw_parts(buf, len as usize).to_vec();
    let dev = hh.dev.desc.id;
    match wait_result(|done| be.set_report(dev, ty, data, done)) {
        Some(Ok(_)) => 1,
        Some(Err(s)) => {
            set_err(status_to_error(s));
            0
        }
        None => {
            set_err(ERROR_SEM_TIMEOUT);
            0
        }
    }
}

unsafe fn put_string(h: HANDLE, buf: *mut u16, bytes: u32, pick: impl FnOnce(&DeviceDesc) -> Option<String>) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    let Some(s) = pick(&hh.dev.desc).filter(|s| !s.is_empty()) else {
        set_err(ERROR_GEN_FAILURE);
        return 0;
    };
    if buf.is_null() || bytes < 2 {
        set_err(ERROR_INVALID_USER_BUFFER);
        return 0;
    }
    let cap = (bytes / 2) as usize;
    let w: Vec<u16> = s.encode_utf16().take(cap - 1).chain(std::iter::once(0)).collect();
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    1
}

pub unsafe extern "win64" fn HidD_GetManufacturerString(h: HANDLE, buf: *mut u16, bytes: u32) -> BOOLEAN {
    put_string(h, buf, bytes, |d| d.manufacturer.clone())
}
pub unsafe extern "win64" fn HidD_GetProductString(h: HANDLE, buf: *mut u16, bytes: u32) -> BOOLEAN {
    put_string(h, buf, bytes, |d| d.product.clone())
}
pub unsafe extern "win64" fn HidD_GetSerialNumberString(h: HANDLE, buf: *mut u16, bytes: u32) -> BOOLEAN {
    put_string(h, buf, bytes, |d| d.serial.clone())
}

pub unsafe extern "win64" fn HidD_FlushQueue(h: HANDLE) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    hh.st.lock().unwrap().queue.clear();
    1
}

pub unsafe extern "win64" fn HidD_GetNumInputBuffers(h: HANDLE, n: *mut u32) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    if n.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let count = hh.st.lock().unwrap().max_buffers as u32;
    std::ptr::write_unaligned(n, count);
    1
}

pub unsafe extern "win64" fn HidD_SetNumInputBuffers(h: HANDLE, n: u32) -> BOOLEAN {
    let Some(hh) = hid_handle(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    if !(2..=512).contains(&n) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let mut st = hh.st.lock().unwrap();
    st.max_buffers = n as usize;
    while st.queue.len() > st.max_buffers {
        st.queue.pop_front();
    }
    1
}

// ---------------------------------------------------------------------------
// HidP_*
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn HidP_GetCaps(pp: *const u8, caps: *mut u8) -> NTSTATUS {
    if caps.is_null() {
        return HIDP_STATUS_INVALID_PREPARSED_DATA;
    }
    match with_preparsed(pp, |c| c.to_hidp_caps()) {
        Some(raw) => {
            std::ptr::copy_nonoverlapping(raw.as_ptr(), caps, raw.len());
            HIDP_STATUS_SUCCESS
        }
        None => HIDP_STATUS_INVALID_PREPARSED_DATA,
    }
}

unsafe fn get_caps_list(ty: u32, out: *mut u8, count: *mut u16, pp: *const u8, buttons: bool) -> NTSTATUS {
    let t = ty as usize;
    if t > HIDP_FEATURE {
        return HIDP_STATUS_INVALID_REPORT_TYPE;
    }
    if count.is_null() {
        return HIDP_STATUS_INVALID_PREPARSED_DATA;
    }
    let r = with_preparsed(pp, |c| {
        let list = if buttons { &c.buttons[t] } else { &c.values[t] };
        if list.is_empty() {
            *count = 0;
            return HIDP_STATUS_USAGE_NOT_FOUND;
        }
        let room = *count as usize;
        let n = list.len().min(room);
        for (i, cap) in list.iter().take(n).enumerate() {
            let raw = if buttons { cap.to_hidp_button_caps() } else { cap.to_hidp_value_caps() };
            std::ptr::copy_nonoverlapping(raw.as_ptr(), out.add(i * 72), 72);
        }
        *count = n as u16;
        if n < list.len() {
            HIDP_STATUS_BUFFER_TOO_SMALL
        } else {
            HIDP_STATUS_SUCCESS
        }
    });
    r.unwrap_or(HIDP_STATUS_INVALID_PREPARSED_DATA)
}

pub unsafe extern "win64" fn HidP_GetValueCaps(ty: u32, out: *mut u8, count: *mut u16, pp: *const u8) -> NTSTATUS {
    get_caps_list(ty, out, count, pp, false)
}

pub unsafe extern "win64" fn HidP_GetButtonCaps(ty: u32, out: *mut u8, count: *mut u16, pp: *const u8) -> NTSTATUS {
    get_caps_list(ty, out, count, pp, true)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::kernel32::file::{CancelIoEx, CreateFileW, GetOverlappedResult, GetOverlappedResultEx, ReadFile, WriteFile};
    use crate::kernel32::threads::CloseHandle;

    /// Test backend: features answered from a table, output reports recorded
    /// and completed from another thread after a short delay.
    pub(crate) struct FakeBackend {
        pub sent: Mutex<Vec<(ReportType, Vec<u8>)>>,
    }

    impl Backend for FakeBackend {
        fn get_report(&self, _dev: u64, ty: ReportType, id: u8, len: usize, done: GetDone) {
            if ty == ReportType::Feature && id == 0x09 {
                let mut r = vec![0u8; 20];
                r[0] = 0x09;
                r[1..7].copy_from_slice(&[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
                r.truncate(len);
                done(Ok(r));
            } else {
                done(Err(STATUS_IO_DEVICE_ERROR));
            }
        }
        fn set_report(&self, _dev: u64, ty: ReportType, data: Vec<u8>, done: SetDone) {
            let n = data.len();
            self.sent.lock().unwrap().push((ty, data));
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(20));
                done(Ok(n));
            });
        }
    }

    pub(crate) fn install_fake_dualsense(id: u64) -> Arc<FakeBackend> {
        let be = Arc::new(FakeBackend { sent: Mutex::new(Vec::new()) });
        set_backend(be.clone());
        assert!(device_added(DeviceDesc {
            id,
            vendor_id: 0x054c,
            product_id: 0x0ce6,
            version: 0x0100,
            bluetooth: false,
            location: 0x0011_0000,
            manufacturer: Some("Sony Interactive Entertainment".into()),
            product: Some("DualSense Wireless Controller".into()),
            serial: None,
            report_descriptor: parse::tests::dualsense_usb(),
        }));
        be
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn new_ov(ev: HANDLE) -> OVERLAPPED {
        OVERLAPPED { internal: 0, internal_high: 0, offset: 0, offset_high: 0, h_event: ev }
    }

    #[test]
    fn dualsense_handle_queries_io_and_cancel() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let be = install_fake_dualsense(0xD5_0001);
        let dev = devices().into_iter().find(|d| d.desc.id == 0xD5_0001).unwrap();
        unsafe {
            let path = wide(&dev.path.to_uppercase().replace("\\\\?\\HID#", "\\\\?\\hid#"));
            let h = CreateFileW(path.as_ptr(), GENERIC_READ | GENERIC_WRITE, 3, std::ptr::null_mut(), 3, FILE_FLAG_OVERLAPPED, std::ptr::null_mut());
            assert_ne!(h, INVALID_HANDLE_VALUE);
            let mut attr = [0u8; 12];
            attr[0] = 12;
            assert_eq!(HidD_GetAttributes(h, attr.as_mut_ptr()), 1);
            assert_eq!(u16::from_le_bytes([attr[4], attr[5]]), 0x054c);
            assert_eq!(u16::from_le_bytes([attr[6], attr[7]]), 0x0ce6);

            // Preparsed data / caps as the game's pad library reads them.
            let mut pp = std::ptr::null_mut();
            assert_eq!(HidD_GetPreparsedData(h, &mut pp), 1);
            let mut caps = [0u8; 64];
            assert_eq!(HidP_GetCaps(pp, caps.as_mut_ptr()), HIDP_STATUS_SUCCESS);
            let nfeat = u16::from_le_bytes([caps[60], caps[61]]);
            let mut vc = vec![0u8; 72 * nfeat as usize];
            let mut cnt = nfeat;
            assert_eq!(HidP_GetValueCaps(2, vc.as_mut_ptr(), &mut cnt, pp), HIDP_STATUS_SUCCESS);
            assert!(vc.chunks(72).any(|c| c[0..2] == [0x00, 0xff] && c[2] == 0x85 && c[20] == 2 && c[56] == 0x2d));
            assert_eq!(HidD_FreePreparsedData(pp), 1);
            assert_eq!(HidD_FreePreparsedData(pp), 0, "double free is rejected");

            // Feature 0x09: pairing MAC.
            let mut f = [0u8; 20];
            f[0] = 0x09;
            assert_eq!(HidD_GetFeature(h, f.as_mut_ptr(), 20), 1);
            assert_eq!(&f[1..7], &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
            let mut s = [0u16; 129];
            assert_eq!(HidD_GetProductString(h, s.as_mut_ptr(), 258), 1);
            assert_eq!(String::from_utf16_lossy(&s[..29]), "DualSense Wireless Controller");
            assert_eq!(HidD_GetSerialNumberString(h, s.as_mut_ptr(), 258), 0);

            // Overlapped read: pending, then completed by a report from a
            // native thread; the event is set.
            let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null());
            let mut ov = new_ov(ev);
            let mut buf = [0u8; 64];
            let mut got = 0u32;
            assert_eq!(ReadFile(h, buf.as_mut_ptr().cast(), 64, &mut got, &mut ov), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_IO_PENDING);
            assert_eq!(ov.internal, STATUS_PENDING);
            let mut n = 0u32;
            assert_eq!(GetOverlappedResultEx(h, &mut ov, &mut n, 0, FALSE), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_IO_INCOMPLETE);
            assert_eq!(GetOverlappedResultEx(h, &mut ov, &mut n, 30, FALSE), FALSE);
            assert_eq!(crate::teb::last_error(), WAIT_TIMEOUT);
            let mut rep = [0u8; 64];
            rep[0] = 0x01;
            rep[1] = 0x7e;
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(30));
                input_report(0xD5_0001, 1, &rep);
            });
            assert_eq!(GetOverlappedResultEx(h, &mut ov, &mut n, 2000, FALSE), TRUE);
            assert_eq!((n, buf[0], buf[1]), (64, 0x01, 0x7e));

            // A queued report completes the next read synchronously.
            input_report(0xD5_0001, 1, &rep[1..10]);
            assert_eq!(ReadFile(h, buf.as_mut_ptr().cast(), 64, &mut got, &mut ov), TRUE);
            assert_eq!((got, ov.internal, ov.internal_high), (64, 0, 64));
            assert_eq!(buf[10..], [0u8; 54], "short reports are zero-padded to InputReportByteLength");

            // The game's timeout path: wait 100 ms, CancelIoEx, then
            // GetOverlappedResult(wait) -> ERROR_OPERATION_ABORTED.
            assert_eq!(ReadFile(h, buf.as_mut_ptr().cast(), 64, &mut got, &mut ov), FALSE);
            assert_eq!(GetOverlappedResultEx(h, &mut ov, &mut n, 20, FALSE), FALSE);
            assert_eq!(crate::teb::last_error(), WAIT_TIMEOUT);
            assert_eq!(CancelIoEx(h, &mut ov), TRUE);
            assert_eq!(GetOverlappedResult(h, &mut ov, &mut n, TRUE), FALSE);
            assert_eq!(crate::teb::last_error(), 995);
            assert_eq!(CancelIoEx(h, &mut ov), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_NOT_FOUND);

            // Output report: asynchronous completion.
            let mut out = [0u8; 48];
            out[0] = 0x02;
            out[1] = 0xff;
            assert_eq!(WriteFile(h, out.as_ptr().cast(), 48, &mut got, &mut ov), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_IO_PENDING);
            assert_eq!(GetOverlappedResultEx(h, &mut ov, &mut n, 2000, FALSE), TRUE);
            assert_eq!(n, 48);
            assert_eq!(be.sent.lock().unwrap().last().unwrap(), &(ReportType::Output, out.to_vec()));

            // HidD_GetNumInputBuffers / HidD_SetNumInputBuffers resize the live ring buffer.
            let mut nbufs = 0u32;
            assert_eq!(HidD_GetNumInputBuffers(h, &mut nbufs), 1);
            assert_eq!(nbufs, INPUT_BUFFERS as u32);
            assert_eq!(HidD_SetNumInputBuffers(h, 1), 0); // < 2 invalid
            assert_eq!(HidD_SetNumInputBuffers(h, 513), 0); // > 512 invalid
            assert_eq!(HidD_SetNumInputBuffers(h, 3), 1);
            assert_eq!(HidD_GetNumInputBuffers(h, &mut nbufs), 1);
            assert_eq!(nbufs, 3);
            for seq in 1u8..=5u8 {
                let mut r = [0u8; 64];
                r[0] = 0x01;
                r[1] = seq;
                input_report(0xD5_0001, 1, &r);
            }
            // Ring cap 3 kept only the 3 most recent reports: seq = 3, 4, 5.
            for expected in [3u8, 4, 5] {
                assert_eq!(ReadFile(h, buf.as_mut_ptr().cast(), 64, &mut got, &mut ov), TRUE);
                assert_eq!(buf[1], expected);
            }
            assert_eq!(HidD_SetNumInputBuffers(h, INPUT_BUFFERS as u32), 1);

            // Removal fails pending reads and later requests.
            assert_eq!(ReadFile(h, buf.as_mut_ptr().cast(), 64, &mut got, &mut ov), FALSE);
            device_removed(0xD5_0001);
            assert_eq!(GetOverlappedResult(h, &mut ov, &mut n, TRUE), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_DEVICE_NOT_CONNECTED);
            assert_eq!(ReadFile(h, buf.as_mut_ptr().cast(), 64, &mut got, &mut ov), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_DEVICE_NOT_CONNECTED);
            assert_eq!(CloseHandle(h), TRUE);
            assert_eq!(CloseHandle(ev), TRUE);
            let gone = CreateFileW(path.as_ptr(), GENERIC_READ, 3, std::ptr::null_mut(), 3, 0, std::ptr::null_mut());
            assert_eq!(gone, INVALID_HANDLE_VALUE);
            assert_eq!(crate::teb::last_error(), err::ERROR_FILE_NOT_FOUND);
        }
    }
}
