#!/usr/bin/env python3
"""Frame-level critical path from a jobprof timeline (crates/aw2-game/src/jobprof.rs).

Usage: jobprof_frame.py FILE [TOP]

Kinds: 0 sys, 1 grp, 2 graph, 3 job, 4 wait-handle, 5 ext (id 1 cv-wait, 2 present, 3 obj-wait),
6 mark (render-thread command: id = handler VA, sys = opcode).
"""
import bisect, collections, struct, sys

path = sys.argv[1]
top = int(sys.argv[2]) if len(sys.argv) > 2 else 15
data = open(path, 'rb').read()
ev = [struct.unpack_from('<QQQHHHH', data, o) for o in range(0, len(data) - 31, 32)]
T0, T1, ID, SYS, GID, THR, KIND = range(7)
t_lo = min(e[T0] for e in ev); t_hi = max(e[T1] for e in ev)
# Events that started before the recording window (long waits, streaming jobs) stretch
# min(t0)..max(t1) well past the real window; use the Present-to-Present range instead.
_p = sorted(e[T0] for e in ev if e[KIND] == 5 and e[ID] == 2)
if len(_p) > 2:
    t_lo, t_hi = _p[0], _p[-1]
span = t_hi - t_lo
print(f"{len(ev)} events, span {span/1e9:.3f}s")

by_kind = collections.defaultdict(list)
for e in ev:
    by_kind[e[KIND]].append(e)

graph_thr = collections.Counter(e[THR] for e in by_kind[2]).most_common(1)
main = graph_thr[0][0] if graph_thr else None
rthr = collections.Counter(e[THR] for e in by_kind[6]).most_common(1)
render = rthr[0][0] if rthr else None
pres = sorted([e for e in by_kind[5] if e[ID] == 2], key=lambda e: e[T0])
pthr = collections.Counter(e[THR] for e in pres).most_common(1)
print(f"main thr{main}  render thr{render}  present thr{pthr[0][0] if pthr else '?'}  presents {len(pres)} "
      f"({len(pres)/(span/1e9):.1f}/s)")

def pct(x):
    return f"{100*x/span:5.1f}%"

def union(iv):
    iv = sorted(iv); tot = 0; cs = ce = None
    for s, e in iv:
        if cs is None or s > ce:
            if cs is not None:
                tot += ce - cs
            cs, ce = s, e
        else:
            ce = max(ce, e)
    if cs is not None:
        tot += ce - cs
    return tot

def thread_report(thr, name):
    print(f"\n== {name} thread thr{thr}")
    ext = [e for e in by_kind[5] if e[THR] == thr]
    for i, lab in ((1, 'cv-wait'), (2, 'present'), (3, 'obj-wait')):
        l = [e for e in ext if e[ID] == i]
        if l:
            d = [e[T1] - e[T0] for e in l]
            print(f"  {lab:9} n={len(l):5d} total {pct(sum(d))} avg {sum(d)/len(d)/1e3:7.1f}us max {max(d)/1e3:7.0f}us")
    w = [e for e in by_kind[4] if e[THR] == thr]
    if w:
        d = [e[T1] - e[T0] for e in w]
        print(f"  waithandle n={len(w):5d} total {pct(sum(d))} avg {sum(d)/len(d)/1e3:7.1f}us max {max(d)/1e3:7.0f}us")
    jobs = [e for e in by_kind[3] if e[THR] == thr]
    print(f"  jobs run (union) {pct(union([(e[T0], e[T1]) for e in jobs]))}")
    g = [e for e in by_kind[2] if e[THR] == thr]
    if g:
        print(f"  graphs (union) {pct(union([(e[T0], e[T1]) for e in g]))}")
    m = [e for e in by_kind[6] if e[THR] == thr]
    if m:
        print(f"  render commands (union) {pct(union([(e[T0], e[T1]) for e in m]))}")
        agg = collections.Counter(); n = collections.Counter(); wt = collections.Counter()
        # waits nested inside each command
        ws = sorted([(e[T0], e[T1]) for e in w])
        wstarts = [x[0] for x in ws]
        for e in m:
            k = (e[ID], e[SYS]); agg[k] += e[T1] - e[T0]; n[k] += 1
            i = bisect.bisect_left(wstarts, e[T0])
            while i < len(ws) and ws[i][0] < e[T1]:
                wt[k] += min(ws[i][1], e[T1]) - ws[i][0]; i += 1
        print("  top render commands (handler, opcode): total  n/s  avg  of-which-waiting")
        for k, v in agg.most_common(top):
            print(f"    {k[0]:#x} op{k[1]:<4d} {pct(v)} n/s={n[k]/(span/1e9):6.0f} avg={v/n[k]/1e3:7.1f}us  wait {pct(wt[k])}")

if main is not None:
    thread_report(main, 'main')
if render is not None:
    thread_report(render, 'render')

# For long waits on the render thread: which job finished last before the wait ended?
if render is not None:
    jobs = sorted(by_kind[3], key=lambda e: e[T1])
    ends = [e[T1] for e in jobs]
    crit = collections.Counter(); critn = collections.Counter(); during = collections.Counter()
    waits = [e for e in by_kind[4] if e[THR] == render and e[T1] - e[T0] > 50_000]
    for wv in waits:
        i = bisect.bisect_right(ends, wv[T1]) - 1
        while i >= 0 and (jobs[i][THR] == render or jobs[i][T1] < wv[T0]):
            i -= 1
        if i >= 0:
            j = jobs[i]
            crit[(j[ID], j[THR] == main)] += 1
            critn[j[ID]] += j[T1] - j[T0]
    print(f"\n== {len(waits)} render-thread waits >50us; last job (other thread) to finish before wake:")
    for (jid, on_main), c in crit.most_common(top):
        print(f"    job {jid:#x} {'(main)' if on_main else ''} x{c}  avg dur {critn[jid]/c/1e3:.0f}us")

# Main-thread CV waits: what was the render thread doing when main woke?
if main is not None and render is not None:
    cvs = [e for e in by_kind[5] if e[THR] == main and e[ID] == 1 and e[T1] - e[T0] > 200_000]
    m = sorted([e for e in by_kind[6] if e[THR] == render], key=lambda e: e[T1])
    mends = [e[T1] for e in m]
    woke = collections.Counter()
    for c in cvs:
        i = bisect.bisect_right(mends, c[T1]) - 1
        if i >= 0:
            woke[(m[i][ID], m[i][SYS])] += 1
    print(f"\n== {len(cvs)} main cv-waits >200us; render command that finished just before main woke:")
    for k, c in woke.most_common(8):
        print(f"    {k[0]:#x} op{k[1]} x{c}")
