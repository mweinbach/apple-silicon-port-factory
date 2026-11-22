#!/usr/bin/env python3
"""fndb - function database for the AlanWake2.exe decompilation (stdlib only).

    tools/fndb.sh build [--exe PATH]          parse the exe -> db/functions.sqlite (gitignored)
    tools/fndb.sh report [--top N]            per-namespace / owner progress (count- and byte-weighted)
    tools/fndb.sh show <va|name-substring>    everything known about a function
    tools/fndb.sh next [--ns NS] [--max-size N] [--all-owners] [--limit N]
    tools/fndb.sh set <va> <todo|draft|verified|skip> [--rust PATH] [--notes TEXT]
    tools/fndb.sh sync                         scan crates/aw2-game/src -> db/progress.tsv
    tools/fndb.sh disasm <va|name>             objdump disassembly annotated with fndb names
    tools/fndb.sh funcsig <substring>          search __FUNCSIG__ strings (ECS systems, types)
    tools/fndb.sh strings <substring>          search referenced strings + referencing functions
    tools/fndb.sh sql "<SQL>"                  ad-hoc query
    tools/fndb.sh ghidra-names [--out PATH]    names TSV for re/ghidra/AW2ImportNames.java
    tools/fndb.sh ingest-ghidra <jsonl>        merge AW2ExportFunctions.java output (names, call graph)

See tools/README.md. All heuristics are approximate; the `source`, `name_src`, `owner_src`
and `ns_src` columns record where each fact came from.
"""
import argparse
import bisect
import collections
import json
import os
import re
import sqlite3
import struct
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from msvc_demangle import type_descriptor_name, symbol_qualified_name, split_qualified  # noqa: E402
import owner as ownermod  # noqa: E402

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
DEFAULT_EXE = os.environ.get('AW2_EXE') or (
    os.path.join(os.environ['AW2_GAME_DIR'], 'AlanWake2.exe')
    if os.environ.get('AW2_GAME_DIR')
    else ''
)
DB_PATH = os.environ.get('AW2_FNDB', os.path.join(ROOT, 'db', 'functions.sqlite'))
PROGRESS = os.path.join(ROOT, 'db', 'progress.tsv')
THROWS_TSV = os.path.join(ROOT, 'db', 'throws.tsv')   # registry can-throw facts for the loader lint
SYMMAP = os.path.join(ROOT, 're', 'symbol_map.json')
GAME_SRC = os.path.join(ROOT, 'crates', 'aw2-game', 'src')
STATUSES = ('todo', 'draft', 'verified', 'skip')

u16 = lambda d, o: struct.unpack_from('<H', d, o)[0]
u32 = lambda d, o: struct.unpack_from('<I', d, o)[0]
u64 = lambda d, o: struct.unpack_from('<Q', d, o)[0]
i32 = lambda d, o: struct.unpack_from('<i', d, o)[0]

T0 = time.time()


def log(msg):
    sys.stderr.write('[%6.1fs] %s\n' % (time.time() - T0, msg))


# --------------------------------------------------------------------------------------------
# Ownership classification
# --------------------------------------------------------------------------------------------
REMEDY_NS = {
    'r', 'rend', 'ecs', 'content', 'contentcore', 'coregame', 'coreshared', 'bigfish', 'bonsai',
    'contentbonsai', 'puppet', 'snd', 'ui', 'platform', 'app', 'input', 'navigation', 'physics',
    'SaveGameContent', 'm', 'g', 'd', 'net', 'xml', 'loc', 'rex', 'operations_queue',
    'anonymous_namespace', 'global', 'unknown',
}
THIRDPARTY_NS = {
    'Luau', 'lua', 'AK', 'WwiseAuthoringAPI', 'opentelemetry', 'fmt', 'std', 'stdext',
    'Concurrency', 'concurrency', 'physx', 'Kaim', 'cohtml', 'renoir', 'd3d', 'nlohmann', 'google',
    'absl', 'Microsoft', 'icu_69', 'v8', 'tbb', 'rapidjson', 'pugi', 'tinyxml2', 'EOS', 'sl',
    'ffx', 'nrd', 'nri', 'NRD', 'NRI', 'curl', 'lz4', 'zstd', 'xxhash', 'bink', 'nvidia', 'nv',
    'protobuf', 'grpc', 'Json', 'json', 'utf8', 'tracy', 'zlib', 'cityhash', 'fastdelegate', 'sparsehash',
}
# Global-scope name prefixes that belong to third-party libraries -> pseudo namespace.
GLOBAL_PREFIX_NS = [
    ('CAk', 'AK'), ('IAk', 'AK'), ('Ak', 'AK'), ('AK_', 'AK'), ('lua_', 'Luau'), ('luaL_', 'Luau'),
    ('luau_', 'Luau'), ('luaG_', 'Luau'), ('luaD_', 'Luau'), ('luaH_', 'Luau'), ('luaV_', 'Luau'),
    ('luaC_', 'Luau'), ('luaM_', 'Luau'), ('luaS_', 'Luau'), ('luaF_', 'Luau'), ('luaE_', 'Luau'),
    ('luaO_', 'Luau'), ('luaT_', 'Luau'), ('luaU_', 'Luau'), ('luaA_', 'Luau'), ('Px', 'physx'),
    ('LZ4', 'lz4'), ('ZSTD_', 'zstd'), ('XXH', 'xxhash'), ('curl_', 'curl'), ('EOS_', 'EOS'),
    ('Bink', 'bink'), ('lua_exception', 'Luau'), ('CityHash', 'cityhash'),
]
CRT_EXACT = {
    'WinMainCRTStartup', 'wWinMainCRTStartup', 'mainCRTStartup', '__security_init_cookie',
    '__security_check_cookie', '__scrt_common_main_seh', '__GSHandlerCheck', '__GSHandlerCheckCommon',
    '__GSHandlerCheck_EH4', '__report_gsfailure', '__raise_securityfailure', '_purecall',
    '__chkstk', '_alloca_probe', '__C_specific_handler', '__CxxFrameHandler3', '__CxxFrameHandler4',
    'memcpy', 'memset', 'memmove', 'memcmp', 'atexit', '_onexit', '__isa_available_init',
    'type_info::operator==', 'type_info::name', '__std_type_info_compare',
}
CRT_PREFIXES = ('__scrt_', '__security_', '_CRT_', '__acrt_', '__vcrt_', '__std_', '__GSHandler',
                '_guard_', '__C_specific', '__CxxFrameHandler', '_RTC_', '__isa_', '__report_',
                '__dyn_tls', '_tls_', '__local_stdio', '_initterm', '__crt_', 'crt_handler_',
                '__CxxThrow', '_CxxThrow', '__FrameUnwind', '__DestructExceptionObject',
                '__NLG_', '_guard_dispatch', '__acrt', 'j___CxxFrameHandler', '_Init_thread',
                '__ArrayUnwind', '`eh_vector', 'tls_callback', '__tlregdtor', '_onexit', '__scrt',
                'vsnprintf', 'snprintf', 'sprintf_s', 'sscanf', '_vsnprintf', '_vsprintf', '_vfprintf',
                'FID_conflict:sprintf', 'FID_conflict:_vsnprintf', '__local_stdio')
ZLIB_NAMES = {'crc32', 'crc32_little', 'crc32_big', 'adler32', 'deflate', 'inflate', 'read_buf',
              'bi_flush', 'bi_windup', '_tr_init', '_tr_flush_block', '_tr_stored_block', '_tr_align',
              'deflateInit_', 'deflateInit2_', 'inflateInit_', 'inflateInit2_', 'deflateEnd',
              'inflateEnd', 'compress_block', 'send_tree', 'build_tree', 'gen_bitlen', 'fill_window',
              'longest_match', 'deflate_stored', 'deflate_fast', 'deflate_slow', 'inflate_fast',
              'inflate_table', 'zcalloc', 'zcfree', 'updatewindow', 'uncompress', 'compress2'}


def top_ns_of(name):
    """Top namespace of a qualified function name ('global' if unqualified)."""
    if not name:
        return 'unknown'
    parts = split_qualified(name)
    if len(parts) == 1:
        if name in ZLIB_NAMES:
            return 'zlib'
        for pre, ns in GLOBAL_PREFIX_NS:
            if name.startswith(pre):
                return ns
        return 'global'
    t = parts[0]
    t = t.split('<')[0]
    if t.startswith(('`anonymous', ANON_ALIAS)):
        return 'anonymous_namespace'
    if t in ('', ):
        return 'global'
    if len(parts) >= 2 and t not in REMEDY_NS and t not in THIRDPARTY_NS:
        for pre, ns in GLOBAL_PREFIX_NS:
            if t.startswith(pre):
                return ns
    return t


ANON_ALIAS = 'anonymous_namespace'


def owner_of(top_ns, name=None):
    if name:
        base = name.split('::')[-1] if '<' not in name else name
        if name in CRT_EXACT or name.startswith(CRT_PREFIXES) or base in CRT_EXACT:
            return 'crt'
    if top_ns in THIRDPARTY_NS:
        return 'thirdparty'
    if top_ns == 'crt':
        return 'crt'
    return 'remedy'


ownermod.HOOKS.update(owner_of=owner_of, top_ns_of=top_ns_of)


# --------------------------------------------------------------------------------------------
# PE parsing
# --------------------------------------------------------------------------------------------
class PE:
    def __init__(self, path):
        self.path = path
        with open(path, 'rb') as f:
            self.data = d = f.read()
        pe = u32(d, 0x3c)
        if d[pe:pe + 4] != b'PE\0\0':
            raise SystemExit('not a PE file: %s' % path)
        nsec = u16(d, pe + 6)
        optsz = u16(d, pe + 20)
        opt = pe + 24
        if u16(d, opt) != 0x20b:
            raise SystemExit('not PE32+')
        self.entry_rva = u32(d, opt + 16)
        self.image_base = u64(d, opt + 24)
        self.size_of_image = u32(d, opt + 56)
        ndd = u32(d, opt + 108)
        self.dd = [struct.unpack_from('<II', d, opt + 112 + 8 * i) for i in range(min(ndd, 16))]
        self.sections = []
        for i in range(nsec):
            o = opt + optsz + 40 * i
            name = d[o:o + 8].rstrip(b'\0').decode('latin1')
            vs, va, rs, ro = struct.unpack_from('<IIII', d, o + 8)
            ch = u32(d, o + 36)
            self.sections.append((name, va, vs, ro, rs, ch))
        self._starts = [s[1] for s in self.sections]
        t = self.sec('.text')
        self.text_rva, self.text_end = t[1], t[1] + t[2]
        self.text_off = t[3]
        self.text = d[t[3]:t[3] + min(t[2], t[4])]

    def sec(self, name):
        for s in self.sections:
            if s[0] == name:
                return s
        return None

    def sec_of(self, rva):
        i = bisect.bisect_right(self._starts, rva) - 1
        if i >= 0:
            s = self.sections[i]
            if rva < s[1] + max(s[2], s[4]):
                return s
        return None

    def off(self, rva):
        s = self.sec_of(rva)
        if s is None or rva - s[1] >= s[4]:
            return None
        return s[3] + rva - s[1]

    def in_text(self, rva):
        return self.text_rva <= rva < self.text_end

    def va(self, rva):
        return self.image_base + rva

    def rva(self, va):
        return va - self.image_base

    def cstr(self, rva, maxlen=512):
        o = self.off(rva)
        if o is None:
            return None
        e = self.data.find(b'\0', o, o + maxlen)
        if e < 0:
            return None
        return self.data[o:e].decode('latin1')

    # ---- directories
    def exports(self):
        rva, size = self.dd[0]
        if not rva:
            return []
        d = self.data
        o = self.off(rva)
        base = u32(d, o + 16)
        nfun, nnames = u32(d, o + 20), u32(d, o + 24)
        afun, anames, aord = u32(d, o + 28), u32(d, o + 32), u32(d, o + 36)
        funcs = [u32(d, self.off(afun) + 4 * i) for i in range(nfun)]
        out, named = [], set()
        for i in range(nnames):
            nrva = u32(d, self.off(anames) + 4 * i)
            ordi = u16(d, self.off(aord) + 2 * i)
            frva = funcs[ordi]
            if rva <= frva < rva + size:
                continue  # forwarder
            out.append((ordi + base, frva, self.cstr(nrva)))
            named.add(ordi)
        return out

    def imports(self):
        rva, size = self.dd[1]
        d = self.data
        out = []
        o = self.off(rva)
        while True:
            oft, _, _, name_rva, ft = struct.unpack_from('<IIIII', d, o)
            if not (oft or name_rva or ft):
                break
            dll = self.cstr(name_rva)
            lookup = oft or ft
            k = 0
            while True:
                lo = self.off(lookup + 8 * k)
                ent = u64(d, lo)
                if ent == 0:
                    break
                if ent >> 63:
                    fname = '#%d' % (ent & 0xffff)
                else:
                    fname = self.cstr((ent & 0x7fffffff) + 2)
                out.append((ft + 8 * k, dll, fname))
                k += 1
            o += 20
        return out

    def relocs_dir64(self):
        """RVAs of all IMAGE_REL_BASED_DIR64 fixups."""
        rva, size = self.dd[5]
        d = self.data
        o = self.off(rva)
        end = o + size
        out = []
        while o < end:
            page, bsz = struct.unpack_from('<II', d, o)
            if bsz < 8:
                break
            n = (bsz - 8) // 2
            for e in struct.unpack_from('<%dH' % n, d, o + 8):
                if e >> 12 == 10:
                    out.append(page + (e & 0xfff))
            o += bsz
        return out

    def pdata(self):
        rva, size = self.dd[3]
        o = self.off(rva)
        n = size // 12
        return list(struct.iter_unpack('<III', self.data[o:o + 12 * n]))


# --------------------------------------------------------------------------------------------
# build
# --------------------------------------------------------------------------------------------
ASCII_RE = re.compile(rb'[\t\n\r\x20-\x7e]{3,}\x00')
WIDE_RE = re.compile(rb'(?:[\t\n\r\x20-\x7e]\x00){3,}\x00\x00')
QUALNAME_RE = re.compile(r'^[A-Za-z_]\w*(?:<[^()]*>)?(?:::~?[A-Za-z_]\w*(?:<[^()]*>)?)+$')
# FF /2, FF /3 (indirect call) and memory-indirect FF /4, FF /5 (virtual tail-jump; mod!=3,
# excluding rip-relative [iat] jumps which are handled separately). Register jumps (switches) excluded.
INDIRECT_MODRM = bytes(m for m in range(256) if ((m >> 3) & 7) in (2, 3) or
                       (((m >> 3) & 7) in (4, 5) and (m >> 6) != 3 and (m & 0xC7) != 0x05))
RE_E8 = re.compile(rb'\xe8', re.S)
RE_E9 = re.compile(rb'\xe9', re.S)
RE_FF15 = re.compile(rb'\xff\x15', re.S)
RE_FF25 = re.compile(rb'\xff\x25', re.S)
RE_LEA = re.compile(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', re.S)
RE_ICALL = re.compile(b'\xff[' + re.escape(INDIRECT_MODRM) + b']', re.S)
SECCOOKIE_RE = re.compile(rb'\x48\x3b\x0d....\x75.\x48\xc1\xc1\x10\x66\xf7\xc1\xff\xff\x75', re.S)


class Builder:
    def __init__(self, exe):
        log('reading %s' % exe)
        self.pe = PE(exe)
        self.names = collections.defaultdict(list)  # rva -> [(prio, name, src, detail)]

    def add_name(self, rva, name, src, detail='', prio=None):
        P = {'export': 0, 'manual': 1, 'import_thunk': 2, 'crt': 3, 'funcsig': 4, 'string': 5,
             'vftable': 6, 'thunk': 7, 'ghidra': 8}
        self.names[rva].append((P[src] if prio is None else prio, name, src, detail))

    # ---------------- functions from .pdata
    def parse_pdata(self):
        pe, d = self.pe, self.pe.data
        ents = pe.pdata()
        by_begin = {}
        info = {}
        for b, e, u in ents:
            by_begin[b] = (b, e, u)
        parent = {}
        handler = {}
        uflags = {}
        for b, e, u in ents:
            if u & 1:  # indirect: points to another RUNTIME_FUNCTION
                ro = pe.off(u & ~1)
                pb = u32(d, ro)
                parent[b] = pb
                continue
            uo = pe.off(u)
            if uo is None:
                continue
            vf = d[uo]
            fl = vf >> 3
            cnt = d[uo + 2]
            uflags[b] = fl
            tail = uo + 4 + ((cnt + 1) & ~1) * 2
            if fl & 4:
                parent[b] = u32(d, tail)
            elif fl & 3:
                handler[b] = u32(d, tail)

        def root(b):
            seen = 0
            while b in parent and seen < 64:
                b = parent[b]
                seen += 1
            return b
        self.funcs = {}   # rva -> dict
        self.frags = []   # (begin, end, root)
        for b, e, u in ents:
            if b in parent:
                continue
            self.funcs[b] = {'end': e, 'size': e - b, 'source': 'pdata', 'nfrag': 0,
                             'handler': handler.get(b), 'uflags': uflags.get(b, 0)}
        orphan = 0
        for b, e, u in ents:
            if b not in parent:
                continue
            r = root(b)
            if r not in self.funcs:
                orphan += 1
                self.funcs[b] = {'end': e, 'size': e - b, 'source': 'pdata_orphan_chain', 'nfrag': 0,
                                 'handler': None, 'uflags': uflags.get(b, 0)}
                continue
            self.frags.append((b, e, r))
            f = self.funcs[r]
            f['size'] += e - b
            f['nfrag'] += 1
        self.handlers = collections.Counter(h for h in handler.values())
        log('pdata: %d entries -> %d functions + %d chained fragments (%d orphan chains)'
            % (len(ents), len(self.funcs), len(self.frags), orphan))

    # ---------------- imports / exports / relocs
    def parse_dirs(self):
        pe, d = self.pe, self.pe.data
        self.imports = pe.imports()
        self.iat = {rva: (dll, fn) for rva, dll, fn in self.imports}
        self.exports = pe.exports()
        relocs = pe.relocs_dir64()
        self.reloc_sites = set(relocs)
        self.reloc_refs = []  # (site_rva, target_rva) for pointers into .text
        ib = pe.image_base
        for r in relocs:
            o = pe.off(r)
            if o is None:
                continue
            v = u64(d, o) - ib
            if pe.in_text(v):
                self.reloc_refs.append((r, v))
        log('imports=%d exports=%d dir64 relocs=%d (into .text: %d)'
            % (len(self.imports), len(self.exports), len(relocs), len(self.reloc_refs)))

    # ---------------- raw scans over .text
    def scan_text(self):
        pe, t = self.pe, self.pe.text
        base = pe.text_rva
        n = len(t)
        self.e8 = []
        for m in RE_E8.finditer(t):
            p = m.start()
            if p + 5 > n:
                break
            tgt = base + p + 5 + i32(t, p + 1)
            if base <= tgt < base + n:
                self.e8.append((base + p, tgt))
        self.e9 = []
        for m in RE_E9.finditer(t):
            p = m.start()
            if p + 5 > n:
                break
            tgt = base + p + 5 + i32(t, p + 1)
            if base <= tgt < base + n:
                self.e9.append((base + p, tgt))
        self.ff15 = []
        for m in RE_FF15.finditer(t):
            p = m.start()
            slot = base + p + 6 + i32(t, p + 2)
            if slot in self.iat:
                self.ff15.append((base + p, slot))
        self.ff25 = []
        for m in RE_FF25.finditer(t):
            p = m.start()
            slot = base + p + 6 + i32(t, p + 2)
            if slot in self.iat:
                self.ff25.append((base + p, slot))
        self.lea = []
        for m in RE_LEA.finditer(t):
            p = m.start()
            if p + 7 > n:
                break
            self.lea.append((base + p, base + p + 7 + i32(t, p + 3)))
        self.icall = [base + m.start() for m in RE_ICALL.finditer(t)]
        log('scan: e8->text=%d e9->text=%d call[iat]=%d jmp[iat]=%d lea-rip=%d indirect-call-pat=%d'
            % (len(self.e8), len(self.e9), len(self.ff15), len(self.ff25), len(self.lea), len(self.icall)))

    # ---------------- covered ranges helper
    def covered_ranges(self):
        rs = [(b, f['end'], b) for b, f in self.funcs.items()] + list(self.frags)
        rs.sort()
        return rs

    # ---------------- import thunks + gap functions
    def find_gap_functions(self):
        pe, t, base = self.pe, self.pe.text, self.pe.text_rva
        covered = self.covered_ranges()
        cstarts = [r[0] for r in covered]

        def inside_covered(x):
            i = bisect.bisect_right(cstarts, x) - 1
            return i >= 0 and covered[i][0] <= x < covered[i][1]
        # import thunks (jmp [iat]) outside any pdata function
        # A real linker import thunk sits at a function boundary (aligned / after padding /
        # directly after another thunk) or is an exact call target; other FF25s outside .pdata
        # are tail-jumps at the end of frameless leaf functions.
        e_targets = set(tg for _, tg in self.e8) | set(tg for _, tg in self.e9)
        ff25_sites = set(s for s, _ in self.ff25)
        self.thunks = {}
        for site, slot in sorted(self.ff25):
            if inside_covered(site):
                continue
            prev = t[site - base - 1] if site > base else 0xCC
            if (site % 16 == 0 or prev == 0xCC or site in e_targets or (site - 6) in self.thunks
                    or site in self.vft_targets):
                self.thunks[site] = slot
        # strong references (exact targets): relocs, vftables, exports, handlers, lea
        strong = set(v for _, v in self.reloc_refs)
        strong.update(self.vft_targets)
        strong.update(r for _, r, _ in self.exports)
        strong.update(self.handlers)
        strong.update(v for _, v in self.lea if pe.in_text(v))
        weak = set(tg for _, tg in self.e8) | set(tg for _, tg in self.e9)
        self.ref_strong = strong
        refs_sorted = sorted(t_ for t_ in (strong | weak) if not inside_covered(t_))
        gaps = []
        prev_end = base
        for b, e, _ in covered:
            if b > prev_end:
                gaps.append((prev_end, b))
            prev_end = max(prev_end, e)
        if prev_end < base + len(t):
            gaps.append((prev_end, base + len(t)))
        thunk_set = set(self.thunks)
        added = 0
        junk = 0
        jt = 0
        stats = collections.Counter()
        for a, b in gaps:
            g = t[a - base:b - base]
            if not g.strip(b'\xcc'):
                continue
            cands = set()
            if g[0] != 0xCC and a % 16 == 0:
                cands.add(a)
            for m in re.finditer(rb'\xcc+', g):
                e = m.end()
                if e < len(g) and (a + e) % 16 == 0:
                    cands.add(a + e)
            lo = bisect.bisect_left(refs_sorted, a)
            hi = bisect.bisect_left(refs_sorted, b)
            for x in refs_sorted[lo:hi]:
                if t[x - base] == 0xCC:
                    continue
                if x in strong or x % 16 == 0:
                    cands.add(x)
            for x in thunk_set_in(thunk_set, a, b):
                cands.add(x)
            if not cands:
                junk += 1
                continue
            cl = sorted(cands)
            for i, s in enumerate(cl):
                if s in self.thunks:
                    stats['import_thunk'] += 1
                    self.funcs[s] = {'end': s + 6, 'size': 6, 'source': 'import_thunk', 'nfrag': 0,
                                     'handler': None, 'uflags': 0}
                    continue
                e = cl[i + 1] if i + 1 < len(cl) else b
                chunk = t[s - base:e - base].rstrip(b'\xcc')
                if not chunk:
                    continue
                # jump tables (u32 RVAs into .text) and zero blocks are data, not code
                if len(chunk) >= 8 and chunk[:4] != b'\0\0\0\0':
                    v0, v1 = u32(chunk, 0), u32(chunk, 4)
                    if pe.in_text(v0) and pe.in_text(v1) and s not in strong:
                        jt += 1
                        continue
                if chunk[:4] == b'\0\0\0\0':
                    stats['zero'] += 1
                    continue
                src = 'gap_ref' if (s in strong or s in weak) else 'gap_unref'
                stats[src] += 1
                self.funcs[s] = {'end': s + len(chunk), 'size': len(chunk), 'source': src,
                                 'nfrag': 0, 'handler': None, 'uflags': 0}
                added += 1
        log('gaps: %d non-pdata functions added (%s); %d jump-table-like chunks skipped, %d gaps w/o start'
            % (added, dict(stats), jt, junk))

    # ---------------- vftables
    def load_rtti(self):
        pe, d = self.pe, self.pe.data
        with open(SYMMAP) as f:
            sm = json.load(f)
        self.symmap = sm
        self.classes = []
        self.vftables = []  # (vft_rva, class_idx, col_rva, nslots)
        self.vft_slots = []  # (vft_rva, slot, target_rva)
        self.vft_targets = set()
        jq2name = {}
        mism = 0
        vft_starts = set(int(vt['vftable_va'], 16) - pe.image_base
                         for c in sm['rtti_classes'] for vt in c.get('vftables', []))
        for ci, c in enumerate(sm['rtti_classes']):
            name = type_descriptor_name(c['mangled'])
            jq2name[c['qualified_name']] = name
            self.classes.append({'td': int(c['td_va'], 16), 'mangled': c['mangled'], 'name': name,
                                 'jq': c['qualified_name'], 'bases': c.get('base_classes', [])})
            for vt in c.get('vftables', []):
                vva = int(vt['vftable_va'], 16)
                vr = pe.rva(vva)
                o = pe.off(vr)
                if o is None:
                    continue
                col = u64(d, o - 8) - pe.image_base
                n = int(vt.get('method_count', 0))
                slots = []
                # Read slots straight from the image: every slot is a relocated pointer into
                # .text; stop at the first non-text / unrelocated slot or the next vftable's COL.
                for k in range(4096):
                    if k > 0 and (vr + 8 * k + 8) in vft_starts:
                        break
                    if (vr + 8 * k) not in self.reloc_sites:
                        break
                    tg = u64(d, o + 8 * k) - pe.image_base
                    if not pe.in_text(tg):
                        break
                    slots.append(tg)
                js = [int(x, 16) - pe.image_base for x in vt.get('methods', [])]
                if len(slots) != n or slots[:len(js)] != js:
                    mism += 1
                self.vftables.append((vr, ci, col, len(slots)))
                for k, tg in enumerate(slots):
                    self.vft_slots.append((vr, k, tg))
                    self.vft_targets.add(tg)
        self.jq2name = jq2name
        log('rtti: %d classes, %d vftables, %d slots read from image, %d distinct targets (%d vftables whose slot count differs from symbol_map method_count)'
            % (len(self.classes), len(self.vftables), len(self.vft_slots), len(self.vft_targets), mism))

    def name_vfuncs(self):
        by_target = collections.defaultdict(list)  # tgt -> [(class_idx, slot)]
        vft_class = {v[0]: v[1] for v in self.vftables}
        for vr, k, tg in self.vft_slots:
            by_target[tg].append((vft_class[vr], k))
        self.vft_by_target = by_target
        shared = 0
        for tg, lst in by_target.items():
            cis = sorted(set(ci for ci, _ in lst))
            if len(cis) == 1:
                ci = cis[0]
                slot = min(k for c, k in lst if c == ci)
                name = '%s::vfunc_%d' % (self.classes[ci]['name'], slot)
                self.add_name(tg, name, 'vftable')
                continue
            shared += 1
            # choose a class that is a base of all the others (deepest such), else first by slot
            jqs = {ci: self.classes[ci]['jq'] for ci in cis}
            best = None
            for ci in cis:
                ok = all(jqs[ci] in self.classes[o]['bases'] or o == ci for o in cis)
                if ok and (best is None or len(self.classes[ci]['bases']) > len(self.classes[best]['bases'])):
                    best = ci
            detail = 'shared by %d classes' % len(cis)
            if best is None:
                best = min(lst, key=lambda x: (x[1], len(self.classes[x[0]]['name'])))[0]
                detail += ' (no common base: likely ICF-folded)'
            slot = min(k for c, k in lst if c == best)
            self.add_name(tg, '%s::vfunc_%d' % (self.classes[best]['name'], slot), 'vftable', detail)
        log('vftable names: %d targets (%d shared between classes)' % (len(by_target), shared))

    # ---------------- other names
    def name_misc(self):
        pe, d, t, base = self.pe, self.pe.data, self.pe.text, self.pe.text_rva
        for ordi, rva, mname in self.exports:
            self.add_name(rva, symbol_qualified_name(mname), 'export', mname)
        for site, slot in self.thunks.items():
            dll, fn = self.iat[slot]
            nm = symbol_qualified_name(fn) if fn.startswith('?') else fn
            self.add_name(site, 'j_' + nm, 'import_thunk', dll)
        # manual names from earlier analysis (re/symbol_map.json hardware_gates)
        for k, v in self.symmap.get('hardware_gates', {}).items():
            if isinstance(v, dict) and 'rva' in v:
                self.add_name(int(v['rva'], 16), k, 'manual', 'symbol_map.hardware_gates')
        # CRT anchors
        ep = pe.entry_rva
        self.add_name(ep, 'WinMainCRTStartup', 'crt', 'entry point')
        eo = pe.off(ep)
        if d[eo:eo + 4] == b'\x48\x83\xec\x28' and d[eo + 4] == 0xE8:
            self.add_name(ep + 9 + i32(d, eo + 5), '__security_init_cookie', 'crt', 'called from entry')
            if d[eo + 13] == 0xE9:
                self.add_name(ep + 18 + i32(d, eo + 14), '__scrt_common_main_seh', 'crt', 'tail of entry')
        for m in SECCOOKIE_RE.finditer(t):
            if (base + m.start()) in self.funcs:
                self.add_name(base + m.start(), '__security_check_cookie', 'crt', 'byte pattern')
        for h, cnt in self.handlers.items():
            if h in self.thunks:
                continue
            self.add_name(h, 'crt_handler_%x' % pe.va(h), 'crt', 'unwind handler, %d users' % cnt)

    def scan_strings_and_funcsigs(self):
        pe, d = self.pe, self.pe.data
        rd = pe.sec('.rdata')
        lo, hi = rd[1], rd[1] + rd[2]
        self.strings = {}
        cache = {}

        def string_at(rva):
            if rva in cache:
                return cache[rva]
            r = None
            o = pe.off(rva)
            if o is not None:
                m = ASCII_RE.match(d, o, o + 2048)
                if m:
                    r = ('a', d[o:m.end() - 1].decode('latin1'))
                else:
                    m = WIDE_RE.match(d, o, o + 4096)
                    if m:
                        r = ('w', d[o:m.end() - 2].decode('utf-16le', 'replace'))
            cache[rva] = r
            return r
        self.strrefs = []  # (site, str_rva)
        for site, tg in self.lea:
            if lo <= tg < hi or (pe.sec_of(tg) is not None and pe.sec_of(tg)[0] in ('.data', '_RDATA')):
                s = string_at(tg)
                if s:
                    self.strings[tg] = s
                    self.strrefs.append((site, tg))
        log('strings: %d referenced strings, %d lea string refs' % (len(self.strings), len(self.strrefs)))
        # __FUNCSIG__ strings anywhere in .rdata
        rdo = rd[3]
        blob = d[rdo:rdo + rd[4]]
        seen = set()
        self.funcsigs = []
        self.funcsig_ranges = []
        for m in re.finditer(rb'__cdecl', blob):
            p = m.start()
            s = blob.rfind(b'\0', max(0, p - 4096), p) + 1
            e = blob.find(b'\0', p)
            if s in seen or e < 0:
                continue
            seen.add(s)
            raw = blob[s:e]
            if not raw or any(c < 0x20 or c > 0x7e for c in raw):
                continue
            text = raw.decode('latin1')
            parsed = parse_funcsig(text)
            if parsed:
                self.funcsigs.append((lo + s, ) + parsed + (text,))
                self.funcsig_ranges.append((lo + s, lo + e, len(self.funcsigs) - 1))
        log('funcsigs: %d __cdecl signature strings' % len(self.funcsigs))

    # ---------------- assemble
    def finalize(self):
        pe = self.pe
        # ranges incl. fragments for site->function mapping
        rs = [(b, f['end'], b) for b, f in self.funcs.items()] + list(self.frags)
        rs.sort()
        self.rs = rs
        self.rstarts = [r[0] for r in rs]
        starts = set(self.funcs)

        def owner(x):
            i = bisect.bisect_right(self.rstarts, x) - 1
            if i >= 0 and rs[i][0] <= x < rs[i][1]:
                return rs[i][2]
            return None
        self.owner = owner
        calls = set()
        tails = set()
        has_call = collections.Counter()
        for site, tg in self.e8:
            f = owner(site)
            if f is None:
                continue
            has_call[f] += 1
            if tg in starts:
                calls.add((f, tg, site, 'call'))
        for site, tg in self.e9:
            f = owner(site)
            if f is None:
                continue
            of = owner(tg)
            if of == f:
                continue
            has_call[f] += 1
            if tg in starts:
                tails.add((f, tg, site, 'tail'))
        self.calls = sorted(calls | tails)
        self.import_calls = []
        for site, slot in self.ff15:
            f = owner(site)
            if f is not None:
                has_call[f] += 1
                self.import_calls.append((f, site) + self.iat[slot] + ('call',))
        for site, slot in self.ff25:
            f = owner(site)
            if f is not None and site not in self.thunks:
                has_call[f] += 1
                self.import_calls.append((f, site) + self.iat[slot] + ('tail',))
        icall = collections.Counter()
        for site in self.icall:
            f = owner(site)
            if f is not None:
                icall[f] += 1
        self.has_call, self.icall_cnt = has_call, icall
        # xrefs: lea to function starts, data pointers to function starts
        self.xrefs = []
        for site, tg in self.lea:
            if tg in starts:
                self.xrefs.append((owner(site), site, tg, 'lea'))
        for site, tg in self.reloc_refs:
            if tg in starts:
                sec = pe.sec_of(site)
                self.xrefs.append((owner(site) if sec and sec[0] == '.text' else None, site, tg,
                                   'data:' + (sec[0] if sec else '?')))
        log('call graph: %d direct edges (%d tail), %d import calls, %d fn-address refs'
            % (len(self.calls), len(tails), len(self.import_calls), len(self.xrefs)))
        # funcsig xrefs
        # code often LEAs a *substring* of the __FUNCSIG__ (constexpr type-name slicing), so map
        # any LEA target inside a signature string back to that string
        fr = sorted(self.funcsig_ranges)
        frs = [x[0] for x in fr]
        refs_by_sig = collections.defaultdict(set)
        for site, tg in self.lea:
            i = bisect.bisect_right(frs, tg) - 1
            if i >= 0 and fr[i][0] <= tg < fr[i][1]:
                f = owner(site)
                if f is not None:
                    refs_by_sig[fr[i][2]].add(f)
        self.funcsig_rows = []
        fs_per_func = collections.defaultdict(list)
        for si, row in enumerate(self.funcsigs):
            refs = sorted(refs_by_sig.get(si, ()))
            for f in refs or [None]:
                self.funcsig_rows.append(row + (f,))
                if f is not None:
                    fs_per_func[f].append(row)
        named_fs = 0
        for f, rows in fs_per_func.items():
            ecs = [r for r in rows if r[1] == 'ecs_system']
            if len(ecs) == 1 and self.funcs[f]['size'] > 0x100 and ecs[0][3]:
                # big function using exactly one system name: the per-system wrapper/invoker
                self.add_name(f, 'ecs::system::wrapper<%s>' % ecs[0][3], 'funcsig',
                              'heuristic: only ecs getName<> funcsig referenced', prio=6)
                named_fs += 1
                continue
            if len(rows) == 1 and self.funcs[f]['size'] <= 0x100:
                kind, outer, inner = rows[0][1], rows[0][2], rows[0][3]
                nm = outer
                if kind == 'ecs_system' and inner:
                    nm = '%s<%s>' % (outer.split('<')[0], inner)
                elif kind == 'type' and inner:
                    nm = '%s<%s>' % (outer.split('<')[0], inner)
                self.add_name(f, nm, 'funcsig', 'refs __FUNCSIG__ string')
                named_fs += 1
        # qualified-name strings (__FUNCTION__ / profiler scopes)
        qual_per_func = collections.defaultdict(set)
        for site, tg in self.strrefs:
            k, s = self.strings[tg]
            if k == 'a' and 6 <= len(s) <= 300 and '::' in s and QUALNAME_RE.match(s):
                f = owner(site)
                if f is not None:
                    qual_per_func[f].add(s)
        self.qual_per_func = qual_per_func
        named_str = 0
        for f, ss in qual_per_func.items():
            if len(ss) != 1:
                continue
            s = next(iter(ss))
            last = split_qualified(s)[-1]
            if not (last[:1].islower() or last.startswith('~')):
                continue  # PascalCase tail: probably a type name (reflection), not a function
            existing = self.names.get(f, [])
            vcls = [n for p, n, src, _ in existing if src == 'vftable']
            if any(src in ('export', 'funcsig', 'manual') for _, _, src, _ in existing):
                continue
            scope = '::'.join(split_qualified(s)[:-1])
            if vcls and not any(v.startswith(scope + '::') for v in vcls):
                self.add_name(f, s, 'string', 'only qualified-name string referenced (conflicts with vftable class)', prio=9)
                continue
            self.add_name(f, s, 'string', 'only qualified-name string referenced')
            named_str += 1
        log('names from strings: %d funcsig-named, %d qualified-name-string-named' % (named_fs, named_str))
        # jmp thunks: function body is a single E9 to a named function
        t, base = pe.text, pe.text_rva
        for b, f in self.funcs.items():
            if f['size'] == 5 and t[b - base] == 0xE9:
                tg = b + 5 + i32(t, b - base + 1)
                if tg in self.names:
                    best = min(self.names[tg])
                    self.add_name(b, 'j_' + best[1], 'thunk', 'jmp thunk')

    def load_ghidra(self, path):
        """Merge AW2ExportFunctions.java output: missing functions, Ghidra-only names (Function ID
        matches such as CRT/zlib routines), direct call edges for leafness."""
        self.ghidra_rows, self.ghidra_calls = [], []
        self.ghidra_callers = set()
        if not path or not os.path.exists(path):
            return
        ib = self.pe.image_base
        added = named = alts = 0
        with open(path) as f:
            for line in f:
                line = line.strip()
                if not line.startswith('{'):
                    continue
                o = json.loads(line)
                va = int(o['va'], 16)
                rva = va - ib
                cl = [int(c, 16) - ib for c in o.get('callees', [])]
                end = int(o['end'], 16) - ib if o.get('end') else None
                th = int(o['thunk'], 16) - ib if o.get('thunk') else None
                name = o.get('name') or ''
                self.ghidra_rows.append((rva, end, o.get('size'), name, o.get('sig'), th,
                                         1 if o.get('noreturn') else 0, len(cl)))
                self.ghidra_calls += [(rva, c) for c in cl]
                if cl or th:
                    self.ghidra_callers.add(rva)
                if rva not in self.funcs and o.get('size') and self.pe.in_text(rva):
                    self.funcs[rva] = {'end': end or rva + o['size'], 'size': o['size'], 'source': 'ghidra',
                                       'nfrag': 0, 'handler': None, 'uflags': 0}
                    added += 1
                if not name or name.startswith(('FUN_', 'thunk_FUN_', 'LAB_', 'SUB_', 'Ordinal_', 'entry')):
                    continue
                ours = self.names.get(rva, [])
                if any(n == name or shorten_for_ghidra(n).replace(' ', '_') == name for _, n, _, _ in ours):
                    continue  # our own name round-tripped through Ghidra
                if '.DLL::' in name.upper():
                    name = 'j_' + name.split('::', 1)[1]
                    self.add_name(rva, name, 'import_thunk', 'ghidra: thunk to import')
                    named += 1
                    continue
                if name.startswith('FID_conflict:') or ours:
                    self.add_name(rva, name, 'ghidra', 'ghidra (alternative)', prio=10)
                    alts += 1
                    continue
                self.add_name(rva, name, 'ghidra', 'ghidra auto-analysis / Function ID')
                named += 1
        log('ghidra: %d functions (%d new to fndb), %d names adopted, %d alternative names'
            % (len(self.ghidra_rows), added, named, alts))

    def assign(self, use_overrides=True):
        """Pick primary names and namespaces; ownership via owner.Classifier (evidence fusion)."""
        order = sorted(self.funcs)
        for b in order:
            f = self.funcs[b]
            cand = self.names.get(b)
            if cand:
                p, n, src, det = min(cand)
                f['name'], f['name_src'] = n, src
                f['top_ns'] = 'crt' if src in ('crt', 'import_thunk') else top_ns_of(n)
                if n.startswith('ecs::system::wrapper<'):
                    f['top_ns'] = top_ns_of(n[len('ecs::system::wrapper<'):-1])  # attribute to the system
                f['ns_src'] = 'name'
                if src == 'import_thunk':
                    f['name_owner'] = 'crt'
                else:
                    f['name_owner'] = owner_of(f['top_ns'], n)
                if src == 'vftable' and len(set(ci for ci, _ in self.vft_by_target.get(b, []))) > 1:
                    owners = collections.Counter(
                        owner_of(top_ns_of(self.classes[ci]['name'])) for ci, _ in self.vft_by_target[b])
                    f['name_owner'] = owners.most_common(1)[0][0]
            else:
                f['name'] = None
                f['name_src'] = None
                f['top_ns'] = None
                f['name_owner'] = None
        # source-file strings
        srcfile = {}
        for site, tg in self.strrefs:
            k, s = self.strings[tg]
            sl = s.replace('\\', '/').lower()
            if sl.endswith(('.cpp', '.h', '.hpp', '.inl', '.c', '.cc')) and '/' in sl:
                fo = self.owner(site)
                if fo is not None and fo not in srcfile:
                    srcfile[fo] = s
        self.srcfile = srcfile
        for fo, s in srcfile.items():
            self.funcs[fo]['src_file'] = s
        # ownership: evidence fusion (tools/fndb/owner.py)
        clf = ownermod.Classifier(self, use_overrides=use_overrides, log=log)
        res = clf.run()
        ownermod.apply(self, res, clf.ev)
        self.owner_stats = clf.stats
        # namespace neighbour inference (for `next --ns`): unnamed remedy functions between two
        # named functions of the same Remedy top namespace
        named = [b for b in order if self.funcs[b].get('name') and self.funcs[b]['source'] != 'import_thunk']
        idx = {b: i for i, b in enumerate(order)}
        inferred = 0
        for a, c in zip(named, named[1:]):
            ia, ic = idx[a], idx[c]
            fa, fc = self.funcs[a], self.funcs[c]
            if ic - ia <= 1 or fa['top_ns'] != fc['top_ns'] or fa['owner'] != 'remedy' or fc['owner'] != 'remedy':
                continue
            for j in range(ia + 1, ic):
                f = self.funcs[order[j]]
                if f['owner'] == 'remedy' and f.get('top_ns') in (None, 'unknown'):
                    f['top_ns'], f['ns_src'] = fa['top_ns'], 'neighbor'
                    inferred += 1
        cnt = collections.Counter(f['owner'] for f in self.funcs.values())
        log('ownership: %s; %d remedy functions got a namespace from neighbours'
            % (', '.join('%s=%d' % kv for kv in sorted(cnt.items())), inferred))

    def leafness(self):
        callers = collections.Counter()
        callees = collections.defaultdict(set)
        for f, tg, site, kind in self.calls:
            callees[f].add(tg)
        for f, s in callees.items():
            for tg in s:
                callers[tg] += 1
        for b, f in self.funcs.items():
            f['n_callees'] = len(callees.get(b, ()))
            f['n_callers'] = callers.get(b, 0)
            f['is_leaf'] = 1 if (self.has_call.get(b, 0) == 0 and self.icall_cnt.get(b, 0) == 0
                                 and b not in getattr(self, 'ghidra_callers', ())) else 0

    def write(self, path):
        pe = self.pe
        ib = pe.image_base
        final_path = path
        path = path[:-7] + '.tmp.sqlite' if path.endswith('.sqlite') else path + '.tmp'   # gitignored
        if os.path.exists(path):
            os.remove(path)
        os.makedirs(os.path.dirname(os.path.abspath(path)), exist_ok=True)
        db = sqlite3.connect(path)
        db.executescript(SCHEMA)
        meta = {'exe': pe.path, 'image_base': hex(ib), 'built_at': time.strftime('%Y-%m-%d %H:%M:%S'),
                'exe_size': str(len(pe.data)), 'pdata_entries': str(len(pe.pdata())), 'fndb_version': '2',
                'fnsig': 'yes' if getattr(self, 'have_fnsig', False) else 'no',
                'owner_overrides': str(len(ownermod.load_overrides()))}
        db.executemany('INSERT INTO meta VALUES (?,?)', meta.items())
        rows = []
        for b, f in sorted(self.funcs.items()):
            rows.append((ib + b, ib + f['end'], f['size'], f.get('name'), f['top_ns'],
                         namespace_of(f.get('name')), f['owner'], f.get('owner_src'), f.get('ns_src'),
                         f['source'], f.get('name_src'), f['nfrag'],
                         ib + f['handler'] if f.get('handler') else None, f.get('uflags', 0),
                         f['is_leaf'], f['n_callees'], f['n_callers'], f.get('src_file'),
                         f.get('owner_conf'), f.get('owner_lib')))
        db.executemany('INSERT INTO functions VALUES (%s)' % ','.join('?' * 20), rows)
        db.executemany('INSERT INTO owner_evidence VALUES (?,?,?,?,?,?)',
                       [(ib + f, o, l, sc, src, det) for f, o, l, sc, src, det in getattr(self, 'owner_evidence', [])])
        db.executemany('INSERT INTO fragments VALUES (?,?,?)', [(ib + a, ib + b, ib + r) for a, b, r in self.frags])
        nrows = []
        for rva, lst in self.names.items():
            for p, n, src, det in sorted(lst):
                nrows.append((ib + rva, n, src, det, p))
        db.executemany('INSERT INTO names VALUES (?,?,?,?,?)', nrows)
        db.executemany('INSERT INTO classes VALUES (?,?,?,?,?,?)',
                       [(i, ib + 0 + (c['td'] - ib), c['mangled'], c['name'], top_ns_of(c['name'] + '::x'),
                         json.dumps(c['bases'])) for i, c in enumerate(self.classes)])
        db.executemany('INSERT INTO vftables VALUES (?,?,?,?,?)',
                       [(ib + v, ci, self.classes[ci]['name'], ib + col, n) for v, ci, col, n in self.vftables])
        db.executemany('INSERT INTO vftable_slots VALUES (?,?,?)',
                       [(ib + v, k, ib + tg) for v, k, tg in self.vft_slots])
        db.executemany('INSERT INTO calls VALUES (?,?,?,?)',
                       [(ib + f, ib + tg, ib + s, k) for f, tg, s, k in self.calls])
        db.executemany('INSERT INTO import_calls VALUES (?,?,?,?,?)',
                       [(ib + f, ib + s, dll, fn, k) for f, s, dll, fn, k in self.import_calls])
        db.executemany('INSERT INTO imports VALUES (?,?,?,?)',
                       [(ib + r, dll, fn, symbol_qualified_name(fn)) for r, dll, fn in self.imports])
        db.executemany('INSERT INTO exports VALUES (?,?,?,?)',
                       [(ib + r, o, m, symbol_qualified_name(m)) for o, r, m in self.exports])
        db.executemany('INSERT INTO xrefs VALUES (?,?,?,?)',
                       [(ib + f if f is not None else None, ib + s, ib + tg, k) for f, s, tg, k in self.xrefs])
        db.executemany('INSERT INTO strings VALUES (?,?,?)',
                       [(ib + r, k, s) for r, (k, s) in self.strings.items()])
        db.executemany('INSERT INTO strrefs VALUES (?,?,?)',
                       [(ib + self.owner(s) if self.owner(s) is not None else None, ib + s, ib + tg)
                        for s, tg in self.strrefs])
        db.executemany('INSERT INTO funcsigs VALUES (?,?,?,?,?,?,?)',
                       [(ib + r[0], r[1], r[2], r[3], r[4], r[5], ib + r[6] if r[6] is not None else None)
                        for r in self.funcsig_rows])
        db.executemany('INSERT OR REPLACE INTO ghidra_functions VALUES (?,?,?,?,?,?,?,?)',
                       [(ib + r[0], ib + r[1] if r[1] is not None else None, r[2], r[3], r[4],
                         ib + r[5] if r[5] is not None else None, r[6], r[7]) for r in self.ghidra_rows])
        db.executemany('INSERT INTO ghidra_calls VALUES (?,?)', [(ib + a, ib + b) for a, b in self.ghidra_calls])
        db.executescript(INDEXES)
        db.commit()
        db.close()
        os.replace(path, final_path)
        path = final_path
        log('wrote %s (%.1f MB)' % (path, os.path.getsize(path) / 1e6))


def thunk_set_in(s, a, b):
    return [x for x in s if a <= x < b] if len(s) < 64 else _thunks_in_sorted(s, a, b)


_TS_CACHE = {}


def _thunks_in_sorted(s, a, b):
    key = id(s)
    if key not in _TS_CACHE:
        _TS_CACHE[key] = sorted(s)
    srt = _TS_CACHE[key]
    return srt[bisect.bisect_left(srt, a):bisect.bisect_left(srt, b)]


def namespace_of(name):
    if not name:
        return None
    parts = split_qualified(name)
    return '::'.join(parts[:-1]) if len(parts) > 1 else ''


def _top_level_find(s, needle, start=0):
    depth = 0
    i = start
    while i < len(s):
        c = s[i]
        if depth == 0 and s.startswith(needle, i):
            return i
        if c in '<(':
            depth += 1
        elif c in '>)':
            depth -= 1
        i += 1
    return -1


def _strip_tag(t):
    for p in ('class ', 'struct ', 'enum ', 'union '):
        if t.startswith(p):
            return t[len(p):]
    return t


def parse_funcsig(text):
    """Parse an MSVC __FUNCSIG__ string. Returns (kind, outer_name, inner, signature) or None."""
    k = _top_level_find(text, '__cdecl ')
    if k < 0:
        return None
    rest = text[k + 8:]
    p = _top_level_find(rest, '(')
    if p < 0:
        return None
    outer = rest[:p].strip()
    if not outer or ' ' in outer.split('<')[0]:
        return None
    base = outer.split('<')[0]
    inner = None
    lt = outer.find('<')
    if lt >= 0 and outer.endswith('>'):
        targ = outer[lt + 1:-1]
        if '__cdecl ' in targ:
            k2 = _top_level_find(targ, '__cdecl ')
            r2 = targ[k2 + 8:] if k2 >= 0 else targ.split('__cdecl ', 1)[1]
            p2 = _top_level_find(r2, '(')
            inner = r2[:p2].strip() if p2 >= 0 else r2.strip()
            kind = 'ecs_system' if base.endswith('getName') else 'templ_func'
            return (kind, outer, inner, targ)
        inner = _strip_tag(targ.strip())
        kind = 'type' if base.endswith(('getTypeName', 'TypeName', 'typeName')) else 'templ'
        return (kind, outer, inner, None)
    return ('func', outer, None, None)


SCHEMA = """
CREATE TABLE meta(key TEXT PRIMARY KEY, value TEXT);
CREATE TABLE functions(
  va INTEGER PRIMARY KEY, "end" INTEGER, size INTEGER, name TEXT, top_ns TEXT, namespace TEXT,
  owner TEXT, owner_src TEXT, ns_src TEXT, source TEXT, name_src TEXT, n_frags INTEGER,
  handler INTEGER, unwind_flags INTEGER, is_leaf INTEGER, n_callees INTEGER, n_callers INTEGER,
  src_file TEXT, owner_confidence REAL, owner_lib TEXT);
CREATE TABLE owner_evidence(va INTEGER, owner TEXT, lib TEXT, score REAL, src TEXT, detail TEXT);
CREATE TABLE fragments(va INTEGER, "end" INTEGER, func_va INTEGER);
CREATE TABLE names(va INTEGER, name TEXT, src TEXT, detail TEXT, prio INTEGER);
CREATE TABLE classes(id INTEGER PRIMARY KEY, td_va INTEGER, mangled TEXT, name TEXT, top_ns TEXT, bases TEXT);
CREATE TABLE vftables(va INTEGER PRIMARY KEY, class_id INTEGER, class TEXT, col_va INTEGER, n_slots INTEGER);
CREATE TABLE vftable_slots(vftable_va INTEGER, slot INTEGER, target INTEGER);
CREATE TABLE calls(caller INTEGER, callee INTEGER, site INTEGER, kind TEXT);
CREATE TABLE import_calls(caller INTEGER, site INTEGER, dll TEXT, func TEXT, kind TEXT);
CREATE TABLE imports(iat_va INTEGER PRIMARY KEY, dll TEXT, func TEXT, demangled TEXT);
CREATE TABLE exports(va INTEGER, ordinal INTEGER, mangled TEXT, name TEXT);
CREATE TABLE xrefs(func INTEGER, site INTEGER, target INTEGER, kind TEXT);
CREATE TABLE strings(va INTEGER PRIMARY KEY, kind TEXT, text TEXT);
CREATE TABLE strrefs(func INTEGER, site INTEGER, str_va INTEGER);
CREATE TABLE funcsigs(str_va INTEGER, kind TEXT, outer TEXT, inner TEXT, inner_sig TEXT, text TEXT, ref_func INTEGER);
CREATE TABLE ghidra_functions(va INTEGER PRIMARY KEY, "end" INTEGER, size INTEGER, name TEXT,
  signature TEXT, thunk_target INTEGER, noreturn INTEGER, n_callees INTEGER);
CREATE TABLE ghidra_calls(caller INTEGER, callee INTEGER);
"""
INDEXES = """
CREATE INDEX i_fn_name ON functions(name);
CREATE INDEX i_fn_ns ON functions(top_ns);
CREATE INDEX i_frag ON fragments(func_va);
CREATE INDEX i_names_va ON names(va);
CREATE INDEX i_names_name ON names(name);
CREATE INDEX i_slots_t ON vftable_slots(target);
CREATE INDEX i_calls_caller ON calls(caller);
CREATE INDEX i_calls_callee ON calls(callee);
CREATE INDEX i_icalls ON import_calls(caller);
CREATE INDEX i_xrefs_t ON xrefs(target);
CREATE INDEX i_xrefs_f ON xrefs(func);
CREATE INDEX i_strrefs_f ON strrefs(func);
CREATE INDEX i_strrefs_s ON strrefs(str_va);
CREATE INDEX i_funcsigs_f ON funcsigs(ref_func);
CREATE INDEX i_owner_ev ON owner_evidence(va);
"""


def cmd_build(args):
    b = Builder(args.exe)
    b.parse_pdata()
    b.parse_dirs()
    b.load_rtti()
    b.scan_text()
    b.find_gap_functions()
    b.name_vfuncs()
    b.name_misc()
    b.scan_strings_and_funcsigs()
    b.finalize()
    gj = getattr(args, 'ghidra', None) or os.path.join(ROOT, 'db', 'ghidra_functions.jsonl')
    if not args.no_ghidra and os.path.exists(gj):
        b.load_ghidra(gj)
    else:
        b.load_ghidra(None)
    b.have_fnsig = run_fnsig(b, args)
    b.assign(use_overrides=not getattr(args, 'no_overrides', False))
    b.leafness()
    b.write(args.db)
    if b.have_fnsig:
        import canthrow
        db = sqlite3.connect(args.db)
        canthrow.build(db, os.path.join(ROOT, 'out', 'fndb'), args.exe, log=log)
        db.close()
    print_summary(args.db)


def run_fnsig(b, args):
    """Normalized instruction features for every function (tools/fndb/fnsig, Rust/iced-x86) ->
    out/fndb/exe_feats.tsv. Optional: ownership falls back to the remaining evidence without it."""
    if getattr(args, 'no_fnsig', False):
        if os.path.exists(ownermod.EXE_FEATS):
            os.remove(ownermod.EXE_FEATS)
        return False
    import libcorpus
    try:
        fnsig = libcorpus.ensure_fnsig()
    except (OSError, subprocess.CalledProcessError) as e:
        log('fnsig unavailable (%s); ownership without instruction features' % e)
        if os.path.exists(ownermod.EXE_FEATS):
            os.remove(ownermod.EXE_FEATS)
        return False
    os.makedirs(os.path.dirname(ownermod.EXE_FEATS), exist_ok=True)
    ib = b.pe.image_base
    frags = collections.defaultdict(list)
    for a, e, r in b.frags:
        frags[r].append('%x-%x' % (ib + a, ib + e))
    lst = ownermod.EXE_FEATS + '.funcs'
    with open(lst, 'w') as f:
        for rva, fn in sorted(b.funcs.items()):
            f.write('\t'.join(['%x' % (ib + rva), '%x' % (ib + fn['end'])] + frags.get(rva, [])) + '\n')
    subprocess.run([fnsig, 'exe', b.pe.path, lst, ownermod.EXE_FEATS], check=True,
                   stderr=subprocess.DEVNULL)
    log('fnsig: instruction features -> %s' % os.path.relpath(ownermod.EXE_FEATS, ROOT))
    return True


def print_summary(dbp):
    db = sqlite3.connect(dbp)
    q = lambda s: db.execute(s).fetchall()
    print('functions by source:')
    for src, n, by in q('SELECT source, count(*), sum(size) FROM functions GROUP BY source ORDER BY 2 DESC'):
        print('  %-20s %8d  %10d bytes' % (src, n, by or 0))
    print('names by source:')
    for src, n in q('SELECT coalesce(name_src,"(unnamed)"), count(*) FROM functions GROUP BY 1 ORDER BY 2 DESC'):
        print('  %-20s %8d' % (src, n))


# --------------------------------------------------------------------------------------------
# progress tracking
# --------------------------------------------------------------------------------------------
def load_progress():
    rows = {}
    if not os.path.exists(PROGRESS):
        return rows
    with open(PROGRESS) as f:
        for line in f:
            line = line.rstrip('\n')
            if not line or line.startswith('#') or line.startswith('va\t'):
                continue
            c = line.split('\t')
            c += [''] * (5 - len(c))
            rows[int(c[0], 16)] = {'name': c[1], 'status': c[2], 'rust_path': c[3], 'notes': c[4]}
    return rows


def save_progress(rows):
    os.makedirs(os.path.dirname(PROGRESS), exist_ok=True)
    with open(PROGRESS, 'w') as f:
        f.write('va\tname\tstatus\trust_path\tnotes\n')
        for va in sorted(rows):
            r = rows[va]
            if r['status'] == 'todo':
                continue
            clean = lambda s: (s or '').replace('\t', ' ').replace('\n', ' ')
            f.write('0x%x\t%s\t%s\t%s\t%s\n' % (va, clean(r['name']), r['status'], clean(r['rust_path']),
                                                clean(r['notes'])))


def open_db(required=True):
    if not os.path.exists(DB_PATH):
        if required:
            raise SystemExit('no %s - run: tools/fndb.sh build' % DB_PATH)
        return None
    db = sqlite3.connect(DB_PATH)
    db.row_factory = sqlite3.Row
    return db


def resolve(db, key, limit=20):
    """va (hex/dec) or name substring -> list of function rows."""
    k = key.strip()
    try:
        va = int(k, 16) if k.lower().startswith('0x') or re.fullmatch(r'[0-9a-fA-F]{9,}', k) else int(k)
        if va < 0x140000000:
            va += 0x140000000  # accept RVAs
        r = db.execute('SELECT * FROM functions WHERE va=?', (va,)).fetchall()
        if r:
            return r
        r = db.execute('SELECT * FROM functions WHERE va<=? ORDER BY va DESC LIMIT 1', (va,)).fetchall()
        if r and va < r[0]['end']:
            return r
        fr = db.execute('SELECT func_va FROM fragments WHERE va<=? AND "end">?', (va, va)).fetchall()
        if fr:
            return db.execute('SELECT * FROM functions WHERE va=?', (fr[0][0],)).fetchall()
        return []
    except ValueError:
        pass
    r = db.execute('SELECT * FROM functions WHERE name=? LIMIT ?', (k, limit)).fetchall()
    if r:
        return r
    r = db.execute('SELECT DISTINCT f.* FROM names n JOIN functions f ON f.va=n.va WHERE n.name LIKE ? '
                   'ORDER BY f.size LIMIT ?', ('%' + k + '%', limit)).fetchall()
    return r


def fmt_fn(r):
    return '0x%x %-6d %-10s %-14s %s' % (r['va'], r['size'], r['owner'], r['top_ns'], r['name'] or '-')


def cmd_show(args):
    db = open_db()
    rows = resolve(db, args.key, args.limit)
    if not rows:
        raise SystemExit('no match for %r' % args.key)
    if len(rows) > 1:
        print('%d matches (showing list; pass a VA for details):' % len(rows))
        for r in rows:
            print('  ' + fmt_fn(r))
        return
    r = rows[0]
    va = r['va']
    prog = load_progress().get(va)
    print('function 0x%x - 0x%x  size=%d  (rva 0x%x)' % (va, r['end'], r['size'], va - 0x140000000))
    print('  name       : %s   [name_src=%s]' % (r['name'], r['name_src']))
    print('  namespace  : top=%s ns=%s [ns_src=%s]' % (r['top_ns'], r['namespace'], r['ns_src']))
    keys = r.keys()
    conf = r['owner_confidence'] if 'owner_confidence' in keys else None
    print('  owner      : %s [%s]%s%s' % (r['owner'], r['owner_src'],
                                         ('  confidence=%.2f' % conf) if conf is not None else '',
                                         ('  lib=%s' % r['owner_lib']) if 'owner_lib' in keys and r['owner_lib'] else ''))
    if db.execute("SELECT name FROM sqlite_master WHERE name='owner_evidence'").fetchone():
        for e in db.execute('SELECT owner, lib, score, src, detail FROM owner_evidence WHERE va=? '
                            'ORDER BY score DESC LIMIT 8', (va,)):
            print('    evidence : %-10s %-14s %.2f %-13s %s' % (e[0], e[1], e[2], e[3], (e[4] or '')[:90]))
    print('  source     : %s  fragments=%d  unwind_flags=%d  handler=%s' % (
        r['source'], r['n_frags'], r['unwind_flags'] or 0, hex(r['handler']) if r['handler'] else '-'))
    print('  leaf       : %s (approx; byte-pattern based)  callees=%d callers=%d' % (
        'yes' if r['is_leaf'] else 'no', r['n_callees'], r['n_callers']))
    if r['src_file']:
        print('  src file   : %s' % r['src_file'])
    print('  progress   : %s' % (('%s rust=%s notes=%s' % (prog['status'], prog['rust_path'], prog['notes']))
                                 if prog else 'todo'))
    g = db.execute('SELECT * FROM ghidra_functions WHERE va=?', (va,)).fetchone()
    if g:
        print('  ghidra     : %s  %s' % (g['name'], g['signature']))
    for fr in db.execute('SELECT * FROM fragments WHERE func_va=?', (va,)):
        print('  fragment   : 0x%x - 0x%x' % (fr['va'], fr['end']))
    alts = db.execute('SELECT * FROM names WHERE va=? ORDER BY prio', (va,)).fetchall()
    if len(alts) > 1 or (alts and alts[0]['detail']):
        print('  names:')
        for a in alts:
            print('    [%s] %s %s' % (a['src'], a['name'], ('(' + a['detail'] + ')') if a['detail'] else ''))
    sl = db.execute('SELECT v.class, s.slot, v.va FROM vftable_slots s JOIN vftables v ON v.va=s.vftable_va '
                    'WHERE s.target=? ORDER BY s.slot LIMIT 30', (va,)).fetchall()
    if sl:
        n = db.execute('SELECT count(*) FROM vftable_slots WHERE target=?', (va,)).fetchone()[0]
        print('  vftable slots (%d):' % n)
        for c in sl:
            print('    %s  slot %d  (vftable 0x%x)' % (c[0], c[1], c[2]))
    cs = db.execute('SELECT DISTINCT c.caller, f.name FROM calls c LEFT JOIN functions f ON f.va=c.caller '
                    'WHERE c.callee=? LIMIT ?', (va, args.limit)).fetchall()
    if cs:
        print('  callers (%d shown):' % len(cs))
        for c in cs:
            print('    0x%x %s' % (c[0], c[1] or ''))
    ce = db.execute('SELECT DISTINCT c.callee, f.name, c.kind FROM calls c LEFT JOIN functions f ON f.va=c.callee '
                    'WHERE c.caller=? ORDER BY c.site', (va,)).fetchall()
    if ce:
        print('  callees:')
        for c in ce[:args.limit * 2]:
            print('    0x%x %s%s' % (c[0], c[1] or '', ' (tail)' if c[2] == 'tail' else ''))
    ic = db.execute('SELECT DISTINCT dll, func FROM import_calls WHERE caller=?', (va,)).fetchall()
    if ic:
        print('  imports called: ' + ', '.join('%s!%s' % (a, b) for a, b in ic))
    xr = db.execute('SELECT func, site, kind FROM xrefs WHERE target=? LIMIT 20', (va,)).fetchall()
    if xr:
        print('  address taken at:')
        for x in xr:
            print('    0x%x (%s)%s' % (x[1], x[2], (' in 0x%x' % x[0]) if x[0] else ''))
    st = db.execute('SELECT DISTINCT s.va, s.kind, s.text FROM strrefs r JOIN strings s ON s.va=r.str_va '
                    'WHERE r.func=? ORDER BY r.site LIMIT 40', (va,)).fetchall()
    if st:
        print('  strings:')
        for s in st:
            print('    0x%x %s"%s"' % (s[0], 'L' if s[1] == 'w' else '', s[2][:160]))
    fs = db.execute('SELECT kind, outer, inner FROM funcsigs WHERE ref_func=?', (va,)).fetchall()
    for x in fs:
        print('  funcsig    : [%s] %s  inner=%s' % tuple(x))
    print('  decompile  : tools/ghidra_decompile.sh 0x%x    disasm: tools/fndb.sh disasm 0x%x' % (va, va))


def cmd_report(args):
    db = open_db()
    prog = load_progress()
    rows = db.execute('SELECT va, size, owner, top_ns FROM functions').fetchall()
    agg = collections.defaultdict(lambda: [0, 0, 0, 0, 0, 0, 0, 0])  # n,b,dn,db,vn,vb,sn,sb
    oagg = collections.defaultdict(lambda: [0, 0, 0, 0, 0, 0, 0, 0])
    tot = [0] * 8
    for r in rows:
        st = prog.get(r['va'], {}).get('status', 'todo')
        for a in (agg[(r['owner'], r['top_ns'])], oagg[r['owner']], tot):
            a[0] += 1
            a[1] += r['size']
            if st == 'draft':
                a[2] += 1; a[3] += r['size']
            elif st == 'verified':
                a[4] += 1; a[5] += r['size']
            elif st == 'skip':
                a[6] += 1; a[7] += r['size']
    pct = lambda x, y: (100.0 * x / y) if y else 0.0

    def line(label, a):
        return '%-28s %8d %11d | %6d %7.3f%% %7.3f%% | %6d %7.3f%% %7.3f%% | %5d' % (
            label[:28], a[0], a[1], a[2], pct(a[2], a[0]), pct(a[3], a[1]),
            a[4], pct(a[4], a[0]), pct(a[5], a[1]), a[6])
    hdr = '%-28s %8s %11s | %6s %8s %8s | %6s %8s %8s | %5s' % (
        'group', 'funcs', 'bytes', 'draft', '%count', '%bytes', 'verif', '%count', '%bytes', 'skip')
    meta = dict(db.execute('SELECT key, value FROM meta').fetchall())
    print('fndb report  (db built %s; progress rows: %d)' % (meta.get('built_at'), len(prog)))
    print(hdr)
    for o in ('remedy', 'thirdparty', 'crt', 'unknown'):
        if o in oagg:
            print(line('OWNER ' + o, oagg[o]))
    print(line('TOTAL', tot))
    cols = [c[1] for c in db.execute('PRAGMA table_info(functions)')]
    if 'owner_confidence' in cols:
        print()
        print('ownership confidence (count / bytes; hi >= 0.8, mid >= 0.5, lo < 0.5):')
        print('%-12s %20s %20s %20s %20s' % ('owner', 'hi', 'mid', 'lo', 'total'))
        bins = collections.defaultdict(lambda: [0] * 8)
        for o, c, n, by in db.execute(
                "SELECT owner, CASE WHEN owner_confidence >= 0.8 THEN 0 WHEN owner_confidence >= 0.5 THEN 1 "
                "ELSE 2 END, count(*), sum(size) FROM functions GROUP BY 1, 2"):
            bins[o][2 * c] += n
            bins[o][2 * c + 1] += by
            bins[o][6] += n
            bins[o][7] += by
        tb = sum(v[7] for v in bins.values()) or 1
        for o in ('remedy', 'thirdparty', 'crt', 'unknown'):
            v = bins.get(o)
            if v:
                print('%-12s %s' % (o, ' '.join('%8d %10s' % (v[2 * i], '%.1f%%B' % (100.0 * v[2 * i + 1] / tb))
                                               for i in range(4))))
        print()
        print('owner evidence (primary source of the final decision):')
        for o, src, n, by in db.execute('SELECT owner, owner_src, count(*), sum(size) FROM functions '
                                        'GROUP BY 1, 2 ORDER BY 4 DESC'):
            print('  %-11s %-14s %8d funcs %11d bytes (%5.1f%%)' % (o, src, n, by, 100.0 * by / tb))
    print()
    print('by namespace (top %d by bytes; owner/top_ns, %% = draft/verified share):' % args.top)
    print(hdr)
    for (o, ns), a in sorted(agg.items(), key=lambda kv: -kv[1][1])[:args.top]:
        print(line('%s/%s' % (o[0:3], ns), a))
    if 'owner_confidence' not in cols:
        src = db.execute('SELECT owner_src, count(*), sum(size) FROM functions GROUP BY 1').fetchall()
        print('\nowner attribution: ' + ', '.join('%s=%d' % (s[0], s[1]) for s in src) +
              '   (default = unnamed with no evidence -> counted as remedy)')


def cmd_next(args):
    db = open_db()
    prog = load_progress()
    where = ['size <= ?', "source NOT IN ('import_thunk','gap_unref')", "coalesce(name_src,'') NOT IN ('crt','import_thunk','thunk')"]
    params = [args.max_size]
    if not args.all_owners:
        where.append("owner IN ('remedy','unknown')")
    if args.min_conf:
        where.append("owner='remedy' AND coalesce(owner_confidence, 0) >= ?")
        params.append(args.min_conf)
    if args.ns:
        where.append('(top_ns=? OR name LIKE ?)')
        params += [args.ns, args.ns + '::%']
    if args.leaf:
        where.append('is_leaf=1')
    if args.named:
        where.append('name IS NOT NULL')
    sql = ('SELECT * FROM functions WHERE %s ORDER BY is_leaf DESC, (name IS NULL), n_callers DESC, size '
           'LIMIT ?' % ' AND '.join(where))
    out = []
    for r in db.execute(sql, params + [args.limit * 20]):
        if r['va'] in prog and prog[r['va']]['status'] != 'todo':
            continue
        out.append(r)
        if len(out) >= args.limit:
            break
    print('%-14s %-6s %-4s %-7s %-10s %s' % ('va', 'size', 'leaf', 'callers', 'ns', 'name'))
    for r in out:
        print('0x%-12x %-6d %-4s %-7d %-10s %s' % (r['va'], r['size'], 'Y' if r['is_leaf'] else '-',
                                                  r['n_callers'], r['top_ns'], r['name'] or '-'))


def cmd_owner_eval(args):
    """Rebuild in memory with overrides disabled: (1) how db/owner_overrides.tsv entries are classified
    by the automatic evidence alone, (2) hold out name evidence for a random sample of named functions
    and report agreement of the remaining evidence with the name-derived owner."""
    exe = args.exe or _db_exe() or DEFAULT_EXE
    b = Builder(exe)
    b.parse_pdata(); b.parse_dirs(); b.load_rtti(); b.scan_text(); b.find_gap_functions()
    b.name_vfuncs(); b.name_misc(); b.scan_strings_and_funcsigs(); b.finalize()
    gj = os.path.join(ROOT, 'db', 'ghidra_functions.jsonl')
    b.load_ghidra(gj if os.path.exists(gj) else None)
    run_fnsig(b, argparse.Namespace(no_fnsig=False))
    b.assign(use_overrides=False)
    ib = b.pe.image_base
    ov = ownermod.load_overrides()
    print('== db/owner_overrides.tsv classified WITHOUT overrides (%d entries)' % len(ov))
    ok = 0
    for va, (owner, lib, note) in sorted(ov.items()):
        fn = b.funcs.get(va - ib)
        if fn is None:
            print('  0x%x  not a function' % va)
            continue
        hit = fn['owner'] == owner
        ok += hit
        print('  0x%x  expected %-10s %-11s got %-10s %-12s conf=%.2f via %-12s %s' % (
            va, owner, lib, fn['owner'], fn.get('owner_lib'), fn.get('owner_conf') or 0, fn['owner_src'],
            'OK' if hit else 'MISS'))
    if ov:
        print('  -> %d/%d classified with the expected owner' % (ok, len(ov)))
    blk = int(str(args.block), 0)
    hold, rows, confm, res = ownermod.holdout_eval(b, frac=args.frac, seed=args.seed, log=log, block=blk)
    print('\n== name hold-out: %d named functions (vftable/funcsig/export/string) with name evidence hidden%s'
          % (len(hold), '' if not blk else ' (all names in %d%% of 0x%x-byte windows, '
             'third-party windows sampled separately)' % (int(args.frac * 100), blk)))
    print('  %-14s %-4s %7s %7s %8s %8s  %s' % ('decided_by', 'conf', 'agree', 'disagr', 'unknown', 'prec', 'errors'))
    tot = collections.Counter()
    for (src, tb), c in sorted(rows.items(), key=lambda kv: -sum(kv[1][k] for k in ('agree', 'disagree', 'unknown'))):
        a, d, u = c['agree'], c['disagree'], c['unknown']
        tot['agree'] += a; tot['disagree'] += d; tot['unknown'] += u
        errs = ' '.join('%s=%d' % (k, v) for k, v in c.items() if '->' in k)
        print('  %-14s %-4s %7d %7d %8d %7.1f%%  %s' % (src, tb, a, d, u, 100.0 * a / max(a + d, 1), errs))
    a, d, u = tot['agree'], tot['disagree'], tot['unknown']
    print('  %-19s %7d %7d %8d %7.1f%%' % ('TOTAL', a, d, u, 100.0 * a / max(a + d, 1)))
    print('\n  by true owner (rows) x predicted owner/confidence bin (columns):')
    cols = [(o, tb) for o in ('remedy', 'thirdparty', 'crt', 'unknown') for tb in ('hi', 'mid', 'lo')]
    cols = [c for c in cols if any(confm[t][c] for t in confm)]
    print('  %-11s' % 'truth' + ''.join('%11s' % ('%s/%s' % (o[:5], tb)) for o, tb in cols) + '   recall')
    for t in sorted(confm):
        n = sum(confm[t].values())
        ok = sum(v for (o, tb), v in confm[t].items() if o == t)
        print('  %-11s' % t + ''.join('%11d' % confm[t][c] for c in cols) + '   %5.1f%%' % (100.0 * ok / max(n, 1)))
    print('  (truth = owner implied by the hidden name; biased towards virtual functions/vftables)')


def cmd_set(args):
    if args.status not in STATUSES:
        raise SystemExit('status must be one of %s' % (STATUSES,))
    va = int(args.va, 16) if args.va.lower().startswith('0x') else int(args.va, 16)
    if va < 0x140000000:
        va += 0x140000000
    rows = load_progress()
    db = open_db(required=False)
    name = ''
    if db:
        r = db.execute('SELECT name FROM functions WHERE va=?', (va,)).fetchone()
        if r is None:
            print('warning: 0x%x is not a known function start in fndb' % va, file=sys.stderr)
        name = (r['name'] if r else '') or ''
    cur = rows.get(va, {'name': name, 'status': 'todo', 'rust_path': '', 'notes': ''})
    cur['status'] = args.status
    if args.rust is not None:
        cur['rust_path'] = args.rust
    if args.notes is not None:
        cur['notes'] = args.notes
    if not cur['name']:
        cur['name'] = name
    rows[va] = cur
    save_progress(rows)
    print('0x%x -> %s' % (va, args.status))


REG_ENTRY_RE = re.compile(r'Replacement\s*\{(?P<body>.*?)\}', re.S)
REG_CTOR_RE = re.compile(
    r'Replacement::(?P<kind>draft(?:_diff|_throws|_throws_diff)?|verified(?:_throws)?)\s*\(\s*'
    r'(?P<va>0x[0-9a-fA-F_]+)\s*,\s*"(?P<name>(?:\\.|[^"\\])*)"\s*,\s*(?P<ptr>[\w:<>]+)'
)
ANNOT_RE = re.compile(r'^\s*(?://[/!]?|\*)\s*@\s*(0x[0-9a-fA-F]+)\b(?P<rest>.*)$')
FN_RE = re.compile(r'\bfn\s+([A-Za-z_]\w*)')


def cmd_sync(args):
    found = {}  # va -> (status, rust_path, name)
    src_root = os.path.abspath(args.src or GAME_SRC)
    reg = os.path.abspath(os.path.join(src_root, 'registry.rs'))
    reg_dir = os.path.abspath(os.path.join(src_root, 'registry'))
    shards_dir = os.path.abspath(os.path.join(src_root, 'shards'))
    for dirpath, _, files in os.walk(src_root):
        for fn in sorted(files):
            if not fn.endswith('.rs'):
                continue
            p = os.path.join(dirpath, fn)
            rel = os.path.relpath(p, ROOT)
            with open(p) as f:
                src = f.read()
            abs_p = os.path.abspath(p)
            abs_dir = os.path.abspath(dirpath)
            in_reg = abs_p == reg or abs_dir == reg_dir or abs_dir.startswith(reg_dir + os.sep)
            in_shards = abs_dir == shards_dir or abs_dir.startswith(shards_dir + os.sep)
            if in_reg or in_shards:
                code = re.sub(r'//[^\n]*', '', src)
                for m in REG_ENTRY_RE.finditer(code):
                    body = m.group('body')
                    vm = re.search(r'\bva\s*:\s*(0x[0-9a-fA-F_]+)', body)
                    if not vm:
                        continue
                    va = int(vm.group(1).replace('_', ''), 16)
                    nm = re.search(r'\bname\s*:\s*"([^"]*)"', body)
                    stm = re.search(r'Status::(\w+)', body)
                    pm = re.search(r'\bptr\s*:\s*([\w:]+)', body)
                    status = 'verified' if stm and stm.group(1) == 'Verified' else 'draft'
                    found[va] = {'status': status, 'name': nm.group(1) if nm else '',
                                 'rust_path': (pm.group(1) if pm else rel), 'from': 'registry'}
                for m in REG_CTOR_RE.finditer(code):
                    va = int(m.group('va').replace('_', ''), 16)
                    status = 'verified' if m.group('kind').startswith('verified') else 'draft'
                    found[va] = {'status': status, 'name': m.group('name'),
                                 'rust_path': m.group('ptr'), 'from': 'registry'}
                continue
            lines = src.split('\n')
            for i, line in enumerate(lines):
                m = ANNOT_RE.match(line)
                if not m:
                    continue
                va = int(m.group(1), 16)
                rest = m.group('rest').lower()
                fname = ''
                for j in range(i + 1, min(i + 40, len(lines))):
                    fm = FN_RE.search(lines[j])
                    if fm:
                        fname = fm.group(1)
                        break
                status = 'verified' if 'verified' in rest else 'draft'
                rp = '%s:%d%s' % (rel, i + 1, ('::' + fname) if fname else '')
                if va in found and found[va]['from'] == 'registry':
                    found[va]['rust_path'] = rp
                    if status == 'verified':
                        found[va]['status'] = 'verified'
                else:
                    found[va] = {'status': status, 'name': '', 'rust_path': rp, 'from': 'annotation'}
    rows = load_progress()
    db = open_db(required=False)
    changed = 0
    for va, info in found.items():
        cur = rows.get(va, {'name': '', 'status': 'todo', 'rust_path': '', 'notes': ''})
        if cur['status'] == 'skip':
            continue
        newst = info['status']
        if cur['status'] == 'verified' and newst == 'draft' and info['from'] == 'annotation':
            newst = 'verified'  # manual 'set verified' wins over a plain annotation
        if (cur['status'], cur['rust_path']) != (newst, info['rust_path']):
            changed += 1
        cur['status'], cur['rust_path'] = newst, info['rust_path']
        name = info['name']
        if not name and db:
            r = db.execute('SELECT name FROM functions WHERE va=?', (va,)).fetchone()
            name = (r['name'] if r else '') or ''
            if r is None:
                print('warning: 0x%x (%s) is not a known function start' % (va, info['rust_path']), file=sys.stderr)
        cur['name'] = name or cur['name']
        rows[va] = cur
    dropped = []
    for va, cur in list(rows.items()):
        if cur['status'] in ('draft', 'verified') and cur['rust_path'] and va not in found and not args.keep_missing:
            dropped.append(va)
            cur['status'] = 'todo'
    if args.dry_run:
        for va in sorted(found):
            print('  0x%x %-8s %s' % (va, rows[va]['status'], rows[va]['rust_path']))
    else:
        save_progress(rows)
        reg_vas = [va for va, info in found.items() if info['from'] == 'registry']
        import canthrow
        if db and canthrow.built(db) and os.path.exists(_db_exe() or ''):
            n, ch = canthrow.write_throws_tsv(db, PE(_db_exe()), reg_vas, THROWS_TSV)
            print('sync: %s %s (%d registry entries)' % (os.path.relpath(THROWS_TSV, ROOT),
                                                           'updated' if ch else 'unchanged', n))
        else:
            print('warning: can-throw data unavailable (run tools/fndb.sh build); %s not updated'
                  % os.path.relpath(THROWS_TSV, ROOT), file=sys.stderr)
    print('sync: %d functions found in %s (%d changed); %d dropped (no longer in code)%s'
          % (len(found), os.path.relpath(src_root, ROOT), changed, len(dropped),
             (': ' + ', '.join('0x%x' % v for v in dropped)) if dropped else ''))


def cmd_disasm(args):
    db = open_db()
    rows = resolve(db, args.key)
    if len(rows) != 1:
        for r in rows:
            print(fmt_fn(r))
        raise SystemExit('need exactly one function (got %d)' % len(rows))
    r = rows[0]
    exe = dict(db.execute('SELECT key, value FROM meta').fetchall())['exe']
    ranges = [(r['va'], r['end'])] + [(f['va'], f['end']) for f in
                                      db.execute('SELECT * FROM fragments WHERE func_va=?', (r['va'],))]
    names = {}
    strs = {}
    print('; %s  0x%x size=%d' % (r['name'] or 'sub_%x' % r['va'], r['va'], r['size']))
    for a, b in ranges:
        out = subprocess.run(['objdump', '-d', '--no-show-raw-insn', '--x86-asm-syntax=intel',
                              '--start-address=0x%x' % a, '--stop-address=0x%x' % b, exe],
                             capture_output=True, text=True).stdout
        lines = out.split('\n')
        for line in lines:
            if not re.match(r'^\s*[0-9a-f]+:', line):
                continue

            def sub(m):
                tva = int(m.group(1), 16)
                if tva not in names:
                    x = db.execute('SELECT name FROM functions WHERE va=?', (tva,)).fetchone()
                    if x is None:
                        x = db.execute('SELECT text FROM strings WHERE va=?', (tva,)).fetchone()
                        names[tva] = ('"%s"' % x[0][:80]) if x else None
                    else:
                        names[tva] = x[0] or 'sub_%x' % tva
                n = names[tva]
                return '0x%x <%s>' % (tva, n) if n else '0x%x' % tva
            line = re.sub(r'0x([0-9a-f]{9})\s*<[^>]*>', sub, line)
            print(line)
        if len(ranges) > 1:
            print('; ---- fragment')


def cmd_resolve(args):
    db = open_db()
    rows = resolve(db, args.key, 50)
    if len(rows) != 1:
        exact = [r for r in rows if r['name'] == args.key]
        if len(exact) == 1:
            rows = exact
        else:
            for r in rows[:20]:
                print('  ' + fmt_fn(r), file=sys.stderr)
            raise SystemExit('%s: %d matches' % (args.key, len(rows)))
    print('0x%x' % rows[0]['va'])


def shorten_for_ghidra(name, limit=1900):
    while len(name) > limit:
        n2 = re.sub(r'<[^<>]*>', '<_>', name, count=0)
        if n2 == name:
            return name[:limit]
        name = n2.replace('<_>', '<~>') if '<_>' in n2 and n2.count('<') == n2.count('<_>') else n2
        if len(name) <= limit:
            break
    return name


def cmd_funcsig(args):
    db = open_db()
    for r in db.execute('SELECT f.*, fn.name AS fname FROM funcsigs f LEFT JOIN functions fn ON fn.va=f.ref_func '
                        'WHERE f.outer LIKE ? OR f.inner LIKE ? LIMIT ?',
                        ('%' + args.substr + '%', '%' + args.substr + '%', args.limit)):
        print('[%s] str=0x%x ref=%s  %s\n      inner=%s' % (
            r['kind'], r['str_va'], ('0x%x %s' % (r['ref_func'], r['fname'] or '')) if r['ref_func'] else '-',
            r['outer'][:200], (r['inner'] or '')[:200]))


def cmd_strings(args):
    db = open_db()
    for r in db.execute('SELECT s.va, s.text, r.func, f.name FROM strings s JOIN strrefs r ON r.str_va=s.va '
                        'LEFT JOIN functions f ON f.va=r.func WHERE s.text LIKE ? LIMIT ?',
                        ('%' + args.substr + '%', args.limit)):
        print('0x%x %-60s  in %s %s' % (r[0], '"%s"' % r[1][:58], hex(r[2]) if r[2] else '-', r[3] or ''))


def cmd_sql(args):
    db = open_db()
    cur = db.execute(args.query)
    cols = [c[0] for c in cur.description] if cur.description else []
    print('\t'.join(cols))
    for r in cur:
        print('\t'.join(hex(v) if isinstance(v, int) and v >= 0x140000000 else str(v) for v in r))


def cmd_ghidra_names(args):
    db = open_db()
    n = 0
    with open(args.out, 'w') as f:
        f.write('# va\tnamespace\tname\tkind\tcomment  (generated by tools/fndb.sh ghidra-names)\n')
        for r in db.execute("SELECT va, name, name_src FROM functions WHERE name IS NOT NULL "
                            "AND name_src NOT IN ('ghidra')"):
            parts = split_qualified(shorten_for_ghidra(r['name']))
            ns = '::'.join(parts[:-1])
            simple = parts[-1]
            alts = db.execute('SELECT name, src, detail FROM names WHERE va=? ORDER BY prio', (r['va'],)).fetchall()
            com = ['fndb: %s' % r['name_src']]
            for a in alts[1:8]:
                com.append('alt[%s]: %s' % (a[1], a[0]))
            if alts and alts[0][2]:
                com.append(alts[0][2])
            kind = 'export' if r['name_src'] == 'export' else 'function'
            line = '0x%x\t%s\t%s\t%s\t%s\n' % (r['va'], ns, simple, kind,
                                              '\\n'.join(com).replace('\t', ' '))
            f.write(line)
            n += 1
    print('wrote %d names to %s' % (n, args.out))


def cmd_can_throw(args):
    """Can the function reach a throw? (see tools/fndb/canthrow.py; ADVICE_SEH.md)."""
    import canthrow
    db = open_db()
    if args.build or not canthrow.built(db):
        canthrow.build(db, os.path.join(ROOT, 'out', 'fndb'), args.exe, log=log)
    if not args.key:
        return
    rows = resolve(db, args.key)
    if len(rows) != 1:
        for r in rows:
            print(fmt_fn(r))
        raise SystemExit('need exactly one function (got %d)' % len(rows))
    va = rows[0]['va']
    md, ma = canthrow.lookup(db, va)
    if args.mask:
        print('%d %d' % (md, ma))
        return
    print('0x%x %s' % (va, rows[0]['name'] or ''))
    print('  direct calls : %s' % canthrow.mask_names(md))
    print('  with vcalls  : %s' % canthrow.mask_names(ma))
    for wva, how in canthrow.witness(db, va):
        n = db.execute('SELECT name FROM functions WHERE va=?', (wva,)).fetchone()
        print('    0x%x %-28s %s' % (wva, (n[0] if n and n[0] else '')[:28], how))


def cmd_ingest_ghidra(args):
    """Merging happens inside `build` (so names feed neighbour inference); just rebuild."""
    if not os.path.exists(args.path):
        raise SystemExit('missing %s (run tools/ghidra_analyze.sh --export)' % args.path)
    cmd_build(argparse.Namespace(exe=_db_exe() or DEFAULT_EXE, db=DB_PATH, no_ghidra=False, ghidra=args.path,
                                 no_fnsig=False, no_overrides=False))


def _db_exe():
    db = open_db(required=False)
    if db is None:
        return None
    r = db.execute("SELECT value FROM meta WHERE key='exe'").fetchone()
    return r[0] if r else None


def main():
    if len(sys.argv) > 1 and sys.argv[1] == 'libcorpus':
        import libcorpus
        return libcorpus.main(sys.argv[2:])
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sp = ap.add_subparsers(dest='cmd', required=True)
    p = sp.add_parser('build'); p.add_argument('--exe', default=DEFAULT_EXE); p.add_argument('--db', default=DB_PATH)
    p.add_argument('--no-ghidra', action='store_true', help='do not merge db/ghidra_functions.jsonl')
    p.add_argument('--no-fnsig', action='store_true', help='skip instruction features (tools/fndb/fnsig)')
    p.add_argument('--no-overrides', action='store_true', help='ignore db/owner_overrides.tsv (evaluation)')
    p.set_defaults(fn=cmd_build)
    p = sp.add_parser('report'); p.add_argument('--top', type=int, default=40); p.set_defaults(fn=cmd_report)
    p = sp.add_parser('show'); p.add_argument('key'); p.add_argument('--limit', type=int, default=25)
    p.set_defaults(fn=cmd_show)
    p = sp.add_parser('next'); p.add_argument('--ns'); p.add_argument('--max-size', type=int, default=128)
    p.add_argument('--limit', type=int, default=25); p.add_argument('--all-owners', action='store_true')
    p.add_argument('--leaf', action='store_true', help='only (approximate) leaf functions')
    p.add_argument('--named', action='store_true', help='only functions with a name')
    p.add_argument('--min-conf', type=float, default=0.0,
                   help='only functions classified remedy with at least this ownership confidence')
    p.set_defaults(fn=cmd_next)
    p = sp.add_parser('owner-eval', help='precision of ownership evidence (positives + name hold-out)')
    p.add_argument('--exe', default=None); p.add_argument('--frac', type=float, default=0.1)
    p.add_argument('--seed', type=int, default=1)
    p.add_argument('--block', default='0x10000', help='hold out all names in random windows of this size '
                   '(default 0x10000); 0 = hide individual functions'); p.set_defaults(fn=cmd_owner_eval)
    p = sp.add_parser('set'); p.add_argument('va'); p.add_argument('status')
    p.add_argument('--rust'); p.add_argument('--notes'); p.set_defaults(fn=cmd_set)
    p = sp.add_parser('sync'); p.add_argument('--keep-missing', action='store_true')
    p.add_argument('--src', help='source root to scan (default crates/aw2-game/src)')
    p.add_argument('--dry-run', action='store_true', help='print what would change, do not write')
    p.set_defaults(fn=cmd_sync)
    p = sp.add_parser('disasm'); p.add_argument('key'); p.set_defaults(fn=cmd_disasm)
    p = sp.add_parser('resolve', help='print the VA of a unique function match')
    p.add_argument('key'); p.set_defaults(fn=cmd_resolve)
    p = sp.add_parser('funcsig'); p.add_argument('substr'); p.add_argument('--limit', type=int, default=50)
    p.set_defaults(fn=cmd_funcsig)
    p = sp.add_parser('strings'); p.add_argument('substr'); p.add_argument('--limit', type=int, default=50)
    p.set_defaults(fn=cmd_strings)
    p = sp.add_parser('sql'); p.add_argument('query'); p.set_defaults(fn=cmd_sql)
    p = sp.add_parser('ghidra-names'); p.add_argument('--out', default=os.path.join(ROOT, 'db', 'ghidra_names.tsv'))
    p.set_defaults(fn=cmd_ghidra_names)
    p = sp.add_parser('can-throw', help='can a function reach a C++/SEH throw (cached reachability)')
    p.add_argument('key', nargs='?'); p.add_argument('--build', action='store_true', help='recompute the cache')
    p.add_argument('--mask', action='store_true', help='print "<mask_direct> <mask_all>" only')
    p.add_argument('--exe', default=DEFAULT_EXE); p.set_defaults(fn=cmd_can_throw)
    p = sp.add_parser('ingest-ghidra'); p.add_argument('path', nargs='?',
                                                       default=os.path.join(ROOT, 'db', 'ghidra_functions.jsonl'))
    p.set_defaults(fn=cmd_ingest_ghidra)
    args = ap.parse_args()
    try:
        args.fn(args)
    except BrokenPipeError:
        pass


if __name__ == '__main__':
    main()
