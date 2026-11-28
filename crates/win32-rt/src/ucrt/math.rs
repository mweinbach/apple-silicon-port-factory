//! ucrt math: libm wrappers (host libSystem libm / Rust std) and the MSVC
//! float classification helpers. `long double` == `double` on MSVC x64.

/// Host libm (names re-bound so they don't clash with our exports).
mod m {
    extern "C" {
        #[link_name = "frexp"]
        pub fn frexp(x: f64, e: *mut i32) -> f64;
        #[link_name = "frexpf"]
        pub fn frexpf(x: f32, e: *mut i32) -> f32;
        #[link_name = "ldexp"]
        pub fn ldexp(x: f64, e: i32) -> f64;
        #[link_name = "ldexpf"]
        pub fn ldexpf(x: f32, e: i32) -> f32;
        #[link_name = "modf"]
        pub fn modf(x: f64, i: *mut f64) -> f64;
        #[link_name = "modff"]
        pub fn modff(x: f32, i: *mut f32) -> f32;
        #[link_name = "nearbyint"]
        pub fn nearbyint(x: f64) -> f64;
        #[link_name = "nearbyintf"]
        pub fn nearbyintf(x: f32) -> f32;
        #[link_name = "rint"]
        pub fn rint(x: f64) -> f64;
        #[link_name = "rintf"]
        pub fn rintf(x: f32) -> f32;
        #[link_name = "ilogb"]
        pub fn ilogb(x: f64) -> i32;
        #[link_name = "ilogbf"]
        pub fn ilogbf(x: f32) -> i32;
        #[link_name = "logb"]
        pub fn logb(x: f64) -> f64;
        #[link_name = "remainder"]
        pub fn remainder(x: f64, y: f64) -> f64;
        #[link_name = "remainderf"]
        pub fn remainderf(x: f32, y: f32) -> f32;
        #[link_name = "erf"]
        pub fn erf(x: f64) -> f64;
        #[link_name = "erfc"]
        pub fn erfc(x: f64) -> f64;
        #[link_name = "tgamma"]
        pub fn tgamma(x: f64) -> f64;
        #[link_name = "lgamma"]
        pub fn lgamma(x: f64) -> f64;
        #[link_name = "nextafter"]
        pub fn nextafter(x: f64, y: f64) -> f64;
        #[link_name = "nextafterf"]
        pub fn nextafterf(x: f32, y: f32) -> f32;
    }
}

// `#[inline(never)]`: exactly one copy of each builtin, so its address in other crates (the
// emulator's inline-callable tables) equals the export-table pointer (tiny functions are
// otherwise inlinable cross-crate and get a separate copy per crate).
macro_rules! unary {
    ($t:ty; $($name:ident => $f:expr),* $(,)?) => {
        $(#[inline(never)] pub extern "win64" fn $name(x: $t) -> $t { let f: fn($t) -> $t = $f; f(x) })*
    };
}
macro_rules! binary {
    ($t:ty; $($name:ident => $f:expr),* $(,)?) => {
        $(#[inline(never)] pub extern "win64" fn $name(x: $t, y: $t) -> $t { let f: fn($t, $t) -> $t = $f; f(x, y) })*
    };
}

// @exports acos asin atan cos sin tan cosh sinh tanh acosh asinh atanh exp exp2 expm1 log log10 log2 log1p sqrt cbrt ceil floor round trunc fabs
unary!(f64;
    acos => f64::acos, asin => f64::asin, atan => f64::atan,
    cos => f64::cos, sin => f64::sin, tan => f64::tan,
    cosh => f64::cosh, sinh => f64::sinh, tanh => f64::tanh,
    acosh => f64::acosh, asinh => f64::asinh, atanh => f64::atanh,
    exp => f64::exp, exp2 => f64::exp2, expm1 => f64::exp_m1,
    log => f64::ln, log10 => f64::log10, log2 => f64::log2, log1p => f64::ln_1p,
    sqrt => f64::sqrt, cbrt => f64::cbrt,
    ceil => f64::ceil, floor => f64::floor, round => f64::round, trunc => f64::trunc, fabs => f64::abs,
);
// @exports acosf asinf atanf cosf sinf tanf coshf sinhf tanhf expf exp2f expm1f logf log10f log2f log1pf sqrtf cbrtf ceilf floorf roundf truncf fabsf
unary!(f32;
    acosf => f32::acos, asinf => f32::asin, atanf => f32::atan,
    cosf => f32::cos, sinf => f32::sin, tanf => f32::tan,
    coshf => f32::cosh, sinhf => f32::sinh, tanhf => f32::tanh,
    expf => f32::exp, exp2f => f32::exp2, expm1f => f32::exp_m1,
    logf => f32::ln, log10f => f32::log10, log2f => f32::log2, log1pf => f32::ln_1p,
    sqrtf => f32::sqrt, cbrtf => f32::cbrt,
    ceilf => f32::ceil, floorf => f32::floor, roundf => f32::round, truncf => f32::trunc, fabsf => f32::abs,
);
// @exports atan2 pow fmod copysign hypot fmax fmin fdim _hypot _copysign
binary!(f64;
    atan2 => f64::atan2, pow => f64::powf, fmod => |a, b| a % b, copysign => f64::copysign,
    hypot => f64::hypot, fmax => f64::max, fmin => f64::min, fdim => |a, b| if a > b { a - b } else { 0.0 },
    _hypot => f64::hypot, _copysign => f64::copysign,
);
// @exports atan2f powf fmodf copysignf hypotf fmaxf fminf _hypotf _copysignf
binary!(f32;
    atan2f => f32::atan2, powf => f32::powf, fmodf => |a, b| a % b, copysignf => f32::copysign,
    hypotf => f32::hypot, fmaxf => f32::max, fminf => f32::min, _hypotf => f32::hypot, _copysignf => f32::copysign,
);

pub extern "win64" fn fma(x: f64, y: f64, z: f64) -> f64 {
    x.mul_add(y, z)
}
pub extern "win64" fn fmaf(x: f32, y: f32, z: f32) -> f32 {
    x.mul_add(y, z)
}
// Mixed float/integer signatures. On aarch64 the interpreter's host bridge
// passes Win64 argument slot k in x(k-1) and xmm0..3 in v0..v3, ignoring the
// callee's types (`aw2_game::emu::call_host_with_xmm`). AAPCS64 numbers integer
// and float parameters separately, so `f(x: f64, e: i32)` would read `e` from
// x0 = rcx (an unused slot-1 register) instead of x1 = rdx. The aarch64 exports
// therefore declare slot 1 as a dummy integer before the real integer argument;
// `x` is still the first float, v0 = xmm0.
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn frexp(x: f64, e: *mut i32) -> f64 {
    m::frexp(x, e)
}
#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn frexp(_rcx: u64, e: *mut i32, x: f64) -> f64 {
    m::frexp(x, e)
}
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn frexpf(x: f32, e: *mut i32) -> f32 {
    m::frexpf(x, e)
}
#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn frexpf(_rcx: u64, e: *mut i32, x: f32) -> f32 {
    m::frexpf(x, e)
}
#[cfg(not(target_arch = "aarch64"))]
pub extern "win64" fn ldexp(x: f64, e: i32) -> f64 {
    unsafe { m::ldexp(x, e) }
}
#[cfg(target_arch = "aarch64")]
pub extern "win64" fn ldexp(_rcx: u64, e: i32, x: f64) -> f64 {
    unsafe { m::ldexp(x, e) }
}
#[cfg(not(target_arch = "aarch64"))]
pub extern "win64" fn ldexpf(x: f32, e: i32) -> f32 {
    unsafe { m::ldexpf(x, e) }
}
#[cfg(target_arch = "aarch64")]
pub extern "win64" fn ldexpf(_rcx: u64, e: i32, x: f32) -> f32 {
    unsafe { m::ldexpf(x, e) }
}
#[cfg(not(target_arch = "aarch64"))]
pub extern "win64" fn scalbn(x: f64, e: i32) -> f64 {
    unsafe { m::ldexp(x, e) }
}
#[cfg(target_arch = "aarch64")]
pub extern "win64" fn scalbn(_rcx: u64, e: i32, x: f64) -> f64 {
    unsafe { m::ldexp(x, e) }
}
#[cfg(not(target_arch = "aarch64"))]
pub extern "win64" fn scalbnf(x: f32, e: i32) -> f32 {
    unsafe { m::ldexpf(x, e) }
}
#[cfg(target_arch = "aarch64")]
pub extern "win64" fn scalbnf(_rcx: u64, e: i32, x: f32) -> f32 {
    unsafe { m::ldexpf(x, e) }
}
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn modf(x: f64, i: *mut f64) -> f64 {
    m::modf(x, i)
}
#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn modf(_rcx: u64, i: *mut f64, x: f64) -> f64 {
    m::modf(x, i)
}
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn modff(x: f32, i: *mut f32) -> f32 {
    m::modff(x, i)
}
#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn modff(_rcx: u64, i: *mut f32, x: f32) -> f32 {
    m::modff(x, i)
}
pub extern "win64" fn nearbyint(x: f64) -> f64 {
    unsafe { m::nearbyint(x) }
}
pub extern "win64" fn nearbyintf(x: f32) -> f32 {
    unsafe { m::nearbyintf(x) }
}
pub extern "win64" fn rint(x: f64) -> f64 {
    unsafe { m::rint(x) }
}
pub extern "win64" fn rintf(x: f32) -> f32 {
    unsafe { m::rintf(x) }
}
/// `long lrint(double)` — Windows `long` is 32 bit.
pub extern "win64" fn lrint(x: f64) -> i32 {
    unsafe { m::rint(x) as i32 }
}
pub extern "win64" fn lrintf(x: f32) -> i32 {
    unsafe { m::rintf(x) as i32 }
}
pub extern "win64" fn llrint(x: f64) -> i64 {
    unsafe { m::rint(x) as i64 }
}
pub extern "win64" fn llrintf(x: f32) -> i64 {
    unsafe { m::rintf(x) as i64 }
}
pub extern "win64" fn lround(x: f64) -> i32 {
    x.round() as i32
}
pub extern "win64" fn lroundf(x: f32) -> i32 {
    x.round() as i32
}
pub extern "win64" fn llround(x: f64) -> i64 {
    x.round() as i64
}
pub extern "win64" fn llroundf(x: f32) -> i64 {
    x.round() as i64
}
pub extern "win64" fn ilogb(x: f64) -> i32 {
    unsafe { m::ilogb(x) }
}
pub extern "win64" fn ilogbf(x: f32) -> i32 {
    unsafe { m::ilogbf(x) }
}
pub extern "win64" fn logb(x: f64) -> f64 {
    unsafe { m::logb(x) }
}
pub extern "win64" fn remainder(x: f64, y: f64) -> f64 {
    unsafe { m::remainder(x, y) }
}
pub extern "win64" fn remainderf(x: f32, y: f32) -> f32 {
    unsafe { m::remainderf(x, y) }
}
#[inline(never)]
pub extern "win64" fn nextafter(x: f64, y: f64) -> f64 {
    unsafe { m::nextafter(x, y) }
}
#[inline(never)]
pub extern "win64" fn nextafterf(x: f32, y: f32) -> f32 {
    unsafe { m::nextafterf(x, y) }
}
pub extern "win64" fn erf(x: f64) -> f64 {
    unsafe { m::erf(x) }
}
pub extern "win64" fn erfc(x: f64) -> f64 {
    unsafe { m::erfc(x) }
}
pub extern "win64" fn tgamma(x: f64) -> f64 {
    unsafe { m::tgamma(x) }
}
pub extern "win64" fn lgamma(x: f64) -> f64 {
    unsafe { m::lgamma(x) }
}
pub unsafe extern "win64" fn nan(_tag: *const u8) -> f64 {
    f64::NAN
}
pub unsafe extern "win64" fn nanf(_tag: *const u8) -> f32 {
    f32::NAN
}

// ---------------------------------------------------------------------------
// MSVC classification helpers
// ---------------------------------------------------------------------------

const FP_INFINITE: i16 = 1;
const FP_NAN: i16 = 2;
const FP_NORMAL: i16 = -1;
const FP_SUBNORMAL: i16 = -2;
const FP_ZERO: i16 = 0;

fn class64(x: f64) -> i16 {
    use std::num::FpCategory::*;
    match x.classify() {
        Nan => FP_NAN,
        Infinite => FP_INFINITE,
        Zero => FP_ZERO,
        Subnormal => FP_SUBNORMAL,
        Normal => FP_NORMAL,
    }
}
fn class32(x: f32) -> i16 {
    class64(x as f64).max(if x.is_subnormal() { FP_SUBNORMAL } else { i16::MIN })
}

pub extern "win64" fn _dclass(x: f64) -> i16 {
    class64(x)
}
pub extern "win64" fn _ldclass(x: f64) -> i16 {
    class64(x)
}
pub extern "win64" fn _fdclass(x: f32) -> i16 {
    if x.is_subnormal() {
        FP_SUBNORMAL
    } else {
        class32(x)
    }
}
pub unsafe extern "win64" fn _dtest(x: *const f64) -> i16 {
    class64(*x)
}
pub unsafe extern "win64" fn _ldtest(x: *const f64) -> i16 {
    class64(*x)
}
pub unsafe extern "win64" fn _fdtest(x: *const f32) -> i16 {
    _fdclass(*x)
}
pub extern "win64" fn _dsign(x: f64) -> i32 {
    if x.is_sign_negative() {
        0x8000
    } else {
        0
    }
}
pub extern "win64" fn _ldsign(x: f64) -> i32 {
    _dsign(x)
}
pub extern "win64" fn _fdsign(x: f32) -> i32 {
    if x.is_sign_negative() {
        0x8000
    } else {
        0
    }
}
fn pcomp(x: f64, y: f64) -> i32 {
    // _FP_LT 1, _FP_EQ 2, _FP_GT 4; 0 = unordered.
    if x < y {
        1
    } else if x == y {
        2
    } else if x > y {
        4
    } else {
        0
    }
}
pub extern "win64" fn _dpcomp(x: f64, y: f64) -> i32 {
    pcomp(x, y)
}
pub extern "win64" fn _ldpcomp(x: f64, y: f64) -> i32 {
    pcomp(x, y)
}
pub extern "win64" fn _fdpcomp(x: f32, y: f32) -> i32 {
    pcomp(x as f64, y as f64)
}
pub extern "win64" fn _finite(x: f64) -> i32 {
    x.is_finite() as i32
}
pub extern "win64" fn _finitef(x: f32) -> i32 {
    x.is_finite() as i32
}
#[inline(never)]
pub extern "win64" fn _isnan(x: f64) -> i32 {
    x.is_nan() as i32
}
#[inline(never)]
pub extern "win64" fn isnan(x: f64) -> i32 {
    x.is_nan() as i32
}
#[inline(never)]
pub extern "win64" fn _isnanf(x: f32) -> i32 {
    x.is_nan() as i32
}
pub extern "win64" fn _fpclass(x: f64) -> i32 {
    use std::num::FpCategory::*;
    let neg = x.is_sign_negative();
    match x.classify() {
        Nan => {
            if x.to_bits() & (1 << 51) != 0 {
                2
            } else {
                1
            }
        }
        Infinite => {
            if neg {
                4
            } else {
                0x200
            }
        }
        Normal => {
            if neg {
                8
            } else {
                0x100
            }
        }
        Subnormal => {
            if neg {
                0x10
            } else {
                0x80
            }
        }
        Zero => {
            if neg {
                0x20
            } else {
                0x40
            }
        }
    }
}
pub extern "win64" fn _fpclassf(x: f32) -> i32 {
    _fpclass(x as f64)
}
pub extern "win64" fn _chgsign(x: f64) -> f64 {
    -x
}
pub extern "win64" fn _logb(x: f64) -> f64 {
    logb(x)
}
#[cfg(not(target_arch = "aarch64"))]
pub extern "win64" fn _scalb(x: f64, e: i32) -> f64 {
    scalbn(x, e)
}
/// aarch64 bridge layout; see the note above `frexp`.
#[cfg(target_arch = "aarch64")]
pub extern "win64" fn _scalb(_rcx: u64, e: i32, x: f64) -> f64 {
    scalbn(0, e, x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classes() {
        assert_eq!(_dclass(0.0), FP_ZERO);
        assert_eq!(_dclass(f64::NAN), FP_NAN);
        assert_eq!(_dclass(1.0), FP_NORMAL);
        assert_eq!(_dclass(f64::MIN_POSITIVE / 2.0), FP_SUBNORMAL);
        assert_eq!(_fdclass(f32::INFINITY), FP_INFINITE);
        assert_eq!(_dpcomp(1.0, 2.0), 1);
        assert_eq!(_dpcomp(f64::NAN, 2.0), 0);
        assert_eq!(_fpclass(-0.0), 0x20);
        assert_eq!(fmod(7.5, 2.0), 1.5);
        let mut e = 0;
        #[cfg(target_arch = "x86_64")]
        assert_eq!(unsafe { frexp(8.0, &mut e) }, 0.5);
        #[cfg(not(target_arch = "x86_64"))]
        assert_eq!(unsafe { frexp(0, &mut e, 8.0) }, 0.5);
        assert_eq!(e, 4);
        assert_eq!(lrint(2.5), 2);
        assert_eq!(nextafterf(1.0f32, 2.0f32), f32::from_bits(1.0f32.to_bits() + 1));
        assert_eq!(nextafterf(1.0f32, 0.0f32), f32::from_bits(1.0f32.to_bits() - 1));
        assert_eq!(nextafter(1.0f64, 2.0f64), f64::from_bits(1.0f64.to_bits() + 1));
    }
}
