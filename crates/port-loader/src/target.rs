//! Declarative per-game target manifest (`game.toml`).
//!
//! Re-exports [`port_manifest::GameManifest`] as [`TargetManifest`] and provides
//! thin discovery helpers that wire manifest defaults into `win32-rt`.

use std::path::{Path, PathBuf};

pub use port_manifest::GameManifest as TargetManifest;

/// Discover and load a manifest (returning both the parsed manifest and its file path)
/// from `--manifest`, `$PORT_MANIFEST`, `<bundle>/game.toml` (iOS flat `.app`), or
/// `<bundle>/../Resources/game.toml` (macOS `.app`).
///
/// Any `[runtime] args` tokens declared in the manifest are registered with
/// `win32_rt::process::set_manifest_game_args`.
pub fn discover_with_path(explicit: Option<&Path>) -> Option<(TargetManifest, PathBuf)> {
    let path = TargetManifest::discover_path(explicit)?;
    let found = TargetManifest::load_file(&path).ok()?;
    let default_args = found.split_args();
    if !default_args.is_empty() {
        win32_rt::process::set_manifest_game_args(default_args);
    }
    Some((found, path))
}

/// Discover and load a manifest from `--manifest`, `$PORT_MANIFEST`, or `.app` bundle
/// resources, registering any `[runtime] args` with `win32-rt`.
pub fn discover(explicit: Option<&Path>) -> Option<TargetManifest> {
    discover_with_path(explicit).map(|(m, _)| m)
}

/// Load a `game.toml` manifest from an explicit filesystem path.
#[allow(dead_code)]
pub fn load_file(path: &Path) -> std::io::Result<TargetManifest> {
    TargetManifest::load_file(path)
}

/// Resolve a default non-empty `Game` directory inside a self-contained `.app` bundle.
pub fn bundled_game_dir() -> Option<PathBuf> {
    TargetManifest::bundled_game_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_game_toml_manifest_via_port_manifest() {
        let src = r#"
# Example target manifest
[app]
id = "overwatch"
name = "Overwatch"
exe = "Overwatch.exe"
win_dir = "C:\\Games\\Overwatch"
log_dir = "OverwatchPort"
steam_app_id = "2357570"

[runtime]
preferred_base_retry = false
v8_jitless = false # comment after value
aot_dir = "aot-cache"
builtin_overrides = "steam_api64.dll,-bink2w64.dll"
args = "-epicuserid=Default-Epic-User -lang \"en US\""

[middleware.physx]
version = "4.1"
mode = "guest"

[dlls]
"custom_foo.dll" = "disabled"

[ios]
seed_file = "ios-config.ini -> AppData/Local/Game/config.ini"
"#;
        let m = TargetManifest::parse(src);
        assert_eq!(m.id.as_deref(), Some("overwatch"));
        assert_eq!(m.name.as_deref(), Some("Overwatch"));
        assert_eq!(m.exe.as_deref(), Some("Overwatch.exe"));
        assert_eq!(m.win_dir.as_deref(), Some("C:\\Games\\Overwatch"));
        assert_eq!(m.log_dir.as_deref(), Some("OverwatchPort"));
        assert_eq!(m.steam_app_id.as_deref(), Some("2357570"));
        assert_eq!(m.preferred_base_retry, Some(false));
        assert_eq!(m.v8_jitless, Some(false));
        assert_eq!(m.aot_dir.as_deref(), Some("aot-cache"));
        assert_eq!(
            m.builtin_overrides.as_deref(),
            Some("steam_api64.dll,-bink2w64.dll")
        );
        assert_eq!(
            m.split_args(),
            vec!["-epicuserid=Default-Epic-User", "-lang", "en US"]
        );
        assert_eq!(m.middleware_version("physx"), Some("4.1"));
        assert_eq!(
            m.dlls.get("custom_foo.dll"),
            Some(&port_manifest::DllRoute::Disabled)
        );
        assert_eq!(
            m.ios_seed_files,
            vec![(
                "ios-config.ini".to_string(),
                "AppData/Local/Game/config.ini".to_string()
            )]
        );
    }
}
