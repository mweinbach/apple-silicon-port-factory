//! Shader stage reflection: `.metallib` `RBUF` inspection, MSC `"UsedResources"`
//! and `"ResourceUsages"` JSON parsing, `ShaderStageSummary`, `ComputeAccess`,
//! and per-stage SRV/UAV/CBV resource-class queries.

use dxil_metal::dxbc::PsvResource;
use dxil_metal::root_signature::{
    DescriptorRangeType, ResourceType as T, RootDescriptorKind, RootParameterLayoutKind,
    RootSignature,
};
use dxil_metal::CompiledShader;

use super::dxil_hk::{
    register_shader_hk_profile, resolve_shader_hk_profile_from_summary, HandleKindProfile,
};
use super::opts::{
    air_no_res_heap, cs_mid_qb_exact_v2, msc_compute_access, msc_stage_usage, sm66_cs_stage_exact,
};

const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
const MAX_BOUNDED_ARRAY: u32 = 128;

/// [`stage_reads_resources`] class bits.
pub const READS_TEX: u8 = 1;
pub const READS_BUF: u8 = 2;

/// Fail-closed check on an embedded `.metallib`'s `RBUF` reflection chunk proving that
/// MSC's `ResourceDescriptorHeap` argument buffer (`struct.res_desc_heap_ab`, `buffer(0)`)
/// is statically marked `unused == 1` by the AIR compiler.
pub fn metallib_res_desc_heap_unused(metallib: &[u8]) -> bool {
    const NAME: &[u8] = b"struct.res_desc_heap_ab\0";
    if !metallib.starts_with(b"MTLB") {
        return false;
    }
    if metallib.windows(NAME.len()).filter(|w| *w == NAME).count() != 1 {
        return false;
    }
    let Some(rbuf_pos) = metallib.windows(4).rposition(|w| w == b"RBUF") else {
        return false;
    };
    let tail = &metallib[rbuf_pos..];
    let Some(rpos) = tail.windows(NAME.len()).position(|w| w == NAME) else {
        return false;
    };
    if rpos < 27 {
        return false;
    }
    // Exact 27-byte FlatBuffer `BufferBinding` layout preceding `struct.res_desc_heap_ab\0`:
    //   [rpos - 27 .. rpos - 16]: access=1 (ReadOnly), addrspace=2 (Constant), unused=1 (true),
    //                             location_index=0u32, location_count=1u32
    //   [rpos - 16 .. rpos - 12]: vtable soffset_t (varies per shader)
    //   [rpos - 12 .. rpos]:      typename uoffset_t=8u32, name uoffset_t=4u32, strlen=23u32
    tail[rpos - 27..rpos - 16] == [0x01, 0x02, 0x01, 0, 0, 0, 0, 1, 0, 0, 0]
        && tail[rpos - 12..rpos] == [8, 0, 0, 0, 4, 0, 0, 0, 23, 0, 0, 0]
}

/// One entry of Metal Shader Converter's `"UsedResources"` reflection array.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MscUsedResource {
    pub binding_index: u32,
    pub is_in_grs: bool,
    pub table_start_index: u32,
    pub table_length: u32,
}

/// Parses Metal Shader Converter's `"UsedResources"` array from `json`
/// (`sh.reflection.json`, already stored in cached `.aw2s` blobs).
pub fn msc_used_resources(json: &str) -> Option<Vec<MscUsedResource>> {
    const NEEDLE: &str = "\"UsedResources\"";
    let pos = json.find(NEEDLE)?;
    let rest = json[pos + NEEDLE.len()..].trim_start().strip_prefix(':')?.trim_start();
    let body = rest.strip_prefix('[')?;
    let mut depth = 1usize;
    let mut in_str = false;
    let mut esc = false;
    let mut end_idx = None;
    for (i, b) in body.bytes().enumerate() {
        if in_str {
            if esc {
                esc = false;
            } else if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_str = false;
            }
            continue;
        }
        match b {
            b'"' => in_str = true,
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    end_idx = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }
    let arr = body[..end_idx?].trim();
    if arr.is_empty() {
        return Some(Vec::new());
    }

    let parse_u32_field = |obj: &str, key: &str| -> Option<u32> {
        let p = obj.find(key)?;
        let r = obj[p + key.len()..].trim_start().strip_prefix(':')?.trim_start();
        let n = r.bytes().take_while(|b| b.is_ascii_digit()).count();
        if n == 0 {
            return None;
        }
        r[..n].parse::<u32>().ok()
    };
    let parse_bool_field = |obj: &str, key: &str| -> Option<bool> {
        let p = obj.find(key)?;
        let r = obj[p + key.len()..].trim_start().strip_prefix(':')?.trim_start();
        if r.starts_with("true") {
            Some(true)
        } else if r.starts_with("false") {
            Some(false)
        } else {
            None
        }
    };

    let mut out = Vec::new();
    let mut cur = arr;
    while let Some(open) = cur.find('{') {
        let after_open = &cur[open + 1..];
        let close = after_open.find('}')?;
        let obj = &after_open[..close];
        let binding_index = parse_u32_field(obj, "\"bindingIndex\"")?;
        let is_in_grs = parse_bool_field(obj, "\"isInGRS\"")?;
        let table_start_index = parse_u32_field(obj, "\"tableStartIndex\"").unwrap_or(u32::MAX);
        let table_length = parse_u32_field(obj, "\"tableLength\"").unwrap_or(u32::MAX);
        out.push(MscUsedResource {
            binding_index,
            is_in_grs,
            table_start_index,
            table_length,
        });
        cur = &after_open[close + 1..];
    }
    Some(out)
}

/// Parses Metal Shader Converter's top-level `"ResourceUsages"` bitmask from
/// `sh.reflection.json` (already stored inside cached `.aw2s` files):
/// - `1 << 1` (`2`): Buffer read (`SRV` buffer / typed buffer)
/// - `1 << 2` (`4`): Buffer write (`UAV` buffer / typed buffer)
/// - `1 << 3` (`8`): Texture read (`SRV` `Texture*`)
/// - `1 << 4` (`16`): Texture write (`UAV` `RWTexture*`)
/// - `1 << 5` (`32`): AccelerationStructure (`RaytracingAccelerationStructure`)
pub fn msc_resource_usages(sh: &CompiledShader) -> Option<u32> {
    const NEEDLE: &str = "\"ResourceUsages\"";
    let json = sh.reflection.json.as_str();
    let pos = json.find(NEEDLE)?;
    let rest = json[pos + NEEDLE.len()..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let digits_len = rest.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digits_len == 0 {
        return None;
    }
    rest[..digits_len].parse::<u32>().ok()
}

/// Shared implementation of PSV0 -> `ComputeAccess` extraction.
fn compute_access_from_parts(
    dxil_parsed: bool,
    dxil_feature_flags: u64,
    psv_opt: Option<&[PsvResource]>,
    rs: &RootSignature,
) -> ComputeAccess {
    if !dxil_parsed || (dxil_feature_flags & RESOURCE_HEAP_INDEXING) != 0 {
        return ComputeAccess::opaque();
    }
    let Some(psv) = psv_opt else {
        return ComputeAccess::opaque();
    };

    let layout = rs.layout();
    let mut uav_ok = true;
    let mut read_all_bounded = true;
    let mut unbounded_srv_classes = 0u8;
    let mut heap_indexed_or_unknown = false;
    let mut psv_touches_textures = false;
    let mut uav_slots: Vec<ComputeBind> = Vec::new();
    let mut bounded_read_slots: Vec<ComputeBind> = Vec::new();

    let push_unique = |v: &mut Vec<ComputeBind>, b: ComputeBind| {
        if !v.contains(&b) {
            v.push(b);
        }
    };

    for r in psv {
        if r.res_type == 1 {
            // Samplers are immutable/stateless in D3D12 (no ResourceBarrier transitions).
            continue;
        }
        let (is_uav, range_ty, root_kind, allow_constants) = match r.res_type {
            2 => (false, DescriptorRangeType::Cbv, RootDescriptorKind::Cbv, true),
            3..=5 => (false, DescriptorRangeType::Srv, RootDescriptorKind::Srv, false),
            6..=9 => (true, DescriptorRangeType::Uav, RootDescriptorKind::Uav, false),
            _ => return ComputeAccess::opaque(),
        };

        let srv_class = if r.is_texture() {
            READS_TEX
        } else if r.is_buffer() {
            READS_BUF
        } else {
            READS_TEX | READS_BUF
        };
        if matches!(r.res_type, 3..=9) && (srv_class & READS_TEX) != 0 {
            psv_touches_textures = true;
        }

        if r.upper_bound == u32::MAX || r.upper_bound < r.lower_bound {
            if is_uav {
                uav_ok = false;
            } else {
                read_all_bounded = false;
                if range_ty == DescriptorRangeType::Srv {
                    unbounded_srv_classes |= srv_class;
                } else {
                    heap_indexed_or_unknown = true;
                }
            }
            continue;
        }
        let count = r.upper_bound - r.lower_bound + 1;
        if count > MAX_BOUNDED_ARRAY {
            if is_uav {
                uav_ok = false;
            } else {
                read_all_bounded = false;
                if range_ty == DescriptorRangeType::Srv {
                    unbounded_srv_classes |= srv_class;
                } else {
                    heap_indexed_or_unknown = true;
                }
            }
            continue;
        }

        for reg in r.lower_bound..=r.upper_bound {
            let mut matched = false;
            for (pi, p) in layout.parameters.iter().enumerate() {
                if pi > u16::MAX as usize {
                    return ComputeAccess::opaque();
                }
                match &p.kind {
                    RootParameterLayoutKind::Constants {
                        shader_register,
                        register_space,
                        ..
                    } if allow_constants
                        && *register_space == r.space
                        && *shader_register == reg =>
                    {
                        // Inline root 32-bit constants live in the uploaded TLAB, not a Resource.
                        matched = true;
                        break;
                    }
                    RootParameterLayoutKind::Descriptor {
                        kind,
                        shader_register,
                        register_space,
                    } if *kind == root_kind
                        && *register_space == r.space
                        && *shader_register == reg =>
                    {
                        let bind = ComputeBind::RootVa { param: pi as u16 };
                        if is_uav {
                            push_unique(&mut uav_slots, bind);
                        } else {
                            push_unique(&mut bounded_read_slots, bind);
                        }
                        matched = true;
                        break;
                    }
                    RootParameterLayoutKind::DescriptorTable { ranges, .. } => {
                        if let Some(rg) = ranges.iter().find(|rg| {
                            rg.range_type == range_ty
                                && rg.register_space == r.space
                                && reg >= rg.base_shader_register
                                && rg
                                    .num_descriptors
                                    .is_none_or(|n| reg - rg.base_shader_register < n)
                        }) {
                            let byte_offset =
                                rg.byte_offset + (reg - rg.base_shader_register) as u64 * 24;
                            let bind = ComputeBind::TableEntry { param: pi as u16, byte_offset };
                            if is_uav {
                                push_unique(&mut uav_slots, bind);
                            } else {
                                push_unique(&mut bounded_read_slots, bind);
                            }
                            matched = true;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if !matched {
                return ComputeAccess::opaque();
            }
        }
    }

    let touches_textures = psv_touches_textures || (unbounded_srv_classes & READS_TEX) != 0;
    ComputeAccess {
        uav_slots: uav_ok.then_some(uav_slots),
        read_slots: read_all_bounded.then(|| bounded_read_slots.clone()),
        bounded_read_slots,
        unbounded_srv_classes,
        heap_indexed_or_unknown,
        touches_textures,
        uav_classes: READS_TEX | READS_BUF,
        read_classes: READS_TEX | READS_BUF,
        uav_srv_classes: READS_TEX | READS_BUF,
        msc_refined: false,
        msc_exact_classes: READS_TEX | READS_BUF,
    }
}

/// Shared implementation of MSC `"UsedResources"` + `"ResourceUsages"` refinement on `ComputeAccess`.
#[allow(clippy::too_many_arguments)]
fn refine_compute_access_from_parts(
    mut acc: ComputeAccess,
    heap_indexed: bool,
    psv_opt: Option<&[PsvResource]>,
    used_opt: Option<&[MscUsedResource]>,
    ru_opt: Option<u32>,
    rs: &RootSignature,
    enable_used_resources: bool,
    ignore_heap_flag: bool,
) -> ComputeAccess {
    let was_uav_none = acc.uav_slots.is_none();
    let was_read_none = acc.read_slots.is_none();
    let prev_uav_slots = acc.uav_slots.clone();
    let prev_read_slots = acc.read_slots.clone();
    let prev_bounded_reads = acc.bounded_read_slots.clone();

    let effective_heap_indexed = heap_indexed && !ignore_heap_flag;
    let layout = rs.layout();

    let mut msc_applied = false;
    let mut msc_has_cbv = false;

    // Fix 1: `"UsedResources"` resolves statically-indexed entries inside unbounded
    // root-signature descriptor tables whenever SM 6.6 `RESOURCE_HEAP_INDEXING` is off
    // (or proven unused in AIR via `metallib_res_desc_heap_unused`).
    if enable_used_resources && !effective_heap_indexed {
        if let Some(used) = used_opt {
            let param_len = layout.parameters.len();
            let has_static_samplers = layout.static_samplers.is_some();
            if used.iter().all(|u| {
                u.is_in_grs
                    && ((u.binding_index as usize) < param_len
                        || (has_static_samplers && (u.binding_index as usize) == param_len))
                    && u.binding_index <= u16::MAX as u32
            }) {
                let mut msc_valid = true;
                let mut msc_uav_ok = true;
                let mut msc_read_all_bounded = true;
                let mut msc_unbounded_srv_classes = 0u8;
                let mut msc_heap_or_unknown = false;
                let mut msc_uav_slots: Vec<ComputeBind> = Vec::new();
                let mut msc_bounded_reads: Vec<ComputeBind> = Vec::new();

                let push_unique = |v: &mut Vec<ComputeBind>, b: ComputeBind| {
                    if !v.contains(&b) {
                        v.push(b);
                    }
                };

                'used_loop: for u in used {
                    if (u.binding_index as usize) == param_len {
                        // MSC static sampler table slot at the end of TLAB (stateless in D3D12).
                        continue;
                    }
                    let param = u.binding_index as u16;
                    match &layout.parameters[u.binding_index as usize].kind {
                        RootParameterLayoutKind::Constants { .. } => {
                            // Inline 32-bit root constants live in TLAB, not a GPU Resource.
                        }
                        RootParameterLayoutKind::Descriptor { kind, .. } => {
                            let bind = ComputeBind::RootVa { param };
                            match kind {
                                RootDescriptorKind::Uav => push_unique(&mut msc_uav_slots, bind),
                                RootDescriptorKind::Cbv => {
                                    msc_has_cbv = true;
                                    push_unique(&mut msc_bounded_reads, bind);
                                }
                                RootDescriptorKind::Srv => {
                                    push_unique(&mut msc_bounded_reads, bind);
                                }
                            }
                        }
                        RootParameterLayoutKind::DescriptorTable { sampler: true, .. } => {}
                        RootParameterLayoutKind::DescriptorTable { sampler: false, ranges } => {
                            if u.table_length == 0 {
                                continue;
                            }
                            let bounded_end = (u.table_start_index != u32::MAX
                                && u.table_length <= MAX_BOUNDED_ARRAY)
                                .then(|| u.table_start_index.checked_add(u.table_length))
                                .flatten();
                            if let Some(end) = bounded_end {
                                for d in u.table_start_index..end {
                                    let Some(rg) = ranges.iter().find(|rg| {
                                        d >= rg.descriptor_offset
                                            && rg
                                                .num_descriptors
                                                .is_none_or(|n| d - rg.descriptor_offset < n)
                                    }) else {
                                        msc_valid = false;
                                        break 'used_loop;
                                    };
                                    let bind = ComputeBind::TableEntry {
                                        param,
                                        byte_offset: (d as u64) * 24,
                                    };
                                    match rg.range_type {
                                        DescriptorRangeType::Sampler => {}
                                        DescriptorRangeType::Uav => {
                                            push_unique(&mut msc_uav_slots, bind);
                                        }
                                        DescriptorRangeType::Cbv => {
                                            msc_has_cbv = true;
                                            push_unique(&mut msc_bounded_reads, bind);
                                        }
                                        DescriptorRangeType::Srv => {
                                            push_unique(&mut msc_bounded_reads, bind);
                                        }
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
                                    match rg.range_type {
                                        DescriptorRangeType::Sampler => {}
                                        DescriptorRangeType::Uav => {
                                            msc_uav_ok = false;
                                        }
                                        DescriptorRangeType::Cbv => {
                                            msc_has_cbv = true;
                                            msc_read_all_bounded = false;
                                            msc_heap_or_unknown = true;
                                        }
                                        DescriptorRangeType::Srv => {
                                            msc_read_all_bounded = false;
                                            let rg_reg_end =
                                                rg.num_descriptors.map_or(u32::MAX, |n| {
                                                    rg.base_shader_register.saturating_add(n)
                                                });
                                            let cls = psv_opt
                                                .map(|psv| {
                                                    let mut c = 0u8;
                                                    for r in psv {
                                                        if !(3..=5).contains(&r.res_type)
                                                            || r.space != rg.register_space
                                                        {
                                                            continue;
                                                        }
                                                        let r_end = if r.upper_bound == u32::MAX {
                                                            u32::MAX
                                                        } else {
                                                            r.upper_bound.saturating_add(1)
                                                        };
                                                        if r_end <= rg.base_shader_register
                                                            || r.lower_bound >= rg_reg_end
                                                        {
                                                            continue;
                                                        }
                                                        c |= if r.is_texture() {
                                                            READS_TEX
                                                        } else if r.is_buffer() {
                                                            READS_BUF
                                                        } else {
                                                            READS_TEX | READS_BUF
                                                        };
                                                    }
                                                    if c == 0 { READS_TEX | READS_BUF } else { c }
                                                })
                                                .unwrap_or(READS_TEX | READS_BUF);
                                            msc_unbounded_srv_classes |= cls;
                                        }
                                    }
                                }
                                if !any_overlap {
                                    msc_valid = false;
                                    break 'used_loop;
                                }
                            }
                        }
                    }
                }

                if msc_valid {
                    msc_applied = true;
                    acc.uav_slots = msc_uav_ok.then_some(msc_uav_slots);
                    acc.read_slots = msc_read_all_bounded.then(|| msc_bounded_reads.clone());
                    acc.bounded_read_slots = msc_bounded_reads;
                    acc.unbounded_srv_classes = msc_unbounded_srv_classes;
                    acc.heap_indexed_or_unknown = msc_heap_or_unknown;
                }
            }
        }
    }

    // Fix 2: `"ResourceUsages"` refines per-class UAV / read masks (`READS_BUF`, `READS_TEX`).
    // Note: MSC `"ResourceUsages"` bit `32` (`1 << 5`) is `AccelerationStructure`, NOT `CBV`;
    // shaders that read a GPU-written buffer via root/table CBV can have `(ru & 2) == 0`, so
    // `may_read_cbv_resource` preserves `READS_BUF` in `read_classes` whenever any non-inline
    // CBV resource may be accessed.
    let may_read_cbv_resource = if msc_applied {
        msc_has_cbv
    } else if heap_indexed {
        true
    } else if let Some(psv) = psv_opt {
        psv.iter().any(|r| {
            r.res_type == 2
                && !layout.parameters.iter().any(|p| {
                    matches!(
                        p.kind,
                        RootParameterLayoutKind::Constants {
                            shader_register,
                            register_space,
                            ..
                        } if register_space == r.space
                            && r.lower_bound == shader_register
                            && r.upper_bound == shader_register
                    )
                })
        })
    } else {
        true
    };

    if let Some(ru) = ru_opt {
        if (ru & !0x3e) == 0 {
            let mut uc = 0u8;
            if (ru & 4) != 0 {
                uc |= READS_BUF;
            }
            if (ru & 16) != 0 {
                uc |= READS_TEX;
            }
            acc.uav_classes = uc;
            if uc == 0 && acc.uav_slots.is_none() {
                acc.uav_slots = Some(Vec::new());
            }

            let mut usc = uc;
            if (ru & (2 | 32)) != 0 {
                usc |= READS_BUF;
            }
            if (ru & 8) != 0 {
                usc |= READS_TEX;
            }
            acc.uav_srv_classes = usc;

            let mut rc = usc;
            if may_read_cbv_resource {
                rc |= READS_BUF;
            }
            acc.read_classes = rc;
            let ro_c = (if (ru & (2 | 32)) != 0 || may_read_cbv_resource { READS_BUF } else { 0 })
                | (if (ru & 8) != 0 { READS_TEX } else { 0 });
            acc.msc_exact_classes &= ro_c;

            let srv_ru_classes = (if (ru & (2 | 32)) != 0 { READS_BUF } else { 0 })
                | (if (ru & 8) != 0 { READS_TEX } else { 0 });
            acc.unbounded_srv_classes &= srv_ru_classes;
            if acc.unbounded_srv_classes == 0
                && !acc.heap_indexed_or_unknown
                && acc.read_slots.is_none()
            {
                acc.read_slots = Some(acc.bounded_read_slots.clone());
            }
            if (ru & (8 | 16)) == 0 {
                acc.touches_textures = false;
            }
        }
    }

    acc.msc_refined = (was_uav_none && acc.uav_slots.is_some())
        || (was_read_none && acc.read_slots.is_some())
        || (msc_applied
            && (acc.uav_slots != prev_uav_slots
                || acc.read_slots != prev_read_slots
                || acc.bounded_read_slots != prev_bounded_reads));
    acc
}

fn stage_srv_uav_classes_from_used(
    base: u8,
    used: &[MscUsedResource],
    rs: &RootSignature,
) -> u8 {
    let layout = rs.layout();
    let param_len = layout.parameters.len();
    let has_static_samplers = layout.static_samplers.is_some();
    let mut m = 0u8;
    for u in used {
        if !u.is_in_grs || u.binding_index > u16::MAX as u32 {
            return base;
        }
        let idx = u.binding_index as usize;
        if idx == param_len && has_static_samplers {
            continue;
        }
        if idx >= param_len {
            return base;
        }
        match &layout.parameters[idx].kind {
            RootParameterLayoutKind::Constants { .. } => {}
            RootParameterLayoutKind::Descriptor { kind, .. } => match kind {
                RootDescriptorKind::Cbv => {}
                RootDescriptorKind::Srv | RootDescriptorKind::Uav => m |= READS_BUF,
            },
            RootParameterLayoutKind::DescriptorTable { sampler: true, .. } => {}
            RootParameterLayoutKind::DescriptorTable { sampler: false, ranges } => {
                if u.table_length == 0 {
                    continue;
                }
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
                    let rg_end = rg
                        .num_descriptors
                        .map_or(u32::MAX, |n| rg.descriptor_offset.saturating_add(n));
                    if rg_end <= start || rg.descriptor_offset >= end {
                        continue;
                    }
                    any_overlap = true;
                    if matches!(rg.range_type, DescriptorRangeType::Srv | DescriptorRangeType::Uav)
                    {
                        m |= base;
                    }
                }
                if !any_overlap {
                    return base;
                }
            }
        }
    }
    base & m
}

fn shader_uses_uav_from_used(used: &[MscUsedResource], rs: &RootSignature) -> bool {
    let layout = rs.layout();
    let param_len = layout.parameters.len();
    let has_static_samplers = layout.static_samplers.is_some();
    for u in used {
        if !u.is_in_grs || u.binding_index > u16::MAX as u32 {
            return true;
        }
        let idx = u.binding_index as usize;
        if idx == param_len && has_static_samplers {
            continue;
        }
        if idx >= param_len {
            return true;
        }
        match &layout.parameters[idx].kind {
            RootParameterLayoutKind::Constants { .. } => {}
            RootParameterLayoutKind::Descriptor { kind, .. } => {
                if *kind == RootDescriptorKind::Uav {
                    return true;
                }
            }
            RootParameterLayoutKind::DescriptorTable { sampler: true, .. } => {}
            RootParameterLayoutKind::DescriptorTable { sampler: false, ranges } => {
                if u.table_length == 0 {
                    continue;
                }
                let (start, end) = if u.table_start_index == u32::MAX {
                    (0u32, u32::MAX)
                } else {
                    (
                        u.table_start_index,
                        u.table_start_index.saturating_add(u.table_length),
                    )
                };
                for rg in ranges {
                    let rg_end = rg
                        .num_descriptors
                        .map_or(u32::MAX, |n| rg.descriptor_offset.saturating_add(n));
                    if rg_end > start
                        && rg.descriptor_offset < end
                        && rg.range_type == DescriptorRangeType::Uav
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn psv_may_read_cbv_buffer(psv: &[PsvResource], rs: &RootSignature) -> bool {
    let layout = rs.layout();
    psv.iter().any(|r| {
        if r.res_type != 2 {
            return false;
        }
        if r.upper_bound != r.lower_bound || r.upper_bound == u32::MAX {
            return true;
        }
        let reg = r.lower_bound;
        let space = r.space;
        let has_inline_const = layout.parameters.iter().any(|p| {
            matches!(
                p.kind,
                RootParameterLayoutKind::Constants {
                    shader_register,
                    register_space,
                    ..
                } if register_space == space && shader_register == reg
            )
        });
        if !has_inline_const {
            return true;
        }
        layout.parameters.iter().any(|p| match &p.kind {
            RootParameterLayoutKind::Descriptor {
                kind: RootDescriptorKind::Cbv,
                shader_register,
                register_space,
            } => *register_space == space && *shader_register == reg,
            RootParameterLayoutKind::DescriptorTable { ranges, .. } => ranges.iter().any(|rg| {
                if rg.range_type != DescriptorRangeType::Cbv || rg.register_space != space {
                    return false;
                }
                let end = rg
                    .num_descriptors
                    .map_or(u32::MAX, |n| rg.base_shader_register.saturating_add(n));
                reg >= rg.base_shader_register && reg < end
            }),
            _ => false,
        })
    })
}

/// Pre-parsed DXBC/DXIL, `.metallib`, and MSC reflection metadata for one shader stage.
///
/// Computed once per shader blob during PSO creation so stage resource/CBV/UAV/handle-kind
/// queries never re-parse DXBC containers, re-run `dxilconv::convert_signed`, re-scan
/// `.metallib` FlatBuffers, or re-parse MSC reflection JSON.
pub(crate) struct ShaderStageSummary {
    raw_parsed: bool,
    raw_is_sm4: bool,
    raw_feature_flags: u64,
    raw_psv: Option<Vec<PsvResource>>,
    converted_dxil: Option<Vec<u8>>,
    converted_feature_flags: u64,
    converted_psv: Option<Vec<PsvResource>>,
    pub(crate) ru_opt: Option<u32>,
    pub(super) used_resources: Option<Vec<MscUsedResource>>,
    pub(crate) res_desc_heap_unused: bool,
    pub(super) msc_cbv_opt: Option<bool>,
}

impl ShaderStageSummary {
    pub(crate) fn new(bytecode: &[u8], sh: &CompiledShader, rs: &RootSignature) -> Self {
        let mut raw_parsed = false;
        let mut raw_is_sm4 = false;
        let mut raw_feature_flags = 0u64;
        let mut raw_psv = None;
        let mut converted_dxil = None;
        let mut converted_feature_flags = 0u64;
        let mut converted_psv = None;

        if let Ok(c) = dxil_metal::dxbc::Container::parse(bytecode) {
            raw_parsed = true;
            raw_is_sm4 = c.is_sm4_bytecode();
            raw_feature_flags = c.feature_flags();
            raw_psv = c.psv_resources();
            if raw_is_sm4 {
                if let Ok(cv) = dxil_metal::dxilconv::convert_signed(bytecode) {
                    if let Ok(cp) = dxil_metal::dxbc::Container::parse(&cv) {
                        converted_feature_flags = cp.feature_flags();
                        converted_psv = cp.psv_resources();
                        converted_dxil = Some(cv);
                    }
                }
            }
        }

        let ru_opt = msc_resource_usages(sh);
        let used_resources = msc_used_resources(sh.reflection.json.as_str());
        let res_desc_heap_unused = metallib_res_desc_heap_unused(&sh.metallib);
        let msc_cbv_opt = msc_stage_may_read_cbv_from_parts(
            res_desc_heap_unused,
            used_resources.as_deref(),
            rs,
        );

        Self {
            raw_parsed,
            raw_is_sm4,
            raw_feature_flags,
            raw_psv,
            converted_dxil,
            converted_feature_flags,
            converted_psv,
            ru_opt,
            used_resources,
            res_desc_heap_unused,
            msc_cbv_opt,
        }
    }

    #[inline]
    pub(super) fn dxil_parsed(&self) -> bool {
        if self.raw_is_sm4 { self.converted_dxil.is_some() } else { self.raw_parsed }
    }

    #[inline]
    pub(super) fn dxil_feature_flags(&self) -> u64 {
        if self.raw_is_sm4 { self.converted_feature_flags } else { self.raw_feature_flags }
    }

    #[inline]
    pub(super) fn dxil_psv(&self) -> Option<&[PsvResource]> {
        if self.raw_is_sm4 { self.converted_psv.as_deref() } else { self.raw_psv.as_deref() }
    }

    #[inline]
    pub(super) fn dxil_bytes<'a>(&'a self, orig: &'a [u8]) -> Option<&'a [u8]> {
        if self.raw_is_sm4 {
            self.converted_dxil.as_deref()
        } else if self.raw_parsed {
            Some(orig)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn msc_proven(&self) -> bool {
        self.res_desc_heap_unused && self.ru_opt.is_some()
    }

    pub(crate) fn stage_reads_resources(&self, sh: &CompiledShader, opt_v2: bool) -> u8 {
        const BOTH: u8 = READS_TEX | READS_BUF;
        if opt_v2 {
            if let Some(ru) = self.ru_opt {
                let mut m = 0u8;
                if (ru & (2 | 4 | 32)) != 0 {
                    m |= READS_BUF;
                }
                if (ru & (8 | 16)) != 0 {
                    m |= READS_TEX;
                }
                return m;
            }
        }
        if !self.raw_parsed || (self.raw_feature_flags & RESOURCE_HEAP_INDEXING) != 0 {
            return BOTH;
        }
        let reflected =
            |f: fn(&T) -> bool| sh.reflection.resources.iter().any(|r| f(&r.resource_type));
        if !reflected(|t| matches!(t, T::Srv | T::Uav | T::Table | T::Invalid)) {
            return 0;
        }
        let Some(res) = self.raw_psv.as_deref() else { return BOTH };
        let m = res.iter().filter(|r| r.is_srv_uav()).fold(0, |m, r| {
            m | if r.is_texture() {
                READS_TEX
            } else if r.is_buffer() {
                READS_BUF
            } else {
                BOTH
            }
        });
        if m == 0 && reflected(|t| matches!(t, T::Srv | T::Uav)) {
            return BOTH;
        }
        m
    }

    pub(crate) fn stage_srv_uav_classes(
        &self,
        sh: &CompiledShader,
        rs: &RootSignature,
        opt_v2: bool,
    ) -> u8 {
        let base = self.stage_reads_resources(sh, opt_v2);
        if base == 0 || !opt_v2 || !self.res_desc_heap_unused {
            return base;
        }
        let Some(used) = self.used_resources.as_deref() else {
            return base;
        };
        stage_srv_uav_classes_from_used(base, used, rs)
    }

    pub(crate) fn stage_may_read_cbv(&self, rs: &RootSignature, opt_sm66: bool) -> bool {
        let msc_cbv = if opt_sm66 { self.msc_cbv_opt } else { None };
        if msc_cbv == Some(false) {
            return false;
        }
        if !self.dxil_parsed() {
            return true;
        }
        if (self.dxil_feature_flags() & RESOURCE_HEAP_INDEXING) != 0 {
            return msc_cbv.unwrap_or(true);
        }
        let Some(psv) = self.dxil_psv() else {
            return true;
        };
        psv_may_read_cbv_buffer(psv, rs)
    }

    #[inline]
    fn raw_writes_uav(&self) -> bool {
        if !self.raw_parsed || (self.raw_feature_flags & RESOURCE_HEAP_INDEXING) != 0 {
            return true;
        }
        self.raw_psv
            .as_deref()
            .is_none_or(|res| res.iter().any(|r| (6..=9).contains(&r.res_type)))
    }

    #[inline]
    pub(crate) fn shader_writes_uav(&self, opt_v2: bool) -> bool {
        if opt_v2 {
            if let Some(ru) = self.ru_opt {
                return (ru & (4 | 16)) != 0;
            }
        }
        self.raw_writes_uav()
    }

    #[inline]
    pub(crate) fn shader_writes_buf_uav(&self) -> bool {
        if let Some(ru) = self.ru_opt {
            return (ru & 4) != 0;
        }
        self.raw_writes_uav()
    }

    pub(crate) fn shader_uses_uav(&self, rs: &RootSignature) -> bool {
        if self.raw_parsed && (self.raw_feature_flags & RESOURCE_HEAP_INDEXING) == 0 {
            if let Some(psv) = self.raw_psv.as_deref() {
                return psv.iter().any(|r| (6..=9).contains(&r.res_type));
            }
        }
        if !self.res_desc_heap_unused {
            return true;
        }
        if let Some(ru) = self.ru_opt {
            if (ru & (4 | 16)) != 0 {
                return true;
            }
        }
        let Some(used) = self.used_resources.as_deref() else {
            return true;
        };
        shader_uses_uav_from_used(used, rs)
    }

    fn exact_classes(&self, ru: u32, fallback_has_cbv: bool) -> u8 {
        if (ru & !0x3e) != 0 {
            return READS_TEX | READS_BUF;
        }
        let has_cbv = self.msc_cbv_opt.unwrap_or(fallback_has_cbv);
        let mut ec = 0u8;
        if (ru & (2 | 32)) != 0 || has_cbv {
            ec |= READS_BUF;
        }
        if (ru & 8) != 0 {
            ec |= READS_TEX;
        }
        ec
    }

    fn compute_access_from_dxil(&self, rs: &RootSignature) -> ComputeAccess {
        compute_access_from_parts(
            self.dxil_parsed(),
            self.dxil_feature_flags(),
            self.dxil_psv(),
            rs,
        )
    }

    fn refine_compute_access(
        &self,
        acc: ComputeAccess,
        rs: &RootSignature,
        enable_used_resources: bool,
        ignore_heap_flag: bool,
    ) -> ComputeAccess {
        let heap_indexed =
            !self.dxil_parsed() || (self.dxil_feature_flags() & RESOURCE_HEAP_INDEXING) != 0;
        refine_compute_access_from_parts(
            acc,
            heap_indexed,
            self.dxil_psv(),
            self.used_resources.as_deref(),
            self.ru_opt,
            rs,
            enable_used_resources,
            ignore_heap_flag,
        )
    }

    pub(crate) fn for_stage(
        &self,
        bytecode: &[u8],
        rs: &RootSignature,
        shader: &CompiledShader,
        allow_ia_buf_ru: bool,
    ) -> (ComputeAccess, HandleKindProfile) {
        let mut acc = self.compute_access_from_dxil(rs);
        if let Some(ru) = self.ru_opt {
            if (ru & !0x3e) == 0 {
                if (ru & (8 | 16)) == 0 {
                    acc.touches_textures = false;
                    acc.unbounded_srv_classes &= !READS_TEX;
                }
                acc.msc_exact_classes = self.exact_classes(ru, self.stage_may_read_cbv(rs, false));
            }
        }
        let air_unused = self.res_desc_heap_unused
            && self.used_resources.as_deref().is_some_and(|u| {
                !u.is_empty()
                    || self.ru_opt == Some(0)
                    || (allow_ia_buf_ru && self.ru_opt.is_some_and(|ru| (ru & !2) == 0))
            });
        let out = self.refine_compute_access(acc, rs, true, air_unused);
        let raw_hk = resolve_shader_hk_profile_from_summary(self, bytecode, rs, shader);
        let hk = raw_hk.intersect_access(&out);
        (out, hk)
    }

    pub(crate) fn for_compute(
        &self,
        bytecode: &[u8],
        rs: &RootSignature,
        shader: &CompiledShader,
    ) -> (ComputeAccess, HandleKindProfile) {
        const BOTH: u8 = READS_TEX | READS_BUF;
        let mid_v2 = cs_mid_qb_exact_v2();
        let sm66_cs = sm66_cs_stage_exact();
        let opt_msc_access = msc_compute_access() || sm66_cs;
        let opt_stage_usage = msc_stage_usage() || mid_v2 || sm66_cs;
        let opt_air_no_res_heap = air_no_res_heap() || mid_v2 || sm66_cs;

        let mut acc = self.compute_access_from_dxil(rs);
        if let Some(ru) = self.ru_opt {
            if (ru & !0x3e) == 0 {
                if (ru & (8 | 16)) == 0 {
                    acc.touches_textures = false;
                    acc.unbounded_srv_classes &= !READS_TEX;
                }
                acc.msc_exact_classes = self.exact_classes(ru, self.stage_may_read_cbv(rs, false));
            }
        }
        let air_unused = opt_air_no_res_heap
            && self.res_desc_heap_unused
            && self
                .used_resources
                .as_deref()
                .is_some_and(|u| !u.is_empty() || self.ru_opt == Some(0));
        if air_unused {
            crate::metal::encstats::inc(&crate::metal::encstats::c::air_no_res_heap_pso);
        }
        let out = if !opt_msc_access && !opt_stage_usage && !air_unused {
            acc
        } else {
            let enable_used_resources = opt_msc_access || air_unused;
            let refined = self.refine_compute_access(acc, rs, enable_used_resources, air_unused);
            if opt_stage_usage
                && (refined.uav_classes != BOTH
                    || refined.read_classes != BOTH
                    || refined.uav_srv_classes != BOTH)
            {
                crate::metal::encstats::inc(&crate::metal::encstats::c::msc_stage_usage_hit);
            }
            refined
        };
        let raw_hk = resolve_shader_hk_profile_from_summary(self, bytecode, rs, shader);
        let hk = raw_hk.intersect_access(&out);
        (out, hk)
    }
}

/// A statically known compute root binding slot (resolved at `dispatch()` time
/// from `RootState` + the bound `CBV_SRV_UAV` descriptor heap).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputeBind {
    /// Root CBV / SRV / UAV descriptor (`D3D12_GPU_VIRTUAL_ADDRESS` in root param `param`).
    RootVa { param: u16 },
    /// Descriptor-table entry (`IRDescriptorTableEntry` at `table_gpu_va + byte_offset`).
    TableEntry { param: u16, byte_offset: u64 },
}

/// Statically reflected resource access profile of a compute shader (`PSV0` + `RootSignatureLayout`
/// + optional MSC `"UsedResources"` / `"ResourceUsages"` refinement under `AW2_MSC_COMPUTE_ACCESS=1`).
#[derive(Clone, Debug)]
pub struct ComputeAccess {
    /// `Some(slots)` when all UAV bindings are bounded and `RESOURCE_HEAP_INDEXING` is off;
    /// `None` when opaque (unparsable, SM 6.6 heap indexing, or unbounded UAV range).
    pub uav_slots: Option<Vec<ComputeBind>>,
    /// `Some(slots)` when all CBV + SRV bindings are bounded and `RESOURCE_HEAP_INDEXING` is off;
    /// `None` when opaque (e.g. unbounded bindless SRV table).
    pub read_slots: Option<Vec<ComputeBind>>,
    /// Bounded CBV + SRV slots even when `read_slots` is `None` because of an additional
    /// unbounded SRV range (used when `AW2_IB_BINDLESS_WAR=1`).
    pub bounded_read_slots: Vec<ComputeBind>,
    /// Resource classes (`READS_TEX` / `READS_BUF`) referenced by unbounded or large SRV ranges
    /// (`0` when `read_slots` is `Some`, `READS_TEX | READS_BUF` when opaque).
    pub unbounded_srv_classes: u8,
    /// True when `RESOURCE_HEAP_INDEXING` (`SFI0` bit `1 << 25`) or missing `PSV0` makes
    /// even `bounded_read_slots` incomplete for non-bindless-table shaders.
    pub heap_indexed_or_unknown: bool,
    /// Whether the compute shader may read or write any texture (`SRV` `Texture*` or `UAV` `RWTexture*`).
    /// Proven `false` for buffer-only SM 6.6 compute shaders (e.g. `skinning:transformCS`, `RU = 6`)
    /// via MSC's `"ResourceUsages"` reflection bitmask (`(ru & 24) == 0`).
    pub touches_textures: bool,
    /// Resource classes (`READS_TEX` / `READS_BUF`) that the compute shader may write via UAVs.
    pub uav_classes: u8,
    /// Resource classes (`READS_TEX` / `READS_BUF`) that the compute shader may read or write
    /// (including `READS_BUF` whenever any non-inline CBV resource may be read).
    pub read_classes: u8,
    /// Resource classes (`READS_TEX` / `READS_BUF`) that the compute shader may access via
    /// SRVs, UAVs, or AccelerationStructures (excluding CBVs, which cannot alias a resource
    /// remaining in `D3D12_RESOURCE_STATE_UNORDERED_ACCESS` across `Barrier::Uav`).
    pub uav_srv_classes: u8,
    /// True when `AW2_MSC_COMPUTE_ACCESS=1` or `AW2_AIR_NO_RES_HEAP=1` upgraded `uav_slots`
    /// or `read_slots` from `None` to `Some`.
    pub msc_refined: bool,
    /// Unconditional MSC `"ResourceUsages"` class mask (`uav_classes | read_classes`, with
    /// `READS_BUF` preserved if CBV resources may be read) for `AW2_CS_MID_QB_EXACT_V2=1`.
    pub msc_exact_classes: u8,
}

impl ComputeAccess {
    pub fn opaque() -> Self {
        Self {
            uav_slots: None,
            read_slots: None,
            bounded_read_slots: Vec::new(),
            unbounded_srv_classes: READS_TEX | READS_BUF,
            heap_indexed_or_unknown: true,
            touches_textures: true,
            uav_classes: READS_TEX | READS_BUF,
            read_classes: READS_TEX | READS_BUF,
            uav_srv_classes: READS_TEX | READS_BUF,
            msc_refined: false,
            msc_exact_classes: READS_TEX | READS_BUF,
        }
    }

    pub fn from_dxil_and_shader(dxil: &[u8], rs: &RootSignature, shader: &CompiledShader) -> Self {
        let mid_v2 = cs_mid_qb_exact_v2();
        let sm66_cs = sm66_cs_stage_exact();
        Self::from_dxil_and_shader_with_opts(
            dxil,
            rs,
            shader,
            msc_compute_access() || sm66_cs,
            msc_stage_usage() || mid_v2 || sm66_cs,
            air_no_res_heap() || mid_v2 || sm66_cs,
        )
    }

    /// Zero-salt stage binding profile (`VS`/`MS`/`AS`/`PS`) for `AW2_GFX_DESC_TABLE_EXACT=1`.
    /// When `allow_ia_buf_ru` is `true` (vertex stage), an empty `"UsedResources"` list is
    /// accepted alongside `"ResourceUsages" == 2` (`BufferRead` from stage-in IA / draw params).
    pub fn for_stage_shader(
        dxil: &[u8],
        rs: &RootSignature,
        shader: &CompiledShader,
        allow_ia_buf_ru: bool,
    ) -> Self {
        let mut acc = Self::from_dxil(dxil, rs);
        let ru_opt = msc_resource_usages(shader);
        if let Some(ru) = ru_opt {
            if (ru & !0x3e) == 0 {
                if (ru & (8 | 16)) == 0 {
                    acc.touches_textures = false;
                    acc.unbounded_srv_classes &= !READS_TEX;
                }
                acc.msc_exact_classes = Self::exact_classes_from_metallib(
                    &shader.metallib,
                    shader.reflection.json.as_str(),
                    ru,
                    stage_may_read_cbv_buffer_with(shader, dxil, rs, false),
                    rs,
                );
            }
        }
        let air_unused = metallib_res_desc_heap_unused(&shader.metallib)
            && msc_used_resources(shader.reflection.json.as_str()).is_some_and(|u| {
                !u.is_empty()
                    || ru_opt == Some(0)
                    || (allow_ia_buf_ru && ru_opt.is_some_and(|ru| (ru & !2) == 0))
            });
        let out = Self::refine_with_msc_full(
            acc,
            dxil,
            rs,
            shader.reflection.json.as_str(),
            ru_opt,
            true,
            air_unused,
        );
        register_shader_hk_profile(dxil, rs, shader, ru_opt);
        out
    }

    pub fn from_dxil_and_shader_with(
        dxil: &[u8],
        rs: &RootSignature,
        shader: &CompiledShader,
        opt_msc_access: bool,
    ) -> Self {
        Self::from_dxil_and_shader_with_opts(dxil, rs, shader, opt_msc_access, false, false)
    }

    /// Computes exact non-UAV read (`READS_TEX | READS_BUF` SRV/AS/CBV) classes from MSC
    /// `"ResourceUsages"` plus `msc_stage_may_read_cbv_from_reflection` (falling back to
    /// `fallback_has_cbv` whenever `metallib_res_desc_heap_unused` is false). UAV write
    /// classes are tracked separately in `uav_classes`.
    pub fn exact_classes_from_metallib(
        metallib: &[u8],
        msc_json: &str,
        ru: u32,
        fallback_has_cbv: bool,
        rs: &RootSignature,
    ) -> u8 {
        if (ru & !0x3e) != 0 {
            return READS_TEX | READS_BUF;
        }
        let has_cbv = msc_stage_may_read_cbv_from_reflection(metallib, msc_json, rs)
            .unwrap_or(fallback_has_cbv);
        let mut ec = 0u8;
        if (ru & (2 | 32)) != 0 || has_cbv {
            ec |= READS_BUF;
        }
        if (ru & 8) != 0 {
            ec |= READS_TEX;
        }
        ec
    }

    pub fn from_dxil_and_shader_with_opts(
        dxil: &[u8],
        rs: &RootSignature,
        shader: &CompiledShader,
        opt_msc_access: bool,
        opt_stage_usage: bool,
        opt_air_no_res_heap: bool,
    ) -> Self {
        const BOTH: u8 = READS_TEX | READS_BUF;
        let mut acc = Self::from_dxil(dxil, rs);
        let ru_opt = msc_resource_usages(shader);
        if let Some(ru) = ru_opt {
            if (ru & !0x3e) == 0 {
                if (ru & (8 | 16)) == 0 {
                    acc.touches_textures = false;
                    acc.unbounded_srv_classes &= !READS_TEX;
                }
                acc.msc_exact_classes = Self::exact_classes_from_metallib(
                    &shader.metallib,
                    shader.reflection.json.as_str(),
                    ru,
                    stage_may_read_cbv_buffer_with(shader, dxil, rs, false),
                    rs,
                );
            }
        }
        let air_unused = opt_air_no_res_heap
            && metallib_res_desc_heap_unused(&shader.metallib)
            && msc_used_resources(shader.reflection.json.as_str())
                .is_some_and(|u| !u.is_empty() || ru_opt == Some(0));
        if air_unused {
            crate::metal::encstats::inc(&crate::metal::encstats::c::air_no_res_heap_pso);
        }
        if !opt_msc_access && !opt_stage_usage && !air_unused {
            register_shader_hk_profile(dxil, rs, shader, ru_opt);
            return acc;
        }
        let enable_used_resources = opt_msc_access || air_unused;
        let out = Self::refine_with_msc_full(
            acc,
            dxil,
            rs,
            shader.reflection.json.as_str(),
            ru_opt,
            enable_used_resources,
            air_unused,
        );
        if opt_stage_usage
            && (out.uav_classes != BOTH
                || out.read_classes != BOTH
                || out.uav_srv_classes != BOTH)
        {
            crate::metal::encstats::inc(&crate::metal::encstats::c::msc_stage_usage_hit);
        }
        register_shader_hk_profile(dxil, rs, shader, ru_opt);
        out
    }

    /// Refines `acc` using MSC's `"UsedResources"` and `"ResourceUsages"` reflection JSON.
    pub fn refine_with_msc(
        acc: Self,
        dxil: &[u8],
        rs: &RootSignature,
        msc_json: &str,
        ru_opt: Option<u32>,
    ) -> Self {
        Self::refine_with_msc_full(acc, dxil, rs, msc_json, ru_opt, true, false)
    }

    /// Like [`Self::refine_with_msc`], with explicit `enable_used_resources` and
    /// `ignore_heap_flag` (`AW2_AIR_NO_RES_HEAP=1` when `struct.res_desc_heap_ab` is unused).
    pub fn refine_with_msc_full(
        acc: Self,
        dxil: &[u8],
        rs: &RootSignature,
        msc_json: &str,
        ru_opt: Option<u32>,
        enable_used_resources: bool,
        ignore_heap_flag: bool,
    ) -> Self {
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
        let heap_indexed = container_opt
            .as_ref()
            .is_none_or(|c| (c.feature_flags() & RESOURCE_HEAP_INDEXING) != 0);
        let psv_opt = container_opt.as_ref().and_then(|c| c.psv_resources());
        let used_opt = if enable_used_resources && !(heap_indexed && !ignore_heap_flag) {
            msc_used_resources(msc_json)
        } else {
            None
        };
        refine_compute_access_from_parts(
            acc,
            heap_indexed,
            psv_opt.as_deref(),
            used_opt.as_deref(),
            ru_opt,
            rs,
            enable_used_resources,
            ignore_heap_flag,
        )
    }

    pub fn from_dxil(dxil: &[u8], rs: &RootSignature) -> Self {
        let Ok(parsed) = dxil_metal::dxbc::Container::parse(dxil) else {
            return Self::opaque();
        };
        let converted;
        let c = if parsed.is_sm4_bytecode() {
            let Ok(cv) = dxil_metal::dxilconv::convert_signed(dxil) else {
                return Self::opaque();
            };
            converted = cv;
            let Ok(cp) = dxil_metal::dxbc::Container::parse(&converted) else {
                return Self::opaque();
            };
            cp
        } else {
            parsed
        };
        let psv = c.psv_resources();
        compute_access_from_parts(true, c.feature_flags(), psv.as_deref(), rs)
    }
}

fn msc_stage_may_read_cbv_from_parts(
    res_desc_heap_unused: bool,
    used: Option<&[MscUsedResource]>,
    rs: &RootSignature,
) -> Option<bool> {
    if !res_desc_heap_unused {
        return None;
    }
    let used = used?;
    let layout = rs.layout();
    let param_len = layout.parameters.len();
    let has_static_samplers = layout.static_samplers.is_some();
    let mut has_cbv = false;
    for u in used {
        if !u.is_in_grs || u.binding_index > u16::MAX as u32 {
            return None;
        }
        let idx = u.binding_index as usize;
        if idx == param_len && has_static_samplers {
            continue;
        }
        if idx >= param_len {
            return None;
        }
        match &layout.parameters[idx].kind {
            RootParameterLayoutKind::Constants { .. } => {}
            RootParameterLayoutKind::Descriptor { kind, .. } => {
                if *kind == RootDescriptorKind::Cbv {
                    has_cbv = true;
                }
            }
            RootParameterLayoutKind::DescriptorTable { sampler: true, .. } => {}
            RootParameterLayoutKind::DescriptorTable { sampler: false, ranges } => {
                if u.table_length == 0 {
                    continue;
                }
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
                    let rg_end = rg
                        .num_descriptors
                        .map_or(u32::MAX, |n| rg.descriptor_offset.saturating_add(n));
                    if rg_end <= start || rg.descriptor_offset >= end {
                        continue;
                    }
                    any_overlap = true;
                    if rg.range_type == DescriptorRangeType::Cbv {
                        has_cbv = true;
                    }
                }
                if !any_overlap {
                    return None;
                }
            }
        }
    }
    Some(has_cbv)
}

/// Inspects an embedded `.metallib` (`RBUF` `struct.res_desc_heap_ab` `unused == 1`) plus
/// MSC's `"UsedResources"` reflection JSON against `rs` to determine whether a SM 6.6 shader
/// stage may read any non-inline CBV buffer resource (root CBV descriptor or CBV table range).
/// Returns `None` if `metallib_res_desc_heap_unused` is `false` or `"UsedResources"` is absent
/// or outside GRS bounds (fail-closed: caller treats `None` as `true`).
pub fn msc_stage_may_read_cbv_from_reflection(
    metallib: &[u8],
    msc_json: &str,
    rs: &RootSignature,
) -> Option<bool> {
    if !metallib_res_desc_heap_unused(metallib) {
        return None;
    }
    let used = msc_used_resources(msc_json)?;
    msc_stage_may_read_cbv_from_parts(true, Some(&used), rs)
}

/// Whether a shader stage may read any GPU buffer via a root CBV descriptor,
/// CBV descriptor table, or SM 6.6 `RESOURCE_HEAP_INDEXING` (`1 << 25`).
/// When `opt_sm66` is `true`, both SM 6.5 and SM 6.6 stages whose embedded
/// `.metallib` proves `struct.res_desc_heap_ab` is unused (`100%` of AW2 VS/MS/AS/PS)
/// resolve dead-code-eliminated CBVs via [`msc_stage_may_read_cbv_from_reflection`].
pub fn stage_may_read_cbv_buffer_with(
    sh: &CompiledShader,
    bytecode: &[u8],
    rs: &RootSignature,
    opt_sm66: bool,
) -> bool {
    let msc_cbv = if opt_sm66 {
        msc_stage_may_read_cbv_from_reflection(&sh.metallib, sh.reflection.json.as_str(), rs)
    } else {
        None
    };
    if msc_cbv == Some(false) {
        return false;
    }
    let Ok(parsed) = dxil_metal::dxbc::Container::parse(bytecode) else {
        return true;
    };
    let converted;
    let c = if parsed.is_sm4_bytecode() {
        let Ok(cv) = dxil_metal::dxilconv::convert_signed(bytecode) else {
            return true;
        };
        converted = cv;
        let Ok(cp) = dxil_metal::dxbc::Container::parse(&converted) else {
            return true;
        };
        cp
    } else {
        parsed
    };
    if c.feature_flags() & RESOURCE_HEAP_INDEXING != 0 {
        return msc_cbv.unwrap_or(true);
    }
    let Some(psv) = c.psv_resources() else {
        return true;
    };
    psv_may_read_cbv_buffer(&psv, rs)
}

/// Whether a vertex shader may read any GPU buffer via a root CBV descriptor,
/// CBV descriptor table, or SM 6.6 `RESOURCE_HEAP_INDEXING` (`1 << 25`).
/// Pure `RootParameterKind::Constants` (32-bit inline root constants uploaded
/// into the internal `UploadRing` TLAB) do not reference D3D12 `Resource` buffers,
/// provided no root CBV descriptor or CBV table overlaps the same `(space, reg)`.
pub fn vs_may_read_cbv_buffer(sh: &CompiledShader, bytecode: &[u8], rs: &RootSignature) -> bool {
    stage_may_read_cbv_buffer_with(sh, bytecode, rs, false)
}

/// Whether a shader stage may write through UAVs (unparsable bytecode and
/// heap indexing count as writing).
pub fn stage_writes_uav(bytecode: &[u8]) -> bool {
    let Ok(c) = dxil_metal::dxbc::Container::parse(bytecode) else { return true };
    if c.feature_flags() & RESOURCE_HEAP_INDEXING != 0 {
        return true;
    }
    c.psv_resources().is_none_or(|res| res.iter().any(|r| (6..=9).contains(&r.res_type)))
}

/// Like [`stage_writes_uav`], consulting MSC's `"ResourceUsages"` reflection bitmask
/// (`BufferWrite = 4`, `TextureWrite = 16`) when `opt_v2` is active so SM 6.6
/// `RESOURCE_HEAP_INDEXING` shaders without UAV stores are proven UAV-free.
pub fn shader_writes_uav(sh: &CompiledShader, bytecode: &[u8], opt_v2: bool) -> bool {
    if opt_v2 {
        if let Some(ru) = msc_resource_usages(sh) {
            return (ru & (4 | 16)) != 0;
        }
    }
    stage_writes_uav(bytecode)
}

/// Whether a shader stage may write to a **buffer** UAV (`BufferWrite = 4` in MSC
/// `"ResourceUsages"`, falling back to `stage_writes_uav`).
pub fn shader_writes_buf_uav(sh: &CompiledShader, bytecode: &[u8]) -> bool {
    if let Some(ru) = msc_resource_usages(sh) {
        return (ru & 4) != 0;
    }
    stage_writes_uav(bytecode)
}

/// Whether a shader stage may read or write any UAV (`u*` root descriptor, UAV table
/// range, or UAV heap index).
pub fn shader_uses_uav(sh: &CompiledShader, bytecode: &[u8], rs: &RootSignature) -> bool {
    if let Ok(c) = dxil_metal::dxbc::Container::parse(bytecode) {
        if c.feature_flags() & RESOURCE_HEAP_INDEXING == 0 {
            if let Some(psv) = c.psv_resources() {
                return psv.iter().any(|r| (6..=9).contains(&r.res_type));
            }
        }
    }
    if !metallib_res_desc_heap_unused(&sh.metallib) {
        return true;
    }
    if let Some(ru) = msc_resource_usages(sh) {
        if (ru & (4 | 16)) != 0 {
            return true;
        }
    }
    let Some(used) = msc_used_resources(sh.reflection.json.as_str()) else {
        return true;
    };
    shader_uses_uav_from_used(&used, rs)
}

/// Which resource classes ([`READS_TEX`] / [`READS_BUF`]) a shader stage may
/// read through SRVs/UAVs (root views, descriptor tables or SM 6.6 heap
/// indexing). Unparsable bytecode, heap indexing and resources of unknown
/// kind count as reading both.
pub fn stage_reads_resources(sh: &CompiledShader, bytecode: &[u8]) -> u8 {
    stage_reads_resources_with(sh, bytecode, false)
}

/// Like [`stage_reads_resources`], consulting MSC's `"ResourceUsages"` reflection
/// bitmask (`BufferRead | BufferWrite | AS = 38`, `TextureRead | TextureWrite = 24`)
/// when `opt_v2` is active so SM 6.6 `RESOURCE_HEAP_INDEXING` vertex/mesh/object
/// stages that never access textures omit `READS_TEX`.
pub fn stage_reads_resources_with(sh: &CompiledShader, bytecode: &[u8], opt_v2: bool) -> u8 {
    const BOTH: u8 = READS_TEX | READS_BUF;
    if opt_v2 {
        if let Some(ru) = msc_resource_usages(sh) {
            let mut m = 0u8;
            if (ru & (2 | 4 | 32)) != 0 {
                m |= READS_BUF;
            }
            if (ru & (8 | 16)) != 0 {
                m |= READS_TEX;
            }
            return m;
        }
    }
    let Ok(c) = dxil_metal::dxbc::Container::parse(bytecode) else { return BOTH };
    if c.feature_flags() & RESOURCE_HEAP_INDEXING != 0 {
        return BOTH;
    }
    let reflected = |f: fn(&T) -> bool| sh.reflection.resources.iter().any(|r| f(&r.resource_type));
    if !reflected(|t| matches!(t, T::Srv | T::Uav | T::Table | T::Invalid)) {
        return 0;
    }
    let Some(res) = c.psv_resources() else { return BOTH };
    let m = res.iter().filter(|r| r.is_srv_uav()).fold(0, |m, r| {
        m | if r.is_texture() {
            READS_TEX
        } else if r.is_buffer() {
            READS_BUF
        } else {
            BOTH
        }
    });
    // PSV0 and the reflection disagree: stay conservative.
    if m == 0 && reflected(|t| matches!(t, T::Srv | T::Uav)) {
        return BOTH;
    }
    m
}

/// Like [`stage_reads_resources_with`], excluding CBV-only and inline-constant /
/// `RuntimeDrawParams` `BufferRead` (`ru & 2`) bits when `opt_v2` is active and
/// [`metallib_res_desc_heap_unused`] plus MSC's `"UsedResources"` prove that no
/// SRV or UAV buffer/table resource is referenced by this shader stage.
pub fn stage_srv_uav_classes(
    sh: &CompiledShader,
    bytecode: &[u8],
    rs: &RootSignature,
    opt_v2: bool,
) -> u8 {
    let base = stage_reads_resources_with(sh, bytecode, opt_v2);
    if base == 0 || !opt_v2 || !metallib_res_desc_heap_unused(&sh.metallib) {
        return base;
    }
    let Some(used) = msc_used_resources(sh.reflection.json.as_str()) else {
        return base;
    };
    stage_srv_uav_classes_from_used(base, &used, rs)
}
