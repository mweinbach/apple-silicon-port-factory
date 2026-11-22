"""owner.py - evidence-based ownership classification for fndb (stdlib only).

Every function gets a list of evidence items (owner, lib, score, src, detail); scores are combined
per owner with a noisy-or, conflicts between owners lower the confidence, and the result is
written to functions.owner / owner_src / owner_confidence plus the owner_evidence table.

    owner  : remedy | thirdparty | crt | unknown
    lib    : library for thirdparty/crt (std, rapidjson, sparsehash, Luau, AK, ...) or 'remedy'

Evidence sources, roughly in decreasing strength (see docs/FNDB.md for measured precision):

    override     db/owner_overrides.tsv (manual; never overwritten)                     1.00
    name         export / funcsig / vftable class / CRT anchor / Ghidra FID name        0.60-0.99
    srcfile      __FILE__ path under middleware/ (Wwise, PhysX, OTel ...)               0.95
    crt_region   inside the static-CRT block around the entry point                     0.90
    string       references a string literal found only in one library's sources        0.60-0.95
    const        uses constants found only in one library (ref objects / source hex)    0.50-0.80
    refsig       same distinctive feature set as a clang-cl reference function          0.75
    refvals      same field offsets + flag immediates as a clang-cl reference function  0.55-0.75
    stl_helper   tiny noreturn wrapper around msvcp140 std::_X* throw helpers           0.85
    vftref       stores the vftable of a class (constructor/destructor inlined)         0.70
    corefstr     references strings otherwise only used by confidently owned functions  0.55-0.70
    dupgroup     identical normalized code to confidently owned functions               0.65-0.75
    neardup      minhash-similar to confidently owned functions                         <=0.70
    smallsim     small function: same value bag + CFG, similar mnemonics to owned code  <=0.70
    callgraph    called only by library X (and calls only X)                            0.60-0.75
    neighbor     nearest confident functions on both sides agree (bounded distance)     0.35-0.70

Template / header-only libraries (std, rapidjson, sparsehash, nlohmann, fmt, ...) are compiled
inside Remedy objects, so layout and caller evidence is weak for them, and library strings or
constants inside big functions that also reference Remedy strings are treated as inlined code.
"""
import bisect
import collections
import difflib
import glob
import math
import os
import random
import re

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, '..', '..'))
OVERRIDES = os.path.join(ROOT, 'db', 'owner_overrides.tsv')
OUT = os.path.join(ROOT, 'out', 'fndb')
EXE_FEATS = os.path.join(OUT, 'exe_feats.tsv')
CORPUS = os.path.join(OUT, 'libcorpus')
REFDIR = os.path.join(OUT, 'ref')

OWNERS = ('remedy', 'thirdparty', 'crt', 'unknown')
TEMPLATE_LIBS = {'std', 'stdext', 'rapidjson', 'sparsehash', 'nlohmann', 'fmt', 'fastdelegate', 'Concurrency',
                 'concurrency'}
# msvcp140 throw helpers: MSVC STL wraps them in tiny [[noreturn]] functions (vector::_Xlength etc.)
STL_THROW = re.compile(r'\?_X(length_error|out_of_range|bad_alloc|invalid_argument|runtime_error|overflow_error|'
                       r'bad_function_call|regex_error)@std@@|\?_Throw_Cpp_error@std@@|\?_Throw_C_error@std@@')
# string prefixes that pin a library even without a corpus entry
STRING_PREFIX_LIB = [('opentelemetry.proto.', 'opentelemetry'), ('sparsehash: ', 'sparsehash'),
                     ('[json.exception.', 'nlohmann'), ('OTel-OTLP-Exporter', 'opentelemetry')]
NAME_SRC_SCORE = {'export': 0.99, 'manual': 0.95, 'import_thunk': 0.99, 'crt': 0.95, 'funcsig': 0.95,
                  'string': 0.8, 'vftable': 0.9, 'thunk': 0.7, 'ghidra': 0.8}


def lib_owner(lib):
    if lib == 'crt':
        return 'crt'
    if lib == 'remedy':
        return 'remedy'
    return 'thirdparty'


STL_MEMBER = re.compile(r'^(push_back|emplace_back|emplace|insert|erase|find|resize|reserve|clear|assign|append|'
                        r'operator\[\]|at|swap|rehash|count|lower_bound|upper_bound|equal_range|try_emplace)$')


def ref_lib(lib, mangled):
    """Library of a reference function: std templates instantiated inside another library's
    reference TU (e.g. std::unordered_map in Luau) are std code."""
    if lib == 'std' or '@std@@' not in mangled:
        return lib
    m = re.match(r'^\?\?\$([A-Za-z_][A-Za-z0-9_]*)@|^\?([A-Za-z_][A-Za-z0-9_]*)@', mangled)
    ident = (m.group(1) or m.group(2)) if m else ''
    if ident.startswith('_') or STL_MEMBER.match(ident):
        return 'std'
    return lib


# ------------------------------------------------------------------------------------------------
# inputs
# ------------------------------------------------------------------------------------------------
def load_overrides(path=OVERRIDES):
    """va<TAB>owner<TAB>lib<TAB>note  ->  {va: (owner, lib, note)}"""
    out = {}
    if not os.path.exists(path):
        return out
    with open(path) as f:
        for ln, line in enumerate(f, 1):
            line = line.rstrip('\n')
            if not line.strip() or line.startswith('#') or line.startswith('va\t'):
                continue
            c = line.split('\t')
            c += [''] * (4 - len(c))
            owner = c[1].strip()
            if owner not in OWNERS:
                raise SystemExit('%s:%d: owner must be one of %s' % (path, ln, OWNERS))
            lib = c[2].strip() or ('remedy' if owner == 'remedy' else owner)
            out[int(c[0], 16)] = (owner, lib, c[3].strip())
    return out


def tsv_unesc(s):
    if '\\' not in s:
        return s
    return re.sub(r'\\(.)', lambda m: {'t': '\t', 'n': '\n', 'r': '\r', '\\': '\\'}.get(m.group(1), m.group(1)), s)


def load_feats(path, ib=None):
    """fnsig TSV -> {key: dict}; key = rva (exe) or row index (ref)."""
    out = {}
    if not os.path.exists(path):
        return out
    with open(path) as f:
        for i, line in enumerate(f):
            c = line.rstrip('\n').split('\t')
            if c[0] == 'id' or len(c) < 12:
                continue
            mh = c[11]
            d = {'name': tsv_unesc(c[1]), 'n_insn': int(c[2]), 'n_bytes': int(c[3]), 'h_exact': c[4],
                 'h_loose': c[5], 'cfg': tuple(int(x) for x in c[7].split(',')),
                 'consts': frozenset(c[8].split(';')) if c[8] else frozenset(),
                 'strings': frozenset(tsv_unesc(x) for x in c[10].split('\x1f')) if c[10] else frozenset(),
                 'mh': tuple(int(mh[k:k + 8], 16) for k in range(0, len(mh), 8)),
                 'vals': frozenset(c[12].split(';')) if len(c) > 12 and c[12] else frozenset(),
                 'mseq': c[13] if len(c) > 13 else ''}
            if ib is not None:
                out[int(c[0], 16) - ib] = d
            else:
                d['lib'] = c[0].split('|')[0]
                out[i] = d
    return out


def load_corpus():
    strings = collections.defaultdict(set)   # string -> {lib}
    consts = collections.defaultdict(set)    # const token -> {lib}
    p = os.path.join(CORPUS, 'strings.tsv')
    if os.path.exists(p):
        with open(p, encoding='utf-8', errors='replace') as f:
            for line in f:
                lib, _, s = line.rstrip('\n').partition('\t')
                strings[tsv_unesc(s)].add(lib)
    p = os.path.join(CORPUS, 'consts.tsv')
    if os.path.exists(p):
        with open(p) as f:
            for line in f:
                lib, _, k = line.rstrip('\n').partition('\t')
                consts[k].add(lib)
    refs = {}
    for p in sorted(glob.glob(os.path.join(REFDIR, '*.tsv'))):
        lib = os.path.basename(p)[:-4]
        refs[lib] = list(load_feats(p).values())
        for r in refs[lib]:
            for k in r['consts']:
                consts[k].add(lib)
    return strings, consts, refs


# ------------------------------------------------------------------------------------------------
# helpers
# ------------------------------------------------------------------------------------------------
IDENT_RE = re.compile(r'^[A-Za-z_][A-Za-z0-9_]*$')


def distinctive_string(s):
    """Long/structured enough that sharing it with a library source is meaningful."""
    if len(s) < 6 or sum(ch.isalpha() for ch in s) < 4:
        return False
    if IDENT_RE.match(s) and len(s) < 14:
        return False   # 'message', 'default', 'invalid' ... appear everywhere
    return True


def string_strength(s):
    return 0.8 if (len(s) >= 16 or ' ' in s.strip()) else 0.65


def generic_const(k):
    """Masks, powers of two, repeated bytes, 'round' floats: compiler idioms rather than library data."""
    kind, _, h = k.partition(':')
    try:
        v = int(h, 16)
    except ValueError:
        return True
    if kind == 'I':
        nbits = 64 if len(h) > 8 else 32
        for x in (v, (~v) & ((1 << nbits) - 1), (-v) & ((1 << nbits) - 1)):
            if x == 0 or (x & (x - 1)) == 0 or ((x + 1) & x) == 0:
                return True   # 2^n, 2^n-1, ~(2^n-1), negated
        b = v.to_bytes(8, 'little')
        nz = [x for x in b if x]
        if len(set(nz)) <= 1:
            return True       # 0x0101010101010101, 0x8080..., 0xff00ff00...
        if len(set(h.lstrip('0'))) <= 2:
            return True
        return v < 0x10000
    if kind == 'F':
        if len(h) >= 32:
            a, b2 = h[:16], h[16:32]
            if a == b2 or len(set(h)) <= 3:
                return True   # broadcast masks
            return False
        if len(h) == 16:
            return (v & 0xffffffff) == 0   # 'round' doubles (1.0, 0.5, 100.0, ...)
        if len(h) == 8:
            return (v & 0xffff) == 0
    return True


def noisy_or(scores):
    p = 1.0
    for s in scores:
        p *= (1.0 - min(s, 0.999))
    return 1.0 - p


class Classifier:
    def __init__(self, b, use_overrides=True, holdout=frozenset(), feats=None, corpus=None, log=None):
        self.b = b
        self.funcs = b.funcs
        self.order = sorted(b.funcs)
        self.idx = {f: i for i, f in enumerate(self.order)}
        self.use_overrides = use_overrides
        self.holdout = holdout
        self.log = log or (lambda m: None)
        self.ev = collections.defaultdict(list)    # rva -> [(owner, lib, score, src, detail)]
        self.feats = feats if feats is not None else load_feats(EXE_FEATS, b.pe.image_base)
        self.corpus = corpus if corpus is not None else load_corpus()
        self.stats = collections.Counter()
        callees = collections.defaultdict(set)
        callers = collections.defaultdict(set)
        for f, tg, site, kind in b.calls:
            if f != tg:
                callees[f].add(tg)
                callers[tg].add(f)
        self.callees, self.callers = callees, callers
        strs_of = collections.defaultdict(set)
        for site, tg in b.strrefs:
            fo = b.owner(site)
            if fo is not None:
                strs_of[fo].add(b.strings[tg][1])
        self.strs_of = strs_of
        self.labels = {}

    def add(self, f, owner, lib, score, src, detail=''):
        self.ev[f].append((owner, lib, round(score, 3), src, detail))
        self.stats[src] += 1

    # -------------------------------------------------------------------------------- resolution
    def resolve(self, f, evs=None):
        evs = self.ev.get(f, ()) if evs is None else evs
        if not evs:
            return None
        for e in evs:
            if e[3] == 'override':
                return (e[0], e[1], 1.0, 'override')
        by_owner = collections.defaultdict(list)
        for owner, lib, s, src, det in evs:
            by_owner[owner].append((s, lib, src))
        scored = []
        for owner, lst in by_owner.items():
            lst.sort(reverse=True)
            libs = collections.defaultdict(list)
            for s, lib, src in lst:
                libs[lib].append(s)
            best_lib = max(libs, key=lambda l: (noisy_or(libs[l]), l))
            scored.append((min(noisy_or([s for s, _, _ in lst]), 0.98), owner, best_lib, lst[0][2]))
        scored.sort(reverse=True)
        s1, owner, lib, src = scored[0]
        if len(scored) > 1 and scored[1][0] >= 0.5:
            s1 = s1 - scored[1][0]
            if s1 < 0.15:
                return ('unknown', 'unknown', round(scored[0][0] - scored[1][0], 3), 'conflict')
        return (owner, lib, round(s1, 3), src)

    def resolve_all(self, min_score=0.0):
        lab = {}
        for f in self.ev:
            r = self.resolve(f)
            if r and r[0] != 'unknown' and r[2] >= min_score:
                lab[f] = r
        return lab

    # -------------------------------------------------------------------------------- sources
    def ev_overrides(self):
        if not self.use_overrides:
            return
        ib = self.b.pe.image_base
        for va, (owner, lib, note) in load_overrides().items():
            f = va - ib
            if f in self.funcs:
                self.add(f, owner, lib, 1.0, 'override', note)
            else:
                self.log('owner_overrides.tsv: 0x%x is not a function start (ignored)' % va)

    def ev_names(self):
        b = self.b
        for f, fn in self.funcs.items():
            if f in self.holdout or not fn.get('name') or fn.get('name_owner') is None:
                continue
            name, src = fn['name'], fn['name_src']
            owner = fn['name_owner']
            lib = 'crt' if owner == 'crt' else ('remedy' if owner == 'remedy' else fn['top_ns'])
            score = NAME_SRC_SCORE.get(src, 0.7)
            det = src
            if src == 'ghidra':
                if name.startswith(('`public:', '`protected:', '`private:', 'FID_conflict')) or '`' in name[:2]:
                    continue   # Function-ID false positives (MFC etc. are not in this exe)
                if fn['size'] < 32:
                    score = 0.55
            if src == 'vftable':
                cls = set(ci for ci, _ in b.vft_by_target.get(f, []))
                if len(cls) > 1:
                    owners = collections.Counter(fn_owner_of_class(b, ci) for ci in cls)
                    if len(owners) > 1:
                        score = 0.6
                        det = 'vftable shared by %d classes of different owners' % len(cls)
                # std::function's _Do_call (slot 2) is the user's lambda body
                if lib == 'std' and re.match(r'^std::_Func_impl(_no_alloc)?<.*>::vfunc_2$', name):
                    self.add(f, 'remedy', 'remedy', 0.6, 'name', 'std::function _Do_call = lambda body')
                    continue
            self.add(f, owner, lib, score, 'name', det)

    def ev_srcfile(self):
        for f, s in self.b.srcfile.items():
            if f in self.holdout:
                continue
            sl = s.replace('\\', '/').lower()
            if '/middleware/wwiseplugins/' in sl:
                self.add(f, 'remedy', 'remedy', 0.6, 'srcfile', 'Remedy-side Wwise plugin: ' + s)
            elif 'wwise' in sl:
                self.add(f, 'thirdparty', 'AK', 0.95, 'srcfile', s)
            elif 'opentelemetry' in sl:
                self.add(f, 'thirdparty', 'opentelemetry', 0.95, 'srcfile', s)
            elif '/middleware/' in sl:
                lib = sl.split('/middleware/')[1].split('/')[0]
                self.add(f, 'thirdparty', lib, 0.95, 'srcfile', s)

    def ev_crt_region(self):
        b = self.b
        ep = b.pe.entry_rva
        crt_named = [f for f in self.order if self.funcs[f].get('name_owner') == 'crt'
                     and self.funcs[f]['source'] != 'import_thunk' and abs(f - ep) < 0x20000]
        if not crt_named:
            return
        lo, hi = min(crt_named + [ep]), max(crt_named + [ep])
        for f in self.order[bisect.bisect_left(self.order, lo):bisect.bisect_right(self.order, hi)]:
            if self.funcs[f].get('name_src') not in ('export', 'vftable', 'funcsig'):
                self.add(f, 'crt', 'crt', 0.9, 'crt_region', '0x%x-0x%x' % (lo, hi))
        for f, fn in self.funcs.items():
            if fn['source'] == 'import_thunk':
                self.add(f, 'crt', 'crt', 0.99, 'name', 'import thunk')

    def strong_name_owner(self):
        """rva -> (owner, lib) from non-held-out name/srcfile evidence with score >= 0.85."""
        out = {}
        for f, evs in self.ev.items():
            for owner, lib, s, src, det in evs:
                if src in ('name', 'srcfile', 'override') and s >= 0.85:
                    out[f] = (owner, lib)
        return out

    def ev_strings(self):
        strings, _, _ = self.corpus
        strong = self.strong_name_owner()
        # strings co-referenced by confidently owned functions
        users = collections.defaultdict(set)
        for f, ss in self.strs_of.items():
            if f in strong:
                for s in ss:
                    users[s].add(strong[f])
        remedy_str = set(s for s, u in users.items() if u and all(o == 'remedy' for o, _ in u))
        for f, ss in self.strs_of.items():
            n = self.feats.get(f, {}).get('n_insn', 0)
            libhits = collections.defaultdict(list)
            for s in ss:
                libs = set(strings.get(s, ()))
                for pre, lib in STRING_PREFIX_LIB:
                    if s.startswith(pre):
                        libs = {lib}
                if not libs or (not distinctive_string(s) and not any(s.startswith(p) for p, _ in STRING_PREFIX_LIB)):
                    continue
                if s in remedy_str:
                    continue   # also used by Remedy-named code: shared/generic
                lib = 'std' if 'std' in libs else sorted(libs)[0]
                libhits[lib].append(s)
            foreign = [s for s in ss if s in remedy_str]
            for lib, hs in libhits.items():
                score = max(string_strength(s) for s in hs) + 0.05 * (len(hs) - 1)
                if foreign:
                    if lib in TEMPLATE_LIBS:
                        continue   # library code inlined into a Remedy function
                    score -= 0.25
                if lib in TEMPLATE_LIBS and n > 150:
                    score *= 0.75
                if score >= 0.4:
                    self.add(f, 'thirdparty', lib, min(score, 0.95), 'string', '; '.join(sorted(hs))[:200])
            # co-referenced strings (no corpus needed: Wwise/Kaim/... and Remedy)
            if f in strong:
                continue
            votes = collections.Counter()
            for s in ss:
                u = users.get(s)
                if u and len(u) == 1 and distinctive_string(s):
                    votes[next(iter(u))] += 1
            if len(votes) == 1:
                (owner, lib), c = votes.most_common(1)[0]
                self.add(f, owner, lib, 0.55 if c == 1 else (0.62 if c == 2 else 0.7), 'corefstr',
                         '%d strings shared with %s functions' % (c, lib))

    def ev_consts(self):
        _, consts, _ = self.corpus
        strong = self.strong_name_owner()
        df = collections.Counter()
        remedy_consts = set()
        for f, d in self.feats.items():
            for k in d['consts']:
                df[k] += 1
                if strong.get(f, ('',))[0] == 'remedy':
                    remedy_consts.add(k)
        for f, d in self.feats.items():
            hits = collections.defaultdict(set)
            for k in d['consts']:
                libs = consts.get(k)
                if not libs or len(libs) > 2 or df[k] > 12 or k in remedy_consts or generic_const(k):
                    continue
                lib = 'std' if 'std' in libs else sorted(libs)[0]
                hits[lib].add(k)
            for lib, ks in hits.items():
                score = {1: 0.5, 2: 0.7}.get(len(ks), 0.8)
                if lib in TEMPLATE_LIBS and d['n_insn'] > 150:
                    score *= 0.8
                self.add(f, 'thirdparty', lib, score, 'const', ' '.join(sorted(ks))[:160])

    def ev_refsig(self):
        """Exact normalized-code matches (rare across compilers) and distinctive feature-set matches."""
        _, consts, refs = self.corpus
        if not refs:
            return
        strings = self.corpus[0]
        df = collections.Counter()
        for d in self.feats.values():
            for k in d['consts']:
                df[k] += 1
        exact = collections.defaultdict(set)
        keyed = collections.defaultdict(set)
        for lib, rs in refs.items():
            for r in rs:
                if r['n_insn'] >= 12:
                    exact[r['h_exact']].add((lib, r['n_insn']))
                key = frozenset(k for k in r['consts'] if not generic_const(k) and 0 < df.get(k, 0) <= 12)
                key |= frozenset('S:' + s for s in r['strings'] if distinctive_string(s))
                if key and r['n_insn'] >= 8:
                    keyed[key].add((lib, r['n_insn'], r['cfg'][2]))
        for f, d in self.feats.items():
            hit = exact.get(d['h_exact'])
            if hit and len(set(l for l, _ in hit)) == 1:
                self.add(f, 'thirdparty', next(iter(hit))[0], 0.9, 'refsig', 'identical normalized code')
                continue
            key = frozenset(k for k in d['consts'] if not generic_const(k) and 0 < df.get(k, 0) <= 12)
            key |= frozenset('S:' + s for s in self.strs_of.get(f, ()) if distinctive_string(s))
            if not key:
                continue
            m = keyed.get(key)
            if not m:
                continue
            libs = set(l for l, n, nb in m if 0.5 <= d['n_insn'] / max(n, 1) <= 2.0 and nb == d['cfg'][2])
            if len(libs) == 1:
                self.add(f, 'thirdparty', libs.pop(), 0.75, 'refsig', 'same distinctive features as reference: '
                         + ' '.join(sorted(key))[:120])

    def vals_idf(self):
        if getattr(self, '_idf', None) is None:
            df = collections.Counter()
            for d in self.feats.values():
                df.update(d['vals'])
            n = max(len(self.feats), 1)
            self._idf = {t: math.log(n / c) for t, c in df.items()}
            self._idf_max = math.log(n)
            self._vals_df = df
        return self._idf

    def ev_refvals(self):
        """Cross-compiler 'value bag' match: the set of non-stack field offsets (m<size>@<disp>) and
        compare/test/mask immediates of a clang-cl reference function vs. exe functions (idf-weighted
        Jaccard). Register allocation, instruction selection and block order differ between clang-cl
        and MSVC, but the struct offsets and flag constants a function touches mostly do not."""
        _, _, refs = self.corpus
        if not refs or not any(d['vals'] for d in self.feats.values()):
            return
        idf = self.vals_idf()
        imax, df = self._idf_max, self._vals_df
        inv = collections.defaultdict(list)
        for f, d in self.feats.items():
            for t in d['vals']:
                if df[t] <= 3000:
                    inv[t].append(f)
        best = {}   # f -> {lib: (wj, refname)}
        seen = set()
        for lib0, rs in refs.items():
            for r in rs:
                if r['n_insn'] < 8 or len(r['vals']) < 3:
                    continue
                lib = ref_lib(lib0, r['name'])
                key = (lib, r['vals'], r['cfg'][3] == 0)
                if key in seen:
                    continue
                seen.add(key)
                w_r = sum(idf.get(t, imax) for t in r['vals'])
                if w_r < 18:
                    continue   # too generic to mean anything
                cand = collections.Counter()
                for t in r['vals']:
                    for f in inv.get(t, ()):
                        cand[f] += 1
                for f, k in cand.items():
                    if k < 2:
                        continue
                    d = self.feats[f]
                    if not (0.5 <= d['n_insn'] / r['n_insn'] <= 2.0) or (d['cfg'][3] == 0) != (r['cfg'][3] == 0):
                        continue
                    inter = sum(idf.get(t, 0) for t in r['vals'] & d['vals'])
                    uni = sum(idf.get(t, imax) for t in r['vals'] | d['vals'])
                    wj = inter / uni if uni else 0
                    if wj >= 0.7:
                        cur = best.setdefault(f, {})
                        if wj > cur.get(lib, (0,))[0]:
                            cur[lib] = (wj, r['name'])
        for f, m in best.items():
            libs = set(m)
            if len(libs) > 1 and 'std' in libs:
                libs = {'std'}       # std templates are instantiated in every reference TU
            if len(libs) != 1:
                continue
            lib = libs.pop()
            wj, rn = m[lib]
            score = 0.75 if wj >= 0.9 else (0.65 if wj >= 0.8 else 0.55)
            self.add(f, 'thirdparty', lib, score, 'refvals', 'value-bag %.2f ~ %s' % (wj, rn[:90]))

    def prop_smallsim(self):
        """Small functions (where minhash is too noisy): same value bag + same CFG shape + similar
        mnemonic-class sequence as a confidently owned function."""
        lab = self.resolve_all(0.8)
        idf = self.vals_idf()
        imax = self._idf_max
        idx = collections.defaultdict(list)
        for f, d in self.feats.items():
            if f in lab and d['mseq'] and 6 <= d['n_insn'] <= 160 and d['vals']:
                idx[(d['vals'], d['cfg'])].append(f)
        n = 0
        for f, d in self.feats.items():
            if f in lab or not d['mseq'] or not (6 <= d['n_insn'] <= 160) or not d['vals']:
                continue
            group = idx.get((d['vals'], d['cfg']))
            if not group or len(group) > 200:
                continue
            if sum(idf.get(t, imax) for t in d["vals"]) < 8:
                continue
            votes = {}
            sm = difflib.SequenceMatcher(None, '', d['mseq'], autojunk=False)
            for c in group:
                if not (0.75 <= d['n_insn'] / self.feats[c]['n_insn'] <= 1.33):
                    continue
                sm.set_seq1(self.feats[c]['mseq'])
                if sm.real_quick_ratio() < 0.75 or sm.quick_ratio() < 0.75:
                    continue
                r = sm.ratio()
                if r >= 0.75:
                    key = lab[c][:2]
                    if r > votes.get(key, (0,))[0]:
                        votes[key] = (r, c)
            if len(set(k[0] for k in votes)) == 1:
                key = max(votes, key=lambda k: votes[k][0])
                r, c = votes[key]
                self.add(f, key[0], key[1], min(0.7, 0.8 * r), 'smallsim',
                         'same value bag/CFG, mnemonic similarity %.2f to 0x%x (%s)' % (r, self.b.pe.va(c), key[1]))
                n += 1
        return n

    def ev_stl_helper(self):
        b = self.b
        thr = collections.defaultdict(set)
        for f, site, dll, fn, kind in b.import_calls:
            if dll.lower().startswith('msvcp') and STL_THROW.search(fn):
                thr[f].add(fn)
        for f, fns in thr.items():
            n = self.feats.get(f, {}).get('n_insn', 99)
            if n <= 16 and not self.callees.get(f):
                self.add(f, 'thirdparty', 'std', 0.85, 'stl_helper', ','.join(sorted(fns))[:120])

    def ev_vftref(self):
        """lea of a class vftable = (inlined) constructor/destructor of that class."""
        b = self.b
        vft_cls = {v[0]: v[1] for v in b.vftables}
        seen = collections.defaultdict(set)
        for site, tg in b.lea:
            ci = vft_cls.get(tg)
            if ci is None:
                continue
            fo = b.owner(site)
            if fo is not None:
                seen[fo].add(ci)
        for f, cis in seen.items():
            owners = collections.Counter(fn_owner_of_class(b, ci) for ci in cis)
            if len(owners) == 1:
                owner = next(iter(owners))
                if owner == 'remedy':
                    self.add(f, 'remedy', 'remedy', 0.7, 'vftref', b.classes[next(iter(cis))]['name'][:100])
                elif owner == 'thirdparty':
                    lib = class_lib(b, next(iter(cis)))
                    if lib not in TEMPLATE_LIBS:   # std vftables are stored by inlined make_shared etc.
                        self.add(f, 'thirdparty', lib, 0.7, 'vftref', b.classes[next(iter(cis))]['name'][:100])

    # -------------------------------------------------------------------------------- propagation
    def prop_dups(self):
        lab = self.resolve_all(0.8)
        groups = collections.defaultdict(list)
        for f, d in self.feats.items():
            if d['n_insn'] >= 12:
                groups[d['h_loose']].append(f)
        n = 0
        for g in groups.values():
            if len(g) < 2:
                continue
            votes = collections.Counter(lab[f][:2] for f in g if f in lab)
            if len(votes) != 1:
                continue
            (owner, lib), c = votes.most_common(1)[0]
            for f in g:
                if f not in lab:
                    self.add(f, owner, lib, 0.75 if c >= 2 else 0.65, 'dupgroup',
                             'identical normalized code as %d %s function(s)' % (c, lib))
                    n += 1
        # near duplicates via minhash LSH (16 bands x 2 rows), large enough functions only
        idx = collections.defaultdict(list)
        for f, d in self.feats.items():
            if d['n_insn'] >= 20 and len(d['mh']) == 32:
                m = d['mh']
                for k in range(16):
                    idx[(k, m[2 * k], m[2 * k + 1])].append(f)
        for f, d in self.feats.items():
            if f in lab or d['n_insn'] < 20 or len(d['mh']) != 32:
                continue
            r = self.resolve(f)
            if r and r[2] >= 0.6:
                continue
            m = d['mh']
            cand = set()
            for k in range(16):
                bucket = idx.get((k, m[2 * k], m[2 * k + 1]), ())
                if len(bucket) <= 400:
                    cand.update(x for x in bucket if x in lab)
            votes = collections.defaultdict(float)
            for c in cand:
                dc = self.feats[c]
                if not (0.75 <= d['n_insn'] / dc['n_insn'] <= 1.33):
                    continue
                sim = sum(1 for a, b2 in zip(m, dc['mh']) if a == b2) / 32.0
                if sim >= 0.75:
                    key = lab[c][:2]
                    votes[key] = max(votes[key], sim)
            owners = set(k[0] for k in votes)
            if len(owners) == 1:
                key = max(votes, key=votes.get)
                self.add(f, key[0], key[1], min(0.7, 0.85 * votes[key]), 'neardup',
                         'minhash %.2f to confident %s code' % (votes[key], key[1]))
                n += 1
        return n

    def prop_callgraph(self, rounds=4):
        total = 0
        for _ in range(rounds):
            lab = self.resolve_all(0.65)
            added = 0
            for f in self.order:
                if f in lab:
                    continue
                cl = self.callers.get(f)
                if not cl or any(c not in lab for c in cl):
                    continue
                libs = set(lab[c][1] for c in cl)
                owners = set(lab[c][0] for c in cl)
                if owners != {'thirdparty'} or len(libs) != 1:
                    continue
                lib = next(iter(libs))
                ce = self.callees.get(f, set())
                ce_lab = [lab.get(c) for c in ce]
                if lib in TEMPLATE_LIBS:
                    if any(x is None or x[1] not in (lib, 'crt') for x in ce_lab):
                        continue   # template code calls user code (comparators, dtors): only leaves
                    score = 0.6
                else:
                    if any(x is not None and x[1] not in (lib, 'crt') for x in ce_lab):
                        continue
                    score = 0.75 if len(cl) >= 2 else 0.65
                    if any(x is None for x in ce_lab):
                        score -= 0.1
                if any(e[3] == 'callgraph' for e in self.ev.get(f, ())):
                    continue
                self.add(f, 'thirdparty', lib, score, 'callgraph', 'all %d caller(s) are %s' % (len(cl), lib))
                added += 1
            total += added
            if not added:
                break
        return total

    def prop_neighbor(self):
        lab = self.resolve_all(0.6)
        anchors = [f for f in self.order if f in lab and self.funcs[f]['source'] != 'import_thunk']
        n = 0
        for a, c in zip(anchors, anchors[1:]):
            ia, ic = self.idx[a], self.idx[c]
            if ic - ia <= 1:
                continue
            la, lc = lab[a], lab[c]
            if la[0] != lc[0]:
                continue
            same_lib = la[1] == lc[1]
            lib = la[1] if same_lib else ('remedy' if la[0] == 'remedy' else 'unknown')
            for j in range(ia + 1, ic):
                f = self.order[j]
                if f in lab:
                    continue
                d = max(j - ia, ic - j)
                if la[0] == 'remedy':
                    base = 0.55
                elif lib in TEMPLATE_LIBS:
                    base = 0.45
                elif la[0] == 'crt':
                    base = 0.6
                else:
                    base = 0.7
                if not same_lib:
                    base -= 0.15
                if d <= 8:
                    s, src = base, 'neighbor'
                elif d <= 32:
                    s, src = base - 0.1, 'neighbor'
                else:
                    s, src = min(base - 0.2, 0.4), 'weak_neighbor'
                self.add(f, la[0], lib, s, src, '0x%x / 0x%x (%d functions away)' %
                         (self.b.pe.va(a), self.b.pe.va(c), d))
                n += 1
        return n

    # -------------------------------------------------------------------------------- driver
    def run(self):
        self.ev_overrides()
        self.ev_names()
        self.ev_srcfile()
        self.ev_crt_region()
        self.ev_stl_helper()
        self.ev_vftref()
        self.ev_strings()
        self.ev_consts()
        self.ev_refsig()
        self.ev_refvals()
        nd = self.prop_dups()
        nd += self.prop_smallsim()
        ncg = self.prop_callgraph()
        nn = self.prop_neighbor()
        self.log('ownership evidence: %s (dup/neardup %d, callgraph %d, neighbor %d)'
                 % (', '.join('%s=%d' % kv for kv in sorted(self.stats.items())), nd, ncg, nn))
        out = {}
        for f in self.order:
            r = self.resolve(f)
            if r is None:
                r = ('unknown', 'unknown', 0.0, 'none')
            out[f] = r
        return out


HOOKS = {}   # owner_of / top_ns_of, installed by fndb.py (avoids a circular import)


def fn_owner_of_class(b, ci):
    return HOOKS['owner_of'](HOOKS['top_ns_of'](b.classes[ci]['name'] + '::x'))


def class_lib(b, ci):
    return HOOKS['top_ns_of'](b.classes[ci]['name'] + '::x')


def apply(b, result, ev):
    """Write classification into Builder.funcs (owner, owner_src, owner_conf, top_ns for libraries)."""
    for f, (owner, lib, conf, src) in result.items():
        fn = b.funcs[f]
        fn['owner'], fn['owner_src'], fn['owner_conf'] = owner, src, conf
        fn['owner_lib'] = lib
        if owner in ('thirdparty', 'crt') and fn.get('top_ns') in (None, 'unknown') and lib not in ('unknown',):
            fn['top_ns'], fn['ns_src'] = lib, 'owner:' + src
        elif owner == 'unknown' and fn.get('top_ns') is None:
            fn['top_ns'] = 'unknown'
        if fn.get('top_ns') is None:
            fn['top_ns'] = 'unknown'
    b.owner_evidence = [(f, o, l, s, src, det) for f, evs in ev.items() for (o, l, s, src, det) in evs]


def holdout_eval(b, frac=0.1, seed=1, log=None, block=0x10000):
    """Hide name evidence for every named function inside random address windows (block hold-out:
    sibling vfuncs/neighbors are hidden too, so this approximates an unnamed region) and report how
    the remaining evidence classifies them vs. the owner implied by the hidden name.
    Returns (held-out set, rows[(src, conf_bin)] -> Counter, confusion[truth] -> Counter, result)."""
    named = [f for f, fn in b.funcs.items() if fn.get('name_owner') in ('remedy', 'thirdparty')
             and fn.get('name_src') in ('vftable', 'funcsig', 'export', 'string')]
    rnd = random.Random(seed)
    if block:
        # stratify: windows that contain third-party names are sampled separately so the
        # (rare) library code is represented
        tp_w = sorted(set(f - f % block for f in named if b.funcs[f]['name_owner'] == 'thirdparty'))
        rem_w = sorted(set(f - f % block for f in named) - set(tp_w))
        pick = set(rnd.sample(tp_w, max(1, int(len(tp_w) * frac)))) | \
            set(rnd.sample(rem_w, max(1, int(len(rem_w) * frac))))
        hold = frozenset(f for f in named if f - f % block in pick)
    else:
        hold = frozenset(rnd.sample(named, int(len(named) * frac)))
    c = Classifier(b, use_overrides=False, holdout=hold, log=log)
    res = c.run()
    rows = collections.defaultdict(lambda: collections.Counter())
    conf = collections.defaultdict(lambda: collections.Counter())
    for f in hold:
        truth = b.funcs[f]['name_owner']
        owner, lib, cf, src = res[f]
        tb = 'hi' if cf >= 0.8 else ('mid' if cf >= 0.5 else 'lo')
        conf[truth][(owner, tb)] += 1
        key = (src, tb)
        if owner == 'unknown':
            rows[key]['unknown'] += 1
        elif owner == truth:
            rows[key]['agree'] += 1
        else:
            rows[key]['disagree'] += 1
            tl = b.funcs[f].get('top_ns') if truth == 'thirdparty' else None
            rows[key]['%s%s->%s' % (truth[:3], '(%s)' % tl if tl else '', owner[:3])] += 1
    return hold, rows, conf, res
