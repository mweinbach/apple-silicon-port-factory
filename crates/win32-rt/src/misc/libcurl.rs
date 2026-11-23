//! Builtin `libcurl.dll`: HTTP/1.1 + HTTPS (TLS 1.2/1.3 via `crate::net_tls`)
//! easy and multi transfer client.
//!
//! * Supports the full `curl_easy_*`, `curl_multi_*`, `curl_slist_*`, `curl_share_*`,
//!   `curl_global_*`, `curl_version*`, and escaping/freeing API surface with Win64 LLP64 ABI.
//! * Invokes guest `CURLOPT_WRITEFUNCTION`, `CURLOPT_HEADERFUNCTION`, and
//!   `CURLOPT_READFUNCTION` callbacks via [`crate::call_guest8`] strictly on the caller's
//!   thread (both in `curl_easy_perform` and in `curl_multi_perform`).
//! * `curl_multi_*` executes network I/O concurrently on background host threads and streams
//!   response metadata, headers, and body chunks back to `curl_multi_perform`, waking
//!   `curl_multi_poll` / `curl_multi_wait` via a condition variable.
//! * Honors `PORT_OFFLINE=1` by failing transfers with `CURLE_COULDNT_CONNECT` (`7`).

mod easy;
mod http;
mod multi;

use std::ffi::c_void;

pub use easy::{CurlMsg, CurlSlist, CurlVersionInfoData};

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

// ---------------------------------------------------------------------------
// Exported Win64 `libcurl.dll` API
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn curl_global_init(flags: i64) -> i32 {
    log::info!("libcurl (builtin): curl_global_init(flags={flags:#x}) -> CURLE_OK");
    easy::CURLE_OK
}

pub unsafe extern "win64" fn curl_global_init_mem(
    flags: i64,
    _m: usize,
    _f: usize,
    _r: usize,
    _s: usize,
    _c: usize,
) -> i32 {
    log::info!("libcurl (builtin): curl_global_init_mem(flags={flags:#x}) -> CURLE_OK");
    easy::CURLE_OK
}

pub unsafe extern "win64" fn curl_global_cleanup() {}

pub unsafe extern "win64" fn curl_version() -> *const u8 {
    b"libcurl/8.7.1 rustls/0.23 zlib/1.2.12\0".as_ptr()
}

pub unsafe extern "win64" fn curl_version_info(_age: i32) -> *const CurlVersionInfoData {
    easy::version_info_ptr()
}

pub unsafe extern "win64" fn curl_easy_init() -> usize {
    easy::easy_init()
}

pub unsafe extern "win64" fn curl_easy_duphandle(easy: usize) -> usize {
    easy::easy_duphandle(easy)
}

pub unsafe extern "win64" fn curl_easy_cleanup(easy: usize) {
    easy::easy_cleanup(easy);
}

pub unsafe extern "win64" fn curl_easy_reset(easy: usize) {
    easy::easy_reset(easy);
}

pub unsafe extern "win64" fn curl_easy_setopt(easy: usize, option: i32, param: usize) -> i32 {
    easy::easy_setopt(easy, option, param)
}

pub unsafe extern "win64" fn curl_easy_perform(easy: usize) -> i32 {
    easy::easy_perform(easy)
}

pub unsafe extern "win64" fn curl_easy_getinfo(easy: usize, info: i32, out: *mut c_void) -> i32 {
    easy::easy_getinfo(easy, info, out)
}

pub unsafe extern "win64" fn curl_easy_strerror(code: i32) -> *const u8 {
    easy::easy_strerror(code)
}

pub unsafe extern "win64" fn curl_easy_escape(
    _easy: usize,
    string: *const u8,
    length: i32,
) -> *mut u8 {
    easy::easy_escape(string, length)
}

pub unsafe extern "win64" fn curl_easy_unescape(
    _easy: usize,
    string: *const u8,
    length: i32,
    outlength: *mut i32,
) -> *mut u8 {
    easy::easy_unescape(string, length, outlength)
}

pub unsafe extern "win64" fn curl_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        crate::heap::free(ptr);
    }
}

pub unsafe extern "win64" fn curl_slist_append(
    list: *mut CurlSlist,
    s: *const u8,
) -> *mut CurlSlist {
    easy::slist_append(list, s)
}

pub unsafe extern "win64" fn curl_slist_free_all(list: *mut CurlSlist) {
    easy::slist_free_all(list);
}

// ---------------------------------------------------------------------------
// Exported Win64 `curl_multi_*` & `curl_share_*` API
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn curl_multi_init() -> usize {
    multi::multi_init()
}

pub unsafe extern "win64" fn curl_multi_cleanup(multi: usize) -> i32 {
    multi::multi_cleanup(multi)
}

pub unsafe extern "win64" fn curl_multi_add_handle(multi: usize, easy: usize) -> i32 {
    multi::multi_add_handle(multi, easy)
}

pub unsafe extern "win64" fn curl_multi_remove_handle(multi: usize, easy: usize) -> i32 {
    multi::multi_remove_handle(multi, easy)
}

pub unsafe extern "win64" fn curl_multi_perform(multi: usize, running_handles: *mut i32) -> i32 {
    multi::multi_perform(multi, running_handles)
}

pub unsafe extern "win64" fn curl_multi_poll(
    multi: usize,
    _extra_fds: usize,
    extra_nfds: u32,
    timeout_ms: i32,
    numfds: *mut i32,
) -> i32 {
    multi::multi_wait_impl(multi, extra_nfds, timeout_ms, numfds, true)
}

pub unsafe extern "win64" fn curl_multi_wait(
    multi: usize,
    _extra_fds: usize,
    extra_nfds: u32,
    timeout_ms: i32,
    numfds: *mut i32,
) -> i32 {
    multi::multi_wait_impl(multi, extra_nfds, timeout_ms, numfds, false)
}

pub unsafe extern "win64" fn curl_multi_wakeup(multi: usize) -> i32 {
    multi::multi_wakeup(multi)
}

pub unsafe extern "win64" fn curl_multi_info_read(
    multi: usize,
    msgs_in_queue: *mut i32,
) -> usize {
    multi::multi_info_read(multi, msgs_in_queue)
}

pub unsafe extern "win64" fn curl_multi_setopt(multi: usize, option: i32, param: usize) -> i32 {
    multi::multi_setopt(multi, option, param)
}

pub unsafe extern "win64" fn curl_multi_strerror(code: i32) -> *const u8 {
    multi::multi_strerror(code)
}

pub unsafe extern "win64" fn curl_share_init() -> usize {
    easy::share_init()
}

pub unsafe extern "win64" fn curl_share_setopt(share: usize, option: i32, param: usize) -> i32 {
    easy::share_setopt(share, option, param)
}

pub unsafe extern "win64" fn curl_share_cleanup(share: usize) -> i32 {
    easy::share_cleanup(share)
}

pub unsafe extern "win64" fn curl_share_strerror(code: i32) -> *const u8 {
    easy::share_strerror(code)
}

#[cfg(test)]
mod tests;
