//! `nri::HelperInterface` callbacks, `nri::WrapperD3D12Interface` callbacks,
//! and static NRI interface dispatch tables.

use std::collections::BTreeMap;
use std::mem::size_of;
use std::ptr;
use std::sync::Once;

use super::abi::*;
use super::cmdlist::{
    core_begin_command_buffer, core_cmd_begin_annotation, core_cmd_begin_query,
    core_cmd_begin_render_pass, core_cmd_clear_attachments, core_cmd_clear_storage_buffer,
    core_cmd_clear_storage_texture, core_cmd_copy_buffer, core_cmd_copy_queries,
    core_cmd_copy_texture, core_cmd_dispatch, core_cmd_dispatch_indirect, core_cmd_draw,
    core_cmd_draw_indexed, core_cmd_draw_indexed_indirect, core_cmd_draw_indirect,
    core_cmd_end_annotation, core_cmd_end_query, core_cmd_end_render_pass,
    core_cmd_pipeline_barrier, core_cmd_readback_texture_to_buffer, core_cmd_reset_queries,
    core_cmd_set_constants, core_cmd_set_descriptor_pool, core_cmd_set_descriptor_set,
    core_cmd_set_index_buffer, core_cmd_set_pipeline, core_cmd_set_pipeline_layout,
    core_cmd_set_sample_positions, core_cmd_set_scissors, core_cmd_set_stencil_reference,
    core_cmd_set_vertex_buffers, core_cmd_set_viewports, core_cmd_upload_buffer_to_texture,
    core_create_command_buffer, core_end_command_buffer, core_get_fence_value, core_queue_signal,
    core_queue_submit, core_queue_wait, core_wait, d3d12_cmd_get_type, NriCommandBuffer,
    CMD_SET_DEPTH_BOUNDS_ENTRY,
};
use super::descriptor::{
    core_allocate_descriptor_sets, core_copy_descriptor_set, core_create_buffer_view,
    core_create_descriptor_pool, core_create_pipeline_layout, core_create_sampler,
    core_create_texture_1d_view, core_create_texture_2d_view, core_create_texture_3d_view,
    core_reset_descriptor_pool, core_update_descriptor_ranges,
    core_update_dynamic_constant_buffers,
};
use super::device::*;
use crate::com::register_static_ptr_slice;
use crate::d3d12::com::com_add_ref;
use crate::strings::{rd, wr};

// ---------------------------------------------------------------------------
// HelperInterface callbacks (`third_party/nri/Source/Shared/Helper*.cpp`)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct MemoryTypeGroup {
    buffers: Vec<(*mut NriBuffer, u64)>,
    textures: Vec<(*mut NriTexture, u64)>,
    total_size: u64,
}

unsafe fn group_resources_for_allocation(
    device: *mut NriDevice,
    rg: &NriResourceGroupDesc,
) -> (
    BTreeMap<u32, MemoryTypeGroup>,
    Vec<(*mut NriBuffer, NriMemoryDesc)>,
    Vec<(*mut NriTexture, NriMemoryDesc)>,
) {
    let mut groups: BTreeMap<u32, MemoryTypeGroup> = BTreeMap::new();
    let mut ded_buffers = Vec::new();
    let mut ded_textures = Vec::new();
    let gran = (*device).desc.buffer_texture_granularity.max(1) as u64;

    if rg.buffer_num > 0 && !rg.buffers.is_null() {
        for i in 0..rg.buffer_num as usize {
            let buf = rd(rg.buffers.add(i));
            if buf.is_null() {
                continue;
            }
            let mut md = NriMemoryDesc::default();
            core_get_buffer_memory_info(buf, rg.memory_location, &mut md);
            if md.must_be_dedicated != 0 {
                ded_buffers.push((buf, md));
            } else {
                let g = groups.entry(md.memory_type).or_default();
                let off = align_up(g.total_size, md.alignment.max(1) as u64);
                g.buffers.push((buf, off));
                g.total_size = off.saturating_add(md.size);
            }
        }
    }

    if rg.texture_num > 0 && !rg.textures.is_null() {
        for i in 0..rg.texture_num as usize {
            let tex = rd(rg.textures.add(i));
            if tex.is_null() {
                continue;
            }
            let mut md = NriMemoryDesc::default();
            core_get_texture_memory_info(tex, rg.memory_location, &mut md);
            if md.must_be_dedicated != 0 {
                ded_textures.push((tex, md));
            } else {
                let g = groups.entry(md.memory_type).or_default();
                if g.textures.is_empty() && g.total_size > 0 {
                    g.total_size = align_up(g.total_size, gran);
                }
                let off = align_up(g.total_size, md.alignment.max(1) as u64);
                g.textures.push((tex, off));
                g.total_size = off.saturating_add(md.size);
            }
        }
    }

    (groups, ded_buffers, ded_textures)
}

unsafe extern "win64" fn helper_calculate_allocation_number(
    device: *mut NriDevice,
    rg_ptr: *const NriResourceGroupDesc,
) -> u32 {
    if device.is_null() || rg_ptr.is_null() {
        return 0;
    }
    let rg = rd(rg_ptr);
    let (groups, ded_bufs, ded_texs) = group_resources_for_allocation(device, &rg);
    (groups.len() + ded_bufs.len() + ded_texs.len()) as u32
}

unsafe extern "win64" fn helper_allocate_and_bind_memory(
    device: *mut NriDevice,
    rg_ptr: *const NriResourceGroupDesc,
    allocations: *mut *mut NriMemory,
) -> u32 {
    if device.is_null() || rg_ptr.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let rg = rd(rg_ptr);
    let (groups, ded_bufs, ded_texs) = group_resources_for_allocation(device, &rg);
    let total = groups.len() + ded_bufs.len() + ded_texs.len();
    if total == 0 {
        return NRI_RESULT_SUCCESS;
    }
    if allocations.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }

    let mut created: Vec<*mut NriMemory> = Vec::with_capacity(total);
    let mut buf_binds: Vec<NriBufferMemoryBindingDesc> = Vec::new();
    let mut tex_binds: Vec<NriTextureMemoryBindingDesc> = Vec::new();
    let mut status = NRI_RESULT_SUCCESS;

    for (mem_type, group) in groups {
        let mut mem: *mut NriMemory = ptr::null_mut();
        status = core_allocate_memory(
            device,
            NRI_WHOLE_DEVICE_GROUP,
            mem_type,
            group.total_size,
            &mut mem,
        );
        if status != NRI_RESULT_SUCCESS {
            break;
        }
        wr(allocations.add(created.len()), mem);
        created.push(mem);
        for (buf, off) in group.buffers {
            buf_binds.push(NriBufferMemoryBindingDesc {
                memory: mem,
                buffer: buf,
                offset: off,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
        for (tex, off) in group.textures {
            tex_binds.push(NriTextureMemoryBindingDesc {
                memory: mem,
                texture: tex,
                offset: off,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
    }

    if status == NRI_RESULT_SUCCESS {
        for (buf, md) in ded_bufs {
            let mut mem: *mut NriMemory = ptr::null_mut();
            status = core_allocate_memory(
                device,
                NRI_WHOLE_DEVICE_GROUP,
                md.memory_type,
                md.size,
                &mut mem,
            );
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            wr(allocations.add(created.len()), mem);
            created.push(mem);
            buf_binds.push(NriBufferMemoryBindingDesc {
                memory: mem,
                buffer: buf,
                offset: 0,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
    }

    if status == NRI_RESULT_SUCCESS {
        for (tex, md) in ded_texs {
            let mut mem: *mut NriMemory = ptr::null_mut();
            status = core_allocate_memory(
                device,
                NRI_WHOLE_DEVICE_GROUP,
                md.memory_type,
                md.size,
                &mut mem,
            );
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            wr(allocations.add(created.len()), mem);
            created.push(mem);
            tex_binds.push(NriTextureMemoryBindingDesc {
                memory: mem,
                texture: tex,
                offset: 0,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
    }

    if status == NRI_RESULT_SUCCESS && !buf_binds.is_empty() {
        status = core_bind_buffer_memory(device, buf_binds.as_ptr(), buf_binds.len() as u32);
    }
    if status == NRI_RESULT_SUCCESS && !tex_binds.is_empty() {
        status = core_bind_texture_memory(device, tex_binds.as_ptr(), tex_binds.len() as u32);
    }

    if status != NRI_RESULT_SUCCESS {
        for (idx, mem) in created.into_iter().enumerate() {
            core_free_memory(mem);
            wr(allocations.add(idx), ptr::null_mut());
        }
    }
    status
}

unsafe extern "win64" fn helper_wait_for_idle(queue: *mut NriCommandQueue) -> u32 {
    if queue.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let dev = (*queue).device;
    let mut fence: *mut NriFence = ptr::null_mut();
    let res = core_create_fence(dev, 0, &mut fence);
    if res != NRI_RESULT_SUCCESS {
        return res;
    }
    core_queue_signal(queue, fence, 1);
    core_wait(fence, 1);
    core_destroy_fence(fence);
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn helper_change_resource_states(
    queue: *mut NriCommandQueue,
    transitions: *const NriTransitionBarrierDesc,
) -> u32 {
    if queue.is_null() || transitions.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let mut alloc: *mut NriCommandAllocator = ptr::null_mut();
    let res = core_create_command_allocator(queue, NRI_WHOLE_DEVICE_GROUP, &mut alloc);
    if res != NRI_RESULT_SUCCESS {
        return res;
    }
    let mut cmd: *mut NriCommandBuffer = ptr::null_mut();
    let res = core_create_command_buffer(alloc, &mut cmd);
    if res != NRI_RESULT_SUCCESS {
        core_destroy_command_allocator(alloc);
        return res;
    }
    let mut status = core_begin_command_buffer(cmd, ptr::null(), 0);
    if status == NRI_RESULT_SUCCESS {
        core_cmd_pipeline_barrier(cmd, transitions, ptr::null(), 0);
        status = core_end_command_buffer(cmd);
    }
    if status == NRI_RESULT_SUCCESS {
        let cb_ptr: *const NriCommandBuffer = cmd;
        let submit = NriQueueSubmitDesc {
            command_buffers: &cb_ptr,
            command_buffer_num: 1,
            physical_device_index: 0,
        };
        core_queue_submit(queue, &submit);
        status = helper_wait_for_idle(queue);
    }
    core_destroy_command_buffer(cmd);
    core_destroy_command_allocator(alloc);
    status
}

unsafe extern "win64" fn helper_upload_data(
    queue: *mut NriCommandQueue,
    tex_descs: *const NriTextureUploadDesc,
    tex_num: u32,
    buf_descs: *const NriBufferUploadDesc,
    buf_num: u32,
) -> u32 {
    if queue.is_null()
        || (tex_num > 0 && tex_descs.is_null())
        || (buf_num > 0 && buf_descs.is_null())
    {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    if tex_num == 0 && buf_num == 0 {
        return NRI_RESULT_SUCCESS;
    }
    let dev = (*queue).device;
    let row_align = (*dev).desc.upload_buffer_texture_row_alignment.max(1) as u64;
    let slice_align = (*dev).desc.upload_buffer_texture_slice_alignment.max(1) as u64;

    let mut staging_size: u64 = 65536;
    for i in 0..tex_num as usize {
        let td = rd(tex_descs.add(i));
        if td.subresources.is_null() {
            continue;
        }
        let sub = rd(td.subresources);
        if sub.row_pitch == 0 {
            continue;
        }
        let rows = (sub.slice_pitch / sub.row_pitch).max(1) as u64;
        let aligned_row = align_up(sub.row_pitch as u64, row_align);
        let aligned_slice = align_up(rows * aligned_row, slice_align);
        let total = aligned_slice * (sub.slice_num.max(1) as u64);
        staging_size = staging_size.max(total);
    }
    staging_size = align_up(staging_size, COPY_ALIGNMENT);

    // Create staging buffer + memory + fence + command allocator + command buffer.
    let mut upload_buf: *mut NriBuffer = ptr::null_mut();
    let bd = NriBufferDesc {
        size: staging_size,
        structure_stride: 0,
        usage_mask: 0,
        _pad0: 0,
        physical_device_mask: 0,
        _pad1: 0,
    };
    let mut status = core_create_buffer(dev, &bd, &mut upload_buf);
    if status != NRI_RESULT_SUCCESS {
        return status;
    }
    let mut md = NriMemoryDesc::default();
    core_get_buffer_memory_info(upload_buf, 1, &mut md); // HOST_UPLOAD = 1
    let mut upload_mem: *mut NriMemory = ptr::null_mut();
    status = core_allocate_memory(
        dev,
        NRI_WHOLE_DEVICE_GROUP,
        md.memory_type,
        md.size,
        &mut upload_mem,
    );
    if status != NRI_RESULT_SUCCESS {
        core_destroy_buffer(upload_buf);
        return status;
    }
    let bind = NriBufferMemoryBindingDesc {
        memory: upload_mem,
        buffer: upload_buf,
        offset: 0,
        physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
        _pad: 0,
    };
    status = core_bind_buffer_memory(dev, &bind, 1);
    let mut fence: *mut NriFence = ptr::null_mut();
    let mut alloc: *mut NriCommandAllocator = ptr::null_mut();
    let mut cmd: *mut NriCommandBuffer = ptr::null_mut();
    if status == NRI_RESULT_SUCCESS {
        status = core_create_fence(dev, 0, &mut fence);
    }
    if status == NRI_RESULT_SUCCESS {
        status = core_create_command_allocator(queue, NRI_WHOLE_DEVICE_GROUP, &mut alloc);
    }
    if status == NRI_RESULT_SUCCESS {
        status = core_create_command_buffer(alloc, &mut cmd);
    }

    let mut fence_val = 1u64;
    let submit_and_wait = |cmd: *mut NriCommandBuffer,
                           alloc: *mut NriCommandAllocator,
                           fence: *mut NriFence,
                           fence_val: &mut u64|
     -> u32 {
        unsafe {
            let r = core_end_command_buffer(cmd);
            if r != NRI_RESULT_SUCCESS {
                return r;
            }
            let cb_ptr: *const NriCommandBuffer = cmd;
            let submit = NriQueueSubmitDesc {
                command_buffers: &cb_ptr,
                command_buffer_num: 1,
                physical_device_index: 0,
            };
            core_queue_submit(queue, &submit);
            core_queue_signal(queue, fence, *fence_val);
            core_wait(fence, *fence_val);
            *fence_val += 1;
            core_reset_command_allocator(alloc);
            NRI_RESULT_SUCCESS
        }
    };

    // Upload textures.
    if status == NRI_RESULT_SUCCESS && tex_num > 0 {
        let mut init_barriers = Vec::with_capacity(tex_num as usize);
        let mut final_barriers = Vec::with_capacity(tex_num as usize);
        for i in 0..tex_num as usize {
            let td = rd(tex_descs.add(i));
            init_barriers.push(NriTextureTransitionBarrierDesc {
                texture: td.texture,
                mip_offset: 0,
                mip_num: td.mip_num,
                array_offset: 0,
                array_size: td.array_size,
                prev_access: 0,
                next_access: 1 << 10, // COPY_DESTINATION
                prev_layout: 8,       // UNKNOWN
                next_layout: 0,       // GENERAL
                _pad: 0,
            });
            final_barriers.push(NriTextureTransitionBarrierDesc {
                texture: td.texture,
                mip_offset: 0,
                mip_num: td.mip_num,
                array_offset: 0,
                array_size: td.array_size,
                prev_access: 1 << 10,
                next_access: td.next_access,
                prev_layout: 0,
                next_layout: td.next_layout,
                _pad: 0,
            });
        }
        status = core_begin_command_buffer(cmd, ptr::null(), 0);
        if status == NRI_RESULT_SUCCESS {
            let tb = NriTransitionBarrierDesc {
                buffers: ptr::null(),
                textures: init_barriers.as_ptr(),
                buffer_num: 0,
                texture_num: init_barriers.len() as u32,
            };
            core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
            status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
        }

        let mut i = 0usize;
        let mut arr_off = 0u16;
        let mut mip_off = 0u16;
        while status == NRI_RESULT_SUCCESS && i < tex_num as usize {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            let mut buf_off = 0u64;
            'batch: while i < tex_num as usize {
                let td = rd(tex_descs.add(i));
                if td.subresources.is_null() {
                    i += 1;
                    continue;
                }
                while arr_off < td.array_size {
                    while mip_off < td.mip_num {
                        let sub = rd(td
                            .subresources
                            .add((arr_off as usize) * (td.mip_num as usize) + (mip_off as usize)));
                        if sub.row_pitch == 0 || sub.slices.is_null() {
                            mip_off += 1;
                            continue;
                        }
                        let rows = (sub.slice_pitch / sub.row_pitch).max(1) as u64;
                        let aligned_row = align_up(sub.row_pitch as u64, row_align);
                        let aligned_slice = align_up(rows * aligned_row, slice_align);
                        let content_size = aligned_slice * (sub.slice_num.max(1) as u64);
                        if content_size > staging_size {
                            status = NRI_RESULT_OUT_OF_MEMORY;
                            break 'batch;
                        }
                        if content_size > staging_size - buf_off {
                            break 'batch;
                        }
                        let dst_ptr = core_map_buffer(upload_buf, buf_off, content_size);
                        if dst_ptr.is_null() {
                            status = NRI_RESULT_FAILURE;
                            break 'batch;
                        }
                        for k in 0..sub.slice_num.max(1) as usize {
                            for l in 0..rows as usize {
                                let d = dst_ptr.add(
                                    k * (aligned_slice as usize) + l * (aligned_row as usize),
                                );
                                let s = sub.slices.add(
                                    k * (sub.slice_pitch as usize) + l * (sub.row_pitch as usize),
                                );
                                ptr::copy_nonoverlapping(s, d, sub.row_pitch as usize);
                            }
                        }
                        core_unmap_buffer(upload_buf);
                        let src_layout = NriTextureDataLayoutDesc {
                            offset: buf_off,
                            row_pitch: aligned_row as u32,
                            slice_pitch: aligned_slice as u32,
                        };
                        let dst_reg = NriTextureRegionDesc {
                            offset: [0; 3],
                            size: [0; 3],
                            mip_offset: mip_off,
                            array_offset: arr_off,
                        };
                        core_cmd_upload_buffer_to_texture(
                            cmd,
                            td.texture,
                            &dst_reg,
                            upload_buf,
                            &src_layout,
                        );
                        buf_off = align_up(buf_off + content_size, COPY_ALIGNMENT);
                        mip_off += 1;
                    }
                    mip_off = 0;
                    arr_off += 1;
                }
                arr_off = 0;
                i += 1;
            }
            if status == NRI_RESULT_SUCCESS {
                status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
            }
        }

        if status == NRI_RESULT_SUCCESS {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status == NRI_RESULT_SUCCESS {
                let tb = NriTransitionBarrierDesc {
                    buffers: ptr::null(),
                    textures: final_barriers.as_ptr(),
                    buffer_num: 0,
                    texture_num: final_barriers.len() as u32,
                };
                core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
                status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
            }
        }
    }

    // Upload buffers.
    if status == NRI_RESULT_SUCCESS && buf_num > 0 {
        let mut init_barriers = Vec::with_capacity(buf_num as usize);
        let mut final_barriers = Vec::with_capacity(buf_num as usize);
        for i in 0..buf_num as usize {
            let bd_u = rd(buf_descs.add(i));
            init_barriers.push(NriBufferTransitionBarrierDesc {
                buffer: bd_u.buffer,
                prev_access: bd_u.prev_access,
                next_access: 1 << 10,
                _pad: 0,
            });
            final_barriers.push(NriBufferTransitionBarrierDesc {
                buffer: bd_u.buffer,
                prev_access: 1 << 10,
                next_access: bd_u.next_access,
                _pad: 0,
            });
        }
        status = core_begin_command_buffer(cmd, ptr::null(), 0);
        if status == NRI_RESULT_SUCCESS {
            let tb = NriTransitionBarrierDesc {
                buffers: init_barriers.as_ptr(),
                textures: ptr::null(),
                buffer_num: init_barriers.len() as u32,
                texture_num: 0,
            };
            core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
            status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
        }

        let mut i = 0usize;
        let mut content_off = 0u64;
        while status == NRI_RESULT_SUCCESS && i < buf_num as usize {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            let mapped = core_map_buffer(upload_buf, 0, staging_size);
            if mapped.is_null() {
                status = NRI_RESULT_FAILURE;
                break;
            }
            let mut buf_off = 0u64;
            while i < buf_num as usize {
                let bd_u = rd(buf_descs.add(i));
                if bd_u.data_size == 0 || bd_u.data.is_null() {
                    i += 1;
                    content_off = 0;
                    continue;
                }
                let free = staging_size - buf_off;
                if free == 0 {
                    break;
                }
                let copy_n = (bd_u.data_size - content_off).min(free);
                ptr::copy_nonoverlapping(
                    bd_u.data.add(content_off as usize),
                    mapped.add(buf_off as usize),
                    copy_n as usize,
                );
                core_cmd_copy_buffer(
                    cmd,
                    bd_u.buffer,
                    0,
                    bd_u.buffer_offset + content_off,
                    upload_buf,
                    0,
                    buf_off,
                    copy_n,
                );
                content_off += copy_n;
                buf_off += copy_n;
                if content_off < bd_u.data_size {
                    break;
                }
                content_off = 0;
                buf_off = align_up(buf_off, COPY_ALIGNMENT);
                i += 1;
            }
            core_unmap_buffer(upload_buf);
            status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
        }

        if status == NRI_RESULT_SUCCESS {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status == NRI_RESULT_SUCCESS {
                let tb = NriTransitionBarrierDesc {
                    buffers: final_barriers.as_ptr(),
                    textures: ptr::null(),
                    buffer_num: final_barriers.len() as u32,
                    texture_num: 0,
                };
                core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
                status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
            }
        }
    }

    core_destroy_command_buffer(cmd);
    core_destroy_command_allocator(alloc);
    core_destroy_fence(fence);
    core_destroy_buffer(upload_buf);
    core_free_memory(upload_mem);
    status
}

// ---------------------------------------------------------------------------
// WrapperD3D12Interface callbacks (`third_party/nri/Include/Extensions/NRIWrapperD3D12.h`)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn wrapper_create_command_buffer_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriCommandBufferD3D12Desc,
    out: *mut *mut NriCommandBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = rd(desc_ptr);
    if d.d3d12_command_list.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_command_list);
    if !d.d3d12_command_allocator.is_null() {
        com_add_ref(d.d3d12_command_allocator);
    }
    let list_type = d3d12_cmd_get_type(d.d3d12_command_list);
    wr(
        out,
        Box::into_raw(Box::new(NriCommandBuffer {
            device,
            d3d12_list: d.d3d12_command_list,
            d3d12_allocator: d.d3d12_command_allocator,
            list_type,
            pipeline_layout: ptr::null(),
            is_graphics_layout: false,
            pipeline: ptr::null(),
            frame_buffer: ptr::null(),
            descriptor_sets: Vec::new(),
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn wrapper_create_buffer_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriBufferD3D12Desc,
    out: *mut *mut NriBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = rd(desc_ptr);
    if d.d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_resource);
    let rdesc = d3d12_res_get_desc(d.d3d12_resource);
    wr(
        out,
        Box::into_raw(Box::new(NriBuffer {
            device,
            d3d12_resource: d.d3d12_resource,
            desc: rdesc,
            structure_stride: d.structure_stride,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn wrapper_create_texture_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriTextureD3D12Desc,
    out: *mut *mut NriTexture,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = rd(desc_ptr);
    if d.d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_resource);
    let rdesc = d3d12_res_get_desc(d.d3d12_resource);
    let nri_format = dxgi_to_nri(rdesc.Format);
    wr(
        out,
        Box::into_raw(Box::new(NriTexture {
            device,
            d3d12_resource: d.d3d12_resource,
            desc: rdesc,
            nri_format,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn wrapper_create_memory_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriMemoryD3D12Desc,
    out: *mut *mut NriMemory,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = rd(desc_ptr);
    if d.d3d12_heap.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_heap);
    let hdesc = d3d12_heap_get_desc(d.d3d12_heap);
    wr(
        out,
        Box::into_raw(Box::new(NriMemory {
            d3d12_heap: d.d3d12_heap,
            heap_desc: hdesc,
            dedicated: false,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn wrapper_create_acceleration_structure_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriAccelerationStructureD3D12Desc,
    out: *mut *mut NriAccelerationStructure,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = rd(desc_ptr);
    if d.d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_resource);
    wr(
        out,
        Box::into_raw(Box::new(NriAccelerationStructure {
            d3d12_resource: d.d3d12_resource,
            _scratch_size: d.scratch_data_size_in_bytes,
            _update_scratch_size: d.update_scratch_data_size_in_bytes,
        })),
    );
    NRI_RESULT_SUCCESS
}

// ---------------------------------------------------------------------------
// Function tables
// ---------------------------------------------------------------------------

/// `nri::CoreInterface` (`0x350` bytes = 106 function pointers).
pub(super) static CORE_INTERFACE: InterfaceTable<106> = InterfaceTable([
    // Get (0..6)
    crate::Ptr(core_get_device_desc as *const ()),                // 00 (+0x000) GetDeviceDesc
    crate::Ptr(core_get_format_support as *const ()),             // 01 (+0x008) GetFormatSupport
    crate::Ptr(core_get_query_size as *const ()),                 // 02 (+0x010) GetQuerySize
    crate::Ptr(core_get_buffer_memory_info as *const ()),         // 03 (+0x018) GetBufferMemoryInfo
    crate::Ptr(core_get_texture_memory_info as *const ()),        // 04 (+0x020) GetTextureMemoryInfo
    crate::Ptr(core_get_command_queue as *const ()),              // 05 (+0x028) GetCommandQueue
    // Create (6..22)
    crate::Ptr(core_create_command_allocator as *const ()),       // 06 (+0x030) CreateCommandAllocator
    crate::Ptr(core_create_command_buffer as *const ()),          // 07 (+0x038) CreateCommandBuffer
    crate::Ptr(core_create_descriptor_pool as *const ()),         // 08 (+0x040) CreateDescriptorPool
    crate::Ptr(core_create_buffer as *const ()),                  // 09 (+0x048) CreateBuffer
    crate::Ptr(core_create_texture as *const ()),                 // 10 (+0x050) CreateTexture
    crate::Ptr(core_create_buffer_view as *const ()),             // 11 (+0x058) CreateBufferView
    crate::Ptr(core_create_texture_1d_view as *const ()),         // 12 (+0x060) CreateTexture1DView
    crate::Ptr(core_create_texture_2d_view as *const ()),         // 13 (+0x068) CreateTexture2DView
    crate::Ptr(core_create_texture_3d_view as *const ()),         // 14 (+0x070) CreateTexture3DView
    crate::Ptr(core_create_sampler as *const ()),                 // 15 (+0x078) CreateSampler
    crate::Ptr(core_create_pipeline_layout as *const ()),         // 16 (+0x080) CreatePipelineLayout
    crate::Ptr(core_create_graphics_pipeline as *const ()),       // 17 (+0x088) CreateGraphicsPipeline
    crate::Ptr(core_create_compute_pipeline as *const ()),        // 18 (+0x090) CreateComputePipeline
    crate::Ptr(core_create_frame_buffer as *const ()),            // 19 (+0x098) CreateFrameBuffer
    crate::Ptr(core_create_query_pool as *const ()),              // 20 (+0x0a0) CreateQueryPool
    crate::Ptr(core_create_fence as *const ()),                   // 21 (+0x0a8) CreateFence
    // Destroy (22..33)
    crate::Ptr(core_destroy_command_allocator as *const ()),      // 22 (+0x0b0) DestroyCommandAllocator
    crate::Ptr(core_destroy_command_buffer as *const ()),         // 23 (+0x0b8) DestroyCommandBuffer
    crate::Ptr(core_destroy_descriptor_pool as *const ()),        // 24 (+0x0c0) DestroyDescriptorPool
    crate::Ptr(core_destroy_buffer as *const ()),                 // 25 (+0x0c8) DestroyBuffer
    crate::Ptr(core_destroy_texture as *const ()),                // 26 (+0x0d0) DestroyTexture
    crate::Ptr(core_destroy_descriptor as *const ()),             // 27 (+0x0d8) DestroyDescriptor
    crate::Ptr(core_destroy_pipeline_layout as *const ()),        // 28 (+0x0e0) DestroyPipelineLayout
    crate::Ptr(core_destroy_pipeline as *const ()),               // 29 (+0x0e8) DestroyPipeline
    crate::Ptr(core_destroy_frame_buffer as *const ()),           // 30 (+0x0f0) DestroyFrameBuffer
    crate::Ptr(core_destroy_query_pool as *const ()),             // 31 (+0x0f8) DestroyQueryPool
    crate::Ptr(core_destroy_fence as *const ()),                  // 32 (+0x100) DestroyFence
    // Memory (33..37)
    crate::Ptr(core_allocate_memory as *const ()),                // 33 (+0x108) AllocateMemory
    crate::Ptr(core_bind_buffer_memory as *const ()),             // 34 (+0x110) BindBufferMemory
    crate::Ptr(core_bind_texture_memory as *const ()),            // 35 (+0x118) BindTextureMemory
    crate::Ptr(core_free_memory as *const ()),                    // 36 (+0x120) FreeMemory
    // Command buffer (37..73)
    crate::Ptr(core_begin_command_buffer as *const ()),           // 37 (+0x128) BeginCommandBuffer
    crate::Ptr(core_end_command_buffer as *const ()),             // 38 (+0x130) EndCommandBuffer
    crate::Ptr(core_cmd_set_pipeline as *const ()),               // 39 (+0x138) CmdSetPipeline
    crate::Ptr(core_cmd_set_pipeline_layout as *const ()),        // 40 (+0x140) CmdSetPipelineLayout
    crate::Ptr(core_cmd_set_descriptor_set as *const ()),         // 41 (+0x148) CmdSetDescriptorSet
    crate::Ptr(core_cmd_set_constants as *const ()),              // 42 (+0x150) CmdSetConstants
    crate::Ptr(core_cmd_set_descriptor_pool as *const ()),        // 43 (+0x158) CmdSetDescriptorPool
    crate::Ptr(core_cmd_pipeline_barrier as *const ()),           // 44 (+0x160) CmdPipelineBarrier
    crate::Ptr(core_cmd_begin_render_pass as *const ()),          // 45 (+0x168) CmdBeginRenderPass
    crate::Ptr(core_cmd_end_render_pass as *const ()),            // 46 (+0x170) CmdEndRenderPass
    crate::Ptr(core_cmd_set_viewports as *const ()),              // 47 (+0x178) CmdSetViewports
    crate::Ptr(core_cmd_set_scissors as *const ()),               // 48 (+0x180) CmdSetScissors
    crate::Ptr(CMD_SET_DEPTH_BOUNDS_ENTRY),                       // 49 (+0x188) CmdSetDepthBounds
    crate::Ptr(core_cmd_set_stencil_reference as *const ()),      // 50 (+0x190) CmdSetStencilReference
    crate::Ptr(core_cmd_set_sample_positions as *const ()),       // 51 (+0x198) CmdSetSamplePositions
    crate::Ptr(core_cmd_clear_attachments as *const ()),          // 52 (+0x1a0) CmdClearAttachments
    crate::Ptr(core_cmd_set_index_buffer as *const ()),           // 53 (+0x1a8) CmdSetIndexBuffer
    crate::Ptr(core_cmd_set_vertex_buffers as *const ()),         // 54 (+0x1b0) CmdSetVertexBuffers
    crate::Ptr(core_cmd_draw as *const ()),                       // 55 (+0x1b8) CmdDraw
    crate::Ptr(core_cmd_draw_indexed as *const ()),               // 56 (+0x1c0) CmdDrawIndexed
    crate::Ptr(core_cmd_draw_indirect as *const ()),              // 57 (+0x1c8) CmdDrawIndirect
    crate::Ptr(core_cmd_draw_indexed_indirect as *const ()),      // 58 (+0x1d0) CmdDrawIndexedIndirect
    crate::Ptr(core_cmd_dispatch as *const ()),                   // 59 (+0x1d8) CmdDispatch
    crate::Ptr(core_cmd_dispatch_indirect as *const ()),          // 60 (+0x1e0) CmdDispatchIndirect
    crate::Ptr(core_cmd_begin_query as *const ()),                // 61 (+0x1e8) CmdBeginQuery
    crate::Ptr(core_cmd_end_query as *const ()),                  // 62 (+0x1f0) CmdEndQuery
    crate::Ptr(core_cmd_begin_annotation as *const ()),           // 63 (+0x1f8) CmdBeginAnnotation
    crate::Ptr(core_cmd_end_annotation as *const ()),             // 64 (+0x200) CmdEndAnnotation
    crate::Ptr(core_cmd_clear_storage_buffer as *const ()),       // 65 (+0x208) CmdClearStorageBuffer
    crate::Ptr(core_cmd_clear_storage_texture as *const ()),      // 66 (+0x210) CmdClearStorageTexture
    crate::Ptr(core_cmd_copy_buffer as *const ()),                // 67 (+0x218) CmdCopyBuffer
    crate::Ptr(core_cmd_copy_texture as *const ()),               // 68 (+0x220) CmdCopyTexture
    crate::Ptr(core_cmd_upload_buffer_to_texture as *const ()),   // 69 (+0x228) CmdUploadBufferToTexture
    crate::Ptr(core_cmd_readback_texture_to_buffer as *const ()), // 70 (+0x230) CmdReadbackTextureToBuffer
    crate::Ptr(core_cmd_copy_queries as *const ()),               // 71 (+0x238) CmdCopyQueries
    crate::Ptr(core_cmd_reset_queries as *const ()),              // 72 (+0x240) CmdResetQueries
    // Synchronization (73..77)
    crate::Ptr(core_get_fence_value as *const ()),                // 73 (+0x248) GetFenceValue
    crate::Ptr(core_queue_signal as *const ()),                   // 74 (+0x250) QueueSignal
    crate::Ptr(core_queue_wait as *const ()),                     // 75 (+0x258) QueueWait
    crate::Ptr(core_wait as *const ()),                           // 76 (+0x260) Wait
    // Work submission (77)
    crate::Ptr(core_queue_submit as *const ()),                   // 77 (+0x268) QueueSubmit
    // Descriptor set (78..81)
    crate::Ptr(core_update_descriptor_ranges as *const ()),       // 78 (+0x270) UpdateDescriptorRanges
    crate::Ptr(core_update_dynamic_constant_buffers as *const ()),// 79 (+0x278) UpdateDynamicConstantBuffers
    crate::Ptr(core_copy_descriptor_set as *const ()),            // 80 (+0x280) CopyDescriptorSet
    // Descriptor pool (81..83)
    crate::Ptr(core_allocate_descriptor_sets as *const ()),       // 81 (+0x288) AllocateDescriptorSets
    crate::Ptr(core_reset_descriptor_pool as *const ()),          // 82 (+0x290) ResetDescriptorPool
    // Command allocator (83)
    crate::Ptr(core_reset_command_allocator as *const ()),        // 83 (+0x298) ResetCommandAllocator
    // Map / Unmap (84..86)
    crate::Ptr(core_map_buffer as *const ()),                     // 84 (+0x2a0) MapBuffer
    crate::Ptr(core_unmap_buffer as *const ()),                   // 85 (+0x2a8) UnmapBuffer
    // Debug name (86..101)
    crate::Ptr(core_set_device_name as *const ()),                // 86 (+0x2b0) SetDeviceDebugName
    crate::Ptr(core_set_queue_name as *const ()),                 // 87 (+0x2b8) SetCommandQueueDebugName
    crate::Ptr(core_set_fence_name as *const ()),                 // 88 (+0x2c0) SetFenceDebugName
    crate::Ptr(core_set_allocator_name as *const ()),             // 89 (+0x2c8) SetCommandAllocatorDebugName
    crate::Ptr(core_set_pool_name as *const ()),                  // 90 (+0x2d0) SetDescriptorPoolDebugName
    crate::Ptr(core_set_buffer_name as *const ()),                // 91 (+0x2d8) SetBufferDebugName
    crate::Ptr(core_set_texture_name as *const ()),               // 92 (+0x2e0) SetTextureDebugName
    crate::Ptr(core_set_name_noop as *const ()),                  // 93 (+0x2e8) SetDescriptorDebugName
    crate::Ptr(core_set_pipeline_layout_name as *const ()),       // 94 (+0x2f0) SetPipelineLayoutDebugName
    crate::Ptr(core_set_pipeline_name as *const ()),              // 95 (+0x2f8) SetPipelineDebugName
    crate::Ptr(core_set_name_noop as *const ()),                  // 96 (+0x300) SetFrameBufferDebugName
    crate::Ptr(core_set_query_pool_name as *const ()),            // 97 (+0x308) SetQueryPoolDebugName
    crate::Ptr(core_set_name_noop as *const ()),                  // 98 (+0x310) SetDescriptorSetDebugName
    crate::Ptr(core_set_cmdbuf_name as *const ()),                // 99 (+0x318) SetCommandBufferDebugName
    crate::Ptr(core_set_memory_name as *const ()),                // 100 (+0x320) SetMemoryDebugName
    // Native objects (101..106)
    crate::Ptr(core_get_device_native_object as *const ()),       // 101 (+0x328) GetDeviceNativeObject
    crate::Ptr(core_get_cmdbuf_native_object as *const ()),       // 102 (+0x330) GetCommandBufferNativeObject
    crate::Ptr(core_get_buffer_native_object as *const ()),       // 103 (+0x338) GetBufferNativeObject
    crate::Ptr(core_get_texture_native_object as *const ()),      // 104 (+0x340) GetTextureNativeObject
    crate::Ptr(core_get_descriptor_native_object as *const ()),   // 105 (+0x348) GetDescriptorNativeObject
]);

const _: () = assert!(size_of::<InterfaceTable<106>>() == 0x350);

/// `nri::HelperInterface` (`0x28` bytes = 5 function pointers).
pub(super) static HELPER_INTERFACE: InterfaceTable<5> = InterfaceTable([
    crate::Ptr(helper_calculate_allocation_number as *const ()), // 0 (+0x00) CalculateAllocationNumber
    crate::Ptr(helper_allocate_and_bind_memory as *const ()),    // 1 (+0x08) AllocateAndBindMemory
    crate::Ptr(helper_change_resource_states as *const ()),      // 2 (+0x10) ChangeResourceStates
    crate::Ptr(helper_upload_data as *const ()),                 // 3 (+0x18) UploadData
    crate::Ptr(helper_wait_for_idle as *const ()),               // 4 (+0x20) WaitForIdle
]);

/// `nri::WrapperD3D12Interface` (`0x28` bytes = 5 function pointers).
pub(super) static WRAPPER_D3D12_INTERFACE: InterfaceTable<5> = InterfaceTable([
    crate::Ptr(wrapper_create_command_buffer_d3d12 as *const ()), // 0 (+0x00) CreateCommandBufferD3D12
    crate::Ptr(wrapper_create_buffer_d3d12 as *const ()),         // 1 (+0x08) CreateBufferD3D12
    crate::Ptr(wrapper_create_texture_d3d12 as *const ()),        // 2 (+0x10) CreateTextureD3D12
    crate::Ptr(wrapper_create_memory_d3d12 as *const ()),         // 3 (+0x18) CreateMemoryD3D12
    crate::Ptr(wrapper_create_acceleration_structure_d3d12 as *const ()), // 4 (+0x20) CreateAccelerationStructureD3D12
]);

pub(super) fn ensure_interfaces_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        register_static_ptr_slice(&CORE_INTERFACE.0);
        register_static_ptr_slice(&HELPER_INTERFACE.0);
        register_static_ptr_slice(&WRAPPER_D3D12_INTERFACE.0);
    });
}
