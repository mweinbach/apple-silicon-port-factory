//! GPU- and OS-independent Metal 4 PSO recipe manifest (`records.p2m`) and parallel
//! cold-device prewarming engine.
//!
//! Why this exists:
//! - Tier 1 (`dxil_metal::ShaderCache`, `.aw2s`) stores portable DXIL/DXBC → `.metallib` (AIR)
//!   bytecode that can be copied across Apple Silicon Macs.
//! - Tier 2 (`dxil_metal::pso_cache::PipelineCache`, `.mtl4a`) stores AGX GPU machine code that
//!   is strictly tied to one `(MTLDevice.name, macOS build)`. Moving a converted `.app` bundle
//!   to a new Mac or updating macOS creates a fresh empty device directory (`is_cold_device()`).
//! - This module records the compact, portable D3D12/D3D11 PSO recipes (content-addressed
//!   DXIL/DXBC + canonical RTS0 blobs + fixed-function state) so that on first launch on a new
//!   Mac or after a macOS update (or via `port-runner --prewarm-shaders`), all recorded PSOs can
//!   be compiled in parallel across CPU cores directly into the new device's `MTL4Archive` before
//!   the game starts rendering.

use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use dxil_metal::hash::{hash128, xxh64};
use dxil_metal::pso_cache::{PipelineCache, PipelineCacheConfig};
use dxil_metal::root_signature::RootSignature;
use dxil_metal::{CompileRequest, InputElement, ShaderPipeline};

use crate::mesh::{MeshDesc, MeshPipeline};
use crate::metal::pipeline::{ComputePipeline, GraphicsDesc, GraphicsPipeline};
use crate::state::{BlendDesc, DepthStencilState, RasterState, RenderTargetBlendDesc};

const RECORDS: &str = "records.p2m";
const RECORD_VERSION: u8 = 1;
const KIND_COMPUTE: u8 = 1;
const KIND_GRAPHICS: u8 = 2;
const KIND_MESH: u8 = 3;
const KIND_GRAPHICS_ICB: u8 = 4;
const KIND_MESH_ICB: u8 = 5;
const KIND_GEOMETRY: u8 = 6;
const KIND_TESSELLATION: u8 = 7;

std::thread_local! {
    static IN_PREWARM: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

struct PrewarmGuard;

impl PrewarmGuard {
    fn enter() -> Self {
        IN_PREWARM.with(|c| c.set(true));
        Self
    }
}

impl Drop for PrewarmGuard {
    fn drop(&mut self) {
        IN_PREWARM.with(|c| c.set(false));
    }
}

pub(crate) fn is_prewarming() -> bool {
    IN_PREWARM.with(|c| c.get())
}

// ---------------------------------------------------------------------------
// Record types & deterministic binary codec (self-contained, zero serde deps)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PsoRecipe {
    Compute {
        rs_hash: u128,
        cs_hash: u128,
        pin_tg: bool,
        immut_opt: bool,
    },
    Graphics {
        rs_hash: u128,
        vs_hash: u128,
        ps_hash: Option<u128>,
        blend: BlendDesc,
        sample_mask: u32,
        input_layout: Vec<InputElement>,
        strip_cut: u32,
        topology_type: u32,
        rtv_formats: Vec<u32>,
        dsv_format: u32,
        sample_count: u32,
        lazy_icb: bool,
        with_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    },
    Mesh {
        rs_hash: u128,
        as_hash: Option<u128>,
        ms_hash: u128,
        ps_hash: Option<u128>,
        blend: BlendDesc,
        sample_mask: u32,
        rtv_formats: Vec<u32>,
        dsv_format: u32,
        sample_count: u32,
        with_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    },
    Geometry {
        rs_hash: u128,
        vs_hash: u128,
        gs_hash: u128,
        ps_hash: Option<u128>,
        blend: BlendDesc,
        sample_mask: u32,
        input_layout: Vec<InputElement>,
        strip_cut: u32,
        topology_type: u32,
        rtv_formats: Vec<u32>,
        dsv_format: u32,
        sample_count: u32,
        immut_opt: bool,
        pso_opt_v2: bool,
    },
    Tessellation {
        rs_hash: u128,
        vs_hash: u128,
        hs_hash: u128,
        ds_hash: u128,
        gs_hash: Option<u128>,
        ps_hash: Option<u128>,
        blend: BlendDesc,
        sample_mask: u32,
        input_layout: Vec<InputElement>,
        strip_cut: u32,
        topology_type: u32,
        rtv_formats: Vec<u32>,
        dsv_format: u32,
        sample_count: u32,
        immut_opt: bool,
        pso_opt_v2: bool,
    },
}

#[derive(Debug, Clone)]
pub struct PsoManifestEntry {
    pub key: u128,
    pub recipe: PsoRecipe,
}

#[derive(Default)]
struct BinWriter {
    buf: Vec<u8>,
}

impl BinWriter {
    fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }
    fn bool(&mut self, v: bool) -> &mut Self {
        self.u8(v as u8)
    }
    fn i32(&mut self, v: i32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn u128(&mut self, v: u128) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn opt_u128(&mut self, v: Option<u128>) -> &mut Self {
        match v {
            Some(x) => self.u8(1).u128(x),
            None => self.u8(0),
        }
    }
    fn str(&mut self, s: &str) -> &mut Self {
        self.u32(s.len() as u32);
        self.buf.extend_from_slice(s.as_bytes());
        self
    }
    fn blend(&mut self, b: &BlendDesc) -> &mut Self {
        self.i32(b.AlphaToCoverageEnable)
            .i32(b.IndependentBlendEnable);
        for rt in &b.RenderTarget {
            self.i32(rt.BlendEnable)
                .i32(rt.LogicOpEnable)
                .u32(rt.SrcBlend)
                .u32(rt.DestBlend)
                .u32(rt.BlendOp)
                .u32(rt.SrcBlendAlpha)
                .u32(rt.DestBlendAlpha)
                .u32(rt.BlendOpAlpha)
                .u32(rt.LogicOp)
                .u8(rt.RenderTargetWriteMask);
        }
        self
    }
    fn input_layout(&mut self, layout: &[InputElement]) -> &mut Self {
        self.u32(layout.len() as u32);
        for e in layout {
            self.str(&e.semantic_name)
                .u32(e.semantic_index)
                .u32(e.format)
                .u32(e.input_slot)
                .u32(e.aligned_byte_offset)
                .u32(e.input_slot_class)
                .u32(e.instance_data_step_rate);
        }
        self
    }
    fn u32_slice(&mut self, xs: &[u32]) -> &mut Self {
        self.u32(xs.len() as u32);
        for &x in xs {
            self.u32(x);
        }
        self
    }
}

struct BinReader<'a> {
    b: &'a [u8],
    pos: usize,
}

impl<'a> BinReader<'a> {
    fn new(b: &'a [u8]) -> Self {
        Self { b, pos: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self.pos.checked_add(n).ok_or("pso manifest: overflow")?;
        let slice = self.b.get(self.pos..end).ok_or("pso manifest: truncated record")?;
        self.pos = end;
        Ok(slice)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn bool(&mut self) -> Result<bool, String> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(format!("pso manifest: invalid bool {v}")),
        }
    }
    fn i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn u128(&mut self) -> Result<u128, String> {
        Ok(u128::from_le_bytes(self.take(16)?.try_into().unwrap()))
    }
    fn opt_u128(&mut self) -> Result<Option<u128>, String> {
        Ok(if self.bool()? { Some(self.u128()?) } else { None })
    }
    fn string(&mut self) -> Result<String, String> {
        let len = self.u32()? as usize;
        if len > 4096 {
            return Err("pso manifest: string too long".into());
        }
        let s = std::str::from_utf8(self.take(len)?).map_err(|_| "pso manifest: invalid utf8")?;
        Ok(s.to_string())
    }
    fn blend(&mut self) -> Result<BlendDesc, String> {
        let alpha_to_cov = self.i32()?;
        let indep = self.i32()?;
        let mut rts = [RenderTargetBlendDesc::default(); 8];
        for rt in &mut rts {
            rt.BlendEnable = self.i32()?;
            rt.LogicOpEnable = self.i32()?;
            rt.SrcBlend = self.u32()?;
            rt.DestBlend = self.u32()?;
            rt.BlendOp = self.u32()?;
            rt.SrcBlendAlpha = self.u32()?;
            rt.DestBlendAlpha = self.u32()?;
            rt.BlendOpAlpha = self.u32()?;
            rt.LogicOp = self.u32()?;
            rt.RenderTargetWriteMask = self.u8()?;
        }
        Ok(BlendDesc {
            AlphaToCoverageEnable: alpha_to_cov,
            IndependentBlendEnable: indep,
            RenderTarget: rts,
        })
    }
    fn input_layout(&mut self) -> Result<Vec<InputElement>, String> {
        let len = self.u32()? as usize;
        if len > 256 {
            return Err("pso manifest: input layout too large".into());
        }
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(InputElement {
                semantic_name: self.string()?,
                semantic_index: self.u32()?,
                format: self.u32()?,
                input_slot: self.u32()?,
                aligned_byte_offset: self.u32()?,
                input_slot_class: self.u32()?,
                instance_data_step_rate: self.u32()?,
            });
        }
        Ok(out)
    }
    fn u32_vec(&mut self, max_len: usize) -> Result<Vec<u32>, String> {
        let len = self.u32()? as usize;
        if len > max_len {
            return Err("pso manifest: vector too large".into());
        }
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(self.u32()?);
        }
        Ok(out)
    }
}

fn encode_recipe(recipe: &PsoRecipe) -> Vec<u8> {
    let mut w = BinWriter::default();
    w.u8(RECORD_VERSION);
    match recipe {
        PsoRecipe::Compute { rs_hash, cs_hash, pin_tg, immut_opt } => {
            w.u8(KIND_COMPUTE)
                .u128(*rs_hash)
                .u128(*cs_hash)
                .bool(*pin_tg)
                .bool(*immut_opt);
        }
        PsoRecipe::Graphics {
            rs_hash,
            vs_hash,
            ps_hash,
            blend,
            sample_mask,
            input_layout,
            strip_cut,
            topology_type,
            rtv_formats,
            dsv_format,
            sample_count,
            lazy_icb,
            with_icb,
            immut_opt,
            pso_opt_v2,
        } => {
            w.u8(if *with_icb { KIND_GRAPHICS_ICB } else { KIND_GRAPHICS })
                .u128(*rs_hash)
                .u128(*vs_hash)
                .opt_u128(*ps_hash)
                .blend(blend)
                .u32(*sample_mask)
                .input_layout(input_layout)
                .u32(*strip_cut)
                .u32(*topology_type)
                .u32_slice(rtv_formats)
                .u32(*dsv_format)
                .u32(*sample_count)
                .bool(*lazy_icb)
                .bool(*immut_opt)
                .bool(*pso_opt_v2);
        }
        PsoRecipe::Mesh {
            rs_hash,
            as_hash,
            ms_hash,
            ps_hash,
            blend,
            sample_mask,
            rtv_formats,
            dsv_format,
            sample_count,
            with_icb,
            immut_opt,
            pso_opt_v2,
        } => {
            w.u8(if *with_icb { KIND_MESH_ICB } else { KIND_MESH })
                .u128(*rs_hash)
                .opt_u128(*as_hash)
                .u128(*ms_hash)
                .opt_u128(*ps_hash)
                .blend(blend)
                .u32(*sample_mask)
                .u32_slice(rtv_formats)
                .u32(*dsv_format)
                .u32(*sample_count)
                .bool(*immut_opt)
                .bool(*pso_opt_v2);
        }
        PsoRecipe::Geometry {
            rs_hash,
            vs_hash,
            gs_hash,
            ps_hash,
            blend,
            sample_mask,
            input_layout,
            strip_cut,
            topology_type,
            rtv_formats,
            dsv_format,
            sample_count,
            immut_opt,
            pso_opt_v2,
        } => {
            w.u8(KIND_GEOMETRY)
                .u128(*rs_hash)
                .u128(*vs_hash)
                .u128(*gs_hash)
                .opt_u128(*ps_hash)
                .blend(blend)
                .u32(*sample_mask)
                .input_layout(input_layout)
                .u32(*strip_cut)
                .u32(*topology_type)
                .u32_slice(rtv_formats)
                .u32(*dsv_format)
                .u32(*sample_count)
                .bool(*immut_opt)
                .bool(*pso_opt_v2);
        }
        PsoRecipe::Tessellation {
            rs_hash,
            vs_hash,
            hs_hash,
            ds_hash,
            gs_hash,
            ps_hash,
            blend,
            sample_mask,
            input_layout,
            strip_cut,
            topology_type,
            rtv_formats,
            dsv_format,
            sample_count,
            immut_opt,
            pso_opt_v2,
        } => {
            w.u8(KIND_TESSELLATION)
                .u128(*rs_hash)
                .u128(*vs_hash)
                .u128(*hs_hash)
                .u128(*ds_hash)
                .opt_u128(*gs_hash)
                .opt_u128(*ps_hash)
                .blend(blend)
                .u32(*sample_mask)
                .input_layout(input_layout)
                .u32(*strip_cut)
                .u32(*topology_type)
                .u32_slice(rtv_formats)
                .u32(*dsv_format)
                .u32(*sample_count)
                .bool(*immut_opt)
                .bool(*pso_opt_v2);
        }
    }
    w.buf
}

fn decode_recipe(payload: &[u8]) -> Result<PsoManifestEntry, String> {
    let key = hash128(payload);
    let mut r = BinReader::new(payload);
    if r.u8()? != RECORD_VERSION {
        return Err("pso manifest: unsupported version".into());
    }
    let kind = r.u8()?;
    let recipe = match kind {
        KIND_COMPUTE => PsoRecipe::Compute {
            rs_hash: r.u128()?,
            cs_hash: r.u128()?,
            pin_tg: r.bool()?,
            immut_opt: r.bool()?,
        },
        KIND_GRAPHICS | KIND_GRAPHICS_ICB => {
            let rs_hash = r.u128()?;
            let vs_hash = r.u128()?;
            let ps_hash = r.opt_u128()?;
            let blend = r.blend()?;
            let sample_mask = r.u32()?;
            let input_layout = r.input_layout()?;
            let strip_cut = r.u32()?;
            let topology_type = r.u32()?;
            let rtv_formats = r.u32_vec(16)?;
            let dsv_format = r.u32()?;
            let sample_count = r.u32()?;
            let lazy_icb = r.bool()?;
            let immut_opt = r.bool()?;
            let pso_opt_v2 = r.bool()?;
            PsoRecipe::Graphics {
                rs_hash,
                vs_hash,
                ps_hash,
                blend,
                sample_mask,
                input_layout,
                strip_cut,
                topology_type,
                rtv_formats,
                dsv_format,
                sample_count,
                lazy_icb,
                with_icb: kind == KIND_GRAPHICS_ICB || !lazy_icb,
                immut_opt,
                pso_opt_v2,
            }
        }
        KIND_MESH | KIND_MESH_ICB => PsoRecipe::Mesh {
            rs_hash: r.u128()?,
            as_hash: r.opt_u128()?,
            ms_hash: r.u128()?,
            ps_hash: r.opt_u128()?,
            blend: r.blend()?,
            sample_mask: r.u32()?,
            rtv_formats: r.u32_vec(16)?,
            dsv_format: r.u32()?,
            sample_count: r.u32()?,
            with_icb: kind == KIND_MESH_ICB,
            immut_opt: r.bool()?,
            pso_opt_v2: r.bool()?,
        },
        KIND_GEOMETRY => PsoRecipe::Geometry {
            rs_hash: r.u128()?,
            vs_hash: r.u128()?,
            gs_hash: r.u128()?,
            ps_hash: r.opt_u128()?,
            blend: r.blend()?,
            sample_mask: r.u32()?,
            input_layout: r.input_layout()?,
            strip_cut: r.u32()?,
            topology_type: r.u32()?,
            rtv_formats: r.u32_vec(16)?,
            dsv_format: r.u32()?,
            sample_count: r.u32()?,
            immut_opt: r.bool()?,
            pso_opt_v2: r.bool()?,
        },
        KIND_TESSELLATION => PsoRecipe::Tessellation {
            rs_hash: r.u128()?,
            vs_hash: r.u128()?,
            hs_hash: r.u128()?,
            ds_hash: r.u128()?,
            gs_hash: r.opt_u128()?,
            ps_hash: r.opt_u128()?,
            blend: r.blend()?,
            sample_mask: r.u32()?,
            input_layout: r.input_layout()?,
            strip_cut: r.u32()?,
            topology_type: r.u32()?,
            rtv_formats: r.u32_vec(16)?,
            dsv_format: r.u32()?,
            sample_count: r.u32()?,
            immut_opt: r.bool()?,
            pso_opt_v2: r.bool()?,
        },
        other => return Err(format!("pso manifest: unknown kind {other}")),
    };
    if r.pos != payload.len() {
        return Err("pso manifest: trailing bytes".into());
    }
    Ok(PsoManifestEntry { key, recipe })
}

fn parse_records(b: &[u8]) -> (Vec<PsoManifestEntry>, bool) {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < b.len() {
        let Some(len) = b.get(pos..pos + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()) as usize) else {
            return (out, false);
        };
        let Some(payload) = b.get(pos + 4..pos + 4 + len) else {
            return (out, false);
        };
        let Some(sum) = b.get(pos + 4 + len..pos + 12 + len) else {
            return (out, false);
        };
        if xxh64(payload, 0).to_le_bytes() != sum {
            return (out, false);
        }
        match decode_recipe(payload) {
            Ok(e) => out.push(e),
            Err(_) => return (out, false),
        }
        pos += 12 + len;
    }
    (out, true)
}

fn blob_path(dir: &Path, hash: u128) -> PathBuf {
    let hex = format!("{hash:032x}");
    dir.join("blobs").join(&hex[..2]).join(format!("{hex}.bin"))
}

fn write_atomic_once(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }
    static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let parent = path.parent().ok_or("pso manifest: missing parent dir")?;
    std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    let tmp = parent.join(format!(
        ".tmp-{}-{}",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&tmp, bytes).map_err(|e| format!("{}: {e}", tmp.display()))?;
    if let Err(e) = std::fs::rename(&tmp, path) {
        let _ = std::fs::remove_file(&tmp);
        if !path.exists() {
            return Err(format!("{}: {e}", path.display()));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Writer (runtime recording) + Reader (new-device prewarming)
// ---------------------------------------------------------------------------

/// Append-only, deduplicated writer for GPU/OS-independent PSO recipes.
pub struct PsoManifestWriter {
    dir: PathBuf,
    file: Mutex<File>,
    seen: Mutex<HashSet<u128>>,
    /// Maps lazy `GraphicsPipeline` ICB PSO keys (`base_key`) to their ready-to-record
    /// `with_icb: true` recipes (whose blobs were already persisted when the primary
    /// graphics PSO was created), so first-demand `icb_pso()` calls can append the ICB
    /// recipe in O(1) without retaining raw DXIL buffers on `GraphicsPipeline`.
    pending_icb: Mutex<HashMap<u128, PsoRecipe>>,
}

impl PsoManifestWriter {
    pub fn open(dir: impl Into<PathBuf>) -> Result<Self, String> {
        let dir = dir.into();
        std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = dir.join(RECORDS);
        let existing = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(format!("{}: {e}", path.display())),
        };
        let (entries, clean) = parse_records(&existing);
        if !clean {
            log::warn!(
                target: "d3d",
                "pso manifest {}: ignoring torn tail after {} record(s)",
                path.display(),
                entries.len()
            );
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(Self {
            dir,
            file: Mutex::new(file),
            seen: Mutex::new(entries.into_iter().map(|e| e.key).collect()),
            pending_icb: Mutex::new(HashMap::new()),
        })
    }

    /// `PORT_PSO_MANIFEST` / `AW2_PSO_MANIFEST` = dir or `off`/`0`/empty to disable.
    /// Default: `<pso cache root>/pso_manifest` when disk PSO caching is enabled.
    pub fn from_env() -> Option<Self> {
        let dir = match std::env::var_os("PORT_PSO_MANIFEST")
            .or_else(|| std::env::var_os("AW2_PSO_MANIFEST"))
        {
            Some(v) if v.is_empty() || v == "0" || v == "off" => return None,
            Some(v) => PathBuf::from(v),
            None => PipelineCacheConfig::from_env().dir?.join("pso_manifest"),
        };
        match Self::open(&dir) {
            Ok(w) => Some(w),
            Err(e) => {
                log::warn!(target: "d3d", "pso manifest disabled: {e}");
                None
            }
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn len(&self) -> usize {
        self.seen.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn store_blob(&self, bytes: &[u8]) -> Result<u128, String> {
        let h = hash128(bytes);
        write_atomic_once(&blob_path(&self.dir, h), bytes)?;
        Ok(h)
    }

    fn record_recipe(&self, recipe: PsoRecipe) -> Result<bool, String> {
        let payload = encode_recipe(&recipe);
        let key = hash128(&payload);
        if self.seen.lock().unwrap().contains(&key) {
            return Ok(false);
        }
        let mut rec = Vec::with_capacity(payload.len() + 12);
        rec.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        rec.extend_from_slice(&payload);
        rec.extend_from_slice(&xxh64(&payload, 0).to_le_bytes());
        let mut seen = self.seen.lock().unwrap();
        if !seen.insert(key) {
            return Ok(false);
        }
        self.file
            .lock()
            .unwrap()
            .write_all(&rec)
            .map_err(|e| format!("{}: {e}", self.dir.join(RECORDS).display()))?;
        Ok(true)
    }

    pub fn record_compute(
        &self,
        rs: &RootSignature,
        cs_dxil: &[u8],
        pin_tg: bool,
        immut_opt: bool,
    ) -> Result<bool, String> {
        let rs_blob = rs.desc().to_rts0();
        let rs_hash = self.store_blob(&rs_blob)?;
        let cs_hash = self.store_blob(cs_dxil)?;
        self.record_recipe(PsoRecipe::Compute { rs_hash, cs_hash, pin_tg, immut_opt })
    }

    pub fn record_graphics(
        &self,
        rs: &RootSignature,
        d: &GraphicsDesc,
        lazy_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<bool, String> {
        self.record_graphics_with_icb(rs, d, lazy_icb, !lazy_icb, immut_opt, pso_opt_v2, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_graphics_with_icb(
        &self,
        rs: &RootSignature,
        d: &GraphicsDesc,
        lazy_icb: bool,
        with_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
        icb_key: Option<u128>,
    ) -> Result<bool, String> {
        let rs_blob = rs.desc().to_rts0();
        let rs_hash = self.store_blob(&rs_blob)?;
        let vs_hash = self.store_blob(d.vs)?;
        let ps_hash = d.ps.map(|b| self.store_blob(b)).transpose()?;
        let recipe = PsoRecipe::Graphics {
            rs_hash,
            vs_hash,
            ps_hash,
            blend: d.blend,
            sample_mask: d.sample_mask,
            input_layout: d.input_layout.clone(),
            strip_cut: d.strip_cut,
            topology_type: d.topology_type,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count,
            lazy_icb,
            with_icb,
            immut_opt,
            pso_opt_v2,
        };
        if lazy_icb && !with_icb {
            if let Some(k) = icb_key {
                let mut icb_recipe = recipe.clone();
                if let PsoRecipe::Graphics { with_icb: ref mut flag, .. } = icb_recipe {
                    *flag = true;
                }
                self.pending_icb.lock().unwrap().insert(k, icb_recipe);
            }
        }
        self.record_recipe(recipe)
    }

    /// Record the `with_icb = true` variant of a previously registered lazy-ICB graphics
    /// pipeline when `GraphicsPipeline::icb_pso()` is demanded at runtime.
    pub fn record_graphics_icb_by_key(&self, icb_key: u128) -> Result<bool, String> {
        let recipe = self.pending_icb.lock().unwrap().remove(&icb_key);
        let Some(recipe) = recipe else {
            return Ok(false);
        };
        self.record_recipe(recipe)
    }

    pub fn record_mesh(
        &self,
        rs: &RootSignature,
        d: &MeshDesc,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<bool, String> {
        self.record_mesh_with_icb(rs, d, false, immut_opt, pso_opt_v2)
    }

    pub fn record_mesh_with_icb(
        &self,
        rs: &RootSignature,
        d: &MeshDesc,
        with_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<bool, String> {
        let rs_blob = rs.desc().to_rts0();
        let rs_hash = self.store_blob(&rs_blob)?;
        let as_hash = d.amplification.map(|b| self.store_blob(b)).transpose()?;
        let ms_hash = self.store_blob(d.mesh)?;
        let ps_hash = d.pixel.map(|b| self.store_blob(b)).transpose()?;
        self.record_recipe(PsoRecipe::Mesh {
            rs_hash,
            as_hash,
            ms_hash,
            ps_hash,
            blend: d.blend,
            sample_mask: d.sample_mask,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count,
            with_icb,
            immut_opt,
            pso_opt_v2,
        })
    }

    pub fn record_geometry(
        &self,
        rs: &RootSignature,
        d: &GraphicsDesc,
        gs_dxil: &[u8],
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<bool, String> {
        let rs_blob = rs.desc().to_rts0();
        let rs_hash = self.store_blob(&rs_blob)?;
        let vs_hash = self.store_blob(d.vs)?;
        let gs_hash = self.store_blob(gs_dxil)?;
        let ps_hash = d.ps.map(|b| self.store_blob(b)).transpose()?;
        self.record_recipe(PsoRecipe::Geometry {
            rs_hash,
            vs_hash,
            gs_hash,
            ps_hash,
            blend: d.blend,
            sample_mask: d.sample_mask,
            input_layout: d.input_layout.clone(),
            strip_cut: d.strip_cut,
            topology_type: d.topology_type,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count,
            immut_opt,
            pso_opt_v2,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_tessellation(
        &self,
        rs: &RootSignature,
        d: &GraphicsDesc,
        hs_dxil: &[u8],
        ds_dxil: &[u8],
        gs_dxil: Option<&[u8]>,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<bool, String> {
        let rs_blob = rs.desc().to_rts0();
        let rs_hash = self.store_blob(&rs_blob)?;
        let vs_hash = self.store_blob(d.vs)?;
        let hs_hash = self.store_blob(hs_dxil)?;
        let ds_hash = self.store_blob(ds_dxil)?;
        let gs_hash = gs_dxil.map(|b| self.store_blob(b)).transpose()?;
        let ps_hash = d.ps.map(|b| self.store_blob(b)).transpose()?;
        self.record_recipe(PsoRecipe::Tessellation {
            rs_hash,
            vs_hash,
            hs_hash,
            ds_hash,
            gs_hash,
            ps_hash,
            blend: d.blend,
            sample_mask: d.sample_mask,
            input_layout: d.input_layout.clone(),
            strip_cut: d.strip_cut,
            topology_type: d.topology_type,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count,
            immut_opt,
            pso_opt_v2,
        })
    }
}

/// Process-wide PSO manifest writer (returns `None` when disabled or unwritable).
pub fn global_writer() -> Option<&'static PsoManifestWriter> {
    static W: OnceLock<Option<PsoManifestWriter>> = OnceLock::new();
    W.get_or_init(PsoManifestWriter::from_env).as_ref()
}

pub(crate) fn note_compute(rs: &RootSignature, cs_dxil: &[u8], pin_tg: bool, immut_opt: bool) {
    if is_prewarming() {
        return;
    }
    if let Some(w) = global_writer() {
        if let Err(e) = w.record_compute(rs, cs_dxil, pin_tg, immut_opt) {
            log::warn!(target: "d3d", "pso manifest compute record failed: {e}");
        }
    }
}

pub(crate) fn note_graphics(
    rs: &RootSignature,
    d: &GraphicsDesc,
    lazy_icb: bool,
    immut_opt: bool,
    pso_opt_v2: bool,
    icb_key: Option<u128>,
) {
    if is_prewarming() {
        return;
    }
    if let Some(w) = global_writer() {
        if let Err(e) =
            w.record_graphics_with_icb(rs, d, lazy_icb, !lazy_icb, immut_opt, pso_opt_v2, icb_key)
        {
            log::warn!(target: "d3d", "pso manifest graphics record failed: {e}");
        }
    }
}

pub(crate) fn note_graphics_icb(icb_key: u128) {
    if is_prewarming() {
        return;
    }
    if let Some(w) = global_writer() {
        if let Err(e) = w.record_graphics_icb_by_key(icb_key) {
            log::warn!(target: "d3d", "pso manifest graphics ICB record failed: {e}");
        }
    }
}

pub(crate) fn note_mesh(rs: &RootSignature, d: &MeshDesc, immut_opt: bool, pso_opt_v2: bool) {
    if is_prewarming() {
        return;
    }
    if let Some(w) = global_writer() {
        if let Err(e) = w.record_mesh(rs, d, immut_opt, pso_opt_v2) {
            log::warn!(target: "d3d", "pso manifest mesh record failed: {e}");
        }
    }
}

pub(crate) fn note_geometry(
    rs: &RootSignature,
    d: &GraphicsDesc,
    gs_dxil: &[u8],
    immut_opt: bool,
    pso_opt_v2: bool,
) {
    if is_prewarming() {
        return;
    }
    if let Some(w) = global_writer() {
        if let Err(e) = w.record_geometry(rs, d, gs_dxil, immut_opt, pso_opt_v2) {
            log::warn!(target: "d3d", "pso manifest geometry record failed: {e}");
        }
    }
}

pub(crate) fn note_tessellation(
    rs: &RootSignature,
    d: &GraphicsDesc,
    hs_dxil: &[u8],
    ds_dxil: &[u8],
    gs_dxil: Option<&[u8]>,
    immut_opt: bool,
    pso_opt_v2: bool,
) {
    if is_prewarming() {
        return;
    }
    if let Some(w) = global_writer() {
        if let Err(e) =
            w.record_tessellation(rs, d, hs_dxil, ds_dxil, gs_dxil, immut_opt, pso_opt_v2)
        {
            log::warn!(target: "d3d", "pso manifest tessellation record failed: {e}");
        }
    }
}

/// Loaded PSO manifest directory (read-only).
pub struct PsoManifest {
    dir: PathBuf,
    pub entries: Vec<PsoManifestEntry>,
}

impl PsoManifest {
    /// Discovers existing PSO manifest directories in priority order: writable user manifest
    /// plus any read-only bundled fallback (`PORT_PSO_MANIFEST_FALLBACK`,
    /// `<PORT_SHADER_CACHE_FALLBACK>/pso/pso_manifest`, `Contents/Resources/ShaderCache/pso/pso_manifest`).
    pub fn discover_dirs() -> Vec<PathBuf> {
        let mut out = Vec::new();
        let mut push_if_valid = |d: PathBuf| {
            if d.join(RECORDS).is_file() && !out.contains(&d) {
                out.push(d);
            }
        };
        match std::env::var_os("PORT_PSO_MANIFEST")
            .or_else(|| std::env::var_os("AW2_PSO_MANIFEST"))
        {
            Some(v) if v.is_empty() || v == "0" || v == "off" => {}
            Some(v) => push_if_valid(PathBuf::from(v)),
            None => {
                if let Some(d) = PipelineCacheConfig::from_env().dir {
                    push_if_valid(d.join("pso_manifest"));
                }
            }
        }
        if let Some(v) = std::env::var_os("PORT_PSO_MANIFEST_FALLBACK").filter(|v| !v.is_empty()) {
            push_if_valid(PathBuf::from(v));
        }
        if let Some(v) = std::env::var_os("PORT_SHADER_CACHE_FALLBACK").filter(|v| !v.is_empty()) {
            push_if_valid(PathBuf::from(v).join("pso/pso_manifest"));
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                push_if_valid(parent.join("ShaderCache/pso/pso_manifest"));
                push_if_valid(parent.join("../Resources/ShaderCache/pso/pso_manifest"));
            }
        }
        out
    }

    pub fn load(dir: impl Into<PathBuf>) -> Result<Self, String> {
        let dir = dir.into();
        let path = dir.join(RECORDS);
        let b = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let (entries, clean) = parse_records(&b);
        if !clean {
            log::warn!(
                target: "d3d",
                "pso manifest {}: ignoring torn tail after {} record(s)",
                path.display(),
                entries.len()
            );
        }
        Ok(Self { dir, entries })
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn read_blob(&self, hash: u128) -> Result<Vec<u8>, String> {
        let p = blob_path(&self.dir, hash);
        let b = std::fs::read(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        if hash128(&b) != hash {
            return Err(format!("{}: blob hash mismatch", p.display()));
        }
        Ok(b)
    }
}

// ---------------------------------------------------------------------------
// Parallel cold-device shader & Metal 4 PSO prewarmer
// ---------------------------------------------------------------------------

/// Progress update emitted during [`prewarm_for_current_device_with_progress`].
#[derive(Debug, Clone)]
pub struct PrewarmProgress {
    pub stage: &'static str,
    pub done: usize,
    pub total: usize,
    pub device_identity: String,
}

/// Returns `Some((device_identity, has_work))` if manifests exist and prewarming will actually
/// run for `(only_if_cold)`. Fast (<1 ms) check so GUI launchers can skip creating a progress
/// window on warm launches or when no manifests are present.
pub fn prewarm_needed(only_if_cold: bool) -> Option<String> {
    let pso_dirs = PsoManifest::discover_dirs();
    let shader_dirs = dxil_metal::manifest::Manifest::discover_dirs();
    if pso_dirs.is_empty() && shader_dirs.is_empty() {
        return None;
    }
    let cache = crate::metal::pipeline::pso_cache().ok()?;
    if only_if_cold && !cache.is_cold_device() {
        return None;
    }
    Some(cache.device_identity_str().to_string())
}

/// Summary returned by [`prewarm_for_current_device`] and [`prewarm_into_cache`].
#[derive(Debug, Clone, Default)]
pub struct PrewarmReport {
    pub device_identity: String,
    pub was_cold_device: bool,
    pub skipped_warm: bool,
    pub tier1_total: usize,
    pub tier1_compiled: usize,
    pub tier1_failed: usize,
    pub pso_total: usize,
    pub pso_compiled: usize,
    pub pso_failed: usize,
    pub archive_written: Option<PathBuf>,
    pub elapsed_ms: u64,
}

fn default_prewarm_jobs() -> usize {
    std::env::var("PORT_PREWARM_SHADER_THREADS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&n| n > 0)
        .unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map_or(4, |n| n.get().clamp(1, 8))
        })
}

/// Prewarms all unique PSO recipes from `manifest_dirs` in parallel into `cache`
/// (and flushes `cache` to disk when any PSO compiled).
pub fn prewarm_into_cache(
    cache: &PipelineCache,
    manifest_dirs: &[PathBuf],
    jobs: usize,
) -> Result<PrewarmReport, String> {
    prewarm_into_cache_with_progress(cache, manifest_dirs, jobs, |_| {})
}

pub fn prewarm_into_cache_with_progress(
    cache: &PipelineCache,
    manifest_dirs: &[PathBuf],
    jobs: usize,
    on_progress: impl Fn(PrewarmProgress) + Send + Sync,
) -> Result<PrewarmReport, String> {
    let t0 = Instant::now();
    let mut report = PrewarmReport {
        device_identity: cache.device_identity_str().to_string(),
        was_cold_device: cache.is_cold_device(),
        ..Default::default()
    };

    let mut manifests = Vec::new();
    for dir in manifest_dirs {
        match PsoManifest::load(dir) {
            Ok(m) => manifests.push(m),
            Err(e) => log::warn!(target: "d3d", "pso prewarm: skip manifest {}: {e}", dir.display()),
        }
    }

    // Deduplicate entries across user + bundled manifests while tracking which manifest owns the blobs.
    let mut seen = HashSet::new();
    let mut items: Vec<(usize, PsoManifestEntry)> = Vec::new();
    for (m_idx, m) in manifests.iter().enumerate() {
        for e in &m.entries {
            if seen.insert(e.key) {
                items.push((m_idx, e.clone()));
            }
        }
    }
    report.pso_total = items.len();
    if items.is_empty() {
        report.elapsed_ms = t0.elapsed().as_millis() as u64;
        return Ok(report);
    }

    let shader_pipe = ShaderPipeline::global().map_err(|e| format!("shader pipeline: {e}"))?;
    let rs_cache: Mutex<HashMap<u128, Arc<RootSignature>>> = Mutex::new(HashMap::new());
    let next_idx = AtomicUsize::new(0);
    let ok_count = AtomicUsize::new(0);
    let fail_count = AtomicUsize::new(0);
    let finished_count = AtomicUsize::new(0);
    let total_items = items.len();
    let dev_id = report.device_identity.clone();
    on_progress(PrewarmProgress {
        stage: "Metal 4 PSOs",
        done: 0,
        total: total_items,
        device_identity: dev_id.clone(),
    });
    let worker_count = jobs.max(1).min(total_items);

    let load_rs = |m: &PsoManifest, rs_hash: u128| -> Result<Arc<RootSignature>, String> {
        if let Some(rs) = rs_cache.lock().unwrap().get(&rs_hash) {
            return Ok(rs.clone());
        }
        let blob = m.read_blob(rs_hash)?;
        let rs = shader_pipe
            .root_signature(&blob)
            .map_err(|e| format!("root signature {rs_hash:032x}: {e}"))?;
        rs_cache.lock().unwrap().insert(rs_hash, rs.clone());
        Ok(rs)
    };

    std::thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| {
                let _guard = PrewarmGuard::enter();
                loop {
                    let i = next_idx.fetch_add(1, Ordering::Relaxed);
                    let Some((m_idx, entry)) = items.get(i) else {
                        break;
                    };
                    let m = &manifests[*m_idx];
                    let res = match &entry.recipe {
                        PsoRecipe::Compute { rs_hash, cs_hash, pin_tg, immut_opt } => {
                            load_rs(m, *rs_hash)
                                .and_then(|rs| m.read_blob(*cs_hash).map(|b| (rs, b)))
                                .and_then(|(rs, dxil)| {
                                    ComputePipeline::new_in_cache(
                                        cache,
                                        rs,
                                        &dxil,
                                        "prewarm_cs",
                                        *pin_tg,
                                        *immut_opt,
                                    )
                                    .map(|_| ())
                                })
                        }
                        PsoRecipe::Graphics {
                            rs_hash,
                            vs_hash,
                            ps_hash,
                            blend,
                            sample_mask,
                            input_layout,
                            strip_cut,
                            topology_type,
                            rtv_formats,
                            dsv_format,
                            sample_count,
                            lazy_icb,
                            with_icb,
                            immut_opt,
                            pso_opt_v2,
                        } => load_rs(m, *rs_hash).and_then(|rs| {
                            let vs = m.read_blob(*vs_hash)?;
                            let ps = ps_hash.map(|h| m.read_blob(h)).transpose()?;
                            let gd = GraphicsDesc {
                                vs: &vs,
                                ps: ps.as_deref(),
                                blend: *blend,
                                sample_mask: *sample_mask,
                                raster: RasterState::default(),
                                depth_stencil: DepthStencilState::default(),
                                input_layout: input_layout.clone(),
                                strip_cut: *strip_cut,
                                topology_type: *topology_type,
                                rtv_formats: rtv_formats.clone(),
                                dsv_format: *dsv_format,
                                sample_count: *sample_count,
                            };
                            let gp = GraphicsPipeline::new_in_cache(
                                cache,
                                rs,
                                &gd,
                                "prewarm_gfx",
                                *lazy_icb,
                                *immut_opt,
                                *pso_opt_v2,
                            )?;
                            if *with_icb {
                                gp.prewarm_icb_in_cache(cache)?;
                            }
                            Ok(())
                        }),
                        PsoRecipe::Mesh {
                            rs_hash,
                            as_hash,
                            ms_hash,
                            ps_hash,
                            blend,
                            sample_mask,
                            rtv_formats,
                            dsv_format,
                            sample_count,
                            with_icb,
                            immut_opt,
                            pso_opt_v2,
                        } => load_rs(m, *rs_hash).and_then(|rs| {
                            let as_b = as_hash.map(|h| m.read_blob(h)).transpose()?;
                            let ms_b = m.read_blob(*ms_hash)?;
                            let ps_b = ps_hash.map(|h| m.read_blob(h)).transpose()?;
                            let md = MeshDesc {
                                amplification: as_b.as_deref(),
                                mesh: &ms_b,
                                pixel: ps_b.as_deref(),
                                blend: *blend,
                                sample_mask: *sample_mask,
                                raster: RasterState::default(),
                                depth_stencil: DepthStencilState::default(),
                                rtv_formats: rtv_formats.clone(),
                                dsv_format: *dsv_format,
                                sample_count: *sample_count,
                            };
                            MeshPipeline::new_in_cache(
                                cache,
                                rs.clone(),
                                &md,
                                "prewarm_mesh",
                                *immut_opt,
                                *pso_opt_v2,
                            )?;
                            if *with_icb && *immut_opt {
                                MeshPipeline::new_in_cache(
                                    cache,
                                    rs,
                                    &md,
                                    "prewarm_mesh_icb",
                                    false,
                                    *pso_opt_v2,
                                )?;
                            }
                            Ok(())
                        }),
                        PsoRecipe::Geometry {
                            rs_hash,
                            vs_hash,
                            gs_hash,
                            ps_hash,
                            blend,
                            sample_mask,
                            input_layout,
                            strip_cut,
                            topology_type,
                            rtv_formats,
                            dsv_format,
                            sample_count,
                            immut_opt,
                            pso_opt_v2,
                        } => load_rs(m, *rs_hash).and_then(|rs| {
                            let vs = m.read_blob(*vs_hash)?;
                            let gs = m.read_blob(*gs_hash)?;
                            let ps = ps_hash.map(|h| m.read_blob(h)).transpose()?;
                            let gd = GraphicsDesc {
                                vs: &vs,
                                ps: ps.as_deref(),
                                blend: *blend,
                                sample_mask: *sample_mask,
                                raster: RasterState::default(),
                                depth_stencil: DepthStencilState::default(),
                                input_layout: input_layout.clone(),
                                strip_cut: *strip_cut,
                                topology_type: *topology_type,
                                rtv_formats: rtv_formats.clone(),
                                dsv_format: *dsv_format,
                                sample_count: *sample_count,
                            };
                            GraphicsPipeline::new_geometry_in_cache(
                                cache,
                                rs,
                                &gd,
                                &gs,
                                "prewarm_gs",
                                *immut_opt,
                                *pso_opt_v2,
                            )
                            .map(|_| ())
                        }),
                        PsoRecipe::Tessellation {
                            rs_hash,
                            vs_hash,
                            hs_hash,
                            ds_hash,
                            gs_hash,
                            ps_hash,
                            blend,
                            sample_mask,
                            input_layout,
                            strip_cut,
                            topology_type,
                            rtv_formats,
                            dsv_format,
                            sample_count,
                            immut_opt,
                            pso_opt_v2,
                        } => load_rs(m, *rs_hash).and_then(|rs| {
                            let vs = m.read_blob(*vs_hash)?;
                            let hs = m.read_blob(*hs_hash)?;
                            let ds = m.read_blob(*ds_hash)?;
                            let gs = gs_hash.map(|h| m.read_blob(h)).transpose()?;
                            let ps = ps_hash.map(|h| m.read_blob(h)).transpose()?;
                            let gd = GraphicsDesc {
                                vs: &vs,
                                ps: ps.as_deref(),
                                blend: *blend,
                                sample_mask: *sample_mask,
                                raster: RasterState::default(),
                                depth_stencil: DepthStencilState::default(),
                                input_layout: input_layout.clone(),
                                strip_cut: *strip_cut,
                                topology_type: *topology_type,
                                rtv_formats: rtv_formats.clone(),
                                dsv_format: *dsv_format,
                                sample_count: *sample_count,
                            };
                            GraphicsPipeline::new_tessellation_in_cache(
                                cache,
                                rs,
                                &gd,
                                &hs,
                                &ds,
                                gs.as_deref(),
                                "prewarm_tess",
                                *immut_opt,
                                *pso_opt_v2,
                            )
                            .map(|_| ())
                        }),
                    };
                    match res {
                        Ok(()) => {
                            ok_count.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(e) => {
                            fail_count.fetch_add(1, Ordering::Relaxed);
                            log::warn!(
                                target: "d3d",
                                "pso prewarm: recipe {:032x} failed: {e}",
                                entry.key
                            );
                        }
                    }
                    let done = finished_count.fetch_add(1, Ordering::Relaxed) + 1;
                    on_progress(PrewarmProgress {
                        stage: "Metal 4 PSOs",
                        done,
                        total: total_items,
                        device_identity: dev_id.clone(),
                    });
                }
            });
        }
    });

    report.pso_compiled = ok_count.load(Ordering::Relaxed);
    report.pso_failed = fail_count.load(Ordering::Relaxed);
    if report.pso_compiled > 0 {
        match cache.flush() {
            Ok(p) => report.archive_written = p,
            Err(e) => log::warn!(target: "d3d", "pso prewarm: flush failed: {e}"),
        }
    }
    report.elapsed_ms = t0.elapsed().as_millis() as u64;
    Ok(report)
}

/// Prewarms any missing Tier-1 `.metallib` shaders (from recorded `dxil-metal` manifests) and
/// Tier-2 Metal 4 `MTL4Archive` PSOs (from recorded `pso_manifest` recipes) for the current
/// `(MTLDevice, macOS build)`.
///
/// When `only_if_cold` is `true`, returns immediately in <1 ms if no manifests exist OR if the
/// current GPU+OS build already has cached `.mtl4a` archives (`!cache.is_cold_device()`).
pub fn prewarm_for_current_device(
    jobs: Option<usize>,
    only_if_cold: bool,
) -> Result<PrewarmReport, String> {
    prewarm_for_current_device_with_progress(jobs, only_if_cold, |_| {})
}

/// Like [`prewarm_for_current_device`], emitting live progress callbacks suitable for a native
/// macOS first-launch progress UI or CLI status line.
pub fn prewarm_for_current_device_with_progress(
    jobs: Option<usize>,
    only_if_cold: bool,
    on_progress: impl Fn(PrewarmProgress) + Send + Sync,
) -> Result<PrewarmReport, String> {
    let t0 = Instant::now();
    let pso_dirs = PsoManifest::discover_dirs();
    let shader_dirs = dxil_metal::manifest::Manifest::discover_dirs();
    if pso_dirs.is_empty() && shader_dirs.is_empty() {
        return Ok(PrewarmReport {
            skipped_warm: only_if_cold,
            elapsed_ms: t0.elapsed().as_millis() as u64,
            ..Default::default()
        });
    }

    let cache = crate::metal::pipeline::pso_cache()?;
    let was_cold = cache.is_cold_device();
    if only_if_cold && !was_cold {
        return Ok(PrewarmReport {
            device_identity: cache.device_identity_str().to_string(),
            was_cold_device: false,
            skipped_warm: true,
            elapsed_ms: t0.elapsed().as_millis() as u64,
            ..Default::default()
        });
    }

    let jobs = jobs.filter(|&n| n > 0).unwrap_or_else(default_prewarm_jobs);
    log::info!(
        target: "d3d",
        "shader/PSO prewarm starting for {} (cold_device={}, shader_manifests={}, pso_manifests={}, threads={})",
        cache.device_identity_str(),
        was_cold,
        shader_dirs.len(),
        pso_dirs.len(),
        jobs
    );

    // 1. Tier-1 DXIL -> .metallib prewarm (skipped instantly for already-cached entries).
    let mut tier1_total = 0usize;
    let tier1_ok = AtomicUsize::new(0);
    let tier1_fail = AtomicUsize::new(0);
    if !shader_dirs.is_empty() {
        let pipe = ShaderPipeline::global().map_err(|e| format!("shader pipeline: {e}"))?;
        let mut manifests = Vec::new();
        for d in &shader_dirs {
            if let Ok(m) = dxil_metal::manifest::Manifest::load(d) {
                manifests.push(m);
            }
        }
        let mut seen = HashSet::new();
        let mut entries = Vec::new();
        for (mi, m) in manifests.iter().enumerate() {
            for e in &m.entries {
                if seen.insert(e.key) {
                    entries.push((mi, e.clone()));
                }
            }
        }
        tier1_total = entries.len();
        if !entries.is_empty() {
            let next_i = AtomicUsize::new(0);
            let tier1_done = AtomicUsize::new(0);
            let dev_id = cache.device_identity_str().to_string();
            on_progress(PrewarmProgress {
                stage: "Shader Libraries",
                done: 0,
                total: tier1_total,
                device_identity: dev_id.clone(),
            });
            let workers = jobs.min(entries.len());
            std::thread::scope(|scope| {
                for _ in 0..workers {
                    scope.spawn(|| loop {
                        let i = next_i.fetch_add(1, Ordering::Relaxed);
                        let Some((mi, e)) = entries.get(i) else { break };
                        let m = &manifests[*mi];
                        let r = (|| -> Result<(), String> {
                            let dxil = m.dxil(e).map_err(|err| format!("{err}"))?;
                            let rs = e
                                .root_signature
                                .map(|h| {
                                    let b = m.root_signature_blob(h).map_err(|err| format!("{err}"))?;
                                    pipe.root_signature(&b).map_err(|err| format!("{err}"))
                                })
                                .transpose()?;
                            let lrs = e
                                .local_root_signature
                                .map(|h| {
                                    let b = m.root_signature_blob(h).map_err(|err| format!("{err}"))?;
                                    pipe.root_signature(&b).map_err(|err| format!("{err}"))
                                })
                                .transpose()?;
                            let req = CompileRequest {
                                dxil: &dxil,
                                entry_point: e.entry_point.as_deref(),
                                expected_stage: e.expected_stage,
                                root_signature: rs.as_deref(),
                                local_root_signature: lrs.as_deref(),
                                options: &e.options,
                            };
                            pipe.compile(&req).map(|_| ()).map_err(|err| format!("{err}"))
                        })();
                        match r {
                            Ok(()) => {
                                tier1_ok.fetch_add(1, Ordering::Relaxed);
                            }
                            Err(err) => {
                                tier1_fail.fetch_add(1, Ordering::Relaxed);
                                log::warn!(target: "shaders", "tier-1 prewarm {:032x}: {err}", e.key);
                            }
                        }
                        let done = tier1_done.fetch_add(1, Ordering::Relaxed) + 1;
                        on_progress(PrewarmProgress {
                            stage: "Shader Libraries",
                            done,
                            total: tier1_total,
                            device_identity: dev_id.clone(),
                        });
                    });
                }
            });
        }
    }

    // 2. Tier-2 Metal 4 GPU PSO prewarm into the current device's MTL4Archive.
    let mut report = prewarm_into_cache_with_progress(cache, &pso_dirs, jobs, &on_progress)?;
    report.tier1_total = tier1_total;
    report.tier1_compiled = tier1_ok.load(Ordering::Relaxed);
    report.tier1_failed = tier1_fail.load(Ordering::Relaxed);
    report.elapsed_ms = t0.elapsed().as_millis() as u64;

    log::info!(
        target: "d3d",
        "shader/PSO prewarm done for {} in {} ms: tier1 {}/{} ({} failed), pso {}/{} ({} failed), archive={:?}",
        report.device_identity,
        report.elapsed_ms,
        report.tier1_compiled,
        report.tier1_total,
        report.tier1_failed,
        report.pso_compiled,
        report.pso_total,
        report.pso_failed,
        report.archive_written
    );
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignatureDesc, RootSignatureVersion,
    };

    const CS_FIXTURE: &[u8] = include_bytes!("../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil");

    fn fixed_cs_dxil() -> Vec<u8> {
        let mut b = CS_FIXTURE.to_vec();
        let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
        let sum = dxil_metal::hash::dxbc_checksum(&b[20..size]);
        b[4..20].copy_from_slice(&sum);
        b
    }

    fn cs_root_signature() -> Arc<RootSignature> {
        let range = |rt, reg, n| DescriptorRange {
            range_type: rt,
            num_descriptors: n,
            base_shader_register: reg,
            register_space: 0,
            flags: 0,
            offset_in_descriptors_from_table_start: rs_flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
        };
        RootSignature::new(RootSignatureDesc {
            version: RootSignatureVersion::V1_1,
            parameters: vec![
                RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Srv, 0, 1)]),
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::Descriptor(
                        RootDescriptorKind::Uav,
                        RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                    ),
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, 0, 2)]),
                    visibility: 0,
                },
            ],
            static_samplers: vec![],
            flags: 0,
        })
        .unwrap()
    }

    #[test]
    fn pso_manifest_roundtrip_dedup_and_torn_tail() {
        let dir = std::env::temp_dir().join(format!("d3d12_pso_manifest_rt_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let rs = RootSignature::new_deferred(RootSignatureDesc {
            version: RootSignatureVersion::V1_1,
            parameters: vec![],
            static_samplers: vec![],
            flags: 0,
        });
        let cs_bytes = b"DXBC-fake-compute";
        let vs_bytes = b"DXBC-fake-vertex";
        let ps_bytes = b"DXBC-fake-pixel";
        let ms_bytes = b"DXBC-fake-mesh";

        {
            let w = PsoManifestWriter::open(&dir).unwrap();
            assert!(w.record_compute(&rs, cs_bytes, true, true).unwrap());
            assert!(!w.record_compute(&rs, cs_bytes, true, true).unwrap(), "duplicate must dedupe");

            let gd = GraphicsDesc {
                vs: vs_bytes,
                ps: Some(ps_bytes),
                blend: BlendDesc::default(),
                sample_mask: u32::MAX,
                raster: RasterState::default(),
                depth_stencil: DepthStencilState::default(),
                input_layout: vec![InputElement {
                    semantic_name: "POSITION".into(),
                    semantic_index: 0,
                    format: 6,
                    input_slot: 0,
                    aligned_byte_offset: 0,
                    input_slot_class: 0,
                    instance_data_step_rate: 0,
                }],
                strip_cut: 0,
                topology_type: 3,
                rtv_formats: vec![28, 10],
                dsv_format: 40,
                sample_count: 1,
            };
            assert!(w.record_graphics_with_icb(&rs, &gd, true, false, true, false, Some(0x9999)).unwrap());
            assert!(!w.record_graphics(&rs, &gd, true, true, false).unwrap());
            // Demanding the ICB variant by key appends the distinct `with_icb = true` recipe.
            assert!(w.record_graphics_icb_by_key(0x9999).unwrap());
            assert!(!w.record_graphics_icb_by_key(0x9999).unwrap(), "second ICB demand must dedupe");

            let md = MeshDesc {
                amplification: None,
                mesh: ms_bytes,
                pixel: Some(ps_bytes),
                blend: BlendDesc::default(),
                sample_mask: u32::MAX,
                raster: RasterState::default(),
                depth_stencil: DepthStencilState::default(),
                rtv_formats: vec![28],
                dsv_format: 40,
                sample_count: 1,
            };
            assert!(w.record_mesh(&rs, &md, true, false).unwrap());
            assert!(w.record_mesh_with_icb(&rs, &md, true, true, false).unwrap());
            assert!(w.record_geometry(&rs, &gd, b"DXBC-fake-geom", true, false).unwrap());
            assert!(
                w.record_tessellation(&rs, &gd, b"DXBC-fake-hull", b"DXBC-fake-domain", Some(b"DXBC-fake-geom"), true, false)
                    .unwrap()
            );
            assert_eq!(w.len(), 7);
        }

        // Append a torn tail and confirm reload ignores it cleanly.
        {
            let mut f = OpenOptions::new().append(true).open(dir.join(RECORDS)).unwrap();
            f.write_all(&[0x44, 0x00, 0x00, 0x00, 0xde, 0xad]).unwrap();
        }
        let m = PsoManifest::load(&dir).unwrap();
        assert_eq!(m.entries.len(), 7);
        assert!(m.entries.iter().any(|e| matches!(e.recipe, PsoRecipe::Graphics { lazy_icb: true, with_icb: true, .. })));
        assert!(m.entries.iter().any(|e| matches!(e.recipe, PsoRecipe::Mesh { with_icb: true, .. })));
        assert!(m.entries.iter().any(|e| matches!(e.recipe, PsoRecipe::Geometry { .. })));
        assert!(m.entries.iter().any(|e| matches!(e.recipe, PsoRecipe::Tessellation { gs_hash: Some(_), .. })));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn simulated_new_device_prewarm_populates_cold_mtl4_archive() {
        let Some(dev) = crate::metal::device() else {
            eprintln!("skipping: no Metal 4 device");
            return;
        };
        if !dxil_metal::converter_available() {
            eprintln!("skipping: Metal Shader Converter not installed");
            return;
        }
        let root = std::env::temp_dir().join(format!("d3d12_cold_device_prewarm_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let manifest_dir = root.join("pso_manifest");
        let new_device_pso_dir = root.join("new_device_pso");

        let rs = cs_root_signature();
        let dxil = fixed_cs_dxil();

        // Device A records the portable PSO recipe.
        let writer = PsoManifestWriter::open(&manifest_dir).unwrap();
        assert!(writer.record_compute(&rs, &dxil, true, true).unwrap());
        drop(writer);

        // Device B starts completely cold (0 archives for this GPU/OS directory).
        let cfg = PipelineCacheConfig {
            dir: Some(new_device_pso_dir.clone()),
            max_archives: 16,
            flush_on_drop: false,
        };
        let cold_cache = PipelineCache::open(&dev.dev, &cfg).unwrap();
        assert!(cold_cache.is_cold_device(), "Device B must start cold");

        // Prewarm in parallel directly into Device B's cold PipelineCache.
        let rep = prewarm_into_cache(&cold_cache, std::slice::from_ref(&manifest_dir), 2).unwrap();
        assert!(rep.was_cold_device);
        assert_eq!(rep.pso_total, 1);
        assert_eq!(rep.pso_compiled, 1);
        assert_eq!(rep.pso_failed, 0);
        assert!(rep.archive_written.as_ref().is_some_and(|p| p.is_file()));
        drop(cold_cache);

        // Next launch on Device B: warm! Rebuilding the exact same ComputePipeline hits the archive with 0 compiles.
        let warm_cache = PipelineCache::open(&dev.dev, &cfg).unwrap();
        assert!(!warm_cache.is_cold_device(), "Device B must now be warm after prewarm");
        let _pso = ComputePipeline::new_in_cache(&warm_cache, rs, &dxil, "verify_warm", true, true).unwrap();
        let st = warm_cache.stats();
        assert_eq!((st.archives_loaded, st.archive_hits, st.compiles), (1, 1, 0), "{st:?}");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn prewarm_compiles_with_icb_graphics_variant_for_zero_gameplay_compiles() {
        let Some(dev) = crate::metal::device() else {
            return;
        };
        if !dxil_metal::converter_available() {
            return;
        }
        let vs_dxil: &[u8] = include_bytes!("../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil");
        let root = std::env::temp_dir().join(format!("d3d12_icb_prewarm_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let manifest_dir = root.join("pso_manifest");
        let pso_dir = root.join("pso_cache");

        let rs = RootSignature::new(RootSignatureDesc {
            version: RootSignatureVersion::V1_1,
            parameters: vec![],
            static_samplers: vec![],
            flags: 0,
        })
        .unwrap();
        let gd = GraphicsDesc {
            vs: vs_dxil,
            ps: None,
            blend: BlendDesc::default(),
            sample_mask: u32::MAX,
            raster: RasterState::default(),
            depth_stencil: DepthStencilState::default(),
            input_layout: vec![],
            strip_cut: 0,
            topology_type: 3,
            rtv_formats: vec![],
            dsv_format: 40,
            sample_count: 1,
        };

        let writer = PsoManifestWriter::open(&manifest_dir).unwrap();
        assert!(writer.record_graphics_with_icb(&rs, &gd, true, true, true, false, None).unwrap());
        drop(writer);

        let cfg = PipelineCacheConfig {
            dir: Some(pso_dir.clone()),
            max_archives: 16,
            flush_on_drop: false,
        };
        let cold = PipelineCache::open(&dev.dev, &cfg).unwrap();
        let rep = prewarm_into_cache(&cold, std::slice::from_ref(&manifest_dir), 1).unwrap();
        assert_eq!(rep.pso_compiled, 1);
        assert_eq!(rep.pso_failed, 0);
        // Both primary (non-ICB) and ICB variants compiled into the archive!
        assert_eq!(cold.stats().compiles, 2);
        drop(cold);

        // Warm launch: both primary GraphicsPipeline AND prewarm_icb_in_cache hit with 0 compiles!
        let warm = PipelineCache::open(&dev.dev, &cfg).unwrap();
        let gp = GraphicsPipeline::new_in_cache(&warm, rs, &gd, "warm_gfx", true, true, false).unwrap();
        gp.prewarm_icb_in_cache(&warm).unwrap();
        let st = warm.stats();
        assert_eq!((st.archive_hits, st.compiles), (2, 0), "{st:?}");

        let _ = std::fs::remove_dir_all(&root);
    }
}
