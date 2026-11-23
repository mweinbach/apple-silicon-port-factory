//! Runtime multi-version middleware configuration for `win32-rt`.
//!
//! Stores active middleware versions selected from `game.toml` (`[middleware.*]`)
//! and resolves version tags against `port_manifest`'s shared middleware catalog.

pub mod physx;

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

static ACTIVE_VERSIONS: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();

fn active_versions() -> &'static RwLock<HashMap<String, String>> {
    ACTIVE_VERSIONS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Clear any previously recorded middleware versions and load every `(family, version)`
/// pair declared in `manifest.middleware`.
pub fn configure_from_manifest(manifest: &port_manifest::GameManifest) {
    let mut map = active_versions()
        .write()
        .unwrap_or_else(|e| e.into_inner());
    map.clear();
    for (family, sel) in &manifest.middleware {
        if let Some(ver) = sel.version.as_deref() {
            let trimmed = ver.trim();
            if !trimmed.is_empty() {
                map.insert(family.trim().to_ascii_lowercase(), trimmed.to_string());
            }
        }
    }
}

/// Set the active configured version string for a middleware family (case-insensitive key).
pub fn set_family_version(family: &str, version: impl Into<String>) {
    let key = family.trim().to_ascii_lowercase();
    if key.is_empty() {
        return;
    }
    let ver = version.into();
    let trimmed = ver.trim();
    let mut map = active_versions()
        .write()
        .unwrap_or_else(|e| e.into_inner());
    if trimmed.is_empty() {
        map.remove(&key);
    } else {
        map.insert(key, trimmed.to_string());
    }
}

/// Remove any explicit configured version override for a middleware family.
pub fn clear_family_version(family: &str) {
    let key = family.trim().to_ascii_lowercase();
    let mut map = active_versions()
        .write()
        .unwrap_or_else(|e| e.into_inner());
    map.remove(&key);
}

/// Query the currently configured version string for a middleware family, if set.
pub fn active_version(family: &str) -> Option<String> {
    let key = family.trim().to_ascii_lowercase();
    let map = active_versions()
        .read()
        .unwrap_or_else(|e| e.into_inner());
    map.get(&key).cloned()
}

/// Resolve the active version (or catalog default if unset) for `family` via
/// `port_manifest` and return its numeric `version_tag`, if any.
pub fn active_version_tag(family: &str) -> Option<u32> {
    if let Some(ver) = active_version(family) {
        return port_manifest::find_version(family, &ver).and_then(|spec| spec.version_tag);
    }
    let fam = port_manifest::find_family(family)?;
    port_manifest::find_version(fam.id, fam.default_version).and_then(|spec| spec.version_tag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use physx::{decode_version, encode_version, PhysxVersion};

    #[test]
    fn configure_from_manifest_and_version_tags() {
        let toml = r#"
[middleware.fmod]
version = "1.10"
mode = "builtin"

[middleware.PhysX]
version = "5.x"
mode = "guest"
"#;
        let manifest = port_manifest::GameManifest::parse(toml);
        configure_from_manifest(&manifest);

        assert_eq!(active_version("fmod").as_deref(), Some("1.10"));
        assert_eq!(active_version("FMOD").as_deref(), Some("1.10"));
        assert_eq!(active_version_tag("fmod"), Some(0x0001_1000));

        assert_eq!(active_version("physx").as_deref(), Some("5.x"));
        assert_eq!(active_version_tag("physx"), Some(0x0501_0000));
        assert_eq!(physx::configured_version(), Some(PhysxVersion::V5));

        set_family_version("fmod", "2.00");
        assert_eq!(active_version_tag("fmod"), Some(0x0002_0000));

        // Clearing falls back to the port_manifest catalog default.
        clear_family_version("fmod");
        assert_eq!(active_version("fmod"), None);
        assert_eq!(active_version_tag("fmod"), Some(0x0002_0200));

        clear_family_version("physx");
        assert_eq!(physx::configured_version(), None);
        assert_eq!(active_version_tag("physx"), Some(0x0304_0200));

        assert_eq!(active_version_tag("nonexistent_middleware"), None);

        // Configuring from an empty manifest clears prior overrides.
        set_family_version("physx", "4.1");
        assert_eq!(physx::configured_version(), Some(PhysxVersion::V4_1));
        configure_from_manifest(&port_manifest::GameManifest::default());
        assert_eq!(active_version("physx"), None);
    }

    #[test]
    fn physx_version_encode_decode_and_parsing() {
        let v34 = encode_version(3, 4, 2);
        assert_eq!(v34, 0x0304_0200);
        assert_eq!(decode_version(v34), (3, 4, 2));
        assert_eq!(PhysxVersion::from_version_u32(v34), Some(PhysxVersion::V3_4));
        assert_eq!(PhysxVersion::V3_4.version_tag(), 0x0304_0200);
        assert_eq!(PhysxVersion::V3_4.as_str(), "3.4");
        assert_eq!(PhysxVersion::format_version_u32(v34), "3.4.2 (0x03040200)");

        let v41 = encode_version(4, 1, 2);
        assert_eq!(v41, 0x0401_0200);
        assert_eq!(decode_version(v41), (4, 1, 2));
        assert_eq!(PhysxVersion::from_version_u32(v41), Some(PhysxVersion::V4_1));
        assert_eq!(PhysxVersion::V4_1.version_tag(), 0x0401_0200);
        assert_eq!(PhysxVersion::V4_1.as_str(), "4.1");

        let v51 = encode_version(5, 1, 0);
        assert_eq!(v51, 0x0501_0000);
        assert_eq!(decode_version(v51), (5, 1, 0));
        assert_eq!(PhysxVersion::from_version_u32(v51), Some(PhysxVersion::V5));
        assert_eq!(PhysxVersion::V5.version_tag(), 0x0501_0000);
        assert_eq!(PhysxVersion::V5.as_str(), "5.x");
        assert_eq!(PhysxVersion::format_version_u32(v51), "5.1.0 (0x05010000)");

        assert_eq!(PhysxVersion::from_version_u32(encode_version(2, 8, 4)), None);
        assert_eq!(PhysxVersion::from_version_u32(encode_version(6, 0, 0)), None);

        assert_eq!(PhysxVersion::from_spec("3"), Some(PhysxVersion::V3_4));
        assert_eq!(PhysxVersion::from_spec("3.4"), Some(PhysxVersion::V3_4));
        assert_eq!(PhysxVersion::from_spec("4"), Some(PhysxVersion::V4_1));
        assert_eq!(PhysxVersion::from_spec("4.1"), Some(PhysxVersion::V4_1));
        assert_eq!(PhysxVersion::from_spec("5"), Some(PhysxVersion::V5));
        assert_eq!(PhysxVersion::from_spec("5.x"), Some(PhysxVersion::V5));
        assert_eq!(PhysxVersion::from_spec("5.1"), Some(PhysxVersion::V5));
        assert_eq!(PhysxVersion::from_spec("unknown"), None);
    }
}
