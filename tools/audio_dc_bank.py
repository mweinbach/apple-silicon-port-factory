#!/usr/bin/env python3
"""DC-signal bank surgery for the A2 oracle (docs/AUDIO.md "A2 oracle harness").

Rewrite every Sound (HIRC type 0x02) in a Wwise 2022 bank to play an in-bank
PCM DC .wem: source plugin -> PCM (0x00010001), stream type 0 (bank data),
media replaced in DIDX/DATA by a 16-bit RIFF/WAVE (Wwise PCM layout: fmt size
0x18, tag 0xFFFE, AkChannelConfig at fmt+0x14) holding a constant value.

  dc_bank.py <in.bnk> <out.bnk> [--value 16384] [--seconds 1.0] [--channels 1]
"""
import argparse
import struct


def chunks(b):
    pos, out = 0, []
    while pos + 8 <= len(b):
        tag = b[pos:pos + 4]
        n = struct.unpack_from('<I', b, pos + 4)[0]
        out.append((tag, b[pos + 8:pos + 8 + n]))
        pos += 8 + n
    return out


def wem(value, frames, ch, rate=48000, impulse=None):
    # AkChannelConfig: count | type(1 = standard) << 8 | mask << 12.
    mask = {1: 0x4, 2: 0x3}[ch]
    cfg = ch | (1 << 8) | (mask << 12)
    fmt = struct.pack('<HHIIHHHHI', 0xFFFE, ch, rate, rate * 2 * ch, 2 * ch, 16, 6, 16, cfg)
    if impulse is None:
        data = struct.pack('<h', value) * (frames * ch)
    else:
        s = [0] * (frames * ch)
        for c in range(ch):
            s[impulse * ch + c] = value
        data = struct.pack('<%dh' % len(s), *s)
    body = b'WAVE' + b'fmt ' + struct.pack('<I', len(fmt)) + fmt + b'data' + struct.pack('<I', len(data)) + data
    return b'RIFF' + struct.pack('<I', len(body)) + body


def neutralize_node(body, p):
    """In-place: NodeBase at body[p:]. Bypass node FX, zero shaping props.
    Layout (v145): fx override u8, fx n u8, [bypass u8, n * (idx u8, id u32,
    shareset u8, rendered u8)], metadata override u8, n u8, n * 6 bytes,
    attach u8, bus u32, parent u32, priority bits u8, props, ranged props."""
    p += 1
    n = body[p]
    p += 1
    if n:
        body[p] |= 0x1F
        p += 1 + 7 * n
    p += 1
    n = body[p]
    p += 1 + 6 * n
    p += 1 + 4 + 4 + 1
    return zero_props(body, p)


ZERO = {0x00, 0x02, 0x03, 0x04, 0x06, 0x0F, 0x10, 0x24, 0x25, 0x26, 0x27, 0x3B, 0x18, 0x19, 0x1A, 0x46}
MUTE = {0x13, 0x14, 0x15, 0x16, 0x17}  # user aux sends 0-3, game aux send -> -96 dB


def zero_props(body, p):
    """In-place: AkPropBundle + ranged AkPropBundle at body[p:]."""
    n = body[p]
    ids = body[p + 1:p + 1 + n]
    q = p + 1 + n
    for k, i in enumerate(ids):
        if i in ZERO:
            struct.pack_into('<I', body, q + 4 * k, 0)
        elif i in MUTE:
            struct.pack_into('<f', body, q + 4 * k, -96.0)
    p = q + 4 * n
    n = body[p]
    ids = body[p + 1:p + 1 + n]
    q = p + 1 + n
    for k, i in enumerate(ids):
        struct.pack_into('<II', body, q + 8 * k, 0, 0)
    return q + 8 * n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('inp')
    ap.add_argument('out')
    ap.add_argument('--value', type=int, default=16384)
    ap.add_argument('--seconds', type=float, default=1.0)
    ap.add_argument('--channels', type=int, default=1)
    ap.add_argument('--neutral', action='store_true', help='zero volume/pitch/LPF/HPF/delay/fade props and randomizers, bypass node FX')
    ap.add_argument('--only', type=int, default=0, help='source id that gets the DC; the others get silence')
    ap.add_argument("--impulse", type=int, default=None, help="media = one sample of VALUE at this index (else DC)")
    a = ap.parse_args()
    b = open(a.inp, 'rb').read()
    cs = chunks(b)
    media = wem(a.value, int(a.seconds * 48000), a.channels, impulse=a.impulse)
    src_ids = []
    out = []
    for tag, body in cs:
        if tag == b'HIRC':
            body = bytearray(body)
            n = struct.unpack_from('<I', body, 0)[0]
            p = 4
            for _ in range(n):
                ty = body[p]
                sz = struct.unpack_from('<I', body, p + 1)[0]
                if ty == 0x02:
                    q = p + 5 + 4  # past id
                    plugin, stype, sid, msize = struct.unpack_from('<IBII', body, q)
                    if plugin & 0xF == 2:
                        print(f'skip sound with source plugin {plugin:#x}')
                    else:
                        struct.pack_into('<IBII', body, q, 0x00010001, 0, sid, len(media))
                        src_ids.append(sid)
                        print(f'sound {struct.unpack_from("<I", body, p + 5)[0]}: plugin {plugin:#010x} stream {stype} src {sid} {msize} B -> PCM DC {len(media)} B')
                        if a.neutral:
                            neutralize_node(body, q + 14)
                elif a.neutral and ty in (0x05, 0x06, 0x07, 0x09):
                    neutralize_node(body, p + 9)
                elif a.neutral and ty == 0x03:
                    zero_props(body, p + 9 + 2 + 4 + 1)
                p += 5 + sz
            body = bytes(body)
        out.append((tag, body))
    # Rebuild DIDX/DATA: every referenced source id -> the DC wem (16-aligned).
    ids = sorted(set(src_ids))
    silent = wem(0, int(a.seconds * 48000), a.channels)
    didx, data = b'', b''
    for sid in ids:
        while len(data) % 16:
            data += b'\0'
        m = silent if a.only and sid != a.only else media
        didx += struct.pack('<III', sid, len(data), len(m))
        data += m
    res = b''
    have = {t for t, _ in out}
    for tag, body in out:
        if tag in (b'DIDX', b'DATA'):
            continue
        res += tag + struct.pack('<I', len(body)) + body
        if tag == b'BKHD':
            res += b'DIDX' + struct.pack('<I', len(didx)) + didx
            res += b'DATA' + struct.pack('<I', len(data)) + data
    open(a.out, 'wb').write(res)
    print(f'{a.out}: {len(ids)} media, {len(res)} bytes (had DIDX: {b"DIDX" in have})')


if __name__ == '__main__':
    main()
