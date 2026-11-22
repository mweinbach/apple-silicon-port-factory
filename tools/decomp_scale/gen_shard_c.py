#!/usr/bin/env python3
"""Generate crates/aw2-game/src/shards/templates_c.rs for Shard C."""

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
    h = pe.image_base + struct.unpack_from('<I', d, tail)[0]
    data = tail + 4
    if h in (canthrow.H_FH4, canthrow.H_GS_FH4):
        hdr = d[pe.off(struct.unpack_from('<I', d, data)[0])]
        if hdr & 0x10:
            return 'catch'
        if hdr & 0x40:
            return 'noexcept'
        return 'cleanup' if hdr & 0x08 else 'none'
    if h in (canthrow.H_FH3, canthrow.H_GS_FH3):
        o = pe.off(struct.unpack_from('<I', d, data)[0])
        ntry = struct.unpack_from('<I', d, o + 12)[0]
        maxstate = struct.unpack_from('<i', d, o + 4)[0]
        return 'catch' if ntry else ('cleanup' if maxstate > 0 else 'none')
    if h == canthrow.H_GS:
        return 'none'
    if h == canthrow.H_SEH:
        n = struct.unpack_from('<I', d, data)[0]
        targets = [struct.unpack_from('<I', d, data + 4 + 16 * k + 12)[0] for k in range(min(n, 256))]
        return 'catch' if any(targets) else ('cleanup' if n else 'none')
    return 'unknown:0x%x' % h


def load_excluded_vas():
    vas = set()
    src_root = os.path.join(ROOT, 'crates', 'aw2-game', 'src')
    ctor_re = re.compile(r'Replacement::(?:draft(?:_diff|_throws|_throws_diff)?|verified(?:_throws)?)\s*\(\s*(0x[0-9a-fA-F_]+)')
    va_re = re.compile(r'\bva\s*:\s*(0x[0-9a-fA-F_]+)')
    for sub in ('registry', 'shards'):
        base_d = os.path.join(src_root, sub)
        for dirpath, _, files in os.walk(base_d):
            if os.path.basename(dirpath) == 'templates_c':
                continue
            for fn in sorted(files):
                if not fn.endswith('.rs') or fn in ('templates_c.rs', 'mod.rs'):
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


_COMP_DESC_TAIL = bytes.fromhex(
    '488d0c924803c9498b00c5fc10442410c5fc1104c8c5fc104c2430c5fc114cc820c5f810442450c5f81144c840'
    '488d4201488bc848c1e9214833c848b8cd8c55edd7af51ff480fafc8488bc148c1e8214833c148b953ec851afeb9cec4'
    '480fafc1488bc848c1e9214833c8498b401048890cd0c5f8774883c468c3'
)

_EP1_A = bytes.fromhex('8b5708488b07488b5c2450488d0c524803c98b4cc8f88d42ff89470833c0898f280300004883c4405fc3')
_EP1_B = bytes.fromhex('448b4708488b07488b5c24504b8d0c404803c98b4cc8f8418d40ff89470833c0898f280300004883c4405fc3')
_EP1_C = bytes.fromhex('8b5308488b03488d0c524803c98b4cc8f88d42ff89430833c0898b28030000488b5c24504883c4405fc3')
_EP1_D = bytes.fromhex('8b5708488b07488d0c524803c98b4cc8f88d42ff89470833c0898f28030000488b5c24504883c4405fc3')
_EP2_A = bytes.fromhex(
    '8b5708488b07488b5c2450488d0c524803c9ffca8b4cc8f8898f28030000'
    '488d0c524803c98957088b4cc8f88d42ff89470833c0898f280300004883c4405fc3'
)
_EP0_A = bytes.fromhex('488b5c245033c04883c4405fc3')
_BEGIN_FN_MAP = {
    0x1401E9770: 1,
    0x1401E9970: 2,
    0x1401E9B60: 3,
    0x1401E9FA0: 4,
}


def _parse_comp_desc(va, size, b):
    if size < 150 or b[:13] != bytes.fromhex('4c8bdc4883ec684c8bc10fb715'):
        return None
    if b[-123:] != _COMP_DESC_TAIL:
        return None
    u16_va = va + 17 + struct.unpack_from('<i', b, 13)[0]
    p = 17
    end = size - 123
    name_va = 0
    name_len = 0
    sz_val = 0
    align_val = 0
    flag_u32 = 0
    fns = {0xD0: 0, 0xD8: 0, 0xE0: 0, 0xE8: 0, 0xF0: 0}
    while p < end:
        if b[p:p + 2] == b'\x33\xc9':
            p += 2
        elif b[p:p + 4] == b'\x89\x4c\x24\x34':
            flag_u32 = 0
            p += 4
        elif b[p:p + 4] == b'\xc7\x44\x24\x34':
            flag_u32 = struct.unpack_from('<I', b, p + 4)[0]
            p += 8
        elif b[p:p + 3] == b'\x49\x89\x4b' and b[p + 3] in fns:
            fns[b[p + 3]] = 0
            p += 4
        elif b[p:p + 3] == b'\x48\x8d\x05' and b[p + 7:p + 11] == b'\x48\x89\x04\x24':
            name_va = va + p + 7 + struct.unpack_from('<i', b, p + 3)[0]
            p += 11
        elif b[p:p + 3] == b'\x48\x8d\x05' and b[p + 7:p + 10] == b'\x49\x89\x43' and b[p + 10] in fns:
            fns[b[p + 10]] = va + p + 7 + struct.unpack_from('<i', b, p + 3)[0]
            p += 11
        elif b[p:p + 3] == b'\x49\xc7\x43' and b[p + 3] in (0xA0, 0xB8, 0xC0):
            val = struct.unpack_from('<I', b, p + 4)[0]
            if b[p + 3] == 0xA0:
                name_len = val
            elif b[p + 3] == 0xB8:
                sz_val = val
            else:
                align_val = val
            p += 8
        elif b[p:p + 11] == bytes.fromhex('c5f8100424c5f811442410'):
            p += 11
        elif b[p:p + 5] == bytes.fromhex('6689542430'):
            p += 5
        else:
            return None
    if name_va == 0:
        return None
    return (
        'tpl_reg_comp_desc',
        (
            u16_va,
            name_va,
            name_len,
            sz_val,
            align_val,
            flag_u32,
            fns[0xD0],
            fns[0xD8],
            fns[0xE0],
            fns[0xE8],
            fns[0xF0],
        ),
    )


def _parse_desc_array_init(va, b):
    m = re.search(b'\x48\xc7\x03(....)\x48\x8d\x05(....)\x48\x89\x43\x08', b, re.DOTALL)
    if not m:
        return None
    n = struct.unpack('<I', m.group(1))[0]
    if not (1 <= n <= 6):
        return None
    arr_va = va + m.start() + 14 + struct.unpack('<i', m.group(2))[0]
    m_hdr = re.search(b'\x48\x8d\x0d(....)\xe8(....)\x83\x3d(....)\xff', b, re.DOTALL)
    if not m_hdr:
        return None
    i_hdr = m_hdr.start()
    guard_va = va + i_hdr + 7 + struct.unpack('<i', m_hdr.group(1))[0]
    hdr_fn = va + i_hdr + 12 + struct.unpack('<i', m_hdr.group(2))[0]
    guard_va2 = va + i_hdr + 19 + struct.unpack('<i', m_hdr.group(3))[0]
    if hdr_fn != 0x141834A78 or guard_va != guard_va2:
        return None
    m_ftr = re.search(b'\x48\x8d\x0d(....)(\xc5\xf8\x77)?\xe8(....)(?:\xeb.|\xe9....)$', b, re.DOTALL)
    if not m_ftr:
        return None
    i_ftr = m_ftr.start()
    guard_va3 = va + i_ftr + 7 + struct.unpack('<i', m_ftr.group(1))[0]
    vzero_len = 3 if m_ftr.group(2) else 0
    ftr_fn = va + i_ftr + 12 + vzero_len + struct.unpack('<i', m_ftr.group(3))[0]
    if ftr_fn != 0x141834A0C or guard_va3 != guard_va:
        return None
    p = m_hdr.end()
    if b[p] == 0x75:
        p += 2
    elif b[p:p + 2] == b'\x0f\x85':
        p += 6
    else:
        return None
    body = b[p:i_ftr]
    params = [guard_va, arr_va]
    pos = 0
    for _ in range(n):
        idx = body.find(b'\x0f\xb7\x05', pos)
        if idx < 0:
            return None
        u16_va = va + p + idx + 7 + struct.unpack_from('<i', body, idx + 3)[0]
        sub = body[idx + 7:]
        if sub[:5] == b'\x66\x89\x44\x24\x20':
            s2 = sub[5:]
        elif sub[:3] == b'\x66\x89\x45':
            s2 = sub[4:]
        else:
            return None
        if s2[:4] == b'\x89\x7c\x24\x24':
            f1 = 0
            s3 = s2[4:]
        elif s2[:2] == b'\x89\x7d':
            f1 = 0
            s3 = s2[3:]
        elif s2[:4] == b'\xc7\x44\x24\x24':
            f1 = struct.unpack_from('<I', s2, 4)[0]
            s3 = s2[8:]
        elif s2[:2] == b'\xc7\x45':
            f1 = struct.unpack_from('<I', s2, 3)[0]
            s3 = s2[7:]
        else:
            return None
        if s3[:4] in (b'\xc5\xfb\x10\x4c', b'\xc5\xfb\x10\x44'):
            s4 = s3[6:]
        elif s3[:4] in (b'\xc5\xfb\x10\x4d', b'\xc5\xfb\x10\x45'):
            s4 = s3[5:]
        else:
            return None
        if s4[:4] == b'\xc5\xfb\x11\x44':
            s4 = s4[6:]
        elif s4[:4] == b'\xc5\xfb\x11\x45':
            s4 = s4[5:]
        if s4[:3] == b'\x89\x7c\x24':
            f2 = 0
        elif s4[:2] == b'\x89\x7d':
            f2 = 0
        elif s4[:3] == b'\xc7\x44\x24':
            f2 = struct.unpack_from('<I', s4, 4)[0]
        elif s4[:2] == b'\xc7\x45':
            f2 = struct.unpack_from('<I', s4, 3)[0]
        else:
            return None
        params.extend((u16_va, f1, f2))
        pos = idx + 7
    return (f'tpl_init_desc_arr_{n}', tuple(params))


def _parse_hashmap_init(va, size, b):
    if (
        size == 284
        and b[:18] == bytes.fromhex('4c8bdc49895b10574881ecc0000000488d05')
        and b[22:42] == bytes.fromhex('498943a849c743b01000000041c643b800488d05')
        and b[46:66] == bytes.fromhex('498943c049c743c80a00000041c643d001488d05')
        and b[70:137] == bytes.fromhex(
            '498943d849c743e00a00000041c643e802498d5ba841c7430800000000b803000000'
            '480fbdc0b93f0000002bc848c7c2ffffffff8bc1c4e2fbf7d24885d2740c488d0d'
        )
        and b[141] == 0xE8
        and b[146:182] == bytes.fromhex(
            '48bfe9580463d2b95fde0f1f400048895c2440488d43104889442448488b5308488b0be8'
        )
        and b[186:230] == bytes.fromhex(
            '488bc8488bc748f7e14c8d0c02488d4424484889442430488d44244048894424284c8bc3488d542450488d0d'
        )
        and b[234] == 0xE8
        and b[239:259] == bytes.fromhex('4883c318488d8424b8000000483bd875a0488d0d')
        and b[263:280] == bytes.fromhex('488b9c24d80000004881c4c00000005fe9')
    ):
        s0 = va + 22 + struct.unpack_from('<i', b, 18)[0]
        s1 = va + 46 + struct.unpack_from('<i', b, 42)[0]
        s2 = va + 70 + struct.unpack_from('<i', b, 66)[0]
        m1 = va + 141 + struct.unpack_from('<i', b, 137)[0]
        f_res = va + 146 + struct.unpack_from('<i', b, 142)[0]
        f_hash = va + 186 + struct.unpack_from('<i', b, 182)[0]
        m2 = va + 234 + struct.unpack_from('<i', b, 230)[0]
        f_ins = va + 239 + struct.unpack_from('<i', b, 235)[0]
        dtor = va + 263 + struct.unpack_from('<i', b, 259)[0]
        f_atexit = va + 284 + struct.unpack_from('<i', b, 280)[0]
        if (
            m1 == m2
            and (s1 - s0, s2 - s1) == (24, 16)
            and (f_res, f_hash, f_ins, f_atexit) == (0x1401DE990, 0x1424B16A0, 0x1401DEDE0, 0x141834D18)
        ):
            return ('tpl_init_hashmap3_atexit', (m1, s0, dtor))
    if (
        size == 395
        and b[:26] == bytes.fromhex('48895c241048897c241855488d6c24e04881ec20010000488d05')
        and b[30:52] == bytes.fromhex('488944245048c744245806000000c644246009488d05')
        and b[56:78] == bytes.fromhex('488944246848c744247008000000c644247808488d05')
        and b[82:101] == bytes.fromhex('4889458048c7458806000000c6459007488d05')
        and b[105:124] == bytes.fromhex('4889459848c745a008000000c645a80c488d05')
        and b[128:147] == bytes.fromhex('488945b048c745b808000000c645c00b488d05')
        and b[151:170] == bytes.fromhex('488945c848c745d006000000c645d80a488d05')
        and b[174:240] == bytes.fromhex(
            '488945e048c745e80b000000c645f00d488d5c2450c7453000000000b807000000'
            '480fbdc0b93f0000002bc848c7c2ffffffff8bc1c4e2fbf7d24885d2740c488d0d'
        )
        and b[244] == 0xE8
        and b[249:294] == bytes.fromhex(
            '48bfe9580463d2b95fde0f1f4000660f1f84000000000048895c2440488d43104889442448488b5308488b0be8'
        )
        and b[298:341] == bytes.fromhex(
            '488bc8488bc748f7e14c8d0c02488d4424484889442430488d44244048894424284c8bc3488d5500488d0d'
        )
        and b[345] == 0xE8
        and b[350:366] == bytes.fromhex('4883c318488d45f8483bd875a5488d0d')
        and b[370:391] == bytes.fromhex('4c8d9c2420010000498b5b18498b7b20498be35de9')
    ):
        s0 = va + 30 + struct.unpack_from('<i', b, 26)[0]
        s1 = va + 56 + struct.unpack_from('<i', b, 52)[0]
        s2 = va + 82 + struct.unpack_from('<i', b, 78)[0]
        s3 = va + 105 + struct.unpack_from('<i', b, 101)[0]
        s4 = va + 128 + struct.unpack_from('<i', b, 124)[0]
        s5 = va + 151 + struct.unpack_from('<i', b, 147)[0]
        s6 = va + 174 + struct.unpack_from('<i', b, 170)[0]
        m1 = va + 244 + struct.unpack_from('<i', b, 240)[0]
        f_res = va + 249 + struct.unpack_from('<i', b, 245)[0]
        f_hash = va + 298 + struct.unpack_from('<i', b, 294)[0]
        m2 = va + 345 + struct.unpack_from('<i', b, 341)[0]
        f_ins = va + 350 + struct.unpack_from('<i', b, 346)[0]
        dtor = va + 370 + struct.unpack_from('<i', b, 366)[0]
        f_atexit = va + 395 + struct.unpack_from('<i', b, 391)[0]
        if (
            m1 == m2
            and (s1 - s0, s2 - s1, s3 - s2, s4 - s3, s5 - s4, s6 - s5) == (12, 12, 12, 16, 12, 12)
            and (f_res, f_hash, f_ins, f_atexit) == (0x1401DE990, 0x1424B16A0, 0x1401DEDE0, 0x141834D18)
        ):
            return ('tpl_init_hashmap7_atexit', (m1, s0, dtor))
    return None


def _parse_obj_ver(va, size, b):
    if (
        size == 126
        and b[:25] == bytes.fromhex('48895c241048894c2408574883ec20488b1a488bfac7442440')
        and b[29:37] == bytes.fromhex('4584c0752c488b05')
        and b[41:49] == bytes.fromhex('4c8d442430488d15')
        and b[53:62] == bytes.fromhex('4889442430488bcbe8')
        and b[66:72] == bytes.fromhex('488b1f488d05')
        and b[76:81] == bytes.fromhex('eb13488b15')
        and b[85:88] == bytes.fromhex('488d0d')
        and b[92] == 0xE8
        and b[97:109] == bytes.fromhex('4c8d442440488bd0488bcbe8')
        and b[113:] == bytes.fromhex('488b5c243833c04883c4205fc3')
    ):
        v1 = struct.unpack_from('<I', b, 25)[0]
        t1_a = va + 41 + struct.unpack_from('<i', b, 37)[0]
        s_ot = va + 53 + struct.unpack_from('<i', b, 49)[0]
        f_ot = va + 66 + struct.unpack_from('<i', b, 62)[0]
        s_ov = va + 76 + struct.unpack_from('<i', b, 72)[0]
        t1_b = va + 85 + struct.unpack_from('<i', b, 81)[0]
        s_ovs = va + 92 + struct.unpack_from('<i', b, 88)[0]
        f_fmt = va + 97 + struct.unpack_from('<i', b, 93)[0]
        f_ver = va + 113 + struct.unpack_from('<i', b, 109)[0]
        if t1_a == t1_b and (s_ot, s_ov, s_ovs, f_ot, f_fmt, f_ver) == (
            0x14307AA58,
            0x14307A9D0,
            0x14307A9E0,
            0x1413C3940,
            0x1423FE030,
            0x142481290,
        ):
            return ('tpl_obj_ver_1', (t1_a, v1))
    if (
        size == 137
        and b[:25] == bytes.fromhex('48895c241048894c2408574883ec20488b3a488bdac7442440')
        and b[29:37] == bytes.fromhex('4584c0752c488b05')
        and b[41:49] == bytes.fromhex('4c8d442430488d15')
        and b[53:62] == bytes.fromhex('4889442430488bcfe8')
        and b[66:72] == bytes.fromhex('488b3b488d05')
        and b[76:81] == bytes.fromhex('eb13488b15')
        and b[85:88] == bytes.fromhex('488d0d')
        and b[92] == 0xE8
        and b[97:109] == bytes.fromhex('4c8d442440488bd0488bcfe8')
        and b[113:120] == bytes.fromhex('41b001488bd3e8')
        and b[124:] == bytes.fromhex('488b5c243833c04883c4205fc3')
    ):
        v1 = struct.unpack_from('<I', b, 25)[0]
        t1_a = va + 41 + struct.unpack_from('<i', b, 37)[0]
        s_ot = va + 53 + struct.unpack_from('<i', b, 49)[0]
        f_ot = va + 66 + struct.unpack_from('<i', b, 62)[0]
        s_ov = va + 76 + struct.unpack_from('<i', b, 72)[0]
        t1_b = va + 85 + struct.unpack_from('<i', b, 81)[0]
        s_ovs = va + 92 + struct.unpack_from('<i', b, 88)[0]
        f_fmt = va + 97 + struct.unpack_from('<i', b, 93)[0]
        f_ver = va + 113 + struct.unpack_from('<i', b, 109)[0]
        sub_fn = va + 124 + struct.unpack_from('<i', b, 120)[0]
        if t1_a == t1_b and (s_ot, s_ov, s_ovs, f_ot, f_fmt, f_ver) == (
            0x14307AA58,
            0x14307A9D0,
            0x14307A9E0,
            0x1413C3940,
            0x1423FE030,
            0x142481290,
        ):
            return ('tpl_obj_ver_1_call', (t1_a, v1, sub_fn))
    if (
        size == 148
        and b[:25] == bytes.fromhex('48895c24084889742410574883ec20488b3a488bdac7442440')
        and b[29:40] == bytes.fromhex('488bf14584c0752c488b05')
        and b[44:52] == bytes.fromhex('4c8d442448488d15')
        and b[56:65] == bytes.fromhex('4889442448488bcfe8')
        and b[69:75] == bytes.fromhex('488b3b488d05')
        and b[79:84] == bytes.fromhex('eb13488b15')
        and b[88:91] == bytes.fromhex('488d0d')
        and b[95] == 0xE8
        and b[100:112] == bytes.fromhex('4c8d442440488bd0488bcfe8')
        and b[116:126] == bytes.fromhex('41b001488bd3488bcee8')
        and b[130:] == bytes.fromhex('488b5c243033c0488b7424384883c4205fc3')
    ):
        v1 = struct.unpack_from('<I', b, 25)[0]
        t1_a = va + 44 + struct.unpack_from('<i', b, 40)[0]
        s_ot = va + 56 + struct.unpack_from('<i', b, 52)[0]
        f_ot = va + 69 + struct.unpack_from('<i', b, 65)[0]
        s_ov = va + 79 + struct.unpack_from('<i', b, 75)[0]
        t1_b = va + 88 + struct.unpack_from('<i', b, 84)[0]
        s_ovs = va + 95 + struct.unpack_from('<i', b, 91)[0]
        f_fmt = va + 100 + struct.unpack_from('<i', b, 96)[0]
        f_ver = va + 116 + struct.unpack_from('<i', b, 112)[0]
        sub_fn = va + 130 + struct.unpack_from('<i', b, 126)[0]
        if t1_a == t1_b and (s_ot, s_ov, s_ovs, f_ot, f_fmt, f_ver) == (
            0x14307AA58,
            0x14307A9D0,
            0x14307A9E0,
            0x1413C3940,
            0x1423FE030,
            0x142481290,
        ):
            return ('tpl_obj_ver_1_call', (t1_a, v1, sub_fn))
    if (
        size == 172
        and b[:25] == bytes.fromhex('48895c241048894c2408574883ec20488b3a488bdac7442440')
        and b[29:37] == bytes.fromhex('4584c0752c488b05')
        and b[41:49] == bytes.fromhex('4c8d442430488d15')
        and b[53:62] == bytes.fromhex('4889442430488bcfe8')
        and b[66:72] == bytes.fromhex('488b3b488d05')
        and b[76:81] == bytes.fromhex('eb13488b15')
        and b[85:88] == bytes.fromhex('488d0d')
        and b[92] == 0xE8
        and b[97:109] == bytes.fromhex('4c8d442440488bd0488bcfe8')
        and b[113:116] == bytes.fromhex('488b15')
        and b[120:123] == bytes.fromhex('488d0d')
        and b[127:134] == bytes.fromhex('488b1bc7442440')
        and b[138] == 0xE8
        and b[143:155] == bytes.fromhex('4c8d442440488bd0488bcbe8')
        and b[159:] == bytes.fromhex('488b5c243833c04883c4205fc3')
    ):
        v1 = struct.unpack_from('<I', b, 25)[0]
        t1_a = va + 41 + struct.unpack_from('<i', b, 37)[0]
        s_ot = va + 53 + struct.unpack_from('<i', b, 49)[0]
        f_ot = va + 66 + struct.unpack_from('<i', b, 62)[0]
        s_ov = va + 76 + struct.unpack_from('<i', b, 72)[0]
        t1_b = va + 85 + struct.unpack_from('<i', b, 81)[0]
        s_ovs1 = va + 92 + struct.unpack_from('<i', b, 88)[0]
        f_fmt1 = va + 97 + struct.unpack_from('<i', b, 93)[0]
        f_ver1 = va + 113 + struct.unpack_from('<i', b, 109)[0]
        t2 = va + 120 + struct.unpack_from('<i', b, 116)[0]
        s_ovs2 = va + 127 + struct.unpack_from('<i', b, 123)[0]
        v2 = struct.unpack_from('<I', b, 134)[0]
        f_fmt2 = va + 143 + struct.unpack_from('<i', b, 139)[0]
        f_ver2 = va + 159 + struct.unpack_from('<i', b, 155)[0]
        if t1_a == t1_b and (s_ot, s_ov, s_ovs1, s_ovs2, f_ot, f_fmt1, f_fmt2, f_ver1, f_ver2) == (
            0x14307AA58,
            0x14307A9D0,
            0x14307A9E0,
            0x14307A9E0,
            0x1413C3940,
            0x1423FE030,
            0x1423FE030,
            0x142481290,
            0x142481290,
        ):
            return ('tpl_obj_ver_2', (t1_a, v1, t2, v2))
    if (
        size == 218
        and b[:25] == bytes.fromhex('48895c241048894c2408574883ec20488b1a488bfac7442440')
        and b[29:37] == bytes.fromhex('4584c0752c488b05')
        and b[41:49] == bytes.fromhex('4c8d442430488d15')
        and b[53:62] == bytes.fromhex('4889442430488bcbe8')
        and b[66:72] == bytes.fromhex('488b1f488d05')
        and b[76:81] == bytes.fromhex('eb13488b15')
        and b[85:88] == bytes.fromhex('488d0d')
        and b[92] == 0xE8
        and b[97:109] == bytes.fromhex('4c8d442440488bd0488bcbe8')
        and b[113:116] == bytes.fromhex('488b15')
        and b[120:123] == bytes.fromhex('488d0d')
        and b[127:134] == bytes.fromhex('488b1fc7442440')
        and b[138] == 0xE8
        and b[143:155] == bytes.fromhex('4c8d442440488bd0488bcbe8')
        and b[159:162] == bytes.fromhex('488b15')
        and b[166:169] == bytes.fromhex('488d0d')
        and b[173:180] == bytes.fromhex('488b1fc7442440')
        and b[184] == 0xE8
        and b[189:201] == bytes.fromhex('4c8d442440488bd0488bcbe8')
        and b[205:] == bytes.fromhex('488b5c243833c04883c4205fc3')
    ):
        v1 = struct.unpack_from('<I', b, 25)[0]
        t1_a = va + 41 + struct.unpack_from('<i', b, 37)[0]
        s_ot = va + 53 + struct.unpack_from('<i', b, 49)[0]
        f_ot = va + 66 + struct.unpack_from('<i', b, 62)[0]
        s_ov = va + 76 + struct.unpack_from('<i', b, 72)[0]
        t1_b = va + 85 + struct.unpack_from('<i', b, 81)[0]
        s_ovs1 = va + 92 + struct.unpack_from('<i', b, 88)[0]
        f_fmt1 = va + 97 + struct.unpack_from('<i', b, 93)[0]
        f_ver1 = va + 113 + struct.unpack_from('<i', b, 109)[0]
        t2 = va + 120 + struct.unpack_from('<i', b, 116)[0]
        s_ovs2 = va + 127 + struct.unpack_from('<i', b, 123)[0]
        v2 = struct.unpack_from('<I', b, 134)[0]
        f_fmt2 = va + 143 + struct.unpack_from('<i', b, 139)[0]
        f_ver2 = va + 159 + struct.unpack_from('<i', b, 155)[0]
        t3 = va + 166 + struct.unpack_from('<i', b, 162)[0]
        s_ovs3 = va + 173 + struct.unpack_from('<i', b, 169)[0]
        v3 = struct.unpack_from('<I', b, 180)[0]
        f_fmt3 = va + 189 + struct.unpack_from('<i', b, 185)[0]
        f_ver3 = va + 205 + struct.unpack_from('<i', b, 201)[0]
        if (
            t1_a == t1_b
            and (s_ot, s_ov, s_ovs1, s_ovs2, s_ovs3)
            == (0x14307AA58, 0x14307A9D0, 0x14307A9E0, 0x14307A9E0, 0x14307A9E0)
            and (f_ot, f_fmt1, f_fmt2, f_fmt3, f_ver1, f_ver2, f_ver3)
            == (0x1413C3940, 0x1423FE030, 0x1423FE030, 0x1423FE030, 0x142481290, 0x142481290, 0x142481290)
        ):
            return ('tpl_obj_ver_3', (t1_a, v1, t2, v2, t3, v3))
    return None


def _parse_refl_leaf(va, b):
    if len(b) < 20:
        return None
    if b[:4] == b'\x48\x83\xec\x38' and b[-5:] == b'\x48\x83\xc4\x38\xc3':
        body = b[4:-5]
    elif b[:4] == b'\x48\x83\xec\x48' and b[-5:] == b'\x48\x83\xc4\x48\xc3':
        body = b[4:-5]
    else:
        return None
    if body[-1] == 0x90 and len(body) >= 6 and body[-6] == 0xE8:
        call_off = 4 + len(body) - 6
        target_fn = va + call_off + 5 + struct.unpack_from('<i', body, len(body) - 5)[0]
        insns = body[:-6]
    elif len(body) >= 5 and body[-5] == 0xE8:
        call_off = 4 + len(body) - 5
        target_fn = va + call_off + 5 + struct.unpack_from('<i', body, len(body) - 4)[0]
        insns = body[:-5]
    else:
        return None
    p = 0
    end = len(insns)
    mode = None
    field_off = 0
    r8d_val = 0
    r9_va = 0
    s20 = (0, 0)
    s28 = (0, 0)
    s30 = (0, 0)
    s38 = (0, 0)
    reg_rcx = (0, 0)
    reg_xmm0 = (0, 0)
    while p < end:
        op = insns[p:]
        if op[:2] == b'\x33\xc9':
            reg_rcx = (0, 0)
            p += 2
        elif op[:3] == b'\x45\x33\xc0':
            r8d_val = 0
            p += 3
        elif op[:3] == b'\x49\x8b\xc0':
            mode = 0
            p += 3
        elif op[:3] == b'\x48\x8b\xc2':
            mode = 1
            p += 3
        elif op[:3] == b'\x48\x8b\xc8':
            p += 3
        elif op[:3] == b'\x4c\x8d\x0d':
            r9_va = va + 4 + p + 7 + struct.unpack_from('<i', op, 3)[0]
            p += 7
        elif op[:3] == b'\x48\x8d\x0d':
            reg_rcx = (1, va + 4 + p + 7 + struct.unpack_from('<i', op, 3)[0])
            p += 7
        elif op[:4] == b'\xc5\xfb\x10\x05':
            reg_xmm0 = (2, va + 4 + p + 8 + struct.unpack_from('<i', op, 4)[0])
            p += 8
        elif op[:4] == b'\xc5\xfa\x10\x05':
            reg_xmm0 = (3, va + 4 + p + 8 + struct.unpack_from('<i', op, 4)[0])
            p += 8
        elif op[:4] == b'\x48\x89\x4c\x24' and op[4] in (0x20, 0x28, 0x30, 0x38):
            slot = op[4]
            if slot == 0x20:
                s20 = reg_rcx
            elif slot == 0x28:
                s28 = reg_rcx
            elif slot == 0x30:
                s30 = reg_rcx
            elif slot == 0x38:
                s38 = reg_rcx
            p += 5
        elif op[:4] == b'\x48\xc7\x44\x24' and op[4] in (0x20, 0x28, 0x30, 0x38):
            slot = op[4]
            imm = struct.unpack_from('<i', op, 5)[0] & 0xFFFFFFFFFFFFFFFF
            if slot == 0x20:
                s20 = (0, imm)
            elif slot == 0x28:
                s28 = (0, imm)
            elif slot == 0x30:
                s30 = (0, imm)
            elif slot == 0x38:
                s38 = (0, imm)
            p += 9
        elif op[:3] == b'\xc7\x44\x24' and op[3] in (0x20, 0x28, 0x30, 0x38):
            slot = op[3]
            imm = struct.unpack_from('<I', op, 4)[0]
            if slot == 0x20:
                s20 = (0, imm)
            elif slot == 0x28:
                s28 = (0, imm)
            elif slot == 0x30:
                s30 = (0, imm)
            elif slot == 0x38:
                s38 = (0, imm)
            p += 8
        elif op[:3] == b'\xc6\x44\x24' and op[3] in (0x20, 0x28, 0x30, 0x38):
            slot = op[3]
            imm = op[4]
            if slot == 0x20:
                s20 = (0, imm)
            elif slot == 0x28:
                s28 = (0, imm)
            elif slot == 0x30:
                s30 = (0, imm)
            elif slot == 0x38:
                s38 = (0, imm)
            p += 5
        elif op[:5] in (b'\xc5\xfb\x11\x44\x24', b'\xc5\xfa\x11\x44\x24') and op[5] in (0x20, 0x28, 0x30, 0x38):
            slot = op[5]
            if slot == 0x20:
                s20 = reg_xmm0
            elif slot == 0x28:
                s28 = reg_xmm0
            elif slot == 0x30:
                s30 = reg_xmm0
            elif slot == 0x38:
                s38 = reg_xmm0
            p += 6
        elif op[:3] == b'\x48\x83\xc2':
            field_off = struct.unpack_from('<b', op, 3)[0]
            p += 4
        elif op[:4] == b'\x48\x83\xea\x80':
            field_off = 0x80
            p += 4
        elif op[:3] == b'\x48\x81\xc2':
            field_off = struct.unpack_from('<i', op, 3)[0]
            p += 7
        elif op[:3] == b'\x48\x8d\x51':
            field_off = struct.unpack_from('<b', op, 3)[0]
            p += 4
        elif op[:3] == b'\x48\x8d\x91':
            field_off = struct.unpack_from('<i', op, 3)[0]
            p += 7
        elif op[:3] == b'\x48\x8b\xd1':
            field_off = 0
            p += 3
        elif op[:3] == b'\x44\x8d\x41' and reg_rcx == (0, 0):
            r8d_val = op[3]
            p += 4
        elif op[:2] == b'\x41\xb8':
            r8d_val = struct.unpack_from('<I', op, 2)[0]
            p += 6
        else:
            return None
    if mode is None or r9_va == 0 or field_off < 0 or r8d_val >= 0x10000:
        return None
    meta = (
        mode
        | (s20[0] << 1)
        | (s28[0] << 3)
        | (s30[0] << 5)
        | (s38[0] << 7)
        | (r8d_val << 16)
        | ((field_off & 0xFFFFFFFF) << 32)
    )
    return ('tpl_refl_leaf', (meta, r9_va, s20[1], s28[1], s30[1], s38[1], target_fn))


def _encode_stream_op(op):
    kind = op[0]
    if kind == 'begin':
        _, off, r9d_va, r8_va, f1, f2, fn = op
        b_id = _BEGIN_FN_MAP.get(fn)
        if b_id is None or f1 > 0xFF or f2 > 0xFF:
            return None
        meta = b_id | (f1 << 8) | (f2 << 16) | ((off & 0xFFFFFFFF) << 32)
        val = ((r9d_va - 0x140000000) << 32) | (r8_va - 0x140000000)
        return (meta, val)
    if kind == 'push':
        _, off, r8_va, f1, f2, fn = op
        if fn != 0x14243B0B0 or f1 > 0xFF or f2 > 0xFF:
            return None
        meta = 5 | (f1 << 8) | (f2 << 16) | ((off & 0xFFFFFFFF) << 32)
        return (meta, r8_va)
    if kind == 'call_obj_s':
        _, off, fn = op
        meta = 6 | ((off & 0xFFFFFFFF) << 32)
        return (meta, fn)
    if kind == 'call_s_obj':
        _, off, fn = op
        meta = 7 | ((off & 0xFFFFFFFF) << 32)
        return (meta, fn)
    if kind == 'vcall':
        _, off, voff = op
        meta = 8 | ((off & 0xFFFFFFFF) << 32)
        return (meta, voff)
    if kind == 'pop':
        return (9, 0)
    if kind == 'end':
        _, fn = op
        return (10, fn)
    return None


def _parse_ec40_body(va, b):
    if b[:10] != bytes.fromhex('48895c2408574883ec40'):
        return None
    if b.endswith(_EP2_A):
        body = b[10:-len(_EP2_A)]
        ep_pops = 2
    elif b.endswith(_EP1_A):
        body = b[10:-len(_EP1_A)]
        ep_pops = 1
    elif b.endswith(_EP1_B):
        body = b[10:-len(_EP1_B)]
        ep_pops = 1
    elif b.endswith(_EP1_C):
        body = b[10:-len(_EP1_C)]
        ep_pops = 1
    elif b.endswith(_EP1_D):
        body = b[10:-len(_EP1_D)]
        ep_pops = 1
    elif b.endswith(_EP0_A):
        body = b[10:-len(_EP0_A)]
        ep_pops = 0
    else:
        return None
    p = 0
    end = len(body)
    ops = []
    if body[:3] == b'\x4c\x8b\x05':
        r8_va = va + 10 + 7 + struct.unpack_from('<i', body, 3)[0]
        if (
            body[7:17] in (bytes.fromhex('488bfa488bd1c7442428'), bytes.fromhex('488bda488bd1c7442428'))
            and body[21:28] in (bytes.fromhex('488bd9c7442420'), bytes.fromhex('488bf9c7442420'))
            and body[32:36] in (b'\x48\x8b\xcf\xe8', b'\x48\x8b\xcb\xe8')
        ):
            s_is_rdi = body[7:10] == b'\x48\x8b\xfa'
            f2 = struct.unpack_from('<I', body, 17)[0]
            f1 = struct.unpack_from('<I', body, 28)[0]
            fn = va + 10 + 40 + struct.unpack_from('<i', body, 36)[0]
            ops.append(('push', 0, r8_va, f1, f2, fn))
            p = 40
        elif body[7:30] in (
            bytes.fromhex('488bfa33c0488bd1488bd989442428488bcf89442420e8'),
            bytes.fromhex('488bda33c0488bd1488bf989442428488bcb89442420e8'),
            bytes.fromhex('488bfa33c0488bd189442428488bd989442420488bcfe8'),
            bytes.fromhex('488bda33c0488bd189442428488bf989442420488bcbe8'),
        ):
            s_is_rdi = body[7:10] == b'\x48\x8b\xfa'
            fn = va + 10 + 34 + struct.unpack_from('<i', body, 30)[0]
            ops.append(('push', 0, r8_va, 0, 0, fn))
            p = 34
        else:
            return None
    elif body[:3] == b'\x44\x8b\x0d':
        r9d_va = va + 10 + 7 + struct.unpack_from('<i', body, 3)[0]
        if body[7:13] in (b'\x48\x8b\xfa\x4c\x8b\x05', b'\x48\x8b\xda\x4c\x8b\x05'):
            s_is_rdi = body[7:10] == b'\x48\x8b\xfa'
            r8_va = va + 10 + 17 + struct.unpack_from('<i', body, 13)[0]
            if (
                body[17:27] in (bytes.fromhex('488bd1488bd9c7442428'), bytes.fromhex('488bd1488bf9c7442428'))
                and body[31:38] in (bytes.fromhex('488bcfc7442420'), bytes.fromhex('488bcbc7442420'))
                and body[42] == 0xE8
            ):
                f2 = struct.unpack_from('<I', body, 27)[0]
                f1 = struct.unpack_from('<I', body, 38)[0]
                fn = va + 10 + 47 + struct.unpack_from('<i', body, 43)[0]
                ops.append(('begin', 0, r9d_va, r8_va, f1, f2, fn))
                p = 47
            elif body[17:41] in (
                bytes.fromhex('33c0488bd189442428488bd989442420488bcfe8'),
                bytes.fromhex('33c0488bd189442428488bf989442420488bcbe8'),
                bytes.fromhex('33c0488bd1488bd989442428488bcf89442420e8'),
                bytes.fromhex('33c0488bd1488bf989442428488bcb89442420e8'),
            ):
                fn = va + 10 + 41 + struct.unpack_from('<i', body, 37)[0]
                ops.append(('begin', 0, r9d_va, r8_va, 0, 0, fn))
                p = 41
            else:
                return None
        else:
            return None
    else:
        return None

    while p < end:
        rem = body[p:]
        if rem[:3] == b'\x4c\x8b\x05':
            r8_va = va + 10 + p + 7 + struct.unpack_from('<i', rem, 3)[0]
            s2 = rem[7:]
            if s2[:3] in (b'\x48\x8b\xd3', b'\x48\x8b\xd7'):
                off = 0
                s3 = s2[3:]
                step = 10
            elif s2[:3] in (b'\x48\x8d\x53', b'\x48\x8d\x57'):
                off = struct.unpack_from('<b', s2, 3)[0]
                s3 = s2[4:]
                step = 11
            elif s2[:3] in (b'\x48\x8d\x93', b'\x48\x8d\x97'):
                off = struct.unpack_from('<i', s2, 3)[0]
                s3 = s2[7:]
                step = 14
            else:
                return None
            if (
                s3[:7] in (b'\x48\x8b\xcf\xc7\x44\x24\x28', b'\x48\x8b\xcb\xc7\x44\x24\x28')
                and s3[11:15] == b'\xc7\x44\x24\x20'
                and s3[19] == 0xE8
            ):
                f2 = struct.unpack_from('<I', s3, 7)[0]
                f1 = struct.unpack_from('<I', s3, 15)[0]
                fn = va + 10 + p + step + 24 + struct.unpack_from('<i', s3, 20)[0]
                ops.append(('push', off, r8_va, f1, f2, fn))
                p += step + 24
                continue
            return None
        if rem[:3] == b'\x44\x8b\x0d':
            r9d_va = va + 10 + p + 7 + struct.unpack_from('<i', rem, 3)[0]
            s2 = rem[7:]
            if s2[:3] in (b'\x48\x8b\xd3', b'\x48\x8b\xd7'):
                off = 0
                s3 = s2[3:]
                step = 10
            elif s2[:3] in (b'\x48\x8d\x53', b'\x48\x8d\x57'):
                off = struct.unpack_from('<b', s2, 3)[0]
                s3 = s2[4:]
                step = 11
            elif s2[:3] in (b'\x48\x8d\x93', b'\x48\x8d\x97'):
                off = struct.unpack_from('<i', s2, 3)[0]
                s3 = s2[7:]
                step = 14
            else:
                return None
            if s3[:3] == b'\x4c\x8b\x05':
                r8_va = va + 10 + p + step + 7 + struct.unpack_from('<i', s3, 3)[0]
                s4 = s3[7:]
                if (
                    s4[:7] in (b'\x48\x8b\xcf\xc7\x44\x24\x28', b'\x48\x8b\xcb\xc7\x44\x24\x28')
                    and s4[11:15] == b'\xc7\x44\x24\x20'
                    and s4[19] == 0xE8
                ):
                    f2 = struct.unpack_from('<I', s4, 7)[0]
                    f1 = struct.unpack_from('<I', s4, 15)[0]
                    fn = va + 10 + p + step + 31 + struct.unpack_from('<i', s4, 20)[0]
                    ops.append(('begin', off, r9d_va, r8_va, f1, f2, fn))
                    p += step + 31
                    continue
            return None
        if rem[:26] == bytes.fromhex('8b5708488b07488d0c524803c98b4cc8f88d42ff898f28030000'):
            s2 = rem[26:]
            if s2[:10] == b'\x48\x8d\x53\x08\x48\x8b\xcf\x89\x47\x08' and s2[10] == 0xE8:
                fn = va + 10 + p + 26 + 15 + struct.unpack_from('<i', s2, 11)[0]
                ops.append(('pop',))
                ops.append(('call_s_obj', 8, fn))
                p += 26 + 15
                continue
            if s2[:10] == b'\x48\x8b\xd7\x48\x8d\x4b\x08\x89\x47\x08' and s2[10] == 0xE8:
                fn = va + 10 + p + 26 + 15 + struct.unpack_from('<i', s2, 11)[0]
                ops.append(('pop',))
                ops.append(('call_obj_s', 8, fn))
                p += 26 + 15
                continue
        if rem[:6] in (b'\x48\x8b\xd7\x48\x8b\xcb', b'\x48\x8b\xd3\x48\x8b\xcf') and rem[6] == 0xE8:
            fn = va + 10 + p + 11 + struct.unpack_from('<i', rem, 7)[0]
            is_d7 = rem[:3] == b'\x48\x8b\xd7'
            kind = 'call_obj_s' if (is_d7 == s_is_rdi) else 'call_s_obj'
            ops.append((kind, 0, fn))
            p += 11
            continue
        if rem[:3] in (b'\x48\x8d\x4b', b'\x48\x8d\x4f') and rem[4:8] in (b'\x48\x8b\xd7\xe8', b'\x48\x8b\xd3\xe8'):
            off8 = struct.unpack_from('<b', rem, 3)[0]
            fn = va + 10 + p + 12 + struct.unpack_from('<i', rem, 8)[0]
            ops.append(('call_obj_s', off8, fn))
            p += 12
            continue
        if rem[:3] in (b'\x48\x8b\xd7', b'\x48\x8b\xd3') and rem[3:6] in (b'\x48\x8d\x4b', b'\x48\x8d\x4f') and rem[7] == 0xE8:
            off8 = struct.unpack_from('<b', rem, 6)[0]
            fn = va + 10 + p + 12 + struct.unpack_from('<i', rem, 8)[0]
            ops.append(('call_obj_s', off8, fn))
            p += 12
            continue
        if rem[:3] in (b'\x48\x8d\x53', b'\x48\x8d\x57') and rem[4:8] in (b'\x48\x8b\xcf\xe8', b'\x48\x8b\xcb\xe8'):
            off8 = struct.unpack_from('<b', rem, 3)[0]
            fn = va + 10 + p + 12 + struct.unpack_from('<i', rem, 8)[0]
            ops.append(('call_s_obj', off8, fn))
            p += 12
            continue
        if rem[:3] in (b'\x48\x8b\xcf', b'\x48\x8b\xcb') and rem[3:6] in (b'\x48\x8d\x53', b'\x48\x8d\x57') and rem[7] == 0xE8:
            off8 = struct.unpack_from('<b', rem, 6)[0]
            fn = va + 10 + p + 12 + struct.unpack_from('<i', rem, 8)[0]
            ops.append(('call_s_obj', off8, fn))
            p += 12
            continue
        if rem[:3] in (b'\x48\x8d\x8b', b'\x48\x8d\x8f') and rem[7:11] in (b'\x48\x8b\xd7\xe8', b'\x48\x8b\xd3\xe8'):
            off32 = struct.unpack_from('<i', rem, 3)[0]
            fn = va + 10 + p + 15 + struct.unpack_from('<i', rem, 11)[0]
            ops.append(('call_obj_s', off32, fn))
            p += 15
            continue
        if rem[:3] in (b'\x48\x8b\xd7', b'\x48\x8b\xd3') and rem[3:6] in (b'\x48\x8d\x8b', b'\x48\x8d\x8f') and rem[10] == 0xE8:
            off32 = struct.unpack_from('<i', rem, 6)[0]
            fn = va + 10 + p + 15 + struct.unpack_from('<i', rem, 11)[0]
            ops.append(('call_obj_s', off32, fn))
            p += 15
            continue
        if rem[:7] in (bytes.fromhex('488b8f38030000'), bytes.fromhex('488b8b38030000')):
            s2 = rem[7:]
            if s2[:6] in (
                b'\x48\x8b\xd3\x48\x8b\x01',
                b'\x48\x8b\xd7\x48\x8b\x01',
                b'\x48\x8b\x01\x48\x8b\xd3',
                b'\x48\x8b\x01\x48\x8b\xd7',
            ):
                rdx_off = 0
                s3 = s2[6:]
                step = 13
            elif s2[:3] in (b'\x48\x8d\x53', b'\x48\x8d\x57') and s2[4:7] == b'\x48\x8b\x01':
                rdx_off = struct.unpack_from('<b', s2, 3)[0]
                s3 = s2[7:]
                step = 14
            elif s2[:3] == b'\x48\x8b\x01' and s2[3:6] in (b'\x48\x8d\x53', b'\x48\x8d\x57'):
                rdx_off = struct.unpack_from('<b', s2, 6)[0]
                s3 = s2[7:]
                step = 14
            elif s2[:3] in (b'\x48\x8d\x93', b'\x48\x8d\x97') and s2[7:10] == b'\x48\x8b\x01':
                rdx_off = struct.unpack_from('<i', s2, 3)[0]
                s3 = s2[10:]
                step = 17
            elif s2[:3] == b'\x48\x8b\x01' and s2[3:6] in (b'\x48\x8d\x93', b'\x48\x8d\x97'):
                rdx_off = struct.unpack_from('<i', s2, 6)[0]
                s3 = s2[10:]
                step = 17
            else:
                return None
            if s3[:2] == b'\xff\x10':
                voff = 0
                step += 2
            elif s3[:2] == b'\xff\x50':
                voff = s3[2]
                step += 3
            elif s3[:2] == b'\xff\x90':
                voff = struct.unpack_from('<I', s3, 2)[0]
                step += 6
            else:
                return None
            ops.append(('vcall', rdx_off, voff))
            p += step
            continue
        if rem[:4] in (b'\x48\x8b\xcf\xe8', b'\x48\x8b\xcb\xe8'):
            fn = va + 10 + p + 8 + struct.unpack_from('<i', rem, 4)[0]
            ops.append(('end', fn))
            p += 8
            continue
        return None
    if not (1 <= len(ops) <= 8):
        return None
    encoded = []
    for op in ops:
        enc = _encode_stream_op(op)
        if enc is None:
            return None
        encoded.extend(enc)
    while len(encoded) < 16:
        encoded.extend((0, 0))
    encoded.append(ep_pops)
    return ('tpl_stream_seq_8', tuple(encoded))


def match_phase4_complex(va, size, b):
    res = (
        _parse_comp_desc(va, size, b)
        or _parse_hashmap_init(va, size, b)
        or _parse_obj_ver(va, size, b)
        or _parse_refl_leaf(va, b)
        or _parse_ec40_body(va, b)
        or _parse_desc_array_init(va, b)
    )
    return res if res is not None else (None, None)


def main():
    exe_path = os.environ.get('AW2_EXE') or (
        os.path.join(os.environ['AW2_GAME_DIR'], 'AlanWake2.exe')
        if os.environ.get('AW2_GAME_DIR')
        else ''
    )
    if not exe_path or not os.path.isfile(exe_path):
        raise SystemExit('set AW2_EXE or AW2_GAME_DIR to your local Alan Wake 2 install')
    db_path = os.path.join(ROOT, 'db', 'functions.sqlite')
    out_path = os.path.join(ROOT, 'crates', 'aw2-game', 'src', 'shards', 'templates_c.rs')

    pe = fndb.PE(exe_path)
    db = sqlite3.connect(db_path)

    excluded_vas = load_excluded_vas()
    can_throw_map = dict(db.execute('SELECT va, mask_direct FROM can_throw').fetchall())

    rows = db.execute(
        "SELECT va, size, name, n_frags FROM functions WHERE owner = 'remedy' ORDER BY va"
    ).fetchall()

    p7_pre = bytes.fromhex('48 8d 05')
    p9_64_pre = bytes.fromhex('48 8b 05')
    p9_32_pre = bytes.fromhex('8b 05')
    p9_16_pre = bytes.fromhex('0f b7 05')
    p9_8_pre = bytes.fromhex('0f b6 05')
    p3_pre = bytes.fromhex('48 8d 0d')
    p_mov_rcx_rip = bytes.fromhex('48 8b 0d')
    p_lea_rdx_rip = bytes.fromhex('48 8d 15')
    p_lea_r8_rip = bytes.fromhex('4c 8d 05')
    p_lea_r9_rip = bytes.fromhex('4c 8d 0d')
    p_store_rcx_rip = bytes.fromhex('48 89 0d')
    p_store_ecx_rip = bytes.fromhex('89 0d')
    p_store_cl_rip = bytes.fromhex('88 0d')
    p_store_imm8_rip = bytes.fromhex('c6 05')
    p_store_imm32_rip = bytes.fromhex('c7 05')
    p_cmp_u64_rip = bytes.fromhex('48 83 3d')
    p_cmp_u32_rip = bytes.fromhex('83 3d')
    p_cmp_u8_rip = bytes.fromhex('80 3d')
    p_setne_ret = bytes.fromhex('0f 95 c0 c3')
    p_sete_ret = bytes.fromhex('0f 94 c0 c3')
    p16_pre = bytes.fromhex('48 83 ec 28 33 d2 48 8d 0d')
    p16_40_pre = bytes.fromhex('48 83 ec 28 ba')
    p16_mid1 = bytes.fromhex('48 8d 05')
    p16_mid2 = bytes.fromhex('48 89 05')
    p16_post = bytes.fromhex('48 83 c4 28 c3')
    p20_pre = bytes.fromhex('48 83 ec 28 48 8d 0d')
    p20_mid = bytes.fromhex('90 48 8d 0d')
    p20_post = bytes.fromhex('90 48 83 c4 28 c3')
    p_vmovups_ld = bytes.fromhex('c5 f8 10 05')
    p_vmovups_st = bytes.fromhex('c5 f8 11 05')
    p_tls_pre = bytes.fromhex('65 48 8b 04 25 58 00 00 00 b9')
    p_tls_mid = bytes.fromhex('48 03 08 e9')
    p_store_rdx_ret_rdx = bytes.fromhex('48 89 02 48 8b c2 c3')
    p_store_rcx_ret_rcx = bytes.fromhex('48 89 01 48 8b c1 c3')
    p_store_rcx_ret = bytes.fromhex('48 89 01 c3')
    p_add_rcx_imm32 = bytes.fromhex('48 81 c1')
    p_add_rcx_imm8 = bytes.fromhex('48 83 c1')
    p_sdel_pre = bytes.fromhex('40 53 48 83 ec 20 48 8d 05')
    p_sdel_mid = bytes.fromhex('48 8b d9 48 89 01 f6 c2 01 74 0a ba')
    p_sdel_post = bytes.fromhex('48 8b c3 48 83 c4 20 5b c3')

    # Phase 3 patterns:
    p134_pre = bytes.fromhex('48895c2408574883ec70488bf90fb71d')
    p134_m1 = bytes.fromhex('488d4c2420e8')
    p134_m2 = bytes.fromhex('c5fc1000c5fc114424204c8d4c2420440fb7c3488d542440c5f877e8')
    p134_m3 = bytes.fromhex('488d4f08488d542440e8')
    p134_m4 = bytes.fromhex('90488b4c24504885c97405e8')
    p134_m5 = bytes.fromhex('33c048894424504889442458488b4c24404885c97406e8')
    p134_post = bytes.fromhex('90488b9c24800000004883c4705fc3')

    p75_pre = bytes.fromhex('40534883ec40498bd8c7442428010000004c8b05')
    p75_m1 = bytes.fromhex('488bcbc744242001000000e8')
    p75_post = bytes.fromhex('8b5308488b03488d0c524803c98b4cc8f88d42ff894308898b280300004883c4405bc3')

    p69_pre = bytes.fromhex('40534883ec40498bd833c04c8b05')
    p69_m1 = bytes.fromhex('488bcb8944242889442420e8')

    p80a_pre = bytes.fromhex('40534883ec404c8b05')
    p80a_m1 = bytes.fromhex('488bda488bd1c744242801000000488bcbc744242001000000e8')
    p80a_post = bytes.fromhex('448b4308488b034b8d0c404803c98b4cc8f8418d40ff894308898b280300004883c4405bc3')

    p74_pre = bytes.fromhex('40534883ec404c8b05')
    p74_m1 = bytes.fromhex('488bda33c0488bd189442428488bcb89442420e8')

    p83_pre = bytes.fromhex('48895c2408574883ec40448b0d')
    p83_m1 = bytes.fromhex('498bf84c8b05')
    p83_m2 = bytes.fromhex('488bcfc744242801000000488bdac744242001000000e8')
    p83_m3 = bytes.fromhex('488bd7488bcbe8')
    p83_m4 = bytes.fromhex('488bcf488b5c24504883c4405fe9')

    p77_m2 = bytes.fromhex('33c089442428488bcf89442420488bdae8')

    p86_m1 = bytes.fromhex('488bfa4c8b05')
    p86_m2 = bytes.fromhex('488bd1488bd9c744242801000000488bcfc744242001000000e8')

    p80b_m2 = bytes.fromhex('33c0488bd189442428488bd989442420488bcfe8')

    p127_m3 = bytes.fromhex('448b0d')
    p127_m4 = bytes.fromhex('488bd34c8b05')
    p127_m5 = bytes.fromhex('488bcfc744242801000000c744242001000000e8')
    p127_m6 = bytes.fromhex('488bcfe8')
    p127_post = bytes.fromhex('488b5c245033c04883c4405fc3')

    p103_pre = bytes.fromhex('48895c2408574883ec404c8b05')
    p103_m1 = bytes.fromhex('488bfa488bd1c744242801000000488bd9c744242001000000488bcfe8')
    p103_post = bytes.fromhex('8b5708488b07488b5c2450488d0c524803c98b4cc8f88d42ff89470833c0898f280300004883c4405fc3')

    p97_m1 = bytes.fromhex('488bfa33c0488bd1488bd989442428488bcf89442420e8')

    p148_m2 = bytes.fromhex('4c8b05')
    p148_m3 = bytes.fromhex('488bd3488bcfc744242801000000c744242001000000e8')
    p148_post = bytes.fromhex(
        '8b5708488b07488b5c2450488d0c524803c9ffca8b4cc8f8898f28030000488d0c524803c98957088b4cc8f88d42ff89470833c0898f280300004883c4405fc3'
    )

    p60_pre = bytes.fromhex('40534883ec40448b0d')
    p60_m1 = bytes.fromhex('498bd84c8b05')
    p60_m2 = bytes.fromhex('488bcbc744242801000000c744242001000000e8')
    p60_m3 = bytes.fromhex('488bcb4883c4405be9')

    p54_m2 = bytes.fromhex('33c089442428488bcb89442420e8')

    p63_m1 = bytes.fromhex('488bda4c8b05')
    p63_m2 = bytes.fromhex('488bd1488bcbc744242801000000c744242001000000e8')

    p57_m2 = bytes.fromhex('33c0488bd189442428488bcb89442420e8')

    entries = []
    counts = {}

    for va, size, name, n_frags in rows:
        if va in excluded_vas or n_frags > 0:
            continue
        rva = va - pe.image_base
        off = rva - pe.text_rva
        if off < 0 or off + size > len(pe.text):
            continue
        b = pe.text[off:off + size]

        tpl_fn = None
        params = None

        # Template #7 (8 bytes): 48 8d 05 <rip_disp32> c3 (lea rax, [rip + disp32]; ret)
        if size == 8 and b[:3] == p7_pre and b[7] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_lea_rip_ret'
            params = (va + 7 + disp32,)
        # Template #9 64-bit (8 bytes): 48 8b 05 <rip_disp32> c3 (mov rax, qword ptr [rip + disp32]; ret)
        elif size == 8 and b[:3] == p9_64_pre and b[7] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_load_rip_u64'
            params = (va + 7 + disp32,)
        # Template #9 32-bit (7 bytes): 8b 05 <rip_disp32> c3 (mov eax, dword ptr [rip + disp32]; ret)
        elif size == 7 and b[:2] == p9_32_pre and b[6] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 2)[0]
            tpl_fn = 'tpl_load_rip_u32'
            params = (va + 6 + disp32,)
        # Template #9 16-bit (8 bytes): 0f b7 05 <rip_disp32> c3 (movzx eax, word ptr [rip + disp32]; ret)
        elif size == 8 and b[:3] == p9_16_pre and b[7] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_load_rip_u16'
            params = (va + 7 + disp32,)
        # Template #9 8-bit (8 bytes): 0f b6 05 <rip_disp32> c3 (movzx eax, byte ptr [rip + disp32]; ret)
        elif size == 8 and b[:3] == p9_8_pre and b[7] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_load_rip_u8'
            params = (va + 7 + disp32,)
        # Template #3 (12 bytes): 48 8d 0d <rip_disp32> e9 <jmp_rel32> (lea rcx, [rip + disp32]; jmp TARGET_VA)
        elif size == 12 and b[:3] == p3_pre and b[7] == 0xe9:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            rel32 = struct.unpack_from('<i', b, 8)[0]
            tpl_fn = 'tpl_bind_rcx_jmp'
            params = (va + 7 + disp32, va + 12 + rel32)
        # mov rcx, [rip + A]; jmp TARGET_VA (12 bytes)
        elif size == 12 and b[:3] == p_mov_rcx_rip and b[7] == 0xe9:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            rel32 = struct.unpack_from('<i', b, 8)[0]
            tpl_fn = 'tpl_load_rcx_rip_jmp'
            params = (va + 7 + disp32, va + 12 + rel32)
        # lea rdx, [rip + A]; jmp TARGET_VA (12 bytes)
        elif size == 12 and b[:3] == p_lea_rdx_rip and b[7] == 0xe9:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            rel32 = struct.unpack_from('<i', b, 8)[0]
            tpl_fn = 'tpl_bind_rdx_jmp'
            params = (va + 7 + disp32, va + 12 + rel32)
        # lea r8, [rip + A]; jmp TARGET_VA (12 bytes)
        elif size == 12 and b[:3] == p_lea_r8_rip and b[7] == 0xe9:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            rel32 = struct.unpack_from('<i', b, 8)[0]
            tpl_fn = 'tpl_bind_r8_jmp'
            params = (va + 7 + disp32, va + 12 + rel32)
        # lea r9, [rip + A]; jmp TARGET_VA (12 bytes)
        elif size == 12 and b[:3] == p_lea_r9_rip and b[7] == 0xe9:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            rel32 = struct.unpack_from('<i', b, 8)[0]
            tpl_fn = 'tpl_bind_r9_jmp'
            params = (va + 7 + disp32, va + 12 + rel32)
        # lea rcx, [rip + A]; add rcx, imm32; jmp TARGET_VA (19 bytes)
        elif size == 19 and b[:3] == p3_pre and b[7:10] == p_add_rcx_imm32 and b[14] == 0xe9:
            disp_a = struct.unpack_from('<i', b, 3)[0]
            add_imm = struct.unpack_from('<i', b, 10)[0]
            rel_t = struct.unpack_from('<i', b, 15)[0]
            tpl_fn = 'tpl_bind_rcx_jmp'
            params = ((va + 7 + disp_a + add_imm) & 0xffffffffffffffff, va + 19 + rel_t)
        # lea rcx, [rip + A]; add rcx, imm8; jmp TARGET_VA (16 bytes)
        elif size == 16 and b[:3] == p3_pre and b[7:10] == p_add_rcx_imm8 and b[11] == 0xe9:
            disp_a = struct.unpack_from('<i', b, 3)[0]
            add_imm = struct.unpack_from('<b', b, 10)[0]
            rel_t = struct.unpack_from('<i', b, 12)[0]
            tpl_fn = 'tpl_bind_rcx_jmp'
            params = ((va + 7 + disp_a + add_imm) & 0xffffffffffffffff, va + 16 + rel_t)
        # mov [rip + A], rcx; ret (8 bytes)
        elif size == 8 and b[:3] == p_store_rcx_rip and b[7] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_store_rip_u64'
            params = (va + 7 + disp32,)
        # mov [rip + A], ecx; ret (7 bytes)
        elif size == 7 and b[:2] == p_store_ecx_rip and b[6] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 2)[0]
            tpl_fn = 'tpl_store_rip_u32'
            params = (va + 6 + disp32,)
        # mov [rip + A], cl; ret (7 bytes)
        elif size == 7 and b[:2] == p_store_cl_rip and b[6] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 2)[0]
            tpl_fn = 'tpl_store_rip_u8'
            params = (va + 6 + disp32,)
        # mov byte ptr [rip + A], imm8; ret (8 bytes)
        elif size == 8 and b[:2] == p_store_imm8_rip and b[7] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 2)[0]
            imm8 = b[6]
            tpl_fn = 'tpl_store_rip_imm8'
            params = (va + 7 + disp32, imm8)
        # mov dword ptr [rip + A], imm32; ret (11 bytes)
        elif size == 11 and b[:2] == p_store_imm32_rip and b[10] == 0xc3:
            disp32 = struct.unpack_from('<i', b, 2)[0]
            imm32 = struct.unpack_from('<I', b, 6)[0]
            tpl_fn = 'tpl_store_rip_imm32'
            params = (va + 10 + disp32, imm32)
        # cmp qword ptr [rip + A], 0; setne/sete al; ret (12 bytes)
        elif size == 12 and b[:3] == p_cmp_u64_rip and b[7] == 0x00 and b[8:12] in (p_setne_ret, p_sete_ret):
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_cmp_rip_u64_zero_ne' if b[8:12] == p_setne_ret else 'tpl_cmp_rip_u64_zero_eq'
            params = (va + 8 + disp32,)
        # cmp dword ptr [rip + A], 0; setne/sete al; ret (11 bytes)
        elif size == 11 and b[:2] == p_cmp_u32_rip and b[6] == 0x00 and b[7:11] in (p_setne_ret, p_sete_ret):
            disp32 = struct.unpack_from('<i', b, 2)[0]
            tpl_fn = 'tpl_cmp_rip_u32_zero_ne' if b[7:11] == p_setne_ret else 'tpl_cmp_rip_u32_zero_eq'
            params = (va + 7 + disp32,)
        # cmp byte ptr [rip + A], 0; setne/sete al; ret (11 bytes)
        elif size == 11 and b[:2] == p_cmp_u8_rip and b[6] == 0x00 and b[7:11] in (p_setne_ret, p_sete_ret):
            disp32 = struct.unpack_from('<i', b, 2)[0]
            tpl_fn = 'tpl_cmp_rip_u8_zero_ne' if b[7:11] == p_setne_ret else 'tpl_cmp_rip_u8_zero_eq'
            params = (va + 7 + disp32,)
        # vmovups xmm0, [rip + A]; vmovups [rip + B], xmm0; ret (17 bytes)
        elif size == 17 and b[:4] == p_vmovups_ld and b[8:12] == p_vmovups_st and b[16] == 0xc3:
            disp_src = struct.unpack_from('<i', b, 4)[0]
            disp_dst = struct.unpack_from('<i', b, 12)[0]
            tpl_fn = 'tpl_copy_rip_16'
            params = (va + 8 + disp_src, va + 16 + disp_dst)
        # lea rax, [rip + A]; mov [rdx], rax; mov rax, rdx; ret (14 bytes)
        elif size == 14 and b[:3] == p7_pre and b[7:14] == p_store_rdx_ret_rdx:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_lea_rip_store_rdx_ret_rdx'
            params = (va + 7 + disp32,)
        # lea rax, [rip + A]; mov [rcx], rax; mov rax, rcx; ret (14 bytes)
        elif size == 14 and b[:3] == p7_pre and b[7:14] == p_store_rcx_ret_rcx:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_lea_rip_store_rcx_ret_rcx'
            params = (va + 7 + disp32,)
        # lea rax, [rip + A]; mov [rcx], rax; ret (11 bytes)
        elif size == 11 and b[:3] == p7_pre and b[7:11] == p_store_rcx_ret:
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_lea_rip_store_rcx'
            params = (va + 7 + disp32,)
        # lea rax, [rip + V]; mov [rdx], rax; xor eax, eax; mov [rdx+8], rax; mov [rdx+16], rax; mov rax, rdx; ret (24 bytes)
        elif (
            size == 24
            and b[:3] == p7_pre
            and b[7:24] == bytes.fromhex('48890233c04889420848894210488bc2c3')
        ):
            disp32 = struct.unpack_from('<i', b, 3)[0]
            tpl_fn = 'tpl_lea_rip_init_rdx_24_ret_rdx'
            params = (va + 7 + disp32,)
        # mov rax, gs:[0x58]; mov ecx, OFF; add rcx, [rax]; jmp TARGET_VA (22 bytes)
        elif size == 22 and b[:10] == p_tls_pre and b[14:18] == p_tls_mid:
            tls_off = struct.unpack_from('<I', b, 10)[0]
            rel32 = struct.unpack_from('<i', b, 18)[0]
            tpl_fn = 'tpl_bind_tls0_jmp'
            params = (tls_off, va + 22 + rel32)
        # Template #16 (37 bytes)
        elif (
            size == 37
            and b[:9] == p16_pre
            and b[13] == 0xe8
            and b[18:21] == p16_mid1
            and b[25:28] == p16_mid2
            and b[32:37] == p16_post
        ):
            disp_a = struct.unpack_from('<i', b, 9)[0]
            rel_b = struct.unpack_from('<i', b, 14)[0]
            disp_c = struct.unpack_from('<i', b, 21)[0]
            disp_a2 = struct.unpack_from('<i', b, 28)[0]
            a_va = va + 13 + disp_a
            b_va = va + 18 + rel_b
            c_va = va + 25 + disp_c
            a2_va = va + 32 + disp_a2
            if a_va == a2_va:
                tpl_fn = 'tpl_init_static_ret_va'
                params = (a_va, b_va, c_va)
        # Template #16 with imm32 edx (40 bytes)
        elif (
            size == 40
            and b[:5] == p16_40_pre
            and b[9:12] == p3_pre
            and b[16] == 0xe8
            and b[21:24] == p16_mid1
            and b[28:31] == p16_mid2
            and b[35:40] == p16_post
        ):
            edx_imm = struct.unpack_from('<I', b, 5)[0]
            disp_a = struct.unpack_from('<i', b, 12)[0]
            rel_b = struct.unpack_from('<i', b, 17)[0]
            disp_c = struct.unpack_from('<i', b, 24)[0]
            disp_a2 = struct.unpack_from('<i', b, 31)[0]
            a_va = va + 16 + disp_a
            b_va = va + 21 + rel_b
            c_va = va + 28 + disp_c
            a2_va = va + 35 + disp_a2
            if a_va == a2_va:
                tpl_fn = 'tpl_init_static_edx_ret_va'
                params = (a_va, edx_imm, b_va, c_va)
        # Template #20 (22 bytes): 48 83 ec 28 48 8d 0d <rip_A> e8 <call_B> 90 48 83 c4 28 c3
        elif size == 22 and b[:7] == p20_pre and b[11] == 0xe8 and b[16:22] == p20_post:
            disp_a = struct.unpack_from('<i', b, 7)[0]
            rel_b = struct.unpack_from('<i', b, 12)[0]
            a_va = va + 11 + disp_a
            b_va = va + 16 + rel_b
            tpl_fn = 'tpl_call_static_rcx'
            params = (a_va, b_va)
        # 35B 2-call static rcx: sub rsp, 0x28; lea rcx, [rip+A1]; call F; nop; lea rcx, [rip+A2]; call F; nop; add rsp, 0x28; ret
        elif (
            size == 35
            and b[:7] == p20_pre
            and b[11] == 0xe8
            and b[16:20] == p20_mid
            and b[24] == 0xe8
            and b[29:35] == p20_post
        ):
            a1 = va + 11 + struct.unpack_from('<i', b, 7)[0]
            f1 = va + 16 + struct.unpack_from('<i', b, 12)[0]
            a2 = va + 24 + struct.unpack_from('<i', b, 20)[0]
            f2 = va + 29 + struct.unpack_from('<i', b, 25)[0]
            if f1 == f2:
                tpl_fn = 'tpl_call_static_rcx_2'
                params = (a1, a2, f1)
        # 48B 3-call static rcx
        elif (
            size == 48
            and b[:7] == p20_pre
            and b[11] == 0xe8
            and b[16:20] == p20_mid
            and b[24] == 0xe8
            and b[29:33] == p20_mid
            and b[37] == 0xe8
            and b[42:48] == p20_post
        ):
            a1 = va + 11 + struct.unpack_from('<i', b, 7)[0]
            f1 = va + 16 + struct.unpack_from('<i', b, 12)[0]
            a2 = va + 24 + struct.unpack_from('<i', b, 20)[0]
            f2 = va + 29 + struct.unpack_from('<i', b, 25)[0]
            a3 = va + 37 + struct.unpack_from('<i', b, 33)[0]
            f3 = va + 42 + struct.unpack_from('<i', b, 38)[0]
            if f1 == f2 == f3:
                tpl_fn = 'tpl_call_static_rcx_3'
                params = (a1, a2, a3, f1)
        # Scalar deleting destructor with RIP vtable (43 bytes)
        elif (
            size == 43
            and b[:9] == p_sdel_pre
            and b[13:25] == p_sdel_mid
            and b[29] == 0xe8
            and b[34:43] == p_sdel_post
        ):
            disp_v = struct.unpack_from('<i', b, 9)[0]
            sz_imm = struct.unpack_from('<I', b, 25)[0]
            rel_free = struct.unpack_from('<i', b, 30)[0]
            tpl_fn = 'tpl_scalar_deleting_dtor_vft'
            params = (va + 13 + disp_v, sz_imm, va + 34 + rel_free)
        # 26B: lea rax, [rip+V]; lea rcx, [rip+RCX_VA]; mov [rip+DST_VA], rax; jmp TARGET_VA
        elif (
            size == 26
            and b[:3] == p7_pre
            and b[7:10] == p3_pre
            and b[14:17] == p16_mid2
            and b[21] == 0xe9
        ):
            v_va = va + 7 + struct.unpack_from('<i', b, 3)[0]
            rcx_va = va + 14 + struct.unpack_from('<i', b, 10)[0]
            dst_va = va + 21 + struct.unpack_from('<i', b, 17)[0]
            t_va = va + 26 + struct.unpack_from('<i', b, 22)[0]
            tpl_fn = 'tpl_store_rip_va_bind_rcx_jmp'
            params = (v_va, dst_va, rcx_va, t_va)
        # 58B FNV-1a compile-time/static string hash initializer
        elif (
            size == 58
            and b[:8] == bytes.fromhex('bac59d1c814c8d05')
            and b[12:15] == p7_pre
            and b[19:53] == bytes.fromhex('0f1f4000660f1f8400000000000fbe0848ffc033ca69d193010001493bc075ed8915')
            and b[57] == 0xc3
        ):
            end_va = va + 12 + struct.unpack_from('<i', b, 8)[0]
            beg_va = va + 19 + struct.unpack_from('<i', b, 15)[0]
            dst_va = va + 57 + struct.unpack_from('<i', b, 53)[0]
            tpl_fn = 'tpl_fnv1a_range_store_rip_u32'
            params = (beg_va, end_va, dst_va)
        # 21B: mov eax, [rip+CNT]; inc eax; mov [rip+CNT], eax; mov [rip+DST], eax; ret
        elif (
            size == 21
            and b[:2] == p9_32_pre
            and b[6:10] == bytes.fromhex('ffc08905')
            and b[14:16] == bytes.fromhex('8905')
            and b[20] == 0xc3
        ):
            cnt1 = va + 6 + struct.unpack_from('<i', b, 2)[0]
            cnt2 = va + 14 + struct.unpack_from('<i', b, 10)[0]
            dst = va + 20 + struct.unpack_from('<i', b, 16)[0]
            if cnt1 == cnt2:
                tpl_fn = 'tpl_inc_rip_u32_store_rip_u32'
                params = (cnt1, dst)
        # 134B: shader/resource registration helper (1,346 functions)
        elif (
            size == 134
            and b[:16] == p134_pre
            and b[20:26] == p134_m1
            and b[30:58] == p134_m2
            and b[62:72] == p134_m3
            and b[76:88] == p134_m4
            and b[92:115] == p134_m5
            and b[119:134] == p134_post
        ):
            u16_va = va + 20 + struct.unpack_from('<i', b, 16)[0]
            init_fn = va + 30 + struct.unpack_from('<i', b, 26)[0]
            f2 = va + 62 + struct.unpack_from('<i', b, 58)[0]
            f3 = va + 76 + struct.unpack_from('<i', b, 72)[0]
            f4 = va + 92 + struct.unpack_from('<i', b, 88)[0]
            f5 = va + 119 + struct.unpack_from('<i', b, 115)[0]
            if (f2, f3, f4, f5) == (0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x14241c190):
                tpl_fn = 'tpl_reg_entry_134'
                params = (u16_va, init_fn)
        # 75B: ObjectStream scope push + pop (`rbx = r8`, f1/f2 imm32)
        elif (
            size == 75
            and b[:16] == bytes.fromhex('40534883ec40498bd8c7442428')
            and b[20:23] == p_lea_r8_rip[:2] + b'\x05'
            and b[27:34] == bytes.fromhex('488bcbc7442420')
            and b[38] == 0xe8
            and b[43:75] in (p75_post, p80a_post)
        ):
            f2 = struct.unpack_from('<I', b, 16)[0]
            r8_ptr_va = va + 24 + struct.unpack_from('<i', b, 20)[0]
            f1 = struct.unpack_from('<I', b, 34)[0]
            push_fn = va + 40 + struct.unpack_from('<i', b, 36)[0]
            tpl_fn = 'tpl_stream_push_pop_r8'
            params = (r8_ptr_va, f1, f2, push_fn)
        # 69B: ObjectStream scope push + pop (`rbx = r8`, flag=0)
        elif size == 69 and b[:14] == p69_pre and b[18:30] == p69_m1 and b[34:69] in (p75_post, p80a_post):
            r8_ptr_va = va + 18 + struct.unpack_from('<i', b, 14)[0]
            push_fn = va + 34 + struct.unpack_from('<i', b, 30)[0]
            tpl_fn = 'tpl_stream_push_pop_r8'
            params = (r8_ptr_va, 0, 0, push_fn)
        # 80B (a): ObjectStream scope push + pop (`rbx = rdx, rdx = rcx`, f1/f2 imm32)
        elif (
            size == 80
            and b[:9] == p80a_pre
            and b[13:23] == bytes.fromhex('488bda488bd1c7442428')
            and b[27:34] == bytes.fromhex('488bcbc7442420')
            and b[38] == 0xe8
            and b[43:80] in (p75_post, p80a_post)
        ):
            r8_ptr_va = va + 13 + struct.unpack_from('<i', b, 9)[0]
            f2 = struct.unpack_from('<I', b, 23)[0]
            f1 = struct.unpack_from('<I', b, 34)[0]
            push_fn = va + 43 + struct.unpack_from('<i', b, 39)[0]
            tpl_fn = 'tpl_stream_push_pop_rdx'
            params = (r8_ptr_va, f1, f2, push_fn)
        # 74B: ObjectStream scope push + pop (`rbx = rdx, rdx = rcx`, flag=0)
        elif size == 74 and b[:9] == p74_pre and b[13:33] == p74_m1 and b[37:74] in (p75_post, p80a_post):
            r8_ptr_va = va + 13 + struct.unpack_from('<i', b, 9)[0]
            push_fn = va + 37 + struct.unpack_from('<i', b, 33)[0]
            tpl_fn = 'tpl_stream_push_pop_rdx'
            params = (r8_ptr_va, 0, 0, push_fn)
        # 83B: ObjectStream begin + body + end (`rdi = r8, rbx = rdx`, f1/f2 imm32)
        elif (
            size == 83
            and b[:13] == p83_pre
            and b[17:23] == p83_m1
            and b[27:34] == bytes.fromhex('488bcfc7442428')
            and b[38:45] == bytes.fromhex('488bdac7442420')
            and b[49] == 0xe8
            and b[54:61] == p83_m3
            and b[65:79] == p83_m4
        ):
            r9d_va = va + 17 + struct.unpack_from('<i', b, 13)[0]
            r8_va = va + 27 + struct.unpack_from('<i', b, 23)[0]
            f2 = struct.unpack_from('<I', b, 34)[0]
            f1 = struct.unpack_from('<I', b, 45)[0]
            begin_fn = va + 54 + struct.unpack_from('<i', b, 50)[0]
            body_fn = va + 65 + struct.unpack_from('<i', b, 61)[0]
            end_fn = va + 83 + struct.unpack_from('<i', b, 79)[0]
            tpl_fn = 'tpl_stream_begin_body_end_r8'
            params = (r9d_va, r8_va, f1, f2, begin_fn, body_fn, end_fn)
        # 77B: ObjectStream begin + body + end (`rdi = r8, rbx = rdx`, flag=0)
        elif (
            size == 77
            and b[:13] == p83_pre
            and b[17:23] == p83_m1
            and b[27:44] == p77_m2
            and b[48:55] == p83_m3
            and b[59:73] == p83_m4
        ):
            r9d_va = va + 17 + struct.unpack_from('<i', b, 13)[0]
            r8_va = va + 27 + struct.unpack_from('<i', b, 23)[0]
            begin_fn = va + 48 + struct.unpack_from('<i', b, 44)[0]
            body_fn = va + 59 + struct.unpack_from('<i', b, 55)[0]
            end_fn = va + 77 + struct.unpack_from('<i', b, 73)[0]
            tpl_fn = 'tpl_stream_begin_body_end_r8'
            params = (r9d_va, r8_va, 0, 0, begin_fn, body_fn, end_fn)
        # 86B: ObjectStream begin + body + end (`rdi = rdx, rbx = rcx`, f1/f2 imm32)
        elif (
            size == 86
            and b[:13] == p83_pre
            and b[17:23] == p86_m1
            and b[27:37] == bytes.fromhex('488bd1488bd9c7442428')
            and b[41:48] == bytes.fromhex('488bcfc7442420')
            and b[52] == 0xe8
            and b[57:64] == p83_m3
            and b[68:82] == p83_m4
        ):
            r9d_va = va + 17 + struct.unpack_from('<i', b, 13)[0]
            r8_va = va + 27 + struct.unpack_from('<i', b, 23)[0]
            f2 = struct.unpack_from('<I', b, 37)[0]
            f1 = struct.unpack_from('<I', b, 48)[0]
            begin_fn = va + 57 + struct.unpack_from('<i', b, 53)[0]
            body_fn = va + 68 + struct.unpack_from('<i', b, 64)[0]
            end_fn = va + 86 + struct.unpack_from('<i', b, 82)[0]
            tpl_fn = 'tpl_stream_begin_body_end_rdx'
            params = (r9d_va, r8_va, f1, f2, begin_fn, body_fn, end_fn)
        # 80B (b): ObjectStream begin + body + end (`rdi = rdx, rbx = rcx`, flag=0)
        elif (
            size == 80
            and b[:13] == p83_pre
            and b[17:23] == p86_m1
            and b[27:47] == p80b_m2
            and b[51:58] == p83_m3
            and b[62:76] == p83_m4
        ):
            r9d_va = va + 17 + struct.unpack_from('<i', b, 13)[0]
            r8_va = va + 27 + struct.unpack_from('<i', b, 23)[0]
            begin_fn = va + 51 + struct.unpack_from('<i', b, 47)[0]
            body_fn = va + 62 + struct.unpack_from('<i', b, 58)[0]
            end_fn = va + 80 + struct.unpack_from('<i', b, 76)[0]
            tpl_fn = 'tpl_stream_begin_body_end_rdx'
            params = (r9d_va, r8_va, 0, 0, begin_fn, body_fn, end_fn)
        # 60B: ObjectStream begin + end (`rbx = r8`, f1/f2 imm32)
        elif (
            size == 60
            and b[:9] == p60_pre
            and b[13:19] == p60_m1
            and b[23:30] == bytes.fromhex('488bcbc7442428')
            and b[34:38] == bytes.fromhex('c7442420')
            and b[42] == 0xe8
            and b[47:56] == p60_m3
        ):
            r9d_va = va + 13 + struct.unpack_from('<i', b, 9)[0]
            r8_va = va + 23 + struct.unpack_from('<i', b, 19)[0]
            f2 = struct.unpack_from('<I', b, 30)[0]
            f1 = struct.unpack_from('<I', b, 38)[0]
            begin_fn = va + 47 + struct.unpack_from('<i', b, 43)[0]
            end_fn = va + 60 + struct.unpack_from('<i', b, 56)[0]
            tpl_fn = 'tpl_stream_begin_end_r8'
            params = (r9d_va, r8_va, f1, f2, begin_fn, end_fn)
        # 54B: ObjectStream begin + end (`rbx = r8`, flag=0)
        elif (
            size == 54
            and b[:9] == p60_pre
            and b[13:19] == p60_m1
            and b[23:37] == p54_m2
            and b[41:50] == p60_m3
        ):
            r9d_va = va + 13 + struct.unpack_from('<i', b, 9)[0]
            r8_va = va + 23 + struct.unpack_from('<i', b, 19)[0]
            begin_fn = va + 41 + struct.unpack_from('<i', b, 37)[0]
            end_fn = va + 54 + struct.unpack_from('<i', b, 50)[0]
            tpl_fn = 'tpl_stream_begin_end_r8'
            params = (r9d_va, r8_va, 0, 0, begin_fn, end_fn)
        # 63B: ObjectStream begin + end (`rbx = rdx, rdx = rcx`, f1/f2 imm32)
        elif (
            size == 63
            and b[:9] == p60_pre
            and b[13:19] == p63_m1
            and b[23:33] == bytes.fromhex('488bd1488bcbc7442428')
            and b[37:41] == bytes.fromhex('c7442420')
            and b[45] == 0xe8
            and b[50:59] == p60_m3
        ):
            r9d_va = va + 13 + struct.unpack_from('<i', b, 9)[0]
            r8_va = va + 23 + struct.unpack_from('<i', b, 19)[0]
            f2 = struct.unpack_from('<I', b, 33)[0]
            f1 = struct.unpack_from('<I', b, 41)[0]
            begin_fn = va + 50 + struct.unpack_from('<i', b, 46)[0]
            end_fn = va + 63 + struct.unpack_from('<i', b, 59)[0]
            tpl_fn = 'tpl_stream_begin_end_rdx'
            params = (r9d_va, r8_va, f1, f2, begin_fn, end_fn)
        # 57B: ObjectStream begin + end (`rbx = rdx, rdx = rcx`, flag=0)
        elif (
            size == 57
            and b[:9] == p60_pre
            and b[13:19] == p63_m1
            and b[23:40] == p57_m2
            and b[44:53] == p60_m3
        ):
            r9d_va = va + 13 + struct.unpack_from('<i', b, 9)[0]
            r8_va = va + 23 + struct.unpack_from('<i', b, 19)[0]
            begin_fn = va + 44 + struct.unpack_from('<i', b, 40)[0]
            end_fn = va + 57 + struct.unpack_from('<i', b, 53)[0]
            tpl_fn = 'tpl_stream_begin_end_rdx'
            params = (r9d_va, r8_va, 0, 0, begin_fn, end_fn)
        # 103B: ObjectStream push + body + inline pop + ret 0 (`rdi = rdx, rbx = rcx`, flag=1)
        elif (
            size == 103
            and b[:13] == p103_pre
            and b[17:46] == p103_m1
            and b[50:57] == p83_m3
            and b[61:103] == p103_post
        ):
            r8_va = va + 17 + struct.unpack_from('<i', b, 13)[0]
            push_fn = va + 50 + struct.unpack_from('<i', b, 46)[0]
            body_fn = va + 61 + struct.unpack_from('<i', b, 57)[0]
            tpl_fn = 'tpl_stream_push_body_pop_rdx'
            params = (r8_va, 1, push_fn, body_fn)
        # 97B: ObjectStream push + body + inline pop + ret 0 (`rdi = rdx, rbx = rcx`, flag=0)
        elif (
            size == 97
            and b[:13] == p103_pre
            and b[17:40] == p97_m1
            and b[44:51] == p83_m3
            and b[55:97] == p103_post
        ):
            r8_va = va + 17 + struct.unpack_from('<i', b, 13)[0]
            push_fn = va + 44 + struct.unpack_from('<i', b, 40)[0]
            body_fn = va + 55 + struct.unpack_from('<i', b, 51)[0]
            tpl_fn = 'tpl_stream_push_body_pop_rdx'
            params = (r8_va, 0, push_fn, body_fn)
        # 148B: ObjectStream 2x push + 2x inline pop + ret 0 (`rdi = rdx, rbx = rcx`, flag=1)
        elif (
            size == 148
            and b[:13] == p103_pre
            and b[17:46] == p103_m1
            and b[50:53] == p148_m2
            and b[57:80] == p148_m3
            and b[84:148] == p148_post
        ):
            r8_va1 = va + 17 + struct.unpack_from('<i', b, 13)[0]
            f1 = va + 50 + struct.unpack_from('<i', b, 46)[0]
            r8_va2 = va + 57 + struct.unpack_from('<i', b, 53)[0]
            f2 = va + 84 + struct.unpack_from('<i', b, 80)[0]
            if f1 == f2:
                tpl_fn = 'tpl_stream_push2_pop2_rdx'
                params = (r8_va1, r8_va2, f1)
        # 127B: ObjectStream 2x begin + 2x end + ret 0 (`rdi = rdx, rbx = rcx`, flag=1)
        elif (
            size == 127
            and b[:13] == p83_pre
            and b[17:23] == p86_m1
            and b[27:53] == p86_m2
            and b[57:60] == p127_m3
            and b[64:70] == p127_m4
            and b[74:94] == p127_m5
            and b[98:102] == p127_m6
            and b[106:110] == p127_m6
            and b[114:127] == p127_post
        ):
            r9d_1 = va + 17 + struct.unpack_from('<i', b, 13)[0]
            r8_1 = va + 27 + struct.unpack_from('<i', b, 23)[0]
            f1 = va + 57 + struct.unpack_from('<i', b, 53)[0]
            r9d_2 = va + 64 + struct.unpack_from('<i', b, 60)[0]
            r8_2 = va + 74 + struct.unpack_from('<i', b, 70)[0]
            f2 = va + 98 + struct.unpack_from('<i', b, 94)[0]
            g1 = va + 106 + struct.unpack_from('<i', b, 102)[0]
            g2 = va + 114 + struct.unpack_from('<i', b, 110)[0]
            if f1 == f2 and g1 == g2:
                tpl_fn = 'tpl_stream_begin2_end2_rdx'
                params = (r9d_1, r8_1, r9d_2, r8_2, f1, g1)
        # 138B: ObjectStream 2x begin + body + 2x end + ret 0 (`rdi = rdx, rbx = rcx`, flag=1)
        elif (
            size == 138
            and b[:13] == p83_pre
            and b[17:23] == p86_m1
            and b[27:53] == p86_m2
            and b[57:60] == p127_m3
            and b[64:70] == p127_m4
            and b[74:94] == p127_m5
            and b[98:105] == p83_m3
            and b[109:113] == p127_m6
            and b[117:121] == p127_m6
            and b[125:138] == p127_post
        ):
            r9d_1 = va + 17 + struct.unpack_from('<i', b, 13)[0]
            r8_1 = va + 27 + struct.unpack_from('<i', b, 23)[0]
            f1 = va + 57 + struct.unpack_from('<i', b, 53)[0]
            r9d_2 = va + 64 + struct.unpack_from('<i', b, 60)[0]
            r8_2 = va + 74 + struct.unpack_from('<i', b, 70)[0]
            f2 = va + 98 + struct.unpack_from('<i', b, 94)[0]
            body_fn = va + 109 + struct.unpack_from('<i', b, 105)[0]
            g1 = va + 117 + struct.unpack_from('<i', b, 113)[0]
            g2 = va + 125 + struct.unpack_from('<i', b, 121)[0]
            if f1 == f2 and g1 == g2:
                tpl_fn = 'tpl_stream_begin2_body_end2_rdx'
                params = (r9d_1, r8_1, r9d_2, r8_2, f1, body_fn, g1)
        # 97B static 3-entry table init + atexit
        elif (
            size == 97
            and b[:7] == bytes.fromhex('4883ec58488d05')
            and b[11:32] == bytes.fromhex('c7442428000000004889442420488d542420488d05')
            and b[36:52] == bytes.fromhex('c7442438010000004889442430488d0d')
            and b[56:59] == p7_pre
            and b[63:77] == bytes.fromhex('c7442448020000004889442440e8')
            and b[81:84] == p3_pre
            and b[88:93] == bytes.fromhex('4883c458e9')
        ):
            s0 = va + 11 + struct.unpack_from('<i', b, 7)[0]
            s1 = va + 36 + struct.unpack_from('<i', b, 32)[0]
            tbl = va + 56 + struct.unpack_from('<i', b, 52)[0]
            s2 = va + 63 + struct.unpack_from('<i', b, 59)[0]
            init_fn = va + 81 + struct.unpack_from('<i', b, 77)[0]
            dtor_fn = va + 88 + struct.unpack_from('<i', b, 84)[0]
            atexit_fn = va + 97 + struct.unpack_from('<i', b, 93)[0]
            if init_fn == 0x1401de5b0 and atexit_fn == 0x141834d18:
                tpl_fn = 'tpl_init_enum3_atexit'
                params = (tbl, s0, s1, s2, dtor_fn)
        # 234B static linked-list node registration with thread-safe init guard (3,197 functions)
        elif (
            size == 234
            and b[:26] == bytes.fromhex('4883ec3865488b042558000000b974000000488b008b04013905')
            and b[30:41] == bytes.fromhex('0f8f8d00000048c7442428')
            and b[45:48] == p7_pre
            and b[52:60] == bytes.fromhex('4889442420488d05')
            and b[64:78] == bytes.fromhex('c5f8104424204889442420488b05')
            and b[82:85] == p16_mid2
            and b[89:92] == p7_pre
            and b[96:99] == p16_mid2
            and b[103:106] == p7_pre
            and b[110:113] == p16_mid2
            and b[117:120] == p7_pre
            and b[124:128] == p_vmovups_st
            and b[132:137] == bytes.fromhex('48c7442428')
            and b[141:150] == bytes.fromhex('c5f810442420488905')
            and b[154:158] == p_vmovups_st
            and b[162:164] == p_store_imm32_rip
            and b[168:180] == bytes.fromhex('ffffffff4883c438c3488d0d')
            and b[184] == 0xe8
            and b[189:191] == p_cmp_u32_rip
            and b[195:210] == bytes.fromhex('ff0f855affffffc5f9efc0c5fe7f05')
            and b[214:217] == p3_pre
            and b[221:225] == bytes.fromhex('c5f877e8')
            and b[229:234] == bytes.fromhex('e93affffff')
        ):
            guard_va = va + 30 + struct.unpack_from('<i', b, 26)[0]
            len1 = struct.unpack_from('<I', b, 41)[0]
            str1_va = va + 52 + struct.unpack_from('<i', b, 48)[0]
            str2_va = va + 64 + struct.unpack_from('<i', b, 60)[0]
            node_va = va + 89 + struct.unpack_from('<i', b, 85)[0]
            id_va = va + 96 + struct.unpack_from('<i', b, 92)[0]
            fn_va = va + 110 + struct.unpack_from('<i', b, 106)[0]
            len2 = struct.unpack_from('<I', b, 137)[0]
            tpl_fn = 'tpl_reg_static_node_234'
            params = (guard_va, node_va, id_va, fn_va, str1_va, len1, str2_va, len2)
        # 59B TLS bitmask clear (2,286 functions)
        elif (
            size == 59
            and b[:6] == bytes.fromhex('4055488beab8')
            and b[10:31] == bytes.fromhex('654c8b0425580000004d8b004c03c0458b004183e0')
            and b[32:42] == bytes.fromhex('65488b142558000000b8')
            and b[46:59] == bytes.fromhex('8bc8488b024803c14489005dc3')
        ):
            off1 = struct.unpack_from('<I', b, 6)[0]
            mask = b[31]
            off2 = struct.unpack_from('<I', b, 42)[0]
            if off1 == off2:
                tpl_fn = 'tpl_tls_clear_bit_59'
                params = (off1, mask)
        # 44B TLS lazy zero init (88 functions)
        elif (
            size == 44
            and b[:10] == bytes.fromhex('65488b042558000000ba')
            and b[14:21] == bytes.fromhex('488b088b040a')
            and b[21] == 0xa8
            and b[23:26] == b'\x75\x13\x0c'
            and b[27:31] == bytes.fromhex('89040ab8')
            and b[35:44] == bytes.fromhex('48c7040800000000c3')
            and b[22] == b[26]
        ):
            flag_off = struct.unpack_from('<I', b, 10)[0]
            bit = b[22]
            val_off = struct.unpack_from('<I', b, 31)[0]
            tpl_fn = 'tpl_tls_lazy_zero_44'
            params = (flag_off, bit, val_off)
        # 29B TLS store RIP address (49 functions)
        elif (
            size == 29
            and b[:10] == bytes.fromhex('65488b042558000000ba')
            and b[14:20] == bytes.fromhex('488b08488d05')
            and b[24:29] == bytes.fromhex('4889040ac3')
        ):
            tls_off = struct.unpack_from('<I', b, 10)[0]
            target_va = va + 24 + struct.unpack_from('<i', b, 20)[0]
            tpl_fn = 'tpl_tls_store_rip_29'
            params = (tls_off, target_va)
        # 104B thread-safe static init + atexit + store rcx (792 functions)
        elif (
            size == 104
            and b[:31] == bytes.fromhex('40534883ec2065488b042558000000488bd9b974000000488b008b04013905')
            and b[35:40] == bytes.fromhex('7f1448891d')
            and b[44:47] == p7_pre
            and b[51:60] == bytes.fromhex('4883c4205bc3488d0d')
            and b[64] == 0xe8
            and b[69:71] == p_cmp_u32_rip
            and b[75:81] == bytes.fromhex('ff75d7488d0d')
            and b[85] == 0xe8
            and b[90:93] == p3_pre
            and b[97] == 0xe8
            and b[102:104] == b'\xeb\xbd'
        ):
            guard_va = va + 35 + struct.unpack_from('<i', b, 31)[0]
            ret_va = va + 51 + struct.unpack_from('<i', b, 47)[0]
            dtor_va = va + 85 + struct.unpack_from('<i', b, 81)[0]
            tpl_fn = 'tpl_init_static_atexit_104'
            params = (guard_va, ret_va, dtor_va)

        if tpl_fn is None:
            tpl_fn, params = match_phase4_complex(va, size, b)

        if tpl_fn is None:
            continue

        eh = fast_eh_info(pe, va)
        mask_direct = can_throw_map.get(va, 0)
        if eh not in ('none', 'noexcept') and not (eh == 'cleanup' and (mask_direct & 21) == 0):
            continue

        counts[tpl_fn] = counts.get(tpl_fn, 0) + 1
        fn_name = name if name else f'sub_{va:x}'
        fn_name_escaped = fn_name.replace('\\', '\\\\').replace('"', '\\"')
        throws_discard = (mask_direct & 23) != 0
        entries.append((va, fn_name_escaped, tpl_fn, params, throws_discard))

    # Re-scan all sibling shard files right before writing templates_c.rs to ensure 0 duplicate VAs
    latest_excluded = load_excluded_vas()
    entries = [e for e in entries if e[0] not in latest_excluded]

    lines = [
        '//! Shard C: RIP-relative global/TypeInfo getters & call-wrapper templates.',
        '//! Generated by `tools/decomp_scale/gen_shard_c.py`.',
        '',
        'use crate::registry::{Replacement, Throws};',
        '',
        '#[inline(always)]',
        'fn exe_va(va: u64) -> usize {',
        '    crate::exe::ptr::<u8>(va) as usize',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn stream_pop_inline(s: usize) {',
        '    let d = unsafe { crate::exe::rd::<u32>(s + 8) };',
        '    let arr = unsafe { crate::exe::rd::<usize>(s) };',
        '    let prev = unsafe { crate::exe::rd::<u32>(arr + (d as usize) * 48 - 8) };',
        '    unsafe {',
        '        crate::exe::wr::<u32>(s + 8, d.wrapping_sub(1));',
        '        crate::exe::wr::<u32>(s + 0x328, prev);',
        '    }',
        '}',
        '',
        '/// Template #7 (8 bytes): `lea rax, [rip + disp32]; ret`',
        'pub unsafe extern "win64" fn tpl_lea_rip_ret<const GLOBAL_VA: u64>() -> usize {',
        '    exe_va(GLOBAL_VA)',
        '}',
        '',
        '/// Template #9 (8 bytes): `mov rax, qword ptr [rip + disp32]; ret`',
        'pub unsafe extern "win64" fn tpl_load_rip_u64<const GLOBAL_VA: u64>() -> u64 {',
        '    unsafe { crate::exe::rd::<u64>(exe_va(GLOBAL_VA)) }',
        '}',
        '',
        '/// Template #9 32-bit (7 bytes): `mov eax, dword ptr [rip + disp32]; ret`',
        'pub unsafe extern "win64" fn tpl_load_rip_u32<const GLOBAL_VA: u64>() -> u32 {',
        '    unsafe { crate::exe::rd::<u32>(exe_va(GLOBAL_VA)) }',
        '}',
        '',
        '/// Template #9 16-bit (8 bytes): `movzx eax, word ptr [rip + disp32]; ret`',
        'pub unsafe extern "win64" fn tpl_load_rip_u16<const GLOBAL_VA: u64>() -> u16 {',
        '    unsafe { crate::exe::rd::<u16>(exe_va(GLOBAL_VA)) }',
        '}',
        '',
        '/// Template #9 8-bit (8 bytes): `movzx eax, byte ptr [rip + disp32]; ret`',
        'pub unsafe extern "win64" fn tpl_load_rip_u8<const GLOBAL_VA: u64>() -> u8 {',
        '    unsafe { crate::exe::rd::<u8>(exe_va(GLOBAL_VA)) }',
        '}',
        '',
        '/// Template #3 (12 bytes): `lea rcx, [rip + disp32]; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_bind_rcx_jmp<const RCX_VA: u64, const TARGET_VA: u64>(',
        '    _rcx: u64,',
        '    rdx: u64,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(exe_va(RCX_VA) as u64, rdx, r8, r9) }',
        '}',
        '',
        '/// `mov rcx, [rip + disp32]; jmp TARGET_VA` (12 bytes)',
        'pub unsafe extern "win64" fn tpl_load_rcx_rip_jmp<const RCX_PTR_VA: u64, const TARGET_VA: u64>(',
        '    _rcx: u64,',
        '    rdx: u64,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let rcx = unsafe { crate::exe::rd::<u64>(exe_va(RCX_PTR_VA)) };',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, r8, r9) }',
        '}',
        '',
        '/// `lea rdx, [rip + disp32]; jmp TARGET_VA` (12 bytes)',
        'pub unsafe extern "win64" fn tpl_bind_rdx_jmp<const RDX_VA: u64, const TARGET_VA: u64>(',
        '    rcx: u64,',
        '    _rdx: u64,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, exe_va(RDX_VA) as u64, r8, r9) }',
        '}',
        '',
        '/// `lea r8, [rip + disp32]; jmp TARGET_VA` (12 bytes)',
        'pub unsafe extern "win64" fn tpl_bind_r8_jmp<const R8_VA: u64, const TARGET_VA: u64>(',
        '    rcx: u64,',
        '    rdx: u64,',
        '    _r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, exe_va(R8_VA) as u64, r9) }',
        '}',
        '',
        '/// `lea r9, [rip + disp32]; jmp TARGET_VA` (12 bytes)',
        'pub unsafe extern "win64" fn tpl_bind_r9_jmp<const R9_VA: u64, const TARGET_VA: u64>(',
        '    rcx: u64,',
        '    rdx: u64,',
        '    r8: u64,',
        '    _r9: u64,',
        ') -> u64 {',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, r8, exe_va(R9_VA) as u64) }',
        '}',
        '',
        '/// `mov [rip + disp32], rcx; ret` (8 bytes)',
        'pub unsafe extern "win64" fn tpl_store_rip_u64<const GLOBAL_VA: u64>(rcx: u64) {',
        '    unsafe { crate::exe::wr::<u64>(exe_va(GLOBAL_VA), rcx) }',
        '}',
        '',
        '/// `mov [rip + disp32], ecx; ret` (7 bytes)',
        'pub unsafe extern "win64" fn tpl_store_rip_u32<const GLOBAL_VA: u64>(ecx: u32) {',
        '    unsafe { crate::exe::wr::<u32>(exe_va(GLOBAL_VA), ecx) }',
        '}',
        '',
        '/// `mov [rip + disp32], cl; ret` (7 bytes)',
        'pub unsafe extern "win64" fn tpl_store_rip_u8<const GLOBAL_VA: u64>(cl: u8) {',
        '    unsafe { crate::exe::wr::<u8>(exe_va(GLOBAL_VA), cl) }',
        '}',
        '',
        '/// `mov byte ptr [rip + disp32], imm8; ret` (8 bytes)',
        'pub unsafe extern "win64" fn tpl_store_rip_imm8<const GLOBAL_VA: u64, const IMM: u8>() {',
        '    unsafe { crate::exe::wr::<u8>(exe_va(GLOBAL_VA), IMM) }',
        '}',
        '',
        '/// `mov dword ptr [rip + disp32], imm32; ret` (11 bytes)',
        'pub unsafe extern "win64" fn tpl_store_rip_imm32<const GLOBAL_VA: u64, const IMM: u32>() {',
        '    unsafe { crate::exe::wr::<u32>(exe_va(GLOBAL_VA), IMM) }',
        '}',
        '',
        '/// `cmp qword ptr [rip + disp32], 0; setne al; ret` (12 bytes)',
        'pub unsafe extern "win64" fn tpl_cmp_rip_u64_zero_ne<const GLOBAL_VA: u64>() -> u8 {',
        '    unsafe { (crate::exe::rd::<u64>(exe_va(GLOBAL_VA)) != 0) as u8 }',
        '}',
        '',
        '/// `cmp qword ptr [rip + disp32], 0; sete al; ret` (12 bytes)',
        'pub unsafe extern "win64" fn tpl_cmp_rip_u64_zero_eq<const GLOBAL_VA: u64>() -> u8 {',
        '    unsafe { (crate::exe::rd::<u64>(exe_va(GLOBAL_VA)) == 0) as u8 }',
        '}',
        '',
        '/// `cmp dword ptr [rip + disp32], 0; setne al; ret` (11 bytes)',
        'pub unsafe extern "win64" fn tpl_cmp_rip_u32_zero_ne<const GLOBAL_VA: u64>() -> u8 {',
        '    unsafe { (crate::exe::rd::<u32>(exe_va(GLOBAL_VA)) != 0) as u8 }',
        '}',
        '',
        '/// `cmp dword ptr [rip + disp32], 0; sete al; ret` (11 bytes)',
        'pub unsafe extern "win64" fn tpl_cmp_rip_u32_zero_eq<const GLOBAL_VA: u64>() -> u8 {',
        '    unsafe { (crate::exe::rd::<u32>(exe_va(GLOBAL_VA)) == 0) as u8 }',
        '}',
        '',
        '/// `cmp byte ptr [rip + disp32], 0; setne al; ret` (11 bytes)',
        'pub unsafe extern "win64" fn tpl_cmp_rip_u8_zero_ne<const GLOBAL_VA: u64>() -> u8 {',
        '    unsafe { (crate::exe::rd::<u8>(exe_va(GLOBAL_VA)) != 0) as u8 }',
        '}',
        '',
        '/// `cmp byte ptr [rip + disp32], 0; sete al; ret` (11 bytes)',
        'pub unsafe extern "win64" fn tpl_cmp_rip_u8_zero_eq<const GLOBAL_VA: u64>() -> u8 {',
        '    unsafe { (crate::exe::rd::<u8>(exe_va(GLOBAL_VA)) == 0) as u8 }',
        '}',
        '',
        '/// `vmovups xmm0, [rip + SRC]; vmovups [rip + DST], xmm0; ret` (17 bytes)',
        'pub unsafe extern "win64" fn tpl_copy_rip_16<const SRC_VA: u64, const DST_VA: u64>() {',
        '    unsafe {',
        '        let v = crate::exe::rd::<[u8; 16]>(exe_va(SRC_VA));',
        '        crate::exe::wr::<[u8; 16]>(exe_va(DST_VA), v);',
        '    }',
        '}',
        '',
        '/// `lea rax, [rip + A]; mov [rdx], rax; mov rax, rdx; ret` (14 bytes)',
        'pub unsafe extern "win64" fn tpl_lea_rip_store_rdx_ret_rdx<const GLOBAL_VA: u64>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        ') -> usize {',
        '    unsafe { crate::exe::wr::<usize>(rdx, exe_va(GLOBAL_VA)) };',
        '    rdx',
        '}',
        '',
        '/// `lea rax, [rip + A]; mov [rcx], rax; mov rax, rcx; ret` (14 bytes)',
        'pub unsafe extern "win64" fn tpl_lea_rip_store_rcx_ret_rcx<const GLOBAL_VA: u64>(rcx: usize) -> usize {',
        '    unsafe { crate::exe::wr::<usize>(rcx, exe_va(GLOBAL_VA)) };',
        '    rcx',
        '}',
        '',
        '/// `lea rax, [rip + A]; mov [rcx], rax; ret` (11 bytes)',
        'pub unsafe extern "win64" fn tpl_lea_rip_store_rcx<const GLOBAL_VA: u64>(rcx: usize) -> usize {',
        '    let v = exe_va(GLOBAL_VA);',
        '    unsafe { crate::exe::wr::<usize>(rcx, v) };',
        '    v',
        '}',
        '',
        '/// `lea rax, [rip + V]; mov [rdx], rax; xor eax, eax; mov [rdx+8], rax; mov [rdx+16], rax; mov rax, rdx; ret` (24 bytes)',
        'pub unsafe extern "win64" fn tpl_lea_rip_init_rdx_24_ret_rdx<const VFT_VA: u64>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        crate::exe::wr::<usize>(rdx, exe_va(VFT_VA));',
        '        crate::exe::wr::<u64>(rdx + 8, 0);',
        '        crate::exe::wr::<u64>(rdx + 16, 0);',
        '    }',
        '    rdx',
        '}',
        '',
        '/// `mov rax, gs:[0x58]; mov ecx, OFF; add rcx, [rax]; jmp TARGET_VA` (22 bytes)',
        'pub unsafe extern "win64" fn tpl_bind_tls0_jmp<const TLS_OFF: usize, const TARGET_VA: u64>(',
        '    _rcx: u64,',
        '    rdx: u64,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    let tls0 = unsafe { crate::exe::rd::<usize>(crate::exe::tls_pointer()) };',
        '    let rcx = tls0.wrapping_add(TLS_OFF) as u64;',
        '    let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 =',
        '        unsafe { crate::exe::func(TARGET_VA as u64) };',
        '    unsafe { f(rcx, rdx, r8, r9) }',
        '}',
        '',
        '/// Template #16 (37 bytes): `sub rsp, 0x28; xor edx, edx; lea rcx, [rip + A]; call B; lea rax, [rip + C]; mov [rip + A], rax; add rsp, 0x28; ret`',
        'pub unsafe extern "win64" fn tpl_init_static_ret_va<const A_VA: u64, const B_VA: u64, const C_VA: u64>() -> usize {',
        '    let a = exe_va(A_VA);',
        '    let f: unsafe extern "win64" fn(usize, u32) = unsafe { crate::exe::func(B_VA as u64) };',
        '    unsafe {',
        '        f(a, 0);',
        '        let c = exe_va(C_VA);',
        '        crate::exe::wr::<usize>(a, c);',
        '        c',
        '    }',
        '}',
        '',
        '/// Template #16 w/ edx imm (40 bytes): `sub rsp, 0x28; mov edx, EDX; lea rcx, [rip + A]; call B; lea rax, [rip + C]; mov [rip + A], rax; add rsp, 0x28; ret`',
        'pub unsafe extern "win64" fn tpl_init_static_edx_ret_va<',
        '    const A_VA: u64,',
        '    const EDX: u32,',
        '    const B_VA: u64,',
        '    const C_VA: u64,',
        '>() -> usize {',
        '    let a = exe_va(A_VA);',
        '    let f: unsafe extern "win64" fn(usize, u32) = unsafe { crate::exe::func(B_VA as u64) };',
        '    unsafe {',
        '        f(a, EDX);',
        '        let c = exe_va(C_VA);',
        '        crate::exe::wr::<usize>(a, c);',
        '        c',
        '    }',
        '}',
        '',
        '/// Template #20 (22 bytes): `sub rsp, 0x28; lea rcx, [rip + A]; call B; nop; add rsp, 0x28; ret`',
        'pub unsafe extern "win64" fn tpl_call_static_rcx<const A_VA: u64, const B_VA: u64>() -> u64 {',
        '    let f: unsafe extern "win64" fn(usize) -> u64 = unsafe { crate::exe::func(B_VA as u64) };',
        '    unsafe { f(exe_va(A_VA)) }',
        '}',
        '',
        '/// 2-call static rcx (35 bytes): `sub rsp, 0x28; lea rcx, [rip + A1]; call F; nop; lea rcx, [rip + A2]; call F; nop; add rsp, 0x28; ret`',
        'pub unsafe extern "win64" fn tpl_call_static_rcx_2<',
        '    const A1_VA: u64,',
        '    const A2_VA: u64,',
        '    const F_VA: u64,',
        '>() -> u64 {',
        '    let f: unsafe extern "win64" fn(usize) -> u64 = unsafe { crate::exe::func(F_VA as u64) };',
        '    unsafe {',
        '        f(exe_va(A1_VA));',
        '        f(exe_va(A2_VA))',
        '    }',
        '}',
        '',
        '/// 3-call static rcx (48 bytes): `sub rsp, 0x28; lea rcx, [rip + A1]; call F; nop; lea rcx, [rip + A2]; call F; nop; lea rcx, [rip + A3]; call F; nop; add rsp, 0x28; ret`',
        'pub unsafe extern "win64" fn tpl_call_static_rcx_3<',
        '    const A1_VA: u64,',
        '    const A2_VA: u64,',
        '    const A3_VA: u64,',
        '    const F_VA: u64,',
        '>() -> u64 {',
        '    let f: unsafe extern "win64" fn(usize) -> u64 = unsafe { crate::exe::func(F_VA as u64) };',
        '    unsafe {',
        '        f(exe_va(A1_VA));',
        '        f(exe_va(A2_VA));',
        '        f(exe_va(A3_VA))',
        '    }',
        '}',
        '',
        '/// Scalar deleting destructor w/ RIP vftable (43 bytes)',
        'pub unsafe extern "win64" fn tpl_scalar_deleting_dtor_vft<',
        '    const VFT_VA: u64,',
        '    const SIZE: u32,',
        '    const FREE_VA: u64,',
        '>(',
        '    rcx: usize,',
        '    dl: u8,',
        ') -> usize {',
        '    unsafe {',
        '        crate::exe::wr::<usize>(rcx, exe_va(VFT_VA));',
        '        if (dl & 1) != 0 {',
        '            let free_fn: unsafe extern "win64" fn(usize, u32) =',
        '                crate::exe::func(FREE_VA as u64);',
        '            free_fn(rcx, SIZE);',
        '        }',
        '    }',
        '    rcx',
        '}',
        '',
        '/// 26B: `lea rax, [rip + VAL_VA]; lea rcx, [rip + RCX_VA]; mov [rip + DST_VA], rax; jmp TARGET_VA`',
        'pub unsafe extern "win64" fn tpl_store_rip_va_bind_rcx_jmp<',
        '    const VAL_VA: u64,',
        '    const DST_VA: u64,',
        '    const RCX_VA: u64,',
        '    const TARGET_VA: u64,',
        '>(',
        '    _rcx: u64,',
        '    rdx: u64,',
        '    r8: u64,',
        '    r9: u64,',
        ') -> u64 {',
        '    unsafe {',
        '        crate::exe::wr::<usize>(exe_va(DST_VA), exe_va(VAL_VA));',
        '        let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 =',
        '            crate::exe::func(TARGET_VA as u64);',
        '        f(exe_va(RCX_VA) as u64, rdx, r8, r9)',
        '    }',
        '}',
        '',
        '/// 58B: FNV-1a hash over `[BEG_VA, END_VA)` stored into `[DST_VA]`',
        'pub unsafe extern "win64" fn tpl_fnv1a_range_store_rip_u32<',
        '    const BEG_VA: u64,',
        '    const END_VA: u64,',
        '    const DST_VA: u64,',
        '>() {',
        '    let mut h: u32 = 0x811c_9dc5;',
        '    let mut p = exe_va(BEG_VA);',
        '    let end = exe_va(END_VA);',
        '    while p != end {',
        '        let b = unsafe { crate::exe::rd::<i8>(p) } as i32 as u32;',
        '        p = p.wrapping_add(1);',
        '        h = (h ^ b).wrapping_mul(0x0100_0193);',
        '    }',
        '    unsafe { crate::exe::wr::<u32>(exe_va(DST_VA), h) };',
        '}',
        '',
        '/// 21B: `mov eax, [rip + CNT_VA]; inc eax; mov [rip + CNT_VA], eax; mov [rip + DST_VA], eax; ret`',
        'pub unsafe extern "win64" fn tpl_inc_rip_u32_store_rip_u32<',
        '    const CNT_VA: u64,',
        '    const DST_VA: u64,',
        '>() -> u32 {',
        '    unsafe {',
        '        let c_addr = exe_va(CNT_VA);',
        '        let v = crate::exe::rd::<u32>(c_addr).wrapping_add(1);',
        '        crate::exe::wr::<u32>(c_addr, v);',
        '        crate::exe::wr::<u32>(exe_va(DST_VA), v);',
        '        v',
        '    }',
        '}',
        '',
        '/// 134B: registration entry helper (1,346 functions)',
        'pub unsafe extern "win64" fn tpl_reg_entry_134<const U16_VA: u64, const INIT_FN: u64>(rcx: usize) {',
        '    let bx = unsafe { crate::exe::rd::<u16>(exe_va(U16_VA)) };',
        '    let mut tmp20 = [0u64; 4];',
        '    let mut tmp40 = [0u64; 5];',
        '    unsafe {',
        '        let f1: unsafe extern "win64" fn(*mut u64) -> *const [u8; 32] =',
        '            crate::exe::func(INIT_FN as u64);',
        '        let p = f1(tmp20.as_mut_ptr());',
        '        let v = crate::exe::rd::<[u8; 32]>(p as usize);',
        '        crate::exe::wr::<[u8; 32]>(tmp20.as_mut_ptr() as usize, v);',
        '        let f2: unsafe extern "win64" fn(*mut u64, *mut u64, u32, *const u64) =',
        '            crate::exe::func(0x1420aeb60 as u64);',
        '        f2(tmp20.as_mut_ptr(), tmp40.as_mut_ptr(), bx as u32, tmp20.as_ptr());',
        '        let f3: unsafe extern "win64" fn(usize, *const u64) =',
        '            crate::exe::func(0x1401ed0c0 as u64);',
        '        f3(rcx + 8, tmp40.as_ptr());',
        '        let f_rel: unsafe extern "win64" fn(u64) = crate::exe::func(0x14241c190 as u64);',
        '        if tmp40[2] != 0 {',
        '            f_rel(tmp40[2]);',
        '        }',
        '        tmp40[2] = 0;',
        '        tmp40[3] = 0;',
        '        if tmp40[0] != 0 {',
        '            f_rel(tmp40[0]);',
        '        }',
        '    }',
        '}',
        '',
        '/// 75B / 69B: ObjectStream scope push + inline pop (`rbx = r8`, `rdx` forwarded)',
        'pub unsafe extern "win64" fn tpl_stream_push_pop_r8<',
        '    const R8_PTR_VA: u64,',
        '    const F1: u32,',
        '    const F2: u32,',
        '    const PUSH_FN: u64,',
        '>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: usize,',
        ') {',
        '    unsafe {',
        '        let name_ptr = crate::exe::rd::<u64>(exe_va(R8_PTR_VA));',
        '        let f: unsafe extern "win64" fn(usize, usize, u64, u64, u32, u32) =',
        '            crate::exe::func(PUSH_FN as u64);',
        '        f(r8, rdx, name_ptr, 0, F1, F2);',
        '        stream_pop_inline(r8);',
        '    }',
        '}',
        '',
        '/// 80B / 74B: ObjectStream scope push + inline pop (`rbx = rdx, rdx = rcx`)',
        'pub unsafe extern "win64" fn tpl_stream_push_pop_rdx<',
        '    const R8_PTR_VA: u64,',
        '    const F1: u32,',
        '    const F2: u32,',
        '    const PUSH_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') {',
        '    unsafe {',
        '        let name_ptr = crate::exe::rd::<u64>(exe_va(R8_PTR_VA));',
        '        let f: unsafe extern "win64" fn(usize, usize, u64, u64, u32, u32) =',
        '            crate::exe::func(PUSH_FN as u64);',
        '        f(rdx, rcx, name_ptr, 0, F1, F2);',
        '        stream_pop_inline(rdx);',
        '    }',
        '}',
        '',
        '/// 83B / 77B: ObjectStream begin + body + end (`rdi = r8, rbx = rdx`)',
        'pub unsafe extern "win64" fn tpl_stream_begin_body_end_r8<',
        '    const R9D_VA: u64,',
        '    const R8_VA: u64,',
        '    const F1: u32,',
        '    const F2: u32,',
        '    const BEGIN_FN: u64,',
        '    const BODY_FN: u64,',
        '    const END_FN: u64,',
        '>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let r9d = crate::exe::rd::<u32>(exe_va(R9D_VA));',
        '        let r8_val = crate::exe::rd::<u64>(exe_va(R8_VA));',
        '        let begin: unsafe extern "win64" fn(usize, usize, u64, u32, u32, u32) =',
        '            crate::exe::func(BEGIN_FN as u64);',
        '        begin(r8, rdx, r8_val, r9d, F1, F2);',
        '        let body: unsafe extern "win64" fn(usize, usize) = crate::exe::func(BODY_FN as u64);',
        '        body(rdx, r8);',
        '        let end: unsafe extern "win64" fn(usize) -> u64 = crate::exe::func(END_FN as u64);',
        '        end(r8)',
        '    }',
        '}',
        '',
        '/// 86B / 80B: ObjectStream begin + body + end (`rdi = rdx, rbx = rcx`)',
        'pub unsafe extern "win64" fn tpl_stream_begin_body_end_rdx<',
        '    const R9D_VA: u64,',
        '    const R8_VA: u64,',
        '    const F1: u32,',
        '    const F2: u32,',
        '    const BEGIN_FN: u64,',
        '    const BODY_FN: u64,',
        '    const END_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let r9d = crate::exe::rd::<u32>(exe_va(R9D_VA));',
        '        let r8_val = crate::exe::rd::<u64>(exe_va(R8_VA));',
        '        let begin: unsafe extern "win64" fn(usize, usize, u64, u32, u32, u32) =',
        '            crate::exe::func(BEGIN_FN as u64);',
        '        begin(rdx, rcx, r8_val, r9d, F1, F2);',
        '        let body: unsafe extern "win64" fn(usize, usize) = crate::exe::func(BODY_FN as u64);',
        '        body(rcx, rdx);',
        '        let end: unsafe extern "win64" fn(usize) -> u64 = crate::exe::func(END_FN as u64);',
        '        end(rdx)',
        '    }',
        '}',
        '',
        '/// 60B / 54B: ObjectStream begin + end (`rbx = r8, rdx` forwarded)',
        'pub unsafe extern "win64" fn tpl_stream_begin_end_r8<',
        '    const R9D_VA: u64,',
        '    const R8_VA: u64,',
        '    const F1: u32,',
        '    const F2: u32,',
        '    const BEGIN_FN: u64,',
        '    const END_FN: u64,',
        '>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let r9d = crate::exe::rd::<u32>(exe_va(R9D_VA));',
        '        let r8_val = crate::exe::rd::<u64>(exe_va(R8_VA));',
        '        let begin: unsafe extern "win64" fn(usize, usize, u64, u32, u32, u32) =',
        '            crate::exe::func(BEGIN_FN as u64);',
        '        begin(r8, rdx, r8_val, r9d, F1, F2);',
        '        let end: unsafe extern "win64" fn(usize) -> u64 = crate::exe::func(END_FN as u64);',
        '        end(r8)',
        '    }',
        '}',
        '',
        '/// 63B / 57B: ObjectStream begin + end (`rbx = rdx, rdx = rcx`)',
        'pub unsafe extern "win64" fn tpl_stream_begin_end_rdx<',
        '    const R9D_VA: u64,',
        '    const R8_VA: u64,',
        '    const F1: u32,',
        '    const F2: u32,',
        '    const BEGIN_FN: u64,',
        '    const END_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let r9d = crate::exe::rd::<u32>(exe_va(R9D_VA));',
        '        let r8_val = crate::exe::rd::<u64>(exe_va(R8_VA));',
        '        let begin: unsafe extern "win64" fn(usize, usize, u64, u32, u32, u32) =',
        '            crate::exe::func(BEGIN_FN as u64);',
        '        begin(rdx, rcx, r8_val, r9d, F1, F2);',
        '        let end: unsafe extern "win64" fn(usize) -> u64 = crate::exe::func(END_FN as u64);',
        '        end(rdx)',
        '    }',
        '}',
        '',
        '/// 103B / 97B: ObjectStream push + body + inline pop + ret 0 (`rdi = rdx, rbx = rcx`)',
        'pub unsafe extern "win64" fn tpl_stream_push_body_pop_rdx<',
        '    const R8_VA: u64,',
        '    const FLAG: u32,',
        '    const PUSH_FN: u64,',
        '    const BODY_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let r8_val = crate::exe::rd::<u64>(exe_va(R8_VA));',
        '        let push: unsafe extern "win64" fn(usize, usize, u64, u64, u32, u32) =',
        '            crate::exe::func(PUSH_FN as u64);',
        '        push(rdx, rcx, r8_val, 0, FLAG, FLAG);',
        '        let body: unsafe extern "win64" fn(usize, usize) = crate::exe::func(BODY_FN as u64);',
        '        body(rcx, rdx);',
        '        stream_pop_inline(rdx);',
        '    }',
        '    0',
        '}',
        '',
        '/// 148B: ObjectStream 2x push + 2x inline pop + ret 0 (`rdi = rdx, rbx = rcx`, flag=1)',
        'pub unsafe extern "win64" fn tpl_stream_push2_pop2_rdx<',
        '    const R8_VA1: u64,',
        '    const R8_VA2: u64,',
        '    const PUSH_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let push: unsafe extern "win64" fn(usize, usize, u64, u64, u32, u32) =',
        '            crate::exe::func(PUSH_FN as u64);',
        '        let r8_1 = crate::exe::rd::<u64>(exe_va(R8_VA1));',
        '        push(rdx, rcx, r8_1, 0, 1, 1);',
        '        let r8_2 = crate::exe::rd::<u64>(exe_va(R8_VA2));',
        '        push(rdx, rcx, r8_2, 0, 1, 1);',
        '        stream_pop_inline(rdx);',
        '        stream_pop_inline(rdx);',
        '    }',
        '    0',
        '}',
        '',
        '/// 127B: ObjectStream 2x begin + 2x end + ret 0 (`rdi = rdx, rbx = rcx`, flag=1)',
        'pub unsafe extern "win64" fn tpl_stream_begin2_end2_rdx<',
        '    const R9D_1: u64,',
        '    const R8_1: u64,',
        '    const R9D_2: u64,',
        '    const R8_2: u64,',
        '    const BEGIN_FN: u64,',
        '    const END_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let begin: unsafe extern "win64" fn(usize, usize, u64, u32, u32, u32) =',
        '            crate::exe::func(BEGIN_FN as u64);',
        '        let end: unsafe extern "win64" fn(usize) = crate::exe::func(END_FN as u64);',
        '        begin(',
        '            rdx,',
        '            rcx,',
        '            crate::exe::rd::<u64>(exe_va(R8_1)),',
        '            crate::exe::rd::<u32>(exe_va(R9D_1)),',
        '            1,',
        '            1,',
        '        );',
        '        begin(',
        '            rdx,',
        '            rcx,',
        '            crate::exe::rd::<u64>(exe_va(R8_2)),',
        '            crate::exe::rd::<u32>(exe_va(R9D_2)),',
        '            1,',
        '            1,',
        '        );',
        '        end(rdx);',
        '        end(rdx);',
        '    }',
        '    0',
        '}',
        '',
        '/// 138B: ObjectStream 2x begin + body + 2x end + ret 0 (`rdi = rdx, rbx = rcx`, flag=1)',
        'pub unsafe extern "win64" fn tpl_stream_begin2_body_end2_rdx<',
        '    const R9D_1: u64,',
        '    const R8_1: u64,',
        '    const R9D_2: u64,',
        '    const R8_2: u64,',
        '    const BEGIN_FN: u64,',
        '    const BODY_FN: u64,',
        '    const END_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        let begin: unsafe extern "win64" fn(usize, usize, u64, u32, u32, u32) =',
        '            crate::exe::func(BEGIN_FN as u64);',
        '        let body: unsafe extern "win64" fn(usize, usize) = crate::exe::func(BODY_FN as u64);',
        '        let end: unsafe extern "win64" fn(usize) = crate::exe::func(END_FN as u64);',
        '        begin(',
        '            rdx,',
        '            rcx,',
        '            crate::exe::rd::<u64>(exe_va(R8_1)),',
        '            crate::exe::rd::<u32>(exe_va(R9D_1)),',
        '            1,',
        '            1,',
        '        );',
        '        begin(',
        '            rdx,',
        '            rcx,',
        '            crate::exe::rd::<u64>(exe_va(R8_2)),',
        '            crate::exe::rd::<u32>(exe_va(R9D_2)),',
        '            1,',
        '            1,',
        '        );',
        '        body(rcx, rdx);',
        '        end(rdx);',
        '        end(rdx);',
        '    }',
        '    0',
        '}',
        '',
        '/// 97B: static 3-entry enum/name table init + atexit registration',
        'pub unsafe extern "win64" fn tpl_init_enum3_atexit<',
        '    const TBL_VA: u64,',
        '    const S0_VA: u64,',
        '    const S1_VA: u64,',
        '    const S2_VA: u64,',
        '    const DTOR_VA: u64,',
        '>() -> u64 {',
        '    #[repr(C)]',
        '    struct Pair {',
        '        s: usize,',
        '        v: u64,',
        '    }',
        '    let pairs = [',
        '        Pair { s: exe_va(S0_VA), v: 0 },',
        '        Pair { s: exe_va(S1_VA), v: 1 },',
        '        Pair { s: exe_va(S2_VA), v: 2 },',
        '    ];',
        '    unsafe {',
        '        let init: unsafe extern "win64" fn(usize, *const Pair) =',
        '            crate::exe::func(0x1401de5b0 as u64);',
        '        init(exe_va(TBL_VA), pairs.as_ptr());',
        '        let atexit: unsafe extern "win64" fn(usize) -> u64 =',
        '            crate::exe::func(0x141834d18 as u64);',
        '        atexit(exe_va(DTOR_VA))',
        '    }',
        '}',
        '',
        '/// 234B: static linked-list node registration with thread-safe init guard (3,197 functions)',
        'pub unsafe extern "win64" fn tpl_reg_static_node_234<',
        '    const GUARD_VA: u64,',
        '    const NODE_VA: u64,',
        '    const ID_VA: u64,',
        '    const FN_VA: u64,',
        '    const STR1_VA: u64,',
        '    const LEN1: u64,',
        '    const STR2_VA: u64,',
        '    const LEN2: u64,',
        '>() {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let epoch = crate::exe::rd::<i32>(tls0 + 0x74);',
        '        let guard = exe_va(GUARD_VA);',
        '        let node = exe_va(NODE_VA);',
        '        if crate::exe::rd::<i32>(guard) > epoch {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                crate::exe::wr::<[u8; 32]>(node + 0x10, [0u8; 32]);',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        let head_addr = exe_va(0x143a00288);',
        '        let old_head = crate::exe::rd::<usize>(head_addr);',
        '        crate::exe::wr::<usize>(node, old_head);',
        '        crate::exe::wr::<usize>(node + 0x08, exe_va(ID_VA));',
        '        crate::exe::wr::<usize>(node + 0x10, exe_va(STR1_VA));',
        '        crate::exe::wr::<u64>(node + 0x18, LEN1);',
        '        crate::exe::wr::<usize>(node + 0x20, exe_va(STR2_VA));',
        '        crate::exe::wr::<u64>(node + 0x28, LEN2);',
        '        crate::exe::wr::<usize>(node + 0x30, exe_va(FN_VA));',
        '        crate::exe::wr::<usize>(head_addr, node);',
        '        crate::exe::wr::<u32>(exe_va(ID_VA), 0xffff_ffff);',
        '    }',
        '}',
        '',
        '/// 59B: TLS bitmask clear (2,286 functions)',
        'pub unsafe extern "win64" fn tpl_tls_clear_bit_59<const OFF: usize, const MASK: u8>() {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let v = crate::exe::rd::<u32>(tls0 + OFF) & ((MASK as i8 as i32) as u32);',
        '        let tls0_2 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        crate::exe::wr::<u32>(tls0_2 + OFF, v);',
        '    }',
        '}',
        '',
        '/// 44B: TLS lazy zero init (88 functions)',
        'pub unsafe extern "win64" fn tpl_tls_lazy_zero_44<',
        '    const FLAG_OFF: usize,',
        '    const BIT: u8,',
        '    const VAL_OFF: usize,',
        '>() {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let fl = crate::exe::rd::<u32>(tls0 + FLAG_OFF);',
        '        if (fl & (BIT as u32)) == 0 {',
        '            crate::exe::wr::<u32>(tls0 + FLAG_OFF, fl | (BIT as u32));',
        '            crate::exe::wr::<u64>(tls0 + VAL_OFF, 0);',
        '        }',
        '    }',
        '}',
        '',
        '/// 29B: TLS store RIP address (49 functions)',
        'pub unsafe extern "win64" fn tpl_tls_store_rip_29<const TLS_OFF: usize, const TARGET_VA: u64>() -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let v = exe_va(TARGET_VA);',
        '        crate::exe::wr::<usize>(tls0 + TLS_OFF, v);',
        '        v',
        '    }',
        '}',
        '',
        '/// 104B: thread-safe static init + atexit + store rcx (792 functions)',
        'pub unsafe extern "win64" fn tpl_init_static_atexit_104<',
        '    const GUARD_VA: u64,',
        '    const RET_VA: u64,',
        '    const DTOR_VA: u64,',
        '>(',
        '    rcx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let epoch = crate::exe::rd::<i32>(tls0 + 0x74);',
        '        let guard = exe_va(GUARD_VA);',
        '        if crate::exe::rd::<i32>(guard) > epoch {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                let atexit: unsafe extern "win64" fn(usize) -> u64 =',
        '                    crate::exe::func(0x141834d18 as u64);',
        '                atexit(exe_va(DTOR_VA));',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        let ret = exe_va(RET_VA);',
        '        crate::exe::wr::<usize>(ret + 8, rcx);',
        '        ret',
        '    }',
        '}',
        '',
        '/// Component descriptor table registration + MurmurHash3 64-bit finalizer (785 functions)',
        'pub unsafe extern "win64" fn tpl_reg_comp_desc<',
        '    const U16_VA: u64,',
        '    const NAME_VA: u64,',
        '    const NAME_LEN: u64,',
        '    const SZ_VAL: u64,',
        '    const ALIGN_VAL: u64,',
        '    const FLAG_U32: u32,',
        '    const FN_D0: u64,',
        '    const FN_D8: u64,',
        '    const FN_E0: u64,',
        '    const FN_E8: u64,',
        '    const FN_F0: u64,',
        '>(',
        '    rcx: usize,',
        ') {',
        '    #[inline(always)]',
        '    fn fn_opt(va: u64) -> usize {',
        '        if va == 0 { 0 } else { exe_va(va) }',
        '    }',
        '    unsafe {',
        '        let edx = crate::exe::rd::<u16>(exe_va(U16_VA)) as usize;',
        '        let base = crate::exe::rd::<usize>(rcx) + edx * 80;',
        '        crate::exe::wr::<usize>(base, exe_va(NAME_VA));',
        '        crate::exe::wr::<u64>(base + 0x08, NAME_LEN);',
        '        crate::exe::wr::<u64>(base + 0x10, SZ_VAL);',
        '        crate::exe::wr::<u64>(base + 0x18, ALIGN_VAL);',
        '        crate::exe::wr::<u16>(base + 0x20, edx as u16);',
        '        crate::exe::wr::<u32>(base + 0x24, FLAG_U32);',
        '        crate::exe::wr::<usize>(base + 0x28, fn_opt(FN_D0));',
        '        crate::exe::wr::<usize>(base + 0x30, fn_opt(FN_D8));',
        '        crate::exe::wr::<usize>(base + 0x38, fn_opt(FN_E0));',
        '        crate::exe::wr::<usize>(base + 0x40, fn_opt(FN_E8));',
        '        crate::exe::wr::<usize>(base + 0x48, fn_opt(FN_F0));',
        '        let mut h = (edx as u64).wrapping_add(1);',
        '        h ^= h >> 33;',
        '        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);',
        '        h ^= h >> 33;',
        '        h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);',
        '        h ^= h >> 33;',
        '        let tbl2 = crate::exe::rd::<usize>(rcx + 0x10);',
        '        crate::exe::wr::<u64>(tbl2 + edx * 8, h);',
        '    }',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn write_desc_entry(arr: usize, idx: usize, u16_va: u64, f1: u32, f2: u32) {',
        '    unsafe {',
        '        let id = crate::exe::rd::<u16>(exe_va(u16_va)) as u32;',
        '        let p = arr + idx * 12;',
        '        crate::exe::wr::<u32>(p, id);',
        '        crate::exe::wr::<u32>(p + 4, f1);',
        '        crate::exe::wr::<u32>(p + 8, f2);',
        '    }',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn finish_desc_arr(rcx: usize, n: u64, arr: usize) -> usize {',
        '    unsafe {',
        '        crate::exe::wr::<u64>(rcx, n);',
        '        crate::exe::wr::<usize>(rcx + 8, arr);',
        '        crate::exe::wr::<u64>(rcx + 16, 0);',
        '        crate::exe::wr::<u64>(rcx + 24, 0);',
        '    }',
        '    rcx',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_desc_arr_1<',
        '    const GUARD_VA: u64,',
        '    const ARR_VA: u64,',
        '    const U0: u64,',
        '    const A0: u32,',
        '    const B0: u32,',
        '>(',
        '    rcx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let guard = exe_va(GUARD_VA);',
        '        let arr = exe_va(ARR_VA);',
        '        if crate::exe::rd::<i32>(guard) > crate::exe::rd::<i32>(tls0 + 0x74) {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                write_desc_entry(arr, 0, U0, A0, B0);',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        finish_desc_arr(rcx, 1, arr)',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_desc_arr_2<',
        '    const GUARD_VA: u64,',
        '    const ARR_VA: u64,',
        '    const U0: u64,',
        '    const A0: u32,',
        '    const B0: u32,',
        '    const U1: u64,',
        '    const A1: u32,',
        '    const B1: u32,',
        '>(',
        '    rcx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let guard = exe_va(GUARD_VA);',
        '        let arr = exe_va(ARR_VA);',
        '        if crate::exe::rd::<i32>(guard) > crate::exe::rd::<i32>(tls0 + 0x74) {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                write_desc_entry(arr, 0, U0, A0, B0);',
        '                write_desc_entry(arr, 1, U1, A1, B1);',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        finish_desc_arr(rcx, 2, arr)',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_desc_arr_3<',
        '    const GUARD_VA: u64,',
        '    const ARR_VA: u64,',
        '    const U0: u64,',
        '    const A0: u32,',
        '    const B0: u32,',
        '    const U1: u64,',
        '    const A1: u32,',
        '    const B1: u32,',
        '    const U2: u64,',
        '    const A2: u32,',
        '    const B2: u32,',
        '>(',
        '    rcx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let guard = exe_va(GUARD_VA);',
        '        let arr = exe_va(ARR_VA);',
        '        if crate::exe::rd::<i32>(guard) > crate::exe::rd::<i32>(tls0 + 0x74) {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                write_desc_entry(arr, 0, U0, A0, B0);',
        '                write_desc_entry(arr, 1, U1, A1, B1);',
        '                write_desc_entry(arr, 2, U2, A2, B2);',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        finish_desc_arr(rcx, 3, arr)',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_desc_arr_4<',
        '    const GUARD_VA: u64,',
        '    const ARR_VA: u64,',
        '    const U0: u64,',
        '    const A0: u32,',
        '    const B0: u32,',
        '    const U1: u64,',
        '    const A1: u32,',
        '    const B1: u32,',
        '    const U2: u64,',
        '    const A2: u32,',
        '    const B2: u32,',
        '    const U3: u64,',
        '    const A3: u32,',
        '    const B3: u32,',
        '>(',
        '    rcx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let guard = exe_va(GUARD_VA);',
        '        let arr = exe_va(ARR_VA);',
        '        if crate::exe::rd::<i32>(guard) > crate::exe::rd::<i32>(tls0 + 0x74) {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                write_desc_entry(arr, 0, U0, A0, B0);',
        '                write_desc_entry(arr, 1, U1, A1, B1);',
        '                write_desc_entry(arr, 2, U2, A2, B2);',
        '                write_desc_entry(arr, 3, U3, A3, B3);',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        finish_desc_arr(rcx, 4, arr)',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_desc_arr_5<',
        '    const GUARD_VA: u64,',
        '    const ARR_VA: u64,',
        '    const U0: u64,',
        '    const A0: u32,',
        '    const B0: u32,',
        '    const U1: u64,',
        '    const A1: u32,',
        '    const B1: u32,',
        '    const U2: u64,',
        '    const A2: u32,',
        '    const B2: u32,',
        '    const U3: u64,',
        '    const A3: u32,',
        '    const B3: u32,',
        '    const U4: u64,',
        '    const A4: u32,',
        '    const B4: u32,',
        '>(',
        '    rcx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let guard = exe_va(GUARD_VA);',
        '        let arr = exe_va(ARR_VA);',
        '        if crate::exe::rd::<i32>(guard) > crate::exe::rd::<i32>(tls0 + 0x74) {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                write_desc_entry(arr, 0, U0, A0, B0);',
        '                write_desc_entry(arr, 1, U1, A1, B1);',
        '                write_desc_entry(arr, 2, U2, A2, B2);',
        '                write_desc_entry(arr, 3, U3, A3, B3);',
        '                write_desc_entry(arr, 4, U4, A4, B4);',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        finish_desc_arr(rcx, 5, arr)',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_desc_arr_6<',
        '    const GUARD_VA: u64,',
        '    const ARR_VA: u64,',
        '    const U0: u64,',
        '    const A0: u32,',
        '    const B0: u32,',
        '    const U1: u64,',
        '    const A1: u32,',
        '    const B1: u32,',
        '    const U2: u64,',
        '    const A2: u32,',
        '    const B2: u32,',
        '    const U3: u64,',
        '    const A3: u32,',
        '    const B3: u32,',
        '    const U4: u64,',
        '    const A4: u32,',
        '    const B4: u32,',
        '    const U5: u64,',
        '    const A5: u32,',
        '    const B5: u32,',
        '>(',
        '    rcx: usize,',
        ') -> usize {',
        '    unsafe {',
        '        let tls0 = crate::exe::rd::<usize>(crate::exe::tls_pointer());',
        '        let guard = exe_va(GUARD_VA);',
        '        let arr = exe_va(ARR_VA);',
        '        if crate::exe::rd::<i32>(guard) > crate::exe::rd::<i32>(tls0 + 0x74) {',
        '            let hdr: unsafe extern "win64" fn(usize) = crate::exe::func(0x141834a78 as u64);',
        '            hdr(guard);',
        '            if crate::exe::rd::<i32>(guard) == -1 {',
        '                write_desc_entry(arr, 0, U0, A0, B0);',
        '                write_desc_entry(arr, 1, U1, A1, B1);',
        '                write_desc_entry(arr, 2, U2, A2, B2);',
        '                write_desc_entry(arr, 3, U3, A3, B3);',
        '                write_desc_entry(arr, 4, U4, A4, B4);',
        '                write_desc_entry(arr, 5, U5, A5, B5);',
        '                let ftr: unsafe extern "win64" fn(usize) =',
        '                    crate::exe::func(0x141834a0c as u64);',
        '                ftr(guard);',
        '            }',
        '        }',
        '        finish_desc_arr(rcx, 6, arr)',
        '    }',
        '}',
        '',
        '#[repr(C)]',
        'struct HashMapEntry {',
        '    ptr: usize,',
        '    len: u64,',
        '    val: u64,',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn init_hashmap_entries(map_va: u64, entries: &[HashMapEntry], dtor_va: u64) -> u64 {',
        '    unsafe {',
        '        let map = exe_va(map_va);',
        '        let f_res: unsafe extern "win64" fn(usize, u64) =',
        '            crate::exe::func(0x1401de990 as u64);',
        '        f_res(map, entries.len() as u64);',
        '        let f_hash: unsafe extern "win64" fn(usize, u64) -> u64 =',
        '            crate::exe::func(0x1424b16a0 as u64);',
        '        let f_ins: unsafe extern "win64" fn(',
        '            usize,',
        '            *mut [u64; 2],',
        '            *const HashMapEntry,',
        '            u64,',
        '            *const *const HashMapEntry,',
        '            *const *const u64,',
        '        ) = crate::exe::func(0x1401dede0 as u64);',
        '        let mut dummy = [0u64; 2];',
        '        for e in entries {',
        '            let key_ptr: *const HashMapEntry = e;',
        '            let val_ptr: *const u64 = &e.val;',
        '            let h = f_hash(e.ptr, e.len);',
        '            let prod = (h as u128).wrapping_mul(0xde5f_b9d2_6304_58e9u128);',
        '            let r9 = ((prod >> 64) as u64).wrapping_add(prod as u64);',
        '            f_ins(map, &mut dummy, e, r9, &key_ptr, &val_ptr);',
        '        }',
        '        let atexit: unsafe extern "win64" fn(usize) -> u64 =',
        '            crate::exe::func(0x141834d18 as u64);',
        '        atexit(exe_va(dtor_va))',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_hashmap3_atexit<',
        '    const MAP_VA: u64,',
        '    const S0_VA: u64,',
        '    const DTOR_VA: u64,',
        '>() -> u64 {',
        '    let entries = [',
        '        HashMapEntry { ptr: exe_va(S0_VA), len: 16, val: 0 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 24), len: 10, val: 1 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 40), len: 10, val: 2 },',
        '    ];',
        '    unsafe { init_hashmap_entries(MAP_VA, &entries, DTOR_VA) }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_init_hashmap7_atexit<',
        '    const MAP_VA: u64,',
        '    const S0_VA: u64,',
        '    const DTOR_VA: u64,',
        '>() -> u64 {',
        '    let entries = [',
        '        HashMapEntry { ptr: exe_va(S0_VA), len: 6, val: 9 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 12), len: 8, val: 8 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 24), len: 6, val: 7 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 36), len: 8, val: 12 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 52), len: 8, val: 11 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 64), len: 6, val: 10 },',
        '        HashMapEntry { ptr: exe_va(S0_VA + 76), len: 11, val: 13 },',
        '    ];',
        '    unsafe { init_hashmap_entries(MAP_VA, &entries, DTOR_VA) }',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn obj_ver_step1(rdx: usize, r8: u8, t1_va: u64, v1: u32) {',
        '    unsafe {',
        '        let mut ver: u32 = v1;',
        '        let s = crate::exe::rd::<usize>(rdx);',
        '        if r8 == 0 {',
        '            let mut tmp = crate::exe::rd::<usize>(exe_va(t1_va));',
        '            let f_ot: unsafe extern "win64" fn(usize, usize, *mut usize) =',
        '                crate::exe::func(0x1413c3940 as u64);',
        '            f_ot(s, exe_va(0x14307aa58), &mut tmp);',
        '            let s2 = crate::exe::rd::<usize>(rdx);',
        '            let f_ver: unsafe extern "win64" fn(usize, usize, *mut u32) =',
        '                crate::exe::func(0x142481290 as u64);',
        '            f_ver(s2, exe_va(0x14307a9d0), &mut ver);',
        '        } else {',
        '            let t1 = crate::exe::rd::<usize>(exe_va(t1_va));',
        '            let f_fmt: unsafe extern "win64" fn(usize, usize) -> usize =',
        '                crate::exe::func(0x1423fe030 as u64);',
        '            let fmt = f_fmt(exe_va(0x14307a9e0), t1);',
        '            let f_ver: unsafe extern "win64" fn(usize, usize, *mut u32) =',
        '                crate::exe::func(0x142481290 as u64);',
        '            f_ver(s, fmt, &mut ver);',
        '        }',
        '    }',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn obj_ver_extra(rdx: usize, t_va: u64, v: u32) {',
        '    unsafe {',
        '        let mut ver: u32 = v;',
        '        let s = crate::exe::rd::<usize>(rdx);',
        '        let t = crate::exe::rd::<usize>(exe_va(t_va));',
        '        let f_fmt: unsafe extern "win64" fn(usize, usize) -> usize =',
        '            crate::exe::func(0x1423fe030 as u64);',
        '        let fmt = f_fmt(exe_va(0x14307a9e0), t);',
        '        let f_ver: unsafe extern "win64" fn(usize, usize, *mut u32) =',
        '            crate::exe::func(0x142481290 as u64);',
        '        f_ver(s, fmt, &mut ver);',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_obj_ver_1<const T1_VA: u64, const V1: u32>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: u8,',
        ') -> u64 {',
        '    unsafe { obj_ver_step1(rdx, r8, T1_VA, V1) };',
        '    0',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_obj_ver_1_call<',
        '    const T1_VA: u64,',
        '    const V1: u32,',
        '    const SUB_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        '    r8: u8,',
        ') -> u64 {',
        '    unsafe {',
        '        obj_ver_step1(rdx, r8, T1_VA, V1);',
        '        let sub: unsafe extern "win64" fn(usize, usize, u8) =',
        '            crate::exe::func(SUB_FN as u64);',
        '        sub(rcx, rdx, 1);',
        '    }',
        '    0',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_obj_ver_2<',
        '    const T1_VA: u64,',
        '    const V1: u32,',
        '    const T2_VA: u64,',
        '    const V2: u32,',
        '>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: u8,',
        ') -> u64 {',
        '    unsafe {',
        '        obj_ver_step1(rdx, r8, T1_VA, V1);',
        '        obj_ver_extra(rdx, T2_VA, V2);',
        '    }',
        '    0',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_obj_ver_3<',
        '    const T1_VA: u64,',
        '    const V1: u32,',
        '    const T2_VA: u64,',
        '    const V2: u32,',
        '    const T3_VA: u64,',
        '    const V3: u32,',
        '>(',
        '    _rcx: usize,',
        '    rdx: usize,',
        '    r8: u8,',
        ') -> u64 {',
        '    unsafe {',
        '        obj_ver_step1(rdx, r8, T1_VA, V1);',
        '        obj_ver_extra(rdx, T2_VA, V2);',
        '        obj_ver_extra(rdx, T3_VA, V3);',
        '    }',
        '    0',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn decode_refl_slot(kind: u64, val: u64) -> u64 {',
        '    match kind {',
        '        1 => exe_va(val) as u64,',
        '        2 => unsafe { crate::exe::rd::<u64>(exe_va(val)) },',
        '        3 => unsafe { crate::exe::rd::<u32>(exe_va(val)) as u64 },',
        '        _ => val,',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_refl_leaf<',
        '    const META: u64,',
        '    const R9_VA: u64,',
        '    const S20: u64,',
        '    const S28: u64,',
        '    const S30: u64,',
        '    const S38: u64,',
        '    const TARGET_FN: u64,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        '    r8: usize,',
        ') -> u64 {',
        '    let field_off = ((META >> 32) as u32) as usize;',
        '    let r8d = ((META >> 16) & 0xffff) as u32;',
        '    let (rcx_out, rdx_out) = if (META & 1) == 0 {',
        '        (r8, rdx.wrapping_add(field_off))',
        '    } else {',
        '        (rdx, rcx.wrapping_add(field_off))',
        '    };',
        '    unsafe {',
        '        let a20 = decode_refl_slot((META >> 1) & 3, S20);',
        '        let a28 = decode_refl_slot((META >> 3) & 3, S28);',
        '        let a30 = decode_refl_slot((META >> 5) & 3, S30);',
        '        let a38 = decode_refl_slot((META >> 7) & 3, S38);',
        '        let f: unsafe extern "win64" fn(usize, usize, u32, usize, u64, u64, u64, u64) -> u64 =',
        '            crate::exe::func(TARGET_FN as u64);',
        '        f(rcx_out, rdx_out, r8d, exe_va(R9_VA), a20, a28, a30, a38)',
        '    }',
        '}',
        '',
        '#[inline(always)]',
        'unsafe fn run_stream_op(s: usize, obj: usize, meta: u64, val: u64) {',
        '    let kind = meta & 0xff;',
        '    if kind == 0 {',
        '        return;',
        '    }',
        '    let off = ((meta >> 32) as i32) as isize;',
        '    let target_obj = obj.wrapping_add_signed(off);',
        '    unsafe {',
        '        match kind {',
        '            1..=4 => {',
        '                let fn_va: u64 = match kind {',
        '                    1 => 0x1401e9770,',
        '                    2 => 0x1401e9970,',
        '                    3 => 0x1401e9b60,',
        '                    _ => 0x1401e9fa0,',
        '                };',
        '                let f1 = ((meta >> 8) & 0xff) as u32;',
        '                let f2 = ((meta >> 16) & 0xff) as u32;',
        '                let r9d_va = 0x140000000 + (val >> 32);',
        '                let r8_va = 0x140000000 + (val & 0xffff_ffff);',
        '                let begin: unsafe extern "win64" fn(usize, usize, u64, u32, u32, u32) =',
        '                    crate::exe::func(fn_va as u64);',
        '                begin(',
        '                    s,',
        '                    target_obj,',
        '                    crate::exe::rd::<u64>(exe_va(r8_va)),',
        '                    crate::exe::rd::<u32>(exe_va(r9d_va)),',
        '                    f1,',
        '                    f2,',
        '                );',
        '            }',
        '            5 => {',
        '                let f1 = ((meta >> 8) & 0xff) as u32;',
        '                let f2 = ((meta >> 16) & 0xff) as u32;',
        '                let push: unsafe extern "win64" fn(usize, usize, u64, u64, u32, u32) =',
        '                    crate::exe::func(0x14243b0b0 as u64);',
        '                push(s, target_obj, crate::exe::rd::<u64>(exe_va(val)), 0, f1, f2);',
        '            }',
        '            6 => {',
        '                let f: unsafe extern "win64" fn(usize, usize) = crate::exe::func(val as u64);',
        '                f(target_obj, s);',
        '            }',
        '            7 => {',
        '                let f: unsafe extern "win64" fn(usize, usize) = crate::exe::func(val as u64);',
        '                f(s, target_obj);',
        '            }',
        '            8 => {',
        '                let sub_s = crate::exe::rd::<usize>(s + 0x338);',
        '                let vtbl = crate::exe::rd::<usize>(sub_s);',
        '                let fn_ptr = crate::exe::rd::<usize>(vtbl + (val as usize));',
        '                let f: unsafe extern "win64" fn(usize, usize) = crate::exe::func(fn_ptr as u64);',
        '                f(sub_s, target_obj);',
        '            }',
        '            9 => {',
        '                stream_pop_inline(s);',
        '            }',
        '            10 => {',
        '                let end: unsafe extern "win64" fn(usize) = crate::exe::func(val as u64);',
        '                end(s);',
        '            }',
        '            _ => {}',
        '        }',
        '    }',
        '}',
        '',
        'pub unsafe extern "win64" fn tpl_stream_seq_8<',
        '    const M0: u64,',
        '    const V0: u64,',
        '    const M1: u64,',
        '    const V1: u64,',
        '    const M2: u64,',
        '    const V2: u64,',
        '    const M3: u64,',
        '    const V3: u64,',
        '    const M4: u64,',
        '    const V4: u64,',
        '    const M5: u64,',
        '    const V5: u64,',
        '    const M6: u64,',
        '    const V6: u64,',
        '    const M7: u64,',
        '    const V7: u64,',
        '    const EP_POPS: u32,',
        '>(',
        '    rcx: usize,',
        '    rdx: usize,',
        ') -> u64 {',
        '    unsafe {',
        '        run_stream_op(rdx, rcx, M0, V0);',
        '        run_stream_op(rdx, rcx, M1, V1);',
        '        run_stream_op(rdx, rcx, M2, V2);',
        '        run_stream_op(rdx, rcx, M3, V3);',
        '        run_stream_op(rdx, rcx, M4, V4);',
        '        run_stream_op(rdx, rcx, M5, V5);',
        '        run_stream_op(rdx, rcx, M6, V6);',
        '        run_stream_op(rdx, rcx, M7, V7);',
        '        if EP_POPS >= 1 {',
        '            stream_pop_inline(rdx);',
        '        }',
        '        if EP_POPS >= 2 {',
        '            stream_pop_inline(rdx);',
        '        }',
        '    }',
        '    0',
        '}',
        '',
        'pub static ENTRIES: &[Replacement] = &[',
    ]

    for va, fn_name, tpl_fn, params, throws_discard in entries:
        param_str = ', '.join(f'0x{p:x}' for p in params)
        ptr_expr = f'{tpl_fn}::<{param_str}> as *const ()'
        if throws_discard:
            lines.append(f'    Replacement::draft_throws(0x{va:x}, "{fn_name}", {ptr_expr}, Throws::Discard),')
        else:
            lines.append(f'    Replacement::draft(0x{va:x}, "{fn_name}", {ptr_expr}),')

    lines.append('];')
    lines.append('')

    with open(out_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))

    print(f'Generated {len(entries)} entries in {out_path}: {counts}')


if __name__ == '__main__':
    main()
