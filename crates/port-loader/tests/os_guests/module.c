#include "check.h"

static DWORD WINAPI noop(void *ctx) { (void)ctx; return 0; }

int main(void) {
    HMODULE dll = LoadLibraryW(L"notify_fixture.dll"), found = NULL;
    CHECK(dll && GetModuleHandleW(L"notify_fixture.dll") == dll);
    LONG (WINAPI *calls)(DWORD) = (void *)GetProcAddress(dll, "notifications");
    CHECK(calls && calls(DLL_PROCESS_ATTACH) == 1);
    CHECK(GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS|GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                             (LPCWSTR)(uintptr_t)calls, &found) && found == dll);
    CHECK(!GetProcAddress(dll, "missing_export") && GetLastError() == ERROR_PROC_NOT_FOUND);
    LONG attach = calls(DLL_THREAD_ATTACH), detach = calls(DLL_THREAD_DETACH);
    HANDLE t = CreateThread(NULL, 0, noop, NULL, 0, NULL);
    CHECK(t && WaitForSingleObject(t, 5000) == WAIT_OBJECT_0);
    CHECK(calls(DLL_THREAD_ATTACH) > attach && calls(DLL_THREAD_DETACH) > detach);
    CHECK(CloseHandle(t) && FreeLibrary(dll));
    puts("PASS module");
    return 0;
}
