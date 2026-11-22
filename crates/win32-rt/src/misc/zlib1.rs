//! Builtin `zlib1.dll` backed by macOS system `libz.dylib`.
//!
//! Translates between the 88-byte (`0x58`) Windows x64 LLP64 `z_stream` (where
//! `uLong` fields `total_in`, `total_out`, `adler`, `reserved` are 32-bit) and
//! the 112-byte (`0x70`) macOS LP64 `z_stream`.

use std::ffi::c_void;

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

pub const Z_STREAM_ERROR: i32 = -2;
pub const Z_VERSION_ERROR: i32 = -6;
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
