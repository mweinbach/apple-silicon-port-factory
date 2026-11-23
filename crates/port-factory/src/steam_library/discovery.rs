//! Multi-source Steam, CrossOver, Heroic/Epic, and external drive game library discovery
//! plus the offline known-AppID catalog.

use super::vdf::{parse_vdf, scan_post_install, PostInstallSummary};
use crate::game_profiles::detect_pe_game_profile;
use crate::scanner::collect_pe_files;
use crate::win32_db::{audit_pe_imports, Win32Database};
use anyhow::{bail, Context, Result};
use pe_image::{ExportTarget, Pe};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ============================================================================
// 1. Steam User Profile, Installed Game, and Owned Library Models
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
// 2. Well-Known Steam AppID Catalog (Titles + Engine Profiles)
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
// 3. Default Paths & Title Sanitization
// ============================================================================

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

pub(super) fn dir_size_bytes_shallow_fast(dir: &Path) -> u64 {
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

// ============================================================================
// 4. Multi-Source Steam & Game Library Scanner
// ============================================================================

/// Scan all Steam installations (macOS Steam, CrossOver Windows Steam bottles,
/// external drives), Heroic/Epic libraries, and Windows `steam.exe` on the host.
pub fn scan_all_steam_libraries(workspace_root: &Path) -> Result<SteamLibraryScanReport> {
    let db = Win32Database::from_workspace_src(workspace_root)?;
    let home = std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/Users/Shared"));

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

fn resolve_library_folder_path(steam_root: &Path, raw_path: &str) -> PathBuf {
    if raw_path.starts_with('/') {
        return PathBuf::from(raw_path);
    }
    // Windows path inside a CrossOver bottle, e.g. `C:\Program Files (x86)\Steam`
    if raw_path.len() >= 3 && raw_path.as_bytes()[1] == b':' {
        // Find `drive_c` ancestor of `steam_root`
        for anc in steam_root.ancestors() {
            if anc.file_name().and_then(|s| s.to_str()) == Some("drive_c") {
                let rel = raw_path[2..]
                    .trim_start_matches(['\\', '/'])
                    .replace('\\', "/");
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

#[allow(clippy::too_many_arguments)]
pub(super) fn inspect_game_directory(
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

pub(super) fn extract_json_str_after_colon(s: &str) -> Option<String> {
    let (_, after_colon) = s.split_once(':')?;
    let trimmed = after_colon.trim_start();
    let rest = trimmed.strip_prefix('"')?;
    let (val, _) = rest.split_once('"')?;
    Some(val.to_string())
}

pub(super) fn extract_json_field(s: &str, field: &str) -> Option<String> {
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
