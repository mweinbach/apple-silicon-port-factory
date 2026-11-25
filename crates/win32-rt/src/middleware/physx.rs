//! NVIDIA PhysX multi-version helpers (`PX_PHYSICS_VERSION` encoding/decoding
//! and manifest version resolution across PhysX 3.4, 4.1, and 5.x).

/// Supported NVIDIA PhysX major ABI lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysxVersion {
    V3_4,
    V4_1,
    V5,
}

/// Encode a `PX_PHYSICS_VERSION` 32-bit word: `(major << 24) | (minor << 16) | (bugfix << 8)`.
pub const fn encode_version(major: u8, minor: u8, bugfix: u8) -> u32 {
    ((major as u32) << 24) | ((minor as u32) << 16) | ((bugfix as u32) << 8)
}

/// Decode a `PX_PHYSICS_VERSION` 32-bit word into `(major, minor, bugfix)`.
pub const fn decode_version(v: u32) -> (u8, u8, u8) {
    (
        ((v >> 24) & 0xff) as u8,
        ((v >> 16) & 0xff) as u8,
        ((v >> 8) & 0xff) as u8,
    )
}

impl PhysxVersion {
    /// Identify the PhysX major ABI line from a guest-supplied `PX_PHYSICS_VERSION` word.
    pub fn from_version_u32(v: u32) -> Option<Self> {
        match decode_version(v).0 {
            3 => Some(Self::V3_4),
            4 => Some(Self::V4_1),
            5 => Some(Self::V5),
            _ => None,
        }
    }

    /// Parse a manifest version specifier (`"3"`, `"3.4"`, `"4"`, `"4.1"`, `"5"`, `"5.x"`).
    pub fn from_spec(s: &str) -> Option<Self> {
        let lower = s.trim().to_ascii_lowercase();
        if lower == "3" || lower == "3.4" || lower.starts_with("3.4.") || lower == "3.x" {
            Some(Self::V3_4)
        } else if lower == "4" || lower == "4.1" || lower.starts_with("4.1.") || lower == "4.x" {
            Some(Self::V4_1)
        } else if lower == "5" || lower == "5.x" || (lower.starts_with("5.") && lower.len() > 2) {
            Some(Self::V5)
        } else {
            None
        }
    }

    /// Canonical version string matching `port_manifest::MIDDLEWARE_CATALOG`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V3_4 => "3.4",
            Self::V4_1 => "4.1",
            Self::V5 => "5.x",
        }
    }

    /// Default `PX_PHYSICS_VERSION` tag for this PhysX line.
    pub fn version_tag(self) -> u32 {
        match self {
            Self::V3_4 => encode_version(3, 4, 2),
            Self::V4_1 => encode_version(4, 1, 2),
            Self::V5 => encode_version(5, 1, 0),
        }
    }

    /// Format a `PX_PHYSICS_VERSION` word for diagnostics, e.g. `"5.1.0 (0x05010000)"`.
    pub fn format_version_u32(v: u32) -> String {
        let (major, minor, bugfix) = decode_version(v);
        format!("{major}.{minor}.{bugfix} (0x{v:08x})")
    }
}

/// Return the PhysX version configured via `[middleware.physx]`, if any.
pub fn configured_version() -> Option<PhysxVersion> {
    super::active_version("physx")
        .as_deref()
        .and_then(PhysxVersion::from_spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proprietary_oodle_and_physx45_have_no_fake_builtins() {
        // Shipping Oodle and PhysX 4.x/5.x core/cooking DLLs run via ARM64 JIT;
        // when no shipping PE is present, LoadLibrary must fail with ERROR_MOD_NOT_FOUND
        // just as on Windows rather than resolving a fake stub builtin.
        for dll in [
            "oo2core_9_win64.dll",
            "oo2core_8_win64.dll",
            "physx_64.dll",
            "PhysX_64.dll",
            "physxcooking_64.dll",
            "PhysXCooking_64.dll",
        ] {
            assert!(
                crate::find_builtin(dll).is_none(),
                "unexpected fake builtin for proprietary middleware {dll}"
            );
        }
        assert_eq!(
            PhysxVersion::from_version_u32(encode_version(4, 1, 2)),
            Some(PhysxVersion::V4_1)
        );
        assert_eq!(
            PhysxVersion::from_version_u32(encode_version(5, 1, 0)),
            Some(PhysxVersion::V5)
        );
    }
}
