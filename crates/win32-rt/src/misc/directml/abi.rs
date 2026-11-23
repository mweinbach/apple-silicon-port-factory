//! DirectML Win64 ABI constants, IIDs, and `#[repr(C)]` descriptor layouts
//! (verified against `DirectML.h` / WinMD).

use std::ffi::c_void;

use crate::com::{guid, Guid};
use crate::types::BOOL;

// ============================================================================
// Official DirectML IIDs & ABI Constants (verified against DirectML.h / WinMD)
// ============================================================================

pub const IID_IDMLObject: Guid = guid("c8263aac-9e0c-4a2d-9b8e-007521a3317c");
pub const IID_IDMLDevice: Guid = guid("6dbd6437-96fd-423f-a98c-ae5e7c2a573f");
pub const IID_IDMLDevice1: Guid = guid("a0884f9a-d2be-4355-aa5d-5901281ad1d2");
pub const IID_IDMLDeviceChild: Guid = guid("27e83142-8165-49e3-974e-2fd66e4cb69d");
pub const IID_IDMLPageable: Guid = guid("b1ab0825-4542-4a4b-8617-6dde6e8f6201");
pub const IID_IDMLDispatchable: Guid = guid("dcb821a8-1039-441e-9f1c-b1759c2f3cec");
pub const IID_IDMLOperator: Guid = guid("26caae7a-3081-4633-9581-226fbe57695d");
pub const IID_IDMLCompiledOperator: Guid = guid("6b15e56a-bf5c-4902-92d8-da3a650afea4");
pub const IID_IDMLOperatorInitializer: Guid = guid("427c1113-435c-469c-8676-4d5dd072f813");
pub const IID_IDMLBindingTable: Guid = guid("29c687dc-de74-4e3b-ab00-1168f2fc3cfc");
pub const IID_IDMLCommandRecorder: Guid = guid("e6857a76-2e3e-4fdd-bff4-5d2ba10fb453");

// DML_CREATE_DEVICE_FLAGS
pub const DML_CREATE_DEVICE_FLAG_NONE: u32 = 0;
pub const DML_CREATE_DEVICE_FLAG_DEBUG: u32 = 1;

// DML_FEATURE
pub const DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT: u32 = 0;
pub const DML_FEATURE_FEATURE_LEVELS: u32 = 1;

// DML_FEATURE_LEVEL
pub const DML_FEATURE_LEVEL_1_0: u32 = 0x1000;
pub const DML_FEATURE_LEVEL_2_0: u32 = 0x2000;
pub const DML_FEATURE_LEVEL_2_1: u32 = 0x2100;
pub const DML_FEATURE_LEVEL_3_0: u32 = 0x3000;
pub const DML_FEATURE_LEVEL_3_1: u32 = 0x3100;
pub const DML_FEATURE_LEVEL_4_0: u32 = 0x4000;
pub const DML_FEATURE_LEVEL_4_1: u32 = 0x4100;
pub const DML_FEATURE_LEVEL_5_0: u32 = 0x5000;
pub(super) const MAX_SUPPORTED_FEATURE_LEVEL: u32 = DML_FEATURE_LEVEL_5_0;

// DML_TENSOR_DATA_TYPE
pub const DML_TENSOR_DATA_TYPE_UNKNOWN: u32 = 0;
pub const DML_TENSOR_DATA_TYPE_FLOAT32: u32 = 1;
pub const DML_TENSOR_DATA_TYPE_FLOAT16: u32 = 2;
pub const DML_TENSOR_DATA_TYPE_UINT32: u32 = 3;
pub const DML_TENSOR_DATA_TYPE_UINT16: u32 = 4;
pub const DML_TENSOR_DATA_TYPE_UINT8: u32 = 5;
pub const DML_TENSOR_DATA_TYPE_INT32: u32 = 6;
pub const DML_TENSOR_DATA_TYPE_INT16: u32 = 7;
pub const DML_TENSOR_DATA_TYPE_INT8: u32 = 8;
pub const DML_TENSOR_DATA_TYPE_FLOAT64: u32 = 9;
pub const DML_TENSOR_DATA_TYPE_UINT64: u32 = 10;
pub const DML_TENSOR_DATA_TYPE_INT64: u32 = 11;

// DML_TENSOR_TYPE
pub const DML_TENSOR_TYPE_INVALID: u32 = 0;
pub const DML_TENSOR_TYPE_BUFFER: u32 = 1;

// DML_TENSOR_FLAGS
pub const DML_TENSOR_FLAG_NONE: u32 = 0;
pub const DML_TENSOR_FLAG_OWNED_BY_DML: u32 = 1;

// DML_BINDING_TYPE
pub const DML_BINDING_TYPE_NONE: u32 = 0;
pub const DML_BINDING_TYPE_BUFFER: u32 = 1;
pub const DML_BINDING_TYPE_BUFFER_ARRAY: u32 = 2;

// DML_MATRIX_TRANSFORM
pub const DML_MATRIX_TRANSFORM_NONE: u32 = 0;
pub const DML_MATRIX_TRANSFORM_TRANSPOSE: u32 = 1;

// DML_CONVOLUTION_MODE / DIRECTION
pub const DML_CONVOLUTION_MODE_CONVOLUTION: u32 = 0;
pub const DML_CONVOLUTION_MODE_CROSS_CORRELATION: u32 = 1;
pub const DML_CONVOLUTION_DIRECTION_FORWARD: u32 = 0;
pub const DML_CONVOLUTION_DIRECTION_BACKWARD: u32 = 1;

// DML_REDUCE_FUNCTION
pub const DML_REDUCE_FUNCTION_ARGMAX: u32 = 0;
pub const DML_REDUCE_FUNCTION_ARGMIN: u32 = 1;
pub const DML_REDUCE_FUNCTION_AVERAGE: u32 = 2;
pub const DML_REDUCE_FUNCTION_L1: u32 = 3;
pub const DML_REDUCE_FUNCTION_L2: u32 = 4;
pub const DML_REDUCE_FUNCTION_LOG_SUM: u32 = 5;
pub const DML_REDUCE_FUNCTION_LOG_SUM_EXP: u32 = 6;
pub const DML_REDUCE_FUNCTION_MAX: u32 = 7;
pub const DML_REDUCE_FUNCTION_MIN: u32 = 8;
pub const DML_REDUCE_FUNCTION_MULTIPLY: u32 = 9;
pub const DML_REDUCE_FUNCTION_SUM: u32 = 10;
pub const DML_REDUCE_FUNCTION_SUM_SQUARE: u32 = 11;

// DML_GRAPH_NODE_TYPE / DML_GRAPH_EDGE_TYPE
pub const DML_GRAPH_NODE_TYPE_OPERATOR: u32 = 1;
pub const DML_GRAPH_EDGE_TYPE_INPUT: u32 = 1;
pub const DML_GRAPH_EDGE_TYPE_OUTPUT: u32 = 2;
pub const DML_GRAPH_EDGE_TYPE_INTERMEDIATE: u32 = 3;

// DML_OPERATOR_TYPE (exact WinMD / DirectML.h values)
pub const DML_OPERATOR_INVALID: u32 = 0;
pub const DML_OPERATOR_ELEMENT_WISE_IDENTITY: u32 = 1;
pub const DML_OPERATOR_ELEMENT_WISE_ABS: u32 = 2;
pub const DML_OPERATOR_ELEMENT_WISE_ACOS: u32 = 3;
pub const DML_OPERATOR_ELEMENT_WISE_ADD: u32 = 4;
pub const DML_OPERATOR_ELEMENT_WISE_ASIN: u32 = 5;
pub const DML_OPERATOR_ELEMENT_WISE_ATAN: u32 = 6;
pub const DML_OPERATOR_ELEMENT_WISE_CEIL: u32 = 7;
pub const DML_OPERATOR_ELEMENT_WISE_CLIP: u32 = 8;
pub const DML_OPERATOR_ELEMENT_WISE_COS: u32 = 9;
pub const DML_OPERATOR_ELEMENT_WISE_DIVIDE: u32 = 10;
pub const DML_OPERATOR_ELEMENT_WISE_EXP: u32 = 11;
pub const DML_OPERATOR_ELEMENT_WISE_FLOOR: u32 = 12;
pub const DML_OPERATOR_ELEMENT_WISE_LOG: u32 = 13;
pub const DML_OPERATOR_ELEMENT_WISE_MAX: u32 = 21;
pub const DML_OPERATOR_ELEMENT_WISE_MEAN: u32 = 22;
pub const DML_OPERATOR_ELEMENT_WISE_MIN: u32 = 23;
pub const DML_OPERATOR_ELEMENT_WISE_MULTIPLY: u32 = 24;
pub const DML_OPERATOR_ELEMENT_WISE_POW: u32 = 25;
pub const DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW: u32 = 26;
pub const DML_OPERATOR_ELEMENT_WISE_RECIP: u32 = 27;
pub const DML_OPERATOR_ELEMENT_WISE_SIN: u32 = 28;
pub const DML_OPERATOR_ELEMENT_WISE_SQRT: u32 = 29;
pub const DML_OPERATOR_ELEMENT_WISE_SUBTRACT: u32 = 30;
pub const DML_OPERATOR_ELEMENT_WISE_TAN: u32 = 31;
pub const DML_OPERATOR_ELEMENT_WISE_THRESHOLD: u32 = 32;
pub const DML_OPERATOR_ACTIVATION_ELU: u32 = 35;
pub const DML_OPERATOR_ACTIVATION_HARD_SIGMOID: u32 = 37;
pub const DML_OPERATOR_ACTIVATION_IDENTITY: u32 = 38;
pub const DML_OPERATOR_ACTIVATION_LEAKY_RELU: u32 = 39;
pub const DML_OPERATOR_ACTIVATION_LINEAR: u32 = 40;
pub const DML_OPERATOR_ACTIVATION_LOG_SOFTMAX: u32 = 41;
pub const DML_OPERATOR_ACTIVATION_PARAMETERIZED_RELU: u32 = 42;
pub const DML_OPERATOR_ACTIVATION_RELU: u32 = 44;
pub const DML_OPERATOR_ACTIVATION_SCALED_ELU: u32 = 45;
pub const DML_OPERATOR_ACTIVATION_SCALED_TANH: u32 = 46;
pub const DML_OPERATOR_ACTIVATION_SIGMOID: u32 = 47;
pub const DML_OPERATOR_ACTIVATION_SOFTMAX: u32 = 48;
pub const DML_OPERATOR_ACTIVATION_SOFTPLUS: u32 = 49;
pub const DML_OPERATOR_ACTIVATION_SOFTSIGN: u32 = 50;
pub const DML_OPERATOR_ACTIVATION_TANH: u32 = 51;
pub const DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU: u32 = 52;
pub const DML_OPERATOR_CONVOLUTION: u32 = 53;
pub const DML_OPERATOR_GEMM: u32 = 54;
pub const DML_OPERATOR_REDUCE: u32 = 55;
pub const DML_OPERATOR_AVERAGE_POOLING: u32 = 56;
pub const DML_OPERATOR_MAX_POOLING: u32 = 58;
pub const DML_OPERATOR_CAST: u32 = 61;
pub const DML_OPERATOR_SPLIT: u32 = 62;
pub const DML_OPERATOR_JOIN: u32 = 63;
pub const DML_OPERATOR_BATCH_NORMALIZATION: u32 = 72;
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION: u32 = 73;
pub const DML_OPERATOR_ELEMENT_WISE_SIGN: u32 = 79;
pub const DML_OPERATOR_ELEMENT_WISE_TANH: u32 = 84;
pub const DML_OPERATOR_ELEMENT_WISE_ADD1: u32 = 89;
pub const DML_OPERATOR_MAX_POOLING1: u32 = 91;
pub const DML_OPERATOR_ELEMENT_WISE_ROUND: u32 = 99;
pub const DML_OPERATOR_MAX_POOLING2: u32 = 110;
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION1: u32 = 115;
pub const DML_OPERATOR_ACTIVATION_CELU: u32 = 128;
pub const DML_OPERATOR_ELEMENT_WISE_NEGATE: u32 = 156;
pub const DML_OPERATOR_ACTIVATION_GELU: u32 = 157;

// ============================================================================
// Exact Win64 ABI Structures (DirectML.h)
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_TENSOR_DESC {
    pub Type: u32,
    pub _pad: u32,
    pub Desc: *const c_void,
}
const _: () = assert!(std::mem::size_of::<DML_TENSOR_DESC>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_BUFFER_TENSOR_DESC {
    pub DataType: u32,
    pub Flags: u32,
    pub DimensionCount: u32,
    pub _pad0: u32,
    pub Sizes: *const u32,
    pub Strides: *const u32,
    pub TotalTensorSizeInBytes: u64,
    pub GuaranteedBaseOffsetAlignment: u32,
    pub _pad1: u32,
}
const _: () = assert!(std::mem::size_of::<DML_BUFFER_TENSOR_DESC>() == 48);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_OPERATOR_DESC {
    pub Type: u32,
    pub _pad: u32,
    pub Desc: *const c_void,
}
const _: () = assert!(std::mem::size_of::<DML_OPERATOR_DESC>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DML_SCALE_BIAS {
    pub Scale: f32,
    pub Bias: f32,
}
const _: () = assert!(std::mem::size_of::<DML_SCALE_BIAS>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DML_BINDING_PROPERTIES {
    pub RequiredDescriptorCount: u32,
    pub _pad: u32,
    pub TemporaryResourceSize: u64,
    pub PersistentResourceSize: u64,
}
const _: () = assert!(std::mem::size_of::<DML_BINDING_PROPERTIES>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_BINDING_TABLE_DESC {
    pub Dispatchable: *mut c_void,
    pub CPUDescriptorHandle: usize,
    pub GPUDescriptorHandle: u64,
    pub SizeInDescriptors: u32,
    pub _pad: u32,
}
const _: () = assert!(std::mem::size_of::<DML_BINDING_TABLE_DESC>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_BINDING_DESC {
    pub Type: u32,
    pub _pad: u32,
    pub Desc: *const c_void,
}
const _: () = assert!(std::mem::size_of::<DML_BINDING_DESC>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_BUFFER_BINDING {
    pub Buffer: *mut c_void,
    pub Offset: u64,
    pub SizeInBytes: u64,
}
const _: () = assert!(std::mem::size_of::<DML_BUFFER_BINDING>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_BUFFER_ARRAY_BINDING {
    pub BindingCount: u32,
    pub _pad: u32,
    pub Bindings: *const DML_BUFFER_BINDING,
}
const _: () = assert!(std::mem::size_of::<DML_BUFFER_ARRAY_BINDING>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_FEATURE_QUERY_FEATURE_LEVELS {
    pub RequestedFeatureLevelCount: u32,
    pub _pad: u32,
    pub RequestedFeatureLevels: *const u32,
}
const _: () = assert!(std::mem::size_of::<DML_FEATURE_QUERY_FEATURE_LEVELS>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ELEMENT_WISE_UNARY_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ELEMENT_WISE_BINARY_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
    pub Min: f32,
    pub Max: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ExponentTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
    pub Exponent: f32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
    pub Min: f32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_SIMPLE_UNARY_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ACTIVATION_ALPHA_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ACTIVATION_ALPHA_BETA_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_ACTIVATION_PRELU_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub SlopeTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_GEMM_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub CTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub TransA: u32,
    pub TransB: u32,
    pub Alpha: f32,
    pub Beta: f32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
const _: () = assert!(std::mem::size_of::<DML_GEMM_OPERATOR_DESC>() == 56);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_CONVOLUTION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub FilterTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Mode: u32,
    pub Direction: u32,
    pub DimensionCount: u32,
    pub _pad0: u32,
    pub Strides: *const u32,
    pub Dilations: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub OutputPadding: *const u32,
    pub GroupCount: u32,
    pub _pad1: u32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
const _: () = assert!(std::mem::size_of::<DML_CONVOLUTION_OPERATOR_DESC>() == 104);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_AVERAGE_POOLING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub _pad0: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub IncludePadding: BOOL,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_MAX_POOLING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub _pad0: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_MAX_POOLING1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub OutputIndicesTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub _pad0: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_MAX_POOLING2_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub OutputIndicesTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub _pad0: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub Dilations: *const u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_JOIN_OPERATOR_DESC {
    pub InputCount: u32,
    pub _pad0: u32,
    pub InputTensors: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_SPLIT_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputCount: u32,
    pub _pad0: u32,
    pub OutputTensors: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_REDUCE_OPERATOR_DESC {
    pub Function: u32,
    pub _pad0: u32,
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub _pad1: u32,
    pub Axes: *const u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub CrossChannel: BOOL,
    pub NormalizeVariance: BOOL,
    pub Epsilon: f32,
    pub _pad0: u32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub _pad0: u32,
    pub Axes: *const u32,
    pub NormalizeVariance: BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub MeanTensor: *const DML_TENSOR_DESC,
    pub VarianceTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Spatial: BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_GRAPH_DESC {
    pub InputCount: u32,
    pub OutputCount: u32,
    pub NodeCount: u32,
    pub _pad0: u32,
    pub Nodes: *const DML_GRAPH_NODE_DESC,
    pub InputEdgeCount: u32,
    pub _pad1: u32,
    pub InputEdges: *const DML_GRAPH_EDGE_DESC,
    pub OutputEdgeCount: u32,
    pub _pad2: u32,
    pub OutputEdges: *const DML_GRAPH_EDGE_DESC,
    pub IntermediateEdgeCount: u32,
    pub _pad3: u32,
    pub IntermediateEdges: *const DML_GRAPH_EDGE_DESC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_GRAPH_NODE_DESC {
    pub Type: u32,
    pub _pad: u32,
    pub Desc: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_OPERATOR_GRAPH_NODE_DESC {
    pub Operator: *mut c_void,
    pub Name: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_GRAPH_EDGE_DESC {
    pub Type: u32,
    pub _pad: u32,
    pub Desc: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_INPUT_GRAPH_EDGE_DESC {
    pub GraphInputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub _pad: u32,
    pub Name: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_OUTPUT_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub GraphOutputIndex: u32,
    pub _pad: u32,
    pub Name: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: *const u8,
}
