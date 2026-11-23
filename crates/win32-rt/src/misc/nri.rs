//! Builtin `NRI.dll` (NVIDIA Render Interface v1.99) backed by real D3D12 COM
//! objects (`crate::d3d12` / `d3d12-metal` / `dxil-metal`).
//!
//! Exposes the three `NRI.dll` entry points (`nriCreateDeviceFromD3D12Device`,
//! `nriDestroyDevice`, `nriGetInterface`) and populates:
//! * `"nri::CoreInterface"` (`0x350` bytes = 106 function pointers)
//! * `"nri::HelperInterface"` (`0x28` bytes = 5 function pointers)
//! * `"nri::WrapperD3D12Interface"` (`0x28` bytes = 5 function pointers)
//!
//! Struct layouts, enums, and D3D12 backend mappings follow `third_party/nri/`
//! (`Include/NRI.h`, `Include/NRIDescs.h`, `Include/Extensions/NRIWrapperD3D12.h`,
//! `Include/Extensions/NRIHelper.h`, and `Source/D3D12/`).

mod abi;
mod cmdlist;
mod descriptor;
mod device;
mod helper;

use std::mem::size_of;
use std::ptr;

use crate::strings::cstr_slice;
use helper::{
    ensure_interfaces_registered, CORE_INTERFACE, HELPER_INTERFACE, WRAPPER_D3D12_INTERFACE,
};

pub use abi::{
    NriAccelerationStructureD3D12Desc, NriAdapterDesc, NriAliasingBarrierDesc, NriBlendingDesc,
    NriBufferAliasingBarrierDesc, NriBufferD3D12Desc, NriBufferDesc, NriBufferMemoryBindingDesc,
    NriBufferTransitionBarrierDesc, NriBufferUploadDesc, NriBufferViewDesc, NriClearDesc,
    NriClearStorageBufferDesc, NriClearStorageTextureDesc, NriColorAttachmentDesc,
    NriCommandBufferD3D12Desc, NriComputePipelineDesc, NriDepthAttachmentDesc,
    NriDescriptorPoolDesc, NriDescriptorRangeDesc, NriDescriptorRangeUpdateDesc,
    NriDescriptorSetCopyDesc, NriDescriptorSetDesc, NriDeviceCreationD3D12Desc, NriDeviceDesc,
    NriDynamicConstantBufferDesc, NriFrameBufferDesc, NriGraphicsPipelineDesc,
    NriInputAssemblyDesc, NriMemoryD3D12Desc, NriMemoryDesc, NriOutputMergerDesc,
    NriPipelineLayoutDesc, NriPushConstantDesc, NriQueryPoolDesc, NriQueueSubmitDesc,
    NriRasterizationDesc, NriRect, NriResourceGroupDesc, NriSamplerDesc, NriShaderDesc,
    NriStencilAttachmentDesc, NriStencilDesc, NriTextureAliasingBarrierDesc, NriTextureD3D12Desc,
    NriTextureDataLayoutDesc, NriTextureDesc, NriTextureMemoryBindingDesc, NriTextureRegionDesc,
    NriTextureSubresourceUploadDesc, NriTextureTransitionBarrierDesc, NriTextureUploadDesc,
    NriTextureViewDesc, NriTransitionBarrierDesc, NriVertexAttributeD3D, NriVertexAttributeDesc,
    NriVertexStreamDesc, NriViewport, NRI_GRAPHICS_API_D3D12, NRI_RESULT_DEVICE_LOST,
    NRI_RESULT_FAILURE, NRI_RESULT_INVALID_ARGUMENT, NRI_RESULT_OUT_OF_MEMORY, NRI_RESULT_SUCCESS,
    NRI_RESULT_UNSUPPORTED,
};
pub use cmdlist::NriCommandBuffer;
pub use descriptor::{NriDescriptor, NriDescriptorPool, NriDescriptorSet, NriPipelineLayout};
pub use device::{
    NriAccelerationStructure, NriBuffer, NriCommandAllocator, NriCommandQueue, NriDevice, NriFence,
    NriFrameBuffer, NriMemory, NriPipeline, NriQueryPool, NriTexture,
};

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

// ---------------------------------------------------------------------------
// Exported NRI.dll entry points
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn nriCreateDeviceFromD3D12Device(
    device_desc: *const u8,
    device_out: *mut *mut NriDevice,
) -> u32 {
    ensure_interfaces_registered();
    device::create_device_from_d3d12(device_desc, device_out)
}

pub unsafe extern "win64" fn nriDestroyDevice(device: *mut NriDevice) {
    if !device.is_null() {
        drop(Box::from_raw(device));
    }
}

pub unsafe extern "win64" fn nriGetInterface(
    device: *const NriDevice,
    interface_name: *const u8,
    interface_size: usize,
    interface_out: *mut u8,
) -> u32 {
    ensure_interfaces_registered();
    if device.is_null() || interface_name.is_null() || interface_out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    ptr::write_bytes(interface_out, 0, interface_size);
    let name = cstr_slice(interface_name);
    let (src_ptr, src_size) = match name {
        b"nri::CoreInterface" | b"CoreInterface" => (
            CORE_INTERFACE.0.as_ptr() as *const u8,
            CORE_INTERFACE.0.len() * size_of::<crate::Ptr>(),
        ),
        b"nri::HelperInterface" | b"HelperInterface" => (
            HELPER_INTERFACE.0.as_ptr() as *const u8,
            HELPER_INTERFACE.0.len() * size_of::<crate::Ptr>(),
        ),
        b"nri::WrapperD3D12Interface" | b"WrapperD3D12Interface" => (
            WRAPPER_D3D12_INTERFACE.0.as_ptr() as *const u8,
            WRAPPER_D3D12_INTERFACE.0.len() * size_of::<crate::Ptr>(),
        ),
        _ => {
            log::debug!(
                target: "d3d",
                "nriGetInterface: unsupported interface {:?} -> UNSUPPORTED",
                String::from_utf8_lossy(name)
            );
            return NRI_RESULT_UNSUPPORTED;
        }
    };
    if interface_size < src_size {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    ptr::copy_nonoverlapping(src_ptr, interface_out, src_size);
    NRI_RESULT_SUCCESS
}

pub mod exports {
    pub use super::{nriCreateDeviceFromD3D12Device, nriDestroyDevice, nriGetInterface};
}

#[cfg(test)]
mod tests;
