#define WIN32_LEAN_AND_MEAN
#include <windows.h>

static volatile LONG calls[4];
BOOL WINAPI DllMain(HINSTANCE instance, DWORD reason, LPVOID reserved) {
    (void)instance; (void)reserved;
    if (reason < 4) InterlockedIncrement(&calls[reason]);
    return TRUE;
}

__declspec(dllexport) LONG WINAPI notifications(DWORD reason) {
    return InterlockedCompareExchange(&calls[reason], 0, 0);
}
