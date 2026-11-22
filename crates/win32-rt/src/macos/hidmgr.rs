//! IOHIDManager backend for `misc::hid` (the game's own DualSense / DS4
//! path through hid.dll + setupapi).
//!
//! * Only game controllers are exposed: Generic Desktop Joystick (4),
//!   Game Pad (5) and Multi-axis Controller (8). Keyboards and mice would
//!   need the Input Monitoring permission and nothing in the game wants
//!   them through HID.
//! * Devices are opened shared (`kIOHIDOptionsTypeNone`), so
//!   GameController.framework keeps working; `macos::gamepad` stops
//!   reporting Sony pads through XInput while they are exposed here.
//! * Threads: IOKit / CoreFoundation are only called on two native threads
//!   spawned here, never on Windows threads:
//!   * `aw2-hid` runs a CFRunLoop with the manager (arrival/removal) and the
//!     per-device input-report callbacks;
//!   * `aw2-hid-io` performs `IOHIDDeviceGetReport`/`SetReport` requests
//!     queued by `misc::hid` (they block, e.g. on Bluetooth round-trips, so
//!     they stay off the input thread).
//! * Report layout follows hidapi's macOS backend: for numbered reports the
//!   buffer passed to `IOHIDDeviceSetReport` starts with the report ID; for
//!   unnumbered reports the leading 0 byte is dropped. `misc::hid` normalizes
//!   whatever IOKit returns to the Windows layout.

#[cfg(target_os = "macos")]
use crate::misc::hid::{self, Backend, DeviceDesc, GetDone, ReportType, SetDone, STATUS_DEVICE_NOT_CONNECTED, STATUS_IO_DEVICE_ERROR};
#[cfg(target_os = "macos")]
use std::sync::{mpsc, Arc, Condvar, Mutex, OnceLock};
#[cfg(target_os = "macos")]
use std::time::Duration;

/// Start the backend (idempotent, any thread). Waits up to 1 s for the
/// initial device scan so the first enumeration already sees connected pads.
pub fn start() {
    #[cfg(target_os = "macos")]
    imp::start();
}

#[cfg(target_os = "macos")]
mod imp {
    use super::*;
    use std::collections::HashMap;
    use std::ffi::{c_char, c_void, CStr};

    type CFTypeRef = *const c_void;
    type CFAllocatorRef = *const c_void;
    type CFStringRef = *const c_void;
    type CFMutableDictionaryRef = *mut c_void;
    type CFArrayRef = *const c_void;
    type CFRunLoopRef = *mut c_void;
    type CFIndex = isize;
    type IOHIDManagerRef = *mut c_void;
    type IOHIDDeviceRef = *mut c_void;
    type IOReturn = i32;

    const K_CFSTRING_ENCODING_UTF8: u32 = 0x0800_0100;
    const K_CFNUMBER_SINT32: CFIndex = 3;
    const K_CFNUMBER_SINT64: CFIndex = 4;
    const K_IOHID_OPTIONS_NONE: u32 = 0;

    #[repr(C)]
    struct CFCallBacks {
        _p: [u8; 0],
    }

    type DeviceCallback = extern "C" fn(ctx: *mut c_void, result: IOReturn, sender: *mut c_void, dev: IOHIDDeviceRef);
    type ReportCallback =
        extern "C" fn(ctx: *mut c_void, result: IOReturn, sender: *mut c_void, ty: u32, id: u32, report: *mut u8, len: CFIndex);

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFAllocatorDefault: CFAllocatorRef;
        static kCFRunLoopDefaultMode: CFStringRef;
        static kCFTypeDictionaryKeyCallBacks: CFCallBacks;
        static kCFTypeDictionaryValueCallBacks: CFCallBacks;
        static kCFTypeArrayCallBacks: u8;
        fn CFRelease(o: CFTypeRef);
        fn CFRetain(o: CFTypeRef) -> CFTypeRef;
        fn CFGetTypeID(o: CFTypeRef) -> usize;
        fn CFNumberGetTypeID() -> usize;
        fn CFStringGetTypeID() -> usize;
        fn CFDataGetTypeID() -> usize;
        fn CFNumberCreate(a: CFAllocatorRef, ty: CFIndex, v: *const c_void) -> CFTypeRef;
        fn CFNumberGetValue(n: CFTypeRef, ty: CFIndex, v: *mut c_void) -> u8;
        fn CFStringCreateWithCString(a: CFAllocatorRef, s: *const c_char, enc: u32) -> CFStringRef;
        fn CFStringGetCString(s: CFStringRef, buf: *mut u8, len: CFIndex, enc: u32) -> u8;
        fn CFDataGetLength(d: CFTypeRef) -> CFIndex;
        fn CFDataGetBytePtr(d: CFTypeRef) -> *const u8;
        fn CFDictionaryCreateMutable(a: CFAllocatorRef, cap: CFIndex, k: *const CFCallBacks, v: *const CFCallBacks) -> CFMutableDictionaryRef;
        fn CFDictionarySetValue(d: CFMutableDictionaryRef, k: CFTypeRef, v: CFTypeRef);
        fn CFArrayCreate(a: CFAllocatorRef, vals: *const CFTypeRef, n: CFIndex, cb: *const u8) -> CFArrayRef;
        fn CFRunLoopGetCurrent() -> CFRunLoopRef;
        fn CFRunLoopRun();
        fn CFRunLoopRunInMode(mode: CFStringRef, secs: f64, ret_after_source: u8) -> i32;
    }

    #[link(name = "IOKit", kind = "framework")]
    extern "C" {
        fn IOHIDManagerCreate(a: CFAllocatorRef, opts: u32) -> IOHIDManagerRef;
        fn IOHIDManagerSetDeviceMatchingMultiple(m: IOHIDManagerRef, multiple: CFArrayRef);
        fn IOHIDManagerRegisterDeviceMatchingCallback(m: IOHIDManagerRef, cb: DeviceCallback, ctx: *mut c_void);
        fn IOHIDManagerRegisterDeviceRemovalCallback(m: IOHIDManagerRef, cb: DeviceCallback, ctx: *mut c_void);
        fn IOHIDManagerScheduleWithRunLoop(m: IOHIDManagerRef, rl: CFRunLoopRef, mode: CFStringRef);
        fn IOHIDManagerOpen(m: IOHIDManagerRef, opts: u32) -> IOReturn;
        fn IOHIDDeviceGetProperty(d: IOHIDDeviceRef, key: CFStringRef) -> CFTypeRef;
        fn IOHIDDeviceOpen(d: IOHIDDeviceRef, opts: u32) -> IOReturn;
        fn IOHIDDeviceClose(d: IOHIDDeviceRef, opts: u32) -> IOReturn;
        fn IOHIDDeviceRegisterInputReportCallback(d: IOHIDDeviceRef, report: *mut u8, len: CFIndex, cb: ReportCallback, ctx: *mut c_void);
        fn IOHIDDeviceGetReport(d: IOHIDDeviceRef, ty: u32, id: CFIndex, report: *mut u8, len: *mut CFIndex) -> IOReturn;
        fn IOHIDDeviceSetReport(d: IOHIDDeviceRef, ty: u32, id: CFIndex, report: *const u8, len: CFIndex) -> IOReturn;
        fn IOHIDDeviceGetService(d: IOHIDDeviceRef) -> u32;
        fn IORegistryEntryGetRegistryEntryID(entry: u32, id: *mut u64) -> IOReturn;
    }

    fn cfstr(s: &str) -> CFStringRef {
        let c = std::ffi::CString::new(s).unwrap();
        unsafe { CFStringCreateWithCString(kCFAllocatorDefault, c.as_ptr(), K_CFSTRING_ENCODING_UTF8) }
    }

    unsafe fn prop(dev: IOHIDDeviceRef, key: &str) -> CFTypeRef {
        let k = cfstr(key);
        let v = IOHIDDeviceGetProperty(dev, k);
        CFRelease(k);
        v
    }

    unsafe fn prop_i64(dev: IOHIDDeviceRef, key: &str) -> Option<i64> {
        let v = prop(dev, key);
        if v.is_null() || CFGetTypeID(v) != CFNumberGetTypeID() {
            return None;
        }
        let mut out = 0i64;
        (CFNumberGetValue(v, K_CFNUMBER_SINT64, &mut out as *mut i64 as *mut c_void) != 0).then_some(out)
    }

    unsafe fn prop_string(dev: IOHIDDeviceRef, key: &str) -> Option<String> {
        let v = prop(dev, key);
        if v.is_null() || CFGetTypeID(v) != CFStringGetTypeID() {
            return None;
        }
        let mut buf = [0u8; 512];
        if CFStringGetCString(v, buf.as_mut_ptr(), buf.len() as CFIndex, K_CFSTRING_ENCODING_UTF8) == 0 {
            return None;
        }
        Some(CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned())
    }

    unsafe fn prop_data(dev: IOHIDDeviceRef, key: &str) -> Option<Vec<u8>> {
        let v = prop(dev, key);
        if v.is_null() || CFGetTypeID(v) != CFDataGetTypeID() {
            return None;
        }
        let n = CFDataGetLength(v) as usize;
        Some(std::slice::from_raw_parts(CFDataGetBytePtr(v), n).to_vec())
    }

    /// Open devices by backend id (retained IOHIDDeviceRef as usize).
    fn open_devices() -> &'static Mutex<HashMap<u64, usize>> {
        static D: OnceLock<Mutex<HashMap<u64, usize>>> = OnceLock::new();
        D.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// Input-report context: leaked while the device is open.
    struct DevCtx {
        id: u64,
        buf: Vec<u8>,
    }

    fn contexts() -> &'static Mutex<HashMap<u64, usize>> {
        static C: OnceLock<Mutex<HashMap<u64, usize>>> = OnceLock::new();
        C.get_or_init(|| Mutex::new(HashMap::new()))
    }

    unsafe fn entry_id(dev: IOHIDDeviceRef) -> u64 {
        let mut id = 0u64;
        let svc = IOHIDDeviceGetService(dev);
        if svc == 0 || IORegistryEntryGetRegistryEntryID(svc, &mut id) != 0 {
            id = dev as usize as u64;
        }
        id
    }

    extern "C" fn on_report(ctx: *mut c_void, result: IOReturn, _s: *mut c_void, _ty: u32, id: u32, report: *mut u8, len: CFIndex) {
        if result != 0 || ctx.is_null() || report.is_null() || len <= 0 {
            return;
        }
        let c = unsafe { &*(ctx as *const DevCtx) };
        let data = unsafe { std::slice::from_raw_parts(report, len as usize) };
        hid::input_report(c.id, id as u8, data);
    }

    extern "C" fn on_match(_ctx: *mut c_void, _r: IOReturn, _s: *mut c_void, dev: IOHIDDeviceRef) {
        unsafe {
            let id = entry_id(dev);
            if open_devices().lock().unwrap().contains_key(&id) {
                return;
            }
            let transport = prop_string(dev, "Transport").unwrap_or_default();
            let desc = DeviceDesc {
                id,
                vendor_id: prop_i64(dev, "VendorID").unwrap_or(0) as u16,
                product_id: prop_i64(dev, "ProductID").unwrap_or(0) as u16,
                version: prop_i64(dev, "VersionNumber").unwrap_or(0) as u16,
                bluetooth: transport.starts_with("Bluetooth"),
                location: prop_i64(dev, "LocationID").unwrap_or(0) as u32,
                manufacturer: prop_string(dev, "Manufacturer"),
                product: prop_string(dev, "Product"),
                serial: prop_string(dev, "SerialNumber"),
                report_descriptor: prop_data(dev, "ReportDescriptor").unwrap_or_default(),
            };
            let r = IOHIDDeviceOpen(dev, K_IOHID_OPTIONS_NONE);
            if r != 0 {
                log::warn!("hid: IOHIDDeviceOpen {:04x}:{:04x} failed: {r:#x}; not exposed", desc.vendor_id, desc.product_id);
                return;
            }
            let max_in = prop_i64(dev, "MaxInputReportSize").unwrap_or(0).max(64) as usize;
            let (vid, pid) = (desc.vendor_id, desc.product_id);
            if !hid::device_added(desc) {
                IOHIDDeviceClose(dev, K_IOHID_OPTIONS_NONE);
                return;
            }
            CFRetain(dev as CFTypeRef);
            open_devices().lock().unwrap().insert(id, dev as usize);
            let ctx = Box::into_raw(Box::new(DevCtx { id, buf: vec![0u8; max_in] }));
            contexts().lock().unwrap().insert(id, ctx as usize);
            IOHIDDeviceRegisterInputReportCallback(dev, (*ctx).buf.as_mut_ptr(), max_in as CFIndex, on_report, ctx as *mut c_void);
            log::debug!("hid: opened {vid:04x}:{pid:04x} ({transport}), input buffer {max_in}");
        }
    }

    extern "C" fn on_remove(_ctx: *mut c_void, _r: IOReturn, _s: *mut c_void, dev: IOHIDDeviceRef) {
        unsafe {
            let id = entry_id(dev);
            let Some(d) = open_devices().lock().unwrap().remove(&id) else { return };
            hid::device_removed(id);
            CFRelease(d as CFTypeRef);
            // The context is freed after IOKit stops delivering (device gone).
            if let Some(c) = contexts().lock().unwrap().remove(&id) {
                drop(Box::from_raw(c as *mut DevCtx));
            }
        }
    }

    unsafe fn matching_dict(page: i32, usage: i32) -> CFTypeRef {
        let d = CFDictionaryCreateMutable(kCFAllocatorDefault, 2, &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
        for (k, v) in [("DeviceUsagePage", page), ("DeviceUsage", usage)] {
            let key = cfstr(k);
            let num = CFNumberCreate(kCFAllocatorDefault, K_CFNUMBER_SINT32, &v as *const i32 as *const c_void);
            CFDictionarySetValue(d, key, num);
            CFRelease(key);
            CFRelease(num);
        }
        d as CFTypeRef
    }

    enum Req {
        Get { dev: u64, ty: ReportType, id: u8, len: usize, done: GetDone },
        Set { dev: u64, ty: ReportType, data: Vec<u8>, done: SetDone },
    }

    struct IoKitBackend {
        tx: Mutex<mpsc::Sender<Req>>,
    }

    impl Backend for IoKitBackend {
        fn get_report(&self, dev: u64, ty: ReportType, id: u8, len: usize, done: GetDone) {
            if let Err(mpsc::SendError(Req::Get { done, .. })) = self.tx.lock().unwrap().send(Req::Get { dev, ty, id, len, done }) {
                done(Err(STATUS_DEVICE_NOT_CONNECTED));
            }
        }
        fn set_report(&self, dev: u64, ty: ReportType, data: Vec<u8>, done: SetDone) {
            if let Err(mpsc::SendError(Req::Set { done, .. })) = self.tx.lock().unwrap().send(Req::Set { dev, ty, data, done }) {
                done(Err(STATUS_DEVICE_NOT_CONNECTED));
            }
        }
    }

    /// Retained device ref for an I/O request (released by the caller).
    fn device_ref(id: u64) -> Option<IOHIDDeviceRef> {
        let g = open_devices().lock().unwrap();
        let d = *g.get(&id)?;
        unsafe { CFRetain(d as CFTypeRef) };
        Some(d as IOHIDDeviceRef)
    }

    fn io_thread(rx: mpsc::Receiver<Req>) {
        for req in rx {
            match req {
                Req::Get { dev, ty, id, len, done } => {
                    let Some(d) = device_ref(dev) else {
                        done(Err(STATUS_DEVICE_NOT_CONNECTED));
                        continue;
                    };
                    // hidapi layout: numbered reports keep the ID byte.
                    let mut buf = vec![0u8; len.max(1)];
                    buf[0] = id;
                    let (ptr, mut n) = if id == 0 { (buf[1..].as_mut_ptr(), (len.max(1) - 1) as CFIndex) } else { (buf.as_mut_ptr(), len as CFIndex) };
                    let r = unsafe { IOHIDDeviceGetReport(d, ty as u32, id as CFIndex, ptr, &mut n) };
                    unsafe { CFRelease(d as CFTypeRef) };
                    if r != 0 {
                        log::debug!("hid: IOHIDDeviceGetReport({ty:?} {id:#x}) failed: {r:#x}");
                        done(Err(STATUS_IO_DEVICE_ERROR));
                    } else {
                        let n = n.max(0) as usize;
                        buf.truncate(if id == 0 { n + 1 } else { n });
                        done(Ok(if id == 0 { buf[1..].to_vec() } else { buf }));
                    }
                }
                Req::Set { dev, ty, data, done } => {
                    let Some(d) = device_ref(dev) else {
                        done(Err(STATUS_DEVICE_NOT_CONNECTED));
                        continue;
                    };
                    let id = data[0];
                    let send: &[u8] = if id == 0 { &data[1..] } else { &data };
                    let r = unsafe { IOHIDDeviceSetReport(d, ty as u32, id as CFIndex, send.as_ptr(), send.len() as CFIndex) };
                    unsafe { CFRelease(d as CFTypeRef) };
                    if r != 0 {
                        log::debug!("hid: IOHIDDeviceSetReport({ty:?} {id:#x}, {} bytes) failed: {r:#x}", data.len());
                        done(Err(STATUS_IO_DEVICE_ERROR));
                    } else {
                        done(Ok(data.len()));
                    }
                }
            }
        }
    }

    pub(super) fn start() {
        static READY: OnceLock<Arc<(Mutex<bool>, Condvar)>> = OnceLock::new();
        let mut first = false;
        let ready = READY
            .get_or_init(|| {
                first = true;
                Arc::new((Mutex::new(false), Condvar::new()))
            })
            .clone();
        if first {
            let (tx, rx) = mpsc::channel::<Req>();
            hid::set_backend(Arc::new(IoKitBackend { tx: Mutex::new(tx) }));
            let spawned = std::thread::Builder::new().name("aw2-hid-io".into()).spawn(move || io_thread(rx)).is_ok()
                && std::thread::Builder::new()
                    .name("aw2-hid".into())
                    .spawn({
                        let ready = ready.clone();
                        move || unsafe { run_loop(ready) }
                    })
                    .is_ok();
            if !spawned {
                log::warn!("hid: could not start the IOHIDManager threads");
                return;
            }
        }
        let (m, cv) = &*ready;
        let g = m.lock().unwrap();
        let _ = cv.wait_timeout_while(g, Duration::from_secs(1), |r| !*r).unwrap();
    }

    unsafe fn run_loop(ready: Arc<(Mutex<bool>, Condvar)>) {
        let signal = || {
            *ready.0.lock().unwrap() = true;
            ready.1.notify_all();
        };
        let mgr = IOHIDManagerCreate(kCFAllocatorDefault, K_IOHID_OPTIONS_NONE);
        if mgr.is_null() {
            log::warn!("hid: IOHIDManagerCreate failed");
            signal();
            return;
        }
        let dicts = [matching_dict(1, 4), matching_dict(1, 5), matching_dict(1, 8)];
        let arr = CFArrayCreate(kCFAllocatorDefault, dicts.as_ptr(), dicts.len() as CFIndex, &kCFTypeArrayCallBacks);
        IOHIDManagerSetDeviceMatchingMultiple(mgr, arr);
        CFRelease(arr);
        for d in dicts {
            CFRelease(d);
        }
        IOHIDManagerRegisterDeviceMatchingCallback(mgr, on_match, std::ptr::null_mut());
        IOHIDManagerRegisterDeviceRemovalCallback(mgr, on_remove, std::ptr::null_mut());
        IOHIDManagerScheduleWithRunLoop(mgr, CFRunLoopGetCurrent(), kCFRunLoopDefaultMode);
        let r = IOHIDManagerOpen(mgr, K_IOHID_OPTIONS_NONE);
        if r != 0 {
            log::warn!("hid: IOHIDManagerOpen failed: {r:#x}");
        }
        // Deliver the initial matches, then let enumeration proceed.
        CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.25, 0);
        log::info!("hid: IOHIDManager monitoring game controllers ({} exposed)", hid::devices().len());
        signal();
        CFRunLoopRun();
    }
}
