//! advapi32.dll
//!
//! * ETW (`Event*`): providers register fine; no trace session can ever be
//!   enabled, so writes are dropped exactly as on Windows with no listener.
//! * Event log (`RegisterEventSourceW`/`ReportEventW`): forwarded to `log`.
//! * Registry: empty, read-only for now. Opens fail with
//!   ERROR_FILE_NOT_FOUND (logged at debug so we can see what is queried);
//!   creating/writing keys is a STUB failure.
//! * Tokens/privileges: a process token exists with full `GetTokenInformation`
//!   coverage; no privilege can be enabled (AdjustTokenPrivileges reports
//!   ERROR_NOT_ALL_ASSIGNED, like a standard user without SeLockMemoryPrivilege).
//! * `GetUserNameW`: the host user.
//! * CAPI (`Crypt*`): provider containers, hashing (MD2/MD4/MD5/SHA-1/256/384/512),
//!   symmetric & RSA keys, blobs (`PUBLICKEYBLOB`, `PRIVATEKEYBLOB`, `SIMPLEBLOB`,
//!   `PLAINTEXTKEYBLOB`), encrypt/decrypt, sign/verify, and key derivation backed by
//!   `misc::bcrypt` (CommonCrypto + Security.framework).
//! * Credential manager (`Cred*`): DPAPI-encrypted local/enterprise vault + session store.

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
    /// Original-case leaf key name keyed by lowercased full registry path.
    key_names: std::collections::BTreeMap<String, String>,
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
        let mut reg = Self { keys, key_names: std::collections::BTreeMap::new(), watchers: Vec::new() };
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
            self.key_names.entry(cur.clone()).or_insert_with(|| seg.to_string());
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
            reg.key_names.insert(cur.clone(), seg.to_string());
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
        reg.key_names.remove(&full);
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
    reg.key_names.retain(|k, _| !k.starts_with(&prefix));
    if !sub_str.is_empty() {
        reg.keys.remove(&full);
        reg.key_names.remove(&full);
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
                let display = reg.key_names.get(k).cloned().unwrap_or_else(|| rest.to_string());
                subkeys.push(display);
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

const TOKEN_PRIMARY: u32 = 1;
const TOKEN_IMPERSONATION: u32 = 2;
const SECURITY_IMPERSONATION: u32 = 2;
const ERROR_NO_SUCH_LOGON_SESSION: u32 = 1312;
const ERROR_PRIVILEGE_NOT_HELD: u32 = 1314;

const SE_GROUP_MANDATORY: u32 = 0x0000_0001;
const SE_GROUP_ENABLED_BY_DEFAULT: u32 = 0x0000_0002;
const SE_GROUP_ENABLED: u32 = 0x0000_0004;
const SE_GROUP_INTEGRITY: u32 = 0x0000_0020;
const SE_GROUP_INTEGRITY_ENABLED: u32 = 0x0000_0040;
const SE_GROUP_LOGON_ID: u32 = 0xC000_0000;
const SE_PRIVILEGE_ENABLED_BY_DEFAULT: u32 = 0x0000_0001;
const SE_PRIVILEGE_ENABLED: u32 = 0x0000_0002;
const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: u32 = 1;

struct TokenObj {
    token_type: u32,
    imp_level: u32,
    access: u32,
    token_id: u64,
    auth_id: u64,
    modified_id: u64,
}

impl handles::KObject for TokenObj {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn type_name(&self) -> &'static str {
        "Token"
    }
}

impl TokenObj {
    fn access_mask(request: u32) -> u32 {
        const TOKEN_ALL_ACCESS: u32 = 0x000f_01ff;
        if request & (0x0200_0000 | 0x1000_0000) != 0 {
            return TOKEN_ALL_ACCESS;
        }
        let mut mask = request & 0x0fff_ffff;
        for (generic, rights) in [(0x8000_0000, 0x2_0008), (0x4000_0000, 0x2_00e0), (0x2000_0000, 0x2_0000)] {
            if request & generic != 0 {
                mask |= rights;
            }
        }
        mask
    }

    fn resolve(h: HANDLE, required_access: u32) -> Result<Self, u32> {
        let raw = h as isize;
        // Windows 8+ pseudo-token handles: GetCurrentProcessToken (-4),
        // GetCurrentThreadToken (-5), GetCurrentThreadEffectiveToken (-6).
        // Per Windows docs, -4 and -6 grant TOKEN_QUERY | TOKEN_QUERY_SOURCE (0x0018),
        // while -5 fails with ERROR_NO_TOKEN (1008) when the thread is not impersonating.
        let tok = if raw == -4 || raw == -6 {
            Self {
                token_type: TOKEN_PRIMARY,
                imp_level: SECURITY_IMPERSONATION,
                access: 0x0008 | 0x0010, // TOKEN_QUERY | TOKEN_QUERY_SOURCE
                token_id: 0x100,
                auth_id: 0x3E7,
                modified_id: 0x101,
            }
        } else if raw == -5 {
            return Err(1008); // ERROR_NO_TOKEN
        } else if let Some(tok) = handles::get_as::<Self>(h) {
            Self {
                token_type: tok.token_type,
                imp_level: tok.imp_level,
                access: tok.access,
                token_id: tok.token_id,
                auth_id: tok.auth_id,
                modified_id: tok.modified_id,
            }
        } else if handles::get(h).is_some_and(|o| o.type_name() == "Token") {
            Self {
                token_type: TOKEN_PRIMARY,
                imp_level: SECURITY_IMPERSONATION,
                access: 0x000f_01ff,
                token_id: 0x100,
                auth_id: 0x3E7,
                modified_id: 0x101,
            }
        } else {
            return Err(ERROR_INVALID_HANDLE);
        };
        if (tok.access & required_access) != required_access {
            return Err(ERROR_ACCESS_DENIED);
        }
        Ok(tok)
    }
}

fn new_token_obj(token_type: u32, imp_level: u32, access: u32) -> Arc<TokenObj> {
    let tid = NEXT_LUID.fetch_add(1, Ordering::Relaxed);
    let mid = NEXT_LUID.fetch_add(1, Ordering::Relaxed);
    Arc::new(TokenObj {
        token_type,
        imp_level,
        access,
        token_id: tid,
        auth_id: 0x0000_03E7, // SYSTEM_LUID / logon session LUID
        modified_id: mid,
    })
}

fn build_sid(authority: u8, subs: &[u32]) -> Vec<u8> {
    let mut v = Vec::with_capacity(8 + 4 * subs.len());
    v.push(1); // SID_REVISION
    v.push(subs.len() as u8);
    v.extend_from_slice(&[0, 0, 0, 0, 0, authority]);
    for &s in subs {
        v.extend_from_slice(&s.to_le_bytes());
    }
    v
}

fn token_is_elevated() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn token_user_sid() -> Vec<u8> {
    // S-1-5-1000 (consistent with existing TokenUser SID layout)
    build_sid(5, &[1000])
}

fn token_primary_group_sid() -> Vec<u8> {
    // S-1-5-32-545 (BUILTIN\Users)
    build_sid(5, &[32, 545])
}

fn token_integrity_sid() -> Vec<u8> {
    // S-1-16-8192 (Medium) or S-1-16-12288 (High when root)
    let rid = if token_is_elevated() { 0x3000 } else { 0x2000 };
    build_sid(16, &[rid])
}

fn token_default_groups() -> Vec<(Vec<u8>, u32)> {
    let std_attrs = SE_GROUP_MANDATORY | SE_GROUP_ENABLED_BY_DEFAULT | SE_GROUP_ENABLED;
    vec![
        (build_sid(5, &[32, 545]), std_attrs), // BUILTIN\Users
        (build_sid(1, &[0]), std_attrs),       // Everyone (World)
        (build_sid(5, &[4]), std_attrs),       // Interactive
        (build_sid(5, &[11]), std_attrs),      // Authenticated Users
        (build_sid(5, &[5, 0, 1]), std_attrs | SE_GROUP_LOGON_ID), // Logon SID
        (token_integrity_sid(), SE_GROUP_INTEGRITY | SE_GROUP_INTEGRITY_ENABLED),
    ]
}

fn token_default_privileges() -> Vec<(u64, u32)> {
    // Standard Windows user token always holds SeChangeNotifyPrivilege (23) enabled by default.
    vec![(23u64, SE_PRIVILEGE_ENABLED_BY_DEFAULT | SE_PRIVILEGE_ENABLED)]
}

fn sid_hash_byte(sid: &[u8]) -> u8 {
    if sid.len() < 8 {
        return 0;
    }
    let sub_count = sid[1] as usize;
    if sub_count > 0 && sid.len() >= 8 + sub_count * 4 {
        // Low byte of the final SubAuthority (little-endian u32), matching Windows RtlSidHashInitialize.
        sid[8 + (sub_count - 1) * 4]
    } else {
        sid[7]
    }
}

fn populate_sid_hash(hash_out: &mut [u64; 32], sids: &[(Vec<u8>, u32)]) {
    for (i, (sid, _)) in sids.iter().take(64).enumerate() {
        let b = sid_hash_byte(sid);
        let bit = 1u64 << i;
        hash_out[(b & 0x0F) as usize] |= bit;
        hash_out[16 + ((b >> 4) as usize)] |= bit;
    }
}

unsafe fn write_token_groups(groups: &[(Vec<u8>, u32)], buf: *mut u8, buf_len: u32, ret_len: *mut u32) -> BOOL {
    // TOKEN_GROUPS on x64: DWORD GroupCount (offset 0), 4-byte pad, SID_AND_ATTRIBUTES[N] (offset 8, 16 bytes each)
    let header_sz = 8usize;
    let entries_sz = groups.len() * 16;
    let sids_sz: usize = groups.iter().map(|(s, _)| s.len()).sum();
    let need = (header_sz + entries_sz + sids_sz) as u32;
    std::ptr::write_unaligned(ret_len, need);
    if buf.is_null() || buf_len < need {
        return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    std::ptr::write_bytes(buf, 0, need as usize);
    std::ptr::write_unaligned(buf as *mut u32, groups.len() as u32);
    let mut sid_off = header_sz + entries_sz;
    for (i, (sid, attrs)) in groups.iter().enumerate() {
        let dst = buf.add(sid_off);
        std::ptr::copy_nonoverlapping(sid.as_ptr(), dst, sid.len());
        let ent = buf.add(header_sz + i * 16);
        std::ptr::write_unaligned(ent as *mut usize, dst as usize);
        std::ptr::write_unaligned(ent.add(8) as *mut u32, *attrs);
        sid_off += sid.len();
    }
    TRUE
}

unsafe fn write_u32_token_info(val: u32, buf: *mut u8, buf_len: u32, ret_len: *mut u32) -> BOOL {
    std::ptr::write_unaligned(ret_len, 4);
    if buf.is_null() || buf_len < 4 {
        return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
    }
    std::ptr::write_unaligned(buf as *mut u32, val);
    TRUE
}

pub unsafe extern "win64" fn AllocateLocallyUniqueId(luid: *mut u64) -> BOOL {
    if luid.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    *luid = NEXT_LUID.fetch_add(1, Ordering::Relaxed);
    TRUE
}

pub unsafe extern "win64" fn PrivilegeCheck(token: HANDLE, _privs: *mut c_void, result: *mut BOOL) -> BOOL {
    if let Err(e) = TokenObj::resolve(token, 0x0008) {
        return err(e, FALSE);
    }
    if result.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    *result = FALSE;
    TRUE
}

pub unsafe extern "win64" fn OpenProcessToken(process: HANDLE, access: u32, out: *mut HANDLE) -> BOOL {
    if out.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if !crate::kernel32::process::is_self(process) {
        return err(ERROR_INVALID_HANDLE, FALSE);
    }
    let access = TokenObj::access_mask(access);
    std::ptr::write_unaligned(out, handles::insert(new_token_obj(TOKEN_PRIMARY, SECURITY_IMPERSONATION, access)));
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
    if ret_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    // TokenSource (7) requires TOKEN_QUERY_SOURCE (0x0010); all other classes require TOKEN_QUERY (0x0008).
    let required_access = if info_class == 7 { 0x0010 } else { 0x0008 };
    let tok = match TokenObj::resolve(token, required_access) {
        Ok(t) => t,
        Err(e) => {
            std::ptr::write_unaligned(ret_len, 0);
            return err(e, FALSE);
        }
    };
    match info_class {
        // TokenUser (1): TOKEN_USER { SID_AND_ATTRIBUTES { PSID @0, DWORD Attrs @8 } } + user SID
        1 => {
            let sid = token_user_sid();
            let need = (16 + sid.len()) as u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            let dst = buf.add(16);
            std::ptr::copy_nonoverlapping(sid.as_ptr(), dst, sid.len());
            std::ptr::write_unaligned(buf as *mut usize, dst as usize);
            TRUE
        }
        // TokenGroups (2): TOKEN_GROUPS
        2 => {
            let groups = token_default_groups();
            write_token_groups(&groups, buf, buf_len, ret_len)
        }
        // TokenPrivileges (3): TOKEN_PRIVILEGES { DWORD PrivilegeCount; LUID_AND_ATTRIBUTES[N] (12 bytes each) }
        3 => {
            let privs = token_default_privileges();
            let need = (4 + privs.len() * 12) as u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            std::ptr::write_unaligned(buf as *mut u32, privs.len() as u32);
            for (i, &(luid, attrs)) in privs.iter().enumerate() {
                let ent = buf.add(4 + i * 12);
                std::ptr::write_unaligned(ent as *mut u64, luid);
                std::ptr::write_unaligned(ent.add(8) as *mut u32, attrs);
            }
            TRUE
        }
        // TokenOwner (4): TOKEN_OWNER { PSID Owner } + user SID
        4 => {
            let sid = token_user_sid();
            let need = (8 + sid.len()) as u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            let dst = buf.add(8);
            std::ptr::copy_nonoverlapping(sid.as_ptr(), dst, sid.len());
            std::ptr::write_unaligned(buf as *mut usize, dst as usize);
            TRUE
        }
        // TokenPrimaryGroup (5): TOKEN_PRIMARY_GROUP { PSID PrimaryGroup } + primary group SID
        5 => {
            let sid = token_primary_group_sid();
            let need = (8 + sid.len()) as u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            let dst = buf.add(8);
            std::ptr::copy_nonoverlapping(sid.as_ptr(), dst, sid.len());
            std::ptr::write_unaligned(buf as *mut usize, dst as usize);
            TRUE
        }
        // TokenDefaultDacl (6): TOKEN_DEFAULT_DACL { PACL DefaultDacl } + 8-byte ACL header
        6 => {
            let need = 16u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            let acl = buf.add(8);
            *acl = 2; // ACL_REVISION
            std::ptr::write_unaligned(acl.add(2) as *mut u16, 8); // AclSize
            std::ptr::write_unaligned(buf as *mut usize, acl as usize);
            TRUE
        }
        // TokenSource (7): TOKEN_SOURCE { CHAR SourceName[8]; LUID SourceIdentifier } (16 bytes)
        7 => {
            let need = 16u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, 16);
            std::ptr::copy_nonoverlapping(b"User32\0\0".as_ptr(), buf, 8);
            std::ptr::write_unaligned(buf.add(8) as *mut u64, tok.auth_id);
            TRUE
        }
        // TokenType (8): TOKEN_TYPE (TokenPrimary=1 or TokenImpersonation=2)
        8 => write_u32_token_info(tok.token_type, buf, buf_len, ret_len),
        // TokenImpersonationLevel (9): valid only for impersonation tokens; primary tokens return ERROR_INVALID_PARAMETER on Windows.
        9 => {
            if tok.token_type != TOKEN_IMPERSONATION {
                std::ptr::write_unaligned(ret_len, 0);
                return err(ERROR_INVALID_PARAMETER, FALSE);
            }
            write_u32_token_info(tok.imp_level, buf, buf_len, ret_len)
        }
        // TokenStatistics (10): TOKEN_STATISTICS (56 bytes on x64)
        10 => {
            let need = 56u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, 56);
            std::ptr::write_unaligned(buf.add(0) as *mut u64, tok.token_id);
            std::ptr::write_unaligned(buf.add(8) as *mut u64, tok.auth_id);
            std::ptr::write_unaligned(buf.add(16) as *mut i64, i64::MAX); // ExpirationTime (no expiry)
            std::ptr::write_unaligned(buf.add(24) as *mut u32, tok.token_type);
            std::ptr::write_unaligned(buf.add(28) as *mut u32, tok.imp_level);
            std::ptr::write_unaligned(buf.add(32) as *mut u32, 4096); // DynamicCharged
            std::ptr::write_unaligned(buf.add(36) as *mut u32, 4096); // DynamicAvailable
            std::ptr::write_unaligned(buf.add(40) as *mut u32, token_default_groups().len() as u32);
            std::ptr::write_unaligned(buf.add(44) as *mut u32, token_default_privileges().len() as u32);
            std::ptr::write_unaligned(buf.add(48) as *mut u64, tok.modified_id);
            TRUE
        }
        // TokenRestrictedSids (11): empty TOKEN_GROUPS
        11 => write_token_groups(&[], buf, buf_len, ret_len),
        // TokenSessionId (12) -> 1
        12 => write_u32_token_info(1, buf, buf_len, ret_len),
        // TokenGroupsAndPrivileges (13): TOKEN_GROUPS_AND_PRIVILEGES (56 bytes header on x64)
        13 => {
            let mut sids = vec![(token_user_sid(), 0u32)];
            sids.extend(token_default_groups());
            let privs = token_default_privileges();
            let header_sz = 56usize;
            let sids_arr_sz = sids.len() * 16;
            let sids_data_sz: usize = sids.iter().map(|(s, _)| s.len()).sum();
            let privs_sz = privs.len() * 12;
            let need = (header_sz + sids_arr_sz + sids_data_sz + privs_sz) as u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            let sids_arr = buf.add(header_sz);
            let mut sid_data_off = header_sz + sids_arr_sz;
            for (i, (sid, attrs)) in sids.iter().enumerate() {
                let dst = buf.add(sid_data_off);
                std::ptr::copy_nonoverlapping(sid.as_ptr(), dst, sid.len());
                let ent = sids_arr.add(i * 16);
                std::ptr::write_unaligned(ent as *mut usize, dst as usize);
                std::ptr::write_unaligned(ent.add(8) as *mut u32, *attrs);
                sid_data_off += sid.len();
            }
            let privs_ptr = buf.add(sid_data_off);
            for (i, &(luid, attrs)) in privs.iter().enumerate() {
                let ent = privs_ptr.add(i * 12);
                std::ptr::write_unaligned(ent as *mut u64, luid);
                std::ptr::write_unaligned(ent.add(8) as *mut u32, attrs);
            }
            std::ptr::write_unaligned(buf.add(0) as *mut u32, sids.len() as u32);
            std::ptr::write_unaligned(buf.add(4) as *mut u32, (sids_arr_sz + sids_data_sz) as u32);
            std::ptr::write_unaligned(buf.add(8) as *mut usize, sids_arr as usize);
            // RestrictedSidCount/Length/Ptr @16, 20, 24 = 0
            std::ptr::write_unaligned(buf.add(32) as *mut u32, privs.len() as u32);
            std::ptr::write_unaligned(buf.add(36) as *mut u32, privs_sz as u32);
            std::ptr::write_unaligned(buf.add(40) as *mut usize, privs_ptr as usize);
            std::ptr::write_unaligned(buf.add(48) as *mut u64, tok.auth_id);
            TRUE
        }
        // TokenSessionReference (14): reserved; Windows returns ERROR_INVALID_PARAMETER.
        14 => {
            std::ptr::write_unaligned(ret_len, 0);
            err(ERROR_INVALID_PARAMETER, FALSE)
        }
        // TokenSandBoxInert (15) -> 0
        15 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenAuditPolicy (16): requires SeSecurityPrivilege; standard user gets ERROR_PRIVILEGE_NOT_HELD.
        16 => {
            std::ptr::write_unaligned(ret_len, 0);
            err(ERROR_PRIVILEGE_NOT_HELD, FALSE)
        }
        // TokenOrigin (17): TOKEN_ORIGIN { LUID OriginatingLogonSession } (8 bytes)
        17 => {
            std::ptr::write_unaligned(ret_len, 8);
            if buf.is_null() || buf_len < 8 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_unaligned(buf as *mut u64, tok.auth_id);
            TRUE
        }
        // TokenElevationType (18): TokenElevationTypeDefault (1)
        18 => write_u32_token_info(1, buf, buf_len, ret_len),
        // TokenLinkedToken (19): non-split (default) token has no linked UAC token -> ERROR_NO_SUCH_LOGON_SESSION (1312)
        19 => {
            std::ptr::write_unaligned(ret_len, 0);
            err(ERROR_NO_SUCH_LOGON_SESSION, FALSE)
        }
        // TokenElevation (20): 1 if root, 0 otherwise
        20 => write_u32_token_info(token_is_elevated() as u32, buf, buf_len, ret_len),
        // TokenHasRestrictions (21) -> 0
        21 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenAccessInformation (22): TOKEN_ACCESS_INFORMATION (88 bytes on x64) + SidHash/RestrictedSidHash/CapabilitiesHash + SidAttr/SIDs + Privileges
        22 => {
            // SID_AND_ATTRIBUTES_HASH on x64 is 4 (SidCount) + 4 (pad) + 8 (SidAttr) + 32*8 (Hash) = 272 bytes.
            const HASH_SZ: usize = 272;
            let header_sz = 88usize;
            let mut sids = vec![(token_user_sid(), 0u32)];
            sids.extend(token_default_groups());
            let privs = token_default_privileges();
            let hashes_end = header_sz + HASH_SZ * 3;
            let sids_arr_sz = sids.len() * 16;
            let sids_data_sz: usize = sids.iter().map(|(s, _)| s.len()).sum();
            let privs_off = (hashes_end + sids_arr_sz + sids_data_sz + 7) & !7;
            let privs_sz = 4 + privs.len() * 12;
            let need = (privs_off + privs_sz) as u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            let sid_hash = buf.add(header_sz);
            let restr_hash = sid_hash.add(HASH_SZ);
            let cap_hash = restr_hash.add(HASH_SZ);
            let sids_arr = buf.add(hashes_end);
            let mut sid_data_off = hashes_end + sids_arr_sz;
            for (i, (sid, attrs)) in sids.iter().enumerate() {
                let dst = buf.add(sid_data_off);
                std::ptr::copy_nonoverlapping(sid.as_ptr(), dst, sid.len());
                let ent = sids_arr.add(i * 16);
                std::ptr::write_unaligned(ent as *mut usize, dst as usize);
                std::ptr::write_unaligned(ent.add(8) as *mut u32, *attrs);
                sid_data_off += sid.len();
            }
            let mut hash_entries = [0u64; 32];
            populate_sid_hash(&mut hash_entries, &sids);
            std::ptr::write_unaligned(sid_hash as *mut u32, sids.len() as u32);
            std::ptr::write_unaligned(sid_hash.add(8) as *mut usize, sids_arr as usize);
            for (i, &entry) in hash_entries.iter().enumerate() {
                std::ptr::write_unaligned(sid_hash.add(16 + i * 8) as *mut u64, entry);
            }
            let privs_ptr = buf.add(privs_off);
            std::ptr::write_unaligned(privs_ptr as *mut u32, privs.len() as u32);
            for (i, &(luid, attrs)) in privs.iter().enumerate() {
                let ent = privs_ptr.add(4 + i * 12);
                std::ptr::write_unaligned(ent as *mut u64, luid);
                std::ptr::write_unaligned(ent.add(8) as *mut u32, attrs);
            }
            std::ptr::write_unaligned(buf.add(0) as *mut usize, sid_hash as usize);
            std::ptr::write_unaligned(buf.add(8) as *mut usize, restr_hash as usize);
            std::ptr::write_unaligned(buf.add(16) as *mut usize, privs_ptr as usize);
            std::ptr::write_unaligned(buf.add(24) as *mut u64, tok.auth_id);
            std::ptr::write_unaligned(buf.add(32) as *mut u32, tok.token_type);
            std::ptr::write_unaligned(buf.add(36) as *mut u32, tok.imp_level);
            std::ptr::write_unaligned(buf.add(40) as *mut u32, TOKEN_MANDATORY_POLICY_NO_WRITE_UP);
            // Flags @44 = 0, AppContainerNumber @48 = 0, PackageSid @56 = NULL
            std::ptr::write_unaligned(buf.add(64) as *mut usize, cap_hash as usize);
            // TrustLevelSid @72 = NULL, SecurityAttributes @80 = NULL
            TRUE
        }
        // TokenVirtualizationAllowed (23) -> 0
        23 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenVirtualizationEnabled (24) -> 0
        24 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenIntegrityLevel (25): TOKEN_MANDATORY_LABEL -> Medium (0x2000) or High (0x3000)
        25 => {
            let sid = token_integrity_sid();
            let need = (16 + sid.len()) as u32;
            std::ptr::write_unaligned(ret_len, need);
            if buf.is_null() || buf_len < need {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, need as usize);
            let dst = buf.add(16);
            std::ptr::copy_nonoverlapping(sid.as_ptr(), dst, sid.len());
            std::ptr::write_unaligned(buf as *mut usize, dst as usize);
            std::ptr::write_unaligned(buf.add(8) as *mut u32, SE_GROUP_INTEGRITY);
            TRUE
        }
        // TokenUIAccess (26) -> 0
        26 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenMandatoryPolicy (27) -> TOKEN_MANDATORY_POLICY_NO_WRITE_UP (1)
        27 => write_u32_token_info(TOKEN_MANDATORY_POLICY_NO_WRITE_UP, buf, buf_len, ret_len),
        // TokenLogonSid (28): TOKEN_GROUPS with single logon SID S-1-5-5-0-1
        28 => {
            let logon = [(
                build_sid(5, &[5, 0, 1]),
                SE_GROUP_MANDATORY | SE_GROUP_ENABLED_BY_DEFAULT | SE_GROUP_ENABLED | SE_GROUP_LOGON_ID,
            )];
            write_token_groups(&logon, buf, buf_len, ret_len)
        }
        // TokenIsAppContainer (29) -> 0
        29 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenCapabilities (30): empty TOKEN_GROUPS
        30 => write_token_groups(&[], buf, buf_len, ret_len),
        // TokenAppContainerSid (31): TOKEN_APPCONTAINER_INFORMATION { PSID TokenAppContainer = NULL } (8 bytes)
        31 => {
            std::ptr::write_unaligned(ret_len, 8);
            if buf.is_null() || buf_len < 8 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_unaligned(buf as *mut usize, 0);
            TRUE
        }
        // TokenAppContainerNumber (32) -> 0
        32 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenUserClaimAttributes (33), TokenDeviceClaimAttributes (34),
        // TokenRestrictedUserClaimAttributes (35), TokenRestrictedDeviceClaimAttributes (36),
        // TokenSecurityAttributes (39), TokenSingletonAttributes (43):
        // CLAIM_SECURITY_ATTRIBUTES_INFORMATION / TOKEN_SECURITY_ATTRIBUTES_INFORMATION (16 bytes on x64):
        // { WORD Version=1, WORD Reserved=0, DWORD AttributeCount=0, PVOID pAttributeV1=NULL }
        33 | 34 | 35 | 36 | 39 | 43 => {
            std::ptr::write_unaligned(ret_len, 16);
            if buf.is_null() || buf_len < 16 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, 16);
            std::ptr::write_unaligned(buf as *mut u16, 1); // Version = 1
            TRUE
        }
        // TokenDeviceGroups (37), TokenRestrictedDeviceGroups (38): empty TOKEN_GROUPS
        37 | 38 => write_token_groups(&[], buf, buf_len, ret_len),
        // TokenIsRestricted (40) -> 0
        40 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenProcessTrustLevel (41): PSID = NULL (8 bytes)
        41 => {
            std::ptr::write_unaligned(ret_len, 8);
            if buf.is_null() || buf_len < 8 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_unaligned(buf as *mut usize, 0);
            TRUE
        }
        // TokenPrivateNameSpace (42) -> 0
        42 => write_u32_token_info(0, buf, buf_len, ret_len),
        // TokenBnoIsolation (44): TOKEN_BNO_ISOLATION_INFORMATION { PWSTR IsolationPrefix=NULL, BOOLEAN IsolationEnabled=0 } (16 bytes)
        44 => {
            std::ptr::write_unaligned(ret_len, 16);
            if buf.is_null() || buf_len < 16 {
                return err(crate::types::err::ERROR_INSUFFICIENT_BUFFER, FALSE);
            }
            std::ptr::write_bytes(buf, 0, 16);
            TRUE
        }
        // TokenChildProcessFlags (45), TokenIsLessPrivilegedAppContainer (46),
        // TokenIsSandboxed (47), TokenIsAppSilo (48) -> 0
        45 | 46 | 47 | 48 => write_u32_token_info(0, buf, buf_len, ret_len),
        // Invalid / out-of-range TOKEN_INFORMATION_CLASS: Windows returns FALSE with ERROR_INVALID_PARAMETER.
        _ => {
            std::ptr::write_unaligned(ret_len, 0);
            err(ERROR_INVALID_PARAMETER, FALSE)
        }
    }
}

pub unsafe extern "win64" fn DuplicateToken(existing: HANDLE, imp_level: u32, out: *mut HANDLE) -> BOOL {
    DuplicateTokenEx(existing, 0x0c, std::ptr::null_mut(), imp_level, TOKEN_IMPERSONATION, out)
}

pub unsafe extern "win64" fn DuplicateTokenEx(
    existing: HANDLE,
    access: u32,
    _attr: *mut c_void,
    imp_level: u32,
    token_type: u32,
    out: *mut HANDLE,
) -> BOOL {
    if out.is_null() || !matches!(token_type, TOKEN_PRIMARY | TOKEN_IMPERSONATION) || imp_level > 3 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let original = match TokenObj::resolve(existing, 0x0002) {
        Ok(t) => t,
        Err(e) => return err(e, FALSE),
    };
    if original.token_type == TOKEN_IMPERSONATION && imp_level > original.imp_level {
        return err(1346, FALSE);
    }
    let new_access = if access == 0 { original.access } else { TokenObj::access_mask(access) };
    std::ptr::write_unaligned(out, handles::insert(new_token_obj(token_type, imp_level, new_access)));
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

pub unsafe extern "win64" fn CheckTokenMembership(token: HANDLE, sid: *const c_void, is_member: *mut BOOL) -> BOOL {
    if is_member.is_null() || sid.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if !token.is_null() {
        match TokenObj::resolve(token, 0x0008) {
            Ok(t) if t.token_type == TOKEN_IMPERSONATION => {}
            Ok(_) => return err(1309, FALSE), // ERROR_NO_IMPERSONATION_TOKEN
            Err(e) => return err(e, FALSE),
        }
    }
    let groups = token_default_groups();
    let enabled = groups
        .iter()
        .any(|(group, flags)| (flags & SE_GROUP_ENABLED) != 0 && EqualSid(sid.cast(), group.as_ptr()) != 0);
    is_member.write_unaligned(enabled as BOOL);
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

use crate::misc::bcrypt::{CipherMode as BcCipherMode, Digest as BcDigest, IncrementalDigest, Rc4State, RsaKey, SymAlg};

const CALG_MD2: u32 = 0x0000_8001;
const CALG_MD4: u32 = 0x0000_8002;
const CALG_MD5: u32 = 0x0000_8003;
const CALG_SHA1: u32 = 0x0000_8004;
const CALG_SHA_256: u32 = 0x0000_800C;
const CALG_SHA_384: u32 = 0x0000_800D;
const CALG_SHA_512: u32 = 0x0000_800E;

const CALG_DES: u32 = 0x0000_6601;
const CALG_RC2: u32 = 0x0000_6602;
const CALG_3DES: u32 = 0x0000_6603;
const CALG_3DES_112: u32 = 0x0000_6609;
const CALG_AES_128: u32 = 0x0000_660E;
const CALG_AES_192: u32 = 0x0000_660F;
const CALG_AES_256: u32 = 0x0000_6610;
const CALG_AES: u32 = 0x0000_6611;
const CALG_RC4: u32 = 0x0000_6801;
const CALG_RSA_SIGN: u32 = 0x0000_2400;
const CALG_RSA_KEYX: u32 = 0x0000_A400;

const AT_KEYEXCHANGE: u32 = 1;
const AT_SIGNATURE: u32 = 2;

const HP_ALGID: u32 = 0x0001;
const HP_HASHVAL: u32 = 0x0002;
const HP_HASHSIZE: u32 = 0x0004;

const KP_IV: u32 = 1;
const KP_SALT: u32 = 2;
const KP_PADDING: u32 = 3;
const KP_MODE: u32 = 4;
const KP_MODE_BITS: u32 = 5;
const KP_PERMISSIONS: u32 = 6;
const KP_ALGID: u32 = 7;
const KP_BLOCKLEN: u32 = 8;
const KP_KEYLEN: u32 = 9;
const KP_SALT_EX: u32 = 10;
const KP_EFFECTIVE_KEYLEN: u32 = 19;

const CRYPT_MODE_CBC: u32 = 1;
const CRYPT_MODE_ECB: u32 = 2;
const CRYPT_MODE_CFB: u32 = 4;

const PKCS5_PADDING: u32 = 1;

const CRYPT_ENCRYPT: u32 = 0x0000_0001;
const CRYPT_DECRYPT: u32 = 0x0000_0002;
const CRYPT_EXPORT: u32 = 0x0000_0004;
const CRYPT_READ: u32 = 0x0000_0008;
const CRYPT_WRITE: u32 = 0x0000_0010;
const CRYPT_MAC: u32 = 0x0000_0020;
const CRYPT_EXPORT_KEY: u32 = 0x0000_0040;
const CRYPT_IMPORT_KEY: u32 = 0x0000_0080;

const CRYPT_EXPORTABLE: u32 = 0x0000_0001;
const CRYPT_CREATE_SALT: u32 = 0x0000_0004;
const CRYPT_NO_SALT: u32 = 0x0000_0010;
const CRYPT_NOHASHOID: u32 = 0x0000_0001;
const CRYPT_OAEP: u32 = 0x0000_0040;
const CRYPT_VERIFYCONTEXT: u32 = 0xF000_0000;
const CRYPT_NEWKEYSET: u32 = 0x0000_0008;
const CRYPT_DELETEKEYSET: u32 = 0x0000_0010;
const CRYPT_MACHINE_KEYSET: u32 = 0x0000_0020;

const SIMPLEBLOB: u32 = 0x1;
const PUBLICKEYBLOB: u32 = 0x6;
const PRIVATEKEYBLOB: u32 = 0x7;
const PLAINTEXTKEYBLOB: u32 = 0x8;
const CUR_BLOB_VERSION: u8 = 2;

const PP_IMPTYPE: u32 = 3;
const PP_NAME: u32 = 4;
const PP_VERSION: u32 = 5;
const PP_CONTAINER: u32 = 6;
const PP_PROVTYPE: u32 = 16;
const PP_KEYSET_TYPE: u32 = 27;

const NTE_BAD_UID: u32 = 0x8009_0001;
const NTE_BAD_HASH: u32 = 0x8009_0002;
const NTE_BAD_KEY: u32 = 0x8009_0003;
const NTE_BAD_LEN: u32 = 0x8009_0004;
const NTE_BAD_DATA: u32 = 0x8009_0005;
const NTE_BAD_SIGNATURE: u32 = 0x8009_0006;
const NTE_BAD_VER: u32 = 0x8009_0007;
const NTE_BAD_ALGID: u32 = 0x8009_0008;
const NTE_BAD_FLAGS: u32 = 0x8009_0009;
const NTE_BAD_TYPE: u32 = 0x8009_000A;
const NTE_BAD_KEY_STATE: u32 = 0x8009_000B;
const NTE_NO_KEY: u32 = 0x8009_000D;
const NTE_EXISTS: u32 = 0x8009_000F;
const NTE_PERM: u32 = 0x8009_0010;
const NTE_BAD_KEYSET: u32 = 0x8009_0016;
const NTE_FAIL: u32 = 0x8009_0020;

const CAPI_STORE_MAGIC_V1: &[u8; 8] = b"PFCAPI01";
const CAPI_STORE_MAGIC_V2: &[u8; 8] = b"PFCAPI02";
const CAPI_ENVELOPE_MAGIC: &[u8; 8] = b"PFCAPIE1";

type CapiDiskKey = (u32, String, String);
type CapiCacheKey = (bool, u32, String, String);

fn capi_default_provider_name(prov_type: u32) -> &'static str {
    match prov_type {
        1 => "Microsoft Strong Cryptographic Provider",
        24 => "Microsoft Enhanced RSA and AES Cryptographic Provider",
        _ => "Microsoft Enhanced RSA and AES Cryptographic Provider",
    }
}

fn capi_provider_default_rc_key_len(prov_name: &str) -> usize {
    if prov_name.to_ascii_lowercase().starts_with("microsoft base cryptographic provider") {
        5 // 40-bit default for Microsoft Base Cryptographic Provider
    } else {
        16 // 128-bit default for Strong / Enhanced / AES providers
    }
}

#[derive(Clone)]
struct StoredUserKey {
    key: RsaKey,
    exportable: bool,
}

#[derive(Clone, Default)]
struct CapiContainerKeys {
    keyx: Option<StoredUserKey>,
    sign: Option<StoredUserKey>,
}

struct CapiProvider {
    prov_type: u32,
    prov_name: String,
    container: String,
    machine_keyset: bool,
    ephemeral: bool,
    ephemeral_keys: CapiContainerKeys,
}

#[derive(Clone)]
struct CapiHash {
    h_prov: usize,
    alg_id: u32,
    digest: BcDigest,
    state: IncrementalDigest,
    override_val: Option<Vec<u8>>,
}

#[derive(Clone)]
enum CapiKeyMaterial {
    Rsa {
        key: RsaKey,
    },
    Symmetric {
        alg: SymAlg,
        key: Vec<u8>,
        salt: Vec<u8>,
        mode: BcCipherMode,
        padding: u32,
        effective_key_len: u32,
        iv: Vec<u8>,
        running_iv: Vec<u8>,
        msg_block_len: u32,
        rc4: Option<Rc4State>,
    },
}

#[derive(Clone)]
struct CapiKey {
    alg_id: u32,
    exportable: bool,
    permissions: u32,
    material: CapiKeyMaterial,
}

static NEXT_CAPI_HANDLE: AtomicU64 = AtomicU64::new(0xC000_0001);

fn capi_providers() -> &'static std::sync::Mutex<std::collections::HashMap<usize, CapiProvider>> {
    static M: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, CapiProvider>>> =
        std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn capi_containers() -> &'static std::sync::Mutex<std::collections::HashMap<CapiCacheKey, CapiContainerKeys>> {
    static M: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<CapiCacheKey, CapiContainerKeys>>> =
        std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn capi_hashes() -> &'static std::sync::Mutex<std::collections::HashMap<usize, CapiHash>> {
    static M: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, CapiHash>>> =
        std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn capi_keys() -> &'static std::sync::Mutex<std::collections::HashMap<usize, CapiKey>> {
    static M: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, CapiKey>>> =
        std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn capi_alg_to_digest(alg_id: u32) -> Option<BcDigest> {
    match alg_id {
        CALG_MD2 => Some(BcDigest::Md2),
        CALG_MD4 => Some(BcDigest::Md4),
        CALG_MD5 => Some(BcDigest::Md5),
        CALG_SHA1 => Some(BcDigest::Sha1),
        CALG_SHA_256 => Some(BcDigest::Sha256),
        CALG_SHA_384 => Some(BcDigest::Sha384),
        CALG_SHA_512 => Some(BcDigest::Sha512),
        _ => None,
    }
}

fn capi_alg_to_sym(alg_id: u32) -> Option<(SymAlg, usize)> {
    match alg_id {
        CALG_AES_128 => Some((SymAlg::Aes, 16)),
        CALG_AES_192 => Some((SymAlg::Aes, 24)),
        CALG_AES_256 => Some((SymAlg::Aes, 32)),
        CALG_AES => Some((SymAlg::Aes, 16)),
        CALG_3DES => Some((SymAlg::Des3, 24)),
        CALG_3DES_112 => Some((SymAlg::Des3_112, 16)),
        CALG_DES => Some((SymAlg::Des, 8)),
        CALG_RC2 => Some((SymAlg::Rc2, 16)),
        CALG_RC4 => Some((SymAlg::Rc4, 16)),
        _ => None,
    }
}

fn capi_valid_sym_key_len(alg_id: u32, sym_alg: SymAlg, cb_key: usize) -> bool {
    match sym_alg {
        SymAlg::Aes => match alg_id {
            CALG_AES_128 => cb_key == 16,
            CALG_AES_192 => cb_key == 24,
            CALG_AES_256 => cb_key == 32,
            _ => matches!(cb_key, 16 | 24 | 32),
        },
        SymAlg::Des => cb_key == 8,
        SymAlg::Des3_112 => cb_key == 16,
        SymAlg::Des3 => cb_key == 24,
        SymAlg::Rc2 => (1..=128).contains(&cb_key),
        SymAlg::Rc4 => (1..=256).contains(&cb_key),
    }
}

fn capi_default_permissions(exportable: bool) -> u32 {
    let base = CRYPT_ENCRYPT
        | CRYPT_DECRYPT
        | CRYPT_READ
        | CRYPT_WRITE
        | CRYPT_MAC
        | CRYPT_EXPORT_KEY
        | CRYPT_IMPORT_KEY;
    if exportable { base | CRYPT_EXPORT } else { base }
}

fn capi_effective_sym_key(alg: SymAlg, key: &[u8], salt: &[u8]) -> Vec<u8> {
    if matches!(alg, SymAlg::Rc2 | SymAlg::Rc4) && !salt.is_empty() {
        let mut out = Vec::with_capacity(key.len() + salt.len());
        out.extend_from_slice(key);
        out.extend_from_slice(salt);
        out
    } else {
        key.to_vec()
    }
}

unsafe fn capi_salt_from_flags(
    alg: SymAlg,
    key_len: usize,
    dw_flags: u32,
    hash_tail: Option<&[u8]>,
) -> Result<Vec<u8>, u32> {
    let no_salt = (dw_flags & CRYPT_NO_SALT) != 0;
    let create_salt = (dw_flags & CRYPT_CREATE_SALT) != 0;
    if no_salt && create_salt {
        return Err(NTE_BAD_FLAGS);
    }
    if !matches!(alg, SymAlg::Rc2 | SymAlg::Rc4) || no_salt {
        return Ok(Vec::new());
    }
    if create_salt {
        let salt_len = 16usize.saturating_sub(key_len);
        if salt_len == 0 {
            return Ok(Vec::new());
        }
        let mut salt = vec![0u8; salt_len];
        if let Some(tail) = hash_tail {
            let n = tail.len().min(salt_len);
            salt[..n].copy_from_slice(&tail[..n]);
        } else if !crate::det_entropy::fill(salt.as_mut_ptr(), salt.len()) {
            libc::arc4random_buf(salt.as_mut_ptr() as *mut c_void, salt.len());
        }
        return Ok(salt);
    }
    // Microsoft Learn (Salt Value Functionality): 40-bit RC2/RC4 keys receive an 11-byte
    // zero salt by default unless CRYPT_NO_SALT is requested.
    if key_len == 5 {
        Ok(vec![0u8; 11])
    } else {
        Ok(Vec::new())
    }
}

fn capi_pkcs5_pad(data: &[u8], bs: usize) -> Vec<u8> {
    let pad = bs - (data.len() % bs);
    let mut out = Vec::with_capacity(data.len() + pad);
    out.extend_from_slice(data);
    out.resize(data.len() + pad, pad as u8);
    out
}

fn capi_pkcs5_unpad(mut data: Vec<u8>, bs: usize) -> Result<Vec<u8>, u32> {
    if data.is_empty() || data.len() % bs != 0 {
        return Err(NTE_BAD_DATA);
    }
    let pad = *data.last().unwrap() as usize;
    if pad == 0 || pad > bs || pad > data.len() {
        return Err(NTE_BAD_DATA);
    }
    if data[data.len() - pad..].iter().any(|&b| b as usize != pad) {
        return Err(NTE_BAD_DATA);
    }
    data.truncate(data.len() - pad);
    Ok(data)
}

fn capi_wrap_sym_bytes(sym_key: &CapiKey, encrypt: bool, data: &[u8]) -> Result<Vec<u8>, u32> {
    let required = if encrypt {
        CRYPT_ENCRYPT | CRYPT_EXPORT_KEY
    } else {
        CRYPT_DECRYPT | CRYPT_IMPORT_KEY
    };
    if (sym_key.permissions & required) != required {
        return Err(NTE_PERM);
    }
    let CapiKeyMaterial::Symmetric {
        alg,
        key,
        salt,
        mode,
        padding,
        effective_key_len,
        iv,
        msg_block_len,
        ..
    } = &sym_key.material
    else {
        return Err(NTE_BAD_KEY);
    };
    let eff_key = capi_effective_sym_key(*alg, key, salt);
    if *alg == SymAlg::Rc4 {
        return Ok(Rc4State::new(&eff_key).apply(data));
    }
    let bs = alg.block_size();
    let use_pkcs7 = *padding == PKCS5_PADDING;
    if (*mode != BcCipherMode::Cfb || (!encrypt && use_pkcs7)) && (!encrypt || !use_pkcs7) && data.len() % bs != 0 {
        return Err(NTE_BAD_DATA);
    }
    let mut iv_work = iv.clone();
    if *mode == BcCipherMode::Cfb && use_pkcs7 {
        if encrypt {
            let padded = capi_pkcs5_pad(data, bs);
            return crate::misc::bcrypt::sym_crypt_block_mode(
                *alg,
                *mode,
                true,
                false,
                &eff_key,
                *effective_key_len,
                &mut iv_work,
                *msg_block_len as usize,
                &padded,
            )
            .map_err(|_| NTE_BAD_DATA);
        }
        let plain = crate::misc::bcrypt::sym_crypt_block_mode(
            *alg,
            *mode,
            false,
            false,
            &eff_key,
            *effective_key_len,
            &mut iv_work,
            *msg_block_len as usize,
            data,
        )
        .map_err(|_| NTE_BAD_DATA)?;
        return capi_pkcs5_unpad(plain, bs);
    }
    crate::misc::bcrypt::sym_crypt_block_mode(
        *alg,
        *mode,
        encrypt,
        use_pkcs7,
        &eff_key,
        *effective_key_len,
        &mut iv_work,
        *msg_block_len as usize,
        data,
    )
    .map_err(|_| NTE_BAD_DATA)
}

fn capi_sym_key(alg_id: u32, alg: SymAlg, key: Vec<u8>, salt: Vec<u8>, exportable: bool) -> CapiKey {
    let bs = alg.block_size();
    let mode = if alg == SymAlg::Rc4 { BcCipherMode::Na } else { BcCipherMode::Cbc };
    let eff = if alg == SymAlg::Rc2 { (key.len() * 8) as u32 } else { 0 };
    let eff_key = capi_effective_sym_key(alg, &key, &salt);
    let rc4 = (alg == SymAlg::Rc4).then(|| Rc4State::new(&eff_key));
    CapiKey {
        alg_id,
        exportable,
        permissions: capi_default_permissions(exportable),
        material: CapiKeyMaterial::Symmetric {
            alg,
            key,
            salt,
            mode,
            padding: PKCS5_PADDING,
            effective_key_len: eff,
            iv: vec![0u8; bs],
            running_iv: vec![0u8; bs],
            // Microsoft Learn (CryptSetKeyParam KP_MODE_BITS): CAPI CFB defaults to 8-bit (1-byte) feedback.
            msg_block_len: 1,
            rc4,
        },
    }
}

fn capi_store_lock(machine_keyset: bool) -> &'static std::sync::Mutex<()> {
    static USER_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    static MACHINE_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    if machine_keyset {
        MACHINE_LOCK.get_or_init(|| std::sync::Mutex::new(()))
    } else {
        USER_LOCK.get_or_init(|| std::sync::Mutex::new(()))
    }
}

struct CapiStoreGuard {
    _mem: std::sync::MutexGuard<'static, ()>,
    flock_file: std::fs::File,
}

impl Drop for CapiStoreGuard {
    fn drop(&mut self) {
        use std::os::unix::io::AsRawFd;
        unsafe {
            libc::flock(self.flock_file.as_raw_fd(), libc::LOCK_UN);
        }
    }
}

fn acquire_capi_store_lock(machine_keyset: bool) -> std::io::Result<CapiStoreGuard> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    use std::os::unix::io::AsRawFd;

    let mem = capi_store_lock(machine_keyset).lock().unwrap();
    let lock_path = capi_store_path(machine_keyset)
        .parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "no parent"))?
        .join(".containers.lock");
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
    Ok(CapiStoreGuard { _mem: mem, flock_file: file })
}

fn capi_store_user() -> String {
    let raw = crate::process::get_env("USERNAME")
        .filter(|s| !s.trim().is_empty())
        .or_else(|| std::env::var("USER").ok().filter(|s| !s.trim().is_empty()))
        .unwrap_or_else(|| "player".to_string());
    let clean: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect();
    if clean.is_empty() {
        "player".to_string()
    } else {
        clean
    }
}

fn capi_store_path(machine_keyset: bool) -> std::path::PathBuf {
    let dir = crate::path::capi_keystore_dir();
    if machine_keyset {
        dir.join("machine_containers.bin")
    } else {
        let user = capi_store_user();
        dir.join(format!("user_{user}_containers.bin"))
    }
}

fn capi_secret_path(machine_keyset: bool) -> std::path::PathBuf {
    let dir = crate::path::capi_keystore_dir();
    if machine_keyset {
        dir.join("machine.key")
    } else {
        let user = capi_store_user();
        dir.join(format!("user_{user}.key"))
    }
}

fn zeroize_bytes(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        unsafe { std::ptr::write_volatile(b, 0) };
    }
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}

/// Load or atomically generate a 32-byte provider-only host secret stored with
/// `0600` permissions in the host-private `.capi-keystore` directory (outside
/// all guest `C:\` mounts and unreachable from guest-callable `CryptUnprotectData`).
fn load_or_create_capi_secret(machine_keyset: bool) -> std::io::Result<[u8; 32]> {
    use std::io::Write;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let path = capi_secret_path(machine_keyset);
    if let Ok(mut bytes) = std::fs::read(&path) {
        if bytes.len() == 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            zeroize_bytes(&mut bytes);
            return Ok(key);
        }
        zeroize_bytes(&mut bytes);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "CAPI provider host secret has invalid length",
        ));
    }

    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "CAPI secret path has no parent")
    })?;
    std::fs::create_dir_all(parent)?;
    let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));

    let mut secret = [0u8; 32];
    unsafe {
        libc::arc4random_buf(secret.as_mut_ptr() as *mut c_void, secret.len());
    }
    let mut rnd = [0u8; 8];
    unsafe {
        libc::arc4random_buf(rnd.as_mut_ptr() as *mut c_void, rnd.len());
    }
    let suffix = u64::from_le_bytes(rnd);
    let tmp_name = format!(
        ".{}.tmp.{}.{suffix:016x}",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("capi.key"),
        std::process::id()
    );
    let tmp_path = parent.join(tmp_name);

    {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&tmp_path)?;
        f.write_all(&secret)?;
        f.sync_all()?;
    }

    // Use hard_link so racing processes never overwrite an already-created secret.
    match std::fs::hard_link(&tmp_path, &path) {
        Ok(()) => {
            let _ = std::fs::remove_file(&tmp_path);
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
            Ok(secret)
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            let _ = std::fs::remove_file(&tmp_path);
            zeroize_bytes(&mut secret);
            let mut bytes = std::fs::read(&path)?;
            if bytes.len() != 32 {
                zeroize_bytes(&mut bytes);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "CAPI provider host secret has invalid length",
                ));
            }
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            zeroize_bytes(&mut bytes);
            Ok(key)
        }
        Err(_) => {
            let _ = std::fs::remove_file(&tmp_path);
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&path)
            {
                Ok(mut f) => {
                    f.write_all(&secret)?;
                    f.sync_all()?;
                    Ok(secret)
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    zeroize_bytes(&mut secret);
                    let mut bytes = std::fs::read(&path)?;
                    if bytes.len() != 32 {
                        zeroize_bytes(&mut bytes);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "CAPI provider host secret has invalid length",
                        ));
                    }
                    let mut key = [0u8; 32];
                    key.copy_from_slice(&bytes);
                    zeroize_bytes(&mut bytes);
                    Ok(key)
                }
                Err(e) => {
                    zeroize_bytes(&mut secret);
                    Err(e)
                }
            }
        }
    }
}

fn capi_hmac_sha256(key: &[u8; 32], msg: &[u8]) -> [u8; 32] {
    let mut ipad = [0x36u8; 64];
    let mut opad = [0x5cu8; 64];
    for i in 0..32 {
        ipad[i] ^= key[i];
        opad[i] ^= key[i];
    }
    let mut inner = IncrementalDigest::new(BcDigest::Sha256);
    inner.update(&ipad);
    inner.update(msg);
    let inner_digest = inner.finalize_clone();
    let mut outer = IncrementalDigest::new(BcDigest::Sha256);
    outer.update(&opad);
    outer.update(&inner_digest);
    let out_vec = outer.finalize_clone();
    let mut out = [0u8; 32];
    out.copy_from_slice(&out_vec[..32]);
    zeroize_bytes(&mut ipad);
    zeroize_bytes(&mut opad);
    out
}

fn capi_store_aad(machine_keyset: bool) -> Vec<u8> {
    let mut aad = Vec::with_capacity(48);
    aad.extend_from_slice(CAPI_ENVELOPE_MAGIC);
    aad.push(machine_keyset as u8);
    if machine_keyset {
        aad.extend_from_slice(b"machine");
    } else {
        aad.extend_from_slice(b"user:");
        aad.extend_from_slice(capi_store_user().as_bytes());
    }
    aad
}

fn derive_capi_store_key(machine_keyset: bool, aad: &[u8]) -> std::io::Result<[u8; 32]> {
    let mut master = load_or_create_capi_secret(machine_keyset)?;
    let mut info = Vec::with_capacity(32 + aad.len());
    info.extend_from_slice(b"port-factory-capi-keystore-v1:");
    info.extend_from_slice(aad);
    let key = capi_hmac_sha256(&master, &info);
    zeroize_bytes(&mut master);
    Ok(key)
}

fn seal_capi_store(machine_keyset: bool, plain: &[u8]) -> std::io::Result<Vec<u8>> {
    let aad = capi_store_aad(machine_keyset);
    let mut key = derive_capi_store_key(machine_keyset, &aad)?;
    let mut nonce = [0u8; 12];
    unsafe {
        libc::arc4random_buf(nonce.as_mut_ptr() as *mut c_void, nonce.len());
    }
    let res = crate::misc::bcrypt::aes_gcm_crypt(true, &key, &nonce, &aad, plain, 16, None);
    zeroize_bytes(&mut key);
    let (ct, tag) = res.map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::Other, "AES-GCM protection of CAPI key store failed")
    })?;
    let mut env = Vec::with_capacity(8 + 1 + 12 + ct.len() + 16);
    env.extend_from_slice(CAPI_ENVELOPE_MAGIC);
    env.push(machine_keyset as u8);
    env.extend_from_slice(&nonce);
    env.extend_from_slice(&ct);
    env.extend_from_slice(&tag);
    Ok(env)
}

fn unseal_capi_store(machine_keyset: bool, env: &[u8]) -> std::io::Result<Vec<u8>> {
    const HDR_LEN: usize = 8 + 1 + 12;
    const TAG_LEN: usize = 16;
    if env.len() < HDR_LEN + TAG_LEN || &env[..8] != CAPI_ENVELOPE_MAGIC || env[8] != (machine_keyset as u8) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid CAPI provider keystore envelope",
        ));
    }
    let nonce = &env[9..21];
    let ct = &env[HDR_LEN..env.len() - TAG_LEN];
    let tag = &env[env.len() - TAG_LEN..];
    let aad = capi_store_aad(machine_keyset);
    let mut key = derive_capi_store_key(machine_keyset, &aad)?;
    let res = crate::misc::bcrypt::aes_gcm_crypt(false, &key, nonce, &aad, ct, TAG_LEN, Some(tag));
    zeroize_bytes(&mut key);
    let (plain, _) = res.map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "failed to authenticate CAPI key store")
    })?;
    Ok(plain)
}

fn encode_capi_containers(map: &std::collections::HashMap<CapiDiskKey, CapiContainerKeys>) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(CAPI_STORE_MAGIC_V2);
    out.extend_from_slice(&(map.len() as u32).to_le_bytes());
    let push_opt_key = |out: &mut Vec<u8>, opt: &Option<StoredUserKey>, alg_id: u32| {
        if let Some(sk) = opt {
            if let Some(blob) = sk.key.to_capi_private_blob(alg_id) {
                out.push(1);
                out.push(sk.exportable as u8);
                out.extend_from_slice(&(blob.len() as u32).to_le_bytes());
                out.extend_from_slice(&blob);
                return;
            }
        }
        out.push(0);
    };
    for ((prov_type, prov_key, name), keys) in map {
        out.extend_from_slice(&prov_type.to_le_bytes());
        let pb = prov_key.as_bytes();
        out.extend_from_slice(&(pb.len() as u32).to_le_bytes());
        out.extend_from_slice(pb);
        let nb = name.as_bytes();
        out.extend_from_slice(&(nb.len() as u32).to_le_bytes());
        out.extend_from_slice(nb);
        push_opt_key(&mut out, &keys.keyx, CALG_RSA_KEYX);
        push_opt_key(&mut out, &keys.sign, CALG_RSA_SIGN);
    }
    out
}

fn decode_capi_containers(data: &[u8]) -> Option<std::collections::HashMap<CapiDiskKey, CapiContainerKeys>> {
    if data.len() < 12 {
        return None;
    }
    let has_prov_name = if &data[..8] == CAPI_STORE_MAGIC_V2 {
        true
    } else if &data[..8] == CAPI_STORE_MAGIC_V1 {
        false
    } else {
        return None;
    };
    let mut pos = 8usize;
    let read_u32 = |pos: &mut usize| -> Option<u32> {
        let end = pos.checked_add(4)?;
        let v = u32::from_le_bytes(data.get(*pos..end)?.try_into().ok()?);
        *pos = end;
        Some(v)
    };
    let read_opt_key = |pos: &mut usize| -> Option<Option<StoredUserKey>> {
        let tag = *data.get(*pos)?;
        *pos += 1;
        match tag {
            0 => Some(None),
            1 => {
                let exp = *data.get(*pos)? != 0;
                *pos += 1;
                let len = read_u32(pos)? as usize;
                let end = pos.checked_add(len)?;
                let blob = data.get(*pos..end)?;
                *pos = end;
                let (rsa, _, is_priv) = RsaKey::from_capi_blob(blob)?;
                if !is_priv {
                    return None;
                }
                Some(Some(StoredUserKey { key: rsa, exportable: exp }))
            }
            _ => None,
        }
    };
    let count = read_u32(&mut pos)? as usize;
    let mut map = std::collections::HashMap::with_capacity(count.min(256));
    for _ in 0..count {
        let prov_type = read_u32(&mut pos)?;
        let prov_key = if has_prov_name {
            let plen = read_u32(&mut pos)? as usize;
            let end = pos.checked_add(plen)?;
            let s = String::from_utf8(data.get(pos..end)?.to_vec()).ok()?;
            pos = end;
            s
        } else {
            capi_default_provider_name(prov_type).to_ascii_lowercase()
        };
        let nlen = read_u32(&mut pos)? as usize;
        let end = pos.checked_add(nlen)?;
        let name = String::from_utf8(data.get(pos..end)?.to_vec()).ok()?;
        pos = end;
        let keyx = read_opt_key(&mut pos)?;
        let sign = read_opt_key(&mut pos)?;
        map.insert((prov_type, prov_key, name), CapiContainerKeys { keyx, sign });
    }
    Some(map)
}

unsafe fn load_persisted_capi_containers(
    machine_keyset: bool,
) -> std::io::Result<std::collections::HashMap<CapiDiskKey, CapiContainerKeys>> {
    let path = capi_store_path(machine_keyset);
    let enc = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(std::collections::HashMap::new()),
        Err(e) => return Err(e),
    };
    let mut plain = unseal_capi_store(machine_keyset, &enc)?;
    let decoded = decode_capi_containers(&plain);
    zeroize_bytes(&mut plain);
    decoded.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "corrupt CAPI key store"))
}

unsafe fn save_persisted_capi_containers(
    machine_keyset: bool,
    map: &std::collections::HashMap<CapiDiskKey, CapiContainerKeys>,
) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let path = capi_store_path(machine_keyset);
    let parent = path.parent().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "no parent"))?;
    std::fs::create_dir_all(parent)?;
    let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));

    let mut raw = encode_capi_containers(map);
    let enc = seal_capi_store(machine_keyset, &raw);
    zeroize_bytes(&mut raw);
    let enc = enc?;

    let mut rnd = [0u8; 8];
    libc::arc4random_buf(rnd.as_mut_ptr() as *mut c_void, rnd.len());
    let suffix = u64::from_le_bytes(rnd);
    let tmp_path = parent.join(format!(".containers.bin.tmp.{}.{suffix:016x}", std::process::id()));
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

unsafe fn store_container_user_key(h_prov: usize, key_spec: u32, rsa: RsaKey, exportable: bool) -> Result<(), u32> {
    let mut provs = capi_providers().lock().unwrap();
    let Some(prov) = provs.get_mut(&h_prov) else {
        return Err(NTE_BAD_UID);
    };
    let stored = StoredUserKey { key: rsa, exportable };
    if prov.ephemeral {
        if key_spec == AT_SIGNATURE {
            prov.ephemeral_keys.sign = Some(stored);
        } else {
            prov.ephemeral_keys.keyx = Some(stored);
        }
        return Ok(());
    }
    let machine = prov.machine_keyset;
    let prov_key = prov.prov_name.to_ascii_lowercase();
    let disk_key = (prov.prov_type, prov_key.clone(), prov.container.clone());
    let cache_key = (machine, prov.prov_type, prov_key, prov.container.clone());
    drop(provs);
    let _guard = acquire_capi_store_lock(machine).map_err(|_| NTE_FAIL)?;
    let mut disk = load_persisted_capi_containers(machine).map_err(|_| NTE_FAIL)?;
    let entry = disk.entry(disk_key).or_default();
    if key_spec == AT_SIGNATURE {
        entry.sign = Some(stored.clone());
    } else {
        entry.keyx = Some(stored.clone());
    }
    save_persisted_capi_containers(machine, &disk).map_err(|_| NTE_FAIL)?;
    let mut mem = capi_containers().lock().unwrap();
    let mem_entry = mem.entry(cache_key).or_default();
    if key_spec == AT_SIGNATURE {
        mem_entry.sign = Some(stored);
    } else {
        mem_entry.keyx = Some(stored);
    }
    Ok(())
}

unsafe fn load_container_user_key(h_prov: usize, key_spec: u32) -> Result<Option<StoredUserKey>, u32> {
    let provs = capi_providers().lock().unwrap();
    let Some(prov) = provs.get(&h_prov) else {
        return Err(NTE_BAD_UID);
    };
    if prov.ephemeral {
        let k = if key_spec == AT_SIGNATURE {
            prov.ephemeral_keys.sign.clone()
        } else {
            prov.ephemeral_keys.keyx.clone()
        };
        return Ok(k);
    }
    let machine = prov.machine_keyset;
    let prov_key = prov.prov_name.to_ascii_lowercase();
    let disk_key = (prov.prov_type, prov_key.clone(), prov.container.clone());
    let cache_key = (machine, prov.prov_type, prov_key, prov.container.clone());
    drop(provs);
    {
        let containers = capi_containers().lock().unwrap();
        if let Some(c) = containers.get(&cache_key) {
            let k = if key_spec == AT_SIGNATURE { c.sign.clone() } else { c.keyx.clone() };
            if k.is_some() {
                return Ok(k);
            }
        }
    }
    // Load from persisted DPAPI-encrypted store (handles cross-process / post-restart access).
    let _guard = acquire_capi_store_lock(machine).map_err(|_| NTE_FAIL)?;
    let disk = load_persisted_capi_containers(machine).map_err(|_| NTE_FAIL)?;
    let found = disk.get(&disk_key).cloned();
    if let Some(ref c) = found {
        capi_containers().lock().unwrap().insert(cache_key, c.clone());
    }
    Ok(found.and_then(|c| if key_spec == AT_SIGNATURE { c.sign } else { c.keyx }))
}

fn capi_hash_value(h: &CapiHash) -> Vec<u8> {
    if let Some(ref ov) = h.override_val {
        ov.clone()
    } else {
        h.state.finalize_clone()
    }
}

unsafe fn crypt_acquire_context_impl(
    ph_prov: *mut usize,
    container: Option<String>,
    provider: Option<String>,
    dw_prov_type: u32,
    dw_flags: u32,
) -> BOOL {
    let cname = container.unwrap_or_default();
    let prov_name = provider
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| capi_default_provider_name(dw_prov_type).to_string());
    let prov_key = prov_name.to_ascii_lowercase();
    let machine = (dw_flags & CRYPT_MACHINE_KEYSET) != 0;
    if (dw_flags & CRYPT_DELETEKEYSET) != 0 {
        let disk_key = (dw_prov_type, prov_key.clone(), cname.clone());
        let cache_key = (machine, dw_prov_type, prov_key, cname);
        let Ok(_guard) = acquire_capi_store_lock(machine) else {
            return err(NTE_FAIL, FALSE);
        };
        let mut disk = match load_persisted_capi_containers(machine) {
            Ok(d) => d,
            Err(_) => return err(NTE_FAIL, FALSE),
        };
        let removed_disk = disk.remove(&disk_key).is_some();
        let removed_mem = capi_containers().lock().unwrap().remove(&cache_key).is_some();
        if removed_disk && save_persisted_capi_containers(machine, &disk).is_err() {
            return err(NTE_FAIL, FALSE);
        }
        if !ph_prov.is_null() {
            std::ptr::write_unaligned(ph_prov, 0);
        }
        return if removed_disk || removed_mem { TRUE } else { err(NTE_BAD_KEYSET, FALSE) };
    }
    if ph_prov.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let ephemeral = (dw_flags & CRYPT_VERIFYCONTEXT) != 0;
    if !ephemeral {
        let disk_key = (dw_prov_type, prov_key.clone(), cname.clone());
        let cache_key = (machine, dw_prov_type, prov_key, cname.clone());
        let Ok(_guard) = acquire_capi_store_lock(machine) else {
            return err(NTE_FAIL, FALSE);
        };
        let mut disk = match load_persisted_capi_containers(machine) {
            Ok(d) => d,
            Err(_) => return err(NTE_FAIL, FALSE),
        };
        if (dw_flags & CRYPT_NEWKEYSET) != 0 {
            if disk.contains_key(&disk_key) {
                return err(NTE_EXISTS, FALSE);
            }
            disk.insert(disk_key, CapiContainerKeys::default());
            if save_persisted_capi_containers(machine, &disk).is_err() {
                return err(NTE_FAIL, FALSE);
            }
            capi_containers().lock().unwrap().insert(cache_key, CapiContainerKeys::default());
        } else {
            let existed = disk.contains_key(&disk_key);
            let entry = disk.entry(disk_key).or_default().clone();
            if !existed && save_persisted_capi_containers(machine, &disk).is_err() {
                return err(NTE_FAIL, FALSE);
            }
            capi_containers().lock().unwrap().insert(cache_key, entry);
        }
    }
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    let prov = CapiProvider {
        prov_type: dw_prov_type,
        prov_name,
        container: cname,
        machine_keyset: machine,
        ephemeral,
        ephemeral_keys: CapiContainerKeys::default(),
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
        PP_KEYSET_TYPE => (if prov.machine_keyset { CRYPT_MACHINE_KEYSET } else { 0u32 }).to_le_bytes().to_vec(),
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
        return err(ERROR_MORE_DATA, FALSE);
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
    let Some(digest) = capi_alg_to_digest(alg_id) else {
        return err(NTE_BAD_ALGID, FALSE);
    };
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    capi_hashes().lock().unwrap().insert(
        h,
        CapiHash {
            h_prov,
            alg_id,
            digest,
            state: IncrementalDigest::new(digest),
            override_val: None,
        },
    );
    std::ptr::write_unaligned(ph_hash, h);
    TRUE
}

pub unsafe extern "win64" fn CryptDuplicateHash(
    h_hash: usize,
    _pdw_reserved: *const u32,
    _dw_flags: u32,
    ph_hash: *mut usize,
) -> BOOL {
    if ph_hash.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let mut hashes = capi_hashes().lock().unwrap();
    let Some(orig) = hashes.get(&h_hash).cloned() else {
        return err(NTE_BAD_HASH, FALSE);
    };
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    hashes.insert(h, orig);
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
        let slice = std::slice::from_raw_parts(pb_data, dw_data_len as usize);
        h.state.update(slice);
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
        HP_HASHSIZE => (h.digest.len() as u32).to_le_bytes().to_vec(),
        HP_HASHVAL => capi_hash_value(h),
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
        return err(ERROR_MORE_DATA, FALSE);
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
        let len = h.digest.len();
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

pub unsafe extern "win64" fn CryptGenKey(
    h_prov: usize,
    alg_id: u32,
    dw_flags: u32,
    ph_key: *mut usize,
) -> BOOL {
    if ph_key.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(ph_key, 0);
    let prov_name = {
        let provs = capi_providers().lock().unwrap();
        let Some(p) = provs.get(&h_prov) else {
            return err(NTE_BAD_UID, FALSE);
        };
        p.prov_name.clone()
    };
    let exportable = (dw_flags & CRYPT_EXPORTABLE) != 0;
    let req_bits = dw_flags >> 16;

    // Asymmetric RSA key generation (AT_KEYEXCHANGE / AT_SIGNATURE / CALG_RSA_KEYX / CALG_RSA_SIGN)
    if matches!(alg_id, AT_KEYEXCHANGE | AT_SIGNATURE | CALG_RSA_KEYX | CALG_RSA_SIGN) {
        let bits = if req_bits == 0 { 2048 } else { req_bits };
        if !(1024..=4096).contains(&bits) || bits % 64 != 0 {
            return err(NTE_BAD_FLAGS, FALSE);
        }
        let Ok(rsa) = RsaKey::generate(bits) else {
            return err(NTE_BAD_FLAGS, FALSE);
        };
        let (key_spec, canonical_alg) = if matches!(alg_id, AT_SIGNATURE | CALG_RSA_SIGN) {
            (AT_SIGNATURE, CALG_RSA_SIGN)
        } else {
            (AT_KEYEXCHANGE, CALG_RSA_KEYX)
        };
        if let Err(e) = store_container_user_key(h_prov, key_spec, rsa.clone(), exportable) {
            return err(e, FALSE);
        }
        let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
        capi_keys().lock().unwrap().insert(
            h,
            CapiKey {
                alg_id: canonical_alg,
                exportable,
                permissions: capi_default_permissions(exportable),
                material: CapiKeyMaterial::Rsa { key: rsa },
            },
        );
        std::ptr::write_unaligned(ph_key, h);
        return TRUE;
    }

    // Symmetric session key generation
    let Some((sym_alg, default_len)) = capi_alg_to_sym(alg_id) else {
        return err(NTE_BAD_ALGID, FALSE);
    };
    let key_len = match sym_alg {
        SymAlg::Aes => match alg_id {
            CALG_AES_128 => 16,
            CALG_AES_192 => 24,
            CALG_AES_256 => 32,
            CALG_AES => match req_bits {
                0 | 128 => 16,
                192 => 24,
                256 => 32,
                _ => return err(NTE_BAD_FLAGS, FALSE),
            },
            _ => default_len,
        },
        SymAlg::Des => 8,
        SymAlg::Des3_112 => 16,
        SymAlg::Des3 => 24,
        SymAlg::Rc2 => {
            if req_bits == 0 {
                capi_provider_default_rc_key_len(&prov_name)
            } else if req_bits % 8 == 0 && (40..=1024).contains(&req_bits) {
                (req_bits / 8) as usize
            } else {
                return err(NTE_BAD_FLAGS, FALSE);
            }
        }
        SymAlg::Rc4 => {
            if req_bits == 0 {
                capi_provider_default_rc_key_len(&prov_name)
            } else if req_bits % 8 == 0 && (40..=2048).contains(&req_bits) {
                (req_bits / 8) as usize
            } else {
                return err(NTE_BAD_FLAGS, FALSE);
            }
        }
    };
    let mut raw = vec![0u8; key_len];
    if !crate::det_entropy::fill(raw.as_mut_ptr(), raw.len()) {
        libc::arc4random_buf(raw.as_mut_ptr() as *mut c_void, raw.len());
    }
    let canonical_alg = if alg_id == CALG_AES {
        match key_len {
            16 => CALG_AES_128,
            24 => CALG_AES_192,
            _ => CALG_AES_256,
        }
    } else {
        alg_id
    };
    let salt = match capi_salt_from_flags(sym_alg, key_len, dw_flags, None) {
        Ok(s) => s,
        Err(e) => return err(e, FALSE),
    };
    let k = capi_sym_key(canonical_alg, sym_alg, raw, salt, exportable);
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    capi_keys().lock().unwrap().insert(h, k);
    std::ptr::write_unaligned(ph_key, h);
    TRUE
}

pub unsafe extern "win64" fn CryptDeriveKey(
    h_prov: usize,
    alg_id: u32,
    h_base_data: usize,
    dw_flags: u32,
    ph_key: *mut usize,
) -> BOOL {
    if ph_key.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(ph_key, 0);
    let prov_name = {
        let provs = capi_providers().lock().unwrap();
        let Some(p) = provs.get(&h_prov) else {
            return err(NTE_BAD_UID, FALSE);
        };
        p.prov_name.clone()
    };
    let Some((sym_alg, default_len)) = capi_alg_to_sym(alg_id) else {
        return err(NTE_BAD_ALGID, FALSE);
    };
    let (digest, hash_val) = {
        let hashes = capi_hashes().lock().unwrap();
        let Some(h) = hashes.get(&h_base_data) else {
            return err(NTE_BAD_HASH, FALSE);
        };
        (h.digest, capi_hash_value(h))
    };
    let req_bits = dw_flags >> 16;
    let key_len = match sym_alg {
        SymAlg::Aes => match alg_id {
            CALG_AES_128 => 16,
            CALG_AES_192 => 24,
            CALG_AES_256 => 32,
            CALG_AES => match req_bits {
                0 | 128 => 16,
                192 => 24,
                256 => 32,
                _ => return err(NTE_BAD_FLAGS, FALSE),
            },
            _ => default_len,
        },
        SymAlg::Des => 8,
        SymAlg::Des3_112 => 16,
        SymAlg::Des3 => 24,
        SymAlg::Rc2 | SymAlg::Rc4 => {
            if req_bits == 0 {
                capi_provider_default_rc_key_len(&prov_name)
            } else if req_bits % 8 == 0 && (40..=128).contains(&req_bits) {
                (req_bits / 8) as usize
            } else {
                return err(NTE_BAD_FLAGS, FALSE);
            }
        }
    };

    // Microsoft Learn CryptDeriveKey algorithm:
    // - SHA-2 family (SHA-256/384/512): truncate raw hash directly to key_len bytes.
    // - Legacy hashes (MD2/MD4/MD5/SHA-1): if hash_len < key_len (or for 3DES/AES),
    //   expand via H(0x36^64 XOR hash) || H(0x5C^64 XOR hash).
    let is_sha2 = matches!(digest, BcDigest::Sha256 | BcDigest::Sha384 | BcDigest::Sha512);
    let derived = if is_sha2 && hash_val.len() >= key_len {
        hash_val[..key_len].to_vec()
    } else if !is_sha2 && (hash_val.len() < key_len || matches!(sym_alg, SymAlg::Aes | SymAlg::Des3 | SymAlg::Des3_112)) {
        let mut buf1 = [0x36u8; 64];
        let mut buf2 = [0x5Cu8; 64];
        for (i, &b) in hash_val.iter().take(64).enumerate() {
            buf1[i] ^= b;
            buf2[i] ^= b;
        }
        let mut exp = digest.hash(&buf1);
        exp.extend_from_slice(&digest.hash(&buf2));
        if exp.len() < key_len {
            return err(NTE_BAD_LEN, FALSE);
        }
        exp[..key_len].to_vec()
    } else {
        hash_val[..key_len].to_vec()
    };

    let exportable = (dw_flags & CRYPT_EXPORTABLE) != 0;
    let hash_tail = hash_val.get(key_len..).unwrap_or(&[]);
    let salt = match capi_salt_from_flags(sym_alg, key_len, dw_flags, Some(hash_tail)) {
        Ok(s) => s,
        Err(e) => return err(e, FALSE),
    };
    let k = capi_sym_key(alg_id, sym_alg, derived, salt, exportable);
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    capi_keys().lock().unwrap().insert(h, k);
    std::ptr::write_unaligned(ph_key, h);
    TRUE
}

pub unsafe extern "win64" fn CryptGetUserKey(
    h_prov: usize,
    dw_key_spec: u32,
    ph_user_key: *mut usize,
) -> BOOL {
    if ph_user_key.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(ph_user_key, 0);
    if !matches!(dw_key_spec, AT_KEYEXCHANGE | AT_SIGNATURE) {
        return err(NTE_BAD_TYPE, FALSE);
    }
    let stored = match load_container_user_key(h_prov, dw_key_spec) {
        Ok(Some(k)) => k,
        Ok(None) => return err(NTE_NO_KEY, FALSE),
        Err(e) => return err(e, FALSE),
    };
    let alg_id = if dw_key_spec == AT_SIGNATURE { CALG_RSA_SIGN } else { CALG_RSA_KEYX };
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    capi_keys().lock().unwrap().insert(
        h,
        CapiKey {
            alg_id,
            exportable: stored.exportable,
            permissions: capi_default_permissions(stored.exportable),
            material: CapiKeyMaterial::Rsa { key: stored.key },
        },
    );
    std::ptr::write_unaligned(ph_user_key, h);
    TRUE
}

pub unsafe extern "win64" fn CryptDuplicateKey(
    h_key: usize,
    _pdw_reserved: *const u32,
    _dw_flags: u32,
    ph_key: *mut usize,
) -> BOOL {
    if ph_key.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let mut keys = capi_keys().lock().unwrap();
    let Some(orig) = keys.get(&h_key).cloned() else {
        return err(NTE_BAD_KEY, FALSE);
    };
    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    keys.insert(h, orig);
    std::ptr::write_unaligned(ph_key, h);
    TRUE
}

pub unsafe extern "win64" fn CryptDestroyKey(h_key: usize) -> BOOL {
    if capi_keys().lock().unwrap().remove(&h_key).is_some() {
        TRUE
    } else {
        err(NTE_BAD_KEY, FALSE)
    }
}

pub unsafe extern "win64" fn CryptGetKeyParam(
    h_key: usize,
    dw_param: u32,
    pb_data: *mut u8,
    pdw_data_len: *mut u32,
    _dw_flags: u32,
) -> BOOL {
    if pdw_data_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let keys = capi_keys().lock().unwrap();
    let Some(k) = keys.get(&h_key) else {
        return err(NTE_BAD_KEY, FALSE);
    };
    if (k.permissions & CRYPT_READ) == 0 {
        return err(NTE_PERM, FALSE);
    }
    let bytes: Vec<u8> = match (dw_param, &k.material) {
        (KP_ALGID, _) => k.alg_id.to_le_bytes().to_vec(),
        (KP_PERMISSIONS, _) => k.permissions.to_le_bytes().to_vec(),
        (KP_BLOCKLEN, CapiKeyMaterial::Rsa { key, .. }) => (key.bits() as u32).to_le_bytes().to_vec(),
        (KP_KEYLEN, CapiKeyMaterial::Rsa { key, .. }) => (key.bits() as u32).to_le_bytes().to_vec(),
        (KP_BLOCKLEN, CapiKeyMaterial::Symmetric { alg, .. }) => ((alg.block_size() * 8) as u32).to_le_bytes().to_vec(),
        (KP_KEYLEN, CapiKeyMaterial::Symmetric { key, .. }) => ((key.len() * 8) as u32).to_le_bytes().to_vec(),
        (KP_IV, CapiKeyMaterial::Symmetric { iv, alg, .. }) if alg.block_size() > 0 => iv.clone(),
        (KP_SALT, CapiKeyMaterial::Symmetric { salt, .. }) => salt.clone(),
        (KP_MODE, CapiKeyMaterial::Symmetric { mode, .. }) => {
            let m = match mode {
                BcCipherMode::Ecb => CRYPT_MODE_ECB,
                BcCipherMode::Cfb => CRYPT_MODE_CFB,
                _ => CRYPT_MODE_CBC,
            };
            m.to_le_bytes().to_vec()
        }
        (KP_PADDING, CapiKeyMaterial::Symmetric { padding, .. }) => padding.to_le_bytes().to_vec(),
        (KP_MODE_BITS, CapiKeyMaterial::Symmetric { msg_block_len, .. }) => (*msg_block_len * 8).to_le_bytes().to_vec(),
        (KP_EFFECTIVE_KEYLEN, CapiKeyMaterial::Symmetric { alg: SymAlg::Rc2, effective_key_len, key, .. }) => {
            let v = if *effective_key_len == 0 { (key.len() * 8) as u32 } else { *effective_key_len };
            v.to_le_bytes().to_vec()
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
        return err(ERROR_MORE_DATA, FALSE);
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), pb_data, bytes.len());
    TRUE
}

pub unsafe extern "win64" fn CryptSetKeyParam(
    h_key: usize,
    dw_param: u32,
    pb_data: *const u8,
    _dw_flags: u32,
) -> BOOL {
    if pb_data.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let mut keys = capi_keys().lock().unwrap();
    let Some(k) = keys.get_mut(&h_key) else {
        return err(NTE_BAD_KEY, FALSE);
    };
    if (k.permissions & CRYPT_WRITE) == 0 {
        return err(NTE_PERM, FALSE);
    }
    if dw_param == KP_PERMISSIONS {
        let new_perms = std::ptr::read_unaligned(pb_data as *const u32);
        // Windows CAPI only permits restricting existing key permissions, never adding new ones.
        if (new_perms & !k.permissions) != 0 {
            return err(NTE_BAD_FLAGS, FALSE);
        }
        k.permissions = new_perms;
        return TRUE;
    }
    let CapiKeyMaterial::Symmetric {
        alg,
        ref mut salt,
        ref mut mode,
        ref mut padding,
        ref mut effective_key_len,
        ref mut iv,
        ref mut running_iv,
        ref mut msg_block_len,
        ref mut rc4,
        ..
    } = k.material
    else {
        return err(NTE_BAD_TYPE, FALSE);
    };
    match dw_param {
        KP_IV => {
            let bs = alg.block_size();
            if bs == 0 {
                return err(NTE_BAD_TYPE, FALSE);
            }
            let slice = std::slice::from_raw_parts(pb_data, bs);
            *iv = slice.to_vec();
            *running_iv = slice.to_vec();
            TRUE
        }
        KP_SALT => {
            if !matches!(alg, SymAlg::Rc2 | SymAlg::Rc4) {
                return err(NTE_BAD_TYPE, FALSE);
            }
            let len = if salt.is_empty() { 11 } else { salt.len() };
            *salt = std::slice::from_raw_parts(pb_data, len).to_vec();
            *rc4 = None;
            TRUE
        }
        KP_SALT_EX => {
            if !matches!(alg, SymAlg::Rc2 | SymAlg::Rc4) {
                return err(NTE_BAD_TYPE, FALSE);
            }
            // CRYPT_DATA_BLOB on x64: cbData (u32) at offset 0, pbData (*const u8) at offset 8.
            let cb_salt = std::ptr::read_unaligned(pb_data as *const u32) as usize;
            let salt_ptr = std::ptr::read_unaligned(pb_data.add(8) as *const *const u8);
            if cb_salt > 24 || (cb_salt > 0 && salt_ptr.is_null()) {
                return err(NTE_BAD_DATA, FALSE);
            }
            *salt = if cb_salt == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(salt_ptr, cb_salt).to_vec()
            };
            *rc4 = None;
            TRUE
        }
        KP_MODE => {
            let m = std::ptr::read_unaligned(pb_data as *const u32);
            match m {
                CRYPT_MODE_CBC => *mode = BcCipherMode::Cbc,
                CRYPT_MODE_ECB => *mode = BcCipherMode::Ecb,
                CRYPT_MODE_CFB => *mode = BcCipherMode::Cfb,
                _ => return err(NTE_BAD_DATA, FALSE),
            }
            TRUE
        }
        KP_PADDING => {
            let p = std::ptr::read_unaligned(pb_data as *const u32);
            // Microsoft Learn (CryptSetKeyParam): Microsoft-supplied CSPs only support PKCS5_PADDING.
            if p != PKCS5_PADDING {
                return err(NTE_BAD_DATA, FALSE);
            }
            *padding = p;
            TRUE
        }
        KP_MODE_BITS => {
            let bits = std::ptr::read_unaligned(pb_data as *const u32);
            let bs = alg.block_size() as u32;
            if bits == 0 || bits % 8 != 0 || bits / 8 > bs {
                return err(NTE_BAD_DATA, FALSE);
            }
            *msg_block_len = bits / 8;
            TRUE
        }
        KP_EFFECTIVE_KEYLEN if alg == SymAlg::Rc2 => {
            let bits = std::ptr::read_unaligned(pb_data as *const u32);
            if !(40..=1024).contains(&bits) {
                return err(NTE_BAD_DATA, FALSE);
            }
            *effective_key_len = bits;
            TRUE
        }
        _ => err(NTE_BAD_TYPE, FALSE),
    }
}

pub unsafe extern "win64" fn CryptExportKey(
    h_key: usize,
    h_exp_key: usize,
    dw_blob_type: u32,
    dw_flags: u32,
    pb_data: *mut u8,
    pdw_data_len: *mut u32,
) -> BOOL {
    if pdw_data_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let keys = capi_keys().lock().unwrap();
    let Some(k) = keys.get(&h_key) else {
        return err(NTE_BAD_KEY, FALSE);
    };
    let blob: Vec<u8> = match (dw_blob_type, &k.material) {
        (PUBLICKEYBLOB, CapiKeyMaterial::Rsa { key, .. }) => {
            let Some(b) = key.to_capi_public_blob(k.alg_id) else {
                return err(NTE_BAD_KEY, FALSE);
            };
            b
        }
        (PRIVATEKEYBLOB, CapiKeyMaterial::Rsa { key, .. }) => {
            if !k.exportable {
                return err(NTE_BAD_KEY_STATE, FALSE);
            }
            if (k.permissions & CRYPT_EXPORT) == 0 {
                return err(NTE_PERM, FALSE);
            }
            let Some(plain) = key.to_capi_private_blob(k.alg_id) else {
                return err(NTE_BAD_KEY_STATE, FALSE);
            };
            if h_exp_key == 0 {
                plain
            } else {
                let Some(exp_k) = keys.get(&h_exp_key) else {
                    return err(NTE_BAD_KEY, FALSE);
                };
                // Per Microsoft Learn (Private Key BLOBs), when hExpKey != 0 everything after the
                // 8-byte PUBLICKEYSTRUC/BLOBHEADER is encrypted with the wrapping session key.
                let enc_body = match capi_wrap_sym_bytes(exp_k, true, &plain[8..]) {
                    Ok(v) => v,
                    Err(e) => return err(e, FALSE),
                };
                let mut out = Vec::with_capacity(8 + enc_body.len());
                out.extend_from_slice(&plain[..8]);
                out.extend_from_slice(&enc_body);
                out
            }
        }
        (PLAINTEXTKEYBLOB, CapiKeyMaterial::Symmetric { key, .. }) => {
            if h_exp_key != 0 {
                return err(NTE_BAD_KEY, FALSE);
            }
            if !k.exportable {
                return err(NTE_BAD_KEY_STATE, FALSE);
            }
            if (k.permissions & CRYPT_EXPORT) == 0 {
                return err(NTE_PERM, FALSE);
            }
            let mut out = Vec::with_capacity(12 + key.len());
            out.push(PLAINTEXTKEYBLOB as u8);
            out.push(CUR_BLOB_VERSION);
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&k.alg_id.to_le_bytes());
            out.extend_from_slice(&(key.len() as u32).to_le_bytes());
            out.extend_from_slice(key);
            out
        }
        (SIMPLEBLOB, CapiKeyMaterial::Symmetric { key: sym_bytes, .. }) => {
            if !k.exportable {
                return err(NTE_BAD_KEY_STATE, FALSE);
            }
            if (k.permissions & CRYPT_EXPORT) == 0 {
                return err(NTE_PERM, FALSE);
            }
            let Some(exp_k) = keys.get(&h_exp_key) else {
                return err(NTE_BAD_KEY, FALSE);
            };
            if (exp_k.permissions & (CRYPT_ENCRYPT | CRYPT_EXPORT_KEY)) != (CRYPT_ENCRYPT | CRYPT_EXPORT_KEY) {
                return err(NTE_PERM, FALSE);
            }
            let CapiKeyMaterial::Rsa { key: ref rsa, .. } = exp_k.material else {
                return err(NTE_BAD_KEY, FALSE);
            };
            let ct_be = if (dw_flags & CRYPT_OAEP) != 0 {
                rsa.encrypt_oaep(BcDigest::Sha1, &[], sym_bytes)
            } else {
                rsa.encrypt_pkcs1(sym_bytes)
            };
            let Ok(mut ct) = ct_be else {
                return err(NTE_BAD_DATA, FALSE);
            };
            // CAPI SIMPLEBLOB stores RSA ciphertext in little-endian byte order.
            ct.reverse();
            let mut out = Vec::with_capacity(12 + ct.len());
            out.push(SIMPLEBLOB as u8);
            out.push(CUR_BLOB_VERSION);
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&k.alg_id.to_le_bytes());
            out.extend_from_slice(&CALG_RSA_KEYX.to_le_bytes());
            out.extend_from_slice(&ct);
            out
        }
        _ => return err(NTE_BAD_TYPE, FALSE),
    };

    let need = blob.len() as u32;
    if pb_data.is_null() {
        std::ptr::write_unaligned(pdw_data_len, need);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pdw_data_len);
    std::ptr::write_unaligned(pdw_data_len, need);
    if cap < need {
        return err(ERROR_MORE_DATA, FALSE);
    }
    std::ptr::copy_nonoverlapping(blob.as_ptr(), pb_data, blob.len());
    TRUE
}

pub unsafe extern "win64" fn CryptImportKey(
    h_prov: usize,
    pb_data: *const u8,
    dw_data_len: u32,
    h_pub_key: usize,
    dw_flags: u32,
    ph_key: *mut usize,
) -> BOOL {
    if pb_data.is_null() || ph_key.is_null() || dw_data_len < 8 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    std::ptr::write_unaligned(ph_key, 0);
    if !capi_providers().lock().unwrap().contains_key(&h_prov) {
        return err(NTE_BAD_UID, FALSE);
    }
    let blob = std::slice::from_raw_parts(pb_data, dw_data_len as usize);
    let b_type = blob[0] as u32;
    let b_ver = blob[1];
    if b_ver != CUR_BLOB_VERSION {
        return err(NTE_BAD_VER, FALSE);
    }
    let ai_key_alg = u32::from_le_bytes(blob[4..8].try_into().unwrap());
    let exportable = (dw_flags & CRYPT_EXPORTABLE) != 0;

    let capi_key = match b_type {
        PUBLICKEYBLOB | PRIVATEKEYBLOB => {
            let decrypted_storage: Vec<u8>;
            let parse_slice = if b_type == PRIVATEKEYBLOB && h_pub_key != 0 {
                let dec_body = {
                    let keys = capi_keys().lock().unwrap();
                    let Some(pk) = keys.get(&h_pub_key) else {
                        return err(NTE_BAD_KEY, FALSE);
                    };
                    match capi_wrap_sym_bytes(pk, false, &blob[8..]) {
                        Ok(v) => v,
                        Err(e) => return err(e, FALSE),
                    }
                };
                let mut combined = Vec::with_capacity(8 + dec_body.len());
                combined.extend_from_slice(&blob[..8]);
                combined.extend_from_slice(&dec_body);
                decrypted_storage = combined;
                decrypted_storage.as_slice()
            } else {
                blob
            };
            let Some((rsa, blob_alg, is_priv)) = RsaKey::from_capi_blob(parse_slice) else {
                return err(NTE_BAD_DATA, FALSE);
            };
            let key_spec = if blob_alg == CALG_RSA_SIGN { AT_SIGNATURE } else { AT_KEYEXCHANGE };
            if is_priv {
                if let Err(e) = store_container_user_key(h_prov, key_spec, rsa.clone(), exportable) {
                    return err(e, FALSE);
                }
            }
            let key_exportable = exportable || !is_priv;
            CapiKey {
                alg_id: if blob_alg != 0 { blob_alg } else { CALG_RSA_KEYX },
                exportable: key_exportable,
                permissions: capi_default_permissions(key_exportable),
                material: CapiKeyMaterial::Rsa { key: rsa },
            }
        }
        PLAINTEXTKEYBLOB => {
            if blob.len() < 12 {
                return err(NTE_BAD_DATA, FALSE);
            }
            let cb_key = u32::from_le_bytes(blob[8..12].try_into().unwrap()) as usize;
            if blob.len() < 12 + cb_key {
                return err(NTE_BAD_DATA, FALSE);
            }
            let Some((sym_alg, _)) = capi_alg_to_sym(ai_key_alg) else {
                return err(NTE_BAD_ALGID, FALSE);
            };
            if !capi_valid_sym_key_len(ai_key_alg, sym_alg, cb_key) {
                return err(NTE_BAD_DATA, FALSE);
            }
            let raw = blob[12..12 + cb_key].to_vec();
            let salt = match capi_salt_from_flags(sym_alg, raw.len(), dw_flags & CRYPT_NO_SALT, None) {
                Ok(s) => s,
                Err(e) => return err(e, FALSE),
            };
            capi_sym_key(ai_key_alg, sym_alg, raw, salt, exportable)
        }
        SIMPLEBLOB => {
            if blob.len() < 12 {
                return err(NTE_BAD_DATA, FALSE);
            }
            let Some((sym_alg, _)) = capi_alg_to_sym(ai_key_alg) else {
                return err(NTE_BAD_ALGID, FALSE);
            };
            let rsa = if h_pub_key == 0 {
                match load_container_user_key(h_prov, AT_KEYEXCHANGE) {
                    Ok(Some(stored)) => stored.key,
                    Ok(None) => return err(NTE_NO_KEY, FALSE),
                    Err(e) => return err(e, FALSE),
                }
            } else {
                let keys = capi_keys().lock().unwrap();
                let Some(pk) = keys.get(&h_pub_key) else {
                    return err(NTE_BAD_KEY, FALSE);
                };
                if (pk.permissions & (CRYPT_DECRYPT | CRYPT_IMPORT_KEY)) != (CRYPT_DECRYPT | CRYPT_IMPORT_KEY) {
                    return err(NTE_PERM, FALSE);
                }
                match &pk.material {
                    CapiKeyMaterial::Rsa { key, .. } => key.clone(),
                    _ => return err(NTE_BAD_KEY, FALSE),
                }
            };
            let mut ct_be = blob[12..].to_vec();
            if ct_be.len() != rsa.k() {
                return err(NTE_BAD_DATA, FALSE);
            }
            // CAPI SIMPLEBLOB ciphertext is little-endian; reverse to big-endian for RSA decryption.
            ct_be.reverse();
            let raw = if (dw_flags & CRYPT_OAEP) != 0 {
                rsa.decrypt_oaep(BcDigest::Sha1, &[], &ct_be)
            } else {
                rsa.decrypt_pkcs1(&ct_be)
            };
            let Ok(raw_key) = raw else {
                return err(NTE_BAD_DATA, FALSE);
            };
            if !capi_valid_sym_key_len(ai_key_alg, sym_alg, raw_key.len()) {
                return err(NTE_BAD_DATA, FALSE);
            }
            let salt = match capi_salt_from_flags(sym_alg, raw_key.len(), dw_flags & CRYPT_NO_SALT, None) {
                Ok(s) => s,
                Err(e) => return err(e, FALSE),
            };
            capi_sym_key(ai_key_alg, sym_alg, raw_key, salt, exportable)
        }
        _ => return err(NTE_BAD_TYPE, FALSE),
    };

    let h = NEXT_CAPI_HANDLE.fetch_add(1, Ordering::Relaxed) as usize;
    capi_keys().lock().unwrap().insert(h, capi_key);
    std::ptr::write_unaligned(ph_key, h);
    TRUE
}

pub unsafe extern "win64" fn CryptEncrypt(
    h_key: usize,
    h_hash: usize,
    final_block: BOOL,
    dw_flags: u32,
    pb_data: *mut u8,
    pdw_data_len: *mut u32,
    dw_buf_len: u32,
) -> BOOL {
    if pdw_data_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let in_len = std::ptr::read_unaligned(pdw_data_len) as usize;
    let is_final = final_block != 0;

    let mut keys = capi_keys().lock().unwrap();
    let Some(k) = keys.get_mut(&h_key) else {
        return err(NTE_BAD_KEY, FALSE);
    };
    if (k.permissions & CRYPT_ENCRYPT) == 0 {
        return err(NTE_PERM, FALSE);
    }

    match &mut k.material {
        CapiKeyMaterial::Rsa { key: rsa, .. } => {
            let need = rsa.k();
            if pb_data.is_null() {
                std::ptr::write_unaligned(pdw_data_len, need as u32);
                return TRUE;
            }
            if (dw_buf_len as usize) < need {
                std::ptr::write_unaligned(pdw_data_len, need as u32);
                return err(ERROR_MORE_DATA, FALSE);
            }
            let pt = std::slice::from_raw_parts(pb_data, in_len).to_vec();
            if h_hash != 0 {
                drop(keys);
                if CryptHashData(h_hash, pt.as_ptr(), pt.len() as u32, 0) == FALSE {
                    return FALSE;
                }
                keys = capi_keys().lock().unwrap();
            }
            let Some(k) = keys.get(&h_key) else {
                return err(NTE_BAD_KEY, FALSE);
            };
            let CapiKeyMaterial::Rsa { key: rsa, .. } = &k.material else {
                return err(NTE_BAD_KEY, FALSE);
            };
            let ct = if (dw_flags & CRYPT_OAEP) != 0 {
                rsa.encrypt_oaep(BcDigest::Sha1, &[], &pt)
            } else {
                rsa.encrypt_pkcs1(&pt)
            };
            let Ok(mut ct) = ct else {
                return err(NTE_BAD_LEN, FALSE);
            };
            // Legacy CAPI RSA ciphertext is little-endian.
            ct.reverse();
            std::ptr::copy_nonoverlapping(ct.as_ptr(), pb_data, ct.len());
            std::ptr::write_unaligned(pdw_data_len, ct.len() as u32);
            TRUE
        }
        CapiKeyMaterial::Symmetric {
            alg,
            key,
            salt,
            mode,
            padding,
            effective_key_len,
            iv,
            running_iv,
            msg_block_len,
            rc4,
        } => {
            let bs = alg.block_size();
            let use_pkcs7 = *alg != SymAlg::Rc4 && is_final && *padding == PKCS5_PADDING;
            let need = if *alg == SymAlg::Rc4 {
                in_len
            } else if use_pkcs7 {
                ((in_len / bs) + 1) * bs
            } else if *mode == BcCipherMode::Cfb {
                in_len
            } else {
                if in_len % bs != 0 {
                    return err(NTE_BAD_DATA, FALSE);
                }
                in_len
            };
            if pb_data.is_null() {
                std::ptr::write_unaligned(pdw_data_len, need as u32);
                return TRUE;
            }
            if (dw_buf_len as usize) < need {
                std::ptr::write_unaligned(pdw_data_len, need as u32);
                return err(ERROR_MORE_DATA, FALSE);
            }
            let pt = if in_len == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(pb_data, in_len).to_vec()
            };

            let eff_key = capi_effective_sym_key(*alg, key, salt);
            let out = if *alg == SymAlg::Rc4 {
                let st = rc4.get_or_insert_with(|| Rc4State::new(&eff_key));
                let r = st.apply(&pt);
                if is_final {
                    *rc4 = Some(Rc4State::new(&eff_key));
                }
                r
            } else {
                let mut iv_work = running_iv.clone();
                let (cfb_padded, call_pkcs7);
                let input_slice: &[u8] = if *mode == BcCipherMode::Cfb && use_pkcs7 {
                    cfb_padded = capi_pkcs5_pad(&pt, bs);
                    call_pkcs7 = false;
                    &cfb_padded
                } else {
                    call_pkcs7 = use_pkcs7;
                    &pt
                };
                let res = crate::misc::bcrypt::sym_crypt_block_mode(
                    *alg,
                    *mode,
                    true,
                    call_pkcs7,
                    &eff_key,
                    *effective_key_len,
                    &mut iv_work,
                    *msg_block_len as usize,
                    input_slice,
                );
                let Ok(v) = res else {
                    return err(NTE_BAD_DATA, FALSE);
                };
                if is_final {
                    *running_iv = iv.clone();
                } else {
                    *running_iv = iv_work;
                }
                v
            };

            drop(keys);
            if h_hash != 0 && !pt.is_empty() && CryptHashData(h_hash, pt.as_ptr(), pt.len() as u32, 0) == FALSE {
                return FALSE;
            }
            if !out.is_empty() {
                std::ptr::copy_nonoverlapping(out.as_ptr(), pb_data, out.len());
            }
            std::ptr::write_unaligned(pdw_data_len, out.len() as u32);
            TRUE
        }
    }
}

pub unsafe extern "win64" fn CryptDecrypt(
    h_key: usize,
    h_hash: usize,
    final_block: BOOL,
    dw_flags: u32,
    pb_data: *mut u8,
    pdw_data_len: *mut u32,
) -> BOOL {
    if pb_data.is_null() || pdw_data_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let in_len = std::ptr::read_unaligned(pdw_data_len) as usize;
    let is_final = final_block != 0;

    let mut keys = capi_keys().lock().unwrap();
    let Some(k) = keys.get_mut(&h_key) else {
        return err(NTE_BAD_KEY, FALSE);
    };
    if (k.permissions & CRYPT_DECRYPT) == 0 {
        return err(NTE_PERM, FALSE);
    }

    let out = match &mut k.material {
        CapiKeyMaterial::Rsa { key: rsa, .. } => {
            if !rsa.has_private() {
                return err(NTE_BAD_KEY, FALSE);
            }
            if in_len != rsa.k() {
                return err(NTE_BAD_DATA, FALSE);
            }
            let mut ct_be = std::slice::from_raw_parts(pb_data, in_len).to_vec();
            ct_be.reverse();
            let res = if (dw_flags & CRYPT_OAEP) != 0 {
                rsa.decrypt_oaep(BcDigest::Sha1, &[], &ct_be)
            } else {
                rsa.decrypt_pkcs1(&ct_be)
            };
            match res {
                Ok(v) => v,
                Err(_) => return err(NTE_BAD_DATA, FALSE),
            }
        }
        CapiKeyMaterial::Symmetric {
            alg,
            key,
            salt,
            mode,
            padding,
            effective_key_len,
            iv,
            running_iv,
            msg_block_len,
            rc4,
        } => {
            let ct = if in_len == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(pb_data, in_len).to_vec()
            };
            let eff_key = capi_effective_sym_key(*alg, key, salt);
            if *alg == SymAlg::Rc4 {
                let st = rc4.get_or_insert_with(|| Rc4State::new(&eff_key));
                let r = st.apply(&ct);
                if is_final {
                    *rc4 = Some(Rc4State::new(&eff_key));
                }
                r
            } else {
                let bs = alg.block_size();
                let use_pkcs7 = is_final && *padding == PKCS5_PADDING;
                if (*mode != BcCipherMode::Cfb || use_pkcs7) && ct.len() % bs != 0 {
                    return err(NTE_BAD_DATA, FALSE);
                }
                let mut iv_work = running_iv.clone();
                let call_pkcs7 = use_pkcs7 && *mode != BcCipherMode::Cfb;
                let res = crate::misc::bcrypt::sym_crypt_block_mode(
                    *alg,
                    *mode,
                    false,
                    call_pkcs7,
                    &eff_key,
                    *effective_key_len,
                    &mut iv_work,
                    *msg_block_len as usize,
                    &ct,
                );
                let Ok(v) = res else {
                    return err(NTE_BAD_DATA, FALSE);
                };
                let v = if *mode == BcCipherMode::Cfb && use_pkcs7 {
                    match capi_pkcs5_unpad(v, bs) {
                        Ok(u) => u,
                        Err(e) => return err(e, FALSE),
                    }
                } else {
                    v
                };
                if is_final {
                    *running_iv = iv.clone();
                } else {
                    *running_iv = iv_work;
                }
                v
            }
        }
    };

    drop(keys);
    if !out.is_empty() {
        std::ptr::copy_nonoverlapping(out.as_ptr(), pb_data, out.len());
    }
    std::ptr::write_unaligned(pdw_data_len, out.len() as u32);
    if h_hash != 0 && !out.is_empty() && CryptHashData(h_hash, out.as_ptr(), out.len() as u32, 0) == FALSE {
        return FALSE;
    }
    TRUE
}

unsafe fn crypt_sign_hash_impl(
    h_hash: usize,
    dw_key_spec: u32,
    dw_flags: u32,
    pb_signature: *mut u8,
    pdw_sig_len: *mut u32,
) -> BOOL {
    if pdw_sig_len.is_null() {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    if !matches!(dw_key_spec, AT_KEYEXCHANGE | AT_SIGNATURE) {
        return err(NTE_BAD_TYPE, FALSE);
    }
    let (h_prov, digest, hash_val) = {
        let hashes = capi_hashes().lock().unwrap();
        let Some(h) = hashes.get(&h_hash) else {
            return err(NTE_BAD_HASH, FALSE);
        };
        (h.h_prov, h.digest, capi_hash_value(h))
    };
    let stored = match load_container_user_key(h_prov, dw_key_spec) {
        Ok(Some(k)) => k,
        Ok(None) => return err(NTE_NO_KEY, FALSE),
        Err(e) => return err(e, FALSE),
    };
    let rsa = stored.key;
    let sig_len = rsa.k() as u32;
    if pb_signature.is_null() {
        std::ptr::write_unaligned(pdw_sig_len, sig_len);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pdw_sig_len);
    std::ptr::write_unaligned(pdw_sig_len, sig_len);
    if cap < sig_len {
        return err(ERROR_MORE_DATA, FALSE);
    }
    let d_opt = if (dw_flags & CRYPT_NOHASHOID) != 0 { None } else { Some(digest) };
    let Ok(mut sig) = rsa.sign_pkcs1(d_opt, &hash_val) else {
        return err(NTE_BAD_HASH, FALSE);
    };
    // Legacy CAPI RSA signatures are little-endian.
    sig.reverse();
    std::ptr::copy_nonoverlapping(sig.as_ptr(), pb_signature, sig.len());
    TRUE
}

pub unsafe extern "win64" fn CryptSignHashW(
    h_hash: usize,
    dw_key_spec: u32,
    _sz_description: LPCWSTR,
    dw_flags: u32,
    pb_signature: *mut u8,
    pdw_sig_len: *mut u32,
) -> BOOL {
    crypt_sign_hash_impl(h_hash, dw_key_spec, dw_flags, pb_signature, pdw_sig_len)
}

pub unsafe extern "win64" fn CryptSignHashA(
    h_hash: usize,
    dw_key_spec: u32,
    _sz_description: LPCSTR,
    dw_flags: u32,
    pb_signature: *mut u8,
    pdw_sig_len: *mut u32,
) -> BOOL {
    crypt_sign_hash_impl(h_hash, dw_key_spec, dw_flags, pb_signature, pdw_sig_len)
}

unsafe fn crypt_verify_signature_impl(
    h_hash: usize,
    pb_signature: *const u8,
    dw_sig_len: u32,
    h_pub_key: usize,
    dw_flags: u32,
) -> BOOL {
    if pb_signature.is_null() || dw_sig_len == 0 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    }
    let (digest, hash_val) = {
        let hashes = capi_hashes().lock().unwrap();
        let Some(h) = hashes.get(&h_hash) else {
            return err(NTE_BAD_HASH, FALSE);
        };
        (h.digest, capi_hash_value(h))
    };
    let rsa = {
        let keys = capi_keys().lock().unwrap();
        let Some(k) = keys.get(&h_pub_key) else {
            return err(NTE_BAD_KEY, FALSE);
        };
        match &k.material {
            CapiKeyMaterial::Rsa { key, .. } => key.clone(),
            _ => return err(NTE_BAD_KEY, FALSE),
        }
    };
    if dw_sig_len as usize != rsa.k() {
        return err(NTE_BAD_SIGNATURE, FALSE);
    }
    let mut sig_be = std::slice::from_raw_parts(pb_signature, dw_sig_len as usize).to_vec();
    sig_be.reverse();
    let d_opt = if (dw_flags & CRYPT_NOHASHOID) != 0 { None } else { Some(digest) };
    if rsa.verify_pkcs1(d_opt, &hash_val, &sig_be) {
        TRUE
    } else {
        err(NTE_BAD_SIGNATURE, FALSE)
    }
}

pub unsafe extern "win64" fn CryptVerifySignatureW(
    h_hash: usize,
    pb_signature: *const u8,
    dw_sig_len: u32,
    h_pub_key: usize,
    _sz_description: LPCWSTR,
    dw_flags: u32,
) -> BOOL {
    crypt_verify_signature_impl(h_hash, pb_signature, dw_sig_len, h_pub_key, dw_flags)
}

pub unsafe extern "win64" fn CryptVerifySignatureA(
    h_hash: usize,
    pb_signature: *const u8,
    dw_sig_len: u32,
    h_pub_key: usize,
    _sz_description: LPCSTR,
    dw_flags: u32,
) -> BOOL {
    crypt_verify_signature_impl(h_hash, pb_signature, dw_sig_len, h_pub_key, dw_flags)
}

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

/// Windows x64 `CREDENTIAL_ATTRIBUTEW` (24 bytes).
#[repr(C)]
struct CredentialAttributeW {
    keyword: *mut u16,
    flags: u32,
    value_size: u32,
    value: *mut u8,
}
const _: () = assert!(std::mem::size_of::<CredentialAttributeW>() == 24);

#[derive(Clone)]
struct StoredCredAttr {
    keyword: String,
    flags: u32,
    value: Vec<u8>,
}

#[derive(Clone)]
struct StoredCredential {
    flags: u32,
    cred_type: u32,
    target_name: String,
    comment: Option<String>,
    last_written: FILETIME,
    blob: Vec<u8>,
    persist: u32,
    attributes: Vec<StoredCredAttr>,
    target_alias: Option<String>,
    user_name: Option<String>,
}

const CRED_PERSIST_SESSION: u32 = 1;
const CRED_PERSIST_LOCAL_MACHINE: u32 = 2;
const CRED_PERSIST_ENTERPRISE: u32 = 3;
const CRED_MAX_ATTRIBUTES: u32 = 64;
const CRED_VAULT_MAGIC_V1: &[u8; 8] = b"PFCRED01";
const CRED_VAULT_MAGIC: &[u8; 8] = b"PFCRED02";
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
        out.extend_from_slice(&(entry.attributes.len() as u32).to_le_bytes());
        for attr in &entry.attributes {
            push_bytes(&mut out, attr.keyword.as_bytes());
            out.extend_from_slice(&attr.flags.to_le_bytes());
            push_bytes(&mut out, &attr.value);
        }
    }
    out
}

fn decode_cred_vault(data: &[u8]) -> Option<std::collections::HashMap<(String, u32), StoredCredential>> {
    if data.len() < 12 {
        return None;
    }
    let has_attrs = if &data[..8] == CRED_VAULT_MAGIC {
        true
    } else if &data[..8] == CRED_VAULT_MAGIC_V1 {
        false
    } else {
        return None;
    };
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
        let attributes = if has_attrs {
            let acount = read_u32(&mut pos)? as usize;
            let mut attrs = Vec::with_capacity(acount.min(CRED_MAX_ATTRIBUTES as usize));
            for _ in 0..acount {
                let keyword = String::from_utf8(read_bytes(&mut pos)?).ok()?;
                let aflags = read_u32(&mut pos)?;
                let value = read_bytes(&mut pos)?;
                attrs.push(StoredCredAttr {
                    keyword,
                    flags: aflags,
                    value,
                });
            }
            attrs
        } else {
            Vec::new()
        };
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
                attributes,
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

const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 0x1;

/// Lookup existing credential blob across session and persisted vaults while caller holds `CredStoreGuard`.
unsafe fn find_existing_cred_blob(key: &(String, u32)) -> Option<Vec<u8>> {
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
        return err(crate::types::err::ERROR_INVALID_FLAGS, FALSE);
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
    let blob = if c.credential_blob_size > 0 && !c.credential_blob.is_null() {
        std::slice::from_raw_parts(c.credential_blob, c.credential_blob_size as usize).to_vec()
    } else if c.credential_blob_size > 0 {
        return err(ERROR_INVALID_PARAMETER, FALSE);
    } else {
        Vec::new()
    };
    let mut attributes = Vec::with_capacity(c.attribute_count.min(CRED_MAX_ATTRIBUTES) as usize);
    if c.attribute_count > 0 {
        if c.attributes.is_null() || c.attribute_count > CRED_MAX_ATTRIBUTES {
            return err(ERROR_INVALID_PARAMETER, FALSE);
        }
        let attrs_ptr = c.attributes as *const CredentialAttributeW;
        for i in 0..c.attribute_count as usize {
            let a = std::ptr::read_unaligned(attrs_ptr.add(i));
            let Some(keyword) = crate::strings::from_wide_opt(a.keyword) else {
                return err(ERROR_INVALID_PARAMETER, FALSE);
            };
            let value = if a.value_size > 0 {
                if a.value.is_null() {
                    return err(ERROR_INVALID_PARAMETER, FALSE);
                }
                std::slice::from_raw_parts(a.value, a.value_size as usize).to_vec()
            } else {
                Vec::new()
            };
            attributes.push(StoredCredAttr {
                keyword,
                flags: a.flags,
                value,
            });
        }
    }
    let key = (target.to_ascii_lowercase(), c.cred_type);
    let mut entry = StoredCredential {
        flags: c.flags,
        cred_type: c.cred_type,
        target_name: target,
        comment: crate::strings::from_wide_opt(c.comment),
        last_written: FILETIME::from_u64(crate::types::now_filetime()),
        blob,
        persist: c.persist,
        attributes,
        target_alias: crate::strings::from_wide_opt(c.target_alias),
        user_name: crate::strings::from_wide_opt(c.user_name),
    };

    match c.persist {
        CRED_PERSIST_SESSION => {
            let _guard = acquire_cred_store_lock_best_effort();
            if preserve_blob {
                let Some(existing_blob) = find_existing_cred_blob(&key) else {
                    return err(ERROR_NOT_FOUND, FALSE);
                };
                entry.blob = existing_blob;
            }
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
            let (target_kind, other_kind) = if c.persist == CRED_PERSIST_LOCAL_MACHINE {
                (CredVaultKind::LocalMachine, CredVaultKind::Enterprise)
            } else {
                (CredVaultKind::Enterprise, CredVaultKind::LocalMachine)
            };
            let mut map = match load_persisted_vault(target_kind) {
                Ok(m) => m,
                Err(_) => return err(ERROR_WRITE_FAULT_CRED, FALSE),
            };
            if preserve_blob {
                let existing_blob = map
                    .get(&key)
                    .map(|e| e.blob.clone())
                    .or_else(|| {
                        session_cred_store()
                            .lock()
                            .unwrap()
                            .get(&key)
                            .map(|e| e.blob.clone())
                    })
                    .or_else(|| {
                        load_persisted_vault(other_kind)
                            .ok()
                            .and_then(|m| m.get(&key).map(|e| e.blob.clone()))
                    });
                let Some(existing_blob) = existing_blob else {
                    return err(ERROR_NOT_FOUND, FALSE);
                };
                entry.blob = existing_blob;
            }
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

    // Pack `CredentialW` + `CredentialAttributeW[]` + strings + blobs into one heap allocation for `CredFree`.
    let target_w = crate::strings::to_wide_z(&e.target_name);
    let comment_w = e.comment.as_deref().map(crate::strings::to_wide_z);
    let alias_w = e.target_alias.as_deref().map(crate::strings::to_wide_z);
    let user_w = e.user_name.as_deref().map(crate::strings::to_wide_z);
    let attr_keywords_w: Vec<Vec<u16>> = e
        .attributes
        .iter()
        .map(|a| crate::strings::to_wide_z(&a.keyword))
        .collect();

    let mut total = std::mem::size_of::<CredentialW>();
    let attrs_off = total;
    total += e.attributes.len() * std::mem::size_of::<CredentialAttributeW>();
    let target_off = total;
    total += target_w.len() * 2;
    let comment_off = comment_w.as_ref().map(|w| { let o = total; total += w.len() * 2; o });
    let alias_off = alias_w.as_ref().map(|w| { let o = total; total += w.len() * 2; o });
    let user_off = user_w.as_ref().map(|w| { let o = total; total += w.len() * 2; o });
    let mut attr_kw_offs = Vec::with_capacity(e.attributes.len());
    for kw in &attr_keywords_w {
        attr_kw_offs.push(total);
        total += kw.len() * 2;
    }
    let blob_off = total;
    total += e.blob.len();
    let mut attr_val_offs = Vec::with_capacity(e.attributes.len());
    for a in &e.attributes {
        attr_val_offs.push(total);
        total += a.value.len();
    }

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
    let attrs_ptr = if e.attributes.is_empty() {
        std::ptr::null_mut()
    } else {
        let arr = base.add(attrs_off) as *mut CredentialAttributeW;
        for (i, attr) in e.attributes.iter().enumerate() {
            let kw_ptr = write_w(attr_kw_offs[i], &attr_keywords_w[i]);
            let val_ptr = if attr.value.is_empty() {
                std::ptr::null_mut()
            } else {
                let p = base.add(attr_val_offs[i]);
                std::ptr::copy_nonoverlapping(attr.value.as_ptr(), p, attr.value.len());
                p
            };
            std::ptr::write_unaligned(
                arr.add(i),
                CredentialAttributeW {
                    keyword: kw_ptr,
                    flags: attr.flags,
                    value_size: attr.value.len() as u32,
                    value: val_ptr,
                },
            );
        }
        arr as *mut c_void
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
            attribute_count: e.attributes.len() as u32,
            attributes: attrs_ptr,
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
            assert_eq!(CryptHashData(hhash, b"abc".as_ptr(), 3, 0), TRUE);

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

            // CRED_PRESERVE_CREDENTIAL_BLOB: rejects unknown flags, rejects non-zero blob,
            // returns ERROR_NOT_FOUND for absent credentials, and preserves existing secret on metadata update.
            assert_eq!(CredWriteW(&c_local as *const _ as *const c_void, 0x2), FALSE);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_INVALID_FLAGS);
            assert_eq!(
                CredWriteW(&c_local as *const _ as *const c_void, CRED_PRESERVE_CREDENTIAL_BLOB),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_PARAMETER);

            let target_missing = crate::strings::to_wide_z("PortFactory/Missing");
            let c_missing = make_cred(&target_missing, &user, &[], CRED_PERSIST_LOCAL_MACHINE);
            assert_eq!(
                CredWriteW(&c_missing as *const _ as *const c_void, CRED_PRESERVE_CREDENTIAL_BLOB),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_NOT_FOUND);

            let user_updated = crate::strings::to_wide_z("alice-updated");
            let c_local_meta = make_cred(&target_local, &user_updated, &[], CRED_PERSIST_LOCAL_MACHINE);
            assert_eq!(
                CredWriteW(&c_local_meta as *const _ as *const c_void, CRED_PRESERVE_CREDENTIAL_BLOB),
                TRUE
            );
            assert_eq!(CredReadW(target_local.as_ptr(), 1, 0, &mut out), TRUE);
            let got_meta = std::ptr::read_unaligned(out as *const CredentialW);
            assert_eq!(got_meta.credential_blob_size as usize, b"local-blob".len());
            assert_eq!(
                std::slice::from_raw_parts(got_meta.credential_blob, got_meta.credential_blob_size as usize),
                b"local-blob"
            );
            assert_eq!(crate::strings::from_wide(got_meta.user_name), "alice-updated");
            CredFree(out);

            // Restore username "alice" and attach a credential attribute to verify roundtrip persistence.
            let attr_kw = crate::strings::to_wide_z("PortFactory:Scope");
            let attr_val = b"game-launch-token";
            let mut attr_in = CredentialAttributeW {
                keyword: attr_kw.as_ptr() as *mut u16,
                flags: 0,
                value_size: attr_val.len() as u32,
                value: attr_val.as_ptr() as *mut u8,
            };
            let mut c_local_restore = make_cred(&target_local, &user, &[], CRED_PERSIST_LOCAL_MACHINE);
            c_local_restore.attribute_count = 1;
            c_local_restore.attributes = &mut attr_in as *mut _ as *mut c_void;
            assert_eq!(
                CredWriteW(&c_local_restore as *const _ as *const c_void, CRED_PRESERVE_CREDENTIAL_BLOB),
                TRUE
            );
            assert_eq!(CredReadW(target_local.as_ptr(), 1, 0, &mut out), TRUE);
            let got_attr_cred = std::ptr::read_unaligned(out as *const CredentialW);
            assert_eq!(got_attr_cred.attribute_count, 1);
            assert!(!got_attr_cred.attributes.is_null());
            let got_attr = std::ptr::read_unaligned(got_attr_cred.attributes as *const CredentialAttributeW);
            assert_eq!(crate::strings::from_wide(got_attr.keyword), "PortFactory:Scope");
            assert_eq!(got_attr.value_size as usize, attr_val.len());
            assert_eq!(std::slice::from_raw_parts(got_attr.value, got_attr.value_size as usize), attr_val);
            CredFree(out);

            for (t, expected) in [(&target_local, &b"local-blob"[..]), (&target_ent, &b"ent-blob"[..])] {
                assert_eq!(CredReadW(t.as_ptr(), 1, 0, &mut out), TRUE);
                assert!(!out.is_null());
                let got = std::ptr::read_unaligned(out as *const CredentialW);
                assert_eq!(got.credential_blob_size as usize, expected.len());
                assert_eq!(std::slice::from_raw_parts(got.credential_blob, expected.len()), expected);
                assert_eq!(crate::strings::from_wide(got.user_name), "alice");
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

    #[test]
    fn token_information_all_classes_windows_semantics() {
        unsafe {
            let mut tok: HANDLE = std::ptr::null_mut();
            assert_eq!(OpenProcessToken((-1isize) as HANDLE, 0x0008 | 0x0010 | 0x0002, &mut tok), TRUE);
            assert!(!tok.is_null());

            // TOKEN_QUERY-only handle cannot be duplicated or query TokenSource (ERROR_ACCESS_DENIED)
            let mut ro_tok: HANDLE = std::ptr::null_mut();
            let mut denied_dup: HANDLE = std::ptr::null_mut();
            let mut src_buf = [0u8; 16];
            let mut src_need = 0u32;
            assert_eq!(OpenProcessToken((-1isize) as HANDLE, 0x0008, &mut ro_tok), TRUE);
            assert_eq!(DuplicateToken(ro_tok, SECURITY_IMPERSONATION, &mut denied_dup), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_ACCESS_DENIED);
            assert_eq!(GetTokenInformation(ro_tok, 7, src_buf.as_mut_ptr(), 16, &mut src_need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_ACCESS_DENIED);
            let _ = crate::kernel32::threads::CloseHandle(ro_tok);

            // TOKEN_QUERY_SOURCE-only handle can query TokenSource (7) but not TOKEN_QUERY classes
            let mut src_tok: HANDLE = std::ptr::null_mut();
            assert_eq!(OpenProcessToken((-1isize) as HANDLE, 0x0010, &mut src_tok), TRUE);
            assert_eq!(GetTokenInformation(src_tok, 7, src_buf.as_mut_ptr(), 16, &mut src_need), TRUE);
            assert_eq!(src_need, 16);
            let mut denied_val = 0u32;
            assert_eq!(GetTokenInformation(src_tok, 8, &mut denied_val as *mut u32 as *mut u8, 4, &mut src_need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_ACCESS_DENIED);
            let _ = crate::kernel32::threads::CloseHandle(src_tok);

            let mut need = 0u32;
            // TokenType (8) -> TokenPrimary (1)
            let mut val = 0u32;
            assert_eq!(GetTokenInformation(tok, 8, &mut val as *mut u32 as *mut u8, 4, &mut need), TRUE);
            assert_eq!((val, need), (TOKEN_PRIMARY, 4));

            // TokenImpersonationLevel (9) on primary token -> ERROR_INVALID_PARAMETER
            assert_eq!(GetTokenInformation(tok, 9, &mut val as *mut u32 as *mut u8, 4, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_PARAMETER);

            // DuplicateToken creates an impersonation token whose level can be queried
            let mut imp_tok: HANDLE = std::ptr::null_mut();
            assert_eq!(DuplicateToken(tok, SECURITY_IMPERSONATION, &mut imp_tok), TRUE);
            assert_eq!(GetTokenInformation(imp_tok, 8, &mut val as *mut u32 as *mut u8, 4, &mut need), TRUE);
            assert_eq!(val, TOKEN_IMPERSONATION);
            assert_eq!(GetTokenInformation(imp_tok, 9, &mut val as *mut u32 as *mut u8, 4, &mut need), TRUE);
            assert_eq!(val, SECURITY_IMPERSONATION);

            // TokenGroups (2): size query + read groups and validate embedded SID pointers
            assert_eq!(GetTokenInformation(tok, 2, std::ptr::null_mut(), 0, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            assert!(need >= 8);
            let mut gbuf = vec![0u8; need as usize];
            assert_eq!(GetTokenInformation(tok, 2, gbuf.as_mut_ptr(), need, &mut need), TRUE);
            let gcount = u32::from_le_bytes(gbuf[0..4].try_into().unwrap());
            assert!(gcount >= 4);
            let first_sid = std::ptr::read_unaligned(gbuf.as_ptr().add(8) as *const *const u8);
            assert!((first_sid as usize) >= (gbuf.as_ptr() as usize) && (first_sid as usize) < (gbuf.as_ptr() as usize + gbuf.len()));
            assert_eq!(*first_sid, 1); // SID_REVISION

            // TokenPrivileges (3)
            assert_eq!(GetTokenInformation(tok, 3, std::ptr::null_mut(), 0, &mut need), FALSE);
            let mut pbuf = vec![0u8; need as usize];
            assert_eq!(GetTokenInformation(tok, 3, pbuf.as_mut_ptr(), need, &mut need), TRUE);
            assert_eq!(u32::from_le_bytes(pbuf[0..4].try_into().unwrap()), 1);

            // TokenOwner (4), TokenPrimaryGroup (5), TokenDefaultDacl (6), TokenSource (7)
            for cls in [4u32, 5, 6, 7, 10, 11, 12, 13, 15, 17, 18, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48] {
                let mut sz = 0u32;
                assert_eq!(GetTokenInformation(tok, cls, std::ptr::null_mut(), 0, &mut sz), FALSE, "class {cls} size query");
                assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER, "class {cls} err");
                assert!(sz > 0, "class {cls} non-zero size");
                let mut b = vec![0u8; sz as usize];
                assert_eq!(GetTokenInformation(tok, cls, b.as_mut_ptr(), sz, &mut sz), TRUE, "class {cls} read");
            }

            // TokenStatistics (10) -> 56 bytes, TokenType=1, non-zero TokenId
            let mut stats = [0u8; 56];
            assert_eq!(GetTokenInformation(tok, 10, stats.as_mut_ptr(), 56, &mut need), TRUE);
            assert_eq!(need, 56);
            assert_ne!(u64::from_le_bytes(stats[0..8].try_into().unwrap()), 0);
            assert_eq!(u32::from_le_bytes(stats[24..28].try_into().unwrap()), TOKEN_PRIMARY);

            // TokenAccessInformation (22) -> SidHash populated with user + groups + bit-sliced hash entries
            let mut ai_need = 0u32;
            assert_eq!(GetTokenInformation(tok, 22, std::ptr::null_mut(), 0, &mut ai_need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            let mut ai_buf = vec![0u8; ai_need as usize];
            assert_eq!(GetTokenInformation(tok, 22, ai_buf.as_mut_ptr(), ai_need, &mut ai_need), TRUE);
            let base = ai_buf.as_ptr() as usize;
            let end = base + ai_buf.len();
            let sid_hash_ptr = std::ptr::read_unaligned(ai_buf.as_ptr() as *const usize);
            assert!(sid_hash_ptr >= base && sid_hash_ptr + 272 <= end);
            let sh = sid_hash_ptr as *const u8;
            let mut expected_sids = vec![(token_user_sid(), 0u32)];
            expected_sids.extend(token_default_groups());
            let sid_count = std::ptr::read_unaligned(sh as *const u32) as usize;
            assert_eq!(sid_count, expected_sids.len());
            let sid_attr_ptr = std::ptr::read_unaligned(sh.add(8) as *const usize);
            assert!(sid_attr_ptr >= base && sid_attr_ptr + sid_count * 16 <= end);
            for (i, (exp_sid, exp_attrs)) in expected_sids.iter().enumerate() {
                let ent = (sid_attr_ptr as *const u8).add(i * 16);
                let sp = std::ptr::read_unaligned(ent as *const usize);
                let attrs = std::ptr::read_unaligned(ent.add(8) as *const u32);
                assert!(sp >= base && sp + exp_sid.len() <= end);
                assert_eq!(std::slice::from_raw_parts(sp as *const u8, exp_sid.len()), exp_sid.as_slice());
                assert_eq!(attrs, *exp_attrs);
                let b = sid_hash_byte(exp_sid);
                let lo = std::ptr::read_unaligned(sh.add(16 + ((b & 0x0F) as usize) * 8) as *const u64);
                let hi = std::ptr::read_unaligned(sh.add(16 + (16 + ((b >> 4) as usize)) * 8) as *const u64);
                assert_ne!((lo & hi) & (1u64 << i), 0, "SID {i} missing from SidHash");
            }
            let privs_ptr = std::ptr::read_unaligned(ai_buf.as_ptr().add(16) as *const usize);
            assert!(privs_ptr >= base && privs_ptr + 16 <= end);
            assert_eq!(std::ptr::read_unaligned(privs_ptr as *const u32), 1);

            // Documented failure classes: 14 (reserved), 16 (SeSecurityPrivilege), 19 (no linked UAC token), 99 (invalid)
            assert_eq!(GetTokenInformation(tok, 14, std::ptr::null_mut(), 0, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_PARAMETER);
            assert_eq!(GetTokenInformation(tok, 16, std::ptr::null_mut(), 0, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_PRIVILEGE_NOT_HELD);
            assert_eq!(GetTokenInformation(tok, 19, std::ptr::null_mut(), 0, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_NO_SUCH_LOGON_SESSION);
            assert_eq!(GetTokenInformation(tok, 99, std::ptr::null_mut(), 0, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_PARAMETER);

            // Windows 8+ pseudo-handles: -4 (GetCurrentProcessToken) & -6 (GetCurrentThreadEffectiveToken)
            // grant TOKEN_QUERY | TOKEN_QUERY_SOURCE, while -5 (GetCurrentThreadToken) returns ERROR_NO_TOKEN
            // when the thread is not impersonating.
            assert_eq!(GetTokenInformation((-4isize) as HANDLE, 12, &mut val as *mut u32 as *mut u8, 4, &mut need), TRUE);
            assert_eq!(val, 1);
            let mut pseudo_src = [0u8; 16];
            assert_eq!(GetTokenInformation((-4isize) as HANDLE, 7, pseudo_src.as_mut_ptr(), 16, &mut need), TRUE);
            assert_eq!(need, 16);
            assert_eq!(GetTokenInformation((-6isize) as HANDLE, 12, &mut val as *mut u32 as *mut u8, 4, &mut need), TRUE);
            assert_eq!(val, 1);
            assert_eq!(GetTokenInformation((-5isize) as HANDLE, 8, &mut val as *mut u32 as *mut u8, 4, &mut need), FALSE);
            assert_eq!(crate::teb::last_error(), 1008); // ERROR_NO_TOKEN

            let _ = crate::kernel32::threads::CloseHandle(imp_tok);
            let _ = crate::kernel32::threads::CloseHandle(tok);
        }
    }

    #[test]
    fn capi_keys_blobs_encrypt_decrypt_sign_verify_and_windows_vectors() {
        unsafe {
            let tmp_root = std::env::temp_dir().join(format!("pf_capi_test_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&tmp_root);
            std::fs::create_dir_all(&tmp_root).unwrap();
            let prev_home = std::env::var_os("TRANSLATOR_HOME");
            let prev_drive = std::env::var_os("AW2_DRIVE_C");
            std::env::set_var("TRANSLATOR_HOME", &tmp_root);
            std::env::remove_var("AW2_DRIVE_C");
            capi_containers().lock().unwrap().clear();

            let hex = |s: &str| -> Vec<u8> {
                (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
            };
            let cname = crate::strings::to_wide_z("PortFactoryCapiUnitTestContainer");
            let mut hprov: usize = 0;
            // Clean any prior container, then create fresh
            let _ = CryptAcquireContextW(&mut hprov, cname.as_ptr(), std::ptr::null(), 24, CRYPT_DELETEKEYSET);
            assert_eq!(CryptAcquireContextW(&mut hprov, cname.as_ptr(), std::ptr::null(), 24, CRYPT_NEWKEYSET), TRUE);

            // Before keygen, CryptGetUserKey returns NTE_NO_KEY
            let mut h_missing: usize = 0;
            assert_eq!(CryptGetUserKey(hprov, AT_SIGNATURE, &mut h_missing), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_NO_KEY);

            // 1. Generate 2048-bit RSA signature & exchange keys
            let mut h_sign: usize = 0;
            assert_eq!(CryptGenKey(hprov, AT_SIGNATURE, (2048 << 16) | CRYPT_EXPORTABLE, &mut h_sign), TRUE);
            let mut h_keyx: usize = 0;
            assert_eq!(CryptGenKey(hprov, AT_KEYEXCHANGE, (2048 << 16) | CRYPT_EXPORTABLE, &mut h_keyx), TRUE);

            // Close and reopen same named container -> CryptGetUserKey retrieves persisted keys!
            assert_eq!(CryptDestroyKey(h_sign), TRUE);
            assert_eq!(CryptDestroyKey(h_keyx), TRUE);
            assert_eq!(CryptReleaseContext(hprov, 0), TRUE);

            assert_eq!(CryptAcquireContextW(&mut hprov, cname.as_ptr(), std::ptr::null(), 24, 0), TRUE);
            assert_eq!(CryptGetUserKey(hprov, AT_SIGNATURE, &mut h_sign), TRUE);
            assert_eq!(CryptGetUserKey(hprov, AT_KEYEXCHANGE, &mut h_keyx), TRUE);

            // 2. Sign & verify SHA-256 hash (both standard DigestInfo and CRYPT_NOHASHOID)
            let mut h_hash: usize = 0;
            assert_eq!(CryptCreateHash(hprov, CALG_SHA_256, 0, 0, &mut h_hash), TRUE);
            assert_eq!(CryptHashData(h_hash, b"capi sign verify payload".as_ptr(), 24, 0), TRUE);

            let mut sig_len = 0u32;
            assert_eq!(CryptSignHashW(h_hash, AT_SIGNATURE, std::ptr::null(), 0, std::ptr::null_mut(), &mut sig_len), TRUE);
            assert_eq!(sig_len, 256);
            let mut sig = vec![0u8; 256];
            assert_eq!(CryptSignHashW(h_hash, AT_SIGNATURE, std::ptr::null(), 0, sig.as_mut_ptr(), &mut sig_len), TRUE);

            // Export PUBLICKEYBLOB, re-import, and verify
            let mut pub_len = 0u32;
            assert_eq!(CryptExportKey(h_sign, 0, PUBLICKEYBLOB, 0, std::ptr::null_mut(), &mut pub_len), TRUE);
            let mut pub_blob = vec![0u8; pub_len as usize];
            assert_eq!(CryptExportKey(h_sign, 0, PUBLICKEYBLOB, 0, pub_blob.as_mut_ptr(), &mut pub_len), TRUE);
            assert_eq!(pub_blob[0], PUBLICKEYBLOB as u8);
            assert_eq!(pub_blob[1], CUR_BLOB_VERSION);
            assert_eq!(&pub_blob[8..12], b"RSA1");

            let mut h_pub: usize = 0;
            assert_eq!(CryptImportKey(hprov, pub_blob.as_ptr(), pub_len, 0, 0, &mut h_pub), TRUE);
            assert_eq!(CryptVerifySignatureW(h_hash, sig.as_ptr(), sig_len, h_pub, std::ptr::null(), 0), TRUE);
            sig[0] ^= 0x80;
            assert_eq!(CryptVerifySignatureW(h_hash, sig.as_ptr(), sig_len, h_pub, std::ptr::null(), 0), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_SIGNATURE);

            // Export PRIVATEKEYBLOB & re-import into a fresh verify-context provider
            let mut priv_len = 0u32;
            assert_eq!(CryptExportKey(h_keyx, 0, PRIVATEKEYBLOB, 0, std::ptr::null_mut(), &mut priv_len), TRUE);
            let mut priv_blob = vec![0u8; priv_len as usize];
            assert_eq!(CryptExportKey(h_keyx, 0, PRIVATEKEYBLOB, 0, priv_blob.as_mut_ptr(), &mut priv_len), TRUE);
            assert_eq!(priv_blob[0], PRIVATEKEYBLOB as u8);
            assert_eq!(&priv_blob[8..12], b"RSA2");

            let mut hprov2: usize = 0;
            assert_eq!(CryptAcquireContextW(&mut hprov2, std::ptr::null(), std::ptr::null(), 24, CRYPT_VERIFYCONTEXT), TRUE);
            let mut h_keyx_imp: usize = 0;
            assert_eq!(CryptImportKey(hprov2, priv_blob.as_ptr(), priv_len, 0, CRYPT_EXPORTABLE, &mut h_keyx_imp), TRUE);

            // 3. RSA CryptEncrypt / CryptDecrypt (PKCS#1 v1.5 and CRYPT_OAEP)
            for flags in [0u32, CRYPT_OAEP] {
                let mut buf = vec![0u8; 256];
                let msg = b"RSA CAPI message";
                buf[..msg.len()].copy_from_slice(msg);
                let mut dlen = msg.len() as u32;
                assert_eq!(CryptEncrypt(h_keyx, 0, TRUE, flags, buf.as_mut_ptr(), &mut dlen, 256), TRUE);
                assert_eq!(dlen, 256);
                assert_eq!(CryptDecrypt(h_keyx_imp, 0, TRUE, flags, buf.as_mut_ptr(), &mut dlen), TRUE);
                assert_eq!(&buf[..dlen as usize], msg);
            }

            // 4. AES-128-CBC NIST SP 800-38A known-answer vector via PLAINTEXTKEYBLOB + SIMPLEBLOB wrap/unwrap
            let key128 = hex("2b7e151628aed2a6abf7158809cf4f3c");
            let mut pt_blob = Vec::new();
            pt_blob.push(PLAINTEXTKEYBLOB as u8);
            pt_blob.push(CUR_BLOB_VERSION);
            pt_blob.extend_from_slice(&0u16.to_le_bytes());
            pt_blob.extend_from_slice(&CALG_AES_128.to_le_bytes());
            pt_blob.extend_from_slice(&16u32.to_le_bytes());
            pt_blob.extend_from_slice(&key128);

            let mut h_aes: usize = 0;
            assert_eq!(CryptImportKey(hprov, pt_blob.as_ptr(), pt_blob.len() as u32, 0, CRYPT_EXPORTABLE, &mut h_aes), TRUE);

            // Export as SIMPLEBLOB (RSA-OAEP wrapped) and import back using imported RSA private key
            let mut sblob_len = 0u32;
            assert_eq!(CryptExportKey(h_aes, h_keyx, SIMPLEBLOB, CRYPT_OAEP, std::ptr::null_mut(), &mut sblob_len), TRUE);
            let mut sblob = vec![0u8; sblob_len as usize];
            assert_eq!(CryptExportKey(h_aes, h_keyx, SIMPLEBLOB, CRYPT_OAEP, sblob.as_mut_ptr(), &mut sblob_len), TRUE);
            assert_eq!(sblob[0], SIMPLEBLOB as u8);
            assert_eq!(u32::from_le_bytes(sblob[4..8].try_into().unwrap()), CALG_AES_128);
            assert_eq!(u32::from_le_bytes(sblob[8..12].try_into().unwrap()), CALG_RSA_KEYX);

            let mut h_aes_unwrapped: usize = 0;
            assert_eq!(CryptImportKey(hprov2, sblob.as_ptr(), sblob_len, h_keyx_imp, CRYPT_OAEP, &mut h_aes_unwrapped), TRUE);

            // Zero-handle SIMPLEBLOB import resolves AT_KEYEXCHANGE from hprov's container
            let mut h_aes_zero_pub: usize = 0;
            assert_eq!(CryptImportKey(hprov, sblob.as_ptr(), sblob_len, 0, CRYPT_OAEP, &mut h_aes_zero_pub), TRUE);
            let mut zbuf = vec![0u8; 32];
            zbuf[..16].copy_from_slice(&hex("6bc1bee22e409f96e93d7e117393172a"));
            let mut zlen = 16u32;
            let iv_check = hex("000102030405060708090a0b0c0d0e0f");
            assert_eq!(CryptSetKeyParam(h_aes_zero_pub, KP_IV, iv_check.as_ptr(), 0), TRUE);
            assert_eq!(CryptEncrypt(h_aes_zero_pub, 0, FALSE, 0, zbuf.as_mut_ptr(), &mut zlen, 32), TRUE);
            assert_eq!(&zbuf[..16], hex("7649abac8119b246cee98e9b12e9197d").as_slice());
            assert_eq!(CryptDestroyKey(h_aes_zero_pub), TRUE);
            // Zero-handle SIMPLEBLOB import on a provider with no AT_KEYEXCHANGE fails with NTE_NO_KEY
            let mut h_empty_prov: usize = 0;
            assert_eq!(CryptAcquireContextW(&mut h_empty_prov, std::ptr::null(), std::ptr::null(), 24, CRYPT_VERIFYCONTEXT), TRUE);
            assert_eq!(CryptImportKey(h_empty_prov, sblob.as_ptr(), sblob_len, 0, CRYPT_OAEP, &mut h_aes_zero_pub), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_NO_KEY);
            assert_eq!(CryptReleaseContext(h_empty_prov, 0), TRUE);

            // Default CAPI KP_MODE_BITS must be 8 bits (1 byte for CFB8), and ZERO_PADDING (3) is rejected
            let mut mode_bits = 0u32;
            let mut mb_len = 4u32;
            assert_eq!(CryptGetKeyParam(h_aes_unwrapped, KP_MODE_BITS, &mut mode_bits as *mut u32 as *mut u8, &mut mb_len, 0), TRUE);
            assert_eq!(mode_bits, 8);
            let zero_pad = 3u32;
            assert_eq!(CryptSetKeyParam(h_aes_unwrapped, KP_PADDING, &zero_pad as *const u32 as *const u8, 0), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_DATA);
            let pkcs5_pad = PKCS5_PADDING;
            assert_eq!(CryptSetKeyParam(h_aes_unwrapped, KP_PADDING, &pkcs5_pad as *const u32 as *const u8, 0), TRUE);

            // SIMPLEBLOB whose unwrapped session key has invalid length (0 bytes for RC2) must be rejected with NTE_BAD_DATA
            let mut empty_ct = vec![0u8; 256];
            let mut empty_len = 0u32;
            assert_eq!(CryptEncrypt(h_keyx, 0, TRUE, 0, empty_ct.as_mut_ptr(), &mut empty_len, 256), TRUE);
            let mut bad_sblob = Vec::with_capacity(12 + 256);
            bad_sblob.push(SIMPLEBLOB as u8);
            bad_sblob.push(CUR_BLOB_VERSION);
            bad_sblob.extend_from_slice(&0u16.to_le_bytes());
            bad_sblob.extend_from_slice(&CALG_RC2.to_le_bytes());
            bad_sblob.extend_from_slice(&CALG_RSA_KEYX.to_le_bytes());
            bad_sblob.extend_from_slice(&empty_ct);
            let mut h_bad_rc2 = 0usize;
            assert_eq!(CryptImportKey(hprov2, bad_sblob.as_ptr(), bad_sblob.len() as u32, h_keyx_imp, 0, &mut h_bad_rc2), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_DATA);

            let iv = hex("000102030405060708090a0b0c0d0e0f");
            assert_eq!(CryptSetKeyParam(h_aes_unwrapped, KP_IV, iv.as_ptr(), 0), TRUE);
            let pt = hex("6bc1bee22e409f96e93d7e117393172a");
            let mut buf = vec![0u8; 32];
            buf[..16].copy_from_slice(&pt);
            let mut dlen = 16u32;
            // Final=FALSE -> no padding, exact NIST SP 800-38A CBC block 1 ciphertext
            assert_eq!(CryptEncrypt(h_aes_unwrapped, 0, FALSE, 0, buf.as_mut_ptr(), &mut dlen, 32), TRUE);
            assert_eq!(dlen, 16);
            assert_eq!(&buf[..16], hex("7649abac8119b246cee98e9b12e9197d").as_slice());

            // Reset IV and test Final=TRUE with PKCS#5 padding + size query + ERROR_MORE_DATA
            assert_eq!(CryptSetKeyParam(h_aes_unwrapped, KP_IV, iv.as_ptr(), 0), TRUE);
            let msg_pad = b"Hello CAPI AES-128-CBC!";
            let mut qlen = msg_pad.len() as u32;
            assert_eq!(CryptEncrypt(h_aes_unwrapped, 0, TRUE, 0, std::ptr::null_mut(), &mut qlen, 0), TRUE);
            assert_eq!(qlen, 32);
            let mut small = vec![0u8; 24];
            let mut slen_in = msg_pad.len() as u32;
            assert_eq!(CryptEncrypt(h_aes_unwrapped, 0, TRUE, 0, small.as_mut_ptr(), &mut slen_in, 24), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_MORE_DATA);
            assert_eq!(slen_in, 32);

            let mut full = vec![0u8; 32];
            full[..msg_pad.len()].copy_from_slice(msg_pad);
            let mut flen = msg_pad.len() as u32;
            assert_eq!(CryptEncrypt(h_aes_unwrapped, 0, TRUE, 0, full.as_mut_ptr(), &mut flen, 32), TRUE);
            assert_eq!(flen, 32);
            assert_eq!(CryptDecrypt(h_aes_unwrapped, 0, TRUE, 0, full.as_mut_ptr(), &mut flen), TRUE);
            assert_eq!(&full[..flen as usize], msg_pad);

            // 4a. CRYPT_MODE_CFB final-block PKCS#5 padding (including extra block for 16-byte input)
            let mut h_cfb = 0usize;
            assert_eq!(CryptDuplicateKey(h_aes_unwrapped, std::ptr::null(), 0, &mut h_cfb), TRUE);
            let cfb_mode = CRYPT_MODE_CFB;
            assert_eq!(CryptSetKeyParam(h_cfb, KP_MODE, &cfb_mode as *const u32 as *const u8, 0), TRUE);
            assert_eq!(CryptSetKeyParam(h_cfb, KP_IV, iv.as_ptr(), 0), TRUE);
            let mut cfb_qlen = 16u32;
            assert_eq!(CryptEncrypt(h_cfb, 0, TRUE, 0, std::ptr::null_mut(), &mut cfb_qlen, 0), TRUE);
            assert_eq!(cfb_qlen, 32, "16-byte final AES-CFB plaintext must pad to 32 bytes in CAPI");
            let mut cfb_buf = vec![0u8; 32];
            cfb_buf[..16].copy_from_slice(&pt);
            let mut cfb_len = 16u32;
            assert_eq!(CryptEncrypt(h_cfb, 0, TRUE, 0, cfb_buf.as_mut_ptr(), &mut cfb_len, 32), TRUE);
            assert_eq!(cfb_len, 32);
            assert_eq!(CryptDecrypt(h_cfb, 0, TRUE, 0, cfb_buf.as_mut_ptr(), &mut cfb_len), TRUE);
            assert_eq!(cfb_len, 16);
            assert_eq!(&cfb_buf[..16], pt.as_slice());
            // Invalid padding on CFB final decrypt must fail with NTE_BAD_DATA
            let mut bad_cfb = vec![0xFFu8; 16];
            let mut bad_cfb_len = 16u32;
            assert_eq!(CryptDecrypt(h_cfb, 0, TRUE, 0, bad_cfb.as_mut_ptr(), &mut bad_cfb_len), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_DATA);
            assert_eq!(CryptDestroyKey(h_cfb), TRUE);

            // 4b. Encrypted PRIVATEKEYBLOB export/import with session wrapping key (hExpKey != 0)
            let mut enc_priv_len = 0u32;
            assert_eq!(CryptExportKey(h_keyx, h_aes, PRIVATEKEYBLOB, 0, std::ptr::null_mut(), &mut enc_priv_len), TRUE);
            assert!(enc_priv_len > priv_len, "CBC-encrypted PRIVATEKEYBLOB should include PKCS#5 padding");
            let mut enc_priv_blob = vec![0u8; enc_priv_len as usize];
            assert_eq!(CryptExportKey(h_keyx, h_aes, PRIVATEKEYBLOB, 0, enc_priv_blob.as_mut_ptr(), &mut enc_priv_len), TRUE);
            assert_eq!(&enc_priv_blob[..8], &priv_blob[..8]);
            assert_ne!(&enc_priv_blob[8..12], b"RSA2", "encrypted PRIVATEKEYBLOB body must not expose plaintext RSA2");
            // Importing encrypted PRIVATEKEYBLOB without wrapping key or with wrong key must fail
            let mut h_bad_priv = 0usize;
            assert_eq!(CryptImportKey(hprov2, enc_priv_blob.as_ptr(), enc_priv_len, 0, 0, &mut h_bad_priv), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_DATA);
            let mut h_wrong_wrap = 0usize;
            assert_eq!(CryptGenKey(hprov2, CALG_AES_128, 0, &mut h_wrong_wrap), TRUE);
            assert_eq!(CryptImportKey(hprov2, enc_priv_blob.as_ptr(), enc_priv_len, h_wrong_wrap, 0, &mut h_bad_priv), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_DATA);
            assert_eq!(CryptDestroyKey(h_wrong_wrap), TRUE);
            // Importing with matching wrapping key succeeds and decrypts ciphertext from original RSA key
            let mut h_keyx_wrapped_imp = 0usize;
            assert_eq!(CryptImportKey(hprov2, enc_priv_blob.as_ptr(), enc_priv_len, h_aes, 0, &mut h_keyx_wrapped_imp), TRUE);
            let mut wrap_ct = vec![0u8; 256];
            wrap_ct[..13].copy_from_slice(b"wrapped priv!");
            let mut wrap_len = 13u32;
            assert_eq!(CryptEncrypt(h_keyx, 0, TRUE, 0, wrap_ct.as_mut_ptr(), &mut wrap_len, 256), TRUE);
            assert_eq!(CryptDecrypt(h_keyx_wrapped_imp, 0, TRUE, 0, wrap_ct.as_mut_ptr(), &mut wrap_len), TRUE);
            assert_eq!(&wrap_ct[..wrap_len as usize], b"wrapped priv!");
            assert_eq!(CryptDestroyKey(h_keyx_wrapped_imp), TRUE);
            // Reject RSA key as hExpKey for PRIVATEKEYBLOB and nonzero hExpKey for PLAINTEXTKEYBLOB
            let mut dummy_len = 0u32;
            assert_eq!(CryptExportKey(h_keyx, h_sign, PRIVATEKEYBLOB, 0, std::ptr::null_mut(), &mut dummy_len), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_KEY);
            assert_eq!(CryptExportKey(h_aes, h_keyx, PLAINTEXTKEYBLOB, 0, std::ptr::null_mut(), &mut dummy_len), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_KEY);

            // 4c. KP_PERMISSIONS enforcement across export, decrypt, encrypt, read, write, and monotonic restriction
            let mut h_perm = 0usize;
            assert_eq!(CryptDuplicateKey(h_aes, std::ptr::null(), 0, &mut h_perm), TRUE);
            let mut perms = 0u32;
            let mut perms_len = 4u32;
            assert_eq!(CryptGetKeyParam(h_perm, KP_PERMISSIONS, &mut perms as *mut u32 as *mut u8, &mut perms_len, 0), TRUE);
            assert_ne!(perms & CRYPT_EXPORT, 0);
            // Remove CRYPT_EXPORT -> PLAINTEXTKEYBLOB & SIMPLEBLOB export fail with NTE_PERM; cannot re-add
            let no_export = perms & !CRYPT_EXPORT;
            assert_eq!(CryptSetKeyParam(h_perm, KP_PERMISSIONS, &no_export as *const u32 as *const u8, 0), TRUE);
            assert_eq!(CryptExportKey(h_perm, 0, PLAINTEXTKEYBLOB, 0, std::ptr::null_mut(), &mut dummy_len), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_PERM);
            assert_eq!(CryptExportKey(h_perm, h_keyx, SIMPLEBLOB, 0, std::ptr::null_mut(), &mut dummy_len), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_PERM);
            assert_eq!(CryptSetKeyParam(h_perm, KP_PERMISSIONS, &perms as *const u32 as *const u8, 0), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_FLAGS);
            // Remove CRYPT_DECRYPT -> CryptEncrypt works, CryptDecrypt fails with NTE_PERM
            let no_decrypt = no_export & !CRYPT_DECRYPT;
            assert_eq!(CryptSetKeyParam(h_perm, KP_PERMISSIONS, &no_decrypt as *const u32 as *const u8, 0), TRUE);
            let mut pbuf = vec![0u8; 32];
            pbuf[..4].copy_from_slice(b"perm");
            let mut plen = 4u32;
            assert_eq!(CryptEncrypt(h_perm, 0, TRUE, 0, pbuf.as_mut_ptr(), &mut plen, 32), TRUE);
            assert_eq!(CryptDecrypt(h_perm, 0, TRUE, 0, pbuf.as_mut_ptr(), &mut plen), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_PERM);
            // Remove CRYPT_ENCRYPT -> CryptEncrypt and wrapping PRIVATEKEYBLOB fail with NTE_PERM
            let no_encrypt = no_decrypt & !CRYPT_ENCRYPT;
            assert_eq!(CryptSetKeyParam(h_perm, KP_PERMISSIONS, &no_encrypt as *const u32 as *const u8, 0), TRUE);
            let mut plen2 = 4u32;
            assert_eq!(CryptEncrypt(h_perm, 0, TRUE, 0, pbuf.as_mut_ptr(), &mut plen2, 32), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_PERM);
            assert_eq!(CryptExportKey(h_keyx, h_perm, PRIVATEKEYBLOB, 0, std::ptr::null_mut(), &mut dummy_len), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_PERM);
            // Remove CRYPT_READ and CRYPT_WRITE -> GetKeyParam & SetKeyParam fail with NTE_PERM
            let no_rw = no_encrypt & !(CRYPT_READ | CRYPT_WRITE);
            assert_eq!(CryptSetKeyParam(h_perm, KP_PERMISSIONS, &no_rw as *const u32 as *const u8, 0), TRUE);
            assert_eq!(CryptGetKeyParam(h_perm, KP_ALGID, &mut perms as *mut u32 as *mut u8, &mut perms_len, 0), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_PERM);
            assert_eq!(CryptSetKeyParam(h_perm, KP_PERMISSIONS, &no_rw as *const u32 as *const u8, 0), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_PERM);
            assert_eq!(CryptDestroyKey(h_perm), TRUE);

            // 5. CryptDeriveKey (SHA-256 -> direct truncation; 40-bit RC4 salt semantics)
            let mut h_pw = 0usize;
            assert_eq!(CryptCreateHash(hprov2, CALG_SHA_256, 0, 0, &mut h_pw), TRUE);
            assert_eq!(CryptHashData(h_pw, b"abc".as_ptr(), 3, 0), TRUE);
            let mut h_dk = 0usize;
            assert_eq!(CryptDeriveKey(hprov2, CALG_AES_128, h_pw, CRYPT_EXPORTABLE, &mut h_dk), TRUE);
            let mut dk_blob = vec![0u8; 64];
            let mut dk_len = 64u32;
            assert_eq!(CryptExportKey(h_dk, 0, PLAINTEXTKEYBLOB, 0, dk_blob.as_mut_ptr(), &mut dk_len), TRUE);
            assert_eq!(&dk_blob[12..28], &BcDigest::Sha256.hash(b"abc")[..16]);
            assert_eq!(CryptDestroyKey(h_dk), TRUE);
            assert_eq!(CryptDestroyHash(h_pw), TRUE);

            // 5b. 40-bit RC4 CryptDeriveKey salt semantics:
            // - Default: 11 zero bytes appended to 5-byte key
            // - CRYPT_NO_SALT (0x10): 0 salt bytes (raw 5-byte RC4 key)
            // - CRYPT_CREATE_SALT (0x04): 11 salt bytes taken from unused hash bytes hash[5..16]
            let mut h_md5 = 0usize;
            assert_eq!(CryptCreateHash(hprov2, CALG_MD5, 0, 0, &mut h_md5), TRUE);
            assert_eq!(CryptHashData(h_md5, b"password".as_ptr(), 8, 0), TRUE);
            let md5_pw = BcDigest::Md5.hash(b"password");
            let rc4_sample = b"capi 40-bit salt test";

            let mut h_rc4_def = 0usize;
            let mut h_rc4_nosalt = 0usize;
            let mut h_rc4_csalt = 0usize;
            let mut h_rc4_bad = 0usize;
            assert_eq!(CryptDeriveKey(hprov2, CALG_RC4, h_md5, 40 << 16, &mut h_rc4_def), TRUE);
            assert_eq!(CryptDeriveKey(hprov2, CALG_RC4, h_md5, (40 << 16) | 0x10, &mut h_rc4_nosalt), TRUE);
            assert_eq!(
                CryptDeriveKey(hprov2, CALG_RC4, h_md5, (40 << 16) | 0x04 | CRYPT_EXPORTABLE, &mut h_rc4_csalt),
                TRUE,
            );
            assert_eq!(CryptDeriveKey(hprov2, CALG_RC4, h_md5, (40 << 16) | 0x14, &mut h_rc4_bad), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_FLAGS);

            let mut salt_buf = [0xAAu8; 16];
            let mut salt_len = 16u32;
            assert_eq!(CryptGetKeyParam(h_rc4_def, KP_SALT, salt_buf.as_mut_ptr(), &mut salt_len, 0), TRUE);
            assert_eq!(salt_len, 11);
            assert_eq!(&salt_buf[..11], &[0u8; 11]);

            salt_len = 16;
            assert_eq!(CryptGetKeyParam(h_rc4_nosalt, KP_SALT, salt_buf.as_mut_ptr(), &mut salt_len, 0), TRUE);
            assert_eq!(salt_len, 0);

            salt_len = 16;
            assert_eq!(CryptGetKeyParam(h_rc4_csalt, KP_SALT, salt_buf.as_mut_ptr(), &mut salt_len, 0), TRUE);
            assert_eq!(salt_len, 11);
            assert_eq!(&salt_buf[..11], &md5_pw[5..16]);

            let mut ct_def = rc4_sample.to_vec();
            let mut ct_nosalt = rc4_sample.to_vec();
            let mut ct_csalt = rc4_sample.to_vec();
            let mut rlen = rc4_sample.len() as u32;
            assert_eq!(CryptEncrypt(h_rc4_def, 0, TRUE, 0, ct_def.as_mut_ptr(), &mut rlen, rlen), TRUE);
            assert_eq!(CryptEncrypt(h_rc4_nosalt, 0, TRUE, 0, ct_nosalt.as_mut_ptr(), &mut rlen, rlen), TRUE);
            assert_eq!(CryptEncrypt(h_rc4_csalt, 0, TRUE, 0, ct_csalt.as_mut_ptr(), &mut rlen, rlen), TRUE);

            let mut k_def = md5_pw[..5].to_vec();
            k_def.extend_from_slice(&[0u8; 11]);
            assert_eq!(ct_def, Rc4State::new(&k_def).apply(rc4_sample));
            assert_eq!(ct_nosalt, Rc4State::new(&md5_pw[..5]).apply(rc4_sample));
            assert_eq!(ct_csalt, Rc4State::new(&md5_pw[..16]).apply(rc4_sample));
            assert_ne!(ct_def, ct_nosalt);
            assert_ne!(ct_def, ct_csalt);
            assert_ne!(ct_nosalt, ct_csalt);

            // Export 5-byte key blob, re-import, apply KP_SALT -> decrypts salted ciphertext cleanly
            let mut rc4_blob = vec![0u8; 32];
            let mut rc4_blen = 32u32;
            assert_eq!(CryptExportKey(h_rc4_csalt, 0, PLAINTEXTKEYBLOB, 0, rc4_blob.as_mut_ptr(), &mut rc4_blen), TRUE);
            assert_eq!(rc4_blen, 17);
            let mut h_rc4_imp = 0usize;
            assert_eq!(CryptImportKey(hprov2, rc4_blob.as_ptr(), rc4_blen, 0, 0, &mut h_rc4_imp), TRUE);
            assert_eq!(CryptSetKeyParam(h_rc4_imp, KP_SALT, salt_buf.as_ptr(), 0), TRUE);
            let mut dec_csalt = ct_csalt.clone();
            let mut dlen_rc4 = dec_csalt.len() as u32;
            assert_eq!(CryptDecrypt(h_rc4_imp, 0, TRUE, 0, dec_csalt.as_mut_ptr(), &mut dlen_rc4), TRUE);
            assert_eq!(&dec_csalt[..dlen_rc4 as usize], rc4_sample);

            assert_eq!(CryptDestroyKey(h_rc4_imp), TRUE);
            assert_eq!(CryptDestroyKey(h_rc4_csalt), TRUE);
            assert_eq!(CryptDestroyKey(h_rc4_nosalt), TRUE);
            assert_eq!(CryptDestroyKey(h_rc4_def), TRUE);
            assert_eq!(CryptDestroyHash(h_md5), TRUE);

            // RC2 key generation rejects >1024-bit keys (NTE_BAD_FLAGS) and roundtrips 1024-bit export/import
            let mut h_rc2_bad = 0usize;
            assert_eq!(CryptGenKey(hprov2, CALG_RC2, (2048 << 16) | CRYPT_EXPORTABLE, &mut h_rc2_bad), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_FLAGS);
            let mut h_rc2_max = 0usize;
            assert_eq!(CryptGenKey(hprov2, CALG_RC2, (1024 << 16) | CRYPT_EXPORTABLE, &mut h_rc2_max), TRUE);
            let mut rc2_blob = vec![0u8; 160];
            let mut rc2_blen = 160u32;
            assert_eq!(CryptExportKey(h_rc2_max, 0, PLAINTEXTKEYBLOB, 0, rc2_blob.as_mut_ptr(), &mut rc2_blen), TRUE);
            assert_eq!(rc2_blen, 12 + 128);
            let mut h_rc2_imp = 0usize;
            assert_eq!(CryptImportKey(hprov2, rc2_blob.as_ptr(), rc2_blen, 0, 0, &mut h_rc2_imp), TRUE);
            assert_eq!(CryptDestroyKey(h_rc2_imp), TRUE);
            assert_eq!(CryptDestroyKey(h_rc2_max), TRUE);

            // 6. Real Windows PUBLICKEYBLOB + little-endian CAPI signature interoperability vector
            // (1024-bit OpenSSL key & SHA-256 PKCS#1 v1.5 signature over "aw2 license test",
            //  converted to little-endian Windows CAPI format)
            let mut n_le = hex("CAA2D2172A1AE3E2C6EFB357E25B0C80B4CC77044032EA5AA7C8369F0C7F9786751D7FE853A0FA801EBB4CF3FBC861138952CFD50D1DFC8E5BF91E98DB4D72B369B8A0C0D4E1FC67B89F223152C9DF04049633CE988BE3050BCF92A606B0E65A2DD12E951EC0D4D4A3B1F1732255CFD81FD08164780F02A9288F6998F913A7D1");
            n_le.reverse();
            let mut win_pub_blob = Vec::with_capacity(20 + 128);
            win_pub_blob.push(PUBLICKEYBLOB as u8);
            win_pub_blob.push(CUR_BLOB_VERSION);
            win_pub_blob.extend_from_slice(&0u16.to_le_bytes());
            win_pub_blob.extend_from_slice(&CALG_RSA_SIGN.to_le_bytes());
            win_pub_blob.extend_from_slice(b"RSA1");
            win_pub_blob.extend_from_slice(&1024u32.to_le_bytes());
            win_pub_blob.extend_from_slice(&65537u32.to_le_bytes());
            win_pub_blob.extend_from_slice(&n_le);

            let mut sig_le = hex("c454e628bf702ee02de25cee7d12c50a459b587bbd50e52246fdb5ad8d4c153437fa8283cbf396dd567c1e6d5c741010f08c87d4fbd895b335e1abf882675a6a8089f1d1508583491f858399a1834536a53094208f5d6cb2e0f8a36d51a56b41e6d20a75e84e4e62cfb5020a3b41532997b7abc208275677c31e5fc3aadac2b9");
            sig_le.reverse();

            let mut h_win_pub = 0usize;
            assert_eq!(CryptImportKey(hprov2, win_pub_blob.as_ptr(), win_pub_blob.len() as u32, 0, 0, &mut h_win_pub), TRUE);
            let mut h_win_hash = 0usize;
            assert_eq!(CryptCreateHash(hprov2, CALG_SHA_256, 0, 0, &mut h_win_hash), TRUE);
            assert_eq!(CryptHashData(h_win_hash, b"aw2 license test".as_ptr(), 16, 0), TRUE);
            assert_eq!(CryptVerifySignatureW(h_win_hash, sig_le.as_ptr(), sig_le.len() as u32, h_win_pub, std::ptr::null(), 0), TRUE);

            assert_eq!(CryptDestroyHash(h_win_hash), TRUE);
            assert_eq!(CryptDestroyKey(h_win_pub), TRUE);
            assert_eq!(CryptDestroyKey(h_aes_unwrapped), TRUE);
            assert_eq!(CryptDestroyKey(h_aes), TRUE);
            assert_eq!(CryptDestroyKey(h_keyx_imp), TRUE);
            assert_eq!(CryptDestroyKey(h_pub), TRUE);
            assert_eq!(CryptDestroyHash(h_hash), TRUE);
            assert_eq!(CryptDestroyKey(h_sign), TRUE);
            assert_eq!(CryptDestroyKey(h_keyx), TRUE);
            assert_eq!(CryptReleaseContext(hprov2, 0), TRUE);
            assert_eq!(CryptReleaseContext(hprov, 0), TRUE);
            assert_eq!(CryptAcquireContextW(&mut hprov, cname.as_ptr(), std::ptr::null(), 24, CRYPT_DELETEKEYSET), TRUE);

            // 7. Persisted container survives simulated restart + preserves non-exportable policy
            let persist_name = crate::strings::to_wide_z("PortFactoryPersistedNoExport");
            let mut hp = 0usize;
            assert_eq!(CryptAcquireContextW(&mut hp, persist_name.as_ptr(), std::ptr::null(), 24, CRYPT_NEWKEYSET), TRUE);
            let mut hk_noexp = 0usize;
            // Generate WITHOUT CRYPT_EXPORTABLE
            assert_eq!(CryptGenKey(hp, AT_KEYEXCHANGE, 2048 << 16, &mut hk_noexp), TRUE);
            let mut ct = vec![0u8; 256];
            let secret_msg = b"survives restart";
            ct[..secret_msg.len()].copy_from_slice(secret_msg);
            let mut ct_len = secret_msg.len() as u32;
            assert_eq!(CryptEncrypt(hk_noexp, 0, TRUE, 0, ct.as_mut_ptr(), &mut ct_len, 256), TRUE);
            assert_eq!(CryptDestroyKey(hk_noexp), TRUE);
            assert_eq!(CryptReleaseContext(hp, 0), TRUE);

            // Verify provider-encrypted container store and secret live in host-private .capi-keystore
            // (0700 dir, 0600 files), outside guest mounts, and are NOT decryptable via guest CryptUnprotectData.
            {
                use std::os::unix::fs::PermissionsExt;
                let store_p = capi_store_path(false);
                let secret_p = capi_secret_path(false);
                let ks_dir = crate::path::capi_keystore_dir();
                assert_eq!(std::fs::metadata(&ks_dir).unwrap().permissions().mode() & 0o777, 0o700);
                assert_eq!(std::fs::metadata(&store_p).expect("persisted CAPI store must exist").permissions().mode() & 0o777, 0o600);
                assert_eq!(std::fs::metadata(&secret_p).expect("CAPI provider secret must exist").permissions().mode() & 0o777, 0o600);
                assert!(!store_p.to_string_lossy().contains("AppData"));
                assert!(!store_p.to_string_lossy().contains("ProgramData"));

                // Even if a drive letter mounts the translator root, guest VFS path resolution must refuse host-private keystore paths
                assert!(crate::path::is_host_private_keystore_path(&store_p));
                assert!(crate::path::is_host_private_keystore_path(&secret_p));
                crate::path::add_mount("K:\\", &tmp_root);
                let store_name = store_p.file_name().unwrap().to_string_lossy();
                let secret_name = secret_p.file_name().unwrap().to_string_lossy();
                assert!(crate::path::to_host(&format!("K:\\.capi-keystore\\{store_name}")).is_none(), "guest VFS must not resolve CAPI store");
                assert!(crate::path::to_host(&format!("K:\\.capi-keystore\\{secret_name}")).is_none(), "guest VFS must not resolve CAPI secret");
                assert!(crate::path::to_host("K:\\AppData").is_some(), "sibling guest mounts still resolve");

                // Even if given the raw ciphertext bytes, guest-callable CryptUnprotectData must reject them
                let enc_bytes = std::fs::read(&store_p).unwrap();
                assert!(crate::misc::crypt32::dpapi_unprotect_bytes(b"", &enc_bytes).is_none());
                assert!(crate::misc::crypt32::dpapi_unprotect_bytes(b"advapi32-capi-containers-v1", &enc_bytes).is_none());
            }

            // Machine keyset delete with the same container name MUST NOT touch the user container
            assert_eq!(
                CryptAcquireContextW(
                    &mut hp,
                    persist_name.as_ptr(),
                    std::ptr::null(),
                    24,
                    CRYPT_MACHINE_KEYSET | CRYPT_DELETEKEYSET,
                ),
                FALSE,
            );
            assert_eq!(crate::teb::last_error(), NTE_BAD_KEYSET);

            // Machine container with the same name & provider type is isolated from the user container
            let mut hp_mach = 0usize;
            assert_eq!(
                CryptAcquireContextW(
                    &mut hp_mach,
                    persist_name.as_ptr(),
                    std::ptr::null(),
                    24,
                    CRYPT_MACHINE_KEYSET | CRYPT_NEWKEYSET,
                ),
                TRUE,
            );
            let mut ks_type = 0u32;
            let mut ks_len = 4u32;
            assert_eq!(
                CryptGetProvParam(hp_mach, PP_KEYSET_TYPE, &mut ks_type as *mut u32 as *mut u8, &mut ks_len, 0),
                TRUE,
            );
            assert_eq!(ks_type, CRYPT_MACHINE_KEYSET);
            let mut hk_mach = 0usize;
            assert_eq!(CryptGenKey(hp_mach, AT_KEYEXCHANGE, 2048 << 16, &mut hk_mach), TRUE);
            let mut mach_ct = vec![0u8; 256];
            let mach_msg = b"machine scope key";
            mach_ct[..mach_msg.len()].copy_from_slice(mach_msg);
            let mut mach_ct_len = mach_msg.len() as u32;
            assert_eq!(CryptEncrypt(hk_mach, 0, TRUE, 0, mach_ct.as_mut_ptr(), &mut mach_ct_len, 256), TRUE);
            assert_eq!(CryptDestroyKey(hk_mach), TRUE);
            assert_eq!(CryptReleaseContext(hp_mach, 0), TRUE);
            {
                use std::os::unix::fs::PermissionsExt;
                let mach_store_p = capi_store_path(true);
                let meta = std::fs::metadata(&mach_store_p).expect("persisted machine CAPI store must exist");
                assert_eq!(meta.permissions().mode() & 0o777, 0o600);
            }

            // Simulate process restart by clearing in-memory container cache
            capi_containers().lock().unwrap().clear();

            // Reload machine container -> decrypts machine ciphertext, then delete machine container
            assert_eq!(
                CryptAcquireContextW(&mut hp_mach, persist_name.as_ptr(), std::ptr::null(), 24, CRYPT_MACHINE_KEYSET),
                TRUE,
            );
            let mut hk_mach_reloaded = 0usize;
            assert_eq!(CryptGetUserKey(hp_mach, AT_KEYEXCHANGE, &mut hk_mach_reloaded), TRUE);
            assert_eq!(CryptDecrypt(hk_mach_reloaded, 0, TRUE, 0, mach_ct.as_mut_ptr(), &mut mach_ct_len), TRUE);
            assert_eq!(&mach_ct[..mach_ct_len as usize], mach_msg);
            assert_eq!(CryptDestroyKey(hk_mach_reloaded), TRUE);
            assert_eq!(CryptReleaseContext(hp_mach, 0), TRUE);
            assert_eq!(
                CryptAcquireContextW(
                    &mut hp_mach,
                    persist_name.as_ptr(),
                    std::ptr::null(),
                    24,
                    CRYPT_MACHINE_KEYSET | CRYPT_DELETEKEYSET,
                ),
                TRUE,
            );

            // User container MUST survive machine container creation and deletion intact
            assert_eq!(CryptAcquireContextW(&mut hp, persist_name.as_ptr(), std::ptr::null(), 24, 0), TRUE);
            ks_len = 4;
            assert_eq!(CryptGetProvParam(hp, PP_KEYSET_TYPE, &mut ks_type as *mut u32 as *mut u8, &mut ks_len, 0), TRUE);
            assert_eq!(ks_type, 0);
            let mut hk_reloaded = 0usize;
            assert_eq!(CryptGetUserKey(hp, AT_KEYEXCHANGE, &mut hk_reloaded), TRUE);

            // Exportability policy MUST be preserved: PRIVATEKEYBLOB rejected, PUBLICKEYBLOB allowed
            let mut pl = 0u32;
            assert_eq!(CryptExportKey(hk_reloaded, 0, PRIVATEKEYBLOB, 0, std::ptr::null_mut(), &mut pl), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_KEY_STATE);
            assert_eq!(CryptExportKey(hk_reloaded, 0, PUBLICKEYBLOB, 0, std::ptr::null_mut(), &mut pl), TRUE);

            // Decrypt ciphertext encrypted before the simulated restart
            assert_eq!(CryptDecrypt(hk_reloaded, 0, TRUE, 0, ct.as_mut_ptr(), &mut ct_len), TRUE);
            assert_eq!(&ct[..ct_len as usize], secret_msg);

            // Incremental CryptHashData + CryptDuplicateHash diverges cleanly without full-input buffering
            let mut hh1 = 0usize;
            let mut hh2 = 0usize;
            assert_eq!(CryptCreateHash(hp, CALG_SHA_256, 0, 0, &mut hh1), TRUE);
            assert_eq!(CryptHashData(hh1, b"ab".as_ptr(), 2, 0), TRUE);
            assert_eq!(CryptDuplicateHash(hh1, std::ptr::null(), 0, &mut hh2), TRUE);
            assert_eq!(CryptHashData(hh1, b"c".as_ptr(), 1, 0), TRUE);
            assert_eq!(CryptHashData(hh2, b"d".as_ptr(), 1, 0), TRUE);
            let mut d1 = [0u8; 32];
            let mut d2 = [0u8; 32];
            let mut dl = 32u32;
            assert_eq!(CryptGetHashParam(hh1, HP_HASHVAL, d1.as_mut_ptr(), &mut dl, 0), TRUE);
            assert_eq!(CryptGetHashParam(hh2, HP_HASHVAL, d2.as_mut_ptr(), &mut dl, 0), TRUE);
            assert_eq!(d1.as_slice(), BcDigest::Sha256.hash(b"abc").as_slice());
            assert_eq!(d2.as_slice(), BcDigest::Sha256.hash(b"abd").as_slice());
            assert_eq!(CryptDestroyHash(hh1), TRUE);
            assert_eq!(CryptDestroyHash(hh2), TRUE);

            assert_eq!(CryptDestroyKey(hk_reloaded), TRUE);
            assert_eq!(CryptReleaseContext(hp, 0), TRUE);
            assert_eq!(CryptAcquireContextW(&mut hp, persist_name.as_ptr(), std::ptr::null(), 24, CRYPT_DELETEKEYSET), TRUE);

            // 8. Distinct CSP names of the same provider type MUST NOT alias or delete each other's containers
            let csp_base = crate::strings::to_wide_z("Microsoft Base Cryptographic Provider v1.0");
            let csp_enh = crate::strings::to_wide_z("Microsoft Enhanced Cryptographic Provider v1.0");
            let shared_cname = crate::strings::to_wide_z("PortFactorySharedContainer");
            let mut hp_base = 0usize;
            let mut hp_enh = 0usize;
            assert_eq!(CryptAcquireContextW(&mut hp_base, shared_cname.as_ptr(), csp_base.as_ptr(), 1, CRYPT_NEWKEYSET), TRUE);
            let mut hk_base = 0usize;
            assert_eq!(CryptGenKey(hp_base, AT_KEYEXCHANGE, 2048 << 16, &mut hk_base), TRUE);
            let mut base_ct = vec![0u8; 256];
            let base_msg = b"base csp key";
            base_ct[..base_msg.len()].copy_from_slice(base_msg);
            let mut base_ct_len = base_msg.len() as u32;
            assert_eq!(CryptEncrypt(hk_base, 0, TRUE, 0, base_ct.as_mut_ptr(), &mut base_ct_len, 256), TRUE);
            assert_eq!(CryptDestroyKey(hk_base), TRUE);
            assert_eq!(CryptReleaseContext(hp_base, 0), TRUE);

            // Deleting the same container name under Enhanced CSP MUST fail and leave Base CSP untouched
            assert_eq!(CryptAcquireContextW(&mut hp_enh, shared_cname.as_ptr(), csp_enh.as_ptr(), 1, CRYPT_DELETEKEYSET), FALSE);
            assert_eq!(crate::teb::last_error(), NTE_BAD_KEYSET);
            // Creating the same container name under Enhanced CSP succeeds independently
            assert_eq!(CryptAcquireContextW(&mut hp_enh, shared_cname.as_ptr(), csp_enh.as_ptr(), 1, CRYPT_NEWKEYSET), TRUE);
            let mut hk_enh = 0usize;
            assert_eq!(CryptGenKey(hp_enh, AT_KEYEXCHANGE, 2048 << 16, &mut hk_enh), TRUE);
            assert_eq!(CryptDestroyKey(hk_enh), TRUE);
            assert_eq!(CryptReleaseContext(hp_enh, 0), TRUE);
            // Delete Enhanced CSP container and simulate restart -> Base CSP container and key still intact
            assert_eq!(CryptAcquireContextW(&mut hp_enh, shared_cname.as_ptr(), csp_enh.as_ptr(), 1, CRYPT_DELETEKEYSET), TRUE);
            capi_containers().lock().unwrap().clear();
            assert_eq!(CryptAcquireContextW(&mut hp_base, shared_cname.as_ptr(), csp_base.as_ptr(), 1, 0), TRUE);
            let mut hk_base_reloaded = 0usize;
            assert_eq!(CryptGetUserKey(hp_base, AT_KEYEXCHANGE, &mut hk_base_reloaded), TRUE);
            assert_eq!(CryptDecrypt(hk_base_reloaded, 0, TRUE, 0, base_ct.as_mut_ptr(), &mut base_ct_len), TRUE);
            assert_eq!(&base_ct[..base_ct_len as usize], base_msg);
            assert_eq!(CryptDestroyKey(hk_base_reloaded), TRUE);

            // Base CSP defaults RC2/RC4 to 40 bits (with 11-byte zero salt), whereas Enhanced defaults to 128 bits
            let mut h_base_rc2 = 0usize;
            assert_eq!(CryptGenKey(hp_base, CALG_RC2, 0, &mut h_base_rc2), TRUE);
            let mut kbits = 0u32;
            let mut kblen = 4u32;
            assert_eq!(CryptGetKeyParam(h_base_rc2, KP_KEYLEN, &mut kbits as *mut u32 as *mut u8, &mut kblen, 0), TRUE);
            assert_eq!(kbits, 40);
            assert_eq!(CryptDestroyKey(h_base_rc2), TRUE);

            let mut h_base_md5 = 0usize;
            assert_eq!(CryptCreateHash(hp_base, CALG_MD5, 0, 0, &mut h_base_md5), TRUE);
            assert_eq!(CryptHashData(h_base_md5, b"password".as_ptr(), 8, 0), TRUE);
            let mut h_base_rc4 = 0usize;
            assert_eq!(CryptDeriveKey(hp_base, CALG_RC4, h_base_md5, 0, &mut h_base_rc4), TRUE);
            kblen = 4;
            assert_eq!(CryptGetKeyParam(h_base_rc4, KP_KEYLEN, &mut kbits as *mut u32 as *mut u8, &mut kblen, 0), TRUE);
            assert_eq!(kbits, 40);
            let mut base_salt = [0xAAu8; 16];
            let mut base_slen = 16u32;
            assert_eq!(CryptGetKeyParam(h_base_rc4, KP_SALT, base_salt.as_mut_ptr(), &mut base_slen, 0), TRUE);
            assert_eq!(base_slen, 11);
            assert_eq!(&base_salt[..11], &[0u8; 11]);
            assert_eq!(CryptDestroyKey(h_base_rc4), TRUE);
            assert_eq!(CryptDestroyHash(h_base_md5), TRUE);

            let mut hp_enh_v = 0usize;
            assert_eq!(CryptAcquireContextW(&mut hp_enh_v, std::ptr::null(), csp_enh.as_ptr(), 1, CRYPT_VERIFYCONTEXT), TRUE);
            let mut h_enh_rc4_gen = 0usize;
            assert_eq!(CryptGenKey(hp_enh_v, CALG_RC4, 0, &mut h_enh_rc4_gen), TRUE);
            kblen = 4;
            assert_eq!(CryptGetKeyParam(h_enh_rc4_gen, KP_KEYLEN, &mut kbits as *mut u32 as *mut u8, &mut kblen, 0), TRUE);
            assert_eq!(kbits, 128);
            assert_eq!(CryptDestroyKey(h_enh_rc4_gen), TRUE);
            assert_eq!(CryptReleaseContext(hp_enh_v, 0), TRUE);

            assert_eq!(CryptReleaseContext(hp_base, 0), TRUE);
            assert_eq!(CryptAcquireContextW(&mut hp_base, shared_cname.as_ptr(), csp_base.as_ptr(), 1, CRYPT_DELETEKEYSET), TRUE);

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
