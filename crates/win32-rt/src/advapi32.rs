//! advapi32.dll
//!
//! * ETW (`Event*`): providers register fine; no trace session can ever be
//!   enabled, so writes are dropped exactly as on Windows with no listener.
//! * Event log (`RegisterEventSourceW`/`ReportEventW`): forwarded to `log`.
//! * Registry: empty, read-only for now. Opens fail with
//!   ERROR_FILE_NOT_FOUND (logged at debug so we can see what is queried);
//!   creating/writing keys is a STUB failure.
//! * Tokens/privileges: a process token exists; no privilege can be enabled
//!   (AdjustTokenPrivileges reports ERROR_NOT_ALL_ASSIGNED, like a standard
//!   user without SeLockMemoryPrivilege).
//! * `GetUserNameW`: the host user.
//! * CAPI (`Crypt*`) and the credential manager (`Cred*`): not implemented /
//!   empty store.

use crate::handles;
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub static EXPORTS: &[crate::Export] = include!(concat!(env!("OUT_DIR"), "/advapi32_exports.rs"));

const ERROR_SUCCESS: u32 = 0;
const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const ERROR_NO_MORE_ITEMS: u32 = 259;
const ERROR_NOT_FOUND: u32 = 1168;
const ERROR_NOT_ALL_ASSIGNED: u32 = 1300;
const ERROR_NO_SUCH_PRIVILEGE: u32 = 1313;

fn err<T>(e: u32, v: T) -> T {
    crate::teb::set_last_error(e);
    v
}

// ------------------------------------------------------------------- ETW

static NEXT_REG: AtomicU64 = AtomicU64::new(1);

/// `EventRegister(provider_guid, enable_cb, ctx, *REGHANDLE)`. The enable
/// callback is never invoked because no session exists.
pub unsafe extern "win64" fn EventRegister(guid: *const u8, _cb: *const c_void, _ctx: *mut c_void, handle: *mut u64) -> u32 {
    if handle.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let h = NEXT_REG.fetch_add(1, Ordering::Relaxed);
    *handle = h;
    if !guid.is_null() {
        log::debug!("EventRegister({}) -> {h} (no ETW sessions; events are dropped)", crate::misc::guid_string(guid));
    }
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn EventUnregister(_h: u64) -> u32 {
    ERROR_SUCCESS
}

/// Provider traits / decode guid: accepted, nothing consumes them.
pub unsafe extern "win64" fn EventSetInformation(_h: u64, _class: u32, _info: *const c_void, _len: u32) -> u32 {
    ERROR_SUCCESS
}

/// No session is listening, so the event is dropped (Windows behaves the
/// same and returns ERROR_SUCCESS).
pub unsafe extern "win64" fn EventWriteTransfer(_h: u64, _desc: *const c_void, _act: *const u8, _rel: *const u8, _n: u32, _data: *const c_void) -> u32 {
    ERROR_SUCCESS
}

// ------------------------------------------------------------- event log

pub unsafe extern "win64" fn RegisterEventSourceW(_server: LPCWSTR, source: LPCWSTR) -> HANDLE {
    let name: &'static str = Box::leak(crate::strings::from_wide_opt(source).unwrap_or_default().into_boxed_str());
    handles::insert(Arc::new(handles::Opaque(name)))
}

pub unsafe extern "win64" fn DeregisterEventSource(h: HANDLE) -> BOOL {
    if handles::close(h) {
        TRUE
    } else {
        err(ERROR_INVALID_HANDLE, FALSE)
    }
}

/// Logged through `log` (target `eventlog`).
pub unsafe extern "win64" fn ReportEventW(h: HANDLE, ty: u16, cat: u16, id: u32, _sid: *mut c_void, n: u16, _data_len: u32, strings: *const LPCWSTR, _data: *mut c_void) -> BOOL {
    let src = handles::get(h).map(|o| o.type_name()).unwrap_or("?");
    let mut msg = String::new();
    if !strings.is_null() {
        for i in 0..n as usize {
            if let Some(s) = crate::strings::from_wide_opt(*strings.add(i)) {
                if !msg.is_empty() {
                    msg.push_str(" | ");
                }
                msg.push_str(&s);
            }
        }
    }
    log::info!(target: "eventlog", "[{src}] type={ty:#x} cat={cat} id={id:#x}: {msg}");
    TRUE
}

// -------------------------------------------------------------- registry

const HKEY_CLASSES_ROOT: usize = 0x8000_0000;
const HKEY_CURRENT_CONFIG: usize = 0x8000_0005;
const ERROR_MORE_DATA: u32 = 234;
const REG_CREATED_NEW_KEY: u32 = 1;
const REG_OPENED_EXISTING_KEY: u32 = 2;
const REG_SZ: u32 = 1;
const REG_EXPAND_SZ: u32 = 2;
const REG_DWORD: u32 = 4;
const REG_QWORD: u32 = 11;

struct RegKey {
    path: String,
}

impl handles::KObject for RegKey {
    fn type_name(&self) -> &'static str {
        "RegKey"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
struct RegValue {
    name: String,
    ty: u32,
    data: Vec<u8>,
}

fn read_sysctl_string(name: &std::ffi::CStr) -> Option<String> {
    let mut len: libc::size_t = 0;
    if unsafe { libc::sysctlbyname(name.as_ptr(), std::ptr::null_mut(), &mut len, std::ptr::null_mut(), 0) } != 0
        || len == 0
    {
        return None;
    }
    let mut buf = vec![0u8; len];
    if unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return None;
    }
    let end = buf[..len].iter().position(|&b| b == 0).unwrap_or(len);
    let s = String::from_utf8_lossy(&buf[..end]).trim().to_string();
    (!s.is_empty()).then_some(s)
}

const REG_NOTIFY_CHANGE_NAME: u32 = 0x0000_0001;
const REG_NOTIFY_CHANGE_ATTRIBUTES: u32 = 0x0000_0002;
const REG_NOTIFY_CHANGE_LAST_SET: u32 = 0x0000_0004;
const REG_NOTIFY_CHANGE_SECURITY: u32 = 0x0000_0008;
const REG_NOTIFY_THREAD_AGNOSTIC: u32 = 0x1000_0000;

struct RegWatcher {
    key_handle: usize,
    path: String,
    watch_subtree: bool,
    filter: u32,
    event: Arc<handles::Event>,
}

struct Registry {
    keys: std::collections::BTreeMap<String, std::collections::BTreeMap<String, RegValue>>,
    watchers: Vec<RegWatcher>,
}

fn path_matches_watch(watch_path: &str, watch_subtree: bool, changed_path: &str) -> bool {
    if changed_path == watch_path {
        return true;
    }
    if watch_subtree {
        if let Some(rest) = changed_path.strip_prefix(watch_path) {
            return rest.starts_with('\\');
        }
    }
    false
}

fn parent_reg_path(path: &str) -> Option<&str> {
    path.rsplit_once('\\').map(|(p, _)| p)
}

fn signal_reg_events(events: Vec<Arc<handles::Event>>) {
    for ev in events {
        ev.set();
    }
}

impl Registry {
    fn take_triggered(&mut self, changed_paths: &[&str], mask: u32) -> Vec<Arc<handles::Event>> {
        let mut fired = Vec::new();
        let mut i = 0;
        while i < self.watchers.len() {
            let w = &self.watchers[i];
            let matched = (w.filter & mask) != 0
                && changed_paths
                    .iter()
                    .any(|p| path_matches_watch(&w.path, w.watch_subtree, p));
            if matched {
                fired.push(self.watchers.swap_remove(i).event);
            } else {
                i += 1;
            }
        }
        fired
    }

    fn take_key_closed(&mut self, key_handle: usize) -> Vec<Arc<handles::Event>> {
        let mut fired = Vec::new();
        let mut i = 0;
        while i < self.watchers.len() {
            if self.watchers[i].key_handle == key_handle {
                fired.push(self.watchers.swap_remove(i).event);
            } else {
                i += 1;
            }
        }
        fired
    }

    fn new() -> Self {
        let mut keys = std::collections::BTreeMap::new();
        for root in ["hkcr", "hkcu", "hklm", "hku", "hkpd", "hkcc"] {
            keys.insert(root.to_string(), std::collections::BTreeMap::new());
        }
        let mut reg = Self { keys, watchers: Vec::new() };
        // Pre-populate standard .NET Framework 4.8 & .NET 8.0 keys so games, launchers,
        // and installers checking for .NET prerequisites succeed automatically.
        for sub in [
            "hklm\\software\\microsoft\\net framework setup\\ndp\\v4\\full",
            "hklm\\software\\microsoft\\net framework setup\\ndp\\v4\\client",
            "hklm\\software\\wow6432node\\microsoft\\net framework setup\\ndp\\v4\\full",
            "hklm\\software\\wow6432node\\microsoft\\net framework setup\\ndp\\v4\\client",
        ] {
            reg.seed_dword(sub, "Install", 1);
            reg.seed_dword(sub, "Release", 528040); // .NET Framework 4.8
            reg.seed_sz(sub, "Version", "4.8.04084");
            reg.seed_sz(
                sub,
                "InstallPath",
                "C:\\Windows\\Microsoft.NET\\Framework64\\v4.0.30319\\",
            );
        }
        reg.seed_sz(
            "hklm\\software\\microsoft\\.netframework",
            "InstallRoot",
            "C:\\Windows\\Microsoft.NET\\Framework64\\",
        );
        reg.seed_sz(
            "hklm\\software\\dotnet\\setup\\installedversions\\x64",
            "InstallLocation",
            "C:\\Program Files\\dotnet\\",
        );
        reg.seed_dword(
            "hklm\\software\\dotnet\\setup\\installedversions\\x64\\sharedhost",
            "Version",
            1,
        );
        // Pre-populate Valve Steam ActiveProcess & SteamPath keys so Steamworks SDK
        // and DRM-free / Steam-stub games detect an active, logged-in Steam client.
        reg.seed_dword("hkcu\\software\\valve\\steam\\activeprocess", "pid", 4242);
        reg.seed_dword(
            "hkcu\\software\\valve\\steam\\activeprocess",
            "ActiveUser",
            0x0000_001A,
        );
        reg.seed_dword(
            "hkcu\\software\\valve\\steam\\activeprocess",
            "Universe",
            1,
        );
        reg.seed_sz(
            "hkcu\\software\\valve\\steam\\activeprocess",
            "SteamClientDll64",
            "C:\\Program Files (x86)\\Steam\\steamclient64.dll",
        );
        reg.seed_sz(
            "hkcu\\software\\valve\\steam",
            "SteamPath",
            "C:/Program Files (x86)/Steam",
        );
        reg.seed_sz(
            "hkcu\\software\\valve\\steam",
            "SteamExe",
            "C:/Program Files (x86)/Steam/steam.exe",
        );
        // Visual C++ 2015-2022 Redistributable (14.40) keys checked by Unreal/Capcom/EA launchers
        for vc_sub in [
            "hklm\\software\\microsoft\\visualstudio\\14.0\\vc\\runtimes\\x64",
            "hklm\\software\\wow6432node\\microsoft\\visualstudio\\14.0\\vc\\runtimes\\x64",
        ] {
            reg.seed_dword(vc_sub, "Installed", 1);
            reg.seed_dword(vc_sub, "Major", 14);
            reg.seed_dword(vc_sub, "Minor", 40);
            reg.seed_dword(vc_sub, "Bld", 33810);
        }
        reg.seed_sz(
            "hklm\\software\\dotnet\\setup\\installedversions\\x64\\sharedhost",
            "Path",
            "C:\\Program Files\\dotnet\\",
        );
        // DirectX End-User Runtime (June 2010) keys checked by Steam installscript.vdf & launchers
        for dx_sub in [
            "hklm\\software\\microsoft\\directx",
            "hklm\\software\\wow6432node\\microsoft\\directx",
        ] {
            reg.seed_sz(dx_sub, "Version", "4.09.00.0904");
            reg.seed_dword(dx_sub, "Installed", 1);
        }
        // Standard Windows 10 22H2 OS version, Cryptography, BIOS, and CPU hardware keys
        // queried by games, telemetry libraries, and launchers.
        let win_nt = "hklm\\software\\microsoft\\windows nt\\currentversion";
        reg.seed_sz(win_nt, "ProductName", "Windows 10 Pro");
        reg.seed_sz(win_nt, "EditionID", "Professional");
        reg.seed_sz(win_nt, "CurrentVersion", "6.3");
        reg.seed_dword(win_nt, "CurrentMajorVersionNumber", 10);
        reg.seed_dword(win_nt, "CurrentMinorVersionNumber", 0);
        reg.seed_sz(win_nt, "CurrentBuild", "19045");
        reg.seed_sz(win_nt, "CurrentBuildNumber", "19045");
        reg.seed_dword(win_nt, "UBR", 5011);
        reg.seed_sz(win_nt, "DisplayVersion", "22H2");
        reg.seed_sz(win_nt, "ReleaseId", "2009");
        reg.seed_sz(win_nt, "InstallationType", "Client");
        reg.seed_sz(win_nt, "SystemRoot", "C:\\Windows");
        reg.seed_sz(win_nt, "PathName", "C:\\Windows");
        reg.seed_sz(win_nt, "RegisteredOwner", "Player");

        reg.seed_sz(
            "hklm\\software\\microsoft\\cryptography",
            "MachineGuid",
            "7a2f1e04-8b5c-4d3a-9e12-6f4b8c2a1d90",
        );

        let hw_model = read_sysctl_string(c"hw.model").unwrap_or_else(|| "Apple Silicon".to_string());
        let cpu_brand = read_sysctl_string(c"machdep.cpu.brand_string")
            .unwrap_or_else(|| "Apple Silicon M-Series".to_string());

        let bios = "hklm\\hardware\\description\\system\\bios";
        reg.seed_sz(bios, "BIOSVendor", "Apple Inc.");
        reg.seed_sz(bios, "BIOSVersion", "1.0");
        reg.seed_sz(bios, "BIOSReleaseDate", "01/01/2024");
        reg.seed_sz(bios, "SystemManufacturer", "Apple Inc.");
        reg.seed_sz(bios, "SystemProductName", &hw_model);
        reg.seed_sz(bios, "SystemFamily", "Mac");
        reg.seed_sz(bios, "BaseBoardManufacturer", "Apple Inc.");
        reg.seed_sz(bios, "BaseBoardProduct", &hw_model);
        reg.seed_dword(bios, "EnclosureType", 3); // SMBIOS Desktop

        let ncpu = crate::process::num_cpus().max(1);
        for idx in 0..ncpu {
            let cpu_key = format!("hklm\\hardware\\description\\system\\centralprocessor\\{idx}");
            reg.seed_sz(&cpu_key, "ProcessorNameString", &cpu_brand);
            reg.seed_sz(&cpu_key, "VendorIdentifier", "GenuineIntel");
            reg.seed_sz(&cpu_key, "Identifier", "Intel64 Family 6 Model 142 Stepping 10");
            reg.seed_dword(&cpu_key, "~MHz", 3200);
            reg.seed_dword(&cpu_key, "FeatureSet", 0x3F3F_FFFF);
            reg.seed_binary(&cpu_key, "Update Revision", &[0, 0, 0, 0, 1, 0, 0, 0]);
        }

        let appmodel = "hklm\\software\\microsoft\\windows\\currentversion\\appmodelunlock";
        reg.seed_dword(appmodel, "AllowDevelopmentWithoutDevLicense", 0);
        reg.seed_dword(appmodel, "AllowAllTrustedApps", 1);

        // Standard Windows Display Adapter class registry key queried by games,
        // DXDiag/WMI-style hardware probes, and driver-version gates.
        let (gpu_name, gpu_dev_id, gpu_vram) = match crate::d3d12::mtl::device() {
            Some(d) => (d.name.as_str(), d.pci_device_id(), d.reported_vram()),
            None => ("Apple Silicon GPU", 0xc140, 8u64 << 30),
        };
        let matching_dev_id = format!(
            "pci\\ven_{:04x}&dev_{:04x}",
            crate::dxgi::adapter::VENDOR_APPLE,
            gpu_dev_id
        );
        let disp_class = "hklm\\system\\currentcontrolset\\control\\class\\{4d36e968-e325-11ce-bfc1-08002be10318}";
        reg.seed_sz(disp_class, "Class", "Display");
        let disp_adapter = format!("{disp_class}\\0000");
        reg.seed_sz(&disp_adapter, "DriverVersion", crate::dxgi::adapter::UMD_DRIVER_VERSION_STR);
        reg.seed_sz(&disp_adapter, "DriverDate", crate::dxgi::adapter::UMD_DRIVER_DATE_STR);
        // Windows FILETIME for 2026-11-01 00:00:00 UTC (matches DriverDate "11-01-2026").
        reg.seed_binary(&disp_adapter, "DriverDateData", &134_379_648_000_000_000u64.to_le_bytes());
        reg.seed_sz(&disp_adapter, "DriverDesc", gpu_name);
        reg.seed_sz(&disp_adapter, "ProviderName", "Apple Inc.");
        reg.seed_sz(&disp_adapter, "MatchingDeviceId", &matching_dev_id);
        reg.seed_sz(&disp_adapter, "HardwareInformation.AdapterString", gpu_name);
        reg.seed_sz(&disp_adapter, "HardwareInformation.ChipType", gpu_name);
        reg.seed_qword(&disp_adapter, "HardwareInformation.qwMemorySize", gpu_vram);
        reg.seed_dword(
            &disp_adapter,
            "HardwareInformation.MemorySize",
            gpu_vram.min(u32::MAX as u64) as u32,
        );

        // Automatically load any Steam post-install registry manifest (`steam_registry.tsv`
        // or `AW2_REGISTRY_TSV`) generated from `installscript.vdf`.
        reg.load_external_tsv();
        reg
    }

    fn load_external_tsv(&mut self) {
        let mut candidates = Vec::new();
        if let Ok(p) = std::env::var("AW2_REGISTRY_TSV") {
            candidates.push(std::path::PathBuf::from(p));
        }
        candidates.push(std::path::PathBuf::from("steam_registry.tsv"));
        for path in candidates {
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let cols: Vec<&str> = line.split('\t').collect();
                if cols.len() < 4 || cols[0].eq_ignore_ascii_case("key_path") {
                    continue;
                }
                let key_path = cols[0].trim().replace('/', "\\").to_ascii_lowercase();
                let val_name = cols[1].trim();
                let val_type = cols[2].trim().to_ascii_uppercase();
                let val_data = cols[3].trim();
                if val_type == "REG_DWORD" || val_type == "DWORD" {
                    let parsed = if let Some(hex) = val_data.strip_prefix("0x") {
                        u32::from_str_radix(hex, 16).unwrap_or(0)
                    } else {
                        val_data.parse::<u32>().unwrap_or(0)
                    };
                    self.seed_dword(&key_path, val_name, parsed);
                } else {
                    self.seed_sz(&key_path, val_name, val_data);
                }
            }
            break;
        }
    }

    fn ensure_hierarchy(&mut self, path: &str) {
        let mut cur = String::new();
        for seg in path.split('\\').filter(|s| !s.is_empty()) {
            if !cur.is_empty() {
                cur.push('\\');
            }
            cur.push_str(&seg.to_ascii_lowercase());
            self.keys.entry(cur.clone()).or_default();
        }
    }

    fn seed_dword(&mut self, key_path: &str, name: &str, val: u32) {
        self.ensure_hierarchy(key_path);
        let vals = self.keys.entry(key_path.to_ascii_lowercase()).or_default();
        vals.insert(
            name.to_ascii_lowercase(),
            RegValue {
                name: name.to_string(),
                ty: REG_DWORD,
                data: val.to_le_bytes().to_vec(),
            },
        );
    }

    fn seed_qword(&mut self, key_path: &str, name: &str, val: u64) {
        self.ensure_hierarchy(key_path);
        let vals = self.keys.entry(key_path.to_ascii_lowercase()).or_default();
        vals.insert(
            name.to_ascii_lowercase(),
            RegValue {
                name: name.to_string(),
                ty: REG_QWORD,
                data: val.to_le_bytes().to_vec(),
            },
        );
    }

    fn seed_sz(&mut self, key_path: &str, name: &str, val: &str) {
        self.ensure_hierarchy(key_path);
        let wide = crate::strings::to_wide_z(val);
        let bytes: Vec<u8> = wide.iter().flat_map(|w| w.to_le_bytes()).collect();
        let vals = self.keys.entry(key_path.to_ascii_lowercase()).or_default();
        vals.insert(
            name.to_ascii_lowercase(),
            RegValue {
                name: name.to_string(),
                ty: REG_SZ,
                data: bytes,
            },
        );
    }

    fn seed_binary(&mut self, key_path: &str, name: &str, data: &[u8]) {
        self.ensure_hierarchy(key_path);
        let vals = self.keys.entry(key_path.to_ascii_lowercase()).or_default();
        vals.insert(
            name.to_ascii_lowercase(),
            RegValue {
                name: name.to_string(),
                ty: 3, // REG_BINARY
                data: data.to_vec(),
            },
        );
    }
}

fn registry() -> &'static std::sync::Mutex<Registry> {
    static REG: std::sync::OnceLock<std::sync::Mutex<Registry>> = std::sync::OnceLock::new();
    REG.get_or_init(|| std::sync::Mutex::new(Registry::new()))
}

fn predefined_name(h: HANDLE) -> Option<&'static str> {
    let v = h as usize;
    // HKEYs are either zero-extended or sign-extended 32-bit values in 0x8000_0000..=0x8000_0005.
    let lo = v as u32 as usize;
    let hi = v >> 32;
    if !(hi == 0 || hi == 0xFFFF_FFFF) || !(HKEY_CLASSES_ROOT..=HKEY_CURRENT_CONFIG).contains(&lo) {
        return None;
    }
    Some(["HKCR", "HKCU", "HKLM", "HKU", "HKPD", "HKCC"][lo - HKEY_CLASSES_ROOT])
}

fn resolve_key_path(h: HANDLE) -> Option<String> {
    if let Some(root) = predefined_name(h) {
        return Some(root.to_ascii_lowercase());
    }
    handles::get_as::<RegKey>(h).map(|k| k.path.clone())
}

fn join_reg_path(base: &str, sub: &str) -> String {
    let mut out = base.to_string();
    for seg in sub.split(['\\', '/']).filter(|s| !s.is_empty()) {
        if !out.is_empty() {
            out.push('\\');
        }
        out.push_str(&seg.to_ascii_lowercase());
    }
    out
}

fn key_desc(h: HANDLE) -> String {
    if let Some(root) = predefined_name(h) {
        return root.to_string();
    }
    if let Some(k) = handles::get_as::<RegKey>(h) {
        return k.path.clone();
    }
    format!("{h:p}")
}

pub unsafe extern "win64" fn RegOpenKeyExW(key: HANDLE, sub: LPCWSTR, _opts: u32, _sam: u32, out: *mut HANDLE) -> u32 {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    let Some(base) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let sub_str = crate::strings::from_wide_opt(sub).unwrap_or_default();
    if sub_str.is_empty() && predefined_name(key).is_some() {
        if !out.is_null() {
            *out = key;
        }
        return ERROR_SUCCESS;
    }
    let full = join_reg_path(&base, &sub_str);
    let reg = registry().lock().unwrap();
    if reg.keys.contains_key(&full) {
        drop(reg);
        if !out.is_null() {
            *out = handles::insert(Arc::new(RegKey { path: full }));
        }
        return ERROR_SUCCESS;
    }
    log::debug!("RegOpenKeyExW({}\\{sub_str}) -> ERROR_FILE_NOT_FOUND", key_desc(key));
    ERROR_FILE_NOT_FOUND
}

pub unsafe extern "win64" fn RegOpenKeyExA(key: HANDLE, sub: LPCSTR, opts: u32, sam: u32, out: *mut HANDLE) -> u32 {
    let w = crate::strings::from_ansi_opt(sub).map(|s| crate::strings::to_wide_z(&s));
    RegOpenKeyExW(key, w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), opts, sam, out)
}

pub unsafe extern "win64" fn RegCreateKeyExW(key: HANDLE, sub: LPCWSTR, _r: u32, _class: LPWSTR, _opts: u32, _sam: u32, _sa: *mut c_void, out: *mut HANDLE, disp: *mut u32) -> u32 {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if !disp.is_null() {
        std::ptr::write_unaligned(disp, 0);
    }
    let Some(base) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let sub_str = crate::strings::from_wide_opt(sub).unwrap_or_default();
    let mut cur = base;
    let mut created_parents: Vec<String> = Vec::new();
    let mut reg = registry().lock().unwrap();
    let mut existed = reg.keys.contains_key(&cur);
    for seg in sub_str.split(['\\', '/']).filter(|s| !s.is_empty()) {
        let parent = cur.clone();
        cur.push('\\');
        cur.push_str(&seg.to_ascii_lowercase());
        existed = reg.keys.contains_key(&cur);
        if !existed {
            created_parents.push(parent);
            reg.keys.insert(cur.clone(), std::collections::BTreeMap::new());
        }
    }
    // Match the observed parent path of every newly created key (including intermediates)
    // in a single pass under the registry lock so each one-shot watcher fires at most once,
    // no-op reopens fire nothing, and direct vs subtree filters remain exact.
    let fired = if !created_parents.is_empty() {
        let refs: Vec<&str> = created_parents.iter().map(String::as_str).collect();
        reg.take_triggered(&refs, REG_NOTIFY_CHANGE_NAME)
    } else {
        Vec::new()
    };
    drop(reg);
    signal_reg_events(fired);
    if !disp.is_null() {
        std::ptr::write_unaligned(disp, if existed { REG_OPENED_EXISTING_KEY } else { REG_CREATED_NEW_KEY });
    }
    if !out.is_null() {
        std::ptr::write_unaligned(out, handles::insert(Arc::new(RegKey { path: cur.clone() })));
    }
    log::debug!("RegCreateKeyExW({}\\{sub_str}) -> {cur} (existed={existed})", key_desc(key));
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegCloseKey(key: HANDLE) -> u32 {
    if predefined_name(key).is_some() {
        ERROR_SUCCESS
    } else if handles::get_as::<RegKey>(key).is_some() && handles::close(key) {
        let fired = registry().lock().unwrap().take_key_closed(key as usize);
        signal_reg_events(fired);
        ERROR_SUCCESS
    } else {
        ERROR_INVALID_HANDLE
    }
}

unsafe fn query_value_bytes(path: &str, name: &str, ty: *mut u32, data: *mut u8, len: *mut u32, as_ansi: bool) -> u32 {
    let reg = registry().lock().unwrap();
    let Some(vals) = reg.keys.get(path) else {
        return ERROR_FILE_NOT_FOUND;
    };
    let Some(val) = vals.get(&name.to_ascii_lowercase()) else {
        return ERROR_FILE_NOT_FOUND;
    };
    if !ty.is_null() {
        *ty = val.ty;
    }
    let bytes: std::borrow::Cow<'_, [u8]> = if as_ansi && (val.ty == REG_SZ || val.ty == REG_EXPAND_SZ) {
        let u16s: Vec<u16> = val.data.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        let end = u16s.iter().position(|&c| c == 0).unwrap_or(u16s.len());
        let mut s = String::from_utf16_lossy(&u16s[..end]).into_bytes();
        s.push(0);
        std::borrow::Cow::Owned(s)
    } else {
        std::borrow::Cow::Borrowed(&val.data)
    };
    let needed = bytes.len() as u32;
    if data.is_null() {
        if !len.is_null() {
            *len = needed;
        }
        return ERROR_SUCCESS;
    }
    if len.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let cap = *len;
    *len = needed;
    if cap < needed {
        return ERROR_MORE_DATA;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), data, bytes.len());
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegQueryValueExW(key: HANDLE, name: LPCWSTR, _r: *mut u32, ty: *mut u32, data: *mut u8, len: *mut u32) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let name_str = crate::strings::from_wide_opt(name).unwrap_or_default();
    let rc = query_value_bytes(&path, &name_str, ty, data, len, false);
    if rc != ERROR_SUCCESS {
        log::debug!("RegQueryValueExW({path}, {name_str:?}) -> {rc}");
    }
    rc
}

pub unsafe extern "win64" fn RegQueryValueExA(key: HANDLE, name: LPCSTR, _r: *mut u32, ty: *mut u32, data: *mut u8, len: *mut u32) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let name_str = crate::strings::from_ansi_opt(name).unwrap_or_default();
    let rc = query_value_bytes(&path, &name_str, ty, data, len, true);
    if rc != ERROR_SUCCESS {
        log::debug!("RegQueryValueExA({path}, {name_str:?}) -> {rc}");
    }
    rc
}

#[repr(C)]
pub struct VALENTW {
    pub ve_valuename: LPWSTR,
    pub ve_valuelen: u32,
    pub _pad0: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
    pub _pad1: u32,
}

#[repr(C)]
pub struct VALENTA {
    pub ve_valuename: LPSTR,
    pub ve_valuelen: u32,
    pub _pad0: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
    pub _pad1: u32,
}

pub unsafe extern "win64" fn RegQueryMultipleValuesW(
    key: HANDLE,
    val_list: *mut VALENTW,
    num_vals: u32,
    buf: *mut u8,
    totsize: *mut u32,
) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    if val_list.is_null() || totsize.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let cap = std::ptr::read_unaligned(totsize) as usize;
    let mut need: usize = 0;
    let mut items: Vec<(u32, Vec<u8>)> = Vec::with_capacity(num_vals as usize);
    for i in 0..num_vals as usize {
        let ent = std::ptr::read_unaligned(val_list.add(i));
        let name = crate::strings::from_wide_opt(ent.ve_valuename).unwrap_or_default();
        let mut ty = 0u32;
        let mut len = 0u32;
        let rc = query_value_bytes(&path, &name, &mut ty, std::ptr::null_mut(), &mut len, false);
        if rc != ERROR_SUCCESS && rc != ERROR_MORE_DATA {
            return rc;
        }
        let mut data = vec![0u8; len as usize];
        let mut len2 = len;
        let rc2 = query_value_bytes(&path, &name, &mut ty, data.as_mut_ptr(), &mut len2, false);
        if rc2 != ERROR_SUCCESS {
            return rc2;
        }
        need += data.len();
        items.push((ty, data));
    }
    std::ptr::write_unaligned(totsize, need as u32);
    if buf.is_null() || cap < need {
        return ERROR_MORE_DATA;
    }
    let mut off = 0usize;
    for (i, (ty, data)) in items.into_iter().enumerate() {
        let mut ent = std::ptr::read_unaligned(val_list.add(i));
        let dst = buf.add(off);
        std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
        ent.ve_valuelen = data.len() as u32;
        ent.ve_valueptr = dst as usize;
        ent.ve_type = ty;
        std::ptr::write_unaligned(val_list.add(i), ent);
        off += data.len();
    }
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegQueryMultipleValuesA(
    key: HANDLE,
    val_list: *mut VALENTA,
    num_vals: u32,
    buf: *mut u8,
    totsize: *mut u32,
) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    if val_list.is_null() || totsize.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let cap = std::ptr::read_unaligned(totsize) as usize;
    let mut need: usize = 0;
    let mut items: Vec<(u32, Vec<u8>)> = Vec::with_capacity(num_vals as usize);
    for i in 0..num_vals as usize {
        let ent = std::ptr::read_unaligned(val_list.add(i));
        let name = crate::strings::from_ansi_opt(ent.ve_valuename).unwrap_or_default();
        let mut ty = 0u32;
        let mut len = 0u32;
        let rc = query_value_bytes(&path, &name, &mut ty, std::ptr::null_mut(), &mut len, true);
        if rc != ERROR_SUCCESS && rc != ERROR_MORE_DATA {
            return rc;
        }
        let mut data = vec![0u8; len as usize];
        let mut len2 = len;
        let rc2 = query_value_bytes(&path, &name, &mut ty, data.as_mut_ptr(), &mut len2, true);
        if rc2 != ERROR_SUCCESS {
            return rc2;
        }
        need += data.len();
        items.push((ty, data));
    }
    std::ptr::write_unaligned(totsize, need as u32);
    if buf.is_null() || cap < need {
        return ERROR_MORE_DATA;
    }
    let mut off = 0usize;
    for (i, (ty, data)) in items.into_iter().enumerate() {
        let mut ent = std::ptr::read_unaligned(val_list.add(i));
        let dst = buf.add(off);
        std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
        ent.ve_valuelen = data.len() as u32;
        ent.ve_valueptr = dst as usize;
        ent.ve_type = ty;
        std::ptr::write_unaligned(val_list.add(i), ent);
        off += data.len();
    }
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegDeleteKeyW(key: HANDLE, sub: LPCWSTR) -> u32 {
    RegDeleteKeyExW(key, sub, 0, 0)
}

pub unsafe extern "win64" fn RegDeleteKeyA(key: HANDLE, sub: LPCSTR) -> u32 {
    let w = crate::strings::from_ansi_opt(sub).map(|s| crate::strings::to_wide_z(&s));
    RegDeleteKeyExW(key, w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), 0, 0)
}

pub unsafe extern "win64" fn RegDeleteKeyExA(key: HANDLE, sub: LPCSTR, sam: u32, r: u32) -> u32 {
    let w = crate::strings::from_ansi_opt(sub).map(|s| crate::strings::to_wide_z(&s));
    RegDeleteKeyExW(key, w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), sam, r)
}

pub unsafe extern "win64" fn RegDeleteTreeA(key: HANDLE, sub: LPCSTR) -> u32 {
    let w = crate::strings::from_ansi_opt(sub).map(|s| crate::strings::to_wide_z(&s));
    RegDeleteTreeW(key, w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()))
}

pub unsafe extern "win64" fn RegGetValueW(key: HANDLE, sub: LPCWSTR, name: LPCWSTR, _flags: u32, ty: *mut u32, data: *mut c_void, len: *mut u32) -> u32 {
    let Some(base) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let sub_str = crate::strings::from_wide_opt(sub).unwrap_or_default();
    let full = join_reg_path(&base, &sub_str);
    let name_str = crate::strings::from_wide_opt(name).unwrap_or_default();
    let rc = query_value_bytes(&full, &name_str, ty, data as *mut u8, len, false);
    if rc != ERROR_SUCCESS {
        if !ty.is_null() {
            *ty = 0;
        }
        if !len.is_null() && data.is_null() {
            *len = 0;
        }
        log::debug!("RegGetValueW({full}, {name_str:?}) -> {rc}");
    }
    rc
}

pub unsafe extern "win64" fn RegSetValueExW(key: HANDLE, name: LPCWSTR, _r: u32, ty: u32, data: *const u8, len: u32) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let name_str = crate::strings::from_wide_opt(name).unwrap_or_default();
    let bytes = if data.is_null() || len == 0 {
        Vec::new()
    } else {
        std::slice::from_raw_parts(data, len as usize).to_vec()
    };
    let mut reg = registry().lock().unwrap();
    let vals = reg.keys.entry(path.clone()).or_default();
    vals.insert(name_str.to_ascii_lowercase(), RegValue { name: name_str.clone(), ty, data: bytes });
    let fired = reg.take_triggered(&[&path], REG_NOTIFY_CHANGE_LAST_SET);
    drop(reg);
    signal_reg_events(fired);
    log::debug!("RegSetValueExW({path}, {name_str:?}, ty={ty}, len={len}) -> ERROR_SUCCESS");
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegFlushKey(key: HANDLE) -> u32 {
    if resolve_key_path(key).is_some() {
        ERROR_SUCCESS
    } else {
        ERROR_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn RegNotifyChangeKeyValue(
    key: HANDLE,
    watch_subtree: BOOL,
    filter: u32,
    event: HANDLE,
    async_flag: BOOL,
) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let valid_mask = REG_NOTIFY_CHANGE_NAME
        | REG_NOTIFY_CHANGE_ATTRIBUTES
        | REG_NOTIFY_CHANGE_LAST_SET
        | REG_NOTIFY_CHANGE_SECURITY
        | REG_NOTIFY_THREAD_AGNOSTIC;
    let notify_bits = filter & !REG_NOTIFY_THREAD_AGNOSTIC;
    if notify_bits == 0 || (filter & !valid_mask) != 0 {
        return ERROR_INVALID_PARAMETER;
    }
    if async_flag != 0 {
        let Some(ev) = handles::get_as::<handles::Event>(event) else {
            return ERROR_INVALID_PARAMETER;
        };
        let mut reg = registry().lock().unwrap();
        reg.watchers.push(RegWatcher {
            key_handle: key as usize,
            path,
            watch_subtree: watch_subtree != 0,
            filter: notify_bits,
            event: ev,
        });
        return ERROR_SUCCESS;
    }
    // Synchronous mode: register a one-shot internal event and wait until signaled by a matching mutation or RegCloseKey.
    let ev = Arc::new(handles::Event::new(true, false, None));
    {
        let mut reg = registry().lock().unwrap();
        reg.watchers.push(RegWatcher {
            key_handle: key as usize,
            path,
            watch_subtree: watch_subtree != 0,
            filter: notify_bits,
            event: Arc::clone(&ev),
        });
    }
    let obj: Arc<dyn handles::KObject> = ev;
    let _ = handles::wait_for(&[obj], false, INFINITE, false);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegDeleteKeyExW(key: HANDLE, sub: LPCWSTR, _sam: u32, _r: u32) -> u32 {
    let Some(base) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let sub_str = crate::strings::from_wide_opt(sub).unwrap_or_default();
    if sub_str.is_empty() {
        return ERROR_INVALID_PARAMETER;
    }
    let full = join_reg_path(&base, &sub_str);
    let mut reg = registry().lock().unwrap();
    if reg.keys.remove(&full).is_some() {
        let fired = if let Some(parent) = parent_reg_path(&full) {
            reg.take_triggered(&[&full, parent], REG_NOTIFY_CHANGE_NAME)
        } else {
            reg.take_triggered(&[&full], REG_NOTIFY_CHANGE_NAME)
        };
        drop(reg);
        signal_reg_events(fired);
        ERROR_SUCCESS
    } else {
        ERROR_FILE_NOT_FOUND
    }
}

pub unsafe extern "win64" fn RegDeleteTreeW(key: HANDLE, sub: LPCWSTR) -> u32 {
    let Some(base) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let sub_str = crate::strings::from_wide_opt(sub).unwrap_or_default();
    let full = join_reg_path(&base, &sub_str);
    let prefix = format!("{full}\\");
    let mut reg = registry().lock().unwrap();
    if !reg.keys.contains_key(&full) {
        return ERROR_FILE_NOT_FOUND;
    }
    reg.keys.retain(|k, _| !k.starts_with(&prefix));
    if !sub_str.is_empty() {
        reg.keys.remove(&full);
    } else if let Some(vals) = reg.keys.get_mut(&full) {
        vals.clear();
    }
    let fired = if let Some(parent) = parent_reg_path(&full) {
        reg.take_triggered(&[&full, parent], REG_NOTIFY_CHANGE_NAME | REG_NOTIFY_CHANGE_LAST_SET)
    } else {
        reg.take_triggered(&[&full], REG_NOTIFY_CHANGE_NAME | REG_NOTIFY_CHANGE_LAST_SET)
    };
    drop(reg);
    signal_reg_events(fired);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegCreateKeyTransactedW(
    key: HANDLE,
    sub: LPCWSTR,
    r: u32,
    class: LPWSTR,
    opts: u32,
    sam: u32,
    sa: *mut c_void,
    out: *mut HANDLE,
    disp: *mut u32,
    _txn: HANDLE,
    _ext: *mut c_void,
) -> u32 {
    RegCreateKeyExW(key, sub, r, class, opts, sam, sa, out, disp)
}

fn direct_subkeys(reg: &Registry, path: &str) -> Vec<String> {
    let prefix = format!("{path}\\");
    let mut subkeys = Vec::new();
    for k in reg.keys.keys() {
        if let Some(rest) = k.strip_prefix(&prefix) {
            if !rest.is_empty() && !rest.contains('\\') {
                subkeys.push(rest.to_string());
            }
        }
    }
    subkeys
}

pub unsafe extern "win64" fn RegEnumKeyExW(key: HANDLE, idx: u32, name: LPWSTR, cch: *mut u32, _r: *mut u32, _class: LPWSTR, _ccls: *mut u32, _ft: *mut u64) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let reg = registry().lock().unwrap();
    let subkeys = direct_subkeys(&reg, &path);
    let Some(sub) = subkeys.get(idx as usize) else {
        return ERROR_NO_MORE_ITEMS;
    };
    if cch.is_null() || name.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let wide: Vec<u16> = sub.encode_utf16().collect();
    if (*cch as usize) <= wide.len() {
        return ERROR_MORE_DATA;
    }
    std::ptr::copy_nonoverlapping(wide.as_ptr(), name, wide.len());
    *name.add(wide.len()) = 0;
    *cch = wide.len() as u32;
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegEnumKeyExA(key: HANDLE, idx: u32, name: LPSTR, cch: *mut u32, _r: *mut u32, _class: LPSTR, _ccls: *mut u32, _ft: *mut u64) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let reg = registry().lock().unwrap();
    let subkeys = direct_subkeys(&reg, &path);
    let Some(sub) = subkeys.get(idx as usize) else {
        return ERROR_NO_MORE_ITEMS;
    };
    if cch.is_null() || name.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let bytes = sub.as_bytes();
    if (*cch as usize) <= bytes.len() {
        return ERROR_MORE_DATA;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), name, bytes.len());
    *name.add(bytes.len()) = 0;
    *cch = bytes.len() as u32;
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegEnumValueW(
    key: HANDLE,
    idx: u32,
    name: LPWSTR,
    cch: *mut u32,
    _r: *mut u32,
    ty: *mut u32,
    data: *mut u8,
    cb: *mut u32,
) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let reg = registry().lock().unwrap();
    let Some(vals) = reg.keys.get(&path) else {
        return ERROR_NO_MORE_ITEMS;
    };
    let Some((_, val)) = vals.iter().nth(idx as usize) else {
        return ERROR_NO_MORE_ITEMS;
    };
    if cch.is_null() || name.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let wide: Vec<u16> = val.name.encode_utf16().collect();
    if (*cch as usize) <= wide.len() {
        return ERROR_MORE_DATA;
    }
    std::ptr::copy_nonoverlapping(wide.as_ptr(), name, wide.len());
    *name.add(wide.len()) = 0;
    *cch = wide.len() as u32;
    if !ty.is_null() {
        *ty = val.ty;
    }
    if !cb.is_null() {
        let needed = val.data.len() as u32;
        if !data.is_null() {
            if *cb < needed {
                *cb = needed;
                return ERROR_MORE_DATA;
            }
            std::ptr::copy_nonoverlapping(val.data.as_ptr(), data, val.data.len());
        }
        *cb = needed;
    }
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegEnumValueA(
    key: HANDLE,
    idx: u32,
    name: LPSTR,
    cch: *mut u32,
    _r: *mut u32,
    ty: *mut u32,
    data: *mut u8,
    cb: *mut u32,
) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let reg = registry().lock().unwrap();
    let Some(vals) = reg.keys.get(&path) else {
        return ERROR_NO_MORE_ITEMS;
    };
    let Some((_, val)) = vals.iter().nth(idx as usize) else {
        return ERROR_NO_MORE_ITEMS;
    };
    if cch.is_null() || name.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let bytes = val.name.as_bytes();
    if (*cch as usize) <= bytes.len() {
        return ERROR_MORE_DATA;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), name, bytes.len());
    *name.add(bytes.len()) = 0;
    *cch = bytes.len() as u32;
    if !ty.is_null() {
        *ty = val.ty;
    }
    if !cb.is_null() {
        let needed = val.data.len() as u32;
        if !data.is_null() {
            if *cb < needed {
                *cb = needed;
                return ERROR_MORE_DATA;
            }
            std::ptr::copy_nonoverlapping(val.data.as_ptr(), data, val.data.len());
        }
        *cb = needed;
    }
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn RegQueryInfoKeyW(
    key: HANDLE,
    class: LPWSTR,
    ccls: *mut u32,
    _r: *mut u32,
    subkeys: *mut u32,
    max_subkey: *mut u32,
    max_class: *mut u32,
    values: *mut u32,
    max_value_name: *mut u32,
    max_value_len: *mut u32,
    sec_desc: *mut u32,
    ft: *mut u64,
) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let reg = registry().lock().unwrap();
    let Some(vals) = reg.keys.get(&path) else {
        return ERROR_FILE_NOT_FOUND;
    };
    let subs = direct_subkeys(&reg, &path);
    if !class.is_null() && !ccls.is_null() && *ccls > 0 {
        *class = 0;
    }
    for p in [ccls, max_class, sec_desc] {
        if !p.is_null() {
            *p = 0;
        }
    }
    if !subkeys.is_null() {
        *subkeys = subs.len() as u32;
    }
    if !max_subkey.is_null() {
        *max_subkey = subs.iter().map(|s| s.encode_utf16().count() as u32).max().unwrap_or(0);
    }
    if !values.is_null() {
        *values = vals.len() as u32;
    }
    if !max_value_name.is_null() {
        *max_value_name = vals.values().map(|v| v.name.encode_utf16().count() as u32).max().unwrap_or(0);
    }
    if !max_value_len.is_null() {
        *max_value_len = vals.values().map(|v| v.data.len() as u32).max().unwrap_or(0);
    }
    if !ft.is_null() {
        *ft = 0;
    }
    ERROR_SUCCESS
}

// ------------------------------------------------------- tokens / privs

static NEXT_LUID: AtomicU64 = AtomicU64::new(0x1000);

pub unsafe extern "win64" fn AllocateLocallyUniqueId(luid: *mut u64) -> BOOL {
    if luid.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    *luid = NEXT_LUID.fetch_add(1, Ordering::Relaxed);
    TRUE
}

pub unsafe extern "win64" fn PrivilegeCheck(token: HANDLE, _privs: *mut c_void, result: *mut BOOL) -> BOOL {
    if handles::get(token).is_none() {
        return err(ERROR_INVALID_HANDLE, FALSE);
    }
    if result.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    *result = FALSE;
    TRUE
}

pub unsafe extern "win64" fn OpenProcessToken(_proc: HANDLE, _access: u32, out: *mut HANDLE) -> BOOL {
    if out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    *out = handles::insert(Arc::new(handles::Opaque("Token")));
    TRUE
}

pub unsafe extern "win64" fn OpenThreadToken(
    _thread: HANDLE,
    _access: u32,
    _open_as_self: BOOL,
    out: *mut HANDLE,
) -> BOOL {
    if out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    // ERROR_NO_TOKEN = 1008 when thread is not impersonating.
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    err(1008, FALSE)
}

pub unsafe extern "win64" fn GetTokenInformation(
    token: HANDLE,
    info_class: u32,
    buf: *mut u8,
    buf_len: u32,
    ret_len: *mut u32,
) -> BOOL {
    if token.is_null() || ret_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    match info_class {
        // TokenUser (1): TOKEN_USER { SID_AND_ATTRIBUTES { PSID @0, DWORD Attrs @8 } } + 12-byte SID
        1 => {
            let need = 16 + 12;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            let sid = buf.add(16);
            *sid = 1; // revision
            *sid.add(1) = 1; // subauthority count
            *sid.add(7) = 5; // SECURITY_NT_AUTHORITY
            std::ptr::write_unaligned(sid.add(8) as *mut u32, 1000);
            std::ptr::write_unaligned(buf as *mut usize, sid as usize);
            TRUE
        }
        // TokenSessionId (12) -> 1
        12 => {
            std::ptr::write_unaligned(ret_len, 4);
            if buf.is_null() || buf_len < 4 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_unaligned(buf as *mut u32, 1);
            TRUE
        }
        // TokenElevationType (18) -> TokenElevationTypeDefault (1)
        18 => {
            std::ptr::write_unaligned(ret_len, 4);
            if buf.is_null() || buf_len < 4 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_unaligned(buf as *mut u32, 1);
            TRUE
        }
        // TokenElevation (20) -> 0 (not elevated)
        20 => {
            std::ptr::write_unaligned(ret_len, 4);
            if buf.is_null() || buf_len < 4 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_unaligned(buf as *mut u32, 0);
            TRUE
        }
        // TokenIntegrityLevel (25) -> Medium (0x2000)
        25 => {
            let need = 16 + 12;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            let sid = buf.add(16);
            *sid = 1;
            *sid.add(1) = 1;
            *sid.add(7) = 16; // SECURITY_MANDATORY_LABEL_AUTHORITY
            std::ptr::write_unaligned(sid.add(8) as *mut u32, 0x2000); // MEDIUM_RID
            std::ptr::write_unaligned(buf as *mut usize, sid as usize);
            std::ptr::write_unaligned(buf.add(8) as *mut u32, 0x20); // SE_GROUP_INTEGRITY
            TRUE
        }
        // TokenIsAppContainer (29) -> 0
        29 => {
            std::ptr::write_unaligned(ret_len, 4);
            if buf.is_null() || buf_len < 4 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_unaligned(buf as *mut u32, 0);
            TRUE
        }
        _ => {
            stub!("GetTokenInformation(class={info_class}) -> ERROR_CALL_NOT_IMPLEMENTED");
            std::ptr::write_unaligned(ret_len, 0);
            err(crate::types::err::ERROR_CALL_NOT_IMPLEMENTED, FALSE)
        }
    }
}

pub unsafe extern "win64" fn DuplicateToken(
    _existing: HANDLE,
    _imp_level: u32,
    out: *mut HANDLE,
) -> BOOL {
    if out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(out, handles::insert(Arc::new(handles::Opaque("Token"))));
    TRUE
}

pub unsafe extern "win64" fn DuplicateTokenEx(
    _existing: HANDLE,
    _access: u32,
    _attr: *mut c_void,
    _imp_level: u32,
    _token_type: u32,
    out: *mut HANDLE,
) -> BOOL {
    if out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(out, handles::insert(Arc::new(handles::Opaque("Token"))));
    TRUE
}

pub unsafe extern "win64" fn ImpersonateLoggedOnUser(_token: HANDLE) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn RevertToSelf() -> BOOL {
    TRUE
}

/// Well-known privilege LUIDs (winnt.h SE_*_PRIVILEGE values).
const PRIVILEGES: &[(&str, u32)] = &[
    ("SeCreateTokenPrivilege", 2),
    ("SeAssignPrimaryTokenPrivilege", 3),
    ("SeLockMemoryPrivilege", 4),
    ("SeIncreaseQuotaPrivilege", 5),
    ("SeMachineAccountPrivilege", 6),
    ("SeTcbPrivilege", 7),
    ("SeSecurityPrivilege", 8),
    ("SeTakeOwnershipPrivilege", 9),
    ("SeLoadDriverPrivilege", 10),
    ("SeSystemProfilePrivilege", 11),
    ("SeSystemtimePrivilege", 12),
    ("SeProfileSingleProcessPrivilege", 13),
    ("SeIncreaseBasePriorityPrivilege", 14),
    ("SeCreatePagefilePrivilege", 15),
    ("SeCreatePermanentPrivilege", 16),
    ("SeBackupPrivilege", 17),
    ("SeRestorePrivilege", 18),
    ("SeShutdownPrivilege", 19),
    ("SeDebugPrivilege", 20),
    ("SeAuditPrivilege", 21),
    ("SeSystemEnvironmentPrivilege", 22),
    ("SeChangeNotifyPrivilege", 23),
    ("SeRemoteShutdownPrivilege", 24),
    ("SeUndockPrivilege", 25),
    ("SeManageVolumePrivilege", 28),
    ("SeImpersonatePrivilege", 29),
    ("SeCreateGlobalPrivilege", 30),
    ("SeIncreaseWorkingSetPrivilege", 33),
    ("SeTimeZonePrivilege", 34),
    ("SeCreateSymbolicLinkPrivilege", 35),
];

pub unsafe extern "win64" fn LookupPrivilegeValueA(_system: LPCSTR, name: LPCSTR, luid: *mut u64) -> BOOL {
    let n = crate::strings::from_ansi_opt(name).unwrap_or_default();
    match PRIVILEGES.iter().find(|(p, _)| p.eq_ignore_ascii_case(&n)) {
        Some((_, v)) if !luid.is_null() => {
            *luid = *v as u64;
            TRUE
        }
        Some(_) => err(ERROR_INVALID_PARAMETER, FALSE),
        None => err(ERROR_NO_SUCH_PRIVILEGE, FALSE),
    }
}

/// The token holds no adjustable privileges (standard user): Windows returns
/// TRUE with last error ERROR_NOT_ALL_ASSIGNED in that case.
pub unsafe extern "win64" fn AdjustTokenPrivileges(token: HANDLE, disable_all: BOOL, new: *const u32, _len: u32, prev: *mut u32, ret_len: *mut u32) -> BOOL {
    if handles::get(token).is_none() {
        return err(ERROR_INVALID_HANDLE, FALSE);
    }
    if !prev.is_null() {
        *prev = 0; // PrivilegeCount = 0
    }
    if !ret_len.is_null() {
        *ret_len = 4;
    }
    if disable_all != 0 || new.is_null() || *new == 0 {
        return err(ERROR_SUCCESS, TRUE);
    }
    log::debug!("AdjustTokenPrivileges: {} privilege(s) requested -> ERROR_NOT_ALL_ASSIGNED", *new);
    err(ERROR_NOT_ALL_ASSIGNED, TRUE)
}

// ------------------------------------------------------------------ user

fn host_user() -> String {
    unsafe {
        let pw = libc::getpwuid(libc::getuid());
        if !pw.is_null() && !(*pw).pw_name.is_null() {
            return std::ffi::CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned();
        }
    }
    std::env::var("USER").unwrap_or_else(|_| "player".into())
}

/// `*size` is in characters and includes the terminator on success.
pub unsafe extern "win64" fn GetUserNameW(buf: LPWSTR, size: *mut u32) -> BOOL {
    if size.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let w = crate::strings::to_wide_z(&host_user());
    if buf.is_null() || (*size as usize) < w.len() {
        *size = w.len() as u32;
        return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    *size = w.len() as u32;
    TRUE
}

pub unsafe extern "win64" fn GetUserNameA(buf: LPSTR, size: *mut u32) -> BOOL {
    if size.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let u = host_user();
    let needed = (u.len() + 1) as u32;
    if buf.is_null() || *size < needed {
        *size = needed;
        return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    crate::strings::write_ansi_trunc(&u, buf, *size as usize);
    *size = needed;
    TRUE
}

pub unsafe extern "win64" fn InitializeSecurityDescriptor(
    p_sec_desc: *mut u8,
    revision: u32,
) -> BOOL {
    if p_sec_desc.is_null() || revision != 1 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_bytes(p_sec_desc, 0, 40);
    *p_sec_desc = 1;
    TRUE
}

pub unsafe extern "win64" fn SetSecurityDescriptorDacl(
    p_sec_desc: *mut u8,
    _dacl_present: BOOL,
    _dacl: *const u8,
    _dacl_defaulted: BOOL,
) -> BOOL {
    if p_sec_desc.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    TRUE
}

pub unsafe extern "win64" fn SetSecurityDescriptorSacl(
    p_sec_desc: *mut u8,
    _sacl_present: BOOL,
    _sacl: *const u8,
    _sacl_defaulted: BOOL,
) -> BOOL {
    if p_sec_desc.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    TRUE
}

pub unsafe extern "win64" fn ConvertStringSecurityDescriptorToSecurityDescriptorW(
    sddl: LPCWSTR,
    revision: u32,
    out_sd: *mut *mut c_void,
    out_size: *mut u32,
) -> BOOL {
    if sddl.is_null() || out_sd.is_null() || revision != 1 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let sd = crate::heap::alloc(40, true) as *mut u8;
    if sd.is_null() {
        return err(crate::types::err::ERROR_NOT_ENOUGH_MEMORY, FALSE);
    }
    *sd = 1; // SECURITY_DESCRIPTOR_REVISION
    std::ptr::write_unaligned(out_sd, sd as *mut c_void);
    if !out_size.is_null() {
        std::ptr::write_unaligned(out_size, 40);
    }
    TRUE
}

pub unsafe extern "win64" fn ConvertStringSecurityDescriptorToSecurityDescriptorA(
    sddl: LPCSTR,
    revision: u32,
    out_sd: *mut *mut c_void,
    out_size: *mut u32,
) -> BOOL {
    if sddl.is_null() || out_sd.is_null() || revision != 1 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let sd = crate::heap::alloc(40, true) as *mut u8;
    if sd.is_null() {
        return err(crate::types::err::ERROR_NOT_ENOUGH_MEMORY, FALSE);
    }
    *sd = 1;
    std::ptr::write_unaligned(out_sd, sd as *mut c_void);
    if !out_size.is_null() {
        std::ptr::write_unaligned(out_size, 40);
    }
    TRUE
}

#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn AllocateAndInitializeSid(
    auth: *const u8,
    sub_count: u8,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    out_sid: *mut *mut c_void,
) -> BOOL {
    if auth.is_null() || out_sid.is_null() || sub_count > 8 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let sz = 8 + 4 * (sub_count as usize);
    let sid = crate::heap::alloc(sz, true) as *mut u8;
    if sid.is_null() {
        return err(crate::types::err::ERROR_NOT_ENOUGH_MEMORY, FALSE);
    }
    *sid = 1; // SID_REVISION
    *sid.add(1) = sub_count;
    std::ptr::copy_nonoverlapping(auth, sid.add(2), 6);
    let subs = [s0, s1, s2, s3, s4, s5, s6, s7];
    for i in 0..sub_count as usize {
        std::ptr::write_unaligned(sid.add(8 + 4 * i) as *mut u32, subs[i]);
    }
    std::ptr::write_unaligned(out_sid, sid as *mut c_void);
    TRUE
}

pub unsafe extern "win64" fn FreeSid(sid: *mut c_void) -> *mut c_void {
    if !sid.is_null() {
        crate::heap::free(sid);
    }
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn GetLengthSid(sid: *const u8) -> u32 {
    if sid.is_null() {
        return 0;
    }
    8 + 4 * (*sid.add(1) as u32)
}

pub unsafe extern "win64" fn EqualSid(s1: *const u8, s2: *const u8) -> BOOL {
    if s1.is_null() || s2.is_null() {
        return FALSE;
    }
    let l1 = GetLengthSid(s1) as usize;
    let l2 = GetLengthSid(s2) as usize;
    if l1 != l2 {
        return FALSE;
    }
    (std::slice::from_raw_parts(s1, l1) == std::slice::from_raw_parts(s2, l2)) as BOOL
}

pub unsafe extern "win64" fn ConvertSidToStringSidW(sid: *const u8, out: *mut LPWSTR) -> BOOL {
    if sid.is_null() || out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let s = "S-1-5-21-0-0-0-1000";
    let w: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let p = crate::heap::alloc(w.len() * 2, false) as *mut u16;
    if p.is_null() {
        return err(crate::types::err::ERROR_NOT_ENOUGH_MEMORY, FALSE);
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), p, w.len());
    std::ptr::write_unaligned(out, p);
    TRUE
}

pub unsafe extern "win64" fn ConvertSidToStringSidA(sid: *const u8, out: *mut LPSTR) -> BOOL {
    if sid.is_null() || out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let s = b"S-1-5-21-0-0-0-1000\0";
    let p = crate::heap::alloc(s.len(), false) as *mut u8;
    if p.is_null() {
        return err(crate::types::err::ERROR_NOT_ENOUGH_MEMORY, FALSE);
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), p, s.len());
    std::ptr::write_unaligned(out, p);
    TRUE
}

pub unsafe extern "win64" fn CheckTokenMembership(
    _token: HANDLE,
    _sid: *const c_void,
    is_member: *mut BOOL,
) -> BOOL {
    if is_member.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(is_member, FALSE);
    TRUE
}

pub unsafe extern "win64" fn LookupPrivilegeValueW(_system: LPCWSTR, name: LPCWSTR, luid: *mut u64) -> BOOL {
    let a = crate::strings::from_wide_opt(name).map(|s| {
        let mut b = s.into_bytes();
        b.push(0);
        b
    });
    LookupPrivilegeValueA(
        std::ptr::null(),
        a.as_ref().map_or(std::ptr::null(), |b| b.as_ptr()),
        luid,
    )
}

pub unsafe extern "win64" fn GetSecurityInfo(
    _h: HANDLE,
    _ty: u32,
    _si: u32,
    _owner: *mut *mut c_void,
    _group: *mut *mut c_void,
    _dacl: *mut *mut c_void,
    _sacl: *mut *mut c_void,
    sd: *mut *mut c_void,
) -> u32 {
    if !sd.is_null() {
        let p = crate::heap::alloc(40, true) as *mut u8;
        if !p.is_null() {
            *p = 1;
        }
        std::ptr::write_unaligned(sd, p as *mut c_void);
    }
    0
}

pub unsafe extern "win64" fn SetSecurityInfo(
    _h: HANDLE,
    _ty: u32,
    _si: u32,
    _owner: *mut c_void,
    _group: *mut c_void,
    _dacl: *mut c_void,
    _sacl: *mut c_void,
) -> u32 {
    0
}

pub unsafe extern "win64" fn GetNamedSecurityInfoW(
    _name: LPCWSTR,
    _ty: u32,
    _si: u32,
    _owner: *mut *mut c_void,
    _group: *mut *mut c_void,
    _dacl: *mut *mut c_void,
    _sacl: *mut *mut c_void,
    sd: *mut *mut c_void,
) -> u32 {
    GetSecurityInfo(std::ptr::null_mut(), _ty, _si, _owner, _group, _dacl, _sacl, sd)
}

pub unsafe extern "win64" fn SetNamedSecurityInfoW(
    _name: LPCWSTR,
    _ty: u32,
    _si: u32,
    _owner: *mut c_void,
    _group: *mut c_void,
    _dacl: *mut c_void,
    _sacl: *mut c_void,
) -> u32 {
    0
}

pub unsafe extern "win64" fn SetEntriesInAclW(
    _count: u32,
    _entries: *mut c_void,
    _old_acl: *mut c_void,
    new_acl: *mut *mut c_void,
) -> u32 {
    if !new_acl.is_null() {
        let p = crate::heap::alloc(16, true);
        std::ptr::write_unaligned(new_acl, p);
    }
    0
}

pub unsafe extern "win64" fn RegOpenKeyW(key: HANDLE, sub: LPCWSTR, out: *mut HANDLE) -> u32 {
    RegOpenKeyExW(key, sub, 0, 0x20019, out)
}

pub unsafe extern "win64" fn RegOpenKeyA(key: HANDLE, sub: LPCSTR, out: *mut HANDLE) -> u32 {
    RegOpenKeyExA(key, sub, 0, 0x20019, out)
}

pub unsafe extern "win64" fn RegCreateKeyExA(
    key: HANDLE,
    sub: LPCSTR,
    r: u32,
    class: LPSTR,
    opts: u32,
    sam: u32,
    sa: *mut c_void,
    out: *mut HANDLE,
    disp: *mut u32,
) -> u32 {
    let _ = class;
    let w = crate::strings::from_ansi_opt(sub).map(|s| crate::strings::to_wide_z(&s));
    RegCreateKeyExW(
        key,
        w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        r,
        std::ptr::null_mut(),
        opts,
        sam,
        sa,
        out,
        disp,
    )
}

pub unsafe extern "win64" fn RegSetValueExA(
    key: HANDLE,
    name: LPCSTR,
    r: u32,
    ty: u32,
    data: *const u8,
    len: u32,
) -> u32 {
    let w_name = crate::strings::from_ansi_opt(name).map(|s| crate::strings::to_wide_z(&s));
    if (ty == REG_SZ || ty == REG_EXPAND_SZ) && !data.is_null() {
        let s = crate::strings::from_ansi(data);
        let w_val = crate::strings::to_wide_z(&s);
        return RegSetValueExW(
            key,
            w_name.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
            r,
            ty,
            w_val.as_ptr() as *const u8,
            (w_val.len() * 2) as u32,
        );
    }
    RegSetValueExW(
        key,
        w_name.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        r,
        ty,
        data,
        len,
    )
}

pub unsafe extern "win64" fn RegDeleteValueW(key: HANDLE, name: LPCWSTR) -> u32 {
    let Some(path) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let name_str = crate::strings::from_wide_opt(name)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let mut reg = registry().lock().unwrap();
    if let Some(vals) = reg.keys.get_mut(&path) {
        if vals.remove(&name_str).is_some() {
            let fired = reg.take_triggered(&[&path], REG_NOTIFY_CHANGE_LAST_SET);
            drop(reg);
            signal_reg_events(fired);
            return ERROR_SUCCESS;
        }
    }
    ERROR_FILE_NOT_FOUND
}

pub unsafe extern "win64" fn RegDeleteValueA(key: HANDLE, name: LPCSTR) -> u32 {
    let w = crate::strings::from_ansi_opt(name).map(|s| crate::strings::to_wide_z(&s));
    RegDeleteValueW(key, w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()))
}

pub unsafe extern "win64" fn RegGetValueA(
    key: HANDLE,
    sub: LPCSTR,
    name: LPCSTR,
    _flags: u32,
    ty: *mut u32,
    data: *mut c_void,
    len: *mut u32,
) -> u32 {
    let Some(base) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let sub_str = crate::strings::from_ansi_opt(sub).unwrap_or_default();
    let full = join_reg_path(&base, &sub_str);
    let name_str = crate::strings::from_ansi_opt(name).unwrap_or_default();
    let rc = query_value_bytes(&full, &name_str, ty, data as *mut u8, len, true);
    if rc != ERROR_SUCCESS {
        if !ty.is_null() {
            *ty = 0;
        }
        if !len.is_null() && data.is_null() {
            *len = 0;
        }
    }
    rc
}

pub unsafe extern "win64" fn RegQueryInfoKeyA(
    key: HANDLE,
    class: LPSTR,
    ccls: *mut u32,
    r: *mut u32,
    subkeys: *mut u32,
    max_subkey: *mut u32,
    max_class: *mut u32,
    values: *mut u32,
    max_value_name: *mut u32,
    max_value_len: *mut u32,
    sec_desc: *mut u32,
    ft: *mut u64,
) -> u32 {
    if !class.is_null() && !ccls.is_null() && *ccls > 0 {
        *class = 0;
    }
    RegQueryInfoKeyW(
        key,
        std::ptr::null_mut(),
        ccls,
        r,
        subkeys,
        max_subkey,
        max_class,
        values,
        max_value_name,
        max_value_len,
        sec_desc,
        ft,
    )
}

// ------------------------------------------------------- userenv.dll
// (`userenv.dll` routes to `advapi32.dll` in `port-manifest::dll_map`).

fn user_profile_dir() -> String {
    if let Some(p) = crate::process::get_env("USERPROFILE").filter(|s| !s.is_empty()) {
        return p;
    }
    format!("C:\\users\\{}", host_user())
}

pub unsafe extern "win64" fn GetUserProfileDirectoryW(
    h_token: HANDLE,
    lp_profile_dir: LPWSTR,
    lpcch_size: *mut u32,
) -> BOOL {
    if lpcch_size.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if h_token.is_null() || handles::get(h_token).is_none() {
        return err(ERROR_INVALID_HANDLE, FALSE);
    }
    let dir = user_profile_dir();
    if let Some(host) = crate::path::to_host(&dir) {
        let _ = std::fs::create_dir_all(host);
    }
    let units = dir.encode_utf16().count() + 1;
    let needed = units as u32;
    let cap = std::ptr::read_unaligned(lpcch_size);
    if lp_profile_dir.is_null() || cap < needed {
        std::ptr::write_unaligned(lpcch_size, needed);
        return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    crate::strings::write_wide_z(lp_profile_dir, units, &dir);
    std::ptr::write_unaligned(lpcch_size, needed);
    TRUE
}

pub unsafe extern "win64" fn GetUserProfileDirectoryA(
    h_token: HANDLE,
    lp_profile_dir: LPSTR,
    lpcch_size: *mut u32,
) -> BOOL {
    if lpcch_size.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if h_token.is_null() || handles::get(h_token).is_none() {
        return err(ERROR_INVALID_HANDLE, FALSE);
    }
    let dir = user_profile_dir();
    if let Some(host) = crate::path::to_host(&dir) {
        let _ = std::fs::create_dir_all(host);
    }
    let needed = dir.len() as u32 + 1;
    let cap = std::ptr::read_unaligned(lpcch_size);
    if lp_profile_dir.is_null() || cap < needed {
        std::ptr::write_unaligned(lpcch_size, needed);
        return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    std::ptr::copy_nonoverlapping(dir.as_ptr(), lp_profile_dir, dir.len());
    *lp_profile_dir.add(dir.len()) = 0;
    std::ptr::write_unaligned(lpcch_size, needed);
    TRUE
}

// ------------------------------------------------------- User-mode Service Control Manager & Dispatcher

const ERROR_INVALID_SERVICE_CONTROL: u32 = 1052;
const ERROR_SERVICE_ALREADY_RUNNING: u32 = 1056;
const ERROR_SERVICE_DOES_NOT_EXIST: u32 = 1060;
const ERROR_SERVICE_NOT_ACTIVE: u32 = 1062;
const ERROR_SERVICE_EXISTS: u32 = 1073;
const ERROR_INVALID_DATA: u32 = 13;

const SERVICE_WIN32_OWN_PROCESS: u32 = 0x10;
const SERVICE_STOPPED: u32 = 1;
const SERVICE_CONTROL_STOP: u32 = 1;
const SERVICE_CONTROL_PAUSE: u32 = 2;
const SERVICE_CONTROL_CONTINUE: u32 = 3;
const SERVICE_CONTROL_INTERROGATE: u32 = 4;
const SERVICE_CONTROL_SHUTDOWN: u32 = 5;
const SERVICE_ACCEPT_STOP: u32 = 0x1;
const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 0x2;
const SERVICE_ACCEPT_SHUTDOWN: u32 = 0x4;
const SERVICE_CONFIG_DESCRIPTION: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct SERVICE_STATUS {
    pub dwServiceType: u32,
    pub dwCurrentState: u32,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct SERVICE_STATUS_PROCESS {
    pub dwServiceType: u32,
    pub dwCurrentState: u32,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
    pub dwProcessId: u32,
    pub dwServiceFlags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: LPWSTR,
    pub lpServiceProc: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: LPSTR,
    pub lpServiceProc: usize,
}

#[derive(Clone, Copy)]
enum ServiceHandler {
    Legacy(usize),
    Ex(usize, usize),
}

struct ServiceEntry {
    name: String,
    display_name: std::sync::Mutex<String>,
    binary_path: std::sync::Mutex<String>,
    description: std::sync::Mutex<String>,
    start_type: std::sync::atomic::AtomicU32,
    error_control: std::sync::atomic::AtomicU32,
    status: std::sync::Mutex<SERVICE_STATUS>,
    handler: std::sync::Mutex<Option<ServiceHandler>>,
}

impl ServiceEntry {
    fn new(name: String, display_name: String, binary_path: String, service_type: u32, start_type: u32, error_control: u32) -> Self {
        Self {
            name,
            display_name: std::sync::Mutex::new(display_name),
            binary_path: std::sync::Mutex::new(binary_path),
            description: std::sync::Mutex::new(String::new()),
            start_type: std::sync::atomic::AtomicU32::new(start_type),
            error_control: std::sync::atomic::AtomicU32::new(error_control),
            status: std::sync::Mutex::new(SERVICE_STATUS {
                dwServiceType: if service_type == 0 { SERVICE_WIN32_OWN_PROCESS } else { service_type },
                dwCurrentState: SERVICE_STOPPED,
                ..Default::default()
            }),
            handler: std::sync::Mutex::new(None),
        }
    }
}

struct ScmObj;
impl handles::KObject for ScmObj {
    fn type_name(&self) -> &'static str {
        "SCManager"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct ServiceHandleObj {
    entry: Arc<ServiceEntry>,
}
impl handles::KObject for ServiceHandleObj {
    fn type_name(&self) -> &'static str {
        "Service"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct ServiceStatusHandleObj {
    entry: Arc<ServiceEntry>,
}
impl handles::KObject for ServiceStatusHandleObj {
    fn type_name(&self) -> &'static str {
        "ServiceStatusHandle"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn services() -> &'static std::sync::Mutex<std::collections::HashMap<String, Arc<ServiceEntry>>> {
    static SVCS: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, Arc<ServiceEntry>>>> =
        std::sync::OnceLock::new();
    SVCS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

static DISPATCHER_RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn get_or_create_service(name: &str) -> Arc<ServiceEntry> {
    let key = name.to_ascii_lowercase();
    let mut map = services().lock().unwrap();
    map.entry(key)
        .or_insert_with(|| {
            Arc::new(ServiceEntry::new(
                name.to_string(),
                name.to_string(),
                String::new(),
                SERVICE_WIN32_OWN_PROCESS,
                3, // SERVICE_DEMAND_START
                1, // SERVICE_ERROR_NORMAL
            ))
        })
        .clone()
}

fn is_local_machine(machine: Option<&str>) -> bool {
    match machine {
        None => true,
        Some(m) => {
            let t = m.trim().trim_start_matches('\\');
            t.is_empty()
                || t == "."
                || t.eq_ignore_ascii_case("localhost")
                || t.eq_ignore_ascii_case(&crate::config::app_config().computer_name)
        }
    }
}

pub unsafe extern "win64" fn OpenSCManagerW(
    machine_name: LPCWSTR,
    _database_name: LPCWSTR,
    _desired_access: u32,
) -> HANDLE {
    let m = crate::strings::from_wide_opt(machine_name);
    if !is_local_machine(m.as_deref()) {
        return err(ERROR_ACCESS_DENIED, std::ptr::null_mut());
    }
    handles::insert(Arc::new(ScmObj))
}

pub unsafe extern "win64" fn OpenSCManagerA(
    machine_name: LPCSTR,
    _database_name: LPCSTR,
    desired_access: u32,
) -> HANDLE {
    let w = crate::strings::from_ansi_opt(machine_name).map(|s| crate::strings::to_wide_z(&s));
    OpenSCManagerW(w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), std::ptr::null(), desired_access)
}

pub unsafe extern "win64" fn CreateServiceW(
    h_sc_manager: HANDLE,
    service_name: LPCWSTR,
    display_name: LPCWSTR,
    _desired_access: u32,
    service_type: u32,
    start_type: u32,
    error_control: u32,
    binary_path: LPCWSTR,
    _load_order_group: LPCWSTR,
    _tag_id: *mut u32,
    _dependencies: LPCWSTR,
    _service_start_name: LPCWSTR,
    _password: LPCWSTR,
) -> HANDLE {
    if handles::get_as::<ScmObj>(h_sc_manager).is_none() {
        return err(ERROR_INVALID_HANDLE, std::ptr::null_mut());
    }
    let Some(name) = crate::strings::from_wide_opt(service_name).filter(|s| !s.is_empty()) else {
        return err(ERROR_INVALID_PARAMETER, std::ptr::null_mut());
    };
    let disp = crate::strings::from_wide_opt(display_name)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| name.clone());
    let bin = crate::strings::from_wide_opt(binary_path).unwrap_or_default();
    let key = name.to_ascii_lowercase();
    let mut map = services().lock().unwrap();
    if map.contains_key(&key) {
        return err(ERROR_SERVICE_EXISTS, std::ptr::null_mut());
    }
    let entry = Arc::new(ServiceEntry::new(name, disp, bin, service_type, start_type, error_control));
    map.insert(key, Arc::clone(&entry));
    drop(map);
    handles::insert(Arc::new(ServiceHandleObj { entry }))
}

pub unsafe extern "win64" fn CreateServiceA(
    h_sc_manager: HANDLE,
    service_name: LPCSTR,
    display_name: LPCSTR,
    desired_access: u32,
    service_type: u32,
    start_type: u32,
    error_control: u32,
    binary_path: LPCSTR,
    _load_order_group: LPCSTR,
    tag_id: *mut u32,
    _dependencies: LPCSTR,
    _service_start_name: LPCSTR,
    _password: LPCSTR,
) -> HANDLE {
    let wn = crate::strings::from_ansi_opt(service_name).map(|s| crate::strings::to_wide_z(&s));
    let wd = crate::strings::from_ansi_opt(display_name).map(|s| crate::strings::to_wide_z(&s));
    let wb = crate::strings::from_ansi_opt(binary_path).map(|s| crate::strings::to_wide_z(&s));
    CreateServiceW(
        h_sc_manager,
        wn.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        wd.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        desired_access,
        service_type,
        start_type,
        error_control,
        wb.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
        std::ptr::null(),
        tag_id,
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    )
}

pub unsafe extern "win64" fn OpenServiceW(
    h_sc_manager: HANDLE,
    service_name: LPCWSTR,
    _desired_access: u32,
) -> HANDLE {
    if handles::get_as::<ScmObj>(h_sc_manager).is_none() {
        return err(ERROR_INVALID_HANDLE, std::ptr::null_mut());
    }
    let Some(name) = crate::strings::from_wide_opt(service_name).filter(|s| !s.is_empty()) else {
        return err(ERROR_INVALID_PARAMETER, std::ptr::null_mut());
    };
    let key = name.to_ascii_lowercase();
    let entry = services().lock().unwrap().get(&key).cloned();
    match entry {
        Some(entry) => handles::insert(Arc::new(ServiceHandleObj { entry })),
        None => err(ERROR_SERVICE_DOES_NOT_EXIST, std::ptr::null_mut()),
    }
}

pub unsafe extern "win64" fn OpenServiceA(
    h_sc_manager: HANDLE,
    service_name: LPCSTR,
    desired_access: u32,
) -> HANDLE {
    let w = crate::strings::from_ansi_opt(service_name).map(|s| crate::strings::to_wide_z(&s));
    OpenServiceW(h_sc_manager, w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), desired_access)
}

pub unsafe extern "win64" fn CloseServiceHandle(h_sc_object: HANDLE) -> BOOL {
    if handles::get_as::<ScmObj>(h_sc_object).is_some()
        || handles::get_as::<ServiceHandleObj>(h_sc_object).is_some()
    {
        handles::close(h_sc_object);
        TRUE
    } else {
        err(ERROR_INVALID_HANDLE, FALSE)
    }
}

pub unsafe extern "win64" fn DeleteService(h_service: HANDLE) -> BOOL {
    let Some(svc) = handles::get_as::<ServiceHandleObj>(h_service) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    services().lock().unwrap().remove(&svc.entry.name.to_ascii_lowercase());
    TRUE
}

pub unsafe extern "win64" fn RegisterServiceCtrlHandlerW(
    service_name: LPCWSTR,
    handler_proc: *const c_void,
) -> HANDLE {
    let Some(name) = crate::strings::from_wide_opt(service_name).filter(|s| !s.is_empty()) else {
        return err(ERROR_INVALID_PARAMETER, std::ptr::null_mut());
    };
    if handler_proc.is_null() {
        return err(ERROR_INVALID_PARAMETER, std::ptr::null_mut());
    }
    let entry = get_or_create_service(&name);
    *entry.handler.lock().unwrap() = Some(ServiceHandler::Legacy(handler_proc as usize));
    handles::insert(Arc::new(ServiceStatusHandleObj { entry }))
}

pub unsafe extern "win64" fn RegisterServiceCtrlHandlerA(
    service_name: LPCSTR,
    handler_proc: *const c_void,
) -> HANDLE {
    let w = crate::strings::from_ansi_opt(service_name).map(|s| crate::strings::to_wide_z(&s));
    RegisterServiceCtrlHandlerW(w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), handler_proc)
}

pub unsafe extern "win64" fn RegisterServiceCtrlHandlerExW(
    service_name: LPCWSTR,
    handler_proc: *const c_void,
    context: *mut c_void,
) -> HANDLE {
    let Some(name) = crate::strings::from_wide_opt(service_name).filter(|s| !s.is_empty()) else {
        return err(ERROR_INVALID_PARAMETER, std::ptr::null_mut());
    };
    if handler_proc.is_null() {
        return err(ERROR_INVALID_PARAMETER, std::ptr::null_mut());
    }
    let entry = get_or_create_service(&name);
    *entry.handler.lock().unwrap() = Some(ServiceHandler::Ex(handler_proc as usize, context as usize));
    handles::insert(Arc::new(ServiceStatusHandleObj { entry }))
}

pub unsafe extern "win64" fn RegisterServiceCtrlHandlerExA(
    service_name: LPCSTR,
    handler_proc: *const c_void,
    context: *mut c_void,
) -> HANDLE {
    let w = crate::strings::from_ansi_opt(service_name).map(|s| crate::strings::to_wide_z(&s));
    RegisterServiceCtrlHandlerExW(w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), handler_proc, context)
}

pub unsafe extern "win64" fn SetServiceStatus(
    h_service_status: HANDLE,
    lp_service_status: *const SERVICE_STATUS,
) -> BOOL {
    if lp_service_status.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let Some(sh) = handles::get_as::<ServiceStatusHandleObj>(h_service_status) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    let st = std::ptr::read_unaligned(lp_service_status);
    *sh.entry.status.lock().unwrap() = st;
    TRUE
}

pub unsafe extern "win64" fn QueryServiceStatus(
    h_service: HANDLE,
    lp_service_status: *mut SERVICE_STATUS,
) -> BOOL {
    if lp_service_status.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let Some(svc) = handles::get_as::<ServiceHandleObj>(h_service) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    let st = *svc.entry.status.lock().unwrap();
    std::ptr::write_unaligned(lp_service_status, st);
    TRUE
}

pub unsafe extern "win64" fn QueryServiceStatusEx(
    h_service: HANDLE,
    info_level: u32,
    lp_buffer: *mut u8,
    cb_buf_size: u32,
    pcb_bytes_needed: *mut u32,
) -> BOOL {
    if pcb_bytes_needed.is_null() || info_level != 0 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let Some(svc) = handles::get_as::<ServiceHandleObj>(h_service) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    let need = std::mem::size_of::<SERVICE_STATUS_PROCESS>() as u32;
    std::ptr::write_unaligned(pcb_bytes_needed, need);
    if lp_buffer.is_null() || cb_buf_size < need {
        return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    let st = *svc.entry.status.lock().unwrap();
    let ssp = SERVICE_STATUS_PROCESS {
        dwServiceType: st.dwServiceType,
        dwCurrentState: st.dwCurrentState,
        dwControlsAccepted: st.dwControlsAccepted,
        dwWin32ExitCode: st.dwWin32ExitCode,
        dwServiceSpecificExitCode: st.dwServiceSpecificExitCode,
        dwCheckPoint: st.dwCheckPoint,
        dwWaitHint: st.dwWaitHint,
        dwProcessId: if st.dwCurrentState == SERVICE_STOPPED { 0 } else { std::process::id() },
        dwServiceFlags: 0,
    };
    std::ptr::write_unaligned(lp_buffer as *mut SERVICE_STATUS_PROCESS, ssp);
    TRUE
}

pub unsafe extern "win64" fn ControlService(
    h_service: HANDLE,
    dw_control: u32,
    lp_service_status: *mut SERVICE_STATUS,
) -> BOOL {
    if lp_service_status.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let Some(svc) = handles::get_as::<ServiceHandleObj>(h_service) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    let handler = *svc.entry.handler.lock().unwrap();
    let Some(handler) = handler else {
        return err(ERROR_SERVICE_NOT_ACTIVE, FALSE);
    };
    let cur = *svc.entry.status.lock().unwrap();
    let accepted = match dw_control {
        SERVICE_CONTROL_INTERROGATE | 128..=255 => true,
        SERVICE_CONTROL_STOP => (cur.dwControlsAccepted & SERVICE_ACCEPT_STOP) != 0,
        SERVICE_CONTROL_PAUSE | SERVICE_CONTROL_CONTINUE => (cur.dwControlsAccepted & SERVICE_ACCEPT_PAUSE_CONTINUE) != 0,
        SERVICE_CONTROL_SHUTDOWN => (cur.dwControlsAccepted & SERVICE_ACCEPT_SHUTDOWN) != 0,
        _ => true,
    };
    if !accepted {
        return err(ERROR_INVALID_SERVICE_CONTROL, FALSE);
    }
    // Invoke handler with NO locks held so handler can call SetServiceStatus re-entrantly.
    match handler {
        ServiceHandler::Legacy(proc_addr) => {
            crate::call_guest8(proc_addr, dw_control as u64, 0, 0, 0, 0, 0, 0, 0);
        }
        ServiceHandler::Ex(proc_addr, ctx) => {
            let rc = crate::call_guest8(proc_addr, dw_control as u64, 0, 0, ctx as u64, 0, 0, 0, 0) as u32;
            if rc != ERROR_SUCCESS {
                let updated = *svc.entry.status.lock().unwrap();
                std::ptr::write_unaligned(lp_service_status, updated);
                return err(rc, FALSE);
            }
        }
    }
    let updated = *svc.entry.status.lock().unwrap();
    std::ptr::write_unaligned(lp_service_status, updated);
    TRUE
}

pub unsafe extern "win64" fn ChangeServiceConfig2W(
    h_service: HANDLE,
    info_level: u32,
    lp_info: *const c_void,
) -> BOOL {
    let Some(svc) = handles::get_as::<ServiceHandleObj>(h_service) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    if lp_info.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if info_level == SERVICE_CONFIG_DESCRIPTION {
        let ptr = std::ptr::read_unaligned(lp_info as *const LPCWSTR);
        let desc = crate::strings::from_wide_opt(ptr).unwrap_or_default();
        *svc.entry.description.lock().unwrap() = desc;
        return TRUE;
    }
    err(ERROR_CALL_NOT_IMPLEMENTED, FALSE)
}

pub unsafe extern "win64" fn QueryServiceConfig2W(
    h_service: HANDLE,
    info_level: u32,
    lp_buffer: *mut u8,
    cb_buf_size: u32,
    pcb_bytes_needed: *mut u32,
) -> BOOL {
    if pcb_bytes_needed.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let Some(svc) = handles::get_as::<ServiceHandleObj>(h_service) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    if info_level == SERVICE_CONFIG_DESCRIPTION {
        let desc = svc.entry.description.lock().unwrap().clone();
        let desc_units = desc.encode_utf16().count() + 1;
        let need = (8 + desc_units * 2) as u32;
        std::ptr::write_unaligned(pcb_bytes_needed, need);
        if lp_buffer.is_null() || cb_buf_size < need {
            return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
        }
        let str_dst = lp_buffer.add(8) as *mut u16;
        crate::strings::write_wide_z(str_dst, desc_units, &desc);
        std::ptr::write_unaligned(lp_buffer as *mut LPWSTR, str_dst);
        return TRUE;
    }
    err(ERROR_CALL_NOT_IMPLEMENTED, FALSE)
}

pub unsafe extern "win64" fn QueryServiceConfigW(
    h_service: HANDLE,
    lp_config: *mut u8,
    cb_buf_size: u32,
    pcb_bytes_needed: *mut u32,
) -> BOOL {
    if pcb_bytes_needed.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let Some(svc) = handles::get_as::<ServiceHandleObj>(h_service) else {
        return err(ERROR_INVALID_HANDLE, FALSE);
    };
    let bin = svc.entry.binary_path.lock().unwrap().clone();
    let disp = svc.entry.display_name.lock().unwrap().clone();
    let bin_units = bin.encode_utf16().count() + 1;
    let disp_units = disp.encode_utf16().count() + 1;
    // QUERY_SERVICE_CONFIGW on x64 is 64 bytes:
    // 0: dwServiceType, 4: dwStartType, 8: dwErrorControl, 12: pad,
    // 16: lpBinaryPathName, 24: lpLoadOrderGroup, 32: dwTagId, 36: pad,
    // 40: lpDependencies, 48: lpServiceStartName, 56: lpDisplayName
    let header_sz = 64usize;
    let need = (header_sz + (bin_units + disp_units) * 2) as u32;
    std::ptr::write_unaligned(pcb_bytes_needed, need);
    if lp_config.is_null() || cb_buf_size < need {
        return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    std::ptr::write_bytes(lp_config, 0, header_sz);
    let st = *svc.entry.status.lock().unwrap();
    (lp_config as *mut u32).write_unaligned(st.dwServiceType);
    (lp_config.add(4) as *mut u32).write_unaligned(svc.entry.start_type.load(Ordering::Relaxed));
    (lp_config.add(8) as *mut u32).write_unaligned(svc.entry.error_control.load(Ordering::Relaxed));
    let bin_dst = lp_config.add(header_sz) as *mut u16;
    crate::strings::write_wide_z(bin_dst, bin_units, &bin);
    let disp_dst = bin_dst.add(bin_units);
    crate::strings::write_wide_z(disp_dst, disp_units, &disp);
    (lp_config.add(16) as *mut LPWSTR).write_unaligned(bin_dst);
    (lp_config.add(56) as *mut LPWSTR).write_unaligned(disp_dst);
    TRUE
}

fn dispatch_service_entries(entries: Vec<(String, usize, bool)>) -> BOOL {
    if entries.is_empty() {
        return err(ERROR_INVALID_DATA, FALSE);
    }
    if DISPATCHER_RUNNING
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return err(ERROR_SERVICE_ALREADY_RUNNING, FALSE);
    }
    struct DispatchGuard;
    impl Drop for DispatchGuard {
        fn drop(&mut self) {
            DISPATCHER_RUNNING.store(false, Ordering::Release);
        }
    }
    let _guard = DispatchGuard;

    let mut threads: Vec<Arc<dyn handles::KObject>> = Vec::with_capacity(entries.len());
    for (name, proc_addr, wide) in entries {
        let _ = get_or_create_service(&name);
        let spawned = crate::thread::spawn(
            Box::new(move || unsafe {
                if wide {
                    let mut wname = crate::strings::to_wide_z(&name);
                    let mut argv: [*mut u16; 2] = [wname.as_mut_ptr(), std::ptr::null_mut()];
                    crate::call_guest8(proc_addr, 1, argv.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0);
                } else {
                    let mut aname = name.into_bytes();
                    aname.push(0);
                    let mut argv: [*mut u8; 2] = [aname.as_mut_ptr(), std::ptr::null_mut()];
                    crate::call_guest8(proc_addr, 1, argv.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0);
                }
                0
            }),
            0,
            false,
            false,
        );
        match spawned {
            Ok(t) => threads.push(t),
            Err(e) => {
                let code = e.raw_os_error().map(errno_to_win32).unwrap_or(ERROR_ACCESS_DENIED);
                if !threads.is_empty() {
                    let _ = handles::wait_for(&threads, true, INFINITE, false);
                }
                return err(code, FALSE);
            }
        }
    }
    let _ = handles::wait_for(&threads, true, INFINITE, false);
    TRUE
}

pub unsafe extern "win64" fn StartServiceCtrlDispatcherW(
    service_start_table: *const SERVICE_TABLE_ENTRYW,
) -> BOOL {
    if service_start_table.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let mut entries = Vec::new();
    let mut i = 0usize;
    loop {
        let ent = std::ptr::read_unaligned(service_start_table.add(i));
        if ent.lpServiceName.is_null() && ent.lpServiceProc == 0 {
            break;
        }
        if ent.lpServiceProc == 0 {
            return err(ERROR_INVALID_DATA, FALSE);
        }
        let name = crate::strings::from_wide_opt(ent.lpServiceName).unwrap_or_default();
        entries.push((name, ent.lpServiceProc, true));
        i += 1;
        if i >= 256 {
            break;
        }
    }
    dispatch_service_entries(entries)
}

pub unsafe extern "win64" fn StartServiceCtrlDispatcherA(
    service_start_table: *const SERVICE_TABLE_ENTRYA,
) -> BOOL {
    if service_start_table.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let mut entries = Vec::new();
    let mut i = 0usize;
    loop {
        let ent = std::ptr::read_unaligned(service_start_table.add(i));
        if ent.lpServiceName.is_null() && ent.lpServiceProc == 0 {
            break;
        }
        if ent.lpServiceProc == 0 {
            return err(ERROR_INVALID_DATA, FALSE);
        }
        let name = crate::strings::from_ansi_opt(ent.lpServiceName).unwrap_or_default();
        entries.push((name, ent.lpServiceProc, false));
        i += 1;
        if i >= 256 {
            break;
        }
    }
    dispatch_service_entries(entries)
}

// ------------------------------------------------------- CAPI / Cred*

const CALG_MD5: u32 = 0x0000_8003;
const CALG_SHA1: u32 = 0x0000_8004;
const CALG_SHA_256: u32 = 0x0000_800C;
const CALG_SHA_384: u32 = 0x0000_800D;
const CALG_SHA_512: u32 = 0x0000_800E;

const HP_ALGID: u32 = 0x0001;
const HP_HASHVAL: u32 = 0x0002;
const HP_HASHSIZE: u32 = 0x0004;

const PP_IMPTYPE: u32 = 3;
const PP_NAME: u32 = 4;
const PP_VERSION: u32 = 5;
const PP_CONTAINER: u32 = 6;
const PP_PROVTYPE: u32 = 16;

const NTE_BAD_ALGID: u32 = 0x8009_0008;
const NTE_BAD_TYPE: u32 = 0x8009_000A;
const NTE_BAD_HASH: u32 = 0x8009_0002;
const NTE_BAD_UID: u32 = 0x8009_0001;

struct CapiProvider {
    prov_type: u32,
    prov_name: String,
    container: String,
}

/// Incremental CommonCrypto digest context (512-byte 8-byte-aligned storage holds
/// any `CC_MD5_CTX` / `CC_SHA1_CTX` / `CC_SHA256_CTX` / `CC_SHA512_CTX` without
/// buffering the full message or truncating lengths above 4 GiB).
struct CapiHash {
    alg_id: u32,
    ctx: [u64; 64],
    override_val: Option<Vec<u8>>,
}

static NEXT_CAPI_HANDLE: AtomicU64 = AtomicU64::new(0xC000_0001);

fn capi_providers() -> &'static std::sync::Mutex<std::collections::HashMap<usize, CapiProvider>> {
    static M: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, CapiProvider>>> =
        std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn capi_hashes() -> &'static std::sync::Mutex<std::collections::HashMap<usize, CapiHash>> {
    static M: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, CapiHash>>> =
        std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

extern "C" {
    fn CC_MD5_Init(c: *mut c_void) -> i32;
    fn CC_MD5_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_MD5_Final(md: *mut u8, c: *mut c_void) -> i32;

    fn CC_SHA1_Init(c: *mut c_void) -> i32;
    fn CC_SHA1_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA1_Final(md: *mut u8, c: *mut c_void) -> i32;

    fn CC_SHA256_Init(c: *mut c_void) -> i32;
    fn CC_SHA256_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA256_Final(md: *mut u8, c: *mut c_void) -> i32;

    fn CC_SHA384_Init(c: *mut c_void) -> i32;
    fn CC_SHA384_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA384_Final(md: *mut u8, c: *mut c_void) -> i32;

    fn CC_SHA512_Init(c: *mut c_void) -> i32;
    fn CC_SHA512_Update(c: *mut c_void, data: *const c_void, len: u32) -> i32;
    fn CC_SHA512_Final(md: *mut u8, c: *mut c_void) -> i32;
}

fn capi_hash_len(alg_id: u32) -> Option<usize> {
    match alg_id {
        CALG_MD5 => Some(16),
        CALG_SHA1 => Some(20),
        CALG_SHA_256 => Some(32),
        CALG_SHA_384 => Some(48),
        CALG_SHA_512 => Some(64),
        _ => None,
    }
}

unsafe fn capi_hash_init(alg_id: u32) -> Option<CapiHash> {
    let mut ctx = [0u64; 64];
    let p = ctx.as_mut_ptr() as *mut c_void;
    match alg_id {
        CALG_MD5 => { CC_MD5_Init(p); }
        CALG_SHA1 => { CC_SHA1_Init(p); }
        CALG_SHA_256 => { CC_SHA256_Init(p); }
        CALG_SHA_384 => { CC_SHA384_Init(p); }
        CALG_SHA_512 => { CC_SHA512_Init(p); }
        _ => return None,
    }
    Some(CapiHash {
        alg_id,
        ctx,
        override_val: None,
    })
}

unsafe fn capi_hash_update(h: &mut CapiHash, data: *const u8, len: u32) {
    let p = h.ctx.as_mut_ptr() as *mut c_void;
    let d = data as *const c_void;
    match h.alg_id {
        CALG_MD5 => { CC_MD5_Update(p, d, len); }
        CALG_SHA1 => { CC_SHA1_Update(p, d, len); }
        CALG_SHA_256 => { CC_SHA256_Update(p, d, len); }
        CALG_SHA_384 => { CC_SHA384_Update(p, d, len); }
        CALG_SHA_512 => { CC_SHA512_Update(p, d, len); }
        _ => {}
    }
}

unsafe fn capi_hash_finalize(h: &CapiHash) -> Option<Vec<u8>> {
    let len = capi_hash_len(h.alg_id)?;
    let mut out = vec![0u8; len];
    // Finalize a copy of the context so size queries / repeated reads stay valid.
    let mut snapshot = h.ctx;
    let p = snapshot.as_mut_ptr() as *mut c_void;
    match h.alg_id {
        CALG_MD5 => { CC_MD5_Final(out.as_mut_ptr(), p); }
        CALG_SHA1 => { CC_SHA1_Final(out.as_mut_ptr(), p); }
        CALG_SHA_256 => { CC_SHA256_Final(out.as_mut_ptr(), p); }
        CALG_SHA_384 => { CC_SHA384_Final(out.as_mut_ptr(), p); }
        CALG_SHA_512 => { CC_SHA512_Final(out.as_mut_ptr(), p); }
        _ => return None,
    }
    Some(out)
}

unsafe fn crypt_acquire_context_impl(
    ph_prov: *mut usize,
    container: Option<String>,
    provider: Option<String>,
    dw_prov_type: u32,
    _dw_flags: u32,
) -> BOOL {
    if ph_prov.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    let prov = CapiProvider {
        prov_type: dw_prov_type,
        prov_name: provider.unwrap_or_else(|| "Microsoft Enhanced RSA and AES Cryptographic Provider".into()),
        container: container.unwrap_or_default(),
    };
    capi_providers().lock().unwrap().insert(h, prov);
    std::ptr::write_unaligned(ph_prov, h);
    TRUE
}

pub unsafe extern "win64" fn CryptAcquireContextW(
    ph_prov: *mut usize,
    sz_container: LPCWSTR,
    sz_provider: LPCWSTR,
    dw_prov_type: u32,
    dw_flags: u32,
) -> BOOL {
    crypt_acquire_context_impl(
        ph_prov,
        crate::strings::from_wide_opt(sz_container),
        crate::strings::from_wide_opt(sz_provider),
        dw_prov_type,
        dw_flags,
    )
}

pub unsafe extern "win64" fn CryptAcquireContextA(
    ph_prov: *mut usize,
    sz_container: LPCSTR,
    sz_provider: LPCSTR,
    dw_prov_type: u32,
    dw_flags: u32,
) -> BOOL {
    crypt_acquire_context_impl(
        ph_prov,
        crate::strings::from_ansi_opt(sz_container),
        crate::strings::from_ansi_opt(sz_provider),
        dw_prov_type,
        dw_flags,
    )
}

pub unsafe extern "win64" fn CryptReleaseContext(h_prov: usize, _dw_flags: u32) -> BOOL {
    if capi_providers().lock().unwrap().remove(&h_prov).is_some() {
        TRUE
    } else {
        err(NTE_BAD_UID, FALSE)
    }
}

pub unsafe extern "win64" fn CryptGetProvParam(
    h_prov: usize,
    dw_param: u32,
    pb_data: *mut u8,
    pdw_data_len: *mut u32,
    _dw_flags: u32,
) -> BOOL {
    if pdw_data_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let provs = capi_providers().lock().unwrap();
    let Some(prov) = provs.get(&h_prov) else {
        return err(NTE_BAD_UID, FALSE);
    };
    let bytes: Vec<u8> = match dw_param {
        PP_NAME => {
            let mut v = prov.prov_name.as_bytes().to_vec();
            v.push(0);
            v
        }
        PP_CONTAINER => {
            let mut v = prov.container.as_bytes().to_vec();
            v.push(0);
            v
        }
        PP_PROVTYPE => prov.prov_type.to_le_bytes().to_vec(),
        PP_VERSION => 0x0000_0200u32.to_le_bytes().to_vec(),
        PP_IMPTYPE => 2u32.to_le_bytes().to_vec(), // CRYPT_IMPL_SOFTWARE
        _ => return err(NTE_BAD_TYPE, FALSE),
    };
    let need = bytes.len() as u32;
    if pb_data.is_null() {
        std::ptr::write_unaligned(pdw_data_len, need);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pdw_data_len);
    std::ptr::write_unaligned(pdw_data_len, need);
    if cap < need {
        return err(234 /* ERROR_MORE_DATA */, FALSE);
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), pb_data, bytes.len());
    TRUE
}

pub unsafe extern "win64" fn CryptCreateHash(
    h_prov: usize,
    alg_id: u32,
    _h_key: usize,
    _dw_flags: u32,
    ph_hash: *mut usize,
) -> BOOL {
    if ph_hash.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if !capi_providers().lock().unwrap().contains_key(&h_prov) {
        return err(NTE_BAD_UID, FALSE);
    }
    let Some(state) = capi_hash_init(alg_id) else {
        return err(NTE_BAD_ALGID, FALSE);
    };
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    capi_hashes().lock().unwrap().insert(h, state);
    std::ptr::write_unaligned(ph_hash, h);
    TRUE
}

pub unsafe extern "win64" fn CryptHashData(
    h_hash: usize,
    pb_data: *const u8,
    dw_data_len: u32,
    _dw_flags: u32,
) -> BOOL {
    if dw_data_len > 0 && pb_data.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let mut hashes = capi_hashes().lock().unwrap();
    let Some(h) = hashes.get_mut(&h_hash) else {
        return err(NTE_BAD_HASH, FALSE);
    };
    if dw_data_len > 0 {
        capi_hash_update(h, pb_data, dw_data_len);
        h.override_val = None;
    }
    TRUE
}

pub unsafe extern "win64" fn CryptGetHashParam(
    h_hash: usize,
    dw_param: u32,
    pb_data: *mut u8,
    pdw_data_len: *mut u32,
    _dw_flags: u32,
) -> BOOL {
    if pdw_data_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let hashes = capi_hashes().lock().unwrap();
    let Some(h) = hashes.get(&h_hash) else {
        return err(NTE_BAD_HASH, FALSE);
    };
    let bytes: Vec<u8> = match dw_param {
        HP_ALGID => h.alg_id.to_le_bytes().to_vec(),
        HP_HASHSIZE => (capi_hash_len(h.alg_id).unwrap_or(0) as u32).to_le_bytes().to_vec(),
        HP_HASHVAL => {
            if let Some(ref ov) = h.override_val {
                ov.clone()
            } else {
                let Some(d) = capi_hash_finalize(h) else {
                    return err(NTE_BAD_ALGID, FALSE);
                };
                d
            }
        }
        _ => return err(NTE_BAD_TYPE, FALSE),
    };
    let need = bytes.len() as u32;
    if pb_data.is_null() {
        std::ptr::write_unaligned(pdw_data_len, need);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pdw_data_len);
    std::ptr::write_unaligned(pdw_data_len, need);
    if cap < need {
        return err(234 /* ERROR_MORE_DATA */, FALSE);
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), pb_data, bytes.len());
    TRUE
}

pub unsafe extern "win64" fn CryptSetHashParam(
    h_hash: usize,
    dw_param: u32,
    pb_data: *const u8,
    _dw_flags: u32,
) -> BOOL {
    if pb_data.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let mut hashes = capi_hashes().lock().unwrap();
    let Some(h) = hashes.get_mut(&h_hash) else {
        return err(NTE_BAD_HASH, FALSE);
    };
    if dw_param == HP_HASHVAL {
        let Some(len) = capi_hash_len(h.alg_id) else {
            return err(NTE_BAD_ALGID, FALSE);
        };
        h.override_val = Some(std::slice::from_raw_parts(pb_data, len).to_vec());
        return TRUE;
    }
    err(NTE_BAD_TYPE, FALSE)
}

pub unsafe extern "win64" fn CryptDestroyHash(h_hash: usize) -> BOOL {
    if capi_hashes().lock().unwrap().remove(&h_hash).is_some() {
        TRUE
    } else {
        err(NTE_BAD_HASH, FALSE)
    }
}

// @exports CryptDecrypt CryptEncrypt CryptDestroyKey CryptExportKey CryptImportKey CryptGetUserKey CryptSignHashW
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    CryptDecrypt, CryptEncrypt, CryptDestroyKey, CryptExportKey,
    CryptImportKey, CryptGetUserKey, CryptSignHashW);

/// `RtlGenRandom` alias exported by advapi32.dll as `SystemFunction036`.
pub unsafe extern "win64" fn SystemFunction036(buf: *mut c_void, len: u32) -> u8 {
    if !buf.is_null() && crate::det_entropy::fill(buf as *mut u8, len as usize) {
        return 1;
    }
    crate::ntdll::RtlGenRandom(buf, len)
}

pub unsafe extern "win64" fn CryptGenRandom(_h_prov: usize, len: u32, buf: *mut u8) -> BOOL {
    if !buf.is_null() && crate::det_entropy::fill(buf, len as usize) {
        return TRUE;
    }
    crate::ntdll::RtlGenRandom(buf as *mut c_void, len) as BOOL
}

/// No legacy CSP registry entries are enumerated.
pub unsafe extern "win64" fn CryptEnumProvidersW() -> BOOL {
    err(ERROR_NO_MORE_ITEMS, FALSE)
}

/// Windows x64 `CREDENTIALW` (80 bytes).
#[repr(C)]
struct CredentialW {
    flags: u32,
    cred_type: u32,
    target_name: *mut u16,
    comment: *mut u16,
    last_written: FILETIME,
    credential_blob_size: u32,
    _pad: u32,
    credential_blob: *mut u8,
    persist: u32,
    attribute_count: u32,
    attributes: *mut c_void,
    target_alias: *mut u16,
    user_name: *mut u16,
}
const _: () = assert!(std::mem::size_of::<CredentialW>() == 80);

#[derive(Clone)]
struct StoredCredential {
    flags: u32,
    cred_type: u32,
    target_name: String,
    comment: Option<String>,
    last_written: FILETIME,
    blob: Vec<u8>,
    persist: u32,
    target_alias: Option<String>,
    user_name: Option<String>,
}

const CRED_PERSIST_SESSION: u32 = 1;
const CRED_PERSIST_LOCAL_MACHINE: u32 = 2;
const CRED_PERSIST_ENTERPRISE: u32 = 3;
const CRED_VAULT_MAGIC: &[u8; 8] = b"PFCRED01";
const CRED_VAULT_ENTROPY_LOCAL: &[u8] = b"advapi32-cred-local-v1";
const CRED_VAULT_ENTROPY_ENTERPRISE: &[u8] = b"advapi32-cred-enterprise-v1";
const ERROR_WRITE_FAULT_CRED: u32 = 29;

fn cred_store_lock() -> &'static std::sync::Mutex<()> {
    static L: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    L.get_or_init(|| std::sync::Mutex::new(()))
}

/// Cross-process + in-process lock guard for the credential vault.
/// Holds the in-process mutex and an exclusive POSIX `flock(LOCK_EX)` on `.vault.lock`
/// so concurrent processes serialise load-modify-rename writes cleanly.
struct CredStoreGuard {
    _mem: std::sync::MutexGuard<'static, ()>,
    flock_file: Option<std::fs::File>,
}

impl Drop for CredStoreGuard {
    fn drop(&mut self) {
        if let Some(ref f) = self.flock_file {
            use std::os::unix::io::AsRawFd;
            unsafe {
                libc::flock(f.as_raw_fd(), libc::LOCK_UN);
            }
        }
    }
}

fn acquire_cred_store_lock() -> std::io::Result<CredStoreGuard> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    use std::os::unix::io::AsRawFd;

    let mem = cred_store_lock().lock().unwrap();
    let lock_path = crate::path::default_translator_root()
        .join("AppData")
        .join("Local")
        .join("Microsoft")
        .join("Credentials")
        .join(".vault.lock");
    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent)?;
        let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .open(&lock_path)?;
    let _ = std::fs::set_permissions(&lock_path, std::fs::Permissions::from_mode(0o600));
    loop {
        let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) };
        if rc == 0 {
            break;
        }
        let e = std::io::Error::last_os_error();
        if e.kind() != std::io::ErrorKind::Interrupted {
            return Err(e);
        }
    }
    Ok(CredStoreGuard {
        _mem: mem,
        flock_file: Some(file),
    })
}

fn acquire_cred_store_lock_best_effort() -> CredStoreGuard {
    match acquire_cred_store_lock() {
        Ok(g) => g,
        Err(_) => CredStoreGuard {
            _mem: cred_store_lock().lock().unwrap(),
            flock_file: None,
        },
    }
}

/// Session-scoped (`CRED_PERSIST_SESSION`) credentials stay in memory only.
fn session_cred_store() -> &'static std::sync::Mutex<std::collections::HashMap<(String, u32), StoredCredential>> {
    static M: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<(String, u32), StoredCredential>>,
    > = std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn cred_allocs() -> &'static std::sync::Mutex<std::collections::HashSet<usize>> {
    static S: std::sync::OnceLock<std::sync::Mutex<std::collections::HashSet<usize>>> =
        std::sync::OnceLock::new();
    S.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CredVaultKind {
    LocalMachine,
    Enterprise,
}

impl CredVaultKind {
    fn path(self) -> std::path::PathBuf {
        let root = crate::path::default_translator_root();
        match self {
            CredVaultKind::LocalMachine => root
                .join("AppData")
                .join("Local")
                .join("Microsoft")
                .join("Credentials")
                .join("vault_local.bin"),
            CredVaultKind::Enterprise => root
                .join("AppData")
                .join("Roaming")
                .join("Microsoft")
                .join("Credentials")
                .join("vault_enterprise.bin"),
        }
    }

    fn dpapi_local_machine(self) -> bool {
        matches!(self, CredVaultKind::LocalMachine)
    }

    fn entropy(self) -> &'static [u8] {
        match self {
            CredVaultKind::LocalMachine => CRED_VAULT_ENTROPY_LOCAL,
            CredVaultKind::Enterprise => CRED_VAULT_ENTROPY_ENTERPRISE,
        }
    }

    fn descr(self) -> &'static str {
        match self {
            CredVaultKind::LocalMachine => "CredVaultLocal",
            CredVaultKind::Enterprise => "CredVaultEnterprise",
        }
    }
}

fn encode_cred_vault(map: &std::collections::HashMap<(String, u32), StoredCredential>) -> Vec<u8> {
    fn push_bytes(out: &mut Vec<u8>, b: &[u8]) {
        out.extend_from_slice(&(b.len() as u32).to_le_bytes());
        out.extend_from_slice(b);
    }
    fn push_opt_str(out: &mut Vec<u8>, s: &Option<String>) {
        match s {
            Some(v) => {
                out.push(1);
                push_bytes(out, v.as_bytes());
            }
            None => out.push(0),
        }
    }

    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(CRED_VAULT_MAGIC);
    out.extend_from_slice(&(map.len() as u32).to_le_bytes());
    for entry in map.values() {
        out.extend_from_slice(&entry.flags.to_le_bytes());
        out.extend_from_slice(&entry.cred_type.to_le_bytes());
        out.extend_from_slice(&entry.persist.to_le_bytes());
        let ft = ((entry.last_written.dwHighDateTime as u64) << 32)
            | (entry.last_written.dwLowDateTime as u64);
        out.extend_from_slice(&ft.to_le_bytes());
        push_bytes(&mut out, entry.target_name.as_bytes());
        push_opt_str(&mut out, &entry.comment);
        push_opt_str(&mut out, &entry.target_alias);
        push_opt_str(&mut out, &entry.user_name);
        push_bytes(&mut out, &entry.blob);
    }
    out
}

fn decode_cred_vault(data: &[u8]) -> Option<std::collections::HashMap<(String, u32), StoredCredential>> {
    if data.len() < 12 || &data[..8] != CRED_VAULT_MAGIC {
        return None;
    }
    let mut pos = 8usize;
    let read_u32 = |pos: &mut usize| -> Option<u32> {
        let end = pos.checked_add(4)?;
        let v = u32::from_le_bytes(data.get(*pos..end)?.try_into().ok()?);
        *pos = end;
        Some(v)
    };
    let read_u64 = |pos: &mut usize| -> Option<u64> {
        let end = pos.checked_add(8)?;
        let v = u64::from_le_bytes(data.get(*pos..end)?.try_into().ok()?);
        *pos = end;
        Some(v)
    };
    let read_bytes = |pos: &mut usize| -> Option<Vec<u8>> {
        let len = read_u32(pos)? as usize;
        let end = pos.checked_add(len)?;
        let v = data.get(*pos..end)?.to_vec();
        *pos = end;
        Some(v)
    };
    let read_opt_str = |pos: &mut usize| -> Option<Option<String>> {
        let tag = *data.get(*pos)?;
        *pos += 1;
        match tag {
            0 => Some(None),
            1 => {
                let b = read_bytes(pos)?;
                Some(Some(String::from_utf8(b).ok()?))
            }
            _ => None,
        }
    };

    let count = read_u32(&mut pos)? as usize;
    let mut map = std::collections::HashMap::with_capacity(count.min(1024));
    for _ in 0..count {
        let flags = read_u32(&mut pos)?;
        let cred_type = read_u32(&mut pos)?;
        let persist = read_u32(&mut pos)?;
        let ft = read_u64(&mut pos)?;
        let target_name = String::from_utf8(read_bytes(&mut pos)?).ok()?;
        let comment = read_opt_str(&mut pos)?;
        let target_alias = read_opt_str(&mut pos)?;
        let user_name = read_opt_str(&mut pos)?;
        let blob = read_bytes(&mut pos)?;
        let key = (target_name.to_ascii_lowercase(), cred_type);
        map.insert(
            key,
            StoredCredential {
                flags,
                cred_type,
                target_name,
                comment,
                last_written: FILETIME::from_u64(ft),
                blob,
                persist,
                target_alias,
                user_name,
            },
        );
    }
    Some(map)
}

unsafe fn load_persisted_vault(
    kind: CredVaultKind,
) -> std::io::Result<std::collections::HashMap<(String, u32), StoredCredential>> {
    let path = kind.path();
    let enc = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(std::collections::HashMap::new());
        }
        Err(e) => return Err(e),
    };
    let (_, plain) = crate::misc::crypt32::dpapi_unprotect_bytes(kind.entropy(), &enc)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "failed to decrypt persisted credential vault",
            )
        })?;
    decode_cred_vault(&plain).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "corrupt persisted credential vault",
        )
    })
}

unsafe fn save_persisted_vault(
    kind: CredVaultKind,
    map: &std::collections::HashMap<(String, u32), StoredCredential>,
) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let path = kind.path();
    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "vault path has no parent")
    })?;
    std::fs::create_dir_all(parent)?;
    let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));

    let raw = encode_cred_vault(map);
    let enc = crate::misc::crypt32::dpapi_protect_bytes(
        kind.dpapi_local_machine(),
        kind.descr(),
        kind.entropy(),
        &raw,
    )
    .ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "DPAPI encryption of credential vault failed",
        )
    })?;

    let mut rnd = [0u8; 8];
    libc::arc4random_buf(rnd.as_mut_ptr() as *mut c_void, rnd.len());
    let suffix = u64::from_le_bytes(rnd);
    let tmp_path = parent.join(format!(
        ".{}.tmp.{}.{suffix:016x}",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("vault.bin"),
        std::process::id()
    ));
    {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&tmp_path)?;
        f.write_all(&enc)?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp_path, &path)?;
    let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    Ok(())
}

const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1;
const ERROR_INVALID_FLAGS_CRED: u32 = 1004;

unsafe fn find_existing_cred_blob_locked(key: &(String, u32)) -> Option<Vec<u8>> {
    if let Some(e) = session_cred_store().lock().unwrap().get(key) {
        return Some(e.blob.clone());
    }
    for kind in [CredVaultKind::LocalMachine, CredVaultKind::Enterprise] {
        if let Ok(map) = load_persisted_vault(kind) {
            if let Some(e) = map.get(key) {
                return Some(e.blob.clone());
            }
        }
    }
    None
}

pub unsafe extern "win64" fn CredWriteW(cred_ptr: *const c_void, flags: u32) -> BOOL {
    if cred_ptr.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if (flags & !CRED_PRESERVE_CREDENTIAL_BLOB) != 0 {
        return err(ERROR_INVALID_FLAGS_CRED, FALSE);
    }
    let preserve_blob = (flags & CRED_PRESERVE_CREDENTIAL_BLOB) != 0;
    let c = std::ptr::read_unaligned(cred_ptr as *const CredentialW);
    if !matches!(
        c.persist,
        CRED_PERSIST_SESSION | CRED_PERSIST_LOCAL_MACHINE | CRED_PERSIST_ENTERPRISE
    ) {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if preserve_blob && c.credential_blob_size != 0 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let Some(target) = crate::strings::from_wide_opt(c.target_name) else {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    };
    let input_blob = if c.credential_blob_size > 0 && !c.credential_blob.is_null() {
        std::slice::from_raw_parts(c.credential_blob, c.credential_blob_size as usize).to_vec()
    } else {
        Vec::new()
    };
    let key = (target.to_ascii_lowercase(), c.cred_type);
    let comment = crate::strings::from_wide_opt(c.comment);
    let target_alias = crate::strings::from_wide_opt(c.target_alias);
    let user_name = crate::strings::from_wide_opt(c.user_name);

    match c.persist {
        CRED_PERSIST_SESSION => {
            let _guard = acquire_cred_store_lock_best_effort();
            let blob = if preserve_blob {
                let Some(existing) = find_existing_cred_blob_locked(&key) else {
                    return err(ERROR_NOT_FOUND, FALSE);
                };
                existing
            } else {
                input_blob
            };
            let entry = StoredCredential {
                flags: c.flags,
                cred_type: c.cred_type,
                target_name: target,
                comment,
                last_written: FILETIME::from_u64(crate::types::now_filetime()),
                blob,
                persist: c.persist,
                target_alias,
                user_name,
            };
            session_cred_store().lock().unwrap().insert(key.clone(), entry);
            // Replace any stale persisted entry for the same (target, type).
            for kind in [CredVaultKind::LocalMachine, CredVaultKind::Enterprise] {
                if let Ok(mut map) = load_persisted_vault(kind) {
                    if map.remove(&key).is_some() {
                        let _ = save_persisted_vault(kind, &map);
                    }
                }
            }
            TRUE
        }
        CRED_PERSIST_LOCAL_MACHINE | CRED_PERSIST_ENTERPRISE => {
            let _guard = match acquire_cred_store_lock() {
                Ok(g) => g,
                Err(_) => return err(ERROR_WRITE_FAULT_CRED, FALSE),
            };
            let blob = if preserve_blob {
                let Some(existing) = find_existing_cred_blob_locked(&key) else {
                    return err(ERROR_NOT_FOUND, FALSE);
                };
                existing
            } else {
                input_blob
            };
            let entry = StoredCredential {
                flags: c.flags,
                cred_type: c.cred_type,
                target_name: target,
                comment,
                last_written: FILETIME::from_u64(crate::types::now_filetime()),
                blob,
                persist: c.persist,
                target_alias,
                user_name,
            };
            let (target_kind, other_kind) = if c.persist == CRED_PERSIST_LOCAL_MACHINE {
                (CredVaultKind::LocalMachine, CredVaultKind::Enterprise)
            } else {
                (CredVaultKind::Enterprise, CredVaultKind::LocalMachine)
            };
            let mut map = match load_persisted_vault(target_kind) {
                Ok(m) => m,
                Err(_) => return err(ERROR_WRITE_FAULT_CRED, FALSE),
            };
            map.insert(key.clone(), entry);
            if save_persisted_vault(target_kind, &map).is_err() {
                return err(ERROR_WRITE_FAULT_CRED, FALSE);
            }
            session_cred_store().lock().unwrap().remove(&key);
            if let Ok(mut other) = load_persisted_vault(other_kind) {
                if other.remove(&key).is_some() {
                    let _ = save_persisted_vault(other_kind, &other);
                }
            }
            TRUE
        }
        _ => err(ERROR_INVALID_PARAMETER, FALSE),
    }
}

pub unsafe extern "win64" fn CredReadW(target: LPCWSTR, ty: u32, _flags: u32, out: *mut *mut c_void) -> BOOL {
    if out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let Some(name) = crate::strings::from_wide_opt(target) else {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    };
    let key = (name.to_ascii_lowercase(), ty);
    let entry = {
        let _guard = acquire_cred_store_lock_best_effort();
        if let Some(e) = session_cred_store().lock().unwrap().get(&key).cloned() {
            Some(e)
        } else {
            let mut found = None;
            for kind in [CredVaultKind::LocalMachine, CredVaultKind::Enterprise] {
                if let Ok(map) = load_persisted_vault(kind) {
                    if let Some(e) = map.get(&key).cloned() {
                        found = Some(e);
                        break;
                    }
                }
            }
            found
        }
    };
    let Some(e) = entry else {
        return err(ERROR_NOT_FOUND, FALSE);
    };

    // Pack `CredentialW` + strings + blob into a single heap allocation so `CredFree` frees all of it.
    let target_w = crate::strings::to_wide_z(&e.target_name);
    let comment_w = e.comment.as_deref().map(crate::strings::to_wide_z);
    let alias_w = e.target_alias.as_deref().map(crate::strings::to_wide_z);
    let user_w = e.user_name.as_deref().map(crate::strings::to_wide_z);

    let mut total = std::mem::size_of::<CredentialW>();
    let target_off = total;
    total += target_w.len() * 2;
    let comment_off = comment_w.as_ref().map(|w| { let o = total; total += w.len() * 2; o });
    let alias_off = alias_w.as_ref().map(|w| { let o = total; total += w.len() * 2; o });
    let user_off = user_w.as_ref().map(|w| { let o = total; total += w.len() * 2; o });
    let blob_off = total;
    total += e.blob.len();

    let base = crate::heap::alloc(total, true) as *mut u8;
    if base.is_null() {
        return err(crate::types::err::ERROR_NOT_ENOUGH_MEMORY, FALSE);
    }
    let write_w = |off: usize, units: &[u16]| -> *mut u16 {
        let p = base.add(off) as *mut u16;
        for (i, &u) in units.iter().enumerate() {
            std::ptr::write_unaligned(p.add(i), u);
        }
        p
    };
    let target_ptr = write_w(target_off, &target_w);
    let comment_ptr = comment_off
        .map(|o| write_w(o, comment_w.as_ref().unwrap()))
        .unwrap_or(std::ptr::null_mut());
    let alias_ptr = alias_off
        .map(|o| write_w(o, alias_w.as_ref().unwrap()))
        .unwrap_or(std::ptr::null_mut());
    let user_ptr = user_off
        .map(|o| write_w(o, user_w.as_ref().unwrap()))
        .unwrap_or(std::ptr::null_mut());
    let blob_ptr = if e.blob.is_empty() {
        std::ptr::null_mut()
    } else {
        let p = base.add(blob_off);
        std::ptr::copy_nonoverlapping(e.blob.as_ptr(), p, e.blob.len());
        p
    };
    std::ptr::write_unaligned(
        base as *mut CredentialW,
        CredentialW {
            flags: e.flags,
            cred_type: e.cred_type,
            target_name: target_ptr,
            comment: comment_ptr,
            last_written: e.last_written,
            credential_blob_size: e.blob.len() as u32,
            _pad: 0,
            credential_blob: blob_ptr,
            persist: e.persist,
            attribute_count: 0,
            attributes: std::ptr::null_mut(),
            target_alias: alias_ptr,
            user_name: user_ptr,
        },
    );
    cred_allocs().lock().unwrap().insert(base as usize);
    std::ptr::write_unaligned(out, base as *mut c_void);
    TRUE
}

pub unsafe extern "win64" fn CredDeleteW(target: LPCWSTR, ty: u32, _flags: u32) -> BOOL {
    let Some(name) = crate::strings::from_wide_opt(target) else {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    };
    let key = (name.to_ascii_lowercase(), ty);
    let _guard = acquire_cred_store_lock_best_effort();
    if session_cred_store().lock().unwrap().remove(&key).is_some() {
        return TRUE;
    }
    for kind in [CredVaultKind::LocalMachine, CredVaultKind::Enterprise] {
        if let Ok(mut map) = load_persisted_vault(kind) {
            if map.remove(&key).is_some() {
                if save_persisted_vault(kind, &map).is_err() {
                    return err(ERROR_WRITE_FAULT_CRED, FALSE);
                }
                return TRUE;
            }
        }
    }
    err(ERROR_NOT_FOUND, FALSE)
}

pub unsafe extern "win64" fn CredFree(p: *mut c_void) {
    if p.is_null() {
        return;
    }
    if cred_allocs().lock().unwrap().remove(&(p as usize)) {
        crate::heap::free(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_empty() {
        unsafe {
            let hkcr = 0xFFFF_FFFF_8000_0000usize as HANDLE;
            let hklm = 0xFFFF_FFFF_8000_0002usize as HANDLE;
            let sub = crate::strings::to_wide_z("SOFTWARE\\NonExistentVendor");
            let mut out = std::ptr::null_mut();
            assert_eq!(RegOpenKeyExW(hklm, sub.as_ptr(), 0, 0, &mut out), ERROR_FILE_NOT_FOUND);
            assert!(out.is_null());
            let mut n = 99u32;
            assert_eq!(
                RegQueryInfoKeyW(hkcr, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), &mut n, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(n, 0);

            // Verify pre-populated .NET Framework 4.8 key (Release = 528040)
            let ndp_sub = crate::strings::to_wide_z("SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Full");
            let mut hndp = std::ptr::null_mut();
            assert_eq!(RegOpenKeyExW(hklm, ndp_sub.as_ptr(), 0, 0x20019, &mut hndp), ERROR_SUCCESS);
            let rel_name = crate::strings::to_wide_z("Release");
            let mut rel_ty = 0u32;
            let mut rel_val = 0u32;
            let mut rel_len = 4u32;
            assert_eq!(
                RegQueryValueExW(hndp, rel_name.as_ptr(), std::ptr::null_mut(), &mut rel_ty, &mut rel_val as *mut u32 as *mut u8, &mut rel_len),
                ERROR_SUCCESS
            );
            assert_eq!(rel_ty, REG_DWORD);
            assert_eq!(rel_val, 528040);
            assert_eq!(RegCloseKey(hndp), ERROR_SUCCESS);

            let hkcu = 0xFFFF_FFFF_8000_0001usize as HANDLE;
            let epic_sub = crate::strings::to_wide_z("Software\\Epic Games\\Unreal Engine\\Identifiers");
            let mut hkey = std::ptr::null_mut();
            let mut disp = 0u32;
            assert_eq!(RegCreateKeyExW(hkcu, epic_sub.as_ptr(), 0, std::ptr::null_mut(), 0, 0x20006, std::ptr::null_mut(), &mut hkey, &mut disp), ERROR_SUCCESS);
            assert_eq!(disp, REG_CREATED_NEW_KEY);
            let val_name = crate::strings::to_wide_z("AccountId");
            let val_data = crate::strings::to_wide_z("1234567890abcdef");
            let val_bytes = (val_data.len() * 2) as u32;
            assert_eq!(RegSetValueExW(hkey, val_name.as_ptr(), 0, REG_SZ, val_data.as_ptr() as *const u8, val_bytes), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hkey), ERROR_SUCCESS);

            let mut hopen = std::ptr::null_mut();
            assert_eq!(RegOpenKeyExW(hkcu, epic_sub.as_ptr(), 0, 0x20019, &mut hopen), ERROR_SUCCESS);
            let mut ty = 0u32;
            let mut read_buf = vec![0u16; val_data.len()];
            let mut read_len = val_bytes;
            assert_eq!(RegQueryValueExW(hopen, val_name.as_ptr(), std::ptr::null_mut(), &mut ty, read_buf.as_mut_ptr() as *mut u8, &mut read_len), ERROR_SUCCESS);
            assert_eq!(ty, REG_SZ);
            assert_eq!(read_buf, val_data);
            assert_eq!(RegCloseKey(hopen), ERROR_SUCCESS);
            let top_sub = crate::strings::to_wide_z("Software");
            assert_eq!(RegDeleteTreeW(hkcu, top_sub.as_ptr()), ERROR_SUCCESS);

            // Verify pre-populated Windows NT\CurrentVersion, BIOS, and CentralProcessor\0 keys
            for sub_path in [
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\",
                "HARDWARE\\DESCRIPTION\\System\\BIOS",
                "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0",
                "SOFTWARE\\Microsoft\\Cryptography",
            ] {
                let w = crate::strings::to_wide_z(sub_path);
                let mut h = std::ptr::null_mut();
                assert_eq!(RegOpenKeyExW(hklm, w.as_ptr(), 0, 0x20019, &mut h), ERROR_SUCCESS, "missing {sub_path}");
                assert_eq!(RegCloseKey(h), ERROR_SUCCESS);
            }
        }
    }

    #[test]
    fn privileges() {
        unsafe {
            let mut luid = 0u64;
            assert_eq!(LookupPrivilegeValueA(std::ptr::null(), b"SeLockMemoryPrivilege\0".as_ptr(), &mut luid), TRUE);
            assert_eq!(luid, 4);
            let mut tok = std::ptr::null_mut();
            assert_eq!(OpenProcessToken(handles::CURRENT_PROCESS, 0x28, &mut tok), TRUE);
            let tp: [u32; 4] = [1, 4, 0, 2]; // count, luid lo, luid hi, SE_PRIVILEGE_ENABLED
            assert_eq!(AdjustTokenPrivileges(tok, FALSE, tp.as_ptr(), 16, std::ptr::null_mut(), std::ptr::null_mut()), TRUE);
            assert_eq!(crate::teb::last_error(), ERROR_NOT_ALL_ASSIGNED);
            assert!(handles::close(tok));
        }
    }

    #[test]
    fn user_name() {
        unsafe {
            let mut size = 0u32;
            assert_eq!(GetUserNameW(std::ptr::null_mut(), &mut size), FALSE);
            let mut buf = vec![0u16; size as usize];
            assert_eq!(GetUserNameW(buf.as_mut_ptr(), &mut size), TRUE);
            assert_eq!(crate::strings::from_wide(buf.as_ptr()), host_user());
        }
    }

    #[test]
    fn display_adapter_registry_keys_seeded() {
        unsafe {
            let hklm = 0xFFFF_FFFF_8000_0002usize as HANDLE;
            let class_path = crate::strings::to_wide_z(
                "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}",
            );
            let mut hclass = std::ptr::null_mut();
            assert_eq!(RegOpenKeyExW(hklm, class_path.as_ptr(), 0, 0x20019, &mut hclass), ERROR_SUCCESS);
            let mut sub_buf = [0u16; 32];
            let mut sub_len = sub_buf.len() as u32;
            assert_eq!(
                RegEnumKeyExW(
                    hclass,
                    0,
                    sub_buf.as_mut_ptr(),
                    &mut sub_len,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                ERROR_SUCCESS
            );
            assert_eq!(crate::strings::from_wide(sub_buf.as_ptr()), "0000");

            let sub_0000 = crate::strings::to_wide_z("0000");
            let mut hdev = std::ptr::null_mut();
            assert_eq!(RegOpenKeyExW(hclass, sub_0000.as_ptr(), 0, 0x20019, &mut hdev), ERROR_SUCCESS);

            let read_sz = |name: &str| -> String {
                let wname = crate::strings::to_wide_z(name);
                let mut ty = 0u32;
                let mut len = 0u32;
                assert_eq!(
                    RegQueryValueExW(hdev, wname.as_ptr(), std::ptr::null_mut(), &mut ty, std::ptr::null_mut(), &mut len),
                    ERROR_SUCCESS,
                    "missing REG_SZ value {name}"
                );
                assert_eq!(ty, REG_SZ, "{name} expected REG_SZ");
                let mut buf = vec![0u8; len as usize];
                let mut len2 = len;
                assert_eq!(
                    RegQueryValueExW(hdev, wname.as_ptr(), std::ptr::null_mut(), &mut ty, buf.as_mut_ptr(), &mut len2),
                    ERROR_SUCCESS
                );
                let u16s: Vec<u16> = buf.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
                let end = u16s.iter().position(|&c| c == 0).unwrap_or(u16s.len());
                String::from_utf16_lossy(&u16s[..end])
            };

            assert_eq!(read_sz("DriverVersion"), crate::dxgi::adapter::UMD_DRIVER_VERSION_STR);
            assert_eq!(read_sz("DriverDate"), crate::dxgi::adapter::UMD_DRIVER_DATE_STR);
            assert_eq!(read_sz("ProviderName"), "Apple Inc.");
            let expected_desc = crate::d3d12::mtl::device()
                .map(|d| d.name.clone())
                .unwrap_or_else(|| "Apple Silicon GPU".to_string());
            assert_eq!(read_sz("DriverDesc"), expected_desc);
            assert_eq!(read_sz("HardwareInformation.AdapterString"), expected_desc);
            assert_eq!(read_sz("HardwareInformation.ChipType"), expected_desc);

            let expected_dev_id = crate::d3d12::mtl::device()
                .map(|d| d.pci_device_id())
                .unwrap_or(0xc140);
            assert_eq!(
                read_sz("MatchingDeviceId"),
                format!("pci\\ven_{:04x}&dev_{:04x}", crate::dxgi::adapter::VENDOR_APPLE, expected_dev_id)
            );

            // Verify HardwareInformation.qwMemorySize (REG_QWORD).
            let qw_name = crate::strings::to_wide_z("HardwareInformation.qwMemorySize");
            let mut qw_ty = 0u32;
            let mut qw_val = 0u64;
            let mut qw_len = 8u32;
            assert_eq!(
                RegQueryValueExW(
                    hdev,
                    qw_name.as_ptr(),
                    std::ptr::null_mut(),
                    &mut qw_ty,
                    &mut qw_val as *mut u64 as *mut u8,
                    &mut qw_len,
                ),
                ERROR_SUCCESS
            );
            assert_eq!(qw_ty, REG_QWORD);
            assert_eq!(qw_len, 8);
            assert!(qw_val > 0);

            assert_eq!(RegCloseKey(hdev), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hclass), ERROR_SUCCESS);
        }
    }

    unsafe fn read_unaligned_utf16_z(ptr: *const u16, max_units: usize) -> Vec<u16> {
        let mut out = Vec::new();
        for i in 0..max_units {
            let u = std::ptr::read_unaligned(ptr.add(i));
            if u == 0 {
                return out;
            }
            out.push(u);
        }
        panic!("missing NUL terminator within {max_units} UTF-16 code units");
    }

    fn odd_sentinel_buf(payload_bytes: usize) -> (Vec<u8>, usize) {
        let backing = vec![0xA5u8; payload_bytes + 8];
        let odd_off = if (backing.as_ptr() as usize) & 1 == 0 { 1 } else { 2 };
        assert_eq!((backing.as_ptr() as usize + odd_off) & 1, 1);
        (backing, odd_off)
    }

    #[test]
    fn user_profile_directory_uses_vfs() {
        unsafe {
            let mut tok = std::ptr::null_mut();
            assert_eq!(OpenProcessToken(handles::CURRENT_PROCESS, 0x8, &mut tok), TRUE);
            let mut sz = 0u32;
            assert_eq!(GetUserProfileDirectoryW(tok, std::ptr::null_mut(), &mut sz), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert!(sz > 4);
            let expected_units = sz as usize;
            let mut buf = vec![0u16; expected_units];
            assert_eq!(GetUserProfileDirectoryW(tok, buf.as_mut_ptr(), &mut sz), TRUE);
            assert_eq!(sz as usize, expected_units);
            let dir = crate::strings::from_wide(buf.as_ptr());
            assert!(dir.to_ascii_lowercase().starts_with("c:\\users\\"));

            // Odd-address buffer: undersize probe preserves sentinels; exact size writes UTF-16 + NUL safely.
            let byte_len = expected_units * 2;
            let (mut odd_backing, odd_off) = odd_sentinel_buf(byte_len);
            let odd_ptr = odd_backing.as_mut_ptr().add(odd_off) as *mut u16;
            assert_eq!((odd_ptr as usize) & 1, 1);

            let mut small = (expected_units - 1) as u32;
            assert_eq!(GetUserProfileDirectoryW(tok, odd_ptr, &mut small), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert_eq!(small as usize, expected_units);
            assert!(odd_backing.iter().all(|&b| b == 0xA5));

            let mut exact = expected_units as u32;
            assert_eq!(GetUserProfileDirectoryW(tok, odd_ptr, &mut exact), TRUE);
            assert_eq!(exact as usize, expected_units);
            let got_units = read_unaligned_utf16_z(odd_ptr, expected_units);
            let expected_utf16: Vec<u16> = dir.encode_utf16().collect();
            assert_eq!(got_units, expected_utf16);
            assert_eq!(std::ptr::read_unaligned(odd_ptr.add(expected_utf16.len())), 0u16);
            assert!(odd_backing[..odd_off].iter().all(|&b| b == 0xA5));
            assert!(odd_backing[odd_off + byte_len..].iter().all(|&b| b == 0xA5));

            // Invalid token fails cleanly.
            assert_eq!(GetUserProfileDirectoryW(std::ptr::null_mut(), buf.as_mut_ptr(), &mut sz), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_HANDLE);
            assert!(handles::close(tok));
        }
    }

    #[test]
    fn registry_notify_change_async_subtree_and_close() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, ResetEvent, WaitForSingleObject};
        unsafe {
            let hkcu = 0xFFFF_FFFF_8000_0001usize as HANDLE;
            let root_sub = crate::strings::to_wide_z("Software\\PortFactoryNotifyTest");
            let mut hroot = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExW(hkcu, root_sub.as_ptr(), 0, std::ptr::null_mut(), 0, 0x2001F, std::ptr::null_mut(), &mut hroot, std::ptr::null_mut()),
                ERROR_SUCCESS
            );
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());

            // Watch subtree for value changes (LAST_SET).
            assert_eq!(RegNotifyChangeKeyValue(hroot, TRUE, REG_NOTIFY_CHANGE_LAST_SET, ev, TRUE), ERROR_SUCCESS);
            assert_eq!(WaitForSingleObject(ev, 0), WAIT_TIMEOUT);

            // Creating a subkey fires NAME, not LAST_SET -> event stays unsignaled.
            let child_sub = crate::strings::to_wide_z("Child");
            let mut hchild = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExW(hroot, child_sub.as_ptr(), 0, std::ptr::null_mut(), 0, 0x2001F, std::ptr::null_mut(), &mut hchild, std::ptr::null_mut()),
                ERROR_SUCCESS
            );
            assert_eq!(WaitForSingleObject(ev, 0), WAIT_TIMEOUT);

            // Setting a value on the child triggers subtree LAST_SET watcher!
            let val_name = crate::strings::to_wide_z("Tick");
            let v = 7u32;
            assert_eq!(RegSetValueExW(hchild, val_name.as_ptr(), 0, REG_DWORD, &v as *const u32 as *const u8, 4), ERROR_SUCCESS);
            assert_eq!(WaitForSingleObject(ev, 0), WAIT_OBJECT_0);

            // Synchronous mode unblocks when key is closed from another thread.
            assert_eq!(ResetEvent(ev), TRUE);
            let hroot_val = hroot as usize;
            let sync_done = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let sync_done2 = Arc::clone(&sync_done);
            let th = std::thread::spawn(move || {
                let rc = RegNotifyChangeKeyValue(hroot_val as HANDLE, FALSE, REG_NOTIFY_CHANGE_NAME, std::ptr::null_mut(), FALSE);
                assert_eq!(rc, ERROR_SUCCESS);
                sync_done2.store(true, Ordering::Release);
            });
            std::thread::sleep(std::time::Duration::from_millis(25));
            assert!(!sync_done.load(Ordering::Acquire));
            assert_eq!(RegCloseKey(hroot), ERROR_SUCCESS);
            th.join().unwrap();
            assert!(sync_done.load(Ordering::Acquire));

            assert_eq!(RegCloseKey(hchild), ERROR_SUCCESS);
            let _ = RegDeleteTreeW(hkcu, root_sub.as_ptr());
            assert_eq!(CloseHandle(ev), TRUE);
        }
    }

    #[test]
    fn registry_create_key_intermediate_notify_direct_subtree_and_reopen() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, ResetEvent, WaitForSingleObject};
        unsafe {
            let hkcu = 0xFFFF_FFFF_8000_0001usize as HANDLE;
            let parent_sub = crate::strings::to_wide_z("Software\\PortFactoryMultiLevelParent");
            let _ = RegDeleteTreeW(hkcu, parent_sub.as_ptr());
            let mut hparent = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExW(
                    hkcu,
                    parent_sub.as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    0x2001F,
                    std::ptr::null_mut(),
                    &mut hparent,
                    std::ptr::null_mut(),
                ),
                ERROR_SUCCESS
            );
            let root_name = crate::strings::to_wide_z("Root");
            let mut hroot = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExW(
                    hparent,
                    root_name.as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    0x2001F,
                    std::ptr::null_mut(),
                    &mut hroot,
                    std::ptr::null_mut(),
                ),
                ERROR_SUCCESS
            );

            // Auto-reset events so a single signal is consumed once (proving one-shot deduplication).
            let ev_root_direct = CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
            let ev_root_subtree = CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
            let ev_parent_direct = CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
            assert!(!ev_root_direct.is_null() && !ev_root_subtree.is_null() && !ev_parent_direct.is_null());

            // 1. Creating two missing levels together (`Mid\Leaf`) under `Root`:
            //    - `Root` direct watcher MUST fire (direct child `Mid` was created), once.
            //    - `Root` subtree watcher MUST fire, once (deduplicated across Mid + Leaf).
            //    - `Parent` direct watcher MUST NOT fire (no direct child of `Parent` was created).
            assert_eq!(
                RegNotifyChangeKeyValue(hroot, FALSE, REG_NOTIFY_CHANGE_NAME, ev_root_direct, TRUE),
                ERROR_SUCCESS
            );
            assert_eq!(
                RegNotifyChangeKeyValue(hroot, TRUE, REG_NOTIFY_CHANGE_NAME, ev_root_subtree, TRUE),
                ERROR_SUCCESS
            );
            assert_eq!(
                RegNotifyChangeKeyValue(hparent, FALSE, REG_NOTIFY_CHANGE_NAME, ev_parent_direct, TRUE),
                ERROR_SUCCESS
            );

            let two_levels = crate::strings::to_wide_z("Mid\\Leaf");
            let mut hleaf = std::ptr::null_mut();
            let mut disp = 0u32;
            assert_eq!(
                RegCreateKeyExW(
                    hroot,
                    two_levels.as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    0x2001F,
                    std::ptr::null_mut(),
                    &mut hleaf,
                    &mut disp,
                ),
                ERROR_SUCCESS
            );
            assert_eq!(disp, REG_CREATED_NEW_KEY);
            assert_eq!(WaitForSingleObject(ev_root_direct, 0), WAIT_OBJECT_0);
            assert_eq!(WaitForSingleObject(ev_root_direct, 0), WAIT_TIMEOUT);
            assert_eq!(WaitForSingleObject(ev_root_subtree, 0), WAIT_OBJECT_0);
            assert_eq!(WaitForSingleObject(ev_root_subtree, 0), WAIT_TIMEOUT);
            assert_eq!(WaitForSingleObject(ev_parent_direct, 0), WAIT_TIMEOUT);

            // 2. Reopening existing `Mid\Leaf` (both W and A) is a no-op: fires NO watchers.
            assert_eq!(
                RegNotifyChangeKeyValue(hroot, FALSE, REG_NOTIFY_CHANGE_NAME, ev_root_direct, TRUE),
                ERROR_SUCCESS
            );
            assert_eq!(
                RegNotifyChangeKeyValue(hroot, TRUE, REG_NOTIFY_CHANGE_NAME, ev_root_subtree, TRUE),
                ERROR_SUCCESS
            );
            let mut hreopen_w = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExW(
                    hroot,
                    two_levels.as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    0x2001F,
                    std::ptr::null_mut(),
                    &mut hreopen_w,
                    &mut disp,
                ),
                ERROR_SUCCESS
            );
            assert_eq!(disp, REG_OPENED_EXISTING_KEY);
            let mut hreopen_a = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExA(
                    hroot,
                    b"Mid\\Leaf\0".as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    0x2001F,
                    std::ptr::null_mut(),
                    &mut hreopen_a,
                    &mut disp,
                ),
                ERROR_SUCCESS
            );
            assert_eq!(disp, REG_OPENED_EXISTING_KEY);
            assert_eq!(WaitForSingleObject(ev_root_direct, 0), WAIT_TIMEOUT);
            assert_eq!(WaitForSingleObject(ev_root_subtree, 0), WAIT_TIMEOUT);

            // 3. Creating a grandchild (`Mid\Leaf2`) when `Mid` already exists:
            //    - `Root` direct watcher MUST NOT fire!
            //    - `Root` subtree watcher MUST fire!
            //    - `Mid` direct watcher MUST fire!
            let mid_w = crate::strings::to_wide_z("Mid");
            let mut hmid = std::ptr::null_mut();
            assert_eq!(RegOpenKeyExW(hroot, mid_w.as_ptr(), 0, 0x2001F, &mut hmid), ERROR_SUCCESS);
            let ev_mid_direct = CreateEventW(std::ptr::null_mut(), FALSE, FALSE, std::ptr::null());
            assert!(!ev_mid_direct.is_null());
            assert_eq!(
                RegNotifyChangeKeyValue(hmid, FALSE, REG_NOTIFY_CHANGE_NAME, ev_mid_direct, TRUE),
                ERROR_SUCCESS
            );
            let leaf2_w = crate::strings::to_wide_z("Mid\\Leaf2");
            let mut hleaf2 = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExW(
                    hroot,
                    leaf2_w.as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    0x2001F,
                    std::ptr::null_mut(),
                    &mut hleaf2,
                    &mut disp,
                ),
                ERROR_SUCCESS
            );
            assert_eq!(disp, REG_CREATED_NEW_KEY);
            assert_eq!(WaitForSingleObject(ev_root_direct, 0), WAIT_TIMEOUT);
            assert_eq!(WaitForSingleObject(ev_root_subtree, 0), WAIT_OBJECT_0);
            assert_eq!(WaitForSingleObject(ev_mid_direct, 0), WAIT_OBJECT_0);

            // 4. RegCreateKeyExA creating two missing levels together (`MidA\LeafA`) shares
            //    the exact same intermediate notification path and wakes `ev_root_direct` once!
            let mut hleaf_a = std::ptr::null_mut();
            assert_eq!(
                RegCreateKeyExA(
                    hroot,
                    b"MidA\\LeafA\0".as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    0x2001F,
                    std::ptr::null_mut(),
                    &mut hleaf_a,
                    &mut disp,
                ),
                ERROR_SUCCESS
            );
            assert_eq!(disp, REG_CREATED_NEW_KEY);
            assert_eq!(WaitForSingleObject(ev_root_direct, 0), WAIT_OBJECT_0);
            assert_eq!(WaitForSingleObject(ev_root_direct, 0), WAIT_TIMEOUT);
            assert_eq!(WaitForSingleObject(ev_parent_direct, 0), WAIT_TIMEOUT);

            // Cleanup.
            let _ = ResetEvent(ev_parent_direct);
            assert_eq!(RegCloseKey(hleaf_a), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hleaf2), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hmid), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hreopen_a), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hreopen_w), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hleaf), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hroot), ERROR_SUCCESS);
            assert_eq!(RegCloseKey(hparent), ERROR_SUCCESS);
            let _ = RegDeleteTreeW(hkcu, parent_sub.as_ptr());
            assert_eq!(CloseHandle(ev_mid_direct), TRUE);
            assert_eq!(CloseHandle(ev_parent_direct), TRUE);
            assert_eq!(CloseHandle(ev_root_subtree), TRUE);
            assert_eq!(CloseHandle(ev_root_direct), TRUE);
        }
    }

    #[test]
    fn user_mode_service_dispatch_control_and_status() {
        use std::sync::atomic::{AtomicBool, AtomicU32};

        static STOP_EV: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        static LAST_CTRL: AtomicU32 = AtomicU32::new(0);
        static HANDLER_RAN: AtomicBool = AtomicBool::new(false);
        static STATUS_HANDLE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

        unsafe extern "win64" fn test_ctrl_ex(ctrl: u32, _ev_ty: u32, _ev_data: *mut c_void, ctx: *mut c_void) -> u32 {
            assert_eq!(ctx as usize, 0xCAFE);
            LAST_CTRL.store(ctrl, Ordering::Release);
            HANDLER_RAN.store(true, Ordering::Release);
            if ctrl == SERVICE_CONTROL_STOP {
                let sh = STATUS_HANDLE.load(Ordering::Acquire) as HANDLE;
                let st = SERVICE_STATUS {
                    dwServiceType: SERVICE_WIN32_OWN_PROCESS,
                    dwCurrentState: SERVICE_STOPPED,
                    dwControlsAccepted: 0,
                    ..Default::default()
                };
                SetServiceStatus(sh, &st);
                if let Some(&ev) = STOP_EV.get() {
                    crate::kernel32::threads::SetEvent(ev as HANDLE);
                }
            }
            ERROR_SUCCESS
        }

        unsafe extern "win64" fn test_service_main(argc: u32, argv: *mut LPWSTR) {
            assert_eq!(argc, 1);
            let svc_name = std::ptr::read_unaligned(argv);
            let sh = RegisterServiceCtrlHandlerExW(svc_name, test_ctrl_ex as *const c_void, 0xCAFE as *mut c_void);
            assert!(!sh.is_null());
            STATUS_HANDLE.store(sh as usize, Ordering::Release);
            let running = SERVICE_STATUS {
                dwServiceType: SERVICE_WIN32_OWN_PROCESS,
                dwCurrentState: 4, // SERVICE_RUNNING
                dwControlsAccepted: SERVICE_ACCEPT_STOP,
                ..Default::default()
            };
            assert_eq!(SetServiceStatus(sh, &running), TRUE);
            if let Some(&ev) = STOP_EV.get() {
                crate::kernel32::threads::WaitForSingleObject(ev as HANDLE, 5000);
            }
        }

        unsafe {
            let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let _ = STOP_EV.set(ev as usize);
            LAST_CTRL.store(0, Ordering::Release);
            HANDLER_RAN.store(false, Ordering::Release);

            let scm = OpenSCManagerW(std::ptr::null(), std::ptr::null(), 0xF003F);
            assert!(!scm.is_null());
            // Remote machine rejected honestly.
            let remote = crate::strings::to_wide_z("\\\\REMOTE-HOST");
            assert!(OpenSCManagerW(remote.as_ptr(), std::ptr::null(), 0xF003F).is_null());

            let svc_name = crate::strings::to_wide_z("PortFactoryTestSvc");
            let bin_path = crate::strings::to_wide_z("C:\\Test\\svc.exe");
            let hsvc = CreateServiceW(
                scm,
                svc_name.as_ptr(),
                svc_name.as_ptr(),
                0xF01FF,
                SERVICE_WIN32_OWN_PROCESS,
                3,
                1,
                bin_path.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            );
            assert!(!hsvc.is_null());

            // ChangeServiceConfig2W + QueryServiceConfig2W description roundtrip (aligned).
            let desc = crate::strings::to_wide_z("Test Service Description");
            let desc_ptr = desc.as_ptr();
            assert_eq!(ChangeServiceConfig2W(hsvc, SERVICE_CONFIG_DESCRIPTION, &desc_ptr as *const _ as *const c_void), TRUE);
            let mut need = 0u32;
            assert_eq!(QueryServiceConfig2W(hsvc, SERVICE_CONFIG_DESCRIPTION, std::ptr::null_mut(), 0, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            let mut dbuf = vec![0u8; need as usize];
            assert_eq!(QueryServiceConfig2W(hsvc, SERVICE_CONFIG_DESCRIPTION, dbuf.as_mut_ptr(), need, &mut need), TRUE);
            let out_ptr = std::ptr::read_unaligned(dbuf.as_ptr() as *const LPCWSTR);
            assert_eq!(crate::strings::from_wide(out_ptr), "Test Service Description");

            // Odd-address + Unicode (including surrogate pair) tests for QueryServiceConfig2W & QueryServiceConfigW.
            let u_svc_name = crate::strings::to_wide_z("PortFactoryUnicodeSvc");
            let u_bin_str = "C:\\Test\\svc_\u{03A9}_\u{1F680}.exe";
            let u_disp_str = "Display \u{03A9} \u{1F680}";
            let u_desc_str = "Desc \u{03A9} \u{1F680}";
            let u_bin_w = crate::strings::to_wide_z(u_bin_str);
            let u_disp_w = crate::strings::to_wide_z(u_disp_str);
            let u_desc_w = crate::strings::to_wide_z(u_desc_str);
            let husvc = CreateServiceW(
                scm,
                u_svc_name.as_ptr(),
                u_disp_w.as_ptr(),
                0xF01FF,
                SERVICE_WIN32_OWN_PROCESS,
                2, // SERVICE_AUTO_START
                1, // SERVICE_ERROR_NORMAL
                u_bin_w.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            );
            assert!(!husvc.is_null());
            let u_desc_ptr = u_desc_w.as_ptr();
            assert_eq!(ChangeServiceConfig2W(husvc, SERVICE_CONFIG_DESCRIPTION, &u_desc_ptr as *const _ as *const c_void), TRUE);

            // QueryServiceConfig2W: size probe, undersize, exact odd-address buffer with sentinels, and aligned control.
            let desc_units = u_desc_str.encode_utf16().count() + 1;
            let expected_desc_need = (8 + desc_units * 2) as u32;
            let mut desc_need = 0u32;
            assert_eq!(QueryServiceConfig2W(husvc, SERVICE_CONFIG_DESCRIPTION, std::ptr::null_mut(), 0, &mut desc_need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert_eq!(desc_need, expected_desc_need);

            let (mut odd_d2_backing, odd_d2_off) = odd_sentinel_buf(desc_need as usize);
            let odd_d2_ptr = odd_d2_backing.as_mut_ptr().add(odd_d2_off);
            assert_eq!((odd_d2_ptr as usize) & 1, 1);
            let mut got_need = 0u32;
            assert_eq!(QueryServiceConfig2W(husvc, SERVICE_CONFIG_DESCRIPTION, odd_d2_ptr, desc_need - 1, &mut got_need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert_eq!(got_need, expected_desc_need);
            assert!(odd_d2_backing.iter().all(|&b| b == 0xA5));

            assert_eq!(QueryServiceConfig2W(husvc, SERVICE_CONFIG_DESCRIPTION, odd_d2_ptr, desc_need, &mut got_need), TRUE);
            assert_eq!(got_need, expected_desc_need);
            let odd_desc_out = std::ptr::read_unaligned(odd_d2_ptr as *const *const u16);
            assert_eq!(odd_desc_out as usize, odd_d2_ptr as usize + 8);
            let got_desc_units = read_unaligned_utf16_z(odd_desc_out, desc_units);
            assert_eq!(got_desc_units, u_desc_str.encode_utf16().collect::<Vec<u16>>());
            assert_eq!(std::ptr::read_unaligned(odd_desc_out.add(desc_units - 1)), 0u16);
            assert_eq!(String::from_utf16(&got_desc_units).unwrap(), u_desc_str);
            assert!(odd_d2_backing[..odd_d2_off].iter().all(|&b| b == 0xA5));
            assert!(odd_d2_backing[odd_d2_off + desc_need as usize..].iter().all(|&b| b == 0xA5));

            // QueryServiceConfigW: size probe, undersize, exact odd-address buffer with sentinels, and aligned control.
            let bin_units = u_bin_str.encode_utf16().count() + 1;
            let disp_units = u_disp_str.encode_utf16().count() + 1;
            let expected_cfg_need = (64 + (bin_units + disp_units) * 2) as u32;
            let mut cfg_need = 0u32;
            assert_eq!(QueryServiceConfigW(husvc, std::ptr::null_mut(), 0, &mut cfg_need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert_eq!(cfg_need, expected_cfg_need);

            let (mut odd_cfg_backing, odd_cfg_off) = odd_sentinel_buf(cfg_need as usize);
            let odd_cfg_ptr = odd_cfg_backing.as_mut_ptr().add(odd_cfg_off);
            assert_eq!((odd_cfg_ptr as usize) & 1, 1);
            let mut cfg_got_need = 0u32;
            assert_eq!(QueryServiceConfigW(husvc, odd_cfg_ptr, cfg_need - 1, &mut cfg_got_need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert_eq!(cfg_got_need, expected_cfg_need);
            assert!(odd_cfg_backing.iter().all(|&b| b == 0xA5));

            assert_eq!(QueryServiceConfigW(husvc, odd_cfg_ptr, cfg_need, &mut cfg_got_need), TRUE);
            assert_eq!(cfg_got_need, expected_cfg_need);
            assert_eq!(std::ptr::read_unaligned(odd_cfg_ptr as *const u32), SERVICE_WIN32_OWN_PROCESS);
            assert_eq!(std::ptr::read_unaligned(odd_cfg_ptr.add(4) as *const u32), 2);
            assert_eq!(std::ptr::read_unaligned(odd_cfg_ptr.add(8) as *const u32), 1);
            let odd_bin_out = std::ptr::read_unaligned(odd_cfg_ptr.add(16) as *const *const u16);
            let odd_disp_out = std::ptr::read_unaligned(odd_cfg_ptr.add(56) as *const *const u16);
            assert_eq!(odd_bin_out as usize, odd_cfg_ptr as usize + 64);
            assert_eq!(odd_disp_out as usize, odd_cfg_ptr as usize + 64 + bin_units * 2);
            let got_bin_units = read_unaligned_utf16_z(odd_bin_out, bin_units);
            let got_disp_units = read_unaligned_utf16_z(odd_disp_out, disp_units);
            assert_eq!(got_bin_units, u_bin_str.encode_utf16().collect::<Vec<u16>>());
            assert_eq!(got_disp_units, u_disp_str.encode_utf16().collect::<Vec<u16>>());
            assert_eq!(std::ptr::read_unaligned(odd_bin_out.add(bin_units - 1)), 0u16);
            assert_eq!(std::ptr::read_unaligned(odd_disp_out.add(disp_units - 1)), 0u16);
            assert_eq!(String::from_utf16(&got_bin_units).unwrap(), u_bin_str);
            assert_eq!(String::from_utf16(&got_disp_units).unwrap(), u_disp_str);
            assert!(odd_cfg_backing[..odd_cfg_off].iter().all(|&b| b == 0xA5));
            assert!(odd_cfg_backing[odd_cfg_off + cfg_need as usize..].iter().all(|&b| b == 0xA5));

            // Aligned QueryServiceConfigW control.
            let mut aligned_cfg = vec![0u64; (cfg_need as usize).div_ceil(8)];
            let aligned_ptr = aligned_cfg.as_mut_ptr() as *mut u8;
            assert_eq!(QueryServiceConfigW(husvc, aligned_ptr, cfg_need, &mut cfg_got_need), TRUE);
            let al_bin_out = std::ptr::read_unaligned(aligned_ptr.add(16) as *const *const u16);
            let al_disp_out = std::ptr::read_unaligned(aligned_ptr.add(56) as *const *const u16);
            assert_eq!(String::from_utf16(&read_unaligned_utf16_z(al_bin_out, bin_units)).unwrap(), u_bin_str);
            assert_eq!(String::from_utf16(&read_unaligned_utf16_z(al_disp_out, disp_units)).unwrap(), u_disp_str);

            assert_eq!(DeleteService(husvc), TRUE);
            assert_eq!(CloseServiceHandle(husvc), TRUE);

            // Start dispatcher on background thread, verify running status, send STOP via ControlService.
            let dispatcher = std::thread::spawn(move || {
                let mut name_w = crate::strings::to_wide_z("PortFactoryTestSvc");
                let table = [
                    SERVICE_TABLE_ENTRYW {
                        lpServiceName: name_w.as_mut_ptr(),
                        lpServiceProc: test_service_main as *const () as usize,
                    },
                    SERVICE_TABLE_ENTRYW {
                        lpServiceName: std::ptr::null_mut(),
                        lpServiceProc: 0,
                    },
                ];
                assert_eq!(StartServiceCtrlDispatcherW(table.as_ptr()), TRUE);
            });

            // Wait until service reports SERVICE_RUNNING (4).
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
            let mut st = SERVICE_STATUS::default();
            while std::time::Instant::now() < deadline {
                assert_eq!(QueryServiceStatus(hsvc, &mut st), TRUE);
                if st.dwCurrentState == 4 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            assert_eq!(st.dwCurrentState, 4);

            // Unsupported PAUSE control is rejected because only ACCEPT_STOP is set.
            assert_eq!(ControlService(hsvc, SERVICE_CONTROL_PAUSE, &mut st), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_SERVICE_CONTROL);

            // Send STOP control -> handler runs -> status becomes SERVICE_STOPPED -> dispatcher returns!
            assert_eq!(ControlService(hsvc, SERVICE_CONTROL_STOP, &mut st), TRUE);
            assert!(HANDLER_RAN.load(Ordering::Acquire));
            assert_eq!(LAST_CTRL.load(Ordering::Acquire), SERVICE_CONTROL_STOP);
            assert_eq!(st.dwCurrentState, SERVICE_STOPPED);
            dispatcher.join().unwrap();

            assert_eq!(DeleteService(hsvc), TRUE);
            assert_eq!(CloseServiceHandle(hsvc), TRUE);
            assert_eq!(CloseServiceHandle(scm), TRUE);
            let _ = crate::kernel32::threads::CloseHandle(ev);
        }
    }

    #[test]
    fn capi_hashing_and_credential_store_roundtrip() {
        unsafe {
            // 1. CAPI SHA-256 of "abc"
            let mut hprov: usize = 0;
            assert_eq!(CryptAcquireContextW(&mut hprov, std::ptr::null(), std::ptr::null(), 24, 0), TRUE);
            assert_ne!(hprov, 0);

            let mut hhash: usize = 0;
            assert_eq!(CryptCreateHash(hprov, CALG_SHA_256, 0, 0, &mut hhash), TRUE);
            // Incremental multi-chunk updates (verifying streaming context + size-query stability).
            assert_eq!(CryptHashData(hhash, b"a".as_ptr(), 1, 0), TRUE);
            assert_eq!(CryptHashData(hhash, b"bc".as_ptr(), 2, 0), TRUE);

            let mut qlen = 0u32;
            assert_eq!(CryptGetHashParam(hhash, HP_HASHVAL, std::ptr::null_mut(), &mut qlen, 0), TRUE);
            assert_eq!(qlen, 32);

            let mut digest = [0u8; 32];
            let mut dlen = 32u32;
            assert_eq!(CryptGetHashParam(hhash, HP_HASHVAL, digest.as_mut_ptr(), &mut dlen, 0), TRUE);
            assert_eq!(dlen, 32);
            // NIST SHA-256("abc") = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
            assert_eq!(
                digest,
                [
                    0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea,
                    0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23,
                    0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c,
                    0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
                ]
            );
            assert_eq!(CryptDestroyHash(hhash), TRUE);
            assert_eq!(CryptReleaseContext(hprov, 0), TRUE);

            // 2. CredWriteW / CredReadW / CredFree / CredDeleteW across persistence modes
            use std::os::unix::fs::PermissionsExt;
            let tmp_root = std::env::temp_dir().join(format!("pf_cred_test_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&tmp_root);
            std::fs::create_dir_all(&tmp_root).unwrap();
            let prev_home = std::env::var_os("TRANSLATOR_HOME");
            let prev_drive = std::env::var_os("AW2_DRIVE_C");
            std::env::set_var("TRANSLATOR_HOME", &tmp_root);
            std::env::remove_var("AW2_DRIVE_C");

            let make_cred = |target: &[u16], user: &[u16], secret: &[u8], persist: u32| CredentialW {
                flags: 0,
                cred_type: 1, // CRED_TYPE_GENERIC
                target_name: target.as_ptr() as *mut u16,
                comment: std::ptr::null_mut(),
                last_written: FILETIME::default(),
                credential_blob_size: secret.len() as u32,
                _pad: 0,
                credential_blob: secret.as_ptr() as *mut u8,
                persist,
                attribute_count: 0,
                attributes: std::ptr::null_mut(),
                target_alias: std::ptr::null_mut(),
                user_name: user.as_ptr() as *mut u16,
            };

            let target_sess = crate::strings::to_wide_z("PortFactory/SessionOnly");
            let target_local = crate::strings::to_wide_z("PortFactory/LocalMachine");
            let target_ent = crate::strings::to_wide_z("PortFactory/Enterprise");
            let user = crate::strings::to_wide_z("alice");

            // Invalid persist is rejected honestly.
            let bad = make_cred(&target_sess, &user, b"x", 0);
            assert_eq!(CredWriteW(&bad as *const _ as *const c_void, 0), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_PARAMETER);

            let c_sess = make_cred(&target_sess, &user, b"sess-blob", CRED_PERSIST_SESSION);
            let c_local = make_cred(&target_local, &user, b"local-blob", CRED_PERSIST_LOCAL_MACHINE);
            let c_ent = make_cred(&target_ent, &user, b"ent-blob", CRED_PERSIST_ENTERPRISE);
            assert_eq!(CredWriteW(&c_sess as *const _ as *const c_void, 0), TRUE);
            assert_eq!(CredWriteW(&c_local as *const _ as *const c_void, 0), TRUE);
            assert_eq!(CredWriteW(&c_ent as *const _ as *const c_void, 0), TRUE);

            // CRED_PRESERVE_CREDENTIAL_BLOB: updates metadata while preserving persisted blob,
            // rejects non-zero input blob, unknown flags, and nonexistent targets.
            let user_updated = crate::strings::to_wide_z("alice-renamed");
            let bad_flags = make_cred(&target_local, &user_updated, b"", CRED_PERSIST_LOCAL_MACHINE);
            assert_eq!(CredWriteW(&bad_flags as *const _ as *const c_void, 0x2), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_FLAGS_CRED);

            let bad_preserve = make_cred(&target_local, &user_updated, b"nonempty", CRED_PERSIST_LOCAL_MACHINE);
            assert_eq!(
                CredWriteW(&bad_preserve as *const _ as *const c_void, CRED_PRESERVE_CREDENTIAL_BLOB),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_PARAMETER);

            let missing_target = crate::strings::to_wide_z("PortFactory/DoesNotExist");
            let missing_preserve = make_cred(&missing_target, &user_updated, b"", CRED_PERSIST_LOCAL_MACHINE);
            assert_eq!(
                CredWriteW(&missing_preserve as *const _ as *const c_void, CRED_PRESERVE_CREDENTIAL_BLOB),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_NOT_FOUND);

            let ok_preserve = make_cred(&target_local, &user_updated, b"", CRED_PERSIST_LOCAL_MACHINE);
            assert_eq!(
                CredWriteW(&ok_preserve as *const _ as *const c_void, CRED_PRESERVE_CREDENTIAL_BLOB),
                TRUE
            );

            // Vault files exist with 0600 permissions and do not store plaintext secret bytes.
            for (kind, needle) in [
                (CredVaultKind::LocalMachine, &b"local-blob"[..]),
                (CredVaultKind::Enterprise, &b"ent-blob"[..]),
            ] {
                let p = kind.path();
                let meta = std::fs::metadata(&p).expect("persisted vault must exist");
                assert_eq!(meta.permissions().mode() & 0o777, 0o600);
                let raw = std::fs::read(&p).unwrap();
                assert!(!raw.windows(needle.len()).any(|w| w == needle), "vault must be encrypted");
            }

            // Simulate a new process by clearing the in-memory session store.
            session_cred_store().lock().unwrap().clear();

            // Session credential is gone; persisted LOCAL_MACHINE and ENTERPRISE survive.
            let mut out: *mut c_void = std::ptr::null_mut();
            assert_eq!(CredReadW(target_sess.as_ptr(), 1, 0, &mut out), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_NOT_FOUND);

            for (t, expected, expected_user) in [
                (&target_local, &b"local-blob"[..], "alice-renamed"),
                (&target_ent, &b"ent-blob"[..], "alice"),
            ] {
                assert_eq!(CredReadW(t.as_ptr(), 1, 0, &mut out), TRUE);
                assert!(!out.is_null());
                let got = std::ptr::read_unaligned(out as *const CredentialW);
                assert_eq!(got.credential_blob_size as usize, expected.len());
                assert_eq!(std::slice::from_raw_parts(got.credential_blob, expected.len()), expected);
                assert_eq!(crate::strings::from_wide(got.user_name), expected_user);
                CredFree(out);

                assert_eq!(CredDeleteW(t.as_ptr(), 1, 0), TRUE);
                assert_eq!(CredReadW(t.as_ptr(), 1, 0, &mut out), FALSE);
                assert_eq!(crate::teb::last_error(), ERROR_NOT_FOUND);
            }

            match prev_home {
                Some(v) => std::env::set_var("TRANSLATOR_HOME", v),
                None => std::env::remove_var("TRANSLATOR_HOME"),
            }
            match prev_drive {
                Some(v) => std::env::set_var("AW2_DRIVE_C", v),
                None => std::env::remove_var("AW2_DRIVE_C"),
            }
            let _ = std::fs::remove_dir_all(&tmp_root);
        }
    }
}
