#include "check.h"

static LRESULT CALLBACK proc(HWND h, UINT msg, WPARAM w, LPARAM l) {
    if (msg == WM_APP+1) return (LRESULT)(w+l);
    return DefWindowProcW(h, msg, w, l);
}
int main(void) {
    WNDCLASSW cls = {0};
    cls.hInstance = GetModuleHandleW(NULL); cls.lpfnWndProc = proc; cls.lpszClassName = L"CorpusMsg";
    CHECK(RegisterClassW(&cls));
    HWND win = CreateWindowExW(0, cls.lpszClassName, L"", 0, 0,0,0,0, HWND_MESSAGE, NULL, cls.hInstance, NULL);
    CHECK(win && PostMessageW(win, WM_APP+1, 7, 13));
    MSG msg;
    CHECK(PeekMessageW(&msg, win, WM_APP+1, WM_APP+1, PM_REMOVE) && DispatchMessageW(&msg) == 20);
    CHECK(DestroyWindow(win) && UnregisterClassW(cls.lpszClassName, cls.hInstance));
    UINT border = 0, hover = 0;
    CHECK(SystemParametersInfoW(SPI_GETFOCUSBORDERWIDTH, 0, &border, 0) && border == (UINT)GetSystemMetrics(SM_CXFOCUSBORDER) && border);
    CHECK(SystemParametersInfoA(SPI_GETMOUSEHOVERWIDTH, 0, &hover, 0) && hover > 0);
    CHECK(GetSystemMetrics(SM_CXEDGE) >= GetSystemMetrics(SM_CXBORDER) && GetSystemMetrics(SM_CYVSCROLL) > 0);
    HDC dc = CreateCompatibleDC(NULL);
    CHECK(dc && GetDeviceCaps(dc, TECHNOLOGY) == DT_RASDISPLAY && (GetDeviceCaps(dc, RASTERCAPS) & RC_BITBLT));
    CHECK(GetDeviceCaps(dc, HORZRES) == GetSystemMetrics(SM_CXSCREEN) && GetDeviceCaps(dc, HORZSIZE) > 0 && GetDeviceCaps(dc, VREFRESH) > 0);
    CHECK(DeleteDC(dc));

    HANDLE token, duplicate;
    CHECK(OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY|TOKEN_DUPLICATE, &token));
    DWORD n = 0;
    TOKEN_TYPE kind;
    CHECK(GetTokenInformation(token, TokenType, &kind, sizeof(kind), &n) && kind == TokenPrimary && n == sizeof(kind));
    CHECK(!GetTokenInformation(token, TokenGroups, NULL, 0, &n) && GetLastError() == ERROR_INSUFFICIENT_BUFFER);
    TOKEN_GROUPS *groups = malloc(n);
    CHECK(groups && GetTokenInformation(token, TokenGroups, groups, n, &n) && groups->GroupCount);
    CHECK(DuplicateToken(token, SecurityImpersonation, &duplicate));
    CHECK(GetTokenInformation(duplicate, TokenType, &kind, sizeof(kind), &n) && kind == TokenImpersonation);
    SECURITY_IMPERSONATION_LEVEL level;
    CHECK(GetTokenInformation(duplicate, TokenImpersonationLevel, &level, sizeof(level), &n) && level == SecurityImpersonation);
    BOOL member = FALSE;
    for (DWORD i = 0; i < groups->GroupCount; ++i) {
        if (groups->Groups[i].Attributes & SE_GROUP_ENABLED) {
            CHECK(CheckTokenMembership(duplicate, groups->Groups[i].Sid, &member) && member);
        }
    }
    CHECK(member);
    free(groups); CloseHandle(duplicate); CloseHandle(token);
    puts("PASS messages_security");
    return 0;
}
