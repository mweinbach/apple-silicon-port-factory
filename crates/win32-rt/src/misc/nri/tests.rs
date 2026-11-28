use super::device::{
    core_create_graphics_pipeline, core_destroy_pipeline, CpuDescriptorAllocator,
};
use super::*;
use crate::d3d12::com::{com_release, Guid};
use crate::d3d12::pso::StreamDesc;
use crate::d3d12::vtables::{IID_ID3D12Device, IID_ID3D12Device2, IID_ID3D12PipelineState};
use crate::types::HRESULT;
use std::collections::HashMap;
use std::ffi::c_void;
use std::ptr;
use std::sync::Mutex;

unsafe fn make_test_d3d12_device() -> Option<*mut c_void> {
    let mut d3d_dev: *mut c_void = ptr::null_mut();
    let hr = crate::d3d12::device::create_device(
        ptr::null_mut(),
        d3d12_metal::caps::FL_12_1,
        &IID_ID3D12Device,
        &mut d3d_dev,
    );
    if hr >= 0 && !d3d_dev.is_null() {
        Some(d3d_dev)
    } else {
        None
    }
}

unsafe fn make_nri_device(
    d3d_dev: *mut c_void,
) -> (*mut NriDevice, [usize; 106], [usize; 5], [usize; 5]) {
    let desc = NriDeviceCreationD3D12Desc {
        d3d12_device: d3d_dev,
        d3d12_graphics_queue: ptr::null_mut(),
        d3d12_compute_queue: ptr::null_mut(),
        d3d12_copy_queue: ptr::null_mut(),
        callback_interface: [0; 3],
        memory_allocator_interface: [0; 4],
        enable_nri_validation: 0,
        _pad: [0; 7],
    };
    let mut dev: *mut NriDevice = ptr::null_mut();
    assert_eq!(
        nriCreateDeviceFromD3D12Device(&desc as *const _ as *const u8, &mut dev),
        NRI_RESULT_SUCCESS
    );
    assert!(!dev.is_null());

    let mut core = [0usize; 106];
    assert_eq!(
        nriGetInterface(
            dev,
            b"nri::CoreInterface\0".as_ptr(),
            0x350,
            core.as_mut_ptr() as *mut u8,
        ),
        NRI_RESULT_SUCCESS
    );
    assert!(core.iter().all(|&p| p != 0));

    let mut helper = [0usize; 5];
    assert_eq!(
        nriGetInterface(
            dev,
            b"nri::HelperInterface\0".as_ptr(),
            0x28,
            helper.as_mut_ptr() as *mut u8,
        ),
        NRI_RESULT_SUCCESS
    );
    assert!(helper.iter().all(|&p| p != 0));

    let mut wrapper = [0usize; 5];
    assert_eq!(
        nriGetInterface(
            dev,
            b"nri::WrapperD3D12Interface\0".as_ptr(),
            0x28,
            wrapper.as_mut_ptr() as *mut u8,
        ),
        NRI_RESULT_SUCCESS
    );
    assert!(wrapper.iter().all(|&p| p != 0));

    (dev, core, helper, wrapper)
}

#[test]
fn nri_rejects_null_and_unsupported_interfaces() {
    unsafe {
        let mut dev: *mut NriDevice = ptr::null_mut();
        assert_eq!(
            nriCreateDeviceFromD3D12Device(ptr::null(), &mut dev),
            NRI_RESULT_INVALID_ARGUMENT
        );
        let zero_desc = [0u8; 0x60];
        assert_eq!(
            nriCreateDeviceFromD3D12Device(zero_desc.as_ptr(), &mut dev),
            NRI_RESULT_INVALID_ARGUMENT
        );

        let Some(d3d_dev) = make_test_d3d12_device() else {
            return;
        };
        let (dev, _core, _helper, wrapper) = make_nri_device(d3d_dev);

        let mut unknown_iface = [0u8; 0x40];
        assert_eq!(
            nriGetInterface(
                dev,
                b"nri::SwapChainInterface\0".as_ptr(),
                0x40,
                unknown_iface.as_mut_ptr(),
            ),
            NRI_RESULT_UNSUPPORTED
        );

        // WrapperD3D12 rejects null descriptor / null resource.
        let create_tex_d3d: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriTextureD3D12Desc,
            *mut *mut NriTexture,
        ) -> u32 = std::mem::transmute(wrapper[2]);
        let mut tex: *mut NriTexture = ptr::null_mut();
        assert_eq!(
            create_tex_d3d(dev, ptr::null(), &mut tex),
            NRI_RESULT_INVALID_ARGUMENT
        );
        assert!(tex.is_null());

        nriDestroyDevice(dev);
        com_release(d3d_dev);
    }
}

#[test]
fn nri_real_device_desc_and_format_support() {
    unsafe {
        let Some(d3d_dev) = make_test_d3d12_device() else {
            return;
        };
        let (dev, core, _helper, _wrapper) = make_nri_device(d3d_dev);

        let get_desc: unsafe extern "win64" fn(*const NriDevice) -> *const NriDeviceDesc =
            std::mem::transmute(core[0]);
        let desc = get_desc(dev);
        assert!(!desc.is_null());
        assert_eq!((*desc).graphics_api, NRI_GRAPHICS_API_D3D12);
        assert_eq!((*desc).nri_version_major, 1);
        assert_eq!((*desc).nri_version_minor, 99);
        assert_eq!((*desc).constant_buffer_offset_alignment, 256);
        assert!((*desc).adapter_desc.video_memory_size > 0);
        assert!((*desc).timestamp_frequency_hz > 0);

        let get_native_dev: unsafe extern "win64" fn(*const NriDevice) -> *mut c_void =
            std::mem::transmute(core[101]);
        assert_eq!(get_native_dev(dev), d3d_dev);

        let get_fmt: unsafe extern "win64" fn(*const NriDevice, u8) -> u8 =
            std::mem::transmute(core[1]);
        // RGBA8_UNORM (11): texture + storage + color attachment.
        let rgba8 = get_fmt(dev, 11);
        assert_ne!(rgba8 & (1 << 0), 0);
        assert_ne!(rgba8 & (1 << 4), 0);
        // D32_SFLOAT (63): depth-stencil attachment.
        let d32 = get_fmt(dev, 63);
        assert_ne!(d32 & (1 << 5), 0);
        // UNKNOWN (0): unsupported.
        assert_eq!(get_fmt(dev, 0), 0);

        nriDestroyDevice(dev);
        com_release(d3d_dev);
    }
}

#[test]
fn nri_buffers_map_independently_without_aliasing() {
    unsafe {
        let Some(d3d_dev) = make_test_d3d12_device() else {
            return;
        };
        let (dev, core, helper, _wrapper) = make_nri_device(d3d_dev);

        let create_buf: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriBufferDesc,
            *mut *mut NriBuffer,
        ) -> u32 = std::mem::transmute(core[9]);
        let destroy_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[25]);
        let free_mem: unsafe extern "win64" fn(*mut NriMemory) = std::mem::transmute(core[36]);
        let map_buf: unsafe extern "win64" fn(*mut NriBuffer, u64, u64) -> *mut u8 =
            std::mem::transmute(core[84]);
        let unmap_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[85]);
        let get_buf_native: unsafe extern "win64" fn(*const NriBuffer, u32) -> u64 =
            std::mem::transmute(core[103]);
        let calc_alloc: unsafe extern "win64" fn(*mut NriDevice, *const NriResourceGroupDesc) -> u32 =
            std::mem::transmute(helper[0]);
        let alloc_bind: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriResourceGroupDesc,
            *mut *mut NriMemory,
        ) -> u32 = std::mem::transmute(helper[1]);

        let bdesc = NriBufferDesc {
            size: 1024,
            structure_stride: 0,
            usage_mask: 1 << 4, // CONSTANT_BUFFER
            _pad0: 0,
            physical_device_mask: 0,
            _pad1: 0,
        };
        let mut b1: *mut NriBuffer = ptr::null_mut();
        let mut b2: *mut NriBuffer = ptr::null_mut();
        assert_eq!(create_buf(dev, &bdesc, &mut b1), NRI_RESULT_SUCCESS);
        assert_eq!(create_buf(dev, &bdesc, &mut b2), NRI_RESULT_SUCCESS);

        let bufs = [b1, b2];
        let rg = NriResourceGroupDesc {
            memory_location: 1, // HOST_UPLOAD
            _pad0: [0; 7],
            textures: ptr::null(),
            texture_num: 0,
            _pad1: 0,
            buffers: bufs.as_ptr(),
            buffer_num: 2,
            _pad2: 0,
        };
        let n_allocs = calc_alloc(dev, &rg);
        assert_eq!(n_allocs, 1);
        let mut mems = vec![ptr::null_mut(); n_allocs as usize];
        assert_eq!(
            alloc_bind(dev, &rg, mems.as_mut_ptr()),
            NRI_RESULT_SUCCESS
        );
        assert_ne!(get_buf_native(b1, 0), 0);
        assert_ne!(get_buf_native(b2, 0), 0);
        assert_ne!(get_buf_native(b1, 0), get_buf_native(b2, 0));

        let p1 = map_buf(b1, 0, 1024);
        let p2 = map_buf(b2, 0, 1024);
        assert!(!p1.is_null());
        assert!(!p2.is_null());
        assert_ne!(p1, p2, "two mapped NRI buffers must NOT alias");

        ptr::write_bytes(p1, 0x11, 1024);
        ptr::write_bytes(p2, 0x22, 1024);
        assert_eq!(*p1, 0x11);
        assert_eq!(*p1.add(1023), 0x11);
        assert_eq!(*p2, 0x22);
        assert_eq!(*p2.add(1023), 0x22);

        unmap_buf(b1);
        unmap_buf(b2);
        destroy_buf(b1);
        destroy_buf(b2);
        for m in mems {
            free_mem(m);
        }
        nriDestroyDevice(dev);
        com_release(d3d_dev);
    }
}

#[test]
fn nri_fence_signal_and_gpu_copy_roundtrip() {
    unsafe {
        let Some(d3d_dev) = make_test_d3d12_device() else {
            return;
        };
        let (dev, core, helper, _wrapper) = make_nri_device(d3d_dev);

        let get_queue: unsafe extern "win64" fn(
            *mut NriDevice,
            u8,
            *mut *mut NriCommandQueue,
        ) -> u32 = std::mem::transmute(core[5]);
        let create_alloc: unsafe extern "win64" fn(
            *const NriCommandQueue,
            u32,
            *mut *mut NriCommandAllocator,
        ) -> u32 = std::mem::transmute(core[6]);
        let create_cmd: unsafe extern "win64" fn(
            *mut NriCommandAllocator,
            *mut *mut NriCommandBuffer,
        ) -> u32 = std::mem::transmute(core[7]);
        let create_buf: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriBufferDesc,
            *mut *mut NriBuffer,
        ) -> u32 = std::mem::transmute(core[9]);
        let create_fence: unsafe extern "win64" fn(
            *mut NriDevice,
            u64,
            *mut *mut NriFence,
        ) -> u32 = std::mem::transmute(core[21]);
        let destroy_alloc: unsafe extern "win64" fn(*mut NriCommandAllocator) =
            std::mem::transmute(core[22]);
        let destroy_cmd: unsafe extern "win64" fn(*mut NriCommandBuffer) =
            std::mem::transmute(core[23]);
        let destroy_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[25]);
        let destroy_fence: unsafe extern "win64" fn(*mut NriFence) = std::mem::transmute(core[32]);
        let free_mem: unsafe extern "win64" fn(*mut NriMemory) = std::mem::transmute(core[36]);
        let begin_cmd: unsafe extern "win64" fn(
            *mut NriCommandBuffer,
            *const NriDescriptorPool,
            u32,
        ) -> u32 = std::mem::transmute(core[37]);
        let end_cmd: unsafe extern "win64" fn(*mut NriCommandBuffer) -> u32 =
            std::mem::transmute(core[38]);
        let copy_buf: unsafe extern "win64" fn(
            *mut NriCommandBuffer,
            *mut NriBuffer,
            u32,
            u64,
            *const NriBuffer,
            u32,
            u64,
            u64,
        ) = std::mem::transmute(core[67]);
        let get_fence_val: unsafe extern "win64" fn(*const NriFence) -> u64 =
            std::mem::transmute(core[73]);
        let queue_signal: unsafe extern "win64" fn(*mut NriCommandQueue, *mut NriFence, u64) =
            std::mem::transmute(core[74]);
        let wait_fence: unsafe extern "win64" fn(*mut NriFence, u64) =
            std::mem::transmute(core[76]);
        let queue_submit: unsafe extern "win64" fn(*mut NriCommandQueue, *const NriQueueSubmitDesc) =
            std::mem::transmute(core[77]);
        let map_buf: unsafe extern "win64" fn(*mut NriBuffer, u64, u64) -> *mut u8 =
            std::mem::transmute(core[84]);
        let unmap_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[85]);
        let alloc_bind: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriResourceGroupDesc,
            *mut *mut NriMemory,
        ) -> u32 = std::mem::transmute(helper[1]);
        let wait_idle: unsafe extern "win64" fn(*mut NriCommandQueue) -> u32 =
            std::mem::transmute(helper[4]);

        let mut queue: *mut NriCommandQueue = ptr::null_mut();
        assert_eq!(get_queue(dev, 0, &mut queue), NRI_RESULT_SUCCESS);

        // Fence initial value + signal/wait tracking.
        let mut fence: *mut NriFence = ptr::null_mut();
        assert_eq!(create_fence(dev, 7, &mut fence), NRI_RESULT_SUCCESS);
        assert_eq!(get_fence_val(fence), 7);

        // Create upload src + readback dst buffers and perform real GPU CopyBuffer.
        let bdesc = NriBufferDesc {
            size: 256,
            structure_stride: 0,
            usage_mask: 0,
            _pad0: 0,
            physical_device_mask: 0,
            _pad1: 0,
        };
        let mut src: *mut NriBuffer = ptr::null_mut();
        let mut dst: *mut NriBuffer = ptr::null_mut();
        assert_eq!(create_buf(dev, &bdesc, &mut src), NRI_RESULT_SUCCESS);
        assert_eq!(create_buf(dev, &bdesc, &mut dst), NRI_RESULT_SUCCESS);

        let mut src_mem: *mut NriMemory = ptr::null_mut();
        let mut dst_mem: *mut NriMemory = ptr::null_mut();
        let rg_src = NriResourceGroupDesc {
            memory_location: 1, // HOST_UPLOAD
            _pad0: [0; 7],
            textures: ptr::null(),
            texture_num: 0,
            _pad1: 0,
            buffers: &src,
            buffer_num: 1,
            _pad2: 0,
        };
        let rg_dst = NriResourceGroupDesc {
            memory_location: 2, // HOST_READBACK
            _pad0: [0; 7],
            textures: ptr::null(),
            texture_num: 0,
            _pad1: 0,
            buffers: &dst,
            buffer_num: 1,
            _pad2: 0,
        };
        assert_eq!(alloc_bind(dev, &rg_src, &mut src_mem), NRI_RESULT_SUCCESS);
        assert_eq!(alloc_bind(dev, &rg_dst, &mut dst_mem), NRI_RESULT_SUCCESS);

        let sp = map_buf(src, 0, 256);
        assert!(!sp.is_null());
        for i in 0..256usize {
            *sp.add(i) = (i as u8).wrapping_mul(3).wrapping_add(1);
        }
        unmap_buf(src);

        let mut alloc: *mut NriCommandAllocator = ptr::null_mut();
        let mut cmd: *mut NriCommandBuffer = ptr::null_mut();
        assert_eq!(create_alloc(queue, 0, &mut alloc), NRI_RESULT_SUCCESS);
        assert_eq!(create_cmd(alloc, &mut cmd), NRI_RESULT_SUCCESS);
        assert_eq!(begin_cmd(cmd, ptr::null(), 0), NRI_RESULT_SUCCESS);
        copy_buf(cmd, dst, 0, 0, src, 0, 0, 256);
        assert_eq!(end_cmd(cmd), NRI_RESULT_SUCCESS);

        let cb_ptr: *const NriCommandBuffer = cmd;
        let submit = NriQueueSubmitDesc {
            command_buffers: &cb_ptr,
            command_buffer_num: 1,
            physical_device_index: 0,
        };
        queue_submit(queue, &submit);
        queue_signal(queue, fence, 42);
        wait_fence(fence, 42);
        assert!(get_fence_val(fence) >= 42);
        assert_eq!(wait_idle(queue), NRI_RESULT_SUCCESS);

        let dp = map_buf(dst, 0, 256);
        assert!(!dp.is_null());
        for i in 0..256usize {
            assert_eq!(*dp.add(i), (i as u8).wrapping_mul(3).wrapping_add(1));
        }
        unmap_buf(dst);

        destroy_cmd(cmd);
        destroy_alloc(alloc);
        destroy_fence(fence);
        destroy_buf(src);
        destroy_buf(dst);
        free_mem(src_mem);
        free_mem(dst_mem);
        nriDestroyDevice(dev);
        com_release(d3d_dev);
    }
}

#[test]
fn nri_texture_views_descriptors_and_pipeline_layout() {
    unsafe {
        let Some(d3d_dev) = make_test_d3d12_device() else {
            return;
        };
        let (dev, core, helper, _wrapper) = make_nri_device(d3d_dev);

        let create_tex: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriTextureDesc,
            *mut *mut NriTexture,
        ) -> u32 = std::mem::transmute(core[10]);
        let create_tex_view: unsafe extern "win64" fn(
            *const NriTextureViewDesc,
            *mut *mut NriDescriptor,
        ) -> u32 = std::mem::transmute(core[13]);
        let create_sampler: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriSamplerDesc,
            *mut *mut NriDescriptor,
        ) -> u32 = std::mem::transmute(core[15]);
        let create_pool: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriDescriptorPoolDesc,
            *mut *mut NriDescriptorPool,
        ) -> u32 = std::mem::transmute(core[8]);
        let create_layout: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriPipelineLayoutDesc,
            *mut *mut NriPipelineLayout,
        ) -> u32 = std::mem::transmute(core[16]);
        let alloc_sets: unsafe extern "win64" fn(
            *mut NriDescriptorPool,
            *const NriPipelineLayout,
            u32,
            *mut *mut NriDescriptorSet,
            u32,
            u32,
            u32,
        ) -> u32 = std::mem::transmute(core[81]);
        let update_ranges: unsafe extern "win64" fn(
            *mut NriDescriptorSet,
            u32,
            u32,
            u32,
            *const NriDescriptorRangeUpdateDesc,
        ) = std::mem::transmute(core[78]);
        let destroy_pool: unsafe extern "win64" fn(*mut NriDescriptorPool) =
            std::mem::transmute(core[24]);
        let destroy_tex: unsafe extern "win64" fn(*mut NriTexture) = std::mem::transmute(core[26]);
        let destroy_desc: unsafe extern "win64" fn(*mut NriDescriptor) =
            std::mem::transmute(core[27]);
        let destroy_layout: unsafe extern "win64" fn(*mut NriPipelineLayout) =
            std::mem::transmute(core[28]);
        let free_mem: unsafe extern "win64" fn(*mut NriMemory) = std::mem::transmute(core[36]);
        let alloc_bind: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriResourceGroupDesc,
            *mut *mut NriMemory,
        ) -> u32 = std::mem::transmute(helper[1]);

        let tdesc = NriTextureDesc {
            texture_type: 1, // TEXTURE_2D
            _pad0: 0,
            usage_mask: (1 << 0) | (1 << 1), // SRV | UAV
            format: 11,                      // RGBA8_UNORM
            _pad1: 0,
            size: [64, 64, 1],
            mip_num: 1,
            array_size: 1,
            sample_num: 1,
            _pad2: [0; 3],
            physical_device_mask: 0,
        };
        let mut tex: *mut NriTexture = ptr::null_mut();
        assert_eq!(create_tex(dev, &tdesc, &mut tex), NRI_RESULT_SUCCESS);
        let mut tmem: *mut NriMemory = ptr::null_mut();
        let rg = NriResourceGroupDesc {
            memory_location: 0, // DEVICE
            _pad0: [0; 7],
            textures: &tex,
            texture_num: 1,
            _pad1: 0,
            buffers: ptr::null(),
            buffer_num: 0,
            _pad2: 0,
        };
        assert_eq!(alloc_bind(dev, &rg, &mut tmem), NRI_RESULT_SUCCESS);

        let vdesc = NriTextureViewDesc {
            texture: tex,
            view_type: 0, // SHADER_RESOURCE_2D
            format: 11,
            mip_offset: 0,
            mip_num: 1,
            array_or_slice_offset: 0,
            array_or_slice_size: 1,
            _pad0: 0,
            physical_device_mask: 0,
            flags: 0,
            _pad1: [0; 7],
        };
        let mut srv: *mut NriDescriptor = ptr::null_mut();
        assert_eq!(create_tex_view(&vdesc, &mut srv), NRI_RESULT_SUCCESS);
        assert!(!srv.is_null());

        let sdesc = NriSamplerDesc {
            magnification: 1,
            minification: 1,
            mip: 1,
            filter_ext: 0,
            anisotropy: 1,
            mip_bias: 0.0,
            mip_min: 0.0,
            mip_max: 16.0,
            address_u: 2,
            address_v: 2,
            address_w: 2,
            compare_func: 0,
            border_color: 0,
            unnormalized_coordinates: 0,
            _pad: [0; 2],
        };
        let mut sampler: *mut NriDescriptor = ptr::null_mut();
        assert_eq!(
            create_sampler(dev, &sdesc, &mut sampler),
            NRI_RESULT_SUCCESS
        );
        assert!(!sampler.is_null());

        let ranges = [
            NriDescriptorRangeDesc {
                base_register_index: 0,
                descriptor_num: 1,
                descriptor_type: 2, // TEXTURE
                visibility: 6,      // COMPUTE
                is_descriptor_num_variable: 0,
                is_array: 0,
            },
            NriDescriptorRangeDesc {
                base_register_index: 0,
                descriptor_num: 1,
                descriptor_type: 0, // SAMPLER
                visibility: 6,      // COMPUTE
                is_descriptor_num_variable: 0,
                is_array: 0,
            },
        ];
        let set_desc = NriDescriptorSetDesc {
            register_space: 0,
            _pad0: 0,
            ranges: ranges.as_ptr(),
            range_num: 2,
            _pad1: 0,
            dynamic_constant_buffers: ptr::null(),
            dynamic_constant_buffer_num: 0,
            binding_mask: 0,
            _pad2: [0; 3],
        };
        let pl_desc = NriPipelineLayoutDesc {
            descriptor_sets: &set_desc,
            push_constants: ptr::null(),
            descriptor_set_num: 1,
            push_constant_num: 0,
            stage_mask: 1 << 6, // COMPUTE
            ignore_global_spirv_offsets: 1,
            _pad: [0; 5],
        };
        let mut layout: *mut NriPipelineLayout = ptr::null_mut();
        assert_eq!(
            create_layout(dev, &pl_desc, &mut layout),
            NRI_RESULT_SUCCESS
        );
        assert!(!layout.is_null());

        let pool_desc = NriDescriptorPoolDesc {
            descriptor_set_max_num: 2,
            sampler_max_num: 4,
            texture_max_num: 4,
            ..Default::default()
        };
        let mut pool: *mut NriDescriptorPool = ptr::null_mut();
        assert_eq!(create_pool(dev, &pool_desc, &mut pool), NRI_RESULT_SUCCESS);

        let mut set: *mut NriDescriptorSet = ptr::null_mut();
        assert_eq!(
            alloc_sets(pool, layout, 0, &mut set, 1, 0, 0),
            NRI_RESULT_SUCCESS
        );
        assert!(!set.is_null());

        let srv_arr: [*const NriDescriptor; 1] = [srv];
        let samp_arr: [*const NriDescriptor; 1] = [sampler];
        let updates = [
            NriDescriptorRangeUpdateDesc {
                descriptors: srv_arr.as_ptr(),
                descriptor_num: 1,
                offset_in_range: 0,
            },
            NriDescriptorRangeUpdateDesc {
                descriptors: samp_arr.as_ptr(),
                descriptor_num: 1,
                offset_in_range: 0,
            },
        ];
        update_ranges(set, 0, 0, 2, updates.as_ptr());

        destroy_pool(pool);
        destroy_layout(layout);
        destroy_desc(srv);
        destroy_desc(sampler);
        destroy_tex(tex);
        free_mem(tmem);
        nriDestroyDevice(dev);
        com_release(d3d_dev);
    }
}

#[test]
fn nri_mesh_and_task_pipeline_stream_encoding_and_device2_dispatch() {
    // Minimal mock COM objects verifying exact D3D12_PIPELINE_STATE_STREAM_DESC
    // encoding and ID3D12Device2::CreatePipelineState (vtable slot 47) routing.
    #[repr(C)]
    struct MockPso {
        vtbl: *const [usize; 3],
        released: std::sync::atomic::AtomicBool,
    }

    unsafe extern "win64" fn mock_pso_qi(
        _this: *mut c_void,
        _riid: *const Guid,
        out: *mut *mut c_void,
    ) -> HRESULT {
        if !out.is_null() {
            *out = ptr::null_mut();
        }
        0x8000_4002u32 as i32 // E_NOINTERFACE
    }
    unsafe extern "win64" fn mock_pso_addref(_this: *mut c_void) -> u32 {
        1
    }
    unsafe extern "win64" fn mock_pso_release(this: *mut c_void) -> u32 {
        if !this.is_null() {
            let p = &*(this as *const MockPso);
            p.released
                .store(true, std::sync::atomic::Ordering::Release);
        }
        0
    }
    #[repr(C)]
    struct MockDev2 {
        vtbl: *const [usize; 48],
        captured_stream: Vec<u8>,
        out_pso: *mut MockPso,
        qi_device2_called: bool,
        create_pso_stream_called: bool,
    }

    unsafe extern "win64" fn mock_dev_qi(
        this: *mut c_void,
        riid: *const Guid,
        out: *mut *mut c_void,
    ) -> HRESULT {
        let d = &mut *(this as *mut MockDev2);
        if !riid.is_null() && ptr::read_unaligned(riid) == IID_ID3D12Device2 {
            d.qi_device2_called = true;
            ptr::write_unaligned(out, this);
            0 // S_OK
        } else {
            ptr::write_unaligned(out, ptr::null_mut());
            0x8000_4002u32 as i32
        }
    }
    unsafe extern "win64" fn mock_dev_addref(_this: *mut c_void) -> u32 {
        2
    }
    unsafe extern "win64" fn mock_dev_release(_this: *mut c_void) -> u32 {
        1
    }
    unsafe extern "win64" fn mock_dev_create_pipeline_state(
        this: *mut c_void,
        desc: *const StreamDesc,
        riid: *const Guid,
        out: *mut *mut c_void,
    ) -> HRESULT {
        let d = &mut *(this as *mut MockDev2);
        d.create_pso_stream_called = true;
        assert!(!desc.is_null());
        assert!(!riid.is_null() && ptr::read_unaligned(riid) == IID_ID3D12PipelineState);
        let sd = ptr::read_unaligned(desc);
        assert!(!sd.stream.is_null() && sd.size > 0);
        assert_eq!(
            sd.stream as usize % 8,
            0,
            "stream must be 8-byte pointer aligned"
        );
        d.captured_stream = std::slice::from_raw_parts(sd.stream, sd.size).to_vec();
        ptr::write_unaligned(out, d.out_pso as *mut c_void);
        0 // S_OK
    }

    unsafe {
        let mock_pso_vtbl: [usize; 3] = [
            mock_pso_qi as *const () as usize,
            mock_pso_addref as *const () as usize,
            mock_pso_release as *const () as usize,
        ];
        let mut vtbl = [0usize; 48];
        vtbl[0] = mock_dev_qi as *const () as usize;
        vtbl[1] = mock_dev_addref as *const () as usize;
        vtbl[2] = mock_dev_release as *const () as usize;
        vtbl[47] = mock_dev_create_pipeline_state as *const () as usize;

        let mut mock_pso = MockPso {
            vtbl: &mock_pso_vtbl,
            released: std::sync::atomic::AtomicBool::new(false),
        };
        let mut mock_dev = MockDev2 {
            vtbl: &vtbl,
            captured_stream: Vec::new(),
            out_pso: &mut mock_pso,
            qi_device2_called: false,
            create_pso_stream_called: false,
        };

        let fake_rs = 0x1234_5678_9ABC_DEF0usize as *mut c_void;
        let layout = std::mem::ManuallyDrop::new(NriPipelineLayout {
            d3d12_root_sig: fake_rs,
            is_graphics: true,
            set_mappings: Vec::new(),
            set_root_offsets: Vec::new(),
            dynamic_cbv_mappings: Vec::new(),
            push_constants_base_index: 0,
        });
        let mut nri_dev = std::mem::ManuallyDrop::new(NriDevice {
            d3d12_device: (&mut mock_dev as *mut MockDev2) as *mut c_void,
            desc: std::mem::zeroed(),
            queues: Mutex::new([None, None, None]),
            cpu_heaps: Mutex::new([
                CpuDescriptorAllocator::default(),
                CpuDescriptorAllocator::default(),
                CpuDescriptorAllocator::default(),
                CpuDescriptorAllocator::default(),
            ]),
            dispatch_cmd_sig: Mutex::new(ptr::null_mut()),
            draw_cmd_sigs: Mutex::new(HashMap::new()),
            draw_indexed_cmd_sigs: Mutex::new(HashMap::new()),
        });

        let task_bc = [0x11u8; 32];
        let mesh_bc = [0x22u8; 48];
        let frag_bc = [0x33u8; 24];
        let stages = [
            NriShaderDesc {
                stage: 13, // MESH_CONTROL / TASK (AS)
                _pad0: [0; 7],
                bytecode: task_bc.as_ptr(),
                size: task_bc.len() as u64,
                entry_point_name: ptr::null(),
            },
            NriShaderDesc {
                stage: 14, // MESH_EVALUATION / MESH (MS)
                _pad0: [0; 7],
                bytecode: mesh_bc.as_ptr(),
                size: mesh_bc.len() as u64,
                entry_point_name: ptr::null(),
            },
            NriShaderDesc {
                stage: 5, // FRAGMENT (PS)
                _pad0: [0; 7],
                bytecode: frag_bc.as_ptr(),
                size: frag_bc.len() as u64,
                entry_point_name: ptr::null(),
            },
        ];
        let color_atts = [
            NriColorAttachmentDesc {
                format: 11, // RGBA8_UNORM -> DXGI 28
                color_blend: NriBlendingDesc {
                    src_factor: 6,
                    dst_factor: 7,
                    func: 0,
                },
                alpha_blend: NriBlendingDesc {
                    src_factor: 1,
                    dst_factor: 0,
                    func: 0,
                },
                color_write_mask: 0xF,
                blend_enabled: 1,
            },
            NriColorAttachmentDesc {
                format: 30, // RGBA16_SFLOAT -> DXGI 10
                color_blend: NriBlendingDesc {
                    src_factor: 1,
                    dst_factor: 0,
                    func: 0,
                },
                alpha_blend: NriBlendingDesc {
                    src_factor: 1,
                    dst_factor: 0,
                    func: 0,
                },
                color_write_mask: 0x7,
                blend_enabled: 0,
            },
        ];
        let om = NriOutputMergerDesc {
            color: color_atts.as_ptr(),
            depth: NriDepthAttachmentDesc {
                compare_func: 3,
                write: 1,
            }, // LESS, write=1
            stencil: NriStencilAttachmentDesc {
                front: NriStencilDesc {
                    compare_func: 0,
                    fail: 0,
                    pass: 0,
                    depth_fail: 0,
                },
                back: NriStencilDesc {
                    compare_func: 0,
                    fail: 0,
                    pass: 0,
                    depth_fail: 0,
                },
                reference: 0,
                compare_mask: 0xFF,
                write_mask: 0xFF,
            },
            depth_stencil_format: 63, // D32_SFLOAT -> DXGI 40
            color_logic_func: 0,
            _pad0: 0,
            color_num: 2,
            blend_consts: [0.25, 0.5, 0.75, 1.0],
            _pad1: 0,
        };
        let rast = NriRasterizationDesc {
            viewport_num: 1,
            depth_bias_constant_factor: 2,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 1.5,
            fill_mode: 0, // SOLID
            cull_mode: 2, // BACK
            sample_mask: 0xFFFF,
            sample_num: 1,
            alpha_to_coverage: 0,
            front_counter_clockwise: 1,
            depth_clamp: 1,
            antialiased_lines: 0,
            rasterizer_discard: 0,
            conservative_rasterization: 0,
            _pad: 0,
        };
        let gd = NriGraphicsPipelineDesc {
            pipeline_layout: &*layout as *const NriPipelineLayout,
            input_assembly: ptr::null(),
            rasterization: &rast,
            output_merger: &om,
            shader_stages: stages.as_ptr(),
            shader_stage_num: stages.len() as u32,
            _pad: 0,
        };

        let mut pipeline: *mut NriPipeline = ptr::null_mut();
        let res = core_create_graphics_pipeline(
            &mut *nri_dev as *mut NriDevice,
            &gd,
            &mut pipeline,
        );
        assert_eq!(res, NRI_RESULT_SUCCESS);
        assert!(!pipeline.is_null());
        assert!(mock_dev.qi_device2_called);
        assert!(mock_dev.create_pso_stream_called);
        assert!((*pipeline).is_graphics);
        assert!((*pipeline).blend_enabled);
        assert_eq!((*pipeline).blend_factor, [0.25, 0.5, 0.75, 1.0]);

        // Parse captured stream using exact D3D12_PIPELINE_STATE_STREAM_DESC layout rules.
        let bytes = &mock_dev.captured_stream;
        let mut off = 0usize;
        let mut seen_rs = 0usize;
        let mut seen_as: Option<(usize, usize)> = None;
        let mut seen_ms: Option<(usize, usize)> = None;
        let mut seen_ps: Option<(usize, usize)> = None;
        let mut seen_rtvs: Vec<u32> = Vec::new();
        let mut seen_dsv = 0u32;
        let mut seen_sample_count = 0u32;
        let mut seen_blend = false;
        let mut seen_raster = false;
        let mut seen_ds = false;
        while off + 4 <= bytes.len() {
            let ty = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            let (al, sz) = match ty {
                0 => (8, 8),
                1..=6 | 24 | 25 => (8, 16),
                8 => (4, 328),
                9 | 13 | 14 | 16 | 18 => (4, 4),
                10 => (4, 44),
                11 => (4, 52),
                15 => (4, 36),
                17 => (4, 8),
                _ => panic!("unexpected subobject type {ty} at offset {off}"),
            };
            let i = (off + 4).next_multiple_of(al);
            assert!(i + sz <= bytes.len());
            let q = &bytes[i..i + sz];
            match ty {
                0 => seen_rs = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize,
                2 => {
                    let p = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize;
                    let n = u64::from_le_bytes(q[8..16].try_into().unwrap()) as usize;
                    seen_ps = Some((p, n));
                }
                8 => seen_blend = true,
                10 => seen_raster = true,
                11 => seen_ds = true,
                15 => {
                    let n = u32::from_le_bytes(q[32..36].try_into().unwrap()) as usize;
                    seen_rtvs = (0..n)
                        .map(|k| u32::from_le_bytes(q[4 * k..4 * k + 4].try_into().unwrap()))
                        .collect();
                }
                16 => seen_dsv = u32::from_le_bytes(q[0..4].try_into().unwrap()),
                17 => seen_sample_count = u32::from_le_bytes(q[0..4].try_into().unwrap()),
                24 => {
                    let p = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize;
                    let n = u64::from_le_bytes(q[8..16].try_into().unwrap()) as usize;
                    seen_as = Some((p, n));
                }
                25 => {
                    let p = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize;
                    let n = u64::from_le_bytes(q[8..16].try_into().unwrap()) as usize;
                    seen_ms = Some((p, n));
                }
                _ => {}
            }
            off = (i + sz).next_multiple_of(8);
        }
        assert_eq!(seen_rs, fake_rs as usize);
        assert_eq!(seen_as, Some((task_bc.as_ptr() as usize, task_bc.len())));
        assert_eq!(seen_ms, Some((mesh_bc.as_ptr() as usize, mesh_bc.len())));
        assert_eq!(seen_ps, Some((frag_bc.as_ptr() as usize, frag_bc.len())));
        assert_eq!(seen_rtvs, vec![28, 10]);
        assert_eq!(seen_dsv, 40);
        assert_eq!(seen_sample_count, 1);
        assert!(seen_blend && seen_raster && seen_ds);

        core_destroy_pipeline(pipeline);
        assert!(mock_pso.released.load(std::sync::atomic::Ordering::Acquire));
    }
}

#[test]
fn nri_mesh_and_task_pipeline_on_real_d3d12_device() {
    const AS_RAW: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../d3d12-metal/tests/fixtures/mesh.as.dxil"
    ));
    const MS_RAW: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../d3d12-metal/tests/fixtures/mesh.ms.dxil"
    ));
    const MS_DIRECT_RAW: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../d3d12-metal/tests/fixtures/mesh_direct.ms.dxil"
    ));
    const PS_RAW: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../d3d12-metal/tests/fixtures/mesh.ps.dxil"
    ));

    struct EnvGuard {
        key: &'static str,
        prev: Option<String>,
    }
    impl EnvGuard {
        fn set(key: &'static str, val: &str) -> Self {
            let prev = std::env::var(key).ok();
            std::env::set_var(key, val);
            Self { key, prev }
        }
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }

    fn sign(raw: &[u8]) -> Vec<u8> {
        let mut v = raw.to_vec();
        dxil_metal::dxbc::sign_container(&mut v).unwrap();
        v
    }

    unsafe {
        let Some(d3d_dev) = make_test_d3d12_device() else {
            return;
        };
        let _mesh_guard = EnvGuard::set("AW2_MESH_SHADERS", "1");
        let (dev, core, _helper, _wrapper) = make_nri_device(d3d_dev);

        let create_layout: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriPipelineLayoutDesc,
            *mut *mut NriPipelineLayout,
        ) -> u32 = std::mem::transmute(core[16]);
        let create_gfx: unsafe extern "win64" fn(
            *mut NriDevice,
            *const NriGraphicsPipelineDesc,
            *mut *mut NriPipeline,
        ) -> u32 = std::mem::transmute(core[17]);
        let destroy_layout: unsafe extern "win64" fn(*mut NriPipelineLayout) =
            std::mem::transmute(core[28]);
        let destroy_pipe: unsafe extern "win64" fn(*mut NriPipeline) =
            std::mem::transmute(core[29]);

        // Root constants matching mesh.hlsl fixture (4 uints at b0, space0).
        let pc = NriPushConstantDesc {
            register_index: 0,
            size: 16,
            visibility: 0, // ALL
            _pad: [0; 3],
        };
        let pl_desc = NriPipelineLayoutDesc {
            descriptor_sets: ptr::null(),
            push_constants: &pc,
            descriptor_set_num: 0,
            push_constant_num: 1,
            stage_mask: (1 << 13) | (1 << 14) | (1 << 5), // TASK | MESH | FRAGMENT
            ignore_global_spirv_offsets: 1,
            _pad: [0; 5],
        };
        let mut layout: *mut NriPipelineLayout = ptr::null_mut();
        assert_eq!(
            create_layout(dev, &pl_desc, &mut layout),
            NRI_RESULT_SUCCESS
        );
        assert!(!layout.is_null());

        let as_bc = sign(AS_RAW);
        let ms_bc = sign(MS_RAW);
        let ms_direct_bc = sign(MS_DIRECT_RAW);
        let ps_bc = sign(PS_RAW);

        let color_att = NriColorAttachmentDesc {
            format: 11, // RGBA8_UNORM
            color_blend: NriBlendingDesc {
                src_factor: 1,
                dst_factor: 0,
                func: 0,
            },
            alpha_blend: NriBlendingDesc {
                src_factor: 1,
                dst_factor: 0,
                func: 0,
            },
            color_write_mask: 0xF,
            blend_enabled: 0,
        };
        let om = NriOutputMergerDesc {
            color: &color_att,
            depth: NriDepthAttachmentDesc {
                compare_func: 0,
                write: 0,
            },
            stencil: NriStencilAttachmentDesc {
                front: NriStencilDesc {
                    compare_func: 0,
                    fail: 0,
                    pass: 0,
                    depth_fail: 0,
                },
                back: NriStencilDesc {
                    compare_func: 0,
                    fail: 0,
                    pass: 0,
                    depth_fail: 0,
                },
                reference: 0,
                compare_mask: 0xFF,
                write_mask: 0xFF,
            },
            depth_stencil_format: 0,
            color_logic_func: 0,
            _pad0: 0,
            color_num: 1,
            blend_consts: [0.0; 4],
            _pad1: 0,
        };
        let rast = NriRasterizationDesc {
            viewport_num: 1,
            depth_bias_constant_factor: 0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            fill_mode: 0,
            cull_mode: 0,
            sample_mask: 0xFFFF,
            sample_num: 1,
            alpha_to_coverage: 0,
            front_counter_clockwise: 0,
            depth_clamp: 1,
            antialiased_lines: 0,
            rasterizer_discard: 0,
            conservative_rasterization: 0,
            _pad: 0,
        };

        // 1. Amplification/Task shader without a Mesh shader must fail via D3D12 CreatePipelineState.
        let task_only = [NriShaderDesc {
            stage: 13,
            _pad0: [0; 7],
            bytecode: as_bc.as_ptr(),
            size: as_bc.len() as u64,
            entry_point_name: ptr::null(),
        }];
        let gd_invalid = NriGraphicsPipelineDesc {
            pipeline_layout: layout,
            input_assembly: ptr::null(),
            rasterization: &rast,
            output_merger: &om,
            shader_stages: task_only.as_ptr(),
            shader_stage_num: 1,
            _pad: 0,
        };
        let mut bad_pipe: *mut NriPipeline = ptr::null_mut();
        assert_eq!(
            create_gfx(dev, &gd_invalid, &mut bad_pipe),
            NRI_RESULT_INVALID_ARGUMENT
        );
        assert!(bad_pipe.is_null());

        // 2. Full AS + MS + PS and MS + PS compilation through real Metal 4 mesh pipelines
        //    when the Metal Shader Converter is installed on this host.
        if dxil_metal::converter_available() {
            let stages_as_ms_ps = [
                NriShaderDesc {
                    stage: 13,
                    _pad0: [0; 7],
                    bytecode: as_bc.as_ptr(),
                    size: as_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
                NriShaderDesc {
                    stage: 14,
                    _pad0: [0; 7],
                    bytecode: ms_bc.as_ptr(),
                    size: ms_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
                NriShaderDesc {
                    stage: 5,
                    _pad0: [0; 7],
                    bytecode: ps_bc.as_ptr(),
                    size: ps_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
            ];
            let gd_as_ms = NriGraphicsPipelineDesc {
                pipeline_layout: layout,
                input_assembly: ptr::null(),
                rasterization: &rast,
                output_merger: &om,
                shader_stages: stages_as_ms_ps.as_ptr(),
                shader_stage_num: stages_as_ms_ps.len() as u32,
                _pad: 0,
            };
            let mut pipe_as_ms: *mut NriPipeline = ptr::null_mut();
            assert_eq!(
                create_gfx(dev, &gd_as_ms, &mut pipe_as_ms),
                NRI_RESULT_SUCCESS
            );
            assert!(!pipe_as_ms.is_null());
            let pso_ref = crate::d3d12::pso::pso_from_ptr((*pipe_as_ms).d3d12_pso).unwrap();
            assert!(matches!(
                pso_ref.kind,
                crate::d3d12::pso::PipelineKind::Mesh(_)
            ));
            destroy_pipe(pipe_as_ms);

            let stages_ms_ps = [
                NriShaderDesc {
                    stage: 14,
                    _pad0: [0; 7],
                    bytecode: ms_direct_bc.as_ptr(),
                    size: ms_direct_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
                NriShaderDesc {
                    stage: 5,
                    _pad0: [0; 7],
                    bytecode: ps_bc.as_ptr(),
                    size: ps_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
            ];
            let gd_ms = NriGraphicsPipelineDesc {
                pipeline_layout: layout,
                input_assembly: ptr::null(),
                rasterization: &rast,
                output_merger: &om,
                shader_stages: stages_ms_ps.as_ptr(),
                shader_stage_num: stages_ms_ps.len() as u32,
                _pad: 0,
            };
            let mut pipe_ms: *mut NriPipeline = ptr::null_mut();
            assert_eq!(create_gfx(dev, &gd_ms, &mut pipe_ms), NRI_RESULT_SUCCESS);
            assert!(!pipe_ms.is_null());
            let pso_ref2 = crate::d3d12::pso::pso_from_ptr((*pipe_ms).d3d12_pso).unwrap();
            assert!(matches!(
                pso_ref2.kind,
                crate::d3d12::pso::PipelineKind::Mesh(_)
            ));
            destroy_pipe(pipe_ms);
        }

        destroy_layout(layout);
        nriDestroyDevice(dev);
        com_release(d3d_dev);
    }
}

#[test]
fn nri_convert_dxgi_format_to_nri_typed_and_typeless() {
    unsafe {
        assert_eq!(nriConvertDXGIFormatToNRI(0), 0); // DXGI_FORMAT_UNKNOWN -> Format::UNKNOWN
        assert_eq!(nriConvertDXGIFormatToNRI(28), 11); // DXGI_FORMAT_R8G8B8A8_UNORM -> RGBA8_UNORM
        assert_eq!(nriConvertDXGIFormatToNRI(87), 9); // DXGI_FORMAT_B8G8R8A8_UNORM -> BGRA8_UNORM
        assert_eq!(nriConvertDXGIFormatToNRI(40), 63); // DXGI_FORMAT_D32_FLOAT -> D32_SFLOAT
        assert_eq!(nriConvertDXGIFormatToNRI(27), 11); // DXGI_FORMAT_R8G8B8A8_TYPELESS -> RGBA8_UNORM
        assert_eq!(nriConvertDXGIFormatToNRI(44), 62); // DXGI_FORMAT_R24G8_TYPELESS -> D24_UNORM_S8_UINT
        assert_eq!(nriConvertDXGIFormatToNRI(9999), 0);
    }
}
