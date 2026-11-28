//! Steam & PC Game Library Scanner, VDF/ACF Parser, Owned Library Browser,
//! Windows Depot Downloader, Post-Download (`installscript.vdf` / `_CommonRedist`)
//! Handler, and One-Click macOS `.app` / AOT / Metal Shader Converter.
//!
//! Organized into cohesive submodules while re-exporting every public item at
//! `port_factory::steam_library::*`:
//! - `vdf`: Valve KeyValues (`.vdf` / `.acf`) parser & post-install registry extractor
//! - `discovery`: multi-source Steam / CrossOver / Heroic / external volume discovery
//! - `download`: `DepotDownloader` setup, depot downloads, `steam-port`, bundle listing/export
//! - `macos_bundle`: framework staging, manifest generation, launcher codegen, `.app` conversion
//! - `verify`: `.app` self-containment & portability verifier
//! - `html`: interactive HTML library browser & CLI report formatting

mod acquire_manifest;
mod discovery;
mod download;
mod html;
mod macos_bundle;
mod vdf;
mod verify;

#[cfg(test)]
mod tests;

pub use crate::bundle_common::{copy_ios_shader_cache, copy_portable_shader_cache};
pub use acquire_manifest::{
    format_game_toml_args, is_numeric_steam_app_id, load_acquire_manifest,
    load_acquire_manifest_in_dir, parse_acquire_manifest, redact_launch_args, AcquireManifest,
    AcquireRedistItem, ACQUIRE_MANIFEST_FILENAME, ACQUIRE_MANIFEST_SCHEMA_VERSION,
};
pub use discovery::{
    default_apps_dir, default_steam_downloads_dir, known_steam_app_metadata,
    resolve_steam_target_app_id, sanitize_game_title, scan_all_steam_libraries,
    scan_port_factory_downloads_dir, GameLibrarySource, InstalledGameEntry, OwnedSteamGame,
    SteamLibraryScanReport, SteamUserAccount,
};
pub use download::{
    download_steam_game, export_app_bundle, find_depot_downloader, install_depot_downloader,
    list_external_volumes, list_packaged_bundles, parse_depot_downloader_release_asset,
    port_steam_game, ExternalVolumeInfo, PackagedBundleInfo, SteamDownloadOptions,
    SteamPortOptions, SteamPortResult,
};
pub use html::{format_steam_scan_report, write_steam_browser_html};
pub use macos_bundle::{
    convert_steam_game, dxilconv_candidates, find_repo_game_manifest,
    generate_bundle_game_manifest_toml, generate_bundle_launcher_source,
    generate_game_manifest_toml, generate_info_plist, is_macho_file,
    metal_irconverter_candidates, moltenvk_candidates, parse_vtool_macos_versions,
    stage_bundle_frameworks, stage_bundle_frameworks_with_env, BundleLauncherConfig,
    BundleManifestInput, BundleStageTimings, ConversionFormat, FrameworkDiscoveryEnv,
    StagedFrameworks, SteamConvertOptions, SteamConvertResult,
};
pub use vdf::{
    apply_post_install, parse_installscript_vdf, parse_vdf, scan_post_install, PostInstallSummary,
    SteamRegistryEntry, VdfValue,
};
pub use verify::{
    format_bundle_verify_report, verify_app_bundle, verify_app_bundle_with_options, BundleCheck,
    BundleCheckStatus, BundleVerifyReport, VerifyBundleOptions,
};
