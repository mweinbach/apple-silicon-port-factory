#!/usr/bin/env python3
"""Summarise macOS `sample` output for the hybrid loader: which threads are
actually busy, and where.

    tools/perf/busy.py out/perf/sample_1.txt [--top 8] [--paths 6] [--idle 0x7ff8...,...]

For every thread, "busy" counts samples whose leaf frame is not an idle wait.
Under Rosetta most blocking syscalls show up as one `??? (in <unknown binary>)`
leaf address. The leaf address seen in the most threads is treated as idle
automatically; add more with --idle. Leaves that name a known blocking host
function (mach_msg, __psynch_cvwait, kevent, ...) are idle too.

Interpreting results:
* `com.apple.rosetta.exceptionserver` busy means faults or self-modifying code.
  A store to a page holding translated code is the classic cause (see AGENTS.md
  "No mutable data on executable pages").
* Busy guest threads whose leaves are `Rosetta Runtime Routines` called from
  `Rosetta JIT` are translating (or re-translating) code.
* Host frames (aw2-loader) are symbolized by `sample` itself (Rust-mangled).
"""
import argparse
import collections
import re

IDLE_NAMES = ('mach_msg', '__psynch_cvwait', '__psynch_mutexwait', '__semwait', 'kevent', '__ulock_wait',
              '__workq_kernreturn', 'nanosleep', '__select', '__wait4', 'semaphore_wait', '__sigsuspend')
NODE = re.compile(r'^([ +!:|]*)(\d+) (.*)$')


def addr(label):
    m = re.search(r'\[(0x[0-9a-f]+)\]', label)
    return m.group(1) if m else label


def parse(path):
    lines = open(path, errors='replace').read().splitlines()
    start = next(i for i, l in enumerate(lines) if l.startswith('Call graph:'))
    end = next((i for i, l in enumerate(lines) if l.startswith('Total number in stack')), len(lines))
    threads, cur, base = [], None, 0
    for l in lines[start + 1:end]:
        m = NODE.match(l)
        if not m:
            continue
        depth, cnt, label = len(m.group(1)), int(m.group(2)), m.group(3).strip()
        if label.startswith('Thread_'):
            cur = {'name': label, 'total': cnt, 'nodes': []}
            threads.append(cur)
            base = depth
        elif cur is not None:
            cur['nodes'].append((depth - base, cnt, label))
    return threads


def leaves(nodes):
    """Yield (self_count, path) for every node with self samples."""
    stack = []
    for i, (d, c, lab) in enumerate(nodes):
        while stack and stack[-1][0] >= d:
            stack.pop()
        stack.append((d, lab))
        kids, first, k = 0, None, i + 1
        while k < len(nodes) and nodes[k][0] > d:
            if first is None:
                first = nodes[k][0]
            if nodes[k][0] == first:
                kids += nodes[k][1]
            k += 1
        if c - kids > 0:
            yield c - kids, [l for _, l in stack]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('sample')
    ap.add_argument('--top', type=int, default=8, help='threads to show')
    ap.add_argument('--paths', type=int, default=6, help='hot leaf paths per thread')
    ap.add_argument('--depth', type=int, default=8, help='frames per path')
    ap.add_argument('--idle', default='', help='extra idle leaf addresses (comma separated)')
    a = ap.parse_args()

    threads = parse(a.sample)
    per_thread_leaves = []
    seen_in = collections.Counter()
    for t in threads:
        ls = list(leaves(t['nodes']))
        per_thread_leaves.append(ls)
        for leaf_addr in {addr(p[-1]) for _, p in ls}:
            seen_in[leaf_addr] += 1
    idle = set(x for x in a.idle.split(',') if x)
    if seen_in:
        common, n = seen_in.most_common(1)[0]
        if n * 2 >= len(threads):
            idle.add(common)

    rows = []
    for t, ls in zip(threads, per_thread_leaves):
        busy = collections.Counter()
        total_busy = 0
        for c, p in ls:
            leaf = p[-1]
            if addr(leaf) in idle or any(n in leaf for n in IDLE_NAMES):
                continue
            total_busy += c
            busy[' <- '.join(reversed(p[-a.depth:]))] += c
        rows.append((total_busy, t, busy))
    rows.sort(key=lambda r: -r[0])
    print(f'{len(threads)} threads; idle leaves: {", ".join(sorted(idle)) or "-"}')
    for total_busy, t, busy in rows[:a.top]:
        print(f"=== {t['name']}  samples={t['total']}  busy={total_busy} ({100 * total_busy / max(t['total'], 1):.0f}%)")
        for p, c in busy.most_common(a.paths):
            print(f'  {c:7d}  {p[:500]}')


if __name__ == '__main__':
    main()
