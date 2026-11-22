#!/usr/bin/env python3
"""Tests for tools/fndb/retbody.py on synthetic listings:  python3 tools/fndb/test_retbody.py"""
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import retbody  # noqa: E402


def listing(*fns):
    """objdump-like text from [(va, [insn text, ...]), ...] (4-byte insns)."""
    out = []
    for va, insns in fns:
        for k, s in enumerate(insns):
            mn, _, rest = s.partition(" ")
            out.append(f"{va + 4 * k:x}:     \t{mn}\t{rest}")
    return "\n".join(out) + "\n"


def run(fns, decl=None, imp_rax=None, imp_xmm=None):
    img = retbody.Image(None, listing(*fns))
    frags = {va: [(va, va + 4 * len(ins))] for va, ins in fns}
    return retbody.analyze(img, frags, set(), decl or {}, imp_rax or {}, imp_xmm or {})


class RetBody(unittest.TestCase):
    def test_void_incoming(self):
        W, X, _ = run([(0x1000, ["mov qword ptr [rcx], rdx", "ret"])])
        self.assertEqual(W[0x1000], 0)
        self.assertEqual(X[0x1000], 1)  # xmm0 may be a float argument returned as is

    def test_incoming_read_is_no_bound(self):  # __chkstk-like: rax is an input
        W, _, _ = run([(0x1000, ["sub r10, rax", "ret"])])
        self.assertEqual(W[0x1000], 64)

    def test_own_def_next_to_incoming_is_no_bound(self):
        W, _, _ = run([(0x1000, ["test ecx, ecx", "je 0x100c", "mov eax, 0x1", "ret"])])
        self.assertEqual(W[0x1000], 64)

    def test_pass_through_bounds_by_callee(self):
        W, _, _ = run([(0x1000, ["call 0x2000", "ret"]), (0x2000, ["mov al, 0x1", "ret"])],
                      decl={0x2000: (8, 0)})
        self.assertEqual(W[0x1000], 8)

    def test_extension_is_own_def(self):
        W, _, _ = run([(0x1000, ["call 0x2000", "movzx eax, al", "ret"]), (0x2000, ["mov al, 0x1", "ret"])],
                      decl={0x2000: (8, 0)})
        self.assertEqual(W[0x1000], 64)

    def test_cookie_check_is_walked_through(self):
        cookie = (0x3000, ["cmp rcx, qword ptr [rip + 0x100]", "jne 0x3010", "rol rcx, 0x10", "ret",
                           "jmp 0x4000"])
        W, _, _ = run([(0x1000, ["mov al, 0x1", "call 0x3000", "ret"]), cookie, (0x4000, ["ret"])])
        self.assertEqual(W[0x1000], 64)  # the own def before the check, not the check's "void"

    def test_xmm_dead_through_nofloat_import(self):
        _, X, _ = run([(0x1000, ["call qword ptr [rip + 0x10] # 0x9000", "ret"])], imp_xmm={0x9000: 0})
        self.assertEqual(X[0x1000], 0)

    def test_unreachable_tail_is_ignored(self):  # export thunk followed by the next thunk
        W, _, _ = run([(0x1000, ["jmp 0x2000", "int3", "jmp 0x3000"]), (0x2000, ["mov eax, 0x1", "ret"]),
                       (0x3000, ["ret"])], decl={0x3000: (0, 0)})
        self.assertEqual(W[0x1000], 64)

    def test_noreturn_callee_cuts_the_path(self):
        W, _, _ = run([(0x1000, ["test ecx, ecx", "je 0x100c", "call 0x2000", "ret"]), (0x2000, ["int3"])])
        self.assertEqual(W[0x1000], 0)  # only the incoming path reaches the ret

    def test_jump_table_cases_are_walked(self):  # v8 +0x363930 (AST visitor switch)
        base, table = 0x0, 0x108000
        fn = (0x1000, ["movzx eax, byte ptr [rdx + 0x4]", "cmp eax, 0x1", "ja 0x1020",
                       "lea r14, [rip - 0x1010] # 0x0", "mov eax, dword ptr [r14 + 4*rax + 0x108000]",
                       "add rax, r14", "jmp rax", "int3",
                       "ret",                             # 0x1020: default, rax = own def
                       "call 0x2000", "ret",              # 0x1024: case 0
                       "call 0x2000", "ret"])             # 0x102c: case 1
        tab = (0x1024).to_bytes(4, "little") + (0x102c).to_bytes(4, "little") + b"\xcc" * 8
        # A declared-void callee (export type) that leaves its own rax (not a pass-through).
        img = retbody.Image(None, listing(fn, (0x2000, ["mov eax, 0x7", "ret"])))
        frags = {0x1000: [(0x1000, 0x1034)], 0x2000: [(0x2000, 0x2008)]}
        W, _, funcs = retbody.analyze(img, frags, set(), {0x2000: (0, 0)}, {}, {})
        self.assertEqual(W[0x1000], 64)  # no image bytes: the switch is an open indirect jmp
        self.assertTrue(funcs[0x1000][2]["open"])
        img.base, img.read = base, lambda va, n: tab[va - table:va - table + n] if va >= table else None
        W, _, funcs = retbody.analyze(img, frags, set(), {0x2000: (0, 0)}, {}, {})
        self.assertFalse(funcs[0x1000][2]["open"])
        self.assertEqual(W[0x2000], 0)
        self.assertEqual(W[0x1000], 0)  # the cases return the void callee's rax


if __name__ == "__main__":
    unittest.main()
