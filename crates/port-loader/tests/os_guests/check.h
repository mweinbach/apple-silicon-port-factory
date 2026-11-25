#define _WIN32_WINNT 0x0A00
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

#define CHECK(expr) do { if (!(expr)) { \
    fprintf(stderr, "FAIL: line %d: %s (last error %lu)\n", __LINE__, #expr, GetLastError()); \
    return 1; \
} } while (0)
