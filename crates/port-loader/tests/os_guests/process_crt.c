#include "check.h"
#include <stdarg.h>

typedef int (__cdecl *CommonFormat)(unsigned long long, char *, size_t, const char *, void *, va_list);
static int format(CommonFormat fn, char *out, size_t cap, const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    int n = fn(2, out, cap, fmt, NULL, ap); // _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR
    va_end(ap);
    return n;
}
int main(void) {
    CHECK(GetCommandLineW() && wcsstr(GetCommandLineW(), L"process_crt"));
    CHECK(SetEnvironmentVariableW(L"CorpusValue", L"CaseTest"));
    WCHAR value[32];
    CHECK(GetEnvironmentVariableW(L"CORPUSVALUE", NULL, 0) == 9);
    CHECK(GetEnvironmentVariableW(L"corpusvalue", value, 32) == 8 && !wcscmp(value, L"CaseTest"));
    CHECK(SetEnvironmentVariableW(L"CorpusValue", NULL) && !GetEnvironmentVariableW(L"CorpusValue", value, 32));
    CHECK(GetLastError() == ERROR_ENVVAR_NOT_FOUND);
    HMODULE crt = LoadLibraryW(L"ucrtbase.dll");
    CHECK(crt);
    CommonFormat print = (CommonFormat)GetProcAddress(crt, "__stdio_common_vsprintf");
    double (__cdecl *root)(double) = (void *)GetProcAddress(crt, "sqrt");
    CHECK(print && root && root(49.0) == 7.0);
    char buf[64];
    CHECK(format(print, buf, sizeof(buf), "%.2f:%I64x:%04d", 1.25, 0x112233445566ULL, 17) == 22);
    CHECK(!strcmp(buf, "1.25:112233445566:0017"));
    CHECK(format(print, buf, 4, "%s", "abcdef") == 6 && !memcmp(buf, "abc", 4));
    CHECK(FreeLibrary(crt));
    puts("PASS process_crt");
    return 0;
}
