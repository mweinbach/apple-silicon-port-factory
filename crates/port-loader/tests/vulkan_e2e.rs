//! End-to-end live Vulkan test: compiles a real Windows x86-64 Vulkan PE `.exe`
//! on the fly with `zig cc -target x86_64-windows-gnu` and runs it through
//! `port-runner` on Apple Silicon with real `libMoltenVK.dylib`.
//!
//! Covers:
//! - Real Win32 `RegisterClassExW` + `CreateWindowExW` -> AppKit `NSWindow` + `CAMetalLayer`
//!   (and headless `PORT_HEADLESS=1` offscreen `CAMetalLayer`)
//! - Both static PE imports from `vulkan-1.dll` and dynamic `LoadLibraryA` + `GetProcAddress` +
//!   `vkGetInstanceProcAddr` + `vkGetDeviceProcAddr` through `x86-runtime` `VA_DISPATCH`
//! - `VK_KHR_win32_surface` -> `VK_EXT_metal_surface` translation (`vkCreateWin32SurfaceKHR`)
//! - Multi-argument (>4, 6, 10 args) and float-argument (`vkCmdSetLineWidth`, `vkCmdSetDepthBias`)
//!   Win64 -> ARM64 AAPCS64 dispatch
//! - Guest `VkAllocationCallbacks` sanitization across dynamic `vkGetDeviceProcAddr` create/destroy calls
//! - SPIR-V vertex + fragment shader compilation, graphics pipeline, 3 pre-resize frames presented
//!   via `vkQueuePresentKHR`, and GPU pixel readback verifying BOTH clear color and drawn triangle
//! - Win32 window resize (`SetWindowPos`) -> `vkDeviceWaitIdle` -> updated surface capabilities
//!   (`vkGetPhysicalDeviceSurfaceCapabilitiesKHR` / `2KHR`) -> swapchain recreation passing
//!   `oldSwapchain` -> 3 post-resize frames presented -> post-resize GPU pixel readback
//!
//! Gated behind `PORT_TEST_MOLTENVK=1` (skips cleanly if unset, if `zig` is not installed, or if
//! no `libMoltenVK.dylib` is found — never fakes success and never commits `.exe` binaries).

#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

struct TempDirGuard(PathBuf);

impl TempDirGuard {
    fn new(prefix: &str) -> Self {
        let id = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("{prefix}-{}-{id}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn find_zig() -> Option<PathBuf> {
    let mut candidates = vec![PathBuf::from("zig")];
    candidates.push(PathBuf::from("/opt/homebrew/bin/zig"));
    candidates.push(PathBuf::from("/usr/local/bin/zig"));
    for cand in candidates {
        if Command::new(&cand)
            .arg("version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
        {
            return Some(cand);
        }
    }
    None
}

fn find_moltenvk_dylib() -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    for var in ["PORT_VULKAN_DYLIB", "PORT_MOLTENVK_DYLIB", "MOLTENVK_DYLIB"] {
        if let Some(v) = std::env::var_os(var).filter(|v| !v.is_empty()) {
            candidates.push(PathBuf::from(v));
        }
    }
    candidates.push(PathBuf::from("/opt/homebrew/lib/libMoltenVK.dylib"));
    candidates.push(PathBuf::from("/usr/local/lib/libMoltenVK.dylib"));
    if let Some(sdk) = std::env::var_os("VULKAN_SDK").filter(|v| !v.is_empty()) {
        candidates.push(PathBuf::from(sdk).join("lib/libMoltenVK.dylib"));
    }
    for android_var in ["ANDROID_HOME", "ANDROID_SDK_ROOT"] {
        if let Some(sdk) = std::env::var_os(android_var).filter(|v| !v.is_empty()) {
            candidates.push(PathBuf::from(sdk).join("emulator/lib64/vulkan/libMoltenVK.dylib"));
        }
    }
    if let Some(home) = std::env::var_os("HOME").filter(|v| !v.is_empty()) {
        candidates.push(
            PathBuf::from(home).join("Library/Android/sdk/emulator/lib64/vulkan/libMoltenVK.dylib"),
        );
    }
    candidates.into_iter().find(|p| p.is_file())
}

const VULKAN_DEF: &str = r#"LIBRARY vulkan-1.dll
EXPORTS
  vkEnumerateInstanceVersion
  vkEnumerateInstanceExtensionProperties
  vkCreateInstance
  vkDestroyInstance
  vkGetInstanceProcAddr
  vkGetDeviceProcAddr
  vkCmdSetLineWidth
  vkCmdSetDepthBias
"#;

const VULKAN_E2E_C: &str = r##"#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* SPIR-V 1.0 Vertex Shader: centered triangle (-0.6,-0.6), (0.6,-0.6), (0.0,0.6) via gl_VertexIndex */
static const uint32_t VERT_SPV[172] = {
    0x07230203u, 0x00010000u, 0x00000000u, 0x00000021u, 0x00000000u, 0x00020011u, 0x00000001u, 0x0003000Eu,
    0x00000000u, 0x00000001u, 0x0007000Fu, 0x00000000u, 0x00000003u, 0x6E69616Du, 0x00000000u, 0x00000009u,
    0x0000000Du, 0x00050048u, 0x00000007u, 0x00000000u, 0x0000000Bu, 0x00000000u, 0x00030047u, 0x00000007u,
    0x00000002u, 0x00040047u, 0x0000000Du, 0x0000000Bu, 0x0000002Au, 0x00020013u, 0x00000001u, 0x00030021u,
    0x00000002u, 0x00000001u, 0x00030016u, 0x00000005u, 0x00000020u, 0x00040017u, 0x00000006u, 0x00000005u,
    0x00000004u, 0x0003001Eu, 0x00000007u, 0x00000006u, 0x00040020u, 0x00000008u, 0x00000003u, 0x00000007u,
    0x0004003Bu, 0x00000008u, 0x00000009u, 0x00000003u, 0x00040015u, 0x0000000Au, 0x00000020u, 0x00000001u,
    0x0004002Bu, 0x0000000Au, 0x0000000Bu, 0x00000000u, 0x00040020u, 0x0000000Cu, 0x00000001u, 0x0000000Au,
    0x0004003Bu, 0x0000000Cu, 0x0000000Du, 0x00000001u, 0x00040015u, 0x0000000Eu, 0x00000020u, 0x00000000u,
    0x0004002Bu, 0x0000000Eu, 0x0000000Fu, 0x00000003u, 0x0004001Cu, 0x00000010u, 0x00000006u, 0x0000000Fu,
    0x0004002Bu, 0x00000005u, 0x00000011u, 0xBF19999Au, 0x0004002Bu, 0x00000005u, 0x00000012u, 0x3F19999Au,
    0x0004002Bu, 0x00000005u, 0x00000013u, 0x00000000u, 0x0004002Bu, 0x00000005u, 0x00000014u, 0x3F800000u,
    0x0007002Cu, 0x00000006u, 0x00000015u, 0x00000011u, 0x00000011u, 0x00000013u, 0x00000014u, 0x0007002Cu,
    0x00000006u, 0x00000016u, 0x00000012u, 0x00000011u, 0x00000013u, 0x00000014u, 0x0007002Cu, 0x00000006u,
    0x00000017u, 0x00000013u, 0x00000012u, 0x00000013u, 0x00000014u, 0x0006002Cu, 0x00000010u, 0x00000018u,
    0x00000015u, 0x00000016u, 0x00000017u, 0x00040020u, 0x00000019u, 0x00000007u, 0x00000010u, 0x00040020u,
    0x0000001Au, 0x00000007u, 0x00000006u, 0x00040020u, 0x0000001Bu, 0x00000003u, 0x00000006u, 0x00050036u,
    0x00000001u, 0x00000003u, 0x00000000u, 0x00000002u, 0x000200F8u, 0x00000004u, 0x0004003Bu, 0x00000019u,
    0x0000001Cu, 0x00000007u, 0x0003003Eu, 0x0000001Cu, 0x00000018u, 0x0004003Du, 0x0000000Au, 0x0000001Du,
    0x0000000Du, 0x00050041u, 0x0000001Au, 0x0000001Eu, 0x0000001Cu, 0x0000001Du, 0x0004003Du, 0x00000006u,
    0x0000001Fu, 0x0000001Eu, 0x00050041u, 0x0000001Bu, 0x00000020u, 0x00000009u, 0x0000000Bu, 0x0003003Eu,
    0x00000020u, 0x0000001Fu, 0x000100FDu, 0x00010038u,
};

/* SPIR-V 1.0 Fragment Shader: outputs vivid orange vec4(1.0, 0.5, 0.0, 1.0) at location 0 */
static const uint32_t FRAG_SPV[74] = {
    0x07230203u, 0x00010000u, 0x00000000u, 0x0000000Du, 0x00000000u, 0x00020011u, 0x00000001u, 0x0003000Eu,
    0x00000000u, 0x00000001u, 0x0006000Fu, 0x00000004u, 0x00000003u, 0x6E69616Du, 0x00000000u, 0x00000008u,
    0x00030010u, 0x00000003u, 0x00000007u, 0x00040047u, 0x00000008u, 0x0000001Eu, 0x00000000u, 0x00020013u,
    0x00000001u, 0x00030021u, 0x00000002u, 0x00000001u, 0x00030016u, 0x00000005u, 0x00000020u, 0x00040017u,
    0x00000006u, 0x00000005u, 0x00000004u, 0x00040020u, 0x00000007u, 0x00000003u, 0x00000006u, 0x0004003Bu,
    0x00000007u, 0x00000008u, 0x00000003u, 0x0004002Bu, 0x00000005u, 0x00000009u, 0x3F800000u, 0x0004002Bu,
    0x00000005u, 0x0000000Au, 0x3F000000u, 0x0004002Bu, 0x00000005u, 0x0000000Bu, 0x00000000u, 0x0007002Cu,
    0x00000006u, 0x0000000Cu, 0x00000009u, 0x0000000Au, 0x0000000Bu, 0x00000009u, 0x00050036u, 0x00000001u,
    0x00000003u, 0x00000000u, 0x00000002u, 0x000200F8u, 0x00000004u, 0x0003003Eu, 0x00000008u, 0x0000000Cu,
    0x000100FDu, 0x00010038u,
};

/* ---- Self-contained Vulkan 1.0-1.3 + Win32 + Swapchain ABI ---- */
typedef uint32_t VkBool32;
typedef uint32_t VkFlags;
typedef uint64_t VkDeviceSize;
typedef int32_t  VkResult;

typedef void*    VkInstance;
typedef void*    VkPhysicalDevice;
typedef void*    VkDevice;
typedef void*    VkQueue;
typedef void*    VkCommandBuffer;

typedef uint64_t VkSurfaceKHR;
typedef uint64_t VkSwapchainKHR;
typedef uint64_t VkImage;
typedef uint64_t VkImageView;
typedef uint64_t VkRenderPass;
typedef uint64_t VkFramebuffer;
typedef uint64_t VkShaderModule;
typedef uint64_t VkPipelineLayout;
typedef uint64_t VkPipeline;
typedef uint64_t VkPipelineCache;
typedef uint64_t VkCommandPool;
typedef uint64_t VkSemaphore;
typedef uint64_t VkFence;
typedef uint64_t VkBuffer;
typedef uint64_t VkDeviceMemory;

typedef void (*PFN_vkVoidFunction)(void);

#define VK_SUCCESS 0
#define VK_SUBOPTIMAL_KHR 1000001003
#define VK_FALSE 0u
#define VK_TRUE  1u
#define VK_NULL_HANDLE 0ull
#define VK_SUBPASS_EXTERNAL (~0u)
#define VK_QUEUE_FAMILY_IGNORED (~0u)
#define VK_MAKE_API_VERSION(v, maj, min, pat) (((uint32_t)(v)<<29)|((uint32_t)(maj)<<22)|((uint32_t)(min)<<12)|(uint32_t)(pat))

#define VK_FORMAT_R8G8B8A8_UNORM 37u
#define VK_FORMAT_B8G8R8A8_UNORM 44u
#define VK_FORMAT_B8G8R8A8_SRGB  50u

#define VK_IMAGE_LAYOUT_UNDEFINED 0u
#define VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL 2u
#define VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL 6u
#define VK_IMAGE_LAYOUT_PRESENT_SRC_KHR 1000001002u

#define VK_IMAGE_USAGE_TRANSFER_SRC_BIT 0x00000001u
#define VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT 0x00000010u
#define VK_IMAGE_ASPECT_COLOR_BIT 0x00000001u
#define VK_IMAGE_VIEW_TYPE_2D 1u

#define VK_BUFFER_USAGE_TRANSFER_DST_BIT 0x00000002u
#define VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT 0x00000002u
#define VK_MEMORY_PROPERTY_HOST_COHERENT_BIT 0x00000004u

#define VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT 0x00000400u
#define VK_PIPELINE_STAGE_TRANSFER_BIT 0x00001000u
#define VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT 0x00002000u

#define VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT 0x00000100u
#define VK_ACCESS_TRANSFER_READ_BIT 0x00000800u

#define VK_SHADER_STAGE_VERTEX_BIT 0x00000001u
#define VK_SHADER_STAGE_FRAGMENT_BIT 0x00000010u

#define VK_DYNAMIC_STATE_VIEWPORT 0u
#define VK_DYNAMIC_STATE_SCISSOR 1u
#define VK_DYNAMIC_STATE_LINE_WIDTH 2u
#define VK_DYNAMIC_STATE_DEPTH_BIAS 3u

typedef struct { uint32_t width, height; } VkExtent2D;
typedef struct { uint32_t width, height, depth; } VkExtent3D;
typedef struct { int32_t x, y; } VkOffset2D;
typedef struct { int32_t x, y, z; } VkOffset3D;
typedef struct { VkOffset2D offset; VkExtent2D extent; } VkRect2D;
typedef struct { float x, y, width, height, minDepth, maxDepth; } VkViewport;
typedef union { float float32[4]; int32_t int32[4]; uint32_t uint32[4]; } VkClearColorValue;
typedef union { VkClearColorValue color; struct { float depth; uint32_t stencil; } depthStencil; } VkClearValue;
typedef struct { uint32_t r, g, b, a; } VkComponentMapping;
typedef struct { VkFlags aspectMask; uint32_t baseMipLevel, levelCount, baseArrayLayer, layerCount; } VkImageSubresourceRange;
typedef struct { VkFlags aspectMask; uint32_t mipLevel, baseArrayLayer, layerCount; } VkImageSubresourceLayers;

typedef struct {
    void* pUserData;
    void* (*pfnAllocation)(void*, size_t, size_t, uint32_t);
    void* (*pfnReallocation)(void*, void*, size_t, size_t, uint32_t);
    void  (*pfnFree)(void*, void*);
    void  (*pfnInternalAllocation)(void*, size_t, uint32_t, uint32_t);
    void  (*pfnInternalFree)(void*, size_t, uint32_t, uint32_t);
} VkAllocationCallbacks;

typedef struct { char extensionName[256]; uint32_t specVersion; } VkExtensionProperties;
typedef struct { uint32_t sType; const void* pNext; const char* pApplicationName; uint32_t applicationVersion; const char* pEngineName; uint32_t engineVersion; uint32_t apiVersion; } VkApplicationInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; const VkApplicationInfo* pApplicationInfo; uint32_t enabledLayerCount; const char* const* ppEnabledLayerNames; uint32_t enabledExtensionCount; const char* const* ppEnabledExtensionNames; } VkInstanceCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; HINSTANCE hinstance; HWND hwnd; } VkWin32SurfaceCreateInfoKHR;
typedef struct { uint32_t apiVersion; uint32_t driverVersion; uint32_t vendorID; uint32_t deviceID; uint32_t deviceType; char deviceName[256]; uint8_t pipelineCacheUUID[16]; uint8_t rest[1024]; } VkPhysicalDeviceProperties;
typedef struct { VkFlags queueFlags; uint32_t queueCount; uint32_t timestampValidBits; VkExtent3D minImageTransferGranularity; } VkQueueFamilyProperties;
typedef struct { uint32_t minImageCount; uint32_t maxImageCount; VkExtent2D currentExtent; VkExtent2D minImageExtent; VkExtent2D maxImageExtent; uint32_t maxImageArrayLayers; VkFlags supportedTransforms; uint32_t currentTransform; VkFlags supportedCompositeAlpha; VkFlags supportedUsageFlags; } VkSurfaceCapabilitiesKHR;
typedef struct { uint32_t sType; const void* pNext; VkSurfaceKHR surface; } VkPhysicalDeviceSurfaceInfo2KHR;
typedef struct { uint32_t sType; void* pNext; VkSurfaceCapabilitiesKHR surfaceCapabilities; } VkSurfaceCapabilities2KHR;
typedef struct { uint32_t format; uint32_t colorSpace; } VkSurfaceFormatKHR;
typedef struct { VkFlags propertyFlags; uint32_t heapIndex; } VkMemoryType;
typedef struct { VkDeviceSize size; VkFlags flags; } VkMemoryHeap;
typedef struct { uint32_t memoryTypeCount; VkMemoryType memoryTypes[32]; uint32_t memoryHeapCount; VkMemoryHeap memoryHeaps[16]; } VkPhysicalDeviceMemoryProperties;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t queueFamilyIndex; uint32_t queueCount; const float* pQueuePriorities; } VkDeviceQueueCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t queueCreateInfoCount; const VkDeviceQueueCreateInfo* pQueueCreateInfos; uint32_t enabledLayerCount; const char* const* ppEnabledLayerNames; uint32_t enabledExtensionCount; const char* const* ppEnabledExtensionNames; const void* pEnabledFeatures; } VkDeviceCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; VkSurfaceKHR surface; uint32_t minImageCount; uint32_t imageFormat; uint32_t imageColorSpace; VkExtent2D imageExtent; uint32_t imageArrayLayers; VkFlags imageUsage; uint32_t imageSharingMode; uint32_t queueFamilyIndexCount; const uint32_t* pQueueFamilyIndices; uint32_t preTransform; uint32_t compositeAlpha; uint32_t presentMode; VkBool32 clipped; VkSwapchainKHR oldSwapchain; } VkSwapchainCreateInfoKHR;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; VkImage image; uint32_t viewType; uint32_t format; VkComponentMapping components; VkImageSubresourceRange subresourceRange; } VkImageViewCreateInfo;
typedef struct { VkFlags flags; uint32_t format; uint32_t samples; uint32_t loadOp; uint32_t storeOp; uint32_t stencilLoadOp; uint32_t stencilStoreOp; uint32_t initialLayout; uint32_t finalLayout; } VkAttachmentDescription;
typedef struct { uint32_t attachment; uint32_t layout; } VkAttachmentReference;
typedef struct { VkFlags flags; uint32_t pipelineBindPoint; uint32_t inputAttachmentCount; const VkAttachmentReference* pInputAttachments; uint32_t colorAttachmentCount; const VkAttachmentReference* pColorAttachments; const VkAttachmentReference* pResolveAttachments; const VkAttachmentReference* pDepthStencilAttachment; uint32_t preserveAttachmentCount; const uint32_t* pPreserveAttachments; } VkSubpassDescription;
typedef struct { uint32_t srcSubpass; uint32_t dstSubpass; VkFlags srcStageMask; VkFlags dstStageMask; VkFlags srcAccessMask; VkFlags dstAccessMask; VkFlags dependencyFlags; } VkSubpassDependency;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t attachmentCount; const VkAttachmentDescription* pAttachments; uint32_t subpassCount; const VkSubpassDescription* pSubpasses; uint32_t dependencyCount; const VkSubpassDependency* pDependencies; } VkRenderPassCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; VkRenderPass renderPass; uint32_t attachmentCount; const VkImageView* pAttachments; uint32_t width; uint32_t height; uint32_t layers; } VkFramebufferCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; size_t codeSize; const uint32_t* pCode; } VkShaderModuleCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t stage; VkShaderModule module; const char* pName; const void* pSpecializationInfo; } VkPipelineShaderStageCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t vertexBindingDescriptionCount; const void* pVertexBindingDescriptions; uint32_t vertexAttributeDescriptionCount; const void* pVertexAttributeDescriptions; } VkPipelineVertexInputStateCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t topology; VkBool32 primitiveRestartEnable; } VkPipelineInputAssemblyStateCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t viewportCount; const VkViewport* pViewports; uint32_t scissorCount; const VkRect2D* pScissors; } VkPipelineViewportStateCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; VkBool32 depthClampEnable; VkBool32 rasterizerDiscardEnable; uint32_t polygonMode; VkFlags cullMode; uint32_t frontFace; VkBool32 depthBiasEnable; float depthBiasConstantFactor; float depthBiasClamp; float depthBiasSlopeFactor; float lineWidth; } VkPipelineRasterizationStateCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t rasterizationSamples; VkBool32 sampleShadingEnable; float minSampleShading; const uint32_t* pSampleMask; VkBool32 alphaToCoverageEnable; VkBool32 alphaToOneEnable; } VkPipelineMultisampleStateCreateInfo;
typedef struct { VkBool32 blendEnable; uint32_t srcColorBlendFactor; uint32_t dstColorBlendFactor; uint32_t colorBlendOp; uint32_t srcAlphaBlendFactor; uint32_t dstAlphaBlendFactor; uint32_t alphaBlendOp; VkFlags colorWriteMask; } VkPipelineColorBlendAttachmentState;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; VkBool32 logicOpEnable; uint32_t logicOp; uint32_t attachmentCount; const VkPipelineColorBlendAttachmentState* pAttachments; float blendConstants[4]; } VkPipelineColorBlendStateCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t dynamicStateCount; const uint32_t* pDynamicStates; } VkPipelineDynamicStateCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t setLayoutCount; const void* pSetLayouts; uint32_t pushConstantRangeCount; const void* pPushConstantRanges; } VkPipelineLayoutCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t stageCount; const VkPipelineShaderStageCreateInfo* pStages; const VkPipelineVertexInputStateCreateInfo* pVertexInputState; const VkPipelineInputAssemblyStateCreateInfo* pInputAssemblyState; const void* pTessellationState; const VkPipelineViewportStateCreateInfo* pViewportState; const VkPipelineRasterizationStateCreateInfo* pRasterizationState; const VkPipelineMultisampleStateCreateInfo* pMultisampleState; const void* pDepthStencilState; const VkPipelineColorBlendStateCreateInfo* pColorBlendState; const VkPipelineDynamicStateCreateInfo* pDynamicState; VkPipelineLayout layout; VkRenderPass renderPass; uint32_t subpass; VkPipeline basePipelineHandle; int32_t basePipelineIndex; } VkGraphicsPipelineCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; uint32_t queueFamilyIndex; } VkCommandPoolCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkCommandPool commandPool; uint32_t level; uint32_t commandBufferCount; } VkCommandBufferAllocateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; const void* pInheritanceInfo; } VkCommandBufferBeginInfo;
typedef struct { uint32_t sType; const void* pNext; VkRenderPass renderPass; VkFramebuffer framebuffer; VkRect2D renderArea; uint32_t clearValueCount; const VkClearValue* pClearValues; } VkRenderPassBeginInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags srcAccessMask; VkFlags dstAccessMask; uint32_t oldLayout; uint32_t newLayout; uint32_t srcQueueFamilyIndex; uint32_t dstQueueFamilyIndex; VkImage image; VkImageSubresourceRange subresourceRange; } VkImageMemoryBarrier;
typedef struct { VkDeviceSize bufferOffset; uint32_t bufferRowLength; uint32_t bufferImageHeight; VkImageSubresourceLayers imageSubresource; VkOffset3D imageOffset; VkExtent3D imageExtent; } VkBufferImageCopy;
typedef struct { uint32_t sType; const void* pNext; uint32_t waitSemaphoreCount; const VkSemaphore* pWaitSemaphores; const VkFlags* pWaitDstStageMask; uint32_t commandBufferCount; const VkCommandBuffer* pCommandBuffers; uint32_t signalSemaphoreCount; const VkSemaphore* pSignalSemaphores; } VkSubmitInfo;
typedef struct { uint32_t sType; const void* pNext; uint32_t waitSemaphoreCount; const VkSemaphore* pWaitSemaphores; uint32_t swapchainCount; const VkSwapchainKHR* pSwapchains; const uint32_t* pImageIndices; VkResult* pResults; } VkPresentInfoKHR;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; } VkSemaphoreCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; } VkFenceCreateInfo;
typedef struct { uint32_t sType; const void* pNext; VkFlags flags; VkDeviceSize size; VkFlags usage; uint32_t sharingMode; uint32_t queueFamilyIndexCount; const uint32_t* pQueueFamilyIndices; } VkBufferCreateInfo;
typedef struct { VkDeviceSize size; VkDeviceSize alignment; uint32_t memoryTypeBits; } VkMemoryRequirements;
typedef struct { uint32_t sType; const void* pNext; VkDeviceSize allocationSize; uint32_t memoryTypeIndex; } VkMemoryAllocateInfo;

/* Statically imported from vulkan-1.dll via import lib */
__declspec(dllimport) VkResult vkEnumerateInstanceVersion(uint32_t* pApiVersion);
__declspec(dllimport) VkResult vkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pCount, VkExtensionProperties* pProps);
__declspec(dllimport) VkResult vkCreateInstance(const VkInstanceCreateInfo* pCI, const void* pAlloc, VkInstance* pInst);
__declspec(dllimport) void     vkDestroyInstance(VkInstance inst, const void* pAlloc);
__declspec(dllimport) PFN_vkVoidFunction vkGetInstanceProcAddr(VkInstance inst, const char* pName);
__declspec(dllimport) PFN_vkVoidFunction vkGetDeviceProcAddr(VkDevice dev, const char* pName);
__declspec(dllimport) void     vkCmdSetLineWidth(VkCommandBuffer cmd, float lineWidth);
__declspec(dllimport) void     vkCmdSetDepthBias(VkCommandBuffer cmd, float constantFactor, float clamp, float slopeFactor);

typedef PFN_vkVoidFunction (*PFN_vkGetInstanceProcAddr)(VkInstance, const char*);
typedef PFN_vkVoidFunction (*PFN_vkGetDeviceProcAddr)(VkDevice, const char*);
typedef VkResult (*PFN_vkCreateWin32SurfaceKHR)(VkInstance, const VkWin32SurfaceCreateInfoKHR*, const void*, VkSurfaceKHR*);
typedef void     (*PFN_vkDestroySurfaceKHR)(VkInstance, VkSurfaceKHR, const void*);
typedef VkBool32 (*PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR)(VkPhysicalDevice, uint32_t);
typedef VkResult (*PFN_vkEnumeratePhysicalDevices)(VkInstance, uint32_t*, VkPhysicalDevice*);
typedef void     (*PFN_vkGetPhysicalDeviceProperties)(VkPhysicalDevice, VkPhysicalDeviceProperties*);
typedef void     (*PFN_vkGetPhysicalDeviceQueueFamilyProperties)(VkPhysicalDevice, uint32_t*, VkQueueFamilyProperties*);
typedef void     (*PFN_vkGetPhysicalDeviceMemoryProperties)(VkPhysicalDevice, VkPhysicalDeviceMemoryProperties*);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceSupportKHR)(VkPhysicalDevice, uint32_t, VkSurfaceKHR, VkBool32*);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(VkPhysicalDevice, VkSurfaceKHR, VkSurfaceCapabilitiesKHR*);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)(VkPhysicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR*, VkSurfaceCapabilities2KHR*);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)(VkPhysicalDevice, VkSurfaceKHR, uint32_t*, VkSurfaceFormatKHR*);
typedef VkResult (*PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)(VkPhysicalDevice, VkSurfaceKHR, uint32_t*, uint32_t*);
typedef VkResult (*PFN_vkCreateDevice)(VkPhysicalDevice, const VkDeviceCreateInfo*, const void*, VkDevice*);
typedef void     (*PFN_vkDestroyDevice)(VkDevice, const void*);
typedef VkResult (*PFN_vkDeviceWaitIdle)(VkDevice);
typedef void     (*PFN_vkGetDeviceQueue)(VkDevice, uint32_t, uint32_t, VkQueue*);
typedef VkResult (*PFN_vkCreateSwapchainKHR)(VkDevice, const VkSwapchainCreateInfoKHR*, const void*, VkSwapchainKHR*);
typedef void     (*PFN_vkDestroySwapchainKHR)(VkDevice, VkSwapchainKHR, const void*);
typedef VkResult (*PFN_vkGetSwapchainImagesKHR)(VkDevice, VkSwapchainKHR, uint32_t*, VkImage*);
typedef VkResult (*PFN_vkAcquireNextImageKHR)(VkDevice, VkSwapchainKHR, uint64_t, VkSemaphore, VkFence, uint32_t*);
typedef VkResult (*PFN_vkQueuePresentKHR)(VkQueue, const VkPresentInfoKHR*);
typedef VkResult (*PFN_vkCreateImageView)(VkDevice, const VkImageViewCreateInfo*, const void*, VkImageView*);
typedef void     (*PFN_vkDestroyImageView)(VkDevice, VkImageView, const void*);
typedef VkResult (*PFN_vkCreateRenderPass)(VkDevice, const VkRenderPassCreateInfo*, const void*, VkRenderPass*);
typedef void     (*PFN_vkDestroyRenderPass)(VkDevice, VkRenderPass, const void*);
typedef VkResult (*PFN_vkCreateFramebuffer)(VkDevice, const VkFramebufferCreateInfo*, const void*, VkFramebuffer*);
typedef void     (*PFN_vkDestroyFramebuffer)(VkDevice, VkFramebuffer, const void*);
typedef VkResult (*PFN_vkCreateShaderModule)(VkDevice, const VkShaderModuleCreateInfo*, const void*, VkShaderModule*);
typedef void     (*PFN_vkDestroyShaderModule)(VkDevice, VkShaderModule, const void*);
typedef VkResult (*PFN_vkCreatePipelineLayout)(VkDevice, const VkPipelineLayoutCreateInfo*, const void*, VkPipelineLayout*);
typedef void     (*PFN_vkDestroyPipelineLayout)(VkDevice, VkPipelineLayout, const void*);
typedef VkResult (*PFN_vkCreateGraphicsPipelines)(VkDevice, VkPipelineCache, uint32_t, const VkGraphicsPipelineCreateInfo*, const void*, VkPipeline*);
typedef void     (*PFN_vkDestroyPipeline)(VkDevice, VkPipeline, const void*);
typedef VkResult (*PFN_vkCreateCommandPool)(VkDevice, const VkCommandPoolCreateInfo*, const void*, VkCommandPool*);
typedef void     (*PFN_vkDestroyCommandPool)(VkDevice, VkCommandPool, const void*);
typedef VkResult (*PFN_vkAllocateCommandBuffers)(VkDevice, const VkCommandBufferAllocateInfo*, VkCommandBuffer*);
typedef void     (*PFN_vkFreeCommandBuffers)(VkDevice, VkCommandPool, uint32_t, const VkCommandBuffer*);
typedef VkResult (*PFN_vkBeginCommandBuffer)(VkCommandBuffer, const VkCommandBufferBeginInfo*);
typedef VkResult (*PFN_vkEndCommandBuffer)(VkCommandBuffer);
typedef VkResult (*PFN_vkResetCommandBuffer)(VkCommandBuffer, VkFlags);
typedef void     (*PFN_vkCmdBeginRenderPass)(VkCommandBuffer, const VkRenderPassBeginInfo*, uint32_t);
typedef void     (*PFN_vkCmdEndRenderPass)(VkCommandBuffer);
typedef void     (*PFN_vkCmdBindPipeline)(VkCommandBuffer, uint32_t, VkPipeline);
typedef void     (*PFN_vkCmdSetViewport)(VkCommandBuffer, uint32_t, uint32_t, const VkViewport*);
typedef void     (*PFN_vkCmdSetScissor)(VkCommandBuffer, uint32_t, uint32_t, const VkRect2D*);
typedef void     (*PFN_vkCmdSetLineWidth)(VkCommandBuffer, float);
typedef void     (*PFN_vkCmdSetDepthBias)(VkCommandBuffer, float, float, float);
typedef void     (*PFN_vkCmdDraw)(VkCommandBuffer, uint32_t, uint32_t, uint32_t, uint32_t);
typedef void     (*PFN_vkCmdPipelineBarrier)(VkCommandBuffer, VkFlags, VkFlags, VkFlags, uint32_t, const void*, uint32_t, const void*, uint32_t, const VkImageMemoryBarrier*);
typedef void     (*PFN_vkCmdCopyImageToBuffer)(VkCommandBuffer, VkImage, uint32_t, VkBuffer, uint32_t, const VkBufferImageCopy*);
typedef VkResult (*PFN_vkCreateSemaphore)(VkDevice, const VkSemaphoreCreateInfo*, const void*, VkSemaphore*);
typedef void     (*PFN_vkDestroySemaphore)(VkDevice, VkSemaphore, const void*);
typedef VkResult (*PFN_vkCreateFence)(VkDevice, const VkFenceCreateInfo*, const void*, VkFence*);
typedef void     (*PFN_vkDestroyFence)(VkDevice, VkFence, const void*);
typedef VkResult (*PFN_vkWaitForFences)(VkDevice, uint32_t, const VkFence*, VkBool32, uint64_t);
typedef VkResult (*PFN_vkResetFences)(VkDevice, uint32_t, const VkFence*);
typedef VkResult (*PFN_vkQueueSubmit)(VkQueue, uint32_t, const VkSubmitInfo*, VkFence);
typedef VkResult (*PFN_vkCreateBuffer)(VkDevice, const VkBufferCreateInfo*, const void*, VkBuffer*);
typedef void     (*PFN_vkDestroyBuffer)(VkDevice, VkBuffer, const void*);
typedef void     (*PFN_vkGetBufferMemoryRequirements)(VkDevice, VkBuffer, VkMemoryRequirements*);
typedef VkResult (*PFN_vkAllocateMemory)(VkDevice, const VkMemoryAllocateInfo*, const void*, VkDeviceMemory*);
typedef void     (*PFN_vkFreeMemory)(VkDevice, VkDeviceMemory, const void*);
typedef VkResult (*PFN_vkBindBufferMemory)(VkDevice, VkBuffer, VkDeviceMemory, VkDeviceSize);
typedef VkResult (*PFN_vkMapMemory)(VkDevice, VkDeviceMemory, VkDeviceSize, VkDeviceSize, VkFlags, void**);
typedef void     (*PFN_vkUnmapMemory)(VkDevice, VkDeviceMemory);

#define CHECK_VK(call) do { \
    VkResult _r = (call); \
    if (_r != VK_SUCCESS) { \
        fprintf(stderr, "FAIL: %s returned VkResult=%d at line %d\n", #call, (int)_r, __LINE__); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

#define CHECK_COND(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s at line %d\n", (msg), __LINE__); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

/* Guest x86-64 allocator callbacks that must NEVER be jumped to directly by native ARM64 MoltenVK. */
static void* trap_alloc(void* u, size_t sz, size_t al, uint32_t sc) {
    (void)u; (void)sz; (void)al; (void)sc;
    ExitProcess(99);
    return NULL;
}
static void* trap_realloc(void* u, void* orig, size_t sz, size_t al, uint32_t sc) {
    (void)u; (void)orig; (void)sz; (void)al; (void)sc;
    ExitProcess(99);
    return NULL;
}
static void trap_free(void* u, void* mem) {
    (void)u; (void)mem;
    ExitProcess(99);
}

static LRESULT CALLBACK wnd_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    return DefWindowProcW(hwnd, msg, wp, lp);
}

static void pump_messages(void) {
    MSG msg;
    while (PeekMessageW(&msg, NULL, 0, 0, PM_REMOVE)) {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

static uint32_t find_memory_type(const VkPhysicalDeviceMemoryProperties* props, uint32_t bits, VkFlags req) {
    for (uint32_t i = 0; i < props->memoryTypeCount; i++) {
        if ((bits & (1u << i)) && (props->memoryTypes[i].propertyFlags & req) == req) {
            return i;
        }
    }
    fprintf(stderr, "FAIL: no matching memory type (bits=0x%x req=0x%x)\n", bits, req);
    ExitProcess(1);
}

typedef struct {
    VkSwapchainKHR swapchain;
    VkExtent2D extent;
    uint32_t image_count;
    VkImage images[16];
    VkImageView views[16];
    VkFramebuffer fbs[16];
} SwapBundle;

static void destroy_swap_views_fbs(
    VkDevice dev,
    SwapBundle* sb,
    PFN_vkDestroyFramebuffer pDestroyFramebuffer,
    PFN_vkDestroyImageView pDestroyImageView
) {
    for (uint32_t i = 0; i < sb->image_count; i++) {
        if (sb->fbs[i]) pDestroyFramebuffer(dev, sb->fbs[i], NULL);
        if (sb->views[i]) pDestroyImageView(dev, sb->views[i], NULL);
        sb->fbs[i] = VK_NULL_HANDLE;
        sb->views[i] = VK_NULL_HANDLE;
    }
    sb->image_count = 0;
}

static void create_swap_bundle(
    VkPhysicalDevice phys,
    VkDevice dev,
    VkSurfaceKHR surface,
    VkRenderPass rp,
    VkSurfaceFormatKHR fmt,
    uint32_t present_mode,
    VkSwapchainKHR old_sc,
    SwapBundle* out,
    VkFlags* out_usage,
    PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR pGetCaps,
    PFN_vkCreateSwapchainKHR pCreateSwapchain,
    PFN_vkGetSwapchainImagesKHR pGetSwapchainImages,
    PFN_vkCreateImageView pCreateImageView,
    PFN_vkCreateFramebuffer pCreateFramebuffer
) {
    memset(out, 0, sizeof(*out));
    VkSurfaceCapabilitiesKHR caps;
    memset(&caps, 0, sizeof(caps));
    CHECK_VK(pGetCaps(phys, surface, &caps));
    printf("SURFACE_CAPS extent=%ux%u min=%ux%u max=%ux%u minImg=%u maxImg=%u usage=0x%x\n",
           caps.currentExtent.width, caps.currentExtent.height,
           caps.minImageExtent.width, caps.minImageExtent.height,
           caps.maxImageExtent.width, caps.maxImageExtent.height,
           caps.minImageCount, caps.maxImageCount, caps.supportedUsageFlags);
    fflush(stdout);

    VkExtent2D ext = caps.currentExtent;
    CHECK_COND(ext.width > 0 && ext.height > 0 && ext.width != 0xFFFFFFFFu, "valid surface extent");

    uint32_t img_count = caps.minImageCount + 1;
    if (caps.maxImageCount > 0 && img_count > caps.maxImageCount) {
        img_count = caps.maxImageCount;
    }

    VkFlags usage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    if (caps.supportedUsageFlags & VK_IMAGE_USAGE_TRANSFER_SRC_BIT) {
        usage |= VK_IMAGE_USAGE_TRANSFER_SRC_BIT;
    }
    *out_usage = usage;

    VkSwapchainCreateInfoKHR sci;
    memset(&sci, 0, sizeof(sci));
    sci.sType = 1000001000u;
    sci.surface = surface;
    sci.minImageCount = img_count;
    sci.imageFormat = fmt.format;
    sci.imageColorSpace = fmt.colorSpace;
    sci.imageExtent = ext;
    sci.imageArrayLayers = 1;
    sci.imageUsage = usage;
    sci.imageSharingMode = 0;
    sci.preTransform = caps.currentTransform ? caps.currentTransform : 1u;
    sci.compositeAlpha = (caps.supportedCompositeAlpha & 1u) ? 1u : (caps.supportedCompositeAlpha & 2u ? 2u : 1u);
    sci.presentMode = present_mode;
    sci.clipped = VK_TRUE;
    sci.oldSwapchain = old_sc;

    CHECK_VK(pCreateSwapchain(dev, &sci, NULL, &out->swapchain));
    CHECK_COND(out->swapchain != VK_NULL_HANDLE, "swapchain non-null");
    out->extent = ext;

    uint32_t n = 0;
    CHECK_VK(pGetSwapchainImages(dev, out->swapchain, &n, NULL));
    CHECK_COND(n >= 1 && n <= 16, "swapchain image count 1..16");
    CHECK_VK(pGetSwapchainImages(dev, out->swapchain, &n, out->images));
    out->image_count = n;

    for (uint32_t i = 0; i < n; i++) {
        VkImageViewCreateInfo ivci;
        memset(&ivci, 0, sizeof(ivci));
        ivci.sType = 15u;
        ivci.image = out->images[i];
        ivci.viewType = VK_IMAGE_VIEW_TYPE_2D;
        ivci.format = fmt.format;
        ivci.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        ivci.subresourceRange.levelCount = 1;
        ivci.subresourceRange.layerCount = 1;
        CHECK_VK(pCreateImageView(dev, &ivci, NULL, &out->views[i]));

        VkFramebufferCreateInfo fbci;
        memset(&fbci, 0, sizeof(fbci));
        fbci.sType = 37u;
        fbci.renderPass = rp;
        fbci.attachmentCount = 1;
        fbci.pAttachments = &out->views[i];
        fbci.width = ext.width;
        fbci.height = ext.height;
        fbci.layers = 1;
        CHECK_VK(pCreateFramebuffer(dev, &fbci, NULL, &out->fbs[i]));
    }
}

static void check_rgba(const char* label, const uint8_t* px, int is_bgra,
                       uint8_t exp_r, uint8_t exp_g, uint8_t exp_b, uint8_t exp_a) {
    uint8_t r = is_bgra ? px[2] : px[0];
    uint8_t g = px[1];
    uint8_t b = is_bgra ? px[0] : px[2];
    uint8_t a = px[3];
    printf("PIXEL_READBACK %s rgba=(%u,%u,%u,%u) expected=(%u,%u,%u,%u)\n",
           label, r, g, b, a, exp_r, exp_g, exp_b, exp_a);
    fflush(stdout);
    int dr = abs((int)r - (int)exp_r);
    int dg = abs((int)g - (int)exp_g);
    int db = abs((int)b - (int)exp_b);
    int da = abs((int)a - (int)exp_a);
    if (dr > 4 || dg > 4 || db > 4 || da > 4) {
        fprintf(stderr, "FAIL: pixel mismatch for %s\n", label);
        ExitProcess(1);
    }
}

int main(void) {
    printf("VK_E2E_START\n");
    fflush(stdout);

    /* 1. Create a real Win32 window (256x192 client area). */
    HINSTANCE hinst = GetModuleHandleW(NULL);
    WNDCLASSEXW wc;
    memset(&wc, 0, sizeof(wc));
    wc.cbSize = sizeof(wc);
    wc.lpfnWndProc = wnd_proc;
    wc.hInstance = hinst;
    wc.lpszClassName = L"PortFactoryVulkanE2E";
    ATOM atom = RegisterClassExW(&wc);
    CHECK_COND(atom != 0, "RegisterClassExW");

    RECT wr = { 0, 0, 256, 192 };
    AdjustWindowRectEx(&wr, WS_OVERLAPPEDWINDOW, FALSE, 0);
    HWND hwnd = CreateWindowExW(
        0, wc.lpszClassName, L"Vulkan E2E",
        WS_OVERLAPPEDWINDOW, 64, 64,
        wr.right - wr.left, wr.bottom - wr.top,
        NULL, NULL, hinst, NULL
    );
    CHECK_COND(hwnd != NULL, "CreateWindowExW");
    ShowWindow(hwnd, SW_SHOWNOACTIVATE);
    pump_messages();

    RECT cr1;
    GetClientRect(hwnd, &cr1);
    printf("WINDOW_CREATED hwnd=0x%llx client=%ldx%ld\n",
           (unsigned long long)(uintptr_t)hwnd,
           (long)(cr1.right - cr1.left), (long)(cr1.bottom - cr1.top));
    fflush(stdout);

    /* 2. Static vulkan-1.dll imports + dynamic LoadLibraryA / GetProcAddress. */
    uint32_t inst_ver = 0;
    CHECK_VK(vkEnumerateInstanceVersion(&inst_ver));
    printf("STATIC_IMPORT_OK api_version=%u.%u.%u\n",
           (inst_ver >> 22) & 0x7fu, (inst_ver >> 12) & 0x3ffu, inst_ver & 0xfffu);

    uint32_t ext_count = 0;
    CHECK_VK(vkEnumerateInstanceExtensionProperties(NULL, &ext_count, NULL));
    CHECK_COND(ext_count > 0, "instance extension count > 0");
    VkExtensionProperties* exts = (VkExtensionProperties*)calloc(ext_count, sizeof(VkExtensionProperties));
    CHECK_VK(vkEnumerateInstanceExtensionProperties(NULL, &ext_count, exts));
    int has_surface = 0, has_win32 = 0, has_caps2 = 0;
    for (uint32_t i = 0; i < ext_count; i++) {
        if (strcmp(exts[i].extensionName, "VK_KHR_surface") == 0) has_surface = 1;
        if (strcmp(exts[i].extensionName, "VK_KHR_win32_surface") == 0) has_win32 = 1;
        if (strcmp(exts[i].extensionName, "VK_KHR_get_surface_capabilities2") == 0) has_caps2 = 1;
    }
    free(exts);
    CHECK_COND(has_surface && has_win32, "VK_KHR_surface + VK_KHR_win32_surface advertised");

    HMODULE vk_mod = LoadLibraryA("vulkan-1.dll");
    CHECK_COND(vk_mod != NULL, "LoadLibraryA(vulkan-1.dll)");
    PFN_vkGetInstanceProcAddr dyn_gipa = (PFN_vkGetInstanceProcAddr)GetProcAddress(vk_mod, "vkGetInstanceProcAddr");
    CHECK_COND(dyn_gipa != NULL, "GetProcAddress(vkGetInstanceProcAddr)");
    CHECK_COND(dyn_gipa(NULL, "vkCreateInstance") != NULL, "dynamic gipa(NULL, vkCreateInstance)");
    printf("DYNAMIC_LOAD_OK\n");

    /* 3. vkCreateInstance requesting VK_KHR_surface + VK_KHR_win32_surface (+ caps2 if advertised). */
    const char* req_inst_exts[3];
    uint32_t req_inst_ext_count = 0;
    req_inst_exts[req_inst_ext_count++] = "VK_KHR_surface";
    req_inst_exts[req_inst_ext_count++] = "VK_KHR_win32_surface";
    if (has_caps2) {
        req_inst_exts[req_inst_ext_count++] = "VK_KHR_get_surface_capabilities2";
    }

    VkApplicationInfo app;
    memset(&app, 0, sizeof(app));
    app.sType = 0u;
    app.pApplicationName = "vulkan_e2e";
    app.applicationVersion = 1;
    app.pEngineName = "port-factory";
    app.engineVersion = 1;
    app.apiVersion = (inst_ver >= VK_MAKE_API_VERSION(0, 1, 3, 0))
        ? VK_MAKE_API_VERSION(0, 1, 3, 0)
        : VK_MAKE_API_VERSION(0, 1, 2, 0);

    VkInstanceCreateInfo ici;
    memset(&ici, 0, sizeof(ici));
    ici.sType = 1u;
    ici.pApplicationInfo = &app;
    ici.enabledExtensionCount = req_inst_ext_count;
    ici.ppEnabledExtensionNames = req_inst_exts;

    VkInstance instance = NULL;
    CHECK_VK(vkCreateInstance(&ici, NULL, &instance));
    CHECK_COND(instance != NULL, "vkCreateInstance non-null");
    printf("INSTANCE_CREATED\n");

    #define LOAD_IPROC(name) PFN_##name p_##name = (PFN_##name)vkGetInstanceProcAddr(instance, #name); CHECK_COND(p_##name != NULL, "gipa " #name)
    LOAD_IPROC(vkCreateWin32SurfaceKHR);
    LOAD_IPROC(vkDestroySurfaceKHR);
    LOAD_IPROC(vkGetPhysicalDeviceWin32PresentationSupportKHR);
    LOAD_IPROC(vkEnumeratePhysicalDevices);
    LOAD_IPROC(vkGetPhysicalDeviceProperties);
    LOAD_IPROC(vkGetPhysicalDeviceQueueFamilyProperties);
    LOAD_IPROC(vkGetPhysicalDeviceMemoryProperties);
    LOAD_IPROC(vkGetPhysicalDeviceSurfaceSupportKHR);
    LOAD_IPROC(vkGetPhysicalDeviceSurfaceCapabilitiesKHR);
    LOAD_IPROC(vkGetPhysicalDeviceSurfaceFormatsKHR);
    LOAD_IPROC(vkGetPhysicalDeviceSurfacePresentModesKHR);
    LOAD_IPROC(vkCreateDevice);
    LOAD_IPROC(vkGetDeviceProcAddr);
    #undef LOAD_IPROC
    PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR p_vkGetPhysicalDeviceSurfaceCapabilities2KHR =
        has_caps2 ? (PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR)vkGetInstanceProcAddr(instance, "vkGetPhysicalDeviceSurfaceCapabilities2KHR") : NULL;

    /* 4. vkCreateWin32SurfaceKHR targeting real HWND -> CAMetalLayer. */
    VkWin32SurfaceCreateInfoKHR wsci;
    memset(&wsci, 0, sizeof(wsci));
    wsci.sType = 1000009000u;
    wsci.hinstance = hinst;
    wsci.hwnd = hwnd;

    VkSurfaceKHR surface = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateWin32SurfaceKHR(instance, &wsci, NULL, &surface));
    CHECK_COND(surface != VK_NULL_HANDLE, "vkCreateWin32SurfaceKHR non-null");
    printf("WIN32_SURFACE_CREATED surface=0x%llx\n", (unsigned long long)surface);

    /* 5. Physical device + graphics/present queue family. */
    uint32_t phys_count = 0;
    CHECK_VK(p_vkEnumeratePhysicalDevices(instance, &phys_count, NULL));
    CHECK_COND(phys_count >= 1, "at least 1 physical device");
    VkPhysicalDevice phys_devs[16];
    if (phys_count > 16) phys_count = 16;
    CHECK_VK(p_vkEnumeratePhysicalDevices(instance, &phys_count, phys_devs));
    VkPhysicalDevice phys = phys_devs[0];

    VkPhysicalDeviceProperties pprops;
    memset(&pprops, 0, sizeof(pprops));
    p_vkGetPhysicalDeviceProperties(phys, &pprops);
    printf("PHYSICAL_DEVICE name=%s api=%u.%u.%u\n",
           pprops.deviceName,
           (pprops.apiVersion >> 22) & 0x7fu,
           (pprops.apiVersion >> 12) & 0x3ffu,
           pprops.apiVersion & 0xfffu);

    uint32_t qf_count = 0;
    p_vkGetPhysicalDeviceQueueFamilyProperties(phys, &qf_count, NULL);
    CHECK_COND(qf_count >= 1, "qf_count >= 1");
    VkQueueFamilyProperties* qfs = (VkQueueFamilyProperties*)calloc(qf_count, sizeof(VkQueueFamilyProperties));
    p_vkGetPhysicalDeviceQueueFamilyProperties(phys, &qf_count, qfs);
    uint32_t gfx_qf = ~0u;
    for (uint32_t i = 0; i < qf_count; i++) {
        VkBool32 win32_ok = p_vkGetPhysicalDeviceWin32PresentationSupportKHR(phys, i);
        VkBool32 surf_ok = VK_FALSE;
        CHECK_VK(p_vkGetPhysicalDeviceSurfaceSupportKHR(phys, i, surface, &surf_ok));
        if ((qfs[i].queueFlags & 1u) && win32_ok && surf_ok) {
            gfx_qf = i;
            break;
        }
    }
    free(qfs);
    CHECK_COND(gfx_qf != ~0u, "found graphics+present queue family");

    uint32_t fmt_count = 0;
    CHECK_VK(p_vkGetPhysicalDeviceSurfaceFormatsKHR(phys, surface, &fmt_count, NULL));
    CHECK_COND(fmt_count >= 1, "surface formats >= 1");
    VkSurfaceFormatKHR* fmts = (VkSurfaceFormatKHR*)calloc(fmt_count, sizeof(VkSurfaceFormatKHR));
    CHECK_VK(p_vkGetPhysicalDeviceSurfaceFormatsKHR(phys, surface, &fmt_count, fmts));
    VkSurfaceFormatKHR chosen_fmt = fmts[0];
    for (uint32_t i = 0; i < fmt_count; i++) {
        if (fmts[i].format == VK_FORMAT_B8G8R8A8_UNORM || fmts[i].format == VK_FORMAT_R8G8B8A8_UNORM) {
            chosen_fmt = fmts[i];
            break;
        }
    }
    free(fmts);
    int is_bgra = (chosen_fmt.format == VK_FORMAT_B8G8R8A8_UNORM || chosen_fmt.format == VK_FORMAT_B8G8R8A8_SRGB);
    printf("SURFACE_FORMAT format=%u colorSpace=%u bgra=%d\n", chosen_fmt.format, chosen_fmt.colorSpace, is_bgra);

    uint32_t pm_count = 0;
    CHECK_VK(p_vkGetPhysicalDeviceSurfacePresentModesKHR(phys, surface, &pm_count, NULL));
    CHECK_COND(pm_count >= 1, "present modes >= 1");
    uint32_t* pms = (uint32_t*)calloc(pm_count, sizeof(uint32_t));
    CHECK_VK(p_vkGetPhysicalDeviceSurfacePresentModesKHR(phys, surface, &pm_count, pms));
    uint32_t chosen_pm = 2u; /* FIFO */
    for (uint32_t i = 0; i < pm_count; i++) {
        if (pms[i] == 0u) { chosen_pm = 0u; break; } /* IMMEDIATE if supported */
    }
    free(pms);

    /* 6. vkCreateDevice with VK_KHR_swapchain + dynamic vkGetDeviceProcAddr resolution. */
    float prio = 1.0f;
    VkDeviceQueueCreateInfo dqci;
    memset(&dqci, 0, sizeof(dqci));
    dqci.sType = 2u;
    dqci.queueFamilyIndex = gfx_qf;
    dqci.queueCount = 1;
    dqci.pQueuePriorities = &prio;

    const char* dev_exts[1] = { "VK_KHR_swapchain" };
    VkDeviceCreateInfo dci;
    memset(&dci, 0, sizeof(dci));
    dci.sType = 3u;
    dci.queueCreateInfoCount = 1;
    dci.pQueueCreateInfos = &dqci;
    dci.enabledExtensionCount = 1;
    dci.ppEnabledExtensionNames = dev_exts;

    VkDevice device = NULL;
    CHECK_VK(p_vkCreateDevice(phys, &dci, NULL, &device));
    CHECK_COND(device != NULL, "vkCreateDevice non-null");
    printf("DEVICE_CREATED\n");

    #define LOAD_DPROC(name) PFN_##name p_##name = (PFN_##name)p_vkGetDeviceProcAddr(device, #name); CHECK_COND(p_##name != NULL, "gdpa " #name)
    LOAD_DPROC(vkDestroyDevice);
    LOAD_DPROC(vkDeviceWaitIdle);
    LOAD_DPROC(vkGetDeviceQueue);
    LOAD_DPROC(vkCreateSwapchainKHR);
    LOAD_DPROC(vkDestroySwapchainKHR);
    LOAD_DPROC(vkGetSwapchainImagesKHR);
    LOAD_DPROC(vkAcquireNextImageKHR);
    LOAD_DPROC(vkQueuePresentKHR);
    LOAD_DPROC(vkCreateImageView);
    LOAD_DPROC(vkDestroyImageView);
    LOAD_DPROC(vkCreateRenderPass);
    LOAD_DPROC(vkDestroyRenderPass);
    LOAD_DPROC(vkCreateFramebuffer);
    LOAD_DPROC(vkDestroyFramebuffer);
    LOAD_DPROC(vkCreateShaderModule);
    LOAD_DPROC(vkDestroyShaderModule);
    LOAD_DPROC(vkCreatePipelineLayout);
    LOAD_DPROC(vkDestroyPipelineLayout);
    LOAD_DPROC(vkCreateGraphicsPipelines);
    LOAD_DPROC(vkDestroyPipeline);
    LOAD_DPROC(vkCreateCommandPool);
    LOAD_DPROC(vkDestroyCommandPool);
    LOAD_DPROC(vkAllocateCommandBuffers);
    LOAD_DPROC(vkFreeCommandBuffers);
    LOAD_DPROC(vkBeginCommandBuffer);
    LOAD_DPROC(vkEndCommandBuffer);
    LOAD_DPROC(vkResetCommandBuffer);
    LOAD_DPROC(vkCmdBeginRenderPass);
    LOAD_DPROC(vkCmdEndRenderPass);
    LOAD_DPROC(vkCmdBindPipeline);
    LOAD_DPROC(vkCmdSetViewport);
    LOAD_DPROC(vkCmdSetScissor);
    LOAD_DPROC(vkCmdSetLineWidth);
    LOAD_DPROC(vkCmdSetDepthBias);
    LOAD_DPROC(vkCmdDraw);
    LOAD_DPROC(vkCmdPipelineBarrier);
    LOAD_DPROC(vkCmdCopyImageToBuffer);
    LOAD_DPROC(vkCreateSemaphore);
    LOAD_DPROC(vkDestroySemaphore);
    LOAD_DPROC(vkCreateFence);
    LOAD_DPROC(vkDestroyFence);
    LOAD_DPROC(vkWaitForFences);
    LOAD_DPROC(vkResetFences);
    LOAD_DPROC(vkQueueSubmit);
    LOAD_DPROC(vkCreateBuffer);
    LOAD_DPROC(vkDestroyBuffer);
    LOAD_DPROC(vkGetBufferMemoryRequirements);
    LOAD_DPROC(vkAllocateMemory);
    LOAD_DPROC(vkFreeMemory);
    LOAD_DPROC(vkBindBufferMemory);
    LOAD_DPROC(vkMapMemory);
    LOAD_DPROC(vkUnmapMemory);
    #undef LOAD_DPROC

    if (pprops.apiVersion >= VK_MAKE_API_VERSION(0, 1, 3, 0)) {
        CHECK_COND(p_vkGetDeviceProcAddr(device, "vkCmdPipelineBarrier2") != NULL ||
                   p_vkGetDeviceProcAddr(device, "vkCmdPipelineBarrier2KHR") != NULL,
                   "vkCmdPipelineBarrier2(KHR) resolvable");
        CHECK_COND(p_vkGetDeviceProcAddr(device, "vkCmdBeginRendering") != NULL ||
                   p_vkGetDeviceProcAddr(device, "vkCmdBeginRenderingKHR") != NULL,
                   "vkCmdBeginRendering(KHR) resolvable");
    }
    printf("DEVICE_PROCS_RESOLVED\n");

    /* Verify non-null guest VkAllocationCallbacks are safely sanitized on dynamic device procs */
    VkAllocationCallbacks guest_alloc;
    memset(&guest_alloc, 0, sizeof(guest_alloc));
    guest_alloc.pfnAllocation = trap_alloc;
    guest_alloc.pfnReallocation = trap_realloc;
    guest_alloc.pfnFree = trap_free;
    {
        VkBufferCreateInfo tbci;
        memset(&tbci, 0, sizeof(tbci));
        tbci.sType = 12u;
        tbci.size = 256u;
        tbci.usage = VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        VkBuffer tmp_buf = VK_NULL_HANDLE;
        CHECK_VK(p_vkCreateBuffer(device, &tbci, &guest_alloc, &tmp_buf));
        CHECK_COND(tmp_buf != VK_NULL_HANDLE, "guest_alloc vkCreateBuffer");
        p_vkDestroyBuffer(device, tmp_buf, &guest_alloc);

        VkSemaphoreCreateInfo tsci = { 9u, NULL, 0u };
        VkSemaphore tmp_sem = VK_NULL_HANDLE;
        CHECK_VK(p_vkCreateSemaphore(device, &tsci, &guest_alloc, &tmp_sem));
        CHECK_COND(tmp_sem != VK_NULL_HANDLE, "guest_alloc vkCreateSemaphore");
        p_vkDestroySemaphore(device, tmp_sem, &guest_alloc);
    }
    printf("ALLOCATOR_SANITIZE_OK\n");

    VkQueue queue = NULL;
    p_vkGetDeviceQueue(device, gfx_qf, 0, &queue);
    CHECK_COND(queue != NULL, "vkGetDeviceQueue");

    /* 7. Render pass + SPIR-V shaders + graphics pipeline */
    VkAttachmentDescription color_att;
    memset(&color_att, 0, sizeof(color_att));
    color_att.format = chosen_fmt.format;
    color_att.samples = 1u;
    color_att.loadOp = 1u;
    color_att.storeOp = 0u;
    color_att.stencilLoadOp = 2u;
    color_att.stencilStoreOp = 1u;
    color_att.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    color_att.finalLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;

    VkAttachmentReference color_ref = { 0u, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL };
    VkSubpassDescription subpass;
    memset(&subpass, 0, sizeof(subpass));
    subpass.pipelineBindPoint = 0u;
    subpass.colorAttachmentCount = 1u;
    subpass.pColorAttachments = &color_ref;

    VkSubpassDependency dep;
    memset(&dep, 0, sizeof(dep));
    dep.srcSubpass = VK_SUBPASS_EXTERNAL;
    dep.dstSubpass = 0u;
    dep.srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
    dep.dstStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
    dep.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;

    VkRenderPassCreateInfo rpci;
    memset(&rpci, 0, sizeof(rpci));
    rpci.sType = 38u;
    rpci.attachmentCount = 1u;
    rpci.pAttachments = &color_att;
    rpci.subpassCount = 1u;
    rpci.pSubpasses = &subpass;
    rpci.dependencyCount = 1u;
    rpci.pDependencies = &dep;

    VkRenderPass render_pass = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateRenderPass(device, &rpci, NULL, &render_pass));

    VkShaderModuleCreateInfo vsci = { 16u, NULL, 0u, sizeof(VERT_SPV), VERT_SPV };
    VkShaderModuleCreateInfo fsci = { 16u, NULL, 0u, sizeof(FRAG_SPV), FRAG_SPV };
    VkShaderModule vert_mod = VK_NULL_HANDLE, frag_mod = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateShaderModule(device, &vsci, NULL, &vert_mod));
    CHECK_VK(p_vkCreateShaderModule(device, &fsci, NULL, &frag_mod));

    VkPipelineShaderStageCreateInfo stages[2];
    memset(stages, 0, sizeof(stages));
    stages[0].sType = 18u; stages[0].stage = VK_SHADER_STAGE_VERTEX_BIT; stages[0].module = vert_mod; stages[0].pName = "main";
    stages[1].sType = 18u; stages[1].stage = VK_SHADER_STAGE_FRAGMENT_BIT; stages[1].module = frag_mod; stages[1].pName = "main";

    VkPipelineVertexInputStateCreateInfo vi; memset(&vi, 0, sizeof(vi)); vi.sType = 19u;
    VkPipelineInputAssemblyStateCreateInfo ia; memset(&ia, 0, sizeof(ia)); ia.sType = 20u; ia.topology = 3u;
    VkPipelineViewportStateCreateInfo vp; memset(&vp, 0, sizeof(vp)); vp.sType = 22u; vp.viewportCount = 1; vp.scissorCount = 1;

    VkPipelineRasterizationStateCreateInfo rs;
    memset(&rs, 0, sizeof(rs));
    rs.sType = 23u;
    rs.polygonMode = 0u;
    rs.cullMode = 0u;
    rs.frontFace = 0u;
    rs.depthBiasEnable = VK_TRUE;
    rs.lineWidth = 1.0f;

    VkPipelineMultisampleStateCreateInfo ms; memset(&ms, 0, sizeof(ms)); ms.sType = 24u; ms.rasterizationSamples = 1u;
    VkPipelineColorBlendAttachmentState cba; memset(&cba, 0, sizeof(cba)); cba.colorWriteMask = 0xFu;
    VkPipelineColorBlendStateCreateInfo cb; memset(&cb, 0, sizeof(cb)); cb.sType = 26u; cb.attachmentCount = 1; cb.pAttachments = &cba;

    uint32_t dyn_states[4] = {
        VK_DYNAMIC_STATE_VIEWPORT,
        VK_DYNAMIC_STATE_SCISSOR,
        VK_DYNAMIC_STATE_LINE_WIDTH,
        VK_DYNAMIC_STATE_DEPTH_BIAS
    };
    VkPipelineDynamicStateCreateInfo ds = { 27u, NULL, 0u, 4u, dyn_states };

    VkPipelineLayoutCreateInfo plci; memset(&plci, 0, sizeof(plci)); plci.sType = 30u;
    VkPipelineLayout pipe_layout = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreatePipelineLayout(device, &plci, NULL, &pipe_layout));

    VkGraphicsPipelineCreateInfo gpci;
    memset(&gpci, 0, sizeof(gpci));
    gpci.sType = 28u;
    gpci.stageCount = 2;
    gpci.pStages = stages;
    gpci.pVertexInputState = &vi;
    gpci.pInputAssemblyState = &ia;
    gpci.pViewportState = &vp;
    gpci.pRasterizationState = &rs;
    gpci.pMultisampleState = &ms;
    gpci.pColorBlendState = &cb;
    gpci.pDynamicState = &ds;
    gpci.layout = pipe_layout;
    gpci.renderPass = render_pass;
    gpci.basePipelineIndex = -1;

    VkPipeline pipeline = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateGraphicsPipelines(device, VK_NULL_HANDLE, 1u, &gpci, NULL, &pipeline));
    CHECK_COND(pipeline != VK_NULL_HANDLE, "vkCreateGraphicsPipelines");
    printf("PIPELINE_CREATED\n");

    /* 8. Command pool, command buffer, sync objects, staging readback buffer */
    VkCommandPoolCreateInfo cpci = { 39u, NULL, 0x2u, gfx_qf };
    VkCommandPool cmd_pool = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateCommandPool(device, &cpci, NULL, &cmd_pool));

    VkCommandBufferAllocateInfo cbai = { 40u, NULL, cmd_pool, 0u, 1u };
    VkCommandBuffer cmd = NULL;
    CHECK_VK(p_vkAllocateCommandBuffers(device, &cbai, &cmd));

    VkSemaphoreCreateInfo semci = { 9u, NULL, 0u };
    VkSemaphore sem_img_avail = VK_NULL_HANDLE, sem_render_done = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateSemaphore(device, &semci, NULL, &sem_img_avail));
    CHECK_VK(p_vkCreateSemaphore(device, &semci, NULL, &sem_render_done));

    VkFenceCreateInfo fci = { 8u, NULL, 0u };
    VkFence fence = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateFence(device, &fci, NULL, &fence));

    VkBufferCreateInfo bci;
    memset(&bci, 0, sizeof(bci));
    bci.sType = 12u;
    bci.size = 16ull * 1024ull * 1024ull;
    bci.usage = VK_BUFFER_USAGE_TRANSFER_DST_BIT;
    VkBuffer staging_buf = VK_NULL_HANDLE;
    CHECK_VK(p_vkCreateBuffer(device, &bci, NULL, &staging_buf));

    VkMemoryRequirements mreq;
    memset(&mreq, 0, sizeof(mreq));
    p_vkGetBufferMemoryRequirements(device, staging_buf, &mreq);

    VkPhysicalDeviceMemoryProperties mprops;
    memset(&mprops, 0, sizeof(mprops));
    p_vkGetPhysicalDeviceMemoryProperties(phys, &mprops);

    VkMemoryAllocateInfo mai = {
        5u, NULL, mreq.size,
        find_memory_type(&mprops, mreq.memoryTypeBits,
                         VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT)
    };
    VkDeviceMemory staging_mem = VK_NULL_HANDLE;
    CHECK_VK(p_vkAllocateMemory(device, &mai, NULL, &staging_mem));
    CHECK_VK(p_vkBindBufferMemory(device, staging_buf, staging_mem, 0));

    /* 9. Initial swapchain + 3 pre-resize frames + pixel readback on frame 2 */
    SwapBundle sb1;
    VkFlags sc1_usage = 0;
    create_swap_bundle(
        phys, device, surface, render_pass, chosen_fmt, chosen_pm, VK_NULL_HANDLE,
        &sb1, &sc1_usage,
        p_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
        p_vkCreateSwapchainKHR, p_vkGetSwapchainImagesKHR,
        p_vkCreateImageView, p_vkCreateFramebuffer
    );
    CHECK_COND((sc1_usage & VK_IMAGE_USAGE_TRANSFER_SRC_BIT) != 0, "swapchain1 supports TRANSFER_SRC for readback");
    printf("SWAPCHAIN1_CREATED extent=%ux%u images=%u\n", sb1.extent.width, sb1.extent.height, sb1.image_count);
    fflush(stdout);

    const float clear_colors[3][4] = {
        { 0.0f, 0.25f, 0.50f, 1.0f },
        { 0.25f, 0.0f, 0.50f, 1.0f },
        { 0.12549f, 0.25098f, 0.87451f, 1.0f }, /* (32, 64, 223, 255) */
    };

    for (int frame = 0; frame < 3; frame++) {
        pump_messages();
        uint32_t img_idx = 0;
        VkResult acq = p_vkAcquireNextImageKHR(device, sb1.swapchain, ~0ull, sem_img_avail, VK_NULL_HANDLE, &img_idx);
        CHECK_COND(acq == VK_SUCCESS || acq == VK_SUBOPTIMAL_KHR, "pre-resize vkAcquireNextImageKHR");

        CHECK_VK(p_vkResetCommandBuffer(cmd, 0));
        VkCommandBufferBeginInfo cbbi = { 42u, NULL, 1u, NULL };
        CHECK_VK(p_vkBeginCommandBuffer(cmd, &cbbi));

        VkClearValue cv;
        cv.color.float32[0] = clear_colors[frame][0];
        cv.color.float32[1] = clear_colors[frame][1];
        cv.color.float32[2] = clear_colors[frame][2];
        cv.color.float32[3] = clear_colors[frame][3];

        VkRenderPassBeginInfo rpbi;
        memset(&rpbi, 0, sizeof(rpbi));
        rpbi.sType = 43u;
        rpbi.renderPass = render_pass;
        rpbi.framebuffer = sb1.fbs[img_idx];
        rpbi.renderArea.extent = sb1.extent;
        rpbi.clearValueCount = 1u;
        rpbi.pClearValues = &cv;

        p_vkCmdBeginRenderPass(cmd, &rpbi, 0u);
        p_vkCmdBindPipeline(cmd, 0u, pipeline);
        VkViewport vport = { 0.0f, 0.0f, (float)sb1.extent.width, (float)sb1.extent.height, 0.0f, 1.0f };
        VkRect2D sciss = { {0, 0}, sb1.extent };
        p_vkCmdSetViewport(cmd, 0, 1, &vport);
        p_vkCmdSetScissor(cmd, 0, 1, &sciss);
        /* Exercise BOTH static import AND dynamic vkGetDeviceProcAddr float-arg ABI bridges */
        if (frame == 0) {
            vkCmdSetLineWidth(cmd, 1.0f);
            vkCmdSetDepthBias(cmd, 0.0f, 0.0f, 0.0f);
        } else {
            p_vkCmdSetLineWidth(cmd, 1.0f);
            p_vkCmdSetDepthBias(cmd, 0.0f, 0.0f, 0.0f);
        }
        p_vkCmdDraw(cmd, 3u, 1u, 0u, 0u);
        p_vkCmdEndRenderPass(cmd);

        if (frame == 2) {
            VkImageMemoryBarrier b1;
            memset(&b1, 0, sizeof(b1));
            b1.sType = 45u;
            b1.srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
            b1.dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
            b1.oldLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
            b1.newLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
            b1.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b1.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b1.image = sb1.images[img_idx];
            b1.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            b1.subresourceRange.levelCount = 1;
            b1.subresourceRange.layerCount = 1;
            p_vkCmdPipelineBarrier(cmd, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                                   VK_PIPELINE_STAGE_TRANSFER_BIT, 0, 0, NULL, 0, NULL, 1, &b1);

            VkBufferImageCopy bic;
            memset(&bic, 0, sizeof(bic));
            bic.imageSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            bic.imageSubresource.layerCount = 1;
            bic.imageExtent.width = sb1.extent.width;
            bic.imageExtent.height = sb1.extent.height;
            bic.imageExtent.depth = 1;
            p_vkCmdCopyImageToBuffer(cmd, sb1.images[img_idx], VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                                     staging_buf, 1, &bic);

            VkImageMemoryBarrier b2 = b1;
            b2.srcAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
            b2.dstAccessMask = 0;
            b2.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
            b2.newLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
            p_vkCmdPipelineBarrier(cmd, VK_PIPELINE_STAGE_TRANSFER_BIT,
                                   VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, 0, 0, NULL, 0, NULL, 1, &b2);
        } else {
            VkImageMemoryBarrier b;
            memset(&b, 0, sizeof(b));
            b.sType = 45u;
            b.srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
            b.dstAccessMask = 0;
            b.oldLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
            b.newLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
            b.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b.image = sb1.images[img_idx];
            b.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            b.subresourceRange.levelCount = 1;
            b.subresourceRange.layerCount = 1;
            p_vkCmdPipelineBarrier(cmd, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                                   VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, 0, 0, NULL, 0, NULL, 1, &b);
        }

        CHECK_VK(p_vkEndCommandBuffer(cmd));

        VkFlags wait_stage = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        VkSubmitInfo si;
        memset(&si, 0, sizeof(si));
        si.sType = 4u;
        si.waitSemaphoreCount = 1;
        si.pWaitSemaphores = &sem_img_avail;
        si.pWaitDstStageMask = &wait_stage;
        si.commandBufferCount = 1;
        si.pCommandBuffers = &cmd;
        si.signalSemaphoreCount = 1;
        si.pSignalSemaphores = &sem_render_done;

        CHECK_VK(p_vkQueueSubmit(queue, 1, &si, fence));
        CHECK_VK(p_vkWaitForFences(device, 1, &fence, VK_TRUE, ~0ull));
        CHECK_VK(p_vkResetFences(device, 1, &fence));

        if (frame == 2) {
            void* mapped = NULL;
            size_t total_bytes = (size_t)sb1.extent.width * sb1.extent.height * 4;
            CHECK_VK(p_vkMapMemory(device, staging_mem, 0, total_bytes, 0, &mapped));
            const uint8_t* px = (const uint8_t*)mapped;
            size_t corner_off = (2u * sb1.extent.width + 2u) * 4u;
            size_t center_off = ((sb1.extent.height / 2u) * sb1.extent.width + (sb1.extent.width / 2u)) * 4u;
            check_rgba("pre_resize_corner", px + corner_off, is_bgra, 32, 64, 223, 255);
            check_rgba("pre_resize_center", px + center_off, is_bgra, 255, 128, 0, 255);
            p_vkUnmapMemory(device, staging_mem);
            printf("PRE_RESIZE_READBACK_OK\n");
        }

        VkPresentInfoKHR pi;
        memset(&pi, 0, sizeof(pi));
        pi.sType = 1000001001u;
        pi.waitSemaphoreCount = 1;
        pi.pWaitSemaphores = &sem_render_done;
        pi.swapchainCount = 1;
        pi.pSwapchains = &sb1.swapchain;
        pi.pImageIndices = &img_idx;
        VkResult pres = p_vkQueuePresentKHR(queue, &pi);
        CHECK_COND(pres == VK_SUCCESS || pres == VK_SUBOPTIMAL_KHR, "pre-resize vkQueuePresentKHR");
        printf("PRE_RESIZE_FRAME_%d_PRESENTED\n", frame);
        fflush(stdout);
    }

    /* 10. Window resize (320x200 client) + swapchain recreation with oldSwapchain */
    RECT wr2 = { 0, 0, 320, 200 };
    AdjustWindowRectEx(&wr2, WS_OVERLAPPEDWINDOW, FALSE, 0);
    BOOL ok_pos = SetWindowPos(hwnd, NULL, 0, 0, wr2.right - wr2.left, wr2.bottom - wr2.top,
                               SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE);
    CHECK_COND(ok_pos != 0, "SetWindowPos resize");
    pump_messages();

    RECT cr2;
    GetClientRect(hwnd, &cr2);
    printf("WINDOW_RESIZED client=%ldx%ld\n",
           (long)(cr2.right - cr2.left), (long)(cr2.bottom - cr2.top));
    fflush(stdout);

    CHECK_VK(p_vkDeviceWaitIdle(device));

    if (p_vkGetPhysicalDeviceSurfaceCapabilities2KHR != NULL) {
        VkPhysicalDeviceSurfaceInfo2KHR si2 = { 1000119000u, NULL, surface };
        VkSurfaceCapabilities2KHR sc2;
        memset(&sc2, 0, sizeof(sc2));
        sc2.sType = 1000119001u;
        CHECK_VK(p_vkGetPhysicalDeviceSurfaceCapabilities2KHR(phys, &si2, &sc2));
        CHECK_COND(sc2.surfaceCapabilities.currentExtent.width > sb1.extent.width &&
                   sc2.surfaceCapabilities.currentExtent.height > sb1.extent.height,
                   "vkGetPhysicalDeviceSurfaceCapabilities2KHR updated after resize");
        printf("SURFACE_CAPS2_RESIZED extent=%ux%u\n",
               sc2.surfaceCapabilities.currentExtent.width,
               sc2.surfaceCapabilities.currentExtent.height);
    }

    SwapBundle sb2;
    VkFlags sc2_usage = 0;
    create_swap_bundle(
        phys, device, surface, render_pass, chosen_fmt, chosen_pm, sb1.swapchain,
        &sb2, &sc2_usage,
        p_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
        p_vkCreateSwapchainKHR, p_vkGetSwapchainImagesKHR,
        p_vkCreateImageView, p_vkCreateFramebuffer
    );
    destroy_swap_views_fbs(device, &sb1, p_vkDestroyFramebuffer, p_vkDestroyImageView);
    p_vkDestroySwapchainKHR(device, sb1.swapchain, NULL);
    sb1.swapchain = VK_NULL_HANDLE;

    printf("SWAPCHAIN2_CREATED extent=%ux%u images=%u (was %ux%u)\n",
           sb2.extent.width, sb2.extent.height, sb2.image_count,
           sb1.extent.width, sb1.extent.height);
    fflush(stdout);
    CHECK_COND(sb2.extent.width > sb1.extent.width && sb2.extent.height > sb1.extent.height,
               "swapchain extent updated after window resize");

    /* 11. Render 3 post-resize frames + pixel readback on final frame */
    const float post_clear[3][4] = {
        { 0.0f, 0.50f, 0.25f, 1.0f },
        { 0.50f, 0.125f, 0.50f, 1.0f },
        { 0.0f, 0.37647f, 0.75294f, 1.0f }, /* (0, 96, 192, 255) */
    };

    for (int frame = 0; frame < 3; frame++) {
        pump_messages();
        uint32_t img_idx = 0;
        VkResult acq = p_vkAcquireNextImageKHR(device, sb2.swapchain, ~0ull, sem_img_avail, VK_NULL_HANDLE, &img_idx);
        CHECK_COND(acq == VK_SUCCESS || acq == VK_SUBOPTIMAL_KHR, "post-resize vkAcquireNextImageKHR");

        CHECK_VK(p_vkResetCommandBuffer(cmd, 0));
        VkCommandBufferBeginInfo cbbi = { 42u, NULL, 1u, NULL };
        CHECK_VK(p_vkBeginCommandBuffer(cmd, &cbbi));

        VkClearValue cv;
        cv.color.float32[0] = post_clear[frame][0];
        cv.color.float32[1] = post_clear[frame][1];
        cv.color.float32[2] = post_clear[frame][2];
        cv.color.float32[3] = post_clear[frame][3];

        VkRenderPassBeginInfo rpbi;
        memset(&rpbi, 0, sizeof(rpbi));
        rpbi.sType = 43u;
        rpbi.renderPass = render_pass;
        rpbi.framebuffer = sb2.fbs[img_idx];
        rpbi.renderArea.extent = sb2.extent;
        rpbi.clearValueCount = 1u;
        rpbi.pClearValues = &cv;

        p_vkCmdBeginRenderPass(cmd, &rpbi, 0u);
        p_vkCmdBindPipeline(cmd, 0u, pipeline);
        VkViewport vport = { 0.0f, 0.0f, (float)sb2.extent.width, (float)sb2.extent.height, 0.0f, 1.0f };
        VkRect2D sciss = { {0, 0}, sb2.extent };
        p_vkCmdSetViewport(cmd, 0, 1, &vport);
        p_vkCmdSetScissor(cmd, 0, 1, &sciss);
        p_vkCmdSetLineWidth(cmd, 1.0f);
        p_vkCmdSetDepthBias(cmd, 0.0f, 0.0f, 0.0f);
        p_vkCmdDraw(cmd, 3u, 1u, 0u, 0u);
        p_vkCmdEndRenderPass(cmd);

        if (frame == 2) {
            VkImageMemoryBarrier b1;
            memset(&b1, 0, sizeof(b1));
            b1.sType = 45u;
            b1.srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
            b1.dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
            b1.oldLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
            b1.newLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
            b1.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b1.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b1.image = sb2.images[img_idx];
            b1.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            b1.subresourceRange.levelCount = 1;
            b1.subresourceRange.layerCount = 1;
            p_vkCmdPipelineBarrier(cmd, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                                   VK_PIPELINE_STAGE_TRANSFER_BIT, 0, 0, NULL, 0, NULL, 1, &b1);

            VkBufferImageCopy bic;
            memset(&bic, 0, sizeof(bic));
            bic.imageSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            bic.imageSubresource.layerCount = 1;
            bic.imageExtent.width = sb2.extent.width;
            bic.imageExtent.height = sb2.extent.height;
            bic.imageExtent.depth = 1;
            p_vkCmdCopyImageToBuffer(cmd, sb2.images[img_idx], VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                                     staging_buf, 1, &bic);

            VkImageMemoryBarrier b2 = b1;
            b2.srcAccessMask = VK_ACCESS_TRANSFER_READ_BIT;
            b2.dstAccessMask = 0;
            b2.oldLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL;
            b2.newLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
            p_vkCmdPipelineBarrier(cmd, VK_PIPELINE_STAGE_TRANSFER_BIT,
                                   VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, 0, 0, NULL, 0, NULL, 1, &b2);
        } else {
            VkImageMemoryBarrier b;
            memset(&b, 0, sizeof(b));
            b.sType = 45u;
            b.srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
            b.dstAccessMask = 0;
            b.oldLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
            b.newLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
            b.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
            b.image = sb2.images[img_idx];
            b.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
            b.subresourceRange.levelCount = 1;
            b.subresourceRange.layerCount = 1;
            p_vkCmdPipelineBarrier(cmd, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                                   VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, 0, 0, NULL, 0, NULL, 1, &b);
        }

        CHECK_VK(p_vkEndCommandBuffer(cmd));

        VkFlags wait_stage = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        VkSubmitInfo si;
        memset(&si, 0, sizeof(si));
        si.sType = 4u;
        si.waitSemaphoreCount = 1;
        si.pWaitSemaphores = &sem_img_avail;
        si.pWaitDstStageMask = &wait_stage;
        si.commandBufferCount = 1;
        si.pCommandBuffers = &cmd;
        si.signalSemaphoreCount = 1;
        si.pSignalSemaphores = &sem_render_done;

        CHECK_VK(p_vkQueueSubmit(queue, 1, &si, fence));
        CHECK_VK(p_vkWaitForFences(device, 1, &fence, VK_TRUE, ~0ull));
        CHECK_VK(p_vkResetFences(device, 1, &fence));

        if (frame == 2) {
            void* mapped = NULL;
            size_t total_bytes = (size_t)sb2.extent.width * sb2.extent.height * 4;
            CHECK_VK(p_vkMapMemory(device, staging_mem, 0, total_bytes, 0, &mapped));
            const uint8_t* px = (const uint8_t*)mapped;
            size_t corner_off = (2u * sb2.extent.width + 2u) * 4u;
            size_t center_off = ((sb2.extent.height / 2u) * sb2.extent.width + (sb2.extent.width / 2u)) * 4u;
            check_rgba("post_resize_corner", px + corner_off, is_bgra, 0, 96, 192, 255);
            check_rgba("post_resize_center", px + center_off, is_bgra, 255, 128, 0, 255);
            p_vkUnmapMemory(device, staging_mem);
            printf("POST_RESIZE_READBACK_OK\n");
        }

        VkPresentInfoKHR pi;
        memset(&pi, 0, sizeof(pi));
        pi.sType = 1000001001u;
        pi.waitSemaphoreCount = 1;
        pi.pWaitSemaphores = &sem_render_done;
        pi.swapchainCount = 1;
        pi.pSwapchains = &sb2.swapchain;
        pi.pImageIndices = &img_idx;
        VkResult pres = p_vkQueuePresentKHR(queue, &pi);
        CHECK_COND(pres == VK_SUCCESS || pres == VK_SUBOPTIMAL_KHR, "post-resize vkQueuePresentKHR");
        printf("POST_RESIZE_FRAME_%d_PRESENTED\n", frame);
        fflush(stdout);
    }

    /* 12. Clean teardown */
    CHECK_VK(p_vkDeviceWaitIdle(device));
    p_vkDestroyBuffer(device, staging_buf, NULL);
    p_vkFreeMemory(device, staging_mem, NULL);
    p_vkDestroyFence(device, fence, NULL);
    p_vkDestroySemaphore(device, sem_render_done, NULL);
    p_vkDestroySemaphore(device, sem_img_avail, NULL);
    p_vkFreeCommandBuffers(device, cmd_pool, 1, &cmd);
    p_vkDestroyCommandPool(device, cmd_pool, NULL);
    p_vkDestroyPipeline(device, pipeline, NULL);
    p_vkDestroyPipelineLayout(device, pipe_layout, NULL);
    p_vkDestroyShaderModule(device, frag_mod, NULL);
    p_vkDestroyShaderModule(device, vert_mod, NULL);
    destroy_swap_views_fbs(device, &sb2, p_vkDestroyFramebuffer, p_vkDestroyImageView);
    p_vkDestroySwapchainKHR(device, sb2.swapchain, NULL);
    p_vkDestroyRenderPass(device, render_pass, NULL);
    p_vkDestroyDevice(device, NULL);
    p_vkDestroySurfaceKHR(instance, surface, NULL);
    vkDestroyInstance(instance, NULL);
    DestroyWindow(hwnd);
    printf("VK_E2E_PASS\n");
    fflush(stdout);
    ExitProcess(0);
    return 0;
}
"##;

fn compile_windows_vulkan_exe(zig: &Path, work_dir: &Path) -> PathBuf {
    let def_file = work_dir.join("vulkan-1.def");
    let lib_file = work_dir.join("libvulkan-1.a");
    let c_file = work_dir.join("vulkan_e2e.c");
    let exe_file = work_dir.join("vulkan_e2e.exe");

    std::fs::write(&def_file, VULKAN_DEF).expect("write vulkan-1.def");
    std::fs::write(&c_file, VULKAN_E2E_C).expect("write vulkan_e2e.c");

    let dlltool = Command::new(zig)
        .args(["dlltool", "-d"])
        .arg(&def_file)
        .arg("-l")
        .arg(&lib_file)
        .args(["-m", "i386:x86-64"])
        .output()
        .expect("run zig dlltool");
    assert!(
        dlltool.status.success(),
        "zig dlltool failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&dlltool.stdout),
        String::from_utf8_lossy(&dlltool.stderr)
    );

    let cc = Command::new(zig)
        .args(["cc", "-target", "x86_64-windows-gnu", "-O2"])
        .arg(&c_file)
        .arg(&lib_file)
        .args(["-luser32", "-lkernel32", "-o"])
        .arg(&exe_file)
        .output()
        .expect("run zig cc");
    assert!(
        cc.status.success(),
        "zig cc failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&cc.stdout),
        String::from_utf8_lossy(&cc.stderr)
    );
    assert!(exe_file.is_file(), "expected compiled PE at {}", exe_file.display());
    exe_file
}

fn run_port_runner(
    exe: &Path,
    moltenvk: &Path,
    work_dir: &Path,
    headless: bool,
    limit: Duration,
) -> (Option<i32>, String) {
    let drive_c = work_dir.join("drive_c");
    let log_dir = work_dir.join("logs");
    let aot_dir = work_dir.join("aot");
    std::fs::create_dir_all(&drive_c).expect("create drive_c");
    std::fs::create_dir_all(&log_dir).expect("create log_dir");
    std::fs::create_dir_all(&aot_dir).expect("create aot_dir");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_port-runner"));
    cmd.arg("--pe")
        .arg(exe)
        .arg("--drive-c")
        .arg(&drive_c)
        .env("PORT_VULKAN_DYLIB", moltenvk)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", &aot_dir)
        .env("AW2_NO_ACTIVATE", "1")
        .env("RUST_LOG", "info")
        .env_remove("PORT_LOG_APPEND")
        .env_remove("AW2_LOG_APPEND")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if headless {
        cmd.env("PORT_HEADLESS", "1");
    } else {
        cmd.env_remove("PORT_HEADLESS").env_remove("AW2_HEADLESS");
    }

    let mut child = cmd.spawn().expect("spawn port-runner");
    let mut out_pipe = child.stdout.take().unwrap();
    let mut err_pipe = child.stderr.take().unwrap();
    let out_reader = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = out_pipe.read_to_string(&mut s);
        s
    });
    let err_reader = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = err_pipe.read_to_string(&mut s);
        s
    });

    let t0 = Instant::now();
    let code = loop {
        if let Some(st) = child.try_wait().expect("try_wait port-runner") {
            break st.code();
        }
        if t0.elapsed() > limit {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(25));
    };

    let mut combined = String::new();
    combined.push_str(&out_reader.join().unwrap_or_default());
    combined.push_str(&err_reader.join().unwrap_or_default());
    if let Ok(log_file) = std::fs::read_to_string(log_dir.join("port-runner.log")) {
        if !combined.is_empty() {
            combined.push('\n');
        }
        combined.push_str(&log_file);
    }
    (code, combined)
}

fn require_e2e_prerequisites(test_name: &str) -> Option<(PathBuf, PathBuf)> {
    if std::env::var("PORT_TEST_MOLTENVK").ok().as_deref() != Some("1") {
        eprintln!(
            "skipping {test_name}: set PORT_TEST_MOLTENVK=1 (and optionally PORT_VULKAN_DYLIB=/path/to/libMoltenVK.dylib) to run"
        );
        return None;
    }
    let Some(zig) = find_zig() else {
        eprintln!("skipping {test_name}: `zig` compiler not found on host");
        return None;
    };
    let Some(moltenvk) = find_moltenvk_dylib() else {
        eprintln!(
            "skipping {test_name}: libMoltenVK.dylib not found on host (set PORT_VULKAN_DYLIB=/path/to/libMoltenVK.dylib)"
        );
        return None;
    };
    Some((zig, moltenvk))
}

fn assert_vulkan_e2e_markers(mode: &str, code: Option<i32>, log: &str) {
    assert_eq!(code, Some(0), "{mode} port-runner exited with {code:?}:\n{log}");
    for marker in [
        "VK_E2E_START",
        "WINDOW_CREATED",
        "STATIC_IMPORT_OK",
        "DYNAMIC_LOAD_OK",
        "INSTANCE_CREATED",
        "WIN32_SURFACE_CREATED",
        "PHYSICAL_DEVICE",
        "DEVICE_CREATED",
        "DEVICE_PROCS_RESOLVED",
        "ALLOCATOR_SANITIZE_OK",
        "PIPELINE_CREATED",
        "SWAPCHAIN1_CREATED",
        "PRE_RESIZE_FRAME_0_PRESENTED",
        "PRE_RESIZE_FRAME_1_PRESENTED",
        "PIXEL_READBACK pre_resize_corner",
        "PIXEL_READBACK pre_resize_center",
        "PRE_RESIZE_READBACK_OK",
        "PRE_RESIZE_FRAME_2_PRESENTED",
        "WINDOW_RESIZED client=320x200",
        "SWAPCHAIN2_CREATED",
        "POST_RESIZE_FRAME_0_PRESENTED",
        "POST_RESIZE_FRAME_1_PRESENTED",
        "PIXEL_READBACK post_resize_corner",
        "PIXEL_READBACK post_resize_center",
        "POST_RESIZE_READBACK_OK",
        "POST_RESIZE_FRAME_2_PRESENTED",
        "VK_E2E_PASS",
    ] {
        assert!(
            log.contains(marker),
            "{mode} missing expected marker {marker:?} in output:\n{log}"
        );
    }
    assert!(
        log.contains("vkCreateMetalSurfaceEXT=0") || log.contains("vkCreateMacOSSurfaceMVK=0"),
        "{mode} expected successful Metal/macOS surface creation:\n{log}"
    );
    assert!(!log.contains("FAIL:"), "{mode} reported failure:\n{log}");
    assert!(!log.contains("UNIMPLEMENTED"), "{mode} hit unimplemented import:\n{log}");
}

#[test]
fn windows_x86_64_vulkan_windowed_present_resize_and_readback() {
    let Some((zig, moltenvk)) =
        require_e2e_prerequisites("windows_x86_64_vulkan_windowed_present_resize_and_readback")
    else {
        return;
    };

    let tmp = TempDirGuard::new("port-vulkan-e2e-win");
    let exe = compile_windows_vulkan_exe(&zig, tmp.path());
    let (code, log) = run_port_runner(&exe, &moltenvk, tmp.path(), false, Duration::from_secs(60));

    assert!(
        log.contains("NSWindow for hwnd"),
        "windowed E2E must create a real AppKit NSWindow + CAMetalLayer:\n{log}"
    );
    assert_vulkan_e2e_markers("windowed", code, &log);
    for line in log.lines() {
        if line.starts_with("VK_E2E")
            || line.starts_with("WINDOW_")
            || line.starts_with("STATIC_")
            || line.starts_with("DYNAMIC_")
            || line.starts_with("INSTANCE_")
            || line.starts_with("WIN32_SURFACE_")
            || line.starts_with("PHYSICAL_DEVICE")
            || line.starts_with("DEVICE_")
            || line.starts_with("ALLOCATOR_")
            || line.starts_with("PIPELINE_")
            || line.starts_with("SURFACE_")
            || line.starts_with("SWAPCHAIN")
            || line.starts_with("PRE_RESIZE_")
            || line.starts_with("POST_RESIZE_")
            || line.starts_with("PIXEL_READBACK")
        {
            eprintln!("[vulkan-e2e/windowed] {line}");
        }
    }
}

#[test]
fn windows_x86_64_vulkan_headless_present_resize_and_readback() {
    let Some((zig, moltenvk)) =
        require_e2e_prerequisites("windows_x86_64_vulkan_headless_present_resize_and_readback")
    else {
        return;
    };

    let tmp = TempDirGuard::new("port-vulkan-e2e-headless");
    let exe = compile_windows_vulkan_exe(&zig, tmp.path());
    let (code, log) = run_port_runner(&exe, &moltenvk, tmp.path(), true, Duration::from_secs(60));

    assert!(
        log.contains("created headless offscreen CAMetalLayer"),
        "headless E2E must exercise headless offscreen CAMetalLayer creation:\n{log}"
    );
    assert_vulkan_e2e_markers("headless", code, &log);
    for line in log.lines() {
        if line.starts_with("VK_E2E")
            || line.starts_with("SWAPCHAIN")
            || line.starts_with("PIXEL_READBACK")
            || line.starts_with("SURFACE_CAPS")
        {
            eprintln!("[vulkan-e2e/headless] {line}");
        }
    }
}
