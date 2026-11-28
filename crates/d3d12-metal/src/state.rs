//! Fixed-function PSO state: D3D12 blend / rasterizer / depth-stencil descriptions (exact
//! Windows layouts) → Metal values.
//!
//! * [`BlendDesc`] → [`BlendState`]. Per-attachment values go on the
//!   `MTL4RenderPipelineColorAttachmentDescriptor` via [`ColorBlend::apply_to`]; alpha-to-coverage
//!   goes on the pipeline descriptor.
//! * [`RasterizerDesc`] / `1` / `2` → [`RasterState`]. In Metal 4 these are encoder state, so the
//!   struct is replayed at draw time ([`RasterState::apply`]).
//! * [`DepthStencilDesc`] / `1` / `2` → [`DepthStencilState`], which configures an
//!   `MTLDepthStencilDescriptor` ([`DepthStencilState::apply_to`]). It is hashable, so it can key a
//!   state-object cache. Depth bounds are encoder state in Metal 4 (`setDepthTestMinBound`).
//!
//! Values the hardware can't express are reported in `unsupported` fields rather than silently
//! dropped. Owned by the shaders engineer.

#![allow(non_snake_case)]

use objc2_metal::{
    MTL4BlendState, MTL4RenderCommandEncoder, MTL4RenderPipelineColorAttachmentDescriptor, MTLBlendFactor, MTLBlendOperation,
    MTLColorWriteMask, MTLCompareFunction, MTLCullMode, MTLDepthClipMode, MTLDepthStencilDescriptor, MTLStencilDescriptor,
    MTLStencilOperation, MTLTriangleFillMode, MTLWinding,
};

pub type BOOL = i32;

// ---- D3D12 layouts ------------------------------------------------------------------------------

/// `D3D12_RENDER_TARGET_BLEND_DESC` (40 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderTargetBlendDesc {
    pub BlendEnable: BOOL,
    pub LogicOpEnable: BOOL,
    pub SrcBlend: u32,
    pub DestBlend: u32,
    pub BlendOp: u32,
    pub SrcBlendAlpha: u32,
    pub DestBlendAlpha: u32,
    pub BlendOpAlpha: u32,
    pub LogicOp: u32,
    pub RenderTargetWriteMask: u8,
}
const _: () = assert!(std::mem::size_of::<RenderTargetBlendDesc>() == 40);

/// `D3D12_BLEND_DESC` (328 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlendDesc {
    pub AlphaToCoverageEnable: BOOL,
    pub IndependentBlendEnable: BOOL,
    pub RenderTarget: [RenderTargetBlendDesc; 8],
}
const _: () = assert!(std::mem::size_of::<BlendDesc>() == 328);

/// `D3D12_RASTERIZER_DESC` (44 bytes; integer `DepthBias`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RasterizerDesc {
    pub FillMode: u32,
    pub CullMode: u32,
    pub FrontCounterClockwise: BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: BOOL,
    pub MultisampleEnable: BOOL,
    pub AntialiasedLineEnable: BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: u32,
}
const _: () = assert!(std::mem::size_of::<RasterizerDesc>() == 44);

/// `D3D12_RASTERIZER_DESC1` (44 bytes; float `DepthBias`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RasterizerDesc1 {
    pub FillMode: u32,
    pub CullMode: u32,
    pub FrontCounterClockwise: BOOL,
    pub DepthBias: f32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: BOOL,
    pub MultisampleEnable: BOOL,
    pub AntialiasedLineEnable: BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: u32,
}
const _: () = assert!(std::mem::size_of::<RasterizerDesc1>() == 44);

/// `D3D12_RASTERIZER_DESC2` (40 bytes; `LineRasterizationMode` replaces the two line BOOLs).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RasterizerDesc2 {
    pub FillMode: u32,
    pub CullMode: u32,
    pub FrontCounterClockwise: BOOL,
    pub DepthBias: f32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: BOOL,
    pub LineRasterizationMode: u32,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: u32,
}
const _: () = assert!(std::mem::size_of::<RasterizerDesc2>() == 40);

/// `D3D12_DEPTH_STENCILOP_DESC` (16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthStencilOpDesc {
    pub StencilFailOp: u32,
    pub StencilDepthFailOp: u32,
    pub StencilPassOp: u32,
    pub StencilFunc: u32,
}
const _: () = assert!(std::mem::size_of::<DepthStencilOpDesc>() == 16);

/// `D3D12_DEPTH_STENCILOP_DESC1` (20 bytes; per-face masks).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthStencilOpDesc1 {
    pub StencilFailOp: u32,
    pub StencilDepthFailOp: u32,
    pub StencilPassOp: u32,
    pub StencilFunc: u32,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
}
const _: () = assert!(std::mem::size_of::<DepthStencilOpDesc1>() == 20);

/// `D3D12_DEPTH_STENCIL_DESC` (52 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthStencilDesc {
    pub DepthEnable: BOOL,
    pub DepthWriteMask: u32,
    pub DepthFunc: u32,
    pub StencilEnable: BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: DepthStencilOpDesc,
    pub BackFace: DepthStencilOpDesc,
}
const _: () = assert!(std::mem::size_of::<DepthStencilDesc>() == 52);

/// `D3D12_DEPTH_STENCIL_DESC1` (56 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthStencilDesc1 {
    pub DepthEnable: BOOL,
    pub DepthWriteMask: u32,
    pub DepthFunc: u32,
    pub StencilEnable: BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: DepthStencilOpDesc,
    pub BackFace: DepthStencilOpDesc,
    pub DepthBoundsTestEnable: BOOL,
}
const _: () = assert!(std::mem::size_of::<DepthStencilDesc1>() == 56);

/// `D3D12_DEPTH_STENCIL_DESC2` (60 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DepthStencilDesc2 {
    pub DepthEnable: BOOL,
    pub DepthWriteMask: u32,
    pub DepthFunc: u32,
    pub StencilEnable: BOOL,
    pub FrontFace: DepthStencilOpDesc1,
    pub BackFace: DepthStencilOpDesc1,
    pub DepthBoundsTestEnable: BOOL,
}
const _: () = assert!(std::mem::size_of::<DepthStencilDesc2>() == 60);

/// `D3D12_BLEND`.
pub mod blend {
    pub const ZERO: u32 = 1;
    pub const ONE: u32 = 2;
    pub const SRC_COLOR: u32 = 3;
    pub const INV_SRC_COLOR: u32 = 4;
    pub const SRC_ALPHA: u32 = 5;
    pub const INV_SRC_ALPHA: u32 = 6;
    pub const DEST_ALPHA: u32 = 7;
    pub const INV_DEST_ALPHA: u32 = 8;
    pub const DEST_COLOR: u32 = 9;
    pub const INV_DEST_COLOR: u32 = 10;
    pub const SRC_ALPHA_SAT: u32 = 11;
    pub const BLEND_FACTOR: u32 = 14;
    pub const INV_BLEND_FACTOR: u32 = 15;
    pub const SRC1_COLOR: u32 = 16;
    pub const INV_SRC1_COLOR: u32 = 17;
    pub const SRC1_ALPHA: u32 = 18;
    pub const INV_SRC1_ALPHA: u32 = 19;
    pub const ALPHA_FACTOR: u32 = 20;
    pub const INV_ALPHA_FACTOR: u32 = 21;
}

/// `D3D12_LOGIC_OP` values that need no logic-op hardware.
pub mod logic_op {
    pub const COPY: u32 = 2;
    pub const NOOP: u32 = 4;
}

// ---- blend --------------------------------------------------------------------------------------

/// `D3D12_BLEND` → `MTLBlendFactor`. Unknown values give `None`.
pub fn blend_factor(b: u32) -> Option<MTLBlendFactor> {
    use MTLBlendFactor as F;
    Some(match b {
        blend::ZERO => F::Zero,
        blend::ONE => F::One,
        blend::SRC_COLOR => F::SourceColor,
        blend::INV_SRC_COLOR => F::OneMinusSourceColor,
        blend::SRC_ALPHA => F::SourceAlpha,
        blend::INV_SRC_ALPHA => F::OneMinusSourceAlpha,
        blend::DEST_ALPHA => F::DestinationAlpha,
        blend::INV_DEST_ALPHA => F::OneMinusDestinationAlpha,
        blend::DEST_COLOR => F::DestinationColor,
        blend::INV_DEST_COLOR => F::OneMinusDestinationColor,
        blend::SRC_ALPHA_SAT => F::SourceAlphaSaturated,
        blend::BLEND_FACTOR => F::BlendColor,
        blend::INV_BLEND_FACTOR => F::OneMinusBlendColor,
        blend::SRC1_COLOR => F::Source1Color,
        blend::INV_SRC1_COLOR => F::OneMinusSource1Color,
        blend::SRC1_ALPHA => F::Source1Alpha,
        blend::INV_SRC1_ALPHA => F::OneMinusSource1Alpha,
        blend::ALPHA_FACTOR => F::BlendAlpha,
        blend::INV_ALPHA_FACTOR => F::OneMinusBlendAlpha,
        _ => return None,
    })
}

/// `D3D12_BLEND_OP` (ADD 1 .. MAX 5) → `MTLBlendOperation`.
pub fn blend_op(op: u32) -> Option<MTLBlendOperation> {
    (1..=5).contains(&op).then(|| MTLBlendOperation(op as usize - 1))
}

/// `D3D12_COLOR_WRITE_ENABLE` (R=1 G=2 B=4 A=8) → `MTLColorWriteMask` (R=8 G=4 B=2 A=1).
pub fn write_mask(m: u8) -> MTLColorWriteMask {
    let mut out = MTLColorWriteMask::None;
    for (bit, mtl) in [(1, MTLColorWriteMask::Red), (2, MTLColorWriteMask::Green), (4, MTLColorWriteMask::Blue), (8, MTLColorWriteMask::Alpha)] {
        if m & bit != 0 {
            out |= mtl;
        }
    }
    out
}

fn is_src1(b: u32) -> bool {
    (blend::SRC1_COLOR..=blend::INV_SRC1_ALPHA).contains(&b)
}
fn is_constant(b: u32) -> bool {
    matches!(b, blend::BLEND_FACTOR | blend::INV_BLEND_FACTOR | blend::ALPHA_FACTOR | blend::INV_ALPHA_FACTOR)
}

/// One color attachment's blend configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ColorBlend {
    pub enabled: bool,
    pub src_rgb: MTLBlendFactor,
    pub dst_rgb: MTLBlendFactor,
    pub op_rgb: MTLBlendOperation,
    pub src_alpha: MTLBlendFactor,
    pub dst_alpha: MTLBlendFactor,
    pub op_alpha: MTLBlendOperation,
    pub write_mask: MTLColorWriteMask,
}

impl Default for ColorBlend {
    fn default() -> Self {
        Self {
            enabled: false,
            src_rgb: MTLBlendFactor::One,
            dst_rgb: MTLBlendFactor::Zero,
            op_rgb: MTLBlendOperation::Add,
            src_alpha: MTLBlendFactor::One,
            dst_alpha: MTLBlendFactor::Zero,
            op_alpha: MTLBlendOperation::Add,
            write_mask: MTLColorWriteMask::All,
        }
    }
}

impl ColorBlend {
    /// True when any blend factor references the secondary pixel-shader output (`SRC1_*`).
    #[inline]
    pub fn uses_dual_source(&self) -> bool {
        use MTLBlendFactor as F;
        [self.src_rgb, self.dst_rgb, self.src_alpha, self.dst_alpha].iter().any(|f| {
            matches!(
                *f,
                F::Source1Color
                    | F::OneMinusSource1Color
                    | F::Source1Alpha
                    | F::OneMinusSource1Alpha
            )
        })
    }

    /// True when any blend factor references the constant blend color/alpha (`OMSetBlendFactor`).
    #[inline]
    pub fn uses_blend_constant(&self) -> bool {
        use MTLBlendFactor as F;
        [self.src_rgb, self.dst_rgb, self.src_alpha, self.dst_alpha].iter().any(|f| {
            matches!(
                *f,
                F::BlendColor
                    | F::OneMinusBlendColor
                    | F::BlendAlpha
                    | F::OneMinusBlendAlpha
            )
        })
    }

    /// True when blending is masked out (`write_mask == None`) or mathematically
    /// identical to unblended overwrite (`1 * Src + 0 * Dst` for RGB and Alpha).
    #[inline]
    pub fn is_effective_blend_noop(&self) -> bool {
        self.write_mask == MTLColorWriteMask::None
            || (self.src_rgb == MTLBlendFactor::One
                && self.dst_rgb == MTLBlendFactor::Zero
                && self.op_rgb == MTLBlendOperation::Add
                && self.src_alpha == MTLBlendFactor::One
                && self.dst_alpha == MTLBlendFactor::Zero
                && self.op_alpha == MTLBlendOperation::Add)
    }

    /// Configures blending and the write mask. The pixel format is the caller's.
    pub fn apply_to(&self, a: &MTL4RenderPipelineColorAttachmentDescriptor) {
        self.apply_to_with_opt(a, false);
    }

    /// Like [`Self::apply_to`], optionally disabling `MTL4BlendState` when
    /// `opt_v2 && self.is_effective_blend_noop()` so AGX TBDR omits dead framebuffer
    /// blend-read preambles on masked-out or identity-blend MRT slots.
    pub fn apply_to_with_opt(&self, a: &MTL4RenderPipelineColorAttachmentDescriptor, opt_v2: bool) {
        let en = self.enabled && !(opt_v2 && self.is_effective_blend_noop());
        a.setBlendingState(if en { MTL4BlendState::Enabled } else { MTL4BlendState::Disabled });
        a.setSourceRGBBlendFactor(self.src_rgb);
        a.setDestinationRGBBlendFactor(self.dst_rgb);
        a.setRgbBlendOperation(self.op_rgb);
        a.setSourceAlphaBlendFactor(self.src_alpha);
        a.setDestinationAlphaBlendFactor(self.dst_alpha);
        a.setAlphaBlendOperation(self.op_alpha);
        a.setWriteMask(self.write_mask);
    }
}

/// Translated `D3D12_BLEND_DESC`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlendState {
    /// `MTL4RenderPipelineDescriptor::setAlphaToCoverageState`.
    pub alpha_to_coverage: bool,
    /// Per render target. `IndependentBlendEnable = FALSE` has already been expanded from RT0.
    pub attachments: [ColorBlend; 8],
    /// Some attachment reads `SRC1_*`. Compile the pixel shader with dual-source blending
    /// (`CompileOptions::dual_source_blending`). D3D12 then allows only RT0.
    pub dual_source: bool,
    /// Some attachment uses the blend constant (`OMSetBlendFactor` → `setBlendColor`).
    pub uses_blend_constant: bool,
    /// Render targets with a logic op Metal can't do: (index, `D3D12_LOGIC_OP`). COPY and NOOP
    /// are handled, as blending off and write mask 0 respectively.
    pub unsupported_logic_ops: Vec8,
    /// Render targets with out-of-range blend enums (they fall back to no blending).
    pub invalid: Vec8,
}

/// Tiny fixed-capacity list of (render target index, value) pairs (Copy + Hash).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec8 {
    pub len: u8,
    pub items: [(u8, u32); 8],
}
impl Vec8 {
    fn push(&mut self, i: usize, v: u32) {
        self.items[self.len as usize] = (i as u8, v);
        self.len += 1;
    }
    pub fn as_slice(&self) -> &[(u8, u32)] {
        &self.items[..self.len as usize]
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Translates `D3D12_BLEND_DESC`.
pub fn blend_state(d: &BlendDesc) -> BlendState {
    let mut out = BlendState {
        alpha_to_coverage: d.AlphaToCoverageEnable != 0,
        attachments: [ColorBlend::default(); 8],
        dual_source: false,
        uses_blend_constant: false,
        unsupported_logic_ops: Vec8::default(),
        invalid: Vec8::default(),
    };
    for i in 0..8 {
        let rt = if d.IndependentBlendEnable != 0 { &d.RenderTarget[i] } else { &d.RenderTarget[0] };
        let mut c = ColorBlend { write_mask: write_mask(rt.RenderTargetWriteMask), ..ColorBlend::default() };
        if rt.LogicOpEnable != 0 {
            // D3D12 forbids BlendEnable together with LogicOpEnable.
            match rt.LogicOp {
                logic_op::COPY => {}
                logic_op::NOOP => c.write_mask = MTLColorWriteMask::None,
                op => out.unsupported_logic_ops.push(i, op),
            }
        } else if rt.BlendEnable != 0 {
            let f = |b| blend_factor(b);
            match (f(rt.SrcBlend), f(rt.DestBlend), blend_op(rt.BlendOp), f(rt.SrcBlendAlpha), f(rt.DestBlendAlpha), blend_op(rt.BlendOpAlpha)) {
                (Some(src_rgb), Some(dst_rgb), Some(op_rgb), Some(src_alpha), Some(dst_alpha), Some(op_alpha)) => {
                    c = ColorBlend { enabled: true, src_rgb, dst_rgb, op_rgb, src_alpha, dst_alpha, op_alpha, write_mask: c.write_mask };
                    let factors = [rt.SrcBlend, rt.DestBlend, rt.SrcBlendAlpha, rt.DestBlendAlpha];
                    out.dual_source |= factors.iter().any(|&b| is_src1(b));
                    out.uses_blend_constant |= factors.iter().any(|&b| is_constant(b));
                }
                _ => out.invalid.push(i, rt.SrcBlend),
            }
        }
        out.attachments[i] = c;
    }
    out
}

/// Converter options for a graphics PSO's pixel shader. This is the one shared rule, so the
/// runtime PSO path and offline tools (`d3d12-replay --shaders`) produce the same shader
/// cache keys:
/// * dual-source blending is `ForceEnabled` iff some attachment reads `SRC1_*`, otherwise
///   `DecideAtRuntime`;
/// * `int_rt_mask` bit i is set iff `rtv_formats[i]` is a UINT/SINT format.
///
/// Everything else is `CompileOptions::default()`.
pub fn pixel_compile_options(blend: &BlendState, rtv_formats: &[u32]) -> dxil_metal::CompileOptions {
    let mut o = dxil_metal::CompileOptions::default();
    if blend.dual_source {
        o.dual_source_blending = dxil_metal::Tristate::ForceEnabled;
    }
    for (i, &f) in rtv_formats.iter().enumerate().take(8) {
        if matches!(crate::format::info(f).num, crate::format::Num::Uint | crate::format::Num::Sint) {
            o.int_rt_mask |= 1 << i;
        }
    }
    o
}

// ---- rasterizer ---------------------------------------------------------------------------------

/// `D3D12_FILL_MODE` / `D3D12_CULL_MODE` / `D3D12_CONSERVATIVE_RASTERIZATION_MODE` /
/// `D3D12_LINE_RASTERIZATION_MODE` values.
pub mod raster {
    pub const FILL_WIREFRAME: u32 = 2;
    pub const FILL_SOLID: u32 = 3;
    pub const CULL_NONE: u32 = 1;
    pub const CULL_FRONT: u32 = 2;
    pub const CULL_BACK: u32 = 3;
    pub const CONSERVATIVE_OFF: u32 = 0;
    pub const CONSERVATIVE_ON: u32 = 1;
    pub const LINE_ALIASED: u32 = 0;
    pub const LINE_ALPHA_ANTIALIASED: u32 = 1;
    pub const LINE_QUADRILATERAL_WIDE: u32 = 2;
    pub const LINE_QUADRILATERAL_NARROW: u32 = 3;
}

/// Rasterizer state as Metal 4 encoder calls (replay at draw time with [`RasterState::apply`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RasterState {
    pub fill: MTLTriangleFillMode,
    pub cull: MTLCullMode,
    pub winding: MTLWinding,
    /// `setDepthBias:slopeScale:clamp:`. D3D12's constant bias is in units of the depth
    /// format's minimum resolvable difference, and so is Metal's, so the value carries over
    /// unchanged. One caveat: D24 is emulated with Depth32Float. There the unit is
    /// 2^(e-23) of the primitive's max depth instead of a constant 2^-24. That is equal for
    /// depth in [0.5, 1) and weaker closer to the camera. This is acceptable for shadow and
    /// decal bias.
    pub depth_bias: f32,
    pub slope_scale: f32,
    pub bias_clamp: f32,
    pub depth_clip: MTLDepthClipMode,
    /// `ConservativeRaster = ON` (Metal has no conservative rasterization).
    pub unsupported_conservative: bool,
    /// `ForcedSampleCount > 0`: UAV-only rendering with forced MSAA rasterization. Metal can't do
    /// this per pipeline. Render with a sample-count-N attachment-less pass instead.
    pub forced_sample_count: u32,
    /// Wide/AA line modes (Metal draws 1-pixel aliased lines only).
    pub non_aliased_lines: bool,
}

impl Default for RasterState {
    /// `D3D12_DEFAULT` rasterizer: solid, cull back, clockwise front, depth clip on.
    fn default() -> Self {
        raster_state(&RasterizerDesc {
            FillMode: raster::FILL_SOLID,
            CullMode: raster::CULL_BACK,
            DepthClipEnable: 1,
            ..Default::default()
        })
    }
}

impl RasterState {
    /// Sets fill, cull, winding, depth bias and depth clip on a render encoder.
    pub fn apply(&self, enc: &objc2::runtime::ProtocolObject<dyn MTL4RenderCommandEncoder>) {
        enc.setTriangleFillMode(self.fill);
        enc.setCullMode(self.cull);
        enc.setFrontFacingWinding(self.winding);
        enc.setDepthBias_slopeScale_clamp(self.depth_bias, self.slope_scale, self.bias_clamp);
        enc.setDepthClipMode(self.depth_clip);
    }
}

#[allow(clippy::too_many_arguments)]
fn raster_common(
    fill: u32,
    cull: u32,
    front_ccw: BOOL,
    bias: f32,
    clamp: f32,
    slope: f32,
    clip: BOOL,
    forced: u32,
    conservative: u32,
    wide_lines: bool,
) -> RasterState {
    RasterState {
        fill: if fill == raster::FILL_WIREFRAME { MTLTriangleFillMode::Lines } else { MTLTriangleFillMode::Fill },
        cull: match cull {
            raster::CULL_FRONT => MTLCullMode::Front,
            raster::CULL_BACK => MTLCullMode::Back,
            _ => MTLCullMode::None,
        },
        // D3D12 and Metal share the y-up NDC and top-left framebuffer origin, so the winding
        // carries over as-is.
        winding: if front_ccw != 0 { MTLWinding::CounterClockwise } else { MTLWinding::Clockwise },
        depth_bias: bias,
        slope_scale: slope,
        bias_clamp: clamp,
        depth_clip: if clip != 0 { MTLDepthClipMode::Clip } else { MTLDepthClipMode::Clamp },
        unsupported_conservative: conservative != raster::CONSERVATIVE_OFF,
        forced_sample_count: forced,
        non_aliased_lines: wide_lines,
    }
}

/// Translates `D3D12_RASTERIZER_DESC`.
pub fn raster_state(d: &RasterizerDesc) -> RasterState {
    raster_common(
        d.FillMode,
        d.CullMode,
        d.FrontCounterClockwise,
        d.DepthBias as f32,
        d.DepthBiasClamp,
        d.SlopeScaledDepthBias,
        d.DepthClipEnable,
        d.ForcedSampleCount,
        d.ConservativeRaster,
        d.AntialiasedLineEnable != 0 && d.MultisampleEnable == 0,
    )
}

/// Translates `D3D12_RASTERIZER_DESC1`.
pub fn raster_state1(d: &RasterizerDesc1) -> RasterState {
    raster_common(
        d.FillMode,
        d.CullMode,
        d.FrontCounterClockwise,
        d.DepthBias,
        d.DepthBiasClamp,
        d.SlopeScaledDepthBias,
        d.DepthClipEnable,
        d.ForcedSampleCount,
        d.ConservativeRaster,
        d.AntialiasedLineEnable != 0 && d.MultisampleEnable == 0,
    )
}

/// Translates `D3D12_RASTERIZER_DESC2`.
pub fn raster_state2(d: &RasterizerDesc2) -> RasterState {
    raster_common(
        d.FillMode,
        d.CullMode,
        d.FrontCounterClockwise,
        d.DepthBias,
        d.DepthBiasClamp,
        d.SlopeScaledDepthBias,
        d.DepthClipEnable,
        d.ForcedSampleCount,
        d.ConservativeRaster,
        d.LineRasterizationMode != raster::LINE_ALIASED,
    )
}

// ---- depth / stencil ----------------------------------------------------------------------------

/// `D3D12_COMPARISON_FUNC` (NEVER 1 .. ALWAYS 8) → `MTLCompareFunction`. NONE (0) and unknown
/// values are treated as ALWAYS.
pub fn compare_function(f: u32) -> MTLCompareFunction {
    if (1..=8).contains(&f) {
        MTLCompareFunction(f as usize - 1)
    } else {
        MTLCompareFunction::Always
    }
}

/// `D3D12_STENCIL_OP` (KEEP 1 .. DECR 8) → `MTLStencilOperation`. Unknown values become KEEP.
pub fn stencil_op(op: u32) -> MTLStencilOperation {
    if (1..=8).contains(&op) {
        MTLStencilOperation(op as usize - 1)
    } else {
        MTLStencilOperation::Keep
    }
}

/// One face's stencil configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StencilFace {
    pub compare: MTLCompareFunction,
    pub fail: MTLStencilOperation,
    pub depth_fail: MTLStencilOperation,
    pub pass: MTLStencilOperation,
    pub read_mask: u32,
    pub write_mask: u32,
}

impl StencilFace {
    fn new(op: (u32, u32, u32, u32), read: u8, write: u8) -> Self {
        Self {
            fail: stencil_op(op.0),
            depth_fail: stencil_op(op.1),
            pass: stencil_op(op.2),
            compare: compare_function(op.3),
            read_mask: read as u32,
            write_mask: write as u32,
        }
    }
    fn descriptor(&self) -> objc2::rc::Retained<MTLStencilDescriptor> {
        let s = MTLStencilDescriptor::new();
        s.setStencilCompareFunction(self.compare);
        s.setStencilFailureOperation(self.fail);
        s.setDepthFailureOperation(self.depth_fail);
        s.setDepthStencilPassOperation(self.pass);
        s.setReadMask(self.read_mask);
        s.setWriteMask(self.write_mask);
        s
    }
}

/// Translated depth-stencil description. Hashable, so it can key an `MTLDepthStencilState` cache.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DepthStencilState {
    /// `Always` when depth is disabled.
    pub depth_compare: MTLCompareFunction,
    pub depth_write: bool,
    /// `None` when stencil is disabled.
    pub front: Option<StencilFace>,
    pub back: Option<StencilFace>,
    /// Depth bounds test enabled. Metal 4 encoder state: `setDepthTestMinBound:maxBound:` with
    /// the values from `OMSetDepthBounds`, or (0, 1) when disabled.
    pub depth_bounds: bool,
}

impl Default for DepthStencilState {
    /// `D3D12_DEFAULT`: depth test LESS with writes, no stencil.
    fn default() -> Self {
        Self { depth_compare: MTLCompareFunction::Less, depth_write: true, front: None, back: None, depth_bounds: false }
    }
}

impl DepthStencilState {
    /// Fills `d`, which should be freshly created. Disabled stencil leaves the faces nil.
    pub fn apply_to(&self, d: &MTLDepthStencilDescriptor) {
        d.setDepthCompareFunction(self.depth_compare);
        d.setDepthWriteEnabled(self.depth_write);
        d.setFrontFaceStencil(self.front.map(|f| f.descriptor()).as_deref());
        d.setBackFaceStencil(self.back.map(|f| f.descriptor()).as_deref());
    }

    /// The same state for a PSO without a depth-stencil format (`DSVFormat` UNKNOWN), where
    /// D3D12 ignores depth and stencil entirely.
    pub fn without_depth_stencil() -> Self {
        Self { depth_compare: MTLCompareFunction::Always, depth_write: false, front: None, back: None, depth_bounds: false }
    }
}

fn ds_common(
    depth_enable: BOOL,
    write_mask: u32,
    func: u32,
    stencil_enable: BOOL,
    front: StencilFace,
    back: StencilFace,
    bounds: BOOL,
) -> DepthStencilState {
    let depth = depth_enable != 0;
    let stencil = stencil_enable != 0;
    DepthStencilState {
        depth_compare: if depth { compare_function(func) } else { MTLCompareFunction::Always },
        depth_write: depth && write_mask != 0,
        front: stencil.then_some(front),
        back: stencil.then_some(back),
        depth_bounds: bounds != 0,
    }
}

fn op_tuple(o: &DepthStencilOpDesc) -> (u32, u32, u32, u32) {
    (o.StencilFailOp, o.StencilDepthFailOp, o.StencilPassOp, o.StencilFunc)
}

/// Translates `D3D12_DEPTH_STENCIL_DESC`.
pub fn depth_stencil_state(d: &DepthStencilDesc) -> DepthStencilState {
    let face = |o: &DepthStencilOpDesc| StencilFace::new(op_tuple(o), d.StencilReadMask, d.StencilWriteMask);
    ds_common(d.DepthEnable, d.DepthWriteMask, d.DepthFunc, d.StencilEnable, face(&d.FrontFace), face(&d.BackFace), 0)
}

/// Translates `D3D12_DEPTH_STENCIL_DESC1`.
pub fn depth_stencil_state1(d: &DepthStencilDesc1) -> DepthStencilState {
    let face = |o: &DepthStencilOpDesc| StencilFace::new(op_tuple(o), d.StencilReadMask, d.StencilWriteMask);
    ds_common(d.DepthEnable, d.DepthWriteMask, d.DepthFunc, d.StencilEnable, face(&d.FrontFace), face(&d.BackFace), d.DepthBoundsTestEnable)
}

/// Translates `D3D12_DEPTH_STENCIL_DESC2` (per-face stencil masks).
pub fn depth_stencil_state2(d: &DepthStencilDesc2) -> DepthStencilState {
    let face = |o: &DepthStencilOpDesc1| {
        StencilFace::new((o.StencilFailOp, o.StencilDepthFailOp, o.StencilPassOp, o.StencilFunc), o.StencilReadMask, o.StencilWriteMask)
    };
    ds_common(d.DepthEnable, d.DepthWriteMask, d.DepthFunc, d.StencilEnable, face(&d.FrontFace), face(&d.BackFace), d.DepthBoundsTestEnable)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rt(src: u32, dst: u32, op: u32) -> RenderTargetBlendDesc {
        RenderTargetBlendDesc {
            BlendEnable: 1,
            SrcBlend: src,
            DestBlend: dst,
            BlendOp: op,
            SrcBlendAlpha: blend::ONE,
            DestBlendAlpha: blend::ZERO,
            BlendOpAlpha: 1,
            RenderTargetWriteMask: 0xF,
            ..Default::default()
        }
    }

    #[test]
    fn layouts() {
        use std::mem::offset_of;
        assert_eq!(offset_of!(RenderTargetBlendDesc, RenderTargetWriteMask), 36);
        assert_eq!(offset_of!(BlendDesc, RenderTarget), 8);
        assert_eq!(offset_of!(RasterizerDesc, ConservativeRaster), 40);
        assert_eq!(offset_of!(RasterizerDesc2, ConservativeRaster), 36);
        assert_eq!(offset_of!(DepthStencilDesc, StencilWriteMask), 17);
        assert_eq!(offset_of!(DepthStencilDesc, FrontFace), 20);
        assert_eq!(offset_of!(DepthStencilDesc, BackFace), 36);
        assert_eq!(offset_of!(DepthStencilDesc1, DepthBoundsTestEnable), 52);
        assert_eq!(offset_of!(DepthStencilDesc2, FrontFace), 16);
        assert_eq!(offset_of!(DepthStencilDesc2, BackFace), 36);
        assert_eq!(offset_of!(DepthStencilDesc2, DepthBoundsTestEnable), 56);
    }

    #[test]
    fn blend_enums() {
        assert_eq!(blend_factor(blend::SRC_ALPHA), Some(MTLBlendFactor::SourceAlpha));
        assert_eq!(blend_factor(blend::INV_SRC_ALPHA), Some(MTLBlendFactor::OneMinusSourceAlpha));
        assert_eq!(blend_factor(blend::DEST_COLOR), Some(MTLBlendFactor::DestinationColor));
        assert_eq!(blend_factor(blend::BLEND_FACTOR), Some(MTLBlendFactor::BlendColor));
        assert_eq!(blend_factor(blend::ALPHA_FACTOR), Some(MTLBlendFactor::BlendAlpha));
        assert_eq!(blend_factor(blend::INV_SRC1_ALPHA), Some(MTLBlendFactor::OneMinusSource1Alpha));
        assert_eq!(blend_factor(12), None); // gap in D3D12_BLEND
        assert_eq!(blend_factor(0), None);
        assert_eq!(blend_op(1), Some(MTLBlendOperation::Add));
        assert_eq!(blend_op(3), Some(MTLBlendOperation::ReverseSubtract));
        assert_eq!(blend_op(5), Some(MTLBlendOperation::Max));
        assert_eq!(blend_op(6), None);
        assert_eq!(write_mask(0xF), MTLColorWriteMask::All);
        assert_eq!(write_mask(0x1), MTLColorWriteMask::Red);
        assert_eq!(write_mask(0x8), MTLColorWriteMask::Alpha);
        assert_eq!(write_mask(0x5), MTLColorWriteMask::Red | MTLColorWriteMask::Blue);
        assert_eq!(write_mask(0), MTLColorWriteMask::None);
    }

    #[test]
    fn blend_desc_expansion() {
        let mut d = BlendDesc::default();
        d.RenderTarget[0] = rt(blend::SRC_ALPHA, blend::INV_SRC_ALPHA, 1);
        d.RenderTarget[3] = rt(blend::ONE, blend::ONE, 1); // ignored: IndependentBlendEnable = FALSE
        let s = blend_state(&d);
        for a in &s.attachments {
            assert!(a.enabled);
            assert_eq!((a.src_rgb, a.dst_rgb), (MTLBlendFactor::SourceAlpha, MTLBlendFactor::OneMinusSourceAlpha));
        }
        d.IndependentBlendEnable = 1;
        let s = blend_state(&d);
        assert_eq!(s.attachments[3].dst_rgb, MTLBlendFactor::One);
        assert!(!s.attachments[1].enabled);
        assert_eq!(s.attachments[1].write_mask, MTLColorWriteMask::None); // zeroed desc = mask 0
        assert!(!s.dual_source && !s.uses_blend_constant && s.invalid.is_empty());
    }

    #[test]
    fn blend_flags() {
        let mut d = BlendDesc { AlphaToCoverageEnable: 1, ..Default::default() };
        d.RenderTarget[0] = rt(blend::ONE, blend::INV_SRC1_COLOR, 1);
        let s = blend_state(&d);
        assert!(s.alpha_to_coverage && s.dual_source && !s.uses_blend_constant);
        d.RenderTarget[0] = rt(blend::BLEND_FACTOR, blend::ZERO, 1);
        assert!(blend_state(&d).uses_blend_constant);
        d.RenderTarget[0] = rt(99, blend::ZERO, 1);
        let s = blend_state(&d);
        assert_eq!(s.invalid.as_slice()[0], (0, 99));
        assert!(!s.attachments[0].enabled);
        // Logic ops: COPY is plain, NOOP masks writes, XOR is unsupported.
        d.RenderTarget[0] = RenderTargetBlendDesc { LogicOpEnable: 1, LogicOp: logic_op::NOOP, RenderTargetWriteMask: 0xF, ..Default::default() };
        let s = blend_state(&d);
        assert_eq!(s.attachments[0].write_mask, MTLColorWriteMask::None);
        d.RenderTarget[0].LogicOp = 6; // XOR
        assert_eq!(blend_state(&d).unsupported_logic_ops.as_slice()[0], (0, 6));
    }

    #[test]
    fn pixel_options() {
        use dxil_metal::{CompileOptions, Tristate};
        let mut d = BlendDesc::default();
        d.RenderTarget[0] = rt(blend::ONE, blend::ZERO, 1);
        // R8G8B8A8_UNORM, R16_UINT, R32_SINT
        let o = pixel_compile_options(&blend_state(&d), &[28, 57, 43]);
        assert_eq!(o, CompileOptions { int_rt_mask: 0b110, ..Default::default() });
        d.RenderTarget[0] = rt(blend::ONE, blend::INV_SRC1_COLOR, 1);
        let o = pixel_compile_options(&blend_state(&d), &[28]);
        assert_eq!((o.dual_source_blending, o.int_rt_mask), (Tristate::ForceEnabled, 0));
    }

    #[test]
    fn rasterizer() {
        let d = RasterState::default();
        assert_eq!((d.fill, d.cull, d.winding, d.depth_clip), (MTLTriangleFillMode::Fill, MTLCullMode::Back, MTLWinding::Clockwise, MTLDepthClipMode::Clip));
        let r = raster_state(&RasterizerDesc {
            FillMode: raster::FILL_WIREFRAME,
            CullMode: raster::CULL_NONE,
            FrontCounterClockwise: 1,
            DepthBias: -100,
            DepthBiasClamp: 0.5,
            SlopeScaledDepthBias: 2.0,
            DepthClipEnable: 0,
            ConservativeRaster: raster::CONSERVATIVE_ON,
            ForcedSampleCount: 4,
            ..Default::default()
        });
        assert_eq!((r.fill, r.cull, r.winding), (MTLTriangleFillMode::Lines, MTLCullMode::None, MTLWinding::CounterClockwise));
        assert_eq!((r.depth_bias, r.bias_clamp, r.slope_scale), (-100.0, 0.5, 2.0));
        assert_eq!(r.depth_clip, MTLDepthClipMode::Clamp);
        assert!(r.unsupported_conservative && r.forced_sample_count == 4);
        let r1 = raster_state1(&RasterizerDesc1 { CullMode: raster::CULL_FRONT, DepthBias: 1.5, ..Default::default() });
        assert_eq!((r1.cull, r1.depth_bias), (MTLCullMode::Front, 1.5));
        let r2 = raster_state2(&RasterizerDesc2 { LineRasterizationMode: raster::LINE_QUADRILATERAL_WIDE, ..Default::default() });
        assert!(r2.non_aliased_lines);
    }

    #[test]
    fn depth_stencil() {
        assert_eq!(compare_function(1), MTLCompareFunction::Never);
        assert_eq!(compare_function(4), MTLCompareFunction::LessEqual);
        assert_eq!(compare_function(7), MTLCompareFunction::GreaterEqual);
        assert_eq!(compare_function(0), MTLCompareFunction::Always);
        assert_eq!(stencil_op(1), MTLStencilOperation::Keep);
        assert_eq!(stencil_op(4), MTLStencilOperation::IncrementClamp);
        assert_eq!(stencil_op(8), MTLStencilOperation::DecrementWrap);

        let face = DepthStencilOpDesc { StencilFailOp: 1, StencilDepthFailOp: 7, StencilPassOp: 3, StencilFunc: 8 };
        let d = DepthStencilDesc {
            DepthEnable: 1,
            DepthWriteMask: 1,
            DepthFunc: 7, // GREATER_EQUAL (reverse Z)
            StencilEnable: 1,
            StencilReadMask: 0xF0,
            StencilWriteMask: 0x0F,
            FrontFace: face,
            BackFace: DepthStencilOpDesc { StencilFunc: 3, ..face },
        };
        let s = depth_stencil_state(&d);
        assert_eq!((s.depth_compare, s.depth_write), (MTLCompareFunction::GreaterEqual, true));
        let f = s.front.unwrap();
        assert_eq!((f.fail, f.depth_fail, f.pass, f.compare), (MTLStencilOperation::Keep, MTLStencilOperation::IncrementWrap, MTLStencilOperation::Replace, MTLCompareFunction::Always));
        assert_eq!((f.read_mask, f.write_mask), (0xF0, 0x0F));
        assert_eq!(s.back.unwrap().compare, MTLCompareFunction::Equal);

        // Depth disabled means no test and no writes, whatever the mask says.
        let s = depth_stencil_state(&DepthStencilDesc { DepthEnable: 0, DepthWriteMask: 1, DepthFunc: 2, ..d });
        assert_eq!((s.depth_compare, s.depth_write), (MTLCompareFunction::Always, false));
        let s = depth_stencil_state(&DepthStencilDesc { StencilEnable: 0, ..d });
        assert!(s.front.is_none() && s.back.is_none());

        let s1 = depth_stencil_state1(&DepthStencilDesc1 { DepthEnable: 1, DepthFunc: 2, DepthBoundsTestEnable: 1, ..Default::default() });
        assert!(s1.depth_bounds && !s1.depth_write);
        let f1 = DepthStencilOpDesc1 { StencilFailOp: 2, StencilDepthFailOp: 1, StencilPassOp: 1, StencilFunc: 8, StencilReadMask: 1, StencilWriteMask: 2 };
        let s2 = depth_stencil_state2(&DepthStencilDesc2 {
            StencilEnable: 1,
            FrontFace: f1,
            BackFace: DepthStencilOpDesc1 { StencilReadMask: 3, StencilWriteMask: 4, ..f1 },
            ..Default::default()
        });
        assert_eq!((s2.front.unwrap().read_mask, s2.back.unwrap().write_mask), (1, 4));
        assert_eq!(s2.front.unwrap().fail, MTLStencilOperation::Zero);
    }

    #[test]
    fn apply_helpers_build_descriptors() {
        // Smoke test: building descriptors needs no device.
        let a = MTL4RenderPipelineColorAttachmentDescriptor::new();
        let mut d = BlendDesc::default();
        d.RenderTarget[0] = rt(blend::SRC_ALPHA, blend::INV_SRC_ALPHA, 1);
        blend_state(&d).attachments[0].apply_to(&a);
        assert_eq!(a.blendingState(), MTL4BlendState::Enabled);
        assert_eq!(a.destinationRGBBlendFactor(), MTLBlendFactor::OneMinusSourceAlpha);
        let ds = MTLDepthStencilDescriptor::new();
        let s = depth_stencil_state(&DepthStencilDesc { DepthEnable: 1, DepthWriteMask: 1, DepthFunc: 2, StencilEnable: 1, ..Default::default() });
        s.apply_to(&ds);
        assert!(ds.isDepthWriteEnabled());
        assert_eq!(ds.depthCompareFunction(), MTLCompareFunction::Less);
        assert_eq!(ds.frontFaceStencil().stencilCompareFunction(), MTLCompareFunction::Always);
    }
}
