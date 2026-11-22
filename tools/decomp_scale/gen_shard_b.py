#!/usr/bin/env python3
"""Generate crates/aw2-game/src/shards/templates_b.rs for Shard B."""

import bisect
import os
import re
import sqlite3
import struct
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
sys.path.insert(0, os.path.join(ROOT, 'tools', 'fndb'))
import canthrow  # noqa: E402
import fndb  # noqa: E402


def fast_eh_info(pe, va):
    rva = va - pe.image_base
    ents = getattr(pe, '_pdata_sorted', None)
    starts = getattr(pe, '_pdata_starts', None)
    if ents is None:
        ents = pe._pdata_sorted = sorted(pe.pdata())
        starts = pe._pdata_starts = [e[0] for e in ents]
    i = bisect.bisect_right(starts, rva) - 1
    if i < 0 or not (ents[i][0] <= rva < ents[i][1]):
        return 'none'
    d = pe.data
    u = ents[i][2]
    for _ in range(64):
        if u & 1:
            u = struct.unpack_from('<I', d, pe.off(u & ~1) + 8)[0]
            continue
        uo = pe.off(u)
        fl, cnt = d[uo] >> 3, d[uo + 2]
        tail = uo + 4 + ((cnt + 1) & ~1) * 2
        if fl & 4:
            u = struct.unpack_from('<I', d, tail + 8)[0]
            continue
        break
    if not (fl & 3):
        return 'none'
    return canthrow.eh_info(pe, va)


def load_excluded_vas():
    vas = set()
    src_root = os.path.join(ROOT, 'crates', 'aw2-game', 'src')
    for sub in ('registry', 'shards'):
        base_d = os.path.join(src_root, sub)
        for dirpath, _, files in os.walk(base_d):
            if os.path.basename(dirpath) == 'templates_b':
                continue
            for fn in sorted(files):
                if not fn.endswith('.rs') or fn in ('templates_b.rs', 'mod.rs'):
                    continue
                p = os.path.join(dirpath, fn)
                with open(p, 'r', encoding='utf-8') as f:
                    code = re.sub(r'//[^\n]*', '', f.read())
                for m in fndb.REG_ENTRY_RE.finditer(code):
                    vm = re.search(r'\bva\s*:\s*(0x[0-9a-fA-F_]+)', m.group('body'))
                    if vm:
                        vas.add(int(vm.group(1).replace('_', ''), 16))
                for m in fndb.REG_CTOR_RE.finditer(code):
                    vas.add(int(m.group('va').replace('_', ''), 16))
    return vas


def main():
    exe_path = os.environ.get('AW2_EXE') or (
        os.path.join(os.environ['AW2_GAME_DIR'], 'AlanWake2.exe')
        if os.environ.get('AW2_GAME_DIR')
        else ''
    )
    if not exe_path or not os.path.isfile(exe_path):
        raise SystemExit('set AW2_EXE or AW2_GAME_DIR to your local Alan Wake 2 install')
    db_path = os.path.join(ROOT, 'db', 'functions.sqlite')
    out_path = os.path.join(ROOT, 'crates', 'aw2-game', 'src', 'shards', 'templates_b.rs')

    pe = fndb.PE(exe_path)
    db = sqlite3.connect(db_path)

    excl_vas = load_excluded_vas()
    can_throw_map = dict(db.execute('SELECT va, mask_direct FROM can_throw').fetchall())
    all_funcs = {
        va: end - va
        for va, end in db.execute('SELECT va, "end" FROM functions').fetchall()
    }

    target_int_cache = {}
    sse_op2 = {
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
        0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
        0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
    }

    def is_target_int_only(tva):
        res = target_int_cache.get(tva)
        if res is not None:
            return res
        span = all_funcs.get(tva)
        if span is None or span <= 0:
            target_int_cache[tva] = False
            return False
        off = (tva - pe.image_base) - pe.text_rva
        if off < 0 or off + span > len(pe.text):
            target_int_cache[tva] = False
            return False
        tb = pe.text[off:off + span]
        for i in range(len(tb) - 1):
            b0 = tb[i]
            b1 = tb[i + 1]
            if b0 == 0x0f and b1 in sse_op2:
                target_int_cache[tva] = False
                return False
            if b0 in (0xf3, 0xf2, 0x66) and b1 == 0x0f:
                target_int_cache[tva] = False
                return False
        target_int_cache[tva] = True
        return True

    def parse_funclet(va, b):
        # 40 55 48 83 ec 20 48 8b ea ... 48 83 c4 20 5d c3
        if b[:9] != b'\x40\x55\x48\x83\xec\x20\x48\x8b\xea' or b[-6:] != b'\x48\x83\xc4\x20\x5d\xc3':
            return None
        mid = b[9:-6]
        # Direct call funclet: mov/lea rcx, [rbp + off]; (add rcx, off2)?; call rel32; (nop)?
        if len(mid) in (9, 10) and mid[:3] in (b'\x48\x8b\x4d', b'\x48\x8d\x4d') and mid[4] == 0xe8:
            if len(mid) == 10 and mid[9] != 0x90:
                return None
            oo = struct.unpack_from('<b', mid, 3)[0]
            tva = va + 9 + 4 + 5 + struct.unpack_from('<i', mid, 5)[0]
            if oo >= 0 and tva in all_funcs:
                fn = 'tpl_funclet_deref_call' if mid[1] == 0x8b else 'tpl_funclet_lea_call'
                return f'{fn}::<0x{oo:x}, 0x0, 0x{tva:x}>'
        if len(mid) in (12, 13) and mid[:3] in (b'\x48\x8b\x8d', b'\x48\x8d\x8d') and mid[7] == 0xe8:
            if len(mid) == 13 and mid[12] != 0x90:
                return None
            oo = struct.unpack_from('<i', mid, 3)[0]
            tva = va + 9 + 7 + 5 + struct.unpack_from('<i', mid, 8)[0]
            if oo >= 0 and tva in all_funcs:
                fn = 'tpl_funclet_deref_call' if mid[1] == 0x8b else 'tpl_funclet_lea_call'
                return f'{fn}::<0x{oo:x}, 0x0, 0x{tva:x}>'
        if len(mid) in (13, 14) and mid[:3] == b'\x48\x8b\x4d' and mid[4:7] == b'\x48\x83\xc1' and mid[8] == 0xe8:
            if len(mid) == 14 and mid[13] != 0x90:
                return None
            oo = struct.unpack_from('<b', mid, 3)[0]
            add_o = struct.unpack_from('<b', mid, 7)[0]
            tva = va + 9 + 8 + 5 + struct.unpack_from('<i', mid, 9)[0]
            if oo >= 0 and add_o >= 0 and tva in all_funcs:
                return f'tpl_funclet_deref_call::<0x{oo:x}, 0x{add_o:x}, 0x{tva:x}>'
        if len(mid) in (14, 15) and mid[0] == 0xba and mid[5:8] == b'\x48\x8b\x4d' and mid[9] == 0xe8:
            if len(mid) == 15 and mid[14] != 0x90:
                return None
            edx_imm = struct.unpack_from('<I', mid, 1)[0]
            oo = struct.unpack_from('<b', mid, 8)[0]
            tva = va + 9 + 9 + 5 + struct.unpack_from('<i', mid, 10)[0]
            if oo >= 0 and tva in all_funcs:
                return f'tpl_funclet_deref_edx_call::<0x{oo:x}, 0x{edx_imm:x}, 0x{tva:x}>'

        # NRVO flag-guarded funclet
        i = 0
        if mid[i:i + 2] == b'\x8b\x45':
            flag_off = struct.unpack_from('<b', mid, i + 2)[0]
            i += 3
        elif mid[i:i + 2] == b'\x8b\x85':
            flag_off = struct.unpack_from('<i', mid, i + 2)[0]
            i += 6
        else:
            return None
        if flag_off < 0:
            return None
        if mid[i:i + 2] == b'\x83\xe0':
            bit = struct.unpack_from('<b', mid, i + 2)[0] & 0xFFFFFFFF
            i += 3
        elif mid[i:i + 1] == b'\x25':
            bit = struct.unpack_from('<I', mid, i + 1)[0]
            i += 5
        else:
            return None
        if mid[i:i + 3] != b'\x85\xc0\x74' or i + 4 + mid[i + 3] != len(mid):
            return None
        i += 4
        if mid[i:i + 2] == b'\x83\x65':
            fo2 = struct.unpack_from('<b', mid, i + 2)[0]
            inv = struct.unpack_from('<b', mid, i + 3)[0] & 0xFFFFFFFF
            i += 4
        elif mid[i:i + 2] == b'\x83\xa5':
            fo2 = struct.unpack_from('<i', mid, i + 2)[0]
            inv = struct.unpack_from('<b', mid, i + 6)[0] & 0xFFFFFFFF
            i += 7
        elif mid[i:i + 2] == b'\x81\x65':
            fo2 = struct.unpack_from('<b', mid, i + 2)[0]
            inv = struct.unpack_from('<I', mid, i + 3)[0]
            i += 7
        elif mid[i:i + 2] == b'\x81\xa5':
            fo2 = struct.unpack_from('<i', mid, i + 2)[0]
            inv = struct.unpack_from('<I', mid, i + 6)[0]
            i += 10
        else:
            return None
        if fo2 != flag_off or (bit & inv) != 0 or (bit | inv) != 0xFFFFFFFF:
            return None
        if mid[i:i + 3] in (b'\x48\x8b\x4d', b'\x48\x8d\x4d'):
            is_deref = mid[i + 1] == 0x8b
            oo = struct.unpack_from('<b', mid, i + 3)[0]
            i += 4
        elif mid[i:i + 3] in (b'\x48\x8b\x8d', b'\x48\x8d\x8d'):
            is_deref = mid[i + 1] == 0x8b
            oo = struct.unpack_from('<i', mid, i + 3)[0]
            i += 7
        else:
            return None
        if oo >= 0 and mid[i] == 0xe8 and (i + 5 == len(mid) or (i + 6 == len(mid) and mid[i + 5] == 0x90)):
            tva = va + 9 + i + 5 + struct.unpack_from('<i', mid, i + 1)[0]
            if tva in all_funcs:
                fn = 'tpl_funclet_nrvo_deref' if is_deref else 'tpl_funclet_nrvo_lea'
                return f'{fn}::<0x{flag_off:x}, 0x{bit:x}, 0x{oo:x}, 0x{tva:x}>'
        return None

    def parse_compound(va, b):
        size = len(b)
        # 1. Array dtor loop:
        # 48 85 d2 74 <j1> 48 89 5c 24 08 57 48 83 ec 20 48 8b fa 48 8b d9 48 8b cb e8 <rel32> (90)? 48 83 c3 <s8>/48 81 c3 <s32> 48 83 ef 01 75 <j2> 48 8b 5c 24 30 48 83 c4 20 5f c3
        if (
            size >= 48
            and b[:4] == b'\x48\x85\xd2\x74'
            and b[5:25] == b'\x48\x89\x5c\x24\x08\x57\x48\x83\xec\x20\x48\x8b\xfa\x48\x8b\xd9\x48\x8b\xcb\xe8'
            and b[-10:] == b'\x48\x8b\x5c\x24\x30\x48\x83\xc4\x20\x5f\xc3'
            and 5 + b[4] == size - 1
        ):
            tva = va + 29 + struct.unpack_from('<i', b, 25)[0]
            if tva in all_funcs:
                idx = 30 if b[29] == 0x90 else 29
                rest = b[idx:-10]
                if len(rest) == 10 and rest[:3] == b'\x48\x83\xc3' and rest[4:9] == b'\x48\x83\xef\x01\x75':
                    stride = struct.unpack_from('<b', rest, 3)[0]
                    if stride > 0:
                        return f'tpl_array_dtor_loop::<0x{stride:x}, 0x{tva:x}>'
                elif len(rest) == 13 and rest[:3] == b'\x48\x81\xc3' and rest[7:12] == b'\x48\x83\xef\x01\x75':
                    stride = struct.unpack_from('<i', rest, 3)[0]
                    if stride > 0:
                        return f'tpl_array_dtor_loop::<0x{stride:x}, 0x{tva:x}>'

        # 2. Span loop [rcx]..[rcx+8]:
        # 48 89 5c 24 08 57 48 83 ec 20 48 8b 79 08 48 8b 19 48 3b df 74 <j1> 48 8b cb e8 <rel32> (90)? 48 83 c3 <s8>/48 81 c3 <s32> 48 3b df 75 <j2> 48 8b 5c 24 30 48 83 c4 20 5f c3
        if (
            size >= 48
            and b[:20] == b'\x48\x89\x5c\x24\x08\x57\x48\x83\xec\x20\x48\x8b\x79\x08\x48\x8b\x19\x48\x3b\xdf'
            and b[20] == 0x74
            and b[22:26] == b'\x48\x8b\xcb\xe8'
            and b[-10:] == b'\x48\x8b\x5c\x24\x30\x48\x83\xc4\x20\x5f\xc3'
            and 22 + b[21] == size - 10
        ):
            tva = va + 30 + struct.unpack_from('<i', b, 26)[0]
            if tva in all_funcs:
                idx = 31 if b[30] == 0x90 else 30
                rest = b[idx:-10]
                if len(rest) == 9 and rest[:3] == b'\x48\x83\xc3' and rest[4:8] == b'\x48\x3b\xdf\x75':
                    stride = struct.unpack_from('<b', rest, 3)[0]
                    if stride > 0:
                        return f'tpl_span_dtor_loop::<0x{stride:x}, 0x{tva:x}>'
                elif len(rest) == 12 and rest[:3] == b'\x48\x81\xc3' and rest[7:11] == b'\x48\x3b\xdf\x75':
                    stride = struct.unpack_from('<i', rest, 3)[0]
                    if stride > 0:
                        return f'tpl_span_dtor_loop::<0x{stride:x}, 0x{tva:x}>'

        # 3. Scalar deleting destructor:
        # 48 89 5c 24 08 57 48 83 ec 20 8b da 48 8b f9 e8 <dtor> (90)? f6 c3 01 74 0d ba <sz32> 48 8b cf e8 <free> (90)? (48 8b 5c 24 30 48 8b c7 | 48 8b c7 48 8b 5c 24 30) 48 83 c4 20 5f c3
        if (
            size in (52, 53, 54)
            and b[:16] == b'\x48\x89\x5c\x24\x08\x57\x48\x83\xec\x20\x8b\xda\x48\x8b\xf9\xe8'
            and b[-13:] in (
                b'\x48\x8b\x5c\x24\x30\x48\x8b\xc7\x48\x83\xc4\x20\x5f\xc3',
                b'\x48\x8b\xc7\x48\x8b\x5c\x24\x30\x48\x83\xc4\x20\x5f\xc3',
            )
        ):
            dtor_va = va + 20 + struct.unpack_from('<i', b, 16)[0]
            idx = 21 if b[20] == 0x90 else 20
            if b[idx:idx + 6] == b'\xf6\xc3\x01\x74\x0d\xba' and b[idx + 10:idx + 14] == b'\x48\x8b\xcf\xe8':
                sz = struct.unpack_from('<I', b, idx + 6)[0]
                free_va = va + idx + 18 + struct.unpack_from('<i', b, idx + 14)[0]
                end_idx = idx + 19 if b[idx + 18] == 0x90 else idx + 18
                if end_idx == size - 13 and dtor_va in all_funcs and free_va in all_funcs:
                    return f'tpl_scalar_del_dtor::<0x{dtor_va:x}, 0x{sz:x}, 0x{free_va:x}>'

        # 4. Deref rcx non-null call1 + call2:
        # 40 53 48 83 ec 20 48 8b 19 48 85 db 74 <j> 48 8b cb e8 <f1> (90)? 48 8b cb e8 <f2> (90)? 48 83 c4 20 5b c3
        if (
            size in (36, 37, 38)
            and b[:13] == b'\x40\x53\x48\x83\xec\x20\x48\x8b\x19\x48\x85\xdb\x74'
            and b[14:18] == b'\x48\x8b\xcb\xe8'
            and b[-6:] == b'\x48\x83\xc4\x20\x5b\xc3'
            and 14 + b[13] == size - 6
        ):
            f1 = va + 22 + struct.unpack_from('<i', b, 18)[0]
            idx = 23 if b[22] == 0x90 else 22
            if b[idx:idx + 4] == b'\x48\x8b\xcb\xe8':
                f2 = va + idx + 8 + struct.unpack_from('<i', b, idx + 4)[0]
                end_idx = idx + 9 if b[idx + 8] == 0x90 else idx + 8
                if end_idx == size - 6 and f1 in all_funcs and f2 in all_funcs:
                    return f'tpl_deref_nonnull_call2::<0x{f1:x}, 0x{f2:x}>'

        # 5. Two-subobject destructor / helper or call_ret_this:
        # 40 53 48 83 ec 20 48 8b d9 (add rcx, d1)? e8 <f1> (90)? ...
        if size >= 23 and b[:9] == b'\x40\x53\x48\x83\xec\x20\x48\x8b\xd9':
            i = 9
            if b[i:i + 3] == b'\x48\x83\xc1':
                d1 = struct.unpack_from('<b', b, i + 3)[0]
                i += 4
            elif b[i:i + 3] == b'\x48\x81\xc1':
                d1 = struct.unpack_from('<i', b, i + 3)[0]
                i += 7
            else:
                d1 = 0
            if d1 >= 0 and i < size and b[i] == 0xe8:
                f1 = va + i + 5 + struct.unpack_from('<i', b, i + 1)[0]
                i += 5
                if i < size and b[i] == 0x90:
                    i += 1
                if i == size - 9 and b[i:] == b'\x48\x8b\xc3\x48\x83\xc4\x20\x5b\xc3' and d1 == 0 and f1 in all_funcs:
                    return f'tpl_call_ret_this::<0x{f1:x}>'
                d2 = -1
                if b[i:i + 3] == b'\x48\x8d\x4b':
                    d2 = struct.unpack_from('<b', b, i + 3)[0]
                    i += 4
                elif b[i:i + 3] == b'\x48\x8d\x8b':
                    d2 = struct.unpack_from('<i', b, i + 3)[0]
                    i += 7
                elif b[i:i + 3] == b'\x48\x8b\xcb':
                    d2 = 0
                    i += 3
                if d2 >= 0 and f1 in all_funcs:
                    if b[i:i + 6] == b'\x48\x83\xc4\x20\x5b\xe9' and i + 10 == size:
                        f2 = va + size + struct.unpack_from('<i', b, i + 6)[0]
                        if f2 in all_funcs:
                            return f'tpl_subobj2_call_jmp::<0x{d1:x}, 0x{f1:x}, 0x{d2:x}, 0x{f2:x}>'
                    elif i < size and b[i] == 0xe8:
                        f2 = va + i + 5 + struct.unpack_from('<i', b, i + 1)[0]
                        i += 5
                        if i < size and b[i] == 0x90:
                            i += 1
                        if b[i:] == b'\x48\x83\xc4\x20\x5b\xc3' and f2 in all_funcs:
                            return f'tpl_subobj2_call_jmp::<0x{d1:x}, 0x{f1:x}, 0x{d2:x}, 0x{f2:x}>'

        # 6. Array index scale + swap rcx/rdx tail-jump:
        # Variant A: 4c 8b c9 41 8b c0 <scale rax->rcx> 48 03 ca 49 8b d1 e9 <rel32>
        if size in (21, 22, 24, 25) and b[:6] == b'\x4c\x8b\xc9\x41\x8b\xc0' and b[-11:-5] == b'\x48\x03\xca\x49\x8b\xd1' and b[-5] == 0xe9:
            tva = va + size + struct.unpack_from('<i', b, size - 4)[0]
            if is_target_int_only(tva):
                sc = b[6:-11]
                stride = None
                if len(sc) == 4 and sc[:3] == b'\x48\x6b\xc8':
                    stride = struct.unpack_from('<b', sc, 3)[0]
                elif len(sc) == 7 and sc[:3] == b'\x48\x69\xc8':
                    stride = struct.unpack_from('<i', sc, 3)[0]
                elif len(sc) == 4 and sc[:3] == b'\x48\xc1\xe1':
                    stride = 1 << sc[3]
                elif len(sc) == 8 and sc[:3] == b'\x48\x8d\x0c' and sc[4:7] == b'\x48\xc1\xe1':
                    sib = sc[3]
                    mult = {0x40: 3, 0x80: 5, 0xc0: 9}.get(sib)
                    if mult:
                        stride = mult << sc[7]
                if stride is not None and stride > 0:
                    return f'tpl_idx_r8d_swap_rcx_rdx_jmp::<0x{stride:x}, 0x{tva:x}>'

        # Variant B: 48 8b c1 41 8b c8 <scale rcx> 48 03 ca 48 8b d0 e9 <rel32>
        if size in (21, 24, 25) and b[:6] == b'\x48\x8b\xc1\x41\x8b\xc8' and b[-11:-5] == b'\x48\x03\xca\x48\x8b\xd0' and b[-5] == 0xe9:
            tva = va + size + struct.unpack_from('<i', b, size - 4)[0]
            if is_target_int_only(tva):
                sc = b[6:-11]
                stride = None
                if len(sc) == 4 and sc[:3] == b'\x48\xc1\xe1':
                    stride = 1 << sc[3]
                elif len(sc) == 4 and sc[:3] == b'\x48\x6b\xc9':
                    stride = struct.unpack_from('<b', sc, 3)[0]
                elif len(sc) == 7 and sc[:3] == b'\x48\x69\xc9':
                    stride = struct.unpack_from('<i', sc, 3)[0]
                if stride is not None and stride > 0:
                    return f'tpl_idx_r8d_swap_rcx_rdx_jmp::<0x{stride:x}, 0x{tva:x}>'

        return None

    rows = db.execute(
        "SELECT va, size, name, n_frags FROM functions WHERE owner = 'remedy' ORDER BY va"
    ).fetchall()

    p1 = bytes.fromhex('48 8b ca 49 8b d0 e9')
    p5 = bytes.fromhex('49 8b c0 48 8b ca 48 8b d0 45 0f b6 c1 e9')
    p13 = bytes.fromhex('49 8b d0 45 0f b6 c1 e9')
    p15_pre = bytes.fromhex('48 83 ec 28 4c 63 ca e8')
    p15_post = bytes.fromhex('48 83 c4 28 c3')
    p34_pre = bytes.fromhex('48 85 d2 74 08 48 8b ca e9')
    p34_post = bytes.fromhex('c3')
    p_call_fwd8_pre = bytes.fromhex('48 83 ec 28 e8')
    p_call_fwd8_post = bytes.fromhex('48 83 c4 28 c3')
    p_call_nop_post = bytes.fromhex('90 48 83 c4 28 c3')

    entries = []
    counts = {}

    for va, size, name, n_frags in rows:
        if va in excl_vas or n_frags > 0:
            continue
        rva = va - pe.image_base
        off = rva - pe.text_rva
        if off < 0 or off + size > len(pe.text):
            continue
        b = pe.text[off:off + size]

        ptr_expr = None

        if size == 11 and b[:7] == p1:
            rel32 = struct.unpack_from('<i', b, 7)[0]
            target = va + 11 + rel32
            ptr_expr = f'tpl_shift2_jmp::<0x{target:x}>'
        elif size == 18 and b[:14] == p5:
            rel32 = struct.unpack_from('<i', b, 14)[0]
            target = va + 18 + rel32
            ptr_expr = f'tpl_shift3_u8_jmp::<0x{target:x}>'
        elif size == 12 and b[:8] == p13:
            rel32 = struct.unpack_from('<i', b, 8)[0]
            target = va + 12 + rel32
            ptr_expr = f'tpl_keep1_shift23_u8_jmp::<0x{target:x}>'
        elif size == 17 and b[:8] == p15_pre and b[12:17] == p15_post:
            rel32 = struct.unpack_from('<i', b, 8)[0]
            target = va + 12 + rel32
            ptr_expr = f'tpl_sext_edx_to_r9_call::<0x{target:x}>'
        elif size == 14 and b[:9] == p34_pre and b[13:14] == p34_post:
            rel32 = struct.unpack_from('<i', b, 9)[0]
            target = va + 13 + rel32
            ptr_expr = f'tpl_if_rdx_shift1_jmp::<0x{target:x}>'
        elif size == 14 and b[:5] == p_call_fwd8_pre and b[9:14] == p_call_fwd8_post:
            rel32 = struct.unpack_from('<i', b, 5)[0]
            target = va + 9 + rel32
            if is_target_int_only(target):
                ptr_expr = f'tpl_jmp_fwd8::<0x{target:x}>'
        elif size == 15 and b[:5] == p_call_fwd8_pre and b[9:15] == p_call_nop_post:
            rel32 = struct.unpack_from('<i', b, 5)[0]
            target = va + 9 + rel32
            if is_target_int_only(target):
                ptr_expr = f'tpl_jmp_fwd8::<0x{target:x}>'
        elif size in (17, 18) and b[:8] == b'\x48\x83\xec\x28\x48\x8b\xca\xe8' and b[12:] in (p_call_fwd8_post, p_call_nop_post):
            rel32 = struct.unpack_from('<i', b, 8)[0]
            target = va + 12 + rel32
            if is_target_int_only(target):
                ptr_expr = f'tpl_mov_rcx_rdx_jmp::<0x{target:x}>'
        elif size in (18, 19) and b[:7] == b'\x48\x83\xec\x28\x48\x83\xc1' and b[8] == 0xe8 and b[13:] in (p_call_fwd8_post, p_call_nop_post):
            disp = struct.unpack_from('<b', b, 7)[0] & 0xFFFFFFFFFFFFFFFF
            rel32 = struct.unpack_from('<i', b, 9)[0]
            target = va + 13 + rel32
            if is_target_int_only(target):
                ptr_expr = f'tpl_add_rcx_jmp::<0x{disp:x}, 0x{target:x}>'
        elif size == 13 and b[:8] == b'\x48\x8b\x09\x48\x85\xc9\x0f\x85' and b[12:13] == b'\xc3':
            rel32 = struct.unpack_from('<i', b, 8)[0]
            target = va + 12 + rel32
            if is_target_int_only(target):
                ptr_expr = f'tpl_deref_rcx_if_nonnull_jmp::<0x{target:x}>'
        elif size == 5 and b[0] == 0xe9:
            rel32 = struct.unpack_from('<i', b, 1)[0]
            target = va + 5 + rel32
            if is_target_int_only(target):
                ptr_expr = f'tpl_jmp_fwd8::<0x{target:x}>'
        elif size >= 7 and b[-5] == 0xe9:
            prefix = b[:-5]
            rel32 = struct.unpack_from('<i', b, size - 4)[0]
            target = va + size + rel32
            if is_target_int_only(target):
                # Do NOT touch 48 8d 0d <rip> e9 <rel32> (owned by Shard C)
                if len(prefix) == 4 and prefix[:3] in (b'\x48\x83\xc1', b'\x48\x8d\x49'):
                    disp = struct.unpack_from('<b', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_add_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif len(prefix) == 7 and prefix[:3] in (b'\x48\x81\xc1', b'\x48\x8d\x89'):
                    disp = struct.unpack_from('<i', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_add_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif len(prefix) == 4 and prefix[:3] == b'\x48\x83\xe9':
                    disp = struct.unpack_from('<b', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_sub_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif len(prefix) == 7 and prefix[:3] == b'\x48\x81\xe9':
                    disp = struct.unpack_from('<i', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_sub_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif prefix == b'\x48\x8b\x09':
                    ptr_expr = f'tpl_deref_rcx_jmp::<0x0, 0x{target:x}>'
                elif len(prefix) == 4 and prefix[:3] == b'\x48\x8b\x49':
                    disp = struct.unpack_from('<b', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_deref_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif len(prefix) == 7 and prefix[:3] == b'\x48\x8b\x89':
                    disp = struct.unpack_from('<i', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_deref_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif len(prefix) == 4 and prefix[:3] == b'\x48\x8d\x4a':
                    disp = struct.unpack_from('<b', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_lea_rcx_from_rdx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif len(prefix) == 7 and prefix[:3] == b'\x48\x8d\x8a':
                    disp = struct.unpack_from('<i', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_lea_rcx_from_rdx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif prefix == b'\x48\x8b\x0a':
                    ptr_expr = f'tpl_deref_rdx_to_rcx_jmp::<0x0, 0x{target:x}>'
                elif len(prefix) == 4 and prefix[:3] == b'\x48\x8b\x4a':
                    disp = struct.unpack_from('<b', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_deref_rdx_to_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif len(prefix) == 7 and prefix[:3] == b'\x48\x8b\x8a':
                    disp = struct.unpack_from('<i', prefix, 3)[0]
                    disp_u64 = disp & 0xFFFFFFFFFFFFFFFF
                    ptr_expr = f'tpl_deref_rdx_to_rcx_jmp::<0x{disp_u64:x}, 0x{target:x}>'
                elif prefix[:3] in (b'\x48\x8b\x0a', b'\x48\x8b\x4a', b'\x48\x8b\x8a'):
                    p1_len = 3 if prefix[:3] == b'\x48\x8b\x0a' else (4 if prefix[:3] == b'\x48\x8b\x4a' else 7)
                    d1 = 0 if p1_len == 3 else (struct.unpack_from('<b', prefix, 3)[0] if p1_len == 4 else struct.unpack_from('<i', prefix, 3)[0])
                    p2 = prefix[p1_len:]
                    d2 = None
                    if len(p2) == 4 and p2[:3] == b'\x48\x83\xc1':
                        d2 = struct.unpack_from('<b', p2, 3)[0]
                    elif len(p2) == 7 and p2[:3] == b'\x48\x81\xc1':
                        d2 = struct.unpack_from('<i', p2, 3)[0]
                    if d2 is not None:
                        d1_u64 = d1 & 0xFFFFFFFFFFFFFFFF
                        d2_u64 = d2 & 0xFFFFFFFFFFFFFFFF
                        ptr_expr = f'tpl_deref_rdx_add_rcx_jmp::<0x{d1_u64:x}, 0x{d2_u64:x}, 0x{target:x}>'
                elif prefix == b'\x48\x8b\xca':
                    ptr_expr = f'tpl_mov_rcx_rdx_jmp::<0x{target:x}>'
                elif prefix == b'\x48\x8b\xd1':
                    ptr_expr = f'tpl_mov_rdx_rcx_jmp::<0x{target:x}>'
                elif prefix == b'\x49\x8b\xc8':
                    ptr_expr = f'tpl_mov_rcx_r8_jmp::<0x{target:x}>'
                elif prefix == b'\x48\x8b\xc2\x48\x8b\xd1\x48\x8b\xc8':
                    ptr_expr = f'tpl_swap_rcx_rdx_jmp::<0x{target:x}>'
                elif prefix == b'\x4c\x8b\xc2\x48\x8b\xd1':
                    ptr_expr = f'tpl_shift_rcx_rdx_to_rdx_r8_jmp::<0x{target:x}>'
                elif prefix == b'\x48\x8b\x12\x48\x83\xc1\x08':
                    ptr_expr = f'tpl_deref_rdx_add_rcx8_jmp::<0x{target:x}>'
                elif prefix == b'\x48\x83\xc2\x08\x49\x8d\x48\x08':
                    ptr_expr = f'tpl_add_rdx8_lea_rcx_r8_8_jmp::<0x{target:x}>'
                elif prefix == b'\x49\x8b\x48\x08\x49\x2b\x08\x48\x03\xca':
                    ptr_expr = f'tpl_span_end_offset_r8_jmp::<0x0, 0x{target:x}>'
                elif prefix == b'\x49\x8b\x48\x08\x48\x83\xc2\x08\x49\x2b\x08\x48\x03\xca':
                    ptr_expr = f'tpl_span_end_offset_r8_jmp::<0x8, 0x{target:x}>'
                elif prefix == b'\x48\x8d\x41\x08\x48\x8b\x4a\x08\x48\x2b\x0a\x48\x8b\xd0\x48\x03\xc8':
                    ptr_expr = f'tpl_span_end_offset_rcx8_jmp::<0x{target:x}>'
                elif prefix in (b'\x33\xd2', b'\x31\xd2'):
                    ptr_expr = f'tpl_xor_edx_jmp::<0x{target:x}>'
                elif prefix == b'\x45\x33\xc0':
                    ptr_expr = f'tpl_xor_r8d_jmp::<0x{target:x}>'
                elif prefix == b'\x45\x33\xc9':
                    ptr_expr = f'tpl_xor_r9d_jmp::<0x{target:x}>'
                elif len(prefix) == 5 and prefix[0] == 0xba:
                    imm32 = struct.unpack_from('<I', prefix, 1)[0]
                    ptr_expr = f'tpl_mov_edx_jmp::<0x{imm32:x}, 0x{target:x}>'
                elif len(prefix) == 2 and prefix[0] == 0xb2:
                    imm8 = prefix[1]
                    ptr_expr = f'tpl_mov_dl_jmp::<0x{imm8:x}, 0x{target:x}>'
                elif len(prefix) == 6 and prefix[:2] == b'\x41\xb8':
                    imm32 = struct.unpack_from('<I', prefix, 2)[0]
                    ptr_expr = f'tpl_mov_r8d_jmp::<0x{imm32:x}, 0x{target:x}>'
                elif len(prefix) == 3 and prefix[:2] == b'\x41\xb0':
                    imm8 = prefix[2]
                    ptr_expr = f'tpl_mov_r8b_jmp::<0x{imm8:x}, 0x{target:x}>'

        if ptr_expr is None and size <= 96:
            ptr_expr = parse_funclet(va, b) or parse_compound(va, b)

        if ptr_expr is None:
            continue

        eh = fast_eh_info(pe, va)
        mask_direct = can_throw_map.get(va, 0)
        if not (eh in ('none', 'noexcept') or (eh == 'cleanup' and (mask_direct & 21) == 0)):
            continue

        base_fn = ptr_expr.split('::')[0]
        counts[base_fn] = counts.get(base_fn, 0) + 1
        fn_name = name if name else f'sub_{va:x}'
        fn_name_escaped = fn_name.replace('\\', '\\\\').replace('"', '\\"')
        # Check mask_direct & 23 != 0 (CXX | CONTRACT | SEH | STD)
        throws_discard = (mask_direct & 23) != 0
        entries.append((va, fn_name_escaped, ptr_expr, throws_discard))

    # Re-read sibling shard files right before writing to ensure 0 duplicate VAs
    latest_excl = load_excluded_vas()
    entries = [e for e in entries if e[0] not in latest_excl]

    lines = [
        '//! Shard B: Tail-jump & argument-forwarding thunk templates.',
        '//! Generated by `tools/decomp_scale/gen_shard_b.py`.',
        '',
        'use crate::registry::{Replacement, Throws};',
        '',
        '#[inline(always)]',
        'fn exe_va(va: u64) -> *const u8 {',
        '    crate::exe::ptr::<u8>(va)',
        '}',
        '',
        '/// Template #1 (11 bytes): `mov rcx, rdx; mov rdx, r8; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_shift2_jmp<const TARGET_VA: u64>(_rcx: u64, rdx: u64, r8: u64) -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64) -> u64 = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rdx, r8) }',
        '}',
        '',
        '/// Template #5 (18 bytes): `mov rax, r8; mov rcx, rdx; mov rdx, rax; movzx r8d, r9b; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_shift3_u8_jmp<const TARGET_VA: u64>(_rcx: u64, rdx: u64, r8: u64, r9: u8) -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64) -> u64 = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rdx, r8, r9 as u64) }',
        '}',
        '',
        '/// Template #13 (12 bytes): `mov rdx, r8; movzx r8d, r9b; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_keep1_shift23_u8_jmp<const TARGET_VA: u64>(rcx: u64, _rdx: u64, r8: u64, r9: u8) -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64) -> u64 = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, r8, r9 as u64) }',
        '}',
        '',
        '/// Template #15 (17 bytes): `sub rsp, 0x28; movsxd r9, edx; call TARGET_VA; add rsp, 0x28; ret`',
        'pub unsafe extern "win64" fn tpl_sext_edx_to_r9_call<const TARGET_VA: u64>(rcx: u64, rdx: i32, r8: u64) -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, i32, u64, i64) -> u64 = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, r8, rdx as i64) }',
        '}',
        '',
        '/// Template #34 (14 bytes): `test rdx, rdx; jz +8; mov rcx, rdx; jmp TARGET_VA; ret`',
        'pub unsafe extern "win64" fn tpl_if_rdx_shift1_jmp<const TARGET_VA: u64>(_rcx: u64, rdx: u64, r8: u64, r9: u64) -> u64 {',
        '    if rdx != 0 {',
        '        let f: unsafe extern "win64" fn(u64, u64, u64) -> u64 = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '        unsafe { f(rdx, r8, r9) }',
        '    } else {',
        '        0',
        '    }',
        '}',
        '',
        '/// Pure 5-byte tail-jump or 14/15-byte call-forwarding thunk: `jmp TARGET_VA` / `sub rsp, 0x28; call TARGET_VA; add rsp, 0x28; ret`',
        'pub unsafe extern "win64" fn tpl_jmp_fwd8<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `add rcx, DISP; jmp TARGET_VA` / `lea rcx, [rcx + DISP]; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_add_rcx_jmp<const DISP: u64, const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx.wrapping_add(DISP), rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `sub rcx, DISP; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_sub_rcx_jmp<const DISP: u64, const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx.wrapping_sub(DISP), rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rcx, [rcx + DISP]; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_deref_rcx_jmp<const DISP: u64, const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let next_rcx = unsafe { crate::exe::rd::<u64>(rcx.wrapping_add(DISP) as usize) };',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(next_rcx, rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rcx, [rcx]; test rcx, rcx; jnz TARGET_VA; ret`',
        'pub unsafe extern "win64" fn tpl_deref_rcx_if_nonnull_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let next_rcx = unsafe { crate::exe::rd::<u64>(rcx as usize) };',
        '    if next_rcx != 0 {',
        '        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '            unsafe { crate::exe::func(TARGET_VA as u64) };',
        '        unsafe { f(next_rcx, rdx, r8, r9, s5, s6, s7, s8) }',
        '    } else {',
        '        0',
        '    }',
        '}',
        '',
        '/// `lea rcx, [rdx + DISP]; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_lea_rcx_from_rdx_jmp<const DISP: u64, const TARGET_VA: u64>(',
        '    _rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rdx.wrapping_add(DISP), rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rcx, [rdx + DISP]; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_deref_rdx_to_rcx_jmp<const DISP: u64, const TARGET_VA: u64>(',
        '    _rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let next_rcx = unsafe { crate::exe::rd::<u64>(rdx.wrapping_add(DISP) as usize) };',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(next_rcx, rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rcx, [rdx + D1]; add rcx, D2; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_deref_rdx_add_rcx_jmp<const D1: u64, const D2: u64, const TARGET_VA: u64>(',
        '    _rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let base_rcx = unsafe { crate::exe::rd::<u64>(rdx.wrapping_add(D1) as usize) };',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(base_rcx.wrapping_add(D2), rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rcx, rdx; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_mov_rcx_rdx_jmp<const TARGET_VA: u64>(',
        '    _rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rdx, rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rdx, rcx; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_mov_rdx_rcx_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, _rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rcx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rcx, r8; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_mov_rcx_r8_jmp<const TARGET_VA: u64>(',
        '    _rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(r8, rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rax, rdx; mov rdx, rcx; mov rcx, rax; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_swap_rcx_rdx_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rdx, rcx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov r8, rdx; mov rdx, rcx; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_shift_rcx_rdx_to_rdx_r8_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, _r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rcx, rdx, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rdx, [rdx]; add rcx, 8; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_deref_rdx_add_rcx8_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let next_rdx = unsafe { crate::exe::rd::<u64>(rdx as usize) };',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx.wrapping_add(8), next_rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `add rdx, 8; lea rcx, [r8 + 8]; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_add_rdx8_lea_rcx_r8_8_jmp<const TARGET_VA: u64>(',
        '    _rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(r8.wrapping_add(8), rdx.wrapping_add(8), r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov rcx, [r8 + 8]; (add rdx, RDX_ADD)?; sub rcx, [r8]; add rcx, rdx; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_span_end_offset_r8_jmp<const RDX_ADD: u64, const TARGET_VA: u64>(',
        '    _rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let end_p = unsafe { crate::exe::rd::<u64>((r8 as usize).wrapping_add(8)) };',
        '    let begin_p = unsafe { crate::exe::rd::<u64>(r8 as usize) };',
        '    let new_rdx = rdx.wrapping_add(RDX_ADD);',
        '    let new_rcx = end_p.wrapping_sub(begin_p).wrapping_add(new_rdx);',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(new_rcx, new_rdx, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `lea rax, [rcx + 8]; mov rcx, [rdx + 8]; sub rcx, [rdx]; mov rdx, rax; add rcx, rax; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_span_end_offset_rcx8_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let rax = rcx.wrapping_add(8);',
        '    let end_p = unsafe { crate::exe::rd::<u64>((rdx as usize).wrapping_add(8)) };',
        '    let begin_p = unsafe { crate::exe::rd::<u64>(rdx as usize) };',
        '    let new_rcx = end_p.wrapping_sub(begin_p).wrapping_add(rax);',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(new_rcx, rax, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `xor edx, edx; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_xor_edx_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, _rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, 0, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `xor r8d, r8d; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_xor_r8d_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, _r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, 0, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `xor r9d, r9d; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_xor_r9d_jmp<const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, _r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, r8, 0, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov edx, IMM; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_mov_edx_jmp<const IMM: u32, const TARGET_VA: u64>(',
        '    rcx: u64, _rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, IMM as u64, r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov dl, IMM; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_mov_dl_jmp<const IMM: u8, const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, (rdx & !0xff) | (IMM as u64), r8, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov r8d, IMM; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_mov_r8d_jmp<const IMM: u32, const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, _r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, IMM as u64, r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// `mov r8b, IMM; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_mov_r8b_jmp<const IMM: u8, const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u64, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, (r8 & !0xff) | (IMM as u64), r9, s5, s6, s7, s8) }',
        '}',
        '',
        '/// Unwind funclet: `if ([rdx + FLAG_OFF] & BIT) { [rdx + FLAG_OFF] &= ~BIT; TARGET_VA(*[rdx + OBJ_OFF]); }`',
        'pub unsafe extern "win64" fn tpl_funclet_nrvo_deref<const FLAG_OFF: usize, const BIT: u32, const OBJ_OFF: usize, const TARGET_VA: u64>(_rcx: usize, rdx: usize) {',
        '    let flag_addr = rdx.wrapping_add(FLAG_OFF);',
        '    let flags = unsafe { crate::exe::rd::<u32>(flag_addr) };',
        '    if (flags & BIT) != 0 {',
        '        unsafe { crate::exe::wr::<u32>(flag_addr, flags & !BIT) };',
        '        let obj = unsafe { crate::exe::rd::<usize>(rdx.wrapping_add(OBJ_OFF)) };',
        '        let f: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '        unsafe { f(obj) };',
        '    }',
        '}',
        '',
        '/// Unwind funclet: `if ([rdx + FLAG_OFF] & BIT) { [rdx + FLAG_OFF] &= ~BIT; TARGET_VA(rdx + OBJ_OFF); }`',
        'pub unsafe extern "win64" fn tpl_funclet_nrvo_lea<const FLAG_OFF: usize, const BIT: u32, const OBJ_OFF: usize, const TARGET_VA: u64>(_rcx: usize, rdx: usize) {',
        '    let flag_addr = rdx.wrapping_add(FLAG_OFF);',
        '    let flags = unsafe { crate::exe::rd::<u32>(flag_addr) };',
        '    if (flags & BIT) != 0 {',
        '        unsafe { crate::exe::wr::<u32>(flag_addr, flags & !BIT) };',
        '        let obj = rdx.wrapping_add(OBJ_OFF);',
        '        let f: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '        unsafe { f(obj) };',
        '    }',
        '}',
        '',
        '/// Unwind funclet: `TARGET_VA(*[rdx + OBJ_OFF] + ADD_OFF)`',
        'pub unsafe extern "win64" fn tpl_funclet_deref_call<const OBJ_OFF: usize, const ADD_OFF: usize, const TARGET_VA: u64>(_rcx: usize, rdx: usize) {',
        '    let obj = unsafe { crate::exe::rd::<usize>(rdx.wrapping_add(OBJ_OFF)) }.wrapping_add(ADD_OFF);',
        '    let f: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(obj) };',
        '}',
        '',
        '/// Unwind funclet: `TARGET_VA(rdx + OBJ_OFF + ADD_OFF)`',
        'pub unsafe extern "win64" fn tpl_funclet_lea_call<const OBJ_OFF: usize, const ADD_OFF: usize, const TARGET_VA: u64>(_rcx: usize, rdx: usize) {',
        '    let obj = rdx.wrapping_add(OBJ_OFF).wrapping_add(ADD_OFF);',
        '    let f: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(obj) };',
        '}',
        '',
        '/// Unwind funclet: `TARGET_VA(*[rdx + OBJ_OFF], EDX_IMM)`',
        'pub unsafe extern "win64" fn tpl_funclet_deref_edx_call<const OBJ_OFF: usize, const EDX_IMM: u32, const TARGET_VA: u64>(_rcx: usize, rdx: usize) {',
        '    let obj = unsafe { crate::exe::rd::<usize>(rdx.wrapping_add(OBJ_OFF)) };',
        '    let f: unsafe extern "win64" fn(usize, u32) = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(obj, EDX_IMM) };',
        '}',
        '',
        '/// Array element loop: `for i in 0..rdx { TARGET_VA(rcx + i * STRIDE); }`',
        'pub unsafe extern "win64" fn tpl_array_dtor_loop<const STRIDE: usize, const TARGET_VA: u64>(mut rcx: usize, mut rdx: usize) {',
        '    if rdx == 0 {',
        '        return;',
        '    }',
        '    let f: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    while rdx != 0 {',
        '        unsafe { f(rcx) };',
        '        rcx = rcx.wrapping_add(STRIDE);',
        '        rdx -= 1;',
        '    }',
        '}',
        '',
        '/// Span element loop: `for p in [rcx]..[rcx+8] step STRIDE { TARGET_VA(p); }`',
        'pub unsafe extern "win64" fn tpl_span_dtor_loop<const STRIDE: usize, const TARGET_VA: u64>(rcx: usize) {',
        '    let end_p = unsafe { crate::exe::rd::<usize>(rcx.wrapping_add(8)) };',
        '    let mut cur = unsafe { crate::exe::rd::<usize>(rcx) };',
        '    if cur == end_p {',
        '        return;',
        '    }',
        '    let f: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    while cur != end_p {',
        '        unsafe { f(cur) };',
        '        cur = cur.wrapping_add(STRIDE);',
        '    }',
        '}',
        '',
        '/// Scalar deleting destructor: `DTOR_VA(this); if (flags & 1) { FREE_VA(this, SIZE); } return this;`',
        'pub unsafe extern "win64" fn tpl_scalar_del_dtor<const DTOR_VA: u64, const SIZE: u32, const FREE_VA: u64>(this: usize, flags: u32) -> usize {',
        '    let dtor: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(DTOR_VA as u64) };',
        '    unsafe { dtor(this) };',
        '    if (flags & 1) != 0 {',
        '        let free_fn: unsafe extern "win64" fn(usize, u32) = unsafe { crate::exe::func(FREE_VA as u64) };',
        '        unsafe { free_fn(this, SIZE) };',
        '    }',
        '    this',
        '}',
        '',
        '/// Deref non-null call2: `let p = *[rcx]; if p != 0 { F1_VA(p); F2_VA(p); }`',
        'pub unsafe extern "win64" fn tpl_deref_nonnull_call2<const F1_VA: u64, const F2_VA: u64>(rcx: usize) {',
        '    let p = unsafe { crate::exe::rd::<usize>(rcx) };',
        '    if p != 0 {',
        '        let f1: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(F1_VA as u64) };',
        '        let f2: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(F2_VA as u64) };',
        '        unsafe {',
        '            f1(p);',
        '            f2(p);',
        '        }',
        '    }',
        '}',
        '',
        '/// Call helper and return `this` (`rbx = rcx; F_VA(rcx); return rbx;`)',
        'pub unsafe extern "win64" fn tpl_call_ret_this<const F_VA: u64>(this: usize) -> usize {',
        '    let f: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(F_VA as u64) };',
        '    unsafe { f(this) };',
        '    this',
        '}',
        '',
        '/// Two-subobject destructor/helper: `F1_VA(this + D1); return F2_VA(this + D2);`',
        'pub unsafe extern "win64" fn tpl_subobj2_call_jmp<const D1: usize, const F1_VA: u64, const D2: usize, const F2_VA: u64>(this: usize) -> u64 {',
        '    let f1: unsafe extern "win64" fn(usize) = unsafe { crate::exe::func(F1_VA as u64) };',
        '    let f2: unsafe extern "win64" fn(usize) -> u64 = unsafe { crate::exe::func(F2_VA as u64) };',
        '    unsafe {',
        '        f1(this.wrapping_add(D1));',
        '        f2(this.wrapping_add(D2))',
        '    }',
        '}',
        '',
        '/// Array index scale + swap rcx/rdx tail-jump: `TARGET_VA(rdx + (r8 as u32 as u64) * STRIDE, rcx)`',
        'pub unsafe extern "win64" fn tpl_idx_r8d_swap_rcx_rdx_jmp<const STRIDE: u64, const TARGET_VA: u64>(',
        '    rcx: u64, rdx: u64, r8: u32, r9: u64,',
        '    s5: u64, s6: u64, s7: u64, s8: u64,',
        ') -> u64 {',
        '    let elem = rdx.wrapping_add((r8 as u64).wrapping_mul(STRIDE));',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(elem, rcx, r8 as u64, r9, s5, s6, s7, s8) }',
        '}',
        '',
        'pub static ENTRIES: &[Replacement] = &[',
    ]

    for va, fn_name, ptr_expr, throws_discard in entries:
        full_ptr = f'{ptr_expr} as *const ()'
        if throws_discard:
            lines.append(f'    Replacement::draft_throws(0x{va:x}, "{fn_name}", {full_ptr}, Throws::Discard),')
        else:
            lines.append(f'    Replacement::draft(0x{va:x}, "{fn_name}", {full_ptr}),')

    lines.append('];')
    lines.append('')

    with open(out_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))

    print(f'Generated {len(entries)} entries in {out_path}: {counts}')


if __name__ == '__main__':
    main()
