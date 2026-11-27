//! Shader-visible descriptors: D3D12 SRV/UAV/CBV/sampler descriptions →
//! 24-byte `IRDescriptorTableEntry`s (`dxil_metal::runtime`).
//!
//! Texture views and samplers produce Metal `MTLResourceID`s that go into the
//! entry; non-identity texture and typed-buffer views are allocated from the
//! Metal 4 [`super::view_pool`] (cached on their resource, recycled after GPU
//! completion), while samplers live in a process-wide table.

use super::resource::{Resource, SWIZZLE_IDENTITY};
use crate::format;
use dxil_metal::runtime::{BufferView, DescriptorTableEntry};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub type Entry = [u8; 24];

// ---------------------------------------------------------------------------
// Samplers
// ---------------------------------------------------------------------------

/// `D3D12_SAMPLER_DESC` (52 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerDesc {
    pub filter: u32,
    pub address: [u32; 3],
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison: u32,
    pub border: [f32; 4],
    pub min_lod: f32,
    pub max_lod: f32,
}
const _: () = assert!(std::mem::size_of::<SamplerDesc>() == 52);

struct SamplerCache(Mutex<HashMap<[u32; 13], Retained<ProtocolObject<dyn MTLSamplerState>>>>);
unsafe impl Send for SamplerCache {}
unsafe impl Sync for SamplerCache {}

fn address_mode(a: u32) -> MTLSamplerAddressMode {
    match a {
        2 => MTLSamplerAddressMode::MirrorRepeat,
        3 => MTLSamplerAddressMode::ClampToEdge,
        4 => MTLSamplerAddressMode::ClampToBorderColor,
        5 => MTLSamplerAddressMode::MirrorClampToEdge,
        _ => MTLSamplerAddressMode::Repeat,
    }
}

/// Metal has three fixed border colors; pick the closest.
fn border_color(c: [f32; 4]) -> MTLSamplerBorderColor {
    if c[3] < 0.5 {
        MTLSamplerBorderColor::TransparentBlack
    } else if c[0] + c[1] + c[2] > 1.5 {
        MTLSamplerBorderColor::OpaqueWhite
    } else {
        MTLSamplerBorderColor::OpaqueBlack
    }
}

/// D3D12_COMPARISON_FUNC (1..=8) → MTLCompareFunction (0..=7).
pub fn compare_function(f: u32) -> MTLCompareFunction {
    MTLCompareFunction(f.clamp(1, 8) as usize - 1)
}

/// The sampler state for `d` (cached; never freed: descriptor heaps may
/// reference it at any time).
pub fn sampler_state(d: &SamplerDesc) -> Result<Retained<ProtocolObject<dyn MTLSamplerState>>, String> {
    static C: OnceLock<SamplerCache> = OnceLock::new();
    let key: [u32; 13] = unsafe { std::mem::transmute(*d) };
    let cache = C.get_or_init(|| SamplerCache(Mutex::new(HashMap::new())));
    let mut m = cache.0.lock().unwrap();
    if let Some(s) = m.get(&key) {
        return Ok(s.clone());
    }
    let dev = super::device().ok_or("no device")?;
    let sd = MTLSamplerDescriptor::new();
    let f = d.filter;
    let lin = |b: u32| if b & 1 != 0 { MTLSamplerMinMagFilter::Linear } else { MTLSamplerMinMagFilter::Nearest };
    sd.setMinFilter(lin(f >> 4));
    sd.setMagFilter(lin(f >> 2));
    sd.setMipFilter(if f & 1 != 0 { MTLSamplerMipFilter::Linear } else { MTLSamplerMipFilter::Nearest });
    if f & 0x40 != 0 {
        sd.setMaxAnisotropy(d.max_anisotropy.clamp(1, 16) as usize);
    }
    match (f >> 7) & 3 {
        1 => sd.setCompareFunction(compare_function(d.comparison)),
        2 => sd.setReductionMode(MTLSamplerReductionMode::Minimum),
        3 => sd.setReductionMode(MTLSamplerReductionMode::Maximum),
        _ => {}
    }
    sd.setSAddressMode(address_mode(d.address[0]));
    sd.setTAddressMode(address_mode(d.address[1]));
    sd.setRAddressMode(address_mode(d.address[2]));
    sd.setBorderColor(border_color(d.border));
    sd.setLodMinClamp(d.min_lod.max(0.0));
    sd.setLodMaxClamp(if d.max_lod.is_finite() { d.max_lod.max(d.min_lod.max(0.0)) } else { f32::MAX });
    sd.setSupportArgumentBuffers(true);
    let s = dev.dev.newSamplerStateWithDescriptor(&sd).ok_or_else(|| format!("newSamplerState({d:?})"))?;
    m.insert(key, s.clone());
    Ok(s)
}

pub fn sampler_entry(d: &SamplerDesc) -> Result<Entry, String> {
    let s = sampler_state(d)?;
    let id: u64 = unsafe { std::mem::transmute::<MTLResourceID, u64>(s.gpuResourceID()) };
    Ok(DescriptorTableEntry::sampler(id, d.mip_lod_bias).to_bytes())
}

// ---------------------------------------------------------------------------
// CBV
// ---------------------------------------------------------------------------

/// `D3D12_CONSTANT_BUFFER_VIEW_DESC`: BufferLocation, SizeInBytes.
pub fn cbv_entry(va: u64, size: u32) -> Entry {
    DescriptorTableEntry::buffer(va, dxil_metal::runtime::buffer_metadata(size as u64, 0, false)).to_bytes()
}

// ---------------------------------------------------------------------------
// SRV / UAV
// ---------------------------------------------------------------------------

/// D3D12 `Shader4ComponentMapping` → packed Metal swizzle, also folding the
/// stencil-in-G convention of `X24_TYPELESS_G8_UINT`/`X32_TYPELESS_G8X24_UINT`
/// (Metal stencil views return stencil in R).
fn swizzle(mapping: u32, dxgi: u32) -> u32 {
    let stencil_in_g = matches!(dxgi, 47 | 22);
    let bgrx = matches!(dxgi, 88 | 93); // B8G8R8X8: alpha reads as 1
    let mut out = 0u32;
    for c in 0..4 {
        let m = (mapping >> (3 * c)) & 7;
        let s = match m {
            0..=3 => {
                let ch = if stencil_in_g {
                    if m == 1 { 0 } else { 4 } // G → stencil (Metal R), others 0
                } else if bgrx && m == 3 {
                    5
                } else {
                    m
                };
                match ch {
                    4 => 0u32, // Zero
                    5 => 1,    // One
                    ch => 2 + ch,
                }
            }
            4 => 0, // FORCE_VALUE_0
            _ => 1, // FORCE_VALUE_1
        };
        out |= s << (8 * c);
    }
    out
}

const DEFAULT_MAPPING: u32 = 0x1688;

fn mip_range(r: &Resource, most_detailed: u32, levels: u32) -> (u32, u32) {
    let total = r.desc.mip_levels();
    let first = most_detailed.min(total.saturating_sub(1));
    let n = if levels == u32::MAX || levels == 0 { total - first } else { levels.min(total - first) };
    (first, n)
}

fn texture_entry(id: Option<u64>, min_lod: f32) -> Result<Entry, String> {
    let id = id.ok_or("texture view creation failed")?;
    Ok(DescriptorTableEntry::texture(id, min_lod, 0).to_bytes())
}

/// Buffer SRV/UAV (`D3D12_BUFFER_SRV`/`_UAV`): raw (flag 1), structured
/// (stride) or typed (format, via a texture-buffer view). When `counter` is
/// supplied on a writable raw/structured buffer UAV, its `R32Uint` texture-buffer
/// view is attached in `texture_buffer_view_id` (matching `IRRuntimeCreateAppendBufferView`)
/// so append/consume and `IncrementCounter`/`DecrementCounter` instructions hit the GPU counter.
fn buffer_entry(
    r: &Resource,
    dxgi: u32,
    first: u64,
    num: u32,
    stride: u32,
    flags: u32,
    writable: bool,
    counter: Option<(&Resource, u64)>,
) -> Result<Entry, String> {
    let raw = flags & 1 != 0;
    let base = r.gpu_va;
    let len = r.desc.Width;
    if raw || stride != 0 {
        let elem = if raw { 4 } else { stride as u64 };
        let off = first * elem;
        let size = (num as u64 * elem).min(len.saturating_sub(off));
        if writable {
            if let Some((cr, coff)) = counter {
                cr.register_ptr();
                let dev = super::device().ok_or("no device")?;
                let align = dev.dev.minimumTextureBufferAlignmentForPixelFormat(MTLPixelFormat::R32Uint).max(16) as u64;
                let aligned = coff / align * align;
                let el_off = (coff - aligned) / 4;
                if el_off > 255 {
                    return Err(format!("UAV counter element offset {el_off} > 255"));
                }
                let tv_id = cr
                    .buffer_texture_view_id(MTLPixelFormat::R32Uint, aligned, 1 + el_off, true)
                    .ok_or("UAV counter texture buffer view creation failed")?;
                let v = BufferView {
                    gpu_va: base + off,
                    size,
                    texture_buffer_view_id: tv_id,
                    texture_view_offset_in_elements: el_off as u32,
                    typed_buffer: false,
                };
                return Ok(DescriptorTableEntry::buffer_view(&v).to_bytes());
            }
        }
        return Ok(DescriptorTableEntry::buffer(base + off, dxil_metal::runtime::buffer_metadata(size, 0, false)).to_bytes());
    }
    let fi = format::info(dxgi);
    if fi.mtl == MTLPixelFormat::Invalid || fi.bytes == 0 {
        return Err(format!("typed buffer view with format {}", format::name(dxgi)));
    }
    let bpe = fi.bytes as u64;
    let dev = super::device().ok_or("no device")?;
    let align = dev.dev.minimumTextureBufferAlignmentForPixelFormat(fi.mtl).max(1) as u64;
    let off = first * bpe;
    let aligned = off / align * align;
    let el_off = (off - aligned) / bpe;
    if el_off > 255 {
        return Err(format!("typed buffer view element offset {el_off} > 255"));
    }
    let tv_id = r.buffer_texture_view_id(fi.mtl, aligned, num as u64 + el_off, writable).ok_or("texture buffer view creation failed")?;
    r.note_view_kind(super::resource::HK_TYPED_BUF);
    let v = BufferView {
        gpu_va: base + aligned,
        size: (num as u64 + el_off) * bpe,
        texture_buffer_view_id: tv_id,
        texture_view_offset_in_elements: el_off as u32,
        typed_buffer: true,
    };
    Ok(DescriptorTableEntry::buffer_view(&v).to_bytes())
}

fn u32s(d: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(d[i * 4..i * 4 + 4].try_into().unwrap())
}
fn f32s(d: &[u8], i: usize) -> f32 {
    f32::from_bits(u32s(d, i))
}
fn u64at(d: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(d[off..off + 8].try_into().unwrap())
}

/// `CreateShaderResourceView(resource, desc)`; `desc` = the 40-byte
/// `D3D12_SHADER_RESOURCE_VIEW_DESC` or `None` (whole resource).
pub fn srv_entry(r: Option<&Resource>, desc: Option<&[u8; 40]>) -> Result<Entry, String> {
    let Some(r) = r else {
        // NULL resource: null descriptor (acceleration structures excepted).
        if let Some(d) = desc {
            if u32s(d, 1) == 11 {
                return Ok(DescriptorTableEntry::buffer(u64at(d, 16), 0).to_bytes());
            }
        }
        return Ok(DescriptorTableEntry::NULL.to_bytes());
    };
    r.register_ptr();
    let Some(d) = desc else {
        if r.desc.is_buffer() {
            return Err("SRV of a buffer without a desc".into());
        }
        let t = r.texture().ok_or("no texture")?;
        let fmt = r.mtl_format;
        let ty = t.textureType();
        let view = r.texture_view_id(fmt, ty, (0, r.desc.mip_levels()), (0, r.desc.array_size()));
        return texture_entry(view, 0.0);
    };
    let dxgi = u32s(d, 0);
    let dimension = u32s(d, 1);
    let mapping = u32s(d, 2);
    let u = &d[16..];
    if dimension == 1 {
        return buffer_entry(r, dxgi, u64at(u, 0), u32s(u, 2), u32s(u, 3), u32s(u, 4), false, None);
    }
    if r.texture().is_none() {
        return Err(format!("texture SRV (dimension {dimension}) of a buffer"));
    }
    let fmt = format::view_format(dxgi, r.mtl_format);
    let swz = if mapping == DEFAULT_MAPPING && !matches!(dxgi, 47 | 22 | 88 | 93) { SWIZZLE_IDENTITY } else { swizzle(mapping, dxgi) };
    use MTLTextureType as T;
    let (ty, mips, slices, min_lod) = match dimension {
        2 => (T::Type1D, mip_range(r, u32s(u, 0), u32s(u, 1)), (0, 1), f32s(u, 2)),
        3 => (T::Type1DArray, mip_range(r, u32s(u, 0), u32s(u, 1)), (u32s(u, 2), u32s(u, 3)), f32s(u, 4)),
        4 => (T::Type2D, mip_range(r, u32s(u, 0), u32s(u, 1)), (0, 1), f32s(u, 3)),
        5 => (T::Type2DArray, mip_range(r, u32s(u, 0), u32s(u, 1)), (u32s(u, 2), u32s(u, 3)), f32s(u, 5)),
        6 => (T::Type2DMultisample, (0, 1), (0, 1), 0.0),
        7 => (T::Type2DMultisampleArray, (0, 1), (u32s(u, 0), u32s(u, 1)), 0.0),
        8 => (T::Type3D, mip_range(r, u32s(u, 0), u32s(u, 1)), (0, 1), f32s(u, 2)),
        9 => (T::TypeCube, mip_range(r, u32s(u, 0), u32s(u, 1)), (0, 6), f32s(u, 2)),
        10 => (T::TypeCubeArray, mip_range(r, u32s(u, 0), u32s(u, 1)), (u32s(u, 2), u32s(u, 3) * 6), f32s(u, 4)),
        _ => return Err(format!("SRV view dimension {dimension}")),
    };
    r.note_view_kind(super::resource::view_tex_handle_kind_mask(dxgi, r.desc.Format, ty));
    // Single-slice 2D views of an array texture keep their slice.
    let view = r.texture_view_id_swizzled(fmt, ty, mips, slices, swz);
    texture_entry(view, min_lod)
}

/// `CreateUnorderedAccessView(resource, counter, desc)`; `desc` = the
/// 40-byte `D3D12_UNORDERED_ACCESS_VIEW_DESC` or `None`.
pub fn uav_entry(r: Option<&Resource>, desc: Option<&[u8; 40]>) -> Result<Entry, String> {
    uav_entry_with_counter(r, None, desc)
}

/// [`uav_entry`] with an optional `(counter_buffer, counter_byte_offset)` for
/// append/consume and structured-counter UAVs.
pub fn uav_entry_with_counter(
    r: Option<&Resource>,
    counter: Option<(&Resource, u64)>,
    desc: Option<&[u8; 40]>,
) -> Result<Entry, String> {
    let Some(r) = r else { return Ok(DescriptorTableEntry::NULL.to_bytes()) };
    r.register_ptr();
    let Some(d) = desc else {
        if r.desc.is_buffer() {
            return Err("UAV of a buffer without a desc".into());
        }
        let t = r.texture().ok_or("no texture")?;
        let view = r.texture_view_id(r.mtl_format, t.textureType(), (0, 1), (0, r.desc.array_size()));
        return texture_entry(view, 0.0);
    };
    let dxgi = u32s(d, 0);
    let dimension = u32s(d, 1);
    let u = &d[8..];
    if dimension == 1 {
        // FirstElement, NumElements, StructureByteStride, CounterOffset, Flags
        let counter_arg = counter.map(|(cr, explicit_off)| {
            let desc_off = u64at(u, 16);
            (cr, if explicit_off != 0 { explicit_off } else { desc_off })
        });
        return buffer_entry(r, dxgi, u64at(u, 0), u32s(u, 2), u32s(u, 3), u32s(u, 6), true, counter_arg);
    }
    if r.texture().is_none() {
        return Err(format!("texture UAV (dimension {dimension}) of a buffer"));
    }
    let fmt = format::view_format(dxgi, r.mtl_format);
    use MTLTextureType as T;
    let (ty, mip, slices) = match dimension {
        2 => (T::Type1D, u32s(u, 0), (0, 1)),
        3 => (T::Type1DArray, u32s(u, 0), (u32s(u, 1), u32s(u, 2))),
        4 => (T::Type2D, u32s(u, 0), (0, 1)),
        5 => (T::Type2DArray, u32s(u, 0), (u32s(u, 1), u32s(u, 2))),
        6 => (T::Type2DMultisample, 0, (0, 1)),
        7 => (T::Type2DMultisampleArray, 0, (u32s(u, 0), u32s(u, 1))),
        // Metal views cannot select a W range of a 3D texture: whole mip.
        8 => (T::Type3D, u32s(u, 0), (0, 1)),
        _ => return Err(format!("UAV view dimension {dimension}")),
    };
    r.note_view_kind(super::resource::view_tex_handle_kind_mask(dxgi, r.desc.Format, ty));
    let view = r.texture_view_id(fmt, ty, (mip, 1), slices);
    texture_entry(view, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swizzles() {
        assert_eq!(swizzle(DEFAULT_MAPPING, 28), SWIZZLE_IDENTITY);
        // B8G8R8X8: alpha forced to one.
        assert_eq!(swizzle(DEFAULT_MAPPING, 88), 0x0104_0302);
        // Stencil SRV (X24_TYPELESS_G8_UINT) reads stencil from G.
        assert_eq!(swizzle(DEFAULT_MAPPING, 47), 0x0000_0200);
        // FORCE_VALUE_1 in all channels: 5 | 5<<3 | 5<<6 | 5<<9.
        assert_eq!(swizzle(0x0B6D, 28), 0x0101_0101);
    }

    #[test]
    fn compare_functions() {
        assert_eq!(compare_function(1), MTLCompareFunction::Never);
        assert_eq!(compare_function(4), MTLCompareFunction::LessEqual);
        assert_eq!(compare_function(8), MTLCompareFunction::Always);
    }

    #[test]
    fn acceleration_structure_srv() {
        let mut d = [0u8; 40];
        d[4..8].copy_from_slice(&11u32.to_le_bytes()); // D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE
        d[8..12].copy_from_slice(&DEFAULT_MAPPING.to_le_bytes());
        d[16..24].copy_from_slice(&0x1234_5600u64.to_le_bytes());
        let e = srv_entry(None, Some(&d)).unwrap();
        assert_eq!(u64::from_le_bytes(e[0..8].try_into().unwrap()), 0x1234_5600);
        assert_eq!(&e[8..24], &[0u8; 16]);
    }

    #[test]
    fn structured_uav_with_counter_encodes_buffer_view() {
        if super::super::device().is_none() {
            return;
        }
        use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};
        let bdesc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 256,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let data = Resource::new_committed_default(&bdesc, &hp, "uav data").unwrap();
        let ctr = Resource::new_committed_default(&bdesc, &hp, "uav counter").unwrap();
        let mut desc = [0u8; 40];
        desc[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
        desc[8..16].copy_from_slice(&0u64.to_le_bytes()); // FirstElement
        desc[16..20].copy_from_slice(&16u32.to_le_bytes()); // NumElements
        desc[20..24].copy_from_slice(&16u32.to_le_bytes()); // StructureByteStride
        let e = uav_entry_with_counter(Some(&data), Some((&ctr, 0)), Some(&desc)).unwrap();
        let gpu_va = u64::from_le_bytes(e[0..8].try_into().unwrap());
        let tex_id = u64::from_le_bytes(e[8..16].try_into().unwrap());
        let meta = u64::from_le_bytes(e[16..24].try_into().unwrap());
        assert_eq!(gpu_va, data.gpu_va);
        assert_ne!(tex_id, 0, "counter texture buffer view resource ID must be populated");
        assert_eq!(meta & 0xffff_ffff, 256);
        assert_eq!(meta >> 63, 0, "structured buffer with counter must NOT set typed_buffer bit");
    }
}
