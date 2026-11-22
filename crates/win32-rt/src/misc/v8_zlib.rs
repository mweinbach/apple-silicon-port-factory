//! Builtin `v8_zlib.dll` (Chromium `Cr_z_*` zlib exports used by `v8.dll`).
//!
//! Shares the Win64 LLP64 `z_stream` <-> macOS `libz.dylib` bridge in
//! [`super::zlib1`].

use super::zlib1::{
    host_adler32, host_compressBound, host_deflate, host_deflateEnd, host_deflateInit2_,
    host_deflateSetHeader, host_zlibVersion, inflate, inflateEnd, inflateInit2_, sync_from_mac,
    sync_to_mac, HostZState, MacZStream, WinGzHeader, WinZStream, WIN_ZSTREAM_SIZE, Z_STREAM_ERROR,
    Z_VERSION_ERROR,
};

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

pub unsafe extern "win64" fn Cr_z_adler32(adler: u32, buf: *const u8, len: u32) -> u32 {
    host_adler32(adler as usize, buf, len) as u32
}

pub unsafe extern "win64" fn Cr_z_compressBound(source_len: u32) -> u32 {
    host_compressBound(source_len as usize) as u32
}

pub unsafe extern "win64" fn Cr_z_inflateInit2_(
    strm: *mut WinZStream,
    window_bits: i32,
    version: *const u8,
    stream_size: i32,
) -> i32 {
    inflateInit2_(strm, window_bits, version, stream_size)
}

pub unsafe extern "win64" fn Cr_z_inflate(strm: *mut WinZStream, flush: i32) -> i32 {
    inflate(strm, flush)
}

pub unsafe extern "win64" fn Cr_z_inflateEnd(strm: *mut WinZStream) -> i32 {
    inflateEnd(strm)
}

pub unsafe extern "win64" fn Cr_z_deflateInit2_(
    strm: *mut WinZStream,
    level: i32,
    method: i32,
    window_bits: i32,
    mem_level: i32,
    strategy: i32,
    version: *const u8,
    stream_size: i32,
) -> i32 {
    if version.is_null() || *version != b'1' || stream_size != WIN_ZSTREAM_SIZE {
        return Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    let mut host = Box::new(HostZState {
        mac: std::mem::zeroed(),
        gz: std::mem::zeroed(),
    });
    sync_to_mac(strm, &mut host.mac);
    let rc = host_deflateInit2_(
        &mut host.mac,
        level,
        method,
        window_bits,
        mem_level,
        strategy,
        host_zlibVersion(),
        std::mem::size_of::<MacZStream>() as i32,
    );
    if rc == 0 {
        sync_from_mac(&host.mac, strm);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*strm).state), Box::into_raw(host));
    } else {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*strm).state), std::ptr::null_mut());
    }
    rc
}

pub unsafe extern "win64" fn Cr_z_deflate(strm: *mut WinZStream, flush: i32) -> i32 {
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    let state = std::ptr::read_unaligned(std::ptr::addr_of!((*strm).state));
    if state.is_null() {
        return Z_STREAM_ERROR;
    }
    sync_to_mac(strm, &mut (*state).mac);
    let rc = host_deflate(&mut (*state).mac, flush);
    sync_from_mac(&(*state).mac, strm);
    rc
}

pub unsafe extern "win64" fn Cr_z_deflateEnd(strm: *mut WinZStream) -> i32 {
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    let state = std::ptr::read_unaligned(std::ptr::addr_of!((*strm).state));
    if state.is_null() {
        return Z_STREAM_ERROR;
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*strm).state), std::ptr::null_mut());
    let mut host = Box::from_raw(state);
    host_deflateEnd(&mut host.mac)
}

pub unsafe extern "win64" fn Cr_z_deflateSetHeader(
    strm: *mut WinZStream,
    head: *mut WinGzHeader,
) -> i32 {
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    let state = std::ptr::read_unaligned(std::ptr::addr_of!((*strm).state));
    if state.is_null() {
        return Z_STREAM_ERROR;
    }
    if head.is_null() {
        return host_deflateSetHeader(&mut (*state).mac, std::ptr::null_mut());
    }
    let h = std::ptr::read_unaligned(head);
    (*state).gz.text = h.text;
    (*state).gz.time = h.time as usize;
    (*state).gz.xflags = h.xflags;
    (*state).gz.os = h.os;
    (*state).gz.extra = h.extra;
    (*state).gz.extra_len = h.extra_len;
    (*state).gz.extra_max = h.extra_max;
    (*state).gz.name = h.name;
    (*state).gz.name_max = h.name_max;
    (*state).gz.comment = h.comment;
    (*state).gz.comm_max = h.comm_max;
    (*state).gz.hcrc = h.hcrc;
    (*state).gz.done = h.done;
    host_deflateSetHeader(&mut (*state).mac, &mut (*state).gz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deflate_inflate_roundtrip() {
        unsafe {
            let input = b"Alan Wake 2 native Apple Silicon zlib bridge test payload";
            let bound = Cr_z_compressBound(input.len() as u32) as usize;
            let mut compressed = vec![0u8; bound];

            let mut dstrm: WinZStream = std::mem::zeroed();
            dstrm.next_in = input.as_ptr();
            dstrm.avail_in = input.len() as u32;
            dstrm.next_out = compressed.as_mut_ptr();
            dstrm.avail_out = compressed.len() as u32;

            assert_eq!(
                Cr_z_deflateInit2_(
                    &mut dstrm,
                    6,
                    8, // Z_DEFLATED
                    15,
                    8,
                    0,
                    b"1.2.8\0".as_ptr(),
                    WIN_ZSTREAM_SIZE,
                ),
                0
            );
            assert_eq!(Cr_z_deflate(&mut dstrm, 4), 1); // Z_FINISH -> Z_STREAM_END
            let clen = dstrm.total_out as usize;
            assert_eq!(Cr_z_deflateEnd(&mut dstrm), 0);

            let mut decompressed = vec![0u8; input.len()];
            let mut istrm: WinZStream = std::mem::zeroed();
            istrm.next_in = compressed.as_ptr();
            istrm.avail_in = clen as u32;
            istrm.next_out = decompressed.as_mut_ptr();
            istrm.avail_out = decompressed.len() as u32;

            assert_eq!(
                Cr_z_inflateInit2_(&mut istrm, 15, b"1.2.8\0".as_ptr(), WIN_ZSTREAM_SIZE),
                0
            );
            assert_eq!(Cr_z_inflate(&mut istrm, 4), 1); // Z_FINISH -> Z_STREAM_END
            assert_eq!(istrm.total_out as usize, input.len());
            assert_eq!(Cr_z_inflateEnd(&mut istrm), 0);
            assert_eq!(&decompressed[..], &input[..]);
        }
    }
}
