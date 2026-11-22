// Minimal UCRT-style setjmp.h for clang-cl reference builds (mingw's conflicts with clang's intrin.h).
#pragma once
#ifndef _INC_SETJMP
#define _INC_SETJMP
typedef struct __declspec(align(16)) _SETJMP_FLOAT128 { unsigned __int64 Part[2]; } SETJMP_FLOAT128;
#define _JBLEN 16
typedef SETJMP_FLOAT128 _JBTYPE;
typedef _JBTYPE jmp_buf[_JBLEN];
#ifdef __cplusplus
extern "C" {
#endif
int __cdecl _setjmp(jmp_buf);
__declspec(noreturn) void __cdecl longjmp(jmp_buf, int);
#ifdef __cplusplus
}
#endif
#define setjmp _setjmp
#endif
