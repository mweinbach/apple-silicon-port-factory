//! ws2_32.dll: Winsock2 over POSIX / BSD sockets.
//!
//! Translates Windows address families (`AF_INET6 = 23` <-> Darwin `30`),
//! `sockaddr_in`/`sockaddr_in6` headers, `ADDRINFOA`/`ADDRINFOW` layouts,
//! socket options (`SOL_SOCKET`, `IPPROTO_TCP`, `IPPROTO_IPV6`, timeouts, linger),
//! `ioctlsocket`/`WSAIoctl`, `select`/`WSAPoll`, and errno -> WSAE* error codes.

use crate::kernel32::file::{
    complete_overlapped_io, external_io_event, get_file, load_ov_internal, load_ov_internal_high,
    prepare_overlapped_pending, register_external_cancel_hook, register_external_io_handle,
    store_ov_internal, store_ov_internal_high, unregister_external_io_handle, win32_to_ntstatus,
    INVALID_HANDLE_VALUE, OVERLAPPED, STATUS_CANCELLED, STATUS_PENDING, STATUS_SUCCESS,
};
use crate::types::*;
use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::os::unix::fs::FileExt;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Condvar, LazyLock, Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (1, "accept"),
    (2, "bind"),
    (3, "closesocket"),
    (4, "connect"),
    (5, "getpeername"),
    (6, "getsockname"),
    (7, "getsockopt"),
    (8, "htonl"),
    (9, "htons"),
    (10, "ioctlsocket"),
    (11, "inet_addr"),
    (12, "inet_ntoa"),
    (13, "listen"),
    (14, "ntohl"),
    (15, "ntohs"),
    (16, "recv"),
    (17, "recvfrom"),
    (18, "select"),
    (19, "send"),
    (20, "sendto"),
    (21, "setsockopt"),
    (22, "shutdown"),
    (23, "socket"),
    (51, "gethostbyaddr"),
    (52, "gethostbyname"),
    (53, "getprotobyname"),
    (57, "gethostname"),
    (111, "WSAGetLastError"),
    (112, "WSASetLastError"),
    (115, "WSAStartup"),
    (116, "WSACleanup"),
    (151, "__WSAFDIsSet"),
    (1142, "getsockopt"),
];

const WSA_OPERATION_ABORTED: u32 = 995;
const WSA_IO_INCOMPLETE: u32 = 996;
const WSA_IO_PENDING: u32 = 997;

const WSAEINTR: u32 = 10004;
const WSAEBADF: u32 = 10009;
const WSAEACCES: u32 = 10013;
const WSAEFAULT: u32 = 10014;
const WSAEINVAL: u32 = 10022;
const WSAEMFILE: u32 = 10024;
const WSAEWOULDBLOCK: u32 = 10035;
const WSAEALREADY: u32 = 10037;
const WSAENOTSOCK: u32 = 10038;
const WSAEDESTADDRREQ: u32 = 10039;
const WSAEMSGSIZE: u32 = 10040;
const WSAEPROTOTYPE: u32 = 10041;
const WSAENOPROTOOPT: u32 = 10042;
const WSAEPROTONOSUPPORT: u32 = 10043;
const WSAEOPNOTSUPP: u32 = 10045;
const WSAEAFNOSUPPORT: u32 = 10047;
const WSAEADDRINUSE: u32 = 10048;
const WSAEADDRNOTAVAIL: u32 = 10049;
const WSAENETDOWN: u32 = 10050;
const WSAENETUNREACH: u32 = 10051;
const WSAECONNABORTED: u32 = 10053;
const WSAECONNRESET: u32 = 10054;
const WSAENOBUFS: u32 = 10055;
const WSAEISCONN: u32 = 10056;
const WSAENOTCONN: u32 = 10057;
const WSAESHUTDOWN: u32 = 10058;
const WSAETIMEDOUT: u32 = 10060;
const WSAECONNREFUSED: u32 = 10061;
const WSAEHOSTDOWN: u32 = 10064;
const WSAEHOSTUNREACH: u32 = 10065;
const WSAVERNOTSUPPORTED: u32 = 10092;
const WSANOTINITIALISED: u32 = 10093;
const WSAHOST_NOT_FOUND: u32 = 11001;
const WSANO_DATA: u32 = 11004;

const AF_UNSPEC: u16 = 0;
const AF_INET: u16 = 2;
const AF_INET6: u16 = 23;
const SOCKET_ERROR: i32 = -1;
const INVALID_SOCKET: usize = usize::MAX;

const TF_DISCONNECT: u32 = 0x01;
const TF_REUSE_SOCKET: u32 = 0x02;
const WSA_FLAG_NO_HANDLE_INHERIT: u32 = 0x80;

#[derive(Clone, Copy)]
struct SocketMeta {
    fd: i32,
    family: i32,
    sock_type: i32,
    protocol: i32,
    bound: bool,
    listening: bool,
    connecting: bool,
    connected: bool,
    /// Guest-visible nonblocking state (`ioctlsocket(FIONBIO)` / `WSAEventSelect`).
    /// The underlying POSIX fd is kept permanently `O_NONBLOCK` so overlapped and
    /// reactor I/O never block in Darwin XNU `sosend` and never toggle `F_SETFL`
    /// concurrently with other operations.
    guest_nonblocking: bool,
    handle_flags: u32,
}

static STARTUPS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static NEXT_SOCK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x10000);
static SOCKETS: LazyLock<Mutex<HashMap<usize, SocketMeta>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
const DUP_SOCK_MAGIC_MASK: u32 = 0xD05C_0000;
static NEXT_DUP_TOKEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
static DUP_SOCKETS: LazyLock<Mutex<HashMap<u32, (i32, i32, i32, i32, bool, bool, bool)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static CANCEL_HOOK_INIT: OnceLock<()> = OnceLock::new();

fn errno_to_wsa(e: i32) -> u32 {
    match e {
        libc::EINTR => WSAEINTR,
        libc::EBADF => WSAEBADF,
        libc::EACCES => WSAEACCES,
        libc::EFAULT => WSAEFAULT,
        libc::EINVAL => WSAEINVAL,
        libc::EMFILE | libc::ENFILE => WSAEMFILE,
        libc::EAGAIN => WSAEWOULDBLOCK,
        libc::EINPROGRESS => WSAEWOULDBLOCK,
        libc::EALREADY => WSAEALREADY,
        libc::ENOTSOCK => WSAENOTSOCK,
        libc::EDESTADDRREQ => WSAEDESTADDRREQ,
        libc::EMSGSIZE => WSAEMSGSIZE,
        libc::EPROTOTYPE => WSAEPROTOTYPE,
        libc::ENOPROTOOPT => WSAENOPROTOOPT,
        libc::EPROTONOSUPPORT => WSAEPROTONOSUPPORT,
        libc::EOPNOTSUPP => WSAEOPNOTSUPP,
        libc::EAFNOSUPPORT => WSAEAFNOSUPPORT,
        libc::EADDRINUSE => WSAEADDRINUSE,
        libc::EADDRNOTAVAIL => WSAEADDRNOTAVAIL,
        libc::ENETDOWN => WSAENETDOWN,
        libc::ENETUNREACH => WSAENETUNREACH,
        libc::ECONNABORTED => WSAECONNABORTED,
        libc::ECONNRESET => WSAECONNRESET,
        libc::ENOBUFS | libc::ENOMEM => WSAENOBUFS,
        libc::EISCONN => WSAEISCONN,
        libc::ENOTCONN => WSAENOTCONN,
        libc::ESHUTDOWN => WSAESHUTDOWN,
        libc::ETIMEDOUT => WSAETIMEDOUT,
        libc::ECONNREFUSED => WSAECONNREFUSED,
        libc::EHOSTDOWN => WSAEHOSTDOWN,
        libc::EHOSTUNREACH => WSAEHOSTUNREACH,
        _ => WSAEINVAL,
    }
}

fn last_errno_wsa() -> u32 {
    let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
    errno_to_wsa(e)
}

fn win_af_to_posix(af: i32) -> Option<i32> {
    match af as u16 {
        AF_UNSPEC => Some(libc::AF_UNSPEC),
        AF_INET => Some(libc::AF_INET),
        AF_INET6 => Some(libc::AF_INET6),
        _ => None,
    }
}

fn posix_af_to_win(af: i32) -> u16 {
    if af == libc::AF_INET {
        AF_INET
    } else if af == libc::AF_INET6 {
        AF_INET6
    } else {
        AF_UNSPEC
    }
}

fn lookup_meta(s: usize) -> Option<SocketMeta> {
    SOCKETS.lock().unwrap().get(&s).copied()
}

fn lookup_fd(s: usize) -> Option<i32> {
    lookup_meta(s).map(|m| m.fd)
}

fn ntstatus_to_wsa(status: usize) -> u32 {
    match status as u32 {
        0 => 0,
        0x0000_0103 => WSA_IO_INCOMPLETE,
        0xC000_0120 => WSA_OPERATION_ABORTED,
        0xC000_020D => WSAECONNRESET,
        0xC000_0241 => WSAECONNABORTED,
        0xC000_0236 => WSAECONNREFUSED,
        0xC000_00B5 => WSAETIMEDOUT,
        0xC000_0128 => WSAENOTCONN,
        0xC000_013C => WSAESHUTDOWN,
        0x8000_0005 => WSAEMSGSIZE,
        0xC000_0008 => WSAENOTSOCK,
        0xC000_000D => WSAEINVAL,
        0xC000_0022 => WSAEACCES,
        s => crate::ntdll::ntstatus_to_win32(s),
    }
}

fn set_nosigpipe(fd: i32) {
    let one: i32 = 1;
    unsafe {
        libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_NOSIGPIPE,
            &one as *const i32 as *const c_void,
            std::mem::size_of::<i32>() as libc::socklen_t,
        );
    }
}

fn set_fd_nonblocking(fd: i32) {
    unsafe {
        let fl = libc::fcntl(fd, libc::F_GETFL);
        if fl >= 0 && (fl & libc::O_NONBLOCK) == 0 {
            let _ = libc::fcntl(fd, libc::F_SETFL, fl | libc::O_NONBLOCK);
        }
    }
}

fn is_guest_nonblocking(s: usize) -> bool {
    lookup_meta(s).is_some_and(|m| m.guest_nonblocking)
}

unsafe fn socket_timeout_duration(fd: i32, opt: libc::c_int) -> Option<std::time::Duration> {
    let mut tv: libc::timeval = std::mem::zeroed();
    let mut len = std::mem::size_of::<libc::timeval>() as libc::socklen_t;
    if libc::getsockopt(
        fd,
        libc::SOL_SOCKET,
        opt,
        &mut tv as *mut libc::timeval as *mut c_void,
        &mut len,
    ) != 0
    {
        return None;
    }
    if tv.tv_sec <= 0 && tv.tv_usec <= 0 {
        None
    } else {
        Some(std::time::Duration::new(
            tv.tv_sec.max(0) as u64,
            (tv.tv_usec.max(0) as u32) * 1000,
        ))
    }
}

/// Wait for `events` (`POLLIN` / `POLLOUT`) on an always-`O_NONBLOCK` socket to emulate
/// guest blocking mode, respecting `SO_RCVTIMEO` / `SO_SNDTIMEO` when `timeout_opt` is given.
unsafe fn wait_socket_ready(fd: i32, events: i16, timeout_opt: Option<libc::c_int>) -> Result<(), i32> {
    let deadline = timeout_opt
        .and_then(|opt| socket_timeout_duration(fd, opt))
        .map(|d| std::time::Instant::now() + d);
    loop {
        let poll_ms: libc::c_int = match deadline {
            None => -1,
            Some(dl) => {
                let now = std::time::Instant::now();
                if now >= dl {
                    return Err(libc::ETIMEDOUT);
                }
                (dl - now).as_millis().clamp(1, i32::MAX as u128) as libc::c_int
            }
        };
        let mut pfd = libc::pollfd {
            fd,
            events,
            revents: 0,
        };
        let pr = libc::poll(&mut pfd, 1, poll_ms);
        if pr < 0 {
            let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
            if e == libc::EINTR {
                continue;
            }
            return Err(e);
        }
        if pr == 0 {
            return Err(libc::ETIMEDOUT);
        }
        return Ok(());
    }
}

fn register_fd_with_mode(
    fd: i32,
    family: i32,
    sock_type: i32,
    protocol: i32,
    bound: bool,
    connected: bool,
    guest_nonblocking: bool,
) -> usize {
    // Suppress SIGPIPE on Darwin sockets and keep the underlying fd permanently O_NONBLOCK.
    set_nosigpipe(fd);
    set_fd_nonblocking(fd);
    CANCEL_HOOK_INIT.get_or_init(|| {
        register_external_cancel_hook(ws2_cancel_io_hook);
    });
    let h = NEXT_SOCK.fetch_add(4, Ordering::Relaxed);
    SOCKETS.lock().unwrap().insert(
        h,
        SocketMeta {
            fd,
            family,
            sock_type,
            protocol,
            bound,
            listening: false,
            connecting: false,
            connected,
            guest_nonblocking,
            handle_flags: crate::handles::HANDLE_FLAG_INHERIT,
        },
    );
    register_external_io_handle(h as HANDLE);
    h
}

fn register_fd(fd: i32, family: i32, sock_type: i32, protocol: i32, bound: bool, connected: bool) -> usize {
    register_fd_with_mode(fd, family, sock_type, protocol, bound, connected, false)
}

pub(crate) fn set_socket_handle_flags(h: HANDLE, mask: u32, flags: u32) -> bool {
    let mut socks = SOCKETS.lock().unwrap();
    let Some(meta) = socks.get_mut(&(h as usize)) else {
        return false;
    };
    meta.handle_flags = (meta.handle_flags & !mask) | (flags & mask);
    if mask & crate::handles::HANDLE_FLAG_INHERIT != 0 {
        let cloexec = if (meta.handle_flags & crate::handles::HANDLE_FLAG_INHERIT) == 0 {
            libc::FD_CLOEXEC
        } else {
            0
        };
        unsafe {
            let cur = libc::fcntl(meta.fd, libc::F_GETFD);
            if cur >= 0 {
                let next = (cur & !libc::FD_CLOEXEC) | cloexec;
                libc::fcntl(meta.fd, libc::F_SETFD, next);
            }
        }
    }
    true
}

pub(crate) fn get_socket_handle_flags(h: HANDLE) -> Option<u32> {
    SOCKETS.lock().unwrap().get(&(h as usize)).map(|m| m.handle_flags)
}

/// Convert a Windows `sockaddr` buffer into a Darwin `sockaddr_storage` + length.
unsafe fn win_to_posix_sockaddr(sa: *const u8, len: i32) -> Result<(libc::sockaddr_storage, libc::socklen_t), u32> {
    if sa.is_null() || len < 2 {
        return Err(WSAEFAULT);
    }
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let family = (sa as *const u16).read_unaligned();
    match family {
        AF_INET => {
            if len < 16 {
                return Err(WSAEINVAL);
            }
            let dst = &mut storage as *mut libc::sockaddr_storage as *mut u8;
            *dst = 16; // sin_len
            *dst.add(1) = libc::AF_INET as u8;
            std::ptr::copy_nonoverlapping(sa.add(2), dst.add(2), 14);
            Ok((storage, 16))
        }
        AF_INET6 => {
            if len < 28 {
                return Err(WSAEINVAL);
            }
            let dst = &mut storage as *mut libc::sockaddr_storage as *mut u8;
            *dst = 28; // sin6_len
            *dst.add(1) = libc::AF_INET6 as u8;
            std::ptr::copy_nonoverlapping(sa.add(2), dst.add(2), 26);
            Ok((storage, 28))
        }
        _ => Err(WSAEAFNOSUPPORT),
    }
}

/// Write a Darwin `sockaddr` into a Windows `sockaddr` buffer.
unsafe fn posix_to_win_sockaddr(src: *const libc::sockaddr, src_len: libc::socklen_t, dst: *mut u8, dst_len: *mut i32) -> Result<(), u32> {
    if dst.is_null() || dst_len.is_null() {
        return Err(WSAEFAULT);
    }
    let af = (*src).sa_family as i32;
    let src_bytes = src as *const u8;
    let avail = std::ptr::read_unaligned(dst_len);
    if af == libc::AF_INET && src_len >= 16 {
        if avail < 16 {
            std::ptr::write_unaligned(dst_len, 16);
            return Err(WSAEFAULT);
        }
        std::ptr::write_bytes(dst, 0, 16);
        (dst as *mut u16).write_unaligned(AF_INET);
        std::ptr::copy_nonoverlapping(src_bytes.add(2), dst.add(2), 14);
        std::ptr::write_unaligned(dst_len, 16);
        Ok(())
    } else if af == libc::AF_INET6 && src_len >= 28 {
        if avail < 28 {
            std::ptr::write_unaligned(dst_len, 28);
            return Err(WSAEFAULT);
        }
        std::ptr::write_bytes(dst, 0, 28);
        (dst as *mut u16).write_unaligned(AF_INET6);
        std::ptr::copy_nonoverlapping(src_bytes.add(2), dst.add(2), 26);
        std::ptr::write_unaligned(dst_len, 28);
        Ok(())
    } else {
        Err(WSAEAFNOSUPPORT)
    }
}

fn err<T>(e: u32, v: T) -> T {
    crate::teb::set_last_error(e);
    v
}

// ---------------------------------------------------------------- startup

/// WSADATA (x64): wVersion@0 wHighVersion@2 iMaxSockets@4 iMaxUdpDg@6
/// lpVendorInfo@8 szDescription[257]@16 szSystemStatus[129]@273, size 408.
pub unsafe extern "win64" fn WSAStartup(ver: u16, data: *mut u8) -> i32 {
    if data.is_null() {
        return WSAEFAULT as i32;
    }
    std::ptr::write_bytes(data, 0, 408);
    let (major, minor) = (ver & 0xFF, ver >> 8);
    let negotiated: u16 = if major > 2 || (major == 2 && minor >= 2) { 0x0202 } else { ver };
    (data as *mut u16).write_unaligned(negotiated);
    (data.add(2) as *mut u16).write_unaligned(0x0202);
    let desc = b"WinSock 2.0";
    std::ptr::copy_nonoverlapping(desc.as_ptr(), data.add(16), desc.len());
    let status = b"Running";
    std::ptr::copy_nonoverlapping(status.as_ptr(), data.add(273), status.len());
    if major < 1 || (major == 1 && minor < 1) {
        return WSAVERNOTSUPPORTED as i32;
    }
    STARTUPS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    0
}

pub unsafe extern "win64" fn WSACleanup() -> i32 {
    let prev = STARTUPS.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    if prev <= 0 {
        STARTUPS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        return err(WSANOTINITIALISED, SOCKET_ERROR);
    }
    0
}

pub unsafe extern "win64" fn WSAGetLastError() -> i32 {
    crate::teb::last_error() as i32
}

pub unsafe extern "win64" fn WSASetLastError(e: i32) {
    crate::teb::set_last_error(e as u32);
}

// ------------------------------------------------------------- byte order

pub unsafe extern "win64" fn htonl(v: u32) -> u32 {
    v.to_be()
}
pub unsafe extern "win64" fn ntohl(v: u32) -> u32 {
    u32::from_be(v)
}
pub unsafe extern "win64" fn htons(v: u16) -> u16 {
    v.to_be()
}
pub unsafe extern "win64" fn ntohs(v: u16) -> u16 {
    u16::from_be(v)
}

// -------------------------------------------------------- address strings

/// Classic inet_addr: dotted quad with 1-4 parts, decimal/octal/hex.
fn parse_inet_addr(s: &str) -> Option<u32> {
    let s = s.trim_end_matches(|c: char| c.is_ascii_whitespace());
    let parts: Vec<&str> = s.split('.').collect();
    if parts.is_empty() || parts.len() > 4 {
        return None;
    }
    let mut vals = Vec::new();
    for p in &parts {
        let v = if let Some(h) = p.strip_prefix("0x").or_else(|| p.strip_prefix("0X")) {
            u32::from_str_radix(h, 16).ok()?
        } else if p.len() > 1 && p.starts_with('0') {
            u32::from_str_radix(&p[1..], 8).ok()?
        } else {
            p.parse::<u32>().ok()?
        };
        vals.push(v);
    }
    let n = vals.len();
    for v in &vals[..n - 1] {
        if *v > 255 {
            return None;
        }
    }
    let last_max: u64 = [0xFFFF_FFFF, 0xFF_FFFF, 0xFFFF, 0xFF][n - 1];
    if vals[n - 1] as u64 > last_max {
        return None;
    }
    let mut a: u32 = 0;
    for (i, v) in vals[..n - 1].iter().enumerate() {
        a |= v << (24 - 8 * i);
    }
    Some(a | vals[n - 1])
}

pub unsafe extern "win64" fn inet_addr(s: *const u8) -> u32 {
    if s.is_null() {
        return u32::MAX;
    }
    match parse_inet_addr(&crate::strings::from_ansi(s)) {
        Some(a) => a.to_be(),
        None => u32::MAX, // INADDR_NONE
    }
}

thread_local! {
    static NTOA_BUF: std::cell::RefCell<[u8; 16]> = const { std::cell::RefCell::new([0; 16]) };
}

/// `inet_ntoa(struct in_addr)` — the 4-byte struct is passed by value in ecx.
pub unsafe extern "win64" fn inet_ntoa(addr: u32) -> *mut u8 {
    let s = Ipv4Addr::from(u32::from_be(addr)).to_string();
    NTOA_BUF.with(|b| {
        let mut b = b.borrow_mut();
        b.fill(0);
        b[..s.len()].copy_from_slice(s.as_bytes());
        b.as_mut_ptr()
    })
}

pub unsafe extern "win64" fn inet_pton(family: i32, s: *const u8, dst: *mut u8) -> i32 {
    if s.is_null() || dst.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let text = crate::strings::from_ansi(s);
    match family as u16 {
        AF_INET => match text.parse::<Ipv4Addr>() {
            Ok(a) => {
                std::ptr::copy_nonoverlapping(a.octets().as_ptr(), dst, 4);
                1
            }
            Err(_) => 0,
        },
        AF_INET6 => match text.parse::<Ipv6Addr>() {
            Ok(a) => {
                std::ptr::copy_nonoverlapping(a.octets().as_ptr(), dst, 16);
                1
            }
            Err(_) => 0,
        },
        _ => err(WSAEAFNOSUPPORT, SOCKET_ERROR),
    }
}

pub unsafe extern "win64" fn inet_ntop(family: i32, src: *const u8, dst: *mut u8, size: usize) -> *const u8 {
    if src.is_null() || dst.is_null() {
        return err(WSAEFAULT, std::ptr::null());
    }
    let s = match family as u16 {
        AF_INET => Ipv4Addr::from(*(src as *const [u8; 4])).to_string(),
        AF_INET6 => Ipv6Addr::from(*(src as *const [u8; 16])).to_string(),
        _ => return err(WSAEAFNOSUPPORT, std::ptr::null()),
    };
    if s.len() + 1 > size {
        return err(WSAEINVAL, std::ptr::null());
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), dst, s.len());
    *dst.add(s.len()) = 0;
    dst
}

/// sockaddr -> "a.b.c.d[:port]" / "[v6]:port".
unsafe fn sockaddr_to_string(sa: *const u8, len: u32) -> Option<String> {
    if sa.is_null() || len < 2 {
        return None;
    }
    let family = (sa as *const u16).read_unaligned();
    let port = u16::from_be((sa.add(2) as *const u16).read_unaligned());
    match family {
        AF_INET if len >= 16 => {
            let a = Ipv4Addr::from(*(sa.add(4) as *const [u8; 4]));
            Some(if port != 0 { format!("{a}:{port}") } else { a.to_string() })
        }
        AF_INET6 if len >= 28 => {
            let a = Ipv6Addr::from(*(sa.add(8) as *const [u8; 16]));
            let scope = (sa.add(24) as *const u32).read_unaligned();
            let mut s = a.to_string();
            if scope != 0 {
                s = format!("{s}%{scope}");
            }
            Some(if port != 0 { format!("[{s}]:{port}") } else { s })
        }
        _ => None,
    }
}

pub unsafe extern "win64" fn WSAAddressToStringW(sa: *const u8, len: u32, _proto: *mut c_void, out: *mut u16, out_len: *mut u32) -> i32 {
    if out_len.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let Some(s) = sockaddr_to_string(sa, len) else {
        return err(WSAEINVAL, SOCKET_ERROR);
    };
    let w = crate::strings::to_wide_z(&s);
    if out.is_null() || (*out_len as usize) < w.len() {
        *out_len = w.len() as u32;
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), out, w.len());
    *out_len = w.len() as u32;
    0
}

pub unsafe extern "win64" fn WSAStringToAddressW(s: *const u16, family: i32, _proto: *mut c_void, sa: *mut u8, len: *mut i32) -> i32 {
    if s.is_null() || sa.is_null() || len.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let text = crate::strings::from_wide(s);
    match family as u16 {
        AF_INET => {
            if *len < 16 {
                *len = 16;
                return err(WSAEFAULT, SOCKET_ERROR);
            }
            let parsed = text.parse::<std::net::SocketAddrV4>().ok().map(|a| (*a.ip(), a.port())).or_else(|| text.parse::<Ipv4Addr>().ok().map(|a| (a, 0)));
            let Some((ip, port)) = parsed else {
                return err(WSAEINVAL, SOCKET_ERROR);
            };
            std::ptr::write_bytes(sa, 0, 16);
            (sa as *mut u16).write_unaligned(AF_INET);
            (sa.add(2) as *mut u16).write_unaligned(port.to_be());
            std::ptr::copy_nonoverlapping(ip.octets().as_ptr(), sa.add(4), 4);
            *len = 16;
            0
        }
        AF_INET6 => {
            if *len < 28 {
                *len = 28;
                return err(WSAEFAULT, SOCKET_ERROR);
            }
            let parsed = text
                .parse::<std::net::SocketAddrV6>()
                .ok()
                .map(|a| (*a.ip(), a.port(), a.scope_id()))
                .or_else(|| text.parse::<Ipv6Addr>().ok().map(|a| (a, 0, 0)));
            let Some((ip, port, scope)) = parsed else {
                return err(WSAEINVAL, SOCKET_ERROR);
            };
            std::ptr::write_bytes(sa, 0, 28);
            (sa as *mut u16).write_unaligned(AF_INET6);
            (sa.add(2) as *mut u16).write_unaligned(port.to_be());
            std::ptr::copy_nonoverlapping(ip.octets().as_ptr(), sa.add(8), 16);
            (sa.add(24) as *mut u32).write_unaligned(scope);
            *len = 28;
            0
        }
        _ => err(WSAEINVAL, SOCKET_ERROR),
    }
}

// ------------------------------------------------------------ host names

pub unsafe extern "win64" fn gethostname(buf: *mut u8, len: i32) -> i32 {
    if buf.is_null() || len <= 0 {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let mut tmp = [0u8; 256];
    if libc::gethostname(tmp.as_mut_ptr() as *mut libc::c_char, tmp.len()) != 0 {
        return err(WSAENETDOWN, SOCKET_ERROR);
    }
    let n = tmp.iter().position(|&c| c == 0).unwrap_or(tmp.len());
    if n + 1 > len as usize {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    std::ptr::copy_nonoverlapping(tmp.as_ptr(), buf, n);
    *buf.add(n) = 0;
    0
}

#[repr(C)]
struct WinAddrInfo {
    ai_flags: i32,
    ai_family: i32,
    ai_socktype: i32,
    ai_protocol: i32,
    ai_addrlen: usize,
    ai_canonname: *mut u8,
    ai_addr: *mut u8,
    ai_next: *mut WinAddrInfo,
}

unsafe fn build_win_addrinfo(
    node: Option<String>,
    service: Option<String>,
    hints: *const c_void,
    wide: bool,
) -> Result<*mut WinAddrInfo, u32> {
    let c_node = match node {
        Some(s) if !s.is_empty() => Some(CString::new(s).map_err(|_| WSAEINVAL)?),
        _ => None,
    };
    let c_svc = match service {
        Some(s) if !s.is_empty() => Some(CString::new(s).map_err(|_| WSAEINVAL)?),
        _ => None,
    };
    let mut posix_hints: libc::addrinfo = std::mem::zeroed();
    let hints_ptr = if !hints.is_null() {
        let h = std::ptr::read_unaligned(hints as *const WinAddrInfo);
        let mut pflags = 0i32;
        if (h.ai_flags & 0x01) != 0 {
            pflags |= libc::AI_PASSIVE;
        }
        if (h.ai_flags & 0x02) != 0 {
            pflags |= libc::AI_CANONNAME;
        }
        if (h.ai_flags & 0x04) != 0 {
            pflags |= libc::AI_NUMERICHOST;
        }
        if (h.ai_flags & 0x08) != 0 {
            pflags |= libc::AI_NUMERICSERV;
        }
        posix_hints.ai_flags = pflags;
        posix_hints.ai_family = win_af_to_posix(h.ai_family).ok_or(WSAEAFNOSUPPORT)?;
        posix_hints.ai_socktype = h.ai_socktype;
        posix_hints.ai_protocol = h.ai_protocol;
        &posix_hints as *const libc::addrinfo
    } else {
        std::ptr::null()
    };

    let t0 = std::time::Instant::now();
    log::debug!(
        "getaddrinfo(node={:?}, service={:?}, wide={wide}) starting",
        c_node.as_deref().and_then(|c| c.to_str().ok()),
        c_svc.as_deref().and_then(|c| c.to_str().ok()),
    );
    let mut raw: *mut libc::addrinfo = std::ptr::null_mut();
    let rc = libc::getaddrinfo(
        c_node.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        c_svc.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        hints_ptr,
        &mut raw,
    );
    let elapsed_ms = t0.elapsed().as_millis();
    if elapsed_ms >= 1000 {
        log::warn!(
            "getaddrinfo(node={:?}, service={:?}) slow: {elapsed_ms}ms (rc={rc})",
            c_node.as_deref().and_then(|c| c.to_str().ok()),
            c_svc.as_deref().and_then(|c| c.to_str().ok()),
        );
    } else {
        log::debug!(
            "getaddrinfo(node={:?}, service={:?}) -> rc={rc} in {elapsed_ms}ms",
            c_node.as_deref().and_then(|c| c.to_str().ok()),
            c_svc.as_deref().and_then(|c| c.to_str().ok()),
        );
    }
    if rc != 0 {
        return Err(match rc {
            libc::EAI_FAMILY => WSAEAFNOSUPPORT,
            libc::EAI_SOCKTYPE => WSAEPROTOTYPE,
            libc::EAI_BADFLAGS => WSAEINVAL,
            _ => WSAHOST_NOT_FOUND,
        });
    }

    let mut head: *mut WinAddrInfo = std::ptr::null_mut();
    let mut tail: *mut *mut WinAddrInfo = &mut head;
    let mut cur = raw;
    while !cur.is_null() {
        let ai = &*cur;
        let win_af = posix_af_to_win(ai.ai_family);
        if (win_af == AF_INET || win_af == AF_INET6) && !ai.ai_addr.is_null() {
            let win_len = if win_af == AF_INET { 16usize } else { 28usize };
            let sa_buf = crate::heap::alloc(win_len, true) as *mut u8;
            let mut out_len = win_len as i32;
            if posix_to_win_sockaddr(ai.ai_addr, ai.ai_addrlen, sa_buf, &mut out_len).is_ok() {
                let canon_ptr = if !ai.ai_canonname.is_null() {
                    let s = std::ffi::CStr::from_ptr(ai.ai_canonname).to_string_lossy();
                    if wide {
                        let w = crate::strings::to_wide_z(&s);
                        let p = crate::heap::alloc(w.len() * 2, false) as *mut u16;
                        std::ptr::copy_nonoverlapping(w.as_ptr(), p, w.len());
                        p as *mut u8
                    } else {
                        let b = s.as_bytes();
                        let p = crate::heap::alloc(b.len() + 1, false) as *mut u8;
                        std::ptr::copy_nonoverlapping(b.as_ptr(), p, b.len());
                        *p.add(b.len()) = 0;
                        p
                    }
                } else {
                    std::ptr::null_mut()
                };
                let node_mem = crate::heap::alloc(std::mem::size_of::<WinAddrInfo>(), true) as *mut WinAddrInfo;
                std::ptr::write_unaligned(
                    node_mem,
                    WinAddrInfo {
                        ai_flags: ai.ai_flags,
                        ai_family: win_af as i32,
                        ai_socktype: ai.ai_socktype,
                        ai_protocol: ai.ai_protocol,
                        ai_addrlen: out_len as usize,
                        ai_canonname: canon_ptr,
                        ai_addr: sa_buf,
                        ai_next: std::ptr::null_mut(),
                    },
                );
                *tail = node_mem;
                tail = &mut (*node_mem).ai_next;
            } else {
                crate::heap::free(sa_buf as *mut c_void);
            }
        }
        cur = ai.ai_next;
    }
    libc::freeaddrinfo(raw);
    if head.is_null() {
        return Err(WSAHOST_NOT_FOUND);
    }
    Ok(head)
}

unsafe fn free_win_addrinfo(mut ai: *mut WinAddrInfo) {
    while !ai.is_null() {
        let next = (*ai).ai_next;
        if !(*ai).ai_addr.is_null() {
            crate::heap::free((*ai).ai_addr as *mut c_void);
        }
        if !(*ai).ai_canonname.is_null() {
            crate::heap::free((*ai).ai_canonname as *mut c_void);
        }
        crate::heap::free(ai as *mut c_void);
        ai = next;
    }
}

pub unsafe extern "win64" fn getaddrinfo(node: *const u8, service: *const u8, hints: *const c_void, res: *mut *mut c_void) -> i32 {
    if res.is_null() {
        return err(WSAEFAULT, WSAEFAULT as i32);
    }
    *res = std::ptr::null_mut();
    let n = crate::strings::from_ansi_opt(node);
    let s = crate::strings::from_ansi_opt(service);
    match build_win_addrinfo(n, s, hints, false) {
        Ok(list) => {
            *res = list as *mut c_void;
            0
        }
        Err(e) => err(e, e as i32),
    }
}

pub unsafe extern "win64" fn GetAddrInfoW(node: *const u16, service: *const u16, hints: *const c_void, res: *mut *mut c_void) -> i32 {
    if res.is_null() {
        return err(WSAEFAULT, WSAEFAULT as i32);
    }
    *res = std::ptr::null_mut();
    let n = (!node.is_null()).then(|| crate::strings::from_wide(node));
    let s = (!service.is_null()).then(|| crate::strings::from_wide(service));
    match build_win_addrinfo(n, s, hints, true) {
        Ok(list) => {
            *res = list as *mut c_void;
            0
        }
        Err(e) => err(e, e as i32),
    }
}

pub unsafe extern "win64" fn freeaddrinfo(ai: *mut c_void) {
    free_win_addrinfo(ai as *mut WinAddrInfo);
}

pub unsafe extern "win64" fn FreeAddrInfoW(ai: *mut c_void) {
    free_win_addrinfo(ai as *mut WinAddrInfo);
}

pub unsafe extern "win64" fn getnameinfo(
    sa: *const u8,
    salen: i32,
    host: *mut u8,
    hostlen: u32,
    serv: *mut u8,
    servlen: u32,
    flags: i32,
) -> i32 {
    let (storage, plen) = match win_to_posix_sockaddr(sa, salen) {
        Ok(v) => v,
        Err(e) => return err(e, e as i32),
    };
    let mut pflags = 0i32;
    if (flags & 0x02) != 0 {
        pflags |= libc::NI_NUMERICHOST;
    }
    if (flags & 0x08) != 0 {
        pflags |= libc::NI_NUMERICSERV;
    }
    if (flags & 0x10) != 0 {
        pflags |= libc::NI_DGRAM;
    }
    let rc = libc::getnameinfo(
        &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
        plen,
        host as *mut libc::c_char,
        hostlen,
        serv as *mut libc::c_char,
        servlen,
        pflags,
    );
    if rc == 0 {
        0
    } else {
        err(WSAHOST_NOT_FOUND, WSAHOST_NOT_FOUND as i32)
    }
}

pub unsafe extern "win64" fn GetNameInfoW(
    sa: *const u8,
    salen: i32,
    host: *mut u16,
    hostlen: u32,
    serv: *mut u16,
    servlen: u32,
    flags: i32,
) -> i32 {
    let mut hbuf = [0u8; 1025];
    let mut sbuf = [0u8; 64];
    let rc = getnameinfo(
        sa,
        salen,
        if host.is_null() { std::ptr::null_mut() } else { hbuf.as_mut_ptr() },
        if host.is_null() { 0 } else { hbuf.len() as u32 },
        if serv.is_null() { std::ptr::null_mut() } else { sbuf.as_mut_ptr() },
        if serv.is_null() { 0 } else { sbuf.len() as u32 },
        flags,
    );
    if rc != 0 {
        return rc;
    }
    if !host.is_null() && hostlen > 0 {
        let s = crate::strings::from_ansi(hbuf.as_ptr());
        let w = crate::strings::to_wide_z(&s);
        let n = w.len().min(hostlen as usize);
        std::ptr::copy_nonoverlapping(w.as_ptr(), host, n);
        *host.add(n - 1) = 0;
    }
    if !serv.is_null() && servlen > 0 {
        let s = crate::strings::from_ansi(sbuf.as_ptr());
        let w = crate::strings::to_wide_z(&s);
        let n = w.len().min(servlen as usize);
        std::ptr::copy_nonoverlapping(w.as_ptr(), serv, n);
        *serv.add(n - 1) = 0;
    }
    0
}

// @exports gethostbyname gethostbyaddr
failing!(*mut c_void = std::ptr::null_mut(), err WSAHOST_NOT_FOUND; gethostbyname, gethostbyaddr);

// @exports getprotobyname
failing!(*mut c_void = std::ptr::null_mut(), err WSANO_DATA; getprotobyname);

// --------------------------------------------------------------- sockets

pub unsafe extern "win64" fn socket(af: i32, ty: i32, proto: i32) -> usize {
    let Some(paf) = win_af_to_posix(af) else {
        return err(WSAEAFNOSUPPORT, INVALID_SOCKET);
    };
    let fd = libc::socket(paf, ty, proto);
    if fd < 0 {
        return err(last_errno_wsa(), INVALID_SOCKET);
    }
    register_fd(fd, paf, ty, proto, false, false)
}

unsafe fn try_open_duplicated_socket(info: *const c_void) -> Option<usize> {
    if info.is_null() {
        return None;
    }
    // `dwProviderReserved` sits at offset 112 in both WSAPROTOCOL_INFOA and WSAPROTOCOL_INFOW.
    let token = ((info as *const u8).add(112) as *const u32).read_unaligned();
    if (token & 0xFFFF_0000) != DUP_SOCK_MAGIC_MASK {
        return None;
    }
    let (fd, family, ty, proto, bound, connected, guest_nb) = DUP_SOCKETS.lock().unwrap().remove(&token)?;
    Some(register_fd_with_mode(fd, family, ty, proto, bound, connected, guest_nb))
}

pub unsafe extern "win64" fn WSASocketW(
    af: i32,
    ty: i32,
    proto: i32,
    info: *const c_void,
    _g: u32,
    flags: u32,
) -> usize {
    let s = if let Some(s) = try_open_duplicated_socket(info) {
        s
    } else {
        socket(af, ty, proto)
    };
    if s != INVALID_SOCKET && (flags & WSA_FLAG_NO_HANDLE_INHERIT) != 0 {
        set_socket_handle_flags(s as HANDLE, crate::handles::HANDLE_FLAG_INHERIT, 0);
    }
    s
}

pub unsafe extern "win64" fn WSASocketA(
    af: i32,
    ty: i32,
    proto: i32,
    info: *const c_void,
    _g: u32,
    flags: u32,
) -> usize {
    let s = if let Some(s) = try_open_duplicated_socket(info) {
        s
    } else {
        socket(af, ty, proto)
    };
    if s != INVALID_SOCKET && (flags & WSA_FLAG_NO_HANDLE_INHERIT) != 0 {
        set_socket_handle_flags(s as HANDLE, crate::handles::HANDLE_FLAG_INHERIT, 0);
    }
    s
}

unsafe fn duplicate_socket_into_info(s: usize, pid: u32, info: *mut u8, struct_len: usize) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if info.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    if pid != 0 && pid != std::process::id() {
        stub!("WSADuplicateSocket cross-process target pid={pid} unsupported");
        return err(WSAEOPNOTSUPP, SOCKET_ERROR);
    }
    let dup_fd = libc::dup(meta.fd);
    if dup_fd < 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    let mut sa: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let win_af = if libc::getsockname(dup_fd, &mut sa as *mut _ as *mut libc::sockaddr, &mut slen) == 0 {
        posix_af_to_win(sa.ss_family as i32) as i32
    } else {
        AF_INET as i32
    };
    let proto = if meta.sock_type == libc::SOCK_DGRAM { libc::IPPROTO_UDP } else { libc::IPPROTO_TCP };
    let seq = NEXT_DUP_TOKEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed) & 0xFFFF;
    let token = DUP_SOCK_MAGIC_MASK | seq.max(1);
    DUP_SOCKETS
        .lock()
        .unwrap()
        .insert(
            token,
            (
                dup_fd,
                meta.family,
                meta.sock_type,
                meta.protocol,
                meta.bound,
                meta.connected,
                meta.guest_nonblocking,
            ),
        );

    std::ptr::write_bytes(info, 0, struct_len);
    (info.add(76) as *mut i32).write_unaligned(win_af); // iAddressFamily
    (info.add(80) as *mut i32).write_unaligned(if win_af == AF_INET6 as i32 { 28 } else { 16 }); // iMaxSockAddr
    (info.add(84) as *mut i32).write_unaligned(16); // iMinSockAddr
    (info.add(88) as *mut i32).write_unaligned(meta.sock_type); // iSocketType
    (info.add(92) as *mut i32).write_unaligned(proto); // iProtocol
    (info.add(112) as *mut u32).write_unaligned(token); // dwProviderReserved
    0
}

pub unsafe extern "win64" fn WSADuplicateSocketW(s: usize, pid: u32, info: *mut u8) -> i32 {
    duplicate_socket_into_info(s, pid, info, 628)
}

pub unsafe extern "win64" fn WSADuplicateSocketA(s: usize, pid: u32, info: *mut u8) -> i32 {
    duplicate_socket_into_info(s, pid, info, 372)
}

pub unsafe extern "win64" fn closesocket(s: usize) -> i32 {
    let Some(meta) = SOCKETS.lock().unwrap().remove(&s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    async_io_cancel_socket_and_sync(s);
    event_select_remove_and_sync(s);
    unregister_external_io_handle(s as HANDLE);
    if libc::close(meta.fd) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    0
}

pub unsafe extern "win64" fn bind(s: usize, name: *const u8, namelen: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let (storage, plen) = match win_to_posix_sockaddr(name, namelen) {
        Ok(v) => v,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    if libc::bind(fd, &storage as *const libc::sockaddr_storage as *const libc::sockaddr, plen) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.bound = true;
    }
    0
}

pub unsafe extern "win64" fn listen(s: usize, backlog: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if libc::listen(fd, backlog) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.listening = true;
    }
    event_select_on_listen(s);
    0
}

pub unsafe extern "win64" fn accept(s: usize, addr: *mut u8, addrlen: *mut i32) -> usize {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, INVALID_SOCKET);
    };
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen: libc::socklen_t;
    let cfd = loop {
        slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let r = libc::accept(
            meta.fd,
            &mut storage as *mut libc::sockaddr_storage as *mut libc::sockaddr,
            &mut slen,
        );
        if r >= 0 {
            break r;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready(meta.fd, libc::POLLIN, Some(libc::SO_RCVTIMEO)) {
                event_select_rearm_accept(s);
                return err(errno_to_wsa(we), INVALID_SOCKET);
            }
            continue;
        }
        event_select_rearm_accept(s);
        return err(errno_to_wsa(raw_e), INVALID_SOCKET);
    };
    event_select_rearm_accept(s);
    if !addr.is_null() && !addrlen.is_null() {
        let _ = posix_to_win_sockaddr(&storage as *const libc::sockaddr_storage as *const libc::sockaddr, slen, addr, addrlen);
    }
    let child = register_fd(cfd, meta.family, meta.sock_type, meta.protocol, true, true);
    event_select_inherit_accept(s, child, cfd, meta.sock_type);
    child
}

pub unsafe extern "win64" fn connect(s: usize, name: *const u8, namelen: i32) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let fd = meta.fd;
    let (storage, plen) = match win_to_posix_sockaddr(name, namelen) {
        Ok(v) => v,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    if libc::connect(fd, &storage as *const libc::sockaddr_storage as *const libc::sockaddr, plen) != 0 {
        let raw_errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_errno == libc::EINPROGRESS {
            if is_guest_nonblocking(s) {
                if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
                    m.connecting = true;
                    m.connected = false;
                }
                event_select_on_connect_pending(s);
                return err(WSAEWOULDBLOCK, SOCKET_ERROR);
            }
            // Guest-blocking connect on an always-O_NONBLOCK underlying fd: wait for completion.
            if let Err(we) = wait_socket_ready(fd, libc::POLLOUT, None) {
                let wsa = errno_to_wsa(we);
                event_select_on_connect_complete(s, wsa as i32);
                return err(wsa, SOCKET_ERROR);
            }
            let so_err = query_so_error(fd);
            if so_err != 0 {
                let wsa = errno_to_wsa(so_err);
                if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
                    m.connecting = false;
                    m.connected = false;
                }
                event_select_on_connect_complete(s, wsa as i32);
                return err(wsa, SOCKET_ERROR);
            }
        } else {
            let wsa = errno_to_wsa(raw_errno);
            event_select_on_connect_complete(s, wsa as i32);
            return err(wsa, SOCKET_ERROR);
        }
    }
    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.connecting = false;
        m.connected = true;
    }
    event_select_on_connect_complete(s, 0);
    0
}

pub unsafe extern "win64" fn shutdown(s: usize, how: i32) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if !(0..=2).contains(&how) {
        return err(WSAEINVAL, SOCKET_ERROR);
    }
    if libc::shutdown(meta.fd, how) != 0 {
        let raw = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        // Darwin returns ENOTCONN from shutdown() if the remote peer already closed a
        // previously connected TCP socket; Windows returns 0 in that case (only failing
        // with WSAENOTCONN if the socket was never connected).
        if raw == libc::ENOTCONN && meta.connected {
            return 0;
        }
        return err(errno_to_wsa(raw), SOCKET_ERROR);
    }
    0
}

pub unsafe extern "win64" fn getsockname(s: usize, name: *mut u8, namelen: *mut i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    if libc::getsockname(fd, &mut storage as *mut libc::sockaddr_storage as *mut libc::sockaddr, &mut slen) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    match posix_to_win_sockaddr(&storage as *const libc::sockaddr_storage as *const libc::sockaddr, slen, name, namelen) {
        Ok(()) => 0,
        Err(e) => err(e, SOCKET_ERROR),
    }
}

pub unsafe extern "win64" fn getpeername(s: usize, name: *mut u8, namelen: *mut i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    if libc::getpeername(fd, &mut storage as *mut libc::sockaddr_storage as *mut libc::sockaddr, &mut slen) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    match posix_to_win_sockaddr(&storage as *const libc::sockaddr_storage as *const libc::sockaddr, slen, name, namelen) {
        Ok(()) => 0,
        Err(e) => err(e, SOCKET_ERROR),
    }
}

fn map_sockopt(level: i32, optname: i32) -> Option<(i32, i32)> {
    const WIN_SOL_SOCKET: i32 = 0xFFFF;
    const WIN_IPPROTO_IP: i32 = 0;
    const WIN_IPPROTO_TCP: i32 = 6;
    const WIN_IPPROTO_IPV6: i32 = 41;
    match (level, optname) {
        (WIN_SOL_SOCKET, 0x0004) => Some((libc::SOL_SOCKET, libc::SO_REUSEADDR)),
        (WIN_SOL_SOCKET, 0x0008) => Some((libc::SOL_SOCKET, libc::SO_KEEPALIVE)),
        (WIN_SOL_SOCKET, 0x0010) => Some((libc::SOL_SOCKET, libc::SO_DONTROUTE)),
        (WIN_SOL_SOCKET, 0x0020) => Some((libc::SOL_SOCKET, libc::SO_BROADCAST)),
        (WIN_SOL_SOCKET, 0x0080) => Some((libc::SOL_SOCKET, libc::SO_LINGER)),
        (WIN_SOL_SOCKET, 0x1001) => Some((libc::SOL_SOCKET, libc::SO_SNDBUF)),
        (WIN_SOL_SOCKET, 0x1002) => Some((libc::SOL_SOCKET, libc::SO_RCVBUF)),
        (WIN_SOL_SOCKET, 0x1005) => Some((libc::SOL_SOCKET, libc::SO_SNDTIMEO)),
        (WIN_SOL_SOCKET, 0x1006) => Some((libc::SOL_SOCKET, libc::SO_RCVTIMEO)),
        (WIN_SOL_SOCKET, 0x1007) => Some((libc::SOL_SOCKET, libc::SO_ERROR)),
        (WIN_SOL_SOCKET, 0x1008) => Some((libc::SOL_SOCKET, libc::SO_TYPE)),
        (WIN_IPPROTO_TCP, 1) => Some((libc::IPPROTO_TCP, libc::TCP_NODELAY)),
        (WIN_IPPROTO_IPV6, 27) => Some((libc::IPPROTO_IPV6, libc::IPV6_V6ONLY)),
        (WIN_IPPROTO_IP, 3) => Some((libc::IPPROTO_IP, libc::IP_TOS)),
        (WIN_IPPROTO_IP, 4) => Some((libc::IPPROTO_IP, libc::IP_TTL)),
        _ => None,
    }
}

pub unsafe extern "win64" fn setsockopt(s: usize, level: i32, optname: i32, optval: *const u8, optlen: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    // Windows SO_UPDATE_ACCEPT_CONTEXT (0x700B) / SO_UPDATE_CONNECT_CONTEXT (0x7010).
    // Note: SO_UPDATE_CONNECT_CONTEXT is commonly called with optval=NULL, optlen=0.
    if level == 0xFFFF && optname == 0x700B {
        if optval.is_null() || (optlen as usize) < std::mem::size_of::<usize>() {
            return err(WSAEFAULT, SOCKET_ERROR);
        }
        let listen_sock = std::ptr::read_unaligned(optval as *const usize);
        if lookup_fd(listen_sock).is_none() {
            return err(WSAENOTSOCK, SOCKET_ERROR);
        }
        if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
            m.connected = true;
            m.connecting = false;
            m.bound = true;
        }
        return 0;
    }
    if level == 0xFFFF && optname == 0x7010 {
        if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
            m.connected = true;
            m.connecting = false;
        }
        return 0;
    }
    if optval.is_null() || optlen <= 0 {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    // Windows SO_EXCLUSIVEADDRUSE (-5) / IP_DONTFRAGMENT (14) / IPV6_DONTFRAG (14)
    if (level == 0xFFFF && optname == -5) || (level == 0 && optname == 14) || (level == 41 && optname == 14) {
        return 0;
    }
    let Some((plevel, popt)) = map_sockopt(level, optname) else {
        return err(WSAENOPROTOOPT, SOCKET_ERROR);
    };
    if plevel == libc::SOL_SOCKET && (popt == libc::SO_SNDTIMEO || popt == libc::SO_RCVTIMEO) && optlen >= 4 {
        let ms = std::ptr::read_unaligned(optval as *const u32);
        let tv = libc::timeval { tv_sec: (ms / 1000) as libc::time_t, tv_usec: ((ms % 1000) * 1000) as libc::suseconds_t };
        if libc::setsockopt(fd, plevel, popt, &tv as *const libc::timeval as *const c_void, std::mem::size_of::<libc::timeval>() as libc::socklen_t) != 0 {
            return err(last_errno_wsa(), SOCKET_ERROR);
        }
        return 0;
    }
    if plevel == libc::SOL_SOCKET && popt == libc::SO_LINGER && optlen >= 4 {
        let onoff = std::ptr::read_unaligned(optval as *const u16) as i32;
        let linger = std::ptr::read_unaligned(optval.add(2) as *const u16) as i32;
        let l = libc::linger { l_onoff: onoff, l_linger: linger };
        if libc::setsockopt(fd, plevel, popt, &l as *const libc::linger as *const c_void, std::mem::size_of::<libc::linger>() as libc::socklen_t) != 0 {
            return err(last_errno_wsa(), SOCKET_ERROR);
        }
        return 0;
    }
    if libc::setsockopt(fd, plevel, popt, optval as *const c_void, optlen as libc::socklen_t) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    0
}

pub unsafe extern "win64" fn getsockopt(s: usize, level: i32, optname: i32, optval: *mut u8, optlen: *mut i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if optval.is_null() || optlen.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let Some((plevel, popt)) = map_sockopt(level, optname) else {
        return err(WSAENOPROTOOPT, SOCKET_ERROR);
    };
    let avail = std::ptr::read_unaligned(optlen);
    if plevel == libc::SOL_SOCKET && popt == libc::SO_ERROR && avail >= 4 {
        let mut e: i32 = 0;
        let mut l = std::mem::size_of::<i32>() as libc::socklen_t;
        if libc::getsockopt(fd, plevel, popt, &mut e as *mut i32 as *mut c_void, &mut l) != 0 {
            return err(last_errno_wsa(), SOCKET_ERROR);
        }
        let wsa = if e == 0 { 0i32 } else { errno_to_wsa(e) as i32 };
        std::ptr::write_unaligned(optval as *mut i32, wsa);
        std::ptr::write_unaligned(optlen, 4);
        return 0;
    }
    if plevel == libc::SOL_SOCKET && (popt == libc::SO_SNDTIMEO || popt == libc::SO_RCVTIMEO) && avail >= 4 {
        let mut tv: libc::timeval = std::mem::zeroed();
        let mut l = std::mem::size_of::<libc::timeval>() as libc::socklen_t;
        if libc::getsockopt(fd, plevel, popt, &mut tv as *mut libc::timeval as *mut c_void, &mut l) != 0 {
            return err(last_errno_wsa(), SOCKET_ERROR);
        }
        let ms = (tv.tv_sec as u64 * 1000 + tv.tv_usec as u64 / 1000) as u32;
        std::ptr::write_unaligned(optval as *mut u32, ms);
        std::ptr::write_unaligned(optlen, 4);
        return 0;
    }
    let mut l = avail as libc::socklen_t;
    if libc::getsockopt(fd, plevel, popt, optval as *mut c_void, &mut l) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    std::ptr::write_unaligned(optlen, l as i32);
    0
}

pub unsafe extern "win64" fn ioctlsocket(s: usize, cmd: i32, argp: *mut u32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if argp.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    const WIN_FIONBIO: u32 = 0x8004_667E;
    const WIN_FIONREAD: u32 = 0x4004_667F;
    let mut val = std::ptr::read_unaligned(argp) as i32;
    match cmd as u32 {
        WIN_FIONBIO => {
            if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
                m.guest_nonblocking = val != 0;
            }
            return 0;
        }
        WIN_FIONREAD => {
            if libc::ioctl(fd, libc::FIONREAD, &mut val) != 0 {
                return err(last_errno_wsa(), SOCKET_ERROR);
            }
            std::ptr::write_unaligned(argp, val as u32);
            0
        }
        _ => err(WSAEINVAL, SOCKET_ERROR),
    }
}

pub unsafe extern "win64" fn WSAIoctl(
    s: usize,
    io_control_code: u32,
    in_buf: *mut c_void,
    in_len: u32,
    out_buf: *mut c_void,
    out_len: u32,
    bytes_returned: *mut u32,
    _overlapped: *mut c_void,
    _completion: *mut c_void,
) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    const SIO_GET_EXTENSION_FUNCTION_POINTER: u32 = 0xC800_0006;
    const SIO_UDP_CONNRESET: u32 = 0x9800_000C;
    const SIO_UDP_NETRESET: u32 = 0x9800_000F;
    const SIO_IDEAL_SEND_BACKLOG_QUERY: u32 = 0x4004_747B;
    const WIN_FIONBIO: u32 = 0x8004_667E;
    const WIN_FIONREAD: u32 = 0x4004_667F;
    if !bytes_returned.is_null() {
        std::ptr::write_unaligned(bytes_returned, 0);
    }
    match io_control_code {
        SIO_GET_EXTENSION_FUNCTION_POINTER => {
            if in_buf.is_null() || in_len < 16 || out_buf.is_null() || out_len < 8 {
                return err(WSAEFAULT, SOCKET_ERROR);
            }
            let guid = std::ptr::read_unaligned(in_buf as *const [u8; 16]);
            let Some(ptr) = lookup_extension_fn(&guid) else {
                return err(WSAEOPNOTSUPP, SOCKET_ERROR);
            };
            crate::register_native_va(ptr as u64, ptr);
            std::ptr::write_unaligned(out_buf as *mut usize, ptr);
            if !bytes_returned.is_null() {
                std::ptr::write_unaligned(bytes_returned, std::mem::size_of::<usize>() as u32);
            }
            0
        }
        SIO_UDP_CONNRESET | SIO_UDP_NETRESET => 0,
        SIO_IDEAL_SEND_BACKLOG_QUERY => {
            if out_buf.is_null() || out_len < 4 {
                return err(WSAEFAULT, SOCKET_ERROR);
            }
            let mut sndbuf: i32 = 65536;
            let mut l = std::mem::size_of::<i32>() as libc::socklen_t;
            let _ = libc::getsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_SNDBUF,
                &mut sndbuf as *mut i32 as *mut c_void,
                &mut l,
            );
            let ideal = (sndbuf.max(65536) as u32).min(4 * 1024 * 1024);
            std::ptr::write_unaligned(out_buf as *mut u32, ideal);
            if !bytes_returned.is_null() {
                std::ptr::write_unaligned(bytes_returned, 4);
            }
            0
        }
        WIN_FIONBIO if !in_buf.is_null() && in_len >= 4 => ioctlsocket(s, io_control_code as i32, in_buf as *mut u32),
        WIN_FIONREAD if !out_buf.is_null() && out_len >= 4 => {
            let r = ioctlsocket(s, io_control_code as i32, out_buf as *mut u32);
            if r == 0 && !bytes_returned.is_null() {
                std::ptr::write_unaligned(bytes_returned, 4);
            }
            r
        }
        _ => {
            stub!("WSAIoctl(code={io_control_code:#x}) -> WSAEOPNOTSUPP");
            err(WSAEOPNOTSUPP, SOCKET_ERROR)
        }
    }
}

pub unsafe extern "win64" fn send(s: usize, buf: *const u8, len: i32, flags: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    loop {
        let n = libc::send(fd, buf as *const c_void, len.max(0) as usize, flags);
        if n >= 0 {
            return n as i32;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready(fd, libc::POLLOUT, Some(libc::SO_SNDTIMEO)) {
                return err(errno_to_wsa(we), SOCKET_ERROR);
            }
            continue;
        }
        let wsa = errno_to_wsa(raw_e);
        if wsa == WSAEWOULDBLOCK {
            event_select_rearm_write(s);
        }
        return err(wsa, SOCKET_ERROR);
    }
}

pub unsafe extern "win64" fn sendto(s: usize, buf: *const u8, len: i32, flags: i32, to: *const u8, tolen: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if to.is_null() || tolen == 0 {
        return send(s, buf, len, flags);
    }
    let (storage, plen) = match win_to_posix_sockaddr(to, tolen) {
        Ok(v) => v,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    loop {
        let n = libc::sendto(
            fd,
            buf as *const c_void,
            len.max(0) as usize,
            flags,
            &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
            plen,
        );
        if n >= 0 {
            return n as i32;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready(fd, libc::POLLOUT, Some(libc::SO_SNDTIMEO)) {
                return err(errno_to_wsa(we), SOCKET_ERROR);
            }
            continue;
        }
        let wsa = errno_to_wsa(raw_e);
        if wsa == WSAEWOULDBLOCK {
            event_select_rearm_write(s);
        }
        return err(wsa, SOCKET_ERROR);
    }
}

pub unsafe extern "win64" fn recv(s: usize, buf: *mut u8, len: i32, flags: i32) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let fd = meta.fd;
    let total_want = len.max(0) as usize;
    let wait_all = (flags & libc::MSG_WAITALL) != 0
        && meta.sock_type == libc::SOCK_STREAM
        && !meta.guest_nonblocking
        && (flags & libc::MSG_PEEK) == 0;
    let mut got_total = 0usize;
    loop {
        let ptr = buf.add(got_total) as *mut c_void;
        let rem = total_want - got_total;
        let n = libc::recv(fd, ptr, rem, flags & !libc::MSG_WAITALL);
        if n >= 0 {
            let k = n as usize;
            got_total += k;
            if !wait_all || k == 0 || got_total >= total_want {
                event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
                return got_total as i32;
            }
            continue;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready(fd, libc::POLLIN, Some(libc::SO_RCVTIMEO)) {
                event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
                if got_total > 0 {
                    return got_total as i32;
                }
                return err(errno_to_wsa(we), SOCKET_ERROR);
            }
            continue;
        }
        event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
        if got_total > 0 {
            return got_total as i32;
        }
        return err(errno_to_wsa(raw_e), SOCKET_ERROR);
    }
}

pub unsafe extern "win64" fn recvfrom(
    s: usize,
    buf: *mut u8,
    len: i32,
    flags: i32,
    from: *mut u8,
    fromlen: *mut i32,
) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen: libc::socklen_t;
    let n = loop {
        slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let r = libc::recvfrom(
            fd,
            buf as *mut c_void,
            len.max(0) as usize,
            flags,
            if from.is_null() { std::ptr::null_mut() } else { &mut storage as *mut libc::sockaddr_storage as *mut libc::sockaddr },
            if from.is_null() { std::ptr::null_mut() } else { &mut slen },
        );
        if r >= 0 {
            break r;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready(fd, libc::POLLIN, Some(libc::SO_RCVTIMEO)) {
                event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
                return err(errno_to_wsa(we), SOCKET_ERROR);
            }
            continue;
        }
        event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
        return err(errno_to_wsa(raw_e), SOCKET_ERROR);
    };
    event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
    if !from.is_null() && !fromlen.is_null() {
        let _ = posix_to_win_sockaddr(&storage as *const libc::sockaddr_storage as *const libc::sockaddr, slen, from, fromlen);
    }
    n as i32
}

#[repr(C)]
pub struct WsaBuf {
    pub len: u32,
    pub buf: *mut u8,
}

fn win_msg_flags_to_posix(flags: u32) -> i32 {
    let mut out = 0i32;
    if (flags & 0x1) != 0 {
        out |= libc::MSG_OOB;
    }
    if (flags & 0x2) != 0 {
        out |= libc::MSG_PEEK;
    }
    if (flags & 0x4) != 0 {
        out |= libc::MSG_DONTROUTE;
    }
    if (flags & 0x8) != 0 {
        out |= libc::MSG_WAITALL;
    }
    out
}

unsafe fn collect_wsa_bufs(buffers: *const WsaBuf, count: u32) -> Result<Vec<(usize, usize)>, u32> {
    if buffers.is_null() || count == 0 {
        return Err(WSAEFAULT);
    }
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        let wb = std::ptr::read_unaligned(buffers.add(i));
        if wb.len > 0 && wb.buf.is_null() {
            return Err(WSAEFAULT);
        }
        out.push((wb.buf as usize, wb.len as usize));
    }
    Ok(out)
}

unsafe fn sendmsg_bufs_from(
    fd: i32,
    bufs: &[(usize, usize)],
    offset: usize,
    pflags: i32,
    to: Option<&(libc::sockaddr_storage, libc::socklen_t)>,
) -> isize {
    let mut iovecs: Vec<libc::iovec> = Vec::with_capacity(bufs.len());
    let mut skip = offset;
    for &(addr, len) in bufs {
        if skip >= len {
            skip -= len;
            continue;
        }
        let rem_len = len - skip;
        if rem_len > 0 {
            iovecs.push(libc::iovec {
                iov_base: (addr + skip) as *mut c_void,
                iov_len: rem_len,
            });
        }
        skip = 0;
    }
    let mut msg: libc::msghdr = std::mem::zeroed();
    if let Some((storage, plen)) = to {
        msg.msg_name = storage as *const libc::sockaddr_storage as *mut c_void;
        msg.msg_namelen = *plen;
    }
    msg.msg_iov = iovecs.as_mut_ptr();
    msg.msg_iovlen = iovecs.len() as libc::c_int;
    libc::sendmsg(fd, &msg, pflags)
}

unsafe fn sendmsg_bufs(
    fd: i32,
    bufs: &[(usize, usize)],
    pflags: i32,
    to: Option<&(libc::sockaddr_storage, libc::socklen_t)>,
) -> isize {
    sendmsg_bufs_from(fd, bufs, 0, pflags, to)
}

enum OverlappedSendStep {
    Complete(usize),
    Pending,
    Error(i32),
}

/// Drive a nonblocking overlapped socket send step.
/// - For stream sockets (`SOCK_STREAM`), loops nonblocking `sendmsg` advancing `*sent_so_far`
///   across whole and partial `WSABUF` entries until `total_len` bytes are sent or `EAGAIN`.
/// - For non-stream (datagram/message) sockets, sends the entire message atomically once from
///   offset 0 without splitting.
unsafe fn step_overlapped_send(
    fd: i32,
    bufs: &[(usize, usize)],
    total_len: usize,
    sent_so_far: &mut usize,
    is_stream: bool,
    pflags: i32,
    to: Option<&(libc::sockaddr_storage, libc::socklen_t)>,
) -> OverlappedSendStep {
    // Underlying socket fds are kept permanently `O_NONBLOCK` (while guest blocking mode
    // lives in `SocketMeta.guest_nonblocking`), so `sendmsg_bufs_from` is genuinely
    // nonblocking on Darwin XNU with zero `F_SETFL` toggling races under concurrency.
    if is_stream {
        if *sent_so_far >= total_len {
            return OverlappedSendStep::Complete(*sent_so_far);
        }
        while *sent_so_far < total_len {
            let n = sendmsg_bufs_from(fd, bufs, *sent_so_far, pflags | libc::MSG_DONTWAIT, to);
            if n > 0 {
                *sent_so_far += n as usize;
                if *sent_so_far >= total_len {
                    return OverlappedSendStep::Complete(*sent_so_far);
                }
            } else if n == 0 {
                return OverlappedSendStep::Error(libc::EPIPE);
            } else {
                let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                if raw_e == libc::EINTR {
                    continue;
                }
                if raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK {
                    return OverlappedSendStep::Pending;
                }
                return OverlappedSendStep::Error(raw_e);
            }
        }
        OverlappedSendStep::Complete(*sent_so_far)
    } else {
        loop {
            let n = sendmsg_bufs_from(fd, bufs, 0, pflags | libc::MSG_DONTWAIT, to);
            if n >= 0 {
                *sent_so_far = n as usize;
                return OverlappedSendStep::Complete(n as usize);
            }
            let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
            if raw_e == libc::EINTR {
                continue;
            }
            if raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK {
                return OverlappedSendStep::Pending;
            }
            return OverlappedSendStep::Error(raw_e);
        }
    }
}

unsafe fn recvmsg_bufs(
    fd: i32,
    bufs: &[(usize, usize)],
    pflags: i32,
    mut from: Option<(&mut libc::sockaddr_storage, &mut libc::socklen_t)>,
    msg_flags_out: &mut i32,
) -> isize {
    let mut iovecs: Vec<libc::iovec> = bufs
        .iter()
        .map(|&(addr, len)| libc::iovec {
            iov_base: addr as *mut c_void,
            iov_len: len,
        })
        .collect();
    let mut msg: libc::msghdr = std::mem::zeroed();
    if let Some((storage, plen)) = from.as_mut() {
        msg.msg_name = (*storage) as *mut libc::sockaddr_storage as *mut c_void;
        msg.msg_namelen = **plen;
    }
    msg.msg_iov = iovecs.as_mut_ptr();
    msg.msg_iovlen = iovecs.len() as libc::c_int;
    let n = libc::recvmsg(fd, &mut msg, pflags);
    *msg_flags_out = msg.msg_flags;
    if let Some((_, plen)) = from {
        *plen = msg.msg_namelen;
    }
    n
}

pub unsafe extern "win64" fn WSASend(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    sent: *mut u32,
    flags: u32,
    overlapped: *mut c_void,
    completion: *mut c_void,
) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let fd = meta.fd;
    let bufs = match collect_wsa_bufs(buffers, count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let pflags = win_msg_flags_to_posix(flags);
    if overlapped.is_null() {
        let n = loop {
            let r = sendmsg_bufs(fd, &bufs, pflags, None);
            if r >= 0 {
                break r;
            }
            let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
            if raw_e == libc::EINTR {
                continue;
            }
            if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
                if let Err(we) = wait_socket_ready(fd, libc::POLLOUT, Some(libc::SO_SNDTIMEO)) {
                    return err(errno_to_wsa(we), SOCKET_ERROR);
                }
                continue;
            }
            let wsa = errno_to_wsa(raw_e);
            if wsa == WSAEWOULDBLOCK {
                event_select_rearm_write(s);
            }
            return err(wsa, SOCKET_ERROR);
        };
        if !sent.is_null() {
            std::ptr::write_unaligned(sent, n as u32);
        }
        return 0;
    }

    let ov = overlapped as *mut OVERLAPPED;
    let total_len: usize = bufs.iter().map(|&(_, l)| l).sum();
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    let mut sent_so_far = 0usize;
    match step_overlapped_send(fd, &bufs, total_len, &mut sent_so_far, is_stream, pflags, None) {
        OverlappedSendStep::Complete(total) => {
            if !sent.is_null() {
                std::ptr::write_unaligned(sent, total as u32);
            }
            let tid = crate::kernel32::threads::GetCurrentThreadId();
            let cur_thread = if !completion.is_null() {
                Some(crate::thread::ensure_current_thread_object())
            } else {
                crate::thread::current_thread_object()
            };
            finish_overlapped_op(
                s,
                ov as usize,
                completion as usize,
                cur_thread.as_ref(),
                tid,
                STATUS_SUCCESS,
                total,
                true,
            );
            0
        }
        OverlappedSendStep::Pending => {
            prepare_socket_pending(s, ov, completion as usize);
            async_io_enqueue(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: fd,
                poll_events: libc::POLLOUT,
                overlapped: ov as usize,
                issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
                issuing_thread: if !completion.is_null() {
                    Some(crate::thread::ensure_current_thread_object())
                } else {
                    crate::thread::current_thread_object()
                },
                completion_routine: completion as usize,
                kind: AsyncOpKind::Send {
                    bufs,
                    total_len,
                    sent_so_far,
                    is_stream,
                    pflags,
                    to: None,
                },
            });
            err(WSA_IO_PENDING, SOCKET_ERROR)
        }
        OverlappedSendStep::Error(raw_e) => err(errno_to_wsa(raw_e), SOCKET_ERROR),
    }
}

pub unsafe extern "win64" fn WSASendTo(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    sent: *mut u32,
    flags: u32,
    to: *const u8,
    tolen: i32,
    overlapped: *mut c_void,
    completion: *mut c_void,
) -> i32 {
    if to.is_null() || tolen == 0 {
        return WSASend(s, buffers, count, sent, flags, overlapped, completion);
    }
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let fd = meta.fd;
    let bufs = match collect_wsa_bufs(buffers, count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let dest = match win_to_posix_sockaddr(to, tolen) {
        Ok(v) => v,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let pflags = win_msg_flags_to_posix(flags);
    if overlapped.is_null() {
        let n = loop {
            let r = sendmsg_bufs(fd, &bufs, pflags, Some(&dest));
            if r >= 0 {
                break r;
            }
            let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
            if raw_e == libc::EINTR {
                continue;
            }
            if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
                if let Err(we) = wait_socket_ready(fd, libc::POLLOUT, Some(libc::SO_SNDTIMEO)) {
                    return err(errno_to_wsa(we), SOCKET_ERROR);
                }
                continue;
            }
            let wsa = errno_to_wsa(raw_e);
            if wsa == WSAEWOULDBLOCK {
                event_select_rearm_write(s);
            }
            return err(wsa, SOCKET_ERROR);
        };
        if !sent.is_null() {
            std::ptr::write_unaligned(sent, n as u32);
        }
        return 0;
    }

    let ov = overlapped as *mut OVERLAPPED;
    let total_len: usize = bufs.iter().map(|&(_, l)| l).sum();
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    let mut sent_so_far = 0usize;
    match step_overlapped_send(
        fd,
        &bufs,
        total_len,
        &mut sent_so_far,
        is_stream,
        pflags,
        Some(&dest),
    ) {
        OverlappedSendStep::Complete(total) => {
            if !sent.is_null() {
                std::ptr::write_unaligned(sent, total as u32);
            }
            let tid = crate::kernel32::threads::GetCurrentThreadId();
            let cur_thread = if !completion.is_null() {
                Some(crate::thread::ensure_current_thread_object())
            } else {
                crate::thread::current_thread_object()
            };
            finish_overlapped_op(
                s,
                ov as usize,
                completion as usize,
                cur_thread.as_ref(),
                tid,
                STATUS_SUCCESS,
                total,
                true,
            );
            0
        }
        OverlappedSendStep::Pending => {
            prepare_socket_pending(s, ov, completion as usize);
            async_io_enqueue(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: fd,
                poll_events: libc::POLLOUT,
                overlapped: ov as usize,
                issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
                issuing_thread: if !completion.is_null() {
                    Some(crate::thread::ensure_current_thread_object())
                } else {
                    crate::thread::current_thread_object()
                },
                completion_routine: completion as usize,
                kind: AsyncOpKind::Send {
                    bufs,
                    total_len,
                    sent_so_far,
                    is_stream,
                    pflags,
                    to: Some(dest),
                },
            });
            err(WSA_IO_PENDING, SOCKET_ERROR)
        }
        OverlappedSendStep::Error(raw_e) => err(errno_to_wsa(raw_e), SOCKET_ERROR),
    }
}

pub unsafe extern "win64" fn WSARecv(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    recvd: *mut u32,
    flags: *mut u32,
    overlapped: *mut c_void,
    completion: *mut c_void,
) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let bufs = match collect_wsa_bufs(buffers, count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let fl = if flags.is_null() { 0 } else { std::ptr::read_unaligned(flags) };
    let pflags = win_msg_flags_to_posix(fl);
    if overlapped.is_null() {
        let mut msg_flags = 0i32;
        let n = loop {
            let r = recvmsg_bufs(fd, &bufs, pflags, None, &mut msg_flags);
            if r >= 0 {
                break r;
            }
            let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
            if raw_e == libc::EINTR {
                continue;
            }
            if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
                if let Err(we) = wait_socket_ready(fd, libc::POLLIN, Some(libc::SO_RCVTIMEO)) {
                    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                    return err(errno_to_wsa(we), SOCKET_ERROR);
                }
                continue;
            }
            event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
            return err(errno_to_wsa(raw_e), SOCKET_ERROR);
        };
        event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
        if !flags.is_null() {
            let out_fl = if (msg_flags & libc::MSG_TRUNC) != 0 { 0x8000 } else { 0 };
            std::ptr::write_unaligned(flags, out_fl);
        }
        if !recvd.is_null() {
            std::ptr::write_unaligned(recvd, n as u32);
        }
        return 0;
    }

    let ov = overlapped as *mut OVERLAPPED;
    let total_len: usize = bufs.iter().map(|&(_, l)| l).sum();
    let nb_flags = (pflags & !libc::MSG_WAITALL) | libc::MSG_DONTWAIT;
    let mut msg_flags = 0i32;
    let try_res: Result<usize, i32> = if total_len == 0 {
        let mut pfd = libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        };
        let prc = libc::poll(&mut pfd, 1, 0);
        if prc < 0 {
            Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL))
        } else if prc == 0 || (pfd.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR)) == 0 {
            Err(libc::EAGAIN)
        } else {
            let so_err = query_so_error(fd);
            if so_err != 0 { Err(so_err) } else { Ok(0) }
        }
    } else {
        let n = recvmsg_bufs(fd, &bufs, nb_flags, None, &mut msg_flags);
        if n >= 0 {
            Ok(n as usize)
        } else {
            Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL))
        }
    };
    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);

    match try_res {
        Ok(got) => {
            if !flags.is_null() {
                let out_fl = if (msg_flags & libc::MSG_TRUNC) != 0 { 0x8000 } else { 0 };
                std::ptr::write_unaligned(flags, out_fl);
            }
            if !recvd.is_null() {
                std::ptr::write_unaligned(recvd, got as u32);
            }
            let tid = crate::kernel32::threads::GetCurrentThreadId();
            let cur_thread = if !completion.is_null() {
                Some(crate::thread::ensure_current_thread_object())
            } else {
                crate::thread::current_thread_object()
            };
            finish_overlapped_op(
                s,
                ov as usize,
                completion as usize,
                cur_thread.as_ref(),
                tid,
                STATUS_SUCCESS,
                got,
                true,
            );
            0
        }
        Err(raw_e) if raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK => {
            prepare_socket_pending(s, ov, completion as usize);
            async_io_enqueue(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: fd,
                poll_events: libc::POLLIN,
                overlapped: ov as usize,
                issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
                issuing_thread: if !completion.is_null() {
                    Some(crate::thread::ensure_current_thread_object())
                } else {
                    crate::thread::current_thread_object()
                },
                completion_routine: completion as usize,
                kind: AsyncOpKind::Recv {
                    bufs,
                    pflags: pflags & !libc::MSG_WAITALL,
                    from: 0,
                    fromlen: 0,
                    zero_len: total_len == 0,
                },
            });
            err(WSA_IO_PENDING, SOCKET_ERROR)
        }
        Err(raw_e) => err(errno_to_wsa(raw_e), SOCKET_ERROR),
    }
}

pub unsafe extern "win64" fn WSARecvFrom(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    recvd: *mut u32,
    flags: *mut u32,
    from: *mut u8,
    fromlen: *mut i32,
    overlapped: *mut c_void,
    completion: *mut c_void,
) -> i32 {
    if from.is_null() {
        return WSARecv(s, buffers, count, recvd, flags, overlapped, completion);
    }
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let bufs = match collect_wsa_bufs(buffers, count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let fl = if flags.is_null() { 0 } else { std::ptr::read_unaligned(flags) };
    let pflags = win_msg_flags_to_posix(fl);
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    if overlapped.is_null() {
        let mut msg_flags = 0i32;
        let n = loop {
            slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
            let r = recvmsg_bufs(fd, &bufs, pflags, Some((&mut storage, &mut slen)), &mut msg_flags);
            if r >= 0 {
                break r;
            }
            let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
            if raw_e == libc::EINTR {
                continue;
            }
            if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
                if let Err(we) = wait_socket_ready(fd, libc::POLLIN, Some(libc::SO_RCVTIMEO)) {
                    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                    return err(errno_to_wsa(we), SOCKET_ERROR);
                }
                continue;
            }
            event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
            return err(errno_to_wsa(raw_e), SOCKET_ERROR);
        };
        event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
        if !fromlen.is_null() {
            let _ = posix_to_win_sockaddr(
                &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
                slen,
                from,
                fromlen,
            );
        }
        if !flags.is_null() {
            let out_fl = if (msg_flags & libc::MSG_TRUNC) != 0 { 0x8000 } else { 0 };
            std::ptr::write_unaligned(flags, out_fl);
        }
        if !recvd.is_null() {
            std::ptr::write_unaligned(recvd, n as u32);
        }
        return 0;
    }

    let ov = overlapped as *mut OVERLAPPED;
    let total_len: usize = bufs.iter().map(|&(_, l)| l).sum();
    let nb_flags = (pflags & !libc::MSG_WAITALL) | libc::MSG_DONTWAIT;
    let mut msg_flags = 0i32;
    let n = recvmsg_bufs(fd, &bufs, nb_flags, Some((&mut storage, &mut slen)), &mut msg_flags);
    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
    if n >= 0 {
        if !fromlen.is_null() {
            let _ = posix_to_win_sockaddr(
                &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
                slen,
                from,
                fromlen,
            );
        }
        if !flags.is_null() {
            let out_fl = if (msg_flags & libc::MSG_TRUNC) != 0 { 0x8000 } else { 0 };
            std::ptr::write_unaligned(flags, out_fl);
        }
        if !recvd.is_null() {
            std::ptr::write_unaligned(recvd, n as u32);
        }
        let tid = crate::kernel32::threads::GetCurrentThreadId();
        let cur_thread = if !completion.is_null() {
            Some(crate::thread::ensure_current_thread_object())
        } else {
            crate::thread::current_thread_object()
        };
        finish_overlapped_op(
            s,
            ov as usize,
            completion as usize,
            cur_thread.as_ref(),
            tid,
            STATUS_SUCCESS,
            n as usize,
            true,
        );
        return 0;
    }
    let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
    if raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK {
        prepare_socket_pending(s, ov, completion as usize);
        async_io_enqueue(PendingSocketOp {
            socket_handle: s,
            secondary_socket: None,
            poll_fd: fd,
            poll_events: libc::POLLIN,
            overlapped: ov as usize,
            issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
            issuing_thread: if !completion.is_null() {
                Some(crate::thread::ensure_current_thread_object())
            } else {
                crate::thread::current_thread_object()
            },
            completion_routine: completion as usize,
            kind: AsyncOpKind::Recv {
                bufs,
                pflags: pflags & !libc::MSG_WAITALL,
                from: from as usize,
                fromlen: fromlen as usize,
                zero_len: total_len == 0,
            },
        });
        return err(WSA_IO_PENDING, SOCKET_ERROR);
    }
    err(errno_to_wsa(raw_e), SOCKET_ERROR)
}

pub unsafe extern "win64" fn WSAGetOverlappedResult(
    s: usize,
    ov: *mut OVERLAPPED,
    cb_transfer: *mut u32,
    f_wait: BOOL,
    flags: *mut u32,
) -> BOOL {
    if lookup_fd(s).is_none() {
        return err(WSAENOTSOCK, FALSE);
    }
    if ov.is_null() {
        return err(WSAEFAULT, FALSE);
    }
    let load_internal = || load_ov_internal(ov);
    let mut status = load_internal();
    if status == STATUS_PENDING {
        if f_wait == 0 {
            return err(WSA_IO_INCOMPLETE, FALSE);
        }
        let raw_ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event)) as usize;
        let ev_h = (raw_ev & !1) as HANDLE;
        if !ev_h.is_null() {
            let Some(obj) = crate::handles::get(ev_h) else {
                return err(WSAEINVAL, FALSE);
            };
            while load_internal() == STATUS_PENDING {
                match crate::handles::wait_for(&[Arc::clone(&obj)], false, INFINITE, false) {
                    crate::handles::WaitResult::Failed(e) => return err(e, FALSE),
                    _ => {}
                }
            }
        } else {
            let mut st = ASYNC_IO.inner.lock().unwrap();
            while load_internal() == STATUS_PENDING {
                let in_reactor = st.ops.iter().any(|op| op.overlapped == ov as usize);
                if in_reactor {
                    st = ASYNC_IO.gen_cv.wait(st).unwrap();
                } else {
                    drop(st);
                    if let Some(ext_ev) = external_io_event(s as HANDLE) {
                        let obj: Arc<dyn crate::handles::KObject> = ext_ev;
                        let _ = crate::handles::wait_for(&[obj], false, 50, false);
                    } else {
                        break;
                    }
                    st = ASYNC_IO.inner.lock().unwrap();
                }
            }
        }
        status = load_internal();
    }
    let bytes = load_ov_internal_high(ov) as u32;
    if !cb_transfer.is_null() {
        std::ptr::write_unaligned(cb_transfer, bytes);
    }
    if !flags.is_null() {
        std::ptr::write_unaligned(flags, 0);
    }
    if status == STATUS_SUCCESS {
        TRUE
    } else {
        err(ntstatus_to_wsa(status), FALSE)
    }
}

#[repr(C)]
pub struct WinTimeVal {
    pub tv_sec: i32,
    pub tv_usec: i32,
}

unsafe fn read_fd_set(set: *const u8) -> Vec<(usize, i32)> {
    if set.is_null() {
        return Vec::new();
    }
    let n = std::ptr::read_unaligned(set as *const u32).min(64) as usize;
    let arr = set.add(8) as *const usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let s = std::ptr::read_unaligned(arr.add(i));
        if let Some(fd) = lookup_fd(s) {
            out.push((s, fd));
        }
    }
    out
}

unsafe fn write_fd_set(set: *mut u8, socks: &[usize]) {
    if set.is_null() {
        return;
    }
    let n = socks.len().min(64);
    std::ptr::write_unaligned(set as *mut u32, n as u32);
    let arr = set.add(8) as *mut usize;
    for (i, &s) in socks[..n].iter().enumerate() {
        std::ptr::write_unaligned(arr.add(i), s);
    }
}

pub unsafe extern "win64" fn select(
    _nfds: i32,
    readfds: *mut u8,
    writefds: *mut u8,
    exceptfds: *mut u8,
    timeout: *const WinTimeVal,
) -> i32 {
    let rlist = read_fd_set(readfds);
    let wlist = read_fd_set(writefds);
    let elist = read_fd_set(exceptfds);
    let mut pfds: Vec<libc::pollfd> = Vec::new();
    let mut add = |fd: i32, ev: i16| {
        if let Some(p) = pfds.iter_mut().find(|p| p.fd == fd) {
            p.events |= ev;
        } else {
            pfds.push(libc::pollfd { fd, events: ev, revents: 0 });
        }
    };
    for &(_, fd) in &rlist {
        add(fd, libc::POLLIN);
    }
    for &(_, fd) in &wlist {
        add(fd, libc::POLLOUT);
    }
    for &(_, fd) in &elist {
        add(fd, libc::POLLPRI);
    }
    let ms = if timeout.is_null() {
        -1
    } else {
        let tv = std::ptr::read_unaligned(timeout);
        (tv.tv_sec as i64 * 1000 + tv.tv_usec as i64 / 1000).clamp(0, i32::MAX as i64) as i32
    };
    if pfds.is_empty() {
        if ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(ms as u64));
        }
        return 0;
    }
    let rc = libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, ms);
    if rc < 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    let mut r_out = Vec::new();
    let mut w_out = Vec::new();
    let mut e_out = Vec::new();
    for &(s, fd) in &rlist {
        if pfds.iter().any(|p| p.fd == fd && (p.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR)) != 0) {
            r_out.push(s);
        }
    }
    for &(s, fd) in &wlist {
        if pfds.iter().any(|p| p.fd == fd && (p.revents & (libc::POLLOUT | libc::POLLERR)) != 0) {
            w_out.push(s);
        }
    }
    for &(s, fd) in &elist {
        if pfds.iter().any(|p| p.fd == fd && (p.revents & (libc::POLLPRI | libc::POLLERR)) != 0) {
            e_out.push(s);
        }
    }
    write_fd_set(readfds, &r_out);
    write_fd_set(writefds, &w_out);
    write_fd_set(exceptfds, &e_out);
    (r_out.len() + w_out.len() + e_out.len()) as i32
}

#[repr(C)]
pub struct WinPollFd {
    pub fd: usize,
    pub events: i16,
    pub revents: i16,
}

pub unsafe extern "win64" fn WSAPoll(fd_array: *mut WinPollFd, fds: u32, timeout: i32) -> i32 {
    if fd_array.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let n = fds as usize;
    let mut pfds = Vec::with_capacity(n);
    for i in 0..n {
        let w = std::ptr::read_unaligned(fd_array.add(i));
        let pfd = lookup_fd(w.fd).unwrap_or(-1);
        let mut pev = 0i16;
        if (w.events & 0x0300) != 0 {
            pev |= libc::POLLIN;
        }
        if (w.events & 0x0010) != 0 {
            pev |= libc::POLLOUT;
        }
        pfds.push(libc::pollfd { fd: pfd, events: pev, revents: 0 });
    }
    let rc = libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, timeout);
    if rc < 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    for (i, p) in pfds.iter().enumerate() {
        let mut rev = 0i16;
        if (p.revents & libc::POLLIN) != 0 {
            rev |= 0x0100; // POLLRDNORM
        }
        if (p.revents & libc::POLLOUT) != 0 {
            rev |= 0x0010; // POLLWRNORM
        }
        if (p.revents & libc::POLLERR) != 0 {
            rev |= 0x0001; // POLLERR
        }
        if (p.revents & libc::POLLHUP) != 0 {
            rev |= 0x0002; // POLLHUP
        }
        if (p.revents & libc::POLLNVAL) != 0 {
            rev |= 0x0004; // POLLNVAL
        }
        let mut w = std::ptr::read_unaligned(fd_array.add(i));
        w.revents = rev;
        std::ptr::write_unaligned(fd_array.add(i), w);
    }
    rc
}

pub unsafe extern "win64" fn WSAAddressToStringA(sa: *const u8, len: u32, _proto: *mut c_void, out: *mut u8, out_len: *mut u32) -> i32 {
    if out_len.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let Some(s) = sockaddr_to_string(sa, len) else {
        return err(WSAEINVAL, SOCKET_ERROR);
    };
    let need = s.len() + 1;
    if out.is_null() || (*out_len as usize) < need {
        *out_len = need as u32;
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), out, s.len());
    *out.add(s.len()) = 0;
    *out_len = need as u32;
    0
}

pub unsafe extern "win64" fn WSAStringToAddressA(s: *const u8, family: i32, proto: *mut c_void, sa: *mut u8, len: *mut i32) -> i32 {
    if s.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let w = crate::strings::to_wide_z(&crate::strings::from_ansi(s));
    WSAStringToAddressW(w.as_ptr(), family, proto, sa, len)
}

pub unsafe extern "win64" fn WSAConnect(
    s: usize,
    name: *const u8,
    namelen: i32,
    _caller_data: *mut c_void,
    _callee_data: *mut c_void,
    _sqos: *mut c_void,
    _gqos: *mut c_void,
) -> i32 {
    connect(s, name, namelen)
}

pub unsafe extern "win64" fn WSAAccept(
    s: usize,
    addr: *mut u8,
    addrlen: *mut i32,
    _cond: *mut c_void,
    _callback_data: usize,
) -> usize {
    accept(s, addr, addrlen)
}

// @exports WSAAsyncSelect WSACancelAsyncRequest WSAConnectByNameA WSAConnectByNameW WSAEnumProtocolsA WSAEnumProtocolsW WSAGetQOSByName WSAHtonl WSAHtons WSAJoinLeaf WSANtohl WSANtohs WSARecvDisconnect WSASendDisconnect WSARecvMsg WSASendMsg
failing!(i32 = SOCKET_ERROR, err WSAEOPNOTSUPP;
    WSAAsyncSelect, WSACancelAsyncRequest, WSAConnectByNameA,
    WSAConnectByNameW,
    WSAEnumProtocolsA, WSAEnumProtocolsW,
    WSAGetQOSByName, WSAHtonl, WSAHtons, WSAJoinLeaf,
    WSANtohl, WSANtohs, WSARecvDisconnect, WSASendDisconnect, WSARecvMsg,
    WSASendMsg);

/// fd_set: u32 fd_count @0, SOCKET fd_array[64] @8.
pub unsafe extern "win64" fn __WSAFDIsSet(s: usize, set: *const u8) -> i32 {
    if set.is_null() {
        return 0;
    }
    let n = (*(set as *const u32)).min(64) as usize;
    let arr = set.add(8) as *const usize;
    (0..n).any(|i| *arr.add(i) == s) as i32
}

// ---------------------------------------------------------------- events

pub unsafe extern "win64" fn WSACreateEvent() -> HANDLE {
    crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null())
}
pub unsafe extern "win64" fn WSASetEvent(h: HANDLE) -> BOOL {
    crate::kernel32::threads::SetEvent(h)
}
pub unsafe extern "win64" fn WSAResetEvent(h: HANDLE) -> BOOL {
    crate::kernel32::threads::ResetEvent(h)
}
pub unsafe extern "win64" fn WSACloseEvent(h: HANDLE) -> BOOL {
    crate::kernel32::threads::CloseHandle(h)
}
/// Same return-code space as WaitForMultipleObjectsEx (WSA_WAIT_EVENT_0 = 0,
/// WSA_WAIT_IO_COMPLETION = 0xC0, WSA_WAIT_TIMEOUT = 0x102).
pub unsafe extern "win64" fn WSAWaitForMultipleEvents(n: u32, hs: *const HANDLE, all: BOOL, ms: u32, alertable: BOOL) -> u32 {
    crate::kernel32::threads::WaitForMultipleObjectsEx(n, hs, all, ms, alertable)
}

const FD_READ_BIT: usize = 0;
const FD_WRITE_BIT: usize = 1;
const FD_OOB_BIT: usize = 2;
const FD_ACCEPT_BIT: usize = 3;
const FD_CONNECT_BIT: usize = 4;
const FD_CLOSE_BIT: usize = 5;
const FD_MAX_EVENTS: usize = 10;

const FD_READ: i32 = 1 << FD_READ_BIT;
const FD_WRITE: i32 = 1 << FD_WRITE_BIT;
const FD_OOB: i32 = 1 << FD_OOB_BIT;
const FD_ACCEPT: i32 = 1 << FD_ACCEPT_BIT;
const FD_CONNECT: i32 = 1 << FD_CONNECT_BIT;
const FD_CLOSE: i32 = 1 << FD_CLOSE_BIT;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WsaNetworkEvents {
    pub l_network_events: i32,
    pub i_error_code: [i32; FD_MAX_EVENTS],
}

struct EventSelectEntry {
    fd: i32,
    is_stream: bool,
    event: usize,
    mask: i32,
    listening: bool,
    connecting: bool,
    connected: bool,
    read_fired: bool,
    write_armed: bool,
    oob_fired: bool,
    accept_fired: bool,
    close_fired: bool,
    pending_events: i32,
    pending_errors: [i32; FD_MAX_EVENTS],
}

impl EventSelectEntry {
    fn record(&mut self, bit: usize, err_code: i32) -> bool {
        let flag = 1i32 << bit;
        if (self.mask & flag) == 0 {
            return false;
        }
        let was_set = (self.pending_events & flag) != 0;
        self.pending_events |= flag;
        self.pending_errors[bit] = err_code;
        !was_set || err_code != 0
    }

    fn poll_events(&self) -> i16 {
        if self.mask == 0 {
            return 0;
        }
        if self.connecting {
            return if (self.mask & (FD_CONNECT | FD_WRITE | FD_READ | FD_CLOSE)) != 0 {
                libc::POLLOUT
            } else {
                0
            };
        }
        if self.listening {
            return if (self.mask & FD_ACCEPT) != 0 && !self.accept_fired {
                libc::POLLIN
            } else {
                0
            };
        }
        if self.is_stream && !self.connected {
            return 0;
        }
        let mut ev = 0i16;
        if ((self.mask & FD_READ) != 0 && !self.read_fired)
            || (self.is_stream && (self.mask & FD_CLOSE) != 0 && !self.close_fired && !self.read_fired)
        {
            ev |= libc::POLLIN;
        }
        if (self.mask & FD_WRITE) != 0 && self.write_armed {
            ev |= libc::POLLOUT;
        }
        if (self.mask & FD_OOB) != 0 && !self.oob_fired {
            ev |= libc::POLLPRI;
        }
        ev
    }
}

struct EventSelectInner {
    entries: HashMap<usize, EventSelectEntry>,
    poll_gen: u64,
    started: bool,
    wake_r: i32,
    wake_w: i32,
}

struct EventSelectEngine {
    inner: Mutex<EventSelectInner>,
    gen_cv: Condvar,
}

static EVENT_SELECT: LazyLock<EventSelectEngine> = LazyLock::new(|| EventSelectEngine {
    inner: Mutex::new(EventSelectInner {
        entries: HashMap::new(),
        poll_gen: 0,
        started: false,
        wake_r: -1,
        wake_w: -1,
    }),
    gen_cv: Condvar::new(),
});

fn wake_fd_write(wake_w: i32) {
    if wake_w >= 0 {
        let b = [1u8];
        unsafe {
            let _ = libc::write(wake_w, b.as_ptr() as *const c_void, 1);
        }
    }
}

fn ensure_event_poller_started(st: &mut EventSelectInner) {
    if st.started {
        return;
    }
    let mut fds = [-1i32; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
        log::warn!("ws2_32: pipe() failed for WSAEventSelect poller");
        return;
    }
    for &fd in &fds {
        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFL);
            if flags >= 0 {
                libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
            }
            let fdflags = libc::fcntl(fd, libc::F_GETFD);
            if fdflags >= 0 {
                libc::fcntl(fd, libc::F_SETFD, fdflags | libc::FD_CLOEXEC);
            }
        }
    }
    st.wake_r = fds[0];
    st.wake_w = fds[1];
    st.started = true;
    let _ = std::thread::Builder::new()
        .name("ws2-event-select".into())
        .spawn(event_select_poller_loop);
}

unsafe fn query_so_error(fd: i32) -> i32 {
    let mut e: i32 = 0;
    let mut l = std::mem::size_of::<i32>() as libc::socklen_t;
    if libc::getsockopt(fd, libc::SOL_SOCKET, libc::SO_ERROR, &mut e as *mut i32 as *mut c_void, &mut l) != 0 {
        return std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
    }
    e
}

/// Apply `poll()` revents to an entry. Returns `(should_signal_event, connect_completed_ok)`.
unsafe fn process_poll_revents(entry: &mut EventSelectEntry, revents: i16) -> (bool, Option<bool>) {
    if (revents & libc::POLLNVAL) != 0 {
        entry.connecting = false;
        entry.close_fired = true;
        entry.read_fired = true;
        entry.write_armed = false;
        return (false, None);
    }
    let mut signal = false;
    let mut connect_done = None;

    if entry.connecting {
        if (revents & (libc::POLLOUT | libc::POLLERR | libc::POLLHUP)) != 0 {
            let so_err = query_so_error(entry.fd);
            entry.connecting = false;
            entry.connected = so_err == 0;
            connect_done = Some(so_err == 0);
            let wsa = if so_err == 0 { 0 } else { errno_to_wsa(so_err) as i32 };
            if entry.record(FD_CONNECT_BIT, wsa) {
                signal = true;
            }
            if so_err == 0 && (entry.mask & FD_WRITE) != 0 {
                entry.write_armed = false;
                if entry.record(FD_WRITE_BIT, 0) {
                    signal = true;
                }
            }
        }
        return (signal, connect_done);
    }

    if entry.listening {
        if (entry.mask & FD_ACCEPT) != 0 && !entry.accept_fired && (revents & (libc::POLLIN | libc::POLLERR | libc::POLLHUP)) != 0 {
            entry.accept_fired = true;
            if entry.record(FD_ACCEPT_BIT, 0) {
                signal = true;
            }
        }
        return (signal, None);
    }

    if (entry.mask & FD_WRITE) != 0 && entry.write_armed && (revents & libc::POLLOUT) != 0 {
        entry.write_armed = false;
        if entry.record(FD_WRITE_BIT, 0) {
            signal = true;
        }
    }

    if (entry.mask & FD_OOB) != 0 && !entry.oob_fired && (revents & libc::POLLPRI) != 0 {
        entry.oob_fired = true;
        if entry.record(FD_OOB_BIT, 0) {
            signal = true;
        }
    }

    if (revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR)) != 0 {
        if !entry.is_stream {
            if (entry.mask & FD_READ) != 0 && !entry.read_fired && (revents & libc::POLLIN) != 0 {
                entry.read_fired = true;
                if entry.record(FD_READ_BIT, 0) {
                    signal = true;
                }
            }
        } else {
            let mut avail: i32 = 0;
            let _ = libc::ioctl(entry.fd, libc::FIONREAD, &mut avail);
            if avail > 0 {
                entry.read_fired = true;
                if entry.record(FD_READ_BIT, 0) {
                    signal = true;
                }
            } else {
                let mut peek_byte = 0u8;
                let rc = libc::recv(
                    entry.fd,
                    &mut peek_byte as *mut u8 as *mut c_void,
                    1,
                    libc::MSG_PEEK | libc::MSG_DONTWAIT,
                );
                if rc > 0 {
                    entry.read_fired = true;
                    if entry.record(FD_READ_BIT, 0) {
                        signal = true;
                    }
                } else if rc == 0 {
                    entry.close_fired = true;
                    if entry.record(FD_CLOSE_BIT, 0) {
                        signal = true;
                    }
                } else {
                    let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
                    if e != libc::EAGAIN && e != libc::EINTR {
                        entry.close_fired = true;
                        let wsa = if e == 0 { 0 } else { errno_to_wsa(e) as i32 };
                        if entry.record(FD_CLOSE_BIT, wsa) {
                            signal = true;
                        }
                    }
                }
            }
        }
    }

    (signal, connect_done)
}

fn event_select_poller_loop() {
    let mut pfds: Vec<libc::pollfd> = Vec::new();
    let mut watched_socks: Vec<usize> = Vec::new();
    let mut to_signal: Vec<usize> = Vec::new();
    let mut connect_updates: Vec<(usize, bool)> = Vec::new();

    loop {
        pfds.clear();
        watched_socks.clear();
        let wake_r = {
            let mut st = EVENT_SELECT.inner.lock().unwrap();
            st.poll_gen = st.poll_gen.wrapping_add(1);
            EVENT_SELECT.gen_cv.notify_all();
            let wr = st.wake_r;
            pfds.push(libc::pollfd { fd: wr, events: libc::POLLIN, revents: 0 });
            for (&sock, entry) in &st.entries {
                let ev = entry.poll_events();
                if ev != 0 {
                    pfds.push(libc::pollfd { fd: entry.fd, events: ev, revents: 0 });
                    watched_socks.push(sock);
                }
            }
            wr
        };

        let rc = unsafe { libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, -1) };
        if rc < 0 {
            let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            if e == libc::EINTR {
                continue;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
            continue;
        }

        if (pfds[0].revents & libc::POLLIN) != 0 {
            let mut drain = [0u8; 64];
            loop {
                let n = unsafe { libc::read(wake_r, drain.as_mut_ptr() as *mut c_void, drain.len()) };
                if n <= 0 {
                    break;
                }
            }
        }

        to_signal.clear();
        connect_updates.clear();
        {
            let mut st = EVENT_SELECT.inner.lock().unwrap();
            for (i, &sock) in watched_socks.iter().enumerate() {
                let pfd = pfds[i + 1];
                if pfd.revents == 0 {
                    continue;
                }
                if let Some(entry) = st.entries.get_mut(&sock) {
                    if entry.fd != pfd.fd {
                        continue;
                    }
                    let (sig, conn) = unsafe { process_poll_revents(entry, pfd.revents) };
                    if sig && entry.event != 0 {
                        to_signal.push(entry.event);
                    }
                    if let Some(ok) = conn {
                        connect_updates.push((sock, ok));
                    }
                }
            }
        }

        if !connect_updates.is_empty() {
            let mut socks = SOCKETS.lock().unwrap();
            for &(sock, ok) in &connect_updates {
                if let Some(m) = socks.get_mut(&sock) {
                    m.connecting = false;
                    m.connected = ok;
                }
            }
        }
        for &ev in &to_signal {
            unsafe {
                crate::kernel32::threads::SetEvent(ev as HANDLE);
            }
        }
    }
}

fn event_select_remove_and_sync(s: usize) {
    let mut st = EVENT_SELECT.inner.lock().unwrap();
    if st.entries.remove(&s).is_none() {
        return;
    }
    if !st.started {
        return;
    }
    let gen = st.poll_gen;
    let ww = st.wake_w;
    wake_fd_write(ww);
    // Wait one poller cycle so `poll()` is guaranteed not to reference the fd when `close()` runs.
    let mut waited = st;
    while waited.poll_gen == gen {
        let (next, res) = EVENT_SELECT
            .gen_cv
            .wait_timeout(waited, std::time::Duration::from_millis(50))
            .unwrap();
        waited = next;
        if res.timed_out() {
            break;
        }
    }
}

fn event_select_on_listen(s: usize) {
    let mut st = EVENT_SELECT.inner.lock().unwrap();
    if let Some(entry) = st.entries.get_mut(&s) {
        entry.listening = true;
        entry.accept_fired = false;
        let ww = st.wake_w;
        wake_fd_write(ww);
    }
}

fn event_select_rearm_accept(s: usize) {
    let mut st = EVENT_SELECT.inner.lock().unwrap();
    if let Some(entry) = st.entries.get_mut(&s) {
        entry.accept_fired = false;
        let ww = st.wake_w;
        wake_fd_write(ww);
    }
}

fn event_select_inherit_accept(parent: usize, child: usize, child_fd: i32, sock_type: i32) {
    let mut st = EVENT_SELECT.inner.lock().unwrap();
    let Some((event, mask)) = st.entries.get(&parent).map(|e| (e.event, e.mask)) else {
        return;
    };
    if mask == 0 {
        return;
    }
    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&child) {
        m.guest_nonblocking = true;
    }
    st.entries.insert(
        child,
        EventSelectEntry {
            fd: child_fd,
            is_stream: sock_type == libc::SOCK_STREAM,
            event,
            mask,
            listening: false,
            connecting: false,
            connected: true,
            read_fired: false,
            write_armed: (mask & FD_WRITE) != 0,
            oob_fired: false,
            accept_fired: false,
            close_fired: false,
            pending_events: 0,
            pending_errors: [0; FD_MAX_EVENTS],
        },
    );
    let ww = st.wake_w;
    wake_fd_write(ww);
}

fn event_select_on_connect_pending(s: usize) {
    let mut st = EVENT_SELECT.inner.lock().unwrap();
    if let Some(entry) = st.entries.get_mut(&s) {
        entry.connecting = true;
        entry.connected = false;
        let ww = st.wake_w;
        wake_fd_write(ww);
    }
}

fn event_select_on_connect_complete(s: usize, wsa_err: i32) {
    let ev_to_signal = {
        let mut st = EVENT_SELECT.inner.lock().unwrap();
        if let Some(entry) = st.entries.get_mut(&s) {
            entry.connecting = false;
            entry.connected = wsa_err == 0;
            let mut sig = entry.record(FD_CONNECT_BIT, wsa_err);
            if wsa_err == 0 && (entry.mask & FD_WRITE) != 0 {
                entry.write_armed = false;
                if entry.record(FD_WRITE_BIT, 0) {
                    sig = true;
                }
            }
            let ev = if sig && entry.event != 0 { Some(entry.event) } else { None };
            let ww = st.wake_w;
            wake_fd_write(ww);
            ev
        } else {
            None
        }
    };
    if let Some(ev) = ev_to_signal {
        unsafe {
            crate::kernel32::threads::SetEvent(ev as HANDLE);
        }
    }
}

fn event_select_rearm_write(s: usize) {
    let mut st = EVENT_SELECT.inner.lock().unwrap();
    if let Some(entry) = st.entries.get_mut(&s) {
        if (entry.mask & FD_WRITE) != 0 && !entry.write_armed {
            entry.write_armed = true;
            let ww = st.wake_w;
            wake_fd_write(ww);
        }
    }
}

fn event_select_rearm_read(s: usize, oob: bool) {
    let mut st = EVENT_SELECT.inner.lock().unwrap();
    if let Some(entry) = st.entries.get_mut(&s) {
        let mut wake = false;
        if oob && entry.oob_fired {
            entry.oob_fired = false;
            wake = true;
        }
        if !oob && entry.read_fired {
            entry.read_fired = false;
            wake = true;
        }
        if wake {
            let ww = st.wake_w;
            wake_fd_write(ww);
        }
    }
}

pub unsafe extern "win64" fn WSAEventSelect(s: usize, h_event_object: HANDLE, l_network_events: i32) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if l_network_events != 0 && h_event_object.is_null() {
        return err(WSAEINVAL, SOCKET_ERROR);
    }
    if l_network_events == 0 {
        let mut st = EVENT_SELECT.inner.lock().unwrap();
        if st.entries.remove(&s).is_some() {
            let ww = st.wake_w;
            wake_fd_write(ww);
        }
        return 0;
    }
    // Windows WSAEventSelect automatically puts the socket into non-blocking mode.
    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.guest_nonblocking = true;
    }
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    let write_armed = (l_network_events & FD_WRITE) != 0 && (!is_stream || meta.connected);
    {
        let mut st = EVENT_SELECT.inner.lock().unwrap();
        ensure_event_poller_started(&mut st);
        st.entries.insert(
            s,
            EventSelectEntry {
                fd: meta.fd,
                is_stream,
                event: h_event_object as usize,
                mask: l_network_events,
                listening: meta.listening,
                connecting: meta.connecting,
                connected: meta.connected,
                read_fired: false,
                write_armed,
                oob_fired: false,
                accept_fired: false,
                close_fired: false,
                pending_events: 0,
                pending_errors: [0; FD_MAX_EVENTS],
            },
        );
        let ww = st.wake_w;
        wake_fd_write(ww);
    }
    0
}

pub unsafe extern "win64" fn WSAEnumNetworkEvents(
    s: usize,
    h_event_object: HANDLE,
    lp_network_events: *mut WsaNetworkEvents,
) -> i32 {
    if lookup_fd(s).is_none() {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    }
    if lp_network_events.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    if !h_event_object.is_null() {
        crate::kernel32::threads::ResetEvent(h_event_object);
    }
    let mut connect_done = None;
    let out = {
        let mut st = EVENT_SELECT.inner.lock().unwrap();
        if let Some(entry) = st.entries.get_mut(&s) {
            let ev = entry.poll_events();
            if ev != 0 {
                let mut pfd = libc::pollfd { fd: entry.fd, events: ev, revents: 0 };
                let rc = libc::poll(&mut pfd, 1, 0);
                if rc > 0 && pfd.revents != 0 {
                    let (_, conn) = process_poll_revents(entry, pfd.revents);
                    connect_done = conn;
                }
            }
            let result = WsaNetworkEvents {
                l_network_events: entry.pending_events,
                i_error_code: entry.pending_errors,
            };
            entry.pending_events = 0;
            entry.pending_errors = [0; FD_MAX_EVENTS];
            let ww = st.wake_w;
            wake_fd_write(ww);
            result
        } else {
            WsaNetworkEvents::default()
        }
    };
    if let Some(ok) = connect_done {
        if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
            m.connecting = false;
            m.connected = ok;
        }
    }
    std::ptr::write_unaligned(lp_network_events, out);
    0
}

// ------------------------------------------------ overlapped async reactor & extensions

/// WSAID_ACCEPTEX: {b5367df1-cbac-11cf-95ca-00805f48a192}
const WSAID_ACCEPTEX: [u8; 16] = [
    0xf1, 0x7d, 0x36, 0xb5, 0xac, 0xcb, 0xcf, 0x11, 0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92,
];
/// WSAID_CONNECTEX: {25a207b9-ddf3-4660-8ee9-76e58c74063e}
const WSAID_CONNECTEX: [u8; 16] = [
    0xb9, 0x07, 0xa2, 0x25, 0xf3, 0xdd, 0x60, 0x46, 0x8e, 0xe9, 0x76, 0xe5, 0x8c, 0x74, 0x06, 0x3e,
];
/// WSAID_DISCONNECTEX: {7fda2e11-8630-436f-a031-f536a6eec157}
const WSAID_DISCONNECTEX: [u8; 16] = [
    0x11, 0x2e, 0xda, 0x7f, 0x30, 0x86, 0x6f, 0x43, 0xa0, 0x31, 0xf5, 0x36, 0xa6, 0xee, 0xc1, 0x57,
];
/// WSAID_TRANSMITFILE: {b5367df0-cbac-11cf-95ca-00805f48a192}
const WSAID_TRANSMITFILE: [u8; 16] = [
    0xf0, 0x7d, 0x36, 0xb5, 0xac, 0xcb, 0xcf, 0x11, 0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92,
];
/// WSAID_GETACCEPTEXSOCKADDRS: {b5367df2-cbac-11cf-95ca-00805f48a192}
const WSAID_GETACCEPTEXSOCKADDRS: [u8; 16] = [
    0xf2, 0x7d, 0x36, 0xb5, 0xac, 0xcb, 0xcf, 0x11, 0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92,
];

fn lookup_extension_fn(guid: &[u8; 16]) -> Option<usize> {
    if guid == &WSAID_ACCEPTEX {
        Some(ext_accept_ex as *const () as usize)
    } else if guid == &WSAID_CONNECTEX {
        Some(ext_connect_ex as *const () as usize)
    } else if guid == &WSAID_DISCONNECTEX {
        Some(ext_disconnect_ex as *const () as usize)
    } else if guid == &WSAID_TRANSMITFILE {
        Some(ext_transmit_file as *const () as usize)
    } else if guid == &WSAID_GETACCEPTEXSOCKADDRS {
        Some(ext_get_accept_ex_sockaddrs as *const () as usize)
    } else {
        None
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TransmitFileBuffers {
    head: *const u8,
    head_length: u32,
    _pad0: u32,
    tail: *const u8,
    tail_length: u32,
    _pad1: u32,
}
const _: () = assert!(std::mem::size_of::<TransmitFileBuffers>() == 32);

struct TransmitState {
    head: Vec<u8>,
    file: Option<Arc<crate::kernel32::file::FileObj>>,
    file_offset: u64,
    file_remaining: u64,
    update_file_pos: bool,
    tail: Vec<u8>,
    chunk_size: usize,
    staging: Vec<u8>,
    staging_pos: usize,
    total_sent: usize,
    flags: u32,
}

impl TransmitState {
    unsafe fn step(&mut self, fd: i32, nonblocking: bool) -> Result<bool, i32> {
        let send_flags = if nonblocking { libc::MSG_DONTWAIT } else { 0 };
        loop {
            if self.staging_pos < self.staging.len() {
                let rem = &self.staging[self.staging_pos..];
                let n = libc::send(fd, rem.as_ptr() as *const c_void, rem.len(), send_flags);
                if n > 0 {
                    self.staging_pos += n as usize;
                    self.total_sent += n as usize;
                    continue;
                }
                if n == 0 {
                    return Err(libc::EPIPE);
                }
                let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                if e == libc::EINTR {
                    continue;
                }
                if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
                    if nonblocking {
                        return Ok(false);
                    }
                    wait_socket_ready(fd, libc::POLLOUT, Some(libc::SO_SNDTIMEO))?;
                    continue;
                }
                return Err(e);
            }

            self.staging.clear();
            self.staging_pos = 0;

            if !self.head.is_empty() {
                self.staging = std::mem::take(&mut self.head);
                continue;
            }
            if self.file_remaining > 0 {
                if let Some(ref f) = self.file {
                    let want = (self.file_remaining as usize).min(self.chunk_size);
                    self.staging.resize(want, 0);
                    let got = loop {
                        match f.file().read_at(&mut self.staging, self.file_offset) {
                            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                            other => break other,
                        }
                    };
                    match got {
                        Ok(0) => {
                            self.staging.clear();
                            self.file_remaining = 0;
                            continue;
                        }
                        Ok(k) => {
                            self.staging.truncate(k);
                            self.file_offset += k as u64;
                            self.file_remaining -= k as u64;
                            if self.update_file_pos {
                                let _ = libc::lseek(f.fd(), self.file_offset as libc::off_t, libc::SEEK_SET);
                            }
                            continue;
                        }
                        Err(e) => {
                            self.staging.clear();
                            return Err(e.raw_os_error().unwrap_or(libc::EIO));
                        }
                    }
                } else {
                    self.file_remaining = 0;
                    continue;
                }
            }
            if !self.tail.is_empty() {
                self.staging = std::mem::take(&mut self.tail);
                continue;
            }
            return Ok(true);
        }
    }
}

enum AsyncOpKind {
    AcceptEx {
        s_accept: usize,
        out_buf: usize,
        recv_data_len: u32,
        local_addr_len: u32,
        remote_addr_len: u32,
    },
    AcceptExRecv {
        out_buf: usize,
        recv_data_len: u32,
    },
    ConnectEx {
        send_buf: usize,
        send_len: u32,
        sent_so_far: u32,
        connected: bool,
    },
    TransmitFile(TransmitState),
    Recv {
        bufs: Vec<(usize, usize)>,
        pflags: i32,
        from: usize,
        fromlen: usize,
        zero_len: bool,
    },
    Send {
        bufs: Vec<(usize, usize)>,
        total_len: usize,
        sent_so_far: usize,
        is_stream: bool,
        pflags: i32,
        to: Option<(libc::sockaddr_storage, libc::socklen_t)>,
    },
}

struct PendingSocketOp {
    socket_handle: usize,
    secondary_socket: Option<usize>,
    poll_fd: i32,
    poll_events: i16,
    overlapped: usize,
    issuing_tid: u32,
    issuing_thread: Option<Arc<crate::thread::ThreadObj>>,
    completion_routine: usize,
    kind: AsyncOpKind,
}

struct CompletedOp {
    socket_handle: usize,
    overlapped: usize,
    completion_routine: usize,
    issuing_thread: Option<Arc<crate::thread::ThreadObj>>,
    issuing_tid: u32,
    nt_status: usize,
    bytes: usize,
}

enum OpStep {
    Pending,
    Complete { nt_status: usize, bytes: usize },
}

struct AsyncIoInner {
    ops: Vec<PendingSocketOp>,
    poll_gen: u64,
    started: bool,
    wake_r: i32,
    wake_w: i32,
}

struct AsyncIoEngine {
    inner: Mutex<AsyncIoInner>,
    gen_cv: Condvar,
}

static ASYNC_IO: LazyLock<AsyncIoEngine> = LazyLock::new(|| AsyncIoEngine {
    inner: Mutex::new(AsyncIoInner {
        ops: Vec::new(),
        poll_gen: 0,
        started: false,
        wake_r: -1,
        wake_w: -1,
    }),
    gen_cv: Condvar::new(),
});

unsafe fn prepare_socket_pending(s: usize, ov: *mut OVERLAPPED, completion_routine: usize) {
    if ov.is_null() {
        return;
    }
    if completion_routine != 0 {
        // When a completion routine is provided, OVERLAPPED.hEvent is ignored by Windows.
        // Use the shared alignment-safe OVERLAPPED helpers without touching hEvent.
        store_ov_internal_high(ov, 0);
        store_ov_internal(ov, STATUS_PENDING);
    } else {
        prepare_overlapped_pending(ov);
        let raw_ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event));
        if raw_ev.is_null() {
            if let Some(ev) = external_io_event(s as HANDLE) {
                ev.reset();
            }
        }
    }
}

unsafe fn finish_overlapped_op(
    socket_handle: usize,
    ov: usize,
    completion_routine: usize,
    issuing_thread: Option<&Arc<crate::thread::ThreadObj>>,
    issuing_tid: u32,
    nt_status: usize,
    bytes: usize,
    synchronous: bool,
) {
    if ov == 0 {
        return;
    }
    let ov_ptr = ov as *mut OVERLAPPED;
    if completion_routine != 0 {
        store_ov_internal_high(ov_ptr, bytes);
        store_ov_internal(ov_ptr, nt_status);
        let wsa_err = ntstatus_to_wsa(nt_status);
        let target = issuing_thread
            .cloned()
            .or_else(|| crate::thread::find_by_tid(issuing_tid));
        if let Some(t) = target {
            let func: crate::thread::IoCompletionRoutine = std::mem::transmute(completion_routine);
            crate::thread::queue_io_completion_apc(&t, func, wsa_err, bytes as u32, ov);
        }
    } else {
        complete_overlapped_io(
            socket_handle as HANDLE,
            ov_ptr,
            nt_status,
            bytes,
            synchronous,
        );
    }
}

fn ensure_async_io_started(st: &mut AsyncIoInner) {
    if st.started {
        return;
    }
    let mut fds = [-1i32; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
        log::warn!("ws2_32: pipe() failed for overlapped async reactor");
        return;
    }
    for &fd in &fds {
        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFL);
            if flags >= 0 {
                libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
            }
            let fdflags = libc::fcntl(fd, libc::F_GETFD);
            if fdflags >= 0 {
                libc::fcntl(fd, libc::F_SETFD, fdflags | libc::FD_CLOEXEC);
            }
        }
    }
    st.wake_r = fds[0];
    st.wake_w = fds[1];
    st.started = true;
    let _ = std::thread::Builder::new()
        .name("ws2-overlapped-reactor".into())
        .spawn(async_io_reactor_loop);
}

fn async_io_enqueue(op: PendingSocketOp) {
    let mut st = ASYNC_IO.inner.lock().unwrap();
    ensure_async_io_started(&mut st);
    st.ops.push(op);
    let ww = st.wake_w;
    wake_fd_write(ww);
}

fn cleanup_cancelled_op(_op: &PendingSocketOp) {}

fn async_io_cancel_socket_and_sync(s: usize) {
    let cancelled: Vec<PendingSocketOp> = {
        let mut st = ASYNC_IO.inner.lock().unwrap();
        let mut removed = Vec::new();
        let mut i = 0;
        while i < st.ops.len() {
            if st.ops[i].socket_handle == s || st.ops[i].secondary_socket == Some(s) {
                removed.push(st.ops.remove(i));
            } else {
                i += 1;
            }
        }
        if st.started {
            let gen = st.poll_gen;
            let ww = st.wake_w;
            wake_fd_write(ww);
            let mut waited = st;
            while waited.poll_gen == gen {
                let (next, res) = ASYNC_IO
                    .gen_cv
                    .wait_timeout(waited, std::time::Duration::from_millis(50))
                    .unwrap();
                waited = next;
                if res.timed_out() {
                    break;
                }
            }
        }
        removed
    };
    if !cancelled.is_empty() {
        for op in cancelled {
            cleanup_cancelled_op(&op);
            unsafe {
                finish_overlapped_op(
                    op.socket_handle,
                    op.overlapped,
                    op.completion_routine,
                    op.issuing_thread.as_ref(),
                    op.issuing_tid,
                    STATUS_CANCELLED,
                    0,
                    false,
                );
            }
        }
        let _g = ASYNC_IO.inner.lock().unwrap();
        ASYNC_IO.gen_cv.notify_all();
    }
}

unsafe fn ws2_cancel_io_hook(h: HANDLE, ov: *mut OVERLAPPED, ex: bool) -> Option<bool> {
    let s = h as usize;
    if lookup_meta(s).is_none() {
        return None;
    }
    let cur_tid = crate::kernel32::threads::GetCurrentThreadId();
    let target_ov = ov as usize;
    let cancelled: Vec<PendingSocketOp> = {
        let mut st = ASYNC_IO.inner.lock().unwrap();
        let mut removed = Vec::new();
        let mut i = 0;
        while i < st.ops.len() {
            let matches_sock = st.ops[i].socket_handle == s || st.ops[i].secondary_socket == Some(s);
            let matches_filter = if ex {
                target_ov == 0 || st.ops[i].overlapped == target_ov
            } else {
                st.ops[i].issuing_tid == cur_tid
            };
            if matches_sock && matches_filter {
                removed.push(st.ops.remove(i));
            } else {
                i += 1;
            }
        }
        if !removed.is_empty() && st.started {
            let ww = st.wake_w;
            wake_fd_write(ww);
        }
        removed
    };
    let did_cancel = !cancelled.is_empty();
    if did_cancel {
        for op in cancelled {
            cleanup_cancelled_op(&op);
            finish_overlapped_op(
                op.socket_handle,
                op.overlapped,
                op.completion_routine,
                op.issuing_thread.as_ref(),
                op.issuing_tid,
                STATUS_CANCELLED,
                0,
                false,
            );
        }
        let _g = ASYNC_IO.inner.lock().unwrap();
        ASYNC_IO.gen_cv.notify_all();
    }
    Some(did_cancel)
}

unsafe fn try_accept_nonblocking(
    fd: i32,
    storage: &mut libc::sockaddr_storage,
    slen: &mut libc::socklen_t,
) -> Result<i32, i32> {
    // Underlying socket fds are permanently O_NONBLOCK.
    let cfd = libc::accept(
        fd,
        storage as *mut libc::sockaddr_storage as *mut libc::sockaddr,
        slen,
    );
    if cfd >= 0 {
        Ok(cfd)
    } else {
        Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL))
    }
}

unsafe fn bind_accepted_fd(s_accept: usize, accepted_fd: i32) -> Result<i32, u32> {
    set_nosigpipe(accepted_fd);
    set_fd_nonblocking(accepted_fd);
    let mut socks = SOCKETS.lock().unwrap();
    let Some(m) = socks.get_mut(&s_accept) else {
        let _ = libc::close(accepted_fd);
        return Err(WSAENOTSOCK);
    };
    let target_fd = m.fd;
    if accepted_fd != target_fd {
        if libc::dup2(accepted_fd, target_fd) < 0 {
            let e = last_errno_wsa();
            let _ = libc::close(accepted_fd);
            return Err(e);
        }
        let _ = libc::close(accepted_fd);
    }
    set_fd_nonblocking(target_fd);
    m.bound = true;
    m.listening = false;
    m.connecting = false;
    m.connected = true;
    Ok(target_fd)
}

unsafe fn write_accept_ex_sockaddrs(
    accept_fd: i32,
    fallback_peer: &libc::sockaddr_storage,
    fallback_peer_len: libc::socklen_t,
    out_buf: *mut u8,
    recv_data_len: u32,
    local_addr_len: u32,
    remote_addr_len: u32,
) {
    let local_dst = out_buf.add(recv_data_len as usize);
    let remote_dst = out_buf.add(recv_data_len as usize + local_addr_len as usize);
    std::ptr::write_bytes(local_dst, 0, local_addr_len as usize);
    std::ptr::write_bytes(remote_dst, 0, remote_addr_len as usize);

    let mut local_sa: libc::sockaddr_storage = std::mem::zeroed();
    let mut local_slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    if libc::getsockname(
        accept_fd,
        &mut local_sa as *mut _ as *mut libc::sockaddr,
        &mut local_slen,
    ) == 0
    {
        let mut avail = local_addr_len as i32;
        let _ = posix_to_win_sockaddr(
            &local_sa as *const _ as *const libc::sockaddr,
            local_slen,
            local_dst,
            &mut avail,
        );
    }

    let mut remote_sa: libc::sockaddr_storage = std::mem::zeroed();
    let mut remote_slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let got_peer = libc::getpeername(
        accept_fd,
        &mut remote_sa as *mut _ as *mut libc::sockaddr,
        &mut remote_slen,
    ) == 0;
    let (peer_ptr, peer_len) = if got_peer {
        (&remote_sa as *const _ as *const libc::sockaddr, remote_slen)
    } else {
        (fallback_peer as *const _ as *const libc::sockaddr, fallback_peer_len)
    };
    let mut avail = remote_addr_len as i32;
    let _ = posix_to_win_sockaddr(peer_ptr, peer_len, remote_dst, &mut avail);
}

unsafe fn reuse_socket_fd(s: usize) -> Result<(), u32> {
    let mut socks = SOCKETS.lock().unwrap();
    let Some(m) = socks.get_mut(&s) else {
        return Err(WSAENOTSOCK);
    };
    let new_fd = libc::socket(m.family, m.sock_type, m.protocol);
    if new_fd < 0 {
        return Err(last_errno_wsa());
    }
    set_nosigpipe(new_fd);
    set_fd_nonblocking(new_fd);
    if new_fd != m.fd {
        if libc::dup2(new_fd, m.fd) < 0 {
            let e = last_errno_wsa();
            let _ = libc::close(new_fd);
            return Err(e);
        }
        let _ = libc::close(new_fd);
    }
    set_fd_nonblocking(m.fd);
    m.bound = false;
    m.listening = false;
    m.connecting = false;
    m.connected = false;
    Ok(())
}

unsafe fn apply_transmit_post_flags(s: usize, fd: i32, flags: u32) -> Result<(), u32> {
    if (flags & (TF_DISCONNECT | TF_REUSE_SOCKET)) != 0 {
        let _ = libc::shutdown(fd, libc::SHUT_RDWR);
        if (flags & TF_REUSE_SOCKET) != 0 {
            reuse_socket_fd(s)?;
        } else if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
            m.connected = false;
            m.connecting = false;
        }
    }
    Ok(())
}

unsafe fn drive_pending_op(op: &mut PendingSocketOp, revents: i16) -> OpStep {
    if (revents & libc::POLLNVAL) != 0 {
        cleanup_cancelled_op(op);
        return OpStep::Complete {
            nt_status: STATUS_CANCELLED,
            bytes: 0,
        };
    }

    match &mut op.kind {
        AsyncOpKind::AcceptEx {
            s_accept,
            out_buf,
            recv_data_len,
            local_addr_len,
            remote_addr_len,
        } => {
            let mut peer_sa: libc::sockaddr_storage = std::mem::zeroed();
            let mut peer_slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
            match try_accept_nonblocking(op.poll_fd, &mut peer_sa, &mut peer_slen) {
                Ok(cfd) => {
                    let s_acc = *s_accept;
                    let buf_ptr = *out_buf as *mut u8;
                    let rlen = *recv_data_len;
                    let llen = *local_addr_len;
                    let rmlen = *remote_addr_len;
                    let accept_fd = match bind_accepted_fd(s_acc, cfd) {
                        Ok(fd) => fd,
                        Err(wsa) => {
                            return OpStep::Complete {
                                nt_status: win32_to_ntstatus(wsa),
                                bytes: 0,
                            };
                        }
                    };
                    write_accept_ex_sockaddrs(accept_fd, &peer_sa, peer_slen, buf_ptr, rlen, llen, rmlen);
                    if rlen == 0 {
                        return OpStep::Complete {
                            nt_status: STATUS_SUCCESS,
                            bytes: 0,
                        };
                    }
                    let n = libc::recv(accept_fd, buf_ptr as *mut c_void, rlen as usize, libc::MSG_DONTWAIT);
                    if n >= 0 {
                        OpStep::Complete {
                            nt_status: STATUS_SUCCESS,
                            bytes: n as usize,
                        }
                    } else {
                        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                        if e == libc::EAGAIN || e == libc::EWOULDBLOCK || e == libc::EINTR {
                            op.poll_fd = accept_fd;
                            op.poll_events = libc::POLLIN;
                            op.kind = AsyncOpKind::AcceptExRecv {
                                out_buf: buf_ptr as usize,
                                recv_data_len: rlen,
                            };
                            OpStep::Pending
                        } else {
                            OpStep::Complete {
                                nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                                bytes: 0,
                            }
                        }
                    }
                }
                Err(e) if e == libc::EAGAIN || e == libc::EWOULDBLOCK || e == libc::EINTR => OpStep::Pending,
                Err(e) => OpStep::Complete {
                    nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                    bytes: 0,
                },
            }
        }
        AsyncOpKind::AcceptExRecv {
            out_buf,
            recv_data_len,
        } => {
            let n = libc::recv(
                op.poll_fd,
                *out_buf as *mut c_void,
                *recv_data_len as usize,
                libc::MSG_DONTWAIT,
            );
            if n >= 0 {
                OpStep::Complete {
                    nt_status: STATUS_SUCCESS,
                    bytes: n as usize,
                }
            } else {
                let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                if e == libc::EAGAIN || e == libc::EWOULDBLOCK || e == libc::EINTR {
                    OpStep::Pending
                } else {
                    OpStep::Complete {
                        nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                        bytes: 0,
                    }
                }
            }
        }
        AsyncOpKind::ConnectEx {
            send_buf,
            send_len,
            sent_so_far,
            connected,
        } => {
            if !*connected {
                let so_err = query_so_error(op.poll_fd);
                if so_err != 0 {
                    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&op.socket_handle) {
                        m.connecting = false;
                        m.connected = false;
                    }
                    return OpStep::Complete {
                        nt_status: win32_to_ntstatus(errno_to_wsa(so_err)),
                        bytes: 0,
                    };
                }
                if let Some(m) = SOCKETS.lock().unwrap().get_mut(&op.socket_handle) {
                    m.connecting = false;
                    m.connected = true;
                }
                *connected = true;
            }
            if *send_len == 0 || *sent_so_far >= *send_len {
                return OpStep::Complete {
                    nt_status: STATUS_SUCCESS,
                    bytes: *sent_so_far as usize,
                };
            }
            loop {
                let rem = (*send_len - *sent_so_far) as usize;
                let ptr = (*send_buf as *const u8).add(*sent_so_far as usize);
                let n = libc::send(op.poll_fd, ptr as *const c_void, rem, libc::MSG_DONTWAIT);
                if n > 0 {
                    *sent_so_far += n as u32;
                    if *sent_so_far >= *send_len {
                        return OpStep::Complete {
                            nt_status: STATUS_SUCCESS,
                            bytes: *sent_so_far as usize,
                        };
                    }
                } else if n == 0 {
                    return OpStep::Complete {
                        nt_status: STATUS_SUCCESS,
                        bytes: *sent_so_far as usize,
                    };
                } else {
                    let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                    if e == libc::EINTR {
                        continue;
                    }
                    if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
                        return OpStep::Pending;
                    }
                    return OpStep::Complete {
                        nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                        bytes: *sent_so_far as usize,
                    };
                }
            }
        }
        AsyncOpKind::TransmitFile(state) => match state.step(op.poll_fd, true) {
            Ok(true) => {
                let total = state.total_sent;
                if let Err(wsa) = apply_transmit_post_flags(op.socket_handle, op.poll_fd, state.flags) {
                    OpStep::Complete {
                        nt_status: win32_to_ntstatus(wsa),
                        bytes: total,
                    }
                } else {
                    OpStep::Complete {
                        nt_status: STATUS_SUCCESS,
                        bytes: total,
                    }
                }
            }
            Ok(false) => OpStep::Pending,
            Err(e) => OpStep::Complete {
                nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                bytes: state.total_sent,
            },
        },
        AsyncOpKind::Recv {
            bufs,
            pflags,
            from,
            fromlen,
            zero_len,
        } => {
            if *zero_len && *from == 0 {
                let so_err = query_so_error(op.poll_fd);
                return if so_err != 0 {
                    OpStep::Complete {
                        nt_status: win32_to_ntstatus(errno_to_wsa(so_err)),
                        bytes: 0,
                    }
                } else {
                    OpStep::Complete {
                        nt_status: STATUS_SUCCESS,
                        bytes: 0,
                    }
                };
            }
            let mut storage: libc::sockaddr_storage = std::mem::zeroed();
            let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
            let mut msg_flags = 0i32;
            let n = if *from != 0 {
                recvmsg_bufs(
                    op.poll_fd,
                    bufs,
                    *pflags | libc::MSG_DONTWAIT,
                    Some((&mut storage, &mut slen)),
                    &mut msg_flags,
                )
            } else {
                recvmsg_bufs(
                    op.poll_fd,
                    bufs,
                    *pflags | libc::MSG_DONTWAIT,
                    None,
                    &mut msg_flags,
                )
            };
            if n >= 0 {
                if *from != 0 && *fromlen != 0 {
                    let _ = posix_to_win_sockaddr(
                        &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
                        slen,
                        *from as *mut u8,
                        *fromlen as *mut i32,
                    );
                }
                OpStep::Complete {
                    nt_status: STATUS_SUCCESS,
                    bytes: n as usize,
                }
            } else {
                let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                if e == libc::EAGAIN || e == libc::EWOULDBLOCK || e == libc::EINTR {
                    OpStep::Pending
                } else {
                    OpStep::Complete {
                        nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                        bytes: 0,
                    }
                }
            }
        }
        AsyncOpKind::Send {
            bufs,
            total_len,
            sent_so_far,
            is_stream,
            pflags,
            to,
        } => match step_overlapped_send(
            op.poll_fd,
            bufs,
            *total_len,
            sent_so_far,
            *is_stream,
            *pflags,
            to.as_ref(),
        ) {
            OverlappedSendStep::Complete(bytes) => OpStep::Complete {
                nt_status: STATUS_SUCCESS,
                bytes,
            },
            OverlappedSendStep::Pending => OpStep::Pending,
            OverlappedSendStep::Error(e) => OpStep::Complete {
                nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                bytes: *sent_so_far,
            },
        },
    }
}

fn async_io_reactor_loop() {
    let mut pfds: Vec<libc::pollfd> = Vec::new();
    let mut completed: Vec<CompletedOp> = Vec::new();

    loop {
        pfds.clear();
        let wake_r = {
            let mut st = ASYNC_IO.inner.lock().unwrap();
            st.poll_gen = st.poll_gen.wrapping_add(1);
            ASYNC_IO.gen_cv.notify_all();
            let wr = st.wake_r;
            pfds.push(libc::pollfd {
                fd: wr,
                events: libc::POLLIN,
                revents: 0,
            });
            for op in &st.ops {
                if op.poll_fd >= 0 && op.poll_events != 0 {
                    if let Some(existing) = pfds[1..].iter_mut().find(|p| p.fd == op.poll_fd) {
                        existing.events |= op.poll_events;
                    } else {
                        pfds.push(libc::pollfd {
                            fd: op.poll_fd,
                            events: op.poll_events,
                            revents: 0,
                        });
                    }
                }
            }
            wr
        };

        let rc = unsafe { libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, -1) };
        if rc < 0 {
            let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            if e == libc::EINTR {
                continue;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
            continue;
        }

        if (pfds[0].revents & libc::POLLIN) != 0 {
            let mut drain = [0u8; 64];
            loop {
                let n = unsafe { libc::read(wake_r, drain.as_mut_ptr() as *mut c_void, drain.len()) };
                if n <= 0 {
                    break;
                }
            }
        }

        completed.clear();
        {
            let mut st = ASYNC_IO.inner.lock().unwrap();
            let mut idx = 0;
            while idx < st.ops.len() {
                let op_fd = st.ops[idx].poll_fd;
                let op_ev = st.ops[idx].poll_events;
                let revents = pfds[1..]
                    .iter()
                    .find(|p| p.fd == op_fd)
                    .map(|p| p.revents)
                    .unwrap_or(0);
                if revents == 0
                    || (revents & (op_ev | libc::POLLERR | libc::POLLHUP | libc::POLLNVAL)) == 0
                {
                    idx += 1;
                    continue;
                }
                match unsafe { drive_pending_op(&mut st.ops[idx], revents) } {
                    OpStep::Pending => {
                        idx += 1;
                    }
                    OpStep::Complete { nt_status, bytes } => {
                        let op = st.ops.remove(idx);
                        completed.push(CompletedOp {
                            socket_handle: op.socket_handle,
                            overlapped: op.overlapped,
                            completion_routine: op.completion_routine,
                            issuing_thread: op.issuing_thread,
                            issuing_tid: op.issuing_tid,
                            nt_status,
                            bytes,
                        });
                    }
                }
            }
        }

        if !completed.is_empty() {
            for c in completed.drain(..) {
                unsafe {
                    finish_overlapped_op(
                        c.socket_handle,
                        c.overlapped,
                        c.completion_routine,
                        c.issuing_thread.as_ref(),
                        c.issuing_tid,
                        c.nt_status,
                        c.bytes,
                        false,
                    );
                }
            }
            let _g = ASYNC_IO.inner.lock().unwrap();
            ASYNC_IO.gen_cv.notify_all();
        }
    }
}

// ------------------------------------------------ Winsock2 extension functions (WSAIoctl)

unsafe extern "win64" fn ext_accept_ex(
    s_listen: usize,
    s_accept: usize,
    out_buf: *mut u8,
    recv_data_len: u32,
    local_addr_len: u32,
    remote_addr_len: u32,
    bytes_received: *mut u32,
    ov: *mut OVERLAPPED,
) -> BOOL {
    let Some(listen_meta) = lookup_meta(s_listen) else {
        return err(WSAENOTSOCK, FALSE);
    };
    if lookup_meta(s_accept).is_none() {
        return err(WSAENOTSOCK, FALSE);
    }
    if s_listen == s_accept {
        return err(WSAEINVAL, FALSE);
    }
    if out_buf.is_null() || ov.is_null() || local_addr_len < 16 || remote_addr_len < 16 {
        return err(WSAEFAULT, FALSE);
    }
    let min_sa = if listen_meta.family == libc::AF_INET6 { 28 } else { 16 };
    if local_addr_len < min_sa || remote_addr_len < min_sa {
        return err(WSAEFAULT, FALSE);
    }

    let mut peer_sa: libc::sockaddr_storage = std::mem::zeroed();
    let mut peer_slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    match try_accept_nonblocking(listen_meta.fd, &mut peer_sa, &mut peer_slen) {
        Ok(cfd) => {
            let accept_fd = match bind_accepted_fd(s_accept, cfd) {
                Ok(fd) => fd,
                Err(wsa) => return err(wsa, FALSE),
            };
            write_accept_ex_sockaddrs(
                accept_fd,
                &peer_sa,
                peer_slen,
                out_buf,
                recv_data_len,
                local_addr_len,
                remote_addr_len,
            );
            if recv_data_len == 0 {
                if !bytes_received.is_null() {
                    std::ptr::write_unaligned(bytes_received, 0);
                }
                complete_overlapped_io(s_listen as HANDLE, ov, STATUS_SUCCESS, 0, true);
                return TRUE;
            }
            let n = libc::recv(
                accept_fd,
                out_buf as *mut c_void,
                recv_data_len as usize,
                libc::MSG_DONTWAIT,
            );
            if n >= 0 {
                if !bytes_received.is_null() {
                    std::ptr::write_unaligned(bytes_received, n as u32);
                }
                complete_overlapped_io(s_listen as HANDLE, ov, STATUS_SUCCESS, n as usize, true);
                TRUE
            } else {
                let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                if raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK {
                    prepare_socket_pending(s_listen, ov, 0);
                    async_io_enqueue(PendingSocketOp {
                        socket_handle: s_listen,
                        secondary_socket: Some(s_accept),
                        poll_fd: accept_fd,
                        poll_events: libc::POLLIN,
                        overlapped: ov as usize,
                        issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
                        issuing_thread: crate::thread::current_thread_object(),
                        completion_routine: 0,
                        kind: AsyncOpKind::AcceptExRecv {
                            out_buf: out_buf as usize,
                            recv_data_len,
                        },
                    });
                    err(WSA_IO_PENDING, FALSE)
                } else {
                    err(errno_to_wsa(raw_e), FALSE)
                }
            }
        }
        Err(raw_e) if raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK => {
            prepare_socket_pending(s_listen, ov, 0);
            async_io_enqueue(PendingSocketOp {
                socket_handle: s_listen,
                secondary_socket: Some(s_accept),
                poll_fd: listen_meta.fd,
                poll_events: libc::POLLIN,
                overlapped: ov as usize,
                issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
                issuing_thread: crate::thread::current_thread_object(),
                completion_routine: 0,
                kind: AsyncOpKind::AcceptEx {
                    s_accept,
                    out_buf: out_buf as usize,
                    recv_data_len,
                    local_addr_len,
                    remote_addr_len,
                },
            });
            err(WSA_IO_PENDING, FALSE)
        }
        Err(raw_e) => err(errno_to_wsa(raw_e), FALSE),
    }
}

unsafe extern "win64" fn ext_get_accept_ex_sockaddrs(
    out_buf: *const u8,
    recv_data_len: u32,
    local_addr_len: u32,
    remote_addr_len: u32,
    local_sa: *mut *mut u8,
    local_sa_len: *mut i32,
    remote_sa: *mut *mut u8,
    remote_sa_len: *mut i32,
) {
    if out_buf.is_null() {
        return;
    }
    let sockaddr_size = |ptr: *const u8, slot_len: u32| -> i32 {
        if slot_len < 2 {
            return 0;
        }
        let fam = (ptr as *const u16).read_unaligned();
        match fam {
            AF_INET => 16.min(slot_len as i32),
            AF_INET6 => 28.min(slot_len as i32),
            _ => 0,
        }
    };
    let local_ptr = out_buf.add(recv_data_len as usize) as *mut u8;
    let remote_ptr = out_buf.add(recv_data_len as usize + local_addr_len as usize) as *mut u8;
    if !local_sa.is_null() {
        std::ptr::write_unaligned(local_sa, local_ptr);
    }
    if !local_sa_len.is_null() {
        std::ptr::write_unaligned(local_sa_len, sockaddr_size(local_ptr, local_addr_len));
    }
    if !remote_sa.is_null() {
        std::ptr::write_unaligned(remote_sa, remote_ptr);
    }
    if !remote_sa_len.is_null() {
        std::ptr::write_unaligned(remote_sa_len, sockaddr_size(remote_ptr, remote_addr_len));
    }
}

unsafe extern "win64" fn ext_connect_ex(
    s: usize,
    name: *const u8,
    namelen: i32,
    send_buf: *const u8,
    send_len: u32,
    bytes_sent: *mut u32,
    ov: *mut OVERLAPPED,
) -> BOOL {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, FALSE);
    };
    if !meta.bound {
        return err(WSAEINVAL, FALSE);
    }
    if name.is_null() || ov.is_null() || (send_len > 0 && send_buf.is_null()) {
        return err(WSAEFAULT, FALSE);
    }
    let (storage, plen) = match win_to_posix_sockaddr(name, namelen) {
        Ok(v) => v,
        Err(e) => return err(e, FALSE),
    };

    let rc = libc::connect(
        meta.fd,
        &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
        plen,
    );
    if rc == 0 {
        if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
            m.connecting = false;
            m.connected = true;
        }
        if send_len == 0 {
            if !bytes_sent.is_null() {
                std::ptr::write_unaligned(bytes_sent, 0);
            }
            complete_overlapped_io(s as HANDLE, ov, STATUS_SUCCESS, 0, true);
            return TRUE;
        }
        let mut sent_so_far = 0u32;
        while sent_so_far < send_len {
            let n = libc::send(
                meta.fd,
                send_buf.add(sent_so_far as usize) as *const c_void,
                (send_len - sent_so_far) as usize,
                libc::MSG_DONTWAIT,
            );
            if n > 0 {
                sent_so_far += n as u32;
            } else if n == 0 {
                break;
            } else {
                let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                if raw_e == libc::EINTR {
                    continue;
                }
                if raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK {
                    prepare_socket_pending(s, ov, 0);
                    async_io_enqueue(PendingSocketOp {
                        socket_handle: s,
                        secondary_socket: None,
                        poll_fd: meta.fd,
                        poll_events: libc::POLLOUT,
                        overlapped: ov as usize,
                        issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
                        issuing_thread: crate::thread::current_thread_object(),
                        completion_routine: 0,
                        kind: AsyncOpKind::ConnectEx {
                            send_buf: send_buf as usize,
                            send_len,
                            sent_so_far,
                            connected: true,
                        },
                    });
                    return err(WSA_IO_PENDING, FALSE);
                }
                return err(errno_to_wsa(raw_e), FALSE);
            }
        }
        if !bytes_sent.is_null() {
            std::ptr::write_unaligned(bytes_sent, sent_so_far);
        }
        complete_overlapped_io(s as HANDLE, ov, STATUS_SUCCESS, sent_so_far as usize, true);
        return TRUE;
    }

    let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
    if raw_e == libc::EINPROGRESS {
        if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
            m.connecting = true;
            m.connected = false;
        }
        prepare_socket_pending(s, ov, 0);
        async_io_enqueue(PendingSocketOp {
            socket_handle: s,
            secondary_socket: None,
            poll_fd: meta.fd,
            poll_events: libc::POLLOUT,
            overlapped: ov as usize,
            issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
            issuing_thread: crate::thread::current_thread_object(),
            completion_routine: 0,
            kind: AsyncOpKind::ConnectEx {
                send_buf: send_buf as usize,
                send_len,
                sent_so_far: 0,
                connected: false,
            },
        });
        return err(WSA_IO_PENDING, FALSE);
    }

    err(errno_to_wsa(raw_e), FALSE)
}

unsafe extern "win64" fn ext_disconnect_ex(
    s: usize,
    ov: *mut OVERLAPPED,
    flags: u32,
    reserved: u32,
) -> BOOL {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, FALSE);
    };
    if reserved != 0 || (flags & !TF_REUSE_SOCKET) != 0 {
        return err(WSAEINVAL, FALSE);
    }
    let _ = libc::shutdown(meta.fd, libc::SHUT_RDWR);
    if (flags & TF_REUSE_SOCKET) != 0 {
        async_io_cancel_socket_and_sync(s);
        if let Err(wsa) = reuse_socket_fd(s) {
            return err(wsa, FALSE);
        }
    } else if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.connected = false;
        m.connecting = false;
    }
    if !ov.is_null() {
        complete_overlapped_io(s as HANDLE, ov, STATUS_SUCCESS, 0, true);
    }
    TRUE
}

unsafe extern "win64" fn ext_transmit_file(
    s: usize,
    h_file: HANDLE,
    bytes_to_write: u32,
    bytes_per_send: u32,
    ov: *mut OVERLAPPED,
    buffers: *const c_void,
    flags: u32,
) -> BOOL {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, FALSE);
    };
    let file_obj = if !h_file.is_null() && h_file != INVALID_HANDLE_VALUE {
        let Some(f) = get_file(h_file) else {
            return err(crate::types::err::ERROR_INVALID_HANDLE, FALSE);
        };
        Some(f)
    } else {
        None
    };

    let (head, tail) = if !buffers.is_null() {
        let tfb = std::ptr::read_unaligned(buffers as *const TransmitFileBuffers);
        if (tfb.head_length > 0 && tfb.head.is_null()) || (tfb.tail_length > 0 && tfb.tail.is_null()) {
            return err(WSAEFAULT, FALSE);
        }
        let h = if tfb.head_length > 0 {
            std::slice::from_raw_parts(tfb.head, tfb.head_length as usize).to_vec()
        } else {
            Vec::new()
        };
        let t = if tfb.tail_length > 0 {
            std::slice::from_raw_parts(tfb.tail, tfb.tail_length as usize).to_vec()
        } else {
            Vec::new()
        };
        (h, t)
    } else {
        (Vec::new(), Vec::new())
    };

    let (file_offset, file_remaining, update_file_pos) = if let Some(ref f) = file_obj {
        let off = if !ov.is_null() {
            let lo = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).offset)) as u64;
            let hi = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).offset_high)) as u64;
            lo | (hi << 32)
        } else {
            let cur = libc::lseek(f.fd(), 0, libc::SEEK_CUR);
            if cur >= 0 { cur as u64 } else { 0 }
        };
        let size = f.file().metadata().map(|m| m.len()).unwrap_or(0);
        let avail = size.saturating_sub(off);
        let rem = if bytes_to_write == 0 {
            avail
        } else {
            avail.min(bytes_to_write as u64)
        };
        (off, rem, ov.is_null())
    } else {
        (0, 0, false)
    };

    let chunk_size = if bytes_per_send == 0 {
        65536
    } else {
        (bytes_per_send as usize).clamp(1, 256 * 1024)
    };

    let mut state = TransmitState {
        head,
        file: file_obj,
        file_offset,
        file_remaining,
        update_file_pos,
        tail,
        chunk_size,
        staging: Vec::new(),
        staging_pos: 0,
        total_sent: 0,
        flags,
    };

    if ov.is_null() {
        return match state.step(meta.fd, false) {
            Ok(_) => {
                if let Err(wsa) = apply_transmit_post_flags(s, meta.fd, state.flags) {
                    err(wsa, FALSE)
                } else {
                    TRUE
                }
            }
            Err(e) => err(errno_to_wsa(e), FALSE),
        };
    }

    match state.step(meta.fd, true) {
        Ok(true) => {
            let total = state.total_sent;
            if let Err(wsa) = apply_transmit_post_flags(s, meta.fd, state.flags) {
                return err(wsa, FALSE);
            }
            complete_overlapped_io(s as HANDLE, ov, STATUS_SUCCESS, total, true);
            TRUE
        }
        Ok(false) => {
            prepare_socket_pending(s, ov, 0);
            async_io_enqueue(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: meta.fd,
                poll_events: libc::POLLOUT,
                overlapped: ov as usize,
                issuing_tid: crate::kernel32::threads::GetCurrentThreadId(),
                issuing_thread: crate::thread::current_thread_object(),
                completion_routine: 0,
                kind: AsyncOpKind::TransmitFile(state),
            });
            err(WSA_IO_PENDING, FALSE)
        }
        Err(e) => err(errno_to_wsa(e), FALSE),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inet_addr_forms() {
        assert_eq!(parse_inet_addr("127.0.0.1"), Some(0x7F00_0001));
        assert_eq!(parse_inet_addr("0x7f.1"), Some(0x7F00_0001));
        assert_eq!(parse_inet_addr("10.0.258"), Some(0x0A00_0102));
        assert_eq!(parse_inet_addr("256.1.1.1"), None);
        assert_eq!(parse_inet_addr("a.b"), None);
    }

    #[test]
    fn pton_ntop_roundtrip() {
        unsafe {
            let mut b = [0u8; 16];
            assert_eq!(inet_pton(23, b"::1\0".as_ptr(), b.as_mut_ptr()), 1);
            let mut out = [0u8; 64];
            let p = inet_ntop(23, b.as_ptr(), out.as_mut_ptr(), 64);
            assert_eq!(crate::strings::from_ansi(p), "::1");
            assert_eq!(inet_pton(2, b"1.2.3.4\0".as_ptr(), b.as_mut_ptr()), 1);
            assert_eq!(&b[..4], &[1, 2, 3, 4]);
            assert_eq!(crate::strings::from_ansi(inet_ntoa(u32::from_ne_bytes([1, 2, 3, 4]))), "1.2.3.4");
        }
    }

    #[test]
    fn string_to_address_and_back() {
        unsafe {
            let w = crate::strings::to_wide_z("192.168.1.2:8080");
            let mut sa = [0u8; 16];
            let mut len = 16i32;
            assert_eq!(WSAStringToAddressW(w.as_ptr(), 2, std::ptr::null_mut(), sa.as_mut_ptr(), &mut len), 0);
            let mut out = [0u16; 64];
            let mut ol = 64u32;
            assert_eq!(WSAAddressToStringW(sa.as_ptr(), 16, std::ptr::null_mut(), out.as_mut_ptr(), &mut ol), 0);
            assert_eq!(crate::strings::from_wide(out.as_ptr()), "192.168.1.2:8080");
        }
    }

    #[test]
    fn tcp_loopback_socket_and_getaddrinfo() {
        unsafe {
            let mut ai: *mut c_void = std::ptr::null_mut();
            assert_eq!(getaddrinfo(b"127.0.0.1\0".as_ptr(), b"0\0".as_ptr(), std::ptr::null(), &mut ai), 0);
            assert!(!ai.is_null());
            let node = std::ptr::read_unaligned(ai as *const WinAddrInfo);
            assert_eq!(node.ai_family, AF_INET as i32);
            assert_eq!(node.ai_addrlen, 16);
            freeaddrinfo(ai);

            let srv = WSASocketW(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP, std::ptr::null(), 0, 0);
            assert_ne!(srv, INVALID_SOCKET);
            let one: i32 = 1;
            assert_eq!(setsockopt(srv, 0xFFFF, 0x0004, &one as *const i32 as *const u8, 4), 0);

            let mut sa = [0u8; 16];
            (sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);
            assert_eq!(listen(srv, 4), 0);

            let cli = WSASocketW(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP, std::ptr::null(), 0, 0);
            assert_ne!(cli, INVALID_SOCKET);
            let mut ideal_backlog = 0u32;
            let mut returned = 0u32;
            assert_eq!(
                WSAIoctl(
                    cli,
                    0x4004_747B,
                    std::ptr::null_mut(),
                    0,
                    &mut ideal_backlog as *mut u32 as *mut c_void,
                    4,
                    &mut returned,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_eq!(returned, 4);
            assert!(ideal_backlog >= 65536);
            assert_eq!(WSAConnect(cli, sa.as_ptr(), 16, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()), 0);
            let mut peer = [0u8; 16];
            let mut plen = 16i32;
            let acc = accept(srv, peer.as_mut_ptr(), &mut plen);
            assert_ne!(acc, INVALID_SOCKET);

            let msg = b"hello-ws2";
            assert_eq!(send(cli, msg.as_ptr(), msg.len() as i32, 0), msg.len() as i32);
            let mut pfd = WinPollFd { fd: acc, events: 0x0100, revents: 0 };
            assert_eq!(WSAPoll(&mut pfd, 1, 1000), 1);
            assert_ne!(pfd.revents & 0x0100, 0);
            let mut buf = [0u8; 32];
            let got = recv(acc, buf.as_mut_ptr(), buf.len() as i32, 0);
            assert_eq!(got, msg.len() as i32);
            assert_eq!(&buf[..got as usize], msg);

            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
        }
    }

    #[test]
    fn wsa_event_select_and_enum_network_events() {
        unsafe {
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(srv, INVALID_SOCKET);
            let mut sa = [0u8; 16];
            (sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);
            assert_eq!(listen(srv, 4), 0);

            let srv_ev = WSACreateEvent();
            assert!(!srv_ev.is_null());
            assert_eq!(WSAEventSelect(srv, srv_ev, FD_ACCEPT), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli, INVALID_SOCKET);
            let cli_ev = WSACreateEvent();
            assert!(!cli_ev.is_null());
            assert_eq!(WSAEventSelect(cli, cli_ev, FD_CONNECT | FD_READ | FD_WRITE | FD_CLOSE), 0);

            let rc = connect(cli, sa.as_ptr(), 16);
            assert!(rc == 0 || WSAGetLastError() == WSAEWOULDBLOCK as i32, "rc={rc} err={}", WSAGetLastError());

            assert_eq!(WSAWaitForMultipleEvents(1, &srv_ev, FALSE, 2000, FALSE), 0);
            let mut ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(srv, srv_ev, &mut ne), 0);
            assert_ne!(ne.l_network_events & FD_ACCEPT, 0);

            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);
            let acc_ev = WSACreateEvent();
            assert_eq!(WSAEventSelect(acc, acc_ev, FD_READ | FD_CLOSE), 0);

            assert_eq!(WSAWaitForMultipleEvents(1, &cli_ev, FALSE, 2000, FALSE), 0);
            let mut cli_ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(cli, cli_ev, &mut cli_ne), 0);
            assert_ne!(cli_ne.l_network_events & FD_CONNECT, 0);
            assert_eq!(cli_ne.i_error_code[FD_CONNECT_BIT], 0);
            assert_ne!(cli_ne.l_network_events & FD_WRITE, 0);

            let payload = b"event-select-ok";
            assert_eq!(send(cli, payload.as_ptr(), payload.len() as i32, 0), payload.len() as i32);

            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 2000, FALSE), 0);
            let mut acc_ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut acc_ne), 0);
            assert_ne!(acc_ne.l_network_events & FD_READ, 0);

            let mut buf = [0u8; 32];
            let got = recv(acc, buf.as_mut_ptr(), buf.len() as i32, 0);
            assert_eq!(got, payload.len() as i32);
            assert_eq!(&buf[..got as usize], payload);

            // Close client and verify FD_CLOSE fires on accepted socket.
            assert_eq!(closesocket(cli), 0);
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 2000, FALSE), 0);
            let mut close_ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut close_ne), 0);
            assert_ne!(close_ne.l_network_events & FD_CLOSE, 0);
            assert_eq!(close_ne.i_error_code[FD_CLOSE_BIT], 0);

            assert_eq!(WSAEventSelect(acc, std::ptr::null_mut(), 0), 0);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(WSACloseEvent(cli_ev), TRUE);
            assert_eq!(WSACloseEvent(acc_ev), TRUE);
            assert_eq!(WSACloseEvent(srv_ev), TRUE);
        }
    }

    #[test]
    fn wsa_duplicate_socket_roundtrip() {
        unsafe {
            let s1 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(s1, INVALID_SOCKET);
            let mut sa = [0u8; 16];
            (sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(s1, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(s1, sa.as_mut_ptr(), &mut slen), 0);

            let mut info = [0u8; 628];
            assert_eq!(WSADuplicateSocketW(s1, std::process::id(), info.as_mut_ptr()), 0);
            let s2 = WSASocketW(-1, -1, -1, info.as_ptr() as *const c_void, 0, 0);
            assert_ne!(s2, INVALID_SOCKET);
            assert_ne!(s2, s1);

            let mut sa2 = [0u8; 16];
            let mut slen2 = 16i32;
            assert_eq!(getsockname(s2, sa2.as_mut_ptr(), &mut slen2), 0);
            assert_eq!(sa2, sa, "duplicated socket must share the same bound endpoint");

            assert_eq!(closesocket(s1), 0);
            assert_eq!(closesocket(s2), 0);
        }
    }

    type AcceptExFn = unsafe extern "win64" fn(
        usize,
        usize,
        *mut u8,
        u32,
        u32,
        u32,
        *mut u32,
        *mut OVERLAPPED,
    ) -> BOOL;
    type GetAcceptExSockaddrsFn = unsafe extern "win64" fn(
        *const u8,
        u32,
        u32,
        u32,
        *mut *mut u8,
        *mut i32,
        *mut *mut u8,
        *mut i32,
    );
    type ConnectExFn = unsafe extern "win64" fn(
        usize,
        *const u8,
        i32,
        *const u8,
        u32,
        *mut u32,
        *mut OVERLAPPED,
    ) -> BOOL;
    type DisconnectExFn = unsafe extern "win64" fn(usize, *mut OVERLAPPED, u32, u32) -> BOOL;
    type TransmitFileFn = unsafe extern "win64" fn(
        usize,
        HANDLE,
        u32,
        u32,
        *mut OVERLAPPED,
        *const c_void,
        u32,
    ) -> BOOL;

    unsafe fn query_ext_ptr(s: usize, guid: &[u8; 16]) -> Result<usize, i32> {
        const SIO_GET_EXTENSION_FUNCTION_POINTER: u32 = 0xC800_0006;
        let mut fn_ptr = 0usize;
        let mut returned = 0u32;
        let rc = WSAIoctl(
            s,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            guid.as_ptr() as *mut c_void,
            16,
            &mut fn_ptr as *mut usize as *mut c_void,
            std::mem::size_of::<usize>() as u32,
            &mut returned,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if rc == 0 {
            assert_eq!(returned, std::mem::size_of::<usize>() as u32);
            Ok(fn_ptr)
        } else {
            Err(WSAGetLastError())
        }
    }

    fn loopback_sa(port: u16) -> [u8; 16] {
        let mut sa = [0u8; 16];
        unsafe {
            (sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            (sa.as_mut_ptr().add(2) as *mut u16).write_unaligned(port.to_be());
        }
        sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
        sa
    }

    #[test]
    fn wsa_ioctl_extension_function_pointers() {
        const SIO_GET_EXTENSION_FUNCTION_POINTER: u32 = 0xC800_0006;
        unsafe {
            // Invalid socket -> WSAENOTSOCK
            let mut fn_ptr = 0usize;
            let mut returned = 0u32;
            assert_eq!(
                WSAIoctl(
                    0xDEAD_BEEF,
                    SIO_GET_EXTENSION_FUNCTION_POINTER,
                    WSAID_ACCEPTEX.as_ptr() as *mut c_void,
                    16,
                    &mut fn_ptr as *mut usize as *mut c_void,
                    8,
                    &mut returned,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAENOTSOCK as i32);

            let s = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(s, INVALID_SOCKET);

            // All 5 Windows extension GUIDs resolve to non-null pointers.
            let p_accept = query_ext_ptr(s, &WSAID_ACCEPTEX).unwrap();
            let p_connect = query_ext_ptr(s, &WSAID_CONNECTEX).unwrap();
            let p_disconnect = query_ext_ptr(s, &WSAID_DISCONNECTEX).unwrap();
            let p_transmit = query_ext_ptr(s, &WSAID_TRANSMITFILE).unwrap();
            let p_sockaddrs = query_ext_ptr(s, &WSAID_GETACCEPTEXSOCKADDRS).unwrap();
            assert_eq!(p_accept, ext_accept_ex as *const () as usize);
            assert_eq!(p_connect, ext_connect_ex as *const () as usize);
            assert_eq!(p_disconnect, ext_disconnect_ex as *const () as usize);
            assert_eq!(p_transmit, ext_transmit_file as *const () as usize);
            assert_eq!(p_sockaddrs, ext_get_accept_ex_sockaddrs as *const () as usize);

            // Unknown GUID -> WSAEOPNOTSUPP
            let bad_guid = [0xAAu8; 16];
            assert_eq!(query_ext_ptr(s, &bad_guid), Err(WSAEOPNOTSUPP as i32));

            // Null / undersized input or output buffer -> WSAEFAULT
            assert_eq!(
                WSAIoctl(
                    s,
                    SIO_GET_EXTENSION_FUNCTION_POINTER,
                    std::ptr::null_mut(),
                    16,
                    &mut fn_ptr as *mut usize as *mut c_void,
                    8,
                    &mut returned,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);

            assert_eq!(
                WSAIoctl(
                    s,
                    SIO_GET_EXTENSION_FUNCTION_POINTER,
                    WSAID_ACCEPTEX.as_ptr() as *mut c_void,
                    8,
                    &mut fn_ptr as *mut usize as *mut c_void,
                    8,
                    &mut returned,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);

            assert_eq!(
                WSAIoctl(
                    s,
                    SIO_GET_EXTENSION_FUNCTION_POINTER,
                    WSAID_ACCEPTEX.as_ptr() as *mut c_void,
                    16,
                    &mut fn_ptr as *mut usize as *mut c_void,
                    4,
                    &mut returned,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);

            assert_eq!(closesocket(s), 0);
        }
    }

    #[test]
    fn accept_ex_and_connect_ex_zero_and_initial_payload_with_iocp() {
        use crate::kernel32::file::{CreateIoCompletionPort, GetQueuedCompletionStatus};
        use crate::kernel32::threads::CloseHandle;

        unsafe {
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());

            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(srv, INVALID_SOCKET);
            assert_eq!(CreateIoCompletionPort(srv as HANDLE, iocp, 0x1111, 0), iocp);

            let mut srv_sa = loopback_sa(0);
            assert_eq!(bind(srv, srv_sa.as_ptr(), 16), 0);
            let mut srv_slen = 16i32;
            assert_eq!(getsockname(srv, srv_sa.as_mut_ptr(), &mut srv_slen), 0);
            assert_eq!(listen(srv, 8), 0);

            let accept_ex: AcceptExFn = std::mem::transmute(query_ext_ptr(srv, &WSAID_ACCEPTEX).unwrap());
            let get_sockaddrs: GetAcceptExSockaddrsFn =
                std::mem::transmute(query_ext_ptr(srv, &WSAID_GETACCEPTEXSOCKADDRS).unwrap());
            let connect_ex: ConnectExFn = std::mem::transmute(query_ext_ptr(srv, &WSAID_CONNECTEX).unwrap());

            // Case 1: 0 initial payload
            let acc1 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(acc1, INVALID_SOCKET);
            let mut acc_buf1 = [0u8; 64];
            let mut ov_acc1: OVERLAPPED = std::mem::zeroed();
            let mut got1 = 0u32;
            let r_acc1 = accept_ex(srv, acc1, acc_buf1.as_mut_ptr(), 0, 32, 32, &mut got1, &mut ov_acc1);
            assert_eq!(r_acc1, FALSE);
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let cli1 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli1, INVALID_SOCKET);
            assert_eq!(CreateIoCompletionPort(cli1 as HANDLE, iocp, 0x2222, 0), iocp);

            // ConnectEx requires bind() first -> WSAEINVAL before bind.
            let mut ov_cli1: OVERLAPPED = std::mem::zeroed();
            let mut sent1 = 0u32;
            assert_eq!(
                connect_ex(cli1, srv_sa.as_ptr(), 16, std::ptr::null(), 0, &mut sent1, &mut ov_cli1),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSAEINVAL as i32);

            let bind_sa = loopback_sa(0);
            assert_eq!(bind(cli1, bind_sa.as_ptr(), 16), 0);
            let mut cli1_local = [0u8; 16];
            let mut cli1_llen = 16i32;
            assert_eq!(getsockname(cli1, cli1_local.as_mut_ptr(), &mut cli1_llen), 0);

            let r_cli1 = connect_ex(cli1, srv_sa.as_ptr(), 16, std::ptr::null(), 0, &mut sent1, &mut ov_cli1);
            assert!(r_cli1 == TRUE || WSAGetLastError() == WSA_IO_PENDING as i32);

            // Drain both completions (AcceptEx on key 0x1111 and ConnectEx on key 0x2222).
            let mut saw_acc1 = false;
            let mut saw_cli1 = false;
            for _ in 0..2 {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
                if key == 0x1111 {
                    assert_eq!(ov_out, &mut ov_acc1 as *mut OVERLAPPED);
                    assert_eq!(bytes, 0);
                    saw_acc1 = true;
                } else if key == 0x2222 {
                    assert_eq!(ov_out, &mut ov_cli1 as *mut OVERLAPPED);
                    assert_eq!(bytes, 0);
                    saw_cli1 = true;
                } else {
                    panic!("unexpected completion key {key:#x}");
                }
            }
            assert!(saw_acc1 && saw_cli1);

            // SO_UPDATE_ACCEPT_CONTEXT & SO_UPDATE_CONNECT_CONTEXT
            assert_eq!(
                setsockopt(acc1, 0xFFFF, 0x700B, &srv as *const usize as *const u8, std::mem::size_of::<usize>() as i32),
                0
            );
            assert_eq!(setsockopt(cli1, 0xFFFF, 0x7010, std::ptr::null(), 0), 0);

            // Verify GetAcceptExSockaddrs extracts local (srv_sa) and remote (cli1_local).
            let mut lsa: *mut u8 = std::ptr::null_mut();
            let mut lsa_len = 0i32;
            let mut rsa: *mut u8 = std::ptr::null_mut();
            let mut rsa_len = 0i32;
            get_sockaddrs(acc_buf1.as_ptr(), 0, 32, 32, &mut lsa, &mut lsa_len, &mut rsa, &mut rsa_len);
            assert_eq!(lsa_len, 16);
            assert_eq!(rsa_len, 16);
            assert_eq!(std::slice::from_raw_parts(lsa, 16), &srv_sa);
            assert_eq!(std::slice::from_raw_parts(rsa, 16), &cli1_local);

            // Case 2: Initial send/recv payload via ConnectEx + AcceptEx
            let acc2 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(acc2, INVALID_SOCKET);
            let mut acc_buf2 = [0u8; 128];
            let mut ov_acc2: OVERLAPPED = std::mem::zeroed();
            let mut got2 = 0u32;
            assert_eq!(
                accept_ex(srv, acc2, acc_buf2.as_mut_ptr(), 64, 32, 32, &mut got2, &mut ov_acc2),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let cli2 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli2, INVALID_SOCKET);
            assert_eq!(CreateIoCompletionPort(cli2 as HANDLE, iocp, 0x3333, 0), iocp);
            assert_eq!(bind(cli2, bind_sa.as_ptr(), 16), 0);

            let initial_msg = b"hello-connectex-payload";
            let mut ov_cli2: OVERLAPPED = std::mem::zeroed();
            let mut sent2 = 0u32;
            let r_cli2 = connect_ex(
                cli2,
                srv_sa.as_ptr(),
                16,
                initial_msg.as_ptr(),
                initial_msg.len() as u32,
                &mut sent2,
                &mut ov_cli2,
            );
            assert!(r_cli2 == TRUE || WSAGetLastError() == WSA_IO_PENDING as i32);

            let mut saw_acc2 = false;
            let mut saw_cli2 = false;
            for _ in 0..2 {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
                if key == 0x1111 {
                    assert_eq!(ov_out, &mut ov_acc2 as *mut OVERLAPPED);
                    assert_eq!(bytes as usize, initial_msg.len());
                    assert_eq!(&acc_buf2[..bytes as usize], initial_msg);
                    saw_acc2 = true;
                } else if key == 0x3333 {
                    assert_eq!(ov_out, &mut ov_cli2 as *mut OVERLAPPED);
                    assert_eq!(bytes as usize, initial_msg.len());
                    saw_cli2 = true;
                } else {
                    panic!("unexpected completion key {key:#x}");
                }
            }
            assert!(saw_acc2 && saw_cli2);

            get_sockaddrs(acc_buf2.as_ptr(), 64, 32, 32, &mut lsa, &mut lsa_len, &mut rsa, &mut rsa_len);
            assert_eq!(lsa_len, 16);
            assert_eq!(rsa_len, 16);
            assert_eq!(std::slice::from_raw_parts(lsa, 16), &srv_sa);

            assert_eq!(closesocket(acc1), 0);
            assert_eq!(closesocket(cli1), 0);
            assert_eq!(closesocket(acc2), 0);
            assert_eq!(closesocket(cli2), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn overlapped_wsa_send_recv_multibuf_and_skip_on_success() {
        use crate::kernel32::file::{
            CreateIoCompletionPort, GetQueuedCompletionStatus, SetFileCompletionNotificationModes,
        };
        use crate::kernel32::threads::CloseHandle;

        unsafe {
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);
            assert_eq!(listen(srv, 4), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert_eq!(CreateIoCompletionPort(acc as HANDLE, iocp, 0xAAAA, 0), iocp);
            assert_eq!(CreateIoCompletionPort(cli as HANDLE, iocp, 0xBBBB, 0), iocp);

            // Start an async multi-buffer WSARecv BEFORE peer sends -> must pend (WSA_IO_PENDING).
            let mut part1 = [0u8; 5];
            let mut part2 = [0u8; 16];
            let rbufs = [
                WsaBuf { len: part1.len() as u32, buf: part1.as_mut_ptr() },
                WsaBuf { len: part2.len() as u32, buf: part2.as_mut_ptr() },
            ];
            let mut rflags = 0u32;
            let mut rbytes = 0u32;
            let mut ov_recv: OVERLAPPED = std::mem::zeroed();
            let ev_recv = WSACreateEvent();
            ov_recv.h_event = ev_recv;
            let r = WSARecv(
                acc,
                rbufs.as_ptr(),
                2,
                &mut rbytes,
                &mut rflags,
                &mut ov_recv as *mut OVERLAPPED as *mut c_void,
                std::ptr::null_mut(),
            );
            assert_eq!(r, SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            // Non-waiting WSAGetOverlappedResult must report WSA_IO_INCOMPLETE while pending.
            let mut xfer = 0u32;
            let mut out_fl = 0u32;
            assert_eq!(WSAGetOverlappedResult(acc, &mut ov_recv, &mut xfer, FALSE, &mut out_fl), FALSE);
            assert_eq!(WSAGetLastError(), WSA_IO_INCOMPLETE as i32);

            // Multi-buffer overlapped WSASend from cli -> completes immediately and posts to IOCP.
            let s1 = b"hello";
            let s2 = b"-multibuf!";
            let sbufs = [
                WsaBuf { len: s1.len() as u32, buf: s1.as_ptr() as *mut u8 },
                WsaBuf { len: s2.len() as u32, buf: s2.as_ptr() as *mut u8 },
            ];
            let mut ov_send: OVERLAPPED = std::mem::zeroed();
            let mut sbytes = 0u32;
            assert_eq!(
                WSASend(
                    cli,
                    sbufs.as_ptr(),
                    2,
                    &mut sbytes,
                    0,
                    &mut ov_send as *mut OVERLAPPED as *mut c_void,
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_eq!(sbytes, (s1.len() + s2.len()) as u32);

            // Wait for WSARecv via WSAGetOverlappedResult(fWait=TRUE).
            assert_eq!(WSAGetOverlappedResult(acc, &mut ov_recv, &mut xfer, TRUE, &mut out_fl), TRUE);
            assert_eq!(xfer, (s1.len() + s2.len()) as u32);
            assert_eq!(&part1, b"hello");
            assert_eq!(&part2[..s2.len()], b"-multibuf!");

            // Both WSASend (key 0xBBBB) and WSARecv (key 0xAAAA) must arrive at IOCP.
            let mut saw_send = false;
            let mut saw_recv = false;
            for _ in 0..2 {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
                if key == 0xAAAA {
                    assert_eq!(ov_out, &mut ov_recv as *mut OVERLAPPED);
                    assert_eq!(bytes, (s1.len() + s2.len()) as u32);
                    saw_recv = true;
                } else if key == 0xBBBB {
                    assert_eq!(ov_out, &mut ov_send as *mut OVERLAPPED);
                    assert_eq!(bytes, (s1.len() + s2.len()) as u32);
                    saw_send = true;
                }
            }
            assert!(saw_send && saw_recv);

            // Enable FILE_SKIP_COMPLETION_PORT_ON_SUCCESS (0x1) on cli: synchronous WSASend must NOT post to IOCP.
            assert_eq!(SetFileCompletionNotificationModes(cli as HANDLE, 0x1), TRUE);
            let mut ov_skip: OVERLAPPED = std::mem::zeroed();
            let msg = b"skip-iocp";
            let wb = WsaBuf { len: msg.len() as u32, buf: msg.as_ptr() as *mut u8 };
            assert_eq!(
                WSASend(
                    cli,
                    &wb,
                    1,
                    &mut sbytes,
                    0,
                    &mut ov_skip as *mut OVERLAPPED as *mut c_void,
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_eq!(sbytes, msg.len() as u32);

            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 20), FALSE);
            assert_eq!(crate::teb::last_error(), WAIT_TIMEOUT);

            // Drain sent bytes from acc synchronously.
            let mut drain = [0u8; 16];
            assert_eq!(recv(acc, drain.as_mut_ptr(), drain.len() as i32, 0), msg.len() as i32);
            assert_eq!(&drain[..msg.len()], msg);

            assert_eq!(WSACloseEvent(ev_recv), TRUE);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn transmit_file_with_head_file_and_tail_over_loopback() {
        use crate::kernel32::file::{CreateFileW, CreateIoCompletionPort, GetQueuedCompletionStatus};
        use crate::kernel32::threads::CloseHandle;

        unsafe {
            crate::path::add_mount("U:\\", std::path::Path::new("/"));
            let tmp = std::env::temp_dir().join(format!("win32_rt_ws2_tf_{}.bin", std::process::id()));
            std::fs::write(&tmp, b"0123456789BODY-CONTENT-END").unwrap();
            let win_p = crate::strings::to_wide_z(&format!("U:{}", tmp.to_string_lossy().replace('/', "\\")));
            let h_file = CreateFileW(
                win_p.as_ptr(),
                0x8000_0000, // GENERIC_READ
                7,
                std::ptr::null_mut(),
                3, // OPEN_EXISTING
                0x80,
                std::ptr::null_mut(),
            );
            assert_ne!(h_file, INVALID_HANDLE_VALUE);

            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);
            assert_eq!(listen(srv, 4), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert_eq!(CreateIoCompletionPort(cli as HANDLE, iocp, 0x7F7F, 0), iocp);

            let transmit_file: TransmitFileFn =
                std::mem::transmute(query_ext_ptr(cli, &WSAID_TRANSMITFILE).unwrap());

            let head = b"[HEAD]";
            let tail = b"[TAIL]";
            let tfb = TransmitFileBuffers {
                head: head.as_ptr(),
                head_length: head.len() as u32,
                _pad0: 0,
                tail: tail.as_ptr(),
                tail_length: tail.len() as u32,
                _pad1: 0,
            };
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.offset = 10; // skip "0123456789" -> read "BODY-CONTENT-END" (16 bytes)
            let ok = transmit_file(
                cli,
                h_file,
                0, // entire remaining file from offset 10
                0,
                &mut ov,
                &tfb as *const TransmitFileBuffers as *const c_void,
                0,
            );
            assert!(ok == TRUE || WSAGetLastError() == WSA_IO_PENDING as i32);

            let expected = b"[HEAD]BODY-CONTENT-END[TAIL]";
            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(key, 0x7F7F);
            assert_eq!(ov_out, &mut ov as *mut OVERLAPPED);
            assert_eq!(bytes as usize, expected.len());

            let mut received = Vec::new();
            let mut chunk = [0u8; 64];
            while received.len() < expected.len() {
                let n = recv(acc, chunk.as_mut_ptr(), chunk.len() as i32, 0);
                assert!(n > 0);
                received.extend_from_slice(&chunk[..n as usize]);
            }
            assert_eq!(received.as_slice(), expected);

            assert_eq!(CloseHandle(h_file), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            let _ = std::fs::remove_file(&tmp);
        }
    }

    #[test]
    fn disconnect_ex_reuse_socket_and_reconnect() {
        use crate::kernel32::file::{CreateIoCompletionPort, GetQueuedCompletionStatus};
        use crate::kernel32::threads::CloseHandle;

        unsafe {
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);
            assert_eq!(listen(srv, 4), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert_eq!(CreateIoCompletionPort(cli as HANDLE, iocp, 0x5555, 0), iocp);

            let connect_ex: ConnectExFn = std::mem::transmute(query_ext_ptr(cli, &WSAID_CONNECTEX).unwrap());
            let disconnect_ex: DisconnectExFn =
                std::mem::transmute(query_ext_ptr(cli, &WSAID_DISCONNECTEX).unwrap());

            // First connection
            let bind_sa = loopback_sa(0);
            assert_eq!(bind(cli, bind_sa.as_ptr(), 16), 0);
            let mut ov1: OVERLAPPED = std::mem::zeroed();
            let mut sent = 0u32;
            let rc1 = connect_ex(cli, sa.as_ptr(), 16, std::ptr::null(), 0, &mut sent, &mut ov1);
            assert!(rc1 == TRUE || WSAGetLastError() == WSA_IO_PENDING as i32);
            let acc1 = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc1, INVALID_SOCKET);

            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov1 as *mut OVERLAPPED);

            // DisconnectEx with TF_REUSE_SOCKET -> resets socket fd and keeps IOCP association.
            let mut ov_disc: OVERLAPPED = std::mem::zeroed();
            assert_eq!(disconnect_ex(cli, &mut ov_disc, TF_REUSE_SOCKET, 0), TRUE);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov_disc as *mut OVERLAPPED);
            assert_eq!(closesocket(acc1), 0);

            // Re-bind and reuse the exact same socket handle for a second connection!
            assert_eq!(bind(cli, bind_sa.as_ptr(), 16), 0);
            let mut ov2: OVERLAPPED = std::mem::zeroed();
            let payload = b"reused!";
            let rc2 = connect_ex(cli, sa.as_ptr(), 16, payload.as_ptr(), payload.len() as u32, &mut sent, &mut ov2);
            assert!(rc2 == TRUE || WSAGetLastError() == WSA_IO_PENDING as i32);
            let acc2 = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc2, INVALID_SOCKET);

            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov2 as *mut OVERLAPPED);
            assert_eq!(bytes as usize, payload.len());

            let mut buf = [0u8; 16];
            let got = recv(acc2, buf.as_mut_ptr(), buf.len() as i32, 0);
            assert_eq!(got as usize, payload.len());
            assert_eq!(&buf[..got as usize], payload);

            assert_eq!(closesocket(acc2), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn cancel_io_ex_and_cancel_io_abort_pending_accept_and_recv() {
        use crate::kernel32::file::{
            CancelIo, CancelIoEx, CreateIoCompletionPort, GetQueuedCompletionStatus,
        };
        use crate::kernel32::threads::CloseHandle;

        unsafe {
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_eq!(CreateIoCompletionPort(srv as HANDLE, iocp, 0xC001, 0), iocp);
            let mut sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);
            assert_eq!(listen(srv, 4), 0);

            let accept_ex: AcceptExFn = std::mem::transmute(query_ext_ptr(srv, &WSAID_ACCEPTEX).unwrap());

            // 1. Cancel pending AcceptEx with CancelIoEx(srv, &ov_acc).
            let acc_slot = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut acc_buf = [0u8; 64];
            let mut ov_acc: OVERLAPPED = std::mem::zeroed();
            let mut got = 0u32;
            assert_eq!(
                accept_ex(srv, acc_slot, acc_buf.as_mut_ptr(), 0, 32, 32, &mut got, &mut ov_acc),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            assert_eq!(CancelIoEx(srv as HANDLE, &mut ov_acc), TRUE);

            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), FALSE);
            assert_eq!(ov_out, &mut ov_acc as *mut OVERLAPPED);
            assert_eq!(key, 0xC001);
            assert_eq!(crate::teb::last_error(), WSA_OPERATION_ABORTED);

            let mut xfer = 0u32;
            let mut flags = 0u32;
            assert_eq!(WSAGetOverlappedResult(srv, &mut ov_acc, &mut xfer, FALSE, &mut flags), FALSE);
            assert_eq!(WSAGetLastError(), WSA_OPERATION_ABORTED as i32);

            // Second CancelIoEx on already-cancelled op must return FALSE + ERROR_NOT_FOUND.
            assert_eq!(CancelIoEx(srv as HANDLE, &mut ov_acc), FALSE);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_NOT_FOUND);

            // 2. Connect a pair and cancel pending WSARecv with CancelIo(acc).
            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);
            assert_eq!(CreateIoCompletionPort(acc as HANDLE, iocp, 0xC002, 0), iocp);

            let mut rbuf_mem = [0u8; 32];
            let wb = WsaBuf { len: rbuf_mem.len() as u32, buf: rbuf_mem.as_mut_ptr() };
            let mut rflags = 0u32;
            let mut rbytes = 0u32;
            let mut ov_recv: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WSARecv(
                    acc,
                    &wb,
                    1,
                    &mut rbytes,
                    &mut rflags,
                    &mut ov_recv as *mut OVERLAPPED as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            assert_eq!(CancelIo(acc as HANDLE), TRUE);

            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), FALSE);
            assert_eq!(ov_out, &mut ov_recv as *mut OVERLAPPED);
            assert_eq!(key, 0xC002);
            assert_eq!(crate::teb::last_error(), WSA_OPERATION_ABORTED);

            assert_eq!(WSAGetOverlappedResult(acc, &mut ov_recv, &mut xfer, FALSE, &mut flags), FALSE);
            assert_eq!(WSAGetLastError(), WSA_OPERATION_ABORTED as i32);

            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(acc_slot), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn socket_handles_support_set_and_get_handle_information() {
        use crate::handles::{HANDLE_FLAG_INHERIT, HANDLE_FLAG_PROTECT_FROM_CLOSE};
        use crate::kernel32::threads::{GetHandleInformation, SetHandleInformation};
        unsafe {
            let mut wsadata = [0u8; 408];
            assert_eq!(WSAStartup(0x0202, wsadata.as_mut_ptr()), 0);

            // WSASocketW with WSA_FLAG_NO_HANDLE_INHERIT clears HANDLE_FLAG_INHERIT initially.
            let s_noinh = WSASocketW(
                AF_INET as i32,
                libc::SOCK_STREAM,
                libc::IPPROTO_TCP,
                std::ptr::null(),
                0,
                WSA_FLAG_NO_HANDLE_INHERIT,
            );
            assert_ne!(s_noinh, INVALID_SOCKET);
            let mut flags = u32::MAX;
            assert_eq!(GetHandleInformation(s_noinh as HANDLE, &mut flags), TRUE);
            assert_eq!(flags & HANDLE_FLAG_INHERIT, 0);

            // Standard socket() starts inheritable and accepts SetHandleInformation toggling.
            let s = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(s, INVALID_SOCKET);
            assert_eq!(GetHandleInformation(s as HANDLE, &mut flags), TRUE);
            assert_eq!(flags, HANDLE_FLAG_INHERIT);

            assert_eq!(SetHandleInformation(s as HANDLE, HANDLE_FLAG_INHERIT, 0), TRUE);
            assert_eq!(GetHandleInformation(s as HANDLE, &mut flags), TRUE);
            assert_eq!(flags, 0);

            assert_eq!(
                SetHandleInformation(s as HANDLE, HANDLE_FLAG_PROTECT_FROM_CLOSE, HANDLE_FLAG_PROTECT_FROM_CLOSE),
                TRUE
            );
            assert_eq!(GetHandleInformation(s as HANDLE, &mut flags), TRUE);
            assert_eq!(flags, HANDLE_FLAG_PROTECT_FROM_CLOSE);

            assert_eq!(closesocket(s_noinh), 0);
            assert_eq!(closesocket(s), 0);

            // Closed socket is no longer valid for GetHandleInformation / SetHandleInformation.
            assert_eq!(GetHandleInformation(s as HANDLE, &mut flags), FALSE);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_INVALID_HANDLE);
            assert_eq!(SetHandleInformation(s as HANDLE, HANDLE_FLAG_INHERIT, 0), FALSE);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_INVALID_HANDLE);
        }
    }

    #[test]
    fn shutdown_on_peer_closed_tcp_socket_matches_windows_semantics() {
        unsafe {
            let mut wsadata = [0u8; 408];
            assert_eq!(WSAStartup(0x0202, wsadata.as_mut_ptr()), 0);

            // Never-connected socket must still fail with WSAENOTCONN.
            let fresh = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(fresh, INVALID_SOCKET);
            assert_eq!(shutdown(fresh, 2), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAENOTCONN as i32);
            assert_eq!(closesocket(fresh), 0);

            // Connected TCP loopback socket: peer closes first, then local shutdown(SD_BOTH) succeeds.
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(srv, INVALID_SOCKET);
            let mut sa = [0u8; 16];
            sa[0..2].copy_from_slice(&AF_INET.to_le_bytes());
            sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            assert_eq!(listen(srv, 4), 0);
            let mut bound_sa = [0u8; 16];
            let mut bound_len = 16i32;
            assert_eq!(getsockname(srv, bound_sa.as_mut_ptr(), &mut bound_len), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli, INVALID_SOCKET);
            assert_eq!(connect(cli, bound_sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);

            // Peer closes first; drain EOF so kernel knows peer is gone.
            assert_eq!(closesocket(cli), 0);
            let mut buf = [0u8; 8];
            assert_eq!(recv(acc, buf.as_mut_ptr(), buf.len() as i32, 0), 0);
            // Windows shutdown(SD_BOTH) succeeds on a previously connected socket after peer FIN.
            assert_eq!(shutdown(acc, 2), 0);

            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(srv), 0);
        }
    }

    #[test]
    fn unaligned_overlapped_completion_routine_ignores_event() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        static APC_CALLS: AtomicU32 = AtomicU32::new(0);
        static APC_ERR: AtomicU32 = AtomicU32::new(u32::MAX);
        static APC_BYTES: AtomicU32 = AtomicU32::new(0);
        static APC_OV: AtomicUsize = AtomicUsize::new(0);

        unsafe extern "win64" fn wsa_completion_cb(err_code: u32, bytes: u32, ov: usize) {
            APC_ERR.store(err_code, Ordering::Release);
            APC_BYTES.store(bytes, Ordering::Release);
            APC_OV.store(ov, Ordering::Release);
            APC_CALLS.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            let mut wsadata = [0u8; 408];
            assert_eq!(WSAStartup(0x0202, wsadata.as_mut_ptr()), 0);

            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(srv, INVALID_SOCKET);
            let mut sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            assert_eq!(listen(srv, 4), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli, INVALID_SOCKET);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);

            // Odd-aligned OVERLAPPED buffer (1-byte offset inside 64-byte storage).
            let mut raw_ov = [0u8; 64];
            let unaligned_ov = raw_ov.as_mut_ptr().add(1) as *mut OVERLAPPED;
            assert_ne!((unaligned_ov as usize) % std::mem::align_of::<OVERLAPPED>(), 0);

            // Populate h_event with a real non-signaled manual-reset event: Windows ignores
            // hEvent when a completion routine is provided, so it must never be signaled.
            let ignored_ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ignored_ev.is_null());
            let mut init_ov: OVERLAPPED = std::mem::zeroed();
            init_ov.h_event = ignored_ev;
            std::ptr::write_unaligned(unaligned_ov, init_ov);

            APC_CALLS.store(0, Ordering::Release);
            APC_ERR.store(u32::MAX, Ordering::Release);
            APC_BYTES.store(0, Ordering::Release);
            APC_OV.store(0, Ordering::Release);

            // 1. Pending WSARecv on odd-aligned OVERLAPPED with completion routine.
            let mut rbuf_mem = [0u8; 32];
            let rwb = WsaBuf { len: rbuf_mem.len() as u32, buf: rbuf_mem.as_mut_ptr() };
            let mut rbytes = 0u32;
            let mut rflags = 0u32;
            assert_eq!(
                WSARecv(
                    acc,
                    &rwb,
                    1,
                    &mut rbytes,
                    &mut rflags,
                    unaligned_ov as *mut c_void,
                    wsa_completion_cb as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            // WSAGetOverlappedResult(f_wait=FALSE) on odd-aligned pending OVERLAPPED reports WSA_IO_INCOMPLETE.
            let mut xfer = 0u32;
            let mut out_flags = 0u32;
            assert_eq!(WSAGetOverlappedResult(acc, unaligned_ov, &mut xfer, FALSE, &mut out_flags), FALSE);
            assert_eq!(WSAGetLastError(), WSA_IO_INCOMPLETE as i32);

            // Send payload from peer to complete the pending WSARecv.
            let payload = b"unaligned-ov-ok!";
            assert_eq!(send(cli, payload.as_ptr(), payload.len() as i32, 0), payload.len() as i32);

            // Deliver completion routine via alertable wait.
            assert_eq!(SleepEx(3000, TRUE), crate::types::WAIT_IO_COMPLETION);
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(APC_BYTES.load(Ordering::Acquire) as usize, payload.len());
            assert_eq!(APC_OV.load(Ordering::Acquire), unaligned_ov as usize);
            assert_eq!(&rbuf_mem[..payload.len()], payload);

            // Ignored hEvent must remain unsignaled!
            assert_eq!(WaitForSingleObject(ignored_ev, 0), crate::types::WAIT_TIMEOUT);

            // WSAGetOverlappedResult must read the odd-aligned OVERLAPPED safely.
            assert_eq!(WSAGetOverlappedResult(acc, unaligned_ov, &mut xfer, FALSE, &mut out_flags), TRUE);
            assert_eq!(xfer as usize, payload.len());

            // 2. Immediate WSASend on odd-aligned OVERLAPPED with completion routine also preserves ignored hEvent.
            APC_CALLS.store(0, Ordering::Release);
            let reply = b"reply-unaligned";
            let swb = WsaBuf { len: reply.len() as u32, buf: reply.as_ptr() as *mut u8 };
            let mut sbytes = 0u32;
            assert_eq!(
                WSASend(
                    acc,
                    &swb,
                    1,
                    &mut sbytes,
                    0,
                    unaligned_ov as *mut c_void,
                    wsa_completion_cb as *mut c_void,
                ),
                0
            );
            assert_eq!(SleepEx(3000, TRUE), crate::types::WAIT_IO_COMPLETION);
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(APC_BYTES.load(Ordering::Acquire) as usize, reply.len());
            assert_eq!(APC_OV.load(Ordering::Acquire), unaligned_ov as usize);
            assert_eq!(WaitForSingleObject(ignored_ev, 0), crate::types::WAIT_TIMEOUT);

            assert_eq!(CloseHandle(ignored_ev), TRUE);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
        }
    }

    #[test]
    fn overlapped_wsasend_scatter_gather_backpressure_and_datagram_semantics() {
        use crate::kernel32::file::{CreateIoCompletionPort, GetQueuedCompletionStatus};
        use crate::kernel32::threads::CloseHandle;
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        static SEND_APC_CALLS: AtomicU32 = AtomicU32::new(0);
        static SEND_APC_ERR: AtomicU32 = AtomicU32::new(u32::MAX);
        static SEND_APC_BYTES: AtomicU32 = AtomicU32::new(0);
        static SEND_APC_OV: AtomicUsize = AtomicUsize::new(0);

        unsafe extern "win64" fn send_apc_cb(err_code: u32, bytes: u32, ov: usize) {
            SEND_APC_ERR.store(err_code, Ordering::Release);
            SEND_APC_BYTES.store(bytes, Ordering::Release);
            SEND_APC_OV.store(ov, Ordering::Release);
            SEND_APC_CALLS.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            let mut wsadata = [0u8; 408];
            assert_eq!(WSAStartup(0x0202, wsadata.as_mut_ptr()), 0);

            // (a) TCP stream backpressure with multi-WSABUF scatter-gather (including
            // small leading chunks, empty chunk, and large chunks that split mid-WSABUF).
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());

            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(srv, INVALID_SOCKET);
            let mut sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            assert_eq!(listen(srv, 4), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli, INVALID_SOCKET);
            // Shrink kernel buffers so 512 KiB reliably triggers short initial sends + reactor POLLOUT loops.
            let small_buf: i32 = 16 * 1024;
            let _ = setsockopt(cli, 0xFFFF, 0x1001 /* SO_SNDBUF */, &small_buf as *const i32 as *const u8, 4);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);
            let _ = setsockopt(acc, 0xFFFF, 0x1002 /* SO_RCVBUF */, &small_buf as *const i32 as *const u8, 4);
            assert_eq!(CreateIoCompletionPort(cli as HANDLE, iocp, 0x5EED, 0), iocp);

            let sizes = [13usize, 3000, 0, 190_000, 331_275];
            let total_len: usize = sizes.iter().sum();
            let mut expected = Vec::with_capacity(total_len);
            let mut chunks: Vec<Vec<u8>> = Vec::with_capacity(sizes.len());
            let mut seq = 0usize;
            for &sz in &sizes {
                let mut v = Vec::with_capacity(sz);
                for _ in 0..sz {
                    let b = ((seq.wrapping_mul(37) ^ (seq >> 8) ^ 0xA5) & 0xFF) as u8;
                    v.push(b);
                    expected.push(b);
                    seq += 1;
                }
                chunks.push(v);
            }
            let wsabufs: Vec<WsaBuf> = chunks
                .iter_mut()
                .map(|v| WsaBuf {
                    len: v.len() as u32,
                    buf: if v.is_empty() { std::ptr::null_mut() } else { v.as_mut_ptr() },
                })
                .collect();

            let mut ov1: OVERLAPPED = std::mem::zeroed();
            let mut immediate_sent = 0u32;
            let rc1 = WSASend(
                cli,
                wsabufs.as_ptr(),
                wsabufs.len() as u32,
                &mut immediate_sent,
                0,
                &mut ov1 as *mut OVERLAPPED as *mut c_void,
                std::ptr::null_mut(),
            );
            assert_eq!(rc1, SOCKET_ERROR, "512 KiB scatter-gather TCP WSASend must pend under backpressure");
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let reader = std::thread::spawn(move || {
                let mut got = Vec::with_capacity(total_len);
                let mut tmp = [0u8; 4096];
                while got.len() < total_len {
                    let n = recv(acc, tmp.as_mut_ptr(), tmp.len() as i32, 0);
                    assert!(n > 0, "unexpected recv={n} err={}", WSAGetLastError());
                    got.extend_from_slice(&tmp[..n as usize]);
                }
                got
            });

            let mut comp_bytes = 0u32;
            let mut comp_key = 0usize;
            let mut comp_ov: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut comp_bytes, &mut comp_key, &mut comp_ov, 5000), TRUE);
            assert_eq!(comp_key, 0x5EED);
            assert_eq!(comp_ov, &mut ov1 as *mut OVERLAPPED);
            assert_eq!(comp_bytes as usize, total_len);
            let mut xfer = 0u32;
            let mut fl = 0u32;
            assert_eq!(WSAGetOverlappedResult(cli, &mut ov1, &mut xfer, FALSE, &mut fl), TRUE);
            assert_eq!(xfer as usize, total_len);
            let received = reader.join().unwrap();
            assert_eq!(received, expected, "stream WSASend must deliver every byte across all WSABUF chunks");

            // (b) Second independent multi-WSABUF overlapped WSASend (with APC completion routine)
            // confirms fresh offset tracking without cross-contamination.
            let mut chunks2: Vec<Vec<u8>> = vec![
                (0..4097usize).map(|i| ((i * 13 + 7) & 0xFF) as u8).collect(),
                (0..260_000usize).map(|i| ((i * 29 + 3) & 0xFF) as u8).collect(),
                (0..12345usize).map(|i| ((i * 53 + 11) & 0xFF) as u8).collect(),
            ];
            let expected2: Vec<u8> = chunks2.iter().flatten().copied().collect();
            let total2 = expected2.len();
            let wsabufs2: Vec<WsaBuf> = chunks2
                .iter_mut()
                .map(|v| WsaBuf { len: v.len() as u32, buf: v.as_mut_ptr() })
                .collect();

            SEND_APC_CALLS.store(0, Ordering::Release);
            SEND_APC_ERR.store(u32::MAX, Ordering::Release);
            SEND_APC_BYTES.store(0, Ordering::Release);
            SEND_APC_OV.store(0, Ordering::Release);

            let mut ov2: OVERLAPPED = std::mem::zeroed();
            let rc2 = WSASend(
                cli,
                wsabufs2.as_ptr(),
                wsabufs2.len() as u32,
                &mut immediate_sent,
                0,
                &mut ov2 as *mut OVERLAPPED as *mut c_void,
                send_apc_cb as *mut c_void,
            );
            assert!(rc2 == 0 || WSAGetLastError() == WSA_IO_PENDING as i32);

            let reader2 = std::thread::spawn(move || {
                let mut got = Vec::with_capacity(total2);
                let mut tmp = [0u8; 4096];
                while got.len() < total2 {
                    let n = recv(acc, tmp.as_mut_ptr(), tmp.len() as i32, 0);
                    assert!(n > 0);
                    got.extend_from_slice(&tmp[..n as usize]);
                }
                got
            });

            assert_eq!(SleepEx(5000, TRUE), crate::types::WAIT_IO_COMPLETION);
            assert_eq!(SEND_APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(SEND_APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(SEND_APC_BYTES.load(Ordering::Acquire) as usize, total2);
            assert_eq!(SEND_APC_OV.load(Ordering::Acquire), &mut ov2 as *mut OVERLAPPED as usize);
            assert_eq!(reader2.join().unwrap(), expected2);

            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);

            // (c) UDP SOCK_DGRAM overlapped WSASendTo preserves message-level datagram semantics.
            let u_rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let u_tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(u_rx, INVALID_SOCKET);
            assert_ne!(u_tx, INVALID_SOCKET);
            let mut u_sa = loopback_sa(0);
            assert_eq!(bind(u_rx, u_sa.as_ptr(), 16), 0);
            let mut u_slen = 16i32;
            assert_eq!(getsockname(u_rx, u_sa.as_mut_ptr(), &mut u_slen), 0);

            let part_a = b"dgram-part-A:";
            let part_b = b"dgram-part-B!";
            let dgram_bufs = [
                WsaBuf { len: part_a.len() as u32, buf: part_a.as_ptr() as *mut u8 },
                WsaBuf { len: part_b.len() as u32, buf: part_b.as_ptr() as *mut u8 },
            ];
            let mut ov_u: OVERLAPPED = std::mem::zeroed();
            let mut u_sent = 0u32;
            let r_u = WSASendTo(
                u_tx,
                dgram_bufs.as_ptr(),
                2,
                &mut u_sent,
                0,
                u_sa.as_ptr(),
                16,
                &mut ov_u as *mut OVERLAPPED as *mut c_void,
                std::ptr::null_mut(),
            );
            assert_eq!(r_u, 0);
            assert_eq!(u_sent as usize, part_a.len() + part_b.len());
            let mut rx_buf = [0u8; 64];
            let rx_n = recv(u_rx, rx_buf.as_mut_ptr(), rx_buf.len() as i32, 0);
            assert_eq!(rx_n as usize, part_a.len() + part_b.len());
            assert_eq!(&rx_buf[..rx_n as usize], b"dgram-part-A:dgram-part-B!");

            assert_eq!(closesocket(u_rx), 0);
            assert_eq!(closesocket(u_tx), 0);
        }
    }

    #[test]
    fn concurrent_overlapped_wsasend_preserves_guest_mode_and_cancels_cleanly() {
        use crate::kernel32::file::{CancelIoEx, CreateIoCompletionPort, GetQueuedCompletionStatus};
        use crate::kernel32::threads::CloseHandle;

        const WIN_FIONBIO: i32 = 0x8004_667Eu32 as i32;

        unsafe {
            let mut wsadata = [0u8; 408];
            assert_eq!(WSAStartup(0x0202, wsadata.as_mut_ptr()), 0);

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());

            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            assert_eq!(listen(srv, 4), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let small_buf: i32 = 16 * 1024;
            let _ = setsockopt(cli, 0xFFFF, 0x1001, &small_buf as *const i32 as *const u8, 4);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            let _ = setsockopt(acc, 0xFFFF, 0x1002, &small_buf as *const i32 as *const u8, 4);
            // Set short SO_RCVTIMEO on both sockets so any blocking recv test is strictly bounded.
            let rcv_timeout_ms: u32 = 40;
            assert_eq!(
                setsockopt(cli, 0xFFFF, 0x1006 /* SO_RCVTIMEO */, &rcv_timeout_ms as *const u32 as *const u8, 4),
                0
            );
            assert_eq!(CreateIoCompletionPort(cli as HANDLE, iocp, 0xCC01, 0), iocp);

            // Underlying POSIX fd stays permanently O_NONBLOCK while guest mode starts blocking.
            let cli_fd = lookup_fd(cli).unwrap();
            assert_ne!(libc::fcntl(cli_fd, libc::F_GETFL) & libc::O_NONBLOCK, 0);
            assert!(!is_guest_nonblocking(cli));

            // Issue two competing large overlapped WSASend operations on the same socket.
            // Neither may block even though guest mode is blocking and no reader is draining yet.
            let mut buf_a = vec![0x41u8; 384 * 1024];
            let mut buf_b = vec![0x42u8; 256 * 1024];
            let wb_a = WsaBuf { len: buf_a.len() as u32, buf: buf_a.as_mut_ptr() };
            let wb_b = WsaBuf { len: buf_b.len() as u32, buf: buf_b.as_mut_ptr() };
            let mut ov_a: OVERLAPPED = std::mem::zeroed();
            let mut ov_b: OVERLAPPED = std::mem::zeroed();
            let mut sent_a = 0u32;
            let mut sent_b = 0u32;

            assert_eq!(
                WSASend(cli, &wb_a, 1, &mut sent_a, 0, &mut ov_a as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            assert_eq!(
                WSASend(cli, &wb_b, 1, &mut sent_b, 0, &mut ov_b as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            // While both overlapped sends are pending, verify guest FIONBIO toggle works accurately
            // and never clears underlying O_NONBLOCK:
            // 1. In guest blocking mode, recv on empty peer hits bounded SO_RCVTIMEO -> WSAETIMEDOUT.
            let mut tmp = [0u8; 8];
            assert_eq!(recv(cli, tmp.as_mut_ptr(), 8, 0), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAETIMEDOUT as i32);

            // 2. Toggle guest FIONBIO=1 -> recv returns WSAEWOULDBLOCK immediately.
            let mut nb: u32 = 1;
            assert_eq!(ioctlsocket(cli, WIN_FIONBIO, &mut nb), 0);
            assert!(is_guest_nonblocking(cli));
            assert_eq!(recv(cli, tmp.as_mut_ptr(), 8, 0), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEWOULDBLOCK as i32);

            // 3. Toggle guest FIONBIO=0 -> guest blocking mode restored, underlying fd still O_NONBLOCK.
            nb = 0;
            assert_eq!(ioctlsocket(cli, WIN_FIONBIO, &mut nb), 0);
            assert!(!is_guest_nonblocking(cli));
            assert_ne!(libc::fcntl(cli_fd, libc::F_GETFL) & libc::O_NONBLOCK, 0);

            // Cancel second pending send (ov_b) cleanly while letting first pending send (ov_a) finish.
            assert_eq!(CancelIoEx(cli as HANDLE, &mut ov_b), TRUE);

            // Bounded drain of acc so ov_a makes progress and completes.
            let drain_handle = std::thread::spawn(move || {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
                let mut total = 0usize;
                let mut rbuf = [0u8; 8192];
                let tmo: u32 = 100;
                let _ = setsockopt(acc, 0xFFFF, 0x1006, &tmo as *const u32 as *const u8, 4);
                while total < 384 * 1024 && std::time::Instant::now() < deadline {
                    let n = recv(acc, rbuf.as_mut_ptr(), rbuf.len() as i32, 0);
                    if n > 0 {
                        total += n as usize;
                    } else if n == 0 {
                        break;
                    }
                }
                total
            });

            let mut saw_a = false;
            let mut saw_b_cancelled = false;
            for _ in 0..2 {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                let ok = GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 5000);
                assert_eq!(key, 0xCC01);
                if ov_out == &mut ov_b as *mut OVERLAPPED {
                    assert_eq!(ok, FALSE);
                    assert_eq!(crate::teb::last_error(), WSA_OPERATION_ABORTED);
                    saw_b_cancelled = true;
                } else if ov_out == &mut ov_a as *mut OVERLAPPED {
                    assert_eq!(ok, TRUE);
                    assert_eq!(bytes as usize, 384 * 1024);
                    saw_a = true;
                } else {
                    panic!("unexpected overlapped {ov_out:p}");
                }
            }
            assert!(saw_a && saw_b_cancelled);
            let drained = drain_handle.join().unwrap();
            assert!(drained >= 384 * 1024, "expected at least 384 KiB drained, got {drained}");

            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }
}
