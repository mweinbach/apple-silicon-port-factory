#!/usr/bin/env python3
"""Standalone full-image (zero `--rva-list`, zero `--dataref-listed`) dry-run audit
for Alan Wake 2 guest DLL lifting.

Runs `gen_subsystems` / `dll_lift` against full PE images redirected strictly to an
isolated temp directory (`/tmp/aw2_dll_full_audit` by default) with `AW2_DLL_PROGRESS=0`,
never touching `crates/` or `db/`.

Usage:
    python3 tools/decomp_scale/audit_full_dlls.py
    python3 tools/decomp_scale/audit_full_dlls.py --only cohtml.windowsdesktop.dll v8.dll
    python3 tools/decomp_scale/audit_full_dlls.py --keep-tmp --json /tmp/aw2_dll_full_audit/report.json
"""

from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time as _time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools" / "fndb"))
sys.path.insert(0, str(ROOT / "tools" / "decomp_scale"))

GAME_DIR_DEFAULT = Path(os.environ.get("PORT_GAME_DIR", os.environ.get("AW2_GAME_DIR", "")))
TMP_DIR_DEFAULT = Path("/tmp/aw2_dll_full_audit")

# Pinned bases from `crates/aw2-game/src/dll_native/mod.rs::PINNED`.
DLL_SPECS: list[tuple[str, str, int]] = [
    ("cohtml.windowsdesktop.dll", "cohtml.WindowsDesktop.dll", 0x70_0000_0000),
    ("v8.dll", "v8.dll", 0x70_006E_C000),
    ("v8_libbase.dll", "v8_libbase.dll", 0x70_01DC_4000),
    ("msvcp140.dll", "msvcp140.dll", 0x70_01E1_4000),
    ("vcruntime140.dll", "vcruntime140.dll", 0x70_01EA_4000),
    ("vcruntime140_1.dll", "vcruntime140_1.dll", 0x70_01EC_4000),
    ("v8_libplatform.dll", "v8_libplatform.dll", 0x70_01ED_0000),
    ("renoircore.windowsdesktop.dll", "RenoirCore.WindowsDesktop.dll", 0x70_01F0_4000),
]


def normalize_reason(reason: str) -> tuple[str, str]:
    """Return `(top_category, sub_detail)` for any `dll_lift` / `gen_subsystems` rejection."""
    r = reason.strip()
    if r.startswith("untranslatable insn at "):
        m = re.match(r"^untranslatable insn at 0x[0-9a-f]+:\s*(\S+)\s*(.*)$", r)
        if m:
            mn, ops = m.group(1), m.group(2)
            shape = _operand_shape(ops)
            return ("TRANSLATE_FAIL:untranslatable_insn", f"{mn} ({shape})" if shape else mn)
        return ("TRANSLATE_FAIL:untranslatable_insn", r)
    if r.startswith("unsupported instruction at "):
        m = re.match(r"^unsupported instruction at 0x[0-9a-f]+:\s*(\S+)\s*(.*)$", r)
        if m:
            mn, ops = m.group(1), m.group(2)
            shape = _operand_shape(ops)
            return ("PRECHECK:unsupported_insn", f"{mn} ({shape})" if shape else mn)
        return ("PRECHECK:unsupported_insn", r)
    if r.startswith("cfg: "):
        m = re.match(r"^cfg:\s*([^0-9]+?)(?:\s+at\s+0x[0-9a-f]+:\s*(\S+).*)?$", r)
        if "blocks >" in r:
            return ("TRANSLATE_FAIL:cfg_blocks_cap", "blocks > 2048")
        if m:
            sub = m.group(1).strip()
            if m.group(2):
                sub = f"{sub} ({m.group(2)})"
            return ("TRANSLATE_FAIL:cfg", sub)
        return ("TRANSLATE_FAIL:cfg", r)
    if r.startswith("unparsed rip operand at "):
        return ("TRANSLATE_FAIL:unparsed_rip", r.split(": ", 1)[-1])
    if r in ("xmm usage analysis failed", "stack usage analysis failed", "cfg translation failed", "translate_function failed"):
        return (f"TRANSLATE_FAIL:{r.replace(' ', '_')}", r)
    if r.startswith("too many instructions "):
        return ("CAP:max_insns", "len(trimmed) > MAX_INSNS")
    if r.startswith("exceeds MAX_FN_BYTES"):
        return ("CAP:max_fn_bytes", r)
    if r == "empty_or_unreadable_primary":
        return ("EXTENT:empty_primary", r)
    if r == "segment decode spilled into padding":
        return ("EXTENT:segment_spill_into_padding", r)
    if r == "truncated extent (no terminator)":
        return ("EXTENT:truncated_extent", r)
    if r == "no instructions":
        return ("EXTENT:no_instructions", r)
    if r == "null-address memory access in output":
        return ("OUTPUT:null_address_access", r)
    if r.startswith("jcc to non-entry code at "):
        return ("LEAF_JCC:jcc_to_non_entry", "jcc to non-entry code")
    for pfx in ("leaf_unaligned", "leaf_no_pad", "leaf_in_padding", "v8_nonabi_leaf"):
        if r.startswith(f"{pfx} at "):
            return (f"LEAF_JCC:{pfx}", pfx)
    if r.startswith("thunk to guest-only target "):
        tgt = r.split("target ", 1)[1].split(" ", 1)[0]
        return ("THUNK:guest_only_target", f"target {tgt}")
    if r.startswith("stack-position reader: "):
        kind = r.split("reader: ", 1)[1].split(" at ", 1)[0]
        return ("ABI:stack_position_reader", kind)
    if r.startswith("calls ") and "machine state" in r:
        imp = r.split(" ", 2)[1]
        return ("ABI:context_import", f"calls {imp}")
    if r.startswith("reads incoming stack arg"):
        return ("STACKARGS:guest_arg_slot_max", r.split(" (", 1)[0])
    if r.startswith("calls 0x") and "incoming stack arg" in r:
        return ("STACKARGS:wide_callee_fwd", "wide_callee_fwd")
    if ": SEH root kept as guest code" in r:
        seh_kind = r.split(" ", 1)[0].rstrip(":")
        return (f"SEH:{seh_kind}", seh_kind)
    if r.startswith("seh_guard_"):
        return ("SEH:seh_guard_fail", r.split(" ", 1)[0])
    if r.startswith("v8_nonabi"):
        tag = r.split(":", 1)[0]
        return (f"V8_NONABI:{tag}", tag)
    return ("OTHER", r[:80])


def _operand_shape(ops: str) -> str:
    if not ops:
        return ""
    parts = [p.strip() for p in re.split(r",(?![^\[]*\])", ops)]
    shapes = []
    for p in parts:
        pl = p.lower()
        if "[" in pl:
            for sz in ("ymmword", "xmmword", "qword", "dword", "word", "byte", "tbyte"):
                if sz in pl:
                    shapes.append(f"mem_{sz}")
                    break
            else:
                shapes.append("mem")
        elif re.match(r"^ymm\d+$", pl):
            shapes.append("ymm")
        elif re.match(r"^xmm\d+$", pl):
            shapes.append("xmm")
        elif re.match(r"^zmm\d+$", pl):
            shapes.append("zmm")
        elif re.match(r"^mm\d+$", pl):
            shapes.append("mmx")
        elif pl.startswith("st"):
            shapes.append("x87")
        elif re.match(r"^(0x[0-9a-f]+|-?\d+|rip_p_\d+|ext_p_\d+|trap_p_\d+)$", pl):
            shapes.append("imm")
        else:
            shapes.append("reg")
    return ",".join(shapes)


def reset_gs_globals(gs) -> None:
    """Restore per-run mutable state on `gen_subsystems` before each DLL audit."""
    gs.LOCKED_INSN_VAS.clear()
    gs.TAIL_TRAP_VAS.clear()
    gs.TRUNCATED_VAS.clear()
    gs.CALL_REDIRECT = {}
    gs.LEAF_STARTS = frozenset()
    gs.LEAF_REJECTS = {}
    gs.NORETURN_CALL_VAS = frozenset()
    gs.EH_SITES = False
    gs.EH_CUR = None
    gs.SEH_GUARD_FAIL[0] = None
    gs.TRANSLATE_FAIL[0] = None


def audit_one_dll(
    mod_name: str,
    dll_filename: str,
    base: int,
    game_dir: Path,
    out_root: Path,
    emit_files: bool = True,
) -> dict:
    os.environ["AW2_DLL_PROGRESS"] = "0"
    import dll_lift
    import gen_subsystems as gs

    reset_gs_globals(gs)
    dll_lift._PDATA_FRAGS.clear()

    img_path = game_dir / dll_filename
    if not img_path.exists():
        raise FileNotFoundError(f"Missing DLL: {img_path}")

    t0 = _time.perf_counter()
    ns = argparse.Namespace(
        only_va=[],
        out_dir=str(out_root),
        image=str(img_path),
        base=hex(base),
        name=mod_name,
        rva_list=None,
        dataref_leaves=True,
        dataref_listed=False,
    )

    pe, all_rows, frag_map, fn_starts, dll_info = dll_lift.load(ns, gs)
    t_load = _time.perf_counter() - t0

    # Mirror `gen_subsystems.main()` exactly, but build the batch ELF inside `out_root`
    # instead of `ROOT / "target" / ".gen_subsystems_batch.elf"`, and record explicit
    # rejection reasons for `sz > MAX_FN_BYTES` so zero candidates vanish silently.
    t1 = _time.perf_counter()
    blob = bytearray()
    offset_map = []
    base_vaddr = 0x140000000
    reject_reason: dict[int, str] = {}
    oversize_details: list[dict] = []

    for va, p_sz, sz, nc, owner, top_ns, ns_str, name in all_rows:
        b = gs.read_frag_va(pe, va, p_sz) or b""
        ehx = dll_info["eh"].get(va)
        cfg_meta = (va, sz, 0x28, ehx["mode"] if ehx else 0, 0, 0, 0, 0)
        if not b:
            reject_reason[va] = "empty_or_unreadable_primary"
            continue
        if sz > gs.MAX_FN_BYTES:
            reject_reason[va] = f"exceeds MAX_FN_BYTES ({sz} > {gs.MAX_FN_BYTES})"
            oversize_details.append({"rva": f"0x{va - base:x}", "sz": sz, "name": name or ""})
            continue
        raw_segs = [(va, b)]
        for fva, fsz in frag_map.get(va, []):
            if fva == va:
                continue
            fb = gs.read_frag_va(pe, fva, fsz)
            if fb:
                raw_segs.append((fva, fb))
        raw_segs.sort(key=lambda x: x[0])
        merged_segs = []
        for s_va, s_b in raw_segs:
            if merged_segs and merged_segs[-1][0] + len(merged_segs[-1][1]) == s_va:
                merged_segs[-1] = (merged_segs[-1][0], merged_segs[-1][1] + s_b)
            else:
                merged_segs.append((s_va, s_b))
        raw_segs = merged_segs
        tot_seg_sz = sum(len(x[1]) for x in raw_segs)
        if tot_seg_sz > gs.MAX_FN_BYTES:
            reject_reason[va] = f"exceeds MAX_FN_BYTES ({tot_seg_sz} > {gs.MAX_FN_BYTES})"
            oversize_details.append({"rva": f"0x{va - base:x}", "sz": tot_seg_sz, "name": name or ""})
            continue
        segs_info = []
        for s_va, s_bytes in raw_segs:
            start_off = len(blob)
            blob.extend(s_bytes)
            blob.extend(b"\xcc" * 16)
            while len(blob) % 16 != 0:
                blob.append(0xCC)
            segs_info.append((start_off, len(s_bytes), s_va))
        offset_map.append((segs_info, va, sz, nc, owner, top_ns, ns_str, name, cfg_meta))

    out_root.mkdir(parents=True, exist_ok=True)
    elf_path = out_root / f".batch_{dll_lift.module_ident(mod_name)}.elf"
    elf_path.write_bytes(gs.g.build_elf64(bytes(blob), base_vaddr=base_vaddr))
    obj_out = subprocess.check_output(
        ["objdump", "--disassemble", "--x86-asm-syntax=intel", "--no-show-raw-insn", str(elf_path)],
        text=True,
    )
    elf_path.unlink(missing_ok=True)
    t_disasm = _time.perf_counter() - t1

    t_lift_start = _time.perf_counter()
    insn_by_off = {}
    locked_offs = set()
    lock_pending = False
    line_re = re.compile(r"^\s*([0-9a-fA-F]+):\s+([a-z0-9_<>]+)\s*(.*)$")
    for line in obj_out.splitlines():
        m = line_re.match(line)
        if m:
            vaddr = int(m.group(1), 16)
            off = vaddr - base_vaddr
            mn = m.group(2)
            ops = m.group(3).split("#")[0].strip()
            if lock_pending:
                locked_offs.add(off)
                lock_pending = False
            if mn == "lock" and not ops:
                lock_pending = True
            elif mn == "lock" and ops:
                parts_l = ops.split(None, 1)
                mn = parts_l[0]
                ops = parts_l[1] if len(parts_l) > 1 else ""
                locked_offs.add(off)
            elif mn in ("rep", "repz", "repe") and ops:
                sub_mn = ops.split()[0]
                if sub_mn == "ret":
                    mn, ops = "ret", ""
                elif sub_mn in ("movsq", "movsb"):
                    mn = f"rep_{sub_mn}"
                    ops = "rdi, rsi, rcx"
                elif sub_mn in ("stosq", "stosd", "stosw", "stosb"):
                    mn = f"rep_{sub_mn}"
                    ops = "rdi, rax, rcx"
                elif sub_mn == "cmpsw":
                    mn = "rep_cmpsw"
                    ops = "rdi, rsi, rcx"
            elif mn in ("repnz", "repne") and ops:
                sub_mn = ops.split()[0]
                if sub_mn in ("scasw", "scasb"):
                    mn = f"repne_{sub_mn}"
                    ops = "rdi, rax, rcx"
            mn, ops = gs.norm_objdump_mn_ops(mn, ops)
            insn_by_off[off] = (mn, ops)

    bad_regs_re = re.compile(r"\b(zmm\d+|mm\d+|fs|cs|ds|es|ss)\b")
    bad_mnems = {"rep", "repz", "repnz", "repe", "repne", "leave", "enter"}

    templates = {}
    lifted_by_va = {}
    stack_infos = {}
    wide_readers = {}
    stack_windows = {}
    max_insns_details: list[dict] = []

    import struct as _st

    for segs_info, va, sz, nc, owner, top_ns, ns_str, fn_name, cfg_meta in offset_map:
        seg_raws = []
        seg_bad = False
        for start_off, seg_len, seg_va in segs_info:
            if start_off not in insn_by_off:
                seg_bad = True
                break
            seg_raw = []
            for off in range(start_off, start_off + seg_len):
                if off in insn_by_off:
                    mn, ops = insn_by_off[off]
                    real_va = seg_va + (off - start_off)
                    if off in locked_offs:
                        gs.LOCKED_INSN_VAS.add(real_va)
                    if mn in ("nop", "lock", "pause"):
                        ops = ""
                    elif (mn.startswith("j") or mn == "call") and ops:
                        first_tok = ops.strip().split()[0]
                        if first_tok.startswith("0x"):
                            try:
                                elf_t_va = int(first_tok, 16)
                                real_t_va = (seg_va + (elf_t_va - (base_vaddr + start_off))) & 0xFFFFFFFFFFFFFFFF
                                ops = f"0x{real_t_va:x}"
                            except ValueError:
                                pass
                    seg_raw.append((real_va, mn, ops))
            if (start_off + seg_len) not in insn_by_off:
                jt_in_seg = any(
                    seg_va < gs.IMAGE_BASE + int(m.group(1), 16) < seg_va + seg_len
                    for _, _, rops in seg_raw
                    for m in gs.JT_DISP_RE.finditer(rops)
                )
                if not jt_in_seg:
                    seg_bad = True
                    break
            seg_raws.append((seg_va, seg_len, seg_raw))
        if seg_bad or not seg_raws:
            reject_reason[va] = "segment decode spilled into padding"
            continue

        s_info = gs.stack_arg_info_segs(seg_raws, va)
        s_max = s_info["max_slot"]
        if s_max >= dll_lift.STACK_ARG_SLOT_LIMIT:
            wide_readers[va] = s_max
        if s_max > dll_lift.GUEST_ARG_SLOT_MAX:
            reject_reason[va] = f"reads incoming stack arg{s_max} (guest callers publish arg5..arg{dll_lift.GUEST_ARG_SLOT_MAX})"
            continue
        stack_infos[va] = s_info
        s_win = dll_lift.stack_window(s_max, dll_lift.window_escapes(s_info))
        all_jt_vas = [
            gs.IMAGE_BASE + int(m.group(1), 16)
            for _, _, s_r in seg_raws
            for _, _, rops in s_r
            for m in gs.JT_DISP_RE.finditer(rops)
        ]
        all_seg_addrs = {a for _, _, s_r in seg_raws for a, _, _ in s_r}
        insns = []
        for seg_va, seg_len, seg_raw in seg_raws:
            true_next = {
                seg_raw[k][0]: (seg_raw[k + 1][0] if k + 1 < len(seg_raw) else seg_va + seg_len)
                for k in range(len(seg_raw))
            }
            jt_cuts = [jva for jva in all_jt_vas if seg_va <= jva < seg_va + seg_len]
            if not jt_cuts and seg_raw and seg_raw[-1][1] not in ("ret", "jmp", "int3", "ud2", "nop"):
                end_aligned = (seg_va + seg_len) & ~3
                t_cur = end_aligned
                n_rvas = 0
                while t_cur - 4 >= seg_va:
                    prev_b = gs.read_frag_va(pe, t_cur - 4, 4)
                    if not prev_b:
                        break
                    rva = _st.unpack("<I", prev_b)[0]
                    t_va = gs.IMAGE_BASE + rva
                    if t_va in all_seg_addrs or (n_rvas >= 2 and seg_va - 64 <= t_va < seg_va):
                        t_cur -= 4
                        n_rvas += 1
                    else:
                        break
                if n_rvas >= 4:
                    jt_cuts.append(t_cur)
            if jt_cuts:
                cut_va = min(jt_cuts)
                while cut_va - 4 > seg_va:
                    prev_b = gs.read_frag_va(pe, cut_va - 4, 4)
                    if not prev_b:
                        break
                    rva = _st.unpack("<I", prev_b)[0]
                    if (gs.IMAGE_BASE + rva) in all_seg_addrs:
                        cut_va -= 4
                    else:
                        break
                seg_raw = [x for x in seg_raw if x[0] < cut_va]
                while len(seg_raw) > 1 and seg_raw[-1][1] in ("int3", "nop"):
                    seg_raw.pop()
            for r_va, r_mn, r_ops in seg_raw:
                insns.append((r_va, r_mn, r_ops, true_next[r_va]))
        if not insns:
            reject_reason[va] = "no instructions"
            continue

        while len(insns) > 1 and insns[-1][1] in ("int3", "nop"):
            insns.pop()
        trimmed = insns
        if len(trimmed) > gs.MAX_INSNS:
            reject_reason[va] = f"too many instructions ({len(trimmed)} > {gs.MAX_INSNS})"
            max_insns_details.append({"rva": f"0x{va - base:x}", "insns": len(trimmed), "sz": sz, "name": fn_name or ""})
            continue

        seg_end_va = seg_raws[-1][0] + seg_raws[-1][1]
        tail_after_call_ok = False
        if trimmed[-1][1] == "call":
            end_b = gs.read_frag_va(pe, seg_end_va, 1)
            tail_after_call_ok = (
                seg_end_va in fn_starts
                or bool(end_b and end_b[0] == 0xCC)
                or gs.is_noreturn_call_insn(trimmed[-1])
            )
            nxt = trimmed[-1][3]
            nxt_b = gs.read_frag_va(pe, nxt, 1)
            if gs.TRAPS and nxt_b and nxt_b[0] == 0xCC:
                trimmed = trimmed + [(nxt, "int3", "", nxt + 1)]
                gs.TAIL_TRAP_VAS.add(nxt)
        elif (
            trimmed[-1][1] not in ("ret", "jmp", "int3", "ud2")
            and trimmed[-1][3] == seg_end_va
            and seg_end_va in fn_starts
            and trimmed[-1][0] + 1 < seg_end_va
        ):
            last = trimmed[-1]
            trimmed = trimmed[:-1] + [
                (last[0], last[1], last[2], last[0] + 1),
                (last[0] + 1, "jmp", f"0x{seg_end_va:x}", seg_end_va),
            ]
        if trimmed[-1][1] not in ("ret", "jmp", "int3", "ud2") and not tail_after_call_ok:
            tail_b = gs.read_frag_va(pe, trimmed[-1][3], 1)
            if not tail_b or tail_b[0] != 0xCC:
                gs.TRUNCATED_VAS.add(va)
                reject_reason[va] = "truncated extent (no terminator)"
                continue

        all_fn_addrs = {a for a, _, _, _ in trimmed}
        bad = None
        for addr, mn, ops, _ in trimmed:
            if (
                mn in bad_mnems
                or mn in gs.EXTRA_BAD_MNEMS
                or (mn.startswith("v") and mn not in gs.ALLOWED_V_MNS)
                or bad_regs_re.search(ops)
            ):
                bad = f"unsupported instruction at 0x{addr:x}: {mn} {ops}"
                break
            if mn.startswith("j") and mn != "jmp":
                t_str = ops.strip().split()[0]
                try:
                    t_va = int(t_str, 16)
                except ValueError:
                    bad = f"unsupported instruction at 0x{addr:x}: {mn} {ops}"
                    break
                if t_va in all_fn_addrs:
                    continue
                t2 = gs.CALL_REDIRECT.get(t_va, t_va)
                tb = gs.read_frag_va(pe, t2, 3) or b""
                if not (t2 in fn_starts or t2 in gs.LEAF_STARTS or tb[:2] == b"\xff\x25" or tb == b"\x48\xff\x25"):
                    why = gs.LEAF_REJECTS.get(t2, "jcc to non-entry code")
                    bad = f"{why} at 0x{addr:x}: {mn} 0x{t_va:x}" + (f" (-> 0x{t2:x})" if t2 != t_va else "")
                    break
        if bad:
            reject_reason[va] = bad
            continue

        _, _, _, eh_mode, _, _, _, _ = cfg_meta
        ehx = dll_info["eh"].get(va)
        gs.EH_CUR = dict(ehx, fn_va=va) if ehx else None
        gs.SEH_GUARD_FAIL[0] = None
        gs.TRANSLATE_FAIL[0] = None
        res = gs.translate_function(trimmed, va, sz, eh_mode, pe)
        gs.EH_CUR = None
        if res is None:
            reject_reason[va] = gs.SEH_GUARD_FAIL[0] or gs.TRANSLATE_FAIL[0] or "translate_function failed"
            continue
        lines, params = res
        if any("(0x0u64) as *" in l for l in lines):
            reject_reason[va] = "null-address memory access in output"
            continue
        if s_win != dll_lift.STACK_ARG_SLOT_LIMIT:
            lines = [f"// stackargs: A={s_win} max={s_max}"] + list(lines)
            stack_windows[va] = s_win

        key = (tuple(lines), len(params), eh_mode)
        if key not in templates:
            templates[key] = (len(templates), len(params))
        lifted_by_va[va] = (key, params)

    slot_max = dict(dll_info.get("slot_max", {}))
    slot_max.update(wide_readers)
    for c_va in sorted(lifted_by_va):
        c_inf = stack_infos.get(c_va)
        if not c_inf:
            continue
        lim = dll_lift.BIG_FRAME_FWD_SLOT_LIMIT if c_inf["big_frame"] else dll_lift.GUEST_FWD_SLOT_LIMIT
        hit = sorted(t for t in c_inf["targets"] if t != c_va and slot_max.get(t, -1) >= lim)
        if hit:
            del lifted_by_va[c_va]
            stack_windows.pop(c_va, None)
            reject_reason[c_va] = (
                f"calls 0x{hit[0]:x}, which reads incoming stack arg{slot_max[hit[0]]} "
                + ("(a big-frame caller publishes arg5..arg24)" if c_inf["big_frame"] else "(beyond arg32)")
            )

    t_translate = _time.perf_counter() - t_lift_start

    # Run `dll_lift.emit` into `out_root/<ident>/` (with AW2_DLL_PROGRESS=0) so we get exact
    # post-lift thunk drops, template dedup counts, E9 alias counts, shard counts, and Rust LOC!
    t3 = _time.perf_counter()
    if emit_files:
        dll_lift.emit(out_root, dll_info, gs, templates, lifted_by_va, all_rows, reject_reason, gs.format_aot_fn)
    t_emit = _time.perf_counter() - t3

    ident = dll_lift.module_ident(mod_name)
    mod_dir = out_root / ident
    t_files = sorted(mod_dir.glob("t*.rs")) if mod_dir.exists() else []
    e_files = sorted(mod_dir.glob("e*.rs")) if mod_dir.exists() else []
    r_files = sorted(mod_dir.glob("r[0-9]*.rs")) if mod_dir.exists() else []
    all_rs = sorted(mod_dir.glob("*.rs")) if mod_dir.exists() else []

    total_loc = sum(p.read_text().count("\n") for p in all_rs)
    t_loc = sum(p.read_text().count("\n") for p in t_files)
    e_loc = sum(p.read_text().count("\n") for p in e_files)
    r_loc = sum(p.read_text().count("\n") for p in r_files)
    total_bytes = sum(p.stat().st_size for p in all_rs)

    # Compute exact template sharing / alias / standalone breakdown matching `dll_lift.emit`.
    def e9_final(va):
        seen = 0
        while seen < 6:
            b_e9 = gs.read_frag_va(pe, va, 5)
            if not b_e9 or b_e9[0] != 0xE9:
                return va if seen else None
            va = (va + 5 + _st.unpack("<i", b_e9[1:5])[0]) & 0xFFFF_FFFF_FFFF_FFFF
            seen += 1
        return None

    alias_of = {}
    for va in lifted_by_va:
        t = e9_final(va)
        if t is not None and t in lifted_by_va and e9_final(t) is None:
            alias_of[va] = t

    key_uses = Counter(k for k, _ in lifted_by_va.values())
    kept_tpls = {
        k
        for k in templates
        if key_uses[k] >= 2 and len(k[0]) <= 64 and not any("EH_META_SELF" in l for l in k[0])
    }
    n_non_alias = len(lifted_by_va) - len(alias_of)
    n_via_tpl = sum(1 for va in lifted_by_va if va not in alias_of and lifted_by_va[va][0] in kept_tpls)
    n_standalone = n_non_alias - n_via_tpl
    n_rust_fns = len(kept_tpls) + n_standalone
    n_eh_lifted = sum(1 for va in lifted_by_va if va not in alias_of and dll_info.get("eh", {}).get(va) is not None)

    regfile = dll_info.get("regfile")
    regfile_lifted_cnt = len(regfile["lifted"]) if regfile else 0
    regfile_rejected = regfile["rejected"] if regfile else []

    # Merge regfile rejections (if not already in reject_reason) for unified accounting
    all_rejects: list[tuple[int, str, str]] = []
    names_by_va = {va: (n or "") for va, *_, n in all_rows}
    seen_rej_rvas = {va - base for va in lifted_by_va}
    for va, why in sorted(reject_reason.items()):
        rva = va - base
        seen_rej_rvas.add(rva)
        all_rejects.append((rva, names_by_va.get(va, ""), why))
    for _va, rva, bid, why in sorted(regfile_rejected):
        if rva not in seen_rej_rvas:
            seen_rej_rvas.add(rva)
            all_rejects.append((rva, f"v8_builtin_{bid}", why))

    cat_counts: Counter[str] = Counter()
    sub_counts: dict[str, Counter[str]] = defaultdict(Counter)
    samples: dict[tuple[str, str], list[dict]] = defaultdict(list)
    thunk_target_causes: Counter[str] = Counter()

    rej_by_rva = {rva: why for rva, _nm, why in all_rejects}
    for rva, nm, why in all_rejects:
        cat, sub = normalize_reason(why)
        cat_counts[cat] += 1
        sub_counts[cat][sub] += 1
        if len(samples[(cat, sub)]) < 8:
            samples[(cat, sub)].append({"rva": f"0x{rva:x}", "name": nm, "reason": why})
        if cat == "THUNK:guest_only_target":
            m_t = re.search(r"\+0x([0-9a-f]+)", why)
            if m_t:
                t_rva = int(m_t.group(1), 16)
                t_why = rej_by_rva.get(t_rva, "skipped_eh_or_not_candidate")
                t_cat, t_sub = normalize_reason(t_why) if t_why != "skipped_eh_or_not_candidate" else ("SKIPPED_EH_OR_CTX", t_why)
                thunk_target_causes[f"{t_cat}:{t_sub}"] += 1

    return {
        "module": mod_name,
        "base": f"0x{base:x}",
        "timing_sec": {
            "load": round(t_load, 2),
            "disasm": round(t_disasm, 2),
            "translate": round(t_translate, 2),
            "emit": round(t_emit, 2),
            "total": round(t_load + t_disasm + t_translate + t_emit, 2),
        },
        "discovery": {
            "pdata_roots": dll_info["roots"],
            "skipped_eh": dll_info["skipped_eh"],
            "eh_kinds_kept_guest": dll_info["eh_kinds"],
            "eh_shapes_candidates": dll_info["eh_shapes"],
            "skipped_ctx_exports": dll_info["skipped_ctx"],
            "leaves_total": dll_info["leaves"],
            "leaves_internal": dll_info["internal_leaves"],
            "candidates_all_rows": len(all_rows),
            "offset_map_disassembled": len(offset_map),
        },
        "lifted": {
            "win64_entries": len(lifted_by_va),
            "regfile_builtins": regfile_lifted_cnt,
            "total_entries": len(lifted_by_va) + regfile_lifted_cnt,
            "e9_aliases": len(alias_of),
            "non_alias_win64": n_non_alias,
            "via_shared_templates": n_via_tpl,
            "shared_template_fns": len(kept_tpls),
            "standalone_fns": n_standalone,
            "emitted_rust_fns": n_rust_fns,
            "eh_meta_entries": n_eh_lifted,
            "wide_stack_windows": len(stack_windows),
        },
        "shards": {
            "t_shards": len(t_files),
            "e_shards": len(e_files),
            "r_shards": len(r_files),
            "total_rs_files": len(all_rs),
            "t_loc": t_loc,
            "e_loc": e_loc,
            "r_loc": r_loc,
            "total_loc": total_loc,
            "total_bytes": total_bytes,
        },
        "rejected": {
            "total": len(all_rejects),
            "by_category": dict(cat_counts.most_common()),
            "by_subdetail": {cat: dict(cnts.most_common()) for cat, cnts in sub_counts.items()},
            "thunk_target_root_causes": dict(thunk_target_causes.most_common()),
            "oversize_bytes": oversize_details,
            "oversize_insns": sorted(max_insns_details, key=lambda d: -d["insns"]),
            "samples": {f"{cat}::{sub}": v for (cat, sub), v in samples.items()},
            "all_non_v8_nonabi": [
                {"rva": f"0x{rva:x}", "name": nm, "reason": why}
                for rva, nm, why in all_rejects
                if not why.startswith("v8_nonabi")
            ],
        },
    }


def print_summary(rep: dict) -> None:
    m = rep["module"]
    lf = rep["lifted"]
    sh = rep["shards"]
    rj = rep["rejected"]
    ds = rep["discovery"]
    tm = rep["timing_sec"]
    print(f"\n{'=' * 92}")
    print(f"{m} (base {rep['base']}) — audited in {tm['total']}s (load {tm['load']}s, disasm {tm['disasm']}s, lift {tm['translate']}s, emit {tm['emit']}s)")
    print(f"{'=' * 92}")
    print(
        f"  Discovery : {ds['pdata_roots']} .pdata roots (skipped_eh={ds['skipped_eh']} {ds['eh_kinds_kept_guest']}, "
        f"skipped_ctx={ds['skipped_ctx_exports']}), {ds['leaves_total']} leaves ({ds['leaves_internal']} internal) -> {ds['candidates_all_rows']} candidates"
    )
    print(
        f"  Lifted    : {lf['win64_entries']} Win64 + {lf['regfile_builtins']} DF_REGFILE = {lf['total_entries']} total entries"
    )
    print(
        f"  Dedup     : {lf['e9_aliases']} E9 aliases, {lf['via_shared_templates']} entries via {lf['shared_template_fns']} tpl_* fns, "
        f"{lf['standalone_fns']} standalone fn_* -> {lf['emitted_rust_fns']} total Rust fns ({lf['eh_meta_entries']} EH_META)"
    )
    print(
        f"  Output    : {sh['t_shards']} t-shards ({sh['t_loc']:,} LOC) + {sh['e_shards']} e-shards ({sh['e_loc']:,} LOC)"
        + (f" + {sh['r_shards']} r-shards ({sh['r_loc']:,} LOC)" if sh["r_shards"] else "")
        + f" = {sh['total_loc']:,} LOC ({sh['total_bytes'] / (1024 * 1024):.2f} MiB)"
    )
    print(f"  Rejected  : {rj['total']} total (+ {ds['skipped_eh']} skipped_eh + {ds['skipped_ctx_exports']} skipped_ctx pre-candidate)")
    for cat, cnt in rj["by_category"].items():
        print(f"    [{cnt:5d}] {cat}")
        for sub, scnt in list(rj["by_subdetail"][cat].items())[:15]:
            ex = rj["samples"].get(f"{cat}::{sub}", [])
            ex_rvas = ", ".join(s["rva"] for s in ex[:4])
            print(f"              {scnt:5d}  {sub:42s}  e.g. {ex_rvas}")
    if rj["thunk_target_root_causes"]:
        print(f"  Thunk guest-target breakdown: {rj['thunk_target_root_causes']}")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--game-dir", type=Path, default=GAME_DIR_DEFAULT, help="Game directory holding shipped PE DLLs")
    ap.add_argument("--tmp-dir", type=Path, default=TMP_DIR_DEFAULT, help="Isolated temp directory for generated dry-run output")
    ap.add_argument("--only", nargs="*", default=None, help="Audit only matching module names (default: all 8 DLLs)")
    ap.add_argument("--json", type=Path, default=None, help="Optional JSON path for structured audit output")
    ap.add_argument("--keep-tmp", action="store_true", help="Keep --tmp-dir instead of cleaning it up on exit")
    args = ap.parse_args(argv)

    tmp_dir = args.tmp_dir.resolve()
    repo_resolved = ROOT.resolve()
    if tmp_dir == repo_resolved or repo_resolved in tmp_dir.parents:
        raise SystemExit(f"Refusing --tmp-dir inside repository: {tmp_dir}")

    if tmp_dir.exists():
        shutil.rmtree(tmp_dir)
    tmp_dir.mkdir(parents=True, exist_ok=True)

    selected = DLL_SPECS
    if args.only:
        wanted = {x.lower() for x in args.only}
        selected = [s for s in DLL_SPECS if s[0] in wanted or s[1].lower() in wanted or s[0].removesuffix(".dll") in wanted]
        if not selected:
            raise SystemExit(f"No DLLs matched --only {args.only}")

    results = {}
    try:
        for mod_name, dll_file, base in selected:
            rep = audit_one_dll(mod_name, dll_file, base, args.game_dir, tmp_dir / "shards")
            results[mod_name] = rep
            print_summary(rep)

        if args.json:
            args.json.parent.mkdir(parents=True, exist_ok=True)
            args.json.write_text(json.dumps(results, indent=2) + "\n")
            print(f"\nWrote JSON audit report to {args.json}")
    finally:
        if not args.keep_tmp and tmp_dir.exists():
            shutil.rmtree(tmp_dir)
            print(f"\nCleaned up {tmp_dir}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
