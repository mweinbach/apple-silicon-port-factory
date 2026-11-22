//! Decoding raw Windows-layout structs from trace blobs.
//!
//! The replayer keeps its own copies of the few small D3D12 layouts it reads, so it doesn't
//! depend on the still-changing `d3d12_metal::types`.

#![allow(non_snake_case)]

use d3d12_metal::state::{
    BlendDesc, DepthStencilDesc, DepthStencilDesc1, DepthStencilDesc2, RasterizerDesc, RasterizerDesc1, RasterizerDesc2,
};

/// Plain-old-data: `#[repr(C)]`, `Copy`, and every bit pattern is a valid value (integers and
/// floats only, no bools, enums, references or pointers).
///
/// # Safety
/// Implement only for types that meet the conditions above.
pub unsafe trait Pod: Copy {}

unsafe impl Pod for BlendDesc {}
unsafe impl Pod for RasterizerDesc {}
unsafe impl Pod for RasterizerDesc1 {}
unsafe impl Pod for RasterizerDesc2 {}
unsafe impl Pod for DepthStencilDesc {}
unsafe impl Pod for DepthStencilDesc1 {}
unsafe impl Pod for DepthStencilDesc2 {}
unsafe impl Pod for ResourceDesc {}
unsafe impl Pod for DescriptorHeapDesc {}
unsafe impl Pod for VertexBufferView {}
unsafe impl Pod for IndexBufferView {}
unsafe impl Pod for Viewport {}
unsafe impl Pod for Rect {}

/// Reads a `T` from a blob of exactly `size_of::<T>()` bytes.
pub fn read<T: Pod>(b: &[u8]) -> Result<T, String> {
    let want = std::mem::size_of::<T>();
    if b.len() != want {
        return Err(format!("{}: {} bytes, expected {want}", short_type_name::<T>(), b.len()));
    }
    // SAFETY: the length matches and `T: Pod` accepts any bit pattern; the read is unaligned.
    Ok(unsafe { std::ptr::read_unaligned(b.as_ptr().cast::<T>()) })
}

/// Reads a `T` from a blob that is at least `size_of::<T>()` bytes long (extended structs, e.g.
/// `D3D12_RESOURCE_DESC1`, which starts with a `D3D12_RESOURCE_DESC`).
pub fn read_prefix<T: Pod>(b: &[u8]) -> Result<T, String> {
    let want = std::mem::size_of::<T>();
    let head = b.get(..want).ok_or_else(|| format!("{}: {} bytes, expected at least {want}", short_type_name::<T>(), b.len()))?;
    read(head)
}

/// Reads an array of `T` from a blob whose length is a multiple of `size_of::<T>()`.
pub fn read_array<T: Pod>(b: &[u8]) -> Result<Vec<T>, String> {
    let n = std::mem::size_of::<T>();
    if !b.len().is_multiple_of(n) {
        return Err(format!("{}[]: {} bytes is not a multiple of {n}", short_type_name::<T>(), b.len()));
    }
    b.chunks_exact(n).map(read).collect()
}

fn short_type_name<T>() -> &'static str {
    let n = std::any::type_name::<T>();
    n.rsplit("::").next().unwrap_or(n)
}

/// `D3D12_RESOURCE_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResourceDesc {
    pub Dimension: u32,
    pub Alignment: u64,
    pub Width: u64,
    pub Height: u32,
    pub DepthOrArraySize: u16,
    pub MipLevels: u16,
    pub Format: u32,
    pub SampleCount: u32,
    pub SampleQuality: u32,
    pub Layout: u32,
    pub Flags: u32,
}
const _: () = assert!(std::mem::size_of::<ResourceDesc>() == 56);

impl ResourceDesc {
    pub fn dimension_name(&self) -> &'static str {
        match self.Dimension {
            1 => "buffer",
            2 => "tex1d",
            3 => "tex2d",
            4 => "tex3d",
            _ => "unknown",
        }
    }
}

/// `D3D12_DESCRIPTOR_HEAP_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DescriptorHeapDesc {
    pub Type: u32,
    pub NumDescriptors: u32,
    pub Flags: u32,
    pub NodeMask: u32,
}

/// `D3D12_VERTEX_BUFFER_VIEW`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VertexBufferView {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub StrideInBytes: u32,
}
const _: () = assert!(std::mem::size_of::<VertexBufferView>() == 16);

/// `D3D12_INDEX_BUFFER_VIEW`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IndexBufferView {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub Format: u32,
}
const _: () = assert!(std::mem::size_of::<IndexBufferView>() == 16);

/// `D3D12_VIEWPORT`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Viewport {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
const _: () = assert!(std::mem::size_of::<Viewport>() == 24);

/// `D3D12_RECT`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
const _: () = assert!(std::mem::size_of::<Rect>() == 16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_checks_length_and_is_unaligned() {
        let mut b = vec![0u8; 17];
        b[1..9].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
        b[9..13].copy_from_slice(&64u32.to_le_bytes());
        b[13..17].copy_from_slice(&57u32.to_le_bytes());
        let v: IndexBufferView = read(&b[1..]).unwrap();
        assert_eq!(v, IndexBufferView { BufferLocation: 0x1122_3344_5566_7788, SizeInBytes: 64, Format: 57 });
        assert!(read::<IndexBufferView>(&b).is_err());
        assert!(read_array::<Rect>(&b[..16]).unwrap().len() == 1);
        assert!(read_array::<Rect>(&b).is_err());
        assert!(read_prefix::<IndexBufferView>(&b[1..]).is_ok());
        assert!(read_prefix::<ResourceDesc>(&b).is_err());
    }
}
