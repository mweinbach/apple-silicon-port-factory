//! Direct3D 11 resources (`ID3D11Buffer`, `ID3D11Texture1D/2D/3D`, `IDXGISurface1`)
//! and views (`SRV`, `UAV`, `RTV`, `DSV`) backed by `d3d12_metal::metal::resource::Resource`.

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock, Weak};

use d3d12_metal::metal::cmdlist::{CopyLoc, TargetView};
use d3d12_metal::metal::resource::{CreateError, Resource as MtlResource};
use d3d12_metal::metal::views;
use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

use crate::handles::KObject;
use crate::types::*;

use super::com::*;
use super::device::DeviceCore;
use super::types::*;

/// Allocate a committed `d3d12_metal` resource without importing `objc2_metal`.
pub fn new_committed_resource(
    desc: &ResourceDesc,
    heap: &HeapProperties,
    label: &str,
) -> Result<MtlResource, CreateError> {
    MtlResource::new_committed_default(desc, heap, label)
}

/// Which D3D11 resource subclass a `ResourceCore` represents.
#[derive(Clone, Copy, Debug)]
pub enum ResourceKind {
    Buffer(D3D11_BUFFER_DESC),
    Texture1D(D3D11_TEXTURE1D_DESC),
    Texture2D(D3D11_TEXTURE2D_DESC),
    Texture3D(D3D11_TEXTURE3D_DESC),
}

impl ResourceKind {
    pub fn dimension(&self) -> u32 {
        match self {
            ResourceKind::Buffer(_) => resource_dim::BUFFER,
            ResourceKind::Texture1D(_) => resource_dim::TEXTURE1D,
            ResourceKind::Texture2D(_) => resource_dim::TEXTURE2D,
            ResourceKind::Texture3D(_) => resource_dim::TEXTURE3D,
        }
    }

    pub fn usage(&self) -> u32 {
        match self {
            ResourceKind::Buffer(d) => d.Usage,
            ResourceKind::Texture1D(d) => d.Usage,
            ResourceKind::Texture2D(d) => d.Usage,
            ResourceKind::Texture3D(d) => d.Usage,
        }
    }

    pub fn cpu_access(&self) -> u32 {
        match self {
            ResourceKind::Buffer(d) => d.CPUAccessFlags,
            ResourceKind::Texture1D(d) => d.CPUAccessFlags,
            ResourceKind::Texture2D(d) => d.CPUAccessFlags,
            ResourceKind::Texture3D(d) => d.CPUAccessFlags,
        }
    }

    pub fn format(&self) -> u32 {
        match self {
            ResourceKind::Buffer(_) => 0,
            ResourceKind::Texture1D(d) => d.Format,
            ResourceKind::Texture2D(d) => d.Format,
            ResourceKind::Texture3D(d) => d.Format,
        }
    }

    pub fn sample_count(&self) -> u32 {
        match self {
            ResourceKind::Texture2D(d) => d.SampleDesc.Count.max(1),
            _ => 1,
        }
    }

    pub fn buffer_stride(&self) -> u32 {
        match self {
            ResourceKind::Buffer(d) => d.StructureByteStride,
            _ => 0,
        }
    }

    pub fn buffer_width(&self) -> u32 {
        match self {
            ResourceKind::Buffer(d) => d.ByteWidth,
            _ => 0,
        }
    }

    pub fn bind_flags(&self) -> u32 {
        match self {
            ResourceKind::Buffer(d) => d.BindFlags,
            ResourceKind::Texture1D(d) => d.BindFlags,
            ResourceKind::Texture2D(d) => d.BindFlags,
            ResourceKind::Texture3D(d) => d.BindFlags,
        }
    }

    pub fn misc_flags(&self) -> u32 {
        match self {
            ResourceKind::Buffer(d) => d.MiscFlags,
            ResourceKind::Texture1D(d) => d.MiscFlags,
            ResourceKind::Texture2D(d) => d.MiscFlags,
            ResourceKind::Texture3D(d) => d.MiscFlags,
        }
    }
}

/// Active `Map` state tracked per subresource.
#[derive(Clone)]
pub struct MapState {
    pub map_type: u32,
    pub writable: bool,
    /// For deferred-context `WRITE_DISCARD` / `WRITE_NO_OVERWRITE`, a CPU scratch buffer.
    pub deferred_scratch: Option<Vec<u8>>,
    pub row_pitch: u32,
    pub depth_pitch: u32,
}

/// Shared state backing any D3D11 resource COM object.
pub struct ResourceCore {
    pub device: Arc<DeviceCore>,
    pub kind: ResourceKind,
    /// Current Metal backing resource. Swapped on `Map(WRITE_DISCARD)` for dynamic
    /// buffers or on `IDXGISwapChain::Present` for backbuffer 0.
    pub mtl: RwLock<Arc<MtlResource>>,
    /// Lazily allocated READBACK staging buffer for CPU `Map`/`Unmap` of textures.
    pub tex_staging: Mutex<Option<TexStaging>>,
    pub mapped: Mutex<HashMap<u32, MapState>>,
    pub eviction_priority: AtomicU32,
    pub min_lod_bits: AtomicU32,
    /// Assigned legacy KMT shared handle from `IDXGIResource::GetSharedHandle` (0 if none).
    legacy_shared_handle: AtomicUsize,
    /// True once this resource has been opened on at least one other `DeviceCore`.
    has_remote_devices: AtomicBool,
    /// All devices currently sharing this resource (populated only for multi-device sharing).
    shared_devices: Mutex<Vec<Weak<DeviceCore>>>,
}

impl Drop for ResourceCore {
    fn drop(&mut self) {
        let h = self.legacy_shared_handle.load(Ordering::Relaxed);
        if h != 0 {
            if let Ok(mut reg) = shared_registry().lock() {
                reg.legacy_by_handle.remove(&h);
            }
        }
    }
}

pub struct TexStaging {
    pub buf: Arc<MtlResource>,
    pub layouts: Vec<d3d12_metal::layout::SubresourceLayout>,
}

impl ResourceCore {
    pub fn current_mtl(&self) -> Arc<MtlResource> {
        self.mtl.read().unwrap().clone()
    }

    pub fn replace_mtl(&self, next: Arc<MtlResource>) -> Arc<MtlResource> {
        let mut g = self.mtl.write().unwrap();
        std::mem::replace(&mut *g, next)
    }

    pub fn is_buffer(&self) -> bool {
        matches!(self.kind, ResourceKind::Buffer(_))
    }

    /// Record that `dev` is using this shared resource so cross-queue GPU work
    /// stays coherent when `dev` differs from the creating device.
    pub fn register_shared_device(&self, dev: &Arc<DeviceCore>) {
        if Arc::ptr_eq(&self.device, dev) {
            return;
        }
        let mut list = self.shared_devices.lock().unwrap();
        list.retain(|w| w.strong_count() > 0);
        let ensure = |list: &mut Vec<Weak<DeviceCore>>, target: &Arc<DeviceCore>| {
            if !list.iter().any(|w| w.upgrade().is_some_and(|d| Arc::ptr_eq(&d, target))) {
                list.push(Arc::downgrade(target));
            }
        };
        ensure(&mut list, &self.device);
        ensure(&mut list, dev);
        self.has_remote_devices.store(true, Ordering::Release);
    }

    /// Flush & wait idle on any OTHER `DeviceCore` sharing this resource.
    /// Must be called WITHOUT holding `current.imm_exec` to prevent lock inversion.
    pub fn sync_other_devices(&self, current: &DeviceCore) {
        if !self.has_remote_devices.load(Ordering::Acquire) {
            return;
        }
        let others: Vec<Arc<DeviceCore>> = {
            let mut list = self.shared_devices.lock().unwrap();
            let mut live = Vec::new();
            list.retain(|w| {
                if let Some(d) = w.upgrade() {
                    if !std::ptr::eq(Arc::as_ptr(&d), current) {
                        live.push(d);
                    }
                    true
                } else {
                    false
                }
            });
            live
        };
        for dev in others {
            dev.flush_and_wait_idle();
        }
    }

    pub fn ensure_tex_staging(&self) -> Result<(Arc<MtlResource>, Vec<d3d12_metal::layout::SubresourceLayout>), String> {
        let mut g = self.tex_staging.lock().unwrap();
        if let Some(ts) = g.as_ref() {
            return Ok((ts.buf.clone(), ts.layouts.clone()));
        }
        let mtl = self.current_mtl();
        let n = mtl.desc.subresource_count().max(1);
        let (layouts, total) = d3d12_metal::layout::copyable_footprints(&mtl.desc, 0, n, 0)
            .ok_or_else(|| format!("copyable_footprints failed for {:?}", mtl.desc))?;
        let buf_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: total.max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
        let buf = Arc::new(
            new_committed_resource(&buf_desc, &hp, "d3d11 tex staging")
                .map_err(|e| format!("tex staging alloc: {e:?}"))?,
        );
        buf.register_ptr();
        *g = Some(TexStaging { buf: buf.clone(), layouts: layouts.clone() });
        Ok((buf, layouts))
    }

    fn new_core(device: Arc<DeviceCore>, kind: ResourceKind, mtl: Arc<MtlResource>) -> Arc<Self> {
        Arc::new(Self {
            device,
            kind,
            mtl: RwLock::new(mtl),
            tex_staging: Mutex::new(None),
            mapped: Mutex::new(HashMap::new()),
            eviction_priority: AtomicU32::new(0x7800_0000), // DXGI_RESOURCE_PRIORITY_NORMAL
            min_lod_bits: AtomicU32::new(0),
            legacy_shared_handle: AtomicUsize::new(0),
            has_remote_devices: AtomicBool::new(false),
            shared_devices: Mutex::new(Vec::new()),
        })
    }

    pub fn new_buffer(device: Arc<DeviceCore>, desc: D3D11_BUFFER_DESC) -> Result<Arc<Self>, HRESULT> {
        if desc.ByteWidth == 0 {
            return Err(d11_failed!(E_INVALIDARG, "CreateBuffer: ByteWidth == 0"));
        }
        let (rdesc, hp) = buffer_d3d12_desc(&desc);
        let mtl = match new_committed_resource(&rdesc, &hp, "d3d11 buffer") {
            Ok(r) => Arc::new(r),
            Err(CreateError::OutOfMemory(e)) => return Err(d11_failed!(E_OUTOFMEMORY, "CreateBuffer: {e}")),
            Err(e) => return Err(d11_failed!(E_INVALIDARG, "CreateBuffer: {e:?}")),
        };
        mtl.register_ptr();
        Ok(Self::new_core(device, ResourceKind::Buffer(desc), mtl))
    }

    pub fn alloc_fresh_buffer_mtl(&self) -> Result<Arc<MtlResource>, String> {
        let ResourceKind::Buffer(desc) = self.kind else {
            return Err("not a buffer".into());
        };
        let (rdesc, hp) = buffer_d3d12_desc(&desc);
        let mtl = Arc::new(
            new_committed_resource(&rdesc, &hp, "d3d11 buffer discard")
                .map_err(|e| format!("{e:?}"))?,
        );
        mtl.register_ptr();
        Ok(mtl)
    }

    pub fn new_texture1d(device: Arc<DeviceCore>, mut desc: D3D11_TEXTURE1D_DESC) -> Result<Arc<Self>, HRESULT> {
        if desc.Width == 0 || desc.ArraySize == 0 {
            return Err(d11_failed!(E_INVALIDARG, "CreateTexture1D: invalid size {desc:?}"));
        }
        let rdesc = ResourceDesc {
            Dimension: dim::TEXTURE1D,
            Alignment: 0,
            Width: desc.Width as u64,
            Height: 1,
            DepthOrArraySize: desc.ArraySize as u16,
            MipLevels: desc.MipLevels as u16,
            Format: desc.Format,
            SampleCount: 1,
            SampleQuality: 0,
            Layout: 0,
            Flags: bind_flags_to_d3d12(desc.BindFlags),
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let mtl = match new_committed_resource(&rdesc, &hp, "d3d11 tex1d") {
            Ok(r) => Arc::new(r),
            Err(CreateError::OutOfMemory(e)) => return Err(d11_failed!(E_OUTOFMEMORY, "CreateTexture1D: {e}")),
            Err(e) => return Err(d11_failed!(E_INVALIDARG, "CreateTexture1D({desc:?}): {e:?}")),
        };
        mtl.register_ptr();
        desc.MipLevels = mtl.desc.mip_levels();
        Ok(Self::new_core(device, ResourceKind::Texture1D(desc), mtl))
    }

    pub fn new_texture2d(device: Arc<DeviceCore>, mut desc: D3D11_TEXTURE2D_DESC) -> Result<Arc<Self>, HRESULT> {
        if desc.Width == 0 || desc.Height == 0 || desc.ArraySize == 0 {
            return Err(d11_failed!(E_INVALIDARG, "CreateTexture2D: invalid size {desc:?}"));
        }
        if desc.SampleDesc.Count == 0 {
            desc.SampleDesc.Count = 1;
        }
        let rdesc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Alignment: 0,
            Width: desc.Width as u64,
            Height: desc.Height,
            DepthOrArraySize: desc.ArraySize as u16,
            MipLevels: desc.MipLevels as u16,
            Format: desc.Format,
            SampleCount: desc.SampleDesc.Count,
            SampleQuality: desc.SampleDesc.Quality,
            Layout: 0,
            Flags: bind_flags_to_d3d12(desc.BindFlags),
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let mtl = match new_committed_resource(&rdesc, &hp, "d3d11 tex2d") {
            Ok(r) => Arc::new(r),
            Err(CreateError::OutOfMemory(e)) => return Err(d11_failed!(E_OUTOFMEMORY, "CreateTexture2D: {e}")),
            Err(e) => return Err(d11_failed!(E_INVALIDARG, "CreateTexture2D({desc:?}): {e:?}")),
        };
        mtl.register_ptr();
        desc.MipLevels = mtl.desc.mip_levels();
        Ok(Self::new_core(device, ResourceKind::Texture2D(desc), mtl))
    }

    pub fn wrap_texture2d(
        device: Arc<DeviceCore>,
        mtl: Arc<MtlResource>,
        desc: D3D11_TEXTURE2D_DESC,
    ) -> Arc<Self> {
        mtl.register_ptr();
        Self::new_core(device, ResourceKind::Texture2D(desc), mtl)
    }

    pub fn new_texture3d(device: Arc<DeviceCore>, mut desc: D3D11_TEXTURE3D_DESC) -> Result<Arc<Self>, HRESULT> {
        if desc.Width == 0 || desc.Height == 0 || desc.Depth == 0 {
            return Err(d11_failed!(E_INVALIDARG, "CreateTexture3D: invalid size {desc:?}"));
        }
        let rdesc = ResourceDesc {
            Dimension: dim::TEXTURE3D,
            Alignment: 0,
            Width: desc.Width as u64,
            Height: desc.Height,
            DepthOrArraySize: desc.Depth as u16,
            MipLevels: desc.MipLevels as u16,
            Format: desc.Format,
            SampleCount: 1,
            SampleQuality: 0,
            Layout: 0,
            Flags: bind_flags_to_d3d12(desc.BindFlags),
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let mtl = match new_committed_resource(&rdesc, &hp, "d3d11 tex3d") {
            Ok(r) => Arc::new(r),
            Err(CreateError::OutOfMemory(e)) => return Err(d11_failed!(E_OUTOFMEMORY, "CreateTexture3D: {e}")),
            Err(e) => return Err(d11_failed!(E_INVALIDARG, "CreateTexture3D({desc:?}): {e:?}")),
        };
        mtl.register_ptr();
        desc.MipLevels = mtl.desc.mip_levels();
        Ok(Self::new_core(device, ResourceKind::Texture3D(desc), mtl))
    }
}

// ---------------------------------------------------------------------------
// Shared resource registry (legacy KMT handles + NT handles & named lookup)
// ---------------------------------------------------------------------------

struct SharedRegistry {
    legacy_by_handle: HashMap<usize, Weak<ResourceCore>>,
    named_nt: HashMap<String, Weak<SharedNtResource>>,
}

fn shared_registry() -> &'static Mutex<SharedRegistry> {
    static REG: OnceLock<Mutex<SharedRegistry>> = OnceLock::new();
    REG.get_or_init(|| {
        Mutex::new(SharedRegistry {
            legacy_by_handle: HashMap::new(),
            named_nt: HashMap::new(),
        })
    })
}

/// Separate handle space for legacy KMT shared handles (`IDXGIResource::GetSharedHandle`)
/// so they never collide with kernel NT handles in `crate::handles`.
static LEGACY_HANDLE_NEXT: AtomicUsize = AtomicUsize::new(0x4000_0004);

struct SharedNtResource {
    core: Arc<ResourceCore>,
    _access: u32,
    name_key: Option<String>,
}

impl Drop for SharedNtResource {
    fn drop(&mut self) {
        if let Some(ref k) = self.name_key {
            if let Ok(mut reg) = shared_registry().lock() {
                if reg.named_nt.get(k).is_some_and(|w| w.upgrade().is_none()) {
                    reg.named_nt.remove(k);
                }
            }
        }
    }
}

impl KObject for SharedNtResource {
    fn type_name(&self) -> &'static str {
        "D3D11SharedResource"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Read a NUL-terminated UTF-16 string safely from potentially unaligned guest memory.
pub unsafe fn read_wide_unaligned(p: *const u16) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let mut units = Vec::new();
    let mut i = 0usize;
    loop {
        let ch = std::ptr::read_unaligned(p.add(i));
        if ch == 0 {
            break;
        }
        units.push(ch);
        i += 1;
        if i >= 32768 {
            break;
        }
    }
    Some(String::from_utf16_lossy(&units))
}

pub fn get_or_create_legacy_shared_handle(core: &Arc<ResourceCore>) -> Result<HANDLE, HRESULT> {
    let misc = core.kind.misc_flags();
    let allowed = (misc & (misc_flag::SHARED | misc_flag::SHARED_KEYEDMUTEX)) != 0
        && (misc & misc_flag::SHARED_NTHANDLE) == 0;
    if !allowed {
        return Err(d11_failed!(
            E_INVALIDARG,
            "IDXGIResource::GetSharedHandle: resource misc_flags={misc:#x} not legacy-shareable"
        ));
    }
    let existing = core.legacy_shared_handle.load(Ordering::Acquire);
    if existing != 0 {
        return Ok(existing as HANDLE);
    }
    let mut reg = shared_registry().lock().unwrap();
    let again = core.legacy_shared_handle.load(Ordering::Acquire);
    if again != 0 {
        return Ok(again as HANDLE);
    }
    let h = LEGACY_HANDLE_NEXT.fetch_add(4, Ordering::Relaxed);
    reg.legacy_by_handle.insert(h, Arc::downgrade(core));
    core.legacy_shared_handle.store(h, Ordering::Release);
    Ok(h as HANDLE)
}

pub fn create_nt_shared_handle(
    core: &Arc<ResourceCore>,
    access: u32,
    name: Option<String>,
) -> Result<HANDLE, HRESULT> {
    let misc = core.kind.misc_flags();
    if (misc & misc_flag::SHARED_NTHANDLE) == 0 {
        return Err(d11_failed!(
            E_INVALIDARG,
            "IDXGIResource1::CreateSharedHandle: resource misc_flags={misc:#x} lacks D3D11_RESOURCE_MISC_SHARED_NTHANDLE"
        ));
    }
    if access == 0 {
        return Err(d11_failed!(
            E_INVALIDARG,
            "IDXGIResource1::CreateSharedHandle: dwAccess == 0"
        ));
    }
    let name_key = name.filter(|s| !s.is_empty()).map(|s| s.to_lowercase());
    let mut reg = shared_registry().lock().unwrap();
    if let Some(ref k) = name_key {
        if reg.named_nt.get(k).and_then(|w| w.upgrade()).is_some() {
            return Err(d11_failed!(
                DXGI_ERROR_NAME_ALREADY_EXISTS,
                "IDXGIResource1::CreateSharedHandle: name {:?} already exists",
                k
            ));
        }
    }
    let entry = Arc::new(SharedNtResource {
        core: core.clone(),
        _access: access,
        name_key: name_key.clone(),
    });
    if let Some(k) = name_key {
        reg.named_nt.insert(k, Arc::downgrade(&entry));
    }
    drop(reg);
    Ok(crate::handles::insert(entry))
}

pub fn lookup_legacy_shared_resource(h: HANDLE) -> Result<Arc<ResourceCore>, HRESULT> {
    if h.is_null() {
        return Err(E_INVALIDARG);
    }
    let key = h as usize;
    let mut reg = shared_registry().lock().unwrap();
    if let Some(w) = reg.legacy_by_handle.get(&key) {
        if let Some(core) = w.upgrade() {
            return Ok(core);
        }
        reg.legacy_by_handle.remove(&key);
    }
    Err(E_HANDLE)
}

pub fn lookup_nt_shared_resource(h: HANDLE) -> Result<Arc<ResourceCore>, HRESULT> {
    if h.is_null() {
        return Err(E_INVALIDARG);
    }
    if let Some(entry) = crate::handles::get_as::<SharedNtResource>(h) {
        return Ok(entry.core.clone());
    }
    Err(E_HANDLE)
}

pub fn lookup_named_shared_resource(name: &str, access: u32) -> Result<Arc<ResourceCore>, HRESULT> {
    if name.is_empty() || access == 0 {
        return Err(E_INVALIDARG);
    }
    let key = name.to_lowercase();
    let mut reg = shared_registry().lock().unwrap();
    if let Some(w) = reg.named_nt.get(&key) {
        if let Some(entry) = w.upgrade() {
            return Ok(entry.core.clone());
        }
        reg.named_nt.remove(&key);
    }
    Err(E_HANDLE)
}

/// Wrap an existing shared `ResourceCore` in a fresh COM wrapper on `device`
/// and QueryInterface it for `riid`.
pub unsafe fn wrap_shared_resource(
    device: Arc<DeviceCore>,
    core: Arc<ResourceCore>,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if riid.is_null() {
        return E_INVALIDARG;
    }
    if out.is_null() {
        return E_POINTER;
    }
    core.register_shared_device(&device);
    let raw: *mut c_void = match core.kind {
        ResourceKind::Buffer(_) => Obj::new(D3d11Buffer { device, core }) as *mut c_void,
        ResourceKind::Texture1D(_) => Obj::new(D3d11Texture1D { device, core }) as *mut c_void,
        ResourceKind::Texture2D(_) => Obj::new(D3d11Texture2D { device, core }) as *mut c_void,
        ResourceKind::Texture3D(_) => Obj::new(D3d11Texture3D { device, core }) as *mut c_void,
    };
    let hr = query_out(raw, riid, out);
    com_release(raw);
    hr
}

fn bind_flags_to_d3d12(bind: u32) -> u32 {
    let mut f = 0u32;
    if bind & bind_flag::RENDER_TARGET != 0 {
        f |= resource_flags::ALLOW_RENDER_TARGET;
    }
    if bind & bind_flag::DEPTH_STENCIL != 0 {
        f |= resource_flags::ALLOW_DEPTH_STENCIL;
    }
    if bind & bind_flag::UNORDERED_ACCESS != 0 {
        f |= resource_flags::ALLOW_UNORDERED_ACCESS;
    }
    f
}

fn buffer_d3d12_desc(desc: &D3D11_BUFFER_DESC) -> (ResourceDesc, HeapProperties) {
    let rdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Alignment: 0,
        Width: (desc.ByteWidth as u64).max(1),
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 0,
        SampleCount: 1,
        SampleQuality: 0,
        Layout: 1, // ROW_MAJOR
        Flags: if desc.BindFlags & bind_flag::UNORDERED_ACCESS != 0 {
            resource_flags::ALLOW_UNORDERED_ACCESS
        } else {
            0
        },
    };
    let hty = if desc.Usage == usage::STAGING || desc.CPUAccessFlags & cpu_access::READ != 0 {
        heap_type::READBACK
    } else if desc.Usage == usage::DYNAMIC || desc.CPUAccessFlags & cpu_access::WRITE != 0 {
        heap_type::UPLOAD
    } else {
        heap_type::DEFAULT
    };
    (rdesc, HeapProperties { Type: hty, ..Default::default() })
}

// ---------------------------------------------------------------------------
// COM classes: Buffer, Texture1D, Texture2D, Texture3D (each 11 slots)
// ---------------------------------------------------------------------------

pub struct D3d11Buffer {
    pub device: Arc<DeviceCore>,
    pub core: Arc<ResourceCore>,
}
pub struct D3d11Texture1D {
    pub device: Arc<DeviceCore>,
    pub core: Arc<ResourceCore>,
}
pub struct D3d11Texture2D {
    pub device: Arc<DeviceCore>,
    pub core: Arc<ResourceCore>,
}
pub struct D3d11Texture3D {
    pub device: Arc<DeviceCore>,
    pub core: Arc<ResourceCore>,
}

impl HasDeviceCore for D3d11Buffer {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.device }
}
impl HasDeviceCore for D3d11Texture1D {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.device }
}
impl HasDeviceCore for D3d11Texture2D {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.device }
}
impl HasDeviceCore for D3d11Texture3D {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.device }
}

unsafe extern "win64" fn res_get_type<T: HasResourceCore>(this: *mut Obj<T>, out: *mut u32) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).res_core().kind.dimension());
    }
}
unsafe extern "win64" fn res_set_eviction_priority<T: HasResourceCore>(this: *mut Obj<T>, prio: u32) {
    Obj::get(this).res_core().eviction_priority.store(prio, Ordering::Relaxed);
}
unsafe extern "win64" fn res_get_eviction_priority<T: HasResourceCore>(this: *mut Obj<T>) -> u32 {
    Obj::get(this).res_core().eviction_priority.load(Ordering::Relaxed)
}

pub trait HasResourceCore: HasDeviceCore {
    fn res_core(&self) -> &Arc<ResourceCore>;
}
impl HasResourceCore for D3d11Buffer {
    fn res_core(&self) -> &Arc<ResourceCore> { &self.core }
}
impl HasResourceCore for D3d11Texture1D {
    fn res_core(&self) -> &Arc<ResourceCore> { &self.core }
}
impl HasResourceCore for D3d11Texture2D {
    fn res_core(&self) -> &Arc<ResourceCore> { &self.core }
}
impl HasResourceCore for D3d11Texture3D {
    fn res_core(&self) -> &Arc<ResourceCore> { &self.core }
}

fn resource_prefix_slots<T: HasResourceCore>() -> Vec<(&'static str, Option<*const ()>)> {
    let mut s = child_slots::<T>();
    s.push(("GetType", Some(res_get_type::<T> as *const ())));
    s.push(("SetEvictionPriority", Some(res_set_eviction_priority::<T> as *const ())));
    s.push(("GetEvictionPriority", Some(res_get_eviction_priority::<T> as *const ())));
    debug_assert_eq!(s.len(), 10);
    s
}

/// Slot count of `ID3D11Resource` base vtable (0..9).
pub const ID3D11_RESOURCE_SLOT_COUNT: usize = 10;
/// Slot count of `ID3D11Buffer` / `ID3D11Texture1D/2D/3D` vtables (0..10).
pub const ID3D11_TYPED_RESOURCE_SLOT_COUNT: usize = 11;

unsafe extern "win64" fn buffer_get_desc(this: *mut Obj<D3d11Buffer>, out: *mut D3D11_BUFFER_DESC) {
    if !out.is_null() {
        if let ResourceKind::Buffer(d) = Obj::get(this).core.kind {
            std::ptr::write_unaligned(out, d);
        }
    }
}
unsafe extern "win64" fn tex1d_get_desc(this: *mut Obj<D3d11Texture1D>, out: *mut D3D11_TEXTURE1D_DESC) {
    if !out.is_null() {
        if let ResourceKind::Texture1D(d) = Obj::get(this).core.kind {
            std::ptr::write_unaligned(out, d);
        }
    }
}
unsafe extern "win64" fn tex2d_get_desc(this: *mut Obj<D3d11Texture2D>, out: *mut D3D11_TEXTURE2D_DESC) {
    if !out.is_null() {
        if let ResourceKind::Texture2D(d) = Obj::get(this).core.kind {
            std::ptr::write_unaligned(out, d);
        }
    }
}
unsafe extern "win64" fn tex3d_get_desc(this: *mut Obj<D3d11Texture3D>, out: *mut D3D11_TEXTURE3D_DESC) {
    if !out.is_null() {
        if let ResourceKind::Texture3D(d) = Obj::get(this).core.kind {
            std::ptr::write_unaligned(out, d);
        }
    }
}

static BUF_CLASS: OnceLock<Class> = OnceLock::new();
static TEX1D_CLASS: OnceLock<Class> = OnceLock::new();
static TEX2D_CLASS: OnceLock<Class> = OnceLock::new();
static TEX3D_CLASS: OnceLock<Class> = OnceLock::new();

fn is_dxgi_resource_iid(iid: &Guid) -> bool {
    *iid == IID_IDXGIResource
        || *iid == IID_IDXGIResource1
        || *iid == crate::dxgi::swapchain::IID_IDXGIDeviceSubObject
        || *iid == crate::dxgi::IID_IDXGIObject
}

unsafe fn query_dxgi_resource<T: HasResourceCore>(this: *mut Obj<T>, iid: &Guid) -> Option<*mut c_void> {
    if is_dxgi_resource_iid(iid) {
        Obj::add_ref(this);
        let r = Obj::get(this);
        let dxgi = Obj::new(DxgiResource {
            res_com: this as usize,
            device: r.device_core().clone(),
            core: r.res_core().clone(),
        });
        return Some(dxgi as *mut c_void);
    }
    None
}

impl ComClass for D3d11Buffer {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&BUF_CLASS, || {
            let mut s = resource_prefix_slots::<D3d11Buffer>();
            s.push(("GetDesc", Some(buffer_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_TYPED_RESOURCE_SLOT_COUNT);
            Class {
                name: "ID3D11Buffer",
                vtbl: build_vtable("ID3D11Buffer", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11Resource, IID_ID3D11Buffer],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        query_dxgi_resource(this, iid)
    }
}

impl ComClass for D3d11Texture1D {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&TEX1D_CLASS, || {
            let mut s = resource_prefix_slots::<D3d11Texture1D>();
            s.push(("GetDesc", Some(tex1d_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_TYPED_RESOURCE_SLOT_COUNT);
            Class {
                name: "ID3D11Texture1D",
                vtbl: build_vtable("ID3D11Texture1D", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11Resource, IID_ID3D11Texture1D],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        query_dxgi_resource(this, iid)
    }
}

impl ComClass for D3d11Texture2D {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&TEX2D_CLASS, || {
            let mut s = resource_prefix_slots::<D3d11Texture2D>();
            s.push(("GetDesc", Some(tex2d_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_TYPED_RESOURCE_SLOT_COUNT);
            Class {
                name: "ID3D11Texture2D",
                vtbl: build_vtable("ID3D11Texture2D", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11Resource, IID_ID3D11Texture2D],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        if *iid == IID_IDXGISurface || *iid == IID_IDXGISurface1 || *iid == IID_IDXGISurface2 {
            Obj::add_ref(this);
            let t = Obj::get(this);
            return Some(DxgiSurface::new_com(this as usize, t.device.clone(), t.core.clone(), 0));
        }
        query_dxgi_resource(this, iid)
    }
}

impl ComClass for D3d11Texture3D {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&TEX3D_CLASS, || {
            let mut s = resource_prefix_slots::<D3d11Texture3D>();
            s.push(("GetDesc", Some(tex3d_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_TYPED_RESOURCE_SLOT_COUNT);
            Class {
                name: "ID3D11Texture3D",
                vtbl: build_vtable("ID3D11Texture3D", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11Resource, IID_ID3D11Texture3D],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        query_dxgi_resource(this, iid)
    }
}

/// Slot count of `IDXGIResource1` vtable (0..13).
pub const IDXGI_RESOURCE1_SLOT_COUNT: usize = 14;

/// Tear-off `IDXGIResource` / `IDXGIResource1` for any D3D11 resource (14 slots).
pub struct DxgiResource {
    res_com: usize,
    device: Arc<DeviceCore>,
    core: Arc<ResourceCore>,
}

impl Drop for DxgiResource {
    fn drop(&mut self) {
        unsafe { com_release(self.res_com as *mut c_void) };
    }
}

static DXGI_RES_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for DxgiResource {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&DXGI_RES_CLASS, || {
            let mut s = dxgi_object_slots::<DxgiResource>();
            s.extend([
                ("GetParent", Some(dxgi_res_get_parent as *const ())),
                ("GetDevice", Some(dxgi_res_get_device as *const ())),
                // IDXGIResource
                ("GetSharedHandle", Some(dxgi_res_get_shared_handle as *const ())),
                ("GetUsage", Some(dxgi_res_get_usage as *const ())),
                ("SetEvictionPriority", Some(dxgi_res_set_eviction_priority as *const ())),
                ("GetEvictionPriority", Some(dxgi_res_get_eviction_priority as *const ())),
                // IDXGIResource1
                ("CreateSubresourceSurface", Some(dxgi_res_create_subresource_surface as *const ())),
                ("CreateSharedHandle", Some(dxgi_res_create_shared_handle as *const ())),
            ]);
            debug_assert_eq!(s.len(), IDXGI_RESOURCE1_SLOT_COUNT);
            Class {
                name: "IDXGIResource1",
                vtbl: build_vtable("IDXGIResource1", &s),
                iids: &[
                    crate::dxgi::IID_IDXGIObject,
                    crate::dxgi::swapchain::IID_IDXGIDeviceSubObject,
                    IID_IDXGIResource,
                    IID_IDXGIResource1,
                ],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        let mut out: *mut c_void = std::ptr::null_mut();
        if com_query(Obj::get(this).res_com as *mut c_void, iid, &mut out) == S_OK && !out.is_null() {
            return Some(out);
        }
        None
    }
}

unsafe extern "win64" fn dxgi_res_get_parent(
    this: *mut Obj<DxgiResource>,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    dxgi_res_get_device(this, riid, out)
}

unsafe extern "win64" fn dxgi_res_get_device(
    this: *mut Obj<DxgiResource>,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    let dev = super::device::D3d11Device::get_or_wrap(&Obj::get(this).device);
    let hr = query_out(dev as *mut c_void, riid, out);
    Obj::release(dev);
    hr
}

unsafe extern "win64" fn dxgi_res_get_shared_handle(
    this: *mut Obj<DxgiResource>,
    out_handle: *mut HANDLE,
) -> HRESULT {
    if out_handle.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out_handle, std::ptr::null_mut());
    match get_or_create_legacy_shared_handle(&Obj::get(this).core) {
        Ok(h) => {
            std::ptr::write_unaligned(out_handle, h);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn dxgi_res_get_usage(
    this: *mut Obj<DxgiResource>,
    out_usage: *mut u32,
) -> HRESULT {
    if out_usage.is_null() {
        return E_INVALIDARG;
    }
    let kind = &Obj::get(this).core.kind;
    let bind = kind.bind_flags();
    let misc = kind.misc_flags();
    let mut u = 0u32;
    if bind & bind_flag::SHADER_RESOURCE != 0 {
        u |= 0x10; // DXGI_USAGE_SHADER_INPUT
    }
    if bind & bind_flag::RENDER_TARGET != 0 {
        u |= 0x20; // DXGI_USAGE_RENDER_TARGET_OUTPUT
    }
    if bind & bind_flag::UNORDERED_ACCESS != 0 {
        u |= 0x400; // DXGI_USAGE_UNORDERED_ACCESS
    }
    if misc & (misc_flag::SHARED | misc_flag::SHARED_KEYEDMUTEX | misc_flag::SHARED_NTHANDLE) != 0 {
        u |= 0x80; // DXGI_USAGE_SHARED
    }
    match kind.usage() {
        usage::DYNAMIC => u |= 1,  // DXGI_CPU_ACCESS_DYNAMIC
        usage::STAGING => u |= 2,  // DXGI_CPU_ACCESS_READ_WRITE
        usage::IMMUTABLE => u |= 0x100, // DXGI_USAGE_READ_ONLY
        _ => {}
    }
    std::ptr::write_unaligned(out_usage, u);
    S_OK
}

unsafe extern "win64" fn dxgi_res_set_eviction_priority(
    this: *mut Obj<DxgiResource>,
    prio: u32,
) -> HRESULT {
    Obj::get(this).core.eviction_priority.store(prio, Ordering::Relaxed);
    S_OK
}

unsafe extern "win64" fn dxgi_res_get_eviction_priority(
    this: *mut Obj<DxgiResource>,
    out_prio: *mut u32,
) -> HRESULT {
    if out_prio.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(
        out_prio,
        Obj::get(this).core.eviction_priority.load(Ordering::Relaxed),
    );
    S_OK
}

unsafe extern "win64" fn dxgi_res_create_subresource_surface(
    this: *mut Obj<DxgiResource>,
    index: u32,
    out_surf: *mut *mut c_void,
) -> HRESULT {
    if out_surf.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out_surf, std::ptr::null_mut());
    let r = Obj::get(this);
    let ResourceKind::Texture2D(d) = r.core.kind else {
        return d11_failed!(E_INVALIDARG, "IDXGIResource1::CreateSubresourceSurface: not a Texture2D");
    };
    let sub_count = d.MipLevels.max(1).saturating_mul(d.ArraySize.max(1));
    if index >= sub_count {
        return d11_failed!(
            E_INVALIDARG,
            "IDXGIResource1::CreateSubresourceSurface: index {index} >= {sub_count}"
        );
    }
    com_add_ref(r.res_com as *mut c_void);
    std::ptr::write_unaligned(
        out_surf,
        DxgiSurface::new_com(r.res_com, r.device.clone(), r.core.clone(), index),
    );
    S_OK
}

unsafe extern "win64" fn dxgi_res_create_shared_handle(
    this: *mut Obj<DxgiResource>,
    _attributes: *const c_void,
    access: u32,
    name_w: LPCWSTR,
    out_handle: *mut HANDLE,
) -> HRESULT {
    if out_handle.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out_handle, std::ptr::null_mut());
    let name = read_wide_unaligned(name_w);
    match create_nt_shared_handle(&Obj::get(this).core, access, name) {
        Ok(h) => {
            std::ptr::write_unaligned(out_handle, h);
            S_OK
        }
        Err(hr) => hr,
    }
}

/// Slot count of `IDXGISurface2` vtable (0..13).
pub const IDXGI_SURFACE2_SLOT_COUNT: usize = 14;

/// Active GDI DC lock on an `IDXGISurface1` (created via `GetDC`, uploaded on `ReleaseDC`).
struct SurfaceDcState {
    hdc: usize,
    hbmp: usize,
    width: u32,
    height: u32,
}

/// Tear-off `IDXGISurface2` for `ID3D11Texture2D` (14 slots).
pub struct DxgiSurface {
    tex_com: usize,
    device: Arc<DeviceCore>,
    core: Arc<ResourceCore>,
    subresource: u32,
    dc_state: Mutex<Option<SurfaceDcState>>,
}

impl DxgiSurface {
    /// Wrap an already-AddRef'd `ID3D11Texture2D` pointer as an `IDXGISurface2` COM object.
    pub fn new_com(
        tex_com: usize,
        device: Arc<DeviceCore>,
        core: Arc<ResourceCore>,
        subresource: u32,
    ) -> *mut c_void {
        Obj::new(DxgiSurface {
            tex_com,
            device,
            core,
            subresource,
            dc_state: Mutex::new(None),
        }) as *mut c_void
    }
}

impl Drop for DxgiSurface {
    fn drop(&mut self) {
        if let Ok(mut g) = self.dc_state.lock() {
            if let Some(dc) = g.take() {
                crate::misc::gdi32::destroy_external_surface_dc(dc.hdc, dc.hbmp);
                self.core.mapped.lock().unwrap().remove(&self.subresource);
            }
        }
        unsafe { com_release(self.tex_com as *mut c_void) };
    }
}

static SURF_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for DxgiSurface {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&SURF_CLASS, || {
            let mut s = dxgi_object_slots::<DxgiSurface>();
            s.extend([
                ("GetParent", Some(surf_get_parent as *const ())),
                ("GetDevice", Some(surf_get_device as *const ())),
                ("GetDesc", Some(surf_get_desc as *const ())),
                ("Map", Some(surf_map as *const ())),
                ("Unmap", Some(surf_unmap as *const ())),
                ("GetDC", Some(surf_get_dc as *const ())),
                ("ReleaseDC", Some(surf_release_dc as *const ())),
                // IDXGISurface2
                ("GetResource", Some(surf_get_resource as *const ())),
            ]);
            debug_assert_eq!(s.len(), IDXGI_SURFACE2_SLOT_COUNT);
            Class {
                name: "IDXGISurface2",
                vtbl: build_vtable("IDXGISurface2", &s),
                iids: &[
                    crate::dxgi::IID_IDXGIObject,
                    crate::dxgi::swapchain::IID_IDXGIDeviceSubObject,
                    IID_IDXGISurface,
                    IID_IDXGISurface1,
                    IID_IDXGISurface2,
                ],
            }
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        if *iid == IID_ID3D11Texture2D || *iid == IID_ID3D11Resource || *iid == IID_ID3D11DeviceChild {
            let p = Obj::get(this).tex_com as *mut c_void;
            com_add_ref(p);
            return Some(p);
        }
        if *iid == IID_IDXGIResource || *iid == IID_IDXGIResource1 {
            let mut out: *mut c_void = std::ptr::null_mut();
            if com_query(Obj::get(this).tex_com as *mut c_void, iid, &mut out) == S_OK && !out.is_null() {
                return Some(out);
            }
        }
        None
    }
}

unsafe extern "win64" fn surf_get_parent(this: *mut Obj<DxgiSurface>, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    surf_get_device(this, riid, out)
}
unsafe extern "win64" fn surf_get_device(this: *mut Obj<DxgiSurface>, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let dev = super::device::D3d11Device::get_or_wrap(&Obj::get(this).device);
    let hr = query_out(dev as *mut c_void, riid, out);
    Obj::release(dev);
    hr
}
unsafe extern "win64" fn surf_get_desc(this: *mut Obj<DxgiSurface>, out: *mut DXGI_SURFACE_DESC) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let surf = Obj::get(this);
    if let ResourceKind::Texture2D(d) = surf.core.kind {
        let mips = d.MipLevels.max(1);
        let mip = surf.subresource % mips;
        let width = (d.Width >> mip).max(1);
        let height = (d.Height >> mip).max(1);
        std::ptr::write_unaligned(
            out,
            DXGI_SURFACE_DESC { Width: width, Height: height, Format: d.Format, SampleDesc: d.SampleDesc },
        );
        S_OK
    } else {
        E_FAIL
    }
}
unsafe extern "win64" fn surf_map(this: *mut Obj<DxgiSurface>, rect: *mut DXGI_MAPPED_RECT, flags: u32) -> HRESULT {
    if rect.is_null() {
        return d11_failed!(E_INVALIDARG, "IDXGISurface::Map: null pLockedRect");
    }
    std::ptr::write_unaligned(rect, DXGI_MAPPED_RECT { Pitch: 0, pBits: std::ptr::null_mut() });

    let valid_mask = dxgi_map::READ | dxgi_map::WRITE | dxgi_map::DISCARD;
    let has_read = (flags & dxgi_map::READ) != 0;
    let has_write = (flags & dxgi_map::WRITE) != 0;
    let has_discard = (flags & dxgi_map::DISCARD) != 0;
    if (flags & !valid_mask) != 0
        || (!has_read && !has_write)
        || (has_discard && (!has_write || has_read))
    {
        return d11_failed!(E_INVALIDARG, "IDXGISurface::Map: invalid MapFlags {flags:#x}");
    }

    let surf = Obj::get(this);
    let core = &surf.core;
    let sub = surf.subresource;

    // Atomically check & reserve the map slot so double-Map fails deterministically.
    {
        let mut mapped = core.mapped.lock().unwrap();
        if mapped.contains_key(&sub) {
            return d11_failed!(
                DXGI_ERROR_WAS_STILL_DRAWING,
                "IDXGISurface::Map: subresource {sub} is already mapped"
            );
        }
        mapped.insert(
            sub,
            MapState {
                map_type: flags,
                writable: has_write,
                deferred_scratch: None,
                row_pitch: 0,
                depth_pitch: 0,
            },
        );
    }

    let unreserve = || {
        core.mapped.lock().unwrap().remove(&sub);
    };

    let had_staging = core.tex_staging.lock().unwrap().is_some();
    let (staging_buf, layouts) = match core.ensure_tex_staging() {
        Ok(x) => x,
        Err(e) => {
            unreserve();
            return d11_failed!(E_FAIL, "IDXGISurface::Map staging alloc: {e}");
        }
    };
    let Some(lay) = layouts.get(sub as usize).copied() else {
        unreserve();
        return d11_failed!(
            E_INVALIDARG,
            "IDXGISurface::Map: subresource {sub} out of range ({})",
            layouts.len()
        );
    };

    // Sync any remote devices sharing this resource before reading/overwriting staging.
    core.sync_other_devices(&surf.device);

    // Readback GPU contents when READ is requested or when WRITE preserves existing pixels (no DISCARD).
    let need_readback = has_read || (has_write && !has_discard);
    if need_readback {
        let mtl = core.current_mtl();
        {
            let mut exec = surf.device.imm_exec.lock().unwrap();
            if let Err(e) = exec.ensure_open() {
                drop(exec);
                unreserve();
                return d11_failed!(E_FAIL, "IDXGISurface::Map ensure_open: {e}");
            }
            exec.cl.barrier();
            if let Err(e) = exec.cl.copy_texture_region(
                CopyLoc::Footprint(&staging_buf, lay.footprint),
                [0, 0, 0],
                CopyLoc::Subresource(&mtl, sub),
                None,
            ) {
                drop(exec);
                unreserve();
                return d11_failed!(E_FAIL, "IDXGISurface::Map readback copy: {e}");
            }
            exec.has_work = true;
            exec.keep((mtl, staging_buf.clone()));
        }
        surf.device.flush_and_wait_idle();
    } else if had_staging {
        // Wait for any previous upload from staging_buf to complete before CPU overwrites it.
        surf.device.flush_and_wait_idle();
    }

    let Some(base_ptr) = staging_buf.cpu_ptr() else {
        unreserve();
        return d11_failed!(E_FAIL, "IDXGISurface::Map: staging buffer has no cpu_ptr");
    };
    let ptr = base_ptr.add(lay.footprint.offset as usize);
    let row_pitch = lay.footprint.row_pitch;
    let depth_pitch = row_pitch * lay.num_rows;
    if let Some(ms) = core.mapped.lock().unwrap().get_mut(&sub) {
        ms.row_pitch = row_pitch;
        ms.depth_pitch = depth_pitch;
    }
    std::ptr::write_unaligned(rect, DXGI_MAPPED_RECT { Pitch: row_pitch as i32, pBits: ptr });
    S_OK
}
unsafe extern "win64" fn surf_unmap(this: *mut Obj<DxgiSurface>) -> HRESULT {
    let surf = Obj::get(this);
    let core = &surf.core;
    let sub = surf.subresource;
    let Some(ms) = core.mapped.lock().unwrap().remove(&sub) else {
        return S_OK;
    };
    if ms.writable {
        let (staging_buf, layouts) = match core.ensure_tex_staging() {
            Ok(x) => x,
            Err(e) => return d11_failed!(E_FAIL, "IDXGISurface::Unmap staging: {e}"),
        };
        let Some(lay) = layouts.get(sub as usize).copied() else {
            return d11_failed!(E_INVALIDARG, "IDXGISurface::Unmap: subresource {sub} out of range");
        };
        core.sync_other_devices(&surf.device);
        let mtl = core.current_mtl();
        {
            let mut exec = surf.device.imm_exec.lock().unwrap();
            if let Err(e) = exec.ensure_open() {
                return d11_failed!(E_FAIL, "IDXGISurface::Unmap ensure_open: {e}");
            }
            exec.cl.barrier();
            if let Err(e) = exec.cl.copy_texture_region(
                CopyLoc::Subresource(&mtl, sub),
                [0, 0, 0],
                CopyLoc::Footprint(&staging_buf, lay.footprint),
                None,
            ) {
                return d11_failed!(E_FAIL, "IDXGISurface::Unmap upload copy: {e}");
            }
            exec.cl.barrier();
            exec.has_work = true;
            exec.keep((mtl, staging_buf));
        }
        surf.device.flush_and_wait_idle();
    }
    S_OK
}
unsafe extern "win64" fn surf_get_dc(
    this: *mut Obj<DxgiSurface>,
    discard: BOOL,
    hdc_out: *mut *mut c_void,
) -> HRESULT {
    if !hdc_out.is_null() {
        std::ptr::write_unaligned(hdc_out, std::ptr::null_mut());
    }
    if hdc_out.is_null() {
        return E_INVALIDARG;
    }
    let surf = Obj::get(this);
    let core = &surf.core;
    let sub = surf.subresource;

    // Windows requires D3D11_RESOURCE_MISC_GDI_COMPATIBLE, non-MSAA, and a BGRA8 format
    // (DXGI_FORMAT_B8G8R8A8_UNORM=87, _TYPELESS=90, _UNORM_SRGB=91).
    let ResourceKind::Texture2D(td) = core.kind else {
        return d11_failed!(DXGI_ERROR_INVALID_CALL, "IDXGISurface1::GetDC: not a Texture2D");
    };
    if (td.MiscFlags & misc_flag::GDI_COMPATIBLE) == 0
        || td.SampleDesc.Count > 1
        || !matches!(td.Format, 87 | 90 | 91)
    {
        return d11_failed!(
            DXGI_ERROR_INVALID_CALL,
            "IDXGISurface1::GetDC: requires GDI_COMPATIBLE non-MSAA BGRA8 texture (fmt={}, samples={}, misc={:#x})",
            td.Format,
            td.SampleDesc.Count,
            td.MiscFlags
        );
    }

    let mips = td.MipLevels.max(1);
    let mip = sub % mips;
    let width = (td.Width >> mip).max(1);
    let height = (td.Height >> mip).max(1);

    // Reject concurrent GetDC or Map on the same subresource.
    {
        let dc_guard = surf.dc_state.lock().unwrap();
        if dc_guard.is_some() {
            return d11_failed!(DXGI_ERROR_INVALID_CALL, "IDXGISurface1::GetDC: DC already acquired");
        }
        let mut mapped = core.mapped.lock().unwrap();
        if mapped.contains_key(&sub) {
            return d11_failed!(
                DXGI_ERROR_INVALID_CALL,
                "IDXGISurface1::GetDC: subresource {sub} is already mapped"
            );
        }
        mapped.insert(
            sub,
            MapState {
                map_type: dxgi_map::READ | dxgi_map::WRITE,
                writable: true,
                deferred_scratch: None,
                row_pitch: 0,
                depth_pitch: 0,
            },
        );
    }

    let unreserve = || {
        core.mapped.lock().unwrap().remove(&sub);
    };

    let had_staging = core.tex_staging.lock().unwrap().is_some();
    let (staging_buf, layouts) = match core.ensure_tex_staging() {
        Ok(x) => x,
        Err(e) => {
            unreserve();
            return d11_failed!(E_FAIL, "IDXGISurface1::GetDC staging alloc: {e}");
        }
    };
    let Some(lay) = layouts.get(sub as usize).copied() else {
        unreserve();
        return d11_failed!(E_INVALIDARG, "IDXGISurface1::GetDC: subresource {sub} out of range");
    };

    core.sync_other_devices(&surf.device);

    if discard == 0 {
        let mtl = core.current_mtl();
        {
            let mut exec = surf.device.imm_exec.lock().unwrap();
            if let Err(e) = exec.ensure_open() {
                drop(exec);
                unreserve();
                return d11_failed!(E_FAIL, "IDXGISurface1::GetDC ensure_open: {e}");
            }
            exec.cl.barrier();
            if let Err(e) = exec.cl.copy_texture_region(
                CopyLoc::Footprint(&staging_buf, lay.footprint),
                [0, 0, 0],
                CopyLoc::Subresource(&mtl, sub),
                None,
            ) {
                drop(exec);
                unreserve();
                return d11_failed!(E_FAIL, "IDXGISurface1::GetDC readback copy: {e}");
            }
            exec.has_work = true;
            exec.keep((mtl, staging_buf.clone()));
        }
        surf.device.flush_and_wait_idle();
    } else if had_staging {
        surf.device.flush_and_wait_idle();
    }

    let Some(base_ptr) = staging_buf.cpu_ptr() else {
        unreserve();
        return d11_failed!(E_FAIL, "IDXGISurface1::GetDC: staging buffer has no cpu_ptr");
    };
    let ptr = base_ptr.add(lay.footprint.offset as usize);
    let stride = lay.footprint.row_pitch as usize;
    let byte_len = stride.saturating_mul(lay.num_rows as usize);
    let (hdc, hbmp) = crate::misc::gdi32::create_external_surface_dc(
        width as usize,
        height as usize,
        stride,
        ptr as usize,
        byte_len,
    );
    *surf.dc_state.lock().unwrap() = Some(SurfaceDcState { hdc, hbmp, width, height });
    // Per IDXGISurface1::GetDC contract, acquiring a DC unbinds the surface from the
    // output merger so OMGetRenderTargets and context-state swaps no longer observe it.
    surf.device.unbind_rtv_resource(core);
    std::ptr::write_unaligned(hdc_out, hdc as *mut c_void);
    S_OK
}

unsafe extern "win64" fn surf_release_dc(
    this: *mut Obj<DxgiSurface>,
    dirty: *const c_void,
) -> HRESULT {
    let surf = Obj::get(this);
    let Some(dc) = surf.dc_state.lock().unwrap().take() else {
        return d11_failed!(DXGI_ERROR_INVALID_CALL, "IDXGISurface1::ReleaseDC: no active DC");
    };
    crate::misc::gdi32::destroy_external_surface_dc(dc.hdc, dc.hbmp);
    let core = &surf.core;
    let sub = surf.subresource;
    core.mapped.lock().unwrap().remove(&sub);

    // Optional pDirtyRect: when specified and empty, skip GPU upload; otherwise clamp to surface.
    let (dst_xyz, src_box) = if !dirty.is_null() {
        let r = std::ptr::read_unaligned(dirty as *const crate::user32::RECT);
        let left = r.left.clamp(0, dc.width as i32) as u32;
        let top = r.top.clamp(0, dc.height as i32) as u32;
        let right = r.right.clamp(0, dc.width as i32) as u32;
        let bottom = r.bottom.clamp(0, dc.height as i32) as u32;
        if right <= left || bottom <= top {
            return S_OK;
        }
        (
            [left, top, 0],
            Some(d3d12_metal::metal::cmdlist::CopyBox {
                left,
                top,
                front: 0,
                right,
                bottom,
                back: 1,
            }),
        )
    } else {
        ([0, 0, 0], None)
    };

    let (staging_buf, layouts) = match core.ensure_tex_staging() {
        Ok(x) => x,
        Err(e) => return d11_failed!(E_FAIL, "IDXGISurface1::ReleaseDC staging: {e}"),
    };
    let Some(lay) = layouts.get(sub as usize).copied() else {
        return d11_failed!(E_INVALIDARG, "IDXGISurface1::ReleaseDC: subresource {sub} out of range");
    };
    core.sync_other_devices(&surf.device);
    let mtl = core.current_mtl();
    {
        let mut exec = surf.device.imm_exec.lock().unwrap();
        if let Err(e) = exec.ensure_open() {
            return d11_failed!(E_FAIL, "IDXGISurface1::ReleaseDC ensure_open: {e}");
        }
        exec.cl.barrier();
        if let Err(e) = exec.cl.copy_texture_region(
            CopyLoc::Subresource(&mtl, sub),
            dst_xyz,
            CopyLoc::Footprint(&staging_buf, lay.footprint),
            src_box,
        ) {
            return d11_failed!(E_FAIL, "IDXGISurface1::ReleaseDC upload copy: {e}");
        }
        exec.cl.barrier();
        exec.has_work = true;
        exec.keep((mtl, staging_buf));
    }
    surf.device.flush_and_wait_idle();
    S_OK
}
unsafe extern "win64" fn surf_get_resource(
    this: *mut Obj<DxgiSurface>,
    riid: *const Guid,
    out_res: *mut *mut c_void,
    out_sub: *mut u32,
) -> HRESULT {
    let surf = Obj::get(this);
    if !out_sub.is_null() {
        std::ptr::write_unaligned(out_sub, surf.subresource);
    }
    query_out(surf.tex_com as *mut c_void, riid, out_res)
}

/// Extract `(Arc<DeviceCore>, Arc<ResourceCore>)` from any D3D11 resource COM pointer.
pub unsafe fn resource_from_ptr_with_device(p: *mut c_void) -> Option<(Arc<DeviceCore>, Arc<ResourceCore>)> {
    if let Some(o) = Obj::<D3d11Buffer>::from_iface(p) {
        let r = Obj::get(o);
        return Some((r.device.clone(), r.core.clone()));
    }
    if let Some(o) = Obj::<D3d11Texture1D>::from_iface(p) {
        let r = Obj::get(o);
        return Some((r.device.clone(), r.core.clone()));
    }
    if let Some(o) = Obj::<D3d11Texture2D>::from_iface(p) {
        let r = Obj::get(o);
        return Some((r.device.clone(), r.core.clone()));
    }
    if let Some(o) = Obj::<D3d11Texture3D>::from_iface(p) {
        let r = Obj::get(o);
        return Some((r.device.clone(), r.core.clone()));
    }
    None
}

/// Extract the `Arc<ResourceCore>` from any D3D11 resource COM pointer.
pub unsafe fn resource_from_ptr(p: *mut c_void) -> Option<Arc<ResourceCore>> {
    resource_from_ptr_with_device(p).map(|(_, c)| c)
}

pub unsafe fn buffer_from_ptr(p: *mut c_void) -> Option<Arc<ResourceCore>> {
    Obj::<D3d11Buffer>::from_iface(p).map(|o| Obj::get(o).core.clone())
}

// ---------------------------------------------------------------------------
// Views: SRV, UAV, RTV, DSV (each 9 slots: 0..6 DeviceChild, 7 GetResource, 8 GetDesc)
// ---------------------------------------------------------------------------

pub const ID3D11_VIEW_SLOT_COUNT: usize = 9;

fn put_u32(dst: &mut [u8], off: usize, v: u32) {
    dst[off..off + 4].copy_from_slice(&v.to_le_bytes());
}
fn put_u64(dst: &mut [u8], off: usize, v: u64) {
    dst[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

const DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 0x1688;

fn default_srv_desc(res: &ResourceCore) -> Result<D3D11_SHADER_RESOURCE_VIEW_DESC, HRESULT> {
    match res.kind {
        ResourceKind::Buffer(b) => {
            if b.StructureByteStride > 0 {
                let count = b.ByteWidth / b.StructureByteStride;
                Ok(D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: 0,
                    ViewDimension: srv_dim::BUFFER,
                    u: [0, count, 0, 0],
                })
            } else {
                Err(d11_failed!(E_INVALIDARG, "CreateShaderResourceView: buffer without desc requires StructureByteStride"))
            }
        }
        ResourceKind::Texture1D(t) => Ok(if t.ArraySize > 1 {
            D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: t.Format,
                ViewDimension: srv_dim::TEXTURE1DARRAY,
                u: [0, t.MipLevels, 0, t.ArraySize],
            }
        } else {
            D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: t.Format,
                ViewDimension: srv_dim::TEXTURE1D,
                u: [0, t.MipLevels, 0, 0],
            }
        }),
        ResourceKind::Texture2D(t) => {
            let ms = t.SampleDesc.Count > 1;
            let cube = t.MiscFlags & misc_flag::TEXTURECUBE != 0;
            Ok(if cube && t.ArraySize > 6 {
                D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: t.Format,
                    ViewDimension: srv_dim::TEXTURECUBEARRAY,
                    u: [0, t.MipLevels, 0, t.ArraySize / 6],
                }
            } else if cube {
                D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: t.Format,
                    ViewDimension: srv_dim::TEXTURECUBE,
                    u: [0, t.MipLevels, 0, 0],
                }
            } else if ms && t.ArraySize > 1 {
                D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: t.Format,
                    ViewDimension: srv_dim::TEXTURE2DMSARRAY,
                    u: [0, t.ArraySize, 0, 0],
                }
            } else if ms {
                D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: t.Format,
                    ViewDimension: srv_dim::TEXTURE2DMS,
                    u: [0, 0, 0, 0],
                }
            } else if t.ArraySize > 1 {
                D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: t.Format,
                    ViewDimension: srv_dim::TEXTURE2DARRAY,
                    u: [0, t.MipLevels, 0, t.ArraySize],
                }
            } else {
                D3D11_SHADER_RESOURCE_VIEW_DESC {
                    Format: t.Format,
                    ViewDimension: srv_dim::TEXTURE2D,
                    u: [0, t.MipLevels, 0, 0],
                }
            })
        }
        ResourceKind::Texture3D(t) => Ok(D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: t.Format,
            ViewDimension: srv_dim::TEXTURE3D,
            u: [0, t.MipLevels, 0, 0],
        }),
    }
}

pub fn srv_desc_to_d3d12(d: &D3D11_SHADER_RESOURCE_VIEW_DESC, buf_stride: u32) -> [u8; 40] {
    let mut out = [0u8; 40];
    put_u32(&mut out, 0, d.Format);
    let dim12 = if d.ViewDimension == srv_dim::BUFFEREX { srv_dim::BUFFER } else { d.ViewDimension };
    put_u32(&mut out, 4, dim12);
    put_u32(&mut out, 8, DEFAULT_SHADER_4_COMPONENT_MAPPING);
    match d.ViewDimension {
        srv_dim::BUFFER => {
            put_u64(&mut out, 16, d.u[0] as u64);
            put_u32(&mut out, 24, d.u[1]);
            put_u32(&mut out, 28, buf_stride);
        }
        srv_dim::BUFFEREX => {
            let raw = d.u[2] & 1 != 0;
            put_u64(&mut out, 16, d.u[0] as u64);
            put_u32(&mut out, 24, d.u[1]);
            put_u32(&mut out, 28, if raw { 0 } else { buf_stride });
            put_u32(&mut out, 32, d.u[2] & 1);
        }
        srv_dim::TEXTURE1D | srv_dim::TEXTURE3D | srv_dim::TEXTURECUBE => {
            put_u32(&mut out, 16, d.u[0]);
            put_u32(&mut out, 20, d.u[1]);
        }
        srv_dim::TEXTURE1DARRAY | srv_dim::TEXTURECUBEARRAY => {
            put_u32(&mut out, 16, d.u[0]);
            put_u32(&mut out, 20, d.u[1]);
            put_u32(&mut out, 24, d.u[2]);
            put_u32(&mut out, 28, d.u[3]);
        }
        srv_dim::TEXTURE2D => {
            put_u32(&mut out, 16, d.u[0]);
            put_u32(&mut out, 20, d.u[1]);
            // PlaneSlice @24 = 0, MinLODClamp @28 = 0.0
        }
        srv_dim::TEXTURE2DARRAY => {
            put_u32(&mut out, 16, d.u[0]);
            put_u32(&mut out, 20, d.u[1]);
            put_u32(&mut out, 24, d.u[2]);
            put_u32(&mut out, 28, d.u[3]);
            // PlaneSlice @32 = 0, MinLODClamp @36 = 0.0
        }
        srv_dim::TEXTURE2DMSARRAY => {
            put_u32(&mut out, 16, d.u[0]);
            put_u32(&mut out, 20, d.u[1]);
        }
        _ => {}
    }
    out
}

fn default_uav_desc(res: &ResourceCore) -> Result<D3D11_UNORDERED_ACCESS_VIEW_DESC, HRESULT> {
    match res.kind {
        ResourceKind::Buffer(b) => {
            if b.StructureByteStride > 0 {
                Ok(D3D11_UNORDERED_ACCESS_VIEW_DESC {
                    Format: 0,
                    ViewDimension: uav_dim::BUFFER,
                    u: [0, b.ByteWidth / b.StructureByteStride, 0],
                })
            } else {
                Err(d11_failed!(E_INVALIDARG, "CreateUnorderedAccessView: buffer without desc requires StructureByteStride"))
            }
        }
        ResourceKind::Texture1D(t) => Ok(if t.ArraySize > 1 {
            D3D11_UNORDERED_ACCESS_VIEW_DESC { Format: t.Format, ViewDimension: uav_dim::TEXTURE1DARRAY, u: [0, 0, t.ArraySize] }
        } else {
            D3D11_UNORDERED_ACCESS_VIEW_DESC { Format: t.Format, ViewDimension: uav_dim::TEXTURE1D, u: [0, 0, 0] }
        }),
        ResourceKind::Texture2D(t) => Ok(if t.ArraySize > 1 {
            D3D11_UNORDERED_ACCESS_VIEW_DESC { Format: t.Format, ViewDimension: uav_dim::TEXTURE2DARRAY, u: [0, 0, t.ArraySize] }
        } else {
            D3D11_UNORDERED_ACCESS_VIEW_DESC { Format: t.Format, ViewDimension: uav_dim::TEXTURE2D, u: [0, 0, 0] }
        }),
        ResourceKind::Texture3D(t) => Ok(D3D11_UNORDERED_ACCESS_VIEW_DESC {
            Format: t.Format,
            ViewDimension: uav_dim::TEXTURE3D,
            u: [0, 0, t.Depth],
        }),
    }
}

pub fn uav_desc_to_d3d12(d: &D3D11_UNORDERED_ACCESS_VIEW_DESC, buf_stride: u32) -> [u8; 40] {
    let mut out = [0u8; 40];
    put_u32(&mut out, 0, d.Format);
    put_u32(&mut out, 4, d.ViewDimension);
    match d.ViewDimension {
        uav_dim::BUFFER => {
            let raw = d.u[2] & 1 != 0;
            put_u64(&mut out, 8, d.u[0] as u64);
            put_u32(&mut out, 16, d.u[1]);
            put_u32(&mut out, 20, if raw { 0 } else { buf_stride });
            put_u32(&mut out, 32, d.u[2] & 1);
        }
        uav_dim::TEXTURE1D | uav_dim::TEXTURE2D => {
            put_u32(&mut out, 8, d.u[0]);
        }
        uav_dim::TEXTURE1DARRAY | uav_dim::TEXTURE2DARRAY | uav_dim::TEXTURE3D => {
            put_u32(&mut out, 8, d.u[0]);
            put_u32(&mut out, 12, d.u[1]);
            put_u32(&mut out, 16, d.u[2]);
        }
        _ => {}
    }
    out
}

fn default_rtv_desc(res: &ResourceCore) -> Result<D3D11_RENDER_TARGET_VIEW_DESC, HRESULT> {
    match res.kind {
        ResourceKind::Texture1D(t) => Ok(if t.ArraySize > 1 {
            D3D11_RENDER_TARGET_VIEW_DESC { Format: t.Format, ViewDimension: rtv_dim::TEXTURE1DARRAY, u: [0, 0, t.ArraySize] }
        } else {
            D3D11_RENDER_TARGET_VIEW_DESC { Format: t.Format, ViewDimension: rtv_dim::TEXTURE1D, u: [0, 0, 0] }
        }),
        ResourceKind::Texture2D(t) => {
            let ms = t.SampleDesc.Count > 1;
            Ok(if ms && t.ArraySize > 1 {
                D3D11_RENDER_TARGET_VIEW_DESC { Format: t.Format, ViewDimension: rtv_dim::TEXTURE2DMSARRAY, u: [0, t.ArraySize, 0] }
            } else if ms {
                D3D11_RENDER_TARGET_VIEW_DESC { Format: t.Format, ViewDimension: rtv_dim::TEXTURE2DMS, u: [0, 0, 0] }
            } else if t.ArraySize > 1 {
                D3D11_RENDER_TARGET_VIEW_DESC { Format: t.Format, ViewDimension: rtv_dim::TEXTURE2DARRAY, u: [0, 0, t.ArraySize] }
            } else {
                D3D11_RENDER_TARGET_VIEW_DESC { Format: t.Format, ViewDimension: rtv_dim::TEXTURE2D, u: [0, 0, 0] }
            })
        }
        ResourceKind::Texture3D(t) => Ok(D3D11_RENDER_TARGET_VIEW_DESC {
            Format: t.Format,
            ViewDimension: rtv_dim::TEXTURE3D,
            u: [0, 0, t.Depth],
        }),
        ResourceKind::Buffer(_) => Err(d11_failed!(E_INVALIDARG, "CreateRenderTargetView: buffer RTV not supported")),
    }
}

fn default_dsv_desc(res: &ResourceCore) -> Result<D3D11_DEPTH_STENCIL_VIEW_DESC, HRESULT> {
    let fmt = match res.kind.format() {
        39 => 40, // R32_TYPELESS -> D32_FLOAT
        53 => 55, // R16_TYPELESS -> D16_UNORM
        44 => 45, // R24G8_TYPELESS -> D24_UNORM_S8_UINT
        19 => 20, // R32G8X24_TYPELESS -> D32_FLOAT_S8X24_UINT
        f => f,
    };
    match res.kind {
        ResourceKind::Texture1D(t) => Ok(if t.ArraySize > 1 {
            D3D11_DEPTH_STENCIL_VIEW_DESC { Format: fmt, ViewDimension: dsv_dim::TEXTURE1DARRAY, Flags: 0, u: [0, 0, t.ArraySize] }
        } else {
            D3D11_DEPTH_STENCIL_VIEW_DESC { Format: fmt, ViewDimension: dsv_dim::TEXTURE1D, Flags: 0, u: [0, 0, 0] }
        }),
        ResourceKind::Texture2D(t) => {
            let ms = t.SampleDesc.Count > 1;
            Ok(if ms && t.ArraySize > 1 {
                D3D11_DEPTH_STENCIL_VIEW_DESC { Format: fmt, ViewDimension: dsv_dim::TEXTURE2DMSARRAY, Flags: 0, u: [0, t.ArraySize, 0] }
            } else if ms {
                D3D11_DEPTH_STENCIL_VIEW_DESC { Format: fmt, ViewDimension: dsv_dim::TEXTURE2DMS, Flags: 0, u: [0, 0, 0] }
            } else if t.ArraySize > 1 {
                D3D11_DEPTH_STENCIL_VIEW_DESC { Format: fmt, ViewDimension: dsv_dim::TEXTURE2DARRAY, Flags: 0, u: [0, 0, t.ArraySize] }
            } else {
                D3D11_DEPTH_STENCIL_VIEW_DESC { Format: fmt, ViewDimension: dsv_dim::TEXTURE2D, Flags: 0, u: [0, 0, 0] }
            })
        }
        _ => Err(d11_failed!(E_INVALIDARG, "CreateDepthStencilView: only 1D/2D textures supported")),
    }
}

pub struct SrvInner {
    pub device: Arc<DeviceCore>,
    pub resource: Arc<ResourceCore>,
    pub res_com: usize,
    pub desc: D3D11_SHADER_RESOURCE_VIEW_DESC,
    pub d3d12_desc: [u8; 40],
    cache: Mutex<(usize, u32, views::Entry)>,
}

impl Drop for SrvInner {
    fn drop(&mut self) {
        unsafe { com_release(self.res_com as *mut c_void) };
    }
}

/// Patch D3D12 `ResourceMinLODClamp` in a 40-byte `D3D12_SHADER_RESOURCE_VIEW_DESC` from D3D11's
/// `ID3D11DeviceContext::SetResourceMinLOD` value.
fn patch_d3d12_srv_min_lod(desc: &mut [u8; 40], min_lod_bits: u32) {
    let dim = u32::from_le_bytes(desc[4..8].try_into().unwrap());
    let off = match dim {
        2 | 8 | 9 => 24,  // TEXTURE1D, TEXTURE3D, TEXTURECUBE
        3 | 10 => 32,     // TEXTURE1DARRAY, TEXTURECUBEARRAY
        4 => 28,          // TEXTURE2D
        5 => 36,          // TEXTURE2DARRAY
        _ => return,
    };
    desc[off..off + 4].copy_from_slice(&min_lod_bits.to_le_bytes());
}

impl SrvInner {
    pub fn entry(&self) -> Result<(Arc<MtlResource>, views::Entry), String> {
        let mtl = self.resource.current_mtl();
        let ptr = Arc::as_ptr(&mtl) as usize;
        let min_lod_bits = self.resource.min_lod_bits.load(Ordering::Relaxed);
        let mut g = self.cache.lock().unwrap();
        if g.0 == ptr && g.1 == min_lod_bits {
            return Ok((mtl, g.2));
        }
        let e = if min_lod_bits == 0 {
            views::srv_entry(Some(&mtl), Some(&self.d3d12_desc))?
        } else {
            let mut d = self.d3d12_desc;
            patch_d3d12_srv_min_lod(&mut d, min_lod_bits);
            views::srv_entry(Some(&mtl), Some(&d))?
        };
        *g = (ptr, min_lod_bits, e);
        Ok((mtl, e))
    }
}

pub struct D3d11Srv {
    pub inner: Arc<SrvInner>,
}

impl HasDeviceCore for D3d11Srv {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

pub struct UavInner {
    pub device: Arc<DeviceCore>,
    pub resource: Arc<ResourceCore>,
    pub res_com: usize,
    pub desc: D3D11_UNORDERED_ACCESS_VIEW_DESC,
    pub d3d12_desc: [u8; 40],
    /// Dedicated 32-bit GPU structure counter buffer when `D3D11_BUFFER_UAV_FLAG_APPEND`
    /// or `D3D11_BUFFER_UAV_FLAG_COUNTER` is set.
    pub counter: Option<Arc<MtlResource>>,
    cache: Mutex<(usize, views::Entry)>,
}

impl Drop for UavInner {
    fn drop(&mut self) {
        unsafe { com_release(self.res_com as *mut c_void) };
    }
}

impl UavInner {
    pub fn entry(&self) -> Result<(Arc<MtlResource>, views::Entry), String> {
        let mtl = self.resource.current_mtl();
        let ptr = Arc::as_ptr(&mtl) as usize;
        let mut g = self.cache.lock().unwrap();
        if g.0 == ptr {
            return Ok((mtl, g.1));
        }
        let counter_arg = self.counter.as_deref().map(|c| (c, 0u64));
        let e = views::uav_entry_with_counter(Some(&mtl), counter_arg, Some(&self.d3d12_desc))?;
        *g = (ptr, e);
        Ok((mtl, e))
    }
}

pub struct D3d11Uav {
    pub inner: Arc<UavInner>,
}

impl HasDeviceCore for D3d11Uav {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

pub struct RtvInner {
    pub device: Arc<DeviceCore>,
    pub resource: Arc<ResourceCore>,
    pub res_com: usize,
    pub desc: D3D11_RENDER_TARGET_VIEW_DESC,
}

impl Drop for RtvInner {
    fn drop(&mut self) {
        unsafe { com_release(self.res_com as *mut c_void) };
    }
}

impl RtvInner {
    pub fn target_view(&self) -> Result<(Arc<MtlResource>, TargetView), String> {
        let mtl = self.resource.current_mtl();
        let words = [
            self.desc.Format,
            self.desc.ViewDimension,
            self.desc.u[0],
            self.desc.u[1],
            self.desc.u[2],
            0,
        ];
        let tv = TargetView::rtv(&mtl, Some(&words))?;
        Ok((mtl, tv))
    }

    pub fn format(&self) -> u32 {
        if self.desc.Format != 0 {
            self.desc.Format
        } else {
            self.resource.kind.format()
        }
    }
}

pub struct D3d11Rtv {
    pub inner: Arc<RtvInner>,
}

impl HasDeviceCore for D3d11Rtv {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

pub struct DsvInner {
    pub device: Arc<DeviceCore>,
    pub resource: Arc<ResourceCore>,
    pub res_com: usize,
    pub desc: D3D11_DEPTH_STENCIL_VIEW_DESC,
}

impl Drop for DsvInner {
    fn drop(&mut self) {
        unsafe { com_release(self.res_com as *mut c_void) };
    }
}

impl DsvInner {
    pub fn target_view(&self) -> Result<(Arc<MtlResource>, TargetView), String> {
        let mtl = self.resource.current_mtl();
        let words = [
            self.desc.Format,
            self.desc.ViewDimension,
            self.desc.Flags,
            self.desc.u[0],
            self.desc.u[1],
            self.desc.u[2],
        ];
        let tv = TargetView::dsv(&mtl, Some(&words))?;
        Ok((mtl, tv))
    }

    pub fn format(&self) -> u32 {
        if self.desc.Format != 0 {
            self.desc.Format
        } else {
            self.resource.kind.format()
        }
    }
}

pub struct D3d11Dsv {
    pub inner: Arc<DsvInner>,
}

impl HasDeviceCore for D3d11Dsv {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

unsafe extern "win64" fn srv_get_res(this: *mut Obj<D3d11Srv>, out: *mut *mut c_void) {
    if !out.is_null() {
        let p = Obj::get(this).inner.res_com as *mut c_void;
        com_add_ref(p);
        std::ptr::write_unaligned(out, p);
    }
}
unsafe extern "win64" fn srv_get_desc(this: *mut Obj<D3d11Srv>, out: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc);
    }
}

unsafe extern "win64" fn uav_get_res(this: *mut Obj<D3d11Uav>, out: *mut *mut c_void) {
    if !out.is_null() {
        let p = Obj::get(this).inner.res_com as *mut c_void;
        com_add_ref(p);
        std::ptr::write_unaligned(out, p);
    }
}
unsafe extern "win64" fn uav_get_desc(this: *mut Obj<D3d11Uav>, out: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc);
    }
}

unsafe extern "win64" fn rtv_get_res(this: *mut Obj<D3d11Rtv>, out: *mut *mut c_void) {
    if !out.is_null() {
        let p = Obj::get(this).inner.res_com as *mut c_void;
        com_add_ref(p);
        std::ptr::write_unaligned(out, p);
    }
}
unsafe extern "win64" fn rtv_get_desc(this: *mut Obj<D3d11Rtv>, out: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc);
    }
}

unsafe extern "win64" fn dsv_get_res(this: *mut Obj<D3d11Dsv>, out: *mut *mut c_void) {
    if !out.is_null() {
        let p = Obj::get(this).inner.res_com as *mut c_void;
        com_add_ref(p);
        std::ptr::write_unaligned(out, p);
    }
}
unsafe extern "win64" fn dsv_get_desc(this: *mut Obj<D3d11Dsv>, out: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc);
    }
}

static SRV_CLASS: OnceLock<Class> = OnceLock::new();
static UAV_CLASS: OnceLock<Class> = OnceLock::new();
static RTV_CLASS: OnceLock<Class> = OnceLock::new();
static DSV_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11Srv {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&SRV_CLASS, || {
            let mut s = child_slots::<D3d11Srv>();
            s.push(("GetResource", Some(srv_get_res as *const ())));
            s.push(("GetDesc", Some(srv_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_VIEW_SLOT_COUNT);
            Class {
                name: "ID3D11ShaderResourceView",
                vtbl: build_vtable("ID3D11ShaderResourceView", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11View, IID_ID3D11ShaderResourceView],
            }
        })
    }
}

impl ComClass for D3d11Uav {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&UAV_CLASS, || {
            let mut s = child_slots::<D3d11Uav>();
            s.push(("GetResource", Some(uav_get_res as *const ())));
            s.push(("GetDesc", Some(uav_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_VIEW_SLOT_COUNT);
            Class {
                name: "ID3D11UnorderedAccessView",
                vtbl: build_vtable("ID3D11UnorderedAccessView", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11View, IID_ID3D11UnorderedAccessView],
            }
        })
    }
}

impl ComClass for D3d11Rtv {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&RTV_CLASS, || {
            let mut s = child_slots::<D3d11Rtv>();
            s.push(("GetResource", Some(rtv_get_res as *const ())));
            s.push(("GetDesc", Some(rtv_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_VIEW_SLOT_COUNT);
            Class {
                name: "ID3D11RenderTargetView",
                vtbl: build_vtable("ID3D11RenderTargetView", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11View, IID_ID3D11RenderTargetView],
            }
        })
    }
}

impl ComClass for D3d11Dsv {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&DSV_CLASS, || {
            let mut s = child_slots::<D3d11Dsv>();
            s.push(("GetResource", Some(dsv_get_res as *const ())));
            s.push(("GetDesc", Some(dsv_get_desc as *const ())));
            debug_assert_eq!(s.len(), ID3D11_VIEW_SLOT_COUNT);
            Class {
                name: "ID3D11DepthStencilView",
                vtbl: build_vtable("ID3D11DepthStencilView", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11View, IID_ID3D11DepthStencilView],
            }
        })
    }
}

pub unsafe fn create_srv(
    res_ptr: *mut c_void,
    desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let Some((device, res)) = resource_from_ptr_with_device(res_ptr) else {
        return d11_failed!(E_INVALIDARG, "CreateShaderResourceView: null/foreign resource {res_ptr:?}");
    };
    let mut d = if desc.is_null() {
        match default_srv_desc(&res) {
            Ok(d) => d,
            Err(hr) => return hr,
        }
    } else {
        std::ptr::read_unaligned(desc)
    };
    if d.Format == 0 {
        d.Format = res.kind.format();
    }
    let d12 = srv_desc_to_d3d12(&d, res.kind.buffer_stride());
    let mtl = res.current_mtl();
    let min_lod_bits = res.min_lod_bits.load(Ordering::Relaxed);
    let initial = if min_lod_bits == 0 {
        views::srv_entry(Some(&mtl), Some(&d12))
    } else {
        let mut patched = d12;
        patch_d3d12_srv_min_lod(&mut patched, min_lod_bits);
        views::srv_entry(Some(&mtl), Some(&patched))
    };
    let initial = match initial {
        Ok(e) => e,
        Err(e) => return d11_failed!(E_INVALIDARG, "CreateShaderResourceView({d:?}): {e}"),
    };
    if out.is_null() {
        return S_FALSE;
    }
    com_add_ref(res_ptr);
    let inner = Arc::new(SrvInner {
        device,
        resource: res,
        res_com: res_ptr as usize,
        desc: d,
        d3d12_desc: d12,
        cache: Mutex::new((Arc::as_ptr(&mtl) as usize, min_lod_bits, initial)),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11Srv { inner }) as *mut c_void);
    S_OK
}

pub unsafe fn create_uav(
    res_ptr: *mut c_void,
    desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let Some((device, res)) = resource_from_ptr_with_device(res_ptr) else {
        return d11_failed!(E_INVALIDARG, "CreateUnorderedAccessView: null/foreign resource {res_ptr:?}");
    };
    let mut d = if desc.is_null() {
        match default_uav_desc(&res) {
            Ok(d) => d,
            Err(hr) => return hr,
        }
    } else {
        std::ptr::read_unaligned(desc)
    };
    if d.Format == 0 {
        d.Format = res.kind.format();
    }
    let d12 = uav_desc_to_d3d12(&d, res.kind.buffer_stride());
    let has_counter = d.ViewDimension == uav_dim::BUFFER
        && (d.u[2] & (uav_flag::APPEND | uav_flag::COUNTER)) != 0;
    let counter = if has_counter {
        let cdesc = ResourceDesc {
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
        let c = match new_committed_resource(&cdesc, &hp, "d3d11 uav counter") {
            Ok(r) => Arc::new(r),
            Err(CreateError::OutOfMemory(e)) => {
                return d11_failed!(E_OUTOFMEMORY, "CreateUnorderedAccessView counter: {e}");
            }
            Err(e) => {
                return d11_failed!(E_FAIL, "CreateUnorderedAccessView counter: {e:?}");
            }
        };
        c.register_ptr();
        Some(c)
    } else {
        None
    };
    let mtl = res.current_mtl();
    let initial = match views::uav_entry_with_counter(
        Some(&mtl),
        counter.as_deref().map(|c| (c, 0u64)),
        Some(&d12),
    ) {
        Ok(e) => e,
        Err(e) => return d11_failed!(E_INVALIDARG, "CreateUnorderedAccessView({d:?}): {e}"),
    };
    if out.is_null() {
        return S_FALSE;
    }
    com_add_ref(res_ptr);
    let inner = Arc::new(UavInner {
        device,
        resource: res,
        res_com: res_ptr as usize,
        desc: d,
        d3d12_desc: d12,
        counter,
        cache: Mutex::new((Arc::as_ptr(&mtl) as usize, initial)),
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11Uav { inner }) as *mut c_void);
    S_OK
}

pub unsafe fn create_rtv(
    res_ptr: *mut c_void,
    desc: *const D3D11_RENDER_TARGET_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let Some((device, res)) = resource_from_ptr_with_device(res_ptr) else {
        return d11_failed!(E_INVALIDARG, "CreateRenderTargetView: null/foreign resource {res_ptr:?}");
    };
    let mut d = if desc.is_null() {
        match default_rtv_desc(&res) {
            Ok(d) => d,
            Err(hr) => return hr,
        }
    } else {
        std::ptr::read_unaligned(desc)
    };
    if d.Format == 0 {
        d.Format = res.kind.format();
    }
    // Validate with TargetView::rtv right away.
    let mtl = res.current_mtl();
    let words = [d.Format, d.ViewDimension, d.u[0], d.u[1], d.u[2], 0];
    if let Err(e) = TargetView::rtv(&mtl, Some(&words)) {
        return d11_failed!(E_INVALIDARG, "CreateRenderTargetView({d:?}): {e}");
    }
    if out.is_null() {
        return S_FALSE;
    }
    com_add_ref(res_ptr);
    let inner = Arc::new(RtvInner { device, resource: res, res_com: res_ptr as usize, desc: d });
    std::ptr::write_unaligned(out, Obj::new(D3d11Rtv { inner }) as *mut c_void);
    S_OK
}

pub unsafe fn create_dsv(
    res_ptr: *mut c_void,
    desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let Some((device, res)) = resource_from_ptr_with_device(res_ptr) else {
        return d11_failed!(E_INVALIDARG, "CreateDepthStencilView: null/foreign resource {res_ptr:?}");
    };
    let mut d = if desc.is_null() {
        match default_dsv_desc(&res) {
            Ok(d) => d,
            Err(hr) => return hr,
        }
    } else {
        std::ptr::read_unaligned(desc)
    };
    if d.Format == 0 {
        d.Format = default_dsv_desc(&res).map(|x| x.Format).unwrap_or(res.kind.format());
    }
    let mtl = res.current_mtl();
    let words = [d.Format, d.ViewDimension, d.Flags, d.u[0], d.u[1], d.u[2]];
    if let Err(e) = TargetView::dsv(&mtl, Some(&words)) {
        return d11_failed!(E_INVALIDARG, "CreateDepthStencilView({d:?}): {e}");
    }
    if out.is_null() {
        return S_FALSE;
    }
    com_add_ref(res_ptr);
    let inner = Arc::new(DsvInner { device, resource: res, res_com: res_ptr as usize, desc: d });
    std::ptr::write_unaligned(out, Obj::new(D3d11Dsv { inner }) as *mut c_void);
    S_OK
}
