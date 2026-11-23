use super::*;
use crate::d3d12::com::thunk_name;

unsafe fn vtbl_slot(obj: *mut c_void, slot: usize) -> usize {
    let vt = *(obj as *const *const usize);
    *vt.add(slot)
}

#[test]
fn vtable_slot_counts_and_key_offsets() {
    // Verify compile-time/class constants and exact slot names in the built vtables.
    assert_eq!(device::ID3D11_DEVICE_BASE_SLOTS, 43);
    assert_eq!(device::ID3D11_DEVICE1_TOTAL_SLOTS, 50);
    assert_eq!(context::ID3D11_CONTEXT_BASE_SLOTS, 115);
    assert_eq!(context::ID3D11_CONTEXT1_TOTAL_SLOTS, 134);
    assert_eq!(resources::ID3D11_RESOURCE_SLOT_COUNT, 10);
    assert_eq!(resources::ID3D11_TYPED_RESOURCE_SLOT_COUNT, 11);
    assert_eq!(resources::ID3D11_VIEW_SLOT_COUNT, 9);
    assert_eq!(device::IDXGI_DEVICE_BASE_SLOTS, 12);
    assert_eq!(device::IDXGI_DEVICE1_TOTAL_SLOTS, 14);
    assert_eq!(device::IDXGI_DEVICE3_TOTAL_SLOTS, 18);
    assert_eq!(resources::IDXGI_RESOURCE1_SLOT_COUNT, 14);
    assert_eq!(resources::IDXGI_SURFACE2_SLOT_COUNT, 14);
    assert_eq!(swapchain::IDXGISWAPCHAIN4_SLOT_COUNT, 41);

    let dev_vt = D3d11Device::class().vtbl;
    let ctx_vt = context::D3d11Context::class().vtbl;
    let buf_vt = resources::D3d11Buffer::class().vtbl;
    let tex2d_vt = resources::D3d11Texture2D::class().vtbl;
    let dxgi_res_vt = resources::DxgiResource::class().vtbl;
    let surf_vt = resources::DxgiSurface::class().vtbl;
    let rtv_vt = resources::D3d11Rtv::class().vtbl;
    let dxgi_dev_vt = device::DxgiDevice::class().vtbl;
    let sc_vt = swapchain::D3d11SwapChain::class().vtbl;

    unsafe {
        // ID3D11Device key slots (0..42) + ID3D11Device1 (43..49)
        assert_eq!(thunk_name(*dev_vt.add(3)).as_deref(), Some("ID3D11Device1::CreateBuffer"));
        assert_eq!(thunk_name(*dev_vt.add(5)).as_deref(), Some("ID3D11Device1::CreateTexture2D"));
        assert_eq!(thunk_name(*dev_vt.add(9)).as_deref(), Some("ID3D11Device1::CreateRenderTargetView"));
        assert_eq!(thunk_name(*dev_vt.add(12)).as_deref(), Some("ID3D11Device1::CreateVertexShader"));
        assert_eq!(thunk_name(*dev_vt.add(15)).as_deref(), Some("ID3D11Device1::CreatePixelShader"));
        assert_eq!(thunk_name(*dev_vt.add(18)).as_deref(), Some("ID3D11Device1::CreateComputeShader"));
        assert_eq!(thunk_name(*dev_vt.add(27)).as_deref(), Some("ID3D11Device1::CreateDeferredContext"));
        assert_eq!(thunk_name(*dev_vt.add(28)).as_deref(), Some("ID3D11Device1::OpenSharedResource"));
        assert_eq!(thunk_name(*dev_vt.add(33)).as_deref(), Some("ID3D11Device1::CheckFeatureSupport"));
        assert_eq!(thunk_name(*dev_vt.add(37)).as_deref(), Some("ID3D11Device1::GetFeatureLevel"));
        assert_eq!(thunk_name(*dev_vt.add(40)).as_deref(), Some("ID3D11Device1::GetImmediateContext"));
        assert_eq!(thunk_name(*dev_vt.add(42)).as_deref(), Some("ID3D11Device1::GetExceptionMode"));
        assert_eq!(thunk_name(*dev_vt.add(43)).as_deref(), Some("ID3D11Device1::GetImmediateContext1"));
        assert_eq!(thunk_name(*dev_vt.add(48)).as_deref(), Some("ID3D11Device1::OpenSharedResource1"));
        assert_eq!(thunk_name(*dev_vt.add(49)).as_deref(), Some("ID3D11Device1::OpenSharedResourceByName"));

        // ID3D11DeviceContext key slots (0..114) + ID3D11DeviceContext1 (115..133)
        assert_eq!(thunk_name(*ctx_vt.add(3)).as_deref(), Some("ID3D11DeviceContext1::GetDevice"));
        assert_eq!(thunk_name(*ctx_vt.add(7)).as_deref(), Some("ID3D11DeviceContext1::VSSetConstantBuffers"));
        assert_eq!(thunk_name(*ctx_vt.add(12)).as_deref(), Some("ID3D11DeviceContext1::DrawIndexed"));
        assert_eq!(thunk_name(*ctx_vt.add(13)).as_deref(), Some("ID3D11DeviceContext1::Draw"));
        assert_eq!(thunk_name(*ctx_vt.add(14)).as_deref(), Some("ID3D11DeviceContext1::Map"));
        assert_eq!(thunk_name(*ctx_vt.add(15)).as_deref(), Some("ID3D11DeviceContext1::Unmap"));
        assert_eq!(thunk_name(*ctx_vt.add(33)).as_deref(), Some("ID3D11DeviceContext1::OMSetRenderTargets"));
        assert_eq!(thunk_name(*ctx_vt.add(39)).as_deref(), Some("ID3D11DeviceContext1::DrawIndexedInstancedIndirect"));
        assert_eq!(thunk_name(*ctx_vt.add(40)).as_deref(), Some("ID3D11DeviceContext1::DrawInstancedIndirect"));
        assert_eq!(thunk_name(*ctx_vt.add(41)).as_deref(), Some("ID3D11DeviceContext1::Dispatch"));
        assert_eq!(thunk_name(*ctx_vt.add(42)).as_deref(), Some("ID3D11DeviceContext1::DispatchIndirect"));
        assert_eq!(thunk_name(*ctx_vt.add(46)).as_deref(), Some("ID3D11DeviceContext1::CopySubresourceRegion"));
        assert_eq!(thunk_name(*ctx_vt.add(47)).as_deref(), Some("ID3D11DeviceContext1::CopyResource"));
        assert_eq!(thunk_name(*ctx_vt.add(48)).as_deref(), Some("ID3D11DeviceContext1::UpdateSubresource"));
        assert_eq!(thunk_name(*ctx_vt.add(49)).as_deref(), Some("ID3D11DeviceContext1::CopyStructureCount"));
        assert_eq!(thunk_name(*ctx_vt.add(50)).as_deref(), Some("ID3D11DeviceContext1::ClearRenderTargetView"));
        assert_eq!(thunk_name(*ctx_vt.add(53)).as_deref(), Some("ID3D11DeviceContext1::ClearDepthStencilView"));
        assert_eq!(thunk_name(*ctx_vt.add(54)).as_deref(), Some("ID3D11DeviceContext1::GenerateMips"));
        assert_eq!(thunk_name(*ctx_vt.add(58)).as_deref(), Some("ID3D11DeviceContext1::ExecuteCommandList"));
        assert_eq!(thunk_name(*ctx_vt.add(110)).as_deref(), Some("ID3D11DeviceContext1::ClearState"));
        assert_eq!(thunk_name(*ctx_vt.add(111)).as_deref(), Some("ID3D11DeviceContext1::Flush"));
        assert_eq!(thunk_name(*ctx_vt.add(112)).as_deref(), Some("ID3D11DeviceContext1::GetType"));
        assert_eq!(thunk_name(*ctx_vt.add(114)).as_deref(), Some("ID3D11DeviceContext1::FinishCommandList"));
        assert_eq!(thunk_name(*ctx_vt.add(115)).as_deref(), Some("ID3D11DeviceContext1::CopySubresourceRegion1"));
        assert_eq!(thunk_name(*ctx_vt.add(132)).as_deref(), Some("ID3D11DeviceContext1::ClearView"));
        assert_eq!(thunk_name(*ctx_vt.add(133)).as_deref(), Some("ID3D11DeviceContext1::DiscardView1"));

        // Buffer / Texture2D / RTV / IDXGIDevice1 / SwapChain key slots
        assert_eq!(thunk_name(*buf_vt.add(7)).as_deref(), Some("ID3D11Buffer::GetType"));
        assert_eq!(thunk_name(*buf_vt.add(10)).as_deref(), Some("ID3D11Buffer::GetDesc"));
        assert_eq!(thunk_name(*tex2d_vt.add(7)).as_deref(), Some("ID3D11Texture2D::GetType"));
        assert_eq!(thunk_name(*tex2d_vt.add(10)).as_deref(), Some("ID3D11Texture2D::GetDesc"));
        assert_eq!(thunk_name(*rtv_vt.add(7)).as_deref(), Some("ID3D11RenderTargetView::GetResource"));
        assert_eq!(thunk_name(*rtv_vt.add(8)).as_deref(), Some("ID3D11RenderTargetView::GetDesc"));
        assert_eq!(thunk_name(*dxgi_dev_vt.add(7)).as_deref(), Some("IDXGIDevice3::GetAdapter"));
        assert_eq!(thunk_name(*dxgi_dev_vt.add(12)).as_deref(), Some("IDXGIDevice3::SetMaximumFrameLatency"));
        assert_eq!(thunk_name(*dxgi_dev_vt.add(14)).as_deref(), Some("IDXGIDevice3::OfferResources"));
        assert_eq!(thunk_name(*dxgi_dev_vt.add(15)).as_deref(), Some("IDXGIDevice3::ReclaimResources"));
        assert_eq!(thunk_name(*dxgi_dev_vt.add(16)).as_deref(), Some("IDXGIDevice3::EnqueueSetEvent"));
        assert_eq!(thunk_name(*dxgi_dev_vt.add(17)).as_deref(), Some("IDXGIDevice3::Trim"));
        assert_eq!(thunk_name(*dxgi_res_vt.add(8)).as_deref(), Some("IDXGIResource1::GetSharedHandle"));
        assert_eq!(thunk_name(*dxgi_res_vt.add(9)).as_deref(), Some("IDXGIResource1::GetUsage"));
        assert_eq!(thunk_name(*dxgi_res_vt.add(10)).as_deref(), Some("IDXGIResource1::SetEvictionPriority"));
        assert_eq!(thunk_name(*dxgi_res_vt.add(11)).as_deref(), Some("IDXGIResource1::GetEvictionPriority"));
        assert_eq!(thunk_name(*dxgi_res_vt.add(12)).as_deref(), Some("IDXGIResource1::CreateSubresourceSurface"));
        assert_eq!(thunk_name(*dxgi_res_vt.add(13)).as_deref(), Some("IDXGIResource1::CreateSharedHandle"));
        assert_eq!(thunk_name(*surf_vt.add(9)).as_deref(), Some("IDXGISurface2::Map"));
        assert_eq!(thunk_name(*surf_vt.add(10)).as_deref(), Some("IDXGISurface2::Unmap"));
        assert_eq!(thunk_name(*surf_vt.add(13)).as_deref(), Some("IDXGISurface2::GetResource"));
        assert_eq!(thunk_name(*sc_vt.add(8)).as_deref(), Some("IDXGISwapChain4(D3D11)::Present"));
        assert_eq!(thunk_name(*sc_vt.add(9)).as_deref(), Some("IDXGISwapChain4(D3D11)::GetBuffer"));
    }
}

#[test]
fn create_device_qi_dxgi_and_unsupported_stages() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0];
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut fl = 0u32;
        let mut ctx: *mut c_void = std::ptr::null_mut();
        let hr = D3D11CreateDevice(
            std::ptr::null_mut(),
            driver_type::HARDWARE,
            std::ptr::null_mut(),
            0,
            levels.as_ptr(),
            levels.len() as u32,
            D3D11_SDK_VERSION,
            &mut dev,
            &mut fl,
            &mut ctx,
        );
        assert_eq!(hr, S_OK);
        assert!(!dev.is_null());
        assert!(!ctx.is_null());
        assert_eq!(fl, D3D_FEATURE_LEVEL_11_1);

        // Check GetFeatureLevel (slot 37) and GetDeviceRemovedReason (slot 39).
        let get_fl: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(vtbl_slot(dev, 37));
        assert_eq!(get_fl(dev), D3D_FEATURE_LEVEL_11_1);
        let get_rem: unsafe extern "win64" fn(*mut c_void) -> HRESULT = std::mem::transmute(vtbl_slot(dev, 39));
        assert_eq!(get_rem(dev), S_OK);

        // QueryInterface -> IDXGIDevice & IDXGIDevice1, and back to ID3D11Device.
        let mut dxgi_dev: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(dev, &IID_IDXGIDevice, &mut dxgi_dev), S_OK);
        assert!(!dxgi_dev.is_null());
        let mut back_dev: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(dxgi_dev, &IID_ID3D11Device, &mut back_dev), S_OK);
        assert_eq!(back_dev, dev);
        com_release(back_dev);

        // IDXGIDevice::GetAdapter (slot 7) -> IDXGIAdapter
        let get_adp: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi_dev, 7));
        let mut adp: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_adp(dxgi_dev, &mut adp), S_OK);
        assert!(!adp.is_null());
        com_release(adp);
        com_release(dxgi_dev);

        // CreateGeometryShader validates bytecode (E_INVALIDARG on null/empty);
        // Hull / Domain shaders remain honest E_NOTIMPL.
        let create_gs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 13));
        let mut gs: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_gs(dev, std::ptr::null(), 0, std::ptr::null_mut(), &mut gs), E_INVALIDARG);
        assert!(gs.is_null());

        let create_hs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 16));
        let mut hs: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_hs(dev, std::ptr::null(), 0, std::ptr::null_mut(), &mut hs), E_NOTIMPL);
        assert!(hs.is_null());

        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn buffer_map_discard_update_subresource_copy_and_readback() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut fl = 0u32;
        let mut ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                &mut fl,
                &mut ctx,
            ),
            S_OK
        );

        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let update_sub: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const D3D11_BOX, *const c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 48));

        // 1. Dynamic buffer -> Map(WRITE_DISCARD) -> Unmap -> CopyResource -> Staging -> Map(READ).
        let dyn_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: usage::DYNAMIC,
            BindFlags: bind_flag::VERTEX_BUFFER,
            CPUAccessFlags: cpu_access::WRITE,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let def_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::CONSTANT_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let stg_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let (mut dyn_b, mut def_b, mut stg_b) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &dyn_desc, std::ptr::null(), &mut dyn_b), S_OK);
        assert_eq!(create_buf(dev, &def_desc, std::ptr::null(), &mut def_b), S_OK);
        assert_eq!(create_buf(dev, &stg_desc, std::ptr::null(), &mut stg_b), S_OK);

        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, dyn_b, 0, map_type::WRITE_DISCARD, 0, &mut mapped), S_OK);
        assert!(!mapped.pData.is_null());
        let pattern: [u32; 16] = std::array::from_fn(|i| 0x1122_0000 + i as u32);
        std::ptr::copy_nonoverlapping(pattern.as_ptr() as *const u8, mapped.pData as *mut u8, 64);
        unmap_fn(ctx, dyn_b, 0);

        copy_res(ctx, stg_b, dyn_b);
        assert_eq!(map_fn(ctx, stg_b, 0, map_type::READ, 0, &mut mapped), S_OK);
        let got = std::slice::from_raw_parts(mapped.pData as *const u32, 16);
        assert_eq!(got, &pattern);
        unmap_fn(ctx, stg_b, 0);

        // 2. UpdateSubresource onto DEFAULT buffer -> CopyResource -> Staging -> Map(READ).
        let pattern2: [u32; 16] = std::array::from_fn(|i| 0xCAFE_B000 + i as u32);
        update_sub(ctx, def_b, 0, std::ptr::null(), pattern2.as_ptr() as *const c_void, 64, 64);
        copy_res(ctx, stg_b, def_b);
        assert_eq!(map_fn(ctx, stg_b, 0, map_type::READ, 0, &mut mapped), S_OK);
        let got2 = std::slice::from_raw_parts(mapped.pData as *const u32, 16);
        assert_eq!(got2, &pattern2);
        unmap_fn(ctx, stg_b, 0);

        com_release(stg_b);
        com_release(def_b);
        com_release(dyn_b);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn texture2d_rtv_clear_flush_and_pixel_readback() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                std::ptr::null_mut(),
                &mut ctx,
            ),
            S_OK
        );

        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let flush_fn: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 111));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));

        let td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28, // R8G8B8A8_UNORM
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let stg_td = D3D11_TEXTURE2D_DESC {
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            ..td
        };
        let (mut tex, mut stg, mut rtv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut tex), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_rtv_fn(dev, tex, std::ptr::null(), &mut rtv), S_OK);

        // Clear to bright green [0, 1, 0, 1] -> RGBA8 LE u32 = 0xFF00FF00.
        let green = [0.0f32, 1.0, 0.0, 1.0];
        clear_rtv(ctx, rtv, &green);
        flush_fn(ctx);
        copy_res(ctx, stg, tex);

        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert!(mapped.RowPitch >= 32);
        let px0 = *(mapped.pData as *const u32);
        assert_eq!(px0, 0xFF00_FF00, "cleared RTV pixel must be exact green");
        unmap_fn(ctx, stg, 0);

        com_release(rtv);
        com_release(stg);
        com_release(tex);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn deferred_context_record_finish_and_execute_command_list() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut imm: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                std::ptr::null_mut(),
                &mut imm,
            ),
            S_OK
        );

        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_def: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 27));

        let mut def_ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_def(dev, 0, &mut def_ctx), S_OK);
        let get_ty: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(vtbl_slot(def_ctx, 112));
        assert_eq!(get_ty(def_ctx), context_type::DEFERRED);
        assert_eq!(get_ty(imm), context_type::IMMEDIATE);

        let td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..td };
        let (mut tex, mut stg, mut rtv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut tex), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_rtv_fn(dev, tex, std::ptr::null(), &mut rtv), S_OK);

        // Record clear to red [1, 0, 0, 1] on deferred context.
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(def_ctx, 50));
        let finish_cl: unsafe extern "win64" fn(*mut c_void, BOOL, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(def_ctx, 114));
        let exec_cl: unsafe extern "win64" fn(*mut c_void, *mut c_void, BOOL) =
            std::mem::transmute(vtbl_slot(imm, 58));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(imm, 47));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(imm, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(imm, 15));

        let red = [1.0f32, 0.0, 0.0, 1.0];
        clear_rtv(def_ctx, rtv, &red);
        let mut cl: *mut c_void = std::ptr::null_mut();
        assert_eq!(finish_cl(def_ctx, FALSE, &mut cl), S_OK);
        assert!(!cl.is_null());

        exec_cl(imm, cl, FALSE);
        copy_res(imm, stg, tex);

        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(imm, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(*(mapped.pData as *const u32), 0xFF00_00FF, "deferred clear must render red");
        unmap_fn(imm, stg, 0);

        com_release(cl);
        com_release(def_ctx);
        com_release(rtv);
        com_release(stg);
        com_release(tex);
        com_release(imm);
        com_release(dev);
    }
}

#[test]
fn device_and_swapchain_clear_and_present_offscreen() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let sc_desc = SwapChainDesc {
            width: 16,
            height: 16,
            refresh_num: 60,
            refresh_den: 1,
            format: 28, // R8G8B8A8_UNORM
            scanline_ordering: 0,
            scaling: 0,
            sample_count: 1,
            sample_quality: 0,
            buffer_usage: 0x20, // DXGI_USAGE_RENDER_TARGET_OUTPUT
            buffer_count: 2,
            output_window: 0, // offscreen headless swapchain
            windowed: TRUE,
            swap_effect: 0, // DXGI_SWAP_EFFECT_DISCARD
            flags: 0,
        };
        let (mut sc, mut dev, mut ctx) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let mut fl = 0u32;
        let hr = D3D11CreateDeviceAndSwapChain(
            std::ptr::null_mut(),
            driver_type::HARDWARE,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION,
            &sc_desc as *const _ as *const c_void,
            &mut sc,
            &mut dev,
            &mut fl,
            &mut ctx,
        );
        assert_eq!(hr, S_OK);
        assert!(!sc.is_null() && !dev.is_null() && !ctx.is_null());

        // GetBuffer(0, IID_ID3D11Texture2D) -> CreateRenderTargetView -> Clear -> Present.
        let get_buf: unsafe extern "win64" fn(*mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(sc, 9));
        let present_fn: unsafe extern "win64" fn(*mut c_void, u32, u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(sc, 8));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));

        let mut bb: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_buf(sc, 0, &IID_ID3D11Texture2D, &mut bb), S_OK);
        assert!(!bb.is_null());

        let mut rtv: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_rtv_fn(dev, bb, std::ptr::null(), &mut rtv), S_OK);

        let blue = [0.0f32, 0.0, 1.0, 1.0];
        clear_rtv(ctx, rtv, &blue);
        assert_eq!(present_fn(sc, 0, 0), S_OK);
        clear_rtv(ctx, rtv, &blue);
        assert_eq!(present_fn(sc, 0, 0), S_OK);

        com_release(rtv);
        com_release(bb);
        com_release(sc);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn port_d3d11_zero_opt_out_returns_unsupported() {
    let prev = std::env::var_os("PORT_D3D11");
    std::env::set_var("PORT_D3D11", "0");
    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut fl = 0u32;
        let mut ctx: *mut c_void = std::ptr::null_mut();
        let hr = D3D11CreateDevice(
            std::ptr::null_mut(),
            driver_type::HARDWARE,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION,
            &mut dev,
            &mut fl,
            &mut ctx,
        );
        assert_eq!(hr, DXGI_ERROR_UNSUPPORTED);
        assert!(dev.is_null());
        assert!(ctx.is_null());
        assert_eq!(fl, 0);

        let mut sc: *mut c_void = std::ptr::null_mut();
        let hr2 = D3D11CreateDeviceAndSwapChain(
            std::ptr::null_mut(),
            driver_type::HARDWARE,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION,
            std::ptr::null(),
            &mut sc,
            &mut dev,
            &mut fl,
            &mut ctx,
        );
        assert_eq!(hr2, DXGI_ERROR_UNSUPPORTED);
    }
    match prev {
        Some(v) => std::env::set_var("PORT_D3D11", v),
        None => std::env::remove_var("PORT_D3D11"),
    }
}

#[test]
fn initial_data_uav_clear_queries_state_getters_and_dxgi_surface() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                std::ptr::null_mut(),
                &mut ctx,
            ),
            S_OK
        );

        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_uav_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 8));
        let create_dsv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_DEPTH_STENCIL_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 10));
        let create_query_fn: unsafe extern "win64" fn(*mut c_void, *const D3D11_QUERY_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 24));
        let check_fmt: unsafe extern "win64" fn(*mut c_void, u32, *mut u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 29));
        let check_msaa: unsafe extern "win64" fn(*mut c_void, u32, u32, *mut u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 30));
        let check_feat: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void, u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 33));

        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let clear_uav_u: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 51));
        let begin_q: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 27));
        let end_q: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 28));
        let get_data_q: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, u32, u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 29));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let rs_get_vp: unsafe extern "win64" fn(*mut c_void, *mut u32, *mut D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 95));
        let clear_st: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 110));

        // Format / MSAA / Feature checks.
        let mut fmt_sup = 0u32;
        assert_eq!(check_fmt(dev, 28, &mut fmt_sup), S_OK);
        assert!(fmt_sup != 0);
        let mut msaa_q = 0u32;
        assert_eq!(check_msaa(dev, 28, 4, &mut msaa_q), S_OK);
        assert!(msaa_q >= 1);
        let mut threading = [0u32; 2];
        assert_eq!(check_feat(dev, feature::THREADING, threading.as_mut_ptr() as *mut c_void, 8), S_OK);
        assert_eq!(threading, [1, 1]);

        // Initial-data Texture2D + IDXGISurface QI + readback verification.
        let init_px: [u32; 16] = std::array::from_fn(|i| 0xFF10_2000 + i as u32);
        let init_sd = D3D11_SUBRESOURCE_DATA {
            pSysMem: init_px.as_ptr() as *const c_void,
            SysMemPitch: 16,
            SysMemSlicePitch: 64,
        };
        let td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..td };
        let (mut tex, mut stg) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &td, &init_sd, &mut tex), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);

        // QI Texture2D -> IDXGISurface
        let mut surf: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(tex, &IID_IDXGISurface, &mut surf), S_OK);
        let surf_get_desc: unsafe extern "win64" fn(*mut c_void, *mut DXGI_SURFACE_DESC) -> HRESULT =
            std::mem::transmute(vtbl_slot(surf, 8));
        let mut sdesc = DXGI_SURFACE_DESC::default();
        assert_eq!(surf_get_desc(surf, &mut sdesc), S_OK);
        assert_eq!((sdesc.Width, sdesc.Height, sdesc.Format), (4, 4, 28));
        com_release(surf);

        copy_res(ctx, stg, tex);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        for y in 0..4usize {
            let row = std::slice::from_raw_parts((mapped.pData as *const u8).add(y * mapped.RowPitch as usize) as *const u32, 4);
            assert_eq!(row, &init_px[y * 4..y * 4 + 4]);
        }
        unmap_fn(ctx, stg, 0);

        // UAV buffer ClearUnorderedAccessViewUint + readback.
        let uav_bdesc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::UNORDERED_ACCESS,
            CPUAccessFlags: 0,
            MiscFlags: misc_flag::BUFFER_ALLOW_RAW_VIEWS,
            StructureByteStride: 0,
        };
        let rb_bdesc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let (mut uav_buf, mut rb_buf, mut uav) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &uav_bdesc, std::ptr::null(), &mut uav_buf), S_OK);
        assert_eq!(create_buf(dev, &rb_bdesc, std::ptr::null(), &mut rb_buf), S_OK);
        let uav_desc = D3D11_UNORDERED_ACCESS_VIEW_DESC {
            Format: 39, // R32_TYPELESS (raw)
            ViewDimension: uav_dim::BUFFER,
            u: [0, 16, 1], // RAW flag = 1
        };
        assert_eq!(create_uav_fn(dev, uav_buf, &uav_desc, &mut uav), S_OK);
        let pat = [0xDEAD_C0DEu32, 0, 0, 0];
        clear_uav_u(ctx, uav, &pat);
        copy_res(ctx, rb_buf, uav_buf);
        assert_eq!(map_fn(ctx, rb_buf, 0, map_type::READ, 0, &mut mapped), S_OK);
        let words = std::slice::from_raw_parts(mapped.pData as *const u32, 16);
        assert!(words.iter().all(|&w| w == 0xDEAD_C0DE));
        unmap_fn(ctx, rb_buf, 0);

        // Depth-stencil texture + DSV + ClearDepthStencilView.
        let ds_td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 1,
            Format: 40, // D32_FLOAT
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::DEPTH_STENCIL,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let (mut ds_tex, mut dsv) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &ds_td, std::ptr::null(), &mut ds_tex), S_OK);
        assert_eq!(create_dsv_fn(dev, ds_tex, std::ptr::null(), &mut dsv), S_OK);
        context::clear_depth_stencil_view(ctx as context::PCtx, dsv, clear_flag::DEPTH, 0.5, 0);

        // Queries: EVENT and TIMESTAMP_DISJOINT.
        let mut ev_q: *mut c_void = std::ptr::null_mut();
        let mut ts_q: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_query_fn(dev, &D3D11_QUERY_DESC { Query: query_kind::EVENT, MiscFlags: 0 }, &mut ev_q), S_OK);
        assert_eq!(create_query_fn(dev, &D3D11_QUERY_DESC { Query: query_kind::TIMESTAMP_DISJOINT, MiscFlags: 0 }, &mut ts_q), S_OK);
        begin_q(ctx, ts_q);
        end_q(ctx, ev_q);
        end_q(ctx, ts_q);
        let mut ev_done: BOOL = 0;
        assert_eq!(get_data_q(ctx, ev_q, &mut ev_done as *mut _ as *mut c_void, 4, 0), S_OK);
        assert_eq!(ev_done, TRUE);
        let mut ts_data = D3D11_QUERY_DATA_TIMESTAMP_DISJOINT::default();
        assert_eq!(get_data_q(ctx, ts_q, &mut ts_data as *mut _ as *mut c_void, 16, 0), S_OK);
        assert!(ts_data.Frequency > 0);
        assert_eq!(ts_data.Disjoint, FALSE);

        // State getter round-trip & ClearState.
        let vp = D3D11_VIEWPORT { TopLeftX: 1.0, TopLeftY: 2.0, Width: 320.0, Height: 240.0, MinDepth: 0.0, MaxDepth: 1.0 };
        rs_set_vp(ctx, 1, &vp);
        let mut n_vp = 1u32;
        let mut got_vp = D3D11_VIEWPORT::default();
        rs_get_vp(ctx, &mut n_vp, &mut got_vp);
        assert_eq!(n_vp, 1);
        assert_eq!(got_vp, vp);
        clear_st(ctx);
        let mut n_vp2 = 0u32;
        rs_get_vp(ctx, &mut n_vp2, std::ptr::null_mut());
        assert_eq!(n_vp2, 0);

        com_release(ts_q);
        com_release(ev_q);
        com_release(dsv);
        com_release(ds_tex);
        com_release(uav);
        com_release(rb_buf);
        com_release(uav_buf);
        com_release(stg);
        com_release(tex);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn sm5_compute_shader_dispatch_end_to_end_on_metal() {
    if d3d12_metal::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
        return;
    }
    // Minimal hand-assembled SM5 compute shader (`cs_5_0`):
    //   dcl_constantbuffer cb0[1], immediateIndexed
    //   dcl_uav_typed_buffer (float,float,float,float) u0
    //   dcl_input vThreadID.x
    //   dcl_thread_group 64, 1, 1
    //   store_uav_typed u0.xyzw, vThreadID.xxxx, cb0[0].xyzw
    //   ret
    let words = |w: &[u32]| -> Vec<u8> { w.iter().flat_map(|v| v.to_le_bytes()).collect() };
    const EMPTY_SIG: [u8; 8] = [0, 0, 0, 0, 8, 0, 0, 0];
    #[rustfmt::skip]
    let shex = words(&[
        0x0005_0050, 25,
        0x0100_086a,
        0x0400_0059, 0x0020_8e46, 0, 1,
        0x0400_089c, 0x0011_e000, 0, 0x5555,
        0x0200_005f, 0x0002_0012,
        0x0400_009b, 64, 1, 1,
        0x0700_00a4, 0x0011_e0f2, 0, 0x0002_0006, 0x0020_8e46, 0, 0,
        0x0100_003e,
    ]);
    let cs_dxbc = dxil_metal::dxbc::build_container(&[
        (*b"ISGN", &EMPTY_SIG),
        (*b"OSGN", &EMPTY_SIG),
        (*b"SHEX", &shex),
    ]);

    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                std::ptr::null_mut(),
                &mut ctx,
            ),
            S_OK
        );

        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_uav_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 8));
        let create_cs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 18));

        let cs_set_uavs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 68));
        let cs_set_shader: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 69));
        let cs_set_cbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 71));
        let dispatch_fn: unsafe extern "win64" fn(*mut c_void, u32, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 41));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));

        let mut cs: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_cs(dev, cs_dxbc.as_ptr() as *const c_void, cs_dxbc.len(), std::ptr::null_mut(), &mut cs), S_OK);
        assert!(!cs.is_null());

        let cb_vals: [f32; 4] = [0.25, 0.5, 0.75, 1.0];
        let cb_init = D3D11_SUBRESOURCE_DATA {
            pSysMem: cb_vals.as_ptr() as *const c_void,
            SysMemPitch: 16,
            SysMemSlicePitch: 16,
        };
        let cb_desc = D3D11_BUFFER_DESC {
            ByteWidth: 16,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::CONSTANT_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let out_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64 * 16, // 64 float4 elements
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::UNORDERED_ACCESS,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let rb_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64 * 16,
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let (mut cb, mut out_buf, mut rb_buf, mut uav) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &cb_desc, &cb_init, &mut cb), S_OK);
        assert_eq!(create_buf(dev, &out_desc, std::ptr::null(), &mut out_buf), S_OK);
        assert_eq!(create_buf(dev, &rb_desc, std::ptr::null(), &mut rb_buf), S_OK);

        let uav_desc = D3D11_UNORDERED_ACCESS_VIEW_DESC {
            Format: 2, // DXGI_FORMAT_R32G32B32A32_FLOAT
            ViewDimension: uav_dim::BUFFER,
            u: [0, 64, 0],
        };
        assert_eq!(create_uav_fn(dev, out_buf, &uav_desc, &mut uav), S_OK);

        cs_set_shader(ctx, cs, std::ptr::null(), 0);
        cs_set_cbs(ctx, 0, 1, &cb);
        cs_set_uavs(ctx, 0, 1, &uav, std::ptr::null());
        dispatch_fn(ctx, 1, 1, 1);

        copy_res(ctx, rb_buf, out_buf);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, rb_buf, 0, map_type::READ, 0, &mut mapped), S_OK);
        let out_f = std::slice::from_raw_parts(mapped.pData as *const [f32; 4], 64);
        for (i, &v) in out_f.iter().enumerate() {
            assert_eq!(v, cb_vals, "SM5 compute shader output element {i}");
        }
        unmap_fn(ctx, rb_buf, 0);

        com_release(uav);
        com_release(rb_buf);
        com_release(out_buf);
        com_release(cb);
        com_release(cs);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn sm5_graphics_vs_ps_draw_and_dxgi_factory_swapchain_on_metal() {
    if d3d12_metal::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
        return;
    }
    let words = |w: &[u32]| -> Vec<u8> { w.iter().flat_map(|v| v.to_le_bytes()).collect() };
    const EMPTY_SIG: [u8; 8] = [0, 0, 0, 0, 8, 0, 0, 0];

    // vs_5_0: dcl_input v0.xyzw; dcl_output_siv o0.xyzw, position; mov o0.xyzw, v0.xyzw; ret
    #[rustfmt::skip]
    let vs_shex = words(&[
        0x0001_0050, 15,
        0x0300_005f, 0x0010_10f2, 0,
        0x0400_0067, 0x0010_20f2, 0, 1,
        0x0500_0036, 0x0010_20f2, 0, 0x0010_1e46, 0,
        0x0100_003e,
    ]);
    let mut vs_isgn = words(&[1, 8, 32, 0, 0, 3, 0]);
    vs_isgn.extend_from_slice(&[0x0f, 0x0f, 0, 0]);
    vs_isgn.extend_from_slice(b"POSITION\0\0\0\0");
    let mut vs_osgn = words(&[1, 8, 32, 0, 1, 3, 0]);
    vs_osgn.extend_from_slice(&[0x0f, 0x00, 0, 0]);
    vs_osgn.extend_from_slice(b"SV_Position\0");
    let vs_dxbc = dxil_metal::dxbc::build_container(&[
        (*b"ISGN", &vs_isgn),
        (*b"OSGN", &vs_osgn),
        (*b"SHEX", &vs_shex),
    ]);

    // ps_5_0: dcl_output o0.xyzw; mov o0.xyzw, l(1.0, 0.5, 0.0, 1.0); ret
    #[rustfmt::skip]
    let ps_shex = words(&[
        0x0000_0050, 14,
        0x0300_0065, 0x0010_20f2, 0,
        0x0800_0036, 0x0010_20f2, 0, 0x0000_4002, 0x3f80_0000, 0x3f00_0000, 0, 0x3f80_0000,
        0x0100_003e,
    ]);
    let mut ps_osgn = words(&[1, 8, 32, 0, 0, 3, 0]);
    ps_osgn.extend_from_slice(&[0x0f, 0x00, 0, 0]);
    ps_osgn.extend_from_slice(b"SV_Target\0\0\0");
    let ps_dxbc = dxil_metal::dxbc::build_container(&[
        (*b"ISGN", &EMPTY_SIG),
        (*b"OSGN", &ps_osgn),
        (*b"SHEX", &ps_shex),
    ]);

    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                std::ptr::null_mut(),
                &mut ctx,
            ),
            S_OK
        );

        // Also verify IDXGIDevice -> GetAdapter -> GetParent(IDXGIFactory) -> CreateSwapChain(dev).
        let mut dxgi_dev: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(dev, &IID_IDXGIDevice1, &mut dxgi_dev), S_OK);
        let get_adp: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi_dev, 7));
        let mut adp: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_adp(dxgi_dev, &mut adp), S_OK);
        let get_parent: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(adp, 6));
        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_parent(adp, &crate::dxgi::IID_IDXGIFactory, &mut fac), S_OK);
        let fac_create_sc: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const SwapChainDesc, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(fac, 10));
        let sc_desc = SwapChainDesc {
            width: 8,
            height: 8,
            refresh_num: 60,
            refresh_den: 1,
            format: 28,
            sample_count: 1,
            buffer_usage: 0x20,
            buffer_count: 2,
            output_window: 0,
            windowed: TRUE,
            swap_effect: 0,
            ..Default::default()
        };
        let mut sc: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_sc(fac, dev, &sc_desc, &mut sc), S_OK);
        assert!(!sc.is_null());
        com_release(sc);
        com_release(fac);
        com_release(adp);
        com_release(dxgi_dev);

        // Now draw a fullscreen triangle with SM5 VS + PS into an 8x8 RGBA8 render target!
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));

        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let draw_fn: unsafe extern "win64" fn(*mut c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 13));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));

        let verts: [[f32; 4]; 3] = [
            [-1.0, -1.0, 0.0, 1.0],
            [-1.0, 3.0, 0.0, 1.0],
            [3.0, -1.0, 0.0, 1.0],
        ];
        let vb_desc = D3D11_BUFFER_DESC {
            ByteWidth: std::mem::size_of_val(&verts) as u32,
            Usage: usage::IMMUTABLE,
            BindFlags: bind_flag::VERTEX_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let vb_init = D3D11_SUBRESOURCE_DATA {
            pSysMem: verts.as_ptr() as *const c_void,
            SysMemPitch: vb_desc.ByteWidth,
            SysMemSlicePitch: vb_desc.ByteWidth,
        };
        let td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..td };

        let (mut vb, mut rt, mut stg, mut rtv) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut vs, mut ps, mut il, mut rs) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut rt), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_rtv_fn(dev, rt, std::ptr::null(), &mut rtv), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_ps(dev, ps_dxbc.as_ptr() as *const c_void, ps_dxbc.len(), std::ptr::null_mut(), &mut ps), S_OK);
        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2, // R32G32B32A32_FLOAT
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1; // CULL_NONE
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        let black = [0.0f32; 4];
        clear_rtv(ctx, rtv, &black);
        om_set_rts(ctx, 1, &rtv, std::ptr::null_mut());
        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 8.0, Height: 8.0, MinDepth: 0.0, MaxDepth: 1.0 };
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        let stride = 16u32;
        let offset = 0u32;
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &offset);
        ia_set_topo(ctx, topology::TRIANGLELIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        ps_set(ctx, ps, std::ptr::null(), 0);
        draw_fn(ctx, 3, 0);

        copy_res(ctx, stg, rt);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        let px0 = *(mapped.pData as *const u32);
        // Output color (1.0, 0.5, 0.0, 1.0) in R8G8B8A8_UNORM = R=0xFF, G=0x80 (or 0x7F), B=0x00, A=0xFF.
        let r = px0 & 0xFF;
        let g = (px0 >> 8) & 0xFF;
        let b = (px0 >> 16) & 0xFF;
        let a = (px0 >> 24) & 0xFF;
        assert_eq!((r, b, a), (0xFF, 0x00, 0xFF), "rendered pixel RGBA = ({r},{g},{b},{a})");
        assert!((127..=128).contains(&g), "rendered pixel G = {g}");
        unmap_fn(ctx, stg, 0);

        com_release(rs);
        com_release(il);
        com_release(ps);
        com_release(vs);
        com_release(rtv);
        com_release(stg);
        com_release(rt);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}

// -----------------------------------------------------------------------
// SM 5.0 DXBC shader builders for Metal 4 GPU tests
// -----------------------------------------------------------------------

const EMPTY_SIG: [u8; 8] = [0, 0, 0, 0, 8, 0, 0, 0];

fn le_words(w: &[u32]) -> Vec<u8> {
    w.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn osgn_one_rt() -> Vec<u8> {
    let mut s = le_words(&[1, 8, 32, 0, 0, 3, 0]);
    s.extend_from_slice(&[0x0f, 0x00, 0, 0]);
    s.extend_from_slice(b"SV_Target\0\0\0");
    s
}

/// `vs_5_0`: passthrough `v0.xyzw` (`POSITION`) -> `o0.xyzw` (`SV_Position`).
fn sm5_vs_passthrough() -> Vec<u8> {
    #[rustfmt::skip]
    let shex = le_words(&[
        0x0001_0050, 15,
        0x0300_005f, 0x0010_10f2, 0,
        0x0400_0067, 0x0010_20f2, 0, 1,
        0x0500_0036, 0x0010_20f2, 0, 0x0010_1e46, 0,
        0x0100_003e,
    ]);
    let mut isgn = le_words(&[1, 8, 32, 0, 0, 3, 0]);
    isgn.extend_from_slice(&[0x0f, 0x0f, 0, 0]);
    isgn.extend_from_slice(b"POSITION\0\0\0\0");
    let mut osgn = le_words(&[1, 8, 32, 0, 1, 3, 0]);
    osgn.extend_from_slice(&[0x0f, 0x00, 0, 0]);
    osgn.extend_from_slice(b"SV_Position\0");
    dxil_metal::dxbc::build_container(&[(*b"ISGN", &isgn), (*b"OSGN", &osgn), (*b"SHEX", &shex)])
}

/// `ps_5_0`: outputs `cb0[0].xyzw` to `SV_Target0`.
fn sm5_ps_cb0_color() -> Vec<u8> {
    #[rustfmt::skip]
    let shex = le_words(&[
        0x0000_0050, 17,
        0x0100_086a,
        0x0400_0059, 0x0020_8e46, 0, 1,
        0x0300_0065, 0x0010_20f2, 0,
        0x0600_0036, 0x0010_20f2, 0, 0x0020_8e46, 0, 0,
        0x0100_003e,
    ]);
    let osgn = osgn_one_rt();
    dxil_metal::dxbc::build_container(&[(*b"ISGN", &EMPTY_SIG), (*b"OSGN", &osgn), (*b"SHEX", &shex)])
}

/// `ps_5_0`: writes distinct colors to two MRTs (`SV_Target0` = red, `SV_Target1` = blue).
fn sm5_ps_mrt2_red_blue() -> Vec<u8> {
    #[rustfmt::skip]
    let shex = le_words(&[
        0x0000_0050, 25,
        0x0300_0065, 0x0010_20f2, 0,
        0x0300_0065, 0x0010_20f2, 1,
        0x0800_0036, 0x0010_20f2, 0, 0x0000_4002, 0x3f80_0000, 0, 0, 0x3f80_0000,
        0x0800_0036, 0x0010_20f2, 1, 0x0000_4002, 0, 0, 0x3f80_0000, 0x3f80_0000,
        0x0100_003e,
    ]);
    let mut osgn = le_words(&[
        2, 8,
        // elem 0: SV_Target0 (reg 0)
        56, 0, 0, 3, 0, 0x0000_000f,
        // elem 1: SV_Target1 (reg 1)
        56, 1, 0, 3, 1, 0x0000_000f,
    ]);
    osgn.extend_from_slice(b"SV_Target\0\0\0");
    dxil_metal::dxbc::build_container(&[(*b"ISGN", &EMPTY_SIG), (*b"OSGN", &osgn), (*b"SHEX", &shex)])
}

/// `ps_5_0`: samples `t0` (dimension token `dcl_res_dim`: `6` = TextureCube, `8` = Texture2DArray)
/// with sampler `s0` using coordinate `cb0[0].xyz`.
fn sm5_ps_sample_3d_coord(dcl_res_dim: u32) -> Vec<u8> {
    let dcl_res_op = 0x0400_0058 | ((dcl_res_dim & 0x1f) << 11);
    #[rustfmt::skip]
    let shex = le_words(&[
        0x0000_0050, 28,
        0x0100_086a,
        0x0400_0059, 0x0020_8e46, 0, 1,
        0x0300_005a, 0x0010_6000, 0,
        dcl_res_op, 0x0010_7000, 0, 0x5555,
        0x0300_0065, 0x0010_20f2, 0,
        0x0a00_0045, 0x0010_20f2, 0, 0x0020_8e46, 0, 0, 0x0010_7e46, 0, 0x0010_6000, 0,
        0x0100_003e,
    ]);
    let osgn = osgn_one_rt();
    dxil_metal::dxbc::build_container(&[(*b"ISGN", &EMPTY_SIG), (*b"OSGN", &osgn), (*b"SHEX", &shex)])
}

/// `cs_5_0`: `[numthreads(4, 1, 1)]` writing `u0[vThreadID.x] = cb0[0]`.
fn sm5_cs_write_cb0_tg4() -> Vec<u8> {
    #[rustfmt::skip]
    let shex = le_words(&[
        0x0005_0050, 25,
        0x0100_086a,
        0x0400_0059, 0x0020_8e46, 0, 1,
        0x0400_089c, 0x0011_e000, 0, 0x5555,
        0x0200_005f, 0x0002_0012,
        0x0400_009b, 4, 1, 1,
        0x0700_00a4, 0x0011_e0f2, 0, 0x0002_0006, 0x0020_8e46, 0, 0,
        0x0100_003e,
    ]);
    dxil_metal::dxbc::build_container(&[(*b"ISGN", &EMPTY_SIG), (*b"OSGN", &EMPTY_SIG), (*b"SHEX", &shex)])
}

/// `cs_5_0`: `[numthreads(4, 1, 1)]` calling `imm_atomic_alloc r0.x, u0`
/// (`u0.IncrementCounter()`) on a 16-byte-stride structured UAV and storing
/// `float4(1.0, 2.0, 3.0, 4.0)` at `u0[r0.x]`.
fn sm5_cs_uav_increment_counter() -> Vec<u8> {
    #[rustfmt::skip]
    let shex = le_words(&[
        0x0005_0050, 31,
        0x0100_086a,
        // dcl_uav_structured_opc u0, 16 (bit 23 = HAS_ORDER_PRESERVING_COUNTER)
        0x0480_009e, 0x0011_e000, 0, 16,
        // dcl_temps 1
        0x0200_0068, 1,
        // dcl_thread_group 4, 1, 1
        0x0400_009b, 4, 1, 1,
        // imm_atomic_alloc r0.x, u0
        0x0500_00b2, 0x0010_0012, 0, 0x0011_e000, 0,
        // store_structured u0.xyzw, r0.x, l(0), l(1.0, 2.0, 3.0, 4.0)
        0x0c00_00a8, 0x0011_e0f2, 0, 0x0010_000a, 0, 0x0000_4001, 0,
            0x0000_4002, 0x3f80_0000, 0x4000_0000, 0x4040_0000, 0x4080_0000,
        // ret
        0x0100_003e,
    ]);
    dxil_metal::dxbc::build_container(&[(*b"ISGN", &EMPTY_SIG), (*b"OSGN", &EMPTY_SIG), (*b"SHEX", &shex)])
}

fn gpu_sm5_ready() -> bool {
    d3d12_metal::metal::device().is_some()
        && dxil_metal::converter_available()
        && dxil_metal::dxilconv::available()
}

unsafe fn make_test_device() -> (*mut c_void, *mut c_void) {
    let mut dev: *mut c_void = std::ptr::null_mut();
    let mut ctx: *mut c_void = std::ptr::null_mut();
    assert_eq!(
        D3D11CreateDevice(
            std::ptr::null_mut(),
            driver_type::HARDWARE,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION,
            &mut dev,
            std::ptr::null_mut(),
            &mut ctx,
        ),
        S_OK
    );
    (dev, ctx)
}

#[test]
fn generate_mips_2d_and_cubemap_immediate_and_deferred_on_metal() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_srv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_SHADER_RESOURCE_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 7));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_def: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 27));

        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let update_sub: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const D3D11_BOX, *const c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 48));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));
        let gen_mips: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 54));
        let exec_cl: unsafe extern "win64" fn(*mut c_void, *mut c_void, BOOL) =
            std::mem::transmute(vtbl_slot(ctx, 58));

        // Part 1: 4x4 2D texture with auto mip chain (MipLevels = 0 -> 3 mips: 4x4, 2x2, 1x1).
        let td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 0,
            ArraySize: 1,
            Format: 28, // R8G8B8A8_UNORM
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: misc_flag::GENERATE_MIPS,
        };
        let stg_td = D3D11_TEXTURE2D_DESC {
            MipLevels: 3,
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            MiscFlags: 0,
            ..td
        };
        let (mut tex, mut stg, mut srv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut tex), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_srv_fn(dev, tex, std::ptr::null(), &mut srv), S_OK);

        // Upload mip 0: 4 solid 2x2 quadrants (TL=Red, TR=Green, BL=Blue, BR=White).
        let red = 0xFF00_00FFu32;
        let green = 0xFF00_FF00u32;
        let blue = 0xFFFF_0000u32;
        let white = 0xFFFF_FFFFu32;
        let mip0: [u32; 16] = [
            red, red, green, green,
            red, red, green, green,
            blue, blue, white, white,
            blue, blue, white, white,
        ];
        update_sub(ctx, tex, 0, std::ptr::null(), mip0.as_ptr() as *const c_void, 16, 64);
        gen_mips(ctx, srv);
        copy_res(ctx, stg, tex);

        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        // Verify mip 0
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        for y in 0..4usize {
            let row = std::slice::from_raw_parts((mapped.pData as *const u8).add(y * mapped.RowPitch as usize) as *const u32, 4);
            assert_eq!(row, &mip0[y * 4..y * 4 + 4], "GenerateMips mip 0 row {y}");
        }
        unmap_fn(ctx, stg, 0);

        // Verify mip 1 (2x2)
        assert_eq!(map_fn(ctx, stg, 1, map_type::READ, 0, &mut mapped), S_OK);
        let r0 = std::slice::from_raw_parts(mapped.pData as *const u32, 2);
        let r1 = std::slice::from_raw_parts((mapped.pData as *const u8).add(mapped.RowPitch as usize) as *const u32, 2);
        assert_eq!(r0, &[red, green], "GenerateMips mip 1 row 0");
        assert_eq!(r1, &[blue, white], "GenerateMips mip 1 row 1");
        unmap_fn(ctx, stg, 1);

        // Verify mip 2 (1x1): average of (R, G, B, W) = (~128, ~128, ~128, 255)
        assert_eq!(map_fn(ctx, stg, 2, map_type::READ, 0, &mut mapped), S_OK);
        let px2 = *(mapped.pData as *const u32);
        let (r, g, b, a) = (px2 & 0xFF, (px2 >> 8) & 0xFF, (px2 >> 16) & 0xFF, (px2 >> 24) & 0xFF);
        assert!(
            (127..=128).contains(&r) && (127..=128).contains(&g) && (127..=128).contains(&b) && a == 255,
            "GenerateMips mip 2 RGBA = ({r},{g},{b},{a})"
        );
        unmap_fn(ctx, stg, 2);

        // Verify partial-mip SRV (MostDetailedMip = 1): overwrite mip 1 with solid cyan and downsample mip 1 -> mip 2 without touching mip 0.
        let cyan = 0xFFFF_FF00u32;
        let mip1_cyan = [cyan; 4];
        update_sub(ctx, tex, 1, std::ptr::null(), mip1_cyan.as_ptr() as *const c_void, 8, 16);
        let sub_srv_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: 28,
            ViewDimension: srv_dim::TEXTURE2D,
            u: [1, u32::MAX, 0, 0],
        };
        let mut sub_srv: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_srv_fn(dev, tex, &sub_srv_desc, &mut sub_srv), S_OK);
        gen_mips(ctx, sub_srv);
        copy_res(ctx, stg, tex);
        assert_eq!(map_fn(ctx, stg, 2, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(*(mapped.pData as *const u32), cyan, "partial-mip GenerateMips must update mip 2 from mip 1");
        unmap_fn(ctx, stg, 2);
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(*(mapped.pData as *const u32), red, "partial-mip GenerateMips must leave mip 0 untouched");
        unmap_fn(ctx, stg, 0);
        com_release(sub_srv);

        // Part 1b: 2D Texture Array (2 slices x 3 mips) GenerateMips
        let arr_td = D3D11_TEXTURE2D_DESC { ArraySize: 2, MipLevels: 3, ..td };
        let arr_stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, MiscFlags: 0, ..arr_td };
        let (mut arr_tex, mut arr_stg, mut arr_srv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &arr_td, std::ptr::null(), &mut arr_tex), S_OK);
        assert_eq!(create_tex2d(dev, &arr_stg_td, std::ptr::null(), &mut arr_stg), S_OK);
        assert_eq!(create_srv_fn(dev, arr_tex, std::ptr::null(), &mut arr_srv), S_OK);
        let s0_px = [green; 16];
        let s1_px = [blue; 16];
        update_sub(ctx, arr_tex, 0, std::ptr::null(), s0_px.as_ptr() as *const c_void, 16, 64);
        update_sub(ctx, arr_tex, 3, std::ptr::null(), s1_px.as_ptr() as *const c_void, 16, 64);
        gen_mips(ctx, arr_srv);
        copy_res(ctx, arr_stg, arr_tex);
        for mip in 0..3u32 {
            assert_eq!(map_fn(ctx, arr_stg, mip, map_type::READ, 0, &mut mapped), S_OK);
            assert_eq!(*(mapped.pData as *const u32), green, "2D array slice 0 mip {mip}");
            unmap_fn(ctx, arr_stg, mip);
            assert_eq!(map_fn(ctx, arr_stg, 3 + mip, map_type::READ, 0, &mut mapped), S_OK);
            assert_eq!(*(mapped.pData as *const u32), blue, "2D array slice 1 mip {mip}");
            unmap_fn(ctx, arr_stg, 3 + mip);
        }
        com_release(arr_srv);
        com_release(arr_stg);
        com_release(arr_tex);

        // Part 2: Cubemap (6 faces x 3 mips) with deferred-context GenerateMips!
        let cube_td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 3,
            ArraySize: 6,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: misc_flag::TEXTURECUBE | misc_flag::GENERATE_MIPS,
        };
        let cube_stg_td = D3D11_TEXTURE2D_DESC {
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            MiscFlags: 0,
            ..cube_td
        };
        let (mut cube, mut cube_stg, mut cube_srv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &cube_td, std::ptr::null(), &mut cube), S_OK);
        assert_eq!(create_tex2d(dev, &cube_stg_td, std::ptr::null(), &mut cube_stg), S_OK);
        assert_eq!(create_srv_fn(dev, cube, std::ptr::null(), &mut cube_srv), S_OK);

        let face_colors: [([f32; 4], u32); 6] = [
            ([1.0, 0.0, 0.0, 1.0], 0xFF00_00FF),
            ([0.0, 1.0, 0.0, 1.0], 0xFF00_FF00),
            ([0.0, 0.0, 1.0, 1.0], 0xFFFF_0000),
            ([1.0, 1.0, 0.0, 1.0], 0xFF00_FFFF),
            ([0.0, 1.0, 1.0, 1.0], 0xFFFF_FF00),
            ([1.0, 0.0, 1.0, 1.0], 0xFFFF_00FF),
        ];
        for (face, &(col, _)) in face_colors.iter().enumerate() {
            let rtv_desc = D3D11_RENDER_TARGET_VIEW_DESC {
                Format: 28,
                ViewDimension: rtv_dim::TEXTURE2DARRAY,
                u: [0, face as u32, 1],
            };
            let mut frtv: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_rtv_fn(dev, cube, &rtv_desc, &mut frtv), S_OK);
            clear_rtv(ctx, frtv, &col);
            com_release(frtv);
        }

        // Record GenerateMips on a deferred context and replay on immediate context.
        let mut def_ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_def(dev, 0, &mut def_ctx), S_OK);
        let def_gen_mips: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(def_ctx, 54));
        let finish_cl: unsafe extern "win64" fn(*mut c_void, BOOL, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(def_ctx, 114));
        def_gen_mips(def_ctx, cube_srv);
        let mut cl: *mut c_void = std::ptr::null_mut();
        assert_eq!(finish_cl(def_ctx, FALSE, &mut cl), S_OK);
        exec_cl(ctx, cl, FALSE);
        copy_res(ctx, cube_stg, cube);

        for (face, &(_, expected)) in face_colors.iter().enumerate() {
            for mip in 0..3u32 {
                let sub = face as u32 * 3 + mip;
                assert_eq!(map_fn(ctx, cube_stg, sub, map_type::READ, 0, &mut mapped), S_OK);
                let got = *(mapped.pData as *const u32);
                assert_eq!(got, expected, "cubemap face {face} mip {mip}");
                unmap_fn(ctx, cube_stg, sub);
            }
        }

        com_release(cl);
        com_release(def_ctx);
        com_release(cube_srv);
        com_release(cube_stg);
        com_release(cube);
        com_release(srv);
        com_release(stg);
        com_release(tex);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn draw_instanced_indirect_and_draw_indexed_instanced_indirect_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let vs_dxbc = sm5_vs_passthrough();
    let ps_dxbc = sm5_ps_cb0_color();
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));
        let create_def: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 27));

        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ps_set_cbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 16));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let ia_set_ib: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 19));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let draw_idx_ind: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 39));
        let draw_ind: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 40));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let update_sub: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const D3D11_BOX, *const c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 48));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));
        let exec_cl: unsafe extern "win64" fn(*mut c_void, *mut c_void, BOOL) =
            std::mem::transmute(vtbl_slot(ctx, 58));

        // Vertices 0..3 are degenerate; vertices 3..6 form a fullscreen triangle (tests StartVertex=3 & BaseVertex=2).
        let verts: [[f32; 4]; 6] = [
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
            [-1.0, -1.0, 0.0, 1.0],
            [-1.0, 3.0, 0.0, 1.0],
            [3.0, -1.0, 0.0, 1.0],
        ];
        let indices: [u16; 6] = [0, 0, 0, 1, 2, 3]; // start_index=3, base_vertex=2 -> verts 3,4,5
        let vb_desc = D3D11_BUFFER_DESC {
            ByteWidth: std::mem::size_of_val(&verts) as u32,
            Usage: usage::IMMUTABLE,
            BindFlags: bind_flag::VERTEX_BUFFER,
            ..Default::default()
        };
        let vb_init = D3D11_SUBRESOURCE_DATA { pSysMem: verts.as_ptr() as *const c_void, SysMemPitch: vb_desc.ByteWidth, SysMemSlicePitch: vb_desc.ByteWidth };
        let ib_desc = D3D11_BUFFER_DESC {
            ByteWidth: std::mem::size_of_val(&indices) as u32,
            Usage: usage::IMMUTABLE,
            BindFlags: bind_flag::INDEX_BUFFER,
            ..Default::default()
        };
        let ib_init = D3D11_SUBRESOURCE_DATA { pSysMem: indices.as_ptr() as *const c_void, SysMemPitch: ib_desc.ByteWidth, SysMemSlicePitch: ib_desc.ByteWidth };
        let cb_desc = D3D11_BUFFER_DESC {
            ByteWidth: 16,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::CONSTANT_BUFFER,
            ..Default::default()
        };
        let args_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: usage::DEFAULT,
            BindFlags: 0,
            MiscFlags: misc_flag::DRAWINDIRECT_ARGS,
            ..Default::default()
        };
        let td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET,
            ..Default::default()
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..td };

        let (mut vb, mut ib, mut cb, mut args_buf) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut rt, mut stg, mut rtv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut vs, mut ps, mut il, mut rs) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_buf(dev, &ib_desc, &ib_init, &mut ib), S_OK);
        assert_eq!(create_buf(dev, &cb_desc, std::ptr::null(), &mut cb), S_OK);
        assert_eq!(create_buf(dev, &args_desc, std::ptr::null(), &mut args_buf), S_OK);
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut rt), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_rtv_fn(dev, rt, std::ptr::null(), &mut rtv), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_ps(dev, ps_dxbc.as_ptr() as *const c_void, ps_dxbc.len(), std::ptr::null_mut(), &mut ps), S_OK);
        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1; // CULL_NONE
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        // Populate indirect buffer:
        // offset 16: DrawInstancedIndirect args [3, 1, 3, 0]
        // offset 32: DrawIndexedInstancedIndirect args [3, 1, 3, 2, 0]
        let raw_args: [u32; 16] = [
            0, 0, 0, 0,
            3, 1, 3, 0,
            3, 1, 3, 2, 0, 0, 0, 0,
        ];
        update_sub(ctx, args_buf, 0, std::ptr::null(), raw_args.as_ptr() as *const c_void, 64, 64);

        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 8.0, Height: 8.0, MinDepth: 0.0, MaxDepth: 1.0 };
        let stride = 16u32;
        let voff = 0u32;

        // Test 1: immediate DrawInstancedIndirect (offset 16) -> bright green
        let black = [0.0f32; 4];
        let c1 = [0.0f32, 1.0, 0.5, 1.0];
        update_sub(ctx, cb, 0, std::ptr::null(), c1.as_ptr() as *const c_void, 16, 16);
        clear_rtv(ctx, rtv, &black);
        om_set_rts(ctx, 1, &rtv, std::ptr::null_mut());
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &voff);
        ia_set_topo(ctx, topology::TRIANGLELIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        ps_set(ctx, ps, std::ptr::null(), 0);
        ps_set_cbs(ctx, 0, 1, &cb);
        draw_ind(ctx, args_buf, 16);

        copy_res(ctx, stg, rt);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        let px1 = *(mapped.pData as *const u32);
        let (r, g, b, a) = (px1 & 0xFF, (px1 >> 8) & 0xFF, (px1 >> 16) & 0xFF, (px1 >> 24) & 0xFF);
        assert_eq!((r, g, a), (0, 255, 255), "DrawInstancedIndirect RGBA=({r},{g},{b},{a})");
        assert!((127..=128).contains(&b), "DrawInstancedIndirect B={b}");
        unmap_fn(ctx, stg, 0);

        // Test 2: deferred DrawIndexedInstancedIndirect (offset 32, R16_UINT IB) -> magenta-ish
        let c2 = [1.0f32, 0.0, 1.0, 1.0];
        update_sub(ctx, cb, 0, std::ptr::null(), c2.as_ptr() as *const c_void, 16, 16);
        let mut def_ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_def(dev, 0, &mut def_ctx), S_OK);
        let finish_cl: unsafe extern "win64" fn(*mut c_void, BOOL, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(def_ctx, 114));
        clear_rtv(def_ctx, rtv, &black);
        om_set_rts(def_ctx, 1, &rtv, std::ptr::null_mut());
        rs_set_vp(def_ctx, 1, &vp);
        rs_set_st(def_ctx, rs);
        ia_set_il(def_ctx, il);
        ia_set_vbs(def_ctx, 0, 1, &vb, &stride, &voff);
        ia_set_ib(def_ctx, ib, 57, 0); // DXGI_FORMAT_R16_UINT
        ia_set_topo(def_ctx, topology::TRIANGLELIST);
        vs_set(def_ctx, vs, std::ptr::null(), 0);
        ps_set(def_ctx, ps, std::ptr::null(), 0);
        ps_set_cbs(def_ctx, 0, 1, &cb);
        draw_idx_ind(def_ctx, args_buf, 32);
        let mut cl: *mut c_void = std::ptr::null_mut();
        assert_eq!(finish_cl(def_ctx, FALSE, &mut cl), S_OK);
        exec_cl(ctx, cl, FALSE);

        copy_res(ctx, stg, rt);
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        let px2 = *(mapped.pData as *const u32);
        assert_eq!(px2, 0xFFFF_00FF, "DrawIndexedInstancedIndirect must render magenta");
        unmap_fn(ctx, stg, 0);

        com_release(cl);
        com_release(def_ctx);
        com_release(rs);
        com_release(il);
        com_release(ps);
        com_release(vs);
        com_release(rtv);
        com_release(stg);
        com_release(rt);
        com_release(args_buf);
        com_release(cb);
        com_release(ib);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn dispatch_indirect_immediate_and_deferred_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let cs_dxbc = sm5_cs_write_cb0_tg4();
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_uav_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 8));
        let create_cs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 18));
        let create_def: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 27));

        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let dispatch_ind: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 42));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let clear_uav_u: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 51));
        let exec_cl: unsafe extern "win64" fn(*mut c_void, *mut c_void, BOOL) =
            std::mem::transmute(vtbl_slot(ctx, 58));
        let cs_set_uavs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 68));
        let cs_set_shader: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 69));
        let cs_set_cbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 71));

        let mut cs: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_cs(dev, cs_dxbc.as_ptr() as *const c_void, cs_dxbc.len(), std::ptr::null_mut(), &mut cs), S_OK);

        let cb_vals: [f32; 4] = [1.5, 2.5, 3.5, 4.5];
        let cb_init = D3D11_SUBRESOURCE_DATA { pSysMem: cb_vals.as_ptr() as *const c_void, SysMemPitch: 16, SysMemSlicePitch: 16 };
        let cb_desc = D3D11_BUFFER_DESC { ByteWidth: 16, Usage: usage::DEFAULT, BindFlags: bind_flag::CONSTANT_BUFFER, ..Default::default() };
        let out_desc = D3D11_BUFFER_DESC { ByteWidth: 16 * 16, Usage: usage::DEFAULT, BindFlags: bind_flag::UNORDERED_ACCESS, ..Default::default() };
        let rb_desc = D3D11_BUFFER_DESC { ByteWidth: 16 * 16, Usage: usage::STAGING, CPUAccessFlags: cpu_access::READ, ..Default::default() };
        // Indirect args at offset 16: (3, 1, 1) -> 3*4 = 12 elements written out of 16.
        let args_words: [u32; 8] = [0, 0, 0, 0, 3, 1, 1, 0];
        let args_init = D3D11_SUBRESOURCE_DATA { pSysMem: args_words.as_ptr() as *const c_void, SysMemPitch: 32, SysMemSlicePitch: 32 };
        let args_desc = D3D11_BUFFER_DESC { ByteWidth: 32, Usage: usage::DEFAULT, MiscFlags: misc_flag::DRAWINDIRECT_ARGS, ..Default::default() };

        let (mut cb, mut out_buf, mut rb_buf, mut args_buf, mut uav) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &cb_desc, &cb_init, &mut cb), S_OK);
        assert_eq!(create_buf(dev, &out_desc, std::ptr::null(), &mut out_buf), S_OK);
        assert_eq!(create_buf(dev, &rb_desc, std::ptr::null(), &mut rb_buf), S_OK);
        assert_eq!(create_buf(dev, &args_desc, &args_init, &mut args_buf), S_OK);

        let uav_desc = D3D11_UNORDERED_ACCESS_VIEW_DESC { Format: 2, ViewDimension: uav_dim::BUFFER, u: [0, 16, 0] };
        assert_eq!(create_uav_fn(dev, out_buf, &uav_desc, &mut uav), S_OK);

        let zeros = [0u32; 4];
        clear_uav_u(ctx, uav, &zeros);
        cs_set_shader(ctx, cs, std::ptr::null(), 0);
        cs_set_cbs(ctx, 0, 1, &cb);
        cs_set_uavs(ctx, 0, 1, &uav, std::ptr::null());
        dispatch_ind(ctx, args_buf, 16);

        copy_res(ctx, rb_buf, out_buf);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, rb_buf, 0, map_type::READ, 0, &mut mapped), S_OK);
        let got = std::slice::from_raw_parts(mapped.pData as *const [f32; 4], 16);
        for i in 0..12 {
            assert_eq!(got[i], cb_vals, "DispatchIndirect active elem {i}");
        }
        for i in 12..16 {
            assert_eq!(got[i], [0.0; 4], "DispatchIndirect untouched tail elem {i}");
        }
        unmap_fn(ctx, rb_buf, 0);

        // Also verify DispatchIndirect recorded & replayed from a deferred context.
        clear_uav_u(ctx, uav, &zeros);
        let mut def_ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_def(dev, 0, &mut def_ctx), S_OK);
        let finish_cl: unsafe extern "win64" fn(*mut c_void, BOOL, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(def_ctx, 114));
        cs_set_shader(def_ctx, cs, std::ptr::null(), 0);
        cs_set_cbs(def_ctx, 0, 1, &cb);
        cs_set_uavs(def_ctx, 0, 1, &uav, std::ptr::null());
        dispatch_ind(def_ctx, args_buf, 16);
        let mut cl: *mut c_void = std::ptr::null_mut();
        assert_eq!(finish_cl(def_ctx, FALSE, &mut cl), S_OK);
        exec_cl(ctx, cl, FALSE);

        copy_res(ctx, rb_buf, out_buf);
        assert_eq!(map_fn(ctx, rb_buf, 0, map_type::READ, 0, &mut mapped), S_OK);
        let got2 = std::slice::from_raw_parts(mapped.pData as *const [f32; 4], 16);
        assert_eq!(got2[0], cb_vals);
        assert_eq!(got2[11], cb_vals);
        assert_eq!(got2[12], [0.0; 4]);
        unmap_fn(ctx, rb_buf, 0);

        com_release(cl);
        com_release(def_ctx);
        com_release(uav);
        com_release(args_buf);
        com_release(rb_buf);
        com_release(out_buf);
        com_release(cb);
        com_release(cs);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn copy_structure_count_and_gpu_uav_counter_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let cs_dxbc = sm5_cs_uav_increment_counter();
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_uav_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 8));
        let create_cs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 18));
        let create_def: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 27));

        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let om_set_rts_uavs: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void, u32, u32, *const *mut c_void, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 34));
        let dispatch_fn: unsafe extern "win64" fn(*mut c_void, u32, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 41));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let copy_struct_cnt: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 49));
        let exec_cl: unsafe extern "win64" fn(*mut c_void, *mut c_void, BOOL) =
            std::mem::transmute(vtbl_slot(ctx, 58));
        let cs_set_uavs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 68));
        let cs_set_shader: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 69));

        let mut cs: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_cs(dev, cs_dxbc.as_ptr() as *const c_void, cs_dxbc.len(), std::ptr::null_mut(), &mut cs), S_OK);

        let struct_desc = D3D11_BUFFER_DESC {
            ByteWidth: 32 * 16,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::UNORDERED_ACCESS,
            MiscFlags: misc_flag::BUFFER_STRUCTURED,
            StructureByteStride: 16,
            ..Default::default()
        };
        let struct_rb_desc = D3D11_BUFFER_DESC {
            ByteWidth: 32 * 16,
            Usage: usage::STAGING,
            CPUAccessFlags: cpu_access::READ,
            ..Default::default()
        };
        let dst_desc = D3D11_BUFFER_DESC {
            ByteWidth: 32,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::CONSTANT_BUFFER,
            ..Default::default()
        };
        let dst_rb_desc = D3D11_BUFFER_DESC {
            ByteWidth: 32,
            Usage: usage::STAGING,
            CPUAccessFlags: cpu_access::READ,
            ..Default::default()
        };

        let (mut sbuf, mut sbuf_rb, mut dst, mut dst_rb, mut uav) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &struct_desc, std::ptr::null(), &mut sbuf), S_OK);
        assert_eq!(create_buf(dev, &struct_rb_desc, std::ptr::null(), &mut sbuf_rb), S_OK);
        assert_eq!(create_buf(dev, &dst_desc, std::ptr::null(), &mut dst), S_OK);
        assert_eq!(create_buf(dev, &dst_rb_desc, std::ptr::null(), &mut dst_rb), S_OK);

        let uav_desc = D3D11_UNORDERED_ACCESS_VIEW_DESC {
            Format: 0,
            ViewDimension: uav_dim::BUFFER,
            u: [0, 32, uav_flag::COUNTER],
        };
        assert_eq!(create_uav_fn(dev, sbuf, &uav_desc, &mut uav), S_OK);

        // 1. Set initial count = 7 via CSSetUnorderedAccessViews, copy to offset 4.
        let init7 = 7u32;
        cs_set_uavs(ctx, 0, 1, &uav, &init7);
        copy_struct_cnt(ctx, dst, 4, uav);

        // 2. Keep sentinel (0xFFFFFFFF) must preserve 7 -> copy to offset 8.
        let keep = D3D11_UAV_COUNTER_KEEP;
        cs_set_uavs(ctx, 0, 1, &uav, &keep);
        copy_struct_cnt(ctx, dst, 8, uav);

        // 3. Dispatch 4-thread compute shader calling IncrementCounter() -> 7 + 4 = 11 -> copy to offset 12.
        cs_set_shader(ctx, cs, std::ptr::null(), 0);
        dispatch_fn(ctx, 1, 1, 1);
        copy_struct_cnt(ctx, dst, 12, uav);

        // 4. Deferred context resets counter to 42 and records CopyStructureCount -> offset 16.
        let mut def_ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_def(dev, 0, &mut def_ctx), S_OK);
        let finish_cl: unsafe extern "win64" fn(*mut c_void, BOOL, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(def_ctx, 114));
        let init42 = 42u32;
        cs_set_uavs(def_ctx, 0, 1, &uav, &init42);
        copy_struct_cnt(def_ctx, dst, 16, uav);
        let mut cl: *mut c_void = std::ptr::null_mut();
        assert_eq!(finish_cl(def_ctx, FALSE, &mut cl), S_OK);
        exec_cl(ctx, cl, FALSE);

        // 5. OMSetRenderTargetsAndUnorderedAccessViews resets counter to 99 -> copy to offset 20.
        let init99 = 99u32;
        om_set_rts_uavs(ctx, u32::MAX, std::ptr::null(), std::ptr::null_mut(), 0, 1, &uav, &init99);
        copy_struct_cnt(ctx, dst, 20, uav);

        // Readback counter destination buffer & structured payload buffer.
        copy_res(ctx, dst_rb, dst);
        copy_res(ctx, sbuf_rb, sbuf);

        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, dst_rb, 0, map_type::READ, 0, &mut mapped), S_OK);
        let words = std::slice::from_raw_parts(mapped.pData as *const u32, 8);
        assert_eq!(words[1], 7, "initial count 7 at offset 4");
        assert_eq!(words[2], 7, "0xFFFFFFFF must keep count 7 at offset 8");
        assert_eq!(words[3], 11, "GPU IncrementCounter x4 from 7 -> 11 at offset 12");
        assert_eq!(words[4], 42, "deferred SetUavCounter + CopyStructureCount -> 42 at offset 16");
        assert_eq!(words[5], 99, "OMSetRenderTargetsAndUAVs initial count -> 99 at offset 20");
        unmap_fn(ctx, dst_rb, 0);

        assert_eq!(map_fn(ctx, sbuf_rb, 0, map_type::READ, 0, &mut mapped), S_OK);
        let elems = std::slice::from_raw_parts(mapped.pData as *const [f32; 4], 32);
        for idx in 7..11 {
            assert_eq!(elems[idx], [1.0, 2.0, 3.0, 4.0], "structured elem {idx} written at allocated counter index");
        }
        unmap_fn(ctx, sbuf_rb, 0);

        com_release(cl);
        com_release(def_ctx);
        com_release(uav);
        com_release(dst_rb);
        com_release(dst);
        com_release(sbuf_rb);
        com_release(sbuf);
        com_release(cs);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn cubemap_per_face_rtv_and_srv_sampling_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let vs_dxbc = sm5_vs_passthrough();
    let ps_solid_dxbc = sm5_ps_cb0_color();
    let ps_cube_dxbc = sm5_ps_sample_3d_coord(6); // D3D10_SB_RESOURCE_DIMENSION_TEXTURECUBE = 6
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_srv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_SHADER_RESOURCE_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 7));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));
        let create_samp: unsafe extern "win64" fn(*mut c_void, *const D3D11_SAMPLER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 23));

        let ps_set_srvs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 8));
        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let ps_set_samps: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 10));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let draw_fn: unsafe extern "win64" fn(*mut c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 13));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ps_set_cbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 16));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let update_sub: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const D3D11_BOX, *const c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 48));

        let verts: [[f32; 4]; 3] = [[-1.0, -1.0, 0.0, 1.0], [-1.0, 3.0, 0.0, 1.0], [3.0, -1.0, 0.0, 1.0]];
        let vb_desc = D3D11_BUFFER_DESC { ByteWidth: 48, Usage: usage::IMMUTABLE, BindFlags: bind_flag::VERTEX_BUFFER, ..Default::default() };
        let vb_init = D3D11_SUBRESOURCE_DATA { pSysMem: verts.as_ptr() as *const c_void, SysMemPitch: 48, SysMemSlicePitch: 48 };
        let cb_desc = D3D11_BUFFER_DESC { ByteWidth: 16, Usage: usage::DEFAULT, BindFlags: bind_flag::CONSTANT_BUFFER, ..Default::default() };

        let cube_td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 6,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: misc_flag::TEXTURECUBE,
        };
        let out_td = D3D11_TEXTURE2D_DESC { ArraySize: 1, MiscFlags: 0, BindFlags: bind_flag::RENDER_TARGET, ..cube_td };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..out_td };

        let (mut vb, mut cb, mut cube, mut out_rt, mut out_stg) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut cube_srv, mut out_rtv, mut samp) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut vs, mut ps_solid, mut ps_cube, mut il, mut rs) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());

        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_buf(dev, &cb_desc, std::ptr::null(), &mut cb), S_OK);
        assert_eq!(create_tex2d(dev, &cube_td, std::ptr::null(), &mut cube), S_OK);
        assert_eq!(create_tex2d(dev, &out_td, std::ptr::null(), &mut out_rt), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut out_stg), S_OK);
        assert_eq!(create_srv_fn(dev, cube, std::ptr::null(), &mut cube_srv), S_OK);
        assert_eq!(create_rtv_fn(dev, out_rt, std::ptr::null(), &mut out_rtv), S_OK);

        // Verify default SRV on TEXTURECUBE Texture2D resolves to srv_dim::TEXTURECUBE.
        let srv_get_desc: unsafe extern "win64" fn(*mut c_void, *mut D3D11_SHADER_RESOURCE_VIEW_DESC) =
            std::mem::transmute(vtbl_slot(cube_srv, 8));
        let mut got_sdesc = D3D11_SHADER_RESOURCE_VIEW_DESC::default();
        srv_get_desc(cube_srv, &mut got_sdesc);
        assert_eq!(got_sdesc.ViewDimension, srv_dim::TEXTURECUBE);

        let sdesc = D3D11_SAMPLER_DESC {
            Filter: 0, // MIN_MAG_MIP_POINT
            AddressU: 3, // CLAMP
            AddressV: 3,
            AddressW: 3,
            MaxAnisotropy: 1,
            ComparisonFunc: 1,
            MaxLOD: f32::MAX,
            ..Default::default()
        };
        assert_eq!(create_samp(dev, &sdesc, &mut samp), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_ps(dev, ps_solid_dxbc.as_ptr() as *const c_void, ps_solid_dxbc.len(), std::ptr::null_mut(), &mut ps_solid), S_OK);
        assert_eq!(create_ps(dev, ps_cube_dxbc.as_ptr() as *const c_void, ps_cube_dxbc.len(), std::ptr::null_mut(), &mut ps_cube), S_OK);
        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1;
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 4.0, Height: 4.0, MinDepth: 0.0, MaxDepth: 1.0 };
        let stride = 16u32;
        let voff = 0u32;
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &voff);
        ia_set_topo(ctx, topology::TRIANGLELIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        ps_set_cbs(ctx, 0, 1, &cb);

        // Render distinct solid colors into all 6 cubemap faces via per-face RTVs.
        let faces: [([f32; 4], [f32; 4], u32); 6] = [
            ([1.0, 0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 0.0], 0xFF00_00FF),  // +X red
            ([0.0, 1.0, 0.0, 1.0], [-1.0, 0.0, 0.0, 0.0], 0xFF00_FF00), // -X green
            ([0.0, 0.0, 1.0, 1.0], [0.0, 1.0, 0.0, 0.0], 0xFFFF_0000),  // +Y blue
            ([1.0, 1.0, 0.0, 1.0], [0.0, -1.0, 0.0, 0.0], 0xFF00_FFFF), // -Y yellow
            ([0.0, 1.0, 1.0, 1.0], [0.0, 0.0, 1.0, 0.0], 0xFFFF_FF00),  // +Z cyan
            ([1.0, 0.0, 1.0, 1.0], [0.0, 0.0, -1.0, 0.0], 0xFFFF_00FF), // -Z magenta
        ];
        ps_set(ctx, ps_solid, std::ptr::null(), 0);
        for (face, &(col, _, _)) in faces.iter().enumerate() {
            let rtv_desc = D3D11_RENDER_TARGET_VIEW_DESC {
                Format: 28,
                ViewDimension: rtv_dim::TEXTURE2DARRAY,
                u: [0, face as u32, 1],
            };
            let mut frtv: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_rtv_fn(dev, cube, &rtv_desc, &mut frtv), S_OK);
            update_sub(ctx, cb, 0, std::ptr::null(), col.as_ptr() as *const c_void, 16, 16);
            om_set_rts(ctx, 1, &frtv, std::ptr::null_mut());
            draw_fn(ctx, 3, 0);
            com_release(frtv);
        }

        // Now bind cubemap as SRV and sample all 6 directions into out_rt!
        om_set_rts(ctx, 1, &out_rtv, std::ptr::null_mut());
        ps_set(ctx, ps_cube, std::ptr::null(), 0);
        ps_set_srvs(ctx, 0, 1, &cube_srv);
        ps_set_samps(ctx, 0, 1, &samp);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        for (face, &(_, dir, expected)) in faces.iter().enumerate() {
            update_sub(ctx, cb, 0, std::ptr::null(), dir.as_ptr() as *const c_void, 16, 16);
            draw_fn(ctx, 3, 0);
            copy_res(ctx, out_stg, out_rt);
            assert_eq!(map_fn(ctx, out_stg, 0, map_type::READ, 0, &mut mapped), S_OK);
            let got = *(mapped.pData as *const u32);
            assert_eq!(got, expected, "cubemap sampled face {face} (dir={dir:?})");
            unmap_fn(ctx, out_stg, 0);
        }

        com_release(rs);
        com_release(il);
        com_release(ps_cube);
        com_release(ps_solid);
        com_release(vs);
        com_release(samp);
        com_release(out_rtv);
        com_release(cube_srv);
        com_release(out_stg);
        com_release(out_rt);
        com_release(cube);
        com_release(cb);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn texture2d_array_per_slice_rtv_and_srv_sampling_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let vs_dxbc = sm5_vs_passthrough();
    let ps_solid_dxbc = sm5_ps_cb0_color();
    let ps_arr_dxbc = sm5_ps_sample_3d_coord(8); // D3D10_SB_RESOURCE_DIMENSION_TEXTURE2DARRAY = 8
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_srv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_SHADER_RESOURCE_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 7));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));
        let create_samp: unsafe extern "win64" fn(*mut c_void, *const D3D11_SAMPLER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 23));

        let ps_set_srvs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 8));
        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let ps_set_samps: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 10));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let draw_fn: unsafe extern "win64" fn(*mut c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 13));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ps_set_cbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 16));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let update_sub: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const D3D11_BOX, *const c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 48));

        let verts: [[f32; 4]; 3] = [[-1.0, -1.0, 0.0, 1.0], [-1.0, 3.0, 0.0, 1.0], [3.0, -1.0, 0.0, 1.0]];
        let vb_desc = D3D11_BUFFER_DESC { ByteWidth: 48, Usage: usage::IMMUTABLE, BindFlags: bind_flag::VERTEX_BUFFER, ..Default::default() };
        let vb_init = D3D11_SUBRESOURCE_DATA { pSysMem: verts.as_ptr() as *const c_void, SysMemPitch: 48, SysMemSlicePitch: 48 };
        let cb_desc = D3D11_BUFFER_DESC { ByteWidth: 16, Usage: usage::DEFAULT, BindFlags: bind_flag::CONSTANT_BUFFER, ..Default::default() };

        let arr_td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 4,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
            ..Default::default()
        };
        let out_td = D3D11_TEXTURE2D_DESC { ArraySize: 1, BindFlags: bind_flag::RENDER_TARGET, ..arr_td };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..out_td };

        let (mut vb, mut cb, mut arr, mut out_rt, mut out_stg) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut arr_srv, mut out_rtv, mut samp) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut vs, mut ps_solid, mut ps_arr, mut il, mut rs) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());

        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_buf(dev, &cb_desc, std::ptr::null(), &mut cb), S_OK);
        assert_eq!(create_tex2d(dev, &arr_td, std::ptr::null(), &mut arr), S_OK);
        assert_eq!(create_tex2d(dev, &out_td, std::ptr::null(), &mut out_rt), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut out_stg), S_OK);
        assert_eq!(create_srv_fn(dev, arr, std::ptr::null(), &mut arr_srv), S_OK);
        assert_eq!(create_rtv_fn(dev, out_rt, std::ptr::null(), &mut out_rtv), S_OK);

        let srv_get_desc: unsafe extern "win64" fn(*mut c_void, *mut D3D11_SHADER_RESOURCE_VIEW_DESC) =
            std::mem::transmute(vtbl_slot(arr_srv, 8));
        let mut got_sdesc = D3D11_SHADER_RESOURCE_VIEW_DESC::default();
        srv_get_desc(arr_srv, &mut got_sdesc);
        assert_eq!(got_sdesc.ViewDimension, srv_dim::TEXTURE2DARRAY);
        assert_eq!(got_sdesc.u[3], 4);

        let sdesc = D3D11_SAMPLER_DESC {
            Filter: 0,
            AddressU: 3,
            AddressV: 3,
            AddressW: 3,
            MaxAnisotropy: 1,
            ComparisonFunc: 1,
            MaxLOD: f32::MAX,
            ..Default::default()
        };
        assert_eq!(create_samp(dev, &sdesc, &mut samp), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_ps(dev, ps_solid_dxbc.as_ptr() as *const c_void, ps_solid_dxbc.len(), std::ptr::null_mut(), &mut ps_solid), S_OK);
        assert_eq!(create_ps(dev, ps_arr_dxbc.as_ptr() as *const c_void, ps_arr_dxbc.len(), std::ptr::null_mut(), &mut ps_arr), S_OK);
        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1;
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 4.0, Height: 4.0, MinDepth: 0.0, MaxDepth: 1.0 };
        let stride = 16u32;
        let voff = 0u32;
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &voff);
        ia_set_topo(ctx, topology::TRIANGLELIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        ps_set_cbs(ctx, 0, 1, &cb);

        let slices: [([f32; 4], u32); 4] = [
            ([1.0, 0.0, 0.0, 1.0], 0xFF00_00FF),
            ([0.0, 1.0, 0.0, 1.0], 0xFF00_FF00),
            ([0.0, 0.0, 1.0, 1.0], 0xFFFF_0000),
            ([1.0, 1.0, 0.0, 1.0], 0xFF00_FFFF),
        ];
        ps_set(ctx, ps_solid, std::ptr::null(), 0);
        for (s, &(col, _)) in slices.iter().enumerate() {
            let rtv_desc = D3D11_RENDER_TARGET_VIEW_DESC {
                Format: 28,
                ViewDimension: rtv_dim::TEXTURE2DARRAY,
                u: [0, s as u32, 1],
            };
            let mut srtv: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_rtv_fn(dev, arr, &rtv_desc, &mut srtv), S_OK);
            update_sub(ctx, cb, 0, std::ptr::null(), col.as_ptr() as *const c_void, 16, 16);
            om_set_rts(ctx, 1, &srtv, std::ptr::null_mut());
            draw_fn(ctx, 3, 0);
            com_release(srtv);
        }

        om_set_rts(ctx, 1, &out_rtv, std::ptr::null_mut());
        ps_set(ctx, ps_arr, std::ptr::null(), 0);
        ps_set_srvs(ctx, 0, 1, &arr_srv);
        ps_set_samps(ctx, 0, 1, &samp);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        for (s, &(_, expected)) in slices.iter().enumerate() {
            let uvw: [f32; 4] = [0.5, 0.5, s as f32, 0.0];
            update_sub(ctx, cb, 0, std::ptr::null(), uvw.as_ptr() as *const c_void, 16, 16);
            draw_fn(ctx, 3, 0);
            copy_res(ctx, out_stg, out_rt);
            assert_eq!(map_fn(ctx, out_stg, 0, map_type::READ, 0, &mut mapped), S_OK);
            let got = *(mapped.pData as *const u32);
            assert_eq!(got, expected, "Texture2DArray sampled slice {s}");
            unmap_fn(ctx, out_stg, 0);
        }

        com_release(rs);
        com_release(il);
        com_release(ps_arr);
        com_release(ps_solid);
        com_release(vs);
        com_release(samp);
        com_release(out_rtv);
        com_release(arr_srv);
        com_release(out_stg);
        com_release(out_rt);
        com_release(arr);
        com_release(cb);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn depth_stencil_less_test_rejects_far_quad_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let vs_dxbc = sm5_vs_passthrough();
    let ps_dxbc = sm5_ps_cb0_color();
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_dsv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_DEPTH_STENCIL_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 10));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_dss: unsafe extern "win64" fn(*mut c_void, *const D3D11_DEPTH_STENCIL_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 21));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));

        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let draw_fn: unsafe extern "win64" fn(*mut c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 13));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ps_set_cbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 16));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let om_set_dss: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 36));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let update_sub: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const D3D11_BOX, *const c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 48));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));

        // 3 fullscreen quads (2 tris each = 6 verts each) at z = 0.25 (near), z = 0.75 (far), z = 0.10 (nearest).
        let quad = |z: f32| -> [[f32; 4]; 6] {
            [
                [-1.0, -1.0, z, 1.0], [-1.0, 1.0, z, 1.0], [1.0, 1.0, z, 1.0],
                [-1.0, -1.0, z, 1.0], [1.0, 1.0, z, 1.0], [1.0, -1.0, z, 1.0],
            ]
        };
        let mut verts = Vec::<[f32; 4]>::new();
        verts.extend_from_slice(&quad(0.25)); // start_vertex 0: near
        verts.extend_from_slice(&quad(0.75)); // start_vertex 6: far
        verts.extend_from_slice(&quad(0.10)); // start_vertex 12: nearest

        let vb_desc = D3D11_BUFFER_DESC {
            ByteWidth: (verts.len() * 16) as u32,
            Usage: usage::IMMUTABLE,
            BindFlags: bind_flag::VERTEX_BUFFER,
            ..Default::default()
        };
        let vb_init = D3D11_SUBRESOURCE_DATA { pSysMem: verts.as_ptr() as *const c_void, SysMemPitch: vb_desc.ByteWidth, SysMemSlicePitch: vb_desc.ByteWidth };
        let cb_desc = D3D11_BUFFER_DESC { ByteWidth: 16, Usage: usage::DEFAULT, BindFlags: bind_flag::CONSTANT_BUFFER, ..Default::default() };

        let rt_td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET,
            ..Default::default()
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..rt_td };
        let ds_td = D3D11_TEXTURE2D_DESC { Format: 40 /* D32_FLOAT */, BindFlags: bind_flag::DEPTH_STENCIL, ..rt_td };

        let (mut vb, mut cb, mut rt, mut stg, mut ds_tex) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut rtv, mut dsv, mut dss, mut rs) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut vs, mut ps, mut il) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());

        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_buf(dev, &cb_desc, std::ptr::null(), &mut cb), S_OK);
        assert_eq!(create_tex2d(dev, &rt_td, std::ptr::null(), &mut rt), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_tex2d(dev, &ds_td, std::ptr::null(), &mut ds_tex), S_OK);
        assert_eq!(create_rtv_fn(dev, rt, std::ptr::null(), &mut rtv), S_OK);
        assert_eq!(create_dsv_fn(dev, ds_tex, std::ptr::null(), &mut dsv), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_ps(dev, ps_dxbc.as_ptr() as *const c_void, ps_dxbc.len(), std::ptr::null_mut(), &mut ps), S_OK);

        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);

        let mut dss_desc = D3D11_DEPTH_STENCIL_DESC::default_d3d11();
        dss_desc.DepthEnable = TRUE;
        dss_desc.DepthWriteMask = 1; // ALL
        dss_desc.DepthFunc = 2; // LESS
        assert_eq!(create_dss(dev, &dss_desc, &mut dss), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1;
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        let black = [0.0f32; 4];
        let green = [0.0f32, 1.0, 0.0, 1.0];
        let red = [1.0f32, 0.0, 0.0, 1.0];
        let blue = [0.0f32, 0.0, 1.0, 1.0];

        clear_rtv(ctx, rtv, &black);
        context::clear_depth_stencil_view(ctx as context::PCtx, dsv, clear_flag::DEPTH, 1.0, 0);
        om_set_rts(ctx, 1, &rtv, dsv);
        om_set_dss(ctx, dss, 0);
        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 8.0, Height: 8.0, MinDepth: 0.0, MaxDepth: 1.0 };
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        let stride = 16u32;
        let voff = 0u32;
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &voff);
        ia_set_topo(ctx, topology::TRIANGLELIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        ps_set(ctx, ps, std::ptr::null(), 0);
        ps_set_cbs(ctx, 0, 1, &cb);

        // Draw 1: near quad (z = 0.25) in green -> passes (0.25 < 1.0), writes depth 0.25.
        update_sub(ctx, cb, 0, std::ptr::null(), green.as_ptr() as *const c_void, 16, 16);
        draw_fn(ctx, 6, 0);

        // Draw 2: far quad (z = 0.75) in red -> rejected by LESS (0.75 < 0.25 is false).
        update_sub(ctx, cb, 0, std::ptr::null(), red.as_ptr() as *const c_void, 16, 16);
        draw_fn(ctx, 6, 6);

        copy_res(ctx, stg, rt);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        let px1 = *(mapped.pData as *const u32);
        assert_eq!(px1, 0xFF00_FF00, "far red quad must be rejected by LESS depth test, preserving near green quad");
        unmap_fn(ctx, stg, 0);

        // Draw 3: even nearer quad (z = 0.10) in blue -> passes (0.10 < 0.25) and overwrites green.
        update_sub(ctx, cb, 0, std::ptr::null(), blue.as_ptr() as *const c_void, 16, 16);
        draw_fn(ctx, 6, 12);
        copy_res(ctx, stg, rt);
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        let px2 = *(mapped.pData as *const u32);
        assert_eq!(px2, 0xFFFF_0000, "nearer blue quad (z=0.10) must pass LESS depth test against z=0.25");
        unmap_fn(ctx, stg, 0);

        com_release(rs);
        com_release(dss);
        com_release(il);
        com_release(ps);
        com_release(vs);
        com_release(dsv);
        com_release(rtv);
        com_release(ds_tex);
        com_release(stg);
        com_release(rt);
        com_release(cb);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn mrt_two_render_targets_distinct_colors_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let vs_dxbc = sm5_vs_passthrough();
    let ps_mrt_dxbc = sm5_ps_mrt2_red_blue();
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));

        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let draw_fn: unsafe extern "win64" fn(*mut c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 13));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));

        let verts: [[f32; 4]; 3] = [[-1.0, -1.0, 0.0, 1.0], [-1.0, 3.0, 0.0, 1.0], [3.0, -1.0, 0.0, 1.0]];
        let vb_desc = D3D11_BUFFER_DESC { ByteWidth: 48, Usage: usage::IMMUTABLE, BindFlags: bind_flag::VERTEX_BUFFER, ..Default::default() };
        let vb_init = D3D11_SUBRESOURCE_DATA { pSysMem: verts.as_ptr() as *const c_void, SysMemPitch: 48, SysMemSlicePitch: 48 };
        let td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET,
            ..Default::default()
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..td };

        let (mut vb, mut rt0, mut rt1, mut stg0, mut stg1) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut rtv0, mut rtv1, mut vs, mut ps, mut il, mut rs) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut rt0), S_OK);
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut rt1), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg0), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg1), S_OK);
        assert_eq!(create_rtv_fn(dev, rt0, std::ptr::null(), &mut rtv0), S_OK);
        assert_eq!(create_rtv_fn(dev, rt1, std::ptr::null(), &mut rtv1), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_ps(dev, ps_mrt_dxbc.as_ptr() as *const c_void, ps_mrt_dxbc.len(), std::ptr::null_mut(), &mut ps), S_OK);
        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1;
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        let black = [0.0f32; 4];
        clear_rtv(ctx, rtv0, &black);
        clear_rtv(ctx, rtv1, &black);
        let rtvs = [rtv0, rtv1];
        om_set_rts(ctx, 2, rtvs.as_ptr(), std::ptr::null_mut());
        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 8.0, Height: 8.0, MinDepth: 0.0, MaxDepth: 1.0 };
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        let stride = 16u32;
        let voff = 0u32;
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &voff);
        ia_set_topo(ctx, topology::TRIANGLELIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        ps_set(ctx, ps, std::ptr::null(), 0);
        draw_fn(ctx, 3, 0);

        copy_res(ctx, stg0, rt0);
        copy_res(ctx, stg1, rt1);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg0, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(*(mapped.pData as *const u32), 0xFF00_00FF, "MRT target 0 must be exact red");
        unmap_fn(ctx, stg0, 0);
        assert_eq!(map_fn(ctx, stg1, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(*(mapped.pData as *const u32), 0xFFFF_0000, "MRT target 1 must be exact blue");
        unmap_fn(ctx, stg1, 0);

        com_release(rs);
        com_release(il);
        com_release(ps);
        com_release(vs);
        com_release(rtv1);
        com_release(rtv0);
        com_release(stg1);
        com_release(stg0);
        com_release(rt1);
        com_release(rt0);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn gpu_driven_append_uav_copy_structure_count_into_draw_instanced_indirect_on_metal() {
    if !gpu_sm5_ready() {
        return;
    }
    let cs_dxbc = sm5_cs_uav_increment_counter();
    let vs_dxbc = sm5_vs_passthrough();
    let ps_dxbc = sm5_ps_cb0_color();
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_uav_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_UNORDERED_ACCESS_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 8));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_cs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 18));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));

        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ps_set_cbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 16));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let draw_ind: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 40));
        let dispatch_fn: unsafe extern "win64" fn(*mut c_void, u32, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 41));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let copy_struct_cnt: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 49));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));
        let cs_set_uavs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 68));
        let cs_set_shader: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 69));

        let verts: [[f32; 4]; 3] = [[-1.0, -1.0, 0.0, 1.0], [-1.0, 3.0, 0.0, 1.0], [3.0, -1.0, 0.0, 1.0]];
        let vb_desc = D3D11_BUFFER_DESC { ByteWidth: 48, Usage: usage::IMMUTABLE, BindFlags: bind_flag::VERTEX_BUFFER, ..Default::default() };
        let vb_init = D3D11_SUBRESOURCE_DATA { pSysMem: verts.as_ptr() as *const c_void, SysMemPitch: 48, SysMemSlicePitch: 48 };
        let green = [0.0f32, 1.0, 0.0, 1.0];
        let cb_desc = D3D11_BUFFER_DESC { ByteWidth: 16, Usage: usage::IMMUTABLE, BindFlags: bind_flag::CONSTANT_BUFFER, ..Default::default() };
        let cb_init = D3D11_SUBRESOURCE_DATA { pSysMem: green.as_ptr() as *const c_void, SysMemPitch: 16, SysMemSlicePitch: 16 };

        // Indirect args buffer starts with VertexCount=3, InstanceCount=0, StartVertex=0, StartInstance=0.
        let init_args: [u32; 4] = [3, 0, 0, 0];
        let args_desc = D3D11_BUFFER_DESC { ByteWidth: 16, Usage: usage::DEFAULT, MiscFlags: misc_flag::DRAWINDIRECT_ARGS, ..Default::default() };
        let args_init = D3D11_SUBRESOURCE_DATA { pSysMem: init_args.as_ptr() as *const c_void, SysMemPitch: 16, SysMemSlicePitch: 16 };

        let append_desc = D3D11_BUFFER_DESC {
            ByteWidth: 16 * 16,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::UNORDERED_ACCESS,
            MiscFlags: misc_flag::BUFFER_STRUCTURED,
            StructureByteStride: 16,
            ..Default::default()
        };
        let rt_td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET,
            ..Default::default()
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..rt_td };

        let (mut vb, mut cb, mut args_buf, mut append_buf, mut append_uav) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut rt, mut stg, mut rtv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut cs, mut vs, mut ps, mut il, mut rs) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_buf(dev, &cb_desc, &cb_init, &mut cb), S_OK);
        assert_eq!(create_buf(dev, &args_desc, &args_init, &mut args_buf), S_OK);
        assert_eq!(create_buf(dev, &append_desc, std::ptr::null(), &mut append_buf), S_OK);
        assert_eq!(create_tex2d(dev, &rt_td, std::ptr::null(), &mut rt), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_rtv_fn(dev, rt, std::ptr::null(), &mut rtv), S_OK);

        // APPEND UAV (default counter == 0)
        let uav_desc = D3D11_UNORDERED_ACCESS_VIEW_DESC {
            Format: 0,
            ViewDimension: uav_dim::BUFFER,
            u: [0, 16, uav_flag::APPEND],
        };
        assert_eq!(create_uav_fn(dev, append_buf, &uav_desc, &mut append_uav), S_OK);
        assert_eq!(create_cs(dev, cs_dxbc.as_ptr() as *const c_void, cs_dxbc.len(), std::ptr::null_mut(), &mut cs), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_ps(dev, ps_dxbc.as_ptr() as *const c_void, ps_dxbc.len(), std::ptr::null_mut(), &mut ps), S_OK);
        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1;
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        // Step 1: Copy default 0 counter into InstanceCount (offset 4) -> DrawInstancedIndirect must draw 0 instances (stays black!).
        let black = [0.0f32, 0.0, 0.0, 1.0];
        clear_rtv(ctx, rtv, &black);
        copy_struct_cnt(ctx, args_buf, 4, append_uav);
        om_set_rts(ctx, 1, &rtv, std::ptr::null_mut());
        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 4.0, Height: 4.0, MinDepth: 0.0, MaxDepth: 1.0 };
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        let stride = 16u32;
        let voff = 0u32;
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &voff);
        ia_set_topo(ctx, topology::TRIANGLELIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        ps_set(ctx, ps, std::ptr::null(), 0);
        ps_set_cbs(ctx, 0, 1, &cb);
        draw_ind(ctx, args_buf, 0);

        copy_res(ctx, stg, rt);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(*(mapped.pData as *const u32), 0xFF00_0000, "0-instance indirect draw must leave RT black");
        unmap_fn(ctx, stg, 0);

        // Step 2: Compute shader appends 4 items -> CopyStructureCount writes InstanceCount=4 -> DrawInstancedIndirect renders green!
        let init0 = 0u32;
        cs_set_shader(ctx, cs, std::ptr::null(), 0);
        cs_set_uavs(ctx, 0, 1, &append_uav, &init0);
        dispatch_fn(ctx, 1, 1, 1);
        copy_struct_cnt(ctx, args_buf, 4, append_uav);
        draw_ind(ctx, args_buf, 0);

        copy_res(ctx, stg, rt);
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(*(mapped.pData as *const u32), 0xFF00_FF00, "GPU-populated InstanceCount=4 indirect draw must render green");
        unmap_fn(ctx, stg, 0);

        com_release(rs);
        com_release(il);
        com_release(ps);
        com_release(vs);
        com_release(cs);
        com_release(rtv);
        com_release(stg);
        com_release(rt);
        com_release(append_uav);
        com_release(append_buf);
        com_release(args_buf);
        com_release(cb);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn dxgi_device3_and_surface2_methods_work_without_vtable_overrun() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let (dev, ctx) = make_test_device();
        let mut dxgi3: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(dev, &IID_IDXGIDevice3, &mut dxgi3), S_OK);
        assert!(!dxgi3.is_null());

        // OfferResources (14), ReclaimResources (15), EnqueueSetEvent (16), Trim (17)
        let offer: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi3, 14));
        let reclaim: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut BOOL) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi3, 15));
        let enqueue_ev: unsafe extern "win64" fn(*mut c_void, HANDLE) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi3, 16));
        let trim: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(vtbl_slot(dxgi3, 17));

        assert_eq!(offer(dxgi3, 0, std::ptr::null(), 1), S_OK);
        let mut discarded: BOOL = TRUE;
        assert_eq!(reclaim(dxgi3, 1, std::ptr::null(), &mut discarded), S_OK);
        assert_eq!(discarded, FALSE);

        let ev = crate::handles::insert(Arc::new(crate::handles::Event::new(true, false, None)));
        assert_eq!(enqueue_ev(dxgi3, ev), S_OK);
        assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev, 5000), 0);
        crate::handles::close(ev);

        trim(dxgi3);

        // IDXGISurface2::GetResource (slot 13)
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let td = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::SHADER_RESOURCE,
            ..Default::default()
        };
        let mut tex: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut tex), S_OK);
        let mut surf2: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(tex, &IID_IDXGISurface2, &mut surf2), S_OK);
        let get_res: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void, *mut u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(surf2, 13));
        let mut parent_tex: *mut c_void = std::ptr::null_mut();
        let mut sub = 99u32;
        assert_eq!(get_res(surf2, &IID_ID3D11Texture2D, &mut parent_tex, &mut sub), S_OK);
        assert_eq!(parent_tex, tex);
        assert_eq!(sub, 0);

        com_release(parent_tex);
        com_release(surf2);
        com_release(tex);
        com_release(dxgi3);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn dxgi_surface_map_write_discard_partial_write_and_gpu_readback_on_metal() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let (dev, ctx) = make_test_device();
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let map_ctx: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_ctx: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));

        let td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 2,
            ArraySize: 1,
            Format: 28, // R8G8B8A8_UNORM
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let stg_td = D3D11_TEXTURE2D_DESC {
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            ..td
        };
        let (mut tex, mut stg, mut rtv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut tex), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_rtv_fn(dev, tex, std::ptr::null(), &mut rtv), S_OK);

        let mut surf: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(tex, &IID_IDXGISurface, &mut surf), S_OK);
        let surf_map_fn: unsafe extern "win64" fn(*mut c_void, *mut DXGI_MAPPED_RECT, u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(surf, 9));
        let surf_unmap_fn: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(surf, 10));

        // Invalid MapFlags must honestly fail with E_INVALIDARG.
        let mut rect = DXGI_MAPPED_RECT { Pitch: 0, pBits: std::ptr::null_mut() };
        assert_eq!(surf_map_fn(surf, &mut rect, 0), E_INVALIDARG);
        assert_eq!(surf_map_fn(surf, &mut rect, dxgi_map::READ | dxgi_map::DISCARD), E_INVALIDARG);
        assert_eq!(surf_map_fn(surf, std::ptr::null_mut(), dxgi_map::READ), E_INVALIDARG);

        // 1) IDXGISurface::Map(WRITE | DISCARD) -> write unique 8x8 pattern -> Unmap -> GPU CopyResource readback.
        assert_eq!(surf_map_fn(surf, &mut rect, dxgi_map::WRITE | dxgi_map::DISCARD), S_OK);
        assert!(rect.Pitch >= 32 && !rect.pBits.is_null());

        // Double-Map while active must fail deterministically.
        let mut rect2 = DXGI_MAPPED_RECT { Pitch: 0, pBits: std::ptr::null_mut() };
        assert_eq!(surf_map_fn(surf, &mut rect2, dxgi_map::READ), DXGI_ERROR_WAS_STILL_DRAWING);
        assert!(rect2.pBits.is_null());

        let pixel_at = |x: usize, y: usize| -> u32 { 0xFF00_0000 | ((y as u32) << 12) | ((x as u32) << 4) | 0x05 };
        for y in 0..8usize {
            let row = rect.pBits.add(y * rect.Pitch as usize) as *mut u32;
            for x in 0..8usize {
                std::ptr::write_unaligned(row.add(x), pixel_at(x, y));
            }
        }
        assert_eq!(surf_unmap_fn(surf), S_OK);

        copy_res(ctx, stg, tex);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_ctx(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        for y in 0..8usize {
            let row = (mapped.pData as *const u8).add(y * mapped.RowPitch as usize) as *const u32;
            for x in 0..8usize {
                let got = std::ptr::read_unaligned(row.add(x));
                assert_eq!(got, pixel_at(x, y), "CopyResource readback mismatch at ({x},{y})");
            }
        }
        unmap_ctx(ctx, stg, 0);

        // 2) IDXGISurface::Map(WRITE) without DISCARD must preserve existing pixels while updating (0, 0).
        assert_eq!(surf_map_fn(surf, &mut rect, dxgi_map::WRITE), S_OK);
        std::ptr::write_unaligned(rect.pBits as *mut u32, 0xDEAD_BEEF);
        assert_eq!(surf_unmap_fn(surf), S_OK);

        assert_eq!(surf_map_fn(surf, &mut rect, dxgi_map::READ), S_OK);
        let p00 = std::ptr::read_unaligned(rect.pBits as *const u32);
        let p57 = std::ptr::read_unaligned((rect.pBits.add(7 * rect.Pitch as usize) as *const u32).add(5));
        assert_eq!(p00, 0xDEAD_BEEF, "partial WRITE must update modified pixel");
        assert_eq!(p57, pixel_at(5, 7), "partial WRITE without DISCARD must preserve untouched pixels");
        assert_eq!(surf_unmap_fn(surf), S_OK);

        // 3) GPU ClearRenderTargetView -> IDXGISurface::Map(READ) sees exact GPU-rendered pixels.
        let cyan = [0.0f32, 1.0, 1.0, 1.0]; // RGBA8 LE = 0xFFFF_FF00
        clear_rtv(ctx, rtv, &cyan);
        assert_eq!(surf_map_fn(surf, &mut rect, dxgi_map::READ), S_OK);
        for y in 0..8usize {
            let row = rect.pBits.add(y * rect.Pitch as usize) as *const u32;
            for x in 0..8usize {
                assert_eq!(
                    std::ptr::read_unaligned(row.add(x)),
                    0xFFFF_FF00,
                    "IDXGISurface::Map(READ) must see GPU-cleared cyan at ({x},{y})"
                );
            }
        }
        assert_eq!(surf_unmap_fn(surf), S_OK);

        // 4) IDXGIResource1::CreateSubresourceSurface for mip 1 (4x4) + Map/Unmap round-trip.
        let mut dxgi_res: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(tex, &IID_IDXGIResource1, &mut dxgi_res), S_OK);
        let create_sub_surf: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi_res, 12));
        let mut mip1_surf: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_sub_surf(dxgi_res, 1, &mut mip1_surf), S_OK);
        let mip1_desc_fn: unsafe extern "win64" fn(*mut c_void, *mut DXGI_SURFACE_DESC) -> HRESULT =
            std::mem::transmute(vtbl_slot(mip1_surf, 8));
        let mut mdesc = DXGI_SURFACE_DESC::default();
        assert_eq!(mip1_desc_fn(mip1_surf, &mut mdesc), S_OK);
        assert_eq!((mdesc.Width, mdesc.Height), (4, 4));
        let mip1_get_res: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void, *mut u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(mip1_surf, 13));
        let mut p_res: *mut c_void = std::ptr::null_mut();
        let mut sub_idx = 99u32;
        assert_eq!(mip1_get_res(mip1_surf, &IID_ID3D11Texture2D, &mut p_res, &mut sub_idx), S_OK);
        assert_eq!(sub_idx, 1);
        com_release(p_res);

        assert_eq!(surf_map_fn(mip1_surf, &mut rect, dxgi_map::WRITE | dxgi_map::DISCARD), S_OK);
        std::ptr::write_unaligned(rect.pBits as *mut u32, 0xCAFE_BABE);
        assert_eq!(surf_unmap_fn(mip1_surf), S_OK);
        assert_eq!(surf_map_fn(mip1_surf, &mut rect, dxgi_map::READ), S_OK);
        assert_eq!(std::ptr::read_unaligned(rect.pBits as *const u32), 0xCAFE_BABE);
        assert_eq!(surf_unmap_fn(mip1_surf), S_OK);

        com_release(mip1_surf);
        com_release(dxgi_res);
        com_release(surf);
        com_release(rtv);
        com_release(stg);
        com_release(tex);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn shared_resource_legacy_nt_named_and_cross_device_coherence_on_metal() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let (dev_a, ctx_a) = make_test_device();
        let (dev_b, ctx_b) = make_test_device();

        let create_buf_a: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_a, 3));
        let create_tex2d_a: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_a, 5));
        let create_rtv_a: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_a, 9));

        let create_buf_b: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_b, 3));
        let create_tex2d_b: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_b, 5));
        let create_rtv_b: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_b, 9));
        let open_shared_b: unsafe extern "win64" fn(*mut c_void, HANDLE, *const Guid, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_b, 28));
        let open_shared1_b: unsafe extern "win64" fn(*mut c_void, HANDLE, *const Guid, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_b, 48));
        let open_shared_name_b: unsafe extern "win64" fn(*mut c_void, LPCWSTR, u32, *const Guid, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev_b, 49));

        let clear_rtv_a: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx_a, 50));
        let clear_rtv_b: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx_b, 50));
        let copy_res_a: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx_a, 47));
        let copy_res_b: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx_b, 47));
        let map_a: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx_a, 14));
        let unmap_a: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx_a, 15));
        let map_b: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx_b, 14));
        let unmap_b: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx_b, 15));
        let update_sub_a: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const D3D11_BOX, *const c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx_a, 48));

        let base_td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28, // R8G8B8A8_UNORM
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let stg_td = D3D11_TEXTURE2D_DESC {
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            ..base_td
        };
        let (mut stg_a, mut stg_b) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d_a(dev_a, &stg_td, std::ptr::null(), &mut stg_a), S_OK);
        assert_eq!(create_tex2d_b(dev_b, &stg_td, std::ptr::null(), &mut stg_b), S_OK);

        // --- Part A: Legacy D3D11_RESOURCE_MISC_SHARED ---
        let shared_td = D3D11_TEXTURE2D_DESC { MiscFlags: misc_flag::SHARED, ..base_td };
        let (mut tex_a, mut unshared_a) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_tex2d_a(dev_a, &shared_td, std::ptr::null(), &mut tex_a), S_OK);
        assert_eq!(create_tex2d_a(dev_a, &base_td, std::ptr::null(), &mut unshared_a), S_OK);

        let (mut dxgi_a, mut dxgi_unshared) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(com_query(tex_a, &IID_IDXGIResource1, &mut dxgi_a), S_OK);
        assert_eq!(com_query(unshared_a, &IID_IDXGIResource1, &mut dxgi_unshared), S_OK);

        let get_shared_h: unsafe extern "win64" fn(*mut c_void, *mut HANDLE) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi_a, 8));
        let get_usage: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi_a, 9));
        let create_shared_h: unsafe extern "win64" fn(*mut c_void, *const c_void, u32, LPCWSTR, *mut HANDLE) -> HRESULT =
            std::mem::transmute(vtbl_slot(dxgi_a, 13));

        let mut u = 0u32;
        assert_eq!(get_usage(dxgi_a, &mut u), S_OK);
        assert_ne!(u & 0x80, 0, "DXGI_USAGE_SHARED must be set");

        // Non-shared resource must reject GetSharedHandle honestly.
        let mut bad_h: HANDLE = std::ptr::null_mut();
        assert_eq!(get_shared_h(dxgi_unshared, &mut bad_h), E_INVALIDARG);
        assert!(bad_h.is_null());

        // Legacy-only shared resource rejects CreateSharedHandle (no SHARED_NTHANDLE).
        assert_eq!(
            create_shared_h(dxgi_a, std::ptr::null(), dxgi_shared_resource::READ, std::ptr::null(), &mut bad_h),
            E_INVALIDARG
        );

        let (mut h1, mut h2): (HANDLE, HANDLE) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(get_shared_h(dxgi_a, &mut h1), S_OK);
        assert_eq!(get_shared_h(dxgi_a, &mut h2), S_OK);
        assert!(!h1.is_null() && h1 == h2, "GetSharedHandle must return stable non-null handle");

        // Open on dev_b as ID3D11Texture2D, IDXGISurface, and verify mismatched IID returns E_NOINTERFACE.
        let (mut tex_b, mut surf_b, mut wrong_buf) =
            (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(open_shared_b(dev_b, h1, &IID_ID3D11Buffer, &mut wrong_buf), E_NOINTERFACE);
        assert!(wrong_buf.is_null());
        assert_eq!(open_shared_b(dev_b, h1, &IID_ID3D11Texture2D, &mut tex_b), S_OK);
        assert_eq!(open_shared_b(dev_b, h1, &IID_IDXGISurface, &mut surf_b), S_OK);
        assert!(!tex_b.is_null() && !surf_b.is_null());

        // Verify tex_b->GetDevice() returns dev_b.
        let get_dev_fn: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) =
            std::mem::transmute(vtbl_slot(tex_b, 3));
        let mut got_dev: *mut c_void = std::ptr::null_mut();
        get_dev_fn(tex_b, &mut got_dev);
        assert_eq!(got_dev, dev_b, "opened shared resource must report opening device");
        com_release(got_dev);

        // Clear on dev_a -> read back via tex_b (CopyResource) AND surf_b (IDXGISurface::Map(READ)) on dev_b!
        let (mut rtv_a, mut rtv_b) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_rtv_a(dev_a, tex_a, std::ptr::null(), &mut rtv_a), S_OK);
        assert_eq!(create_rtv_b(dev_b, tex_b, std::ptr::null(), &mut rtv_b), S_OK);

        let red = [1.0f32, 0.0, 0.0, 1.0];
        clear_rtv_a(ctx_a, rtv_a, &red);

        copy_res_b(ctx_b, stg_b, tex_b);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_b(ctx_b, stg_b, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(std::ptr::read_unaligned(mapped.pData as *const u32), 0xFF00_00FF, "dev_b must see red cleared on dev_a");
        unmap_b(ctx_b, stg_b, 0);

        let surf_map_b: unsafe extern "win64" fn(*mut c_void, *mut DXGI_MAPPED_RECT, u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(surf_b, 9));
        let surf_unmap_b: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(surf_b, 10));
        let mut srect = DXGI_MAPPED_RECT { Pitch: 0, pBits: std::ptr::null_mut() };
        assert_eq!(surf_map_b(surf_b, &mut srect, dxgi_map::READ), S_OK);
        assert_eq!(std::ptr::read_unaligned(srect.pBits as *const u32), 0xFF00_00FF);
        assert_eq!(surf_unmap_b(surf_b), S_OK);

        // Reverse direction: clear blue on dev_b -> read back on dev_a!
        let blue = [0.0f32, 0.0, 1.0, 1.0];
        clear_rtv_b(ctx_b, rtv_b, &blue);
        copy_res_a(ctx_a, stg_a, tex_a);
        assert_eq!(map_a(ctx_a, stg_a, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(std::ptr::read_unaligned(mapped.pData as *const u32), 0xFFFF_0000, "dev_a must see blue cleared on dev_b");
        unmap_a(ctx_a, stg_a, 0);

        // --- Part B: NT shared handle + named lookup (case-insensitive) ---
        let nt_td = D3D11_TEXTURE2D_DESC {
            MiscFlags: misc_flag::SHARED | misc_flag::SHARED_NTHANDLE,
            ..base_td
        };
        let mut nt_a: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_tex2d_a(dev_a, &nt_td, std::ptr::null(), &mut nt_a), S_OK);
        let mut dxgi_nt: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(nt_a, &IID_IDXGIResource1, &mut dxgi_nt), S_OK);

        // SHARED_NTHANDLE resource rejects legacy GetSharedHandle.
        assert_eq!(get_shared_h(dxgi_nt, &mut bad_h), E_INVALIDARG);

        let name_w = crate::strings::to_wide_z("Global\\D3D11_Test_SharedTex");
        let name_lower_w = crate::strings::to_wide_z("global\\d3d11_test_sharedtex");
        let mut h_nt: HANDLE = std::ptr::null_mut();
        assert_eq!(
            create_shared_h(
                dxgi_nt,
                std::ptr::null(),
                dxgi_shared_resource::READ | dxgi_shared_resource::WRITE,
                name_w.as_ptr(),
                &mut h_nt,
            ),
            S_OK
        );
        assert!(!h_nt.is_null());

        // Duplicate live name (case-insensitive) must fail honestly with DXGI_ERROR_NAME_ALREADY_EXISTS.
        assert_eq!(
            create_shared_h(
                dxgi_nt,
                std::ptr::null(),
                dxgi_shared_resource::READ,
                name_lower_w.as_ptr(),
                &mut bad_h,
            ),
            DXGI_ERROR_NAME_ALREADY_EXISTS
        );

        let (mut nt_b1, mut nt_b2) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(open_shared1_b(dev_b, h_nt, &IID_ID3D11Texture2D, &mut nt_b1), S_OK);
        assert_eq!(
            open_shared_name_b(
                dev_b,
                name_lower_w.as_ptr(),
                dxgi_shared_resource::READ | dxgi_shared_resource::WRITE,
                &IID_ID3D11Texture2D,
                &mut nt_b2,
            ),
            S_OK
        );

        // Clear green on dev_a -> visible on both nt_b1 (OpenSharedResource1) and nt_b2 (OpenSharedResourceByName).
        let mut nt_rtv_a: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_rtv_a(dev_a, nt_a, std::ptr::null(), &mut nt_rtv_a), S_OK);
        let green = [0.0f32, 1.0, 0.0, 1.0];
        clear_rtv_a(ctx_a, nt_rtv_a, &green);

        for opened in [nt_b1, nt_b2] {
            copy_res_b(ctx_b, stg_b, opened);
            assert_eq!(map_b(ctx_b, stg_b, 0, map_type::READ, 0, &mut mapped), S_OK);
            assert_eq!(std::ptr::read_unaligned(mapped.pData as *const u32), 0xFF00_FF00);
            unmap_b(ctx_b, stg_b, 0);
        }

        // Write via IDXGISurface::Map(WRITE|DISCARD) on nt_b2 -> visible on nt_a!
        let mut nt_surf_b2: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(nt_b2, &IID_IDXGISurface, &mut nt_surf_b2), S_OK);
        assert_eq!(surf_map_b(nt_surf_b2, &mut srect, dxgi_map::WRITE | dxgi_map::DISCARD), S_OK);
        std::ptr::write_unaligned(srect.pBits as *mut u32, 0xFFFF_00FF);
        assert_eq!(surf_unmap_b(nt_surf_b2), S_OK);
        copy_res_a(ctx_a, stg_a, nt_a);
        assert_eq!(map_a(ctx_a, stg_a, 0, map_type::READ, 0, &mut mapped), S_OK);
        assert_eq!(std::ptr::read_unaligned(mapped.pData as *const u32), 0xFFFF_00FF);
        unmap_a(ctx_a, stg_a, 0);

        // Closing the NT handle invalidates subsequent opens by handle and by name.
        assert!(crate::handles::close(h_nt));
        let mut closed_out: *mut c_void = std::ptr::null_mut();
        assert_eq!(open_shared1_b(dev_b, h_nt, &IID_ID3D11Texture2D, &mut closed_out), E_HANDLE);
        assert_eq!(
            open_shared_name_b(dev_b, name_w.as_ptr(), dxgi_shared_resource::READ, &IID_ID3D11Texture2D, &mut closed_out),
            E_HANDLE
        );

        // --- Part C: Shared ID3D11Buffer across devices ---
        let sbuf_desc = D3D11_BUFFER_DESC {
            ByteWidth: 32,
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::SHADER_RESOURCE,
            CPUAccessFlags: 0,
            MiscFlags: misc_flag::SHARED,
            StructureByteStride: 4,
        };
        let stg_bdesc = D3D11_BUFFER_DESC {
            Usage: usage::STAGING,
            BindFlags: 0,
            CPUAccessFlags: cpu_access::READ,
            MiscFlags: 0,
            ..sbuf_desc
        };
        let (mut sbuf_a, mut sbuf_stg_b) = (std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(create_buf_a(dev_a, &sbuf_desc, std::ptr::null(), &mut sbuf_a), S_OK);
        assert_eq!(create_buf_b(dev_b, &stg_bdesc, std::ptr::null(), &mut sbuf_stg_b), S_OK);
        let mut dxgi_buf: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_query(sbuf_a, &IID_IDXGIResource, &mut dxgi_buf), S_OK);
        let mut h_buf: HANDLE = std::ptr::null_mut();
        assert_eq!(get_shared_h(dxgi_buf, &mut h_buf), S_OK);
        let mut sbuf_b: *mut c_void = std::ptr::null_mut();
        assert_eq!(open_shared_b(dev_b, h_buf, &IID_ID3D11Buffer, &mut sbuf_b), S_OK);
        let words: [u32; 8] = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        update_sub_a(ctx_a, sbuf_a, 0, std::ptr::null(), words.as_ptr() as *const c_void, 32, 32);
        copy_res_b(ctx_b, sbuf_stg_b, sbuf_b);
        assert_eq!(map_b(ctx_b, sbuf_stg_b, 0, map_type::READ, 0, &mut mapped), S_OK);
        let got_words = std::slice::from_raw_parts(mapped.pData as *const u32, 8);
        assert_eq!(got_words, &words);
        unmap_b(ctx_b, sbuf_stg_b, 0);

        // --- Part D: Honest failure on unknown handles / names / null pointers ---
        let mut dummy: *mut c_void = std::ptr::null_mut();
        let bogus_h = 0x1234_5678usize as HANDLE;
        let missing_w = crate::strings::to_wide_z("Global\\DoesNotExist_SharedTex");
        assert_eq!(open_shared_b(dev_b, std::ptr::null_mut(), &IID_ID3D11Texture2D, &mut dummy), E_INVALIDARG);
        assert_eq!(open_shared_b(dev_b, bogus_h, &IID_ID3D11Texture2D, &mut dummy), E_HANDLE);
        assert_eq!(open_shared1_b(dev_b, std::ptr::null_mut(), &IID_ID3D11Texture2D, &mut dummy), E_INVALIDARG);
        assert_eq!(open_shared1_b(dev_b, bogus_h, &IID_ID3D11Texture2D, &mut dummy), E_HANDLE);
        assert_eq!(open_shared_name_b(dev_b, std::ptr::null(), dxgi_shared_resource::READ, &IID_ID3D11Texture2D, &mut dummy), E_INVALIDARG);
        assert_eq!(open_shared_name_b(dev_b, missing_w.as_ptr(), dxgi_shared_resource::READ, &IID_ID3D11Texture2D, &mut dummy), E_HANDLE);

        com_release(sbuf_b);
        com_release(dxgi_buf);
        com_release(sbuf_stg_b);
        com_release(sbuf_a);
        com_release(nt_surf_b2);
        com_release(nt_rtv_a);
        com_release(nt_b2);
        com_release(nt_b1);
        com_release(dxgi_nt);
        com_release(nt_a);
        com_release(rtv_b);
        com_release(rtv_a);
        com_release(surf_b);
        com_release(tex_b);
        com_release(dxgi_unshared);
        com_release(dxgi_a);
        com_release(unshared_a);
        com_release(tex_a);
        com_release(stg_b);
        com_release(stg_a);
        com_release(ctx_b);
        com_release(dev_b);
        com_release(ctx_a);
        com_release(dev_a);
    }
}

#[test]
fn timestamp_query_pool_supports_256_concurrent_queries_on_metal() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                std::ptr::null_mut(),
                &mut ctx,
            ),
            S_OK
        );

        let create_query: unsafe extern "win64" fn(*mut c_void, *const D3D11_QUERY_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 24));
        let end_query: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 28));
        let get_data: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, u32, u32) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 29));
        let flush_fn: unsafe extern "win64" fn(*mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 111));

        // Exceed Apple AGX's ~32 MTL4CounterHeap limit by 8x on a single device.
        const N: usize = 256;
        let qd = D3D11_QUERY_DESC { Query: query_kind::TIMESTAMP, MiscFlags: 0 };
        let mut queries = Vec::with_capacity(N);
        for _ in 0..N {
            let mut q: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_query(dev, &qd, &mut q), S_OK);
            assert!(!q.is_null());
            end_query(ctx, q);
            queries.push(q);
        }
        flush_fn(ctx);

        for (i, &q) in queries.iter().enumerate() {
            let mut ts = 0u64;
            assert_eq!(get_data(ctx, q, &mut ts as *mut u64 as *mut c_void, 8, 0), S_OK, "query {i}");
            assert!(ts > 0, "query {i} timestamp must be nonzero");
        }

        // Releasing all queries returns slots to the pool; creating a fresh batch reuses them cleanly.
        for q in queries {
            com_release(q);
        }
        let mut reused: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_query(dev, &qd, &mut reused), S_OK);
        end_query(ctx, reused);
        flush_fn(ctx);
        let mut ts = 0u64;
        assert_eq!(get_data(ctx, reused, &mut ts as *mut u64 as *mut c_void, 8, 0), S_OK);
        assert!(ts > 0);

        com_release(reused);
        com_release(ctx);
        com_release(dev);
    }
}

#[test]
fn sm5_geometry_shader_emulation_draw_on_metal() {
    if d3d12_metal::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
        return;
    }
    let words = |w: &[u32]| -> Vec<u8> { w.iter().flat_map(|v| v.to_le_bytes()).collect() };
    const EMPTY_SIG: [u8; 8] = [0, 0, 0, 0, 8, 0, 0, 0];

    // vs_5_0: dcl_input v0.xyzw; dcl_output_siv o0.xyzw, position; mov o0.xyzw, v0.xyzw; ret
    #[rustfmt::skip]
    let vs_shex = words(&[
        0x0001_0050, 15,
        0x0300_005f, 0x0010_10f2, 0,
        0x0400_0067, 0x0010_20f2, 0, 1,
        0x0500_0036, 0x0010_20f2, 0, 0x0010_1e46, 0,
        0x0100_003e,
    ]);
    let mut vs_isgn = words(&[1, 8, 32, 0, 0, 3, 0]);
    vs_isgn.extend_from_slice(&[0x0f, 0x0f, 0, 0]);
    vs_isgn.extend_from_slice(b"POSITION\0\0\0\0");
    let mut vs_osgn = words(&[1, 8, 32, 0, 1, 3, 0]);
    vs_osgn.extend_from_slice(&[0x0f, 0x00, 0, 0]);
    vs_osgn.extend_from_slice(b"SV_Position\0");
    let vs_dxbc = dxil_metal::dxbc::build_container(&[
        (*b"ISGN", &vs_isgn),
        (*b"OSGN", &vs_osgn),
        (*b"SHEX", &vs_shex),
    ]);

    // gs_5_0: point -> trianglestrip (maxvertexcount 3) emitting a fullscreen triangle!
    // dcl_inputprimitive point; dcl_outputtopology trianglestrip; dcl_maxout 3;
    // dcl_output_siv o0.xyzw, position;
    // mov o0, l(-1,-1,0,1); emit;
    // mov o0, l(-1, 3,0,1); emit;
    // mov o0, l( 3,-1,0,1); emit;
    // cut; ret
    #[rustfmt::skip]
    let gs_shex = words(&[
        0x0002_0050, 39,
        0x0100_085d, // dcl_inputprimitive point
        0x0100_285c, // dcl_outputtopology trianglestrip
        0x0200_005e, 3, // dcl_maxout 3
        0x0400_0067, 0x0010_20f2, 0, 1, // dcl_output_siv o0.xyzw, position
        0x0800_0036, 0x0010_20f2, 0, 0x0000_4002, 0xbf80_0000, 0xbf80_0000, 0, 0x3f80_0000,
        0x0100_0013, // emit
        0x0800_0036, 0x0010_20f2, 0, 0x0000_4002, 0xbf80_0000, 0x4040_0000, 0, 0x3f80_0000,
        0x0100_0013, // emit
        0x0800_0036, 0x0010_20f2, 0, 0x0000_4002, 0x4040_0000, 0xbf80_0000, 0, 0x3f80_0000,
        0x0100_0013, // emit
        0x0100_0014, // cut
        0x0100_003e, // ret
    ]);
    let gs_dxbc = dxil_metal::dxbc::build_container(&[
        (*b"ISGN", &EMPTY_SIG),
        (*b"OSGN", &vs_osgn),
        (*b"SHEX", &gs_shex),
    ]);

    // ps_5_0: output solid cyan (0.0, 1.0, 1.0, 1.0)
    #[rustfmt::skip]
    let ps_shex = words(&[
        0x0000_0050, 14,
        0x0300_0065, 0x0010_20f2, 0,
        0x0800_0036, 0x0010_20f2, 0, 0x0000_4002, 0x0000_0000, 0x3f80_0000, 0x3f80_0000, 0x3f80_0000,
        0x0100_003e,
    ]);
    let mut ps_osgn = words(&[1, 8, 32, 0, 0, 3, 0]);
    ps_osgn.extend_from_slice(&[0x0f, 0x00, 0, 0]);
    ps_osgn.extend_from_slice(b"SV_Target\0\0\0");
    let ps_dxbc = dxil_metal::dxbc::build_container(&[
        (*b"ISGN", &EMPTY_SIG),
        (*b"OSGN", &ps_osgn),
        (*b"SHEX", &ps_shex),
    ]);

    unsafe {
        let mut dev: *mut c_void = std::ptr::null_mut();
        let mut ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D3D11CreateDevice(
                std::ptr::null_mut(),
                driver_type::HARDWARE,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                D3D11_SDK_VERSION,
                &mut dev,
                std::ptr::null_mut(),
                &mut ctx,
            ),
            S_OK
        );

        let create_buf: unsafe extern "win64" fn(*mut c_void, *const D3D11_BUFFER_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 3));
        let create_tex2d: unsafe extern "win64" fn(*mut c_void, *const D3D11_TEXTURE2D_DESC, *const D3D11_SUBRESOURCE_DATA, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 5));
        let create_rtv_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const D3D11_RENDER_TARGET_VIEW_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 9));
        let create_il: unsafe extern "win64" fn(*mut c_void, *const D3D11_INPUT_ELEMENT_DESC, u32, *const c_void, usize, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 11));
        let create_vs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 12));
        let create_gs: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 13));
        let create_ps: unsafe extern "win64" fn(*mut c_void, *const c_void, usize, *mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 15));
        let create_rs: unsafe extern "win64" fn(*mut c_void, *const D3D11_RASTERIZER_DESC, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_slot(dev, 22));

        let ps_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 9));
        let vs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 11));
        let draw_fn: unsafe extern "win64" fn(*mut c_void, u32, u32) =
            std::mem::transmute(vtbl_slot(ctx, 13));
        let map_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT =
            std::mem::transmute(vtbl_slot(ctx, 14));
        let unmap_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 15));
        let ia_set_il: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 17));
        let ia_set_vbs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const *mut c_void, *const u32, *const u32) =
            std::mem::transmute(vtbl_slot(ctx, 18));
        let gs_set: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const *mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 23));
        let ia_set_topo: unsafe extern "win64" fn(*mut c_void, u32) =
            std::mem::transmute(vtbl_slot(ctx, 24));
        let om_set_rts: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 33));
        let rs_set_st: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 43));
        let rs_set_vp: unsafe extern "win64" fn(*mut c_void, u32, *const D3D11_VIEWPORT) =
            std::mem::transmute(vtbl_slot(ctx, 44));
        let copy_res: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
            std::mem::transmute(vtbl_slot(ctx, 47));
        let clear_rtv: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]) =
            std::mem::transmute(vtbl_slot(ctx, 50));

        // Single point vertex; the geometry shader expands it to a fullscreen triangle!
        let pt: [[f32; 4]; 1] = [[0.0, 0.0, 0.0, 1.0]];
        let vb_desc = D3D11_BUFFER_DESC {
            ByteWidth: 16,
            Usage: usage::IMMUTABLE,
            BindFlags: bind_flag::VERTEX_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let vb_init = D3D11_SUBRESOURCE_DATA { pSysMem: pt.as_ptr() as *const c_void, SysMemPitch: 16, SysMemSlicePitch: 16 };
        let td = D3D11_TEXTURE2D_DESC {
            Width: 8,
            Height: 8,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: usage::DEFAULT,
            BindFlags: bind_flag::RENDER_TARGET,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let stg_td = D3D11_TEXTURE2D_DESC { Usage: usage::STAGING, BindFlags: 0, CPUAccessFlags: cpu_access::READ, ..td };

        let (mut vb, mut rt, mut stg, mut rtv) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
        let (mut vs, mut gs, mut ps, mut il, mut rs) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(create_buf(dev, &vb_desc, &vb_init, &mut vb), S_OK);
        assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut rt), S_OK);
        assert_eq!(create_tex2d(dev, &stg_td, std::ptr::null(), &mut stg), S_OK);
        assert_eq!(create_rtv_fn(dev, rt, std::ptr::null(), &mut rtv), S_OK);
        assert_eq!(create_vs(dev, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), std::ptr::null_mut(), &mut vs), S_OK);
        assert_eq!(create_gs(dev, gs_dxbc.as_ptr() as *const c_void, gs_dxbc.len(), std::ptr::null_mut(), &mut gs), S_OK);
        assert_eq!(create_ps(dev, ps_dxbc.as_ptr() as *const c_void, ps_dxbc.len(), std::ptr::null_mut(), &mut ps), S_OK);
        let ie = D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr(),
            SemanticIndex: 0,
            Format: 2,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: 0,
            InstanceDataStepRate: 0,
        };
        assert_eq!(create_il(dev, &ie, 1, vs_dxbc.as_ptr() as *const c_void, vs_dxbc.len(), &mut il), S_OK);
        let mut rdesc = D3D11_RASTERIZER_DESC::default_d3d11();
        rdesc.CullMode = 1; // CULL_NONE
        assert_eq!(create_rs(dev, &rdesc, &mut rs), S_OK);

        let black = [0.0f32; 4];
        clear_rtv(ctx, rtv, &black);
        om_set_rts(ctx, 1, &rtv, std::ptr::null_mut());
        let vp = D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: 8.0, Height: 8.0, MinDepth: 0.0, MaxDepth: 1.0 };
        rs_set_vp(ctx, 1, &vp);
        rs_set_st(ctx, rs);
        ia_set_il(ctx, il);
        let stride = 16u32;
        let offset = 0u32;
        ia_set_vbs(ctx, 0, 1, &vb, &stride, &offset);
        ia_set_topo(ctx, topology::POINTLIST);
        vs_set(ctx, vs, std::ptr::null(), 0);
        gs_set(ctx, gs, std::ptr::null(), 0);
        ps_set(ctx, ps, std::ptr::null(), 0);
        draw_fn(ctx, 1, 0);

        copy_res(ctx, stg, rt);
        let mut mapped = D3D11_MAPPED_SUBRESOURCE { pData: std::ptr::null_mut(), RowPitch: 0, DepthPitch: 0 };
        assert_eq!(map_fn(ctx, stg, 0, map_type::READ, 0, &mut mapped), S_OK);
        // Solid cyan (0, 255, 255, 255) = 0xFFFF_FF00 in RGBA8_UNORM.
        let px0 = std::ptr::read_unaligned(mapped.pData as *const u32);
        assert_eq!(px0, 0xFFFF_FF00, "GS-emitted fullscreen triangle must render cyan, got {px0:#010x}");
        unmap_fn(ctx, stg, 0);

        com_release(rs);
        com_release(il);
        com_release(ps);
        com_release(gs);
        com_release(vs);
        com_release(rtv);
        com_release(stg);
        com_release(rt);
        com_release(vb);
        com_release(ctx);
        com_release(dev);
    }
}
