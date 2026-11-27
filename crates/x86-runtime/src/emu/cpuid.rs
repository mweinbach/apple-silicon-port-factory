//! The guest CPU contract, queried at execution time (including from precompiled code).

use super::features::{host, HostFeatures};
use super::Cpu;

pub(super) const XCR0: u32 = 7; // x87, SSE, AVX upper halves
pub(super) const YMM_OFFSET: usize = 576;
pub(super) const XSAVE_SIZE: usize = YMM_OFFSET + 16 * 16;

pub(crate) fn query(leaf: u32, subleaf: u32) -> [u32; 4] {
    let topo = win32_rt::process::cpu_topology();
    let cores = topo.guest_logical.clamp(1, 64);
    let apic = || unsafe { win32_rt::kernel32::threads::GetCurrentProcessorNumber() } % cores;
    let features = host();
    match (leaf, subleaf) {
        (0, _) => [0x1f, 0x756e_6547, 0x6c65_746e, 0x4965_6e69], // GenuineIntel
        (1, _) => {
            // Advertise only extensions whose instructions and XSAVE/YMM state are implemented.
            let ecx = 1 // SSE3
                | (u32::from(features.has(HostFeatures::PMULL)) << 1)
                | (1 << 9) | (1 << 12) | (1 << 13) | (1 << 19) // SSSE3, FMA, CMPXCHG16B, SSE4.1
                | (u32::from(features.has(HostFeatures::CRC32)) << 20) // SSE4.2
                | (1 << 22) | (1 << 23) // MOVBE, POPCNT
                | (u32::from(features.has(HostFeatures::AES)) << 25)
                | (1 << 26) | (1 << 27) | (1 << 28) | (1 << 29) // XSAVE, OSXSAVE, AVX, F16C
                | (1 << 30); // RDRAND (OS CSPRNG via getentropy)
            let edx = (1 << 4) | (1 << 8) | (1 << 15) | (1 << 23) | (1 << 24) | (1 << 25) | (1 << 26) // TSC, CX8, CMOV, MMX, FXSR, SSE, SSE2
                | (u32::from(cores > 1) << 28);
            [0x0009_06ea, (apic() << 24) | (cores << 16), ecx, edx]
        }
        (4, 0..=3) => {
            // Match the virtual caches and single-package, no-SMT layout in Win32 sysinfo.
            let line = topo.cache_line_bytes.max(64);
            let ways = 8u32;
            let (kind, level, size, sharing) = match subleaf {
                0 => (1, 1, topo.l1d_bytes, 1),
                1 => (2, 1, topo.l1i_bytes, 1),
                2 => (3, 2, topo.l2_bytes, cores),
                _ => (3, 3, topo.l3_bytes, cores),
            };
            let sets = (size / (ways * line)).max(1);
            let eax = kind | (level << 5) | (1 << 8) | ((sharing - 1) << 14) | ((cores - 1) << 26);
            [eax, ((ways - 1) << 22) | (line - 1), sets - 1, 0]
        }
        (7, 0) => {
            let ebx = (1 << 3) | (1 << 5) | (1 << 8) | (1 << 9) | (1 << 13) // BMI1, AVX2, BMI2, ERMS, deprecated FCS/FDS
                | (1 << 18) | (1 << 19) // RDSEED, ADX
                | (u32::from(features.has(HostFeatures::SHA1 | HostFeatures::SHA256)) << 29);
            let ecx = (1 << 8) // GFNI (legacy + VEX 128/256)
                | (u32::from(features.has(HostFeatures::AES)) << 9) // VAES (VEX 256)
                | (u32::from(features.has(HostFeatures::PMULL)) << 10); // VPCLMULQDQ (VEX 256)
            let edx = 1 << 14; // SERIALIZE
            [1, ebx, ecx, edx]
        }
        (7, 1) => [1 << 4, 0, 0, 0], // AVX-VNNI (VEX 128/256)
        (0xb | 0x1f, 0) => [0, 1, 1 << 8, apic()],
        (0xb | 0x1f, 1) => [cores.next_power_of_two().ilog2(), cores, (2 << 8) | 1, apic()],
        (0xb | 0x1f, _) => [0, 0, subleaf & 0xff, apic()],
        (0xd, 0) => [XCR0, XSAVE_SIZE as u32, XSAVE_SIZE as u32, 0],
        (0xd, 1) => [3, XSAVE_SIZE as u32, 0, 0], // XSAVEOPT and XSAVEC, no supervisor components
        (0xd, 2) => [256, YMM_OFFSET as u32, 0, 0],
        (0x8000_0000, _) => [0x8000_0008, 0, 0, 0],
        (0x8000_0001, _) => [0, 0, 0x121, (1 << 11) | (1 << 20) | (1 << 27) | (1 << 29)],
        (0x8000_0002..=0x8000_0004, _) => {
            const BRAND: &[u8; 48] = b"Apple Silicon x86-64 Translation Runtime       \0";
            let start = (leaf - 0x8000_0002) as usize * 16;
            std::array::from_fn(|i| {
                let p = start + i * 4;
                u32::from_le_bytes(BRAND[p..p + 4].try_into().unwrap())
            })
        }
        (0x8000_0008, _) => {
            let apic_bits = cores.next_power_of_two().ilog2().min(15);
            let ecx = (apic_bits << 12) | ((cores - 1) & 0xff);
            [48 << 8 | 48, 0, ecx, 0]
        }
        _ => [0; 4],
    }
}

impl Cpu {
    pub(super) fn cpuid(&mut self) {
        let [eax, ebx, ecx, edx] = query(self.gpr[0] as u32, self.gpr[1] as u32);
        self.gpr[..4].copy_from_slice(&[eax.into(), ecx.into(), edx.into(), ebx.into()]);
        self.last_bits[..4].fill(64);
    }
}

#[cfg(all(test, target_arch = "aarch64"))]
#[path = "cpuid_audit_tests.rs"]
mod audit_tests;
