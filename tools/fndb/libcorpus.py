#!/usr/bin/env python3
"""libcorpus - reference corpus of statically linked open-source libraries for fndb ownership.

    tools/fndb.sh libcorpus fetch      clone pinned library sources into out/fndb/libsrc/
    tools/fndb.sh libcorpus strings    string literals + hex constants -> out/fndb/libcorpus/
    tools/fndb.sh libcorpus refbuild   clang-cl reference objects -> out/fndb/ref/<lib>.tsv (fnsig)
    tools/fndb.sh libcorpus all        all of the above

Everything generated lives under out/ (gitignored). `fndb build` picks the results up when present
and degrades gracefully when they are missing (see docs/FNDB.md).

Reference objects are built with clang-cl (~/.swiftly/bin) against microsoft/STL headers + a small
vcruntime shim (tools/fndb/refsrc/shim) + mingw-w64 C headers shipped with zig (patched copy under
out/). clang-cl is *not* MSVC: instruction streams differ, so the objects are mostly mined for
compiler-independent features (constants, constant-pool values, strings, CFG shape, callees).
"""
import concurrent.futures
import glob
import os
import re
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, '..', '..'))
OUT = os.path.join(ROOT, 'out', 'fndb')
LIBSRC = os.path.join(OUT, 'libsrc')
CORPUS = os.path.join(OUT, 'libcorpus')
REFDIR = os.path.join(OUT, 'ref')
REFOBJ = os.path.join(OUT, 'refobj')
FNSIG_DIR = os.path.join(HERE, 'fnsig')
FNSIG_TARGET = os.path.join(OUT, 'fnsig-target')
FNSIG_BIN = os.path.join(FNSIG_TARGET, 'release', 'fnsig')
CLANG_CL = os.environ.get('AW2_CLANG_CL', os.path.expanduser('~/.swiftly/bin/clang-cl'))

# (lib label, git url, ref, source subdirs for the string/constant corpus)
# Versions: MSVC toolset from the exe's Rich header (14.39.33519 = VS 2022 17.9), OTel from the
# "OTel-OTLP-Exporter-Cpp/1.9.0" user agent string, others from docs/THIRD_PARTY.md / AGENTS.md.
LIBS = [
    ('std', 'https://github.com/microsoft/STL.git', 'vs-2022-17.9', ['stl/inc', 'stl/src']),
    ('rapidjson', 'https://github.com/Tencent/rapidjson.git', None, ['include']),
    ('sparsehash', 'https://github.com/sparsehash/sparsehash.git', None, ['src/sparsehash']),
    ('opentelemetry', 'https://github.com/open-telemetry/opentelemetry-cpp.git', 'v1.9.0',
     ['api', 'sdk', 'exporters', 'ext']),
    ('nlohmann', 'https://github.com/nlohmann/json.git', 'v3.11.2', ['include']),
    ('zlib', 'https://github.com/madler/zlib.git', 'v1.2.8', ['.']),
    ('protobuf', 'https://github.com/protocolbuffers/protobuf.git', 'v3.15.8', ['src/google/protobuf']),
    ('curl', 'https://github.com/curl/curl.git', 'curl-8_0_1', ['lib', 'include']),
    ('lz4', 'https://github.com/lz4/lz4.git', 'v1.9.4', ['lib']),
]
# optional upstream reference checkouts under third_party/ (fetched via ./scripts/third_party.sh)
LOCAL_LIBS = [
    ('Luau', 'third_party/luau', ['VM', 'Ast', 'Compiler', 'Common', 'Analysis', 'CodeGen', 'Config']),
    ('physx', 'third_party/physx-3.4', ['PhysX_3.4/Source', 'PxShared/src']),
]
SRC_EXT = ('.h', '.hpp', '.hh', '.inl', '.c', '.cc', '.cpp', '.cxx', '.ipp', '.tcc', '')


def log(m):
    sys.stderr.write('[libcorpus] %s\n' % m)


def lib_dir(lib):
    for name, path, _ in LOCAL_LIBS:
        if name == lib:
            return os.path.join(ROOT, path)
    return os.path.join(LIBSRC, lib)


# ------------------------------------------------------------------------------------------------
def cmd_fetch(_args=None):
    os.makedirs(LIBSRC, exist_ok=True)
    procs = []
    for lib, url, ref, _ in LIBS:
        d = os.path.join(LIBSRC, lib)
        if os.path.isdir(os.path.join(d, '.git')):
            continue
        cmd = ['git', 'clone', '-q', '--depth', '1'] + (['--branch', ref] if ref else []) + [url, d]
        log(' '.join(cmd))
        procs.append(subprocess.Popen(cmd, stderr=subprocess.DEVNULL))
    for p in procs:
        p.wait()
    for lib, url, ref, _ in LIBS:
        d = os.path.join(LIBSRC, lib)
        if os.path.isdir(d):
            rev = subprocess.run(['git', '-C', d, 'rev-parse', '--short', 'HEAD'], capture_output=True,
                                 text=True).stdout.strip()
            log('%-14s %-12s %s' % (lib, ref or 'HEAD', rev))
        else:
            log('%-14s MISSING' % lib)


# ------------------------------------------------------------------------------------------------
LIT_RE = re.compile(r'(?:\bu8|\bL|\bu|\bU)?"((?:[^"\\\n]|\\.)*)"')
HEX_RE = re.compile(r'\b0[xX]([0-9a-fA-F]{5,16})(?:[uUlL]{0,3})\b')
ESC_RE = re.compile(r'\\(x[0-9a-fA-F]{1,2}|[0-7]{1,3}|.)', re.S)
ESC = {'n': '\n', 't': '\t', 'r': '\r', '\\': '\\', '"': '"', "'": "'", 'a': '\a', 'b': '\b',
       'f': '\f', 'v': '\v', '?': '?', '0': '\0'}


def unescape(s):
    def rep(m):
        e = m.group(1)
        if e[0] == 'x' and len(e) > 1:
            return chr(int(e[1:], 16))
        if e[0] in '01234567':
            return chr(int(e, 8) & 0xff)
        return ESC.get(e, e)
    return ESC_RE.sub(rep, s)


def tsv_esc(s):
    return s.replace('\\', '\\\\').replace('\t', '\\t').replace('\n', '\\n').replace('\r', '\\r')


def scan_source(path):
    try:
        with open(path, encoding='latin1') as f:
            txt = f.read()
    except OSError:
        return set(), set()
    strs = set()
    # concatenate adjacent literals ("abc" "def") separated only by whitespace
    run, last_end = [], -1
    for m in LIT_RE.finditer(txt):
        if run and txt[last_end:m.start()].strip() == '':
            run.append(unescape(m.group(1)))
        else:
            if len(run) > 1:
                strs.add(''.join(run))
            run = [unescape(m.group(1))]
        strs.add(unescape(m.group(1)))
        last_end = m.end()
    if len(run) > 1:
        strs.add(''.join(run))
    consts = set()
    for m in HEX_RE.finditer(txt):
        v = int(m.group(1), 16)
        if v >= 0x10000:
            consts.add(v)
    return strs, consts


def cmd_strings(_args=None):
    os.makedirs(CORPUS, exist_ok=True)
    jobs = [(lib, subs) for lib, _, _, subs in LIBS] + [(lib, subs) for lib, _, subs in LOCAL_LIBS]
    out_s = open(os.path.join(CORPUS, 'strings.tsv'), 'w')
    out_c = open(os.path.join(CORPUS, 'consts.tsv'), 'w')
    for lib, subs in jobs:
        base = lib_dir(lib)
        if not os.path.isdir(base):
            log('skip %s (not fetched)' % lib)
            continue
        files = []
        for sd in subs:
            d = os.path.join(base, sd)
            for dp, dn, fn in os.walk(d):
                dn[:] = [x for x in dn if x not in ('.git', 'test', 'tests', 'testing', 'docs', 'doc',
                                                     'examples', 'example', 'benchmark', 'benchmarks')]
                if sd == '.' and dp != d:
                    continue
                for f in fn:
                    ext = os.path.splitext(f)[1].lower()
                    if ext in SRC_EXT and (ext or lib == 'std'):
                        if 'unittest' in f or f.endswith('_test.cc'):
                            continue
                        files.append(os.path.join(dp, f))
        S, C = set(), set()
        with concurrent.futures.ThreadPoolExecutor(8) as ex:
            for s, c in ex.map(scan_source, files):
                S |= s
                C |= c
        for s in sorted(S):
            if len(s) >= 4:
                out_s.write('%s\t%s\n' % (lib, tsv_esc(s)))
        for c in sorted(C):
            out_c.write('%s\tI:%x\n' % (lib, c))
        log('%-14s %5d files  %7d strings  %6d hex constants' % (lib, len(files), len(S), len(C)))
    out_s.close()
    out_c.close()


# ------------------------------------------------------------------------------------------------
def ensure_fnsig():
    srcs = [os.path.join(FNSIG_DIR, 'Cargo.toml')] + glob.glob(os.path.join(FNSIG_DIR, 'src', '*.rs'))
    if os.path.exists(FNSIG_BIN) and all(os.path.getmtime(s) <= os.path.getmtime(FNSIG_BIN) for s in srcs):
        return FNSIG_BIN
    log('building fnsig (cargo, release)')
    env = dict(os.environ, CARGO_TARGET_DIR=FNSIG_TARGET)
    subprocess.run(['cargo', 'build', '--release', '--quiet'], cwd=FNSIG_DIR, env=env, check=True)
    return FNSIG_BIN


def mingw_headers():
    """Patched copy of zig's mingw-w64 headers usable from clang-cl (MSVC target)."""
    dst = os.path.join(OUT, 'refbuild', 'mingw')
    if os.path.exists(os.path.join(dst, '.patched')):
        return dst
    zig = shutil.which('zig')
    if not zig:
        raise SystemExit('refbuild needs zig (for mingw-w64 C headers): brew install zig')
    src = os.path.join(os.path.dirname(os.path.realpath(zig)), '..', 'lib', 'zig', 'libc', 'include',
                       'any-windows-any')
    if not os.path.isdir(src):
        raise SystemExit('mingw headers not found at %s' % src)
    if os.path.exists(dst):
        shutil.rmtree(dst)
    shutil.copytree(src, dst)
    for dp, _, fn in os.walk(dst):  # zig installs its headers read-only
        for f in fn:
            os.chmod(os.path.join(dp, f), 0o644)
    m = os.path.join(dst, '_mingw.h')
    s = open(m).read()
    # attribute placement after the declarator is GNU-only; clang-cl accepts __attribute__
    s = s.replace('# define __attribute__(x) /* nothing */', '/* fndb: keep __attribute__ (clang-cl) */')
    s = s.replace('#define __MINGW_NOTHROW __declspec(nothrow)', '#define __MINGW_NOTHROW')
    s = s.replace('#define __MINGW_ATTRIB_NORETURN __declspec(noreturn)',
                  '#define __MINGW_ATTRIB_NORETURN __attribute__ ((__noreturn__))')
    s = s.replace('#define __MINGW_ATTRIB_DEPRECATED __declspec(deprecated)',
                  '#define __MINGW_ATTRIB_DEPRECATED __attribute__ ((__deprecated__))')
    s = s.replace('#define __MINGW_ATTRIB_MALLOC __declspec(noalias) __declspec(restrict)',
                  '#define __MINGW_ATTRIB_MALLOC __attribute__ ((__malloc__))')
    open(m, 'w').write(s)
    w = os.path.join(dst, 'wchar.h')
    s = open(w).read()
    s = s.replace("""  typedef struct _Mbstatet {
    unsigned long _Wchar;
    unsigned short _Byte, _State;
  } _Mbstatet;""", """#ifndef _MBSTATET_DEFINED_SHIM
#define _MBSTATET_DEFINED_SHIM
  typedef struct _Mbstatet {
    unsigned long _Wchar;
    unsigned short _Byte, _State;
  } _Mbstatet;
#endif""")
    open(w, 'w').write(s)
    sj = os.path.join(dst, 'setjmp.h')
    s = open(sj).read()
    # clang's intrin.h already declares __intrinsic_setjmpex with a different prototype
    s = s.replace('#    define _setjmp __intrinsic_setjmpex', '/* fndb: no _setjmp -> __intrinsic_setjmpex */')
    open(sj, 'w').write(s)
    open(os.path.join(dst, '.patched'), 'w').write('ok\n')
    return dst


def ref_jobs():
    """(lib, source, extra flags, variant) for every reference compile."""
    R = os.path.join(HERE, 'refsrc')
    jobs = []
    for opt in ('O2', 'Ob1'):
        jobs.append(('std', os.path.join(R, 'stl_ref.cpp'), [], opt))
        jobs.append(('rapidjson', os.path.join(R, 'rapidjson_ref.cpp'),
                     ['-I', os.path.join(LIBSRC, 'rapidjson', 'include')], opt))
        jobs.append(('sparsehash', os.path.join(R, 'sparsehash_ref.cpp'),
                     ['-I', os.path.join(HERE, 'refsrc', 'sparsehash_cfg'),
                      '-I', os.path.join(LIBSRC, 'sparsehash', 'src')], opt))
        jobs.append(('nlohmann', os.path.join(R, 'nlohmann_ref.cpp'),
                     ['-I', os.path.join(LIBSRC, 'nlohmann', 'include')], opt))
    z = os.path.join(LIBSRC, 'zlib')
    for c in sorted(glob.glob(os.path.join(z, '*.c'))):
        if os.path.basename(c) in ('example.c', 'minigzip.c'):
            continue
        jobs.append(('zlib', c, ['-I', z, '/TC'], 'O2'))
    l4 = os.path.join(LIBSRC, 'lz4', 'lib')
    for c in sorted(glob.glob(os.path.join(l4, '*.c'))):
        jobs.append(('lz4', c, ['-I', l4, '/TC'], 'O2'))
    lu = os.path.join(ROOT, 'third_party', 'luau')
    inc = []
    for d in ('VM/include', 'VM/src', 'Ast/include', 'Compiler/include', 'Common/include', 'Bytecode/include'):
        inc += ['-I', os.path.join(lu, d)]
    for sub in ('VM/src', 'Ast/src', 'Compiler/src'):
        for c in sorted(glob.glob(os.path.join(lu, sub, '*.cpp'))):
            jobs.append(('Luau', c, inc, 'O2'))
    return jobs


def compile_one(job, mingw):
    lib, src, extra, variant = job
    od = os.path.join(REFOBJ, lib, variant)
    os.makedirs(od, exist_ok=True)
    obj = os.path.join(od, os.path.splitext(os.path.basename(src))[0] + '.obj')
    if os.path.exists(obj) and os.path.getmtime(obj) >= os.path.getmtime(src):
        return obj, None
    is_c = '/TC' in extra
    cmd = [CLANG_CL, '--target=x86_64-pc-windows-msvc', '-fms-compatibility-version=19.39',
           '/O2', '/MD', '/EHsc', '/GS-', '/Gy', '/c', '/X', '-D_UCRT', '-D_CRT_SECURE_NO_WARNINGS',
           '-DNDEBUG', '-Wno-everything']
    if variant == 'Ob1':
        cmd.append('/Ob1')
    if not is_c:
        cmd.append('/std:c++17')
    cmd += ['-imsvc', os.path.join(LIBSRC, 'std', 'stl', 'inc'), '-imsvc', os.path.join(HERE, 'refsrc', 'shim'),
            '-imsvc', mingw]
    cmd += extra + ['/Fo' + obj, '--', src]  # '--': clang-cl would parse /Users/... as /U
    p = subprocess.run(cmd, capture_output=True, text=True)
    if p.returncode != 0:
        err = [l for l in (p.stdout + p.stderr).splitlines() if 'error' in l][:3]
        return None, '%s: %s' % (os.path.relpath(src, ROOT), ' | '.join(err))
    return obj, None


def cmd_refbuild(_args=None):
    if not os.path.exists(CLANG_CL):
        raise SystemExit('clang-cl not found at %s (set AW2_CLANG_CL)' % CLANG_CL)
    if not os.path.isdir(os.path.join(LIBSRC, 'std')):
        raise SystemExit('run: tools/fndb.sh libcorpus fetch')
    fnsig = ensure_fnsig()
    mingw = mingw_headers()
    jobs = ref_jobs()
    objs, errs = {}, []
    with concurrent.futures.ThreadPoolExecutor(os.cpu_count() or 8) as ex:
        for job, (obj, err) in zip(jobs, ex.map(lambda j: compile_one(j, mingw), jobs)):
            if obj:
                objs.setdefault(job[0], []).append(obj)
            else:
                errs.append(err)
    for e in errs:
        log('compile failed: ' + e)
    os.makedirs(REFDIR, exist_ok=True)
    for lib, ol in sorted(objs.items()):
        outp = os.path.join(REFDIR, lib + '.tsv')
        subprocess.run([fnsig, 'coff', lib, outp] + sorted(ol), check=True)
    log('reference objects: %s; %d compile failures' %
        (', '.join('%s=%d' % (k, len(v)) for k, v in sorted(objs.items())), len(errs)))


def main(argv):
    cmd = argv[0] if argv else 'all'
    if cmd == 'fetch':
        cmd_fetch()
    elif cmd == 'strings':
        cmd_strings()
    elif cmd == 'refbuild':
        cmd_refbuild()
    elif cmd == 'all':
        cmd_fetch()
        cmd_strings()
        cmd_refbuild()
    else:
        raise SystemExit(__doc__)


if __name__ == '__main__':
    main(sys.argv[1:])
