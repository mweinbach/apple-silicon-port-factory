#pragma once
#include_next <math.h>
#ifdef __cplusplus
#undef fpclassify
#undef signbit
#undef isfinite
#undef isinf
#undef isnan
#undef isnormal
#undef isgreater
#undef isgreaterequal
#undef isless
#undef islessequal
#undef islessgreater
#undef isunordered
extern "C++" {
template <class _Ty> inline int fpclassify(_Ty _X) noexcept { return __builtin_fpclassify(FP_NAN, FP_INFINITE, FP_NORMAL, FP_SUBNORMAL, FP_ZERO, _X); }
template <class _Ty> inline bool signbit(_Ty _X) noexcept { return __builtin_signbit(_X); }
template <class _Ty> inline bool isfinite(_Ty _X) noexcept { return __builtin_isfinite(_X); }
template <class _Ty> inline bool isinf(_Ty _X) noexcept { return __builtin_isinf(_X); }
template <class _Ty> inline bool isnan(_Ty _X) noexcept { return __builtin_isnan(_X); }
template <class _Ty> inline bool isnormal(_Ty _X) noexcept { return __builtin_isnormal(_X); }
template <class _T1, class _T2> inline bool isgreater(_T1 a, _T2 b) noexcept { return __builtin_isgreater(a, b); }
template <class _T1, class _T2> inline bool isgreaterequal(_T1 a, _T2 b) noexcept { return __builtin_isgreaterequal(a, b); }
template <class _T1, class _T2> inline bool isless(_T1 a, _T2 b) noexcept { return __builtin_isless(a, b); }
template <class _T1, class _T2> inline bool islessequal(_T1 a, _T2 b) noexcept { return __builtin_islessequal(a, b); }
template <class _T1, class _T2> inline bool islessgreater(_T1 a, _T2 b) noexcept { return __builtin_islessgreater(a, b); }
template <class _T1, class _T2> inline bool isunordered(_T1 a, _T2 b) noexcept { return __builtin_isunordered(a, b); }
}
#endif
