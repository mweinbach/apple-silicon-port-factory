"""lldb helpers for the AW2 hybrid loader (Rosetta 2).

Loaded by tools/lldb.sh (`command script import tools/debug/lldb_aw2.py`).

Commands
    aw2sym <addr-expr>...      fndb name of exe addresses (`aw2sym $pc 0x14207b233`)
    aw2b <va|rva|name>...      breakpoint by exe VA / RVA / fndb name; queued until
                               the exe is mapped, then rebased on its real base
    aw2bt [N]                  backtrace of the current thread with fndb names, plus
                               a scan of the stack for exe return addresses
    aw2report [all]            in-process mixed guest/host backtrace of the current
                               (or every) thread, printed to the loader log
    aw2info                    exe base/size, symbol file, queued breakpoints
    aw2attach                  after `process attach`: read AW2_EXE_BASE/SIZE and
                               turn queued aw2b requests into breakpoints
    aw2go                      release a loader waiting with AW2_WAIT_DEBUGGER

tools/lldb.sh starts the loader with AW2_WAIT_DEBUGGER: it maps the exe,
installs hooks and waits in aw2_debug_image_mapped() before any Windows code
runs; lldb attaches by pid, runs aw2attach + aw2b + aw2go. Never launch the
loader under lldb (wedged Rosetta's oahd once, see tools/README.md).

Symbols come from out/symbols/AlanWake2.exe.syms (tools/debug/export_symbols.py),
override with AW2_SYMBOLS=<file>.
"""
import bisect
import os
import shlex

import lldb

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
PREF_BASE = 0x140000000


class Syms:
    def __init__(self, path):
        self.path = path
        self.rvas, self.sizes, self.owners, self.status, self.names = [], [], [], [], []
        self.by_name = {}
        self.base = PREF_BASE
        if not os.path.exists(path):
            return
        with open(path) as f:
            header = f.readline()
            for w in header.split():
                if w.startswith('base='):
                    self.base = int(w[5:], 16)
            for line in f:
                p = line.rstrip('\n').split('\t', 4)
                if len(p) < 5:
                    continue
                rva = int(p[0], 16)
                self.rvas.append(rva)
                self.sizes.append(int(p[1], 16))
                self.owners.append(p[2])
                self.status.append(p[3])
                self.names.append(p[4])
                if p[4]:
                    self.by_name.setdefault(p[4], rva)

    def __len__(self):
        return len(self.rvas)

    def lookup(self, rva):
        i = bisect.bisect_right(self.rvas, rva) - 1
        if i < 0:
            return None
        off = rva - self.rvas[i]
        if off >= (self.sizes[i] or 0x10000):
            return None
        name = self.names[i] or 'sub_%x' % (self.base + self.rvas[i])
        s = name + ('+0x%x' % off if off else '')
        st = self.status[i]
        return s + (' {%s}' % self.owners[i] if st in ('-', '') else ' {%s, %s}' % (self.owners[i], st))

    def resolve(self, key):
        """VA/RVA/name -> RVA (or raise)."""
        try:
            v = int(key, 0)
            return v - PREF_BASE if v >= PREF_BASE else v
        except ValueError:
            pass
        if key in self.by_name:
            return self.by_name[key]
        hits = [(n, r) for n, r in self.by_name.items() if key in n]
        if len(hits) == 1:
            return hits[0][1]
        if not hits:
            raise ValueError('no function named %r' % key)
        raise ValueError('%r is ambiguous (%d matches, e.g. %s)' % (key, len(hits), ', '.join(n for n, _ in hits[:5])))


STATE = {'syms': None, 'base': None, 'size': None, 'pending': []}


def syms():
    if STATE['syms'] is None:
        STATE['syms'] = Syms(os.environ.get('AW2_SYMBOLS') or os.path.join(ROOT, 'out', 'symbols', 'AlanWake2.exe.syms'))
    return STATE['syms']


def exe_range():
    base = STATE['base'] if STATE['base'] is not None else PREF_BASE
    return base, STATE['size'] or 0x4000000


def symbolize(addr):
    base, size = exe_range()
    if base <= addr < base + size:
        s = syms().lookup(addr - base)
        rva = addr - base
        return 'AlanWake2.exe+0x%x [va 0x%x]%s' % (rva, PREF_BASE + rva, ' ' + s if s else '')
    return None


def frame_desc(target, addr):
    s = symbolize(addr)
    if s:
        return s
    sa = target.ResolveLoadAddress(addr)
    sym = sa.GetSymbol()
    if sym and sym.IsValid():
        mod = sa.GetModule().GetFileSpec().GetFilename() or '?'
        return '%s!%s+0x%x' % (mod, sym.GetName(), addr - sym.GetStartAddress().GetLoadAddress(target))
    return '0x%x' % addr


def eval_addr(frame, target, expr):
    try:
        return int(expr, 0)
    except ValueError:
        pass
    v = (frame.EvaluateExpression(expr) if frame and frame.IsValid() else target.EvaluateExpression(expr))
    if not v.IsValid() or v.GetError().Fail():
        raise ValueError('cannot evaluate %r' % expr)
    return v.GetValueAsUnsigned()


def set_bp(target, rva, result):
    base, _ = exe_range()
    addr = base + rva
    bp = target.BreakpointCreateByAddress(addr)
    print('breakpoint %d at 0x%x %s' % (bp.GetID(), addr, symbolize(addr) or ''), file=result)


# --------------------------------------------------------------------------
# Commands
# --------------------------------------------------------------------------

def cmd_aw2sym(debugger, command, exe_ctx, result, _dict):
    target = exe_ctx.GetTarget()
    frame = exe_ctx.GetFrame()
    for expr in shlex.split(command) or ['$pc']:
        try:
            a = eval_addr(frame, target, expr)
        except ValueError as e:
            result.SetError(str(e))
            return
        print('0x%x %s' % (a, frame_desc(target, a)), file=result)


def cmd_aw2b(debugger, command, exe_ctx, result, _dict):
    target = debugger.GetSelectedTarget()
    for key in shlex.split(command):
        try:
            rva = syms().resolve(key)
        except ValueError as e:
            result.SetError(str(e))
            return
        if STATE['base'] is None:
            STATE['pending'].append(rva)
            print('queued 0x%x %s (exe not mapped yet)' % (PREF_BASE + rva, syms().lookup(rva) or ''), file=result)
        else:
            set_bp(target, rva, result)


def cmd_aw2bt(debugger, command, exe_ctx, result, _dict):
    target = exe_ctx.GetTarget()
    thread = exe_ctx.GetThread()
    n = int(command) if command.strip() else 64
    if not thread or not thread.IsValid():
        result.SetError('no thread')
        return
    print('thread #%d tid 0x%x %s' % (thread.GetIndexID(), thread.GetThreadID(), thread.GetName() or ''), file=result)
    for i, f in enumerate(thread.frames[:n]):
        print('  #%-2d 0x%016x %s' % (i, f.GetPC(), frame_desc(target, f.GetPC())), file=result)
    # Raw scan: lldb cannot unwind guest frames (no PE unwind info), so list
    # exe return addresses on the stack too.
    frame = thread.GetFrameAtIndex(0)
    rsp = frame.FindRegister('rsp').GetValueAsUnsigned()
    err = lldb.SBError()
    data = target.GetProcess().ReadMemory(rsp, 0x2000, err)
    if err.Fail():
        return
    base, size = exe_range()
    shown = 0
    print('stack scan from rsp=0x%x (exe addresses):' % rsp, file=result)
    for off in range(0, len(data), 8):
        v = int.from_bytes(data[off:off + 8], 'little')
        if base <= v < base + size:
            print('  [rsp+0x%04x] %s' % (off, symbolize(v)), file=result)
            shown += 1
            if shown >= 24:
                break


def cmd_aw2report(debugger, command, exe_ctx, result, _dict):
    """Mixed guest/host backtrace (pdata + frame pointers) computed in-process
    by aw2_debug_backtrace(rip, rsp, rbp) for the current thread, or every
    thread with `aw2report all`. Runs the function on the selected thread
    only (other threads stay stopped); output goes to the loader log."""
    process = exe_ctx.GetProcess()
    cur = exe_ctx.GetThread()
    threads = list(process) if command.strip() == 'all' else [cur]
    opts = lldb.SBExpressionOptions()
    opts.SetTryAllThreads(False)
    opts.SetStopOthers(True)
    opts.SetTimeoutInMicroSeconds(10 * 1000 * 1000)
    opts.SetUnwindOnError(True)
    opts.SetIgnoreBreakpoints(True)
    frame = cur.GetFrameAtIndex(0)
    for t in threads:
        f0 = t.GetFrameAtIndex(0)
        regs = [f0.FindRegister(r).GetValueAsUnsigned() for r in ('rip', 'rsp', 'rbp')]
        expr = '(unsigned long)aw2_debug_backtrace(0x%x, 0x%x, 0x%x)' % tuple(regs)
        v = frame.EvaluateExpression(expr, opts)
        if v.GetError().Fail():
            result.SetError('thread #%d: %s' % (t.GetIndexID(), v.GetError().GetCString()))
            return
        print('thread #%d: %d frames (see the loader log)' % (t.GetIndexID(), v.GetValueAsUnsigned()), file=result)


def cmd_aw2info(debugger, command, exe_ctx, result, _dict):
    s = syms()
    print('symbols: %s (%d functions)' % (s.path, len(s)), file=result)
    if STATE['base'] is None:
        print('exe: not mapped yet', file=result)
    else:
        print('exe: base 0x%x size 0x%x' % (STATE['base'], STATE['size']), file=result)
    for rva in STATE['pending']:
        print('queued: 0x%x %s' % (PREF_BASE + rva, s.lookup(rva) or ''), file=result)


def read_global(target, name, size=8):
    syms = target.FindSymbols(name)
    if syms.GetSize() == 0:
        raise ValueError('symbol %s not found' % name)
    addr = syms.GetContextAtIndex(0).GetSymbol().GetStartAddress().GetLoadAddress(target)
    err = lldb.SBError()
    v = target.GetProcess().ReadUnsignedFromMemory(addr, size, err)
    if err.Fail():
        raise ValueError('cannot read %s: %s' % (name, err.GetCString()))
    return addr, v


def cmd_aw2attach(debugger, command, exe_ctx, result, _dict):
    """After `process attach`: learn the exe range, turn queued aw2b into breakpoints."""
    target = debugger.GetSelectedTarget()
    try:
        _, base = read_global(target, 'AW2_EXE_BASE')
        _, size = read_global(target, 'AW2_EXE_SIZE')
    except ValueError as e:
        result.SetError(str(e))
        return
    if base == 0:
        print('aw2: exe not mapped yet (AW2_EXE_BASE = 0); breakpoints stay queued', file=result)
        return
    STATE['base'], STATE['size'] = base, size
    print('aw2: exe mapped at 0x%x (size 0x%x)' % (base, size), file=result)
    for rva in STATE['pending']:
        set_bp(target, rva, result)
    STATE['pending'] = []


def cmd_aw2go(debugger, command, exe_ctx, result, _dict):
    """Release a loader waiting in aw2_debug_image_mapped (AW2_WAIT_DEBUGGER)."""
    target = debugger.GetSelectedTarget()
    try:
        addr, v = read_global(target, 'AW2_DEBUGGER_GO', 4)
    except ValueError as e:
        result.SetError(str(e))
        return
    err = lldb.SBError()
    target.GetProcess().WriteMemory(addr, (1).to_bytes(4, 'little'), err)
    if err.Fail():
        result.SetError('cannot write AW2_DEBUGGER_GO: %s' % err.GetCString())
        return
    print('aw2: loader released; `continue` to run', file=result)


def __lldb_init_module(debugger, _dict):
    for name in ('aw2sym', 'aw2b', 'aw2bt', 'aw2report', 'aw2info', 'aw2attach', 'aw2go'):
        debugger.HandleCommand('command script add -o -f lldb_aw2.cmd_%s %s' % (name, name))
    print('aw2: commands aw2sym aw2b aw2bt aw2report aw2info (symbols: %d)' % len(syms()))
