//! Pipeline state objects: the game's DXIL → metallib (`dxil_metal`,
//! cached on disk) → Metal 4 pipelines (`dxil_metal::pso_cache`, MTL4Compiler
//! + archives).

use dxil_metal::pso_cache::PipelineCache;
use dxil_metal::root_signature::RootSignature;
use dxil_metal::{CompileOptions, CompiledShader, ShaderPipeline, ShaderStage, StageInMode, StageInfo};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSArray, NSString};
use objc2_metal::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

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
    let dropped_icbs = super::indirect_icb::trim_pool();
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
        "pipeline::trim_caches: evicted {dropped_libs} MTLLibrary handles, {dropped_icbs} idle ICBs, {reaped_deferred} deferred drops, and {:.1} MiB of cached metallibs",
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

/// `AW2_FAST_BIND_PSO`: dirty-track `MTL4ArgumentTable` buffer/VB bindings on
/// `CommandList`, deduplicate `compute_clear_batch` encoder state, and pin
/// internal MSL compute PSO threadgroup sizes (`[16,16,1]` for 2D clear/upscale,
/// `[256,1,1]` for buffer fill, `[64,1,1]` for indirect expanders).
/// Default **on** (`AW2_FAST_BIND_PSO=0` disables).
pub fn fast_bind_pso() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_FAST_BIND_PSO").is_none_or(|v| v != "0"))
}

/// `AW2_SHADER_IMMUTABLE_OPT`: specialize Metal 4 compute/graphics/mesh PSO descriptors
/// (`MTLShaderValidation::Disabled`, `MTL4ShaderReflection::None`, `support*BinaryLinking = false`,
/// compute/mesh `MTL4IndirectCommandBufferSupportState::Disabled`, explicit `Identity` color
/// attachment mapping, mesh SIMD32 execution-width hints) and force `MTLMathMode::Fast` on
/// internal MSL helper libraries, while keeping `.aw2s` DXIL->metallib disk cache keys untouched.
/// Default **on** (`AW2_SHADER_IMMUTABLE_OPT=0` disables).
pub fn shader_immutable_opt() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_SHADER_IMMUTABLE_OPT").is_none_or(|v| v != "0"))
}

/// PSO-cache key salt when `AW2_SHADER_IMMUTABLE_OPT=1` is active (`"IMMUT_OPT1"`).
pub const IMMUTABLE_OPT_KEY_SALT: u128 = 0x494d_4d55_545f_4f50_5431_u128;

/// `AW2_SHADER_PSO_OPT_V2=1`: refine SM 6.6 `GraphicsPipeline` / `MeshPipeline`
/// `writes_uav` and `pre_raster_reads` from MSC's cached `"ResourceUsages"` reflection
/// bitmask (zero `.aw2s` cold misses), and specialize Metal 4 Render/Mesh PSO descriptors
/// (elide dead AGX TBDR blend-read preambles on `write_mask == None` or identity
/// `1*Src + 0*Dst` attachments, pin `maxVertexAmplificationCount = 1` and
/// `MTL4AlphaToOneState::Disabled`).
/// Default **off** (`AW2_SHADER_PSO_OPT_V2=1` enables).
pub fn shader_pso_opt_v2() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_SHADER_PSO_OPT_V2").is_some_and(|v| v == "1"))
}

/// PSO-cache key salt when `AW2_SHADER_PSO_OPT_V2=1` is active (`"PSO_OPT2"`).
pub const SHADER_PSO_OPT_V2_SALT: u128 = 0x5053_4f5f_4f50_5432_u128;

/// `AW2_MSC_COMPUTE_ACCESS=1` (default off): refine `ComputeAccess` (`uav_slots`,
/// `read_slots`, `bounded_read_slots`, `uav_classes`, `read_classes`) from MSC's
/// cached `"UsedResources"` and `"ResourceUsages"` reflection JSON without
/// touching `.aw2s` shader cache keys or PSO archive keys.
pub fn msc_compute_access() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_MSC_COMPUTE_ACCESS").is_some_and(|v| v == "1"))
}

/// `AW2_MSC_STAGE_USAGE=1` (default off): refine `GraphicsPipeline` / `MeshPipeline`
/// `pre_raster_reads` & `writes_uav` and `ComputeAccess` `uav_classes` / `read_classes` /
/// `uav_srv_classes` from MSC's cached `"ResourceUsages"` JSON with zero `.aw2s` or PSO
/// archive cache key changes (`0` cold PSO compiles).
pub fn msc_stage_usage() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_MSC_STAGE_USAGE").is_some_and(|v| v == "1"))
}

/// `AW2_AIR_NO_RES_HEAP=1` (default off): when a cached `.aw2s` compute shader's embedded
/// `.metallib` `RBUF` reflection proves `struct.res_desc_heap_ab` (`buffer(0)`) is unused
/// (`unused == 1`), bypass DXIL `-HV 2021` `SFI0` `RESOURCE_HEAP_INDEXING` opacity and
/// refine `ComputeAccess` (`uav_slots`, `read_slots`, `bounded_read_slots`) from MSC
/// `"UsedResources"` + `"ResourceUsages"` with zero PSO salt.
pub fn air_no_res_heap() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_AIR_NO_RES_HEAP").is_some_and(|v| v == "1"))
}

/// `AW2_SM66_GFX_STAGE_EXACT` (default **on**; `AW2_SM66_GFX_STAGE_EXACT=0` disables):
/// zero-salt MSC `"ResourceUsages"` + `metallib_res_desc_heap_unused` + `"UsedResources"`
/// CBV/SRV/UAV inspection on SM 6.5 / SM 6.6 `GraphicsPipeline` and `MeshPipeline` stages
/// (`0` cold PSO compiles).
pub fn sm66_gfx_stage_exact() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_SM66_GFX_STAGE_EXACT").is_none_or(|v| v != "0"))
}

/// `AW2_CS_MID_QB_EXACT_V2=1` (default off): zero-salt MSC `"ResourceUsages"` +
/// `metallib_res_desc_heap_unused` compute stage classification paired with sound
/// `stage_gen`-stamped mid-compute `Transition` deferral (`0` cold PSO compiles).
pub fn cs_mid_qb_exact_v2() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_CS_MID_QB_EXACT_V2").is_some_and(|v| v == "1"))
}

/// `AW2_SM66_CS_STAGE_EXACT` (default **on**, `AW2_SM66_CS_STAGE_EXACT=0` disables): zero-salt
/// compute shader reflection exactness (`UsedResources` + `ResourceUsages` +
/// `metallib_res_desc_heap_unused` + exact CBV buffer check) on `ComputeAccess` without
/// `cs_mid_qb_exact_v2`'s `cs_end_after` side-effects.
pub fn sm66_cs_stage_exact() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_SM66_CS_STAGE_EXACT").is_none_or(|v| v != "0"))
}

static ANY_FRAG_BUF_WRITE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// True once any compiled pixel shader with buffer UAV writes (`(ru & 4) != 0` or opaque UAV)
/// or any occlusion query (`visibilityResultBuffer`) has been created/started in this process.
#[inline]
pub fn any_frag_buf_write() -> bool {
    ANY_FRAG_BUF_WRITE.load(std::sync::atomic::Ordering::Relaxed)
}

#[inline]
pub fn note_frag_buf_write() {
    ANY_FRAG_BUF_WRITE.store(true, std::sync::atomic::Ordering::Relaxed);
}

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

/// Applies `AW2_SHADER_IMMUTABLE_OPT=1` compile-time options to any `MTL4PipelineDescriptor`.
pub fn apply_immutable_pipeline_options(desc: &MTL4PipelineDescriptor) {
    let opts = MTL4PipelineOptions::new();
    opts.setShaderValidation(MTLShaderValidation::Disabled);
    opts.setShaderReflection(MTL4ShaderReflection::None);
    desc.setOptions(Some(&opts));
}

/// Builds a `MTLComputePipelineState` for `function_name` in `lib` through the `MTL4Compiler`
/// and `MTL4Archive` PSO cache.
pub fn compute_pipeline_from_function(lib: &MtlLibrary, function_name: &str) -> Result<Retained<MtlComputePipeline>, String> {
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
    if tg.is_none() && !immut_opt {
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
    pso_cache()?
        .compute_pipeline(Some(cache_key), &desc)
        .map_err(|e| format!("{function_name}: {e}"))
}

pub fn function_descriptor(lib: &MtlLibrary, name: &str) -> Retained<MTL4LibraryFunctionDescriptor> {
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
pub fn shader_function_descriptor(lib: &MtlLibrary, shader: &CompiledShader) -> Retained<MTL4FunctionDescriptor> {
    specialized_function_descriptor(lib, &shader.function_name, shader, &[])
}

/// Convert one stage of the game's DXIL against `rs`.
pub fn compile(stage: ShaderStage, dxil: &[u8], rs: &RootSignature, opts: &CompileOptions) -> Result<Arc<CompiledShader>, String> {
    let p = ShaderPipeline::global().map_err(|e| format!("shader pipeline: {e}"))?;
    p.compile_stage(stage, dxil, Some(rs), opts).map_err(|e| format!("{stage:?} shader: {e}"))
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
            super::encstats::inc(&super::encstats::c::air_no_res_heap_pso);
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
            super::encstats::inc(&super::encstats::c::msc_stage_usage_hit);
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
        mut acc: Self,
        dxil: &[u8],
        rs: &RootSignature,
        msc_json: &str,
        ru_opt: Option<u32>,
        enable_used_resources: bool,
        ignore_heap_flag: bool,
    ) -> Self {
        use dxil_metal::root_signature::{
            DescriptorRangeType, RootDescriptorKind, RootParameterLayoutKind,
        };
        const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
        const MAX_BOUNDED_ARRAY: u32 = 128;

        let was_uav_none = acc.uav_slots.is_none();
        let was_read_none = acc.read_slots.is_none();
        let prev_uav_slots = acc.uav_slots.clone();
        let prev_read_slots = acc.read_slots.clone();
        let prev_bounded_reads = acc.bounded_read_slots.clone();

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
        let effective_heap_indexed = heap_indexed && !ignore_heap_flag;
        let psv_opt = container_opt.as_ref().and_then(|c| c.psv_resources());
        let layout = rs.layout();

        let mut msc_applied = false;
        let mut msc_has_cbv = false;

        // Fix 1: `"UsedResources"` resolves statically-indexed entries inside unbounded
        // root-signature descriptor tables whenever SM 6.6 `RESOURCE_HEAP_INDEXING` is off
        // (or proven unused in AIR via `metallib_res_desc_heap_unused`).
        if enable_used_resources && !effective_heap_indexed {
            if let Some(used) = msc_used_resources(msc_json) {
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

                    'used_loop: for u in &used {
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
                                                    .as_ref()
                                                    .map(|psv| {
                                                        let mut c = 0u8;
                                                        for r in psv {
                                                            if !(3..=5).contains(&r.res_type)
                                                                || r.space != rg.register_space
                                                            {
                                                                continue;
                                                            }
                                                            let r_end = if r.upper_bound == u32::MAX
                                                            {
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
                                                        if c == 0 {
                                                            READS_TEX | READS_BUF
                                                        } else {
                                                            c
                                                        }
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
                        acc.read_slots =
                            msc_read_all_bounded.then(|| msc_bounded_reads.clone());
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
        } else if let Some(psv) = &psv_opt {
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
                let ro_c = (if (ru & (2 | 32)) != 0 || may_read_cbv_resource {
                    READS_BUF
                } else {
                    0
                }) | (if (ru & 8) != 0 { READS_TEX } else { 0 });
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

    pub fn from_dxil(dxil: &[u8], rs: &RootSignature) -> Self {
        use dxil_metal::root_signature::{DescriptorRangeType, RootDescriptorKind, RootParameterLayoutKind};
        const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
        const MAX_BOUNDED_ARRAY: u32 = 128;

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
        if c.feature_flags() & RESOURCE_HEAP_INDEXING != 0 {
            return Self::opaque();
        }
        let Some(psv) = c.psv_resources() else {
            return Self::opaque();
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

        for r in &psv {
            if r.res_type == 1 {
                // Samplers are immutable/stateless in D3D12 (no ResourceBarrier transitions).
                continue;
            }
            let (is_uav, range_ty, root_kind, allow_constants) = match r.res_type {
                2 => (false, DescriptorRangeType::Cbv, RootDescriptorKind::Cbv, true),
                3..=5 => (false, DescriptorRangeType::Srv, RootDescriptorKind::Srv, false),
                6..=9 => (true, DescriptorRangeType::Uav, RootDescriptorKind::Uav, false),
                _ => return Self::opaque(),
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
                        return Self::opaque();
                    }
                    match &p.kind {
                        RootParameterLayoutKind::Constants { shader_register, register_space, .. }
                            if allow_constants && *register_space == r.space && *shader_register == reg =>
                        {
                            // Inline root 32-bit constants live in the uploaded TLAB, not a Resource.
                            matched = true;
                            break;
                        }
                        RootParameterLayoutKind::Descriptor { kind, shader_register, register_space }
                            if *kind == root_kind && *register_space == r.space && *shader_register == reg =>
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
                                    && rg.num_descriptors.is_none_or(|n| reg - rg.base_shader_register < n)
                            }) {
                                let byte_offset = rg.byte_offset + (reg - rg.base_shader_register) as u64 * 24;
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
                    return Self::opaque();
                }
            }
        }

        let touches_textures = psv_touches_textures || (unbounded_srv_classes & READS_TEX) != 0;
        Self {
            uav_slots: uav_ok.then(|| uav_slots),
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
}

/// `AW2_CS_PSO_TG`: specialize `MTL4ComputePipelineDescriptor` with DXIL `[numthreads(x, y, z)]`
/// (`maxTotalThreadsPerThreadgroup`, `requiredThreadsPerThreadgroup`, and SIMD32 multiple hint).
/// Default **on** (`AW2_CS_PSO_TG=0` disables).
pub fn cs_pso_tg() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_CS_PSO_TG").is_none_or(|v| v != "0"))
}

pub struct ComputePipeline {
    pub pso: Retained<MtlComputePipeline>,
    pub rs: Arc<RootSignature>,
    pub threadgroup: [u32; 3],
    pub shader: Arc<CompiledShader>,
    pub access: ComputeAccess,
}
unsafe impl Send for ComputePipeline {}
unsafe impl Sync for ComputePipeline {}

impl ComputePipeline {
    pub fn new(rs: Arc<RootSignature>, dxil: &[u8], label: &str) -> Result<ComputePipeline, String> {
        Self::new_with_all_opts(rs, dxil, label, cs_pso_tg(), shader_immutable_opt())
    }

    pub fn new_with_opts(rs: Arc<RootSignature>, dxil: &[u8], label: &str, pin_tg: bool) -> Result<ComputePipeline, String> {
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
        super::pso_manifest::note_compute(&rs_for_manifest, dxil, pin_tg, immut_opt);
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
            ref other => return Err(format!("compute shader without compute reflection: {other:?}")),
        };
        let access = ComputeAccess::from_dxil_and_shader(dxil, &rs, &shader);
        let lib = library(&shader)?;
        let desc = MTL4ComputePipelineDescriptor::new();
        desc.setComputeFunctionDescriptor(Some(&shader_function_descriptor(&lib, &shader)));
        if !label.is_empty() {
            desc.setLabel(Some(&NSString::from_str(label)));
        } else if let Some(l) = super::gpucapture::pipeline_label(label, "cs", shader.key as u64) {
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
        let pso = cache.compute_pipeline(Some(cache_key), &desc).map_err(|e| format!("compute pipeline: {e}"))?;
        Ok(ComputePipeline { pso, rs, threadgroup, shader, access })
    }
}

// ---------------------------------------------------------------------------
// Graphics pipelines
// ---------------------------------------------------------------------------

use crate::state::{BlendDesc, BlendState, DepthStencilState, RasterState};
use dxil_metal::InputElement;

pub type MtlDepthStencil = ProtocolObject<dyn MTLDepthStencilState>;

/// Metal buffer index of input-assembler slot `slot` (converter
/// `kIRVertexBufferBindPoint` convention).
pub fn vertex_buffer_index(slot: u32) -> usize {
    dxil_metal::runtime::VERTEX_BUFFER_BIND_POINT as usize + slot as usize
}
/// Highest usable IA slot (Metal has 31 buffer bind points).
pub const MAX_VERTEX_SLOTS: u32 = 24;

/// `D3D12_GRAPHICS_PIPELINE_STATE_DESC` (or a flattened stream), parsed.
pub struct GraphicsDesc<'a> {
    pub vs: &'a [u8],
    pub ps: Option<&'a [u8]>,
    pub blend: BlendDesc,
    pub sample_mask: u32,
    pub raster: RasterState,
    pub depth_stencil: DepthStencilState,
    pub input_layout: Vec<InputElement>,
    pub strip_cut: u32,
    pub topology_type: u32,
    /// `NumRenderTargets` entries.
    pub rtv_formats: Vec<u32>,
    pub dsv_format: u32,
    pub sample_count: u32,
}

/// `AW2_GFX_LAZY_ICB`: compile graphics PSOs with `MTL4IndirectCommandBufferSupportState::Disabled`
/// by default and lazily compile an `Enabled` variant only when `execute_indirect_icb` uses the PSO.
/// Default **on** (`AW2_GFX_LAZY_ICB=0` disables).
pub fn gfx_lazy_icb() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("AW2_GFX_LAZY_ICB").is_none_or(|v| v != "0"))
}

/// Metal 4 object+mesh geometry shader emulation metadata attached to a [`GraphicsPipeline`]
/// built via [`GraphicsPipeline::new_with_geometry`].
pub struct GeometryEmulation {
    pub gs: Arc<CompiledShader>,
    pub stage_in: Option<Arc<CompiledShader>>,
    pub vertex_output_size_in_bytes: u32,
    pub max_input_primitives_per_mesh_threadgroup: u32,
    pub instance_count: u32,
}

pub struct GraphicsPipeline {
    pub pso: Retained<MtlRenderPipeline>,
    pub rs: Arc<RootSignature>,
    pub raster: RasterState,
    pub blend: BlendState,
    pub depth_stencil: Retained<MtlDepthStencil>,
    pub depth_bounds: bool,
    pub needs_draw_params: bool,
    pub topology_type: u32,
    pub strip_cut: u32,
    pub rtv_formats: Vec<u32>,
    pub dsv_format: u32,
    pub sample_count: u32,
    /// IA slots the vertex descriptor reads (bit per slot).
    pub vertex_slots: u32,
    pub vs: Arc<CompiledShader>,
    pub ps: Option<Arc<CompiledShader>>,
    /// Resource classes the vertex stage may read through SRVs/UAVs
    /// ([`stage_reads_resources`]).
    pub pre_raster_reads: u8,
    /// Whether the vertex stage reads any IA vertex buffer slot (`vertex_slots != 0`)
    /// or any non-inline CBV resource (`AW2_VS_BUF_DEFER_EXACT=1`).
    pub vs_reads_cbv_or_ia: bool,
    /// The vertex or pixel stage may write through UAVs ([`stage_writes_uav`]).
    pub writes_uav: bool,
    /// Zero-salt MSC + AIR reflection exact stage flags (`AW2_SM66_GFX_STAGE_EXACT=1`).
    pub sm66_pre_raster_reads: u8,
    pub sm66_vs_reads_cbv_or_ia: bool,
    pub sm66_writes_uav: bool,
    pub sm66_narrowed: bool,
    /// Whether the vertex stage alone may write through UAVs (`AW2_LIST_START_RENDER_SPLIT=1`).
    pub vs_writes_uav: bool,
    /// Whether the pixel stage alone may write through UAVs (`AW2_RP_KEEP_WRITE_EXACT=1`).
    pub ps_writes_uav: bool,
    /// Resource classes (`READS_TEX | READS_BUF`, including CBVs) the pixel stage may read
    /// (`0` for depth-only `ps == None` passes; `AW2_RP_KEEP_WRITE_EXACT=1`).
    pub ps_read_classes: u8,
    /// SRV/UAV-only resource classes (`READS_TEX | READS_BUF`, excluding CBVs) the pixel stage
    /// may read (`AW2_PS_SRV_CLASS_EXACT=1`).
    pub ps_srv_uav_classes: u8,
    /// Whether either stage may read or write any UAV (`AW2_RP_KEEP_WRITE_EXACT=1`).
    pub uses_uav: bool,
    /// Zero-salt per-stage binding profiles (`AW2_GFX_DESC_TABLE_EXACT=1`).
    pub vs_access: ComputeAccess,
    pub ps_access: Option<ComputeAccess>,
    /// Present when this pipeline emulates a D3D11/D3D12 geometry shader on a Metal 4
    /// mesh render pipeline (object = VS + linked stage-in, mesh = GS, fragment = PS).
    pub gs_emu: Option<GeometryEmulation>,
    pub(crate) icb_desc: Option<(Retained<MTL4RenderPipelineDescriptor>, u128)>,
    pub(crate) icb_pso: OnceLock<Result<Retained<MtlRenderPipeline>, String>>,
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
    use dxil_metal::root_signature::{
        DescriptorRangeType, RootDescriptorKind, RootParameterLayoutKind,
    };
    if !metallib_res_desc_heap_unused(metallib) {
        return None;
    }
    let used = msc_used_resources(msc_json)?;
    let layout = rs.layout();
    let param_len = layout.parameters.len();
    let has_static_samplers = layout.static_samplers.is_some();
    let mut has_cbv = false;
    for u in &used {
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
    use dxil_metal::root_signature::{
        DescriptorRangeType, RootDescriptorKind, RootParameterLayoutKind,
    };
    const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
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
        let overlaps_cbv_res = layout.parameters.iter().any(|p| match &p.kind {
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
        });
        overlaps_cbv_res
    })
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
    const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
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
    use dxil_metal::root_signature::{
        DescriptorRangeType, RootDescriptorKind, RootParameterLayoutKind,
    };
    const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
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
    let layout = rs.layout();
    let param_len = layout.parameters.len();
    let has_static_samplers = layout.static_samplers.is_some();
    for u in &used {
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

/// [`stage_reads_resources`] class bits.
pub const READS_TEX: u8 = 1;
pub const READS_BUF: u8 = 2;

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
    use dxil_metal::root_signature::ResourceType as T;
    const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
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
    use dxil_metal::root_signature::{
        DescriptorRangeType, RootDescriptorKind, RootParameterLayoutKind,
    };
    let base = stage_reads_resources_with(sh, bytecode, opt_v2);
    if base == 0 || !opt_v2 || !metallib_res_desc_heap_unused(&sh.metallib) {
        return base;
    }
    let Some(used) = msc_used_resources(sh.reflection.json.as_str()) else {
        return base;
    };
    let layout = rs.layout();
    let param_len = layout.parameters.len();
    let has_static_samplers = layout.static_samplers.is_some();
    let mut m = 0u8;
    for u in &used {
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
unsafe impl Send for GraphicsPipeline {}
unsafe impl Sync for GraphicsPipeline {}

struct DsCache(Mutex<HashMap<DepthStencilState, Retained<MtlDepthStencil>>>);
unsafe impl Send for DsCache {}
unsafe impl Sync for DsCache {}

/// The `MTLDepthStencilState` for `s` (cached for the process lifetime).
pub fn depth_stencil_state(s: &DepthStencilState) -> Result<Retained<MtlDepthStencil>, String> {
    static C: OnceLock<DsCache> = OnceLock::new();
    let c = C.get_or_init(|| DsCache(Mutex::new(HashMap::new())));
    let mut m = c.0.lock().unwrap();
    if let Some(d) = m.get(s) {
        return Ok(d.clone());
    }
    let d = super::device().ok_or("no device")?;
    let desc = MTLDepthStencilDescriptor::new();
    s.apply_to(&desc);
    let st = d.dev.newDepthStencilStateWithDescriptor(&desc).ok_or_else(|| format!("newDepthStencilState({s:?})"))?;
    m.insert(*s, st.clone());
    Ok(st)
}

fn semantic_matches(reflected: &str, name: &str, index: u32) -> bool {
    // The converter reports e.g. "TEXCOORD3" (index appended).
    let n = name.len();
    reflected.len() >= n && reflected[..n].eq_ignore_ascii_case(name) && reflected[n..].parse::<u32>().unwrap_or(0) == index && (reflected.len() > n || index == 0)
}

/// Infers a 4-component DXGI format (`R32G32B32A32_FLOAT` = 2, `_UINT` = 3, `_SINT` = 4)
/// for a vertex shader input from the DXBC `ISG1` / `ISGN` input signature chunk when the
/// PSO's `input_layout` omits that attribute.
pub fn infer_vertex_input_format(vs_dxil: &[u8], reflected_name: &str) -> u32 {
    let rd32 = |b: &[u8], o: usize| b.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()));
    if let Ok(c) = dxil_metal::dxbc::Container::parse(vs_dxil) {
        for (fcc, stride, name_off_pos, idx_pos, comp_pos) in [
            (*b"ISG1", 32usize, 4usize, 8usize, 16usize),
            (*b"ISGN", 24usize, 0usize, 4usize, 12usize),
        ] {
            if let Some(d) = c.part(fcc) {
                let count = rd32(d, 0).unwrap_or(0) as usize;
                for i in 0..count {
                    let base = 8 + i * stride;
                    let (Some(noff), Some(idx), Some(comp)) = (
                        rd32(d, base + name_off_pos),
                        rd32(d, base + idx_pos),
                        rd32(d, base + comp_pos),
                    ) else {
                        break;
                    };
                    if let Some(tail) = d.get(noff as usize..) {
                        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
                        let sem = std::str::from_utf8(&tail[..end]).unwrap_or("");
                        if semantic_matches(reflected_name, sem, idx) {
                            return match comp {
                                1 => 3, // DXGI_FORMAT_R32G32B32A32_UINT
                                2 => 4, // DXGI_FORMAT_R32G32B32A32_SINT
                                _ => 2, // DXGI_FORMAT_R32G32B32A32_FLOAT
                            };
                        }
                    }
                }
            }
        }
    }
    2 // DXGI_FORMAT_R32G32B32A32_FLOAT
}

impl GraphicsPipeline {
    pub fn new(rs: Arc<RootSignature>, d: &GraphicsDesc, label: &str) -> Result<GraphicsPipeline, String> {
        Self::new_with_pso_opts(rs, d, label, gfx_lazy_icb(), shader_immutable_opt(), shader_pso_opt_v2())
    }

    /// Build a graphics pipeline that includes a D3D11/D3D12 Geometry Shader (`gs`) using
    /// Apple Metal Shader Converter geometry emulation on a Metal 4 mesh render pipeline:
    /// VS (`<name>.dxil_irconverter_object_shader`) + linked stage-in function in the object
    /// stage, GS in the mesh stage, and optional PS in the fragment stage.
    pub fn new_with_geometry(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        gs_dxil: &[u8],
        label: &str,
    ) -> Result<GraphicsPipeline, String> {
        Self::new_geometry_in_cache(
            pso_cache()?,
            rs,
            d,
            gs_dxil,
            label,
            shader_immutable_opt(),
            shader_pso_opt_v2(),
        )
    }

    fn new_geometry_in_cache(
        cache: &PipelineCache,
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        gs_dxil: &[u8],
        label: &str,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let blend = crate::state::blend_state(&d.blend);
        if !blend.unsupported_logic_ops.is_empty() {
            log::warn!(target: "d3d", "geometry PSO: unsupported logic ops {:?} (blending off)", blend.unsupported_logic_ops.as_slice());
        }
        let mut vs_opts = CompileOptions::default();
        vs_opts.geometry_tessellation_emulation = true;
        vs_opts.stage_in_mode = StageInMode::SeparateFunction;
        vs_opts.input_topology = Some(crate::topology::input_topology(d.topology_type));
        let vs = compile(ShaderStage::Vertex, d.vs, &rs, &vs_opts)?;
        let StageInfo::Vertex { needs_draw_params, vertex_output_size_in_bytes, inputs, .. } = &vs.reflection.info else {
            return Err(format!("geometry PSO: vertex shader without vertex reflection: {:?}", vs.reflection.info));
        };
        let vertex_output_size = *vertex_output_size_in_bytes;

        let mut layout = d.input_layout.clone();
        dxil_metal::compiler::resolve_append_aligned(&mut layout, crate::format_ext::vertex_format_size);
        let mut vertex_slots = 0u32;
        for inp in inputs {
            if let Some(e) = layout.iter().find(|e| semantic_matches(&inp.name, &e.semantic_name, e.semantic_index)) {
                if e.input_slot < MAX_VERTEX_SLOTS {
                    vertex_slots |= 1 << e.input_slot;
                }
            } else {
                layout.push(InputElement {
                    semantic_name: inp.name.clone(),
                    semantic_index: 0,
                    format: infer_vertex_input_format(d.vs, &inp.name),
                    input_slot: MAX_VERTEX_SLOTS - 1,
                    aligned_byte_offset: 0,
                    input_slot_class: 1,
                    instance_data_step_rate: 0,
                });
                vertex_slots |= 1 << (MAX_VERTEX_SLOTS - 1);
            }
        }
        let stage_in = if !inputs.is_empty() || !layout.is_empty() {
            let sp = ShaderPipeline::global().map_err(|e| format!("shader pipeline: {e}"))?;
            Some(sp.stage_in(&vs, &layout, &vs_opts).map_err(|e| format!("stage-in synthesis: {e}"))?)
        } else {
            // Even for input-less VS, synthesize stage-in if MSC succeeds (mirrors IRRuntimeNewGeometryEmulationPipeline).
            ShaderPipeline::global().ok().and_then(|sp| sp.stage_in(&vs, &[], &vs_opts).ok())
        };

        let mut gs_opts = CompileOptions::default();
        gs_opts.geometry_tessellation_emulation = true;
        let gs = compile(ShaderStage::Geometry, gs_dxil, &rs, &gs_opts)?;
        let (max_input_prims, gs_instance_count) = match &gs.reflection.info {
            StageInfo::Geometry { max_input_primitives_per_mesh_threadgroup, instance_count, .. } => {
                ((*max_input_primitives_per_mesh_threadgroup).max(1), (*instance_count).max(1))
            }
            other => return Err(format!("geometry PSO: geometry shader without geometry reflection: {other:?}")),
        };

        let mut ps_opts = crate::state::pixel_compile_options(&blend, &d.rtv_formats);
        let full = if d.sample_count >= 32 { u32::MAX } else { (1u32 << d.sample_count.max(1)) - 1 };
        if d.sample_mask & full != full {
            ps_opts.sample_mask = Some(d.sample_mask);
        }
        let ps = match d.ps {
            Some(b) => Some(compile(ShaderStage::Fragment, b, &rs, &ps_opts)?),
            None => None,
        };

        let desc = MTL4MeshRenderPipelineDescriptor::new();
        let vlib = library(&vs)?;
        let obj_name = format!("{}.dxil_irconverter_object_shader", vs.function_name);
        let tess_off: [u8; 1] = [0];
        let obj_fn = specialized_function_descriptor(
            &vlib,
            &obj_name,
            &vs,
            &[("tessellationEnabled", MTLDataType::Bool, &tess_off)],
        );
        desc.setObjectFunctionDescriptor(Some(&obj_fn));

        if let Some(si) = &stage_in {
            let silib = library(si)?;
            let sifn: Retained<MTL4FunctionDescriptor> =
                Retained::into_super(function_descriptor(&silib, &si.function_name));
            let linked = NSArray::from_retained_slice(&[sifn]);
            let linking = MTL4StaticLinkingDescriptor::new();
            linking.setFunctionDescriptors(Some(&linked));
            desc.setObjectStaticLinkingDescriptor(Some(&linking));
        }

        let glib = library(&gs)?;
        let vs_out_bytes = (vertex_output_size as i32).to_ne_bytes();
        let so_off: [u8; 1] = [0];
        let mesh_fn = specialized_function_descriptor(
            &glib,
            &gs.function_name,
            &gs,
            &[
                ("tessellationEnabled", MTLDataType::Bool, &tess_off),
                ("streamOutEnabled", MTLDataType::Bool, &so_off),
                ("vertex_shader_output_size_fc", MTLDataType::Int, &vs_out_bytes),
            ],
        );
        desc.setMeshFunctionDescriptor(Some(&mesh_fn));

        if let Some(ps) = &ps {
            let plib = library(ps)?;
            desc.setFragmentFunctionDescriptor(Some(&shader_function_descriptor(&plib, ps)));
        }

        if !label.is_empty() {
            desc.setLabel(Some(&NSString::from_str(label)));
        }
        desc.setPayloadMemoryLength(16384);
        desc.setMaxTotalThreadgroupsPerMeshGrid(crate::mesh::MAX_MESH_GRID_THREADGROUPS as usize);
        if immut_opt {
            apply_immutable_pipeline_options(&desc);
            desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Disabled);
            desc.setSupportObjectBinaryLinking(false);
            desc.setSupportMeshBinaryLinking(false);
            desc.setSupportFragmentBinaryLinking(false);
            desc.setColorAttachmentMappingState(MTL4LogicalToPhysicalColorAttachmentMappingState::Identity);
        }
        if pso_opt_v2 {
            unsafe { desc.setMaxVertexAmplificationCount(1) };
            desc.setAlphaToOneState(MTL4AlphaToOneState::Disabled);
        }

        let ca = desc.colorAttachments();
        for (i, f) in d.rtv_formats.iter().enumerate().take(8) {
            let a = unsafe { ca.objectAtIndexedSubscript(i) };
            let pf = crate::format::info(*f).mtl;
            a.setPixelFormat(pf);
            if pf != MTLPixelFormat::Invalid {
                blend.attachments[i].apply_to_with_opt(&a, pso_opt_v2);
            }
        }
        unsafe { desc.setRasterSampleCount(d.sample_count.max(1) as usize) };
        desc.setAlphaToCoverageState(if blend.alpha_to_coverage {
            MTL4AlphaToCoverageState::Enabled
        } else {
            MTL4AlphaToCoverageState::Disabled
        });

        let mut key = dxil_metal::hash::KeyHasher::new("d3d12-metal gs-emu pso v1");
        key.u128(vs.key)
            .u128(gs.key)
            .u128(stage_in.as_ref().map_or(0, |s| s.key))
            .u128(ps.as_ref().map_or(0, |p| p.key))
            .u32(vertex_output_size);
        key.bytes(format!("{blend:?}|{:?}|{}|{}", d.rtv_formats, d.sample_count, d.topology_type).as_bytes());
        let mut pso_key = key.finish();
        if immut_opt {
            pso_key ^= IMMUTABLE_OPT_KEY_SALT;
        }
        if pso_opt_v2 {
            pso_key ^= SHADER_PSO_OPT_V2_SALT;
        }
        let pso = cache.render_pipeline(Some(pso_key), &desc).map_err(|e| format!("geometry render pipeline: {e}"))?;
        let ds = if d.dsv_format == 0 { DepthStencilState::without_depth_stencil() } else { d.depth_stencil };
        let pre_raster_reads = stage_reads_resources_with(&vs, d.vs, pso_opt_v2)
            | stage_reads_resources_with(&gs, gs_dxil, pso_opt_v2);
        let vs_writes_uav = shader_writes_uav(&vs, d.vs, true) || shader_writes_uav(&gs, gs_dxil, true);
        let ps_writes_uav = ps.as_ref().zip(d.ps).is_some_and(|(p, b)| shader_writes_uav(p, b, true));
        let writes_uav = vs_writes_uav || ps_writes_uav;
        let (ps_srv_uav_classes, ps_read_classes) = match ps.as_ref().zip(d.ps) {
            Some((p, b)) => {
                let srv_uav = stage_srv_uav_classes(p, b, &rs, true);
                let mut m = srv_uav;
                if stage_may_read_cbv_buffer_with(p, b, &rs, true) {
                    m |= READS_BUF;
                }
                (srv_uav, m)
            }
            None => (0, 0),
        };
        let uses_uav = writes_uav
            || shader_uses_uav(&vs, d.vs, &rs)
            || shader_uses_uav(&gs, gs_dxil, &rs)
            || ps.as_ref().zip(d.ps).is_some_and(|(p, b)| shader_uses_uav(p, b, &rs));
        let vs_access = ComputeAccess::for_stage_shader(d.vs, &rs, &vs, true);
        let ps_access = ps.as_ref().zip(d.ps).map(|(p, b)| ComputeAccess::for_stage_shader(b, &rs, p, false));
        Ok(GraphicsPipeline {
            pso,
            rs,
            raster: d.raster,
            blend,
            depth_stencil: depth_stencil_state(&ds)?,
            depth_bounds: ds.depth_bounds,
            needs_draw_params: *needs_draw_params,
            topology_type: d.topology_type,
            strip_cut: d.strip_cut,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count.max(1),
            vertex_slots,
            pre_raster_reads,
            vs_reads_cbv_or_ia: true,
            writes_uav,
            sm66_pre_raster_reads: pre_raster_reads,
            sm66_vs_reads_cbv_or_ia: true,
            sm66_writes_uav: writes_uav,
            sm66_narrowed: false,
            vs_writes_uav,
            ps_writes_uav,
            ps_read_classes,
            ps_srv_uav_classes,
            uses_uav,
            vs_access,
            ps_access,
            vs,
            ps,
            gs_emu: Some(GeometryEmulation {
                gs,
                stage_in,
                vertex_output_size_in_bytes: vertex_output_size,
                max_input_primitives_per_mesh_threadgroup: max_input_prims,
                instance_count: gs_instance_count,
            }),
            icb_desc: None,
            icb_pso: OnceLock::new(),
        })
    }

    pub fn new_with_opts(rs: Arc<RootSignature>, d: &GraphicsDesc, label: &str, lazy_icb: bool) -> Result<GraphicsPipeline, String> {
        Self::new_with_pso_opts(rs, d, label, lazy_icb, shader_immutable_opt(), shader_pso_opt_v2())
    }

    pub fn new_with_all_opts(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
        lazy_icb: bool,
        immut_opt: bool,
    ) -> Result<GraphicsPipeline, String> {
        Self::new_with_pso_opts(rs, d, label, lazy_icb, immut_opt, shader_pso_opt_v2())
    }

    pub fn new_with_pso_opts(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
        lazy_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let rs_for_manifest = rs.clone();
        let out = Self::new_in_cache(pso_cache()?, rs, d, label, lazy_icb, immut_opt, pso_opt_v2)?;
        super::pso_manifest::note_graphics(&rs_for_manifest, d, lazy_icb, immut_opt, pso_opt_v2);
        Ok(out)
    }

    pub(crate) fn new_in_cache(
        cache: &PipelineCache,
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
        lazy_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let blend = crate::state::blend_state(&d.blend);
        if !blend.unsupported_logic_ops.is_empty() {
            log::warn!(target: "d3d", "graphics PSO: unsupported logic ops {:?} (blending off)", blend.unsupported_logic_ops.as_slice());
        }
        let vs = compile(ShaderStage::Vertex, d.vs, &rs, &CompileOptions::default())?;
        let StageInfo::Vertex { needs_draw_params, inputs, .. } = &vs.reflection.info else {
            return Err(format!("vertex shader without vertex reflection: {:?}", vs.reflection.info));
        };
        let mut ps_opts = crate::state::pixel_compile_options(&blend, &d.rtv_formats);
        let full = if d.sample_count >= 32 { u32::MAX } else { (1u32 << d.sample_count.max(1)) - 1 };
        if d.sample_mask & full != full {
            ps_opts.sample_mask = Some(d.sample_mask);
        }
        let ps = match d.ps {
            Some(b) => Some(compile(ShaderStage::Fragment, b, &rs, &ps_opts)?),
            None => None,
        };

        let desc = MTL4RenderPipelineDescriptor::new();
        let vlib = library(&vs)?;
        desc.setVertexFunctionDescriptor(Some(&shader_function_descriptor(&vlib, &vs)));
        if let Some(ps) = &ps {
            let plib = library(ps)?;
            desc.setFragmentFunctionDescriptor(Some(&shader_function_descriptor(&plib, ps)));
        }
        if !label.is_empty() {
            desc.setLabel(Some(&NSString::from_str(label)));
        } else if let Some(l) = super::gpucapture::pipeline_label(label, "gfx", vs.key as u64 ^ ps.as_ref().map_or(0, |s| (s.key as u64).rotate_left(1))) {
            desc.setLabel(Some(&NSString::from_str(&l)));
        }
        if immut_opt {
            apply_immutable_pipeline_options(&desc);
            desc.setSupportVertexBinaryLinking(false);
            desc.setSupportFragmentBinaryLinking(false);
            desc.setColorAttachmentMappingState(MTL4LogicalToPhysicalColorAttachmentMappingState::Identity);
        }
        if pso_opt_v2 {
            unsafe { desc.setMaxVertexAmplificationCount(1) };
            desc.setAlphaToOneState(MTL4AlphaToOneState::Disabled);
        }

        // Vertex fetch: match the layout's elements to the shader's inputs.
        // If a PSO's input layout omits a declared VS input (e.g. an unused attribute or a
        // synthesized default layout), bind a constant step-rate fallback on slot MAX_VERTEX_SLOTS - 1
        // with the scalar format inferred from the DXBC input signature.
        let mut layout = d.input_layout.clone();
        dxil_metal::compiler::resolve_append_aligned(&mut layout, crate::format_ext::vertex_format_size);
        desc.setSupportIndirectCommandBuffers(if lazy_icb {
            MTL4IndirectCommandBufferSupportState::Disabled
        } else {
            MTL4IndirectCommandBufferSupportState::Enabled
        });
        let mut key = dxil_metal::hash::KeyHasher::new("d3d12-metal graphics pso v2");
        key.u128(vs.key).u128(ps.as_ref().map_or(0, |p| p.key));
        let mut vertex_slots = 0u32;
        if !inputs.is_empty() {
            let vd = MTLVertexDescriptor::new();
            for inp in inputs {
                let fallback;
                let e = match layout.iter().find(|e| semantic_matches(&inp.name, &e.semantic_name, e.semantic_index)) {
                    Some(e) => e,
                    None => {
                        fallback = InputElement {
                            semantic_name: inp.name.clone(),
                            semantic_index: 0,
                            format: infer_vertex_input_format(d.vs, &inp.name),
                            input_slot: MAX_VERTEX_SLOTS - 1,
                            aligned_byte_offset: 0,
                            input_slot_class: 1,
                            instance_data_step_rate: 0,
                        };
                        &fallback
                    }
                };
                if e.input_slot >= MAX_VERTEX_SLOTS {
                    return Err(format!("input slot {} (max {})", e.input_slot, MAX_VERTEX_SLOTS - 1));
                }
                let fmt = crate::format_ext::vertex_format(e.format).ok_or_else(|| format!("vertex format {} ({})", e.format, crate::format::info(e.format).name))?;
                let a = unsafe { vd.attributes().objectAtIndexedSubscript(dxil_metal::runtime::STAGE_IN_ATTRIBUTE_START_INDEX as usize + inp.attribute_index as usize) };
                a.setFormat(fmt);
                unsafe {
                    a.setOffset(e.aligned_byte_offset as usize);
                    a.setBufferIndex(vertex_buffer_index(e.input_slot));
                }
                let l = unsafe { vd.layouts().objectAtIndexedSubscript(vertex_buffer_index(e.input_slot)) };
                unsafe { l.setStride(usize::MAX) }; // MTLBufferLayoutStrideDynamic
                if e.input_slot_class == 1 {
                    if e.instance_data_step_rate == 0 {
                        l.setStepFunction(MTLVertexStepFunction::Constant);
                        unsafe { l.setStepRate(0) };
                    } else {
                        l.setStepFunction(MTLVertexStepFunction::PerInstance);
                        unsafe { l.setStepRate(e.instance_data_step_rate as usize) };
                    }
                }
                vertex_slots |= 1 << e.input_slot;
                key.u32(inp.attribute_index as u32).u32(e.format).u32(e.input_slot).u32(e.aligned_byte_offset).u32(e.input_slot_class).u32(e.instance_data_step_rate);
            }
            desc.setVertexDescriptor(Some(&vd));
        }

        let ca = desc.colorAttachments();
        for (i, f) in d.rtv_formats.iter().enumerate().take(8) {
            let a = unsafe { ca.objectAtIndexedSubscript(i) };
            let pf = crate::format::info(*f).mtl;
            a.setPixelFormat(pf);
            if pf != MTLPixelFormat::Invalid {
                blend.attachments[i].apply_to_with_opt(&a, pso_opt_v2);
            }
        }
        unsafe { desc.setRasterSampleCount(d.sample_count.max(1) as usize) };
        desc.setAlphaToCoverageState(if blend.alpha_to_coverage { MTL4AlphaToCoverageState::Enabled } else { MTL4AlphaToCoverageState::Disabled });
        desc.setInputPrimitiveTopology(crate::topology::topology_class(d.topology_type));
        key.bytes(format!("{blend:?}|{:?}|{}|{}", d.rtv_formats, d.sample_count, d.topology_type).as_bytes());

        let mut base_key = key.finish();
        if immut_opt {
            base_key ^= IMMUTABLE_OPT_KEY_SALT;
        }
        if pso_opt_v2 {
            base_key ^= SHADER_PSO_OPT_V2_SALT;
        }
        let primary_key = if lazy_icb { base_key ^ 0x4e4f_5f49_4342_5f31_u128 } else { base_key };
        let pso = cache.render_pipeline(Some(primary_key), &desc).map_err(|e| format!("render pipeline: {e}"))?;
        let icb_desc = lazy_icb.then_some((desc, base_key));
        let ds = if d.dsv_format == 0 { DepthStencilState::without_depth_stencil() } else { d.depth_stencil };
        if ds.depth_bounds {
            log::warn!(target: "d3d", "graphics PSO enables the depth bounds test, which is reported unsupported; ignored");
        }
        let msc_usage = msc_stage_usage();
        let sm66_gfx = sm66_gfx_stage_exact();
        let base_use_msc = pso_opt_v2 || msc_usage;
        let dxil_pre_raster = stage_reads_resources_with(&vs, d.vs, false);
        let base_pre_raster = if base_use_msc {
            stage_reads_resources_with(&vs, d.vs, true)
        } else {
            dxil_pre_raster
        };
        let sm66_pre_raster_reads = stage_srv_uav_classes(&vs, d.vs, &rs, true);
        let dxil_vs_cbv = vertex_slots != 0 || stage_may_read_cbv_buffer_with(&vs, d.vs, &rs, false);
        let base_vs_cbv = dxil_vs_cbv;
        let sm66_vs_reads_cbv_or_ia =
            vertex_slots != 0 || stage_may_read_cbv_buffer_with(&vs, d.vs, &rs, true);
        let dxil_writes_uav = shader_writes_uav(&vs, d.vs, false)
            || ps
                .as_ref()
                .zip(d.ps)
                .is_some_and(|(p, b)| shader_writes_uav(p, b, false));
        let base_writes_uav = if base_use_msc {
            shader_writes_uav(&vs, d.vs, true)
                || ps
                    .as_ref()
                    .zip(d.ps)
                    .is_some_and(|(p, b)| shader_writes_uav(p, b, true))
        } else {
            dxil_writes_uav
        };
        let vs_writes_uav = shader_writes_uav(&vs, d.vs, true);
        let ps_writes_uav = ps
            .as_ref()
            .zip(d.ps)
            .is_some_and(|(p, b)| shader_writes_uav(p, b, true));
        if ps
            .as_ref()
            .zip(d.ps)
            .is_some_and(|(p, b)| shader_writes_buf_uav(p, b))
        {
            note_frag_buf_write();
        }
        let sm66_writes_uav = vs_writes_uav || ps_writes_uav;
        let pre_raster_reads = if sm66_gfx { sm66_pre_raster_reads } else { base_pre_raster };
        let vs_reads_cbv_or_ia = if sm66_gfx { sm66_vs_reads_cbv_or_ia } else { base_vs_cbv };
        let writes_uav = if sm66_gfx { sm66_writes_uav } else { base_writes_uav };
        let (ps_srv_uav_classes, ps_read_classes) = match ps.as_ref().zip(d.ps) {
            Some((p, b)) => {
                let srv_uav = stage_srv_uav_classes(p, b, &rs, true);
                let mut m = srv_uav;
                if stage_may_read_cbv_buffer_with(p, b, &rs, true) {
                    m |= READS_BUF;
                }
                (srv_uav, m)
            }
            None => (0, 0),
        };
        let uses_uav = writes_uav
            || shader_uses_uav(&vs, d.vs, &rs)
            || ps
                .as_ref()
                .zip(d.ps)
                .is_some_and(|(p, b)| shader_uses_uav(p, b, &rs));
        let both_classes = READS_TEX | READS_BUF;
        let msc_proven = metallib_res_desc_heap_unused(&vs.metallib)
            && msc_resource_usages(&vs).is_some()
            && ps
                .as_ref()
                .is_none_or(|p| metallib_res_desc_heap_unused(&p.metallib) && msc_resource_usages(p).is_some());
        let sm66_narrowed = sm66_pre_raster_reads != dxil_pre_raster
            || sm66_vs_reads_cbv_or_ia != dxil_vs_cbv
            || sm66_writes_uav != dxil_writes_uav
            || (msc_proven
                && (sm66_pre_raster_reads != both_classes
                    || !sm66_writes_uav
                    || ps_read_classes != both_classes
                    || !uses_uav));
        if msc_usage
            && !pso_opt_v2
            && (pre_raster_reads != both_classes || !writes_uav)
        {
            super::encstats::inc(&super::encstats::c::msc_stage_usage_hit);
        }
        let vs_access = ComputeAccess::for_stage_shader(d.vs, &rs, &vs, true);
        let ps_access = ps
            .as_ref()
            .zip(d.ps)
            .map(|(p, b)| ComputeAccess::for_stage_shader(b, &rs, p, false));
        Ok(GraphicsPipeline {
            pso,
            rs,
            raster: d.raster,
            blend,
            depth_stencil: depth_stencil_state(&ds)?,
            depth_bounds: ds.depth_bounds,
            needs_draw_params: *needs_draw_params,
            topology_type: d.topology_type,
            strip_cut: d.strip_cut,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count.max(1),
            vertex_slots,
            pre_raster_reads,
            vs_reads_cbv_or_ia,
            writes_uav,
            sm66_pre_raster_reads,
            sm66_vs_reads_cbv_or_ia,
            sm66_writes_uav,
            sm66_narrowed,
            vs_writes_uav,
            ps_writes_uav,
            ps_read_classes,
            ps_srv_uav_classes,
            uses_uav,
            vs_access,
            ps_access,
            vs: vs.clone(),
            ps,
            gs_emu: None,
            icb_desc,
            icb_pso: OnceLock::new(),
        })
    }

    /// Returns an ICB-capable `MTLRenderPipelineState` (`supportIndirectCommandBuffers == Enabled`),
    /// compiling and caching it on first demand when `AW2_GFX_LAZY_ICB=1` is active.
    pub fn icb_pso(&self) -> Result<&MtlRenderPipeline, String> {
        let Some((desc, icb_key)) = &self.icb_desc else {
            return Ok(&self.pso);
        };
        self.icb_pso
            .get_or_init(|| {
                desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Enabled);
                pso_cache()?
                    .render_pipeline(Some(*icb_key), desc)
                    .map_err(|e| format!("render pipeline (icb): {e}"))
            })
            .as_deref()
            .map_err(Clone::clone)
    }
}

// ---------------------------------------------------------------------------
// Gate g50_A / g51_A: Zero-salt DXIL handle-kind (`!dx.resources` + `@dx.op.annotateHandle`)
// + MSC `ResourceUsages` / `UsedResources` profile
// ---------------------------------------------------------------------------

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
        uav_write_kinds: super::resource::HK_ALL & !super::resource::HK_CBV_BUF,
        uav_srv_kinds: super::resource::HK_ALL & !super::resource::HK_CBV_BUF,
        ro_srv_kinds: super::resource::HK_ALL & !super::resource::HK_CBV_BUF,
        read_kinds: super::resource::HK_ALL,
        unbounded_srv_kinds: super::resource::HK_ALL & !super::resource::HK_CBV_BUF,
    };

    /// Intersects `self` with coarse `ComputeAccess` class masks (`READS_TEX`, `READS_BUF`,
    /// `msc_exact_classes`, `unbounded_srv_classes`) so both real shaders and synthetic
    /// unit-test `ComputeAccess` structs stay 100% consistent.
    #[inline]
    pub fn intersect_access(mut self, acc: &ComputeAccess) -> Self {
        use super::resource::{HK_ALL_BUF, HK_ALL_SRV_UAV_BUF, HK_ALL_TEX, HK_CBV_BUF};
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
        use super::resource::{HK_ALL_BUF, HK_ALL_TEX, HK_CBV_BUF};
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

fn hk_profile_registry() -> &'static std::sync::RwLock<HashMap<u128, HandleKindProfile>> {
    static REG: OnceLock<std::sync::RwLock<HashMap<u128, HandleKindProfile>>> = OnceLock::new();
    REG.get_or_init(|| std::sync::RwLock::new(HashMap::new()))
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

#[cfg(test)]
pub fn register_shader_hk_profile_for_test(key: u128, prof: HandleKindProfile) {
    hk_profile_registry().write().unwrap().insert(key, prof);
}

fn register_shader_hk_profile(
    dxil: &[u8],
    rs: &RootSignature,
    shader: &CompiledShader,
    ru_opt: Option<u32>,
) {
    use super::cmdlist::{
        opt_cs_bind_intra_safe_env, opt_ps_to_cs_hk_overlap_env, opt_sm66_handle_kind_exact_env,
        opt_sm66_handle_kind_v2_env,
    };
    if shader.key == 0
        || !(opt_sm66_handle_kind_exact_env()
            || opt_sm66_handle_kind_v2_env()
            || opt_ps_to_cs_hk_overlap_env()
            || opt_cs_bind_intra_safe_env())
    {
        return;
    }
    if hk_profile_registry().read().unwrap().contains_key(&shader.key) {
        return;
    }
    let raw = build_handle_kind_profile_raw(dxil, rs, shader, ru_opt);
    hk_profile_registry().write().unwrap().insert(shader.key, raw);
}

fn psv_single_srv_hk(r: &dxil_metal::dxbc::PsvResource) -> u32 {
    use super::resource::*;
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

fn compute_unbounded_srv_hk(
    dxil: &[u8],
    rs: &RootSignature,
    shader: &CompiledShader,
    ru_opt: Option<u32>,
    detail_opt: Option<&DxilHkDetail>,
    ro_srv_kinds: u32,
) -> u32 {
    use dxil_metal::root_signature::{DescriptorRangeType, RootParameterLayoutKind};
    use super::resource::{HK_ALL, HK_CBV_BUF};
    const RESOURCE_HEAP_INDEXING: u64 = 1 << 25;
    const MAX_BOUNDED_ARRAY: u32 = 128;

    let Some(detail) = detail_opt else {
        return (HK_ALL & !HK_CBV_BUF) & ro_srv_kinds;
    };

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
    let air_unused = metallib_res_desc_heap_unused(&shader.metallib)
        && msc_used_resources(shader.reflection.json.as_str())
            .is_some_and(|u| !u.is_empty() || ru_opt == Some(0));

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
        if let Some(psv) = &psv_opt {
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

    if let Some(used) = msc_used_resources(shader.reflection.json.as_str()) {
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
            'u_loop: for u in &used {
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
                                msc_unb_hk |=
                                    range_srv_hk(rg.register_space, rg.base_shader_register, rg_reg_end);
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
    } else if let Some(psv) = &psv_opt {
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

fn build_handle_kind_profile_raw(
    dxil: &[u8],
    rs: &RootSignature,
    shader: &CompiledShader,
    ru_opt: Option<u32>,
) -> HandleKindProfile {
    use super::resource::{
        HK_ACCEL_STRUCT, HK_ALL, HK_ALL_TEX, HK_CBV_BUF, HK_RAW_BUF, HK_STRUCT_BUF, HK_TYPED_BUF,
    };
    const MSC_BUF_RW: u32 = HK_TYPED_BUF | HK_RAW_BUF | HK_STRUCT_BUF;
    let detail_opt = dxil_handle_kinds_detail_opt(dxil);
    let (dxil_srv, dxil_uav) = detail_opt
        .as_ref()
        .map_or((HK_ALL, HK_ALL & !HK_CBV_BUF), |d| (d.srv_mask, d.uav_mask));
    let has_cbv = msc_stage_may_read_cbv_from_reflection(
        &shader.metallib,
        shader.reflection.json.as_str(),
        rs,
    )
    .unwrap_or_else(|| stage_may_read_cbv_buffer_with(shader, dxil, rs, false));

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
    let cbv_bit = if has_cbv && (dxil_srv & HK_CBV_BUF) != 0 {
        HK_CBV_BUF
    } else {
        0
    };
    let read_kinds = uav_srv_kinds | cbv_bit;
    let unbounded_srv_kinds = compute_unbounded_srv_hk(
        dxil,
        rs,
        shader,
        ru_opt,
        detail_opt.as_ref(),
        ro_srv_kinds,
    );
    HandleKindProfile {
        uav_write_kinds,
        uav_srv_kinds,
        ro_srv_kinds,
        read_kinds,
        unbounded_srv_kinds,
    }
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
    use super::resource::*;
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
    use super::resource::HK_CBV_BUF;
    let kind = d0 & 0xff;
    let is_uav = (d0 & ((1 << 12) | (1 << 13))) != 0;
    let bits = decode_dxil_shape_and_comp(kind, Some(d1 & 0xff))?;
    if is_uav {
        Some((0, bits & !HK_CBV_BUF))
    } else {
        Some((bits, 0))
    }
}

fn psv_resource_handle_kinds(psv: &[dxil_metal::dxbc::PsvResource]) -> (u32, u32) {
    use super::resource::*;
    let kind_bits = |k: u32| -> u32 {
        decode_dxil_shape_and_comp(k, None).unwrap_or(HK_ALL)
    };
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
    use super::resource::{HK_ALL, HK_CBV_BUF};
    const FAIL: (u32, u32) = (HK_ALL, HK_ALL & !HK_CBV_BUF);
    dxil_annotate_handle_kinds_opt(dxil).unwrap_or(FAIL)
}

pub fn dxil_annotate_handle_kinds_opt(dxil: &[u8]) -> Option<(u32, u32)> {
    dxil_handle_kinds_detail_opt(dxil).map(|d| (d.srv_mask, d.uav_mask))
}

pub fn dxil_handle_kinds_detail_opt(dxil: &[u8]) -> Option<DxilHkDetail> {
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
                        vals.push(read_scalar(b, elt)?);
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
                        vals.push(b.read(8)?);
                    }
                    b.align32();
                }
                op => vals.push(read_scalar(b, op)?),
            }
            i += 1;
        }
        let code = *vals.first()?;
        vals.remove(0);
        Some((code, vals))
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

    let parsed = dxil_metal::dxbc::Container::parse(dxil).ok()?;
    let converted;
    let c = if parsed.is_sm4_bytecode() {
        converted = dxil_metal::dxilconv::convert_signed(dxil).ok()?;
        dxil_metal::dxbc::Container::parse(&converted).ok()?
    } else {
        parsed
    };
    let dxil_part = c.part(dxil_metal::dxbc::FOURCC_DXIL)?;
    let bitcode = dxil_metal::bitcode::dxil_part_bitcode(dxil_part)?;
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
                            let name = ops.get(start..)?.iter().map(|&ch| ch as u8 as char).collect();
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
                    let iv = ops
                        .get(1)
                        .and_then(|v| mod_const_i32.get(v).copied());
                    md_nodes.push(MdItem::Int(iv));
                }
                BcEntry::Rec(METADATA_NODE | METADATA_DISTINCT_NODE, ops) => {
                    md_nodes.push(MdItem::Node(ops));
                }
                BcEntry::Rec(METADATA_NAME, ops) => {
                    pending_md_name =
                        Some(ops.iter().map(|&ch| (ch as u8) as char).collect());
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
                    let hk = decode_dxil_shape_and_comp(shape, comp)?
                        & !super::resource::HK_CBV_BUF;
                    srv_mask |= hk;
                    srv_ranges.push(DxilSrvRangeHk {
                        space,
                        lower_bound,
                        upper_bound,
                        hk,
                    });
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
                    let hk = decode_dxil_shape_and_comp(shape, comp)?
                        & !super::resource::HK_CBV_BUF;
                    uav_mask |= hk;
                }
            }
            // rlist[2]: CBV list
            if rlist[2] != 0 && !md_node_1(rlist[2])?.is_empty() {
                srv_mask |= super::resource::HK_CBV_BUF;
            }
        }
    } else if has_legacy_create_handle || has_lib_create_handle {
        let psv = c.psv_resources()?;
        let (ps, pu) = psv_resource_handle_kinds(&psv);
        srv_mask |= ps;
        uav_mask |= pu;
    }

    Some(DxilHkDetail {
        srv_mask,
        uav_mask,
        srv_ranges,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignatureDesc,
        RootSignatureVersion,
    };

    /// Builds a minimal valid DXBC container with an `SFI0` feature-flags part (`flags`).
    fn fake_dxbc_with_sfi0(flags: u64) -> Vec<u8> {
        // Header (32 bytes): "DXBC" (4) + digest (16) + ver (4) + total_size (4) + part_count (4)
        // Part offset table (4 bytes): [36]
        // Part 0 (16 bytes): "SFI0" (4) + size=8 (4) + flags (8)
        let total_size: u32 = 32 + 4 + 16;
        let mut b = Vec::with_capacity(total_size as usize);
        b.extend_from_slice(b"DXBC");
        b.extend_from_slice(&[0u8; 16]);
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&total_size.to_le_bytes());
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&36u32.to_le_bytes());
        b.extend_from_slice(b"SFI0");
        b.extend_from_slice(&8u32.to_le_bytes());
        b.extend_from_slice(&flags.to_le_bytes());
        let sum = dxil_metal::hash::dxbc_checksum(&b[20..]);
        b[4..20].copy_from_slice(&sum);
        b
    }

    fn northlight_like_root_sig_with(with_static_sampler: bool) -> Arc<RootSignature> {
        // Mirrors AW2's compute GRS shape (`fillLightBufferDirectLightCS` / `transformCS`):
        // param 0: table [CBV b0..b9 (0..10), SRV t0..t50 (10..61), UAV u0..unbounded (61..)]
        // param 1: 32-bit inline root constants (b10)
        // param 2: root CBV (b11)
        // param 3: unbounded SRV table space1 (t0..unbounded)
        RootSignature::new(RootSignatureDesc {
            version: RootSignatureVersion::V1_1,
            parameters: vec![
                RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![
                        DescriptorRange {
                            range_type: DescriptorRangeType::Cbv,
                            num_descriptors: 10,
                            base_shader_register: 0,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 0,
                        },
                        DescriptorRange {
                            range_type: DescriptorRangeType::Srv,
                            num_descriptors: 51,
                            base_shader_register: 0,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 10,
                        },
                        DescriptorRange {
                            range_type: DescriptorRangeType::Uav,
                            num_descriptors: rs_flags::UNBOUNDED,
                            base_shader_register: 0,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 61,
                        },
                    ]),
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::Constants {
                        shader_register: 10,
                        register_space: 0,
                        num_32bit_values: 4,
                    },
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::Descriptor(
                        RootDescriptorKind::Cbv,
                        RootDescriptor {
                            shader_register: 11,
                            register_space: 0,
                            flags: 0,
                        },
                    ),
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                        range_type: DescriptorRangeType::Srv,
                        num_descriptors: rs_flags::UNBOUNDED,
                        base_shader_register: 0,
                        register_space: 1,
                        flags: 0,
                        offset_in_descriptors_from_table_start: 0,
                    }]),
                    visibility: 0,
                },
            ],
            static_samplers: if with_static_sampler {
                vec![dxil_metal::root_signature::StaticSampler::default()]
            } else {
                Vec::new()
            },
            flags: 0,
        })
        .unwrap()
    }

    #[test]
    fn msc_compute_access_resolves_bounded_tables_and_enforces_heap_and_cbv_guards() {
        if std::env::var_os("AW2_MSC_COMPUTE_ACCESS").is_none() {
            assert!(!msc_compute_access(), "AW2_MSC_COMPUTE_ACCESS must default OFF");
        }
        let rs = northlight_like_root_sig_with(false);
        let dxil_no_heap = fake_dxbc_with_sfi0(0);

        // 1) `fillLightBufferDirectLightCS`-shaped shader: unbounded UAV/SRV tables in RS,
        //    all `"UsedResources"` entries have `tableLength: 1` (`ResourceUsages = 26`).
        let json_fill = r#"{
            "Features": 0,
            "ResourceUsages": 26,
            "UsedResources": [
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 10},
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 62},
                {"bindingIndex": 1, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295},
                {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295},
                {"bindingIndex": 3, "isInGRS": true, "tableLength": 1, "tableStartIndex": 6}
            ]
        }"#;
        let parsed = msc_used_resources(json_fill).unwrap();
        assert_eq!(parsed.len(), 6);
        let acc_fill = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_no_heap,
            &rs,
            json_fill,
            Some(26),
        );
        assert!(acc_fill.msc_refined);
        assert_eq!(
            acc_fill.uav_slots,
            Some(vec![
                ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 },
                ComputeBind::TableEntry { param: 0, byte_offset: 62 * 24 },
            ])
        );
        assert_eq!(
            acc_fill.read_slots,
            Some(vec![
                ComputeBind::TableEntry { param: 0, byte_offset: 10 * 24 },
                ComputeBind::RootVa { param: 2 },
                ComputeBind::TableEntry { param: 3, byte_offset: 6 * 24 },
            ])
        );
        assert_eq!(acc_fill.uav_classes, READS_TEX);
        assert_eq!(acc_fill.read_classes, READS_TEX | READS_BUF);
        assert!(!acc_fill.heap_indexed_or_unknown);

        // 2) `transformCS`-shaped shader: bounded UAV (`61`) + unbounded buffer SRV (`param 3`), `RU = 6`.
        let json_skin = r#"{
            "Features": 0,
            "ResourceUsages": 6,
            "UsedResources": [
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 13},
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
                {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295},
                {"bindingIndex": 3, "isInGRS": true, "tableLength": 4294967295, "tableStartIndex": 0}
            ]
        }"#;
        let acc_skin = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_no_heap,
            &rs,
            json_skin,
            Some(6),
        );
        assert!(acc_skin.msc_refined);
        assert_eq!(
            acc_skin.uav_slots,
            Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 }])
        );
        assert_eq!(acc_skin.read_slots, None);
        assert_eq!(
            acc_skin.bounded_read_slots,
            vec![
                ComputeBind::TableEntry { param: 0, byte_offset: 13 * 24 },
                ComputeBind::RootVa { param: 2 },
            ]
        );
        assert_eq!(acc_skin.unbounded_srv_classes, READS_BUF);
        assert!(!acc_skin.touches_textures);
        assert_eq!(acc_skin.uav_classes, READS_BUF);
        assert_eq!(acc_skin.read_classes, READS_BUF);

        // 3) SM 6.6 `RESOURCE_HEAP_INDEXING` (`1 << 25`): MUST reject Fix 1 (`uav_slots == None`
        //    when UAVs exist) while still refining `uav_classes` via Fix 2 (`RU = 16` -> `READS_TEX`).
        let dxil_heap = fake_dxbc_with_sfi0(1u64 << 25);
        let acc_heap = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_heap,
            &rs,
            json_fill,
            Some(16),
        );
        assert!(!acc_heap.msc_refined);
        assert_eq!(acc_heap.uav_slots, None);
        assert_eq!(acc_heap.read_slots, None);
        assert_eq!(acc_heap.uav_classes, READS_TEX);
        assert_eq!(acc_heap.read_classes, READS_TEX | READS_BUF);

        // 4) `isInGRS == false`: rejects Fix 1 (`read_slots == None`), while `RU = 8` (`TextureRead` only,
        //    zero UAVs) proves `uav_slots == Some(vec![])` and `uav_classes == 0`.
        let json_non_grs = r#"{
            "ResourceUsages": 8,
            "UsedResources": [
                {"bindingIndex": 0, "isInGRS": false, "tableLength": 1, "tableStartIndex": 10}
            ]
        }"#;
        let acc_non_grs = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_no_heap,
            &rs,
            json_non_grs,
            Some(8),
        );
        assert_eq!(acc_non_grs.read_slots, None);
        assert_eq!(acc_non_grs.uav_slots, Some(Vec::new()));
        assert_eq!(acc_non_grs.uav_classes, 0);

        // 5) CBV-only `ResourceUsages == 0` soundness: root CBV (`param 2`) keeps `READS_BUF` in
        //    `read_classes`, whereas inline 32-bit root constants (`param 1`) alone have `read_classes == 0`.
        let json_root_cbv = r#"{
            "ResourceUsages": 0,
            "UsedResources": [
                {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295}
            ]
        }"#;
        let acc_cbv = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_no_heap,
            &rs,
            json_root_cbv,
            Some(0),
        );
        assert_eq!(acc_cbv.uav_classes, 0);
        assert_eq!(acc_cbv.read_classes, READS_BUF, "root CBV resource must retain READS_BUF");

        let json_inline_const = r#"{
            "ResourceUsages": 0,
            "UsedResources": [
                {"bindingIndex": 1, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295}
            ]
        }"#;
        let acc_const = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_no_heap,
            &rs,
            json_inline_const,
            Some(0),
        );
        assert_eq!(acc_const.uav_classes, 0);
        assert_eq!(acc_const.read_classes, 0);

        // 6) Static-sampler TLAB slot (`bindingIndex == parameters.len() == 4`):
        //    rejected when `static_samplers` is empty, accepted as stateless sampler table when non-empty.
        let json_static_smp = r#"{
            "ResourceUsages": 12,
            "UsedResources": [
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
                {"bindingIndex": 4, "isInGRS": true, "tableLength": 1, "tableStartIndex": 0}
            ]
        }"#;
        let acc_no_smp = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_no_heap,
            &rs,
            json_static_smp,
            Some(12),
        );
        assert!(!acc_no_smp.msc_refined);
        assert_eq!(acc_no_smp.uav_slots, None);

        let rs_with_smp = northlight_like_root_sig_with(true);
        let acc_with_smp = ComputeAccess::refine_with_msc(
            ComputeAccess::opaque(),
            &dxil_no_heap,
            &rs_with_smp,
            json_static_smp,
            Some(12),
        );
        assert!(acc_with_smp.msc_refined);
        assert_eq!(
            acc_with_smp.uav_slots,
            Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 }])
        );
        assert_eq!(acc_with_smp.read_slots, Some(Vec::new()));
    }

    #[test]
    fn g40_air_no_res_heap_and_stage_usage_refinement() {
        if std::env::var_os("AW2_MSC_STAGE_USAGE").is_none() {
            assert!(!msc_stage_usage(), "AW2_MSC_STAGE_USAGE must default OFF");
        }
        if std::env::var_os("AW2_AIR_NO_RES_HEAP").is_none() {
            assert!(!air_no_res_heap(), "AW2_AIR_NO_RES_HEAP must default OFF");
        }

        let build_mock_metallib = |magic: &[u8; 4], include_rbuf: bool, unused_byte: u8, loc_idx: u32, dup_name: bool| -> Vec<u8> {
            let mut m = Vec::new();
            m.extend_from_slice(magic);
            m.extend_from_slice(&[0u8; 32]);
            if include_rbuf {
                m.extend_from_slice(b"RBUF");
                m.extend_from_slice(&[0u8; 16]);
            }
            m.extend_from_slice(&[0x01, 0x02, unused_byte]);
            m.extend_from_slice(&loc_idx.to_le_bytes());
            m.extend_from_slice(&1u32.to_le_bytes());
            m.extend_from_slice(&0x1234_5678u32.to_le_bytes()); // vtable soffset_t
            m.extend_from_slice(&8u32.to_le_bytes());
            m.extend_from_slice(&4u32.to_le_bytes());
            m.extend_from_slice(&23u32.to_le_bytes());
            m.extend_from_slice(b"struct.res_desc_heap_ab\0");
            if dup_name {
                m.extend_from_slice(b"struct.res_desc_heap_ab\0");
            }
            m
        };

        assert!(metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 1, 0, false)));
        assert!(!metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 0, 0, false)), "unused==0 (active ResourceDescriptorHeap) MUST fail closed");
        assert!(!metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 1, 2, false)), "unexpected location_index MUST fail closed");
        assert!(!metallib_res_desc_heap_unused(&build_mock_metallib(b"XXXX", true, 1, 0, false)), "non-MTLB magic MUST fail closed");
        assert!(!metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", false, 1, 0, false)), "missing RBUF section MUST fail closed");
        assert!(!metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 1, 0, true)), "duplicate res_desc_heap_ab symbol MUST fail closed");

        let rs = northlight_like_root_sig_with(false);
        let dxil_heap = fake_dxbc_with_sfi0(1u64 << 25);
        let json_tex_cs = r#"{
            "Features": 0,
            "ResourceUsages": 24,
            "UsedResources": [
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 10},
                {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
                {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295}
            ]
        }"#;

        // Stage B (`enable_used_resources = false`): class masks narrow (`uav_srv_classes == READS_TEX`)
        // while `read_classes` preserves `READS_BUF` for root CBV and `uav_slots` stays `None` on SM 6.6.
        let acc_stage_b = ComputeAccess::refine_with_msc_full(
            ComputeAccess::opaque(),
            &dxil_heap,
            &rs,
            json_tex_cs,
            Some(24),
            false,
            false,
        );
        assert_eq!(acc_stage_b.uav_slots, None);
        assert_eq!(acc_stage_b.uav_classes, READS_TEX);
        assert_eq!(acc_stage_b.uav_srv_classes, READS_TEX, "uav_srv_classes must exclude CBV-only READS_BUF");
        assert_eq!(acc_stage_b.read_classes, READS_TEX | READS_BUF, "read_classes must retain READS_BUF for CBV");

        // Stage D (`enable_used_resources = true, ignore_heap_flag = true`): SM 6.6 `SFI0`
        // `RESOURCE_HEAP_INDEXING` is bypassed when AIR proves `res_desc_heap_ab` is unused!
        let acc_stage_d = ComputeAccess::refine_with_msc_full(
            ComputeAccess::opaque(),
            &dxil_heap,
            &rs,
            json_tex_cs,
            Some(24),
            true,
            true,
        );
        assert!(acc_stage_d.msc_refined);
        assert_eq!(
            acc_stage_d.uav_slots,
            Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 }])
        );
        assert_eq!(
            acc_stage_d.read_slots,
            Some(vec![
                ComputeBind::TableEntry { param: 0, byte_offset: 10 * 24 },
                ComputeBind::RootVa { param: 2 },
            ])
        );
        assert_eq!(acc_stage_d.uav_classes, READS_TEX);
        assert_eq!(acc_stage_d.uav_srv_classes, READS_TEX);
        assert_eq!(acc_stage_d.read_classes, READS_TEX | READS_BUF);
        assert_eq!(acc_stage_d.msc_exact_classes, READS_TEX | READS_BUF);
    }

    #[test]
    fn g42_msc_gfx_cbv_and_cs_msc_exact_classes_distinguish_inline_constants_from_buffer_cbvs() {
        use dxil_metal::root_signature::{
            flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
            RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
            RootSignatureDesc, RootSignatureVersion,
        };

        // Root signature with inline constants at b0 (param 0 -> bindingIndex 0),
        // root CBV at b1 (param 1 -> bindingIndex 1), and SRV/UAV table at param 2 (bindingIndex 2).
        let rs = RootSignature::new(RootSignatureDesc {
            version: RootSignatureVersion::V1_1,
            parameters: vec![
                RootParameter {
                    kind: RootParameterKind::Constants {
                        shader_register: 0,
                        register_space: 0,
                        num_32bit_values: 4,
                    },
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::Descriptor(
                        RootDescriptorKind::Cbv,
                        RootDescriptor {
                            shader_register: 1,
                            register_space: 0,
                            flags: 0,
                        },
                    ),
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![
                        DescriptorRange {
                            range_type: DescriptorRangeType::Srv,
                            num_descriptors: rs_flags::UNBOUNDED,
                            base_shader_register: 0,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 0,
                        },
                        DescriptorRange {
                            range_type: DescriptorRangeType::Cbv,
                            num_descriptors: 2,
                            base_shader_register: 2,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 16,
                        },
                    ]),
                    visibility: 0,
                },
            ],
            static_samplers: Vec::new(),
            flags: rs_flags::CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED,
        })
        .unwrap();

        let make_mtlb = |unused: u8| -> Vec<u8> {
            let mut m = Vec::new();
            m.extend_from_slice(b"MTLB");
            m.extend_from_slice(&[0u8; 32]);
            m.extend_from_slice(b"RBUF");
            m.extend_from_slice(&[0u8; 16]);
            m.extend_from_slice(&[0x01, 0x02, unused]);
            m.extend_from_slice(&0u32.to_le_bytes());
            m.extend_from_slice(&1u32.to_le_bytes());
            m.extend_from_slice(&0x1234_5678u32.to_le_bytes());
            m.extend_from_slice(&8u32.to_le_bytes());
            m.extend_from_slice(&4u32.to_le_bytes());
            m.extend_from_slice(&23u32.to_le_bytes());
            m.extend_from_slice(b"struct.res_desc_heap_ab\0");
            m
        };
        let mtlb_unused = make_mtlb(1);
        let mtlb_active = make_mtlb(0);

        // 1) SM 6.6 shader touching ONLY inline constants at `bindingIndex: 0` + texture SRV (`tableStartIndex: 5`):
        //    `msc_stage_may_read_cbv_from_reflection` MUST return `Some(false)` when `res_desc_heap_ab` is unused
        //    (and `None` when active!), and `msc_exact_classes` MUST be `READS_TEX`!
        let json_inline_const_tex = r#"{"ResourceUsages":8,"UsedResources":[{"bindingIndex":0,"isInGRS":true,"tableLength":1,"tableStartIndex":0},{"bindingIndex":2,"isInGRS":true,"tableLength":1,"tableStartIndex":5}]}"#;
        assert_eq!(
            super::msc_stage_may_read_cbv_from_reflection(&mtlb_unused, json_inline_const_tex, &rs),
            Some(false),
            "inline root constants + texture SRV table must not report CBV buffer read"
        );
        assert_eq!(
            super::msc_stage_may_read_cbv_from_reflection(&mtlb_active, json_inline_const_tex, &rs),
            None,
            "active ResourceDescriptorHeap must fail closed (None)"
        );
        let dxil_heap = fake_dxbc_with_sfi0(1u64 << 25);
        assert_eq!(
            ComputeAccess::exact_classes_from_metallib(&mtlb_unused, json_inline_const_tex, 8, true, &rs),
            READS_TEX,
            "unused res_desc_heap_ab + inline constants must narrow msc_exact_classes to READS_TEX"
        );
        assert_eq!(
            ComputeAccess::exact_classes_from_metallib(&mtlb_active, json_inline_const_tex, 8, true, &rs),
            READS_TEX | READS_BUF,
            "active res_desc_heap_ab must fail closed and retain READS_BUF"
        );
        let mut init_tex = ComputeAccess::opaque();
        init_tex.msc_exact_classes =
            ComputeAccess::exact_classes_from_metallib(&mtlb_unused, json_inline_const_tex, 8, true, &rs);
        let acc_tex_only = ComputeAccess::refine_with_msc_full(
            init_tex,
            &dxil_heap,
            &rs,
            json_inline_const_tex,
            Some(8),
            false,
            false,
        );
        // Even with `enable_used_resources = false` (zero salt!), `msc_exact_classes` stays `READS_TEX` (`!READS_BUF`).
        assert_eq!(acc_tex_only.read_classes, READS_TEX | READS_BUF);
        assert_eq!(acc_tex_only.msc_exact_classes, READS_TEX);

        // 2) Shader touching root CBV at `bindingIndex: 1` -> `Some(true)` & `msc_exact_classes` includes `READS_BUF`.
        let json_root_cbv = r#"{"ResourceUsages":0,"UsedResources":[{"bindingIndex":1,"isInGRS":true,"tableLength":1,"tableStartIndex":4294967295}]}"#;
        assert_eq!(super::msc_stage_may_read_cbv_from_reflection(&mtlb_unused, json_root_cbv, &rs), Some(true));
        let mut init_cbv = ComputeAccess::opaque();
        init_cbv.msc_exact_classes =
            ComputeAccess::exact_classes_from_metallib(&mtlb_unused, json_root_cbv, 0, true, &rs);
        let acc_root_cbv = ComputeAccess::refine_with_msc_full(
            init_cbv,
            &dxil_heap,
            &rs,
            json_root_cbv,
            Some(0),
            false,
            false,
        );
        assert_eq!(acc_root_cbv.msc_exact_classes, READS_BUF);

        // 3) Shader touching CBV descriptor table entry (`bindingIndex: 2`, `tableStartIndex: 17` inside `[16, 18)`) -> `Some(true)`.
        let json_tbl_cbv = r#"{"ResourceUsages":0,"UsedResources":[{"bindingIndex":2,"isInGRS":true,"tableLength":1,"tableStartIndex":17}]}"#;
        assert_eq!(super::msc_stage_may_read_cbv_from_reflection(&mtlb_unused, json_tbl_cbv, &rs), Some(true));
    }

    fn build_synth_dxil_with_annotate(props: &[(u32, u32)], fn_names: &[&str]) -> Vec<u8> {
        #[derive(Default)]
        struct Tw {
            out: Vec<u8>,
            bit: usize,
        }
        impl Tw {
            fn put(&mut self, v: u64, n: u32) {
                for i in 0..n {
                    if self.bit.is_multiple_of(8) {
                        self.out.push(0);
                    }
                    if (v >> i) & 1 == 1 {
                        *self.out.last_mut().unwrap() |= 1 << (self.bit % 8);
                    }
                    self.bit += 1;
                }
            }
            fn vbr(&mut self, mut v: u64, n: u32) {
                let hi = 1u64 << (n - 1);
                loop {
                    if v < hi {
                        self.put(v, n);
                        return;
                    }
                    self.put((v & (hi - 1)) | hi, n);
                    v >>= n - 1;
                }
            }
            fn align(&mut self) {
                while !self.bit.is_multiple_of(32) {
                    self.put(0, 1);
                }
            }
            fn enter(&mut self, w: u32, id: u64, new_w: u32) -> usize {
                self.put(1, w);
                self.vbr(id, 8);
                self.vbr(new_w as u64, 4);
                self.align();
                let at = self.out.len();
                self.put(0, 32);
                at
            }
            fn end(&mut self, w: u32, len_at: usize) {
                self.put(0, w);
                self.align();
                let words = ((self.out.len() - len_at - 4) / 4) as u32;
                self.out[len_at..len_at + 4].copy_from_slice(&words.to_le_bytes());
            }
            fn rec(&mut self, w: u32, code: u64, ops: &[u64]) {
                self.put(3, w);
                self.vbr(code, 6);
                self.vbr(ops.len() as u64, 6);
                for &o in ops {
                    self.vbr(o, 6);
                }
            }
        }

        let mut w = Tw::default();
        w.out.extend_from_slice(b"BC\xC0\xDE");
        w.bit = 32;
        let m_at = w.enter(2, 8, 4);

        // TYPE_BLOCK_NEW (17):
        // tid 0: void (2)
        // tid 1: i32 (7, [32])
        // tid 2: %dx.types.Handle (20, [0, ...])
        // tid 3: %dx.types.ResourceProperties (20, [0, 1, 1])
        // tid 4: void() (21, [0, 0])
        // tid 5: Handle(i32, Handle, ResourceProperties) (21, [0, 2, 1, 2, 3])
        let t_at = w.enter(4, 17, 4);
        w.rec(4, 1, &[6]);
        w.rec(4, 2, &[]);
        w.rec(4, 7, &[32]);
        let h_nm: Vec<u64> = b"dx.types.Handle".iter().map(|&c| c as u64).collect();
        w.rec(4, 19, &h_nm);
        w.rec(4, 20, &[0, 1]);
        let rp_nm: Vec<u64> = b"dx.types.ResourceProperties".iter().map(|&c| c as u64).collect();
        w.rec(4, 19, &rp_nm);
        w.rec(4, 20, &[0, 1, 1]);
        w.rec(4, 21, &[0, 0]);
        w.rec(4, 21, &[0, 2, 1, 2, 3]);
        w.end(4, t_at);

        // vid 0: entry `main` (isproto = 0, ty = 4)
        w.rec(4, 8, &[4, 0, 0, 0]);
        // vid 1..: declared handle-returning functions (isproto = 1, ty = 5)
        for _ in fn_names {
            w.rec(4, 8, &[5, 0, 1, 0]);
        }

        // VALUE_SYMTAB_BLOCK (14)
        let v_at = w.enter(4, 14, 4);
        let mut main_ops = vec![0u64];
        main_ops.extend(b"main".iter().map(|&c| c as u64));
        w.rec(4, 1, &main_ops);
        for (i, nm) in fn_names.iter().enumerate() {
            let mut ops = vec![(i + 1) as u64];
            ops.extend(nm.bytes().map(|c| c as u64));
            w.rec(4, 1, &ops);
        }
        w.end(4, v_at);

        // FUNCTION_BLOCK (12) for `main` with function-local CONSTANTS_BLOCK (11)
        let f_at = w.enter(4, 12, 4);
        w.rec(4, 1, &[1]); // DECLAREBLOCKS
        let c_at = w.enter(4, 11, 4);
        let base_vid = 1 + fn_names.len() as u64;
        w.rec(4, 1, &[1]); // SETTYPE i32
        for &(d0, d1) in props {
            w.rec(4, 4, &[(d0 as u64) << 1]);
            w.rec(4, 4, &[(d1 as u64) << 1]);
        }
        w.rec(4, 1, &[3]); // SETTYPE %dx.types.ResourceProperties
        for (k, _) in props.iter().enumerate() {
            let v0 = base_vid + (k as u64) * 2;
            let v1 = v0 + 1;
            w.rec(4, 7, &[v0, v1]); // CST_CODE_AGGREGATE
        }
        w.end(4, c_at);
        w.rec(4, 10, &[]); // RET void
        w.end(4, f_at);

        w.end(2, m_at);

        // Wrap bitcode in a DXIL part + DXBC container
        let bc = w.out;
        let mut dxil_part = Vec::with_capacity(24 + bc.len());
        dxil_part.extend_from_slice(&0x0005_0066u32.to_le_bytes()); // version (CS 6.6)
        dxil_part.extend_from_slice(&(((24 + bc.len()) / 4) as u32).to_le_bytes());
        dxil_part.extend_from_slice(b"DXIL");
        dxil_part.extend_from_slice(&0x0000_0106u32.to_le_bytes());
        dxil_part.extend_from_slice(&16u32.to_le_bytes()); // off from +8 -> 24
        dxil_part.extend_from_slice(&(bc.len() as u32).to_le_bytes());
        dxil_part.extend_from_slice(&bc);

        let total_size = (32 + 4 + 8 + dxil_part.len()) as u32;
        let mut dxbc = Vec::with_capacity(total_size as usize);
        dxbc.extend_from_slice(b"DXBC");
        dxbc.extend_from_slice(&[0u8; 16]);
        dxbc.extend_from_slice(&1u32.to_le_bytes());
        dxbc.extend_from_slice(&total_size.to_le_bytes());
        dxbc.extend_from_slice(&1u32.to_le_bytes());
        dxbc.extend_from_slice(&36u32.to_le_bytes());
        dxbc.extend_from_slice(b"DXIL");
        dxbc.extend_from_slice(&(dxil_part.len() as u32).to_le_bytes());
        dxbc.extend_from_slice(&dxil_part);
        let sum = dxil_metal::hash::dxbc_checksum(&dxbc[20..]);
        dxbc[4..20].copy_from_slice(&sum);
        dxbc
    }

    #[test]
    fn g50_dxil_annotate_handle_kinds_flt_vs_int_and_fail_closed() {
        use super::super::resource::*;

        // 1) SM 6.6 shader with Texture2D<float4> SRV (kind=2, comp=F32=9),
        //    StructuredBuffer SRV (kind=12), and RWTexture3D<float4> UAV (kind=4 | (1<<12), comp=F32=9).
        let dxil_flt = build_synth_dxil_with_annotate(
            &[(2, 9), (12, 16), (4 | (1 << 12), 9)],
            &["dx.op.createHandleFromHeap", "dx.op.annotateHandle"],
        );
        let (srv_f, uav_f) = dxil_annotate_handle_kinds(&dxil_flt);
        assert_eq!(srv_f, HK_TEX2D_FLT | HK_STRUCT_BUF);
        assert_eq!(uav_f, HK_TEX3D_FLT);
        assert_eq!(srv_f & HK_TEX2D_INT, 0, "float Texture2D must not match uint Texture2D");
        assert_eq!(srv_f & HK_RAW_BUF, 0, "StructuredBuffer must not match RawBuffer");

        // 2) SM 6.6 shader with Texture2D<uint4> SRV (kind=2, comp=U32=5) and RWByteAddressBuffer UAV (kind=11 | (1<<12)).
        let dxil_int = build_synth_dxil_with_annotate(
            &[(2, 5), (11 | (1 << 12), 0)],
            &["dx.op.createHandleFromHeap", "dx.op.annotateHandle"],
        );
        let (srv_i, uav_i) = dxil_annotate_handle_kinds(&dxil_int);
        assert_eq!(srv_i, HK_TEX2D_INT);
        assert_eq!(uav_i, HK_RAW_BUF);
        assert_eq!(srv_i & HK_TEX2D_FLT, 0, "uint Texture2D must not match float Texture2D");

        // 3) Unknown handle-producing function fails closed to (HK_ALL, HK_ALL & !HK_CBV_BUF).
        let dxil_unknown = build_synth_dxil_with_annotate(
            &[(2, 9)],
            &["dx.op.annotateHandle", "dx.op.unknownMakeHandle"],
        );
        assert_eq!(
            dxil_annotate_handle_kinds(&dxil_unknown),
            (HK_ALL, HK_ALL & !HK_CBV_BUF),
            "unknown handle-returning intrinsic must fail closed"
        );
    }

    #[test]
    fn g51_dxil_cst_data_undef_and_cache_hit_parity() {
        use super::super::resource::*;

        let mut raw = dxil_metal::dxbc::PsvResource {
            res_type: 4, space: 0, lower_bound: 0, upper_bound: u32::MAX, kind: 0,
        };
        assert_eq!(psv_resource_handle_kinds(&[raw]), (HK_RAW_BUF | HK_ACCEL_STRUCT, 0));
        raw.kind = 16;
        assert_eq!(psv_resource_handle_kinds(&[raw]), (HK_ACCEL_STRUCT, 0));

        // 1) `createHandleFromHeap` WITHOUT `annotateHandle` MUST fail closed (`None`).
        let dxil_unannotated =
            build_synth_dxil_with_annotate(&[], &["dx.op.createHandleFromHeap"]);
        assert_eq!(
            dxil_annotate_handle_kinds_opt(&dxil_unannotated),
            None,
            "SM 6.6 createHandleFromHeap without annotateHandle must fail closed"
        );

        assert_eq!(
            dxil_annotate_handle_kinds_opt(&build_synth_dxil_with_annotate(
                &[],
                &["dx.op.createHandleFromHeap", "dx.op.annotateHandle"],
            )),
            None,
            "an annotateHandle declaration alone cannot prove the set of resource kinds",
        );

        // 2) `CST_CODE_DATA = 22` + `VST_CODE_FNENTRY = 3` + multiple module `CONSTANTS_BLOCK`s
        //    must parse cleanly, whereas `CST_CODE_UNDEF = 3` on `%dx.types.ResourceProperties`
        //    must fail closed (`None`).
        let build_variant = |use_undef: bool| -> Vec<u8> {
            #[derive(Default)]
            struct Tw {
                out: Vec<u8>,
                bit: usize,
            }
            impl Tw {
                fn put(&mut self, v: u64, n: u32) {
                    for i in 0..n {
                        if self.bit.is_multiple_of(8) {
                            self.out.push(0);
                        }
                        if (v >> i) & 1 == 1 {
                            *self.out.last_mut().unwrap() |= 1 << (self.bit % 8);
                        }
                        self.bit += 1;
                    }
                }
                fn vbr(&mut self, mut v: u64, n: u32) {
                    let hi = 1u64 << (n - 1);
                    loop {
                        if v < hi {
                            self.put(v, n);
                            return;
                        }
                        self.put((v & (hi - 1)) | hi, n);
                        v >>= n - 1;
                    }
                }
                fn align(&mut self) {
                    while !self.bit.is_multiple_of(32) {
                        self.put(0, 1);
                    }
                }
                fn enter(&mut self, w: u32, id: u64, new_w: u32) -> usize {
                    self.put(1, w);
                    self.vbr(id, 8);
                    self.vbr(new_w as u64, 4);
                    self.align();
                    let at = self.out.len();
                    self.put(0, 32);
                    at
                }
                fn end(&mut self, w: u32, len_at: usize) {
                    self.put(0, w);
                    self.align();
                    let words = ((self.out.len() - len_at - 4) / 4) as u32;
                    self.out[len_at..len_at + 4].copy_from_slice(&words.to_le_bytes());
                }
                fn rec(&mut self, w: u32, code: u64, ops: &[u64]) {
                    self.put(3, w);
                    self.vbr(code, 6);
                    self.vbr(ops.len() as u64, 6);
                    for &o in ops {
                        self.vbr(o, 6);
                    }
                }
            }

            let mut w = Tw::default();
            w.out.extend_from_slice(b"BC\xC0\xDE");
            w.bit = 32;
            let m_at = w.enter(2, 8, 4);

            let t_at = w.enter(4, 17, 4);
            w.rec(4, 1, &[6]);
            w.rec(4, 2, &[]);
            w.rec(4, 7, &[32]);
            let h_nm: Vec<u64> = b"dx.types.Handle".iter().map(|&c| c as u64).collect();
            w.rec(4, 19, &h_nm);
            w.rec(4, 20, &[0, 1]);
            let rp_nm: Vec<u64> =
                b"dx.types.ResourceProperties".iter().map(|&c| c as u64).collect();
            w.rec(4, 19, &rp_nm);
            w.rec(4, 20, &[0, 1, 1]);
            w.rec(4, 21, &[0, 0]);
            w.rec(4, 21, &[0, 2, 1, 2, 3]);
            w.end(4, t_at);

            w.rec(4, 8, &[4, 0, 0, 0]); // vid 0: main
            w.rec(4, 8, &[5, 0, 1, 0]); // vid 1: createHandleFromHeap
            w.rec(4, 8, &[5, 0, 1, 0]); // vid 2: annotateHandle

            let v_at = w.enter(4, 14, 4);
            w.rec(4, 2, &[1, b'b' as u64, b'b' as u64]); // BB ID 1 is independent of value ID 1.
            for (vid, nm) in [
                (0u64, "main"),
                (1, "dx.op.createHandleFromHeap"),
                (2, "dx.op.annotateHandle"),
            ] {
                let code = if vid == 0 { 3 } else { 1 };
                let mut ops = if vid == 0 { vec![vid, 0] } else { vec![vid] };
                ops.extend(nm.bytes().map(|c| c as u64));
                w.rec(4, code, &ops);
            }
            w.end(4, v_at);

            // Two module CONSTANTS_BLOCKs: first holds i32, second holds CST_CODE_DATA=22 (or UNDEF=3).
            let mc1 = w.enter(4, 11, 4);
            w.rec(4, 1, &[1]);
            w.rec(4, 4, &[42 << 1]);
            w.end(4, mc1);

            let mc2 = w.enter(4, 11, 4);
            w.rec(4, 1, &[3]); // SETTYPE %dx.types.ResourceProperties
            if use_undef {
                w.rec(4, 3, &[]); // CST_CODE_UNDEF = 3 -> MUST fail closed
            } else {
                // CST_CODE_DATA = 22: Texture3D<float4> SRV (kind=4, comp=9)
                w.rec(4, 22, &[4, 9]);
            }
            w.end(4, mc2);

            let f_at = w.enter(4, 12, 4);
            w.rec(4, 1, &[1]);
            let fc = w.enter(4, 11, 4);
            w.rec(4, 1, &[3]);
            // Function-local CST_CODE_DATA = 22: RWStructuredBuffer UAV (kind=12 | (1<<12))
            w.rec(4, 22, &[12 | (1 << 12), 0]);
            w.end(4, fc);
            w.rec(4, 10, &[]);
            w.end(4, f_at);
            w.end(2, m_at);

            let bc = w.out;
            let mut dxil_part = Vec::with_capacity(24 + bc.len());
            dxil_part.extend_from_slice(&0x0005_0066u32.to_le_bytes());
            dxil_part.extend_from_slice(&(((24 + bc.len()) / 4) as u32).to_le_bytes());
            dxil_part.extend_from_slice(b"DXIL");
            dxil_part.extend_from_slice(&0x0000_0106u32.to_le_bytes());
            dxil_part.extend_from_slice(&16u32.to_le_bytes());
            dxil_part.extend_from_slice(&(bc.len() as u32).to_le_bytes());
            dxil_part.extend_from_slice(&bc);
            let total_size = (32 + 4 + 8 + dxil_part.len()) as u32;
            let mut dxbc = Vec::with_capacity(total_size as usize);
            dxbc.extend_from_slice(b"DXBC");
            dxbc.extend_from_slice(&[0u8; 16]);
            dxbc.extend_from_slice(&1u32.to_le_bytes());
            dxbc.extend_from_slice(&total_size.to_le_bytes());
            dxbc.extend_from_slice(&1u32.to_le_bytes());
            dxbc.extend_from_slice(&36u32.to_le_bytes());
            dxbc.extend_from_slice(b"DXIL");
            dxbc.extend_from_slice(&(dxil_part.len() as u32).to_le_bytes());
            dxbc.extend_from_slice(&dxil_part);
            let sum = dxil_metal::hash::dxbc_checksum(&dxbc[20..]);
            dxbc[4..20].copy_from_slice(&sum);
            dxbc
        };

        assert_eq!(
            dxil_annotate_handle_kinds_opt(&build_variant(false)),
            Some((HK_TEX3D_FLT, HK_STRUCT_BUF))
        );
        assert_eq!(
            dxil_annotate_handle_kinds_opt(&build_variant(true)),
            None,
            "CST_CODE_UNDEF on ResourceProperties must fail closed"
        );

        // 3) `.aw2s` cold-compile MISS vs warm-cache HIT `HandleKindProfile` parity on a real
        //    compiled shader fixture.
        if super::super::device().is_some() && dxil_metal::converter_available() {
            let raw_cs = include_bytes!("../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil");
            let mut cs_dxil = raw_cs.to_vec();
            let sum = dxil_metal::hash::dxbc_checksum(&cs_dxil[20..]);
            cs_dxil[4..20].copy_from_slice(&sum);
            let cs_rs = RootSignature::new(RootSignatureDesc::covering_dxil(&cs_dxil).unwrap()).unwrap();
            let tmp_dir = std::env::temp_dir().join(format!(
                "aw2_g51_hk_cache_parity_{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&tmp_dir);
            let pipe = dxil_metal::ShaderPipeline::new(dxil_metal::CacheConfig {
                disk_dir: Some(tmp_dir.clone()),
                memory_limit_bytes: 16 * 1024 * 1024,
            })
            .unwrap();
            let sh_miss = pipe
                .compile_stage(
                    dxil_metal::ShaderStage::Compute,
                    &cs_dxil,
                    Some(&cs_rs),
                    &dxil_metal::CompileOptions::default(),
                )
                .unwrap();
            let pipe_warm = dxil_metal::ShaderPipeline::new(dxil_metal::CacheConfig {
                disk_dir: Some(tmp_dir.clone()),
                memory_limit_bytes: 16 * 1024 * 1024,
            })
            .unwrap();
            let sh_hit = pipe_warm
                .compile_stage(
                    dxil_metal::ShaderStage::Compute,
                    &cs_dxil,
                    Some(&cs_rs),
                    &dxil_metal::CompileOptions::default(),
                )
                .unwrap();
            assert_eq!(pipe_warm.cache().stats().disk_hits, 1, "second pipeline must hit .aw2s disk cache");
            let acc_miss = ComputeAccess::from_dxil_and_shader(&cs_dxil, &cs_rs, &sh_miss);
            let prof_miss = build_handle_kind_profile(
                &cs_dxil,
                &cs_rs,
                &sh_miss,
                msc_resource_usages(&sh_miss),
                &acc_miss,
            );
            let acc_hit = ComputeAccess::from_dxil_and_shader(&cs_dxil, &cs_rs, &sh_hit);
            let prof_hit = build_handle_kind_profile(
                &cs_dxil,
                &cs_rs,
                &sh_hit,
                msc_resource_usages(&sh_hit),
                &acc_hit,
            );
            assert_eq!(prof_miss, prof_hit, ".aw2s disk cache HIT and MISS must produce identical HandleKindProfile");
            assert_eq!(
                prof_hit.ro_srv_kinds, HK_TEX2D_FLT,
                "static_samplers.cs.dxil Texture2D<float4> SRV must narrow ro_srv_kinds to HK_TEX2D_FLT (not HK_TEX2D_INT)"
            );
            let _ = std::fs::remove_dir_all(&tmp_dir);
        }
    }

    #[test]
    fn g51_real_northlight_binrfx_and_pe_dxil_handle_kinds() {
        use super::super::resource::*;
        use std::path::PathBuf;

        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let candidates = [
            std::env::var_os("AW2_GAME_DIR").map(PathBuf::from),
            Some(manifest.join("../../Alan Wake 2.app/Contents/Resources/game")),
            Some(manifest.join("../../../../Alan Wake 2.app/Contents/Resources/game")),
            Some(manifest.join("../../Alan Wake 2.app/Contents/Resources/Alan Wake 2")),
            Some(manifest.join("../../../../Alan Wake 2.app/Contents/Resources/Alan Wake 2")),
        ];
        let Some(game_dir) = candidates
            .into_iter()
            .flatten()
            .find(|p| p.join("data/shaders/build/pc_dx12").is_dir())
        else {
            assert!(std::env::var_os("AW2_GAME_DIR").is_none(), "AW2_GAME_DIR has no Northlight shader directory");
            eprintln!("skipping real DXIL corpus: no local game installation");
            return;
        };

        let rfx_dir = game_dir.join("data/shaders/build/pc_dx12");
        let mut rfx_total = 0;
        let mut rfx_proven = 0;
        let mut pe_total = 0;
        let mut pe_proven = 0;
        let mut sm66_proven = 0;
        let mut skin = false;
        let mut depth = false;
        let mut framegen = false;
        for path in dxil_metal::rfx::list_rfx_bundles(&rfx_dir, None).unwrap() {
            let bundle = dxil_metal::rfx::RfxBundle::open(path).unwrap();
            for prog in bundle.programs {
                rfx_total += 1;
                let detail = dxil_handle_kinds_detail_opt(&prog.dxbc_bytes);
                rfx_proven += usize::from(detail.as_ref().is_some_and(|d| d.srv_mask != HK_ALL));
                // Expected masks from independent LLVM disassembly of the shipped shader.
                match (bundle.file_name.as_str(), prog.entry_point.as_str()) {
                    ("skinning.binrfx", "transformCS") if !skin => {
                        let d = detail.unwrap();
                        assert_eq!(d.srv_mask, HK_RAW_STRUCT_BUF | HK_CBV_BUF);
                        assert_eq!(d.uav_mask, HK_RAW_BUF);
                        assert!(d.srv_ranges.contains(&DxilSrvRangeHk {
                            space: 2, lower_bound: 0, upper_bound: u32::MAX, hk: HK_RAW_BUF,
                        }));
                        skin = true;
                    }
                    ("converttolineardepth.binrfx", "convert_to_linear") if !depth => {
                        let d = detail.unwrap();
                        assert_eq!(d.srv_mask, HK_TEX2D_FLT | HK_CBV_BUF);
                        assert_eq!(d.uav_mask, 0);
                        depth = true;
                    }
                    _ => {}
                }
            }
        }
        for path in dxil_metal::pe::list_pe_binaries(&game_dir, None).unwrap() {
            let bytes = std::fs::read(&path).unwrap();
            let name = path.file_name().unwrap().to_string_lossy();
            for emb in dxil_metal::pe::extract_pe_shaders(name.as_ref(), &bytes) {
                if emb.is_sm4_5 {
                    continue;
                }
                let c = dxil_metal::dxbc::Container::parse(&emb.dxbc_bytes).unwrap();
                if c.part(dxil_metal::dxbc::FOURCC_DXIL).is_none() {
                    continue;
                }
                pe_total += 1;
                let profile = dxil_annotate_handle_kinds_opt(&emb.dxbc_bytes);
                let precise = profile.is_some_and(|(srv, _)| srv != HK_ALL);
                pe_proven += usize::from(precise);
                sm66_proven += usize::from(precise && c.program().is_some_and(|p| (p.major, p.minor) >= (6, 6)));
                if name == "amd_fidelityfx_framegeneration_dx12.dll" && emb.file_offset == 0x2384af0 {
                    // SM6.6 annotateHandle constants have dword1 = 265/521 (SRV F32)
                    // and 261 (UAV U32). The same shader also binds a CBV and a sampler.
                    assert_eq!(profile, Some((HK_TEX2D_FLT | HK_CBV_BUF, HK_TEX2D_INT)));
                    framegen = true;
                }
            }
        }
        eprintln!(
            "g51 DXIL corpus: {rfx_proven}/{rfx_total} bundle programs and {pe_proven}/{pe_total} PE programs narrowed SRV masks ({sm66_proven} PE SM6.6+)"
        );
        assert!(skin && depth && framegen, "missing expected Alan Wake 2 v1.2.10 shader fixtures");
    }
}
