#!/usr/bin/env python3
"""Generate crates/aw2-game/src/shards/templates_a.rs for Shard A."""

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
    ctor_re = re.compile(r'Replacement::(?:draft(?:_diff|_throws|_throws_diff)?|verified(?:_throws)?)\s*\(\s*(0x[0-9a-fA-F_]+)')
    va_re = re.compile(r'\bva\s*:\s*(0x[0-9a-fA-F_]+)')
    for sub in ('registry', 'shards'):
        base_d = os.path.join(src_root, sub)
        for dirpath, _, files in os.walk(base_d):
            if os.path.basename(dirpath) == 'templates_a':
                continue
            for fn in sorted(files):
                if not fn.endswith('.rs') or fn in ('templates_a.rs', 'mod.rs'):
                    continue
                p = os.path.join(dirpath, fn)
                with open(p, 'r', encoding='utf-8') as f:
                    for line in f:
                        s = line.lstrip()
                        if s.startswith('//'):
                            continue
                        m = ctor_re.search(s) or va_re.search(s)
                        if m:
                            vas.add(int(m.group(1).replace('_', ''), 16))
    return vas


def parse_vjmp_rax_tail(b, prefix_len):
    """If b[prefix_len:] is `48 ff 20`, `48 ff 60 <disp8>`, or `48 ff a0 <disp32>`,
    return non-negative VTABLE_OFF or None."""
    tail = b[prefix_len:]
    if tail == b'\x48\xff\x20':
        return 0
    if len(tail) == 4 and tail[:3] == b'\x48\xff\x60':
        off = struct.unpack_from('<b', tail, 3)[0]
        return off if off >= 0 else None
    if len(tail) == 7 and tail[:3] == b'\x48\xff\xa0':
        off = struct.unpack_from('<i', tail, 3)[0]
        return off if off >= 0 else None
    return None


def match_template(b):
    size = len(b)

    # 1. Zero-operand exact leaves
    if b in (b'\xc3', b'\xc2\x00\x00'):
        return 'tpl_ret_void'
    if b == b'\xb0\x01\xc3':
        return 'tpl_ret_u8_1'
    if b in (b'\x32\xc0\xc3', b'\x31\xc0\xc3', b'\x33\xc0\xc3'):
        return 'tpl_ret_zero'
    if b == b'\x48\x8b\xc2\xc3':
        return 'tpl_ret_rdx'
    if b == b'\x48\x8b\xc1\xc3':
        return 'tpl_ret_rcx'
    if b == b'\x48\x8b\x01\xc3':
        return 'tpl_deref_rcx_u64'

    # 1b. High-frequency exact pointer/struct/branch leaves
    if b == b'\x48\x8b\xc1\x48\x85\xc9\x74\x04\x48\x8d\x04\x11\xc3':
        return 'tpl_if_rcx_add_rdx'
    if b == b'\x48\x89\x11\x48\x8b\xc1\xc3':
        return 'tpl_store_rcx0_rdx_ret_rcx'
    if b == b'\x48\x8b\x02\x48\x89\x01\x48\x8b\xc1\xc3':
        return 'tpl_copy_u64_rdx_to_rcx_ret_rcx'
    if b == b'\x33\xc0\x48\x89\x01\x48\x89\x41\x08\x48\x8b\xc1\xc3':
        return 'tpl_zero_rcx_16_ret_rcx'
    if b == b'\x48\x8b\x02\x48\x39\x01\x0f\x95\xc0\xc3':
        return 'tpl_cmp_ne_deref_rcx_rdx_u64'
    if b == b'\x48\x8b\x02\x48\x39\x01\x0f\x94\xc0\xc3':
        return 'tpl_cmp_eq_deref_rcx_rdx_u64'
    if b == b'\x48\x8b\x02\x49\x89\x00\xc3':
        return 'tpl_copy_u64_rdx_to_r8'
    if b == b'\x48\xc7\x02\x00\x00\x00\x00\x48\x8b\xc2\xc3':
        return 'tpl_zero_rdx0_ret_rdx'
    if b == b'\x48\x8b\x01\x48\xc7\x01\x00\x00\x00\x00\xc3':
        return 'tpl_take_rcx0_u64'
    if b == b'\x48\x8b\x41\x08\x48\x89\x01\xc3':
        return 'tpl_copy_rcx8_to_rcx0'
    if b == b'\x48\x89\x11\x48\x8b\xc1\x48\x89\x51\x08\xc3':
        return 'tpl_store_rcx0_rcx8_rdx_ret_rcx'
    if b == b'\x48\x89\x11\x48\x8b\xc1\x4c\x89\x41\x08\xc3':
        return 'tpl_store_rcx0_rdx_rcx8_r8_ret_rcx'
    if b == b'\x48\x8b\x01\x48\x89\x02\x48\x8b\xc2\xc3':
        return 'tpl_copy_u64_rcx_to_rdx_ret_rdx'
    if b == b'\x48\x8b\x01\x45\x8b\xc0\x4a\x89\x04\xc2\xc3':
        return 'tpl_store_idx8_rdx_r8_from_rcx0'
    if b == b'\x33\xc0\x48\x89\x01\x48\x89\x41\x08\xc3':
        return 'tpl_zero_rcx_16'
    if b == b'\x0f\xb6\x42\x08\x41\x88\x40\x08\xc3':
        return 'tpl_copy_u8_rdx8_to_r8_8'

    # 2. Immediate return leaves
    if size == 6 and b[0] == 0xb8 and b[5] == 0xc3:
        imm32 = struct.unpack_from('<I', b, 1)[0]
        return f'tpl_ret_u32::<0x{imm32:x}>'
    if size == 3 and b[0] == 0xb0 and b[2] == 0xc3:
        imm8 = b[1]
        return f'tpl_ret_u8::<0x{imm8:x}>'
    if size == 4 and b == b'\x83\xc8\xff\xc3':
        return 'tpl_ret_u32::<0xffffffff>'
    if size == 5 and b == b'\x48\x83\xc8\xff\xc3':
        return 'tpl_ret_u64::<0xffffffffffffffff>'
    if size == 8 and b[:3] == b'\x48\xc7\xc0' and b[7] == 0xc3:
        imm32_s = struct.unpack_from('<i', b, 3)[0]
        imm64 = imm32_s & 0xFFFFFFFFFFFFFFFF
        return f'tpl_ret_u64::<0x{imm64:x}>'

    # 3. Struct field offset getters/setters & pointer-arithmetic leaves ([rcx + disp8/disp32])
    # mov rax, [rcx + OFF]; ret
    if size == 5 and b[:3] == b'\x48\x8b\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u64::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x48\x8b\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u64::<0x{disp:x}>'

    # mov eax, [rcx + OFF]; ret
    if size == 4 and b[:2] == b'\x8b\x41' and b[3] == 0xc3:
        disp = struct.unpack_from('<b', b, 2)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u32::<0x{disp:x}>'
    if size == 7 and b[:2] == b'\x8b\x81' and b[6] == 0xc3:
        disp = struct.unpack_from('<i', b, 2)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u32::<0x{disp:x}>'

    # movzx eax, byte ptr [rcx + OFF]; ret
    if size == 5 and b[:3] == b'\x0f\xb6\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u8::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x0f\xb6\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u8::<0x{disp:x}>'

    # movzx eax, word ptr [rcx + OFF]; ret
    if size == 5 and b[:3] == b'\x0f\xb7\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u16::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x0f\xb7\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_u16::<0x{disp:x}>'

    # movsx eax, byte ptr [rcx + OFF]; ret
    if size == 5 and b[:3] == b'\x0f\xbe\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_i8::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x0f\xbe\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_i8::<0x{disp:x}>'

    # movsx eax, word ptr [rcx + OFF]; ret
    if size == 5 and b[:3] == b'\x0f\xbf\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_i16::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x0f\xbf\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_i16::<0x{disp:x}>'

    # movsxd rax, dword ptr [rcx + OFF]; ret
    if size == 5 and b[:3] == b'\x48\x63\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_i32_sext::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x48\x63\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_get_rcx_i32_sext::<0x{disp:x}>'

    # lea rax, [rcx + OFF]; ret
    if size == 5 and b[:3] == b'\x48\x8d\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_lea_rcx::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x48\x8d\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_lea_rcx::<0x{disp:x}>'

    # mov [rcx + OFF], edx; ret
    if size == 4 and b[:2] == b'\x89\x51' and b[3] == 0xc3:
        disp = struct.unpack_from('<b', b, 2)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u32::<0x{disp:x}>'
    if size == 7 and b[:2] == b'\x89\x91' and b[6] == 0xc3:
        disp = struct.unpack_from('<i', b, 2)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u32::<0x{disp:x}>'

    # mov [rcx + OFF], rdx; ret
    if size == 5 and b[:3] == b'\x48\x89\x51' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u64::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x48\x89\x91' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u64::<0x{disp:x}>'

    # mov [rcx + OFF], dl; ret
    if size == 4 and b[:2] == b'\x88\x51' and b[3] == 0xc3:
        disp = struct.unpack_from('<b', b, 2)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u8::<0x{disp:x}>'
    if size == 7 and b[:2] == b'\x88\x91' and b[6] == 0xc3:
        disp = struct.unpack_from('<i', b, 2)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u8::<0x{disp:x}>'

    # mov [rcx + OFF], dx; ret
    if size == 5 and b[:3] == b'\x66\x89\x51' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u16::<0x{disp:x}>'
    if size == 8 and b[:3] == b'\x66\x89\x91' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        if disp >= 0:
            return f'tpl_set_rcx_u16::<0x{disp:x}>'

    # mov byte ptr [rcx + OFF], IMM8; ret
    if size == 5 and b[:2] == b'\xc6\x41' and b[4] == 0xc3:
        disp = struct.unpack_from('<b', b, 2)[0]
        imm8 = b[3]
        if disp >= 0:
            return f'tpl_set_rcx_imm8::<0x{disp:x}, 0x{imm8:x}>'
    if size == 8 and b[:2] == b'\xc6\x81' and b[7] == 0xc3:
        disp = struct.unpack_from('<i', b, 2)[0]
        imm8 = b[6]
        if disp >= 0:
            return f'tpl_set_rcx_imm8::<0x{disp:x}, 0x{imm8:x}>'

    # mov dword ptr [rcx + OFF], IMM32; ret
    if size == 8 and b[:2] == b'\xc7\x41' and b[7] == 0xc3:
        disp = struct.unpack_from('<b', b, 2)[0]
        imm32 = struct.unpack_from('<I', b, 3)[0]
        if disp >= 0:
            return f'tpl_set_rcx_imm32::<0x{disp:x}, 0x{imm32:x}>'
    if size == 11 and b[:2] == b'\xc7\x81' and b[10] == 0xc3:
        disp = struct.unpack_from('<i', b, 2)[0]
        imm32 = struct.unpack_from('<I', b, 6)[0]
        if disp >= 0:
            return f'tpl_set_rcx_imm32::<0x{disp:x}, 0x{imm32:x}>'

    # mov qword ptr [rcx + OFF], IMM32_SEXT; ret
    if size == 9 and b[:3] == b'\x48\xc7\x41' and b[8] == 0xc3:
        disp = struct.unpack_from('<b', b, 3)[0]
        imm64 = struct.unpack_from('<i', b, 4)[0] & 0xFFFFFFFFFFFFFFFF
        if disp >= 0:
            return f'tpl_set_rcx_imm64::<0x{disp:x}, 0x{imm64:x}>'
    if size == 12 and b[:3] == b'\x48\xc7\x81' and b[11] == 0xc3:
        disp = struct.unpack_from('<i', b, 3)[0]
        imm64 = struct.unpack_from('<i', b, 7)[0] & 0xFFFFFFFFFFFFFFFF
        if disp >= 0:
            return f'tpl_set_rcx_imm64::<0x{disp:x}, 0x{imm64:x}>'

    # Two-hop pointer getters
    if size == 9 and b[:3] == b'\x48\x8b\x41' and b[4:7] == b'\x48\x8b\x40' and b[8] == 0xc3:
        d1 = struct.unpack_from('<b', b, 3)[0]
        d2 = struct.unpack_from('<b', b, 7)[0]
        if d1 >= 0 and d2 >= 0:
            return f'tpl_two_hop_rcx_u64::<0x{d1:x}, 0x{d2:x}>'
    if size == 8 and b[:3] == b'\x48\x8b\x41' and b[4:6] == b'\x8b\x40' and b[7] == 0xc3:
        d1 = struct.unpack_from('<b', b, 3)[0]
        d2 = struct.unpack_from('<b', b, 6)[0]
        if d1 >= 0 and d2 >= 0:
            return f'tpl_two_hop_rcx_u32::<0x{d1:x}, 0x{d2:x}>'
    if size == 9 and b[:3] == b'\x48\x8b\x41' and b[4:7] == b'\x0f\xb6\x40' and b[8] == 0xc3:
        d1 = struct.unpack_from('<b', b, 3)[0]
        d2 = struct.unpack_from('<b', b, 7)[0]
        if d1 >= 0 and d2 >= 0:
            return f'tpl_two_hop_rcx_u8::<0x{d1:x}, 0x{d2:x}>'
    if size == 8 and b[:6] == b'\x48\x8b\x01\x48\x8b\x40' and b[7] == 0xc3:
        d2 = struct.unpack_from('<b', b, 6)[0]
        if d2 >= 0:
            return f'tpl_two_hop_rcx_u64::<0x0, 0x{d2:x}>'
    if size == 7 and b[:5] == b'\x48\x8b\x01\x8b\x40' and b[6] == 0xc3:
        d2 = struct.unpack_from('<b', b, 5)[0]
        if d2 >= 0:
            return f'tpl_two_hop_rcx_u32::<0x0, 0x{d2:x}>'

    # Two-hop pointer setter: mov rax, [rcx]; mov [rax + OFF], rdx; ret
    if size == 8 and b[:6] == b'\x48\x8b\x01\x48\x89\x50' and b[7] == 0xc3:
        disp = struct.unpack_from('<b', b, 6)[0]
        if disp >= 0:
            return f'tpl_deref_rcx0_set_u64::<0x{disp:x}>'
    if size == 11 and b[:6] == b'\x48\x8b\x01\x48\x89\x90' and b[10] == 0xc3:
        disp = struct.unpack_from('<i', b, 6)[0]
        if disp >= 0:
            return f'tpl_deref_rcx0_set_u64::<0x{disp:x}>'

    # Field equality / zero comparisons
    if size == 8 and b[:2] == b'\x80\x79' and b[3] == 0:
        disp = struct.unpack_from('<b', b, 2)[0]
        if disp >= 0:
            if b[4:8] == b'\x0f\x94\xc0\xc3':
                return f'tpl_cmp_rcx_u8_eq0::<0x{disp:x}>'
            if b[4:8] == b'\x0f\x95\xc0\xc3':
                return f'tpl_cmp_rcx_u8_ne0::<0x{disp:x}>'
    if size == 8 and b[:2] == b'\x83\x79' and b[3] == 0:
        disp = struct.unpack_from('<b', b, 2)[0]
        if disp >= 0:
            if b[4:8] == b'\x0f\x94\xc0\xc3':
                return f'tpl_cmp_rcx_u32_eq0::<0x{disp:x}>'
            if b[4:8] == b'\x0f\x95\xc0\xc3':
                return f'tpl_cmp_rcx_u32_ne0::<0x{disp:x}>'
    if size == 9 and b[:3] == b'\x48\x83\x79' and b[4] == 0:
        disp = struct.unpack_from('<b', b, 3)[0]
        if disp >= 0:
            if b[5:9] == b'\x0f\x94\xc0\xc3':
                return f'tpl_cmp_rcx_u64_eq0::<0x{disp:x}>'
            if b[5:9] == b'\x0f\x95\xc0\xc3':
                return f'tpl_cmp_rcx_u64_ne0::<0x{disp:x}>'

    # Bitfield extractors: mov eax, [rcx + disp8]; shr eax, sh; and eax/al, mask; ret
    if size == 10 and b[:2] == b'\x8b\x41' and b[3:5] == b'\xc1\xe8' and b[6:8] == b'\x83\xe0' and b[9] == 0xc3:
        disp = struct.unpack_from('<b', b, 2)[0]
        sh = b[5]
        mask = struct.unpack_from('<b', b, 8)[0] & 0xFFFFFFFF
        if disp >= 0 and sh < 32:
            return f'tpl_bitfield_rcx_u32::<0x{disp:x}, {sh}, 0x{mask:x}>'
    if size == 9 and b[:2] == b'\x8b\x41' and b[3:5] == b'\xc1\xe8' and b[6] == 0x24 and b[8] == 0xc3:
        disp = struct.unpack_from('<b', b, 2)[0]
        sh = b[5]
        mask = b[7]
        if disp >= 0 and sh < 32:
            return f'tpl_bitfield_rcx_u32::<0x{disp:x}, {sh}, 0x{mask:x}>'

    # 4. Vtable Slot Dispatchers (`jmp [rax + OFF]`)
    # 4a. ClassTypeInfoImplementation shift3:
    # mov rax, [rdx]; mov r10, r8; mov rcx, rdx; movzx r8d, r9b; mov rdx, r10; jmp [rax + OFF]
    p4_shift3 = b'\x48\x8b\x02\x4d\x8b\xd0\x48\x8b\xca\x45\x0f\xb6\xc1\x49\x8b\xd2'
    if b.startswith(p4_shift3):
        voff = parse_vjmp_rax_tail(b, len(p4_shift3))
        if voff is not None:
            return f'tpl_typeinfo_vcall_shift3::<0x{voff:x}>'

    # 4b. mov rax, [rdx]; mov rcx, rdx; xor edx, edx; jmp [rax + OFF]
    p4_rdx_xor = b'\x48\x8b\x02\x48\x8b\xca\x33\xd2'
    if b.startswith(p4_rdx_xor):
        voff = parse_vjmp_rax_tail(b, len(p4_rdx_xor))
        if voff is not None:
            return f'tpl_vcall_rdx_xor_edx::<0x{voff:x}>'

    # 4c. mov rax, [rdx]; mov rcx, rdx; jmp [rax + OFF]
    p4_rdx = b'\x48\x8b\x02\x48\x8b\xca'
    if b.startswith(p4_rdx):
        voff = parse_vjmp_rax_tail(b, len(p4_rdx))
        if voff is not None:
            return f'tpl_vcall_rdx::<0x{voff:x}>'

    # 4d. mov rax, [rdx]; mov rdx, r8; mov rcx, rdx; jmp [rax + OFF]
    p4_rdx_r8 = b'\x48\x8b\x02\x49\x8b\xd0\x48\x8b\xca'
    if b.startswith(p4_rdx_r8):
        voff = parse_vjmp_rax_tail(b, len(p4_rdx_r8))
        if voff is not None:
            return f'tpl_vcall_rdx_r8::<0x{voff:x}>'

    # 4e. mov rax, [rcx]; jmp [rax + OFF]
    if b.startswith(b'\x48\x8b\x01'):
        voff = parse_vjmp_rax_tail(b, 3)
        if voff is not None:
            return f'tpl_vcall_rcx::<0x{voff:x}>'

    # 4f. mov rcx, [rcx + DISP]; mov rax, [rcx]; jmp [rax + OFF]
    if b.startswith(b'\x48\x8b\x09\x48\x8b\x01'):
        voff = parse_vjmp_rax_tail(b, 6)
        if voff is not None:
            return f'tpl_vcall_rcx_field::<0x0, 0x{voff:x}>'
    if size >= 10 and b[:3] == b'\x48\x8b\x49' and b[4:7] == b'\x48\x8b\x01':
        disp = struct.unpack_from('<b', b, 3)[0]
        voff = parse_vjmp_rax_tail(b, 7)
        if disp >= 0 and voff is not None:
            return f'tpl_vcall_rcx_field::<0x{disp:x}, 0x{voff:x}>'
    if size >= 13 and b[:3] == b'\x48\x8b\x89' and b[7:10] == b'\x48\x8b\x01':
        disp = struct.unpack_from('<i', b, 3)[0]
        voff = parse_vjmp_rax_tail(b, 10)
        if disp >= 0 and voff is not None:
            return f'tpl_vcall_rcx_field::<0x{disp:x}, 0x{voff:x}>'

    # 4g. mov rcx, [rcx + disp8]; mov r8d, imm32; mov rax, [rcx]; jmp [rax + OFF]
    if size >= 16 and b[:3] == b'\x48\x8b\x49' and b[4:6] == b'\x41\xb8' and b[10:13] == b'\x48\x8b\x01':
        disp = struct.unpack_from('<b', b, 3)[0]
        imm32 = struct.unpack_from('<I', b, 6)[0]
        voff = parse_vjmp_rax_tail(b, 13)
        if disp >= 0 and voff is not None:
            return f'tpl_vcall_rcx_field_r8_imm::<0x{disp:x}, 0x{imm32:x}, 0x{voff:x}>'

    return None


def main():
    exe_path = os.environ.get('AW2_EXE') or (
        os.path.join(os.environ['AW2_GAME_DIR'], 'AlanWake2.exe')
        if os.environ.get('AW2_GAME_DIR')
        else ''
    )
    if not exe_path or not os.path.isfile(exe_path):
        raise SystemExit('set AW2_EXE or AW2_GAME_DIR to your local Alan Wake 2 install')
    db_path = os.path.join(ROOT, 'db', 'functions.sqlite')
    out_path = os.path.join(ROOT, 'crates', 'aw2-game', 'src', 'shards', 'templates_a.rs')

    pe = fndb.PE(exe_path)
    db = sqlite3.connect(db_path)

    excl_vas = load_excluded_vas()
    can_throw_map = dict(db.execute('SELECT va, mask_direct FROM can_throw').fetchall())

    rows = db.execute(
        "SELECT va, size, name, n_frags FROM functions WHERE owner = 'remedy' ORDER BY va"
    ).fetchall()

    entries = []
    counts = {}

    for va, size, name, n_frags in rows:
        if va in excl_vas or n_frags > 0:
            continue
        rva = va - pe.image_base
        off = rva - pe.text_rva
        if off < 0 or off + max(size, 5) > len(pe.text):
            continue
        b = pe.text[off:off + size]
        if size < 5 and pe.text[off + size:off + 5] != b'\xcc' * (5 - size):
            continue

        ptr_fn = match_template(b)
        if ptr_fn is None:
            continue

        if fast_eh_info(pe, va) != 'none':
            continue

        base_fn = ptr_fn.split('::')[0]
        counts[base_fn] = counts.get(base_fn, 0) + 1

        fn_name = name if name else f'sub_{va:x}'
        fn_name_escaped = fn_name.replace('\\', '').replace('"', '')
        mask_direct = can_throw_map.get(va, 0)
        throws_discard = (mask_direct & 23) != 0
        entries.append((va, fn_name_escaped, ptr_fn, throws_discard))

    use_throws = any(td for _, _, _, td in entries)
    import_line = (
        'use crate::registry::{Replacement, Throws};'
        if use_throws
        else 'use crate::registry::Replacement;'
    )

    lines = [
        '//! Shard A: Exact & immediate/vtable-offset parameterized templates.',
        '//! Generated by `tools/decomp_scale/gen_shard_a.py`.',
        '',
        import_line,
        '',
        '// ---------------------------------------------------------------------------',
        '// 1. Zero-operand & exact pointer/struct/branch leaves',
        '// ---------------------------------------------------------------------------',
        '',
        '/// `ret` / `ret 0`',
        'pub unsafe extern "win64" fn tpl_ret_void() {}',
        '',
        '/// `mov al, 1; ret`',
        'pub unsafe extern "win64" fn tpl_ret_u8_1() -> u8 {',
        '    1',
        '}',
        '',
        '/// `xor al/eax, al/eax; ret`',
        'pub unsafe extern "win64" fn tpl_ret_zero() -> u64 {',
        '    0',
        '}',
        '',
        '/// `mov rax, rdx; ret`',
        'pub unsafe extern "win64" fn tpl_ret_rdx(_rcx: u64, rdx: u64) -> u64 {',
        '    rdx',
        '}',
        '',
        '/// `mov rax, rcx; ret`',
        'pub unsafe extern "win64" fn tpl_ret_rcx(rcx: u64) -> u64 {',
        '    rcx',
        '}',
        '',
        '/// `mov rax, [rcx]; ret`',
        'pub unsafe extern "win64" fn tpl_deref_rcx_u64(rcx: usize) -> u64 {',
        '    unsafe { crate::exe::rd::<u64>(rcx) }',
        '}',
        '',
        '/// `mov rax, rcx; test rcx, rcx; jz +4; lea rax, [rcx + rdx]; ret`',
        'pub unsafe extern "win64" fn tpl_if_rcx_add_rdx(rcx: u64, rdx: u64) -> u64 {',
        '    if rcx == 0 { 0 } else { rcx.wrapping_add(rdx) }',
        '}',
        '',
        '/// `mov [rcx], rdx; mov rax, rcx; ret`',
        'pub unsafe extern "win64" fn tpl_store_rcx0_rdx_ret_rcx(rcx: usize, rdx: u64) -> usize {',
        '    unsafe { crate::exe::wr::<u64>(rcx, rdx) };',
        '    rcx',
        '}',
        '',
        '/// `mov rax, [rdx]; mov [rcx], rax; mov rax, rcx; ret`',
        'pub unsafe extern "win64" fn tpl_copy_u64_rdx_to_rcx_ret_rcx(rcx: usize, rdx: usize) -> usize {',
        '    let v = unsafe { crate::exe::rd::<u64>(rdx) };',
        '    unsafe { crate::exe::wr::<u64>(rcx, v) };',
        '    rcx',
        '}',
        '',
        '/// `xor eax, eax; mov [rcx], rax; mov [rcx + 8], rax; mov rax, rcx; ret`',
        'pub unsafe extern "win64" fn tpl_zero_rcx_16_ret_rcx(rcx: usize) -> usize {',
        '    unsafe {',
        '        crate::exe::wr::<u64>(rcx, 0);',
        '        crate::exe::wr::<u64>(rcx.wrapping_add(8), 0);',
        '    }',
        '    rcx',
        '}',
        '',
        '/// `mov rax, [rdx]; cmp [rcx], rax; setne al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_ne_deref_rcx_rdx_u64(rcx: usize, rdx: usize) -> u8 {',
        '    let a = unsafe { crate::exe::rd::<u64>(rcx) };',
        '    let b = unsafe { crate::exe::rd::<u64>(rdx) };',
        '    (a != b) as u8',
        '}',
        '',
        '/// `mov rax, [rdx]; cmp [rcx], rax; sete al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_eq_deref_rcx_rdx_u64(rcx: usize, rdx: usize) -> u8 {',
        '    let a = unsafe { crate::exe::rd::<u64>(rcx) };',
        '    let b = unsafe { crate::exe::rd::<u64>(rdx) };',
        '    (a == b) as u8',
        '}',
        '',
        '/// `mov rax, [rdx]; mov [r8], rax; ret`',
        'pub unsafe extern "win64" fn tpl_copy_u64_rdx_to_r8(_rcx: usize, rdx: usize, r8: usize) -> u64 {',
        '    let v = unsafe { crate::exe::rd::<u64>(rdx) };',
        '    unsafe { crate::exe::wr::<u64>(r8, v) };',
        '    v',
        '}',
        '',
        '/// `mov qword ptr [rdx], 0; mov rax, rdx; ret`',
        'pub unsafe extern "win64" fn tpl_zero_rdx0_ret_rdx(_rcx: usize, rdx: usize) -> usize {',
        '    unsafe { crate::exe::wr::<u64>(rdx, 0) };',
        '    rdx',
        '}',
        '',
        '/// `mov rax, [rcx]; mov qword ptr [rcx], 0; ret`',
        'pub unsafe extern "win64" fn tpl_take_rcx0_u64(rcx: usize) -> u64 {',
        '    let v = unsafe { crate::exe::rd::<u64>(rcx) };',
        '    unsafe { crate::exe::wr::<u64>(rcx, 0) };',
        '    v',
        '}',
        '',
        '/// `mov rax, [rcx + 8]; mov [rcx], rax; ret`',
        'pub unsafe extern "win64" fn tpl_copy_rcx8_to_rcx0(rcx: usize) -> u64 {',
        '    let v = unsafe { crate::exe::rd::<u64>(rcx.wrapping_add(8)) };',
        '    unsafe { crate::exe::wr::<u64>(rcx, v) };',
        '    v',
        '}',
        '',
        '/// `mov [rcx], rdx; mov rax, rcx; mov [rcx + 8], rdx; ret`',
        'pub unsafe extern "win64" fn tpl_store_rcx0_rcx8_rdx_ret_rcx(rcx: usize, rdx: u64) -> usize {',
        '    unsafe {',
        '        crate::exe::wr::<u64>(rcx, rdx);',
        '        crate::exe::wr::<u64>(rcx.wrapping_add(8), rdx);',
        '    }',
        '    rcx',
        '}',
        '',
        '/// `mov [rcx], rdx; mov rax, rcx; mov [rcx + 8], r8; ret`',
        'pub unsafe extern "win64" fn tpl_store_rcx0_rdx_rcx8_r8_ret_rcx(rcx: usize, rdx: u64, r8: u64) -> usize {',
        '    unsafe {',
        '        crate::exe::wr::<u64>(rcx, rdx);',
        '        crate::exe::wr::<u64>(rcx.wrapping_add(8), r8);',
        '    }',
        '    rcx',
        '}',
        '',
        '/// `mov rax, [rcx]; mov [rdx], rax; mov rax, rdx; ret`',
        'pub unsafe extern "win64" fn tpl_copy_u64_rcx_to_rdx_ret_rdx(rcx: usize, rdx: usize) -> usize {',
        '    let v = unsafe { crate::exe::rd::<u64>(rcx) };',
        '    unsafe { crate::exe::wr::<u64>(rdx, v) };',
        '    rdx',
        '}',
        '',
        '/// `mov rax, [rcx]; mov r8d, r8d; mov [rdx + r8*8], rax; ret`',
        'pub unsafe extern "win64" fn tpl_store_idx8_rdx_r8_from_rcx0(rcx: usize, rdx: usize, r8: u32) -> u64 {',
        '    let v = unsafe { crate::exe::rd::<u64>(rcx) };',
        '    unsafe { crate::exe::wr::<u64>(rdx.wrapping_add((r8 as usize) * 8), v) };',
        '    v',
        '}',
        '',
        '/// `xor eax, eax; mov [rcx], rax; mov [rcx + 8], rax; ret`',
        'pub unsafe extern "win64" fn tpl_zero_rcx_16(rcx: usize) -> u64 {',
        '    unsafe {',
        '        crate::exe::wr::<u64>(rcx, 0);',
        '        crate::exe::wr::<u64>(rcx.wrapping_add(8), 0);',
        '    }',
        '    0',
        '}',
        '',
        '/// `movzx eax, byte ptr [rdx + 8]; mov [r8 + 8], al; ret`',
        'pub unsafe extern "win64" fn tpl_copy_u8_rdx8_to_r8_8(_rcx: usize, rdx: usize, r8: usize) -> u32 {',
        '    let v = unsafe { crate::exe::rd::<u8>(rdx.wrapping_add(8)) };',
        '    unsafe { crate::exe::wr::<u8>(r8.wrapping_add(8), v) };',
        '    v as u32',
        '}',
        '',
        '// ---------------------------------------------------------------------------',
        '// 2. Immediate return leaves',
        '// ---------------------------------------------------------------------------',
        '',
        '/// `mov eax, imm32; ret` / `or eax, -1; ret`',
        'pub unsafe extern "win64" fn tpl_ret_u32<const V: u32>() -> u32 {',
        '    V',
        '}',
        '',
        '/// `mov al, imm8; ret`',
        'pub unsafe extern "win64" fn tpl_ret_u8<const V: u8>() -> u8 {',
        '    V',
        '}',
        '',
        '/// `mov rax, imm32_sext; ret` / `or rax, -1; ret`',
        'pub unsafe extern "win64" fn tpl_ret_u64<const V: u64>() -> u64 {',
        '    V',
        '}',
        '',
        '// ---------------------------------------------------------------------------',
        '// 3. Struct field offset getters/setters, two-hop & bitfield leaves',
        '// ---------------------------------------------------------------------------',
        '',
        '/// `mov rax, [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_get_rcx_u64<const OFF: usize>(rcx: usize) -> u64 {',
        '    unsafe { crate::exe::rd::<u64>(rcx.wrapping_add(OFF)) }',
        '}',
        '',
        '/// `mov eax, [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_get_rcx_u32<const OFF: usize>(rcx: usize) -> u32 {',
        '    unsafe { crate::exe::rd::<u32>(rcx.wrapping_add(OFF)) }',
        '}',
        '',
        '/// `movzx eax, byte ptr [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_get_rcx_u8<const OFF: usize>(rcx: usize) -> u32 {',
        '    unsafe { crate::exe::rd::<u8>(rcx.wrapping_add(OFF)) as u32 }',
        '}',
        '',
        '/// `movzx eax, word ptr [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_get_rcx_u16<const OFF: usize>(rcx: usize) -> u32 {',
        '    unsafe { crate::exe::rd::<u16>(rcx.wrapping_add(OFF)) as u32 }',
        '}',
        '',
        '/// `movsx eax, byte ptr [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_get_rcx_i8<const OFF: usize>(rcx: usize) -> u32 {',
        '    unsafe { (crate::exe::rd::<i8>(rcx.wrapping_add(OFF)) as i32) as u32 }',
        '}',
        '',
        '/// `movsx eax, word ptr [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_get_rcx_i16<const OFF: usize>(rcx: usize) -> u32 {',
        '    unsafe { (crate::exe::rd::<i16>(rcx.wrapping_add(OFF)) as i32) as u32 }',
        '}',
        '',
        '/// `movsxd rax, dword ptr [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_get_rcx_i32_sext<const OFF: usize>(rcx: usize) -> u64 {',
        '    unsafe { (crate::exe::rd::<i32>(rcx.wrapping_add(OFF)) as i64) as u64 }',
        '}',
        '',
        '/// `lea rax, [rcx + OFF]; ret`',
        'pub unsafe extern "win64" fn tpl_lea_rcx<const OFF: usize>(rcx: usize) -> usize {',
        '    rcx.wrapping_add(OFF)',
        '}',
        '',
        '/// `mov [rcx + OFF], edx; ret`',
        'pub unsafe extern "win64" fn tpl_set_rcx_u32<const OFF: usize>(rcx: usize, edx: u32) {',
        '    unsafe { crate::exe::wr::<u32>(rcx.wrapping_add(OFF), edx) }',
        '}',
        '',
        '/// `mov [rcx + OFF], rdx; ret`',
        'pub unsafe extern "win64" fn tpl_set_rcx_u64<const OFF: usize>(rcx: usize, rdx: u64) {',
        '    unsafe { crate::exe::wr::<u64>(rcx.wrapping_add(OFF), rdx) }',
        '}',
        '',
        '/// `mov [rcx + OFF], dl; ret`',
        'pub unsafe extern "win64" fn tpl_set_rcx_u8<const OFF: usize>(rcx: usize, dl: u8) {',
        '    unsafe { crate::exe::wr::<u8>(rcx.wrapping_add(OFF), dl) }',
        '}',
        '',
        '/// `mov [rcx + OFF], dx; ret`',
        'pub unsafe extern "win64" fn tpl_set_rcx_u16<const OFF: usize>(rcx: usize, dx: u16) {',
        '    unsafe { crate::exe::wr::<u16>(rcx.wrapping_add(OFF), dx) }',
        '}',
        '',
        '/// `mov byte ptr [rcx + OFF], IMM; ret`',
        'pub unsafe extern "win64" fn tpl_set_rcx_imm8<const OFF: usize, const IMM: u8>(rcx: usize) {',
        '    unsafe { crate::exe::wr::<u8>(rcx.wrapping_add(OFF), IMM) }',
        '}',
        '',
        '/// `mov dword ptr [rcx + OFF], IMM; ret`',
        'pub unsafe extern "win64" fn tpl_set_rcx_imm32<const OFF: usize, const IMM: u32>(rcx: usize) {',
        '    unsafe { crate::exe::wr::<u32>(rcx.wrapping_add(OFF), IMM) }',
        '}',
        '',
        '/// `mov qword ptr [rcx + OFF], IMM; ret`',
        'pub unsafe extern "win64" fn tpl_set_rcx_imm64<const OFF: usize, const IMM: u64>(rcx: usize) {',
        '    unsafe { crate::exe::wr::<u64>(rcx.wrapping_add(OFF), IMM) }',
        '}',
        '',
        '/// `mov rax, [rcx + D1]; mov rax, [rax + D2]; ret`',
        'pub unsafe extern "win64" fn tpl_two_hop_rcx_u64<const D1: usize, const D2: usize>(rcx: usize) -> u64 {',
        '    let p = unsafe { crate::exe::rd::<usize>(rcx.wrapping_add(D1)) };',
        '    unsafe { crate::exe::rd::<u64>(p.wrapping_add(D2)) }',
        '}',
        '',
        '/// `mov rax, [rcx + D1]; mov eax, [rax + D2]; ret`',
        'pub unsafe extern "win64" fn tpl_two_hop_rcx_u32<const D1: usize, const D2: usize>(rcx: usize) -> u32 {',
        '    let p = unsafe { crate::exe::rd::<usize>(rcx.wrapping_add(D1)) };',
        '    unsafe { crate::exe::rd::<u32>(p.wrapping_add(D2)) }',
        '}',
        '',
        '/// `mov rax, [rcx + D1]; movzx eax, byte ptr [rax + D2]; ret`',
        'pub unsafe extern "win64" fn tpl_two_hop_rcx_u8<const D1: usize, const D2: usize>(rcx: usize) -> u32 {',
        '    let p = unsafe { crate::exe::rd::<usize>(rcx.wrapping_add(D1)) };',
        '    unsafe { crate::exe::rd::<u8>(p.wrapping_add(D2)) as u32 }',
        '}',
        '',
        '/// `mov rax, [rcx]; mov [rax + OFF], rdx; ret`',
        'pub unsafe extern "win64" fn tpl_deref_rcx0_set_u64<const OFF: usize>(rcx: usize, rdx: u64) -> usize {',
        '    let p = unsafe { crate::exe::rd::<usize>(rcx) };',
        '    unsafe { crate::exe::wr::<u64>(p.wrapping_add(OFF), rdx) };',
        '    p',
        '}',
        '',
        '/// `cmp byte ptr [rcx + OFF], 0; sete al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_rcx_u8_eq0<const OFF: usize>(rcx: usize) -> u8 {',
        '    (unsafe { crate::exe::rd::<u8>(rcx.wrapping_add(OFF)) } == 0) as u8',
        '}',
        '',
        '/// `cmp byte ptr [rcx + OFF], 0; setne al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_rcx_u8_ne0<const OFF: usize>(rcx: usize) -> u8 {',
        '    (unsafe { crate::exe::rd::<u8>(rcx.wrapping_add(OFF)) } != 0) as u8',
        '}',
        '',
        '/// `cmp dword ptr [rcx + OFF], 0; sete al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_rcx_u32_eq0<const OFF: usize>(rcx: usize) -> u8 {',
        '    (unsafe { crate::exe::rd::<u32>(rcx.wrapping_add(OFF)) } == 0) as u8',
        '}',
        '',
        '/// `cmp dword ptr [rcx + OFF], 0; setne al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_rcx_u32_ne0<const OFF: usize>(rcx: usize) -> u8 {',
        '    (unsafe { crate::exe::rd::<u32>(rcx.wrapping_add(OFF)) } != 0) as u8',
        '}',
        '',
        '/// `cmp qword ptr [rcx + OFF], 0; sete al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_rcx_u64_eq0<const OFF: usize>(rcx: usize) -> u8 {',
        '    (unsafe { crate::exe::rd::<u64>(rcx.wrapping_add(OFF)) } == 0) as u8',
        '}',
        '',
        '/// `cmp qword ptr [rcx + OFF], 0; setne al; ret`',
        'pub unsafe extern "win64" fn tpl_cmp_rcx_u64_ne0<const OFF: usize>(rcx: usize) -> u8 {',
        '    (unsafe { crate::exe::rd::<u64>(rcx.wrapping_add(OFF)) } != 0) as u8',
        '}',
        '',
        '/// `mov eax, [rcx + OFF]; shr eax, SH; and eax, MASK; ret`',
        'pub unsafe extern "win64" fn tpl_bitfield_rcx_u32<const OFF: usize, const SH: u32, const MASK: u32>(',
        '    rcx: usize,',
        ') -> u32 {',
        '    (unsafe { crate::exe::rd::<u32>(rcx.wrapping_add(OFF)) } >> SH) & MASK',
        '}',
        '',
        '// ---------------------------------------------------------------------------',
        '// 4. Vtable Slot Dispatchers (`jmp [rax + VTABLE_OFF]`)',
        '// ---------------------------------------------------------------------------',
        '',
        '/// `mov rax, [rdx]; mov r10, r8; mov rcx, rdx; movzx r8d, r9b; mov rdx, r10; jmp qword ptr [rax + VTABLE_OFF]`',
        'pub unsafe extern "win64" fn tpl_typeinfo_vcall_shift3<const VTABLE_OFF: usize>(',
        '    _this: usize,',
        '    obj: usize,',
        '    a2: u64,',
        '    a3: u8,',
        ') -> u64 {',
        '    let vtable = unsafe { crate::exe::rd::<usize>(obj) };',
        '    let fn_ptr = unsafe { crate::exe::rd::<usize>(vtable.wrapping_add(VTABLE_OFF)) };',
        '    let f: unsafe extern "win64" fn(usize, u64, u64) -> u64 = unsafe { crate::exe::func(fn_ptr as u64) };',
        '    unsafe { f(obj, a2, a3 as u64) }',
        '}',
        '',
        '/// `mov rax, [rdx]; mov rcx, rdx; xor edx, edx; jmp qword ptr [rax + VTABLE_OFF]`',
        'pub unsafe extern "win64" fn tpl_vcall_rdx_xor_edx<const VTABLE_OFF: usize>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let vtable = unsafe { crate::exe::rd::<usize>(rdx) };',
        '    let fn_ptr = unsafe { crate::exe::rd::<usize>(vtable.wrapping_add(VTABLE_OFF)) };',
        '    let f: unsafe extern "win64" fn(usize, u64, u64, u64) -> u64 = unsafe { crate::exe::func(fn_ptr as u64) };',
        '    unsafe { f(rdx, 0, r8, r9) }',
        '}',
        '',
        '/// `mov rax, [rdx]; mov rcx, rdx; jmp qword ptr [rax + VTABLE_OFF]`',
        'pub unsafe extern "win64" fn tpl_vcall_rdx<const VTABLE_OFF: usize>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let vtable = unsafe { crate::exe::rd::<usize>(rdx) };',
        '    let fn_ptr = unsafe { crate::exe::rd::<usize>(vtable.wrapping_add(VTABLE_OFF)) };',
        '    let f: unsafe extern "win64" fn(usize, usize, u64, u64) -> u64 = unsafe { crate::exe::func(fn_ptr as u64) };',
        '    unsafe { f(rdx, rdx, r8, r9) }',
        '}',
        '',
        '/// `mov rax, [rdx]; mov rdx, r8; mov rcx, rdx; jmp qword ptr [rax + VTABLE_OFF]`',
        'pub unsafe extern "win64" fn tpl_vcall_rdx_r8<const VTABLE_OFF: usize>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let vtable = unsafe { crate::exe::rd::<usize>(rdx) };',
        '    let fn_ptr = unsafe { crate::exe::rd::<usize>(vtable.wrapping_add(VTABLE_OFF)) };',
        '    let f: unsafe extern "win64" fn( u64, u64, u64, u64) -> u64 = unsafe { crate::exe::func(fn_ptr as u64) };',
        '    unsafe { f(r8, r8, r8, r9) }',
        '}',
        '',
        '/// `mov rax, [rcx]; jmp qword ptr [rax + VTABLE_OFF]`',
        'pub unsafe extern "win64" fn tpl_vcall_rcx<const VTABLE_OFF: usize>(',
        '    rcx: usize,',
        '    rdx: u64,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let vtable = unsafe { crate::exe::rd::<usize>(rcx) };',
        '    let fn_ptr = unsafe { crate::exe::rd::<usize>(vtable.wrapping_add(VTABLE_OFF)) };',
        '    let f: unsafe extern "win64" fn(usize, u64, u64, u64) -> u64 = unsafe { crate::exe::func(fn_ptr as u64) };',
        '    unsafe { f(rcx, rdx, r8, r9) }',
        '}',
        '',
        '/// `mov rcx, [rcx + DISP]; mov rax, [rcx]; jmp qword ptr [rax + VTABLE_OFF]`',
        'pub unsafe extern "win64" fn tpl_vcall_rcx_field<const DISP: usize, const VTABLE_OFF: usize>(',
        '    rcx: usize,',
        '    rdx: u64,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let sub = unsafe { crate::exe::rd::<usize>(rcx.wrapping_add(DISP)) };',
        '    let vtable = unsafe { crate::exe::rd::<usize>(sub) };',
        '    let fn_ptr = unsafe { crate::exe::rd::<usize>(vtable.wrapping_add(VTABLE_OFF)) };',
        '    let f: unsafe extern "win64" fn(usize, u64, u64, u64) -> u64 = unsafe { crate::exe::func(fn_ptr as u64) };',
        '    unsafe { f(sub, rdx, r8, r9) }',
        '}',
        '',
        '/// `mov rcx, [rcx + DISP]; mov r8d, IMM; mov rax, [rcx]; jmp qword ptr [rax + VTABLE_OFF]`',
        'pub unsafe extern "win64" fn tpl_vcall_rcx_field_r8_imm<',
        '    const DISP: usize,',
        '    const IMM: u32,',
        '    const VTABLE_OFF: usize,',
        '>(',
        '    rcx: usize,',
        '    rdx: u64,',
        '    _r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let sub = unsafe { crate::exe::rd::<usize>(rcx.wrapping_add(DISP)) };',
        '    let vtable = unsafe { crate::exe::rd::<usize>(sub) };',
        '    let fn_ptr = unsafe { crate::exe::rd::<usize>(vtable.wrapping_add(VTABLE_OFF)) };',
        '    let f: unsafe extern "win64" fn(usize, u64, u64, u64) -> u64 = unsafe { crate::exe::func(fn_ptr as u64) };',
        '    unsafe { f(sub, rdx, IMM as u64, r9) }',
        '}',
        '',
        'pub static ENTRIES: &[Replacement] = &[',
    ]

    for va, fn_name, ptr_fn, throws_discard in entries:
        ptr_expr = f'{ptr_fn} as *const ()'
        if throws_discard:
            lines.append(f'    Replacement::draft_throws(0x{va:x}, "{fn_name}", {ptr_expr}, Throws::Discard),')
        else:
            lines.append(f'    Replacement::draft(0x{va:x}, "{fn_name}", {ptr_expr}),')

    lines.append('];')
    lines.append('')

    with open(out_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))

    print(f'Generated {len(entries)} entries in {out_path}:')
    for k, v in sorted(counts.items()):
        print(f'  {k:34s}: {v}')


if __name__ == '__main__':
    main()
