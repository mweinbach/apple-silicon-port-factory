//! powrprof.dll: power management, platform role, battery state, and processor power queries.

use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// POWER_PLATFORM_ROLE values (winnt.h)
const PLATFORM_ROLE_DESKTOP: u32 = 1;
const PLATFORM_ROLE_MOBILE: u32 = 2;

// POWER_INFORMATION_LEVEL values (winnt.h)
const SYSTEM_POWER_POLICY_AC: u32 = 0;
const SYSTEM_POWER_POLICY_DC: u32 = 1;
const VERIFY_SYSTEM_POLICY_AC: u32 = 2;
const VERIFY_SYSTEM_POLICY_DC: u32 = 3;
const SYSTEM_POWER_CAPABILITIES: u32 = 4;
const SYSTEM_BATTERY_STATE: u32 = 5;
const SYSTEM_POWER_STATE_HANDLER: u32 = 6;
const PROCESSOR_STATE_HANDLER: u32 = 7;
const SYSTEM_POWER_POLICY_CURRENT: u32 = 8;
const ADMINISTRATOR_POWER_POLICY: u32 = 9;
const SYSTEM_RESERVE_HIBER_FILE: u32 = 10;
const PROCESSOR_INFORMATION: u32 = 11;
const SYSTEM_POWER_INFORMATION: u32 = 12;
const PROCESSOR_STATE_HANDLER2: u32 = 13;
const LAST_WAKE_TIME: u32 = 14;
const LAST_SLEEP_TIME: u32 = 15;
const SYSTEM_EXECUTION_STATE: u32 = 16;
const SYSTEM_POWER_STATE_NOTIFY_HANDLER: u32 = 17;
const PROCESSOR_POWER_POLICY_AC: u32 = 18;
const PROCESSOR_POWER_POLICY_DC: u32 = 19;
const VERIFY_PROCESSOR_POLICY_AC: u32 = 20;
const VERIFY_PROCESSOR_POLICY_DC: u32 = 21;
const PROCESSOR_POWER_POLICY_CURRENT: u32 = 22;
const SYSTEM_POWER_STATE_LOGGING: u32 = 23;
const SYSTEM_POWER_LOGGING_ENTRY: u32 = 24;
const SET_POWER_SETTING_VALUE: u32 = 25;
const SYSTEM_WAKE_SOURCE: u32 = 35;
const SYSTEM_HIBER_FILE_SIZE: u32 = 51;
const SYSTEM_HIBER_FILE_INFORMATION: u32 = 68;
const POWER_INFORMATION_LEVEL_MAX: u32 = 100;

const STATUS_SUCCESS: i32 = 0;
const STATUS_INVALID_PARAMETER: i32 = 0xC000_000Du32 as i32;
const STATUS_ACCESS_DENIED: i32 = 0xC000_0022u32 as i32;
const STATUS_BUFFER_TOO_SMALL: i32 = 0xC000_0023u32 as i32;

const DEVICE_NOTIFY_WINDOW_HANDLE: u32 = 0;
const DEVICE_NOTIFY_SERVICE_HANDLE: u32 = 1;
const DEVICE_NOTIFY_CALLBACK: u32 = 2;

#[cfg(any(target_os = "macos", test))]
const PBT_APMSUSPEND: u32 = 0x0004;
#[cfg(any(target_os = "macos", test))]
const PBT_APMRESUMESUSPEND: u32 = 0x0007;
#[cfg(any(target_os = "macos", test))]
const PBT_APMRESUMEAUTOMATIC: u32 = 0x0012;
const PBT_POWERSETTINGCHANGE: u32 = 0x8013;

// Standard Windows Power Setting GUIDs (winnt.h)
// GUID_ACDC_POWER_SOURCE: {5d3e9a59-e9d5-4b00-a6bd-ff34ff516548}
const GUID_ACDC_POWER_SOURCE: [u8; 16] = [
    0x59, 0x9A, 0x3E, 0x5D, 0xD5, 0xE9, 0x00, 0x4B, 0xA6, 0xBD, 0xFF, 0x34, 0xFF, 0x51, 0x65, 0x48,
];
// GUID_BATTERY_PERCENTAGE_REMAINING: {a7ad8041-b45a-4cae-87a3-eecbb468a9e1}
const GUID_BATTERY_PERCENTAGE_REMAINING: [u8; 16] = [
    0x41, 0x80, 0xAD, 0xA7, 0x5A, 0xB4, 0xAE, 0x4C, 0x87, 0xA3, 0xEE, 0xCB, 0xB4, 0x68, 0xA9, 0xE1,
];
// GUID_POWERSCHEME_PERSONALITY: {245d8541-3943-4422-b025-13a784f679b7}
const GUID_POWERSCHEME_PERSONALITY: [u8; 16] = [
    0x41, 0x85, 0x5D, 0x24, 0x43, 0x39, 0x22, 0x44, 0xB0, 0x25, 0x13, 0xA7, 0x84, 0xF6, 0x79, 0xB7,
];
// GUID_CONSOLE_DISPLAY_STATE: {6fe69556-704a-47a0-8f24-c28d936fda47}
const GUID_CONSOLE_DISPLAY_STATE: [u8; 16] = [
    0x56, 0x95, 0xE6, 0x6F, 0x4A, 0x70, 0xA0, 0x47, 0x8F, 0x24, 0xC2, 0x8D, 0x93, 0x6F, 0xDA, 0x47,
];
// GUID_MONITOR_POWER_ON: {02731015-4510-4526-99e6-e5a17ebd1aea}
const GUID_MONITOR_POWER_ON: [u8; 16] = [
    0x15, 0x10, 0x73, 0x02, 0x10, 0x45, 0x26, 0x45, 0x99, 0xE6, 0xE5, 0xA1, 0x7E, 0xBD, 0x1A, 0xEA,
];
// GUID_POWER_SAVING_STATUS: {e00958c0-c213-4ace-ac77-fecced2eeea5}
const GUID_POWER_SAVING_STATUS: [u8; 16] = [
    0xC0, 0x58, 0x09, 0xE0, 0x13, 0xC2, 0xCE, 0x4A, 0xAC, 0x77, 0xFE, 0xCC, 0xED, 0x2E, 0xEE, 0xA5,
];

const BATTERY_UNKNOWN_TIME: u32 = 0xFFFF_FFFF;

// Balanced power scheme GUID: {381b4222-f694-41f0-9685-ff5bb260df2e}
const GUID_TYPICAL_POWER_SAVINGS: [u8; 16] = [
    0x22, 0x42, 0x1B, 0x38, 0x94, 0xF6, 0xF0, 0x41, 0x96, 0x85, 0xFF, 0x5B, 0xB2, 0x60, 0xDF, 0x2E,
];

#[derive(Clone, Copy, Debug)]
struct BatterySnapshot {
    ac_online: bool,
    battery_present: bool,
    charging: bool,
    discharging: bool,
    max_capacity: u32,
    remaining_capacity: u32,
    rate: i32,
    estimated_time: u32,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum PowerNotifyKind {
    SuspendResume,
    Setting([u8; 16]),
}

#[derive(Clone, Debug)]
enum PowerNotifyTarget {
    Window(usize),
    Service,
    Callback { func: usize, context: usize },
}

#[derive(Clone, Debug)]
struct PowerNotification {
    kind: PowerNotifyKind,
    #[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
    target: PowerNotifyTarget,
}

static NEXT_NOTIFY_HANDLE: AtomicUsize = AtomicUsize::new(0x7050_0004);

fn notifications() -> &'static Mutex<HashMap<usize, PowerNotification>> {
    static M: OnceLock<Mutex<HashMap<usize, PowerNotification>>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(HashMap::new()))
}

fn host_model() -> String {
    let mut buf = [0u8; 128];
    let mut sz = buf.len();
    unsafe {
        if libc::sysctlbyname(
            c"hw.model".as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            &mut sz,
            std::ptr::null_mut(),
            0,
        ) == 0
            && sz > 0
        {
            let end = buf[..sz].iter().position(|&b| b == 0).unwrap_or(sz);
            return String::from_utf8_lossy(&buf[..end]).into_owned();
        }
    }
    String::new()
}

fn is_mobile_host() -> bool {
    if cfg!(target_os = "ios") {
        return true;
    }
    host_model().starts_with("MacBook")
}

fn host_cpu_count() -> u32 {
    std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(8)
}

fn host_cpu_mhz() -> u32 {
    let mut freq: u64 = 0;
    let mut sz = std::mem::size_of::<u64>();
    unsafe {
        if libc::sysctlbyname(
            c"hw.cpufrequency_max".as_ptr(),
            &mut freq as *mut u64 as *mut c_void,
            &mut sz,
            std::ptr::null_mut(),
            0,
        ) == 0
            && freq >= 1_000_000
        {
            return (freq / 1_000_000) as u32;
        }
    }
    // Apple Silicon does not expose `hw.cpufrequency_max`; report nominal 3200 MHz.
    3200
}

#[cfg(target_os = "macos")]
mod macos_power {
    use super::{BatterySnapshot, BATTERY_UNKNOWN_TIME};
    use std::ffi::{c_char, c_void, CStr};
    use std::sync::OnceLock;

    const K_CFSTRING_ENCODING_UTF8: u32 = 0x0800_0100;
    const K_CFNUMBER_SINT32: isize = 3;

    #[derive(Clone, Copy)]
    struct PowerApis {
        copy_ps_info: unsafe extern "C" fn() -> *const c_void,
        copy_ps_list: unsafe extern "C" fn(*const c_void) -> *const c_void,
        get_ps_desc: unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void,
        cf_release: unsafe extern "C" fn(*const c_void),
        cf_array_get_count: unsafe extern "C" fn(*const c_void) -> isize,
        cf_array_get_value_at_index: unsafe extern "C" fn(*const c_void, isize) -> *const c_void,
        cf_dict_get_value: unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void,
        cf_string_create: unsafe extern "C" fn(*const c_void, *const c_char, u32) -> *const c_void,
        cf_string_get_cstr: unsafe extern "C" fn(*const c_void, *mut c_char, isize, u32) -> u8,
        cf_number_get_value: unsafe extern "C" fn(*const c_void, isize, *mut c_void) -> u8,
        cf_boolean_get_value: unsafe extern "C" fn(*const c_void) -> u8,
        cf_get_type_id: unsafe extern "C" fn(*const c_void) -> usize,
        cf_string_type_id: unsafe extern "C" fn() -> usize,
        cf_number_type_id: unsafe extern "C" fn() -> usize,
        cf_boolean_type_id: unsafe extern "C" fn() -> usize,
    }

    fn apis() -> Option<&'static PowerApis> {
        static APIS: OnceLock<Option<PowerApis>> = OnceLock::new();
        APIS.get_or_init(|| unsafe {
            let iokit = libc::dlopen(
                c"/System/Library/Frameworks/IOKit.framework/IOKit".as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            );
            let cf = libc::dlopen(
                c"/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation".as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            );
            if iokit.is_null() || cf.is_null() {
                return None;
            }
            macro_rules! sym {
                ($lib:expr, $name:expr, $ty:ty) => {{
                    let p = libc::dlsym($lib, $name.as_ptr());
                    if p.is_null() {
                        return None;
                    }
                    std::mem::transmute::<*mut c_void, $ty>(p)
                }};
            }
            Some(PowerApis {
                copy_ps_info: sym!(iokit, c"IOPSCopyPowerSourcesInfo", unsafe extern "C" fn() -> *const c_void),
                copy_ps_list: sym!(iokit, c"IOPSCopyPowerSourcesList", unsafe extern "C" fn(*const c_void) -> *const c_void),
                get_ps_desc: sym!(iokit, c"IOPSGetPowerSourceDescription", unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void),
                cf_release: sym!(cf, c"CFRelease", unsafe extern "C" fn(*const c_void)),
                cf_array_get_count: sym!(cf, c"CFArrayGetCount", unsafe extern "C" fn(*const c_void) -> isize),
                cf_array_get_value_at_index: sym!(cf, c"CFArrayGetValueAtIndex", unsafe extern "C" fn(*const c_void, isize) -> *const c_void),
                cf_dict_get_value: sym!(cf, c"CFDictionaryGetValue", unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void),
                cf_string_create: sym!(cf, c"CFStringCreateWithCString", unsafe extern "C" fn(*const c_void, *const c_char, u32) -> *const c_void),
                cf_string_get_cstr: sym!(cf, c"CFStringGetCString", unsafe extern "C" fn(*const c_void, *mut c_char, isize, u32) -> u8),
                cf_number_get_value: sym!(cf, c"CFNumberGetValue", unsafe extern "C" fn(*const c_void, isize, *mut c_void) -> u8),
                cf_boolean_get_value: sym!(cf, c"CFBooleanGetValue", unsafe extern "C" fn(*const c_void) -> u8),
                cf_get_type_id: sym!(cf, c"CFGetTypeID", unsafe extern "C" fn(*const c_void) -> usize),
                cf_string_type_id: sym!(cf, c"CFStringGetTypeID", unsafe extern "C" fn() -> usize),
                cf_number_type_id: sym!(cf, c"CFNumberGetTypeID", unsafe extern "C" fn() -> usize),
                cf_boolean_type_id: sym!(cf, c"CFBooleanGetTypeID", unsafe extern "C" fn() -> usize),
            })
        })
        .as_ref()
    }

    unsafe fn dict_lookup(a: &PowerApis, dict: *const c_void, key: &CStr) -> *const c_void {
        let cf_key = (a.cf_string_create)(std::ptr::null(), key.as_ptr(), K_CFSTRING_ENCODING_UTF8);
        if cf_key.is_null() {
            return std::ptr::null();
        }
        let val = (a.cf_dict_get_value)(dict, cf_key);
        (a.cf_release)(cf_key);
        val
    }

    unsafe fn dict_string(a: &PowerApis, dict: *const c_void, key: &CStr) -> Option<String> {
        let v = dict_lookup(a, dict, key);
        if v.is_null() || (a.cf_get_type_id)(v) != (a.cf_string_type_id)() {
            return None;
        }
        let mut buf = [0 as c_char; 128];
        if (a.cf_string_get_cstr)(v, buf.as_mut_ptr(), buf.len() as isize, K_CFSTRING_ENCODING_UTF8) != 0 {
            Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned())
        } else {
            None
        }
    }

    unsafe fn dict_i32(a: &PowerApis, dict: *const c_void, key: &CStr) -> Option<i32> {
        let v = dict_lookup(a, dict, key);
        if v.is_null() || (a.cf_get_type_id)(v) != (a.cf_number_type_id)() {
            return None;
        }
        let mut out = 0i32;
        if (a.cf_number_get_value)(v, K_CFNUMBER_SINT32, &mut out as *mut i32 as *mut c_void) != 0 {
            Some(out)
        } else {
            None
        }
    }

    unsafe fn dict_bool(a: &PowerApis, dict: *const c_void, key: &CStr) -> Option<bool> {
        let v = dict_lookup(a, dict, key);
        if v.is_null() || (a.cf_get_type_id)(v) != (a.cf_boolean_type_id)() {
            return None;
        }
        Some((a.cf_boolean_get_value)(v) != 0)
    }

    pub(super) fn query_battery() -> Option<BatterySnapshot> {
        let a = apis()?;
        unsafe {
            let blob = (a.copy_ps_info)();
            if blob.is_null() {
                return None;
            }
            let list = (a.copy_ps_list)(blob);
            if list.is_null() {
                (a.cf_release)(blob);
                return None;
            }

            let count = (a.cf_array_get_count)(list);
            if count <= 0 {
                // Desktop Mac with no battery/UPS power sources -> AC line online, no battery.
                (a.cf_release)(list);
                (a.cf_release)(blob);
                return Some(BatterySnapshot {
                    ac_online: true,
                    battery_present: false,
                    charging: false,
                    discharging: false,
                    max_capacity: 0,
                    remaining_capacity: 0,
                    rate: 0,
                    estimated_time: BATTERY_UNKNOWN_TIME,
                });
            }

            let mut found: Option<BatterySnapshot> = None;
            for i in 0..count {
                let ps = (a.cf_array_get_value_at_index)(list, i);
                if ps.is_null() {
                    continue;
                }
                let desc = (a.get_ps_desc)(blob, ps);
                if desc.is_null() {
                    continue;
                }
                let present = dict_bool(a, desc, c"Is Present").unwrap_or(true);
                let state = dict_string(a, desc, c"Power Source State").unwrap_or_default();
                let ac_online = state != "Battery Power";
                let charging = dict_bool(a, desc, c"Is Charging").unwrap_or(false);
                let cur_cap = dict_i32(a, desc, c"Current Capacity").unwrap_or(100).max(0) as u32;
                let max_cap = dict_i32(a, desc, c"Max Capacity").unwrap_or(100).max(0) as u32;
                let tte_mins = dict_i32(a, desc, c"Time to Empty").unwrap_or(-1);
                let estimated_time = if !ac_online && tte_mins > 0 {
                    (tte_mins as u32).saturating_mul(60)
                } else {
                    BATTERY_UNKNOWN_TIME
                };
                let rate = dict_i32(a, desc, c"Current").unwrap_or(0);
                let discharging = present && !ac_online && !charging;
                found = Some(BatterySnapshot {
                    ac_online,
                    battery_present: present,
                    charging,
                    discharging,
                    max_capacity: max_cap,
                    remaining_capacity: if max_cap > 0 { cur_cap.min(max_cap) } else { cur_cap },
                    rate,
                    estimated_time,
                });
                if present {
                    break;
                }
            }

            (a.cf_release)(list);
            (a.cf_release)(blob);
            found
        }
    }

    // Background IOKit power-event & power-source notification loop.
    // Runs on its own dedicated thread + CFRunLoop so it never contends with
    // the AppKit main thread or any Win32 thread lock.
    pub(super) fn ensure_power_monitor_started() {
        if cfg!(test) {
            return;
        }
        static STARTED: OnceLock<()> = OnceLock::new();
        STARTED.get_or_init(|| {
            let _ = std::thread::Builder::new()
                .name("win32-power-monitor".into())
                .spawn(|| unsafe { run_iokit_power_loop() });
        });
    }

    unsafe fn run_iokit_power_loop() {
        let iokit = libc::dlopen(
            c"/System/Library/Frameworks/IOKit.framework/IOKit".as_ptr(),
            libc::RTLD_LAZY | libc::RTLD_LOCAL,
        );
        let cf = libc::dlopen(
            c"/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation".as_ptr(),
            libc::RTLD_LAZY | libc::RTLD_LOCAL,
        );
        if iokit.is_null() || cf.is_null() {
            return;
        }
        macro_rules! sym_opt {
            ($lib:expr, $name:expr, $ty:ty) => {{
                let p = libc::dlsym($lib, $name.as_ptr());
                if p.is_null() { None } else { Some(std::mem::transmute::<*mut c_void, $ty>(p)) }
            }};
        }

        type IoRegFn = unsafe extern "C" fn(
            *mut c_void,
            *mut *mut c_void,
            unsafe extern "C" fn(*mut c_void, u32, u32, *mut c_void),
            *mut u32,
        ) -> u32;
        type IoAllowFn = unsafe extern "C" fn(u32, isize) -> i32;
        type IoPortSourceFn = unsafe extern "C" fn(*mut c_void) -> *const c_void;
        type IopsNotifySourceFn =
            unsafe extern "C" fn(unsafe extern "C" fn(*mut c_void), *mut c_void) -> *const c_void;
        type CfRunLoopCurFn = unsafe extern "C" fn() -> *mut c_void;
        type CfRunLoopAddSrcFn = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void);
        type CfRunLoopRunFn = unsafe extern "C" fn();

        let (Some(io_reg), Some(io_allow), Some(io_port_src), Some(cf_cur), Some(cf_add), Some(cf_run)) = (
            sym_opt!(iokit, c"IORegisterForSystemPower", IoRegFn),
            sym_opt!(iokit, c"IOAllowPowerChange", IoAllowFn),
            sym_opt!(iokit, c"IONotificationPortGetRunLoopSource", IoPortSourceFn),
            sym_opt!(cf, c"CFRunLoopGetCurrent", CfRunLoopCurFn),
            sym_opt!(cf, c"CFRunLoopAddSource", CfRunLoopAddSrcFn),
            sym_opt!(cf, c"CFRunLoopRun", CfRunLoopRunFn),
        ) else {
            return;
        };
        let default_mode_sym = libc::dlsym(cf, c"kCFRunLoopDefaultMode".as_ptr());
        if default_mode_sym.is_null() {
            return;
        }
        let default_mode = *(default_mode_sym as *const *const c_void);

        struct RootCtx {
            root_port: u32,
            io_allow: IoAllowFn,
        }
        unsafe extern "C" fn sleep_cb(ctx: *mut c_void, _svc: u32, msg_type: u32, msg_arg: *mut c_void) {
            const K_IO_MSG_CAN_SYSTEM_SLEEP: u32 = 0xE000_0270;
            const K_IO_MSG_SYSTEM_WILL_SLEEP: u32 = 0xE000_0280;
            const K_IO_MSG_SYSTEM_HAS_POWERED_ON: u32 = 0xE000_0300;
            let r = &*(ctx as *const RootCtx);
            match msg_type {
                K_IO_MSG_CAN_SYSTEM_SLEEP => {
                    (r.io_allow)(r.root_port, msg_arg as isize);
                }
                K_IO_MSG_SYSTEM_WILL_SLEEP => {
                    super::dispatch_suspend_resume_event(super::PBT_APMSUSPEND);
                    (r.io_allow)(r.root_port, msg_arg as isize);
                }
                K_IO_MSG_SYSTEM_HAS_POWERED_ON => {
                    super::dispatch_suspend_resume_event(super::PBT_APMRESUMEAUTOMATIC);
                    super::dispatch_suspend_resume_event(super::PBT_APMRESUMESUSPEND);
                }
                _ => {}
            }
        }
        unsafe extern "C" fn ps_change_cb(_ctx: *mut c_void) {
            if let Some(snap) = query_battery() {
                let ac_val: u32 = if snap.ac_online { 0 } else { 1 };
                super::dispatch_power_setting_change(super::GUID_ACDC_POWER_SOURCE, &ac_val.to_le_bytes());
                let pct: u32 = if snap.battery_present && snap.max_capacity > 0 {
                    ((snap.remaining_capacity as u64 * 100) / snap.max_capacity as u64).min(100) as u32
                } else {
                    100
                };
                super::dispatch_power_setting_change(
                    super::GUID_BATTERY_PERCENTAGE_REMAINING,
                    &pct.to_le_bytes(),
                );
            }
        }

        let ctx = Box::into_raw(Box::new(RootCtx { root_port: 0, io_allow }));
        let mut notify_port: *mut c_void = std::ptr::null_mut();
        let mut notifier: u32 = 0;
        let root_port = io_reg(ctx as *mut c_void, &mut notify_port, sleep_cb, &mut notifier);
        let rl = cf_cur();
        let mut any_source = false;
        if root_port != 0 && !notify_port.is_null() {
            (*ctx).root_port = root_port;
            let src = io_port_src(notify_port);
            if !src.is_null() {
                cf_add(rl, src, default_mode);
                any_source = true;
            }
        }
        if let Some(iops_src_fn) = sym_opt!(iokit, c"IOPSNotificationCreateRunLoopSource", IopsNotifySourceFn) {
            let ps_src = iops_src_fn(ps_change_cb, std::ptr::null_mut());
            if !ps_src.is_null() {
                cf_add(rl, ps_src, default_mode);
                any_source = true;
            }
        }
        if any_source {
            cf_run();
        }
    }
}

#[cfg(target_os = "ios")]
fn query_battery_state() -> Option<BatterySnapshot> {
    use objc2::msg_send;
    use objc2::runtime::{AnyClass, AnyObject, Bool};
    unsafe {
        let cls = AnyClass::get(c"UIDevice")?;
        let dev: *mut AnyObject = msg_send![cls, currentDevice];
        if dev.is_null() {
            return None;
        }
        let enabled: Bool = msg_send![dev, isBatteryMonitoringEnabled];
        if !enabled.as_bool() {
            let _: () = msg_send![dev, setBatteryMonitoringEnabled: Bool::YES];
        }
        // UIDeviceBatteryState: 0 = Unknown, 1 = Unplugged, 2 = Charging, 3 = Full
        let state: isize = msg_send![dev, batteryState];
        let level: f32 = msg_send![dev, batteryLevel];
        if state == 0 && level < 0.0 {
            return None;
        }
        let max_capacity = 100u32;
        let remaining_capacity = if level >= 0.0 && level.is_finite() {
            (level.clamp(0.0, 1.0) * 100.0).round() as u32
        } else {
            100
        };
        let ac_online = matches!(state, 2 | 3);
        let charging = state == 2;
        let discharging = state == 1;
        Some(BatterySnapshot {
            ac_online,
            battery_present: true,
            charging,
            discharging,
            max_capacity,
            remaining_capacity,
            rate: 0,
            estimated_time: BATTERY_UNKNOWN_TIME,
        })
    }
}

#[cfg(target_os = "macos")]
fn query_battery_state() -> Option<BatterySnapshot> {
    macos_power::query_battery()
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn query_battery_state() -> Option<BatterySnapshot> {
    None
}

fn fallback_battery_state() -> BatterySnapshot {
    let mobile = is_mobile_host();
    BatterySnapshot {
        ac_online: true,
        battery_present: mobile,
        charging: false,
        discharging: false,
        max_capacity: if mobile { 100 } else { 0 },
        remaining_capacity: if mobile { 100 } else { 0 },
        rate: 0,
        estimated_time: BATTERY_UNKNOWN_TIME,
    }
}

pub unsafe extern "win64" fn PowerDeterminePlatformRole() -> u32 {
    if is_mobile_host() {
        PLATFORM_ROLE_MOBILE
    } else {
        PLATFORM_ROLE_DESKTOP
    }
}

pub unsafe extern "win64" fn PowerDeterminePlatformRoleEx(_version: u32) -> u32 {
    PowerDeterminePlatformRole()
}

pub unsafe extern "win64" fn GetPwrCapabilities(out: *mut c_void) -> u8 {
    if out.is_null() {
        return 0;
    }
    let bat = query_battery_state().unwrap_or_else(fallback_battery_state);
    // SYSTEM_POWER_CAPABILITIES is 76 bytes on Windows x64.
    let buf = out as *mut u8;
    std::ptr::write_bytes(buf, 0, 76);
    buf.add(0).write_unaligned(1); // PowerButtonPresent
    buf.add(1).write_unaligned(1); // SleepButtonPresent
    buf.add(2).write_unaligned(if is_mobile_host() { 1 } else { 0 }); // LidPresent
    buf.add(3).write_unaligned(1); // SystemS1
    buf.add(4).write_unaligned(1); // SystemS2
    buf.add(5).write_unaligned(1); // SystemS3
    buf.add(6).write_unaligned(1); // SystemS4
    buf.add(7).write_unaligned(1); // SystemS5
    buf.add(10).write_unaligned(if bat.battery_present { 1 } else { 0 }); // SystemBatteriesPresent
    buf.add(16).write_unaligned(1); // ProcessorThrottle
    buf.add(17).write_unaligned(5); // ProcessorMinThrottle
    buf.add(18).write_unaligned(100); // ProcessorMaxThrottle
    1
}

fn query_sysctl_timeval_100ns(name: &std::ffi::CStr) -> u64 {
    #[repr(C)]
    struct Timeval {
        tv_sec: i64,
        tv_usec: i32,
        _pad: i32,
    }
    let mut tv = Timeval { tv_sec: 0, tv_usec: 0, _pad: 0 };
    let mut sz = std::mem::size_of::<Timeval>();
    unsafe {
        if libc::sysctlbyname(
            name.as_ptr(),
            &mut tv as *mut Timeval as *mut c_void,
            &mut sz,
            std::ptr::null_mut(),
            0,
        ) == 0
            && tv.tv_sec > 0
        {
            return (tv.tv_sec as u64)
                .saturating_mul(10_000_000)
                .saturating_add((tv.tv_usec.max(0) as u64).saturating_mul(10));
        }
    }
    0
}

fn fill_default_system_power_policy(buf: *mut u8) {
    // SYSTEM_POWER_POLICY is 232 bytes on Windows x64.
    unsafe {
        std::ptr::write_bytes(buf, 0, 232);
        std::ptr::write_unaligned(buf.add(0) as *mut u32, 1); // Revision = 1
        std::ptr::write_unaligned(buf.add(104) as *mut u32, 300); // IdleTimeout = 300s
        std::ptr::write_unaligned(buf.add(108), 50u8); // IdleSensitivity = 50%
        std::ptr::write_unaligned(buf.add(109), 2u8); // DynamicThrottle = PO_THROTTLE_ADAPTIVE
        std::ptr::write_unaligned(buf.add(112) as *mut u32, 4); // MaxSleep = PowerSystemSleeping3
        std::ptr::write_unaligned(buf.add(116) as *mut u32, 2); // MinSleep = PowerSystemSleeping1
        std::ptr::write_unaligned(buf.add(120) as *mut u32, 600); // VideoTimeout = 600s
        std::ptr::write_unaligned(buf.add(124) as *mut u32, 1200); // SpindownTimeout = 1200s
    }
}

fn fill_default_processor_power_policy(buf: *mut u8) {
    // PROCESSOR_POWER_POLICY is 76 bytes on Windows x64 (Revision @0, DynamicThrottle @4, PolicyCount @20).
    unsafe {
        std::ptr::write_bytes(buf, 0, 76);
        std::ptr::write_unaligned(buf.add(0) as *mut u32, 1); // Revision = 1
        std::ptr::write_unaligned(buf.add(4), 2u8); // DynamicThrottle = PO_THROTTLE_ADAPTIVE
    }
}

pub unsafe extern "win64" fn CallNtPowerInformation(
    info_level: u32,
    in_buf: *const c_void,
    in_len: u32,
    out_buf: *mut c_void,
    out_len: u32,
) -> i32 {
    match info_level {
        SYSTEM_POWER_POLICY_AC | SYSTEM_POWER_POLICY_DC | SYSTEM_POWER_POLICY_CURRENT => {
            if out_buf.is_null() || out_len < 232 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            if !in_buf.is_null() && in_len >= 232 {
                std::ptr::copy_nonoverlapping(in_buf as *const u8, out_buf as *mut u8, 232);
            } else {
                fill_default_system_power_policy(out_buf as *mut u8);
            }
            STATUS_SUCCESS
        }
        VERIFY_SYSTEM_POLICY_AC | VERIFY_SYSTEM_POLICY_DC => {
            if in_buf.is_null() || in_len < 232 {
                return STATUS_INVALID_PARAMETER;
            }
            if out_buf.is_null() || out_len < 232 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            std::ptr::copy_nonoverlapping(in_buf as *const u8, out_buf as *mut u8, 232);
            STATUS_SUCCESS
        }
        SYSTEM_POWER_CAPABILITIES => {
            if out_buf.is_null() || out_len < 76 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            GetPwrCapabilities(out_buf);
            STATUS_SUCCESS
        }
        SYSTEM_BATTERY_STATE => {
            // SYSTEM_BATTERY_STATE is 32 bytes on Windows x64.
            if out_buf.is_null() || out_len < 32 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            let snap = query_battery_state().unwrap_or_else(fallback_battery_state);
            let buf = out_buf as *mut u8;
            std::ptr::write_bytes(buf, 0, 32);
            buf.add(0).write_unaligned(snap.ac_online as u8);
            buf.add(1).write_unaligned(snap.battery_present as u8);
            buf.add(2).write_unaligned(snap.charging as u8);
            buf.add(3).write_unaligned(snap.discharging as u8);
            buf.add(7).write_unaligned(if snap.battery_present { 1 } else { 0 }); // Tag
            (buf.add(8) as *mut u32).write_unaligned(snap.max_capacity);
            (buf.add(12) as *mut u32).write_unaligned(snap.remaining_capacity);
            (buf.add(16) as *mut u32).write_unaligned(snap.rate as u32);
            (buf.add(20) as *mut u32).write_unaligned(snap.estimated_time);
            (buf.add(24) as *mut u32).write_unaligned(snap.max_capacity / 10); // DefaultAlert1
            (buf.add(28) as *mut u32).write_unaligned(snap.max_capacity / 5); // DefaultAlert2
            STATUS_SUCCESS
        }
        PROCESSOR_INFORMATION => {
            // PROCESSOR_POWER_INFORMATION: 6 x u32 = 24 bytes per logical processor.
            const ENTRY_SZ: u32 = 24;
            let ncpus = host_cpu_count();
            let need = ncpus.saturating_mul(ENTRY_SZ);
            if out_buf.is_null() || out_len < need {
                return STATUS_BUFFER_TOO_SMALL;
            }
            let mhz = host_cpu_mhz();
            let base = out_buf as *mut u8;
            for i in 0..ncpus {
                let p = base.add((i * ENTRY_SZ) as usize);
                std::ptr::write_unaligned(p.add(0) as *mut u32, i); // Number
                std::ptr::write_unaligned(p.add(4) as *mut u32, mhz); // MaxMhz
                std::ptr::write_unaligned(p.add(8) as *mut u32, mhz); // CurrentMhz
                std::ptr::write_unaligned(p.add(12) as *mut u32, mhz); // MhzLimit
                std::ptr::write_unaligned(p.add(16) as *mut u32, 2); // MaxIdleState
                std::ptr::write_unaligned(p.add(20) as *mut u32, 0); // CurrentIdleState
            }
            STATUS_SUCCESS
        }
        SYSTEM_POWER_INFORMATION => {
            // SYSTEM_POWER_INFORMATION (16 bytes): MaxIdlenessAllowed, Idleness, TimeRemaining, CoolingMode
            if out_buf.is_null() || out_len < 16 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            let p = out_buf as *mut u8;
            std::ptr::write_bytes(p, 0, 16);
            std::ptr::write_unaligned(p.add(0) as *mut u32, 50); // MaxIdlenessAllowed
            std::ptr::write_unaligned(p.add(4) as *mut u32, 0); // Idleness
            std::ptr::write_unaligned(p.add(8) as *mut u32, 0); // TimeRemaining
            std::ptr::write_unaligned(p.add(12), 0u8); // CoolingMode = PO_TZ_ACTIVE
            STATUS_SUCCESS
        }
        LAST_WAKE_TIME => {
            if out_buf.is_null() || out_len < 8 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            let t = query_sysctl_timeval_100ns(c"kern.waketime");
            std::ptr::write_unaligned(out_buf as *mut u64, t);
            STATUS_SUCCESS
        }
        LAST_SLEEP_TIME => {
            if out_buf.is_null() || out_len < 8 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            let t = query_sysctl_timeval_100ns(c"kern.sleeptime");
            std::ptr::write_unaligned(out_buf as *mut u64, t);
            STATUS_SUCCESS
        }
        SYSTEM_EXECUTION_STATE => {
            if out_buf.is_null() || out_len < 4 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            std::ptr::write_unaligned(out_buf as *mut u32, 0);
            STATUS_SUCCESS
        }
        PROCESSOR_POWER_POLICY_AC | PROCESSOR_POWER_POLICY_DC | PROCESSOR_POWER_POLICY_CURRENT => {
            if out_buf.is_null() || out_len < 76 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            if !in_buf.is_null() && in_len >= 76 {
                std::ptr::copy_nonoverlapping(in_buf as *const u8, out_buf as *mut u8, 76);
            } else {
                fill_default_processor_power_policy(out_buf as *mut u8);
            }
            STATUS_SUCCESS
        }
        VERIFY_PROCESSOR_POLICY_AC | VERIFY_PROCESSOR_POLICY_DC => {
            if in_buf.is_null() || in_len < 76 {
                return STATUS_INVALID_PARAMETER;
            }
            if out_buf.is_null() || out_len < 76 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            std::ptr::copy_nonoverlapping(in_buf as *const u8, out_buf as *mut u8, 76);
            STATUS_SUCCESS
        }
        SYSTEM_WAKE_SOURCE | SYSTEM_HIBER_FILE_SIZE | SYSTEM_HIBER_FILE_INFORMATION => {
            if out_buf.is_null() || out_len < 4 {
                return STATUS_BUFFER_TOO_SMALL;
            }
            std::ptr::write_unaligned(out_buf as *mut u32, 0);
            STATUS_SUCCESS
        }
        SYSTEM_POWER_STATE_HANDLER
        | PROCESSOR_STATE_HANDLER
        | ADMINISTRATOR_POWER_POLICY
        | SYSTEM_RESERVE_HIBER_FILE
        | PROCESSOR_STATE_HANDLER2
        | SYSTEM_POWER_STATE_NOTIFY_HANDLER
        | SYSTEM_POWER_STATE_LOGGING
        | SYSTEM_POWER_LOGGING_ENTRY
        | SET_POWER_SETTING_VALUE => STATUS_ACCESS_DENIED,
        lvl if lvl < POWER_INFORMATION_LEVEL_MAX => STATUS_INVALID_PARAMETER,
        _ => STATUS_INVALID_PARAMETER,
    }
}

pub unsafe extern "win64" fn PowerGetActiveScheme(
    _user_root_power_key: *mut c_void,
    active_policy_guid: *mut *mut c_void,
) -> u32 {
    if active_policy_guid.is_null() {
        return err::ERROR_INVALID_PARAMETER;
    }
    let mem = crate::kernel32::memory::LocalAlloc(0, 16) as *mut u8;
    if mem.is_null() {
        return err::ERROR_NOT_ENOUGH_MEMORY;
    }
    std::ptr::copy_nonoverlapping(GUID_TYPICAL_POWER_SAVINGS.as_ptr(), mem, 16);
    std::ptr::write_unaligned(active_policy_guid, mem as *mut c_void);
    0 // ERROR_SUCCESS
}

pub unsafe extern "win64" fn PowerReadACValueIndex(
    _root_power_key: *mut c_void,
    _scheme_guid: *const c_void,
    _subgroup_guid: *const c_void,
    _setting_guid: *const c_void,
    ac_value_index: *mut u32,
) -> u32 {
    if !ac_value_index.is_null() {
        std::ptr::write_unaligned(ac_value_index, 0);
    }
    err::ERROR_FILE_NOT_FOUND
}

pub unsafe extern "win64" fn PowerReadDCValueIndex(
    _root_power_key: *mut c_void,
    _scheme_guid: *const c_void,
    _subgroup_guid: *const c_void,
    _setting_guid: *const c_void,
    dc_value_index: *mut u32,
) -> u32 {
    if !dc_value_index.is_null() {
        std::ptr::write_unaligned(dc_value_index, 0);
    }
    err::ERROR_FILE_NOT_FOUND
}

unsafe fn parse_notify_target(flags: u32, recipient: HANDLE) -> Result<PowerNotifyTarget, u32> {
    if recipient.is_null() {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    match flags {
        DEVICE_NOTIFY_WINDOW_HANDLE => Ok(PowerNotifyTarget::Window(recipient as usize)),
        DEVICE_NOTIFY_SERVICE_HANDLE => Ok(PowerNotifyTarget::Service),
        DEVICE_NOTIFY_CALLBACK => {
            // DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS: { PDEVICE_NOTIFY_CALLBACK_ROUTINE Callback; PVOID Context; }
            let p = recipient as *const usize;
            let func = std::ptr::read_unaligned(p);
            let context = std::ptr::read_unaligned(p.add(1));
            if func == 0 {
                return Err(err::ERROR_INVALID_PARAMETER);
            }
            Ok(PowerNotifyTarget::Callback { func, context })
        }
        _ => Err(err::ERROR_INVALID_PARAMETER),
    }
}

fn build_powerbroadcast_setting(guid: [u8; 16], data: &[u8]) -> Vec<u8> {
    // POWERBROADCAST_SETTING: GUID PowerSetting (@0, 16B), DWORD DataLength (@16, 4B), UCHAR Data[] (@20)
    let mut buf = Vec::with_capacity(20 + data.len().max(1));
    buf.extend_from_slice(&guid);
    buf.extend_from_slice(&(data.len() as u32).to_le_bytes());
    buf.extend_from_slice(data);
    if data.is_empty() {
        buf.push(0);
    }
    buf
}

fn current_power_setting_value(guid: [u8; 16]) -> Option<Vec<u8>> {
    if guid == GUID_ACDC_POWER_SOURCE {
        let snap = query_battery_state().unwrap_or_else(fallback_battery_state);
        let v: u32 = if snap.ac_online { 0 } else { 1 };
        Some(v.to_le_bytes().to_vec())
    } else if guid == GUID_BATTERY_PERCENTAGE_REMAINING {
        let snap = query_battery_state().unwrap_or_else(fallback_battery_state);
        let pct: u32 = if snap.battery_present && snap.max_capacity > 0 {
            ((snap.remaining_capacity as u64 * 100) / snap.max_capacity as u64).min(100) as u32
        } else {
            100
        };
        Some(pct.to_le_bytes().to_vec())
    } else if guid == GUID_POWERSCHEME_PERSONALITY {
        Some(GUID_TYPICAL_POWER_SAVINGS.to_vec())
    } else if guid == GUID_CONSOLE_DISPLAY_STATE || guid == GUID_MONITOR_POWER_ON {
        Some(1u32.to_le_bytes().to_vec()) // display on
    } else if guid == GUID_POWER_SAVING_STATUS {
        Some(0u32.to_le_bytes().to_vec()) // battery saver off
    } else {
        None
    }
}

unsafe fn deliver_to_target(target: &PowerNotifyTarget, pbt_type: u32, setting_buf: Option<&[u8]>) {
    match *target {
        PowerNotifyTarget::Callback { func, context } => {
            let setting_ptr = setting_buf.map_or(0u64, |b| b.as_ptr() as u64);
            let _ = crate::call_guest8(
                func,
                context as u64,
                pbt_type as u64,
                setting_ptr,
                0,
                0,
                0,
                0,
                0,
            );
        }
        PowerNotifyTarget::Window(hwnd) => {
            if let Some(w) = crate::user32::window::get(hwnd) {
                // Keep an owned copy of `POWERBROADCAST_SETTING` alive until the owning Win32
                // thread finishes running the window procedure (Windows delivers WM_POWERBROADCAST
                // as a sent notification so lParam stays valid across the WndProc call).
                let owned_pkt = setting_buf.map(|b| b.to_vec());
                let q = crate::user32::message::queue_for(w.tid);
                q.call(move || {
                    if crate::user32::window::exists(hwnd) {
                        let lp = owned_pkt.as_ref().map_or(0isize, |b| b.as_ptr() as isize);
                        crate::user32::window::call_window_proc(
                            hwnd,
                            crate::user32::wm::POWERBROADCAST,
                            pbt_type as usize,
                            lp,
                        );
                    }
                });
            }
        }
        PowerNotifyTarget::Service => {}
    }
}

#[cfg(any(target_os = "macos", test))]
pub(crate) fn dispatch_suspend_resume_event(pbt_type: u32) {
    let targets: Vec<PowerNotifyTarget> = {
        let map = notifications().lock().unwrap();
        map.values()
            .filter(|n| matches!(n.kind, PowerNotifyKind::SuspendResume))
            .map(|n| n.target.clone())
            .collect()
    };
    for t in targets {
        unsafe { deliver_to_target(&t, pbt_type, None) };
    }
}

#[cfg(any(target_os = "macos", test))]
pub(crate) fn dispatch_power_setting_change(guid: [u8; 16], data: &[u8]) {
    let targets: Vec<PowerNotifyTarget> = {
        let map = notifications().lock().unwrap();
        map.values()
            .filter(|n| matches!(n.kind, PowerNotifyKind::Setting(g) if g == guid))
            .map(|n| n.target.clone())
            .collect()
    };
    if targets.is_empty() {
        return;
    }
    let pkt = build_powerbroadcast_setting(guid, data);
    for t in targets {
        unsafe { deliver_to_target(&t, PBT_POWERSETTINGCHANGE, Some(&pkt)) };
    }
}

pub unsafe extern "win64" fn PowerRegisterSuspendResumeNotification(
    flags: u32,
    recipient: HANDLE,
    registration_handle: *mut *mut c_void,
) -> u32 {
    if registration_handle.is_null() {
        return err::ERROR_INVALID_PARAMETER;
    }
    let target = match parse_notify_target(flags, recipient) {
        Ok(t) => t,
        Err(e) => return e,
    };
    let handle = NEXT_NOTIFY_HANDLE.fetch_add(4, Ordering::Relaxed);
    notifications().lock().unwrap().insert(
        handle,
        PowerNotification { kind: PowerNotifyKind::SuspendResume, target },
    );
    #[cfg(target_os = "macos")]
    macos_power::ensure_power_monitor_started();
    std::ptr::write_unaligned(registration_handle, handle as *mut c_void);
    0 // ERROR_SUCCESS
}

pub unsafe extern "win64" fn PowerUnregisterSuspendResumeNotification(
    registration_handle: *mut c_void,
) -> u32 {
    if registration_handle.is_null() {
        return err::ERROR_INVALID_PARAMETER;
    }
    let handle = registration_handle as usize;
    let mut map = notifications().lock().unwrap();
    if matches!(
        map.get(&handle),
        Some(PowerNotification { kind: PowerNotifyKind::SuspendResume, .. })
    ) {
        map.remove(&handle);
        0
    } else {
        err::ERROR_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn PowerReadFriendlyName(
    _root_power_key: *mut c_void,
    _scheme_guid: *const c_void,
    _subgroup_guid: *const c_void,
    _setting_guid: *const c_void,
    buf: *mut u8,
    buf_size: *mut u32,
) -> u32 {
    if buf_size.is_null() {
        return err::ERROR_INVALID_PARAMETER;
    }
    let name: Vec<u16> = "Balanced\0".encode_utf16().collect();
    let need = (name.len() * 2) as u32;
    let cap = std::ptr::read_unaligned(buf_size);
    std::ptr::write_unaligned(buf_size, need);
    if buf.is_null() {
        return 0;
    }
    if cap < need {
        return err::ERROR_MORE_DATA;
    }
    std::ptr::copy_nonoverlapping(name.as_ptr() as *const u8, buf, need as usize);
    0
}

pub unsafe extern "win64" fn PowerSettingRegisterNotification(
    setting_guid: *const c_void,
    flags: u32,
    recipient: HANDLE,
    registration_handle: *mut *mut c_void,
) -> u32 {
    if setting_guid.is_null() || registration_handle.is_null() {
        return err::ERROR_INVALID_PARAMETER;
    }
    let target = match parse_notify_target(flags, recipient) {
        Ok(t) => t,
        Err(e) => return e,
    };
    let mut guid = [0u8; 16];
    std::ptr::copy_nonoverlapping(setting_guid as *const u8, guid.as_mut_ptr(), 16);
    let handle = NEXT_NOTIFY_HANDLE.fetch_add(4, Ordering::Relaxed);
    notifications().lock().unwrap().insert(
        handle,
        PowerNotification { kind: PowerNotifyKind::Setting(guid), target: target.clone() },
    );
    #[cfg(target_os = "macos")]
    macos_power::ensure_power_monitor_started();
    std::ptr::write_unaligned(registration_handle, handle as *mut c_void);

    // Windows immediately delivers the initial value of the registered power setting.
    if let Some(val) = current_power_setting_value(guid) {
        let pkt = build_powerbroadcast_setting(guid, &val);
        deliver_to_target(&target, PBT_POWERSETTINGCHANGE, Some(&pkt));
    }
    0
}

pub unsafe extern "win64" fn PowerSettingUnregisterNotification(
    registration_handle: *mut c_void,
) -> u32 {
    if registration_handle.is_null() {
        return err::ERROR_INVALID_PARAMETER;
    }
    let handle = registration_handle as usize;
    let mut map = notifications().lock().unwrap();
    if matches!(
        map.get(&handle),
        Some(PowerNotification { kind: PowerNotifyKind::Setting(_), .. })
    ) {
        map.remove(&handle);
        0
    } else {
        err::ERROR_INVALID_HANDLE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_role_and_processor_power_info() {
        unsafe {
            let role = PowerDeterminePlatformRoleEx(2);
            assert!(role == PLATFORM_ROLE_DESKTOP || role == PLATFORM_ROLE_MOBILE);

            let ncpus = host_cpu_count() as usize;
            let mut buf = vec![0u8; ncpus * 24];
            let st = CallNtPowerInformation(
                PROCESSOR_INFORMATION,
                std::ptr::null(),
                0,
                buf.as_mut_ptr() as *mut c_void,
                buf.len() as u32,
            );
            assert_eq!(st, STATUS_SUCCESS);
            let max_mhz = u32::from_le_bytes(buf[4..8].try_into().unwrap());
            assert!(max_mhz > 0);

            let mut guid_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(PowerGetActiveScheme(std::ptr::null_mut(), &mut guid_ptr), 0);
            assert!(!guid_ptr.is_null());
            let got = std::slice::from_raw_parts(guid_ptr as *const u8, 16);
            assert_eq!(got, &GUID_TYPICAL_POWER_SAVINGS);
            crate::kernel32::memory::LocalFree(guid_ptr);
        }
    }

    #[test]
    fn system_battery_state_queries_host_power() {
        unsafe {
            // Buffer too small must fail honestly.
            let mut small = [0u8; 16];
            assert_eq!(
                CallNtPowerInformation(
                    SYSTEM_BATTERY_STATE,
                    std::ptr::null(),
                    0,
                    small.as_mut_ptr() as *mut c_void,
                    small.len() as u32,
                ),
                STATUS_BUFFER_TOO_SMALL
            );

            let mut buf = [0u8; 32];
            assert_eq!(
                CallNtPowerInformation(
                    SYSTEM_BATTERY_STATE,
                    std::ptr::null(),
                    0,
                    buf.as_mut_ptr() as *mut c_void,
                    buf.len() as u32,
                ),
                STATUS_SUCCESS
            );
            let ac_online = buf[0];
            let bat_present = buf[1];
            let charging = buf[2];
            let discharging = buf[3];
            assert!(ac_online <= 1 && bat_present <= 1 && charging <= 1 && discharging <= 1);
            // Cannot simultaneously be charging and discharging.
            assert!(!(charging == 1 && discharging == 1));
            let max_cap = u32::from_le_bytes(buf[8..12].try_into().unwrap());
            let rem_cap = u32::from_le_bytes(buf[12..16].try_into().unwrap());
            if bat_present == 1 && max_cap > 0 {
                assert!(rem_cap <= max_cap);
            }
        }
    }

    #[test]
    fn power_notification_handle_lifecycle() {
        unsafe {
            let mut h1: *mut c_void = std::ptr::null_mut();
            let mut h2: *mut c_void = std::ptr::null_mut();
            assert_eq!(PowerRegisterSuspendResumeNotification(0, 0x1234 as HANDLE, &mut h1), 0);
            assert_eq!(
                PowerSettingRegisterNotification(
                    GUID_TYPICAL_POWER_SAVINGS.as_ptr() as *const c_void,
                    0,
                    0x5678 as HANDLE,
                    &mut h2,
                ),
                0
            );
            assert!(!h1.is_null() && !h2.is_null() && h1 != h2);

            // Cross-kind unregister must fail without removing the handle.
            assert_eq!(PowerSettingUnregisterNotification(h1), err::ERROR_INVALID_HANDLE);
            assert_eq!(PowerUnregisterSuspendResumeNotification(h2), err::ERROR_INVALID_HANDLE);

            // Proper unregister succeeds once, then double-unregister fails cleanly.
            assert_eq!(PowerUnregisterSuspendResumeNotification(h1), 0);
            assert_eq!(PowerUnregisterSuspendResumeNotification(h1), err::ERROR_INVALID_HANDLE);
            assert_eq!(PowerSettingUnregisterNotification(h2), 0);
            assert_eq!(PowerSettingUnregisterNotification(h2), err::ERROR_INVALID_HANDLE);
        }
    }

    #[test]
    fn call_nt_power_information_levels_and_callback_notifications() {
        unsafe {
            // 1. Additional CallNtPowerInformation levels
            let mut pol = [0u8; 232];
            assert_eq!(
                CallNtPowerInformation(
                    SYSTEM_POWER_POLICY_CURRENT,
                    std::ptr::null(),
                    0,
                    pol.as_mut_ptr() as *mut c_void,
                    pol.len() as u32,
                ),
                STATUS_SUCCESS
            );
            assert_eq!(u32::from_le_bytes(pol[0..4].try_into().unwrap()), 1); // Revision == 1

            let mut spi = [0u8; 16];
            assert_eq!(
                CallNtPowerInformation(
                    SYSTEM_POWER_INFORMATION,
                    std::ptr::null(),
                    0,
                    spi.as_mut_ptr() as *mut c_void,
                    spi.len() as u32,
                ),
                STATUS_SUCCESS
            );
            assert_eq!(u32::from_le_bytes(spi[0..4].try_into().unwrap()), 50);

            let mut wake_t = 0u64;
            assert_eq!(
                CallNtPowerInformation(
                    LAST_WAKE_TIME,
                    std::ptr::null(),
                    0,
                    &mut wake_t as *mut u64 as *mut c_void,
                    8,
                ),
                STATUS_SUCCESS
            );

            let mut ppol = [0u8; 76];
            assert_eq!(
                CallNtPowerInformation(
                    PROCESSOR_POWER_POLICY_CURRENT,
                    std::ptr::null(),
                    0,
                    ppol.as_mut_ptr() as *mut c_void,
                    ppol.len() as u32,
                ),
                STATUS_SUCCESS
            );
            assert_eq!(u32::from_le_bytes(ppol[0..4].try_into().unwrap()), 1);

            // Privileged admin-only level returns STATUS_ACCESS_DENIED; unknown returns STATUS_INVALID_PARAMETER.
            assert_eq!(
                CallNtPowerInformation(SYSTEM_RESERVE_HIBER_FILE, std::ptr::null(), 0, std::ptr::null_mut(), 0),
                STATUS_ACCESS_DENIED
            );
            assert_eq!(
                CallNtPowerInformation(999, std::ptr::null(), 0, std::ptr::null_mut(), 0),
                STATUS_INVALID_PARAMETER
            );

            // 2. DEVICE_NOTIFY_CALLBACK for PowerSettingRegisterNotification (immediate initial delivery)
            //    and PowerRegisterSuspendResumeNotification (dispatched suspend/resume events).
            static EVENTS: Mutex<Vec<(usize, u32, Option<u32>)>> = Mutex::new(Vec::new());
            unsafe extern "win64" fn power_cb(ctx: *mut c_void, pbt_type: u32, setting: *const u8) -> u32 {
                let val = if !setting.is_null() && pbt_type == PBT_POWERSETTINGCHANGE {
                    let len = std::ptr::read_unaligned(setting.add(16) as *const u32);
                    if len >= 4 {
                        Some(std::ptr::read_unaligned(setting.add(20) as *const u32))
                    } else {
                        None
                    }
                } else {
                    None
                };
                EVENTS.lock().unwrap().push((ctx as usize, pbt_type, val));
                0
            }
            crate::com::register_vtable_ptrs(&[power_cb as *const () as usize]);

            EVENTS.lock().unwrap().clear();
            let params: [usize; 2] = [power_cb as *const () as usize, 0xCAFE_BABE];
            let mut h_set: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                PowerSettingRegisterNotification(
                    GUID_ACDC_POWER_SOURCE.as_ptr() as *const c_void,
                    DEVICE_NOTIFY_CALLBACK,
                    params.as_ptr() as HANDLE,
                    &mut h_set,
                ),
                0
            );
            // Initial setting value must have been delivered immediately.
            {
                let evs = EVENTS.lock().unwrap();
                assert_eq!(evs.len(), 1);
                assert_eq!(evs[0].0, 0xCAFE_BABE);
                assert_eq!(evs[0].1, PBT_POWERSETTINGCHANGE);
                assert!(matches!(evs[0].2, Some(0) | Some(1)));
            }

            // Dispatch a battery percentage setting change and suspend/resume event.
            let mut h_sr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                PowerRegisterSuspendResumeNotification(
                    DEVICE_NOTIFY_CALLBACK,
                    params.as_ptr() as HANDLE,
                    &mut h_sr,
                ),
                0
            );
            dispatch_suspend_resume_event(PBT_APMSUSPEND);
            dispatch_suspend_resume_event(PBT_APMRESUMEAUTOMATIC);
            dispatch_power_setting_change(GUID_ACDC_POWER_SOURCE, &1u32.to_le_bytes());

            let evs = EVENTS.lock().unwrap().clone();
            assert_eq!(evs.len(), 4);
            assert_eq!(evs[1], (0xCAFE_BABE, PBT_APMSUSPEND, None));
            assert_eq!(evs[2], (0xCAFE_BABE, PBT_APMRESUMEAUTOMATIC, None));
            assert_eq!(evs[3], (0xCAFE_BABE, PBT_POWERSETTINGCHANGE, Some(1)));

            assert_eq!(PowerUnregisterSuspendResumeNotification(h_sr), 0);
            assert_eq!(PowerSettingUnregisterNotification(h_set), 0);
        }
    }

    #[test]
    fn window_power_setting_payload_stays_alive_until_wndproc_delivery() {
        static RECEIVED: Mutex<Vec<(usize, Option<([u8; 16], Vec<u8>)>)>> = Mutex::new(Vec::new());

        unsafe extern "win64" fn power_wndproc(
            hwnd: HWND,
            msg: u32,
            wparam: usize,
            lparam: isize,
        ) -> isize {
            if msg == crate::user32::wm::POWERBROADCAST {
                let setting = if wparam as u32 == PBT_POWERSETTINGCHANGE && lparam != 0 {
                    let p = lparam as *const u8;
                    let mut guid = [0u8; 16];
                    std::ptr::copy_nonoverlapping(p, guid.as_mut_ptr(), 16);
                    let len = std::ptr::read_unaligned(p.add(16) as *const u32) as usize;
                    let mut data = vec![0u8; len];
                    if len > 0 {
                        std::ptr::copy_nonoverlapping(p.add(20), data.as_mut_ptr(), len);
                    }
                    Some((guid, data))
                } else {
                    None
                };
                RECEIVED.lock().unwrap().push((wparam, setting));
                return 1;
            }
            crate::user32::defproc::DefWindowProcW(hwnd, msg, wparam, lparam)
        }

        std::thread::spawn(|| unsafe {
            let wndproc_handle = crate::user32::register_typed_host_wndproc(power_wndproc);
            RECEIVED.lock().unwrap().clear();

            let cls = crate::strings::to_wide_z("PowerNotifyPayloadTestWnd");
            let wc = crate::user32::class::WNDCLASSEX {
                cb_size: std::mem::size_of::<crate::user32::class::WNDCLASSEX>() as u32,
                style: 0,
                wndproc: wndproc_handle,
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
                icon_sm: 0,
            };
            let _ = crate::user32::class::RegisterClassExW(&wc);
            let hwnd = crate::user32::window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                crate::user32::ws::POPUP,
                0,
                0,
                40,
                40,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());

            // 1. Register window-based power-setting notification. Its local packet in
            //    PowerSettingRegisterNotification goes out of scope immediately upon return.
            let mut h_set: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                PowerSettingRegisterNotification(
                    GUID_ACDC_POWER_SOURCE.as_ptr() as *const c_void,
                    DEVICE_NOTIFY_WINDOW_HANDLE,
                    hwnd as HANDLE,
                    &mut h_set,
                ),
                0
            );
            // Clobber recently freed small heap blocks to expose any use-after-free.
            let _churn: Vec<Vec<u8>> = (0..64).map(|_| vec![0xDEu8; 24]).collect();

            let mut msg = crate::user32::MSG::default();
            let _ = crate::user32::message::PeekMessageW(
                &mut msg,
                hwnd,
                0,
                0,
                crate::user32::message::PM_REMOVE,
            );
            {
                let got = RECEIVED.lock().unwrap();
                assert_eq!(got.len(), 1, "expected initial WM_POWERBROADCAST delivery");
                assert_eq!(got[0].0, PBT_POWERSETTINGCHANGE as usize);
                let (guid, data) = got[0].1.as_ref().expect("missing POWERBROADCAST_SETTING");
                assert_eq!(*guid, GUID_ACDC_POWER_SOURCE);
                assert_eq!(data.len(), 4);
                let val = u32::from_le_bytes(data[..4].try_into().unwrap());
                assert!(val == 0 || val == 1, "unexpected initial AC/DC value {val:#x}");
            }

            // 2. Dispatch a subsequent change from another thread (mirrors IOKit callback thread).
            let payload_val = 0x1357_2468u32;
            std::thread::spawn(move || {
                dispatch_power_setting_change(GUID_ACDC_POWER_SOURCE, &payload_val.to_le_bytes());
            })
            .join()
            .unwrap();
            let _churn2: Vec<Vec<u8>> = (0..64).map(|_| vec![0xCCu8; 24]).collect();

            let _ = crate::user32::message::PeekMessageW(
                &mut msg,
                hwnd,
                0,
                0,
                crate::user32::message::PM_REMOVE,
            );
            {
                let got = RECEIVED.lock().unwrap();
                assert_eq!(got.len(), 2, "expected dispatched WM_POWERBROADCAST delivery");
                assert_eq!(got[1].0, PBT_POWERSETTINGCHANGE as usize);
                let (guid, data) = got[1].1.as_ref().expect("missing POWERBROADCAST_SETTING");
                assert_eq!(*guid, GUID_ACDC_POWER_SOURCE);
                assert_eq!(data, &payload_val.to_le_bytes());
            }

            // 3. Window-based suspend/resume notification delivers with lParam == 0.
            let mut h_sr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                PowerRegisterSuspendResumeNotification(
                    DEVICE_NOTIFY_WINDOW_HANDLE,
                    hwnd as HANDLE,
                    &mut h_sr,
                ),
                0
            );
            dispatch_suspend_resume_event(PBT_APMSUSPEND);
            let _ = crate::user32::message::PeekMessageW(
                &mut msg,
                hwnd,
                0,
                0,
                crate::user32::message::PM_REMOVE,
            );
            {
                let got = RECEIVED.lock().unwrap();
                assert_eq!(got.len(), 3);
                assert_eq!(got[2], (PBT_APMSUSPEND as usize, None));
            }

            assert_eq!(PowerUnregisterSuspendResumeNotification(h_sr), 0);
            assert_eq!(PowerSettingUnregisterNotification(h_set), 0);
            let _ = crate::user32::window::DestroyWindow(hwnd);
        })
        .join()
        .unwrap();
    }
}
