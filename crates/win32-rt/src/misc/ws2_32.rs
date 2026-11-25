//! ws2_32.dll: Winsock2 over POSIX / BSD sockets.
//!
//! Translates Windows address families (`AF_INET6 = 23` <-> Darwin `30`),
//! `sockaddr_in`/`sockaddr_in6` headers, `ADDRINFOA`/`ADDRINFOW` layouts,
//! socket options (`SOL_SOCKET`, `IPPROTO_TCP`, `IPPROTO_IPV6`, timeouts, linger),
//! `ioctlsocket`/`WSAIoctl`, `select`/`WSAPoll`, and errno -> WSAE* error codes.

use crate::kernel32::file::{
    complete_overlapped_binding, complete_overlapped_io, external_io_event, get_file,
    is_io_success_status, load_ov_internal, load_ov_internal_high, lookup_io_completion_binding,
    prepare_overlapped_pending, register_external_cancel_hook, register_external_io_handle,
    store_ov_internal, store_ov_internal_high, unregister_external_io_handle, win32_to_ntstatus,
    IoCompletionBinding, INVALID_HANDLE_VALUE, OVERLAPPED, STATUS_CANCELLED, STATUS_PENDING,
    STATUS_RECEIVE_EXPEDITED, STATUS_SUCCESS,
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
    /// Default multicast destination set by `WSAJoinLeaf` (`JL_SENDER_ONLY` / `JL_BOTH`).
    /// Retained separately instead of calling POSIX `connect(2)` so bidirectional (`JL_BOTH`)
    /// UDP sockets remain unconnected on Darwin and continue receiving multicast datagrams
    /// whose source IP is a sender's unicast address rather than the group address.
    mcast_dest: Option<(libc::sockaddr_storage, libc::socklen_t)>,
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
        0 | 0x4000_0010 => 0, // STATUS_SUCCESS | STATUS_RECEIVE_EXPEDITED
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
        0xC000_0005 => WSAEFAULT,
        0xC000_000D => WSAEINVAL,
        0xC000_0022 => WSAEACCES,
        s => crate::ntdll::ntstatus_to_win32(s),
    }
}

#[inline]
fn ntstatus_to_wsa_recv_flags(status: usize) -> u32 {
    if status == STATUS_RECEIVE_EXPEDITED {
        WIN_MSG_OOB
    } else {
        0
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

/// Wait for `events` (`POLLIN` / `POLLOUT`) on an always-`O_NONBLOCK` socket until `deadline`
/// (or indefinitely when `deadline` is `None`).
unsafe fn wait_socket_ready_until(
    fd: i32,
    events: i16,
    deadline: Option<std::time::Instant>,
) -> Result<(), i32> {
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

/// Wait for `events` (`POLLIN` / `POLLOUT`) on an always-`O_NONBLOCK` socket to emulate
/// guest blocking mode, respecting `SO_RCVTIMEO` / `SO_SNDTIMEO` when `timeout_opt` is given.
unsafe fn wait_socket_ready(fd: i32, events: i16, timeout_opt: Option<libc::c_int>) -> Result<(), i32> {
    let deadline = timeout_opt
        .and_then(|opt| socket_timeout_duration(fd, opt))
        .map(|d| std::time::Instant::now() + d);
    wait_socket_ready_until(fd, events, deadline)
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
            mcast_dest: None,
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
    match family {
        AF_INET if len >= 16 => {
            let port = u16::from_be((sa.add(2) as *const u16).read_unaligned());
            let a = Ipv4Addr::from((sa.add(4) as *const [u8; 4]).read_unaligned());
            Some(if port != 0 { format!("{a}:{port}") } else { a.to_string() })
        }
        AF_INET6 if len >= 28 => {
            let port = u16::from_be((sa.add(2) as *const u16).read_unaligned());
            let a = Ipv6Addr::from((sa.add(8) as *const [u8; 16]).read_unaligned());
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
    if out.is_null() || (out_len.read_unaligned() as usize) < w.len() {
        out_len.write_unaligned(w.len() as u32);
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, out as *mut u8, std::mem::size_of_val(w.as_slice()));
    out_len.write_unaligned(w.len() as u32);
    0
}

pub unsafe extern "win64" fn WSAStringToAddressW(s: *const u16, family: i32, _proto: *mut c_void, sa: *mut u8, len: *mut i32) -> i32 {
    if s.is_null() || sa.is_null() || len.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let text = crate::strings::from_wide(s);
    match family as u16 {
        AF_INET => {
            if len.read_unaligned() < 16 {
                len.write_unaligned(16);
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
            len.write_unaligned(16);
            0
        }
        AF_INET6 => {
            if len.read_unaligned() < 28 {
                len.write_unaligned(28);
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
            len.write_unaligned(28);
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

/// Windows x64 `HOSTENT` (32 bytes): `h_addrtype` and `h_length` are `i16`
/// (unlike POSIX `int` on LP64!), followed by 4 bytes padding before `h_addr_list`.
#[repr(C)]
struct WinHostent {
    h_name: *mut u8,
    h_aliases: *mut *mut u8,
    h_addrtype: i16,
    h_length: i16,
    _pad: u32,
    h_addr_list: *mut *mut u8,
}
const _: () = assert!(std::mem::size_of::<WinHostent>() == 32);

/// Windows x64 `PROTOENT` (24 bytes): `p_proto` is `i16`.
#[repr(C)]
struct WinProtoent {
    p_name: *mut u8,
    p_aliases: *mut *mut u8,
    p_proto: i16,
    _pad: [u8; 6],
}
const _: () = assert!(std::mem::size_of::<WinProtoent>() == 24);

struct ThreadHostentBuf {
    hostent: Box<WinHostent>,
    name: Vec<u8>,
    alias_ptrs: Vec<*mut u8>,
    addrs: Vec<[u8; 4]>,
    addr_ptrs: Vec<*mut u8>,
}

struct ThreadProtoentBuf {
    protoent: Box<WinProtoent>,
    name: Vec<u8>,
    #[allow(dead_code)]
    alias_bufs: Vec<Vec<u8>>,
    alias_ptrs: Vec<*mut u8>,
}

thread_local! {
    static TLS_HOSTENT: std::cell::RefCell<Option<ThreadHostentBuf>> = const { std::cell::RefCell::new(None) };
    static TLS_PROTOENT: std::cell::RefCell<Option<ThreadProtoentBuf>> = const { std::cell::RefCell::new(None) };
}

unsafe fn store_tls_hostent(canon_name: &str, addrs: Vec<[u8; 4]>) -> *mut c_void {
    TLS_HOSTENT.with(|cell| {
        let mut name = canon_name.as_bytes().to_vec();
        name.push(0);
        let alias_ptrs = vec![std::ptr::null_mut()];
        let mut buf = ThreadHostentBuf {
            hostent: Box::new(WinHostent {
                h_name: std::ptr::null_mut(),
                h_aliases: std::ptr::null_mut(),
                h_addrtype: AF_INET as i16,
                h_length: 4,
                _pad: 0,
                h_addr_list: std::ptr::null_mut(),
            }),
            name,
            alias_ptrs,
            addrs,
            addr_ptrs: Vec::new(),
        };
        for a in buf.addrs.iter_mut() {
            buf.addr_ptrs.push(a.as_mut_ptr());
        }
        buf.addr_ptrs.push(std::ptr::null_mut());
        buf.hostent.h_name = buf.name.as_mut_ptr();
        buf.hostent.h_aliases = buf.alias_ptrs.as_mut_ptr();
        buf.hostent.h_addr_list = buf.addr_ptrs.as_mut_ptr();
        let ptr = (&mut *buf.hostent) as *mut WinHostent as *mut c_void;
        *cell.borrow_mut() = Some(buf);
        ptr
    })
}

pub unsafe extern "win64" fn gethostbyname(name: LPCSTR) -> *mut c_void {
    let query = if name.is_null() {
        let mut h = [0u8; 256];
        if libc::gethostname(h.as_mut_ptr() as *mut libc::c_char, h.len()) == 0 {
            crate::strings::from_ansi(h.as_ptr())
        } else {
            "localhost".to_string()
        }
    } else {
        crate::strings::from_ansi(name)
    };
    if let Ok(ip) = query.parse::<Ipv4Addr>() {
        return store_tls_hostent(&query, vec![ip.octets()]);
    }
    let Ok(c_host) = CString::new(query.as_str()) else {
        return err(WSAHOST_NOT_FOUND, std::ptr::null_mut());
    };
    let mut hints: libc::addrinfo = std::mem::zeroed();
    hints.ai_family = libc::AF_INET;
    hints.ai_socktype = libc::SOCK_STREAM;
    hints.ai_flags = libc::AI_CANONNAME;
    let mut res: *mut libc::addrinfo = std::ptr::null_mut();
    let rc = libc::getaddrinfo(c_host.as_ptr(), std::ptr::null(), &hints, &mut res);
    if rc != 0 || res.is_null() {
        return err(WSAHOST_NOT_FOUND, std::ptr::null_mut());
    }
    let mut canon = query.clone();
    let mut addrs: Vec<[u8; 4]> = Vec::new();
    let mut cur = res;
    while !cur.is_null() {
        let ai = &*cur;
        if cur == res && !ai.ai_canonname.is_null() {
            canon = crate::strings::from_ansi(ai.ai_canonname as *const u8);
        }
        if ai.ai_family == libc::AF_INET && !ai.ai_addr.is_null() {
            let sin = &*(ai.ai_addr as *const libc::sockaddr_in);
            let octets = sin.sin_addr.s_addr.to_ne_bytes();
            if !addrs.contains(&octets) {
                addrs.push(octets);
            }
        }
        cur = ai.ai_next;
    }
    libc::freeaddrinfo(res);
    if addrs.is_empty() {
        return err(WSANO_DATA, std::ptr::null_mut());
    }
    store_tls_hostent(&canon, addrs)
}

pub unsafe extern "win64" fn gethostbyaddr(addr: *const u8, len: i32, ty: i32) -> *mut c_void {
    if addr.is_null() {
        return err(WSAEFAULT, std::ptr::null_mut());
    }
    if ty != AF_INET as i32 || len != 4 {
        return err(WSAEAFNOSUPPORT, std::ptr::null_mut());
    }
    let octets = [
        addr.read_unaligned(),
        addr.add(1).read_unaligned(),
        addr.add(2).read_unaligned(),
        addr.add(3).read_unaligned(),
    ];
    let mut sin: libc::sockaddr_in = std::mem::zeroed();
    sin.sin_len = std::mem::size_of::<libc::sockaddr_in>() as u8;
    sin.sin_family = libc::AF_INET as libc::sa_family_t;
    sin.sin_addr.s_addr = u32::from_ne_bytes(octets);
    let mut hbuf = [0u8; 256];
    let rc = libc::getnameinfo(
        &sin as *const libc::sockaddr_in as *const libc::sockaddr,
        std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        hbuf.as_mut_ptr() as *mut libc::c_char,
        hbuf.len() as libc::socklen_t,
        std::ptr::null_mut(),
        0,
        0,
    );
    let host_name = if rc == 0 {
        crate::strings::from_ansi(hbuf.as_ptr())
    } else {
        Ipv4Addr::from(octets).to_string()
    };
    store_tls_hostent(&host_name, vec![octets])
}

pub unsafe extern "win64" fn getprotobyname(name: LPCSTR) -> *mut c_void {
    if name.is_null() {
        return err(WSAEFAULT, std::ptr::null_mut());
    }
    let query = crate::strings::from_ansi(name);
    let Ok(c_name) = CString::new(query.as_str()) else {
        return err(WSANO_DATA, std::ptr::null_mut());
    };
    let pe = libc::getprotobyname(c_name.as_ptr());
    let (pname, aliases, proto_num): (String, Vec<String>, i16) = if !pe.is_null() {
        let p = &*pe;
        let pn = if p.p_name.is_null() {
            query.clone()
        } else {
            crate::strings::from_ansi(p.p_name as *const u8)
        };
        let mut als = Vec::new();
        if !p.p_aliases.is_null() {
            let mut idx = 0usize;
            loop {
                let ap = p.p_aliases.add(idx).read_unaligned();
                if ap.is_null() {
                    break;
                }
                als.push(crate::strings::from_ansi(ap as *const u8));
                idx += 1;
                if idx >= 32 {
                    break;
                }
            }
        }
        (pn, als, p.p_proto as i16)
    } else {
        match query.to_ascii_lowercase().as_str() {
            "ip" => ("ip".into(), vec!["IP".into()], 0),
            "icmp" => ("icmp".into(), vec!["ICMP".into()], 1),
            "tcp" => ("tcp".into(), vec!["TCP".into()], 6),
            "udp" => ("udp".into(), vec!["UDP".into()], 17),
            "ipv6" => ("ipv6".into(), vec!["IPV6".into()], 41),
            "icmpv6" => ("icmpv6".into(), vec!["ICMPV6".into()], 58),
            _ => return err(WSANO_DATA, std::ptr::null_mut()),
        }
    };
    TLS_PROTOENT.with(|cell| {
        let mut name_bytes = pname.into_bytes();
        name_bytes.push(0);
        let mut alias_bufs: Vec<Vec<u8>> = aliases
            .into_iter()
            .map(|a| {
                let mut v = a.into_bytes();
                v.push(0);
                v
            })
            .collect();
        let mut alias_ptrs: Vec<*mut u8> = alias_bufs.iter_mut().map(|v| v.as_mut_ptr()).collect();
        alias_ptrs.push(std::ptr::null_mut());
        let mut buf = ThreadProtoentBuf {
            protoent: Box::new(WinProtoent {
                p_name: std::ptr::null_mut(),
                p_aliases: std::ptr::null_mut(),
                p_proto: proto_num,
                _pad: [0; 6],
            }),
            name: name_bytes,
            alias_bufs,
            alias_ptrs,
        };
        buf.protoent.p_name = buf.name.as_mut_ptr();
        buf.protoent.p_aliases = buf.alias_ptrs.as_mut_ptr();
        let ptr = (&mut *buf.protoent) as *mut WinProtoent as *mut c_void;
        *cell.borrow_mut() = Some(buf);
        ptr
    })
}

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
        // Native-unsupported: macOS does not permit injecting a socket file descriptor into
        // an arbitrary external process we do not host. Matches Microsoft Learn
        // `WSADuplicateSocketW` behavior when the underlying transport/provider cannot share
        // the socket with the target process (`WSAEOPNOTSUPP`).
        log::debug!("WSADuplicateSocket: cross-process target pid={pid} unsupported -> WSAEOPNOTSUPP");
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
    clear_socket_ov_result_flags(s);
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
        m.mcast_dest = None;
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
        (WIN_SOL_SOCKET, 0x0100) => Some((libc::SOL_SOCKET, libc::SO_OOBINLINE)),
        (WIN_SOL_SOCKET, 0x1001) => Some((libc::SOL_SOCKET, libc::SO_SNDBUF)),
        (WIN_SOL_SOCKET, 0x1002) => Some((libc::SOL_SOCKET, libc::SO_RCVBUF)),
        (WIN_SOL_SOCKET, 0x1005) => Some((libc::SOL_SOCKET, libc::SO_SNDTIMEO)),
        (WIN_SOL_SOCKET, 0x1006) => Some((libc::SOL_SOCKET, libc::SO_RCVTIMEO)),
        (WIN_SOL_SOCKET, 0x1007) => Some((libc::SOL_SOCKET, libc::SO_ERROR)),
        (WIN_SOL_SOCKET, 0x1008) => Some((libc::SOL_SOCKET, libc::SO_TYPE)),
        (WIN_IPPROTO_TCP, 1) => Some((libc::IPPROTO_TCP, libc::TCP_NODELAY)),
        (WIN_IPPROTO_IPV6, 19) => Some((libc::IPPROTO_IPV6, libc::IPV6_RECVPKTINFO)),
        (WIN_IPPROTO_IPV6, 21) => Some((libc::IPPROTO_IPV6, libc::IPV6_RECVHOPLIMIT)),
        (WIN_IPPROTO_IPV6, 27) => Some((libc::IPPROTO_IPV6, libc::IPV6_V6ONLY)),
        (WIN_IPPROTO_IPV6, 39) => Some((libc::IPPROTO_IPV6, libc::IPV6_RECVTCLASS)),
        (WIN_IPPROTO_IP, 3) => Some((libc::IPPROTO_IP, libc::IP_TOS)),
        (WIN_IPPROTO_IP, 4) => Some((libc::IPPROTO_IP, libc::IP_TTL)),
        (WIN_IPPROTO_IP, 9) => Some((libc::IPPROTO_IP, libc::IP_MULTICAST_IF)),
        (WIN_IPPROTO_IP, 10) => Some((libc::IPPROTO_IP, libc::IP_MULTICAST_TTL)),
        (WIN_IPPROTO_IP, 11) => Some((libc::IPPROTO_IP, libc::IP_MULTICAST_LOOP)),
        (WIN_IPPROTO_IP, 12) => Some((libc::IPPROTO_IP, libc::IP_ADD_MEMBERSHIP)),
        (WIN_IPPROTO_IP, 13) => Some((libc::IPPROTO_IP, libc::IP_DROP_MEMBERSHIP)),
        (WIN_IPPROTO_IP, 19) => Some((libc::IPPROTO_IP, libc::IP_PKTINFO)),
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
    if plevel == libc::IPPROTO_IP
        && (popt == libc::IP_MULTICAST_TTL || popt == libc::IP_MULTICAST_LOOP)
        && optlen >= 4
    {
        let v = std::ptr::read_unaligned(optval as *const u32).min(u8::MAX as u32) as u8;
        if libc::setsockopt(fd, plevel, popt, &v as *const u8 as *const c_void, 1) != 0 {
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
    const WIN_SIOCATMARK: u32 = 0x4004_7307;
    match cmd as u32 {
        WIN_FIONBIO => {
            if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
                m.guest_nonblocking = argp.read_unaligned() != 0;
            }
            return 0;
        }
        WIN_FIONREAD => {
            let mut val = 0i32;
            if libc::ioctl(fd, libc::FIONREAD, &mut val) != 0 {
                return err(last_errno_wsa(), SOCKET_ERROR);
            }
            std::ptr::write_unaligned(argp, val as u32);
            0
        }
        WIN_SIOCATMARK => {
            let mut val = 0i32;
            if libc::ioctl(fd, libc::SIOCATMARK, &mut val) != 0 {
                return err(last_errno_wsa(), SOCKET_ERROR);
            }
            std::ptr::write_unaligned(argp, u32::from(val != 0));
            0
        }
        _ => err(WSAEINVAL, SOCKET_ERROR),
    }
}

/// Windows `tcp_keepalive` (12 bytes) for `WSAIoctl(SIO_KEEPALIVE_VALS)`.
#[repr(C)]
#[derive(Clone, Copy)]
struct WinTcpKeepalive {
    onoff: u32,
    keepalivetime: u32,
    keepaliveinterval: u32,
}
const _: () = assert!(std::mem::size_of::<WinTcpKeepalive>() == 12);

/// Windows `INTERFACE_INFO` (76 bytes) for `WSAIoctl(SIO_GET_INTERFACE_LIST)`.
/// `iiFlags` (+0, 4B), `iiAddress` (+4, 24B), `iiBroadcastAddress` (+28, 24B), `iiNetmask` (+52, 24B).
#[repr(C)]
#[derive(Clone, Copy)]
struct WinInterfaceInfo {
    flags: u32,
    address: [u8; 24],
    broadcast: [u8; 24],
    netmask: [u8; 24],
}
const _: () = assert!(std::mem::size_of::<WinInterfaceInfo>() == 76);

unsafe fn write_sockaddr_gen(dst: &mut [u8; 24], sa: *const libc::sockaddr) {
    dst.fill(0);
    if sa.is_null() {
        return;
    }
    let mut len = 16i32;
    let _ = posix_to_win_sockaddr(
        sa,
        std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        dst.as_mut_ptr(),
        &mut len,
    );
}

unsafe fn wsa_ioctl_get_interface_list(
    out_buf: *mut c_void,
    out_len: u32,
    bytes_returned: *mut u32,
) -> i32 {
    let mut ifa_head: *mut libc::ifaddrs = std::ptr::null_mut();
    if libc::getifaddrs(&mut ifa_head) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    let mut entries: Vec<WinInterfaceInfo> = Vec::new();
    let mut cur = ifa_head;
    while !cur.is_null() {
        let ifa = &*cur;
        if !ifa.ifa_addr.is_null() && (*ifa.ifa_addr).sa_family as i32 == libc::AF_INET {
            let mut win_flags = 0u32;
            if (ifa.ifa_flags as i32 & libc::IFF_UP) != 0 {
                win_flags |= 0x0000_0001; // IFF_UP
            }
            if (ifa.ifa_flags as i32 & libc::IFF_BROADCAST) != 0 {
                win_flags |= 0x0000_0002; // IFF_BROADCAST
            }
            if (ifa.ifa_flags as i32 & libc::IFF_LOOPBACK) != 0 {
                win_flags |= 0x0000_0004; // IFF_LOOPBACK
            }
            if (ifa.ifa_flags as i32 & libc::IFF_POINTOPOINT) != 0 {
                win_flags |= 0x0000_0008; // IFF_POINTTOPOINT
            }
            if (ifa.ifa_flags as i32 & libc::IFF_MULTICAST) != 0 {
                win_flags |= 0x0000_0010; // IFF_MULTICAST
            }
            let mut info = WinInterfaceInfo {
                flags: win_flags,
                address: [0; 24],
                broadcast: [0; 24],
                netmask: [0; 24],
            };
            write_sockaddr_gen(&mut info.address, ifa.ifa_addr);
            if (ifa.ifa_flags as i32 & (libc::IFF_BROADCAST | libc::IFF_POINTOPOINT)) != 0 {
                write_sockaddr_gen(&mut info.broadcast, ifa.ifa_dstaddr);
            }
            write_sockaddr_gen(&mut info.netmask, ifa.ifa_netmask);
            entries.push(info);
        }
        cur = ifa.ifa_next;
    }
    libc::freeifaddrs(ifa_head);

    let needed = (entries.len() * std::mem::size_of::<WinInterfaceInfo>()) as u32;
    if !bytes_returned.is_null() {
        std::ptr::write_unaligned(bytes_returned, needed);
    }
    if out_buf.is_null() || out_len < needed {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    for (i, entry) in entries.iter().enumerate() {
        std::ptr::write_unaligned((out_buf as *mut WinInterfaceInfo).add(i), *entry);
    }
    0
}

unsafe fn wsa_ioctl_address_list_query(
    family: i32,
    out_buf: *mut c_void,
    out_len: u32,
    bytes_returned: *mut u32,
) -> i32 {
    let mut ifa_head: *mut libc::ifaddrs = std::ptr::null_mut();
    if libc::getifaddrs(&mut ifa_head) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    let mut addrs: Vec<Vec<u8>> = Vec::new();
    let mut cur = ifa_head;
    while !cur.is_null() {
        let ifa = &*cur;
        if !ifa.ifa_addr.is_null() {
            let sa_fam = (*ifa.ifa_addr).sa_family as i32;
            if sa_fam == family && (sa_fam == libc::AF_INET || sa_fam == libc::AF_INET6) {
                let win_len = if sa_fam == libc::AF_INET { 16usize } else { 28usize };
                let posix_len = if sa_fam == libc::AF_INET {
                    std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t
                } else {
                    std::mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t
                };
                let mut buf = vec![0u8; win_len];
                let mut out_l = win_len as i32;
                if posix_to_win_sockaddr(ifa.ifa_addr, posix_len, buf.as_mut_ptr(), &mut out_l).is_ok() {
                    buf.truncate(out_l as usize);
                    addrs.push(buf);
                }
            }
        }
        cur = ifa.ifa_next;
    }
    libc::freeifaddrs(ifa_head);

    // Win64 SOCKET_ADDRESS_LIST:
    // +0: iAddressCount (i32) + 4B pad
    // +8: SOCKET_ADDRESS[count] (each 16B: lpSockaddr: *mut u8 @+0, iSockaddrLength: i32 @+8, pad 4B)
    // followed by packed sockaddr payloads.
    let header_bytes = 8usize + addrs.len().max(1) * 16;
    let payload_bytes: usize = addrs.iter().map(Vec::len).sum();
    let needed = (header_bytes + payload_bytes) as u32;
    if !bytes_returned.is_null() {
        std::ptr::write_unaligned(bytes_returned, needed);
    }
    if out_buf.is_null() || out_len < needed {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let base = out_buf as *mut u8;
    std::ptr::write_bytes(base, 0, needed as usize);
    (base as *mut i32).write_unaligned(addrs.len() as i32);
    let mut data_off = header_bytes;
    for (i, sa_bytes) in addrs.iter().enumerate() {
        let sa_ptr = base.add(data_off);
        std::ptr::copy_nonoverlapping(sa_bytes.as_ptr(), sa_ptr, sa_bytes.len());
        let entry_ptr = base.add(8 + i * 16);
        (entry_ptr as *mut usize).write_unaligned(sa_ptr as usize);
        (entry_ptr.add(8) as *mut i32).write_unaligned(sa_bytes.len() as i32);
        data_off += sa_bytes.len();
    }
    0
}

unsafe fn wsa_ioctl_routing_interface_query(
    in_buf: *mut c_void,
    in_len: u32,
    out_buf: *mut c_void,
    out_len: u32,
    bytes_returned: *mut u32,
) -> i32 {
    if in_buf.is_null() || in_len < 16 {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let (storage, plen) = match win_to_posix_sockaddr(in_buf as *const u8, in_len as i32) {
        Ok(v) => v,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let paf = storage.ss_family as i32;
    let need_len: u32 = if paf == libc::AF_INET6 { 28 } else { 16 };
    if !bytes_returned.is_null() {
        std::ptr::write_unaligned(bytes_returned, need_len);
    }
    if out_buf.is_null() || out_len < need_len {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    // Probe the kernel routing table via a temporary unconnected UDP socket + connect() + getsockname().
    let probe_fd = libc::socket(paf, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
    if probe_fd < 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    let mut probe_sa = storage;
    // Darwin connect(UDP) requires a non-zero destination port when querying a route.
    if paf == libc::AF_INET {
        let sin = &mut *(&mut probe_sa as *mut _ as *mut libc::sockaddr_in);
        if sin.sin_port == 0 {
            sin.sin_port = 9u16.to_be();
        }
    } else if paf == libc::AF_INET6 {
        let sin6 = &mut *(&mut probe_sa as *mut _ as *mut libc::sockaddr_in6);
        if sin6.sin6_port == 0 {
            sin6.sin6_port = 9u16.to_be();
        }
    }
    let rc = libc::connect(
        probe_fd,
        &probe_sa as *const libc::sockaddr_storage as *const libc::sockaddr,
        plen,
    );
    if rc != 0 {
        let e = last_errno_wsa();
        libc::close(probe_fd);
        return err(e, SOCKET_ERROR);
    }
    let mut local_sa: libc::sockaddr_storage = std::mem::zeroed();
    let mut local_len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let gsn = libc::getsockname(
        probe_fd,
        &mut local_sa as *mut libc::sockaddr_storage as *mut libc::sockaddr,
        &mut local_len,
    );
    let gsn_err = if gsn != 0 { last_errno_wsa() } else { 0 };
    libc::close(probe_fd);
    if gsn != 0 {
        return err(gsn_err, SOCKET_ERROR);
    }
    // Zero ephemeral port on returned routing interface address, matching Windows.
    if local_sa.ss_family as i32 == libc::AF_INET {
        (*(&mut local_sa as *mut _ as *mut libc::sockaddr_in)).sin_port = 0;
    } else if local_sa.ss_family as i32 == libc::AF_INET6 {
        (*(&mut local_sa as *mut _ as *mut libc::sockaddr_in6)).sin6_port = 0;
    }
    let mut avail = out_len as i32;
    if let Err(e) = posix_to_win_sockaddr(
        &local_sa as *const libc::sockaddr_storage as *const libc::sockaddr,
        local_len,
        out_buf as *mut u8,
        &mut avail,
    ) {
        return err(e, SOCKET_ERROR);
    }
    if !bytes_returned.is_null() {
        std::ptr::write_unaligned(bytes_returned, avail as u32);
    }
    0
}

pub unsafe extern "win64" fn WSAIoctl(
    s: usize,
    io_control_code: u32,
    in_buf: *mut c_void,
    in_len: u32,
    out_buf: *mut c_void,
    out_len: u32,
    bytes_returned: *mut u32,
    overlapped: *mut c_void,
    completion: *mut c_void,
) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let fd = meta.fd;
    const SIO_GET_EXTENSION_FUNCTION_POINTER: u32 = 0xC800_0006;
    const SIO_KEEPALIVE_VALS: u32 = 0x9800_0004;
    const SIO_UDP_CONNRESET: u32 = 0x9800_000C;
    const SIO_UDP_NETRESET: u32 = 0x9800_000F;
    const SIO_IDEAL_SEND_BACKLOG_QUERY: u32 = 0x4004_747B;
    const SIO_GET_INTERFACE_LIST: u32 = 0x4004_747F;
    const SIO_ADDRESS_LIST_QUERY: u32 = 0x4800_0016;
    const SIO_ROUTING_INTERFACE_QUERY: u32 = 0xC800_0014;
    const WIN_FIONBIO: u32 = 0x8004_667E;
    const WIN_FIONREAD: u32 = 0x4004_667F;
    const WIN_SIOCATMARK: u32 = 0x4004_7307;
    if !bytes_returned.is_null() {
        std::ptr::write_unaligned(bytes_returned, 0);
    }
    let mut transferred = 0;
    let rc = match io_control_code {
        SIO_GET_EXTENSION_FUNCTION_POINTER => {
            if in_buf.is_null() || in_len < 16 || out_buf.is_null() || out_len < 8 {
                return err(WSAEFAULT, SOCKET_ERROR);
            }
            let guid = std::ptr::read_unaligned(in_buf as *const [u8; 16]);
            let Some(ext) = lookup_extension_kind(&guid) else {
                return err(WSAEOPNOTSUPP, SOCKET_ERROR);
            };
            let Some(thunk_va) = resolve_wsa_extension_thunk(ext) else {
                // Never disclose or fall back to a raw internal host code pointer if
                // guest thunk provisioning is unavailable.
                return err(WSAEOPNOTSUPP, SOCKET_ERROR);
            };
            std::ptr::write_unaligned(out_buf as *mut usize, thunk_va);
            transferred = size_of::<usize>() as u32;
            0
        }
        SIO_KEEPALIVE_VALS => {
            if meta.sock_type != libc::SOCK_STREAM {
                return err(WSAEOPNOTSUPP, SOCKET_ERROR);
            }
            if in_buf.is_null() || (in_len as usize) < std::mem::size_of::<WinTcpKeepalive>() {
                return err(WSAEFAULT, SOCKET_ERROR);
            }
            let ka = std::ptr::read_unaligned(in_buf as *const WinTcpKeepalive);
            let on: i32 = i32::from(ka.onoff != 0);
            if on != 0 {
                let idle_secs = ka.keepalivetime.div_ceil(1000).max(1) as i32;
                let _ = libc::setsockopt(
                    fd,
                    libc::IPPROTO_TCP,
                    libc::TCP_KEEPALIVE,
                    &idle_secs as *const i32 as *const c_void,
                    std::mem::size_of::<i32>() as libc::socklen_t,
                );
                let intvl_secs = ka.keepaliveinterval.div_ceil(1000).max(1) as i32;
                let _ = libc::setsockopt(
                    fd,
                    libc::IPPROTO_TCP,
                    libc::TCP_KEEPINTVL,
                    &intvl_secs as *const i32 as *const c_void,
                    std::mem::size_of::<i32>() as libc::socklen_t,
                );
            }
            if libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_KEEPALIVE,
                &on as *const i32 as *const c_void,
                std::mem::size_of::<i32>() as libc::socklen_t,
            ) != 0
            {
                return err(last_errno_wsa(), SOCKET_ERROR);
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
            transferred = 4;
            0
        }
        SIO_GET_INTERFACE_LIST => {
            let r = wsa_ioctl_get_interface_list(out_buf, out_len, &mut transferred);
            if !bytes_returned.is_null() {
                bytes_returned.write_unaligned(transferred);
            }
            r
        }
        SIO_ADDRESS_LIST_QUERY => {
            let r = wsa_ioctl_address_list_query(meta.family, out_buf, out_len, &mut transferred);
            if !bytes_returned.is_null() {
                bytes_returned.write_unaligned(transferred);
            }
            r
        }
        SIO_ROUTING_INTERFACE_QUERY => {
            let r = wsa_ioctl_routing_interface_query(in_buf, in_len, out_buf, out_len, &mut transferred);
            if !bytes_returned.is_null() {
                bytes_returned.write_unaligned(transferred);
            }
            r
        }
        WIN_FIONBIO if !in_buf.is_null() && in_len >= 4 => ioctlsocket(s, io_control_code as i32, in_buf as *mut u32),
        WIN_FIONREAD | WIN_SIOCATMARK if !out_buf.is_null() && out_len >= 4 => {
            let r = ioctlsocket(s, io_control_code as i32, out_buf as *mut u32);
            if r == 0 { transferred = 4; }
            r
        }
        WIN_FIONREAD | WIN_FIONBIO => err(WSAEFAULT, SOCKET_ERROR),
        _ => {
            // Standard Winsock2 behavior for unrecognized or provider-unsupported IOCTL codes.
            log::debug!("WSAIoctl: unrecognized/unsupported code={io_control_code:#x} -> WSAEOPNOTSUPP");
            err(WSAEOPNOTSUPP, SOCKET_ERROR)
        }
    };
    if rc == 0 {
        if !bytes_returned.is_null() {
            bytes_returned.write_unaligned(transferred);
        }
        if !overlapped.is_null() {
            let thread = (!completion.is_null()).then(crate::thread::ensure_current_thread_object);
            let binding = lookup_io_completion_binding(s as HANDLE);
            finish_overlapped_op(
                s,
                overlapped as usize,
                completion as usize,
                binding.as_deref(),
                thread.as_ref(),
                crate::teb::thread_id(),
                STATUS_SUCCESS,
                transferred as usize,
                0,
                true,
            );
        }
    }
    rc
}

pub unsafe extern "win64" fn send(s: usize, buf: *const u8, len: i32, flags: i32) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let fd = meta.fd;
    let mcast_dest = if !meta.connected { meta.mcast_dest } else { None };
    loop {
        let n = if let Some((ref dst, dlen)) = mcast_dest {
            libc::sendto(
                fd,
                buf as *const c_void,
                len.max(0) as usize,
                flags,
                dst as *const libc::sockaddr_storage as *const libc::sockaddr,
                dlen,
            )
        } else {
            libc::send(fd, buf as *const c_void, len.max(0) as usize, flags)
        };
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

const WIN_MSG_OOB: u32 = 0x1;
const WIN_MSG_PEEK: u32 = 0x2;
const WIN_MSG_DONTROUTE: u32 = 0x4;
const WIN_MSG_WAITALL: u32 = 0x8;
const WIN_MSG_PARTIAL: u32 = 0x8000;

/// Enforce Microsoft's `WSARecv` / `recv` `MSG_WAITALL` restrictions on the raw Windows flags
/// BEFORE POSIX flag translation (which drops Windows-only bits such as `MSG_PARTIAL`):
/// `MSG_WAITALL` is rejected with `WSAEOPNOTSUPP` on non-stream or guest-nonblocking sockets,
/// or when combined with `MSG_OOB`, `MSG_PEEK`, or `MSG_PARTIAL`.
fn validate_waitall_flags(win_flags: u32, is_stream: bool, guest_nonblocking: bool) -> Result<(), u32> {
    if (win_flags & WIN_MSG_WAITALL) != 0
        && (!is_stream
            || guest_nonblocking
            || (win_flags & (WIN_MSG_OOB | WIN_MSG_PEEK | WIN_MSG_PARTIAL)) != 0)
    {
        return Err(WSAEOPNOTSUPP);
    }
    Ok(())
}

pub unsafe extern "win64" fn recv(s: usize, buf: *mut u8, len: i32, flags: i32) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let win_flags = flags as u32;
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    if let Err(e) = validate_waitall_flags(win_flags, is_stream, meta.guest_nonblocking) {
        return err(e, SOCKET_ERROR);
    }
    let pflags = win_msg_flags_to_posix(win_flags);
    let call_flags = pflags & !libc::MSG_WAITALL;
    let fd = meta.fd;
    let total_want = len.max(0) as usize;
    let wait_all = (pflags & libc::MSG_WAITALL) != 0 && total_want > 0;
    let deadline = socket_timeout_duration(fd, libc::SO_RCVTIMEO).map(|d| std::time::Instant::now() + d);
    let mut got_total = 0usize;
    loop {
        let ptr = buf.add(got_total) as *mut c_void;
        let rem = total_want - got_total;
        let n = libc::recv(fd, ptr, rem, call_flags);
        if n >= 0 {
            let k = n as usize;
            got_total += k;
            if !wait_all || k == 0 || got_total >= total_want {
                event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                return got_total as i32;
            }
            continue;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready_until(fd, libc::POLLIN, deadline) {
                event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                return err(errno_to_wsa(we), SOCKET_ERROR);
            }
            continue;
        }
        event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
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
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let win_flags = flags as u32;
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    if let Err(e) = validate_waitall_flags(win_flags, is_stream, meta.guest_nonblocking) {
        return err(e, SOCKET_ERROR);
    }
    let pflags = win_msg_flags_to_posix(win_flags);
    let call_flags = pflags & !libc::MSG_WAITALL;
    let fd = meta.fd;
    let total_want = len.max(0) as usize;
    let wait_all = (pflags & libc::MSG_WAITALL) != 0 && total_want > 0;
    let deadline = socket_timeout_duration(fd, libc::SO_RCVTIMEO).map(|d| std::time::Instant::now() + d);
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen: libc::socklen_t = 0;
    let mut got_from = false;
    let mut got_total = 0usize;
    let n = loop {
        let capture_from = !from.is_null() && !got_from;
        if capture_from {
            slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        }
        let r = libc::recvfrom(
            fd,
            buf.add(got_total) as *mut c_void,
            total_want - got_total,
            call_flags,
            if capture_from { &mut storage as *mut libc::sockaddr_storage as *mut libc::sockaddr } else { std::ptr::null_mut() },
            if capture_from { &mut slen } else { std::ptr::null_mut() },
        );
        if r >= 0 {
            if capture_from {
                got_from = true;
            }
            let k = r as usize;
            got_total += k;
            if !wait_all || k == 0 || got_total >= total_want {
                break got_total;
            }
            continue;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready_until(fd, libc::POLLIN, deadline) {
                event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                return err(errno_to_wsa(we), SOCKET_ERROR);
            }
            continue;
        }
        event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
        return err(errno_to_wsa(raw_e), SOCKET_ERROR);
    };
    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
    if !from.is_null() && got_from && !fromlen.is_null() {
        let _ = posix_to_win_sockaddr(&storage as *const libc::sockaddr_storage as *const libc::sockaddr, slen, from, fromlen);
    }
    n as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WsaBuf {
    pub len: u32,
    pub buf: *mut u8,
}

fn win_msg_flags_to_posix(flags: u32) -> i32 {
    let mut out = 0i32;
    if (flags & WIN_MSG_OOB) != 0 {
        out |= libc::MSG_OOB;
    }
    if (flags & WIN_MSG_PEEK) != 0 {
        out |= libc::MSG_PEEK;
    }
    if (flags & WIN_MSG_DONTROUTE) != 0 {
        out |= libc::MSG_DONTROUTE;
    }
    if (flags & WIN_MSG_WAITALL) != 0 {
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

unsafe fn recvmsg_bufs_from(
    fd: i32,
    bufs: &[(usize, usize)],
    offset: usize,
    pflags: i32,
    mut from: Option<(&mut libc::sockaddr_storage, &mut libc::socklen_t)>,
    msg_flags_out: &mut i32,
) -> isize {
    let mut iovecs: Vec<libc::iovec> = Vec::with_capacity(bufs.len());
    if offset == 0 {
        for &(addr, len) in bufs {
            iovecs.push(libc::iovec {
                iov_base: addr as *mut c_void,
                iov_len: len,
            });
        }
    } else {
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
    }
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

/// Shared overlapped submission path for `WSASend` and `WSASendTo`.
///
/// For `SOCK_STREAM` sockets, synchronizes admission and any immediate nonblocking write with the
/// async reactor queue (`ASYNC_IO.inner`) using the same `has_prior_stream_write` predicate so a
/// later send can never bypass an earlier partially-sent or awaiting-retry stream write on the
/// same socket. Datagrams (`!is_stream`) remain independent and atomic; all completion delivery
/// happens strictly outside `ASYNC_IO.inner`.
#[allow(clippy::too_many_arguments)]
unsafe fn submit_overlapped_send(
    s: usize,
    fd: i32,
    bufs: Vec<(usize, usize)>,
    sent: *mut u32,
    is_stream: bool,
    pflags: i32,
    to: Option<(libc::sockaddr_storage, libc::socklen_t)>,
    ov: *mut OVERLAPPED,
    completion: *mut c_void,
) -> i32 {
    let total_len: usize = bufs.iter().map(|&(_, l)| l).sum();
    let issuing_tid = crate::kernel32::threads::GetCurrentThreadId();
    let issuing_thread = if !completion.is_null() {
        Some(crate::thread::ensure_current_thread_object())
    } else {
        crate::thread::current_thread_object()
    };
    let completion_binding = lookup_io_completion_binding(s as HANDLE);

    let step = if is_stream {
        let mut st = ASYNC_IO.inner.lock().unwrap();
        if has_prior_stream_write(&st.ops, s) {
            prepare_socket_pending(s, ov, completion as usize);
            ensure_async_io_started(&mut st);
            st.ops.push(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: fd,
                poll_events: libc::POLLOUT,
                overlapped: ov as usize,
                issuing_tid,
                issuing_thread: issuing_thread.clone(),
                completion_routine: completion as usize,
                completion_binding: completion_binding.clone(),
                kind: AsyncOpKind::Send {
                    bufs,
                    total_len,
                    sent_so_far: 0,
                    is_stream: true,
                    pflags,
                    to,
                },
            });
            wake_fd_write(st.wake_w);
            OverlappedSendStep::Pending
        } else {
            let mut sent_so_far = 0usize;
            match step_overlapped_send(fd, &bufs, total_len, &mut sent_so_far, true, pflags, to.as_ref()) {
                OverlappedSendStep::Complete(total) => OverlappedSendStep::Complete(total),
                OverlappedSendStep::Pending => {
                    prepare_socket_pending(s, ov, completion as usize);
                    ensure_async_io_started(&mut st);
                    st.ops.push(PendingSocketOp {
                        socket_handle: s,
                        secondary_socket: None,
                        poll_fd: fd,
                        poll_events: libc::POLLOUT,
                        overlapped: ov as usize,
                        issuing_tid,
                        issuing_thread: issuing_thread.clone(),
                        completion_routine: completion as usize,
                        completion_binding: completion_binding.clone(),
                        kind: AsyncOpKind::Send {
                            bufs,
                            total_len,
                            sent_so_far,
                            is_stream: true,
                            pflags,
                            to,
                        },
                    });
                    wake_fd_write(st.wake_w);
                    OverlappedSendStep::Pending
                }
                OverlappedSendStep::Error(raw_e) => OverlappedSendStep::Error(raw_e),
            }
        }
    } else {
        let mut sent_so_far = 0usize;
        match step_overlapped_send(fd, &bufs, total_len, &mut sent_so_far, false, pflags, to.as_ref()) {
            OverlappedSendStep::Complete(total) => OverlappedSendStep::Complete(total),
            OverlappedSendStep::Pending => {
                prepare_socket_pending(s, ov, completion as usize);
                async_io_enqueue(PendingSocketOp {
                    socket_handle: s,
                    secondary_socket: None,
                    poll_fd: fd,
                    poll_events: libc::POLLOUT,
                    overlapped: ov as usize,
                    issuing_tid,
                    issuing_thread: issuing_thread.clone(),
                    completion_routine: completion as usize,
                    completion_binding: completion_binding.clone(),
                    kind: AsyncOpKind::Send {
                        bufs,
                        total_len,
                        sent_so_far,
                        is_stream: false,
                        pflags,
                        to,
                    },
                });
                OverlappedSendStep::Pending
            }
            OverlappedSendStep::Error(raw_e) => OverlappedSendStep::Error(raw_e),
        }
    };

    match step {
        OverlappedSendStep::Complete(total) => {
            if !sent.is_null() {
                std::ptr::write_unaligned(sent, total as u32);
            }
            finish_overlapped_op(
                s,
                ov as usize,
                completion as usize,
                completion_binding.as_deref(),
                issuing_thread.as_ref(),
                issuing_tid,
                STATUS_SUCCESS,
                total,
                0,
                true,
            );
            0
        }
        OverlappedSendStep::Pending => err(WSA_IO_PENDING, SOCKET_ERROR),
        OverlappedSendStep::Error(raw_e) => err(errno_to_wsa(raw_e), SOCKET_ERROR),
    }
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
    send_to(s, buffers, count, sent, flags, std::ptr::null(), 0, overlapped, completion)
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
    send_to(s, buffers, count, sent, flags, to, tolen, overlapped, completion)
}

#[allow(clippy::too_many_arguments)]
unsafe fn send_to(
    s: usize, buffers: *const WsaBuf, count: u32, sent: *mut u32, flags: u32, to: *const u8,
    tolen: i32, overlapped: *mut c_void, completion: *mut c_void,
) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let fd = meta.fd;
    let bufs = match collect_wsa_bufs(buffers, count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    // Per Windows WSASendTo docs, `lpTo`/`iToLen` are ignored on connection-oriented sockets
    // (whereas Darwin `sendmsg` returns EISCONN if `msg_name` is set on a connected TCP socket).
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    let dest = if is_stream {
        None
    } else if !to.is_null() && tolen > 0 {
        match win_to_posix_sockaddr(to, tolen) {
            Ok(v) => Some(v),
            Err(e) => return err(e, SOCKET_ERROR),
        }
    } else if !meta.connected {
        meta.mcast_dest
    } else {
        None
    };
    let pflags = win_msg_flags_to_posix(flags);
    if overlapped.is_null() {
        let n = loop {
            let r = sendmsg_bufs_from(fd, &bufs, 0, pflags, dest.as_ref());
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

    submit_overlapped_send(
        s,
        fd,
        bufs,
        sent,
        is_stream,
        pflags,
        dest,
        overlapped as *mut OVERLAPPED,
        completion,
    )
}

enum RecvStep {
    Complete {
        nt_status: usize,
        bytes: usize,
        msg_flags: i32,
        from: Option<(libc::sockaddr_storage, libc::socklen_t)>,
    },
    Pending,
    Error(i32),
}

#[allow(clippy::too_many_arguments)]
unsafe fn step_overlapped_recv(
    fd: i32,
    bufs: &[(usize, usize)],
    total_len: usize,
    recvd_so_far: &mut usize,
    pflags: i32,
    is_stream: bool,
    want_from: bool,
    from_captured: &mut Option<(libc::sockaddr_storage, libc::socklen_t)>,
    acc_msg_flags: &mut i32,
) -> RecvStep {
    // Zero-length TCP stream receive is a readiness probe that does not consume bytes.
    if is_stream && total_len == 0 && !want_from {
        let mut pfd = libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        };
        let prc = libc::poll(&mut pfd, 1, 0);
        if prc < 0 {
            let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
            return if e == libc::EINTR {
                RecvStep::Pending
            } else {
                RecvStep::Error(e)
            };
        }
        if prc == 0 || (pfd.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR)) == 0 {
            return RecvStep::Pending;
        }
        let so_err = query_so_error(fd);
        return if so_err != 0 {
            RecvStep::Error(so_err)
        } else {
            RecvStep::Complete {
                nt_status: STATUS_SUCCESS,
                bytes: 0,
                msg_flags: 0,
                from: None,
            }
        };
    }

    let wait_all = is_stream
        && (pflags & libc::MSG_WAITALL) != 0
        && (pflags & (libc::MSG_PEEK | libc::MSG_OOB)) == 0
        && total_len > 0;
    let want_oob = is_stream && (pflags & libc::MSG_OOB) != 0;
    let nb_flags = (pflags & !libc::MSG_WAITALL) | libc::MSG_DONTWAIT;
    loop {
        let mut chunk_flags = 0i32;
        let capture_from = want_from && from_captured.is_none();
        let mut storage: libc::sockaddr_storage = std::mem::zeroed();
        let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        let r = if capture_from {
            recvmsg_bufs_from(
                fd,
                bufs,
                *recvd_so_far,
                nb_flags,
                Some((&mut storage, &mut slen)),
                &mut chunk_flags,
            )
        } else {
            recvmsg_bufs_from(fd, bufs, *recvd_so_far, nb_flags, None, &mut chunk_flags)
        };
        if r >= 0 {
            if capture_from {
                *from_captured = Some((storage, slen));
            }
            *acc_msg_flags |= chunk_flags;
            let k = r as usize;
            *recvd_so_far += k;
            if !wait_all || k == 0 || *recvd_so_far >= total_len {
                let truncated_dgram = !is_stream && (*acc_msg_flags & libc::MSG_TRUNC) != 0;
                let expedited_oob = is_stream
                    && *recvd_so_far > 0
                    && ((*acc_msg_flags & libc::MSG_OOB) != 0 || want_oob);
                if expedited_oob {
                    *acc_msg_flags |= libc::MSG_OOB;
                }
                let nt_status = if truncated_dgram {
                    win32_to_ntstatus(WSAEMSGSIZE)
                } else if expedited_oob {
                    STATUS_RECEIVE_EXPEDITED
                } else {
                    STATUS_SUCCESS
                };
                return RecvStep::Complete {
                    nt_status,
                    bytes: *recvd_so_far,
                    msg_flags: *acc_msg_flags,
                    from: from_captured.take(),
                };
            }
            continue;
        }
        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if e == libc::EINTR {
            continue;
        }
        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
            return RecvStep::Pending;
        }
        if want_oob && e == libc::EINVAL {
            // Darwin/BSD `recvmsg(..., MSG_OOB | MSG_DONTWAIT)` returns `EINVAL` when no urgent
            // byte has arrived yet on a connected stream socket with `SO_OOBINLINE == 0`.
            // Distinguish genuine `EINVAL` (`SO_OOBINLINE`), unconnected sockets, peer EOF, and
            // socket errors from "urgent byte pending".
            let mut oobinline: libc::c_int = 0;
            let mut optlen = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
            if libc::getsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_OOBINLINE,
                &mut oobinline as *mut _ as *mut c_void,
                &mut optlen,
            ) == 0
                && oobinline != 0
            {
                return RecvStep::Error(libc::EINVAL);
            }
            let so_err = query_so_error(fd);
            if so_err != 0 {
                return RecvStep::Error(so_err);
            }
            let mut peer: libc::sockaddr_storage = std::mem::zeroed();
            let mut plen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
            if libc::getpeername(fd, &mut peer as *mut _ as *mut libc::sockaddr, &mut plen) != 0 {
                let pe = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::ENOTCONN);
                return RecvStep::Error(pe);
            }
            let mut peek_b = 0u8;
            let pr = libc::recv(
                fd,
                &mut peek_b as *mut u8 as *mut c_void,
                1,
                libc::MSG_PEEK | libc::MSG_DONTWAIT,
            );
            if pr == 0 {
                return RecvStep::Complete {
                    nt_status: STATUS_SUCCESS,
                    bytes: 0,
                    msg_flags: 0,
                    from: from_captured.take(),
                };
            }
            if pr < 0 {
                let pe = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
                if pe != libc::EAGAIN && pe != libc::EWOULDBLOCK && pe != libc::EINTR {
                    return RecvStep::Error(pe);
                }
            }
            return RecvStep::Pending;
        }
        return RecvStep::Error(e);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn sync_wsa_recv(
    s: usize,
    fd: i32,
    bufs: &[(usize, usize)],
    recvd: *mut u32,
    flags: *mut u32,
    from: *mut u8,
    fromlen: *mut i32,
    is_stream: bool,
    pflags: i32,
) -> i32 {
    let want_from = !from.is_null();
    let total_len: usize = bufs.iter().map(|&(_, l)| l).sum();
    let wait_all = is_stream
        && (pflags & libc::MSG_WAITALL) != 0
        && !is_guest_nonblocking(s)
        && (pflags & (libc::MSG_PEEK | libc::MSG_OOB)) == 0
        && total_len > 0;
    let call_flags = pflags & !libc::MSG_WAITALL;
    let deadline = socket_timeout_duration(fd, libc::SO_RCVTIMEO).map(|d| std::time::Instant::now() + d);
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let mut slen: libc::socklen_t = 0;
    let mut got_from = false;
    let mut msg_flags = 0i32;
    let mut got_total = 0usize;
    let n = loop {
        let mut chunk_flags = 0i32;
        let capture_from = want_from && !got_from;
        if capture_from {
            slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        }
        let r = if capture_from {
            recvmsg_bufs_from(fd, bufs, got_total, call_flags, Some((&mut storage, &mut slen)), &mut chunk_flags)
        } else {
            recvmsg_bufs_from(fd, bufs, got_total, call_flags, None, &mut chunk_flags)
        };
        if r >= 0 {
            if capture_from {
                got_from = true;
            }
            msg_flags |= chunk_flags;
            let k = r as usize;
            got_total += k;
            if !wait_all || k == 0 || got_total >= total_len {
                break got_total;
            }
            continue;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINTR {
            continue;
        }
        if (raw_e == libc::EAGAIN || raw_e == libc::EWOULDBLOCK) && !is_guest_nonblocking(s) {
            if let Err(we) = wait_socket_ready_until(fd, libc::POLLIN, deadline) {
                event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                return err(errno_to_wsa(we), SOCKET_ERROR);
            }
            continue;
        }
        event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
        return err(errno_to_wsa(raw_e), SOCKET_ERROR);
    };
    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
    if want_from && got_from && !fromlen.is_null() {
        let _ = posix_to_win_sockaddr(
            &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
            slen,
            from,
            fromlen,
        );
    }
    if !flags.is_null() {
        let out_fl = if (msg_flags & libc::MSG_OOB) != 0 { WIN_MSG_OOB } else { 0 };
        std::ptr::write_unaligned(flags, out_fl);
    }
    if !recvd.is_null() {
        std::ptr::write_unaligned(recvd, n as u32);
    }
    if !is_stream && (msg_flags & libc::MSG_TRUNC) != 0 {
        return err(WSAEMSGSIZE, SOCKET_ERROR);
    }
    0
}

/// Shared overlapped submission path for `WSARecv` and `WSARecvFrom`.
///
/// Serializes admission, immediate nonblocking receive, and reactor progress under `ASYNC_IO.inner`
/// using the shared `has_prior_recv` predicate so a later receive can never consume data ahead of
/// an earlier queued receive on the same socket. Sends and distinct sockets remain independent, and
/// all completion delivery happens strictly outside `ASYNC_IO.inner`.
#[allow(clippy::too_many_arguments)]
unsafe fn submit_overlapped_recv(
    s: usize,
    fd: i32,
    bufs: Vec<(usize, usize)>,
    recvd: *mut u32,
    flags: *mut u32,
    from: *mut u8,
    fromlen: *mut i32,
    is_stream: bool,
    pflags: i32,
    ov: *mut OVERLAPPED,
    completion: *mut c_void,
) -> i32 {
    let total_len: usize = bufs.iter().map(|&(_, l)| l).sum();
    let want_from = !from.is_null();
    let issuing_tid = crate::kernel32::threads::GetCurrentThreadId();
    let issuing_thread = if !completion.is_null() {
        Some(crate::thread::ensure_current_thread_object())
    } else {
        crate::thread::current_thread_object()
    };
    let completion_binding = lookup_io_completion_binding(s as HANDLE);

    let mut recvd_so_far = 0usize;
    let mut from_captured = None;
    let mut acc_msg_flags = 0i32;

    let step = {
        let mut st = ASYNC_IO.inner.lock().unwrap();
        let step = if has_prior_recv(&st.ops, s) {
            RecvStep::Pending
        } else {
            step_overlapped_recv(
                fd,
                &bufs,
                total_len,
                &mut recvd_so_far,
                pflags,
                is_stream,
                want_from,
                &mut from_captured,
                &mut acc_msg_flags,
            )
        };
        if matches!(step, RecvStep::Pending) {
            prepare_socket_pending(s, ov, completion as usize);
            ensure_async_io_started(&mut st);
            let poll_events = if is_stream && (pflags & libc::MSG_OOB) != 0 {
                libc::POLLPRI
            } else {
                libc::POLLIN
            };
            st.ops.push(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: fd,
                poll_events,
                overlapped: ov as usize,
                issuing_tid,
                issuing_thread: issuing_thread.clone(),
                completion_routine: completion as usize,
                completion_binding: completion_binding.clone(),
                kind: AsyncOpKind::Recv {
                    bufs,
                    total_len,
                    recvd_so_far,
                    pflags,
                    is_stream,
                    from: from as usize,
                    fromlen: fromlen as usize,
                    from_captured,
                    msg_flags: acc_msg_flags,
                },
            });
            wake_fd_write(st.wake_w);
        }
        step
    };

    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);

    match step {
        RecvStep::Complete {
            nt_status,
            bytes,
            msg_flags,
            from: got_from,
        } => {
            if let Some((storage, slen)) = got_from {
                if !from.is_null() && !fromlen.is_null() {
                    let _ = posix_to_win_sockaddr(
                        &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
                        slen,
                        from,
                        fromlen,
                    );
                }
            }
            if !flags.is_null() {
                let out_fl = if (msg_flags & libc::MSG_OOB) != 0 { WIN_MSG_OOB } else { 0 };
                std::ptr::write_unaligned(flags, out_fl);
            }
            if !recvd.is_null() {
                std::ptr::write_unaligned(recvd, bytes as u32);
            }
            if !is_io_success_status(nt_status) {
                // Immediate failure (e.g. oversized UDP datagram -> WSAEMSGSIZE): per Win32
                // WSARecv/WSARecvFrom contract, return SOCKET_ERROR with the WSA error code
                // and emit no completion indication.
                return err(ntstatus_to_wsa(nt_status), SOCKET_ERROR);
            }
            finish_overlapped_op(
                s,
                ov as usize,
                completion as usize,
                completion_binding.as_deref(),
                issuing_thread.as_ref(),
                issuing_tid,
                nt_status,
                bytes,
                ntstatus_to_wsa_recv_flags(nt_status),
                true,
            );
            0
        }
        RecvStep::Pending => err(WSA_IO_PENDING, SOCKET_ERROR),
        RecvStep::Error(raw_e) => err(errno_to_wsa(raw_e), SOCKET_ERROR),
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
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let bufs = match collect_wsa_bufs(buffers, count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let fl = if flags.is_null() { 0 } else { std::ptr::read_unaligned(flags) };
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    if let Err(e) = validate_waitall_flags(fl, is_stream, meta.guest_nonblocking) {
        return err(e, SOCKET_ERROR);
    }
    let pflags = win_msg_flags_to_posix(fl);
    if overlapped.is_null() {
        return sync_wsa_recv(
            s,
            meta.fd,
            &bufs,
            recvd,
            flags,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            is_stream,
            pflags,
        );
    }

    submit_overlapped_recv(
        s,
        meta.fd,
        bufs,
        recvd,
        flags,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        is_stream,
        pflags,
        overlapped as *mut OVERLAPPED,
        completion,
    )
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
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let bufs = match collect_wsa_bufs(buffers, count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let fl = if flags.is_null() { 0 } else { std::ptr::read_unaligned(flags) };
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    if let Err(e) = validate_waitall_flags(fl, is_stream, meta.guest_nonblocking) {
        return err(e, SOCKET_ERROR);
    }
    let pflags = win_msg_flags_to_posix(fl);
    if overlapped.is_null() {
        return sync_wsa_recv(s, meta.fd, &bufs, recvd, flags, from, fromlen, is_stream, pflags);
    }

    submit_overlapped_recv(
        s,
        meta.fd,
        bufs,
        recvd,
        flags,
        from,
        fromlen,
        is_stream,
        pflags,
        overlapped as *mut OVERLAPPED,
        completion,
    )
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
        let out_flags = load_ov_result_flags(s, ov as usize)
            .unwrap_or_else(|| ntstatus_to_wsa_recv_flags(status));
        std::ptr::write_unaligned(flags, out_flags);
    }
    if is_io_success_status(status) {
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
    if out.is_null() || (out_len.read_unaligned() as usize) < need {
        out_len.write_unaligned(need as u32);
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), out, s.len());
    *out.add(s.len()) = 0;
    out_len.write_unaligned(need as u32);
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

pub unsafe extern "win64" fn WSAHtonl(s: usize, hostlong: u32, lpnetlong: *mut u32) -> i32 {
    if lookup_meta(s).is_none() {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    }
    if lpnetlong.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    lpnetlong.write_unaligned(hostlong.to_be());
    0
}

pub unsafe extern "win64" fn WSAHtons(s: usize, hostshort: u16, lpnetshort: *mut u16) -> i32 {
    if lookup_meta(s).is_none() {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    }
    if lpnetshort.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    lpnetshort.write_unaligned(hostshort.to_be());
    0
}

pub unsafe extern "win64" fn WSANtohl(s: usize, netlong: u32, lphostlong: *mut u32) -> i32 {
    if lookup_meta(s).is_none() {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    }
    if lphostlong.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    lphostlong.write_unaligned(u32::from_be(netlong));
    0
}

pub unsafe extern "win64" fn WSANtohs(s: usize, netshort: u16, lphostshort: *mut u16) -> i32 {
    if lookup_meta(s).is_none() {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    }
    if lphostshort.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    lphostshort.write_unaligned(u16::from_be(netshort));
    0
}

pub unsafe extern "win64" fn WSARecvDisconnect(s: usize, _lp_inbound_disconnect_data: *mut c_void) -> i32 {
    shutdown(s, 0)
}

pub unsafe extern "win64" fn WSASendDisconnect(s: usize, _lp_outbound_disconnect_data: *mut c_void) -> i32 {
    shutdown(s, 1)
}

unsafe fn enum_protocols_impl(
    lpi_protocols: *const i32,
    buf: *mut u8,
    lpdw_buffer_length: *mut u32,
    wide: bool,
) -> i32 {
    if lpdw_buffer_length.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    // Standard Windows Winsock2 TCP/UDP IPv4/IPv6 providers.
    const ENTRIES: &[(u32, i32, i32, i32, i32, i32, &str)] = &[
        // (service_flags1, af, max_sa, min_sa, sock_type, proto, name)
        (0x0002_0066, AF_INET as i32, 16, 16, libc::SOCK_STREAM, libc::IPPROTO_TCP, "MSAFD Tcpip [TCP/IP]"),
        (0x0002_0209, AF_INET as i32, 16, 16, libc::SOCK_DGRAM, libc::IPPROTO_UDP, "MSAFD Tcpip [UDP/IP]"),
        (0x0002_0066, AF_INET6 as i32, 28, 28, libc::SOCK_STREAM, libc::IPPROTO_TCP, "MSAFD Tcpip [TCP/IPv6]"),
        (0x0002_0209, AF_INET6 as i32, 28, 28, libc::SOCK_DGRAM, libc::IPPROTO_UDP, "MSAFD Tcpip [UDP/IPv6]"),
    ];
    let mut filter: Vec<i32> = Vec::new();
    if !lpi_protocols.is_null() {
        let mut i = 0usize;
        while i < 64 {
            let p = lpi_protocols.add(i).read_unaligned();
            if p == 0 {
                break;
            }
            filter.push(p);
            i += 1;
        }
    }
    let matched: Vec<_> = ENTRIES
        .iter()
        .enumerate()
        .filter(|(_, e)| filter.is_empty() || filter.contains(&e.5))
        .collect();
    let entry_size: usize = if wide { 628 } else { 372 };
    let needed = (matched.len() * entry_size) as u32;
    let avail = lpdw_buffer_length.read_unaligned();
    if buf.is_null() || avail < needed {
        lpdw_buffer_length.write_unaligned(needed);
        return err(WSAENOBUFS, SOCKET_ERROR);
    }
    for (idx, (cat_idx, &(flags1, af, max_sa, min_sa, stype, proto, name))) in matched.iter().enumerate() {
        let dst = buf.add(idx * entry_size);
        std::ptr::write_bytes(dst, 0, entry_size);
        (dst.add(0) as *mut u32).write_unaligned(flags1);          // dwServiceFlags1
        (dst.add(36) as *mut u32).write_unaligned(1001 + *cat_idx as u32); // dwCatalogEntryId
        (dst.add(40) as *mut i32).write_unaligned(1);              // ProtocolChain.ChainLen = BASE_PROTOCOL
        (dst.add(72) as *mut i32).write_unaligned(2);              // iVersion
        (dst.add(76) as *mut i32).write_unaligned(af);             // iAddressFamily
        (dst.add(80) as *mut i32).write_unaligned(max_sa);         // iMaxSockAddr
        (dst.add(84) as *mut i32).write_unaligned(min_sa);         // iMinSockAddr
        (dst.add(88) as *mut i32).write_unaligned(stype);          // iSocketType
        (dst.add(92) as *mut i32).write_unaligned(proto);          // iProtocol
        (dst.add(100) as *mut i32).write_unaligned(1);             // iNetworkByteOrder = LITTLEENDIAN
        (dst.add(104) as *mut i32).write_unaligned(0);             // iSecurityScheme
        (dst.add(108) as *mut u32).write_unaligned(if stype == libc::SOCK_DGRAM { 65507 } else { 0 }); // dwMessageSize
        // szProtocol starts at offset 116 (256 chars: u8 for A, u16 for W).
        if wide {
            crate::strings::write_wide_trunc(name, dst.add(116) as *mut u16, 256);
        } else {
            let bytes = name.as_bytes();
            let n = bytes.len().min(255);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst.add(116), n);
            *dst.add(116 + n) = 0;
        }
    }
    lpdw_buffer_length.write_unaligned(needed);
    matched.len() as i32
}

pub unsafe extern "win64" fn WSAEnumProtocolsA(
    lpi_protocols: *const i32,
    lp_protocol_buffer: *mut u8,
    lpdw_buffer_length: *mut u32,
) -> i32 {
    enum_protocols_impl(lpi_protocols, lp_protocol_buffer, lpdw_buffer_length, false)
}

pub unsafe extern "win64" fn WSAEnumProtocolsW(
    lpi_protocols: *const i32,
    lp_protocol_buffer: *mut u8,
    lpdw_buffer_length: *mut u32,
) -> i32 {
    enum_protocols_impl(lpi_protocols, lp_protocol_buffer, lpdw_buffer_length, true)
}

/// Win64 `WSAMSG` (56 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WsaMsg {
    pub name: *mut u8,
    pub namelen: i32,
    pub _pad0: u32,
    pub lp_buffers: *mut WsaBuf,
    pub dw_buffer_count: u32,
    pub _pad1: u32,
    pub control: WsaBuf,
    pub dw_flags: u32,
    pub _pad2: u32,
}
const _: () = assert!(std::mem::size_of::<WsaMsg>() == 56);

/// Win64 `WSACMSGHDR` (16 bytes; aligned to 8 bytes on x64).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WsaCmsgHdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
const _: () = assert!(std::mem::size_of::<WsaCmsgHdr>() == 16);

const WIN_MSG_TRUNC: u32 = 0x0100;
const WIN_MSG_CTRUNC: u32 = 0x0200;
const WIN_MSG_BCAST: u32 = 0x0400;
const WIN_MSG_MCAST: u32 = 0x0800;

const WIN_IPPROTO_IP_CMSG: i32 = 0;
const WIN_IPPROTO_IPV6_CMSG: i32 = 41;
const WIN_IP_TOS_CMSG: i32 = 3;
const WIN_IP_TTL_CMSG: i32 = 4;
const WIN_IP_PKTINFO_CMSG: i32 = 19;
const WIN_IPV6_PKTINFO_CMSG: i32 = 19;
const WIN_IPV6_HOPLIMIT_CMSG: i32 = 21;
const WIN_IPV6_TCLASS_CMSG: i32 = 39;

#[inline]
fn win_cmsg_align(n: usize) -> usize {
    (n + 7) & !7
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DarwinInPktinfo {
    ipi_ifindex: u32,
    ipi_spec_dst: [u8; 4],
    ipi_addr: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WinInPktinfo {
    ipi_addr: [u8; 4],
    ipi_ifindex: u32,
}

/// Convert Windows `WSAMSG.Control` (`WSACMSGHDR`, 16B header, 8B align) into a Darwin
/// `msghdr.msg_control` buffer (`cmsghdr`, 12B header, 4B align).
unsafe fn translate_win_cmsg_to_posix(win_buf: *const u8, win_len: usize) -> Vec<u8> {
    if win_buf.is_null() || win_len < std::mem::size_of::<WsaCmsgHdr>() {
        return Vec::new();
    }
    let mut entries: Vec<(i32, i32, Vec<u8>)> = Vec::new();
    let mut off = 0usize;
    while off + std::mem::size_of::<WsaCmsgHdr>() <= win_len {
        let hdr = std::ptr::read_unaligned(win_buf.add(off) as *const WsaCmsgHdr);
        if hdr.cmsg_len < std::mem::size_of::<WsaCmsgHdr>() || off + hdr.cmsg_len > win_len {
            break;
        }
        let data_len = hdr.cmsg_len - std::mem::size_of::<WsaCmsgHdr>();
        let data_ptr = win_buf.add(off + std::mem::size_of::<WsaCmsgHdr>());
        let data = std::slice::from_raw_parts(data_ptr, data_len);

        match (hdr.cmsg_level, hdr.cmsg_type) {
            (WIN_IPPROTO_IP_CMSG, WIN_IP_PKTINFO_CMSG) if data_len >= 8 => {
                let win_pi = std::ptr::read_unaligned(data.as_ptr() as *const WinInPktinfo);
                let dar_pi = DarwinInPktinfo {
                    ipi_ifindex: win_pi.ipi_ifindex,
                    ipi_spec_dst: win_pi.ipi_addr,
                    ipi_addr: win_pi.ipi_addr,
                };
                let bytes = std::slice::from_raw_parts(
                    &dar_pi as *const DarwinInPktinfo as *const u8,
                    std::mem::size_of::<DarwinInPktinfo>(),
                );
                entries.push((libc::IPPROTO_IP, libc::IP_PKTINFO, bytes.to_vec()));
            }
            (WIN_IPPROTO_IP_CMSG, WIN_IP_TTL_CMSG) => {
                entries.push((libc::IPPROTO_IP, libc::IP_TTL, data.to_vec()));
            }
            (WIN_IPPROTO_IP_CMSG, WIN_IP_TOS_CMSG) => {
                entries.push((libc::IPPROTO_IP, libc::IP_TOS, data.to_vec()));
            }
            (WIN_IPPROTO_IPV6_CMSG, WIN_IPV6_PKTINFO_CMSG) if data_len >= 20 => {
                entries.push((libc::IPPROTO_IPV6, libc::IPV6_PKTINFO, data[..20].to_vec()));
            }
            (WIN_IPPROTO_IPV6_CMSG, WIN_IPV6_HOPLIMIT_CMSG) => {
                entries.push((libc::IPPROTO_IPV6, libc::IPV6_HOPLIMIT, data.to_vec()));
            }
            (WIN_IPPROTO_IPV6_CMSG, WIN_IPV6_TCLASS_CMSG) => {
                entries.push((libc::IPPROTO_IPV6, libc::IPV6_TCLASS, data.to_vec()));
            }
            _ => {}
        }
        let step = win_cmsg_align(hdr.cmsg_len);
        if step == 0 {
            break;
        }
        off += step;
    }
    if entries.is_empty() {
        return Vec::new();
    }
    let total_space: usize = entries
        .iter()
        .map(|(_, _, d)| libc::CMSG_SPACE(d.len() as u32) as usize)
        .sum();
    let mut out = vec![0u8; total_space];
    let mut dummy: libc::msghdr = std::mem::zeroed();
    dummy.msg_control = out.as_mut_ptr() as *mut c_void;
    dummy.msg_controllen = out.len() as libc::socklen_t;
    let mut cmsg = libc::CMSG_FIRSTHDR(&dummy);
    for (level, ty, data) in &entries {
        if cmsg.is_null() {
            break;
        }
        (*cmsg).cmsg_level = *level;
        (*cmsg).cmsg_type = *ty;
        (*cmsg).cmsg_len = libc::CMSG_LEN(data.len() as u32);
        let dst = libc::CMSG_DATA(cmsg);
        std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
        cmsg = libc::CMSG_NXTHDR(&dummy, cmsg);
    }
    out
}

/// Convert Darwin `msghdr.msg_control` back into a Windows `WSAMSG.Control` buffer.
/// Returns `(bytes_written, truncated, extra_win_flags)`.
unsafe fn translate_posix_cmsg_to_win(
    posix_msg: &libc::msghdr,
    win_buf: *mut u8,
    win_cap: usize,
) -> (usize, bool, u32) {
    if win_buf.is_null() || win_cap == 0 || posix_msg.msg_control.is_null() || posix_msg.msg_controllen == 0 {
        return (0, false, 0);
    }
    let mut off = 0usize;
    let mut truncated = (posix_msg.msg_flags & libc::MSG_CTRUNC) != 0;
    let mut extra_flags = 0u32;
    let mut cmsg = libc::CMSG_FIRSTHDR(posix_msg);
    while !cmsg.is_null() {
        let level = (*cmsg).cmsg_level;
        let ty = (*cmsg).cmsg_type;
        let data_ptr = libc::CMSG_DATA(cmsg);
        let hdr_offset = (data_ptr as usize).saturating_sub(cmsg as usize);
        let data_len = ((*cmsg).cmsg_len as usize).saturating_sub(hdr_offset);
        let data = std::slice::from_raw_parts(data_ptr, data_len);

        let translated: Option<(i32, i32, Vec<u8>)> = match (level, ty) {
            (libc::IPPROTO_IP, libc::IP_PKTINFO) if data_len >= std::mem::size_of::<DarwinInPktinfo>() => {
                let dar_pi = std::ptr::read_unaligned(data.as_ptr() as *const DarwinInPktinfo);
                let ip = u32::from_be_bytes(dar_pi.ipi_addr);
                if ip == u32::MAX {
                    extra_flags |= WIN_MSG_BCAST;
                } else if (ip >> 28) == 14 {
                    extra_flags |= WIN_MSG_MCAST;
                }
                let win_pi = WinInPktinfo {
                    ipi_addr: dar_pi.ipi_addr,
                    ipi_ifindex: dar_pi.ipi_ifindex,
                };
                let bytes = std::slice::from_raw_parts(
                    &win_pi as *const WinInPktinfo as *const u8,
                    std::mem::size_of::<WinInPktinfo>(),
                );
                Some((WIN_IPPROTO_IP_CMSG, WIN_IP_PKTINFO_CMSG, bytes.to_vec()))
            }
            (libc::IPPROTO_IP, libc::IP_RECVTTL) | (libc::IPPROTO_IP, libc::IP_TTL) => {
                let ttl: u32 = if data_len >= 4 {
                    std::ptr::read_unaligned(data.as_ptr() as *const u32)
                } else if data_len >= 1 {
                    data[0] as u32
                } else {
                    0
                };
                Some((WIN_IPPROTO_IP_CMSG, WIN_IP_TTL_CMSG, ttl.to_ne_bytes().to_vec()))
            }
            (libc::IPPROTO_IP, libc::IP_RECVTOS) | (libc::IPPROTO_IP, libc::IP_TOS) => {
                let tos: u32 = if data_len >= 4 {
                    std::ptr::read_unaligned(data.as_ptr() as *const u32)
                } else if data_len >= 1 {
                    data[0] as u32
                } else {
                    0
                };
                Some((WIN_IPPROTO_IP_CMSG, WIN_IP_TOS_CMSG, tos.to_ne_bytes().to_vec()))
            }
            (libc::IPPROTO_IPV6, libc::IPV6_PKTINFO) if data_len >= 20 => {
                if data[0] == 0xff {
                    extra_flags |= WIN_MSG_MCAST;
                }
                Some((WIN_IPPROTO_IPV6_CMSG, WIN_IPV6_PKTINFO_CMSG, data[..20].to_vec()))
            }
            (libc::IPPROTO_IPV6, libc::IPV6_HOPLIMIT) => {
                Some((WIN_IPPROTO_IPV6_CMSG, WIN_IPV6_HOPLIMIT_CMSG, data.to_vec()))
            }
            (libc::IPPROTO_IPV6, libc::IPV6_TCLASS) => {
                Some((WIN_IPPROTO_IPV6_CMSG, WIN_IPV6_TCLASS_CMSG, data.to_vec()))
            }
            _ => None,
        };

        if let Some((wlevel, wtype, wdata)) = translated {
            let entry_len = std::mem::size_of::<WsaCmsgHdr>() + wdata.len();
            let aligned_len = win_cmsg_align(entry_len);
            if off + entry_len > win_cap {
                truncated = true;
                break;
            }
            let whdr = WsaCmsgHdr {
                cmsg_len: entry_len,
                cmsg_level: wlevel,
                cmsg_type: wtype,
            };
            std::ptr::write_unaligned(win_buf.add(off) as *mut WsaCmsgHdr, whdr);
            std::ptr::copy_nonoverlapping(
                wdata.as_ptr(),
                win_buf.add(off + std::mem::size_of::<WsaCmsgHdr>()),
                wdata.len(),
            );
            let pad_end = (off + aligned_len).min(win_cap);
            if pad_end > off + entry_len {
                std::ptr::write_bytes(win_buf.add(off + entry_len), 0, pad_end - (off + entry_len));
            }
            off = pad_end;
        }
        cmsg = libc::CMSG_NXTHDR(posix_msg, cmsg);
    }
    (off, truncated, extra_flags)
}

fn posix_recvmsg_flags_to_win(pflags: i32, ctrunc: bool, extra_flags: u32) -> u32 {
    let mut out = extra_flags;
    if (pflags & libc::MSG_OOB) != 0 {
        out |= WIN_MSG_OOB;
    }
    if (pflags & libc::MSG_TRUNC) != 0 {
        out |= WIN_MSG_TRUNC;
    }
    if ctrunc || (pflags & libc::MSG_CTRUNC) != 0 {
        out |= WIN_MSG_CTRUNC;
    }
    out
}

/// Execute a single nonblocking `recvmsg` into a guest `WsaMsg`, writing back `name`/`namelen`,
/// translated `control` buffer, and `dw_flags`. Returns `Ok((bytes, nt_status, wsa_flags))`,
/// `Err(0)` for `EAGAIN`/`EWOULDBLOCK` (pending), or `Err(raw_errno)` for hard errors.
unsafe fn try_wsa_recvmsg_once(
    fd: i32,
    is_stream: bool,
    lp_msg: *mut WsaMsg,
    bufs: &[(usize, usize)],
    pflags: i32,
) -> Result<(usize, usize, u32), i32> {
    let mut wmsg = std::ptr::read_unaligned(lp_msg);
    if (wmsg.control.len > 0 && wmsg.control.buf.is_null())
        || (wmsg.namelen > 0 && wmsg.name.is_null())
        || wmsg.namelen < 0
    {
        return Err(libc::EFAULT);
    }
    let mut iovecs: Vec<libc::iovec> = bufs
        .iter()
        .map(|&(addr, len)| libc::iovec {
            iov_base: addr as *mut c_void,
            iov_len: len,
        })
        .collect();
    let mut storage: libc::sockaddr_storage = std::mem::zeroed();
    let want_name = !wmsg.name.is_null();
    let want_ctrl = !wmsg.control.buf.is_null() && wmsg.control.len > 0;
    let mut ctrl_buf = if want_ctrl {
        vec![0u8; (wmsg.control.len as usize).max(256)]
    } else {
        Vec::new()
    };

    let mut pmsg: libc::msghdr = std::mem::zeroed();
    if want_name {
        pmsg.msg_name = &mut storage as *mut _ as *mut c_void;
        pmsg.msg_namelen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    }
    pmsg.msg_iov = iovecs.as_mut_ptr();
    pmsg.msg_iovlen = iovecs.len() as libc::c_int;
    if want_ctrl {
        pmsg.msg_control = ctrl_buf.as_mut_ptr() as *mut c_void;
        pmsg.msg_controllen = ctrl_buf.len() as libc::socklen_t;
    }

    let n = loop {
        let r = libc::recvmsg(fd, &mut pmsg, pflags | libc::MSG_DONTWAIT);
        if r >= 0 {
            break r as usize;
        }
        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if e == libc::EINTR {
            continue;
        }
        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
            return Err(0);
        }
        return Err(e);
    };

    let mut name_err: Option<u32> = None;
    if want_name && pmsg.msg_namelen > 0 {
        let mut out_len = wmsg.namelen;
        match posix_to_win_sockaddr(
            &storage as *const _ as *const libc::sockaddr,
            pmsg.msg_namelen,
            wmsg.name,
            &mut out_len,
        ) {
            Ok(()) => {
                wmsg.namelen = out_len;
            }
            Err(wsa) => {
                name_err = Some(wsa);
            }
        }
    }
    if let Some(wsa) = name_err {
        let status = match wsa {
            WSAEFAULT => 0xC000_0005usize, // STATUS_ACCESS_VIOLATION -> WSAEFAULT
            WSAEINVAL => 0xC000_000Dusize, // STATUS_INVALID_PARAMETER -> WSAEINVAL
            other => win32_to_ntstatus(other),
        };
        return Ok((0, status, 0));
    }
    let (ctrunc, extra_flags) = if want_ctrl {
        let (written, trunc, extra) =
            translate_posix_cmsg_to_win(&pmsg, wmsg.control.buf, wmsg.control.len as usize);
        wmsg.control.len = written as u32;
        (trunc, extra)
    } else {
        (false, 0)
    };
    let wsa_flags = posix_recvmsg_flags_to_win(pmsg.msg_flags, ctrunc, extra_flags);
    wmsg.dw_flags = wsa_flags;
    std::ptr::write_unaligned(lp_msg, wmsg);

    let nt_status = if !is_stream
        && ((pmsg.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC)) != 0 || ctrunc)
    {
        win32_to_ntstatus(WSAEMSGSIZE)
    } else {
        STATUS_SUCCESS
    };
    Ok((n, nt_status, wsa_flags))
}

/// Execute a single nonblocking `sendmsg` for `WSASendMsg`.
unsafe fn try_wsa_sendmsg_once(
    fd: i32,
    bufs: &[(usize, usize)],
    to: Option<&(libc::sockaddr_storage, libc::socklen_t)>,
    ctrl: &[u8],
    pflags: i32,
) -> Result<usize, i32> {
    let mut iovecs: Vec<libc::iovec> = bufs
        .iter()
        .map(|&(addr, len)| libc::iovec {
            iov_base: addr as *mut c_void,
            iov_len: len,
        })
        .collect();
    let mut pmsg: libc::msghdr = std::mem::zeroed();
    if let Some((storage, plen)) = to {
        pmsg.msg_name = storage as *const _ as *mut c_void;
        pmsg.msg_namelen = *plen;
    }
    pmsg.msg_iov = iovecs.as_mut_ptr();
    pmsg.msg_iovlen = iovecs.len() as libc::c_int;
    if !ctrl.is_empty() {
        pmsg.msg_control = ctrl.as_ptr() as *mut c_void;
        pmsg.msg_controllen = ctrl.len() as libc::socklen_t;
    }
    loop {
        let n = libc::sendmsg(fd, &pmsg, pflags | libc::MSG_DONTWAIT);
        if n >= 0 {
            return Ok(n as usize);
        }
        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if e == libc::EINTR {
            continue;
        }
        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
            return Err(0);
        }
        return Err(e);
    }
}

pub unsafe extern "win64" fn WSARecvMsg(
    s: usize,
    lp_msg: *mut WsaMsg,
    lpdw_number_of_bytes_recvd: *mut u32,
    lp_overlapped: *mut c_void,
    lp_completion_routine: *mut c_void,
) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if lp_msg.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let wmsg = std::ptr::read_unaligned(lp_msg);
    if (wmsg.control.len > 0 && wmsg.control.buf.is_null())
        || (wmsg.namelen > 0 && wmsg.name.is_null())
        || wmsg.namelen < 0
    {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let bufs = match collect_wsa_bufs(wmsg.lp_buffers, wmsg.dw_buffer_count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    let pflags = win_msg_flags_to_posix(wmsg.dw_flags);

    if lp_overlapped.is_null() {
        let deadline = socket_timeout_duration(meta.fd, libc::SO_RCVTIMEO).map(|d| std::time::Instant::now() + d);
        loop {
            match try_wsa_recvmsg_once(meta.fd, is_stream, lp_msg, &bufs, pflags) {
                Ok((n, nt_status, _wsa_flags)) => {
                    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                    if !lpdw_number_of_bytes_recvd.is_null() {
                        std::ptr::write_unaligned(lpdw_number_of_bytes_recvd, n as u32);
                    }
                    if !is_io_success_status(nt_status) {
                        return err(ntstatus_to_wsa(nt_status), SOCKET_ERROR);
                    }
                    return 0;
                }
                Err(0) if !is_guest_nonblocking(s) => {
                    if let Err(we) = wait_socket_ready_until(meta.fd, libc::POLLIN, deadline) {
                        event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                        return err(errno_to_wsa(we), SOCKET_ERROR);
                    }
                }
                Err(0) => {
                    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                    return err(WSAEWOULDBLOCK, SOCKET_ERROR);
                }
                Err(e) => {
                    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);
                    return err(errno_to_wsa(e), SOCKET_ERROR);
                }
            }
        }
    }

    let ov = lp_overlapped as *mut OVERLAPPED;
    let issuing_tid = crate::kernel32::threads::GetCurrentThreadId();
    let issuing_thread = if !lp_completion_routine.is_null() {
        Some(crate::thread::ensure_current_thread_object())
    } else {
        crate::thread::current_thread_object()
    };
    let completion_binding = lookup_io_completion_binding(s as HANDLE);

    let immediate = {
        let mut st = ASYNC_IO.inner.lock().unwrap();
        if has_prior_recv(&st.ops, s) {
            prepare_socket_pending(s, ov, lp_completion_routine as usize);
            ensure_async_io_started(&mut st);
            st.ops.push(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: meta.fd,
                poll_events: libc::POLLIN,
                overlapped: ov as usize,
                issuing_tid,
                issuing_thread: issuing_thread.clone(),
                completion_routine: lp_completion_routine as usize,
                completion_binding: completion_binding.clone(),
                kind: AsyncOpKind::RecvMsg {
                    lp_msg: lp_msg as usize,
                    bufs,
                    pflags,
                    is_stream,
                },
            });
            wake_fd_write(st.wake_w);
            None
        } else {
            match try_wsa_recvmsg_once(meta.fd, is_stream, lp_msg, &bufs, pflags) {
                Ok(res) => Some(Ok(res)),
                Err(0) => {
                    prepare_socket_pending(s, ov, lp_completion_routine as usize);
                    ensure_async_io_started(&mut st);
                    st.ops.push(PendingSocketOp {
                        socket_handle: s,
                        secondary_socket: None,
                        poll_fd: meta.fd,
                        poll_events: libc::POLLIN,
                        overlapped: ov as usize,
                        issuing_tid,
                        issuing_thread: issuing_thread.clone(),
                        completion_routine: lp_completion_routine as usize,
                        completion_binding: completion_binding.clone(),
                        kind: AsyncOpKind::RecvMsg {
                            lp_msg: lp_msg as usize,
                            bufs,
                            pflags,
                            is_stream,
                        },
                    });
                    wake_fd_write(st.wake_w);
                    None
                }
                Err(e) => Some(Err(e)),
            }
        }
    };

    event_select_rearm_read(s, (pflags & libc::MSG_OOB) != 0);

    match immediate {
        Some(Ok((n, nt_status, wsa_flags))) => {
            if !lpdw_number_of_bytes_recvd.is_null() {
                std::ptr::write_unaligned(lpdw_number_of_bytes_recvd, n as u32);
            }
            if !is_io_success_status(nt_status) {
                return err(ntstatus_to_wsa(nt_status), SOCKET_ERROR);
            }
            finish_overlapped_op(
                s,
                ov as usize,
                lp_completion_routine as usize,
                completion_binding.as_deref(),
                issuing_thread.as_ref(),
                issuing_tid,
                nt_status,
                n,
                wsa_flags,
                true,
            );
            0
        }
        Some(Err(e)) => err(errno_to_wsa(e), SOCKET_ERROR),
        None => err(WSA_IO_PENDING, SOCKET_ERROR),
    }
}

pub unsafe extern "win64" fn WSASendMsg(
    s: usize,
    lp_msg: *const WsaMsg,
    dw_flags: u32,
    lp_number_of_bytes_sent: *mut u32,
    lp_overlapped: *mut c_void,
    lp_completion_routine: *mut c_void,
) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if lp_msg.is_null() {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let wmsg = std::ptr::read_unaligned(lp_msg);
    if (wmsg.control.len > 0 && wmsg.control.buf.is_null())
        || (wmsg.namelen > 0 && wmsg.name.is_null())
        || wmsg.namelen < 0
    {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let bufs = match collect_wsa_bufs(wmsg.lp_buffers, wmsg.dw_buffer_count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let to = if !wmsg.name.is_null() && wmsg.namelen > 0 {
        match win_to_posix_sockaddr(wmsg.name, wmsg.namelen) {
            Ok(v) => Some(v),
            Err(e) => return err(e, SOCKET_ERROR),
        }
    } else if !meta.connected {
        meta.mcast_dest
    } else {
        None
    };
    let ctrl = translate_win_cmsg_to_posix(wmsg.control.buf, wmsg.control.len as usize);
    let pflags = win_msg_flags_to_posix(dw_flags | wmsg.dw_flags);

    if lp_overlapped.is_null() {
        let deadline = socket_timeout_duration(meta.fd, libc::SO_SNDTIMEO).map(|d| std::time::Instant::now() + d);
        loop {
            match try_wsa_sendmsg_once(meta.fd, &bufs, to.as_ref(), &ctrl, pflags) {
                Ok(n) => {
                    if !lp_number_of_bytes_sent.is_null() {
                        std::ptr::write_unaligned(lp_number_of_bytes_sent, n as u32);
                    }
                    return 0;
                }
                Err(0) if !is_guest_nonblocking(s) => {
                    if let Err(we) = wait_socket_ready_until(meta.fd, libc::POLLOUT, deadline) {
                        return err(errno_to_wsa(we), SOCKET_ERROR);
                    }
                }
                Err(0) => {
                    event_select_rearm_write(s);
                    return err(WSAEWOULDBLOCK, SOCKET_ERROR);
                }
                Err(e) => return err(errno_to_wsa(e), SOCKET_ERROR),
            }
        }
    }

    let ov = lp_overlapped as *mut OVERLAPPED;
    let issuing_tid = crate::kernel32::threads::GetCurrentThreadId();
    let issuing_thread = if !lp_completion_routine.is_null() {
        Some(crate::thread::ensure_current_thread_object())
    } else {
        crate::thread::current_thread_object()
    };
    let completion_binding = lookup_io_completion_binding(s as HANDLE);

    match try_wsa_sendmsg_once(meta.fd, &bufs, to.as_ref(), &ctrl, pflags) {
        Ok(n) => {
            if !lp_number_of_bytes_sent.is_null() {
                std::ptr::write_unaligned(lp_number_of_bytes_sent, n as u32);
            }
            finish_overlapped_op(
                s,
                ov as usize,
                lp_completion_routine as usize,
                completion_binding.as_deref(),
                issuing_thread.as_ref(),
                issuing_tid,
                STATUS_SUCCESS,
                n,
                0,
                true,
            );
            0
        }
        Err(0) => {
            prepare_socket_pending(s, ov, lp_completion_routine as usize);
            async_io_enqueue(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: meta.fd,
                poll_events: libc::POLLOUT,
                overlapped: ov as usize,
                issuing_tid,
                issuing_thread,
                completion_routine: lp_completion_routine as usize,
                completion_binding,
                kind: AsyncOpKind::SendMsg {
                    bufs,
                    to,
                    ctrl,
                    pflags,
                },
            });
            err(WSA_IO_PENDING, SOCKET_ERROR)
        }
        Err(e) => err(errno_to_wsa(e), SOCKET_ERROR),
    }
}

/// Outstanding async Winsock 1.1 lookup task registry for `WSACancelAsyncRequest`.
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum AsyncTaskState {
    Pending,
    Completed,
}

static ASYNC_TASKS: LazyLock<Mutex<HashMap<usize, AsyncTaskState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub unsafe extern "win64" fn WSACancelAsyncRequest(h_async_task_handle: HANDLE) -> i32 {
    let key = h_async_task_handle as usize;
    if key == 0 {
        return err(WSAEINVAL, SOCKET_ERROR);
    }
    let mut tasks = ASYNC_TASKS.lock().unwrap();
    match tasks.remove(&key) {
        Some(AsyncTaskState::Pending) => 0,
        Some(AsyncTaskState::Completed) => err(WSAEALREADY, SOCKET_ERROR),
        None => err(WSAEINVAL, SOCKET_ERROR),
    }
}

/// Shared implementation for `WSAConnectByNameA` and `WSAConnectByNameW`.
#[allow(clippy::too_many_arguments)]
unsafe fn connect_by_name_impl(
    s: usize,
    node: Option<String>,
    service: Option<String>,
    local_address_length: *mut u32,
    local_address: *mut u8,
    remote_address_length: *mut u32,
    remote_address: *mut u8,
    timeout: *const WinTimeVal,
) -> BOOL {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, FALSE);
    };
    if (!local_address.is_null() && local_address_length.is_null())
        || (!remote_address.is_null() && remote_address_length.is_null())
    {
        return err(WSAEFAULT, FALSE);
    }
    let min_sa_len: Option<u32> = match meta.family {
        libc::AF_INET => Some(16),
        libc::AF_INET6 => Some(28),
        _ => None,
    };
    if let Some(need) = min_sa_len {
        if !local_address.is_null() {
            let avail = std::ptr::read_unaligned(local_address_length);
            if avail < need {
                std::ptr::write_unaligned(local_address_length, need);
                return err(WSAEFAULT, FALSE);
            }
        }
        if !remote_address.is_null() {
            let avail = std::ptr::read_unaligned(remote_address_length);
            if avail < need {
                std::ptr::write_unaligned(remote_address_length, need);
                return err(WSAEFAULT, FALSE);
            }
        }
    }
    let node_str = match node {
        Some(v) if !v.is_empty() => v,
        _ => return err(WSAEINVAL, FALSE),
    };
    let svc_str = match service {
        Some(v) if !v.is_empty() => v,
        _ => return err(WSAEINVAL, FALSE),
    };
    let Ok(c_node) = CString::new(node_str) else {
        return err(WSAEINVAL, FALSE);
    };
    let Ok(c_svc) = CString::new(svc_str) else {
        return err(WSAEINVAL, FALSE);
    };

    let deadline = if !timeout.is_null() {
        let tv = std::ptr::read_unaligned(timeout);
        if tv.tv_sec < 0 || tv.tv_usec < 0 {
            return err(WSAEINVAL, FALSE);
        }
        Some(
            std::time::Instant::now()
                + std::time::Duration::new(tv.tv_sec as u64, (tv.tv_usec as u32).saturating_mul(1000)),
        )
    } else {
        None
    };

    // Check dual-stack IPV6_V6ONLY=0 so IPv6 sockets can connect to IPv4 destinations via ::ffff:.
    let dual_stack_v6 = if meta.family == libc::AF_INET6 {
        let mut v6only: i32 = 1;
        let mut l = std::mem::size_of::<i32>() as libc::socklen_t;
        libc::getsockopt(
            meta.fd,
            libc::IPPROTO_IPV6,
            libc::IPV6_V6ONLY,
            &mut v6only as *mut i32 as *mut c_void,
            &mut l,
        ) == 0
            && v6only == 0
    } else {
        false
    };

    let mut hints: libc::addrinfo = std::mem::zeroed();
    hints.ai_family = if dual_stack_v6 { libc::AF_UNSPEC } else { meta.family };
    hints.ai_socktype = meta.sock_type;
    hints.ai_protocol = meta.protocol;

    let mut raw: *mut libc::addrinfo = std::ptr::null_mut();
    let rc = libc::getaddrinfo(c_node.as_ptr(), c_svc.as_ptr(), &hints, &mut raw);
    if rc != 0 || raw.is_null() {
        return err(WSAHOST_NOT_FOUND, FALSE);
    }

    let mut last_wsa = WSAHOST_NOT_FOUND;
    let mut connected_sa: Option<(libc::sockaddr_storage, libc::socklen_t)> = None;
    let mut cur = raw;
    while !cur.is_null() {
        let ai = &*cur;
        cur = ai.ai_next;
        if ai.ai_addr.is_null() {
            continue;
        }
        let mut target_sa: libc::sockaddr_storage = std::mem::zeroed();
        let target_len: libc::socklen_t;
        if ai.ai_family == meta.family {
            let copy_len = (ai.ai_addrlen as usize).min(std::mem::size_of::<libc::sockaddr_storage>());
            std::ptr::copy_nonoverlapping(
                ai.ai_addr as *const u8,
                &mut target_sa as *mut _ as *mut u8,
                copy_len,
            );
            target_len = ai.ai_addrlen;
        } else if dual_stack_v6 && ai.ai_family == libc::AF_INET {
            let sin = &*(ai.ai_addr as *const libc::sockaddr_in);
            let mut sin6: libc::sockaddr_in6 = std::mem::zeroed();
            sin6.sin6_len = std::mem::size_of::<libc::sockaddr_in6>() as u8;
            sin6.sin6_family = libc::AF_INET6 as libc::sa_family_t;
            sin6.sin6_port = sin.sin_port;
            sin6.sin6_addr.s6_addr[10] = 0xFF;
            sin6.sin6_addr.s6_addr[11] = 0xFF;
            sin6.sin6_addr.s6_addr[12..16].copy_from_slice(&sin.sin_addr.s_addr.to_ne_bytes());
            std::ptr::copy_nonoverlapping(
                &sin6 as *const libc::sockaddr_in6 as *const u8,
                &mut target_sa as *mut _ as *mut u8,
                std::mem::size_of::<libc::sockaddr_in6>(),
            );
            target_len = std::mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t;
        } else {
            continue;
        }

        let cres = libc::connect(
            meta.fd,
            &target_sa as *const libc::sockaddr_storage as *const libc::sockaddr,
            target_len,
        );
        if cres == 0 {
            connected_sa = Some((target_sa, target_len));
            break;
        }
        let raw_e = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        if raw_e == libc::EINPROGRESS {
            match wait_socket_ready_until(meta.fd, libc::POLLOUT, deadline) {
                Ok(()) => {
                    let so_err = query_so_error(meta.fd);
                    if so_err == 0 {
                        connected_sa = Some((target_sa, target_len));
                        break;
                    }
                    last_wsa = errno_to_wsa(so_err);
                }
                Err(we) => {
                    last_wsa = errno_to_wsa(we);
                    if we == libc::ETIMEDOUT {
                        break;
                    }
                }
            }
        } else {
            last_wsa = errno_to_wsa(raw_e);
        }
    }
    libc::freeaddrinfo(raw);

    let Some((remote_sa, remote_len)) = connected_sa else {
        event_select_on_connect_complete(s, last_wsa as i32);
        return err(last_wsa, FALSE);
    };

    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.bound = true;
        m.connecting = false;
        m.connected = true;
    }
    event_select_on_connect_complete(s, 0);

    if !local_address.is_null() {
        let mut lsa: libc::sockaddr_storage = std::mem::zeroed();
        let mut llen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        if libc::getsockname(meta.fd, &mut lsa as *mut _ as *mut libc::sockaddr, &mut llen) != 0 {
            return err(last_errno_wsa(), FALSE);
        }
        let mut avail = std::ptr::read_unaligned(local_address_length) as i32;
        let res = posix_to_win_sockaddr(
            &lsa as *const _ as *const libc::sockaddr,
            llen,
            local_address,
            &mut avail,
        );
        std::ptr::write_unaligned(local_address_length, avail.max(0) as u32);
        if let Err(wsa) = res {
            return err(wsa, FALSE);
        }
    }
    if !remote_address.is_null() {
        let mut avail = std::ptr::read_unaligned(remote_address_length) as i32;
        let res = posix_to_win_sockaddr(
            &remote_sa as *const _ as *const libc::sockaddr,
            remote_len,
            remote_address,
            &mut avail,
        );
        std::ptr::write_unaligned(remote_address_length, avail.max(0) as u32);
        if let Err(wsa) = res {
            return err(wsa, FALSE);
        }
    }
    TRUE
}

pub unsafe extern "win64" fn WSAConnectByNameA(
    s: usize,
    nodename: LPCSTR,
    servicename: LPCSTR,
    local_address_length: *mut u32,
    local_address: *mut u8,
    remote_address_length: *mut u32,
    remote_address: *mut u8,
    timeout: *const WinTimeVal,
    _reserved: *mut c_void,
) -> BOOL {
    let n = crate::strings::from_ansi_opt(nodename);
    let svc = crate::strings::from_ansi_opt(servicename);
    connect_by_name_impl(
        s,
        n,
        svc,
        local_address_length,
        local_address,
        remote_address_length,
        remote_address,
        timeout,
    )
}

pub unsafe extern "win64" fn WSAConnectByNameW(
    s: usize,
    nodename: LPCWSTR,
    servicename: LPCWSTR,
    local_address_length: *mut u32,
    local_address: *mut u8,
    remote_address_length: *mut u32,
    remote_address: *mut u8,
    timeout: *const WinTimeVal,
    _reserved: *mut c_void,
) -> BOOL {
    let n = (!nodename.is_null()).then(|| crate::strings::from_wide(nodename));
    let svc = (!servicename.is_null()).then(|| crate::strings::from_wide(servicename));
    connect_by_name_impl(
        s,
        n,
        svc,
        local_address_length,
        local_address,
        remote_address_length,
        remote_address,
        timeout,
    )
}

pub unsafe extern "win64" fn WSAGetQOSByName(
    s: usize,
    lp_qos_name: *mut WsaBuf,
    lp_qos: *mut c_void,
) -> BOOL {
    if lookup_fd(s).is_none() {
        return err(WSAENOTSOCK, FALSE);
    }
    if lp_qos_name.is_null() || lp_qos.is_null() {
        return err(WSAEFAULT, FALSE);
    }
    // Native-unsupported: RSVP / Winsock2 QOS templates were deprecated in Windows Vista and
    // are not supported by the standard Windows TCP/IP provider (`XP1_QOS_SUPPORTED` is unset).
    // Matches Microsoft Learn `WSAGetQOSByName` returning `FALSE` with `WSAEOPNOTSUPP`.
    log::debug!("WSAGetQOSByName: QOS templates unsupported by TCP/IP provider -> WSAEOPNOTSUPP");
    err(WSAEOPNOTSUPP, FALSE)
}

const JL_SENDER_ONLY: u32 = 0x01;
const JL_RECEIVER_ONLY: u32 = 0x02;
const JL_BOTH: u32 = 0x04;

pub unsafe extern "win64" fn WSAJoinLeaf(
    s: usize,
    name: *const u8,
    namelen: i32,
    _lp_caller_data: *mut c_void,
    _lp_callee_data: *mut c_void,
    _lp_sqos: *mut c_void,
    _lp_gqos: *mut c_void,
    dw_flags: u32,
) -> usize {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, INVALID_SOCKET);
    };
    if name.is_null() {
        return err(WSAEFAULT, INVALID_SOCKET);
    }
    if dw_flags == 0 || (dw_flags & !(JL_SENDER_ONLY | JL_RECEIVER_ONLY | JL_BOTH)) != 0 {
        return err(WSAEINVAL, INVALID_SOCKET);
    }
    let (storage, plen) = match win_to_posix_sockaddr(name, namelen) {
        Ok(v) => v,
        Err(e) => return err(e, INVALID_SOCKET),
    };
    let join_recv = (dw_flags & (JL_RECEIVER_ONLY | JL_BOTH)) != 0;
    let join_send = (dw_flags & (JL_SENDER_ONLY | JL_BOTH)) != 0;

    match storage.ss_family as i32 {
        libc::AF_INET => {
            let sin = &*(&storage as *const libc::sockaddr_storage as *const libc::sockaddr_in);
            if join_recv {
                let mut iface_addr = libc::in_addr { s_addr: 0 };
                let mut local_sa: libc::sockaddr_storage = std::mem::zeroed();
                let mut local_len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
                if libc::getsockname(
                    meta.fd,
                    &mut local_sa as *mut _ as *mut libc::sockaddr,
                    &mut local_len,
                ) == 0
                    && local_sa.ss_family as i32 == libc::AF_INET
                {
                    let lsin = &*(&local_sa as *const _ as *const libc::sockaddr_in);
                    iface_addr = lsin.sin_addr;
                }
                let mreq = libc::ip_mreq {
                    imr_multiaddr: sin.sin_addr,
                    imr_interface: iface_addr,
                };
                if libc::setsockopt(
                    meta.fd,
                    libc::IPPROTO_IP,
                    libc::IP_ADD_MEMBERSHIP,
                    &mreq as *const libc::ip_mreq as *const c_void,
                    std::mem::size_of::<libc::ip_mreq>() as libc::socklen_t,
                ) != 0
                {
                    return err(last_errno_wsa(), INVALID_SOCKET);
                }
            }
            if join_send && sin.sin_port != 0 {
                // Keep the native UDP socket unconnected on Darwin so bidirectional (`JL_BOTH`)
                // sockets still receive multicast datagrams from unicast source addresses; store
                // the group destination on `SocketMeta` for default `send` / `WSASend` calls.
                if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
                    m.mcast_dest = Some((storage, plen));
                }
            }
            s
        }
        libc::AF_INET6 => {
            let sin6 = &*(&storage as *const libc::sockaddr_storage as *const libc::sockaddr_in6);
            if join_recv {
                let mreq6 = libc::ipv6_mreq {
                    ipv6mr_multiaddr: sin6.sin6_addr,
                    ipv6mr_interface: sin6.sin6_scope_id,
                };
                if libc::setsockopt(
                    meta.fd,
                    libc::IPPROTO_IPV6,
                    libc::IPV6_JOIN_GROUP,
                    &mreq6 as *const libc::ipv6_mreq as *const c_void,
                    std::mem::size_of::<libc::ipv6_mreq>() as libc::socklen_t,
                ) != 0
                {
                    return err(last_errno_wsa(), INVALID_SOCKET);
                }
            }
            if join_send && sin6.sin6_port != 0 {
                if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
                    m.mcast_dest = Some((storage, plen));
                }
            }
            s
        }
        _ => err(WSAEAFNOSUPPORT, INVALID_SOCKET),
    }
}

pub unsafe extern "win64" fn WSAAsyncSelect(s: usize, hwnd: HWND, wmsg: u32, l_event: i32) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let hwnd_val = hwnd as usize;
    if hwnd_val == 0 || crate::user32::window::get(hwnd_val).is_none() {
        return err(WSAEINVAL, SOCKET_ERROR);
    }
    // Microsoft Learn: WSAAsyncSelect automatically sets the socket to nonblocking mode,
    // regardless of the value of lEvent.
    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.guest_nonblocking = true;
    }
    if l_event == 0 {
        let mut st = EVENT_SELECT.inner.lock().unwrap();
        if st.entries.remove(&s).is_some() {
            let ww = st.wake_w;
            wake_fd_write(ww);
        }
        return 0;
    }
    let is_stream = meta.sock_type == libc::SOCK_STREAM;
    let write_armed = (l_event & FD_WRITE) != 0 && (!is_stream || meta.connected);
    {
        let mut st = EVENT_SELECT.inner.lock().unwrap();
        ensure_event_poller_started(&mut st);
        st.entries.insert(
            s,
            EventSelectEntry {
                fd: meta.fd,
                is_stream,
                event: 0,
                hwnd: hwnd_val,
                wmsg,
                mask: l_event,
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
    hwnd: usize,
    wmsg: u32,
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
    fn drain_async_select_messages(&mut self, sock: usize, out: &mut Vec<(usize, u32, usize, isize)>) {
        if self.hwnd == 0 || self.pending_events == 0 {
            return;
        }
        for bit in 0..FD_MAX_EVENTS {
            let flag = 1i32 << bit;
            if (self.pending_events & flag) != 0 {
                let err_code = (self.pending_errors[bit] as u32) & 0xFFFF;
                let ev_word = (flag as u32) & 0xFFFF;
                let lparam = ((err_code << 16) | ev_word) as isize;
                out.push((self.hwnd, self.wmsg, sock, lparam));
                self.pending_errors[bit] = 0;
            }
        }
        self.pending_events = 0;
    }
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
        let stream_rx_closed = self.is_stream && self.close_fired;
        let mut ev = 0i16;
        if !stream_rx_closed
            && !self.read_fired
            && ((self.mask & FD_READ) != 0 || (self.is_stream && (self.mask & FD_CLOSE) != 0))
        {
            ev |= libc::POLLIN;
        }
        if (self.mask & FD_WRITE) != 0 && self.write_armed {
            ev |= libc::POLLOUT;
        }
        if !stream_rx_closed && (self.mask & FD_OOB) != 0 && !self.oob_fired {
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
        entry.oob_fired = true;
        entry.accept_fired = true;
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

    if !(entry.is_stream && entry.close_fired)
        && (entry.mask & FD_OOB) != 0
        && !entry.oob_fired
        && (revents & libc::POLLPRI) != 0
    {
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
        } else if !entry.close_fired {
            let mut avail: i32 = 0;
            let _ = libc::ioctl(entry.fd, libc::FIONREAD, &mut avail);
            if avail > 0 {
                if !entry.read_fired {
                    entry.read_fired = true;
                    if entry.record(FD_READ_BIT, 0) {
                        signal = true;
                    }
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
                    if !entry.read_fired {
                        entry.read_fired = true;
                        if entry.record(FD_READ_BIT, 0) {
                            signal = true;
                        }
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
    let mut to_post_msgs: Vec<(usize, u32, usize, isize)> = Vec::new();
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
        to_post_msgs.clear();
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
                    if sig {
                        if entry.event != 0 {
                            to_signal.push(entry.event);
                        }
                        if entry.hwnd != 0 {
                            entry.drain_async_select_messages(sock, &mut to_post_msgs);
                        }
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
        for &(hwnd, wmsg, sock, lparam) in &to_post_msgs {
            crate::user32::message::post_message(hwnd, wmsg, sock, lparam);
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
    let Some((event, hwnd, wmsg, mask)) = st.entries.get(&parent).map(|e| (e.event, e.hwnd, e.wmsg, e.mask)) else {
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
            hwnd,
            wmsg,
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
    let mut post_msgs: Vec<(usize, u32, usize, isize)> = Vec::new();
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
            if sig && entry.hwnd != 0 {
                entry.drain_async_select_messages(s, &mut post_msgs);
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
    for (hwnd, wmsg, sock, lparam) in post_msgs {
        crate::user32::message::post_message(hwnd, wmsg, sock, lparam);
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
        if wake && !(entry.is_stream && entry.close_fired) {
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
                hwnd: 0,
                wmsg: 0,
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
/// WSAID_WSARECVMSG: {f689d7c8-6f1f-436b-8a53-e54fe351c322}
const WSAID_WSARECVMSG: [u8; 16] = [
    0xc8, 0xd7, 0x89, 0xf6, 0x1f, 0x6f, 0x6b, 0x43, 0x8a, 0x53, 0xe5, 0x4f, 0xe3, 0x51, 0xc3, 0x22,
];
/// WSAID_WSASENDMSG: {a441e712-754f-43ca-84a7-0dee44cf606d}
const WSAID_WSASENDMSG: [u8; 16] = [
    0x12, 0xe7, 0x41, 0xa4, 0x4f, 0x75, 0xca, 0x43, 0x84, 0xa7, 0x0d, 0xee, 0x44, 0xcf, 0x60, 0x6d,
];

fn lookup_extension_kind(guid: &[u8; 16]) -> Option<crate::WsaExtension> {
    if guid == &WSAID_ACCEPTEX {
        Some(crate::WsaExtension::AcceptEx)
    } else if guid == &WSAID_CONNECTEX {
        Some(crate::WsaExtension::ConnectEx)
    } else if guid == &WSAID_DISCONNECTEX {
        Some(crate::WsaExtension::DisconnectEx)
    } else if guid == &WSAID_TRANSMITFILE {
        Some(crate::WsaExtension::TransmitFile)
    } else if guid == &WSAID_GETACCEPTEXSOCKADDRS {
        Some(crate::WsaExtension::GetAcceptExSockaddrs)
    } else if guid == &WSAID_WSARECVMSG {
        Some(crate::WsaExtension::RecvMsg)
    } else if guid == &WSAID_WSASENDMSG {
        Some(crate::WsaExtension::SendMsg)
    } else {
        None
    }
}

/// Fixed internal ABI-correct implementation address for a verified Winsock2 extension.
/// Used exclusively by the loader when provisioning immutable guest-visible thunks; never
/// returned directly to guest memory.
pub fn wsa_extension_target(ext: crate::WsaExtension) -> usize {
    match ext {
        crate::WsaExtension::AcceptEx => ext_accept_ex as *const () as usize,
        crate::WsaExtension::ConnectEx => ext_connect_ex as *const () as usize,
        crate::WsaExtension::DisconnectEx => ext_disconnect_ex as *const () as usize,
        crate::WsaExtension::TransmitFile => ext_transmit_file as *const () as usize,
        crate::WsaExtension::GetAcceptExSockaddrs => ext_get_accept_ex_sockaddrs as *const () as usize,
        crate::WsaExtension::RecvMsg => WSARecvMsg as *const () as usize,
        crate::WsaExtension::SendMsg => WSASendMsg as *const () as usize,
    }
}

/// Resolve a stable guest-visible thunk VA for `ext` via the loader (never returning 0 or a
/// raw host code pointer).
fn resolve_wsa_extension_thunk(ext: crate::WsaExtension) -> Option<usize> {
    crate::resolve_wsa_extension_thunk(ext, wsa_extension_target(ext))
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
        total_len: usize,
        recvd_so_far: usize,
        pflags: i32,
        is_stream: bool,
        from: usize,
        fromlen: usize,
        from_captured: Option<(libc::sockaddr_storage, libc::socklen_t)>,
        msg_flags: i32,
    },
    Send {
        bufs: Vec<(usize, usize)>,
        total_len: usize,
        sent_so_far: usize,
        is_stream: bool,
        pflags: i32,
        to: Option<(libc::sockaddr_storage, libc::socklen_t)>,
    },
    RecvMsg {
        lp_msg: usize,
        bufs: Vec<(usize, usize)>,
        pflags: i32,
        is_stream: bool,
    },
    SendMsg {
        bufs: Vec<(usize, usize)>,
        to: Option<(libc::sockaddr_storage, libc::socklen_t)>,
        ctrl: Vec<u8>,
        pflags: i32,
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
    completion_binding: Option<Arc<IoCompletionBinding>>,
    kind: AsyncOpKind,
}

fn is_ordered_stream_write(op: &PendingSocketOp) -> bool {
    match &op.kind {
        AsyncOpKind::Send { is_stream, .. } => *is_stream,
        AsyncOpKind::ConnectEx { .. } | AsyncOpKind::TransmitFile(_) => true,
        AsyncOpKind::AcceptEx { .. }
        | AsyncOpKind::AcceptExRecv { .. }
        | AsyncOpKind::Recv { .. }
        | AsyncOpKind::RecvMsg { .. }
        | AsyncOpKind::SendMsg { .. } => false,
    }
}

fn has_prior_stream_write(ops: &[PendingSocketOp], socket_handle: usize) -> bool {
    ops.iter()
        .any(|op| op.socket_handle == socket_handle && is_ordered_stream_write(op))
}

fn is_ordered_recv(op: &PendingSocketOp) -> bool {
    matches!(op.kind, AsyncOpKind::Recv { .. } | AsyncOpKind::RecvMsg { .. })
}

fn has_prior_recv(ops: &[PendingSocketOp], socket_handle: usize) -> bool {
    ops.iter()
        .any(|op| op.socket_handle == socket_handle && is_ordered_recv(op))
}

struct CompletedOp {
    socket_handle: usize,
    overlapped: usize,
    completion_routine: usize,
    completion_binding: Option<Arc<IoCompletionBinding>>,
    issuing_thread: Option<Arc<crate::thread::ThreadObj>>,
    issuing_tid: u32,
    nt_status: usize,
    bytes: usize,
    wsa_flags: u32,
}

enum OpStep {
    Pending,
    Complete { nt_status: usize, bytes: usize },
    CompleteWithFlags { nt_status: usize, bytes: usize, wsa_flags: u32 },
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

/// Per-`(socket_handle, overlapped_ptr)` completion flags for overlapped Winsock operations
/// (e.g., `MSG_CTRUNC` / `MSG_TRUNC` / `MSG_OOB` returned by `WSARecvMsg`) so both completion
/// routines and `WSAGetOverlappedResult` observe the exact receive flags rather than deriving
/// them solely from `NTSTATUS`.
static OVERLAPPED_RESULT_FLAGS: LazyLock<Mutex<HashMap<(usize, usize), u32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn store_ov_result_flags(socket_handle: usize, ov: usize, wsa_flags: u32) {
    if ov == 0 {
        return;
    }
    let mut map = OVERLAPPED_RESULT_FLAGS.lock().unwrap();
    if wsa_flags == 0 {
        map.remove(&(socket_handle, ov));
    } else {
        map.insert((socket_handle, ov), wsa_flags);
    }
}

fn load_ov_result_flags(socket_handle: usize, ov: usize) -> Option<u32> {
    OVERLAPPED_RESULT_FLAGS
        .lock()
        .unwrap()
        .get(&(socket_handle, ov))
        .copied()
}

fn clear_socket_ov_result_flags(socket_handle: usize) {
    OVERLAPPED_RESULT_FLAGS
        .lock()
        .unwrap()
        .retain(|&(s, _), _| s != socket_handle);
}

unsafe fn prepare_socket_pending(s: usize, ov: *mut OVERLAPPED, completion_routine: usize) {
    if ov.is_null() {
        return;
    }
    store_ov_result_flags(s, ov as usize, 0);
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

#[allow(clippy::too_many_arguments)]
unsafe fn finish_overlapped_op(
    socket_handle: usize,
    ov: usize,
    completion_routine: usize,
    completion_binding: Option<&IoCompletionBinding>,
    issuing_thread: Option<&Arc<crate::thread::ThreadObj>>,
    issuing_tid: u32,
    nt_status: usize,
    bytes: usize,
    wsa_flags: u32,
    synchronous: bool,
) {
    if ov == 0 {
        return;
    }
    store_ov_result_flags(socket_handle, ov, wsa_flags);
    let ov_ptr = ov as *mut OVERLAPPED;
    if completion_routine != 0 {
        store_ov_internal_high(ov_ptr, bytes);
        store_ov_internal(ov_ptr, nt_status);
        let wsa_err = ntstatus_to_wsa(nt_status);
        let target = issuing_thread
            .cloned()
            .or_else(|| crate::thread::find_by_tid(issuing_tid));
        if let Some(t) = target {
            crate::thread::queue_io_completion_apc_with_flags(
                &t,
                completion_routine,
                wsa_err,
                bytes as u32,
                ov,
                wsa_flags,
            );
        }
    } else if completion_binding.is_some() {
        complete_overlapped_binding(completion_binding, ov_ptr, nt_status, bytes, synchronous);
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
                    op.completion_binding.as_deref(),
                    op.issuing_thread.as_ref(),
                    op.issuing_tid,
                    STATUS_CANCELLED,
                    0,
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
                op.completion_binding.as_deref(),
                op.issuing_thread.as_ref(),
                op.issuing_tid,
                STATUS_CANCELLED,
                0,
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
            total_len,
            recvd_so_far,
            pflags,
            is_stream,
            from,
            fromlen,
            from_captured,
            msg_flags,
        } => match step_overlapped_recv(
            op.poll_fd,
            bufs,
            *total_len,
            recvd_so_far,
            *pflags,
            *is_stream,
            *from != 0,
            from_captured,
            msg_flags,
        ) {
            RecvStep::Complete {
                nt_status,
                bytes,
                from: got_from,
                ..
            } => {
                if let Some((storage, slen)) = got_from {
                    if *from != 0 && *fromlen != 0 {
                        let _ = posix_to_win_sockaddr(
                            &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
                            slen,
                            *from as *mut u8,
                            *fromlen as *mut i32,
                        );
                    }
                }
                OpStep::Complete { nt_status, bytes }
            }
            RecvStep::Pending => OpStep::Pending,
            RecvStep::Error(e) => OpStep::Complete {
                nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                bytes: 0,
            },
        },
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
        AsyncOpKind::RecvMsg {
            lp_msg,
            bufs,
            pflags,
            is_stream,
        } => match try_wsa_recvmsg_once(op.poll_fd, *is_stream, *lp_msg as *mut WsaMsg, bufs, *pflags) {
            Ok((bytes, nt_status, wsa_flags)) => OpStep::CompleteWithFlags {
                nt_status,
                bytes,
                wsa_flags,
            },
            Err(0) => OpStep::Pending,
            Err(e) => OpStep::Complete {
                nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                bytes: 0,
            },
        },
        AsyncOpKind::SendMsg {
            bufs,
            to,
            ctrl,
            pflags,
        } => match try_wsa_sendmsg_once(op.poll_fd, bufs, to.as_ref(), ctrl, *pflags) {
            Ok(bytes) => OpStep::Complete {
                nt_status: STATUS_SUCCESS,
                bytes,
            },
            Err(0) => OpStep::Pending,
            Err(e) => OpStep::Complete {
                nt_status: win32_to_ntstatus(errno_to_wsa(e)),
                bytes: 0,
            },
        },
    }
}

fn async_io_reactor_loop() {
    #[cfg(target_vendor = "apple")]
    const DARWIN_EVFILT_EXCEPT: i16 = -15;
    #[cfg(target_vendor = "apple")]
    const DARWIN_NOTE_OOB: u32 = 0x2;

    let mut pfds: Vec<libc::pollfd> = Vec::new();
    let mut completed: Vec<CompletedOp> = Vec::new();
    // On Darwin, `poll(2)` does not report TCP OOB via `POLLPRI`; a pollable `kqueue`
    // watching `EVFILT_EXCEPT` + `NOTE_OOB` wakes `poll(2)` cleanly whenever urgent data
    // arrives without letting ordinary `POLLIN` traffic mask or spin pending OOB receives.
    #[cfg(target_vendor = "apple")]
    let oob_kq: i32 = unsafe {
        let kq = libc::kqueue();
        if kq >= 0 {
            let fl = libc::fcntl(kq, libc::F_GETFD);
            if fl >= 0 {
                libc::fcntl(kq, libc::F_SETFD, fl | libc::FD_CLOEXEC);
            }
        }
        kq
    };
    #[cfg(target_vendor = "apple")]
    let mut oob_fds: Vec<i32> = Vec::new();
    #[cfg(target_vendor = "apple")]
    let mut prev_oob_fds: Vec<i32> = Vec::new();

    loop {
        pfds.clear();
        #[cfg(target_vendor = "apple")]
        oob_fds.clear();
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
            for (i, op) in st.ops.iter().enumerate() {
                if is_ordered_stream_write(op) && has_prior_stream_write(&st.ops[..i], op.socket_handle) {
                    continue;
                }
                if is_ordered_recv(op) && has_prior_recv(&st.ops[..i], op.socket_handle) {
                    continue;
                }
                if op.poll_fd >= 0 && op.poll_events != 0 {
                    #[cfg(target_vendor = "apple")]
                    if (op.poll_events & libc::POLLPRI) != 0 && !oob_fds.contains(&op.poll_fd) {
                        oob_fds.push(op.poll_fd);
                    }
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

        #[cfg(target_vendor = "apple")]
        let oob_kq_idx = if oob_kq >= 0 {
            for &old_fd in &prev_oob_fds {
                if !oob_fds.contains(&old_fd) {
                    let del = libc::kevent {
                        ident: old_fd as usize,
                        filter: DARWIN_EVFILT_EXCEPT,
                        flags: libc::EV_DELETE,
                        fflags: 0,
                        data: 0,
                        udata: std::ptr::null_mut(),
                    };
                    unsafe {
                        let _ = libc::kevent(oob_kq, &del, 1, std::ptr::null_mut(), 0, std::ptr::null());
                    }
                }
            }
            for &fd in &oob_fds {
                if !prev_oob_fds.contains(&fd) {
                    let add = libc::kevent {
                        ident: fd as usize,
                        filter: DARWIN_EVFILT_EXCEPT,
                        flags: libc::EV_ADD | libc::EV_CLEAR,
                        fflags: DARWIN_NOTE_OOB,
                        data: 0,
                        udata: std::ptr::null_mut(),
                    };
                    unsafe {
                        let _ = libc::kevent(oob_kq, &add, 1, std::ptr::null_mut(), 0, std::ptr::null());
                    }
                }
            }
            prev_oob_fds.clear();
            prev_oob_fds.extend_from_slice(&oob_fds);
            if !oob_fds.is_empty() {
                let idx = pfds.len();
                pfds.push(libc::pollfd {
                    fd: oob_kq,
                    events: libc::POLLIN,
                    revents: 0,
                });
                Some(idx)
            } else {
                None
            }
        } else {
            None
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

        #[cfg(target_vendor = "apple")]
        if let Some(kq_idx) = oob_kq_idx {
            if (pfds[kq_idx].revents & libc::POLLIN) != 0 {
                let zero_ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
                let mut evs: [libc::kevent; 16] = unsafe { std::mem::zeroed() };
                loop {
                    let n = unsafe {
                        libc::kevent(
                            oob_kq,
                            std::ptr::null(),
                            0,
                            evs.as_mut_ptr(),
                            evs.len() as libc::c_int,
                            &zero_ts,
                        )
                    };
                    if n <= 0 {
                        break;
                    }
                    for ev in &evs[..n as usize] {
                        let ready_fd = ev.ident as i32;
                        if let Some(p) = pfds[1..kq_idx].iter_mut().find(|p| p.fd == ready_fd) {
                            p.revents |= libc::POLLPRI;
                        }
                    }
                    if (n as usize) < evs.len() {
                        break;
                    }
                }
            }
        }

        completed.clear();
        {
            let mut st = ASYNC_IO.inner.lock().unwrap();
            let mut idx = 0;
            while idx < st.ops.len() {
                if is_ordered_stream_write(&st.ops[idx])
                    && has_prior_stream_write(&st.ops[..idx], st.ops[idx].socket_handle)
                {
                    idx += 1;
                    continue;
                }
                if is_ordered_recv(&st.ops[idx])
                    && has_prior_recv(&st.ops[..idx], st.ops[idx].socket_handle)
                {
                    idx += 1;
                    continue;
                }
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
                            completion_binding: op.completion_binding,
                            issuing_thread: op.issuing_thread,
                            issuing_tid: op.issuing_tid,
                            nt_status,
                            bytes,
                            wsa_flags: ntstatus_to_wsa_recv_flags(nt_status),
                        });
                    }
                    OpStep::CompleteWithFlags {
                        nt_status,
                        bytes,
                        wsa_flags,
                    } => {
                        let op = st.ops.remove(idx);
                        completed.push(CompletedOp {
                            socket_handle: op.socket_handle,
                            overlapped: op.overlapped,
                            completion_routine: op.completion_routine,
                            completion_binding: op.completion_binding,
                            issuing_thread: op.issuing_thread,
                            issuing_tid: op.issuing_tid,
                            nt_status,
                            bytes,
                            wsa_flags,
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
                        c.completion_binding.as_deref(),
                        c.issuing_thread.as_ref(),
                        c.issuing_tid,
                        c.nt_status,
                        c.bytes,
                        c.wsa_flags,
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
                        completion_binding: lookup_io_completion_binding(s_listen as HANDLE),
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
                completion_binding: lookup_io_completion_binding(s_listen as HANDLE),
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
                        completion_binding: lookup_io_completion_binding(s as HANDLE),
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
            completion_binding: lookup_io_completion_binding(s as HANDLE),
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

    let issuing_tid = crate::kernel32::threads::GetCurrentThreadId();
    let issuing_thread = crate::thread::current_thread_object();
    let completion_binding = lookup_io_completion_binding(s as HANDLE);
    let step_res = {
        let mut st = ASYNC_IO.inner.lock().unwrap();
        if meta.sock_type == libc::SOCK_STREAM && has_prior_stream_write(&st.ops, s) {
            prepare_socket_pending(s, ov, 0);
            ensure_async_io_started(&mut st);
            st.ops.push(PendingSocketOp {
                socket_handle: s,
                secondary_socket: None,
                poll_fd: meta.fd,
                poll_events: libc::POLLOUT,
                overlapped: ov as usize,
                issuing_tid,
                issuing_thread,
                completion_routine: 0,
                completion_binding: completion_binding.clone(),
                kind: AsyncOpKind::TransmitFile(state),
            });
            wake_fd_write(st.wake_w);
            Ok(None)
        } else {
            match state.step(meta.fd, true) {
                Ok(true) => Ok(Some((state.total_sent, state.flags))),
                Ok(false) => {
                    prepare_socket_pending(s, ov, 0);
                    ensure_async_io_started(&mut st);
                    st.ops.push(PendingSocketOp {
                        socket_handle: s,
                        secondary_socket: None,
                        poll_fd: meta.fd,
                        poll_events: libc::POLLOUT,
                        overlapped: ov as usize,
                        issuing_tid,
                        issuing_thread,
                        completion_routine: 0,
                        completion_binding: completion_binding.clone(),
                        kind: AsyncOpKind::TransmitFile(state),
                    });
                    wake_fd_write(st.wake_w);
                    Ok(None)
                }
                Err(e) => Err(e),
            }
        }
    };
    match step_res {
        Ok(Some((total, post_flags))) => {
            if let Err(wsa) = apply_transmit_post_flags(s, meta.fd, post_flags) {
                return err(wsa, FALSE);
            }
            complete_overlapped_binding(completion_binding.as_deref(), ov, STATUS_SUCCESS, total, true);
            TRUE
        }
        Ok(None) => err(WSA_IO_PENDING, FALSE),
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
        #[repr(align(8))]
        struct Aligned<const N: usize>([u8; N]);
        const GUARD: u8 = 0xA5;

        unsafe {
            for (family, want_len, text) in [(AF_INET as i32, 16i32, "192.168.1.2:8080"), (AF_INET6 as i32, 28i32, "[fe80::1%3]:8080")] {
                let want_bytes = want_len as usize;
                let w = crate::strings::to_wide_z(text);
                let mut a_in = text.as_bytes().to_vec();
                a_in.push(0);
                let need_units = w.len() as u32;
                let need_ansi = (text.len() + 1) as u32;

                // Odd-aligned length cell and sockaddr buffer for WSAStringToAddressW.
                let mut len_cell = Aligned([GUARD; 8]);
                let len_ptr = len_cell.0.as_mut_ptr().add(1) as *mut i32;
                assert_eq!((len_ptr as usize) % 4, 1);
                let mut sa_w = Aligned([GUARD; 32]);
                let sa_w_ptr = sa_w.0.as_mut_ptr().add(1);

                len_ptr.write_unaligned(want_len - 1);
                assert_eq!(WSAStringToAddressW(w.as_ptr(), family, std::ptr::null_mut(), sa_w_ptr, len_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                assert_eq!(len_ptr.read_unaligned(), want_len);
                assert_eq!(len_cell.0[0], GUARD);
                assert_eq!(&len_cell.0[5..], &[GUARD; 3]);
                assert!(sa_w.0.iter().all(|&b| b == GUARD));

                assert_eq!(WSAStringToAddressW(w.as_ptr(), family, std::ptr::null_mut(), sa_w_ptr, len_ptr), 0);
                assert_eq!(len_ptr.read_unaligned(), want_len);
                assert_eq!(len_cell.0[0], GUARD);
                assert_eq!(&len_cell.0[5..], &[GUARD; 3]);
                assert_eq!(sa_w.0[0], GUARD);
                assert!(sa_w.0[1 + want_bytes..].iter().all(|&b| b == GUARD));

                // Odd-aligned length cell and sockaddr buffer for WSAStringToAddressA.
                let mut sa_a = Aligned([GUARD; 32]);
                let sa_a_ptr = sa_a.0.as_mut_ptr().add(1);
                len_ptr.write_unaligned(want_len - 1);
                assert_eq!(WSAStringToAddressA(a_in.as_ptr(), family, std::ptr::null_mut(), sa_a_ptr, len_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                assert_eq!(len_ptr.read_unaligned(), want_len);
                assert!(sa_a.0.iter().all(|&b| b == GUARD));

                assert_eq!(WSAStringToAddressA(a_in.as_ptr(), family, std::ptr::null_mut(), sa_a_ptr, len_ptr), 0);
                assert_eq!(len_ptr.read_unaligned(), want_len);
                assert_eq!(sa_a.0[0], GUARD);
                assert!(sa_a.0[1 + want_bytes..].iter().all(|&b| b == GUARD));
                assert_eq!(&sa_a.0[1..1 + want_bytes], &sa_w.0[1..1 + want_bytes]);

                // Odd-aligned out_len and odd-aligned UTF-16 output buffer for WSAAddressToStringW.
                let mut ol_cell = Aligned([GUARD; 8]);
                let ol_ptr = ol_cell.0.as_mut_ptr().add(1) as *mut u32;
                assert_eq!((ol_ptr as usize) % 4, 1);

                ol_ptr.write_unaligned(0xDEAD_BEEF);
                assert_eq!(WSAAddressToStringW(sa_w_ptr, want_len as u32, std::ptr::null_mut(), std::ptr::null_mut(), ol_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                assert_eq!(ol_ptr.read_unaligned(), need_units);
                assert_eq!(ol_cell.0[0], GUARD);
                assert_eq!(&ol_cell.0[5..], &[GUARD; 3]);

                let mut out_w = Aligned([GUARD; 64]);
                let out_w_ptr = out_w.0.as_mut_ptr().add(1) as *mut u16;
                assert_eq!((out_w_ptr as usize) % 2, 1);
                ol_ptr.write_unaligned(need_units - 1);
                assert_eq!(WSAAddressToStringW(sa_w_ptr, want_len as u32, std::ptr::null_mut(), out_w_ptr, ol_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                assert_eq!(ol_ptr.read_unaligned(), need_units);
                assert!(out_w.0.iter().all(|&b| b == GUARD));

                assert_eq!(WSAAddressToStringW(sa_w_ptr, want_len as u32, std::ptr::null_mut(), out_w_ptr, ol_ptr), 0);
                assert_eq!(ol_ptr.read_unaligned(), need_units);
                assert_eq!(crate::strings::from_wide(out_w_ptr), text);
                assert_eq!(ol_cell.0[0], GUARD);
                assert_eq!(&ol_cell.0[5..], &[GUARD; 3]);
                assert_eq!(out_w.0[0], GUARD);
                assert!(out_w.0[1 + (need_units as usize) * 2..].iter().all(|&b| b == GUARD));

                // Odd-aligned out_len and ANSI output buffer for WSAAddressToStringA.
                ol_ptr.write_unaligned(0xDEAD_BEEF);
                assert_eq!(WSAAddressToStringA(sa_a_ptr, want_len as u32, std::ptr::null_mut(), std::ptr::null_mut(), ol_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                assert_eq!(ol_ptr.read_unaligned(), need_ansi);
                assert_eq!(ol_cell.0[0], GUARD);
                assert_eq!(&ol_cell.0[5..], &[GUARD; 3]);

                let mut out_a = Aligned([GUARD; 64]);
                let out_a_ptr = out_a.0.as_mut_ptr().add(1);
                ol_ptr.write_unaligned(need_ansi - 1);
                assert_eq!(WSAAddressToStringA(sa_a_ptr, want_len as u32, std::ptr::null_mut(), out_a_ptr, ol_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                assert_eq!(ol_ptr.read_unaligned(), need_ansi);
                assert!(out_a.0.iter().all(|&b| b == GUARD));

                assert_eq!(WSAAddressToStringA(sa_a_ptr, want_len as u32, std::ptr::null_mut(), out_a_ptr, ol_ptr), 0);
                assert_eq!(ol_ptr.read_unaligned(), need_ansi);
                assert_eq!(crate::strings::from_ansi(out_a_ptr), text);
                assert_eq!(ol_cell.0[0], GUARD);
                assert_eq!(&ol_cell.0[5..], &[GUARD; 3]);
                assert_eq!(out_a.0[0], GUARD);
                assert!(out_a.0[1 + need_ansi as usize..].iter().all(|&b| b == GUARD));
            }

            // Truncated sockaddrs (len 2 and 3) must fail cleanly without reading bytes 2..4.
            let mut ol_cell = Aligned([GUARD; 8]);
            let ol_ptr = ol_cell.0.as_mut_ptr().add(1) as *mut u32;
            ol_ptr.write_unaligned(64);
            let sa_len2 = AF_INET.to_ne_bytes();
            let sa_len3 = [AF_INET6.to_ne_bytes()[0], AF_INET6.to_ne_bytes()[1], 0u8];
            for (ptr, len) in [(sa_len2.as_ptr(), 2u32), (sa_len3.as_ptr(), 3u32)] {
                assert_eq!(WSAAddressToStringW(ptr, len, std::ptr::null_mut(), std::ptr::null_mut(), ol_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEINVAL as i32);
                assert_eq!(WSAAddressToStringA(ptr, len, std::ptr::null_mut(), std::ptr::null_mut(), ol_ptr), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEINVAL as i32);
                assert_eq!(ol_ptr.read_unaligned(), 64);
                assert_eq!(ol_cell.0[0], GUARD);
                assert_eq!(&ol_cell.0[5..], &[GUARD; 3]);
            }
            assert_eq!(WSAAddressToStringW(sa_len2.as_ptr(), 2, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
            assert_eq!(WSAAddressToStringA(sa_len2.as_ptr(), 2, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
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

    unsafe fn test_wsa_ext_guest_dispatcher(
        va: u64,
        rcx: u64,
        rdx: u64,
        r8: u64,
        r9: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
        arg8: u64,
    ) -> u64 {
        let va_usize = va as usize;
        for ext in crate::WsaExtension::ALL {
            if va_usize
                == crate::tests::TEST_WSA_EXT_THUNK_BASE
                    + (ext as usize) * crate::tests::TEST_WSA_EXT_THUNK_STRIDE
            {
                let target = wsa_extension_target(ext);
                let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
                    std::mem::transmute(target);
                return f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
            }
        }
        panic!("unexpected test guest dispatch target {va:#x}");
    }

    struct TestWsaExtThunksGuard {
        _ext: crate::tests::ScopedTestWsaExtGuard,
        _disp: crate::tests::ScopedGuestDispatcherGuard,
    }

    impl TestWsaExtThunksGuard {
        fn install() -> Self {
            let ext = crate::tests::enable_test_wsa_ext_thunks();
            let disp = crate::tests::install_test_guest_dispatcher(Some(
                test_wsa_ext_guest_dispatcher,
            ));
            Self { _ext: ext, _disp: disp }
        }
    }

    unsafe fn call_ext_accept_ex(
        thunk: usize,
        s_listen: usize,
        s_accept: usize,
        out_buf: *mut u8,
        recv_data_len: u32,
        local_addr_len: u32,
        remote_addr_len: u32,
        bytes_received: *mut u32,
        ov: *mut OVERLAPPED,
    ) -> BOOL {
        crate::call_guest8(
            thunk,
            s_listen as u64,
            s_accept as u64,
            out_buf as u64,
            recv_data_len as u64,
            local_addr_len as u64,
            remote_addr_len as u64,
            bytes_received as u64,
            ov as u64,
        ) as BOOL
    }

    unsafe fn call_ext_get_accept_ex_sockaddrs(
        thunk: usize,
        out_buf: *const u8,
        recv_data_len: u32,
        local_addr_len: u32,
        remote_addr_len: u32,
        local_sa: *mut *mut u8,
        local_sa_len: *mut i32,
        remote_sa: *mut *mut u8,
        remote_sa_len: *mut i32,
    ) {
        let _ = crate::call_guest8(
            thunk,
            out_buf as u64,
            recv_data_len as u64,
            local_addr_len as u64,
            remote_addr_len as u64,
            local_sa as u64,
            local_sa_len as u64,
            remote_sa as u64,
            remote_sa_len as u64,
        );
    }

    unsafe fn call_ext_connect_ex(
        thunk: usize,
        s: usize,
        name: *const u8,
        namelen: i32,
        send_buf: *const u8,
        send_len: u32,
        bytes_sent: *mut u32,
        ov: *mut OVERLAPPED,
    ) -> BOOL {
        crate::call_guest8(
            thunk,
            s as u64,
            name as u64,
            namelen as u64,
            send_buf as u64,
            send_len as u64,
            bytes_sent as u64,
            ov as u64,
            0,
        ) as BOOL
    }

    unsafe fn call_ext_disconnect_ex(
        thunk: usize,
        s: usize,
        ov: *mut OVERLAPPED,
        flags: u32,
        reserved: u32,
    ) -> BOOL {
        crate::call_guest8(
            thunk,
            s as u64,
            ov as u64,
            flags as u64,
            reserved as u64,
            0,
            0,
            0,
            0,
        ) as BOOL
    }

    unsafe fn call_ext_transmit_file(
        thunk: usize,
        s: usize,
        h_file: HANDLE,
        n_bytes: u32,
        n_bytes_per_send: u32,
        ov: *mut OVERLAPPED,
        buffers: *const c_void,
        flags: u32,
    ) -> BOOL {
        crate::call_guest8(
            thunk,
            s as u64,
            h_file as u64,
            n_bytes as u64,
            n_bytes_per_send as u64,
            ov as u64,
            buffers as u64,
            flags as u64,
            0,
        ) as BOOL
    }

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
            let s2 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(s, INVALID_SOCKET);
            assert_ne!(s2, INVALID_SOCKET);

            // Without a thunk provider, WSAIoctl must fail with WSAEOPNOTSUPP rather than
            // falling back to disclosing raw internal host code pointers.
            for guid in [
                &WSAID_ACCEPTEX,
                &WSAID_CONNECTEX,
                &WSAID_DISCONNECTEX,
                &WSAID_TRANSMITFILE,
                &WSAID_GETACCEPTEXSOCKADDRS,
            ] {
                assert_eq!(query_ext_ptr(s, guid), Err(WSAEOPNOTSUPP as i32));
            }

            // With thunk provisioning active, all 5 Windows extension GUIDs resolve to stable,
            // distinct guest-visible thunk tokens that never equal raw internal host pointers.
            {
                let _ext = TestWsaExtThunksGuard::install();
                let p_accept = query_ext_ptr(s, &WSAID_ACCEPTEX).unwrap();
                let p_connect = query_ext_ptr(s, &WSAID_CONNECTEX).unwrap();
                let p_disconnect = query_ext_ptr(s, &WSAID_DISCONNECTEX).unwrap();
                let p_transmit = query_ext_ptr(s, &WSAID_TRANSMITFILE).unwrap();
                let p_sockaddrs = query_ext_ptr(s, &WSAID_GETACCEPTEXSOCKADDRS).unwrap();

                let raw_targets = [
                    ext_accept_ex as *const () as usize,
                    ext_connect_ex as *const () as usize,
                    ext_disconnect_ex as *const () as usize,
                    ext_transmit_file as *const () as usize,
                    ext_get_accept_ex_sockaddrs as *const () as usize,
                ];
                let thunks = [p_accept, p_connect, p_disconnect, p_transmit, p_sockaddrs];
                for (&th, &raw) in thunks.iter().zip(raw_targets.iter()) {
                    assert_ne!(th, 0);
                    assert_ne!(th, raw, "WSAIoctl must never disclose raw host function pointer");
                }
                // All 5 thunks are pairwise distinct and stable across repeated queries/sockets.
                let unique: std::collections::HashSet<usize> = thunks.iter().copied().collect();
                assert_eq!(unique.len(), 5);
                assert_eq!(query_ext_ptr(s, &WSAID_ACCEPTEX).unwrap(), p_accept);
                assert_eq!(query_ext_ptr(s2, &WSAID_ACCEPTEX).unwrap(), p_accept);
                assert_eq!(query_ext_ptr(s2, &WSAID_CONNECTEX).unwrap(), p_connect);
                assert_eq!(query_ext_ptr(s2, &WSAID_DISCONNECTEX).unwrap(), p_disconnect);
                assert_eq!(query_ext_ptr(s2, &WSAID_TRANSMITFILE).unwrap(), p_transmit);
                assert_eq!(query_ext_ptr(s2, &WSAID_GETACCEPTEXSOCKADDRS).unwrap(), p_sockaddrs);

                // Unknown GUID -> WSAEOPNOTSUPP
                let bad_guid = [0xAAu8; 16];
                assert_eq!(query_ext_ptr(s, &bad_guid), Err(WSAEOPNOTSUPP as i32));
            }

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

            assert_eq!(closesocket(s2), 0);
            assert_eq!(closesocket(s), 0);
        }
    }

    #[test]
    fn accept_ex_and_connect_ex_zero_and_initial_payload_with_iocp() {
        use crate::kernel32::file::{CreateIoCompletionPort, GetQueuedCompletionStatus};
        use crate::kernel32::threads::CloseHandle;

        let _ext = TestWsaExtThunksGuard::install();
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

            let accept_ex = query_ext_ptr(srv, &WSAID_ACCEPTEX).unwrap();
            let get_sockaddrs = query_ext_ptr(srv, &WSAID_GETACCEPTEXSOCKADDRS).unwrap();
            let connect_ex = query_ext_ptr(srv, &WSAID_CONNECTEX).unwrap();
            assert_ne!(accept_ex, ext_accept_ex as *const () as usize);
            assert_ne!(get_sockaddrs, ext_get_accept_ex_sockaddrs as *const () as usize);
            assert_ne!(connect_ex, ext_connect_ex as *const () as usize);

            // Case 1: 0 initial payload
            let acc1 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(acc1, INVALID_SOCKET);
            let mut acc_buf1 = [0u8; 64];
            let mut ov_acc1: OVERLAPPED = std::mem::zeroed();
            let mut got1 = 0u32;
            let r_acc1 = call_ext_accept_ex(accept_ex, srv, acc1, acc_buf1.as_mut_ptr(), 0, 32, 32, &mut got1, &mut ov_acc1);
            assert_eq!(r_acc1, FALSE);
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let cli1 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli1, INVALID_SOCKET);
            assert_eq!(CreateIoCompletionPort(cli1 as HANDLE, iocp, 0x2222, 0), iocp);

            // ConnectEx requires bind() first -> WSAEINVAL before bind.
            let mut ov_cli1: OVERLAPPED = std::mem::zeroed();
            let mut sent1 = 0u32;
            assert_eq!(
                call_ext_connect_ex(connect_ex, cli1, srv_sa.as_ptr(), 16, std::ptr::null(), 0, &mut sent1, &mut ov_cli1),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSAEINVAL as i32);

            let bind_sa = loopback_sa(0);
            assert_eq!(bind(cli1, bind_sa.as_ptr(), 16), 0);
            let mut cli1_local = [0u8; 16];
            let mut cli1_llen = 16i32;
            assert_eq!(getsockname(cli1, cli1_local.as_mut_ptr(), &mut cli1_llen), 0);

            let r_cli1 = call_ext_connect_ex(connect_ex, cli1, srv_sa.as_ptr(), 16, std::ptr::null(), 0, &mut sent1, &mut ov_cli1);
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
            call_ext_get_accept_ex_sockaddrs(
                get_sockaddrs,
                acc_buf1.as_ptr(),
                0,
                32,
                32,
                &mut lsa,
                &mut lsa_len,
                &mut rsa,
                &mut rsa_len,
            );
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
                call_ext_accept_ex(accept_ex, srv, acc2, acc_buf2.as_mut_ptr(), 64, 32, 32, &mut got2, &mut ov_acc2),
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
            let r_cli2 = call_ext_connect_ex(
                connect_ex,
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

            call_ext_get_accept_ex_sockaddrs(
                get_sockaddrs,
                acc_buf2.as_ptr(),
                64,
                32,
                32,
                &mut lsa,
                &mut lsa_len,
                &mut rsa,
                &mut rsa_len,
            );
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

            let _ext = TestWsaExtThunksGuard::install();
            let transmit_file = query_ext_ptr(cli, &WSAID_TRANSMITFILE).unwrap();
            assert_ne!(transmit_file, ext_transmit_file as *const () as usize);

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
            let ok = call_ext_transmit_file(
                transmit_file,
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

            let _ext = TestWsaExtThunksGuard::install();
            let connect_ex = query_ext_ptr(cli, &WSAID_CONNECTEX).unwrap();
            let disconnect_ex = query_ext_ptr(cli, &WSAID_DISCONNECTEX).unwrap();
            assert_ne!(connect_ex, ext_connect_ex as *const () as usize);
            assert_ne!(disconnect_ex, ext_disconnect_ex as *const () as usize);

            // First connection
            let bind_sa = loopback_sa(0);
            assert_eq!(bind(cli, bind_sa.as_ptr(), 16), 0);
            let mut ov1: OVERLAPPED = std::mem::zeroed();
            let mut sent = 0u32;
            let rc1 = call_ext_connect_ex(connect_ex, cli, sa.as_ptr(), 16, std::ptr::null(), 0, &mut sent, &mut ov1);
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
            assert_eq!(call_ext_disconnect_ex(disconnect_ex, cli, &mut ov_disc, TF_REUSE_SOCKET, 0), TRUE);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov_disc as *mut OVERLAPPED);
            assert_eq!(closesocket(acc1), 0);

            // Re-bind and reuse the exact same socket handle for a second connection!
            assert_eq!(bind(cli, bind_sa.as_ptr(), 16), 0);
            let mut ov2: OVERLAPPED = std::mem::zeroed();
            let payload = b"reused!";
            let rc2 = call_ext_connect_ex(connect_ex, cli, sa.as_ptr(), 16, payload.as_ptr(), payload.len() as u32, &mut sent, &mut ov2);
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

            let _ext = TestWsaExtThunksGuard::install();
            let accept_ex = query_ext_ptr(srv, &WSAID_ACCEPTEX).unwrap();

            // 1. Cancel pending AcceptEx with CancelIoEx(srv, &ov_acc).
            let acc_slot = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut acc_buf = [0u8; 64];
            let mut ov_acc: OVERLAPPED = std::mem::zeroed();
            let mut got = 0u32;
            assert_eq!(
                call_ext_accept_ex(accept_ex, srv, acc_slot, acc_buf.as_mut_ptr(), 0, 32, 32, &mut got, &mut ov_acc),
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
        static APC_FLAGS: AtomicU32 = AtomicU32::new(u32::MAX);

        unsafe extern "win64" fn wsa_completion_cb(err_code: u32, bytes: u32, ov: usize, flags: u32) {
            APC_ERR.store(err_code, Ordering::Release);
            APC_BYTES.store(bytes, Ordering::Release);
            APC_OV.store(ov, Ordering::Release);
            APC_FLAGS.store(flags, Ordering::Release);
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
            assert_eq!(APC_FLAGS.load(Ordering::Acquire), 0);
            assert_eq!(&rbuf_mem[..payload.len()], payload);

            // Ignored hEvent must remain unsignaled!
            assert_eq!(WaitForSingleObject(ignored_ev, 0), crate::types::WAIT_TIMEOUT);

            // WSAGetOverlappedResult must read the odd-aligned OVERLAPPED safely.
            out_flags = 0xDEAD;
            assert_eq!(WSAGetOverlappedResult(acc, unaligned_ov, &mut xfer, FALSE, &mut out_flags), TRUE);
            assert_eq!(xfer as usize, payload.len());
            assert_eq!(out_flags, 0);

            // 2. Immediate WSASend on odd-aligned OVERLAPPED with completion routine also preserves ignored hEvent.
            APC_CALLS.store(0, Ordering::Release);
            APC_FLAGS.store(u32::MAX, Ordering::Release);
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
            assert_eq!(APC_FLAGS.load(Ordering::Acquire), 0);
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

    #[test]
    fn wsa_event_select_stream_eof_is_one_shot_and_stops_rx_polling() {
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

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli, INVALID_SOCKET);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);

            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);
            let acc_fd = lookup_fd(acc).unwrap();

            let acc_ev = WSACreateEvent();
            assert!(!acc_ev.is_null());
            assert_eq!(
                WSAEventSelect(acc, acc_ev, FD_READ | FD_WRITE | FD_OOB | FD_CLOSE),
                0
            );

            // Drain initial FD_WRITE so subsequent waits reflect receive/close transitions.
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 2000, FALSE), 0);
            let mut ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut ne), 0);
            assert_ne!(ne.l_network_events & FD_WRITE, 0);
            assert_eq!(ne.l_network_events & FD_CLOSE, 0);

            // Peer sends buffered data and half-closes (FIN) before receiver reads.
            let payload = b"before-fin";
            assert_eq!(
                send(cli, payload.as_ptr(), payload.len() as i32, 0),
                payload.len() as i32
            );
            assert_eq!(shutdown(cli, 1 /* SD_SEND */), 0);

            // Buffered data must be delivered via FD_READ before terminal FD_CLOSE.
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 2000, FALSE), 0);
            ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut ne), 0);
            assert_ne!(ne.l_network_events & FD_READ, 0);
            assert_eq!(ne.l_network_events & FD_CLOSE, 0);

            let mut buf = [0u8; 32];
            let got = recv(acc, buf.as_mut_ptr(), buf.len() as i32, 0);
            assert_eq!(got, payload.len() as i32);
            assert_eq!(&buf[..got as usize], payload);

            // After draining buffered data, terminal FD_CLOSE fires once.
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 2000, FALSE), 0);
            ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut ne), 0);
            assert_ne!(ne.l_network_events & FD_CLOSE, 0);
            assert_eq!(ne.i_error_code[FD_CLOSE_BIT], 0);

            // Verify stream rx polling (POLLIN and POLLPRI) has ceased and stays off after recv/rearm.
            {
                let st = EVENT_SELECT.inner.lock().unwrap();
                let entry = st.entries.get(&acc).unwrap();
                assert!(entry.close_fired);
                assert_eq!(entry.poll_events(), 0);
            }
            assert_eq!(recv(acc, buf.as_mut_ptr(), buf.len() as i32, 0), 0);
            {
                let st = EVENT_SELECT.inner.lock().unwrap();
                let entry = st.entries.get(&acc).unwrap();
                assert!(entry.close_fired);
                assert_eq!(entry.poll_events(), 0);
            }

            // Event remains reset and FD_CLOSE does not repeat.
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 80, FALSE), 258 /* WAIT_TIMEOUT */);
            ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut ne), 0);
            assert_eq!(ne.l_network_events, 0);

            // Half-close send path (FD_WRITE) still functions without re-firing FD_CLOSE.
            let reply = b"after-fin";
            assert_eq!(
                send(acc, reply.as_ptr(), reply.len() as i32, 0),
                reply.len() as i32
            );
            let mut cli_buf = [0u8; 32];
            let cli_got = recv(cli, cli_buf.as_mut_ptr(), cli_buf.len() as i32, 0);
            assert_eq!(cli_got, reply.len() as i32);
            assert_eq!(&cli_buf[..cli_got as usize], reply);

            event_select_rearm_write(acc);
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 2000, FALSE), 0);
            ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut ne), 0);
            assert_ne!(ne.l_network_events & FD_WRITE, 0);
            assert_eq!(ne.l_network_events & FD_CLOSE, 0);
            assert_eq!(ne.l_network_events & FD_READ, 0);

            // Direct state-machine checks:
            // 1. After close_fired on a stream socket, POLLIN/POLLPRI are never requested
            //    (including FD_OOB-only) and stale POLLIN/POLLHUP/POLLERR revents are idempotent.
            let mut stream_entry = EventSelectEntry {
                fd: acc_fd,
                is_stream: true,
                event: 0,
                hwnd: 0,
                wmsg: 0,
                mask: FD_READ | FD_WRITE | FD_OOB | FD_CLOSE,
                listening: false,
                connecting: false,
                connected: true,
                read_fired: false,
                write_armed: false,
                oob_fired: false,
                accept_fired: false,
                close_fired: true,
                pending_events: 0,
                pending_errors: [0; FD_MAX_EVENTS],
            };
            assert_eq!(stream_entry.poll_events(), 0);
            assert_eq!(
                process_poll_revents(&mut stream_entry, libc::POLLIN | libc::POLLHUP | libc::POLLERR | libc::POLLPRI),
                (false, None)
            );
            assert_eq!(stream_entry.pending_events, 0);

            // FD_OOB-only stream entry after close_fired must also stop requesting POLLPRI.
            stream_entry.mask = FD_OOB;
            assert_eq!(stream_entry.poll_events(), 0);

            // Half-close write interest still requests only POLLOUT; stale POLLHUP/POLLERR alongside
            // POLLOUT signals only FD_WRITE and never repeats FD_CLOSE.
            stream_entry.mask = FD_READ | FD_WRITE | FD_OOB | FD_CLOSE;
            stream_entry.write_armed = true;
            assert_eq!(stream_entry.poll_events(), libc::POLLOUT);
            assert_eq!(
                process_poll_revents(&mut stream_entry, libc::POLLOUT | libc::POLLHUP | libc::POLLERR),
                (true, None)
            );
            assert_eq!(stream_entry.pending_events, FD_WRITE);
            assert!(!stream_entry.write_armed);
            assert_eq!(stream_entry.poll_events(), 0);

            // 2. Non-stream (UDP) entries retain POLLIN/POLLPRI interest unaffected by stream EOF rules.
            let udp_entry = EventSelectEntry {
                fd: acc_fd,
                is_stream: false,
                event: 0,
                hwnd: 0,
                wmsg: 0,
                mask: FD_READ | FD_OOB | FD_CLOSE,
                listening: false,
                connecting: false,
                connected: false,
                read_fired: false,
                write_armed: false,
                oob_fired: false,
                accept_fired: false,
                close_fired: false,
                pending_events: 0,
                pending_errors: [0; FD_MAX_EVENTS],
            };
            assert_eq!(udp_entry.poll_events(), libc::POLLIN | libc::POLLPRI);

            // 3. Re-subscribing with a fresh WSAEventSelect observes current EOF once and settles again.
            assert_eq!(WSAEventSelect(acc, acc_ev, FD_READ | FD_CLOSE), 0);
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 2000, FALSE), 0);
            ne = WsaNetworkEvents::default();
            assert_eq!(WSAEnumNetworkEvents(acc, acc_ev, &mut ne), 0);
            assert_ne!(ne.l_network_events & FD_CLOSE, 0);
            assert_eq!(WSAWaitForMultipleEvents(1, &acc_ev, FALSE, 80, FALSE), 258 /* WAIT_TIMEOUT */);

            assert_eq!(WSAEventSelect(acc, std::ptr::null_mut(), 0), 0);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(WSACloseEvent(acc_ev), TRUE);
        }
    }

    #[test]
    fn overlapped_tcp_wsasend_preserves_wire_order_under_backpressure() {
        use crate::kernel32::file::{
            CancelIoEx, CreateIoCompletionPort, GetQueuedCompletionStatus, INVALID_HANDLE_VALUE,
            OVERLAPPED,
        };
        use crate::kernel32::threads::CloseHandle;

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

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli, INVALID_SOCKET);
            let small_buf: i32 = 8192;
            let _ = setsockopt(cli, 0xFFFF, 0x1001 /* SO_SNDBUF */, &small_buf as *const i32 as *const u8, 4);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);

            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);
            let _ = setsockopt(acc, 0xFFFF, 0x1002 /* SO_RCVBUF */, &small_buf as *const i32 as *const u8, 4);

            // Second independent TCP pair to verify distinct sockets are not blocked by `cli`.
            let cli2 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli2, INVALID_SOCKET);
            assert_eq!(connect(cli2, sa.as_ptr(), 16), 0);
            let acc2 = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc2, INVALID_SOCKET);

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());
            assert_eq!(CreateIoCompletionPort(cli as HANDLE, iocp, 0x5111, 0), iocp);

            // Build three uniquely identifiable payloads so any interleaving or reordering is caught.
            let payload_a: Vec<u8> = (0..(256 * 1024))
                .map(|i| 0xA0u8.wrapping_add(((i / 251) & 0x0F) as u8))
                .collect();
            let payload_b: Vec<u8> = (0..(48 * 1024))
                .map(|i| 0xB0u8.wrapping_add(((i / 199) & 0x0F) as u8))
                .collect();
            let payload_c: Vec<u8> = (0..(32 * 1024))
                .map(|i| 0xC0u8.wrapping_add(((i / 131) & 0x0F) as u8))
                .collect();
            let mut expected = Vec::with_capacity(payload_a.len() + payload_b.len() + payload_c.len());
            expected.extend_from_slice(&payload_a);
            expected.extend_from_slice(&payload_b);
            expected.extend_from_slice(&payload_c);

            // 1. Submit A (large enough to partially write and pend under backpressure).
            let mut ov_a: OVERLAPPED = std::mem::zeroed();
            let wb_a = WsaBuf { len: payload_a.len() as u32, buf: payload_a.as_ptr() as *mut u8 };
            let mut sent_a = 0u32;
            assert_eq!(
                WSASend(cli, &wb_a, 1, &mut sent_a, 0, &mut ov_a as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            // 2. Drain a small chunk on `acc` to create send-buffer space right before submitting B
            // (WSASend) and C (WSASendTo). Even with newly freed buffer space, B and C MUST queue
            // behind A's unfinished remainder instead of bypassing A on the immediate path.
            let mut initial_drained = Vec::new();
            let mut chunk = [0u8; 4096];
            let tmo: u32 = 100;
            let _ = setsockopt(acc, 0xFFFF, 0x1006 /* SO_RCVTIMEO */, &tmo as *const u32 as *const u8, 4);
            let n0 = recv(acc, chunk.as_mut_ptr(), chunk.len() as i32, 0);
            assert!(n0 > 0, "expected initial partial bytes from A, got {n0}");
            initial_drained.extend_from_slice(&chunk[..n0 as usize]);

            let mut ov_b: OVERLAPPED = std::mem::zeroed();
            let wb_b = WsaBuf { len: payload_b.len() as u32, buf: payload_b.as_ptr() as *mut u8 };
            let mut sent_b = 0u32;
            assert_eq!(
                WSASend(cli, &wb_b, 1, &mut sent_b, 0, &mut ov_b as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let mut ov_c: OVERLAPPED = std::mem::zeroed();
            let wb_c = WsaBuf { len: payload_c.len() as u32, buf: payload_c.as_ptr() as *mut u8 };
            let mut sent_c = 0u32;
            assert_eq!(
                WSASendTo(
                    cli,
                    &wb_c,
                    1,
                    &mut sent_c,
                    0,
                    sa.as_ptr(),
                    16,
                    &mut ov_c as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            // Distinct socket `cli2` still completes immediately without waiting on `cli`.
            let ping = b"distinct-socket-ok";
            let mut ov2: OVERLAPPED = std::mem::zeroed();
            let wb2 = WsaBuf { len: ping.len() as u32, buf: ping.as_ptr() as *mut u8 };
            let mut sent2 = 0u32;
            assert_eq!(
                WSASend(cli2, &wb2, 1, &mut sent2, 0, &mut ov2 as *mut _ as *mut c_void, std::ptr::null_mut()),
                0
            );
            assert_eq!(sent2 as usize, ping.len());
            let mut ping_buf = [0u8; 32];
            assert_eq!(recv(acc2, ping_buf.as_mut_ptr(), ping_buf.len() as i32, 0), ping.len() as i32);
            assert_eq!(&ping_buf[..ping.len()], ping);

            // 3. Drain all remaining bytes on `acc` in a bounded reader thread.
            let total_want = expected.len();
            let reader = std::thread::spawn(move || {
                let mut got = initial_drained;
                let mut rbuf = [0u8; 8192];
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
                while got.len() < total_want && std::time::Instant::now() < deadline {
                    let n = recv(acc, rbuf.as_mut_ptr(), rbuf.len() as i32, 0);
                    if n > 0 {
                        got.extend_from_slice(&rbuf[..n as usize]);
                    } else if n == 0 {
                        break;
                    }
                }
                got
            });

            // Collect all three IOCP completions and verify exact cumulative byte counts & order.
            let mut completed_order = Vec::new();
            for _ in 0..3 {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                assert_eq!(
                    GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 5000),
                    TRUE
                );
                assert_eq!(key, 0x5111);
                completed_order.push((ov_out as usize, bytes as usize));
            }
            assert_eq!(
                completed_order,
                vec![
                    (&mut ov_a as *mut OVERLAPPED as usize, payload_a.len()),
                    (&mut ov_b as *mut OVERLAPPED as usize, payload_b.len()),
                    (&mut ov_c as *mut OVERLAPPED as usize, payload_c.len()),
                ],
                "TCP stream overlapped sends must complete in submission order with exact cumulative counts"
            );

            let received = reader.join().unwrap();
            assert_eq!(received.len(), expected.len());
            assert_eq!(received, expected, "peer must receive exact concatenation A || B || C");

            // 4. Cancelling the head pending stream write advances its queued successor cleanly.
            let payload_d = vec![0xD5u8; 256 * 1024];
            let payload_e = b"SUCCESSOR-AFTER-CANCEL-1234567890".to_vec();
            let mut ov_d: OVERLAPPED = std::mem::zeroed();
            let wb_d = WsaBuf { len: payload_d.len() as u32, buf: payload_d.as_ptr() as *mut u8 };
            let mut sent_d = 0u32;
            assert_eq!(
                WSASend(cli, &wb_d, 1, &mut sent_d, 0, &mut ov_d as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let mut ov_e: OVERLAPPED = std::mem::zeroed();
            let wb_e = WsaBuf { len: payload_e.len() as u32, buf: payload_e.as_ptr() as *mut u8 };
            let mut sent_e = 0u32;
            assert_eq!(
                WSASend(cli, &wb_e, 1, &mut sent_e, 0, &mut ov_e as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            assert_eq!(CancelIoEx(cli as HANDLE, &mut ov_d), TRUE);

            let tail_marker = payload_e.clone();
            let tail_reader = std::thread::spawn(move || {
                let mut got = Vec::new();
                let mut rbuf = [0u8; 8192];
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
                while std::time::Instant::now() < deadline {
                    if got.ends_with(&tail_marker) {
                        break;
                    }
                    let n = recv(acc, rbuf.as_mut_ptr(), rbuf.len() as i32, 0);
                    if n > 0 {
                        got.extend_from_slice(&rbuf[..n as usize]);
                    } else if n == 0 {
                        break;
                    }
                }
                got
            });

            let mut saw_d_cancel = false;
            let mut saw_e_ok = false;
            for _ in 0..2 {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                let ok = GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 5000);
                assert_eq!(key, 0x5111);
                if ov_out == &mut ov_d as *mut OVERLAPPED {
                    assert_eq!(ok, FALSE);
                    assert_eq!(crate::teb::last_error(), WSA_OPERATION_ABORTED);
                    saw_d_cancel = true;
                } else if ov_out == &mut ov_e as *mut OVERLAPPED {
                    assert_eq!(ok, TRUE);
                    assert_eq!(bytes as usize, payload_e.len());
                    saw_e_ok = true;
                } else {
                    panic!("unexpected overlapped {ov_out:p}");
                }
            }
            assert!(saw_d_cancel && saw_e_ok);
            let tail_bytes = tail_reader.join().unwrap();
            assert!(
                tail_bytes.ends_with(&payload_e),
                "queued successor E must advance and deliver its exact payload after head D is cancelled"
            );

            assert_eq!(closesocket(acc2), 0);
            assert_eq!(closesocket(cli2), 0);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn overlapped_wsarecv_and_wsarecvfrom_preserve_submission_order_and_cancel_head() {
        use crate::kernel32::file::{
            CancelIoEx, CreateIoCompletionPort, GetQueuedCompletionStatus, INVALID_HANDLE_VALUE,
            OVERLAPPED,
        };
        use crate::kernel32::threads::CloseHandle;

        unsafe {
            // Deterministic predicate check: second Recv on the same socket is ordered behind the
            // first Recv, while Send on the same socket and Recv on a different socket remain free.
            let dummy_ops = vec![PendingSocketOp {
                socket_handle: 0x1001,
                secondary_socket: None,
                poll_fd: -1,
                poll_events: libc::POLLIN,
                overlapped: 0x2001,
                issuing_tid: 1,
                issuing_thread: None,
                completion_routine: 0,
                completion_binding: None,
                kind: AsyncOpKind::Recv {
                    bufs: Vec::new(),
                    total_len: 0,
                    recvd_so_far: 0,
                    pflags: 0,
                    is_stream: true,
                    from: 0,
                    fromlen: 0,
                    from_captured: None,
                    msg_flags: 0,
                },
            }];
            assert!(has_prior_recv(&dummy_ops, 0x1001));
            assert!(!has_prior_recv(&dummy_ops, 0x1002));
            assert!(!has_prior_stream_write(&dummy_ops, 0x1001));

            // Part A: TCP WSARecv submission-order preservation when data arrives between recv1 and recv2.
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(srv, INVALID_SOCKET);
            let mut sa = [0u8; 16];
            (sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            let mut slen = 16i32;
            assert_eq!(getsockname(srv, sa.as_mut_ptr(), &mut slen), 0);
            assert_eq!(listen(srv, 4), 0);

            let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli, INVALID_SOCKET);
            assert_eq!(connect(cli, sa.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);

            // Second independent TCP pair to verify distinct sockets are not blocked by `acc`.
            let cli2 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(cli2, INVALID_SOCKET);
            assert_eq!(connect(cli2, sa.as_ptr(), 16), 0);
            let acc2 = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc2, INVALID_SOCKET);

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());
            assert_eq!(CreateIoCompletionPort(acc as HANDLE, iocp, 0x7101, 0), iocp);

            for iter in 0u8..6 {
                let mut b1 = [0u8; 8];
                let mut b2 = [0u8; 8];
                let wb1 = WsaBuf { len: b1.len() as u32, buf: b1.as_mut_ptr() };
                let wb2 = WsaBuf { len: b2.len() as u32, buf: b2.as_mut_ptr() };
                let mut fl1 = 0u32;
                let mut fl2 = 0u32;
                let mut r1 = 0u32;
                let mut r2 = 0u32;
                let mut ov1: OVERLAPPED = std::mem::zeroed();
                let mut ov2: OVERLAPPED = std::mem::zeroed();

                // 1. First receive pends on empty socket.
                assert_eq!(
                    WSARecv(acc, &wb1, 1, &mut r1, &mut fl1, &mut ov1 as *mut _ as *mut c_void, std::ptr::null_mut()),
                    SOCKET_ERROR
                );
                assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

                // While `ov1` is pending on `acc`, send on `acc` and recv on `acc2` must stay independent.
                if iter == 0 {
                    let out_msg = b"send-indep";
                    let wb_out = WsaBuf { len: out_msg.len() as u32, buf: out_msg.as_ptr() as *mut u8 };
                    let mut sent_n = 0u32;
                    let mut ov_send: OVERLAPPED = std::mem::zeroed();
                    assert_eq!(
                        WSASend(acc, &wb_out, 1, &mut sent_n, 0, &mut ov_send as *mut _ as *mut c_void, std::ptr::null_mut()),
                        0
                    );
                    assert_eq!(sent_n as usize, out_msg.len());
                    let mut cli_got = [0u8; 16];
                    assert_eq!(recv(cli, cli_got.as_mut_ptr(), 16, 0), out_msg.len() as i32);
                    assert_eq!(&cli_got[..out_msg.len()], out_msg);
                    // Drain `ov_send` completion from IOCP.
                    let mut sb = 0u32;
                    let mut sk = 0usize;
                    let mut sov: *mut OVERLAPPED = std::ptr::null_mut();
                    assert_eq!(GetQueuedCompletionStatus(iocp, &mut sb, &mut sk, &mut sov, 2000), TRUE);
                    assert_eq!(sov, &mut ov_send as *mut OVERLAPPED);

                    let other = b"sock2!";
                    assert_eq!(send(cli2, other.as_ptr(), other.len() as i32, 0), other.len() as i32);
                    let mut obuf = [0u8; 16];
                    let wb_o = WsaBuf { len: obuf.len() as u32, buf: obuf.as_mut_ptr() };
                    let mut ofl = 0u32;
                    let mut on = 0u32;
                    let mut ov_o: OVERLAPPED = std::mem::zeroed();
                    let mut pfd = libc::pollfd { fd: lookup_fd(acc2).unwrap(), events: libc::POLLIN, revents: 0 };
                    assert!(libc::poll(&mut pfd, 1, 2000) > 0);
                    assert_eq!(
                        WSARecv(acc2, &wb_o, 1, &mut on, &mut ofl, &mut ov_o as *mut _ as *mut c_void, std::ptr::null_mut()),
                        0
                    );
                    assert_eq!(&obuf[..on as usize], other);
                }

                // 2. Peer sends 16 bytes, then issuing thread immediately submits second WSARecv.
                let first = [b'A' + iter; 8];
                let second = [b'a' + iter; 8];
                let mut combined = [0u8; 16];
                combined[..8].copy_from_slice(&first);
                combined[8..].copy_from_slice(&second);
                assert_eq!(send(cli, combined.as_ptr(), combined.len() as i32, 0), 16);

                let rc2 = WSARecv(
                    acc,
                    &wb2,
                    1,
                    &mut r2,
                    &mut fl2,
                    &mut ov2 as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                );
                assert!(
                    rc2 == 0 || (rc2 == SOCKET_ERROR && WSAGetLastError() == WSA_IO_PENDING as i32),
                    "unexpected rc2={rc2} err={}",
                    WSAGetLastError()
                );

                // Both completions must arrive in exact submission order with unswapped payloads.
                let mut order = Vec::new();
                for _ in 0..2 {
                    let mut bytes = 0u32;
                    let mut key = 0usize;
                    let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                    assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 3000), TRUE);
                    assert_eq!(key, 0x7101);
                    order.push((ov_out as usize, bytes as usize));
                }
                assert_eq!(
                    order,
                    vec![
                        (&mut ov1 as *mut OVERLAPPED as usize, 8),
                        (&mut ov2 as *mut OVERLAPPED as usize, 8),
                    ],
                    "iter {iter}: overlapped WSARecv completions must remain in submission order"
                );
                assert_eq!(b1, first, "iter {iter}: first WSARecv must receive first 8 bytes");
                assert_eq!(b2, second, "iter {iter}: second WSARecv must receive second 8 bytes");
            }

            // Part B: UDP WSARecvFrom submission-order preservation with source address.
            let rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(rx, INVALID_SOCKET);
            assert_ne!(tx, INVALID_SOCKET);
            let mut rx_sa = [0u8; 16];
            (rx_sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            rx_sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(rx, rx_sa.as_ptr(), 16), 0);
            let mut rx_slen = 16i32;
            assert_eq!(getsockname(rx, rx_sa.as_mut_ptr(), &mut rx_slen), 0);
            assert_eq!(CreateIoCompletionPort(rx as HANDLE, iocp, 0x7102, 0), iocp);

            let mut ub1 = [0u8; 32];
            let mut ub2 = [0u8; 32];
            let uwb1 = WsaBuf { len: ub1.len() as u32, buf: ub1.as_mut_ptr() };
            let uwb2 = WsaBuf { len: ub2.len() as u32, buf: ub2.as_mut_ptr() };
            let mut ufl1 = 0u32;
            let mut ufl2 = 0u32;
            let mut ur1 = 0u32;
            let mut ur2 = 0u32;
            let mut from1 = [0u8; 16];
            let mut from2 = [0u8; 16];
            let mut flen1 = 16i32;
            let mut flen2 = 16i32;
            let mut uov1: OVERLAPPED = std::mem::zeroed();
            let mut uov2: OVERLAPPED = std::mem::zeroed();

            assert_eq!(
                WSARecvFrom(
                    rx,
                    &uwb1,
                    1,
                    &mut ur1,
                    &mut ufl1,
                    from1.as_mut_ptr(),
                    &mut flen1,
                    &mut uov1 as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let d1 = b"DGRAM-ONE";
            let d2 = b"DGRAM-TWO";
            assert_eq!(sendto(tx, d1.as_ptr(), d1.len() as i32, 0, rx_sa.as_ptr(), 16), d1.len() as i32);
            assert_eq!(sendto(tx, d2.as_ptr(), d2.len() as i32, 0, rx_sa.as_ptr(), 16), d2.len() as i32);

            let urc2 = WSARecvFrom(
                rx,
                &uwb2,
                1,
                &mut ur2,
                &mut ufl2,
                from2.as_mut_ptr(),
                &mut flen2,
                &mut uov2 as *mut _ as *mut c_void,
                std::ptr::null_mut(),
            );
            assert!(
                urc2 == 0 || (urc2 == SOCKET_ERROR && WSAGetLastError() == WSA_IO_PENDING as i32),
                "unexpected urc2={urc2} err={}",
                WSAGetLastError()
            );

            let mut uorder = Vec::new();
            for _ in 0..2 {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 3000), TRUE);
                assert_eq!(key, 0x7102);
                uorder.push((ov_out as usize, bytes as usize));
            }
            assert_eq!(
                uorder,
                vec![
                    (&mut uov1 as *mut OVERLAPPED as usize, d1.len()),
                    (&mut uov2 as *mut OVERLAPPED as usize, d2.len()),
                ]
            );
            assert_eq!(&ub1[..d1.len()], d1);
            assert_eq!(&ub2[..d2.len()], d2);
            assert_eq!(&from1[4..8], &[127, 0, 0, 1]);
            assert_eq!(&from2[4..8], &[127, 0, 0, 1]);

            // Part C: Cancelling the head queued receive advances its successor cleanly.
            let mut cb1 = [0u8; 8];
            let mut cb2 = [0u8; 8];
            let cwb1 = WsaBuf { len: cb1.len() as u32, buf: cb1.as_mut_ptr() };
            let cwb2 = WsaBuf { len: cb2.len() as u32, buf: cb2.as_mut_ptr() };
            let mut cfl1 = 0u32;
            let mut cfl2 = 0u32;
            let mut cr1 = 0u32;
            let mut cr2 = 0u32;
            let mut cov1: OVERLAPPED = std::mem::zeroed();
            let mut cov2: OVERLAPPED = std::mem::zeroed();

            assert_eq!(
                WSARecv(acc, &cwb1, 1, &mut cr1, &mut cfl1, &mut cov1 as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            assert_eq!(
                WSARecv(acc, &cwb2, 1, &mut cr2, &mut cfl2, &mut cov2 as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            assert_eq!(CancelIoEx(acc as HANDLE, &mut cov1), TRUE);
            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 3000), FALSE);
            assert_eq!(ov_out, &mut cov1 as *mut OVERLAPPED);
            assert_eq!(crate::teb::last_error(), WSA_OPERATION_ABORTED);

            let after = b"AFTER_CL";
            assert_eq!(send(cli, after.as_ptr(), after.len() as i32, 0), after.len() as i32);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 3000), TRUE);
            assert_eq!(ov_out, &mut cov2 as *mut OVERLAPPED);
            assert_eq!(bytes as usize, after.len());
            assert_eq!(&cb2, after);

            assert_eq!(closesocket(tx), 0);
            assert_eq!(closesocket(rx), 0);
            assert_eq!(closesocket(acc2), 0);
            assert_eq!(closesocket(cli2), 0);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn udp_oversized_datagram_reports_wsaemsgsize_across_pending_immediate_sync_and_recovers() {
        use crate::kernel32::file::{
            CreateIoCompletionPort, GetQueuedCompletionStatus, INVALID_HANDLE_VALUE, OVERLAPPED,
        };
        use crate::kernel32::threads::CloseHandle;
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::AtomicUsize;

        const ERROR_MORE_DATA: u32 = 234;
        const WAIT_TIMEOUT_CODE: u32 = 258;
        const WAIT_IO_COMPLETION: u32 = 0xC0;

        static UDP_APC_ERR: AtomicUsize = AtomicUsize::new(usize::MAX);
        static UDP_APC_BYTES: AtomicUsize = AtomicUsize::new(usize::MAX);
        static UDP_APC_OV: AtomicUsize = AtomicUsize::new(0);
        static UDP_APC_CALLS: AtomicUsize = AtomicUsize::new(0);

        unsafe extern "win64" fn udp_apc_cb(err_code: u32, bytes: u32, ov: *mut OVERLAPPED) {
            UDP_APC_ERR.store(err_code as usize, Ordering::Release);
            UDP_APC_BYTES.store(bytes as usize, Ordering::Release);
            UDP_APC_OV.store(ov as usize, Ordering::Release);
            UDP_APC_CALLS.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            let rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(rx, INVALID_SOCKET);
            assert_ne!(tx, INVALID_SOCKET);
            let mut rx_sa = [0u8; 16];
            (rx_sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            rx_sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(rx, rx_sa.as_ptr(), 16), 0);
            let mut rx_slen = 16i32;
            assert_eq!(getsockname(rx, rx_sa.as_mut_ptr(), &mut rx_slen), 0);
            let rx_fd = lookup_fd(rx).unwrap();

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());
            assert_eq!(CreateIoCompletionPort(rx as HANDLE, iocp, 0x7201, 0), iocp);

            // 1. Pending overlapped WSARecvFrom -> IOCP + WSAGetOverlappedResult.
            let mut small = [0u8; 4];
            let wb_small = WsaBuf { len: small.len() as u32, buf: small.as_mut_ptr() };
            let mut fl = 0u32;
            let mut recvd = 0u32;
            let mut from = [0u8; 16];
            let mut fromlen = 16i32;
            let mut ov_big: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WSARecvFrom(
                    rx,
                    &wb_small,
                    1,
                    &mut recvd,
                    &mut fl,
                    from.as_mut_ptr(),
                    &mut fromlen,
                    &mut ov_big as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let big = b"0123456789ABCDEF";
            assert_eq!(sendto(tx, big.as_ptr(), big.len() as i32, 0, rx_sa.as_ptr(), 16), big.len() as i32);

            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 3000), FALSE);
            assert_eq!(ov_out, &mut ov_big as *mut OVERLAPPED);
            assert_eq!(key, 0x7201);
            assert_eq!(bytes, 4);
            assert_eq!(crate::teb::last_error(), ERROR_MORE_DATA);
            assert_eq!(&small, b"0123");
            assert_eq!(&from[4..8], &[127, 0, 0, 1]);

            let mut xfer = 0u32;
            let mut out_fl = 0u32;
            assert_eq!(WSAGetOverlappedResult(rx, &mut ov_big, &mut xfer, FALSE, &mut out_fl), FALSE);
            assert_eq!(xfer, 4);
            assert_eq!(WSAGetLastError(), WSAEMSGSIZE as i32);

            // Follow-up normal datagram must arrive cleanly.
            let mut norm_buf = [0u8; 16];
            let wb_norm = WsaBuf { len: norm_buf.len() as u32, buf: norm_buf.as_mut_ptr() };
            let mut ov_norm: OVERLAPPED = std::mem::zeroed();
            fl = 0;
            fromlen = 16;
            assert_eq!(
                WSARecvFrom(
                    rx,
                    &wb_norm,
                    1,
                    &mut recvd,
                    &mut fl,
                    from.as_mut_ptr(),
                    &mut fromlen,
                    &mut ov_norm as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            let ok_msg = b"OKAY!";
            assert_eq!(sendto(tx, ok_msg.as_ptr(), ok_msg.len() as i32, 0, rx_sa.as_ptr(), 16), ok_msg.len() as i32);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 3000), TRUE);
            assert_eq!(ov_out, &mut ov_norm as *mut OVERLAPPED);
            assert_eq!(bytes as usize, ok_msg.len());
            assert_eq!(&norm_buf[..ok_msg.len()], ok_msg);

            // 2. Immediate overlapped WSARecvFrom when oversized datagram is already queued:
            //    returns SOCKET_ERROR + WSAEMSGSIZE with NO completion packet posted to IOCP.
            let imm_big = b"IMMD_OVERSIZED!!";
            assert_eq!(sendto(tx, imm_big.as_ptr(), imm_big.len() as i32, 0, rx_sa.as_ptr(), 16), imm_big.len() as i32);
            let mut pfd = libc::pollfd { fd: rx_fd, events: libc::POLLIN, revents: 0 };
            assert!(libc::poll(&mut pfd, 1, 2000) > 0);

            small = [0u8; 4];
            fl = 0;
            recvd = 0;
            fromlen = 16;
            let mut ov_imm: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WSARecvFrom(
                    rx,
                    &wb_small,
                    1,
                    &mut recvd,
                    &mut fl,
                    from.as_mut_ptr(),
                    &mut fromlen,
                    &mut ov_imm as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEMSGSIZE as i32);
            assert_eq!(recvd, 4);
            assert_eq!(&small, b"IMMD");
            assert_eq!(&from[4..8], &[127, 0, 0, 1]);
            // Official WSARecvFrom contract: immediate non-WSA_IO_PENDING error emits no completion.
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 30), FALSE);
            assert_eq!(crate::teb::last_error(), WAIT_TIMEOUT_CODE);

            // 3. Pending overlapped WSARecv -> APC completion routine (connected UDP), then recovery.
            let rx_apc = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx_apc = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(rx_apc, INVALID_SOCKET);
            assert_ne!(tx_apc, INVALID_SOCKET);
            let mut apc_sa = [0u8; 16];
            (apc_sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            apc_sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(rx_apc, apc_sa.as_ptr(), 16), 0);
            let mut apc_slen = 16i32;
            assert_eq!(getsockname(rx_apc, apc_sa.as_mut_ptr(), &mut apc_slen), 0);

            UDP_APC_CALLS.store(0, Ordering::Release);
            let mut apc_small = [0u8; 4];
            let wb_apc_small = WsaBuf { len: apc_small.len() as u32, buf: apc_small.as_mut_ptr() };
            let mut ov_apc: OVERLAPPED = std::mem::zeroed();
            fl = 0;
            assert_eq!(
                WSARecv(
                    rx_apc,
                    &wb_apc_small,
                    1,
                    &mut recvd,
                    &mut fl,
                    &mut ov_apc as *mut _ as *mut c_void,
                    udp_apc_cb as *const () as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let apc_big = b"APC_TOO_LONG";
            assert_eq!(sendto(tx_apc, apc_big.as_ptr(), apc_big.len() as i32, 0, apc_sa.as_ptr(), 16), apc_big.len() as i32);
            assert_eq!(SleepEx(2000, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(UDP_APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(UDP_APC_ERR.load(Ordering::Acquire), WSAEMSGSIZE as usize);
            assert_eq!(UDP_APC_BYTES.load(Ordering::Acquire), 4);
            assert_eq!(UDP_APC_OV.load(Ordering::Acquire), &mut ov_apc as *mut OVERLAPPED as usize);
            assert_eq!(&apc_small, b"APC_");

            // Normal follow-up datagram on APC socket succeeds with dwError == 0.
            let mut apc_ok_buf = [0u8; 16];
            let wb_apc_ok = WsaBuf { len: apc_ok_buf.len() as u32, buf: apc_ok_buf.as_mut_ptr() };
            let mut ov_apc_ok: OVERLAPPED = std::mem::zeroed();
            fl = 0;
            assert_eq!(
                WSARecv(
                    rx_apc,
                    &wb_apc_ok,
                    1,
                    &mut recvd,
                    &mut fl,
                    &mut ov_apc_ok as *mut _ as *mut c_void,
                    udp_apc_cb as *const () as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            let apc_ok = b"APC_OK";
            assert_eq!(sendto(tx_apc, apc_ok.as_ptr(), apc_ok.len() as i32, 0, apc_sa.as_ptr(), 16), apc_ok.len() as i32);
            assert_eq!(SleepEx(2000, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(UDP_APC_CALLS.load(Ordering::Acquire), 2);
            assert_eq!(UDP_APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(UDP_APC_BYTES.load(Ordering::Acquire), apc_ok.len());
            assert_eq!(&apc_ok_buf[..apc_ok.len()], apc_ok);

            // Immediate overlapped WSARecv on oversized datagram -> SOCKET_ERROR + WSAEMSGSIZE, no APC.
            let imm_apc = b"NO_APC_TRUNC";
            assert_eq!(sendto(tx_apc, imm_apc.as_ptr(), imm_apc.len() as i32, 0, apc_sa.as_ptr(), 16), imm_apc.len() as i32);
            let mut pfd_apc = libc::pollfd { fd: lookup_fd(rx_apc).unwrap(), events: libc::POLLIN, revents: 0 };
            assert!(libc::poll(&mut pfd_apc, 1, 2000) > 0);
            fl = 0;
            recvd = 0;
            let mut ov_imm_apc: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WSARecv(
                    rx_apc,
                    &wb_apc_small,
                    1,
                    &mut recvd,
                    &mut fl,
                    &mut ov_imm_apc as *mut _ as *mut c_void,
                    udp_apc_cb as *const () as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEMSGSIZE as i32);
            assert_eq!(recvd, 4);
            assert_eq!(&apc_small, b"NO_A");
            assert_eq!(SleepEx(20, TRUE), 0);
            assert_eq!(UDP_APC_CALLS.load(Ordering::Acquire), 2);

            // 4. Synchronous (overlapped == NULL) WSARecvFrom and WSARecv front doors.
            let sync_big = b"SYNC_TRUNC_12345";
            assert_eq!(sendto(tx, sync_big.as_ptr(), sync_big.len() as i32, 0, rx_sa.as_ptr(), 16), sync_big.len() as i32);
            small = [0u8; 4];
            fl = 0;
            recvd = 0;
            fromlen = 16;
            assert_eq!(
                WSARecvFrom(
                    rx,
                    &wb_small,
                    1,
                    &mut recvd,
                    &mut fl,
                    from.as_mut_ptr(),
                    &mut fromlen,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEMSGSIZE as i32);
            assert_eq!(recvd, 4);
            assert_eq!(&small, b"SYNC");

            let sync_ok = b"GOOD";
            assert_eq!(sendto(tx, sync_ok.as_ptr(), sync_ok.len() as i32, 0, rx_sa.as_ptr(), 16), sync_ok.len() as i32);
            fl = 0;
            recvd = 0;
            assert_eq!(
                WSARecv(rx, &wb_norm, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd as usize, sync_ok.len());
            assert_eq!(&norm_buf[..sync_ok.len()], sync_ok);

            assert_eq!(sendto(tx, sync_big.as_ptr(), sync_big.len() as i32, 0, rx_sa.as_ptr(), 16), sync_big.len() as i32);
            fl = 0;
            recvd = 0;
            assert_eq!(
                WSARecv(rx, &wb_small, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEMSGSIZE as i32);
            assert_eq!(recvd, 4);

            // 5. TCP stream read into smaller buffer must NEVER be misclassified as WSAEMSGSIZE.
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut tcp_sa = [0u8; 16];
            (tcp_sa.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            tcp_sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(srv, tcp_sa.as_ptr(), 16), 0);
            let mut tcp_slen = 16i32;
            assert_eq!(getsockname(srv, tcp_sa.as_mut_ptr(), &mut tcp_slen), 0);
            assert_eq!(listen(srv, 2), 0);
            let tcli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_eq!(connect(tcli, tcp_sa.as_ptr(), 16), 0);
            let tacc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(tacc, INVALID_SOCKET);

            let stream_msg = b"STREAM_PAYLOAD16";
            assert_eq!(send(tcli, stream_msg.as_ptr(), stream_msg.len() as i32, 0), 16);
            let mut sbuf = [0u8; 4];
            let wb_s = WsaBuf { len: 4, buf: sbuf.as_mut_ptr() };
            fl = 0;
            recvd = 0;
            assert_eq!(WSARecv(tacc, &wb_s, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()), 0);
            assert_eq!(recvd, 4);
            assert_eq!(&sbuf, b"STRE");
            let mut ov_s: OVERLAPPED = std::mem::zeroed();
            fl = 0;
            recvd = 0;
            assert_eq!(
                WSARecv(tacc, &wb_s, 1, &mut recvd, &mut fl, &mut ov_s as *mut _ as *mut c_void, std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 4);
            assert_eq!(&sbuf, b"AM_P");

            assert_eq!(closesocket(tacc), 0);
            assert_eq!(closesocket(tcli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(closesocket(tx_apc), 0);
            assert_eq!(closesocket(rx_apc), 0);
            assert_eq!(closesocket(tx), 0);
            assert_eq!(closesocket(rx), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn sync_wsa_recv_and_recv_msg_waitall_accumulate_and_validate_flags() {
        unsafe {
            let mut wd = [0u8; 512];
            assert_eq!(WSAStartup(0x0202, wd.as_mut_ptr()), 0);

            // Helper closure to create a connected loopback TCP pair (cli, acc).
            let make_tcp_pair = || -> (usize, usize, usize) {
                let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
                assert_ne!(srv, INVALID_SOCKET);
                let mut bind_sa = [0u8; 16];
                bind_sa[0..2].copy_from_slice(&AF_INET.to_le_bytes());
                bind_sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
                assert_eq!(bind(srv, bind_sa.as_ptr(), 16), 0);
                assert_eq!(listen(srv, 4), 0);
                let mut bound_sa = [0u8; 16];
                let mut bound_len = 16i32;
                assert_eq!(getsockname(srv, bound_sa.as_mut_ptr(), &mut bound_len), 0);

                let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
                assert_ne!(cli, INVALID_SOCKET);
                assert_eq!(connect(cli, bound_sa.as_ptr(), 16), 0);
                let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
                assert_ne!(acc, INVALID_SOCKET);
                (srv, cli, acc)
            };

            let (srv, cli, acc) = make_tcp_pair();

            // 1. Staged sends with first byte already available, crossing multiple WSABUFs
            //    (including a zero-length middle buffer).
            assert_eq!(send(cli, b"A".as_ptr(), 1, 0), 1);
            let sender = std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(25));
                assert_eq!(send(cli, b"BCDE".as_ptr(), 4, 0), 4);
                std::thread::sleep(std::time::Duration::from_millis(25));
                assert_eq!(send(cli, b"FGHIJKL".as_ptr(), 7, 0), 7);
                std::thread::sleep(std::time::Duration::from_millis(25));
                assert_eq!(send(cli, b"MNOP".as_ptr(), 4, 0), 4);
                cli
            });
            let mut b1 = [0u8; 3];
            let mut b2 = [0u8; 0];
            let mut b3 = [0u8; 5];
            let mut b4 = [0u8; 8];
            let wbs = [
                WsaBuf { len: 3, buf: b1.as_mut_ptr() },
                WsaBuf { len: 0, buf: b2.as_mut_ptr() },
                WsaBuf { len: 5, buf: b3.as_mut_ptr() },
                WsaBuf { len: 8, buf: b4.as_mut_ptr() },
            ];
            let mut recvd = 0u32;
            let mut fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecv(acc, wbs.as_ptr(), wbs.len() as u32, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 16);
            assert_eq!(&b1, b"ABC");
            assert_eq!(&b3, b"DEFGH");
            assert_eq!(&b4, b"IJKLMNOP");
            let cli = sender.join().unwrap();

            // 2. Synchronous WSARecvFrom on blocking stream socket with MSG_WAITALL.
            assert_eq!(send(cli, b"12".as_ptr(), 2, 0), 2);
            let sender2 = std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(25));
                assert_eq!(send(cli, b"345678".as_ptr(), 6, 0), 6);
                cli
            });
            let mut f1 = [0u8; 3];
            let mut f2 = [0u8; 5];
            let wfbs = [
                WsaBuf { len: 3, buf: f1.as_mut_ptr() },
                WsaBuf { len: 5, buf: f2.as_mut_ptr() },
            ];
            let mut from_sa = [0u8; 32];
            let mut from_len = 32i32;
            fl = WIN_MSG_WAITALL;
            recvd = 0;
            assert_eq!(
                WSARecvFrom(
                    acc,
                    wfbs.as_ptr(),
                    wfbs.len() as u32,
                    &mut recvd,
                    &mut fl,
                    from_sa.as_mut_ptr(),
                    &mut from_len,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_eq!(recvd, 8);
            assert_eq!(&f1, b"123");
            assert_eq!(&f2, b"45678");
            let cli = sender2.join().unwrap();

            // 3. Plain recv() with Windows MSG_WAITALL (0x8) also accumulates staged bytes.
            assert_eq!(send(cli, b"p".as_ptr(), 1, 0), 1);
            let sender3 = std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(25));
                assert_eq!(send(cli, b"qrstuvw".as_ptr(), 7, 0), 7);
                cli
            });
            let mut rbuf = [0u8; 8];
            assert_eq!(recv(acc, rbuf.as_mut_ptr(), 8, WIN_MSG_WAITALL as i32), 8);
            assert_eq!(&rbuf, b"pqrstuvw");
            let cli = sender3.join().unwrap();

            // 4. Zero-length WSARecv with MSG_WAITALL returns 0 immediately without spinning.
            let zero_wb = WsaBuf { len: 0, buf: std::ptr::null_mut() };
            fl = WIN_MSG_WAITALL;
            recvd = 99;
            assert_eq!(
                WSARecv(acc, &zero_wb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 0);

            // 5. Ordinary WSARecv WITHOUT MSG_WAITALL still returns the first short read.
            assert_eq!(send(cli, b"XYZ".as_ptr(), 3, 0), 3);
            let mut short_buf = [0u8; 16];
            let short_wb = WsaBuf { len: 16, buf: short_buf.as_mut_ptr() };
            fl = 0;
            recvd = 0;
            assert_eq!(
                WSARecv(acc, &short_wb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 3);
            assert_eq!(&short_buf[..3], b"XYZ");

            // 6. Premature EOF under MSG_WAITALL returns partial accumulated bytes cleanly,
            //    followed by 0 on the next call.
            let (srv_eof, cli_eof, acc_eof) = make_tcp_pair();
            assert_eq!(send(cli_eof, b"HE".as_ptr(), 2, 0), 2);
            let eof_sender = std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(20));
                assert_eq!(send(cli_eof, b"LLO".as_ptr(), 3, 0), 3);
                assert_eq!(closesocket(cli_eof), 0);
            });
            let mut e1 = [0u8; 3];
            let mut e2 = [0u8; 7];
            let ewbs = [
                WsaBuf { len: 3, buf: e1.as_mut_ptr() },
                WsaBuf { len: 7, buf: e2.as_mut_ptr() },
            ];
            fl = WIN_MSG_WAITALL;
            recvd = 0;
            assert_eq!(
                WSARecv(acc_eof, ewbs.as_ptr(), ewbs.len() as u32, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            eof_sender.join().unwrap();
            assert_eq!(recvd, 5);
            assert_eq!(&e1, b"HEL");
            assert_eq!(&e2[..2], b"LO");
            fl = WIN_MSG_WAITALL;
            recvd = 99;
            assert_eq!(
                WSARecv(acc_eof, ewbs.as_ptr(), ewbs.len() as u32, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 0);
            assert_eq!(closesocket(acc_eof), 0);
            assert_eq!(closesocket(srv_eof), 0);

            // 7. SO_RCVTIMEO expires under MSG_WAITALL (even after partial bytes arrive) with WSAETIMEDOUT.
            let rcv_ms: u32 = 80;
            assert_eq!(
                setsockopt(acc, 0xFFFF, 0x1006, &rcv_ms as *const u32 as *const u8, 4),
                0
            );
            assert_eq!(send(cli, b"TO".as_ptr(), 2, 0), 2);
            let mut to_buf = [0u8; 8];
            let to_wb = WsaBuf { len: 8, buf: to_buf.as_mut_ptr() };
            let t0 = std::time::Instant::now();
            fl = WIN_MSG_WAITALL;
            recvd = 0;
            assert_eq!(
                WSARecv(acc, &to_wb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAETIMEDOUT as i32);
            assert!(t0.elapsed() < std::time::Duration::from_millis(1500));

            // 8. Microsoft WSARecv / recv flag-table rejections -> WSAEOPNOTSUPP:
            //    MSG_WAITALL combined with MSG_OOB, MSG_PEEK, or MSG_PARTIAL.
            let mut vbuf = [0u8; 4];
            let vwb = WsaBuf { len: 4, buf: vbuf.as_mut_ptr() };
            for bad_fl in [
                WIN_MSG_WAITALL | WIN_MSG_OOB,
                WIN_MSG_WAITALL | WIN_MSG_PEEK,
                WIN_MSG_WAITALL | WIN_MSG_PARTIAL,
            ] {
                let mut f_in = bad_fl;
                assert_eq!(
                    WSARecv(acc, &vwb, 1, &mut recvd, &mut f_in, std::ptr::null_mut(), std::ptr::null_mut()),
                    SOCKET_ERROR
                );
                assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);
                assert_eq!(recv(acc, vbuf.as_mut_ptr(), 4, bad_fl as i32), SOCKET_ERROR);
                assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);
            }

            //    MSG_WAITALL on guest-nonblocking (FIONBIO) stream socket -> WSAEOPNOTSUPP,
            //    while ordinary nonblocking WSARecv (fl = 0) still returns WSAEWOULDBLOCK.
            let mut nb: u32 = 1;
            assert_eq!(ioctlsocket(acc, 0x8004_667Eu32 as i32, &mut nb), 0);
            fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecv(acc, &vwb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);
            assert_eq!(recv(acc, vbuf.as_mut_ptr(), 4, WIN_MSG_WAITALL as i32), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);
            fl = 0;
            assert_eq!(
                WSARecv(acc, &vwb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEWOULDBLOCK as i32);

            //    MSG_WAITALL on UDP datagram socket -> WSAEOPNOTSUPP, while ordinary UDP
            //    datagram boundary & MSG_PEEK semantics remain untouched.
            let udp_rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let udp_tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(udp_rx, INVALID_SOCKET);
            assert_ne!(udp_tx, INVALID_SOCKET);
            let mut usa = [0u8; 16];
            usa[0..2].copy_from_slice(&AF_INET.to_le_bytes());
            usa[4..8].copy_from_slice(&[127, 0, 0, 1]);
            assert_eq!(bind(udp_rx, usa.as_ptr(), 16), 0);
            let mut ubound = [0u8; 16];
            let mut ulen = 16i32;
            assert_eq!(getsockname(udp_rx, ubound.as_mut_ptr(), &mut ulen), 0);

            assert_eq!(sendto(udp_tx, b"DG1".as_ptr(), 3, 0, ubound.as_ptr(), 16), 3);
            assert_eq!(sendto(udp_tx, b"DG2".as_ptr(), 3, 0, ubound.as_ptr(), 16), 3);

            let mut ubuf = [0u8; 16];
            let uwb = WsaBuf { len: 16, buf: ubuf.as_mut_ptr() };
            fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecv(udp_rx, &uwb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);
            fl = WIN_MSG_WAITALL;
            let mut ufrom = [0u8; 32];
            let mut ufromlen = 32i32;
            assert_eq!(
                WSARecvFrom(
                    udp_rx,
                    &uwb,
                    1,
                    &mut recvd,
                    &mut fl,
                    ufrom.as_mut_ptr(),
                    &mut ufromlen,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);
            assert_eq!(recv(udp_rx, ubuf.as_mut_ptr(), 16, WIN_MSG_WAITALL as i32), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);
            assert_eq!(
                recvfrom(udp_rx, ubuf.as_mut_ptr(), 16, WIN_MSG_WAITALL as i32, ufrom.as_mut_ptr(), &mut ufromlen),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);

            // MSG_PEEK without MSG_WAITALL does not consume DG1; ordinary WSARecv reads DG1 then DG2 separately.
            fl = WIN_MSG_PEEK;
            recvd = 0;
            assert_eq!(
                WSARecv(udp_rx, &uwb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 3);
            assert_eq!(&ubuf[..3], b"DG1");

            fl = 0;
            recvd = 0;
            assert_eq!(
                WSARecv(udp_rx, &uwb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 3);
            assert_eq!(&ubuf[..3], b"DG1");

            fl = 0;
            recvd = 0;
            assert_eq!(
                WSARecv(udp_rx, &uwb, 1, &mut recvd, &mut fl, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 3);
            assert_eq!(&ubuf[..3], b"DG2");

            assert_eq!(closesocket(udp_tx), 0);
            assert_eq!(closesocket(udp_rx), 0);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
        }
    }

    #[test]
    fn overlapped_wsa_recv_and_recvfrom_msg_waitall_accumulate_order_eof_and_cancel() {
        use crate::kernel32::file::{
            CancelIoEx, CreateIoCompletionPort, GetQueuedCompletionStatus, INVALID_HANDLE_VALUE,
            OVERLAPPED,
        };
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};

        unsafe {
            let mut wd = [0u8; 512];
            assert_eq!(WSAStartup(0x0202, wd.as_mut_ptr()), 0);

            let make_tcp_pair = || -> (usize, usize, usize) {
                let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
                assert_ne!(srv, INVALID_SOCKET);
                let mut bind_sa = [0u8; 16];
                bind_sa[0..2].copy_from_slice(&AF_INET.to_le_bytes());
                bind_sa[4..8].copy_from_slice(&[127, 0, 0, 1]);
                assert_eq!(bind(srv, bind_sa.as_ptr(), 16), 0);
                assert_eq!(listen(srv, 4), 0);
                let mut bound_sa = [0u8; 16];
                let mut bound_len = 16i32;
                assert_eq!(getsockname(srv, bound_sa.as_mut_ptr(), &mut bound_len), 0);

                let cli = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
                assert_ne!(cli, INVALID_SOCKET);
                assert_eq!(connect(cli, bound_sa.as_ptr(), 16), 0);
                let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
                assert_ne!(acc, INVALID_SOCKET);
                (srv, cli, acc)
            };

            let wait_readable = |sock: usize| {
                let mut pfd = libc::pollfd {
                    fd: lookup_fd(sock).unwrap(),
                    events: libc::POLLIN,
                    revents: 0,
                };
                assert!(libc::poll(&mut pfd, 1, 2000) > 0, "expected readable data on socket");
            };

            let assert_no_iocp = |iocp: HANDLE, ms: u32| {
                let mut bytes = 0u32;
                let mut key = 0usize;
                let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
                assert_eq!(
                    GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, ms),
                    FALSE
                );
                assert_eq!(crate::kernel32::process::GetLastError(), WAIT_TIMEOUT);
            };

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());

            let (srv, cli, acc) = make_tcp_pair();
            assert_eq!(CreateIoCompletionPort(acc as HANDLE, iocp, 0x8201, 0), iocp);

            // 1. Leading byte already buffered before submission + scatter across multiple WSABUFs
            //    (including zero-length middle buffer): must stay pending on BOTH hEvent and IOCP
            //    across intermediate short reads and complete exactly once when full.
            assert_eq!(send(cli, b"A".as_ptr(), 1, 0), 1);
            wait_readable(acc);

            let ev = CreateEventW(std::ptr::null_mut(), TRUE, TRUE, std::ptr::null());
            assert!(!ev.is_null());
            let mut b1 = [0u8; 3];
            let mut b2 = [0u8; 0];
            let mut b3 = [0u8; 5];
            let mut b4 = [0u8; 4];
            let wbs = [
                WsaBuf { len: 3, buf: b1.as_mut_ptr() },
                WsaBuf { len: 0, buf: b2.as_mut_ptr() },
                WsaBuf { len: 5, buf: b3.as_mut_ptr() },
                WsaBuf { len: 4, buf: b4.as_mut_ptr() },
            ];
            let mut recvd = 99u32;
            let mut fl = WIN_MSG_WAITALL;
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.h_event = ev;
            assert_eq!(
                WSARecv(
                    acc,
                    wbs.as_ptr(),
                    wbs.len() as u32,
                    &mut recvd,
                    &mut fl,
                    &mut ov as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            assert_eq!(WaitForSingleObject(ev, 50), WAIT_TIMEOUT);
            assert_no_iocp(iocp, 50);
            let mut res_bytes = 0u32;
            let mut res_fl = 0u32;
            assert_eq!(
                WSAGetOverlappedResult(acc, &mut ov, &mut res_bytes, FALSE, &mut res_fl),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSA_IO_INCOMPLETE as i32);

            // Stage intermediate chunks (still < 12 bytes): must remain pending on event & IOCP.
            assert_eq!(send(cli, b"BCDE".as_ptr(), 4, 0), 4);
            assert_eq!(WaitForSingleObject(ev, 60), WAIT_TIMEOUT);
            assert_no_iocp(iocp, 50);
            assert_eq!(send(cli, b"FGHI".as_ptr(), 4, 0), 4);
            assert_eq!(WaitForSingleObject(ev, 60), WAIT_TIMEOUT);
            assert_no_iocp(iocp, 50);

            // Final chunk completes the 12-byte scatter receive exactly once.
            assert_eq!(send(cli, b"JKL".as_ptr(), 3, 0), 3);
            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(key, 0x8201);
            assert_eq!(ov_out, &mut ov as *mut OVERLAPPED);
            assert_eq!(bytes, 12);
            assert_eq!(WaitForSingleObject(ev, 2000), WAIT_OBJECT_0);
            assert_eq!(
                WSAGetOverlappedResult(acc, &mut ov, &mut res_bytes, FALSE, &mut res_fl),
                TRUE
            );
            assert_eq!(res_bytes, 12);
            assert_eq!(&b1, b"ABC");
            assert_eq!(&b3, b"DEFGH");
            assert_eq!(&b4, b"IJKL");
            assert_no_iocp(iocp, 50);
            assert_eq!(CloseHandle(ev), TRUE);

            // 2. Empty-at-submission MSG_WAITALL progressing in the reactor + a second overlapped
            //    WSARecv queued behind the partially-progressed WAITALL receive stays strictly FIFO.
            let mut f1 = [0u8; 8];
            let mut f2 = [0u8; 4];
            let wb1 = WsaBuf { len: 8, buf: f1.as_mut_ptr() };
            let wb2 = WsaBuf { len: 4, buf: f2.as_mut_ptr() };
            let mut ov1: OVERLAPPED = std::mem::zeroed();
            let mut ov2: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecv(acc, &wb1, 1, &mut recvd, &mut fl, &mut ov1 as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            // Partial progress in reactor (3/8 bytes): still pending.
            assert_eq!(send(cli, b"123".as_ptr(), 3, 0), 3);
            assert_no_iocp(iocp, 60);

            // Submit second overlapped receive while ov1 is still pending at partial progress.
            let mut fl2 = 0u32;
            assert_eq!(
                WSARecv(acc, &wb2, 1, &mut recvd, &mut fl2, &mut ov2 as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            // Send remaining 5 bytes for ov1 + 4 bytes for ov2: ov1 must complete first with
            // full 8-byte payload, followed by ov2 with the trailing 4 bytes.
            assert_eq!(send(cli, b"45678WXYZ".as_ptr(), 9, 0), 9);
            let mut order = Vec::new();
            for _ in 0..2 {
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
                assert_eq!(key, 0x8201);
                order.push((ov_out as usize, bytes));
            }
            assert_eq!(
                order,
                vec![
                    (&mut ov1 as *mut OVERLAPPED as usize, 8),
                    (&mut ov2 as *mut OVERLAPPED as usize, 4),
                ]
            );
            assert_eq!(&f1, b"12345678");
            assert_eq!(&f2, b"WXYZ");
            assert_no_iocp(iocp, 50);

            // 3. Cancellation (CancelIoEx) at partial progress under MSG_WAITALL produces a single
            //    WSA_OPERATION_ABORTED completion.
            assert_eq!(send(cli, b"PQ".as_ptr(), 2, 0), 2);
            wait_readable(acc);
            let mut cbuf = [0u8; 8];
            let cwb = WsaBuf { len: 8, buf: cbuf.as_mut_ptr() };
            let mut ov_c: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecv(acc, &cwb, 1, &mut recvd, &mut fl, &mut ov_c as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            assert_no_iocp(iocp, 50);
            assert_eq!(CancelIoEx(acc as HANDLE, &mut ov_c), TRUE);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), FALSE);
            assert_eq!(ov_out, &mut ov_c as *mut OVERLAPPED);
            assert_eq!(key, 0x8201);
            assert_eq!(crate::teb::last_error(), WSA_OPERATION_ABORTED);
            assert_eq!(
                WSAGetOverlappedResult(acc, &mut ov_c, &mut res_bytes, FALSE, &mut res_fl),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSA_OPERATION_ABORTED as i32);
            assert_no_iocp(iocp, 50);

            // 4. Zero-length overlapped WSARecv with MSG_WAITALL remains a non-consuming readiness probe.
            let zero_wb = WsaBuf { len: 0, buf: std::ptr::null_mut() };
            let mut ov_z: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecv(acc, &zero_wb, 1, &mut recvd, &mut fl, &mut ov_z as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            assert_no_iocp(iocp, 50);
            assert_eq!(send(cli, b"Z".as_ptr(), 1, 0), 1);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov_z as *mut OVERLAPPED);
            assert_eq!(bytes, 0);
            let mut z_byte = [0u8; 1];
            assert_eq!(recv(acc, z_byte.as_mut_ptr(), 1, 0), 1);
            assert_eq!(&z_byte, b"Z");

            // Invalid overlapped flag combination still fails immediately with WSAEOPNOTSUPP.
            fl = WIN_MSG_WAITALL | WIN_MSG_PEEK;
            let mut ov_bad: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WSARecv(acc, &cwb, 1, &mut recvd, &mut fl, &mut ov_bad as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);

            // 5. Overlapped WSARecvFrom with MSG_WAITALL (leading byte + staged completion) and
            //    graceful partial EOF (completes with accumulated byte count).
            let (srv2, cli2, acc2) = make_tcp_pair();
            assert_eq!(CreateIoCompletionPort(acc2 as HANDLE, iocp, 0x8202, 0), iocp);

            assert_eq!(send(cli2, b"XY".as_ptr(), 2, 0), 2);
            wait_readable(acc2);
            let mut rf1 = [0u8; 2];
            let mut rf2 = [0u8; 4];
            let rfwbs = [
                WsaBuf { len: 2, buf: rf1.as_mut_ptr() },
                WsaBuf { len: 4, buf: rf2.as_mut_ptr() },
            ];
            let mut from_sa = [0u8; 32];
            let mut from_len = 32i32;
            let mut ov_rf: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecvFrom(
                    acc2,
                    rfwbs.as_ptr(),
                    rfwbs.len() as u32,
                    &mut recvd,
                    &mut fl,
                    from_sa.as_mut_ptr(),
                    &mut from_len,
                    &mut ov_rf as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            assert_no_iocp(iocp, 50);
            assert_eq!(send(cli2, b"Z123".as_ptr(), 4, 0), 4);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(key, 0x8202);
            assert_eq!(ov_out, &mut ov_rf as *mut OVERLAPPED);
            assert_eq!(bytes, 6);
            assert_eq!(&rf1, b"XY");
            assert_eq!(&rf2, b"Z123");
            assert_no_iocp(iocp, 50);

            // Graceful partial EOF on empty-at-submission overlapped WSARecv with MSG_WAITALL.
            let mut e1 = [0u8; 3];
            let mut e2 = [0u8; 7];
            let ewbs = [
                WsaBuf { len: 3, buf: e1.as_mut_ptr() },
                WsaBuf { len: 7, buf: e2.as_mut_ptr() },
            ];
            let mut ov_eof: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_WAITALL;
            assert_eq!(
                WSARecv(
                    acc2,
                    ewbs.as_ptr(),
                    ewbs.len() as u32,
                    &mut recvd,
                    &mut fl,
                    &mut ov_eof as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            assert_eq!(send(cli2, b"HE".as_ptr(), 2, 0), 2);
            assert_no_iocp(iocp, 60);
            assert_eq!(send(cli2, b"LLO".as_ptr(), 3, 0), 3);
            assert_eq!(closesocket(cli2), 0);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(key, 0x8202);
            assert_eq!(ov_out, &mut ov_eof as *mut OVERLAPPED);
            assert_eq!(bytes, 5);
            assert_eq!(
                WSAGetOverlappedResult(acc2, &mut ov_eof, &mut res_bytes, FALSE, &mut res_fl),
                TRUE
            );
            assert_eq!(res_bytes, 5);
            assert_eq!(&e1, b"HEL");
            assert_eq!(&e2[..2], b"LO");
            assert_no_iocp(iocp, 50);

            assert_eq!(closesocket(acc2), 0);
            assert_eq!(closesocket(srv2), 0);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn overlapped_wsa_recv_and_recvfrom_preserve_oob_flags_and_apc_arg4() {
        use crate::kernel32::file::{
            CreateIoCompletionPort, GetOverlappedResult, GetQueuedCompletionStatus,
            SetFileCompletionNotificationModes, INVALID_HANDLE_VALUE, OVERLAPPED,
        };
        use crate::kernel32::threads::CloseHandle;
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        static OOB_APC_CALLS: AtomicU32 = AtomicU32::new(0);
        static OOB_APC_ERR: AtomicU32 = AtomicU32::new(u32::MAX);
        static OOB_APC_BYTES: AtomicU32 = AtomicU32::new(0);
        static OOB_APC_OV: AtomicUsize = AtomicUsize::new(0);
        static OOB_APC_FLAGS: AtomicU32 = AtomicU32::new(u32::MAX);

        unsafe extern "win64" fn oob_completion_cb(
            err_code: u32,
            bytes: u32,
            ov: usize,
            flags: u32,
        ) {
            OOB_APC_ERR.store(err_code, Ordering::Release);
            OOB_APC_BYTES.store(bytes, Ordering::Release);
            OOB_APC_OV.store(ov, Ordering::Release);
            OOB_APC_FLAGS.store(flags, Ordering::Release);
            OOB_APC_CALLS.fetch_add(1, Ordering::AcqRel);
        }

        let reset_apc = || {
            OOB_APC_CALLS.store(0, Ordering::Release);
            OOB_APC_ERR.store(u32::MAX, Ordering::Release);
            OOB_APC_BYTES.store(0, Ordering::Release);
            OOB_APC_OV.store(0, Ordering::Release);
            OOB_APC_FLAGS.store(u32::MAX, Ordering::Release);
        };

        unsafe {
            let mut wd = [0u8; 512];
            assert_eq!(WSAStartup(0x0202, wd.as_mut_ptr()), 0);

            let make_tcp_pair = || -> (usize, usize, usize) {
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
                (srv, cli, acc)
            };

            let (srv, cli, acc) = make_tcp_pair();

            // Odd-aligned OVERLAPPED to keep unaligned-access safety covered too.
            let mut raw_ov = [0u8; 64];
            let ov = raw_ov.as_mut_ptr().add(1) as *mut OVERLAPPED;
            std::ptr::write_unaligned(ov, std::mem::zeroed());

            // 1. Delayed reactor OOB completion even when ordinary data is ALREADY buffered:
            //    ordinary data must NOT mask or prematurely complete the MSG_OOB receive.
            assert_eq!(send(cli, b"norm".as_ptr(), 4, 0), 4);
            let mut pfd = libc::pollfd {
                fd: lookup_fd(acc).unwrap(),
                events: libc::POLLIN,
                revents: 0,
            };
            assert!(libc::poll(&mut pfd, 1, 2000) > 0);

            reset_apc();
            let mut oob_buf = [0u8; 8];
            let wb = WsaBuf { len: oob_buf.len() as u32, buf: oob_buf.as_mut_ptr() };
            let mut recvd = 0u32;
            let mut fl = WIN_MSG_OOB;
            assert_eq!(
                WSARecv(
                    acc,
                    &wb,
                    1,
                    &mut recvd,
                    &mut fl,
                    ov as *mut c_void,
                    oob_completion_cb as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            // Still pending before urgent byte arrives (ordinary "norm" did not complete it).
            assert_eq!(SleepEx(40, TRUE), 0);
            assert_eq!(OOB_APC_CALLS.load(Ordering::Acquire), 0);

            // Now send an actual local urgent TCP byte.
            assert_eq!(send(cli, b"!".as_ptr(), 1, WIN_MSG_OOB as i32), 1);
            assert_eq!(SleepEx(2000, TRUE), crate::types::WAIT_IO_COMPLETION);
            assert_eq!(OOB_APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(OOB_APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(OOB_APC_BYTES.load(Ordering::Acquire), 1);
            assert_eq!(OOB_APC_OV.load(Ordering::Acquire), ov as usize);
            assert_eq!(OOB_APC_FLAGS.load(Ordering::Acquire), WIN_MSG_OOB);
            assert_eq!(&oob_buf[..1], b"!");

            // Repeated WSAGetOverlappedResult queries must consistently report TRUE + WIN_MSG_OOB.
            for _ in 0..2 {
                let mut xfer = 0u32;
                let mut out_fl = 0u32;
                assert_eq!(WSAGetOverlappedResult(acc, ov, &mut xfer, FALSE, &mut out_fl), TRUE);
                assert_eq!(xfer, 1);
                assert_eq!(out_fl, WIN_MSG_OOB);
            }
            let mut gor_n = 0u32;
            assert_eq!(GetOverlappedResult(acc as HANDLE, ov, &mut gor_n, FALSE), TRUE);
            assert_eq!(gor_n, 1);

            // 2. Reuse the EXACT SAME OVERLAPPED for an ordinary WSARecv (draining untouched "norm"):
            //    must return flags == 0 in BOTH callback arg4 and WSAGetOverlappedResult (no stale OOB).
            reset_apc();
            let mut norm_buf = [0u8; 8];
            let nwb = WsaBuf { len: norm_buf.len() as u32, buf: norm_buf.as_mut_ptr() };
            fl = 0;
            assert_eq!(
                WSARecv(
                    acc,
                    &nwb,
                    1,
                    &mut recvd,
                    &mut fl,
                    ov as *mut c_void,
                    oob_completion_cb as *mut c_void,
                ),
                0
            );
            assert_eq!(recvd, 4);
            assert_eq!(fl, 0);
            assert_eq!(&norm_buf[..4], b"norm");
            assert_eq!(SleepEx(2000, TRUE), crate::types::WAIT_IO_COMPLETION);
            assert_eq!(OOB_APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(OOB_APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(OOB_APC_BYTES.load(Ordering::Acquire), 4);
            assert_eq!(OOB_APC_FLAGS.load(Ordering::Acquire), 0);
            let mut xfer = 0u32;
            let mut out_fl = 0xDEAD;
            assert_eq!(WSAGetOverlappedResult(acc, ov, &mut xfer, FALSE, &mut out_fl), TRUE);
            assert_eq!(xfer, 4);
            assert_eq!(out_fl, 0);

            let send_oob_and_wait = |tx: usize, rx: usize, byte: u8| {
                let rx_fd = lookup_fd(rx).unwrap();
                let kq = libc::kqueue();
                assert!(kq >= 0);
                let ev_add = libc::kevent {
                    ident: rx_fd as usize,
                    filter: -15, // EVFILT_EXCEPT
                    flags: libc::EV_ADD | libc::EV_CLEAR,
                    fflags: 0x2, // NOTE_OOB
                    data: 0,
                    udata: std::ptr::null_mut(),
                };
                assert_eq!(
                    libc::kevent(kq, &ev_add, 1, std::ptr::null_mut(), 0, std::ptr::null()),
                    0
                );
                let zero_ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
                let mut out: libc::kevent = std::mem::zeroed();
                let _ = libc::kevent(kq, std::ptr::null(), 0, &mut out, 1, &zero_ts);
                assert_eq!(send(tx, &byte as *const u8, 1, WIN_MSG_OOB as i32), 1);
                let wait_ts = libc::timespec { tv_sec: 2, tv_nsec: 0 };
                let rc = libc::kevent(kq, std::ptr::null(), 0, &mut out, 1, &wait_ts);
                let _ = libc::close(kq);
                assert_eq!(rc, 1, "expected fresh OOB arrival on fd={rx_fd}");
            };

            // 3. Immediate OOB completion (urgent byte already buffered before submission):
            //    verify lpFlags, callback arg4, and WSAGetOverlappedResult all carry WIN_MSG_OOB.
            send_oob_and_wait(cli, acc, b'Q');

            reset_apc();
            oob_buf = [0u8; 8];
            fl = WIN_MSG_OOB;
            recvd = 0;
            assert_eq!(
                WSARecv(
                    acc,
                    &wb,
                    1,
                    &mut recvd,
                    &mut fl,
                    ov as *mut c_void,
                    oob_completion_cb as *mut c_void,
                ),
                0
            );
            assert_eq!(recvd, 1);
            assert_eq!(fl, WIN_MSG_OOB);
            assert_eq!(&oob_buf[..1], b"Q");
            assert_eq!(SleepEx(2000, TRUE), crate::types::WAIT_IO_COMPLETION);
            assert_eq!(OOB_APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(OOB_APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(OOB_APC_BYTES.load(Ordering::Acquire), 1);
            assert_eq!(OOB_APC_FLAGS.load(Ordering::Acquire), WIN_MSG_OOB);
            out_fl = 0;
            assert_eq!(WSAGetOverlappedResult(acc, ov, &mut xfer, FALSE, &mut out_fl), TRUE);
            assert_eq!(xfer, 1);
            assert_eq!(out_fl, WIN_MSG_OOB);

            // Reuse OVERLAPPED for WSASend -> clears stale OOB flag to 0.
            reset_apc();
            let swb = WsaBuf { len: 2, buf: b"hi".as_ptr() as *mut u8 };
            let mut sent = 0u32;
            assert_eq!(
                WSASend(acc, &swb, 1, &mut sent, 0, ov as *mut c_void, oob_completion_cb as *mut c_void),
                0
            );
            assert_eq!(SleepEx(2000, TRUE), crate::types::WAIT_IO_COMPLETION);
            assert_eq!(OOB_APC_FLAGS.load(Ordering::Acquire), 0);
            out_fl = 0xDEAD;
            assert_eq!(WSAGetOverlappedResult(acc, ov, &mut xfer, FALSE, &mut out_fl), TRUE);
            assert_eq!(out_fl, 0);

            // 4. IOCP / GetQueuedCompletionStatus + WSARecvFrom (delayed & immediate with
            //    FILE_SKIP_COMPLETION_PORT_ON_SUCCESS) treat STATUS_RECEIVE_EXPEDITED as success.
            let (srv2, cli2, acc2) = make_tcp_pair();
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());
            assert_eq!(CreateIoCompletionPort(acc2 as HANDLE, iocp, 0x9301, 0), iocp);

            let mut rf_buf = [0u8; 8];
            let rf_wb = WsaBuf { len: rf_buf.len() as u32, buf: rf_buf.as_mut_ptr() };
            let mut from_sa = [0u8; 32];
            let mut from_len = 32i32;
            let mut ov_rf: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_OOB;
            assert_eq!(
                WSARecvFrom(
                    acc2,
                    &rf_wb,
                    1,
                    &mut recvd,
                    &mut fl,
                    from_sa.as_mut_ptr(),
                    &mut from_len,
                    &mut ov_rf as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            assert_eq!(send(cli2, b"R".as_ptr(), 1, WIN_MSG_OOB as i32), 1);
            let mut gq_bytes = 0u32;
            let mut gq_key = 0usize;
            let mut gq_ov: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut gq_bytes, &mut gq_key, &mut gq_ov, 2000), TRUE);
            assert_eq!(gq_key, 0x9301);
            assert_eq!(gq_ov, &mut ov_rf as *mut OVERLAPPED);
            assert_eq!(gq_bytes, 1);
            assert_eq!(&rf_buf[..1], b"R");
            out_fl = 0;
            assert_eq!(WSAGetOverlappedResult(acc2, &mut ov_rf, &mut xfer, FALSE, &mut out_fl), TRUE);
            assert_eq!(xfer, 1);
            assert_eq!(out_fl, WIN_MSG_OOB);

            // Enable FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: immediate OOB completion succeeds
            // without queueing an IOCP packet, while WSAGetOverlappedResult still reports WIN_MSG_OOB.
            assert_eq!(SetFileCompletionNotificationModes(acc2 as HANDLE, 0x1), TRUE);
            send_oob_and_wait(cli2, acc2, b'S');
            let mut ov_skip: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_OOB;
            recvd = 0;
            assert_eq!(
                WSARecv(acc2, &rf_wb, 1, &mut recvd, &mut fl, &mut ov_skip as *mut _ as *mut c_void, std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 1);
            assert_eq!(fl, WIN_MSG_OOB);
            assert_eq!(&rf_buf[..1], b"S");
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut gq_bytes, &mut gq_key, &mut gq_ov, 50), FALSE);
            assert_eq!(crate::kernel32::process::GetLastError(), WAIT_TIMEOUT);
            out_fl = 0;
            assert_eq!(WSAGetOverlappedResult(acc2, &mut ov_skip, &mut xfer, FALSE, &mut out_fl), TRUE);
            assert_eq!(xfer, 1);
            assert_eq!(out_fl, WIN_MSG_OOB);

            // 5. Genuine unsupported/invalid MSG_OOB failures remain failures:
            //    SO_OOBINLINE=1 + MSG_OOB -> WSAEINVAL; UDP + MSG_OOB -> WSAEOPNOTSUPP.
            let one: i32 = 1;
            assert_eq!(
                setsockopt(acc2, 0xFFFF, 0x0100 /* SO_OOBINLINE */, &one as *const i32 as *const u8, 4),
                0
            );
            let mut ov_inv: OVERLAPPED = std::mem::zeroed();
            fl = WIN_MSG_OOB;
            assert_eq!(
                WSARecv(acc2, &rf_wb, 1, &mut recvd, &mut fl, &mut ov_inv as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEINVAL as i32);

            let udp = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(udp, INVALID_SOCKET);
            let usa = loopback_sa(0);
            assert_eq!(bind(udp, usa.as_ptr(), 16), 0);
            fl = WIN_MSG_OOB;
            assert_eq!(
                WSARecv(udp, &rf_wb, 1, &mut recvd, &mut fl, &mut ov_inv as *mut _ as *mut c_void, std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);

            assert_eq!(closesocket(udp), 0);
            assert_eq!(closesocket(acc2), 0);
            assert_eq!(closesocket(cli2), 0);
            assert_eq!(closesocket(srv2), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(cli), 0);
            assert_eq!(closesocket(srv), 0);
        }
    }

    #[test]
    fn host_proto_byteorder_and_enum_protocols_work() {
        unsafe {
            // gethostbyname on loopback literal & localhost
            let he_ptr = gethostbyname(c"127.0.0.1".as_ptr() as *const u8) as *const WinHostent;
            assert!(!he_ptr.is_null());
            assert_eq!((*he_ptr).h_addrtype, AF_INET as i16);
            assert_eq!((*he_ptr).h_length, 4);
            let first_addr = *(*he_ptr).h_addr_list;
            assert!(!first_addr.is_null());
            assert_eq!(std::slice::from_raw_parts(first_addr, 4), &[127, 0, 0, 1]);

            // gethostbyaddr on 127.0.0.1
            let ip = [127u8, 0, 0, 1];
            let he2 = gethostbyaddr(ip.as_ptr(), 4, AF_INET as i32) as *const WinHostent;
            assert!(!he2.is_null());
            assert!(!(*he2).h_name.is_null());

            // getprotobyname("tcp") / ("udp") / unknown
            let pe_tcp = getprotobyname(c"tcp".as_ptr() as *const u8) as *const WinProtoent;
            assert!(!pe_tcp.is_null());
            assert_eq!((*pe_tcp).p_proto, 6);
            let pe_udp = getprotobyname(c"udp".as_ptr() as *const u8) as *const WinProtoent;
            assert!(!pe_udp.is_null());
            assert_eq!((*pe_udp).p_proto, 17);
            assert!(getprotobyname(c"definitely_not_a_proto_xyz".as_ptr() as *const u8).is_null());
            assert_eq!(WSAGetLastError(), WSANO_DATA as i32);

            // WSAHtonl/WSAHtons/WSANtohl/WSANtohs + WSAENOTSOCK validation
            let s = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(s, INVALID_SOCKET);
            let mut nl = 0u32;
            let mut hl = 0u32;
            let mut ns = 0u16;
            let mut hs = 0u16;
            assert_eq!(WSAHtonl(s, 0x1234_5678, &mut nl), 0);
            assert_eq!(nl, 0x1234_5678u32.to_be());
            assert_eq!(WSANtohl(s, nl, &mut hl), 0);
            assert_eq!(hl, 0x1234_5678);
            assert_eq!(WSAHtons(s, 0x1234, &mut ns), 0);
            assert_eq!(ns, 0x1234u16.to_be());
            assert_eq!(WSANtohs(s, ns, &mut hs), 0);
            assert_eq!(hs, 0x1234);
            assert_eq!(WSAHtonl(0xDEAD_BEEF, 1, &mut nl), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAENOTSOCK as i32);

            // WSARecvDisconnect / WSASendDisconnect on connected loopback pair
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            assert_eq!(listen(srv, 1), 0);
            let mut bound = [0u8; 16];
            let mut blen = 16i32;
            assert_eq!(getsockname(srv, bound.as_mut_ptr(), &mut blen), 0);
            assert_eq!(connect(s, bound.as_ptr(), 16), 0);
            let acc = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(acc, INVALID_SOCKET);
            assert_eq!(WSASendDisconnect(s, std::ptr::null_mut()), 0);
            assert_eq!(WSARecvDisconnect(acc, std::ptr::null_mut()), 0);
            assert_eq!(closesocket(acc), 0);
            assert_eq!(closesocket(srv), 0);
            assert_eq!(closesocket(s), 0);

            // WSAEnumProtocolsA / W sizing + filter
            let mut len = 0u32;
            assert_eq!(WSAEnumProtocolsA(std::ptr::null(), std::ptr::null_mut(), &mut len), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAENOBUFS as i32);
            assert_eq!(len, 4 * 372);
            let mut buf_a = vec![0u8; len as usize];
            assert_eq!(WSAEnumProtocolsA(std::ptr::null(), buf_a.as_mut_ptr(), &mut len), 4);
            let tcp_filter = [libc::IPPROTO_TCP, 0];
            let mut len_w = 0u32;
            assert_eq!(WSAEnumProtocolsW(tcp_filter.as_ptr(), std::ptr::null_mut(), &mut len_w), SOCKET_ERROR);
            assert_eq!(len_w, 2 * 628);
            let mut buf_w = vec![0u8; len_w as usize];
            assert_eq!(WSAEnumProtocolsW(tcp_filter.as_ptr(), buf_w.as_mut_ptr(), &mut len_w), 2);
        }
    }

    #[test]
    fn wsarecvmsg_wsasendmsg_sync_overlapped_and_pktinfo() {
        use crate::kernel32::file::OVERLAPPED;

        unsafe {
            let _thunks = TestWsaExtThunksGuard::install();

            let rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(rx, INVALID_SOCKET);
            assert_ne!(tx, INVALID_SOCKET);

            // Verify WSAID_WSARECVMSG & WSAID_WSASENDMSG resolve via WSAIoctl.
            let mut fn_recvmsg = 0usize;
            let mut fn_sendmsg = 0usize;
            let mut br = 0u32;
            assert_eq!(
                WSAIoctl(
                    rx,
                    0xC800_0006, // SIO_GET_EXTENSION_FUNCTION_POINTER
                    WSAID_WSARECVMSG.as_ptr() as *mut c_void,
                    16,
                    &mut fn_recvmsg as *mut usize as *mut c_void,
                    8,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_ne!(fn_recvmsg, 0);
            assert_eq!(br, 8);
            assert_eq!(
                WSAIoctl(
                    tx,
                    0xC800_0006,
                    WSAID_WSASENDMSG.as_ptr() as *mut c_void,
                    16,
                    &mut fn_sendmsg as *mut usize as *mut c_void,
                    8,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_ne!(fn_sendmsg, 0);

            // Enable IP_PKTINFO on receiver so WSARecvMsg populates WSACMSGHDR.
            let one: i32 = 1;
            assert_eq!(
                setsockopt(rx, 0 /* IPPROTO_IP */, 19 /* IP_PKTINFO */, &one as *const i32 as *const u8, 4),
                0
            );
            let sa_bind = loopback_sa(0);
            assert_eq!(bind(rx, sa_bind.as_ptr(), 16), 0);
            let mut rx_sa = [0u8; 16];
            let mut rx_sa_len = 16i32;
            assert_eq!(getsockname(rx, rx_sa.as_mut_ptr(), &mut rx_sa_len), 0);

            // Scatter-gather send via WSASendMsg.
            let mut p1 = *b"hello ";
            let mut p2 = *b"wsamsg!";
            let mut send_bufs = [
                WsaBuf { len: p1.len() as u32, buf: p1.as_mut_ptr() },
                WsaBuf { len: p2.len() as u32, buf: p2.as_mut_ptr() },
            ];
            let send_msg = WsaMsg {
                name: rx_sa.as_mut_ptr(),
                namelen: 16,
                _pad0: 0,
                lp_buffers: send_bufs.as_mut_ptr(),
                dw_buffer_count: 2,
                _pad1: 0,
                control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                dw_flags: 0,
                _pad2: 0,
            };
            let mut sent = 0u32;
            assert_eq!(
                WSASendMsg(tx, &send_msg, 0, &mut sent, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(sent, 13);

            // Sync WSARecvMsg verifying payload, peer sockaddr, and IP_PKTINFO control message.
            let mut r1 = [0u8; 6];
            let mut r2 = [0u8; 16];
            let mut recv_bufs = [
                WsaBuf { len: r1.len() as u32, buf: r1.as_mut_ptr() },
                WsaBuf { len: r2.len() as u32, buf: r2.as_mut_ptr() },
            ];
            let mut from_sa = [0u8; 16];
            let mut ctrl = [0u8; 128];
            let mut recv_msg = WsaMsg {
                name: from_sa.as_mut_ptr(),
                namelen: 16,
                _pad0: 0,
                lp_buffers: recv_bufs.as_mut_ptr(),
                dw_buffer_count: 2,
                _pad1: 0,
                control: WsaBuf { len: ctrl.len() as u32, buf: ctrl.as_mut_ptr() },
                dw_flags: 0,
                _pad2: 0,
            };
            let mut recvd = 0u32;
            assert_eq!(
                WSARecvMsg(rx, &mut recv_msg, &mut recvd, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(recvd, 13);
            assert_eq!(&r1, b"hello ");
            assert_eq!(&r2[..7], b"wsamsg!");
            assert_eq!((from_sa.as_ptr() as *const u16).read_unaligned(), AF_INET);
            assert_eq!(&from_sa[4..8], &[127, 0, 0, 1]);
            assert!(recv_msg.control.len as usize >= std::mem::size_of::<WsaCmsgHdr>() + 8);
            let chdr = (ctrl.as_ptr() as *const WsaCmsgHdr).read_unaligned();
            assert_eq!(chdr.cmsg_level, WIN_IPPROTO_IP_CMSG);
            assert_eq!(chdr.cmsg_type, WIN_IP_PKTINFO_CMSG);
            let pktinfo = (ctrl.as_ptr().add(std::mem::size_of::<WsaCmsgHdr>()) as *const WinInPktinfo).read_unaligned();
            assert_eq!(pktinfo.ipi_addr, [127, 0, 0, 1]);

            // Overlapped WSARecvMsg pending -> completed when WSASendMsg sends.
            let ev = WSACreateEvent();
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.h_event = ev;
            let mut async_buf = [0u8; 32];
            let mut async_wb = WsaBuf { len: async_buf.len() as u32, buf: async_buf.as_mut_ptr() };
            let mut async_msg = WsaMsg {
                name: std::ptr::null_mut(),
                namelen: 0,
                _pad0: 0,
                lp_buffers: &mut async_wb,
                dw_buffer_count: 1,
                _pad1: 0,
                control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                dw_flags: 0,
                _pad2: 0,
            };
            let mut ov_recvd = 0u32;
            let rc = WSARecvMsg(
                rx,
                &mut async_msg,
                &mut ov_recvd,
                &mut ov as *mut _ as *mut c_void,
                std::ptr::null_mut(),
            );
            assert_eq!(rc, SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            let mut msg2 = *b"async-dgram";
            let mut wb2 = WsaBuf { len: msg2.len() as u32, buf: msg2.as_mut_ptr() };
            let sm2 = WsaMsg {
                name: rx_sa.as_mut_ptr(),
                namelen: 16,
                _pad0: 0,
                lp_buffers: &mut wb2,
                dw_buffer_count: 1,
                _pad1: 0,
                control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                dw_flags: 0,
                _pad2: 0,
            };
            assert_eq!(
                WSASendMsg(tx, &sm2, 0, &mut sent, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(WSAWaitForMultipleEvents(1, &ev, FALSE, 2000, FALSE), 0);
            let mut xfer = 0u32;
            let mut fl = 0u32;
            assert_eq!(WSAGetOverlappedResult(rx, &mut ov, &mut xfer, FALSE, &mut fl), TRUE);
            assert_eq!(xfer, 11);
            assert_eq!(&async_buf[..11], b"async-dgram");

            assert_eq!(WSACloseEvent(ev), TRUE);
            assert_eq!(closesocket(rx), 0);
            assert_eq!(closesocket(tx), 0);

            // Undersized WSAMSG.name buffer must fail with WSAEFAULT on both sync and overlapped WSARecvMsg.
            let rx6 = socket(AF_INET6 as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx6 = socket(AF_INET6 as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            if rx6 != INVALID_SOCKET && tx6 != INVALID_SOCKET {
                let mut bind6 = [0u8; 28];
                (bind6.as_mut_ptr() as *mut u16).write_unaligned(AF_INET6);
                bind6[23] = 1; // ::1
                if bind(rx6, bind6.as_ptr(), 28) == 0 {
                    let mut bound6 = [0u8; 28];
                    let mut blen6 = 28i32;
                    assert_eq!(getsockname(rx6, bound6.as_mut_ptr(), &mut blen6), 0);

                    // 1. Sync WSARecvMsg with 16-byte name buffer for IPv6 datagram -> SOCKET_ERROR + WSAEFAULT.
                    let mut p1 = *b"v6sync";
                    let mut wb1 = WsaBuf { len: p1.len() as u32, buf: p1.as_mut_ptr() };
                    let sm1 = WsaMsg {
                        name: bound6.as_mut_ptr(),
                        namelen: 28,
                        _pad0: 0,
                        lp_buffers: &mut wb1,
                        dw_buffer_count: 1,
                        _pad1: 0,
                        control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                        dw_flags: 0,
                        _pad2: 0,
                    };
                    let mut sent6 = 0u32;
                    assert_eq!(WSASendMsg(tx6, &sm1, 0, &mut sent6, std::ptr::null_mut(), std::ptr::null_mut()), 0);

                    let mut rbuf = [0u8; 32];
                    let mut rwb = WsaBuf { len: rbuf.len() as u32, buf: rbuf.as_mut_ptr() };
                    let mut small_name = [0xDDu8; 16];
                    let mut rmsg = WsaMsg {
                        name: small_name.as_mut_ptr(),
                        namelen: 16,
                        _pad0: 0,
                        lp_buffers: &mut rwb,
                        dw_buffer_count: 1,
                        _pad1: 0,
                        control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                        dw_flags: 0,
                        _pad2: 0,
                    };
                    let mut got6 = 99u32;
                    assert_eq!(
                        WSARecvMsg(rx6, &mut rmsg, &mut got6, std::ptr::null_mut(), std::ptr::null_mut()),
                        SOCKET_ERROR
                    );
                    assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                    assert_eq!(small_name, [0xDDu8; 16]);

                    // 2. Overlapped WSARecvMsg pending -> completed with WSAEFAULT when IPv6 datagram arrives.
                    let ev6 = WSACreateEvent();
                    let mut ov6: OVERLAPPED = std::mem::zeroed();
                    ov6.h_event = ev6;
                    let mut rmsg2 = WsaMsg {
                        name: small_name.as_mut_ptr(),
                        namelen: 16,
                        _pad0: 0,
                        lp_buffers: &mut rwb,
                        dw_buffer_count: 1,
                        _pad1: 0,
                        control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                        dw_flags: 0,
                        _pad2: 0,
                    };
                    let mut ov_got6 = 0u32;
                    assert_eq!(
                        WSARecvMsg(rx6, &mut rmsg2, &mut ov_got6, &mut ov6 as *mut _ as *mut c_void, std::ptr::null_mut()),
                        SOCKET_ERROR
                    );
                    assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
                    assert_eq!(WSASendMsg(tx6, &sm1, 0, &mut sent6, std::ptr::null_mut(), std::ptr::null_mut()), 0);
                    let mut xfer6 = 99u32;
                    let mut flags6 = 0u32;
                    assert_eq!(WSAGetOverlappedResult(rx6, &mut ov6, &mut xfer6, TRUE, &mut flags6), FALSE);
                    assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                    assert_eq!(WSACloseEvent(ev6), TRUE);
                }
            }
            if tx6 != INVALID_SOCKET {
                assert_eq!(closesocket(tx6), 0);
            }
            if rx6 != INVALID_SOCKET {
                assert_eq!(closesocket(rx6), 0);
            }
        }
    }

    #[test]
    fn overlapped_wsarecvmsg_preserves_msg_ctrunc_flags_across_apc_and_get_overlapped_result() {
        use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

        static APC_CALLS: AtomicUsize = AtomicUsize::new(0);
        static APC_LAST_ERR: AtomicU32 = AtomicU32::new(u32::MAX);
        static APC_LAST_BYTES: AtomicU32 = AtomicU32::new(0);
        static APC_LAST_FLAGS: AtomicU32 = AtomicU32::new(0);

        unsafe extern "win64" fn recvmsg_apc(
            dw_error: u32,
            cb_transferred: u32,
            _ov: usize,
            dw_flags: u32,
        ) {
            APC_LAST_ERR.store(dw_error, Ordering::Release);
            APC_LAST_BYTES.store(cb_transferred, Ordering::Release);
            APC_LAST_FLAGS.store(dw_flags, Ordering::Release);
            APC_CALLS.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            crate::register_native_va(
                recvmsg_apc as *const () as u64,
                recvmsg_apc as *const () as usize,
            );
            APC_CALLS.store(0, Ordering::Release);
            APC_LAST_ERR.store(u32::MAX, Ordering::Release);
            APC_LAST_BYTES.store(0, Ordering::Release);
            APC_LAST_FLAGS.store(0, Ordering::Release);

            let rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(rx, INVALID_SOCKET);
            assert_ne!(tx, INVALID_SOCKET);

            // Enable IP_PKTINFO so every received datagram produces ancillary control data.
            let one: i32 = 1;
            assert_eq!(
                setsockopt(rx, 0 /* IPPROTO_IP */, 19 /* IP_PKTINFO */, &one as *const i32 as *const u8, 4),
                0
            );
            let sa_bind = loopback_sa(0);
            assert_eq!(bind(rx, sa_bind.as_ptr(), 16), 0);
            let mut rx_sa = [0u8; 16];
            let mut rx_sa_len = 16i32;
            assert_eq!(getsockname(rx, rx_sa.as_mut_ptr(), &mut rx_sa_len), 0);

            let mut send_dgram = |payload: &[u8]| {
                let mut p = payload.to_vec();
                let mut wb = WsaBuf { len: p.len() as u32, buf: p.as_mut_ptr() };
                let sm = WsaMsg {
                    name: rx_sa.as_mut_ptr(),
                    namelen: 16,
                    _pad0: 0,
                    lp_buffers: &mut wb,
                    dw_buffer_count: 1,
                    _pad1: 0,
                    control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                    dw_flags: 0,
                    _pad2: 0,
                };
                let mut sent = 0u32;
                assert_eq!(
                    WSASendMsg(tx, &sm, 0, &mut sent, std::ptr::null_mut(), std::ptr::null_mut()),
                    0
                );
                assert_eq!(sent as usize, payload.len());
            };

            // 1. Pending overlapped WSARecvMsg with undersized Control.len (4 bytes) and completion
            //    routine: must report WIN_MSG_CTRUNC in WSAMSG.dw_flags, APC arg4, and repeated
            //    WSAGetOverlappedResult queries.
            let mut ov: OVERLAPPED = std::mem::zeroed();
            let mut rbuf = [0u8; 32];
            let mut rwb = WsaBuf { len: rbuf.len() as u32, buf: rbuf.as_mut_ptr() };
            let mut tiny_ctrl = [0u8; 4];
            let mut rmsg = WsaMsg {
                name: std::ptr::null_mut(),
                namelen: 0,
                _pad0: 0,
                lp_buffers: &mut rwb,
                dw_buffer_count: 1,
                _pad1: 0,
                control: WsaBuf { len: tiny_ctrl.len() as u32, buf: tiny_ctrl.as_mut_ptr() },
                dw_flags: 0,
                _pad2: 0,
            };
            let mut recvd = 0u32;
            assert_eq!(
                WSARecvMsg(
                    rx,
                    &mut rmsg,
                    &mut recvd,
                    &mut ov as *mut _ as *mut c_void,
                    recvmsg_apc as *const () as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);

            send_dgram(b"ctrunc-pend");
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
            while APC_CALLS.load(Ordering::Acquire) < 1 && std::time::Instant::now() < deadline {
                crate::kernel32::time::SleepEx(20, TRUE);
            }
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(APC_LAST_ERR.load(Ordering::Acquire), WSAEMSGSIZE);
            assert_eq!(APC_LAST_BYTES.load(Ordering::Acquire), 11);
            assert_ne!(
                APC_LAST_FLAGS.load(Ordering::Acquire) & WIN_MSG_CTRUNC,
                0,
                "pending WSARecvMsg completion routine must receive WIN_MSG_CTRUNC"
            );
            assert_ne!(rmsg.dw_flags & WIN_MSG_CTRUNC, 0);
            for _ in 0..2 {
                let mut xfer = 0u32;
                let mut fl = 0u32;
                assert_eq!(WSAGetOverlappedResult(rx, &mut ov, &mut xfer, FALSE, &mut fl), FALSE);
                assert_eq!(WSAGetLastError(), WSAEMSGSIZE as i32);
                assert_eq!(xfer, 11);
                assert_ne!(
                    fl & WIN_MSG_CTRUNC,
                    0,
                    "WSAGetOverlappedResult must preserve WIN_MSG_CTRUNC across calls"
                );
            }

            // 2. Immediate overlapped WSARecvMsg (datagram already queued) with undersized control
            //    buffer fails synchronously with WSAEMSGSIZE and sets WIN_MSG_CTRUNC without
            //    queuing a completion routine.
            send_dgram(b"ctrunc-imm");
            std::thread::sleep(std::time::Duration::from_millis(10));
            rmsg.control = WsaBuf { len: tiny_ctrl.len() as u32, buf: tiny_ctrl.as_mut_ptr() };
            rmsg.dw_flags = 0;
            assert_eq!(
                WSARecvMsg(
                    rx,
                    &mut rmsg,
                    &mut recvd,
                    &mut ov as *mut _ as *mut c_void,
                    recvmsg_apc as *const () as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEMSGSIZE as i32);
            assert_eq!(recvd, 10);
            assert_ne!(rmsg.dw_flags & WIN_MSG_CTRUNC, 0);
            crate::kernel32::time::SleepEx(0, TRUE);
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 1);

            // 3. Reusing the same OVERLAPPED with a full 128-byte control buffer must clear flags to 0
            //    (no stale WIN_MSG_CTRUNC carried over).
            let mut full_ctrl = [0u8; 128];
            rmsg.control = WsaBuf { len: full_ctrl.len() as u32, buf: full_ctrl.as_mut_ptr() };
            rmsg.dw_flags = 0;
            assert_eq!(
                WSARecvMsg(
                    rx,
                    &mut rmsg,
                    &mut recvd,
                    &mut ov as *mut _ as *mut c_void,
                    recvmsg_apc as *const () as *mut c_void,
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSA_IO_PENDING as i32);
            send_dgram(b"ctrl-ok");
            let deadline2 = std::time::Instant::now() + std::time::Duration::from_secs(2);
            while APC_CALLS.load(Ordering::Acquire) < 2 && std::time::Instant::now() < deadline2 {
                crate::kernel32::time::SleepEx(20, TRUE);
            }
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 2);
            assert_eq!(APC_LAST_ERR.load(Ordering::Acquire), 0);
            assert_eq!(APC_LAST_FLAGS.load(Ordering::Acquire), 0);
            assert_eq!(rmsg.dw_flags, 0);
            let mut xfer3 = 0u32;
            let mut fl3 = 0xFFFFu32;
            assert_eq!(WSAGetOverlappedResult(rx, &mut ov, &mut xfer3, FALSE, &mut fl3), TRUE);
            assert_eq!(xfer3, 7);
            assert_eq!(fl3, 0);

            assert_eq!(closesocket(rx), 0);
            assert_eq!(closesocket(tx), 0);
        }
    }

    #[test]
    fn wsarecvmsg_and_wsasendmsg_reject_null_control_or_name_with_nonzero_len_without_consuming_dgram() {
        unsafe {
            let rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(rx, INVALID_SOCKET);
            assert_ne!(tx, INVALID_SOCKET);

            let sa_bind = loopback_sa(0);
            assert_eq!(bind(rx, sa_bind.as_ptr(), 16), 0);
            let mut rx_sa = [0u8; 16];
            let mut rx_sa_len = 16i32;
            assert_eq!(getsockname(rx, rx_sa.as_mut_ptr(), &mut rx_sa_len), 0);

            // WSASendMsg with null Control.buf and nonzero Control.len must fail with WSAEFAULT.
            let mut payload = *b"keep-me";
            let mut swb = WsaBuf { len: payload.len() as u32, buf: payload.as_mut_ptr() };
            let bad_sm = WsaMsg {
                name: rx_sa.as_mut_ptr(),
                namelen: 16,
                _pad0: 0,
                lp_buffers: &mut swb,
                dw_buffer_count: 1,
                _pad1: 0,
                control: WsaBuf { len: 32, buf: std::ptr::null_mut() },
                dw_flags: 0,
                _pad2: 0,
            };
            let mut sent = 0u32;
            assert_eq!(
                WSASendMsg(tx, &bad_sm, 0, &mut sent, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);

            // Queue a valid datagram onto `rx`.
            let good_sm = WsaMsg {
                control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                ..bad_sm
            };
            assert_eq!(
                WSASendMsg(tx, &good_sm, 0, &mut sent, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(sent as usize, payload.len());
            std::thread::sleep(std::time::Duration::from_millis(10));

            // Sync and overlapped WSARecvMsg with null Control.buf + nonzero len (or null name +
            // nonzero namelen) must fail immediately with WSAEFAULT without consuming the datagram.
            let mut rbuf = [0u8; 32];
            let mut rwb = WsaBuf { len: rbuf.len() as u32, buf: rbuf.as_mut_ptr() };
            let mut bad_ctrl_msg = WsaMsg {
                name: std::ptr::null_mut(),
                namelen: 0,
                _pad0: 0,
                lp_buffers: &mut rwb,
                dw_buffer_count: 1,
                _pad1: 0,
                control: WsaBuf { len: 32, buf: std::ptr::null_mut() },
                dw_flags: 0,
                _pad2: 0,
            };
            let mut recvd = 0u32;
            assert_eq!(
                WSARecvMsg(rx, &mut bad_ctrl_msg, &mut recvd, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);

            let mut ov: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WSARecvMsg(
                    rx,
                    &mut bad_ctrl_msg,
                    &mut recvd,
                    &mut ov as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);

            let mut bad_name_msg = WsaMsg {
                name: std::ptr::null_mut(),
                namelen: 16,
                control: WsaBuf { len: 0, buf: std::ptr::null_mut() },
                ..bad_ctrl_msg
            };
            assert_eq!(
                WSARecvMsg(rx, &mut bad_name_msg, &mut recvd, std::ptr::null_mut(), std::ptr::null_mut()),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);

            // Retrying with a valid control buffer must still receive the original queued datagram.
            let mut ctrl = [0u8; 64];
            let mut good_msg = WsaMsg {
                name: std::ptr::null_mut(),
                namelen: 0,
                control: WsaBuf { len: ctrl.len() as u32, buf: ctrl.as_mut_ptr() },
                ..bad_ctrl_msg
            };
            assert_eq!(
                WSARecvMsg(rx, &mut good_msg, &mut recvd, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(&rbuf[..recvd as usize], b"keep-me");

            assert_eq!(closesocket(rx), 0);
            assert_eq!(closesocket(tx), 0);
        }
    }

    #[test]
    fn wsaconnect_by_name_a_and_w_loopback() {
        unsafe {
            let srv = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(srv, INVALID_SOCKET);
            let sa = loopback_sa(0);
            assert_eq!(bind(srv, sa.as_ptr(), 16), 0);
            assert_eq!(listen(srv, 4), 0);
            let mut bound = [0u8; 16];
            let mut blen = 16i32;
            assert_eq!(getsockname(srv, bound.as_mut_ptr(), &mut blen), 0);
            let port = u16::from_be((bound.as_ptr().add(2) as *const u16).read_unaligned());
            let port_str = format!("{port}\0");
            let port_w = crate::strings::to_wide_z(&port.to_string());
            let host_w = crate::strings::to_wide_z("127.0.0.1");

            // WSAConnectByNameA
            let c1 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut loc = [0u8; 28];
            let mut loc_len = 28u32;
            let mut rem = [0u8; 28];
            let mut rem_len = 28u32;
            let tv = WinTimeVal { tv_sec: 2, tv_usec: 0 };
            assert_eq!(
                WSAConnectByNameA(
                    c1,
                    b"127.0.0.1\0".as_ptr(),
                    port_str.as_ptr(),
                    &mut loc_len,
                    loc.as_mut_ptr(),
                    &mut rem_len,
                    rem.as_mut_ptr(),
                    &tv,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            assert_eq!(loc_len, 16);
            assert_eq!(rem_len, 16);
            assert_eq!(&rem[4..8], &[127, 0, 0, 1]);
            let a1 = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(a1, INVALID_SOCKET);
            assert_eq!(send(c1, b"ok".as_ptr(), 2, 0), 2);
            let mut buf = [0u8; 8];
            assert_eq!(recv(a1, buf.as_mut_ptr(), 8, 0), 2);
            assert_eq!(&buf[..2], b"ok");

            // WSAConnectByNameW
            let c2 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut loc_len2 = 28u32;
            let mut rem_len2 = 28u32;
            assert_eq!(
                WSAConnectByNameW(
                    c2,
                    host_w.as_ptr(),
                    port_w.as_ptr(),
                    &mut loc_len2,
                    loc.as_mut_ptr(),
                    &mut rem_len2,
                    rem.as_mut_ptr(),
                    &tv,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            let a2 = accept(srv, std::ptr::null_mut(), std::ptr::null_mut());
            assert_ne!(a2, INVALID_SOCKET);

            // Invalid host fails with FALSE.
            let c3 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_eq!(
                WSAConnectByNameA(
                    c3,
                    b"nonexistent.invalid.\0".as_ptr(),
                    port_str.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &tv,
                    std::ptr::null_mut(),
                ),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSAHOST_NOT_FOUND as i32);

            // Undersized address output buffers fail with WSAEFAULT and report required size.
            let c4 = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            let mut small_rem = [0xAAu8; 8];
            let mut small_rem_len = 8u32;
            assert_eq!(
                WSAConnectByNameA(
                    c4,
                    b"127.0.0.1\0".as_ptr(),
                    port_str.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut small_rem_len,
                    small_rem.as_mut_ptr(),
                    &tv,
                    std::ptr::null_mut(),
                ),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
            assert_eq!(small_rem_len, 16);
            assert_eq!(small_rem, [0xAAu8; 8]);

            let mut small_loc = [0xBBu8; 8];
            let mut small_loc_len = 8u32;
            assert_eq!(
                WSAConnectByNameA(
                    c4,
                    b"127.0.0.1\0".as_ptr(),
                    port_str.as_ptr(),
                    &mut small_loc_len,
                    small_loc.as_mut_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &tv,
                    std::ptr::null_mut(),
                ),
                FALSE
            );
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
            assert_eq!(small_loc_len, 16);
            assert_eq!(small_loc, [0xBBu8; 8]);

            // IPv6 socket with 16-byte remote buffer (too small for 28-byte sockaddr_in6) -> WSAEFAULT + 28.
            let c6 = socket(AF_INET6 as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            if c6 != INVALID_SOCKET {
                let mut v6_rem = [0xCCu8; 16];
                let mut v6_rem_len = 16u32;
                assert_eq!(
                    WSAConnectByNameA(
                        c6,
                        b"::1\0".as_ptr(),
                        port_str.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &mut v6_rem_len,
                        v6_rem.as_mut_ptr(),
                        &tv,
                        std::ptr::null_mut(),
                    ),
                    FALSE
                );
                assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
                assert_eq!(v6_rem_len, 28);
                assert_eq!(v6_rem, [0xCCu8; 16]);
                assert_eq!(closesocket(c6), 0);
            }
            assert_eq!(closesocket(c4), 0);

            assert_eq!(closesocket(c3), 0);
            assert_eq!(closesocket(a2), 0);
            assert_eq!(closesocket(c2), 0);
            assert_eq!(closesocket(a1), 0);
            assert_eq!(closesocket(c1), 0);
            assert_eq!(closesocket(srv), 0);
        }
    }

    #[test]
    fn wsa_async_select_posts_window_messages() {
        use crate::user32::class::{RegisterClassW, UnregisterClassW, WNDCLASS};
        use crate::user32::defproc::DefWindowProcW;
        use crate::user32::message::{PeekMessageW, PM_REMOVE};
        use crate::user32::window::{CreateWindowExW, DestroyWindow};
        use crate::user32::MSG;

        unsafe {
            // Register a real window class and create a hidden Win32 window on this thread.
            let cls = crate::strings::to_wide_z("Ws2AsyncSelectTestClass");
            let mut wc: WNDCLASS = std::mem::zeroed();
            wc.wndproc = crate::user32::register_typed_host_wndproc(DefWindowProcW);
            wc.class_name = cls.as_ptr() as usize;
            let atom = RegisterClassW(&wc);
            assert_ne!(atom, 0);
            let title = crate::strings::to_wide_z("ws2-async-test");
            let hwnd = CreateWindowExW(
                0,
                cls.as_ptr(),
                title.as_ptr(),
                0,
                0,
                0,
                0,
                0,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());

            let rx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let tx = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            let sa = loopback_sa(0);
            assert_eq!(bind(rx, sa.as_ptr(), 16), 0);
            let mut bound = [0u8; 16];
            let mut blen = 16i32;
            assert_eq!(getsockname(rx, bound.as_mut_ptr(), &mut blen), 0);

            // Invalid HWND -> WSAEINVAL.
            assert_eq!(WSAAsyncSelect(rx, std::ptr::null_mut(), 0x0401, FD_READ), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEINVAL as i32);

            // Subscribe rx for FD_READ with WM_USER + 7.
            const WM_SOCK: u32 = 0x0407;
            assert_eq!(WSAAsyncSelect(rx, hwnd, WM_SOCK, FD_READ), 0);
            assert!(is_guest_nonblocking(rx));

            // Send datagram and wait for WM_SOCK in the thread's message queue.
            assert_eq!(sendto(tx, b"ping".as_ptr(), 4, 0, bound.as_ptr(), 16), 4);
            let mut msg: MSG = std::mem::zeroed();
            let mut got = false;
            for _ in 0..100 {
                if PeekMessageW(&mut msg, hwnd, WM_SOCK, WM_SOCK, PM_REMOVE) != 0 {
                    got = true;
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            assert!(got, "expected WM_SOCK FD_READ message from WSAAsyncSelect");
            assert_eq!(msg.wparam, rx);
            assert_eq!((msg.lparam as u32) & 0xFFFF, FD_READ as u32);
            assert_eq!(((msg.lparam as u32) >> 16) & 0xFFFF, 0);

            // Cancel notifications with lEvent == 0.
            assert_eq!(WSAAsyncSelect(rx, hwnd, 0, 0), 0);
            assert_eq!(closesocket(rx), 0);
            assert_eq!(closesocket(tx), 0);
            assert_eq!(DestroyWindow(hwnd), TRUE);
            let _ = UnregisterClassW(cls.as_ptr(), std::ptr::null_mut());
        }
    }

    #[test]
    fn wsa_ioctl_extensions_joinleaf_qos_cancel_and_dup() {
        unsafe {
            let s = socket(AF_INET as i32, libc::SOCK_STREAM, libc::IPPROTO_TCP);
            assert_ne!(s, INVALID_SOCKET);

            // 1. SIO_KEEPALIVE_VALS
            let ka = WinTcpKeepalive {
                onoff: 1,
                keepalivetime: 30_000,
                keepaliveinterval: 5_000,
            };
            let mut br = 0u32;
            assert_eq!(
                WSAIoctl(
                    s,
                    0x9800_0004, // SIO_KEEPALIVE_VALS
                    &ka as *const _ as *mut c_void,
                    std::mem::size_of::<WinTcpKeepalive>() as u32,
                    std::ptr::null_mut(),
                    0,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            let mut ka_enabled = 0i32;
            let mut ka_len = 4i32;
            assert_eq!(getsockopt(s, 0xFFFF, 0x0008 /* SO_KEEPALIVE */, &mut ka_enabled as *mut i32 as *mut u8, &mut ka_len), 0);
            assert_ne!(ka_enabled, 0);

            // 2. SIOCATMARK (both ioctlsocket and WSAIoctl)
            let mut atmark = 99u32;
            assert_eq!(ioctlsocket(s, 0x4004_7307u32 as i32, &mut atmark), 0);
            assert_eq!(atmark, 0);
            let mut atmark2 = 99u32;
            assert_eq!(
                WSAIoctl(
                    s,
                    0x4004_7307,
                    std::ptr::null_mut(),
                    0,
                    &mut atmark2 as *mut u32 as *mut c_void,
                    4,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_eq!(br, 4);
            assert_eq!(atmark2, 0);

            // 3. SIO_GET_INTERFACE_LIST
            let mut if_buf = [0u8; 76 * 32];
            assert_eq!(
                WSAIoctl(
                    s,
                    0x4004_747F,
                    std::ptr::null_mut(),
                    0,
                    if_buf.as_mut_ptr() as *mut c_void,
                    if_buf.len() as u32,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            assert!(br >= 76 && br % 76 == 0);
            let count = (br / 76) as usize;
            let has_loopback = (0..count).any(|i| {
                let info = (if_buf.as_ptr().add(i * 76) as *const WinInterfaceInfo).read_unaligned();
                (info.flags & 0x05) == 0x05 && &info.address[4..8] == &[127, 0, 0, 1]
            });
            assert!(has_loopback, "expected 127.0.0.1 loopback in SIO_GET_INTERFACE_LIST");

            // 4. SIO_ADDRESS_LIST_QUERY
            let mut addr_buf = [0u8; 4096];
            assert_eq!(
                WSAIoctl(
                    s,
                    0x4800_0016,
                    std::ptr::null_mut(),
                    0,
                    addr_buf.as_mut_ptr() as *mut c_void,
                    addr_buf.len() as u32,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            let addr_count = (addr_buf.as_ptr() as *const i32).read_unaligned();
            assert!(addr_count >= 1);

            // 5. SIO_ROUTING_INTERFACE_QUERY for 127.0.0.1
            let dst_sa = loopback_sa(80);
            let mut route_out = [0u8; 28];
            assert_eq!(
                WSAIoctl(
                    s,
                    0xC800_0014,
                    dst_sa.as_ptr() as *mut c_void,
                    16,
                    route_out.as_mut_ptr() as *mut c_void,
                    28,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                0
            );
            assert_eq!(br, 16);
            assert_eq!(&route_out[4..8], &[127, 0, 0, 1]);

            // 6. Unknown WSAIoctl code -> WSAEOPNOTSUPP
            assert_eq!(
                WSAIoctl(
                    s,
                    0xDEAD_BEEF,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    0,
                    &mut br,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SOCKET_ERROR
            );
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);

            // 7. WSAGetQOSByName: validates args then returns FALSE + WSAEOPNOTSUPP (Windows Vista+ behavior)
            let mut qname = WsaBuf { len: 0, buf: std::ptr::null_mut() };
            let mut qos = [0u8; 72];
            assert_eq!(WSAGetQOSByName(0xDEAD_BEEF, &mut qname, qos.as_mut_ptr() as *mut c_void), FALSE);
            assert_eq!(WSAGetLastError(), WSAENOTSOCK as i32);
            assert_eq!(WSAGetQOSByName(s, std::ptr::null_mut(), qos.as_mut_ptr() as *mut c_void), FALSE);
            assert_eq!(WSAGetLastError(), WSAEFAULT as i32);
            assert_eq!(WSAGetQOSByName(s, &mut qname, qos.as_mut_ptr() as *mut c_void), FALSE);
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);

            // 8. WSACancelAsyncRequest: unknown/null -> WSAEINVAL, pending -> 0, completed -> WSAEALREADY
            assert_eq!(WSACancelAsyncRequest(std::ptr::null_mut()), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEINVAL as i32);
            assert_eq!(WSACancelAsyncRequest(0x12345usize as HANDLE), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEINVAL as i32);
            ASYNC_TASKS.lock().unwrap().insert(0x7001, AsyncTaskState::Pending);
            ASYNC_TASKS.lock().unwrap().insert(0x7002, AsyncTaskState::Completed);
            assert_eq!(WSACancelAsyncRequest(0x7001usize as HANDLE), 0);
            assert_eq!(WSACancelAsyncRequest(0x7002usize as HANDLE), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEALREADY as i32);

            // 9. WSADuplicateSocketW cross-process -> WSAEOPNOTSUPP
            let mut pinfo = [0u8; 628];
            assert_eq!(WSADuplicateSocketW(s, std::process::id().wrapping_add(9999), pinfo.as_mut_ptr()), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAEOPNOTSUPP as i32);

            // 10. WSAJoinLeaf on UDP multicast group
            let mcast = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(mcast, INVALID_SOCKET);
            let reuse: i32 = 1;
            assert_eq!(setsockopt(mcast, 0xFFFF, 0x0004 /* SO_REUSEADDR */, &reuse as *const i32 as *const u8, 4), 0);
            let any_sa = [AF_INET as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            assert_eq!(bind(mcast, any_sa.as_ptr(), 16), 0);
            let mut grp = [0u8; 16];
            (grp.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            (grp.as_mut_ptr().add(2) as *mut u16).write_unaligned(5353u16.to_be());
            grp[4..8].copy_from_slice(&[239, 255, 1, 1]);
            // Invalid flags -> WSAEINVAL
            assert_eq!(
                WSAJoinLeaf(mcast, grp.as_ptr(), 16, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), 0),
                INVALID_SOCKET
            );
            assert_eq!(WSAGetLastError(), WSAEINVAL as i32);
            // Valid multicast join (JL_RECEIVER_ONLY) returns same socket handle `mcast`.
            let joined = WSAJoinLeaf(
                mcast,
                grp.as_ptr(),
                16,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                JL_RECEIVER_ONLY,
            );
            assert_eq!(joined, mcast);

            assert_eq!(closesocket(mcast), 0);
            assert_eq!(closesocket(s), 0);
        }
    }

    #[test]
    fn wsa_join_leaf_jl_both_keeps_socket_unconnected_and_sends_to_group() {
        unsafe {
            let mcast = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(mcast, INVALID_SOCKET);
            let reuse: i32 = 1;
            assert_eq!(
                setsockopt(mcast, 0xFFFF, 0x0004 /* SO_REUSEADDR */, &reuse as *const i32 as *const u8, 4),
                0
            );
            let bind_sa = loopback_sa(0);
            assert_eq!(bind(mcast, bind_sa.as_ptr(), 16), 0);

            let mut local_sa = [0u8; 16];
            let mut local_len = 16i32;
            assert_eq!(getsockname(mcast, local_sa.as_mut_ptr(), &mut local_len), 0);
            let bound_port = u16::from_be((local_sa.as_ptr().add(2) as *const u16).read_unaligned());
            assert_ne!(bound_port, 0);

            // Configure multicast interface to loopback so send() to 239.255.1.2 routes locally.
            let lo_if = [127u8, 0, 0, 1];
            assert_eq!(
                setsockopt(mcast, 0 /* IPPROTO_IP */, 9 /* IP_MULTICAST_IF */, lo_if.as_ptr(), 4),
                0
            );
            let loop_en: i32 = 1;
            assert_eq!(
                setsockopt(mcast, 0 /* IPPROTO_IP */, 11 /* IP_MULTICAST_LOOP */, &loop_en as *const i32 as *const u8, 4),
                0
            );

            let mut grp = [0u8; 16];
            (grp.as_mut_ptr() as *mut u16).write_unaligned(AF_INET);
            (grp.as_mut_ptr().add(2) as *mut u16).write_unaligned(bound_port.to_be());
            grp[4..8].copy_from_slice(&[239, 255, 1, 2]);
            let joined = WSAJoinLeaf(
                mcast,
                grp.as_ptr(),
                16,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                JL_BOTH,
            );
            assert_eq!(joined, mcast);

            // Socket must remain unconnected on Darwin so incoming packets from unicast senders
            // are never filtered out by connect(2) peer matching.
            let mut peer_sa = [0u8; 16];
            let mut peer_len = 16i32;
            assert_eq!(getpeername(mcast, peer_sa.as_mut_ptr(), &mut peer_len), SOCKET_ERROR);
            assert_eq!(WSAGetLastError(), WSAENOTCONN as i32);

            // Unicast peer sends a datagram to `mcast`: must be received despite JL_BOTH.
            let sender = socket(AF_INET as i32, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
            assert_ne!(sender, INVALID_SOCKET);
            let dst = loopback_sa(bound_port);
            assert_eq!(sendto(sender, b"ping".as_ptr(), 4, 0, dst.as_ptr(), 16), 4);
            let mut rbuf = [0u8; 16];
            let mut from_sa = [0u8; 16];
            let mut from_len = 16i32;
            let got = recvfrom(mcast, rbuf.as_mut_ptr(), rbuf.len() as i32, 0, from_sa.as_mut_ptr(), &mut from_len);
            assert_eq!(got, 4);
            assert_eq!(&rbuf[..4], b"ping");

            // send() and WSASend() without explicit destination must use the stored multicast group.
            assert_eq!(send(mcast, b"grp1".as_ptr(), 4, 0), 4);
            let mut wbuf = WsaBuf { len: 4, buf: b"grp2".as_ptr() as *mut u8 };
            let mut sent = 0u32;
            assert_eq!(
                WSASend(mcast, &mut wbuf, 1, &mut sent, 0, std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            assert_eq!(sent, 4);

            assert_eq!(closesocket(sender), 0);
            assert_eq!(closesocket(mcast), 0);
        }
    }
}
