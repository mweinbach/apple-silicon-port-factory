//! D3D12 structs with their Windows x64 layout (identical on arm64 macOS:
//! only fixed-width fields and pointers are used).

#![allow(non_snake_case)]

/// `D3D12_RESOURCE_DESC` (56 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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

pub mod dim {
    pub const UNKNOWN: u32 = 0;
    pub const BUFFER: u32 = 1;
    pub const TEXTURE1D: u32 = 2;
    pub const TEXTURE2D: u32 = 3;
    pub const TEXTURE3D: u32 = 4;
}

pub mod resource_flags {
    pub const ALLOW_RENDER_TARGET: u32 = 0x1;
    pub const ALLOW_DEPTH_STENCIL: u32 = 0x2;
    pub const ALLOW_UNORDERED_ACCESS: u32 = 0x4;
    pub const DENY_SHADER_RESOURCE: u32 = 0x8;
    pub const ALLOW_CROSS_ADAPTER: u32 = 0x10;
    pub const ALLOW_SIMULTANEOUS_ACCESS: u32 = 0x20;
    pub const RAYTRACING_ACCELERATION_STRUCTURE: u32 = 0x100;
}

impl ResourceDesc {
    pub fn is_buffer(&self) -> bool {
        self.Dimension == dim::BUFFER
    }
    pub fn array_size(&self) -> u32 {
        if self.Dimension == dim::TEXTURE3D {
            1
        } else {
            self.DepthOrArraySize.max(1) as u32
        }
    }
    pub fn depth(&self) -> u32 {
        if self.Dimension == dim::TEXTURE3D {
            self.DepthOrArraySize.max(1) as u32
        } else {
            1
        }
    }
    /// Mip count with 0 = full chain resolved.
    pub fn mip_levels(&self) -> u32 {
        if self.MipLevels != 0 {
            return self.MipLevels as u32;
        }
        let m = (self.Width.max(self.Height as u64)).max(self.depth() as u64).max(1);
        64 - m.leading_zeros()
    }
    pub fn subresource_count(&self) -> u32 {
        if self.is_buffer() {
            return 1;
        }
        self.mip_levels() * self.array_size() * crate::format::info(self.Format).planes.max(1)
    }
}

/// `D3D12_HEAP_PROPERTIES` (20 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HeapProperties {
    pub Type: u32,
    pub CPUPageProperty: u32,
    pub MemoryPoolPreference: u32,
    pub CreationNodeMask: u32,
    pub VisibleNodeMask: u32,
}

pub mod heap_type {
    pub const DEFAULT: u32 = 1;
    pub const UPLOAD: u32 = 2;
    pub const READBACK: u32 = 3;
    pub const CUSTOM: u32 = 4;
    pub const GPU_UPLOAD: u32 = 5;
}

/// CPU access of a heap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuAccess {
    None,
    WriteCombine,
    WriteBack,
}

impl HeapProperties {
    pub fn cpu_access(&self) -> CpuAccess {
        match self.Type {
            heap_type::UPLOAD | heap_type::GPU_UPLOAD => CpuAccess::WriteCombine,
            heap_type::READBACK => CpuAccess::WriteBack,
            heap_type::CUSTOM => match self.CPUPageProperty {
                2 => CpuAccess::WriteCombine, // WRITE_COMBINE
                3 => CpuAccess::WriteBack,    // WRITE_BACK
                _ => CpuAccess::None,
            },
            _ => CpuAccess::None,
        }
    }
    /// `GetCustomHeapProperties` view of a standard heap type (UMA, cache coherent).
    pub fn custom(&self) -> HeapProperties {
        let (cpu, pool) = match self.Type {
            heap_type::DEFAULT => (1, 1),            // NOT_AVAILABLE, L0
            heap_type::UPLOAD => (2, 1),             // WRITE_COMBINE
            heap_type::READBACK => (3, 1),           // WRITE_BACK
            heap_type::GPU_UPLOAD => (2, 1),
            _ => (self.CPUPageProperty, self.MemoryPoolPreference),
        };
        HeapProperties { Type: heap_type::CUSTOM, CPUPageProperty: cpu, MemoryPoolPreference: pool, CreationNodeMask: 1, VisibleNodeMask: 1 }
    }
}

/// `D3D12_HEAP_DESC` (48 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct HeapDesc {
    pub SizeInBytes: u64,
    pub Properties: HeapProperties,
    pub Alignment: u64,
    pub Flags: u32,
}
const _: () = assert!(std::mem::size_of::<HeapDesc>() == 48);

/// `D3D12_CLEAR_VALUE` (20 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ClearValue {
    pub Format: u32,
    pub Color: [f32; 4],
}

/// `D3D12_RESOURCE_ALLOCATION_INFO`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AllocationInfo {
    pub SizeInBytes: u64,
    pub Alignment: u64,
}

/// `D3D12_COMMAND_QUEUE_DESC` (16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandQueueDesc {
    pub Type: u32,
    pub Priority: i32,
    pub Flags: u32,
    pub NodeMask: u32,
}

/// `D3D12_DESCRIPTOR_HEAP_DESC` (16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DescriptorHeapDesc {
    pub Type: u32,
    pub NumDescriptors: u32,
    pub Flags: u32,
    pub NodeMask: u32,
}

/// `D3D12_RECT`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// `D3D12_VIEWPORT` (24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
const _: () = assert!(std::mem::size_of::<Viewport>() == 24);

/// `D3D12_RESOURCE_BARRIER` (32 bytes): Type @0, Flags @4, union @8.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ResourceBarrier {
    pub Type: u32,
    pub Flags: u32,
    /// Transition: pResource, Subresource, StateBefore, StateAfter.
    /// Aliasing: pResourceBefore, pResourceAfter. UAV: pResource.
    pub u: [u64; 3],
}
const _: () = assert!(std::mem::size_of::<ResourceBarrier>() == 32);
