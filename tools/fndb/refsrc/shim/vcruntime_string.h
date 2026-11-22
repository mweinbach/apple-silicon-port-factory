#pragma once
#include <vcruntime.h>
extern "C" {
void* __cdecl memchr(const void* _Buf, int _Val, size_t _MaxCount);
int __cdecl memcmp(const void* _Buf1, const void* _Buf2, size_t _Size);
void* __cdecl memcpy(void* _Dst, const void* _Src, size_t _Size);
void* __cdecl memmove(void* _Dst, const void* _Src, size_t _Size);
void* __cdecl memset(void* _Dst, int _Val, size_t _Size);
char* __cdecl strchr(const char* _Str, int _Val);
char* __cdecl strrchr(const char* _Str, int _Ch);
char* __cdecl strstr(const char* _Str, const char* _SubStr);
wchar_t* __cdecl wcschr(const wchar_t* _Str, wchar_t _Ch);
wchar_t* __cdecl wcsrchr(const wchar_t* _Str, wchar_t _Ch);
wchar_t* __cdecl wcsstr(const wchar_t* _Str, const wchar_t* _SubStr);
}
