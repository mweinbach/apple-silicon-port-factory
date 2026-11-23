//! Universal game-plugin trait and process-wide registry for `x86-runtime`.
//!
//! Decouples game-specific metadata, runtime hooks, replacement tables, and
//! post-map patches from the universal PE loader. A game crate implements
//! [`GamePlugin`] and registers a `&'static` instance at startup via
//! [`register_plugin`]; manifest-only games register no plugin at all.

use crate::registry_types::Replacement;
use std::sync::RwLock;

/// Static and dynamic function-replacement tables supplied by a [`GamePlugin`].
pub struct ReplacementSet {
    /// Hand-decompiled / base replacement entries.
    pub base: Vec<&'static Replacement>,
    /// All replacement entries (base + any optional generated shards).
    pub all: Vec<&'static Replacement>,
    /// Total count of replacements across all active tables.
    pub total_count: usize,
    /// Virtual addresses of replacements that return floating-point/vector values in `xmm0`.
    pub float_return_vas: &'static [u64],
    /// Virtual addresses of functions with non-standard calling conventions that must not be hooked.
    pub special_abi_vas: &'static [u64],
    /// Preferred PE `ImageBase` used to compute relative offsets for replacement VAs.
    pub preferred_exe_base: u64,
}

/// Callback supplied by the loader during [`GamePlugin::on_post_map`] to install
/// an executable hook at `(addr, optional_func_end, target) -> Result<trampoline, String>`.
pub type HookInstaller<'a> =
    &'a mut dyn FnMut(usize, Option<usize>, usize) -> Result<usize, String>;

/// Loader context passed to [`GamePlugin::on_post_map`] after the target executable
/// has been mapped into memory and before guest execution starts.
pub struct GamePostMapContext<'a> {
    /// Actual runtime base address where the target PE was mapped.
    pub exe_base: usize,
    /// Preferred PE `ImageBase` from the image header.
    pub preferred_exe_base: u64,
    /// True when the loader was invoked with `--audio-harness`.
    pub audio_harness: bool,
    /// Loader hook installation callback.
    pub install_hook: HookInstaller<'a>,
}

/// Default application identity, runtime policy flags, and DLL routing overrides
/// contributed by a [`GamePlugin`] when not overridden by a manifest or CLI flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GamePluginDefaults {
    pub app_name: &'static str,
    pub exe: &'static str,
    pub log_name: &'static str,
    pub log_dir: &'static str,
    pub computer_name: &'static str,
    pub preferred_base_retry: bool,
    pub legacy_remedy_migration: bool,
    pub v8_jitless: bool,
    pub dll_routes: &'static [(&'static str, &'static str)],
}

/// Extension interface implemented by optional per-game native crates.
pub trait GamePlugin: Send + Sync {
    /// Unique lowercase identifier for this plugin (used for deduplication and matching).
    fn id(&self) -> &'static str;

    /// Returns `true` if this plugin applies to the given manifest ID or normalized executable name.
    fn matches(&self, manifest_id: Option<&str>, exe_norm: &str) -> bool;

    /// Default configuration values for this game target.
    fn defaults(&self) -> GamePluginDefaults;

    /// Invoked before mapping the PE image to install runtime tables and middleware hooks.
    fn on_pre_load(&self);

    /// Optional native function replacement tables for this game executable.
    fn replacements(&self) -> Option<ReplacementSet> {
        None
    }

    /// Optional post-map hook invoked after the executable is mapped into memory.
    fn on_post_map(&self, _ctx: &mut GamePostMapContext<'_>) -> Result<(), String> {
        Ok(())
    }
}

static PLUGINS: RwLock<Vec<&'static dyn GamePlugin>> = RwLock::new(Vec::new());

/// Register a game plugin in the global registry.
///
/// Deduplicates by [`GamePlugin::id`] so repeated calls for the same plugin are idempotent.
pub fn register_plugin(plugin: &'static dyn GamePlugin) {
    let mut plugins = PLUGINS.write().unwrap();
    if !plugins.iter().any(|existing| existing.id() == plugin.id()) {
        plugins.push(plugin);
    }
}

/// Find the first registered plugin matching `manifest_id` or `exe_norm`.
pub fn find_plugin(manifest_id: Option<&str>, exe_norm: &str) -> Option<&'static dyn GamePlugin> {
    let plugins = PLUGINS.read().ok()?;
    plugins
        .iter()
        .copied()
        .find(|p| p.matches(manifest_id, exe_norm))
}

/// Return the default executable name of the primary (first non-empty) registered plugin.
pub fn primary_default_exe() -> Option<&'static str> {
    let plugins = PLUGINS.read().ok()?;
    plugins
        .iter()
        .map(|p| p.defaults().exe)
        .find(|exe| !exe.is_empty())
}

/// Return a snapshot of all currently registered plugins in registration order.
pub fn registered_plugins() -> Vec<&'static dyn GamePlugin> {
    PLUGINS
        .read()
        .map_or_else(|_| Vec::new(), |plugins| plugins.clone())
}

/// Remove all registered plugins (test-only helper).
#[cfg(test)]
pub fn clear_plugins_for_test() {
    PLUGINS.write().unwrap().clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    struct MinimalPlugin;

    impl GamePlugin for MinimalPlugin {
        fn id(&self) -> &'static str {
            "minimal-game"
        }

        fn matches(&self, manifest_id: Option<&str>, exe_norm: &str) -> bool {
            manifest_id.is_some_and(|id| id.eq_ignore_ascii_case("minimal-game"))
                || exe_norm.eq_ignore_ascii_case("minimal.exe")
        }

        fn defaults(&self) -> GamePluginDefaults {
            GamePluginDefaults {
                app_name: "Minimal Game",
                exe: "Minimal.exe",
                log_name: "Minimal.log",
                log_dir: "MinimalPort",
                computer_name: "MINIMAL",
                preferred_base_retry: false,
                legacy_remedy_migration: false,
                v8_jitless: false,
                dll_routes: &[],
            }
        }

        fn on_pre_load(&self) {}
    }

    struct DuplicateIdPlugin;

    impl GamePlugin for DuplicateIdPlugin {
        fn id(&self) -> &'static str {
            "minimal-game"
        }

        fn matches(&self, _manifest_id: Option<&str>, _exe_norm: &str) -> bool {
            false
        }

        fn defaults(&self) -> GamePluginDefaults {
            GamePluginDefaults {
                app_name: "Duplicate",
                exe: "Duplicate.exe",
                log_name: "Duplicate.log",
                log_dir: "DuplicatePort",
                computer_name: "DUP",
                preferred_base_retry: false,
                legacy_remedy_migration: false,
                v8_jitless: false,
                dll_routes: &[],
            }
        }

        fn on_pre_load(&self) {}
    }

    struct EmptyExePlugin;

    impl GamePlugin for EmptyExePlugin {
        fn id(&self) -> &'static str {
            "empty-exe"
        }

        fn matches(&self, manifest_id: Option<&str>, _exe_norm: &str) -> bool {
            manifest_id == Some("empty-exe")
        }

        fn defaults(&self) -> GamePluginDefaults {
            GamePluginDefaults {
                app_name: "Empty Exe",
                exe: "",
                log_name: "Empty.log",
                log_dir: "EmptyPort",
                computer_name: "EMPTY",
                preferred_base_retry: false,
                legacy_remedy_migration: false,
                v8_jitless: false,
                dll_routes: &[],
            }
        }

        fn on_pre_load(&self) {}
    }

    struct RichPlugin {
        pre_load_calls: AtomicUsize,
    }

    static RICH_REPLACEMENT: Replacement =
        Replacement::draft(0x1_4000_1000, "demo::sample_fn", std::ptr::null());
    static RICH_FLOAT_VAS: &[u64] = &[0x1_4000_2000];
    static RICH_SPECIAL_VAS: &[u64] = &[0x1_4000_3000];
    static RICH_DLL_ROUTES: &[(&str, &str)] = &[("physics_x64.dll", "guest")];

    impl GamePlugin for RichPlugin {
        fn id(&self) -> &'static str {
            "rich-game"
        }

        fn matches(&self, manifest_id: Option<&str>, exe_norm: &str) -> bool {
            manifest_id.is_some_and(|id| id.eq_ignore_ascii_case("rich-game"))
                || exe_norm.eq_ignore_ascii_case("richgame.exe")
        }

        fn defaults(&self) -> GamePluginDefaults {
            GamePluginDefaults {
                app_name: "Rich Game",
                exe: "RichGame.exe",
                log_name: "RichGame.log",
                log_dir: "RichPort",
                computer_name: "RICHPORT",
                preferred_base_retry: true,
                legacy_remedy_migration: false,
                v8_jitless: true,
                dll_routes: RICH_DLL_ROUTES,
            }
        }

        fn on_pre_load(&self) {
            self.pre_load_calls.fetch_add(1, Ordering::Relaxed);
        }

        fn replacements(&self) -> Option<ReplacementSet> {
            Some(ReplacementSet {
                base: vec![&RICH_REPLACEMENT],
                all: vec![&RICH_REPLACEMENT],
                total_count: 1,
                float_return_vas: RICH_FLOAT_VAS,
                special_abi_vas: RICH_SPECIAL_VAS,
                preferred_exe_base: 0x1_4000_0000,
            })
        }

        fn on_post_map(&self, ctx: &mut GamePostMapContext<'_>) -> Result<(), String> {
            if ctx.audio_harness {
                let target = ctx.exe_base.wrapping_add(0x1000);
                (ctx.install_hook)(target, Some(target + 0x40), 0xdead_beef)?;
            }
            Ok(())
        }
    }

    static MINIMAL_PLUGIN: MinimalPlugin = MinimalPlugin;
    static DUPLICATE_ID_PLUGIN: DuplicateIdPlugin = DuplicateIdPlugin;
    static EMPTY_EXE_PLUGIN: EmptyExePlugin = EmptyExePlugin;
    static RICH_PLUGIN: RichPlugin = RichPlugin {
        pre_load_calls: AtomicUsize::new(0),
    };

    #[test]
    fn empty_registry_behavior() {
        let _guard = TEST_SERIAL.lock().unwrap();
        clear_plugins_for_test();

        assert!(registered_plugins().is_empty());
        assert_eq!(primary_default_exe(), None);
        assert!(find_plugin(None, "anything.exe").is_none());
        assert!(find_plugin(Some("minimal-game"), "minimal.exe").is_none());
    }

    #[test]
    fn register_deduplicates_by_id_and_preserves_first() {
        let _guard = TEST_SERIAL.lock().unwrap();
        clear_plugins_for_test();

        register_plugin(&MINIMAL_PLUGIN);
        register_plugin(&MINIMAL_PLUGIN);
        register_plugin(&DUPLICATE_ID_PLUGIN);

        let list = registered_plugins();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id(), "minimal-game");
        assert_eq!(list[0].defaults().app_name, "Minimal Game");
        assert_eq!(primary_default_exe(), Some("Minimal.exe"));

        clear_plugins_for_test();
    }

    #[test]
    fn find_plugin_matches_manifest_or_exe_case_insensitively() {
        let _guard = TEST_SERIAL.lock().unwrap();
        clear_plugins_for_test();

        register_plugin(&MINIMAL_PLUGIN);
        register_plugin(&RICH_PLUGIN);

        assert_eq!(
            find_plugin(Some("MINIMAL-GAME"), "other.exe").map(|p| p.id()),
            Some("minimal-game")
        );
        assert_eq!(
            find_plugin(None, "Minimal.EXE").map(|p| p.id()),
            Some("minimal-game")
        );
        assert_eq!(
            find_plugin(Some("rich-game"), "other.exe").map(|p| p.id()),
            Some("rich-game")
        );
        assert_eq!(
            find_plugin(None, "RICHGAME.EXE").map(|p| p.id()),
            Some("rich-game")
        );
        assert!(find_plugin(Some("unknown"), "unknown.exe").is_none());

        clear_plugins_for_test();
    }

    #[test]
    fn primary_default_exe_skips_empty_exe_names() {
        let _guard = TEST_SERIAL.lock().unwrap();
        clear_plugins_for_test();

        register_plugin(&EMPTY_EXE_PLUGIN);
        assert_eq!(primary_default_exe(), None);

        register_plugin(&RICH_PLUGIN);
        assert_eq!(primary_default_exe(), Some("RichGame.exe"));

        clear_plugins_for_test();
    }

    #[test]
    fn default_trait_methods_and_rich_hooks() {
        let _guard = TEST_SERIAL.lock().unwrap();
        clear_plugins_for_test();

        // Minimal plugin uses default implementations for replacements() and on_post_map().
        assert!(MINIMAL_PLUGIN.replacements().is_none());
        let mut called = 0usize;
        let mut installer = |_at: usize, _end: Option<usize>, _target: usize| {
            called += 1;
            Ok(0x1234)
        };
        let mut ctx = GamePostMapContext {
            exe_base: 0x2_0000_0000,
            preferred_exe_base: 0x1_4000_0000,
            audio_harness: true,
            install_hook: &mut installer,
        };
        assert!(MINIMAL_PLUGIN.on_post_map(&mut ctx).is_ok());
        assert_eq!(called, 0);

        // Rich plugin provides replacements, pre-load side effect, and post-map hook installation.
        let before = RICH_PLUGIN.pre_load_calls.load(Ordering::Relaxed);
        RICH_PLUGIN.on_pre_load();
        assert_eq!(RICH_PLUGIN.pre_load_calls.load(Ordering::Relaxed), before + 1);

        let reps = RICH_PLUGIN.replacements().expect("expected ReplacementSet");
        assert_eq!(reps.base.len(), 1);
        assert_eq!(reps.all.len(), 1);
        assert_eq!(reps.total_count, 1);
        assert_eq!(reps.float_return_vas, RICH_FLOAT_VAS);
        assert_eq!(reps.special_abi_vas, RICH_SPECIAL_VAS);
        assert_eq!(reps.preferred_exe_base, 0x1_4000_0000);
        assert_eq!(RICH_PLUGIN.defaults().dll_routes, RICH_DLL_ROUTES);

        let mut recorded = Vec::new();
        let mut rich_installer = |at: usize, end: Option<usize>, target: usize| {
            recorded.push((at, end, target));
            Ok(0x5678)
        };
        let mut rich_ctx = GamePostMapContext {
            exe_base: 0x2_0000_0000,
            preferred_exe_base: 0x1_4000_0000,
            audio_harness: true,
            install_hook: &mut rich_installer,
        };
        assert!(RICH_PLUGIN.on_post_map(&mut rich_ctx).is_ok());
        assert_eq!(
            recorded,
            vec![(0x2_0000_1000, Some(0x2_0000_1040), 0xdead_beef)]
        );

        // Error propagation from install_hook.
        let mut failing_installer = |_at: usize, _end: Option<usize>, _target: usize| {
            Err("hook failed".to_string())
        };
        let mut err_ctx = GamePostMapContext {
            exe_base: 0x2_0000_0000,
            preferred_exe_base: 0x1_4000_0000,
            audio_harness: true,
            install_hook: &mut failing_installer,
        };
        assert_eq!(
            RICH_PLUGIN.on_post_map(&mut err_ctx),
            Err("hook failed".to_string())
        );
    }
}
