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
        *out = std::ptr::null_mut();
    }
    if !disp.is_null() {
        *disp = 0;
    }
    let Some(base) = resolve_key_path(key) else {
        return ERROR_INVALID_HANDLE;
    };
    let sub_str = crate::strings::from_wide_opt(sub).unwrap_or_default();
    let mut cur = base;
    let mut created_any = false;
    let mut reg = registry().lock().unwrap();
    let mut existed = reg.keys.contains_key(&cur);
    for seg in sub_str.split(['\\', '/']).filter(|s| !s.is_empty()) {
        cur.push('\\');
        cur.push_str(&seg.to_ascii_lowercase());
        existed = reg.keys.contains_key(&cur);
        if !existed {
            created_any = true;
        }
        reg.keys.entry(cur.clone()).or_default();
    }
    let fired = if created_any {
        if let Some(parent) = parent_reg_path(&cur) {
            reg.take_triggered(&[&cur, parent], REG_NOTIFY_CHANGE_NAME)
        } else {
            reg.take_triggered(&[&cur], REG_NOTIFY_CHANGE_NAME)
        }
    } else {
        Vec::new()
    };
    drop(reg);
    signal_reg_events(fired);
    if !disp.is_null() {
        *disp = if existed { REG_OPENED_EXISTING_KEY } else { REG_CREATED_NEW_KEY };
    }
    if !out.is_null() {
        *out = handles::insert(Arc::new(RegKey { path: cur.clone() }));
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
    let wide = crate::strings::to_wide_z(&dir);
    let needed = wide.len() as u32;
    let cap = std::ptr::read_unaligned(lpcch_size);
    if lp_profile_dir.is_null() || cap < needed {
        std::ptr::write_unaligned(lpcch_size, needed);
        return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    std::ptr::copy_nonoverlapping(wide.as_ptr(), lp_profile_dir, wide.len());
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
        let wide = crate::strings::to_wide_z(&desc);
        let need = (8 + wide.len() * 2) as u32;
        std::ptr::write_unaligned(pcb_bytes_needed, need);
        if lp_buffer.is_null() || cb_buf_size < need {
            return err(ERROR_INSUFFICIENT_BUFFER, FALSE);
        }
        let str_dst = lp_buffer.add(8) as *mut u16;
        std::ptr::copy_nonoverlapping(wide.as_ptr(), str_dst, wide.len());
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
    let bin_w = crate::strings::to_wide_z(&bin);
    let disp_w = crate::strings::to_wide_z(&disp);
    // QUERY_SERVICE_CONFIGW on x64 is 64 bytes:
    // 0: dwServiceType, 4: dwStartType, 8: dwErrorControl, 12: pad,
    // 16: lpBinaryPathName, 24: lpLoadOrderGroup, 32: dwTagId, 36: pad,
    // 40: lpDependencies, 48: lpServiceStartName, 56: lpDisplayName
    let header_sz = 64usize;
    let need = (header_sz + (bin_w.len() + disp_w.len()) * 2) as u32;
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
    std::ptr::copy_nonoverlapping(bin_w.as_ptr(), bin_dst, bin_w.len());
    let disp_dst = bin_dst.add(bin_w.len());
    std::ptr::copy_nonoverlapping(disp_w.as_ptr(), disp_dst, disp_w.len());
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

// @exports CryptAcquireContextW CryptAcquireContextA CryptReleaseContext CryptCreateHash CryptHashData CryptGetHashParam CryptSetHashParam CryptDestroyHash CryptDecrypt CryptEncrypt CryptDestroyKey CryptExportKey CryptImportKey CryptGetUserKey CryptGetProvParam CryptSignHashW CredWriteW
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    CryptAcquireContextW, CryptAcquireContextA, CryptReleaseContext, CryptCreateHash, CryptHashData,
    CryptGetHashParam, CryptSetHashParam, CryptDestroyHash, CryptDecrypt,
    CryptEncrypt, CryptDestroyKey, CryptExportKey, CryptImportKey,
    CryptGetUserKey, CryptGetProvParam, CryptSignHashW, CredWriteW);

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

/// No CSPs are installed.
pub unsafe extern "win64" fn CryptEnumProvidersW() -> BOOL {
    err(ERROR_NO_MORE_ITEMS, FALSE)
}

/// The credential store is empty.
pub unsafe extern "win64" fn CredReadW(target: LPCWSTR, _ty: u32, _flags: u32, out: *mut *mut c_void) -> BOOL {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    log::debug!("CredReadW({:?}) -> ERROR_NOT_FOUND (empty credential store)", crate::strings::from_wide_opt(target));
    err(ERROR_NOT_FOUND, FALSE)
}

pub unsafe extern "win64" fn CredDeleteW(_target: LPCWSTR, _ty: u32, _flags: u32) -> BOOL {
    err(ERROR_NOT_FOUND, FALSE)
}

/// CredReadW never allocates, so there is nothing to free.
pub unsafe extern "win64" fn CredFree(p: *mut c_void) {
    if !p.is_null() {
        stub!("CredFree({p:p}): buffer not allocated by us");
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

    #[test]
    fn user_profile_directory_uses_vfs() {
        unsafe {
            let mut tok = std::ptr::null_mut();
            assert_eq!(OpenProcessToken(handles::CURRENT_PROCESS, 0x8, &mut tok), TRUE);
            let mut sz = 0u32;
            assert_eq!(GetUserProfileDirectoryW(tok, std::ptr::null_mut(), &mut sz), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert!(sz > 4);
            let mut buf = vec![0u16; sz as usize];
            assert_eq!(GetUserProfileDirectoryW(tok, buf.as_mut_ptr(), &mut sz), TRUE);
            let dir = crate::strings::from_wide(buf.as_ptr());
            assert!(dir.to_ascii_lowercase().starts_with("c:\\users\\"));

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

            // ChangeServiceConfig2W + QueryServiceConfig2W description roundtrip.
            let desc = crate::strings::to_wide_z("Test Service Description");
            let desc_ptr = desc.as_ptr();
            assert_eq!(ChangeServiceConfig2W(hsvc, SERVICE_CONFIG_DESCRIPTION, &desc_ptr as *const _ as *const c_void), TRUE);
            let mut need = 0u32;
            assert_eq!(QueryServiceConfig2W(hsvc, SERVICE_CONFIG_DESCRIPTION, std::ptr::null_mut(), 0, &mut need), FALSE);
            let mut dbuf = vec![0u8; need as usize];
            assert_eq!(QueryServiceConfig2W(hsvc, SERVICE_CONFIG_DESCRIPTION, dbuf.as_mut_ptr(), need, &mut need), TRUE);
            let out_ptr = std::ptr::read_unaligned(dbuf.as_ptr() as *const LPCWSTR);
            assert_eq!(crate::strings::from_wide(out_ptr), "Test Service Description");

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
}
