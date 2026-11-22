"""Best-effort MSVC (Visual C++) name demangler, stdlib only.

Scope: enough to turn RTTI type descriptor names (``.?AV...@@``) and exported function symbols
(``?name@scope@@...``) into readable qualified names, including templates, anonymous
namespaces, lambdas and local scopes (``?1??func@...@Z``). It is NOT a complete undname
replacement: exotic encodings fall back to a raw rendering. Never raises from the public API.
"""

PRIM = {
    'C': 'signed char', 'D': 'char', 'E': 'unsigned char', 'F': 'short', 'G': 'unsigned short',
    'H': 'int', 'I': 'unsigned int', 'J': 'long', 'K': 'unsigned long', 'M': 'float',
    'N': 'double', 'O': 'long double', 'X': 'void', 'Z': '...',
}
PRIM_EXT = {
    'D': '__int8', 'E': 'unsigned __int8', 'F': '__int16', 'G': 'unsigned __int16',
    'H': '__int32', 'I': 'unsigned __int32', 'J': '__int64', 'K': 'unsigned __int64',
    'L': '__int128', 'M': 'unsigned __int128', 'N': 'bool', 'Q': 'char8_t', 'S': 'char16_t',
    'U': 'char32_t', 'W': 'wchar_t',
}
OPS = {
    '0': None, '1': None,  # ctor / dtor handled specially
    '2': 'operator new', '3': 'operator delete', '4': 'operator=', '5': 'operator>>',
    '6': 'operator<<', '7': 'operator!', '8': 'operator==', '9': 'operator!=',
    'A': 'operator[]', 'B': 'operator cast', 'C': 'operator->', 'D': 'operator*',
    'E': 'operator++', 'F': 'operator--', 'G': 'operator-', 'H': 'operator+', 'I': 'operator&',
    'J': 'operator->*', 'K': 'operator/', 'L': 'operator%', 'M': 'operator<', 'N': 'operator<=',
    'O': 'operator>', 'P': 'operator>=', 'Q': 'operator,', 'R': 'operator()', 'S': 'operator~',
    'T': 'operator^', 'U': 'operator|', 'V': 'operator&&', 'W': 'operator||', 'X': 'operator*=',
    'Y': 'operator+=', 'Z': 'operator-=',
}
OPS_U = {
    '0': 'operator/=', '1': 'operator%=', '2': 'operator>>=', '3': 'operator<<=',
    '4': 'operator&=', '5': 'operator|=', '6': 'operator^=', '7': "`vftable'",
    '8': "`vbtable'", '9': "`vcall'", 'A': "`typeof'", 'B': "`local static guard'",
    'C': "`string'", 'D': "`vbase destructor'", 'E': "`vector deleting destructor'",
    'F': "`default constructor closure'", 'G': "`scalar deleting destructor'",
    'H': "`vector constructor iterator'", 'I': "`vector destructor iterator'",
    'J': "`vector vbase constructor iterator'", 'K': "`virtual displacement map'",
    'L': "`eh vector constructor iterator'", 'M': "`eh vector destructor iterator'",
    'N': "`eh vector vbase constructor iterator'", 'O': "`copy constructor closure'",
    'S': "`local vftable'", 'T': "`local vftable constructor closure'",
    'U': 'operator new[]', 'V': 'operator delete[]', 'X': "`placement delete closure'",
    'Y': "`placement delete[] closure'",
}
OPS_UU = {'L': 'co_await', 'M': 'operator<=>', 'K': 'operator ""'}

ANON = 'anonymous_namespace'


class DemangleError(Exception):
    pass


class _D:
    def __init__(self, s, pos=0):
        self.s = s
        self.i = pos
        self.names = []
        self.types = []

    # ---- low level
    def peek(self, k=0):
        j = self.i + k
        return self.s[j] if j < len(self.s) else ''

    def eat(self, t):
        if self.s.startswith(t, self.i):
            self.i += len(t)
            return True
        return False

    def need(self, t):
        if not self.eat(t):
            raise DemangleError('expected %r at %d in %r' % (t, self.i, self.s))

    def simple(self):
        j = self.s.find('@', self.i)
        if j < 0:
            raise DemangleError('unterminated name')
        n = self.s[self.i:j]
        self.i = j + 1
        return n

    def number(self):
        neg = self.eat('?')
        c = self.peek()
        if c.isdigit():
            self.i += 1
            v = int(c) + 1
        else:
            v = 0
            while True:
                c = self.peek()
                if c == '@':
                    self.i += 1
                    break
                if 'A' <= c <= 'P':
                    v = v * 16 + (ord(c) - 65)
                    self.i += 1
                else:
                    raise DemangleError('bad number')
        return -v if neg else v

    def memo_name(self, n):
        if len(self.names) < 10 and n not in self.names:
            self.names.append(n)

    # ---- names
    def template_name(self):
        # after '?$'
        saved = (self.names, self.types)
        self.names, self.types = [], []
        if self.peek() == '?':
            self.i += 1
            base = self.operator_name(None)
        else:
            base = self.simple()
            self.memo_name(base)
        args = []
        while self.peek() != '@':
            if self.i >= len(self.s):
                raise DemangleError('unterminated template')
            args.append(self.template_arg())
        self.i += 1
        self.names, self.types = saved
        return '%s<%s>' % (base, ','.join(a for a in args if a is not None))

    def template_arg(self):
        if self.eat('$$V') or self.eat('$$Z') or self.eat('$S'):
            return None  # empty pack / pack separator
        if self.eat('$0'):
            return str(self.number())
        if self.eat('$1'):
            if self.peek() == '?':
                return '&' + self.symbol_name_only()
            return '&' + self.qualified_name()
        if self.eat('$H') or self.eat('$I') or self.eat('$J'):
            return '&' + self.symbol_name_only()
        if self.eat('$M'):
            self.type_()          # type of the auto param
            return self.template_arg()
        if self.eat('$2'):
            a = self.number(); b = self.number()
            return '%de%d' % (a, b)
        if self.eat('$E?'):
            return '&' + self.symbol_name_only()
        return self.param_type()

    def operator_name(self, cls):
        c = self.peek()
        self.i += 1
        if c == '0':
            return cls or 'ctor'
        if c == '1':
            return '~' + (cls or 'dtor')
        if c == '_':
            c2 = self.peek(); self.i += 1
            if c2 == '_':
                c3 = self.peek(); self.i += 1
                return OPS_UU.get(c3, 'op__' + c3)
            if c2 == 'R':  # RTTI descriptors
                c3 = self.peek(); self.i += 1
                return "`RTTI %s'" % c3
            return OPS_U.get(c2, 'op_' + c2)
        if c == '$':
            self.i -= 1
            self.need('$')
            return self.template_name()
        return OPS.get(c, 'op' + c)

    def unqualified_first(self):
        """First (innermost) name fragment of a qualified name."""
        c = self.peek()
        if c.isdigit():
            self.i += 1
            return self.names[int(c)] if int(c) < len(self.names) else '?bref%s' % c
        if self.eat('?$'):
            n = self.template_name()
            self.memo_name(n)
            return n
        if c == '?':
            self.i += 1
            return ('?op', self.operator_name(None))
        n = self.simple()
        self.memo_name(n)
        return n

    def scope_fragment(self):
        c = self.peek()
        if c.isdigit():
            self.i += 1
            return self.names[int(c)] if int(c) < len(self.names) else '?bref%s' % c
        if self.eat('?$'):
            n = self.template_name()
            self.memo_name(n)
            return n
        if self.eat('?A0x') or self.eat('?A'):
            self.simple()
            return ANON
        if c == '?':
            self.i += 1
            if self.peek() == '?':  # ??nested symbol  (rare form without number)
                return self.symbol_name_only()
            self.number()
            if self.peek() == '?':
                self.i += 1                      # separator; nested symbol follows ('?name...')
                return self.symbol_name_only()   # local scope: name of the enclosing function
            return '`%s' % self.simple()
        n = self.simple()
        self.memo_name(n)
        return n

    def qualified_name(self):
        first = self.unqualified_first()
        scopes = []
        while self.peek() != '@':
            if self.i >= len(self.s):
                raise DemangleError('unterminated qualified name')
            scopes.append(self.scope_fragment())
        self.i += 1
        if isinstance(first, tuple):  # operator: ctor/dtor need class name
            op = first[1]
            cls = scopes[0] if scopes else ''
            base = cls.split('<')[0] if cls else ''
            if op == 'ctor':
                op = base
            elif op == '~dtor':
                op = '~' + base
            first = op
        return '::'.join(list(reversed(scopes)) + [first])

    def symbol_name_only(self):
        """Parse a full nested symbol ('?name@...@Z') and return only its qualified name."""
        sub = _D(self.s, self.i)
        sub.need('?')
        if sub.peek() == '?' and sub.peek(1) not in ('$',):
            # operator / special name: ??0Foo@@...
            sub.i += 1
            op = sub.operator_name(None)
            scopes = []
            while sub.peek() != '@':
                scopes.append(sub.scope_fragment())
            sub.i += 1
            cls = scopes[0] if scopes else ''
            base = cls.split('<')[0]
            if op == 'ctor':
                op = base
            elif op == '~dtor':
                op = '~' + base
            name = '::'.join(list(reversed(scopes)) + [op])
        else:
            name = sub.qualified_name()
        sub.skip_symbol_type()
        self.i = sub.i
        return name

    # ---- types
    def cv(self):
        c = self.peek()
        self.i += 1
        return {'A': '', 'B': 'const ', 'C': 'volatile ', 'D': 'const volatile '}.get(c, '')

    def ptr_mods(self):
        while self.peek() in ('E', 'F', 'I'):
            self.i += 1

    def calling_conv(self):
        self.i += 1  # A cdecl, E thiscall, G stdcall, I fastcall, Q vectorcall ...

    def func_type_rest(self):
        """After calling convention: return type, params, throw spec. Returns (ret, params)."""
        if self.eat('@'):
            ret = ''
        else:
            if self.eat('?'):
                self.cv()
            ret = self.type_()
        params = self.param_list()
        # throw spec
        if not self.eat('Z'):
            if self.eat('_E'):
                pass
            else:
                self.type_()
        return ret, params

    def param_list(self):
        if self.eat('X'):
            return 'void'
        ps = []
        while True:
            c = self.peek()
            if c == '@':
                self.i += 1
                break
            if c == 'Z':
                self.i += 1
                ps.append('...')
                break
            if c == '':
                raise DemangleError('unterminated params')
            ps.append(self.param_type())
        return ','.join(ps)

    def param_type(self):
        c = self.peek()
        if c.isdigit():
            self.i += 1
            k = int(c)
            return self.types[k] if k < len(self.types) else '?tref%d' % k
        start = self.i
        t = self.type_()
        if self.i - start > 1 and len(self.types) < 10:
            self.types.append(t)
        return t

    def type_(self):
        c = self.peek()
        if c == '':
            raise DemangleError('eof in type')
        if c in PRIM:
            self.i += 1
            return PRIM[c]
        if c == '_':
            self.i += 1
            c2 = self.peek(); self.i += 1
            if c2 == 'O':  # array
                return self.type_() + '[]'
            return PRIM_EXT.get(c2, '_' + c2)
        if c in 'TUV':
            self.i += 1
            return self.qualified_name()
        if c == 'W':
            self.i += 2
            return self.qualified_name()
        if c in 'PQRS':
            self.i += 1
            const_ptr = ' const' if c in 'QS' else ''
            if self.peek() == '6':
                self.i += 1
                self.calling_conv()
                ret, params = self.func_type_rest()
                return '%s (*%s)(%s)' % (ret, const_ptr.strip(), params)
            if self.peek() == '8':
                self.i += 1
                cls = self.qualified_name()
                self.ptr_mods(); self.cv(); self.calling_conv()
                ret, params = self.func_type_rest()
                return '%s (%s::*)(%s)' % (ret, cls, params)
            if self.eat('$$C'):  # pointer to cv-qualified
                pass
            self.ptr_mods()
            if self.peek() in 'QRST':      # pointer to data member: cv + class + type
                self.i += 1
                cls = self.qualified_name()
                return '%s %s::*' % (self.type_(), cls)
            q = self.cv()
            return '%s%s*%s' % (q, self.type_(), const_ptr)
        if c in 'AB':
            self.i += 1
            self.ptr_mods()
            q = self.cv()
            return '%s%s&' % (q, self.type_())
        if c == '?':
            self.i += 1
            q = self.cv()
            return q + self.type_()
        if c == 'Y':
            self.i += 1
            n = self.number()
            dims = [self.number() for _ in range(n)]
            return self.type_() + ''.join('[%d]' % d for d in dims)
        if c == '$':
            if self.eat('$$Q') or self.eat('$$R'):
                self.ptr_mods(); q = self.cv()
                return '%s%s&&' % (q, self.type_())
            if self.eat('$$A6'):
                self.calling_conv()
                ret, params = self.func_type_rest()
                return '%s(%s)' % (ret, params)
            if self.eat('$$A8@@'):
                self.calling_conv()
                ret, params = self.func_type_rest()
                return '%s(%s)' % (ret, params)
            if self.eat('$$B'):
                return self.type_()
            if self.eat('$$C'):
                q = self.cv()
                return q + self.type_()
            if self.eat('$$T'):
                return 'std::nullptr_t'
            if self.eat('$$Y'):
                return self.qualified_name()
            if self.eat('$$V') or self.eat('$$Z'):
                return ''
            return self.template_arg()
        raise DemangleError('unknown type code %r at %d' % (c, self.i))

    def skip_symbol_type(self):
        """Skip the type encoding after a symbol's qualified name."""
        c = self.peek()
        if c == '':
            return
        self.i += 1
        if c in '0123456789':           # data: storage class + type + cv
            if c in '01234':
                self.type_()
                self.ptr_mods(); self.cv()
            elif c in '67':             # vftable
                self.ptr_mods(); self.cv()
                while self.peek() not in ('@', ''):
                    self.qualified_name()
                self.eat('@')
            return
        if c in 'YZ':                    # free function: cc + type
            self.calling_conv()
            self.func_type_rest()
            return
        if c == '$':                     # thunks etc.
            c2 = self.peek(); self.i += 1
            if c2 in '012345':
                self.number(); self.number() if c2 in '012345' and False else None
            self.ptr_mods(); self.cv(); self.calling_conv()
            self.func_type_rest()
            return
        # member function: access code letters A..X (static: C,D,K,L,S,T; virtual: E,F,M,N,U,V)
        if c in 'CDKLST':
            self.calling_conv()
            self.func_type_rest()
            return
        if c in 'GHOPWX':                # adjustor thunks
            self.number()
        self.ptr_mods(); self.cv()       # this-pointer qualifiers
        self.calling_conv()
        self.func_type_rest()


def type_descriptor_name(mangled):
    """'.?AVfoo@bar@@' -> 'bar::foo'. Falls back to a crude split."""
    try:
        s = mangled
        if s.startswith('.?A'):
            d = _D(s, 3)
            d.i += 1  # V/U/T
            if s[3] == 'W':
                d.i += 1
            return d.qualified_name()
    except (DemangleError, IndexError, ValueError):
        pass
    return _crude(mangled)


def symbol_qualified_name(mangled):
    """'?foo@bar@@QEAAXXZ' -> 'bar::foo'. Non-mangled names are returned unchanged."""
    if not mangled.startswith('?'):
        return mangled
    try:
        d = _D(mangled, 0)
        return d.symbol_name_only()
    except (DemangleError, IndexError, ValueError):
        return _crude(mangled)


def _crude(m):
    s = m.lstrip('.?').lstrip('?')
    if s.startswith(('AV', 'AU', 'AT')):
        s = s[2:]
    core = s.split('@@')[0]
    parts = [p for p in core.split('@') if p and not p.startswith('$')]
    return '::'.join(reversed(parts)) or m


def split_qualified(name):
    """Split 'a::b<c::d>::e' on top-level '::' (ignores separators inside <> and ())."""
    parts, depth, cur, i = [], 0, [], 0
    while i < len(name):
        ch = name[i]
        if ch in '<(':
            depth += 1
        elif ch in '>)':
            depth = max(0, depth - 1)
        if depth == 0 and name.startswith('::', i):
            parts.append(''.join(cur)); cur = []; i += 2
            continue
        cur.append(ch); i += 1
    parts.append(''.join(cur))
    return parts


if __name__ == '__main__':
    import sys
    for a in sys.argv[1:]:
        print(type_descriptor_name(a) if a.startswith('.?A') else symbol_qualified_name(a))
