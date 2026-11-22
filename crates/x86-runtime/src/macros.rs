//! Exported runtime macros (`diff_eq!`, `call_va!`, `call_native!`, `call_iat_native!`, `call_iat!`).
//!
//! Defined outside `win64_abi::win64_mods!` so `#[macro_export]` macros can be re-exported
//! cleanly from submodules and downstream game crates without triggering Rust's
//! `macro_expanded_macro_exports_accessed_by_absolute_paths` lint.

/// Compare two values; on mismatch return a formatted error from the enclosing fn.
#[macro_export]
macro_rules! diff_eq {
    ($o:expr, $n:expr, $($ctx:tt)+) => {{
        let (o, n) = (&$o, &$n);
        if o != n {
            return Err(format!("mismatch: orig={:?} new={:?} for {}", o, n, format!($($ctx)+)));
        }
    }};
}

/// Dispatch a call to `va_or_addr` (preferred-base VA, mapped address, or vtable slot)
/// with 0..12 arguments via `exe::call_va8` / `call_va10` / `call_va12`.
#[macro_export]
macro_rules! call_va {
    ($va:expr, ($($arg:expr),* $(,)?) -> $ret:ty) => {
        <$ret as $crate::exe::FromWin64Ret>::from_win64_ret(
            $crate::call_va!($va $(, $arg)*)
        )
    };
    ($va:expr, ($($arg:expr),* $(,)?)) => {{
        let _ = $crate::call_va!($va $(, $arg)*);
    }};
    ($va:expr $(,)?) => {
        $crate::exe::call_va8(($va) as u64, 0, 0, 0, 0, 0, 0, 0, 0)
    };
    ($va:expr, $a0:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            0, 0, 0, 0, 0, 0, 0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            0, 0, 0, 0, 0, 0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            0, 0, 0, 0, 0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            0, 0, 0, 0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            0, 0, 0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            $crate::exe::IntoWin64Arg::into_win64_arg($a5),
            0, 0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            $crate::exe::IntoWin64Arg::into_win64_arg($a5),
            $crate::exe::IntoWin64Arg::into_win64_arg($a6),
            0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr $(,)?) => {
        $crate::exe::call_va8(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            $crate::exe::IntoWin64Arg::into_win64_arg($a5),
            $crate::exe::IntoWin64Arg::into_win64_arg($a6),
            $crate::exe::IntoWin64Arg::into_win64_arg($a7),
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr $(,)?) => {
        $crate::exe::call_va10(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            $crate::exe::IntoWin64Arg::into_win64_arg($a5),
            $crate::exe::IntoWin64Arg::into_win64_arg($a6),
            $crate::exe::IntoWin64Arg::into_win64_arg($a7),
            $crate::exe::IntoWin64Arg::into_win64_arg($a8),
            0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr $(,)?) => {
        $crate::exe::call_va10(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            $crate::exe::IntoWin64Arg::into_win64_arg($a5),
            $crate::exe::IntoWin64Arg::into_win64_arg($a6),
            $crate::exe::IntoWin64Arg::into_win64_arg($a7),
            $crate::exe::IntoWin64Arg::into_win64_arg($a8),
            $crate::exe::IntoWin64Arg::into_win64_arg($a9),
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr $(,)?) => {
        $crate::exe::call_va12(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            $crate::exe::IntoWin64Arg::into_win64_arg($a5),
            $crate::exe::IntoWin64Arg::into_win64_arg($a6),
            $crate::exe::IntoWin64Arg::into_win64_arg($a7),
            $crate::exe::IntoWin64Arg::into_win64_arg($a8),
            $crate::exe::IntoWin64Arg::into_win64_arg($a9),
            $crate::exe::IntoWin64Arg::into_win64_arg($a10),
            0,
        )
    };
    ($va:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr $(,)?) => {
        $crate::exe::call_va12(
            ($va) as u64,
            $crate::exe::IntoWin64Arg::into_win64_arg($a0),
            $crate::exe::IntoWin64Arg::into_win64_arg($a1),
            $crate::exe::IntoWin64Arg::into_win64_arg($a2),
            $crate::exe::IntoWin64Arg::into_win64_arg($a3),
            $crate::exe::IntoWin64Arg::into_win64_arg($a4),
            $crate::exe::IntoWin64Arg::into_win64_arg($a5),
            $crate::exe::IntoWin64Arg::into_win64_arg($a6),
            $crate::exe::IntoWin64Arg::into_win64_arg($a7),
            $crate::exe::IntoWin64Arg::into_win64_arg($a8),
            $crate::exe::IntoWin64Arg::into_win64_arg($a9),
            $crate::exe::IntoWin64Arg::into_win64_arg($a10),
            $crate::exe::IntoWin64Arg::into_win64_arg($a11),
        )
    };
}

/// `call_va!` to a VA whose registered replacement is `$f`, bypassing the Win64 bridge
/// when the dispatch cache confirms `$va` still points at `$f`.
#[macro_export]
macro_rules! call_native {
    ($va:expr, $f:path, ($($arg:expr),* $(,)?) -> $ret:ty) => {{
        if $crate::emu::resolve_dispatch(($va) as u64).0 == ($f as *const () as usize) {
            $f($($arg),*)
        } else {
            $crate::call_va!($va, ($($arg),*) -> $ret)
        }
    }};
}

/// `call_iat!` to an import whose slot normally holds native builtin `$f`.
#[macro_export]
macro_rules! call_iat_native {
    ($iat_va:expr, $f:path, ($($arg:expr),* $(,)?) -> $ret:ty) => {{
        let fn_addr = $crate::exe::rd::<u64>($crate::exe::ptr::<u64>($iat_va) as usize);
        if fn_addr as usize == ($f as *const () as usize) {
            $f($($arg),*)
        } else {
            $crate::call_va!(fn_addr, ($($arg),*) -> $ret)
        }
    }};
}

/// Read a function pointer from the IAT slot at `iat_va` and dispatch a call via [`call_va!`].
#[macro_export]
macro_rules! call_iat {
    ($iat_va:expr, ($($arg:expr),* $(,)?) -> $ret:ty) => {{
        let fn_addr = $crate::exe::rd::<u64>($crate::exe::ptr::<u64>($iat_va) as usize);
        $crate::call_va!(fn_addr, ($($arg),*) -> $ret)
    }};
    ($iat_va:expr, ($($arg:expr),* $(,)?)) => {{
        let fn_addr = $crate::exe::rd::<u64>($crate::exe::ptr::<u64>($iat_va) as usize);
        $crate::call_va!(fn_addr, ($($arg),*))
    }};
    ($iat_va:expr $(, $arg:expr)* $(,)?) => {{
        let fn_addr = $crate::exe::rd::<u64>($crate::exe::ptr::<u64>($iat_va) as usize);
        $crate::call_va!(fn_addr $(, $arg)*)
    }};
}
