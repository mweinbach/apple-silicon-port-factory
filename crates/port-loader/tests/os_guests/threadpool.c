#include "check.h"

typedef struct {
    HANDLE started, release, closing;
    PTP_CLEANUP_GROUP group;
    volatile LONG runs, cleanups;
} State;

static void CALLBACK work(PTP_CALLBACK_INSTANCE instance, void *ctx, PTP_WORK w) {
    (void)instance; (void)w;
    State *s = ctx;
    SetEvent(s->started);
    if (WaitForSingleObject(s->release, 5000) == WAIT_OBJECT_0) InterlockedIncrement(&s->runs);
}
static void CALLBACK cleanup(void *ctx, void *cleanup_ctx) {
    State *s = ctx;
    if (cleanup_ctx == s) InterlockedIncrement(&s->cleanups);
}
static void CALLBACK unused_timer(PTP_CALLBACK_INSTANCE i, void *ctx, PTP_TIMER t) {
    (void)i; (void)t;
    InterlockedIncrement(&((State *)ctx)->runs);
}
static void CALLBACK unused_wait(PTP_CALLBACK_INSTANCE i, void *ctx, PTP_WAIT w, TP_WAIT_RESULT r) {
    (void)i; (void)w; (void)r;
    InterlockedIncrement(&((State *)ctx)->runs);
}
static void CALLBACK simple(PTP_CALLBACK_INSTANCE i, void *ctx) {
    (void)i;
    InterlockedIncrement(&((State *)ctx)->runs);
}
static DWORD WINAPI close_members(void *ctx) {
    State *s = ctx;
    SetEvent(s->closing);
    CloseThreadpoolCleanupGroupMembers(s->group, FALSE, s);
    return 0;
}

int main(void) {
    State s = {0};
    s.started = CreateEventW(NULL, TRUE, FALSE, NULL);
    s.release = CreateEventW(NULL, TRUE, FALSE, NULL);
    s.closing = CreateEventW(NULL, TRUE, FALSE, NULL);
    PTP_CLEANUP_GROUP group = CreateThreadpoolCleanupGroup();
    s.group = group;
    CHECK(group && s.started && s.release && s.closing);
    TP_CALLBACK_ENVIRON env;
    InitializeThreadpoolEnvironment(&env);
    SetThreadpoolCallbackCleanupGroup(&env, group, cleanup);
    PTP_WORK w = CreateThreadpoolWork(work, &s, &env);
    CHECK(w);
    SubmitThreadpoolWork(w);
    CHECK(WaitForSingleObject(s.started, 5000) == WAIT_OBJECT_0);
    CHECK(TrySubmitThreadpoolCallback(simple, &s, &env));
    HANDLE closer = CreateThread(NULL, 0, close_members, &s, 0, NULL);
    CHECK(closer && WaitForSingleObject(s.closing, 5000) == WAIT_OBJECT_0);
    CHECK(WaitForSingleObject(closer, 50) == WAIT_TIMEOUT);
    SetEvent(s.release);
    CHECK(WaitForSingleObject(closer, 5000) == WAIT_OBJECT_0 && CloseHandle(closer));
    CHECK(s.runs == 2 && s.cleanups == 0);

    // A drained group can be reused. An object closed separately is no longer a member.
    PTP_TIMER timer = CreateThreadpoolTimer(unused_timer, &s, &env);
    PTP_WAIT wait = CreateThreadpoolWait(unused_wait, &s, &env);
    w = CreateThreadpoolWork(work, &s, &env);
    CHECK(timer && wait && w);
    CloseThreadpoolWork(w);
    ResetEvent(s.started);
    SetThreadpoolWait(wait, s.started, NULL);
    LARGE_INTEGER due; due.QuadPart = -600000000LL;
    SetThreadpoolTimer(timer, (FILETIME *)&due, 0, 0);
    CloseThreadpoolCleanupGroupMembers(group, TRUE, &s);
    CHECK(s.runs == 2 && s.cleanups == 2);
    CloseThreadpoolCleanupGroupMembers(group, TRUE, &s);
    CHECK(s.cleanups == 2);
    CloseThreadpoolCleanupGroup(group);
    DestroyThreadpoolEnvironment(&env);
    CloseHandle(s.started); CloseHandle(s.release); CloseHandle(s.closing);
    puts("PASS threadpool");
    return 0;
}
