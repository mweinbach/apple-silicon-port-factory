//! Encoder/barrier counters for GPU submission diagnostics (`AW2_ENC_STATS=1`).
//!
//! Relaxed atomic increments only; [`dump`] logs per-frame averages since the
//! previous dump.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
#[cfg(not(test))]
use std::sync::atomic::AtomicU8;

macro_rules! counters {
    ($($name:ident),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        pub mod c {
            use super::AtomicU64;
            $(pub static $name: AtomicU64 = AtomicU64::new(0);)*
        }
        const NAMES: &[&str] = &[$(stringify!($name)),*];
        fn all() -> &'static [&'static AtomicU64] {
            static ALL: &[&AtomicU64] = &[$(&c::$name),*];
            ALL
        }
    };
}

counters!(
    draw_pass,
    clear_pass,
    util_pass,
    compute_enc,
    q_barrier_all,
    q_barrier_frag,
    enc_barrier,
    app_barrier,
    app_barrier_split,
    clear_folded,
    clear_deferred,
    occl_begin,
    ei_expand,
    q_resolve,
    ei_calls,
    ei_max_count,
    ei_draws,
    ei_icb,
    ei_icb_new,
    lists,
    qb_list,
    qb_app_full,
    qb_clear_uav,
    qb_internal,
    qb_written,
    qb_frag_to_vtx,
    qb_masked_other,
    clear_cs,
    dv_defer,
    dv_hit,
    dv_carry,
    cs_split,
    cs_mid_qb,
    cs_keep_gfx,
    rb_keep,
    rb_mid_qb,
    cs_defer_gfx,
    qw_event,
    res_add,
    res_remove,
    res_commit,
    present_pq,
    present_drop,
    present_mbox,
    present_bbwait,
    present_acq_us,
    qw_elided,
    cf_close,
    cf_barrier_full,
    cf_barrier_uav0,
    cf_barrier_res,
    cf_defer,
    cf_rect,
    cf_view,
    cf_resolve,
    cf_copy,
    cp_dsv,
    cp_rtv_fmt,
    cp_rtv_nows,
    cx_view,
    cx_num,
    disc_call,
    disc_whole,
    disc_hit,
    disc_dc,
    rb_r2r,
    rb_r2r_elided,
    rb_noop,
    qb_upscale_masked,
    cf_uav0_kept,
    ib_skip_stage,
    ib_skip_touch,
    ib_masked,
    rb_keep_write,
    rb_uav_split,
    cf_smart_keep,
    icb_hoist_saved,
    cf_dsv_keep,
    ib_skip_exact,
    present_sq,
    present_sq_wait,
    qb_list_tight,
    qb_list_trail,
    bind_skip,
    empty_list_elided,
    blit_list_narrow,
    uav0_clear_overlap,
    clear_pass_merged,
    enc_state_skip,
    clear_overlap_exact,
    subrange_uav_skip,
    cs_uav_mid_skip,
    submit_coalesced,
    submit_nosig,
    copy_q_light,
    stagesync_skip,
    stagesync_narrow,
    clear_tex_cs_keep,
    draw_pass_no_uav,
    msc_access_exact,
    ib_class_skip,
    clear_cs_exact_sync,
    present_blit_narrow,
    icb_v3_fast,
    fence_sig_coalesced,
    render_split_sync,
    ro_dsv_dontcare,
    res_soft_sync,
    cs_uav_prod_exact,
    split_cs_substage,
    split_cs_dom_fold,
    msc_stage_usage_hit,
    render_dom_fold,
    air_no_res_heap_pso,
    vs_buf_defer,
    cs_mid_qb_defer,
    ei_cs_exact,
    blit_list_v2,
    list_start_substage,
    sm66_gfx_exact,
    cs_mid_qb_v2,
    ib_substage_narrow,
    sm66_cs_exact,
    list_start_rsplit,
    cs_mid_qb_v3,
    rp_keep_exact,
    ib_uav0_exact,
    clear_prelist_overlap,
    cs_mid_qb_v4,
    ps_srv_class_exact,
    desc_table_cs_exact,
    gfx_desc_table_exact,
    cs_mid_qb_v5,
    icb_enc_barrier_exact,
    icb_prepass_cs,
    desc_table_cs_fast,
    rp_split_exact_keep,
    clear_cs_tile4,
    list_boundary_overlap,
    cs_intra_barrier_exact,
    clear_pass_fold_v2,
    cs_to_ps_overlap,
    cs_to_ps_overlap_v2,
    gfx_to_ps_overlap,
    clear_isolate_coalesce,
    queue_start_stage_exact,
    ps_to_cs_overlap,
    gfx_to_ps_overlap_v2,
    clear_zero_qb_coalesce,
    cs_intra_barrier_v2,
    sm66_handle_kind_exact,
    ps_to_cs_overlap_v2,
    gfx_overlap_capped,
    cs_fast_bind_exact,
    sm66_handle_kind_v2,
    ps_to_cs_hk_overlap,
    gfx_overlap_frag_cap,
    cs_bind_intra_safe,
);

pub type Counter = AtomicU64;

/// Fast relaxed gate controlling whether counter increments and stat-only
/// diagnostic re-evaluations run.
///
/// Starts `true` under `#[cfg(test)]` so all unit tests observe exact counter
/// increments; in non-test builds it is lazily initialized from `AW2_ENC_STATS`
/// (`!= "0"`) so production runs avoid cross-core atomic RMW false sharing when
/// encoder statistics are disabled.
#[cfg(test)]
pub static ENABLED: AtomicBool = AtomicBool::new(true);
#[cfg(not(test))]
pub static ENABLED: AtomicBool = AtomicBool::new(false);

#[cfg(not(test))]
static INIT_STATE: AtomicU8 = AtomicU8::new(0);

#[cfg(not(test))]
#[cold]
#[inline(never)]
fn init_enabled() -> bool {
    let on = std::env::var_os("AW2_ENC_STATS").is_some_and(|v| v != "0");
    ENABLED.store(on, Relaxed);
    INIT_STATE.store(if on { 2 } else { 1 }, Relaxed);
    on
}

/// Fast-path check: true when `AW2_ENC_STATS` is enabled (or during unit tests).
#[inline(always)]
pub fn active() -> bool {
    #[cfg(test)]
    {
        ENABLED.load(Relaxed)
    }
    #[cfg(not(test))]
    {
        match INIT_STATE.load(Relaxed) {
            1 => false,
            2 => true,
            _ => init_enabled(),
        }
    }
}

#[inline(always)]
pub fn inc(c: &AtomicU64) {
    if active() {
        c.fetch_add(1, Relaxed);
    }
}

#[inline(always)]
pub fn add(c: &AtomicU64, n: u64) {
    if active() {
        c.fetch_add(n, Relaxed);
    }
}

pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let on = std::env::var_os("AW2_ENC_STATS").is_some_and(|v| v != "0");
        #[cfg(not(test))]
        {
            ENABLED.store(on, Relaxed);
            INIT_STATE.store(if on { 2 } else { 1 }, Relaxed);
        }
        on
    })
}

/// Log per-frame averages over the last `frames` frames and reset.
pub fn dump(frames: u64) {
    let frames = frames.max(1);
    let mut s = String::new();
    for (n, c) in NAMES.iter().zip(all()) {
        let v = c.swap(0, Relaxed);
        s.push_str(&format!(" {n}={:.1}", v as f64 / frames as f64));
    }
    log::info!(target: "d3d", "enc/frame:{s}");
}

#[cfg(test)]
pub(crate) fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    static L: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let g = L.lock().unwrap_or_else(|e| e.into_inner());
    ENABLED.store(true, Relaxed);
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc_and_add_are_gated_by_enabled_flag() {
        let _lock = test_lock();
        struct Restore;
        impl Drop for Restore {
            fn drop(&mut self) {
                ENABLED.store(true, Relaxed);
            }
        }
        let _restore = Restore;

        let local = AtomicU64::new(0);
        assert!(active());
        inc(&local);
        add(&local, 5);
        assert_eq!(local.load(Relaxed), 6);

        ENABLED.store(false, Relaxed);
        assert!(!active());
        inc(&local);
        add(&local, 10);
        assert_eq!(local.load(Relaxed), 6, "counters must not mutate when ENABLED is false");

        ENABLED.store(true, Relaxed);
        inc(&local);
        assert_eq!(local.load(Relaxed), 7);
    }
}
