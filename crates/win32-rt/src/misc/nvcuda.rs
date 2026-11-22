//! `nvcuda.dll`: NVIDIA CUDA Driver API entry points so compute/ML/creative apps
//! cleanly detect CUDA status on Apple Silicon and fall back to DirectML / Vulkan /
//! Metal / OpenCL / CPU.
//!
//! - `cuGetProcAddress` resolves every exported CUDA function (plus unversioned
//!   aliases for `*_v2` symbols) to its real export pointer and registers it with
//!   `VA_DISPATCH`, returning `CUDA_SUCCESS` for known symbols and
//!   `CUDA_ERROR_NOT_FOUND` for unknown symbols.
//! - `cuGetErrorName` and `cuGetErrorString` map standard CUDA Driver API error codes
//!   accurately and return `CUDA_ERROR_INVALID_VALUE` for unrecognized codes.
//! - All guest memory writes use unaligned stores (`ptr::write_unaligned` /
//!   `ptr::write_bytes`).
//! - Honestly reports `CUDA_ERROR_NO_DEVICE` and 0 devices on Apple Silicon.

use std::ffi::{c_char, c_void, CStr};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// CUDA Driver API Result Codes (`CUresult`)
// ---------------------------------------------------------------------------

pub const CUDA_SUCCESS: u32 = 0;
pub const CUDA_ERROR_INVALID_VALUE: u32 = 1;
pub const CUDA_ERROR_OUT_OF_MEMORY: u32 = 2;
pub const CUDA_ERROR_NOT_INITIALIZED: u32 = 3;
pub const CUDA_ERROR_DEINITIALIZED: u32 = 4;
pub const CUDA_ERROR_PROFILER_DISABLED: u32 = 5;
pub const CUDA_ERROR_PROFILER_NOT_INITIALIZED: u32 = 6;
pub const CUDA_ERROR_PROFILER_ALREADY_STARTED: u32 = 7;
pub const CUDA_ERROR_PROFILER_ALREADY_STOPPED: u32 = 8;
pub const CUDA_ERROR_STUB_LIBRARY: u32 = 34;
pub const CUDA_ERROR_DEVICE_UNAVAILABLE: u32 = 46;
pub const CUDA_ERROR_NO_DEVICE: u32 = 100;
pub const CUDA_ERROR_INVALID_DEVICE: u32 = 101;
pub const CUDA_ERROR_DEVICE_NOT_LICENSED: u32 = 102;
pub const CUDA_ERROR_INVALID_IMAGE: u32 = 200;
pub const CUDA_ERROR_INVALID_CONTEXT: u32 = 201;
pub const CUDA_ERROR_CONTEXT_ALREADY_CURRENT: u32 = 202;
pub const CUDA_ERROR_MAP_FAILED: u32 = 205;
pub const CUDA_ERROR_UNMAP_FAILED: u32 = 206;
pub const CUDA_ERROR_ARRAY_IS_MAPPED: u32 = 207;
pub const CUDA_ERROR_ALREADY_MAPPED: u32 = 208;
pub const CUDA_ERROR_NO_BINARY_FOR_GPU: u32 = 209;
pub const CUDA_ERROR_ALREADY_ACQUIRED: u32 = 210;
pub const CUDA_ERROR_NOT_MAPPED: u32 = 211;
pub const CUDA_ERROR_NOT_MAPPED_AS_ARRAY: u32 = 212;
pub const CUDA_ERROR_NOT_MAPPED_AS_POINTER: u32 = 213;
pub const CUDA_ERROR_ECC_UNCORRECTABLE: u32 = 214;
pub const CUDA_ERROR_UNSUPPORTED_LIMIT: u32 = 215;
pub const CUDA_ERROR_CONTEXT_ALREADY_IN_USE: u32 = 216;
pub const CUDA_ERROR_PEER_ACCESS_UNSUPPORTED: u32 = 217;
pub const CUDA_ERROR_INVALID_PTX: u32 = 218;
pub const CUDA_ERROR_INVALID_GRAPHICS_CONTEXT: u32 = 219;
pub const CUDA_ERROR_NVLINK_UNCORRECTABLE: u32 = 220;
pub const CUDA_ERROR_JIT_COMPILER_NOT_FOUND: u32 = 221;
pub const CUDA_ERROR_UNSUPPORTED_PTX_VERSION: u32 = 222;
pub const CUDA_ERROR_JIT_COMPILATION_DISABLED: u32 = 223;
pub const CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY: u32 = 224;
pub const CUDA_ERROR_UNSUPPORTED_DEVSIDE_SYNC: u32 = 225;
pub const CUDA_ERROR_INVALID_SOURCE: u32 = 300;
pub const CUDA_ERROR_FILE_NOT_FOUND: u32 = 301;
pub const CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND: u32 = 302;
pub const CUDA_ERROR_SHARED_OBJECT_INIT_FAILED: u32 = 303;
pub const CUDA_ERROR_OPERATING_SYSTEM: u32 = 304;
pub const CUDA_ERROR_INVALID_HANDLE: u32 = 400;
pub const CUDA_ERROR_ILLEGAL_STATE: u32 = 401;
pub const CUDA_ERROR_LOSSY_QUERY: u32 = 402;
pub const CUDA_ERROR_NOT_FOUND: u32 = 500;
pub const CUDA_ERROR_NOT_READY: u32 = 600;
pub const CUDA_ERROR_ILLEGAL_ADDRESS: u32 = 700;
pub const CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES: u32 = 701;
pub const CUDA_ERROR_LAUNCH_TIMEOUT: u32 = 702;
pub const CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING: u32 = 703;
pub const CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED: u32 = 704;
pub const CUDA_ERROR_PEER_ACCESS_NOT_ENABLED: u32 = 705;
pub const CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE: u32 = 708;
pub const CUDA_ERROR_CONTEXT_IS_DESTROYED: u32 = 709;
pub const CUDA_ERROR_ASSERT: u32 = 710;
pub const CUDA_ERROR_TOO_MANY_PEERS: u32 = 711;
pub const CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED: u32 = 712;
pub const CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED: u32 = 713;
pub const CUDA_ERROR_HARDWARE_STACK_ERROR: u32 = 714;
pub const CUDA_ERROR_ILLEGAL_INSTRUCTION: u32 = 715;
pub const CUDA_ERROR_MISALIGNED_ADDRESS: u32 = 716;
pub const CUDA_ERROR_INVALID_ADDRESS_SPACE: u32 = 717;
pub const CUDA_ERROR_INVALID_PC: u32 = 718;
pub const CUDA_ERROR_LAUNCH_FAILED: u32 = 719;
pub const CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE: u32 = 720;
pub const CUDA_ERROR_NOT_PERMITTED: u32 = 800;
pub const CUDA_ERROR_NOT_SUPPORTED: u32 = 801;
pub const CUDA_ERROR_SYSTEM_NOT_READY: u32 = 802;
pub const CUDA_ERROR_SYSTEM_DRIVER_MISMATCH: u32 = 803;
pub const CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE: u32 = 804;
pub const CUDA_ERROR_MPS_CONNECTION_FAILED: u32 = 805;
pub const CUDA_ERROR_MPS_RPC_FAILURE: u32 = 806;
pub const CUDA_ERROR_MPS_SERVER_NOT_READY: u32 = 807;
pub const CUDA_ERROR_MPS_MAX_CLIENTS_REACHED: u32 = 808;
pub const CUDA_ERROR_MPS_MAX_CONNECTIONS_REACHED: u32 = 809;
pub const CUDA_ERROR_MPS_CLIENT_TERMINATED: u32 = 810;
pub const CUDA_ERROR_CDP_NOT_SUPPORTED: u32 = 811;
pub const CUDA_ERROR_CDP_VERSION_MISMATCH: u32 = 812;
pub const CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED: u32 = 900;
pub const CUDA_ERROR_STREAM_CAPTURE_INVALIDATED: u32 = 901;
pub const CUDA_ERROR_STREAM_CAPTURE_MERGE: u32 = 902;
pub const CUDA_ERROR_STREAM_CAPTURE_UNMATCHED: u32 = 903;
pub const CUDA_ERROR_STREAM_CAPTURE_UNJOINED: u32 = 904;
pub const CUDA_ERROR_STREAM_CAPTURE_ISOLATION: u32 = 905;
pub const CUDA_ERROR_STREAM_CAPTURE_IMPLICIT: u32 = 906;
pub const CUDA_ERROR_CAPTURED_EVENT: u32 = 907;
pub const CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD: u32 = 908;
pub const CUDA_ERROR_TIMEOUT: u32 = 909;
pub const CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE: u32 = 910;
pub const CUDA_ERROR_EXTERNAL_DEVICE: u32 = 911;
pub const CUDA_ERROR_INVALID_CLUSTER_SIZE: u32 = 912;
pub const CUDA_ERROR_UNKNOWN: u32 = 999;

#[inline]
unsafe fn write_opt<T>(dst: *mut T, val: T) {
    if !dst.is_null() {
        std::ptr::write_unaligned(dst, val);
    }
}

fn cuda_error_info(error: u32) -> Option<(&'static CStr, &'static CStr)> {
    match error {
        CUDA_SUCCESS => Some((c"CUDA_SUCCESS", c"no error")),
        CUDA_ERROR_INVALID_VALUE => Some((c"CUDA_ERROR_INVALID_VALUE", c"invalid argument")),
        CUDA_ERROR_OUT_OF_MEMORY => Some((c"CUDA_ERROR_OUT_OF_MEMORY", c"out of memory")),
        CUDA_ERROR_NOT_INITIALIZED => {
            Some((c"CUDA_ERROR_NOT_INITIALIZED", c"initialization error"))
        }
        CUDA_ERROR_DEINITIALIZED => Some((c"CUDA_ERROR_DEINITIALIZED", c"driver shutting down")),
        CUDA_ERROR_PROFILER_DISABLED => Some((
            c"CUDA_ERROR_PROFILER_DISABLED",
            c"profiler disabled while using external profiling tool",
        )),
        CUDA_ERROR_PROFILER_NOT_INITIALIZED => Some((
            c"CUDA_ERROR_PROFILER_NOT_INITIALIZED",
            c"profiler not initialized: call cudaProfilerInitialize()",
        )),
        CUDA_ERROR_PROFILER_ALREADY_STARTED => Some((
            c"CUDA_ERROR_PROFILER_ALREADY_STARTED",
            c"profiler already started",
        )),
        CUDA_ERROR_PROFILER_ALREADY_STOPPED => Some((
            c"CUDA_ERROR_PROFILER_ALREADY_STOPPED",
            c"profiler already stopped",
        )),
        CUDA_ERROR_STUB_LIBRARY => Some((
            c"CUDA_ERROR_STUB_LIBRARY",
            c"CUDA driver is a stub library",
        )),
        CUDA_ERROR_DEVICE_UNAVAILABLE => Some((
            c"CUDA_ERROR_DEVICE_UNAVAILABLE",
            c"CUDA-capable device(s) is/are busy or unavailable",
        )),
        CUDA_ERROR_NO_DEVICE => Some((
            c"CUDA_ERROR_NO_DEVICE",
            c"no CUDA-capable device is detected",
        )),
        CUDA_ERROR_INVALID_DEVICE => {
            Some((c"CUDA_ERROR_INVALID_DEVICE", c"invalid device ordinal"))
        }
        CUDA_ERROR_DEVICE_NOT_LICENSED => Some((
            c"CUDA_ERROR_DEVICE_NOT_LICENSED",
            c"device doesn't have valid Grid license",
        )),
        CUDA_ERROR_INVALID_IMAGE => Some((
            c"CUDA_ERROR_INVALID_IMAGE",
            c"device kernel image is invalid",
        )),
        CUDA_ERROR_INVALID_CONTEXT => {
            Some((c"CUDA_ERROR_INVALID_CONTEXT", c"invalid device context"))
        }
        CUDA_ERROR_CONTEXT_ALREADY_CURRENT => Some((
            c"CUDA_ERROR_CONTEXT_ALREADY_CURRENT",
            c"context already current",
        )),
        CUDA_ERROR_MAP_FAILED => Some((
            c"CUDA_ERROR_MAP_FAILED",
            c"mapping of buffer object failed",
        )),
        CUDA_ERROR_UNMAP_FAILED => Some((
            c"CUDA_ERROR_UNMAP_FAILED",
            c"unmapping of buffer object failed",
        )),
        CUDA_ERROR_ARRAY_IS_MAPPED => Some((c"CUDA_ERROR_ARRAY_IS_MAPPED", c"array is mapped")),
        CUDA_ERROR_ALREADY_MAPPED => {
            Some((c"CUDA_ERROR_ALREADY_MAPPED", c"resource already mapped"))
        }
        CUDA_ERROR_NO_BINARY_FOR_GPU => Some((
            c"CUDA_ERROR_NO_BINARY_FOR_GPU",
            c"no kernel image is available for execution on the device",
        )),
        CUDA_ERROR_ALREADY_ACQUIRED => Some((
            c"CUDA_ERROR_ALREADY_ACQUIRED",
            c"resource already acquired",
        )),
        CUDA_ERROR_NOT_MAPPED => Some((c"CUDA_ERROR_NOT_MAPPED", c"resource not mapped")),
        CUDA_ERROR_NOT_MAPPED_AS_ARRAY => Some((
            c"CUDA_ERROR_NOT_MAPPED_AS_ARRAY",
            c"resource not mapped as array",
        )),
        CUDA_ERROR_NOT_MAPPED_AS_POINTER => Some((
            c"CUDA_ERROR_NOT_MAPPED_AS_POINTER",
            c"resource not mapped as pointer",
        )),
        CUDA_ERROR_ECC_UNCORRECTABLE => Some((
            c"CUDA_ERROR_ECC_UNCORRECTABLE",
            c"uncorrectable ECC error encountered",
        )),
        CUDA_ERROR_UNSUPPORTED_LIMIT => Some((
            c"CUDA_ERROR_UNSUPPORTED_LIMIT",
            c"limit is not supported on this architecture",
        )),
        CUDA_ERROR_CONTEXT_ALREADY_IN_USE => Some((
            c"CUDA_ERROR_CONTEXT_ALREADY_IN_USE",
            c"exclusive-thread device already in use by a different thread",
        )),
        CUDA_ERROR_PEER_ACCESS_UNSUPPORTED => Some((
            c"CUDA_ERROR_PEER_ACCESS_UNSUPPORTED",
            c"peer access is not supported between these two devices",
        )),
        CUDA_ERROR_INVALID_PTX => {
            Some((c"CUDA_ERROR_INVALID_PTX", c"a PTX JIT compilation failed"))
        }
        CUDA_ERROR_INVALID_GRAPHICS_CONTEXT => Some((
            c"CUDA_ERROR_INVALID_GRAPHICS_CONTEXT",
            c"invalid OpenGL or DirectX context",
        )),
        CUDA_ERROR_NVLINK_UNCORRECTABLE => Some((
            c"CUDA_ERROR_NVLINK_UNCORRECTABLE",
            c"uncorrectable NVLink error detected during the execution",
        )),
        CUDA_ERROR_JIT_COMPILER_NOT_FOUND => Some((
            c"CUDA_ERROR_JIT_COMPILER_NOT_FOUND",
            c"PTX JIT compiler library not found",
        )),
        CUDA_ERROR_UNSUPPORTED_PTX_VERSION => Some((
            c"CUDA_ERROR_UNSUPPORTED_PTX_VERSION",
            c"the provided PTX was compiled with an unsupported toolchain",
        )),
        CUDA_ERROR_JIT_COMPILATION_DISABLED => Some((
            c"CUDA_ERROR_JIT_COMPILATION_DISABLED",
            c"PTX JIT compilation was disabled",
        )),
        CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY => Some((
            c"CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY",
            c"the provided execution affinity is not supported",
        )),
        CUDA_ERROR_UNSUPPORTED_DEVSIDE_SYNC => Some((
            c"CUDA_ERROR_UNSUPPORTED_DEVSIDE_SYNC",
            c"invalid device-side synchronization",
        )),
        CUDA_ERROR_INVALID_SOURCE => Some((
            c"CUDA_ERROR_INVALID_SOURCE",
            c"device kernel image is invalid",
        )),
        CUDA_ERROR_FILE_NOT_FOUND => Some((c"CUDA_ERROR_FILE_NOT_FOUND", c"file not found")),
        CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND => Some((
            c"CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND",
            c"shared object symbol not found",
        )),
        CUDA_ERROR_SHARED_OBJECT_INIT_FAILED => Some((
            c"CUDA_ERROR_SHARED_OBJECT_INIT_FAILED",
            c"shared object initialization failed",
        )),
        CUDA_ERROR_OPERATING_SYSTEM => Some((
            c"CUDA_ERROR_OPERATING_SYSTEM",
            c"OS call failed or operation not supported on this OS",
        )),
        CUDA_ERROR_INVALID_HANDLE => {
            Some((c"CUDA_ERROR_INVALID_HANDLE", c"invalid resource handle"))
        }
        CUDA_ERROR_ILLEGAL_STATE => Some((
            c"CUDA_ERROR_ILLEGAL_STATE",
            c"the operation cannot be performed in the present state",
        )),
        CUDA_ERROR_LOSSY_QUERY => Some((
            c"CUDA_ERROR_LOSSY_QUERY",
            c"requested query would lose precision",
        )),
        CUDA_ERROR_NOT_FOUND => Some((c"CUDA_ERROR_NOT_FOUND", c"named symbol not found")),
        CUDA_ERROR_NOT_READY => Some((c"CUDA_ERROR_NOT_READY", c"device not ready")),
        CUDA_ERROR_ILLEGAL_ADDRESS => Some((
            c"CUDA_ERROR_ILLEGAL_ADDRESS",
            c"an illegal memory access was encountered",
        )),
        CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES => Some((
            c"CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES",
            c"too many resources requested for launch",
        )),
        CUDA_ERROR_LAUNCH_TIMEOUT => Some((
            c"CUDA_ERROR_LAUNCH_TIMEOUT",
            c"the launch timed out and was terminated",
        )),
        CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING => Some((
            c"CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING",
            c"launch uses incompatible texturing mode",
        )),
        CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED => Some((
            c"CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED",
            c"peer access is already enabled",
        )),
        CUDA_ERROR_PEER_ACCESS_NOT_ENABLED => Some((
            c"CUDA_ERROR_PEER_ACCESS_NOT_ENABLED",
            c"peer access has not been enabled",
        )),
        CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE => Some((
            c"CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE",
            c"cannot set while device is active in this process",
        )),
        CUDA_ERROR_CONTEXT_IS_DESTROYED => {
            Some((c"CUDA_ERROR_CONTEXT_IS_DESTROYED", c"context is destroyed"))
        }
        CUDA_ERROR_ASSERT => Some((c"CUDA_ERROR_ASSERT", c"device-side assert triggered")),
        CUDA_ERROR_TOO_MANY_PEERS => Some((c"CUDA_ERROR_TOO_MANY_PEERS", c"too many peers")),
        CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED => Some((
            c"CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED",
            c"part or all of the requested memory range is already mapped",
        )),
        CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED => Some((
            c"CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED",
            c"pointer does not correspond to a registered memory region",
        )),
        CUDA_ERROR_HARDWARE_STACK_ERROR => {
            Some((c"CUDA_ERROR_HARDWARE_STACK_ERROR", c"hardware stack error"))
        }
        CUDA_ERROR_ILLEGAL_INSTRUCTION => Some((
            c"CUDA_ERROR_ILLEGAL_INSTRUCTION",
            c"an illegal instruction was encountered",
        )),
        CUDA_ERROR_MISALIGNED_ADDRESS => {
            Some((c"CUDA_ERROR_MISALIGNED_ADDRESS", c"misaligned address"))
        }
        CUDA_ERROR_INVALID_ADDRESS_SPACE => Some((
            c"CUDA_ERROR_INVALID_ADDRESS_SPACE",
            c"operation not supported on global/shared address space",
        )),
        CUDA_ERROR_INVALID_PC => Some((c"CUDA_ERROR_INVALID_PC", c"invalid program counter")),
        CUDA_ERROR_LAUNCH_FAILED => {
            Some((c"CUDA_ERROR_LAUNCH_FAILED", c"unspecified launch failure"))
        }
        CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE => Some((
            c"CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE",
            c"too many blocks in cooperative launch",
        )),
        CUDA_ERROR_NOT_PERMITTED => {
            Some((c"CUDA_ERROR_NOT_PERMITTED", c"operation not permitted"))
        }
        CUDA_ERROR_NOT_SUPPORTED => {
            Some((c"CUDA_ERROR_NOT_SUPPORTED", c"operation not supported"))
        }
        CUDA_ERROR_SYSTEM_NOT_READY => {
            Some((c"CUDA_ERROR_SYSTEM_NOT_READY", c"system not yet initialized"))
        }
        CUDA_ERROR_SYSTEM_DRIVER_MISMATCH => Some((
            c"CUDA_ERROR_SYSTEM_DRIVER_MISMATCH",
            c"system has unsupported display driver / cuda driver combination",
        )),
        CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE => Some((
            c"CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE",
            c"forward compatibility was attempted on non supported HW",
        )),
        CUDA_ERROR_MPS_CONNECTION_FAILED => Some((
            c"CUDA_ERROR_MPS_CONNECTION_FAILED",
            c"MPS client failed to connect to the MPS control daemon or the MPS server",
        )),
        CUDA_ERROR_MPS_RPC_FAILURE => Some((
            c"CUDA_ERROR_MPS_RPC_FAILURE",
            c"the remote procedural call between the MPS server and the MPS client failed",
        )),
        CUDA_ERROR_MPS_SERVER_NOT_READY => Some((
            c"CUDA_ERROR_MPS_SERVER_NOT_READY",
            c"MPS server is not ready to accept new CUDA client requests",
        )),
        CUDA_ERROR_MPS_MAX_CLIENTS_REACHED => Some((
            c"CUDA_ERROR_MPS_MAX_CLIENTS_REACHED",
            c"the hardware resources required to create MPS client have been exhausted",
        )),
        CUDA_ERROR_MPS_MAX_CONNECTIONS_REACHED => Some((
            c"CUDA_ERROR_MPS_MAX_CONNECTIONS_REACHED",
            c"the maximum number of MPS client connections has been reached",
        )),
        CUDA_ERROR_MPS_CLIENT_TERMINATED => Some((
            c"CUDA_ERROR_MPS_CLIENT_TERMINATED",
            c"the MPS client has been terminated",
        )),
        CUDA_ERROR_CDP_NOT_SUPPORTED => Some((
            c"CUDA_ERROR_CDP_NOT_SUPPORTED",
            c"CUDA Dynamic Parallelism not supported on this device",
        )),
        CUDA_ERROR_CDP_VERSION_MISMATCH => Some((
            c"CUDA_ERROR_CDP_VERSION_MISMATCH",
            c"CUDA Dynamic Parallelism requires an unsupported version of CUDA",
        )),
        CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED",
            c"operation not permitted when stream is capturing",
        )),
        CUDA_ERROR_STREAM_CAPTURE_INVALIDATED => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_INVALIDATED",
            c"operation failed due to a previous error during capture",
        )),
        CUDA_ERROR_STREAM_CAPTURE_MERGE => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_MERGE",
            c"operation would result in a merge of separate capture sequences",
        )),
        CUDA_ERROR_STREAM_CAPTURE_UNMATCHED => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_UNMATCHED",
            c"capture was not ended in the same stream as it began",
        )),
        CUDA_ERROR_STREAM_CAPTURE_UNJOINED => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_UNJOINED",
            c"capturing stream has unjoined work",
        )),
        CUDA_ERROR_STREAM_CAPTURE_ISOLATION => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_ISOLATION",
            c"dependency created on uncaptured work in another stream",
        )),
        CUDA_ERROR_STREAM_CAPTURE_IMPLICIT => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_IMPLICIT",
            c"operation would make the legacy stream depend on a capturing blocking stream",
        )),
        CUDA_ERROR_CAPTURED_EVENT => Some((
            c"CUDA_ERROR_CAPTURED_EVENT",
            c"operation not permitted on an event last recorded in a capturing stream",
        )),
        CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD => Some((
            c"CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD",
            c"operation not permitted on stream captured by another thread",
        )),
        CUDA_ERROR_TIMEOUT => Some((c"CUDA_ERROR_TIMEOUT", c"wait operation timed out")),
        CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE => Some((
            c"CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE",
            c"the graph update was not performed due to constraint violations",
        )),
        CUDA_ERROR_EXTERNAL_DEVICE => Some((
            c"CUDA_ERROR_EXTERNAL_DEVICE",
            c"an async error has occurred in a device outside of CUDA",
        )),
        CUDA_ERROR_INVALID_CLUSTER_SIZE => Some((
            c"CUDA_ERROR_INVALID_CLUSTER_SIZE",
            c"a kernel launch error has occurred due to cluster specification",
        )),
        CUDA_ERROR_UNKNOWN => Some((c"CUDA_ERROR_UNKNOWN", c"unknown error")),
        _ => None,
    }
}

fn lookup_cuda_proc(name: &str) -> Option<*mut c_void> {
    let ptr: *const () = match name {
        "cuInit" => cuInit as *const (),
        "cuDriverGetVersion" => cuDriverGetVersion as *const (),
        "cuDeviceGetCount" => cuDeviceGetCount as *const (),
        "cuDeviceGet" => cuDeviceGet as *const (),
        "cuGetProcAddress" => cuGetProcAddress as *const (),
        "cuGetErrorName" => cuGetErrorName as *const (),
        "cuGetErrorString" => cuGetErrorString as *const (),
        "cuCtxCreate_v2" | "cuCtxCreate" => cuCtxCreate_v2 as *const (),
        "cuCtxDestroy_v2" | "cuCtxDestroy" => cuCtxDestroy_v2 as *const (),
        "cuCtxGetCurrent" => cuCtxGetCurrent as *const (),
        "cuCtxPopCurrent_v2" | "cuCtxPopCurrent" => cuCtxPopCurrent_v2 as *const (),
        "cuCtxPushCurrent_v2" | "cuCtxPushCurrent" => cuCtxPushCurrent_v2 as *const (),
        "cuDestroyExternalMemory" => cuDestroyExternalMemory as *const (),
        "cuDestroyExternalSemaphore" => cuDestroyExternalSemaphore as *const (),
        "cuDeviceGetAttribute" => cuDeviceGetAttribute as *const (),
        "cuDeviceGetLuid" => cuDeviceGetLuid as *const (),
        "cuExternalMemoryGetMappedMipmappedArray" => {
            cuExternalMemoryGetMappedMipmappedArray as *const ()
        }
        "cuImportExternalMemory" => cuImportExternalMemory as *const (),
        "cuImportExternalSemaphore" => cuImportExternalSemaphore as *const (),
        "cuMemcpy2DAsync_v2" | "cuMemcpy2DAsync" => cuMemcpy2DAsync_v2 as *const (),
        "cuMipmappedArrayDestroy" => cuMipmappedArrayDestroy as *const (),
        "cuMipmappedArrayGetLevel" => cuMipmappedArrayGetLevel as *const (),
        "cuSignalExternalSemaphoresAsync" => cuSignalExternalSemaphoresAsync as *const (),
        "cuStreamCreate" => cuStreamCreate as *const (),
        "cuStreamDestroy_v2" | "cuStreamDestroy" => cuStreamDestroy_v2 as *const (),
        "cuSurfObjectCreate" => cuSurfObjectCreate as *const (),
        "cuSurfObjectDestroy" => cuSurfObjectDestroy as *const (),
        "cuWaitExternalSemaphoresAsync" => cuWaitExternalSemaphoresAsync as *const (),
        _ => return None,
    };
    let addr = ptr as usize;
    crate::register_native_va(addr as u64, addr);
    Some(ptr as *mut c_void)
}

// ---------------------------------------------------------------------------
// Exported CUDA Driver API Win64 Entry Points
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn cuInit(flags: u32) -> u32 {
    if flags != 0 {
        return CUDA_ERROR_INVALID_VALUE;
    }
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuDriverGetVersion(driver_version: *mut i32) -> u32 {
    if driver_version.is_null() {
        return CUDA_ERROR_INVALID_VALUE;
    }
    std::ptr::write_unaligned(driver_version, 12060); // CUDA 12.6
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuDeviceGetCount(count: *mut i32) -> u32 {
    if count.is_null() {
        return CUDA_ERROR_INVALID_VALUE;
    }
    std::ptr::write_unaligned(count, 0);
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuDeviceGet(device: *mut i32, _ordinal: i32) -> u32 {
    write_opt(device, -1);
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuGetProcAddress(
    symbol: *const c_char,
    pfn: *mut *mut c_void,
    _cuda_version: i32,
    _flags: u64,
) -> u32 {
    if pfn.is_null() {
        return CUDA_ERROR_INVALID_VALUE;
    }
    if symbol.is_null() {
        std::ptr::write_unaligned(pfn, std::ptr::null_mut());
        return CUDA_ERROR_INVALID_VALUE;
    }
    let Ok(name) = CStr::from_ptr(symbol).to_str() else {
        std::ptr::write_unaligned(pfn, std::ptr::null_mut());
        return CUDA_ERROR_NOT_FOUND;
    };
    if let Some(addr) = lookup_cuda_proc(name) {
        std::ptr::write_unaligned(pfn, addr);
        CUDA_SUCCESS
    } else {
        std::ptr::write_unaligned(pfn, std::ptr::null_mut());
        CUDA_ERROR_NOT_FOUND
    }
}

pub unsafe extern "win64" fn cuGetErrorName(error: u32, p_str: *mut *const c_char) -> u32 {
    if p_str.is_null() {
        return CUDA_ERROR_INVALID_VALUE;
    }
    match cuda_error_info(error) {
        Some((name, _)) => {
            std::ptr::write_unaligned(p_str, name.as_ptr());
            CUDA_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(p_str, std::ptr::null());
            CUDA_ERROR_INVALID_VALUE
        }
    }
}

pub unsafe extern "win64" fn cuGetErrorString(error: u32, p_str: *mut *const c_char) -> u32 {
    if p_str.is_null() {
        return CUDA_ERROR_INVALID_VALUE;
    }
    match cuda_error_info(error) {
        Some((_, desc)) => {
            std::ptr::write_unaligned(p_str, desc.as_ptr());
            CUDA_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(p_str, std::ptr::null());
            CUDA_ERROR_INVALID_VALUE
        }
    }
}

pub unsafe extern "win64" fn cuCtxCreate_v2(
    pctx: *mut *mut c_void,
    _flags: u32,
    _dev: i32,
) -> u32 {
    write_opt(pctx, std::ptr::null_mut());
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuCtxDestroy_v2(_ctx: *mut c_void) -> u32 {
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuCtxGetCurrent(pctx: *mut *mut c_void) -> u32 {
    write_opt(pctx, std::ptr::null_mut());
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuCtxPopCurrent_v2(pctx: *mut *mut c_void) -> u32 {
    write_opt(pctx, std::ptr::null_mut());
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuCtxPushCurrent_v2(_ctx: *mut c_void) -> u32 {
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuDestroyExternalMemory(_ext_mem: *mut c_void) -> u32 {
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuDestroyExternalSemaphore(_ext_sem: *mut c_void) -> u32 {
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuDeviceGetAttribute(pi: *mut i32, _attrib: i32, _dev: i32) -> u32 {
    write_opt(pi, 0);
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuDeviceGetLuid(
    luid: *mut u8,
    node_mask: *mut u32,
    _dev: i32,
) -> u32 {
    if !luid.is_null() {
        std::ptr::write_bytes(luid, 0, 8);
    }
    write_opt(node_mask, 0);
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuExternalMemoryGetMappedMipmappedArray(
    mipmap: *mut *mut c_void,
    _ext_mem: *mut c_void,
    _desc: *const c_void,
) -> u32 {
    write_opt(mipmap, std::ptr::null_mut());
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuImportExternalMemory(
    ext_mem_out: *mut *mut c_void,
    _mem_handle_desc: *const c_void,
) -> u32 {
    write_opt(ext_mem_out, std::ptr::null_mut());
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuImportExternalSemaphore(
    ext_sem_out: *mut *mut c_void,
    _sem_handle_desc: *const c_void,
) -> u32 {
    write_opt(ext_sem_out, std::ptr::null_mut());
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuMemcpy2DAsync_v2(
    _p_copy: *const c_void,
    _h_stream: *mut c_void,
) -> u32 {
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuMipmappedArrayDestroy(_h_mipmapped_array: *mut c_void) -> u32 {
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuMipmappedArrayGetLevel(
    p_level_array: *mut *mut c_void,
    _h_mipmapped_array: *mut c_void,
    _level: u32,
) -> u32 {
    write_opt(p_level_array, std::ptr::null_mut());
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuSignalExternalSemaphoresAsync(
    _ext_sem_array: *const *mut c_void,
    _params_array: *const c_void,
    num_ext_sems: u32,
    _stream: *mut c_void,
) -> u32 {
    if num_ext_sems == 0 {
        CUDA_SUCCESS
    } else {
        CUDA_ERROR_NO_DEVICE
    }
}

pub unsafe extern "win64" fn cuStreamCreate(ph_stream: *mut *mut c_void, _flags: u32) -> u32 {
    write_opt(ph_stream, std::ptr::null_mut());
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuStreamDestroy_v2(_h_stream: *mut c_void) -> u32 {
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuSurfObjectCreate(
    p_surf_object: *mut u64,
    _p_res_desc: *const c_void,
) -> u32 {
    write_opt(p_surf_object, 0);
    CUDA_ERROR_NO_DEVICE
}

pub unsafe extern "win64" fn cuSurfObjectDestroy(_surf_object: u64) -> u32 {
    CUDA_SUCCESS
}

pub unsafe extern "win64" fn cuWaitExternalSemaphoresAsync(
    _ext_sem_array: *const *mut c_void,
    _params_array: *const c_void,
    num_ext_sems: u32,
    _stream: *mut c_void,
) -> u32 {
    if num_ext_sems == 0 {
        CUDA_SUCCESS
    } else {
        CUDA_ERROR_NO_DEVICE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn cu_get_proc_address_resolves_every_exported_symbol_and_aliases() {
        let dll = crate::builtin_dlls()
            .iter()
            .find(|d| d.name == "nvcuda.dll")
            .expect("nvcuda.dll must be registered");
        assert!(dll.exports.len() >= 28);

        for exp in dll.exports {
            let cname = CString::new(exp.name).unwrap();
            let mut pfn: *mut c_void = std::ptr::null_mut();
            let rc = unsafe { cuGetProcAddress(cname.as_ptr(), &mut pfn, 12060, 0) };
            assert_eq!(
                rc, CUDA_SUCCESS,
                "cuGetProcAddress failed to resolve exported symbol {}",
                exp.name
            );
            assert_eq!(
                pfn as *const (), exp.ptr.0,
                "cuGetProcAddress returned wrong pointer for {}",
                exp.name
            );
        }

        // Unversioned aliases for *_v2 functions resolve to their v2 exports.
        let aliases: [(&str, &str); 6] = [
            ("cuCtxCreate", "cuCtxCreate_v2"),
            ("cuCtxDestroy", "cuCtxDestroy_v2"),
            ("cuCtxPopCurrent", "cuCtxPopCurrent_v2"),
            ("cuCtxPushCurrent", "cuCtxPushCurrent_v2"),
            ("cuMemcpy2DAsync", "cuMemcpy2DAsync_v2"),
            ("cuStreamDestroy", "cuStreamDestroy_v2"),
        ];
        for (alias, v2_name) in aliases {
            let expected = dll
                .exports
                .iter()
                .find(|e| e.name == v2_name)
                .expect("v2 export must exist")
                .ptr
                .0;
            let cname = CString::new(alias).unwrap();
            let mut pfn: *mut c_void = std::ptr::null_mut();
            let rc = unsafe { cuGetProcAddress(cname.as_ptr(), &mut pfn, 12060, 0) };
            assert_eq!(rc, CUDA_SUCCESS, "alias {alias} did not resolve");
            assert_eq!(pfn as *const (), expected, "alias {alias} pointer mismatch");
        }

        // Unknown symbol returns CUDA_ERROR_NOT_FOUND and clears pfn.
        let mut pfn: *mut c_void = 0x1234usize as *mut c_void;
        let rc = unsafe { cuGetProcAddress(c"cuNonExistentSymbol".as_ptr(), &mut pfn, 12060, 0) };
        assert_eq!(rc, CUDA_ERROR_NOT_FOUND);
        assert!(pfn.is_null());

        // Null symbol / null pfn return CUDA_ERROR_INVALID_VALUE.
        let mut pfn2: *mut c_void = 0x5678usize as *mut c_void;
        assert_eq!(
            unsafe { cuGetProcAddress(std::ptr::null(), &mut pfn2, 12060, 0) },
            CUDA_ERROR_INVALID_VALUE
        );
        assert!(pfn2.is_null());
        assert_eq!(
            unsafe { cuGetProcAddress(c"cuInit".as_ptr(), std::ptr::null_mut(), 12060, 0) },
            CUDA_ERROR_INVALID_VALUE
        );
    }

    #[test]
    fn error_name_and_string_cover_standard_cuda_codes() {
        let cases = [
            (CUDA_SUCCESS, "CUDA_SUCCESS", "no error"),
            (CUDA_ERROR_INVALID_VALUE, "CUDA_ERROR_INVALID_VALUE", "invalid argument"),
            (CUDA_ERROR_OUT_OF_MEMORY, "CUDA_ERROR_OUT_OF_MEMORY", "out of memory"),
            (CUDA_ERROR_NOT_INITIALIZED, "CUDA_ERROR_NOT_INITIALIZED", "initialization error"),
            (CUDA_ERROR_DEINITIALIZED, "CUDA_ERROR_DEINITIALIZED", "driver shutting down"),
            (CUDA_ERROR_NO_DEVICE, "CUDA_ERROR_NO_DEVICE", "no CUDA-capable device is detected"),
            (CUDA_ERROR_INVALID_DEVICE, "CUDA_ERROR_INVALID_DEVICE", "invalid device ordinal"),
            (CUDA_ERROR_INVALID_CONTEXT, "CUDA_ERROR_INVALID_CONTEXT", "invalid device context"),
            (CUDA_ERROR_INVALID_HANDLE, "CUDA_ERROR_INVALID_HANDLE", "invalid resource handle"),
            (CUDA_ERROR_NOT_FOUND, "CUDA_ERROR_NOT_FOUND", "named symbol not found"),
            (CUDA_ERROR_NOT_READY, "CUDA_ERROR_NOT_READY", "device not ready"),
            (
                CUDA_ERROR_ILLEGAL_ADDRESS,
                "CUDA_ERROR_ILLEGAL_ADDRESS",
                "an illegal memory access was encountered",
            ),
            (CUDA_ERROR_NOT_SUPPORTED, "CUDA_ERROR_NOT_SUPPORTED", "operation not supported"),
            (CUDA_ERROR_TIMEOUT, "CUDA_ERROR_TIMEOUT", "wait operation timed out"),
            (CUDA_ERROR_UNKNOWN, "CUDA_ERROR_UNKNOWN", "unknown error"),
        ];

        for (code, expected_name, expected_desc) in cases {
            let mut name_ptr: *const c_char = std::ptr::null();
            let mut desc_ptr: *const c_char = std::ptr::null();
            assert_eq!(unsafe { cuGetErrorName(code, &mut name_ptr) }, CUDA_SUCCESS);
            assert_eq!(unsafe { cuGetErrorString(code, &mut desc_ptr) }, CUDA_SUCCESS);
            assert!(!name_ptr.is_null());
            assert!(!desc_ptr.is_null());
            let name = unsafe { CStr::from_ptr(name_ptr) }.to_str().unwrap();
            let desc = unsafe { CStr::from_ptr(desc_ptr) }.to_str().unwrap();
            assert_eq!(name, expected_name);
            assert_eq!(desc, expected_desc);
        }

        // Unknown error code returns CUDA_ERROR_INVALID_VALUE and NULL string.
        let mut p: *const c_char = c"sentinel".as_ptr();
        assert_eq!(unsafe { cuGetErrorName(123456, &mut p) }, CUDA_ERROR_INVALID_VALUE);
        assert!(p.is_null());
        p = c"sentinel".as_ptr();
        assert_eq!(unsafe { cuGetErrorString(123456, &mut p) }, CUDA_ERROR_INVALID_VALUE);
        assert!(p.is_null());

        // Null output pointer returns CUDA_ERROR_INVALID_VALUE.
        assert_eq!(
            unsafe { cuGetErrorName(CUDA_SUCCESS, std::ptr::null_mut()) },
            CUDA_ERROR_INVALID_VALUE
        );
        assert_eq!(
            unsafe { cuGetErrorString(CUDA_SUCCESS, std::ptr::null_mut()) },
            CUDA_ERROR_INVALID_VALUE
        );
    }

    #[test]
    fn honest_no_device_and_unaligned_guest_writes() {
        unsafe {
            assert_eq!(cuInit(0), CUDA_ERROR_NO_DEVICE);
            assert_eq!(cuInit(1), CUDA_ERROR_INVALID_VALUE);
            assert_eq!(cuDriverGetVersion(std::ptr::null_mut()), CUDA_ERROR_INVALID_VALUE);
            assert_eq!(cuDeviceGetCount(std::ptr::null_mut()), CUDA_ERROR_INVALID_VALUE);

            // Exercise unaligned (+1 / +3 byte offset) guest pointer writes across all writers.
            let mut buf = [0xAAu8; 64];

            let ver_ptr = buf.as_mut_ptr().add(1) as *mut i32;
            assert_eq!(cuDriverGetVersion(ver_ptr), CUDA_SUCCESS);
            assert_eq!(std::ptr::read_unaligned(ver_ptr), 12060);

            let cnt_ptr = buf.as_mut_ptr().add(3) as *mut i32;
            assert_eq!(cuDeviceGetCount(cnt_ptr), CUDA_SUCCESS);
            assert_eq!(std::ptr::read_unaligned(cnt_ptr), 0);

            let dev_ptr = buf.as_mut_ptr().add(1) as *mut i32;
            assert_eq!(cuDeviceGet(dev_ptr, 0), CUDA_ERROR_NO_DEVICE);
            assert_eq!(std::ptr::read_unaligned(dev_ptr), -1);

            let pfn_ptr = buf.as_mut_ptr().add(1) as *mut *mut c_void;
            assert_eq!(
                cuGetProcAddress(c"cuDeviceGetCount".as_ptr(), pfn_ptr, 12060, 0),
                CUDA_SUCCESS
            );
            assert_eq!(
                std::ptr::read_unaligned(pfn_ptr) as *const (),
                cuDeviceGetCount as *const ()
            );

            let err_str_ptr = buf.as_mut_ptr().add(3) as *mut *const c_char;
            assert_eq!(cuGetErrorName(CUDA_ERROR_NOT_FOUND, err_str_ptr), CUDA_SUCCESS);
            assert_eq!(
                CStr::from_ptr(std::ptr::read_unaligned(err_str_ptr))
                    .to_str()
                    .unwrap(),
                "CUDA_ERROR_NOT_FOUND"
            );

            let ctx_ptr = buf.as_mut_ptr().add(1) as *mut *mut c_void;
            assert_eq!(cuCtxCreate_v2(ctx_ptr, 0, 0), CUDA_ERROR_NO_DEVICE);
            assert!(std::ptr::read_unaligned(ctx_ptr).is_null());

            std::ptr::write_unaligned(ctx_ptr, 0xDEADusize as *mut c_void);
            assert_eq!(cuCtxGetCurrent(ctx_ptr), CUDA_SUCCESS);
            assert!(std::ptr::read_unaligned(ctx_ptr).is_null());

            std::ptr::write_unaligned(ctx_ptr, 0xBEEFusize as *mut c_void);
            assert_eq!(cuCtxPopCurrent_v2(ctx_ptr), CUDA_SUCCESS);
            assert!(std::ptr::read_unaligned(ctx_ptr).is_null());

            let attr_ptr = buf.as_mut_ptr().add(1) as *mut i32;
            assert_eq!(cuDeviceGetAttribute(attr_ptr, 1, 0), CUDA_ERROR_NO_DEVICE);
            assert_eq!(std::ptr::read_unaligned(attr_ptr), 0);

            let luid_ptr = buf.as_mut_ptr().add(1);
            let mask_ptr = buf.as_mut_ptr().add(11) as *mut u32;
            assert_eq!(cuDeviceGetLuid(luid_ptr, mask_ptr, 0), CUDA_ERROR_NO_DEVICE);
            assert_eq!(&buf[1..9], &[0u8; 8]);
            assert_eq!(std::ptr::read_unaligned(mask_ptr), 0);

            let out_ptr = buf.as_mut_ptr().add(3) as *mut *mut c_void;
            assert_eq!(
                cuExternalMemoryGetMappedMipmappedArray(out_ptr, std::ptr::null_mut(), std::ptr::null()),
                CUDA_ERROR_NO_DEVICE
            );
            assert!(std::ptr::read_unaligned(out_ptr).is_null());

            assert_eq!(
                cuImportExternalMemory(out_ptr, std::ptr::null()),
                CUDA_ERROR_NO_DEVICE
            );
            assert!(std::ptr::read_unaligned(out_ptr).is_null());

            assert_eq!(
                cuImportExternalSemaphore(out_ptr, std::ptr::null()),
                CUDA_ERROR_NO_DEVICE
            );
            assert!(std::ptr::read_unaligned(out_ptr).is_null());

            assert_eq!(
                cuMipmappedArrayGetLevel(out_ptr, std::ptr::null_mut(), 0),
                CUDA_ERROR_NO_DEVICE
            );
            assert!(std::ptr::read_unaligned(out_ptr).is_null());

            assert_eq!(cuStreamCreate(out_ptr, 0), CUDA_ERROR_NO_DEVICE);
            assert!(std::ptr::read_unaligned(out_ptr).is_null());

            let surf_ptr = buf.as_mut_ptr().add(1) as *mut u64;
            assert_eq!(
                cuSurfObjectCreate(surf_ptr, std::ptr::null()),
                CUDA_ERROR_NO_DEVICE
            );
            assert_eq!(std::ptr::read_unaligned(surf_ptr), 0);

            assert_eq!(
                cuSignalExternalSemaphoresAsync(
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                    std::ptr::null_mut(),
                ),
                CUDA_ERROR_NO_DEVICE
            );
            assert_eq!(
                cuWaitExternalSemaphoresAsync(
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                    std::ptr::null_mut(),
                ),
                CUDA_ERROR_NO_DEVICE
            );
        }
    }
}
