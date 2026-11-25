#include "check.h"

int main(void) {
    SYSTEM_INFO si;
    GetSystemInfo(&si);
    size_t page = si.dwPageSize;
    char *base = VirtualAlloc(NULL, 3 * page, MEM_RESERVE, PAGE_READWRITE);
    MEMORY_BASIC_INFORMATION mbi;
    CHECK(base && ((uintptr_t)base % si.dwAllocationGranularity) == 0);
    CHECK(VirtualQuery(base, &mbi, sizeof(mbi)) == sizeof(mbi));
    CHECK(mbi.State == MEM_RESERVE && mbi.AllocationBase == base);
    char *p = VirtualAlloc(base + page, page, MEM_COMMIT, PAGE_READWRITE);
    CHECK(p == base + page && p[0] == 0);
    DWORD unchanged = 0x12345678;
    CHECK(!VirtualProtect(base, 2*page, PAGE_READONLY, &unchanged) && unchanged == 0x12345678);
    CHECK(VirtualQuery(base, &mbi, sizeof(mbi)) == sizeof(mbi) && mbi.State == MEM_RESERVE);
    p[0] = 42;
    DWORD prev;
    CHECK(VirtualProtect(p, page, PAGE_READONLY, &prev) && prev == PAGE_READWRITE);
    CHECK(VirtualQuery(p, &mbi, sizeof(mbi)) == sizeof(mbi));
    CHECK(mbi.Type == MEM_PRIVATE && mbi.RegionSize == page && mbi.Protect == PAGE_READONLY && p[0] == 42);
    CHECK(VirtualProtect(p, page, PAGE_READWRITE, &prev) && prev == PAGE_READONLY);
    CHECK(VirtualFree(p, page, MEM_DECOMMIT));
    CHECK(VirtualAlloc(p, page, MEM_COMMIT, PAGE_READWRITE) == p && p[0] == 0);
    CHECK(VirtualFree(base, 0, MEM_RELEASE));

    p = VirtualAlloc(NULL, page, MEM_COMMIT, PAGE_READWRITE);
    CHECK(p && p[0] == 0 && VirtualFree(p, 0, MEM_RELEASE));
    ULONG_PTR count = 1;
    void *dirty = NULL;
    DWORD granularity = 0;
    CHECK(GetWriteWatch(0, &mbi, 1, &dirty, &count, &granularity) != 0);
    CHECK(ResetWriteWatch(&mbi, 1) != 0);

    HANDLE mapping = CreateFileMappingW(INVALID_HANDLE_VALUE, NULL, PAGE_READWRITE, 0, (DWORD)page, NULL);
    CHECK(mapping);
    char *rw = MapViewOfFile(mapping, FILE_MAP_WRITE, 0, 0, 0);
    const char *ro = MapViewOfFile(mapping, FILE_MAP_READ, 0, 0, 0);
    CHECK(rw && ro && rw != ro);
    rw[37] = 'x';
    CHECK(ro[37] == 'x' && VirtualQuery(ro, &mbi, sizeof(mbi)) == sizeof(mbi) && mbi.Type == MEM_MAPPED);
    CHECK(UnmapViewOfFile(rw) && UnmapViewOfFile(ro) && CloseHandle(mapping));

    WCHAR temp[MAX_PATH], name[MAX_PATH];
    CHECK(GetTempPathW(MAX_PATH, temp) && GetTempFileNameW(temp, L"mem", 0, name));
    HANDLE file = CreateFileW(name, GENERIC_READ|GENERIC_WRITE, 0, NULL, OPEN_EXISTING, FILE_FLAG_DELETE_ON_CLOSE, NULL);
    CHECK(file != INVALID_HANDLE_VALUE);
    FILE_ALLOCATION_INFO alloc;
    alloc.AllocationSize.QuadPart = 65536;
    CHECK(SetFileInformationByHandle(file, FileAllocationInfo, &alloc, sizeof(alloc)));
    FILE_STANDARD_INFO st;
    CHECK(GetFileInformationByHandleEx(file, FileStandardInfo, &st, sizeof(st)));
    CHECK(st.EndOfFile.QuadPart == 0 && st.AllocationSize.QuadPart >= 65536);
    DWORD written;
    CHECK(WriteFile(file, "abc", 3, &written, NULL) && written == 3);
    BYTE buf[128];
    FILE_STREAM_INFO *stream = (FILE_STREAM_INFO *)buf;
    CHECK(GetFileInformationByHandleEx(file, FileStreamInfo, buf, sizeof(buf)));
    CHECK(!stream->NextEntryOffset && stream->StreamNameLength == 14 && stream->StreamSize.QuadPart == 3);
    CHECK(!memcmp(stream->StreamName, L"::$DATA", 14));
    FILE_COMPRESSION_INFO compression;
    CHECK(GetFileInformationByHandleEx(file, FileCompressionInfo, &compression, sizeof(compression)));
    CHECK(compression.CompressionFormat == COMPRESSION_FORMAT_NONE && compression.CompressedFileSize.QuadPart == 3);
    FILE_IO_PRIORITY_HINT_INFO hint = {IoPriorityHintLow};
    CHECK(SetFileInformationByHandle(file, FileIoPriorityHintInfo, &hint, sizeof(hint)));
    hint.PriorityHint = MaximumIoPriorityHintType;
    CHECK(GetFileInformationByHandleEx(file, FileIoPriorityHintInfo, &hint, sizeof(hint)) && hint.PriorityHint == IoPriorityHintLow);
    CHECK(!SetFileInformationByHandle(file, FileEndOfFileInfo, &alloc, 1));
    CHECK(CloseHandle(file));
    puts("PASS memory_file");
    return 0;
}
