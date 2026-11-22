//! ws2_32.dll: Winsock2 over POSIX / BSD sockets.
//!
//! Translates Windows address families (`AF_INET6 = 23` <-> Darwin `30`),
//! `sockaddr_in`/`sockaddr_in6` headers, `ADDRINFOA`/`ADDRINFOW` layouts,
//! socket options (`SOL_SOCKET`, `IPPROTO_TCP`, `IPPROTO_IPV6`, timeouts, linger),
//! `ioctlsocket`/`WSAIoctl`, `select`/`WSAPoll`, and errno -> WSAE* error codes.

use crate::types::*;
use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::{Condvar, LazyLock, Mutex};

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

#[derive(Clone, Copy)]
struct SocketMeta {
    fd: i32,
    sock_type: i32,
    listening: bool,
    connecting: bool,
    connected: bool,
}

static STARTUPS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static NEXT_SOCK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x10000);
static SOCKETS: LazyLock<Mutex<HashMap<usize, SocketMeta>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
const DUP_SOCK_MAGIC_MASK: u32 = 0xD05C_0000;
static NEXT_DUP_TOKEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
static DUP_SOCKETS: LazyLock<Mutex<HashMap<u32, (i32, i32, bool)>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

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

fn register_fd(fd: i32, sock_type: i32, connected: bool) -> usize {
    // Suppress SIGPIPE on macOS sockets.
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
    let h = NEXT_SOCK.fetch_add(4, std::sync::atomic::Ordering::Relaxed);
    SOCKETS.lock().unwrap().insert(
        h,
        SocketMeta {
            fd,
            sock_type,
            listening: false,
            connecting: false,
            connected,
        },
    );
    h
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
    register_fd(fd, ty, false)
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
    let (fd, ty, connected) = DUP_SOCKETS.lock().unwrap().remove(&token)?;
    Some(register_fd(fd, ty, connected))
}

pub unsafe extern "win64" fn WSASocketW(
    af: i32,
    ty: i32,
    proto: i32,
    info: *const c_void,
    _g: u32,
    _flags: u32,
) -> usize {
    if let Some(s) = try_open_duplicated_socket(info) {
        return s;
    }
    socket(af, ty, proto)
}

pub unsafe extern "win64" fn WSASocketA(
    af: i32,
    ty: i32,
    proto: i32,
    info: *const c_void,
    _g: u32,
    _flags: u32,
) -> usize {
    if let Some(s) = try_open_duplicated_socket(info) {
        return s;
    }
    socket(af, ty, proto)
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
    DUP_SOCKETS.lock().unwrap().insert(token, (dup_fd, meta.sock_type, meta.connected));

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
    event_select_remove_and_sync(s);
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
    let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let cfd = libc::accept(meta.fd, &mut storage as *mut libc::sockaddr_storage as *mut libc::sockaddr, &mut slen);
    event_select_rearm_accept(s);
    if cfd < 0 {
        return err(last_errno_wsa(), INVALID_SOCKET);
    }
    if !addr.is_null() && !addrlen.is_null() {
        let _ = posix_to_win_sockaddr(&storage as *const libc::sockaddr_storage as *const libc::sockaddr, slen, addr, addrlen);
    }
    let child = register_fd(cfd, meta.sock_type, true);
    event_select_inherit_accept(s, child, cfd, meta.sock_type);
    child
}

pub unsafe extern "win64" fn connect(s: usize, name: *const u8, namelen: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let (storage, plen) = match win_to_posix_sockaddr(name, namelen) {
        Ok(v) => v,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    if libc::connect(fd, &storage as *const libc::sockaddr_storage as *const libc::sockaddr, plen) != 0 {
        let raw_errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL);
        let wsa = errno_to_wsa(raw_errno);
        if raw_errno == libc::EINPROGRESS {
            if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
                m.connecting = true;
                m.connected = false;
            }
            event_select_on_connect_pending(s);
        } else {
            event_select_on_connect_complete(s, wsa as i32);
        }
        return err(wsa, SOCKET_ERROR);
    }
    if let Some(m) = SOCKETS.lock().unwrap().get_mut(&s) {
        m.connecting = false;
        m.connected = true;
    }
    event_select_on_connect_complete(s, 0);
    0
}

pub unsafe extern "win64" fn shutdown(s: usize, how: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if libc::shutdown(fd, how) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
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
    let rc = match cmd as u32 {
        WIN_FIONBIO => libc::ioctl(fd, libc::FIONBIO, &mut val),
        WIN_FIONREAD => libc::ioctl(fd, libc::FIONREAD, &mut val),
        _ => return err(WSAEINVAL, SOCKET_ERROR),
    };
    if rc != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    std::ptr::write_unaligned(argp, val as u32);
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
    _overlapped: *mut c_void,
    _completion: *mut c_void,
) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    const SIO_UDP_CONNRESET: u32 = 0x9800_000C;
    const SIO_UDP_NETRESET: u32 = 0x9800_000F;
    const SIO_IDEAL_SEND_BACKLOG_QUERY: u32 = 0x4004_747B;
    const WIN_FIONBIO: u32 = 0x8004_667E;
    const WIN_FIONREAD: u32 = 0x4004_667F;
    if !bytes_returned.is_null() {
        std::ptr::write_unaligned(bytes_returned, 0);
    }
    match io_control_code {
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
    let n = libc::send(fd, buf as *const c_void, len.max(0) as usize, flags);
    if n < 0 {
        let wsa = last_errno_wsa();
        if wsa == WSAEWOULDBLOCK {
            event_select_rearm_write(s);
        }
        return err(wsa, SOCKET_ERROR);
    }
    n as i32
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
    let n = libc::sendto(
        fd,
        buf as *const c_void,
        len.max(0) as usize,
        flags,
        &storage as *const libc::sockaddr_storage as *const libc::sockaddr,
        plen,
    );
    if n < 0 {
        let wsa = last_errno_wsa();
        if wsa == WSAEWOULDBLOCK {
            event_select_rearm_write(s);
        }
        return err(wsa, SOCKET_ERROR);
    }
    n as i32
}

pub unsafe extern "win64" fn recv(s: usize, buf: *mut u8, len: i32, flags: i32) -> i32 {
    let Some(fd) = lookup_fd(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    let n = libc::recv(fd, buf as *mut c_void, len.max(0) as usize, flags);
    event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
    if n < 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
    n as i32
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
    let mut slen = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let n = libc::recvfrom(
        fd,
        buf as *mut c_void,
        len.max(0) as usize,
        flags,
        if from.is_null() { std::ptr::null_mut() } else { &mut storage as *mut libc::sockaddr_storage as *mut libc::sockaddr },
        if from.is_null() { std::ptr::null_mut() } else { &mut slen },
    );
    event_select_rearm_read(s, (flags & libc::MSG_OOB) != 0);
    if n < 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
    }
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

pub unsafe extern "win64" fn WSASend(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    sent: *mut u32,
    flags: u32,
    _overlapped: *mut c_void,
    _completion: *mut c_void,
) -> i32 {
    if buffers.is_null() || count == 0 {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let mut total = 0u32;
    for i in 0..count as usize {
        let wb = std::ptr::read_unaligned(buffers.add(i));
        let r = send(s, wb.buf, wb.len as i32, flags as i32);
        if r < 0 {
            return SOCKET_ERROR;
        }
        total += r as u32;
    }
    if !sent.is_null() {
        std::ptr::write_unaligned(sent, total);
    }
    0
}

pub unsafe extern "win64" fn WSASendTo(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    sent: *mut u32,
    flags: u32,
    to: *const u8,
    tolen: i32,
    _overlapped: *mut c_void,
    _completion: *mut c_void,
) -> i32 {
    if buffers.is_null() || count == 0 {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let mut flat = Vec::new();
    for i in 0..count as usize {
        let wb = std::ptr::read_unaligned(buffers.add(i));
        if !wb.buf.is_null() && wb.len > 0 {
            flat.extend_from_slice(std::slice::from_raw_parts(wb.buf, wb.len as usize));
        }
    }
    let r = sendto(s, flat.as_ptr(), flat.len() as i32, flags as i32, to, tolen);
    if r < 0 {
        return SOCKET_ERROR;
    }
    if !sent.is_null() {
        std::ptr::write_unaligned(sent, r as u32);
    }
    0
}

pub unsafe extern "win64" fn WSARecv(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    recvd: *mut u32,
    flags: *mut u32,
    _overlapped: *mut c_void,
    _completion: *mut c_void,
) -> i32 {
    if buffers.is_null() || count == 0 {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let fl = if flags.is_null() { 0 } else { std::ptr::read_unaligned(flags) as i32 };
    let wb = std::ptr::read_unaligned(buffers);
    let r = recv(s, wb.buf, wb.len as i32, fl);
    if r < 0 {
        return SOCKET_ERROR;
    }
    if !recvd.is_null() {
        std::ptr::write_unaligned(recvd, r as u32);
    }
    0
}

pub unsafe extern "win64" fn WSARecvFrom(
    s: usize,
    buffers: *const WsaBuf,
    count: u32,
    recvd: *mut u32,
    flags: *mut u32,
    from: *mut u8,
    fromlen: *mut i32,
    _overlapped: *mut c_void,
    _completion: *mut c_void,
) -> i32 {
    if buffers.is_null() || count == 0 {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let fl = if flags.is_null() { 0 } else { std::ptr::read_unaligned(flags) as i32 };
    let wb = std::ptr::read_unaligned(buffers);
    let r = recvfrom(s, wb.buf, wb.len as i32, fl, from, fromlen);
    if r < 0 {
        return SOCKET_ERROR;
    }
    if !recvd.is_null() {
        std::ptr::write_unaligned(recvd, r as u32);
    }
    0
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

// @exports WSAAsyncSelect WSACancelAsyncRequest WSAConnectByNameA WSAConnectByNameW WSAEnumProtocolsA WSAEnumProtocolsW WSAGetOverlappedResult WSAGetQOSByName WSAHtonl WSAHtons WSAJoinLeaf WSANtohl WSANtohs WSARecvDisconnect WSASendDisconnect WSARecvMsg WSASendMsg
failing!(i32 = SOCKET_ERROR, err WSAEOPNOTSUPP;
    WSAAsyncSelect, WSACancelAsyncRequest, WSAConnectByNameA,
    WSAConnectByNameW,
    WSAEnumProtocolsA, WSAEnumProtocolsW,
    WSAGetOverlappedResult, WSAGetQOSByName, WSAHtonl, WSAHtons, WSAJoinLeaf,
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
    let mut nb: i32 = 1;
    unsafe {
        libc::ioctl(child_fd, libc::FIONBIO, &mut nb);
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
    let mut nb: i32 = 1;
    if libc::ioctl(meta.fd, libc::FIONBIO, &mut nb) != 0 {
        return err(last_errno_wsa(), SOCKET_ERROR);
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
}
