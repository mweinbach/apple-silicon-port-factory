//! Translate the Win64 WSAMSG / WSACMSGHDR layouts at the Darwin sendmsg boundary.
use super::*;
use std::mem::size_of;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WsaMsg {
    name: *mut u8,
    name_len: i32,
    buffers: *const WsaBuf,
    count: u32,
    control: WsaBuf,
    flags: u32,
}
const _: () = assert!(size_of::<WsaMsg>() == 56);

#[derive(Clone, Copy)]
pub(super) enum PacketInfo {
    V4(libc::in_pktinfo),
    V6(libc::in6_pktinfo),
}

impl PacketInfo {
    unsafe fn parse(buf: WsaBuf) -> Result<Option<Self>, u32> {
        if buf.len == 0 {
            return Ok(None);
        }
        if buf.buf.is_null() || buf.len < 16 {
            return Err(WSAEFAULT);
        }
        let p = buf.buf;
        let len = p.cast::<usize>().read_unaligned();
        let level = p.add(8).cast::<i32>().read_unaligned();
        let kind = p.add(12).cast::<i32>().read_unaligned();
        if len < 16 || len > buf.len as usize {
            return Err(WSAEINVAL);
        }
        if kind != 19 || buf.len as usize > len.next_multiple_of(8) {
            return Err(WSAEOPNOTSUPP);
        }
        let data = p.add(16);
        match level {
            libc::IPPROTO_IP if len == 24 => {
                let mut v: libc::in_pktinfo = std::mem::zeroed();
                v.ipi_spec_dst = data.cast::<libc::in_addr>().read_unaligned();
                v.ipi_addr = v.ipi_spec_dst;
                v.ipi_ifindex = data.add(4).cast::<u32>().read_unaligned();
                Ok(Some(Self::V4(v)))
            }
            libc::IPPROTO_IPV6 if len == 36 => Ok(Some(Self::V6(
                data.cast::<libc::in6_pktinfo>().read_unaligned(),
            ))),
            _ => Err(WSAEINVAL),
        }
    }

    pub(super) unsafe fn attach(&self, msg: &mut libc::msghdr, storage: &mut [u32; 8]) {
        let (level, kind, ptr, bytes) = match self {
            Self::V4(v) => (
                libc::IPPROTO_IP,
                libc::IP_PKTINFO,
                (v as *const libc::in_pktinfo).cast::<u8>(),
                size_of::<libc::in_pktinfo>(),
            ),
            Self::V6(v) => (
                libc::IPPROTO_IPV6,
                libc::IPV6_PKTINFO,
                (v as *const libc::in6_pktinfo).cast::<u8>(),
                size_of::<libc::in6_pktinfo>(),
            ),
        };
        msg.msg_control = storage.as_mut_ptr().cast();
        msg.msg_controllen = libc::CMSG_SPACE(bytes as u32);
        let hdr = libc::CMSG_FIRSTHDR(msg);
        (*hdr).cmsg_level = level;
        (*hdr).cmsg_type = kind;
        (*hdr).cmsg_len = libc::CMSG_LEN(bytes as u32);
        std::ptr::copy_nonoverlapping(ptr, libc::CMSG_DATA(hdr), bytes);
    }
}

pub(super) unsafe fn received(msg: &mut libc::msghdr, output: *mut WsaMsg) {
    let control = std::ptr::addr_of!((*output).control).read_unaligned();
    let mut used = 0usize;
    let mut flags = if msg.msg_flags & libc::MSG_TRUNC != 0 {
        0x100
    } else {
        0
    };
    let mut hdr = libc::CMSG_FIRSTHDR(msg);
    while !hdr.is_null() {
        let level = (*hdr).cmsg_level;
        let data = libc::CMSG_DATA(hdr);
        let (addr, ifindex, bytes) = match (level, (*hdr).cmsg_type) {
            (libc::IPPROTO_IP, libc::IP_PKTINFO)
                if (*hdr).cmsg_len >= libc::CMSG_LEN(size_of::<libc::in_pktinfo>() as u32) =>
            {
                let v = data.cast::<libc::in_pktinfo>();
                let ip = u32::from_be((*v).ipi_addr.s_addr);
                if ip == u32::MAX {
                    flags |= 0x400;
                } else if ip >> 28 == 14 {
                    flags |= 0x800;
                }
                (
                    std::ptr::addr_of!((*v).ipi_addr).cast::<u8>(),
                    (*v).ipi_ifindex,
                    4,
                )
            }
            (libc::IPPROTO_IPV6, libc::IPV6_PKTINFO)
                if (*hdr).cmsg_len >= libc::CMSG_LEN(size_of::<libc::in6_pktinfo>() as u32) =>
            {
                let v = data.cast::<libc::in6_pktinfo>();
                if (*v).ipi6_addr.s6_addr[0] == 0xff {
                    flags |= 0x800;
                }
                (
                    std::ptr::addr_of!((*v).ipi6_addr).cast::<u8>(),
                    (*v).ipi6_ifindex,
                    16,
                )
            }
            _ => {
                hdr = libc::CMSG_NXTHDR(msg, hdr);
                continue;
            }
        };
        let len = 16 + bytes + 4;
        if used + len > control.len as usize {
            msg.msg_flags |= libc::MSG_CTRUNC;
            break;
        }
        let p = control.buf.add(used);
        p.cast::<usize>().write_unaligned(len);
        p.add(8).cast::<i32>().write_unaligned(level);
        p.add(12).cast::<i32>().write_unaligned(19); // IP_PKTINFO and IPV6_PKTINFO
        std::ptr::copy_nonoverlapping(addr, p.add(16), bytes);
        p.add(16 + bytes).cast::<u32>().write_unaligned(ifindex);
        used += len.next_multiple_of(8).min(control.len as usize - used);
        hdr = libc::CMSG_NXTHDR(msg, hdr);
    }
    if msg.msg_flags & libc::MSG_CTRUNC != 0 {
        flags |= 0x200;
    }
    std::ptr::addr_of_mut!((*output).flags).write_unaligned(flags);
    std::ptr::addr_of_mut!((*output).control.len).write_unaligned(used as u32);
}

pub(super) unsafe fn send(
    s: usize,
    ptr: *const WsaMsg,
    flags: u32,
    sent: *mut u32,
    ov: *mut c_void,
    cb: *mut c_void,
) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if !matches!(meta.sock_type, libc::SOCK_DGRAM | libc::SOCK_RAW)
        || flags & !(WIN_MSG_DONTROUTE | WIN_MSG_OOB) != 0
    {
        return err(WSAEOPNOTSUPP, SOCKET_ERROR);
    }
    if ptr.is_null() || (ov.is_null() && sent.is_null()) {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let m = ptr.read_unaligned();
    let control = match PacketInfo::parse(m.control) {
        Ok(c) => c,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    send_to(
        s, m.buffers, m.count, sent, flags, m.name, m.name_len, ov, cb, control,
    )
}

pub(super) unsafe fn recv(
    s: usize,
    ptr: *mut WsaMsg,
    bytes: *mut u32,
    ov: *mut c_void,
    cb: *mut c_void,
) -> i32 {
    let Some(meta) = lookup_meta(s) else {
        return err(WSAENOTSOCK, SOCKET_ERROR);
    };
    if !matches!(meta.sock_type, libc::SOCK_DGRAM | libc::SOCK_RAW) {
        return err(WSAEOPNOTSUPP, SOCKET_ERROR);
    }
    if ptr.is_null() || (ov.is_null() && bytes.is_null()) {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let m = ptr.read_unaligned();
    if m.flags & !WIN_MSG_PEEK != 0 {
        return err(WSAEOPNOTSUPP, SOCKET_ERROR);
    }
    let addr_size = if meta.family == libc::AF_INET6 {
        28
    } else {
        16
    };
    if (m.control.len != 0 && m.control.buf.is_null())
        || (!m.name.is_null() && m.name_len < addr_size)
    {
        return err(WSAEFAULT, SOCKET_ERROR);
    }
    let bufs = match collect_wsa_bufs(m.buffers, m.count) {
        Ok(b) => b,
        Err(e) => return err(e, SOCKET_ERROR),
    };
    let pflags = win_msg_flags_to_posix(m.flags);
    let len = std::ptr::addr_of_mut!((*ptr).name_len);
    if ov.is_null() {
        sync_wsa_recv(
            s,
            meta.fd,
            &bufs,
            bytes,
            std::ptr::null_mut(),
            m.name,
            len,
            false,
            pflags,
            ptr as usize,
        )
    } else {
        submit_overlapped_recv(
            s,
            meta.fd,
            bufs,
            bytes,
            std::ptr::null_mut(),
            m.name,
            len,
            false,
            pflags,
            ov.cast(),
            cb,
            ptr as usize,
        )
    }
}
