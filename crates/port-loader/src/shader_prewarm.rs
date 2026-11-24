//! Cold-device Metal 4 shader & PSO prewarm orchestration with native macOS progress UI.

use win32_rt as w;

/// On first launch on a new Mac or after a macOS update (0 cached MTL4Archives for this
/// GPU+OS build), compile any recorded shader/PSO manifests in parallel before starting
/// the Windows entry point — showing a native macOS progress window in GUI launches so
/// double-clicked `.app` bundles never look frozen during first-time shader compilation.
pub(crate) fn run_cold_device_prewarm() {
    let mode = std::env::var("PORT_PREWARM_SHADERS").unwrap_or_default();
    let lower = mode.trim().to_ascii_lowercase();
    if lower == "0" || lower == "off" {
        return;
    }
    let only_if_cold = !(lower == "1" || lower == "force");
    let Some(device_id) = d3d12_metal::metal::pso_manifest::prewarm_needed(only_if_cold) else {
        return;
    };
    w::hitch::heartbeat_stage(w::hitch::HeartbeatKind::LoaderMap, "shader_prewarm");
    let res = if w::macos::active() {
        let app_name = w::config::app_config().app_name;
        w::macos::progress_window_open(
            format!("{app_name} — Preparing Shaders"),
            format!("Compiling Metal 4 shaders for {device_id}..."),
            0,
        );
        let done_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let done_flag_bg = done_flag.clone();
        let last_tick = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let bg = std::thread::spawn(move || {
            let r = d3d12_metal::metal::pso_manifest::prewarm_for_current_device_with_progress(
                None,
                only_if_cold,
                |p| {
                    w::hitch::heartbeat(w::hitch::HeartbeatKind::LoaderMap);
                    let prev = last_tick.load(std::sync::atomic::Ordering::Relaxed);
                    if p.done == 0
                        || p.done >= p.total
                        || p.done.saturating_sub(prev) >= 4
                        || p.total <= 32
                    {
                        last_tick.store(p.done, std::sync::atomic::Ordering::Relaxed);
                        w::macos::progress_window_update(
                            format!(
                                "{}: {} / {} ({})",
                                p.stage, p.done, p.total, p.device_identity
                            ),
                            p.done,
                            p.total,
                        );
                    }
                },
            );
            done_flag_bg.store(true, std::sync::atomic::Ordering::Release);
            r
        });
        let _ = w::macos::run_main_loop(&|| {
            done_flag.load(std::sync::atomic::Ordering::Acquire).then_some(0)
        });
        w::macos::progress_window_close();
        bg.join().unwrap_or_else(|_| Err("shader prewarm thread panicked".into()))
    } else {
        d3d12_metal::metal::pso_manifest::prewarm_for_current_device_with_progress(
            None,
            only_if_cold,
            |_| {
                w::hitch::heartbeat(w::hitch::HeartbeatKind::LoaderMap);
            },
        )
    };
    w::hitch::heartbeat_stage(w::hitch::HeartbeatKind::LoaderMap, "shader_prewarm_done");
    match res {
        Ok(rep) if !rep.skipped_warm && (rep.tier1_total > 0 || rep.pso_total > 0) => {
            log::info!(
                "cold-device shader prewarm for {}: tier1 {}/{}, pso {}/{} in {} ms",
                rep.device_identity,
                rep.tier1_compiled,
                rep.tier1_total,
                rep.pso_compiled,
                rep.pso_total,
                rep.elapsed_ms
            );
        }
        Ok(_) => {}
        Err(e) => log::warn!("cold-device shader prewarm skipped: {e}"),
    }
}
