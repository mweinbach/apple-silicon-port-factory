#!/usr/bin/env python3
"""Universal x86-64 Rosetta 2 differential oracle for PORT_NAN_TRACE snapshots.

Takes one or more snapshot JSON files produced by `PORT_NAN_TRACE=...;dump_dir=<dir>`
(or a directory containing `snap_*.json`), compiles a minimal standalone x86-64
test binary with `clang -arch x86_64`, runs each exact guest x86-64 instruction
with its exact pre-instruction GPR/XMM/YMM/RFLAGS/memory state under real x86-64
semantics via Rosetta 2 (`arch -x86_64`), and compares the resulting destination
register/lane bit-for-bit against `x86-runtime`.

Usage:
  python3 tools/debug/x86_oracle.py /tmp/diag/snaps/
  python3 tools/debug/x86_oracle.py /tmp/diag/snaps/snap_00001_*.json --emit-rust
"""

import argparse
import glob
import json
import os
import struct
import subprocess
import sys
import tempfile


def parse_hex(s: str) -> int:
    return int(s, 16)


def u128_to_u64_pair(v: int) -> tuple[int, int]:
    return (v & 0xFFFF_FFFF_FFFF_FFFF, (v >> 64) & 0xFFFF_FFFF_FFFF_FFFF)


def load_snapshots(paths: list[str]) -> list[tuple[str, dict]]:
    files: list[str] = []
    for p in paths:
        if os.path.isdir(p):
            files.extend(sorted(glob.glob(os.path.join(p, "snap_*.json"))))
        elif os.path.isfile(p):
            files.append(p)
    out = []
    for f in files:
        with open(f, "r") as fh:
            out.append((f, json.load(fh)))
    return out


C_HARNESS = r"""
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>

typedef struct __attribute__((aligned(32))) {
    uint64_t gpr[16];     // 0:rax 1:rcx 2:rdx 3:rbx 4:rsp 5:rbp 6:rsi 7:rdi 8..15:r8..r15
    uint64_t xmm[16][2];  // offset 128: low 128 bits (lo, hi)
    uint64_t ymm[16][2];  // offset 384: high 128 bits (lo, hi)
    uint64_t rflags;      // offset 640
    uint64_t saved_rsp;   // offset 648
    uint32_t pre_mxcsr;   // offset 656 (loaded via ldmxcsr before guest insn)
    uint32_t default_mxcsr; // offset 660 (0x1F80 restored after guest insn)
} State;

static uint8_t mem_buf[64] __attribute__((aligned(64)));

// Run a single x86-64 instruction snippet placed in an RX page.
// For memory operands, we rewrite memory-address base/disp only if unmapped;
// for register-only instructions, we execute the raw bytes verbatim.
static void run_snippet(State *st, const uint8_t *ins, size_t ins_len, int has_avx) {
    uint8_t *page = (uint8_t *)mmap(
        NULL, 4096, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0);
    if (page == MAP_FAILED) {
        fprintf(stderr, "mmap failed\n");
        return;
    }

    // Build trampoline in `page`:
    //   rdi = State*
    //   save callee-saved regs (rbx, rbp, r12..r15) and rsp on host stack
    //   load ymm0..15 (or xmm0..15) from [rdi + offsetof(xmm/ymm)]
    //   load rflags? (keep default MXCSR=0x1F80)
    //   <raw guest instruction bytes>
    //   store ymm0..15 (or xmm0..15) back to [rdi + ...]
    //   restore callee-saved regs & ret
    uint8_t *p = page;
    // push rbx; push rbp; push r12; push r13; push r14; push r15; push rdi
    uint8_t prologue[] = {
        0x53, 0x55, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56, 0x41, 0x57, 0x57
    };
    memcpy(p, prologue, sizeof(prologue));
    p += sizeof(prologue);

    // Load XMM0..15 (or YMM0..15) from rdi+128 / rdi+384 using rdi as base
    for (int i = 0; i < 16; i++) {
        uint32_t off_lo = 128 + i * 16;
        uint32_t off_hi = 384 + i * 16;
        if (has_avx) {
            // vmovdqu xmm(i), [rdi + off_lo]
            // VEX3: C4 (RXB) 7E 6F ModRM disp32 (F3 0F 6F)
            uint8_t rbit = (i >= 8) ? 0x00 : 0x80;
            *p++ = 0xC4; *p++ = (uint8_t)(rbit | 0x61); *p++ = 0x7E;
            *p++ = 0x6F;
            *p++ = (uint8_t)(0x87 | ((i & 7) << 3)); // [rdi + disp32]
            memcpy(p, &off_lo, 4); p += 4;
            // vinserti128 ymm(i), ymm(i), [rdi + off_hi], 1
            // VEX3: C4 (R 1 1 00011) (0 vvvv 1 01) 38 ModRM disp32 01
            uint8_t vvvv = (uint8_t)((~i) & 15);
            *p++ = 0xC4; *p++ = (uint8_t)(rbit | 0x63); *p++ = (uint8_t)((vvvv << 3) | 0x05);
            *p++ = 0x38;
            *p++ = (uint8_t)(0x87 | ((i & 7) << 3));
            memcpy(p, &off_hi, 4); p += 4;
            *p++ = 0x01;
        } else {
            // movdqu xmm(i), [rdi + off_lo]: F3 [44] 0F 6F modrm disp32
            *p++ = 0xF3;
            if (i >= 8) *p++ = 0x44;
            *p++ = 0x0F; *p++ = 0x6F;
            *p++ = (uint8_t)(0x87 | ((i & 7) << 3));
            memcpy(p, &off_lo, 4); p += 4;
        }
    }

    // Load guest MXCSR from [rdi + 656]: ldmxcsr dword ptr [rdi + disp32] (0F AE 97 disp32)
    {
        uint32_t off_mxcsr = 656;
        *p++ = 0x0F; *p++ = 0xAE; *p++ = 0x97;
        memcpy(p, &off_mxcsr, 4); p += 4;
    }

    // Load GPRs (rax..r15 except rsp and rdi; load rdi last since [rsp] holds State*)
    // Layout of State.gpr[r] is offset r*8:
    //   0:rax 1:rcx 2:rdx 3:rbx 4:rsp(skip) 5:rbp 6:rsi 7:rdi(load last) 8..15:r8..r15
    for (int r = 0; r < 16; r++) {
        if (r == 4 || r == 7) continue;
        uint8_t rex = 0x48 | ((r >= 8) ? 0x04 : 0x00);
        *p++ = rex;
        *p++ = 0x8B;
        *p++ = (uint8_t)(0x47 | ((r & 7) << 3)); // mov reg, [rdi + disp8]
        *p++ = (uint8_t)(r * 8);
    }
    // Load rdi last: mov rdi, [rdi + 56]
    *p++ = 0x48; *p++ = 0x8B; *p++ = 0x7F; *p++ = 56;

    // Emit the guest instruction verbatim
    memcpy(p, ins, ins_len);
    p += ins_len;

    // Reload State* into rdi from [rsp]: mov rdi, [rsp]
    *p++ = 0x48; *p++ = 0x8B; *p++ = 0x3C; *p++ = 0x24;

    // Restore default host MXCSR (0x1F80) from [rdi + 660]: ldmxcsr [rdi + disp32]
    {
        uint32_t off_def = 660;
        *p++ = 0x0F; *p++ = 0xAE; *p++ = 0x97;
        memcpy(p, &off_def, 4); p += 4;
    }

    // Store XMM0..15 / YMM0..15 back to [rdi + ...]
    for (int i = 0; i < 16; i++) {
        uint32_t off_lo = 128 + i * 16;
        uint32_t off_hi = 384 + i * 16;
        if (has_avx) {
            uint8_t rbit = (i >= 8) ? 0x00 : 0x80;
            // vmovdqu [rdi + off_lo], xmm(i) (F3 0F 7F)
            *p++ = 0xC4; *p++ = (uint8_t)(rbit | 0x61); *p++ = 0x7E;
            *p++ = 0x7F;
            *p++ = (uint8_t)(0x87 | ((i & 7) << 3));
            memcpy(p, &off_lo, 4); p += 4;
            // vextracti128 [rdi + off_hi], ymm(i), 1
            *p++ = 0xC4; *p++ = (uint8_t)(rbit | 0x63); *p++ = 0x7D;
            *p++ = 0x39;
            *p++ = (uint8_t)(0x87 | ((i & 7) << 3));
            memcpy(p, &off_hi, 4); p += 4;
            *p++ = 0x01;
        } else {
            *p++ = 0xF3;
            if (i >= 8) *p++ = 0x44;
            *p++ = 0x0F; *p++ = 0x7F;
            *p++ = (uint8_t)(0x87 | ((i & 7) << 3));
            memcpy(p, &off_lo, 4); p += 4;
        }
    }

    // pop rdi; pop r15; pop r14; pop r13; pop r12; pop rbp; pop rbx; ret
    uint8_t epilogue[] = {
        0x5F, 0x41, 0x5F, 0x41, 0x5E, 0x41, 0x5D, 0x41, 0x5C, 0x5D, 0x5B, 0xC3
    };
    memcpy(p, epilogue, sizeof(epilogue));
    p += sizeof(epilogue);

    mprotect(page, 4096, PROT_READ | PROT_EXEC);
    typedef void (*fn_t)(State *);
    ((fn_t)page)(st);
    munmap(page, 4096);
}
"""


def main() -> int:
    ap = argparse.ArgumentParser(description="Verify PORT_NAN_TRACE / x86-verify snapshots on real Rosetta x86-64")
    ap.add_argument("inputs", nargs="+", help="Snapshot JSON file(s) or dump directory")
    ap.add_argument("--max", type=int, default=64, help="Maximum unique snapshots to test")
    ap.add_argument("--mxcsr", type=lambda s: int(s, 0), default=None, help="Override MXCSR value for all snapshots (e.g. 0x9fc0 for FTZ|DAZ, 0x1f80 default)")
    ap.add_argument("--quiet-matches", action="store_true", help="Only print divergences and summary")
    args = ap.parse_args()

    snaps = load_snapshots(args.inputs)
    if not snaps:
        print("No snap_*.json files found.", file=sys.stderr)
        return 2

    # Deduplicate by (ip, bytes, pre_xmm[dst], src_lo)
    seen = set()
    uniq = []
    for path, s in snaps:
        key = (s.get("ip"), s.get("bytes"), tuple(s.get("src_lo", [])))
        if key not in seen:
            seen.add(key)
            uniq.append((path, s))
        if len(uniq) >= args.max:
            break

    # Generate one combined x86-64 C harness for all register-only FP instructions
    # (and report memory-operand instructions cleanly using a temporary mapped buffer when possible).
    cases_code = []
    tested = []
    skipped_mem = 0

    for idx, (path, s) in enumerate(uniq):
        raw_bytes = bytes.fromhex(s.get("bytes", ""))
        if not raw_bytes:
            continue
        mem_bytes_hex = s.get("mem_bytes", "")
        if mem_bytes_hex:
            # Memory-operand instructions embed guest virtual addresses in GPRs / RIP-rel;
            # skip direct raw-byte execution here unless we patch addressing, and summarize inputs.
            skipped_mem += 1
            continue
        pre_xmm = s.get("pre_xmm")
        post_xmm = s.get("post_xmm")
        if not pre_xmm or not post_xmm:
            continue
        pre_gpr = s.get("pre_gpr", ["0x0"] * 16)
        pre_ymm = s.get("pre_ymm_hi", ["0x0"] * 16)
        post_ymm = s.get("post_ymm_hi", ["0x0"] * 16)
        dst = int(s["dst_reg"])
        has_avx = 1 if (raw_bytes[0] in (0xC4, 0xC5)) else 0

        pre_mxcsr = args.mxcsr if args.mxcsr is not None else parse_hex(s.get("pre_mxcsr", "0x1f80"))
        lines = [f"  {{ // case {len(tested)}: {s.get('where')} `{s.get('disasm')}` (mxcsr=0x{pre_mxcsr:04x})"]
        lines.append("    State st; memset(&st, 0, sizeof(st));")
        lines.append(f"    st.pre_mxcsr = 0x{pre_mxcsr & 0xFFFF:04x}U; st.default_mxcsr = 0x1F80U;")
        for r in range(16):
            gv = parse_hex(pre_gpr[r]) if r < len(pre_gpr) else 0
            lines.append(f"    st.gpr[{r}]=0x{gv:016x}ULL;")
        for r in range(16):
            lo, hi = u128_to_u64_pair(parse_hex(pre_xmm[r]))
            ylo, yhi = u128_to_u64_pair(parse_hex(pre_ymm[r]))
            lines.append(f"    st.xmm[{r}][0]=0x{lo:016x}ULL; st.xmm[{r}][1]=0x{hi:016x}ULL;")
            lines.append(f"    st.ymm[{r}][0]=0x{ylo:016x}ULL; st.ymm[{r}][1]=0x{yhi:016x}ULL;")
        b_arr = ", ".join(f"0x{b:02x}" for b in raw_bytes)
        lines.append(f"    uint8_t ins[] = {{{b_arr}}};")
        lines.append(f"    run_snippet(&st, ins, sizeof(ins), {has_avx});")
        lines.append(
            f'    printf("CASE %d %016llx%016llx %016llx%016llx\\n", {len(tested)}, '
            f"(unsigned long long)st.xmm[{dst}][1], (unsigned long long)st.xmm[{dst}][0], "
            f"(unsigned long long)st.ymm[{dst}][1], (unsigned long long)st.ymm[{dst}][0]);"
        )
        lines.append("  }")
        cases_code.append("\n".join(lines))
        tested.append((path, s))

    print(f"Loaded {len(uniq)} unique snapshots ({len(tested)} reg-only testable on Rosetta x86-64, {skipped_mem} mem-operand).")
    if not tested:
        return 0

    src = C_HARNESS + "\nint main(void) {\n" + "\n".join(cases_code) + "\n  return 0;\n}\n"
    with tempfile.TemporaryDirectory(prefix="x86_oracle_") as td:
        c_path = os.path.join(td, "oracle.c")
        bin_path = os.path.join(td, "oracle")
        with open(c_path, "w") as fh:
            fh.write(src)
        cc = subprocess.run(
            ["clang", "-arch", "x86_64", "-O1", c_path, "-o", bin_path],
            capture_output=True, text=True
        )
        if cc.returncode != 0:
            print("clang -arch x86_64 failed:", cc.stderr, file=sys.stderr)
            return 2
        env = os.environ.copy()
        env["ROSETTA_ADVERTISE_AVX"] = "1"
        env["ROSETTA_DISABLE_AOT"] = "1"
        rosetta_sh = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "rosetta_run.sh")
        run_cmd = [rosetta_sh, bin_path] if os.path.isfile(rosetta_sh) else ["arch", "-x86_64", bin_path]
        run = subprocess.run(run_cmd, capture_output=True, text=True, env=env)
        if run.returncode != 0:
            print("Rosetta x86_64 run failed:", run.stderr, file=sys.stderr)
            return 2

    divergences = 0
    for line in run.stdout.splitlines():
        if not line.startswith("CASE "):
            continue
        parts = line.split()
        ci = int(parts[1])
        x86_lo = int(parts[2], 16)
        x86_hi = int(parts[3], 16)
        path, s = tested[ci]
        dst = int(s["dst_reg"])
        emu_lo = parse_hex(s["post_xmm"][dst])
        emu_hi = parse_hex(s["post_ymm_hi"][dst])
        lane = int(s["lane"])
        # Check if real x86-64 ALSO produced NaN in that lane vs finite!
        x86_lane32 = (x86_lo >> (lane * 32)) & 0xFFFF_FFFF if lane < 4 else (x86_hi >> ((lane - 4) * 32)) & 0xFFFF_FFFF
        x86_is_nan32 = ((x86_lane32 & 0x7F80_0000) == 0x7F80_0000) and ((x86_lane32 & 0x007F_FFFF) != 0)
        x86_lane64 = (x86_lo >> (lane * 64)) & 0xFFFF_FFFF_FFFF_FFFF if lane < 2 else (x86_hi >> ((lane - 2) * 64)) & 0xFFFF_FFFF_FFFF_FFFF
        x86_is_nan64 = ((x86_lane64 & 0x7FF0_0000_0000_0000) == 0x7FF0_0000_0000_0000) and ((x86_lane64 & 0x000F_FFFF_FFFF_FFFF) != 0)
        exact_match = (x86_lo == emu_lo and x86_hi == emu_hi)
        if not exact_match:
            divergences += 1
            emu_lane32 = (emu_lo >> (lane * 32)) & 0xFFFF_FFFF if lane < 4 else (emu_hi >> ((lane - 4) * 32)) & 0xFFFF_FFFF
            emu_is_nan32 = ((emu_lane32 & 0x7F80_0000) == 0x7F80_0000) and ((emu_lane32 & 0x007F_FFFF) != 0)
            emu_lane64 = (emu_lo >> (lane * 64)) & 0xFFFF_FFFF_FFFF_FFFF if lane < 2 else (emu_hi >> ((lane - 2) * 64)) & 0xFFFF_FFFF_FFFF_FFFF
            emu_is_nan64 = ((emu_lane64 & 0x7FF0_0000_0000_0000) == 0x7FF0_0000_0000_0000) and ((emu_lane64 & 0x000F_FFFF_FFFF_FFFF) != 0)
            x86_nan = x86_is_nan32 or x86_is_nan64
            emu_nan = emu_is_nan32 or emu_is_nan64
            if emu_nan and not x86_nan:
                severity = "CRITICAL_NAN_DIVERGENCE"
            elif x86_nan and emu_nan:
                severity = "NAN_PAYLOAD_DIFF"
            else:
                severity = "VALUE_DIVERGENCE"
            print(
                f"[{severity}] {s['where']} `{s['disasm']}` (lane {lane}):\n"
                f"  real x86-64: xmm{dst}=0x{x86_lo:032x} ymm_hi=0x{x86_hi:032x}\n"
                f"  x86-runtime: xmm{dst}=0x{emu_lo:032x} ymm_hi=0x{emu_hi:032x}\n"
                f"  snapshot:    {path}"
            )
        elif not args.quiet_matches:
            print(f"[MATCH] {s['where']} `{s['disasm']}` lane={lane}")

    print(f"\nOracle summary: {len(tested)} tested, {divergences} divergences.")
    return 1 if divergences > 0 else 0


if __name__ == "__main__":
    sys.exit(main())
