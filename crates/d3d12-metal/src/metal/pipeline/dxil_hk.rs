//! Zero-salt DXIL handle-kind (`!dx.resources` + `@dx.op.annotateHandle`)
//! and MSC `ResourceUsages` / `UsedResources` handle-kind profile analysis.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use dxil_metal::dxbc::PsvResource;
use dxil_metal::root_signature::{DescriptorRangeType, RootParameterLayoutKind, RootSignature};
use dxil_metal::CompiledShader;

use crate::metal::cmdlist::{
    opt_cs_bind_intra_safe_env, opt_ps_to_cs_hk_overlap_env, opt_sm66_handle_kind_exact_env,
    opt_sm66_handle_kind_v2_env,
};
use crate::metal::resource::{
    HK_ACCEL_STRUCT, HK_ALL, HK_ALL_BUF, HK_ALL_SRV_UAV_BUF, HK_ALL_TEX, HK_CBV_BUF,
    HK_FEEDBACK_TEX, HK_RAW_BUF, HK_STRUCT_BUF, HK_TEX1D_FLT, HK_TEX1D_INT, HK_TEX2D_FLT,
    HK_TEX2D_INT, HK_TEX2DMS_FLT, HK_TEX2DMS_INT, HK_TEX3D_FLT, HK_TEX3D_INT, HK_TEXCUBE_FLT,
    HK_TEXCUBE_INT, HK_TYPED_BUF,
};

use super::reflection::{
    metallib_res_desc_heap_unused, msc_stage_may_read_cbv_from_reflection, msc_used_resources,
    stage_may_read_cbv_buffer_with, ComputeAccess, MscUsedResource, ShaderStageSummary, READS_BUF,
    READS_TEX,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandleKindProfile {
    pub uav_write_kinds: u32,
    pub uav_srv_kinds: u32,
    pub ro_srv_kinds: u32,
    pub read_kinds: u32,
    /// Exact `HK_*` mask of SRVs that may be read through unbounded (> 128) root-signature
    /// SRV descriptor tables (`0` when all SRV reads are in `bounded_read_slots`).
    pub unbounded_srv_kinds: u32,
}

impl Default for HandleKindProfile {
    fn default() -> Self {
        Self::OPAQUE
    }
}

impl HandleKindProfile {
    pub const OPAQUE: Self = Self {
        uav_write_kinds: HK_ALL & !HK_CBV_BUF,
        uav_srv_kinds: HK_ALL & !HK_CBV_BUF,
        ro_srv_kinds: HK_ALL & !HK_CBV_BUF,
        read_kinds: HK_ALL,
        unbounded_srv_kinds: HK_ALL & !HK_CBV_BUF,
    };

    /// Intersects `self` with coarse `ComputeAccess` class masks (`READS_TEX`, `READS_BUF`,
    /// `msc_exact_classes`, `unbounded_srv_classes`) so both real shaders and synthetic
    /// unit-test `ComputeAccess` structs stay 100% consistent.
    #[inline]
    pub fn intersect_access(mut self, acc: &ComputeAccess) -> Self {
        let uav_allow = (if (acc.uav_classes & READS_TEX) != 0 { HK_ALL_TEX } else { 0 })
            | (if (acc.uav_classes & READS_BUF) != 0 { HK_ALL_SRV_UAV_BUF } else { 0 });
        let us_allow = (if (acc.uav_srv_classes & READS_TEX) != 0 { HK_ALL_TEX } else { 0 })
            | (if (acc.uav_srv_classes & READS_BUF) != 0 { HK_ALL_SRV_UAV_BUF } else { 0 });
        let rd_allow = (if (acc.read_classes & READS_TEX) != 0 { HK_ALL_TEX } else { 0 })
            | (if (acc.read_classes & READS_BUF) != 0 { HK_ALL_BUF } else { 0 });
        let ro_allow = (if (acc.msc_exact_classes & READS_TEX) != 0 { HK_ALL_TEX } else { 0 })
            | (if (acc.msc_exact_classes & READS_BUF) != 0 { HK_ALL_BUF } else { 0 });
        let unb_allow = (if (acc.unbounded_srv_classes & READS_TEX) != 0 { HK_ALL_TEX } else { 0 })
            | (if (acc.unbounded_srv_classes & READS_BUF) != 0 { HK_ALL_SRV_UAV_BUF } else { 0 });
        self.uav_write_kinds &= uav_allow;
        self.uav_srv_kinds &= us_allow;
        self.read_kinds &= rd_allow;
        self.ro_srv_kinds &= ro_allow & !HK_CBV_BUF;
        if !acc.touches_textures {
            self.uav_write_kinds &= !HK_ALL_TEX;
            self.uav_srv_kinds &= !HK_ALL_TEX;
            self.ro_srv_kinds &= !HK_ALL_TEX;
            self.read_kinds &= !HK_ALL_TEX;
            self.unbounded_srv_kinds &= !HK_ALL_TEX;
        }
        if acc.read_slots.is_some() {
            self.unbounded_srv_kinds = 0;
        } else {
            self.unbounded_srv_kinds &= unb_allow & self.ro_srv_kinds;
        }
        self
    }

    #[inline]
    pub fn ro_read_kinds(&self, acc: &ComputeAccess) -> u32 {
        let ro_allow = (if (acc.msc_exact_classes & READS_TEX) != 0 { HK_ALL_TEX } else { 0 })
            | (if (acc.msc_exact_classes & READS_BUF) != 0 { HK_ALL_BUF } else { 0 });
        (self.ro_srv_kinds | (self.read_kinds & HK_CBV_BUF)) & ro_allow
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DxilSrvRangeHk {
    pub space: u32,
    pub lower_bound: u32,
    pub upper_bound: u32,
    pub hk: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DxilHkDetail {
    pub srv_mask: u32,
    pub uav_mask: u32,
    pub srv_ranges: Vec<DxilSrvRangeHk>,
}

fn hk_profile_registry() -> &'static RwLock<HashMap<u128, HandleKindProfile>> {
    static REG: OnceLock<RwLock<HashMap<u128, HandleKindProfile>>> = OnceLock::new();
    REG.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn shader_handle_kind_profile(shader: &CompiledShader, acc: &ComputeAccess) -> HandleKindProfile {
    let base = if shader.key != 0 {
        hk_profile_registry()
            .read()
            .unwrap()
            .get(&shader.key)
            .copied()
            .unwrap_or(HandleKindProfile::OPAQUE)
    } else {
        HandleKindProfile::OPAQUE
    };
    base.intersect_access(acc)
}

/// Returns the PSO's embedded [`HandleKindProfile`] with zero locking in production;
/// under `#[cfg(test)]`, consults [`shader_handle_kind_profile`] so tests that mutate
/// `access`, construct synthetic PSO literals, or register test profiles via
/// [`register_shader_hk_profile_for_test`] after pipeline creation see exact results.
#[inline]
pub fn resolve_pso_hk_profile(
    embedded: HandleKindProfile,
    shader: &CompiledShader,
    acc: &ComputeAccess,
) -> HandleKindProfile {
    #[cfg(test)]
    {
        let _ = embedded;
        shader_handle_kind_profile(shader, acc)
    }
    #[cfg(not(test))]
    {
        let _ = (shader, acc);
        embedded
    }
}

#[cfg(test)]
pub fn register_shader_hk_profile_for_test(key: u128, prof: HandleKindProfile) {
    hk_profile_registry().write().unwrap().insert(key, prof);
}

fn hk_tracking_enabled() -> bool {
    opt_sm66_handle_kind_exact_env()
        || opt_sm66_handle_kind_v2_env()
        || opt_ps_to_cs_hk_overlap_env()
        || opt_cs_bind_intra_safe_env()
}

pub(super) fn resolve_shader_hk_profile_from_summary(
    summary: &ShaderStageSummary,
    bytecode: &[u8],
    rs: &RootSignature,
    shader: &CompiledShader,
) -> HandleKindProfile {
    if shader.key == 0 || !hk_tracking_enabled() {
        return HandleKindProfile::OPAQUE;
    }
    if let Some(&cached) = hk_profile_registry().read().unwrap().get(&shader.key) {
        return cached;
    }
    let raw = build_handle_kind_profile_raw_from_summary(summary, bytecode, rs);
    hk_profile_registry().write().unwrap().insert(shader.key, raw);
    raw
}

pub(super) fn register_shader_hk_profile(
    dxil: &[u8],
    rs: &RootSignature,
    shader: &CompiledShader,
    ru_opt: Option<u32>,
) {
    if shader.key == 0 || !hk_tracking_enabled() {
        return;
    }
    if hk_profile_registry().read().unwrap().contains_key(&shader.key) {
        return;
    }
    let raw = build_handle_kind_profile_raw(dxil, rs, shader, ru_opt);
    hk_profile_registry().write().unwrap().insert(shader.key, raw);
}

fn psv_single_srv_hk(r: &PsvResource) -> u32 {
    match r.res_type {
        3 => decode_dxil_shape_and_comp(r.kind, None).unwrap_or(HK_ALL) & !HK_CBV_BUF,
        // DXC emits SRVRaw for both ByteAddressBuffer and RTAccelerationStructure.
        // PSV0 records before ResKind was added cannot distinguish them.
        4 => match r.kind {
            11 => HK_RAW_BUF,
            16 => HK_ACCEL_STRUCT,
            _ => HK_RAW_BUF | HK_ACCEL_STRUCT,
        },
        5 => HK_STRUCT_BUF,
        _ => 0,
    }
}

#[allow(clippy::too_many_arguments)]
fn compute_unbounded_srv_hk_from_parts(
    psv_opt: Option<&[PsvResource]>,
    heap_indexed: bool,
    air_unused: bool,
    used_opt: Option<&[MscUsedResource]>,
    ru_opt: Option<u32>,
    rs: &RootSignature,
    detail_opt: Option<&DxilHkDetail>,
    ro_srv_kinds: u32,
) -> u32 {
    const MAX_BOUNDED_ARRAY: u32 = 128;

    let Some(detail) = detail_opt else {
        return (HK_ALL & !HK_CBV_BUF) & ro_srv_kinds;
    };

    // Direct ResourceDescriptorHeap accesses have no register range in !dx.resources
    // or PSV. Only the AIR proof can exclude them from the unbounded SRVs.
    if heap_indexed && !air_unused {
        return ro_srv_kinds;
    }

    let range_srv_hk = |space: u32, reg_lo: u32, reg_end: u32| -> u32 {
        let mut h = 0u32;
        let mut matched_dxil = false;
        for sr in &detail.srv_ranges {
            if sr.space != space {
                continue;
            }
            let sr_end = if sr.upper_bound == u32::MAX {
                u32::MAX
            } else {
                sr.upper_bound.saturating_add(1)
            };
            if sr_end <= reg_lo || sr.lower_bound >= reg_end {
                continue;
            }
            matched_dxil = true;
            h |= sr.hk;
        }
        if matched_dxil {
            return h;
        }
        if let Some(psv) = psv_opt {
            let mut ph = 0u32;
            for r in psv {
                if !(3..=5).contains(&r.res_type) || r.space != space {
                    continue;
                }
                let r_end = if r.upper_bound == u32::MAX {
                    u32::MAX
                } else {
                    r.upper_bound.saturating_add(1)
                };
                if r_end <= reg_lo || r.lower_bound >= reg_end {
                    continue;
                }
                ph |= psv_single_srv_hk(r) & (detail.srv_mask & !HK_CBV_BUF);
            }
            if ph != 0 {
                return ph;
            }
        }
        detail.srv_mask & !HK_CBV_BUF
    };

    if let Some(used) = used_opt {
        let layout = rs.layout();
        let param_len = layout.parameters.len();
        let has_static_samplers = layout.static_samplers.is_some();
        if used.iter().all(|u| {
            u.is_in_grs
                && ((u.binding_index as usize) < param_len
                    || (has_static_samplers && (u.binding_index as usize) == param_len))
                && u.binding_index <= u16::MAX as u32
        }) {
            let mut msc_valid = !used.is_empty() || ru_opt == Some(0);
            let mut msc_unb_hk = 0u32;
            'u_loop: for u in used {
                if (u.binding_index as usize) == param_len {
                    continue;
                }
                if let RootParameterLayoutKind::DescriptorTable { sampler: false, ranges } =
                    &layout.parameters[u.binding_index as usize].kind
                {
                    if u.table_length == 0 {
                        continue;
                    }
                    let bounded_end = (u.table_start_index != u32::MAX
                        && u.table_length <= MAX_BOUNDED_ARRAY)
                        .then(|| u.table_start_index.checked_add(u.table_length))
                        .flatten();
                    if let Some(end) = bounded_end {
                        for d in u.table_start_index..end {
                            if !ranges.iter().any(|rg| {
                                d >= rg.descriptor_offset
                                    && rg.num_descriptors.is_none_or(|n| d - rg.descriptor_offset < n)
                            }) {
                                msc_valid = false;
                                break 'u_loop;
                            }
                        }
                    } else {
                        let (start, end) = if u.table_start_index == u32::MAX {
                            (0u32, u32::MAX)
                        } else {
                            (
                                u.table_start_index,
                                u.table_start_index.saturating_add(u.table_length),
                            )
                        };
                        let mut any_overlap = false;
                        for rg in ranges {
                            let rg_end = rg.num_descriptors.map_or(u32::MAX, |n| {
                                rg.descriptor_offset.saturating_add(n)
                            });
                            if rg_end <= start || rg.descriptor_offset >= end {
                                continue;
                            }
                            any_overlap = true;
                            if rg.range_type == DescriptorRangeType::Srv {
                                let rg_reg_end = rg.num_descriptors.map_or(u32::MAX, |n| {
                                    rg.base_shader_register.saturating_add(n)
                                });
                                msc_unb_hk |= range_srv_hk(
                                    rg.register_space,
                                    rg.base_shader_register,
                                    rg_reg_end,
                                );
                            }
                        }
                        if !any_overlap {
                            msc_valid = false;
                            break 'u_loop;
                        }
                    }
                }
            }
            if msc_valid {
                return msc_unb_hk & ro_srv_kinds;
            }
        }
    }

    let mut unb_hk = 0u32;
    if !detail.srv_ranges.is_empty() {
        for sr in &detail.srv_ranges {
            let is_unb = sr.upper_bound == u32::MAX
                || sr.upper_bound < sr.lower_bound
                || (sr.upper_bound - sr.lower_bound + 1) > MAX_BOUNDED_ARRAY;
            if is_unb {
                unb_hk |= sr.hk;
            }
        }
    } else if let Some(psv) = psv_opt {
        for r in psv {
            if !(3..=5).contains(&r.res_type) {
                continue;
            }
            let is_unb = r.upper_bound == u32::MAX
                || r.upper_bound < r.lower_bound
                || (r.upper_bound - r.lower_bound + 1) > MAX_BOUNDED_ARRAY;
            if is_unb {
                unb_hk |= psv_single_srv_hk(r) & (detail.srv_mask & !HK_CBV_BUF);
            }
        }
    } else {
        unb_hk = detail.srv_mask & !HK_CBV_BUF;
    }
    unb_hk & ro_srv_kinds
}

fn assemble_handle_kind_profile(
    detail_opt: Option<&DxilHkDetail>,
    has_cbv: bool,
    ru_opt: Option<u32>,
    compute_unbounded: impl FnOnce(u32) -> u32,
) -> HandleKindProfile {
    const MSC_BUF_RW: u32 = HK_TYPED_BUF | HK_RAW_BUF | HK_STRUCT_BUF;
    let (dxil_srv, dxil_uav) =
        detail_opt.map_or((HK_ALL, HK_ALL & !HK_CBV_BUF), |d| (d.srv_mask, d.uav_mask));

    let (msc_w, msc_us, msc_ro) = match ru_opt {
        Some(ru) if (ru & !0x3e) == 0 => {
            let w = (if (ru & 16) != 0 { HK_ALL_TEX } else { 0 })
                | (if (ru & 4) != 0 { MSC_BUF_RW } else { 0 });
            let ro = (if (ru & 8) != 0 { HK_ALL_TEX } else { 0 })
                | (if (ru & 2) != 0 { MSC_BUF_RW } else { 0 })
                | (if (ru & 32) != 0 { HK_ACCEL_STRUCT } else { 0 });
            (w, w | ro, ro)
        }
        _ => (HK_ALL & !HK_CBV_BUF, HK_ALL & !HK_CBV_BUF, HK_ALL & !HK_CBV_BUF),
    };

    let uav_write_kinds = dxil_uav & msc_w;
    let uav_srv_kinds = ((dxil_srv & !HK_CBV_BUF) | dxil_uav) & msc_us;
    let ro_srv_kinds = (dxil_srv & !HK_CBV_BUF) & msc_ro;
    let cbv_bit = if has_cbv && (dxil_srv & HK_CBV_BUF) != 0 { HK_CBV_BUF } else { 0 };
    let read_kinds = uav_srv_kinds | cbv_bit;
    let unbounded_srv_kinds = compute_unbounded(ro_srv_kinds);
    HandleKindProfile {
        uav_write_kinds,
        uav_srv_kinds,
        ro_srv_kinds,
        read_kinds,
        unbounded_srv_kinds,
    }
}

fn build_handle_kind_profile_raw(
    dxil: &[u8],
    rs: &RootSignature,
    shader: &CompiledShader,
    ru_opt: Option<u32>,
) -> HandleKindProfile {
    const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
    let detail_opt = dxil_handle_kinds_detail_opt(dxil);
    let has_cbv = msc_stage_may_read_cbv_from_reflection(
        &shader.metallib,
        shader.reflection.json.as_str(),
        rs,
    )
    .unwrap_or_else(|| stage_may_read_cbv_buffer_with(shader, dxil, rs, false));

    assemble_handle_kind_profile(detail_opt.as_ref(), has_cbv, ru_opt, |ro_srv_kinds| {
        let converted;
        let container_opt = match dxil_metal::dxbc::Container::parse(dxil).ok() {
            Some(parsed) if parsed.is_sm4_bytecode() => match dxil_metal::dxilconv::convert_signed(dxil) {
                Ok(cv) => {
                    converted = cv;
                    dxil_metal::dxbc::Container::parse(&converted).ok()
                }
                Err(_) => None,
            },
            other => other,
        };
        let psv_opt = container_opt.as_ref().and_then(|c| c.psv_resources());
        let heap_indexed = container_opt
            .as_ref()
            .is_none_or(|c| (c.feature_flags() & RESOURCE_HEAP_INDEXING) != 0);
        let used_opt = msc_used_resources(shader.reflection.json.as_str());
        let air_unused = metallib_res_desc_heap_unused(&shader.metallib)
            && used_opt.as_deref().is_some_and(|u| !u.is_empty() || ru_opt == Some(0));
        compute_unbounded_srv_hk_from_parts(
            psv_opt.as_deref(),
            heap_indexed,
            air_unused,
            used_opt.as_deref(),
            ru_opt,
            rs,
            detail_opt.as_ref(),
            ro_srv_kinds,
        )
    })
}

fn build_handle_kind_profile_raw_from_summary(
    summary: &ShaderStageSummary,
    bytecode: &[u8],
    rs: &RootSignature,
) -> HandleKindProfile {
    const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
    let detail_opt = summary
        .dxil_bytes(bytecode)
        .and_then(|bytes| dxil_handle_kinds_detail_from_dxil_container(bytes, summary.dxil_psv()));
    let has_cbv = summary
        .msc_cbv_opt
        .unwrap_or_else(|| summary.stage_may_read_cbv(rs, false));

    assemble_handle_kind_profile(detail_opt.as_ref(), has_cbv, summary.ru_opt, |ro_srv_kinds| {
        let heap_indexed =
            !summary.dxil_parsed() || (summary.dxil_feature_flags() & RESOURCE_HEAP_INDEXING) != 0;
        let air_unused = summary.res_desc_heap_unused
            && summary
                .used_resources
                .as_deref()
                .is_some_and(|u| !u.is_empty() || summary.ru_opt == Some(0));
        compute_unbounded_srv_hk_from_parts(
            summary.dxil_psv(),
            heap_indexed,
            air_unused,
            summary.used_resources.as_deref(),
            summary.ru_opt,
            rs,
            detail_opt.as_ref(),
            ro_srv_kinds,
        )
    })
}

pub fn build_handle_kind_profile(
    dxil: &[u8],
    rs: &RootSignature,
    shader: &CompiledShader,
    ru_opt: Option<u32>,
    acc: &ComputeAccess,
) -> HandleKindProfile {
    build_handle_kind_profile_raw(dxil, rs, shader, ru_opt).intersect_access(acc)
}

fn decode_dxil_comp_flt_int(comp: u32) -> (bool, bool) {
    match comp {
        1..=7 | 17 | 18 => (false, true),
        8..=16 => (true, false),
        _ => (true, true),
    }
}

fn decode_dxil_shape_and_comp(kind: u32, comp_opt: Option<u32>) -> Option<u32> {
    let (has_flt, has_int) = comp_opt.map_or((true, true), decode_dxil_comp_flt_int);
    let pick = |flt: u32, int: u32| -> u32 {
        (if has_flt { flt } else { 0 }) | (if has_int { int } else { 0 })
    };
    Some(match kind {
        1 | 6 => pick(HK_TEX1D_FLT, HK_TEX1D_INT),
        2 | 7 => pick(HK_TEX2D_FLT, HK_TEX2D_INT),
        3 | 8 => pick(HK_TEX2DMS_FLT, HK_TEX2DMS_INT),
        4 => pick(HK_TEX3D_FLT, HK_TEX3D_INT),
        5 | 9 => pick(HK_TEXCUBE_FLT, HK_TEXCUBE_INT),
        10 => HK_TYPED_BUF,
        11 => HK_RAW_BUF,
        12 => HK_STRUCT_BUF,
        13 => HK_CBV_BUF,
        14 => 0, // Sampler
        15 => HK_TYPED_BUF | HK_RAW_BUF | HK_STRUCT_BUF | HK_CBV_BUF,
        16 => HK_ACCEL_STRUCT,
        17 | 18 => HK_FEEDBACK_TEX,
        _ => return None,
    })
}

fn decode_dxil_resource_properties(d0: u32, d1: u32) -> Option<(u32, u32)> {
    let kind = d0 & 0xff;
    let is_uav = (d0 & ((1 << 12) | (1 << 13))) != 0;
    let bits = decode_dxil_shape_and_comp(kind, Some(d1 & 0xff))?;
    if is_uav {
        Some((0, bits & !HK_CBV_BUF))
    } else {
        Some((bits, 0))
    }
}

pub(super) fn psv_resource_handle_kinds(psv: &[PsvResource]) -> (u32, u32) {
    let kind_bits = |k: u32| -> u32 { decode_dxil_shape_and_comp(k, None).unwrap_or(HK_ALL) };
    let mut srv = 0u32;
    let mut uav = 0u32;
    for r in psv {
        match r.res_type {
            1 => {}
            2 => srv |= HK_CBV_BUF,
            3..=5 => srv |= psv_single_srv_hk(r),
            6 => uav |= kind_bits(r.kind) & !HK_CBV_BUF,
            7 => uav |= HK_RAW_BUF,
            8 | 9 => uav |= HK_STRUCT_BUF,
            10 => uav |= HK_FEEDBACK_TEX,
            _ => {
                srv |= HK_ALL;
                uav |= HK_ALL & !HK_CBV_BUF;
            }
        }
    }
    (srv, uav)
}

/// Scans DXIL LLVM 3.7 bitcode for `!dx.resources` typed metadata (`METADATA_BLOCK = 15`, covering
/// SM 6.0..6.5 `@dx.op.createHandle` and `@dx.op.createHandleForLib.*` with exact `CompType`
/// `FLT` vs `INT`) and `@dx.op.annotateHandle` `%dx.types.ResourceProperties` constants
/// (`CST_CODE_AGGREGATE = 7` and `CST_CODE_DATA = 22` across module and function `CONSTANTS_BLOCK`s),
/// returning `(srv_or_cbv_kinds, uav_kinds)`. Fails closed to `(HK_ALL, HK_ALL & !HK_CBV_BUF)` on
/// any unparsed container, unknown handle-producing intrinsic, or non-constant `ResourceProperties`.
pub fn dxil_annotate_handle_kinds(dxil: &[u8]) -> (u32, u32) {
    const FAIL: (u32, u32) = (HK_ALL, HK_ALL & !HK_CBV_BUF);
    dxil_annotate_handle_kinds_opt(dxil).unwrap_or(FAIL)
}

pub fn dxil_annotate_handle_kinds_opt(dxil: &[u8]) -> Option<(u32, u32)> {
    dxil_handle_kinds_detail_opt(dxil).map(|d| (d.srv_mask, d.uav_mask))
}

/// Like `dxil_handle_kinds_detail_opt`, reusing an already-converted DXIL container
/// and pre-parsed PSV resources (avoids repeating `dxilconv::convert_signed`).
fn dxil_handle_kinds_detail_from_dxil_container(
    dxil_container: &[u8],
    psv_opt: Option<&[PsvResource]>,
) -> Option<DxilHkDetail> {
    let c = dxil_metal::dxbc::Container::parse(dxil_container).ok()?;
    let dxil_part = c.part(dxil_metal::dxbc::FOURCC_DXIL)?;
    dxil_handle_kinds_detail_from_part(dxil_part, psv_opt)
}

pub fn dxil_handle_kinds_detail_opt(dxil: &[u8]) -> Option<DxilHkDetail> {
    let parsed = dxil_metal::dxbc::Container::parse(dxil).ok()?;
    let converted;
    let c = if parsed.is_sm4_bytecode() {
        converted = dxil_metal::dxilconv::convert_signed(dxil).ok()?;
        dxil_metal::dxbc::Container::parse(&converted).ok()?
    } else {
        parsed
    };
    let dxil_part = c.part(dxil_metal::dxbc::FOURCC_DXIL)?;
    let psv_owned = c.psv_resources();
    dxil_handle_kinds_detail_from_part(dxil_part, psv_owned.as_deref())
}

fn dxil_handle_kinds_detail_from_part(
    dxil_part: &[u8],
    psv_opt: Option<&[PsvResource]>,
) -> Option<DxilHkDetail> {
    use dxil_metal::bitcode::dxil_part_bitcode;
    dxil_handle_kinds_detail_inner(dxil_part_bitcode(dxil_part)?, psv_opt)
}

fn dxil_handle_kinds_detail_inner(
    bitcode: &[u8],
    psv_opt: Option<&[PsvResource]>,
) -> Option<DxilHkDetail> {
    const BLOCKINFO: u64 = 0;
    const MODULE_BLOCK: u64 = 8;
    const CONSTANTS_BLOCK: u64 = 11;
    const FUNCTION_BLOCK: u64 = 12;
    const VALUE_SYMTAB_BLOCK: u64 = 14;
    const METADATA_BLOCK: u64 = 15;
    const TYPE_BLOCK_NEW: u64 = 17;

    const MODULE_CODE_GLOBALVAR: u64 = 7;
    const MODULE_CODE_FUNCTION: u64 = 8;
    const MODULE_CODE_ALIAS_OLD: u64 = 9;
    const MODULE_CODE_ALIAS: u64 = 14;

    const TYPE_CODE_NUMENTRY: u64 = 1;
    const TYPE_CODE_POINTER: u64 = 8;
    const TYPE_CODE_STRUCT_NAME: u64 = 19;
    const TYPE_CODE_FUNCTION: u64 = 21;

    const CST_CODE_SETTYPE: u64 = 1;
    const CST_CODE_NULL: u64 = 2;
    const CST_CODE_INTEGER: u64 = 4;
    const CST_CODE_AGGREGATE: u64 = 7;
    const CST_CODE_DATA: u64 = 22;

    const FUNC_CODE_DECLAREBLOCKS: u64 = 1;

    const VST_CODE_ENTRY: u64 = 1;
    const VST_CODE_FNENTRY: u64 = 3;
    const BLOCKINFO_CODE_SETBID: u64 = 1;

    const METADATA_STRING: u64 = 1;
    const METADATA_VALUE: u64 = 2;
    const METADATA_NODE: u64 = 3;
    const METADATA_NAME: u64 = 4;
    const METADATA_DISTINCT_NODE: u64 = 5;
    const METADATA_KIND: u64 = 6;
    const METADATA_NAMED_NODE: u64 = 10;
    const METADATA_ATTACHMENT: u64 = 11;

    #[derive(Clone)]
    enum BcOp {
        Lit(u64),
        Fixed(u32),
        Vbr(u32),
        Array,
        Char6,
        Blob,
    }
    type BcAbbrev = Vec<BcOp>;

    struct BcBits<'a> {
        data: &'a [u8],
        pos: usize,
    }
    impl<'a> BcBits<'a> {
        fn read(&mut self, n: u32) -> Option<u64> {
            if n == 0 {
                return Some(0);
            }
            if n > 64 || self.pos + n as usize > self.data.len() * 8 {
                return None;
            }
            let mut v = 0u64;
            let mut got = 0u32;
            while got < n {
                let byte = self.data[self.pos / 8] as u64;
                let bit = (self.pos % 8) as u32;
                let take = (8 - bit).min(n - got);
                v |= ((byte >> bit) & ((1u64 << take) - 1)) << got;
                got += take;
                self.pos += take as usize;
            }
            Some(v)
        }
        fn vbr(&mut self, n: u32) -> Option<u64> {
            if !(2..=32).contains(&n) {
                return None;
            }
            let hi = 1u64 << (n - 1);
            let mut v = 0u64;
            let mut shift = 0u32;
            loop {
                let piece = self.read(n)?;
                v |= (piece & (hi - 1)).checked_shl(shift)?;
                if piece & hi == 0 {
                    return Some(v);
                }
                shift += n - 1;
                if shift >= 64 {
                    return None;
                }
            }
        }
        fn align32(&mut self) {
            self.pos = self.pos.next_multiple_of(32);
        }
        fn block_header(&mut self) -> Option<(u32, usize)> {
            let w = self.vbr(4)? as u32;
            self.align32();
            let words = self.read(32)? as usize;
            let end = self.pos.checked_add(words.checked_mul(32)?)?;
            (end <= self.data.len() * 8 && w >= 2).then_some((w, end))
        }
    }

    fn read_scalar(b: &mut BcBits, op: &BcOp) -> Option<u64> {
        match *op {
            BcOp::Lit(v) => Some(v),
            BcOp::Fixed(w) => b.read(w),
            BcOp::Vbr(w) => b.vbr(w),
            BcOp::Char6 => b
                .read(6)
                .map(|v| b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._"[v as usize & 63] as u64),
            BcOp::Array | BcOp::Blob => None,
        }
    }

    fn read_define_abbrev(b: &mut BcBits) -> Option<BcAbbrev> {
        let n = b.vbr(5)?;
        let mut ops = Vec::with_capacity((n as usize).min(64));
        for _ in 0..n {
            if b.read(1)? == 1 {
                ops.push(BcOp::Lit(b.vbr(8)?));
            } else {
                ops.push(match b.read(3)? {
                    1 => BcOp::Fixed(b.vbr(5)? as u32),
                    2 => BcOp::Vbr(b.vbr(5)? as u32),
                    3 => BcOp::Array,
                    4 => BcOp::Char6,
                    5 => BcOp::Blob,
                    _ => return None,
                });
            }
        }
        Some(ops)
    }

    fn read_abbrev_rec(b: &mut BcBits, a: &BcAbbrev) -> Option<(u64, Vec<u64>)> {
        #[inline]
        fn push_val(code: &mut Option<u64>, vals: &mut Vec<u64>, v: u64) {
            if code.is_none() {
                *code = Some(v);
            } else {
                vals.push(v);
            }
        }
        let mut code: Option<u64> = None;
        let mut vals = Vec::new();
        let mut i = 0;
        while i < a.len() {
            match &a[i] {
                BcOp::Array => {
                    let elt = a.get(i + 1)?;
                    let n = b.vbr(6)?;
                    if n > 1 << 20 {
                        return None;
                    }
                    vals.reserve((n as usize).min(4096));
                    for _ in 0..n {
                        push_val(&mut code, &mut vals, read_scalar(b, elt)?);
                    }
                    i += 2;
                    continue;
                }
                BcOp::Blob => {
                    let n = b.vbr(6)? as usize;
                    b.align32();
                    if b.pos + n.saturating_mul(8) > b.data.len() * 8 {
                        return None;
                    }
                    vals.reserve(n.min(4096));
                    for _ in 0..n {
                        push_val(&mut code, &mut vals, b.read(8)?);
                    }
                    b.align32();
                }
                op => push_val(&mut code, &mut vals, read_scalar(b, op)?),
            }
            i += 1;
        }
        Some((code?, vals))
    }

    enum BcEntry {
        End,
        Block(u64),
        Rec(u64, Vec<u64>),
    }

    fn next_entry(b: &mut BcBits, w: u32, abbrevs: &mut Vec<BcAbbrev>) -> Option<BcEntry> {
        loop {
            match b.read(w)? {
                0 => {
                    b.align32();
                    return Some(BcEntry::End);
                }
                1 => return Some(BcEntry::Block(b.vbr(8)?)),
                2 => abbrevs.push(read_define_abbrev(b)?),
                3 => {
                    let code = b.vbr(6)?;
                    let n = b.vbr(6)?;
                    if n > 1 << 20 {
                        return None;
                    }
                    let mut ops = Vec::with_capacity((n as usize).min(4096));
                    for _ in 0..n {
                        ops.push(b.vbr(6)?);
                    }
                    return Some(BcEntry::Rec(code, ops));
                }
                id => {
                    let a = abbrevs.get((id as usize).checked_sub(4)?)?;
                    let (c, ops) = read_abbrev_rec(b, a)?;
                    return Some(BcEntry::Rec(c, ops));
                }
            }
        }
    }

    fn parse_blockinfo(b: &mut BcBits, blockinfo: &mut HashMap<u64, Vec<BcAbbrev>>) -> Option<()> {
        let (bw, bend) = b.block_header()?;
        let mut cur: Option<u64> = None;
        while b.pos < bend {
            match b.read(bw)? {
                0 => {
                    b.align32();
                    return Some(());
                }
                1 => {
                    let _ = b.vbr(8)?;
                    let (_, send) = b.block_header()?;
                    b.pos = send;
                }
                2 => {
                    let a = read_define_abbrev(b)?;
                    blockinfo.entry(cur?).or_default().push(a);
                }
                3 => {
                    let code = b.vbr(6)?;
                    let n = b.vbr(6)?;
                    let mut first = None;
                    for idx in 0..n {
                        let v = b.vbr(6)?;
                        if idx == 0 {
                            first = Some(v);
                        }
                    }
                    if code == BLOCKINFO_CODE_SETBID {
                        cur = first;
                    }
                }
                _ => return None,
            }
        }
        Some(())
    }

    if bitcode.get(0..4)? != b"BC\xC0\xDE" {
        return None;
    }

    let mut b = BcBits { data: bitcode, pos: 32 };
    let mut blockinfo: HashMap<u64, Vec<BcAbbrev>> = HashMap::new();
    let mut top_abbrevs = Vec::new();
    let mut mod_header: Option<(u32, usize)> = None;
    while b.pos < bitcode.len() * 8 {
        match next_entry(&mut b, 2, &mut top_abbrevs)? {
            BcEntry::Block(BLOCKINFO) => parse_blockinfo(&mut b, &mut blockinfo)?,
            BcEntry::Block(MODULE_BLOCK) => {
                mod_header = Some(b.block_header()?);
                break;
            }
            BcEntry::Block(_) => {
                let (_, send) = b.block_header()?;
                b.pos = send;
            }
            BcEntry::End | BcEntry::Rec(..) => return None,
        }
    }
    let (mw, mend) = mod_header?;
    let mut mab = blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();

    let mut types: Vec<(u64, Vec<u64>)> = Vec::new();
    let mut res_props_ty: Option<u64> = None;
    let mut handle_ty: Option<u64> = None;
    let mut names: HashMap<u64, String> = HashMap::new();
    let mut mod_consts_spans: Vec<(u32, usize, usize)> = Vec::new();
    let mut md_spans: Vec<(u32, usize, usize)> = Vec::new();
    let mut fn_spans: Vec<(u32, usize, usize)> = Vec::new();
    let mut mod_fns: Vec<(u64, u64, bool)> = Vec::new();
    let mut next_mod_val = 0u64;

    while b.pos < mend {
        match next_entry(&mut b, mw, &mut mab)? {
            BcEntry::End => break,
            BcEntry::Block(BLOCKINFO) => {
                parse_blockinfo(&mut b, &mut blockinfo)?;
                mab = blockinfo.get(&MODULE_BLOCK).cloned().unwrap_or_default();
            }
            BcEntry::Block(TYPE_BLOCK_NEW) => {
                let (tw, tend) = b.block_header()?;
                let mut tab = blockinfo.get(&TYPE_BLOCK_NEW).cloned().unwrap_or_default();
                let mut pending_name: Option<String> = None;
                while b.pos < tend {
                    match next_entry(&mut b, tw, &mut tab)? {
                        BcEntry::End => break,
                        BcEntry::Block(_) => {
                            let (_, send) = b.block_header()?;
                            b.pos = send;
                        }
                        BcEntry::Rec(TYPE_CODE_NUMENTRY, _) => {}
                        BcEntry::Rec(TYPE_CODE_STRUCT_NAME, ops) => {
                            pending_name =
                                Some(ops.iter().map(|&ch| (ch as u8) as char).collect());
                        }
                        BcEntry::Rec(rc, ops) => {
                            let tid = types.len() as u64;
                            if let Some(nm) = pending_name.take() {
                                if nm == "dx.types.ResourceProperties" {
                                    res_props_ty = Some(tid);
                                } else if nm == "dx.types.Handle" {
                                    handle_ty = Some(tid);
                                }
                            }
                            types.push((rc, ops));
                        }
                    }
                }
                b.pos = tend;
            }
            BcEntry::Block(VALUE_SYMTAB_BLOCK) => {
                let (vw, vend) = b.block_header()?;
                let mut vab = blockinfo.get(&VALUE_SYMTAB_BLOCK).cloned().unwrap_or_default();
                while b.pos < vend {
                    match next_entry(&mut b, vw, &mut vab)? {
                        BcEntry::End => break,
                        BcEntry::Block(_) => {
                            let (_, send) = b.block_header()?;
                            b.pos = send;
                        }
                        BcEntry::Rec(code @ (VST_CODE_ENTRY | VST_CODE_FNENTRY), ops) => {
                            // ENTRY has [value ID, name...]. Newer LLVM FNENTRY adds a
                            // function offset before the name. BBENTRY (2) names blocks.
                            let start = if code == VST_CODE_FNENTRY { 2 } else { 1 };
                            let name =
                                ops.get(start..)?.iter().map(|&ch| ch as u8 as char).collect();
                            if names.insert(*ops.first()?, name).is_some() {
                                return None;
                            }
                        }
                        BcEntry::Rec(..) => {}
                    }
                }
                b.pos = vend;
            }
            BcEntry::Block(CONSTANTS_BLOCK) => {
                let (cw, cend) = b.block_header()?;
                let cstart = b.pos;
                mod_consts_spans.push((cw, cstart, cend));
                b.pos = cend;
            }
            BcEntry::Block(METADATA_BLOCK) => {
                let (mdw, mdend) = b.block_header()?;
                let mdstart = b.pos;
                md_spans.push((mdw, mdstart, mdend));
                b.pos = mdend;
            }
            BcEntry::Block(FUNCTION_BLOCK) => {
                let (fw, fend) = b.block_header()?;
                let fstart = b.pos;
                fn_spans.push((fw, fstart, fend));
                b.pos = fend;
            }
            BcEntry::Block(_) => {
                let (_, send) = b.block_header()?;
                b.pos = send;
            }
            BcEntry::Rec(code, ops) => match code {
                MODULE_CODE_GLOBALVAR | MODULE_CODE_ALIAS_OLD | MODULE_CODE_ALIAS => {
                    next_mod_val += 1;
                }
                MODULE_CODE_FUNCTION => {
                    let vid = next_mod_val;
                    next_mod_val += 1;
                    let ty_id = *ops.first()?;
                    let is_proto = ops.get(2).copied().unwrap_or(1) != 0;
                    mod_fns.push((vid, ty_id, is_proto));
                }
                _ => {}
            },
        }
    }

    if types.is_empty() || mod_fns.is_empty() {
        return None;
    }
    let get_fn_sig = |mut ty_id: u64| -> Option<(u64, &[u64])> {
        let (mut rc, mut rops) = types.get(ty_id as usize).map(|(c, o)| (*c, o.as_slice()))?;
        if rc == TYPE_CODE_POINTER {
            ty_id = *rops.first()?;
            (rc, rops) = types.get(ty_id as usize).map(|(c, o)| (*c, o.as_slice()))?;
        }
        if rc != TYPE_CODE_FUNCTION || rops.len() < 2 {
            return None;
        }
        Some((rops[1], &rops[2..]))
    };

    let mut has_annotate = false;
    let mut has_sm66_create = false;
    let mut has_legacy_create_handle = false;
    let mut has_lib_create_handle = false;
    let mut defined_param_counts: Vec<u64> = Vec::new();
    for &(vid, ty_id, is_proto) in &mod_fns {
        let (retty, params) = get_fn_sig(ty_id)?;
        if !is_proto {
            if let Some(hty) = handle_ty {
                if retty == hty || params.contains(&hty) {
                    return None;
                }
            }
            defined_param_counts.push(params.len() as u64);
        }
        if Some(retty) == handle_ty {
            match names.get(&vid).map(String::as_str).unwrap_or("") {
                "dx.op.annotateHandle" => has_annotate = true,
                "dx.op.createHandleFromHeap" | "dx.op.createHandleFromBinding" => {
                    has_sm66_create = true;
                }
                "dx.op.createHandle" => has_legacy_create_handle = true,
                s if s.starts_with("dx.op.createHandleForLib.") => has_lib_create_handle = true,
                _ => return None,
            }
        }
    }
    if has_sm66_create && !has_annotate {
        return None;
    }
    if has_annotate && res_props_ty.is_none() {
        return None;
    }

    let mut srv_mask = 0u32;
    let mut uav_mask = 0u32;
    let mut srv_ranges: Vec<DxilSrvRangeHk> = Vec::new();
    let mut mod_const_i32: HashMap<u64, u32> = HashMap::new();

    let decode_signed_vbr_u32 = |raw: u64| -> u32 {
        if (raw & 1) != 0 {
            0u64.wrapping_sub(raw >> 1) as u32
        } else {
            (raw >> 1) as u32
        }
    };

    let mut saw_properties = false;
    let mut val_cursor = next_mod_val;
    for &(cw, cstart, cend) in &mod_consts_spans {
        let mut cb = BcBits { data: bitcode, pos: cstart };
        let mut cab = blockinfo.get(&CONSTANTS_BLOCK).cloned().unwrap_or_default();
        let mut cur_ty: Option<u64> = None;
        while cb.pos < cend {
            match next_entry(&mut cb, cw, &mut cab)? {
                BcEntry::End => break,
                BcEntry::Block(_) => {
                    let (_, send) = cb.block_header()?;
                    cb.pos = send;
                }
                BcEntry::Rec(CST_CODE_SETTYPE, ops) => {
                    cur_ty = Some(*ops.first()?);
                }
                BcEntry::Rec(rc, ops) => {
                    let vid = val_cursor;
                    val_cursor += 1;
                    if cur_ty.is_some() && cur_ty == res_props_ty {
                        let (d0, d1) = if rc == CST_CODE_AGGREGATE && ops.len() == 2 {
                            (*mod_const_i32.get(&ops[0])?, *mod_const_i32.get(&ops[1])?)
                        } else if rc == CST_CODE_DATA && ops.len() == 2 {
                            (ops[0] as u32, ops[1] as u32)
                        } else {
                            return None;
                        };
                        let (s, u) = decode_dxil_resource_properties(d0, d1)?;
                        saw_properties = true;
                        srv_mask |= s;
                        uav_mask |= u;
                    } else if rc == CST_CODE_NULL {
                        mod_const_i32.insert(vid, 0);
                    } else if rc == CST_CODE_INTEGER {
                        mod_const_i32.insert(vid, decode_signed_vbr_u32(*ops.first()?));
                    }
                }
            }
        }
    }
    let module_val_size = val_cursor;

    if fn_spans.len() != defined_param_counts.len() {
        return None;
    }

    if res_props_ty.is_some() {
        for (&(fw, fstart, fend), &nparams) in fn_spans.iter().zip(&defined_param_counts) {
            let mut fb = BcBits { data: bitcode, pos: fstart };
            let mut fab = blockinfo.get(&FUNCTION_BLOCK).cloned().unwrap_or_default();
            let mut saw_inst_before_consts = false;
            let mut fn_val_cursor = module_val_size + nparams;
            let mut fn_const_i32: HashMap<u64, u32> = HashMap::new();
            while fb.pos < fend {
                match next_entry(&mut fb, fw, &mut fab)? {
                    BcEntry::End => break,
                    BcEntry::Block(CONSTANTS_BLOCK) => {
                        if saw_inst_before_consts {
                            return None;
                        }
                        let (cw, cend) = fb.block_header()?;
                        let mut cab =
                            blockinfo.get(&CONSTANTS_BLOCK).cloned().unwrap_or_default();
                        let mut cur_ty: Option<u64> = None;
                        while fb.pos < cend {
                            match next_entry(&mut fb, cw, &mut cab)? {
                                BcEntry::End => break,
                                BcEntry::Block(_) => {
                                    let (_, send) = fb.block_header()?;
                                    fb.pos = send;
                                }
                                BcEntry::Rec(CST_CODE_SETTYPE, ops) => {
                                    cur_ty = Some(*ops.first()?);
                                }
                                BcEntry::Rec(rc, ops) => {
                                    let vid = fn_val_cursor;
                                    fn_val_cursor += 1;
                                    if cur_ty == res_props_ty {
                                        let (d0, d1) = if rc == CST_CODE_AGGREGATE && ops.len() == 2
                                        {
                                            let a = fn_const_i32
                                                .get(&ops[0])
                                                .or_else(|| mod_const_i32.get(&ops[0]))
                                                .copied()?;
                                            let b = fn_const_i32
                                                .get(&ops[1])
                                                .or_else(|| mod_const_i32.get(&ops[1]))
                                                .copied()?;
                                            (a, b)
                                        } else if rc == CST_CODE_DATA && ops.len() == 2 {
                                            (ops[0] as u32, ops[1] as u32)
                                        } else {
                                            return None;
                                        };
                                        let (s, u) = decode_dxil_resource_properties(d0, d1)?;
                                        saw_properties = true;
                                        srv_mask |= s;
                                        uav_mask |= u;
                                    } else if rc == CST_CODE_NULL {
                                        fn_const_i32.insert(vid, 0);
                                    } else if rc == CST_CODE_INTEGER {
                                        fn_const_i32
                                            .insert(vid, decode_signed_vbr_u32(*ops.first()?));
                                    }
                                }
                            }
                        }
                        fb.pos = cend;
                    }
                    BcEntry::Block(_) => {
                        let (_, send) = fb.block_header()?;
                        fb.pos = send;
                    }
                    BcEntry::Rec(FUNC_CODE_DECLAREBLOCKS, _) => {}
                    BcEntry::Rec(..) => {
                        saw_inst_before_consts = true;
                    }
                }
            }
        }
    }

    if has_annotate && !saw_properties {
        return None;
    }

    enum MdItem {
        Int(Option<u32>),
        Node(Vec<u64>),
        Other,
    }

    let mut md_nodes: Vec<MdItem> = Vec::new();
    let mut dx_res_ops: Vec<u64> = Vec::new();
    for &(mdw, mdstart, mdend) in &md_spans {
        let mut mb = BcBits { data: bitcode, pos: mdstart };
        let mut mdab = blockinfo.get(&METADATA_BLOCK).cloned().unwrap_or_default();
        let mut pending_md_name: Option<String> = None;
        while mb.pos < mdend {
            match next_entry(&mut mb, mdw, &mut mdab)? {
                BcEntry::End => break,
                BcEntry::Block(_) => {
                    let (_, send) = mb.block_header()?;
                    mb.pos = send;
                }
                BcEntry::Rec(METADATA_STRING, _) => {
                    md_nodes.push(MdItem::Other);
                }
                BcEntry::Rec(METADATA_VALUE, ops) => {
                    let iv = ops.get(1).and_then(|v| mod_const_i32.get(v).copied());
                    md_nodes.push(MdItem::Int(iv));
                }
                BcEntry::Rec(METADATA_NODE | METADATA_DISTINCT_NODE, ops) => {
                    md_nodes.push(MdItem::Node(ops));
                }
                BcEntry::Rec(METADATA_NAME, ops) => {
                    pending_md_name = Some(ops.iter().map(|&ch| (ch as u8) as char).collect());
                }
                BcEntry::Rec(METADATA_NAMED_NODE, ops) => {
                    if pending_md_name.take().as_deref() == Some("dx.resources") {
                        dx_res_ops.extend_from_slice(&ops);
                    }
                }
                BcEntry::Rec(METADATA_KIND | METADATA_ATTACHMENT, _) => {}
                BcEntry::Rec(7 | 12..=32, _) => md_nodes.push(MdItem::Other),
                // A new record need not occupy exactly one metadata slot. Guessing its
                // arity would shift every following reference in !dx.resources.
                BcEntry::Rec(..) => return None,
            }
        }
    }

    if !dx_res_ops.is_empty() {
        let md_node_1 = |id_1based: u64| -> Option<&[u64]> {
            let idx = (id_1based as usize).checked_sub(1)?;
            match md_nodes.get(idx)? {
                MdItem::Node(v) => Some(v.as_slice()),
                _ => None,
            }
        };
        let md_i32_1 = |id_1based: u64| -> Option<u32> {
            let idx = (id_1based as usize).checked_sub(1)?;
            match md_nodes.get(idx)? {
                MdItem::Int(v) => *v,
                _ => None,
            }
        };
        let extract_elem_comp = |tag_ref: u64| -> Option<Option<u32>> {
            if tag_ref == 0 {
                return Some(None);
            }
            let tags = md_node_1(tag_ref)?;
            if !tags.len().is_multiple_of(2) {
                return None;
            }
            for pair in tags.chunks_exact(2) {
                let k = md_i32_1(pair[0])?;
                if k == 0 {
                    return Some(Some(md_i32_1(pair[1])?));
                }
            }
            Some(None)
        };

        for &root_idx in &dx_res_ops {
            let rlist = match md_nodes.get(root_idx as usize)? {
                MdItem::Node(v) if v.len() == 4 => v.as_slice(),
                _ => return None,
            };
            // rlist[0]: SRV list (`[id, ptr, name, space, lower, range_size, shape, samples, tags]`)
            if rlist[0] != 0 {
                for &s_ref in md_node_1(rlist[0])? {
                    let flds = md_node_1(s_ref)?;
                    if flds.len() < 9 {
                        return None;
                    }
                    let space = md_i32_1(flds[3])?;
                    let lower_bound = md_i32_1(flds[4])?;
                    let range_size = md_i32_1(flds[5])?;
                    if range_size == 0 {
                        return None;
                    }
                    let upper_bound = if range_size == u32::MAX {
                        u32::MAX
                    } else {
                        lower_bound.saturating_add(range_size - 1)
                    };
                    let shape = md_i32_1(flds[6])?;
                    let comp = extract_elem_comp(flds[8])?;
                    let hk = decode_dxil_shape_and_comp(shape, comp)? & !HK_CBV_BUF;
                    srv_mask |= hk;
                    srv_ranges.push(DxilSrvRangeHk { space, lower_bound, upper_bound, hk });
                }
            }
            // rlist[1]: UAV list (`[id, ptr, name, space, lower, range_size, shape, glob, ctr, rov, tags]`)
            if rlist[1] != 0 {
                for &u_ref in md_node_1(rlist[1])? {
                    let flds = md_node_1(u_ref)?;
                    if flds.len() < 11 {
                        return None;
                    }
                    let shape = md_i32_1(flds[6])?;
                    let comp = extract_elem_comp(flds[10])?;
                    let hk = decode_dxil_shape_and_comp(shape, comp)? & !HK_CBV_BUF;
                    uav_mask |= hk;
                }
            }
            // rlist[2]: CBV list
            if rlist[2] != 0 && !md_node_1(rlist[2])?.is_empty() {
                srv_mask |= HK_CBV_BUF;
            }
        }
    } else if has_legacy_create_handle || has_lib_create_handle {
        let psv = psv_opt?;
        let (ps, pu) = psv_resource_handle_kinds(psv);
        srv_mask |= ps;
        uav_mask |= pu;
    }

    Some(DxilHkDetail { srv_mask, uav_mask, srv_ranges })
}
