//! Command list state types, barrier tracking primitives, and extracted sub-structs.

use crate::metal::binding::{RootState, UploadRing};
use crate::metal::encstats as stats;
use crate::metal::pipeline::{ComputeBind, GraphicsPipeline, HandleKindProfile, MAX_VERTEX_SLOTS};
use crate::metal::resource::{push_subrange, BufVaCache, MtlTexture, ResRegistry, Resource, SubRange};
use crate::types::Rect;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::sync::{Arc, Mutex};

/// What an RTV/DSV descriptor (host memory, 32 bytes) holds.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TargetView {
    /// `*const Resource` (not owning; D3D12 descriptors hold no reference).
    pub resource: usize,
    pub format: u32,
    pub mip: u16,
    pub plane: u16,
    pub first_slice: u32,
    pub slices: u32,
    /// `MTLTextureType` raw value of the view.
    pub ty: u32,
    pub flags: u32,
}
const _: () = assert!(std::mem::size_of::<TargetView>() == 32);

/// `D3D12_RENDER_TARGET_VIEW_DESC` / `D3D12_DEPTH_STENCIL_VIEW_DESC` as raw
/// words: RTV = Format, ViewDimension, union @8 (16 bytes); DSV = Format,
/// ViewDimension, Flags, union @12 (12 bytes).
pub mod view_flags {
    pub const DSV_READ_ONLY_DEPTH: u32 = 1;
    pub const DSV_READ_ONLY_STENCIL: u32 = 2;
    pub const IS_DSV: u32 = 0x100;
}

fn clamp_slices(total: u32, first: u32, size: u32) -> (u32, u32) {
    let f = first.min(total);
    let rem = total.saturating_sub(f);
    (f, if size == u32::MAX { rem } else { size.min(rem) })
}

impl TargetView {
    pub const NULL: TargetView = TargetView {
        resource: 0,
        format: 0,
        mip: 0,
        plane: 0,
        first_slice: 0,
        slices: 0,
        ty: 0,
        flags: 0,
    };

    fn whole(r: &Resource, fmt: MTLPixelFormat) -> TargetView {
        r.register_ptr();
        let arr = if r.desc.Dimension == crate::types::dim::TEXTURE3D {
            r.desc.depth()
        } else {
            r.desc.array_size()
        };
        let ty = match r.texture() {
            Some(t) => t.textureType().0 as u32,
            None => 0,
        };
        TargetView {
            resource: r as *const Resource as usize,
            format: fmt.0 as u32,
            mip: 0,
            plane: 0,
            first_slice: 0,
            slices: arr,
            ty,
            flags: 0,
        }
    }

    /// `CreateRenderTargetView(resource, desc)`; `desc` = 6 u32 words
    /// (Format, ViewDimension, union) or `None`.
    pub fn rtv(r: &Resource, desc: Option<&[u32; 6]>) -> Result<TargetView, String> {
        if r.texture().is_none() {
            return Err("RTV of a buffer is not supported".into());
        }
        let Some(d) = desc else {
            return Ok(Self::whole(r, r.mtl_format));
        };
        let fmt = crate::format::view_format(d[0], r.mtl_format);
        let mut v = Self::whole(r, fmt);
        let u = &d[2..];
        let arr = r.desc.array_size();
        match d[1] {
            2 => {
                v.mip = u[0] as u16; // TEXTURE1D
                v.slices = 1;
            }
            3 | 5 => {
                // TEXTURE1DARRAY / TEXTURE2DARRAY {Mip, First, Size[, Plane]}
                v.mip = u[0] as u16;
                (v.first_slice, v.slices) = clamp_slices(arr, u[1], u[2]);
                if d[1] == 5 {
                    v.plane = u[3] as u16;
                }
            }
            4 => {
                v.mip = u[0] as u16; // TEXTURE2D {Mip, Plane}
                v.plane = u[1] as u16;
                v.slices = 1;
            }
            6 => v.slices = 1, // TEXTURE2DMS
            7 => (v.first_slice, v.slices) = clamp_slices(arr, u[0], u[1]),
            8 => {
                // TEXTURE3D {Mip, FirstW, WSize}
                v.mip = u[0] as u16;
                let depth = (r.desc.depth() >> (u[0].min(15))).max(1);
                (v.first_slice, v.slices) = clamp_slices(depth, u[1], u[2]);
            }
            dim => return Err(format!("RTV view dimension {dim}")),
        }
        Ok(v)
    }

    /// `CreateDepthStencilView(resource, desc)`; `desc` = 6 u32 words
    /// (Format, ViewDimension, Flags, union) or `None`.
    pub fn dsv(r: &Resource, desc: Option<&[u32; 6]>) -> Result<TargetView, String> {
        if r.texture().is_none() {
            return Err("DSV of a buffer".into());
        }
        let Some(d) = desc else {
            let mut v = Self::whole(r, r.mtl_format);
            v.flags = view_flags::IS_DSV;
            return Ok(v);
        };
        let fmt = crate::format::view_format(d[0], r.mtl_format);
        // A DSV always targets the full depth-stencil texture (Metal binds
        // depth and stencil attachments separately from the same texture).
        let fmt = if r.mtl_format == MTLPixelFormat::Depth32Float_Stencil8 {
            r.mtl_format
        } else {
            fmt
        };
        let mut v = Self::whole(r, fmt);
        v.flags = view_flags::IS_DSV | (d[2] & 3);
        let u = &d[3..];
        let arr = r.desc.array_size();
        match d[1] {
            1 | 3 => {
                v.mip = u[0] as u16; // TEXTURE1D / TEXTURE2D
                v.slices = 1;
            }
            2 | 4 => {
                v.mip = u[0] as u16;
                (v.first_slice, v.slices) = clamp_slices(arr, u[1], u[2]);
            }
            5 => v.slices = 1,
            6 => (v.first_slice, v.slices) = clamp_slices(arr, u[0], u[1]),
            dim => return Err(format!("DSV view dimension {dim}")),
        }
        Ok(v)
    }
}

pub(super) enum Encoder {
    None,
    Render(Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>),
    Compute(Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>),
}

/// Active GPU predication (`ID3D12GraphicsCommandList::SetPredication`).
#[derive(Clone, Copy, Debug)]
pub struct Predication {
    pub gpu_va: u64,
    /// `0` = `D3D12_PREDICATION_OP_EQUAL_ZERO` (skip when `== 0`),
    /// `1` = `D3D12_PREDICATION_OP_NOT_EQUAL_ZERO` (skip when `!= 0`).
    pub op: u32,
}

/// A deferred whole-subresource `ClearRenderTargetView`/`ClearDepthStencilView`.
#[derive(Clone, Copy, Debug)]
pub(super) enum ClearKind {
    Color([f32; 4]),
    DepthStencil { depth: Option<f32>, stencil: Option<u8> },
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PendingClear {
    pub(super) view: TargetView,
    pub(super) kind: ClearKind,
    pub(super) defer_frag_after: MTLStages,
    pub(super) defer_frag_req: [u32; 7],
}

impl PendingClear {
    pub(super) fn is_dsv(&self) -> bool {
        matches!(self.kind, ClearKind::DepthStencil { .. })
    }
    /// Same subresource (and for colour, same view format, which defines how
    /// the clear value is encoded).
    pub(super) fn same_key(&self, v: &TargetView, dsv: bool) -> bool {
        self.is_dsv() == dsv
            && self.view.resource == v.resource
            && self.view.mip == v.mip
            && self.view.first_slice == v.first_slice
            && (dsv || self.view.format == v.format)
    }
}

/// One parsed `D3D12_RESOURCE_BARRIER`.
#[derive(Clone, Copy, Debug)]
pub enum Barrier {
    /// (`*const Resource`, 0 = unknown; StateBefore; StateAfter).
    Transition(usize, u32, u32),
    /// UAV barrier on a resource (0 = all UAV accesses).
    Uav(usize),
    /// Aliasing or anything else: full barrier.
    Full,
}

/// GPU stages that access a resource in D3D12 state `s` (bit set).
pub fn state_stages(s: u32) -> MTLStages {
    use MTLStages as S;
    let pre_raster = S::Vertex | S::Object | S::Mesh | S::Dispatch;
    let shader = pre_raster | S::Fragment;
    if s == 0 {
        return S::All; // COMMON / PRESENT
    }
    let mut m = S::empty();
    let mut rest = s;
    let mut take = |bit: u32, st: S| {
        if s & bit != 0 {
            m |= st;
            rest &= !bit;
        }
    };
    take(0x1, shader); // VERTEX_AND_CONSTANT_BUFFER
    take(0x2, S::Vertex); // INDEX_BUFFER
    take(0x4, S::Fragment); // RENDER_TARGET
    take(0x8, shader | S::AccelerationStructure); // UNORDERED_ACCESS
    take(0x10, S::Fragment); // DEPTH_WRITE
    take(0x20, S::Fragment); // DEPTH_READ
    take(0x40, pre_raster | S::AccelerationStructure); // NON_PIXEL_SHADER_RESOURCE
    take(0x80, S::Fragment); // PIXEL_SHADER_RESOURCE
    take(0x100, S::Vertex); // STREAM_OUT
    take(0x200, pre_raster); // INDIRECT_ARGUMENT / PREDICATION (incl. our expansion kernel)
    take(0x400, S::Blit); // COPY_DEST
    take(0x800, S::Blit); // COPY_SOURCE
    take(0x1000, S::Fragment | S::Blit); // RESOLVE_DEST
    take(0x2000, S::Fragment | S::Blit); // RESOLVE_SOURCE
    take(0x40_0000, S::Dispatch | S::AccelerationStructure); // RAYTRACING_ACCELERATION_STRUCTURE
    if rest != 0 {
        return S::All;
    }
    m
}

/// Strip D3D12 state bits impossible for `r`'s resource dimension (`BUFFER` vs
/// `TEXTURE*`) when `r != 0 && s != 0`, preserving `s` if stripping would empty it.
#[inline]
pub fn filter_res_state_bits(r: usize, s: u32) -> u32 {
    if r == 0 || s == 0 {
        return s;
    }
    const BUF_ONLY: u32 = 0x1 | 0x2 | 0x100 | 0x200 | 0x40_0000;
    const TEX_ONLY: u32 = 0x4 | 0x10 | 0x20 | 0x1000 | 0x2000;
    let is_buf = unsafe { (*(r as *const Resource)).desc.Dimension } == crate::types::dim::BUFFER;
    let filtered = if is_buf { s & !TEX_ONLY } else { s & !BUF_ONLY };
    if filtered != 0 { filtered } else { s }
}

/// Textures (`Dimension != BUFFER`) are never RTAS geometry, instance, scratch,
/// or destination buffers (`MTLStages::AccelerationStructure` operates strictly on `MTLBuffer`s).
#[inline]
pub(super) fn strip_nonbuf_accel(r: usize, st: MTLStages) -> MTLStages {
    if r != 0
        && st != MTLStages::All
        && unsafe { (*(r as *const Resource)).desc.Dimension } != crate::types::dim::BUFFER
    {
        st - MTLStages::AccelerationStructure
    } else {
        st
    }
}

/// GPU stages accessing `r` in D3D12 state `s`, filtering impossible state bits
/// for `r`'s dimension (`AW2_BARRIER_TIGHT_V2=1`).
#[inline]
pub fn state_stages_for_res(r: usize, s: u32) -> MTLStages {
    strip_nonbuf_accel(r, state_stages(filter_res_state_bits(r, s)))
}

/// Texture types a layered render pass can clear in one go.
pub(super) fn layered(t: &MtlTexture) -> bool {
    use MTLTextureType as T;
    matches!(
        t.textureType(),
        T::Type2DArray | T::TypeCube | T::TypeCubeArray | T::Type2DMultisampleArray | T::Type3D
    )
}

/// Misaligned non-zero sentinel resource keys representing prior-list `Fragment` texture / buffer
/// writes deferred by `list_start_render_split` (`AW2_GFX_TO_PS_OVERLAP_V2=1`).
pub(super) const SOFT_RES_PRE_LIST_FRAG_TEX: usize = 1;
pub(super) const SOFT_RES_PRE_LIST_FRAG_BUF: usize = 2;
pub(super) const MAX_CS_SOFT_DISP: usize = 64;

static ANY_FRAG_RESOLVE_DST: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static FRAG_RESOLVE_DST_SET: std::sync::OnceLock<Mutex<std::collections::HashSet<usize>>> =
    std::sync::OnceLock::new();

#[inline]
pub(super) fn note_frag_resolve_dst(r: usize) {
    if r != 0 {
        FRAG_RESOLVE_DST_SET
            .get_or_init(|| Mutex::new(std::collections::HashSet::new()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(r);
        ANY_FRAG_RESOLVE_DST.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

#[inline]
pub(super) fn is_frag_resolve_dst(r: usize) -> bool {
    ANY_FRAG_RESOLVE_DST.load(std::sync::atomic::Ordering::Relaxed)
        && FRAG_RESOLVE_DST_SET
            .get()
            .is_some_and(|m| m.lock().unwrap_or_else(|e| e.into_inner()).contains(&r))
}

#[inline]
pub(super) fn sub_stages_preserving_all(src: MTLStages, sub: MTLStages) -> MTLStages {
    if sub == MTLStages::All {
        MTLStages::empty()
    } else if src == MTLStages::All {
        MTLStages::All
    } else {
        src - sub
    }
}

#[inline]
fn same_raster(a: &crate::state::RasterState, b: &crate::state::RasterState) -> bool {
    a.fill == b.fill
        && a.cull == b.cull
        && a.winding == b.winding
        && a.depth_bias.to_bits() == b.depth_bias.to_bits()
        && a.slope_scale.to_bits() == b.slope_scale.to_bits()
        && a.bias_clamp.to_bits() == b.bias_clamp.to_bits()
        && a.depth_clip == b.depth_clip
}

#[inline]
fn same_viewport(a: &MTLViewport, b: &MTLViewport) -> bool {
    a.originX.to_bits() == b.originX.to_bits()
        && a.originY.to_bits() == b.originY.to_bits()
        && a.width.to_bits() == b.width.to_bits()
        && a.height.to_bits() == b.height.to_bits()
        && a.znear.to_bits() == b.znear.to_bits()
        && a.zfar.to_bits() == b.zfar.to_bits()
}

/// Dirty bitmask bits for `GfxState.dirty` so updating one dynamic field (e.g. a
/// viewport or scissor) skips re-evaluating PSO, raster, depth-stencil, blend, and stencil.
pub(super) const GFX_DIRTY_PSO: u8 = 1 << 0;
pub(super) const GFX_DIRTY_VIEWPORTS: u8 = 1 << 1;
pub(super) const GFX_DIRTY_SCISSORS: u8 = 1 << 2;
pub(super) const GFX_DIRTY_BLEND: u8 = 1 << 3;
pub(super) const GFX_DIRTY_STENCIL: u8 = 1 << 4;
pub(super) const GFX_DIRTY_DEPTH_BOUNDS: u8 = 1 << 5;
pub(super) const GFX_DIRTY_SHARED: u8 =
    GFX_DIRTY_VIEWPORTS | GFX_DIRTY_SCISSORS | GFX_DIRTY_BLEND | GFX_DIRTY_STENCIL;
pub(super) const GFX_DIRTY_ALL: u8 = GFX_DIRTY_PSO | GFX_DIRTY_SHARED | GFX_DIRTY_DEPTH_BOUNDS;

/// Per-open-render-encoder state cache (`AW2_FAST_ENC_STATE=1`).
#[derive(Default)]
pub(super) struct EncRenderCache {
    pub(super) mtl_pso_ptr: usize,
    pub(super) ds_ptr: usize,
    pub(super) raster: Option<crate::state::RasterState>,
    pub(super) stencil_refs: Option<(u32, u32)>,
    pub(super) blend_bits: Option<[u32; 4]>,
    pub(super) viewports: Vec<MTLViewport>,
    pub(super) scissors: Vec<MTLScissorRect>,
    pub(super) mesh_stages_bound: bool,
}

impl EncRenderCache {
    #[inline]
    pub(super) fn reset(&mut self) {
        self.mtl_pso_ptr = 0;
        self.ds_ptr = 0;
        self.raster = None;
        self.stencil_refs = None;
        self.blend_bits = None;
        self.viewports.clear();
        self.scissors.clear();
        self.mesh_stages_bound = false;
    }

    #[inline]
    pub(super) fn apply_pso(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        pso: &crate::metal::pipeline::MtlRenderPipeline,
        fast: bool,
    ) {
        let ptr = pso as *const _ as usize;
        if fast && self.mtl_pso_ptr == ptr {
            stats::inc(&stats::c::enc_state_skip);
            return;
        }
        e.setRenderPipelineState(pso);
        self.mtl_pso_ptr = ptr;
    }

    #[inline]
    pub(super) fn apply_raster(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        raster: &crate::state::RasterState,
        fast: bool,
    ) {
        if fast && self.raster.as_ref().is_some_and(|r| same_raster(r, raster)) {
            stats::inc(&stats::c::enc_state_skip);
            return;
        }
        raster.apply(e);
        self.raster = Some(*raster);
    }

    #[inline]
    pub(super) fn apply_ds(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        ds: &crate::metal::pipeline::MtlDepthStencil,
        fast: bool,
    ) {
        let ptr = ds as *const _ as usize;
        if fast && self.ds_ptr == ptr {
            stats::inc(&stats::c::enc_state_skip);
            return;
        }
        e.setDepthStencilState(Some(ds));
        self.ds_ptr = ptr;
    }

    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub(super) fn apply_shared(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        size: (usize, usize),
        viewports: &[MTLViewport],
        scissors: &[Rect],
        blend_factor: [f32; 4],
        stencil_ref: u32,
        stencil_back_ref: u32,
        fast: bool,
    ) {
        self.apply_shared_dirty(
            e,
            size,
            GFX_DIRTY_SHARED,
            viewports,
            scissors,
            blend_factor,
            stencil_ref,
            stencil_back_ref,
            fast,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn apply_shared_dirty(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        size: (usize, usize),
        dirty: u8,
        viewports: &[MTLViewport],
        scissors: &[Rect],
        blend_factor: [f32; 4],
        stencil_ref: u32,
        stencil_back_ref: u32,
        fast: bool,
    ) {
        if (dirty & GFX_DIRTY_STENCIL) != 0 {
            let sr = (stencil_ref, stencil_back_ref);
            if fast && self.stencil_refs == Some(sr) {
                stats::inc(&stats::c::enc_state_skip);
            } else {
                if stencil_ref == stencil_back_ref {
                    e.setStencilReferenceValue(stencil_ref);
                } else {
                    e.setStencilFrontReferenceValue_backReferenceValue(stencil_ref, stencil_back_ref);
                }
                self.stencil_refs = Some(sr);
            }
        }

        if (dirty & GFX_DIRTY_BLEND) != 0 {
            let bb = blend_factor.map(f32::to_bits);
            if fast && self.blend_bits == Some(bb) {
                stats::inc(&stats::c::enc_state_skip);
            } else {
                e.setBlendColorRed_green_blue_alpha(
                    blend_factor[0],
                    blend_factor[1],
                    blend_factor[2],
                    blend_factor[3],
                );
                self.blend_bits = Some(bb);
            }
        }

        if (dirty & GFX_DIRTY_VIEWPORTS) != 0 {
            let full = [MTLViewport {
                originX: 0.0,
                originY: 0.0,
                width: size.0 as f64,
                height: size.1 as f64,
                znear: 0.0,
                zfar: 1.0,
            }];
            let eff_vps: &[MTLViewport] = if viewports.is_empty() { &full } else { viewports };
            let vp_same = fast
                && self.viewports.len() == eff_vps.len()
                && self.viewports.iter().zip(eff_vps).all(|(a, b)| same_viewport(a, b));
            if vp_same {
                stats::inc(&stats::c::enc_state_skip);
            } else {
                unsafe {
                    e.setViewports_count(
                        std::ptr::NonNull::new(eff_vps.as_ptr() as *mut MTLViewport).unwrap(),
                        eff_vps.len(),
                    );
                }
                self.viewports.clear();
                self.viewports.extend_from_slice(eff_vps);
            }
        }

        if (dirty & GFX_DIRTY_SCISSORS) != 0 {
            let mut sc_buf = [MTLScissorRect { x: 0, y: 0, width: size.0, height: size.1 }; 16];
            let sc_len = if scissors.is_empty() {
                1
            } else {
                let n = scissors.len().min(sc_buf.len());
                for (dst, r) in sc_buf[..n].iter_mut().zip(&scissors[..n]) {
                    let x = (r.left.max(0) as usize).min(size.0);
                    let y = (r.top.max(0) as usize).min(size.1);
                    let r_ = (r.right.max(0) as usize).min(size.0);
                    let b = (r.bottom.max(0) as usize).min(size.1);
                    *dst = MTLScissorRect {
                        x,
                        y,
                        width: r_.saturating_sub(x),
                        height: b.saturating_sub(y),
                    };
                }
                n
            };
            let eff_scs = &sc_buf[..sc_len];
            let sc_same = fast
                && self.scissors.len() == eff_scs.len()
                && self.scissors.iter().zip(eff_scs).all(|(a, b)| {
                    a.x == b.x && a.y == b.y && a.width == b.width && a.height == b.height
                });
            if sc_same {
                stats::inc(&stats::c::enc_state_skip);
            } else {
                unsafe {
                    e.setScissorRects_count(
                        std::ptr::NonNull::new(eff_scs.as_ptr() as *mut MTLScissorRect).unwrap(),
                        sc_len,
                    );
                }
                self.scissors.clear();
                self.scissors.extend_from_slice(eff_scs);
            }
        }
    }
}

/// Access-kind refinement on an intra-compute obligation (`AW2_IB_EXACT_OBL=1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OblKind {
    Any,
    ClassSrv(u8),
    UavOrCbv,
    UavOnly,
    UavWriteOnly,
    ClassUavOnly(u8),
}

impl OblKind {
    #[inline]
    pub(super) fn join(self, other: Self) -> Self {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => Self::Any,
            (Self::ClassUavOnly(a), Self::ClassUavOnly(b)) => Self::ClassUavOnly(a | b),
            (Self::ClassUavOnly(_), _) | (_, Self::ClassUavOnly(_)) => Self::Any,
            (Self::ClassSrv(a), Self::ClassSrv(b)) => Self::ClassSrv(a | b),
            (Self::ClassSrv(c), Self::UavOrCbv | Self::UavOnly | Self::UavWriteOnly)
            | (Self::UavOrCbv | Self::UavOnly | Self::UavWriteOnly, Self::ClassSrv(c)) => {
                Self::ClassSrv(c)
            }
            (Self::UavOrCbv, Self::UavOrCbv | Self::UavOnly | Self::UavWriteOnly)
            | (Self::UavOnly | Self::UavWriteOnly, Self::UavOrCbv) => Self::UavOrCbv,
            (Self::UavOnly, Self::UavOnly | Self::UavWriteOnly)
            | (Self::UavWriteOnly, Self::UavOnly) => Self::UavOnly,
            (Self::UavWriteOnly, Self::UavWriteOnly) => Self::UavWriteOnly,
        }
    }
}

pub(super) const STAGE_FRAGMENT: usize = 3;
pub(super) const STAGE_DISPATCH: usize = 4;
pub(super) const STAGE_BLIT: usize = 5;
pub(super) const STAGE_ACCEL: usize = 6;
pub(super) const STAGE_BITS: [MTLStages; 7] = [
    MTLStages::Vertex,
    MTLStages::Object,
    MTLStages::Mesh,
    MTLStages::Fragment,
    MTLStages::Dispatch,
    MTLStages::Blit,
    MTLStages::AccelerationStructure,
];

/// Transitive queue-barrier synchronization watermark per Metal stage (`g34` / `g35` / `g47`).
#[derive(Clone, Copy, Debug)]
pub(super) struct StageSync {
    pub(super) pre_list_all: bool,
    pub(super) pre_list_stages: MTLStages,
    pub(super) gen: [u32; 7],
    pub(super) disp_gen: u32,
    pub(super) as_gen: u32,
    pub(super) gfx_uav_gen: u32,
}

impl Default for StageSync {
    #[inline]
    fn default() -> Self {
        Self {
            pre_list_all: false,
            pre_list_stages: MTLStages::empty(),
            gen: [0; 7],
            disp_gen: 0,
            as_gen: 0,
            gfx_uav_gen: 0,
        }
    }
}

impl StageSync {
    #[inline]
    pub(super) fn merge(&mut self, other: Self) {
        self.pre_list_all |= other.pre_list_all;
        self.pre_list_stages |= other.pre_list_stages;
        for i in 0..7 {
            self.gen[i] = self.gen[i].max(other.gen[i]);
        }
        self.disp_gen = self.disp_gen.max(other.disp_gen);
        self.as_gen = self.as_gen.max(other.as_gen);
        self.gfx_uav_gen = self.gfx_uav_gen.max(other.gfx_uav_gen);
    }
}

/// Inline-backed buffer sub-range list avoiding heap allocation for the common
/// 1–2 sub-range case per intra-compute buffer barrier obligation.
#[derive(Clone, Debug)]
pub(super) enum SmallSpans {
    Inline {
        buf: [SubRange; 2],
        len: u8,
    },
    Heap(Vec<SubRange>),
}

impl SmallSpans {
    #[inline]
    pub(super) const fn new() -> Self {
        Self::Inline {
            buf: [SubRange::whole(0); 2],
            len: 0,
        }
    }

    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        match self {
            Self::Inline { len, .. } => *len == 0,
            Self::Heap(v) => v.is_empty(),
        }
    }

    #[inline]
    pub(super) fn as_slice(&self) -> &[SubRange] {
        match self {
            Self::Inline { buf, len } => &buf[..(*len as usize)],
            Self::Heap(v) => v.as_slice(),
        }
    }

    #[inline]
    pub(super) fn push_coalesced(&mut self, mut s: SubRange) {
        if s.lo >= s.hi {
            return;
        }
        match self {
            Self::Inline { buf, len } => {
                let mut n = *len as usize;
                let mut i = 0;
                while i < n {
                    let e = buf[i];
                    if e.r == s.r && e.lo <= s.hi && s.lo <= e.hi {
                        s.lo = s.lo.min(e.lo);
                        s.hi = s.hi.max(e.hi);
                        n -= 1;
                        buf[i] = buf[n];
                    } else {
                        i += 1;
                    }
                }
                if n < 2 {
                    buf[n] = s;
                    *len = (n + 1) as u8;
                } else {
                    let mut v = Vec::with_capacity(4);
                    v.extend_from_slice(&buf[..n]);
                    v.push(s);
                    *self = Self::Heap(v);
                }
            }
            Self::Heap(v) => push_subrange(v, s),
        }
    }

    #[inline]
    pub(super) fn extend_coalesced(&mut self, src: &Self) {
        for &s in src.as_slice() {
            self.push_coalesced(s);
        }
    }
}

impl std::ops::Deref for SmallSpans {
    type Target = [SubRange];

    #[inline]
    fn deref(&self) -> &[SubRange] {
        self.as_slice()
    }
}

/// One pending intra-compute obligation (`AW2_IB_TIGHT=1`).
#[derive(Clone, Debug)]
pub(super) struct IbObl {
    pub(super) r: usize,
    pub(super) after: MTLStages,
    pub(super) before: MTLStages,
    pub(super) kind: OblKind,
    pub(super) spans: Option<SmallSpans>,
}

#[derive(Clone, Copy)]
pub(super) struct DispatchExact<'a> {
    pub(super) uav_exact: bool,
    pub(super) full_read_exact: bool,
    pub(super) bounded_read_exact: bool,
    pub(super) touches_textures: bool,
    pub(super) unbounded_srv_classes: u8,
    pub(super) uav_classes: u8,
    pub(super) read_classes: u8,
    pub(super) exact_read_only_classes: u8,
    pub(super) uav_srv_classes: u8,
    pub(super) hk: HandleKindProfile,
    pub(super) uavs: &'a [usize],
    pub(super) bounded_reads: &'a [usize],
    pub(super) uav_spans: Option<&'a [SubRange]>,
    pub(super) bounded_read_spans: Option<&'a [SubRange]>,
}

pub(super) struct DynSnapshot {
    pub(super) viewports: Vec<MTLViewport>,
    pub(super) scissors: Vec<Rect>,
    pub(super) blend_factor: [f32; 4],
    pub(super) stencil_ref: u32,
    pub(super) stencil_back_ref: u32,
}

pub(super) struct DirectBind {
    pub(super) heaps: [u64; 2],
    pub(super) root: u64,
    pub(super) pa: u64,
    pub(super) ia: u64,
    pub(super) vbs: Vec<(u32, u64, usize)>,
}

#[derive(Default)]
pub(super) struct CsBindCache {
    pub(super) valid: bool,
    pub(super) pso_ptr: usize,
    pub(super) want_spans: bool,
    pub(super) want_bounded: bool,
    pub(super) reg_gen: u64,
    pub(super) desc_gen: u64,
    pub(super) root_gen: u64,
    pub(super) heap0_base: u64,
    pub(super) tokens: Vec<u64>,
    pub(super) uav_exact: bool,
    pub(super) full_read_exact: bool,
    pub(super) bounded_read_exact: bool,
    pub(super) uavs: Vec<usize>,
    pub(super) reads: Vec<usize>,
    pub(super) uav_spans: Vec<SubRange>,
    pub(super) read_spans: Vec<SubRange>,
}

impl CsBindCache {
    #[inline]
    pub(super) fn matches_gen(
        &self,
        pso_ptr: usize,
        want_spans: bool,
        reg_gen: u64,
        desc_gen: u64,
        root_gen: u64,
        heap0_base: u64,
    ) -> bool {
        self.valid
            && self.pso_ptr == pso_ptr
            && self.want_spans == want_spans
            && self.want_bounded
            && self.reg_gen == reg_gen
            && self.desc_gen == desc_gen
            && self.root_gen == root_gen
            && self.heap0_base == heap0_base
    }

    #[inline]
    pub(super) fn matches_tokens(
        &self,
        pso_ptr: usize,
        want_spans: bool,
        want_bounded: bool,
        reg_gen: u64,
        heap0_base: u64,
        tokens: &[u64],
    ) -> bool {
        self.valid
            && self.pso_ptr == pso_ptr
            && self.want_spans == want_spans
            && self.want_bounded == want_bounded
            && self.reg_gen == reg_gen
            && self.heap0_base == heap0_base
            && self.tokens == tokens
    }

    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub(super) fn store(
        &mut self,
        pso_ptr: usize,
        want_spans: bool,
        want_bounded: bool,
        reg_gen: u64,
        desc_gen: u64,
        root_gen: u64,
        heap0_base: u64,
        tokens: &[u64],
        uav_exact: bool,
        full_read_exact: bool,
        bounded_read_exact: bool,
        uavs: &[usize],
        reads: &[usize],
        uav_spans: &[SubRange],
        read_spans: &[SubRange],
    ) {
        self.valid = true;
        self.pso_ptr = pso_ptr;
        self.want_spans = want_spans;
        self.want_bounded = want_bounded;
        self.reg_gen = reg_gen;
        self.desc_gen = desc_gen;
        self.root_gen = root_gen;
        self.heap0_base = heap0_base;
        self.tokens.clear();
        self.tokens.extend_from_slice(tokens);
        self.uav_exact = uav_exact;
        self.full_read_exact = full_read_exact;
        self.bounded_read_exact = bounded_read_exact;
        self.uavs.clear();
        self.uavs.extend_from_slice(uavs);
        self.reads.clear();
        self.reads.extend_from_slice(reads);
        self.uav_spans.clear();
        if want_spans {
            self.uav_spans.extend_from_slice(uav_spans);
        }
        self.read_spans.clear();
        if want_spans {
            self.read_spans.extend_from_slice(read_spans);
        }
    }
}

/// Per-command-list token-validated cache of resolved graphics draw bindings
/// (`pre_exact`, `ps_exact`, and `uav_exact`), mirroring [`CsBindCache`].
#[derive(Default)]
pub(super) struct GfxBindCache {
    pub(super) valid: bool,
    pub(super) pso_ptr: usize,
    pub(super) flags: u8,
    pub(super) reg_gen: u64,
    pub(super) heap0_base: u64,
    pub(super) tokens: Vec<u64>,
    pub(super) pre_unb: u8,
    pub(super) ps_unb: u8,
    pub(super) uav_unb: bool,
    pub(super) pre_slot_failed: bool,
    pub(super) pre_va_failed: bool,
    pub(super) ps_slot_failed: bool,
    pub(super) pre_exact: Vec<usize>,
    pub(super) ps_exact: Vec<usize>,
    pub(super) uav_exact: Vec<usize>,
}

impl GfxBindCache {
    #[inline]
    pub(super) fn matches(
        &self,
        pso_ptr: usize,
        flags: u8,
        reg_gen: u64,
        heap0_base: u64,
        tokens: &[u64],
    ) -> bool {
        self.valid
            && self.pso_ptr == pso_ptr
            && self.flags == flags
            && self.reg_gen == reg_gen
            && self.heap0_base == heap0_base
            && self.tokens == tokens
    }
}

pub(super) struct DrawSegExact {
    pub(super) eff_reads: u8,
    pub(super) srv_uav_pre_reads: u8,
    pub(super) ps_reads: u8,
    pub(super) ps_srv_uav_reads: u8,
    pub(super) pre_hk: u32,
    pub(super) ps_hk: u32,
    pub(super) pre_unb_hk: u32,
    pub(super) ps_unb_hk: u32,
    pub(super) writes_uav: bool,
    pub(super) vs_writes_uav: bool,
    pub(super) ps_writes_uav: bool,
    pub(super) uses_uav: bool,
    pub(super) pre_unb: u8,
    pub(super) pre_exact: Vec<usize>,
    pub(super) ps_unb: u8,
    pub(super) ps_exact: Vec<usize>,
    pub(super) uav_unb: bool,
    pub(super) uav_exact: Vec<usize>,
}

pub(super) enum DrawOp {
    Icb {
        ex: Option<Retained<crate::metal::indirect_icb::MtlRenderPso>>,
        params_va: u64,
        max_count: u32,
        location: u32,
        icb: Retained<crate::metal::indirect_icb::MtlIcb>,
        range_out: u64,
        args_va: u64,
        args_bytes: u64,
        count_va: u64,
        pred_va: u64,
        pred_op: u32,
    },
    Draw {
        bind: DirectBind,
        prim: MTLPrimitiveType,
        first_vertex: usize,
        vertices: usize,
        instances: usize,
        first_instance: usize,
    },
    DrawIndexed {
        bind: DirectBind,
        prim: MTLPrimitiveType,
        indices: usize,
        index_type: MTLIndexType,
        ib_va: u64,
        ib_len: usize,
        instances: usize,
        base_vertex: isize,
        first_instance: usize,
    },
}

pub(super) struct DrawSeg {
    pub(super) pso: Arc<GraphicsPipeline>,
    pub(super) dyn_idx: u32,
    pub(super) deferred_meta: Option<Box<DrawSegExact>>,
    pub(super) op: DrawOp,
}

/// D3D12 resource state bits that permit GPU writes.
pub(super) const D3D12_WRITE_STATES: u32 =
    0x4 | 0x8 | 0x10 | 0x100 | 0x400 | 0x1000 | 0x20000 | 0x80000 | 0x800000;
/// Known D3D12 non-zero read-only state bits.
pub(super) const D3D12_READ_ONLY_MASK: u32 =
    0x1 | 0x2 | 0x20 | 0x40 | 0x80 | 0x200 | 0x800 | 0x2000 | 0x40_0000 | 0x100_0000 | 0x100_00
        | 0x40000 | 0x200000;

/// Returns `true` when transitioning `s0 -> s1` is a pure read-to-read state
/// change between known non-COMMON D3D12 read states.
#[inline]
pub fn is_read_to_read_transition(s0: u32, s1: u32) -> bool {
    s0 != 0
        && s1 != 0
        && ((s0 | s1) & D3D12_WRITE_STATES) == 0
        && ((s0 | s1) & !D3D12_READ_ONLY_MASK) == 0
}

/// Vertex/object/mesh stages.
pub(super) const PRE_RASTER: MTLStages =
    MTLStages::Vertex.union(MTLStages::Object).union(MTLStages::Mesh);
/// Stages work in a compute/blit encoder runs in.
pub(super) const COMPUTE_STAGES: MTLStages = MTLStages::Dispatch
    .union(MTLStages::Blit)
    .union(MTLStages::AccelerationStructure);

/// No deferred pre-raster obligations. Per-class arrays are indexed 0 =
/// textures, 1 = buffers.
pub(super) const NO_VTX: [Option<(MTLStages, MTLStages)>; 2] = [None, None];

/// Union of two (after, before) barrier obligations.
pub(super) fn merge_vtx(
    a: Option<(MTLStages, MTLStages)>,
    b: Option<(MTLStages, MTLStages)>,
) -> Option<(MTLStages, MTLStages)> {
    match (a, b) {
        (Some((a0, a1)), Some((b0, b1))) => Some((a0 | b0, a1 | b1)),
        (x, None) | (None, x) => x,
    }
}

/// Class bits of barrier resource `r` (`*const Resource`; 0 = any resource).
pub(super) fn res_class(r: usize) -> u8 {
    use crate::metal::pipeline::{READS_BUF, READS_TEX};
    if r == 0 {
        return READS_TEX | READS_BUF;
    }
    if unsafe { (*(r as *const Resource)).desc.Dimension } == crate::types::dim::BUFFER {
        READS_BUF
    } else {
        READS_TEX
    }
}

/// Per-consumer-stage producer stage masks (`AW2_RENDER_SPLIT_STAGE_SYNC=1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SplitPend {
    pub(super) vtx_after: MTLStages,
    pub(super) frag_after: MTLStages,
    pub(super) disp_after: MTLStages,
    pub(super) blit_after: MTLStages,
    pub(super) as_after: MTLStages,
    pub(super) soft_after: [MTLStages; 2],
    pub(super) soft_before: [MTLStages; 2],
    pub(super) vtx_req: [u32; 7],
    pub(super) frag_req: [u32; 7],
    pub(super) disp_req: [u32; 7],
    pub(super) blit_req: [u32; 7],
    pub(super) as_req: [u32; 7],
    pub(super) soft_req: [[u32; 7]; 2],
}

impl Default for SplitPend {
    #[inline]
    fn default() -> Self {
        Self {
            vtx_after: MTLStages::empty(),
            frag_after: MTLStages::empty(),
            disp_after: MTLStages::empty(),
            blit_after: MTLStages::empty(),
            as_after: MTLStages::empty(),
            soft_after: [MTLStages::empty(); 2],
            soft_before: [MTLStages::empty(); 2],
            vtx_req: [0; 7],
            frag_req: [0; 7],
            disp_req: [0; 7],
            blit_req: [0; 7],
            as_req: [0; 7],
            soft_req: [[0; 7]; 2],
        }
    }
}

#[inline]
pub(super) fn merge_req_all(dst: &mut [u32; 7], src: &[u32; 7]) {
    for i in 0..7 {
        dst[i] = dst[i].max(src[i]);
    }
}

#[inline(always)]
pub(super) fn merge_req_for_after(dst: &mut [u32; 7], after: MTLStages, src: &[u32; 7]) {
    if after.is_empty() {
        return;
    }
    if after == MTLStages::All {
        dst[0] = dst[0].max(src[0]);
        dst[1] = dst[1].max(src[1]);
        dst[2] = dst[2].max(src[2]);
        dst[3] = dst[3].max(src[3]);
        dst[4] = dst[4].max(src[4]);
        dst[5] = dst[5].max(src[5]);
        dst[6] = dst[6].max(src[6]);
        return;
    }
    if after.contains(STAGE_BITS[0]) {
        dst[0] = dst[0].max(src[0]);
    }
    if after.contains(STAGE_BITS[1]) {
        dst[1] = dst[1].max(src[1]);
    }
    if after.contains(STAGE_BITS[2]) {
        dst[2] = dst[2].max(src[2]);
    }
    if after.contains(STAGE_BITS[3]) {
        dst[3] = dst[3].max(src[3]);
    }
    if after.contains(STAGE_BITS[4]) {
        dst[4] = dst[4].max(src[4]);
    }
    if after.contains(STAGE_BITS[5]) {
        dst[5] = dst[5].max(src[5]);
    }
    if after.contains(STAGE_BITS[6]) {
        dst[6] = dst[6].max(src[6]);
    }
}

impl SplitPend {
    #[inline]
    pub(super) fn add(&mut self, after: MTLStages, before: MTLStages, soft: [MTLStages; 2]) {
        if after.is_empty() || (before | soft[0] | soft[1]).is_empty() {
            return;
        }
        if before == MTLStages::All {
            self.vtx_after |= after;
            self.frag_after |= after;
            self.disp_after |= after;
            self.blit_after |= after;
            self.as_after |= after;
            return;
        }
        if before.intersects(PRE_RASTER) {
            self.vtx_after |= after;
        }
        if before.contains(MTLStages::Fragment) {
            self.frag_after |= after;
        }
        if before.contains(MTLStages::Dispatch) {
            self.disp_after |= after;
        }
        if before.contains(MTLStages::Blit) {
            self.blit_after |= after;
        }
        if before.contains(MTLStages::AccelerationStructure) {
            self.as_after |= after;
        }
        if !soft[0].is_empty() {
            self.soft_after[0] |= after;
            self.soft_before[0] |= soft[0];
        }
        if !soft[1].is_empty() {
            self.soft_after[1] |= after;
            self.soft_before[1] |= soft[1];
        }
    }

    #[inline]
    pub(super) fn add_with_req(
        &mut self,
        after: MTLStages,
        before: MTLStages,
        soft: [MTLStages; 2],
        req: [u32; 7],
    ) {
        if after.is_empty() || (before | soft[0] | soft[1]).is_empty() {
            return;
        }
        if before == MTLStages::All {
            self.vtx_after |= after;
            self.frag_after |= after;
            self.disp_after |= after;
            self.blit_after |= after;
            self.as_after |= after;
            merge_req_for_after(&mut self.vtx_req, after, &req);
            merge_req_for_after(&mut self.frag_req, after, &req);
            merge_req_for_after(&mut self.disp_req, after, &req);
            merge_req_for_after(&mut self.blit_req, after, &req);
            merge_req_for_after(&mut self.as_req, after, &req);
            return;
        }
        if before.intersects(PRE_RASTER) {
            self.vtx_after |= after;
            merge_req_for_after(&mut self.vtx_req, after, &req);
        }
        if before.contains(MTLStages::Fragment) {
            self.frag_after |= after;
            merge_req_for_after(&mut self.frag_req, after, &req);
        }
        if before.contains(MTLStages::Dispatch) {
            self.disp_after |= after;
            merge_req_for_after(&mut self.disp_req, after, &req);
        }
        if before.contains(MTLStages::Blit) {
            self.blit_after |= after;
            merge_req_for_after(&mut self.blit_req, after, &req);
        }
        if before.contains(MTLStages::AccelerationStructure) {
            self.as_after |= after;
            merge_req_for_after(&mut self.as_req, after, &req);
        }
        for i in 0..2 {
            if !soft[i].is_empty() {
                self.soft_after[i] |= after;
                self.soft_before[i] |= soft[i];
                merge_req_for_after(&mut self.soft_req[i], after, &req);
            }
        }
    }

    #[inline]
    pub(super) fn stamp_all_req(&mut self, req: [u32; 7]) {
        merge_req_for_after(&mut self.vtx_req, self.vtx_after, &req);
        merge_req_for_after(&mut self.frag_req, self.frag_after, &req);
        merge_req_for_after(&mut self.disp_req, self.disp_after, &req);
        merge_req_for_after(&mut self.blit_req, self.blit_after, &req);
        merge_req_for_after(&mut self.as_req, self.as_after, &req);
        for i in 0..2 {
            if !self.soft_before[i].is_empty() {
                merge_req_for_after(&mut self.soft_req[i], self.soft_after[i], &req);
            }
        }
    }

    #[inline]
    pub(super) fn bump_for_ran(&mut self, ran: MTLStages, cur_gen: &[u32; 7]) {
        if ran.is_empty() {
            return;
        }
        for (pi, &bit) in STAGE_BITS.iter().enumerate() {
            if !ran.contains(bit) {
                continue;
            }
            let g = cur_gen[pi];
            for (a, r) in [
                (self.vtx_after, &mut self.vtx_req),
                (self.frag_after, &mut self.frag_req),
                (self.disp_after, &mut self.disp_req),
                (self.blit_after, &mut self.blit_req),
                (self.as_after, &mut self.as_req),
            ] {
                if a == MTLStages::All || a.contains(bit) {
                    r[pi] = r[pi].max(g);
                }
            }
            for i in 0..2 {
                let a = self.soft_after[i];
                if !self.soft_before[i].is_empty() && (a == MTLStages::All || a.contains(bit)) {
                    self.soft_req[i][pi] = self.soft_req[i][pi].max(g);
                }
            }
        }
    }

    #[inline]
    pub(super) fn merge(&mut self, other: Self) {
        self.vtx_after |= other.vtx_after;
        self.frag_after |= other.frag_after;
        self.disp_after |= other.disp_after;
        self.blit_after |= other.blit_after;
        self.as_after |= other.as_after;
        merge_req_for_after(&mut self.vtx_req, other.vtx_after, &other.vtx_req);
        merge_req_for_after(&mut self.frag_req, other.frag_after, &other.frag_req);
        merge_req_for_after(&mut self.disp_req, other.disp_after, &other.disp_req);
        merge_req_for_after(&mut self.blit_req, other.blit_after, &other.blit_req);
        merge_req_for_after(&mut self.as_req, other.as_after, &other.as_req);
        for i in 0..2 {
            if !other.soft_after[i].is_empty() && !other.soft_before[i].is_empty() {
                self.soft_after[i] |= other.soft_after[i];
                self.soft_before[i] |= other.soft_before[i];
                merge_req_for_after(
                    &mut self.soft_req[i],
                    other.soft_after[i],
                    &other.soft_req[i],
                );
            }
        }
    }

    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        (self.vtx_after
            | self.frag_after
            | self.disp_after
            | self.blit_after
            | self.as_after
            | self.soft_after[0]
            | self.soft_after[1])
            .is_empty()
    }

    #[inline]
    pub(super) fn has_compute(&self) -> bool {
        !(self.disp_after | self.blit_after | self.as_after).is_empty()
    }

    #[inline]
    pub(super) fn has_compute_for(&self, op_stage: MTLStages) -> bool {
        !self.after_for_compute_stage(op_stage).is_empty()
    }

    #[inline]
    pub(super) fn take_compute(&mut self) -> (MTLStages, MTLStages) {
        let (a, b, _) = self.take_compute_for(COMPUTE_STAGES);
        (a, b)
    }

    #[inline]
    pub(super) fn take_compute_for(
        &mut self,
        op_stage: MTLStages,
    ) -> (MTLStages, MTLStages, [u32; 7]) {
        let mut a = MTLStages::empty();
        let mut b = MTLStages::empty();
        let mut req = [0u32; 7];
        if op_stage.contains(MTLStages::Dispatch) && !self.disp_after.is_empty() {
            let da = std::mem::replace(&mut self.disp_after, MTLStages::empty());
            let dr = std::mem::take(&mut self.disp_req);
            a |= da;
            b |= MTLStages::Dispatch;
            merge_req_for_after(&mut req, da, &dr);
        }
        if op_stage.contains(MTLStages::Blit) && !self.blit_after.is_empty() {
            let ba = std::mem::replace(&mut self.blit_after, MTLStages::empty());
            let br = std::mem::take(&mut self.blit_req);
            a |= ba;
            b |= MTLStages::Blit;
            merge_req_for_after(&mut req, ba, &br);
        }
        if op_stage.contains(MTLStages::AccelerationStructure) && !self.as_after.is_empty() {
            let aa = std::mem::replace(&mut self.as_after, MTLStages::empty());
            let ar = std::mem::take(&mut self.as_req);
            a |= aa;
            b |= MTLStages::AccelerationStructure;
            merge_req_for_after(&mut req, aa, &ar);
        }
        (a, b, req)
    }

    #[inline]
    pub(super) fn after_for_compute_stage(&self, op_stage: MTLStages) -> MTLStages {
        let mut a = MTLStages::empty();
        if op_stage.contains(MTLStages::Dispatch) {
            a |= self.disp_after;
        }
        if op_stage.contains(MTLStages::Blit) {
            a |= self.blit_after;
        }
        if op_stage.contains(MTLStages::AccelerationStructure) {
            a |= self.as_after;
        }
        a
    }

    #[inline]
    pub(super) fn to_legacy(&self) -> (MTLStages, MTLStages, [MTLStages; 2]) {
        let mut a = MTLStages::empty();
        let mut b = MTLStages::empty();
        let mut s = [MTLStages::empty(); 2];
        if !self.vtx_after.is_empty() {
            a |= self.vtx_after;
            b |= PRE_RASTER;
        }
        if !self.frag_after.is_empty() {
            a |= self.frag_after;
            b |= MTLStages::Fragment;
        }
        if !self.disp_after.is_empty() {
            a |= self.disp_after;
            b |= MTLStages::Dispatch;
        }
        if !self.blit_after.is_empty() {
            a |= self.blit_after;
            b |= MTLStages::Blit;
        }
        if !self.as_after.is_empty() {
            a |= self.as_after;
            b |= MTLStages::AccelerationStructure;
        }
        for i in 0..2 {
            if !self.soft_after[i].is_empty() && !self.soft_before[i].is_empty() {
                a |= self.soft_after[i];
                s[i] = self.soft_before[i];
            }
        }
        (a, b, s)
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct CsMidResObl {
    pub(super) r: usize,
    pub(super) disp_after: MTLStages,
    pub(super) blit_after: MTLStages,
    pub(super) as_after: MTLStages,
    pub(super) req: [u32; 7],
}

/// Zero-reallocation replacement for `Option<Vec<T>>`, with a 64-bit bloom filter
/// for fast O(1) negative membership checks on `TrackedSet<usize>` resource sets.
#[derive(Clone, Debug)]
pub(super) struct TrackedSet<T> {
    pub(super) exact: bool,
    pub(super) bloom: u64,
    pub(super) items: Vec<T>,
}

impl<T> TrackedSet<T> {
    #[inline]
    pub(super) fn new_exact() -> Self {
        Self { exact: true, bloom: 0, items: Vec::new() }
    }

    #[inline]
    pub(super) fn clear_exact(&mut self) {
        self.exact = true;
        self.bloom = 0;
        self.items.clear();
    }

    #[inline]
    pub(super) fn invalidate(&mut self) {
        self.exact = false;
        self.bloom = 0;
        self.items.clear();
    }

    #[inline]
    pub(super) fn is_exact(&self) -> bool {
        self.exact
    }

    #[inline]
    pub(super) fn as_slice(&self) -> Option<&[T]> {
        self.exact.then_some(self.items.as_slice())
    }

    #[inline]
    pub(super) fn as_mut_vec(&mut self) -> Option<&mut Vec<T>> {
        self.exact.then_some(&mut self.items)
    }

    #[inline]
    pub(super) fn copy_from(&mut self, other: &Self)
    where
        T: Copy,
    {
        self.exact = other.exact;
        self.bloom = other.bloom;
        self.items.clear();
        if other.exact {
            self.items.extend_from_slice(&other.items);
        }
    }
}

impl TrackedSet<usize> {
    #[inline(always)]
    fn res_bloom_bit(r: usize) -> u64 {
        1u64 << (((r as u64 >> 6).wrapping_mul(0x9E37_79B9_7F4A_7C15)) >> 58)
    }

    /// Insert `r` into an exact resource set, skipping the linear scan when its bloom bit is 0.
    #[inline(always)]
    pub(super) fn insert_res(&mut self, r: usize) {
        let bit = Self::res_bloom_bit(r);
        if (self.bloom & bit) == 0 {
            self.bloom |= bit;
            self.items.push(r);
        } else if !self.items.contains(&r) {
            self.items.push(r);
        }
    }

    /// Exact resource hit test (`true` when non-exact / poisoned, `!items.is_empty()` for `r == 0`,
    /// or bloom-filtered exact membership for `r != 0`).
    #[inline(always)]
    pub(super) fn hits_res(&self, r: usize) -> bool {
        if !self.exact {
            return true;
        }
        if r == 0 {
            !self.items.is_empty()
        } else {
            (self.bloom & Self::res_bloom_bit(r)) != 0 && self.items.contains(&r)
        }
    }
}

/// Per-consumer-substage (`0 = Dispatch`, `1 = Blit`, `2 = AccelerationStructure`)
/// producer touch sets in the open compute encoder (`AW2_IB_SUBSTAGE_NARROW=1`).
#[derive(Clone, Debug)]
pub(super) struct IbSubState {
    pub(super) stages: MTLStages,
    pub(super) disp_uav: TrackedSet<usize>,
    pub(super) disp_read: TrackedSet<usize>,
    pub(super) disp_uav_cls: [TrackedSet<usize>; 2],
    pub(super) disp_read_cls: [TrackedSet<usize>; 2],
    pub(super) disp_uav_spans: TrackedSet<SubRange>,
    pub(super) disp_read_spans: TrackedSet<SubRange>,
    pub(super) unbounded_read_classes: u8,
    pub(super) disp_uav_classes: u8,
    pub(super) disp_read_classes: u8,
    pub(super) disp_uav_srv_classes: u8,
    pub(super) disp_uav_hk: u32,
    pub(super) disp_read_hk: u32,
    pub(super) disp_ro_hk: u32,
    pub(super) disp_uav_srv_hk: u32,
    pub(super) blit_touch: TrackedSet<usize>,
    pub(super) as_touch: TrackedSet<usize>,
    pub(super) cs_cleared: Vec<usize>,
}

impl Default for IbSubState {
    fn default() -> Self {
        Self {
            stages: MTLStages::empty(),
            disp_uav: TrackedSet::new_exact(),
            disp_read: TrackedSet::new_exact(),
            disp_uav_cls: [TrackedSet::new_exact(), TrackedSet::new_exact()],
            disp_read_cls: [TrackedSet::new_exact(), TrackedSet::new_exact()],
            disp_uav_spans: TrackedSet::new_exact(),
            disp_read_spans: TrackedSet::new_exact(),
            unbounded_read_classes: 0,
            disp_uav_classes: 0,
            disp_read_classes: 0,
            disp_uav_srv_classes: 0,
            disp_uav_hk: 0,
            disp_read_hk: 0,
            disp_ro_hk: 0,
            disp_uav_srv_hk: 0,
            blit_touch: TrackedSet::new_exact(),
            as_touch: TrackedSet::new_exact(),
            cs_cleared: Vec::new(),
        }
    }
}

impl IbSubState {
    pub(super) fn reset(&mut self) {
        self.stages = MTLStages::empty();
        self.disp_uav.clear_exact();
        self.disp_read.clear_exact();
        self.disp_uav_cls[0].clear_exact();
        self.disp_uav_cls[1].clear_exact();
        self.disp_read_cls[0].clear_exact();
        self.disp_read_cls[1].clear_exact();
        self.disp_uav_spans.clear_exact();
        self.disp_read_spans.clear_exact();
        self.unbounded_read_classes = 0;
        self.disp_uav_classes = 0;
        self.disp_read_classes = 0;
        self.disp_uav_srv_classes = 0;
        self.disp_uav_hk = 0;
        self.disp_read_hk = 0;
        self.disp_ro_hk = 0;
        self.disp_uav_srv_hk = 0;
        self.blit_touch.clear_exact();
        self.as_touch.clear_exact();
        self.cs_cleared.clear();
    }

    pub(super) fn clear_after(&mut self, need_after: MTLStages) {
        self.stages -= need_after;
        if need_after.contains(MTLStages::Dispatch) {
            self.disp_uav.clear_exact();
            self.disp_read.clear_exact();
            self.disp_uav_cls[0].clear_exact();
            self.disp_uav_cls[1].clear_exact();
            self.disp_read_cls[0].clear_exact();
            self.disp_read_cls[1].clear_exact();
            self.disp_uav_spans.clear_exact();
            self.disp_read_spans.clear_exact();
            self.unbounded_read_classes = 0;
            self.disp_uav_classes = 0;
            self.disp_read_classes = 0;
            self.disp_uav_srv_classes = 0;
            self.disp_uav_hk = 0;
            self.disp_read_hk = 0;
            self.disp_ro_hk = 0;
            self.disp_uav_srv_hk = 0;
            self.cs_cleared.clear();
        }
        if need_after.contains(MTLStages::Blit) {
            self.blit_touch.clear_exact();
        }
        if need_after.contains(MTLStages::AccelerationStructure) {
            self.as_touch.clear_exact();
        }
    }
}

pub(super) const CS_SUB_BITS: [(usize, MTLStages); 3] = [
    (0, MTLStages::Dispatch),
    (1, MTLStages::Blit),
    (2, MTLStages::AccelerationStructure),
];

#[inline]
pub(super) fn cs_sub_idx(op_stage: MTLStages) -> Option<usize> {
    if op_stage == MTLStages::Dispatch {
        Some(0)
    } else if op_stage == MTLStages::Blit {
        Some(1)
    } else if op_stage == MTLStages::AccelerationStructure {
        Some(2)
    } else {
        None
    }
}

/// Why the next encoder needs a full (all → all) barrier.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum FullWhy {
    ListStart,
    App,
    ClearUav,
    Internal,
}

/// Per-item deferred obligation from a kept write transition or `Barrier::Uav` inside an open
/// draw pass (`AW2_RP_KEEP_WRITE_EXACT=1`).
#[derive(Clone, Copy)]
pub(super) struct KeptWriteItem {
    pub(super) r: usize,
    pub(super) s0: u32,
    pub(super) eff_s0: u32,
    pub(super) eff_s1: u32,
    pub(super) is_uav_barrier: bool,
    pub(super) split: SplitPend,
}

/// Per-resource soft pre-raster obligation (`AW2_CS_TO_PS_OVERLAP=1`).
#[derive(Clone, Copy, Debug)]
pub(super) struct SoftPreResObl {
    pub(super) r: usize,
    pub(super) after: MTLStages,
    pub(super) before: MTLStages,
    pub(super) req: [u32; 7],
}

/// Per-resource soft graphics (`Fragment | PRE_RASTER`) -> `Dispatch` obligation.
#[derive(Clone, Copy, Debug)]
pub(super) struct CsSoftDispObl {
    pub(super) r: usize,
    pub(super) after: MTLStages,
    pub(super) req: [u32; 7],
}

/// Input-assembler / output-merger / rasterizer state consumed by draws.
#[derive(Clone)]
pub struct GfxState {
    pub pso: Option<Arc<GraphicsPipeline>>,
    /// Mesh-shader pipeline (exclusive with `pso`).
    pub mesh: Option<Arc<crate::mesh::MeshPipeline>>,
    pub rtvs: Vec<TargetView>,
    pub dsv: Option<TargetView>,
    pub open_rtvs: Vec<TargetView>,
    pub open_dsv: Option<TargetView>,
    pub had_target_rebind: bool,
    pub viewports: Vec<MTLViewport>,
    pub scissors: Vec<Rect>,
    /// Per IA slot: (GPU address, size, stride).
    pub vbs: [(u64, u32, u32); MAX_VERTEX_SLOTS as usize],
    /// (GPU address, size, DXGI format).
    pub ib: Option<(u64, u32, u32)>,
    pub topology: u32,
    pub blend_factor: [f32; 4],
    pub stencil_ref: u32,
    pub stencil_back_ref: u32,
    pub depth_bounds: (f32, f32),
    /// The open render encoder was opened by a draw for the current targets.
    pub(super) draw_pass: bool,
    /// A draw in the open draw pass may have written through UAVs.
    pub(super) pass_uav: bool,
    /// A kept `Barrier::Uav` or `Transition(_, _, UAV)` occurred in the open draw pass;
    /// any subsequent UAV-accessing draw must split the pass first.
    pub(super) pass_uav_hazard: bool,
    /// Dynamic state dirty bitmask (`GFX_DIRTY_*`) that must be re-applied to the render encoder.
    pub(super) dirty: u8,
    /// Render target size of the draw pass.
    pub(super) pass_size: (usize, usize),
}

impl Default for GfxState {
    fn default() -> Self {
        GfxState {
            pso: None,
            mesh: None,
            rtvs: Vec::new(),
            dsv: None,
            open_rtvs: Vec::new(),
            open_dsv: None,
            had_target_rebind: false,
            viewports: Vec::new(),
            scissors: Vec::new(),
            vbs: [(0, 0, 0); MAX_VERTEX_SLOTS as usize],
            ib: None,
            topology: 0,
            blend_factor: [1.0; 4],
            stencil_ref: 0,
            stencil_back_ref: 0,
            depth_bounds: (0.0, 1.0),
            draw_pass: false,
            pass_uav: false,
            pass_uav_hazard: false,
            dirty: GFX_DIRTY_ALL,
            pass_size: (0, 0),
        }
    }
}

// ---------------------------------------------------------------------------
// Extracted CommandList Sub-Structs
// ---------------------------------------------------------------------------

/// Cross-encoder / list-level barrier tracking, stage generations, soft pre-raster
/// deferrals, compute mid-queue sync, and reusable barrier scratch vectors.
pub(super) struct BarrierState {
    pub(super) needs_barrier: bool,
    pub(super) full_why: FullWhy,
    pub(super) pend_after: MTLStages,
    pub(super) pend_before: MTLStages,
    pub(super) pend_before_soft: [MTLStages; 2],
    pub(super) split_pend: SplitPend,
    pub(super) soft_pend_items: [Vec<SoftPreResObl>; 2],
    pub(super) soft_pend_poisoned: [bool; 2],
    pub(super) cs_soft_disp_items: Vec<CsSoftDispObl>,
    pub(super) cs_soft_disp_hard_after: MTLStages,
    pub(super) cs_soft_disp_hard_req: [u32; 7],
    pub(super) cs_soft_disp_poisoned: bool,
    pub(super) enc_cs_soft_disp: Vec<CsSoftDispObl>,
    pub(super) cs_soft_disp_scratch: Vec<CsSoftDispObl>,
    pub(super) cs_disp_passthrough: Vec<(usize, MTLStages, [u32; 7])>,
    pub(super) vtx_defer: [Option<(MTLStages, MTLStages)>; 2],
    pub(super) vtx_defer_req: [[u32; 7]; 2],
    pub(super) vtx_defer_items: [Vec<SoftPreResObl>; 2],
    pub(super) vtx_defer_poisoned: [bool; 2],
    pub(super) vtx_carry: [Option<(MTLStages, MTLStages)>; 2],
    pub(super) vtx_carry_req: [[u32; 7]; 2],
    pub(super) vtx_carry_items: [Vec<SoftPreResObl>; 2],
    pub(super) vtx_carry_poisoned: [bool; 2],
    pub(super) enc_sync_after: MTLStages,
    pub(super) enc_sync_before: MTLStages,
    pub(super) pass_targets: Vec<usize>,
    pub(super) written: Vec<usize>,
    pub(super) list_sync_compute: MTLStages,
    pub(super) list_sync_vtx: MTLStages,
    pub(super) list_sync_frag: MTLStages,
    pub(super) has_gpu_work: bool,
    pub(super) opening_compute_stage: MTLStages,
    pub(super) opening_blit_nonfrag_safe: bool,
    pub(super) opening_render_split_ok: bool,
    pub(super) opening_clear_frag_cap: Option<(MTLStages, [u32; 7])>,
    pub(super) clear_rp_unwaited_blit: Vec<usize>,
    pub(super) blit_dst_unwaited_cross: Vec<usize>,
    pub(super) enc_blit_only_sync: bool,
    pub(super) list_cs_unwaited_frag: bool,
    pub(super) list_unwaited_cs: bool,
    pub(super) list_unwaited_gfx: bool,
    pub(super) list_unwaited_vtx_frag: bool,
    pub(super) vtx_defer_list_frag: bool,
    pub(super) vtx_carry_list_frag: bool,
    pub(super) arm_post_draw0_list_frag: [bool; 2],
    pub(super) blit_safe_resources: Vec<usize>,
    pub(super) blit_frag_tainted: Vec<usize>,
    pub(super) uav_gfx_dirty: bool,
    pub(super) stage_sync: [StageSync; 7],
    pub(super) enc_cur_sync: [StageSync; 7],
    pub(super) enc_completed_sync: [StageSync; 7],
    pub(super) stage_gen: [u32; 7],
    pub(super) enc_ran_stages: MTLStages,
    pub(super) enc_had_uav_gfx: bool,
    pub(super) cs_disp_gen: u32,
    pub(super) cs_as_gen: u32,
    pub(super) gfx_uav_gen: u32,
    pub(super) gfx_uav_stage_gen: [u32; 4],
    pub(super) in_list_gfx_non_clear: bool,
    pub(super) in_list_rt_res_ids: Vec<usize>,
    pub(super) clear_rp_res_ids: Vec<usize>,
    pub(super) clear_tex_cs: Vec<usize>,
    pub(super) clear_tex_gfx_before: MTLStages,
    pub(super) clear_tex_gfx_soft: [MTLStages; 2],
    pub(super) clear_tex_gfx_soft_res: Vec<usize>,
    pub(super) clear_tex_gfx_soft_poisoned: bool,
    pub(super) cs_uav_after: MTLStages,
    pub(super) cs_uav_before: MTLStages,
    pub(super) cs_uav_soft: [MTLStages; 2],
    pub(super) cs_uav_soft_items: [Vec<usize>; 2],
    pub(super) cs_uav_soft_poisoned: [bool; 2],
    pub(super) frag_rt_pend: Vec<(usize, MTLStages, [u32; 7])>,
    pub(super) frag_non_rt_after: MTLStages,
    pub(super) frag_non_rt_req: [u32; 7],
    pub(super) frag_rt_poisoned: bool,
    pub(super) uav_passthrough: Vec<(usize, MTLStages)>,
    pub(super) cs_mid_res_pend: Vec<CsMidResObl>,
    pub(super) cs_mid_v3_items: Vec<CsMidResObl>,
    pub(super) cs_mid_v3_poisoned: [bool; 3],
    pub(super) pend_cs_items: Vec<CsMidResObl>,
    pub(super) pend_cs_poisoned: [bool; 3],
    pub(super) cs_end_after: [MTLStages; 3],
    pub(super) cs_end_res: Vec<(usize, MTLStages, MTLStages, MTLStages)>,
    pub(super) bar_scratch_res: Vec<usize>,
    pub(super) bar_scratch_rt_frag: Vec<(usize, MTLStages)>,
    pub(super) bar_scratch_soft: [Vec<(usize, MTLStages, MTLStages)>; 2],
    pub(super) bar_scratch_read_soft: [Vec<(usize, MTLStages, MTLStages)>; 2],
    pub(super) bar_scratch_cs_mid: Vec<(usize, MTLStages, MTLStages, MTLStages)>,
    pub(super) bar_scratch_read_cs_mid: Vec<(usize, MTLStages, MTLStages, MTLStages)>,
    pub(super) bar_scratch_active: Vec<Barrier>,
    pub(super) bar_scratch_pt: Vec<(usize, MTLStages)>,
    pub(super) bar_scratch_n_soft: [Vec<(usize, MTLStages, MTLStages)>; 2],
    pub(super) bar_scratch_n_cs_mid: Vec<(usize, MTLStages, MTLStages, MTLStages)>,
    pub(super) bar_scratch_exact_pt: Vec<(usize, MTLStages)>,
    pub(super) bar_scratch_rp_pt: Vec<(usize, MTLStages)>,
    pub(super) bar_scratch_ib_adds: Vec<(MTLStages, MTLStages, OblKind, Option<SmallSpans>)>,
    pub(super) bar_scratch_ib_uav_adds:
        Vec<(usize, MTLStages, MTLStages, OblKind, Option<SmallSpans>)>,
}

impl BarrierState {
    pub(super) fn new() -> Self {
        Self {
            needs_barrier: true,
            full_why: FullWhy::ListStart,
            pend_after: MTLStages::empty(),
            pend_before: MTLStages::empty(),
            pend_before_soft: [MTLStages::empty(); 2],
            split_pend: SplitPend::default(),
            soft_pend_items: [Vec::new(), Vec::new()],
            soft_pend_poisoned: [false; 2],
            cs_soft_disp_items: Vec::new(),
            cs_soft_disp_hard_after: MTLStages::empty(),
            cs_soft_disp_hard_req: [0; 7],
            cs_soft_disp_poisoned: false,
            enc_cs_soft_disp: Vec::new(),
            cs_soft_disp_scratch: Vec::new(),
            cs_disp_passthrough: Vec::new(),
            vtx_defer: NO_VTX,
            vtx_defer_req: [[0; 7]; 2],
            vtx_defer_items: [Vec::new(), Vec::new()],
            vtx_defer_poisoned: [false; 2],
            vtx_carry: NO_VTX,
            vtx_carry_req: [[0; 7]; 2],
            vtx_carry_items: [Vec::new(), Vec::new()],
            vtx_carry_poisoned: [false; 2],
            enc_sync_after: MTLStages::empty(),
            enc_sync_before: MTLStages::empty(),
            pass_targets: Vec::new(),
            written: Vec::new(),
            list_sync_compute: MTLStages::empty(),
            list_sync_vtx: MTLStages::empty(),
            list_sync_frag: MTLStages::empty(),
            has_gpu_work: false,
            opening_compute_stage: COMPUTE_STAGES,
            opening_blit_nonfrag_safe: false,
            opening_render_split_ok: false,
            opening_clear_frag_cap: None,
            clear_rp_unwaited_blit: Vec::new(),
            blit_dst_unwaited_cross: Vec::new(),
            enc_blit_only_sync: false,
            list_cs_unwaited_frag: false,
            list_unwaited_cs: false,
            list_unwaited_gfx: false,
            list_unwaited_vtx_frag: false,
            vtx_defer_list_frag: false,
            vtx_carry_list_frag: false,
            arm_post_draw0_list_frag: [false; 2],
            blit_safe_resources: Vec::new(),
            blit_frag_tainted: Vec::new(),
            uav_gfx_dirty: false,
            stage_sync: [StageSync::default(); 7],
            enc_cur_sync: [StageSync::default(); 7],
            enc_completed_sync: [StageSync::default(); 7],
            stage_gen: [0; 7],
            enc_ran_stages: MTLStages::empty(),
            enc_had_uav_gfx: false,
            cs_disp_gen: 0,
            cs_as_gen: 0,
            gfx_uav_gen: 0,
            gfx_uav_stage_gen: [0; 4],
            in_list_gfx_non_clear: false,
            in_list_rt_res_ids: Vec::new(),
            clear_rp_res_ids: Vec::new(),
            clear_tex_cs: Vec::new(),
            clear_tex_gfx_before: MTLStages::empty(),
            clear_tex_gfx_soft: [MTLStages::empty(); 2],
            clear_tex_gfx_soft_res: Vec::new(),
            clear_tex_gfx_soft_poisoned: false,
            cs_uav_after: MTLStages::empty(),
            cs_uav_before: MTLStages::empty(),
            cs_uav_soft: [MTLStages::empty(); 2],
            cs_uav_soft_items: [Vec::new(), Vec::new()],
            cs_uav_soft_poisoned: [false; 2],
            frag_rt_pend: Vec::new(),
            frag_non_rt_after: MTLStages::empty(),
            frag_non_rt_req: [0; 7],
            frag_rt_poisoned: false,
            uav_passthrough: Vec::new(),
            cs_mid_res_pend: Vec::new(),
            cs_mid_v3_items: Vec::new(),
            cs_mid_v3_poisoned: [false; 3],
            pend_cs_items: Vec::new(),
            pend_cs_poisoned: [false; 3],
            cs_end_after: [MTLStages::empty(); 3],
            cs_end_res: Vec::new(),
            bar_scratch_res: Vec::new(),
            bar_scratch_rt_frag: Vec::new(),
            bar_scratch_soft: [Vec::new(), Vec::new()],
            bar_scratch_read_soft: [Vec::new(), Vec::new()],
            bar_scratch_cs_mid: Vec::new(),
            bar_scratch_read_cs_mid: Vec::new(),
            bar_scratch_active: Vec::new(),
            bar_scratch_pt: Vec::new(),
            bar_scratch_n_soft: [Vec::new(), Vec::new()],
            bar_scratch_n_cs_mid: Vec::new(),
            bar_scratch_exact_pt: Vec::new(),
            bar_scratch_rp_pt: Vec::new(),
            bar_scratch_ib_adds: Vec::new(),
            bar_scratch_ib_uav_adds: Vec::new(),
        }
    }

    pub(super) fn reset_for_begin(&mut self, list_barrier_tight: bool) {
        self.has_gpu_work = false;
        self.opening_compute_stage = COMPUTE_STAGES;
        self.opening_blit_nonfrag_safe = false;
        self.opening_render_split_ok = false;
        self.enc_blit_only_sync = false;
        self.list_cs_unwaited_frag = false;
        self.list_unwaited_cs = false;
        self.list_unwaited_gfx = false;
        self.list_unwaited_vtx_frag = false;
        self.vtx_defer_list_frag = false;
        self.vtx_carry_list_frag = false;
        self.arm_post_draw0_list_frag = [false; 2];
        self.blit_safe_resources.clear();
        self.blit_frag_tainted.clear();
        self.uav_gfx_dirty = false;
        self.stage_sync = [StageSync::default(); 7];
        self.stage_gen = [0; 7];
        self.enc_cur_sync = [StageSync::default(); 7];
        self.enc_completed_sync = [StageSync::default(); 7];
        self.enc_ran_stages = MTLStages::empty();
        self.enc_had_uav_gfx = false;
        self.cs_disp_gen = 0;
        self.cs_as_gen = 0;
        self.gfx_uav_gen = 0;
        self.gfx_uav_stage_gen = [0; 4];
        self.in_list_gfx_non_clear = false;
        self.in_list_rt_res_ids.clear();
        self.clear_rp_res_ids.clear();
        self.clear_tex_cs.clear();
        self.clear_tex_gfx_before = MTLStages::empty();
        self.clear_tex_gfx_soft = [MTLStages::empty(); 2];
        self.clear_tex_gfx_soft_res.clear();
        self.clear_tex_gfx_soft_poisoned = false;
        self.cs_uav_after = MTLStages::empty();
        self.cs_uav_before = MTLStages::empty();
        self.cs_uav_soft = [MTLStages::empty(); 2];
        self.cs_uav_soft_items[0].clear();
        self.cs_uav_soft_items[1].clear();
        self.cs_uav_soft_poisoned = [false; 2];
        self.cs_end_after = [MTLStages::empty(); 3];
        self.cs_end_res.clear();
        self.uav_passthrough.clear();
        self.cs_disp_passthrough.clear();
        self.enc_cs_soft_disp.clear();
        self.cs_soft_disp_items.clear();
        self.cs_soft_disp_hard_after = MTLStages::empty();
        self.cs_soft_disp_hard_req = [0; 7];
        self.cs_soft_disp_poisoned = false;
        self.needs_barrier = false;
        if list_barrier_tight {
            self.list_sync_compute = COMPUTE_STAGES | PRE_RASTER;
            self.list_sync_vtx = COMPUTE_STAGES | PRE_RASTER;
            self.list_sync_frag = MTLStages::All;
        } else {
            self.full_why = FullWhy::ListStart;
            self.needs_barrier = true;
            self.list_sync_compute = MTLStages::empty();
            self.list_sync_vtx = MTLStages::empty();
            self.list_sync_frag = MTLStages::empty();
        }
        self.pend_after = MTLStages::empty();
        self.pend_before = MTLStages::empty();
        self.pend_before_soft = [MTLStages::empty(); 2];
        self.split_pend = SplitPend::default();
        self.soft_pend_items[0].clear();
        self.soft_pend_items[1].clear();
        self.soft_pend_poisoned = [false; 2];
        self.cs_mid_res_pend.clear();
        self.cs_mid_v3_items.clear();
        self.cs_mid_v3_poisoned = [false; 3];
        self.pend_cs_items.clear();
        self.pend_cs_poisoned = [false; 3];
        self.frag_rt_pend.clear();
        self.frag_non_rt_after = MTLStages::empty();
        self.frag_non_rt_req = [0; 7];
        self.frag_rt_poisoned = false;
        self.vtx_defer = NO_VTX;
        self.vtx_defer_req = [[0; 7]; 2];
        self.vtx_carry = NO_VTX;
        self.vtx_carry_req = [[0; 7]; 2];
        self.vtx_defer_items[0].clear();
        self.vtx_defer_items[1].clear();
        self.vtx_defer_poisoned = [false; 2];
        self.vtx_carry_items[0].clear();
        self.vtx_carry_items[1].clear();
        self.vtx_carry_poisoned = [false; 2];
        self.opening_clear_frag_cap = None;
        self.clear_rp_unwaited_blit.clear();
        self.blit_dst_unwaited_cross.clear();
        self.enc_sync_after = MTLStages::empty();
        self.enc_sync_before = MTLStages::empty();
        self.pass_targets.clear();
        self.written.clear();
    }
}

/// Intra-compute-encoder touch tracking, obligations, substage buckets, and
/// reusable dispatch scratch vectors.
pub(super) struct IntraComputeState {
    pub(super) ib_touch: TrackedSet<usize>,
    pub(super) ib_obl: Vec<usize>,
    pub(super) ib_obl_all: bool,
    pub(super) ib_cs_clear: bool,
    pub(super) ib_stages: MTLStages,
    pub(super) ib_disp_uav: TrackedSet<usize>,
    pub(super) ib_disp_read: TrackedSet<usize>,
    pub(super) ib_disp_uav_cls: [TrackedSet<usize>; 2],
    pub(super) ib_disp_read_cls: [TrackedSet<usize>; 2],
    pub(super) ib_disp_uav_spans: TrackedSet<SubRange>,
    pub(super) ib_disp_read_spans: TrackedSet<SubRange>,
    pub(super) ib_unbounded_read_classes: u8,
    pub(super) ib_disp_uav_classes: u8,
    pub(super) ib_disp_read_classes: u8,
    pub(super) ib_disp_uav_srv_classes: u8,
    pub(super) ib_disp_uav_hk: u32,
    pub(super) ib_disp_read_hk: u32,
    pub(super) ib_disp_ro_hk: u32,
    pub(super) ib_disp_uav_srv_hk: u32,
    pub(super) ib_blit_touch: TrackedSet<usize>,
    pub(super) ib_blit_dst: Vec<usize>,
    pub(super) ib_as_touch: TrackedSet<usize>,
    pub(super) ib_cs_cleared: Vec<usize>,
    pub(super) ib_sub: [IbSubState; 3],
    pub(super) ib_obl_tight: Vec<IbObl>,
    pub(super) ib_scratch_uav: Vec<usize>,
    pub(super) ib_scratch_read: Vec<usize>,
    pub(super) ib_scratch_op: Vec<usize>,
    pub(super) ib_scratch_uav_spans: Vec<SubRange>,
    pub(super) ib_scratch_read_spans: Vec<SubRange>,
}

impl IntraComputeState {
    pub(super) fn new() -> Self {
        Self {
            ib_touch: TrackedSet::new_exact(),
            ib_obl: Vec::new(),
            ib_obl_all: false,
            ib_cs_clear: false,
            ib_stages: MTLStages::empty(),
            ib_disp_uav: TrackedSet::new_exact(),
            ib_disp_read: TrackedSet::new_exact(),
            ib_disp_uav_cls: [TrackedSet::new_exact(), TrackedSet::new_exact()],
            ib_disp_read_cls: [TrackedSet::new_exact(), TrackedSet::new_exact()],
            ib_disp_uav_spans: TrackedSet::new_exact(),
            ib_disp_read_spans: TrackedSet::new_exact(),
            ib_unbounded_read_classes: 0,
            ib_disp_uav_classes: 0,
            ib_disp_read_classes: 0,
            ib_disp_uav_srv_classes: 0,
            ib_disp_uav_hk: 0,
            ib_disp_read_hk: 0,
            ib_disp_ro_hk: 0,
            ib_disp_uav_srv_hk: 0,
            ib_blit_touch: TrackedSet::new_exact(),
            ib_blit_dst: Vec::new(),
            ib_as_touch: TrackedSet::new_exact(),
            ib_cs_cleared: Vec::new(),
            ib_sub: std::array::from_fn(|_| IbSubState::default()),
            ib_obl_tight: Vec::new(),
            ib_scratch_uav: Vec::new(),
            ib_scratch_read: Vec::new(),
            ib_scratch_op: Vec::new(),
            ib_scratch_uav_spans: Vec::new(),
            ib_scratch_read_spans: Vec::new(),
        }
    }
}

/// Open render-pass kept-write tracking, per-stage exact bindings, UAV hazard sets,
/// render-encoder state cache, and `RuntimeDrawParams` upload dedup cache.
pub(super) struct RenderPassSyncState {
    pub(super) rp_write_after: MTLStages,
    pub(super) rp_write_before: MTLStages,
    pub(super) rp_write_soft: [MTLStages; 2],
    pub(super) rp_write_split: SplitPend,
    pub(super) rp_kept_writes: Vec<KeptWriteItem>,
    pub(super) rp_kept_later_draw: bool,
    pub(super) enc_sync_targets: Vec<usize>,
    pub(super) enc_pre_raster_reads: u8,
    pub(super) enc_pre_raster_srv_uav_reads: u8,
    pub(super) enc_pre_raster_unbounded_classes: u8,
    pub(super) enc_pre_raster_exact_res: Vec<usize>,
    pub(super) enc_pre_raster_hk: u32,
    pub(super) enc_pre_raster_unb_hk: u32,
    pub(super) enc_ps_reads: u8,
    pub(super) enc_ps_srv_uav_reads: u8,
    pub(super) enc_ps_unbounded_classes: u8,
    pub(super) enc_ps_exact_res: Vec<usize>,
    pub(super) enc_ps_hk: u32,
    pub(super) enc_ps_unb_hk: u32,
    pub(super) enc_deferred_frag_ps: bool,
    pub(super) gfx_ps_overlap_chain: u8,
    pub(super) cs_hk_profile: HandleKindProfile,
    pub(super) cur_draw_exact_valid: bool,
    pub(super) cur_draw_pre_unb: u8,
    pub(super) cur_draw_pre_unb_hk: u32,
    pub(super) cur_draw_pre_exact: Vec<usize>,
    pub(super) cur_draw_ps_unb: u8,
    pub(super) cur_draw_ps_unb_hk: u32,
    pub(super) cur_draw_ps_exact: Vec<usize>,
    pub(super) enc_uav_used: bool,
    pub(super) enc_uav_write_stages: MTLStages,
    pub(super) enc_uav_write_unbounded: bool,
    pub(super) enc_uav_write_exact_res: Vec<usize>,
    pub(super) pass_uav_hazard_all: bool,
    pub(super) pass_uav_hazard_res: Vec<usize>,
    pub(super) enc_render_cache: EncRenderCache,
    pub(super) last_draw_params: [u8; 20],
    pub(super) last_draw_params_len: u8,
    pub(super) last_draw_params_va: u64,
}

impl RenderPassSyncState {
    pub(super) fn new() -> Self {
        Self {
            rp_write_after: MTLStages::empty(),
            rp_write_before: MTLStages::empty(),
            rp_write_soft: [MTLStages::empty(); 2],
            rp_write_split: SplitPend::default(),
            rp_kept_writes: Vec::new(),
            rp_kept_later_draw: false,
            enc_sync_targets: Vec::new(),
            enc_pre_raster_reads: 0,
            enc_pre_raster_srv_uav_reads: 0,
            enc_pre_raster_unbounded_classes: 0,
            enc_pre_raster_exact_res: Vec::new(),
            enc_pre_raster_hk: 0,
            enc_pre_raster_unb_hk: 0,
            enc_ps_reads: 0,
            enc_ps_srv_uav_reads: 0,
            enc_ps_unbounded_classes: 0,
            enc_ps_exact_res: Vec::new(),
            enc_ps_hk: 0,
            enc_ps_unb_hk: 0,
            enc_deferred_frag_ps: false,
            gfx_ps_overlap_chain: 0,
            cs_hk_profile: HandleKindProfile::OPAQUE,
            cur_draw_exact_valid: false,
            cur_draw_pre_unb: 0,
            cur_draw_pre_unb_hk: 0,
            cur_draw_pre_exact: Vec::new(),
            cur_draw_ps_unb: 0,
            cur_draw_ps_unb_hk: 0,
            cur_draw_ps_exact: Vec::new(),
            enc_uav_used: false,
            enc_uav_write_stages: MTLStages::empty(),
            enc_uav_write_unbounded: false,
            enc_uav_write_exact_res: Vec::new(),
            pass_uav_hazard_all: false,
            pass_uav_hazard_res: Vec::new(),
            enc_render_cache: EncRenderCache::default(),
            last_draw_params: [0u8; 20],
            last_draw_params_len: 0,
            last_draw_params_va: 0,
        }
    }

    pub(super) fn reset_for_begin(&mut self) {
        self.enc_render_cache.reset();
        self.last_draw_params_len = 0;
        self.last_draw_params_va = 0;
        self.rp_write_after = MTLStages::empty();
        self.rp_write_before = MTLStages::empty();
        self.rp_write_soft = [MTLStages::empty(); 2];
        self.rp_write_split = SplitPend::default();
        self.rp_kept_writes.clear();
        self.rp_kept_later_draw = false;
        self.reset_for_render_pass(&[], 0);
        self.gfx_ps_overlap_chain = 0;
        self.cs_hk_profile = HandleKindProfile::OPAQUE;
        self.cur_draw_exact_valid = false;
        self.cur_draw_pre_unb = 0;
        self.cur_draw_pre_unb_hk = 0;
        self.cur_draw_pre_exact.clear();
        self.cur_draw_ps_unb = 0;
        self.cur_draw_ps_unb_hk = 0;
        self.cur_draw_ps_exact.clear();
    }

    /// Reset per-render-encoder tracking fields when opening a new draw or utility render pass.
    pub(super) fn reset_for_render_pass(
        &mut self,
        sync_targets: &[usize],
        opening_pre_raster_reads: u8,
    ) {
        self.enc_sync_targets.clear();
        self.enc_sync_targets.extend_from_slice(sync_targets);
        self.enc_pre_raster_reads = opening_pre_raster_reads;
        self.enc_pre_raster_srv_uav_reads = 0;
        self.enc_pre_raster_unbounded_classes = 0;
        self.enc_pre_raster_exact_res.clear();
        self.enc_ps_reads = 0;
        self.enc_ps_srv_uav_reads = 0;
        self.enc_ps_unbounded_classes = 0;
        self.enc_ps_exact_res.clear();
        self.enc_pre_raster_hk = 0;
        self.enc_pre_raster_unb_hk = 0;
        self.enc_ps_hk = 0;
        self.enc_ps_unb_hk = 0;
        self.enc_deferred_frag_ps = false;
        self.enc_uav_used = false;
        self.enc_uav_write_stages = MTLStages::empty();
        self.enc_uav_write_unbounded = false;
        self.enc_uav_write_exact_res.clear();
        self.pass_uav_hazard_all = false;
        self.pass_uav_hazard_res.clear();
    }
}

/// Deferred whole-subresource clears and `DiscardResource` marks.
#[derive(Default)]
pub(super) struct ClearState {
    pub(super) pending_clears: Vec<PendingClear>,
    pub(super) discards: Vec<usize>,
}

/// `ExecuteIndirect` ICB batching, ring handles, and pass-split counters.
#[derive(Default)]
pub(super) struct IcbBatchState {
    pub(super) icb_scratch: Option<Arc<Mutex<UploadRing>>>,
    pub(super) icbs: Option<Arc<Mutex<crate::metal::indirect_icb::IcbRing>>>,
    pub(super) icb_segs: Vec<DrawSeg>,
    pub(super) enc_used_icb: bool,
    pub(super) enc_icb_war_done: bool,
    pub(super) enc_icb_cmds: u32,
    pub(super) icb_dyn: Vec<DynSnapshot>,
    pub(super) icb_qbarriers: Vec<(MTLStages, MTLStages)>,
    pub(super) icb_prepass_pending: bool,
    pub(super) icb_prepass_flushing: bool,
}

impl IcbBatchState {
    pub(super) fn reset_for_begin(
        &mut self,
        icb_scratch: Arc<Mutex<UploadRing>>,
        icbs: Arc<Mutex<crate::metal::indirect_icb::IcbRing>>,
    ) {
        self.icb_scratch = Some(icb_scratch);
        self.icbs = Some(icbs);
        self.icb_segs.clear();
        self.enc_used_icb = false;
        self.enc_icb_war_done = false;
        self.enc_icb_cmds = 0;
        self.icb_dyn.clear();
        self.icb_qbarriers.clear();
        self.icb_prepass_pending = false;
        self.icb_prepass_flushing = false;
    }
}

/// One compiled slot-extraction operation inside a [`SlotPlan`].
#[derive(Clone, Copy, Debug)]
pub(super) enum SlotOp {
    /// Root CBV/SRV/UAV virtual address from root parameter `param` (1 u64 token).
    RootVa { param: u16 },
    /// Coalesced run of consecutive descriptor-table entries sharing `param`, with
    /// precomputed `[min_off, max_end_off)` bounds so heap0 range checks and root
    /// parameter reads happen ONCE per table run instead of once per descriptor.
    TableRun {
        param: u16,
        min_off: u64,
        max_end_off: u64,
        off_start: u32,
        off_len: u32,
    },
}

/// Compiled root-signature + PSO slot-lookup plan eliminating repeated root-parameter
/// lookups, per-slot heap bounds checks, and double descriptor reads on bind-cache misses.
#[derive(Clone, Default)]
pub(super) struct SlotPlan {
    pub(super) valid: bool,
    pub(super) total_tokens: usize,
    pub(super) ops: Vec<SlotOp>,
    pub(super) offsets: Vec<u64>,
}

impl SlotPlan {
    pub(super) fn compile_into(&mut self, slots: &[ComputeBind], root: &RootState) {
        self.ops.clear();
        self.offsets.clear();
        self.total_tokens = 0;
        self.valid = true;
        for &slot in slots {
            match slot {
                ComputeBind::RootVa { param } => {
                    if root.param_u64(param as usize).is_none() {
                        self.valid = false;
                        return;
                    }
                    self.ops.push(SlotOp::RootVa { param });
                    self.total_tokens += 1;
                }
                ComputeBind::TableEntry { param, byte_offset } => {
                    if root.param_u64(param as usize).is_none() {
                        self.valid = false;
                        return;
                    }
                    let Some(end_off) = byte_offset.checked_add(24) else {
                        self.valid = false;
                        return;
                    };
                    if let Some(SlotOp::TableRun {
                        param: prev_p,
                        min_off,
                        max_end_off,
                        off_len,
                        ..
                    }) = self.ops.last_mut()
                    {
                        if *prev_p == param {
                            *min_off = (*min_off).min(byte_offset);
                            *max_end_off = (*max_end_off).max(end_off);
                            *off_len += 1;
                            self.offsets.push(byte_offset);
                            self.total_tokens += 3;
                            continue;
                        }
                    }
                    let off_start = self.offsets.len() as u32;
                    self.offsets.push(byte_offset);
                    self.ops.push(SlotOp::TableRun {
                        param,
                        min_off: byte_offset,
                        max_end_off: end_off,
                        off_start,
                        off_len: 1,
                    });
                    self.total_tokens += 3;
                }
            }
        }
    }

    /// Extract descriptor-table / root-VA tokens into `tokens`, preserving exact
    /// token order and failure semantics of `extract_compute_slot_tokens`.
    #[inline]
    pub(super) fn extract_tokens(
        &self,
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        tokens: &mut Vec<u64>,
    ) -> bool {
        if !self.valid {
            return false;
        }
        tokens.reserve(self.total_tokens);
        for &op in &self.ops {
            match op {
                SlotOp::RootVa { param } => {
                    let Some(va) = root.param_u64(param as usize) else {
                        return false;
                    };
                    tokens.push(va);
                }
                SlotOp::TableRun {
                    param,
                    min_off,
                    max_end_off,
                    off_start,
                    off_len,
                } => {
                    let Some(base_gpu) = root.param_u64(param as usize) else {
                        return false;
                    };
                    let n = off_len as usize;
                    if base_gpu == 0 {
                        tokens.resize(tokens.len() + n * 3, 0);
                        continue;
                    }
                    let Some((hbase, hlen, hcpu)) = heap0 else {
                        return false;
                    };
                    let h_end = hbase.saturating_add(hlen);
                    let Some(lo_gpu) = base_gpu.checked_add(min_off) else {
                        return false;
                    };
                    let Some(hi_gpu) = base_gpu.checked_add(max_end_off) else {
                        return false;
                    };
                    if lo_gpu < hbase || hi_gpu > h_end {
                        return false;
                    }
                    let base_lo_cpu = hcpu + (lo_gpu - hbase) as usize;
                    let offs = &self.offsets[off_start as usize..off_start as usize + n];
                    // SAFETY: `tokens.reserve(self.total_tokens)` reserved enough space for
                    // every op in `self.ops`, and `lo_gpu >= hbase && hi_gpu <= h_end` ensures
                    // every `[base_gpu + byte_offset, +24)` lies inside `[hbase, h_end)`.
                    unsafe {
                        let cur_len = tokens.len();
                        let mut dst = tokens.as_mut_ptr().add(cur_len);
                        for &byte_offset in offs {
                            let p = (base_lo_cpu + (byte_offset - min_off) as usize) as *const u64;
                            let gpu_va = p.read_unaligned();
                            let tex_id = p.add(1).read_unaligned();
                            let meta = p.add(2).read_unaligned() & 0xffff_ffff;
                            dst.write(gpu_va);
                            dst.add(1).write(tex_id);
                            dst.add(2).write(meta);
                            dst = dst.add(3);
                        }
                        tokens.set_len(cur_len + n * 3);
                    }
                }
            }
        }
        true
    }

    /// Resolve resource keys (and optional buffer sub-ranges) directly from the exact
    /// token slice previously produced by [`Self::extract_tokens`], avoiding re-reading
    /// `RootState` and descriptor-heap memory on bind-cache misses.
    #[inline]
    pub(super) fn resolve_from_tokens(
        &self,
        reg: &ResRegistry,
        va_cache: &mut BufVaCache,
        toks: &[u64],
        out: &mut Vec<usize>,
        mut spans: Option<&mut Vec<SubRange>>,
    ) -> bool {
        out.clear();
        if let Some(sp) = spans.as_deref_mut() {
            sp.clear();
        }
        if !self.valid || toks.len() != self.total_tokens {
            return false;
        }
        let mut pos = 0usize;
        for &op in &self.ops {
            match op {
                SlotOp::RootVa { .. } => {
                    let va = toks[pos];
                    pos += 1;
                    if va != 0 {
                        let ok = match spans.as_deref_mut() {
                            Some(sp) => reg.push_buffer_span_cached(va_cache, va, 0, out, sp),
                            None => reg.push_buffer_va_cached(va_cache, va, out),
                        };
                        if !ok {
                            return false;
                        }
                    }
                }
                SlotOp::TableRun { off_len, .. } => {
                    let end = pos + (off_len as usize) * 3;
                    while pos < end {
                        let gpu_va = toks[pos];
                        let tex_id = toks[pos + 1];
                        let meta_lo = toks[pos + 2];
                        pos += 3;
                        if gpu_va != 0 {
                            let found = match spans.as_deref_mut() {
                                Some(sp) => {
                                    reg.push_buffer_span_cached(va_cache, gpu_va, meta_lo, out, sp)
                                }
                                None => reg.push_buffer_va_cached(va_cache, gpu_va, out),
                            };
                            if !found {
                                return false;
                            }
                        } else if tex_id != 0 {
                            let Some(rk) = reg.lookup_texture_id(tex_id) else {
                                return false;
                            };
                            if out.last() != Some(&rk) && !out.contains(&rk) {
                                out.push(rk);
                            }
                            if let Some(sp) = spans.as_deref_mut() {
                                push_subrange(sp, SubRange::whole(rk));
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct SlotPlanKey {
    epoch: u64,
    pso_ptr: usize,
    rs_hash: u128,
    slots_ptr: usize,
    slots_len: usize,
    first_slot: Option<ComputeBind>,
    last_slot: Option<ComputeBind>,
}

#[derive(Default)]
struct SlotPlanEntry {
    key: SlotPlanKey,
    plan: SlotPlan,
}

const SLOT_PLAN_CACHE_SIZE: usize = 128;

/// Per-command-list direct-mapped cache of compiled [`SlotPlan`]s keyed by
/// `(epoch, pso_ptr, rs_hash, slots_ptr, slots_len, first_slot, last_slot)`.
/// Low 2 bits of index are reserved for `stage_tag` (`0..4`) so a draw/dispatch's
/// UAV and read plans never evict one another.
pub(super) struct SlotPlanCache {
    epoch: u64,
    entries: Box<[SlotPlanEntry; SLOT_PLAN_CACHE_SIZE]>,
}

impl SlotPlanCache {
    pub(super) fn new() -> Self {
        let v: Vec<SlotPlanEntry> = (0..SLOT_PLAN_CACHE_SIZE)
            .map(|_| SlotPlanEntry::default())
            .collect();
        let entries: Box<[SlotPlanEntry; SLOT_PLAN_CACHE_SIZE]> = v
            .into_boxed_slice()
            .try_into()
            .unwrap_or_else(|_| unreachable!());
        Self { epoch: 1, entries }
    }

    #[inline]
    pub(super) fn invalidate(&mut self) {
        self.epoch = self.epoch.wrapping_add(1).max(1);
    }

    #[inline]
    pub(super) fn get_or_compile(
        &mut self,
        pso_ptr: usize,
        stage_tag: usize,
        slots: &[ComputeBind],
        root: &RootState,
    ) -> usize {
        let key = SlotPlanKey {
            epoch: self.epoch,
            pso_ptr,
            rs_hash: root.rs_hash,
            slots_ptr: slots.as_ptr() as usize,
            slots_len: slots.len(),
            first_slot: slots.first().copied(),
            last_slot: slots.last().copied(),
        };
        let h = ((pso_ptr as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ (key.rs_hash as u64)
            ^ ((key.slots_ptr as u64) >> 4)) as usize;
        let idx = ((h << 2) | (stage_tag & 3)) & (SLOT_PLAN_CACHE_SIZE - 1);
        let entry = &mut self.entries[idx];
        if entry.key != key {
            entry.plan.compile_into(slots, root);
            entry.key = key;
        }
        idx
    }

    #[inline(always)]
    pub(super) fn plan(&self, idx: usize) -> &SlotPlan {
        &self.entries[idx].plan
    }
}

/// Dirty-tracked `MTL4ArgumentTable` addresses/VBs plus token-validated compute & graphics bind caches.
pub(super) struct BindTableCache {
    pub(super) bound_table_addrs: [u64; 7],
    pub(super) bound_vbs: [(u64, usize); MAX_VERTEX_SLOTS as usize],
    pub(super) cs_bind_cache: CsBindCache,
    pub(super) cs_bind_tokens: Vec<u64>,
    pub(super) gfx_bind_cache: GfxBindCache,
    pub(super) gfx_bind_tokens: Vec<u64>,
    /// Per-command-list buffer VA -> resource key cache (replaces TLS `BUF_VA_CACHE`).
    pub(super) buf_va_cache: Box<BufVaCache>,
    /// Per-(root signature, PSO) compiled slot-lookup plan cache.
    pub(super) slot_plans: SlotPlanCache,
}

impl BindTableCache {
    pub(super) fn new() -> Self {
        Self {
            bound_table_addrs: [u64::MAX; 7],
            bound_vbs: [(u64::MAX, usize::MAX); MAX_VERTEX_SLOTS as usize],
            cs_bind_cache: CsBindCache::default(),
            cs_bind_tokens: Vec::new(),
            gfx_bind_cache: GfxBindCache::default(),
            gfx_bind_tokens: Vec::new(),
            buf_va_cache: BufVaCache::new_boxed(),
            slot_plans: SlotPlanCache::new(),
        }
    }
}
