#!/usr/bin/env python3
"""CPU+GPU frame timeline from a jobprof capture (crates/aw2-game/src/jobprof.rs).

Usage: jobprof_gpu.py FILE

Needs a capture with K_GPU (Metal commit feedback) and K_QOP (D3D12 queue operations) events.
Frames are the render thread's Present-to-Present intervals inside the recording window
(long events that started before the window are clipped, so per-second rates are real).

Reports:
  * render-thread time per frame: op0 (frame render), op147 (Present + GPU-lag fence spin),
    other commands, idle;
  * GPU occupancy per frame: union of commit [GPU start, GPU end] intervals, split per queue,
    and the idle time split into "starved" (nothing committed and unfinished) versus "blocked"
    (committed work waiting to start: a queue wait on a fence or a busy other queue);
  * CPU->GPU latency: ExecuteCommandLists call -> CS-thread commit -> GPU start -> GPU end;
  * which GPU commit end released each op147 fence spin.
"""
import bisect, collections, struct, sys

K_EXT, K_MARK, K_GPU, K_QOP = 5, 6, 10, 11
QK = {0: 'direct', 2: 'compute', 3: 'copy'}
QOP = {0: 'ecl', 1: 'signal', 2: 'wait', 3: 'signal-exec', 4: 'wait-exec'}

data = open(sys.argv[1], 'rb').read()
ev = [struct.unpack_from('<QQQHHHH', data, o) for o in range(0, len(data) - 31, 32)]
T0, T1, ID, SYS, GID, THR, KIND = range(7)

marks = [e for e in ev if e[KIND] == K_MARK]
rthr = collections.Counter(e[THR] for e in marks).most_common(1)[0][0]
pres = sorted(e[T0] for e in ev if e[KIND] == K_EXT and e[ID] == 2 and e[THR] == rthr)
gpu = sorted((e[T0], e[T1], e[ID], e[SYS], e[GID]) for e in ev if e[KIND] == K_GPU and e[T1] >= e[T0] > 0)
qops = sorted((e[T0], e[SYS] >> 8, e[SYS] & 0xff, e[ID], e[GID], e[THR]) for e in ev if e[KIND] == K_QOP)
if len(pres) < 3 or not gpu:
    sys.exit(f"need presents and K_GPU events (presents {len(pres)}, gpu {len(gpu)})")

# GPU feedback arrives late: stop at the last present whose frame the GPU fully reported.
gpu_hi = max(g[1] for g in gpu)
pres = [p for p in pres if p <= gpu_hi - 30_000_000] or pres
lo, hi = pres[0], pres[-1]
W = hi - lo
nf = len(pres) - 1
print(f"{len(ev)} events; window {W/1e9:.3f}s, {nf} frames = {nf/(W/1e9):.1f} fps "
      f"({W/nf/1e6:.2f} ms/frame); render thr{rthr}")

def clip(a, b, l=lo, h=hi):
    return max(0, min(b, h) - max(a, l))

def union(iv):
    out = []
    for s, e in sorted(iv):
        if out and s <= out[-1][1]:
            out[-1][1] = max(out[-1][1], e)
        else:
            out.append([s, e])
    return out

ms = lambda x: x / nf / 1e6

# --- render thread ------------------------------------------------------------------------
rt = collections.Counter()
rmarks = sorted((e[T0], e[T1], e[SYS]) for e in marks if e[THR] == rthr)
for s, e, op in rmarks:
    rt['op0' if op == 0 else 'op147' if op == 147 else 'other cmds'] += clip(s, e)
busy_r = sum(clip(s, e) for s, e in union([(s, e) for s, e, _ in rmarks]))
present = sum(clip(e[T0], e[T1]) for e in ev if e[KIND] == K_EXT and e[ID] == 2 and e[THR] == rthr)
print("\n== render thread per frame")
for k in ('op0', 'op147', 'other cmds'):
    print(f"  {k:10} {ms(rt[k]):6.2f} ms  {100*rt[k]/W:5.1f}%")
print(f"  {'(present)':10} {ms(present):6.2f} ms  (inside op147)")
print(f"  {'no command':10} {ms(W-busy_r):6.2f} ms  {100*(W-busy_r)/W:5.1f}%")

# --- GPU occupancy ------------------------------------------------------------------------
bad = sum(1 for g in gpu if g[0] < g[2])
print(f"\n== GPU commits: {len(gpu)} ({len(gpu)/(W/1e9):.0f}/s); start before commit: {bad}")
gu = union([(s, e) for s, e, *_ in gpu])
gbusy = sum(clip(s, e) for s, e in gu)
print(f"  occupied (union of commit intervals) {ms(gbusy):.2f} ms/frame = {100*gbusy/W:.1f}%")
for k in sorted({g[3] for g in gpu}):
    u = union([(s, e) for s, e, _, kk, _ in gpu if kk == k])
    b = sum(clip(s, e) for s, e in u)
    n = sum(1 for g in gpu if g[3] == k and lo <= g[0] < hi)
    print(f"    {QK.get(k, k):8} {ms(b):6.2f} ms/frame  {100*b/W:5.1f}%  commits/frame {n/nf:5.1f}")

# Idle gaps: starved (no committed-unfinished work) vs blocked (committed work not started).
commits = sorted((c, s, e, k) for s, e, c, k, _ in gpu)
cts = [c[0] for c in commits]
starved = blocked = 0
starved_gaps = []
gaps = [(a[1], b[0]) for a, b in zip(gu, gu[1:])]
for gs, ge in gaps:
    if clip(gs, ge) == 0:
        continue
    # committed before the gap ended and started at/after the gap end
    j = bisect.bisect_right(cts, gs)
    pend = any(commits[i][1] >= ge for i in range(max(0, j - 64), j))
    if pend:
        blocked += clip(gs, ge)
        continue
    # starved until the next commit lands (then possibly blocked until start)
    k = bisect.bisect_left(cts, gs)
    c_next = cts[k] if k < len(cts) else ge
    starved += clip(gs, min(ge, c_next))
    blocked += clip(min(ge, c_next), ge)
    starved_gaps.append((clip(gs, min(ge, c_next)), gs, min(ge, c_next)))
idle = W - gbusy
print(f"  idle {ms(idle):.2f} ms/frame: starved {ms(starved):.2f} (no committed work), "
      f"blocked {ms(blocked):.2f} (committed, not started)")
sg = sorted(starved_gaps, reverse=True)
if sg:
    sizes = sorted(x[0] for x in sg)
    big = sum(x for x in sizes if x > 500_000)
    print(f"  starved gaps: n/frame {len(sg)/nf:.1f}, >0.5 ms total {ms(big):.2f} ms/frame, "
          f"p50 {sizes[len(sizes)//2]/1e3:.0f} us, max {sizes[-1]/1e3:.0f} us")

# --- latency ------------------------------------------------------------------------------
def pct(a, q):
    a = sorted(a)
    return a[int(q * (len(a) - 1))] if a else float('nan')

print("\n== commit latency (us): commit->GPU start, GPU duration")
for k in sorted({g[3] for g in gpu}):
    q = [(s - c) / 1e3 for s, e, c, kk, _ in gpu if kk == k and lo <= c < hi]
    d = [(e - s) / 1e3 for s, e, c, kk, _ in gpu if kk == k and lo <= c < hi]
    if q:
        print(f"  {QK.get(k, k):8} queue p50 {pct(q,.5):7.0f} p90 {pct(q,.9):7.0f}   dur p50 {pct(d,.5):6.0f} p90 {pct(d,.9):6.0f}  sum {sum(d)/1e3/nf:.2f} ms/frame")

ecl = [(t, k) for t, op, k, *_ in qops if op == 0 and lo <= t < hi]
print(f"  ExecuteCommandLists calls/frame: " + ", ".join(
    f"{QK.get(k, k)} {n/nf:.1f}" for k, n in sorted(collections.Counter(k for _, k in ecl).items())))
qc = collections.Counter((QOP.get(op, op), QK.get(k, k)) for t, op, k, *_ in qops if lo <= t < hi)
print("  queue ops/frame: " + ", ".join(f"{o}/{k} {n/nf:.1f}" for (o, k), n in sorted(qc.items())))

# --- what released each op147 spin --------------------------------------------------------
print("\n== op147 (Present + fence spin) vs GPU completion")
ends = sorted((e, s, k) for s, e, c, k, _ in gpu)
eend = [x[0] for x in ends]
rel = []
for s, e, op in rmarks:
    if op != 147 or not (lo <= s < hi):
        continue
    j = bisect.bisect_right(eend, e) - 1
    if j >= 0:
        rel.append(((e - eend[j]) / 1e3, (e - s) / 1e3))
if rel:
    print(f"  op147 n={len(rel)} dur p50 {pct([r[1] for r in rel],.5):.0f} us; "
          f"op147 end - latest GPU commit end p50 {pct([r[0] for r in rel],.5):.0f} us p90 {pct([r[0] for r in rel],.9):.0f} us")

# --- per-frame table ----------------------------------------------------------------------
print("\n frame  period  op0  op147  gpu_occ  starved  blocked   (ms)")
for i in range(min(nf, 12)):
    a, b = pres[i], pres[i + 1]
    o0 = sum(clip(s, e, a, b) for s, e, op in rmarks if op == 0)
    o147 = sum(clip(s, e, a, b) for s, e, op in rmarks if op == 147)
    occ = sum(clip(s, e, a, b) for s, e in gu)
    st = sum(clip(gs, ge, a, b) for _, gs, ge in starved_gaps)
    print(f" {i:5d} {(b-a)/1e6:7.2f} {o0/1e6:5.2f} {o147/1e6:5.2f} {occ/1e6:7.2f} {st/1e6:7.2f} {(b-a-occ-st)/1e6:7.2f}")
