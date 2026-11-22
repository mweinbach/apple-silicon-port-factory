#!/usr/bin/env python3
"""Analyze out/hitch/jobprof-<pid>.bin (see crates/aw2-game/src/jobprof.rs).

Usage: jobprof.py FILE [TOP]
"""
import struct, sys, collections, bisect

K = {0: 'sys', 1: 'grp', 2: 'graph', 3: 'job'}
path = sys.argv[1]
top = int(sys.argv[2]) if len(sys.argv) > 2 else 25
data = open(path, 'rb').read()
ev = [struct.unpack_from('<QQQHHHH', data, o) for o in range(0, len(data) - 31, 32)]
print(f'{len(ev)} events')
if not ev:
    sys.exit()
t_lo = min(e[0] for e in ev)
t_hi = max(e[1] for e in ev)
span = (t_hi - t_lo) / 1e9
print(f'span {span:.3f}s')

graphs = sorted([e for e in ev if e[6] == 2], key=lambda e: e[0])
sysev = [e for e in ev if e[6] in (0, 1)]
jobs = [e for e in ev if e[6] == 3]

# --- graph runs
gd = collections.defaultdict(list)
for e in graphs:
    gd[(e[2], e[5])].append((e[1] - e[0]) / 1e3)
print('\n== graph runs (start_scheduling) by sched_graph ptr, thread')
for (gid, thr), v in sorted(gd.items(), key=lambda kv: -sum(kv[1])):
    v.sort()
    print(f'  graph {gid:#x} thr{thr}: n={len(v)} per_s={len(v)/span:.1f} avg={sum(v)/len(v):.0f}us p50={v[len(v)//2]:.0f}us '
          f'max={v[-1]:.0f}us total={sum(v)/1e3/span*100:.1f}% of wall')

# --- thread busy
busy = collections.Counter()
for e in sysev:
    busy[e[5]] += e[1] - e[0]
print('\n== system time per thread (% of span)')
for thr, t in busy.most_common():
    print(f'  thr{thr}: {t/1e9/span*100:.1f}%')

# --- per system totals
tot = collections.defaultdict(lambda: [0, 0, collections.Counter()])
for e in sysev:
    k = (e[2], e[3])
    tot[k][0] += e[1] - e[0]
    tot[k][1] += 1
    tot[k][2][e[5]] += e[1] - e[0]
print(f'\n== top {top} systems by total time')
for (fn, sid), (t, n, per) in sorted(tot.items(), key=lambda kv: -kv[1][0])[:top]:
    thr = ', '.join(f'thr{a}:{b/t*100:.0f}%' for a, b in per.most_common(3))
    print(f'  {fn:#x} sys{sid:<4} {t/1e9/span*100:6.2f}%  n/s={n/span:7.0f}  avg={t/n/1e3:7.1f}us  [{thr}]')

# --- critical-path attribution inside each graph run on its own thread:
# while the graph thread is not inside a system, charge the gap to whatever systems
# are running elsewhere (split evenly); if nothing runs anywhere, charge 'sched-gap'.
by_thr = collections.defaultdict(list)
for e in sysev:
    by_thr[e[5]].append(e)
for v in by_thr.values():
    v.sort()
starts = sorted(sysev, key=lambda e: e[0])
st0 = [e[0] for e in starts]
blame = collections.Counter()
own = collections.Counter()
gap_none = 0
wall = 0
STEP = 2000  # ns sampling resolution
for g in graphs:
    thr = g[5]
    mine = by_thr.get(thr, [])
    i0 = bisect.bisect_left([e[0] for e in mine], g[0]) if mine else 0
    # active intervals of systems overlapping g (any thread)
    j = bisect.bisect_left(st0, g[0] - 50_000_000)
    cand = [e for e in starts[j:bisect.bisect_right(st0, g[1])] if e[1] > g[0]]
    t = g[0]
    while t < g[1]:
        wall += STEP
        running_here = [e for e in cand if e[5] == thr and e[0] <= t < e[1]]
        if running_here:
            inner = min(running_here, key=lambda e: e[1] - e[0])
            own[(inner[2], inner[3])] += STEP
        else:
            others = [e for e in cand if e[5] != thr and e[0] <= t < e[1]]
            if others:
                # innermost per thread
                per = {}
                for e in others:
                    if e[5] not in per or (e[1] - e[0]) < (per[e[5]][1] - per[e[5]][0]):
                        per[e[5]] = e
                for e in per.values():
                    blame[(e[2], e[3])] += STEP / len(per)
            else:
                gap_none += STEP
        t += STEP
    if wall > 3e9:
        break
print(f'\n== graph-thread breakdown over {wall/1e6:.0f} ms of graph wall time')
own_t = sum(own.values()); bl_t = sum(blame.values())
print(f'  running systems itself: {own_t/wall*100:.1f}%   waiting on other threads: {bl_t/wall*100:.1f}%   '
      f'nothing running anywhere (sched/other): {gap_none/wall*100:.1f}%')
print(f'\n== top {top} systems the graph thread runs itself (% of graph wall)')
for (fn, sid), t in own.most_common(top):
    print(f'  {fn:#x} sys{sid:<4} {t/wall*100:6.2f}%')
print(f'\n== top {top} systems the graph thread waits on (% of graph wall)')
for (fn, sid), t in blame.most_common(top):
    print(f'  {fn:#x} sys{sid:<4} {t/wall*100:6.2f}%')
