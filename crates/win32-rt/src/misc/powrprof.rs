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

// POWER_INFORMATION_LEVEL values
const SYSTEM_POWER_CAPABILITIES: u32 = 4;
const SYSTEM_BATTERY_STATE: u32 = 5;
const PROCESSOR_INFORMATION: u32 = 11;

const STATUS_SUCCESS: i32 = 0;
const STATUS_BUFFER_TOO_SMALL: i32 = 0xC000_0023u32 as i32;
const STATUS_NOT_IMPLEMENTED: i32 = 0xC000_0002u32 as i32;

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
#[allow(dead_code)]
struct PowerNotification {
    kind: PowerNotifyKind,
    flags: u32,
    recipient: usize,
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

pub unsafe extern "win64" fn CallNtPowerInformation(
    info_level: u32,
    _in_buf: *const c_void,
    _in_len: u32,
    out_buf: *mut c_void,
    out_len: u32,
) -> i32 {
    match info_level {
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
        _ => {
            stub!("CallNtPowerInformation(level={info_level}) -> STATUS_NOT_IMPLEMENTED");
            STATUS_NOT_IMPLEMENTED
        }
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

pub unsafe extern "win64" fn PowerRegisterSuspendResumeNotification(
    flags: u32,
    recipient: HANDLE,
    registration_handle: *mut *mut c_void,
) -> u32 {
    if registration_handle.is_null() {
        return err::ERROR_INVALID_PARAMETER;
    }
    let handle = NEXT_NOTIFY_HANDLE.fetch_add(4, Ordering::Relaxed);
    notifications().lock().unwrap().insert(
        handle,
        PowerNotification {
            kind: PowerNotifyKind::SuspendResume,
            flags,
            recipient: recipient as usize,
        },
    );
    stub!(
        "PowerRegisterSuspendResumeNotification(flags={flags:#x}) -> {handle:#x} (handle tracked; OS wake callback not armed)"
    );
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
    let mut guid = [0u8; 16];
    std::ptr::copy_nonoverlapping(setting_guid as *const u8, guid.as_mut_ptr(), 16);
    let handle = NEXT_NOTIFY_HANDLE.fetch_add(4, Ordering::Relaxed);
    notifications().lock().unwrap().insert(
        handle,
        PowerNotification {
            kind: PowerNotifyKind::Setting(guid),
            flags,
            recipient: recipient as usize,
        },
    );
    stub!(
        "PowerSettingRegisterNotification(flags={flags:#x}) -> {handle:#x} (handle tracked; OS setting callback not armed)"
    );
    std::ptr::write_unaligned(registration_handle, handle as *mut c_void);
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
}
