#!/usr/bin/env python3
"""Render-pass wait analysis for `propagate_render_pass_masks` from a jobprof timeline.

Usage: jobprof_passes.py FILE [TOP]

Uses kinds 7 (pass ready: call start -> pass observed ready, sys = pass index, id = execute
callback VA), 8 (pass execute on the render thread) and 9 (whole call, sys = poll rounds,
id = poll-callback ns), plus kind 3 jobs (id = job run VA) on the other threads.
"""
import bisect, collections, struct, sys

path = sys.argv[1]
top = int(sys.argv[2]) if len(sys.argv) > 2 else 12
data = open(path, 'rb').read()
ev = [struct.unpack_from('<QQQHHHH', data, o) for o in range(0, len(data) - 31, 32)]
T0, T1, ID, SYS, GID, THR, KIND = range(7)
by = collections.defaultdict(list)
for e in ev:
    by[e[KIND]].append(e)
props = sorted(by[9], key=lambda e: e[T0])
if not props:
    sys.exit("no kind-9 propagate events (build without the instrumentation?)")
span = max(e[T1] for e in ev) - min(e[T0] for e in ev)
rthr = props[0][THR]
pres = [e for e in by[5] if e[ID] == 2]
nframes = max(1, len(pres))
us = lambda ns: ns / 1e3

d = [e[T1] - e[T0] for e in props]
poll = [e[ID] for e in props]
print(f"{len(props)} propagate calls on thr{rthr} over {span/1e9:.2f}s ({nframes} presents)")
print(f"  per call: avg {us(sum(d)/len(d)):.0f}us  max {us(max(d)):.0f}us  "
      f"poll-cb avg {us(sum(poll)/len(poll)):.0f}us  rounds avg {sum(e[SYS] for e in props)/len(props):.0f}")
print(f"  share of wall time: {100*sum(d)/span:.1f}%")

ready = sorted(by[7], key=lambda e: e[T1])
execs = by[8]
per_pass = collections.defaultdict(lambda: [0, 0, 0, 0, 0])  # n, wait, exec, last_count, fn
ex_by = {}
for e in execs:
    ex_by[(e[THR], e[T0] // 1000, e[SYS])] = e
call_starts = [e[T0] for e in props]

def call_of(t):
    i = bisect.bisect_right(call_starts, t) - 1
    return props[i] if i >= 0 else None

last_ready = {}
for r in ready:
    c = call_of(r[T1])
    if c is None or r[T0] != c[T0]:
        continue
    p = per_pass[r[SYS]]
    p[0] += 1; p[1] += r[T1] - r[T0]; p[4] = r[ID]
    lr = last_ready.get(c[T0])
    if lr is None or r[T1] > lr[T1]:
        last_ready[c[T0]] = r
for e in execs:
    per_pass[e[SYS]][2] += e[T1] - e[T0]
for r in last_ready.values():
    per_pass[r[SYS]][3] += 1

print("\n pass  n    avg-ready-us  avg-exec-us  last-ready  exec-cb")
for idx, (n, w, x, lastc, fn) in sorted(per_pass.items(), key=lambda kv: -kv[1][1])[:top]:
    if n == 0:
        continue
    print(f" {idx:4d} {n:5d} {us(w/n):12.0f} {us(x/n):12.0f} {lastc:10d}  {fn:#x}")

# Jobs on other threads finishing just before the critical (last-ready) pass became ready.
jobs = sorted((e for e in by[3] if e[THR] != rthr), key=lambda e: e[T1])
jend = [e[T1] for e in jobs]
fin = collections.Counter(); fin_dur = collections.defaultdict(list); fin_lat = collections.defaultdict(list)
for r in last_ready.values():
    i = bisect.bisect_right(jend, r[T1]) - 1
    while i >= 0 and r[T1] - jobs[i][T1] < 30_000:
        j = jobs[i]
        fin[j[ID]] += 1
        fin_dur[j[ID]].append(j[T1] - j[T0])
        fin_lat[j[ID]].append(r[T1] - j[T1])
        i -= 1
print("\njobs ending <=30us before the last pass became ready (count, avg dur, avg end->ready):")
for va, n in fin.most_common(top):
    print(f"  {va:#x}  n={n:4d}  dur {us(sum(fin_dur[va])/n):7.0f}us  lat {us(sum(fin_lat[va])/n):5.1f}us")

# Worker occupancy while the render thread waits inside propagate (excluding pass execution).
workers = sorted({e[THR] for e in jobs})
busy = collections.Counter(); wait_tot = 0
jobs_by_t0 = sorted(jobs, key=lambda e: e[T0])
t0s = [j[T0] for j in jobs_by_t0]
maxdur = max((j[T1] - j[T0] for j in jobs), default=0)
for c in props:
    a, b = c[T0], c[T1]
    wait_tot += b - a
    for k in range(bisect.bisect_left(t0s, a - maxdur), len(jobs_by_t0)):
        j = jobs_by_t0[k]
        if j[T0] >= b:
            break
        if j[T1] > a:
            busy[j[THR]] += min(b, j[T1]) - max(a, j[T0])
print(f"\nworker occupancy during propagate calls ({len(workers)} job threads):")
occ = sum(busy.values()) / max(1, wait_tot)
print(f"  avg busy job threads {occ:.2f}")
for t, v in busy.most_common(top):
    print(f"  thr{t}: {100*v/wait_tot:5.1f}%")
