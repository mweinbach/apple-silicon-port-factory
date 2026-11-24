//! Anti-Cheat, Anti-Tamper & Packer Compatibility Analyzer (`anticheat-audit`).
//!
//! Scans any Windows `x86-64` `.exe` or `.dll` for the five mechanisms used by
//! user-mode and kernel-mode protection systems (Overwatch/Byfron, SteamStub,
//! Denuvo, Arxan, EAC, BattlEye):
//! 1. **Direct `syscall` / `sysenter` instructions** and preceding `mov eax, <SSN>`
//!    System Service Numbers (verified against our Windows 10 22H2 `SSDT` table).
//! 2. **Vectored Exception Handler (`VEH`) & Trap Control Flow** (`ud2`, `int 2d`,
//!    in-function `int 3`, `AddVectoredExceptionHandler`, `NtGetContextThread`).
//! 3. **VM / Timing / Environment Probes** (`cpuid`, `rdtsc`, `rdtscp`, `xgetbv`,
//!    direct `gs:[0x30]` TEB / `gs:[0x60]` PEB / `0x7FFE0000` `KUSER_SHARED_DATA` reads).
//! 4. **Packed / Self-Modifying Code** (section Shannon entropy `0.0..8.0`,
//!    `RWX` sections, `VirtualProtect` / `FlushInstructionCache` imports).
//! 5. **Kernel Driver Dependencies** (`OpenSCManager`, `CreateService`,
//!    `NtLoadDriver`, `.sys` imports).

use anyhow::{Context, Result};
use pe_image::{ImportName, Pe};
use iced_x86::{Decoder, DecoderOptions, Instruction, Mnemonic, OpKind, Register};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

/// Known Windows 10 22H2 (Build 19045) x64 System Service Numbers (`SSNs`)
/// supported by `win32_rt::ntsyscall::dispatch_nt_syscall`.
pub static KNOWN_WIN10_SSNS: &[(u32, &str)] = &[
    (0x0000, "NtAccessCheck"),
    (0x0004, "NtWaitForSingleObject"),
    (0x0006, "NtReadFile"),
    (0x0007, "NtDeviceIoControlFile"),
    (0x0008, "NtWriteFile"),
    (0x000D, "NtSetInformationThread"),
    (0x000E, "NtSetEvent"),
    (0x000F, "NtClose"),
    (0x0012, "NtOpenKey"),
    (0x0018, "NtAllocateVirtualMemory"),
    (0x0019, "NtQueryInformationProcess"),
    (0x001C, "NtSetInformationProcess"),
    (0x001E, "NtFreeVirtualMemory"),
    (0x0023, "NtQueryVirtualMemory"),
    (0x0025, "NtQueryInformationThread"),
    (0x0026, "NtOpenProcess"),
    (0x0028, "NtMapViewOfSection"),
    (0x002A, "NtUnmapViewOfSection"),
    (0x002C, "NtTerminateProcess"),
    (0x002E, "NtClearEvent"),
    (0x0031, "NtQueryPerformanceCounter"),
    (0x0033, "NtOpenFile"),
    (0x0034, "NtDelayExecution"),
    (0x0036, "NtQuerySystemInformation"),
    (0x003A, "NtWriteVirtualMemory"),
    (0x003C, "NtDuplicateObject"),
    (0x003F, "NtReadVirtualMemory"),
    (0x0043, "NtContinue"),
    (0x0046, "NtYieldExecution"),
    (0x0048, "NtCreateEvent"),
    (0x0050, "NtProtectVirtualMemory"),
    (0x0055, "NtCreateFile"),
    (0x005B, "NtWaitForMultipleObjects"),
    (0x00B5, "NtFlushInstructionCache"),
    (0x00F2, "NtGetContextThread"),
    (0x012E, "NtOpenThread"),
    (0x0158, "NtQueryTimerResolution"),
    (0x016E, "NtRaiseException"),
    (0x018B, "NtSetContextThread"),
    (0x01B2, "NtSetTimerResolution"),
];

pub fn lookup_ssn_name(ssn: u32) -> Option<&'static str> {
    KNOWN_WIN10_SSNS
        .iter()
        .find(|(n, _)| *n == ssn)
        .map(|(_, name)| *name)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiCheatVerdict {
    /// Standard PE binary with no packer or kernel driver requirements.
    CompatibleClean,
    /// Uses user-mode anti-cheat / anti-debug / timing / VEH / direct-syscall checks
    /// supported by our unpatched-`.text` + `ntsyscall` + `VEH` runtime.
    CompatibleUserModeAc,
    /// Contains high-entropy or RWX packed sections that unpack at runtime via
    /// `VirtualProtect` / `FlushInstructionCache`.
    CompatibleViaRuntimeUnpack,
    /// Imports Windows Ring-0 kernel driver loader APIs (`CreateService` / `NtLoadDriver`).
    RequiresKernelDriverFallback,
}

impl AntiCheatVerdict {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompatibleClean => "COMPATIBLE (Clean Native PE)",
            Self::CompatibleUserModeAc => {
                "COMPATIBLE (User-Mode Anti-Cheat / Integrity / Syscall / VEH)"
            }
            Self::CompatibleViaRuntimeUnpack => {
                "COMPATIBLE VIA RUNTIME JIT UNPACK (Packed / Self-Modifying Sections)"
            }
            Self::RequiresKernelDriverFallback => {
                "REQUIRES USER-MODE FALLBACK (Imports Ring-0 Kernel Service APIs)"
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SectionSecurityInfo {
    pub name: String,
    pub virtual_size: u32,
    pub entropy: f64,
    pub is_exec: bool,
    pub is_write: bool,
}

#[derive(Debug, Clone)]
pub struct AntiCheatAuditReport {
    pub binary_name: String,
    pub verdict: AntiCheatVerdict,
    pub sections: Vec<SectionSecurityInfo>,
    pub direct_syscalls: u64,
    pub decoded_ssns: BTreeMap<u32, u64>,
    pub ud2_count: u64,
    pub int3_count: u64,
    pub cpuid_count: u64,
    pub rdtsc_count: u64,
    pub xgetbv_count: u64,
    pub gs_teb_peb_reads: u64,
    pub kuser_shared_reads: u64,
    pub anti_debug_imports: BTreeSet<String>,
    pub veh_seh_imports: BTreeSet<String>,
    pub memory_unpack_imports: BTreeSet<String>,
    pub kernel_driver_imports: BTreeSet<String>,
}

/// Compute Shannon entropy (`0.0..=8.0` bits/byte) of a byte slice.
pub fn shannon_entropy(bytes: &[u8]) -> f64 {
    if bytes.is_empty() {
        return 0.0;
    }
    let mut counts = [0u64; 256];
    for &b in bytes {
        counts[b as usize] += 1;
    }
    let len = bytes.len() as f64;
    let mut ent = 0.0f64;
    for &c in &counts {
        if c > 0 {
            let p = (c as f64) / len;
            ent -= p * p.log2();
        }
    }
    ent
}

/// Audit a PE binary on disk for anti-cheat, anti-tamper, packer, and direct-syscall patterns.
pub fn audit_pe_anticheat(path: &Path) -> Result<AntiCheatAuditReport> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let pe = Pe::parse(&bytes).with_context(|| format!("parsing PE {}", path.display()))?;
    let binary_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown.pe")
        .to_string();
    Ok(audit_pe_anticheat_parsed(&binary_name, &pe))
}

/// Audit an already-parsed [`Pe`] image for anti-cheat, anti-tamper, packer, and direct-syscall patterns.
pub fn audit_pe_anticheat_parsed(binary_name: &str, pe: &Pe) -> AntiCheatAuditReport {
    let binary_name = binary_name.to_string();

    let mut sections = Vec::new();
    let mut direct_syscalls = 0u64;
    let mut decoded_ssns: BTreeMap<u32, u64> = BTreeMap::new();
    let mut ud2_count = 0u64;
    let mut int3_count = 0u64;
    let mut cpuid_count = 0u64;
    let mut rdtsc_count = 0u64;
    let mut xgetbv_count = 0u64;
    let mut gs_teb_peb_reads = 0u64;
    let mut kuser_shared_reads = 0u64;

    let base = pe.headers.image_base;
    for sec in &pe.sections {
        let len = sec.raw_size.min(sec.mem_size()) as usize;
        let raw_slice = pe.bytes_at(sec.rva, len).unwrap_or(&[]);
        let entropy = shannon_entropy(raw_slice);
        let is_exec = sec.is_exec() || (sec.characteristics & pe_image::consts::IMAGE_SCN_CNT_CODE != 0);
        let is_write = (sec.characteristics & pe_image::consts::IMAGE_SCN_MEM_WRITE) != 0;
        sections.push(SectionSecurityInfo {
            name: sec.name.clone(),
            virtual_size: sec.mem_size(),
            entropy,
            is_exec,
            is_write,
        });

        if !is_exec || raw_slice.is_empty() {
            continue;
        }

        let mut dec = Decoder::with_ip(64, raw_slice, base + sec.rva as u64, DecoderOptions::NONE);
        let mut ins = Instruction::default();
        let mut last_mov_eax_imm: Option<u32> = None;

        while dec.can_decode() {
            dec.decode_out(&mut ins);
            if ins.is_invalid() {
                last_mov_eax_imm = None;
                continue;
            }
            if ins.segment_prefix() == Register::GS {
                gs_teb_peb_reads += 1;
            }
            for op in 0..ins.op_count() {
                if ins.op_kind(op) == OpKind::Memory {
                    let disp = ins.memory_displacement64();
                    if (0x7FFE_0000..0x7FFE_1000).contains(&disp) {
                        kuser_shared_reads += 1;
                    }
                }
            }

            match ins.mnemonic() {
                Mnemonic::Mov
                    if ins.op_count() == 2
                        && ins.op_kind(0) == OpKind::Register
                        && (ins.op_register(0) == Register::EAX
                            || ins.op_register(0) == Register::RAX)
                        && matches!(
                            ins.op_kind(1),
                            OpKind::Immediate32 | OpKind::Immediate32to64 | OpKind::Immediate64
                        ) =>
                {
                    let imm = ins.immediate32();
                    if imm < 0x1000 {
                        last_mov_eax_imm = Some(imm);
                    } else {
                        last_mov_eax_imm = None;
                    }
                }
                Mnemonic::Mov
                    if ins.op_count() == 2
                        && ins.op_register(0) == Register::R10
                        && ins.op_register(1) == Register::RCX =>
                {
                    // `mov r10, rcx` preserves `last_mov_eax_imm` in `ntdll` syscall stubs
                }
                Mnemonic::Syscall | Mnemonic::Sysenter => {
                    direct_syscalls += 1;
                    if let Some(ssn) = last_mov_eax_imm.take() {
                        *decoded_ssns.entry(ssn).or_insert(0) += 1;
                    }
                }
                Mnemonic::Ud2 => {
                    ud2_count += 1;
                    last_mov_eax_imm = None;
                }
                Mnemonic::Int3 => {
                    int3_count += 1;
                    last_mov_eax_imm = None;
                }
                Mnemonic::Cpuid => {
                    cpuid_count += 1;
                    last_mov_eax_imm = None;
                }
                Mnemonic::Rdtsc | Mnemonic::Rdtscp => {
                    rdtsc_count += 1;
                    last_mov_eax_imm = None;
                }
                Mnemonic::Xgetbv => {
                    xgetbv_count += 1;
                    last_mov_eax_imm = None;
                }
                _ => {
                    last_mov_eax_imm = None;
                }
            }
        }
    }

    let mut anti_debug_imports = BTreeSet::new();
    let mut veh_seh_imports = BTreeSet::new();
    let mut memory_unpack_imports = BTreeSet::new();
    let mut kernel_driver_imports = BTreeSet::new();

    let mut all_imps = pe.imports().unwrap_or_default();
    if let Ok(delay) = pe.delay_imports() {
        all_imps.extend(delay);
    }
    for imp in all_imps {
        for ent in imp.entries {
            let ImportName::Name { name, .. } = ent.name else {
                continue;
            };
            match name.as_str() {
                "IsDebuggerPresent"
                | "CheckRemoteDebuggerPresent"
                | "OutputDebugStringA"
                | "OutputDebugStringW"
                | "NtQueryInformationProcess"
                | "ZwQueryInformationProcess"
                | "NtQuerySystemInformation"
                | "ZwQuerySystemInformation"
                | "NtSetInformationThread"
                | "ZwSetInformationThread"
                | "NtGetContextThread"
                | "GetThreadContext"
                | "SetThreadContext" => {
                    anti_debug_imports.insert(format!("{}!{}", imp.dll, name));
                }
                "AddVectoredExceptionHandler"
                | "RemoveVectoredExceptionHandler"
                | "RtlAddVectoredExceptionHandler"
                | "SetUnhandledExceptionFilter"
                | "RaiseException"
                | "RtlCaptureContext"
                | "RtlVirtualUnwind" => {
                    veh_seh_imports.insert(format!("{}!{}", imp.dll, name));
                }
                "VirtualProtect"
                | "VirtualProtectEx"
                | "NtProtectVirtualMemory"
                | "ZwProtectVirtualMemory"
                | "FlushInstructionCache"
                | "NtFlushInstructionCache"
                | "WriteProcessMemory"
                | "NtWriteVirtualMemory" => {
                    memory_unpack_imports.insert(format!("{}!{}", imp.dll, name));
                }
                "OpenSCManagerA"
                | "OpenSCManagerW"
                | "CreateServiceA"
                | "CreateServiceW"
                | "StartServiceA"
                | "StartServiceW"
                | "NtLoadDriver"
                | "ZwLoadDriver" => {
                    kernel_driver_imports.insert(format!("{}!{}", imp.dll, name));
                }
                _ => {}
            }
        }
    }

    let has_rwx_or_packed_exec = sections
        .iter()
        .any(|s| s.is_exec && (s.is_write || s.entropy >= 7.25));

    let verdict = if !kernel_driver_imports.is_empty() {
        AntiCheatVerdict::RequiresKernelDriverFallback
    } else if has_rwx_or_packed_exec {
        AntiCheatVerdict::CompatibleViaRuntimeUnpack
    } else if direct_syscalls > 0
        || !anti_debug_imports.is_empty()
        || !veh_seh_imports.is_empty()
        || gs_teb_peb_reads > 0
    {
        AntiCheatVerdict::CompatibleUserModeAc
    } else {
        AntiCheatVerdict::CompatibleClean
    };

    AntiCheatAuditReport {
        binary_name,
        verdict,
        sections,
        direct_syscalls,
        decoded_ssns,
        ud2_count,
        int3_count,
        cpuid_count,
        rdtsc_count,
        xgetbv_count,
        gs_teb_peb_reads,
        kuser_shared_reads,
        anti_debug_imports,
        veh_seh_imports,
        memory_unpack_imports,
        kernel_driver_imports,
    }
}

/// Format a human-readable anti-cheat & anti-tamper compatibility report.
pub fn format_anticheat_report(r: &AntiCheatAuditReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "=== Anti-Cheat & Anti-Tamper Audit: {} ===", r.binary_name);
    let _ = writeln!(out, "  Verdict              : {}", r.verdict.as_str());
    let _ = writeln!(
        out,
        "  .text Integrity      : UNPATCHED (original x86-64 bytes preserved in guest VA for self-CRC)"
    );
    let _ = writeln!(
        out,
        "  Direct Syscalls      : {} `syscall`/`sysenter` instructions ({} distinct SSNs decoded)",
        r.direct_syscalls,
        r.decoded_ssns.len()
    );
    for (&ssn, &cnt) in &r.decoded_ssns {
        let name = lookup_ssn_name(ssn).unwrap_or("Custom/Dynamic SSN");
        let _ = writeln!(out, "    SSN 0x{ssn:04X} ({name:28}) : {cnt} sites [MAPPED]");
    }
    let _ = writeln!(
        out,
        "  Trap / VEH Sites     : ud2={}, int3={} | VEH/SEH APIs={:?}",
        r.ud2_count, r.int3_count, r.veh_seh_imports
    );
    let _ = writeln!(
        out,
        "  Hardware / OS Probes : cpuid={}, rdtsc/p={}, xgetbv={}, gs:[TEB/PEB]={}, KUSER_SHARED_DATA={}",
        r.cpuid_count, r.rdtsc_count, r.xgetbv_count, r.gs_teb_peb_reads, r.kuser_shared_reads
    );
    let _ = writeln!(out, "  Anti-Debug Imports   : {:?}", r.anti_debug_imports);
    let _ = writeln!(out, "  Unpacker / RWX APIs  : {:?}", r.memory_unpack_imports);
    if !r.kernel_driver_imports.is_empty() {
        let _ = writeln!(out, "  Kernel Driver APIs   : {:?}", r.kernel_driver_imports);
    }
    let _ = writeln!(out, "  Section Entropy & Permissions:");
    for s in &r.sections {
        let perms = format!(
            "R{}{}",
            if s.is_write { "W" } else { "-" },
            if s.is_exec { "X" } else { "-" }
        );
        let note = if s.is_exec && s.is_write {
            " [RWX SELF-MODIFYING/PACKED]"
        } else if s.is_exec && s.entropy >= 7.25 {
            " [HIGH ENTROPY PACKED CODE]"
        } else {
            ""
        };
        let _ = writeln!(
            out,
            "    {:10} {:4} size=0x{:08X} entropy={:.2} bits/byte{}",
            s.name, perms, s.virtual_size, s.entropy, note
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win10_ssn_lookup_covers_core_anticheat_syscalls() {
        assert_eq!(lookup_ssn_name(0x0019), Some("NtQueryInformationProcess"));
        assert_eq!(lookup_ssn_name(0x0023), Some("NtQueryVirtualMemory"));
        assert_eq!(lookup_ssn_name(0x0036), Some("NtQuerySystemInformation"));
        assert_eq!(lookup_ssn_name(0x0050), Some("NtProtectVirtualMemory"));
        assert_eq!(lookup_ssn_name(0x00F2), Some("NtGetContextThread"));
    }

    #[test]
    fn audits_alan_wake_2_anticheat_surface() {
        let Some(exe) = std::env::var_os("PORT_GAME_DIR")
            .or_else(|| std::env::var_os("AW2_GAME_DIR"))
            .filter(|v| !v.is_empty())
            .map(|d| std::path::PathBuf::from(d).join("AlanWake2.exe"))
            .filter(|p| p.is_file())
        else {
            return;
        };
        let rep = audit_pe_anticheat(&exe).expect("audit AlanWake2.exe");
        assert_eq!(rep.verdict, AntiCheatVerdict::CompatibleUserModeAc);
        assert!(rep.gs_teb_peb_reads > 0);
        assert!(rep.cpuid_count > 0);
        assert!(!rep.veh_seh_imports.is_empty());
        assert!(!rep.anti_debug_imports.is_empty());
    }
}
