//! `vulkan-1.dll` (`vulkan_1.rs`): Universal Vulkan Loader -> MoltenVK + `CAMetalLayer` bridge.
//!
//! Dynamically loads `libMoltenVK.dylib` (with `libvulkan.1.dylib` / `libvulkan.dylib` fallback)
//! and bridges Windows Vulkan 1.0-1.3 / KHR applications directly onto Apple Metal:
//! - Translates `VK_KHR_win32_surface` <-> `VK_EXT_metal_surface` (with `VK_MVK_macos_surface`
//!   fallback) and injects `VK_KHR_portability_enumeration` +
//!   `VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR` when advertised by the host loader/ICD.
//! - Resolves `HWND` -> `CAMetalLayer*` via [`crate::macos::metal_layer_for_hwnd`] in
//!   [`vkCreateWin32SurfaceKHR`].
//! - On `aarch64-apple-darwin`, `win64_abi::win64_mods!` lowers `extern "win64"` to AAPCS64
//!   `extern "C"`, matching MoltenVK's native ABI for integer/pointer/handle arguments. Any
//!   non-intercepted entrypoint resolved through [`vkGetInstanceProcAddr`] or
//!   [`vkGetDeviceProcAddr`] is registered in [`crate::register_native_va`] so `x86-runtime` can
//!   invoke it directly.
//! - Vulkan commands with by-value `f32` arguments after a `VkCommandBuffer` (`vkCmdSetLineWidth`,
//!   `vkCmdSetDepthBias`, `vkCmdSetDepthBounds`, `vkCmdSetExtraPrimitiveOverestimationSizeEXT`)
//!   are bridged from Win64 positional `xmm1..xmm3` to ARM64 AAPCS64 `s0..s2` in both static
//!   exports and dynamic `vkGetInstanceProcAddr` / `vkGetDeviceProcAddr` lookups.
//! - Surface capability queries (`vkGetPhysicalDeviceSurfaceCapabilitiesKHR` /
//!   `vkGetPhysicalDeviceSurfaceCapabilities2KHR`) and swapchain creation (`vkCreateSwapchainKHR`)
//!   synchronize pending AppKit window-resize operations on the main queue (and update offscreen
//!   headless `CAMetalLayer` bounds when `PORT_HEADLESS=1` / `AW2_HEADLESS=1`) before querying
//!   MoltenVK so resize + swapchain recreation always observes the updated window extent.
//! - Device/instance create and destroy procs resolved through `vkGetDeviceProcAddr` and
//!   `vkGetInstanceProcAddr` sanitize guest `VkAllocationCallbacks` (whose x86-64 function
//!   pointers cannot be invoked directly by native ARM64 MoltenVK).
//! - Vulkan 1.2 / 1.3 & KHR extensions (`VK_KHR_dynamic_rendering`, `VK_KHR_synchronization2`,
//!   `VK_KHR_buffer_device_address`, `VK_KHR_copy_commands2`, `VK_KHR_maintenance4`,
//!   `VK_KHR_timeline_semaphore`) use fixed-width C ABI structs (`u32`/`i32`/`u64`/`f32`/ptrs,
//!   identical on Win64 LLP64 and macOS ARM64 LP64) and pass 1..4 integer/handle/pointer args
//!   (no float args, no `VkAllocationCallbacks`), so no struct rewriting is needed beyond
//!   injecting `VK_KHR_portability_subset` in [`vkCreateDevice`] when advertised by MoltenVK.
//!   Both static exports and `vkGetInstanceProcAddr`/`vkGetDeviceProcAddr` resolve core <-> KHR/EXT
//!   promoted aliases seamlessly.
//! - When no MoltenVK / Vulkan runtime is installed on the host, every entrypoint reports an
//!   honest Vulkan error (`VK_ERROR_INCOMPATIBLE_DRIVER` / `VK_ERROR_INITIALIZATION_FAILED` /
//!   `VK_FALSE`) and zeroes output handles rather than synthesizing fake handles.
//! - Offline SPIR-V -> Metal precompilation lives in `dxil_metal::spirv_metal`
//!   (`~/Translator/ShaderCache`); live Vulkan devices forward [`vkCreateShaderModule`] straight
//!   to MoltenVK.

use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, c_void, CStr, CString};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Vulkan ABI constants
// ---------------------------------------------------------------------------

pub const VK_SUCCESS: i32 = 0;
pub const VK_INCOMPLETE: i32 = 5;
pub const VK_ERROR_INITIALIZATION_FAILED: i32 = -3;
pub const VK_ERROR_EXTENSION_NOT_PRESENT: i32 = -7;
pub const VK_ERROR_INCOMPATIBLE_DRIVER: i32 = -9;

pub const VK_FALSE: u32 = 0;
pub const VK_TRUE: u32 = 1;

const VK_API_VERSION_1_0: u32 = 1 << 22;
#[allow(dead_code)]
const VK_API_VERSION_1_2: u32 = (1 << 22) | (2 << 12);
#[allow(dead_code)]
const VK_API_VERSION_1_3: u32 = (1 << 22) | (3 << 12);
const VK_QUEUE_GRAPHICS_BIT: u32 = 0x0000_0001;

pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;

pub const VK_SUBOPTIMAL_KHR: i32 = 1_000_001_003;

pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: u32 = 1;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: u32 = 2;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: u32 = 3;
const VK_STRUCTURE_TYPE_SUBMIT_INFO: u32 = 4;
const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: u32 = 5;
const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: u32 = 8;
const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: u32 = 12;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: u32 = 16;
const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: u32 = 39;
const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: u32 = 40;
const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: u32 = 42;
const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: u32 = 45;
pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: u32 = 1_000_009_000;
pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: u32 = 1_000_123_000;
pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT: u32 = 1_000_217_000;

const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: u32 = 0x0000_0001;
const VK_IMAGE_ASPECT_COLOR_BIT: u32 = 0x0000_0001;
const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: u32 = 6;
const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: u32 = 1_000_001_002;
const VK_BUFFER_USAGE_TRANSFER_DST_BIT: u32 = 0x0000_0002;
const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: u32 = 0x0000_0002;
const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: u32 = 0x0000_0004;
const VK_PIPELINE_STAGE_TRANSFER_BIT: u32 = 0x0000_1000;
const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: u32 = 0x0000_2000;
const VK_ACCESS_TRANSFER_READ_BIT: u32 = 0x0000_0800;
const VK_QUEUE_FAMILY_IGNORED: u32 = !0u32;
const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: u32 = 0x0000_0001;

const VK_FORMAT_R8G8B8A8_UNORM: u32 = 37;
const VK_FORMAT_R8G8B8A8_SRGB: u32 = 43;
const VK_FORMAT_B8G8R8A8_UNORM: u32 = 44;
const VK_FORMAT_B8G8R8A8_SRGB: u32 = 50;
const VK_FORMAT_A2B10G10R10_UNORM_PACK32: u32 = 64;
const VK_FORMAT_R16G16B16A16_SFLOAT: u32 = 97;

pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: u32 = 0x0000_0001;

pub const EXT_KHR_SURFACE: &str = "VK_KHR_surface";
pub const EXT_KHR_WIN32_SURFACE: &str = "VK_KHR_win32_surface";
pub const EXT_EXT_METAL_SURFACE: &str = "VK_EXT_metal_surface";
pub const EXT_MVK_MACOS_SURFACE: &str = "VK_MVK_macos_surface";
pub const EXT_KHR_PORTABILITY_ENUMERATION: &str = "VK_KHR_portability_enumeration";
pub const EXT_KHR_PORTABILITY_SUBSET: &str = "VK_KHR_portability_subset";

// NVIDIA Low Latency Vulkan status (`nvlowlatencyvk.dll` routed here).
const NVLL_VK_NO_IMPLEMENTATION: i32 = -3;

// ---------------------------------------------------------------------------
// Vulkan C ABI structs (identical on Win64 LLP64 and macOS ARM64 LP64)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkApplicationInfo {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}
const _: () = assert!(std::mem::size_of::<VkApplicationInfo>() == 48);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkInstanceCreateInfo {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
}
const _: () = assert!(std::mem::size_of::<VkInstanceCreateInfo>() == 64);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExtensionProperties {
    pub extension_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
}
const _: () = assert!(std::mem::size_of::<VkExtensionProperties>() == 260);

impl VkExtensionProperties {
    pub const fn zeroed() -> Self {
        Self {
            extension_name: [0; VK_MAX_EXTENSION_NAME_SIZE],
            spec_version: 0,
        }
    }

    pub fn new(name: &str, spec_version: u32) -> Self {
        let mut prop = Self::zeroed();
        let bytes = name.as_bytes();
        let len = bytes.len().min(VK_MAX_EXTENSION_NAME_SIZE - 1);
        for (dst, &src) in prop.extension_name.iter_mut().zip(bytes.iter().take(len)) {
            *dst = src as c_char;
        }
        prop.spec_version = spec_version;
        prop
    }

    pub fn name_str(&self) -> &str {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                self.extension_name.as_ptr() as *const u8,
                VK_MAX_EXTENSION_NAME_SIZE,
            )
        };
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        std::str::from_utf8(&bytes[..end]).unwrap_or("")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub hinstance: usize,
    pub hwnd: usize,
}
const _: () = assert!(std::mem::size_of::<VkWin32SurfaceCreateInfoKHR>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkMetalSurfaceCreateInfoEXT {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub p_layer: *const c_void,
}
const _: () = assert!(std::mem::size_of::<VkMetalSurfaceCreateInfoEXT>() == 32);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub p_view: *const c_void,
}
const _: () = assert!(std::mem::size_of::<VkMacOSSurfaceCreateInfoMVK>() == 32);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkDeviceQueueCreateInfo {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}
const _: () = assert!(std::mem::size_of::<VkDeviceQueueCreateInfo>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkDeviceCreateInfo {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const c_void,
}
const _: () = assert!(std::mem::size_of::<VkDeviceCreateInfo>() == 72);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VkShaderModuleCreateInfo {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub code_size: usize,
    pub p_code: *const u32,
}
const _: () = assert!(std::mem::size_of::<VkShaderModuleCreateInfo>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkQueueFamilyProperties {
    pub queue_flags: u32,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: VkExtent3D,
}
const _: () = assert!(std::mem::size_of::<VkQueueFamilyProperties>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkMemoryRequirements {
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
}
const _: () = assert!(std::mem::size_of::<VkMemoryRequirements>() == 24);

// ---------------------------------------------------------------------------
// Native C function pointer types for MoltenVK / Vulkan loader
// ---------------------------------------------------------------------------

type PfnVkGetInstanceProcAddr = unsafe extern "C" fn(usize, *const c_char) -> *const c_void;
type PfnVkGetDeviceProcAddr = unsafe extern "C" fn(usize, *const c_char) -> *const c_void;
type PfnVkEnumerateInstanceVersion = unsafe extern "C" fn(*mut u32) -> i32;
type PfnVkEnumerateInstanceExtensionProperties =
    unsafe extern "C" fn(*const c_char, *mut u32, *mut VkExtensionProperties) -> i32;
type PfnVkEnumerateInstanceLayerProperties = unsafe extern "C" fn(*mut u32, *mut c_void) -> i32;
type PfnVkCreateInstance =
    unsafe extern "C" fn(*const VkInstanceCreateInfo, *const c_void, *mut usize) -> i32;
type PfnVkDestroyInstance = unsafe extern "C" fn(usize, *const c_void);
type PfnVkEnumeratePhysicalDevices = unsafe extern "C" fn(usize, *mut u32, *mut usize) -> i32;
type PfnVkCreateMetalSurfaceEXT =
    unsafe extern "C" fn(usize, *const VkMetalSurfaceCreateInfoEXT, *const c_void, *mut u64) -> i32;
type PfnVkCreateMacOSSurfaceMVK =
    unsafe extern "C" fn(usize, *const VkMacOSSurfaceCreateInfoMVK, *const c_void, *mut u64) -> i32;
type PfnVkDestroySurfaceKHR = unsafe extern "C" fn(usize, u64, *const c_void);
type PfnVkCreateShaderModule =
    unsafe extern "C" fn(usize, *const VkShaderModuleCreateInfo, *const c_void, *mut u64) -> i32;
type PfnVkDestroyShaderModule = unsafe extern "C" fn(usize, u64, *const c_void);
type PfnVkGetPhysicalDeviceQueueFamilyProperties =
    unsafe extern "C" fn(usize, *mut u32, *mut VkQueueFamilyProperties);

// ---------------------------------------------------------------------------
// Dynamic MoltenVK / Vulkan loader
// ---------------------------------------------------------------------------

struct VulkanBackend {
    handle: usize,
    get_instance_proc_addr: PfnVkGetInstanceProcAddr,
    get_device_proc_addr: Option<PfnVkGetDeviceProcAddr>,
    supports_metal_surface: bool,
    supports_macos_surface: bool,
    supports_portability_enumeration: bool,
}

unsafe impl Send for VulkanBackend {}
unsafe impl Sync for VulkanBackend {}

#[derive(Clone, Copy)]
struct InstanceProcs {
    create_metal_surface_ext: Option<PfnVkCreateMetalSurfaceEXT>,
    create_macos_surface_mvk: Option<PfnVkCreateMacOSSurfaceMVK>,
    destroy_surface_khr: Option<PfnVkDestroySurfaceKHR>,
}

static FALLBACK_GDPA: AtomicUsize = AtomicUsize::new(0);

fn live_instances() -> &'static Mutex<HashMap<usize, InstanceProcs>> {
    static MAP: OnceLock<Mutex<HashMap<usize, InstanceProcs>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VkSwapchainCreateInfoKHR {
    s_type: u32,
    p_next: *const c_void,
    flags: u32,
    surface: u64,
    min_image_count: u32,
    image_format: u32,
    image_color_space: u32,
    image_extent_width: u32,
    image_extent_height: u32,
    image_array_layers: u32,
    image_usage: u32,
    image_sharing_mode: u32,
    queue_family_index_count: u32,
    p_queue_family_indices: *const u32,
    pre_transform: u32,
    composite_alpha: u32,
    present_mode: u32,
    clipped: u32,
    old_swapchain: u64,
}
const _: () = assert!(std::mem::size_of::<VkSwapchainCreateInfoKHR>() == 104);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkPresentInfoKHR {
    s_type: u32,
    p_next: *const c_void,
    wait_semaphore_count: u32,
    p_wait_semaphores: *const u64,
    swapchain_count: u32,
    p_swapchains: *const u64,
    p_image_indices: *const u32,
    p_results: *mut i32,
}
const _: () = assert!(std::mem::size_of::<VkPresentInfoKHR>() == 64);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkBufferCreateInfo {
    s_type: u32,
    p_next: *const c_void,
    flags: u32,
    size: u64,
    usage: u32,
    sharing_mode: u32,
    queue_family_index_count: u32,
    p_queue_family_indices: *const u32,
}
const _: () = assert!(std::mem::size_of::<VkBufferCreateInfo>() == 56);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkMemoryAllocateInfo {
    s_type: u32,
    p_next: *const c_void,
    allocation_size: u64,
    memory_type_index: u32,
}
const _: () = assert!(std::mem::size_of::<VkMemoryAllocateInfo>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkCommandPoolCreateInfo {
    s_type: u32,
    p_next: *const c_void,
    flags: u32,
    queue_family_index: u32,
}
const _: () = assert!(std::mem::size_of::<VkCommandPoolCreateInfo>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkCommandBufferAllocateInfo {
    s_type: u32,
    p_next: *const c_void,
    command_pool: u64,
    level: u32,
    command_buffer_count: u32,
}
const _: () = assert!(std::mem::size_of::<VkCommandBufferAllocateInfo>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkCommandBufferBeginInfo {
    s_type: u32,
    p_next: *const c_void,
    flags: u32,
    p_inheritance_info: *const c_void,
}
const _: () = assert!(std::mem::size_of::<VkCommandBufferBeginInfo>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkFenceCreateInfo {
    s_type: u32,
    p_next: *const c_void,
    flags: u32,
}
const _: () = assert!(std::mem::size_of::<VkFenceCreateInfo>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkImageMemoryBarrier {
    s_type: u32,
    p_next: *const c_void,
    src_access_mask: u32,
    dst_access_mask: u32,
    old_layout: u32,
    new_layout: u32,
    src_queue_family_index: u32,
    dst_queue_family_index: u32,
    image: u64,
    aspect_mask: u32,
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
}
const _: () = assert!(std::mem::size_of::<VkImageMemoryBarrier>() == 72);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkBufferImageCopy {
    buffer_offset: u64,
    buffer_row_length: u32,
    buffer_image_height: u32,
    aspect_mask: u32,
    mip_level: u32,
    base_array_layer: u32,
    layer_count: u32,
    offset_x: i32,
    offset_y: i32,
    offset_z: i32,
    extent_width: u32,
    extent_height: u32,
    extent_depth: u32,
}
const _: () = assert!(std::mem::size_of::<VkBufferImageCopy>() == 56);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkSubmitInfo {
    s_type: u32,
    p_next: *const c_void,
    wait_semaphore_count: u32,
    p_wait_semaphores: *const u64,
    p_wait_dst_stage_mask: *const u32,
    command_buffer_count: u32,
    p_command_buffers: *const usize,
    signal_semaphore_count: u32,
    p_signal_semaphores: *const u64,
}
const _: () = assert!(std::mem::size_of::<VkSubmitInfo>() == 72);

#[repr(C)]
#[derive(Clone, Copy)]
struct VkMemoryType {
    property_flags: u32,
    heap_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VkMemoryHeap {
    size: u64,
    flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VkPhysicalDeviceMemoryProperties {
    memory_type_count: u32,
    memory_types: [VkMemoryType; 32],
    memory_heap_count: u32,
    memory_heaps: [VkMemoryHeap; 16],
}
const _: () = assert!(std::mem::size_of::<VkPhysicalDeviceMemoryProperties>() == 520);

fn live_devices() -> &'static Mutex<HashMap<usize, usize>> {
    static MAP: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone, Copy)]
struct QueueRecord {
    device: usize,
    family_index: u32,
}

fn live_queues() -> &'static Mutex<HashMap<usize, QueueRecord>> {
    static MAP: OnceLock<Mutex<HashMap<usize, QueueRecord>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone)]
struct SwapchainRecord {
    device: usize,
    hwnd: usize,
    width: u32,
    height: u32,
    format: u32,
    usage: u32,
    images: Vec<u64>,
}

fn live_swapchains() -> &'static Mutex<HashMap<u64, SwapchainRecord>> {
    static MAP: OnceLock<Mutex<HashMap<u64, SwapchainRecord>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

struct SurfaceRecord {
    instance: usize,
    hwnd: usize,
    layer: usize,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    headless_layer: Option<objc2::rc::Retained<objc2_quartz_core::CAMetalLayer>>,
}

// SAFETY: Windowed `CAMetalLayer` pointers are retained by their AppKit `AW2View` (accessed on
// the main thread via `crate::macos::run_sync`), and headless layers are owned by `headless_layer`
// under the `live_surfaces` mutex until after `vkDestroySurfaceKHR` / `vkDestroyInstance`.
unsafe impl Send for SurfaceRecord {}
unsafe impl Sync for SurfaceRecord {}

fn live_surfaces() -> &'static Mutex<HashMap<u64, SurfaceRecord>> {
    static MAP: OnceLock<Mutex<HashMap<u64, SurfaceRecord>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

fn is_headless_env() -> bool {
    std::env::var("PORT_HEADLESS")
        .or_else(|_| std::env::var("AW2_HEADLESS"))
        .is_ok_and(|v| {
            let t = v.trim();
            !t.is_empty() && t != "0"
        })
}

fn push_unique_candidate(out: &mut Vec<String>, seen: &mut HashSet<String>, cand: String) {
    if !cand.is_empty() && seen.insert(cand.clone()) {
        out.push(cand);
    }
}

/// Builds the ordered list of MoltenVK / macOS Vulkan loader dylib candidates:
/// 1. `$PORT_VULKAN_DYLIB` / `$PORT_MOLTENVK_DYLIB` / `$MOLTENVK_DYLIB`
/// 2. `<current_exe>/../Frameworks/libMoltenVK.dylib` and
///    `@executable_path/../Frameworks/libMoltenVK.dylib` (self-contained `.app` bundles)
/// 3. `/opt/homebrew/lib/libMoltenVK.dylib`
/// 4. `/usr/local/lib/libMoltenVK.dylib`
/// 5. `$VULKAN_SDK/lib/libMoltenVK.dylib`
/// 6. `libMoltenVK.dylib` (standard dyld search)
/// 7. Fallback macOS Vulkan loader (`libvulkan.1.dylib` / `libvulkan.dylib`)
pub(crate) fn build_loader_candidates(
    env_port_vulkan: Option<&str>,
    env_moltenvk: Option<&str>,
    env_port_moltenvk: Option<&str>,
    exe_dir: Option<&Path>,
    vulkan_sdk: Option<&str>,
) -> Vec<String> {
    let mut out = Vec::with_capacity(18);
    let mut seen = HashSet::new();

    if let Some(s) = env_port_vulkan.map(str::trim).filter(|s| !s.is_empty()) {
        push_unique_candidate(&mut out, &mut seen, s.to_string());
    }
    if let Some(s) = env_port_moltenvk.map(str::trim).filter(|s| !s.is_empty()) {
        push_unique_candidate(&mut out, &mut seen, s.to_string());
    }
    if let Some(s) = env_moltenvk.map(str::trim).filter(|s| !s.is_empty()) {
        push_unique_candidate(&mut out, &mut seen, s.to_string());
    }
    #[cfg(target_os = "ios")]
    {
        if let Some(dir) = exe_dir {
            push_unique_candidate(
                &mut out,
                &mut seen,
                dir.join("Frameworks/libMoltenVK.dylib")
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        push_unique_candidate(
            &mut out,
            &mut seen,
            "@executable_path/Frameworks/libMoltenVK.dylib".to_string(),
        );
    }
    if let Some(dir) = exe_dir {
        push_unique_candidate(
            &mut out,
            &mut seen,
            dir.join("../Frameworks/libMoltenVK.dylib")
                .to_string_lossy()
                .into_owned(),
        );
    }
    push_unique_candidate(
        &mut out,
        &mut seen,
        "@executable_path/../Frameworks/libMoltenVK.dylib".to_string(),
    );
    push_unique_candidate(
        &mut out,
        &mut seen,
        "/opt/homebrew/lib/libMoltenVK.dylib".to_string(),
    );
    push_unique_candidate(
        &mut out,
        &mut seen,
        "/usr/local/lib/libMoltenVK.dylib".to_string(),
    );
    if let Some(sdk) = vulkan_sdk.map(str::trim).filter(|s| !s.is_empty()) {
        push_unique_candidate(
            &mut out,
            &mut seen,
            Path::new(sdk)
                .join("lib/libMoltenVK.dylib")
                .to_string_lossy()
                .into_owned(),
        );
    }
    push_unique_candidate(&mut out, &mut seen, "libMoltenVK.dylib".to_string());

    // Fallback macOS Vulkan loader if libMoltenVK.dylib was not found directly.
    if let Some(dir) = exe_dir {
        push_unique_candidate(
            &mut out,
            &mut seen,
            dir.join("../Frameworks/libvulkan.1.dylib")
                .to_string_lossy()
                .into_owned(),
        );
    }
    push_unique_candidate(
        &mut out,
        &mut seen,
        "@executable_path/../Frameworks/libvulkan.1.dylib".to_string(),
    );
    push_unique_candidate(
        &mut out,
        &mut seen,
        "/opt/homebrew/lib/libvulkan.1.dylib".to_string(),
    );
    push_unique_candidate(
        &mut out,
        &mut seen,
        "/usr/local/lib/libvulkan.1.dylib".to_string(),
    );
    if let Some(sdk) = vulkan_sdk.map(str::trim).filter(|s| !s.is_empty()) {
        push_unique_candidate(
            &mut out,
            &mut seen,
            Path::new(sdk)
                .join("lib/libvulkan.1.dylib")
                .to_string_lossy()
                .into_owned(),
        );
    }
    push_unique_candidate(&mut out, &mut seen, "libvulkan.1.dylib".to_string());
    push_unique_candidate(&mut out, &mut seen, "libvulkan.dylib".to_string());

    out
}

impl VulkanBackend {
    unsafe fn load() -> Option<Self> {
        let env_port_vk = std::env::var("PORT_VULKAN_DYLIB").ok();
        let env_mvk = std::env::var("MOLTENVK_DYLIB").ok();
        let env_port_mvk = std::env::var("PORT_MOLTENVK_DYLIB").ok();
        let exe_path = std::env::current_exe().ok();
        let exe_dir = exe_path.as_deref().and_then(Path::parent);
        let vulkan_sdk = std::env::var("VULKAN_SDK").ok();

        let candidates = build_loader_candidates(
            env_port_vk.as_deref(),
            env_mvk.as_deref(),
            env_port_mvk.as_deref(),
            exe_dir,
            vulkan_sdk.as_deref(),
        );

        for cand in candidates {
            let Ok(c_path) = CString::new(cand.as_str()) else {
                continue;
            };
            let h = libc::dlopen(c_path.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL);
            if h.is_null() {
                continue;
            }
            let gipa_ptr = libc::dlsym(h, c"vkGetInstanceProcAddr".as_ptr());
            if gipa_ptr.is_null() {
                log::warn!(
                    target: "vulkan",
                    "Loaded {cand} but it does not export vkGetInstanceProcAddr; closing"
                );
                libc::dlclose(h);
                continue;
            }
            let get_instance_proc_addr: PfnVkGetInstanceProcAddr = std::mem::transmute(gipa_ptr);
            let gdpa_ptr = libc::dlsym(h, c"vkGetDeviceProcAddr".as_ptr());
            let get_device_proc_addr = if !gdpa_ptr.is_null() {
                Some(std::mem::transmute::<*mut c_void, PfnVkGetDeviceProcAddr>(gdpa_ptr))
            } else {
                None
            };

            let mut backend = Self {
                handle: h as usize,
                get_instance_proc_addr,
                get_device_proc_addr,
                supports_metal_surface: false,
                supports_macos_surface: false,
                supports_portability_enumeration: false,
            };
            backend.probe_instance_extensions();

            log::info!(
                target: "vulkan",
                "Loaded Vulkan/MoltenVK backend from {cand} (metal_surface={}, macos_surface={}, portability_enum={})",
                backend.supports_metal_surface,
                backend.supports_macos_surface,
                backend.supports_portability_enumeration
            );
            return Some(backend);
        }

        log::info!(
            target: "vulkan",
            "No MoltenVK/Vulkan dylib found; vulkan-1.dll will report VK_ERROR_INCOMPATIBLE_DRIVER"
        );
        None
    }

    unsafe fn probe_instance_extensions(&mut self) {
        let Some(ptr) = self.global_proc_ptr(c"vkEnumerateInstanceExtensionProperties") else {
            return;
        };
        let enumerate: PfnVkEnumerateInstanceExtensionProperties = std::mem::transmute(ptr);
        let mut count: u32 = 0;
        if enumerate(std::ptr::null(), &mut count, std::ptr::null_mut()) != VK_SUCCESS || count == 0 {
            return;
        }
        let mut props = vec![VkExtensionProperties::zeroed(); count as usize];
        let mut fetched = count;
        let res = enumerate(std::ptr::null(), &mut fetched, props.as_mut_ptr());
        if res != VK_SUCCESS && res != VK_INCOMPLETE {
            return;
        }
        props.truncate(fetched as usize);
        for p in &props {
            match p.name_str() {
                EXT_EXT_METAL_SURFACE => self.supports_metal_surface = true,
                EXT_MVK_MACOS_SURFACE => self.supports_macos_surface = true,
                EXT_KHR_PORTABILITY_ENUMERATION => self.supports_portability_enumeration = true,
                _ => {}
            }
        }
    }

    #[inline]
    unsafe fn dlsym_ptr(&self, name: &CStr) -> Option<*const c_void> {
        let p = libc::dlsym(self.handle as *mut c_void, name.as_ptr());
        (!p.is_null()).then_some(p as *const c_void)
    }

    #[inline]
    unsafe fn global_proc_ptr(&self, name: &CStr) -> Option<*const c_void> {
        let p = (self.get_instance_proc_addr)(0, name.as_ptr());
        if !p.is_null() {
            return Some(p);
        }
        self.dlsym_ptr(name)
    }

    #[inline]
    unsafe fn instance_proc_ptr(&self, instance: usize, name: &CStr) -> Option<*const c_void> {
        if instance != 0 {
            let p = (self.get_instance_proc_addr)(instance, name.as_ptr());
            if !p.is_null() {
                return Some(p);
            }
        }
        self.dlsym_ptr(name)
    }

    #[inline]
    unsafe fn device_proc_resolver(&self) -> Option<PfnVkGetDeviceProcAddr> {
        if let Some(f) = self.get_device_proc_addr {
            return Some(f);
        }
        let fallback = FALLBACK_GDPA.load(Ordering::Acquire);
        if fallback != 0 {
            return Some(std::mem::transmute::<usize, PfnVkGetDeviceProcAddr>(fallback));
        }
        None
    }

    #[inline]
    unsafe fn device_proc_raw(&self, device: usize, name: *const c_char) -> *const c_void {
        if device != 0 {
            if let Some(gdpa) = self.device_proc_resolver() {
                let p = gdpa(device, name);
                if !p.is_null() {
                    return p;
                }
            }
        }
        std::ptr::null()
    }

    #[inline]
    unsafe fn device_proc_ptr(&self, device: usize, name: &CStr) -> Option<*const c_void> {
        let p = self.device_proc_raw(device, name.as_ptr());
        if !p.is_null() {
            return Some(p);
        }
        self.dlsym_ptr(name)
    }

    #[inline]
    unsafe fn any_instance_or_dlsym_ptr(&self, name: &CStr) -> Option<*const c_void> {
        if let Some(p) = self.dlsym_ptr(name) {
            return Some(p);
        }
        let inst = live_instances().lock().unwrap().keys().next().copied();
        if let Some(i) = inst {
            let p = (self.get_instance_proc_addr)(i, name.as_ptr());
            if !p.is_null() {
                return Some(p);
            }
        }
        None
    }

    #[inline]
    unsafe fn device_proc_ptr_with_aliases(
        &self,
        device: usize,
        name: &CStr,
    ) -> Option<*const c_void> {
        if let Some(p) = self.device_proc_ptr(device, name) {
            return Some(p);
        }
        if let Ok(s) = name.to_str() {
            for &alias in promoted_proc_aliases(s) {
                if let Some(p) = self.device_proc_ptr(device, alias) {
                    return Some(p);
                }
            }
        }
        None
    }

    /// Resolves a command-buffer or queue entrypoint (`VkCommandBuffer` / `VkQueue` first arg)
    /// across `dlsym`, live `VkDevice` dispatch (`vkGetDeviceProcAddr`), and live `VkInstance`
    /// dispatch (`vkGetInstanceProcAddr`), trying Vulkan 1.2/1.3 <-> KHR/EXT promoted aliases.
    #[inline]
    unsafe fn any_device_or_instance_or_dlsym_ptr(&self, name: &CStr) -> Option<*const c_void> {
        let lookup_one = |sym: &CStr| -> Option<*const c_void> {
            if let Some(p) = self.dlsym_ptr(sym) {
                return Some(p);
            }
            let dev = live_devices().lock().unwrap().keys().next().copied();
            if let Some(d) = dev {
                let p = self.device_proc_raw(d, sym.as_ptr());
                if !p.is_null() {
                    return Some(p);
                }
            }
            let inst = live_instances().lock().unwrap().keys().next().copied();
            if let Some(i) = inst {
                let p = (self.get_instance_proc_addr)(i, sym.as_ptr());
                if !p.is_null() {
                    return Some(p);
                }
            }
            None
        };

        if let Some(p) = lookup_one(name) {
            return Some(p);
        }
        if let Ok(s) = name.to_str() {
            for &alias in promoted_proc_aliases(s) {
                if let Some(p) = lookup_one(alias) {
                    return Some(p);
                }
            }
        }
        None
    }
}

fn vulkan_backend() -> Option<&'static VulkanBackend> {
    static BACKEND: OnceLock<Option<VulkanBackend>> = OnceLock::new();
    BACKEND.get_or_init(|| unsafe { VulkanBackend::load() }).as_ref()
}

/// Registers a native Vulkan function pointer in `VA_DISPATCH` so `x86-runtime` can invoke it.
#[inline]
fn register_proc(p: *const c_void) -> *const c_void {
    if !p.is_null() {
        let addr = p as usize;
        crate::register_native_va(addr as u64, addr);
    }
    p
}

#[inline]
fn register_fn(p: *const ()) -> *const c_void {
    register_proc(p as *const c_void)
}

/// Guest `VkAllocationCallbacks` hold x86-64 callback pointers that native ARM64 MoltenVK cannot
/// jump to directly; when running a guest image under the loader, pass `NULL` consistently across
/// matched create/destroy pairs so MoltenVK uses the host allocator.
#[inline]
fn sanitize_allocator(allocator: *const c_void) -> *const c_void {
    if !allocator.is_null() && crate::try_loader().is_some() {
        std::ptr::null()
    } else {
        allocator
    }
}

/// Resolves the top-level Win32 `HWND` and its AppKit `CAMetalLayer*` (walking child windows to
/// their top-level parent when necessary).
fn metal_layer_for_win32_hwnd(hwnd: usize) -> Option<(usize, *mut c_void)> {
    if hwnd == 0 {
        return None;
    }
    if let Some(layer) = crate::macos::metal_layer_for_hwnd(hwnd) {
        return Some((hwnd, layer));
    }
    let mut cur = hwnd;
    for _ in 0..64 {
        let w = crate::user32::window::get(cur)?;
        let parent = {
            let d = w.data();
            if d.style & crate::user32::ws::CHILD != 0 {
                d.parent
            } else {
                0
            }
        };
        if parent == 0 || parent == cur {
            break;
        }
        cur = parent;
        if let Some(layer) = crate::macos::metal_layer_for_hwnd(cur) {
            return Some((cur, layer));
        }
    }
    None
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe fn set_metal_layer_size(layer: &objc2::runtime::AnyObject, width_px: f64, height_px: f64) {
    use objc2::msg_send;
    use objc2_core_foundation::{CGPoint, CGRect, CGSize};
    let scale: f64 = msg_send![layer, contentsScale];
    let scale = if scale > 0.0 && scale.is_finite() { scale } else { 1.0 };
    let bounds = CGRect::new(
        CGPoint::new(0.0, 0.0),
        CGSize::new((width_px / scale).max(1.0), (height_px / scale).max(1.0)),
    );
    let _: () = msg_send![layer, setBounds: bounds];
    let drawable = CGSize::new(width_px.max(1.0), height_px.max(1.0));
    let _: () = msg_send![layer, setDrawableSize: drawable];
}

struct ResolvedSurfaceLayer {
    hwnd: usize,
    layer: *mut c_void,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    headless_layer: Option<objc2::rc::Retained<objc2_quartz_core::CAMetalLayer>>,
}

/// Resolves a real `CAMetalLayer*` for `hwnd`:
/// - In windowed mode, uses the `NSWindow` content view's `CAMetalLayer` (draining pending main
///   queue window operations first so initial size is live).
/// - In headless mode (`PORT_HEADLESS=1` / `AW2_HEADLESS=1` with AppKit inactive), creates a real
///   offscreen `CAMetalLayer` sized to `hwnd`'s Win32 client rect — ONLY if `hwnd` is a genuine
///   window created in `user32::window` (bogus/null `HWND`s still honestly fail).
fn resolve_win32_surface_layer(hwnd: usize) -> Option<ResolvedSurfaceLayer> {
    if hwnd == 0 {
        return None;
    }
    if crate::macos::active() {
        let _ = crate::macos::run_sync(|| ());
    }
    if let Some((resolved_hwnd, layer)) = metal_layer_for_win32_hwnd(hwnd) {
        return Some(ResolvedSurfaceLayer {
            hwnd: resolved_hwnd,
            layer,
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            headless_layer: None,
        });
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    if !crate::macos::active() && is_headless_env() {
        let win = crate::user32::window::get(hwnd)?;
        let (w, h) = {
            let d = win.data();
            (d.client.width().max(1) as f64, d.client.height().max(1) as f64)
        };
        let layer = objc2_quartz_core::CAMetalLayer::new();
        layer.setContentsScale(1.0);
        layer.setOpaque(true);
        unsafe {
            let obj = &*(objc2::rc::Retained::as_ptr(&layer) as *const objc2::runtime::AnyObject);
            set_metal_layer_size(obj, w, h);
        }
        let layer_ptr = objc2::rc::Retained::as_ptr(&layer) as *mut c_void;
        log::info!(
            target: "vulkan",
            "vkCreateWin32SurfaceKHR: created headless offscreen CAMetalLayer {:p} ({}x{}) for hwnd {hwnd:#x}",
            layer_ptr,
            w as u32,
            h as u32
        );
        return Some(ResolvedSurfaceLayer {
            hwnd,
            layer: layer_ptr,
            headless_layer: Some(layer),
        });
    }

    None
}

/// Synchronizes a surface's backing `CAMetalLayer` with the latest Win32 window size before
/// MoltenVK queries surface capabilities or creates/recreates a swapchain:
/// - In windowed mode, `SetWindowPos`/`MoveWindow` enqueue `appkit::window_set_frame`
///   asynchronously on `_dispatch_main_q`. Running a FIFO `run_sync` barrier guarantees the
///   `NSWindow` resize has completed on the main thread and verifies the hosting `CAMetalLayer`
///   bounds match the updated client size.
/// - In headless mode, updates the offscreen `CAMetalLayer` bounds/drawableSize directly from
///   `user32::window::get(hwnd)`.
fn sync_surface_layer(surface: u64) {
    if surface == 0 {
        return;
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        let windowed_target = {
            let guard = live_surfaces().lock().unwrap();
            let Some(rec) = guard.get(&surface) else {
                return;
            };
            if let Some(ref headless_layer) = rec.headless_layer {
                if let Some(win) = crate::user32::window::get(rec.hwnd) {
                    let (w, h) = {
                        let d = win.data();
                        (d.client.width().max(1) as f64, d.client.height().max(1) as f64)
                    };
                    unsafe {
                        let obj = &*(objc2::rc::Retained::as_ptr(headless_layer)
                            as *const objc2::runtime::AnyObject);
                        set_metal_layer_size(obj, w, h);
                    }
                }
                return;
            }
            (rec.hwnd, rec.layer)
        };

        if crate::macos::active() {
            let (hwnd, expected_layer) = windowed_target;
            let _ = crate::macos::run_sync(move || {
                // Verify the window and its CAMetalLayer are still live after draining prior
                // async `window_set_frame` operations on `_dispatch_main_q`.
                let Some(shared) = crate::macos::window_shared(hwnd) else {
                    return;
                };
                if shared.layer != expected_layer || expected_layer == 0 {
                    return;
                }
                let (w_px, h_px) = shared.client_size();
                if w_px == 0 || h_px == 0 {
                    return;
                }
                unsafe {
                    use objc2::msg_send;
                    use objc2_core_foundation::{CGPoint, CGRect, CGSize};
                    let obj = &*(expected_layer as *const objc2::runtime::AnyObject);
                    let bounds: CGRect = msg_send![obj, bounds];
                    let scale: f64 = msg_send![obj, contentsScale];
                    let scale = if scale > 0.0 && scale.is_finite() { scale } else { 1.0 };
                    let cur_w = (bounds.size.width * scale).round() as u32;
                    let cur_h = (bounds.size.height * scale).round() as u32;
                    if cur_w != w_px || cur_h != h_px {
                        let new_bounds = CGRect::new(
                            CGPoint::new(0.0, 0.0),
                            CGSize::new(w_px as f64 / scale, h_px as f64 / scale),
                        );
                        let _: () = msg_send![obj, setBounds: new_bounds];
                    }
                }
            });
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _ = surface;
    }
}

// ---------------------------------------------------------------------------
// Extension translation helpers (pure & unit-tested)
// ---------------------------------------------------------------------------

/// If MoltenVK exposes `VK_EXT_metal_surface` or `VK_MVK_macos_surface`, also advertise
/// `VK_KHR_win32_surface` (spec version 6) so Windows Vulkan apps detect Win32 surface support.
pub(crate) fn augment_instance_extensions(exts: &mut Vec<VkExtensionProperties>) {
    let mut has_metal_or_macos = false;
    let mut has_win32 = false;
    for ext in exts.iter() {
        match ext.name_str() {
            EXT_EXT_METAL_SURFACE | EXT_MVK_MACOS_SURFACE => has_metal_or_macos = true,
            EXT_KHR_WIN32_SURFACE => has_win32 = true,
            _ => {}
        }
    }
    if has_metal_or_macos && !has_win32 {
        exts.push(VkExtensionProperties::new(
            EXT_KHR_WIN32_SURFACE,
            VK_KHR_WIN32_SURFACE_SPEC_VERSION,
        ));
    }
}

/// Rewrites Windows Vulkan instance extensions for MoltenVK / macOS:
/// - Rewrites `VK_KHR_win32_surface` -> `VK_EXT_metal_surface` (keeping `VK_KHR_surface`).
/// - Deduplicates extension names while preserving order.
/// - When `has_portability_enumeration` is true, appends `VK_KHR_portability_enumeration`
///   (if not already present) and sets `VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR`
///   (`0x0000_0001`) in `flags`.
#[allow(dead_code)]
pub(crate) fn rewrite_instance_extensions(
    requested: &[&str],
    flags: u32,
    has_portability_enumeration: bool,
) -> (Vec<CString>, u32) {
    rewrite_instance_extensions_with_surface(
        requested,
        flags,
        EXT_EXT_METAL_SURFACE,
        has_portability_enumeration,
    )
}

fn rewrite_instance_extensions_with_surface(
    requested: &[&str],
    mut flags: u32,
    surface_ext: &str,
    has_portability_enumeration: bool,
) -> (Vec<CString>, u32) {
    let mut out = Vec::with_capacity(requested.len() + 1);
    let mut seen = HashSet::with_capacity(requested.len() + 2);

    for &raw in requested {
        let mapped = if raw == EXT_KHR_WIN32_SURFACE {
            surface_ext
        } else {
            raw
        };
        if seen.insert(mapped.to_string()) {
            if let Ok(cs) = CString::new(mapped) {
                out.push(cs);
            }
        }
    }

    if has_portability_enumeration {
        if seen.insert(EXT_KHR_PORTABILITY_ENUMERATION.to_string()) {
            out.push(CString::new(EXT_KHR_PORTABILITY_ENUMERATION).unwrap());
        }
        flags |= VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR;
    }

    (out, flags)
}

/// Rewrites Windows Vulkan device extensions for MoltenVK / macOS:
/// - Preserves order and deduplicates requested extension names.
/// - Never silently drops explicitly requested extensions (honest `VK_ERROR_EXTENSION_NOT_PRESENT`
///   if an unsupported Windows-only device extension is requested).
/// - When `has_portability_subset` is true (advertised by MoltenVK's `VkPhysicalDevice`), appends
///   `VK_KHR_portability_subset` if not already present (mandated by Vulkan spec
///   `VUID-VkDeviceCreateInfo-pProperties-04451`).
pub(crate) fn rewrite_device_extensions(
    requested: &[&str],
    has_portability_subset: bool,
) -> Vec<CString> {
    let mut out = Vec::with_capacity(requested.len() + 1);
    let mut seen = HashSet::with_capacity(requested.len() + 2);

    for &raw in requested {
        if seen.insert(raw.to_string()) {
            if let Ok(cs) = CString::new(raw) {
                out.push(cs);
            }
        }
    }

    if has_portability_subset && seen.insert(EXT_KHR_PORTABILITY_SUBSET.to_string()) {
        out.push(CString::new(EXT_KHR_PORTABILITY_SUBSET).unwrap());
    }

    out
}

/// Bidirectional promoted Vulkan 1.1/1.2/1.3 <-> KHR/EXT procedure aliases with identical C ABI.
///
/// Used as fallback when a Vulkan loader/ICD exposes only the promoted core entrypoint or only
/// the KHR/EXT entrypoint (e.g. Vulkan 1.2 device enabling `VK_KHR_dynamic_rendering` /
/// `VK_KHR_synchronization2` / `VK_KHR_copy_commands2` / `VK_KHR_maintenance4` /
/// `VK_KHR_buffer_device_address` / `VK_KHR_timeline_semaphore`).
pub(crate) fn promoted_proc_aliases(name: &str) -> &'static [&'static CStr] {
    match name {
        // VK_KHR_dynamic_rendering <-> Vulkan 1.3
        "vkCmdBeginRendering" => &[c"vkCmdBeginRenderingKHR"],
        "vkCmdBeginRenderingKHR" => &[c"vkCmdBeginRendering"],
        "vkCmdEndRendering" => &[c"vkCmdEndRenderingKHR"],
        "vkCmdEndRenderingKHR" => &[c"vkCmdEndRendering"],

        // VK_KHR_synchronization2 <-> Vulkan 1.3
        "vkCmdPipelineBarrier2" => &[c"vkCmdPipelineBarrier2KHR"],
        "vkCmdPipelineBarrier2KHR" => &[c"vkCmdPipelineBarrier2"],
        "vkCmdSetEvent2" => &[c"vkCmdSetEvent2KHR"],
        "vkCmdSetEvent2KHR" => &[c"vkCmdSetEvent2"],
        "vkCmdResetEvent2" => &[c"vkCmdResetEvent2KHR"],
        "vkCmdResetEvent2KHR" => &[c"vkCmdResetEvent2"],
        "vkCmdWaitEvents2" => &[c"vkCmdWaitEvents2KHR"],
        "vkCmdWaitEvents2KHR" => &[c"vkCmdWaitEvents2"],
        "vkCmdWriteTimestamp2" => &[c"vkCmdWriteTimestamp2KHR"],
        "vkCmdWriteTimestamp2KHR" => &[c"vkCmdWriteTimestamp2"],
        "vkQueueSubmit2" => &[c"vkQueueSubmit2KHR"],
        "vkQueueSubmit2KHR" => &[c"vkQueueSubmit2"],

        // VK_KHR_buffer_device_address / VK_EXT_buffer_device_address <-> Vulkan 1.2
        "vkGetBufferDeviceAddress" => {
            &[c"vkGetBufferDeviceAddressKHR", c"vkGetBufferDeviceAddressEXT"]
        }
        "vkGetBufferDeviceAddressKHR" => {
            &[c"vkGetBufferDeviceAddress", c"vkGetBufferDeviceAddressEXT"]
        }
        "vkGetBufferDeviceAddressEXT" => {
            &[c"vkGetBufferDeviceAddressKHR", c"vkGetBufferDeviceAddress"]
        }
        "vkGetBufferOpaqueCaptureAddress" => &[c"vkGetBufferOpaqueCaptureAddressKHR"],
        "vkGetBufferOpaqueCaptureAddressKHR" => &[c"vkGetBufferOpaqueCaptureAddress"],
        "vkGetDeviceMemoryOpaqueCaptureAddress" => &[c"vkGetDeviceMemoryOpaqueCaptureAddressKHR"],
        "vkGetDeviceMemoryOpaqueCaptureAddressKHR" => &[c"vkGetDeviceMemoryOpaqueCaptureAddress"],

        // VK_KHR_copy_commands2 <-> Vulkan 1.3
        "vkCmdCopyBuffer2" => &[c"vkCmdCopyBuffer2KHR"],
        "vkCmdCopyBuffer2KHR" => &[c"vkCmdCopyBuffer2"],
        "vkCmdCopyImage2" => &[c"vkCmdCopyImage2KHR"],
        "vkCmdCopyImage2KHR" => &[c"vkCmdCopyImage2"],
        "vkCmdCopyBufferToImage2" => &[c"vkCmdCopyBufferToImage2KHR"],
        "vkCmdCopyBufferToImage2KHR" => &[c"vkCmdCopyBufferToImage2"],
        "vkCmdCopyImageToBuffer2" => &[c"vkCmdCopyImageToBuffer2KHR"],
        "vkCmdCopyImageToBuffer2KHR" => &[c"vkCmdCopyImageToBuffer2"],
        "vkCmdBlitImage2" => &[c"vkCmdBlitImage2KHR"],
        "vkCmdBlitImage2KHR" => &[c"vkCmdBlitImage2"],
        "vkCmdResolveImage2" => &[c"vkCmdResolveImage2KHR"],
        "vkCmdResolveImage2KHR" => &[c"vkCmdResolveImage2"],

        // VK_KHR_maintenance4 <-> Vulkan 1.3
        "vkGetDeviceBufferMemoryRequirements" => &[c"vkGetDeviceBufferMemoryRequirementsKHR"],
        "vkGetDeviceBufferMemoryRequirementsKHR" => &[c"vkGetDeviceBufferMemoryRequirements"],
        "vkGetDeviceImageMemoryRequirements" => &[c"vkGetDeviceImageMemoryRequirementsKHR"],
        "vkGetDeviceImageMemoryRequirementsKHR" => &[c"vkGetDeviceImageMemoryRequirements"],
        "vkGetDeviceImageSparseMemoryRequirements" => {
            &[c"vkGetDeviceImageSparseMemoryRequirementsKHR"]
        }
        "vkGetDeviceImageSparseMemoryRequirementsKHR" => {
            &[c"vkGetDeviceImageSparseMemoryRequirements"]
        }

        // VK_KHR_timeline_semaphore <-> Vulkan 1.2
        "vkGetSemaphoreCounterValue" => &[c"vkGetSemaphoreCounterValueKHR"],
        "vkGetSemaphoreCounterValueKHR" => &[c"vkGetSemaphoreCounterValue"],
        "vkWaitSemaphores" => &[c"vkWaitSemaphoresKHR"],
        "vkWaitSemaphoresKHR" => &[c"vkWaitSemaphores"],
        "vkSignalSemaphore" => &[c"vkSignalSemaphoreKHR"],
        "vkSignalSemaphoreKHR" => &[c"vkSignalSemaphore"],

        // VK_KHR_get_physical_device_properties2 <-> Vulkan 1.1
        "vkGetPhysicalDeviceFeatures2" => &[c"vkGetPhysicalDeviceFeatures2KHR"],
        "vkGetPhysicalDeviceFeatures2KHR" => &[c"vkGetPhysicalDeviceFeatures2"],
        "vkGetPhysicalDeviceProperties2" => &[c"vkGetPhysicalDeviceProperties2KHR"],
        "vkGetPhysicalDeviceProperties2KHR" => &[c"vkGetPhysicalDeviceProperties2"],
        "vkGetPhysicalDeviceQueueFamilyProperties2" => {
            &[c"vkGetPhysicalDeviceQueueFamilyProperties2KHR"]
        }
        "vkGetPhysicalDeviceQueueFamilyProperties2KHR" => {
            &[c"vkGetPhysicalDeviceQueueFamilyProperties2"]
        }
        "vkGetPhysicalDeviceMemoryProperties2" => &[c"vkGetPhysicalDeviceMemoryProperties2KHR"],
        "vkGetPhysicalDeviceMemoryProperties2KHR" => &[c"vkGetPhysicalDeviceMemoryProperties2"],
        "vkGetPhysicalDeviceFormatProperties2" => &[c"vkGetPhysicalDeviceFormatProperties2KHR"],
        "vkGetPhysicalDeviceFormatProperties2KHR" => &[c"vkGetPhysicalDeviceFormatProperties2"],
        "vkGetPhysicalDeviceImageFormatProperties2" => {
            &[c"vkGetPhysicalDeviceImageFormatProperties2KHR"]
        }
        "vkGetPhysicalDeviceImageFormatProperties2KHR" => {
            &[c"vkGetPhysicalDeviceImageFormatProperties2"]
        }

        // VK_KHR_create_renderpass2 <-> Vulkan 1.2
        "vkCreateRenderPass2" => &[c"vkCreateRenderPass2KHR"],
        "vkCreateRenderPass2KHR" => &[c"vkCreateRenderPass2"],

        // VK_KHR_sampler_ycbcr_conversion <-> Vulkan 1.1
        "vkCreateSamplerYcbcrConversion" => &[c"vkCreateSamplerYcbcrConversionKHR"],
        "vkCreateSamplerYcbcrConversionKHR" => &[c"vkCreateSamplerYcbcrConversion"],
        "vkDestroySamplerYcbcrConversion" => &[c"vkDestroySamplerYcbcrConversionKHR"],
        "vkDestroySamplerYcbcrConversionKHR" => &[c"vkDestroySamplerYcbcrConversion"],

        // VK_KHR_descriptor_update_template <-> Vulkan 1.1
        "vkCreateDescriptorUpdateTemplate" => &[c"vkCreateDescriptorUpdateTemplateKHR"],
        "vkCreateDescriptorUpdateTemplateKHR" => &[c"vkCreateDescriptorUpdateTemplate"],
        "vkDestroyDescriptorUpdateTemplate" => &[c"vkDestroyDescriptorUpdateTemplateKHR"],
        "vkDestroyDescriptorUpdateTemplateKHR" => &[c"vkDestroyDescriptorUpdateTemplate"],

        // VK_EXT_private_data <-> Vulkan 1.3
        "vkCreatePrivateDataSlot" => &[c"vkCreatePrivateDataSlotEXT"],
        "vkCreatePrivateDataSlotEXT" => &[c"vkCreatePrivateDataSlot"],
        "vkDestroyPrivateDataSlot" => &[c"vkDestroyPrivateDataSlotEXT"],
        "vkDestroyPrivateDataSlotEXT" => &[c"vkDestroyPrivateDataSlot"],

        _ => &[],
    }
}

unsafe fn physical_device_supports_portability_subset(
    vk: &VulkanBackend,
    physical_device: usize,
) -> bool {
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkEnumerateDeviceExtensionProperties") else {
        return false;
    };
    let enumerate: unsafe extern "C" fn(
        usize,
        *const c_char,
        *mut u32,
        *mut VkExtensionProperties,
    ) -> i32 = std::mem::transmute(ptr);
    let mut count = 0u32;
    if enumerate(
        physical_device,
        std::ptr::null(),
        &mut count,
        std::ptr::null_mut(),
    ) != VK_SUCCESS
        || count == 0
    {
        return false;
    }
    let mut props = vec![VkExtensionProperties::zeroed(); count as usize];
    let mut fetched = count;
    let res = enumerate(
        physical_device,
        std::ptr::null(),
        &mut fetched,
        props.as_mut_ptr(),
    );
    if res != VK_SUCCESS && res != VK_INCOMPLETE {
        return false;
    }
    props.truncate(fetched as usize);
    props
        .iter()
        .any(|p| p.name_str() == EXT_KHR_PORTABILITY_SUBSET)
}

// ---------------------------------------------------------------------------
// Exported Vulkan entrypoints (intercepted & translated)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn vkGetInstanceProcAddr(
    instance: usize,
    p_name: *const c_char,
) -> *const c_void {
    if p_name.is_null() {
        return std::ptr::null();
    }
    let c_name = CStr::from_ptr(p_name);
    let name = c_name.to_str().unwrap_or("");

    if name == "vkGetInstanceProcAddr" {
        return register_fn(vkGetInstanceProcAddr as *const ());
    }

    if instance == 0 {
        return match name {
            "vkEnumerateInstanceVersion" => register_fn(vkEnumerateInstanceVersion as *const ()),
            "vkEnumerateInstanceExtensionProperties" => {
                register_fn(vkEnumerateInstanceExtensionProperties as *const ())
            }
            "vkEnumerateInstanceLayerProperties" => {
                register_fn(vkEnumerateInstanceLayerProperties as *const ())
            }
            "vkCreateInstance" => register_fn(vkCreateInstance as *const ()),
            _ => std::ptr::null(),
        };
    }

    match name {
        // Per Vulkan 1.2+, global commands return NULL when instance != VK_NULL_HANDLE.
        "vkCreateInstance"
        | "vkEnumerateInstanceVersion"
        | "vkEnumerateInstanceExtensionProperties"
        | "vkEnumerateInstanceLayerProperties" => std::ptr::null(),

        "vkGetDeviceProcAddr" => register_fn(vkGetDeviceProcAddr as *const ()),
        "vkDestroyInstance" => register_fn(vkDestroyInstance as *const ()),
        "vkCreateDevice" => {
            if vulkan_backend().is_some() {
                register_fn(vkCreateDevice as *const ())
            } else {
                std::ptr::null()
            }
        }
        "vkDestroyDevice" => {
            if vulkan_backend().is_some() {
                register_fn(vkDestroyDevice as *const ())
            } else {
                std::ptr::null()
            }
        }
        "vkCreateWin32SurfaceKHR" => {
            if vulkan_backend().is_some() {
                register_fn(vkCreateWin32SurfaceKHR as *const ())
            } else {
                std::ptr::null()
            }
        }
        "vkGetPhysicalDeviceWin32PresentationSupportKHR" => {
            if vulkan_backend().is_some() {
                register_fn(vkGetPhysicalDeviceWin32PresentationSupportKHR as *const ())
            } else {
                std::ptr::null()
            }
        }
        "vkDestroySurfaceKHR" => {
            if vulkan_backend().is_some() {
                register_fn(vkDestroySurfaceKHR as *const ())
            } else {
                std::ptr::null()
            }
        }
        _ => {
            let Some(vk) = vulkan_backend() else {
                return std::ptr::null();
            };
            let mut ptr = (vk.get_instance_proc_addr)(instance, p_name);
            if ptr.is_null() {
                for &alias in promoted_proc_aliases(name) {
                    ptr = (vk.get_instance_proc_addr)(instance, alias.as_ptr());
                    if !ptr.is_null() {
                        break;
                    }
                }
            }
            if ptr.is_null() {
                return std::ptr::null();
            }
            if let Some(wrapper) = intercepted_instance_or_device_proc(name) {
                return register_fn(wrapper);
            }
            register_proc(ptr)
        }
    }
}

pub unsafe extern "win64" fn vkGetDeviceProcAddr(
    device: usize,
    p_name: *const c_char,
) -> *const c_void {
    if device == 0 || p_name.is_null() {
        return std::ptr::null();
    }
    let name = CStr::from_ptr(p_name).to_str().unwrap_or("");
    if name == "vkGetDeviceProcAddr" {
        return register_fn(vkGetDeviceProcAddr as *const ());
    }
    let Some(vk) = vulkan_backend() else {
        return std::ptr::null();
    };
    if name == "vkDestroyDevice" {
        return register_fn(vkDestroyDevice as *const ());
    }
    let mut ptr = vk.device_proc_raw(device, p_name);
    if ptr.is_null() {
        for &alias in promoted_proc_aliases(name) {
            ptr = vk.device_proc_raw(device, alias.as_ptr());
            if !ptr.is_null() {
                break;
            }
        }
    }
    if ptr.is_null() {
        return std::ptr::null();
    }
    if let Some(wrapper) = intercepted_device_proc(name) {
        return register_fn(wrapper);
    }
    register_proc(ptr)
}

pub unsafe extern "win64" fn vkEnumerateInstanceVersion(api_version: *mut u32) -> i32 {
    if api_version.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(api_version, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    if let Some(ptr) = vk.global_proc_ptr(c"vkEnumerateInstanceVersion") {
        let f: PfnVkEnumerateInstanceVersion = std::mem::transmute(ptr);
        let mut ver = 0u32;
        let res = f(&mut ver);
        std::ptr::write_unaligned(api_version, ver);
        return res;
    }
    std::ptr::write_unaligned(api_version, VK_API_VERSION_1_0);
    VK_SUCCESS
}

pub unsafe extern "win64" fn vkEnumerateInstanceExtensionProperties(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> i32 {
    if property_count.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(property_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.global_proc_ptr(c"vkEnumerateInstanceExtensionProperties") else {
        std::ptr::write_unaligned(property_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let enumerate: PfnVkEnumerateInstanceExtensionProperties = std::mem::transmute(ptr);

    // Layer-specific extension queries pass through directly.
    if !layer_name.is_null() {
        let mut count = std::ptr::read_unaligned(property_count);
        let res = enumerate(layer_name, &mut count, properties);
        std::ptr::write_unaligned(property_count, count);
        return res;
    }

    let mut real_count: u32 = 0;
    let r1 = enumerate(std::ptr::null(), &mut real_count, std::ptr::null_mut());
    if r1 != VK_SUCCESS {
        std::ptr::write_unaligned(property_count, 0);
        return r1;
    }

    let mut exts = vec![VkExtensionProperties::zeroed(); real_count as usize];
    if real_count > 0 {
        let mut fetched = real_count;
        let r2 = enumerate(std::ptr::null(), &mut fetched, exts.as_mut_ptr());
        if r2 != VK_SUCCESS && r2 != VK_INCOMPLETE {
            std::ptr::write_unaligned(property_count, 0);
            return r2;
        }
        exts.truncate(fetched as usize);
    }

    augment_instance_extensions(&mut exts);

    if properties.is_null() {
        std::ptr::write_unaligned(property_count, exts.len() as u32);
        return VK_SUCCESS;
    }

    let cap = std::ptr::read_unaligned(property_count) as usize;
    let to_copy = cap.min(exts.len());
    for (i, ext) in exts.iter().take(to_copy).enumerate() {
        std::ptr::write_unaligned(properties.add(i), *ext);
    }
    std::ptr::write_unaligned(property_count, to_copy as u32);
    if to_copy < exts.len() {
        VK_INCOMPLETE
    } else {
        VK_SUCCESS
    }
}

pub unsafe extern "win64" fn vkEnumerateInstanceLayerProperties(
    property_count: *mut u32,
    properties: *mut c_void,
) -> i32 {
    if property_count.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(property_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.global_proc_ptr(c"vkEnumerateInstanceLayerProperties") else {
        std::ptr::write_unaligned(property_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let enumerate: PfnVkEnumerateInstanceLayerProperties = std::mem::transmute(ptr);
    let mut count = std::ptr::read_unaligned(property_count);
    let res = enumerate(&mut count, properties);
    std::ptr::write_unaligned(property_count, count);
    res
}

pub unsafe extern "win64" fn vkCreateInstance(
    create_info: *const VkInstanceCreateInfo,
    allocator: *const c_void,
    instance: *mut usize,
) -> i32 {
    if !instance.is_null() {
        std::ptr::write_unaligned(instance, 0);
    }
    if create_info.is_null() || instance.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.global_proc_ptr(c"vkCreateInstance") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let create_fn: PfnVkCreateInstance = std::mem::transmute(ptr);

    let orig = std::ptr::read_unaligned(create_info);
    if orig.enabled_extension_count > 0 && orig.pp_enabled_extension_names.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    if orig.enabled_layer_count > 0 && orig.pp_enabled_layer_names.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }

    let mut req_owned = Vec::with_capacity(orig.enabled_extension_count as usize);
    for i in 0..orig.enabled_extension_count as usize {
        let s_ptr = std::ptr::read_unaligned(orig.pp_enabled_extension_names.add(i));
        if s_ptr.is_null() {
            return VK_ERROR_INITIALIZATION_FAILED;
        }
        req_owned.push(CStr::from_ptr(s_ptr).to_string_lossy().into_owned());
    }
    let req_refs: Vec<&str> = req_owned.iter().map(String::as_str).collect();

    let target_surface_ext = if vk.supports_metal_surface || !vk.supports_macos_surface {
        EXT_EXT_METAL_SURFACE
    } else {
        EXT_MVK_MACOS_SURFACE
    };
    let (rewritten_exts, new_flags) = rewrite_instance_extensions_with_surface(
        &req_refs,
        orig.flags,
        target_surface_ext,
        vk.supports_portability_enumeration,
    );
    let ext_ptrs: Vec<*const c_char> = rewritten_exts.iter().map(|s| s.as_ptr()).collect();

    let translated = VkInstanceCreateInfo {
        flags: new_flags,
        enabled_extension_count: ext_ptrs.len() as u32,
        pp_enabled_extension_names: if ext_ptrs.is_empty() {
            std::ptr::null()
        } else {
            ext_ptrs.as_ptr()
        },
        ..orig
    };

    let mut out_inst: usize = 0;
    let res = create_fn(&translated, sanitize_allocator(allocator), &mut out_inst);
    std::ptr::write_unaligned(instance, out_inst);

    if res == VK_SUCCESS && out_inst != 0 {
        let create_metal_surface_ext = vk
            .instance_proc_ptr(out_inst, c"vkCreateMetalSurfaceEXT")
            .map(|p| unsafe { std::mem::transmute::<*const c_void, PfnVkCreateMetalSurfaceEXT>(p) });
        let create_macos_surface_mvk = vk
            .instance_proc_ptr(out_inst, c"vkCreateMacOSSurfaceMVK")
            .map(|p| unsafe { std::mem::transmute::<*const c_void, PfnVkCreateMacOSSurfaceMVK>(p) });
        let destroy_surface_khr = vk
            .instance_proc_ptr(out_inst, c"vkDestroySurfaceKHR")
            .map(|p| unsafe { std::mem::transmute::<*const c_void, PfnVkDestroySurfaceKHR>(p) });
        if vk.get_device_proc_addr.is_none() {
            if let Some(p) = vk.instance_proc_ptr(out_inst, c"vkGetDeviceProcAddr") {
                FALLBACK_GDPA.store(p as usize, Ordering::Release);
            }
        }
        live_instances().lock().unwrap().insert(
            out_inst,
            InstanceProcs {
                create_metal_surface_ext,
                create_macos_surface_mvk,
                destroy_surface_khr,
            },
        );
        log::info!(
            target: "vulkan",
            "vkCreateInstance -> {out_inst:#x} (flags={new_flags:#x}, exts={:?})",
            rewritten_exts
        );
    } else {
        log::warn!(
            target: "vulkan",
            "vkCreateInstance failed with VkResult {res} (exts={:?})",
            rewritten_exts
        );
    }

    res
}

pub unsafe extern "win64" fn vkDestroyInstance(instance: usize, allocator: *const c_void) {
    if instance == 0 {
        return;
    }
    live_instances().lock().unwrap().remove(&instance);
    if let Some(vk) = vulkan_backend() {
        if let Some(ptr) = vk.instance_proc_ptr(instance, c"vkDestroyInstance") {
            let destroy: PfnVkDestroyInstance = std::mem::transmute(ptr);
            destroy(instance, sanitize_allocator(allocator));
        }
    }
    // Drop any headless CAMetalLayer records owned by surfaces on this instance ONLY AFTER
    // MoltenVK has finished destroying the VkInstance.
    live_surfaces()
        .lock()
        .unwrap()
        .retain(|_, rec| rec.instance != instance);
}

pub unsafe extern "win64" fn vkCreateWin32SurfaceKHR(
    instance: usize,
    create_info: *const VkWin32SurfaceCreateInfoKHR,
    allocator: *const c_void,
    surface: *mut u64,
) -> i32 {
    if !surface.is_null() {
        std::ptr::write_unaligned(surface, 0);
    }
    if instance == 0 || create_info.is_null() || surface.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };

    let info = std::ptr::read_unaligned(create_info);
    if info.s_type != VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR {
        log::warn!(
            target: "vulkan",
            "vkCreateWin32SurfaceKHR: unexpected sType={} (expected {})",
            info.s_type,
            VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR
        );
    }

    let Some(resolved) = resolve_win32_surface_layer(info.hwnd) else {
        log::warn!(
            target: "vulkan",
            "vkCreateWin32SurfaceKHR: hwnd {:#x} has no CAMetalLayer",
            info.hwnd
        );
        return VK_ERROR_INITIALIZATION_FAILED;
    };
    let layer = resolved.layer;

    let cached = live_instances().lock().unwrap().get(&instance).copied();
    let create_metal = cached
        .and_then(|p| p.create_metal_surface_ext)
        .or_else(|| unsafe {
            vk.instance_proc_ptr(instance, c"vkCreateMetalSurfaceEXT")
                .map(|p| std::mem::transmute::<*const c_void, PfnVkCreateMetalSurfaceEXT>(p))
        });

    let alloc = sanitize_allocator(allocator);

    if let Some(f) = create_metal {
        let metal_ci = VkMetalSurfaceCreateInfoEXT {
            s_type: VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: 0,
            p_layer: layer as *const c_void,
        };
        let mut out_surface: u64 = 0;
        let res = f(instance, &metal_ci, alloc, &mut out_surface);
        std::ptr::write_unaligned(surface, out_surface);
        if res == VK_SUCCESS && out_surface != 0 {
            live_surfaces().lock().unwrap().insert(
                out_surface,
                SurfaceRecord {
                    instance,
                    hwnd: resolved.hwnd,
                    layer: layer as usize,
                    #[cfg(any(target_os = "macos", target_os = "ios"))]
                    headless_layer: resolved.headless_layer,
                },
            );
        }
        log::info!(
            target: "vulkan",
            "vkCreateWin32SurfaceKHR(hwnd={:#x}, layer={:p}) -> vkCreateMetalSurfaceEXT={res} (surface={out_surface:#x})",
            info.hwnd,
            layer
        );
        return res;
    }

    let create_macos = cached
        .and_then(|p| p.create_macos_surface_mvk)
        .or_else(|| unsafe {
            vk.instance_proc_ptr(instance, c"vkCreateMacOSSurfaceMVK")
                .map(|p| std::mem::transmute::<*const c_void, PfnVkCreateMacOSSurfaceMVK>(p))
        });

    if let Some(f) = create_macos {
        let macos_ci = VkMacOSSurfaceCreateInfoMVK {
            s_type: VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK,
            p_next: std::ptr::null(),
            flags: 0,
            p_view: layer as *const c_void,
        };
        let mut out_surface: u64 = 0;
        let res = f(instance, &macos_ci, alloc, &mut out_surface);
        std::ptr::write_unaligned(surface, out_surface);
        if res == VK_SUCCESS && out_surface != 0 {
            live_surfaces().lock().unwrap().insert(
                out_surface,
                SurfaceRecord {
                    instance,
                    hwnd: resolved.hwnd,
                    layer: layer as usize,
                    #[cfg(any(target_os = "macos", target_os = "ios"))]
                    headless_layer: resolved.headless_layer,
                },
            );
        }
        log::info!(
            target: "vulkan",
            "vkCreateWin32SurfaceKHR(hwnd={:#x}, layer={:p}) -> vkCreateMacOSSurfaceMVK={res} (surface={out_surface:#x})",
            info.hwnd,
            layer
        );
        return res;
    }

    VK_ERROR_EXTENSION_NOT_PRESENT
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
    physical_device: usize,
    queue_family_index: u32,
) -> u32 {
    if physical_device == 0 {
        return VK_FALSE;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_FALSE;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceQueueFamilyProperties")
    else {
        return VK_FALSE;
    };
    let get_qfp: PfnVkGetPhysicalDeviceQueueFamilyProperties = std::mem::transmute(ptr);
    let mut count: u32 = 0;
    get_qfp(physical_device, &mut count, std::ptr::null_mut());
    if queue_family_index >= count {
        return VK_FALSE;
    }
    let mut props = vec![VkQueueFamilyProperties::default(); count as usize];
    let mut fetched = count;
    get_qfp(physical_device, &mut fetched, props.as_mut_ptr());
    let Some(qf) = props.get(queue_family_index as usize) else {
        return VK_FALSE;
    };
    if qf.queue_flags & VK_QUEUE_GRAPHICS_BIT != 0 {
        VK_TRUE
    } else {
        VK_FALSE
    }
}

pub unsafe extern "win64" fn vkDestroySurfaceKHR(
    instance: usize,
    surface: u64,
    allocator: *const c_void,
) {
    if instance == 0 || surface == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    let cached = live_instances()
        .lock()
        .unwrap()
        .get(&instance)
        .and_then(|p| p.destroy_surface_khr);
    let destroy = cached.or_else(|| unsafe {
        vk.instance_proc_ptr(instance, c"vkDestroySurfaceKHR")
            .map(|p| std::mem::transmute::<*const c_void, PfnVkDestroySurfaceKHR>(p))
    });
    if let Some(f) = destroy {
        f(instance, surface, sanitize_allocator(allocator));
    }
    // Remove AFTER MoltenVK destroys the surface so any headless CAMetalLayer stays alive
    // throughout `vkDestroySurfaceKHR`.
    live_surfaces().lock().unwrap().remove(&surface);
}

// ---------------------------------------------------------------------------
// Statically exported core Vulkan 1.0-1.3 / KHR forwarding entrypoints
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn vkEnumeratePhysicalDevices(
    instance: usize,
    device_count: *mut u32,
    devices: *mut usize,
) -> i32 {
    if device_count.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(device_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    if instance == 0 {
        std::ptr::write_unaligned(device_count, 0);
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(ptr) = vk.instance_proc_ptr(instance, c"vkEnumeratePhysicalDevices") else {
        std::ptr::write_unaligned(device_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: PfnVkEnumeratePhysicalDevices = std::mem::transmute(ptr);
    let mut count = std::ptr::read_unaligned(device_count);
    let res = f(instance, &mut count, devices);
    std::ptr::write_unaligned(device_count, count);
    res
}

pub unsafe extern "win64" fn vkCreateDevice(
    physical_device: usize,
    create_info: *const c_void,
    allocator: *const c_void,
    device: *mut usize,
) -> i32 {
    if !device.is_null() {
        std::ptr::write_unaligned(device, 0);
    }
    if physical_device == 0 || create_info.is_null() || device.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkCreateDevice") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, *const VkDeviceCreateInfo, *const c_void, *mut usize) -> i32 =
        std::mem::transmute(ptr);

    let orig = std::ptr::read_unaligned(create_info as *const VkDeviceCreateInfo);
    if orig.enabled_extension_count > 0 && orig.pp_enabled_extension_names.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    if orig.enabled_layer_count > 0 && orig.pp_enabled_layer_names.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }

    let mut req_owned = Vec::with_capacity(orig.enabled_extension_count as usize);
    for i in 0..orig.enabled_extension_count as usize {
        let s_ptr = std::ptr::read_unaligned(orig.pp_enabled_extension_names.add(i));
        if s_ptr.is_null() {
            return VK_ERROR_INITIALIZATION_FAILED;
        }
        req_owned.push(CStr::from_ptr(s_ptr).to_string_lossy().into_owned());
    }
    let req_refs: Vec<&str> = req_owned.iter().map(String::as_str).collect();
    let has_portability_subset = physical_device_supports_portability_subset(vk, physical_device);
    let rewritten_exts = rewrite_device_extensions(&req_refs, has_portability_subset);
    let ext_ptrs: Vec<*const c_char> = rewritten_exts.iter().map(|s| s.as_ptr()).collect();

    let translated = VkDeviceCreateInfo {
        enabled_extension_count: ext_ptrs.len() as u32,
        pp_enabled_extension_names: if ext_ptrs.is_empty() {
            std::ptr::null()
        } else {
            ext_ptrs.as_ptr()
        },
        ..orig
    };

    let mut out = 0usize;
    let res = f(physical_device, &translated, sanitize_allocator(allocator), &mut out);
    std::ptr::write_unaligned(device, out);
    if res == VK_SUCCESS && out != 0 {
        live_devices().lock().unwrap().insert(out, physical_device);
    }
    res
}

pub unsafe extern "win64" fn vkDestroyDevice(device: usize, allocator: *const c_void) {
    if device == 0 {
        return;
    }
    live_devices().lock().unwrap().remove(&device);
    live_queues().lock().unwrap().retain(|_, rec| rec.device != device);
    live_swapchains().lock().unwrap().retain(|_, rec| rec.device != device);
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkDestroyDevice") {
        let f: unsafe extern "C" fn(usize, *const c_void) = std::mem::transmute(ptr);
        f(device, sanitize_allocator(allocator));
    }
}

pub unsafe extern "win64" fn vkDeviceWaitIdle(device: usize) -> i32 {
    if device == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkDeviceWaitIdle") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize) -> i32 = std::mem::transmute(ptr);
    f(device)
}

pub unsafe extern "win64" fn vkEnumerateDeviceExtensionProperties(
    physical_device: usize,
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> i32 {
    if property_count.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(property_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    if physical_device == 0 {
        std::ptr::write_unaligned(property_count, 0);
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkEnumerateDeviceExtensionProperties") else {
        std::ptr::write_unaligned(property_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(
        usize,
        *const c_char,
        *mut u32,
        *mut VkExtensionProperties,
    ) -> i32 = std::mem::transmute(ptr);
    let mut count = std::ptr::read_unaligned(property_count);
    let res = f(physical_device, layer_name, &mut count, properties);
    std::ptr::write_unaligned(property_count, count);
    res
}

/// Forwards SPIR-V shader module creation directly to the active MoltenVK `VkDevice`
/// (offline SPIR-V -> Metal precompilation caches in `~/Translator/ShaderCache`).
pub unsafe extern "win64" fn vkCreateShaderModule(
    device: usize,
    create_info: *const VkShaderModuleCreateInfo,
    allocator: *const c_void,
    shader_module: *mut u64,
) -> i32 {
    if !shader_module.is_null() {
        std::ptr::write_unaligned(shader_module, 0);
    }
    if create_info.is_null() || shader_module.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    if device == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(ptr) = vk.device_proc_ptr(device, c"vkCreateShaderModule") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: PfnVkCreateShaderModule = std::mem::transmute(ptr);
    let info = std::ptr::read_unaligned(create_info);
    let mut out_mod = 0u64;
    let res = f(device, &info, sanitize_allocator(allocator), &mut out_mod);
    std::ptr::write_unaligned(shader_module, out_mod);
    res
}

pub unsafe extern "win64" fn vkDestroyShaderModule(
    device: usize,
    shader_module: u64,
    allocator: *const c_void,
) {
    if device == 0 || shader_module == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkDestroyShaderModule") {
        let f: PfnVkDestroyShaderModule = std::mem::transmute(ptr);
        f(device, shader_module, sanitize_allocator(allocator));
    }
}

pub unsafe extern "win64" fn vkAllocateCommandBuffers(
    device: usize,
    alloc_info: *const c_void,
    cmd_buffers: *mut usize,
) -> i32 {
    if !cmd_buffers.is_null() {
        std::ptr::write_unaligned(cmd_buffers, 0);
    }
    if device == 0 || alloc_info.is_null() || cmd_buffers.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkAllocateCommandBuffers") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *mut usize) -> i32 = std::mem::transmute(ptr);
    f(device, alloc_info, cmd_buffers)
}

pub unsafe extern "win64" fn vkFreeCommandBuffers(
    device: usize,
    cmd_pool: u64,
    count: u32,
    cmd_buffers: *const usize,
) {
    if device == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkFreeCommandBuffers") {
        let f: unsafe extern "C" fn(usize, u64, u32, *const usize) = std::mem::transmute(ptr);
        f(device, cmd_pool, count, cmd_buffers);
    }
}

pub unsafe extern "win64" fn vkBeginCommandBuffer(
    cmd_buf: usize,
    begin_info: *const c_void,
) -> i32 {
    if cmd_buf == 0 || begin_info.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkBeginCommandBuffer") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, *const c_void) -> i32 = std::mem::transmute(ptr);
    f(cmd_buf, begin_info)
}

pub unsafe extern "win64" fn vkEndCommandBuffer(cmd_buf: usize) -> i32 {
    if cmd_buf == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkEndCommandBuffer") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize) -> i32 = std::mem::transmute(ptr);
    f(cmd_buf)
}

pub unsafe extern "win64" fn vkResetCommandBuffer(cmd_buf: usize, flags: u32) -> i32 {
    if cmd_buf == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkResetCommandBuffer") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, u32) -> i32 = std::mem::transmute(ptr);
    f(cmd_buf, flags)
}

pub unsafe extern "win64" fn vkCreateCommandPool(
    device: usize,
    create_info: *const c_void,
    alloc: *const c_void,
    cmd_pool: *mut u64,
) -> i32 {
    if !cmd_pool.is_null() {
        std::ptr::write_unaligned(cmd_pool, 0);
    }
    if device == 0 || create_info.is_null() || cmd_pool.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkCreateCommandPool") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *const c_void, *mut u64) -> i32 =
        std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(device, create_info, sanitize_allocator(alloc), &mut out);
    std::ptr::write_unaligned(cmd_pool, out);
    res
}

pub unsafe extern "win64" fn vkDestroyCommandPool(
    device: usize,
    cmd_pool: u64,
    alloc: *const c_void,
) {
    if device == 0 || cmd_pool == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkDestroyCommandPool") {
        let f: unsafe extern "C" fn(usize, u64, *const c_void) = std::mem::transmute(ptr);
        f(device, cmd_pool, sanitize_allocator(alloc));
    }
}

pub unsafe extern "win64" fn vkResetCommandPool(device: usize, cmd_pool: u64, flags: u32) -> i32 {
    if device == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkResetCommandPool") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, u64, u32) -> i32 = std::mem::transmute(ptr);
    f(device, cmd_pool, flags)
}

pub unsafe extern "win64" fn vkCreateSemaphore(
    device: usize,
    create_info: *const c_void,
    alloc: *const c_void,
    semaphore: *mut u64,
) -> i32 {
    if !semaphore.is_null() {
        std::ptr::write_unaligned(semaphore, 0);
    }
    if device == 0 || create_info.is_null() || semaphore.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkCreateSemaphore") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *const c_void, *mut u64) -> i32 =
        std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(device, create_info, sanitize_allocator(alloc), &mut out);
    std::ptr::write_unaligned(semaphore, out);
    res
}

pub unsafe extern "win64" fn vkDestroySemaphore(
    device: usize,
    semaphore: u64,
    alloc: *const c_void,
) {
    if device == 0 || semaphore == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkDestroySemaphore") {
        let f: unsafe extern "C" fn(usize, u64, *const c_void) = std::mem::transmute(ptr);
        f(device, semaphore, sanitize_allocator(alloc));
    }
}

pub unsafe extern "win64" fn vkGetSemaphoreCounterValue(
    device: usize,
    semaphore: u64,
    value: *mut u64,
) -> i32 {
    if !value.is_null() {
        std::ptr::write_unaligned(value, 0);
    }
    if device == 0 || value.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk
        .device_proc_ptr(device, c"vkGetSemaphoreCounterValue")
        .or_else(|| unsafe { vk.device_proc_ptr(device, c"vkGetSemaphoreCounterValueKHR") })
    else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, u64, *mut u64) -> i32 = std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(device, semaphore, &mut out);
    std::ptr::write_unaligned(value, out);
    res
}

pub unsafe extern "win64" fn vkWaitSemaphores(
    device: usize,
    wait_info: *const c_void,
    timeout: u64,
) -> i32 {
    if device == 0 || wait_info.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk
        .device_proc_ptr(device, c"vkWaitSemaphores")
        .or_else(|| unsafe { vk.device_proc_ptr(device, c"vkWaitSemaphoresKHR") })
    else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, u64) -> i32 = std::mem::transmute(ptr);
    f(device, wait_info, timeout)
}

pub unsafe extern "win64" fn vkCreateFence(
    device: usize,
    create_info: *const c_void,
    alloc: *const c_void,
    fence: *mut u64,
) -> i32 {
    if !fence.is_null() {
        std::ptr::write_unaligned(fence, 0);
    }
    if device == 0 || create_info.is_null() || fence.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkCreateFence") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *const c_void, *mut u64) -> i32 =
        std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(device, create_info, sanitize_allocator(alloc), &mut out);
    std::ptr::write_unaligned(fence, out);
    res
}

pub unsafe extern "win64" fn vkDestroyFence(device: usize, fence: u64, alloc: *const c_void) {
    if device == 0 || fence == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkDestroyFence") {
        let f: unsafe extern "C" fn(usize, u64, *const c_void) = std::mem::transmute(ptr);
        f(device, fence, sanitize_allocator(alloc));
    }
}

pub unsafe extern "win64" fn vkWaitForFences(
    device: usize,
    fence_count: u32,
    fences: *const u64,
    wait_all: u32,
    timeout: u64,
) -> i32 {
    if device == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkWaitForFences") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, u32, *const u64, u32, u64) -> i32 =
        std::mem::transmute(ptr);
    f(device, fence_count, fences, wait_all, timeout)
}

pub unsafe extern "win64" fn vkGetFenceStatus(device: usize, fence: u64) -> i32 {
    if device == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkGetFenceStatus") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, u64) -> i32 = std::mem::transmute(ptr);
    f(device, fence)
}

pub unsafe extern "win64" fn vkResetFences(
    device: usize,
    fence_count: u32,
    fences: *const u64,
) -> i32 {
    if device == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkResetFences") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, u32, *const u64) -> i32 = std::mem::transmute(ptr);
    f(device, fence_count, fences)
}

pub unsafe extern "win64" fn vkDestroyImageView(
    device: usize,
    image_view: u64,
    alloc: *const c_void,
) {
    if device == 0 || image_view == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkDestroyImageView") {
        let f: unsafe extern "C" fn(usize, u64, *const c_void) = std::mem::transmute(ptr);
        f(device, image_view, sanitize_allocator(alloc));
    }
}

pub unsafe extern "win64" fn vkGetImageMemoryRequirements(
    device: usize,
    image: u64,
    mem_reqs: *mut VkMemoryRequirements,
) {
    if mem_reqs.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(mem_reqs, VkMemoryRequirements::default());
        return;
    };
    if device == 0 {
        std::ptr::write_unaligned(mem_reqs, VkMemoryRequirements::default());
        return;
    }
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkGetImageMemoryRequirements") {
        let f: unsafe extern "C" fn(usize, u64, *mut VkMemoryRequirements) =
            std::mem::transmute(ptr);
        let mut req = VkMemoryRequirements::default();
        f(device, image, &mut req);
        std::ptr::write_unaligned(mem_reqs, req);
    } else {
        std::ptr::write_unaligned(mem_reqs, VkMemoryRequirements::default());
    }
}

pub unsafe extern "win64" fn vkGetDeviceQueue(
    device: usize,
    queue_family_idx: u32,
    queue_idx: u32,
    p_queue: *mut usize,
) {
    if p_queue.is_null() {
        return;
    }
    std::ptr::write_unaligned(p_queue, 0);
    if device == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkGetDeviceQueue") {
        let f: unsafe extern "C" fn(usize, u32, u32, *mut usize) = std::mem::transmute(ptr);
        let mut q = 0usize;
        f(device, queue_family_idx, queue_idx, &mut q);
        if q != 0 {
            live_queues().lock().unwrap().insert(
                q,
                QueueRecord {
                    device,
                    family_index: queue_family_idx,
                },
            );
        }
        std::ptr::write_unaligned(p_queue, q);
    }
}

pub unsafe extern "win64" fn vkQueueSubmit(
    queue: usize,
    submit_count: u32,
    submits: *const c_void,
    fence: u64,
) -> i32 {
    if queue == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkQueueSubmit") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize, u32, *const c_void, u64) -> i32 = std::mem::transmute(ptr);
    f(queue, submit_count, submits, fence)
}

pub unsafe extern "win64" fn vkQueueWaitIdle(queue: usize) -> i32 {
    if queue == 0 {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkQueueWaitIdle") else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let f: unsafe extern "C" fn(usize) -> i32 = std::mem::transmute(ptr);
    f(queue)
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceQueueFamilyProperties(
    phys_dev: usize,
    count: *mut u32,
    props: *mut VkQueueFamilyProperties,
) {
    if count.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(count, 0);
        return;
    };
    if phys_dev == 0 {
        std::ptr::write_unaligned(count, 0);
        return;
    }
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceQueueFamilyProperties")
    else {
        std::ptr::write_unaligned(count, 0);
        return;
    };
    let f: PfnVkGetPhysicalDeviceQueueFamilyProperties = std::mem::transmute(ptr);
    let mut c = std::ptr::read_unaligned(count);
    f(phys_dev, &mut c, props);
    std::ptr::write_unaligned(count, c);
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceProperties(phys_dev: usize, props: *mut c_void) {
    if phys_dev == 0 || props.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceProperties") {
        let f: unsafe extern "C" fn(usize, *mut c_void) = std::mem::transmute(ptr);
        f(phys_dev, props);
    }
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceFeatures(phys_dev: usize, features: *mut c_void) {
    if phys_dev == 0 || features.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceFeatures") {
        let f: unsafe extern "C" fn(usize, *mut c_void) = std::mem::transmute(ptr);
        f(phys_dev, features);
    }
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceMemoryProperties(
    phys_dev: usize,
    props: *mut c_void,
) {
    if phys_dev == 0 || props.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceMemoryProperties") {
        let f: unsafe extern "C" fn(usize, *mut c_void) = std::mem::transmute(ptr);
        f(phys_dev, props);
    }
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceFeatures2(phys_dev: usize, features: *mut c_void) {
    if phys_dev == 0 || features.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk
        .any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceFeatures2")
        .or_else(|| unsafe { vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceFeatures2KHR") })
    {
        let f: unsafe extern "C" fn(usize, *mut c_void) = std::mem::transmute(ptr);
        f(phys_dev, features);
    }
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceProperties2(phys_dev: usize, props: *mut c_void) {
    if phys_dev == 0 || props.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk
        .any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceProperties2")
        .or_else(|| unsafe { vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceProperties2KHR") })
    {
        let f: unsafe extern "C" fn(usize, *mut c_void) = std::mem::transmute(ptr);
        f(phys_dev, props);
    }
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    phys_dev: usize,
    queue_family_index: u32,
    surface: u64,
    supported: *mut u32,
) -> i32 {
    if !supported.is_null() {
        std::ptr::write_unaligned(supported, VK_FALSE);
    }
    if phys_dev == 0 || surface == 0 || supported.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceSurfaceSupportKHR") else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, u32, u64, *mut u32) -> i32 = std::mem::transmute(ptr);
    let mut out = VK_FALSE;
    let res = f(phys_dev, queue_family_index, surface, &mut out);
    std::ptr::write_unaligned(supported, out);
    res
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    phys_dev: usize,
    surface: u64,
    capabilities: *mut c_void,
) -> i32 {
    if phys_dev == 0 || surface == 0 || capabilities.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR")
    else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    sync_surface_layer(surface);
    let f: unsafe extern "C" fn(usize, u64, *mut [u32; 13]) -> i32 = std::mem::transmute(ptr);
    let mut caps = [0u32; 13];
    let res = f(phys_dev, surface, &mut caps);
    std::ptr::write_unaligned(capabilities as *mut [u32; 13], caps);
    res
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    phys_dev: usize,
    surface: u64,
    format_count: *mut u32,
    formats: *mut c_void,
) -> i32 {
    if format_count.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(format_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    if phys_dev == 0 || surface == 0 {
        std::ptr::write_unaligned(format_count, 0);
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceSurfaceFormatsKHR") else {
        std::ptr::write_unaligned(format_count, 0);
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, u64, *mut u32, *mut c_void) -> i32 =
        std::mem::transmute(ptr);
    let mut count = std::ptr::read_unaligned(format_count);
    let res = f(phys_dev, surface, &mut count, formats);
    std::ptr::write_unaligned(format_count, count);
    res
}

pub unsafe extern "win64" fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    phys_dev: usize,
    surface: u64,
    mode_count: *mut u32,
    modes: *mut u32,
) -> i32 {
    if mode_count.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(mode_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    if phys_dev == 0 || surface == 0 {
        std::ptr::write_unaligned(mode_count, 0);
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceSurfacePresentModesKHR")
    else {
        std::ptr::write_unaligned(mode_count, 0);
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, u64, *mut u32, *mut u32) -> i32 = std::mem::transmute(ptr);
    let mut count = std::ptr::read_unaligned(mode_count);
    let res = f(phys_dev, surface, &mut count, modes);
    std::ptr::write_unaligned(mode_count, count);
    res
}

pub unsafe extern "win64" fn vkCreateSwapchainKHR(
    device: usize,
    create_info: *const c_void,
    allocator: *const c_void,
    swapchain: *mut u64,
) -> i32 {
    if !swapchain.is_null() {
        std::ptr::write_unaligned(swapchain, 0);
    }
    if device == 0 || create_info.is_null() || swapchain.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkCreateSwapchainKHR") else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let mut info = std::ptr::read_unaligned(create_info as *const VkSwapchainCreateInfoKHR);
    sync_surface_layer(info.surface);

    // Ensure `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` is enabled whenever the surface supports it
    // so on-demand `PORT_CONTROL_DIR` `shot` captures can read back swapchain images natively.
    if info.image_usage & VK_IMAGE_USAGE_TRANSFER_SRC_BIT == 0 {
        let phys = live_devices().lock().unwrap().get(&device).copied().unwrap_or(0);
        if phys != 0 {
            if let Some(caps_ptr) =
                vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR")
            {
                let get_caps: unsafe extern "C" fn(usize, u64, *mut [u32; 13]) -> i32 =
                    std::mem::transmute(caps_ptr);
                let mut caps = [0u32; 13];
                if get_caps(phys, info.surface, &mut caps) == VK_SUCCESS
                    && (caps[12] & VK_IMAGE_USAGE_TRANSFER_SRC_BIT) != 0
                {
                    info.image_usage |= VK_IMAGE_USAGE_TRANSFER_SRC_BIT;
                }
            }
        }
    }

    let f: unsafe extern "C" fn(
        usize,
        *const VkSwapchainCreateInfoKHR,
        *const c_void,
        *mut u64,
    ) -> i32 = std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(device, &info, sanitize_allocator(allocator), &mut out);
    std::ptr::write_unaligned(swapchain, out);
    if res == VK_SUCCESS && out != 0 {
        let hwnd = live_surfaces()
            .lock()
            .unwrap()
            .get(&info.surface)
            .map(|s| s.hwnd)
            .unwrap_or(0);
        live_swapchains().lock().unwrap().insert(
            out,
            SwapchainRecord {
                device,
                hwnd,
                width: info.image_extent_width,
                height: info.image_extent_height,
                format: info.image_format,
                usage: info.image_usage,
                images: Vec::new(),
            },
        );
    }
    res
}

pub unsafe extern "win64" fn vkDestroySwapchainKHR(
    device: usize,
    swapchain: u64,
    allocator: *const c_void,
) {
    if device == 0 || swapchain == 0 {
        return;
    }
    live_swapchains().lock().unwrap().remove(&swapchain);
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr(device, c"vkDestroySwapchainKHR") {
        let f: unsafe extern "C" fn(usize, u64, *const c_void) = std::mem::transmute(ptr);
        f(device, swapchain, sanitize_allocator(allocator));
    }
}

pub unsafe extern "win64" fn vkGetSwapchainImagesKHR(
    device: usize,
    swapchain: u64,
    image_count: *mut u32,
    images: *mut u64,
) -> i32 {
    if image_count.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(image_count, 0);
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    if device == 0 || swapchain == 0 {
        std::ptr::write_unaligned(image_count, 0);
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(ptr) = vk.device_proc_ptr(device, c"vkGetSwapchainImagesKHR") else {
        std::ptr::write_unaligned(image_count, 0);
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, u64, *mut u32, *mut u64) -> i32 = std::mem::transmute(ptr);
    let mut count = std::ptr::read_unaligned(image_count);
    let res = f(device, swapchain, &mut count, images);
    std::ptr::write_unaligned(image_count, count);
    if (res == VK_SUCCESS || res == VK_INCOMPLETE) && !images.is_null() && count > 0 {
        let mut cached = Vec::with_capacity(count as usize);
        for i in 0..count as usize {
            cached.push(std::ptr::read_unaligned(images.add(i)));
        }
        if let Some(rec) = live_swapchains().lock().unwrap().get_mut(&swapchain) {
            rec.images = cached;
        }
    }
    res
}

pub unsafe extern "win64" fn vkAcquireNextImageKHR(
    device: usize,
    swapchain: u64,
    timeout: u64,
    semaphore: u64,
    fence: u64,
    image_index: *mut u32,
) -> i32 {
    if !image_index.is_null() {
        std::ptr::write_unaligned(image_index, 0);
    }
    if device == 0 || swapchain == 0 || image_index.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkAcquireNextImageKHR") else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, u64, u64, u64, u64, *mut u32) -> i32 =
        std::mem::transmute(ptr);
    let mut idx = 0u32;
    let res = f(device, swapchain, timeout, semaphore, fence, &mut idx);
    std::ptr::write_unaligned(image_index, idx);
    res
}

fn f16_to_u8(h: u16) -> u8 {
    let s = if h & 0x8000 != 0 { -1.0f32 } else { 1.0f32 };
    let e = ((h >> 10) & 0x1f) as i32;
    let m = (h & 0x3ff) as f32;
    let f = s * match e {
        0 => m * 2f32.powi(-24),
        31 => f32::INFINITY,
        _ => (1.0 + m / 1024.0) * 2f32.powi(e - 15),
    };
    (f.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

fn vk_format_bpp(format: u32) -> Option<usize> {
    match format {
        VK_FORMAT_R8G8B8A8_UNORM
        | VK_FORMAT_R8G8B8A8_SRGB
        | VK_FORMAT_B8G8R8A8_UNORM
        | VK_FORMAT_B8G8R8A8_SRGB
        | VK_FORMAT_A2B10G10R10_UNORM_PACK32 => Some(4),
        VK_FORMAT_R16G16B16A16_SFLOAT => Some(8),
        _ => None,
    }
}

fn vk_pixels_to_rgb8(format: u32, data: &[u8], w: usize, h: usize) -> Option<Vec<u8>> {
    let bpp = vk_format_bpp(format)?;
    let total = w.checked_mul(h)?;
    if data.len() < total.checked_mul(bpp)? {
        return None;
    }
    let mut out = Vec::with_capacity(total * 3);
    for i in 0..total {
        let px = &data[i * bpp..][..bpp];
        match format {
            VK_FORMAT_R8G8B8A8_UNORM | VK_FORMAT_R8G8B8A8_SRGB => {
                out.extend_from_slice(&[px[0], px[1], px[2]]);
            }
            VK_FORMAT_B8G8R8A8_UNORM | VK_FORMAT_B8G8R8A8_SRGB => {
                out.extend_from_slice(&[px[2], px[1], px[0]]);
            }
            VK_FORMAT_A2B10G10R10_UNORM_PACK32 => {
                let v = u32::from_le_bytes([px[0], px[1], px[2], px[3]]);
                let r10 = v & 0x3ff;
                let g10 = (v >> 10) & 0x3ff;
                let b10 = (v >> 20) & 0x3ff;
                out.extend_from_slice(&[
                    ((r10 * 255 + 511) / 1023) as u8,
                    ((g10 * 255 + 511) / 1023) as u8,
                    ((b10 * 255 + 511) / 1023) as u8,
                ]);
            }
            VK_FORMAT_R16G16B16A16_SFLOAT => {
                let r = f16_to_u8(u16::from_le_bytes([px[0], px[1]]));
                let g = f16_to_u8(u16::from_le_bytes([px[2], px[3]]));
                let b = f16_to_u8(u16::from_le_bytes([px[4], px[5]]));
                out.extend_from_slice(&[r, g, b]);
            }
            _ => return None,
        }
    }
    Some(out)
}

/// Captures the swapchain image about to be presented by `vkQueuePresentKHR` into a PNG file
/// using pure Vulkan transfer commands (`vkCmdCopyImageToBuffer` into a host-visible buffer).
///
/// Returns `Ok(true)` if the capture submitted on `queue` and consumed `present_info`'s
/// `pWaitSemaphores` (so the caller should pass `waitSemaphoreCount = 0` to `vkQueuePresentKHR`),
/// `Ok(false)` if no semaphores were consumed, or `Err(msg)` if capture could not be performed
/// before submission.
unsafe fn capture_vulkan_swapchain_image(
    vk: &VulkanBackend,
    queue: usize,
    swapchain: u64,
    image_index: u32,
    present_info: &VkPresentInfoKHR,
    path: &str,
    frame_num: u64,
) -> Result<bool, String> {
    let mut sc = live_swapchains()
        .lock()
        .unwrap()
        .get(&swapchain)
        .cloned()
        .ok_or_else(|| "unknown swapchain".to_string())?;
    if sc.width == 0 || sc.height == 0 {
        return Err("zero-sized swapchain".into());
    }
    if sc.usage & VK_IMAGE_USAGE_TRANSFER_SRC_BIT == 0 {
        return Err("swapchain lacks VK_IMAGE_USAGE_TRANSFER_SRC_BIT".into());
    }
    let bpp = vk_format_bpp(sc.format)
        .ok_or_else(|| format!("unsupported Vulkan swapchain format {}", sc.format))?;

    let device = sc.device;
    let phys = live_devices()
        .lock()
        .unwrap()
        .get(&device)
        .copied()
        .ok_or_else(|| "unknown VkPhysicalDevice for swapchain device".to_string())?;
    let queue_family = live_queues()
        .lock()
        .unwrap()
        .get(&queue)
        .map(|q| q.family_index)
        .unwrap_or(0);

    // Lazily populate swapchain images if the app queried them before interception.
    if sc.images.get(image_index as usize).copied().unwrap_or(0) == 0 {
        let get_imgs_ptr = vk
            .device_proc_ptr(device, c"vkGetSwapchainImagesKHR")
            .ok_or("missing vkGetSwapchainImagesKHR")?;
        let get_imgs: unsafe extern "C" fn(usize, u64, *mut u32, *mut u64) -> i32 =
            std::mem::transmute(get_imgs_ptr);
        let mut n = 0u32;
        if get_imgs(device, swapchain, &mut n, std::ptr::null_mut()) == VK_SUCCESS && n > 0 {
            let mut imgs = vec![0u64; n as usize];
            if get_imgs(device, swapchain, &mut n, imgs.as_mut_ptr()) == VK_SUCCESS {
                imgs.truncate(n as usize);
                sc.images = imgs.clone();
                if let Some(entry) = live_swapchains().lock().unwrap().get_mut(&swapchain) {
                    entry.images = imgs;
                }
            }
        }
    }
    let image = sc
        .images
        .get(image_index as usize)
        .copied()
        .filter(|&img| img != 0)
        .ok_or_else(|| format!("missing VkImage for swapchain image_index={image_index}"))?;

    macro_rules! dev_proc {
        ($sym:expr, $ty:ty) => {{
            let p = vk
                .device_proc_ptr(device, $sym)
                .ok_or_else(|| format!("missing {:?}", $sym))?;
            std::mem::transmute::<*const c_void, $ty>(p)
        }};
    }

    let get_mem_props_ptr = vk
        .any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceMemoryProperties")
        .ok_or("missing vkGetPhysicalDeviceMemoryProperties")?;
    let get_mem_props: unsafe extern "C" fn(usize, *mut VkPhysicalDeviceMemoryProperties) =
        std::mem::transmute(get_mem_props_ptr);

    let create_buf = dev_proc!(
        c"vkCreateBuffer",
        unsafe extern "C" fn(usize, *const VkBufferCreateInfo, *const c_void, *mut u64) -> i32
    );
    let destroy_buf = dev_proc!(
        c"vkDestroyBuffer",
        unsafe extern "C" fn(usize, u64, *const c_void)
    );
    let get_buf_reqs = dev_proc!(
        c"vkGetBufferMemoryRequirements",
        unsafe extern "C" fn(usize, u64, *mut VkMemoryRequirements)
    );
    let alloc_mem = dev_proc!(
        c"vkAllocateMemory",
        unsafe extern "C" fn(usize, *const VkMemoryAllocateInfo, *const c_void, *mut u64) -> i32
    );
    let free_mem = dev_proc!(
        c"vkFreeMemory",
        unsafe extern "C" fn(usize, u64, *const c_void)
    );
    let bind_buf_mem = dev_proc!(
        c"vkBindBufferMemory",
        unsafe extern "C" fn(usize, u64, u64, u64) -> i32
    );
    let map_mem = dev_proc!(
        c"vkMapMemory",
        unsafe extern "C" fn(usize, u64, u64, u64, u32, *mut *mut c_void) -> i32
    );
    let unmap_mem = dev_proc!(c"vkUnmapMemory", unsafe extern "C" fn(usize, u64));
    let create_pool = dev_proc!(
        c"vkCreateCommandPool",
        unsafe extern "C" fn(usize, *const VkCommandPoolCreateInfo, *const c_void, *mut u64) -> i32
    );
    let destroy_pool = dev_proc!(
        c"vkDestroyCommandPool",
        unsafe extern "C" fn(usize, u64, *const c_void)
    );
    let alloc_cmd = dev_proc!(
        c"vkAllocateCommandBuffers",
        unsafe extern "C" fn(usize, *const VkCommandBufferAllocateInfo, *mut usize) -> i32
    );
    let begin_cmd = dev_proc!(
        c"vkBeginCommandBuffer",
        unsafe extern "C" fn(usize, *const VkCommandBufferBeginInfo) -> i32
    );
    let end_cmd = dev_proc!(c"vkEndCommandBuffer", unsafe extern "C" fn(usize) -> i32);
    let cmd_barrier = dev_proc!(
        c"vkCmdPipelineBarrier",
        unsafe extern "C" fn(
            usize,
            u32,
            u32,
            u32,
            u32,
            *const c_void,
            u32,
            *const c_void,
            u32,
            *const VkImageMemoryBarrier,
        )
    );
    let cmd_copy = dev_proc!(
        c"vkCmdCopyImageToBuffer",
        unsafe extern "C" fn(usize, u64, u32, u64, u32, *const VkBufferImageCopy)
    );
    let create_fence = dev_proc!(
        c"vkCreateFence",
        unsafe extern "C" fn(usize, *const VkFenceCreateInfo, *const c_void, *mut u64) -> i32
    );
    let destroy_fence = dev_proc!(
        c"vkDestroyFence",
        unsafe extern "C" fn(usize, u64, *const c_void)
    );
    let wait_fences = dev_proc!(
        c"vkWaitForFences",
        unsafe extern "C" fn(usize, u32, *const u64, u32, u64) -> i32
    );
    let queue_submit = dev_proc!(
        c"vkQueueSubmit",
        unsafe extern "C" fn(usize, u32, *const VkSubmitInfo, u64) -> i32
    );

    let byte_len = (sc.width as u64) * (sc.height as u64) * (bpp as u64);
    let buf_ci = VkBufferCreateInfo {
        s_type: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: 0,
        size: byte_len,
        usage: VK_BUFFER_USAGE_TRANSFER_DST_BIT,
        sharing_mode: 0,
        queue_family_index_count: 0,
        p_queue_family_indices: std::ptr::null(),
    };
    let mut buf = 0u64;
    let r = create_buf(device, &buf_ci, std::ptr::null(), &mut buf);
    if r != VK_SUCCESS || buf == 0 {
        return Err(format!("vkCreateBuffer failed ({r})"));
    }

    let mut reqs = VkMemoryRequirements::default();
    get_buf_reqs(device, buf, &mut reqs);
    let mut mem_props: VkPhysicalDeviceMemoryProperties = std::mem::zeroed();
    get_mem_props(phys, &mut mem_props);
    let want = VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
    let mem_type = (0..mem_props.memory_type_count.min(32))
        .find(|&i| {
            (reqs.memory_type_bits & (1u32 << i)) != 0
                && (mem_props.memory_types[i as usize].property_flags & want) == want
        })
        .ok_or_else(|| {
            destroy_buf(device, buf, std::ptr::null());
            "no HOST_VISIBLE|HOST_COHERENT memory type".to_string()
        })?;

    let alloc_info = VkMemoryAllocateInfo {
        s_type: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        allocation_size: reqs.size.max(byte_len),
        memory_type_index: mem_type,
    };
    let mut mem = 0u64;
    let r = alloc_mem(device, &alloc_info, std::ptr::null(), &mut mem);
    if r != VK_SUCCESS || mem == 0 {
        destroy_buf(device, buf, std::ptr::null());
        return Err(format!("vkAllocateMemory failed ({r})"));
    }
    let r = bind_buf_mem(device, buf, mem, 0);
    if r != VK_SUCCESS {
        free_mem(device, mem, std::ptr::null());
        destroy_buf(device, buf, std::ptr::null());
        return Err(format!("vkBindBufferMemory failed ({r})"));
    }

    let pool_ci = VkCommandPoolCreateInfo {
        s_type: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: 0,
        queue_family_index: queue_family,
    };
    let mut pool = 0u64;
    let r = create_pool(device, &pool_ci, std::ptr::null(), &mut pool);
    if r != VK_SUCCESS || pool == 0 {
        free_mem(device, mem, std::ptr::null());
        destroy_buf(device, buf, std::ptr::null());
        return Err(format!("vkCreateCommandPool failed ({r})"));
    }

    let cmd_ai = VkCommandBufferAllocateInfo {
        s_type: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        command_pool: pool,
        level: 0,
        command_buffer_count: 1,
    };
    let mut cmd = 0usize;
    let r = alloc_cmd(device, &cmd_ai, &mut cmd);
    if r != VK_SUCCESS || cmd == 0 {
        destroy_pool(device, pool, std::ptr::null());
        free_mem(device, mem, std::ptr::null());
        destroy_buf(device, buf, std::ptr::null());
        return Err(format!("vkAllocateCommandBuffers failed ({r})"));
    }

    let fence_ci = VkFenceCreateInfo {
        s_type: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: 0,
    };
    let mut fence = 0u64;
    let r = create_fence(device, &fence_ci, std::ptr::null(), &mut fence);
    if r != VK_SUCCESS || fence == 0 {
        destroy_pool(device, pool, std::ptr::null());
        free_mem(device, mem, std::ptr::null());
        destroy_buf(device, buf, std::ptr::null());
        return Err(format!("vkCreateFence failed ({r})"));
    }

    let begin_ci = VkCommandBufferBeginInfo {
        s_type: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        p_next: std::ptr::null(),
        flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        p_inheritance_info: std::ptr::null(),
    };
    let r = begin_cmd(cmd, &begin_ci);
    if r != VK_SUCCESS {
        destroy_fence(device, fence, std::ptr::null());
        destroy_pool(device, pool, std::ptr::null());
        free_mem(device, mem, std::ptr::null());
        destroy_buf(device, buf, std::ptr::null());
        return Err(format!("vkBeginCommandBuffer failed ({r})"));
    }

    let to_transfer = VkImageMemoryBarrier {
        s_type: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
        p_next: std::ptr::null(),
        src_access_mask: 0,
        dst_access_mask: VK_ACCESS_TRANSFER_READ_BIT,
        old_layout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        new_layout: VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        src_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
        dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
        image,
        aspect_mask: VK_IMAGE_ASPECT_COLOR_BIT,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    };
    cmd_barrier(
        cmd,
        VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
        VK_PIPELINE_STAGE_TRANSFER_BIT,
        0,
        0,
        std::ptr::null(),
        0,
        std::ptr::null(),
        1,
        &to_transfer,
    );

    let copy = VkBufferImageCopy {
        buffer_offset: 0,
        buffer_row_length: 0,
        buffer_image_height: 0,
        aspect_mask: VK_IMAGE_ASPECT_COLOR_BIT,
        mip_level: 0,
        base_array_layer: 0,
        layer_count: 1,
        offset_x: 0,
        offset_y: 0,
        offset_z: 0,
        extent_width: sc.width,
        extent_height: sc.height,
        extent_depth: 1,
    };
    cmd_copy(cmd, image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, buf, 1, &copy);

    let to_present = VkImageMemoryBarrier {
        src_access_mask: VK_ACCESS_TRANSFER_READ_BIT,
        dst_access_mask: 0,
        old_layout: VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        new_layout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        ..to_transfer
    };
    cmd_barrier(
        cmd,
        VK_PIPELINE_STAGE_TRANSFER_BIT,
        VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
        0,
        0,
        std::ptr::null(),
        0,
        std::ptr::null(),
        1,
        &to_present,
    );

    let r = end_cmd(cmd);
    if r != VK_SUCCESS {
        destroy_fence(device, fence, std::ptr::null());
        destroy_pool(device, pool, std::ptr::null());
        free_mem(device, mem, std::ptr::null());
        destroy_buf(device, buf, std::ptr::null());
        return Err(format!("vkEndCommandBuffer failed ({r})"));
    }

    let wait_count = if present_info.p_wait_semaphores.is_null() {
        0
    } else {
        present_info.wait_semaphore_count
    };
    let wait_stages = vec![VK_PIPELINE_STAGE_TRANSFER_BIT; wait_count as usize];
    let submit = VkSubmitInfo {
        s_type: VK_STRUCTURE_TYPE_SUBMIT_INFO,
        p_next: std::ptr::null(),
        wait_semaphore_count: wait_count,
        p_wait_semaphores: if wait_count > 0 {
            present_info.p_wait_semaphores
        } else {
            std::ptr::null()
        },
        p_wait_dst_stage_mask: if wait_count > 0 {
            wait_stages.as_ptr()
        } else {
            std::ptr::null()
        },
        command_buffer_count: 1,
        p_command_buffers: &cmd,
        signal_semaphore_count: 0,
        p_signal_semaphores: std::ptr::null(),
    };

    let submit_res = queue_submit(queue, 1, &submit, fence);
    let consumed_semaphores = submit_res == VK_SUCCESS && wait_count > 0;
    if submit_res == VK_SUCCESS {
        let wait_res = wait_fences(device, 1, &fence, VK_TRUE, u64::MAX);
        if wait_res == VK_SUCCESS {
            let mut mapped: *mut c_void = std::ptr::null_mut();
            if map_mem(device, mem, 0, byte_len, 0, &mut mapped) == VK_SUCCESS && !mapped.is_null() {
                let bytes = std::slice::from_raw_parts(mapped as *const u8, byte_len as usize);
                match vk_pixels_to_rgb8(sc.format, bytes, sc.width as usize, sc.height as usize) {
                    Some(rgb) => match d3d12_metal::metal::capture::write_png(
                        path, sc.width, sc.height, &rgb,
                    ) {
                        Ok(()) => log::info!(
                            target: "vulkan",
                            "vulkan present #{frame_num}: captured back buffer (fmt={} {}x{}) -> {path}",
                            sc.format,
                            sc.width,
                            sc.height
                        ),
                        Err(e) => log::warn!(
                            target: "vulkan",
                            "vulkan present #{frame_num}: writing {path}: {e}"
                        ),
                    },
                    None => log::warn!(
                        target: "vulkan",
                        "vulkan present #{frame_num}: pixel conversion failed for format {}",
                        sc.format
                    ),
                }
                unmap_mem(device, mem);
            }
        } else {
            log::warn!(
                target: "vulkan",
                "vulkan present #{frame_num}: capture vkWaitForFences failed ({wait_res})"
            );
        }
    } else {
        log::warn!(
            target: "vulkan",
            "vulkan present #{frame_num}: capture vkQueueSubmit failed ({submit_res})"
        );
    }

    destroy_fence(device, fence, std::ptr::null());
    destroy_pool(device, pool, std::ptr::null());
    free_mem(device, mem, std::ptr::null());
    destroy_buf(device, buf, std::ptr::null());
    Ok(consumed_semaphores)
}

pub unsafe extern "win64" fn vkQueuePresentKHR(queue: usize, present_info: *const c_void) -> i32 {
    if queue == 0 || present_info.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(c"vkQueuePresentKHR") else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };

    let info = std::ptr::read_unaligned(present_info as *const VkPresentInfoKHR);
    let primary_sc = if info.swapchain_count > 0
        && !info.p_swapchains.is_null()
        && !info.p_image_indices.is_null()
    {
        Some((
            std::ptr::read_unaligned(info.p_swapchains),
            std::ptr::read_unaligned(info.p_image_indices),
        ))
    } else {
        None
    };

    let hwnd = primary_sc
        .and_then(|(sc, _)| live_swapchains().lock().unwrap().get(&sc).map(|r| r.hwnd))
        .unwrap_or(0);

    let next_frame = (crate::present::present_count() as u64) + 1;
    let capture_path = d3d12_metal::metal::capture::take_pending().or_else(|| {
        d3d12_metal::metal::capture::config()
            .filter(|(_, at)| at.contains(&next_frame))
            .map(|(base, _)| format!("{base}_{next_frame}.png"))
    });

    let mut patched_info = info;
    let mut use_patched = false;
    if let (Some(path), Some((sc, img_idx))) = (capture_path, primary_sc) {
        match capture_vulkan_swapchain_image(vk, queue, sc, img_idx, &info, &path, next_frame) {
            Ok(true) => {
                patched_info.wait_semaphore_count = 0;
                patched_info.p_wait_semaphores = std::ptr::null();
                use_patched = true;
            }
            Ok(false) => {}
            Err(e) => {
                log::warn!(target: "vulkan", "vulkan present #{next_frame}: capture skipped ({e})");
            }
        }
    }

    let t0 = crate::hitch::span_begin();
    let present_scope =
        crate::hitch::Scope::enter(crate::hitch::ActKind::Present, info.swapchain_count as u64);
    let f: unsafe extern "C" fn(usize, *const c_void) -> i32 = std::mem::transmute(ptr);
    let res = if use_patched {
        f(queue, &patched_info as *const VkPresentInfoKHR as *const c_void)
    } else {
        f(queue, present_info)
    };
    let present_ns = present_scope.elapsed_ns();
    drop(present_scope);
    crate::hitch::span_end(t0, crate::hitch::SPAN_PRESENT);

    if res == VK_SUCCESS || res == VK_SUBOPTIMAL_KHR {
        let n = crate::present::on_present(crate::present::PresentApi::Vulkan, hwnd, present_ns);
        if n <= 10 || n % 60 == 0 {
            log::info!(
                target: "vulkan",
                "vulkan Present #{n} (result {res}, swapchains {})",
                info.swapchain_count
            );
        }
    }
    res
}

// ---------------------------------------------------------------------------
// Vulkan 1.2 / 1.3 & KHR forwarding entrypoints (dynamic rendering,
// synchronization2, timeline semaphores, buffer device address, copy_commands2,
// maintenance4). All structs are fixed-width C ABI identical between Win64 and
// ARM64; entrypoints take 1..4 integer/pointer/handle args with no allocators.
// ---------------------------------------------------------------------------

unsafe fn forward_cmd_1ptr(name: &CStr, cmd_buf: usize, info: *const c_void) {
    if cmd_buf == 0 || info.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(name) {
        let f: unsafe extern "C" fn(usize, *const c_void) = std::mem::transmute(ptr);
        f(cmd_buf, info);
    }
}

unsafe fn forward_device_u64_query(name: &CStr, device: usize, info: *const c_void) -> u64 {
    if device == 0 || info.is_null() {
        return 0;
    }
    let Some(vk) = vulkan_backend() else {
        return 0;
    };
    let Some(ptr) = vk.device_proc_ptr_with_aliases(device, name) else {
        return 0;
    };
    let f: unsafe extern "C" fn(usize, *const c_void) -> u64 = std::mem::transmute(ptr);
    f(device, info)
}

unsafe fn forward_device_mem_reqs(
    name: &CStr,
    device: usize,
    info: *const c_void,
    mem_reqs: *mut c_void,
) {
    if device == 0 || info.is_null() || mem_reqs.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr_with_aliases(device, name) {
        let f: unsafe extern "C" fn(usize, *const c_void, *mut c_void) = std::mem::transmute(ptr);
        f(device, info, mem_reqs);
    }
}

unsafe fn forward_device_sparse_mem_reqs(
    name: &CStr,
    device: usize,
    info: *const c_void,
    count: *mut u32,
    mem_reqs: *mut c_void,
) {
    if count.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        std::ptr::write_unaligned(count, 0);
        return;
    };
    if device == 0 || info.is_null() {
        std::ptr::write_unaligned(count, 0);
        return;
    }
    let Some(ptr) = vk.device_proc_ptr_with_aliases(device, name) else {
        std::ptr::write_unaligned(count, 0);
        return;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *mut u32, *mut c_void) =
        std::mem::transmute(ptr);
    let mut c = std::ptr::read_unaligned(count);
    f(device, info, &mut c, mem_reqs);
    std::ptr::write_unaligned(count, c);
}

// -- VK_KHR_dynamic_rendering / Vulkan 1.3 --

pub unsafe extern "win64" fn vkCmdBeginRendering(cmd_buf: usize, rendering_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdBeginRendering", cmd_buf, rendering_info);
}

pub unsafe extern "win64" fn vkCmdBeginRenderingKHR(cmd_buf: usize, rendering_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdBeginRenderingKHR", cmd_buf, rendering_info);
}

pub unsafe extern "win64" fn vkCmdEndRendering(cmd_buf: usize) {
    if cmd_buf == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(c"vkCmdEndRendering") {
        let f: unsafe extern "C" fn(usize) = std::mem::transmute(ptr);
        f(cmd_buf);
    }
}

pub unsafe extern "win64" fn vkCmdEndRenderingKHR(cmd_buf: usize) {
    if cmd_buf == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(c"vkCmdEndRenderingKHR") {
        let f: unsafe extern "C" fn(usize) = std::mem::transmute(ptr);
        f(cmd_buf);
    }
}

// -- VK_KHR_synchronization2 / Vulkan 1.3 --

pub unsafe extern "win64" fn vkCmdPipelineBarrier2(cmd_buf: usize, dep_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdPipelineBarrier2", cmd_buf, dep_info);
}

pub unsafe extern "win64" fn vkCmdPipelineBarrier2KHR(cmd_buf: usize, dep_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdPipelineBarrier2KHR", cmd_buf, dep_info);
}

unsafe fn forward_cmd_set_event2(
    name: &CStr,
    cmd_buf: usize,
    event: u64,
    dep_info: *const c_void,
) {
    if cmd_buf == 0 || dep_info.is_null() {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(name) {
        let f: unsafe extern "C" fn(usize, u64, *const c_void) = std::mem::transmute(ptr);
        f(cmd_buf, event, dep_info);
    }
}

pub unsafe extern "win64" fn vkCmdSetEvent2(cmd_buf: usize, event: u64, dep_info: *const c_void) {
    forward_cmd_set_event2(c"vkCmdSetEvent2", cmd_buf, event, dep_info);
}

pub unsafe extern "win64" fn vkCmdSetEvent2KHR(
    cmd_buf: usize,
    event: u64,
    dep_info: *const c_void,
) {
    forward_cmd_set_event2(c"vkCmdSetEvent2KHR", cmd_buf, event, dep_info);
}

unsafe fn forward_cmd_reset_event2(name: &CStr, cmd_buf: usize, event: u64, stage_mask: u64) {
    if cmd_buf == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(name) {
        let f: unsafe extern "C" fn(usize, u64, u64) = std::mem::transmute(ptr);
        f(cmd_buf, event, stage_mask);
    }
}

pub unsafe extern "win64" fn vkCmdResetEvent2(cmd_buf: usize, event: u64, stage_mask: u64) {
    forward_cmd_reset_event2(c"vkCmdResetEvent2", cmd_buf, event, stage_mask);
}

pub unsafe extern "win64" fn vkCmdResetEvent2KHR(cmd_buf: usize, event: u64, stage_mask: u64) {
    forward_cmd_reset_event2(c"vkCmdResetEvent2KHR", cmd_buf, event, stage_mask);
}

unsafe fn forward_cmd_wait_events2(
    name: &CStr,
    cmd_buf: usize,
    event_count: u32,
    events: *const u64,
    dep_infos: *const c_void,
) {
    if cmd_buf == 0 || (event_count > 0 && (events.is_null() || dep_infos.is_null())) {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(name) {
        let f: unsafe extern "C" fn(usize, u32, *const u64, *const c_void) =
            std::mem::transmute(ptr);
        f(cmd_buf, event_count, events, dep_infos);
    }
}

pub unsafe extern "win64" fn vkCmdWaitEvents2(
    cmd_buf: usize,
    event_count: u32,
    events: *const u64,
    dep_infos: *const c_void,
) {
    forward_cmd_wait_events2(c"vkCmdWaitEvents2", cmd_buf, event_count, events, dep_infos);
}

pub unsafe extern "win64" fn vkCmdWaitEvents2KHR(
    cmd_buf: usize,
    event_count: u32,
    events: *const u64,
    dep_infos: *const c_void,
) {
    forward_cmd_wait_events2(
        c"vkCmdWaitEvents2KHR",
        cmd_buf,
        event_count,
        events,
        dep_infos,
    );
}

unsafe fn forward_cmd_write_timestamp2(
    name: &CStr,
    cmd_buf: usize,
    stage: u64,
    query_pool: u64,
    query: u32,
) {
    if cmd_buf == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(name) {
        let f: unsafe extern "C" fn(usize, u64, u64, u32) = std::mem::transmute(ptr);
        f(cmd_buf, stage, query_pool, query);
    }
}

pub unsafe extern "win64" fn vkCmdWriteTimestamp2(
    cmd_buf: usize,
    stage: u64,
    query_pool: u64,
    query: u32,
) {
    forward_cmd_write_timestamp2(c"vkCmdWriteTimestamp2", cmd_buf, stage, query_pool, query);
}

pub unsafe extern "win64" fn vkCmdWriteTimestamp2KHR(
    cmd_buf: usize,
    stage: u64,
    query_pool: u64,
    query: u32,
) {
    forward_cmd_write_timestamp2(
        c"vkCmdWriteTimestamp2KHR",
        cmd_buf,
        stage,
        query_pool,
        query,
    );
}

unsafe fn forward_queue_submit2(
    name: &CStr,
    queue: usize,
    submit_count: u32,
    submits: *const c_void,
    fence: u64,
) -> i32 {
    if queue == 0 || (submit_count > 0 && submits.is_null()) {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr(name) else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, u32, *const c_void, u64) -> i32 = std::mem::transmute(ptr);
    f(queue, submit_count, submits, fence)
}

pub unsafe extern "win64" fn vkQueueSubmit2(
    queue: usize,
    submit_count: u32,
    submits: *const c_void,
    fence: u64,
) -> i32 {
    forward_queue_submit2(c"vkQueueSubmit2", queue, submit_count, submits, fence)
}

pub unsafe extern "win64" fn vkQueueSubmit2KHR(
    queue: usize,
    submit_count: u32,
    submits: *const c_void,
    fence: u64,
) -> i32 {
    forward_queue_submit2(c"vkQueueSubmit2KHR", queue, submit_count, submits, fence)
}

// -- VK_KHR_timeline_semaphore / Vulkan 1.2 --

pub unsafe extern "win64" fn vkGetSemaphoreCounterValueKHR(
    device: usize,
    semaphore: u64,
    value: *mut u64,
) -> i32 {
    vkGetSemaphoreCounterValue(device, semaphore, value)
}

pub unsafe extern "win64" fn vkWaitSemaphoresKHR(
    device: usize,
    wait_info: *const c_void,
    timeout: u64,
) -> i32 {
    vkWaitSemaphores(device, wait_info, timeout)
}

unsafe fn forward_signal_semaphore(
    name: &CStr,
    device: usize,
    signal_info: *const c_void,
) -> i32 {
    if device == 0 || signal_info.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr_with_aliases(device, name) else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, *const c_void) -> i32 = std::mem::transmute(ptr);
    f(device, signal_info)
}

pub unsafe extern "win64" fn vkSignalSemaphore(
    device: usize,
    signal_info: *const c_void,
) -> i32 {
    forward_signal_semaphore(c"vkSignalSemaphore", device, signal_info)
}

pub unsafe extern "win64" fn vkSignalSemaphoreKHR(
    device: usize,
    signal_info: *const c_void,
) -> i32 {
    forward_signal_semaphore(c"vkSignalSemaphoreKHR", device, signal_info)
}

// -- VK_KHR_buffer_device_address / Vulkan 1.2 --

pub unsafe extern "win64" fn vkGetBufferDeviceAddress(
    device: usize,
    info: *const c_void,
) -> u64 {
    forward_device_u64_query(c"vkGetBufferDeviceAddress", device, info)
}

pub unsafe extern "win64" fn vkGetBufferDeviceAddressKHR(
    device: usize,
    info: *const c_void,
) -> u64 {
    forward_device_u64_query(c"vkGetBufferDeviceAddressKHR", device, info)
}

pub unsafe extern "win64" fn vkGetBufferDeviceAddressEXT(
    device: usize,
    info: *const c_void,
) -> u64 {
    forward_device_u64_query(c"vkGetBufferDeviceAddressEXT", device, info)
}

pub unsafe extern "win64" fn vkGetBufferOpaqueCaptureAddress(
    device: usize,
    info: *const c_void,
) -> u64 {
    forward_device_u64_query(c"vkGetBufferOpaqueCaptureAddress", device, info)
}

pub unsafe extern "win64" fn vkGetBufferOpaqueCaptureAddressKHR(
    device: usize,
    info: *const c_void,
) -> u64 {
    forward_device_u64_query(c"vkGetBufferOpaqueCaptureAddressKHR", device, info)
}

pub unsafe extern "win64" fn vkGetDeviceMemoryOpaqueCaptureAddress(
    device: usize,
    info: *const c_void,
) -> u64 {
    forward_device_u64_query(c"vkGetDeviceMemoryOpaqueCaptureAddress", device, info)
}

pub unsafe extern "win64" fn vkGetDeviceMemoryOpaqueCaptureAddressKHR(
    device: usize,
    info: *const c_void,
) -> u64 {
    forward_device_u64_query(c"vkGetDeviceMemoryOpaqueCaptureAddressKHR", device, info)
}

// -- VK_KHR_copy_commands2 / Vulkan 1.3 --

pub unsafe extern "win64" fn vkCmdCopyBuffer2(cmd_buf: usize, copy_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdCopyBuffer2", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdCopyBuffer2KHR(cmd_buf: usize, copy_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdCopyBuffer2KHR", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdCopyImage2(cmd_buf: usize, copy_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdCopyImage2", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdCopyImage2KHR(cmd_buf: usize, copy_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdCopyImage2KHR", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdCopyBufferToImage2(cmd_buf: usize, copy_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdCopyBufferToImage2", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdCopyBufferToImage2KHR(
    cmd_buf: usize,
    copy_info: *const c_void,
) {
    forward_cmd_1ptr(c"vkCmdCopyBufferToImage2KHR", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdCopyImageToBuffer2(cmd_buf: usize, copy_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdCopyImageToBuffer2", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdCopyImageToBuffer2KHR(
    cmd_buf: usize,
    copy_info: *const c_void,
) {
    forward_cmd_1ptr(c"vkCmdCopyImageToBuffer2KHR", cmd_buf, copy_info);
}

pub unsafe extern "win64" fn vkCmdBlitImage2(cmd_buf: usize, blit_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdBlitImage2", cmd_buf, blit_info);
}

pub unsafe extern "win64" fn vkCmdBlitImage2KHR(cmd_buf: usize, blit_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdBlitImage2KHR", cmd_buf, blit_info);
}

pub unsafe extern "win64" fn vkCmdResolveImage2(cmd_buf: usize, resolve_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdResolveImage2", cmd_buf, resolve_info);
}

pub unsafe extern "win64" fn vkCmdResolveImage2KHR(cmd_buf: usize, resolve_info: *const c_void) {
    forward_cmd_1ptr(c"vkCmdResolveImage2KHR", cmd_buf, resolve_info);
}

// -- VK_KHR_maintenance4 / Vulkan 1.3 --

pub unsafe extern "win64" fn vkGetDeviceBufferMemoryRequirements(
    device: usize,
    info: *const c_void,
    mem_reqs: *mut c_void,
) {
    forward_device_mem_reqs(c"vkGetDeviceBufferMemoryRequirements", device, info, mem_reqs);
}

pub unsafe extern "win64" fn vkGetDeviceBufferMemoryRequirementsKHR(
    device: usize,
    info: *const c_void,
    mem_reqs: *mut c_void,
) {
    forward_device_mem_reqs(
        c"vkGetDeviceBufferMemoryRequirementsKHR",
        device,
        info,
        mem_reqs,
    );
}

pub unsafe extern "win64" fn vkGetDeviceImageMemoryRequirements(
    device: usize,
    info: *const c_void,
    mem_reqs: *mut c_void,
) {
    forward_device_mem_reqs(c"vkGetDeviceImageMemoryRequirements", device, info, mem_reqs);
}

pub unsafe extern "win64" fn vkGetDeviceImageMemoryRequirementsKHR(
    device: usize,
    info: *const c_void,
    mem_reqs: *mut c_void,
) {
    forward_device_mem_reqs(
        c"vkGetDeviceImageMemoryRequirementsKHR",
        device,
        info,
        mem_reqs,
    );
}

pub unsafe extern "win64" fn vkGetDeviceImageSparseMemoryRequirements(
    device: usize,
    info: *const c_void,
    count: *mut u32,
    mem_reqs: *mut c_void,
) {
    forward_device_sparse_mem_reqs(
        c"vkGetDeviceImageSparseMemoryRequirements",
        device,
        info,
        count,
        mem_reqs,
    );
}

pub unsafe extern "win64" fn vkGetDeviceImageSparseMemoryRequirementsKHR(
    device: usize,
    info: *const c_void,
    count: *mut u32,
    mem_reqs: *mut c_void,
) {
    forward_device_sparse_mem_reqs(
        c"vkGetDeviceImageSparseMemoryRequirementsKHR",
        device,
        info,
        count,
        mem_reqs,
    );
}

// ---------------------------------------------------------------------------
// Win64 -> ARM64 AAPCS64 float-argument ABI adapters
//
// In Win64, a `VkCommandBuffer` in slot 1 occupies `rcx` (`x0`) and subsequent
// `float` arguments occupy positional `xmm1..xmm3` (`v1..v3`), NOT `xmm0..xmm2`.
// Native ARM64 MoltenVK (`extern "C"`) reads its first float arguments from
// `s0..s2` (`v0..v2`). `crate::win64_bridge!` unpacks Win64 GPR/XMM slots on
// `aarch64` and forwards typed `f32` arguments straight to MoltenVK.
// ---------------------------------------------------------------------------

macro_rules! vk_cmd_float_bridge {
    (
        export $export_name:ident => $impl_fn:ident($sym:literal; $($arg:ident : $ty:ty),+ $(,)?)
    ) => {
        vk_cmd_float_bridge!($impl_fn($sym; $($arg : $ty),+));
        #[cfg(target_arch = "aarch64")]
        pub(crate) use $impl_fn::bridge as $export_name;
        #[cfg(not(target_arch = "aarch64"))]
        pub(crate) use $impl_fn as $export_name;
    };
    (
        $impl_fn:ident($sym:literal; $($arg:ident : $ty:ty),+ $(,)?)
    ) => {
        pub(crate) unsafe extern "win64" fn $impl_fn(cmd_buf: usize, $($arg: $ty),+) {
            if cmd_buf == 0 {
                return;
            }
            let Some(vk) = vulkan_backend() else {
                return;
            };
            if let Some(ptr) = vk.any_device_or_instance_or_dlsym_ptr($sym) {
                let f: unsafe extern "C" fn(usize, $($ty),+) = std::mem::transmute(ptr);
                f(cmd_buf, $($arg),+);
            }
        }
        crate::win64_bridge!(fn $impl_fn(usize, $($ty),+) -> ());
    };
}

// @exports vkCmdSetLineWidth vkCmdSetDepthBias vkCmdSetDepthBounds
vk_cmd_float_bridge!(export vkCmdSetLineWidth => cmd_set_line_width_impl(
    c"vkCmdSetLineWidth";
    line_width: f32,
));
vk_cmd_float_bridge!(export vkCmdSetDepthBias => cmd_set_depth_bias_impl(
    c"vkCmdSetDepthBias";
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
));
vk_cmd_float_bridge!(export vkCmdSetDepthBounds => cmd_set_depth_bounds_impl(
    c"vkCmdSetDepthBounds";
    min_depth_bounds: f32,
    max_depth_bounds: f32,
));
vk_cmd_float_bridge!(cmd_set_extra_primitive_overestimation_size_ext_impl(
    c"vkCmdSetExtraPrimitiveOverestimationSizeEXT";
    size: f32,
));
#[cfg(target_arch = "aarch64")]
use cmd_set_extra_primitive_overestimation_size_ext_impl::bridge as tramp_vkCmdSetExtraPrimitiveOverestimationSizeEXT;
#[cfg(not(target_arch = "aarch64"))]
use cmd_set_extra_primitive_overestimation_size_ext_impl as tramp_vkCmdSetExtraPrimitiveOverestimationSizeEXT;

// ---------------------------------------------------------------------------
// Dynamic proc-addr interceptors (surface sync + VkAllocationCallbacks safety)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn tramp_vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    phys_dev: usize,
    surface_info: *const c_void,
    capabilities: *mut c_void,
) -> i32 {
    if phys_dev == 0 || surface_info.is_null() || capabilities.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.any_instance_or_dlsym_ptr(c"vkGetPhysicalDeviceSurfaceCapabilities2KHR")
    else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    // VkPhysicalDeviceSurfaceInfo2KHR::surface sits at byte offset 16 (after sType[4]+pad[4], pNext[8]).
    let surface = std::ptr::read_unaligned((surface_info as *const u8).add(16) as *const u64);
    sync_surface_layer(surface);
    let f: unsafe extern "C" fn(usize, *const c_void, *mut c_void) -> i32 =
        std::mem::transmute(ptr);
    f(phys_dev, surface_info, capabilities)
}

unsafe extern "win64" fn tramp_vkCreateHeadlessSurfaceEXT(
    instance: usize,
    create_info: *const c_void,
    allocator: *const c_void,
    surface: *mut u64,
) -> i32 {
    if !surface.is_null() {
        std::ptr::write_unaligned(surface, 0);
    }
    if instance == 0 || create_info.is_null() || surface.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.instance_proc_ptr(instance, c"vkCreateHeadlessSurfaceEXT") else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *const c_void, *mut u64) -> i32 =
        std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(instance, create_info, sanitize_allocator(allocator), &mut out);
    std::ptr::write_unaligned(surface, out);
    res
}

unsafe fn forward_device_create_u64(
    name: &CStr,
    device: usize,
    create_info: *const c_void,
    allocator: *const c_void,
    out_handle: *mut u64,
) -> i32 {
    if !out_handle.is_null() {
        std::ptr::write_unaligned(out_handle, 0);
    }
    if device == 0 || create_info.is_null() || out_handle.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr_with_aliases(device, name) else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *const c_void, *mut u64) -> i32 =
        std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(device, create_info, sanitize_allocator(allocator), &mut out);
    std::ptr::write_unaligned(out_handle, out);
    res
}

unsafe fn forward_device_destroy_u64(
    name: &CStr,
    device: usize,
    handle: u64,
    allocator: *const c_void,
) {
    if device == 0 || handle == 0 {
        return;
    }
    let Some(vk) = vulkan_backend() else {
        return;
    };
    if let Some(ptr) = vk.device_proc_ptr_with_aliases(device, name) {
        let f: unsafe extern "C" fn(usize, u64, *const c_void) = std::mem::transmute(ptr);
        f(device, handle, sanitize_allocator(allocator));
    }
}

unsafe fn forward_create_pipelines(
    name: &CStr,
    device: usize,
    pipeline_cache: u64,
    create_info_count: u32,
    create_infos: *const c_void,
    allocator: *const c_void,
    pipelines: *mut u64,
) -> i32 {
    if device == 0 || (create_info_count > 0 && (create_infos.is_null() || pipelines.is_null())) {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr_with_aliases(device, name) else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, u64, u32, *const c_void, *const c_void, *mut u64) -> i32 =
        std::mem::transmute(ptr);
    let count = create_info_count as usize;
    let mut out = vec![0u64; count];
    let out_ptr = if count == 0 {
        std::ptr::null_mut()
    } else {
        out.as_mut_ptr()
    };
    let res = f(
        device,
        pipeline_cache,
        create_info_count,
        create_infos,
        sanitize_allocator(allocator),
        out_ptr,
    );
    for (i, &h) in out.iter().enumerate() {
        std::ptr::write_unaligned(pipelines.add(i), h);
    }
    res
}

unsafe extern "win64" fn tramp_vkCreateGraphicsPipelines(
    device: usize,
    pipeline_cache: u64,
    create_info_count: u32,
    create_infos: *const c_void,
    allocator: *const c_void,
    pipelines: *mut u64,
) -> i32 {
    forward_create_pipelines(
        c"vkCreateGraphicsPipelines",
        device,
        pipeline_cache,
        create_info_count,
        create_infos,
        allocator,
        pipelines,
    )
}

unsafe extern "win64" fn tramp_vkCreateComputePipelines(
    device: usize,
    pipeline_cache: u64,
    create_info_count: u32,
    create_infos: *const c_void,
    allocator: *const c_void,
    pipelines: *mut u64,
) -> i32 {
    forward_create_pipelines(
        c"vkCreateComputePipelines",
        device,
        pipeline_cache,
        create_info_count,
        create_infos,
        allocator,
        pipelines,
    )
}

unsafe extern "win64" fn tramp_vkCreateDeferredOperationKHR(
    device: usize,
    allocator: *const c_void,
    operation: *mut u64,
) -> i32 {
    if !operation.is_null() {
        std::ptr::write_unaligned(operation, 0);
    }
    if device == 0 || operation.is_null() {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let Some(vk) = vulkan_backend() else {
        return VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    let Some(ptr) = vk.device_proc_ptr(device, c"vkCreateDeferredOperationKHR") else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    };
    let f: unsafe extern "C" fn(usize, *const c_void, *mut u64) -> i32 = std::mem::transmute(ptr);
    let mut out = 0u64;
    let res = f(device, sanitize_allocator(allocator), &mut out);
    std::ptr::write_unaligned(operation, out);
    res
}

macro_rules! define_device_proc_interceptors {
    (
        direct { $($dir_name:literal => $dir_fn:ident),* $(,)? }
        create { $($c_tramp:ident => $c_name:literal),* $(,)? }
        destroy { $($d_tramp:ident => $d_name:literal),* $(,)? }
    ) => {
        $(
            #[allow(non_snake_case)]
            unsafe extern "win64" fn $c_tramp(
                device: usize,
                create_info: *const c_void,
                allocator: *const c_void,
                out_handle: *mut u64,
            ) -> i32 {
                const SYM: &CStr = match CStr::from_bytes_with_nul(concat!($c_name, "\0").as_bytes()) {
                    Ok(s) => s,
                    Err(_) => unreachable!(),
                };
                forward_device_create_u64(SYM, device, create_info, allocator, out_handle)
            }
        )*

        $(
            #[allow(non_snake_case)]
            unsafe extern "win64" fn $d_tramp(
                device: usize,
                handle: u64,
                allocator: *const c_void,
            ) {
                const SYM: &CStr = match CStr::from_bytes_with_nul(concat!($d_name, "\0").as_bytes()) {
                    Ok(s) => s,
                    Err(_) => unreachable!(),
                };
                forward_device_destroy_u64(SYM, device, handle, allocator);
            }
        )*

        /// Returns our ABI-bridged / allocator-sanitizing / surface-syncing wrapper for a device-level
        /// Vulkan entrypoint (called ONLY after confirming MoltenVK exposes the symbol for the device or
        /// instance).
        fn intercepted_device_proc(name: &str) -> Option<*const ()> {
            match name {
                $($dir_name => Some($dir_fn as *const ()),)*
                $($c_name => Some($c_tramp as *const ()),)*
                $($d_name => Some($d_tramp as *const ()),)*
                _ => None,
            }
        }
    };
}

define_device_proc_interceptors! {
    direct {
        // Float-argument Win64 -> AAPCS64 ABI adapters
        "vkCmdSetLineWidth" => vkCmdSetLineWidth,
        "vkCmdSetDepthBias" => vkCmdSetDepthBias,
        "vkCmdSetDepthBounds" => vkCmdSetDepthBounds,
        "vkCmdSetExtraPrimitiveOverestimationSizeEXT" => tramp_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
        // Surface-syncing + allocator-sanitizing + present-hook swapchain/queue procs
        "vkGetDeviceQueue" => vkGetDeviceQueue,
        "vkCreateSwapchainKHR" => vkCreateSwapchainKHR,
        "vkDestroySwapchainKHR" => vkDestroySwapchainKHR,
        "vkGetSwapchainImagesKHR" => vkGetSwapchainImagesKHR,
        "vkQueuePresentKHR" => vkQueuePresentKHR,
        // Already-exported allocator-sanitizing device procs
        "vkCreateShaderModule" => vkCreateShaderModule,
        "vkDestroyShaderModule" => vkDestroyShaderModule,
        "vkCreateCommandPool" => vkCreateCommandPool,
        "vkDestroyCommandPool" => vkDestroyCommandPool,
        "vkCreateSemaphore" => vkCreateSemaphore,
        "vkDestroySemaphore" => vkDestroySemaphore,
        "vkCreateFence" => vkCreateFence,
        "vkDestroyFence" => vkDestroyFence,
        "vkDestroyImageView" => vkDestroyImageView,
        // Multi-handle / custom-signature allocator-sanitizing device procs
        "vkCreateGraphicsPipelines" => tramp_vkCreateGraphicsPipelines,
        "vkCreateComputePipelines" => tramp_vkCreateComputePipelines,
        "vkCreateDeferredOperationKHR" => tramp_vkCreateDeferredOperationKHR,
    }
    create {
        tramp_vkAllocateMemory => "vkAllocateMemory",
        tramp_vkCreateEvent => "vkCreateEvent",
        tramp_vkCreateQueryPool => "vkCreateQueryPool",
        tramp_vkCreateBuffer => "vkCreateBuffer",
        tramp_vkCreateBufferView => "vkCreateBufferView",
        tramp_vkCreateImage => "vkCreateImage",
        tramp_vkCreateImageView => "vkCreateImageView",
        tramp_vkCreatePipelineCache => "vkCreatePipelineCache",
        tramp_vkCreatePipelineLayout => "vkCreatePipelineLayout",
        tramp_vkCreateSampler => "vkCreateSampler",
        tramp_vkCreateDescriptorSetLayout => "vkCreateDescriptorSetLayout",
        tramp_vkCreateDescriptorPool => "vkCreateDescriptorPool",
        tramp_vkCreateFramebuffer => "vkCreateFramebuffer",
        tramp_vkCreateRenderPass => "vkCreateRenderPass",
        tramp_vkCreateRenderPass2 => "vkCreateRenderPass2",
        tramp_vkCreateRenderPass2KHR => "vkCreateRenderPass2KHR",
        tramp_vkCreateSamplerYcbcrConversion => "vkCreateSamplerYcbcrConversion",
        tramp_vkCreateSamplerYcbcrConversionKHR => "vkCreateSamplerYcbcrConversionKHR",
        tramp_vkCreateDescriptorUpdateTemplate => "vkCreateDescriptorUpdateTemplate",
        tramp_vkCreateDescriptorUpdateTemplateKHR => "vkCreateDescriptorUpdateTemplateKHR",
        tramp_vkCreatePrivateDataSlot => "vkCreatePrivateDataSlot",
        tramp_vkCreatePrivateDataSlotEXT => "vkCreatePrivateDataSlotEXT",
    }
    destroy {
        tramp_vkFreeMemory => "vkFreeMemory",
        tramp_vkDestroyEvent => "vkDestroyEvent",
        tramp_vkDestroyQueryPool => "vkDestroyQueryPool",
        tramp_vkDestroyBuffer => "vkDestroyBuffer",
        tramp_vkDestroyBufferView => "vkDestroyBufferView",
        tramp_vkDestroyImage => "vkDestroyImage",
        tramp_vkDestroyPipelineCache => "vkDestroyPipelineCache",
        tramp_vkDestroyPipeline => "vkDestroyPipeline",
        tramp_vkDestroyPipelineLayout => "vkDestroyPipelineLayout",
        tramp_vkDestroySampler => "vkDestroySampler",
        tramp_vkDestroyDescriptorSetLayout => "vkDestroyDescriptorSetLayout",
        tramp_vkDestroyDescriptorPool => "vkDestroyDescriptorPool",
        tramp_vkDestroyFramebuffer => "vkDestroyFramebuffer",
        tramp_vkDestroyRenderPass => "vkDestroyRenderPass",
        tramp_vkDestroySamplerYcbcrConversion => "vkDestroySamplerYcbcrConversion",
        tramp_vkDestroySamplerYcbcrConversionKHR => "vkDestroySamplerYcbcrConversionKHR",
        tramp_vkDestroyDescriptorUpdateTemplate => "vkDestroyDescriptorUpdateTemplate",
        tramp_vkDestroyDescriptorUpdateTemplateKHR => "vkDestroyDescriptorUpdateTemplateKHR",
        tramp_vkDestroyPrivateDataSlot => "vkDestroyPrivateDataSlot",
        tramp_vkDestroyPrivateDataSlotEXT => "vkDestroyPrivateDataSlotEXT",
        tramp_vkDestroyDeferredOperationKHR => "vkDestroyDeferredOperationKHR",
    }
}

/// Returns our intercepted wrapper for instance-level or device-level Vulkan procs resolved via
/// `vkGetInstanceProcAddr` (after confirming MoltenVK resolves the symbol as non-null).
fn intercepted_instance_or_device_proc(name: &str) -> Option<*const ()> {
    match name {
        "vkGetPhysicalDeviceSurfaceCapabilitiesKHR" => {
            Some(vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const ())
        }
        "vkGetPhysicalDeviceSurfaceCapabilities2KHR" => {
            Some(tramp_vkGetPhysicalDeviceSurfaceCapabilities2KHR as *const ())
        }
        "vkCreateHeadlessSurfaceEXT" => Some(tramp_vkCreateHeadlessSurfaceEXT as *const ()),
        _ => intercepted_device_proc(name),
    }
}

// ---------------------------------------------------------------------------
// NVIDIA Low Latency Vulkan (`nvlowlatencyvk.dll` routed here)
// Honest NVLL_VK_NO_IMPLEMENTATION (-3) on non-NVIDIA Apple Silicon GPUs.
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn NvLL_VK_InitLowLatencyDevice(
    _vk_device: usize,
    signal_sem: *mut u64,
) -> i32 {
    if !signal_sem.is_null() {
        std::ptr::write_unaligned(signal_sem, 0);
    }
    crate::stub!("NvLL_VK_InitLowLatencyDevice -> NVLL_VK_NO_IMPLEMENTATION (-3)");
    NVLL_VK_NO_IMPLEMENTATION
}

crate::failing!(i32 = NVLL_VK_NO_IMPLEMENTATION, err 0;
    NvLL_VK_Initialize,
    NvLL_VK_Unload,
    NvLL_VK_DestroyLowLatencyDevice,
    NvLL_VK_SetSleepMode,
    NvLL_VK_Sleep,
    NvLL_VK_GetSleepStatus,
    NvLL_VK_SetLatencyMarker,
    NvLL_VK_GetLatency,
    NvLL_VK_NotifyOutOfBandQueue,
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, offset_of, size_of};

    #[test]
    fn vk_struct_layouts_match_vulkan_c_abi() {
        assert_eq!(size_of::<VkApplicationInfo>(), 48);
        assert_eq!(align_of::<VkApplicationInfo>(), 8);
        assert_eq!(offset_of!(VkApplicationInfo, s_type), 0);
        assert_eq!(offset_of!(VkApplicationInfo, p_next), 8);
        assert_eq!(offset_of!(VkApplicationInfo, p_application_name), 16);
        assert_eq!(offset_of!(VkApplicationInfo, application_version), 24);
        assert_eq!(offset_of!(VkApplicationInfo, p_engine_name), 32);
        assert_eq!(offset_of!(VkApplicationInfo, engine_version), 40);
        assert_eq!(offset_of!(VkApplicationInfo, api_version), 44);

        assert_eq!(size_of::<VkInstanceCreateInfo>(), 64);
        assert_eq!(align_of::<VkInstanceCreateInfo>(), 8);
        assert_eq!(offset_of!(VkInstanceCreateInfo, s_type), 0);
        assert_eq!(offset_of!(VkInstanceCreateInfo, p_next), 8);
        assert_eq!(offset_of!(VkInstanceCreateInfo, flags), 16);
        assert_eq!(offset_of!(VkInstanceCreateInfo, p_application_info), 24);
        assert_eq!(offset_of!(VkInstanceCreateInfo, enabled_layer_count), 32);
        assert_eq!(offset_of!(VkInstanceCreateInfo, pp_enabled_layer_names), 40);
        assert_eq!(offset_of!(VkInstanceCreateInfo, enabled_extension_count), 48);
        assert_eq!(
            offset_of!(VkInstanceCreateInfo, pp_enabled_extension_names),
            56
        );

        assert_eq!(size_of::<VkWin32SurfaceCreateInfoKHR>(), 40);
        assert_eq!(align_of::<VkWin32SurfaceCreateInfoKHR>(), 8);
        assert_eq!(offset_of!(VkWin32SurfaceCreateInfoKHR, s_type), 0);
        assert_eq!(offset_of!(VkWin32SurfaceCreateInfoKHR, p_next), 8);
        assert_eq!(offset_of!(VkWin32SurfaceCreateInfoKHR, flags), 16);
        assert_eq!(offset_of!(VkWin32SurfaceCreateInfoKHR, hinstance), 24);
        assert_eq!(offset_of!(VkWin32SurfaceCreateInfoKHR, hwnd), 32);

        assert_eq!(size_of::<VkMetalSurfaceCreateInfoEXT>(), 32);
        assert_eq!(align_of::<VkMetalSurfaceCreateInfoEXT>(), 8);
        assert_eq!(offset_of!(VkMetalSurfaceCreateInfoEXT, s_type), 0);
        assert_eq!(offset_of!(VkMetalSurfaceCreateInfoEXT, p_next), 8);
        assert_eq!(offset_of!(VkMetalSurfaceCreateInfoEXT, flags), 16);
        assert_eq!(offset_of!(VkMetalSurfaceCreateInfoEXT, p_layer), 24);

        assert_eq!(size_of::<VkMacOSSurfaceCreateInfoMVK>(), 32);
        assert_eq!(align_of::<VkMacOSSurfaceCreateInfoMVK>(), 8);
        assert_eq!(offset_of!(VkMacOSSurfaceCreateInfoMVK, p_view), 24);

        assert_eq!(size_of::<VkExtensionProperties>(), 260);
        assert_eq!(align_of::<VkExtensionProperties>(), 4);
        assert_eq!(offset_of!(VkExtensionProperties, extension_name), 0);
        assert_eq!(offset_of!(VkExtensionProperties, spec_version), 256);

        assert_eq!(size_of::<VkDeviceQueueCreateInfo>(), 40);
        assert_eq!(align_of::<VkDeviceQueueCreateInfo>(), 8);
        assert_eq!(offset_of!(VkDeviceQueueCreateInfo, s_type), 0);
        assert_eq!(offset_of!(VkDeviceQueueCreateInfo, p_next), 8);
        assert_eq!(offset_of!(VkDeviceQueueCreateInfo, flags), 16);
        assert_eq!(offset_of!(VkDeviceQueueCreateInfo, queue_family_index), 20);
        assert_eq!(offset_of!(VkDeviceQueueCreateInfo, queue_count), 24);
        assert_eq!(offset_of!(VkDeviceQueueCreateInfo, p_queue_priorities), 32);

        assert_eq!(size_of::<VkDeviceCreateInfo>(), 72);
        assert_eq!(align_of::<VkDeviceCreateInfo>(), 8);
        assert_eq!(offset_of!(VkDeviceCreateInfo, s_type), 0);
        assert_eq!(offset_of!(VkDeviceCreateInfo, p_next), 8);
        assert_eq!(offset_of!(VkDeviceCreateInfo, flags), 16);
        assert_eq!(offset_of!(VkDeviceCreateInfo, queue_create_info_count), 20);
        assert_eq!(offset_of!(VkDeviceCreateInfo, p_queue_create_infos), 24);
        assert_eq!(offset_of!(VkDeviceCreateInfo, enabled_layer_count), 32);
        assert_eq!(offset_of!(VkDeviceCreateInfo, pp_enabled_layer_names), 40);
        assert_eq!(offset_of!(VkDeviceCreateInfo, enabled_extension_count), 48);
        assert_eq!(offset_of!(VkDeviceCreateInfo, pp_enabled_extension_names), 56);
        assert_eq!(offset_of!(VkDeviceCreateInfo, p_enabled_features), 64);

        assert_eq!(size_of::<VkShaderModuleCreateInfo>(), 40);
        assert_eq!(align_of::<VkShaderModuleCreateInfo>(), 8);
        assert_eq!(offset_of!(VkShaderModuleCreateInfo, code_size), 24);
        assert_eq!(offset_of!(VkShaderModuleCreateInfo, p_code), 32);

        assert_eq!(size_of::<VkQueueFamilyProperties>(), 24);
        assert_eq!(align_of::<VkQueueFamilyProperties>(), 4);

        assert_eq!(size_of::<VkMemoryRequirements>(), 24);
        assert_eq!(align_of::<VkMemoryRequirements>(), 8);
    }

    #[test]
    fn rewrite_device_extensions_injects_portability_subset_and_dedups() {
        let to_strs = |v: &[CString]| -> Vec<String> {
            v.iter()
                .map(|s| s.to_str().unwrap().to_string())
                .collect()
        };

        let exts = rewrite_device_extensions(&["VK_KHR_swapchain", "VK_KHR_swapchain"], true);
        assert_eq!(
            to_strs(&exts),
            vec!["VK_KHR_swapchain", EXT_KHR_PORTABILITY_SUBSET]
        );

        // Already requested -> do not duplicate.
        let already = rewrite_device_extensions(
            &[
                "VK_KHR_swapchain",
                EXT_KHR_PORTABILITY_SUBSET,
                "VK_KHR_dynamic_rendering",
            ],
            true,
        );
        assert_eq!(
            to_strs(&already),
            vec![
                "VK_KHR_swapchain",
                EXT_KHR_PORTABILITY_SUBSET,
                "VK_KHR_dynamic_rendering",
            ]
        );

        // Physical device without portability subset -> leave requested list untouched.
        let none = rewrite_device_extensions(&["VK_KHR_swapchain"], false);
        assert_eq!(to_strs(&none), vec!["VK_KHR_swapchain"]);
    }

    #[test]
    fn promoted_proc_aliases_cover_vulkan_12_13_extensions() {
        for (core, khr) in [
            ("vkCmdBeginRendering", "vkCmdBeginRenderingKHR"),
            ("vkCmdEndRendering", "vkCmdEndRenderingKHR"),
            ("vkCmdPipelineBarrier2", "vkCmdPipelineBarrier2KHR"),
            ("vkCmdSetEvent2", "vkCmdSetEvent2KHR"),
            ("vkCmdResetEvent2", "vkCmdResetEvent2KHR"),
            ("vkCmdWaitEvents2", "vkCmdWaitEvents2KHR"),
            ("vkCmdWriteTimestamp2", "vkCmdWriteTimestamp2KHR"),
            ("vkQueueSubmit2", "vkQueueSubmit2KHR"),
            ("vkGetBufferDeviceAddress", "vkGetBufferDeviceAddressKHR"),
            (
                "vkGetBufferOpaqueCaptureAddress",
                "vkGetBufferOpaqueCaptureAddressKHR",
            ),
            (
                "vkGetDeviceMemoryOpaqueCaptureAddress",
                "vkGetDeviceMemoryOpaqueCaptureAddressKHR",
            ),
            ("vkCmdCopyBuffer2", "vkCmdCopyBuffer2KHR"),
            ("vkCmdCopyImage2", "vkCmdCopyImage2KHR"),
            ("vkCmdCopyBufferToImage2", "vkCmdCopyBufferToImage2KHR"),
            ("vkCmdCopyImageToBuffer2", "vkCmdCopyImageToBuffer2KHR"),
            ("vkCmdBlitImage2", "vkCmdBlitImage2KHR"),
            ("vkCmdResolveImage2", "vkCmdResolveImage2KHR"),
            (
                "vkGetDeviceBufferMemoryRequirements",
                "vkGetDeviceBufferMemoryRequirementsKHR",
            ),
            (
                "vkGetDeviceImageMemoryRequirements",
                "vkGetDeviceImageMemoryRequirementsKHR",
            ),
            (
                "vkGetDeviceImageSparseMemoryRequirements",
                "vkGetDeviceImageSparseMemoryRequirementsKHR",
            ),
            (
                "vkGetSemaphoreCounterValue",
                "vkGetSemaphoreCounterValueKHR",
            ),
            ("vkWaitSemaphores", "vkWaitSemaphoresKHR"),
            ("vkSignalSemaphore", "vkSignalSemaphoreKHR"),
        ] {
            let core_aliases: Vec<&str> = promoted_proc_aliases(core)
                .iter()
                .map(|c| c.to_str().unwrap())
                .collect();
            assert!(
                core_aliases.contains(&khr),
                "{core} should alias to {khr}, got {core_aliases:?}"
            );
            let khr_aliases: Vec<&str> = promoted_proc_aliases(khr)
                .iter()
                .map(|c| c.to_str().unwrap())
                .collect();
            assert!(
                khr_aliases.contains(&core),
                "{khr} should alias to {core}, got {khr_aliases:?}"
            );
        }
        assert!(promoted_proc_aliases("vkGetBufferDeviceAddress")
            .iter()
            .any(|c| c.to_str().unwrap() == "vkGetBufferDeviceAddressEXT"));
        assert!(promoted_proc_aliases("vkUnknownProc").is_empty());
    }

    #[test]
    fn rewrite_instance_extensions_rewrites_dedups_and_injects_portability() {
        let to_strs = |v: &[CString]| -> Vec<String> {
            v.iter()
                .map(|s| s.to_str().unwrap().to_string())
                .collect()
        };

        // 1. Standard Windows surface request + portability enumeration advertised.
        let (exts, flags) = rewrite_instance_extensions(
            &[EXT_KHR_SURFACE, EXT_KHR_WIN32_SURFACE],
            0,
            true,
        );
        assert_eq!(
            to_strs(&exts),
            vec![
                EXT_KHR_SURFACE,
                EXT_EXT_METAL_SURFACE,
                EXT_KHR_PORTABILITY_ENUMERATION,
            ]
        );
        assert_eq!(flags, VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR);

        // 2. Deduplication when both Win32 & Metal surface or duplicates are requested.
        let (exts, flags) = rewrite_instance_extensions(
            &[
                EXT_KHR_SURFACE,
                EXT_EXT_METAL_SURFACE,
                EXT_KHR_WIN32_SURFACE,
                EXT_KHR_SURFACE,
                EXT_KHR_PORTABILITY_ENUMERATION,
                "VK_EXT_debug_utils",
            ],
            0x4,
            true,
        );
        assert_eq!(
            to_strs(&exts),
            vec![
                EXT_KHR_SURFACE,
                EXT_EXT_METAL_SURFACE,
                EXT_KHR_PORTABILITY_ENUMERATION,
                "VK_EXT_debug_utils",
            ]
        );
        assert_eq!(flags, 0x4 | VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR);

        // 3. No portability enumeration advertised -> do not inject extension or alter flags.
        let (exts, flags) = rewrite_instance_extensions(
            &[EXT_KHR_SURFACE, EXT_KHR_WIN32_SURFACE, EXT_KHR_WIN32_SURFACE],
            0x2,
            false,
        );
        assert_eq!(to_strs(&exts), vec![EXT_KHR_SURFACE, EXT_EXT_METAL_SURFACE]);
        assert_eq!(flags, 0x2);
    }

    #[test]
    fn augment_instance_extensions_exposes_win32_surface_when_metal_present() {
        let mut exts = vec![
            VkExtensionProperties::new(EXT_KHR_SURFACE, 25),
            VkExtensionProperties::new(EXT_EXT_METAL_SURFACE, 1),
        ];
        augment_instance_extensions(&mut exts);
        assert_eq!(exts.len(), 3);
        let win32 = exts
            .iter()
            .find(|e| e.name_str() == EXT_KHR_WIN32_SURFACE)
            .expect("VK_KHR_win32_surface should be exposed");
        assert_eq!(win32.spec_version, VK_KHR_WIN32_SURFACE_SPEC_VERSION);

        // Idempotent when called again.
        augment_instance_extensions(&mut exts);
        assert_eq!(exts.len(), 3);

        // Also works if only VK_MVK_macos_surface is advertised.
        let mut macos_only = vec![
            VkExtensionProperties::new(EXT_KHR_SURFACE, 25),
            VkExtensionProperties::new(EXT_MVK_MACOS_SURFACE, 3),
        ];
        augment_instance_extensions(&mut macos_only);
        assert!(macos_only
            .iter()
            .any(|e| e.name_str() == EXT_KHR_WIN32_SURFACE));

        // Does not advertise Win32 surface if neither Metal nor macOS surface exists.
        let mut headless = vec![VkExtensionProperties::new(EXT_KHR_SURFACE, 25)];
        augment_instance_extensions(&mut headless);
        assert_eq!(headless.len(), 1);
    }

    #[test]
    fn loader_candidate_search_order() {
        let cands = build_loader_candidates(
            Some("/custom/port/libMoltenVK.dylib"),
            Some("/custom/a/libMoltenVK.dylib"),
            Some("/custom/b/libMoltenVK.dylib"),
            Some(Path::new("/App.app/Contents/MacOS")),
            Some("/opt/VulkanSDK/1.3/macOS"),
        );
        assert_eq!(cands[0], "/custom/port/libMoltenVK.dylib");
        assert_eq!(cands[1], "/custom/b/libMoltenVK.dylib");
        assert_eq!(cands[2], "/custom/a/libMoltenVK.dylib");
        assert_eq!(
            cands[3],
            "/App.app/Contents/MacOS/../Frameworks/libMoltenVK.dylib"
        );
        assert_eq!(
            cands[4],
            "@executable_path/../Frameworks/libMoltenVK.dylib"
        );
        assert_eq!(cands[5], "/opt/homebrew/lib/libMoltenVK.dylib");
        assert_eq!(cands[6], "/usr/local/lib/libMoltenVK.dylib");
        assert_eq!(
            cands[7],
            "/opt/VulkanSDK/1.3/macOS/lib/libMoltenVK.dylib"
        );
        assert_eq!(cands[8], "libMoltenVK.dylib");
        assert!(cands[9..]
            .iter()
            .any(|c| c == "@executable_path/../Frameworks/libvulkan.1.dylib"));
        assert!(cands[9..].iter().any(|c| c == "libvulkan.1.dylib"));
        assert!(cands[9..].iter().any(|c| c == "libvulkan.dylib"));

        // Deduplicates repeated paths cleanly.
        let dedup = build_loader_candidates(
            Some("/opt/homebrew/lib/libMoltenVK.dylib"),
            Some("/opt/homebrew/lib/libMoltenVK.dylib"),
            Some("/opt/homebrew/lib/libMoltenVK.dylib"),
            None,
            None,
        );
        assert_eq!(
            dedup
                .iter()
                .filter(|c| c.as_str() == "/opt/homebrew/lib/libMoltenVK.dylib")
                .count(),
            1
        );
    }

    #[test]
    fn exports_present_in_vulkan_dll_table() {
        for sym in [
            "vkGetInstanceProcAddr",
            "vkGetDeviceProcAddr",
            "vkEnumerateInstanceVersion",
            "vkEnumerateInstanceExtensionProperties",
            "vkEnumerateInstanceLayerProperties",
            "vkCreateInstance",
            "vkDestroyInstance",
            "vkEnumeratePhysicalDevices",
            "vkCreateWin32SurfaceKHR",
            "vkGetPhysicalDeviceWin32PresentationSupportKHR",
            "vkDestroySurfaceKHR",
            "vkCreateDevice",
            "vkDestroyDevice",
            "vkDeviceWaitIdle",
            "vkCreateShaderModule",
            "vkDestroyShaderModule",
            "vkAllocateCommandBuffers",
            "vkCreateCommandPool",
            "vkCreateSemaphore",
            "vkGetDeviceQueue",
            "vkQueueSubmit",
            "vkCreateSwapchainKHR",
            "vkQueuePresentKHR",
            // Float-arg Win64 -> AAPCS64 bridged core commands
            "vkCmdSetLineWidth",
            "vkCmdSetDepthBias",
            "vkCmdSetDepthBounds",
            // Vulkan 1.2 / 1.3 & KHR entrypoints
            "vkCmdBeginRendering",
            "vkCmdBeginRenderingKHR",
            "vkCmdEndRendering",
            "vkCmdEndRenderingKHR",
            "vkCmdPipelineBarrier2",
            "vkCmdPipelineBarrier2KHR",
            "vkCmdSetEvent2",
            "vkCmdSetEvent2KHR",
            "vkCmdResetEvent2",
            "vkCmdResetEvent2KHR",
            "vkCmdWaitEvents2",
            "vkCmdWaitEvents2KHR",
            "vkCmdWriteTimestamp2",
            "vkCmdWriteTimestamp2KHR",
            "vkQueueSubmit2",
            "vkQueueSubmit2KHR",
            "vkGetSemaphoreCounterValue",
            "vkGetSemaphoreCounterValueKHR",
            "vkWaitSemaphores",
            "vkWaitSemaphoresKHR",
            "vkSignalSemaphore",
            "vkSignalSemaphoreKHR",
            "vkGetBufferDeviceAddress",
            "vkGetBufferDeviceAddressKHR",
            "vkGetBufferDeviceAddressEXT",
            "vkGetBufferOpaqueCaptureAddress",
            "vkGetBufferOpaqueCaptureAddressKHR",
            "vkGetDeviceMemoryOpaqueCaptureAddress",
            "vkGetDeviceMemoryOpaqueCaptureAddressKHR",
            "vkCmdCopyBuffer2",
            "vkCmdCopyBuffer2KHR",
            "vkCmdCopyImage2",
            "vkCmdCopyImage2KHR",
            "vkCmdCopyBufferToImage2",
            "vkCmdCopyBufferToImage2KHR",
            "vkCmdCopyImageToBuffer2",
            "vkCmdCopyImageToBuffer2KHR",
            "vkCmdBlitImage2",
            "vkCmdBlitImage2KHR",
            "vkCmdResolveImage2",
            "vkCmdResolveImage2KHR",
            "vkGetDeviceBufferMemoryRequirements",
            "vkGetDeviceBufferMemoryRequirementsKHR",
            "vkGetDeviceImageMemoryRequirements",
            "vkGetDeviceImageMemoryRequirementsKHR",
            "vkGetDeviceImageSparseMemoryRequirements",
            "vkGetDeviceImageSparseMemoryRequirementsKHR",
            "NvLL_VK_Initialize",
        ] {
            assert!(
                crate::lookup("vulkan-1.dll", sym).is_some(),
                "missing vulkan-1.dll export: {sym}"
            );
        }
    }

    #[test]
    fn vulkan_runtime_honest_absence_or_live_moltenvk() {
        unsafe {
            // Global proc lookups must always resolve through vkGetInstanceProcAddr(NULL, ...).
            assert_eq!(
                vkGetInstanceProcAddr(0, c"vkCreateInstance".as_ptr()),
                vkCreateInstance as *const () as *const c_void
            );
            assert_eq!(
                vkGetInstanceProcAddr(0, c"vkEnumerateInstanceVersion".as_ptr()),
                vkEnumerateInstanceVersion as *const () as *const c_void
            );
            assert_eq!(
                vkGetInstanceProcAddr(0, c"vkEnumerateInstanceExtensionProperties".as_ptr()),
                vkEnumerateInstanceExtensionProperties as *const () as *const c_void
            );
            assert!(vkGetInstanceProcAddr(0, std::ptr::null()).is_null());
            assert!(vkGetInstanceProcAddr(0, c"vkDestroyInstance".as_ptr()).is_null());
            assert!(vkGetDeviceProcAddr(0, c"vkCreateShaderModule".as_ptr()).is_null());
            assert!(vkGetDeviceProcAddr(1, std::ptr::null()).is_null());

            // Null create_info must return VK_ERROR_INITIALIZATION_FAILED and zero output handle.
            let mut null_ci_inst: usize = 0xCAFE;
            assert_eq!(
                vkCreateInstance(std::ptr::null(), std::ptr::null(), &mut null_ci_inst),
                VK_ERROR_INITIALIZATION_FAILED
            );
            assert_eq!(null_ci_inst, 0);

            // NVIDIA Reflex Vulkan is honestly reported as not implemented on Apple Silicon.
            assert_eq!(NvLL_VK_Initialize(), NVLL_VK_NO_IMPLEMENTATION);
            let mut reflex_sem: u64 = 0x1234;
            assert_eq!(
                NvLL_VK_InitLowLatencyDevice(1, &mut reflex_sem),
                NVLL_VK_NO_IMPLEMENTATION
            );
            assert_eq!(reflex_sem, 0);

            let ext_ptrs = [
                c"VK_KHR_surface".as_ptr(),
                c"VK_KHR_win32_surface".as_ptr(),
            ];
            let app_info = VkApplicationInfo {
                s_type: 0,
                p_next: std::ptr::null(),
                p_application_name: c"vulkan-bridge-test".as_ptr(),
                application_version: 1,
                p_engine_name: c"port-factory".as_ptr(),
                engine_version: 1,
                api_version: VK_API_VERSION_1_0,
            };
            let ci = VkInstanceCreateInfo {
                s_type: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: 0,
                p_application_info: &app_info,
                enabled_layer_count: 0,
                pp_enabled_layer_names: std::ptr::null(),
                enabled_extension_count: ext_ptrs.len() as u32,
                pp_enabled_extension_names: ext_ptrs.as_ptr(),
            };

            match vulkan_backend() {
                None => {
                    // MoltenVK absent: every call must fail honestly and zero output handles.
                    let mut inst: usize = 0xDEAD_BEEF;
                    assert_eq!(
                        vkCreateInstance(&ci, std::ptr::null(), &mut inst),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(inst, 0);

                    let mut ver: u32 = 99;
                    assert_eq!(
                        vkEnumerateInstanceVersion(&mut ver),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(ver, 0);

                    let mut ext_count: u32 = 99;
                    assert_eq!(
                        vkEnumerateInstanceExtensionProperties(
                            std::ptr::null(),
                            &mut ext_count,
                            std::ptr::null_mut(),
                        ),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(ext_count, 0);

                    let mut phys_count: u32 = 99;
                    assert_eq!(
                        vkEnumeratePhysicalDevices(1, &mut phys_count, std::ptr::null_mut()),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(phys_count, 0);

                    let surf_ci = VkWin32SurfaceCreateInfoKHR {
                        s_type: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                        p_next: std::ptr::null(),
                        flags: 0,
                        hinstance: 0,
                        hwnd: 0x1000,
                    };
                    let mut surf: u64 = 0xDEAD;
                    assert_eq!(
                        vkCreateWin32SurfaceKHR(1, &surf_ci, std::ptr::null(), &mut surf),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(surf, 0);
                    assert_eq!(vkGetPhysicalDeviceWin32PresentationSupportKHR(1, 0), VK_FALSE);

                    // No synthetic fake handles (e.g. 0x766B_*) returned anywhere.
                    let mut cmd_buf: usize = 0xBEEF;
                    assert_eq!(
                        vkAllocateCommandBuffers(1, std::ptr::dangling::<c_void>(), &mut cmd_buf),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(cmd_buf, 0);

                    let mut pool: u64 = 0xBEEF;
                    assert_eq!(
                        vkCreateCommandPool(
                            1,
                            std::ptr::dangling::<c_void>(),
                            std::ptr::null(),
                            &mut pool,
                        ),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(pool, 0);

                    let mut sem: u64 = 0xBEEF;
                    assert_eq!(
                        vkCreateSemaphore(
                            1,
                            std::ptr::dangling::<c_void>(),
                            std::ptr::null(),
                            &mut sem,
                        ),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(sem, 0);

                    let mut queue: usize = 0xBEEF;
                    vkGetDeviceQueue(1, 0, 0, &mut queue);
                    assert_eq!(queue, 0);

                    let mut reqs = VkMemoryRequirements {
                        size: 123,
                        alignment: 456,
                        memory_type_bits: 789,
                    };
                    vkGetImageMemoryRequirements(1, 1, &mut reqs);
                    assert_eq!(reqs.size, 0);
                    assert_eq!(reqs.alignment, 0);
                    assert_eq!(reqs.memory_type_bits, 0);

                    let dummy_spirv = [0x0723_0203u32; 5];
                    let sm_ci = VkShaderModuleCreateInfo {
                        s_type: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
                        p_next: std::ptr::null(),
                        flags: 0,
                        code_size: dummy_spirv.len() * 4,
                        p_code: dummy_spirv.as_ptr(),
                    };
                    let mut sm: u64 = 0xBEEF;
                    assert_eq!(
                        vkCreateShaderModule(1, &sm_ci, std::ptr::null(), &mut sm),
                        VK_ERROR_INCOMPATIBLE_DRIVER
                    );
                    assert_eq!(sm, 0);
                }
                Some(_) => {
                    // Live MoltenVK present on host: exercise real Vulkan instance & GPU enumeration.
                    let mut ver = 0u32;
                    assert_eq!(vkEnumerateInstanceVersion(&mut ver), VK_SUCCESS);
                    assert!(ver >= VK_API_VERSION_1_0);

                    let mut ext_count = 0u32;
                    assert_eq!(
                        vkEnumerateInstanceExtensionProperties(
                            std::ptr::null(),
                            &mut ext_count,
                            std::ptr::null_mut(),
                        ),
                        VK_SUCCESS
                    );
                    assert!(ext_count > 0);
                    let mut exts = vec![VkExtensionProperties::zeroed(); ext_count as usize];
                    assert_eq!(
                        vkEnumerateInstanceExtensionProperties(
                            std::ptr::null(),
                            &mut ext_count,
                            exts.as_mut_ptr(),
                        ),
                        VK_SUCCESS
                    );
                    assert!(
                        exts.iter().any(|e| e.name_str() == EXT_KHR_WIN32_SURFACE),
                        "MoltenVK bridge must advertise VK_KHR_win32_surface"
                    );

                    let mut inst: usize = 0;
                    assert_eq!(vkCreateInstance(&ci, std::ptr::null(), &mut inst), VK_SUCCESS);
                    assert_ne!(inst, 0);

                    assert_eq!(
                        vkGetInstanceProcAddr(inst, c"vkCreateWin32SurfaceKHR".as_ptr()),
                        vkCreateWin32SurfaceKHR as *const () as *const c_void
                    );
                    assert_eq!(
                        vkGetInstanceProcAddr(inst, c"vkDestroySurfaceKHR".as_ptr()),
                        vkDestroySurfaceKHR as *const () as *const c_void
                    );
                    assert_eq!(
                        vkGetInstanceProcAddr(
                            inst,
                            c"vkGetPhysicalDeviceWin32PresentationSupportKHR".as_ptr()
                        ),
                        vkGetPhysicalDeviceWin32PresentationSupportKHR as *const () as *const c_void
                    );

                    let mut phys_count = 0u32;
                    assert_eq!(
                        vkEnumeratePhysicalDevices(inst, &mut phys_count, std::ptr::null_mut()),
                        VK_SUCCESS
                    );
                    assert!(phys_count >= 1, "expected at least one Apple Metal GPU");
                    let mut phys_devs = vec![0usize; phys_count as usize];
                    assert_eq!(
                        vkEnumeratePhysicalDevices(inst, &mut phys_count, phys_devs.as_mut_ptr()),
                        VK_SUCCESS
                    );
                    assert_ne!(phys_devs[0], 0);
                    assert_eq!(
                        vkGetPhysicalDeviceWin32PresentationSupportKHR(phys_devs[0], 0),
                        VK_TRUE
                    );

                    // Invalid HWND must honestly fail surface creation.
                    let bad_surf_ci = VkWin32SurfaceCreateInfoKHR {
                        s_type: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                        p_next: std::ptr::null(),
                        flags: 0,
                        hinstance: 0,
                        hwnd: 0,
                    };
                    let mut bad_surf = 0xDEADu64;
                    assert_eq!(
                        vkCreateWin32SurfaceKHR(
                            inst,
                            &bad_surf_ci,
                            std::ptr::null(),
                            &mut bad_surf,
                        ),
                        VK_ERROR_INITIALIZATION_FAILED
                    );
                    assert_eq!(bad_surf, 0);

                    vkDestroyInstance(inst, std::ptr::null());
                }
            }
        }
    }

    /// Optional live-MoltenVK end-to-end integration test.
    ///
    /// Gated behind `PORT_TEST_MOLTENVK=1` (and requires a real `libMoltenVK.dylib` via
    /// `PORT_VULKAN_DYLIB` or standard macOS search paths). Skips cleanly with a clear message
    /// when unset or when no MoltenVK runtime is installed — never fakes success.
    #[test]
    fn live_moltenvk_end_to_end_integration() {
        if std::env::var("PORT_TEST_MOLTENVK").ok().as_deref() != Some("1") {
            eprintln!(
                "skipping live_moltenvk_end_to_end_integration: set PORT_TEST_MOLTENVK=1 (and optionally PORT_VULKAN_DYLIB=/path/to/libMoltenVK.dylib) to run"
            );
            return;
        }
        let Some(_) = vulkan_backend() else {
            eprintln!(
                "skipping live_moltenvk_end_to_end_integration: libMoltenVK.dylib not found on host (set PORT_VULKAN_DYLIB=/path/to/libMoltenVK.dylib)"
            );
            return;
        };

        #[repr(C)]
        struct VkPhysicalDevicePropertiesHead {
            api_version: u32,
            driver_version: u32,
            vendor_id: u32,
            device_id: u32,
            device_type: u32,
            device_name: [c_char; 256],
            _rest: [u8; 768],
        }

        #[repr(C)]
        struct VkSurfaceCapabilitiesKHR {
            min_image_count: u32,
            max_image_count: u32,
            current_width: u32,
            current_height: u32,
            min_width: u32,
            min_height: u32,
            max_width: u32,
            max_height: u32,
            max_image_array_layers: u32,
            supported_transforms: u32,
            current_transform: u32,
            supported_composite_alpha: u32,
            supported_usage_flags: u32,
        }

        struct TestHwndGuard {
            hwnd: usize,
            #[cfg(target_os = "macos")]
            _layer: objc2::rc::Retained<objc2_quartz_core::CAMetalLayer>,
        }
        impl Drop for TestHwndGuard {
            fn drop(&mut self) {
                crate::macos::unregister_shared(self.hwnd);
            }
        }

        unsafe {
            // 1. Instance version + vkCreateInstance (Win32 surface rewritten to Metal + portability).
            let mut inst_ver = 0u32;
            assert_eq!(vkEnumerateInstanceVersion(&mut inst_ver), VK_SUCCESS);
            let req_api_ver = if inst_ver >= VK_API_VERSION_1_3 {
                VK_API_VERSION_1_3
            } else if inst_ver >= VK_API_VERSION_1_2 {
                VK_API_VERSION_1_2
            } else {
                VK_API_VERSION_1_0
            };

            let inst_exts = [
                c"VK_KHR_surface".as_ptr(),
                c"VK_KHR_win32_surface".as_ptr(),
            ];
            let app_info = VkApplicationInfo {
                s_type: 0,
                p_next: std::ptr::null(),
                p_application_name: c"port-factory-moltenvk-test".as_ptr(),
                application_version: 1,
                p_engine_name: c"port-factory".as_ptr(),
                engine_version: 1,
                api_version: req_api_ver,
            };
            let inst_ci = VkInstanceCreateInfo {
                s_type: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: 0,
                p_application_info: &app_info,
                enabled_layer_count: 0,
                pp_enabled_layer_names: std::ptr::null(),
                enabled_extension_count: inst_exts.len() as u32,
                pp_enabled_extension_names: inst_exts.as_ptr(),
            };
            let mut inst: usize = 0;
            assert_eq!(
                vkCreateInstance(&inst_ci, std::ptr::null(), &mut inst),
                VK_SUCCESS
            );
            assert_ne!(inst, 0);

            // Verify vkGetInstanceProcAddr intercepts vkCreateDevice, surface creation, and surface caps.
            assert_eq!(
                vkGetInstanceProcAddr(inst, c"vkCreateDevice".as_ptr()),
                vkCreateDevice as *const () as *const c_void
            );
            assert_eq!(
                vkGetInstanceProcAddr(inst, c"vkCreateWin32SurfaceKHR".as_ptr()),
                vkCreateWin32SurfaceKHR as *const () as *const c_void
            );
            assert_eq!(
                vkGetInstanceProcAddr(inst, c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR".as_ptr()),
                vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const () as *const c_void
            );

            // 2. Enumerate physical devices + vkGetPhysicalDeviceProperties.
            let mut phys_count = 0u32;
            assert_eq!(
                vkEnumeratePhysicalDevices(inst, &mut phys_count, std::ptr::null_mut()),
                VK_SUCCESS
            );
            assert!(phys_count >= 1, "expected at least 1 MoltenVK physical device");
            let mut phys_devs = vec![0usize; phys_count as usize];
            assert_eq!(
                vkEnumeratePhysicalDevices(inst, &mut phys_count, phys_devs.as_mut_ptr()),
                VK_SUCCESS
            );
            let phys_dev = phys_devs[0];
            assert_ne!(phys_dev, 0);

            let mut props = std::mem::zeroed::<VkPhysicalDevicePropertiesHead>();
            vkGetPhysicalDeviceProperties(phys_dev, &mut props as *mut _ as *mut c_void);
            let dev_name = CStr::from_ptr(props.device_name.as_ptr())
                .to_string_lossy()
                .into_owned();
            assert!(!dev_name.is_empty(), "deviceName must be non-empty");
            assert!(
                props.api_version >= VK_API_VERSION_1_0,
                "invalid physical device apiVersion: {:#x}",
                props.api_version
            );
            let api_major = (props.api_version >> 22) & 0x7f;
            let api_minor = (props.api_version >> 12) & 0x3ff;
            let api_patch = props.api_version & 0xfff;
            eprintln!(
                "[moltenvk-live] GPU={dev_name:?} vendor={:#x} device={:#x} Vulkan={api_major}.{api_minor}.{api_patch}",
                props.vendor_id, props.device_id
            );

            // 3. Real CAMetalLayer-backed Win32 surface via vkCreateWin32SurfaceKHR.
            const TEST_HWND: usize = 0x0000_7F00_0001_4B56;
            #[cfg(target_os = "macos")]
            let _hwnd_guard = {
                use std::sync::atomic::{AtomicBool, AtomicU64};
                use std::sync::Arc;
                let layer = objc2_quartz_core::CAMetalLayer::new();
                let layer_ptr = objc2::rc::Retained::as_ptr(&layer) as usize;
                let shared = Arc::new(crate::macos::WindowShared {
                    layer: layer_ptr,
                    client_px: AtomicU64::new(640u64 | (480u64 << 32)),
                    scale: AtomicU64::new(1.0f64.to_bits()),
                    occluded: AtomicBool::new(false),
                });
                crate::macos::register_shared(TEST_HWND, shared);
                TestHwndGuard {
                    hwnd: TEST_HWND,
                    _layer: layer,
                }
            };

            let surf_ci = VkWin32SurfaceCreateInfoKHR {
                s_type: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                p_next: std::ptr::null(),
                flags: 0,
                hinstance: 0,
                hwnd: TEST_HWND,
            };
            let mut surface: u64 = 0;
            assert_eq!(
                vkCreateWin32SurfaceKHR(inst, &surf_ci, std::ptr::null(), &mut surface),
                VK_SUCCESS
            );
            assert_ne!(surface, 0, "vkCreateWin32SurfaceKHR must return a real VkSurfaceKHR");

            // Find graphics queue family and check presentation support on the new surface.
            let mut qf_count = 0u32;
            vkGetPhysicalDeviceQueueFamilyProperties(phys_dev, &mut qf_count, std::ptr::null_mut());
            assert!(qf_count >= 1);
            let mut qf_props = vec![VkQueueFamilyProperties::default(); qf_count as usize];
            vkGetPhysicalDeviceQueueFamilyProperties(phys_dev, &mut qf_count, qf_props.as_mut_ptr());
            let gfx_qf = qf_props
                .iter()
                .position(|q| q.queue_flags & VK_QUEUE_GRAPHICS_BIT != 0)
                .expect("expected a graphics queue family") as u32;

            let mut surf_supported = VK_FALSE;
            assert_eq!(
                vkGetPhysicalDeviceSurfaceSupportKHR(
                    phys_dev,
                    gfx_qf,
                    surface,
                    &mut surf_supported
                ),
                VK_SUCCESS
            );
            assert_eq!(surf_supported, VK_TRUE);

            let mut surf_caps = std::mem::zeroed::<VkSurfaceCapabilitiesKHR>();
            assert_eq!(
                vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                    phys_dev,
                    surface,
                    &mut surf_caps as *mut _ as *mut c_void
                ),
                VK_SUCCESS
            );
            assert!(surf_caps.min_image_count >= 1);

            // 4. vkCreateDevice (auto-injects VK_KHR_portability_subset when advertised) +
            //    vkGetDeviceProcAddr verification across Vulkan 1.2 / 1.3 & KHR entrypoints.
            let queue_priority = 1.0f32;
            let qci = VkDeviceQueueCreateInfo {
                s_type: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: 0,
                queue_family_index: gfx_qf,
                queue_count: 1,
                p_queue_priorities: &queue_priority,
            };
            let dev_ci = VkDeviceCreateInfo {
                s_type: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: 0,
                queue_create_info_count: 1,
                p_queue_create_infos: &qci,
                enabled_layer_count: 0,
                pp_enabled_layer_names: std::ptr::null(),
                enabled_extension_count: 0,
                pp_enabled_extension_names: std::ptr::null(),
                p_enabled_features: std::ptr::null(),
            };
            let mut device: usize = 0;
            assert_eq!(
                vkCreateDevice(
                    phys_dev,
                    &dev_ci as *const _ as *const c_void,
                    std::ptr::null(),
                    &mut device,
                ),
                VK_SUCCESS
            );
            assert_ne!(device, 0);

            let mut queue: usize = 0;
            vkGetDeviceQueue(device, gfx_qf, 0, &mut queue);
            assert_ne!(queue, 0);

            // Verify vkGetDeviceProcAddr intercepts vkDestroyDevice, float-arg ABI adapters, and
            // allocator-sanitizing device procs, while forwarding Vulkan 1.2/1.3 & KHR procs.
            assert_eq!(
                vkGetDeviceProcAddr(device, c"vkDestroyDevice".as_ptr()),
                vkDestroyDevice as *const () as *const c_void
            );
            assert_eq!(
                vkGetDeviceProcAddr(device, c"vkCmdSetLineWidth".as_ptr()),
                vkCmdSetLineWidth as *const () as *const c_void
            );
            assert_eq!(
                vkGetDeviceProcAddr(device, c"vkCmdSetDepthBias".as_ptr()),
                vkCmdSetDepthBias as *const () as *const c_void
            );
            assert_eq!(
                vkGetDeviceProcAddr(device, c"vkCmdSetDepthBounds".as_ptr()),
                vkCmdSetDepthBounds as *const () as *const c_void
            );
            assert_eq!(
                vkGetDeviceProcAddr(device, c"vkCreateBuffer".as_ptr()),
                tramp_vkCreateBuffer as *const () as *const c_void
            );
            assert_eq!(
                vkGetDeviceProcAddr(device, c"vkDestroyBuffer".as_ptr()),
                tramp_vkDestroyBuffer as *const () as *const c_void
            );
            assert_eq!(
                vkGetDeviceProcAddr(device, c"vkCreateGraphicsPipelines".as_ptr()),
                tramp_vkCreateGraphicsPipelines as *const () as *const c_void
            );

            let device_procs: &[&CStr] = &[
                // VK_KHR_dynamic_rendering / 1.3
                c"vkCmdBeginRendering",
                c"vkCmdEndRendering",
                c"vkCmdBeginRenderingKHR",
                c"vkCmdEndRenderingKHR",
                // VK_KHR_synchronization2 / 1.3
                c"vkCmdPipelineBarrier2",
                c"vkCmdPipelineBarrier2KHR",
                c"vkCmdSetEvent2",
                c"vkCmdSetEvent2KHR",
                c"vkCmdResetEvent2",
                c"vkCmdResetEvent2KHR",
                c"vkCmdWaitEvents2",
                c"vkCmdWaitEvents2KHR",
                c"vkCmdWriteTimestamp2",
                c"vkCmdWriteTimestamp2KHR",
                c"vkQueueSubmit2",
                c"vkQueueSubmit2KHR",
                // VK_KHR_buffer_device_address / 1.2
                c"vkGetBufferDeviceAddress",
                c"vkGetBufferDeviceAddressKHR",
                c"vkGetBufferDeviceAddressEXT",
                c"vkGetBufferOpaqueCaptureAddress",
                c"vkGetBufferOpaqueCaptureAddressKHR",
                c"vkGetDeviceMemoryOpaqueCaptureAddress",
                c"vkGetDeviceMemoryOpaqueCaptureAddressKHR",
                // VK_KHR_copy_commands2 / 1.3
                c"vkCmdCopyBuffer2",
                c"vkCmdCopyBuffer2KHR",
                c"vkCmdCopyImage2",
                c"vkCmdCopyImage2KHR",
                c"vkCmdCopyBufferToImage2",
                c"vkCmdCopyBufferToImage2KHR",
                c"vkCmdCopyImageToBuffer2",
                c"vkCmdCopyImageToBuffer2KHR",
                c"vkCmdBlitImage2",
                c"vkCmdBlitImage2KHR",
                c"vkCmdResolveImage2",
                c"vkCmdResolveImage2KHR",
                // VK_KHR_maintenance4 / 1.3
                c"vkGetDeviceBufferMemoryRequirements",
                c"vkGetDeviceBufferMemoryRequirementsKHR",
                c"vkGetDeviceImageMemoryRequirements",
                c"vkGetDeviceImageMemoryRequirementsKHR",
                c"vkGetDeviceImageSparseMemoryRequirements",
                c"vkGetDeviceImageSparseMemoryRequirementsKHR",
                // VK_KHR_timeline_semaphore / 1.2
                c"vkGetSemaphoreCounterValue",
                c"vkGetSemaphoreCounterValueKHR",
                c"vkWaitSemaphores",
                c"vkWaitSemaphoresKHR",
                c"vkSignalSemaphore",
                c"vkSignalSemaphoreKHR",
            ];
            for &sym in device_procs {
                let p = vkGetDeviceProcAddr(device, sym.as_ptr());
                assert!(
                    !p.is_null(),
                    "vkGetDeviceProcAddr returned NULL for {}",
                    sym.to_string_lossy()
                );
            }

            // Also verify vkGetInstanceProcAddr forwards Vulkan 1.2/1.3 & KHR entrypoints.
            for &sym in &[
                c"vkCmdBeginRendering",
                c"vkCmdBeginRenderingKHR",
                c"vkCmdEndRendering",
                c"vkCmdEndRenderingKHR",
                c"vkCmdPipelineBarrier2",
                c"vkCmdPipelineBarrier2KHR",
                c"vkQueueSubmit2",
                c"vkQueueSubmit2KHR",
                c"vkGetBufferDeviceAddress",
                c"vkGetBufferDeviceAddressKHR",
                c"vkCmdCopyBuffer2",
                c"vkCmdCopyBuffer2KHR",
                c"vkGetDeviceBufferMemoryRequirements",
                c"vkGetDeviceBufferMemoryRequirementsKHR",
                c"vkSignalSemaphore",
                c"vkSignalSemaphoreKHR",
            ] {
                let p = vkGetInstanceProcAddr(inst, sym.as_ptr());
                assert!(
                    !p.is_null(),
                    "vkGetInstanceProcAddr returned NULL for {}",
                    sym.to_string_lossy()
                );
            }

            // 5. vkCreateShaderModule with a minimal valid SPIR-V 1.0 compute shader (`void main() {}`).
            const MINIMAL_COMPUTE_SPIRV: &[u32] = &[
                0x0723_0203, // SPIR-V magic
                0x0001_0000, // Version 1.0
                0x0000_0000, // Generator
                5,           // Bound (IDs 1..=4)
                0,           // Schema
                (2 << 16) | 17, 1, // OpCapability Shader
                (3 << 16) | 14, 0, 1, // OpMemoryModel Logical GLSL450
                (5 << 16) | 15, 5, 4, 0x6E69_616D, 0x0000_0000, // OpEntryPoint GLCompute %4 "main"
                (6 << 16) | 16, 4, 17, 1, 1, 1, // OpExecutionMode %4 LocalSize 1 1 1
                (2 << 16) | 19, 1, // %1 = OpTypeVoid
                (3 << 16) | 33, 2, 1, // %2 = OpTypeFunction %1
                (5 << 16) | 54, 1, 4, 0, 2, // %4 = OpFunction %1 None %2
                (2 << 16) | 248, 3, // %3 = OpLabel
                (1 << 16) | 253, // OpReturn
                (1 << 16) | 56, // OpFunctionEnd
            ];
            let sm_ci = VkShaderModuleCreateInfo {
                s_type: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: 0,
                code_size: std::mem::size_of_val(MINIMAL_COMPUTE_SPIRV),
                p_code: MINIMAL_COMPUTE_SPIRV.as_ptr(),
            };
            let mut shader_module: u64 = 0;
            assert_eq!(
                vkCreateShaderModule(device, &sm_ci, std::ptr::null(), &mut shader_module),
                VK_SUCCESS
            );
            assert_ne!(shader_module, 0);

            // 6. Clean teardown.
            assert_eq!(vkDeviceWaitIdle(device), VK_SUCCESS);
            vkDestroyShaderModule(device, shader_module, std::ptr::null());
            vkDestroyDevice(device, std::ptr::null());
            vkDestroySurfaceKHR(inst, surface, std::ptr::null());
            vkDestroyInstance(inst, std::ptr::null());
        }
    }

    #[test]
    fn dynamic_interceptors_cover_float_args_surface_sync_and_allocators() {
        // Float-argument Win64 -> AAPCS64 adapters must be intercepted for both device & instance lookups.
        for sym in [
            "vkCmdSetLineWidth",
            "vkCmdSetDepthBias",
            "vkCmdSetDepthBounds",
            "vkCmdSetExtraPrimitiveOverestimationSizeEXT",
        ] {
            assert!(
                intercepted_device_proc(sym).is_some(),
                "{sym} must be intercepted for Win64 -> AAPCS64 float ABI bridging"
            );
            assert_eq!(
                intercepted_instance_or_device_proc(sym),
                intercepted_device_proc(sym)
            );
        }

        // Surface capabilities & swapchain creation must be intercepted for CAMetalLayer resize sync.
        assert_eq!(
            intercepted_instance_or_device_proc("vkGetPhysicalDeviceSurfaceCapabilitiesKHR"),
            Some(vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const ())
        );
        assert_eq!(
            intercepted_instance_or_device_proc("vkGetPhysicalDeviceSurfaceCapabilities2KHR"),
            Some(tramp_vkGetPhysicalDeviceSurfaceCapabilities2KHR as *const ())
        );
        assert_eq!(
            intercepted_device_proc("vkCreateSwapchainKHR"),
            Some(vkCreateSwapchainKHR as *const ())
        );

        // Allocator-taking device creates/destroys must be intercepted for guest VkAllocationCallbacks safety.
        for sym in [
            "vkCreateBuffer",
            "vkDestroyBuffer",
            "vkCreateImage",
            "vkDestroyImage",
            "vkCreateImageView",
            "vkDestroyImageView",
            "vkCreateRenderPass",
            "vkDestroyRenderPass",
            "vkCreateFramebuffer",
            "vkDestroyFramebuffer",
            "vkCreateGraphicsPipelines",
            "vkCreateComputePipelines",
            "vkDestroyPipeline",
            "vkAllocateMemory",
            "vkFreeMemory",
            "vkCreateShaderModule",
            "vkDestroyShaderModule",
            "vkCreateCommandPool",
            "vkDestroyCommandPool",
            "vkCreateSemaphore",
            "vkDestroySemaphore",
            "vkCreateFence",
            "vkDestroyFence",
        ] {
            assert!(
                intercepted_device_proc(sym).is_some(),
                "{sym} must be intercepted to sanitize guest VkAllocationCallbacks"
            );
        }

        // Pure integer/pointer hot commands without allocators or floats pass directly to MoltenVK.
        for sym in ["vkCmdDraw", "vkCmdDrawIndexed", "vkQueueSubmit", "vkAcquireNextImageKHR"] {
            assert!(
                intercepted_device_proc(sym).is_none(),
                "{sym} should dispatch directly to MoltenVK without wrapper overhead"
            );
        }

        // Null command buffer safely no-ops even with poisoned xmm0 on aarch64.
        unsafe {
            #[cfg(target_arch = "aarch64")]
            {
                let f1 = f64::from_bits(1.0f32.to_bits() as u64);
                vkCmdSetLineWidth(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64::NAN, f1, 0.0, 0.0);
                vkCmdSetDepthBias(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64::NAN, 0.0, 0.0, 0.0);
                vkCmdSetDepthBounds(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64::NAN, 0.0, f1, 0.0);
                tramp_vkCmdSetExtraPrimitiveOverestimationSizeEXT(
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64::NAN, 0.0, 0.0, 0.0,
                );
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                vkCmdSetLineWidth(0, 1.0);
                vkCmdSetDepthBias(0, 0.0, 0.0, 0.0);
                vkCmdSetDepthBounds(0, 0.0, 1.0);
                tramp_vkCmdSetExtraPrimitiveOverestimationSizeEXT(0, 0.0);
            }
        }
    }
}
