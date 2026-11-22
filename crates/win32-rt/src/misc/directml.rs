//! `directml.dll`: Native DirectML (`IDMLDevice` / `IDMLDevice1`) runtime on D3D12.
//!
//! Provides real DirectML COM objects (every vtable slot registered via
//! [`crate::register_native_va`]) and real CPU-backed D3D12 buffer tensor execution
//! inside `IDMLCommandRecorder::RecordDispatch` (handling both `IDMLCompiledOperator`
//! dispatch and `IDMLOperatorInitializer` initialization), accessing D3D12 buffer
//! memory strictly through [`crate::d3d12`] and `d3d12_metal` safe Rust wrappers
//! (zero `objc2-metal` imports).

use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::metal::cmdlist::{
    CommandAllocator as MtlCommandAllocator, CommandList as MtlCommandList,
};
use d3d12_metal::metal::queue::{ListType, Queue as MtlQueue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};

use crate::d3d12::com::{
    com_add_ref, com_query, com_release, guid, Guid, DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_FOUND,
    DXGI_ERROR_UNSUPPORTED, IID_IUnknown, WKPDID_D3DDebugObjectName, WKPDID_D3DDebugObjectNameW,
};
use crate::types::{
    BOOL, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER, FALSE,
    HRESULT, S_OK, TRUE,
};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

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
const MAX_SUPPORTED_FEATURE_LEVEL: u32 = DML_FEATURE_LEVEL_5_0;

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

// ============================================================================
// Tensor Data Types, IEEE-754 f16, and Multi-Dimensional Strided Indexing
// ============================================================================

fn elem_byte_size(dt: u32) -> Option<usize> {
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
struct TensorDesc {
    data_type: u32,
    flags: u32,
    sizes: Vec<u32>,
    strides: Vec<u32>,
    total_bytes: u64,
}

impl TensorDesc {
    unsafe fn parse(p: *const DML_TENSOR_DESC) -> Result<Self, HRESULT> {
        if p.is_null() {
            return Err(E_INVALIDARG);
        }
        let td = std::ptr::read_unaligned(p);
        if td.Type != DML_TENSOR_TYPE_BUFFER || td.Desc.is_null() {
            return Err(E_INVALIDARG);
        }
        let bd = std::ptr::read_unaligned(td.Desc as *const DML_BUFFER_TENSOR_DESC);
        let elem_sz = elem_byte_size(bd.DataType).ok_or(E_INVALIDARG)?;
        if bd.DimensionCount == 0 || bd.DimensionCount > 8 || bd.Sizes.is_null() {
            return Err(E_INVALIDARG);
        }
        let rank = bd.DimensionCount as usize;
        let mut sizes = Vec::with_capacity(rank);
        for i in 0..rank {
            let s = std::ptr::read_unaligned(bd.Sizes.add(i));
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
                strides[i] = std::ptr::read_unaligned(bd.Strides.add(i));
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
enum FusedAct {
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
        let d = std::ptr::read_unaligned(p);
        if d.Desc.is_null() {
            return Err(E_INVALIDARG);
        }
        let act = match d.Type {
            DML_OPERATOR_ACTIVATION_IDENTITY | DML_OPERATOR_ELEMENT_WISE_IDENTITY => FusedAct::Identity,
            DML_OPERATOR_ACTIVATION_RELU => FusedAct::Relu,
            DML_OPERATOR_ACTIVATION_SIGMOID => FusedAct::Sigmoid,
            DML_OPERATOR_ACTIVATION_TANH | DML_OPERATOR_ELEMENT_WISE_TANH => FusedAct::Tanh,
            DML_OPERATOR_ACTIVATION_LEAKY_RELU => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::LeakyRelu(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_ELU => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::Elu(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_LINEAR => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::Linear(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_HARD_SIGMOID => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::HardSigmoid(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_SCALED_ELU => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::ScaledElu(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_SCALED_TANH => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
                FusedAct::ScaledTanh(s.Alpha, s.Beta)
            }
            DML_OPERATOR_ACTIVATION_SOFTPLUS => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::Softplus(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_SOFTSIGN => FusedAct::Softsign,
            DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::ThresholdedRelu(s.Alpha)
            }
            DML_OPERATOR_ACTIVATION_GELU => FusedAct::Gelu,
            DML_OPERATOR_ACTIVATION_CELU => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ACTIVATION_ALPHA_DESC);
                FusedAct::Celu(s.Alpha)
            }
            DML_OPERATOR_ELEMENT_WISE_CLIP => {
                let s = std::ptr::read_unaligned(d.Desc as *const DML_ELEMENT_WISE_CLIP_OPERATOR_DESC);
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
enum UnaryOp {
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
enum BinaryOp {
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
enum OpSpec {
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
    fn input_descs(&self) -> Vec<Option<&TensorDesc>> {
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

    fn output_descs(&self) -> Vec<Option<&TensorDesc>> {
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
    fn persistent_layout(&self) -> (Vec<(usize, u64, u64)>, u64) {
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

    unsafe fn parse(desc_ptr: *const DML_OPERATOR_DESC) -> Result<Self, HRESULT> {
        if desc_ptr.is_null() {
            return Err(E_INVALIDARG);
        }
        let od = std::ptr::read_unaligned(desc_ptr);
        if od.Desc.is_null() {
            return Err(E_INVALIDARG);
        }

        let read_sb = |p: *const DML_SCALE_BIAS| -> Option<(f32, f32)> {
            if p.is_null() {
                None
            } else {
                let sb = std::ptr::read_unaligned(p);
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
                v.push(std::ptr::read_unaligned(p.add(i)));
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ELEMENT_WISE_UNARY_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_SIMPLE_UNARY_DESC);
                Ok(OpSpec::Unary {
                    op: UnaryOp::Neg,
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: None,
                })
            }
            DML_OPERATOR_ELEMENT_WISE_CLIP => {
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ELEMENT_WISE_CLIP_OPERATOR_DESC);
                Ok(OpSpec::Unary {
                    op: UnaryOp::Clip(d.Min, d.Max),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: read_sb(d.ScaleBias),
                })
            }
            DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW => {
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC);
                Ok(OpSpec::Unary {
                    op: UnaryOp::ConstPow(d.Exponent),
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    scale_bias: read_sb(d.ScaleBias),
                })
            }
            DML_OPERATOR_ELEMENT_WISE_THRESHOLD => {
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ELEMENT_WISE_BINARY_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ELEMENT_WISE_ADD1_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ELEMENT_WISE_POW_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_SIMPLE_UNARY_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ACTIVATION_ALPHA_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ACTIVATION_ALPHA_BETA_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_ACTIVATION_PRELU_DESC);
                Ok(OpSpec::PRelu {
                    input: TensorDesc::parse(d.InputTensor)?,
                    slope: TensorDesc::parse(d.SlopeTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                })
            }
            DML_OPERATOR_ACTIVATION_SOFTMAX | DML_OPERATOR_ACTIVATION_LOG_SOFTMAX => {
                let d = std::ptr::read_unaligned(od.Desc as *const DML_SIMPLE_UNARY_DESC);
                Ok(OpSpec::Softmax {
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                    log_mode: od.Type == DML_OPERATOR_ACTIVATION_LOG_SOFTMAX,
                })
            }
            DML_OPERATOR_GEMM => {
                let d = std::ptr::read_unaligned(od.Desc as *const DML_GEMM_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_CONVOLUTION_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_AVERAGE_POOLING_OPERATOR_DESC);
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
                        let d = std::ptr::read_unaligned(od.Desc as *const DML_MAX_POOLING_OPERATOR_DESC);
                        (d.InputTensor, d.OutputTensor, std::ptr::null(), d.DimensionCount, d.Strides, d.WindowSize, d.StartPadding, d.EndPadding, std::ptr::null())
                    }
                    DML_OPERATOR_MAX_POOLING1 => {
                        let d = std::ptr::read_unaligned(od.Desc as *const DML_MAX_POOLING1_OPERATOR_DESC);
                        (d.InputTensor, d.OutputTensor, d.OutputIndicesTensor, d.DimensionCount, d.Strides, d.WindowSize, d.StartPadding, d.EndPadding, std::ptr::null())
                    }
                    _ => {
                        let d = std::ptr::read_unaligned(od.Desc as *const DML_MAX_POOLING2_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_SIMPLE_UNARY_DESC);
                Ok(OpSpec::Cast {
                    input: TensorDesc::parse(d.InputTensor)?,
                    output: TensorDesc::parse(d.OutputTensor)?,
                })
            }
            DML_OPERATOR_JOIN => {
                let d = std::ptr::read_unaligned(od.Desc as *const DML_JOIN_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_SPLIT_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_REDUCE_OPERATOR_DESC);
                let input = TensorDesc::parse(d.InputTensor)?;
                let output = TensorDesc::parse(d.OutputTensor)?;
                if d.AxisCount == 0 || d.Axes.is_null() || output.sizes.len() != input.sizes.len() {
                    return Err(E_INVALIDARG);
                }
                let mut axes = Vec::with_capacity(d.AxisCount as usize);
                for i in 0..d.AxisCount as usize {
                    let a = std::ptr::read_unaligned(d.Axes.add(i)) as usize;
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC);
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC);
                let input = TensorDesc::parse(d.InputTensor)?;
                let mut axes = Vec::with_capacity(d.AxisCount as usize);
                if d.AxisCount > 0 && !d.Axes.is_null() {
                    for i in 0..d.AxisCount as usize {
                        let a = std::ptr::read_unaligned(d.Axes.add(i)) as usize;
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
                let d = std::ptr::read_unaligned(od.Desc as *const DML_BATCH_NORMALIZATION_OPERATOR_DESC);
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

fn execute_op(op: &OpSpec, inputs: &[Option<&[u8]>], outputs: &mut [Option<&mut [u8]>]) {
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

// ============================================================================
// Safe D3D12 Buffer Memory Access (via crate::d3d12 + d3d12_metal, zero objc2-metal)
// ============================================================================

struct BufferCopier {
    queue: MtlQueue,
    alloc: MtlCommandAllocator,
    cl: MtlCommandList,
}

impl BufferCopier {
    fn new() -> Option<Self> {
        let queue = MtlQueue::new(ListType::Copy, "DirectML copy")?;
        let alloc = MtlCommandAllocator::new(ListType::Copy)?;
        let cl = MtlCommandList::new(ListType::Copy)?;
        Some(Self { queue, alloc, cl })
    }

    fn alloc_staging(size: u64, is_readback: bool) -> Option<Arc<MtlResource>> {
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: size.max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties {
            Type: if is_readback { heap_type::READBACK } else { heap_type::UPLOAD },
            ..Default::default()
        };
        let r = MtlResource::new_committed_default(&desc, &hp, "directml staging").ok()?;
        let arc = Arc::new(r);
        arc.register_ptr();
        Some(arc)
    }

    fn read_buffer(&mut self, src: &Arc<MtlResource>, offset: u64, len: usize) -> Option<Vec<u8>> {
        let rb = Self::alloc_staging(len as u64, true)?;
        self.alloc.reset();
        self.cl.begin(&self.alloc).ok()?;
        self.cl.barrier();
        self.cl.copy_buffer(&rb, 0, src, offset, len as u64).ok()?;
        self.cl.barrier();
        self.cl.close().ok()?;
        let v = self.queue.submit(&[&self.cl.cb]);
        self.queue.timeline.wait_cpu(v, u64::MAX);
        let p = rb.cpu_ptr()?;
        let mut out = vec![0u8; len];
        unsafe { std::ptr::copy_nonoverlapping(p, out.as_mut_ptr(), len) };
        Some(out)
    }

    fn write_buffer(&mut self, dst: &Arc<MtlResource>, offset: u64, data: &[u8]) -> bool {
        if data.is_empty() {
            return true;
        }
        let Some(up) = Self::alloc_staging(data.len() as u64, false) else {
            return false;
        };
        let Some(p) = up.cpu_ptr() else {
            return false;
        };
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), p, data.len()) };
        self.alloc.reset();
        if self.cl.begin(&self.alloc).is_err() {
            return false;
        }
        self.cl.barrier();
        if self.cl.copy_buffer(dst, offset, &up, 0, data.len() as u64).is_err() {
            let _ = self.cl.close();
            return false;
        }
        self.cl.barrier();
        if self.cl.close().is_err() {
            return false;
        }
        let v = self.queue.submit(&[&self.cl.cb]);
        self.queue.timeline.wait_cpu(v, u64::MAX);
        true
    }
}

fn copier_lock() -> &'static Mutex<Option<BufferCopier>> {
    static C: OnceLock<Mutex<Option<BufferCopier>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

#[derive(Clone, Debug)]
struct BufferBinding {
    resource_ptr: usize,
    offset: u64,
    size: u64,
}

#[derive(Clone, Debug)]
enum BindingSlot {
    None,
    Buffer(BufferBinding),
    BufferArray(Vec<BufferBinding>),
}

impl BindingSlot {
    fn first_buffer(&self) -> Option<&BufferBinding> {
        match self {
            BindingSlot::None => None,
            BindingSlot::Buffer(b) => Some(b),
            BindingSlot::BufferArray(v) => v.first(),
        }
    }
}

/// Read `len` bytes from a D3D12 buffer binding (fast-path on `cpu_ptr` / `shared_slice`,
/// GPU staging readback fallback via `BufferCopier`, or foreign COM `Map`/`Unmap`).
unsafe fn read_d3d12_buffer(binding: &BufferBinding, min_len: usize) -> Option<Vec<u8>> {
    if binding.resource_ptr == 0 {
        return None;
    }
    let len = (binding.size as usize).max(min_len);
    if len == 0 {
        return Some(Vec::new());
    }
    if let Some(res) = crate::d3d12::resource::from_ptr(binding.resource_ptr as *mut c_void) {
        let width = res.r.desc.Width;
        if binding.offset >= width {
            return Some(vec![0u8; len]);
        }
        let avail = (width - binding.offset).min(len as u64) as usize;
        if let Some(p) = res.r.cpu_ptr() {
            let mut out = vec![0u8; len];
            std::ptr::copy_nonoverlapping(p.add(binding.offset as usize), out.as_mut_ptr(), avail);
            return Some(out);
        }
        if let Some(s) = res.r.shared_slice(binding.offset, avail) {
            let mut out = vec![0u8; len];
            out[..s.len()].copy_from_slice(s);
            return Some(out);
        }
        let mut cg = copier_lock().lock().unwrap();
        if cg.is_none() {
            *cg = BufferCopier::new();
        }
        if let Some(copier) = cg.as_mut() {
            if let Some(mut v) = copier.read_buffer(&res.r, binding.offset, avail) {
                v.resize(len, 0);
                return Some(v);
            }
        }
        return None;
    }
    // Foreign ID3D12Resource COM fallback (Map slot 8, Unmap slot 9)
    let p_res = binding.resource_ptr as *mut c_void;
    let vt = std::ptr::read_unaligned(p_res as *const *const usize);
    let map_fn: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, *mut *mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(8)));
    let unmap_fn: unsafe extern "win64" fn(*mut c_void, u32, *const c_void) =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(9)));
    let mut mapped: *mut c_void = std::ptr::null_mut();
    if map_fn(p_res, 0, std::ptr::null(), &mut mapped) < 0 || mapped.is_null() {
        return None;
    }
    let mut out = vec![0u8; len];
    std::ptr::copy_nonoverlapping((mapped as *const u8).add(binding.offset as usize), out.as_mut_ptr(), len);
    unmap_fn(p_res, 0, std::ptr::null());
    Some(out)
}

/// Write `data` into a D3D12 buffer binding (`cpu_ptr` direct write, GPU upload via
/// `BufferCopier` for DEFAULT heaps, or foreign COM `Map`/`Unmap`).
unsafe fn write_d3d12_buffer(binding: &BufferBinding, data: &[u8]) {
    if binding.resource_ptr == 0 || data.is_empty() {
        return;
    }
    let max_write = if binding.size > 0 {
        data.len().min(binding.size as usize)
    } else {
        data.len()
    };
    let slice = &data[..max_write];
    if let Some(res) = crate::d3d12::resource::from_ptr(binding.resource_ptr as *mut c_void) {
        let width = res.r.desc.Width;
        if binding.offset >= width {
            return;
        }
        let clamped = slice.len().min((width - binding.offset) as usize);
        if let Some(p) = res.r.cpu_ptr() {
            std::ptr::copy_nonoverlapping(slice.as_ptr(), p.add(binding.offset as usize), clamped);
            return;
        }
        let mut cg = copier_lock().lock().unwrap();
        if cg.is_none() {
            *cg = BufferCopier::new();
        }
        if let Some(copier) = cg.as_mut() {
            let _ = copier.write_buffer(&res.r, binding.offset, &slice[..clamped]);
        }
        return;
    }
    let p_res = binding.resource_ptr as *mut c_void;
    let vt = std::ptr::read_unaligned(p_res as *const *const usize);
    let map_fn: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, *mut *mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(8)));
    let unmap_fn: unsafe extern "win64" fn(*mut c_void, u32, *const c_void) =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(9)));
    let mut mapped: *mut c_void = std::ptr::null_mut();
    if map_fn(p_res, 0, std::ptr::null(), &mut mapped) >= 0 && !mapped.is_null() {
        std::ptr::copy_nonoverlapping(slice.as_ptr(), (mapped as *mut u8).add(binding.offset as usize), slice.len());
        unmap_fn(p_res, 0, std::ptr::null());
    }
}

// ============================================================================
// Graph Execution Plan (for IDMLDevice1::CompileGraph)
// ============================================================================

#[derive(Clone)]
struct GraphPlan {
    input_count: usize,
    output_count: usize,
    nodes: Vec<OpSpec>,
    order: Vec<usize>,
    /// `(graph_input_idx, to_node, to_node_input)`
    input_edges: Vec<(usize, usize, usize)>,
    /// `(from_node, from_node_output, graph_output_idx)`
    output_edges: Vec<(usize, usize, usize)>,
    /// `(from_node, from_node_output, to_node, to_node_input)`
    inter_edges: Vec<(usize, usize, usize, usize)>,
}

#[derive(Clone)]
enum ExecutionPlan {
    Single(OpSpec),
    Graph(GraphPlan),
}

struct CompiledOpState {
    plan: ExecutionPlan,
    binding_props: DML_BINDING_PROPERTIES,
    /// Persistent-resource byte spans for `OWNED_BY_DML` inputs (single-op path).
    persistent_inputs: Vec<(usize, u64, u64)>,
}

// ============================================================================
// Shared DirectML COM Object Header (IDMLObject + IDMLDeviceChild)
// ============================================================================

fn register_vtable<const N: usize>(slots: [usize; N]) -> [usize; N] {
    for &addr in &slots {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
    slots
}

#[inline]
unsafe fn read_guid(p: *const Guid) -> Option<Guid> {
    if p.is_null() {
        None
    } else {
        Some(std::ptr::read_unaligned(p))
    }
}

#[inline]
unsafe fn write_null_out(pp: *mut *mut c_void) {
    if !pp.is_null() {
        std::ptr::write_unaligned(pp, std::ptr::null_mut());
    }
}

enum PrivEntry {
    Bytes(Vec<u8>),
    Iface(usize),
}

impl Drop for PrivEntry {
    fn drop(&mut self) {
        if let PrivEntry::Iface(p) = *self {
            if p != 0 {
                unsafe { com_release(p as *mut c_void) };
            }
        }
    }
}

#[repr(C)]
struct DmlObjHeader {
    vtbl: *const usize,
    refs: AtomicU32,
    private: Mutex<Vec<(Guid, PrivEntry)>>,
    name: Mutex<String>,
}

impl DmlObjHeader {
    fn new(vtbl: *const usize) -> Self {
        Self {
            vtbl,
            refs: AtomicU32::new(1),
            private: Mutex::new(Vec::new()),
            name: Mutex::new(String::new()),
        }
    }
}

/// Shared header for all `IDMLDeviceChild` implementations.
#[repr(C)]
struct DmlChildHeader {
    obj: DmlObjHeader,
    device_ptr: usize, // AddRef'd IDMLDevice1*
}

impl DmlChildHeader {
    unsafe fn new(vtbl: *const usize, device_ptr: usize) -> Self {
        if device_ptr != 0 {
            com_add_ref(device_ptr as *mut c_void);
        }
        Self {
            obj: DmlObjHeader::new(vtbl),
            device_ptr,
        }
    }
}

impl Drop for DmlChildHeader {
    fn drop(&mut self) {
        if self.device_ptr != 0 {
            unsafe { com_release(self.device_ptr as *mut c_void) };
            self.device_ptr = 0;
        }
    }
}

#[inline]
unsafe fn obj_hdr<'a>(this: usize) -> Option<&'a DmlObjHeader> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const DmlObjHeader))
    }
}

unsafe fn dml_qi_common(this: usize, riid: *const Guid, ppv: *mut *mut c_void, allowed: &[Guid]) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let (Some(h), Some(iid)) = (obj_hdr(this), read_guid(riid)) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IDMLObject || allowed.contains(&iid) {
        h.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn dml_add_ref(this: usize) -> u32 {
    let Some(h) = obj_hdr(this) else {
        return 0;
    };
    h.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dml_release<T>(this: usize) -> u32 {
    let Some(h) = obj_hdr(this) else {
        return 0;
    };
    let prev = h.refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(this as *mut T));
    }
    next
}

// IDMLObject slots 3..6: GetPrivateData, SetPrivateData, SetPrivateDataInterface, SetName
unsafe extern "win64" fn dml_get_private_data(
    this: usize,
    g: *const Guid,
    size: *mut u32,
    data: *mut c_void,
) -> HRESULT {
    let (Some(h), Some(guid_val)) = (obj_hdr(this), read_guid(g)) else {
        return E_INVALIDARG;
    };
    if size.is_null() {
        return E_INVALIDARG;
    }
    let pd = h.private.lock().unwrap();
    let Some((_, entry)) = pd.iter().find(|(k, _)| *k == guid_val) else {
        std::ptr::write_unaligned(size, 0);
        return DXGI_ERROR_NOT_FOUND;
    };
    let avail = std::ptr::read_unaligned(size) as usize;
    match entry {
        PrivEntry::Bytes(b) => {
            if data.is_null() {
                std::ptr::write_unaligned(size, b.len() as u32);
                return S_OK;
            }
            if avail < b.len() {
                std::ptr::write_unaligned(size, b.len() as u32);
                return DXGI_ERROR_MORE_DATA;
            }
            std::ptr::copy_nonoverlapping(b.as_ptr(), data as *mut u8, b.len());
            std::ptr::write_unaligned(size, b.len() as u32);
            S_OK
        }
        PrivEntry::Iface(p) => {
            if data.is_null() {
                std::ptr::write_unaligned(size, 8);
                return S_OK;
            }
            if avail < 8 {
                std::ptr::write_unaligned(size, 8);
                return DXGI_ERROR_MORE_DATA;
            }
            if *p != 0 {
                com_add_ref(*p as *mut c_void);
            }
            std::ptr::write_unaligned(data as *mut usize, *p);
            std::ptr::write_unaligned(size, 8);
            S_OK
        }
    }
}

unsafe extern "win64" fn dml_set_private_data(
    this: usize,
    g: *const Guid,
    size: u32,
    data: *const c_void,
) -> HRESULT {
    let (Some(h), Some(guid_val)) = (obj_hdr(this), read_guid(g)) else {
        return E_INVALIDARG;
    };
    let mut pd = h.private.lock().unwrap();
    if let Some(pos) = pd.iter().position(|(k, _)| *k == guid_val) {
        pd.remove(pos);
    }
    if data.is_null() || size == 0 {
        return S_OK;
    }
    let mut bytes = vec![0u8; size as usize];
    std::ptr::copy_nonoverlapping(data as *const u8, bytes.as_mut_ptr(), size as usize);
    if guid_val == WKPDID_D3DDebugObjectName {
        *h.name.lock().unwrap() = String::from_utf8_lossy(&bytes).trim_end_matches('\0').to_string();
    } else if guid_val == WKPDID_D3DDebugObjectNameW {
        let w: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        *h.name.lock().unwrap() = String::from_utf16_lossy(&w).trim_end_matches('\0').to_string();
    }
    pd.push((guid_val, PrivEntry::Bytes(bytes)));
    S_OK
}

unsafe extern "win64" fn dml_set_private_data_interface(
    this: usize,
    g: *const Guid,
    unk: *mut c_void,
) -> HRESULT {
    let (Some(h), Some(guid_val)) = (obj_hdr(this), read_guid(g)) else {
        return E_INVALIDARG;
    };
    let mut pd = h.private.lock().unwrap();
    if let Some(pos) = pd.iter().position(|(k, _)| *k == guid_val) {
        pd.remove(pos);
    }
    if !unk.is_null() {
        com_add_ref(unk);
        pd.push((guid_val, PrivEntry::Iface(unk as usize)));
    }
    S_OK
}

unsafe extern "win64" fn dml_set_name(this: usize, name: *const u16) -> HRESULT {
    let Some(h) = obj_hdr(this) else {
        return E_INVALIDARG;
    };
    if name.is_null() {
        h.name.lock().unwrap().clear();
        return dml_set_private_data(this, &WKPDID_D3DDebugObjectNameW, 0, std::ptr::null());
    }
    let mut units = Vec::new();
    let mut i = 0usize;
    loop {
        let ch = std::ptr::read_unaligned(name.add(i));
        units.push(ch);
        if ch == 0 {
            break;
        }
        i += 1;
    }
    let byte_len = (units.len() * 2) as u32;
    dml_set_private_data(this, &WKPDID_D3DDebugObjectNameW, byte_len, units.as_ptr() as *const c_void)
}

// IDMLDeviceChild slot 7: GetDevice
unsafe extern "win64" fn dml_child_get_device(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if this == 0 {
        return E_INVALIDARG;
    }
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let ch = &*(this as *const DmlChildHeader);
    if ch.device_ptr == 0 {
        return E_FAIL;
    }
    com_query(ch.device_ptr as *mut c_void, &iid, ppv)
}

// ============================================================================
// DirectML COM Object Structs & Vtables
// ============================================================================

#[repr(C)]
struct DmlDeviceObj {
    hdr: DmlObjHeader,
    d3d12_device: usize, // AddRef'd ID3D12Device*
    flags: u32,
}

impl Drop for DmlDeviceObj {
    fn drop(&mut self) {
        if self.d3d12_device != 0 {
            unsafe { com_release(self.d3d12_device as *mut c_void) };
            self.d3d12_device = 0;
        }
    }
}

#[repr(C)]
struct DmlOperatorObj {
    child: DmlChildHeader,
    spec: Arc<OpSpec>,
}

#[repr(C)]
struct DmlCompiledOperatorObj {
    child: DmlChildHeader,
    compiled: Arc<CompiledOpState>,
}

#[repr(C)]
struct DmlInitializerObj {
    child: DmlChildHeader,
    ops: Mutex<Vec<Arc<CompiledOpState>>>,
}

struct BindingTableState {
    target_ptr: usize,
    cpu_handle: usize,
    gpu_handle: u64,
    size_in_descriptors: u32,
    inputs: Vec<BindingSlot>,
    outputs: Vec<BindingSlot>,
    temp: BindingSlot,
    persistent: BindingSlot,
}

#[repr(C)]
struct DmlBindingTableObj {
    child: DmlChildHeader,
    state: Mutex<BindingTableState>,
}

#[repr(C)]
struct DmlCommandRecorderObj {
    child: DmlChildHeader,
}

// ---- IDMLOperator (8 slots) ----

const OP_IIDS: &[Guid] = &[IID_IDMLDeviceChild, IID_IDMLOperator];

unsafe extern "win64" fn op_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    dml_qi_common(this, riid, ppv, OP_IIDS)
}

fn operator_vtable() -> &'static [usize; 8] {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            op_qi as *const () as usize,
            dml_add_ref as *const () as usize,
            dml_release::<DmlOperatorObj> as *const () as usize,
            dml_get_private_data as *const () as usize,
            dml_set_private_data as *const () as usize,
            dml_set_private_data_interface as *const () as usize,
            dml_set_name as *const () as usize,
            dml_child_get_device as *const () as usize,
        ])
    })
}

unsafe fn operator_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlOperatorObj> {
    if p.is_null() {
        return None;
    }
    let vt = std::ptr::read_unaligned(p as *const *const usize);
    if vt == operator_vtable().as_ptr() {
        Some(&*(p as *const DmlOperatorObj))
    } else {
        None
    }
}

// ---- IDMLCompiledOperator (9 slots) ----

const COMPILED_IIDS: &[Guid] = &[
    IID_IDMLDeviceChild,
    IID_IDMLPageable,
    IID_IDMLDispatchable,
    IID_IDMLCompiledOperator,
];

unsafe extern "win64" fn compiled_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    dml_qi_common(this, riid, ppv, COMPILED_IIDS)
}

unsafe extern "win64" fn compiled_get_binding_props(
    this: usize,
    ret: *mut DML_BINDING_PROPERTIES,
) -> *mut DML_BINDING_PROPERTIES {
    if !ret.is_null() {
        let props = if this == 0 {
            DML_BINDING_PROPERTIES::default()
        } else {
            (&*(this as *const DmlCompiledOperatorObj)).compiled.binding_props
        };
        std::ptr::write_unaligned(ret, props);
    }
    ret
}

fn compiled_vtable() -> &'static [usize; 9] {
    static VTBL: OnceLock<[usize; 9]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            compiled_qi as *const () as usize,
            dml_add_ref as *const () as usize,
            dml_release::<DmlCompiledOperatorObj> as *const () as usize,
            dml_get_private_data as *const () as usize,
            dml_set_private_data as *const () as usize,
            dml_set_private_data_interface as *const () as usize,
            dml_set_name as *const () as usize,
            dml_child_get_device as *const () as usize,
            compiled_get_binding_props as *const () as usize,
        ])
    })
}

unsafe fn compiled_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlCompiledOperatorObj> {
    if p.is_null() {
        return None;
    }
    let vt = std::ptr::read_unaligned(p as *const *const usize);
    if vt == compiled_vtable().as_ptr() {
        Some(&*(p as *const DmlCompiledOperatorObj))
    } else {
        None
    }
}

// ---- IDMLOperatorInitializer (10 slots) ----

const INIT_IIDS: &[Guid] = &[
    IID_IDMLDeviceChild,
    IID_IDMLPageable,
    IID_IDMLDispatchable,
    IID_IDMLOperatorInitializer,
];

unsafe extern "win64" fn init_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    dml_qi_common(this, riid, ppv, INIT_IIDS)
}

unsafe extern "win64" fn init_get_binding_props(
    this: usize,
    ret: *mut DML_BINDING_PROPERTIES,
) -> *mut DML_BINDING_PROPERTIES {
    if !ret.is_null() {
        let props = if this == 0 {
            DML_BINDING_PROPERTIES::default()
        } else {
            let obj = &*(this as *const DmlInitializerObj);
            let ops = obj.ops.lock().unwrap();
            let req = ops
                .iter()
                .map(|o| o.binding_props.RequiredDescriptorCount)
                .sum::<u32>()
                .max(1);
            DML_BINDING_PROPERTIES {
                RequiredDescriptorCount: req,
                _pad: 0,
                TemporaryResourceSize: 0,
                PersistentResourceSize: 0,
            }
        };
        std::ptr::write_unaligned(ret, props);
    }
    ret
}

unsafe extern "win64" fn init_reset(
    this: usize,
    op_count: u32,
    ops_ptr: *const *mut c_void,
) -> HRESULT {
    if this == 0 {
        return E_INVALIDARG;
    }
    if op_count > 0 && ops_ptr.is_null() {
        return E_INVALIDARG;
    }
    let mut new_ops = Vec::with_capacity(op_count as usize);
    for i in 0..op_count as usize {
        let p = std::ptr::read_unaligned(ops_ptr.add(i));
        let Some(co) = compiled_from_ptr(p) else {
            return E_INVALIDARG;
        };
        new_ops.push(co.compiled.clone());
    }
    let obj = &*(this as *const DmlInitializerObj);
    *obj.ops.lock().unwrap() = new_ops;
    S_OK
}

fn initializer_vtable() -> &'static [usize; 10] {
    static VTBL: OnceLock<[usize; 10]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            init_qi as *const () as usize,
            dml_add_ref as *const () as usize,
            dml_release::<DmlInitializerObj> as *const () as usize,
            dml_get_private_data as *const () as usize,
            dml_set_private_data as *const () as usize,
            dml_set_private_data_interface as *const () as usize,
            dml_set_name as *const () as usize,
            dml_child_get_device as *const () as usize,
            init_get_binding_props as *const () as usize,
            init_reset as *const () as usize,
        ])
    })
}

unsafe fn initializer_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlInitializerObj> {
    if p.is_null() {
        return None;
    }
    let vt = std::ptr::read_unaligned(p as *const *const usize);
    if vt == initializer_vtable().as_ptr() {
        Some(&*(p as *const DmlInitializerObj))
    } else {
        None
    }
}

unsafe fn is_valid_dispatchable(p: *mut c_void) -> bool {
    compiled_from_ptr(p).is_some() || initializer_from_ptr(p).is_some()
}

// ---- IDMLBindingTable (13 slots: 0..7 DeviceChild, 8 BindInputs, 9 BindOutputs, 10 BindTemp, 11 BindPersist, 12 Reset) ----

const BT_IIDS: &[Guid] = &[IID_IDMLDeviceChild, IID_IDMLBindingTable];

unsafe fn parse_binding_desc(p: *const DML_BINDING_DESC) -> BindingSlot {
    if p.is_null() {
        return BindingSlot::None;
    }
    let bd = std::ptr::read_unaligned(p);
    match bd.Type {
        DML_BINDING_TYPE_BUFFER if !bd.Desc.is_null() => {
            let bb = std::ptr::read_unaligned(bd.Desc as *const DML_BUFFER_BINDING);
            BindingSlot::Buffer(BufferBinding {
                resource_ptr: bb.Buffer as usize,
                offset: bb.Offset,
                size: bb.SizeInBytes,
            })
        }
        DML_BINDING_TYPE_BUFFER_ARRAY if !bd.Desc.is_null() => {
            let ba = std::ptr::read_unaligned(bd.Desc as *const DML_BUFFER_ARRAY_BINDING);
            let mut list = Vec::with_capacity(ba.BindingCount as usize);
            if !ba.Bindings.is_null() {
                for i in 0..ba.BindingCount as usize {
                    let bb = std::ptr::read_unaligned(ba.Bindings.add(i));
                    list.push(BufferBinding {
                        resource_ptr: bb.Buffer as usize,
                        offset: bb.Offset,
                        size: bb.SizeInBytes,
                    });
                }
            }
            BindingSlot::BufferArray(list)
        }
        _ => BindingSlot::None,
    }
}

unsafe extern "win64" fn bt_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    dml_qi_common(this, riid, ppv, BT_IIDS)
}

unsafe extern "win64" fn bt_bind_inputs(this: usize, count: u32, bindings: *const DML_BINDING_DESC) {
    if this == 0 {
        return;
    }
    let obj = &*(this as *const DmlBindingTableObj);
    let mut list = Vec::with_capacity(count as usize);
    if !bindings.is_null() {
        for i in 0..count as usize {
            list.push(parse_binding_desc(bindings.add(i)));
        }
    }
    obj.state.lock().unwrap().inputs = list;
}

unsafe extern "win64" fn bt_bind_outputs(this: usize, count: u32, bindings: *const DML_BINDING_DESC) {
    if this == 0 {
        return;
    }
    let obj = &*(this as *const DmlBindingTableObj);
    let mut list = Vec::with_capacity(count as usize);
    if !bindings.is_null() {
        for i in 0..count as usize {
            list.push(parse_binding_desc(bindings.add(i)));
        }
    }
    obj.state.lock().unwrap().outputs = list;
}

unsafe extern "win64" fn bt_bind_temp(this: usize, binding: *const DML_BINDING_DESC) {
    if this == 0 {
        return;
    }
    let obj = &*(this as *const DmlBindingTableObj);
    obj.state.lock().unwrap().temp = parse_binding_desc(binding);
}

unsafe extern "win64" fn bt_bind_persistent(this: usize, binding: *const DML_BINDING_DESC) {
    if this == 0 {
        return;
    }
    let obj = &*(this as *const DmlBindingTableObj);
    obj.state.lock().unwrap().persistent = parse_binding_desc(binding);
}

unsafe extern "win64" fn bt_reset(this: usize, desc: *const DML_BINDING_TABLE_DESC) -> HRESULT {
    if this == 0 {
        return E_INVALIDARG;
    }
    let obj = &*(this as *const DmlBindingTableObj);
    let mut st = obj.state.lock().unwrap();
    st.inputs.clear();
    st.outputs.clear();
    st.temp = BindingSlot::None;
    st.persistent = BindingSlot::None;
    if desc.is_null() {
        st.target_ptr = 0;
        st.cpu_handle = 0;
        st.gpu_handle = 0;
        st.size_in_descriptors = 0;
        return S_OK;
    }
    let d = std::ptr::read_unaligned(desc);
    if !d.Dispatchable.is_null() {
        if !is_valid_dispatchable(d.Dispatchable) {
            return E_INVALIDARG;
        }
        st.target_ptr = d.Dispatchable as usize;
    } else {
        st.target_ptr = 0;
    }
    st.cpu_handle = d.CPUDescriptorHandle;
    st.gpu_handle = d.GPUDescriptorHandle;
    st.size_in_descriptors = d.SizeInDescriptors;
    S_OK
}

fn binding_table_vtable() -> &'static [usize; 13] {
    static VTBL: OnceLock<[usize; 13]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            bt_qi as *const () as usize,
            dml_add_ref as *const () as usize,
            dml_release::<DmlBindingTableObj> as *const () as usize,
            dml_get_private_data as *const () as usize,
            dml_set_private_data as *const () as usize,
            dml_set_private_data_interface as *const () as usize,
            dml_set_name as *const () as usize,
            dml_child_get_device as *const () as usize,
            bt_bind_inputs as *const () as usize,
            bt_bind_outputs as *const () as usize,
            bt_bind_temp as *const () as usize,
            bt_bind_persistent as *const () as usize,
            bt_reset as *const () as usize,
        ])
    })
}

unsafe fn binding_table_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlBindingTableObj> {
    if p.is_null() {
        return None;
    }
    let vt = std::ptr::read_unaligned(p as *const *const usize);
    if vt == binding_table_vtable().as_ptr() {
        Some(&*(p as *const DmlBindingTableObj))
    } else {
        None
    }
}

// ---- IDMLCommandRecorder (9 slots: 0..7 DeviceChild, 8 RecordDispatch) ----

const RECORDER_IIDS: &[Guid] = &[IID_IDMLDeviceChild, IID_IDMLCommandRecorder];

unsafe extern "win64" fn recorder_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    dml_qi_common(this, riid, ppv, RECORDER_IIDS)
}

/// Execute `IDMLOperatorInitializer` initialization: copy any `DML_TENSOR_FLAG_OWNED_BY_DML`
/// input tensors from the initializer's bound inputs into each operator's persistent output buffer.
unsafe fn run_initialize(ops: &[Arc<CompiledOpState>], bt: &BindingTableState) {
    crate::d3d12::cs::flush();
    for (op_idx, compiled) in ops.iter().enumerate() {
        if compiled.persistent_inputs.is_empty() {
            continue;
        }
        let Some(persist_dst) = bt.outputs.get(op_idx).and_then(|s| s.first_buffer()) else {
            continue;
        };
        let input_bindings: Vec<Option<&BufferBinding>> = match bt.inputs.get(op_idx) {
            Some(BindingSlot::BufferArray(arr)) => arr.iter().map(Some).collect(),
            Some(BindingSlot::Buffer(b)) => vec![Some(b)],
            _ => continue,
        };
        for &(in_slot, dst_rel_off, byte_len) in &compiled.persistent_inputs {
            if let Some(Some(src_b)) = input_bindings.get(in_slot) {
                if let Some(bytes) = read_d3d12_buffer(src_b, byte_len as usize) {
                    let sub_dst = BufferBinding {
                        resource_ptr: persist_dst.resource_ptr,
                        offset: persist_dst.offset.saturating_add(dst_rel_off),
                        size: byte_len,
                    };
                    write_d3d12_buffer(&sub_dst, &bytes[..byte_len as usize]);
                }
            }
        }
    }
}

/// Execute one compiled single operator against D3D12 buffer bindings.
unsafe fn run_single_op(compiled: &CompiledOpState, spec: &OpSpec, bt: &BindingTableState) {
    crate::d3d12::cs::flush();
    let in_descs = spec.input_descs();
    let out_descs = spec.output_descs();
    let persist_buf = bt.persistent.first_buffer();

    // Flatten bindings: allow either N separate Buffer slots OR a single BufferArray (e.g. Join).
    let flat_inputs: Vec<Option<BufferBinding>> =
        if bt.inputs.len() == 1 && in_descs.len() > 1 {
            match &bt.inputs[0] {
                BindingSlot::BufferArray(arr) => arr.iter().cloned().map(Some).collect(),
                _ => bt.inputs.iter().map(|s| s.first_buffer().cloned()).collect(),
            }
        } else {
            bt.inputs.iter().map(|s| s.first_buffer().cloned()).collect()
        };

    let mut owned_inputs: Vec<Option<Vec<u8>>> = Vec::with_capacity(in_descs.len());
    for (i, td_opt) in in_descs.iter().enumerate() {
        let Some(td) = td_opt else {
            owned_inputs.push(None);
            continue;
        };
        if let Some(Some(bb)) = flat_inputs.get(i) {
            if bb.resource_ptr != 0 {
                owned_inputs.push(read_d3d12_buffer(bb, td.total_bytes as usize));
                continue;
            }
        }
        // Fallback for OWNED_BY_DML tensor stored in persistent buffer during Initialize.
        if let Some(&(_, p_off, p_len)) = compiled.persistent_inputs.iter().find(|&&(idx, _, _)| idx == i) {
            if let Some(pb) = persist_buf {
                let sub = BufferBinding {
                    resource_ptr: pb.resource_ptr,
                    offset: pb.offset.saturating_add(p_off),
                    size: p_len,
                };
                owned_inputs.push(read_d3d12_buffer(&sub, td.total_bytes as usize));
                continue;
            }
        }
        owned_inputs.push(None);
    }

    let mut owned_outputs: Vec<Option<Vec<u8>>> = out_descs
        .iter()
        .map(|td_opt| td_opt.map(|td| vec![0u8; td.total_bytes as usize]))
        .collect();

    {
        let in_slices: Vec<Option<&[u8]>> = owned_inputs.iter().map(|v| v.as_deref()).collect();
        let mut out_slices: Vec<Option<&mut [u8]>> = owned_outputs.iter_mut().map(|v| v.as_deref_mut()).collect();
        execute_op(spec, &in_slices, &mut out_slices);
    }

    let flat_outputs: Vec<Option<BufferBinding>> =
        if bt.outputs.len() == 1 && out_descs.len() > 1 {
            match &bt.outputs[0] {
                BindingSlot::BufferArray(arr) => arr.iter().cloned().map(Some).collect(),
                _ => bt.outputs.iter().map(|s| s.first_buffer().cloned()).collect(),
            }
        } else {
            bt.outputs.iter().map(|s| s.first_buffer().cloned()).collect()
        };

    for (i, out_data) in owned_outputs.into_iter().enumerate() {
        if let (Some(bytes), Some(Some(bb))) = (out_data, flat_outputs.get(i)) {
            write_d3d12_buffer(bb, &bytes);
        }
    }
}

/// Execute a compiled multi-node DirectML graph in topological order.
unsafe fn run_graph(graph: &GraphPlan, bt: &BindingTableState) {
    crate::d3d12::cs::flush();
    // Read all graph inputs once.
    let mut graph_in_bufs: Vec<Option<Vec<u8>>> = vec![None; graph.input_count];
    for &(g_in, to_node, to_slot) in &graph.input_edges {
        if g_in < graph.input_count && graph_in_bufs[g_in].is_none() {
            let min_len = graph
                .nodes
                .get(to_node)
                .and_then(|n| n.input_descs().get(to_slot).copied().flatten())
                .map_or(0, |t| t.total_bytes as usize);
            if let Some(bb) = bt.inputs.get(g_in).and_then(|s| s.first_buffer()) {
                graph_in_bufs[g_in] = read_d3d12_buffer(bb, min_len);
            }
        }
    }

    let mut node_outputs: Vec<Vec<Option<Vec<u8>>>> = graph
        .nodes
        .iter()
        .map(|n| vec![None; n.output_descs().len()])
        .collect();

    for &node_idx in &graph.order {
        let spec = &graph.nodes[node_idx];
        let in_descs = spec.input_descs();
        let out_descs = spec.output_descs();

        let mut in_slices: Vec<Option<&[u8]>> = vec![None; in_descs.len()];
        for &(g_in, to_n, to_slot) in &graph.input_edges {
            if to_n == node_idx && to_slot < in_slices.len() {
                in_slices[to_slot] = graph_in_bufs.get(g_in).and_then(|v| v.as_deref());
            }
        }
        for &(from_n, from_slot, to_n, to_slot) in &graph.inter_edges {
            if to_n == node_idx && to_slot < in_slices.len() {
                in_slices[to_slot] = node_outputs
                    .get(from_n)
                    .and_then(|outs| outs.get(from_slot))
                    .and_then(|v| v.as_deref());
            }
        }

        let mut cur_outs: Vec<Option<Vec<u8>>> = out_descs
            .iter()
            .map(|td_opt| td_opt.map(|td| vec![0u8; td.total_bytes as usize]))
            .collect();
        {
            let mut out_slices: Vec<Option<&mut [u8]>> =
                cur_outs.iter_mut().map(|v| v.as_deref_mut()).collect();
            execute_op(spec, &in_slices, &mut out_slices);
        }
        node_outputs[node_idx] = cur_outs;
    }

    for &(from_n, from_slot, g_out) in &graph.output_edges {
        if g_out >= graph.output_count {
            continue;
        }
        if let (Some(bytes), Some(bb)) = (
            node_outputs
                .get(from_n)
                .and_then(|outs| outs.get(from_slot))
                .and_then(|v| v.as_ref()),
            bt.outputs.get(g_out).and_then(|s| s.first_buffer()),
        ) {
            write_d3d12_buffer(bb, bytes);
        }
    }
}

unsafe extern "win64" fn recorder_record_dispatch(
    this: usize,
    _command_list: *mut c_void,
    dispatchable: *mut c_void,
    bindings: *mut c_void,
) {
    if this == 0 || dispatchable.is_null() || bindings.is_null() {
        return;
    }
    let Some(bt_obj) = binding_table_from_ptr(bindings) else {
        log::warn!(target: "d3d", "IDMLCommandRecorder::RecordDispatch: foreign binding table {bindings:?}");
        return;
    };
    let bt_guard = bt_obj.state.lock().unwrap();
    log::trace!(
        target: "d3d",
        "IDMLCommandRecorder::RecordDispatch(disp={dispatchable:?}, target={:#x}, descs={}, cpu={:#x}, gpu={:#x}, temp={})",
        bt_guard.target_ptr,
        bt_guard.size_in_descriptors,
        bt_guard.cpu_handle,
        bt_guard.gpu_handle,
        bt_guard.temp.first_buffer().is_some(),
    );
    if let Some(init) = initializer_from_ptr(dispatchable) {
        let ops = init.ops.lock().unwrap().clone();
        run_initialize(&ops, &bt_guard);
    } else if let Some(co) = compiled_from_ptr(dispatchable) {
        match &co.compiled.plan {
            ExecutionPlan::Single(spec) => run_single_op(&co.compiled, spec, &bt_guard),
            ExecutionPlan::Graph(graph) => run_graph(graph, &bt_guard),
        }
    } else {
        log::warn!(target: "d3d", "IDMLCommandRecorder::RecordDispatch: unknown dispatchable {dispatchable:?}");
    }
}

fn recorder_vtable() -> &'static [usize; 9] {
    static VTBL: OnceLock<[usize; 9]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            recorder_qi as *const () as usize,
            dml_add_ref as *const () as usize,
            dml_release::<DmlCommandRecorderObj> as *const () as usize,
            dml_get_private_data as *const () as usize,
            dml_set_private_data as *const () as usize,
            dml_set_private_data_interface as *const () as usize,
            dml_set_name as *const () as usize,
            dml_child_get_device as *const () as usize,
            recorder_record_dispatch as *const () as usize,
        ])
    })
}

// ---- IDMLDevice / IDMLDevice1 (18 slots: 0..6 IDMLObject, 7..16 IDMLDevice, 17 IDMLDevice1) ----

const DEVICE_IIDS: &[Guid] = &[IID_IDMLDevice, IID_IDMLDevice1];

unsafe extern "win64" fn dev_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    dml_qi_common(this, riid, ppv, DEVICE_IIDS)
}

// Slot 7: CheckFeatureSupport
unsafe extern "win64" fn dev_check_feature_support(
    _this: usize,
    feature: u32,
    query_size: u32,
    query_data: *const c_void,
    data_size: u32,
    feature_data: *mut c_void,
) -> HRESULT {
    if query_data.is_null() || feature_data.is_null() {
        return E_INVALIDARG;
    }
    match feature {
        DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT => {
            if query_size < 4 || data_size < 4 {
                return E_INVALIDARG;
            }
            let dt = std::ptr::read_unaligned(query_data as *const u32);
            let supported = elem_byte_size(dt).is_some();
            std::ptr::write_unaligned(feature_data as *mut BOOL, if supported { TRUE } else { FALSE });
            S_OK
        }
        DML_FEATURE_FEATURE_LEVELS => {
            if query_size < std::mem::size_of::<DML_FEATURE_QUERY_FEATURE_LEVELS>() as u32 || data_size < 4 {
                return E_INVALIDARG;
            }
            let q = std::ptr::read_unaligned(query_data as *const DML_FEATURE_QUERY_FEATURE_LEVELS);
            if q.RequestedFeatureLevelCount == 0 || q.RequestedFeatureLevels.is_null() {
                return E_INVALIDARG;
            }
            let mut max_fl = 0u32;
            for i in 0..q.RequestedFeatureLevelCount as usize {
                let fl = std::ptr::read_unaligned(q.RequestedFeatureLevels.add(i));
                if fl >= DML_FEATURE_LEVEL_1_0 && fl <= MAX_SUPPORTED_FEATURE_LEVEL {
                    max_fl = max_fl.max(fl);
                }
            }
            if max_fl == 0 {
                std::ptr::write_unaligned(feature_data as *mut u32, 0);
                return DXGI_ERROR_UNSUPPORTED;
            }
            std::ptr::write_unaligned(feature_data as *mut u32, max_fl);
            S_OK
        }
        _ => E_INVALIDARG,
    }
}

// Slot 8: CreateOperator
unsafe extern "win64" fn dev_create_operator(
    this: usize,
    desc: *const DML_OPERATOR_DESC,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let spec = match OpSpec::parse(desc) {
        Ok(s) => s,
        Err(hr) => return hr,
    };
    if this != 0 && ((&*(this as *const DmlDeviceObj)).flags & DML_CREATE_DEVICE_FLAG_DEBUG) != 0 {
        log::debug!(target: "d3d", "IDMLDevice::CreateOperator({spec:?})");
    }
    let obj = Box::into_raw(Box::new(DmlOperatorObj {
        child: DmlChildHeader::new(operator_vtable().as_ptr(), this),
        spec: Arc::new(spec),
    }));
    let hr = com_query(obj as *mut c_void, &iid, ppv);
    dml_release::<DmlOperatorObj>(obj as usize);
    hr
}

// Slot 9: CompileOperator
unsafe extern "win64" fn dev_compile_operator(
    this: usize,
    op_ptr: *mut c_void,
    _flags: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let Some(op) = operator_from_ptr(op_ptr) else {
        return E_INVALIDARG;
    };
    let (persistent_inputs, persistent_size) = op.spec.persistent_layout();
    let req_descs = (op.spec.input_descs().len() + op.spec.output_descs().len()).max(1) as u32;
    let state = Arc::new(CompiledOpState {
        plan: ExecutionPlan::Single((*op.spec).clone()),
        binding_props: DML_BINDING_PROPERTIES {
            RequiredDescriptorCount: req_descs,
            _pad: 0,
            TemporaryResourceSize: 0,
            PersistentResourceSize: persistent_size,
        },
        persistent_inputs,
    });
    let obj = Box::into_raw(Box::new(DmlCompiledOperatorObj {
        child: DmlChildHeader::new(compiled_vtable().as_ptr(), this),
        compiled: state,
    }));
    let hr = com_query(obj as *mut c_void, &iid, ppv);
    dml_release::<DmlCompiledOperatorObj>(obj as usize);
    hr
}

// Slot 10: CreateOperatorInitializer
unsafe extern "win64" fn dev_create_operator_initializer(
    this: usize,
    op_count: u32,
    ops_ptr: *const *mut c_void,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if op_count > 0 && ops_ptr.is_null() {
        return E_INVALIDARG;
    }
    let mut ops = Vec::with_capacity(op_count as usize);
    for i in 0..op_count as usize {
        let p = std::ptr::read_unaligned(ops_ptr.add(i));
        let Some(co) = compiled_from_ptr(p) else {
            return E_INVALIDARG;
        };
        ops.push(co.compiled.clone());
    }
    let obj = Box::into_raw(Box::new(DmlInitializerObj {
        child: DmlChildHeader::new(initializer_vtable().as_ptr(), this),
        ops: Mutex::new(ops),
    }));
    let hr = com_query(obj as *mut c_void, &iid, ppv);
    dml_release::<DmlInitializerObj>(obj as usize);
    hr
}

// Slot 11: CreateCommandRecorder
unsafe extern "win64" fn dev_create_command_recorder(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = Box::into_raw(Box::new(DmlCommandRecorderObj {
        child: DmlChildHeader::new(recorder_vtable().as_ptr(), this),
    }));
    let hr = com_query(obj as *mut c_void, &iid, ppv);
    dml_release::<DmlCommandRecorderObj>(obj as usize);
    hr
}

// Slot 12: CreateBindingTable
unsafe extern "win64" fn dev_create_binding_table(
    this: usize,
    desc: *const DML_BINDING_TABLE_DESC,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = Box::into_raw(Box::new(DmlBindingTableObj {
        child: DmlChildHeader::new(binding_table_vtable().as_ptr(), this),
        state: Mutex::new(BindingTableState {
            target_ptr: 0,
            cpu_handle: 0,
            gpu_handle: 0,
            size_in_descriptors: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            temp: BindingSlot::None,
            persistent: BindingSlot::None,
        }),
    }));
    let rhr = bt_reset(obj as usize, desc);
    if rhr < 0 {
        dml_release::<DmlBindingTableObj>(obj as usize);
        return rhr;
    }
    let hr = com_query(obj as *mut c_void, &iid, ppv);
    dml_release::<DmlBindingTableObj>(obj as usize);
    hr
}

// Slot 13: Evict
unsafe extern "win64" fn dev_evict(_this: usize, count: u32, objects: *const *mut c_void) -> HRESULT {
    if count > 0 && objects.is_null() {
        return E_INVALIDARG;
    }
    S_OK
}

// Slot 14: MakeResident
unsafe extern "win64" fn dev_make_resident(_this: usize, count: u32, objects: *const *mut c_void) -> HRESULT {
    if count > 0 && objects.is_null() {
        return E_INVALIDARG;
    }
    S_OK
}

// Slot 15: GetDeviceRemovedReason
unsafe extern "win64" fn dev_get_device_removed_reason(this: usize) -> HRESULT {
    if this == 0 {
        return E_INVALIDARG;
    }
    let dev = &*(this as *const DmlDeviceObj);
    if dev.d3d12_device == 0 {
        return S_OK;
    }
    // ID3D12Device::GetDeviceRemovedReason is vtable slot 37.
    let d3d = dev.d3d12_device as *mut c_void;
    let vt = std::ptr::read_unaligned(d3d as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(37)));
    f(d3d)
}

// Slot 16: GetParentDevice
unsafe extern "win64" fn dev_get_parent_device(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    if this == 0 {
        return E_INVALIDARG;
    }
    let dev = &*(this as *const DmlDeviceObj);
    if dev.d3d12_device == 0 {
        return E_FAIL;
    }
    com_query(dev.d3d12_device as *mut c_void, &iid, ppv)
}

// Slot 17 (IDMLDevice1): CompileGraph
unsafe extern "win64" fn dev_compile_graph(
    this: usize,
    desc_ptr: *const DML_GRAPH_DESC,
    _flags: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if desc_ptr.is_null() {
        return E_INVALIDARG;
    }
    let gd = std::ptr::read_unaligned(desc_ptr);
    if gd.NodeCount == 0 || gd.Nodes.is_null() {
        return E_INVALIDARG;
    }
    let n_nodes = gd.NodeCount as usize;
    let mut nodes = Vec::with_capacity(n_nodes);
    for i in 0..n_nodes {
        let nd = std::ptr::read_unaligned(gd.Nodes.add(i));
        if nd.Type != DML_GRAPH_NODE_TYPE_OPERATOR || nd.Desc.is_null() {
            return E_INVALIDARG;
        }
        let ond = std::ptr::read_unaligned(nd.Desc as *const DML_OPERATOR_GRAPH_NODE_DESC);
        let Some(op) = operator_from_ptr(ond.Operator) else {
            return E_INVALIDARG;
        };
        nodes.push((*op.spec).clone());
    }

    let mut input_edges = Vec::with_capacity(gd.InputEdgeCount as usize);
    if gd.InputEdgeCount > 0 {
        if gd.InputEdges.is_null() {
            return E_INVALIDARG;
        }
        for i in 0..gd.InputEdgeCount as usize {
            let e = std::ptr::read_unaligned(gd.InputEdges.add(i));
            if e.Type != DML_GRAPH_EDGE_TYPE_INPUT || e.Desc.is_null() {
                return E_INVALIDARG;
            }
            let ie = std::ptr::read_unaligned(e.Desc as *const DML_INPUT_GRAPH_EDGE_DESC);
            if ie.GraphInputIndex >= gd.InputCount || ie.ToNodeIndex >= gd.NodeCount {
                return E_INVALIDARG;
            }
            input_edges.push((ie.GraphInputIndex as usize, ie.ToNodeIndex as usize, ie.ToNodeInputIndex as usize));
        }
    }

    let mut output_edges = Vec::with_capacity(gd.OutputEdgeCount as usize);
    if gd.OutputEdgeCount > 0 {
        if gd.OutputEdges.is_null() {
            return E_INVALIDARG;
        }
        for i in 0..gd.OutputEdgeCount as usize {
            let e = std::ptr::read_unaligned(gd.OutputEdges.add(i));
            if e.Type != DML_GRAPH_EDGE_TYPE_OUTPUT || e.Desc.is_null() {
                return E_INVALIDARG;
            }
            let oe = std::ptr::read_unaligned(e.Desc as *const DML_OUTPUT_GRAPH_EDGE_DESC);
            if oe.FromNodeIndex >= gd.NodeCount || oe.GraphOutputIndex >= gd.OutputCount {
                return E_INVALIDARG;
            }
            output_edges.push((oe.FromNodeIndex as usize, oe.FromNodeOutputIndex as usize, oe.GraphOutputIndex as usize));
        }
    }

    let mut inter_edges = Vec::with_capacity(gd.IntermediateEdgeCount as usize);
    let mut in_deg = vec![0usize; n_nodes];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n_nodes];
    if gd.IntermediateEdgeCount > 0 {
        if gd.IntermediateEdges.is_null() {
            return E_INVALIDARG;
        }
        for i in 0..gd.IntermediateEdgeCount as usize {
            let e = std::ptr::read_unaligned(gd.IntermediateEdges.add(i));
            if e.Type != DML_GRAPH_EDGE_TYPE_INTERMEDIATE || e.Desc.is_null() {
                return E_INVALIDARG;
            }
            let me = std::ptr::read_unaligned(e.Desc as *const DML_INTERMEDIATE_GRAPH_EDGE_DESC);
            if me.FromNodeIndex >= gd.NodeCount || me.ToNodeIndex >= gd.NodeCount {
                return E_INVALIDARG;
            }
            let u = me.FromNodeIndex as usize;
            let v = me.ToNodeIndex as usize;
            inter_edges.push((u, me.FromNodeOutputIndex as usize, v, me.ToNodeInputIndex as usize));
            adj[u].push(v);
            in_deg[v] += 1;
        }
    }

    // Kahn's topological sort
    let mut queue: std::collections::VecDeque<usize> =
        (0..n_nodes).filter(|&i| in_deg[i] == 0).collect();
    let mut order = Vec::with_capacity(n_nodes);
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if order.len() != n_nodes {
        return E_INVALIDARG; // Cycle detected
    }

    let req_descs = (gd.InputCount + gd.OutputCount).max(1);
    let compiled = Arc::new(CompiledOpState {
        plan: ExecutionPlan::Graph(GraphPlan {
            input_count: gd.InputCount as usize,
            output_count: gd.OutputCount as usize,
            nodes,
            order,
            input_edges,
            output_edges,
            inter_edges,
        }),
        binding_props: DML_BINDING_PROPERTIES {
            RequiredDescriptorCount: req_descs,
            _pad: 0,
            TemporaryResourceSize: 0,
            PersistentResourceSize: 0,
        },
        persistent_inputs: Vec::new(),
    });
    let obj = Box::into_raw(Box::new(DmlCompiledOperatorObj {
        child: DmlChildHeader::new(compiled_vtable().as_ptr(), this),
        compiled,
    }));
    let hr = com_query(obj as *mut c_void, &iid, ppv);
    dml_release::<DmlCompiledOperatorObj>(obj as usize);
    hr
}

fn device_vtable() -> &'static [usize; 18] {
    static VTBL: OnceLock<[usize; 18]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            dev_qi as *const () as usize,
            dml_add_ref as *const () as usize,
            dml_release::<DmlDeviceObj> as *const () as usize,
            dml_get_private_data as *const () as usize,
            dml_set_private_data as *const () as usize,
            dml_set_private_data_interface as *const () as usize,
            dml_set_name as *const () as usize,
            dev_check_feature_support as *const () as usize,
            dev_create_operator as *const () as usize,
            dev_compile_operator as *const () as usize,
            dev_create_operator_initializer as *const () as usize,
            dev_create_command_recorder as *const () as usize,
            dev_create_binding_table as *const () as usize,
            dev_evict as *const () as usize,
            dev_make_resident as *const () as usize,
            dev_get_device_removed_reason as *const () as usize,
            dev_get_parent_device as *const () as usize,
            dev_compile_graph as *const () as usize,
        ])
    })
}

// ============================================================================
// DLL Exports: DMLCreateDevice, DMLCreateDevice1, DMLCreateDeviceAndDirect3D12Device
// ============================================================================

pub unsafe extern "win64" fn DMLCreateDevice(
    d3d12_device: *mut c_void,
    flags: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    DMLCreateDevice1(d3d12_device, flags, DML_FEATURE_LEVEL_1_0, riid, ppv)
}

pub unsafe extern "win64" fn DMLCreateDevice1(
    d3d12_device: *mut c_void,
    flags: u32,
    minimum_feature_level: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if d3d12_device.is_null() || (flags & !DML_CREATE_DEVICE_FLAG_DEBUG) != 0 {
        return E_INVALIDARG;
    }
    if minimum_feature_level < DML_FEATURE_LEVEL_1_0 {
        return E_INVALIDARG;
    }
    if minimum_feature_level > MAX_SUPPORTED_FEATURE_LEVEL {
        return DXGI_ERROR_UNSUPPORTED;
    }
    com_add_ref(d3d12_device);
    let obj = Box::into_raw(Box::new(DmlDeviceObj {
        hdr: DmlObjHeader::new(device_vtable().as_ptr()),
        d3d12_device: d3d12_device as usize,
        flags,
    }));
    let hr = com_query(obj as *mut c_void, &iid, ppv);
    dml_release::<DmlDeviceObj>(obj as usize);
    log::info!(target: "d3d", "DMLCreateDevice1(flags={flags:#x}, fl={minimum_feature_level:#x}) -> {hr:#x}");
    hr
}

pub unsafe extern "win64" fn DMLCreateDeviceAndDirect3D12Device(
    adapter: *mut c_void,
    min_d3d_feature_level: u32,
    dml_flags: u32,
    min_dml_feature_level: u32,
    d3d12_riid: *const Guid,
    pp_d3d12_device: *mut *mut c_void,
    dml_riid: *const Guid,
    pp_dml_device: *mut *mut c_void,
) -> HRESULT {
    write_null_out(pp_d3d12_device);
    write_null_out(pp_dml_device);
    if pp_dml_device.is_null() {
        return E_POINTER;
    }
    let d3d_fl = if min_d3d_feature_level == 0 {
        0xB000 // D3D_FEATURE_LEVEL_11_0
    } else {
        min_d3d_feature_level
    };
    let d3d_iid = read_guid(d3d12_riid).unwrap_or(crate::d3d12::vtables::IID_ID3D12Device);
    let mut d3d_dev: *mut c_void = std::ptr::null_mut();
    let hr = crate::d3d12::D3D12CreateDevice(adapter, d3d_fl, &d3d_iid, &mut d3d_dev);
    if hr < 0 || d3d_dev.is_null() {
        return if hr < 0 { hr } else { E_OUTOFMEMORY };
    }
    let dml_fl = if min_dml_feature_level == 0 {
        DML_FEATURE_LEVEL_1_0
    } else {
        min_dml_feature_level
    };
    let dml_iid_val = read_guid(dml_riid).unwrap_or(IID_IDMLDevice);
    let dml_hr = DMLCreateDevice1(d3d_dev, dml_flags, dml_fl, &dml_iid_val, pp_dml_device);
    if dml_hr < 0 {
        com_release(d3d_dev);
        return dml_hr;
    }
    if !pp_d3d12_device.is_null() {
        std::ptr::write_unaligned(pp_d3d12_device, d3d_dev);
    } else {
        com_release(d3d_dev);
    }
    S_OK
}

// ============================================================================
// Unit Tests (must remain at the bottom for win32-rt/build.rs export scanner)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::d3d12::vtables::{IID_ID3D12Device, IID_ID3D12Resource};

    unsafe fn vslot<F: Copy>(obj: *mut c_void, idx: usize) -> F {
        let vt = std::ptr::read_unaligned(obj as *const *const usize);
        std::mem::transmute_copy(&std::ptr::read_unaligned(vt.add(idx)))
    }

    unsafe fn make_d3d12_buffer(d3d_dev: *mut c_void, bytes: u64, heap_ty: u32) -> *mut c_void {
        let create_res: unsafe extern "win64" fn(
            *mut c_void,
            *const HeapProperties,
            u32,
            *const ResourceDesc,
            u32,
            *const c_void,
            *const Guid,
            *mut *mut c_void,
        ) -> HRESULT = vslot(d3d_dev, 27);
        let hp = HeapProperties { Type: heap_ty, ..Default::default() };
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: bytes.max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let mut res = std::ptr::null_mut();
        assert_eq!(
            create_res(d3d_dev, &hp, 0, &desc, 0, std::ptr::null(), &IID_ID3D12Resource, &mut res),
            S_OK
        );
        res
    }

    unsafe fn write_f32_buf(res: *mut c_void, vals: &[f32]) {
        let bytes = std::slice::from_raw_parts(vals.as_ptr() as *const u8, vals.len() * 4);
        write_d3d12_buffer(&BufferBinding { resource_ptr: res as usize, offset: 0, size: bytes.len() as u64 }, bytes);
    }

    unsafe fn read_f32_buf(res: *mut c_void, count: usize) -> Vec<f32> {
        let bytes = read_d3d12_buffer(
            &BufferBinding { resource_ptr: res as usize, offset: 0, size: (count * 4) as u64 },
            count * 4,
        )
        .unwrap();
        bytes
            .chunks_exact(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect()
    }

    fn tensor_f32(sizes: &[u32]) -> (DML_BUFFER_TENSOR_DESC, DML_TENSOR_DESC) {
        let total = sizes.iter().fold(1u64, |a, &b| a * b as u64) * 4;
        let bd = DML_BUFFER_TENSOR_DESC {
            DataType: DML_TENSOR_DATA_TYPE_FLOAT32,
            Flags: DML_TENSOR_FLAG_NONE,
            DimensionCount: sizes.len() as u32,
            _pad0: 0,
            Sizes: sizes.as_ptr(),
            Strides: std::ptr::null(),
            TotalTensorSizeInBytes: total,
            GuaranteedBaseOffsetAlignment: 0,
            _pad1: 0,
        };
        let td = DML_TENSOR_DESC {
            Type: DML_TENSOR_TYPE_BUFFER,
            _pad: 0,
            Desc: std::ptr::null(),
        };
        (bd, td)
    }

    unsafe fn dispatch_compiled(
        dml_dev: *mut c_void,
        op_desc: &DML_OPERATOR_DESC,
        in_res: &[*mut c_void],
        in_sizes: &[u64],
        out_res: &[*mut c_void],
        out_sizes: &[u64],
    ) {
        let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
        let compile_op: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 9);
        let create_init: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 10);
        let create_rec: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 11);
        let create_bt: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 12);

        let mut op = std::ptr::null_mut();
        assert_eq!(create_op(dml_dev, op_desc, &IID_IDMLOperator, &mut op), S_OK);
        let mut compiled = std::ptr::null_mut();
        assert_eq!(compile_op(dml_dev, op, 0, &IID_IDMLCompiledOperator, &mut compiled), S_OK);

        // Verify GetBindingProperties
        let get_props: unsafe extern "win64" fn(*mut c_void, *mut DML_BINDING_PROPERTIES) -> *mut DML_BINDING_PROPERTIES = vslot(compiled, 8);
        let mut props = DML_BINDING_PROPERTIES::default();
        get_props(compiled, &mut props);
        assert!(props.RequiredDescriptorCount >= 1);

        // Run initializer through RecordDispatch
        let mut init = std::ptr::null_mut();
        assert_eq!(create_init(dml_dev, 1, &compiled, &IID_IDMLOperatorInitializer, &mut init), S_OK);
        let mut rec = std::ptr::null_mut();
        assert_eq!(create_rec(dml_dev, &IID_IDMLCommandRecorder, &mut rec), S_OK);
        let mut bt = std::ptr::null_mut();
        let bt_init_desc = DML_BINDING_TABLE_DESC {
            Dispatchable: init,
            CPUDescriptorHandle: 0,
            GPUDescriptorHandle: 0,
            SizeInDescriptors: 4,
            _pad: 0,
        };
        assert_eq!(create_bt(dml_dev, &bt_init_desc, &IID_IDMLBindingTable, &mut bt), S_OK);
        let record_dispatch: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) = vslot(rec, 8);
        record_dispatch(rec, std::ptr::null_mut(), init, bt);

        // Reset binding table for compiled op dispatch
        let bt_reset_fn: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC) -> HRESULT = vslot(bt, 12);
        let bt_bind_in: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 8);
        let bt_bind_out: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 9);

        let bt_exec_desc = DML_BINDING_TABLE_DESC {
            Dispatchable: compiled,
            CPUDescriptorHandle: 0,
            GPUDescriptorHandle: 0,
            SizeInDescriptors: props.RequiredDescriptorCount,
            _pad: 0,
        };
        assert_eq!(bt_reset_fn(bt, &bt_exec_desc), S_OK);

        let in_bbs: Vec<DML_BUFFER_BINDING> = in_res
            .iter()
            .zip(in_sizes.iter())
            .map(|(&r, &sz)| DML_BUFFER_BINDING { Buffer: r, Offset: 0, SizeInBytes: sz })
            .collect();
        let in_bds: Vec<DML_BINDING_DESC> = in_bbs
            .iter()
            .map(|bb| {
                if bb.Buffer.is_null() {
                    DML_BINDING_DESC { Type: DML_BINDING_TYPE_NONE, _pad: 0, Desc: std::ptr::null() }
                } else {
                    DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: bb as *const _ as *const c_void }
                }
            })
            .collect();
        bt_bind_in(bt, in_bds.len() as u32, in_bds.as_ptr());

        let out_bbs: Vec<DML_BUFFER_BINDING> = out_res
            .iter()
            .zip(out_sizes.iter())
            .map(|(&r, &sz)| DML_BUFFER_BINDING { Buffer: r, Offset: 0, SizeInBytes: sz })
            .collect();
        let out_bds: Vec<DML_BINDING_DESC> = out_bbs
            .iter()
            .map(|bb| DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: bb as *const _ as *const c_void })
            .collect();
        bt_bind_out(bt, out_bds.len() as u32, out_bds.as_ptr());

        record_dispatch(rec, std::ptr::null_mut(), compiled, bt);

        com_release(bt);
        com_release(rec);
        com_release(init);
        com_release(compiled);
        com_release(op);
    }

    #[test]
    fn device_creation_feature_support_and_gemm_execution() {
        unsafe {
            if d3d12_metal::metal::device().is_none() {
                return;
            }
            let mut d3d_dev = std::ptr::null_mut();
            let mut dml_dev = std::ptr::null_mut();
            assert_eq!(
                DMLCreateDeviceAndDirect3D12Device(
                    std::ptr::null_mut(),
                    0xB000,
                    DML_CREATE_DEVICE_FLAG_NONE,
                    DML_FEATURE_LEVEL_4_0,
                    &IID_ID3D12Device,
                    &mut d3d_dev,
                    &IID_IDMLDevice1,
                    &mut dml_dev,
                ),
                S_OK
            );
            assert!(!d3d_dev.is_null());
            assert!(!dml_dev.is_null());

            // CheckFeatureSupport: FLOAT32 & FLOAT16 supported, feature levels up to 5_0.
            let check_fs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const c_void, u32, *mut c_void) -> HRESULT = vslot(dml_dev, 7);
            let q_f32 = DML_TENSOR_DATA_TYPE_FLOAT32;
            let mut is_sup: BOOL = FALSE;
            assert_eq!(check_fs(dml_dev, DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT, 4, &q_f32 as *const _ as *const c_void, 4, &mut is_sup as *mut _ as *mut c_void), S_OK);
            assert_eq!(is_sup, TRUE);

            let req_fls = [DML_FEATURE_LEVEL_1_0, DML_FEATURE_LEVEL_4_0, DML_FEATURE_LEVEL_5_0];
            let q_fl = DML_FEATURE_QUERY_FEATURE_LEVELS {
                RequestedFeatureLevelCount: req_fls.len() as u32,
                _pad: 0,
                RequestedFeatureLevels: req_fls.as_ptr(),
            };
            let mut max_fl = 0u32;
            assert_eq!(
                check_fs(dml_dev, DML_FEATURE_FEATURE_LEVELS, std::mem::size_of::<DML_FEATURE_QUERY_FEATURE_LEVELS>() as u32, &q_fl as *const _ as *const c_void, 4, &mut max_fl as *mut _ as *mut c_void),
                S_OK
            );
            assert_eq!(max_fl, DML_FEATURE_LEVEL_5_0);

            // GEMM: Y = ReLU(2.0 * A(2x3) * B(3x2) + 1.0 * C(2x2)) on DEFAULT & UPLOAD D3D12 buffers!
            let a_buf = make_d3d12_buffer(d3d_dev, 24, heap_type::DEFAULT);
            let b_buf = make_d3d12_buffer(d3d_dev, 24, heap_type::UPLOAD);
            let c_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let y_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::DEFAULT);
            // A = [[1, 2, 3], [-1, 0, 2]], B = [[1, -1], [0, 2], [1, 1]], C = [[-10, 1], [0, -5]]
            // A*B = [[4, 6], [1, 3]], 2*A*B + C = [[-2, 13], [2, 1]] -> ReLU => [[0, 13], [2, 1]]
            write_f32_buf(a_buf, &[1.0, 2.0, 3.0, -1.0, 0.0, 2.0]);
            write_f32_buf(b_buf, &[1.0, -1.0, 0.0, 2.0, 1.0, 1.0]);
            write_f32_buf(c_buf, &[-10.0, 1.0, 0.0, -5.0]);

            let s_a = [1u32, 1, 2, 3];
            let s_b = [1u32, 1, 3, 2];
            let s_c = [1u32, 1, 2, 2];
            let (bd_a, mut td_a) = tensor_f32(&s_a);
            td_a.Desc = &bd_a as *const _ as *const c_void;
            let (bd_b, mut td_b) = tensor_f32(&s_b);
            td_b.Desc = &bd_b as *const _ as *const c_void;
            let (bd_c, mut td_c) = tensor_f32(&s_c);
            td_c.Desc = &bd_c as *const _ as *const c_void;
            let (bd_y, mut td_y) = tensor_f32(&s_c);
            td_y.Desc = &bd_y as *const _ as *const c_void;

            let relu_inner = DML_SIMPLE_UNARY_DESC { InputTensor: std::ptr::null(), OutputTensor: std::ptr::null() };
            let relu_desc = DML_OPERATOR_DESC {
                Type: DML_OPERATOR_ACTIVATION_RELU,
                _pad: 0,
                Desc: &relu_inner as *const _ as *const c_void,
            };
            let gemm_desc = DML_GEMM_OPERATOR_DESC {
                ATensor: &td_a,
                BTensor: &td_b,
                CTensor: &td_c,
                OutputTensor: &td_y,
                TransA: DML_MATRIX_TRANSFORM_NONE,
                TransB: DML_MATRIX_TRANSFORM_NONE,
                Alpha: 2.0,
                Beta: 1.0,
                FusedActivation: &relu_desc,
            };
            let op_desc = DML_OPERATOR_DESC {
                Type: DML_OPERATOR_GEMM,
                _pad: 0,
                Desc: &gemm_desc as *const _ as *const c_void,
            };

            dispatch_compiled(dml_dev, &op_desc, &[a_buf, b_buf, c_buf], &[24, 24, 16], &[y_buf], &[16]);
            let y = read_f32_buf(y_buf, 4);
            assert!((y[0] - 0.0).abs() < 1e-5, "y={y:?}");
            assert!((y[1] - 13.0).abs() < 1e-5, "y={y:?}");
            assert!((y[2] - 2.0).abs() < 1e-5, "y={y:?}");
            assert!((y[3] - 1.0).abs() < 1e-5, "y={y:?}");

            com_release(a_buf);
            com_release(b_buf);
            com_release(c_buf);
            com_release(y_buf);
            com_release(dml_dev);
            com_release(d3d_dev);
        }
    }

    #[test]
    fn elementwise_activations_conv2d_pool_reduce_and_norm() {
        unsafe {
            if d3d12_metal::metal::device().is_none() {
                return;
            }
            let mut d3d_dev = std::ptr::null_mut();
            let mut dml_dev = std::ptr::null_mut();
            assert_eq!(
                DMLCreateDeviceAndDirect3D12Device(
                    std::ptr::null_mut(),
                    0xB000,
                    0,
                    DML_FEATURE_LEVEL_1_0,
                    &IID_ID3D12Device,
                    &mut d3d_dev,
                    &IID_IDMLDevice1,
                    &mut dml_dev,
                ),
                S_OK
            );

            // 1) ElementWise Add + LeakyReLU
            let b_a = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let b_b = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let b_mid = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let b_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(b_a, &[1.0, -4.0, 2.5, -2.0]);
            write_f32_buf(b_b, &[3.0, 2.0, -2.5, 0.0]);
            let s4 = [1u32, 1, 1, 4];
            let (bd0, mut td0) = tensor_f32(&s4);
            td0.Desc = &bd0 as *const _ as *const c_void;
            let add_desc = DML_ELEMENT_WISE_BINARY_DESC { ATensor: &td0, BTensor: &td0, OutputTensor: &td0 };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_ADD, _pad: 0, Desc: &add_desc as *const _ as *const c_void },
                &[b_a, b_b],
                &[16, 16],
                &[b_mid],
                &[16],
            );
            assert_eq!(read_f32_buf(b_mid, 4), vec![4.0, -2.0, 0.0, -2.0]);

            let lrelu = DML_ACTIVATION_ALPHA_DESC { InputTensor: &td0, OutputTensor: &td0, Alpha: 0.25, _pad: 0 };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_LEAKY_RELU, _pad: 0, Desc: &lrelu as *const _ as *const c_void },
                &[b_mid],
                &[16],
                &[b_out],
                &[16],
            );
            assert_eq!(read_f32_buf(b_out, 4), vec![4.0, -0.5, 0.0, -0.5]);

            // 2) 2D Cross-Correlation Convolution (1x1x3x3 * 1x1x2x2 -> 1x1x2x2)
            let c_in = make_d3d12_buffer(d3d_dev, 36, heap_type::UPLOAD);
            let c_flt = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let c_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(c_in, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
            write_f32_buf(c_flt, &[1.0, 0.0, 0.0, 1.0]);
            let s_in = [1u32, 1, 3, 3];
            let s_flt = [1u32, 1, 2, 2];
            let s_co = [1u32, 1, 2, 2];
            let (bdi, mut tdi) = tensor_f32(&s_in); tdi.Desc = &bdi as *const _ as *const c_void;
            let (bdf, mut tdf) = tensor_f32(&s_flt); tdf.Desc = &bdf as *const _ as *const c_void;
            let (bdo, mut tdo) = tensor_f32(&s_co); tdo.Desc = &bdo as *const _ as *const c_void;
            let st = [1u32, 1];
            let dil = [1u32, 1];
            let pad = [0u32, 0];
            let conv = DML_CONVOLUTION_OPERATOR_DESC {
                InputTensor: &tdi,
                FilterTensor: &tdf,
                BiasTensor: std::ptr::null(),
                OutputTensor: &tdo,
                Mode: DML_CONVOLUTION_MODE_CROSS_CORRELATION,
                Direction: DML_CONVOLUTION_DIRECTION_FORWARD,
                DimensionCount: 2,
                _pad0: 0,
                Strides: st.as_ptr(),
                Dilations: dil.as_ptr(),
                StartPadding: pad.as_ptr(),
                EndPadding: pad.as_ptr(),
                OutputPadding: pad.as_ptr(),
                GroupCount: 1,
                _pad1: 0,
                FusedActivation: std::ptr::null(),
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_CONVOLUTION, _pad: 0, Desc: &conv as *const _ as *const c_void },
                &[c_in, c_flt, std::ptr::null_mut()],
                &[36, 16, 0],
                &[c_out],
                &[16],
            );
            // Windows: (1+5)=6, (2+6)=8, (4+8)=12, (5+9)=14
            assert_eq!(read_f32_buf(c_out, 4), vec![6.0, 8.0, 12.0, 14.0]);

            // 3) MaxPooling 2x2 over 2x2 output -> 1x1x1x1 = 14.0
            let p_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let s_p = [1u32, 1, 1, 1];
            let (bdp, mut tdp) = tensor_f32(&s_p); tdp.Desc = &bdp as *const _ as *const c_void;
            let win = [2u32, 2];
            let mp = DML_MAX_POOLING_OPERATOR_DESC {
                InputTensor: &tdo,
                OutputTensor: &tdp,
                DimensionCount: 2,
                _pad0: 0,
                Strides: win.as_ptr(),
                WindowSize: win.as_ptr(),
                StartPadding: pad.as_ptr(),
                EndPadding: pad.as_ptr(),
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_MAX_POOLING, _pad: 0, Desc: &mp as *const _ as *const c_void },
                &[c_out],
                &[16],
                &[p_out],
                &[4],
            );
            assert_eq!(read_f32_buf(p_out, 1), vec![14.0]);

            // 4) Reduce SUM across H, W on [6, 8, 12, 14] -> 40.0
            let r_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let axes = [2u32, 3];
            let red = DML_REDUCE_OPERATOR_DESC {
                Function: DML_REDUCE_FUNCTION_SUM,
                _pad0: 0,
                InputTensor: &tdo,
                OutputTensor: &tdp,
                AxisCount: 2,
                _pad1: 0,
                Axes: axes.as_ptr(),
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_REDUCE, _pad: 0, Desc: &red as *const _ as *const c_void },
                &[c_out],
                &[16],
                &[r_out],
                &[4],
            );
            assert_eq!(read_f32_buf(r_out, 1), vec![40.0]);

            // 5) Cast FLOAT32 -> FLOAT16 -> FLOAT32 round-trip
            let f16_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let rt_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let bd_f16 = DML_BUFFER_TENSOR_DESC {
                DataType: DML_TENSOR_DATA_TYPE_FLOAT16,
                Flags: 0,
                DimensionCount: 4,
                _pad0: 0,
                Sizes: s4.as_ptr(),
                Strides: std::ptr::null(),
                TotalTensorSizeInBytes: 8,
                GuaranteedBaseOffsetAlignment: 0,
                _pad1: 0,
            };
            let td_f16 = DML_TENSOR_DESC { Type: DML_TENSOR_TYPE_BUFFER, _pad: 0, Desc: &bd_f16 as *const _ as *const c_void };
            let cast1 = DML_SIMPLE_UNARY_DESC { InputTensor: &td0, OutputTensor: &td_f16 };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_CAST, _pad: 0, Desc: &cast1 as *const _ as *const c_void },
                &[b_out],
                &[16],
                &[f16_buf],
                &[8],
            );
            let cast2 = DML_SIMPLE_UNARY_DESC { InputTensor: &td_f16, OutputTensor: &td0 };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_CAST, _pad: 0, Desc: &cast2 as *const _ as *const c_void },
                &[f16_buf],
                &[8],
                &[rt_buf],
                &[16],
            );
            assert_eq!(read_f32_buf(rt_buf, 4), vec![4.0, -0.5, 0.0, -0.5]);

            // 6) BatchNormalization: scale * (x - mean) / sqrt(var + eps) + bias
            let bn_x = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let bn_m = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let bn_v = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let bn_s = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let bn_b = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let bn_y = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(bn_x, &[2.0, 4.0, 6.0, 8.0]);
            write_f32_buf(bn_m, &[4.0]);
            write_f32_buf(bn_v, &[4.0]); // sqrt(4) = 2
            write_f32_buf(bn_s, &[2.0]);
            write_f32_buf(bn_b, &[1.0]);
            // y = 2*(x - 4)/2 + 1 = x - 3 => [-1, 1, 3, 5]
            let bn_desc = DML_BATCH_NORMALIZATION_OPERATOR_DESC {
                InputTensor: &td0,
                MeanTensor: &tdp,
                VarianceTensor: &tdp,
                ScaleTensor: &tdp,
                BiasTensor: &tdp,
                OutputTensor: &td0,
                Spatial: TRUE,
                Epsilon: 0.0,
                FusedActivation: std::ptr::null(),
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_BATCH_NORMALIZATION, _pad: 0, Desc: &bn_desc as *const _ as *const c_void },
                &[bn_x, bn_m, bn_v, bn_s, bn_b],
                &[16, 4, 4, 4, 4],
                &[bn_y],
                &[16],
            );
            let bny = read_f32_buf(bn_y, 4);
            for (got, exp) in bny.iter().zip([-1.0f32, 1.0, 3.0, 5.0]) {
                assert!((got - exp).abs() < 1e-5, "bny={bny:?}");
            }

            // 7) IDMLDevice1::CompileGraph: Negate -> Exp
            let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
            let compile_graph: unsafe extern "win64" fn(*mut c_void, *const DML_GRAPH_DESC, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 17);
            let create_rec: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 11);
            let create_bt: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 12);

            let neg_d = DML_SIMPLE_UNARY_DESC { InputTensor: &td0, OutputTensor: &td0 };
            let exp_d = DML_ELEMENT_WISE_UNARY_DESC { InputTensor: &td0, OutputTensor: &td0, ScaleBias: std::ptr::null() };
            let mut op_neg = std::ptr::null_mut();
            let mut op_exp = std::ptr::null_mut();
            assert_eq!(create_op(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_NEGATE, _pad: 0, Desc: &neg_d as *const _ as *const c_void }, &IID_IDMLOperator, &mut op_neg), S_OK);
            assert_eq!(create_op(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_EXP, _pad: 0, Desc: &exp_d as *const _ as *const c_void }, &IID_IDMLOperator, &mut op_exp), S_OK);

            let n0 = DML_OPERATOR_GRAPH_NODE_DESC { Operator: op_neg, Name: std::ptr::null() };
            let n1 = DML_OPERATOR_GRAPH_NODE_DESC { Operator: op_exp, Name: std::ptr::null() };
            let nodes = [
                DML_GRAPH_NODE_DESC { Type: DML_GRAPH_NODE_TYPE_OPERATOR, _pad: 0, Desc: &n0 as *const _ as *const c_void },
                DML_GRAPH_NODE_DESC { Type: DML_GRAPH_NODE_TYPE_OPERATOR, _pad: 0, Desc: &n1 as *const _ as *const c_void },
            ];
            let ie = DML_INPUT_GRAPH_EDGE_DESC { GraphInputIndex: 0, ToNodeIndex: 0, ToNodeInputIndex: 0, _pad: 0, Name: std::ptr::null() };
            let me = DML_INTERMEDIATE_GRAPH_EDGE_DESC { FromNodeIndex: 0, FromNodeOutputIndex: 0, ToNodeIndex: 1, ToNodeInputIndex: 0, Name: std::ptr::null() };
            let oe = DML_OUTPUT_GRAPH_EDGE_DESC { FromNodeIndex: 1, FromNodeOutputIndex: 0, GraphOutputIndex: 0, _pad: 0, Name: std::ptr::null() };
            let in_edges = [DML_GRAPH_EDGE_DESC { Type: DML_GRAPH_EDGE_TYPE_INPUT, _pad: 0, Desc: &ie as *const _ as *const c_void }];
            let inter_edges = [DML_GRAPH_EDGE_DESC { Type: DML_GRAPH_EDGE_TYPE_INTERMEDIATE, _pad: 0, Desc: &me as *const _ as *const c_void }];
            let out_edges = [DML_GRAPH_EDGE_DESC { Type: DML_GRAPH_EDGE_TYPE_OUTPUT, _pad: 0, Desc: &oe as *const _ as *const c_void }];
            let gdesc = DML_GRAPH_DESC {
                InputCount: 1,
                OutputCount: 1,
                NodeCount: 2,
                _pad0: 0,
                Nodes: nodes.as_ptr(),
                InputEdgeCount: 1,
                _pad1: 0,
                InputEdges: in_edges.as_ptr(),
                OutputEdgeCount: 1,
                _pad2: 0,
                OutputEdges: out_edges.as_ptr(),
                IntermediateEdgeCount: 1,
                _pad3: 0,
                IntermediateEdges: inter_edges.as_ptr(),
            };
            let mut gcomp = std::ptr::null_mut();
            assert_eq!(compile_graph(dml_dev, &gdesc, 0, &IID_IDMLCompiledOperator, &mut gcomp), S_OK);
            let mut rec = std::ptr::null_mut();
            assert_eq!(create_rec(dml_dev, &IID_IDMLCommandRecorder, &mut rec), S_OK);
            let mut bt = std::ptr::null_mut();
            let btd = DML_BINDING_TABLE_DESC { Dispatchable: gcomp, CPUDescriptorHandle: 0, GPUDescriptorHandle: 0, SizeInDescriptors: 2, _pad: 0 };
            assert_eq!(create_bt(dml_dev, &btd, &IID_IDMLBindingTable, &mut bt), S_OK);

            let g_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let g_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(g_in, &[0.0, 1.0, -1.0, 2.0]);
            let bb_in = DML_BUFFER_BINDING { Buffer: g_in, Offset: 0, SizeInBytes: 16 };
            let bb_out = DML_BUFFER_BINDING { Buffer: g_out, Offset: 0, SizeInBytes: 16 };
            let bd_in = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &bb_in as *const _ as *const c_void };
            let bd_out = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &bb_out as *const _ as *const c_void };
            let bind_in: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 8);
            let bind_out: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 9);
            let rec_disp: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) = vslot(rec, 8);
            bind_in(bt, 1, &bd_in);
            bind_out(bt, 1, &bd_out);
            rec_disp(rec, std::ptr::null_mut(), gcomp, bt);

            let gy = read_f32_buf(g_out, 4);
            let expected = [0.0f32.exp(), (-1.0f32).exp(), 1.0f32.exp(), (-2.0f32).exp()];
            for (got, exp) in gy.iter().zip(expected) {
                assert!((got - exp).abs() < 1e-5, "gy={gy:?}");
            }

            for p in [g_in, g_out, bt, rec, gcomp, op_neg, op_exp, bn_x, bn_m, bn_v, bn_s, bn_b, bn_y, f16_buf, rt_buf, r_out, p_out, c_in, c_flt, c_out, b_a, b_b, b_mid, b_out, dml_dev, d3d_dev] {
                com_release(p);
            }
        }
    }

    #[test]
    fn join_split_avgpool_mvn_clip_sigmoid_tanh_and_owned_persistent_init() {
        unsafe {
            if d3d12_metal::metal::device().is_none() {
                return;
            }
            let mut d3d_dev = std::ptr::null_mut();
            let mut dml_dev = std::ptr::null_mut();
            assert_eq!(
                DMLCreateDeviceAndDirect3D12Device(
                    std::ptr::null_mut(),
                    0xB000,
                    0,
                    DML_FEATURE_LEVEL_1_0,
                    &IID_ID3D12Device,
                    &mut d3d_dev,
                    &IID_IDMLDevice1,
                    &mut dml_dev,
                ),
                S_OK
            );

            // 1) Join two [1, 2] tensors along axis 1 -> [1, 4], then Split back into two [1, 2]
            let j0 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let j1 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let joined = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let sp0 = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let sp1 = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(j0, &[10.0, 20.0]);
            write_f32_buf(j1, &[30.0, 40.0]);
            let s12 = [1u32, 2];
            let s14 = [1u32, 4];
            let (bd12, mut td12) = tensor_f32(&s12); td12.Desc = &bd12 as *const _ as *const c_void;
            let (bd14, mut td14) = tensor_f32(&s14); td14.Desc = &bd14 as *const _ as *const c_void;
            let in_tds = [td12, td12];
            let join_desc = DML_JOIN_OPERATOR_DESC {
                InputCount: 2,
                _pad0: 0,
                InputTensors: in_tds.as_ptr(),
                OutputTensor: &td14,
                Axis: 1,
                _pad1: 0,
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_JOIN, _pad: 0, Desc: &join_desc as *const _ as *const c_void },
                &[j0, j1],
                &[8, 8],
                &[joined],
                &[16],
            );
            assert_eq!(read_f32_buf(joined, 4), vec![10.0, 20.0, 30.0, 40.0]);

            let split_desc = DML_SPLIT_OPERATOR_DESC {
                InputTensor: &td14,
                OutputCount: 2,
                _pad0: 0,
                OutputTensors: in_tds.as_ptr(),
                Axis: 1,
                _pad1: 0,
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_SPLIT, _pad: 0, Desc: &split_desc as *const _ as *const c_void },
                &[joined],
                &[16],
                &[sp0, sp1],
                &[8, 8],
            );
            assert_eq!(read_f32_buf(sp0, 2), vec![10.0, 20.0]);
            assert_eq!(read_f32_buf(sp1, 2), vec![30.0, 40.0]);

            // 2) AveragePooling 2x2 over [1, 1, 2, 2] = [2, 4, 6, 8] -> 5.0
            let ap_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let ap_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(ap_in, &[2.0, 4.0, 6.0, 8.0]);
            let s22 = [1u32, 1, 2, 2];
            let s11 = [1u32, 1, 1, 1];
            let (bd22, mut td22) = tensor_f32(&s22); td22.Desc = &bd22 as *const _ as *const c_void;
            let (bd11, mut td11) = tensor_f32(&s11); td11.Desc = &bd11 as *const _ as *const c_void;
            let win = [2u32, 2];
            let pad = [0u32, 0];
            let ap_desc = DML_AVERAGE_POOLING_OPERATOR_DESC {
                InputTensor: &td22,
                OutputTensor: &td11,
                DimensionCount: 2,
                _pad0: 0,
                Strides: win.as_ptr(),
                WindowSize: win.as_ptr(),
                StartPadding: pad.as_ptr(),
                EndPadding: pad.as_ptr(),
                IncludePadding: FALSE,
                _pad1: 0,
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_AVERAGE_POOLING, _pad: 0, Desc: &ap_desc as *const _ as *const c_void },
                &[ap_in],
                &[16],
                &[ap_out],
                &[4],
            );
            assert_eq!(read_f32_buf(ap_out, 1), vec![5.0]);

            // 3) MeanVarianceNormalization over [1, 1, 2, 2] = [1, 3, 5, 7] (mean=4, var=5)
            let mvn_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(ap_in, &[1.0, 3.0, 5.0, 7.0]);
            let mvn_desc = DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
                InputTensor: &td22,
                ScaleTensor: std::ptr::null(),
                BiasTensor: std::ptr::null(),
                OutputTensor: &td22,
                CrossChannel: FALSE,
                NormalizeVariance: TRUE,
                Epsilon: 0.0,
                _pad0: 0,
                FusedActivation: std::ptr::null(),
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION, _pad: 0, Desc: &mvn_desc as *const _ as *const c_void },
                &[ap_in, std::ptr::null_mut(), std::ptr::null_mut()],
                &[16, 0, 0],
                &[mvn_out],
                &[16],
            );
            let mv = read_f32_buf(mvn_out, 4);
            let inv_s = 1.0f32 / 5.0f32.sqrt();
            for (got, exp) in mv.iter().zip([-3.0 * inv_s, -inv_s, inv_s, 3.0 * inv_s]) {
                assert!((got - exp).abs() < 1e-5, "mv={mv:?}");
            }

            // 4) Clip, Sigmoid, Tanh, Identity+ScaleBias, Sub, Mul, Div, Abs, Sqrt, Log
            let u_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let u_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(u_in, &[-2.0, 0.0, 1.0, 5.0]);
            let clip_d = DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
                InputTensor: &td14,
                OutputTensor: &td14,
                ScaleBias: std::ptr::null(),
                Min: -1.0,
                Max: 2.0,
            };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_CLIP, _pad: 0, Desc: &clip_d as *const _ as *const c_void },
                &[u_in],
                &[16],
                &[u_out],
                &[16],
            );
            assert_eq!(read_f32_buf(u_out, 4), vec![-1.0, 0.0, 1.0, 2.0]);

            let sig_d = DML_SIMPLE_UNARY_DESC { InputTensor: &td14, OutputTensor: &td14 };
            dispatch_compiled(
                dml_dev,
                &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_SIGMOID, _pad: 0, Desc: &sig_d as *const _ as *const c_void },
                &[u_out],
                &[16],
                &[u_in],
                &[16],
            );
            let sv = read_f32_buf(u_in, 4);
            assert!((sv[1] - 0.5).abs() < 1e-5);

            // 5) OWNED_BY_DML persistent resource initialization via IDMLOperatorInitializer
            let w_src = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let x_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let p_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::DEFAULT);
            let y_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            write_f32_buf(x_in, &[2.0, 3.0, 4.0, 5.0]);
            write_f32_buf(w_src, &[10.0, 10.0, 10.0, 10.0]);
            let bd_owned = DML_BUFFER_TENSOR_DESC {
                DataType: DML_TENSOR_DATA_TYPE_FLOAT32,
                Flags: DML_TENSOR_FLAG_OWNED_BY_DML,
                DimensionCount: 2,
                _pad0: 0,
                Sizes: s14.as_ptr(),
                Strides: std::ptr::null(),
                TotalTensorSizeInBytes: 16,
                GuaranteedBaseOffsetAlignment: 0,
                _pad1: 0,
            };
            let td_owned = DML_TENSOR_DESC { Type: DML_TENSOR_TYPE_BUFFER, _pad: 0, Desc: &bd_owned as *const _ as *const c_void };
            let mul_d = DML_ELEMENT_WISE_BINARY_DESC { ATensor: &td14, BTensor: &td_owned, OutputTensor: &td14 };
            let op_desc = DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_MULTIPLY, _pad: 0, Desc: &mul_d as *const _ as *const c_void };

            let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
            let compile_op: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 9);
            let create_init: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 10);
            let create_rec: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 11);
            let create_bt: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 12);

            let mut op = std::ptr::null_mut();
            let mut co = std::ptr::null_mut();
            let mut init = std::ptr::null_mut();
            let mut rec = std::ptr::null_mut();
            let mut bt = std::ptr::null_mut();
            assert_eq!(create_op(dml_dev, &op_desc, &IID_IDMLOperator, &mut op), S_OK);
            assert_eq!(compile_op(dml_dev, op, 0, &IID_IDMLCompiledOperator, &mut co), S_OK);
            let get_props: unsafe extern "win64" fn(*mut c_void, *mut DML_BINDING_PROPERTIES) -> *mut DML_BINDING_PROPERTIES = vslot(co, 8);
            let mut props = DML_BINDING_PROPERTIES::default();
            get_props(co, &mut props);
            assert_eq!(props.PersistentResourceSize, 16);

            assert_eq!(create_init(dml_dev, 1, &co, &IID_IDMLOperatorInitializer, &mut init), S_OK);
            assert_eq!(create_rec(dml_dev, &IID_IDMLCommandRecorder, &mut rec), S_OK);
            let btd_i = DML_BINDING_TABLE_DESC { Dispatchable: init, CPUDescriptorHandle: 0, GPUDescriptorHandle: 0, SizeInDescriptors: 4, _pad: 0 };
            assert_eq!(create_bt(dml_dev, &btd_i, &IID_IDMLBindingTable, &mut bt), S_OK);
            let bind_in: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 8);
            let bind_out: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 9);
            let bind_pers: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_DESC) = vslot(bt, 11);
            let bt_reset_fn: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC) -> HRESULT = vslot(bt, 12);
            let rec_disp: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) = vslot(rec, 8);

            // Bind initializer inputs (BufferArray [null, w_src]) and output (persistent p_buf)
            let arr_bbs = [
                DML_BUFFER_BINDING { Buffer: std::ptr::null_mut(), Offset: 0, SizeInBytes: 0 },
                DML_BUFFER_BINDING { Buffer: w_src, Offset: 0, SizeInBytes: 16 },
            ];
            let arr_b = DML_BUFFER_ARRAY_BINDING { BindingCount: 2, _pad: 0, Bindings: arr_bbs.as_ptr() };
            let init_in = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER_ARRAY, _pad: 0, Desc: &arr_b as *const _ as *const c_void };
            let p_bb = DML_BUFFER_BINDING { Buffer: p_buf, Offset: 0, SizeInBytes: 16 };
            let p_bd = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &p_bb as *const _ as *const c_void };
            bind_in(bt, 1, &init_in);
            bind_out(bt, 1, &p_bd);
            rec_disp(rec, std::ptr::null_mut(), init, bt);

            // Now dispatch compiled op with input 1 = NONE (reads owned weights from persistent buffer!)
            let btd_e = DML_BINDING_TABLE_DESC { Dispatchable: co, CPUDescriptorHandle: 0, GPUDescriptorHandle: 0, SizeInDescriptors: 4, _pad: 0 };
            assert_eq!(bt_reset_fn(bt, &btd_e), S_OK);
            let x_bb = DML_BUFFER_BINDING { Buffer: x_in, Offset: 0, SizeInBytes: 16 };
            let y_bb = DML_BUFFER_BINDING { Buffer: y_out, Offset: 0, SizeInBytes: 16 };
            let exec_ins = [
                DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &x_bb as *const _ as *const c_void },
                DML_BINDING_DESC { Type: DML_BINDING_TYPE_NONE, _pad: 0, Desc: std::ptr::null() },
            ];
            let exec_out = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &y_bb as *const _ as *const c_void };
            bind_in(bt, 2, exec_ins.as_ptr());
            bind_out(bt, 1, &exec_out);
            bind_pers(bt, &p_bd);
            rec_disp(rec, std::ptr::null_mut(), co, bt);
            assert_eq!(read_f32_buf(y_out, 4), vec![20.0, 30.0, 40.0, 50.0]);

            for p in [bt, rec, init, co, op, w_src, x_in, p_buf, y_out, u_in, u_out, mvn_out, ap_in, ap_out, j0, j1, joined, sp0, sp1, dml_dev, d3d_dev] {
                com_release(p);
            }
        }
    }

    #[test]
    fn device_errors_object_metadata_and_extra_operators() {
        unsafe {
            if d3d12_metal::metal::device().is_none() {
                return;
            }
            let mut out = std::ptr::null_mut();
            assert_eq!(DMLCreateDevice(std::ptr::null_mut(), 0, &IID_IDMLDevice, &mut out), E_INVALIDARG);

            let mut d3d_dev = std::ptr::null_mut();
            let mut dml_dev = std::ptr::null_mut();
            assert_eq!(
                DMLCreateDeviceAndDirect3D12Device(
                    std::ptr::null_mut(),
                    0xB000,
                    DML_CREATE_DEVICE_FLAG_DEBUG,
                    DML_FEATURE_LEVEL_1_0,
                    &IID_ID3D12Device,
                    &mut d3d_dev,
                    &IID_IDMLDevice1,
                    &mut dml_dev,
                ),
                S_OK
            );
            assert_eq!(
                DMLCreateDevice1(d3d_dev, 0, 0x9000, &IID_IDMLDevice1, &mut out),
                DXGI_ERROR_UNSUPPORTED
            );

            // IDMLObject: SetName, SetPrivateData, GetPrivateData, GetDeviceRemovedReason, GetParentDevice
            let get_pd: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32, *mut c_void) -> HRESULT = vslot(dml_dev, 3);
            let set_pd: unsafe extern "win64" fn(*mut c_void, *const Guid, u32, *const c_void) -> HRESULT = vslot(dml_dev, 4);
            let set_name: unsafe extern "win64" fn(*mut c_void, *const u16) -> HRESULT = vslot(dml_dev, 6);
            let get_rem: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(dml_dev, 15);
            let get_parent: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 16);

            assert_eq!(get_rem(dml_dev), S_OK);
            let mut parent = std::ptr::null_mut();
            assert_eq!(get_parent(dml_dev, &IID_ID3D12Device, &mut parent), S_OK);
            assert!(!parent.is_null());
            com_release(parent);

            let wname: Vec<u16> = "TestDML\0".encode_utf16().collect();
            assert_eq!(set_name(dml_dev, wname.as_ptr()), S_OK);
            let pk = guid("33333333-4444-5555-6666-777777777777");
            let payload = [9u8, 8, 7, 6];
            assert_eq!(set_pd(dml_dev, &pk, 4, payload.as_ptr() as *const c_void), S_OK);
            let mut psz = 2u32;
            let mut pbuf = [0u8; 4];
            assert_eq!(get_pd(dml_dev, &pk, &mut psz, pbuf.as_mut_ptr() as *mut c_void), DXGI_ERROR_MORE_DATA);
            assert_eq!(psz, 4);
            assert_eq!(get_pd(dml_dev, &pk, &mut psz, pbuf.as_mut_ptr() as *mut c_void), S_OK);
            assert_eq!(pbuf, payload);

            // Identity + ScaleBias, Sub, Div, Abs, Sqrt, Log, Tanh, Softmax
            let b0 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let b1 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
            let bo = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let s4 = [1u32, 1, 1, 4];
            let (bd4, mut td4) = tensor_f32(&s4);
            td4.Desc = &bd4 as *const _ as *const c_void;

            write_f32_buf(b0, &[1.0, 2.0, -3.0, 4.0]);
            let sb = DML_SCALE_BIAS { Scale: 2.0, Bias: -1.0 };
            let id_d = DML_ELEMENT_WISE_UNARY_DESC { InputTensor: &td4, OutputTensor: &td4, ScaleBias: &sb };
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_IDENTITY, _pad: 0, Desc: &id_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
            assert_eq!(read_f32_buf(bo, 4), vec![1.0, 3.0, -7.0, 7.0]);

            write_f32_buf(b1, &[1.0, 2.0, 2.0, 4.0]);
            let bin_d = DML_ELEMENT_WISE_BINARY_DESC { ATensor: &td4, BTensor: &td4, OutputTensor: &td4 };
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_SUBTRACT, _pad: 0, Desc: &bin_d as *const _ as *const c_void }, &[b0, b1], &[16, 16], &[bo], &[16]);
            assert_eq!(read_f32_buf(bo, 4), vec![0.0, 0.0, -5.0, 0.0]);
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_DIVIDE, _pad: 0, Desc: &bin_d as *const _ as *const c_void }, &[b0, b1], &[16, 16], &[bo], &[16]);
            assert_eq!(read_f32_buf(bo, 4), vec![1.0, 1.0, -1.5, 1.0]);

            let u_d = DML_ELEMENT_WISE_UNARY_DESC { InputTensor: &td4, OutputTensor: &td4, ScaleBias: std::ptr::null() };
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_ABS, _pad: 0, Desc: &u_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
            assert_eq!(read_f32_buf(bo, 4), vec![1.0, 2.0, 3.0, 4.0]);

            write_f32_buf(b0, &[1.0, 4.0, 9.0, 16.0]);
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_SQRT, _pad: 0, Desc: &u_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
            assert_eq!(read_f32_buf(bo, 4), vec![1.0, 2.0, 3.0, 4.0]);

            write_f32_buf(b0, &[1.0, std::f32::consts::E, 1.0, 1.0]);
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_LOG, _pad: 0, Desc: &u_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
            let lg = read_f32_buf(bo, 4);
            assert!(lg[0].abs() < 1e-5 && (lg[1] - 1.0).abs() < 1e-5);

            write_f32_buf(b0, &[0.0, 0.0, 0.0, 0.0]);
            let su_d = DML_SIMPLE_UNARY_DESC { InputTensor: &td4, OutputTensor: &td4 };
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_SOFTMAX, _pad: 0, Desc: &su_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
            assert_eq!(read_f32_buf(bo, 4), vec![0.25, 0.25, 0.25, 0.25]);

            write_f32_buf(b0, &[0.0, 1.0, -1.0, 0.5]);
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_TANH, _pad: 0, Desc: &su_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
            let th = read_f32_buf(bo, 4);
            assert!(th[0].abs() < 1e-5 && (th[1] - 1.0f32.tanh()).abs() < 1e-5);

            // GEMM with TransA = TRANSPOSE, TransB = TRANSPOSE: A(3x2)^T * B(2x3)^T = (2x2)
            let ga = make_d3d12_buffer(d3d_dev, 24, heap_type::UPLOAD);
            let gb = make_d3d12_buffer(d3d_dev, 24, heap_type::UPLOAD);
            let gy = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            // A = [[1, 4], [2, 5], [3, 6]] -> A^T = [[1, 2, 3], [4, 5, 6]]
            // B = [[1, 0, 1], [0, 1, 0]] -> B^T = [[1, 0], [0, 1], [1, 0]]
            // A^T * B^T = [[4, 2], [10, 5]]
            write_f32_buf(ga, &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
            write_f32_buf(gb, &[1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
            let s32 = [1u32, 1, 3, 2];
            let s23 = [1u32, 1, 2, 3];
            let s22 = [1u32, 1, 2, 2];
            let (bda, mut tda) = tensor_f32(&s32); tda.Desc = &bda as *const _ as *const c_void;
            let (bdb, mut tdb) = tensor_f32(&s23); tdb.Desc = &bdb as *const _ as *const c_void;
            let (bdy, mut tdy) = tensor_f32(&s22); tdy.Desc = &bdy as *const _ as *const c_void;
            let gemm_t = DML_GEMM_OPERATOR_DESC {
                ATensor: &tda,
                BTensor: &tdb,
                CTensor: std::ptr::null(),
                OutputTensor: &tdy,
                TransA: DML_MATRIX_TRANSFORM_TRANSPOSE,
                TransB: DML_MATRIX_TRANSFORM_TRANSPOSE,
                Alpha: 1.0,
                Beta: 0.0,
                FusedActivation: std::ptr::null(),
            };
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_GEMM, _pad: 0, Desc: &gemm_t as *const _ as *const c_void }, &[ga, gb, std::ptr::null_mut()], &[24, 24, 0], &[gy], &[16]);
            assert_eq!(read_f32_buf(gy, 4), vec![4.0, 2.0, 10.0, 5.0]);

            // MaxPooling1 with UINT32 output indices
            let mp_val = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let mp_idx = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
            let s11 = [1u32, 1, 1, 1];
            let (bd11, mut td11) = tensor_f32(&s11); td11.Desc = &bd11 as *const _ as *const c_void;
            let bd_u32 = DML_BUFFER_TENSOR_DESC {
                DataType: DML_TENSOR_DATA_TYPE_UINT32,
                Flags: 0,
                DimensionCount: 4,
                _pad0: 0,
                Sizes: s11.as_ptr(),
                Strides: std::ptr::null(),
                TotalTensorSizeInBytes: 4,
                GuaranteedBaseOffsetAlignment: 0,
                _pad1: 0,
            };
            let td_u32 = DML_TENSOR_DESC { Type: DML_TENSOR_TYPE_BUFFER, _pad: 0, Desc: &bd_u32 as *const _ as *const c_void };
            let win2 = [2u32, 2];
            let pad0 = [0u32, 0];
            let mp1 = DML_MAX_POOLING1_OPERATOR_DESC {
                InputTensor: &tdy,
                OutputTensor: &td11,
                OutputIndicesTensor: &td_u32,
                DimensionCount: 2,
                _pad0: 0,
                Strides: win2.as_ptr(),
                WindowSize: win2.as_ptr(),
                StartPadding: pad0.as_ptr(),
                EndPadding: pad0.as_ptr(),
            };
            // gy = [4, 2, 10, 5] -> max = 10 at flat spatial index 2
            dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_MAX_POOLING1, _pad: 0, Desc: &mp1 as *const _ as *const c_void }, &[gy], &[16], &[mp_val, mp_idx], &[4, 4]);
            assert_eq!(read_f32_buf(mp_val, 1), vec![10.0]);
            let idx_bytes = read_d3d12_buffer(&BufferBinding { resource_ptr: mp_idx as usize, offset: 0, size: 4 }, 4).unwrap();
            assert_eq!(u32::from_le_bytes([idx_bytes[0], idx_bytes[1], idx_bytes[2], idx_bytes[3]]), 2);

            // IDMLOperatorInitializer::Reset + IDMLDeviceChild::GetDevice
            let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
            let compile_op: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 9);
            let create_init: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 10);
            let mut op = std::ptr::null_mut();
            let mut co = std::ptr::null_mut();
            let mut init = std::ptr::null_mut();
            assert_eq!(create_op(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_TANH, _pad: 0, Desc: &su_d as *const _ as *const c_void }, &IID_IDMLOperator, &mut op), S_OK);
            assert_eq!(compile_op(dml_dev, op, 0, &IID_IDMLCompiledOperator, &mut co), S_OK);
            assert_eq!(create_init(dml_dev, 0, std::ptr::null(), &IID_IDMLOperatorInitializer, &mut init), S_OK);
            let init_reset_fn: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void) -> HRESULT = vslot(init, 9);
            assert_eq!(init_reset_fn(init, 1, &co), S_OK);
            let child_get_dev: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(op, 7);
            let mut got_dev = std::ptr::null_mut();
            assert_eq!(child_get_dev(op, &IID_IDMLDevice1, &mut got_dev), S_OK);
            assert_eq!(got_dev, dml_dev);
            com_release(got_dev);

            for p in [init, co, op, mp_val, mp_idx, ga, gb, gy, b0, b1, bo, dml_dev, d3d_dev] {
                com_release(p);
            }
        }
    }
}
