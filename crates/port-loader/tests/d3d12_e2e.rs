//! Headless end-to-end Direct3D 12 Windows x86-64 guest test compiled with
//! `zig cc -target x86_64-windows-gnu` and executed through `port-runner` on
//! Apple Silicon (D3D12 COM -> `d3d12-metal` -> Metal 4).
//!
//! Runs the guest twice: once on the live ARM64 JIT (`PORT_AOT=0`) and once
//! from a prewarmed `.aot` block cache (`--prewarm-aot` + `PORT_AOT=1`).
//!
//! Verifies:
//! - `D3D12CreateDevice`, `CreateCommandQueue`, `CreateCommandAllocator`,
//!   `CreateCommandList`, `CreateFence` + event completion wait
//! - `D3D12SerializeRootSignature` + `CreateRootSignature` (graphics + compute)
//! - `CreateGraphicsPipelineState` (SM 5.0 DXBC VS+PS) + triangle draw over
//!   clear color + `GetCopyableFootprints` + `CopyTextureRegion` readback +
//!   exact RGBA8 pixel verification
//! - `CreateComputePipelineState` (SM 5.0 DXBC CS) + shader-visible UAV
//!   descriptor table + `Dispatch(1, 1, 1)` + UAV barrier + `CopyResource`
//!   readback + exact 64x `float4` verification
#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

mod guest_corpus_common;

use guest_corpus_common::{
    compile_c_guest, find_zig, has_dxilconv, has_metal_irconverter, run_guest_both_modes, TempDir,
};
use std::path::PathBuf;
use std::time::Duration;

fn check_prerequisites() -> Option<PathBuf> {
    let Some(zig) = find_zig() else {
        eprintln!("skipping d3d12_e2e: zig not found");
        return None;
    };
    if !has_dxilconv() {
        eprintln!("skipping d3d12_e2e: libdxilconv.dylib not found");
        return None;
    }
    if !has_metal_irconverter() {
        eprintln!("skipping d3d12_e2e: libmetalirconverter.dylib not found");
        return None;
    }
    Some(zig)
}

#[test]
fn d3d12_e2e_triangle_and_compute_readback() {
    let Some(zig) = check_prerequisites() else {
        return;
    };
    let tmp = TempDir::new("d3d12-e2e");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "d3d12_e2e_guest",
        D3D12_GUEST_C,
        &["d3d12", "kernel32"],
    );
    run_guest_both_modes(
        "d3d12_e2e",
        &exe,
        tmp.path(),
        &[
            "D3D12_E2E: device+queue+fence created",
            "D3D12_E2E: graphics+compute PSOs created",
            "D3D12_E2E: fence signaled value=1",
            "D3D12_E2E: pixels corner=(0,64,255,255) center=(0,255,0,255) far=(0,64,255,255)",
            "D3D12_E2E: compute dispatch verified (64/64 float4 exact match)",
            "D3D12_E2E: PASS ALL",
        ],
        Duration::from_secs(60),
    );
}

const D3D12_GUEST_C: &str = r##"
#define WIDL_C_INLINE_WRAPPERS
#define COBJMACROS
#define CINTERFACE
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <d3d12.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

/* Explicit IIDs so linking never depends on missing mingw GUID symbols. */
static const GUID MY_IID_ID3D12Device =
    {0x189819f1, 0x1db6, 0x4b57, {0xbe, 0x54, 0x18, 0x21, 0x33, 0x9b, 0x85, 0xf7}};
static const GUID MY_IID_ID3D12CommandQueue =
    {0x0ec870a6, 0x5d7e, 0x4c22, {0x8c, 0xfc, 0x5b, 0xaa, 0xe0, 0x76, 0x16, 0xed}};
static const GUID MY_IID_ID3D12CommandAllocator =
    {0x6102dee4, 0xaf59, 0x4b09, {0xb9, 0x99, 0xb4, 0x4d, 0x73, 0xf0, 0x9b, 0x24}};
static const GUID MY_IID_ID3D12GraphicsCommandList =
    {0x5b160d0f, 0xac1b, 0x4185, {0x8b, 0xa8, 0xb3, 0xae, 0x42, 0xa5, 0xa4, 0x55}};
static const GUID MY_IID_ID3D12Fence =
    {0x0a753dcf, 0xc4d8, 0x4b91, {0xad, 0xf6, 0xbe, 0x5a, 0x60, 0xd9, 0x5a, 0x76}};
static const GUID MY_IID_ID3D12DescriptorHeap =
    {0x8efb471d, 0x616c, 0x4f49, {0x90, 0xf7, 0x12, 0x7b, 0xb7, 0x63, 0xfa, 0x51}};
static const GUID MY_IID_ID3D12Resource =
    {0x696442be, 0xa72e, 0x4059, {0xbc, 0x79, 0x5b, 0x5c, 0x98, 0x04, 0x0f, 0xad}};
static const GUID MY_IID_ID3D12RootSignature =
    {0xc54a6b66, 0x72df, 0x4ee8, {0x8b, 0xe5, 0xa9, 0x46, 0xa1, 0x42, 0x92, 0x14}};
static const GUID MY_IID_ID3D12PipelineState =
    {0x765a30f3, 0xf624, 0x4c6f, {0xa8, 0x28, 0xac, 0xe9, 0x48, 0x62, 0x24, 0x45}};

/* Signed SM 5.0 DXBC shaders (converted through libdxilconv -> libmetalirconverter):
 * 1) vs_5_0: passthrough POSITION (v0.xyzw) -> SV_Position (o0.xyzw)
 * 2) ps_5_0: outputs cb0[0].xyzw -> SV_Target0
 * 3) cs_5_0: [numthreads(64,1,1)] writes u0[vThreadID.x] = cb0[0].xyzw
 */
static const uint8_t kVsPassthroughDxbc[216] = {
    0x44, 0x58, 0x42, 0x43, 0x9d, 0x3b, 0x28, 0xb4, 0xb7, 0xe9, 0x69, 0xf4,
    0x0e, 0x49, 0x0a, 0xe5, 0xae, 0x5e, 0xa2, 0x25, 0x01, 0x00, 0x00, 0x00,
    0xd8, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x60, 0x00, 0x00, 0x00, 0x94, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x0f, 0x00, 0x00,
    0x50, 0x4f, 0x53, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x00, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x00, 0x53, 0x48, 0x45, 0x58, 0x3c, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x01, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x5f, 0x00, 0x00, 0x03,
    0xf2, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x00, 0x04,
    0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x36, 0x00, 0x00, 0x05, 0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x46, 0x1e, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x01,
};

static const uint8_t kPsCb0ColorDxbc[188] = {
    0x44, 0x58, 0x42, 0x43, 0xf4, 0x68, 0x60, 0xac, 0x53, 0x58, 0x0a, 0x02,
    0xf3, 0x2f, 0xb6, 0x13, 0xd5, 0x77, 0x32, 0x24, 0x01, 0x00, 0x00, 0x00,
    0xbc, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x3c, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x54, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x00, 0x00, 0x00, 0x53, 0x48, 0x45, 0x58, 0x44, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x6a, 0x08, 0x00, 0x01,
    0x59, 0x00, 0x00, 0x04, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x65, 0x00, 0x00, 0x03, 0xf2, 0x20, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x06, 0xf2, 0x20, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x01,
};

static const uint8_t kCsWriteCb0Dxbc[184] = {
    0x44, 0x58, 0x42, 0x43, 0x3d, 0xf7, 0x0a, 0x03, 0xa5, 0xda, 0x5c, 0xa0,
    0x07, 0x84, 0xc9, 0x7e, 0x27, 0x46, 0xe9, 0x80, 0x01, 0x00, 0x00, 0x00,
    0xb8, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x3c, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x53, 0x48, 0x45, 0x58, 0x64, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x05, 0x00, 0x19, 0x00, 0x00, 0x00, 0x6a, 0x08, 0x00, 0x01,
    0x59, 0x00, 0x00, 0x04, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x9c, 0x08, 0x00, 0x04, 0x00, 0xe0, 0x11, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x55, 0x55, 0x00, 0x00, 0x5f, 0x00, 0x00, 0x02,
    0x12, 0x00, 0x02, 0x00, 0x9b, 0x00, 0x00, 0x04, 0x40, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xa4, 0x00, 0x00, 0x07,
    0xf2, 0xe0, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x02, 0x00,
    0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x3e, 0x00, 0x00, 0x01,
};

#define CHECK_HR(hr, msg) do { \
    HRESULT _h = (hr); \
    if (FAILED(_h)) { \
        fprintf(stderr, "FAIL: %s (hr=0x%08lx, line %d)\n", (msg), (unsigned long)_h, __LINE__); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

#define CHECK_COND(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d)\n", (msg), __LINE__); \
        fflush(stderr); \
        ExitProcess(2); \
    } \
} while (0)

static int near_u8(uint8_t got, uint8_t expect, int tol) {
    int d = (int)got - (int)expect;
    if (d < 0) d = -d;
    return d <= tol;
}

static ID3D12Resource *create_buffer(
    ID3D12Device *dev,
    D3D12_HEAP_TYPE heap_type,
    UINT64 size,
    D3D12_RESOURCE_FLAGS flags,
    D3D12_RESOURCE_STATES init_state
) {
    D3D12_HEAP_PROPERTIES hp;
    memset(&hp, 0, sizeof(hp));
    hp.Type = heap_type;

    D3D12_RESOURCE_DESC rd;
    memset(&rd, 0, sizeof(rd));
    rd.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    rd.Width = size;
    rd.Height = 1;
    rd.DepthOrArraySize = 1;
    rd.MipLevels = 1;
    rd.Format = DXGI_FORMAT_UNKNOWN;
    rd.SampleDesc.Count = 1;
    rd.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
    rd.Flags = flags;

    ID3D12Resource *res = NULL;
    CHECK_HR(
        ID3D12Device_CreateCommittedResource(
            dev, &hp, D3D12_HEAP_FLAG_NONE, &rd, init_state, NULL, &MY_IID_ID3D12Resource, (void **)&res
        ),
        "CreateCommittedResource(buffer)"
    );
    return res;
}

static void add_barrier(
    ID3D12GraphicsCommandList *cl,
    ID3D12Resource *res,
    D3D12_RESOURCE_STATES before,
    D3D12_RESOURCE_STATES after
) {
    D3D12_RESOURCE_BARRIER b;
    memset(&b, 0, sizeof(b));
    b.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    b.Transition.pResource = res;
    b.Transition.StateBefore = before;
    b.Transition.StateAfter = after;
    b.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    ID3D12GraphicsCommandList_ResourceBarrier(cl, 1, &b);
}

int main(void) {
    /* 1. Device, queue, allocator, command list, fence */
    ID3D12Device *dev = NULL;
    CHECK_HR(D3D12CreateDevice(NULL, D3D_FEATURE_LEVEL_11_0, &MY_IID_ID3D12Device, (void **)&dev),
             "D3D12CreateDevice");

    D3D12_COMMAND_QUEUE_DESC qd;
    memset(&qd, 0, sizeof(qd));
    qd.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
    ID3D12CommandQueue *queue = NULL;
    CHECK_HR(ID3D12Device_CreateCommandQueue(dev, &qd, &MY_IID_ID3D12CommandQueue, (void **)&queue),
             "CreateCommandQueue");

    ID3D12CommandAllocator *alloc = NULL;
    CHECK_HR(ID3D12Device_CreateCommandAllocator(dev, D3D12_COMMAND_LIST_TYPE_DIRECT,
             &MY_IID_ID3D12CommandAllocator, (void **)&alloc), "CreateCommandAllocator");

    ID3D12GraphicsCommandList *cl = NULL;
    CHECK_HR(ID3D12Device_CreateCommandList(dev, 0, D3D12_COMMAND_LIST_TYPE_DIRECT, alloc, NULL,
             &MY_IID_ID3D12GraphicsCommandList, (void **)&cl), "CreateCommandList");

    ID3D12Fence *fence = NULL;
    CHECK_HR(ID3D12Device_CreateFence(dev, 0, D3D12_FENCE_FLAG_NONE, &MY_IID_ID3D12Fence, (void **)&fence),
             "CreateFence");
    HANDLE fence_ev = CreateEventW(NULL, FALSE, FALSE, NULL);
    CHECK_COND(fence_ev != NULL, "CreateEventW fence");
    printf("D3D12_E2E: device+queue+fence created\n");

    /* 2. Render target 256x256 RGBA8_UNORM + RTV heap */
    const UINT W = 256, H = 256;
    D3D12_HEAP_PROPERTIES default_hp;
    memset(&default_hp, 0, sizeof(default_hp));
    default_hp.Type = D3D12_HEAP_TYPE_DEFAULT;

    D3D12_RESOURCE_DESC rt_desc;
    memset(&rt_desc, 0, sizeof(rt_desc));
    rt_desc.Dimension = D3D12_RESOURCE_DIMENSION_TEXTURE2D;
    rt_desc.Width = W;
    rt_desc.Height = H;
    rt_desc.DepthOrArraySize = 1;
    rt_desc.MipLevels = 1;
    rt_desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    rt_desc.SampleDesc.Count = 1;
    rt_desc.Flags = D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET;

    D3D12_CLEAR_VALUE clear_val;
    memset(&clear_val, 0, sizeof(clear_val));
    clear_val.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    clear_val.Color[0] = 0.0f;
    clear_val.Color[1] = 64.0f / 255.0f;
    clear_val.Color[2] = 1.0f;
    clear_val.Color[3] = 1.0f;

    ID3D12Resource *rt = NULL;
    CHECK_HR(ID3D12Device_CreateCommittedResource(
        dev, &default_hp, D3D12_HEAP_FLAG_NONE, &rt_desc,
        D3D12_RESOURCE_STATE_COMMON, &clear_val, &MY_IID_ID3D12Resource, (void **)&rt),
        "CreateCommittedResource(RT)");

    D3D12_DESCRIPTOR_HEAP_DESC rtv_hd;
    memset(&rtv_hd, 0, sizeof(rtv_hd));
    rtv_hd.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
    rtv_hd.NumDescriptors = 1;
    ID3D12DescriptorHeap *rtv_heap = NULL;
    CHECK_HR(ID3D12Device_CreateDescriptorHeap(dev, &rtv_hd, &MY_IID_ID3D12DescriptorHeap, (void **)&rtv_heap),
             "CreateDescriptorHeap(RTV)");
    D3D12_CPU_DESCRIPTOR_HANDLE rtv_cpu = ID3D12DescriptorHeap_GetCPUDescriptorHandleForHeapStart(rtv_heap);
    ID3D12Device_CreateRenderTargetView(dev, rt, NULL, rtv_cpu);

    /* 3. Graphics root signature (1 root CBV b0) + Graphics PSO */
    D3D12_ROOT_PARAMETER gfx_param;
    memset(&gfx_param, 0, sizeof(gfx_param));
    gfx_param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_CBV;
    gfx_param.Descriptor.ShaderRegister = 0;
    gfx_param.Descriptor.RegisterSpace = 0;
    gfx_param.ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;

    D3D12_ROOT_SIGNATURE_DESC gfx_rs_desc;
    memset(&gfx_rs_desc, 0, sizeof(gfx_rs_desc));
    gfx_rs_desc.NumParameters = 1;
    gfx_rs_desc.pParameters = &gfx_param;
    gfx_rs_desc.Flags = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;

    ID3DBlob *rs_blob = NULL, *err_blob = NULL;
    CHECK_HR(D3D12SerializeRootSignature(&gfx_rs_desc, D3D_ROOT_SIGNATURE_VERSION_1, &rs_blob, &err_blob),
             "D3D12SerializeRootSignature(gfx)");
    ID3D12RootSignature *gfx_rs = NULL;
    CHECK_HR(ID3D12Device_CreateRootSignature(
        dev, 0, ID3D10Blob_GetBufferPointer(rs_blob), ID3D10Blob_GetBufferSize(rs_blob),
        &MY_IID_ID3D12RootSignature, (void **)&gfx_rs), "CreateRootSignature(gfx)");
    ID3D10Blob_Release(rs_blob);
    if (err_blob) ID3D10Blob_Release(err_blob);

    D3D12_INPUT_ELEMENT_DESC ie;
    memset(&ie, 0, sizeof(ie));
    ie.SemanticName = "POSITION";
    ie.Format = DXGI_FORMAT_R32G32B32A32_FLOAT;
    ie.InputSlotClass = D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA;

    D3D12_GRAPHICS_PIPELINE_STATE_DESC gpso_desc;
    memset(&gpso_desc, 0, sizeof(gpso_desc));
    gpso_desc.pRootSignature = gfx_rs;
    gpso_desc.VS.pShaderBytecode = kVsPassthroughDxbc;
    gpso_desc.VS.BytecodeLength = sizeof(kVsPassthroughDxbc);
    gpso_desc.PS.pShaderBytecode = kPsCb0ColorDxbc;
    gpso_desc.PS.BytecodeLength = sizeof(kPsCb0ColorDxbc);
    gpso_desc.BlendState.RenderTarget[0].RenderTargetWriteMask = D3D12_COLOR_WRITE_ENABLE_ALL;
    gpso_desc.SampleMask = 0xFFFFFFFFu;
    gpso_desc.RasterizerState.FillMode = D3D12_FILL_MODE_SOLID;
    gpso_desc.RasterizerState.CullMode = D3D12_CULL_MODE_NONE;
    gpso_desc.InputLayout.pInputElementDescs = &ie;
    gpso_desc.InputLayout.NumElements = 1;
    gpso_desc.PrimitiveTopologyType = D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE;
    gpso_desc.NumRenderTargets = 1;
    gpso_desc.RTVFormats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;
    gpso_desc.SampleDesc.Count = 1;

    ID3D12PipelineState *gfx_pso = NULL;
    CHECK_HR(ID3D12Device_CreateGraphicsPipelineState(
        dev, &gpso_desc, &MY_IID_ID3D12PipelineState, (void **)&gfx_pso),
        "CreateGraphicsPipelineState");

    /* 4. Compute root signature (param 0 = CBV b0, param 1 = descriptor table UAV u0) + Compute PSO */
    D3D12_DESCRIPTOR_RANGE uav_range;
    memset(&uav_range, 0, sizeof(uav_range));
    uav_range.RangeType = D3D12_DESCRIPTOR_RANGE_TYPE_UAV;
    uav_range.NumDescriptors = 1;
    uav_range.BaseShaderRegister = 0;
    uav_range.RegisterSpace = 0;
    uav_range.OffsetInDescriptorsFromTableStart = 0;

    D3D12_ROOT_PARAMETER cs_params[2];
    memset(cs_params, 0, sizeof(cs_params));
    cs_params[0].ParameterType = D3D12_ROOT_PARAMETER_TYPE_CBV;
    cs_params[0].Descriptor.ShaderRegister = 0;
    cs_params[0].Descriptor.RegisterSpace = 0;
    cs_params[0].ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;

    cs_params[1].ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
    cs_params[1].DescriptorTable.NumDescriptorRanges = 1;
    cs_params[1].DescriptorTable.pDescriptorRanges = &uav_range;
    cs_params[1].ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;

    D3D12_ROOT_SIGNATURE_DESC cs_rs_desc;
    memset(&cs_rs_desc, 0, sizeof(cs_rs_desc));
    cs_rs_desc.NumParameters = 2;
    cs_rs_desc.pParameters = cs_params;

    rs_blob = NULL; err_blob = NULL;
    CHECK_HR(D3D12SerializeRootSignature(&cs_rs_desc, D3D_ROOT_SIGNATURE_VERSION_1, &rs_blob, &err_blob),
             "D3D12SerializeRootSignature(cs)");
    ID3D12RootSignature *cs_rs = NULL;
    CHECK_HR(ID3D12Device_CreateRootSignature(
        dev, 0, ID3D10Blob_GetBufferPointer(rs_blob), ID3D10Blob_GetBufferSize(rs_blob),
        &MY_IID_ID3D12RootSignature, (void **)&cs_rs), "CreateRootSignature(cs)");
    ID3D10Blob_Release(rs_blob);
    if (err_blob) ID3D10Blob_Release(err_blob);

    D3D12_COMPUTE_PIPELINE_STATE_DESC cpso_desc;
    memset(&cpso_desc, 0, sizeof(cpso_desc));
    cpso_desc.pRootSignature = cs_rs;
    cpso_desc.CS.pShaderBytecode = kCsWriteCb0Dxbc;
    cpso_desc.CS.BytecodeLength = sizeof(kCsWriteCb0Dxbc);
    ID3D12PipelineState *cs_pso = NULL;
    CHECK_HR(ID3D12Device_CreateComputePipelineState(
        dev, &cpso_desc, &MY_IID_ID3D12PipelineState, (void **)&cs_pso),
        "CreateComputePipelineState");
    printf("D3D12_E2E: graphics+compute PSOs created\n");

    /* 5. Upload buffers (triangle VB, gfx CB, cs CB), UAV buffer, readback buffers */
    const float tri_verts[12] = {
        -0.6f, -0.6f, 0.0f, 1.0f,
         0.0f,  0.6f, 0.0f, 1.0f,
         0.6f, -0.6f, 0.0f, 1.0f,
    };
    ID3D12Resource *vb = create_buffer(dev, D3D12_HEAP_TYPE_UPLOAD, sizeof(tri_verts),
                                       D3D12_RESOURCE_FLAG_NONE, D3D12_RESOURCE_STATE_GENERIC_READ);
    void *mapped = NULL;
    CHECK_HR(ID3D12Resource_Map(vb, 0, NULL, &mapped), "Map(vb)");
    memcpy(mapped, tri_verts, sizeof(tri_verts));
    ID3D12Resource_Unmap(vb, 0, NULL);

    const float green[4] = { 0.0f, 1.0f, 0.0f, 1.0f };
    ID3D12Resource *gfx_cb = create_buffer(dev, D3D12_HEAP_TYPE_UPLOAD, 256,
                                           D3D12_RESOURCE_FLAG_NONE, D3D12_RESOURCE_STATE_GENERIC_READ);
    CHECK_HR(ID3D12Resource_Map(gfx_cb, 0, NULL, &mapped), "Map(gfx_cb)");
    memset(mapped, 0, 256);
    memcpy(mapped, green, sizeof(green));
    ID3D12Resource_Unmap(gfx_cb, 0, NULL);

    const float cs_vals[4] = { 0.25f, 0.5f, 0.75f, 1.0f };
    ID3D12Resource *cs_cb = create_buffer(dev, D3D12_HEAP_TYPE_UPLOAD, 256,
                                          D3D12_RESOURCE_FLAG_NONE, D3D12_RESOURCE_STATE_GENERIC_READ);
    CHECK_HR(ID3D12Resource_Map(cs_cb, 0, NULL, &mapped), "Map(cs_cb)");
    memset(mapped, 0, 256);
    memcpy(mapped, cs_vals, sizeof(cs_vals));
    ID3D12Resource_Unmap(cs_cb, 0, NULL);

    const UINT64 cs_bytes = 64 * 16;
    ID3D12Resource *cs_uav_buf = create_buffer(
        dev, D3D12_HEAP_TYPE_DEFAULT, cs_bytes,
        D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS, D3D12_RESOURCE_STATE_UNORDERED_ACCESS);
    ID3D12Resource *cs_rb_buf = create_buffer(
        dev, D3D12_HEAP_TYPE_READBACK, cs_bytes,
        D3D12_RESOURCE_FLAG_NONE, D3D12_RESOURCE_STATE_COPY_DEST);

    D3D12_DESCRIPTOR_HEAP_DESC uav_hd;
    memset(&uav_hd, 0, sizeof(uav_hd));
    uav_hd.Type = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;
    uav_hd.NumDescriptors = 1;
    uav_hd.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE;
    ID3D12DescriptorHeap *uav_heap = NULL;
    CHECK_HR(ID3D12Device_CreateDescriptorHeap(dev, &uav_hd, &MY_IID_ID3D12DescriptorHeap, (void **)&uav_heap),
             "CreateDescriptorHeap(UAV)");

    D3D12_UNORDERED_ACCESS_VIEW_DESC uav_desc;
    memset(&uav_desc, 0, sizeof(uav_desc));
    uav_desc.Format = DXGI_FORMAT_R32G32B32A32_FLOAT;
    uav_desc.ViewDimension = D3D12_UAV_DIMENSION_BUFFER;
    uav_desc.Buffer.FirstElement = 0;
    uav_desc.Buffer.NumElements = 64;
    D3D12_CPU_DESCRIPTOR_HANDLE uav_cpu = ID3D12DescriptorHeap_GetCPUDescriptorHandleForHeapStart(uav_heap);
    D3D12_GPU_DESCRIPTOR_HANDLE uav_gpu = ID3D12DescriptorHeap_GetGPUDescriptorHandleForHeapStart(uav_heap);
    ID3D12Device_CreateUnorderedAccessView(dev, cs_uav_buf, NULL, &uav_desc, uav_cpu);

    D3D12_PLACED_SUBRESOURCE_FOOTPRINT fp;
    UINT num_rows = 0;
    UINT64 row_size = 0, total_bytes = 0;
    ID3D12Device_GetCopyableFootprints(dev, &rt_desc, 0, 1, 0, &fp, &num_rows, &row_size, &total_bytes);
    CHECK_COND(total_bytes >= (UINT64)W * H * 4 && fp.Footprint.RowPitch >= W * 4, "GetCopyableFootprints");
    ID3D12Resource *rt_rb_buf = create_buffer(
        dev, D3D12_HEAP_TYPE_READBACK, total_bytes,
        D3D12_RESOURCE_FLAG_NONE, D3D12_RESOURCE_STATE_COPY_DEST);

    /* 6. Record graphics triangle draw + texture readback + compute dispatch + buffer readback */
    add_barrier(cl, rt, D3D12_RESOURCE_STATE_COMMON, D3D12_RESOURCE_STATE_RENDER_TARGET);
    ID3D12GraphicsCommandList_ClearRenderTargetView(cl, rtv_cpu, clear_val.Color, 0, NULL);

    D3D12_VIEWPORT vp = { 0.0f, 0.0f, (float)W, (float)H, 0.0f, 1.0f };
    D3D12_RECT sc = { 0, 0, (LONG)W, (LONG)H };
    ID3D12GraphicsCommandList_RSSetViewports(cl, 1, &vp);
    ID3D12GraphicsCommandList_RSSetScissorRects(cl, 1, &sc);
    ID3D12GraphicsCommandList_OMSetRenderTargets(cl, 1, &rtv_cpu, FALSE, NULL);
    ID3D12GraphicsCommandList_SetGraphicsRootSignature(cl, gfx_rs);
    ID3D12GraphicsCommandList_SetPipelineState(cl, gfx_pso);
    ID3D12GraphicsCommandList_SetGraphicsRootConstantBufferView(
        cl, 0, ID3D12Resource_GetGPUVirtualAddress(gfx_cb));
    ID3D12GraphicsCommandList_IASetPrimitiveTopology(cl, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

    D3D12_VERTEX_BUFFER_VIEW vbv;
    vbv.BufferLocation = ID3D12Resource_GetGPUVirtualAddress(vb);
    vbv.SizeInBytes = sizeof(tri_verts);
    vbv.StrideInBytes = 16;
    ID3D12GraphicsCommandList_IASetVertexBuffers(cl, 0, 1, &vbv);
    ID3D12GraphicsCommandList_DrawInstanced(cl, 3, 1, 0, 0);

    add_barrier(cl, rt, D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_STATE_COPY_SOURCE);
    D3D12_TEXTURE_COPY_LOCATION dst_loc;
    memset(&dst_loc, 0, sizeof(dst_loc));
    dst_loc.pResource = rt_rb_buf;
    dst_loc.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    dst_loc.PlacedFootprint = fp;

    D3D12_TEXTURE_COPY_LOCATION src_loc;
    memset(&src_loc, 0, sizeof(src_loc));
    src_loc.pResource = rt;
    src_loc.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    src_loc.SubresourceIndex = 0;
    ID3D12GraphicsCommandList_CopyTextureRegion(cl, &dst_loc, 0, 0, 0, &src_loc, NULL);

    /* Compute pass */
    ID3D12DescriptorHeap *heaps[1] = { uav_heap };
    ID3D12GraphicsCommandList_SetDescriptorHeaps(cl, 1, heaps);
    ID3D12GraphicsCommandList_SetComputeRootSignature(cl, cs_rs);
    ID3D12GraphicsCommandList_SetPipelineState(cl, cs_pso);
    ID3D12GraphicsCommandList_SetComputeRootConstantBufferView(
        cl, 0, ID3D12Resource_GetGPUVirtualAddress(cs_cb));
    ID3D12GraphicsCommandList_SetComputeRootDescriptorTable(cl, 1, uav_gpu);
    ID3D12GraphicsCommandList_Dispatch(cl, 1, 1, 1);

    D3D12_RESOURCE_BARRIER uav_bar;
    memset(&uav_bar, 0, sizeof(uav_bar));
    uav_bar.Type = D3D12_RESOURCE_BARRIER_TYPE_UAV;
    uav_bar.UAV.pResource = cs_uav_buf;
    ID3D12GraphicsCommandList_ResourceBarrier(cl, 1, &uav_bar);
    add_barrier(cl, cs_uav_buf, D3D12_RESOURCE_STATE_UNORDERED_ACCESS, D3D12_RESOURCE_STATE_COPY_SOURCE);
    ID3D12GraphicsCommandList_CopyResource(cl, cs_rb_buf, cs_uav_buf);

    CHECK_HR(ID3D12GraphicsCommandList_Close(cl), "CommandList::Close");

    ID3D12CommandList *lists[1] = { (ID3D12CommandList *)cl };
    ID3D12CommandQueue_ExecuteCommandLists(queue, 1, lists);
    CHECK_HR(ID3D12CommandQueue_Signal(queue, fence, 1), "CommandQueue::Signal");
    CHECK_HR(ID3D12Fence_SetEventOnCompletion(fence, 1, fence_ev), "Fence::SetEventOnCompletion");
    CHECK_COND(WaitForSingleObject(fence_ev, 10000) == WAIT_OBJECT_0, "WaitForSingleObject(fence)");
    CHECK_COND(ID3D12Fence_GetCompletedValue(fence) == 1, "Fence::GetCompletedValue == 1");
    printf("D3D12_E2E: fence signaled value=1\n");

    /* 7. Readback & assert exact pixels */
    const uint8_t *px_base = NULL;
    CHECK_HR(ID3D12Resource_Map(rt_rb_buf, 0, NULL, (void **)&px_base), "Map(rt_rb_buf)");
    const UINT pitch = fp.Footprint.RowPitch;
    const uint8_t *c0 = px_base + (size_t)8 * pitch + 8 * 4;
    const uint8_t *mid = px_base + (size_t)128 * pitch + 128 * 4;
    const uint8_t *far_px = px_base + (size_t)248 * pitch + 248 * 4;
    printf("D3D12_E2E: pixels corner=(%u,%u,%u,%u) center=(%u,%u,%u,%u) far=(%u,%u,%u,%u)\n",
           c0[0], c0[1], c0[2], c0[3],
           mid[0], mid[1], mid[2], mid[3],
           far_px[0], far_px[1], far_px[2], far_px[3]);
    fflush(stdout);
    CHECK_COND(c0[0] == 0 && near_u8(c0[1], 64, 1) && c0[2] == 255 && c0[3] == 255, "corner clear blue");
    CHECK_COND(mid[0] == 0 && mid[1] == 255 && mid[2] == 0 && mid[3] == 255, "center triangle green");
    CHECK_COND(far_px[0] == 0 && near_u8(far_px[1], 64, 1) && far_px[2] == 255 && far_px[3] == 255, "far corner clear blue");
    D3D12_RANGE empty_range = { 0, 0 };
    ID3D12Resource_Unmap(rt_rb_buf, 0, &empty_range);

    /* 8. Readback & assert exact compute output */
    const float *got_f = NULL;
    CHECK_HR(ID3D12Resource_Map(cs_rb_buf, 0, NULL, (void **)&got_f), "Map(cs_rb_buf)");
    for (int i = 0; i < 64; ++i) {
        CHECK_COND(got_f[i * 4 + 0] == cs_vals[0]
                       && got_f[i * 4 + 1] == cs_vals[1]
                       && got_f[i * 4 + 2] == cs_vals[2]
                       && got_f[i * 4 + 3] == cs_vals[3],
                   "CS output float4 exact match");
    }
    ID3D12Resource_Unmap(cs_rb_buf, 0, &empty_range);
    printf("D3D12_E2E: compute dispatch verified (64/64 float4 exact match)\n");

    /* 9. Clean teardown */
    CloseHandle(fence_ev);
    ID3D12Resource_Release(rt_rb_buf);
    ID3D12Resource_Release(cs_rb_buf);
    ID3D12Resource_Release(cs_uav_buf);
    ID3D12Resource_Release(cs_cb);
    ID3D12Resource_Release(gfx_cb);
    ID3D12Resource_Release(vb);
    ID3D12DescriptorHeap_Release(uav_heap);
    ID3D12DescriptorHeap_Release(rtv_heap);
    ID3D12PipelineState_Release(cs_pso);
    ID3D12PipelineState_Release(gfx_pso);
    ID3D12RootSignature_Release(cs_rs);
    ID3D12RootSignature_Release(gfx_rs);
    ID3D12Resource_Release(rt);
    ID3D12Fence_Release(fence);
    ID3D12GraphicsCommandList_Release(cl);
    ID3D12CommandAllocator_Release(alloc);
    ID3D12CommandQueue_Release(queue);
    ID3D12Device_Release(dev);

    printf("D3D12_E2E: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;
