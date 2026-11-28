//! iphlpapi.dll: IP Helper API backed by POSIX `getifaddrs()` network interface enumeration.

use crate::types::*;
use std::ffi::{c_void, CStr};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_SUCCESS: u32 = 0;
const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_BUFFER_OVERFLOW: u32 = 111;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const ERROR_NO_DATA: u32 = 232;

// Windows address families
const WIN_AF_UNSPEC: u16 = 0;
const WIN_AF_INET: u16 = 2;
const WIN_AF_INET6: u16 = 23;

// Windows MIB interface types (<ipifcons.h>)
const IF_TYPE_OTHER: u32 = 1;
const IF_TYPE_ETHERNET_CSMACD: u32 = 6;
const IF_TYPE_PPP: u32 = 23;
const IF_TYPE_SOFTWARE_LOOPBACK: u32 = 24;
const IF_TYPE_IEEE80211: u32 = 71;

// IF_OPER_STATUS (GetAdaptersAddresses)
const IF_OPER_STATUS_UP: u32 = 1;
const IF_OPER_STATUS_DOWN: u32 = 2;

// INTERNAL_IF_OPER_STATUS (MIB_IFROW.dwOperStatus)
const MIB_IF_OPER_STATUS_NON_OPERATIONAL: u32 = 0;
const MIB_IF_OPER_STATUS_OPERATIONAL: u32 = 5;

// GetAdaptersAddresses flags
const GAA_FLAG_SKIP_UNICAST: u32 = 0x0001;

// IP_ADAPTER_ADDRESSES_LH flags
const IP_ADAPTER_NO_MULTICAST: u32 = 0x0010;
const IP_ADAPTER_IPV4_ENABLED: u32 = 0x0080;
const IP_ADAPTER_IPV6_ENABLED: u32 = 0x0100;

// Struct sizes on Windows x64 (LLP64)
const IP_ADAPTER_ADDRESSES_LH_SIZE: usize = 448;
const IP_ADAPTER_UNICAST_ADDRESS_LH_SIZE: usize = 64;
const IP_ADAPTER_INFO_SIZE: usize = 704;
const IP_ADDR_STRING_SIZE: usize = 48;
const MIB_IFROW_SIZE: usize = 860;
const MIB_IPADDRROW_SIZE: usize = 24;
const MIB_IPFORWARDROW_SIZE: usize = 56;
const MIB_IPROUTE_TYPE_DIRECT: u32 = 3;
const MIB_IPROUTE_TYPE_INDIRECT: u32 = 4;
const MIB_IPROUTE_PROTO_LOCAL: u32 = 2;
const MIB_IPROUTE_PROTO_NETMGMT: u32 = 3;

#[derive(Clone, Debug)]
struct HostIpv4 {
    addr: [u8; 4],
    mask: [u8; 4],
    prefix_len: u8,
}

#[derive(Clone, Debug)]
struct HostIpv6 {
    addr: [u8; 16],
    scope_id: u32,
    prefix_len: u8,
}

#[derive(Clone, Debug)]
struct HostInterface {
    name: String,
    index: u32,
    flags: u32,
    mac: [u8; 8],
    mac_len: usize,
    if_type: u32,
    mtu: u32,
    ipv4: Vec<HostIpv4>,
    ipv6: Vec<HostIpv6>,
}

impl HostInterface {
    fn is_up(&self) -> bool {
        (self.flags & (libc::IFF_UP as u32)) != 0
    }

    fn is_loopback(&self) -> bool {
        (self.flags & (libc::IFF_LOOPBACK as u32)) != 0 || self.if_type == IF_TYPE_SOFTWARE_LOOPBACK
    }
}

fn align_up(v: usize, align: usize) -> usize {
    (v + align - 1) & !(align - 1)
}

fn classify_if_type(flags: u32, sdl_type: u8, mac_len: usize) -> u32 {
    if (flags & (libc::IFF_LOOPBACK as u32)) != 0 || sdl_type == 24 {
        IF_TYPE_SOFTWARE_LOOPBACK
    } else if (flags & (libc::IFF_POINTOPOINT as u32)) != 0 || sdl_type == 23 {
        IF_TYPE_PPP
    } else if sdl_type == 71 {
        IF_TYPE_IEEE80211
    } else if sdl_type == 6 || mac_len == 6 {
        IF_TYPE_ETHERNET_CSMACD
    } else {
        IF_TYPE_OTHER
    }
}

fn enumerate_host_interfaces() -> Vec<HostInterface> {
    let mut head: *mut libc::ifaddrs = std::ptr::null_mut();
    let rc = unsafe { libc::getifaddrs(&mut head) };
    if rc != 0 || head.is_null() {
        return Vec::new();
    }

    let mut ifaces: Vec<HostInterface> = Vec::new();
    let mut cur = head;
    while !cur.is_null() {
        unsafe {
            let ifa = &*cur;
            if !ifa.ifa_name.is_null() {
                let name = CStr::from_ptr(ifa.ifa_name).to_string_lossy().into_owned();
                let flags = ifa.ifa_flags;
                let idx_pos = if let Some(pos) = ifaces.iter().position(|x| x.name == name) {
                    ifaces[pos].flags |= flags;
                    pos
                } else {
                    let os_idx = libc::if_nametoindex(ifa.ifa_name);
                    let if_type = classify_if_type(flags, 0, 0);
                    let mtu = if if_type == IF_TYPE_SOFTWARE_LOOPBACK { 16384 } else { 1500 };
                    ifaces.push(HostInterface {
                        name,
                        index: os_idx,
                        flags,
                        mac: [0u8; 8],
                        mac_len: 0,
                        if_type,
                        mtu,
                        ipv4: Vec::new(),
                        ipv6: Vec::new(),
                    });
                    ifaces.len() - 1
                };

                if !ifa.ifa_addr.is_null() {
                    let sa = ifa.ifa_addr as *const u8;
                    let sa_len = (*ifa.ifa_addr).sa_len as usize;
                    let family = (*ifa.ifa_addr).sa_family as i32;

                    if family == libc::AF_LINK && sa_len >= 8 {
                        // Darwin sockaddr_dl: sdl_len@0, sdl_family@1, sdl_index@2 (u16),
                        // sdl_type@4, sdl_nlen@5, sdl_alen@6, sdl_slen@7, sdl_data@8.
                        let sdl_index = (sa.add(2) as *const u16).read_unaligned() as u32;
                        let sdl_type = sa.add(4).read_unaligned();
                        let sdl_nlen = sa.add(5).read_unaligned() as usize;
                        let sdl_alen = sa.add(6).read_unaligned() as usize;
                        let iface = &mut ifaces[idx_pos];
                        if iface.index == 0 && sdl_index != 0 {
                            iface.index = sdl_index;
                        }
                        if sdl_alen > 0 && 8 + sdl_nlen + sdl_alen <= sa_len {
                            let copy_len = sdl_alen.min(8);
                            std::ptr::copy_nonoverlapping(
                                sa.add(8 + sdl_nlen),
                                iface.mac.as_mut_ptr(),
                                copy_len,
                            );
                            iface.mac_len = copy_len;
                        }
                        iface.if_type = classify_if_type(iface.flags, sdl_type, iface.mac_len);
                        iface.mtu = if iface.if_type == IF_TYPE_SOFTWARE_LOOPBACK {
                            16384
                        } else {
                            1500
                        };
                    } else if family == libc::AF_INET && sa_len >= 8 {
                        let mut addr = [0u8; 4];
                        std::ptr::copy_nonoverlapping(sa.add(4), addr.as_mut_ptr(), 4);
                        let mut mask = [0u8; 4];
                        if !ifa.ifa_netmask.is_null() {
                            let mlen = (*ifa.ifa_netmask).sa_len as usize;
                            let avail = mlen.saturating_sub(4).min(4);
                            if avail > 0 {
                                std::ptr::copy_nonoverlapping(
                                    (ifa.ifa_netmask as *const u8).add(4),
                                    mask.as_mut_ptr(),
                                    avail,
                                );
                            }
                        }
                        let prefix_len = u32::from_be_bytes(mask).leading_ones() as u8;
                        ifaces[idx_pos].ipv4.push(HostIpv4 { addr, mask, prefix_len });
                    } else if family == libc::AF_INET6 && sa_len >= 24 {
                        let mut addr = [0u8; 16];
                        std::ptr::copy_nonoverlapping(sa.add(8), addr.as_mut_ptr(), 16);
                        let mut scope_id = if sa_len >= 28 {
                            (sa.add(24) as *const u32).read_unaligned()
                        } else {
                            0
                        };
                        // Strip KAME embedded interface index in link-local IPv6 (fe80::/10).
                        if addr[0] == 0xFE && (addr[1] & 0xC0) == 0x80 {
                            let embedded = u16::from_be_bytes([addr[2], addr[3]]) as u32;
                            if scope_id == 0 && embedded != 0 {
                                scope_id = embedded;
                            }
                            addr[2] = 0;
                            addr[3] = 0;
                        }
                        let mut mask = [0u8; 16];
                        if !ifa.ifa_netmask.is_null() {
                            let mlen = (*ifa.ifa_netmask).sa_len as usize;
                            let avail = mlen.saturating_sub(8).min(16);
                            if avail > 0 {
                                std::ptr::copy_nonoverlapping(
                                    (ifa.ifa_netmask as *const u8).add(8),
                                    mask.as_mut_ptr(),
                                    avail,
                                );
                            }
                        }
                        let prefix_len: u8 = mask.iter().map(|b| b.count_ones() as u8).sum();
                        ifaces[idx_pos].ipv6.push(HostIpv6 { addr, scope_id, prefix_len });
                    }
                }
            }
            cur = ifa.ifa_next;
        }
    }

    unsafe {
        libc::freeifaddrs(head);
    }

    for (i, iface) in ifaces.iter_mut().enumerate() {
        if iface.index == 0 {
            iface.index = (i + 1) as u32;
        }
    }
    ifaces
}

fn format_ipv4(a: [u8; 4]) -> String {
    format!("{}.{}.{}.{}", a[0], a[1], a[2], a[3])
}

fn ipv4_match_prefix(dest: [u8; 4], ip: &HostIpv4) -> Option<u8> {
    let d = u32::from_be_bytes(dest);
    let a = u32::from_be_bytes(ip.addr);
    let m = u32::from_be_bytes(ip.mask);
    if m != 0 && (d & m) == (a & m) {
        Some(ip.prefix_len)
    } else {
        None
    }
}

fn ipv6_match_prefix(dest: [u8; 16], ip: &HostIpv6) -> Option<u8> {
    let plen = ip.prefix_len.min(128) as usize;
    if plen == 0 {
        return None;
    }
    let full_bytes = plen / 8;
    if dest[..full_bytes] != ip.addr[..full_bytes] {
        return None;
    }
    let rem_bits = plen % 8;
    if rem_bits > 0 {
        let bitmask = !((1u8 << (8 - rem_bits)) - 1);
        if (dest[full_bytes] & bitmask) != (ip.addr[full_bytes] & bitmask) {
            return None;
        }
    }
    Some(ip.prefix_len)
}

fn best_interface_v4(ifaces: &[HostInterface], dest: [u8; 4]) -> Option<u32> {
    if ifaces.is_empty() {
        return None;
    }
    // Loopback destination 127.0.0.0/8 prefers an up loopback interface.
    if dest[0] == 127 {
        if let Some(lo) = ifaces.iter().find(|i| i.is_up() && i.is_loopback()) {
            return Some(lo.index);
        }
    }
    // Longest matching subnet prefix on an UP interface.
    let mut best_match: Option<(u8, u32)> = None;
    for iface in ifaces.iter().filter(|i| i.is_up()) {
        for v4 in &iface.ipv4 {
            if let Some(plen) = ipv4_match_prefix(dest, v4) {
                if best_match.map_or(true, |(bp, _)| plen > bp) {
                    best_match = Some((plen, iface.index));
                }
            }
        }
    }
    if let Some((_, idx)) = best_match {
        return Some(idx);
    }
    // Default route heuristic: first UP non-loopback interface with a non-APIPA IPv4 address.
    if let Some(iface) = ifaces.iter().find(|i| {
        i.is_up()
            && !i.is_loopback()
            && i.ipv4.iter().any(|v| !(v.addr[0] == 169 && v.addr[1] == 254))
    }) {
        return Some(iface.index);
    }
    if let Some(iface) = ifaces.iter().find(|i| i.is_up() && !i.is_loopback() && !i.ipv4.is_empty()) {
        return Some(iface.index);
    }
    if let Some(iface) = ifaces.iter().find(|i| i.is_up() && !i.ipv4.is_empty()) {
        return Some(iface.index);
    }
    ifaces.iter().find(|i| i.is_up()).or_else(|| ifaces.first()).map(|i| i.index)
}

fn best_interface_v6(ifaces: &[HostInterface], dest: [u8; 16], scope_id: u32) -> Option<u32> {
    if ifaces.is_empty() {
        return None;
    }
    if scope_id != 0 {
        if let Some(iface) = ifaces.iter().find(|i| i.index == scope_id) {
            return Some(iface.index);
        }
    }
    // ::1 loopback
    if dest == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] {
        if let Some(lo) = ifaces.iter().find(|i| i.is_up() && i.is_loopback()) {
            return Some(lo.index);
        }
    }
    let mut best_match: Option<(u8, u32)> = None;
    for iface in ifaces.iter().filter(|i| i.is_up()) {
        for v6 in &iface.ipv6 {
            if let Some(plen) = ipv6_match_prefix(dest, v6) {
                if best_match.map_or(true, |(bp, _)| plen > bp) {
                    best_match = Some((plen, iface.index));
                }
            }
        }
    }
    if let Some((_, idx)) = best_match {
        return Some(idx);
    }
    if let Some(iface) = ifaces.iter().find(|i| i.is_up() && !i.is_loopback() && !i.ipv6.is_empty()) {
        return Some(iface.index);
    }
    if let Some(iface) = ifaces.iter().find(|i| i.is_up() && !i.ipv6.is_empty()) {
        return Some(iface.index);
    }
    ifaces.iter().find(|i| i.is_up()).or_else(|| ifaces.first()).map(|i| i.index)
}

/// Enumerates host network adapters via `libc::getifaddrs()` and packs linked
/// `IP_ADAPTER_ADDRESSES_LH` + `IP_ADAPTER_UNICAST_ADDRESS_LH` + Windows `sockaddr`
/// + name strings directly inside the caller-supplied buffer.
pub unsafe extern "win64" fn GetAdaptersAddresses(
    family: u32,
    flags: u32,
    _res: *mut c_void,
    out: *mut c_void,
    size: *mut u32,
) -> u32 {
    if size.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let fam = family as u16;
    if fam != WIN_AF_UNSPEC && fam != WIN_AF_INET && fam != WIN_AF_INET6 {
        return ERROR_INVALID_PARAMETER;
    }

    let all = enumerate_host_interfaces();
    let include_v4 = fam == WIN_AF_UNSPEC || fam == WIN_AF_INET;
    let include_v6 = fam == WIN_AF_UNSPEC || fam == WIN_AF_INET6;
    let skip_unicast = (flags & GAA_FLAG_SKIP_UNICAST) != 0;

    let adapters: Vec<&HostInterface> = all
        .iter()
        .filter(|iface| {
            if fam == WIN_AF_INET {
                !iface.ipv4.is_empty()
            } else if fam == WIN_AF_INET6 {
                !iface.ipv6.is_empty()
            } else {
                true
            }
        })
        .collect();

    if adapters.is_empty() {
        size.write_unaligned(0);
        return ERROR_NO_DATA;
    }

    // Pass 1: compute exact packed buffer size.
    struct AdapterLayout {
        node_off: usize,
        name_ascii_off: usize,
        dns_suffix_off: usize,
        friendly_off: usize,
        unicast_entries: Vec<(usize, usize, bool)>, // (unicast_off, sockaddr_off, is_v6)
    }

    let mut cur = 0usize;
    let mut layouts = Vec::with_capacity(adapters.len());
    for iface in &adapters {
        cur = align_up(cur, 8);
        let node_off = cur;
        cur += IP_ADAPTER_ADDRESSES_LH_SIZE;

        let name_ascii_off = cur;
        cur += iface.name.len() + 1;

        cur = align_up(cur, 2);
        let dns_suffix_off = cur;
        cur += 2; // L""

        let friendly_units = iface.name.encode_utf16().count() + 1;
        cur = align_up(cur, 2);
        let friendly_off = cur;
        cur += friendly_units * 2;

        let mut unicast_entries = Vec::new();
        if !skip_unicast {
            if include_v4 {
                for _ in &iface.ipv4 {
                    cur = align_up(cur, 8);
                    let u_off = cur;
                    cur += IP_ADAPTER_UNICAST_ADDRESS_LH_SIZE;
                    cur = align_up(cur, 8);
                    let sa_off = cur;
                    cur += 16; // sizeof(sockaddr_in)
                    unicast_entries.push((u_off, sa_off, false));
                }
            }
            if include_v6 {
                for _ in &iface.ipv6 {
                    cur = align_up(cur, 8);
                    let u_off = cur;
                    cur += IP_ADAPTER_UNICAST_ADDRESS_LH_SIZE;
                    cur = align_up(cur, 8);
                    let sa_off = cur;
                    cur += 28; // sizeof(sockaddr_in6)
                    unicast_entries.push((u_off, sa_off, true));
                }
            }
        }

        layouts.push(AdapterLayout {
            node_off,
            name_ascii_off,
            dns_suffix_off,
            friendly_off,
            unicast_entries,
        });
    }
    let needed = align_up(cur, 8);
    let avail = size.read_unaligned() as usize;
    if out.is_null() || avail < needed {
        size.write_unaligned(needed as u32);
        return ERROR_BUFFER_OVERFLOW;
    }

    // Pass 2: serialize into caller buffer.
    let base = out as *mut u8;
    std::ptr::write_bytes(base, 0, needed);

    for (idx, (iface, lay)) in adapters.iter().zip(layouts.iter()).enumerate() {
        let p = base.add(lay.node_off);
        let next_ptr = layouts
            .get(idx + 1)
            .map_or(0usize, |nl| base.add(nl.node_off) as usize);

        // AdapterName (ASCII)
        let name_ptr = base.add(lay.name_ascii_off);
        std::ptr::copy_nonoverlapping(iface.name.as_ptr(), name_ptr, iface.name.len());
        name_ptr.add(iface.name.len()).write_unaligned(0);

        // DnsSuffix (empty UTF-16)
        let dns_ptr = base.add(lay.dns_suffix_off);
        (dns_ptr as *mut u16).write_unaligned(0);

        // FriendlyName & Description (UTF-16 interface name)
        let friendly_ptr = base.add(lay.friendly_off);
        for (k, u) in iface.name.encode_utf16().chain(std::iter::once(0u16)).enumerate() {
            (friendly_ptr.add(k * 2) as *mut u16).write_unaligned(u);
        }

        let first_unicast = lay
            .unicast_entries
            .first()
            .map_or(0usize, |&(u_off, _, _)| base.add(u_off) as usize);

        // IP_ADAPTER_ADDRESSES_LH header fields
        (p.add(0x000) as *mut u32).write_unaligned(IP_ADAPTER_ADDRESSES_LH_SIZE as u32); // Length
        (p.add(0x004) as *mut u32).write_unaligned(iface.index); // IfIndex
        (p.add(0x008) as *mut usize).write_unaligned(next_ptr); // Next
        (p.add(0x010) as *mut usize).write_unaligned(name_ptr as usize); // AdapterName
        (p.add(0x018) as *mut usize).write_unaligned(first_unicast); // FirstUnicastAddress
        (p.add(0x038) as *mut usize).write_unaligned(dns_ptr as usize); // DnsSuffix
        (p.add(0x040) as *mut usize).write_unaligned(friendly_ptr as usize); // Description
        (p.add(0x048) as *mut usize).write_unaligned(friendly_ptr as usize); // FriendlyName

        let mac_len = iface.mac_len.min(8);
        if mac_len > 0 {
            std::ptr::copy_nonoverlapping(iface.mac.as_ptr(), p.add(0x050), mac_len);
        }
        (p.add(0x058) as *mut u32).write_unaligned(mac_len as u32); // PhysicalAddressLength

        let mut win_flags: u32 = 0;
        if !iface.ipv4.is_empty() {
            win_flags |= IP_ADAPTER_IPV4_ENABLED;
        }
        if !iface.ipv6.is_empty() {
            win_flags |= IP_ADAPTER_IPV6_ENABLED;
        }
        if (iface.flags & (libc::IFF_MULTICAST as u32)) == 0 {
            win_flags |= IP_ADAPTER_NO_MULTICAST;
        }
        (p.add(0x05C) as *mut u32).write_unaligned(win_flags); // Flags
        (p.add(0x060) as *mut u32).write_unaligned(iface.mtu); // Mtu
        (p.add(0x064) as *mut u32).write_unaligned(iface.if_type); // IfType
        (p.add(0x068) as *mut u32).write_unaligned(if iface.is_up() {
            IF_OPER_STATUS_UP
        } else {
            IF_OPER_STATUS_DOWN
        }); // OperStatus
        (p.add(0x06C) as *mut u32).write_unaligned(iface.index); // Ipv6IfIndex
        // ZoneIndices[0..16]
        for z in 0..16usize {
            (p.add(0x070 + z * 4) as *mut u32).write_unaligned(iface.index);
        }
        // Vista+ link speeds & LUID
        let speed: u64 = if iface.is_loopback() { 10_000_000_000 } else { 1_000_000_000 };
        (p.add(0x0B8) as *mut u64).write_unaligned(speed); // TransmitLinkSpeed
        (p.add(0x0C0) as *mut u64).write_unaligned(speed); // ReceiveLinkSpeed
        (p.add(0x0D8) as *mut u32).write_unaligned(if iface.is_loopback() { 75 } else { 25 }); // Ipv4Metric
        (p.add(0x0DC) as *mut u32).write_unaligned(if iface.is_loopback() { 75 } else { 25 }); // Ipv6Metric
        let luid: u64 = ((iface.if_type as u64) << 48) | ((iface.index as u64) << 24);
        (p.add(0x0E0) as *mut u64).write_unaligned(luid); // Luid
        (p.add(0x0F8) as *mut u32).write_unaligned(1); // CompartmentId
        (p.add(0x10C) as *mut u32).write_unaligned(1); // ConnectionType = NET_IF_CONNECTION_DEDICATED

        // Unicast entries
        let mut v4_i = 0usize;
        let mut v6_i = 0usize;
        for (ui, &(u_off, sa_off, is_v6)) in lay.unicast_entries.iter().enumerate() {
            let up = base.add(u_off);
            let sap = base.add(sa_off);
            let u_next = lay
                .unicast_entries
                .get(ui + 1)
                .map_or(0usize, |&(nu_off, _, _)| base.add(nu_off) as usize);

            let (sa_len, prefix_len) = if !is_v6 {
                let v4 = &iface.ipv4[v4_i];
                v4_i += 1;
                (sap as *mut u16).write_unaligned(WIN_AF_INET);
                std::ptr::copy_nonoverlapping(v4.addr.as_ptr(), sap.add(4), 4);
                (16i32, v4.prefix_len)
            } else {
                let v6 = &iface.ipv6[v6_i];
                v6_i += 1;
                (sap as *mut u16).write_unaligned(WIN_AF_INET6);
                std::ptr::copy_nonoverlapping(v6.addr.as_ptr(), sap.add(8), 16);
                (sap.add(24) as *mut u32).write_unaligned(v6.scope_id);
                (28i32, v6.prefix_len)
            };

            (up.add(0x00) as *mut u32).write_unaligned(IP_ADAPTER_UNICAST_ADDRESS_LH_SIZE as u32);
            (up.add(0x04) as *mut u32).write_unaligned(0x01); // IP_ADAPTER_ADDRESS_DNS_ELIGIBLE
            (up.add(0x08) as *mut usize).write_unaligned(u_next);
            (up.add(0x10) as *mut usize).write_unaligned(sap as usize); // SOCKET_ADDRESS.lpSockaddr
            (up.add(0x18) as *mut i32).write_unaligned(sa_len); // SOCKET_ADDRESS.iSockaddrLength
            (up.add(0x20) as *mut u32).write_unaligned(1); // PrefixOrigin = Manual
            (up.add(0x24) as *mut u32).write_unaligned(1); // SuffixOrigin = Manual
            (up.add(0x28) as *mut u32).write_unaligned(4); // DadState = IpDadStatePreferred
            (up.add(0x2C) as *mut u32).write_unaligned(u32::MAX); // ValidLifetime
            (up.add(0x30) as *mut u32).write_unaligned(u32::MAX); // PreferredLifetime
            (up.add(0x34) as *mut u32).write_unaligned(u32::MAX); // LeaseLifetime
            up.add(0x38).write_unaligned(prefix_len); // OnLinkPrefixLength
        }
    }

    size.write_unaligned(needed as u32);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn if_nametoindex(name: *const libc::c_char) -> u32 {
    if name.is_null() {
        return 0;
    }
    libc::if_nametoindex(name)
}

pub unsafe extern "win64" fn if_indextoname(idx: u32, name: *mut libc::c_char) -> *mut libc::c_char {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    libc::if_indextoname(idx, name)
}

pub unsafe extern "win64" fn GetAdaptersInfo(adapter_info: *mut c_void, size: *mut u32) -> u32 {
    if size.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let all = enumerate_host_interfaces();
    // On Windows GetAdaptersInfo returns IPv4-capable adapters (preferring non-loopback;
    // fall back to loopback if no non-loopback IPv4 adapter is present).
    let mut adapters: Vec<&HostInterface> = all
        .iter()
        .filter(|i| !i.ipv4.is_empty() && !i.is_loopback())
        .collect();
    if adapters.is_empty() {
        adapters = all.iter().filter(|i| !i.ipv4.is_empty()).collect();
    }
    if adapters.is_empty() {
        size.write_unaligned(0);
        return ERROR_NO_DATA;
    }

    let extra_addr_nodes: usize = adapters.iter().map(|a| a.ipv4.len().saturating_sub(1)).sum();
    let needed = adapters.len() * IP_ADAPTER_INFO_SIZE + extra_addr_nodes * IP_ADDR_STRING_SIZE;
    let avail = size.read_unaligned() as usize;
    if adapter_info.is_null() || avail < needed {
        size.write_unaligned(needed as u32);
        return ERROR_BUFFER_OVERFLOW;
    }

    let base = adapter_info as *mut u8;
    std::ptr::write_bytes(base, 0, needed);

    let mut extra_off = adapters.len() * IP_ADAPTER_INFO_SIZE;
    for (idx, iface) in adapters.iter().enumerate() {
        let p = base.add(idx * IP_ADAPTER_INFO_SIZE);
        let next_ptr = if idx + 1 < adapters.len() {
            base.add((idx + 1) * IP_ADAPTER_INFO_SIZE) as usize
        } else {
            0usize
        };
        (p.add(0x000) as *mut usize).write_unaligned(next_ptr); // Next
        (p.add(0x008) as *mut u32).write_unaligned(iface.index); // ComboIndex

        let name_bytes = iface.name.as_bytes();
        let copy_name = name_bytes.len().min(259);
        std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), p.add(0x00C), copy_name);
        let copy_desc = name_bytes.len().min(131);
        std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), p.add(0x110), copy_desc);

        let mac_len = iface.mac_len.min(8);
        (p.add(0x194) as *mut u32).write_unaligned(mac_len as u32); // AddressLength
        if mac_len > 0 {
            std::ptr::copy_nonoverlapping(iface.mac.as_ptr(), p.add(0x198), mac_len);
        }
        (p.add(0x1A0) as *mut u32).write_unaligned(iface.index); // Index
        (p.add(0x1A4) as *mut u32).write_unaligned(iface.if_type); // Type
        (p.add(0x1A8) as *mut u32).write_unaligned(0); // DhcpEnabled

        // Populate IpAddressList @0x1B8 (+ extra linked IP_ADDR_STRING nodes if >1 IPv4)
        let first_ip_node = p.add(0x1B8);
        (p.add(0x1B0) as *mut usize).write_unaligned(first_ip_node as usize); // CurrentIpAddress

        for (ai, v4) in iface.ipv4.iter().enumerate() {
            let node_p = if ai == 0 {
                first_ip_node
            } else {
                let np = base.add(extra_off);
                extra_off += IP_ADDR_STRING_SIZE;
                np
            };
            let next_ip_ptr = if ai + 1 < iface.ipv4.len() {
                base.add(extra_off) as usize
            } else {
                0usize
            };
            (node_p.add(0x00) as *mut usize).write_unaligned(next_ip_ptr);
            let ip_s = format_ipv4(v4.addr);
            let mask_s = format_ipv4(v4.mask);
            std::ptr::copy_nonoverlapping(ip_s.as_ptr(), node_p.add(0x08), ip_s.len().min(15));
            std::ptr::copy_nonoverlapping(mask_s.as_ptr(), node_p.add(0x18), mask_s.len().min(15));
            (node_p.add(0x28) as *mut u32).write_unaligned(iface.index);
        }

        // GatewayList @0x1E8 default "0.0.0.0"
        let zero_ip = b"0.0.0.0";
        std::ptr::copy_nonoverlapping(zero_ip.as_ptr(), p.add(0x1E8 + 0x08), zero_ip.len());
        std::ptr::copy_nonoverlapping(zero_ip.as_ptr(), p.add(0x1E8 + 0x18), zero_ip.len());
    }

    size.write_unaligned(needed as u32);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn GetBestInterface(dest_addr: u32, best_if_index: *mut u32) -> u32 {
    if best_if_index.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let ifaces = enumerate_host_interfaces();
    let dest = dest_addr.to_ne_bytes();
    match best_interface_v4(&ifaces, dest) {
        Some(idx) => {
            best_if_index.write_unaligned(idx);
            ERROR_SUCCESS
        }
        None => ERROR_NO_DATA,
    }
}

pub unsafe extern "win64" fn GetBestInterfaceEx(dest_addr: *const u8, best_if_index: *mut u32) -> u32 {
    if dest_addr.is_null() || best_if_index.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let family = (dest_addr as *const u16).read_unaligned();
    let ifaces = enumerate_host_interfaces();
    let chosen = match family {
        WIN_AF_INET => {
            let mut addr = [0u8; 4];
            std::ptr::copy_nonoverlapping(dest_addr.add(4), addr.as_mut_ptr(), 4);
            best_interface_v4(&ifaces, addr)
        }
        WIN_AF_INET6 => {
            let mut addr = [0u8; 16];
            std::ptr::copy_nonoverlapping(dest_addr.add(8), addr.as_mut_ptr(), 16);
            let scope_id = (dest_addr.add(24) as *const u32).read_unaligned();
            best_interface_v6(&ifaces, addr, scope_id)
        }
        _ => return ERROR_INVALID_PARAMETER,
    };
    match chosen {
        Some(idx) => {
            best_if_index.write_unaligned(idx);
            ERROR_SUCCESS
        }
        None => ERROR_NO_DATA,
    }
}

pub unsafe extern "win64" fn GetIfTable(if_table: *mut u8, size: *mut u32, order: BOOL) -> u32 {
    if size.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let mut ifaces = enumerate_host_interfaces();
    if order != FALSE {
        ifaces.sort_by_key(|i| i.index);
    }

    let needed = 4 + ifaces.len() * MIB_IFROW_SIZE;
    let avail = size.read_unaligned() as usize;
    if if_table.is_null() || avail < needed {
        size.write_unaligned(needed as u32);
        return ERROR_INSUFFICIENT_BUFFER;
    }

    std::ptr::write_bytes(if_table, 0, needed);
    (if_table as *mut u32).write_unaligned(ifaces.len() as u32);

    for (idx, iface) in ifaces.iter().enumerate() {
        let row = if_table.add(4 + idx * MIB_IFROW_SIZE);
        // wszName: [u16; 256] @0x000
        for (k, u) in iface.name.encode_utf16().take(255).enumerate() {
            (row.add(k * 2) as *mut u16).write_unaligned(u);
        }
        (row.add(0x200) as *mut u32).write_unaligned(iface.index); // dwIndex
        (row.add(0x204) as *mut u32).write_unaligned(iface.if_type); // dwType
        (row.add(0x208) as *mut u32).write_unaligned(iface.mtu); // dwMtu
        (row.add(0x20C) as *mut u32).write_unaligned(1_000_000_000u32); // dwSpeed (1 Gbps)
        let mac_len = iface.mac_len.min(8);
        (row.add(0x210) as *mut u32).write_unaligned(mac_len as u32); // dwPhysAddrLen
        if mac_len > 0 {
            std::ptr::copy_nonoverlapping(iface.mac.as_ptr(), row.add(0x214), mac_len);
        }
        let up = iface.is_up();
        (row.add(0x21C) as *mut u32).write_unaligned(if up { 1 } else { 2 }); // dwAdminStatus
        (row.add(0x220) as *mut u32).write_unaligned(if up {
            MIB_IF_OPER_STATUS_OPERATIONAL
        } else {
            MIB_IF_OPER_STATUS_NON_OPERATIONAL
        }); // dwOperStatus
        let descr_bytes = iface.name.as_bytes();
        let dlen = descr_bytes.len().min(255);
        (row.add(0x258) as *mut u32).write_unaligned(dlen as u32); // dwDescrLen
        std::ptr::copy_nonoverlapping(descr_bytes.as_ptr(), row.add(0x25C), dlen); // bDescr
    }

    size.write_unaligned(needed as u32);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn GetIpAddrTable(ip_addr_table: *mut u8, size: *mut u32, order: BOOL) -> u32 {
    if size.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let ifaces = enumerate_host_interfaces();
    let mut rows: Vec<(u32, [u8; 4], [u8; 4])> = Vec::new();
    for iface in &ifaces {
        for v4 in &iface.ipv4 {
            rows.push((iface.index, v4.addr, v4.mask));
        }
    }
    if order != FALSE {
        rows.sort_by_key(|&(_, addr, _)| u32::from_be_bytes(addr));
    }

    let needed = 4 + rows.len() * MIB_IPADDRROW_SIZE;
    let avail = size.read_unaligned() as usize;
    if ip_addr_table.is_null() || avail < needed {
        size.write_unaligned(needed as u32);
        return ERROR_INSUFFICIENT_BUFFER;
    }

    std::ptr::write_bytes(ip_addr_table, 0, needed);
    (ip_addr_table as *mut u32).write_unaligned(rows.len() as u32);

    for (idx, &(if_index, addr, mask)) in rows.iter().enumerate() {
        let row = ip_addr_table.add(4 + idx * MIB_IPADDRROW_SIZE);
        (row.add(0x00) as *mut u32).write_unaligned(u32::from_ne_bytes(addr)); // dwAddr (net order)
        (row.add(0x04) as *mut u32).write_unaligned(if_index); // dwIndex
        (row.add(0x08) as *mut u32).write_unaligned(u32::from_ne_bytes(mask)); // dwMask (net order)
        (row.add(0x0C) as *mut u32).write_unaligned(1); // dwBCastAddr
        (row.add(0x10) as *mut u32).write_unaligned(65535); // dwReasmSize
        (row.add(0x14) as *mut u16).write_unaligned(0); // unused1
        (row.add(0x16) as *mut u16).write_unaligned(0x0001); // wType = MIB_IPADDR_PRIMARY
    }

    size.write_unaligned(needed as u32);
    ERROR_SUCCESS
}

/// `GetIpForwardTable`: builds an IPv4 routing table (`MIB_IPFORWARDTABLE` + 56-byte
/// `MIB_IPFORWARDROW` entries) from live host interfaces (`getifaddrs()`).
pub unsafe extern "win64" fn GetIpForwardTable(
    ip_forward_table: *mut u8,
    size: *mut u32,
    order: BOOL,
) -> u32 {
    if size.is_null() {
        return ERROR_INVALID_PARAMETER;
    }

    #[derive(Clone, Copy)]
    struct ForwardRow {
        dest: [u8; 4],
        mask: [u8; 4],
        next_hop: [u8; 4],
        if_index: u32,
        route_type: u32,
        proto: u32,
        metric1: u32,
    }

    let ifaces = enumerate_host_interfaces();
    let mut rows: Vec<ForwardRow> = Vec::new();

    // Synthesize default route (0.0.0.0/0) on the primary up non-loopback IPv4 interface, if any.
    let default_iface = ifaces
        .iter()
        .find(|i| {
            i.is_up()
                && !i.is_loopback()
                && i.ipv4.iter().any(|v| !(v.addr[0] == 169 && v.addr[1] == 254))
        })
        .or_else(|| ifaces.iter().find(|i| i.is_up() && !i.is_loopback() && !i.ipv4.is_empty()));
    if let Some(iface) = default_iface {
        if let Some(v4) = iface.ipv4.first() {
            rows.push(ForwardRow {
                dest: [0, 0, 0, 0],
                mask: [0, 0, 0, 0],
                next_hop: v4.addr,
                if_index: iface.index,
                route_type: MIB_IPROUTE_TYPE_INDIRECT,
                proto: MIB_IPROUTE_PROTO_NETMGMT,
                metric1: 25,
            });
        }
    }

    // On-link subnet and host routes for each UP IPv4 interface.
    for iface in ifaces.iter().filter(|i| i.is_up()) {
        let metric = if iface.is_loopback() { 75 } else { 25 };
        for v4 in &iface.ipv4 {
            let net = [
                v4.addr[0] & v4.mask[0],
                v4.addr[1] & v4.mask[1],
                v4.addr[2] & v4.mask[2],
                v4.addr[3] & v4.mask[3],
            ];
            if !rows
                .iter()
                .any(|r| r.dest == net && r.mask == v4.mask && r.if_index == iface.index)
            {
                rows.push(ForwardRow {
                    dest: net,
                    mask: v4.mask,
                    next_hop: v4.addr,
                    if_index: iface.index,
                    route_type: MIB_IPROUTE_TYPE_DIRECT,
                    proto: MIB_IPROUTE_PROTO_LOCAL,
                    metric1: metric,
                });
            }
            let host_mask = [255, 255, 255, 255];
            if !rows
                .iter()
                .any(|r| r.dest == v4.addr && r.mask == host_mask && r.if_index == iface.index)
            {
                rows.push(ForwardRow {
                    dest: v4.addr,
                    mask: host_mask,
                    next_hop: v4.addr,
                    if_index: iface.index,
                    route_type: MIB_IPROUTE_TYPE_DIRECT,
                    proto: MIB_IPROUTE_PROTO_LOCAL,
                    metric1: metric,
                });
            }
        }
    }

    if order != FALSE {
        rows.sort_by_key(|r| {
            (
                u32::from_be_bytes(r.dest),
                r.proto,
                u32::from_be_bytes(r.next_hop),
            )
        });
    }

    let needed = 4 + rows.len() * MIB_IPFORWARDROW_SIZE;
    let avail = size.read_unaligned() as usize;
    if ip_forward_table.is_null() || avail < needed {
        size.write_unaligned(needed as u32);
        return ERROR_INSUFFICIENT_BUFFER;
    }

    std::ptr::write_bytes(ip_forward_table, 0, needed);
    (ip_forward_table as *mut u32).write_unaligned(rows.len() as u32);

    for (idx, r) in rows.iter().enumerate() {
        let row = ip_forward_table.add(4 + idx * MIB_IPFORWARDROW_SIZE);
        (row.add(0x00) as *mut u32).write_unaligned(u32::from_ne_bytes(r.dest));
        (row.add(0x04) as *mut u32).write_unaligned(u32::from_ne_bytes(r.mask));
        (row.add(0x08) as *mut u32).write_unaligned(0); // dwForwardPolicy
        (row.add(0x0C) as *mut u32).write_unaligned(u32::from_ne_bytes(r.next_hop));
        (row.add(0x10) as *mut u32).write_unaligned(r.if_index);
        (row.add(0x14) as *mut u32).write_unaligned(r.route_type);
        (row.add(0x18) as *mut u32).write_unaligned(r.proto);
        (row.add(0x1C) as *mut u32).write_unaligned(0); // dwForwardAge
        (row.add(0x20) as *mut u32).write_unaligned(0); // dwForwardNextHopAS
        (row.add(0x24) as *mut u32).write_unaligned(r.metric1);
        (row.add(0x28) as *mut u32).write_unaligned(u32::MAX); // dwForwardMetric2
        (row.add(0x2C) as *mut u32).write_unaligned(u32::MAX); // dwForwardMetric3
        (row.add(0x30) as *mut u32).write_unaligned(u32::MAX); // dwForwardMetric4
        (row.add(0x34) as *mut u32).write_unaligned(u32::MAX); // dwForwardMetric5
    }

    size.write_unaligned(needed as u32);
    ERROR_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_adapters_addresses_enumerates_live_interfaces_and_checks_overflow() {
        unsafe {
            let mut sz: u32 = 0;
            let rc = GetAdaptersAddresses(WIN_AF_UNSPEC as u32, 0, std::ptr::null_mut(), std::ptr::null_mut(), &mut sz);
            assert_eq!(rc, ERROR_BUFFER_OVERFLOW);
            assert!(sz >= IP_ADAPTER_ADDRESSES_LH_SIZE as u32);

            // Buffer smaller than required must also report ERROR_BUFFER_OVERFLOW.
            let mut tiny = vec![0u8; 32];
            let mut tiny_sz: u32 = 32;
            let rc2 = GetAdaptersAddresses(
                WIN_AF_UNSPEC as u32,
                0,
                std::ptr::null_mut(),
                tiny.as_mut_ptr() as *mut c_void,
                &mut tiny_sz,
            );
            assert_eq!(rc2, ERROR_BUFFER_OVERFLOW);
            assert!(tiny_sz >= IP_ADAPTER_ADDRESSES_LH_SIZE as u32);

            let mut buf = vec![0u8; tiny_sz as usize + 4096];
            let mut out_sz = buf.len() as u32;
            let rc3 = GetAdaptersAddresses(
                WIN_AF_UNSPEC as u32,
                0,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut c_void,
                &mut out_sz,
            );
            assert_eq!(rc3, ERROR_SUCCESS);

            // Walk linked list and verify at least one adapter (e.g. lo0) has valid index & IPv4 127.0.0.1.
            let mut cur = buf.as_ptr();
            let mut count = 0usize;
            let mut saw_loopback = false;
            while !cur.is_null() {
                count += 1;
                let len = (cur.add(0x000) as *const u32).read_unaligned();
                assert_eq!(len as usize, IP_ADAPTER_ADDRESSES_LH_SIZE);
                let if_index = (cur.add(0x004) as *const u32).read_unaligned();
                assert_ne!(if_index, 0);
                let name_ptr = (cur.add(0x010) as *const *const libc::c_char).read_unaligned();
                assert!(!name_ptr.is_null());
                let os_idx = if_nametoindex(name_ptr);
                assert_eq!(os_idx, if_index);

                let mut ua = (cur.add(0x018) as *const *const u8).read_unaligned();
                while !ua.is_null() {
                    let sa = (ua.add(0x10) as *const *const u8).read_unaligned();
                    let sa_len = (ua.add(0x18) as *const i32).read_unaligned();
                    assert!(!sa.is_null());
                    let fam = (sa as *const u16).read_unaligned();
                    if fam == WIN_AF_INET {
                        assert_eq!(sa_len, 16);
                        let mut ip = [0u8; 4];
                        std::ptr::copy_nonoverlapping(sa.add(4), ip.as_mut_ptr(), 4);
                        if ip == [127, 0, 0, 1] {
                            saw_loopback = true;
                        }
                    } else if fam == WIN_AF_INET6 {
                        assert_eq!(sa_len, 28);
                    }
                    ua = (ua.add(0x08) as *const *const u8).read_unaligned();
                }

                cur = (cur.add(0x008) as *const *const u8).read_unaligned();
            }
            assert!(count >= 1);
            assert!(saw_loopback, "expected 127.0.0.1 loopback in GetAdaptersAddresses");
        }
    }

    #[test]
    fn get_adapters_info_if_table_ip_addr_table_and_best_interface() {
        unsafe {
            // GetAdaptersInfo
            let mut info_sz = 0u32;
            let rc = GetAdaptersInfo(std::ptr::null_mut(), &mut info_sz);
            assert_eq!(rc, ERROR_BUFFER_OVERFLOW);
            assert!(info_sz >= IP_ADAPTER_INFO_SIZE as u32);
            let mut info_buf = vec![0u8; info_sz as usize];
            assert_eq!(GetAdaptersInfo(info_buf.as_mut_ptr() as *mut c_void, &mut info_sz), ERROR_SUCCESS);
            let idx0 = (info_buf.as_ptr().add(0x1A0) as *const u32).read_unaligned();
            assert_ne!(idx0, 0);

            // GetIfTable
            let mut if_sz = 0u32;
            assert_eq!(GetIfTable(std::ptr::null_mut(), &mut if_sz, TRUE), ERROR_INSUFFICIENT_BUFFER);
            let mut if_buf = vec![0u8; if_sz as usize];
            assert_eq!(GetIfTable(if_buf.as_mut_ptr(), &mut if_sz, TRUE), ERROR_SUCCESS);
            let num_if = (if_buf.as_ptr() as *const u32).read_unaligned();
            assert!(num_if >= 1);

            // GetIpAddrTable
            let mut ip_sz = 0u32;
            assert_eq!(GetIpAddrTable(std::ptr::null_mut(), &mut ip_sz, TRUE), ERROR_INSUFFICIENT_BUFFER);
            let mut ip_buf = vec![0u8; ip_sz as usize];
            assert_eq!(GetIpAddrTable(ip_buf.as_mut_ptr(), &mut ip_sz, TRUE), ERROR_SUCCESS);
            let num_ip = (ip_buf.as_ptr() as *const u32).read_unaligned();
            assert!(num_ip >= 1);

            // GetBestInterface & GetBestInterfaceEx for 127.0.0.1 and ::1
            let lo_v4 = u32::from_ne_bytes([127, 0, 0, 1]);
            let mut best4 = 0u32;
            assert_eq!(GetBestInterface(lo_v4, &mut best4), ERROR_SUCCESS);
            assert_ne!(best4, 0);

            let mut sa4 = [0u8; 16];
            (sa4.as_mut_ptr() as *mut u16).write_unaligned(WIN_AF_INET);
            sa4[4..8].copy_from_slice(&[127, 0, 0, 1]);
            let mut best4_ex = 0u32;
            assert_eq!(GetBestInterfaceEx(sa4.as_ptr(), &mut best4_ex), ERROR_SUCCESS);
            assert_eq!(best4_ex, best4);

            let mut sa6 = [0u8; 28];
            (sa6.as_mut_ptr() as *mut u16).write_unaligned(WIN_AF_INET6);
            sa6[23] = 1; // ::1
            let mut best6_ex = 0u32;
            assert_eq!(GetBestInterfaceEx(sa6.as_ptr(), &mut best6_ex), ERROR_SUCCESS);
            assert_ne!(best6_ex, 0);

            // GetIpForwardTable
            assert_eq!(GetIpForwardTable(std::ptr::null_mut(), std::ptr::null_mut(), TRUE), ERROR_INVALID_PARAMETER);
            let mut fwd_sz = 0u32;
            assert_eq!(GetIpForwardTable(std::ptr::null_mut(), &mut fwd_sz, TRUE), ERROR_INSUFFICIENT_BUFFER);
            assert!(fwd_sz >= (4 + MIB_IPFORWARDROW_SIZE) as u32);
            let mut fwd_buf = vec![0u8; fwd_sz as usize];
            assert_eq!(GetIpForwardTable(fwd_buf.as_mut_ptr(), &mut fwd_sz, TRUE), ERROR_SUCCESS);
            let num_fwd = (fwd_buf.as_ptr() as *const u32).read_unaligned() as usize;
            assert!(num_fwd >= 1);
            let mut saw_lo_route = false;
            for i in 0..num_fwd {
                let row = fwd_buf.as_ptr().add(4 + i * MIB_IPFORWARDROW_SIZE);
                let dest = (row.add(0x00) as *const u32).read_unaligned().to_ne_bytes();
                let if_idx = (row.add(0x10) as *const u32).read_unaligned();
                assert_ne!(if_idx, 0);
                if dest == [127, 0, 0, 1] {
                    saw_lo_route = true;
                }
            }
            assert!(saw_lo_route, "expected 127.0.0.1 host route in GetIpForwardTable");
        }
    }
}
