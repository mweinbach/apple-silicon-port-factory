#!/usr/bin/env python3
"""Analyse a DC-test float capture (A2 oracle): onset, first-frame shape,
plateau, stop/end ramp. Usage: dc_analyze.py <x.f32.wav> [frame=512]"""
import struct
import sys

p = sys.argv[1]
N = int(sys.argv[2]) if len(sys.argv) > 2 else 512
b = open(p, 'rb').read()
pos = 12
ch = 2
while pos + 8 <= len(b):
    t = b[pos:pos + 4]
    n = struct.unpack_from('<I', b, pos + 4)[0]
    if t == b'fmt ':
        ch = struct.unpack_from('<H', b, pos + 10)[0]
    if t == b'data':
        s = struct.unpack_from('<%df' % (n // 4), b, pos + 8)
        break
    pos += 8 + n + (n & 1)
L = s[0::ch]
R = s[1::ch] if ch > 1 else L
nz = [i for i, v in enumerate(L) if v != 0.0]
if not nz:
    print('silent')
    sys.exit()
a, z = nz[0], nz[-1]
print(f'{len(L)} frames-of-samples, {len(L)/N:.2f} audio frames; nonzero samples {a}..{z} (onset at frame {a//N} + {a%N})')
plateau = L[a + 2 * N]
print(f'plateau L={plateau!r} R={R[a + 2 * N]!r} L/R={plateau / R[a + 2 * N] if R[a + 2 * N] else float("nan")}')
f0 = L[a:a + N]
print('first frame L[0..4] =', f0[:4], ' L[N-2..N] =', f0[-2:])
print('first frame / plateau: first %.6f, mid %.6f, last %.6f' % (f0[0] / plateau, f0[N // 2] / plateau, f0[-1] / plateau))
lin = all(abs(f0[i] / plateau - (i + 1) / N) < 1e-4 for i in range(N))
flat = all(abs(f0[i] / plateau - 1) < 1e-6 for i in range(N))
print('first frame shape:', 'FLAT (no fade-in)' if flat else ('LINEAR 0->1 ramp (k+1)/N' if lin else 'other'))
# second frame flat?
f1 = L[a + N:a + 2 * N]
print('second frame flat:', all(abs(v / plateau - 1) < 1e-6 for v in f1))
# end: find last frame(s)
e0 = (z // N) * N
fe = L[e0:z + 1]
print(f'last nonzero frame starts at sample {e0} (frame {e0//N}), {len(fe)} nonzero samples; first/last ratio {fe[0]/plateau:.6f} / {fe[-1]/plateau:.6f}')
prev = L[e0 - N:e0]
print('frame before last: first/mid/last ratio %.6f %.6f %.6f' % (prev[0] / plateau, prev[N // 2] / plateau, prev[-1] / plateau))
lin_dn = all(abs(fe[i] / plateau - (1 - (i + 1) / N)) < 1e-4 for i in range(len(fe)))
print('last frame shape:', 'LINEAR 1->0 ramp 1-(k+1)/N' if lin_dn and len(fe) >= N - 1 else ('FLAT then cut' if all(abs(v / plateau - 1) < 1e-6 for v in fe) else 'other'))
