#!/usr/bin/env python3
"""Inside-view of hot systems: jobprof_sys.py FILE SYSID [SYSID...]"""
import struct, sys, collections, bisect
data = open(sys.argv[1], 'rb').read()
want = {int(s) for s in sys.argv[2:]}
ev = [struct.unpack_from('<QQQHHHH', data, o) for o in range(0, len(data) - 31, 32)]
jobs = sorted([e for e in ev if e[6] == 3])
j0 = [e[0] for e in jobs]
for sid in sorted(want):
    calls = [e for e in ev if e[6] in (0, 1) and e[3] == sid]
    if not calls:
        continue
    by_thr = collections.Counter(e[5] for e in calls)
    tot = sum(e[1] - e[0] for e in calls)
    helped = 0
    other = collections.Counter()
    other_thr_time = 0
    nested_ids = collections.Counter()
    for c in calls:
        i = bisect.bisect_left(j0, c[0])
        k = i
        while k < len(jobs) and jobs[k][0] < c[1]:
            e = jobs[k]
            ov = min(e[1], c[1]) - max(e[0], c[0])
            if e[5] == c[5]:
                helped += ov
                nested_ids[e[2]] += ov
            else:
                other[e[2]] += ov
                other_thr_time += ov
            k += 1
    print(f'sys{sid} fn={calls[0][2]:#x}: n={len(calls)} avg={tot/len(calls)/1e3:.0f}us threads={dict(by_thr.most_common(4))}')
    print(f'  same-thread nested jobs: {helped/tot*100:.1f}% of its time; other threads running jobs meanwhile: {other_thr_time/tot:.2f} threads avg')
    for jid, t in nested_ids.most_common(5):
        print(f'    nested job {jid:#x}: {t/tot*100:.1f}%')
    for jid, t in other.most_common(5):
        print(f'    concurrent job {jid:#x}: {t/tot:.2f} thr-avg')
    # duration distribution
    d = sorted((e[1] - e[0]) / 1e3 for e in calls)
    print(f'  dur p10={d[len(d)//10]:.0f} p50={d[len(d)//2]:.0f} p90={d[len(d)*9//10]:.0f} us')
