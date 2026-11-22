//! `r::` path helpers operating on [`InplaceString`] paths.
//!
//! Paths use `/` as the separator on output; input components may be split
//! on either `/` or `\` (the separator set view at 0x143079458 = `"/\\"`).
//! Character comparisons go through the fold table at 0x1426d01e0 (the
//! to-upper table also used by the `*_nocase` string compares), as signed bytes.

use crate::exe::{self, rd};
use crate::ns::r::inplace_string::{InplaceString, StrView};

const FOLD_VA: u64 = 0x1_426d_01e0;
/// `std::string_view` `{ "/\\", 2 }` in .rdata (separator set).
const SEPS_VIEW_VA: u64 = 0x1_4307_9458;
/// `"."` (1 char compared).
const DOT_VA: u64 = 0x1_4307_8be4;
/// `".."` (2 chars compared).
const DOTDOT_VA: u64 = 0x1_4307_8bec;

const RESERVE_VA: u64 = 0x1_4240_8f50;
const APPEND_VA: u64 = 0x1_4240_a630;
const CLEAR_VA: u64 = 0x1_4240_a8f0;
const PUSH_BACK_VA: u64 = 0x1_4240_a590;
/// `r::InplaceString` resize/truncate to `n` (`(s*, n)`); not decompiled.
const RESIZE_VA: u64 = 0x1_4240_a960;
/// Pops the last component of the path (`(path*, bool) -> bool`), see
/// [`path_pop_component`]. path_append calls it through the VA so the mock/hook
/// machinery still sees the call.
const POP_COMPONENT_VA: u64 = 0x1_401e_00a0;
/// `std::_Xout_of_range` (throws).
const XOUT_OF_RANGE_VA: u64 = 0x1_401d_e590;
/// `"/\\"` literal in .rdata (the bytes the separator view points at); the path
/// iterators use it with a constant count of 2.
const SEP_LIT_VA: u64 = 0x1_4307_8be0;
/// `"/"` (the root-directory view returned for an all-separator path).
const ROOT_SLASH_VA: u64 = 0x1_4307_8be8;

use crate::ns::stl;

#[inline(always)]
unsafe fn fold(c: u8) -> i8 {
    rd::<u8>(exe::ptr::<u8>(FOLD_VA) as usize + c as usize) as i8
}

#[inline(always)]
unsafe fn len_of(s: *const InplaceString) -> u32 {
    rd::<u32>(s as usize + 4)
}

#[inline(always)]
unsafe fn set_len(s: *mut InplaceString, n: u32) {
    exe::wr::<u32>(s as usize + 4, n)
}

#[inline(always)]
unsafe fn capacity(s: *const InplaceString) -> u64 {
    ((rd::<u32>(s as usize) & 0xff_ffff) << 3) as u64
}

#[inline(always)]
unsafe fn storage(s: *const InplaceString) -> *mut u8 {
    let c = rd::<u8>(s as usize + 3) as u32;
    let inline_cap = if c < 0x80 { c * 8 } else { (c << 8).wrapping_sub(0x7c00) };
    let inline = (s as usize + 8) as *mut u8;
    if (capacity(s) as u32) > inline_cap {
        rd::<u64>(inline as usize) as *mut u8
    } else {
        inline
    }
}

#[inline(always)]
unsafe fn seps_view() -> StrView {
    let p = exe::ptr::<u8>(SEPS_VIEW_VA) as usize;
    StrView { ptr: rd::<u64>(p) as *const u8, len: rd::<u64>(p + 8) }
}

/// Inlined `find_first_of(seps, from)` from the original: scans `view[from..]`
/// for a char whose fold equals the fold of `seps.ptr[0]` or `seps.ptr[1]`
/// (the count is the constant 2, the pointer comes from the separator view).
#[inline(always)]
unsafe fn find_sep(vptr: u64, vlen: u64, seps_ptr: u64, from: u64) -> u64 {
    if from < vlen {
        let (s0, s1) = (fold(rd::<u8>(seps_ptr as usize)), fold(rd::<u8>(seps_ptr as usize + 1)));
        for i in from..vlen {
            let c = fold(rd::<u8>((vptr + i) as usize));
            if s0 == c || s1 == c {
                return i;
            }
        }
    }
    u64::MAX
}

/// `path /= view`: append path `view` to the `r::InplaceString` path `s`,
/// normalizing as it goes; returns `s`.
///
/// Original: 0x1401e01e0 (1247 bytes; 71 callers, e.g. the Pack2FileSystem and
/// resource-path builders). Behavior, from the disassembly:
/// - Empty `view`: no-op.
/// - Rooted `view` (`/x`, `\x` or `C:...`): `s` is emptied (len 0, NUL), reserved
///   to `view.len` if needed, and the root (`view[..1]` or `view[..2]`) appended.
///   Otherwise: if `s` is exactly `"."` (folded) it is cleared, then reserved to
///   `len + (len != 0) + view.len` if needed.
/// - Components are the runs between separators (`find_first_not_of(seps, pos)`
///   via 0x1401e1a40, then an inlined `find_first_of`). Scanning starts after
///   the root (1 for `/`, 2 for `C:`, else 0). For each component:
///   `"."` is skipped; `".."` with a non-empty `s` calls 0x1401e00a0(s, true) and
///   is dropped if that returns true; anything else appends `/` (unless `s` is
///   empty or already ends in `/`) and then the component.
/// - If `s` ends up empty it becomes `"."`.
pub unsafe extern "win64" fn path_append(s: *mut InplaceString, view: *const StrView) -> *mut InplaceString {
    let vptr = rd::<u64>(view as usize);
    let vlen = rd::<u64>(view as usize + 8);
    if vlen == 0 {
        return s;
    }
    let c0 = rd::<u8>(vptr as usize);
    if c0 == b'/' || c0 == b'\\' || (vlen >= 2 && rd::<u8>(vptr as usize + 1) == b':') {
        set_len(s, 0);
        *storage(s) = 0;
        if vlen >= capacity(s) {
            exe::call_va!(RESERVE_VA, (s, vlen));
        }
        let p = rd::<u64>(view as usize);
        let c0 = rd::<u8>(p as usize);
        let root = if c0 == b'/' || c0 == b'\\' {
            StrView { ptr: p as *const u8, len: vlen.min(1) }
        } else if vlen >= 2 && rd::<u8>(p as usize + 1) == b':' {
            StrView { ptr: p as *const u8, len: vlen.min(2) }
        } else {
            StrView { ptr: core::ptr::null(), len: 0 }
        };
        exe::call_va!(APPEND_VA, (s, &root as *const StrView));
    } else {
        let d = storage(s);
        if len_of(s) == 1 && fold(rd::<u8>(d as usize)) == fold(rd::<u8>(exe::ptr::<u8>(DOT_VA) as usize)) {
            exe::call_va!(CLEAR_VA, (s));
        }
        let l = len_of(s) as u64;
        let need = l + (l != 0) as u64 + vlen;
        if need >= capacity(s) {
            exe::call_va!(RESERVE_VA, (s, need));
        }
    }

    // First component starts after the root.
    let p = rd::<u64>(view as usize);
    let c0 = rd::<u8>(p as usize);
    let start = if c0 == b'/' || c0 == b'\\' {
        if 0 < vlen { 1 } else { vlen }
    } else if vlen >= 2 && rd::<u8>(p as usize + 1) == b':' {
        if 1 < vlen { 2 } else { vlen }
    } else {
        0
    };

    let view_copy = StrView { ptr: vptr as *const u8, len: vlen };
    let dot = fold(rd::<u8>(exe::ptr::<u8>(DOT_VA) as usize));
    let dd = exe::ptr::<u8>(DOTDOT_VA) as usize;
    let (dd0, dd1) = (fold(rd::<u8>(dd)), fold(rd::<u8>(dd + 1)));

    let mut seps = seps_view();
    let mut pos = stl::traits_find_first_not_of(&view_copy, &seps, start);
    let mut end = find_sep(vptr, vlen, seps.ptr as u64, pos);
    while pos != u64::MAX {
        if vlen < pos {
            exe::call_va!(XOUT_OF_RANGE_VA, ());
            unreachable!("_Xout_of_range returned");
        }
        let comp_ptr = vptr + pos;
        let comp_len = end.wrapping_sub(pos).min(vlen - pos);
        let is_dot = comp_len == 1 && fold(rd::<u8>(comp_ptr as usize)) == dot;
        if !is_dot {
            let mut popped = false;
            if len_of(s) != 0
                && comp_len == 2
                && fold(rd::<u8>(comp_ptr as usize)) == dd0
                && fold(rd::<u8>(comp_ptr as usize + 1)) == dd1
            {
                popped = exe::call_va!(POP_COMPONENT_VA, (s, 1u8) -> bool);
            }
            if !popped {
                let l = len_of(s);
                if l != 0 && rd::<u8>(storage(s) as usize + l as usize - 1) != b'/' {
                    exe::call_va!(PUSH_BACK_VA, (s, b'/'));
                }
                let comp = StrView { ptr: comp_ptr as *const u8, len: comp_len };
                exe::call_va!(APPEND_VA, (s, &comp as *const StrView));
            }
            seps = seps_view();
        }
        pos = stl::traits_find_first_not_of(&view_copy, &seps, end);
        end = find_sep(vptr, vlen, seps.ptr as u64, pos);
    }
    if len_of(s) == 0 {
        exe::call_va!(PUSH_BACK_VA, (s, b'.'));
    }
    s
}

#[inline(always)]
unsafe fn view_of(v: *const StrView) -> (u64, u64) {
    (rd::<u64>(v as usize), rd::<u64>(v as usize + 8))
}

#[inline(always)]
unsafe fn set_view(out: *mut StrView, p: u64, n: u64) {
    exe::wr::<u64>(out as usize, p);
    exe::wr::<u64>(out as usize + 8, n);
}

/// Index of the first char after the root (`/x`, `\x` -> 1, `C:x` -> 2, clamped
/// to `len`), else 0. This is the `root_name + root_directory` skip that the
/// path iterators inline.
#[inline(always)]
unsafe fn root_skip(p: u64, n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let c = rd::<u8>(p as usize);
    let r = if c == b'/' || c == b'\\' {
        0
    } else if n >= 2 && rd::<u8>(p as usize + 1) == b':' {
        1
    } else {
        return 0;
    };
    if r < n { r + 1 } else { n }
}

/// Strip trailing separators from a path view: `out = view` minus its trailing
/// run of chars whose fold matches `seps[0]` or `seps[1]` (the bytes behind the
/// separator view at 0x143079458, count fixed at 2). If that leaves nothing and
/// the input starts with `/` or `\`, the result is the `"/"` literal at
/// 0x143078be8 instead. Returns `out`.
///
/// Original: 0x1401df850 (293 bytes in 5 fragments, leaf, 245 callers). Note the
/// original writes `out` and only then re-reads `view->len` / `view->ptr[0]` for
/// the root fix-up, so the in-place call (`out == view`) never gets the `"/"`
/// result. That order is preserved here.
pub unsafe extern "win64" fn path_strip_trailing_seps(out: *mut StrView, view: *const StrView) -> *mut StrView {
    let (p, n) = view_of(view);
    let seps = rd::<u64>(exe::ptr::<u8>(SEPS_VIEW_VA) as usize);
    let end = p.wrapping_add(n);
    let mut len = n;
    if end != p {
        let (s0, s1) = (fold(rd::<u8>(seps as usize)), fold(rd::<u8>(seps as usize + 1)));
        let mut q = end;
        loop {
            let c = fold(rd::<u8>(q as usize - 1));
            if c != s0 && c != s1 {
                break;
            }
            q -= 1;
            if q == p {
                break;
            }
        }
        len = n.wrapping_add(q.wrapping_sub(end));
    }
    set_view(out, p, len);
    if rd::<u64>(out as usize + 8) == 0 && rd::<u64>(view as usize + 8) != 0 {
        let c = rd::<u8>(rd::<u64>(view as usize) as usize);
        if c == b'/' || c == b'\\' {
            set_view(out, exe::ptr::<u8>(ROOT_SLASH_VA) as u64, 1);
        }
    }
    out
}

/// `path.filename()` on a view (member function: `this` in rcx, hidden return
/// view in rdx). Returns the part after the last separator. If there is no
/// separator, it returns the part after the root (`/`, `\` or `C:`), or the whole
/// view. Returns `out`.
///
/// Original: 0x1401dfa80 (250 bytes). It calls `_Traits_find_last_of(ptr, len, -,
/// seps, 2)` (0x1401e22e0, [`stl::traits_find_last_of`]) with the separator view's
/// pointer. The `substr` bounds check throws `std::out_of_range` via 0x1401de590,
/// which can't happen for these offsets but is kept.
pub unsafe extern "win64" fn path_filename(this: *const StrView, out: *mut StrView) -> *mut StrView {
    let (p, n) = view_of(this);
    let seps = rd::<u64>(exe::ptr::<u8>(SEPS_VIEW_VA) as usize);
    let i = stl::find_last_of_raw(p, n, seps, 2);
    let (rp, rl);
    if i != u64::MAX {
        let start = i + 1;
        if n < start {
            exe::call_va!(XOUT_OF_RANGE_VA, ());
            unreachable!("_Xout_of_range returned");
        }
        rp = p + start;
        rl = n - start;
    } else {
        let p2 = rd::<u64>(this as usize); // the original re-reads the view here
        let k = root_skip(p2, n);
        if n < k {
            exe::call_va!(XOUT_OF_RANGE_VA, ());
            unreachable!("_Xout_of_range returned");
        }
        rp = p2 + k;
        rl = n - k;
    }
    set_view(out, rp, rl);
    out
}

/// Remove the last component of the `r::InplaceString` path `s`. Returns false,
/// without changing `s`, if `guard` is set and that component is exactly `..`.
///
/// Original: 0x1401e00a0 (306 bytes; called with `guard = true` by path_append on
/// `..`). With `f = filename(s)`: if `f` is non-empty and `f.len + 1 < s.len`, it
/// truncates to `s.len - f.len - 1`. That drops `f` and the one separator before
/// it, so `a//b` becomes `a/`. Otherwise it truncates to the root (`/`, `\` -> 1,
/// `C:` -> 2, clamped; else 0). The truncation goes through 0x14240a960. The `..`
/// test compares raw bytes, not folded ones.
pub unsafe extern "win64" fn path_pop_component(s: *mut InplaceString, guard: u8) -> bool {
    let data = storage(s) as u64;
    let len = len_of(s) as u64;
    let snap = StrView { ptr: data as *const u8, len };
    let mut f = StrView { ptr: core::ptr::null(), len: 0 };
    path_filename(&snap, &mut f);
    let (fp, fl) = (f.ptr as u64, f.len);
    if guard != 0 && fl == 2 && rd::<u8>(fp as usize) == b'.' && rd::<u8>(fp as usize + 1) == b'.' {
        return false;
    }
    let cur = len_of(s) as u64;
    let new_len = if fl != 0 && fl + 1 < cur {
        cur - fl - 1
    } else if len == 0 {
        0
    } else {
        let c = rd::<u8>(data as usize);
        let r = if c == b'/' || c == b'\\' {
            1
        } else if len >= 2 && rd::<u8>(data as usize + 1) == b':' {
            2
        } else {
            0
        };
        r.min(len)
    };
    exe::call_va!(RESIZE_VA, (s, new_len));
    true
}

/// One path iterator as the mismatch functions return it: the whole path view
/// plus the `[begin, end)` of the current component (`end` may be npos).
#[inline(always)]
unsafe fn first_component(p: u64, n: u64, sep: u64) -> (u64, u64) {
    let b = stl::find_first_not_of_raw(p, n, sep, 2, root_skip(p, n));
    (b, stl::find_first_of_raw(p, n, sep, 2, b))
}

/// Shared body of [`path_mismatch_view_str`] and [`path_mismatch_view_view`]:
/// `std::mismatch` over the components of paths `a` and `b`, comparing
/// components case-insensitively through the fold table. It stops at the
/// first pair whose lengths or folded bytes differ, or when either path runs
/// out. Writes `{a.ptr, a.len, a.begin, a.end, b.ptr, b.len, b.begin, b.end}`
/// (64 bytes) to `out`.
unsafe fn path_mismatch(out: u64, ap: u64, al: u64, bp: u64, bl: u64) {
    let sep = exe::ptr::<u8>(SEP_LIT_VA) as u64;
    let (mut ab, mut ae) = first_component(ap, al, sep);
    let (mut bb, mut be) = first_component(bp, bl, sep);
    if ab != u64::MAX {
        'outer: while bb != u64::MAX {
            if bl < bb || al < ab {
                exe::call_va!(XOUT_OF_RANGE_VA, ());
                unreachable!("_Xout_of_range returned");
            }
            let bn = be.wrapping_sub(bb).min(bl - bb);
            let an = ae.wrapping_sub(ab).min(al - ab);
            if an != bn {
                break;
            }
            for i in 0..an {
                if fold(rd::<u8>((ap + ab + i) as usize)) != fold(rd::<u8>((bp + bb + i) as usize)) {
                    break 'outer;
                }
            }
            ab = stl::find_first_not_of_raw(ap, al, sep, 2, ae);
            ae = stl::find_first_of_raw(ap, al, sep, 2, ab);
            bb = stl::find_first_not_of_raw(bp, bl, sep, 2, be);
            be = stl::find_first_of_raw(bp, bl, sep, 2, bb);
            if ab == u64::MAX {
                break;
            }
        }
    }
    for (k, v) in [ap, al, ab, ae, bp, bl, bb, be].into_iter().enumerate() {
        exe::wr::<u64>(out as usize + k * 8, v);
    }
}

/// `std::mismatch(a.begin(), a.end(), b.begin(), b.end())` over path components,
/// with `a` a path view and `b` an `r::InplaceString` path (`b.len` is its u32 length).
/// Returns `out`, which receives two 32-byte iterators (see `path_mismatch`).
///
/// Original: 0x1413a7e00 (1086 bytes, 3 callers). One of the two hot
/// path-relativization helpers on the resource loading thread (7.1% of its
/// samples). The out-of-range checks call 0x1401de590 like the original.
pub unsafe extern "win64" fn path_mismatch_view_str(out: *mut u8, a: *const StrView, b: *const InplaceString) -> *mut u8 {
    let (ap, al) = view_of(a);
    let (bp, bl) = (storage(b) as u64, len_of(b) as u64);
    path_mismatch(out as u64, ap, al, bp, bl);
    out
}

/// Same as [`path_mismatch_view_str`] with two path views.
///
/// Original: 0x1401f5ea0 (1093 bytes, 16 callers; 6.4% of the loading thread's samples).
pub unsafe extern "win64" fn path_mismatch_view_view(out: *mut u8, a: *const StrView, b: *const StrView) -> *mut u8 {
    let (ap, al) = view_of(a);
    let (bp, bl) = view_of(b);
    path_mismatch(out as u64, ap, al, bp, bl);
    out
}

/// `"data"` (4 chars compared) — the resource root directory name.
const DATA_LIT_VA: u64 = 0x1_4307_dd78;
/// 256-byte to-lower table used by the resource-path hash.
const LOWER_VA: u64 = 0x1_427e_54e0;
/// `r::InplaceString<>::assign(view, pos, count)` (verified replacement).
const ASSIGN_RANGE_VA: u64 = 0x1_4240_9b50;
/// `r::InplaceString<>::~InplaceString()` (verified replacement).
const STRING_DTOR_VA: u64 = 0x1_4240_ab10;
/// [`path_append`] (called through the VA so hooks/mocks/try thunk see it).
const PATH_APPEND_VA: u64 = 0x1_401e_01e0;

/// `find_first_of(seps, pos)` on a path view, with the separator view's
/// pointer and a count of 2. Returns the index or npos.
///
/// Original: 0x1401dfdf0 (129 bytes, leaf, 5 callers). This is the non-inlined
/// copy of the scan that the path iterators otherwise inline.
pub unsafe extern "win64" fn path_find_sep(view: *const StrView, pos: u64) -> u64 {
    let (p, n) = view_of(view);
    let seps = rd::<u64>(exe::ptr::<u8>(SEPS_VIEW_VA) as usize);
    stl::find_first_of_raw(p, n, seps, 2, pos)
}

/// True if the path component `comp` starts with `"data"`, compared
/// case-insensitively through the fold table. So `data`, `Data2` and `DATAx`
/// all match. Only `min(len, 4) == 4` is checked; there is no full-length
/// compare.
///
/// Original: 0x142423740 (86 bytes, leaf; reached only from 0x1413a2490).
pub unsafe extern "win64" fn path_component_is_data(comp: *const StrView) -> bool {
    let (p, n) = view_of(comp);
    if n.min(4) != 4 {
        return false;
    }
    let lit = exe::ptr::<u8>(DATA_LIT_VA) as usize;
    for i in 0..4 {
        if fold(rd::<u8>(p as usize + i)) != fold(rd::<u8>(lit + i)) {
            return false;
        }
    }
    true
}

/// Path iterator `{ptr, len, begin, end}` to the first component of `view` that
/// [`path_component_is_data`] accepts, or the end iterator (`begin = npos`).
/// Returns `out`.
///
/// Original: 0x1413a2490 (481 bytes in 5 fragments; fndb's sparsehash owner
/// guess is a neighbor artifact). This is `std::find_if(path.begin(), path.end(),
/// is_data)`. Iteration starts after the root. It advances with
/// `find_first_not_of(seps, end)` and then 0x1401dfdf0 ([`path_find_sep`]), and
/// it reads the view back from `out` while advancing.
pub unsafe extern "win64" fn path_find_data_component(out: *mut u64, view: *const StrView) -> *mut u64 {
    let (p, n) = view_of(view);
    let sep = exe::ptr::<u8>(SEP_LIT_VA) as u64;
    let (b, e) = first_component(p, n, sep);
    let o = out as usize;
    for (k, v) in [p, n, b, e].into_iter().enumerate() {
        exe::wr::<u64>(o + k * 8, v);
    }
    let mut b = b;
    loop {
        if b == u64::MAX {
            return out;
        }
        let (op, ol, oe) = (rd::<u64>(o), rd::<u64>(o + 8), rd::<u64>(o + 24));
        if ol < b {
            exe::call_va!(XOUT_OF_RANGE_VA, ());
            unreachable!("_Xout_of_range returned");
        }
        let comp = StrView { ptr: (op + b) as *const u8, len: (ol - b).min(oe.wrapping_sub(b)) };
        if path_component_is_data(&comp) {
            return out;
        }
        b = stl::find_first_not_of_raw(op, ol, sep, 2, oe);
        exe::wr::<u64>(o + 16, b);
        exe::wr::<u64>(o + 24, path_find_sep(out as *const StrView, b));
    }
}

/// A local `r::InplaceString<N>` laid out like the original's stack objects.
#[repr(C, align(8))]
struct LocalStr<const N: usize>([u8; N]);

/// Resource hash of a file path: `CityHash64(lower(backslash(normalize(rel))))`.
/// `rel` is the part of `path` after its first `data*` component, with leading
/// and trailing separators stripped. Writes the hash to `*out` and returns
/// `out`. The hash is 0 if there is no `data*` component or nothing follows it.
///
/// Original: 0x142423da0 (1999 bytes, 19 callers; 4.6% of the loading thread's
/// samples, plus the callees it pulls through the interpreter). Steps, from the
/// disassembly:
/// 1. `it = find_data_component(path)` (0x1413a2490). If `it` is at the end, the hash is 0.
/// 2. `b = find_first_not_of(it.view, seps_view@0x143079458, it.end)`. If that is npos,
///    the hash is 0. Otherwise `tail = it.view.substr(b)` (out_of_range check), then
///    strip trailing separators (0x1401df850).
/// 3. `A`, an `InplaceString<16>` (header 0x02000002), gets `A /= tail` (path_append
///    0x1401e01e0). Then every `/` in `A` becomes `\`.
/// 4. `B`, an `InplaceString<256>` (header 0x20000020), gets `assign(A, 0, A.len)`
///    (0x142409b50). Then each byte is mapped through the to-lower table at 0x1427e54e0.
/// 5. `*out = CityHash64(B)`, and B then A are destroyed (0x14240ab10).
///
/// The unwind map destroys `A` (and `B` once it exists) if path_append or assign
/// throws. That is replicated with [`exe::try_call`]: destroy, then rethrow.
pub unsafe extern "win64" fn path_resource_hash(out: *mut u64, path: *const StrView) -> *mut u64 {
    let input = StrView { ptr: rd::<u64>(path as usize) as *const u8, len: rd::<u64>(path as usize + 8) };
    let mut it = [0u64; 4];
    path_find_data_component(it.as_mut_ptr(), &input);
    if it[2] == u64::MAX {
        exe::wr::<u64>(out as usize, 0);
        return out;
    }
    let sv = exe::ptr::<u8>(SEPS_VIEW_VA) as usize;
    let seps = StrView { ptr: rd::<u64>(sv) as *const u8, len: rd::<u64>(sv + 8) };
    let b = stl::traits_find_first_not_of(it.as_ptr() as *const StrView, &seps, it[3]);
    if b == u64::MAX {
        exe::wr::<u64>(out as usize, 0);
        return out;
    }
    if it[1] < b {
        exe::call_va!(XOUT_OF_RANGE_VA, ());
        unreachable!("_Xout_of_range returned");
    }
    let tail = StrView { ptr: (it[0] + b) as *const u8, len: it[1] - b };
    path_strip_trailing_seps(it.as_mut_ptr() as *mut StrView, &tail);
    let stripped = StrView { ptr: it[0] as *const u8, len: it[1] };

    let mut a_buf = LocalStr::<{ 8 + 16 + 8 }>([0; 8 + 16 + 8]);
    let a = a_buf.0.as_mut_ptr() as *mut InplaceString;
    exe::wr::<u32>(a as usize, 0x0200_0002);
    exe::wr::<u32>(a as usize + 4, 0);
    exe::wr::<u8>(a as usize + 8, 0);
    if let Err(e) = exe::try_call(
        exe::ptr::<u8>(PATH_APPEND_VA) as usize,
        &[a as u64, &stripped as *const StrView as u64],
        &[],
    ) {
        exe::call_va!(STRING_DTOR_VA, (a));
        e.rethrow();
    }

    let mut b_buf = LocalStr::<{ 8 + 256 + 8 }>([0; 8 + 256 + 8]);
    let bs = b_buf.0.as_mut_ptr() as *mut InplaceString;
    exe::wr::<u64>(bs as usize, 0x2000_0020);
    exe::wr::<u8>(bs as usize + 8, 0);

    let ad = storage(a) as usize;
    let al = len_of(a) as usize;
    for i in 0..al {
        if rd::<u8>(ad + i) == b'/' {
            exe::wr::<u8>(ad + i, b'\\');
        }
    }
    let av = StrView { ptr: storage(a), len: len_of(a) as u64 };
    if let Err(e) = exe::try_call(
        exe::ptr::<u8>(ASSIGN_RANGE_VA) as usize,
        &[bs as u64, &av as *const StrView as u64, 0, len_of(a) as u64],
        &[],
    ) {
        exe::call_va!(STRING_DTOR_VA, (bs));
        exe::call_va!(STRING_DTOR_VA, (a));
        e.rethrow();
    }

    let lower = exe::ptr::<u8>(LOWER_VA) as usize;
    let bl = len_of(bs) as u64;
    let bd = storage(bs) as usize;
    for i in 0..bl as usize {
        exe::wr::<u8>(bd + i, rd::<u8>(lower + rd::<u8>(bd + i) as usize));
    }
    let h = super::city_hash::city_hash64(storage(bs) as usize, bl);
    exe::wr::<u64>(out as usize, h);
    exe::call_va!(STRING_DTOR_VA, (bs));
    exe::call_va!(STRING_DTOR_VA, (a));
    out
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

use crate::difftest::Rng;


/// Builds `r::InplaceString<128>` paths (inline, and some forced to the heap by
/// long prefixes through the real reserve) and appends random paths made of
/// `a`, `B`, `.`, `..`, `/`, `\`, `C:` pieces to both copies.
pub unsafe fn diff_path_append(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const StrView) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let pieces: [&[u8]; 12] = [b"a", b"Bc", b".", b"..", b"/", b"\\", b"//", b"C:", b"x.y", b"..a", b"d", b"\\.."];
    let mut r = Rng::new(0x1401e01e0);
    const CASES: u32 = 1500;
    for i in 0..CASES {
        let mk = |r: &mut Rng, k: usize| -> Vec<u8> {
            let mut v = Vec::new();
            for _ in 0..k {
                v.extend_from_slice(pieces[r.below(pieces.len() as u64) as usize]);
            }
            v
        };
        let (ni, na) = (r.below(6) as usize, r.below(7) as usize);
        let init = mk(&mut r, ni);
        let add = mk(&mut r, na);
        // Two identical 128-inline strings (header + 128 bytes + slack), every
        // other case at a 4-byte offset (embedded strings are only 4-aligned).
        let off = (i % 2) as usize * 4;
        let mut bo = vec![0u8; off + 8 + 128 + 16];
        let init_str = |b: &mut Vec<u8>| {
            let s = b.as_mut_ptr().add(off);
            crate::exe::wr::<u32>(s as usize, 0x1000_0010); // capacity 16 units, inline code 0x10
            crate::exe::wr::<u32>(s as usize + 4, init.len() as u32);
            std::ptr::copy_nonoverlapping(init.as_ptr(), s.add(8), init.len());
            *s.add(8 + init.len()) = 0;
        };
        init_str(&mut bo);
        let mut bn = bo.clone();
        let view = StrView { ptr: add.as_ptr(), len: add.len() as u64 };
        let so = bo.as_mut_ptr().add(off) as *mut InplaceString;
        let sn = bn.as_mut_ptr().add(off) as *mut InplaceString;
        let ro = o(so, &view);
        let rn = n(sn, &view);
        if (ro as usize).wrapping_sub(bo.as_ptr() as usize) != (rn as usize).wrapping_sub(bn.as_ptr() as usize) {
            return Err(format!("case {i}: return value differs"));
        }
        let read = |s: *mut InplaceString| -> (u32, Vec<u8>) {
            let l = len_of(s);
            let d = storage(s);
            (l, std::slice::from_raw_parts(d, l as usize + 1).to_vec())
        };
        let (a, b) = (read(so), read(sn));
        if a != b || rd::<u32>(so as usize) != rd::<u32>(sn as usize) {
            return Err(format!(
                "mismatch: case {i}: init {:?} + {:?}: orig {:?} new {:?}",
                String::from_utf8_lossy(&init),
                String::from_utf8_lossy(&add),
                String::from_utf8_lossy(&a.1),
                String::from_utf8_lossy(&b.1)
            ));
        }
        // Results past the 128-byte inline buffer would have gone through the real
        // engine reserve; keep inputs short enough that they don't.
        debug_assert!(a.0 < 128);
    }
    Ok(CASES)
}

/// Random path text from separator/dot/drive/case pieces (shared by the view tests).
fn rand_path(r: &mut Rng, max_pieces: u64) -> Vec<u8> {
    let pieces: [&[u8]; 14] =
        [b"a", b"A", b"Bc", b"bC", b".", b"..", b"/", b"\\", b"//", b"C:", b"x.y", b"..a", b"\xe4", b"\xc4"];
    let k = r.below(max_pieces + 1);
    let mut v = Vec::new();
    for _ in 0..k {
        v.extend_from_slice(pieces[r.below(pieces.len() as u64) as usize]);
    }
    v
}

/// Writes a `StrView` header for `s` at `buf+off` (`off` 0 or 4 for under-aligned cases).
unsafe fn put_view(buf: &mut [u8], off: usize, s: &[u8]) -> *mut StrView {
    let p = buf.as_mut_ptr().add(off);
    exe::wr::<u64>(p as usize, s.as_ptr() as u64);
    exe::wr::<u64>(p as usize + 8, s.len() as u64);
    p as *mut StrView
}

/// `(out*, view*) -> out*` / `(this*, out*) -> out*` view functions: compares
/// the returned pointer (relative to the buffer) and the 16 output bytes. Also
/// runs the in-place (`out == view`) call when `inplace` is set.
unsafe fn diff_view_fn(orig: usize, new: usize, seed: u64, this_first: bool, inplace: bool) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut StrView, *mut StrView) -> *mut StrView;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(seed);
    const CASES: u32 = 3000;
    for i in 0..CASES {
        let s = rand_path(&mut r, 6);
        let off = (i % 2) as usize * 4;
        let same = inplace && i % 3 == 0;
        let run = |f: F| -> (usize, Vec<u8>) {
            let mut buf = vec![0xccu8; 48];
            let v = put_view(&mut buf, off, &s);
            let out = if same { v } else { buf.as_mut_ptr().add(off + 24) as *mut StrView };
            let ret = if this_first { f(v, out) } else { f(out, v) };
            ((ret as usize).wrapping_sub(buf.as_ptr() as usize), buf.clone())
        };
        let (a, b) = (run(o), run(n));
        if a != b {
            return Err(format!("case {i}: {:?} (inplace {same}): orig {:?} new {:?}", String::from_utf8_lossy(&s), a, b));
        }
    }
    Ok(CASES)
}

pub unsafe fn diff_path_strip_trailing_seps(orig: usize, new: usize) -> Result<u32, String> {
    diff_view_fn(orig, new, 0x1401df850, false, true)
}

pub unsafe fn diff_path_filename(orig: usize, new: usize) -> Result<u32, String> {
    diff_view_fn(orig, new, 0x1401dfa80, true, false)
}

/// Makes an inline `r::InplaceString<128>` holding `init` at `buf+off`.
unsafe fn put_inplace(buf: &mut [u8], off: usize, init: &[u8]) -> *mut InplaceString {
    let s = buf.as_mut_ptr().add(off);
    exe::wr::<u32>(s as usize, 0x1000_0010); // capacity 16 units, inline code 0x10
    exe::wr::<u32>(s as usize + 4, init.len() as u32);
    std::ptr::copy_nonoverlapping(init.as_ptr(), s.add(8), init.len());
    *s.add(8 + init.len()) = 0;
    s as *mut InplaceString
}

/// Pops from random paths with `guard` both ways; compares `al` and the whole
/// string buffer (header, bytes, terminator).
pub unsafe fn diff_path_pop_component(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u8) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e00a0);
    const CASES: u32 = 3000;
    for i in 0..CASES {
        let init = rand_path(&mut r, 8);
        let guard = (r.below(2) as u8) * (1 + r.below(200) as u8);
        let off = (i % 2) as usize * 4;
        let run = |f: F| -> (u8, Vec<u8>) {
            let mut buf = vec![0u8; off + 8 + 128 + 16];
            let s = put_inplace(&mut buf, off, &init);
            let ret = f(s, guard) as u8;
            (ret, buf.clone())
        };
        let (a, b) = (run(o), run(n));
        if a != b {
            return Err(format!(
                "case {i}: {:?} guard {guard}: orig al={} new al={} (buffers {})",
                String::from_utf8_lossy(&init),
                a.0,
                b.0,
                if a.1 == b.1 { "equal" } else { "differ" }
            ));
        }
    }
    Ok(CASES)
}

/// Random path pairs, often sharing a prefix (mixed case and separators) so
/// the mismatch runs past the first component. Compares all 64 output bytes
/// and the returned pointer.
unsafe fn diff_mismatch(orig: usize, new: usize, seed: u64, b_is_str: bool) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const StrView, *const u8) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(seed);
    const CASES: u32 = 3000;
    for i in 0..CASES {
        let common = rand_path(&mut r, 5);
        let mut a = common.clone();
        a.extend(rand_path(&mut r, 3));
        let mut b: Vec<u8> = common
            .iter()
            .map(|&c| match (c, r.below(4)) {
                (b'a', 0) => b'A',
                (b'/', 0) => b'\\',
                (b'\\', 0) => b'/',
                (c, _) => c,
            })
            .collect();
        b.extend(rand_path(&mut r, 3));
        let off = (i % 2) as usize * 4;
        let run = |f: F| -> (usize, Vec<u8>) {
            let mut vbuf = vec![0u8; 32];
            let av = put_view(&mut vbuf, off, &a);
            let mut bbuf = vec![0u8; off + 8 + 128 + 16];
            let bp: *const u8 = if b_is_str {
                put_inplace(&mut bbuf, off, &b) as *const u8
            } else {
                put_view(&mut bbuf, off, &b) as *const u8
            };
            let mut out = vec![0xccu8; 64 + 8];
            let op = out.as_mut_ptr().add(off);
            let ret = f(op, av, bp);
            // Component pointers point into `a`/`b` (shared) or the string
            // buffer (per run): store the latter relative to the buffer.
            let mut words: Vec<u64> = (0..8).map(|k| exe::rd::<u64>(op as usize + k * 8)).collect();
            if b_is_str {
                words[4] = words[4].wrapping_sub(bbuf.as_ptr() as u64);
            }
            let mut flat = Vec::new();
            for w in words {
                flat.extend_from_slice(&w.to_le_bytes());
            }
            ((ret as usize).wrapping_sub(out.as_ptr() as usize), flat)
        };
        let (x, y) = (run(o), run(n));
        if x != y {
            return Err(format!(
                "case {i}: a {:?} b {:?}: orig {:?} new {:?}",
                String::from_utf8_lossy(&a),
                String::from_utf8_lossy(&b),
                x,
                y
            ));
        }
    }
    Ok(CASES)
}

pub unsafe fn diff_path_mismatch_view_str(orig: usize, new: usize) -> Result<u32, String> {
    diff_mismatch(orig, new, 0x1413a7e00, true)
}

pub unsafe fn diff_path_mismatch_view_view(orig: usize, new: usize) -> Result<u32, String> {
    diff_mismatch(orig, new, 0x1401f5ea0, false)
}

/// Random resource-like paths: components (with `data` look-alikes, dots,
/// drive roots, mixed case, high bytes, long names) joined by random separators.
fn rand_resource_path(r: &mut Rng, max_comps: u64) -> Vec<u8> {
    let comps: [&[u8]; 16] = [
        b"data", b"Data", b"DATA", b"datax", b"dat", b"x", b"Foo", b"bar.bin", b".", b"..", b"C:", b"\xe4\xc4",
        b"aBcDeFgHiJkLmNoP", b"textures", b"Level_01", b"d",
    ];
    let seps: [&[u8]; 4] = [b"/", b"\\", b"//", b"/\\"];
    let mut v = Vec::new();
    if r.below(4) == 0 {
        v.extend_from_slice(seps[r.below(4) as usize]);
    }
    let k = r.below(max_comps + 1);
    for i in 0..k {
        if i > 0 {
            v.extend_from_slice(seps[r.below(4) as usize]);
        }
        v.extend_from_slice(comps[r.below(comps.len() as u64) as usize]);
    }
    if r.below(4) == 0 {
        v.extend_from_slice(seps[r.below(4) as usize]);
    }
    v
}

pub unsafe fn diff_path_find_sep(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const StrView, u64) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401dfdf0);
    const CASES: u32 = 3000;
    for i in 0..CASES {
        let s = rand_path(&mut r, 6);
        let pos = if r.below(8) == 0 { u64::MAX } else { r.below(s.len() as u64 + 2) };
        let mut buf = vec![0u8; 24];
        let v = put_view(&mut buf, (i % 2) as usize * 4, &s);
        let (a, b) = (o(v, pos), n(v, pos));
        if a != b {
            return Err(format!("case {i}: {:?} pos {pos}: orig {a:#x} new {b:#x}", String::from_utf8_lossy(&s)));
        }
    }
    Ok(CASES)
}

pub unsafe fn diff_path_component_is_data(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const StrView) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x142423740);
    let alpha = b"dDaAtTx/\xe4";
    const CASES: u32 = 3000;
    for i in 0..CASES {
        let len = r.below(7) as usize;
        let s: Vec<u8> = if r.below(2) == 0 {
            // Mostly near-misses of "data".
            let mut s = b"data".to_vec();
            s.truncate(len.min(4));
            for c in s.iter_mut() {
                if r.below(3) == 0 {
                    *c = alpha[r.below(alpha.len() as u64) as usize];
                }
            }
            s.extend((4..len).map(|_| alpha[r.below(alpha.len() as u64) as usize]));
            s
        } else {
            (0..len).map(|_| alpha[r.below(alpha.len() as u64) as usize]).collect()
        };
        let mut buf = vec![0u8; 24];
        let v = put_view(&mut buf, (i % 2) as usize * 4, &s);
        let (a, b) = (o(v) as u8, n(v) as u8);
        if a != b {
            return Err(format!("case {i}: {:?}: orig {a} new {b}", String::from_utf8_lossy(&s)));
        }
    }
    Ok(CASES)
}

pub unsafe fn diff_path_find_data_component(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u64, *const StrView) -> *mut u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1413a2490);
    const CASES: u32 = 3000;
    for i in 0..CASES {
        let s = rand_resource_path(&mut r, 8);
        let off = (i % 2) as usize * 4;
        let run = |f: F| -> (usize, Vec<u8>) {
            let mut vbuf = vec![0u8; 24];
            let v = put_view(&mut vbuf, off, &s);
            let mut out = vec![0xccu8; 40];
            let op = out.as_mut_ptr().add(off) as *mut u64;
            let ret = f(op, v);
            ((ret as usize).wrapping_sub(out.as_ptr() as usize), out)
        };
        let (a, b) = (run(o), run(n));
        if a != b {
            return Err(format!("case {i}: {:?}: orig {:?} new {:?}", String::from_utf8_lossy(&s), a, b));
        }
    }
    Ok(CASES)
}

use std::cell::RefCell;
thread_local! {
    /// Buffers handed out by [`test_reserve`] (kept alive for the thread).
    static TEST_BUFS: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

/// Test stand-in for `reserve(s, n)` (0x142408f50), not the engine's growth
/// policy: no-op if `n + 1` fits, else moves the string into a fresh 0xCD-filled
/// heap buffer of `max(round_up(n + 1, 16), 512)` bytes (always larger than any
/// inline area here). Both sides see the same callee.
unsafe extern "win64" fn test_reserve(s: *mut InplaceString, n: u64) {
    let need = n + 1;
    if need <= capacity(s) {
        return;
    }
    let size = ((need + 15) & !15).max(512) as usize;
    let mut buf = vec![0xCDu8; size];
    std::ptr::copy_nonoverlapping(storage(s), buf.as_mut_ptr(), len_of(s) as usize + 1);
    exe::wr::<u64>(s as usize + 8, buf.as_mut_ptr() as u64);
    let bits = rd::<u32>(s as usize);
    exe::wr::<u32>(s as usize, (bits & 0xff00_0000) | (size as u32 >> 3));
    TEST_BUFS.with(|v| v.borrow_mut().push(buf));
}

/// Test stand-in for `~InplaceString` (the fixture buffers are freed with the thread).
unsafe extern "win64" fn test_string_dtor(_s: *mut InplaceString) {}

/// Hashes random resource paths, from empty up to ~200 chars after
/// normalization. That covers every CityHash64 length class (0-16, 17-32,
/// 33-64, and the 64-byte block loop), `A` growing past its 16 inline bytes,
/// and the no-`data` / nothing-after-`data` zero results. `reserve` and the
/// string dtor are mocked (see the fixtures above). Compares `*out` and the
/// returned pointer.
pub unsafe fn diff_path_resource_hash(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u64, *const StrView) -> *mut u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _reserve = crate::difftest::Mock::new(RESERVE_VA, test_reserve as *const ())?;
    let _dtor = crate::difftest::Mock::new(STRING_DTOR_VA, test_string_dtor as *const ())?;
    let mut r = Rng::new(0x142423da0);
    const CASES: u32 = 2000;
    let mut nonzero = 0u32;
    for i in 0..CASES {
        let mut s = rand_resource_path(&mut r, 12);
        if r.below(2) == 0 {
            // Force a data root most of the time.
            let mut p = b"C:/Games/AW2/data/".to_vec();
            p.extend_from_slice(&s);
            s = p;
        }
        s.truncate(220);
        let off = (i % 2) as usize * 4;
        let run = |f: F| -> (usize, u64) {
            let mut vbuf = vec![0u8; 24];
            let v = put_view(&mut vbuf, off, &s);
            let mut out = vec![0xccu8; 16];
            let op = out.as_mut_ptr().add(off) as *mut u64;
            let ret = f(op, v);
            ((ret as usize).wrapping_sub(out.as_ptr() as usize), rd::<u64>(op as usize))
        };
        let (a, b) = (run(o), run(n));
        if a != b {
            return Err(format!("case {i}: {:?}: orig {:x?} new {:x?}", String::from_utf8_lossy(&s), a, b));
        }
        nonzero += (a.1 != 0) as u32;
    }
    if nonzero < CASES / 4 {
        return Err(format!("only {nonzero} of {CASES} cases produced a non-zero hash; generator too weak"));
    }
    Ok(CASES)
}
