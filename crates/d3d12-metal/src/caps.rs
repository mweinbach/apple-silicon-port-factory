//! `ID3D12Device::CheckFeatureSupport`: what we report and why
//! (`docs/design/ADVICE_M3.md` §4).
//!
//! The game's gates (`d3d12_init_and_check` 0x14208d490,
//! `gpu_feature_validator` 0x142070370) need: `TypedUAVLoadAdditionalFormats`,
//! `ResourceBindingTier 3`, shader model ≥ 6.5. Mesh shaders (unless
//! `AW2_MESH_SHADERS=1`), VRS, DXR,
//! enhanced barriers and work graphs are reported absent, which selects the
//! game's VS / legacy-barrier / raster paths. Struct sizes are checked
//! exactly (a mismatch is `E_INVALIDARG`, like the runtime).

use crate::format;

#[derive(Debug, PartialEq, Eq)]
pub enum CapsError {
    /// Wrong `DataSize` or bad input field.
    InvalidArg(String),
    /// Feature value unknown to us.
    Unknown(u32),
}

pub const FL_12_1: u32 = 0xC100;
pub const SM_6_6: u32 = 0x66;
pub const RS_1_1: u32 = 2;

fn feature_name(f: u32) -> &'static str {
    match f {
        0 => "D3D12_OPTIONS",
        1 => "ARCHITECTURE",
        2 => "FEATURE_LEVELS",
        3 => "FORMAT_SUPPORT",
        4 => "MULTISAMPLE_QUALITY_LEVELS",
        5 => "FORMAT_INFO",
        6 => "GPU_VIRTUAL_ADDRESS_SUPPORT",
        7 => "SHADER_MODEL",
        8 => "D3D12_OPTIONS1",
        10 => "PROTECTED_RESOURCE_SESSION_SUPPORT",
        12 => "ROOT_SIGNATURE",
        16 => "ARCHITECTURE1",
        18 => "D3D12_OPTIONS2",
        19 => "SHADER_CACHE",
        20 => "COMMAND_QUEUE_PRIORITY",
        21 => "D3D12_OPTIONS3",
        22 => "EXISTING_HEAPS",
        23 => "D3D12_OPTIONS4",
        24 => "SERIALIZATION",
        25 => "CROSS_NODE",
        27 => "D3D12_OPTIONS5",
        28 => "DISPLAYABLE",
        30 => "D3D12_OPTIONS6",
        31 => "QUERY_META_COMMAND",
        32 => "D3D12_OPTIONS7",
        33 => "PROTECTED_RESOURCE_SESSION_TYPE_COUNT",
        34 => "PROTECTED_RESOURCE_SESSION_TYPES",
        36..=49 => "D3D12_OPTIONS8..20",
        50 => "PREDICATION",
        51 => "PLACED_RESOURCE_SUPPORT_INFO",
        52 => "HARDWARE_COPY",
        53 => "D3D12_OPTIONS21",
        54 => "D3D12_TIGHT_ALIGNMENT",
        64 => "BARRIER_LAYOUT",
        65 => "D3D12_OPTIONS22",
        _ => "?",
    }
}

fn rd(d: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(d[i * 4..i * 4 + 4].try_into().unwrap())
}

fn put(d: &mut [u8], vals: &[u32]) {
    for (i, v) in vals.iter().enumerate() {
        d[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
}

/// Fill `data` (the caller's struct, in/out) for `feature`.
///
/// # Safety
/// For `FEATURE_LEVELS` the struct holds a pointer to the requested levels,
/// which is dereferenced.
pub unsafe fn check_feature_support(feature: u32, data: &mut [u8]) -> Result<(), CapsError> {
    let size = data.len();
    let need = |n: usize| -> Result<(), CapsError> {
        if size == n {
            Ok(())
        } else {
            Err(CapsError::InvalidArg(format!("{}: DataSize {size} != {n}", feature_name(feature))))
        }
    };
    const T: u32 = 1;
    const F: u32 = 0;
    match feature {
        // D3D12_OPTIONS
        0 => {
            need(60)?;
            put(
                data,
                &[
                    F,  // DoublePrecisionFloatShaderOps (Metal has no fp64)
                    F,  // OutputMergerLogicOp
                    2,  // MinPrecisionSupport: 16_BIT
                    0,  // TiledResourcesTier: NOT_SUPPORTED
                    3,  // ResourceBindingTier 3
                    F,  // PSSpecifiedStencilRefSupported (MSC support unverified)
                    T,  // TypedUAVLoadAdditionalFormats (game gate)
                    F,  // ROVsSupported
                    0,  // ConservativeRasterizationTier
                    40, // MaxGPUVirtualAddressBitsPerResource
                    F,  // StandardSwizzle64KBSupported
                    0,  // CrossNodeSharingTier
                    F,  // CrossAdapterRowMajorTextureSupported
                    T,  // VPAndRTArrayIndexFromAnyShaderFeedingRasterizer...
                    2,  // ResourceHeapTier 2
                ],
            );
        }
        // ARCHITECTURE / ARCHITECTURE1 (NodeIndex is input)
        1 | 16 => {
            need(if feature == 1 { 16 } else { 20 })?;
            if rd(data, 0) != 0 {
                return Err(CapsError::InvalidArg("ARCHITECTURE: NodeIndex != 0".into()));
            }
            // NodeIndex, TileBasedRenderer (honest: Apple GPUs are TBDR), UMA,
            // CacheCoherentUMA (, IsolatedMMU)
            put(data, &[0, T, T, T]);
            if feature == 16 {
                put(&mut data[16..], &[T]);
            }
        }
        // FEATURE_LEVELS: {NumFeatureLevels, pad, const D3D_FEATURE_LEVEL*, MaxSupported}
        2 => {
            need(24)?;
            let n = rd(data, 0) as usize;
            let p = u64::from_le_bytes(data[8..16].try_into().unwrap()) as usize as *const u32;
            if n == 0 || p.is_null() {
                return Err(CapsError::InvalidArg("FEATURE_LEVELS: no levels requested".into()));
            }
            let req = std::slice::from_raw_parts(p, n);
            let max = req.iter().copied().filter(|&l| l <= FL_12_1).max().unwrap_or(0);
            if max == 0 {
                return Err(CapsError::InvalidArg(format!("FEATURE_LEVELS: none of {req:x?} supported")));
            }
            data[16..20].copy_from_slice(&max.to_le_bytes());
        }
        // FORMAT_SUPPORT {Format, Support1, Support2}
        3 => {
            need(12)?;
            let (s1, s2) = format::support(rd(data, 0));
            put(&mut data[4..], &[s1, s2]);
            if s1 == 0 {
                // The runtime returns E_FAIL for formats with no support.
                return Err(CapsError::InvalidArg(format!("FORMAT_SUPPORT: {} unsupported", format::name(rd(data, 0)))));
            }
        }
        // MULTISAMPLE_QUALITY_LEVELS {Format, SampleCount, Flags, NumQualityLevels}
        4 => {
            need(16)?;
            let q = format::msaa_quality_levels(rd(data, 0), rd(data, 1));
            put(&mut data[12..], &[q]);
        }
        // FORMAT_INFO {Format, PlaneCount}
        5 => {
            need(8)?;
            let f = format::info(rd(data, 0));
            if f.planes == 0 {
                return Err(CapsError::InvalidArg(format!("FORMAT_INFO: {} unknown", format::name(rd(data, 0)))));
            }
            put(&mut data[4..], &[f.planes]);
        }
        // GPU_VIRTUAL_ADDRESS_SUPPORT
        6 => {
            need(8)?;
            put(data, &[40, 40]);
        }
        // SHADER_MODEL {HighestShaderModel} (in: highest the app knows)
        7 => {
            need(4)?;
            let req = rd(data, 0);
            if req < 0x51 {
                return Err(CapsError::InvalidArg(format!("SHADER_MODEL: request {req:#x}")));
            }
            put(data, &[req.min(SM_6_6)]);
        }
        // D3D12_OPTIONS1 {WaveOps, LaneMin, LaneMax, TotalLaneCount, ExpandedComputeResourceStates, Int64ShaderOps}
        8 => {
            need(24)?;
            put(data, &[T, 32, 32, 32 * 1024, T, T]);
        }
        // PROTECTED_RESOURCE_SESSION_SUPPORT {NodeIndex, Support}
        10 => {
            need(8)?;
            put(&mut data[4..], &[0]);
        }
        // ROOT_SIGNATURE {HighestVersion} (in/out)
        12 => {
            need(4)?;
            let req = rd(data, 0);
            put(data, &[req.min(RS_1_1)]);
        }
        // D3D12_OPTIONS2 {DepthBoundsTestSupported, ProgrammableSamplePositionsTier}
        18 => {
            need(8)?;
            put(data, &[F, 0]);
        }
        // SHADER_CACHE {SupportFlags}: SINGLE_PSO | LIBRARY would promise
        // GetCachedBlob reuse; report none.
        19 => {
            need(4)?;
            put(data, &[0]);
        }
        // COMMAND_QUEUE_PRIORITY {CommandListType, Priority, PriorityForTypeIsSupported}
        20 => {
            need(12)?;
            let prio = rd(data, 1);
            put(&mut data[8..], &[(prio == 0 || prio == 100) as u32]);
        }
        // D3D12_OPTIONS3
        21 => {
            need(20)?;
            put(data, &[F, T, 0, 0, F]);
        }
        // EXISTING_HEAPS
        22 => {
            need(4)?;
            put(data, &[F]);
        }
        // D3D12_OPTIONS4 {MSAA64KBAligned, SharedResourceCompatibilityTier, Native16BitShaderOps}
        23 => {
            need(12)?;
            put(data, &[F, 0, T]);
        }
        // SERIALIZATION {NodeIndex, HeapSerializationTier}
        24 => {
            need(8)?;
            put(&mut data[4..], &[0]);
        }
        // CROSS_NODE
        25 => {
            need(8)?;
            put(data, &[0, F]);
        }
        // D3D12_OPTIONS5 {SRVOnlyTiledResourceTier3, RenderPassesTier, RaytracingTier}
        27 => {
            need(12)?;
            put(data, &[F, 0, if crate::rt::enabled() { 11 } else { 0 }]);
        }
        // DISPLAYABLE
        28 => {
            need(8)?;
            put(data, &[F, 0]);
        }
        // D3D12_OPTIONS6: no VRS
        30 => {
            need(20)?;
            put(data, &[F, F, 0, 0, F]);
        }
        // D3D12_OPTIONS7 {MeshShaderTier, SamplerFeedbackTier}. Mesh
        // shaders (TIER_1 = 10) only behind AW2_MESH_SHADERS=1 until the
        // game's mesh path is validated; the VS path is the default.
        32 => {
            need(8)?;
            put(data, &[if crate::mesh::enabled() { 10 } else { 0 }, 0]);
        }
        // PROTECTED_RESOURCE_SESSION_TYPE_COUNT {NodeIndex, Count}
        33 => {
            need(8)?;
            put(&mut data[4..], &[0]);
        }
        // D3D12_OPTIONS8 {UnalignedBlockTexturesSupported}
        36 => {
            need(4)?;
            put(data, &[F]);
        }
        // D3D12_OPTIONS9
        37 => {
            need(24)?;
            put(data, &[F, F, F, F, F, 0]);
        }
        // D3D12_OPTIONS10
        39 => {
            need(8)?;
            put(data, &[F, F]);
        }
        // D3D12_OPTIONS11
        40 => {
            need(4)?;
            put(data, &[F]);
        }
        // D3D12_OPTIONS12 {MSPrimitivesPipelineStatistic..., EnhancedBarriersSupported, RelaxedFormatCasting}
        41 => {
            need(12)?;
            put(data, &[F, F, F]);
        }
        // D3D12_OPTIONS13
        42 => {
            need(24)?;
            put(data, &[F, F, F, F, F, F]);
        }
        // D3D12_OPTIONS14
        43 => {
            need(12)?;
            put(data, &[F, F, F]);
        }
        // D3D12_OPTIONS15 {TriangleFanSupported, DynamicIndexBufferStripCutSupported}
        44 => {
            need(8)?;
            put(data, &[F, F]);
        }
        // D3D12_OPTIONS16 {DynamicDepthBiasSupported, GPUUploadHeapSupported}
        45 => {
            need(8)?;
            put(data, &[F, F]);
        }
        // D3D12_OPTIONS17
        46 => {
            need(8)?;
            put(data, &[F, F]);
        }
        // D3D12_OPTIONS18 {RenderPassesValid}
        47 => {
            need(4)?;
            put(data, &[F]);
        }
        // D3D12_OPTIONS19
        48 => {
            need(40)?;
            put(data, &[F, 0, F, F, F, F, 2048, 2048, 1_000_000, F]);
        }
        // D3D12_OPTIONS20
        49 => {
            need(8)?;
            put(data, &[F, 0]);
        }
        // PREDICATION / HARDWARE_COPY / TIGHT_ALIGNMENT
        50 | 52 | 54 => {
            need(4)?;
            put(data, &[0]);
        }
        // D3D12_OPTIONS21 {WorkGraphsTier, ExecuteIndirectTier (1_0 = 10), SampleCmpGradientAndBias, ExtendedCommandInfo}
        53 => {
            need(16)?;
            put(data, &[0, 10, F, F]);
        }
        // D3D12_OPTIONS22: all zero
        65 => {
            need(16)?;
            put(data, &[F, F, 0, 0]);
        }
        _ => return Err(CapsError::Unknown(feature)),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(feature: u32, size: usize, init: &[u32]) -> Result<Vec<u32>, CapsError> {
        let mut d = vec![0u8; size];
        put(&mut d, init);
        unsafe { check_feature_support(feature, &mut d) }?;
        Ok((0..size / 4).map(|i| rd(&d, i)).collect())
    }

    /// Golden values for every (feature, size) the game queries.
    #[test]
    fn golden_feature_table() {
        assert_eq!(q(0, 60, &[]).unwrap(), vec![0, 0, 2, 0, 3, 0, 1, 0, 0, 40, 0, 0, 0, 1, 2]);
        assert!(matches!(q(0, 0x38, &[]), Err(CapsError::InvalidArg(_))));
        assert_eq!(q(7, 4, &[0x69]).unwrap(), vec![0x66]);
        assert_eq!(q(7, 4, &[0x65]).unwrap(), vec![0x65]);
        assert_eq!(q(27, 12, &[]).unwrap(), vec![0, 0, 0]);
        assert_eq!(q(30, 20, &[]).unwrap(), vec![0; 5]);
        assert_eq!(q(32, 8, &[]).unwrap(), vec![0, 0]);
        assert_eq!(q(41, 12, &[]).unwrap(), vec![0, 0, 0]);
        assert_eq!(q(65, 16, &[]).unwrap(), vec![0; 4]);
        assert_eq!(q(12, 4, &[3]).unwrap(), vec![2]);
        assert_eq!(q(16, 20, &[0]).unwrap(), vec![0, 1, 1, 1, 1]);
        assert_eq!(q(5, 8, &[45]).unwrap(), vec![45, 2]);
        assert_eq!(q(999, 4, &[]), Err(CapsError::Unknown(999)));
        let levels = [0xb000u32, 0xc000, 0xc100, 0xc200];
        let mut d = vec![0u8; 24];
        put(&mut d, &[4]);
        d[8..16].copy_from_slice(&(levels.as_ptr() as u64).to_le_bytes());
        unsafe { check_feature_support(2, &mut d) }.unwrap();
        assert_eq!(rd(&d, 4), 0xc100);
    }
}
