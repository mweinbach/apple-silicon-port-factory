//! In-memory D3D12 root signature descriptions (`D3D12_ROOT_SIGNATURE_DESC`,
//! `D3D12_VERSIONED_ROOT_SIGNATURE_DESC`) → `dxil_metal::RootSignatureDesc`,
//! for `D3D12SerializeRootSignature` / `D3D12SerializeVersionedRootSignature`.
//!
//! x64 layouts (pointers 8 bytes, unions 8-aligned):
//! * `ROOT_SIGNATURE_DESC{,1,2}`: NumParameters @0, pParameters @8,
//!   NumStaticSamplers @16, pStaticSamplers @24, Flags @32 (40 bytes).
//! * `ROOT_PARAMETER{,1}`: ParameterType @0, union @8, ShaderVisibility @24
//!   (32 bytes). Table = {NumDescriptorRanges @8, pDescriptorRanges @16};
//!   constants = {reg, space, num}; descriptor = {reg, space(, flags)}.
//! * `DESCRIPTOR_RANGE` 20 bytes / `DESCRIPTOR_RANGE1` 24 bytes (+Flags @16).
//! * `STATIC_SAMPLER_DESC` 52 bytes / `STATIC_SAMPLER_DESC1` 56 (+Flags).
//! * `VERSIONED_ROOT_SIGNATURE_DESC`: Version @0, desc @8.

use dxil_metal::root_signature::*;

unsafe fn u32_at(p: *const u8, off: usize) -> u32 {
    std::ptr::read_unaligned(p.add(off) as *const u32)
}
unsafe fn f32_at(p: *const u8, off: usize) -> f32 {
    std::ptr::read_unaligned(p.add(off) as *const f32)
}
unsafe fn ptr_at(p: *const u8, off: usize) -> *const u8 {
    std::ptr::read_unaligned(p.add(off) as *const usize) as *const u8
}

/// Parse a `D3D12_ROOT_SIGNATURE_DESC{,1,2}` at `p` of version `version`.
///
/// # Safety
/// `p` must point at a valid description of that version.
pub unsafe fn desc_from_mem(version: RootSignatureVersion, p: *const u8) -> Result<RootSignatureDesc, String> {
    if p.is_null() {
        return Err("null root signature desc".into());
    }
    let v10 = version == RootSignatureVersion::V1_0;
    let np = u32_at(p, 0) as usize;
    let params = ptr_at(p, 8);
    let ns = u32_at(p, 16) as usize;
    let samplers = ptr_at(p, 24);
    let flags = u32_at(p, 32);
    if (np > 0 && params.is_null()) || (ns > 0 && samplers.is_null()) {
        return Err("null parameter/sampler array".into());
    }
    let mut parameters = Vec::with_capacity(np);
    for i in 0..np {
        let rp = params.add(i * 32);
        let ty = u32_at(rp, 0);
        let visibility = u32_at(rp, 24);
        let kind = match ty {
            0 => {
                let nr = u32_at(rp, 8) as usize;
                let ranges = ptr_at(rp, 16);
                if nr > 0 && ranges.is_null() {
                    return Err(format!("parameter {i}: null ranges"));
                }
                let stride = if v10 { 20 } else { 24 };
                let mut v = Vec::with_capacity(nr);
                for r in 0..nr {
                    let d = ranges.add(r * stride);
                    let rt = u32_at(d, 0);
                    let range_type = DescriptorRangeType::from_raw(rt).ok_or_else(|| format!("parameter {i} range {r}: type {rt}"))?;
                    let (rflags, off) = if v10 {
                        let def = if range_type == DescriptorRangeType::Sampler {
                            flags::RANGE_V1_0_DEFAULT_SAMPLER
                        } else {
                            flags::RANGE_V1_0_DEFAULT_CBV_SRV_UAV
                        };
                        (def, u32_at(d, 16))
                    } else {
                        (u32_at(d, 16), u32_at(d, 20))
                    };
                    v.push(DescriptorRange {
                        range_type,
                        num_descriptors: u32_at(d, 4),
                        base_shader_register: u32_at(d, 8),
                        register_space: u32_at(d, 12),
                        flags: rflags,
                        offset_in_descriptors_from_table_start: off,
                    });
                }
                RootParameterKind::DescriptorTable(v)
            }
            1 => RootParameterKind::Constants { shader_register: u32_at(rp, 8), register_space: u32_at(rp, 12), num_32bit_values: u32_at(rp, 16) },
            2..=4 => {
                let k = match ty {
                    2 => RootDescriptorKind::Cbv,
                    3 => RootDescriptorKind::Srv,
                    _ => RootDescriptorKind::Uav,
                };
                let f = if v10 { flags::ROOT_DESCRIPTOR_V1_0_DEFAULT } else { u32_at(rp, 16) };
                RootParameterKind::Descriptor(k, RootDescriptor { shader_register: u32_at(rp, 8), register_space: u32_at(rp, 12), flags: f })
            }
            _ => return Err(format!("parameter {i}: type {ty}")),
        };
        parameters.push(RootParameter { kind, visibility });
    }
    let sstride = if version == RootSignatureVersion::V1_2 { 56 } else { 52 };
    let mut static_samplers = Vec::with_capacity(ns);
    for i in 0..ns {
        let s = samplers.add(i * sstride);
        static_samplers.push(StaticSampler {
            filter: u32_at(s, 0),
            address_u: u32_at(s, 4),
            address_v: u32_at(s, 8),
            address_w: u32_at(s, 12),
            mip_lod_bias: f32_at(s, 16),
            max_anisotropy: u32_at(s, 20),
            comparison_func: u32_at(s, 24),
            border_color: u32_at(s, 28),
            min_lod: f32_at(s, 32),
            max_lod: f32_at(s, 36),
            shader_register: u32_at(s, 40),
            register_space: u32_at(s, 44),
            visibility: u32_at(s, 48),
            flags: if sstride == 56 { u32_at(s, 52) } else { 0 },
        });
    }
    Ok(RootSignatureDesc { version, parameters, static_samplers, flags })
}

/// Parse a `D3D12_VERSIONED_ROOT_SIGNATURE_DESC`.
///
/// # Safety
/// `p` must point at a valid versioned description.
pub unsafe fn desc_from_versioned(p: *const u8) -> Result<RootSignatureDesc, String> {
    if p.is_null() {
        return Err("null versioned desc".into());
    }
    let v = u32_at(p, 0);
    let version = RootSignatureVersion::from_raw(v).ok_or_else(|| format!("version {v}"))?;
    desc_from_mem(version, p.add(8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    struct Range1 {
        ty: u32,
        num: u32,
        base: u32,
        space: u32,
        flags: u32,
        off: u32,
    }
    #[repr(C)]
    struct Param {
        ty: u32,
        _pad: u32,
        a: u64,
        b: u64,
        vis: u32,
        _pad2: u32,
    }
    #[repr(C)]
    struct Desc {
        np: u32,
        params: *const Param,
        ns: u32,
        samplers: *const u8,
        flags: u32,
    }
    #[repr(C)]
    struct Versioned {
        version: u32,
        desc: Desc,
    }

    #[test]
    fn versioned_1_1_round_trips_through_rts0() {
        let ranges = [Range1 { ty: 0, num: 4, base: 0, space: 0, flags: 0, off: 0 }, Range1 { ty: 1, num: 1, base: 2, space: 1, flags: 2, off: 0xFFFF_FFFF }];
        let params = [
            Param { ty: 0, _pad: 0, a: 2, b: ranges.as_ptr() as u64, vis: 0, _pad2: 0 },
            Param { ty: 1, _pad: 0, a: 3 | (0 << 32), b: 8, vis: 5, _pad2: 0 },
            Param { ty: 2, _pad: 0, a: 1, b: 0, vis: 0, _pad2: 0 },
        ];
        let v = Versioned { version: 2, desc: Desc { np: 3, params: params.as_ptr(), ns: 0, samplers: std::ptr::null(), flags: 1 } };
        let d = unsafe { desc_from_versioned(&v as *const Versioned as *const u8) }.unwrap();
        assert_eq!(d.parameters.len(), 3);
        match &d.parameters[0].kind {
            RootParameterKind::DescriptorTable(r) => {
                assert_eq!(r.len(), 2);
                assert_eq!(r[1].range_type, DescriptorRangeType::Uav);
                assert_eq!(r[1].register_space, 1);
                assert_eq!(r[1].flags, 2);
            }
            k => panic!("{k:?}"),
        }
        assert_eq!(d.parameters[1].kind, RootParameterKind::Constants { shader_register: 3, register_space: 0, num_32bit_values: 8 });
        assert_eq!(d.parameters[1].visibility, 5);
        let blob = d.to_blob();
        let back = RootSignatureDesc::from_blob(&blob).unwrap();
        assert_eq!(back, d);
    }
}
