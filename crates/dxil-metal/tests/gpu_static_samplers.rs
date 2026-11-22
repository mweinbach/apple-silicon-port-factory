//! GPU check of the static-sampler binding model: converts our own fixture compute shader
//! (`fixtures/static_samplers.hlsl`) with root signatures that declare s0/s1 either as a sampler
//! descriptor table or as static samplers (declared in reverse register order), runs it on Metal
//! with the top-level argument buffer built by `RootSignatureLayout`/`TopLevelArgumentBuffer`, and
//! compares every sample against a CPU reference of the address modes.
//!
//! Skips (passes) without a Metal device or the converter. Uses the Metal 3 API on purpose: this
//! tests the converter's binding layout, not the D3D12 layer's submission path.
#![cfg(target_os = "macos")]

use std::ptr::NonNull;

use dxil_metal::runtime::{DescriptorTableEntry, ARGUMENT_BUFFER_BIND_POINT};
use dxil_metal::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSString, NSURL};
use objc2_metal::*;

const FIXTURE: &[u8] = include_bytes!("fixtures/static_samplers.cs.dxil");
const THREADS: usize = 16;
const TEX_W: usize = 4;
/// Distinct texels (RGBA32Float, 4x1).
const TEXELS: [[f32; 4]; TEX_W] = [[1.0, 0.0, 0.0, 1.0], [0.0, 2.0, 0.0, 1.0], [0.0, 0.0, 3.0, 1.0], [4.0, 4.0, 4.0, 1.0]];

// D3D12_TEXTURE_ADDRESS_MODE
const WRAP: u32 = 1;
const MIRROR: u32 = 2;
const CLAMP: u32 = 3;

fn fixture_dxil() -> Vec<u8> {
    // The fixture is written unsigned by our DXC driver; sign it like the game's shaders are.
    let mut b = FIXTURE.to_vec();
    let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
    let sum = hash::dxbc_checksum(&b[20..size]);
    b[4..20].copy_from_slice(&sum);
    b
}

/// CPU reference: texel index for point sampling at `u` with a D3D12 address mode.
fn texel_index(u: f32, mode: u32) -> usize {
    let w = TEX_W as f32;
    let i = match mode {
        CLAMP => (u * w).floor().clamp(0.0, w - 1.0),
        WRAP => ((u - u.floor()) * w).floor(),
        MIRROR => {
            let t = u.rem_euclid(2.0);
            let t = if t > 1.0 { 2.0 - t } else { t };
            (t * w).floor().min(w - 1.0)
        }
        _ => unreachable!(),
    };
    i as usize
}

fn shader_u(id: usize) -> f32 {
    -1.0 + (id as f32 + 0.5) / THREADS as f32 * 3.0
}

fn mtl_address_mode(mode: u32) -> MTLSamplerAddressMode {
    match mode {
        WRAP => MTLSamplerAddressMode::Repeat,
        MIRROR => MTLSamplerAddressMode::MirrorRepeat,
        CLAMP => MTLSamplerAddressMode::ClampToEdge,
        _ => unreachable!(),
    }
}

fn resource_id(id: MTLResourceID) -> u64 {
    // MTLResourceID is a repr(C) struct around one u64 (`_impl`).
    unsafe { std::mem::transmute::<MTLResourceID, u64>(id) }
}

struct Gpu {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    texture: Retained<ProtocolObject<dyn MTLTexture>>,
}

impl Gpu {
    fn new() -> Option<Self> {
        let device = MTLCreateSystemDefaultDevice()?;
        let queue = device.newCommandQueue()?;
        let desc = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                MTLPixelFormat::RGBA32Float,
                TEX_W,
                1,
                false,
            )
        };
        desc.setUsage(MTLTextureUsage::ShaderRead);
        desc.setStorageMode(MTLStorageMode::Shared);
        let texture = device.newTextureWithDescriptor(&desc)?;
        let region = MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: TEX_W, height: 1, depth: 1 } };
        unsafe {
            texture.replaceRegion_mipmapLevel_withBytes_bytesPerRow(
                region,
                0,
                NonNull::new(TEXELS.as_ptr() as *mut _).unwrap(),
                TEX_W * 16,
            );
        }
        Some(Self { device, queue, texture })
    }

    fn sampler(&self, mode: u32) -> Retained<ProtocolObject<dyn MTLSamplerState>> {
        let d = MTLSamplerDescriptor::new();
        d.setMinFilter(MTLSamplerMinMagFilter::Nearest);
        d.setMagFilter(MTLSamplerMinMagFilter::Nearest);
        d.setSAddressMode(mtl_address_mode(mode));
        d.setTAddressMode(mtl_address_mode(mode));
        d.setRAddressMode(mtl_address_mode(mode));
        d.setSupportArgumentBuffers(true);
        self.device.newSamplerStateWithDescriptor(&d).expect("sampler")
    }

    fn buffer(&self, bytes: &[u8]) -> Retained<ProtocolObject<dyn MTLBuffer>> {
        let len = bytes.len().max(16);
        let b = self.device.newBufferWithLength_options(len, MTLResourceOptions::StorageModeShared).expect("buffer");
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), b.contents().as_ptr() as *mut u8, bytes.len()) };
        b
    }

    fn pipeline(&self, shader: &CompiledShader) -> Retained<ProtocolObject<dyn MTLComputePipelineState>> {
        let path = std::env::temp_dir().join(format!("aw2-gpu-static-samplers-{}-{:032x}.metallib", std::process::id(), shader.key));
        std::fs::write(&path, &shader.metallib).unwrap();
        let url = NSURL::fileURLWithPath(&NSString::from_str(path.to_str().unwrap()));
        let lib = self.device.newLibraryWithURL_error(&url).expect("newLibraryWithURL");
        let _ = std::fs::remove_file(&path);
        let f = lib.newFunctionWithName(&NSString::from_str(&shader.function_name)).expect("function");
        self.device.newComputePipelineStateWithFunction_error(&f).expect("compute pipeline")
    }

    /// Runs the shader; `sampler_heap` is the content of the table bound to the sampler slot
    /// (descriptor table param 2, or the static sampler table). Returns 2*THREADS texels.
    fn run(
        &self,
        rs: &RootSignature,
        shader: &CompiledShader,
        sampler_heap: &[Retained<ProtocolObject<dyn MTLSamplerState>>],
    ) -> Vec<[f32; 4]> {
        let pso = self.pipeline(shader);
        let layout = rs.layout();
        let srv_heap = self.buffer(&DescriptorTableEntry::texture(resource_id(self.texture.gpuResourceID()), 0.0, 0).to_bytes());
        let smp_bytes: Vec<u8> =
            sampler_heap.iter().flat_map(|s| DescriptorTableEntry::sampler(resource_id(s.gpuResourceID()), 0.0).to_bytes()).collect();
        let smp_heap = self.buffer(&smp_bytes);
        let out = self.buffer(&vec![0u8; THREADS * 2 * 16]);

        let mut tlab = layout.new_argument_buffer();
        tlab.set_descriptor_table(layout, 0, srv_heap.gpuAddress());
        tlab.set_root_descriptor(layout, 1, out.gpuAddress());
        if layout.static_samplers.is_some() {
            tlab.set_static_sampler_table(layout, smp_heap.gpuAddress());
        } else {
            tlab.set_descriptor_table(layout, 2, smp_heap.gpuAddress());
        }
        let tlab_buf = self.buffer(&tlab.bytes);

        let cb = self.queue.commandBuffer().expect("command buffer");
        let enc = cb.computeCommandEncoder().expect("encoder");
        enc.setComputePipelineState(&pso);
        unsafe { enc.setBuffer_offset_atIndex(Some(&tlab_buf), 0, ARGUMENT_BUFFER_BIND_POINT as usize) };
        enc.useResource_usage(ProtocolObject::from_ref(&*self.texture), MTLResourceUsage::Read);
        enc.useResource_usage(ProtocolObject::from_ref(&*srv_heap), MTLResourceUsage::Read);
        enc.useResource_usage(ProtocolObject::from_ref(&*smp_heap), MTLResourceUsage::Read);
        enc.useResource_usage(ProtocolObject::from_ref(&*out), MTLResourceUsage::Read | MTLResourceUsage::Write);
        enc.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize { width: 1, height: 1, depth: 1 },
            MTLSize { width: THREADS, height: 1, depth: 1 },
        );
        enc.endEncoding();
        cb.commit();
        cb.waitUntilCompleted();
        assert_eq!(cb.status(), MTLCommandBufferStatus::Completed, "command buffer error: {:?}", cb.error());

        let p = out.contents().as_ptr() as *const [f32; 4];
        (0..THREADS * 2).map(|i| unsafe { *p.add(i) }).collect()
    }
}

fn range(range_type: DescriptorRangeType, reg: u32, n: u32) -> DescriptorRange {
    DescriptorRange {
        range_type,
        num_descriptors: n,
        base_shader_register: reg,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: root_signature::flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
    }
}

fn root_signature(sampler_table: bool) -> std::sync::Arc<RootSignature> {
    let mut parameters = vec![
        RootParameter { kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Srv, 0, 1)]), visibility: 0 },
        RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Uav,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 0,
        },
    ];
    let mut static_samplers = vec![];
    if sampler_table {
        parameters.push(RootParameter {
            kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, 0, 2)]),
            visibility: 0,
        });
    } else {
        // Declared in reverse register order: s1 = clamp first, then s0 = mirror.
        let s = |reg, mode| StaticSampler {
            filter: 0, // MIN_MAG_MIP_POINT
            address_u: mode,
            address_v: mode,
            address_w: mode,
            shader_register: reg,
            ..Default::default()
        };
        static_samplers = vec![s(1, CLAMP), s(0, MIRROR)];
    }
    RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters,
        static_samplers,
        flags: 0,
    })
    .expect("root signature")
}

/// Compares the GPU output against s0 = MIRROR, s1 = CLAMP; returns mismatching sample indices.
fn mismatches(out: &[[f32; 4]]) -> Vec<String> {
    let mut bad = vec![];
    for id in 0..THREADS {
        let u = shader_u(id);
        for (k, mode) in [MIRROR, CLAMP].into_iter().enumerate() {
            let want = TEXELS[texel_index(u, mode)];
            let got = out[id * 2 + k];
            if got != want {
                bad.push(format!("thread {id} s{k} u={u}: got {got:?}, want {want:?}"));
            }
        }
    }
    bad
}

#[test]
fn static_samplers_on_gpu() {
    let Some(gpu) = Gpu::new() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    if !converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return;
    }
    let dxil = fixture_dxil();
    let opts = CompileOptions::default();
    let compile = |rs: &RootSignature| {
        compile_uncached(&CompileRequest::new(&dxil, Some(rs), &opts).stage(ShaderStage::Compute)).expect("convert fixture")
    };

    // The CPU reference must distinguish the modes we use, or the test proves nothing.
    assert!((0..THREADS).any(|i| texel_index(shader_u(i), MIRROR) != texel_index(shader_u(i), CLAMP)));
    assert!((0..THREADS).any(|i| texel_index(shader_u(i), MIRROR) != texel_index(shader_u(i), WRAP)));

    // A: sampler descriptor table [s0 = mirror, s1 = clamp] — validates the harness itself.
    let rs_a = root_signature(true);
    let sh_a = compile(&rs_a);
    let out = gpu.run(&rs_a, &sh_a, &[gpu.sampler(MIRROR), gpu.sampler(CLAMP)]);
    let bad = mismatches(&out);
    assert!(bad.is_empty(), "sampler table baseline failed:\n{}", bad.join("\n"));

    // Static samplers declared [s1 = clamp, s0 = mirror].
    let rs_s = root_signature(false);
    assert_eq!(rs_s.layout().static_samplers.map(|t| t.count), Some(2));
    let sh_s = compile(&rs_s);
    let clamp = gpu.sampler(CLAMP);
    let mirror = gpu.sampler(MIRROR);
    let wrap = gpu.sampler(WRAP);
    let decl_order = mismatches(&gpu.run(&rs_s, &sh_s, &[clamp.clone(), mirror.clone()]));
    let reg_order = mismatches(&gpu.run(&rs_s, &sh_s, &[mirror.clone(), clamp.clone()]));
    let junk = mismatches(&gpu.run(&rs_s, &sh_s, &[wrap.clone(), wrap.clone()]));
    eprintln!(
        "static samplers: declaration-order table ok={}, register-order table ok={}, junk table ok={}",
        decl_order.is_empty(),
        reg_order.is_empty(),
        junk.is_empty()
    );
    // The documented contract (StaticSamplerTableLayout): one sampler per static sampler, in
    // root-signature declaration order.
    assert!(decl_order.is_empty(), "declaration-order static sampler table failed:\n{}", decl_order.join("\n"));
    assert!(!reg_order.is_empty(), "register order also matched; the test can't tell the orders apart");
    assert!(!junk.is_empty(), "converter ignored the static sampler table (samplers embedded in the shader?)");
}
