//! Pipeline state objects: the game's DXIL → metallib (`dxil_metal`,
//! cached on disk) → Metal 4 pipelines (`dxil_metal::pso_cache`, MTL4Compiler
//! + archives).

mod compute;
mod dxil_hk;
mod graphics;
mod opts;
mod reflection;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use dxil_metal::pso_cache::PipelineCache;
use dxil_metal::root_signature::RootSignature;
use dxil_metal::{CompileOptions, CompiledShader, ShaderPipeline, ShaderStage};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

pub use compute::{
    apply_compute_scheduling_options, compute_pipeline_from_function,
    compute_pipeline_from_function_tg, compute_pipeline_from_function_tg_opts, ComputePipeline,
};
#[cfg(test)]
pub use dxil_hk::register_shader_hk_profile_for_test;
pub use dxil_hk::{
    build_handle_kind_profile, dxil_annotate_handle_kinds, dxil_annotate_handle_kinds_opt,
    dxil_handle_kinds_detail_opt, resolve_pso_hk_profile, shader_handle_kind_profile,
    DxilHkDetail, DxilSrvRangeHk, HandleKindProfile,
};
pub use graphics::{
    depth_stencil_state, infer_vertex_input_format, vertex_buffer_index, GeometryEmulation,
    GraphicsDesc, GraphicsPipeline, MtlDepthStencil, TessellationEmulation, MAX_VERTEX_SLOTS,
};
pub(crate) use opts::RenderPsoStateKey;
pub use opts::{
    air_no_res_heap, any_frag_buf_write, cs_mid_qb_exact_v2, cs_pso_tg, fast_bind_pso,
    gfx_lazy_icb, msc_compute_access, msc_stage_usage, note_frag_buf_write, shader_immutable_opt,
    shader_pso_opt_v2, sm66_cs_stage_exact, sm66_gfx_stage_exact, IMMUTABLE_OPT_KEY_SALT,
    SHADER_PSO_OPT_V2_SALT,
};
pub(crate) use reflection::ShaderStageSummary;
pub use reflection::{
    metallib_res_desc_heap_unused, msc_resource_usages, msc_stage_may_read_cbv_from_reflection,
    msc_used_resources, shader_uses_uav, shader_writes_buf_uav, shader_writes_uav,
    stage_may_read_cbv_buffer_with, stage_reads_resources, stage_reads_resources_with,
    stage_srv_uav_classes, stage_writes_uav, vs_may_read_cbv_buffer, ComputeAccess, ComputeBind,
    MscUsedResource, READS_BUF, READS_TEX,
};

pub type MtlLibrary = ProtocolObject<dyn MTLLibrary>;
pub type MtlComputePipeline = ProtocolObject<dyn MTLComputePipelineState>;
pub type MtlRenderPipeline = ProtocolObject<dyn MTLRenderPipelineState>;

struct Cache(PipelineCache);
unsafe impl Send for Cache {}
unsafe impl Sync for Cache {}

static PSO_CACHE: OnceLock<Result<Cache, String>> = OnceLock::new();

/// The process-wide PSO cache (`PORT_PSO_CACHE` / `AW2_PSO_CACHE`, archives per GPU/OS).
pub fn pso_cache() -> Result<&'static PipelineCache, String> {
    PSO_CACHE
        .get_or_init(|| {
            let d = super::device().ok_or("no device")?;
            PipelineCache::from_env(&d.dev)
                .map(|c| {
                    if cfg!(target_os = "ios") {
                        c.set_post_batch_hook(on_ios_pso_batch);
                    }
                    Cache(c)
                })
                .map_err(|e| format!("PSO cache: {e}"))
        })
        .as_ref()
        .map(|c| &c.0)
        .map_err(Clone::clone)
}

#[cfg(target_vendor = "apple")]
fn release_malloc_pressure() {
    extern "C" {
        fn malloc_zone_pressure_relief(zone: *mut std::ffi::c_void, goal: usize) -> usize;
    }
    unsafe {
        malloc_zone_pressure_relief(std::ptr::null_mut(), 0);
    }
}

#[cfg(not(target_vendor = "apple"))]
fn release_malloc_pressure() {}

/// Invoked automatically on iOS after each mid-frame PSO batch flush + `MTL4Compiler` reset:
/// evict transient `MTLLibrary` / metallib caches, purge libmalloc magazines, and log footprint.
fn on_ios_pso_batch() {
    let dropped_libs = LIBS_CACHE
        .get()
        .map(|libs| {
            let mut g = libs.0.lock().unwrap();
            let n = g.len();
            g.clear();
            n
        })
        .unwrap_or(0);
    let dropped_shader_bytes = dxil_metal::ShaderPipeline::trim_memory_if_initialized();
    release_malloc_pressure();
    log::info!(
        target: "d3d",
        "pipeline::pso_batch: reset MTL4Compiler, evicted {dropped_libs} MTLLibrary handles and {:.1} MiB of cached metallibs",
        dropped_shader_bytes as f64 / (1024.0 * 1024.0)
    );
    super::device::log_memory_snapshot("pso-batch");
}

/// Flush newly compiled pipelines mid-session to an incremental on-disk archive (never wipes
/// existing archives during a compacting session).
pub fn flush_pso_cache() {
    if let Ok(c) = pso_cache() {
        match c.flush_incremental() {
            Ok(Some(p)) => log::info!(target: "d3d", "PSO cache: wrote {}", p.display()),
            Ok(None) => {}
            Err(e) => log::warn!(target: "d3d", "PSO cache flush: {e}"),
        }
    }
}

/// Flush newly compiled pipelines ONLY if at least `min_compiles` cold compilations
/// have accumulated since the last flush (avoids losing compiled level PSOs on iOS).
pub fn flush_pso_cache_if_burst(min_compiles: u64) {
    if let Some(Ok(c)) = PSO_CACHE.get() {
        match c.0.flush_if_dirty_compiles(min_compiles) {
            Ok(Some(p)) => log::info!(target: "d3d", "PSO cache (burst): wrote {}", p.display()),
            Ok(None) => {}
            Err(e) => log::warn!(target: "d3d", "PSO cache burst flush: {e}"),
        }
    }
}

/// Final flush at process exit: writes newly compiled pipelines and completes compaction if active.
/// Safe to invoke from process exit hooks without lazily initializing Metal on non-GPU runs.
pub fn flush_pso_cache_if_initialized() {
    if let Some(Ok(c)) = PSO_CACHE.get() {
        match c.0.flush() {
            Ok(Some(p)) => log::info!(target: "d3d", "PSO cache (exit): wrote {}", p.display()),
            Ok(None) => {}
            Err(e) => log::warn!(target: "d3d", "PSO cache flush (exit): {e}"),
        }
    }
}

/// Plain snapshot of Tier-1 shader converter and Tier-2 Metal 4 PSO cache counters
/// for boot reports (`port-runner --report-json`). Never lazily initializes Metal or `dxil-metald`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ShaderReportStats {
    pub tier1_initialized: bool,
    pub tier1_memory_hits: u64,
    pub tier1_disk_hits: u64,
    pub tier1_compiles: u64,
    pub tier1_failures: u64,
    pub tier1_compile_ms: u64,
    pub pso_initialized: bool,
    pub pso_memory_hits: u64,
    pub pso_archive_hits: u64,
    pub pso_compiles: u64,
    pub pso_failures: u64,
    pub pso_compile_ms: u64,
}

/// Snapshot Tier-2 Metal 4 PSO cache stats if initialized (never lazily initializes Metal).
pub fn pso_cache_stats_if_initialized() -> Option<dxil_metal::pso_cache::PipelineCacheStats> {
    PSO_CACHE.get().and_then(|r| r.as_ref().ok()).map(|c| c.0.stats())
}

/// Combined Tier-1 + Tier-2 shader stats snapshot without lazily initializing either subsystem.
pub fn shader_report_stats_if_initialized() -> ShaderReportStats {
    let mut s = ShaderReportStats::default();
    if let Some(t1) = dxil_metal::ShaderPipeline::stats_if_initialized() {
        s.tier1_initialized = true;
        s.tier1_memory_hits = t1.memory_hits;
        s.tier1_disk_hits = t1.disk_hits;
        s.tier1_compiles = t1.compiles;
        s.tier1_failures = t1.compile_failures;
        s.tier1_compile_ms = t1.compile_nanos / 1_000_000;
    }
    if let Some(t2) = pso_cache_stats_if_initialized() {
        s.pso_initialized = true;
        s.pso_memory_hits = t2.memory_hits;
        s.pso_archive_hits = t2.archive_hits;
        s.pso_compiles = t2.compiles;
        s.pso_failures = t2.failures;
        s.pso_compile_ms = t2.compile_nanos / 1_000_000;
    }
    s
}

struct Libs(Mutex<HashMap<u128, Retained<MtlLibrary>>>);
unsafe impl Send for Libs {}
unsafe impl Sync for Libs {}

static LIBS_CACHE: OnceLock<Libs> = OnceLock::new();

/// Drop cached `MTLLibrary` objects and in-memory metallib byte buffers, persist any newly
/// compiled PSOs incrementally (and recreate `MTL4Compiler` on iOS to free LLVM arenas),
/// and return freed malloc magazines to XNU.
pub fn trim_caches() {
    let dropped_libs = LIBS_CACHE
        .get()
        .map(|libs| {
            let mut g = libs.0.lock().unwrap();
            let n = g.len();
            g.clear();
            n
        })
        .unwrap_or(0);
    let dropped_shader_bytes = dxil_metal::ShaderPipeline::trim_memory_if_initialized();
    // On iOS, avoid nuking the active steady-state ICB / ring working set on soft memory
    // warnings when >= 1 GiB of Jetsam headroom remains (wiping warm ICBs triggers an
    // immediate 30+ `newIndirectCommandBuffer` stall on the very next frame). Under real
    // Jetsam pressure (< 1 GiB headroom) or explicit macOS trims, evict 100% of idle pools.
    let hard_pressure = super::device::available_headroom_bytes()
        .map(|avail| avail < (1024 << 20))
        .unwrap_or(true);
    let dropped_icbs = if hard_pressure {
        super::indirect_icb::trim_pool()
    } else {
        super::indirect_icb::trim_pool_soft()
    };
    let dropped_rings = if hard_pressure {
        super::binding::trim_pool()
    } else {
        super::binding::trim_pool_keep(16 << 20)
    };
    let reaped_deferred = super::queue::collect_all();
    if let Some(Ok(c)) = PSO_CACHE.get() {
        let res = if cfg!(target_os = "ios") {
            c.0.flush_and_reset_compiler()
        } else {
            c.0.flush_incremental()
        };
        if let Err(e) = res {
            log::warn!(target: "d3d", "PSO cache flush (trim): {e}");
        }
    }
    release_malloc_pressure();
    log::info!(
        target: "d3d",
        "pipeline::trim_caches (hard={hard_pressure}): evicted {dropped_libs} MTLLibrary handles, {dropped_icbs} idle ICBs, {dropped_rings} ring buffers, {reaped_deferred} deferred drops, and {:.1} MiB of cached metallibs",
        dropped_shader_bytes as f64 / (1024.0 * 1024.0)
    );
}

/// The `MTLLibrary` of a converted shader (cached by shader key).
pub fn library(s: &CompiledShader) -> Result<Retained<MtlLibrary>, String> {
    let libs = LIBS_CACHE.get_or_init(|| Libs(Mutex::new(HashMap::new())));
    if let Some(l) = libs.0.lock().unwrap().get(&s.key) {
        return Ok(l.clone());
    }
    let d = super::device().ok_or("no device")?;
    let lib = objc2::rc::autoreleasepool(|_| {
        let data = dispatch2::DispatchData::from_bytes(&s.metallib);
        d.dev.newLibraryWithData_error(&data).map_err(|e| format!("newLibraryWithData: {e}"))
    })?;
    let mut guard = libs.0.lock().unwrap();
    if cfg!(target_os = "ios") && guard.len() >= 128 {
        guard.clear();
    }
    guard.insert(s.key, lib.clone());
    Ok(lib)
}

/// Compiles an MSL source string into a `MTLLibrary` using the process-wide `MTL4Compiler`
/// targeting Metal 4.1 (`MTLLanguageVersion4_1` / macOS 27).
pub fn library_from_source(name: &str, source: &str) -> Result<Retained<MtlLibrary>, String> {
    library_from_source_with_opts(name, source, shader_immutable_opt())
}

/// Like [`library_from_source`], with explicit `fast_math` control (`AW2_SHADER_IMMUTABLE_OPT=1`).
pub fn library_from_source_with_opts(
    name: &str,
    source: &str,
    fast_math: bool,
) -> Result<Retained<MtlLibrary>, String> {
    pso_cache()?
        .library_from_source_with_opts(name, source, fast_math)
        .map_err(|e| format!("{e}"))
}

/// Applies `AW2_SHADER_IMMUTABLE_OPT=1` compile-time options to any `MTL4PipelineDescriptor`.
pub fn apply_immutable_pipeline_options(desc: &MTL4PipelineDescriptor) {
    let opts = MTL4PipelineOptions::new();
    opts.setShaderValidation(MTLShaderValidation::Disabled);
    opts.setShaderReflection(MTL4ShaderReflection::None);
    desc.setOptions(Some(&opts));
}

pub fn function_descriptor(
    lib: &MtlLibrary,
    name: &str,
) -> Retained<MTL4LibraryFunctionDescriptor> {
    let f = MTL4LibraryFunctionDescriptor::new();
    f.setName(Some(&NSString::from_str(name)));
    f.setLibrary(Some(lib));
    f
}

/// Builds a `MTL4FunctionDescriptor` for `name` in `lib` with explicit function constant
/// overrides (`(name, MTLDataType, raw_bytes)`) plus zero-initialized defaults for any
/// remaining reflected function constants on `shader`.
fn specialized_function_descriptor(
    lib: &MtlLibrary,
    name: &str,
    shader: &CompiledShader,
    overrides: &[(&str, MTLDataType, &[u8])],
) -> Retained<MTL4FunctionDescriptor> {
    let base = function_descriptor(lib, name);
    if overrides.is_empty()
        && !shader.reflection.needs_function_constants
        && shader.reflection.function_constants.is_empty()
    {
        return Retained::into_super(base);
    }
    let vals = MTLFunctionConstantValues::new();
    let zero = [0u8; 64];
    let zero_ptr = std::ptr::NonNull::new(zero.as_ptr() as *mut std::ffi::c_void).unwrap();
    for fc in &shader.reflection.function_constants {
        if overrides.iter().any(|(n, _, _)| *n == fc.name) {
            continue;
        }
        unsafe {
            vals.setConstantValue_type_withName(
                zero_ptr,
                MTLDataType(fc.ty as usize),
                &NSString::from_str(&fc.name),
            );
        }
    }
    for &(cname, ty, bytes) in overrides {
        let ptr = std::ptr::NonNull::new(bytes.as_ptr() as *mut std::ffi::c_void).unwrap();
        unsafe {
            vals.setConstantValue_type_withName(ptr, ty, &NSString::from_str(cname));
        }
    }
    let spec = MTL4SpecializedFunctionDescriptor::new();
    spec.setFunctionDescriptor(Some(&base));
    spec.setConstantValues(Some(&vals));
    Retained::into_super(spec)
}

/// Builds a `MTL4FunctionDescriptor` for `shader`, wrapping in a
/// `MTL4SpecializedFunctionDescriptor` with zero-initialized `MTLFunctionConstantValues`
/// whenever the converter emitted Metal function constants (`needs_function_constants`).
pub fn shader_function_descriptor(
    lib: &MtlLibrary,
    shader: &CompiledShader,
) -> Retained<MTL4FunctionDescriptor> {
    specialized_function_descriptor(lib, &shader.function_name, shader, &[])
}

/// Convert one stage of the game's DXIL against `rs`.
pub fn compile(
    stage: ShaderStage,
    dxil: &[u8],
    rs: &RootSignature,
    opts: &CompileOptions,
) -> Result<Arc<CompiledShader>, String> {
    let p = ShaderPipeline::global().map_err(|e| format!("shader pipeline: {e}"))?;
    p.compile_stage(stage, dxil, Some(rs), opts).map_err(|e| format!("{stage:?} shader: {e}"))
}
