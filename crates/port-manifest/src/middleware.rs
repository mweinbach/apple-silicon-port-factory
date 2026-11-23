//! Multi-version Windows game middleware catalog and DLL detection helpers.
//!
//! Describes common middleware families shipped alongside Windows x86-64 games,
//! their version-specific DLL filenames, canonical builtin mapping targets, and
//! default execution policies (`Builtin`, `Guest`, or `Disabled`).

use std::collections::{BTreeMap, BTreeSet};

use crate::dll_map::DllRoute;

/// Default loading policy for a middleware family when no manifest or env override is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultDllMode {
    /// Prefer the runtime's native ARM64 builtin implementation over a game-dir PE DLL.
    Builtin,
    /// Prefer the game's own shipping PE DLL via x86-runtime (builtin acts as fallback only).
    Guest,
    /// Block loading the DLL entirely unless explicitly re-enabled.
    Disabled,
}

impl DefaultDllMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Builtin => "builtin",
            Self::Guest => "guest",
            Self::Disabled => "disabled",
        }
    }

    pub fn to_dll_route(self) -> DllRoute {
        match self {
            Self::Builtin => DllRoute::Builtin { provider: None },
            Self::Guest => DllRoute::Guest,
            Self::Disabled => DllRoute::Disabled,
        }
    }
}

/// Specification for one major/minor version of a middleware family.
#[derive(Debug, Clone, Copy)]
pub struct MiddlewareVersionSpec {
    pub version: &'static str,
    pub dlls: &'static [&'static str],
    pub canonical_builtins: &'static [(&'static str, &'static str)],
    pub version_tag: Option<u32>,
    pub notes: &'static str,
}

/// Catalog entry for a middleware family (potentially spanning multiple ABI versions).
#[derive(Debug, Clone, Copy)]
pub struct MiddlewareFamily {
    pub id: &'static str,
    pub display_name: &'static str,
    pub default_mode: DefaultDllMode,
    pub default_version: &'static str,
    pub versions: &'static [MiddlewareVersionSpec],
}

/// Canonical multi-version catalog of Windows game middleware families.
pub const MIDDLEWARE_CATALOG: &[MiddlewareFamily] = &[
    MiddlewareFamily {
        id: "physx",
        display_name: "NVIDIA PhysX",
        default_mode: DefaultDllMode::Guest,
        default_version: "3.4",
        versions: &[
            MiddlewareVersionSpec {
                version: "3.4",
                dlls: &[
                    "physx3_x64.dll",
                    "physx3common_x64.dll",
                    "physx3cooking_x64.dll",
                    "pxfoundation_x64.dll",
                ],
                canonical_builtins: &[],
                version_tag: Some(0x0304_0200),
                notes: "PhysX 3.4 ABI; run shipping retail DLLs via x86-runtime for full physics.",
            },
            MiddlewareVersionSpec {
                version: "4.1",
                dlls: &[
                    "physx_64.dll",
                    "physxcommon_64.dll",
                    "physxcooking_64.dll",
                    "physxfoundation_64.dll",
                ],
                canonical_builtins: &[],
                version_tag: Some(0x0401_0200),
                notes: "PhysX 4.1 ABI; run shipping retail DLLs via x86-runtime.",
            },
            MiddlewareVersionSpec {
                version: "5.x",
                dlls: &[
                    "physx_64.dll",
                    "physxcommon_64.dll",
                    "physxcooking_64.dll",
                    "physxfoundation_64.dll",
                ],
                canonical_builtins: &[],
                version_tag: Some(0x0501_0000),
                notes: "PhysX 5.x ABI; shares DLL names with 4.1, distinguished by version_tag.",
            },
        ],
    },
    MiddlewareFamily {
        id: "fmod",
        display_name: "FMOD",
        default_mode: DefaultDllMode::Guest,
        default_version: "2.02",
        versions: &[
            MiddlewareVersionSpec {
                version: "1.10",
                dlls: &["fmod.dll", "fmod64.dll", "fmodstudio.dll", "fmodstudio64.dll"],
                canonical_builtins: &[
                    ("fmod64.dll", "fmod.dll"),
                    ("fmodstudio.dll", "fmod.dll"),
                    ("fmodstudio64.dll", "fmod.dll"),
                ],
                version_tag: Some(0x0001_1000),
                notes: "FMOD Studio 1.10 ABI; prefer shipping DLLs unless builtin is requested.",
            },
            MiddlewareVersionSpec {
                version: "2.00",
                dlls: &["fmod.dll", "fmod64.dll", "fmodstudio.dll", "fmodstudio64.dll"],
                canonical_builtins: &[
                    ("fmod64.dll", "fmod.dll"),
                    ("fmodstudio.dll", "fmod.dll"),
                    ("fmodstudio64.dll", "fmod.dll"),
                ],
                version_tag: Some(0x0002_0000),
                notes: "FMOD Studio 2.00 ABI; prefer shipping DLLs unless builtin is requested.",
            },
            MiddlewareVersionSpec {
                version: "2.02",
                dlls: &["fmod.dll", "fmod64.dll", "fmodstudio.dll", "fmodstudio64.dll"],
                canonical_builtins: &[
                    ("fmod64.dll", "fmod.dll"),
                    ("fmodstudio.dll", "fmod.dll"),
                    ("fmodstudio64.dll", "fmod.dll"),
                ],
                version_tag: Some(0x0002_0200),
                notes: "FMOD Studio 2.02 ABI; prefer shipping DLLs unless builtin is requested.",
            },
        ],
    },
    MiddlewareFamily {
        id: "bink",
        display_name: "RAD Bink Video",
        default_mode: DefaultDllMode::Builtin,
        default_version: "2",
        versions: &[
            MiddlewareVersionSpec {
                version: "2",
                dlls: &["bink2w64.dll"],
                canonical_builtins: &[],
                version_tag: None,
                notes: "Bink 2 native ARM64 decoder (crates/bink2-codec).",
            },
            MiddlewareVersionSpec {
                version: "1",
                dlls: &["binkw64.dll"],
                canonical_builtins: &[("binkw64.dll", "bink2w64.dll")],
                version_tag: None,
                notes: "Bink 1 legacy DLL routed to bink2w64 builtin.",
            },
        ],
    },
    MiddlewareFamily {
        id: "oodle",
        display_name: "RAD Oodle Data Compression",
        default_mode: DefaultDllMode::Guest,
        default_version: "9",
        versions: &[
            MiddlewareVersionSpec {
                version: "9",
                dlls: &["oo2core_9_win64.dll"],
                canonical_builtins: &[],
                version_tag: None,
                notes: "Run shipping Oodle 9 DLL via x86-runtime (never stub decompressors).",
            },
            MiddlewareVersionSpec {
                version: "8",
                dlls: &["oo2core_8_win64.dll"],
                canonical_builtins: &[("oo2core_8_win64.dll", "oo2core_9_win64.dll")],
                version_tag: None,
                notes: "Run shipping Oodle 8 DLL via x86-runtime.",
            },
            MiddlewareVersionSpec {
                version: "7",
                dlls: &["oo2core_7_win64.dll"],
                canonical_builtins: &[("oo2core_7_win64.dll", "oo2core_9_win64.dll")],
                version_tag: None,
                notes: "Run shipping Oodle 7 DLL via x86-runtime.",
            },
            MiddlewareVersionSpec {
                version: "6",
                dlls: &["oo2core_6_win64.dll"],
                canonical_builtins: &[("oo2core_6_win64.dll", "oo2core_9_win64.dll")],
                version_tag: None,
                notes: "Run shipping Oodle 6 DLL via x86-runtime.",
            },
            MiddlewareVersionSpec {
                version: "5",
                dlls: &["oo2core_5_win64.dll"],
                canonical_builtins: &[("oo2core_5_win64.dll", "oo2core_9_win64.dll")],
                version_tag: None,
                notes: "Run shipping Oodle 5 DLL via x86-runtime.",
            },
            MiddlewareVersionSpec {
                version: "4",
                dlls: &["oo2core_4_win64.dll"],
                canonical_builtins: &[("oo2core_4_win64.dll", "oo2core_9_win64.dll")],
                version_tag: None,
                notes: "Run shipping Oodle 4 DLL via x86-runtime.",
            },
        ],
    },
    MiddlewareFamily {
        id: "xaudio2",
        display_name: "Microsoft XAudio2",
        default_mode: DefaultDllMode::Builtin,
        default_version: "2.9",
        versions: &[
            MiddlewareVersionSpec {
                version: "2.9",
                dlls: &["xaudio2_9redist.dll", "xaudio2_9.dll"],
                canonical_builtins: &[("xaudio2_9.dll", "xaudio2_9redist.dll")],
                version_tag: None,
                notes: "XAudio2 2.9 CoreAudio backend.",
            },
            MiddlewareVersionSpec {
                version: "2.8",
                dlls: &["xaudio2_8.dll"],
                canonical_builtins: &[("xaudio2_8.dll", "xaudio2_9redist.dll")],
                version_tag: None,
                notes: "XAudio2 2.8 mapped to xaudio2_9redist builtin.",
            },
            MiddlewareVersionSpec {
                version: "2.7",
                dlls: &["xaudio2_7.dll"],
                canonical_builtins: &[("xaudio2_7.dll", "xaudio2_9redist.dll")],
                version_tag: None,
                notes: "XAudio2 2.7 mapped to xaudio2_9redist builtin.",
            },
        ],
    },
    MiddlewareFamily {
        id: "xinput",
        display_name: "Microsoft XInput",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1.4",
        versions: &[
            MiddlewareVersionSpec {
                version: "1.4",
                dlls: &["xinput1_4.dll"],
                canonical_builtins: &[],
                version_tag: None,
                notes: "XInput 1.4 GameController backend.",
            },
            MiddlewareVersionSpec {
                version: "1.3",
                dlls: &["xinput1_3.dll"],
                canonical_builtins: &[("xinput1_3.dll", "xinput1_4.dll")],
                version_tag: None,
                notes: "XInput 1.3 routed to xinput1_4 builtin.",
            },
            MiddlewareVersionSpec {
                version: "9.1.0",
                dlls: &["xinput9_1_0.dll"],
                canonical_builtins: &[("xinput9_1_0.dll", "xinput1_4.dll")],
                version_tag: None,
                notes: "XInput 9.1.0 routed to xinput1_4 builtin.",
            },
            MiddlewareVersionSpec {
                version: "1.2",
                dlls: &["xinput1_2.dll"],
                canonical_builtins: &[("xinput1_2.dll", "xinput1_4.dll")],
                version_tag: None,
                notes: "XInput 1.2 routed to xinput1_4 builtin.",
            },
            MiddlewareVersionSpec {
                version: "1.1",
                dlls: &["xinput1_1.dll"],
                canonical_builtins: &[("xinput1_1.dll", "xinput1_4.dll")],
                version_tag: None,
                notes: "XInput 1.1 routed to xinput1_4 builtin.",
            },
        ],
    },
    MiddlewareFamily {
        id: "d3dcompiler",
        display_name: "Microsoft D3DCompiler",
        default_mode: DefaultDllMode::Builtin,
        default_version: "47",
        versions: &[
            MiddlewareVersionSpec {
                version: "47",
                dlls: &["d3dcompiler_47.dll"],
                canonical_builtins: &[],
                version_tag: None,
                notes: "D3DCompiler 47 runtime compiler/reflection shim.",
            },
            MiddlewareVersionSpec {
                version: "46",
                dlls: &["d3dcompiler_46.dll"],
                canonical_builtins: &[("d3dcompiler_46.dll", "d3dcompiler_47.dll")],
                version_tag: None,
                notes: "D3DCompiler 46 routed to d3dcompiler_47 builtin.",
            },
            MiddlewareVersionSpec {
                version: "43",
                dlls: &["d3dcompiler_43.dll"],
                canonical_builtins: &[("d3dcompiler_43.dll", "d3dcompiler_47.dll")],
                version_tag: None,
                notes: "D3DCompiler 43 routed to d3dcompiler_47 builtin.",
            },
        ],
    },
    MiddlewareFamily {
        id: "dlss",
        display_name: "NVIDIA DLSS / Streamline",
        default_mode: DefaultDllMode::Builtin,
        default_version: "streamline",
        versions: &[
            MiddlewareVersionSpec {
                version: "streamline",
                dlls: &[
                    "sl.interposer.dll",
                    "nvngx_dlss.dll",
                    "_nvngx.dll",
                    "nvngx_dlssg.dll",
                    "nvngx_dlssd.dll",
                ],
                canonical_builtins: &[
                    ("_nvngx.dll", "nvngx_dlss.dll"),
                    ("nvngx_dlssg.dll", "nvngx_dlss.dll"),
                    ("nvngx_dlssd.dll", "nvngx_dlss.dll"),
                ],
                version_tag: None,
                notes: "Streamline interposer + DLSS 3/RR MetalFX bridge.",
            },
            MiddlewareVersionSpec {
                version: "3",
                dlls: &[
                    "nvngx_dlss.dll",
                    "_nvngx.dll",
                    "nvngx_dlssg.dll",
                    "nvngx_dlssd.dll",
                ],
                canonical_builtins: &[
                    ("_nvngx.dll", "nvngx_dlss.dll"),
                    ("nvngx_dlssg.dll", "nvngx_dlss.dll"),
                    ("nvngx_dlssd.dll", "nvngx_dlss.dll"),
                ],
                version_tag: None,
                notes: "Standalone DLSS 3 / Frame Generation / Ray Reconstruction bridge.",
            },
            MiddlewareVersionSpec {
                version: "2",
                dlls: &["nvngx_dlss.dll", "_nvngx.dll"],
                canonical_builtins: &[("_nvngx.dll", "nvngx_dlss.dll")],
                version_tag: None,
                notes: "DLSS 2 MetalFX bridge.",
            },
        ],
    },
    MiddlewareFamily {
        id: "fsr",
        display_name: "AMD FidelityFX Super Resolution",
        default_mode: DefaultDllMode::Builtin,
        default_version: "3",
        versions: &[
            MiddlewareVersionSpec {
                version: "3",
                dlls: &[
                    "amd_fidelityfx_loader_dx12.dll",
                    "ffx_fsr3_x64.dll",
                    "amd_fidelityfx_dx12.dll",
                ],
                canonical_builtins: &[
                    ("ffx_fsr3_x64.dll", "amd_fidelityfx_loader_dx12.dll"),
                    ("amd_fidelityfx_dx12.dll", "amd_fidelityfx_loader_dx12.dll"),
                ],
                version_tag: None,
                notes: "AMD FidelityFX SDK / FSR 3 MetalFX bridge.",
            },
            MiddlewareVersionSpec {
                version: "2",
                dlls: &["ffx_fsr2_api_x64.dll"],
                canonical_builtins: &[("ffx_fsr2_api_x64.dll", "amd_fidelityfx_loader_dx12.dll")],
                version_tag: None,
                notes: "AMD FSR 2 API bridged through FidelityFX loader builtin.",
            },
        ],
    },
    MiddlewareFamily {
        id: "xess",
        display_name: "Intel XeSS",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["libxess.dll", "libxess_dx12.dll", "libxess_dx11.dll"],
            canonical_builtins: &[("libxess_dx12.dll", "libxess.dll")],
            version_tag: None,
            notes: "Intel XeSS MetalFX bridge (DX12 and DX11 entry points).",
        }],
    },
    MiddlewareFamily {
        id: "nvapi",
        display_name: "NVIDIA NVAPI",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["nvapi64.dll", "nvapi.dll"],
            canonical_builtins: &[("nvapi.dll", "nvapi64.dll")],
            version_tag: None,
            notes: "NVAPI builtin shim.",
        }],
    },
    MiddlewareFamily {
        id: "ags",
        display_name: "AMD GPU Services (AGS)",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["amd_ags_x64.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "AMD AGS builtin shim.",
        }],
    },
    MiddlewareFamily {
        id: "dstorage",
        display_name: "Microsoft DirectStorage",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1.2",
        versions: &[MiddlewareVersionSpec {
            version: "1.2",
            dlls: &["dstorage.dll", "dstoragecore.dll"],
            canonical_builtins: &[("dstoragecore.dll", "dstorage.dll")],
            version_tag: None,
            notes: "DirectStorage 1.2 native async I/O + GDeflate bridge.",
        }],
    },
    MiddlewareFamily {
        id: "eos",
        display_name: "Epic Online Services (EOS)",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["eossdk-win64-shipping.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "EOS SDK offline/local builtin shim.",
        }],
    },
    MiddlewareFamily {
        id: "steam",
        display_name: "Valve Steamworks",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["steam_api64.dll", "steam_api.dll"],
            canonical_builtins: &[("steam_api.dll", "steam_api64.dll")],
            version_tag: None,
            notes: "Steamworks builtin bridge.",
        }],
    },
    MiddlewareFamily {
        id: "cohtml",
        display_name: "Coherent Gameface (cohtml)",
        default_mode: DefaultDllMode::Guest,
        default_version: "1.40",
        versions: &[MiddlewareVersionSpec {
            version: "1.40",
            dlls: &["cohtml.windowsdesktop.dll", "cohtml_icuuc.dll"],
            canonical_builtins: &[("cohtml.windowsdesktop.dll", "cohtml_windowsdesktop.dll")],
            version_tag: None,
            notes: "Gameface UI; cohtml_icuuc.dll remains in DEFAULT_BUILTIN_OVERRIDES.",
        }],
    },
    MiddlewareFamily {
        id: "mediadecoders",
        display_name: "Coherent MediaDecoders",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["mediadecoders.windowsdesktop.dll"],
            canonical_builtins: &[(
                "mediadecoders.windowsdesktop.dll",
                "mediadecoders_windowsdesktop.dll",
            )],
            version_tag: None,
            notes: "Native ARM64 media decoder builtin.",
        }],
    },
    MiddlewareFamily {
        id: "nrd",
        display_name: "NVIDIA Real-Time Denoisers (NRD)",
        default_mode: DefaultDllMode::Guest,
        default_version: "4",
        versions: &[MiddlewareVersionSpec {
            version: "4",
            dlls: &["nrd.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "Run shipping retail NRD binary via x86-runtime; builtin acts as fallback.",
        }],
    },
    MiddlewareFamily {
        id: "nri",
        display_name: "NVIDIA Rendering Interface (NRI)",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["nri.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "NRI builtin bridge.",
        }],
    },
    MiddlewareFamily {
        id: "nvcloth",
        display_name: "NVIDIA NvCloth",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["nvcloth_x64.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "NvCloth builtin bridge.",
        }],
    },
    MiddlewareFamily {
        id: "tbb",
        display_name: "Intel oneTBB / TBBMalloc",
        default_mode: DefaultDllMode::Builtin,
        default_version: "12",
        versions: &[MiddlewareVersionSpec {
            version: "12",
            dlls: &["tbb12.dll", "tbb.dll", "tbbmalloc.dll"],
            canonical_builtins: &[("tbb12.dll", "tbbmalloc.dll"), ("tbb.dll", "tbbmalloc.dll")],
            version_tag: None,
            notes: "TBB allocator & runtime builtin.",
        }],
    },
    MiddlewareFamily {
        id: "zlib",
        display_name: "zlib",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["zlib1.dll", "v8_zlib.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "Native zlib builtin.",
        }],
    },
    MiddlewareFamily {
        id: "lz4",
        display_name: "LZ4",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["lz4.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "Native LZ4 builtin.",
        }],
    },
    MiddlewareFamily {
        id: "zstd",
        display_name: "Zstandard (zstd)",
        default_mode: DefaultDllMode::Guest,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["zstd.dll", "libzstd.dll"],
            canonical_builtins: &[("libzstd.dll", "zstd.dll")],
            version_tag: None,
            notes: "Prefer shipping zstd DLL via x86-runtime when bundled.",
        }],
    },
    MiddlewareFamily {
        id: "protobuf",
        display_name: "Protocol Buffers",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["libprotobuf.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "Native libprotobuf builtin.",
        }],
    },
    MiddlewareFamily {
        id: "curl",
        display_name: "libcurl",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["libcurl.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "Native libcurl builtin.",
        }],
    },
    MiddlewareFamily {
        id: "gameinput",
        display_name: "Microsoft GameInput",
        default_mode: DefaultDllMode::Builtin,
        default_version: "1",
        versions: &[MiddlewareVersionSpec {
            version: "1",
            dlls: &["gameinput.dll"],
            canonical_builtins: &[],
            version_tag: None,
            notes: "Native GameInput builtin.",
        }],
    },
    MiddlewareFamily {
        id: "mono",
        display_name: "Mono CLR Runtime",
        default_mode: DefaultDllMode::Builtin,
        default_version: "2.0",
        versions: &[MiddlewareVersionSpec {
            version: "2.0",
            dlls: &["mono-2.0-bdwgc.dll", "mono.dll", "mono-2.0-sgen.dll"],
            canonical_builtins: &[
                ("mono-2.0-bdwgc.dll", "mono_2_0_bdwgc.dll"),
                ("mono.dll", "mono_2_0_bdwgc.dll"),
                ("mono-2.0-sgen.dll", "mono_2_0_bdwgc.dll"),
            ],
            version_tag: None,
            notes: "Mono 2.0 native ARM64 runtime bridge.",
        }],
    },
    MiddlewareFamily {
        id: "dotnet",
        display_name: ".NET / CoreCLR Hosting",
        default_mode: DefaultDllMode::Builtin,
        default_version: "8",
        versions: &[MiddlewareVersionSpec {
            version: "8",
            dlls: &[
                "mscoree.dll",
                "mscoreei.dll",
                "hostfxr.dll",
                "coreclr.dll",
                "clr.dll",
            ],
            canonical_builtins: &[
                ("mscoreei.dll", "mscoree.dll"),
                ("clr.dll", "coreclr.dll"),
            ],
            version_tag: None,
            notes: ".NET / CoreCLR host bridge.",
        }],
    },
];

/// Look up a middleware family by its ID (case-insensitive).
pub fn find_family(id: &str) -> Option<&'static MiddlewareFamily> {
    let trimmed = id.trim();
    MIDDLEWARE_CATALOG
        .iter()
        .find(|f| f.id.eq_ignore_ascii_case(trimmed))
}

/// Look up a specific version within a middleware family.
///
/// Supports:
/// - `"auto"` (or empty) -> `family.default_version`
/// - Exact case-insensitive match (e.g. `"3.4"`, `"5.x"`, `"streamline"`)
/// - Major-version prefix match (e.g. `"5"` -> `"5.x"`, `"4"` -> `"4.1"`, `"3"` -> `"3.4"`,
///   `"1"` -> `"1.10"`, `"2"` -> prefers `default_version` `"2.02"` for `fmod`)
/// - Wildcard `.x` match (e.g. `"5.2"` -> `"5.x"`) or minor-prefix match (e.g. `"3.4.2"` -> `"3.4"`)
pub fn find_version(family_id: &str, version: &str) -> Option<&'static MiddlewareVersionSpec> {
    let family = find_family(family_id)?;
    let ver = version.trim();
    if ver.is_empty() || ver.eq_ignore_ascii_case("auto") {
        return family
            .versions
            .iter()
            .find(|v| v.version.eq_ignore_ascii_case(family.default_version))
            .or_else(|| family.versions.first());
    }

    // 1. Exact case-insensitive match.
    if let Some(exact) = family
        .versions
        .iter()
        .find(|v| v.version.eq_ignore_ascii_case(ver))
    {
        return Some(exact);
    }

    let ver_lower = ver.to_ascii_lowercase();
    let dot_prefix = format!("{ver_lower}.");

    // Helper: does `spec.version` match `ver_lower` via major/minor prefix or `.x` wildcard?
    let matches_prefix = |spec_ver: &str| -> bool {
        let sv = spec_ver.to_ascii_lowercase();
        if sv.starts_with(&dot_prefix) {
            return true;
        }
        if let Some(wild_base) = sv.strip_suffix(".x") {
            if ver_lower == wild_base || ver_lower.starts_with(&format!("{wild_base}.")) {
                return true;
            }
        }
        ver_lower.starts_with(&format!("{sv}."))
    };

    // 2. Prefer `default_version` if it matches the prefix (e.g. "2" -> "2.02" on FMOD).
    if let Some(def) = family
        .versions
        .iter()
        .find(|v| v.version.eq_ignore_ascii_case(family.default_version) && matches_prefix(v.version))
    {
        return Some(def);
    }

    // 3. First version matching prefix/wildcard.
    family.versions.iter().find(|v| matches_prefix(v.version))
}

/// Return all unique DLL filenames across all versions of `family`.
pub fn family_all_dlls(family: &MiddlewareFamily) -> BTreeSet<&'static str> {
    let mut set = BTreeSet::new();
    for v in family.versions {
        for &dll in v.dlls {
            set.insert(dll);
        }
    }
    set
}

/// Look up the `(family, version)` owning `dll`.
///
/// When a DLL appears in multiple versions of a family, prefers the most specific
/// version (fewest DLLs in its spec), breaking ties by `family.default_version` and
/// then catalog order.
pub fn find_by_dll(dll: &str) -> Option<(&'static MiddlewareFamily, &'static MiddlewareVersionSpec)> {
    let norm = pe_image::normalize_dll_name(dll);
    for family in MIDDLEWARE_CATALOG {
        let mut best: Option<&'static MiddlewareVersionSpec> = None;
        for spec in family.versions {
            if !spec.dlls.contains(&norm.as_str()) {
                continue;
            }
            best = match best {
                None => Some(spec),
                Some(prev) => {
                    let prev_len = prev.dlls.len();
                    let cur_len = spec.dlls.len();
                    if cur_len < prev_len {
                        Some(spec)
                    } else if cur_len == prev_len
                        && spec.version.eq_ignore_ascii_case(family.default_version)
                        && !prev.version.eq_ignore_ascii_case(family.default_version)
                    {
                        Some(spec)
                    } else {
                        Some(prev)
                    }
                }
            };
        }
        if let Some(spec) = best {
            return Some((family, spec));
        }
    }
    None
}

/// Result of detecting a middleware family from a list of imported or bundled DLLs.
#[derive(Debug, Clone)]
pub struct DetectedMiddleware {
    pub family: &'static MiddlewareFamily,
    pub version: &'static MiddlewareVersionSpec,
    pub matched_dlls: Vec<String>,
}

/// Detect middleware families and their most specific matching versions from a set
/// of DLL filenames (imported or discovered in a game directory).
///
/// Deduplicates per family, prefers the version matching the most input DLLs (then
/// the most specific version with the fewest total DLLs, then `default_version`),
/// and returns results sorted deterministically by `family.id`.
pub fn detect_middleware_from_dlls<'a>(
    dlls: impl IntoIterator<Item = &'a str>,
) -> Vec<DetectedMiddleware> {
    let normalized: BTreeSet<String> = dlls
        .into_iter()
        .map(pe_image::normalize_dll_name)
        .filter(|s| !s.is_empty())
        .collect();

    let mut by_family: BTreeMap<&'static str, DetectedMiddleware> = BTreeMap::new();

    for family in MIDDLEWARE_CATALOG {
        let mut best: Option<(&'static MiddlewareVersionSpec, Vec<String>)> = None;
        for spec in family.versions {
            let matched: Vec<String> = normalized
                .iter()
                .filter(|d| spec.dlls.contains(&d.as_str()))
                .cloned()
                .collect();
            if matched.is_empty() {
                continue;
            }
            best = match best {
                None => Some((spec, matched)),
                Some((prev_spec, prev_matched)) => {
                    let better = if matched.len() != prev_matched.len() {
                        matched.len() > prev_matched.len()
                    } else if spec.dlls.len() != prev_spec.dlls.len() {
                        spec.dlls.len() < prev_spec.dlls.len()
                    } else {
                        spec.version.eq_ignore_ascii_case(family.default_version)
                            && !prev_spec.version.eq_ignore_ascii_case(family.default_version)
                    };
                    if better {
                        Some((spec, matched))
                    } else {
                        Some((prev_spec, prev_matched))
                    }
                }
            };
        }

        if let Some((version, matched_dlls)) = best {
            by_family.insert(
                family.id,
                DetectedMiddleware {
                    family,
                    version,
                    matched_dlls,
                },
            );
        }
    }

    by_family.into_values().collect()
}

/// Generate clean `[middleware.<family>]` TOML snippets for detected middleware.
pub fn suggest_game_toml_middleware(detected: &[DetectedMiddleware]) -> String {
    let mut out = String::new();
    for (i, item) in detected.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&format!(
            "# {} (matched: {})\n",
            item.family.display_name,
            item.matched_dlls.join(", ")
        ));
        out.push_str(&format!("[middleware.{}]\n", item.family.id));
        out.push_str(&format!("version = \"{}\"\n", item.version.version));
        out.push_str(&format!("mode = \"{}\"\n", item.family.default_mode.as_str()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_family_and_versions() {
        assert_eq!(find_family("PhysX").unwrap().id, "physx");
        assert_eq!(find_version("physx", "auto").unwrap().version, "3.4");
        assert_eq!(find_version("physx", "3.4").unwrap().version_tag, Some(0x0304_0200));
        assert_eq!(find_version("physx", "3").unwrap().version, "3.4");
        assert_eq!(find_version("physx", "4").unwrap().version, "4.1");
        assert_eq!(find_version("physx", "4.1").unwrap().version_tag, Some(0x0401_0200));
        assert_eq!(find_version("physx", "5").unwrap().version, "5.x");
        assert_eq!(find_version("physx", "5.x").unwrap().version_tag, Some(0x0501_0000));
        assert_eq!(find_version("physx", "5.3").unwrap().version, "5.x");

        assert_eq!(find_version("fmod", "1").unwrap().version, "1.10");
        assert_eq!(find_version("fmod", "1.10").unwrap().version_tag, Some(0x0001_1000));
        assert_eq!(find_version("fmod", "2.00").unwrap().version_tag, Some(0x0002_0000));
        assert_eq!(find_version("fmod", "2.02").unwrap().version_tag, Some(0x0002_0200));
        assert_eq!(find_version("fmod", "2").unwrap().version, "2.02");

        for ver in ["9", "8", "7", "6", "5", "4"] {
            let spec = find_version("oodle", ver).unwrap();
            assert_eq!(spec.version, ver);
            assert_eq!(spec.dlls, &[format!("oo2core_{ver}_win64.dll").as_str()]);
        }
    }

    #[test]
    fn detect_middleware_deduplicates_and_prefers_specificity() {
        let detected = detect_middleware_from_dlls([
            "PhysX3_x64.dll",
            "PxFoundation_x64.dll",
            "oo2core_8_win64.dll",
            "nvngx_dlss.dll",
            "sl.interposer.dll",
            "fmodstudio64.dll",
            "binkw64.dll",
        ]);

        let ids: Vec<&str> = detected.iter().map(|d| d.family.id).collect();
        assert_eq!(ids, vec!["bink", "dlss", "fmod", "oodle", "physx"]);

        let by_id = |id: &str| detected.iter().find(|d| d.family.id == id).unwrap();
        assert_eq!(by_id("bink").version.version, "1");
        assert_eq!(by_id("dlss").version.version, "streamline");
        assert_eq!(by_id("fmod").version.version, "2.02");
        assert_eq!(by_id("oodle").version.version, "8");
        assert_eq!(by_id("physx").version.version, "3.4");
        assert_eq!(
            by_id("physx").matched_dlls,
            vec!["physx3_x64.dll", "pxfoundation_x64.dll"]
        );

        // Standalone DLSS 2 vs DLSS 3 specificity
        let dlss2 = detect_middleware_from_dlls(["nvngx_dlss.dll"]);
        assert_eq!(dlss2.len(), 1);
        assert_eq!(dlss2[0].version.version, "2");

        let dlss3 = detect_middleware_from_dlls(["nvngx_dlss.dll", "nvngx_dlssg.dll"]);
        assert_eq!(dlss3.len(), 1);
        assert_eq!(dlss3[0].version.version, "3");

        // Suggested TOML parses cleanly
        let toml = suggest_game_toml_middleware(&detected);
        assert!(toml.contains("[middleware.physx]"));
        assert!(toml.contains("version = \"3.4\""));
        assert!(toml.contains("mode = \"guest\""));
    }

    #[test]
    fn find_by_dll_and_family_all_dlls() {
        let (fam, spec) = find_by_dll("C:\\Game\\PhysX3Cooking_x64.DLL").unwrap();
        assert_eq!(fam.id, "physx");
        assert_eq!(spec.version, "3.4");

        let all = family_all_dlls(fam);
        assert_eq!(all.len(), 8);
        assert!(all.contains("physx3_x64.dll"));
        assert!(all.contains("physx_64.dll"));
    }
}
