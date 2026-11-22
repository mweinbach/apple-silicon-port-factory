//! Steam & PC Game Library Scanner, VDF/ACF Parser, Owned Library Browser,
//! Windows Depot Downloader, Post-Download (`installscript.vdf` / `_CommonRedist`)
//! Handler, and One-Click macOS `.app` / AOT / Metal Shader Converter.
//!
//! Supports:
//! 1. **Multi-Source Library Discovery**:
//!    - Native macOS Steam (`~/Library/Application Support/Steam`)
//!    - Windows Steam inside CrossOver bottles (`~/Library/Application Support/CrossOver/Bottles/*/drive_c/Program Files (x86)/Steam`)
//!    - Heroic / Epic / GOG (`~/Library/Application Support/heroic/legendaryConfig/legendary/installed.json`)
//!    - External drives (`/Volumes/*/SteamLibrary`)
//!    - Windows `steam.exe` client itself (`PE32+ x86-64`)
//! 2. **Owned Steam Library Browser**:
//!    - Parses `loginusers.vdf`, `localconfig.vdf`, and `userdata/<id>/config/librarycache/*.json`
//!      to enumerate the user's owned Steam games even before they are downloaded.
//! 3. **Post-Download & Post-Install (`installscript.vdf` & `_CommonRedist`)**:
//!    - Parses Valve KeyValues `installscript.vdf` files to extract `Registry` keys into
//!      `steam_registry.tsv` (automatically loaded by `win32-rt`'s virtual registry) and
//!      neutralizes redundant `_CommonRedist` installers (`VC_redist.x64.exe`, `DXSETUP.exe`,
//!      `dotnetfx.exe`) that are natively built into `win32-rt`.
//! 4. **One-Click Conversion (`convert_steam_game`)**:
//!    - Packages any installed Steam AppID, Heroic game, or `steam.exe` into a native macOS
//!      `.app` bundle (with `port-runner`, `dxil-metald`, `steam_appid.txt`, `steam_registry.tsv`,
//!      prewarmed `.aot` ARM64 block caches, and pre-converted Metal shaders), an embeddable Rust
//!      AOT crate, or a Metal shader cache.

use crate::aot::{compile_pe_file_to_rust, AotOptions};
use crate::game_profiles::detect_pe_game_profile;
use crate::gpu_shaders::precompile_shaders_for_paths;
use crate::scaffold::generate_embeddable_aot_crate;
use crate::scanner::collect_pe_files;
use crate::win32_db::{audit_pe_imports, Win32Database};
use anyhow::{bail, Context, Result};
use pe_image::{ExportTarget, Pe};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

// ============================================================================
// 1. Valve KeyValues (.vdf / .acf) Recursive Parser
// ============================================================================

/// A parsed node in a Valve KeyValues (`.vdf` / `.acf`) document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VdfValue {
    Str(String),
    Map(Vec<(String, VdfValue)>),
}

impl VdfValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s.as_str()),
            Self::Map(_) => None,
        }
    }

    pub fn as_map(&self) -> Option<&[(String, VdfValue)]> {
        match self {
            Self::Map(m) => Some(m.as_slice()),
            Self::Str(_) => None,
        }
    }

    /// Case-insensitive lookup in a `Map` node.
    pub fn get(&self, key: &str) -> Option<&VdfValue> {
        let m = self.as_map()?;
        m.iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    }

    /// Case-insensitive string value lookup.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }

    /// Deep case-insensitive path lookup, e.g. `&["AppState", "name"]`.
    pub fn get_path(&self, path: &[&str]) -> Option<&VdfValue> {
        let mut cur = self;
        for seg in path {
            cur = cur.get(seg)?;
        }
        Some(cur)
    }
}

/// Tokenize and parse a Valve KeyValues (`.vdf` / `.acf`) text document.
pub fn parse_vdf(input: &str) -> VdfValue {
    let tokens = tokenize_vdf(input);
    let mut pos = 0;
    VdfValue::Map(parse_vdf_map(&tokens, &mut pos))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum VdfToken {
    LBrace,
    RBrace,
    Text(String),
}

fn tokenize_vdf(input: &str) -> Vec<VdfToken> {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let n = bytes.len();
    let mut i = 0;
    while i < n {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            while i < n && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b == b'{' {
            out.push(VdfToken::LBrace);
            i += 1;
            continue;
        }
        if b == b'}' {
            out.push(VdfToken::RBrace);
            i += 1;
            continue;
        }
        if b == b'"' {
            i += 1;
            let mut s = String::new();
            while i < n {
                if bytes[i] == b'\\' && i + 1 < n {
                    let esc = bytes[i + 1];
                    match esc {
                        b'n' => s.push('\n'),
                        b't' => s.push('\t'),
                        b'\\' => s.push('\\'),
                        b'"' => s.push('"'),
                        other => s.push(other as char),
                    }
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                let ch = input[i..].chars().next().unwrap_or(' ');
                s.push(ch);
                i += ch.len_utf8();
            }
            out.push(VdfToken::Text(s));
            continue;
        }
        // Unquoted token
        let start = i;
        while i < n
            && !bytes[i].is_ascii_whitespace()
            && bytes[i] != b'{'
            && bytes[i] != b'}'
            && bytes[i] != b'"'
        {
            i += 1;
        }
        if i > start {
            out.push(VdfToken::Text(input[start..i].to_string()));
        } else {
            i += 1;
        }
    }
    out
}

fn parse_vdf_map(tokens: &[VdfToken], pos: &mut usize) -> Vec<(String, VdfValue)> {
    let mut entries = Vec::new();
    while *pos < tokens.len() {
        match &tokens[*pos] {
            VdfToken::RBrace => {
                *pos += 1;
                break;
            }
            VdfToken::LBrace => {
                *pos += 1;
                let _ = parse_vdf_map(tokens, pos);
            }
            VdfToken::Text(key) => {
                let key = key.clone();
                *pos += 1;
                if *pos >= tokens.len() {
                    break;
                }
                match &tokens[*pos] {
                    VdfToken::LBrace => {
                        *pos += 1;
                        let child = parse_vdf_map(tokens, pos);
                        entries.push((key, VdfValue::Map(child)));
                    }
                    VdfToken::Text(val) => {
                        let val = val.clone();
                        *pos += 1;
                        entries.push((key, VdfValue::Str(val)));
                    }
                    VdfToken::RBrace => {
                        break;
                    }
                }
            }
        }
    }
    entries
}

// ============================================================================
// 2. Steam User Profile, Installed Game, and Owned Library Models
// ============================================================================

/// A logged-in Steam account discovered from `config/loginusers.vdf`.
#[derive(Debug, Clone)]
pub struct SteamUserAccount {
    pub steam_id64: String,
    pub account_id32: String,
    pub account_name: String,
    pub persona_name: String,
    pub most_recent: bool,
}

/// Source library where a game or client was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameLibrarySource {
    MacOsSteam,
    CrossOverSteam { bottle_name: String },
    HeroicEpic,
    PortFactoryLibrary,
    ExternalSteam { volume_name: String },
    WindowsSteamClient { bottle_name: String },
}

impl GameLibrarySource {
    pub fn label(&self) -> String {
        match self {
            Self::MacOsSteam => "macOS Steam".to_string(),
            Self::CrossOverSteam { bottle_name } => format!("CrossOver Steam ({bottle_name})"),
            Self::HeroicEpic => "Heroic / Epic Games".to_string(),
            Self::PortFactoryLibrary => "Port Factory Steam".to_string(),
            Self::ExternalSteam { volume_name } => format!("External Drive ({volume_name})"),
            Self::WindowsSteamClient { bottle_name } => {
                format!("Windows Steam Client ({bottle_name})")
            }
        }
    }
}

/// A registry key extracted from a Steam `installscript.vdf` post-install file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamRegistryEntry {
    pub key_path: String,
    pub value_name: String,
    pub value_type: String,
    pub value_data: String,
}

/// Summary of post-download / post-install requirements (`installscript.vdf` & `_CommonRedist`).
#[derive(Debug, Clone, Default)]
pub struct PostInstallSummary {
    pub installscript_files: Vec<PathBuf>,
    pub common_redist_dirs: Vec<PathBuf>,
    pub bypassable_redistributables: Vec<String>,
    pub registry_entries: Vec<SteamRegistryEntry>,
    pub run_processes: Vec<String>,
}

/// An installed game or client discovered on the Mac, audited for conversion readiness.
#[derive(Debug, Clone)]
pub struct InstalledGameEntry {
    pub app_id: String,
    pub name: String,
    pub source: GameLibrarySource,
    pub install_dir: PathBuf,
    pub size_on_disk_bytes: u64,
    pub build_id: String,
    pub platform: String,
    pub primary_exe: Option<PathBuf>,
    pub pe_count: usize,
    pub total_imports: usize,
    pub resolved_imports: usize,
    pub engine_profile: String,
    pub post_install: PostInstallSummary,
}

impl InstalledGameEntry {
    pub fn readiness_pct(&self) -> f64 {
        if self.total_imports == 0 {
            100.0
        } else {
            (self.resolved_imports as f64) * 100.0 / (self.total_imports as f64)
        }
    }
}

/// An owned Steam game discovered from `userdata/*/config/librarycache/*.json` or `localconfig.vdf`.
#[derive(Debug, Clone)]
pub struct OwnedSteamGame {
    pub app_id: u32,
    pub title: String,
    pub engine_hint: String,
    pub installed: bool,
    pub install_source: Option<String>,
    pub playtime_minutes: u64,
    pub last_played_unix: u64,
    pub achievements_cached: usize,
}

/// Complete scan report across all local Steam, CrossOver, Heroic, and external libraries.
#[derive(Debug, Clone, Default)]
pub struct SteamLibraryScanReport {
    pub users: Vec<SteamUserAccount>,
    pub library_folders: Vec<PathBuf>,
    pub installed_games: Vec<InstalledGameEntry>,
    pub owned_games: Vec<OwnedSteamGame>,
}

// ============================================================================
// 3. Well-Known Steam AppID Catalog (Titles + Engine Profiles)
// ============================================================================

/// Lookup known Steam AppID title and engine/conversion profile for fast offline browsing.
pub fn known_steam_app_metadata(app_id: u32) -> Option<(&'static str, &'static str)> {
    Some(match app_id {
        730 => ("Counter-Strike 2", "VALVE_SOURCE_2 (Vulkan/DX11 -> Metal)"),
        570 => ("Dota 2", "VALVE_SOURCE_2 (Vulkan -> Metal)"),
        440 => ("Team Fortress 2", "Valve Source Engine (DX9/Vulkan -> Metal)"),
        220 => ("Half-Life 2", "Valve Source Engine"),
        400 => ("Portal", "Valve Source Engine"),
        620 => ("Portal 2", "Valve Source Engine"),
        1086940 => ("Baldur's Gate 3", "LARIAN_DIVINITY_4 (DX11/Vulkan -> Metal)"),
        2357570 => ("Overwatch 2", "BLIZZARD_TANK_DX12 (DX12 -> Metal, 100% Win32 Ready)"),
        2767030 => ("Marvel Rivals", "UNREAL_ENGINE_5 (DX12 Ultimate -> Metal)"),
        1151640 => ("Horizon Zero Dawn Complete Edition", "SONY_DECIMA_NIXXES (DX12 -> Metal)"),
        1172470 => ("Apex Legends", "EA_FROSTBITE_DX12 (DX12 -> Metal)"),
        275850 => ("No Man's Sky", "Hello Games Custom Engine (Vulkan/Metal)"),
        346010 => ("Besiege", "UNITY_ENGINE (Mono/IL2CPP -> Metal)"),
        105600 => ("Terraria", "FNA / XNA .NET Framework (-> Native CLR Bridge)"),
        367520 => ("Hollow Knight", "UNITY_ENGINE (Unity -> Metal)"),
        1030300 => ("Hollow Knight: Silksong", "UNITY_ENGINE (Unity -> Metal)"),
        255710 => ("Cities: Skylines", "UNITY_ENGINE (Unity Mono -> Metal)"),
        289070 => ("Sid Meier's Civilization VI", "Firaxis LORE Engine (DX12 -> Metal)"),
        750920 => ("Shadow of the Tomb Raider", "Crystal Foundation Engine (DX12 -> Metal)"),
        203160 => ("Tomb Raider", "Crystal Engine (DX11 -> Metal)"),
        391540 => ("Undertale", "GameMaker Studio"),
        960090 => ("Bloons TD 6", "UNITY_ENGINE (Unity IL2CPP -> Metal)"),
        236390 => ("War Thunder", "Dagor Engine (DX12/Vulkan -> Metal)"),
        236850 => ("Europa Universalis IV", "Clausewitz Engine"),
        590380 => ("Into the Breach", "Subset Engine (OpenGL/SDL)"),
        319630 => ("Life is Strange", "Unreal Engine 3/4"),
        223850 => ("3DMark", "UL Futuremark DX12 Benchmark (DX12 -> Metal)"),
        341800 => ("Keep Talking and Nobody Explodes", "UNITY_ENGINE"),
        431240 => ("Golf With Your Friends", "UNITY_ENGINE"),
        444090 => ("Paladins", "Unreal Engine 3/4"),
        674940 => ("Stick Fight: The Game", "UNITY_ENGINE"),
        210970 => ("The Witness", "Thekla Custom Engine (DX11 -> Metal)"),
        210770 => ("Sanctum 2", "Unreal Engine 3"),
        22000 => ("World of Goo", "2D Boy Engine"),
        221640 => ("Super Hexagon", "OpenGL Engine"),
        225260 => ("Brutal Legend", "Double Fine Buddha Engine"),
        232790 => ("Broken Age", "Moai Engine"),
        240720 => ("Getting Over It with Bennett Foddy", "UNITY_ENGINE"),
        316610 => ("5D Chess With Multiverse Time Travel", "Custom OpenGL/C++"),
        323580 => ("Jotun: Valhalla Edition", "UNITY_ENGINE"),
        327140 => ("ibbb", "Custom 2D Engine"),
        351510 => ("Quiplash", "Jackbox Engine"),
        363970 => ("Clicker Heroes", "Adobe AIR / Unity"),
        365450 => ("Hacknet", "FNA / XNA .NET (-> Native CLR Bridge)"),
        397460 => ("The Jackbox Party Pack 2", "Jackbox Engine"),
        417860 => ("Emily is Away", "Ren'Py / Unity"),
        444640 => ("Bloons TD Battles", "Ninja Kiwi Engine"),
        445980 => ("Wizard of Legend", "UNITY_ENGINE"),
        474960 => ("Quantum Break", "NORTHLIGHT_DX12 (Remedy Northlight DX12 -> Metal)"),
        870780 => ("Control Ultimate Edition", "NORTHLIGHT_DX12 (Remedy Northlight DX12 -> Metal)"),
        1245620 => ("ELDEN RING", "FROMSOFTWARE_DX12 (DX12 -> Metal)"),
        1091500 => ("Cyberpunk 2077", "REDENGINE_4_DX12 (DX12 Ultimate -> Metal)"),
        2358720 => ("Black Myth: Wukong", "UNREAL_ENGINE_5 (DX12 Ultimate -> Metal)"),
        553850 => ("HELLDIVERS 2", "ARROWHEAD_STINGRAY_DX12 (DX12 -> Metal)"),
        2246340 => ("Monster Hunter Wilds", "CAPCOM_RE_ENGINE (DX12 -> Metal)"),
        2050650 => ("Resident Evil 4", "CAPCOM_RE_ENGINE (DX12 -> Metal)"),
        1623730 => ("Palworld", "UNREAL_ENGINE_5 (DX12 -> Metal)"),
        4069520 => ("Steam App 4069520", "Windows x64 Game"),
        3321460 => ("Steam App 3321460", "Windows x64 Game"),
        2790030 => ("Steam App 2790030", "Windows x64 Game"),
        2371090 => ("Steam App 2371090", "Windows x64 Game"),
        1111940 => ("Steam App 1111940", "Windows x64 Game"),
        1276390 => ("Steam App 1276390", "Windows x64 Game"),
        508440 => ("Totally Accurate Battle Simulator", "UNITY_ENGINE (Unity -> Metal)"),
        531510 => ("Just Shapes & Beats", "UNITY_ENGINE"),
        578720 => ("The Jackbox Party Pack 4", "Jackbox Engine"),
        624820 => ("Clustertruck", "UNITY_ENGINE"),
        664830 => ("The Jackbox Party Pack 5", "Jackbox Engine"),
        70300 => ("VVVVVV", "C++ SDL2"),
        _ => return None,
    })
}

// ============================================================================
// 4. Multi-Source Steam & Game Library Scanner
// ============================================================================

/// Scan all Steam installations (macOS Steam, CrossOver Windows Steam bottles,
/// external drives), Heroic/Epic libraries, and Windows `steam.exe` on the host.
pub fn scan_all_steam_libraries(workspace_root: &Path) -> Result<SteamLibraryScanReport> {
    let db = Win32Database::from_workspace_src(workspace_root)?;
    let home = std::env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/Users/Shared"));

    let mut report = SteamLibraryScanReport::default();
    let mut seen_users = BTreeSet::new();
    let mut seen_library_dirs = BTreeSet::new();

    // 1. Discover Steam roots: macOS Steam + CrossOver Bottles + External Volumes
    let mut steam_roots: Vec<(PathBuf, GameLibrarySource)> = Vec::new();
    let macos_steam = home.join("Library/Application Support/Steam");
    if macos_steam.is_dir() {
        steam_roots.push((macos_steam, GameLibrarySource::MacOsSteam));
    }

    let crossover_bottles = home.join("Library/Application Support/CrossOver/Bottles");
    if let Ok(entries) = fs::read_dir(&crossover_bottles) {
        for entry in entries.flatten() {
            let bottle_path = entry.path();
            if !bottle_path.is_dir() {
                continue;
            }
            let bottle_name = entry.file_name().to_string_lossy().into_owned();
            let win_steam = bottle_path.join("drive_c/Program Files (x86)/Steam");
            if win_steam.is_dir() {
                steam_roots.push((
                    win_steam.clone(),
                    GameLibrarySource::CrossOverSteam {
                        bottle_name: bottle_name.clone(),
                    },
                ));
                // Check if Windows 64-bit `steam.exe` itself is present in this bottle
                let steam_exe = win_steam.join("steam.exe");
                if steam_exe.is_file() {
                    if let Ok(client_entry) =
                        inspect_windows_steam_client(&win_steam, &bottle_name, &db)
                    {
                        report.installed_games.push(client_entry);
                    }
                }
            }
        }
    }

    if let Ok(vols) = fs::read_dir("/Volumes") {
        for vol in vols.flatten() {
            let vol_path = vol.path();
            let vol_name = vol.file_name().to_string_lossy().into_owned();
            for sub in ["SteamLibrary", "Steam"] {
                let candidate = vol_path.join(sub);
                if candidate.join("steamapps").is_dir() {
                    steam_roots.push((
                        candidate,
                        GameLibrarySource::ExternalSteam {
                            volume_name: vol_name.clone(),
                        },
                    ));
                }
            }
        }
    }

    // 2. Parse users, libraryfolders.vdf, and appmanifest_*.acf from each Steam root
    let mut owned_map: BTreeMap<u32, OwnedSteamGame> = BTreeMap::new();

    for (steam_root, source) in &steam_roots {
        // Parse loginusers.vdf
        let login_vdf_path = steam_root.join("config/loginusers.vdf");
        if let Ok(raw) = fs::read_to_string(&login_vdf_path) {
            let vdf = parse_vdf(&raw);
            if let Some(users_node) = vdf.get("users").and_then(|v| v.as_map()) {
                for (steam_id64, user_val) in users_node {
                    if !seen_users.insert(steam_id64.clone()) {
                        continue;
                    }
                    let id64_num = steam_id64.parse::<u64>().unwrap_or(0);
                    let account_id32 = if id64_num > 76561197960265728 {
                        (id64_num - 76561197960265728).to_string()
                    } else {
                        String::new()
                    };
                    report.users.push(SteamUserAccount {
                        steam_id64: steam_id64.clone(),
                        account_id32,
                        account_name: user_val
                            .get_str("AccountName")
                            .unwrap_or_default()
                            .to_string(),
                        persona_name: user_val
                            .get_str("PersonaName")
                            .unwrap_or_default()
                            .to_string(),
                        most_recent: user_val.get_str("MostRecent") == Some("1"),
                    });
                }
            }
        }

        // Collect steamapps directories (from root + libraryfolders.vdf)
        let mut steamapps_dirs = vec![steam_root.join("steamapps")];
        let lib_vdf_path = steam_root.join("steamapps/libraryfolders.vdf");
        if let Ok(raw) = fs::read_to_string(&lib_vdf_path) {
            let vdf = parse_vdf(&raw);
            if let Some(folders) = vdf.get("libraryfolders").and_then(|v| v.as_map()) {
                for (_, folder_val) in folders {
                    if let Some(p) = folder_val.get_str("path") {
                        let resolved = resolve_library_folder_path(steam_root, p);
                        if resolved.join("steamapps").is_dir() {
                            steamapps_dirs.push(resolved.join("steamapps"));
                        }
                    }
                }
            }
        }

        for sa_dir in steamapps_dirs {
            if !sa_dir.is_dir() || !seen_library_dirs.insert(sa_dir.clone()) {
                continue;
            }
            report.library_folders.push(sa_dir.clone());

            let Ok(entries) = fs::read_dir(&sa_dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let fname = entry.file_name().to_string_lossy().into_owned();
                if fname.starts_with("appmanifest_") && fname.ends_with(".acf") {
                    if let Ok(game) = inspect_acf_manifest(&path, &sa_dir, source.clone(), &db) {
                        if let Ok(id_num) = game.app_id.parse::<u32>() {
                            let entry = owned_map.entry(id_num).or_insert_with(|| OwnedSteamGame {
                                app_id: id_num,
                                title: game.name.clone(),
                                engine_hint: game.engine_profile.clone(),
                                installed: true,
                                install_source: Some(source.label()),
                                playtime_minutes: 0,
                                last_played_unix: 0,
                                achievements_cached: 0,
                            });
                            entry.installed = true;
                            entry.title = game.name.clone();
                            entry.engine_hint = game.engine_profile.clone();
                            entry.install_source = Some(source.label());
                        }
                        report.installed_games.push(game);
                    }
                }
            }
        }

        // 3. Scan userdata/*/config/librarycache/*.json and localconfig.vdf for Owned Games
        let userdata_root = steam_root.join("userdata");
        if let Ok(user_dirs) = fs::read_dir(&userdata_root) {
            for udir in user_dirs.flatten() {
                let cfg_dir = udir.path().join("config");
                if !cfg_dir.is_dir() {
                    continue;
                }
                // Scan librarycache/*.json
                let cache_dir = cfg_dir.join("librarycache");
                if let Ok(cache_files) = fs::read_dir(&cache_dir) {
                    for cf in cache_files.flatten() {
                        let cpath = cf.path();
                        let Some(stem) = cpath.file_stem().and_then(|s| s.to_str()) else {
                            continue;
                        };
                        let Ok(app_id) = stem.parse::<u32>() else {
                            continue;
                        };
                        let ach_count = fs::read_to_string(&cpath)
                            .ok()
                            .map(|s| s.matches("\"strID\"").count())
                            .unwrap_or(0);
                        let (title, engine_hint) = known_steam_app_metadata(app_id)
                            .map(|(t, e)| (t.to_string(), e.to_string()))
                            .unwrap_or_else(|| {
                                (
                                    format!("Steam App {app_id}"),
                                    "Windows x64 PC Game (Auto-Detect on Download)".to_string(),
                                )
                            });
                        let entry = owned_map.entry(app_id).or_insert_with(|| OwnedSteamGame {
                            app_id,
                            title,
                            engine_hint,
                            installed: false,
                            install_source: None,
                            playtime_minutes: 0,
                            last_played_unix: 0,
                            achievements_cached: ach_count,
                        });
                        if ach_count > entry.achievements_cached {
                            entry.achievements_cached = ach_count;
                        }
                    }
                }

                // Scan localconfig.vdf for Playtime and LastPlayed
                let localconfig_path = cfg_dir.join("localconfig.vdf");
                if let Ok(raw) = fs::read_to_string(&localconfig_path) {
                    let vdf = parse_vdf(&raw);
                    if let Some(apps) = vdf
                        .get_path(&["UserLocalConfigStore", "Software", "Valve", "Steam", "apps"])
                        .and_then(|v| v.as_map())
                    {
                        for (app_id_str, app_val) in apps {
                            let Ok(app_id) = app_id_str.parse::<u32>() else {
                                continue;
                            };
                            let playtime = app_val
                                .get_str("Playtime")
                                .and_then(|s| s.parse::<u64>().ok())
                                .unwrap_or(0);
                            let last_played = app_val
                                .get_str("LastPlayed")
                                .and_then(|s| s.parse::<u64>().ok())
                                .unwrap_or(0);
                            // Include apps that either have known metadata, librarycache presence, or non-zero playtime
                            if known_steam_app_metadata(app_id).is_some()
                                || owned_map.contains_key(&app_id)
                                || playtime > 0
                            {
                                let (title, engine_hint) = known_steam_app_metadata(app_id)
                                    .map(|(t, e)| (t.to_string(), e.to_string()))
                                    .unwrap_or_else(|| {
                                        (
                                            format!("Steam App {app_id}"),
                                            "Windows x64 PC Game".to_string(),
                                        )
                                    });
                                let entry =
                                    owned_map.entry(app_id).or_insert_with(|| OwnedSteamGame {
                                        app_id,
                                        title,
                                        engine_hint,
                                        installed: false,
                                        install_source: None,
                                        playtime_minutes: playtime,
                                        last_played_unix: last_played,
                                        achievements_cached: 0,
                                    });
                                entry.playtime_minutes = entry.playtime_minutes.max(playtime);
                                entry.last_played_unix = entry.last_played_unix.max(last_played);
                            }
                        }
                    }
                }
            }
        }
    }

    // 4. Scan Heroic / Epic Games (`installed.json`)
    let heroic_installed =
        home.join("Library/Application Support/heroic/legendaryConfig/legendary/installed.json");
    if let Ok(raw) = fs::read_to_string(&heroic_installed) {
        for game in parse_heroic_installed_json(&raw, &db) {
            report.installed_games.push(game);
        }
    }

    // 5. Scan Port Factory's own downloaded Windows Steam library (`~/Games/PortFactory/Steam`)
    let pf_steam_dir = default_steam_downloads_dir();
    if pf_steam_dir.is_dir() {
        report.library_folders.push(pf_steam_dir.clone());
        for game in scan_port_factory_downloads_dir(&pf_steam_dir, &db) {
            if let Ok(app_id_num) = game.app_id.parse::<u32>() {
                let entry = owned_map.entry(app_id_num).or_insert_with(|| OwnedSteamGame {
                    app_id: app_id_num,
                    title: game.name.clone(),
                    engine_hint: game.engine_profile.clone(),
                    installed: true,
                    install_source: Some(game.source.label()),
                    playtime_minutes: 0,
                    last_played_unix: 0,
                    achievements_cached: 0,
                });
                entry.installed = true;
                entry.install_source = Some(game.source.label());
            }
            report.installed_games.push(game);
        }
    }

    // Sort owned games: installed first, then known titles, then by playtime descending
    let mut owned_list: Vec<OwnedSteamGame> = owned_map.into_values().collect();
    owned_list.sort_by(|a, b| {
        b.installed
            .cmp(&a.installed)
            .then_with(|| {
                let a_named = !a.title.starts_with("Steam App ");
                let b_named = !b.title.starts_with("Steam App ");
                b_named.cmp(&a_named)
            })
            .then_with(|| b.playtime_minutes.cmp(&a.playtime_minutes))
            .then_with(|| a.app_id.cmp(&b.app_id))
    });
    report.owned_games = owned_list;

    Ok(report)
}

fn resolve_library_folder_path(steam_root: &Path, raw_path: &str) -> PathBuf {
    if raw_path.starts_with('/') {
        return PathBuf::from(raw_path);
    }
    // Windows path inside a CrossOver bottle, e.g. `C:\Program Files (x86)\Steam`
    if raw_path.len() >= 3 && raw_path.as_bytes()[1] == b':' {
        // Find `drive_c` ancestor of `steam_root`
        for anc in steam_root.ancestors() {
            if anc.file_name().and_then(|s| s.to_str()) == Some("drive_c") {
                let rel = raw_path[2..].trim_start_matches(['\\', '/']).replace('\\', "/");
                return anc.join(rel);
            }
        }
    }
    PathBuf::from(raw_path)
}

fn inspect_acf_manifest(
    acf_path: &Path,
    steamapps_dir: &Path,
    source: GameLibrarySource,
    db: &Win32Database,
) -> Result<InstalledGameEntry> {
    let raw = fs::read_to_string(acf_path)
        .with_context(|| format!("reading {}", acf_path.display()))?;
    let vdf = parse_vdf(&raw);
    let app_state = vdf
        .get("AppState")
        .ok_or_else(|| anyhow::anyhow!("missing AppState in {}", acf_path.display()))?;

    let app_id = app_state.get_str("appid").unwrap_or("0").to_string();
    let name = app_state
        .get_str("name")
        .unwrap_or("Unknown Steam Game")
        .to_string();
    let installdir = app_state.get_str("installdir").unwrap_or(&name);
    let size_on_disk_bytes = app_state
        .get_str("SizeOnDisk")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let build_id = app_state.get_str("buildid").unwrap_or("0").to_string();
    let launcher_path = app_state.get_str("LauncherPath").unwrap_or("");
    let install_dir = steamapps_dir.join("common").join(installdir);

    inspect_game_directory(
        &app_id,
        &name,
        source,
        &install_dir,
        size_on_disk_bytes,
        &build_id,
        launcher_path,
        db,
    )
}

fn inspect_windows_steam_client(
    win_steam_dir: &Path,
    bottle_name: &str,
    db: &Win32Database,
) -> Result<InstalledGameEntry> {
    let steam_exe = win_steam_dir.join("steam.exe");
    let tier0 = win_steam_dir.join("tier0_s64.dll");
    let vstdlib = win_steam_dir.join("vstdlib_s64.dll");
    let mut files = vec![steam_exe.clone()];
    if tier0.is_file() {
        files.push(tier0);
    }
    if vstdlib.is_file() {
        files.push(vstdlib);
    }
    let (total_imports, resolved_imports) = audit_pe_set(&files, db);
    let size_bytes = fs::metadata(&steam_exe).map(|m| m.len()).unwrap_or(0);

    Ok(InstalledGameEntry {
        app_id: "steam-client-win64".to_string(),
        name: "Steam Client (Windows x64 PE32+)".to_string(),
        source: GameLibrarySource::WindowsSteamClient {
            bottle_name: bottle_name.to_string(),
        },
        install_dir: win_steam_dir.to_path_buf(),
        size_on_disk_bytes: size_bytes,
        build_id: "win64-cef".to_string(),
        platform: "Windows x86-64 (PE32+)".to_string(),
        primary_exe: Some(steam_exe),
        pe_count: files.len(),
        total_imports,
        resolved_imports,
        engine_profile: "Valve Steam Client (vstdlib_s64 + tier0_s64 + SDL3)".to_string(),
        post_install: PostInstallSummary::default(),
    })
}

fn inspect_game_directory(
    app_id: &str,
    name: &str,
    source: GameLibrarySource,
    install_dir: &Path,
    size_on_disk_bytes: u64,
    build_id: &str,
    launcher_hint: &str,
    db: &Win32Database,
) -> Result<InstalledGameEntry> {
    let post_install = scan_post_install(install_dir);
    let pe_files = if install_dir.is_dir() {
        collect_pe_files(install_dir).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Filter out `_CommonRedist` installer executables and third-party optional upscaler DLLs
    // that are overridden by our native builtins (`libxess.dll` -> `nvngx_dlss.dll`).
    let runtime_pe_files: Vec<PathBuf> = pe_files
        .iter()
        .filter(|p| {
            let s = p.to_string_lossy().to_ascii_lowercase();
            !s.contains("_commonredist")
                && !s.contains("dxsetup")
                && !s.contains("vc_redist")
                && !s.ends_with("libxess.dll")
        })
        .cloned()
        .collect();

    let primary_exe = choose_primary_exe(&runtime_pe_files, name);
    let (total_imports, resolved_imports) = audit_pe_set(&runtime_pe_files, db);

    let mut engine_profile = String::from("Native macOS Universal / ARM64 Bundle");
    let mut platform = if launcher_hint.contains(":\\")
        || matches!(
            source,
            GameLibrarySource::CrossOverSteam { .. }
                | GameLibrarySource::HeroicEpic
                | GameLibrarySource::PortFactoryLibrary
        )
        || !runtime_pe_files.is_empty()
    {
        "Windows x86-64 (PE32+)".to_string()
    } else {
        "macOS Native (.app)".to_string()
    };

    if let Some(exe) = &primary_exe {
        platform = "Windows x86-64 (PE32+)".to_string();
        if let Ok(id_num) = app_id.parse::<u32>() {
            if let Some((_, hint)) = known_steam_app_metadata(id_num) {
                engine_profile = hint.to_string();
            }
        }
        if engine_profile == "Native macOS Universal / ARM64 Bundle" {
            if let Ok(bytes) = fs::read(exe) {
                if let Ok(pe) = Pe::parse(&bytes) {
                    if let Some(best) = detect_pe_game_profile(&pe) {
                        engine_profile = format!("{} ({})", best.profile_id, best.engine_family);
                    } else {
                        engine_profile = "Windows x86-64 DirectX 12 / 11 Engine".to_string();
                    }
                }
            }
        }
    } else if let Ok(id_num) = app_id.parse::<u32>() {
        if let Some((_, hint)) = known_steam_app_metadata(id_num) {
            engine_profile = hint.to_string();
        }
    }

    Ok(InstalledGameEntry {
        app_id: app_id.to_string(),
        name: name.to_string(),
        source,
        install_dir: install_dir.to_path_buf(),
        size_on_disk_bytes,
        build_id: build_id.to_string(),
        platform,
        primary_exe,
        pe_count: runtime_pe_files.len(),
        total_imports,
        resolved_imports,
        engine_profile,
        post_install,
    })
}

fn choose_primary_exe(pe_files: &[PathBuf], game_name: &str) -> Option<PathBuf> {
    let mut exes: Vec<&PathBuf> = pe_files
        .iter()
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("exe"))
        })
        .collect();
    if exes.is_empty() {
        return None;
    }
    let clean_game: String = game_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    exes.sort_by(|a, b| {
        let a_name = a
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let b_name = b
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let a_match = !clean_game.is_empty() && a_name.contains(&clean_game);
        let b_match = !clean_game.is_empty() && b_name.contains(&clean_game);
        b_match.cmp(&a_match).then_with(|| {
            let a_sz = fs::metadata(a).map(|m| m.len()).unwrap_or(0);
            let b_sz = fs::metadata(b).map(|m| m.len()).unwrap_or(0);
            b_sz.cmp(&a_sz)
        })
    });
    exes.first().map(|p| (*p).clone())
}

fn audit_pe_set(files: &[PathBuf], db: &Win32Database) -> (usize, usize) {
    let mut sibling_exports: HashMap<(String, String), u32> = HashMap::new();
    for f in files {
        if let Ok(bytes) = fs::read(f) {
            if let Ok(pe) = Pe::parse(&bytes) {
                let dll_name = pe_image::normalize_dll_name(
                    f.file_name().and_then(|s| s.to_str()).unwrap_or(""),
                );
                if let Ok(exps) = pe.exports() {
                    for e in exps.entries {
                        let rva = match e.target {
                            ExportTarget::Rva(r) => r,
                            ExportTarget::Forwarder(_) => 0,
                        };
                        if let Some(n) = e.name {
                            sibling_exports.insert((dll_name.clone(), n), rva);
                        }
                        sibling_exports.insert((dll_name.clone(), format!("#{}", e.ordinal)), rva);
                    }
                }
            }
        }
    }

    let mut total = 0usize;
    let mut resolved = 0usize;
    for f in files {
        let Ok(bytes) = fs::read(f) else {
            continue;
        };
        let name = f
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.pe");
        if let Ok(audit) = audit_pe_imports(name, &bytes, db, Some(&sibling_exports)) {
            total += audit.total_imports;
            resolved += audit.resolved_win32;
        }
    }
    (total, resolved)
}

fn parse_heroic_installed_json(raw: &str, db: &Win32Database) -> Vec<InstalledGameEntry> {
    let mut out = Vec::new();
    // Lightweight extraction of `"app_name"`, `"title"`, `"install_path"`, `"install_size"`, `"version"`, `"executable"`
    for chunk in raw.split("\"app_name\"").skip(1) {
        let app_id = extract_json_str_after_colon(chunk).unwrap_or_default();
        let title = extract_json_field(chunk, "title").unwrap_or_else(|| app_id.clone());
        let install_path = extract_json_field(chunk, "install_path").unwrap_or_default();
        let version = extract_json_field(chunk, "version").unwrap_or_default();
        let size_bytes = extract_json_u64(chunk, "install_size").unwrap_or(0);
        if install_path.is_empty() {
            continue;
        }
        let dir = PathBuf::from(&install_path);
        if let Ok(entry) = inspect_game_directory(
            &app_id,
            &title,
            GameLibrarySource::HeroicEpic,
            &dir,
            size_bytes,
            &version,
            "Windows",
            db,
        ) {
            out.push(entry);
        }
    }
    out
}

fn extract_json_str_after_colon(s: &str) -> Option<String> {
    let (_, after_colon) = s.split_once(':')?;
    let trimmed = after_colon.trim_start();
    let rest = trimmed.strip_prefix('"')?;
    let (val, _) = rest.split_once('"')?;
    Some(val.to_string())
}

fn extract_json_field(s: &str, field: &str) -> Option<String> {
    let needle = format!("\"{field}\"");
    let (_, after) = s.split_once(&needle)?;
    extract_json_str_after_colon(after)
}

fn extract_json_u64(s: &str, field: &str) -> Option<u64> {
    let needle = format!("\"{field}\"");
    let (_, after) = s.split_once(&needle)?;
    let (_, after_colon) = after.split_once(':')?;
    let digits: String = after_colon
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse::<u64>().ok()
}

// ============================================================================
// 5. Post-Download / Post-Install (`installscript.vdf` & `_CommonRedist`) Handler
// ============================================================================

/// Scan a game directory for `installscript.vdf` / `*install*.vdf` and `_CommonRedist`
/// post-download installers, extracting all required virtual registry keys and
/// identifying redistributables natively satisfied by `win32-rt`.
pub fn scan_post_install(game_dir: &Path) -> PostInstallSummary {
    let mut summary = PostInstallSummary::default();
    if !game_dir.is_dir() {
        return summary;
    }

    let mut stack = vec![(game_dir.to_path_buf(), 0usize)];
    while let Some((dir, depth)) = stack.pop() {
        if depth > 3 {
            continue;
        }
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let fname = entry.file_name().to_string_lossy().into_owned();
            let lower = fname.to_ascii_lowercase();
            if path.is_dir() {
                if lower == "_commonredist" || lower == "redist" || lower == "support" {
                    summary.common_redist_dirs.push(path.clone());
                }
                if depth < 3 && !lower.starts_with('.') && lower != "data" && lower != "data_pack2"
                {
                    stack.push((path, depth + 1));
                }
            } else if lower.ends_with(".vdf") && lower.contains("install") {
                if let Ok(raw) = fs::read_to_string(&path) {
                    let parsed = parse_installscript_vdf(&raw);
                    summary.installscript_files.push(path);
                    summary.registry_entries.extend(parsed.registry_entries);
                    summary.run_processes.extend(parsed.run_processes);
                    for b in parsed.bypassable_redistributables {
                        if !summary.bypassable_redistributables.contains(&b) {
                            summary.bypassable_redistributables.push(b);
                        }
                    }
                }
            } else if (lower.contains("vc_redist")
                || lower.contains("vcredist")
                || lower.contains("dxsetup")
                || lower.contains("dotnet")
                || lower.contains("ndp4")
                || lower.contains("oalinst")
                || lower.contains("physx"))
                && lower.ends_with(".exe")
            {
                let label = format!("{fname} (Natively provided by win32-rt runtime)");
                if !summary.bypassable_redistributables.contains(&label) {
                    summary.bypassable_redistributables.push(label);
                }
            }
        }
    }

    summary
}

/// Parse a Valve `installscript.vdf` file to extract `Registry` entries and `Run Process` steps.
pub fn parse_installscript_vdf(raw: &str) -> PostInstallSummary {
    let vdf = parse_vdf(raw);
    let mut summary = PostInstallSummary::default();

    let Some(root_entries) = vdf.as_map() else {
        return summary;
    };
    for (_, root_val) in root_entries {
        let Some(sections) = root_val.as_map() else {
            continue;
        };
        for (sec_name, sec_val) in sections {
            if sec_name.eq_ignore_ascii_case("Registry") {
                if let Some(reg_keys) = sec_val.as_map() {
                    for (key_path, key_body) in reg_keys {
                        collect_vdf_registry_entries(key_path, key_body, &mut summary.registry_entries);
                    }
                }
            } else if sec_name.eq_ignore_ascii_case("Run Process") {
                if let Some(procs) = sec_val.as_map() {
                    for (proc_name, proc_body) in procs {
                        let process = proc_body
                            .get_str("process 1")
                            .or_else(|| proc_body.get_str("process"))
                            .unwrap_or(proc_name);
                        let args = proc_body
                            .get_str("command 1")
                            .or_else(|| proc_body.get_str("command"))
                            .unwrap_or("");
                        let desc = if args.is_empty() {
                            format!("{proc_name}: {process}")
                        } else {
                            format!("{proc_name}: {process} {args}")
                        };
                        summary.run_processes.push(desc);

                        let lower = process.to_ascii_lowercase();
                        if lower.contains("vcredist")
                            || lower.contains("vc_redist")
                            || lower.contains("dxsetup")
                            || lower.contains("dotnet")
                            || lower.contains("physx")
                        {
                            summary.bypassable_redistributables.push(format!(
                                "{proc_name} ({process} -> bypassed, native in win32-rt)"
                            ));
                        }
                    }
                }
            }
        }
    }
    summary
}

fn collect_vdf_registry_entries(
    key_path: &str,
    key_body: &VdfValue,
    out: &mut Vec<SteamRegistryEntry>,
) {
    let Some(items) = key_body.as_map() else {
        return;
    };
    let normalized_key = normalize_vdf_reg_path(key_path);
    for (type_or_sub, val_node) in items {
        if type_or_sub.eq_ignore_ascii_case("string") {
            if let Some(kv_list) = val_node.as_map() {
                for (k, v) in kv_list {
                    if let Some(s) = v.as_str() {
                        out.push(SteamRegistryEntry {
                            key_path: normalized_key.clone(),
                            value_name: k.clone(),
                            value_type: "REG_SZ".to_string(),
                            value_data: s.to_string(),
                        });
                    }
                }
            }
        } else if type_or_sub.eq_ignore_ascii_case("dword") {
            if let Some(kv_list) = val_node.as_map() {
                for (k, v) in kv_list {
                    if let Some(s) = v.as_str() {
                        out.push(SteamRegistryEntry {
                            key_path: normalized_key.clone(),
                            value_name: k.clone(),
                            value_type: "REG_DWORD".to_string(),
                            value_data: s.to_string(),
                        });
                    }
                }
            }
        }
    }
}

fn normalize_vdf_reg_path(raw: &str) -> String {
    let mut s = raw.trim().replace('/', "\\");
    for (long, short) in [
        ("HKEY_LOCAL_MACHINE\\", "hklm\\"),
        ("HKEY_CURRENT_USER\\", "hkcu\\"),
        ("HKEY_CLASSES_ROOT\\", "hkcr\\"),
        ("HKEY_USERS\\", "hku\\"),
        ("HKEY_CURRENT_CONFIG\\", "hkcc\\"),
    ] {
        if s.len() >= long.len() && s[..long.len()].eq_ignore_ascii_case(long) {
            s = format!("{short}{}", &s[long.len()..]);
            break;
        }
    }
    s.to_ascii_lowercase()
}

/// Apply post-download / post-install steps for a Steam game directory:
/// 1. Writes `steam_appid.txt` so `steam_api64.dll` knows the exact AppID.
/// 2. Extracts all `installscript.vdf` registry entries + standard Steam/Redistributable
///    completion flags (`HasRunKey`) into `<out_dir>/steam_registry.tsv`.
pub fn apply_post_install(
    game_dir: &Path,
    app_id: &str,
    out_dir: &Path,
) -> Result<PostInstallSummary> {
    fs::create_dir_all(out_dir)?;
    let mut summary = scan_post_install(game_dir);

    // 1. Write steam_appid.txt if numeric AppID
    if app_id.chars().all(|c| c.is_ascii_digit()) && !app_id.is_empty() && app_id != "0" {
        let appid_file = out_dir.join("steam_appid.txt");
        let _ = fs::write(&appid_file, format!("{app_id}\n"));
        if game_dir.is_dir() {
            let _ = fs::write(game_dir.join("steam_appid.txt"), format!("{app_id}\n"));
        }
        // Also seed Steam's HasRunKey in registry so Steam never re-runs _CommonRedist installers
        summary.registry_entries.push(SteamRegistryEntry {
            key_path: format!("hkcu\\software\\valve\\steam\\apps\\{app_id}"),
            value_name: "Installed".to_string(),
            value_type: "REG_DWORD".to_string(),
            value_data: "1".to_string(),
        });
    }

    // 2. Write steam_registry.tsv for `win32-rt::advapi32::Registry`
    let reg_tsv_path = out_dir.join("steam_registry.tsv");
    let mut f = fs::File::create(&reg_tsv_path)
        .with_context(|| format!("creating {}", reg_tsv_path.display()))?;
    writeln!(
        f,
        "# Virtual Registry manifest generated by port-factory steam-postinstall"
    )?;
    writeln!(f, "key_path\tvalue_name\tvalue_type\tvalue_data")?;
    for entry in &summary.registry_entries {
        writeln!(
            f,
            "{}\t{}\t{}\t{}",
            entry.key_path, entry.value_name, entry.value_type, entry.value_data
        )?;
    }

    Ok(summary)
}

// ============================================================================
// 6. Windows Depot Downloader (`steam-setup`, `steam-download`, `steam-port`)
// ============================================================================

const DEPOT_DOWNLOADER_FALLBACK_TAG: &str = "DepotDownloader_3.4.0";

/// Default directory where Port Factory downloads Windows x64 Steam depots.
pub fn default_steam_downloads_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join("Games/PortFactory/Steam")
}

/// Default directory where packaged `.app` bundles live inside the workspace (`<ws>/out/apps`).
pub fn default_apps_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join("out/apps")
}

/// Sanitize a game title for safe use as a macOS `.app` or directory name.
pub fn sanitize_game_title(name: &str) -> String {
    let clean: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .trim()
        .to_string();
    if clean.is_empty() {
        "ConvertedGame".to_string()
    } else {
        clean
    }
}

/// Scan Port Factory's managed Windows Steam download directory (`~/Games/PortFactory/Steam`)
/// so games downloaded via `steam-download` / `steam-port` appear immediately in library scans.
pub fn scan_port_factory_downloads_dir(
    root_dir: &Path,
    db: &Win32Database,
) -> Vec<InstalledGameEntry> {
    let mut out = Vec::new();
    let Ok(rd) = fs::read_dir(root_dir) else {
        return out;
    };
    let mut subdirs: Vec<PathBuf> = rd
        .flatten()
        .filter_map(|e| {
            let ft = e.file_type().ok()?;
            (ft.is_dir() && !ft.is_symlink()).then(|| e.path())
        })
        .collect();
    subdirs.sort();

    for dir in subdirs {
        let mut app_id = String::new();
        let mut title = String::new();

        if let Ok(meta) = fs::read_to_string(dir.join(".port-factory-game.tsv")) {
            for line in meta.lines() {
                if let Some((k, v)) = line.split_once('\t') {
                    match k.trim() {
                        "app_id" => app_id = v.trim().to_string(),
                        "title" => title = v.trim().to_string(),
                        _ => {}
                    }
                }
            }
        }
        if app_id.is_empty() {
            if let Ok(raw) = fs::read_to_string(dir.join("steam_appid.txt")) {
                let trimmed = raw.trim();
                if trimmed.chars().all(|c| c.is_ascii_digit()) && !trimmed.is_empty() {
                    app_id = trimmed.to_string();
                }
            }
        }
        if title.is_empty() {
            if let Ok(num) = app_id.parse::<u32>() {
                if let Some((known_title, _)) = known_steam_app_metadata(num) {
                    title = known_title.to_string();
                }
            }
        }
        if title.is_empty() {
            title = dir
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("SteamGame")
                .to_string();
        }
        if app_id.is_empty() {
            app_id = "custom".to_string();
        }

        let size_bytes = dir_size_bytes_shallow_fast(&dir);
        if let Ok(entry) = inspect_game_directory(
            &app_id,
            &title,
            GameLibrarySource::PortFactoryLibrary,
            &dir,
            size_bytes,
            "depot",
            "Windows",
            db,
        ) {
            if entry.primary_exe.is_some() || app_id != "custom" {
                out.push(entry);
            }
        }
    }
    out
}

fn dir_size_bytes_shallow_fast(dir: &Path) -> u64 {
    #[cfg(unix)]
    {
        if let Ok(out) = Command::new("du")
            .args(["-sk", &dir.to_string_lossy()])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                if let Some(kb) = s
                    .split_whitespace()
                    .next()
                    .and_then(|tok| tok.parse::<u64>().ok())
                {
                    return kb.saturating_mul(1024);
                }
            }
        }
    }
    0
}

/// Locate an installed `DepotDownloader` binary across env override, sibling `.app` directory,
/// `~/.local/opt/depotdownloader`, `~/.local/bin`, `$PATH`, and standard prefixes.
pub fn find_depot_downloader() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("PORT_DEPOT_DOWNLOADER")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
    {
        if p.is_file() {
            return Some(p);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for name in ["DepotDownloader", "depotdownloader"] {
                let cand = dir.join(name);
                if cand.is_file() {
                    return Some(cand);
                }
            }
        }
    }
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        for rel in [
            ".local/opt/depotdownloader/DepotDownloader",
            ".local/bin/DepotDownloader",
            ".local/bin/depotdownloader",
        ] {
            let cand = home.join(rel);
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    which_binary("DepotDownloader").or_else(|| which_binary("depotdownloader"))
}

/// Parse the official `SteamRE/DepotDownloader` GitHub release JSON to extract
/// `(tag_name, download_url)` for the native macOS archive matching `arch` (`"arm64"` or `"x64"`).
pub fn parse_depot_downloader_release_asset(
    release_json: &str,
    arch: &str,
) -> Option<(String, String)> {
    let tag = extract_json_field(release_json, "tag_name")
        .unwrap_or_else(|| DEPOT_DOWNLOADER_FALLBACK_TAG.to_string());
    let target_name = format!("DepotDownloader-macos-{arch}.zip");
    for chunk in release_json.split("\"browser_download_url\"").skip(1) {
        if let Some(url) = extract_json_str_after_colon(chunk) {
            if url.ends_with(&target_name) {
                return Some((tag, url));
            }
        }
    }
    None
}

/// Download and install the standalone native macOS `SteamRE/DepotDownloader` release
/// (zero `.NET` runtime dependency; native Apple Silicon `arm64` binary) into
/// `~/.local/opt/depotdownloader/DepotDownloader` (or `install_dir`) and symlink `~/.local/bin/DepotDownloader`.
pub fn install_depot_downloader(install_dir: Option<&Path>) -> Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME environment variable is not set")?;
    let target_dir = install_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| home.join(".local/opt/depotdownloader"));
    fs::create_dir_all(&target_dir)
        .with_context(|| format!("creating {}", target_dir.display()))?;

    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x64"
    };
    let fallback_url = format!(
        "https://github.com/SteamRE/DepotDownloader/releases/download/{DEPOT_DOWNLOADER_FALLBACK_TAG}/DepotDownloader-macos-{arch}.zip"
    );

    let (tag, zip_url) = Command::new("/usr/bin/curl")
        .args([
            "-fsSL",
            "-H",
            "Accept: application/vnd.github+json",
            "-H",
            "User-Agent: port-factory",
            "https://api.github.com/repos/SteamRE/DepotDownloader/releases/latest",
        ])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let body = String::from_utf8_lossy(&o.stdout);
            parse_depot_downloader_release_asset(&body, arch)
        })
        .unwrap_or_else(|| (DEPOT_DOWNLOADER_FALLBACK_TAG.to_string(), fallback_url));

    println!("=== Installing Native macOS ({arch}) SteamRE/DepotDownloader ({tag}) ===");
    println!("  Download URL : {zip_url}");
    let tmp_zip = std::env::temp_dir().join(format!(
        "depotdownloader_{}_{}.zip",
        std::process::id(),
        arch
    ));
    let dl = Command::new("/usr/bin/curl")
        .args(["-fSL", "-o", &tmp_zip.to_string_lossy(), &zip_url])
        .status()
        .context("invoking /usr/bin/curl to download DepotDownloader")?;
    if !dl.success() {
        let _ = fs::remove_file(&tmp_zip);
        bail!("failed to download DepotDownloader from {zip_url} (exit {dl})");
    }

    let extract = Command::new("/usr/bin/unzip")
        .args([
            "-qo",
            &tmp_zip.to_string_lossy(),
            "-d",
            &target_dir.to_string_lossy(),
        ])
        .status()
        .context("invoking /usr/bin/unzip for DepotDownloader")?;
    let _ = fs::remove_file(&tmp_zip);
    if !extract.success() {
        bail!("failed to extract DepotDownloader archive into {}", target_dir.display());
    }

    let bin_path = target_dir.join("DepotDownloader");
    if !bin_path.is_file() {
        bail!(
            "extracted archive did not contain expected binary {}",
            bin_path.display()
        );
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&bin_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin_path, perms)?;
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("/usr/bin/xattr")
            .args(["-dr", "com.apple.quarantine", &target_dir.to_string_lossy()])
            .status();
    }

    // Best-effort symlink into ~/.local/bin/DepotDownloader when using default install dir.
    if install_dir.is_none() {
        let local_bin = home.join(".local/bin");
        if fs::create_dir_all(&local_bin).is_ok() {
            let link_path = local_bin.join("DepotDownloader");
            let _ = fs::remove_file(&link_path);
            let _ = std::os::unix::fs::symlink(&bin_path, &link_path);
        }
    }

    let probe = Command::new(&bin_path)
        .arg("--version")
        .output()
        .with_context(|| format!("verifying installed {}", bin_path.display()))?;
    let ver_line = String::from_utf8_lossy(&probe.stdout).trim().to_string();
    if !probe.status.success() {
        bail!(
            "installed {} failed --version verification (status {}): {}",
            bin_path.display(),
            probe.status,
            String::from_utf8_lossy(&probe.stderr).trim()
        );
    }

    println!("  Installed    : {} ({ver_line})", bin_path.display());
    Ok(bin_path)
}

/// Resolve a Steam AppID or game name against the scanned local + owned Steam library.
/// Returns `(app_id, title, existing_windows_install_dir)`.
pub fn resolve_steam_target_app_id(
    report: &SteamLibraryScanReport,
    query: &str,
) -> Result<(u32, String, Option<PathBuf>)> {
    let q = query.trim();
    if q.is_empty() {
        bail!("empty Steam AppID or game name");
    }

    if let Ok(num) = q.parse::<u32>() {
        let num_str = num.to_string();
        let existing_win = report
            .installed_games
            .iter()
            .find(|g| g.app_id == num_str && g.primary_exe.is_some())
            .map(|g| (g.name.clone(), g.install_dir.clone()));
        if let Some((name, dir)) = existing_win {
            return Ok((num, name, Some(dir)));
        }
        if let Some(owned) = report.owned_games.iter().find(|o| o.app_id == num) {
            return Ok((num, owned.title.clone(), None));
        }
        let title = known_steam_app_metadata(num)
            .map(|(t, _)| t.to_string())
            .unwrap_or_else(|| format!("Steam App {num}"));
        return Ok((num, title, None));
    }

    let q_lower = q.to_ascii_lowercase();
    if let Some(inst) = report
        .installed_games
        .iter()
        .find(|g| g.primary_exe.is_some() && g.name.to_ascii_lowercase().contains(&q_lower))
    {
        if let Ok(num) = inst.app_id.parse::<u32>() {
            return Ok((num, inst.name.clone(), Some(inst.install_dir.clone())));
        }
    }
    if let Some(owned) = report
        .owned_games
        .iter()
        .find(|o| o.title.to_ascii_lowercase().contains(&q_lower))
    {
        return Ok((owned.app_id, owned.title.clone(), None));
    }

    bail!(
        "could not resolve Steam game {q:?} (pass a numeric Steam AppID like `2357570` or a title from `steam-scan`)"
    )
}

/// Options for downloading or updating a Windows 64-bit Steam game depot on macOS.
#[derive(Debug, Clone)]
pub struct SteamDownloadOptions {
    pub app_id: u32,
    pub output_dir: PathBuf,
    pub username: Option<String>,
    pub branch: String,
    pub use_qr: bool,
    pub validate: bool,
    pub dry_run: bool,
}

fn parse_qr_cached_username(stdout: &str) -> Option<String> {
    const MARKER: &str = "login with -username ";
    let idx = stdout.find(MARKER)?;
    let rest = &stdout[idx + MARKER.len()..];
    let user = rest.split_whitespace().next()?.trim();
    (!user.is_empty()).then(|| user.to_string())
}

fn build_depot_downloader_args(opts: &SteamDownloadOptions, default_user: &str) -> Vec<String> {
    let mut args = vec![
        "-app".to_string(),
        opts.app_id.to_string(),
        "-os".to_string(),
        "windows".to_string(),
        "-osarch".to_string(),
        "64".to_string(),
        "-branch".to_string(),
        opts.branch.clone(),
        "-dir".to_string(),
        opts.output_dir.to_string_lossy().into_owned(),
    ];
    if opts.validate {
        args.push("-validate".to_string());
    }
    if opts.use_qr {
        args.push("-qr".to_string());
    } else if !default_user.eq_ignore_ascii_case("anonymous") {
        args.push("-username".to_string());
        args.push(default_user.to_string());
        args.push("-remember-password".to_string());
    }
    args
}

/// Download or verify/repair a Windows 64-bit (`-os windows -osarch 64`) Steam game by AppID
/// and run post-install preparation (`steam_appid.txt`, `steam_registry.tsv`, metadata stamp).
pub fn download_steam_game(
    workspace_root: &Path,
    opts: &SteamDownloadOptions,
) -> Result<PathBuf> {
    let scan = scan_all_steam_libraries(workspace_root)?;
    let default_user = opts
        .username
        .clone()
        .or_else(|| {
            scan.users
                .iter()
                .find(|u| u.most_recent)
                .map(|u| u.account_name.clone())
        })
        .or_else(|| scan.users.first().map(|u| u.account_name.clone()))
        .unwrap_or_else(|| "anonymous".to_string());

    let owned_title = scan
        .owned_games
        .iter()
        .find(|o| o.app_id == opts.app_id)
        .map(|o| o.title.clone());
    let (known_title, engine) = known_steam_app_metadata(opts.app_id)
        .unwrap_or(("Steam Game", "Windows x86-64 Engine"));
    let title = owned_title.unwrap_or_else(|| known_title.to_string());

    let dd_args = build_depot_downloader_args(opts, &default_user);

    println!("=== Port Factory Steam Windows-x64 Depot Downloader ===");
    println!("  AppID         : {} ({})", opts.app_id, title);
    println!("  Engine Profile: {}", engine);
    println!(
        "  Auth Mode     : {}",
        if opts.use_qr {
            "Steam Mobile App QR Code (-qr)".to_string()
        } else {
            format!("Steam Account ({default_user}, -remember-password)")
        }
    );
    println!("  Target OS/Arch: windows / x86_64 (64-bit)");
    println!("  Validate Files: {}", opts.validate);
    println!("  Destination   : {}", opts.output_dir.display());

    if opts.dry_run {
        println!("\n[Dry-Run] Planned download & post-install commands:");
        println!("  1. DepotDownloader {}", dd_args.join(" "));
        println!(
            "  2. port-factory steam-postinstall \"{}\" --appid {}",
            opts.output_dir.display(),
            opts.app_id
        );
        return Ok(opts.output_dir.clone());
    }

    fs::create_dir_all(&opts.output_dir)?;

    let depot_downloader = match find_depot_downloader() {
        Some(bin) => Some(bin),
        None => {
            if which_binary("steamcmd").is_some() {
                None
            } else {
                println!(
                    "  [Info] DepotDownloader not found; downloading official native macOS release..."
                );
                Some(install_depot_downloader(None)?)
            }
        }
    };

    if let Some(dd_bin) = depot_downloader {
        println!("  Backend       : DepotDownloader ({})", dd_bin.display());
        if opts.use_qr {
            // Stream stdout live while watching for DepotDownloader's known `-qr` race where QR
            // approval succeeds ("Success! Next time you can login with -username <acct> ...")
            // and a lingering QR poll disconnects Steam3 before manifests finish downloading.
            use std::io::Read as _;
            use std::process::Stdio;
            let mut child = Command::new(&dd_bin)
                .args(&dd_args)
                .stdout(Stdio::piped())
                .spawn()
                .with_context(|| format!("running {}", dd_bin.display()))?;
            let mut captured = String::new();
            if let Some(mut out) = child.stdout.take() {
                let mut buf = [0u8; 2048];
                let mut stdout_handle = std::io::stdout();
                loop {
                    match out.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let _ = stdout_handle.write_all(&buf[..n]);
                            let _ = stdout_handle.flush();
                            captured.push_str(&String::from_utf8_lossy(&buf[..n]));
                        }
                        Err(_) => break,
                    }
                }
            }
            let status = child
                .wait()
                .with_context(|| format!("waiting for {}", dd_bin.display()))?;
            if !status.success() {
                if let Some(authed_user) = parse_qr_cached_username(&captured) {
                    println!(
                        "  [Info] QR login succeeded for '{authed_user}' before transient disconnect; resuming automatically with cached token..."
                    );
                    let mut retry_opts = opts.clone();
                    retry_opts.use_qr = false;
                    retry_opts.username = Some(authed_user.clone());
                    let retry_args = build_depot_downloader_args(&retry_opts, &authed_user);
                    let retry_status = Command::new(&dd_bin)
                        .args(&retry_args)
                        .status()
                        .with_context(|| format!("retrying {}", dd_bin.display()))?;
                    if !retry_status.success() {
                        bail!("DepotDownloader exited with status {retry_status}");
                    }
                } else {
                    bail!("DepotDownloader exited with status {status}");
                }
            }
        } else {
            let status = Command::new(&dd_bin)
                .args(&dd_args)
                .status()
                .with_context(|| format!("running {}", dd_bin.display()))?;
            if !status.success() {
                bail!("DepotDownloader exited with status {status}");
            }
        }
    } else if let Some(scmd_bin) = which_binary("steamcmd") {
        println!("  Backend       : SteamCMD ({})", scmd_bin.display());
        let status = Command::new(&scmd_bin)
            .args([
                "+@sSteamCmdForcePlatformType",
                "windows",
                "+force_install_dir",
                &opts.output_dir.to_string_lossy(),
                "+login",
                &default_user,
                "+app_update",
                &opts.app_id.to_string(),
                "validate",
                "+quit",
            ])
            .status()
            .with_context(|| format!("running {}", scmd_bin.display()))?;
        if !status.success() {
            bail!("steamcmd exited with status {status}");
        }
    } else {
        bail!("neither DepotDownloader nor steamcmd is available");
    }

    let _ = fs::write(
        opts.output_dir.join(".port-factory-game.tsv"),
        format!("app_id\t{}\ntitle\t{}\n", opts.app_id, title),
    );
    let post = apply_post_install(&opts.output_dir, &opts.app_id.to_string(), &opts.output_dir)?;
    println!(
        "  Post-Install  : {} registry keys extracted, {} redistributables satisfied natively",
        post.registry_entries.len(),
        post.bypassable_redistributables.len()
    );
    Ok(opts.output_dir.clone())
}

/// Options for the one-command end-to-end Steam -> SSD-portable `.app` workflow (`steam-port`).
#[derive(Debug, Clone)]
pub struct SteamPortOptions {
    pub target: String,
    pub download_dir: Option<PathBuf>,
    pub output_app: Option<PathBuf>,
    pub username: Option<String>,
    pub branch: String,
    pub use_qr: bool,
    pub validate: bool,
    pub skip_download: bool,
    pub precompile_shaders: bool,
    pub copy_to: Option<PathBuf>,
    pub dry_run: bool,
}

/// Summary of an end-to-end `steam-port` run.
#[derive(Debug, Clone)]
pub struct SteamPortResult {
    pub app_id: u32,
    pub title: String,
    pub game_dir: PathBuf,
    pub app_bundle: PathBuf,
    pub verify_report: Option<BundleVerifyReport>,
    pub exported_path: Option<PathBuf>,
}

/// End-to-end Steam game port workflow:
/// 1. Download / update / repair Windows x64 depot via `DepotDownloader` (unless `skip_download`).
/// 2. Convert into a self-contained SSD-portable `.app` (`--copy-game --max-fns 0`).
/// 3. Verify portability with [`verify_app_bundle`].
/// 4. Optionally copy the verified `.app` to an external SSD volume (`copy_to`).
pub fn port_steam_game(
    workspace_root: &Path,
    opts: &SteamPortOptions,
) -> Result<SteamPortResult> {
    let report = scan_all_steam_libraries(workspace_root)?;
    let (app_id, title, existing_win_dir) = resolve_steam_target_app_id(&report, &opts.target)?;
    let clean_title = sanitize_game_title(&title);

    let game_dir = opts
        .download_dir
        .clone()
        .or(existing_win_dir.clone())
        .unwrap_or_else(|| default_steam_downloads_dir().join(&clean_title));
    let app_bundle = opts
        .output_app
        .clone()
        .unwrap_or_else(|| default_apps_dir(workspace_root).join(format!("{clean_title}.app")));

    if opts.dry_run {
        println!("=== Port Factory One-Command Steam Port Plan ===");
        println!("  Game          : {title} (AppID {app_id})");
        println!("  Game Dir      : {}", game_dir.display());
        println!("  Output Bundle : {}", app_bundle.display());
        if !opts.skip_download {
            let dl_opts = SteamDownloadOptions {
                app_id,
                output_dir: game_dir.clone(),
                username: opts.username.clone(),
                branch: opts.branch.clone(),
                use_qr: opts.use_qr,
                validate: opts.validate || existing_win_dir.is_some(),
                dry_run: true,
            };
            let _ = download_steam_game(workspace_root, &dl_opts)?;
        } else {
            println!("  Step 1        : skip download (use existing {})", game_dir.display());
        }
        println!(
            "  Step 2        : steam-convert \"{}\" --format app --copy-game --max-fns 0 --out \"{}\"",
            game_dir.display(),
            app_bundle.display()
        );
        println!("  Step 3        : bundle-verify \"{}\"", app_bundle.display());
        if let Some(ref dest) = opts.copy_to {
            println!(
                "  Step 4        : bundle-export \"{}\" \"{}\"",
                app_bundle.display(),
                dest.display()
            );
        }
        return Ok(SteamPortResult {
            app_id,
            title,
            game_dir,
            app_bundle,
            verify_report: None,
            exported_path: None,
        });
    }

    if !opts.skip_download {
        let dl_opts = SteamDownloadOptions {
            app_id,
            output_dir: game_dir.clone(),
            username: opts.username.clone(),
            branch: opts.branch.clone(),
            use_qr: opts.use_qr,
            validate: opts.validate || existing_win_dir.is_some(),
            dry_run: false,
        };
        download_steam_game(workspace_root, &dl_opts)?;
    }

    let convert_target = if existing_win_dir
        .as_ref()
        .is_some_and(|d| d == &game_dir)
    {
        app_id.to_string()
    } else {
        game_dir.to_string_lossy().into_owned()
    };

    let convert_opts = SteamConvertOptions {
        target: convert_target,
        format: ConversionFormat::MacOsApp,
        output_path: Some(app_bundle.clone()),
        symlink_game: false,
        precompile_shaders: opts.precompile_shaders,
        max_aot_functions: 0,
        bundle_shader_cache_from: None,
    };
    convert_steam_game(workspace_root, &convert_opts)?;

    let verify = verify_app_bundle(&app_bundle)?;
    print!("{}", format_bundle_verify_report(&verify));
    if !verify.passed() {
        bail!(
            "packaged bundle {} failed portability verification ({} failure(s))",
            app_bundle.display(),
            verify.fail_count()
        );
    }

    let exported_path = if let Some(ref dest) = opts.copy_to {
        Some(export_app_bundle(&app_bundle, dest, false)?)
    } else {
        None
    };

    Ok(SteamPortResult {
        app_id,
        title,
        game_dir,
        app_bundle,
        verify_report: Some(verify),
        exported_path,
    })
}

/// Summary of a packaged macOS `.app` bundle managed by Port Factory.
#[derive(Debug, Clone)]
pub struct PackagedBundleInfo {
    pub name: String,
    pub path: PathBuf,
    pub runtime: String,
    pub game_exe: String,
    pub aot_modules: usize,
    pub has_metal_irconverter: bool,
    pub has_dxilconv: bool,
    pub has_moltenvk: bool,
    pub is_symlinked_game: bool,
    pub size_bytes: u64,
}

/// Enumerate packaged `.app` bundles in `<workspace>/out/apps/` and `~/Games/PortFactory/Apps/`.
pub fn list_packaged_bundles(workspace_root: &Path) -> Vec<PackagedBundleInfo> {
    let mut dirs = vec![default_apps_dir(workspace_root)];
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        let user_apps = home.join("Games/PortFactory/Apps");
        if user_apps.is_dir() && !dirs.contains(&user_apps) {
            dirs.push(user_apps);
        }
    }

    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for root in dirs {
        let Ok(rd) = fs::read_dir(&root) else {
            continue;
        };
        let mut entries: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.is_dir()
                    && p.extension()
                        .and_then(|s| s.to_str())
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("app"))
            })
            .collect();
        entries.sort();

        for app in entries {
            let canon = fs::canonicalize(&app).unwrap_or_else(|_| app.clone());
            if !seen.insert(canon) {
                continue;
            }
            let contents = app.join("Contents");
            let macos = contents.join("MacOS");
            let fw = contents.join("Frameworks");
            let res = contents.join("Resources");

            let runtime = if macos.join("port-runner").is_file() {
                "port-runner".to_string()
            } else if macos.join("aw2-loader").is_file() {
                "aw2-loader".to_string()
            } else if macos.join("port-factory").is_file() {
                // Skip the Port Factory launcher bundle itself when listing ported game apps.
                continue;
            } else {
                "unknown".to_string()
            };

            let game_exe = fs::read_to_string(res.join("game.toml"))
                .ok()
                .and_then(|s| parse_toml_string_field(&s, "app", "exe"))
                .unwrap_or_default();

            let aot_modules = fs::read_dir(res.join("AotCache"))
                .ok()
                .map(|r| {
                    r.flatten()
                        .filter(|e| {
                            e.path()
                                .extension()
                                .and_then(|s| s.to_str())
                                == Some("aot")
                        })
                        .count()
                })
                .unwrap_or(0);

            let is_symlinked_game = fs::symlink_metadata(res.join("Game"))
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);

            let name = app
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("App")
                .to_string();

            out.push(PackagedBundleInfo {
                name,
                size_bytes: dir_size_bytes_shallow_fast(&app),
                path: app,
                runtime,
                game_exe,
                aot_modules,
                has_metal_irconverter: fw.join("libmetalirconverter.dylib").is_file(),
                has_dxilconv: fw.join("libdxilconv.dylib").is_file(),
                has_moltenvk: fw.join("libMoltenVK.dylib").is_file(),
                is_symlinked_game,
            });
        }
    }
    out
}

/// Mounted external volume under `/Volumes/` suitable as an SSD copy destination.
#[derive(Debug, Clone)]
pub struct ExternalVolumeInfo {
    pub name: String,
    pub path: PathBuf,
    pub free_bytes: u64,
    pub total_bytes: u64,
}

/// List non-system mounted volumes under `/Volumes/` (external SSDs, USB drives, disk images).
pub fn list_external_volumes() -> Vec<ExternalVolumeInfo> {
    let mut out = Vec::new();
    let volumes_root = Path::new("/Volumes");
    let Ok(rd) = fs::read_dir(volumes_root) else {
        return out;
    };
    let mut entries: Vec<PathBuf> = rd
        .flatten()
        .filter_map(|e| {
            let ft = e.file_type().ok()?;
            if ft.is_symlink() || !ft.is_dir() {
                return None;
            }
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with('.')
                || matches!(
                    name.as_str(),
                    "Recovery" | "Preboot" | "VM" | "Update" | "Macintosh HD"
                )
            {
                return None;
            }
            Some(e.path())
        })
        .collect();
    entries.sort();

    for vol in entries {
        let name = vol
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Volume")
            .to_string();
        let (total_bytes, free_bytes) = df_volume_bytes(&vol).unwrap_or((0, 0));
        out.push(ExternalVolumeInfo {
            name,
            path: vol,
            free_bytes,
            total_bytes,
        });
    }
    out
}

fn df_volume_bytes(path: &Path) -> Option<(u64, u64)> {
    let out = Command::new("/bin/df")
        .args(["-k", &path.to_string_lossy()])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.lines().nth(1)?;
    let cols: Vec<&str> = line.split_whitespace().collect();
    if cols.len() < 4 {
        return None;
    }
    let total_kb = cols[1].parse::<u64>().ok()?;
    let avail_kb = cols[3].parse::<u64>().ok()?;
    Some((total_kb.saturating_mul(1024), avail_kb.saturating_mul(1024)))
}

/// Verify and copy a self-contained `.app` bundle to an external SSD or target folder.
/// Refuses to export a non-portable bundle unless `force == true`.
pub fn export_app_bundle(app_path: &Path, dest: &Path, force: bool) -> Result<PathBuf> {
    if !app_path.is_dir() {
        bail!("source bundle does not exist: {}", app_path.display());
    }
    let report = verify_app_bundle(app_path)?;
    if !report.passed() && !force {
        bail!(
            "refusing to export non-portable bundle {} (pass --force to override):\n{}",
            app_path.display(),
            format_bundle_verify_report(&report)
        );
    }

    let app_name = app_path
        .file_name()
        .context("source bundle has no file name")?;
    let dest_app = if dest
        .extension()
        .and_then(|s| s.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("app"))
    {
        dest.to_path_buf()
    } else {
        fs::create_dir_all(dest)
            .with_context(|| format!("creating export destination {}", dest.display()))?;
        dest.join(app_name)
    };

    if let Some(parent) = dest_app.parent() {
        fs::create_dir_all(parent)?;
    }

    println!(
        "Exporting {} -> {} ...",
        app_path.display(),
        dest_app.display()
    );
    let rsync_status = Command::new("/usr/bin/rsync")
        .env("COPYFILE_DISABLE", "1")
        .args([
            "-aW",
            "--delete",
            &format!("{}/", app_path.display()),
            &format!("{}/", dest_app.display()),
        ])
        .status();

    let copied = match rsync_status {
        Ok(st) if st.success() => true,
        _ => {
            let _ = fs::remove_dir_all(&dest_app);
            Command::new("/bin/cp")
                .env("COPYFILE_DISABLE", "1")
                .args([
                    "-R",
                    &app_path.to_string_lossy(),
                    &dest_app.to_string_lossy(),
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
    };
    if !copied {
        bail!(
            "failed to copy {} to {}",
            app_path.display(),
            dest_app.display()
        );
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("/usr/bin/xattr")
            .args(["-dr", "com.apple.quarantine", &dest_app.to_string_lossy()])
            .status();
        let verify = Command::new("codesign")
            .args(["--verify", "--deep", "--strict", &dest_app.to_string_lossy()])
            .output();
        if let Ok(o) = verify {
            if !o.status.success() {
                eprintln!(
                    "warning: codesign verify on exported bundle {}: {}",
                    dest_app.display(),
                    String::from_utf8_lossy(&o.stderr).trim()
                );
            }
        }
    }

    println!("Exported verified bundle to {}", dest_app.display());
    Ok(dest_app)
}

fn which_binary(name: &str) -> Option<PathBuf> {
    if let Some(path_env) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_env) {
            let p = dir.join(name);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    for prefix in ["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"] {
        let p = Path::new(prefix).join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

// ============================================================================
// 7. One-Click Game Converter (`steam-convert`)
// ============================================================================

/// Target conversion format for `steam-convert`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionFormat {
    /// Self-contained macOS `.app` bundle with `port-runner`, `dxil-metald`,
    /// `steam_appid.txt`, `steam_registry.tsv`, prewarmed `.aot` block caches,
    /// and pre-converted Metal shaders.
    MacOsApp,
    /// Standalone embeddable Rust AOT crate (`Cargo.toml` + `src/lib.rs`).
    AotCrate,
    /// Pre-compiled Metal `.metallib` shader cache only.
    MetalShaders,
}

impl ConversionFormat {
    pub fn parse(s: &str) -> Result<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "app" | "macos" | "macos-app" | "bundle" | "hybrid" => Ok(Self::MacOsApp),
            "aot" | "aot-crate" | "rust" | "crate" => Ok(Self::AotCrate),
            "shaders" | "metal" | "metallib" | "shaders-only" => Ok(Self::MetalShaders),
            other => bail!(
                "unknown conversion format {other:?} (expected: app, aot-crate, or shaders)"
            ),
        }
    }
}

/// Options for one-click converting an installed Steam game or Windows `.exe` directory.
#[derive(Debug, Clone)]
pub struct SteamConvertOptions {
    pub target: String,
    pub format: ConversionFormat,
    pub output_path: Option<PathBuf>,
    pub symlink_game: bool,
    pub precompile_shaders: bool,
    pub max_aot_functions: usize,
    /// Optional host shader cache root (e.g. `~/Translator/ShaderCache`) whose portable
    /// Tier-1 entries (`ir*`, `manifest/`, `pso/pso_manifest/`, excluding host-specific
    /// `*.mtl4a` archives) are copied into `Contents/Resources/ShaderCache/`.
    pub bundle_shader_cache_from: Option<PathBuf>,
}

/// Environment inputs used to locate host runtime frameworks when staging an `.app` bundle.
#[derive(Debug, Clone)]
pub struct FrameworkDiscoveryEnv {
    pub home: Option<PathBuf>,
    pub metal_irconverter_dylib: Option<PathBuf>,
    pub aw2_dxilconv: Option<PathBuf>,
    pub port_vulkan_dylib: Option<PathBuf>,
    pub port_moltenvk_dylib: Option<PathBuf>,
    pub moltenvk_dylib: Option<PathBuf>,
    pub vulkan_sdk: Option<PathBuf>,
    /// Include standard macOS system prefixes (`/opt/homebrew/lib`, `/usr/local/lib`,
    /// `/opt/metal-shaderconverter/lib`). Set `false` in isolated unit tests.
    pub include_system_paths: bool,
}

impl Default for FrameworkDiscoveryEnv {
    fn default() -> Self {
        Self {
            home: None,
            metal_irconverter_dylib: None,
            aw2_dxilconv: None,
            port_vulkan_dylib: None,
            port_moltenvk_dylib: None,
            moltenvk_dylib: None,
            vulkan_sdk: None,
            include_system_paths: true,
        }
    }
}

impl FrameworkDiscoveryEnv {
    pub fn from_process_env() -> Self {
        let env_path = |key: &str| {
            std::env::var_os(key)
                .filter(|v| !v.is_empty())
                .map(PathBuf::from)
        };
        Self {
            home: env_path("HOME"),
            metal_irconverter_dylib: env_path("METAL_IRCONVERTER_DYLIB"),
            aw2_dxilconv: env_path("AW2_DXILCONV"),
            port_vulkan_dylib: env_path("PORT_VULKAN_DYLIB"),
            port_moltenvk_dylib: env_path("PORT_MOLTENVK_DYLIB"),
            moltenvk_dylib: env_path("MOLTENVK_DYLIB"),
            vulkan_sdk: env_path("VULKAN_SDK"),
            include_system_paths: true,
        }
    }
}

/// Runtime dynamic libraries copied into `<App>.app/Contents/Frameworks/`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StagedFrameworks {
    pub metal_irconverter: Option<PathBuf>,
    pub dxilconv: Option<PathBuf>,
    pub moltenvk: Option<PathBuf>,
}

impl StagedFrameworks {
    pub fn staged_names(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if self.metal_irconverter.is_some() {
            out.push("libmetalirconverter.dylib");
        }
        if self.dxilconv.is_some() {
            out.push("libdxilconv.dylib");
        }
        if self.moltenvk.is_some() {
            out.push("libMoltenVK.dylib");
        }
        out
    }
}

fn push_unique_path(out: &mut Vec<PathBuf>, candidate: PathBuf) {
    if !candidate.as_os_str().is_empty() && !out.contains(&candidate) {
        out.push(candidate);
    }
}

/// Ordered candidate search paths for `libmetalirconverter.dylib`.
pub fn metal_irconverter_candidates(env: &FrameworkDiscoveryEnv) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(ref p) = env.metal_irconverter_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref home) = env.home {
        push_unique_path(
            &mut out,
            home.join(".local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib"),
        );
    }
    if env.include_system_paths {
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/homebrew/lib/libmetalirconverter.dylib"),
        );
        push_unique_path(
            &mut out,
            PathBuf::from("/usr/local/lib/libmetalirconverter.dylib"),
        );
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/metal-shaderconverter/lib/libmetalirconverter.dylib"),
        );
    }
    out
}

/// Ordered candidate search paths for optional `libdxilconv.dylib` (DX11 SM 4/5 -> DXIL).
pub fn dxilconv_candidates(workspace_root: &Path, env: &FrameworkDiscoveryEnv) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(ref p) = env.aw2_dxilconv {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref home) = env.home {
        push_unique_path(&mut out, home.join(".local/lib/libdxilconv.dylib"));
    }
    push_unique_path(
        &mut out,
        workspace_root.join("third_party/DirectXShaderCompiler/build/lib/libdxilconv.dylib"),
    );
    if env.include_system_paths {
        push_unique_path(&mut out, PathBuf::from("/opt/homebrew/lib/libdxilconv.dylib"));
        push_unique_path(&mut out, PathBuf::from("/usr/local/lib/libdxilconv.dylib"));
    }
    out
}

/// Ordered candidate search paths for optional `libMoltenVK.dylib` (Vulkan -> Metal).
pub fn moltenvk_candidates(env: &FrameworkDiscoveryEnv) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(ref p) = env.port_vulkan_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref p) = env.port_moltenvk_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref p) = env.moltenvk_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if env.include_system_paths {
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/homebrew/opt/molten-vk/lib/libMoltenVK.dylib"),
        );
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/homebrew/lib/libMoltenVK.dylib"),
        );
        push_unique_path(&mut out, PathBuf::from("/usr/local/lib/libMoltenVK.dylib"));
    }
    if let Some(ref sdk) = env.vulkan_sdk {
        push_unique_path(&mut out, sdk.join("lib/libMoltenVK.dylib"));
    }
    if let Some(ref home) = env.home {
        push_unique_path(&mut out, home.join(".local/lib/libMoltenVK.dylib"));
        push_unique_path(
            &mut out,
            home.join("Library/Android/sdk/emulator/lib64/vulkan/libMoltenVK.dylib"),
        );
    }
    out
}

/// Returns `true` if `path` starts with a Mach-O or universal/fat Mach-O magic number.
pub fn is_macho_file(path: &Path) -> bool {
    use std::io::Read;
    let Ok(mut f) = fs::File::open(path) else {
        return false;
    };
    let mut magic = [0u8; 4];
    if f.read_exact(&mut magic).is_err() {
        return false;
    }
    matches!(
        u32::from_be_bytes(magic),
        0xFEED_FACE
            | 0xCEFA_EDFE
            | 0xFEED_FACF
            | 0xCFFA_EDFE
            | 0xCAFE_BABE
            | 0xBEBA_FECA
            | 0xCAFE_BABF
            | 0xBFBA_BECA
    )
}

/// Parse `(minos, sdk)` version pairs for all `platform MACOS` slices from `vtool -show-build`.
pub fn parse_vtool_macos_versions(stdout: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut in_macos = false;
    let mut cur_minos: Option<String> = None;
    let mut cur_sdk: Option<String> = None;

    let flush = |in_macos: &mut bool,
                 cur_minos: &mut Option<String>,
                 cur_sdk: &mut Option<String>,
                 out: &mut Vec<(String, String)>| {
        if *in_macos {
            if let Some(minos) = cur_minos.take() {
                let sdk = cur_sdk.take().unwrap_or_else(|| minos.clone());
                out.push((minos, sdk));
            }
        }
        *in_macos = false;
        *cur_minos = None;
        *cur_sdk = None;
    };

    for raw_line in stdout.lines() {
        let line = raw_line.trim();
        if line.starts_with("Load command ") || line.ends_with(':') {
            flush(&mut in_macos, &mut cur_minos, &mut cur_sdk, &mut out);
            continue;
        }
        if let Some(rest) = line.strip_prefix("platform ") {
            in_macos = rest.trim().eq_ignore_ascii_case("MACOS");
        } else if in_macos {
            if let Some(rest) = line.strip_prefix("minos ") {
                cur_minos = Some(rest.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("sdk ") {
                cur_sdk = Some(rest.trim().to_string());
            }
        }
    }
    flush(&mut in_macos, &mut cur_minos, &mut cur_sdk, &mut out);
    out
}

fn version_major(ver: &str) -> Option<u32> {
    ver.trim().split('.').next()?.parse::<u32>().ok()
}

/// Lower `LC_BUILD_VERSION` macOS `minos` to `14.0` (preserving `sdk`) when a staged dylib
/// was built on a newer beta/host macOS (`minos > 15`, e.g. `27.2`), so the bundle remains
/// portable to other Apple Silicon Macs running macOS 14/15/26+. Also normalizes non-relative
/// dylib install names to `@rpath/<filename>` for clean relocation.
fn normalize_staged_macho_framework(dst: &Path) -> Result<()> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = dst;
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        if !is_macho_file(dst) {
            return Ok(());
        }
        if let Ok(out) = Command::new("vtool")
            .args(["-show-build", &dst.to_string_lossy()])
            .output()
        {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let versions = parse_vtool_macos_versions(&stdout);
                if let Some((_, sdk)) = versions
                    .iter()
                    .find(|(minos, _)| version_major(minos).is_some_and(|m| m > 15))
                {
                    let dst_s = dst.to_string_lossy().into_owned();
                    let status = Command::new("vtool")
                        .args([
                            "-set-build-version",
                            "macos",
                            "14.0",
                            sdk.as_str(),
                            "-replace",
                            "-output",
                            &dst_s,
                            &dst_s,
                        ])
                        .status();
                    if let Ok(st) = status {
                        if !st.success() {
                            eprintln!(
                                "warning: vtool failed to lower minos on {} (status {st})",
                                dst.display()
                            );
                        }
                    }
                }
            }
        }

        // Normalize absolute install names (e.g. `/Users/.../libfoo.dylib` or
        // `/opt/homebrew/.../libfoo.dylib`) to `@rpath/<filename>`.
        if let Some(fname) = dst.file_name().and_then(|s| s.to_str()) {
            if let Ok(id_out) = Command::new("otool")
                .args(["-D", &dst.to_string_lossy()])
                .output()
            {
                if id_out.status.success() {
                    let needs_id_fix = String::from_utf8_lossy(&id_out.stdout)
                        .lines()
                        .map(str::trim)
                        .filter(|l| !l.is_empty() && !l.ends_with(':'))
                        .any(|id| {
                            id.starts_with('/')
                                && !id.starts_with("/System/")
                                && !id.starts_with("/usr/lib/")
                        });
                    if needs_id_fix {
                        let _ = Command::new("install_name_tool")
                            .args(["-id", &format!("@rpath/{fname}"), &dst.to_string_lossy()])
                            .status();
                    }
                }
            }
        }
        Ok(())
    }
}

fn stage_first_existing(candidates: &[PathBuf], dst: &Path) -> Result<Option<PathBuf>> {
    for src in candidates {
        if src.is_file() {
            // Remove any stale destination file or symlink first so re-staging over a read-only
            // copy or dangling symlink never fails.
            if dst.exists() || dst.is_symlink() {
                let _ = fs::remove_file(dst);
            }
            fs::copy(src, dst).with_context(|| {
                format!("copying framework {} -> {}", src.display(), dst.display())
            })?;
            // Ensure staged frameworks are owner-writable and loadable/executable (e.g. Homebrew
            // Cellar dylibs are often 0444/0555, which would block `codesign --force --sign -`).
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(meta) = fs::metadata(dst) {
                    let mut perms = meta.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(dst, perms).with_context(|| {
                        format!("setting 0755 permissions on staged {}", dst.display())
                    })?;
                }
            }
            normalize_staged_macho_framework(dst)?;
            return Ok(Some(dst.to_path_buf()));
        }
    }
    Ok(None)
}

/// Discover and copy `libmetalirconverter.dylib` and optional `libdxilconv.dylib` /
/// `libMoltenVK.dylib` into `frameworks_dir` using an explicit [`FrameworkDiscoveryEnv`].
pub fn stage_bundle_frameworks_with_env(
    frameworks_dir: &Path,
    workspace_root: &Path,
    env: &FrameworkDiscoveryEnv,
) -> Result<StagedFrameworks> {
    fs::create_dir_all(frameworks_dir)
        .with_context(|| format!("creating {}", frameworks_dir.display()))?;

    let metal_irconverter = stage_first_existing(
        &metal_irconverter_candidates(env),
        &frameworks_dir.join("libmetalirconverter.dylib"),
    )?;
    let dxilconv = stage_first_existing(
        &dxilconv_candidates(workspace_root, env),
        &frameworks_dir.join("libdxilconv.dylib"),
    )?;
    let moltenvk = stage_first_existing(
        &moltenvk_candidates(env),
        &frameworks_dir.join("libMoltenVK.dylib"),
    )?;

    Ok(StagedFrameworks {
        metal_irconverter,
        dxilconv,
        moltenvk,
    })
}

/// Discover and copy host runtime frameworks into `<App>.app/Contents/Frameworks/`.
pub fn stage_bundle_frameworks(
    frameworks_dir: &Path,
    workspace_root: &Path,
) -> Result<StagedFrameworks> {
    let env = FrameworkDiscoveryEnv::from_process_env();
    stage_bundle_frameworks_with_env(frameworks_dir, workspace_root, &env)
}

/// Parameters for generating the native ARM64 `.app` launcher (`Contents/MacOS/<exec_ident>`).
#[derive(Debug, Clone)]
pub struct BundleLauncherConfig<'a> {
    pub clean_title: &'a str,
    pub app_id: &'a str,
    pub exe_rel: &'a str,
    pub image_base: u64,
    pub size_of_image: u32,
    pub aot_fn_count: usize,
    pub aot_dylib_name: &'a str,
    pub extra_aot_dylibs: &'a [String],
}

/// Generate a valid macOS `Contents/Info.plist` for a self-contained `.app` bundle.
pub fn generate_info_plist(clean_title: &str, exec_ident: &str, build_id: &str) -> String {
    let title_xml = html_escape(clean_title);
    let exec_xml = html_escape(exec_ident);
    let bundle_id = format!(
        "com.portfactory.steam.{}",
        exec_ident.to_ascii_lowercase()
    );
    let version = if build_id.trim().is_empty() {
        "1"
    } else {
        build_id.trim()
    };
    let version_xml = html_escape(version);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>{title_xml}</string>
    <key>CFBundleExecutable</key>
    <string>{exec_xml}</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>{bundle_id}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{title_xml}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>{version_xml}</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.games</string>
    <key>LSMinimumSystemVersion</key>
    <string>14.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
"#
    )
}

/// Generate the declarative `Contents/Resources/game.toml` manifest for an `.app` bundle.
pub fn generate_game_manifest_toml(
    clean_title: &str,
    exec_ident: &str,
    exe_rel: &str,
    app_id: &str,
) -> String {
    format!(
        "[app]\nid = \"{ident_lower}\"\nname = \"{clean_title}\"\nexe = \"{exe_rel}\"\nwin_dir = \"C:\\\\Games\\\\{exec_ident}\"\nlog_name = \"{exec_ident}.log\"\nsteam_app_id = \"{app_id}\"\n\n[runtime]\naot_dir = \"AotCache\"\npreferred_base_retry = false\nv8_jitless = false\n",
        ident_lower = exec_ident.to_ascii_lowercase(),
        clean_title = clean_title.replace('\\', "\\\\").replace('"', "\\\""),
        exe_rel = exe_rel.replace('\\', "\\\\").replace('"', "\\\""),
        exec_ident = exec_ident,
        app_id = app_id.replace('\\', "\\\\").replace('"', "\\\""),
    )
}

fn parse_toml_string_field(content: &str, section: &str, key: &str) -> Option<String> {
    let mut cur_section = String::new();
    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(sec) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            cur_section = sec.trim().to_string();
            continue;
        }
        if cur_section != section {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                let v = v.trim();
                let v = v.split('#').next().unwrap_or(v).trim();
                if let Some(inner) = v.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    return Some(inner.replace("\\\\", "\\").replace("\\\"", "\""));
                }
            }
        }
    }
    None
}

/// Look up a curated declarative manifest under `<workspace_root>/games/*/game.toml` matching
/// `app_id` (when non-empty and not `"custom"`) or `exe_rel` / executable filename.
///
/// Preserves curated `[runtime]` settings (e.g. `builtin_overrides`), updates `exe = "..."` to
/// `exe_rel` if the bundled relative path differs, and ensures `aot_dir = "AotCache"` is set.
pub fn find_repo_game_manifest(
    workspace_root: &Path,
    app_id: &str,
    exe_rel: &str,
) -> Option<String> {
    let games_dir = workspace_root.join("games");
    let mut entries: Vec<PathBuf> = fs::read_dir(&games_dir)
        .ok()?
        .flatten()
        .filter_map(|e| {
            let ft = e.file_type().ok()?;
            if ft.is_dir() && !ft.is_symlink() {
                let m = e.path().join("game.toml");
                if m.is_file() {
                    return Some(m);
                }
            }
            None
        })
        .collect();
    entries.sort();

    let want_app_id = (!app_id.trim().is_empty() && !app_id.eq_ignore_ascii_case("custom"))
        .then_some(app_id.trim());
    let want_exe_rel = exe_rel.replace('\\', "/");
    let want_exe_name = Path::new(&want_exe_rel)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&want_exe_rel);

    let mut matched_content: Option<String> = None;
    for manifest_path in entries {
        let Ok(content) = fs::read_to_string(&manifest_path) else {
            continue;
        };
        let m_app_id = parse_toml_string_field(&content, "app", "steam_app_id");
        let m_exe = parse_toml_string_field(&content, "app", "exe");

        let by_id = want_app_id.is_some_and(|want| {
            m_app_id
                .as_deref()
                .is_some_and(|got| got.eq_ignore_ascii_case(want))
        });
        let by_exe = m_exe.as_deref().is_some_and(|got| {
            let got_norm = got.replace('\\', "/");
            let got_name = Path::new(&got_norm)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(&got_norm);
            got_norm.eq_ignore_ascii_case(&want_exe_rel)
                || got_name.eq_ignore_ascii_case(want_exe_name)
        });

        if by_id || by_exe {
            matched_content = Some(content);
            if by_id {
                break;
            }
        }
    }

    let raw = matched_content?;
    let has_aot_dir = parse_toml_string_field(&raw, "runtime", "aot_dir").is_some();
    let escaped_exe = want_exe_rel.replace('\\', "\\\\").replace('"', "\\\"");

    let mut out_lines: Vec<String> = Vec::new();
    let mut cur_section = String::new();
    let mut saw_runtime_section = false;
    let mut injected_aot_dir = has_aot_dir;

    for line in raw.lines() {
        let trimmed = line.trim();
        if let Some(sec) = trimmed
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
        {
            if cur_section == "runtime" && !injected_aot_dir {
                out_lines.push("aot_dir = \"AotCache\"".to_string());
                injected_aot_dir = true;
            }
            cur_section = sec.trim().to_string();
            if cur_section == "runtime" {
                saw_runtime_section = true;
            }
            out_lines.push(line.to_string());
            continue;
        }
        if cur_section == "app" {
            if let Some((k, _)) = trimmed.split_once('=') {
                if k.trim() == "exe" {
                    out_lines.push(format!("exe = \"{escaped_exe}\""));
                    continue;
                }
            }
        }
        out_lines.push(line.to_string());
    }

    if !injected_aot_dir {
        if !saw_runtime_section {
            out_lines.push(String::new());
            out_lines.push("[runtime]".to_string());
        }
        out_lines.push("aot_dir = \"AotCache\"".to_string());
    }

    let mut result = out_lines.join("\n");
    if !result.ends_with('\n') {
        result.push('\n');
    }
    Some(result)
}

/// Copy portable Tier-1 shader caches (`ir*`, `manifest/`, `pso/pso_manifest/`) from `src_root`
/// into `dst_root`, deliberately skipping GPU/macOS-build-specific `*.mtl4a` archives so `.app`
/// bundles stay compact and 100% cross-Mac portable. Returns total files copied.
pub fn copy_portable_shader_cache(src_root: &Path, dst_root: &Path) -> Result<usize> {
    if !src_root.is_dir() {
        return Ok(0);
    }
    fs::create_dir_all(dst_root)
        .with_context(|| format!("creating {}", dst_root.display()))?;

    fn copy_dir_filtered(src: &Path, dst: &Path) -> Result<usize> {
        fs::create_dir_all(dst)?;
        let mut count = 0usize;
        for entry in fs::read_dir(src)?.flatten() {
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            if ft.is_symlink() {
                continue;
            }
            let src_p = entry.path();
            let dst_p = dst.join(entry.file_name());
            if ft.is_dir() {
                count += copy_dir_filtered(&src_p, &dst_p)?;
            } else if ft.is_file() {
                if src_p
                    .extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("mtl4a"))
                {
                    continue;
                }
                fs::copy(&src_p, &dst_p)?;
                count += 1;
            }
        }
        Ok(count)
    }

    let mut total = 0usize;
    for entry in fs::read_dir(src_root)?.flatten() {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() || !ft.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("ir") || name_str == "manifest" {
            let dst_sub = dst_root.join(&name);
            let _ = fs::remove_dir_all(&dst_sub);
            total += copy_dir_filtered(&entry.path(), &dst_sub)?;
        }
    }

    let pso_manifest_src = src_root.join("pso/pso_manifest");
    if pso_manifest_src.is_dir() {
        let pso_manifest_dst = dst_root.join("pso/pso_manifest");
        let _ = fs::remove_dir_all(&pso_manifest_dst);
        total += copy_dir_filtered(&pso_manifest_src, &pso_manifest_dst)?;
    }

    Ok(total)
}

/// Copy only runtime-required Tier-1 IR shader caches (`ir*`, e.g. `.aw2s` / `.metallib`)
/// into an iOS `<App>.app/ShaderCache`.
///
/// iOS runs in `CacheOnly` mode and never executes offline DXIL prewarming or macOS PSO
/// manifest replay, so skipping `manifest/` (raw DXIL) and `pso/` saves ~130+ MB per bundle
/// with zero runtime impact.
pub fn copy_ios_shader_cache(src_root: &Path, dst_root: &Path) -> Result<usize> {
    if !src_root.is_dir() {
        return Ok(0);
    }
    fs::create_dir_all(dst_root)
        .with_context(|| format!("creating {}", dst_root.display()))?;

    fn copy_ir_dir(src: &Path, dst: &Path) -> Result<usize> {
        fs::create_dir_all(dst)?;
        let mut count = 0usize;
        for entry in fs::read_dir(src)?.flatten() {
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            if ft.is_symlink() {
                continue;
            }
            let src_p = entry.path();
            let dst_p = dst.join(entry.file_name());
            if ft.is_dir() {
                count += copy_ir_dir(&src_p, &dst_p)?;
            } else if ft.is_file() {
                if src_p
                    .extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("mtl4a"))
                {
                    continue;
                }
                fs::copy(&src_p, &dst_p)?;
                count += 1;
            }
        }
        Ok(count)
    }

    let mut total = 0usize;
    for entry in fs::read_dir(src_root)?.flatten() {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() || !ft.is_dir() {
            continue;
        }
        let name = entry.file_name();
        if name.to_string_lossy().starts_with("ir") {
            let dst_sub = dst_root.join(&name);
            let _ = fs::remove_dir_all(&dst_sub);
            total += copy_ir_dir(&entry.path(), &dst_sub)?;
        }
    }

    Ok(total)
}

/// Generate the Rust source for the self-contained `.app` launcher binary.
///
/// The launcher resolves `Contents/Frameworks/`, `Contents/Resources/`, and `Contents/MacOS/`
/// relative to `std::env::current_exe()` so double-clicking the `.app` from Finder works with
/// zero developer environment variables set:
/// - Sets `PORT_MANIFEST` when `Contents/Resources/game.toml` exists.
/// - Sets `PORT_AOT_DIR` / `AW2_DLL_AOT_DIR` to `Contents/Resources/AotCache`.
/// - Sets `PORT_SHADERD` / `AW2_SHADERD` when `Contents/MacOS/dxil-metald` exists, and prepends
///   `Contents/MacOS` to `PATH`.
/// - Sets `METAL_IRCONVERTER_DYLIB` when `Contents/Frameworks/libmetalirconverter.dylib` exists.
/// - Sets `AW2_DXILCONV` when `Contents/Frameworks/libdxilconv.dylib` exists.
/// - Sets `PORT_VULKAN_DYLIB` when `Contents/Frameworks/libMoltenVK.dylib` exists.
/// - Prepends `Contents/Frameworks` to `DYLD_FALLBACK_LIBRARY_PATH`.
/// - Preserves any explicit developer override already present in the environment.
pub fn generate_bundle_launcher_source(cfg: &BundleLauncherConfig<'_>) -> String {
    format!(
        r#"fn main() {{
    let current_exe = std::env::current_exe().expect("resolve current_exe");
    let macos_dir = current_exe.parent().expect("Contents/MacOS");
    let contents_dir = macos_dir.parent().expect("Contents");
    let resources_dir = contents_dir.join("Resources");
    let frameworks_dir = contents_dir.join("Frameworks");

    let bundled_manifest = resources_dir.join("game.toml");
    let bundled_aot_dir = resources_dir.join("AotCache");
    let bundled_shader_cache = resources_dir.join("ShaderCache");
    let bundled_shader_manifest = bundled_shader_cache.join("manifest");
    let bundled_pso_manifest = bundled_shader_cache.join("pso").join("pso_manifest");
    let bundled_registry_tsv = resources_dir.join("steam_registry.tsv");
    let bundled_shaderd = macos_dir.join("dxil-metald");
    let bundled_msc = frameworks_dir.join("libmetalirconverter.dylib");
    let bundled_dxilconv = frameworks_dir.join("libdxilconv.dylib");
    let bundled_moltenvk = frameworks_dir.join("libMoltenVK.dylib");

    fn dir_is_writable(dir: &std::path::Path) -> bool {{
        #[cfg(unix)]
        {{
            extern "C" {{
                fn access(path: *const std::ffi::c_char, amode: i32) -> i32;
            }}
            const W_OK: i32 = 2;
            let mut cur = Some(dir);
            while let Some(p) = cur {{
                if p.exists() {{
                    let Ok(cstr) = std::ffi::CString::new(p.as_os_str().as_encoded_bytes()) else {{
                        return false;
                    }};
                    return unsafe {{ access(cstr.as_ptr(), W_OK) == 0 }};
                }}
                cur = p.parent();
            }}
            false
        }}
        #[cfg(not(unix))]
        {{
            let _ = dir;
            true
        }}
    }}

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--aot-info") {{
        println!("=== Native ARM64 Game Bundle Launcher: {clean_title} ===");
        println!("  Steam AppID            : {app_id}");
        println!("  Original PE Executable : {exe_rel}");
        println!("  Preferred ImageBase    : 0x{image_base:016X} (SizeOfImage: 0x{size_of_image:08X})");
        println!("  AOT Dylib Functions    : {aot_fn_count}");
        println!("  AOT Dylib              : {aot_dylib_name}");
        println!("  Bundled Manifest (TOML): {{}}", bundled_manifest.is_file());
        println!("  Bundled AotCache Dir   : {{}}", bundled_aot_dir.is_dir());
        println!("  Bundled Shader Helper  : {{}}", bundled_shaderd.is_file());
        println!("  Bundled Metal IR Conv  : {{}}", bundled_msc.is_file());
        println!("  Bundled DXILConv (DX11): {{}}", bundled_dxilconv.is_file());
        println!("  Bundled MoltenVK (VK)  : {{}}", bundled_moltenvk.is_file());
        return;
    }}

    let game_dir = resources_dir.join("Game");
    let target_pe = game_dir.join({exe_rel:?});
    let loader_bin = macos_dir.join("port-runner");

    // Best-effort strip Gatekeeper quarantine xattrs on executables/dylibs when copied from an
    // external SSD or another Mac, only when the bundle is writable (skipping read-only installs).
    if dir_is_writable(macos_dir) {{
        let _ = std::process::Command::new("/usr/bin/xattr")
            .args(["-dr", "com.apple.quarantine"])
            .arg(macos_dir)
            .arg(&frameworks_dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }}

    let mut aot_dylibs = Vec::new();
    let primary_aot_name: &str = {aot_dylib_name:?};
    if !primary_aot_name.is_empty() {{
        let primary_aot = frameworks_dir.join(primary_aot_name);
        if primary_aot.is_file() {{
            aot_dylibs.push(primary_aot.to_string_lossy().into_owned());
        }}
    }}
    let extra_list: &[&str] = &{extra_aot_dylibs:?};
    for &extra in extra_list {{
        let p = frameworks_dir.join(extra);
        if p.is_file() {{
            aot_dylibs.push(p.to_string_lossy().into_owned());
        }}
    }}

    let dyld_fallback = match std::env::var("DYLD_FALLBACK_LIBRARY_PATH") {{
        Ok(existing) if !existing.trim().is_empty() => {{
            format!("{{}}:{{existing}}", frameworks_dir.display())
        }}
        _ => format!(
            "{{}}:/opt/homebrew/lib:/usr/local/lib:/usr/lib",
            frameworks_dir.display()
        ),
    }};

    let bundled_path = match std::env::var("PATH") {{
        Ok(existing) if !existing.trim().is_empty() => {{
            format!("{{}}:{{existing}}", macos_dir.display())
        }}
        _ => format!(
            "{{}}:/usr/bin:/bin:/usr/sbin:/sbin",
            macos_dir.display()
        ),
    }};

    let env_unset = |key: &str| {{
        std::env::var_os(key).filter(|v| !v.is_empty()).is_none()
    }};

    let mut cmd = std::process::Command::new(&loader_bin);
    cmd.arg("--game-dir")
        .arg(&game_dir)
        .arg("--pe")
        .arg(&target_pe)
        .args(&args)
        .env("STEAM_APPID", {app_id:?})
        .env("SteamAppId", {app_id:?})
        .env("SteamGameId", {app_id:?})
        .env("PATH", &bundled_path)
        .env("DYLD_FALLBACK_LIBRARY_PATH", &dyld_fallback);

    if env_unset("PORT_GAME_DIR") {{
        cmd.env("PORT_GAME_DIR", &game_dir);
    }}
    if bundled_manifest.is_file() && env_unset("PORT_MANIFEST") {{
        cmd.env("PORT_MANIFEST", &bundled_manifest);
    }}
    if env_unset("PORT_AOT_FALLBACK_DIR") {{
        cmd.env("PORT_AOT_FALLBACK_DIR", &bundled_aot_dir);
    }}
    if env_unset("PORT_SHADER_CACHE_FALLBACK") {{
        cmd.env("PORT_SHADER_CACHE_FALLBACK", &bundled_shader_cache);
    }}
    if env_unset("PORT_SHADER_MANIFEST_FALLBACK") {{
        cmd.env("PORT_SHADER_MANIFEST_FALLBACK", &bundled_shader_manifest);
    }}
    if env_unset("PORT_PSO_MANIFEST_FALLBACK") {{
        cmd.env("PORT_PSO_MANIFEST_FALLBACK", &bundled_pso_manifest);
    }}

    // Keep signed `.app/Contents/Resources/` strictly immutable during normal user launches on
    // any Mac (e.g. read-only `/Applications` or App Translocation): read prewarmed `.aot` and
    // `.aw2s` / manifests via `*_FALLBACK*` above, and write new runtime caches + per-GPU/OS
    // Metal 4 PSO archives to the user's writable cache directories. During explicit
    // `--prewarm-aot` packaging or headless `env_clear()` (no `HOME`), write into the bundle if
    // writable or fall back to a per-app temp cache dir if read-only.
    let prewarm_bundle = args.iter().any(|a| a == "--prewarm-aot");
    let user_home = std::env::var_os("HOME")
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from);
    let fallback_tmp_root = std::env::temp_dir().join(format!("port-factory-cache-{{}}", {app_id:?}));
    let writable_aot_dir = if !prewarm_bundle {{
        if let Some(ref home) = user_home {{
            home.join("Library/Caches/port-factory/aot")
        }} else if dir_is_writable(&bundled_aot_dir) {{
            bundled_aot_dir.clone()
        }} else {{
            fallback_tmp_root.join("aot")
        }}
    }} else if dir_is_writable(&bundled_aot_dir) {{
        bundled_aot_dir.clone()
    }} else {{
        fallback_tmp_root.join("aot")
    }};
    let writable_shader_cache = if !prewarm_bundle {{
        if let Some(ref home) = user_home {{
            home.join("Translator/ShaderCache")
        }} else if dir_is_writable(&bundled_shader_cache) {{
            bundled_shader_cache.clone()
        }} else {{
            fallback_tmp_root.join("ShaderCache")
        }}
    }} else if dir_is_writable(&bundled_shader_cache) {{
        bundled_shader_cache.clone()
    }} else {{
        fallback_tmp_root.join("ShaderCache")
    }};

    if env_unset("PORT_AOT_DIR") {{
        cmd.env("PORT_AOT_DIR", &writable_aot_dir);
    }}
    if env_unset("AW2_DLL_AOT_DIR") {{
        cmd.env("AW2_DLL_AOT_DIR", &writable_aot_dir);
    }}
    if !aot_dylibs.is_empty() && env_unset("PORT_AOT_DYLIB") {{
        cmd.env("PORT_AOT_DYLIB", aot_dylibs.join(":"));
    }}
    if env_unset("PORT_REGISTRY_TSV") {{
        cmd.env("PORT_REGISTRY_TSV", &bundled_registry_tsv);
    }}
    if env_unset("AW2_REGISTRY_TSV") {{
        cmd.env("AW2_REGISTRY_TSV", &bundled_registry_tsv);
    }}
    if env_unset("PORT_SHADER_CACHE") {{
        cmd.env("PORT_SHADER_CACHE", &writable_shader_cache);
    }}
    if env_unset("AW2_SHADER_CACHE") {{
        cmd.env("AW2_SHADER_CACHE", &writable_shader_cache);
    }}
    if bundled_shaderd.is_file() {{
        if env_unset("PORT_SHADERD") {{
            cmd.env("PORT_SHADERD", &bundled_shaderd);
        }}
        if env_unset("AW2_SHADERD") {{
            cmd.env("AW2_SHADERD", &bundled_shaderd);
        }}
    }}
    if bundled_msc.is_file() && env_unset("METAL_IRCONVERTER_DYLIB") {{
        cmd.env("METAL_IRCONVERTER_DYLIB", &bundled_msc);
    }}
    if bundled_dxilconv.is_file() && env_unset("AW2_DXILCONV") {{
        cmd.env("AW2_DXILCONV", &bundled_dxilconv);
    }}
    if bundled_moltenvk.is_file() && env_unset("PORT_VULKAN_DYLIB") {{
        cmd.env("PORT_VULKAN_DYLIB", &bundled_moltenvk);
    }}

    use std::os::unix::process::CommandExt;
    let err = cmd.exec();
    eprintln!("Failed to exec {{}}: {{err}}", loader_bin.display());
    std::process::exit(1);
}}
"#,
        clean_title = cfg.clean_title,
        app_id = cfg.app_id,
        exe_rel = cfg.exe_rel,
        image_base = cfg.image_base,
        size_of_image = cfg.size_of_image,
        aot_fn_count = cfg.aot_fn_count,
        aot_dylib_name = cfg.aot_dylib_name,
        extra_aot_dylibs = cfg.extra_aot_dylibs,
    )
}

/// Result of converting a Steam game or Windows application.
#[derive(Debug, Clone)]
pub struct SteamConvertResult {
    pub app_id: String,
    pub game_name: String,
    pub source_dir: PathBuf,
    pub primary_exe: PathBuf,
    pub output_path: PathBuf,
    pub format: ConversionFormat,
    pub win32_readiness_pct: f64,
    pub shaders_converted: usize,
    pub registry_keys_seeded: usize,
    pub aot_functions_compiled: usize,
    pub aot_instructions_lowered: usize,
    pub aot_dylib_path: Option<PathBuf>,
    pub standalone_executable: Option<PathBuf>,
    pub staged_frameworks: StagedFrameworks,
}

/// Resolve a target (`AppID` like `2357570`, game name like `Overwatch`, `steam`, or path)
/// to an [`InstalledGameEntry`] and convert it in one click to the requested format.
pub fn convert_steam_game(
    workspace_root: &Path,
    opts: &SteamConvertOptions,
) -> Result<SteamConvertResult> {
    let report = scan_all_steam_libraries(workspace_root)?;
    let db = Win32Database::from_workspace_src(workspace_root)?;

    let q = opts.target.trim();
    let q_lower = q.to_ascii_lowercase();

    let resolved_entry = if let Some(found) = report.installed_games.iter().find(|g| {
        g.app_id.eq_ignore_ascii_case(q)
            || g.name.to_ascii_lowercase().contains(&q_lower)
            || (q_lower == "steam" && g.app_id == "steam-client-win64")
    }) {
        found.clone()
    } else {
        let p = PathBuf::from(q);
        if !p.exists() {
            bail!(
                "target {q:?} not found in installed Steam/CrossOver/Heroic libraries and path does not exist.\nRun `port-factory steam-scan` to list installed & owned games, or `port-factory steam-download <appid>` first."
            );
        }
        let dir = if p.is_file() {
            p.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            p.clone()
        };
        let name = dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("ConvertedGame")
            .to_string();
        let mut entry = inspect_game_directory(
            "custom",
            &name,
            GameLibrarySource::MacOsSteam,
            &dir,
            0,
            "local",
            "Windows",
            &db,
        )?;
        if p.is_file() {
            entry.primary_exe = Some(p);
        }
        entry
    };

    let primary_exe = resolved_entry.primary_exe.clone().ok_or_else(|| {
        anyhow::anyhow!(
            "no Windows .exe found in {} (game may be a macOS-only install; use `steam-download {}` to fetch the Windows x64 depot)",
            resolved_entry.install_dir.display(),
            resolved_entry.app_id
        )
    })?;

    let mut clean_title: String = resolved_entry
        .name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .trim()
        .to_string();
    if clean_title.is_empty() {
        clean_title = "ConvertedGame".to_string();
    }
    let mut exec_ident: String = clean_title
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    if exec_ident.is_empty() {
        exec_ident = "ConvertedGame".to_string();
    }

    match opts.format {
        ConversionFormat::AotCrate => {
            let out_dir = opts.output_path.clone().unwrap_or_else(|| {
                workspace_root
                    .join("target/converted")
                    .join(format!("{}-aot", exec_ident.to_ascii_lowercase()))
            });
            let crate_name = format!("{}_aot", exec_ident.to_ascii_lowercase());
            let summary = generate_embeddable_aot_crate(
                &primary_exe,
                &out_dir,
                Some(&crate_name),
                &db,
            )?;
            let post = apply_post_install(
                &resolved_entry.install_dir,
                &resolved_entry.app_id,
                &out_dir,
            )?;
            Ok(SteamConvertResult {
                app_id: resolved_entry.app_id.clone(),
                game_name: resolved_entry.name.clone(),
                source_dir: resolved_entry.install_dir.clone(),
                primary_exe,
                output_path: out_dir,
                format: ConversionFormat::AotCrate,
                win32_readiness_pct: resolved_entry.readiness_pct(),
                shaders_converted: 0,
                registry_keys_seeded: post.registry_entries.len(),
                aot_functions_compiled: summary.function_count,
                aot_instructions_lowered: summary.lowered_instructions,
                aot_dylib_path: None,
                standalone_executable: None,
                staged_frameworks: StagedFrameworks::default(),
            })
        }
        ConversionFormat::MetalShaders => {
            let out_dir = opts.output_path.clone().unwrap_or_else(|| {
                workspace_root
                    .join("target/converted")
                    .join(format!("{}-shaders", exec_ident.to_ascii_lowercase()))
            });
            let stats = precompile_shaders_for_paths(
                std::slice::from_ref(&resolved_entry.install_dir),
                Some(out_dir.clone()),
                1024,
            )?;
            Ok(SteamConvertResult {
                app_id: resolved_entry.app_id.clone(),
                game_name: resolved_entry.name.clone(),
                source_dir: resolved_entry.install_dir.clone(),
                primary_exe,
                output_path: out_dir,
                format: ConversionFormat::MetalShaders,
                win32_readiness_pct: resolved_entry.readiness_pct(),
                shaders_converted: stats.active_wave32_converted,
                registry_keys_seeded: 0,
                aot_functions_compiled: 0,
                aot_instructions_lowered: 0,
                aot_dylib_path: None,
                standalone_executable: None,
                staged_frameworks: StagedFrameworks::default(),
            })
        }
        ConversionFormat::MacOsApp => {
            let out_app = opts.output_path.clone().unwrap_or_else(|| {
                workspace_root
                    .join("target/converted")
                    .join(format!("{clean_title}.app"))
            });
            package_macos_game_bundle(
                workspace_root,
                &resolved_entry,
                &primary_exe,
                &clean_title,
                &exec_ident,
                &out_app,
                opts.symlink_game,
                opts.precompile_shaders,
                opts.max_aot_functions,
                opts.bundle_shader_cache_from.as_deref(),
                &db,
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn package_macos_game_bundle(
    workspace_root: &Path,
    entry: &InstalledGameEntry,
    primary_exe: &Path,
    clean_title: &str,
    exec_ident: &str,
    out_app: &Path,
    symlink_game: bool,
    precompile_shaders: bool,
    max_aot_functions: usize,
    bundle_shader_cache_from: Option<&Path>,
    db: &Win32Database,
) -> Result<SteamConvertResult> {
    let contents_dir = out_app.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let frameworks_dir = contents_dir.join("Frameworks");
    let resources_dir = contents_dir.join("Resources");
    let shader_cache_dir = resources_dir.join("ShaderCache");
    let aot_cache_dir = resources_dir.join("AotCache");
    let aot_dir = resources_dir.join("AOT");
    let symbols_dir = resources_dir.join("symbols");

    // Clean stale Mach-O binaries, frameworks, AOT cache, and signatures before repacking so
    // updated game builds and new runtimes never inherit obsolete artifacts.
    let _ = fs::remove_dir_all(contents_dir.join("_CodeSignature"));
    let _ = fs::remove_dir_all(&macos_dir);
    let _ = fs::remove_dir_all(&frameworks_dir);
    let _ = fs::remove_dir_all(&aot_cache_dir);
    fs::create_dir_all(&macos_dir)?;
    fs::create_dir_all(&frameworks_dir)?;
    fs::create_dir_all(&resources_dir)?;
    fs::create_dir_all(&shader_cache_dir)?;
    fs::create_dir_all(&aot_cache_dir)?;
    fs::create_dir_all(&aot_dir)?;
    fs::create_dir_all(&symbols_dir)?;

    let exe_rel = primary_exe
        .strip_prefix(&entry.install_dir)
        .unwrap_or_else(|_| Path::new(primary_exe.file_name().unwrap_or_default()))
        .to_string_lossy()
        .replace('\\', "/");

    // 1. Extract AppIcon.icns from the PE .rsrc section if available
    let icon_icns = resources_dir.join("AppIcon.icns");
    let extract_icon_py = workspace_root.join("tools/extract_pe_icon.py");
    if extract_icon_py.is_file() {
        let _ = Command::new("python3")
            .arg(&extract_icon_py)
            .arg(primary_exe)
            .arg(&icon_icns)
            .status();
    }

    // 2. Write Info.plist
    let info_plist = generate_info_plist(clean_title, exec_ident, &entry.build_id);
    fs::write(contents_dir.join("Info.plist"), info_plist)?;

    // 3. Copy or build lightweight universal `port-runner` (zero game-specific shards) and `dxil-metald`
    let loader_bin = find_or_build_binary(workspace_root, "port-runner")?;
    let shaderd_bin = find_or_build_binary(workspace_root, "dxil-metald")?;
    let dst_runner = macos_dir.join("port-runner");
    let dst_shaderd = macos_dir.join("dxil-metald");
    fs::copy(&loader_bin, &dst_runner)?;
    fs::copy(&shaderd_bin, &dst_shaderd)?;

    // Write declarative per-game manifest (`Contents/Resources/game.toml`), adopting any curated
    // repo manifest under `games/<id>/game.toml` (e.g. `builtin_overrides`) when available.
    let manifest_path = resources_dir.join("game.toml");
    let manifest_toml = find_repo_game_manifest(workspace_root, &entry.app_id, &exe_rel)
        .unwrap_or_else(|| {
            generate_game_manifest_toml(clean_title, exec_ident, &exe_rel, &entry.app_id)
        });
    fs::write(&manifest_path, manifest_toml)?;

    // Copy runtime frameworks (`libmetalirconverter.dylib`, optional `libdxilconv.dylib`,
    // optional `libMoltenVK.dylib`) into `Contents/Frameworks/` so cold launch from Finder
    // works with zero developer environment variables set.
    let staged_frameworks = stage_bundle_frameworks(&frameworks_dir, workspace_root)?;

    // 4. Apply Post-Install (`steam_appid.txt` + `steam_registry.tsv`)
    let post = apply_post_install(&entry.install_dir, &entry.app_id, &resources_dir)?;

    // 5. Link or Copy the game directory into `Contents/Resources/Game`
    let bundle_game_dir = resources_dir.join("Game");
    if symlink_game {
        if bundle_game_dir.exists() || bundle_game_dir.is_symlink() {
            let _ = fs::remove_file(&bundle_game_dir);
            let _ = fs::remove_dir_all(&bundle_game_dir);
        }
        std::os::unix::fs::symlink(&entry.install_dir, &bundle_game_dir)?;
    } else {
        let same_dir = fs::canonicalize(&entry.install_dir)
            .ok()
            .zip(fs::canonicalize(&bundle_game_dir).ok())
            .is_some_and(|(a, b)| a == b);
        if !same_dir {
            if bundle_game_dir.is_symlink() {
                let _ = fs::remove_file(&bundle_game_dir);
            } else if bundle_game_dir.exists() {
                let _ = fs::remove_dir_all(&bundle_game_dir);
            }
            let status = Command::new("cp")
                .args([
                    "-Rc",
                    &format!("{}/", entry.install_dir.display()),
                    &bundle_game_dir.to_string_lossy(),
                ])
                .status();
            if !status.is_ok_and(|s| s.success()) {
                let _ = fs::remove_dir_all(&bundle_game_dir);
                let status_plain = Command::new("cp")
                    .args([
                        "-R",
                        &format!("{}/", entry.install_dir.display()),
                        &bundle_game_dir.to_string_lossy(),
                    ])
                    .status()
                    .with_context(|| {
                        format!(
                            "copying game assets from {} to {}",
                            entry.install_dir.display(),
                            bundle_game_dir.display()
                        )
                    })?;
                if !status_plain.success() {
                    bail!(
                        "failed to copy game assets into {}",
                        bundle_game_dir.display()
                    );
                }
            }
            // Strip DepotDownloader internal staging metadata from the packaged `.app` copy.
            let _ = fs::remove_dir_all(bundle_game_dir.join(".DepotDownloader"));
        }
        let _ = apply_post_install(&entry.install_dir, &entry.app_id, &bundle_game_dir);
    }

    // 6. Shader Pre-Compilation & optional portable Tier-1 cache copy into `Contents/Resources/ShaderCache`
    let mut shaders_converted = 0usize;
    if precompile_shaders {
        if let Ok(stats) = precompile_shaders_for_paths(
            std::slice::from_ref(&entry.install_dir),
            Some(shader_cache_dir.clone()),
            1024,
        ) {
            shaders_converted = stats.active_wave32_converted;
        }
    }
    if let Some(src_cache) = bundle_shader_cache_from {
        if let Ok(copied) = copy_portable_shader_cache(src_cache, &shader_cache_dir) {
            shaders_converted += copied;
        }
    }

    // 6b. Offline ARM64 `.aot` block-cache prewarming into `Contents/Resources/AotCache`
    //     (best-effort: passes `--manifest` so curated `builtin_overrides` are prewarmed too).
    let prewarm_target_pe = bundle_game_dir.join(&exe_rel);
    let prewarm_pe = if prewarm_target_pe.is_file() {
        prewarm_target_pe.as_path()
    } else {
        primary_exe
    };
    let prewarm_game_dir = if bundle_game_dir.exists() {
        bundle_game_dir.as_path()
    } else {
        entry.install_dir.as_path()
    };
    match Command::new(&loader_bin)
        .arg("--manifest")
        .arg(&manifest_path)
        .arg("--game-dir")
        .arg(prewarm_game_dir)
        .arg("--pe")
        .arg(prewarm_pe)
        .arg("--prewarm-aot")
        .arg("--aot-dir")
        .arg(&aot_cache_dir)
        .output()
    {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if !stdout.trim().is_empty() {
                print!("{stdout}");
            }
        }
        Ok(out) => {
            eprintln!(
                "warning: offline .aot prewarm failed for {clean_title} (status {}): {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        Err(e) => {
            eprintln!(
                "warning: could not launch port-runner for offline .aot prewarm of {clean_title}: {e}"
            );
        }
    }

    // 7. Optional Prototype Static Rust AOT (`--max-fns N`, skipped cleanly when `max_aot_functions == 0`,
    //    since large games use `x86-runtime`'s prewarmed `.aot` block caches above).
    let ident_lower = exec_ident.to_ascii_lowercase();
    let mut aot_functions_compiled = 0usize;
    let mut aot_instructions_lowered = 0usize;
    let mut extra_aot_dylibs: Vec<String> = Vec::new();
    let (image_base, size_of_image, aot_dylib_name, aot_dylib_path) = if max_aot_functions == 0 {
        let pe_bytes = fs::read(primary_exe)
            .with_context(|| format!("reading primary PE {}", primary_exe.display()))?;
        let pe = Pe::parse(&pe_bytes)
            .with_context(|| format!("parsing primary PE {}", primary_exe.display()))?;
        (
            pe.headers.image_base,
            pe.headers.size_of_image,
            String::new(),
            None,
        )
    } else {
        let mut aot_opts = AotOptions::default();
        aot_opts.emit_win64_abi_attr = false;
        aot_opts.max_functions = max_aot_functions;

        let aot_out = compile_pe_file_to_rust(primary_exe, db, &aot_opts)?;
        aot_functions_compiled = aot_out.functions.len();
        aot_instructions_lowered = aot_out.lowered_instructions;

        let aot_rs_path = aot_dir.join(format!("{ident_lower}_aot.rs"));
        fs::write(&aot_rs_path, &aot_out.rust_source)?;

        // Write symbol table `.syms` for crash / profiler symbolication
        let exe_file_name = primary_exe
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("game.exe");
        let mut syms_txt = format!(
            "# aw2-syms v1 image={} base=0x{:x} count={}\n",
            exe_file_name,
            aot_out.image_base,
            aot_out.functions.len()
        );
        for f in &aot_out.functions {
            use std::fmt::Write as _;
            let _ = writeln!(
                syms_txt,
                "{:08x}\t{:08x}\taot\tVerified\t{}",
                f.begin_rva,
                f.end_rva.saturating_sub(f.begin_rva),
                f.rust_ident
            );
        }
        fs::write(symbols_dir.join(format!("{exe_file_name}.syms")), syms_txt)?;

        // Also AOT-compile companion `*_loader.dll` in the game directory if present
        for pe_file in collect_pe_files(&entry.install_dir).unwrap_or_default() {
            if &pe_file == primary_exe {
                continue;
            }
            let pe_file = &pe_file;
            let fname = pe_file
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            if fname.to_ascii_lowercase().contains("loader")
                && fname.to_ascii_lowercase().ends_with(".dll")
            {
                if let Ok(dll_aot) = compile_pe_file_to_rust(pe_file, db, &aot_opts) {
                    aot_functions_compiled += dll_aot.functions.len();
                    aot_instructions_lowered += dll_aot.lowered_instructions;
                    let stem = pe_file
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("companion")
                        .to_ascii_lowercase();
                    let dll_rs = aot_dir.join(format!("{stem}_aot.rs"));
                    let _ = fs::write(&dll_rs, &dll_aot.rust_source);
                    let dll_dylib = frameworks_dir.join(format!("lib{stem}_aot.dylib"));
                    let st = Command::new("rustc")
                        .args([
                            "--edition=2021",
                            "--crate-type",
                            "cdylib",
                            "-C",
                            "opt-level=2",
                            &dll_rs.to_string_lossy(),
                            "-o",
                            &dll_dylib.to_string_lossy(),
                        ])
                        .status();
                    if st.is_ok_and(|s| s.success()) {
                        extra_aot_dylibs.push(format!("lib{stem}_aot.dylib"));
                    }
                }
            }
        }

        // Compile primary AOT dynamic library (`Contents/Frameworks/lib<ident_lower>_aot.dylib`)
        let dylib_name = format!("lib{ident_lower}_aot.dylib");
        let dylib_path = frameworks_dir.join(&dylib_name);
        let dylib_status = Command::new("rustc")
            .args([
                "--edition=2021",
                "--crate-type",
                "cdylib",
                "-C",
                "opt-level=2",
                &aot_rs_path.to_string_lossy(),
                "-o",
                &dylib_path.to_string_lossy(),
            ])
            .status()
            .context("compiling native ARM64 AOT dylib with rustc")?;
        if !dylib_status.success() {
            bail!(
                "rustc failed to compile AOT dylib {}",
                dylib_path.display()
            );
        }
        (
            aot_out.image_base,
            aot_out.size_of_image,
            dylib_name,
            Some(dylib_path),
        )
    };

    // 8. Build lightweight native ARM64 launcher executable (`Contents/MacOS/<exec_ident>`).
    let standalone_bin_path = macos_dir.join(exec_ident);
    let standalone_rs_path = aot_dir.join(format!("{ident_lower}_standalone_main.rs"));
    let launcher_cfg = BundleLauncherConfig {
        clean_title,
        app_id: &entry.app_id,
        exe_rel: &exe_rel,
        image_base,
        size_of_image,
        aot_fn_count: aot_functions_compiled,
        aot_dylib_name: &aot_dylib_name,
        extra_aot_dylibs: &extra_aot_dylibs,
    };
    let standalone_src = generate_bundle_launcher_source(&launcher_cfg);
    fs::write(&standalone_rs_path, &standalone_src)?;

    let bin_status = Command::new("rustc")
        .args([
            "--edition=2021",
            "-C",
            "opt-level=2",
            &standalone_rs_path.to_string_lossy(),
            "-o",
            &standalone_bin_path.to_string_lossy(),
        ])
        .status()
        .context("compiling standalone native ARM64 Mach-O executable with rustc")?;
    if !bin_status.success() {
        bail!(
            "rustc failed to compile standalone Mach-O executable {}",
            standalone_bin_path.display()
        );
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for bin in [
            &standalone_bin_path,
            &macos_dir.join("port-runner"),
            &macos_dir.join("dxil-metald"),
        ] {
            if let Ok(meta) = fs::metadata(bin) {
                let mut perms = meta.permissions();
                perms.set_mode(0o755);
                let _ = fs::set_permissions(bin, perms);
            }
        }
    }

    // 9. Ad-hoc sign all Mach-O binaries in Frameworks and MacOS, then the .app bundle
    if let Ok(rd) = fs::read_dir(&frameworks_dir) {
        for f in rd.flatten() {
            let p = f.path();
            if p.is_file() {
                let _ = Command::new("codesign")
                    .args(["--force", "--sign", "-", &p.to_string_lossy()])
                    .status();
            }
        }
    }
    for bin in [
        macos_dir.join("dxil-metald"),
        macos_dir.join("port-runner"),
        standalone_bin_path.clone(),
    ] {
        if bin.is_file() {
            let _ = Command::new("codesign")
                .args(["--force", "--sign", "-", &bin.to_string_lossy()])
                .status();
        }
    }
    let _ = Command::new("codesign")
        .args(["--force", "--sign", "-", &out_app.to_string_lossy()])
        .status();

    Ok(SteamConvertResult {
        app_id: entry.app_id.clone(),
        game_name: entry.name.clone(),
        source_dir: entry.install_dir.clone(),
        primary_exe: primary_exe.to_path_buf(),
        output_path: out_app.to_path_buf(),
        format: ConversionFormat::MacOsApp,
        win32_readiness_pct: entry.readiness_pct(),
        shaders_converted,
        registry_keys_seeded: post.registry_entries.len(),
        aot_functions_compiled,
        aot_instructions_lowered,
        aot_dylib_path,
        standalone_executable: Some(standalone_bin_path),
        staged_frameworks,
    })
}

fn find_or_build_binary(workspace_root: &Path, bin_name: &str) -> Result<PathBuf> {
    let pkg = if bin_name == "dxil-metald" {
        "dxil-metal"
    } else {
        "port-loader"
    };
    let status = Command::new("cargo")
        .current_dir(workspace_root)
        .args([
            "build",
            "--release",
            "--target",
            "aarch64-apple-darwin",
            "-p",
            pkg,
            "--bin",
            bin_name,
        ])
        .status();
    if status.is_ok_and(|s| s.success()) {
        for rel in [
            format!("target/aarch64-apple-darwin/release/{bin_name}"),
            format!("target/release/{bin_name}"),
        ] {
            let p = workspace_root.join(rel);
            if p.is_file() {
                return Ok(p);
            }
        }
    }
    for rel in [
        format!("target/aarch64-apple-darwin/release/{bin_name}"),
        format!("target/release/{bin_name}"),
        format!("target/aarch64-apple-darwin/debug/{bin_name}"),
        format!("target/debug/{bin_name}"),
    ] {
        let p = workspace_root.join(rel);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!("failed to build {bin_name}")
}

// ============================================================================
// 7b. Portable `.app` Bundle Verifier (`bundle-verify`)
// ============================================================================

/// Outcome of a single `.app` portability check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundleCheckStatus {
    Pass,
    Warn,
    Fail,
}

/// One diagnostic check from [`verify_app_bundle`].
#[derive(Debug, Clone)]
pub struct BundleCheck {
    pub name: String,
    pub status: BundleCheckStatus,
    pub detail: String,
}

/// Full portability and self-containment verification report for a macOS `.app` bundle.
#[derive(Debug, Clone)]
pub struct BundleVerifyReport {
    pub app_path: PathBuf,
    pub checks: Vec<BundleCheck>,
}

impl BundleVerifyReport {
    pub fn passed(&self) -> bool {
        self.checks
            .iter()
            .all(|c| c.status != BundleCheckStatus::Fail)
    }

    pub fn fail_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == BundleCheckStatus::Fail)
            .count()
    }

    pub fn warn_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == BundleCheckStatus::Warn)
            .count()
    }
}

fn extract_plist_string(plist_xml: &str, key: &str) -> Option<String> {
    let needle = format!("<key>{key}</key>");
    let pos = plist_xml.find(&needle)?;
    let rest = &plist_xml[pos + needle.len()..];
    let s_start = rest.find("<string>")?;
    let after = &rest[s_start + "<string>".len()..];
    let s_end = after.find("</string>")?;
    Some(after[..s_end].trim().to_string())
}

fn is_portable_dylib_dep(dep: &str) -> bool {
    dep.starts_with("/System/Library/")
        || dep.starts_with("/usr/lib/")
        || dep.starts_with("@rpath/")
        || dep.starts_with("@loader_path/")
        || dep.starts_with("@executable_path/")
}

/// Verify that a macOS `.app` bundle is genuinely self-contained and ready to copy to an
/// external SSD and launch on another Apple Silicon Mac without developer environment setup.
pub fn verify_app_bundle(app_path: &Path) -> Result<BundleVerifyReport> {
    let mut checks = Vec::new();
    let push = |checks: &mut Vec<BundleCheck>,
                name: &str,
                status: BundleCheckStatus,
                detail: String| {
        checks.push(BundleCheck {
            name: name.to_string(),
            status,
            detail,
        });
    };

    if !app_path.is_dir() {
        push(
            &mut checks,
            "bundle_root",
            BundleCheckStatus::Fail,
            format!("not a directory: {}", app_path.display()),
        );
        return Ok(BundleVerifyReport {
            app_path: app_path.to_path_buf(),
            checks,
        });
    }

    let canon_app = fs::canonicalize(app_path).unwrap_or_else(|_| app_path.to_path_buf());
    let contents = app_path.join("Contents");
    let macos_dir = contents.join("MacOS");
    let frameworks_dir = contents.join("Frameworks");
    let resources_dir = contents.join("Resources");

    // 1. Info.plist + CFBundleExecutable
    let plist_path = contents.join("Info.plist");
    let mut bundle_executable: Option<String> = None;
    if !plist_path.is_file() {
        push(
            &mut checks,
            "info_plist",
            BundleCheckStatus::Fail,
            "missing Contents/Info.plist".to_string(),
        );
    } else {
        let plist_xml = fs::read_to_string(&plist_path).unwrap_or_default();
        bundle_executable = extract_plist_string(&plist_xml, "CFBundleExecutable");
        #[cfg(target_os = "macos")]
        {
            let lint = Command::new("/usr/bin/plutil")
                .args(["-lint", &plist_path.to_string_lossy()])
                .output();
            match lint {
                Ok(o) if o.status.success() => push(
                    &mut checks,
                    "info_plist",
                    BundleCheckStatus::Pass,
                    format!(
                        "valid XML (CFBundleExecutable={})",
                        bundle_executable.as_deref().unwrap_or("<missing>")
                    ),
                ),
                Ok(o) => push(
                    &mut checks,
                    "info_plist",
                    BundleCheckStatus::Fail,
                    format!(
                        "plutil -lint failed: {}",
                        String::from_utf8_lossy(&o.stderr).trim()
                    ),
                ),
                Err(e) => push(
                    &mut checks,
                    "info_plist",
                    BundleCheckStatus::Warn,
                    format!("could not invoke plutil: {e}"),
                ),
            }
        }
        #[cfg(not(target_os = "macos"))]
        push(
            &mut checks,
            "info_plist",
            BundleCheckStatus::Pass,
            format!(
                "present (CFBundleExecutable={})",
                bundle_executable.as_deref().unwrap_or("<missing>")
            ),
        );
    }

    // 2. Core executables
    let launcher_path = bundle_executable.as_ref().map(|e| macos_dir.join(e));
    match launcher_path {
        Some(ref p) if p.is_file() => push(
            &mut checks,
            "launcher_executable",
            BundleCheckStatus::Pass,
            format!("{}", p.file_name().unwrap_or_default().to_string_lossy()),
        ),
        _ => push(
            &mut checks,
            "launcher_executable",
            BundleCheckStatus::Fail,
            format!(
                "missing Contents/MacOS/{}",
                bundle_executable.as_deref().unwrap_or("<unset>")
            ),
        ),
    }

    let runner_path = macos_dir.join("port-runner");
    let aw2_runner_path = macos_dir.join("aw2-loader");
    let has_port_runner = runner_path.is_file();
    if has_port_runner || aw2_runner_path.is_file() {
        let which = if has_port_runner {
            "port-runner"
        } else {
            "aw2-loader"
        };
        let sz_mb = fs::metadata(if has_port_runner {
            &runner_path
        } else {
            &aw2_runner_path
        })
        .map(|m| m.len() as f64 / (1024.0 * 1024.0))
        .unwrap_or(0.0);
        push(
            &mut checks,
            "runtime_binary",
            BundleCheckStatus::Pass,
            format!("{which} ({sz_mb:.1} MiB)"),
        );
    } else {
        push(
            &mut checks,
            "runtime_binary",
            BundleCheckStatus::Fail,
            "missing Contents/MacOS/port-runner (or aw2-loader)".to_string(),
        );
    }

    let shaderd_path = macos_dir.join("dxil-metald");
    if shaderd_path.is_file() {
        push(
            &mut checks,
            "shader_helper",
            BundleCheckStatus::Pass,
            "Contents/MacOS/dxil-metald present".to_string(),
        );
    } else {
        push(
            &mut checks,
            "shader_helper",
            BundleCheckStatus::Fail,
            "missing Contents/MacOS/dxil-metald".to_string(),
        );
    }

    // 3. Manifest & Game payload + symlink audit across Contents/
    let manifest_path = resources_dir.join("game.toml");
    if manifest_path.is_file() {
        let content = fs::read_to_string(&manifest_path).unwrap_or_default();
        let exe_field = parse_toml_string_field(&content, "app", "exe").unwrap_or_default();
        let overrides = parse_toml_string_field(&content, "runtime", "builtin_overrides")
            .unwrap_or_else(|| "(none)".to_string());
        push(
            &mut checks,
            "game_manifest",
            BundleCheckStatus::Pass,
            format!("Contents/Resources/game.toml (exe={exe_field:?}, builtin_overrides={overrides:?})"),
        );
    } else {
        push(
            &mut checks,
            "game_manifest",
            BundleCheckStatus::Fail,
            "missing Contents/Resources/game.toml".to_string(),
        );
    }

    let game_dir = resources_dir.join("Game");
    match fs::symlink_metadata(&game_dir) {
        Ok(m) if m.file_type().is_symlink() => push(
            &mut checks,
            "game_payload",
            BundleCheckStatus::Fail,
            "Contents/Resources/Game is a symlink (not portable to an SSD; re-run with --copy-game)"
                .to_string(),
        ),
        Ok(m) if m.is_dir() => push(
            &mut checks,
            "game_payload",
            BundleCheckStatus::Pass,
            "Contents/Resources/Game is a self-contained directory".to_string(),
        ),
        _ => push(
            &mut checks,
            "game_payload",
            BundleCheckStatus::Fail,
            "missing Contents/Resources/Game directory".to_string(),
        ),
    }

    // Walk Contents/ without following symlinks and fail on any symlink escaping the bundle.
    let mut escaping_symlinks = Vec::new();
    let mut stack = vec![contents.clone()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            let p = entry.path();
            if ft.is_symlink() {
                let target_ok = fs::canonicalize(&p)
                    .map(|resolved| resolved.starts_with(&canon_app))
                    .unwrap_or(false);
                if !target_ok {
                    let rel = p.strip_prefix(app_path).unwrap_or(&p);
                    let raw_target = fs::read_link(&p)
                        .map(|t| t.display().to_string())
                        .unwrap_or_else(|_| "<unreadable>".to_string());
                    escaping_symlinks.push(format!("{} -> {raw_target}", rel.display()));
                }
            } else if ft.is_dir() {
                stack.push(p);
            }
        }
    }
    if escaping_symlinks.is_empty() {
        push(
            &mut checks,
            "external_symlinks",
            BundleCheckStatus::Pass,
            "zero symlinks escaping the .app bundle".to_string(),
        );
    } else {
        push(
            &mut checks,
            "external_symlinks",
            BundleCheckStatus::Fail,
            format!(
                "{} symlink(s) escape bundle or dangle: {}",
                escaping_symlinks.len(),
                escaping_symlinks
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        );
    }

    // 4. Frameworks
    let msc = frameworks_dir.join("libmetalirconverter.dylib");
    let dxilconv = frameworks_dir.join("libdxilconv.dylib");
    let moltenvk = frameworks_dir.join("libMoltenVK.dylib");
    if msc.is_file() {
        let mut extras = Vec::new();
        if dxilconv.is_file() {
            extras.push("libdxilconv.dylib");
        }
        if moltenvk.is_file() {
            extras.push("libMoltenVK.dylib");
        }
        push(
            &mut checks,
            "frameworks",
            BundleCheckStatus::Pass,
            format!(
                "libmetalirconverter.dylib present{}",
                if extras.is_empty() {
                    String::new()
                } else {
                    format!(" (+ {})", extras.join(", "))
                }
            ),
        );
    } else {
        push(
            &mut checks,
            "frameworks",
            BundleCheckStatus::Fail,
            "missing Contents/Frameworks/libmetalirconverter.dylib".to_string(),
        );
    }

    // 5. Mach-O dylib link & minos/arch portability audit (`otool -L`, `vtool`, `lipo`)
    #[cfg(target_os = "macos")]
    {
        let mut macho_files = Vec::new();
        for dir in [&macos_dir, &frameworks_dir] {
            if let Ok(rd) = fs::read_dir(dir) {
                for entry in rd.flatten() {
                    let p = entry.path();
                    if p.is_file() && is_macho_file(&p) {
                        macho_files.push(p);
                    }
                }
            }
        }
        macho_files.sort();

        let mut bad_links = Vec::new();
        let mut bad_arch_or_minos = Vec::new();

        for macho in &macho_files {
            let rel = macho
                .strip_prefix(app_path)
                .unwrap_or(macho)
                .display()
                .to_string();
            let macho_s = macho.to_string_lossy().into_owned();

            // Architecture check (`arm64` required for Apple Silicon).
            if let Ok(out) = Command::new("lipo").args(["-archs", &macho_s]).output() {
                if out.status.success() {
                    let archs = String::from_utf8_lossy(&out.stdout);
                    if !archs.split_whitespace().any(|a| a == "arm64") {
                        bad_arch_or_minos.push(format!("{rel}: missing arm64 (archs={})", archs.trim()));
                    }
                }
            }

            // `minos` check (`<= 15.x` so newer-beta-built binaries don't fail on macOS 15/26 Macs).
            let vtool_out = Command::new("vtool")
                .args(["-arch", "arm64", "-show-build", &macho_s])
                .output()
                .or_else(|_| Command::new("vtool").args(["-show-build", &macho_s]).output());
            if let Ok(out) = vtool_out {
                if out.status.success() {
                    for (minos, _sdk) in parse_vtool_macos_versions(&String::from_utf8_lossy(&out.stdout)) {
                        if version_major(&minos).is_some_and(|m| m > 15) {
                            bad_arch_or_minos.push(format!("{rel}: minos={minos} (> 15.x)"));
                        }
                    }
                }
            }

            // Own install name (`otool -D`), so we ignore a dylib's self-ID line in `otool -L`.
            let own_ids: BTreeSet<String> = Command::new("otool")
                .args(["-arch", "arm64", "-D", &macho_s])
                .output()
                .ok()
                .filter(|o| o.status.success())
                .map(|o| {
                    String::from_utf8_lossy(&o.stdout)
                        .lines()
                        .map(str::trim)
                        .filter(|l| !l.is_empty() && !l.ends_with(':'))
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();

            if let Ok(out) = Command::new("otool")
                .args(["-arch", "arm64", "-L", &macho_s])
                .output()
            {
                if out.status.success() {
                    for raw in String::from_utf8_lossy(&out.stdout).lines() {
                        let trimmed = raw.trim();
                        if trimmed.is_empty() || trimmed.ends_with(':') {
                            continue;
                        }
                        let dep = trimmed
                            .split(" (compatibility version")
                            .next()
                            .unwrap_or(trimmed)
                            .trim();
                        if own_ids.contains(dep) {
                            continue;
                        }
                        if !is_portable_dylib_dep(dep) {
                            bad_links.push(format!("{rel} -> {dep}"));
                        }
                    }
                }
            }
        }

        if bad_links.is_empty() {
            push(
                &mut checks,
                "macho_dylib_links",
                BundleCheckStatus::Pass,
                format!(
                    "{} Mach-O file(s) link only macOS system or bundle-relative libraries",
                    macho_files.len()
                ),
            );
        } else {
            push(
                &mut checks,
                "macho_dylib_links",
                BundleCheckStatus::Fail,
                format!("non-portable dylib references: {}", bad_links.join("; ")),
            );
        }

        if bad_arch_or_minos.is_empty() {
            push(
                &mut checks,
                "macho_arch_minos",
                BundleCheckStatus::Pass,
                format!(
                    "all {} Mach-O file(s) include arm64 with portable minos (<= 15.x)",
                    macho_files.len()
                ),
            );
        } else {
            push(
                &mut checks,
                "macho_arch_minos",
                BundleCheckStatus::Fail,
                bad_arch_or_minos.join("; "),
            );
        }
    }

    // 6. Prewarmed AotCache & portable ShaderCache (zero host-specific `*.mtl4a` bloat)
    let aot_cache_dir = resources_dir.join("AotCache");
    let mut aot_count = 0usize;
    let mut aot_bytes = 0u64;
    if let Ok(rd) = fs::read_dir(&aot_cache_dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("aot") {
                aot_count += 1;
                aot_bytes += fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            }
        }
    }
    if aot_count > 0 {
        push(
            &mut checks,
            "aot_cache",
            BundleCheckStatus::Pass,
            format!(
                "{aot_count} prewarmed .aot module(s) ({:.1} MiB)",
                aot_bytes as f64 / (1024.0 * 1024.0)
            ),
        );
    } else if has_port_runner {
        push(
            &mut checks,
            "aot_cache",
            BundleCheckStatus::Warn,
            "no prewarmed .aot files in Contents/Resources/AotCache (first launch will JIT on demand)"
                .to_string(),
        );
    }

    let shader_cache_dir = resources_dir.join("ShaderCache");
    let mut mtl4a_files = Vec::new();
    let mut portable_shader_files = 0usize;
    if shader_cache_dir.is_dir() {
        let mut s_stack = vec![shader_cache_dir.clone()];
        while let Some(d) = s_stack.pop() {
            let Ok(rd) = fs::read_dir(&d) else {
                continue;
            };
            for e in rd.flatten() {
                let Ok(ft) = e.file_type() else {
                    continue;
                };
                if ft.is_symlink() {
                    continue;
                }
                let p = e.path();
                if ft.is_dir() {
                    s_stack.push(p);
                } else if ft.is_file() {
                    if p.extension()
                        .and_then(|s| s.to_str())
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("mtl4a"))
                    {
                        mtl4a_files.push(
                            p.strip_prefix(app_path)
                                .unwrap_or(&p)
                                .display()
                                .to_string(),
                        );
                    } else {
                        portable_shader_files += 1;
                    }
                }
            }
        }
    }
    if mtl4a_files.is_empty() {
        push(
            &mut checks,
            "shader_cache_portability",
            BundleCheckStatus::Pass,
            format!(
                "zero device-specific *.mtl4a archives ({portable_shader_files} portable shader/manifest file(s))"
            ),
        );
    } else {
        push(
            &mut checks,
            "shader_cache_portability",
            BundleCheckStatus::Fail,
            format!(
                "found {} host-specific *.mtl4a archive(s) inside bundle (remove; cold Macs rebuild from pso_manifest): {}",
                mtl4a_files.len(),
                mtl4a_files.iter().take(3).cloned().collect::<Vec<_>>().join(", ")
            ),
        );
    }

    // 7. Code signature verification
    #[cfg(target_os = "macos")]
    {
        match Command::new("codesign")
            .args(["--verify", "--deep", "--strict", &app_path.to_string_lossy()])
            .output()
        {
            Ok(out) if out.status.success() => push(
                &mut checks,
                "codesign",
                BundleCheckStatus::Pass,
                "codesign --verify --deep --strict OK".to_string(),
            ),
            Ok(out) => push(
                &mut checks,
                "codesign",
                BundleCheckStatus::Fail,
                format!(
                    "codesign verify failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                ),
            ),
            Err(e) => push(
                &mut checks,
                "codesign",
                BundleCheckStatus::Warn,
                format!("could not invoke codesign: {e}"),
            ),
        }
    }

    // 8. Live `env_clear()` cold-launch checks
    if shaderd_path.is_file() && is_macho_file(&shaderd_path) {
        match Command::new(&shaderd_path)
            .arg("--version")
            .env_clear()
            .output()
        {
            Ok(out) if out.status.success() => {
                let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if ver.starts_with("dxil-metald protocol v") {
                    push(
                        &mut checks,
                        "cold_dxil_metald",
                        BundleCheckStatus::Pass,
                        format!("env_clear() OK ({ver})"),
                    );
                } else {
                    push(
                        &mut checks,
                        "cold_dxil_metald",
                        BundleCheckStatus::Fail,
                        format!("unexpected dxil-metald --version output: {ver:?}"),
                    );
                }
            }
            Ok(out) => push(
                &mut checks,
                "cold_dxil_metald",
                BundleCheckStatus::Fail,
                format!(
                    "env_clear() failed (status {}): {}",
                    out.status,
                    String::from_utf8_lossy(&out.stderr).trim()
                ),
            ),
            Err(e) => push(
                &mut checks,
                "cold_dxil_metald",
                BundleCheckStatus::Fail,
                format!("failed to exec dxil-metald: {e}"),
            ),
        }
    }

    if let Some(ref launcher) = launcher_path {
        if launcher.is_file() && is_macho_file(launcher) {
            match Command::new(launcher)
                .arg("--aot-info")
                .env_clear()
                .output()
            {
                Ok(out) if out.status.success() => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let msc_ok = stdout.contains("Bundled Metal IR Conv  : true");
                    let helper_ok = stdout.contains("Bundled Shader Helper  : true");
                    if msc_ok && helper_ok {
                        push(
                            &mut checks,
                            "cold_launcher_info",
                            BundleCheckStatus::Pass,
                            "env_clear() --aot-info verified bundled helper + Metal IR converter"
                                .to_string(),
                        );
                    } else {
                        push(
                            &mut checks,
                            "cold_launcher_info",
                            BundleCheckStatus::Fail,
                            format!(
                                "--aot-info reported missing runtime components:\n{}",
                                stdout.trim()
                            ),
                        );
                    }
                }
                Ok(out) => push(
                    &mut checks,
                    "cold_launcher_info",
                    BundleCheckStatus::Fail,
                    format!(
                        "env_clear() --aot-info failed (status {}): {}",
                        out.status,
                        String::from_utf8_lossy(&out.stderr).trim()
                    ),
                ),
                Err(e) => push(
                    &mut checks,
                    "cold_launcher_info",
                    BundleCheckStatus::Fail,
                    format!("failed to exec launcher: {e}"),
                ),
            }
        }
    }

    Ok(BundleVerifyReport {
        app_path: app_path.to_path_buf(),
        checks,
    })
}

/// Format a [`BundleVerifyReport`] for CLI display.
pub fn format_bundle_verify_report(report: &BundleVerifyReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "=== Portable .app Bundle Verification: {} ===\n",
        report.app_path.display()
    ));
    for c in &report.checks {
        let tag = match c.status {
            BundleCheckStatus::Pass => "PASS",
            BundleCheckStatus::Warn => "WARN",
            BundleCheckStatus::Fail => "FAIL",
        };
        out.push_str(&format!("  [{tag}] {:26} {}\n", c.name, c.detail));
    }
    let verdict = if report.passed() {
        format!(
            "READY FOR SSD / ANOTHER APPLE SILICON MAC ({} checks, {} warning(s))",
            report.checks.len(),
            report.warn_count()
        )
    } else {
        format!(
            "NOT PORTABLE ({} failure(s), {} warning(s))",
            report.fail_count(),
            report.warn_count()
        )
    };
    out.push_str(&format!("Result: {verdict}\n"));
    out
}

// ============================================================================
// 8. Interactive HTML Steam Library Browser (`out/steam_library_browser.html`)
// ============================================================================

/// Export a rich, interactive dark-mode HTML Steam Library Browser & One-Click Converter UI.
pub fn write_steam_browser_html(report: &SteamLibraryScanReport, out_path: &Path) -> Result<()> {
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let user_banner = report
        .users
        .first()
        .map(|u| {
            format!(
                "{} ({}) — SteamID64: {}",
                u.persona_name, u.account_name, u.steam_id64
            )
        })
        .unwrap_or_else(|| "Local macOS Steam User".to_string());

    let mut installed_rows = String::new();
    for g in &report.installed_games {
        let size_gb = (g.size_on_disk_bytes as f64) / 1_000_000_000.0;
        let readiness = g.readiness_pct();
        let badge_color = if readiness >= 99.0 {
            "#10b981"
        } else if readiness >= 85.0 {
            "#3b82f6"
        } else {
            "#f59e0b"
        };
        let exe_name = g
            .primary_exe
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("macOS Bundle");
        let cmd_app = format!(
            "cargo opsdb -- steam-convert {} --format app --symlink-game",
            g.app_id
        );
        let cmd_aot = format!(
            "cargo opsdb -- steam-convert {} --format aot-crate",
            g.app_id
        );
        installed_rows.push_str(&format!(
            r#"<div class="card installed-card" data-name="{}" data-id="{}">
                <div class="card-header">
                    <div>
                        <span class="source-pill">{}</span>
                        <span class="appid-pill">AppID: {}</span>
                    </div>
                    <span class="readiness-pill" style="background:{badge_color}22;color:{badge_color};border:1px solid {badge_color}55">
                        {:.1}% Win32 Ready ({}/{})
                    </span>
                </div>
                <h3>{}</h3>
                <div class="meta-grid">
                    <div><strong>Executable:</strong> <code>{}</code> ({} PE modules)</div>
                    <div><strong>Engine Profile:</strong> {}</div>
                    <div><strong>Platform:</strong> {} ({:.2} GB)</div>
                    <div><strong>Install Path:</strong> <code>{}</code></div>
                </div>
                <div class="action-bar">
                    <button class="btn-primary" onclick="copyCmd('{}', this)">One-Click Convert (.app)</button>
                    <button class="btn-secondary" onclick="copyCmd('{}', this)">Convert to Rust AOT Crate</button>
                </div>
            </div>"#,
            html_escape(&g.name.to_ascii_lowercase()),
            html_escape(&g.app_id),
            html_escape(&g.source.label()),
            html_escape(&g.app_id),
            readiness,
            g.resolved_imports,
            g.total_imports,
            html_escape(&g.name),
            html_escape(exe_name),
            g.pe_count,
            html_escape(&g.engine_profile),
            html_escape(&g.platform),
            size_gb,
            html_escape(&g.install_dir.to_string_lossy()),
            html_escape(&cmd_app),
            html_escape(&cmd_aot),
        ));
    }

    let mut owned_rows = String::new();
    for o in &report.owned_games {
        let status_badge = if o.installed {
            format!(
                r#"<span class="status-installed">Installed ({})</span>"#,
                html_escape(o.install_source.as_deref().unwrap_or("Local"))
            )
        } else {
            r#"<span class="status-cloud">Owned in Steam Cloud</span>"#.to_string()
        };
        let hours = (o.playtime_minutes as f64) / 60.0;
        let dl_cmd = format!(
            "cargo opsdb -- steam-download {} && cargo opsdb -- steam-convert {} --format app",
            o.app_id, o.app_id
        );
        owned_rows.push_str(&format!(
            r#"<tr class="owned-row" data-name="{}" data-id="{}">
                <td><code>{}</code></td>
                <td><strong>{}</strong></td>
                <td>{}</td>
                <td>{}</td>
                <td>{:.1} hrs ({} cached achievements)</td>
                <td>
                    <button class="btn-small" onclick="copyCmd('{}', this)">Download Win64 &amp; Convert</button>
                </td>
            </tr>"#,
            html_escape(&o.title.to_ascii_lowercase()),
            o.app_id,
            o.app_id,
            html_escape(&o.title),
            html_escape(&o.engine_hint),
            status_badge,
            hours,
            o.achievements_cached,
            html_escape(&dl_cmd),
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Port Factory — Steam &amp; PC Game Library Converter</title>
<style>
  :root {{
    --bg: #0b0f17;
    --panel: #151c28;
    --border: #263248;
    --text: #e2e8f0;
    --muted: #94a3b8;
    --accent: #38bdf8;
    --green: #10b981;
  }}
  * {{ box-sizing: border-box; }}
  body {{
    margin: 0;
    padding: 32px;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Display", "Inter", sans-serif;
    background: var(--bg);
    color: var(--text);
  }}
  header {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border);
    padding-bottom: 20px;
    margin-bottom: 28px;
  }}
  h1 {{ margin: 0 0 6px 0; font-size: 26px; }}
  .subtitle {{ color: var(--muted); font-size: 14px; }}
  .search-box {{
    background: var(--panel);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 10px 16px;
    border-radius: 8px;
    width: 320px;
    font-size: 14px;
  }}
  .cards-grid {{
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(460px, 1fr));
    gap: 18px;
    margin-bottom: 36px;
  }}
  .card {{
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
  }}
  .card-header {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }}
  .source-pill, .appid-pill {{
    background: #1e293b;
    color: var(--accent);
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 12px;
    margin-right: 6px;
  }}
  .readiness-pill {{
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 12px;
    font-weight: 600;
  }}
  .card h3 {{ margin: 8px 0 12px 0; font-size: 20px; }}
  .meta-grid {{
    display: grid;
    gap: 6px;
    font-size: 13px;
    color: var(--muted);
    margin-bottom: 16px;
  }}
  .meta-grid strong {{ color: var(--text); }}
  code {{
    background: #0f172a;
    padding: 2px 6px;
    border-radius: 4px;
    font-family: "SF Mono", Menlo, monospace;
    font-size: 12px;
  }}
  .action-bar {{ display: flex; gap: 10px; }}
  button {{
    cursor: pointer;
    border-radius: 8px;
    padding: 9px 14px;
    font-size: 13px;
    font-weight: 600;
    border: none;
    transition: opacity 0.15s;
  }}
  button:hover {{ opacity: 0.9; }}
  .btn-primary {{ background: var(--accent); color: #082f49; }}
  .btn-secondary {{ background: #1e293b; color: var(--text); border: 1px solid var(--border); }}
  .btn-small {{ background: #1e293b; color: var(--accent); border: 1px solid var(--border); padding: 6px 12px; }}
  table {{
    width: 100%;
    border-collapse: collapse;
    background: var(--panel);
    border-radius: 12px;
    overflow: hidden;
    border: 1px solid var(--border);
  }}
  th, td {{
    text-align: left;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
    font-size: 13px;
  }}
  th {{ background: #0f172a; color: var(--muted); font-weight: 600; }}
  .status-installed {{ color: var(--green); font-weight: 600; }}
  .status-cloud {{ color: var(--muted); }}
</style>
</head>
<body>
<header>
  <div>
    <h1>Port Factory — Steam &amp; PC Game Library One-Click Converter</h1>
    <div class="subtitle">Account: {user_banner} &bull; {} Installed Targets &bull; {} Owned Steam Games</div>
  </div>
  <input type="text" id="filterInput" class="search-box" placeholder="Search games, AppID, or engine..." oninput="filterGames(this.value)">
</header>

<h2>Installed Games &amp; Clients Ready for One-Click Conversion</h2>
<div class="cards-grid">
  {installed_rows}
</div>

<h2>Owned Steam Library ({} Games — One-Click Windows x64 Depot Download + Convert)</h2>
<table>
  <thead>
    <tr>
      <th>AppID</th>
      <th>Title</th>
      <th>Engine / Translation Pipeline</th>
      <th>Status</th>
      <th>Playtime / Cache</th>
      <th>One-Click Action</th>
    </tr>
  </thead>
  <tbody>
    {owned_rows}
  </tbody>
</table>

<script>
function copyCmd(cmd, btn) {{
  navigator.clipboard.writeText(cmd);
  const orig = btn.innerText;
  btn.innerText = "Copied CLI Command!";
  setTimeout(() => {{ btn.innerText = orig; }}, 1800);
}}
function filterGames(q) {{
  const s = q.toLowerCase().trim();
  document.querySelectorAll('.installed-card, .owned-row').forEach(el => {{
    const match = el.dataset.name.includes(s) || el.dataset.id.includes(s) || el.innerText.toLowerCase().includes(s);
    el.style.display = match ? '' : 'none';
  }});
}}
</script>
</body>
</html>
"#,
        report.installed_games.len(),
        report.owned_games.len(),
        report.owned_games.len(),
    );

    fs::write(out_path, html)?;
    Ok(())
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Format a human-readable CLI summary of the [`SteamLibraryScanReport`].
pub fn format_steam_scan_report(report: &SteamLibraryScanReport) -> String {
    let mut out = String::new();
    out.push_str("=== Port Factory Steam & PC Game Library Scanner ===\n");
    if !report.users.is_empty() {
        out.push_str("  Steam Accounts:\n");
        for u in &report.users {
            out.push_str(&format!(
                "    - {} ({}) [SteamID64: {}, AccountID: {}{}]\n",
                u.persona_name,
                u.account_name,
                u.steam_id64,
                u.account_id32,
                if u.most_recent { ", MostRecent" } else { "" }
            ));
        }
    }
    out.push_str(&format!(
        "  Scanned Library Folders: {}\n",
        report.library_folders.len()
    ));
    for dir in &report.library_folders {
        out.push_str(&format!("    - {}\n", dir.display()));
    }

    out.push_str(&format!(
        "\n=== Installed Games & Clients ({}) ===\n",
        report.installed_games.len()
    ));
    out.push_str(&format!(
        "  {:18} {:28} {:28} {:>9} {:>12} {}\n",
        "APPID", "TITLE", "SOURCE", "SIZE", "WIN32 READY", "ENGINE / EXECUTABLE"
    ));
    out.push_str(&format!("  {}\n", "-".repeat(130)));
    for g in &report.installed_games {
        let size_gb = (g.size_on_disk_bytes as f64) / 1_000_000_000.0;
        let exe_label = g
            .primary_exe
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("macOS.app");
        out.push_str(&format!(
            "  {:18} {:28} {:28} {:>6.1} GB {:>6.1}% ({}/{}) {} [{}]\n",
            g.app_id,
            truncate_str(&g.name, 27),
            truncate_str(&g.source.label(), 27),
            size_gb,
            g.readiness_pct(),
            g.resolved_imports,
            g.total_imports,
            exe_label,
            g.engine_profile
        ));
    }

    out.push_str(&format!(
        "\n=== Owned Steam Library ({} Games Discovered in Local Cache) ===\n",
        report.owned_games.len()
    ));
    out.push_str(&format!(
        "  {:>10} {:34} {:16} {:>10} {}\n",
        "APPID", "TITLE", "STATUS", "PLAYTIME", "ENGINE / PIPELINE"
    ));
    out.push_str(&format!("  {}\n", "-".repeat(115)));
    for o in report.owned_games.iter().take(35) {
        let status = if o.installed { "INSTALLED" } else { "CLOUD" };
        let hrs = (o.playtime_minutes as f64) / 60.0;
        out.push_str(&format!(
            "  {:>10} {:34} {:16} {:>7.1} hr {}\n",
            o.app_id,
            truncate_str(&o.title, 33),
            status,
            hrs,
            o.engine_hint
        ));
    }
    if report.owned_games.len() > 35 {
        out.push_str(&format!(
            "  ... and {} more owned Steam games (see out/steam_library_browser.html)\n",
            report.owned_games.len() - 35
        ));
    }
    out
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        let mut t: String = chars[..max_chars.saturating_sub(1)].iter().collect();
        t.push('…');
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_acf_and_installscript_vdf() {
        let acf = r#"
"AppState"
{
    "appid"     "2357570"
    "name"      "Overwatch®"
    "installdir" "Overwatch"
    "SizeOnDisk" "72292658910"
}
"#;
        let vdf = parse_vdf(acf);
        assert_eq!(vdf.get_path(&["AppState", "appid"]).and_then(|v| v.as_str()), Some("2357570"));
        assert_eq!(vdf.get_path(&["AppState", "installdir"]).and_then(|v| v.as_str()), Some("Overwatch"));

        let install_vdf = r#"
"InstallScript"
{
    "Registry"
    {
        "HKEY_LOCAL_MACHINE\\Software\\Blizzard Entertainment\\Overwatch"
        {
            "string"
            {
                "InstallPath" "%INSTALLDIR%"
            }
            "dword"
            {
                "Installed" "1"
            }
        }
    }
    "Run Process"
    {
        "VC_Redist_x64"
        {
            "process 1" "%INSTALLDIR%\\_CommonRedist\\vcredist\\2022\\VC_redist.x64.exe"
            "command 1" "/quiet /norestart"
        }
    }
}
"#;
        let summary = parse_installscript_vdf(install_vdf);
        assert_eq!(summary.registry_entries.len(), 2);
        assert_eq!(
            summary.registry_entries[0].key_path,
            "hklm\\software\\blizzard entertainment\\overwatch"
        );
        assert_eq!(summary.registry_entries[0].value_name, "InstallPath");
        assert_eq!(summary.registry_entries[1].value_type, "REG_DWORD");
        assert_eq!(summary.run_processes.len(), 1);
        assert_eq!(summary.bypassable_redistributables.len(), 1);
    }

    struct TempDirGuard(PathBuf);
    impl Drop for TempDirGuard {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn make_temp_dir(tag: &str) -> TempDirGuard {
        let dir = std::env::temp_dir().join(format!(
            "port_factory_steam_test_{}_{}",
            tag,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp test dir");
        TempDirGuard(dir)
    }

    #[test]
    fn stages_bundle_frameworks_from_home_vulkan_sdk_and_env_overrides() {
        let root = make_temp_dir("frameworks");
        let fake_home = root.0.join("home");
        let fake_sdk = root.0.join("vulkan_sdk");
        let fake_workspace = root.0.join("ws");

        let home_msc = fake_home
            .join(".local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib");
        let home_dxilconv = fake_home.join(".local/lib/libdxilconv.dylib");
        let sdk_moltenvk = fake_sdk.join("lib/libMoltenVK.dylib");

        fs::create_dir_all(home_msc.parent().unwrap()).unwrap();
        fs::create_dir_all(home_dxilconv.parent().unwrap()).unwrap();
        fs::create_dir_all(sdk_moltenvk.parent().unwrap()).unwrap();
        fs::write(&home_msc, b"FAKE_MSC_HOME").unwrap();
        fs::write(&home_dxilconv, b"FAKE_DXILCONV_HOME").unwrap();
        fs::write(&sdk_moltenvk, b"FAKE_MVK_SDK").unwrap();

        // 1. Auto-discovery from $HOME + $VULKAN_SDK (no explicit dylib env overrides).
        let fw_dir_1 = root.0.join("Bundle1.app/Contents/Frameworks");
        let env1 = FrameworkDiscoveryEnv {
            home: Some(fake_home.clone()),
            vulkan_sdk: Some(fake_sdk.clone()),
            include_system_paths: false,
            ..Default::default()
        };
        let staged1 =
            stage_bundle_frameworks_with_env(&fw_dir_1, &fake_workspace, &env1).unwrap();
        assert_eq!(
            staged1.staged_names(),
            vec![
                "libmetalirconverter.dylib",
                "libdxilconv.dylib",
                "libMoltenVK.dylib",
            ]
        );
        assert_eq!(
            fs::read(staged1.metal_irconverter.as_ref().unwrap()).unwrap(),
            b"FAKE_MSC_HOME"
        );
        assert_eq!(
            fs::read(staged1.dxilconv.as_ref().unwrap()).unwrap(),
            b"FAKE_DXILCONV_HOME"
        );
        assert_eq!(
            fs::read(staged1.moltenvk.as_ref().unwrap()).unwrap(),
            b"FAKE_MVK_SDK"
        );

        // 2. Explicit env vars (`METAL_IRCONVERTER_DYLIB`, `AW2_DXILCONV`, `PORT_VULKAN_DYLIB`)
        //    take precedence over `$HOME` and `$VULKAN_SDK`.
        let override_dir = root.0.join("overrides");
        fs::create_dir_all(&override_dir).unwrap();
        let env_msc = override_dir.join("libmetalirconverter.dylib");
        let env_dxilconv = override_dir.join("libdxilconv.dylib");
        let env_mvk = override_dir.join("libMoltenVK.dylib");
        fs::write(&env_msc, b"FAKE_MSC_ENV").unwrap();
        fs::write(&env_dxilconv, b"FAKE_DXILCONV_ENV").unwrap();
        fs::write(&env_mvk, b"FAKE_MVK_ENV").unwrap();

        let fw_dir_2 = root.0.join("Bundle2.app/Contents/Frameworks");
        let env2 = FrameworkDiscoveryEnv {
            home: Some(fake_home.clone()),
            metal_irconverter_dylib: Some(env_msc),
            aw2_dxilconv: Some(env_dxilconv),
            port_vulkan_dylib: Some(env_mvk),
            vulkan_sdk: Some(fake_sdk),
            include_system_paths: false,
            ..Default::default()
        };
        let staged2 =
            stage_bundle_frameworks_with_env(&fw_dir_2, &fake_workspace, &env2).unwrap();
        assert_eq!(
            fs::read(staged2.metal_irconverter.as_ref().unwrap()).unwrap(),
            b"FAKE_MSC_ENV"
        );
        assert_eq!(
            fs::read(staged2.dxilconv.as_ref().unwrap()).unwrap(),
            b"FAKE_DXILCONV_ENV"
        );
        assert_eq!(
            fs::read(staged2.moltenvk.as_ref().unwrap()).unwrap(),
            b"FAKE_MVK_ENV"
        );

        // 3. Optional missing dylibs (`libdxilconv.dylib`, `libMoltenVK.dylib`) stay None.
        let fw_dir_3 = root.0.join("Bundle3.app/Contents/Frameworks");
        let env3 = FrameworkDiscoveryEnv {
            metal_irconverter_dylib: Some(home_msc),
            include_system_paths: false,
            ..Default::default()
        };
        let staged3 =
            stage_bundle_frameworks_with_env(&fw_dir_3, &fake_workspace, &env3).unwrap();
        assert!(staged3.metal_irconverter.is_some());
        assert!(staged3.dxilconv.is_none());
        assert!(staged3.moltenvk.is_none());
        assert!(!fw_dir_3.join("libdxilconv.dylib").exists());
        assert!(!fw_dir_3.join("libMoltenVK.dylib").exists());
        // 4. Symlinked read-only (0444) source dylibs (like Homebrew Cellar libraries) are copied
        //    as self-contained regular 0755 files so `codesign --force --sign -` and re-staging work.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let cellar_dir = root.0.join("cellar");
            let brew_lib = root.0.join("brew_lib");
            fs::create_dir_all(&cellar_dir).unwrap();
            fs::create_dir_all(&brew_lib).unwrap();
            let ro_target = cellar_dir.join("libMoltenVK.dylib");
            fs::write(&ro_target, b"READONLY_CELLAR_MVK").unwrap();
            fs::set_permissions(&ro_target, fs::Permissions::from_mode(0o444)).unwrap();
            let brew_link = brew_lib.join("libMoltenVK.dylib");
            std::os::unix::fs::symlink(&ro_target, &brew_link).unwrap();

            let fw_dir_4 = root.0.join("Bundle4.app/Contents/Frameworks");
            let env4 = FrameworkDiscoveryEnv {
                port_vulkan_dylib: Some(brew_link),
                include_system_paths: false,
                ..Default::default()
            };
            let staged4 =
                stage_bundle_frameworks_with_env(&fw_dir_4, &fake_workspace, &env4).unwrap();
            let dst_mvk = staged4.moltenvk.as_ref().unwrap();
            let meta = fs::symlink_metadata(dst_mvk).unwrap();
            assert!(!meta.file_type().is_symlink(), "staged framework must be a real file copy");
            assert_eq!(meta.permissions().mode() & 0o777, 0o755);
            assert_eq!(fs::read(dst_mvk).unwrap(), b"READONLY_CELLAR_MVK");

            // Re-staging over an already staged bundle must succeed cleanly too.
            let staged4_again =
                stage_bundle_frameworks_with_env(&fw_dir_4, &fake_workspace, &env4).unwrap();
            assert_eq!(staged4_again.moltenvk.as_deref(), Some(dst_mvk.as_path()));
        }
    }

    #[test]
    fn generates_and_executes_self_contained_launcher_with_empty_env() {
        let root = make_temp_dir("launcher");
        let contents_dir = root.0.join("TestGame.app/Contents");
        let macos_dir = contents_dir.join("MacOS");
        let frameworks_dir = contents_dir.join("Frameworks");
        let resources_dir = contents_dir.join("Resources");
        let aot_cache_dir = resources_dir.join("AotCache");
        let shader_cache_dir = resources_dir.join("ShaderCache");
        fs::create_dir_all(&macos_dir).unwrap();
        fs::create_dir_all(&frameworks_dir).unwrap();
        fs::create_dir_all(resources_dir.join("Game")).unwrap();
        fs::create_dir_all(&aot_cache_dir).unwrap();
        fs::create_dir_all(shader_cache_dir.join("manifest")).unwrap();
        fs::create_dir_all(shader_cache_dir.join("pso/pso_manifest")).unwrap();

        // Place fake staged frameworks, manifest, and helper binary inside the bundle.
        let fw_msc = frameworks_dir.join("libmetalirconverter.dylib");
        let fw_dxilconv = frameworks_dir.join("libdxilconv.dylib");
        let fw_moltenvk = frameworks_dir.join("libMoltenVK.dylib");
        let bundled_manifest = resources_dir.join("game.toml");
        let bundled_shaderd = macos_dir.join("dxil-metald");
        fs::write(&fw_msc, b"MSC").unwrap();
        fs::write(&fw_dxilconv, b"DXILCONV").unwrap();
        fs::write(&fw_moltenvk, b"MVK").unwrap();
        fs::write(
            &bundled_manifest,
            generate_game_manifest_toml("TestGame", "TestGame", "Bin/TestGame.exe", "12345"),
        )
        .unwrap();
        fs::write(&bundled_shaderd, b"FAKE_SHADERD").unwrap();

        let cfg = BundleLauncherConfig {
            clean_title: "TestGame",
            app_id: "12345",
            exe_rel: "Bin/TestGame.exe",
            image_base: 0x14000_0000,
            size_of_image: 0x20_0000,
            aot_fn_count: 42,
            aot_dylib_name: "libtestgame_aot.dylib",
            extra_aot_dylibs: &[],
        };
        let src = generate_bundle_launcher_source(&cfg);
        assert!(src.contains("METAL_IRCONVERTER_DYLIB"));
        assert!(src.contains("AW2_DXILCONV"));
        assert!(src.contains("PORT_VULKAN_DYLIB"));
        assert!(src.contains("PORT_MANIFEST"));
        assert!(src.contains("PORT_AOT_DIR"));
        assert!(src.contains("PORT_SHADERD"));
        assert!(src.contains("DYLD_FALLBACK_LIBRARY_PATH"));
        assert!(src.contains("libmetalirconverter.dylib"));
        assert!(src.contains("libdxilconv.dylib"));
        assert!(src.contains("libMoltenVK.dylib"));

        // Compile the generated launcher with rustc and run it with an EMPTY environment
        // (`env_clear()`, matching a cold launch from Finder) against a stub `port-runner`.
        let launcher_rs = root.0.join("launcher.rs");
        let launcher_bin = macos_dir.join("TestGame");
        fs::write(&launcher_rs, &src).unwrap();
        let compile = Command::new("rustc")
            .args([
                "--edition=2021",
                &launcher_rs.to_string_lossy(),
                "-o",
                &launcher_bin.to_string_lossy(),
            ])
            .output()
            .expect("invoke rustc for generated bundle launcher");
        assert!(
            compile.status.success(),
            "generated launcher failed to compile:\n{}",
            String::from_utf8_lossy(&compile.stderr)
        );

        // Compile a tiny native stub `port-runner` (instead of `/bin/sh`, which macOS SIP strips
        // `DYLD_*` env vars from) so we can verify both `PORT_*` and `DYLD_FALLBACK_LIBRARY_PATH`.
        let fake_runner_rs = root.0.join("fake_runner.rs");
        let fake_runner = macos_dir.join("port-runner");
        fs::write(
            &fake_runner_rs,
            r#"fn main() {
    for k in [
        "METAL_IRCONVERTER_DYLIB",
        "AW2_DXILCONV",
        "PORT_VULKAN_DYLIB",
        "DYLD_FALLBACK_LIBRARY_PATH",
        "PATH",
        "PORT_MANIFEST",
        "PORT_SHADERD",
        "AW2_SHADERD",
        "PORT_AOT_DIR",
        "AW2_DLL_AOT_DIR",
        "PORT_AOT_FALLBACK_DIR",
        "PORT_SHADER_CACHE",
        "PORT_SHADER_CACHE_FALLBACK",
        "PORT_SHADER_MANIFEST_FALLBACK",
        "PORT_PSO_MANIFEST_FALLBACK",
    ] {
        println!("{k}={}", std::env::var(k).unwrap_or_default());
    }
}
"#,
        )
        .unwrap();
        let runner_compile = Command::new("rustc")
            .args([
                "--edition=2021",
                &fake_runner_rs.to_string_lossy(),
                "-o",
                &fake_runner.to_string_lossy(),
            ])
            .output()
            .expect("invoke rustc for stub port-runner");
        assert!(
            runner_compile.status.success(),
            "stub port-runner failed to compile:\n{}",
            String::from_utf8_lossy(&runner_compile.stderr)
        );

        let parse_env_dump = |stdout: &str| -> HashMap<String, String> {
            stdout
                .lines()
                .filter_map(|l| l.split_once('='))
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        };

        // 1. Cold launch with ZERO env vars set -> all bundled frameworks, manifest, AotCache,
        //    dxil-metald, PATH, and DYLD_FALLBACK_LIBRARY_PATH resolved!
        let run_cold = Command::new(&launcher_bin)
            .env_clear()
            .output()
            .expect("run compiled launcher with empty env");
        assert!(
            run_cold.status.success(),
            "cold launch failed: {}",
            String::from_utf8_lossy(&run_cold.stderr)
        );
        let out_cold = String::from_utf8_lossy(&run_cold.stdout);
        let env_cold = parse_env_dump(&out_cold);
        let canon_fw = fs::canonicalize(&frameworks_dir).unwrap();
        let canon_macos = fs::canonicalize(&macos_dir).unwrap();
        let canon_res = fs::canonicalize(&resources_dir).unwrap();
        assert_eq!(
            fs::canonicalize(env_cold.get("METAL_IRCONVERTER_DYLIB").unwrap()).unwrap(),
            canon_fw.join("libmetalirconverter.dylib")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("AW2_DXILCONV").unwrap()).unwrap(),
            canon_fw.join("libdxilconv.dylib")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_VULKAN_DYLIB").unwrap()).unwrap(),
            canon_fw.join("libMoltenVK.dylib")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_MANIFEST").unwrap()).unwrap(),
            canon_res.join("game.toml")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_AOT_DIR").unwrap()).unwrap(),
            canon_res.join("AotCache")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("AW2_DLL_AOT_DIR").unwrap()).unwrap(),
            canon_res.join("AotCache")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_AOT_FALLBACK_DIR").unwrap()).unwrap(),
            canon_res.join("AotCache")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_SHADER_CACHE_FALLBACK").unwrap()).unwrap(),
            canon_res.join("ShaderCache")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_SHADER_MANIFEST_FALLBACK").unwrap()).unwrap(),
            canon_res.join("ShaderCache/manifest")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_PSO_MANIFEST_FALLBACK").unwrap()).unwrap(),
            canon_res.join("ShaderCache/pso/pso_manifest")
        );

        // 1b. With HOME set (normal Finder / new-Mac launch): writable caches route to $HOME
        //     while *_FALLBACK* still point read-only into Contents/Resources/!
        let fake_home = root.0.join("fake_home");
        fs::create_dir_all(&fake_home).unwrap();
        let run_home = Command::new(&launcher_bin)
            .env_clear()
            .env("HOME", &fake_home)
            .output()
            .expect("run compiled launcher with HOME");
        assert!(run_home.status.success());
        let env_home = parse_env_dump(&String::from_utf8_lossy(&run_home.stdout));
        assert_eq!(
            PathBuf::from(env_home.get("PORT_AOT_DIR").unwrap()),
            fake_home.join("Library/Caches/port-factory/aot")
        );
        assert_eq!(
            PathBuf::from(env_home.get("PORT_SHADER_CACHE").unwrap()),
            fake_home.join("Translator/ShaderCache")
        );
        assert_eq!(
            fs::canonicalize(env_home.get("PORT_AOT_FALLBACK_DIR").unwrap()).unwrap(),
            canon_res.join("AotCache")
        );
        assert_eq!(
            fs::canonicalize(env_home.get("PORT_SHADER_CACHE_FALLBACK").unwrap()).unwrap(),
            canon_res.join("ShaderCache")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("PORT_SHADERD").unwrap()).unwrap(),
            canon_macos.join("dxil-metald")
        );
        assert_eq!(
            fs::canonicalize(env_cold.get("AW2_SHADERD").unwrap()).unwrap(),
            canon_macos.join("dxil-metald")
        );
        let path_first = env_cold.get("PATH").unwrap().split(':').next().unwrap();
        assert_eq!(fs::canonicalize(path_first).unwrap(), canon_macos);
        let dyld_first = env_cold
            .get("DYLD_FALLBACK_LIBRARY_PATH")
            .unwrap()
            .split(':')
            .next()
            .unwrap();
        assert_eq!(fs::canonicalize(dyld_first).unwrap(), canon_fw);

        // 2. Explicit developer overrides are preserved instead of overwritten.
        let run_override = Command::new(&launcher_bin)
            .env_clear()
            .env("PORT_VULKAN_DYLIB", "/custom/override/libMoltenVK.dylib")
            .env("PORT_MANIFEST", "/custom/override/game.toml")
            .env("PORT_AOT_DIR", "/custom/override/aot")
            .output()
            .expect("run compiled launcher with explicit override");
        assert!(run_override.status.success());
        let env_override = parse_env_dump(&String::from_utf8_lossy(&run_override.stdout));
        assert_eq!(
            env_override.get("PORT_VULKAN_DYLIB").map(String::as_str),
            Some("/custom/override/libMoltenVK.dylib")
        );
        assert_eq!(
            env_override.get("PORT_MANIFEST").map(String::as_str),
            Some("/custom/override/game.toml")
        );
        assert_eq!(
            env_override.get("PORT_AOT_DIR").map(String::as_str),
            Some("/custom/override/aot")
        );

        // 3. When optional libMoltenVK.dylib is removed from Frameworks, PORT_VULKAN_DYLIB stays empty.
        fs::remove_file(&fw_moltenvk).unwrap();
        let run_no_vk = Command::new(&launcher_bin)
            .env_clear()
            .output()
            .expect("run compiled launcher without bundled MoltenVK");
        assert!(run_no_vk.status.success());
        let env_no_vk = parse_env_dump(&String::from_utf8_lossy(&run_no_vk.stdout));
        assert_eq!(env_no_vk.get("PORT_VULKAN_DYLIB").map(String::as_str), Some(""));
        assert_eq!(
            fs::canonicalize(env_no_vk.get("METAL_IRCONVERTER_DYLIB").unwrap()).unwrap(),
            canon_fw.join("libmetalirconverter.dylib")
        );

        // 4. Explicit `--aot-info` prints bundled framework, manifest, AotCache, and helper presence cleanly.
        let info = Command::new(&launcher_bin)
            .arg("--aot-info")
            .env_clear()
            .output()
            .expect("run --aot-info");
        assert!(info.status.success());
        let info_out = String::from_utf8_lossy(&info.stdout);
        assert!(info_out.contains("Bundled Manifest (TOML): true"));
        assert!(info_out.contains("Bundled AotCache Dir   : true"));
        assert!(info_out.contains("Bundled Shader Helper  : true"));
        assert!(info_out.contains("Bundled Metal IR Conv  : true"));
        assert!(info_out.contains("Bundled DXILConv (DX11): true"));
        assert!(info_out.contains("Bundled MoltenVK (VK)  : false"));
    }

    #[test]
    fn parses_vtool_macos_versions_and_adopts_repo_manifests() {
        let vtool_out = r#"/tmp/libdxilconv.dylib (architecture x86_64):
Load command 10
      cmd LC_BUILD_VERSION
  cmdsize 32
 platform MACOS
    minos 13.0
      sdk 26.4
/tmp/libdxilconv.dylib (architecture arm64):
Load command 10
      cmd LC_BUILD_VERSION
  cmdsize 32
 platform MACOS
    minos 27.2
      sdk 27.2
"#;
        let parsed = parse_vtool_macos_versions(vtool_out);
        assert_eq!(
            parsed,
            vec![
                ("13.0".to_string(), "26.4".to_string()),
                ("27.2".to_string(), "27.2".to_string()),
            ]
        );

        let root = make_temp_dir("repo_manifest");
        let game_dir = root.0.join("games/sample");
        fs::create_dir_all(&game_dir).unwrap();
        fs::write(
            game_dir.join("game.toml"),
            "[app]\nid = \"sample\"\nname = \"Sample\"\nexe = \"Sample.exe\"\nsteam_app_id = \"424242\"\n\n[runtime]\nbuiltin_overrides = \"-bink2w64.dll\"\n",
        )
        .unwrap();

        let adopted = find_repo_game_manifest(&root.0, "424242", "Bin/Sample.exe")
            .expect("should match repo manifest by steam_app_id");
        assert!(adopted.contains("exe = \"Bin/Sample.exe\""));
        assert!(adopted.contains("builtin_overrides = \"-bink2w64.dll\""));
        assert!(adopted.contains("aot_dir = \"AotCache\""));

        let by_exe = find_repo_game_manifest(&root.0, "custom", "Sample.exe")
            .expect("should match repo manifest by exe basename");
        assert!(by_exe.contains("builtin_overrides = \"-bink2w64.dll\""));
        assert!(find_repo_game_manifest(&root.0, "999999", "Other.exe").is_none());
    }

    #[test]
    fn copies_only_portable_shader_cache_and_flags_nonportable_bundles() {
        let root = make_temp_dir("verify_bundle");
        let src_cache = root.0.join("host_shader_cache");
        fs::create_dir_all(src_cache.join("ir_abc123")).unwrap();
        fs::create_dir_all(src_cache.join("manifest")).unwrap();
        fs::create_dir_all(src_cache.join("pso/pso_manifest")).unwrap();
        fs::create_dir_all(src_cache.join("pso/apple_m3_ultra_26C5047e")).unwrap();
        fs::write(src_cache.join("ir_abc123/shader1.aw2s"), b"IR").unwrap();
        fs::write(src_cache.join("manifest/records.aw2m"), b"MAN").unwrap();
        fs::write(src_cache.join("pso/pso_manifest/records.p2m"), b"PSO").unwrap();
        fs::write(
            src_cache.join("pso/apple_m3_ultra_26C5047e/pso.mtl4a"),
            b"HOST_SPECIFIC",
        )
        .unwrap();

        let dst_cache = root.0.join("dst_shader_cache");
        let copied = copy_portable_shader_cache(&src_cache, &dst_cache).unwrap();
        assert_eq!(copied, 3);
        assert!(dst_cache.join("ir_abc123/shader1.aw2s").is_file());
        assert!(dst_cache.join("manifest/records.aw2m").is_file());
        assert!(dst_cache.join("pso/pso_manifest/records.p2m").is_file());
        assert!(!dst_cache.join("pso/apple_m3_ultra_26C5047e/pso.mtl4a").exists());

        // Build a broken .app bundle (symlinked Game, missing libmetalirconverter, stray .mtl4a)
        // and ensure `verify_app_bundle` catches every single portability violation.
        let external_game = root.0.join("external_game");
        fs::create_dir_all(&external_game).unwrap();
        let bad_app = root.0.join("Bad.app");
        let contents = bad_app.join("Contents");
        fs::create_dir_all(contents.join("MacOS")).unwrap();
        fs::create_dir_all(contents.join("Frameworks")).unwrap();
        fs::create_dir_all(contents.join("Resources/ShaderCache/pso")).unwrap();
        fs::write(
            contents.join("Info.plist"),
            generate_info_plist("Bad", "Bad", "1"),
        )
        .unwrap();
        std::os::unix::fs::symlink(&external_game, contents.join("Resources/Game")).unwrap();
        fs::write(
            contents.join("Resources/ShaderCache/pso/host.mtl4a"),
            b"BLOAT",
        )
        .unwrap();

        let report = verify_app_bundle(&bad_app).unwrap();
        assert!(!report.passed());
        let failed_names: BTreeSet<&str> = report
            .checks
            .iter()
            .filter(|c| c.status == BundleCheckStatus::Fail)
            .map(|c| c.name.as_str())
            .collect();
        assert!(failed_names.contains("game_payload"));
        assert!(failed_names.contains("external_symlinks"));
        assert!(failed_names.contains("frameworks"));
        assert!(failed_names.contains("shader_cache_portability"));

        // `export_app_bundle` must refuse to export a failing bundle unless `force == true`.
        let export_dest = root.0.join("FakeSSD");
        assert!(export_app_bundle(&bad_app, &export_dest, false).is_err());
    }

    #[test]
    fn parses_depot_downloader_release_and_scans_managed_downloads() {
        let release_json = r#"{
  "tag_name": "DepotDownloader_3.4.0",
  "assets": [
    {"name": "DepotDownloader-linux-x64.zip", "browser_download_url": "https://example.invalid/DepotDownloader-linux-x64.zip"},
    {"name": "DepotDownloader-macos-arm64.zip", "browser_download_url": "https://example.invalid/DepotDownloader-macos-arm64.zip"},
    {"name": "DepotDownloader-macos-x64.zip", "browser_download_url": "https://example.invalid/DepotDownloader-macos-x64.zip"}
  ]
}"#;
        let (tag, url) = parse_depot_downloader_release_asset(release_json, "arm64").unwrap();
        assert_eq!(tag, "DepotDownloader_3.4.0");
        assert_eq!(url, "https://example.invalid/DepotDownloader-macos-arm64.zip");

        let root = make_temp_dir("managed_dl");
        let game_dir = root.0.join("Control");
        fs::create_dir_all(&game_dir).unwrap();
        fs::write(
            game_dir.join(".port-factory-game.tsv"),
            "app_id\t870780\ntitle\tControl Ultimate Edition\n",
        )
        .unwrap();
        fs::write(game_dir.join("steam_appid.txt"), "870780\n").unwrap();

        let db = Win32Database::default();
        let scanned = scan_port_factory_downloads_dir(&root.0, &db);
        assert_eq!(scanned.len(), 1);
        assert_eq!(scanned[0].app_id, "870780");
        assert_eq!(scanned[0].name, "Control Ultimate Edition");
        assert_eq!(scanned[0].source, GameLibrarySource::PortFactoryLibrary);

        // Verify `resolve_steam_target_app_id` matches both numeric AppID and case-insensitive title.
        let report = SteamLibraryScanReport {
            users: Vec::new(),
            library_folders: Vec::new(),
            installed_games: Vec::new(),
            owned_games: vec![OwnedSteamGame {
                app_id: 870780,
                title: "Control Ultimate Edition".to_string(),
                engine_hint: "NORTHLIGHT_DX12".to_string(),
                installed: false,
                install_source: None,
                playtime_minutes: 120,
                last_played_unix: 0,
                achievements_cached: 0,
            }],
        };
        let (id1, name1, _) = resolve_steam_target_app_id(&report, "control").unwrap();
        assert_eq!(id1, 870780);
        assert_eq!(name1, "Control Ultimate Edition");
        let (id2, name2, _) = resolve_steam_target_app_id(&report, "870780").unwrap();
        assert_eq!(id2, 870780);
        assert_eq!(name2, "Control Ultimate Edition");
        assert_eq!(
            parse_qr_cached_username(
                "Success! Next time you can login with -username alice123 -remember-password instead of -qr."
            ),
            Some("alice123".to_string())
        );
        assert_eq!(parse_qr_cached_username("no login line here"), None);
    }
}
