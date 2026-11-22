//! `opencl.dll`: OpenCL 1.2 compute runtime bridge.
//!
//! - On macOS (`#[cfg(target_os = "macos")]`), dynamically loads Apple's native
//!   `/System/Library/Frameworks/OpenCL.framework/OpenCL` via `dlopen`/`dlsym` and
//!   forwards OpenCL 1.2 API calls with Win64 -> native C ABI translation, unaligned
//!   guest pointer staging, and guest callback safety.
//! - On iOS (or if `OpenCL.framework` fails to load or exposes no platforms), provides
//!   a clean native CPU OpenCL 1.2 platform (`PortFactory CPU OpenCL 1.2`) supporting
//!   platform/device queries, contexts, command queues, real host-memory buffers
//!   (create/read/write/copy/release), events, and honest `CL_BUILD_PROGRAM_FAILURE`
//!   when kernel compilation is requested.

use std::ffi::{c_char, c_void};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// OpenCL 1.2 Constants
// ---------------------------------------------------------------------------

pub const CL_SUCCESS: i32 = 0;
pub const CL_DEVICE_NOT_FOUND: i32 = -1;
pub const CL_MEM_COPY_OVERLAP: i32 = -8;
pub const CL_BUILD_PROGRAM_FAILURE: i32 = -11;
pub const CL_INVALID_VALUE: i32 = -30;
pub const CL_INVALID_DEVICE_TYPE: i32 = -31;
pub const CL_INVALID_PLATFORM: i32 = -32;
pub const CL_INVALID_DEVICE: i32 = -33;
pub const CL_INVALID_CONTEXT: i32 = -34;
pub const CL_INVALID_QUEUE_PROPERTIES: i32 = -35;
pub const CL_INVALID_COMMAND_QUEUE: i32 = -36;
pub const CL_INVALID_HOST_PTR: i32 = -37;
pub const CL_INVALID_MEM_OBJECT: i32 = -38;
pub const CL_INVALID_PROGRAM: i32 = -44;
pub const CL_INVALID_PROGRAM_EXECUTABLE: i32 = -45;
pub const CL_INVALID_KERNEL_NAME: i32 = -46;
pub const CL_INVALID_KERNEL: i32 = -48;
pub const CL_INVALID_EVENT_WAIT_LIST: i32 = -57;
pub const CL_INVALID_EVENT: i32 = -58;
pub const CL_INVALID_BUFFER_SIZE: i32 = -61;
pub const CL_INVALID_PROPERTY: i32 = -64;

pub const CL_FALSE: u32 = 0;
pub const CL_TRUE: u32 = 1;

// cl_platform_info
pub const CL_PLATFORM_PROFILE: u32 = 0x0900;
pub const CL_PLATFORM_VERSION: u32 = 0x0901;
pub const CL_PLATFORM_NAME: u32 = 0x0902;
pub const CL_PLATFORM_VENDOR: u32 = 0x0903;
pub const CL_PLATFORM_EXTENSIONS: u32 = 0x0904;

// cl_device_type
pub const CL_DEVICE_TYPE_DEFAULT: u64 = 1 << 0;
pub const CL_DEVICE_TYPE_CPU: u64 = 1 << 1;
pub const CL_DEVICE_TYPE_GPU: u64 = 1 << 2;
pub const CL_DEVICE_TYPE_ACCELERATOR: u64 = 1 << 3;
pub const CL_DEVICE_TYPE_CUSTOM: u64 = 1 << 4;
pub const CL_DEVICE_TYPE_ALL: u64 = 0xFFFF_FFFF;

// cl_device_info
pub const CL_DEVICE_TYPE: u32 = 0x1000;
pub const CL_DEVICE_VENDOR_ID: u32 = 0x1001;
pub const CL_DEVICE_MAX_COMPUTE_UNITS: u32 = 0x1002;
pub const CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: u32 = 0x1003;
pub const CL_DEVICE_MAX_WORK_GROUP_SIZE: u32 = 0x1004;
pub const CL_DEVICE_MAX_WORK_ITEM_SIZES: u32 = 0x1005;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: u32 = 0x1006;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: u32 = 0x1007;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: u32 = 0x1008;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: u32 = 0x1009;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT: u32 = 0x100A;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: u32 = 0x100B;
pub const CL_DEVICE_MAX_CLOCK_FREQUENCY: u32 = 0x100C;
pub const CL_DEVICE_ADDRESS_BITS: u32 = 0x100D;
pub const CL_DEVICE_MAX_READ_IMAGE_ARGS: u32 = 0x100E;
pub const CL_DEVICE_MAX_WRITE_IMAGE_ARGS: u32 = 0x100F;
pub const CL_DEVICE_MAX_MEM_ALLOC_SIZE: u32 = 0x1010;
pub const CL_DEVICE_IMAGE2D_MAX_WIDTH: u32 = 0x1011;
pub const CL_DEVICE_IMAGE2D_MAX_HEIGHT: u32 = 0x1012;
pub const CL_DEVICE_IMAGE3D_MAX_WIDTH: u32 = 0x1013;
pub const CL_DEVICE_IMAGE3D_MAX_HEIGHT: u32 = 0x1014;
pub const CL_DEVICE_IMAGE3D_MAX_DEPTH: u32 = 0x1015;
pub const CL_DEVICE_IMAGE_SUPPORT: u32 = 0x1016;
pub const CL_DEVICE_MAX_PARAMETER_SIZE: u32 = 0x1017;
pub const CL_DEVICE_MAX_SAMPLERS: u32 = 0x1018;
pub const CL_DEVICE_MEM_BASE_ADDR_ALIGN: u32 = 0x1019;
pub const CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: u32 = 0x101A;
pub const CL_DEVICE_SINGLE_FP_CONFIG: u32 = 0x101B;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: u32 = 0x101C;
pub const CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: u32 = 0x101D;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: u32 = 0x101E;
pub const CL_DEVICE_GLOBAL_MEM_SIZE: u32 = 0x101F;
pub const CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: u32 = 0x1020;
pub const CL_DEVICE_MAX_CONSTANT_ARGS: u32 = 0x1021;
pub const CL_DEVICE_LOCAL_MEM_TYPE: u32 = 0x1022;
pub const CL_DEVICE_LOCAL_MEM_SIZE: u32 = 0x1023;
pub const CL_DEVICE_ERROR_CORRECTION_SUPPORT: u32 = 0x1024;
pub const CL_DEVICE_PROFILING_TIMER_RESOLUTION: u32 = 0x1025;
pub const CL_DEVICE_ENDIAN_LITTLE: u32 = 0x1026;
pub const CL_DEVICE_AVAILABLE: u32 = 0x1027;
pub const CL_DEVICE_COMPILER_AVAILABLE: u32 = 0x1028;
pub const CL_DEVICE_EXECUTION_CAPABILITIES: u32 = 0x1029;
pub const CL_DEVICE_QUEUE_PROPERTIES: u32 = 0x102A;
pub const CL_DEVICE_NAME: u32 = 0x102B;
pub const CL_DEVICE_VENDOR: u32 = 0x102C;
pub const CL_DRIVER_VERSION: u32 = 0x102D;
pub const CL_DEVICE_PROFILE: u32 = 0x102E;
pub const CL_DEVICE_VERSION: u32 = 0x102F;
pub const CL_DEVICE_EXTENSIONS: u32 = 0x1030;
pub const CL_DEVICE_PLATFORM: u32 = 0x1031;
pub const CL_DEVICE_DOUBLE_FP_CONFIG: u32 = 0x1032;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: u32 = 0x1034;
pub const CL_DEVICE_HOST_UNIFIED_MEMORY: u32 = 0x1035;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: u32 = 0x1036;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: u32 = 0x1037;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: u32 = 0x1038;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: u32 = 0x1039;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: u32 = 0x103A;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: u32 = 0x103B;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: u32 = 0x103C;
pub const CL_DEVICE_OPENCL_C_VERSION: u32 = 0x103D;
pub const CL_DEVICE_LINKER_AVAILABLE: u32 = 0x103E;
pub const CL_DEVICE_BUILT_IN_KERNELS: u32 = 0x103F;
pub const CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: u32 = 0x1040;
pub const CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: u32 = 0x1041;
pub const CL_DEVICE_PARENT_DEVICE: u32 = 0x1042;
pub const CL_DEVICE_PARTITION_MAX_SUB_DEVICES: u32 = 0x1043;
pub const CL_DEVICE_REFERENCE_COUNT: u32 = 0x1047;
pub const CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: u32 = 0x1048;
pub const CL_DEVICE_PRINTF_BUFFER_SIZE: u32 = 0x1049;

// cl_device_fp_config
const CL_FP_DENORM: u64 = 1 << 0;
const CL_FP_INF_NAN: u64 = 1 << 1;
const CL_FP_ROUND_TO_NEAREST: u64 = 1 << 2;
const CL_FP_ROUND_TO_ZERO: u64 = 1 << 3;
const CL_FP_ROUND_TO_INF: u64 = 1 << 4;
const CL_FP_FMA: u64 = 1 << 5;

const CL_READ_WRITE_CACHE: u32 = 0x2;
const CL_GLOBAL: u32 = 0x2;
const CL_EXEC_KERNEL: u64 = 1 << 0;
const CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE: u64 = 1 << 0;
const CL_QUEUE_PROFILING_ENABLE: u64 = 1 << 1;

// cl_context_properties
pub const CL_CONTEXT_PLATFORM: isize = 0x1084;
pub const CL_CONTEXT_INTEROP_USER_SYNC: isize = 0x1085;

// cl_mem_flags
pub const CL_MEM_READ_WRITE: u64 = 1 << 0;
pub const CL_MEM_WRITE_ONLY: u64 = 1 << 1;
pub const CL_MEM_READ_ONLY: u64 = 1 << 2;
pub const CL_MEM_USE_HOST_PTR: u64 = 1 << 3;
pub const CL_MEM_ALLOC_HOST_PTR: u64 = 1 << 4;
pub const CL_MEM_COPY_HOST_PTR: u64 = 1 << 5;
pub const CL_MEM_HOST_WRITE_ONLY: u64 = 1 << 7;
pub const CL_MEM_HOST_READ_ONLY: u64 = 1 << 8;
pub const CL_MEM_HOST_NO_ACCESS: u64 = 1 << 9;

// cl_program_build_info
pub const CL_PROGRAM_BUILD_STATUS: u32 = 0x1181;
pub const CL_PROGRAM_BUILD_OPTIONS: u32 = 0x1182;
pub const CL_PROGRAM_BUILD_LOG: u32 = 0x1183;
pub const CL_PROGRAM_BINARY_TYPE: u32 = 0x1184;

pub const CL_PROGRAM_BINARY_TYPE_NONE: u32 = 0x0;

// cl_build_status
pub const CL_BUILD_SUCCESS: i32 = 0;
pub const CL_BUILD_NONE: i32 = -1;
pub const CL_BUILD_ERROR: i32 = -2;

// cl_command_type
const CL_COMMAND_READ_BUFFER: u32 = 0x11F3;
const CL_COMMAND_WRITE_BUFFER: u32 = 0x11F4;
const CL_COMMAND_COPY_BUFFER: u32 = 0x11F5;

// command execution status
const CL_COMPLETE: i32 = 0x0;

// ---------------------------------------------------------------------------
// Unaligned guest pointer helpers
// ---------------------------------------------------------------------------

#[inline]
unsafe fn write_opt<T>(dst: *mut T, val: T) {
    if !dst.is_null() {
        std::ptr::write_unaligned(dst, val);
    }
}

/// Reads a zero-terminated `cl_context_properties` key/value list from guest memory
/// using unaligned loads.
unsafe fn read_context_properties(props: *const isize) -> Option<Vec<isize>> {
    if props.is_null() {
        return None;
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < 128 {
        let key = std::ptr::read_unaligned(props.add(i));
        out.push(key);
        if key == 0 {
            break;
        }
        let val = std::ptr::read_unaligned(props.add(i + 1));
        out.push(val);
        i += 2;
    }
    if out.last().copied() != Some(0) {
        out.push(0);
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// macOS Native OpenCL.framework Backend (`dlopen` / `dlsym`)
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod native {
    use super::*;
    use std::ffi::CStr;
    use std::sync::OnceLock;

    #[inline]
    fn is_aligned<T>(ptr: *const T) -> bool {
        (ptr as usize) % std::mem::align_of::<T>() == 0
    }

    const OPENCL_FRAMEWORK_PATH: &CStr =
        c"/System/Library/Frameworks/OpenCL.framework/OpenCL";

    type PfnClGetPlatformIDs =
        unsafe extern "C" fn(u32, *mut *mut c_void, *mut u32) -> i32;
    type PfnClGetPlatformInfo =
        unsafe extern "C" fn(*mut c_void, u32, usize, *mut c_void, *mut usize) -> i32;
    type PfnClGetDeviceIDs =
        unsafe extern "C" fn(*mut c_void, u64, u32, *mut *mut c_void, *mut u32) -> i32;
    type PfnClGetDeviceInfo =
        unsafe extern "C" fn(*mut c_void, u32, usize, *mut c_void, *mut usize) -> i32;
    type PfnClCreateContext = unsafe extern "C" fn(
        *const isize,
        u32,
        *const *mut c_void,
        *const c_void,
        *mut c_void,
        *mut i32,
    ) -> *mut c_void;
    type PfnClCreateContextFromType = unsafe extern "C" fn(
        *const isize,
        u64,
        *const c_void,
        *mut c_void,
        *mut i32,
    ) -> *mut c_void;
    type PfnClRetainContext = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClReleaseContext = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClCreateCommandQueue =
        unsafe extern "C" fn(*mut c_void, *mut c_void, u64, *mut i32) -> *mut c_void;
    type PfnClReleaseCommandQueue = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClCreateBuffer =
        unsafe extern "C" fn(*mut c_void, u64, usize, *mut c_void, *mut i32) -> *mut c_void;
    type PfnClReleaseMemObject = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClEnqueueReadBuffer = unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        u32,
        usize,
        usize,
        *mut c_void,
        u32,
        *const *mut c_void,
        *mut *mut c_void,
    ) -> i32;
    type PfnClEnqueueWriteBuffer = unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        u32,
        usize,
        usize,
        *const c_void,
        u32,
        *const *mut c_void,
        *mut *mut c_void,
    ) -> i32;
    type PfnClEnqueueCopyBuffer = unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *mut c_void,
        usize,
        usize,
        usize,
        u32,
        *const *mut c_void,
        *mut *mut c_void,
    ) -> i32;
    type PfnClCreateProgramWithSource = unsafe extern "C" fn(
        *mut c_void,
        u32,
        *const *const c_char,
        *const usize,
        *mut i32,
    ) -> *mut c_void;
    type PfnClBuildProgram = unsafe extern "C" fn(
        *mut c_void,
        u32,
        *const *mut c_void,
        *const c_char,
        *const c_void,
        *mut c_void,
    ) -> i32;
    type PfnClGetProgramBuildInfo = unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        u32,
        usize,
        *mut c_void,
        *mut usize,
    ) -> i32;
    type PfnClReleaseProgram = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClCreateKernel =
        unsafe extern "C" fn(*mut c_void, *const c_char, *mut i32) -> *mut c_void;
    type PfnClSetKernelArg =
        unsafe extern "C" fn(*mut c_void, u32, usize, *const c_void) -> i32;
    type PfnClEnqueueNDRangeKernel = unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        u32,
        *const usize,
        *const usize,
        *const usize,
        u32,
        *const *mut c_void,
        *mut *mut c_void,
    ) -> i32;
    type PfnClReleaseKernel = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClFlush = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClFinish = unsafe extern "C" fn(*mut c_void) -> i32;
    type PfnClWaitForEvents = unsafe extern "C" fn(u32, *const *mut c_void) -> i32;
    type PfnClReleaseEvent = unsafe extern "C" fn(*mut c_void) -> i32;

    #[repr(align(16))]
    struct AlignedArgBuf([u8; 256]);

    pub struct NativeOpenCl {
        _handle: usize,
        pub get_platform_ids: PfnClGetPlatformIDs,
        pub get_platform_info: PfnClGetPlatformInfo,
        pub get_device_ids: PfnClGetDeviceIDs,
        pub get_device_info: PfnClGetDeviceInfo,
        pub create_context: PfnClCreateContext,
        pub create_context_from_type: PfnClCreateContextFromType,
        pub retain_context: PfnClRetainContext,
        pub release_context: PfnClReleaseContext,
        pub create_command_queue: PfnClCreateCommandQueue,
        pub release_command_queue: PfnClReleaseCommandQueue,
        pub create_buffer: PfnClCreateBuffer,
        pub release_mem_object: PfnClReleaseMemObject,
        pub enqueue_read_buffer: PfnClEnqueueReadBuffer,
        pub enqueue_write_buffer: PfnClEnqueueWriteBuffer,
        pub enqueue_copy_buffer: PfnClEnqueueCopyBuffer,
        pub create_program_with_source: PfnClCreateProgramWithSource,
        pub build_program: PfnClBuildProgram,
        pub get_program_build_info: PfnClGetProgramBuildInfo,
        pub release_program: PfnClReleaseProgram,
        pub create_kernel: PfnClCreateKernel,
        pub set_kernel_arg: PfnClSetKernelArg,
        pub enqueue_nd_range_kernel: PfnClEnqueueNDRangeKernel,
        pub release_kernel: PfnClReleaseKernel,
        pub flush: PfnClFlush,
        pub finish: PfnClFinish,
        pub wait_for_events: PfnClWaitForEvents,
        pub release_event: PfnClReleaseEvent,
    }

    unsafe impl Send for NativeOpenCl {}
    unsafe impl Sync for NativeOpenCl {}

    unsafe fn sym<T: Copy>(handle: *mut c_void, name: &CStr) -> Option<T> {
        let p = libc::dlsym(handle, name.as_ptr());
        if p.is_null() {
            return None;
        }
        Some(std::mem::transmute_copy::<*mut c_void, T>(&p))
    }

    impl NativeOpenCl {
        unsafe fn load() -> Option<Self> {
            let handle = libc::dlopen(
                OPENCL_FRAMEWORK_PATH.as_ptr(),
                libc::RTLD_NOW | libc::RTLD_LOCAL,
            );
            if handle.is_null() {
                log::info!(
                    target: "opencl",
                    "OpenCL.framework unavailable; falling back to PortFactory CPU OpenCL 1.2"
                );
                return None;
            }

            let loaded = (|| -> Option<Self> {
                Some(Self {
                    _handle: handle as usize,
                    get_platform_ids: sym(handle, c"clGetPlatformIDs")?,
                    get_platform_info: sym(handle, c"clGetPlatformInfo")?,
                    get_device_ids: sym(handle, c"clGetDeviceIDs")?,
                    get_device_info: sym(handle, c"clGetDeviceInfo")?,
                    create_context: sym(handle, c"clCreateContext")?,
                    create_context_from_type: sym(handle, c"clCreateContextFromType")?,
                    retain_context: sym(handle, c"clRetainContext")?,
                    release_context: sym(handle, c"clReleaseContext")?,
                    create_command_queue: sym(handle, c"clCreateCommandQueue")?,
                    release_command_queue: sym(handle, c"clReleaseCommandQueue")?,
                    create_buffer: sym(handle, c"clCreateBuffer")?,
                    release_mem_object: sym(handle, c"clReleaseMemObject")?,
                    enqueue_read_buffer: sym(handle, c"clEnqueueReadBuffer")?,
                    enqueue_write_buffer: sym(handle, c"clEnqueueWriteBuffer")?,
                    enqueue_copy_buffer: sym(handle, c"clEnqueueCopyBuffer")?,
                    create_program_with_source: sym(handle, c"clCreateProgramWithSource")?,
                    build_program: sym(handle, c"clBuildProgram")?,
                    get_program_build_info: sym(handle, c"clGetProgramBuildInfo")?,
                    release_program: sym(handle, c"clReleaseProgram")?,
                    create_kernel: sym(handle, c"clCreateKernel")?,
                    set_kernel_arg: sym(handle, c"clSetKernelArg")?,
                    enqueue_nd_range_kernel: sym(handle, c"clEnqueueNDRangeKernel")?,
                    release_kernel: sym(handle, c"clReleaseKernel")?,
                    flush: sym(handle, c"clFlush")?,
                    finish: sym(handle, c"clFinish")?,
                    wait_for_events: sym(handle, c"clWaitForEvents")?,
                    release_event: sym(handle, c"clReleaseEvent")?,
                })
            })();

            let Some(backend) = loaded else {
                log::warn!(
                    target: "opencl",
                    "OpenCL.framework missing required OpenCL 1.2 symbols; falling back to CPU OpenCL 1.2"
                );
                libc::dlclose(handle);
                return None;
            };

            let mut num_platforms: u32 = 0;
            let rc = (backend.get_platform_ids)(0, std::ptr::null_mut(), &mut num_platforms);
            if rc != CL_SUCCESS || num_platforms == 0 {
                log::info!(
                    target: "opencl",
                    "OpenCL.framework reported rc={rc}, platforms={num_platforms}; falling back to CPU OpenCL 1.2"
                );
                libc::dlclose(handle);
                return None;
            }

            log::info!(
                target: "opencl",
                "Loaded macOS OpenCL.framework ({num_platforms} platform(s))"
            );
            Some(backend)
        }
    }

    pub fn backend() -> Option<&'static NativeOpenCl> {
        static BACKEND: OnceLock<Option<NativeOpenCl>> = OnceLock::new();
        BACKEND.get_or_init(|| unsafe { NativeOpenCl::load() }).as_ref()
    }

    /// Stages a possibly-unaligned input array of `T` into an aligned `Vec<T>` when needed.
    unsafe fn stage_in_array<T: Copy>(ptr: *const T, count: usize) -> Option<Vec<T>> {
        if ptr.is_null() || count == 0 || is_aligned(ptr) {
            return None;
        }
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            v.push(std::ptr::read_unaligned(ptr.add(i)));
        }
        Some(v)
    }

    /// Calls an OpenCL info query (`clGetPlatformInfo` / `clGetDeviceInfo` /
    /// `clGetProgramBuildInfo`) with safe staging for unaligned guest `param_value`
    /// and `param_value_size_ret`.
    unsafe fn forward_info_query<F>(
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
        call: F,
    ) -> i32
    where
        F: FnOnce(usize, *mut c_void, *mut usize) -> i32,
    {
        let mut size_ret: usize = 0;
        let need_staging =
            !param_value.is_null() && param_value_size > 0 && (param_value as usize) % 16 != 0;

        if need_staging {
            let words = param_value_size.div_ceil(std::mem::size_of::<u128>());
            let mut staged = vec![0u128; words];
            let rc = call(
                param_value_size,
                staged.as_mut_ptr() as *mut c_void,
                &mut size_ret,
            );
            if rc == CL_SUCCESS {
                let copy_len = if size_ret > 0 {
                    size_ret.min(param_value_size)
                } else {
                    param_value_size
                };
                std::ptr::copy_nonoverlapping(
                    staged.as_ptr() as *const u8,
                    param_value as *mut u8,
                    copy_len,
                );
                write_opt(param_value_size_ret, size_ret);
            }
            rc
        } else {
            let ret_ptr = if param_value_size_ret.is_null() {
                std::ptr::null_mut()
            } else {
                &mut size_ret
            };
            let rc = call(param_value_size, param_value, ret_ptr);
            if rc == CL_SUCCESS {
                write_opt(param_value_size_ret, size_ret);
            }
            rc
        }
    }

    /// Sanitizes `pfn_notify` / `user_data` for `clCreateContext` / `clCreateContextFromType`.
    /// Under a Windows guest, passing raw x86-64 callback pointers into Apple's OpenCL
    /// runtime would crash if invoked from an internal GCD thread.
    #[inline]
    fn sanitize_context_notify(
        pfn_notify: usize,
        user_data: *mut c_void,
    ) -> (*const c_void, *mut c_void) {
        if pfn_notify == 0 {
            (std::ptr::null(), user_data)
        } else if crate::try_loader().is_some() {
            (std::ptr::null(), std::ptr::null_mut())
        } else {
            (pfn_notify as *const c_void, user_data)
        }
    }

    pub unsafe fn get_platform_ids(
        cl: &NativeOpenCl,
        num_entries: u32,
        platforms: *mut *mut c_void,
        num_platforms: *mut u32,
    ) -> i32 {
        let mut count_ret: u32 = 0;
        let count_ptr = if num_platforms.is_null() {
            std::ptr::null_mut()
        } else {
            &mut count_ret
        };

        let need_staging = !platforms.is_null() && num_entries > 0 && !is_aligned(platforms);
        let rc = if need_staging {
            let cap = (num_entries as usize).min(1024);
            let mut staged = vec![std::ptr::null_mut(); cap];
            let mut actual: u32 = 0;
            let r = (cl.get_platform_ids)(cap as u32, staged.as_mut_ptr(), &mut actual);
            if r == CL_SUCCESS {
                let to_copy = (actual as usize).min(cap);
                for (i, &h) in staged.iter().take(to_copy).enumerate() {
                    std::ptr::write_unaligned(platforms.add(i), h);
                }
                count_ret = actual;
            }
            r
        } else {
            (cl.get_platform_ids)(num_entries, platforms, count_ptr)
        };

        if rc == CL_SUCCESS {
            write_opt(num_platforms, count_ret);
        }
        rc
    }

    pub unsafe fn get_platform_info(
        cl: &NativeOpenCl,
        platform: *mut c_void,
        param_name: u32,
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
    ) -> i32 {
        forward_info_query(
            param_value_size,
            param_value,
            param_value_size_ret,
            |sz, ptr, ret| (cl.get_platform_info)(platform, param_name, sz, ptr, ret),
        )
    }

    pub unsafe fn get_device_ids(
        cl: &NativeOpenCl,
        platform: *mut c_void,
        device_type: u64,
        num_entries: u32,
        devices: *mut *mut c_void,
        num_devices: *mut u32,
    ) -> i32 {
        let mut count_ret: u32 = 0;
        let count_ptr = if num_devices.is_null() {
            std::ptr::null_mut()
        } else {
            &mut count_ret
        };

        let need_staging = !devices.is_null() && num_entries > 0 && !is_aligned(devices);
        let rc = if need_staging {
            let cap = (num_entries as usize).min(1024);
            let mut staged = vec![std::ptr::null_mut(); cap];
            let mut actual: u32 = 0;
            let r = (cl.get_device_ids)(
                platform,
                device_type,
                cap as u32,
                staged.as_mut_ptr(),
                &mut actual,
            );
            if r == CL_SUCCESS {
                let to_copy = (actual as usize).min(cap);
                for (i, &h) in staged.iter().take(to_copy).enumerate() {
                    std::ptr::write_unaligned(devices.add(i), h);
                }
                count_ret = actual;
            }
            r
        } else {
            (cl.get_device_ids)(platform, device_type, num_entries, devices, count_ptr)
        };

        if rc == CL_SUCCESS {
            write_opt(num_devices, count_ret);
        }
        rc
    }

    pub unsafe fn get_device_info(
        cl: &NativeOpenCl,
        device: *mut c_void,
        param_name: u32,
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
    ) -> i32 {
        forward_info_query(
            param_value_size,
            param_value,
            param_value_size_ret,
            |sz, ptr, ret| (cl.get_device_info)(device, param_name, sz, ptr, ret),
        )
    }

    pub unsafe fn create_context(
        cl: &NativeOpenCl,
        properties: *const isize,
        num_devices: u32,
        devices: *const *mut c_void,
        pfn_notify: usize,
        user_data: *mut c_void,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        let props_buf = read_context_properties(properties);
        let props_ptr = props_buf
            .as_ref()
            .map_or(std::ptr::null(), |v| v.as_ptr());
        let devs_buf = if num_devices <= 1024 {
            stage_in_array(devices, num_devices as usize)
        } else {
            None
        };
        let devs_ptr = devs_buf.as_ref().map_or(devices, |v| v.as_ptr());
        let (notify_ptr, notify_ud) = sanitize_context_notify(pfn_notify, user_data);
        let mut err: i32 = CL_SUCCESS;
        let ctx = (cl.create_context)(
            props_ptr,
            num_devices,
            devs_ptr,
            notify_ptr,
            notify_ud,
            &mut err,
        );
        write_opt(errcode_ret, err);
        ctx
    }

    pub unsafe fn create_context_from_type(
        cl: &NativeOpenCl,
        properties: *const isize,
        device_type: u64,
        pfn_notify: usize,
        user_data: *mut c_void,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        let props_buf = read_context_properties(properties);
        let props_ptr = props_buf
            .as_ref()
            .map_or(std::ptr::null(), |v| v.as_ptr());
        let (notify_ptr, notify_ud) = sanitize_context_notify(pfn_notify, user_data);
        let mut err: i32 = CL_SUCCESS;
        let ctx = (cl.create_context_from_type)(
            props_ptr,
            device_type,
            notify_ptr,
            notify_ud,
            &mut err,
        );
        write_opt(errcode_ret, err);
        ctx
    }

    pub unsafe fn create_command_queue(
        cl: &NativeOpenCl,
        context: *mut c_void,
        device: *mut c_void,
        properties: u64,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        let mut err: i32 = CL_SUCCESS;
        let q = (cl.create_command_queue)(context, device, properties, &mut err);
        write_opt(errcode_ret, err);
        q
    }

    pub unsafe fn create_buffer(
        cl: &NativeOpenCl,
        context: *mut c_void,
        flags: u64,
        size: usize,
        host_ptr: *mut c_void,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        let mut err: i32 = CL_SUCCESS;
        let mem = (cl.create_buffer)(context, flags, size, host_ptr, &mut err);
        write_opt(errcode_ret, err);
        mem
    }

    pub unsafe fn enqueue_read_buffer(
        cl: &NativeOpenCl,
        command_queue: *mut c_void,
        buffer: *mut c_void,
        blocking_read: u32,
        offset: usize,
        size: usize,
        ptr: *mut c_void,
        num_events_in_wait_list: u32,
        event_wait_list: *const *mut c_void,
        event: *mut *mut c_void,
    ) -> i32 {
        let wl_buf = if num_events_in_wait_list <= 4096 {
            stage_in_array(event_wait_list, num_events_in_wait_list as usize)
        } else {
            None
        };
        let wl_ptr = wl_buf.as_ref().map_or(event_wait_list, |v| v.as_ptr());
        let mut ev_out: *mut c_void = std::ptr::null_mut();
        let ev_ptr = if event.is_null() {
            std::ptr::null_mut()
        } else {
            &mut ev_out
        };
        let rc = (cl.enqueue_read_buffer)(
            command_queue,
            buffer,
            blocking_read,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            wl_ptr,
            ev_ptr,
        );
        if rc == CL_SUCCESS {
            write_opt(event, ev_out);
        }
        rc
    }

    pub unsafe fn enqueue_write_buffer(
        cl: &NativeOpenCl,
        command_queue: *mut c_void,
        buffer: *mut c_void,
        blocking_write: u32,
        offset: usize,
        size: usize,
        ptr: *const c_void,
        num_events_in_wait_list: u32,
        event_wait_list: *const *mut c_void,
        event: *mut *mut c_void,
    ) -> i32 {
        let wl_buf = if num_events_in_wait_list <= 4096 {
            stage_in_array(event_wait_list, num_events_in_wait_list as usize)
        } else {
            None
        };
        let wl_ptr = wl_buf.as_ref().map_or(event_wait_list, |v| v.as_ptr());
        let mut ev_out: *mut c_void = std::ptr::null_mut();
        let ev_ptr = if event.is_null() {
            std::ptr::null_mut()
        } else {
            &mut ev_out
        };
        let rc = (cl.enqueue_write_buffer)(
            command_queue,
            buffer,
            blocking_write,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            wl_ptr,
            ev_ptr,
        );
        if rc == CL_SUCCESS {
            write_opt(event, ev_out);
        }
        rc
    }

    pub unsafe fn enqueue_copy_buffer(
        cl: &NativeOpenCl,
        command_queue: *mut c_void,
        src_buffer: *mut c_void,
        dst_buffer: *mut c_void,
        src_offset: usize,
        dst_offset: usize,
        size: usize,
        num_events_in_wait_list: u32,
        event_wait_list: *const *mut c_void,
        event: *mut *mut c_void,
    ) -> i32 {
        let wl_buf = if num_events_in_wait_list <= 4096 {
            stage_in_array(event_wait_list, num_events_in_wait_list as usize)
        } else {
            None
        };
        let wl_ptr = wl_buf.as_ref().map_or(event_wait_list, |v| v.as_ptr());
        let mut ev_out: *mut c_void = std::ptr::null_mut();
        let ev_ptr = if event.is_null() {
            std::ptr::null_mut()
        } else {
            &mut ev_out
        };
        let rc = (cl.enqueue_copy_buffer)(
            command_queue,
            src_buffer,
            dst_buffer,
            src_offset,
            dst_offset,
            size,
            num_events_in_wait_list,
            wl_ptr,
            ev_ptr,
        );
        if rc == CL_SUCCESS {
            write_opt(event, ev_out);
        }
        rc
    }

    pub unsafe fn create_program_with_source(
        cl: &NativeOpenCl,
        context: *mut c_void,
        count: u32,
        strings: *const *const c_char,
        lengths: *const usize,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        let str_buf = if count <= 65536 {
            stage_in_array(strings, count as usize)
        } else {
            None
        };
        let str_ptr = str_buf.as_ref().map_or(strings, |v| v.as_ptr());
        let len_buf = if count <= 65536 {
            stage_in_array(lengths, count as usize)
        } else {
            None
        };
        let len_ptr = len_buf.as_ref().map_or(lengths, |v| v.as_ptr());
        let mut err: i32 = CL_SUCCESS;
        let prog = (cl.create_program_with_source)(context, count, str_ptr, len_ptr, &mut err);
        write_opt(errcode_ret, err);
        prog
    }

    pub unsafe fn build_program(
        cl: &NativeOpenCl,
        program: *mut c_void,
        num_devices: u32,
        device_list: *const *mut c_void,
        options: *const c_char,
        pfn_notify: usize,
        user_data: *mut c_void,
    ) -> i32 {
        let dev_buf = if num_devices <= 1024 {
            stage_in_array(device_list, num_devices as usize)
        } else {
            None
        };
        let dev_ptr = dev_buf.as_ref().map_or(device_list, |v| v.as_ptr());

        // When `pfn_notify` is supplied, run the build synchronously on the calling
        // thread and dispatch the callback through `call_guest8` once complete so
        // Win64 guest callbacks execute on a thread with a valid TEB/guest stack.
        let (native_notify, native_ud) = if pfn_notify == 0 {
            (std::ptr::null(), user_data)
        } else {
            (std::ptr::null(), std::ptr::null_mut())
        };
        let rc = (cl.build_program)(
            program,
            num_devices,
            dev_ptr,
            options,
            native_notify,
            native_ud,
        );
        if pfn_notify != 0 && (rc == CL_SUCCESS || rc == CL_BUILD_PROGRAM_FAILURE) {
            let _ = crate::call_guest8(
                pfn_notify,
                program as u64,
                user_data as u64,
                0,
                0,
                0,
                0,
                0,
                0,
            );
        }
        rc
    }

    pub unsafe fn get_program_build_info(
        cl: &NativeOpenCl,
        program: *mut c_void,
        device: *mut c_void,
        param_name: u32,
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
    ) -> i32 {
        forward_info_query(
            param_value_size,
            param_value,
            param_value_size_ret,
            |sz, ptr, ret| (cl.get_program_build_info)(program, device, param_name, sz, ptr, ret),
        )
    }

    pub unsafe fn create_kernel(
        cl: &NativeOpenCl,
        program: *mut c_void,
        kernel_name: *const c_char,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        let mut err: i32 = CL_SUCCESS;
        let k = (cl.create_kernel)(program, kernel_name, &mut err);
        write_opt(errcode_ret, err);
        k
    }

    pub unsafe fn set_kernel_arg(
        cl: &NativeOpenCl,
        kernel: *mut c_void,
        arg_index: u32,
        arg_size: usize,
        arg_value: *const c_void,
    ) -> i32 {
        if arg_value.is_null() || arg_size == 0 || (arg_value as usize) % 16 == 0 {
            return (cl.set_kernel_arg)(kernel, arg_index, arg_size, arg_value);
        }
        if arg_size <= 256 {
            let mut buf = AlignedArgBuf([0u8; 256]);
            std::ptr::copy_nonoverlapping(
                arg_value as *const u8,
                buf.0.as_mut_ptr(),
                arg_size,
            );
            (cl.set_kernel_arg)(kernel, arg_index, arg_size, buf.0.as_ptr() as *const c_void)
        } else {
            let words = arg_size.div_ceil(std::mem::size_of::<u128>());
            let mut buf = vec![0u128; words];
            std::ptr::copy_nonoverlapping(
                arg_value as *const u8,
                buf.as_mut_ptr() as *mut u8,
                arg_size,
            );
            (cl.set_kernel_arg)(kernel, arg_index, arg_size, buf.as_ptr() as *const c_void)
        }
    }

    pub unsafe fn enqueue_nd_range_kernel(
        cl: &NativeOpenCl,
        command_queue: *mut c_void,
        kernel: *mut c_void,
        work_dim: u32,
        global_work_offset: *const usize,
        global_work_size: *const usize,
        local_work_size: *const usize,
        num_events_in_wait_list: u32,
        event_wait_list: *const *mut c_void,
        event: *mut *mut c_void,
    ) -> i32 {
        let dims = if work_dim <= 64 { work_dim as usize } else { 0 };
        let gwo_buf = stage_in_array(global_work_offset, dims);
        let gws_buf = stage_in_array(global_work_size, dims);
        let lws_buf = stage_in_array(local_work_size, dims);
        let wl_buf = if num_events_in_wait_list <= 4096 {
            stage_in_array(event_wait_list, num_events_in_wait_list as usize)
        } else {
            None
        };
        let gwo_ptr = gwo_buf.as_ref().map_or(global_work_offset, |v| v.as_ptr());
        let gws_ptr = gws_buf.as_ref().map_or(global_work_size, |v| v.as_ptr());
        let lws_ptr = lws_buf.as_ref().map_or(local_work_size, |v| v.as_ptr());
        let wl_ptr = wl_buf.as_ref().map_or(event_wait_list, |v| v.as_ptr());

        let mut ev_out: *mut c_void = std::ptr::null_mut();
        let ev_ptr = if event.is_null() {
            std::ptr::null_mut()
        } else {
            &mut ev_out
        };
        let rc = (cl.enqueue_nd_range_kernel)(
            command_queue,
            kernel,
            work_dim,
            gwo_ptr,
            gws_ptr,
            lws_ptr,
            num_events_in_wait_list,
            wl_ptr,
            ev_ptr,
        );
        if rc == CL_SUCCESS {
            write_opt(event, ev_out);
        }
        rc
    }

    pub unsafe fn wait_for_events(
        cl: &NativeOpenCl,
        num_events: u32,
        event_list: *const *mut c_void,
    ) -> i32 {
        let ev_buf = if num_events <= 4096 {
            stage_in_array(event_list, num_events as usize)
        } else {
            None
        };
        let ev_ptr = ev_buf.as_ref().map_or(event_list, |v| v.as_ptr());
        (cl.wait_for_events)(num_events, ev_ptr)
    }
}

// ---------------------------------------------------------------------------
// Native CPU OpenCL 1.2 Fallback (`PortFactory CPU OpenCL 1.2`)
// Used on iOS and whenever macOS OpenCL.framework is unavailable.
// ---------------------------------------------------------------------------

mod cpu_fallback {
    use super::*;
    use std::collections::HashMap;
    use std::ffi::CStr;
    use std::sync::{Mutex, OnceLock};

    static CPU_PLATFORM_TAG: u8 = 0x50;
    static CPU_DEVICE_TAG: u8 = 0x44;

    const PLATFORM_NAME: &[u8] = b"PortFactory CPU OpenCL 1.2\0";
    const PLATFORM_VENDOR: &[u8] = b"PortFactory\0";
    const PLATFORM_VERSION: &[u8] = b"OpenCL 1.2 PortFactory\0";
    const PLATFORM_PROFILE: &[u8] = b"FULL_PROFILE\0";
    const DEVICE_VERSION: &[u8] = b"OpenCL 1.2 CPU\0";
    const DEVICE_OPENCL_C_VERSION: &[u8] = b"OpenCL C 1.2 \0";
    const DRIVER_VERSION: &[u8] = b"1.2\0";
    const EMPTY_CSTR: &[u8] = b"\0";
    const BUILD_UNSUPPORTED_LOG: &str = "PortFactory CPU OpenCL 1.2 does not provide an OpenCL C compiler on this target; kernel compilation is unsupported.\n";

    #[inline]
    pub fn platform_id() -> *mut c_void {
        std::ptr::addr_of!(CPU_PLATFORM_TAG) as *mut c_void
    }

    #[inline]
    pub fn device_id() -> *mut c_void {
        std::ptr::addr_of!(CPU_DEVICE_TAG) as *mut c_void
    }

    struct CpuContext {
        ref_count: u32,
    }

    struct CpuQueue {
        ref_count: u32,
        context: usize,
    }

    enum BufferBacking {
        Owned(Vec<u8>),
        HostPtr(usize),
    }

    struct CpuBuffer {
        ref_count: u32,
        context: usize,
        size: usize,
        backing: BufferBacking,
    }

    impl CpuBuffer {
        fn data_ptr_mut(&mut self) -> *mut u8 {
            match &mut self.backing {
                BufferBacking::Owned(v) => v.as_mut_ptr(),
                BufferBacking::HostPtr(addr) => *addr as *mut u8,
            }
        }

        fn data_ptr(&self) -> *const u8 {
            match &self.backing {
                BufferBacking::Owned(v) => v.as_ptr(),
                BufferBacking::HostPtr(addr) => *addr as *const u8,
            }
        }
    }

    struct CpuProgram {
        ref_count: u32,
        _context: usize,
        _source: String,
        build_status: i32,
        build_options: String,
        build_log: String,
    }

    struct CpuEvent {
        ref_count: u32,
        _command_type: u32,
        _status: i32,
    }

    struct CpuState {
        next_handle: usize,
        contexts: HashMap<usize, CpuContext>,
        queues: HashMap<usize, CpuQueue>,
        buffers: HashMap<usize, CpuBuffer>,
        programs: HashMap<usize, CpuProgram>,
        events: HashMap<usize, CpuEvent>,
    }

    impl CpuState {
        fn new() -> Self {
            Self {
                next_handle: 0x0C11_0010,
                contexts: HashMap::new(),
                queues: HashMap::new(),
                buffers: HashMap::new(),
                programs: HashMap::new(),
                events: HashMap::new(),
            }
        }

        fn alloc_handle(&mut self) -> usize {
            let h = self.next_handle;
            self.next_handle = self.next_handle.wrapping_add(0x10);
            if self.next_handle == 0 {
                self.next_handle = 0x0C11_0010;
            }
            h
        }

        fn create_event(&mut self, command_type: u32) -> *mut c_void {
            let h = self.alloc_handle();
            self.events.insert(
                h,
                CpuEvent {
                    ref_count: 1,
                    _command_type: command_type,
                    _status: CL_COMPLETE,
                },
            );
            h as *mut c_void
        }

        unsafe fn validate_wait_list(
            &self,
            num_events: u32,
            event_wait_list: *const *mut c_void,
        ) -> i32 {
            if (num_events > 0 && event_wait_list.is_null())
                || (num_events == 0 && !event_wait_list.is_null())
            {
                return CL_INVALID_EVENT_WAIT_LIST;
            }
            for i in 0..(num_events as usize) {
                let ev = std::ptr::read_unaligned(event_wait_list.add(i)) as usize;
                if ev == 0 || !self.events.contains_key(&ev) {
                    return CL_INVALID_EVENT_WAIT_LIST;
                }
            }
            CL_SUCCESS
        }
    }

    fn state() -> &'static Mutex<CpuState> {
        static STATE: OnceLock<Mutex<CpuState>> = OnceLock::new();
        STATE.get_or_init(|| Mutex::new(CpuState::new()))
    }

    fn host_mem_size() -> u64 {
        let mut mem: u64 = 0;
        let mut len = std::mem::size_of::<u64>();
        let rc = unsafe {
            libc::sysctlbyname(
                c"hw.memsize".as_ptr(),
                (&mut mem as *mut u64) as *mut c_void,
                &mut len,
                std::ptr::null_mut(),
                0,
            )
        };
        if rc == 0 && mem > 0 {
            mem
        } else {
            4 * 1024 * 1024 * 1024
        }
    }

    unsafe fn write_info_bytes(
        bytes: &[u8],
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
    ) -> i32 {
        write_opt(param_value_size_ret, bytes.len());
        if !param_value.is_null() {
            if param_value_size < bytes.len() {
                return CL_INVALID_VALUE;
            }
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), param_value as *mut u8, bytes.len());
        }
        CL_SUCCESS
    }

    fn validate_context_properties(props: &Option<Vec<isize>>) -> i32 {
        let Some(list) = props else {
            return CL_SUCCESS;
        };
        let mut i = 0usize;
        let mut seen_platform = false;
        let mut seen_interop_sync = false;
        while i < list.len() {
            let key = list[i];
            if key == 0 {
                return CL_SUCCESS;
            }
            if i + 1 >= list.len() {
                return CL_INVALID_PROPERTY;
            }
            let val = list[i + 1];
            match key {
                CL_CONTEXT_PLATFORM => {
                    if seen_platform {
                        return CL_INVALID_PROPERTY;
                    }
                    seen_platform = true;
                    if val != 0 && (val as usize) != (platform_id() as usize) {
                        return CL_INVALID_PLATFORM;
                    }
                }
                CL_CONTEXT_INTEROP_USER_SYNC => {
                    if seen_interop_sync {
                        return CL_INVALID_PROPERTY;
                    }
                    seen_interop_sync = true;
                }
                _ => return CL_INVALID_PROPERTY,
            }
            i += 2;
        }
        CL_SUCCESS
    }

    fn validate_device_type(device_type: u64) -> Result<bool, i32> {
        let valid_mask = CL_DEVICE_TYPE_DEFAULT
            | CL_DEVICE_TYPE_CPU
            | CL_DEVICE_TYPE_GPU
            | CL_DEVICE_TYPE_ACCELERATOR
            | CL_DEVICE_TYPE_CUSTOM;
        if device_type == 0
            || (device_type != CL_DEVICE_TYPE_ALL && (device_type & !valid_mask) != 0)
        {
            return Err(CL_INVALID_DEVICE_TYPE);
        }
        let matches = device_type == CL_DEVICE_TYPE_ALL
            || (device_type & (CL_DEVICE_TYPE_CPU | CL_DEVICE_TYPE_DEFAULT)) != 0;
        Ok(matches)
    }

    pub unsafe fn get_platform_ids(
        num_entries: u32,
        platforms: *mut *mut c_void,
        num_platforms: *mut u32,
    ) -> i32 {
        if (num_entries == 0 && !platforms.is_null())
            || (platforms.is_null() && num_platforms.is_null())
        {
            return CL_INVALID_VALUE;
        }
        write_opt(num_platforms, 1);
        if !platforms.is_null() {
            std::ptr::write_unaligned(platforms, platform_id());
        }
        CL_SUCCESS
    }

    pub unsafe fn get_platform_info(
        platform: *mut c_void,
        param_name: u32,
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
    ) -> i32 {
        if !platform.is_null() && platform != platform_id() {
            return CL_INVALID_PLATFORM;
        }
        let bytes: &[u8] = match param_name {
            CL_PLATFORM_PROFILE => PLATFORM_PROFILE,
            CL_PLATFORM_VERSION => PLATFORM_VERSION,
            CL_PLATFORM_NAME => PLATFORM_NAME,
            CL_PLATFORM_VENDOR => PLATFORM_VENDOR,
            CL_PLATFORM_EXTENSIONS => EMPTY_CSTR,
            _ => return CL_INVALID_VALUE,
        };
        write_info_bytes(bytes, param_value_size, param_value, param_value_size_ret)
    }

    pub unsafe fn get_device_ids(
        platform: *mut c_void,
        device_type: u64,
        num_entries: u32,
        devices: *mut *mut c_void,
        num_devices: *mut u32,
    ) -> i32 {
        if !platform.is_null() && platform != platform_id() {
            return CL_INVALID_PLATFORM;
        }
        if (num_entries == 0 && !devices.is_null())
            || (devices.is_null() && num_devices.is_null())
        {
            return CL_INVALID_VALUE;
        }
        let matches = match validate_device_type(device_type) {
            Ok(m) => m,
            Err(e) => return e,
        };
        if !matches {
            write_opt(num_devices, 0);
            return CL_DEVICE_NOT_FOUND;
        }
        write_opt(num_devices, 1);
        if !devices.is_null() {
            std::ptr::write_unaligned(devices, device_id());
        }
        CL_SUCCESS
    }

    pub unsafe fn get_device_info(
        device: *mut c_void,
        param_name: u32,
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
    ) -> i32 {
        if device != device_id() {
            return CL_INVALID_DEVICE;
        }
        let cores = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(4);
        let global_mem = host_mem_size();
        let max_alloc = (global_mem / 4).max(256 * 1024 * 1024);
        let fp_cfg = CL_FP_DENORM
            | CL_FP_INF_NAN
            | CL_FP_ROUND_TO_NEAREST
            | CL_FP_ROUND_TO_ZERO
            | CL_FP_ROUND_TO_INF
            | CL_FP_FMA;

        macro_rules! ret_val {
            ($v:expr) => {{
                let val = $v;
                let bytes = std::slice::from_raw_parts(
                    (&val as *const _) as *const u8,
                    std::mem::size_of_val(&val),
                );
                return write_info_bytes(
                    bytes,
                    param_value_size,
                    param_value,
                    param_value_size_ret,
                );
            }};
        }

        match param_name {
            CL_DEVICE_TYPE => ret_val!(CL_DEVICE_TYPE_CPU | CL_DEVICE_TYPE_DEFAULT),
            CL_DEVICE_VENDOR_ID => ret_val!(0x106Bu32),
            CL_DEVICE_MAX_COMPUTE_UNITS => ret_val!(cores),
            CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS => ret_val!(3u32),
            CL_DEVICE_MAX_WORK_GROUP_SIZE => ret_val!(1024usize),
            CL_DEVICE_MAX_WORK_ITEM_SIZES => ret_val!([1024usize, 1024usize, 1024usize]),
            CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR | CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR => {
                ret_val!(16u32)
            }
            CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT | CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT => {
                ret_val!(8u32)
            }
            CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT | CL_DEVICE_NATIVE_VECTOR_WIDTH_INT => {
                ret_val!(4u32)
            }
            CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG | CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG => {
                ret_val!(2u32)
            }
            CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT | CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT => {
                ret_val!(4u32)
            }
            CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE | CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE => {
                ret_val!(2u32)
            }
            CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF | CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF => {
                ret_val!(0u32)
            }
            CL_DEVICE_MAX_CLOCK_FREQUENCY => ret_val!(3000u32),
            CL_DEVICE_ADDRESS_BITS => ret_val!(64u32),
            CL_DEVICE_MAX_READ_IMAGE_ARGS
            | CL_DEVICE_MAX_WRITE_IMAGE_ARGS
            | CL_DEVICE_MAX_SAMPLERS
            | CL_DEVICE_PARTITION_MAX_SUB_DEVICES => ret_val!(0u32),
            CL_DEVICE_IMAGE2D_MAX_WIDTH
            | CL_DEVICE_IMAGE2D_MAX_HEIGHT
            | CL_DEVICE_IMAGE3D_MAX_WIDTH
            | CL_DEVICE_IMAGE3D_MAX_HEIGHT
            | CL_DEVICE_IMAGE3D_MAX_DEPTH
            | CL_DEVICE_IMAGE_MAX_BUFFER_SIZE
            | CL_DEVICE_IMAGE_MAX_ARRAY_SIZE => ret_val!(0usize),
            CL_DEVICE_IMAGE_SUPPORT => ret_val!(CL_FALSE),
            CL_DEVICE_MAX_MEM_ALLOC_SIZE => ret_val!(max_alloc),
            CL_DEVICE_MAX_PARAMETER_SIZE => ret_val!(1024usize),
            CL_DEVICE_MEM_BASE_ADDR_ALIGN => ret_val!(1024u32),
            CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE => ret_val!(128u32),
            CL_DEVICE_SINGLE_FP_CONFIG | CL_DEVICE_DOUBLE_FP_CONFIG => ret_val!(fp_cfg),
            CL_DEVICE_GLOBAL_MEM_CACHE_TYPE => ret_val!(CL_READ_WRITE_CACHE),
            CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE => ret_val!(128u32),
            CL_DEVICE_GLOBAL_MEM_CACHE_SIZE => ret_val!(4u64 * 1024 * 1024),
            CL_DEVICE_GLOBAL_MEM_SIZE => ret_val!(global_mem),
            CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE => ret_val!(64u64 * 1024),
            CL_DEVICE_MAX_CONSTANT_ARGS => ret_val!(8u32),
            CL_DEVICE_LOCAL_MEM_TYPE => ret_val!(CL_GLOBAL),
            CL_DEVICE_LOCAL_MEM_SIZE => ret_val!(32u64 * 1024),
            CL_DEVICE_ERROR_CORRECTION_SUPPORT => ret_val!(CL_FALSE),
            CL_DEVICE_HOST_UNIFIED_MEMORY => ret_val!(CL_TRUE),
            CL_DEVICE_PROFILING_TIMER_RESOLUTION => ret_val!(1usize),
            CL_DEVICE_ENDIAN_LITTLE => ret_val!(CL_TRUE),
            CL_DEVICE_AVAILABLE => ret_val!(CL_TRUE),
            CL_DEVICE_COMPILER_AVAILABLE => ret_val!(CL_FALSE),
            CL_DEVICE_LINKER_AVAILABLE => ret_val!(CL_FALSE),
            CL_DEVICE_EXECUTION_CAPABILITIES => ret_val!(CL_EXEC_KERNEL),
            CL_DEVICE_QUEUE_PROPERTIES => ret_val!(CL_QUEUE_PROFILING_ENABLE),
            CL_DEVICE_PLATFORM => ret_val!(platform_id() as usize),
            CL_DEVICE_PARENT_DEVICE => ret_val!(0usize),
            CL_DEVICE_REFERENCE_COUNT => ret_val!(1u32),
            CL_DEVICE_PREFERRED_INTEROP_USER_SYNC => ret_val!(CL_TRUE),
            CL_DEVICE_PRINTF_BUFFER_SIZE => ret_val!(1024usize * 1024),
            CL_DEVICE_NAME => {
                write_info_bytes(PLATFORM_NAME, param_value_size, param_value, param_value_size_ret)
            }
            CL_DEVICE_VENDOR => write_info_bytes(
                PLATFORM_VENDOR,
                param_value_size,
                param_value,
                param_value_size_ret,
            ),
            CL_DRIVER_VERSION => write_info_bytes(
                DRIVER_VERSION,
                param_value_size,
                param_value,
                param_value_size_ret,
            ),
            CL_DEVICE_PROFILE => write_info_bytes(
                PLATFORM_PROFILE,
                param_value_size,
                param_value,
                param_value_size_ret,
            ),
            CL_DEVICE_VERSION => write_info_bytes(
                DEVICE_VERSION,
                param_value_size,
                param_value,
                param_value_size_ret,
            ),
            CL_DEVICE_OPENCL_C_VERSION => write_info_bytes(
                DEVICE_OPENCL_C_VERSION,
                param_value_size,
                param_value,
                param_value_size_ret,
            ),
            CL_DEVICE_EXTENSIONS | CL_DEVICE_BUILT_IN_KERNELS => write_info_bytes(
                EMPTY_CSTR,
                param_value_size,
                param_value,
                param_value_size_ret,
            ),
            _ => CL_INVALID_VALUE,
        }
    }

    pub unsafe fn create_context(
        properties: *const isize,
        num_devices: u32,
        devices: *const *mut c_void,
        pfn_notify: usize,
        user_data: *mut c_void,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        if num_devices == 0 || devices.is_null() || (pfn_notify == 0 && !user_data.is_null()) {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        let props = read_context_properties(properties);
        let prop_rc = validate_context_properties(&props);
        if prop_rc != CL_SUCCESS {
            write_opt(errcode_ret, prop_rc);
            return std::ptr::null_mut();
        }
        for i in 0..(num_devices as usize) {
            let dev = std::ptr::read_unaligned(devices.add(i));
            if dev != device_id() {
                write_opt(errcode_ret, CL_INVALID_DEVICE);
                return std::ptr::null_mut();
            }
        }
        let mut st = state().lock().unwrap();
        let h = st.alloc_handle();
        st.contexts.insert(h, CpuContext { ref_count: 1 });
        write_opt(errcode_ret, CL_SUCCESS);
        h as *mut c_void
    }

    pub unsafe fn create_context_from_type(
        properties: *const isize,
        device_type: u64,
        pfn_notify: usize,
        user_data: *mut c_void,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        if pfn_notify == 0 && !user_data.is_null() {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        let props = read_context_properties(properties);
        let prop_rc = validate_context_properties(&props);
        if prop_rc != CL_SUCCESS {
            write_opt(errcode_ret, prop_rc);
            return std::ptr::null_mut();
        }
        match validate_device_type(device_type) {
            Ok(true) => {}
            Ok(false) => {
                write_opt(errcode_ret, CL_DEVICE_NOT_FOUND);
                return std::ptr::null_mut();
            }
            Err(e) => {
                write_opt(errcode_ret, e);
                return std::ptr::null_mut();
            }
        }
        let mut st = state().lock().unwrap();
        let h = st.alloc_handle();
        st.contexts.insert(h, CpuContext { ref_count: 1 });
        write_opt(errcode_ret, CL_SUCCESS);
        h as *mut c_void
    }

    pub fn retain_context(context: *mut c_void) -> i32 {
        let mut st = state().lock().unwrap();
        let Some(ctx) = st.contexts.get_mut(&(context as usize)) else {
            return CL_INVALID_CONTEXT;
        };
        ctx.ref_count = ctx.ref_count.saturating_add(1);
        CL_SUCCESS
    }

    pub fn release_context(context: *mut c_void) -> i32 {
        let mut st = state().lock().unwrap();
        let key = context as usize;
        let Some(ctx) = st.contexts.get_mut(&key) else {
            return CL_INVALID_CONTEXT;
        };
        if ctx.ref_count <= 1 {
            st.contexts.remove(&key);
        } else {
            ctx.ref_count -= 1;
        }
        CL_SUCCESS
    }

    pub unsafe fn create_command_queue(
        context: *mut c_void,
        device: *mut c_void,
        properties: u64,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        let valid_props = CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE | CL_QUEUE_PROFILING_ENABLE;
        if (properties & !valid_props) != 0 {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        if (properties & CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE) != 0 {
            write_opt(errcode_ret, CL_INVALID_QUEUE_PROPERTIES);
            return std::ptr::null_mut();
        }
        if device != device_id() {
            write_opt(errcode_ret, CL_INVALID_DEVICE);
            return std::ptr::null_mut();
        }
        let mut st = state().lock().unwrap();
        let ctx_key = context as usize;
        if !st.contexts.contains_key(&ctx_key) {
            write_opt(errcode_ret, CL_INVALID_CONTEXT);
            return std::ptr::null_mut();
        }
        let h = st.alloc_handle();
        st.queues.insert(
            h,
            CpuQueue {
                ref_count: 1,
                context: ctx_key,
            },
        );
        write_opt(errcode_ret, CL_SUCCESS);
        h as *mut c_void
    }

    pub fn release_command_queue(command_queue: *mut c_void) -> i32 {
        let mut st = state().lock().unwrap();
        let key = command_queue as usize;
        let Some(q) = st.queues.get_mut(&key) else {
            return CL_INVALID_COMMAND_QUEUE;
        };
        if q.ref_count <= 1 {
            st.queues.remove(&key);
        } else {
            q.ref_count -= 1;
        }
        CL_SUCCESS
    }

    pub unsafe fn create_buffer(
        context: *mut c_void,
        flags: u64,
        size: usize,
        host_ptr: *mut c_void,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        if size == 0 {
            write_opt(errcode_ret, CL_INVALID_BUFFER_SIZE);
            return std::ptr::null_mut();
        }
        let rw_bits = flags & (CL_MEM_READ_WRITE | CL_MEM_WRITE_ONLY | CL_MEM_READ_ONLY);
        if rw_bits.count_ones() > 1 {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        let host_access_bits =
            flags & (CL_MEM_HOST_WRITE_ONLY | CL_MEM_HOST_READ_ONLY | CL_MEM_HOST_NO_ACCESS);
        if host_access_bits.count_ones() > 1 {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        let use_host = (flags & CL_MEM_USE_HOST_PTR) != 0;
        let alloc_host = (flags & CL_MEM_ALLOC_HOST_PTR) != 0;
        let copy_host = (flags & CL_MEM_COPY_HOST_PTR) != 0;
        if use_host && (alloc_host || copy_host) {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        if (use_host || copy_host) && host_ptr.is_null() {
            write_opt(errcode_ret, CL_INVALID_HOST_PTR);
            return std::ptr::null_mut();
        }
        if !use_host && !copy_host && !host_ptr.is_null() {
            write_opt(errcode_ret, CL_INVALID_HOST_PTR);
            return std::ptr::null_mut();
        }

        let ctx_key = context as usize;
        let mut st = state().lock().unwrap();
        if !st.contexts.contains_key(&ctx_key) {
            write_opt(errcode_ret, CL_INVALID_CONTEXT);
            return std::ptr::null_mut();
        }

        let backing = if use_host {
            BufferBacking::HostPtr(host_ptr as usize)
        } else {
            let mut data = vec![0u8; size];
            if copy_host {
                std::ptr::copy_nonoverlapping(host_ptr as *const u8, data.as_mut_ptr(), size);
            }
            BufferBacking::Owned(data)
        };

        let h = st.alloc_handle();
        st.buffers.insert(
            h,
            CpuBuffer {
                ref_count: 1,
                context: ctx_key,
                size,
                backing,
            },
        );
        write_opt(errcode_ret, CL_SUCCESS);
        h as *mut c_void
    }

    pub fn release_mem_object(memobj: *mut c_void) -> i32 {
        let mut st = state().lock().unwrap();
        let key = memobj as usize;
        let Some(buf) = st.buffers.get_mut(&key) else {
            return CL_INVALID_MEM_OBJECT;
        };
        if buf.ref_count <= 1 {
            st.buffers.remove(&key);
        } else {
            buf.ref_count -= 1;
        }
        CL_SUCCESS
    }

    pub unsafe fn enqueue_read_buffer(
        command_queue: *mut c_void,
        buffer: *mut c_void,
        _blocking_read: u32,
        offset: usize,
        size: usize,
        ptr: *mut c_void,
        num_events_in_wait_list: u32,
        event_wait_list: *const *mut c_void,
        event: *mut *mut c_void,
    ) -> i32 {
        if ptr.is_null() {
            return CL_INVALID_VALUE;
        }
        let mut st = state().lock().unwrap();
        let Some(q) = st.queues.get(&(command_queue as usize)) else {
            return CL_INVALID_COMMAND_QUEUE;
        };
        let q_ctx = q.context;
        let wl_rc = st.validate_wait_list(num_events_in_wait_list, event_wait_list);
        if wl_rc != CL_SUCCESS {
            return wl_rc;
        }
        let Some(buf) = st.buffers.get(&(buffer as usize)) else {
            return CL_INVALID_MEM_OBJECT;
        };
        if buf.context != q_ctx {
            return CL_INVALID_CONTEXT;
        }
        if offset.checked_add(size).is_none_or(|end| end > buf.size) {
            return CL_INVALID_VALUE;
        }
        if size > 0 {
            let src = buf.data_ptr().add(offset);
            std::ptr::copy(src, ptr as *mut u8, size);
        }
        if !event.is_null() {
            let ev = st.create_event(CL_COMMAND_READ_BUFFER);
            std::ptr::write_unaligned(event, ev);
        }
        CL_SUCCESS
    }

    pub unsafe fn enqueue_write_buffer(
        command_queue: *mut c_void,
        buffer: *mut c_void,
        _blocking_write: u32,
        offset: usize,
        size: usize,
        ptr: *const c_void,
        num_events_in_wait_list: u32,
        event_wait_list: *const *mut c_void,
        event: *mut *mut c_void,
    ) -> i32 {
        if ptr.is_null() {
            return CL_INVALID_VALUE;
        }
        let mut st = state().lock().unwrap();
        let Some(q) = st.queues.get(&(command_queue as usize)) else {
            return CL_INVALID_COMMAND_QUEUE;
        };
        let q_ctx = q.context;
        let wl_rc = st.validate_wait_list(num_events_in_wait_list, event_wait_list);
        if wl_rc != CL_SUCCESS {
            return wl_rc;
        }
        let Some(buf) = st.buffers.get_mut(&(buffer as usize)) else {
            return CL_INVALID_MEM_OBJECT;
        };
        if buf.context != q_ctx {
            return CL_INVALID_CONTEXT;
        }
        if offset.checked_add(size).is_none_or(|end| end > buf.size) {
            return CL_INVALID_VALUE;
        }
        if size > 0 {
            let dst = buf.data_ptr_mut().add(offset);
            std::ptr::copy(ptr as *const u8, dst, size);
        }
        if !event.is_null() {
            let ev = st.create_event(CL_COMMAND_WRITE_BUFFER);
            std::ptr::write_unaligned(event, ev);
        }
        CL_SUCCESS
    }

    pub unsafe fn enqueue_copy_buffer(
        command_queue: *mut c_void,
        src_buffer: *mut c_void,
        dst_buffer: *mut c_void,
        src_offset: usize,
        dst_offset: usize,
        size: usize,
        num_events_in_wait_list: u32,
        event_wait_list: *const *mut c_void,
        event: *mut *mut c_void,
    ) -> i32 {
        let mut st = state().lock().unwrap();
        let Some(q) = st.queues.get(&(command_queue as usize)) else {
            return CL_INVALID_COMMAND_QUEUE;
        };
        let q_ctx = q.context;
        let wl_rc = st.validate_wait_list(num_events_in_wait_list, event_wait_list);
        if wl_rc != CL_SUCCESS {
            return wl_rc;
        }
        let src_key = src_buffer as usize;
        let dst_key = dst_buffer as usize;

        let (src_ctx, src_size, src_ptr) = {
            let Some(b) = st.buffers.get(&src_key) else {
                return CL_INVALID_MEM_OBJECT;
            };
            (b.context, b.size, b.data_ptr())
        };
        let (dst_ctx, dst_size, dst_ptr) = {
            let Some(b) = st.buffers.get_mut(&dst_key) else {
                return CL_INVALID_MEM_OBJECT;
            };
            (b.context, b.size, b.data_ptr_mut())
        };
        if src_ctx != q_ctx || dst_ctx != q_ctx {
            return CL_INVALID_CONTEXT;
        }
        let Some(src_end) = src_offset.checked_add(size) else {
            return CL_INVALID_VALUE;
        };
        let Some(dst_end) = dst_offset.checked_add(size) else {
            return CL_INVALID_VALUE;
        };
        if src_end > src_size || dst_end > dst_size {
            return CL_INVALID_VALUE;
        }
        if src_key == dst_key && size > 0 && src_offset < dst_end && dst_offset < src_end {
            return CL_MEM_COPY_OVERLAP;
        }
        if size > 0 {
            std::ptr::copy(src_ptr.add(src_offset), dst_ptr.add(dst_offset), size);
        }
        if !event.is_null() {
            let ev = st.create_event(CL_COMMAND_COPY_BUFFER);
            std::ptr::write_unaligned(event, ev);
        }
        CL_SUCCESS
    }

    pub unsafe fn create_program_with_source(
        context: *mut c_void,
        count: u32,
        strings: *const *const c_char,
        lengths: *const usize,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        if count == 0 || strings.is_null() {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        let mut source = String::new();
        for i in 0..(count as usize) {
            let s_ptr = std::ptr::read_unaligned(strings.add(i));
            if s_ptr.is_null() {
                write_opt(errcode_ret, CL_INVALID_VALUE);
                return std::ptr::null_mut();
            }
            let explicit_len = if lengths.is_null() {
                0
            } else {
                std::ptr::read_unaligned(lengths.add(i))
            };
            let bytes = if explicit_len == 0 {
                CStr::from_ptr(s_ptr).to_bytes()
            } else {
                std::slice::from_raw_parts(s_ptr as *const u8, explicit_len)
            };
            source.push_str(&String::from_utf8_lossy(bytes));
        }

        let ctx_key = context as usize;
        let mut st = state().lock().unwrap();
        if !st.contexts.contains_key(&ctx_key) {
            write_opt(errcode_ret, CL_INVALID_CONTEXT);
            return std::ptr::null_mut();
        }
        let h = st.alloc_handle();
        st.programs.insert(
            h,
            CpuProgram {
                ref_count: 1,
                _context: ctx_key,
                _source: source,
                build_status: CL_BUILD_NONE,
                build_options: String::new(),
                build_log: String::new(),
            },
        );
        write_opt(errcode_ret, CL_SUCCESS);
        h as *mut c_void
    }

    pub unsafe fn build_program(
        program: *mut c_void,
        num_devices: u32,
        device_list: *const *mut c_void,
        options: *const c_char,
        pfn_notify: usize,
        user_data: *mut c_void,
    ) -> i32 {
        if (num_devices > 0 && device_list.is_null())
            || (num_devices == 0 && !device_list.is_null())
            || (pfn_notify == 0 && !user_data.is_null())
        {
            return CL_INVALID_VALUE;
        }
        for i in 0..(num_devices as usize) {
            let dev = std::ptr::read_unaligned(device_list.add(i));
            if dev != device_id() {
                return CL_INVALID_DEVICE;
            }
        }
        let opts = if options.is_null() {
            String::new()
        } else {
            CStr::from_ptr(options).to_string_lossy().into_owned()
        };
        {
            let mut st = state().lock().unwrap();
            let Some(prog) = st.programs.get_mut(&(program as usize)) else {
                return CL_INVALID_PROGRAM;
            };
            prog.build_options = opts;
            prog.build_status = CL_BUILD_ERROR;
            prog.build_log = BUILD_UNSUPPORTED_LOG.to_string();
        }
        if pfn_notify != 0 {
            let _ = crate::call_guest8(
                pfn_notify,
                program as u64,
                user_data as u64,
                0,
                0,
                0,
                0,
                0,
                0,
            );
        }
        CL_BUILD_PROGRAM_FAILURE
    }

    pub unsafe fn get_program_build_info(
        program: *mut c_void,
        device: *mut c_void,
        param_name: u32,
        param_value_size: usize,
        param_value: *mut c_void,
        param_value_size_ret: *mut usize,
    ) -> i32 {
        if device != device_id() {
            return CL_INVALID_DEVICE;
        }
        let st = state().lock().unwrap();
        let Some(prog) = st.programs.get(&(program as usize)) else {
            return CL_INVALID_PROGRAM;
        };
        match param_name {
            CL_PROGRAM_BUILD_STATUS => {
                let status = prog.build_status;
                let bytes = std::slice::from_raw_parts(
                    (&status as *const i32) as *const u8,
                    std::mem::size_of::<i32>(),
                );
                write_info_bytes(bytes, param_value_size, param_value, param_value_size_ret)
            }
            CL_PROGRAM_BUILD_OPTIONS => {
                let mut buf = prog.build_options.as_bytes().to_vec();
                buf.push(0);
                write_info_bytes(&buf, param_value_size, param_value, param_value_size_ret)
            }
            CL_PROGRAM_BUILD_LOG => {
                let mut buf = prog.build_log.as_bytes().to_vec();
                buf.push(0);
                write_info_bytes(&buf, param_value_size, param_value, param_value_size_ret)
            }
            CL_PROGRAM_BINARY_TYPE => {
                let bt = CL_PROGRAM_BINARY_TYPE_NONE;
                let bytes = std::slice::from_raw_parts(
                    (&bt as *const u32) as *const u8,
                    std::mem::size_of::<u32>(),
                );
                write_info_bytes(bytes, param_value_size, param_value, param_value_size_ret)
            }
            _ => CL_INVALID_VALUE,
        }
    }

    pub fn release_program(program: *mut c_void) -> i32 {
        let mut st = state().lock().unwrap();
        let key = program as usize;
        let Some(prog) = st.programs.get_mut(&key) else {
            return CL_INVALID_PROGRAM;
        };
        if prog.ref_count <= 1 {
            st.programs.remove(&key);
        } else {
            prog.ref_count -= 1;
        }
        CL_SUCCESS
    }

    pub unsafe fn create_kernel(
        program: *mut c_void,
        kernel_name: *const c_char,
        errcode_ret: *mut i32,
    ) -> *mut c_void {
        if kernel_name.is_null() {
            write_opt(errcode_ret, CL_INVALID_VALUE);
            return std::ptr::null_mut();
        }
        let st = state().lock().unwrap();
        let Some(prog) = st.programs.get(&(program as usize)) else {
            write_opt(errcode_ret, CL_INVALID_PROGRAM);
            return std::ptr::null_mut();
        };
        let err = if prog.build_status == CL_BUILD_SUCCESS {
            CL_INVALID_KERNEL_NAME
        } else {
            CL_INVALID_PROGRAM_EXECUTABLE
        };
        write_opt(errcode_ret, err);
        std::ptr::null_mut()
    }

    pub fn set_kernel_arg(_kernel: *mut c_void) -> i32 {
        CL_INVALID_KERNEL
    }

    pub fn enqueue_nd_range_kernel(command_queue: *mut c_void) -> i32 {
        let st = state().lock().unwrap();
        if !st.queues.contains_key(&(command_queue as usize)) {
            return CL_INVALID_COMMAND_QUEUE;
        }
        CL_INVALID_KERNEL
    }

    pub fn release_kernel(_kernel: *mut c_void) -> i32 {
        CL_INVALID_KERNEL
    }

    pub fn flush_or_finish(command_queue: *mut c_void) -> i32 {
        let st = state().lock().unwrap();
        if st.queues.contains_key(&(command_queue as usize)) {
            CL_SUCCESS
        } else {
            CL_INVALID_COMMAND_QUEUE
        }
    }

    pub unsafe fn wait_for_events(num_events: u32, event_list: *const *mut c_void) -> i32 {
        if num_events == 0 || event_list.is_null() {
            return CL_INVALID_VALUE;
        }
        let st = state().lock().unwrap();
        for i in 0..(num_events as usize) {
            let ev = std::ptr::read_unaligned(event_list.add(i)) as usize;
            if ev == 0 || !st.events.contains_key(&ev) {
                return CL_INVALID_EVENT;
            }
        }
        CL_SUCCESS
    }

    pub fn release_event(event: *mut c_void) -> i32 {
        let mut st = state().lock().unwrap();
        let key = event as usize;
        let Some(ev) = st.events.get_mut(&key) else {
            return CL_INVALID_EVENT;
        };
        if ev.ref_count <= 1 {
            st.events.remove(&key);
        } else {
            ev.ref_count -= 1;
        }
        CL_SUCCESS
    }
}

// ---------------------------------------------------------------------------
// Exported OpenCL 1.2 Win64 Entry Points (27 functions)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn clGetPlatformIDs(
    num_entries: u32,
    platforms: *mut *mut c_void,
    num_platforms: *mut u32,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::get_platform_ids(cl, num_entries, platforms, num_platforms);
    }
    cpu_fallback::get_platform_ids(num_entries, platforms, num_platforms)
}

pub unsafe extern "win64" fn clGetPlatformInfo(
    platform: *mut c_void,
    param_name: u32,
    param_value_size: usize,
    param_value: *mut c_void,
    param_value_size_ret: *mut usize,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::get_platform_info(
            cl,
            platform,
            param_name,
            param_value_size,
            param_value,
            param_value_size_ret,
        );
    }
    cpu_fallback::get_platform_info(
        platform,
        param_name,
        param_value_size,
        param_value,
        param_value_size_ret,
    )
}

pub unsafe extern "win64" fn clGetDeviceIDs(
    platform: *mut c_void,
    device_type: u64,
    num_entries: u32,
    devices: *mut *mut c_void,
    num_devices: *mut u32,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::get_device_ids(cl, platform, device_type, num_entries, devices, num_devices);
    }
    cpu_fallback::get_device_ids(platform, device_type, num_entries, devices, num_devices)
}

pub unsafe extern "win64" fn clGetDeviceInfo(
    device: *mut c_void,
    param_name: u32,
    param_value_size: usize,
    param_value: *mut c_void,
    param_value_size_ret: *mut usize,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::get_device_info(
            cl,
            device,
            param_name,
            param_value_size,
            param_value,
            param_value_size_ret,
        );
    }
    cpu_fallback::get_device_info(
        device,
        param_name,
        param_value_size,
        param_value,
        param_value_size_ret,
    )
}

pub unsafe extern "win64" fn clCreateContext(
    properties: *const isize,
    num_devices: u32,
    devices: *const *mut c_void,
    pfn_notify: usize,
    user_data: *mut c_void,
    errcode_ret: *mut i32,
) -> *mut c_void {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::create_context(
            cl,
            properties,
            num_devices,
            devices,
            pfn_notify,
            user_data,
            errcode_ret,
        );
    }
    cpu_fallback::create_context(
        properties,
        num_devices,
        devices,
        pfn_notify,
        user_data,
        errcode_ret,
    )
}

pub unsafe extern "win64" fn clCreateContextFromType(
    properties: *const isize,
    device_type: u64,
    pfn_notify: usize,
    user_data: *mut c_void,
    errcode_ret: *mut i32,
) -> *mut c_void {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::create_context_from_type(
            cl,
            properties,
            device_type,
            pfn_notify,
            user_data,
            errcode_ret,
        );
    }
    cpu_fallback::create_context_from_type(
        properties,
        device_type,
        pfn_notify,
        user_data,
        errcode_ret,
    )
}

pub unsafe extern "win64" fn clRetainContext(context: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.retain_context)(context);
    }
    cpu_fallback::retain_context(context)
}

pub unsafe extern "win64" fn clReleaseContext(context: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.release_context)(context);
    }
    cpu_fallback::release_context(context)
}

pub unsafe extern "win64" fn clCreateCommandQueue(
    context: *mut c_void,
    device: *mut c_void,
    properties: u64,
    errcode_ret: *mut i32,
) -> *mut c_void {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::create_command_queue(cl, context, device, properties, errcode_ret);
    }
    cpu_fallback::create_command_queue(context, device, properties, errcode_ret)
}

pub unsafe extern "win64" fn clReleaseCommandQueue(command_queue: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.release_command_queue)(command_queue);
    }
    cpu_fallback::release_command_queue(command_queue)
}

pub unsafe extern "win64" fn clCreateBuffer(
    context: *mut c_void,
    flags: u64,
    size: usize,
    host_ptr: *mut c_void,
    errcode_ret: *mut i32,
) -> *mut c_void {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::create_buffer(cl, context, flags, size, host_ptr, errcode_ret);
    }
    cpu_fallback::create_buffer(context, flags, size, host_ptr, errcode_ret)
}

pub unsafe extern "win64" fn clReleaseMemObject(memobj: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.release_mem_object)(memobj);
    }
    cpu_fallback::release_mem_object(memobj)
}

pub unsafe extern "win64" fn clEnqueueReadBuffer(
    command_queue: *mut c_void,
    buffer: *mut c_void,
    blocking_read: u32,
    offset: usize,
    size: usize,
    ptr: *mut c_void,
    num_events_in_wait_list: u32,
    event_wait_list: *const *mut c_void,
    event: *mut *mut c_void,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::enqueue_read_buffer(
            cl,
            command_queue,
            buffer,
            blocking_read,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            event,
        );
    }
    cpu_fallback::enqueue_read_buffer(
        command_queue,
        buffer,
        blocking_read,
        offset,
        size,
        ptr,
        num_events_in_wait_list,
        event_wait_list,
        event,
    )
}

pub unsafe extern "win64" fn clEnqueueWriteBuffer(
    command_queue: *mut c_void,
    buffer: *mut c_void,
    blocking_write: u32,
    offset: usize,
    size: usize,
    ptr: *const c_void,
    num_events_in_wait_list: u32,
    event_wait_list: *const *mut c_void,
    event: *mut *mut c_void,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::enqueue_write_buffer(
            cl,
            command_queue,
            buffer,
            blocking_write,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            event,
        );
    }
    cpu_fallback::enqueue_write_buffer(
        command_queue,
        buffer,
        blocking_write,
        offset,
        size,
        ptr,
        num_events_in_wait_list,
        event_wait_list,
        event,
    )
}

pub unsafe extern "win64" fn clEnqueueCopyBuffer(
    command_queue: *mut c_void,
    src_buffer: *mut c_void,
    dst_buffer: *mut c_void,
    src_offset: usize,
    dst_offset: usize,
    size: usize,
    num_events_in_wait_list: u32,
    event_wait_list: *const *mut c_void,
    event: *mut *mut c_void,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::enqueue_copy_buffer(
            cl,
            command_queue,
            src_buffer,
            dst_buffer,
            src_offset,
            dst_offset,
            size,
            num_events_in_wait_list,
            event_wait_list,
            event,
        );
    }
    cpu_fallback::enqueue_copy_buffer(
        command_queue,
        src_buffer,
        dst_buffer,
        src_offset,
        dst_offset,
        size,
        num_events_in_wait_list,
        event_wait_list,
        event,
    )
}

pub unsafe extern "win64" fn clCreateProgramWithSource(
    context: *mut c_void,
    count: u32,
    strings: *const *const c_char,
    lengths: *const usize,
    errcode_ret: *mut i32,
) -> *mut c_void {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::create_program_with_source(cl, context, count, strings, lengths, errcode_ret);
    }
    cpu_fallback::create_program_with_source(context, count, strings, lengths, errcode_ret)
}

pub unsafe extern "win64" fn clBuildProgram(
    program: *mut c_void,
    num_devices: u32,
    device_list: *const *mut c_void,
    options: *const c_char,
    pfn_notify: usize,
    user_data: *mut c_void,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::build_program(
            cl,
            program,
            num_devices,
            device_list,
            options,
            pfn_notify,
            user_data,
        );
    }
    cpu_fallback::build_program(
        program,
        num_devices,
        device_list,
        options,
        pfn_notify,
        user_data,
    )
}

pub unsafe extern "win64" fn clGetProgramBuildInfo(
    program: *mut c_void,
    device: *mut c_void,
    param_name: u32,
    param_value_size: usize,
    param_value: *mut c_void,
    param_value_size_ret: *mut usize,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::get_program_build_info(
            cl,
            program,
            device,
            param_name,
            param_value_size,
            param_value,
            param_value_size_ret,
        );
    }
    cpu_fallback::get_program_build_info(
        program,
        device,
        param_name,
        param_value_size,
        param_value,
        param_value_size_ret,
    )
}

pub unsafe extern "win64" fn clReleaseProgram(program: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.release_program)(program);
    }
    cpu_fallback::release_program(program)
}

pub unsafe extern "win64" fn clCreateKernel(
    program: *mut c_void,
    kernel_name: *const c_char,
    errcode_ret: *mut i32,
) -> *mut c_void {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::create_kernel(cl, program, kernel_name, errcode_ret);
    }
    cpu_fallback::create_kernel(program, kernel_name, errcode_ret)
}

pub unsafe extern "win64" fn clSetKernelArg(
    kernel: *mut c_void,
    arg_index: u32,
    arg_size: usize,
    arg_value: *const c_void,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::set_kernel_arg(cl, kernel, arg_index, arg_size, arg_value);
    }
    let _ = (arg_index, arg_size, arg_value);
    cpu_fallback::set_kernel_arg(kernel)
}

pub unsafe extern "win64" fn clEnqueueNDRangeKernel(
    command_queue: *mut c_void,
    kernel: *mut c_void,
    work_dim: u32,
    global_work_offset: *const usize,
    global_work_size: *const usize,
    local_work_size: *const usize,
    num_events_in_wait_list: u32,
    event_wait_list: *const *mut c_void,
    event: *mut *mut c_void,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::enqueue_nd_range_kernel(
            cl,
            command_queue,
            kernel,
            work_dim,
            global_work_offset,
            global_work_size,
            local_work_size,
            num_events_in_wait_list,
            event_wait_list,
            event,
        );
    }
    let _ = (
        kernel,
        work_dim,
        global_work_offset,
        global_work_size,
        local_work_size,
        num_events_in_wait_list,
        event_wait_list,
        event,
    );
    cpu_fallback::enqueue_nd_range_kernel(command_queue)
}

pub unsafe extern "win64" fn clReleaseKernel(kernel: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.release_kernel)(kernel);
    }
    cpu_fallback::release_kernel(kernel)
}

pub unsafe extern "win64" fn clFlush(command_queue: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.flush)(command_queue);
    }
    cpu_fallback::flush_or_finish(command_queue)
}

pub unsafe extern "win64" fn clFinish(command_queue: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.finish)(command_queue);
    }
    cpu_fallback::flush_or_finish(command_queue)
}

pub unsafe extern "win64" fn clWaitForEvents(
    num_events: u32,
    event_list: *const *mut c_void,
) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return native::wait_for_events(cl, num_events, event_list);
    }
    cpu_fallback::wait_for_events(num_events, event_list)
}

pub unsafe extern "win64" fn clReleaseEvent(event: *mut c_void) -> i32 {
    #[cfg(target_os = "macos")]
    if let Some(cl) = native::backend() {
        return (cl.release_event)(event);
    }
    cpu_fallback::release_event(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::sync::atomic::{AtomicUsize, Ordering};

    unsafe extern "win64" fn build_notify_cb(program: *mut c_void, user_data: *mut c_void) {
        if !user_data.is_null() {
            let slot = &*(user_data as *const AtomicUsize);
            slot.store(program as usize, Ordering::SeqCst);
        }
    }

    fn c_bytes_to_string(buf: &[u8]) -> String {
        let nul = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..nul]).into_owned()
    }

    #[test]
    fn opencl_exports_are_all_registered() {
        let dll = crate::builtin_dlls()
            .iter()
            .find(|d| d.name == "opencl.dll")
            .expect("opencl.dll must be registered");
        let expected = [
            "clGetPlatformIDs",
            "clGetPlatformInfo",
            "clGetDeviceIDs",
            "clGetDeviceInfo",
            "clCreateContext",
            "clCreateContextFromType",
            "clRetainContext",
            "clReleaseContext",
            "clCreateCommandQueue",
            "clReleaseCommandQueue",
            "clCreateBuffer",
            "clReleaseMemObject",
            "clEnqueueReadBuffer",
            "clEnqueueWriteBuffer",
            "clEnqueueCopyBuffer",
            "clCreateProgramWithSource",
            "clBuildProgram",
            "clGetProgramBuildInfo",
            "clReleaseProgram",
            "clCreateKernel",
            "clSetKernelArg",
            "clEnqueueNDRangeKernel",
            "clReleaseKernel",
            "clFlush",
            "clFinish",
            "clWaitForEvents",
            "clReleaseEvent",
        ];
        for name in expected {
            assert!(
                dll.exports.iter().any(|e| e.name == name),
                "opencl.dll missing export {name}"
            );
        }
    }

    #[test]
    fn cpu_fallback_platform_buffers_events_and_honest_build_failure() {
        unsafe {
            // Platform query (also test unaligned output pointer).
            let mut raw_u32 = [0u8; 16];
            let unaligned_count = raw_u32.as_mut_ptr().add(1) as *mut u32;
            assert_eq!(
                cpu_fallback::get_platform_ids(0, std::ptr::null_mut(), unaligned_count),
                CL_SUCCESS
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_count), 1);

            let mut raw_plat = [0u8; 24];
            let unaligned_plat = raw_plat.as_mut_ptr().add(3) as *mut *mut c_void;
            assert_eq!(
                cpu_fallback::get_platform_ids(1, unaligned_plat, std::ptr::null_mut()),
                CL_SUCCESS
            );
            let platform = std::ptr::read_unaligned(unaligned_plat);
            assert!(!platform.is_null());

            let mut name_buf = [0u8; 64];
            let mut name_len: usize = 0;
            assert_eq!(
                cpu_fallback::get_platform_info(
                    platform,
                    CL_PLATFORM_NAME,
                    name_buf.len(),
                    name_buf.as_mut_ptr() as *mut c_void,
                    &mut name_len,
                ),
                CL_SUCCESS
            );
            assert_eq!(
                c_bytes_to_string(&name_buf[..name_len]),
                "PortFactory CPU OpenCL 1.2"
            );

            // Device query: CPU succeeds, GPU honestly reports CL_DEVICE_NOT_FOUND.
            let mut gpu_count: u32 = 99;
            assert_eq!(
                cpu_fallback::get_device_ids(
                    platform,
                    CL_DEVICE_TYPE_GPU,
                    0,
                    std::ptr::null_mut(),
                    &mut gpu_count,
                ),
                CL_DEVICE_NOT_FOUND
            );
            assert_eq!(gpu_count, 0);

            let mut device: *mut c_void = std::ptr::null_mut();
            let mut dev_count: u32 = 0;
            assert_eq!(
                cpu_fallback::get_device_ids(
                    platform,
                    CL_DEVICE_TYPE_CPU,
                    1,
                    &mut device,
                    &mut dev_count,
                ),
                CL_SUCCESS
            );
            assert_eq!(dev_count, 1);
            assert!(!device.is_null());

            let mut dev_type: u64 = 0;
            assert_eq!(
                cpu_fallback::get_device_info(
                    device,
                    CL_DEVICE_TYPE,
                    std::mem::size_of::<u64>(),
                    (&mut dev_type as *mut u64) as *mut c_void,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_ne!(dev_type & CL_DEVICE_TYPE_CPU, 0);

            let mut compiler_avail: u32 = 99;
            assert_eq!(
                cpu_fallback::get_device_info(
                    device,
                    CL_DEVICE_COMPILER_AVAILABLE,
                    std::mem::size_of::<u32>(),
                    (&mut compiler_avail as *mut u32) as *mut c_void,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_eq!(compiler_avail, CL_FALSE);

            // Context + retain/release + queue.
            let props = [CL_CONTEXT_PLATFORM, platform as isize, 0];
            let mut err: i32 = -1;
            let ctx = cpu_fallback::create_context(
                props.as_ptr(),
                1,
                &device,
                0,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert!(!ctx.is_null());
            assert_eq!(cpu_fallback::retain_context(ctx), CL_SUCCESS);
            assert_eq!(cpu_fallback::release_context(ctx), CL_SUCCESS);

            let ctx2 = cpu_fallback::create_context_from_type(
                props.as_ptr(),
                CL_DEVICE_TYPE_DEFAULT,
                0,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert!(!ctx2.is_null());
            assert_eq!(cpu_fallback::release_context(ctx2), CL_SUCCESS);

            let queue = cpu_fallback::create_command_queue(ctx, device, 0, &mut err);
            assert_eq!(err, CL_SUCCESS);
            assert!(!queue.is_null());

            // Real host-memory buffer write / copy / read / overlap / events.
            let initial = [10u32, 20, 30, 40];
            let buf_a = cpu_fallback::create_buffer(
                ctx,
                CL_MEM_READ_WRITE | CL_MEM_COPY_HOST_PTR,
                std::mem::size_of_val(&initial),
                initial.as_ptr() as *mut c_void,
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert!(!buf_a.is_null());

            let buf_b = cpu_fallback::create_buffer(
                ctx,
                CL_MEM_READ_WRITE,
                std::mem::size_of_val(&initial),
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);

            let mut ev_copy: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                cpu_fallback::enqueue_copy_buffer(
                    queue,
                    buf_a,
                    buf_b,
                    0,
                    0,
                    std::mem::size_of_val(&initial),
                    0,
                    std::ptr::null(),
                    &mut ev_copy,
                ),
                CL_SUCCESS
            );
            assert!(!ev_copy.is_null());
            assert_eq!(cpu_fallback::wait_for_events(1, &ev_copy), CL_SUCCESS);

            // Overlapping self-copy must fail with CL_MEM_COPY_OVERLAP.
            assert_eq!(
                cpu_fallback::enqueue_copy_buffer(
                    queue,
                    buf_b,
                    buf_b,
                    0,
                    4,
                    8,
                    0,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                ),
                CL_MEM_COPY_OVERLAP
            );

            // Overwrite middle elements in buf_b and read back waiting on ev_copy.
            let patch = [99u32, 100u32];
            assert_eq!(
                cpu_fallback::enqueue_write_buffer(
                    queue,
                    buf_b,
                    CL_TRUE,
                    4,
                    8,
                    patch.as_ptr() as *const c_void,
                    1,
                    &ev_copy,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            let mut out = [0u32; 4];
            assert_eq!(
                cpu_fallback::enqueue_read_buffer(
                    queue,
                    buf_b,
                    CL_TRUE,
                    0,
                    std::mem::size_of_val(&out),
                    out.as_mut_ptr() as *mut c_void,
                    0,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_eq!(out, [10, 99, 100, 40]);

            // CL_MEM_USE_HOST_PTR reads/writes directly against backing host memory.
            let mut host_backing = [1u32, 2, 3, 4];
            let buf_host = cpu_fallback::create_buffer(
                ctx,
                CL_MEM_READ_WRITE | CL_MEM_USE_HOST_PTR,
                std::mem::size_of_val(&host_backing),
                host_backing.as_mut_ptr() as *mut c_void,
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            let replacement = [7u32, 8];
            assert_eq!(
                cpu_fallback::enqueue_write_buffer(
                    queue,
                    buf_host,
                    CL_TRUE,
                    8,
                    8,
                    replacement.as_ptr() as *const c_void,
                    0,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_eq!(host_backing, [1, 2, 7, 8]);
            assert_eq!(cpu_fallback::release_mem_object(buf_host), CL_SUCCESS);

            // Program build must honestly fail with CL_BUILD_PROGRAM_FAILURE + log.
            let src = c"__kernel void noop(__global int* x) { x[0] = 1; }";
            let src_ptr = src.as_ptr();
            let prog = cpu_fallback::create_program_with_source(
                ctx,
                1,
                &src_ptr,
                std::ptr::null(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert!(!prog.is_null());

            let cb_seen = AtomicUsize::new(0);
            assert_eq!(
                cpu_fallback::build_program(
                    prog,
                    1,
                    &device,
                    c"-O2".as_ptr(),
                    build_notify_cb as *const () as usize,
                    (&cb_seen as *const AtomicUsize) as *mut c_void,
                ),
                CL_BUILD_PROGRAM_FAILURE
            );
            assert_eq!(cb_seen.load(Ordering::SeqCst), prog as usize);

            let mut status: i32 = 0;
            assert_eq!(
                cpu_fallback::get_program_build_info(
                    prog,
                    device,
                    CL_PROGRAM_BUILD_STATUS,
                    std::mem::size_of::<i32>(),
                    (&mut status as *mut i32) as *mut c_void,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_eq!(status, CL_BUILD_ERROR);

            let mut log_size: usize = 0;
            assert_eq!(
                cpu_fallback::get_program_build_info(
                    prog,
                    device,
                    CL_PROGRAM_BUILD_LOG,
                    0,
                    std::ptr::null_mut(),
                    &mut log_size,
                ),
                CL_SUCCESS
            );
            assert!(log_size > 1);
            let mut log_buf = vec![0u8; log_size];
            assert_eq!(
                cpu_fallback::get_program_build_info(
                    prog,
                    device,
                    CL_PROGRAM_BUILD_LOG,
                    log_buf.len(),
                    log_buf.as_mut_ptr() as *mut c_void,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            let log_str = CStr::from_bytes_with_nul(&log_buf).unwrap().to_str().unwrap();
            assert!(log_str.contains("PortFactory CPU OpenCL 1.2"));

            let k = cpu_fallback::create_kernel(prog, c"noop".as_ptr(), &mut err);
            assert!(k.is_null());
            assert_eq!(err, CL_INVALID_PROGRAM_EXECUTABLE);

            assert_eq!(cpu_fallback::flush_or_finish(queue), CL_SUCCESS);
            assert_eq!(cpu_fallback::release_event(ev_copy), CL_SUCCESS);
            assert_eq!(cpu_fallback::release_program(prog), CL_SUCCESS);
            assert_eq!(cpu_fallback::release_mem_object(buf_b), CL_SUCCESS);
            assert_eq!(cpu_fallback::release_mem_object(buf_a), CL_SUCCESS);
            assert_eq!(cpu_fallback::release_command_queue(queue), CL_SUCCESS);
            assert_eq!(cpu_fallback::release_context(ctx), CL_SUCCESS);
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_native_opencl_buffer_and_kernel_execution() {
        unsafe {
            assert!(
                native::backend().is_some(),
                "expected Apple OpenCL.framework to load on macOS"
            );

            // 1. Platform discovery & info (test unaligned pointer staging too).
            let mut unaligned_np_buf = [0u8; 16];
            let unaligned_np = unaligned_np_buf.as_mut_ptr().add(1) as *mut u32;
            assert_eq!(clGetPlatformIDs(0, std::ptr::null_mut(), unaligned_np), CL_SUCCESS);
            let num_platforms = std::ptr::read_unaligned(unaligned_np);
            assert!(num_platforms >= 1, "expected at least 1 OpenCL platform on macOS");

            let mut platform: *mut c_void = std::ptr::null_mut();
            assert_eq!(clGetPlatformIDs(1, &mut platform, std::ptr::null_mut()), CL_SUCCESS);
            assert!(!platform.is_null());

            let mut pname_len: usize = 0;
            assert_eq!(
                clGetPlatformInfo(
                    platform,
                    CL_PLATFORM_NAME,
                    0,
                    std::ptr::null_mut(),
                    &mut pname_len,
                ),
                CL_SUCCESS
            );
            assert!(pname_len > 1);
            let mut pname_raw = vec![0u8; pname_len + 8];
            // Write into an intentionally unaligned (+3) guest buffer.
            assert_eq!(
                clGetPlatformInfo(
                    platform,
                    CL_PLATFORM_NAME,
                    pname_len,
                    pname_raw.as_mut_ptr().add(3) as *mut c_void,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            let pname = c_bytes_to_string(&pname_raw[3..3 + pname_len]);
            assert!(!pname.is_empty());

            // 2. Device discovery & info.
            let mut device: *mut c_void = std::ptr::null_mut();
            let mut num_devices: u32 = 0;
            assert_eq!(
                clGetDeviceIDs(
                    platform,
                    CL_DEVICE_TYPE_ALL,
                    1,
                    &mut device,
                    &mut num_devices,
                ),
                CL_SUCCESS
            );
            assert!(num_devices >= 1);
            assert!(!device.is_null());

            let mut dname_buf = [0u8; 256];
            let mut dname_len: usize = 0;
            assert_eq!(
                clGetDeviceInfo(
                    device,
                    CL_DEVICE_NAME,
                    dname_buf.len(),
                    dname_buf.as_mut_ptr() as *mut c_void,
                    &mut dname_len,
                ),
                CL_SUCCESS
            );
            assert!(dname_len > 1);

            // 3. Context (both clCreateContext and clCreateContextFromType) & queue.
            let mut err: i32 = -1;
            let ctx_from_type = clCreateContextFromType(
                std::ptr::null(),
                CL_DEVICE_TYPE_DEFAULT,
                0,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert!(!ctx_from_type.is_null());
            assert_eq!(clRetainContext(ctx_from_type), CL_SUCCESS);
            assert_eq!(clReleaseContext(ctx_from_type), CL_SUCCESS);
            assert_eq!(clReleaseContext(ctx_from_type), CL_SUCCESS);

            let props = [CL_CONTEXT_PLATFORM, platform as isize, 0];
            let ctx = clCreateContext(
                props.as_ptr(),
                1,
                &device,
                0,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert!(!ctx.is_null());

            let queue = clCreateCommandQueue(ctx, device, 0, &mut err);
            assert_eq!(err, CL_SUCCESS);
            assert!(!queue.is_null());

            // 4. Buffers: write, copy, kernel execute, read back.
            const N: usize = 64;
            let host_a: Vec<f32> = (0..N).map(|i| (i as f32) * 1.5 + 1.0).collect();
            let host_b: Vec<f32> = (0..N).map(|i| (i as f32) * 2.25 - 0.5).collect();
            let byte_len = N * std::mem::size_of::<f32>();

            let buf_a = clCreateBuffer(
                ctx,
                CL_MEM_READ_WRITE,
                byte_len,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            let buf_a_copy = clCreateBuffer(
                ctx,
                CL_MEM_READ_ONLY,
                byte_len,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            let buf_b = clCreateBuffer(
                ctx,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                byte_len,
                host_b.as_ptr() as *mut c_void,
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            let buf_out = clCreateBuffer(
                ctx,
                CL_MEM_WRITE_ONLY,
                byte_len,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);

            let mut ev_write: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                clEnqueueWriteBuffer(
                    queue,
                    buf_a,
                    CL_FALSE,
                    0,
                    byte_len,
                    host_a.as_ptr() as *const c_void,
                    0,
                    std::ptr::null(),
                    &mut ev_write,
                ),
                CL_SUCCESS
            );
            assert!(!ev_write.is_null());

            let mut ev_copy: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                clEnqueueCopyBuffer(
                    queue,
                    buf_a,
                    buf_a_copy,
                    0,
                    0,
                    byte_len,
                    1,
                    &ev_write,
                    &mut ev_copy,
                ),
                CL_SUCCESS
            );
            assert!(!ev_copy.is_null());

            // 5. Compile & execute an OpenCL C kernel on Apple's OpenCL runtime.
            let kernel_src = c"
                __kernel void vec_fma(
                    __global const float* a,
                    __global const float* b,
                    __global float* out,
                    float scale
                ) {
                    int i = get_global_id(0);
                    out[i] = a[i] * scale + b[i];
                }
            ";
            let src_ptr = kernel_src.as_ptr();
            let prog = clCreateProgramWithSource(
                ctx,
                1,
                &src_ptr,
                std::ptr::null(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert!(!prog.is_null());

            let build_cb_seen = AtomicUsize::new(0);
            let build_rc = clBuildProgram(
                prog,
                1,
                &device,
                std::ptr::null(),
                build_notify_cb as *const () as usize,
                (&build_cb_seen as *const AtomicUsize) as *mut c_void,
            );
            assert_eq!(build_rc, CL_SUCCESS);
            assert_eq!(build_cb_seen.load(Ordering::SeqCst), prog as usize);

            let mut build_status: i32 = -99;
            assert_eq!(
                clGetProgramBuildInfo(
                    prog,
                    device,
                    CL_PROGRAM_BUILD_STATUS,
                    std::mem::size_of::<i32>(),
                    (&mut build_status as *mut i32) as *mut c_void,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_eq!(build_status, CL_BUILD_SUCCESS);

            let kernel = clCreateKernel(prog, c"vec_fma".as_ptr(), &mut err);
            assert_eq!(err, CL_SUCCESS);
            assert!(!kernel.is_null());

            // Pass arg 0 from an intentionally unaligned guest buffer to exercise staging.
            let mut unaligned_arg = [0u8; 24];
            let unaligned_mem_ptr = unaligned_arg.as_mut_ptr().add(1) as *mut *mut c_void;
            std::ptr::write_unaligned(unaligned_mem_ptr, buf_a_copy);
            assert_eq!(
                clSetKernelArg(
                    kernel,
                    0,
                    std::mem::size_of::<*mut c_void>(),
                    unaligned_mem_ptr as *const c_void,
                ),
                CL_SUCCESS
            );
            assert_eq!(
                clSetKernelArg(
                    kernel,
                    1,
                    std::mem::size_of::<*mut c_void>(),
                    (&buf_b as *const *mut c_void) as *const c_void,
                ),
                CL_SUCCESS
            );
            assert_eq!(
                clSetKernelArg(
                    kernel,
                    2,
                    std::mem::size_of::<*mut c_void>(),
                    (&buf_out as *const *mut c_void) as *const c_void,
                ),
                CL_SUCCESS
            );
            let scale: f32 = 3.0;
            assert_eq!(
                clSetKernelArg(
                    kernel,
                    3,
                    std::mem::size_of::<f32>(),
                    (&scale as *const f32) as *const c_void,
                ),
                CL_SUCCESS
            );

            let global_size: [usize; 1] = [N];
            let mut ev_kernel: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                clEnqueueNDRangeKernel(
                    queue,
                    kernel,
                    1,
                    std::ptr::null(),
                    global_size.as_ptr(),
                    std::ptr::null(),
                    1,
                    &ev_copy,
                    &mut ev_kernel,
                ),
                CL_SUCCESS
            );
            assert!(!ev_kernel.is_null());
            assert_eq!(clFlush(queue), CL_SUCCESS);
            assert_eq!(clWaitForEvents(1, &ev_kernel), CL_SUCCESS);

            let mut host_out = vec![0.0f32; N];
            assert_eq!(
                clEnqueueReadBuffer(
                    queue,
                    buf_out,
                    CL_TRUE,
                    0,
                    byte_len,
                    host_out.as_mut_ptr() as *mut c_void,
                    0,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_eq!(clFinish(queue), CL_SUCCESS);

            for i in 0..N {
                let expected = host_a[i] * scale + host_b[i];
                assert!(
                    (host_out[i] - expected).abs() < 1e-4,
                    "mismatch at index {i}: got {}, expected {expected}",
                    host_out[i]
                );
            }

            // 6. Verify invalid kernel source reports CL_BUILD_PROGRAM_FAILURE + non-empty log.
            let bad_src = c"__kernel void broken( { this is not valid opencl c }";
            let bad_ptr = bad_src.as_ptr();
            let bad_prog = clCreateProgramWithSource(
                ctx,
                1,
                &bad_ptr,
                std::ptr::null(),
                &mut err,
            );
            assert_eq!(err, CL_SUCCESS);
            assert_eq!(
                clBuildProgram(
                    bad_prog,
                    1,
                    &device,
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                ),
                CL_BUILD_PROGRAM_FAILURE
            );
            let mut bad_status: i32 = 0;
            assert_eq!(
                clGetProgramBuildInfo(
                    bad_prog,
                    device,
                    CL_PROGRAM_BUILD_STATUS,
                    std::mem::size_of::<i32>(),
                    (&mut bad_status as *mut i32) as *mut c_void,
                    std::ptr::null_mut(),
                ),
                CL_SUCCESS
            );
            assert_eq!(bad_status, CL_BUILD_ERROR);
            let mut bad_log_len: usize = 0;
            assert_eq!(
                clGetProgramBuildInfo(
                    bad_prog,
                    device,
                    CL_PROGRAM_BUILD_LOG,
                    0,
                    std::ptr::null_mut(),
                    &mut bad_log_len,
                ),
                CL_SUCCESS
            );
            assert!(bad_log_len > 1);

            // Cleanup all OpenCL objects.
            assert_eq!(clReleaseProgram(bad_prog), CL_SUCCESS);
            assert_eq!(clReleaseEvent(ev_kernel), CL_SUCCESS);
            assert_eq!(clReleaseEvent(ev_copy), CL_SUCCESS);
            assert_eq!(clReleaseEvent(ev_write), CL_SUCCESS);
            assert_eq!(clReleaseKernel(kernel), CL_SUCCESS);
            assert_eq!(clReleaseProgram(prog), CL_SUCCESS);
            assert_eq!(clReleaseMemObject(buf_out), CL_SUCCESS);
            assert_eq!(clReleaseMemObject(buf_b), CL_SUCCESS);
            assert_eq!(clReleaseMemObject(buf_a_copy), CL_SUCCESS);
            assert_eq!(clReleaseMemObject(buf_a), CL_SUCCESS);
            assert_eq!(clReleaseCommandQueue(queue), CL_SUCCESS);
            assert_eq!(clReleaseContext(ctx), CL_SUCCESS);
        }
    }
}
