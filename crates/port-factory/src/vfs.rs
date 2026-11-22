//! Wine-style Windows Virtual Filesystem (VFS) & Path Router.
//!
//! Provides a reusable, embeddable `WinFs` table that maps Windows drives,
//! prefixes, and special folders (`C:\`, `C:\Windows\System32`,
//! `C:\Users\<user>\AppData\Roaming`, `C:\Users\<user>\Documents`, `Z:\` host
//! root, custom game/app mounts, and read-only overlay directories) to host
//! macOS/Unix paths with case-insensitive component resolution.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A single Windows prefix -> host directory mount entry.
#[derive(Debug, Clone)]
pub struct FsMount {
    /// Normalized uppercase key without trailing backslash (e.g. `C:\GAMES\APP`).
    pub key: String,
    /// Original-case Windows prefix (`C:\Games\App`).
    pub display: String,
    /// Backing host directory on macOS/Unix.
    pub host: PathBuf,
    /// Optional read-only flag or description.
    pub description: String,
}

/// Configurable Windows Virtual Filesystem (`WinFs`) for testing, CLI inspection,
/// or embedding in standalone AOT-compiled binaries.
#[derive(Debug, Clone)]
pub struct WinFs {
    mounts: Vec<FsMount>,
    cwd: String,
    env_vars: HashMap<String, String>,
}

impl Default for WinFs {
    fn default() -> Self {
        Self::new()
    }
}

impl WinFs {
    pub fn new() -> Self {
        Self {
            mounts: Vec::new(),
            cwd: "C:\\".to_string(),
            env_vars: HashMap::new(),
        }
    }

    /// Default standardized Translator root (`~/Translator`), overridable via
    /// `TRANSLATOR_HOME` or `AW2_DRIVE_C`.
    pub fn default_translator_root() -> PathBuf {
        if let Some(p) =
            std::env::var_os("TRANSLATOR_HOME").or_else(|| std::env::var_os("AW2_DRIVE_C"))
        {
            return PathBuf::from(p);
        }
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/tmp"));
        home.join("Translator")
    }

    /// Create the standardized `~/Translator` layout (`Documents`, `Desktop`,
    /// `Downloads`, `AppData/{Roaming,Local,LocalLow}`, `Saved Games`, `Pictures`,
    /// `Music`, `Videos`, `Temp`, `ProgramData`, `Program Files`, `Windows`,
    /// `Public`, `Profile`, `drive_c`) + `Z:\` host root mount + optional
    /// application directory mounted at `app_win_dir`.
    pub fn with_standard_layout(
        translator_root: &Path,
        app_host_dir: Option<&Path>,
        app_win_dir: Option<&str>,
    ) -> Self {
        let mut fs = Self::new();
        let user = std::env::var("USER").unwrap_or_else(|_| "player".to_string());
        let u_base = format!("C:\\users\\{user}");

        fs.add_mount("Z:\\", Path::new("/"), "Host root filesystem (Wine Z: drive)");
        fs.add_mount(
            "C:\\",
            &translator_root.join("drive_c"),
            "Fallback Windows C: drive root (~/Translator/drive_c)",
        );
        fs.add_mount(
            "C:\\Windows",
            &translator_root.join("Windows"),
            "Windows system directory (~/Translator/Windows)",
        );
        fs.add_mount(
            "C:\\Windows\\Temp",
            &translator_root.join("Temp"),
            "Windows system temp directory (~/Translator/Temp)",
        );
        fs.add_mount(
            "C:\\ProgramData",
            &translator_root.join("ProgramData"),
            "Shared application data (~/Translator/ProgramData)",
        );
        fs.add_mount(
            "C:\\Program Files",
            &translator_root.join("Program Files"),
            "64-bit Program Files (~/Translator/Program Files)",
        );
        fs.add_mount(
            "C:\\Program Files (x86)",
            &translator_root.join("Program Files (x86)"),
            "32-bit Program Files (~/Translator/Program Files (x86))",
        );
        fs.add_mount(
            "C:\\users\\Public",
            &translator_root.join("Public"),
            "Public shared user folders (~/Translator/Public)",
        );

        // User profile & standardized special folders under ~/Translator/*
        fs.add_mount(
            &u_base,
            &translator_root.join("Profile"),
            "User profile root (~/Translator/Profile)",
        );
        fs.add_mount(
            &format!("{u_base}\\Documents"),
            &translator_root.join("Documents"),
            "Windows Documents (~/Translator/Documents)",
        );
        fs.add_mount(
            &format!("{u_base}\\Desktop"),
            &translator_root.join("Desktop"),
            "Windows Desktop (~/Translator/Desktop)",
        );
        fs.add_mount(
            &format!("{u_base}\\Downloads"),
            &translator_root.join("Downloads"),
            "Windows Downloads (~/Translator/Downloads)",
        );
        fs.add_mount(
            &format!("{u_base}\\Saved Games"),
            &translator_root.join("Saved Games"),
            "Windows Saved Games (~/Translator/Saved Games)",
        );
        fs.add_mount(
            &format!("{u_base}\\Pictures"),
            &translator_root.join("Pictures"),
            "Windows Pictures & Screenshots (~/Translator/Pictures)",
        );
        fs.add_mount(
            &format!("{u_base}\\Music"),
            &translator_root.join("Music"),
            "Windows Music (~/Translator/Music)",
        );
        fs.add_mount(
            &format!("{u_base}\\Videos"),
            &translator_root.join("Videos"),
            "Windows Videos & Captures (~/Translator/Videos)",
        );
        fs.add_mount(
            &format!("{u_base}\\AppData"),
            &translator_root.join("AppData"),
            "Windows AppData root (~/Translator/AppData)",
        );
        fs.add_mount(
            &format!("{u_base}\\AppData\\Roaming"),
            &translator_root.join("AppData/Roaming"),
            "Windows Roaming AppData (%APPDATA% -> ~/Translator/AppData/Roaming)",
        );
        fs.add_mount(
            &format!("{u_base}\\AppData\\Local"),
            &translator_root.join("AppData/Local"),
            "Windows Local AppData (%LOCALAPPDATA% -> ~/Translator/AppData/Local)",
        );
        fs.add_mount(
            &format!("{u_base}\\AppData\\LocalLow"),
            &translator_root.join("AppData/LocalLow"),
            "Windows LocalLow AppData (~/Translator/AppData/LocalLow)",
        );
        fs.add_mount(
            &format!("{u_base}\\AppData\\Local\\Temp"),
            &translator_root.join("Temp"),
            "Windows Temp directory (%TEMP% -> ~/Translator/Temp)",
        );

        fs.add_mount(
            "C:\\ProgramData\\ShaderCache",
            &translator_root.join("ShaderCache"),
            "AOT & runtime pre-converted Metal shader cache (~/Translator/ShaderCache)",
        );
        fs.add_mount(
            "C:\\Windows\\Microsoft.NET",
            &translator_root.join("Windows/Microsoft.NET"),
            ".NET Framework 4.x runtime root (~/Translator/Windows/Microsoft.NET)",
        );
        fs.add_mount(
            "C:\\Program Files\\dotnet",
            &translator_root.join("Program Files/dotnet"),
            ".NET Core / .NET 6-9 hostfxr & shared frameworks (~/Translator/Program Files/dotnet)",
        );

        let win_dir = app_win_dir.unwrap_or("C:\\App");
        if let Some(app_dir) = app_host_dir {
            fs.add_mount(win_dir, app_dir, "Application / Game root directory");
            fs.set_cwd(win_dir);
        }

        // Standard Windows environment variables for `%APPDATA%`, `%LOCALAPPDATA%`, `%TEMP%`, etc.
        fs.env_vars.insert("ALLUSERSPROFILE".into(), "C:\\ProgramData".into());
        fs.env_vars.insert("SystemDrive".into(), "C:".into());
        fs.env_vars.insert("SystemRoot".into(), "C:\\Windows".into());
        fs.env_vars.insert("windir".into(), "C:\\Windows".into());
        fs.env_vars
            .insert("ProgramData".into(), "C:\\ProgramData".into());
        fs.env_vars
            .insert("ProgramFiles".into(), "C:\\Program Files".into());
        fs.env_vars
            .insert("ProgramFiles(x86)".into(), "C:\\Program Files (x86)".into());
        fs.env_vars
            .insert("ProgramW6432".into(), "C:\\Program Files".into());
        fs.env_vars
            .insert("CommonProgramFiles".into(), "C:\\Program Files\\Common Files".into());
        fs.env_vars.insert(
            "CommonProgramFiles(x86)".into(),
            "C:\\Program Files (x86)\\Common Files".into(),
        );
        fs.env_vars
            .insert("PUBLIC".into(), "C:\\users\\Public".into());
        fs.env_vars.insert("HOMEDRIVE".into(), "C:".into());
        fs.env_vars
            .insert("HOMEPATH".into(), format!("\\users\\{user}"));
        fs.env_vars
            .insert("USERPROFILE".into(), u_base.clone());
        fs.env_vars.insert(
            "APPDATA".into(),
            format!("{u_base}\\AppData\\Roaming"),
        );
        fs.env_vars.insert(
            "LOCALAPPDATA".into(),
            format!("{u_base}\\AppData\\Local"),
        );
        fs.env_vars
            .insert("TEMP".into(), format!("{u_base}\\AppData\\Local\\Temp"));
        fs.env_vars
            .insert("TMP".into(), format!("{u_base}\\AppData\\Local\\Temp"));
        fs.env_vars
            .insert("DOTNET_ROOT".into(), "C:\\Program Files\\dotnet".into());
        fs
    }

    /// Default AOT & runtime Metal shader cache directory (`~/Translator/ShaderCache`).
    pub fn default_shader_cache_dir() -> PathBuf {
        if let Some(p) = std::env::var_os("AW2_SHADER_CACHE").filter(|v| !v.is_empty() && v != "0" && v != "off") {
            return PathBuf::from(p);
        }
        Self::default_translator_root().join("ShaderCache")
    }

    /// Ensure the standardized `~/Translator` subdirectories exist on the host filesystem.
    pub fn ensure_drive_c_dirs(translator_root: &Path) -> std::io::Result<()> {
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
            std::fs::create_dir_all(translator_root.join(rel))?;
        }

        if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
            let user = std::env::var("USER").unwrap_or_else(|_| "player".into());
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
                        copy_dir_if_missing(&src, &translator_root.join(new_rel));
                    }
                }
            }
        }
        Ok(())
    }

    /// Parse and register a `WIN_PREFIX=HOST_PATH` mount specification (e.g. `D:\Data=/Volumes/Data`).
    pub fn add_mount_spec(&mut self, spec: &str) -> Result<(), String> {
        let Some((win, host)) = spec.split_once('=') else {
            return Err(format!(
                "invalid mount spec {spec:?}: expected WIN_PATH=HOST_PATH (e.g. D:\\Data=/path/on/mac)"
            ));
        };
        let win = win.trim();
        let host = host.trim();
        if win.is_empty() || host.is_empty() {
            return Err(format!("invalid mount spec {spec:?}: empty prefix or path"));
        }
        self.add_mount(win, Path::new(host), "User-configured mount");
        Ok(())
    }

    pub fn add_mount(&mut self, win_prefix: &str, host: &Path, description: &str) {
        let display = normalize_win_path(win_prefix)
            .trim_end_matches('\\')
            .to_string();
        let key = display.to_ascii_uppercase();
        self.mounts.retain(|m| m.key != key);
        self.mounts.push(FsMount {
            key,
            display,
            host: host.to_path_buf(),
            description: description.to_string(),
        });
        // Longest prefix wins
        self.mounts.sort_by(|a, b| b.key.len().cmp(&a.key.len()));
    }

    pub fn mounts(&self) -> &[FsMount] {
        &self.mounts
    }

    pub fn set_cwd(&mut self, win_cwd: &str) {
        let abs = self.absolute_win_path(win_cwd);
        self.cwd = normalize_win_path(&abs);
    }

    pub fn cwd(&self) -> &str {
        &self.cwd
    }

    /// Expand `%VAR%` environment placeholders in a Windows path.
    pub fn expand_env_vars(&self, input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                let mut var_name = String::new();
                let mut closed = false;
                while let Some(&nc) = chars.peek() {
                    chars.next();
                    if nc == '%' {
                        closed = true;
                        break;
                    }
                    var_name.push(nc);
                }
                if closed && !var_name.is_empty() {
                    if let Some(val) = self
                        .env_vars
                        .iter()
                        .find(|(k, _)| k.eq_ignore_ascii_case(&var_name))
                        .map(|(_, v)| v.as_str())
                    {
                        out.push_str(val);
                        continue;
                    }
                }
                out.push('%');
                out.push_str(&var_name);
                if closed {
                    out.push('%');
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    /// Convert a relative or drive-relative Windows path into an absolute Windows path.
    pub fn absolute_win_path(&self, raw: &str) -> String {
        let expanded = self.expand_env_vars(raw);
        let p = expanded.replace('/', "\\");
        let p = p
            .strip_prefix("\\\\?\\")
            .map(|s| s.to_string())
            .unwrap_or(p);
        let b = p.as_bytes();
        let full = if b.len() >= 3 && b[1] == b':' && b[2] == b'\\' {
            p
        } else if b.len() >= 2 && b[1] == b':' {
            let c = &self.cwd;
            if c.len() >= 2 && c[..2].eq_ignore_ascii_case(&p[..2]) {
                format!("{}\\{}", c, &p[2..])
            } else {
                format!("{}\\{}", &p[..2], &p[2..])
            }
        } else if p.starts_with('\\') {
            format!("{}{}", &self.cwd[..2.min(self.cwd.len())], p)
        } else {
            format!("{}\\{}", self.cwd, p)
        };
        normalize_win_path(&full)
    }

    /// Resolve a Windows path (supporting `%APPDATA%`, forward/backslashes, `.`/`..`,
    /// and case-insensitive host directory traversal) to a host `PathBuf`.
    pub fn to_host(&self, win_path: &str) -> Option<(PathBuf, &FsMount)> {
        let full = self.absolute_win_path(win_path);
        let up = full.to_ascii_uppercase();
        for m in &self.mounts {
            if up == m.key
                || (up.starts_with(&m.key) && up.as_bytes().get(m.key.len()) == Some(&b'\\'))
            {
                let rest = full[m.key.len()..].trim_start_matches('\\');
                let comps: Vec<&str> = rest.split('\\').filter(|c| !c.is_empty()).collect();
                let resolved = ci_resolve_host(&m.host, &comps);
                return Some((resolved, m));
            }
        }
        None
    }

    /// Map a host path back to the most specific mounted Windows path.
    pub fn to_win(&self, host_path: &Path) -> Option<String> {
        let mut best: Option<(usize, String)> = None;
        for m in &self.mounts {
            if let Ok(rest) = host_path.strip_prefix(&m.host) {
                let depth = m.host.components().count();
                if best.as_ref().map_or(true, |(d, _)| depth > *d) {
                    let mut s = m.display.clone();
                    for c in rest.components() {
                        if !s.ends_with('\\') {
                            s.push('\\');
                        }
                        s.push_str(&c.as_os_str().to_string_lossy());
                    }
                    if s.len() == 2 && s.as_bytes()[1] == b':' {
                        s.push('\\');
                    }
                    best = Some((depth, s));
                }
            }
        }
        best.map(|(_, s)| s)
    }
}

fn copy_dir_if_missing(src: &Path, dst: &Path) {
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
            copy_dir_if_missing(&sp, &dp);
        } else if ft.is_file() && !dp.exists() {
            let _ = std::fs::copy(&sp, &dp);
        }
    }
}

/// Normalize Windows path separators and resolve `.` / `..` components lexically.
pub fn normalize_win_path(p: &str) -> String {
    let mut s = p.replace('/', "\\");
    for pre in ["\\\\?\\", "\\??\\", "\\\\.\\"] {
        if let Some(r) = s.strip_prefix(pre) {
            s = r.to_string();
        }
    }
    let (prefix, rest) = if s.len() >= 2 && s.as_bytes()[1] == b':' {
        (s[..2].to_ascii_uppercase(), s[2..].to_string())
    } else {
        (String::new(), s)
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

fn ci_resolve_host(base: &Path, rest: &[&str]) -> PathBuf {
    let mut cur = base.to_path_buf();
    for (i, comp) in rest.iter().enumerate() {
        let found = std::fs::read_dir(&cur).ok().and_then(|rd| {
            let mut ci_match: Option<PathBuf> = None;
            for e in rd.filter_map(|e| e.ok()) {
                let fname = e.file_name();
                let Some(name) = fname.to_str() else {
                    continue;
                };
                if name == *comp {
                    return Some(e.path());
                }
                if ci_match.is_none() && name.eq_ignore_ascii_case(comp) {
                    ci_match = Some(e.path());
                }
            }
            ci_match
        });
        match found {
            Some(p) => cur = p,
            None => {
                for c in &rest[i..] {
                    cur.push(c);
                }
                return cur;
            }
        }
    }
    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winfs_resolves_drives_env_vars_and_case_insensitive_paths() {
        let tmp = std::env::temp_dir().join("aw2_x86_vfs_test");
        let translator_root = tmp.join("Translator");
        let app_dir = tmp.join("game_root");
        std::fs::create_dir_all(app_dir.join("Shaders/Cache")).unwrap();
        WinFs::ensure_drive_c_dirs(&translator_root).unwrap();

        let mut vfs = WinFs::with_standard_layout(
            &translator_root,
            Some(&app_dir),
            Some("C:\\Games\\TestApp"),
        );
        vfs.add_mount_spec(&format!("D:\\Assets={}", app_dir.display()))
            .unwrap();

        // Case-insensitive relative lookup inside CWD (`C:\Games\TestApp`)
        let (resolved, _) = vfs.to_host("shaders\\cache\\test.bin").unwrap();
        assert_eq!(resolved, app_dir.join("Shaders/Cache/test.bin"));

        // Custom D: mount
        let (d_resolved, _) = vfs.to_host("d:/assets/shaders/cache").unwrap();
        assert_eq!(d_resolved, app_dir.join("Shaders/Cache"));

        // %APPDATA% -> ~/Translator/AppData/Roaming
        let (appdata, _) = vfs.to_host("%APPDATA%\\MyGame\\settings.ini").unwrap();
        assert_eq!(
            appdata,
            translator_root.join("AppData/Roaming/MyGame/settings.ini")
        );

        // %USERPROFILE%\Documents -> ~/Translator/Documents
        let (docs, _) = vfs
            .to_host("%USERPROFILE%\\Documents\\My Games\\save1.sav")
            .unwrap();
        assert_eq!(
            docs,
            translator_root.join("Documents/My Games/save1.sav")
        );

        // %USERPROFILE%\Desktop -> ~/Translator/Desktop
        let (desktop, _) = vfs.to_host("%USERPROFILE%\\Desktop\\readme.txt").unwrap();
        assert_eq!(desktop, translator_root.join("Desktop/readme.txt"));

        // %TEMP% -> ~/Translator/Temp
        let (temp, _) = vfs.to_host("%TEMP%\\shader_compile.tmp").unwrap();
        assert_eq!(temp, translator_root.join("Temp/shader_compile.tmp"));
    }
}
