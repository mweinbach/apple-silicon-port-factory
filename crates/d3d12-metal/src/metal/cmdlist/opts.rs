//! Command list feature gates, environment option readers, and [`CmdListOpts`].

use super::CommandList;
use crate::metal::encstats as stats;
use crate::metal::queue::ListType;
use objc2_metal::MTL4TimestampGranularity;

/// Read a default-ON boolean feature knob with a universal `PORT_*` primary env var
/// and optional legacy `AW2_*` alias (`=0` disables; unset or anything else enables).
#[inline]
fn env_flag_default_on(primary: &str, legacy: &str) -> bool {
    std::env::var_os(primary)
        .or_else(|| std::env::var_os(legacy))
        .is_none_or(|v| v != "0")
}

/// Read a default-OFF boolean feature knob with a universal `PORT_*` primary env var
/// and optional legacy `AW2_*` alias (`=1` enables; unset or anything else disables).
#[inline]
fn env_flag_default_off(primary: &str, legacy: &str) -> bool {
    std::env::var_os(primary)
        .or_else(|| std::env::var_os(legacy))
        .is_some_and(|v| v == "1")
}

/// Define a `OnceLock<bool>` default-ON env reader (`=0` disables).
macro_rules! opt_default_on {
    ($vis:vis $name:ident, $env:literal $(, $doc:literal)?) => {
        $(#[doc = $doc])?
        $vis fn $name() -> bool {
            static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            *ON.get_or_init(|| std::env::var_os($env).is_none_or(|v| v != "0"))
        }
    };
}

/// Define a `OnceLock<bool>` default-OFF env reader (`=1` enables).
macro_rules! opt_default_off {
    ($vis:vis $name:ident, $env:literal $(, $doc:literal)?) => {
        $(#[doc = $doc])?
        $vis fn $name() -> bool {
            static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            *ON.get_or_init(|| std::env::var_os($env).is_some_and(|v| v == "1"))
        }
    };
}

/// Define a `OnceLock<bool>` default-OFF env reader (any value other than `"0"` enables).
macro_rules! opt_default_off_nonzero {
    ($vis:vis $name:ident, $env:literal $(, $doc:literal)?) => {
        $(#[doc = $doc])?
        $vis fn $name() -> bool {
            static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            *ON.get_or_init(|| std::env::var_os($env).is_some_and(|v| v != "0"))
        }
    };
}

/// Define a `OnceLock<bool>` default-ON dual-env (`PORT_*` + legacy `AW2_*`) reader.
macro_rules! opt_dual_default_on {
    ($vis:vis $name:ident, $primary:literal, $legacy:literal $(, $doc:literal)?) => {
        $(#[doc = $doc])?
        $vis fn $name() -> bool {
            static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            *ON.get_or_init(|| env_flag_default_on($primary, $legacy))
        }
    };
}

/// Define a `OnceLock<bool>` default-OFF dual-env (`PORT_*` + legacy `AW2_*`) reader.
macro_rules! opt_dual_default_off {
    ($vis:vis $name:ident, $primary:literal, $legacy:literal $(, $doc:literal)?) => {
        $(#[doc = $doc])?
        $vis fn $name() -> bool {
            static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            *ON.get_or_init(|| env_flag_default_off($primary, $legacy))
        }
    };
}

/// Reuse `IcbRing` and GPU-written `icb_scratch` across closed render passes (rewound
/// strictly after `endEncoding()`, synchronized by a cross-encoder
/// `barrierAfterQueueStages(All, Vertex|Dispatch)`). Default ON on iOS (`PORT_REUSE_ICB=0`
/// / `PORT_IOS_REUSE_ICB=0` disables; `PORT_REUSE_ICB=1` enables on macOS).
pub(super) fn icb_reuse_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var_os("PORT_REUSE_ICB")
            .or_else(|| std::env::var_os("PORT_IOS_REUSE_ICB"))
            .map(|v| v != "0")
            .unwrap_or(cfg!(target_os = "ios"))
    })
}

/// Maximum indirect commands accumulated inside a single open Metal render pass on iOS
/// (`PORT_IOS_ICB_PASS_CAP`, default `65536`) before splitting via `end_encoder()`.
pub(super) fn ios_icb_pass_cap() -> u32 {
    static CAP: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *CAP.get_or_init(|| {
        std::env::var("PORT_IOS_ICB_PASS_CAP")
            .ok()
            .and_then(|v| v.trim().parse::<u32>().ok())
            .unwrap_or(65536)
    })
}

pub(super) static IOS_ICB_PASS_SPLITS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

/// Return and reset the count of iOS ICB render-pass splits since the previous call.
pub fn take_ios_icb_pass_splits() -> u64 {
    IOS_ICB_PASS_SPLITS.swap(0, std::sync::atomic::Ordering::Relaxed)
}

/// GPU timestamp granularity: relaxed unless `AW2_PRECISE_TIMESTAMPS=1`
/// (precise timestamps can split encoders on Apple GPUs).
pub(super) fn timestamp_granularity() -> MTL4TimestampGranularity {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *ON.get_or_init(|| std::env::var_os("AW2_PRECISE_TIMESTAMPS").is_some_and(|v| v != "0")) {
        MTL4TimestampGranularity::Precise
    } else {
        MTL4TimestampGranularity::Relaxed
    }
}

/// Default ON on iOS (`PORT_ICB_PREPASS_COMPUTE=0/1` / `AW2_ICB_PREPASS_COMPUTE=0/1`
/// overrides): expand batched indirect ICB draws in a compute pre-pass before opening
/// the render pass when no draw pass is open yet.
pub(crate) fn opt_icb_prepass_compute_env() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var_os("PORT_ICB_PREPASS_COMPUTE")
            .or_else(|| std::env::var_os("AW2_ICB_PREPASS_COMPUTE"))
            .map(|v| v != "0")
            .unwrap_or(cfg!(target_os = "ios"))
    })
}

/// Default ON (`AW2_FAST_BIND_PSO=0` disables): dirty-track `MTL4ArgumentTable`
/// buffer/VB bindings and specialize internal utility MSL compute PSOs.
pub(crate) fn opt_fast_bind_pso_env() -> bool {
    crate::metal::pipeline::fast_bind_pso()
}

// Core barrier & render-pass mode env readers (internal to cmdlist).
opt_default_on!(pub(super) barrier_masks, "AW2_BARRIER_MASKS");
opt_default_off_nonzero!(pub(super) legacy_barriers, "AW2_LEGACY_BARRIERS");
opt_default_off_nonzero!(pub(super) rp_discard, "AW2_RP_DISCARD");
opt_default_on!(pub(super) barrier_r2r_elide, "AW2_BARRIER_R2R_ELIDE");
opt_default_on!(pub(super) barrier_tight_stages, "AW2_BARRIER_TIGHT_STAGES");
opt_default_off_nonzero!(pub(super) barrier_uav0_keep, "AW2_BARRIER_UAV0_KEEP");

// Timeline optimization gates (crate-visible env readers, overridable per CommandList in tests).
opt_default_on!(pub(crate) opt_ib_tight_env, "AW2_IB_TIGHT");
opt_default_off_nonzero!(pub(crate) opt_ib_bindless_war_env, "AW2_IB_BINDLESS_WAR");
opt_default_on!(pub(crate) opt_rp_keep_write_env, "AW2_RP_KEEP_WRITE");
opt_default_on!(pub(crate) opt_clear_keep_smart_env, "AW2_CLEAR_KEEP_SMART");
opt_default_on!(pub(crate) opt_icb_batch_env, "AW2_ICB_BATCH");
opt_default_off!(pub(crate) opt_clear_keep_dsv_env, "AW2_CLEAR_KEEP_DSV");
opt_default_on!(pub(crate) opt_ib_exact_obl_env, "AW2_IB_EXACT_OBL");
opt_default_off!(pub(crate) opt_list_barrier_tight_env, "AW2_LIST_BARRIER_TIGHT");
opt_default_off!(pub(crate) opt_barrier_tight_v2_env, "AW2_BARRIER_TIGHT_V2");
opt_dual_default_on!(pub(crate) opt_empty_enc_elide_env, "PORT_EMPTY_ENC_ELIDE", "AW2_EMPTY_ENC_ELIDE");
opt_default_on!(pub(crate) opt_uav0_clear_overlap_env, "AW2_UAV0_CLEAR_OVERLAP");
opt_default_on!(pub(crate) opt_fast_enc_state_env, "AW2_FAST_ENC_STATE");
opt_default_on!(pub(crate) opt_clear_overlap_exact_env, "AW2_CLEAR_OVERLAP_EXACT");
opt_default_on!(pub(crate) opt_subrange_uav_barrier_env, "AW2_SUBRANGE_UAV_BARRIER");
opt_default_on!(pub(crate) opt_copy_queue_light_env, "AW2_COPY_QUEUE_LIGHT");
opt_default_off!(pub(crate) opt_stagesync_barrier_v2_env, "AW2_STAGESYNC_BARRIER_V2");
opt_default_on!(pub(crate) opt_clear_cs_exact_sync_env, "AW2_CLEAR_CS_EXACT_SYNC");
opt_default_on!(pub(crate) opt_render_split_stage_sync_env, "AW2_RENDER_SPLIT_STAGE_SYNC");
opt_default_on!(pub(crate) opt_mid_qb_stagesync_env, "AW2_MID_QB_STAGESYNC");
opt_dual_default_on!(pub(crate) opt_ro_dsv_dontcare_env, "PORT_RO_DSV_DONTCARE", "AW2_RO_DSV_DONTCARE");
opt_default_on!(pub(crate) opt_split_res_soft_sync_env, "AW2_SPLIT_RES_SOFT_SYNC");
opt_default_on!(pub(crate) opt_cs_uav_producer_exact_env, "AW2_CS_UAV_PRODUCER_EXACT");
opt_default_on!(pub(crate) opt_split_compute_exact_env, "AW2_SPLIT_COMPUTE_EXACT");
opt_default_on!(pub(crate) opt_render_dom_exact_env, "AW2_RENDER_DOM_EXACT");
opt_default_on!(pub(crate) opt_vs_buf_defer_exact_env, "AW2_VS_BUF_DEFER_EXACT");
opt_default_off!(pub(crate) opt_cs_mid_qb_touch_defer_env, "AW2_CS_MID_QB_TOUCH_DEFER");
opt_default_on!(pub(crate) opt_ei_dispatch_exact_env, "AW2_EI_DISPATCH_EXACT");
opt_default_off!(pub(crate) opt_blit_list_narrow_v2_env, "AW2_BLIT_LIST_NARROW_V2");
opt_default_on!(pub(crate) opt_list_start_substage_env, "AW2_LIST_START_SUBSTAGE");
opt_default_on!(pub(crate) opt_ib_substage_narrow_env, "AW2_IB_SUBSTAGE_NARROW");
opt_default_off!(pub(crate) opt_list_start_render_split_env, "AW2_LIST_START_RENDER_SPLIT");
opt_default_off!(pub(crate) opt_cs_mid_qb_defer_v3_env, "AW2_CS_MID_QB_DEFER_V3");
opt_default_on!(pub(crate) opt_rp_keep_write_exact_env, "AW2_RP_KEEP_WRITE_EXACT");
opt_default_off!(pub(crate) opt_ib_uav0_unbounded_exact_env, "AW2_IB_UAV0_UNBOUNDED_EXACT");
opt_default_off!(pub(crate) opt_clear_prelist_overlap_env, "AW2_CLEAR_PRELIST_OVERLAP");
opt_default_off!(pub(crate) opt_cs_mid_qb_unpoison_v4_env, "AW2_CS_MID_QB_UNPOISON_V4");
opt_default_on!(pub(crate) opt_ps_srv_class_exact_env, "AW2_PS_SRV_CLASS_EXACT");
opt_default_off!(pub(crate) opt_desc_table_res_exact_env, "AW2_DESC_TABLE_RES_EXACT");
opt_default_on!(pub(crate) opt_gfx_desc_table_exact_env, "AW2_GFX_DESC_TABLE_EXACT");
opt_default_off!(pub(crate) opt_cs_mid_qb_res_defer_v5_env, "AW2_CS_MID_QB_RES_DEFER_V5");
opt_dual_default_on!(pub(crate) opt_icb_enc_barrier_exact_env, "PORT_ICB_ENC_BARRIER_EXACT", "AW2_ICB_ENC_BARRIER_EXACT");
opt_default_off!(pub(crate) opt_desc_table_cs_fast_env, "AW2_DESC_TABLE_CS_FAST");
opt_dual_default_on!(pub(crate) opt_rp_split_exact_keep_env, "PORT_RP_SPLIT_EXACT_KEEP", "AW2_RP_SPLIT_EXACT_KEEP");
opt_dual_default_on!(pub(crate) opt_clear_cs_tile4_env, "PORT_CLEAR_CS_TILE4", "AW2_CLEAR_CS_TILE4");
opt_default_off!(pub(crate) opt_list_boundary_overlap_env, "AW2_LIST_BOUNDARY_OVERLAP");
opt_default_off!(pub(crate) opt_cs_intra_barrier_exact_env, "AW2_CS_INTRA_BARRIER_EXACT");
opt_default_off!(pub(crate) opt_clear_pass_fold_v2_env, "AW2_CLEAR_PASS_FOLD_V2");
opt_default_off!(pub(crate) opt_cs_to_ps_overlap_env, "AW2_CS_TO_PS_OVERLAP");
opt_default_on!(pub(crate) opt_cs_to_ps_overlap_v2_env, "AW2_CS_TO_PS_OVERLAP_V2");
opt_default_off!(pub(crate) opt_gfx_to_ps_overlap_env, "AW2_GFX_TO_PS_OVERLAP");
opt_default_off!(pub(crate) opt_clear_isolate_coalesce_env, "AW2_CLEAR_ISOLATE_COALESCE");
opt_default_off!(pub(crate) opt_queue_start_stage_exact_env, "AW2_QUEUE_START_STAGE_EXACT");
opt_default_off!(pub(crate) opt_ps_to_cs_overlap_env, "AW2_PS_TO_CS_OVERLAP");
opt_default_off!(pub(crate) opt_gfx_to_ps_overlap_v2_env, "AW2_GFX_TO_PS_OVERLAP_V2");
opt_default_off!(pub(crate) opt_clear_zero_qb_coalesce_env, "AW2_CLEAR_ZERO_QB_COALESCE");
opt_default_off!(pub(crate) opt_cs_intra_barrier_v2_env, "AW2_CS_INTRA_BARRIER_V2");
opt_default_off!(pub(crate) opt_sm66_handle_kind_exact_env, "AW2_SM66_HANDLE_KIND_EXACT");
opt_default_off!(pub(crate) opt_ps_to_cs_overlap_v2_env, "AW2_PS_TO_CS_OVERLAP_V2");
opt_default_off!(pub(crate) opt_gfx_overlap_capped_env, "AW2_GFX_OVERLAP_CAPPED");
opt_default_off!(pub(crate) opt_cs_fast_bind_exact_env, "AW2_CS_FAST_BIND_EXACT");
opt_default_off!(pub(crate) opt_sm66_handle_kind_v2_env, "AW2_SM66_HANDLE_KIND_V2");
opt_default_off!(pub(crate) opt_ps_to_cs_hk_overlap_env, "AW2_PS_TO_CS_HK_OVERLAP");
opt_default_off!(pub(crate) opt_gfx_overlap_frag_cap_env, "AW2_GFX_OVERLAP_FRAG_CAP");
opt_dual_default_on!(pub(crate) opt_cs_bind_cache_env, "PORT_CS_BIND_CACHE", "AW2_CS_BIND_CACHE");
opt_dual_default_off!(pub(crate) opt_cs_bind_intra_safe_env, "PORT_CS_BIND_INTRA_SAFE", "AW2_CS_BIND_INTRA_SAFE");

#[inline]
pub fn copy_queue_light() -> bool {
    opt_copy_queue_light_env()
}

/// Per-command-list timeline optimization toggles (initialized from env vars,
/// overridable per instance in unit/GPU tests).
pub(crate) struct CmdListOpts {
    pub(crate) opt_ib_tight: bool,
    pub(crate) opt_ib_bindless_war: bool,
    pub(crate) opt_rp_keep_write: bool,
    pub(crate) opt_clear_keep_smart: bool,
    pub(crate) opt_icb_batch: bool,
    pub(crate) opt_clear_keep_dsv: bool,
    pub(crate) opt_ib_exact_obl: bool,
    pub(crate) opt_list_barrier_tight: bool,
    pub(crate) opt_barrier_tight_v2: bool,
    pub(crate) opt_fast_bind_pso: bool,
    pub(crate) opt_empty_enc_elide: bool,
    pub(crate) opt_uav0_clear_overlap: bool,
    pub(crate) opt_fast_enc_state: bool,
    pub(crate) opt_clear_overlap_exact: bool,
    pub(crate) opt_subrange_uav_barrier: bool,
    pub(crate) opt_copy_queue_light: bool,
    pub(crate) opt_stagesync_barrier_v2: bool,
    pub(crate) opt_msc_compute_access: bool,
    pub(crate) opt_clear_cs_exact_sync: bool,
    pub(crate) opt_icb_fast_v3: bool,
    pub(crate) opt_render_split_stage_sync: bool,
    pub(crate) opt_mid_qb_stagesync: bool,
    pub(crate) opt_ro_dsv_dontcare: bool,
    pub(crate) opt_split_res_soft_sync: bool,
    pub(crate) opt_cs_uav_producer_exact: bool,
    pub(crate) opt_split_compute_exact: bool,
    pub(crate) opt_msc_stage_usage: bool,
    pub(crate) opt_render_dom_exact: bool,
    pub(crate) opt_air_no_res_heap: bool,
    pub(crate) opt_vs_buf_defer_exact: bool,
    pub(crate) opt_cs_mid_qb_touch_defer: bool,
    pub(crate) opt_ei_dispatch_exact: bool,
    pub(crate) opt_blit_list_narrow_v2: bool,
    pub(crate) opt_list_start_substage: bool,
    pub(crate) opt_sm66_gfx_stage_exact: bool,
    pub(crate) opt_cs_mid_qb_exact_v2: bool,
    pub(crate) opt_ib_substage_narrow: bool,
    pub(crate) opt_sm66_cs_stage_exact: bool,
    pub(crate) opt_list_start_render_split: bool,
    pub(crate) opt_cs_mid_qb_defer_v3: bool,
    pub(crate) opt_rp_keep_write_exact: bool,
    pub(crate) opt_ib_uav0_unbounded_exact: bool,
    pub(crate) opt_clear_prelist_overlap: bool,
    pub(crate) opt_cs_mid_qb_unpoison_v4: bool,
    pub(crate) opt_ps_srv_class_exact: bool,
    pub(crate) opt_desc_table_res_exact: bool,
    pub(crate) opt_gfx_desc_table_exact: bool,
    pub(crate) opt_cs_mid_qb_res_defer_v5: bool,
    pub(crate) opt_icb_enc_barrier_exact: bool,
    pub(crate) opt_icb_prepass_compute: bool,
    pub(crate) opt_desc_table_cs_fast: bool,
    pub(crate) opt_rp_split_exact_keep: bool,
    pub(crate) opt_clear_cs_tile4: bool,
    pub(crate) opt_list_boundary_overlap: bool,
    pub(crate) opt_cs_intra_barrier_exact: bool,
    pub(crate) opt_clear_pass_fold_v2: bool,
    pub(crate) opt_cs_to_ps_overlap: bool,
    pub(crate) opt_cs_to_ps_overlap_v2: bool,
    pub(crate) opt_gfx_to_ps_overlap: bool,
    pub(crate) opt_clear_isolate_coalesce: bool,
    pub(crate) opt_queue_start_stage_exact: bool,
    pub(crate) opt_ps_to_cs_overlap: bool,
    pub(crate) opt_gfx_to_ps_overlap_v2: bool,
    pub(crate) opt_clear_zero_qb_coalesce: bool,
    pub(crate) opt_cs_intra_barrier_v2: bool,
    pub(crate) opt_sm66_handle_kind_exact: bool,
    pub(crate) opt_ps_to_cs_overlap_v2: bool,
    pub(crate) opt_gfx_overlap_capped: bool,
    pub(crate) opt_cs_fast_bind_exact: bool,
    pub(crate) opt_sm66_handle_kind_v2: bool,
    pub(crate) opt_ps_to_cs_hk_overlap: bool,
    pub(crate) opt_gfx_overlap_frag_cap: bool,
    pub(crate) opt_cs_bind_cache: bool,
    pub(crate) opt_cs_bind_intra_safe: bool,
    #[cfg(test)]
    pub(crate) test_force_frag_buf_write: Option<bool>,
}

impl CmdListOpts {
    pub(super) fn from_env() -> Self {
        use crate::metal::{indirect_icb, pipeline};
        Self {
            opt_ib_tight: opt_ib_tight_env(),
            opt_ib_bindless_war: opt_ib_bindless_war_env(),
            opt_rp_keep_write: opt_rp_keep_write_env(),
            opt_clear_keep_smart: opt_clear_keep_smart_env(),
            opt_icb_batch: opt_icb_batch_env(),
            opt_clear_keep_dsv: opt_clear_keep_dsv_env(),
            opt_ib_exact_obl: opt_ib_exact_obl_env(),
            opt_list_barrier_tight: opt_list_barrier_tight_env(),
            opt_barrier_tight_v2: opt_barrier_tight_v2_env(),
            opt_fast_bind_pso: opt_fast_bind_pso_env(),
            opt_empty_enc_elide: opt_empty_enc_elide_env(),
            opt_uav0_clear_overlap: opt_uav0_clear_overlap_env(),
            opt_fast_enc_state: opt_fast_enc_state_env(),
            opt_clear_overlap_exact: opt_clear_overlap_exact_env(),
            opt_subrange_uav_barrier: opt_subrange_uav_barrier_env(),
            opt_copy_queue_light: opt_copy_queue_light_env(),
            opt_stagesync_barrier_v2: opt_stagesync_barrier_v2_env(),
            opt_msc_compute_access: pipeline::msc_compute_access(),
            opt_clear_cs_exact_sync: opt_clear_cs_exact_sync_env(),
            opt_icb_fast_v3: indirect_icb::fast_v3(),
            opt_render_split_stage_sync: opt_render_split_stage_sync_env(),
            opt_mid_qb_stagesync: opt_mid_qb_stagesync_env(),
            opt_ro_dsv_dontcare: opt_ro_dsv_dontcare_env(),
            opt_split_res_soft_sync: opt_split_res_soft_sync_env(),
            opt_cs_uav_producer_exact: opt_cs_uav_producer_exact_env(),
            opt_split_compute_exact: opt_split_compute_exact_env(),
            opt_msc_stage_usage: pipeline::msc_stage_usage(),
            opt_render_dom_exact: opt_render_dom_exact_env(),
            opt_air_no_res_heap: pipeline::air_no_res_heap(),
            opt_vs_buf_defer_exact: opt_vs_buf_defer_exact_env(),
            opt_cs_mid_qb_touch_defer: opt_cs_mid_qb_touch_defer_env(),
            opt_ei_dispatch_exact: opt_ei_dispatch_exact_env(),
            opt_blit_list_narrow_v2: opt_blit_list_narrow_v2_env(),
            opt_list_start_substage: opt_list_start_substage_env(),
            opt_sm66_gfx_stage_exact: pipeline::sm66_gfx_stage_exact(),
            opt_cs_mid_qb_exact_v2: pipeline::cs_mid_qb_exact_v2(),
            opt_ib_substage_narrow: opt_ib_substage_narrow_env(),
            opt_sm66_cs_stage_exact: pipeline::sm66_cs_stage_exact(),
            opt_list_start_render_split: opt_list_start_render_split_env(),
            opt_cs_mid_qb_defer_v3: opt_cs_mid_qb_defer_v3_env(),
            opt_rp_keep_write_exact: opt_rp_keep_write_exact_env(),
            opt_ib_uav0_unbounded_exact: opt_ib_uav0_unbounded_exact_env(),
            opt_clear_prelist_overlap: opt_clear_prelist_overlap_env(),
            opt_cs_mid_qb_unpoison_v4: opt_cs_mid_qb_unpoison_v4_env(),
            opt_ps_srv_class_exact: opt_ps_srv_class_exact_env(),
            opt_desc_table_res_exact: opt_desc_table_res_exact_env(),
            opt_gfx_desc_table_exact: opt_gfx_desc_table_exact_env(),
            opt_cs_mid_qb_res_defer_v5: opt_cs_mid_qb_res_defer_v5_env(),
            opt_icb_enc_barrier_exact: opt_icb_enc_barrier_exact_env(),
            opt_icb_prepass_compute: opt_icb_prepass_compute_env(),
            opt_desc_table_cs_fast: opt_desc_table_cs_fast_env(),
            opt_rp_split_exact_keep: opt_rp_split_exact_keep_env(),
            opt_clear_cs_tile4: opt_clear_cs_tile4_env(),
            opt_list_boundary_overlap: opt_list_boundary_overlap_env(),
            opt_cs_intra_barrier_exact: opt_cs_intra_barrier_exact_env(),
            opt_clear_pass_fold_v2: opt_clear_pass_fold_v2_env(),
            opt_cs_to_ps_overlap: opt_cs_to_ps_overlap_env(),
            opt_cs_to_ps_overlap_v2: opt_cs_to_ps_overlap_v2_env(),
            opt_gfx_to_ps_overlap: opt_gfx_to_ps_overlap_env(),
            opt_clear_isolate_coalesce: opt_clear_isolate_coalesce_env(),
            opt_queue_start_stage_exact: opt_queue_start_stage_exact_env(),
            opt_ps_to_cs_overlap: opt_ps_to_cs_overlap_env(),
            opt_gfx_to_ps_overlap_v2: opt_gfx_to_ps_overlap_v2_env(),
            opt_clear_zero_qb_coalesce: opt_clear_zero_qb_coalesce_env(),
            opt_cs_intra_barrier_v2: opt_cs_intra_barrier_v2_env(),
            opt_sm66_handle_kind_exact: opt_sm66_handle_kind_exact_env(),
            opt_ps_to_cs_overlap_v2: opt_ps_to_cs_overlap_v2_env(),
            opt_gfx_overlap_capped: opt_gfx_overlap_capped_env(),
            opt_cs_fast_bind_exact: opt_cs_fast_bind_exact_env(),
            opt_sm66_handle_kind_v2: opt_sm66_handle_kind_v2_env(),
            opt_ps_to_cs_hk_overlap: opt_ps_to_cs_hk_overlap_env(),
            opt_gfx_overlap_frag_cap: opt_gfx_overlap_frag_cap_env(),
            opt_cs_bind_cache: opt_cs_bind_cache_env(),
            opt_cs_bind_intra_safe: opt_cs_bind_intra_safe_env(),
            #[cfg(test)]
            test_force_frag_buf_write: None,
        }
    }
}

impl CommandList {
    #[inline]
    pub(super) fn cs_bind_hazards_active(&self) -> bool {
        // Dispatch history is accumulated across resources. Once placed allocations
        // overlap, distinct descriptor identities no longer prove disjoint storage.
        self.opts.opt_cs_bind_intra_safe && !crate::metal::resource::any_placed_alias()
    }

    #[inline]
    pub(super) fn hk_v2_active(&self) -> bool {
        self.opts.opt_sm66_handle_kind_v2
            || self.opts.opt_ps_to_cs_hk_overlap
            || self.cs_bind_hazards_active()
    }

    #[inline]
    pub(super) fn hk_any_active(&self) -> bool {
        self.opts.opt_sm66_handle_kind_exact || self.hk_v2_active()
    }

    #[inline]
    pub(super) fn bump_hk_stat(&self) {
        if self.opts.opt_ps_to_cs_hk_overlap {
            stats::inc(&stats::c::ps_to_cs_hk_overlap);
        } else if self.opts.opt_sm66_handle_kind_v2 {
            stats::inc(&stats::c::sm66_handle_kind_v2);
        } else if self.opts.opt_cs_bind_intra_safe {
            stats::inc(&stats::c::cs_bind_intra_safe);
        } else {
            stats::inc(&stats::c::sm66_handle_kind_exact);
        }
    }

    #[inline]
    pub(super) fn cs_intra_safe_active(&self) -> bool {
        self.opts.opt_cs_fast_bind_exact || self.cs_bind_hazards_active()
    }

    #[inline]
    pub(super) fn bump_cs_intra_stat(&self) {
        if self.opts.opt_cs_bind_intra_safe {
            stats::inc(&stats::c::cs_bind_intra_safe);
        } else {
            stats::inc(&stats::c::cs_fast_bind_exact);
        }
    }

    #[inline]
    pub(super) fn ps_to_cs_v2_active(&self) -> bool {
        self.opts.opt_ps_to_cs_overlap_v2 || self.opts.opt_ps_to_cs_hk_overlap
    }

    #[inline]
    pub(super) fn ps_to_cs_active(&self) -> bool {
        self.opts.opt_ps_to_cs_overlap || self.ps_to_cs_v2_active()
    }

    #[inline]
    pub(super) fn list_rsplit_v2_active(&self) -> bool {
        self.opts.opt_gfx_to_ps_overlap_v2
            || self.opts.opt_gfx_overlap_capped
            || self.opts.opt_gfx_overlap_frag_cap
    }

    #[inline]
    pub(super) fn gfx_to_ps_overlap_active(&self) -> bool {
        self.opts.opt_gfx_to_ps_overlap
            || self.opts.opt_gfx_to_ps_overlap_v2
            || (self.opts.opt_gfx_overlap_capped && self.rp_sync.gfx_ps_overlap_chain < 2)
            || (self.opts.opt_gfx_overlap_frag_cap && self.rp_sync.gfx_ps_overlap_chain < 1)
    }

    #[inline]
    pub(super) fn clear_isolate_active(&self) -> bool {
        self.opts.opt_clear_isolate_coalesce || self.opts.opt_clear_zero_qb_coalesce
    }

    #[inline]
    pub(super) fn list_barrier_tight_active(&self) -> bool {
        self.opts.opt_list_barrier_tight && !legacy_barriers() && barrier_masks()
    }

    #[inline]
    pub(super) fn is_copy_queue_light(&self) -> bool {
        self.opts.opt_copy_queue_light
            && self.kind == ListType::Copy
            && !legacy_barriers()
            && barrier_masks()
    }

    #[inline]
    pub(super) fn render_split_sync_active(&self) -> bool {
        self.opts.opt_render_split_stage_sync
            && !legacy_barriers()
            && barrier_masks()
            && !self.is_copy_queue_light()
    }

    #[inline]
    pub(super) fn mid_qb_stagesync_active(&self) -> bool {
        (self.opts.opt_stagesync_barrier_v2 || self.opts.opt_mid_qb_stagesync)
            && !legacy_barriers()
            && barrier_masks()
    }

    #[inline]
    pub(super) fn msc_class_active(&self) -> bool {
        self.opts.opt_msc_compute_access
            || self.opts.opt_msc_stage_usage
            || self.opts.opt_air_no_res_heap
            || self.opts.opt_cs_mid_qb_exact_v2
            || self.opts.opt_sm66_cs_stage_exact
            || self.hk_any_active()
    }

    #[inline]
    pub(super) fn has_frag_buf_write(&self) -> bool {
        #[cfg(test)]
        if let Some(forced) = self.opts.test_force_frag_buf_write {
            return forced;
        }
        self.occlusion.is_some() || crate::metal::pipeline::any_frag_buf_write()
    }

    #[inline]
    pub(super) fn split_compute_exact_active(&self) -> bool {
        self.opts.opt_split_compute_exact && self.render_split_sync_active()
    }

    #[inline]
    pub(super) fn render_dom_exact_active(&self) -> bool {
        self.opts.opt_render_dom_exact
            && !legacy_barriers()
            && barrier_masks()
            && !self.is_copy_queue_light()
    }

    #[inline]
    pub(super) fn vs_buf_defer_exact_active(&self) -> bool {
        self.opts.opt_vs_buf_defer_exact && self.render_split_sync_active()
    }

    #[inline]
    pub(super) fn cs_mid_qb_touch_defer_active(&self) -> bool {
        self.opts.opt_cs_mid_qb_touch_defer
            && self.render_split_sync_active()
            && self.ib_tight_active()
    }

    #[inline]
    pub(super) fn cs_mid_qb_exact_v2_active(&self) -> bool {
        self.opts.opt_cs_mid_qb_exact_v2
            && self.render_split_sync_active()
            && self.ib_tight_active()
    }

    #[inline]
    pub(super) fn ib_tight_active(&self) -> bool {
        self.opts.opt_ib_tight && !legacy_barriers() && barrier_masks()
    }
}
