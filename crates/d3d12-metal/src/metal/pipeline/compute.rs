//! Compute pipeline creation (`ComputePipeline` and MSL helper compute PSOs).

use std::sync::Arc;

use dxil_metal::pso_cache::PipelineCache;
use dxil_metal::root_signature::RootSignature;
use dxil_metal::{CompileOptions, CompiledShader, ShaderStage, StageInfo};
use objc2::rc::Retained;
use objc2_foundation::NSString;
use objc2_metal::*;

use super::dxil_hk::HandleKindProfile;
use super::opts::{cs_pso_tg, shader_immutable_opt, IMMUTABLE_OPT_KEY_SALT};
use super::reflection::{ComputeAccess, ShaderStageSummary};
use super::{
    apply_immutable_pipeline_options, compile, function_descriptor, library, pso_cache,
    shader_function_descriptor, MtlComputePipeline, MtlLibrary,
};
use crate::metal::m41::{ComputeSchedulingOpts, ForwardProgressUsage};

/// Applies Metal 4.1 (`macOS 27` / `iOS 27`) compute pipeline scheduling hints
/// (`forwardProgressUsage`, `contentionRelief`, `optimizeForPersistentKernel`) via
/// `respondsToSelector:` so macOS 26 remains unaffected.
pub fn apply_compute_scheduling_options(
    desc: &MTL4ComputePipelineDescriptor,
    sched: &ComputeSchedulingOpts,
) {
    if !sched.is_active() {
        return;
    }
    unsafe {
        if let Some(fp) = sched.forward_progress {
            let has_sel: objc2::runtime::Bool = objc2::msg_send![
                desc,
                respondsToSelector: objc2::sel!(setForwardProgressUsage:)
            ];
            if has_sel.as_bool() {
                let _: () = objc2::msg_send![desc, setForwardProgressUsage: fp as isize];
            }
        }
        if let Some(cr) = sched.contention_relief {
            let has_sel: objc2::runtime::Bool = objc2::msg_send![
                desc,
                respondsToSelector: objc2::sel!(setContentionRelief:)
            ];
            if has_sel.as_bool() {
                let _: () = objc2::msg_send![desc, setContentionRelief: cr as isize];
            }
        }
        if sched.optimize_for_persistent_kernel
            && sched.forward_progress != Some(ForwardProgressUsage::Weak)
        {
            let has_sel: objc2::runtime::Bool = objc2::msg_send![
                desc,
                respondsToSelector: objc2::sel!(setOptimizeForPersistentKernel:)
            ];
            if has_sel.as_bool() {
                let _: () = objc2::msg_send![
                    desc,
                    setOptimizeForPersistentKernel: objc2::runtime::Bool::YES
                ];
            }
        }
    }
}

/// Builds a `MTLComputePipelineState` for `function_name` in `lib` through the `MTL4Compiler`
/// and `MTL4Archive` PSO cache.
pub fn compute_pipeline_from_function(
    lib: &MtlLibrary,
    function_name: &str,
) -> Result<Retained<MtlComputePipeline>, String> {
    compute_pipeline_from_function_tg(lib, function_name, None)
}

/// Builds a `MTLComputePipelineState` for `function_name` in `lib`, optionally
/// pinning `requiredThreadsPerThreadgroup` and `threadGroupSizeIsMultipleOfThreadExecutionWidth`.
pub fn compute_pipeline_from_function_tg(
    lib: &MtlLibrary,
    function_name: &str,
    tg: Option<[u32; 3]>,
) -> Result<Retained<MtlComputePipeline>, String> {
    compute_pipeline_from_function_tg_opts(lib, function_name, tg, shader_immutable_opt())
}

/// Like [`compute_pipeline_from_function_tg`], with explicit `immut_opt` control.
pub fn compute_pipeline_from_function_tg_opts(
    lib: &MtlLibrary,
    function_name: &str,
    tg: Option<[u32; 3]>,
    immut_opt: bool,
) -> Result<Retained<MtlComputePipeline>, String> {
    let sched = ComputeSchedulingOpts::from_env();
    if tg.is_none() && !immut_opt && !sched.is_active() {
        let mut k = dxil_metal::hash::KeyHasher::new("d3d12-metal msl compute pso v1");
        k.bytes(function_name.as_bytes());
        return pso_cache()?
            .compute_pipeline_from_function(Some(k.finish()), lib, function_name)
            .map_err(|e| format!("{function_name}: {e}"));
    }
    let mut cache_key = if let Some(tg) = tg {
        let mut k = dxil_metal::hash::KeyHasher::new("d3d12-metal msl compute pso tg v1");
        k.bytes(function_name.as_bytes());
        k.u32(tg[0]);
        k.u32(tg[1]);
        k.u32(tg[2]);
        k.finish()
    } else {
        let mut k = dxil_metal::hash::KeyHasher::new("d3d12-metal msl compute pso v1");
        k.bytes(function_name.as_bytes());
        k.finish()
    };
    let desc = MTL4ComputePipelineDescriptor::new();
    desc.setComputeFunctionDescriptor(Some(&function_descriptor(lib, function_name)));
    desc.setLabel(Some(&NSString::from_str(function_name)));
    if let Some(tg) = tg {
        let total = (tg[0] as usize).saturating_mul(tg[1] as usize).saturating_mul(tg[2] as usize);
        if total > 0 && total <= 1024 {
            desc.setMaxTotalThreadsPerThreadgroup(total);
            desc.setRequiredThreadsPerThreadgroup(MTLSize {
                width: tg[0] as usize,
                height: tg[1] as usize,
                depth: tg[2] as usize,
            });
            desc.setThreadGroupSizeIsMultipleOfThreadExecutionWidth(total.is_multiple_of(32));
        }
    }
    if immut_opt {
        apply_immutable_pipeline_options(&desc);
        desc.setSupportBinaryLinking(false);
        desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Disabled);
        cache_key ^= IMMUTABLE_OPT_KEY_SALT;
    }
    if sched.is_active() {
        apply_compute_scheduling_options(&desc, &sched);
        cache_key ^= sched.cache_key_salt();
    }
    pso_cache()?
        .compute_pipeline(Some(cache_key), &desc)
        .map_err(|e| format!("{function_name}: {e}"))
}

pub struct ComputePipeline {
    pub pso: Retained<MtlComputePipeline>,
    pub rs: Arc<RootSignature>,
    pub threadgroup: [u32; 3],
    pub shader: Arc<CompiledShader>,
    pub access: ComputeAccess,
    pub hk_profile: HandleKindProfile,
}
unsafe impl Send for ComputePipeline {}
unsafe impl Sync for ComputePipeline {}

impl ComputePipeline {
    pub fn new(rs: Arc<RootSignature>, dxil: &[u8], label: &str) -> Result<ComputePipeline, String> {
        Self::new_with_all_opts(rs, dxil, label, cs_pso_tg(), shader_immutable_opt())
    }

    pub fn new_with_opts(
        rs: Arc<RootSignature>,
        dxil: &[u8],
        label: &str,
        pin_tg: bool,
    ) -> Result<ComputePipeline, String> {
        Self::new_with_all_opts(rs, dxil, label, pin_tg, shader_immutable_opt())
    }

    pub fn new_with_all_opts(
        rs: Arc<RootSignature>,
        dxil: &[u8],
        label: &str,
        pin_tg: bool,
        immut_opt: bool,
    ) -> Result<ComputePipeline, String> {
        let rs_for_manifest = rs.clone();
        let out = Self::new_in_cache(pso_cache()?, rs, dxil, label, pin_tg, immut_opt)?;
        crate::metal::pso_manifest::note_compute(&rs_for_manifest, dxil, pin_tg, immut_opt);
        Ok(out)
    }

    pub(crate) fn new_in_cache(
        cache: &PipelineCache,
        rs: Arc<RootSignature>,
        dxil: &[u8],
        label: &str,
        pin_tg: bool,
        immut_opt: bool,
    ) -> Result<ComputePipeline, String> {
        let shader = compile(ShaderStage::Compute, dxil, &rs, &CompileOptions::default())?;
        let threadgroup = match shader.reflection.info {
            StageInfo::Compute { threadgroup_size } => threadgroup_size,
            ref other => {
                return Err(format!("compute shader without compute reflection: {other:?}"));
            }
        };
        let summary = ShaderStageSummary::new(dxil, &shader, &rs);
        let (access, hk_profile) = summary.for_compute(dxil, &rs, &shader);
        let lib = library(&shader)?;
        let desc = MTL4ComputePipelineDescriptor::new();
        desc.setComputeFunctionDescriptor(Some(&shader_function_descriptor(&lib, &shader)));
        if !label.is_empty() {
            desc.setLabel(Some(&NSString::from_str(label)));
        } else if let Some(l) =
            crate::metal::gpucapture::pipeline_label(label, "cs", shader.key as u64)
        {
            desc.setLabel(Some(&NSString::from_str(&l)));
        }
        let mut cache_key = shader.key;
        if pin_tg {
            let total = (threadgroup[0] as usize)
                .saturating_mul(threadgroup[1] as usize)
                .saturating_mul(threadgroup[2] as usize);
            if total > 0 && total <= 1024 {
                desc.setMaxTotalThreadsPerThreadgroup(total);
                desc.setRequiredThreadsPerThreadgroup(MTLSize {
                    width: threadgroup[0] as usize,
                    height: threadgroup[1] as usize,
                    depth: threadgroup[2] as usize,
                });
                desc.setThreadGroupSizeIsMultipleOfThreadExecutionWidth(total.is_multiple_of(32));
                cache_key ^= 0x5447_5f52_4551_5f31_u128;
            }
        }
        if immut_opt {
            apply_immutable_pipeline_options(&desc);
            desc.setSupportBinaryLinking(false);
            desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Disabled);
            cache_key ^= IMMUTABLE_OPT_KEY_SALT;
        }
        let sched = ComputeSchedulingOpts::from_env();
        if sched.is_active() {
            apply_compute_scheduling_options(&desc, &sched);
            cache_key ^= sched.cache_key_salt();
        }
        let pso = cache
            .compute_pipeline(Some(cache_key), &desc)
            .map_err(|e| format!("compute pipeline: {e}"))?;
        Ok(ComputePipeline { pso, rs, threadgroup, shader, access, hk_profile })
    }
}
