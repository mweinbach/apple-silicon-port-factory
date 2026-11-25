//! Windows path <-> host path mapping.
//!
//! A small mount table maps Windows path prefixes to host directories, e.g.
//! `C:\Games\AlanWake2` -> the real game directory, `C:\` -> a per-user
//! "drive_c" directory. Longest prefix wins. Lookups are case-insensitive:
//! if the exact host path does not exist we resolve each component by a
//! case-insensitive directory scan.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock};

struct Mount {
    /// Normalized, uppercase, no trailing backslash (e.g. `C:\GAMES\ALANWAKE2`).
    key: String,
    /// Original-case prefix used when mapping host -> Windows.
    display: String,
    host: PathBuf,
}

static MOUNTS: RwLock<Vec<Mount>> = RwLock::new(Vec::new());
static CWD: RwLock<String> = RwLock::new(String::new());

fn path_cache() -> &'static RwLock<HashMap<String, PathBuf>> {
    static CACHE: OnceLock<RwLock<HashMap<String, PathBuf>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn add_mount(win_prefix: &str, host: &Path) {
    let display = normalize(win_prefix).trim_end_matches('\\').to_string();
    let key = display.to_ascii_uppercase();
    let mut m = MOUNTS.write().unwrap();
    m.retain(|x| x.key != key);
    m.push(Mount { key, display, host: host.to_path_buf() });
    m.sort_by(|a, b| b.key.len().cmp(&a.key.len()));
    drop(m);
    path_cache().write().unwrap().clear();
}

/// Parse and register a `WIN_PREFIX=HOST_PATH` mount string (e.g. `D:\Data=/Volumes/Data`).
pub fn add_mount_spec(spec: &str) -> Result<(String, PathBuf), String> {
    let Some((win, host)) = spec.split_once('=') else {
        return Err(format!("invalid mount spec {spec:?}: expected WIN_PATH=HOST_PATH"));
    };
    let win = win.trim();
    let host = PathBuf::from(host.trim());
    if win.is_empty() || host.as_os_str().is_empty() {
        return Err(format!("invalid mount spec {spec:?}: empty prefix or path"));
    }
    add_mount(win, &host);
    Ok((normalize(win), host))
}

/// Return a snapshot of all currently registered `(windows_prefix, host_path)` mounts.
pub fn list_mounts() -> Vec<(String, PathBuf)> {
    MOUNTS
        .read()
        .unwrap()
        .iter()
        .map(|m| (m.display.clone(), m.host.clone()))
        .collect()
}

/// Default standardized Translator root directory (`~/Translator` on macOS,
/// `~/Library/Application Support/Translator` on iOS where `$HOME` is read-only
/// and `Documents/` should stay untouched), overridable via `TRANSLATOR_HOME`
/// or `AW2_DRIVE_C`.
pub fn default_translator_root() -> PathBuf {
    if let Some(p) = std::env::var_os("TRANSLATOR_HOME").or_else(|| std::env::var_os("AW2_DRIVE_C")) {
        return PathBuf::from(p);
    }
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    if cfg!(target_os = "ios") {
        home.join("Library").join("Application Support").join("Translator")
    } else {
        home.join("Translator")
    }
}

fn copy_dir_recursive_if_missing(src: &Path, dst: &Path) {
    let Ok(rd) = std::fs::read_dir(src) else {
        return;
    };
    let _ = std::fs::create_dir_all(dst);
    for entry in rd.filter_map(|e| e.ok()) {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() {
            continue;
        }
        let sp = entry.path();
        let dp = dst.join(entry.file_name());
        if ft.is_dir() {
            copy_dir_recursive_if_missing(&sp, &dp);
        } else if ft.is_file() && !dp.exists() {
            let _ = std::fs::copy(&sp, &dp);
        }
    }
}

/// On iOS, preserve any existing user saves/settings from older installs that used
/// `~/Documents/Translator` by copying missing files into `root` (`~/Library/Application Support/Translator`).
pub fn migrate_legacy_ios_translator_root(root: &Path) {
    if !cfg!(target_os = "ios") {
        return;
    }
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        let legacy_docs = home.join("Documents").join("Translator");
        if legacy_docs.is_dir() && legacy_docs != root {
            for rel in ["AppData", "Saved Games", "Documents", "ProgramData"] {
                let src = legacy_docs.join(rel);
                if src.is_dir() {
                    copy_dir_recursive_if_missing(&src, &root.join(rel));
                }
            }
        }
    }
}

/// Create the standardized `~/Translator/{Documents,Desktop,Downloads,AppData,Saved Games,Pictures,Music,Videos,Temp,ProgramData,Program Files,Windows,Public,Profile,drive_c}`
/// hierarchy on disk and mount all standard Windows paths to their corresponding `~/Translator/*` folders.
pub fn init_translator_mounts(override_root: Option<&Path>) -> std::io::Result<PathBuf> {
    let root = override_root
        .map(|p| p.to_path_buf())
        .unwrap_or_else(default_translator_root);
    let user = std::env::var("USER").unwrap_or_else(|_| "player".into());

    // Create standardized top-level Translator folders and subdirectories.
    for rel in [
        "Documents",
        "Desktop",
        "Downloads",
        "Saved Games",
        "ShaderCache",
        "Pictures/Screenshots",
        "Music",
        "Videos/Captures",
        "AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup",
        "AppData/Roaming/Microsoft/Windows/Recent",
        "AppData/Roaming/Microsoft/Windows/Templates",
        "AppData/Local/Temp",
        "AppData/Local/Microsoft/Windows/INetCache",
        "AppData/Local/Microsoft/Windows/INetCookies",
        "AppData/LocalLow",
        "Temp",
        "ProgramData",
        "Program Files/Common Files",
        "Program Files/dotnet/host/fxr/8.0.0",
        "Program Files/dotnet/shared/Microsoft.NETCore.App/8.0.0",
        "Program Files/dotnet/shared/Microsoft.WindowsDesktop.App/8.0.0",
        "Program Files (x86)/Common Files",
        "Public/Documents",
        "Public/Desktop",
        "Public/Downloads",
        "Public/Pictures",
        "Public/Music",
        "Public/Videos",
        "Windows/System32",
        "Windows/SysWOW64",
        "Windows/Fonts",
        "Windows/Temp",
        "Windows/Microsoft.NET/Framework64/v4.0.30319",
        "Windows/Microsoft.NET/Framework/v4.0.30319",
        "Profile/Favorites",
        "drive_c",
    ] {
        std::fs::create_dir_all(root.join(rel))?;
    }

    if override_root.is_none() {
        migrate_legacy_ios_translator_root(&root);
    }

    // Optional legacy migration from `~/Library/Application Support/AW2Port/drive_c`
    // into `~/Translator` (enabled only by targets that opt into `legacy_remedy_migration`).
    if crate::config::app_config().legacy_remedy_migration {
        if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
            let legacy_user = home
                .join("Library/Application Support/AW2Port/drive_c/users")
                .join(&user);
            if legacy_user.is_dir() {
                for (legacy_rel, new_rel) in [
                    ("AppData/Local/Remedy", "AppData/Local/Remedy"),
                    ("AppData/Roaming", "AppData/Roaming"),
                    ("Documents", "Documents"),
                    ("Saved Games", "Saved Games"),
                ] {
                    let src = legacy_user.join(legacy_rel);
                    if src.is_dir() {
                        copy_dir_recursive_if_missing(&src, &root.join(new_rel));
                    }
                }
            }
        }
    }

    // Register mounts from most-specific to root (add_mount sorts by longest prefix).
    add_mount("Z:\\", Path::new("/"));
    add_mount("C:\\", &root.join("drive_c"));
    add_mount("C:\\Windows", &root.join("Windows"));
    add_mount("C:\\Windows\\Temp", &root.join("Temp"));
    add_mount("C:\\Windows\\Microsoft.NET", &root.join("Windows/Microsoft.NET"));
    add_mount("C:\\ProgramData", &root.join("ProgramData"));
    add_mount("C:\\ProgramData\\ShaderCache", &root.join("ShaderCache"));
    add_mount("C:\\Program Files", &root.join("Program Files"));
    add_mount("C:\\Program Files\\dotnet", &root.join("Program Files/dotnet"));
    add_mount("C:\\Program Files (x86)", &root.join("Program Files (x86)"));
    add_mount("C:\\users\\Public", &root.join("Public"));

    let u_base = format!("C:\\users\\{user}");
    add_mount(&u_base, &root.join("Profile"));
    add_mount(&format!("{u_base}\\Documents"), &root.join("Documents"));
    add_mount(&format!("{u_base}\\Desktop"), &root.join("Desktop"));
    add_mount(&format!("{u_base}\\Downloads"), &root.join("Downloads"));
    add_mount(&format!("{u_base}\\Saved Games"), &root.join("Saved Games"));
    add_mount(&format!("{u_base}\\Pictures"), &root.join("Pictures"));
    add_mount(&format!("{u_base}\\Music"), &root.join("Music"));
    add_mount(&format!("{u_base}\\Videos"), &root.join("Videos"));
    add_mount(&format!("{u_base}\\AppData"), &root.join("AppData"));
    add_mount(&format!("{u_base}\\AppData\\Roaming"), &root.join("AppData/Roaming"));
    add_mount(&format!("{u_base}\\AppData\\Local"), &root.join("AppData/Local"));
    add_mount(&format!("{u_base}\\AppData\\LocalLow"), &root.join("AppData/LocalLow"));
    add_mount(&format!("{u_base}\\AppData\\Local\\Temp"), &root.join("Temp"));

    Ok(root)
}

pub fn set_cwd(win: &str) {
    *CWD.write().unwrap() = normalize(&absolute(win));
}

pub fn cwd() -> String {
    let c = CWD.read().unwrap();
    if c.is_empty() {
        "C:\\".into()
    } else {
        c.clone()
    }
}

/// Return the canonical NT device name for a drive letter (e.g. `'C'` -> `\Device\HarddiskVolume1`).
pub fn dos_device_target(drive: char) -> String {
    let up = drive.to_ascii_uppercase();
    let vol = match up {
        'C' => 1u32,
        'Z' => 2u32,
        'A'..='Z' => 3 + (up as u32 - 'A' as u32),
        _ => 1u32,
    };
    format!("\\Device\\HarddiskVolume{vol}")
}

/// Map a canonical NT device volume number back to its DOS drive letter.
fn drive_for_volume(vol: u32) -> Option<char> {
    match vol {
        1 => Some('C'),
        2 => Some('Z'),
        3..=28 => Some((b'A' + (vol - 3) as u8) as char),
        _ => None,
    }
}

/// Convert a Windows path (e.g. `C:\Windows\System32\kernel32.dll`) to its NT device
/// path (`\Device\HarddiskVolume1\Windows\System32\kernel32.dll`).
pub fn win32_to_nt_device_path(win: &str) -> String {
    let norm = normalize(win);
    let b = norm.as_bytes();
    if b.len() >= 2 && b[1] == b':' {
        let dev = dos_device_target(b[0] as char);
        let rest = &norm[2..];
        if rest.is_empty() {
            dev
        } else if rest.starts_with('\\') {
            format!("{dev}{rest}")
        } else {
            format!("{dev}\\{rest}")
        }
    } else if norm.starts_with('\\') {
        format!("\\Device\\HarddiskVolume1{norm}")
    } else {
        format!("\\Device\\HarddiskVolume1\\{norm}")
    }
}

/// Convert an NT device path (e.g. `\Device\HarddiskVolume1\Windows\System32\kernel32.dll`
/// or `\\?\GLOBALROOT\Device\HarddiskVolume1\...`) back to a Win32 drive path (`C:\...`).
pub fn nt_device_path_to_win32(nt: &str) -> Option<String> {
    let s = nt.replace('/', "\\");
    let up = s.to_ascii_uppercase();
    let body = if up.starts_with("\\\\?\\GLOBALROOT\\") || up.starts_with("\\\\.\\GLOBALROOT\\") {
        &s[14..]
    } else if up.starts_with("\\??\\GLOBALROOT\\") {
        &s[14..]
    } else {
        s.as_str()
    };
    let body_up = body.to_ascii_uppercase();
    const DEV_PREFIX: &str = "\\DEVICE\\HARDDISKVOLUME";
    if !body_up.starts_with(DEV_PREFIX) {
        return None;
    }
    let after = &body[DEV_PREFIX.len()..];
    let digits_len = after.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digits_len == 0 {
        return None;
    }
    let vol: u32 = after[..digits_len].parse().ok()?;
    let drive = drive_for_volume(vol)?;
    let rest = &after[digits_len..];
    if rest.is_empty() {
        Some(format!("{drive}:\\"))
    } else if rest.starts_with('\\') {
        Some(format!("{drive}:{rest}"))
    } else {
        None
    }
}

fn strip_win_nt_prefix(s: &str) -> String {
    if let Some(w) = nt_device_path_to_win32(s) {
        return w;
    }
    let up = s.to_ascii_uppercase();
    if up.starts_with("\\DOSDEVICES\\") {
        let rest = &s[12..];
        return nt_device_path_to_win32(rest).unwrap_or_else(|| rest.to_string());
    }
    for pre in ["\\\\?\\", "\\??\\", "\\\\.\\"] {
        if let Some(r) = s.strip_prefix(pre) {
            return nt_device_path_to_win32(r).unwrap_or_else(|| r.to_string());
        }
    }
    s.to_string()
}

/// Normalize separators, strip `\\?\` / `\??\` / `\DosDevices\` and map NT device
/// paths, then resolve `.`/`..` (lexically).
pub fn normalize(p: &str) -> String {
    let s = strip_win_nt_prefix(&p.replace('/', "\\"));
    let (prefix, rest) = if s.len() >= 2 && s.as_bytes()[1] == b':' {
        (s[..2].to_ascii_uppercase(), s[2..].to_string())
    } else {
        (String::new(), s.clone())
    };
    let absolute = rest.starts_with('\\');
    let mut parts: Vec<&str> = Vec::new();
    for c in rest.split('\\') {
        match c {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            c => parts.push(c),
        }
    }
    let mut out = prefix;
    if absolute {
        out.push('\\');
    }
    out.push_str(&parts.join("\\"));
    if out.len() == 2 && out.as_bytes()[1] == b':' && absolute {
        out.push('\\');
    }
    out
}

/// Make a Windows path absolute (GetFullPathName semantics, lexical).
pub fn absolute(p: &str) -> String {
    let expanded = if p.contains('%') {
        crate::kernel32::process::expand_env(p)
    } else {
        p.to_string()
    };
    let p = strip_win_nt_prefix(&expanded.replace('/', "\\"));
    let b = p.as_bytes();
    let full = if b.len() >= 3 && b[1] == b':' && b[2] == b'\\' {
        p
    } else if b.len() >= 2 && b[1] == b':' {
        // Drive-relative "C:foo": relative to cwd if same drive, else root.
        let c = cwd();
        if c[..2].eq_ignore_ascii_case(&p[..2]) {
            format!("{}\\{}", c, &p[2..])
        } else {
            format!("{}\\{}", &p[..2], &p[2..])
        }
    } else if p.starts_with('\\') {
        format!("{}{}", &cwd()[..2], p)
    } else {
        format!("{}\\{}", cwd(), p)
    };
    normalize(&full)
}

fn ci_resolve(base: &Path, rest: &[&str]) -> PathBuf {
    let mut cur = base.to_path_buf();
    for (i, comp) in rest.iter().enumerate() {
        let direct = cur.join(comp);
        if direct.exists() {
            cur = direct;
            continue;
        }
        let found = std::fs::read_dir(&cur).ok().and_then(|rd| {
            rd.filter_map(|e| e.ok())
                .find(|e| e.file_name().to_str().is_some_and(|n| n.eq_ignore_ascii_case(comp)))
                .map(|e| e.path())
        });
        match found {
            Some(p) => cur = p,
            None => {
                // Not found: keep the remaining components verbatim.
                for c in &rest[i..] {
                    cur.push(c);
                }
                return cur;
            }
        }
    }
    cur
}

/// Host-only directory for CAPI provider keystore files (secrets and persisted
/// key containers). Kept outside every guest `C:\` mount and excluded from
/// guest VFS path resolution (`to_host`) so guests cannot read or tamper with
/// non-exportable private keys.
pub fn capi_keystore_dir() -> PathBuf {
    default_translator_root().join(".capi-keystore")
}

/// True if `host` (canonically or via its canonical parent) refers to the
/// host-private CAPI keystore directory or anything inside it.
pub fn is_host_private_keystore_path(host: &Path) -> bool {
    let ks_root = resolve_canonical_host(&capi_keystore_dir());
    let canon = resolve_canonical_host(host);
    if path_eq_or_under_ci(&canon, &ks_root) {
        return true;
    }
    let parent_join = resolve_parent_canonical_host(host);
    path_eq_or_under_ci(&parent_join, &ks_root)
}

/// Map a Windows path to a host path. Returns `None` for paths outside every
/// mount (e.g. a drive letter we do not provide), for device paths, and for
/// host-private keystore paths.
pub fn to_host(win: &str) -> Option<PathBuf> {
    let full = absolute(win);
    let up = full.to_ascii_uppercase();
    if let Some(cached) = path_cache().read().unwrap().get(&up) {
        if is_host_private_keystore_path(cached) {
            return None;
        }
        return Some(cached.clone());
    }
    let mounts = MOUNTS.read().unwrap();
    for m in mounts.iter() {
        if up == m.key || (up.starts_with(&m.key) && up.as_bytes().get(m.key.len()) == Some(&b'\\')) {
            let rest = full[m.key.len()..].trim_start_matches('\\');
            let comps: Vec<&str> = rest.split('\\').filter(|c| !c.is_empty()).collect();
            let direct = comps.iter().fold(m.host.clone(), |acc, c| acc.join(c));
            if direct.exists() {
                drop(mounts);
                if is_host_private_keystore_path(&direct) {
                    return None;
                }
                path_cache().write().unwrap().insert(up, direct.clone());
                return Some(direct);
            }
            let resolved = ci_resolve(&m.host, &comps);
            drop(mounts);
            if is_host_private_keystore_path(&resolved) {
                return None;
            }
            if resolved.exists() {
                path_cache().write().unwrap().insert(up, resolved.clone());
            }
            return Some(resolved);
        }
    }
    None
}

/// Map a host path back to Windows (inverse of the mount table).
pub fn to_win(host: &Path) -> Option<String> {
    let mounts = MOUNTS.read().unwrap();
    let mut best: Option<(usize, String)> = None;
    for m in mounts.iter() {
        if let Ok(rest) = host.strip_prefix(&m.host) {
            let depth = m.host.components().count();
            if best.as_ref().map_or(true, |(d, _)| depth > *d) {
                let mut s = m.key_display();
                for c in rest.components() {
                    s.push('\\');
                    s.push_str(&c.as_os_str().to_string_lossy());
                }
                best = Some((depth, s));
            }
        }
    }
    best.map(|(_, s)| s)
}

impl Mount {
    fn key_display(&self) -> String {
        self.display.clone()
    }
}

/// Is this a well-known device name (`NUL`, `CONOUT$`, ...)?
pub fn device_name(win: &str) -> Option<&'static str> {
    let base = win.rsplit(['\\', '/']).next().unwrap_or(win).to_ascii_uppercase();
    let base = base.split('.').next().unwrap_or("").to_string();
    match base.as_str() {
        "NUL" => Some("NUL"),
        "CONOUT$" => Some("CONOUT$"),
        "CONIN$" => Some("CONIN$"),
        "CON" => Some("CON"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Native runner / bundle write-exclusion guardrails
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct ProtectedNativeSet {
    /// Exact canonicalized protected native files (runner binary, Info.plist, game.toml, etc.).
    files: Vec<PathBuf>,
    /// Protected native directory subtrees (e.g. `<App>.app/Contents/{MacOS,Frameworks,PlugIns,_CodeSignature,Resources/AotCache}`).
    subtrees: Vec<PathBuf>,
    /// Exact `(st_dev, st_ino)` pairs for protected native files (blocks hardlink & symlink aliases).
    inodes: Vec<(u64, u64)>,
}

fn protected_native() -> &'static RwLock<ProtectedNativeSet> {
    static SET: OnceLock<RwLock<ProtectedNativeSet>> = OnceLock::new();
    SET.get_or_init(|| RwLock::new(ProtectedNativeSet::default()))
}

fn lexical_normalize_host(p: &Path) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() && !out.has_root() {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Resolve existing symlink/case-insensitive prefixes via `canonicalize`, preserving any
/// non-existent tail components (lexically normalized).
fn resolve_canonical_host(p: &Path) -> PathBuf {
    if let Ok(c) = std::fs::canonicalize(p) {
        return c;
    }
    let norm = lexical_normalize_host(p);
    let mut tail: Vec<std::ffi::OsString> = Vec::new();
    let mut cur = norm.as_path();
    loop {
        if let Ok(c) = std::fs::canonicalize(cur) {
            let mut out = c;
            for part in tail.into_iter().rev() {
                out.push(part);
            }
            return lexical_normalize_host(&out);
        }
        let Some(name) = cur.file_name() else {
            return norm;
        };
        tail.push(name.to_os_string());
        let Some(parent) = cur.parent() else {
            return norm;
        };
        if parent == cur {
            return norm;
        }
        cur = parent;
    }
}

/// Resolve parent symlinks without following a leaf symlink (catches dangling symlinks placed
/// inside a protected subtree).
fn resolve_parent_canonical_host(p: &Path) -> PathBuf {
    let norm = lexical_normalize_host(p);
    if let (Some(parent), Some(name)) = (norm.parent(), norm.file_name()) {
        if !parent.as_os_str().is_empty() {
            return lexical_normalize_host(&resolve_canonical_host(parent).join(name));
        }
    }
    resolve_canonical_host(&norm)
}

fn path_components_lower(p: &Path) -> Vec<String> {
    p.components()
        .map(|c| c.as_os_str().to_string_lossy().to_ascii_lowercase())
        .collect()
}

fn path_eq_ci(a: &Path, b: &Path) -> bool {
    path_components_lower(a) == path_components_lower(b)
}

fn path_eq_or_under_ci(candidate: &Path, root: &Path) -> bool {
    let cc = path_components_lower(candidate);
    let rc = path_components_lower(root);
    cc.len() >= rc.len() && cc[..rc.len()] == rc[..]
}

fn path_is_ancestor_of_ci(candidate: &Path, target: &Path) -> bool {
    let cc = path_components_lower(candidate);
    let tc = path_components_lower(target);
    !cc.is_empty() && cc.len() <= tc.len() && tc[..cc.len()] == cc[..]
}

fn file_dev_ino(p: &Path, follow_symlinks: bool) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let meta = if follow_symlinks {
        std::fs::metadata(p).ok()?
    } else {
        std::fs::symlink_metadata(p).ok()?
    };
    Some((meta.dev(), meta.ino()))
}

fn push_unique_path(vec: &mut Vec<PathBuf>, p: PathBuf) {
    if !vec.iter().any(|existing| path_eq_ci(existing, &p)) {
        vec.push(p);
    }
}

fn push_unique_inode(vec: &mut Vec<(u64, u64)>, dev_ino: (u64, u64)) {
    if !vec.contains(&dev_ino) {
        vec.push(dev_ino);
    }
}

fn record_dir_file_inodes(set: &mut ProtectedNativeSet, dir: &Path, depth: usize) {
    if depth > 4 {
        return;
    }
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in rd.filter_map(|e| e.ok()) {
        let p = entry.path();
        if let Ok(meta) = std::fs::symlink_metadata(&p) {
            if meta.is_file() {
                if let Some(di) = file_dev_ino(&p, false) {
                    push_unique_inode(&mut set.inodes, di);
                }
            } else if meta.is_dir() {
                record_dir_file_inodes(set, &p, depth + 1);
            }
        }
    }
}

/// Detect `<Bundle>.app/Contents` when `exe_path` resides inside `<Bundle>.app/Contents/MacOS/...`.
pub fn detect_app_bundle_contents(exe_path: &Path) -> Option<PathBuf> {
    let macos_dir = exe_path.parent()?;
    if !macos_dir
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("MacOS"))
    {
        return None;
    }
    let contents_dir = macos_dir.parent()?;
    if !contents_dir
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("Contents"))
    {
        return None;
    }
    let app_dir = contents_dir.parent()?;
    if app_dir
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|n| n.to_ascii_lowercase().ends_with(".app"))
    {
        Some(contents_dir.to_path_buf())
    } else {
        None
    }
}

/// Register a native host runner executable (and, when inside a macOS `.app` bundle, its
/// native bundle code/frameworks/signature/manifest/AOT subtrees) as protected against guest
/// VFS writes, truncation, metadata edits, renames, replacement, removal, and link aliases.
///
/// Ordinary guest data—including `<Bundle>.app/Contents/Resources/Game/`—remains writable.
pub fn protect_native_runner(exe_path: &Path) {
    let canon = resolve_canonical_host(exe_path);
    let mut set = protected_native().write().unwrap_or_else(|e| e.into_inner());
    push_unique_path(&mut set.files, canon.clone());
    if let Some(di) = file_dev_ino(&canon, true) {
        push_unique_inode(&mut set.inodes, di);
    }

    if let Some(contents_dir) = detect_app_bundle_contents(&canon) {
        for sub in ["MacOS", "Frameworks", "PlugIns", "_CodeSignature", "Resources/AotCache"] {
            let dir = resolve_canonical_host(&contents_dir.join(sub));
            push_unique_path(&mut set.subtrees, dir.clone());
            record_dir_file_inodes(&mut set, &dir, 0);
        }
        for file_rel in ["Info.plist", "PkgInfo", "Resources/game.toml"] {
            let f = resolve_canonical_host(&contents_dir.join(file_rel));
            push_unique_path(&mut set.files, f.clone());
            if let Some(di) = file_dev_ino(&f, true) {
                push_unique_inode(&mut set.inodes, di);
            }
        }
    }
}

/// Register a host native artifact subtree (such as the host AOT block-cache directory) as
/// protected against guest VFS writes, truncation, renames, replacement, removal, and link aliases.
pub fn protect_native_subtree(dir: &Path) {
    if dir.as_os_str().is_empty() {
        return;
    }
    let canon = resolve_canonical_host(dir);
    let mut set = protected_native().write().unwrap_or_else(|e| e.into_inner());
    push_unique_path(&mut set.subtrees, canon.clone());
    record_dir_file_inodes(&mut set, &canon, 0);
}

/// True if `host` (following symlinks or via its canonical parent or inode) targets a
/// protected native runner file or protected bundle subtree that guest VFS operations must
/// not create, write, truncate, modify attributes on, or overwrite.
pub fn is_protected_host_write(host: &Path) -> bool {
    if is_host_private_keystore_path(host) {
        return true;
    }
    let set = protected_native().read().unwrap_or_else(|e| e.into_inner());
    if set.files.is_empty() && set.subtrees.is_empty() && set.inodes.is_empty() {
        return false;
    }
    let canon = resolve_canonical_host(host);
    let parent_join = resolve_parent_canonical_host(host);

    for cand in [&canon, &parent_join] {
        if set.files.iter().any(|f| path_eq_ci(cand, f)) {
            return true;
        }
        if set.subtrees.iter().any(|sub| path_eq_or_under_ci(cand, sub)) {
            return true;
        }
    }
    if !set.inodes.is_empty() {
        if let Some(di) = file_dev_ino(host, true) {
            if set.inodes.contains(&di) {
                return true;
            }
        }
        if let Some(di) = file_dev_ino(host, false) {
            if set.inodes.contains(&di) {
                return true;
            }
        }
    }
    false
}

/// True if `host` targets a protected native file/subtree OR any ancestor directory of a
/// protected native file/subtree (prevents renaming, replacing, or deleting `<App>.app`,
/// `<App>.app/Contents`, or the runner's containing directory).
pub fn is_protected_host_remove_or_rename(host: &Path) -> bool {
    if is_protected_host_write(host) {
        return true;
    }
    let canon = resolve_canonical_host(host);
    let parent_join = resolve_parent_canonical_host(host);
    let ks_root = resolve_canonical_host(&capi_keystore_dir());
    for cand in [&canon, &parent_join] {
        if path_is_ancestor_of_ci(cand, &ks_root) {
            return true;
        }
    }
    let set = protected_native().read().unwrap_or_else(|e| e.into_inner());
    if set.files.is_empty() && set.subtrees.is_empty() {
        return false;
    }
    for cand in [&canon, &parent_join] {
        if set.files.iter().any(|f| path_is_ancestor_of_ci(cand, f)) {
            return true;
        }
        if set.subtrees.iter().any(|sub| path_is_ancestor_of_ci(cand, sub)) {
            return true;
        }
    }
    false
}

/// True if `host` is a protected file, subtree, or ancestor that guest hardlinks/symlinks
/// must not alias.
pub fn is_protected_host_alias_target(host: &Path) -> bool {
    is_protected_host_remove_or_rename(host)
}

/// True if an open POSIX descriptor refers to a protected native file or subtree (verified via
/// `fstat` inode match and macOS `F_GETPATH`).
pub fn is_protected_fd(fd: i32) -> bool {
    if fd < 0 {
        return false;
    }
    let (has_inodes, matched_inode) = {
        let set = protected_native().read().unwrap_or_else(|e| e.into_inner());
        if set.files.is_empty() && set.subtrees.is_empty() && set.inodes.is_empty() {
            return false;
        }
        let mut st: libc::stat = unsafe { std::mem::zeroed() };
        let matched = if unsafe { libc::fstat(fd, &mut st) } == 0 {
            set.inodes.contains(&(st.st_dev as u64, st.st_ino as u64))
        } else {
            false
        };
        (!set.inodes.is_empty(), matched)
    };
    let _ = has_inodes;
    if matched_inode {
        return true;
    }
    #[cfg(target_vendor = "apple")]
    {
        let mut buf = [0u8; libc::PATH_MAX as usize];
        if unsafe { libc::fcntl(fd, libc::F_GETPATH, buf.as_mut_ptr()) } == 0 {
            let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            if let Ok(s) = std::str::from_utf8(&buf[..len]) {
                return is_protected_host_remove_or_rename(Path::new(s));
            }
        }
    }
    false
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) struct ProtectedNativeTestGuard {
        prev: ProtectedNativeSet,
        mount_key: Option<String>,
    }

    impl Drop for ProtectedNativeTestGuard {
        fn drop(&mut self) {
            *protected_native().write().unwrap_or_else(|e| e.into_inner()) = self.prev.clone();
            if let Some(ref key) = self.mount_key {
                let mut m = MOUNTS.write().unwrap();
                m.retain(|x| &x.key != key);
                drop(m);
                path_cache().write().unwrap().clear();
            }
        }
    }

    pub(crate) fn protect_native_runner_scoped_for_test(
        mount_prefix: &str,
        mount_host: &Path,
        exe_path: &Path,
    ) -> ProtectedNativeTestGuard {
        let key = normalize(mount_prefix).trim_end_matches('\\').to_ascii_uppercase();
        add_mount(mount_prefix, mount_host);
        let prev = protected_native().read().unwrap_or_else(|e| e.into_inner()).clone();
        protect_native_runner(exe_path);
        ProtectedNativeTestGuard { prev, mount_key: Some(key) }
    }

    #[test]
    fn normalize_paths() {
        assert_eq!(normalize("c:/a/./b/../c"), "C:\\a\\c");
        assert_eq!(normalize("\\\\?\\C:\\x\\y"), "C:\\x\\y");
        assert_eq!(normalize("C:\\"), "C:\\");
        assert_eq!(normalize("C:\\.."), "C:\\");
    }
    #[test]
    fn mount_mapping() {
        let tmp = std::env::temp_dir().join("aw2_path_test");
        std::fs::create_dir_all(tmp.join("Data")).unwrap();
        add_mount("Q:\\Game", &tmp);
        assert_eq!(to_host("q:\\game\\Data").unwrap(), tmp.join("Data"));
        let h = to_host("Q:\\GAME\\data").unwrap();
        assert!(h.file_name().unwrap().to_str().unwrap().eq_ignore_ascii_case("Data") && h.exists());
        assert_eq!(to_win(&tmp.join("Data")).unwrap(), "Q:\\Game\\Data");
        assert!(to_host("Z:\\nothing").is_none());
    }

    #[test]
    fn nt_device_path_roundtrip() {
        assert_eq!(
            win32_to_nt_device_path("C:\\Windows\\System32\\kernel32.dll"),
            "\\Device\\HarddiskVolume1\\Windows\\System32\\kernel32.dll"
        );
        assert_eq!(
            nt_device_path_to_win32("\\Device\\HarddiskVolume1\\Windows\\System32\\kernel32.dll").as_deref(),
            Some("C:\\Windows\\System32\\kernel32.dll")
        );
        assert_eq!(
            nt_device_path_to_win32("\\\\?\\GLOBALROOT\\Device\\HarddiskVolume1\\Windows\\System32\\kernel32.dll").as_deref(),
            Some("C:\\Windows\\System32\\kernel32.dll")
        );
        assert_eq!(
            absolute("\\Device\\HarddiskVolume1\\Windows\\System32\\kernel32.dll"),
            "C:\\Windows\\System32\\kernel32.dll"
        );
        assert_eq!(
            win32_to_nt_device_path("Z:\\tmp\\foo.bin"),
            "\\Device\\HarddiskVolume2\\tmp\\foo.bin"
        );
        assert_eq!(
            nt_device_path_to_win32("\\Device\\HarddiskVolume2\\tmp\\foo.bin").as_deref(),
            Some("Z:\\tmp\\foo.bin")
        );
    }
}
