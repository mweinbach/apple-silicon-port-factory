//! Host application identity and runtime configuration for the Win32 OS layer.
//!
//! Call [`set_app_config`] at process startup before creating windows or initializing
//! virtual paths so AppKit menus, window titles, `COMPUTERNAME`, log directories, and
//! game-specific prefix migrations reflect the active target rather than a hardcoded game.

use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Human-readable application name used for macOS menus (`Hide/Quit <name>`) and
    /// fallback window titles when a Win32 window passes an empty title.
    pub app_name: String,
    /// Windows `COMPUTERNAME` environment variable and `GetComputerNameA/W` result.
    pub computer_name: String,
    /// Subdirectory name under `~/Library/Logs/` for crash/hitch/boot logs.
    pub log_dir_name: String,
    /// When `true`, copies legacy `~/Library/Application Support/AW2Port/drive_c/.../Remedy`
    /// save data into `~/Translator` on first run. Off by default for generic games.
    pub legacy_remedy_migration: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "Translator".to_string(),
            computer_name: "WIN64PORT".to_string(),
            log_dir_name: "Translator".to_string(),
            legacy_remedy_migration: false,
        }
    }
}

static APP_CONFIG: RwLock<Option<AppConfig>> = RwLock::new(None);

/// Configure the active application identity (may be called during startup before UI/VFS init).
pub fn set_app_config(cfg: AppConfig) {
    *APP_CONFIG.write().unwrap() = Some(cfg);
}

/// Snapshot of the current [`AppConfig`].
pub fn app_config() -> AppConfig {
    APP_CONFIG
        .read()
        .unwrap()
        .clone()
        .unwrap_or_default()
}
