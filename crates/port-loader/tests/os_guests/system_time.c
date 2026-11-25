#include "check.h"
#include <mmsystem.h>

typedef LONG (NTAPI *Query)(HANDLE, ULONG, PVOID, ULONG, PULONG);
typedef LONG (NTAPI *QuerySystem)(ULONG, PVOID, ULONG, PULONG);
typedef struct { USHORT length, capacity; PWSTR data; } CountedString;

int main(void) {
    SYSTEM_INFO si;
    GetSystemInfo(&si);
    CHECK(si.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_AMD64 && si.dwPageSize == 4096);
    CHECK(si.dwAllocationGranularity == 65536 && si.dwActiveProcessorMask && si.dwNumberOfProcessors);
    BYTE odd[sizeof(SYSTEM_INFO) + 1];
    GetSystemInfo((SYSTEM_INFO *)(odd+1));
    CHECK(!memcmp(odd+1, &si, sizeof(si)));
    DWORD len = 0;
    CHECK(!GetLogicalProcessorInformationEx(RelationProcessorCore, NULL, &len) && GetLastError() == ERROR_INSUFFICIENT_BUFFER);
    BYTE *topology = malloc(len);
    CHECK(topology && GetLogicalProcessorInformationEx(RelationProcessorCore, (void *)topology, &len));
    ULONG_PTR seen = 0;
    for (DWORD pos = 0; pos < len;) {
        PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX r = (void *)(topology + pos);
        CHECK(r->Relationship == RelationProcessorCore && r->Size >= 48 && r->Size <= len-pos);
        CHECK(r->Processor.GroupCount == 1 && !(seen & r->Processor.GroupMask[0].Mask));
        seen |= r->Processor.GroupMask[0].Mask;
        pos += r->Size;
    }
    CHECK(seen == si.dwActiveProcessorMask);
    free(topology);
    MEMORYSTATUSEX mem = {0};
    mem.dwLength = sizeof(mem);
    CHECK(GlobalMemoryStatusEx(&mem) && mem.ullTotalPhys && mem.ullAvailPhys <= mem.ullTotalPhys && mem.dwMemoryLoad <= 100);
    CHECK(!GetLogicalProcessorInformationEx((LOGICAL_PROCESSOR_RELATIONSHIP)0x12345, NULL, &len) && GetLastError() == ERROR_INVALID_PARAMETER);
    LARGE_INTEGER freq, start, end;
    FILETIME a,b;
    DWORD ms = timeGetTime();
    CHECK(QueryPerformanceFrequency(&freq) && freq.QuadPart > 0 && QueryPerformanceCounter(&start));
    GetSystemTimePreciseAsFileTime(&a);
    Sleep(30);
    GetSystemTimePreciseAsFileTime(&b);
    CHECK(QueryPerformanceCounter(&end) && end.QuadPart > start.QuadPart && (DWORD)(timeGetTime()-ms) > 0);
    CHECK(CompareFileTime(&a, &b) <= 0 && a.dwHighDateTime);

    HMODULE ntdll = GetModuleHandleW(L"ntdll.dll");
    Query qp = (Query)GetProcAddress(ntdll, "NtQueryInformationProcess");
    Query qt = (Query)GetProcAddress(ntdll, "NtQueryInformationThread");
    QuerySystem qs = (QuerySystem)GetProcAddress(ntdll, "NtQuerySystemInformation");
    CHECK(qp && qt && qs);
    ULONG need = 0;
    CHECK(qp(GetCurrentProcess(), 60, NULL, 0, &need) < 0 && need >= sizeof(CountedString));
    CountedString *cmd = malloc(need);
    CHECK(cmd && qp(GetCurrentProcess(), 60, cmd, need, NULL) == 0);
    CHECK(cmd->length == 2*wcslen(GetCommandLineW()) && !memcmp(cmd->data, GetCommandLineW(), cmd->length));
    free(cmd);
    FILETIME times[4];
    CHECK(qp(GetCurrentProcess(), 4, times, sizeof(times), &need) == 0 && need == sizeof(times) && times[0].dwHighDateTime);
    CHECK(qt(GetCurrentThread(), 1, times, sizeof(times), &need) == 0 && need == sizeof(times) && times[0].dwHighDateTime);
    GROUP_AFFINITY group;
    CHECK(qt(GetCurrentThread(), 30, &group, sizeof(group), &need) == 0 && need == sizeof(group) && group.Mask);
    CHECK(qs(73, NULL, 0, &need) < 0 && need >= sizeof(SYSTEM_LOGICAL_PROCESSOR_INFORMATION));
    topology = malloc(need);
    CHECK(topology && qs(73, topology, need, &need) == 0);
    CHECK(((PSYSTEM_LOGICAL_PROCESSOR_INFORMATION)topology)->ProcessorMask);
    free(topology);
    puts("PASS system_time");
    return 0;
}
