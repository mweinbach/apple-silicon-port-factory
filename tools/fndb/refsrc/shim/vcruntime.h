// Minimal vcruntime.h shim for compiling microsoft/STL headers with clang-cl + mingw-w64 C headers.
// Only used to build reference objects for fndb signature matching (never shipped).
#pragma once
#include <_mingw.h>
#include <sal.h>
#include <vadefs.h>
#ifndef _VCRUNTIME_H
#define _VCRUNTIME_H
#define _VCRT_COMPILER_PREPROCESSOR 1
#define _CRT_BEGIN_C_HEADER __pragma(pack(push, 8)) extern "C" {
#define _CRT_END_C_HEADER } __pragma(pack(pop))
#ifndef _VCRTIMP
#define _VCRTIMP
#endif
#ifndef _CRTIMP
#define _CRTIMP
#endif
#ifndef __CRTDECL
#define __CRTDECL __cdecl
#endif
#define _CRT_STRINGIZE_(x) #x
#define _CRT_STRINGIZE(x) _CRT_STRINGIZE_(x)
#define _CRT_WIDE_(s) L ## s
#define _CRT_WIDE(s) _CRT_WIDE_(s)
#define _CRT_CONCATENATE_(a, b) a ## b
#define _CRT_CONCATENATE(a, b) _CRT_CONCATENATE_(a, b)
#define _CRT_UNPARENTHESIZE_(...) __VA_ARGS__
#define _CRT_UNPARENTHESIZE(...) _CRT_UNPARENTHESIZE_ __VA_ARGS__
#ifndef _VCRT_ALIGN
#define _VCRT_ALIGN(x) __declspec(align(x))
#endif
#ifndef _CRT_ALIGN
#define _CRT_ALIGN(x) __declspec(align(x))
#endif
#define _VCRT_NOALIAS __declspec(noalias)
#define _VCRT_RESTRICT __declspec(restrict)
#define _VCRT_ALLOCATOR __declspec(allocator)
#define _CRT_GUARDOVERFLOW
#define _CRT_HYBRIDPATCHABLE
#ifndef _CRT_DEPRECATE_TEXT
#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
#endif
#ifndef _CRT_INSECURE_DEPRECATE
#define _CRT_INSECURE_DEPRECATE(_Replacement)
#endif
#ifndef _CRT_INSECURE_DEPRECATE_MEMORY
#define _CRT_INSECURE_DEPRECATE_MEMORY(_Replacement)
#endif
#ifndef _HAS_EXCEPTIONS
#define _HAS_EXCEPTIONS 1
#endif
#ifndef _HAS_NODISCARD
#define _HAS_NODISCARD 1
#endif
#define _NODISCARD [[nodiscard]]
#define _CRT_SECURE_CPP_NOTHROW throw()
#ifndef _CRT_FUNCTIONS_REQUIRED
#define _CRT_FUNCTIONS_REQUIRED 1
#endif
#define _CRT_HAS_CXX17 1
#define _CRT_INTERNAL_NONSTDC_NAMES 1
#ifndef _UINTPTR_T_DEFINED
#define _UINTPTR_T_DEFINED
typedef unsigned __int64 uintptr_t;
#endif
#ifndef _INTPTR_T_DEFINED
#define _INTPTR_T_DEFINED
typedef __int64 intptr_t;
#endif
#ifndef _SIZE_T_DEFINED
#define _SIZE_T_DEFINED
typedef unsigned __int64 size_t;
#endif
#ifndef _PTRDIFF_T_DEFINED
#define _PTRDIFF_T_DEFINED
typedef __int64 ptrdiff_t;
#endif
typedef size_t rsize_t;
typedef bool __vcrt_bool;
#define __CLR_OR_THIS_CALL
#define __CLRCALL_OR_CDECL __cdecl
#define __CLRCALL_PURE_OR_CDECL __cdecl
#define __CRTDECL __cdecl
#ifndef __MINGW_GNU_PRINTF
#define __MINGW_GNU_PRINTF(a,b)
#define __MINGW_GNU_SCANF(a,b)
#define __MINGW_MS_PRINTF(a,b)
#define __MINGW_MS_SCANF(a,b)
#endif
extern "C" {
extern uintptr_t __security_cookie;
void __cdecl __security_init_cookie(void);
void __cdecl __security_check_cookie(uintptr_t _StackCookie);
__declspec(noreturn) void __cdecl __report_gsfailure(uintptr_t _StackCookie);
}
#define __crt_countof(_Array) (sizeof(_Array) / sizeof(_Array[0]))
#endif
#ifndef _MBSTATET_DEFINED_SHIM
#define _MBSTATET_DEFINED_SHIM
typedef struct _Mbstatet { unsigned long _Wchar; unsigned short _Byte, _State; } _Mbstatet;
#endif
#ifndef _HAS_CXX17
#if defined(_MSVC_LANG) && _MSVC_LANG > 201402L
#define _HAS_CXX17 1
#else
#define _HAS_CXX17 0
#endif
#endif
#ifndef _HAS_CXX20
#if defined(_MSVC_LANG) && _MSVC_LANG > 201703L
#define _HAS_CXX20 1
#else
#define _HAS_CXX20 0
#endif
#endif
extern "C" __declspec(noreturn) void __cdecl _invalid_parameter_noinfo_noreturn(void);
extern "C" void __cdecl _invalid_parameter_noinfo(void);
#ifndef _CRT_SECURE_INVALID_PARAMETER
#define _CRT_SECURE_INVALID_PARAMETER(expr) ::_invalid_parameter_noinfo_noreturn()
#endif
#ifndef _ACRTIMP
#define _ACRTIMP
#endif
#ifndef _ACRTIMP_ALT
#define _ACRTIMP_ALT
#endif
#ifndef _DCRTIMP
#define _DCRTIMP
#endif
#ifndef _CRT_SATELLITE_1
#define _CRT_SATELLITE_1
#endif
#ifndef _CRT_SATELLITE_2
#define _CRT_SATELLITE_2
#endif
#ifndef _CRT_SATELLITE_CODECVT_IDS_NOIMPORT
#define _CRT_SATELLITE_CODECVT_IDS_NOIMPORT
#endif
#ifndef _CRT_SATELLITE_CODECVT_IDS
#define _CRT_SATELLITE_CODECVT_IDS
#endif
