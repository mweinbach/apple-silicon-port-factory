//! Windows ABI types and constants (x64). Note `LONG`/`ULONG` are 32-bit.

use std::ffi::c_void;

pub type BOOL = i32;
pub type BOOLEAN = u8;
pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type LONG = i32;
pub type ULONG = u32;
pub type UINT = u32;
pub type INT = i32;
pub type LONGLONG = i64;
pub type ULONGLONG = u64;
pub type SIZE_T = usize;
pub type ULONG_PTR = usize;
pub type LONG_PTR = isize;
pub type DWORD_PTR = usize;
pub type WCHAR = u16;
pub type LPCWSTR = *const u16;
pub type LPWSTR = *mut u16;
pub type LPCSTR = *const u8;
pub type LPSTR = *mut u8;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type HANDLE = *mut c_void;
pub type HMODULE = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HWND = *mut c_void;
pub type HRESULT = i32;
pub type NTSTATUS = i32;
pub type LCID = u32;

pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;
pub const INVALID_HANDLE_VALUE: HANDLE = usize::MAX as HANDLE;
pub const NULL: HANDLE = std::ptr::null_mut();

pub const INFINITE: u32 = 0xFFFF_FFFF;
pub const WAIT_OBJECT_0: u32 = 0;
pub const WAIT_ABANDONED_0: u32 = 0x80;
pub const WAIT_IO_COMPLETION: u32 = 0xC0;
pub const WAIT_TIMEOUT: u32 = 0x102;
pub const WAIT_FAILED: u32 = 0xFFFF_FFFF;

pub const S_OK: HRESULT = 0;
pub const S_FALSE: HRESULT = 1;
pub const E_NOTIMPL: HRESULT = 0x8000_4001u32 as i32;
pub const E_NOINTERFACE: HRESULT = 0x8000_4002u32 as i32;
pub const E_POINTER: HRESULT = 0x8000_4003u32 as i32;
pub const E_FAIL: HRESULT = 0x8000_4005u32 as i32;
pub const E_INVALIDARG: HRESULT = 0x8007_0057u32 as i32;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000Eu32 as i32;

/// Win32 error codes (winerror.h).
pub mod err {
    pub const ERROR_SUCCESS: u32 = 0;
    pub const ERROR_INVALID_FUNCTION: u32 = 1;
    pub const ERROR_FILE_NOT_FOUND: u32 = 2;
    pub const ERROR_PATH_NOT_FOUND: u32 = 3;
    pub const ERROR_TOO_MANY_OPEN_FILES: u32 = 4;
    pub const ERROR_ACCESS_DENIED: u32 = 5;
    pub const ERROR_INVALID_HANDLE: u32 = 6;
    pub const ERROR_NOT_ENOUGH_MEMORY: u32 = 8;
    pub const ERROR_INVALID_DATA: u32 = 13;
    pub const ERROR_OUTOFMEMORY: u32 = 14;
    pub const ERROR_NO_MORE_FILES: u32 = 18;
    pub const ERROR_WRITE_PROTECT: u32 = 19;
    pub const ERROR_SHARING_VIOLATION: u32 = 32;
    pub const ERROR_HANDLE_EOF: u32 = 38;
    pub const ERROR_NOT_SUPPORTED: u32 = 50;
    pub const ERROR_FILE_EXISTS: u32 = 80;
    pub const ERROR_INVALID_PARAMETER: u32 = 87;
    pub const ERROR_BROKEN_PIPE: u32 = 109;
    pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;
    pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
    pub const ERROR_INVALID_NAME: u32 = 123;
    pub const ERROR_MOD_NOT_FOUND: u32 = 126;
    pub const ERROR_PROC_NOT_FOUND: u32 = 127;
    pub const ERROR_NEGATIVE_SEEK: u32 = 131;
    pub const ERROR_DIR_NOT_EMPTY: u32 = 145;
    pub const ERROR_BAD_ARGUMENTS: u32 = 160;
    pub const ERROR_ALREADY_EXISTS: u32 = 183;
    pub const ERROR_ENVVAR_NOT_FOUND: u32 = 203;
    pub const ERROR_MORE_DATA: u32 = 234;
    pub const ERROR_NO_MORE_ITEMS: u32 = 259;
    pub const ERROR_NOT_OWNER: u32 = 288;
    pub const ERROR_TOO_MANY_POSTS: u32 = 298;
    pub const ERROR_PARTIAL_COPY: u32 = 299;
    pub const ERROR_INVALID_ADDRESS: u32 = 487;
    pub const ERROR_IO_INCOMPLETE: u32 = 996;
    pub const ERROR_IO_PENDING: u32 = 997;
    pub const ERROR_NOACCESS: u32 = 998;
    pub const ERROR_CANT_ACCESS_FILE: u32 = 1920;
    pub const ERROR_TIMEOUT: u32 = 1460;
    pub const ERROR_RESOURCE_DATA_NOT_FOUND: u32 = 1812;
    pub const ERROR_RESOURCE_TYPE_NOT_FOUND: u32 = 1813;
    pub const ERROR_RESOURCE_NAME_NOT_FOUND: u32 = 1814;
    pub const ERROR_NOT_FOUND: u32 = 1168;
    pub const ERROR_INVALID_FLAGS: u32 = 1004;
    pub const ERROR_NO_UNICODE_TRANSLATION: u32 = 1113;
    pub const ERROR_INVALID_WINDOW_HANDLE: u32 = 1400;
    pub const ERROR_CLASS_ALREADY_EXISTS: u32 = 1410;
    pub const ERROR_DLL_INIT_FAILED: u32 = 1114;
}

/// Map a host `errno` to the closest Win32 error.
pub fn errno_to_win32(e: i32) -> u32 {
    use err::*;
    match e {
        libc::ENOENT => ERROR_FILE_NOT_FOUND,
        libc::ENOTDIR => ERROR_PATH_NOT_FOUND,
        libc::EACCES | libc::EPERM | libc::EROFS => ERROR_ACCESS_DENIED,
        libc::EEXIST => ERROR_ALREADY_EXISTS,
        libc::EBADF => ERROR_INVALID_HANDLE,
        libc::ENOMEM => ERROR_NOT_ENOUGH_MEMORY,
        libc::EINVAL => ERROR_INVALID_PARAMETER,
        libc::EMFILE | libc::ENFILE => ERROR_TOO_MANY_OPEN_FILES,
        libc::ENOTEMPTY => ERROR_DIR_NOT_EMPTY,
        libc::EPIPE => ERROR_BROKEN_PIPE,
        libc::ENAMETOOLONG => ERROR_INVALID_NAME,
        libc::EISDIR => ERROR_ACCESS_DENIED,
        libc::ENOSPC => 112, // ERROR_DISK_FULL
        _ => ERROR_INVALID_FUNCTION,
    }
}

pub fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}

impl FILETIME {
    pub fn from_u64(v: u64) -> Self {
        FILETIME { dwLowDateTime: v as u32, dwHighDateTime: (v >> 32) as u32 }
    }
    pub fn to_u64(self) -> u64 {
        (self.dwHighDateTime as u64) << 32 | self.dwLowDateTime as u64
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
            self.data1, self.data2, self.data3, self.data4[0], self.data4[1], self.data4[2],
            self.data4[3], self.data4[4], self.data4[5], self.data4[6], self.data4[7]
        )
    }
}

/// 100ns ticks between 1601-01-01 and 1970-01-01.
pub const EPOCH_DIFF_100NS: u64 = 116_444_736_000_000_000;

pub fn unix_to_filetime(secs: i64, nanos: i64) -> u64 {
    (secs as i128 * 10_000_000 + (nanos as i128) / 100 + EPOCH_DIFF_100NS as i128) as u64
}

pub fn now_filetime() -> u64 {
    if let Some(v) = crate::shadow::frozen_filetime() {
        return v;
    }
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts) };
    unix_to_filetime(ts.tv_sec, ts.tv_nsec)
}
