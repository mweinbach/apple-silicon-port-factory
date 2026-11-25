//! Builtin `zlib1.dll` backed by macOS system `libz.dylib`.
//!
//! Translates between the 88-byte (`0x58`) Windows x64 LLP64 `z_stream` (where
//! `uLong` fields `total_in`, `total_out`, `adler`, `reserved` are 32-bit) and
//! the 112-byte (`0x70`) macOS LP64 `z_stream`.

use std::ffi::c_void;

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

pub const Z_OK: i32 = 0;
pub const Z_STREAM_END: i32 = 1;
pub const Z_NO_FLUSH: i32 = 0;
pub const Z_FINISH: i32 = 4;
pub const Z_STREAM_ERROR: i32 = -2;
pub const Z_DATA_ERROR: i32 = -3;
pub const Z_BUF_ERROR: i32 = -5;
pub const Z_VERSION_ERROR: i32 = -6;
pub const Z_DEFLATED: i32 = 8;
pub const Z_DEFAULT_STRATEGY: i32 = 0;
pub const WIN_ZSTREAM_SIZE: i32 = 0x58;

#[repr(C)]
pub struct WinZStream {
    pub next_in: *const u8,
    pub avail_in: u32,
    pub total_in: u32,
    pub next_out: *mut u8,
    pub avail_out: u32,
    pub total_out: u32,
    pub msg: *const u8,
    pub state: *mut HostZState,
    pub zalloc: usize,
    pub zfree: usize,
    pub opaque: usize,
    pub data_type: i32,
    pub adler: u32,
    pub reserved: u32,
    pub _pad: u32,
}
const _: () = assert!(std::mem::size_of::<WinZStream>() == 0x58);

#[repr(C)]
pub struct MacZStream {
    pub next_in: *const u8,
    pub avail_in: u32,
    pub _pad0: u32,
    pub total_in: usize,
    pub next_out: *mut u8,
    pub avail_out: u32,
    pub _pad1: u32,
    pub total_out: usize,
    pub msg: *const u8,
    pub state: *mut c_void,
    pub zalloc: usize,
    pub zfree: usize,
    pub opaque: usize,
    pub data_type: i32,
    pub _pad2: u32,
    pub adler: usize,
    pub reserved: usize,
}
const _: () = assert!(std::mem::size_of::<MacZStream>() == 0x70);

#[repr(C)]
pub struct WinGzHeader {
    pub text: i32,
    pub time: u32,
    pub xflags: i32,
    pub os: i32,
    pub extra: *mut u8,
    pub extra_len: u32,
    pub extra_max: u32,
    pub name: *mut u8,
    pub name_max: u32,
    pub _pad0: u32,
    pub comment: *mut u8,
    pub comm_max: u32,
    pub hcrc: i32,
    pub done: i32,
    pub _pad1: u32,
}

#[repr(C)]
pub struct MacGzHeader {
    pub text: i32,
    pub _pad0: u32,
    pub time: usize,
    pub xflags: i32,
    pub os: i32,
    pub extra: *mut u8,
    pub extra_len: u32,
    pub extra_max: u32,
    pub name: *mut u8,
    pub name_max: u32,
    pub _pad1: u32,
    pub comment: *mut u8,
    pub comm_max: u32,
    pub hcrc: i32,
    pub done: i32,
    pub _pad2: u32,
}

pub struct HostZState {
    pub mac: MacZStream,
    pub gz: MacGzHeader,
}

#[link(name = "z")]
extern "C" {
    #[link_name = "zlibVersion"]
    pub fn host_zlibVersion() -> *const u8;
    #[link_name = "inflateInit2_"]
    pub fn host_inflateInit2_(strm: *mut MacZStream, window_bits: i32, version: *const u8, stream_size: i32) -> i32;
    #[link_name = "inflate"]
    pub fn host_inflate(strm: *mut MacZStream, flush: i32) -> i32;
    #[link_name = "inflateEnd"]
    pub fn host_inflateEnd(strm: *mut MacZStream) -> i32;
    #[link_name = "deflateInit2_"]
    pub fn host_deflateInit2_(
        strm: *mut MacZStream,
        level: i32,
        method: i32,
        window_bits: i32,
        mem_level: i32,
        strategy: i32,
        version: *const u8,
        stream_size: i32,
    ) -> i32;
    #[link_name = "deflate"]
    pub fn host_deflate(strm: *mut MacZStream, flush: i32) -> i32;
    #[link_name = "deflateEnd"]
    pub fn host_deflateEnd(strm: *mut MacZStream) -> i32;
    #[link_name = "deflateSetHeader"]
    pub fn host_deflateSetHeader(strm: *mut MacZStream, head: *mut MacGzHeader) -> i32;
    #[link_name = "adler32"]
    pub fn host_adler32(adler: usize, buf: *const u8, len: u32) -> usize;
    #[link_name = "compressBound"]
    pub fn host_compressBound(source_len: usize) -> usize;
}

pub unsafe fn sync_to_mac(win: *const WinZStream, mac: *mut MacZStream) {
    (*mac).next_in = std::ptr::read_unaligned(std::ptr::addr_of!((*win).next_in));
    (*mac).avail_in = std::ptr::read_unaligned(std::ptr::addr_of!((*win).avail_in));
    (*mac).next_out = std::ptr::read_unaligned(std::ptr::addr_of!((*win).next_out));
    (*mac).avail_out = std::ptr::read_unaligned(std::ptr::addr_of!((*win).avail_out));
}

pub unsafe fn sync_from_mac(mac: *const MacZStream, win: *mut WinZStream) {
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).next_in), (*mac).next_in);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).avail_in), (*mac).avail_in);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).total_in), (*mac).total_in as u32);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).next_out), (*mac).next_out);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).avail_out), (*mac).avail_out);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).total_out), (*mac).total_out as u32);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).msg), (*mac).msg);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).data_type), (*mac).data_type);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*win).adler), (*mac).adler as u32);
}

pub unsafe extern "win64" fn zlibVersion() -> *const u8 {
    host_zlibVersion()
}

pub unsafe extern "win64" fn inflateInit2_(
    strm: *mut WinZStream,
    window_bits: i32,
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
    let rc = host_inflateInit2_(
        &mut host.mac,
        window_bits,
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

pub unsafe extern "win64" fn inflateInit_(
    strm: *mut WinZStream,
    version: *const u8,
    stream_size: i32,
) -> i32 {
    inflateInit2_(strm, 15, version, stream_size)
}

pub unsafe extern "win64" fn inflate(strm: *mut WinZStream, flush: i32) -> i32 {
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    let state = std::ptr::read_unaligned(std::ptr::addr_of!((*strm).state));
    if state.is_null() {
        return Z_STREAM_ERROR;
    }
    sync_to_mac(strm, &mut (*state).mac);
    let rc = host_inflate(&mut (*state).mac, flush);
    sync_from_mac(&(*state).mac, strm);
    rc
}

pub unsafe extern "win64" fn inflateEnd(strm: *mut WinZStream) -> i32 {
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    let state = std::ptr::read_unaligned(std::ptr::addr_of!((*strm).state));
    if state.is_null() {
        return Z_STREAM_ERROR;
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*strm).state), std::ptr::null_mut());
    let mut host = Box::from_raw(state);
    host_inflateEnd(&mut host.mac)
}

// ---------------------------------------------------------------------------
// Safe RAII `libz.dylib` stream wrapper for crate-internal callers
// (e.g. `libcurl.rs`, `dstorage.rs`).
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ZlibMode {
    Inflate,
    Deflate,
}

/// Safe RAII wrapper around macOS system `libz.dylib`'s [`MacZStream`].
///
/// Heap-boxes the underlying `MacZStream` so its address stays stable across
/// moves, clears temporary slice pointers after every step, and invokes
/// `inflateEnd` / `deflateEnd` in [`Drop`] so streams never leak.
#[allow(dead_code)]
pub(crate) struct ZlibStream {
    strm: Box<MacZStream>,
    mode: ZlibMode,
}

/// Error returned by [`ZlibStream::inflate_chunk_streaming`].
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum InflateStreamError<E> {
    Zlib(i32),
    Callback(E),
}

#[allow(dead_code)]
impl ZlibStream {
    /// Initialize a streaming inflater (`window_bits`: `15` for zlib, `15 + 16`
    /// for gzip, `-15` for raw RFC 1951 deflate, `15 + 32` for auto zlib/gzip).
    pub(crate) fn new_inflate(window_bits: i32) -> Result<Self, i32> {
        let mut strm: Box<MacZStream> = Box::new(unsafe { std::mem::zeroed() });
        let rc = unsafe {
            host_inflateInit2_(
                &mut *strm,
                window_bits,
                host_zlibVersion(),
                std::mem::size_of::<MacZStream>() as i32,
            )
        };
        if rc != Z_OK {
            return Err(rc);
        }
        Ok(Self {
            strm,
            mode: ZlibMode::Inflate,
        })
    }

    /// Initialize a streaming deflater with standard deflate parameters
    /// (`method = Z_DEFLATED`, `mem_level = 8`, `strategy = Z_DEFAULT_STRATEGY`).
    pub(crate) fn new_deflate(level: i32, window_bits: i32) -> Result<Self, i32> {
        Self::new_deflate_with(level, Z_DEFLATED, window_bits, 8, Z_DEFAULT_STRATEGY)
    }

    /// Initialize a streaming deflater with explicit `deflateInit2_` parameters.
    pub(crate) fn new_deflate_with(
        level: i32,
        method: i32,
        window_bits: i32,
        mem_level: i32,
        strategy: i32,
    ) -> Result<Self, i32> {
        let mut strm: Box<MacZStream> = Box::new(unsafe { std::mem::zeroed() });
        let rc = unsafe {
            host_deflateInit2_(
                &mut *strm,
                level,
                method,
                window_bits,
                mem_level,
                strategy,
                host_zlibVersion(),
                std::mem::size_of::<MacZStream>() as i32,
            )
        };
        if rc != Z_OK {
            return Err(rc);
        }
        Ok(Self {
            strm,
            mode: ZlibMode::Deflate,
        })
    }

    /// Run a single `inflate` or `deflate` step and return `(rc, consumed, produced)`.
    ///
    /// Clears `next_in` / `next_out` before returning so temporary slice pointers
    /// never outlive this call.
    pub(crate) fn step(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        flush: i32,
    ) -> (i32, usize, usize) {
        let in_len = input.len().min(u32::MAX as usize);
        let out_len = output.len().min(u32::MAX as usize);
        // libz returns Z_STREAM_ERROR if next_in/next_out is NULL even when avail_* == 0.
        let dummy_in: u8 = 0;
        let mut dummy_out: u8 = 0;
        self.strm.next_in = if in_len == 0 {
            &dummy_in as *const u8
        } else {
            input.as_ptr()
        };
        self.strm.avail_in = in_len as u32;
        self.strm.next_out = if out_len == 0 {
            &mut dummy_out as *mut u8
        } else {
            output.as_mut_ptr()
        };
        self.strm.avail_out = out_len as u32;

        let rc = unsafe {
            match self.mode {
                ZlibMode::Inflate => host_inflate(&mut *self.strm, flush),
                ZlibMode::Deflate => host_deflate(&mut *self.strm, flush),
            }
        };

        let consumed = in_len - self.strm.avail_in as usize;
        let produced = out_len - self.strm.avail_out as usize;
        self.strm.next_in = std::ptr::null();
        self.strm.avail_in = 0;
        self.strm.next_out = std::ptr::null_mut();
        self.strm.avail_out = 0;
        (rc, consumed, produced)
    }

    /// Total input bytes consumed by this stream so far.
    pub(crate) fn total_in(&self) -> usize {
        self.strm.total_in
    }

    /// Total output bytes produced by this stream so far.
    pub(crate) fn total_out(&self) -> usize {
        self.strm.total_out
    }

    /// Feed a compressed chunk into an inflate stream, delivering decoded slices
    /// through the caller-supplied fixed-size `out_buf` without allocating an
    /// unbounded intermediate buffer. Stops immediately if `on_output` returns
    /// `Err(e)`. Returns `Ok(true)` when `Z_STREAM_END` is reached, `Ok(false)`
    /// when more input is required.
    pub(crate) fn inflate_chunk_streaming<E>(
        &mut self,
        input: &[u8],
        out_buf: &mut [u8],
        mut on_output: impl FnMut(&[u8]) -> Result<(), E>,
    ) -> Result<bool, InflateStreamError<E>> {
        if self.mode != ZlibMode::Inflate || out_buf.is_empty() {
            return Err(InflateStreamError::Zlib(Z_STREAM_ERROR));
        }
        if input.is_empty() {
            return Ok(false);
        }
        let mut offset = 0usize;
        while offset < input.len() {
            let mut consumed_pass = 0usize;
            let mut stream_end = false;
            loop {
                let (rc, consumed, produced) =
                    self.step(&input[offset + consumed_pass..], out_buf, Z_NO_FLUSH);
                consumed_pass += consumed;
                if produced > 0 {
                    on_output(&out_buf[..produced]).map_err(InflateStreamError::Callback)?;
                }
                if rc == Z_STREAM_END {
                    stream_end = true;
                    break;
                }
                if rc == Z_BUF_ERROR && offset + consumed_pass == input.len() {
                    break;
                }
                if rc != Z_OK && rc != Z_BUF_ERROR {
                    return Err(InflateStreamError::Zlib(rc));
                }
                if consumed == 0 && produced == 0 {
                    break;
                }
            }
            offset += consumed_pass;
            if stream_end {
                return Ok(true);
            }
            if consumed_pass == 0 {
                break;
            }
        }
        Ok(false)
    }

    /// Feed a compressed chunk into an inflate stream, appending decoded bytes
    /// to `out`. Returns `Ok(true)` when `Z_STREAM_END` is reached, `Ok(false)`
    /// when more input is required, or `Err(rc)` on zlib errors (e.g. `Z_DATA_ERROR`).
    pub(crate) fn inflate_chunk(&mut self, input: &[u8], out: &mut Vec<u8>) -> Result<bool, i32> {
        let mut scratch = [0u8; 16384];
        match self.inflate_chunk_streaming::<std::convert::Infallible>(input, &mut scratch, |chunk| {
            out.extend_from_slice(chunk);
            Ok(())
        }) {
            Ok(done) => Ok(done),
            Err(InflateStreamError::Zlib(rc)) => Err(rc),
            Err(InflateStreamError::Callback(never)) => match never {},
        }
    }

    /// One-shot deflate of `src` into a newly allocated `Vec<u8>`.
    pub(crate) fn deflate_all(src: &[u8], level: i32, window_bits: i32) -> Result<Vec<u8>, i32> {
        let mut strm = Self::new_deflate(level, window_bits)?;
        let cap = src.len().saturating_add(src.len() >> 3).saturating_add(64);
        let mut out = vec![0u8; cap];
        let mut in_off = 0usize;
        let mut out_off = 0usize;
        loop {
            if out_off == out.len() {
                let new_len = out.len().saturating_mul(2).max(64);
                out.resize(new_len, 0);
            }
            let rem_in = &src[in_off..];
            let flush = if rem_in.len() <= u32::MAX as usize {
                Z_FINISH
            } else {
                Z_NO_FLUSH
            };
            let (rc, consumed, produced) = strm.step(rem_in, &mut out[out_off..], flush);
            in_off += consumed;
            out_off += produced;
            match rc {
                Z_STREAM_END => {
                    out.truncate(out_off);
                    return Ok(out);
                }
                Z_OK => {
                    if consumed == 0 && produced == 0 {
                        return Err(Z_BUF_ERROR);
                    }
                }
                err => return Err(err),
            }
        }
    }

    /// One-shot inflate of `src` into a fixed-size destination slice `dst`.
    /// Returns the number of bytes written when `Z_STREAM_END` is reached.
    pub(crate) fn inflate_into(src: &[u8], dst: &mut [u8], window_bits: i32) -> Result<usize, i32> {
        let mut strm = Self::new_inflate(window_bits)?;
        let mut in_off = 0usize;
        let mut out_off = 0usize;
        loop {
            let rem_in = &src[in_off..];
            let rem_out = &mut dst[out_off..];
            let flush = if rem_in.len() <= u32::MAX as usize && rem_out.len() <= u32::MAX as usize {
                Z_FINISH
            } else {
                Z_NO_FLUSH
            };
            let (rc, consumed, produced) = strm.step(rem_in, rem_out, flush);
            in_off += consumed;
            out_off += produced;
            match rc {
                Z_STREAM_END => return Ok(out_off),
                Z_OK => {
                    if consumed == 0 && produced == 0 {
                        return Err(Z_BUF_ERROR);
                    }
                }
                err => return Err(err),
            }
        }
    }

    /// One-shot inflate of `src` into a newly allocated `Vec<u8>`.
    pub(crate) fn inflate_all(src: &[u8], window_bits: i32) -> Result<Vec<u8>, i32> {
        let mut strm = Self::new_inflate(window_bits)?;
        let mut out = Vec::with_capacity(src.len().saturating_mul(2));
        if !strm.inflate_chunk(src, &mut out)? {
            return Err(Z_BUF_ERROR);
        }
        Ok(out)
    }
}

impl Drop for ZlibStream {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                ZlibMode::Inflate => {
                    let _ = host_inflateEnd(&mut *self.strm);
                }
                ZlibMode::Deflate => {
                    let _ = host_deflateEnd(&mut *self.strm);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zlib_stream_raw_deflate_and_inflate_into_roundtrip() {
        let payload = b"DirectStorage GDeflate raw RFC 1951 payload 0123456789abcdef".repeat(64);
        let compressed = ZlibStream::deflate_all(&payload, 6, -15).expect("raw deflate failed");
        assert!(compressed.len() < payload.len());

        let mut dst = vec![0u8; payload.len()];
        let n = ZlibStream::inflate_into(&compressed, &mut dst, -15).expect("raw inflate failed");
        assert_eq!(n, payload.len());
        assert_eq!(dst, payload);

        // Destination too small must fail honestly rather than silently truncating.
        let mut short = vec![0u8; 16];
        assert!(ZlibStream::inflate_into(&compressed, &mut short, -15).is_err());

        // Empty payload roundtrips cleanly.
        let empty_comp = ZlibStream::deflate_all(b"", 6, -15).expect("empty deflate failed");
        let mut empty_dst = [];
        assert_eq!(
            ZlibStream::inflate_into(&empty_comp, &mut empty_dst, -15),
            Ok(0)
        );
    }

    #[test]
    fn zlib_stream_gzip_and_zlib_streaming_chunks() {
        // Larger than the internal 16 KiB scratch buffer to test multi-iteration drainage.
        let payload: Vec<u8> = (0..40_000usize)
            .map(|i| ((i.wrapping_mul(31) ^ (i >> 4)) & 0xFF) as u8)
            .collect();

        for window_bits in [15, 15 + 16, -15] {
            let compressed =
                ZlibStream::deflate_all(&payload, 6, window_bits).expect("deflate_all failed");

            // One-shot inflate_all
            let one_shot =
                ZlibStream::inflate_all(&compressed, window_bits).expect("inflate_all failed");
            assert_eq!(one_shot, payload);

            // Chunked streaming inflate (tiny 7-byte chunks)
            let mut inf = ZlibStream::new_inflate(window_bits).expect("new_inflate failed");
            let mut streamed = Vec::new();
            let mut done = false;
            for chunk in compressed.chunks(7) {
                if inf.inflate_chunk(chunk, &mut streamed).expect("chunk failed") {
                    done = true;
                }
            }
            assert!(done, "stream did not reach Z_STREAM_END for window_bits={window_bits}");
            assert_eq!(inf.total_out(), payload.len());
            assert_eq!(inf.total_in(), compressed.len());
            assert_eq!(streamed, payload);
        }
    }

    #[test]
    fn zlib_stream_error_handling_and_mode_checks() {
        assert_eq!(ZlibStream::new_inflate(99).err(), Some(Z_STREAM_ERROR));
        assert_eq!(ZlibStream::new_deflate(6, 99).err(), Some(Z_STREAM_ERROR));

        // Corrupt zlib stream reports Z_DATA_ERROR (enabling libcurl raw-deflate fallback).
        let mut inf = ZlibStream::new_inflate(15).unwrap();
        let mut out = Vec::new();
        assert_eq!(
            inf.inflate_chunk(b"\xff\xff\xff\xff\xff", &mut out),
            Err(Z_DATA_ERROR)
        );

        // Calling inflate_chunk on a deflate stream fails cleanly with Z_STREAM_ERROR.
        let mut def = ZlibStream::new_deflate(6, 15).unwrap();
        assert_eq!(def.inflate_chunk(b"abc", &mut out), Err(Z_STREAM_ERROR));
    }

    #[test]
    fn win64_inflate_exports_roundtrip() {
        let payload = b"Win64 LLP64 z_stream inflate export smoke test";
        let compressed = ZlibStream::deflate_all(payload, 6, 15).unwrap();
        let mut out = vec![0u8; payload.len()];
        unsafe {
            let mut strm: WinZStream = std::mem::zeroed();
            strm.next_in = compressed.as_ptr();
            strm.avail_in = compressed.len() as u32;
            strm.next_out = out.as_mut_ptr();
            strm.avail_out = out.len() as u32;

            assert_eq!(
                inflateInit_(&mut strm, b"1.2.12\0".as_ptr(), WIN_ZSTREAM_SIZE),
                Z_OK
            );
            assert_eq!(inflate(&mut strm, Z_FINISH), Z_STREAM_END);
            assert_eq!(strm.total_out as usize, payload.len());
            assert_eq!(inflateEnd(&mut strm), Z_OK);
            assert_eq!(out, payload);
        }
    }
}
