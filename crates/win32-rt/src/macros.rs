//! Helper macros: `stub!`, `exports!`, `fn_name!`.

/// Name of the enclosing function (`win32_rt::kernel32::GetFoo`).
#[macro_export]
macro_rules! fn_name {
    () => {{
        fn f() {}
        let n = ::std::any::type_name_of_val(&f);
        let n = n.strip_suffix("::f").unwrap_or(n);
        n.rsplit("::").next().unwrap_or(n)
    }};
}

/// Internal helper used by [`stub!`] and [`failing!`]. Records the hit in
/// [`crate::stub_registry`] and logs the first 3 hits at `warn`, later ones at `trace`.
#[doc(hidden)]
#[macro_export]
macro_rules! __record_stub {
    ($kind:expr, $api:expr, $($arg:tt)*) => {{
        static SITE: $crate::stub_registry::StubSite = $crate::stub_registry::StubSite::new(
            $kind,
            module_path!(),
            file!(),
            line!(),
        );
        let api: &'static str = $api;
        let n = SITE.record_hit(
            api,
            || ::std::fmt::format(format_args!($($arg)*)),
        );
        if n < 3 {
            ::log::warn!(target: "stub", "STUB {}: {}", api, format_args!($($arg)*));
        } else {
            ::log::trace!(target: "stub", "STUB {}: {}", api, format_args!($($arg)*));
        }
    }};
}

/// Log that the enclosing API is only partially implemented / faked and record
/// the hit in the global [`crate::stub_registry`].
///
/// The first few hits per call site are logged at `warn` (target `stub`),
/// later ones at `trace`, so `grep STUB` finds every fake behaviour and the
/// exit hook prints a ranked summary table.
#[macro_export]
macro_rules! stub {
    ($($arg:tt)*) => {{
        $crate::__record_stub!(
            $crate::stub_registry::StubKind::Stub,
            $crate::fn_name!(),
            $($arg)*
        );
    }};
}

/// Build a `&'static [Export]` from function idents and optional
/// `"literal name" => pointer-expr` pairs (for C++-mangled / data exports).
#[macro_export]
macro_rules! exports {
    ($($name:ident),* $(,)? $(; $($lit:literal => $e:expr),* $(,)?)?) => {
        &[
            $( $crate::Export { name: stringify!($name), ptr: $crate::Ptr($name as *const ()) }, )*
            $( $( $crate::Export { name: $lit, ptr: $crate::Ptr($e as *const ()) }, )* )?
        ]
    };
}

/// Honest failing stubs: each function logs `STUB name -> <ret>`, sets the
/// thread's last error (when `err` is non-zero) and returns `val`.
/// Automatically exported by `crates/win32-rt/build.rs` and `port-factory win32-sync`
/// without needing a separate `// @exports` line.
#[macro_export]
macro_rules! failing {
    ($ret:ty = $val:expr, err $err:expr; $($name:ident),* $(,)?) => {$(
        #[cfg(target_arch = "x86_64")]
        pub unsafe extern "win64" fn $name() -> $ret {
            let e: u32 = $err;
            if e != 0 {
                $crate::teb::set_last_error(e);
            }
            $crate::__record_stub!(
                $crate::stub_registry::StubKind::Failing,
                stringify!($name),
                "{} -> {} (last error {})",
                stringify!($name),
                stringify!($val),
                e
            );
            $val
        }
        #[cfg(not(target_arch = "x86_64"))]
        pub unsafe extern "C" fn $name() -> $ret {
            let e: u32 = $err;
            if e != 0 {
                $crate::teb::set_last_error(e);
            }
            $crate::__record_stub!(
                $crate::stub_registry::StubKind::Failing,
                stringify!($name),
                "{} -> {} (last error {})",
                stringify!($name),
                stringify!($val),
                e
            );
            $val
        }
    )*};
}

/// Concise declarative Win64 stub macro:
/// - `win64_stubs!(Foo, Bar, Baz);` -> returns `0usize`
/// - `win64_stubs!(i32 = -1; Foo, Bar);` -> returns custom value
/// Automatically exported by `build.rs` and `port-factory win32-sync`.
#[macro_export]
macro_rules! win64_stubs {
    ($($name:ident),* $(,)?) => {
        $crate::failing!(usize = 0, err 0; $($name),*);
    };
    ($ret:ty = $val:expr; $($name:ident),* $(,)?) => {
        $crate::failing!($ret = $val, err 0; $($name),*);
    };
}

/// Generates the aarch64 host-bridge entry `$f::bridge` for a natively
/// implemented Win64 function `$f` whose by-value arguments mix integers and
/// floats. See [`crate::bridge`] for why such functions can't be called
/// through the interpreter's positional bridge directly. Repeat the function's
/// parameter types exactly:
///
/// ```ignore
/// win64_bridge!(fn set_rtpc(u32, f32, u64, i32, i32, bool) -> i32);
/// hook!(T_SET_RTPC, bridge_entry!(set_rtpc), "...");
/// ```
///
/// Off aarch64 this expands to nothing and `bridge_entry!(f)` is `f`.
#[macro_export]
macro_rules! win64_bridge {
    (fn $f:ident($($t:ty),* $(,)?) -> $r:ty) => {
        #[cfg(target_arch = "aarch64")]
        #[allow(non_snake_case, dead_code)]
        pub(crate) mod $f {
            #[allow(unused_imports)]
            use super::*;
            /// aarch64 host-bridge entry (all 16 Win64 slots raw + xmm0..3).
            #[allow(clippy::too_many_arguments)]
            pub(crate) unsafe extern "C" fn bridge(
                s1: u64, s2: u64, s3: u64, s4: u64, s5: u64, s6: u64, s7: u64, s8: u64,
                s9: u64, s10: u64, s11: u64, s12: u64, s13: u64, s14: u64, s15: u64, s16: u64,
                x0: f64, x1: f64, x2: f64, x3: f64,
            ) -> $r {
                let mut s = $crate::bridge::Slots::new(
                    [s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16],
                    [x0.to_bits(), x1.to_bits(), x2.to_bits(), x3.to_bits()],
                );
                #[allow(unused_unsafe)]
                unsafe { super::$f($(s.next::<$t>()),*) }
            }
        }
    };
}

/// The guest-callable entry for a function declared with [`win64_bridge!`]:
/// `$f::bridge` on aarch64, `$f` elsewhere.
#[macro_export]
macro_rules! bridge_entry {
    ($f:ident) => {{
        #[cfg(target_arch = "aarch64")]
        let p = $f::bridge as *const ();
        #[cfg(not(target_arch = "aarch64"))]
        let p = $f as *const ();
        p
    }};
}
