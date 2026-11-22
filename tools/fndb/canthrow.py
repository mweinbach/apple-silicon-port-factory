"""`fndb can-throw`: which functions can (transitively) raise an exception.

Used by the hook-safety policy (docs/design/ADVICE_SEH.md): a Rust replacement
whose original can reach a throw must say so in its `throws:` annotation.

Seeds (bit classes):
  CXX      1   _CxxThrowException (via the j_ thunk 0x14183acce or the import) and
               __ExceptionPtrRethrow: engine exceptions, e.g. ObjectStreamException
  CONTRACT 2   msvcp _Xbad_function_call / _Xlength_error / _Xbad_alloc: STL
               contract violations, treated as abort-equivalent
  SEH      4   KERNEL32 RaiseException (PhysX)
  UNKNOWN  8   `call reg` / call through a non-IAT global function pointer
               (target unknown)
  STD     16   msvcp _Xout_of_range / _Xinvalid_argument / _Xregex_error /
               _Throw_Cpp_error: std::exception types that catch(std::exception)
               or catch(...) may handle

Propagation goes backwards over the `calls` table (call + tail) to a fixpoint.
`mask_direct` uses direct edges only. `mask_all` also follows vcalls
(`call/jmp [reg+disp]`, from `fnsig icalls`), over-approximated as "any
vftable whose slot disp/8 targets a throwing function". Imports from other game
DLLs (cohtml, protobuf, ...) are assumed not to throw across the boundary.
"""
import collections
import os
import subprocess

CXX, CONTRACT, SEH, UNKNOWN, STD = 1, 2, 4, 8, 16
BIT_NAMES = [(CXX, 'cxx'), (CONTRACT, 'contract'), (SEH, 'seh'), (UNKNOWN, 'unknown-indirect'), (STD, 'std')]

J_CXX_THROW = 0x14183acce

SEED_IMPORTS = {
    '_CxxThrowException': CXX,
    '__ExceptionPtrRethrow': CXX,
    '?_Xbad_function_call@std@@YAXXZ': CONTRACT,
    '?_Xlength_error@std@@YAXPEBD@Z': CONTRACT,
    '?_Xbad_alloc@std@@YAXXZ': CONTRACT,
    'RaiseException': SEH,
    '?_Xout_of_range@std@@YAXPEBD@Z': STD,
    '?_Xinvalid_argument@std@@YAXPEBD@Z': STD,
    '?_Xregex_error@std@@YAXW4error_type@regex_constants@1@@Z': STD,
    '?_Throw_Cpp_error@std@@YAXH@Z': STD,
}

# Vtable displacements beyond this are not slot calls (struct fn-pointer fields at large offsets
# are still matched if they happen to be a multiple of 8 below it; that is part of the
# over-approximation).
MAX_SLOT_DISP = 0x2000

# Functions whose indirect calls were audited and cannot throw: they are not seeded UNKNOWN.
# Keep each entry justified (docs/design/ADVICE_SEH.md); this is the only manual input.
NOTHROW_INDIRECT = {
    # mimalloc 2.1.2 (C): `_mi_malloc_generic` calls the `deferred_free` hook at 0x143a63258 only
    # when non-null; nothing in .text stores to it (only 4 rip-relative loads, all in mimalloc).
    0x142537280: 'mimalloc _mi_malloc_generic: deferred_free hook never registered',
    0x142535600: 'mimalloc _mi_deferred_free: deferred_free hook never registered',
    # mimalloc `_mi_vfprintf`-style message path: calls the output hook 0x143a632e8 or the default
    # stderr writer 0x142538090; the hook is only stored by mimalloc-internal functions (assumed
    # never a throwing C++ callback).
    0x142537b60: 'mimalloc message output: mi_output_fun (C callback)',
    # mimalloc `_mi_error_message` ("mimalloc: error: "): calls the error hook 0x143a637a8
    # (mi_register_error) on heap corruption / OOM only.
    0x1425386a0: 'mimalloc _mi_error_message: mi_error_fun (C callback)',
}


def mask_names(m):
    return ','.join(n for b, n in BIT_NAMES if m & b) or 'none'


def _icalls(db, out_dir, exe):
    """Run `fnsig icalls` over all functions (with fragments) -> {va: [tokens]}."""
    import libcorpus
    fnsig = libcorpus.ensure_fnsig()
    os.makedirs(out_dir, exist_ok=True)
    lst = os.path.join(out_dir, 'icalls.funcs')
    frags = collections.defaultdict(list)
    for fv, a, b in db.execute('SELECT func_va, va, "end" FROM fragments'):
        frags[fv].append('%x-%x' % (a, b))
    with open(lst, 'w') as f:
        for va, end in db.execute('SELECT va, "end" FROM functions ORDER BY va'):
            f.write('\t'.join(['%x' % va, '%x' % end] + frags.get(va, [])) + '\n')
    out = os.path.join(out_dir, 'icalls.tsv')
    subprocess.run([fnsig, 'icalls', exe, lst, out], check=True, stderr=subprocess.DEVNULL)
    res = {}
    with open(out) as f:
        next(f)
        for line in f:
            va, toks = line.rstrip('\n').split('\t')
            res[int(va, 16)] = toks.split(',')
    return res


def build(db, out_dir, exe, log=print):
    iat = {va: func for va, func in db.execute('SELECT iat_va, func FROM imports')}
    callers = collections.defaultdict(set)
    for caller, callee in db.execute('SELECT caller, callee FROM calls'):
        callers[callee].add(caller)

    seed = collections.defaultdict(int)
    why = {}
    for caller in callers.get(J_CXX_THROW, ()):
        seed[caller] |= CXX
        why.setdefault(caller, ('seed', 'j__CxxThrowException'))
    for caller, func in db.execute('SELECT caller, func FROM import_calls'):
        b = SEED_IMPORTS.get(func)
        if b:
            seed[caller] |= b
            why.setdefault(caller, ('seed', func))

    icalls = _icalls(db, out_dir, exe)
    slot_users = collections.defaultdict(set)   # slot -> functions calling [reg + slot*8]
    for va, toks in icalls.items():
        audited = va in NOTHROW_INDIRECT
        for t in toks:
            if t == 'r':
                if not audited:
                    seed[va] |= UNKNOWN
                    why.setdefault(va, ('seed', 'call reg'))
            elif t[0] == 'g':
                g = int(t[1:], 16)
                func = iat.get(g)
                if func is None:
                    if not audited:
                        seed[va] |= UNKNOWN
                        why.setdefault(va, ('seed', 'call [global 0x%x]' % g))
                elif func in SEED_IMPORTS:
                    seed[va] |= SEED_IMPORTS[func]
                    why.setdefault(va, ('seed', func))
            elif t[0] == 'v':
                d = int(t[1:], 16)
                if d % 8 == 0 and d < MAX_SLOT_DISP:
                    slot_users[d // 8].add(va)
    target_slots = collections.defaultdict(set)  # function -> slots it occupies in some vftable
    for slot, target in db.execute('SELECT slot, target FROM vftable_slots'):
        if slot in slot_users:
            target_slots[target].add(slot)

    def propagate(use_vcalls):
        mask = dict(seed)
        via = {}
        work = collections.deque(mask)
        while work:
            f = work.popleft()
            m = mask[f]
            edges = [(c, ('call', f)) for c in callers.get(f, ())]
            if use_vcalls:
                for s in target_slots.get(f, ()):
                    edges += [(u, ('vcall', s, f)) for u in slot_users[s]]
            for c, w in edges:
                old = mask.get(c, 0)
                if old | m != old:
                    mask[c] = old | m
                    via.setdefault(c, w)
                    work.append(c)
        return mask, via

    mask_d, _ = propagate(False)
    mask_a, via_a = propagate(True)
    db.execute('DROP TABLE IF EXISTS can_throw')
    db.execute('CREATE TABLE can_throw(va INTEGER PRIMARY KEY, mask_direct INTEGER, mask_all INTEGER, '
               'via_kind TEXT, via_va INTEGER, via_slot INTEGER)')
    rows = []
    for va in set(mask_a) | set(mask_d):
        w = why.get(va) if va in seed else None
        v = via_a.get(va)
        if w:
            kind, vva, vslot = 'seed:' + w[1], None, None
        elif v and v[0] == 'call':
            kind, vva, vslot = 'call', v[1], None
        elif v:
            kind, vva, vslot = 'vcall', v[2], v[1]
        else:
            kind, vva, vslot = None, None, None
        rows.append((va, mask_d.get(va, 0), mask_a.get(va, 0), kind, vva, vslot))
    db.executemany('INSERT INTO can_throw VALUES (?,?,?,?,?,?)', rows)
    db.execute("INSERT OR REPLACE INTO meta(key, value) VALUES ('can_throw', 'built')")
    db.commit()
    n = db.execute('SELECT COUNT(*) FROM functions').fetchone()[0]
    def count(col, bits):
        return db.execute('SELECT COUNT(*) FROM can_throw WHERE %s & ?' % col, (bits,)).fetchone()[0]
    log('can-throw: %d functions; direct calls: %d can throw cxx/seh/std, %d contract-only-or-more; '
        'with vcalls: %d cxx/seh/std, %d unknown-indirect' % (
            n, count('mask_direct', CXX | SEH | STD), count('mask_direct', CONTRACT),
            count('mask_all', CXX | SEH | STD), count('mask_all', UNKNOWN)))


def built(db):
    return db.execute("SELECT 1 FROM sqlite_master WHERE name='can_throw'").fetchone() is not None


def lookup(db, va):
    """(mask_direct, mask_all) for `va` (0, 0 if it can't reach any seed)."""
    r = db.execute('SELECT mask_direct, mask_all FROM can_throw WHERE va=?', (va,)).fetchone()
    return (r[0], r[1]) if r else (0, 0)


def witness(db, va, limit=12):
    """Chain of (va, how) from `va` to a seed, following the recorded first propagation edge."""
    out = []
    seen = set()
    while va is not None and va not in seen and len(out) < limit:
        seen.add(va)
        r = db.execute('SELECT via_kind, via_va, via_slot FROM can_throw WHERE va=?', (va,)).fetchone()
        if r is None:
            break
        kind, vva, slot = r
        if kind is None:
            break
        if kind.startswith('seed:'):
            out.append((va, kind[5:]))
            break
        out.append((va, 'call' if kind == 'call' else 'vcall slot %d (0x%x) ->' % (slot, slot * 8)))
        va = vva
    return out


# ---------------------------------------------------------------------------------------------
# Exception-handling shape of a function (from its own unwind info), for the loader lint.

# Handlers by the in-exe stub that dispatches to them (db names; GS variants identified by what
# they call after __GSHandlerCheckCommon).
H_FH4, H_FH3, H_SEH = 0x14183ac80, 0x14183ace6, 0x14183aca4
H_GS_FH4, H_GS, H_GS_FH3 = 0x1418353f0, 0x141835344, 0x1418356f4


def eh_info(pe, va):
    """'none' | 'cleanup' (unwind map / __finally only) | 'catch' (try map / __except) |
    'noexcept' (FH4 NoExcept: a throw through it calls terminate) | 'unknown:<handler>'."""
    import bisect
    import struct
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
    for _ in range(64):            # follow chained infos to the primary
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

    def fh4(fi_rva):
        hdr = d[pe.off(fi_rva)]
        if hdr & 0x10:
            return 'catch'
        if hdr & 0x40:
            return 'noexcept'
        return 'cleanup' if hdr & 0x08 else 'none'

    def fh3(fi_rva):
        o = pe.off(fi_rva)
        ntry = struct.unpack_from('<I', d, o + 12)[0]
        maxstate = struct.unpack_from('<i', d, o + 4)[0]
        return 'catch' if ntry else ('cleanup' if maxstate > 0 else 'none')

    if h in (H_FH4, H_GS_FH4):
        return fh4(struct.unpack_from('<I', d, data)[0])
    if h in (H_FH3, H_GS_FH3):
        return fh3(struct.unpack_from('<I', d, data)[0])
    if h == H_GS:
        return 'none'
    if h == H_SEH:
        n = struct.unpack_from('<I', d, data)[0]
        targets = [struct.unpack_from('<I', d, data + 4 + 16 * k + 12)[0] for k in range(min(n, 256))]
        return 'catch' if any(targets) else ('cleanup' if n else 'none')
    return 'unknown:0x%x' % h


THROWS_TSV_HEADER = ('# generated by `tools/fndb.sh sync` (tools/fndb/canthrow.py); read by the loader\'s\n'
                     '# hook-safety lint (aw2-loader hooks.rs, docs/design/ADVICE_SEH.md). Bits: cxx=1\n'
                     '# contract=2 seh=4 unknown-indirect=8 std=16. mask_direct follows direct calls only,\n'
                     '# mask_all also vcalls (over-approximated by vftable slot). eh: none|cleanup|catch|noexcept.\n'
                     'va\tmask_direct\tmask_all\teh\twitness\n')


def write_throws_tsv(db, pe, vas, path):
    rows = []
    for va in sorted(vas):
        md, ma = lookup(db, va)
        w = witness(db, va)
        wit = ' > '.join('0x%x' % a for a, _ in w)
        if w:
            wit += ' (%s)' % w[-1][1]
        rows.append('0x%x\t%d\t%d\t%s\t%s\n' % (va, md, ma, eh_info(pe, va), wit))
    new = THROWS_TSV_HEADER + ''.join(rows)
    old = open(path).read() if os.path.exists(path) else None
    if new != old:
        with open(path, 'w') as f:
            f.write(new)
    return len(rows), new != old
