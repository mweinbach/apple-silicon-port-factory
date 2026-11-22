// ============================================================================
// Northlight Engine (Alan Wake 2) — Decompiled Startup & Hardware Gate Module
// Reconstructed from AlanWake2.exe (PE32+ x86-64, ImageBase 0x140000000)
// ============================================================================
//
// Function Address Table:
//   - EntryPoint (__scrt_common_main_seh): VA 0x141835330 | RVA 0x01835330 | Raw 0x01834730
//   - WinMain (Northlight Boot):           VA 0x1418401e0 | RVA 0x018401e0 | Raw 0x0183f5e0
//   - DetectCpuFeaturesAndTopology:        VA 0x1424a6440 | RVA 0x024a6440 | Raw 0x024a5840
//   - ValidateCpuRequirements:             VA 0x1424a6bc0 | RVA 0x024a6bc0 | Raw 0x024a5fc0
//   - CheckCpuFeaturesOrTerminate:         VA 0x141841a40 | RVA 0x01841a40 | Raw 0x01840e40
//   - CheckStorageSeekPenalty:             VA 0x141841bd0 | RVA 0x01841bd0 | Raw 0x01840fd0
//   - CheckPhysicalMemoryWarning:          VA 0x1418573d0 | RVA 0x018573d0 | Raw 0x018567d0
//   - InitGpuErrorStrings:                 VA 0x141841f60 | RVA 0x01841f60 | Raw 0x01841360
//   - InitD3D12AndValidateGpu:             VA 0x14208d490 | RVA 0x0208d490 | Raw 0x0208c890
//   - ValidateGpuFeatures:                 VA 0x142070370 | RVA 0x02070370 | Raw 0x0206f770
// ============================================================================

#include <cstdint>
#include <cstring>

namespace northlight::platform {

// Global variables in .data section
// VA 0x143c1bf9c: Physical CPU core count
static uint32_t g_PhysicalCoreCount = 0;
// VA 0x143c1bfa0: Logical hardware thread count
static uint32_t g_HardwareThreadCount = 0;
// VA 0x143c1bfa4: 26-bit CPU feature bitmask populated by CPUID
static uint32_t g_CpuFeatureMask = 0;

// Bit positions in g_CpuFeatureMask (0x143c1bfa4)
enum CpuFeatureBit : uint32_t {
    CPU_BIT_SSE3                = 0,  // CPUID(1).ECX[0]
    CPU_BIT_MMX                 = 1,  // CPUID(1).EDX[23]
    CPU_BIT_SSE                 = 2,  // CPUID(1).EDX[25]
    CPU_BIT_SSE2                = 3,  // CPUID(1).EDX[26]
    CPU_BIT_SSSE3               = 4,  // CPUID(1).ECX[9]
    CPU_BIT_POPCNT              = 5,  // CPUID(1).ECX[23]
    CPU_BIT_SSE41               = 6,  // CPUID(1).ECX[19]
    CPU_BIT_SSE42               = 7,  // CPUID(1).ECX[20]
    CPU_BIT_SSE4A               = 8,  // CPUID(0x80000001).ECX[6]
    CPU_BIT_AVX                 = 9,  // CPUID(1).ECX[28]
    CPU_BIT_AVX2                = 10, // CPUID(7,0).EBX[5]
    CPU_BIT_FMA                 = 11, // CPUID(1).ECX[12]
    CPU_BIT_AVX512F             = 12, // CPUID(7,0).EBX[16]
    CPU_BIT_AVX512CD            = 13, // CPUID(7,0).EBX[28]
    CPU_BIT_AVX512ER            = 14, // CPUID(7,0).EBX[27]
    CPU_BIT_AVX512PF            = 15, // CPUID(7,0).EBX[26]
    CPU_BIT_AVX512BW            = 16, // CPUID(7,0).EBX[30]
    CPU_BIT_AVX512DQ            = 17, // CPUID(7,0).EBX[17]
    CPU_BIT_AVX512VL            = 18, // CPUID(7,0).EBX[31]
    CPU_BIT_AVX512IFMA          = 19, // CPUID(7,0).EBX[21]
    CPU_BIT_MULTITHREAD_CAPABLE = 20, // PhysicalCores > 1 || LogicalThreads > 1
    CPU_BIT_MULTICORE           = 21, // PhysicalCores > 1
    CPU_BIT_SMT_ENABLED         = 22, // LogicalThreads > PhysicalCores
    CPU_BIT_LZCNT               = 23, // CPUID(0x80000001).ECX[5]
    CPU_BIT_ABM                 = 24, // CPUID(0x80000001).ECX[5]
    CPU_BIT_F16C                = 25, // CPUID(1).ECX[29]
};

struct CpuRequirementsSpec {
    uint8_t required_bits[26]; // +0x00..+0x19: 1 if feature bit is mandatory
    uint16_t pad_1a;           // +0x1A
    uint32_t min_threads;      // +0x1C: Minimum logical hardware threads (4)
};

struct CpuErrorLabels {
    const wchar_t* title;
    const wchar_t* body;
    const wchar_t* feature_names[26];
    const wchar_t* threads_fmt;
    const wchar_t* ok_button;
};

// ============================================================================
// VA 0x1424a6440 (File Offset 0x024a5840)
// Queries CPUID leaves 0, 1, 7, 0x80000000..0x80000004 and
// GetLogicalProcessorInformation(RelationProcessorCore) to populate
// g_CpuFeatureMask (0x143c1bfa4) and g_HardwareThreadCount (0x143c1bfa0).
// ============================================================================
void DetectCpuFeaturesAndTopology(uint32_t cpuid1_ecx,
                                  uint32_t cpuid1_edx,
                                  uint32_t cpuid7_ebx,
                                  uint32_t cpuid_ext1_ecx,
                                  uint32_t physical_cores,
                                  uint32_t logical_threads) {
    uint32_t mask = 0;

    if (cpuid1_ecx & (1u << 0))  mask |= (1u << CPU_BIT_SSE3);
    if (cpuid1_ecx & (1u << 9))  mask |= (1u << CPU_BIT_SSSE3);
    if (cpuid1_ecx & (1u << 12)) mask |= (1u << CPU_BIT_FMA);
    if (cpuid1_ecx & (1u << 19)) mask |= (1u << CPU_BIT_SSE41);
    if (cpuid1_ecx & (1u << 20)) mask |= (1u << CPU_BIT_SSE42);
    if (cpuid1_ecx & (1u << 23)) mask |= (1u << CPU_BIT_POPCNT);
    if (cpuid1_ecx & (1u << 28)) mask |= (1u << CPU_BIT_AVX);
    if (cpuid1_ecx & (1u << 29)) mask |= (1u << CPU_BIT_F16C);

    if (cpuid1_edx & (1u << 23)) mask |= (1u << CPU_BIT_MMX);
    if (cpuid1_edx & (1u << 25)) mask |= (1u << CPU_BIT_SSE);
    if (cpuid1_edx & (1u << 26)) mask |= (1u << CPU_BIT_SSE2);

    if (cpuid7_ebx & (1u << 5))  mask |= (1u << CPU_BIT_AVX2);
    if (cpuid7_ebx & (1u << 16)) mask |= (1u << CPU_BIT_AVX512F);
    if (cpuid7_ebx & (1u << 17)) mask |= (1u << CPU_BIT_AVX512DQ);
    if (cpuid7_ebx & (1u << 21)) mask |= (1u << CPU_BIT_AVX512IFMA);
    if (cpuid7_ebx & (1u << 26)) mask |= (1u << CPU_BIT_AVX512PF);
    if (cpuid7_ebx & (1u << 27)) mask |= (1u << CPU_BIT_AVX512ER);
    if (cpuid7_ebx & (1u << 28)) mask |= (1u << CPU_BIT_AVX512CD);
    if (cpuid7_ebx & (1u << 30)) mask |= (1u << CPU_BIT_AVX512BW);
    if (cpuid7_ebx & (1u << 31)) mask |= (1u << CPU_BIT_AVX512VL);

    if (cpuid_ext1_ecx & (1u << 5)) {
        mask |= (1u << CPU_BIT_LZCNT) | (1u << CPU_BIT_ABM);
    }
    if (cpuid_ext1_ecx & (1u << 6)) {
        mask |= (1u << CPU_BIT_SSE4A);
    }

    if (physical_cores > 1 || logical_threads > 1) {
        mask |= (1u << CPU_BIT_MULTITHREAD_CAPABLE);
    }
    if (physical_cores > 1) {
        mask |= (1u << CPU_BIT_MULTICORE);
    }
    if (logical_threads > physical_cores) {
        mask |= (1u << CPU_BIT_SMT_ENABLED);
    }

    g_PhysicalCoreCount = physical_cores;
    g_HardwareThreadCount = logical_threads;
    g_CpuFeatureMask = mask;
}

// ============================================================================
// VA 0x1424a6bc0 (File Offset 0x024a5fc0)
// Validates that all required bits in `req` are present in `g_CpuFeatureMask`
// and that `g_HardwareThreadCount >= req->min_threads`.
// ============================================================================
bool ValidateCpuRequirements(const CpuRequirementsSpec* req, const CpuErrorLabels* /*labels*/) {
    bool all_ok = true;
    for (uint32_t bit = 0; bit < 26; ++bit) {
        if (bit == CPU_BIT_MULTITHREAD_CAPABLE) {
            continue; // Checked separately below via min_threads
        }
        if (req->required_bits[bit] != 0) {
            if ((g_CpuFeatureMask & (1u << bit)) == 0) {
                all_ok = false;
            }
        }
    }

    if (req->required_bits[CPU_BIT_MULTITHREAD_CAPABLE] != 0) {
        if (g_HardwareThreadCount < req->min_threads) {
            all_ok = false;
        }
    }

    return all_ok;
}

// ============================================================================
// VA 0x141841a40 (File Offset 0x01840e40)
// Builds the Alan Wake 2 CPU requirement table:
//   - POPCNT (bit 5) = 1
//   - SSE4.1 (bit 6) = 1
//   - SSE4.2 (bit 7) = 1
//   - AVX    (bit 9) = 1
//   - AVX2   (bit 10) = 1
//   - FMA    (bit 11) = 1
//   - MULTITHREAD_CAPABLE (bit 20) = 1, min_threads = 4
// On failure, terminates process with exit code 0x0B501E7E (VA 0x141841baa).
//
// macOS Note:
//   Under CrossOver Preview / GPTK with `ROSETTA_ADVERTISE_AVX=1`, Rosetta 2
//   advertises AVX, AVX2, FMA, SSE4.1, SSE4.2, and POPCNT via CPUID and passes
//   this check without binary modification.
//   To patch the binary directly: at VA 0x141841b9f (File Offset 0x01840f9f),
//   replace `75 14` (`jne +0x14`) with `eb 14` (`jmp +0x14`).
// ============================================================================
bool CheckCpuFeaturesOrTerminate() {
    CpuRequirementsSpec req{};
    req.required_bits[CPU_BIT_POPCNT] = 1;
    req.required_bits[CPU_BIT_SSE41]  = 1;
    req.required_bits[CPU_BIT_SSE42]  = 1;
    req.required_bits[CPU_BIT_AVX]    = 1;
    req.required_bits[CPU_BIT_AVX2]   = 1;
    req.required_bits[CPU_BIT_FMA]    = 1;
    req.required_bits[CPU_BIT_MULTITHREAD_CAPABLE] = 1;
    req.min_threads = 4;

    CpuErrorLabels labels{};
    if (!ValidateCpuRequirements(&req, &labels)) {
        // TerminateProcess(GetCurrentProcess(), 0x0B501E7E);
        return false;
    }
    return true;
}

// ============================================================================
// VA 0x141841bd0 (File Offset 0x01840fd0)
// Queries `IOCTL_STORAGE_QUERY_PROPERTY` (0x2D1400) with
// `StorageDeviceSeekPenaltyProperty` (PropertyId = 7) on `\\.\X:` to warn if
// the game is installed on a mechanical HDD (`IncursSeekPenalty != 0`).
// Skipped if `-nodiskcheck` (`g_Config->bNoDiskCheck` at +0x9a40) is set.
// ============================================================================
void CheckStorageSeekPenalty(bool no_disk_check_flag) {
    if (no_disk_check_flag) {
        return;
    }
    // Opens "\\.\C:" via CreateFileW and calls DeviceIoControl(hDevice, 0x2D1400, ...)
}

// ============================================================================
// VA 0x142070370 (File Offset 0x0206f770)
// Validates D3D12 GPU capabilities after `D3D12CreateDevice`:
//   1. `D3D12_FEATURE_D3D12_OPTIONS.TypedUAVLoadAdditionalFormats != 0` (Fatal)
//   2. `D3D12_FEATURE_D3D12_OPTIONS.ResourceBindingTier >= 3`           (Fatal)
//   3. `D3D12_FEATURE_SHADER_MODEL.HighestShaderModel >= 0x65` (SM 6.5) (Fatal)
//   4. `DedicatedVideoMemory >= 0x157C00000` (5,500 MiB)                (Warning)
// If any fatal check fails, caller `0x14208dd4e` terminates with `0x0B501E7F`.
//
// macOS Note:
//   Apple Silicon M3 Ultra under D3DMetal 4.0b1 natively satisfies all 4 checks
//   plus Mesh Shaders (`D3D12_FEATURE_D3D12_OPTIONS7.MeshShaderTier >= 1`) and
//   Hardware Raytracing (`D3D12_FEATURE_D3D12_OPTIONS5.RaytracingTier >= 11`).
// ============================================================================
bool ValidateGpuFeatures(bool typed_uav_load,
                         uint32_t resource_binding_tier,
                         uint32_t highest_shader_model,
                         uint64_t dedicated_vram_bytes) {
    bool fatal_missing = false;

    if (!typed_uav_load) {
        fatal_missing = true; // GPU_FEATURE_TYPEDUAVLOAD
    }
    if (resource_binding_tier < 3) {
        fatal_missing = true; // GPU_FEATURE_RESOURCEBINDINGTIER3
    }
    if (highest_shader_model < 0x65) {
        fatal_missing = true; // GPU_FEATURE_SHADER_MODEL_65
    }
    if (dedicated_vram_bytes < 0x157C00000ULL) {
        // Non-fatal warning: GPU_MEMORY_TEXT (5,500 MiB threshold)
    }

    return !fatal_missing;
}

} // namespace northlight::platform
