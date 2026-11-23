//! DirectML tensor indexing, IEEE-754 float16 conversion, operator descriptor
//! parsing, and CPU tensor execution engine.

use super::abi::*;
use crate::strings::rd;
use crate::types::{E_INVALIDARG, E_NOTIMPL, HRESULT};

// ============================================================================
// Tensor Data Types, IEEE-754 f16, and Multi-Dimensional Strided Indexing
// ============================================================================

pub(super) fn elem_byte_size(dt: u32) -> Option<usize> {
    match dt {
        DML_TENSOR_DATA_TYPE_FLOAT32 | DML_TENSOR_DATA_TYPE_UINT32 | DML_TENSOR_DATA_TYPE_INT32 => Some(4),
        DML_TENSOR_DATA_TYPE_FLOAT16 | DML_TENSOR_DATA_TYPE_UINT16 | DML_TENSOR_DATA_TYPE_INT16 => Some(2),
        DML_TENSOR_DATA_TYPE_UINT8 | DML_TENSOR_DATA_TYPE_INT8 => Some(1),
        DML_TENSOR_DATA_TYPE_FLOAT64 | DML_TENSOR_DATA_TYPE_UINT64 | DML_TENSOR_DATA_TYPE_INT64 => Some(8),
        _ => None,
    }
}

fn f16_to_f32(bits: u16) -> f32 {
    let sign = ((bits & 0x8000) as u32) << 16;
    let exp = ((bits >> 10) & 0x1F) as u32;
    let frac = (bits & 0x03FF) as u32;
    let out = if exp == 0 {
        if frac == 0 {
            sign
        } else {
            let mut m = frac;
            let mut e = 127i32 - 14;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e -= 1;
            }
            m &= 0x03FF;
            sign | ((e as u32) << 23) | (m << 13)
        }
    } else if exp == 31 {
        sign | 0x7F80_0000 | (frac << 13)
    } else {
        sign | ((exp + 127 - 15) << 23) | (frac << 13)
    };
    f32::from_bits(out)
}

fn f32_to_f16(val: f32) -> u16 {
    let bits = val.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let frac = bits & 0x007F_FFFF;
    if exp == 255 {
        return if frac == 0 {
            sign | 0x7C00
        } else {
            sign | 0x7E00 | ((frac >> 13) as u16)
        };
    }
    let half_exp = exp - 127 + 15;
    if half_exp >= 31 {
        return sign | 0x7C00;
    }
    if half_exp <= 0 {
        if half_exp < -10 {
            return sign;
        }
        let m = (frac | 0x0080_0000) >> (14 - half_exp);
        let round = (m & 1) as u16;
        return sign | ((m >> 1) as u16).saturating_add(round);
    }
    let rounded_frac = frac + 0x0000_1000;
    if rounded_frac >= 0x0080_0000 {
        let next_exp = half_exp + 1;
        if next_exp >= 31 {
            sign | 0x7C00
        } else {
            sign | ((next_exp as u16) << 10)
        }
    } else {
        sign | ((half_exp as u16) << 10) | ((rounded_frac >> 13) as u16)
    }
}

#[derive(Clone, Debug)]
pub(super) struct TensorDesc {
    pub(super) data_type: u32,
    pub(super) flags: u32,
    pub(super) sizes: Vec<u32>,
    pub(super) strides: Vec<u32>,
    pub(super) total_bytes: u64,
}

impl TensorDesc {
    unsafe fn parse(p: *const DML_TENSOR_DESC) -> Result<Self, HRESULT> {
        if p.is_null() {
            return Err(E_INVALIDARG);
        }
        let td = rd(p);
        if td.Type != DML_TENSOR_TYPE_BUFFER || td.Desc.is_null() {
            return Err(E_INVALIDARG);
        }
        let bd = rd(td.Desc as *const DML_BUFFER_TENSOR_DESC);
        let elem_sz = elem_byte_size(bd.DataType).ok_or(E_INVALIDARG)?;
        if bd.DimensionCount == 0 || bd.DimensionCount > 8 || bd.Sizes.is_null() {
            return Err(E_INVALIDARG);
        }
        let rank = bd.DimensionCount as usize;
        let mut sizes = Vec::with_capacity(rank);
        for i in 0..rank {
            let s = rd(bd.Sizes.add(i));
            if s == 0 {
                return Err(E_INVALIDARG);
            }
            sizes.push(s);
        }
        let mut strides = vec![0u32; rank];
        if bd.Strides.is_null() {
            let mut acc = 1u32;
            for i in (0..rank).rev() {
                strides[i] = acc;
                acc = acc.saturating_mul(sizes[i]);
            }
        } else {
            for i in 0..rank {
                strides[i] = rd(bd.Strides.add(i));
            }
        }
        let mut max_elem_idx = 0u64;
        for i in 0..rank {
            max_elem_idx = max_elem_idx.saturating_add((sizes[i] as u64 - 1) * strides[i] as u64);
        }
        let min_bytes = max_elem_idx.saturating_add(1).saturating_mul(elem_sz as u64);
        let total_bytes = bd.TotalTensorSizeInBytes.max(min_bytes);
        Ok(Self {
            data_type: bd.DataType,
            flags: bd.Flags,
            sizes,
            strides,
            total_bytes,
        })
    }

    unsafe fn parse_opt(p: *const DML_TENSOR_DESC) -> Result<Option<Self>, HRESULT> {
        if p.is_null() {
            Ok(None)
        } else {
            Ok(Some(Self::parse(p)?))
        }
    }

    fn elem_count(&self) -> usize {
        self.sizes.iter().fold(1usize, |a, &b| a.saturating_mul(b as usize))
    }

    fn elem_size(&self) -> usize {
        elem_byte_size(self.data_type).unwrap_or(4)
    }

    fn byte_offset_coords(&self, coords: &[usize]) -> usize {
        let mut elem_off = 0usize;
        let r = self.sizes.len();
        let cr = coords.len();
        for d in 0..r {
            // Right-aligned broadcasting when coords has more dims than self.
            let c_idx = if cr >= r {
                coords[cr - r + d]
            } else if d >= r - cr {
                coords[d - (r - cr)]
            } else {
                0
            };
            let idx = if self.sizes[d] == 1 { 0 } else { c_idx.min(self.sizes[d] as usize - 1) };
            elem_off += idx * self.strides[d] as usize;
        }
        elem_off * self.elem_size()
    }

    fn linear_to_coords(&self, mut flat: usize, out: &mut [usize]) {
        for d in (0..self.sizes.len()).rev() {
            let sz = self.sizes[d] as usize;
            out[d] = flat % sz;
            flat /= sz;
        }
    }

    fn read_f64(&self, buf: &[u8], coords: &[usize]) -> f64 {
        let off = self.byte_offset_coords(coords);
        let sz = self.elem_size();
        if off + sz > buf.len() {
            return 0.0;
        }
        match self.data_type {
            DML_TENSOR_DATA_TYPE_FLOAT32 => f32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) as f64,
            DML_TENSOR_DATA_TYPE_FLOAT16 => f16_to_f32(u16::from_le_bytes(buf[off..off + 2].try_into().unwrap())) as f64,
            DML_TENSOR_DATA_TYPE_FLOAT64 => f64::from_le_bytes(buf[off..off + 8].try_into().unwrap()),
            DML_TENSOR_DATA_TYPE_UINT32 => u32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) as f64,
            DML_TENSOR_DATA_TYPE_INT32 => i32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) as f64,
            DML_TENSOR_DATA_TYPE_UINT16 => u16::from_le_bytes(buf[off..off + 2].try_into().unwrap()) as f64,
            DML_TENSOR_DATA_TYPE_INT16 => i16::from_le_bytes(buf[off..off + 2].try_into().unwrap()) as f64,
            DML_TENSOR_DATA_TYPE_UINT8 => buf[off] as f64,
            DML_TENSOR_DATA_TYPE_INT8 => (buf[off] as i8) as f64,
            DML_TENSOR_DATA_TYPE_UINT64 => u64::from_le_bytes(buf[off..off + 8].try_into().unwrap()) as f64,
            DML_TENSOR_DATA_TYPE_INT64 => i64::from_le_bytes(buf[off..off + 8].try_into().unwrap()) as f64,
            _ => 0.0,
        }
    }

    fn write_f64(&self, buf: &mut [u8], coords: &[usize], v: f64) {
        let off = self.byte_offset_coords(coords);
        let sz = self.elem_size();
        if off + sz > buf.len() {
            return;
        }
        match self.data_type {
            DML_TENSOR_DATA_TYPE_FLOAT32 => buf[off..off + 4].copy_from_slice(&(v as f32).to_le_bytes()),
            DML_TENSOR_DATA_TYPE_FLOAT16 => buf[off..off + 2].copy_from_slice(&f32_to_f16(v as f32).to_le_bytes()),
            DML_TENSOR_DATA_TYPE_FLOAT64 => buf[off..off + 8].copy_from_slice(&v.to_le_bytes()),
            DML_TENSOR_DATA_TYPE_UINT32 => buf[off..off + 4].copy_from_slice(&(v as u32).to_le_bytes()),
            DML_TENSOR_DATA_TYPE_INT32 => buf[off..off + 4].copy_from_slice(&(v as i32).to_le_bytes()),
            DML_TENSOR_DATA_TYPE_UINT16 => buf[off..off + 2].copy_from_slice(&(v as u16).to_le_bytes()),
            DML_TENSOR_DATA_TYPE_INT16 => buf[off..off + 2].copy_from_slice(&(v as i16).to_le_bytes()),
            DML_TENSOR_DATA_TYPE_UINT8 => buf[off] = v as u8,
            DML_TENSOR_DATA_TYPE_INT8 => buf[off] = (v as i8) as u8,
            DML_TENSOR_DATA_TYPE_UINT64 => buf[off..off + 8].copy_from_slice(&(v as u64).to_le_bytes()),
            DML_TENSOR_DATA_TYPE_INT64 => buf[off..off + 8].copy_from_slice(&(v as i64).to_le_bytes()),
            _ => {}
        }
    }
}

// ============================================================================
// Owned Operator Specs & Activation Functions
// ============================================================================

#[derive(Clone, Copy, Debug)]
pub(super) enum FusedAct {
    Identity,
    Relu,
    Sigmoid,
    Tanh,
    LeakyRelu(f32),
    Elu(f32),
    Linear(f32, f32),
    HardSigmoid(f32, f32),
    ScaledElu(f32, f32),
    ScaledTanh(f32, f32),
    Softplus(f32),
    Softsign,
    ThresholdedRelu(f32),
    Gelu,
    Celu(f32),
    Clip(f32, f32),
}

fn erf_approx(x: f64) -> f64 {
    // Abramowitz & Stegun 7.1.26 (max error < 1.5e-7)
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let ax = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * ax);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-ax * ax).exp();
    sign * y
}

impl FusedAct {
    unsafe fn parse_opt(p: *const DML_OPERATOR_DESC) -> Result<Option<Self>, HRESULT> {
        if p.is_null() {
            return Ok(None);
        }
        let d = rd(p);
        if d.Desc.is_null() {
            return Err(E_INVALIDARG);
        }
        let act = match d.Type {
            DML_OPERATOR_ACTIVATION_IDENTITY | DML_OPERATOR_ELEMENT_WISE_IDENTITY => FusedAct::Identity,
            DML_OPERATOR_ACTIVATION_RELU => FusedAct::Relu,
            DML_OPERATOR_ACTIVATION_SIGMOID => FusedAct::Sigmoid,
            DML_OPERATOR_ACTIVATION_TANH | DML_OPERATOR_ELEMENT_WISE_TANH => FusedAct::Tanh,
            DML_OPERATOR_ACTIVATION_LEAKY_RELU => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::LeakyRelu(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_ELU => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::Elu(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_LINEAR => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::Linear(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_HARD_SIGMOID => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::HardSigmoid(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_SCALED_ELU => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::ScaledElu(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_SCALED_TANH => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::ScaledTanh(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_SOFTPLUS => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::Softplus(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_SOFTSIGN => FusedAct::Softsign,
            DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::ThresholdedRelu(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_GELU => FusedAct::Gelu,
            DML_OPERATOR_ACTIVATION_CELU => {
                let s = rd(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::Celu(s.Alpha)
            }
            DML_OPERATOR_ELEMENT_WISE_CLIP => {
                let s = rd(d.Desc as *const DML_ELEMENT_WISE_CLIP_OPERATOR_DESC);
                FusedAct::Clip(s.Min, s.Max)
            }
            _ => return Err(E_NOTIMPL),
        };
        Ok(Some(act))
    }

    #[inline]
    fn apply(self, x: f64) -> f64 {
        match self {
            FusedAct::Identity => x,
            FusedAct::Relu => x.max(0.0),
            FusedAct::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            FusedAct::Tanh => x.tanh(),
            FusedAct::LeakyRelu(a) => if x >= 0.0 { x } else { (a as f64) * x },
            FusedAct::Elu(a) => if x >= 0.0 { x } else { (a as f64) * (x.exp() - 1.0) },
            FusedAct::Linear(a, b) => (a as f64) * x + (b as f64),
            FusedAct::HardSigmoid(a, b) => ((a as f64) * x + (b as f64)).clamp(0.0, 1.0),
            FusedAct::ScaledElu(a, g) => {
                (g as f64) * if x > 0.0 { x } else { (a as f64) * (x.exp() - 1.0) }
            }
            FusedAct::ScaledTanh(a, b) => (a as f64) * ((b as f64) * x).tanh(),
            FusedAct::Softplus(st) => {
                let s = st as f64;
                ((s * x).exp() + 1.0).ln() / s
            }
            FusedAct::Softsign => x / (1.0 + x.abs()),
            FusedAct::ThresholdedRelu(a) => if x > a as f64 { x } else { 0.0 },
            FusedAct::Gelu => 0.5 * x * (1.0 + erf_approx(x * std::f64::consts::FRAC_1_SQRT_2)),
            FusedAct::Celu(a) => {
                let af = a as f64;
                x.max(0.0) + (af * ((x / af).exp() - 1.0)).min(0.0)
            }
            FusedAct::Clip(lo, hi) => x.clamp(lo as f64, hi as f64),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum UnaryOp {
    Identity,
    Abs,
    Exp,
    Log,
    Sqrt,
    Neg,
    Clip(f32, f32),
    Recip,
    Floor,
    Ceil,
    Round,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sign,
    ConstPow(f32),
    Threshold(f32),
    Activation(FusedAct),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Mean,
    Pow,
}

#[derive(Clone, Debug)]
pub(super) enum OpSpec {
    Unary {
        op: UnaryOp,
        input: TensorDesc,
        output: TensorDesc,
        scale_bias: Option<(f32, f32)>,
    },
    Binary {
        op: BinaryOp,
        a: TensorDesc,
        b: TensorDesc,
        output: TensorDesc,
        scale_bias: Option<(f32, f32)>,
        fused: Option<FusedAct>,
    },
    PRelu {
        input: TensorDesc,
        slope: TensorDesc,
        output: TensorDesc,
    },
    Softmax {
        input: TensorDesc,
        output: TensorDesc,
        log_mode: bool,
    },
    Gemm {
        a: TensorDesc,
        b: TensorDesc,
        c: Option<TensorDesc>,
        output: TensorDesc,
        trans_a: bool,
        trans_b: bool,
        alpha: f32,
        beta: f32,
        fused: Option<FusedAct>,
    },
    Convolution {
        input: TensorDesc,
        filter: TensorDesc,
        bias: Option<TensorDesc>,
        output: TensorDesc,
        mode: u32,
        direction: u32,
        strides: Vec<u32>,
        dilations: Vec<u32>,
        start_pad: Vec<u32>,
        group_count: u32,
        fused: Option<FusedAct>,
    },
    AvgPool {
        input: TensorDesc,
        output: TensorDesc,
        strides: Vec<u32>,
        window: Vec<u32>,
        start_pad: Vec<u32>,
        include_padding: bool,
    },
    MaxPool {
        input: TensorDesc,
        output: TensorDesc,
        output_indices: Option<TensorDesc>,
        strides: Vec<u32>,
        window: Vec<u32>,
        start_pad: Vec<u32>,
        dilations: Vec<u32>,
    },
    Cast {
        input: TensorDesc,
        output: TensorDesc,
    },
    Join {
        inputs: Vec<TensorDesc>,
        output: TensorDesc,
        axis: usize,
    },
    Split {
        input: TensorDesc,
        outputs: Vec<TensorDesc>,
        axis: usize,
    },
    Reduce {
        func: u32,
        input: TensorDesc,
        output: TensorDesc,
        axes: Vec<usize>,
    },
    Mvn {
        input: TensorDesc,
        scale: Option<TensorDesc>,
        bias: Option<TensorDesc>,
        output: TensorDesc,
        axes: Vec<usize>,
        normalize_variance: bool,
        epsilon: f32,
        fused: Option<FusedAct>,
    },
    BatchNorm {
        input: TensorDesc,
        mean: TensorDesc,
        variance: TensorDesc,
        scale: TensorDesc,
        bias: TensorDesc,
        output: TensorDesc,
        epsilon: f32,
        fused: Option<FusedAct>,
    },
}

impl OpSpec {
    pub(super) fn input_descs(&self) -> Vec<Option<&TensorDesc>> {
        match self {
            OpSpec::Unary { input, .. }
            | OpSpec::Softmax { input, .. }
            | OpSpec::AvgPool { input, .. }
            | OpSpec::MaxPool { input, .. }
            | OpSpec::Cast { input, .. }
            | OpSpec::Split { input, .. }
            | OpSpec::Reduce { input, .. } => vec![Some(input)],
            OpSpec::Binary { a, b, .. } => vec![Some(a), Some(b)],
            OpSpec::PRelu { input, slope, .. } => vec![Some(input), Some(slope)],
            OpSpec::Gemm { a, b, c, .. } => vec![Some(a), Some(b), c.as_ref()],
            OpSpec::Convolution { input, filter, bias, .. } => vec![Some(input), Some(filter), bias.as_ref()],
            OpSpec::Join { inputs, .. } => inputs.iter().map(Some).collect(),
            OpSpec::Mvn { input, scale, bias, .. } => vec![Some(input), scale.as_ref(), bias.as_ref()],
            OpSpec::BatchNorm { input, mean, variance, scale, bias, .. } => {
                vec![Some(input), Some(mean), Some(variance), Some(scale), Some(bias)]
            }
        }
    }

    pub(super) fn output_descs(&self) -> Vec<Option<&TensorDesc>> {
        match self {
            OpSpec::Unary { output, .. }
            | OpSpec::Binary { output, .. }
            | OpSpec::PRelu { output, .. }
            | OpSpec::Softmax { output, .. }
            | OpSpec::Gemm { output, .. }
            | OpSpec::Convolution { output, .. }
            | OpSpec::AvgPool { output, .. }
            | OpSpec::Cast { output, .. }
            | OpSpec::Join { output, .. }
            | OpSpec::Reduce { output, .. }
            | OpSpec::Mvn { output, .. }
            | OpSpec::BatchNorm { output, .. } => vec![Some(output)],
            OpSpec::MaxPool { output, output_indices, .. } => {
                if let Some(idx) = output_indices {
                    vec![Some(output), Some(idx)]
                } else {
                    vec![Some(output)]
                }
            }
            OpSpec::Split { outputs, .. } => outputs.iter().map(Some).collect(),
        }
    }

    /// Owned-by-DML input layout inside persistent resource: `(input_idx, persistent_offset, byte_len)`.
    pub(super) fn persistent_layout(&self) -> (Vec<(usize, u64, u64)>, u64) {
        let mut layout = Vec::new();
        let mut cursor = 0u64;
        for (i, td) in self.input_descs().into_iter().enumerate() {
            if let Some(t) = td {
                if (t.flags & DML_TENSOR_FLAG_OWNED_BY_DML) != 0 {
                    let aligned = cursor.next_multiple_of(16);
                    layout.push((i, aligned, t.total_bytes));
                    cursor = aligned.saturating_add(t.total_bytes);
                }
            }
        }
        (layout, cursor)
    }

    pub(super) unsafe fn parse(desc_ptr: *const DML_OPERATOR_DESC) -> Result<Self, HRESULT> {
        if desc_ptr.is_null() {
            return Err(E_INVALIDARG);
        }
        let od = rd(desc_ptr);
        if od.Desc.is_null() {
            return Err(E_INVALIDARG);
        }

        let read_sb = |p: *const DML_SCALE_BIAS| -> Option<(f32, f32)> {
            if p.is_null() {
                None
            } else {
                let sb = rd(p);
                Some((sb.Scale, sb.Bias))
            }
        };
        let read_u32_vec = |p: *const u32, n: usize, default_val: u32| -> Result<Vec<u32>, HRESULT> {
            if n == 0 {
                return Ok(Vec::new());
            }
            if p.is_null() {
                return Ok(vec![default_val; n]);
            }
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                v.push(rd(p.add(i)));
            }
            Ok(v)
        };

        match od.Type {
            DML_OPERATOR_ELEMENT_WISE_IDENTITY
            | DML_OPERATOR_ELEMENT_WISE_ABS
            | DML_OPERATOR_ELEMENT_WISE_ACOS
            | DML_OPERATOR_ELEMENT_WISE_ASIN
            | DML_OPERATOR_ELEMENT_WISE_ATAN
            | DML_OPERATOR_ELEMENT_WISE_CEIL
            | DML_OPERATOR_ELEMENT_WISE_COS
            | DML_OPERATOR_ELEMENT_WISE_EXP
            | DML_OPERATOR_ELEMENT_WISE_FLOOR
            | DML_OPERATOR_ELEMENT_WISE_LOG
            | DML_OPERATOR_ELEMENT_WISE_RECIP
            | DML_OPERATOR_ELEMENT_WISE_SIN
            | DML_OPERATOR_ELEMENT_WISE_SQRT
            | DML_OPERATOR_ELEMENT_WISE_TAN
            | DML_OPERATOR_ELEMENT_WISE_SIGN
            | DML_OPERATOR_ELEMENT_WISE_TANH
            | DML_OPERATOR_ELEMENT_WISE_ROUND => {
                let d = rd(od.Desc as *const DML_ELEMENT_WISE_UNARY_DESC);
                let op = match od.Type {
                    DML_OPERATOR_ELEMENT_WISE_IDENTITY => UnaryOp::Identity,
                    DML_OPERATOR_ELEMENT_WISE_ABS => UnaryOp::Abs,
                    DML_OPERATOR_ELEMENT_WISE_ACOS => UnaryOp::Acos,
                    DML_OPERATOR_ELEMENT_WISE_ASIN => UnaryOp::Asin,
                    DML_OPERATOR_ELEMENT_WISE_ATAN => UnaryOp::Atan,
                    DML_OPERATOR_ELEMENT_WISE_CEIL => UnaryOp::Ceil,
                    DML_OPERATOR_ELEMENT_WISE_COS => UnaryOp::Cos,
                    DML_OPERATOR_ELEMENT_WISE_EXP => UnaryOp::Exp,
                    DML_OPERATOR_ELEMENT_WISE_FLOOR => UnaryOp::Floor,
                    DML_OPERATOR_ELEMENT_WISE_LOG => UnaryOp::Log,
                    DML_OPERATOR_ELEMENT_WISE_RECIP => UnaryOp::Recip,
                    DML_OPERATOR_ELEMENT_WISE_SIN => UnaryOp::Sin,
                    DML_OPERATOR_ELEMENT_WISE_SQRT => UnaryOp::Sqrt,
                    DML_OPERATOR_ELEMENT_WISE_TAN => UnaryOp::Tan,
                    DML_OPERATOR_ELEMENT_WISE_SIGN => UnaryOp::Sign,
                    DML_OPERATOR_ELEMENT_WISE_TANH => UnaryOp::Activation(FusedAct::Tanh),
                    DML_OPERATOR_ELEMENT_WISE_ROUND => UnaryOp::Round,
                    _ => unreachable!(),
                };
                Ok(OpSpec::Unary {
                    op,
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: read_sb(d.ScaleBias),
                })
            }
            DML_OPERATOR_ELEMENT_WISE_NEGATE => {
                let d = rd(od.Desc as *const DML_SIMPLE_UNARY_DESC);
                Ok(OpSpec::Unary {
                    op: UnaryOp::Neg,
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: None,
                })
            }
            DML_OPERATOR_ELEMENT_WISE_CLIP => {
                let d = rd(od.Desc as *const DML_ELEMENT_WISE_CLIP_OPERATOR_DESC);
                Ok(OpSpec::Unary {
                    op: UnaryOp::Clip(d.Min, d.Max),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: read_sb(d.ScaleBias),
                })
            }
            DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW => {
                let d = rd(od.Desc as *const DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC);
                Ok(OpSpec::Unary {
                    op: UnaryOp::ConstPow(d.Exponent),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: read_sb(d.ScaleBias),
                })
            }
            DML_OPERATOR_ELEMENT_WISE_THRESHOLD => {
                let d = rd(od.Desc as *const DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC);
                Ok(OpSpec::Unary {
                    op: UnaryOp::Threshold(d.Min),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: read_sb(d.ScaleBias),
                })
            }
            DML_OPERATOR_ELEMENT_WISE_ADD
            | DML_OPERATOR_ELEMENT_WISE_SUBTRACT
            | DML_OPERATOR_ELEMENT_WISE_MULTIPLY
            | DML_OPERATOR_ELEMENT_WISE_DIVIDE
            | DML_OPERATOR_ELEMENT_WISE_MIN
            | DML_OPERATOR_ELEMENT_WISE_MAX
            | DML_OPERATOR_ELEMENT_WISE_MEAN => {
                let d = rd(od.Desc as *const DML_ELEMENT_WISE_BINARY_DESC);
                let op = match od.Type {
                    DML_OPERATOR_ELEMENT_WISE_ADD => BinaryOp::Add,
                    DML_OPERATOR_ELEMENT_WISE_SUBTRACT => BinaryOp::Sub,
                    DML_OPERATOR_ELEMENT_WISE_MULTIPLY => BinaryOp::Mul,
                    DML_OPERATOR_ELEMENT_WISE_DIVIDE => BinaryOp::Div,
                    DML_OPERATOR_ELEMENT_WISE_MIN => BinaryOp::Min,
                    DML_OPERATOR_ELEMENT_WISE_MAX => BinaryOp::Max,
                    DML_OPERATOR_ELEMENT_WISE_MEAN => BinaryOp::Mean,
                    _ => unreachable!(),
                };
                Ok(OpSpec::Binary {
                    op,
                    a: TensorDesc::parse(d.ATensor)?,
                    b: TensorDesc::parse(d.BTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: None,
                    fused: None,
                })
            }
            DML_OPERATOR_ELEMENT_WISE_ADD1 => {
                let d = rd(od.Desc as *const DML_ELEMENT_WISE_ADD1_OPERATOR_DESC);
                Ok(OpSpec::Binary {
                    op: BinaryOp::Add,
                    a: TensorDesc::parse(d.ATensor)?,
                    b: TensorDesc::parse(d.BTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: None,
                    fused: FusedAct::parse_opt(d.FusedActivation)?,
                })
            }
            DML_OPERATOR_ELEMENT_WISE_POW => {
                let d = rd(od.Desc as *const DML_ELEMENT_WISE_POW_OPERATOR_DESC);
                Ok(OpSpec::Binary {
                    op: BinaryOp::Pow,
                    a: TensorDesc::parse(d.InputTensor)?,
                    b: TensorDesc::parse(d.ExponentTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: read_sb(d.ScaleBias),
                    fused: None,
                })
            }
            DML_OPERATOR_ACTIVATION_IDENTITY
            | DML_OPERATOR_ACTIVATION_RELU
            | DML_OPERATOR_ACTIVATION_SIGMOID
            | DML_OPERATOR_ACTIVATION_TANH
            | DML_OPERATOR_ACTIVATION_SOFTSIGN
            | DML_OPERATOR_ACTIVATION_GELU => {
                let d = rd(od.Desc as *const DML_SIMPLE_UNARY_DESC);
                let act = match od.Type {
                    DML_OPERATOR_ACTIVATION_IDENTITY => FusedAct::Identity,
                    DML_OPERATOR_ACTIVATION_RELU => FusedAct::Relu,
                    DML_OPERATOR_ACTIVATION_SIGMOID => FusedAct::Sigmoid,
                    DML_OPERATOR_ACTIVATION_TANH => FusedAct::Tanh,
                    DML_OPERATOR_ACTIVATION_SOFTSIGN => FusedAct::Softsign,
                    DML_OPERATOR_ACTIVATION_GELU => FusedAct::Gelu,
                    _ => unreachable!(),
                };
                Ok(OpSpec::Unary {
                    op: UnaryOp::Activation(act),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: None,
                })
            }
            DML_OPERATOR_ACTIVATION_LEAKY_RELU
            | DML_OPERATOR_ACTIVATION_ELU
            | DML_OPERATOR_ACTIVATION_SOFTPLUS
            | DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU
            | DML_OPERATOR_ACTIVATION_CELU => {
                let d = rd(od.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                let act = match od.Type {
                    DML_OPERATOR_ACTIVATION_LEAKY_RELU => FusedAct::LeakyRelu(d.Alpha),
                    DML_OPERATOR_ACTIVATION_ELU => FusedAct::Elu(d.Alpha),
                    DML_OPERATOR_ACTIVATION_SOFTPLUS => FusedAct::Softplus(d.Alpha),
                    DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU => FusedAct::ThresholdedRelu(d.Alpha),
                    DML_OPERATOR_ACTIVATION_CELU => FusedAct::Celu(d.Alpha),
                    _ => unreachable!(),
                };
                Ok(OpSpec::Unary {
                    op: UnaryOp::Activation(act),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: None,
                })
            }
            DML_OPERATOR_ACTIVATION_LINEAR
            | DML_OPERATOR_ACTIVATION_HARD_SIGMOID
            | DML_OPERATOR_ACTIVATION_SCALED_ELU
            | DML_OPERATOR_ACTIVATION_SCALED_TANH => {
                let d = rd(od.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                let act = match od.Type {
                    DML_OPERATOR_ACTIVATION_LINEAR => FusedAct::Linear(d.Alpha, d.Beta),
                    DML_OPERATOR_ACTIVATION_HARD_SIGMOID => FusedAct::HardSigmoid(d.Alpha, d.Beta),
                    DML_OPERATOR_ACTIVATION_SCALED_ELU => FusedAct::ScaledElu(d.Alpha, d.Beta),
                    DML_OPERATOR_ACTIVATION_SCALED_TANH => FusedAct::ScaledTanh(d.Alpha, d.Beta),
                    _ => unreachable!(),
                };
                Ok(OpSpec::Unary {
                    op: UnaryOp::Activation(act),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: None,
                })
            }
            DML_OPERATOR_ACTIVATION_PARAMETERIZED_RELU => {
                let d = rd(od.Desc as *const DML_ACTIVATION_PRELU_DESC);
                Ok(OpSpec::PRelu {
                    input: TensorDesc::parse(d.InputTensor)?,
                    slope: TensorDesc::parse(d.SlopeTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                })
            }
            DML_OPERATOR_ACTIVATION_SOFTMAX | DML_OPERATOR_ACTIVATION_LOG_SOFTMAX => {
                let d = rd(od.Desc as *const DML_SIMPLE_UNARY_DESC);
                Ok(OpSpec::Softmax {
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    log_mode: od.Type == DML_OPERATOR_ACTIVATION_LOG_SOFTMAX,
                })
            }
            DML_OPERATOR_GEMM => {
                let d = rd(od.Desc as *const DML_GEMM_OPERATOR_DESC);
                let a = TensorDesc::parse(d.ATensor)?;
                let b = TensorDesc::parse(d.BTensor)?;
                let c = TensorDesc::parse_opt(d.CTensor)?;
                let output = TensorDesc::parse(d.OutputTensor)?;
                if a.sizes.len() < 2 || b.sizes.len() < 2 || output.sizes.len() < 2 {
                    return Err(E_INVALIDARG);
                }
                Ok(OpSpec::Gemm {
                    a,
                    b,
                    c,
                    output,
                    trans_a: d.TransA == DML_MATRIX_TRANSFORM_TRANSPOSE,
                    trans_b: d.TransB == DML_MATRIX_TRANSFORM_TRANSPOSE,
                    alpha: d.Alpha,
                    beta: d.Beta,
                    fused: FusedAct::parse_opt(d.FusedActivation)?,
                })
            }
            DML_OPERATOR_CONVOLUTION => {
                let d = rd(od.Desc as *const DML_CONVOLUTION_OPERATOR_DESC);
                let input = TensorDesc::parse(d.InputTensor)?;
                let filter = TensorDesc::parse(d.FilterTensor)?;
                let bias = TensorDesc::parse_opt(d.BiasTensor)?;
                let output = TensorDesc::parse(d.OutputTensor)?;
                let sdim = d.DimensionCount as usize;
                if sdim == 0 || sdim > 3 || input.sizes.len() != sdim + 2 || filter.sizes.len() != sdim + 2 || output.sizes.len() != sdim + 2 {
                    return Err(E_INVALIDARG);
                }
                let strides = read_u32_vec(d.Strides, sdim, 1)?;
                let dilations = read_u32_vec(d.Dilations, sdim, 1)?;
                let start_pad = read_u32_vec(d.StartPadding, sdim, 0)?;
                let _ = read_u32_vec(d.EndPadding, sdim, 0)?;
                if strides.iter().any(|&s| s == 0) || dilations.iter().any(|&s| s == 0) || d.GroupCount == 0 {
                    return Err(E_INVALIDARG);
                }
                Ok(OpSpec::Convolution {
                    input,
                    filter,
                    bias,
                    output,
                    mode: d.Mode,
                    direction: d.Direction,
                    strides,
                    dilations,
                    start_pad,
                    group_count: d.GroupCount,
                    fused: FusedAct::parse_opt(d.FusedActivation)?,
                })
            }
            DML_OPERATOR_AVERAGE_POOLING => {
                let d = rd(od.Desc as *const DML_AVERAGE_POOLING_OPERATOR_DESC);
                let input = TensorDesc::parse(d.InputTensor)?;
                let output = TensorDesc::parse(d.OutputTensor)?;
                let sdim = d.DimensionCount as usize;
                if sdim == 0 || sdim > 3 || input.sizes.len() < sdim + 2 {
                    return Err(E_INVALIDARG);
                }
                let _ = read_u32_vec(d.EndPadding, sdim, 0)?;
                Ok(OpSpec::AvgPool {
                    input,
                    output,
                    strides: read_u32_vec(d.Strides, sdim, 1)?,
                    window: read_u32_vec(d.WindowSize, sdim, 1)?,
                    start_pad: read_u32_vec(d.StartPadding, sdim, 0)?,
                    include_padding: d.IncludePadding != 0,
                })
            }
            DML_OPERATOR_MAX_POOLING | DML_OPERATOR_MAX_POOLING1 | DML_OPERATOR_MAX_POOLING2 => {
                let (in_p, out_p, idx_p, dim_c, st_p, win_p, sp_p, ep_p, dil_p) = match od.Type {
                    DML_OPERATOR_MAX_POOLING => {
                        let d = rd(od.Desc as *const DML_MAX_POOLING_OPERATOR_DESC);
                        (d.InputTensor, d.OutputTensor, std::ptr::null(), d.DimensionCount, d.Strides, d.WindowSize, d.StartPadding, d.EndPadding, std::ptr::null())
                    }
                    DML_OPERATOR_MAX_POOLING1 => {
                        let d = rd(od.Desc as *const DML_MAX_POOLING1_OPERATOR_DESC);
                        (d.InputTensor, d.OutputTensor, d.OutputIndicesTensor, d.DimensionCount, d.Strides, d.WindowSize, d.StartPadding, d.EndPadding, std::ptr::null())
                    }
                    _ => {
                        let d = rd(od.Desc as *const DML_MAX_POOLING2_OPERATOR_DESC);
                        (d.InputTensor, d.OutputTensor, d.OutputIndicesTensor, d.DimensionCount, d.Strides, d.WindowSize, d.StartPadding, d.EndPadding, d.Dilations)
                    }
                };
                let input = TensorDesc::parse(in_p)?;
                let output = TensorDesc::parse(out_p)?;
                let output_indices = TensorDesc::parse_opt(idx_p)?;
                let sdim = dim_c as usize;
                if sdim == 0 || sdim > 3 || input.sizes.len() < sdim + 2 {
                    return Err(E_INVALIDARG);
                }
                let _ = read_u32_vec(ep_p, sdim, 0)?;
                Ok(OpSpec::MaxPool {
                    input,
                    output,
                    output_indices,
                    strides: read_u32_vec(st_p, sdim, 1)?,
                    window: read_u32_vec(win_p, sdim, 1)?,
                    start_pad: read_u32_vec(sp_p, sdim, 0)?,
                    dilations: read_u32_vec(dil_p, sdim, 1)?,
                })
            }
            DML_OPERATOR_CAST => {
                let d = rd(od.Desc as *const DML_SIMPLE_UNARY_DESC);
                Ok(OpSpec::Cast {
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                })
            }
            DML_OPERATOR_JOIN => {
                let d = rd(od.Desc as *const DML_JOIN_OPERATOR_DESC);
                if d.InputCount == 0 || d.InputTensors.is_null() {
                    return Err(E_INVALIDARG);
                }
                let output = TensorDesc::parse(d.OutputTensor)?;
                if (d.Axis as usize) >= output.sizes.len() {
                    return Err(E_INVALIDARG);
                }
                let mut inputs = Vec::with_capacity(d.InputCount as usize);
                for i in 0..d.InputCount as usize {
                    let td = TensorDesc::parse(d.InputTensors.add(i))?;
                    if td.sizes.len() != output.sizes.len() {
                        return Err(E_INVALIDARG);
                    }
                    inputs.push(td);
                }
                Ok(OpSpec::Join {
                    inputs,
                    output,
                    axis: d.Axis as usize,
                })
            }
            DML_OPERATOR_SPLIT => {
                let d = rd(od.Desc as *const DML_SPLIT_OPERATOR_DESC);
                if d.OutputCount == 0 || d.OutputTensors.is_null() {
                    return Err(E_INVALIDARG);
                }
                let input = TensorDesc::parse(d.InputTensor)?;
                if (d.Axis as usize) >= input.sizes.len() {
                    return Err(E_INVALIDARG);
                }
                let mut outputs = Vec::with_capacity(d.OutputCount as usize);
                for i in 0..d.OutputCount as usize {
                    let td = TensorDesc::parse(d.OutputTensors.add(i))?;
                    if td.sizes.len() != input.sizes.len() {
                        return Err(E_INVALIDARG);
                    }
                    outputs.push(td);
                }
                Ok(OpSpec::Split {
                    input,
                    outputs,
                    axis: d.Axis as usize,
                })
            }
            DML_OPERATOR_REDUCE => {
                let d = rd(od.Desc as *const DML_REDUCE_OPERATOR_DESC);
                let input = TensorDesc::parse(d.InputTensor)?;
                let output = TensorDesc::parse(d.OutputTensor)?;
                if d.AxisCount == 0 || d.Axes.is_null() || output.sizes.len() != input.sizes.len() {
                    return Err(E_INVALIDARG);
                }
                let mut axes = Vec::with_capacity(d.AxisCount as usize);
                for i in 0..d.AxisCount as usize {
                    let a = rd(d.Axes.add(i)) as usize;
                    if a >= input.sizes.len() {
                        return Err(E_INVALIDARG);
                    }
                    axes.push(a);
                }
                Ok(OpSpec::Reduce {
                    func: d.Function,
                    input,
                    output,
                    axes,
                })
            }
            DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION => {
                let d = rd(od.Desc as *const DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC);
                let input = TensorDesc::parse(d.InputTensor)?;
                let rank = input.sizes.len();
                let axes = if d.CrossChannel != 0 {
                    (1..rank).collect()
                } else if rank >= 2 {
                    (2..rank).collect()
                } else {
                    vec![0]
                };
                Ok(OpSpec::Mvn {
                    input,
                    scale: TensorDesc::parse_opt(d.ScaleTensor)?,
                    bias: TensorDesc::parse_opt(d.BiasTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    axes,
                    normalize_variance: d.NormalizeVariance != 0,
                    epsilon: d.Epsilon,
                    fused: FusedAct::parse_opt(d.FusedActivation)?,
                })
            }
            DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION1 => {
                let d = rd(od.Desc as *const DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC);
                let input = TensorDesc::parse(d.InputTensor)?;
                let mut axes = Vec::with_capacity(d.AxisCount as usize);
                if d.AxisCount > 0 && !d.Axes.is_null() {
                    for i in 0..d.AxisCount as usize {
                        let a = rd(d.Axes.add(i)) as usize;
                        if a >= input.sizes.len() {
                            return Err(E_INVALIDARG);
                        }
                        axes.push(a);
                    }
                }
                Ok(OpSpec::Mvn {
                    input,
                    scale: TensorDesc::parse_opt(d.ScaleTensor)?,
                    bias: TensorDesc::parse_opt(d.BiasTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    axes,
                    normalize_variance: d.NormalizeVariance != 0,
                    epsilon: d.Epsilon,
                    fused: FusedAct::parse_opt(d.FusedActivation)?,
                })
            }
            DML_OPERATOR_BATCH_NORMALIZATION => {
                let d = rd(od.Desc as *const DML_BATCH_NORMALIZATION_OPERATOR_DESC);
                Ok(OpSpec::BatchNorm {
                    input: TensorDesc::parse(d.InputTensor)?,
                    mean: TensorDesc::parse(d.MeanTensor)?,
                    variance: TensorDesc::parse(d.VarianceTensor)?,
                    scale: TensorDesc::parse(d.ScaleTensor)?,
                    bias: TensorDesc::parse(d.BiasTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    epsilon: d.Epsilon,
                    fused: FusedAct::parse_opt(d.FusedActivation)?,
                })
            }
            other => {
                stub!("DirectML CreateOperator: unsupported operator type {other}");
                Err(E_NOTIMPL)
            }
        }
    }
}

// ============================================================================
// CPU Tensor Execution Engine
// ============================================================================

pub(super) fn execute_op(op: &OpSpec, inputs: &[Option<&[u8]>], outputs: &mut [Option<&mut [u8]>]) {
    match op {
        OpSpec::Unary { op, input, output, scale_bias } => {
            let (Some(in_b), Some(out_b)) = (inputs.first().and_then(|x| *x), outputs.first_mut().and_then(|x| x.as_deref_mut())) else {
                return;
            };
            let rank = output.sizes.len();
            let mut coords = vec![0usize; rank];
            let n = output.elem_count();
            for flat in 0..n {
                output.linear_to_coords(flat, &mut coords);
                let mut x = input.read_f64(in_b, &coords);
                if let Some((sc, bi)) = scale_bias {
                    x = x * (*sc as f64) + (*bi as f64);
                }
                let y = match *op {
                    UnaryOp::Identity => x,
                    UnaryOp::Abs => x.abs(),
                    UnaryOp::Exp => x.exp(),
                    UnaryOp::Log => x.ln(),
                    UnaryOp::Sqrt => x.sqrt(),
                    UnaryOp::Neg => -x,
                    UnaryOp::Clip(lo, hi) => x.clamp(lo as f64, hi as f64),
                    UnaryOp::Recip => 1.0 / x,
                    UnaryOp::Floor => x.floor(),
                    UnaryOp::Ceil => x.ceil(),
                    UnaryOp::Round => x.round_ties_even(),
                    UnaryOp::Sin => x.sin(),
                    UnaryOp::Cos => x.cos(),
                    UnaryOp::Tan => x.tan(),
                    UnaryOp::Asin => x.asin(),
                    UnaryOp::Acos => x.acos(),
                    UnaryOp::Atan => x.atan(),
                    UnaryOp::Sign => if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { 0.0 },
                    UnaryOp::ConstPow(e) => x.powf(e as f64),
                    UnaryOp::Threshold(m) => if x > m as f64 { x } else { m as f64 },
                    UnaryOp::Activation(a) => a.apply(x),
                };
                output.write_f64(out_b, &coords, y);
            }
        }
        OpSpec::Binary { op, a, b, output, scale_bias, fused } => {
            let (Some(ab), Some(bb), Some(out_b)) = (
                inputs.first().and_then(|x| *x),
                inputs.get(1).and_then(|x| *x),
                outputs.first_mut().and_then(|x| x.as_deref_mut()),
            ) else {
                return;
            };
            let rank = output.sizes.len();
            let mut coords = vec![0usize; rank];
            let n = output.elem_count();
            for flat in 0..n {
                output.linear_to_coords(flat, &mut coords);
                let mut av = a.read_f64(ab, &coords);
                if let Some((sc, bi)) = scale_bias {
                    av = av * (*sc as f64) + (*bi as f64);
                }
                let bv = b.read_f64(bb, &coords);
                let mut y = match *op {
                    BinaryOp::Add => av + bv,
                    BinaryOp::Sub => av - bv,
                    BinaryOp::Mul => av * bv,
                    BinaryOp::Div => av / bv,
                    BinaryOp::Min => av.min(bv),
                    BinaryOp::Max => av.max(bv),
                    BinaryOp::Mean => 0.5 * (av + bv),
                    BinaryOp::Pow => av.powf(bv),
                };
                if let Some(act) = fused {
                    y = act.apply(y);
                }
                output.write_f64(out_b, &coords, y);
            }
        }
        OpSpec::PRelu { input, slope, output } => {
            let (Some(in_b), Some(sl_b), Some(out_b)) = (
                inputs.first().and_then(|x| *x),
                inputs.get(1).and_then(|x| *x),
                outputs.first_mut().and_then(|x| x.as_deref_mut()),
            ) else {
                return;
            };
            let mut coords = vec![0usize; output.sizes.len()];
            for flat in 0..output.elem_count() {
                output.linear_to_coords(flat, &mut coords);
                let x = input.read_f64(in_b, &coords);
                let s = slope.read_f64(sl_b, &coords);
                output.write_f64(out_b, &coords, if x >= 0.0 { x } else { s * x });
            }
        }
        OpSpec::Softmax { input, output, log_mode } => {
            let (Some(in_b), Some(out_b)) = (inputs.first().and_then(|x| *x), outputs.first_mut().and_then(|x| x.as_deref_mut())) else {
                return;
            };
            let rank = input.sizes.len();
            let last = input.sizes[rank - 1] as usize;
            let outer = input.elem_count() / last.max(1);
            let mut coords = vec![0usize; rank];
            for o in 0..outer {
                let mut tmp = o;
                for d in (0..rank - 1).rev() {
                    let sz = input.sizes[d] as usize;
                    coords[d] = tmp % sz;
                    tmp /= sz;
                }
                let mut max_v = f64::NEG_INFINITY;
                for j in 0..last {
                    coords[rank - 1] = j;
                    max_v = max_v.max(input.read_f64(in_b, &coords));
                }
                let mut sum_exp = 0.0f64;
                let mut vals = Vec::with_capacity(last);
                for j in 0..last {
                    coords[rank - 1] = j;
                    let shifted = input.read_f64(in_b, &coords) - max_v;
                    let e = shifted.exp();
                    sum_exp += e;
                    vals.push((shifted, e));
                }
                let log_sum = sum_exp.ln();
                for (j, (shifted, e)) in vals.into_iter().enumerate() {
                    coords[rank - 1] = j;
                    let y = if *log_mode { shifted - log_sum } else { e / sum_exp };
                    output.write_f64(out_b, &coords, y);
                }
            }
        }
        OpSpec::Gemm { a, b, c, output, trans_a, trans_b, alpha, beta, fused } => {
            let (Some(ab), Some(bb), Some(out_b)) = (
                inputs.first().and_then(|x| *x),
                inputs.get(1).and_then(|x| *x),
                outputs.first_mut().and_then(|x| x.as_deref_mut()),
            ) else {
                return;
            };
            let cb = inputs.get(2).and_then(|x| *x);
            let out_rank = output.sizes.len();
            let m = output.sizes[out_rank - 2] as usize;
            let n = output.sizes[out_rank - 1] as usize;
            let a_rank = a.sizes.len();
            let k = if *trans_a { a.sizes[a_rank - 2] as usize } else { a.sizes[a_rank - 1] as usize };
            let batch_dims = &output.sizes[..out_rank - 2];
            let batch_total = batch_dims.iter().fold(1usize, |acc, &v| acc * v as usize);
            let mut batch_coords = vec![0usize; batch_dims.len()];
            let mut a_coords = vec![0usize; a_rank];
            let mut b_coords = vec![0usize; b.sizes.len()];
            let mut out_coords = vec![0usize; out_rank];

            for bi in 0..batch_total {
                let mut tmp = bi;
                for d in (0..batch_dims.len()).rev() {
                    let sz = batch_dims[d] as usize;
                    batch_coords[d] = tmp % sz;
                    tmp /= sz;
                }
                out_coords[..batch_dims.len()].copy_from_slice(&batch_coords);
                let copy_batch = |dst: &mut [usize], dst_rank: usize| {
                    let dst_b = dst_rank - 2;
                    for d in 0..dst_b {
                        let src_d = batch_dims.len().saturating_sub(dst_b) + d;
                        dst[d] = if src_d < batch_dims.len() { batch_coords[src_d] } else { 0 };
                    }
                };
                copy_batch(&mut a_coords, a_rank);
                copy_batch(&mut b_coords, b.sizes.len());

                for row in 0..m {
                    out_coords[out_rank - 2] = row;
                    for col in 0..n {
                        out_coords[out_rank - 1] = col;
                        let mut acc = 0.0f64;
                        for kk in 0..k {
                            if *trans_a {
                                a_coords[a_rank - 2] = kk;
                                a_coords[a_rank - 1] = row;
                            } else {
                                a_coords[a_rank - 2] = row;
                                a_coords[a_rank - 1] = kk;
                            }
                            let b_rank = b.sizes.len();
                            if *trans_b {
                                b_coords[b_rank - 2] = col;
                                b_coords[b_rank - 1] = kk;
                            } else {
                                b_coords[b_rank - 2] = kk;
                                b_coords[b_rank - 1] = col;
                            }
                            acc += a.read_f64(ab, &a_coords) * b.read_f64(bb, &b_coords);
                        }
                        let mut y = (*alpha as f64) * acc;
                        if let (Some(c_desc), Some(c_buf)) = (c.as_ref(), cb) {
                            if *beta != 0.0 {
                                y += (*beta as f64) * c_desc.read_f64(c_buf, &out_coords);
                            }
                        }
                        if let Some(act) = fused {
                            y = act.apply(y);
                        }
                        output.write_f64(out_b, &out_coords, y);
                    }
                }
            }
        }
        OpSpec::Convolution {
            input,
            filter,
            bias,
            output,
            mode,
            direction,
            strides,
            dilations,
            start_pad,
            group_count,
            fused,
        } => {
            let (Some(in_b), Some(flt_b), Some(out_b)) = (
                inputs.first().and_then(|x| *x),
                inputs.get(1).and_then(|x| *x),
                outputs.first_mut().and_then(|x| x.as_deref_mut()),
            ) else {
                return;
            };
            let bias_b = inputs.get(2).and_then(|x| *x);
            let sdim = strides.len();
            let batch = output.sizes[0] as usize;
            let c_out = output.sizes[1] as usize;
            let c_in = input.sizes[1] as usize;
            let groups = (*group_count as usize).max(1);
            let c_in_g = (c_in / groups).max(1);
            let c_out_g = (c_out / groups).max(1);
            let flip_kernel = *mode == DML_CONVOLUTION_MODE_CONVOLUTION;

            let out_spatial: Vec<usize> = output.sizes[2..2 + sdim].iter().map(|&v| v as usize).collect();
            let in_spatial: Vec<usize> = input.sizes[2..2 + sdim].iter().map(|&v| v as usize).collect();
            let k_spatial: Vec<usize> = filter.sizes[2..2 + sdim].iter().map(|&v| v as usize).collect();
            let out_sp_total = out_spatial.iter().product::<usize>();
            let k_sp_total = k_spatial.iter().product::<usize>();

            let mut out_coords = vec![0usize; sdim + 2];
            let mut in_coords = vec![0usize; sdim + 2];
            let mut flt_coords = vec![0usize; sdim + 2];
            let mut k_idx = vec![0usize; sdim];

            for n in 0..batch {
                out_coords[0] = n;
                in_coords[0] = n;
                for oc in 0..c_out {
                    out_coords[1] = oc;
                    let g = oc / c_out_g;
                    let oc_in_g = oc % c_out_g;
                    for osp in 0..out_sp_total {
                        let mut tmp = osp;
                        for d in (0..sdim).rev() {
                            out_coords[2 + d] = tmp % out_spatial[d];
                            tmp /= out_spatial[d];
                        }
                        let mut acc = match (bias.as_ref(), bias_b) {
                            (Some(bd), Some(bb)) => bd.read_f64(bb, &out_coords),
                            _ => 0.0,
                        };
                        for ic_off in 0..c_in_g {
                            let ic = g * c_in_g + ic_off;
                            in_coords[1] = ic;
                            if *direction == DML_CONVOLUTION_DIRECTION_FORWARD {
                                flt_coords[0] = oc;
                                flt_coords[1] = ic_off;
                            } else {
                                flt_coords[0] = ic;
                                flt_coords[1] = oc_in_g;
                            }
                            for ksp in 0..k_sp_total {
                                let mut kt = ksp;
                                for d in (0..sdim).rev() {
                                    k_idx[d] = kt % k_spatial[d];
                                    kt /= k_spatial[d];
                                }
                                let mut in_bounds = true;
                                for d in 0..sdim {
                                    let kd = if flip_kernel { k_spatial[d] - 1 - k_idx[d] } else { k_idx[d] };
                                    flt_coords[2 + d] = kd;
                                    if *direction == DML_CONVOLUTION_DIRECTION_FORWARD {
                                        let pos = (out_coords[2 + d] as isize) * (strides[d] as isize)
                                            + (k_idx[d] as isize) * (dilations[d] as isize)
                                            - (start_pad[d] as isize);
                                        if pos < 0 || pos >= in_spatial[d] as isize {
                                            in_bounds = false;
                                            break;
                                        }
                                        in_coords[2 + d] = pos as usize;
                                    } else {
                                        let num = (out_coords[2 + d] as isize) + (start_pad[d] as isize)
                                            - (k_idx[d] as isize) * (dilations[d] as isize);
                                        let st = strides[d] as isize;
                                        if num < 0 || num % st != 0 {
                                            in_bounds = false;
                                            break;
                                        }
                                        let pos = num / st;
                                        if pos >= in_spatial[d] as isize {
                                            in_bounds = false;
                                            break;
                                        }
                                        in_coords[2 + d] = pos as usize;
                                    }
                                }
                                if in_bounds {
                                    acc += input.read_f64(in_b, &in_coords) * filter.read_f64(flt_b, &flt_coords);
                                }
                            }
                        }
                        if let Some(act) = fused {
                            acc = act.apply(acc);
                        }
                        output.write_f64(out_b, &out_coords, acc);
                    }
                }
            }
        }
        OpSpec::AvgPool { input, output, strides, window, start_pad, include_padding } => {
            let (Some(in_b), Some(out_b)) = (inputs.first().and_then(|x| *x), outputs.first_mut().and_then(|x| x.as_deref_mut())) else {
                return;
            };
            let sdim = strides.len();
            let rank = output.sizes.len();
            let lead = rank - sdim;
            let lead_total = output.sizes[..lead].iter().fold(1usize, |a, &b| a * b as usize);
            let out_sp: Vec<usize> = output.sizes[lead..].iter().map(|&v| v as usize).collect();
            let in_sp: Vec<usize> = input.sizes[lead..].iter().map(|&v| v as usize).collect();
            let out_sp_total = out_sp.iter().product::<usize>();
            let win_total = window.iter().fold(1usize, |a, &b| a * b as usize);
            let mut out_coords = vec![0usize; rank];
            let mut in_coords = vec![0usize; rank];

            for l in 0..lead_total {
                let mut tmp = l;
                for d in (0..lead).rev() {
                    let sz = output.sizes[d] as usize;
                    out_coords[d] = tmp % sz;
                    in_coords[d] = out_coords[d];
                    tmp /= sz;
                }
                for osp in 0..out_sp_total {
                    let mut tmp = osp;
                    for d in (0..sdim).rev() {
                        out_coords[lead + d] = tmp % out_sp[d];
                        tmp /= out_sp[d];
                    }
                    let mut sum = 0.0f64;
                    let mut cnt = 0usize;
                    for wi in 0..win_total {
                        let mut wt = wi;
                        let mut ok = true;
                        for d in (0..sdim).rev() {
                            let wv = wt % (window[d] as usize);
                            wt /= window[d] as usize;
                            let pos = (out_coords[lead + d] as isize) * (strides[d] as isize)
                                + (wv as isize)
                                - (start_pad[d] as isize);
                            if pos < 0 || pos >= in_sp[d] as isize {
                                ok = false;
                            } else {
                                in_coords[lead + d] = pos as usize;
                            }
                        }
                        if ok {
                            sum += input.read_f64(in_b, &in_coords);
                            cnt += 1;
                        }
                    }
                    let denom = if *include_padding { win_total } else { cnt.max(1) } as f64;
                    output.write_f64(out_b, &out_coords, sum / denom);
                }
            }
        }
        OpSpec::MaxPool { input, output, output_indices, strides, window, start_pad, dilations } => {
            let Some(in_b) = inputs.first().and_then(|x| *x) else { return };
            let (out_first, out_rest) = match outputs.split_first_mut() {
                Some(x) => x,
                None => return,
            };
            let Some(out_b) = out_first.as_deref_mut() else { return };
            let mut idx_b = out_rest.first_mut().and_then(|x| x.as_deref_mut());

            let sdim = strides.len();
            let rank = output.sizes.len();
            let lead = rank - sdim;
            let lead_total = output.sizes[..lead].iter().fold(1usize, |a, &b| a * b as usize);
            let out_sp: Vec<usize> = output.sizes[lead..].iter().map(|&v| v as usize).collect();
            let in_sp: Vec<usize> = input.sizes[lead..].iter().map(|&v| v as usize).collect();
            let out_sp_total = out_sp.iter().product::<usize>();
            let win_total = window.iter().fold(1usize, |a, &b| a * b as usize);
            let mut out_coords = vec![0usize; rank];
            let mut in_coords = vec![0usize; rank];

            for l in 0..lead_total {
                let mut tmp = l;
                for d in (0..lead).rev() {
                    let sz = output.sizes[d] as usize;
                    out_coords[d] = tmp % sz;
                    in_coords[d] = out_coords[d];
                    tmp /= sz;
                }
                for osp in 0..out_sp_total {
                    let mut tmp = osp;
                    for d in (0..sdim).rev() {
                        out_coords[lead + d] = tmp % out_sp[d];
                        tmp /= out_sp[d];
                    }
                    let mut best = f64::NEG_INFINITY;
                    let mut best_flat = 0usize;
                    let mut found = false;
                    for wi in 0..win_total {
                        let mut wt = wi;
                        let mut ok = true;
                        for d in (0..sdim).rev() {
                            let wv = wt % (window[d] as usize);
                            wt /= window[d] as usize;
                            let pos = (out_coords[lead + d] as isize) * (strides[d] as isize)
                                + (wv as isize) * (dilations[d] as isize)
                                - (start_pad[d] as isize);
                            if pos < 0 || pos >= in_sp[d] as isize {
                                ok = false;
                            } else {
                                in_coords[lead + d] = pos as usize;
                            }
                        }
                        if ok {
                            let v = input.read_f64(in_b, &in_coords);
                            if !found || v > best {
                                best = v;
                                found = true;
                                let mut flat_in = 0usize;
                                for d in 0..sdim {
                                    flat_in = flat_in * in_sp[d] + in_coords[lead + d];
                                }
                                best_flat = flat_in;
                            }
                        }
                    }
                    output.write_f64(out_b, &out_coords, if found { best } else { 0.0 });
                    if let (Some(id_desc), Some(ib)) = (output_indices.as_ref(), idx_b.as_deref_mut()) {
                        id_desc.write_f64(ib, &out_coords, best_flat as f64);
                    }
                }
            }
        }
        OpSpec::Cast { input, output } => {
            let (Some(in_b), Some(out_b)) = (inputs.first().and_then(|x| *x), outputs.first_mut().and_then(|x| x.as_deref_mut())) else {
                return;
            };
            let mut coords = vec![0usize; output.sizes.len()];
            for flat in 0..output.elem_count() {
                output.linear_to_coords(flat, &mut coords);
                let v = input.read_f64(in_b, &coords);
                output.write_f64(out_b, &coords, v);
            }
        }
        OpSpec::Join { inputs: in_descs, output, axis } => {
            let Some(out_b) = outputs.first_mut().and_then(|x| x.as_deref_mut()) else { return };
            let mut axis_offset = 0usize;
            for (idx, td) in in_descs.iter().enumerate() {
                if let Some(in_b) = inputs.get(idx).and_then(|x| *x) {
                    let mut in_coords = vec![0usize; td.sizes.len()];
                    let mut out_coords = vec![0usize; output.sizes.len()];
                    for flat in 0..td.elem_count() {
                        td.linear_to_coords(flat, &mut in_coords);
                        out_coords.copy_from_slice(&in_coords);
                        out_coords[*axis] += axis_offset;
                        let v = td.read_f64(in_b, &in_coords);
                        output.write_f64(out_b, &out_coords, v);
                    }
                }
                axis_offset += td.sizes[*axis] as usize;
            }
        }
        OpSpec::Split { input, outputs: out_descs, axis } => {
            let Some(in_b) = inputs.first().and_then(|x| *x) else { return };
            let mut axis_offset = 0usize;
            for (idx, td) in out_descs.iter().enumerate() {
                if let Some(out_b) = outputs.get_mut(idx).and_then(|x| x.as_deref_mut()) {
                    let mut out_coords = vec![0usize; td.sizes.len()];
                    let mut in_coords = vec![0usize; input.sizes.len()];
                    for flat in 0..td.elem_count() {
                        td.linear_to_coords(flat, &mut out_coords);
                        in_coords.copy_from_slice(&out_coords);
                        in_coords[*axis] += axis_offset;
                        let v = input.read_f64(in_b, &in_coords);
                        td.write_f64(out_b, &out_coords, v);
                    }
                }
                axis_offset += td.sizes[*axis] as usize;
            }
        }
        OpSpec::Reduce { func, input, output, axes } => {
            let (Some(in_b), Some(out_b)) = (inputs.first().and_then(|x| *x), outputs.first_mut().and_then(|x| x.as_deref_mut())) else {
                return;
            };
            let rank = input.sizes.len();
            let mut is_reduced = vec![false; rank];
            for &a in axes {
                if a < rank {
                    is_reduced[a] = true;
                }
            }
            let red_dims: Vec<usize> = (0..rank).filter(|&d| is_reduced[d]).collect();
            let red_total = red_dims.iter().fold(1usize, |acc, &d| acc * input.sizes[d] as usize);
            let mut out_coords = vec![0usize; rank];
            let mut in_coords = vec![0usize; rank];

            for flat in 0..output.elem_count() {
                output.linear_to_coords(flat, &mut out_coords);
                in_coords.copy_from_slice(&out_coords);
                let mut sum = 0.0f64;
                let mut prod = 1.0f64;
                let mut max_v = f64::NEG_INFINITY;
                let mut min_v = f64::INFINITY;
                let mut best_idx = 0usize;
                let mut vals = if *func == DML_REDUCE_FUNCTION_LOG_SUM_EXP {
                    Vec::with_capacity(red_total)
                } else {
                    Vec::new()
                };

                for r in 0..red_total {
                    let mut tmp = r;
                    for &d in red_dims.iter().rev() {
                        let sz = input.sizes[d] as usize;
                        in_coords[d] = tmp % sz;
                        tmp /= sz;
                    }
                    let v = input.read_f64(in_b, &in_coords);
                    match *func {
                        DML_REDUCE_FUNCTION_SUM | DML_REDUCE_FUNCTION_AVERAGE | DML_REDUCE_FUNCTION_LOG_SUM => sum += v,
                        DML_REDUCE_FUNCTION_SUM_SQUARE | DML_REDUCE_FUNCTION_L2 => sum += v * v,
                        DML_REDUCE_FUNCTION_L1 => sum += v.abs(),
                        DML_REDUCE_FUNCTION_MULTIPLY => prod *= v,
                        DML_REDUCE_FUNCTION_MAX => max_v = max_v.max(v),
                        DML_REDUCE_FUNCTION_MIN => min_v = min_v.min(v),
                        DML_REDUCE_FUNCTION_ARGMAX => {
                            if r == 0 || v > max_v {
                                max_v = v;
                                best_idx = r;
                            }
                        }
                        DML_REDUCE_FUNCTION_ARGMIN => {
                            if r == 0 || v < min_v {
                                min_v = v;
                                best_idx = r;
                            }
                        }
                        DML_REDUCE_FUNCTION_LOG_SUM_EXP => {
                            max_v = max_v.max(v);
                            vals.push(v);
                        }
                        _ => sum += v,
                    }
                }
                let res = match *func {
                    DML_REDUCE_FUNCTION_SUM | DML_REDUCE_FUNCTION_L1 | DML_REDUCE_FUNCTION_SUM_SQUARE => sum,
                    DML_REDUCE_FUNCTION_AVERAGE => sum / (red_total.max(1) as f64),
                    DML_REDUCE_FUNCTION_L2 => sum.sqrt(),
                    DML_REDUCE_FUNCTION_LOG_SUM => sum.ln(),
                    DML_REDUCE_FUNCTION_LOG_SUM_EXP => {
                        let se: f64 = vals.into_iter().map(|v| (v - max_v).exp()).sum();
                        max_v + se.ln()
                    }
                    DML_REDUCE_FUNCTION_MAX => max_v,
                    DML_REDUCE_FUNCTION_MIN => min_v,
                    DML_REDUCE_FUNCTION_MULTIPLY => prod,
                    DML_REDUCE_FUNCTION_ARGMAX | DML_REDUCE_FUNCTION_ARGMIN => best_idx as f64,
                    _ => sum,
                };
                output.write_f64(out_b, &out_coords, res);
            }
        }
        OpSpec::Mvn { input, scale, bias, output, axes, normalize_variance, epsilon, fused } => {
            let (Some(in_b), Some(out_b)) = (inputs.first().and_then(|x| *x), outputs.first_mut().and_then(|x| x.as_deref_mut())) else {
                return;
            };
            let sc_b = inputs.get(1).and_then(|x| *x);
            let bi_b = inputs.get(2).and_then(|x| *x);
            let rank = input.sizes.len();
            let mut is_norm = vec![false; rank];
            for &a in axes {
                if a < rank {
                    is_norm[a] = true;
                }
            }
            let norm_dims: Vec<usize> = (0..rank).filter(|&d| is_norm[d]).collect();
            let keep_dims: Vec<usize> = (0..rank).filter(|&d| !is_norm[d]).collect();
            let keep_total = keep_dims.iter().fold(1usize, |a, &d| a * input.sizes[d] as usize);
            let norm_total = norm_dims.iter().fold(1usize, |a, &d| a * input.sizes[d] as usize);
            let mut coords = vec![0usize; rank];

            for k in 0..keep_total {
                let mut tmp = k;
                for &d in keep_dims.iter().rev() {
                    let sz = input.sizes[d] as usize;
                    coords[d] = tmp % sz;
                    tmp /= sz;
                }
                let mut sum = 0.0f64;
                for r in 0..norm_total {
                    let mut rt = r;
                    for &d in norm_dims.iter().rev() {
                        let sz = input.sizes[d] as usize;
                        coords[d] = rt % sz;
                        rt /= sz;
                    }
                    sum += input.read_f64(in_b, &coords);
                }
                let mean = sum / (norm_total.max(1) as f64);
                let inv_std = if *normalize_variance {
                    let mut var_sum = 0.0f64;
                    for r in 0..norm_total {
                        let mut rt = r;
                        for &d in norm_dims.iter().rev() {
                            let sz = input.sizes[d] as usize;
                            coords[d] = rt % sz;
                            rt /= sz;
                        }
                        let diff = input.read_f64(in_b, &coords) - mean;
                        var_sum += diff * diff;
                    }
                    1.0 / (var_sum / (norm_total.max(1) as f64) + (*epsilon as f64)).sqrt()
                } else {
                    1.0
                };
                for r in 0..norm_total {
                    let mut rt = r;
                    for &d in norm_dims.iter().rev() {
                        let sz = input.sizes[d] as usize;
                        coords[d] = rt % sz;
                        rt /= sz;
                    }
                    let mut y = (input.read_f64(in_b, &coords) - mean) * inv_std;
                    if let (Some(sd), Some(sb)) = (scale.as_ref(), sc_b) {
                        y *= sd.read_f64(sb, &coords);
                    }
                    if let (Some(bd), Some(bb)) = (bias.as_ref(), bi_b) {
                        y += bd.read_f64(bb, &coords);
                    }
                    if let Some(act) = fused {
                        y = act.apply(y);
                    }
                    output.write_f64(out_b, &coords, y);
                }
            }
        }
        OpSpec::BatchNorm { input, mean, variance, scale, bias, output, epsilon, fused } => {
            let (Some(in_b), Some(mn_b), Some(vr_b), Some(sc_b), Some(bi_b), Some(out_b)) = (
                inputs.first().and_then(|x| *x),
                inputs.get(1).and_then(|x| *x),
                inputs.get(2).and_then(|x| *x),
                inputs.get(3).and_then(|x| *x),
                inputs.get(4).and_then(|x| *x),
                outputs.first_mut().and_then(|x| x.as_deref_mut()),
            ) else {
                return;
            };
            let mut coords = vec![0usize; output.sizes.len()];
            for flat in 0..output.elem_count() {
                output.linear_to_coords(flat, &mut coords);
                let x = input.read_f64(in_b, &coords);
                let m = mean.read_f64(mn_b, &coords);
                let v = variance.read_f64(vr_b, &coords);
                let s = scale.read_f64(sc_b, &coords);
                let b = bias.read_f64(bi_b, &coords);
                let mut y = s * (x - m) / (v + (*epsilon as f64)).sqrt() + b;
                if let Some(act) = fused {
                    y = act.apply(y);
                }
                output.write_f64(out_b, &coords, y);
            }
        }
    }
}
