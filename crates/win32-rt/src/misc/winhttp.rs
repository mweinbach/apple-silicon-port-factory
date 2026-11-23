//! `winhttp.dll` / `wininet.dll` / `sensapi.dll`: real HTTP/1.1 + HTTPS client
//! (TLS 1.2/1.3 via shared `crate::net_tls`), URL cracking/building,
//! proxy configuration queries, and POSIX interface network-liveness detection.
//!
//! Works identically on macOS (`aarch64-apple-darwin`) and iOS
//! (`aarch64-apple-ios`) with zero platform-specific UI frameworks.

mod http;
mod state;
mod url;

pub use state::{
    URL_COMPONENTSA, URL_COMPONENTSW, WINHTTP_AUTOPROXY_OPTIONS,
    WINHTTP_CURRENT_USER_IE_PROXY_CONFIG, WINHTTP_PROXY_INFO,
};

use crate::strings::{self, DEFAULT_MAX_STR_LEN};
use crate::types::*;
use state::{
    WinHttpHandle, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER,
    ERROR_WINHTTP_AUTODETECTION_FAILED, ERROR_WINHTTP_INCORRECT_HANDLE_TYPE, HANDLES,
    INTERNET_CONNECTION_LAN, NETWORK_ALIVE_LAN, WINHTTP_ACCESS_TYPE_NO_PROXY, WINHTTP_QUERY_CUSTOM,
};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Exported WinHTTP / SENSAPI functions
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn WinHttpCheckPlatform() -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn WinHttpGetDefaultProxyConfiguration(
    info: *mut WINHTTP_PROXY_INFO,
) -> BOOL {
    if info.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    log::debug!("WinHttpGetDefaultProxyConfiguration -> WINHTTP_ACCESS_TYPE_NO_PROXY");
    strings::wr(
        info,
        WINHTTP_PROXY_INFO {
            access_type: WINHTTP_ACCESS_TYPE_NO_PROXY,
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        },
    );
    TRUE
}

pub unsafe extern "win64" fn WinHttpGetIEProxyConfigForCurrentUser(
    cfg: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG,
) -> BOOL {
    if cfg.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    log::debug!("WinHttpGetIEProxyConfigForCurrentUser -> no IE/WPAD proxy configured");
    strings::wr(
        cfg,
        WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
            auto_detect: FALSE,
            auto_config_url: std::ptr::null_mut(),
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        },
    );
    TRUE
}

pub unsafe extern "win64" fn WinHttpGetProxyForUrl(
    session: usize,
    url: LPCWSTR,
    auto_proxy_options: *const WINHTTP_AUTOPROXY_OPTIONS,
    proxy_info: *mut WINHTTP_PROXY_INFO,
) -> BOOL {
    if url.is_null() || auto_proxy_options.is_null() || proxy_info.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    match HANDLES.with(session, |h| matches!(h, WinHttpHandle::Session(_))) {
        Some(true) => {}
        Some(false) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    }
    let opts = strings::rd(auto_proxy_options);
    strings::wr(
        proxy_info,
        WINHTTP_PROXY_INFO {
            access_type: WINHTTP_ACCESS_TYPE_NO_PROXY,
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        },
    );
    log::debug!(
        "WinHttpGetProxyForUrl(flags={:#x}, auto_detect={:#x}) -> ERROR_WINHTTP_AUTODETECTION_FAILED",
        opts.dw_flags,
        opts.dw_auto_detect_flags
    );
    crate::teb::set_last_error(ERROR_WINHTTP_AUTODETECTION_FAILED);
    FALSE
}

pub unsafe extern "win64" fn WinHttpDetectAutoProxyConfigUrl(
    _auto_detect_flags: u32,
    auto_config_url: *mut LPWSTR,
) -> BOOL {
    if !auto_config_url.is_null() {
        strings::wr(auto_config_url, std::ptr::null_mut());
    }
    crate::teb::set_last_error(ERROR_WINHTTP_AUTODETECTION_FAILED);
    FALSE
}

pub unsafe extern "win64" fn IsNetworkAlive(flags: *mut u32) -> BOOL {
    if state::is_offline_mode() {
        if !flags.is_null() {
            strings::wr(flags, 0);
        }
        return FALSE;
    }
    if state::has_active_network_interface() {
        if !flags.is_null() {
            strings::wr(flags, NETWORK_ALIVE_LAN);
        }
        TRUE
    } else {
        if !flags.is_null() {
            strings::wr(flags, 0);
        }
        FALSE
    }
}

pub unsafe extern "win64" fn WinHttpOpen(
    user_agent: LPCWSTR,
    _access_type: u32,
    _proxy_name: LPCWSTR,
    _proxy_bypass: LPCWSTR,
    _flags: u32,
) -> usize {
    state::open_session_impl(strings::read_wide_z_opt(user_agent, DEFAULT_MAX_STR_LEN))
}

pub unsafe extern "win64" fn WinHttpConnect(
    session: usize,
    server_name: LPCWSTR,
    server_port: u16,
    reserved: u32,
) -> usize {
    if reserved != 0 || server_name.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(host) = strings::read_wide_z_opt(server_name, DEFAULT_MAX_STR_LEN) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    state::connect_impl(session, host, server_port, 0)
}

pub unsafe extern "win64" fn WinHttpOpenRequest(
    connect: usize,
    verb: LPCWSTR,
    object_name: LPCWSTR,
    version: LPCWSTR,
    referrer: LPCWSTR,
    accept_types: *const LPCWSTR,
    flags: u32,
) -> usize {
    state::open_request_impl(
        connect,
        strings::read_wide_z_opt(verb, DEFAULT_MAX_STR_LEN),
        strings::read_wide_z_opt(object_name, DEFAULT_MAX_STR_LEN),
        strings::read_wide_z_opt(version, DEFAULT_MAX_STR_LEN),
        strings::read_wide_z_opt(referrer, DEFAULT_MAX_STR_LEN),
        state::read_wide_z_list(accept_types),
        flags,
        0,
        0,
        0,
    )
}

pub unsafe extern "win64" fn WinHttpAddRequestHeaders(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    modifiers: u32,
) -> BOOL {
    let Some(raw) = strings::read_wide_opt_len(headers, headers_len as i32) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    state::add_request_headers_impl(request, &raw, modifiers)
}

pub unsafe extern "win64" fn WinHttpSetTimeouts(
    h_internet: usize,
    resolve_timeout: i32,
    connect_timeout: i32,
    send_timeout: i32,
    receive_timeout: i32,
) -> BOOL {
    state::set_timeouts_impl(
        h_internet,
        resolve_timeout,
        connect_timeout,
        send_timeout,
        receive_timeout,
    )
}

pub unsafe extern "win64" fn WinHttpSetOption(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
) -> BOOL {
    state::set_option_impl(h_internet, option, buffer, buffer_len, false)
}

pub unsafe extern "win64" fn WinHttpQueryOption(
    h_internet: usize,
    option: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
) -> BOOL {
    state::query_option_impl(h_internet, option, buffer, buffer_len, false)
}

pub unsafe extern "win64" fn WinHttpSetStatusCallback(
    h_internet: usize,
    callback: usize,
    _notification_flags: u32,
    _reserved: usize,
) -> usize {
    state::set_status_callback_impl(h_internet, callback)
}

pub unsafe extern "win64" fn WinHttpSetCredentials(
    request: usize,
    auth_targets: u32,
    auth_scheme: u32,
    user_name: LPCWSTR,
    password: LPCWSTR,
    _auth_params: *mut u8,
) -> BOOL {
    state::set_credentials_impl(request, auth_targets, auth_scheme, user_name, password)
}

pub unsafe extern "win64" fn WinHttpQueryAuthSchemes(
    request: usize,
    supported_schemes: *mut u32,
    first_scheme: *mut u32,
    auth_target: *mut u32,
) -> BOOL {
    state::query_auth_schemes_impl(request, supported_schemes, first_scheme, auth_target)
}

pub unsafe extern "win64" fn WinHttpSendRequest(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    optional: *const u8,
    optional_len: u32,
    total_len: u32,
    context: usize,
) -> BOOL {
    let extra_headers = strings::read_wide_opt_len(headers, headers_len as i32);
    state::send_request_impl(
        request,
        extra_headers,
        optional,
        optional_len,
        total_len,
        context,
    )
}

pub unsafe extern "win64" fn WinHttpWriteData(
    request: usize,
    buffer: *const u8,
    bytes_to_write: u32,
    bytes_written: *mut u32,
) -> BOOL {
    state::write_data_impl(request, buffer, bytes_to_write, bytes_written)
}

pub unsafe extern "win64" fn WinHttpReceiveResponse(request: usize, _reserved: *mut u8) -> BOOL {
    state::receive_response_impl(request)
}

pub unsafe extern "win64" fn WinHttpQueryHeaders(
    request: usize,
    info_level: u32,
    name: LPCWSTR,
    buffer: *mut u8,
    buffer_len: *mut u32,
    index: *mut u32,
) -> BOOL {
    let custom_name = strings::read_wide_z_opt(name, DEFAULT_MAX_STR_LEN);
    state::query_headers_impl(request, info_level, custom_name, buffer, buffer_len, index, false)
}

pub unsafe extern "win64" fn WinHttpQueryDataAvailable(
    request: usize,
    bytes_available: *mut u32,
) -> BOOL {
    state::query_data_available_impl(request, bytes_available)
}

pub unsafe extern "win64" fn WinHttpReadData(
    request: usize,
    buffer: *mut u8,
    bytes_to_read: u32,
    bytes_read: *mut u32,
) -> BOOL {
    state::read_data_impl(request, buffer, bytes_to_read, bytes_read)
}

pub unsafe extern "win64" fn WinHttpCrackUrl(
    url: LPCWSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSW,
) -> BOOL {
    url::crack_url_w_impl(url, url_len, flags, components, false)
}

pub unsafe extern "win64" fn WinHttpCreateUrl(
    components: *const URL_COMPONENTSW,
    flags: u32,
    url: LPWSTR,
    url_len: *mut u32,
) -> BOOL {
    url::create_url_w_impl(components, flags, url, url_len)
}

pub unsafe extern "win64" fn WinHttpCloseHandle(h_internet: usize) -> BOOL {
    if h_internet == 0 {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if HANDLES.remove(h_internet).is_some() {
        TRUE
    } else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        FALSE
    }
}

// ---------------------------------------------------------------------------
// Exported WinINet functions (`wininet.dll` aliases to `winhttp.dll`)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn InternetGetConnectedState(flags: *mut u32, _reserved: u32) -> BOOL {
    if state::is_offline_mode() {
        if !flags.is_null() {
            strings::wr(flags, 0);
        }
        return FALSE;
    }
    if state::has_active_network_interface() {
        if !flags.is_null() {
            strings::wr(flags, INTERNET_CONNECTION_LAN);
        }
        TRUE
    } else {
        if !flags.is_null() {
            strings::wr(flags, 0);
        }
        FALSE
    }
}

pub unsafe extern "win64" fn InternetOpenW(
    agent: LPCWSTR,
    _access_type: u32,
    _proxy: LPCWSTR,
    _proxy_bypass: LPCWSTR,
    _flags: u32,
) -> usize {
    state::open_session_impl(strings::read_wide_z_opt(agent, DEFAULT_MAX_STR_LEN))
}

pub unsafe extern "win64" fn InternetOpenA(
    agent: LPCSTR,
    _access_type: u32,
    _proxy: LPCSTR,
    _proxy_bypass: LPCSTR,
    _flags: u32,
) -> usize {
    state::open_session_impl(strings::read_ansi_z_opt(agent, DEFAULT_MAX_STR_LEN))
}

pub unsafe extern "win64" fn InternetConnectW(
    session: usize,
    server_name: LPCWSTR,
    server_port: u16,
    _user_name: LPCWSTR,
    _password: LPCWSTR,
    _service: u32,
    _flags: u32,
    context: usize,
) -> usize {
    let Some(host) = strings::read_wide_z_opt(server_name, DEFAULT_MAX_STR_LEN) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    state::connect_impl(session, host, server_port, context)
}

pub unsafe extern "win64" fn InternetConnectA(
    session: usize,
    server_name: LPCSTR,
    server_port: u16,
    _user_name: LPCSTR,
    _password: LPCSTR,
    _service: u32,
    _flags: u32,
    context: usize,
) -> usize {
    let Some(host) = strings::read_ansi_z_opt(server_name, DEFAULT_MAX_STR_LEN) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    state::connect_impl(session, host, server_port, context)
}

pub unsafe extern "win64" fn HttpOpenRequestW(
    connect: usize,
    verb: LPCWSTR,
    object_name: LPCWSTR,
    version: LPCWSTR,
    referrer: LPCWSTR,
    accept_types: *const LPCWSTR,
    flags: u32,
    context: usize,
) -> usize {
    let (disable, sec) = state::translate_wininet_request_flags(flags);
    state::open_request_impl(
        connect,
        strings::read_wide_z_opt(verb, DEFAULT_MAX_STR_LEN),
        strings::read_wide_z_opt(object_name, DEFAULT_MAX_STR_LEN),
        strings::read_wide_z_opt(version, DEFAULT_MAX_STR_LEN),
        strings::read_wide_z_opt(referrer, DEFAULT_MAX_STR_LEN),
        state::read_wide_z_list(accept_types),
        flags,
        disable,
        sec,
        context,
    )
}

pub unsafe extern "win64" fn HttpOpenRequestA(
    connect: usize,
    verb: LPCSTR,
    object_name: LPCSTR,
    version: LPCSTR,
    referrer: LPCSTR,
    accept_types: *const LPCSTR,
    flags: u32,
    context: usize,
) -> usize {
    let (disable, sec) = state::translate_wininet_request_flags(flags);
    state::open_request_impl(
        connect,
        strings::read_ansi_z_opt(verb, DEFAULT_MAX_STR_LEN),
        strings::read_ansi_z_opt(object_name, DEFAULT_MAX_STR_LEN),
        strings::read_ansi_z_opt(version, DEFAULT_MAX_STR_LEN),
        strings::read_ansi_z_opt(referrer, DEFAULT_MAX_STR_LEN),
        state::read_ansi_z_list(accept_types),
        flags,
        disable,
        sec,
        context,
    )
}

pub unsafe extern "win64" fn HttpAddRequestHeadersW(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    modifiers: u32,
) -> BOOL {
    WinHttpAddRequestHeaders(request, headers, headers_len, modifiers)
}

pub unsafe extern "win64" fn HttpAddRequestHeadersA(
    request: usize,
    headers: LPCSTR,
    headers_len: u32,
    modifiers: u32,
) -> BOOL {
    let Some(raw) = strings::read_ansi_opt_len(headers, headers_len as i32) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    state::add_request_headers_impl(request, &raw, modifiers)
}

pub unsafe extern "win64" fn HttpSendRequestW(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    optional: *const u8,
    optional_len: u32,
) -> BOOL {
    let extra = strings::read_wide_opt_len(headers, headers_len as i32);
    if state::send_request_impl(request, extra, optional, optional_len, optional_len, 0) == FALSE {
        return FALSE;
    }
    state::receive_response_impl(request)
}

pub unsafe extern "win64" fn HttpSendRequestA(
    request: usize,
    headers: LPCSTR,
    headers_len: u32,
    optional: *const u8,
    optional_len: u32,
) -> BOOL {
    let extra = strings::read_ansi_opt_len(headers, headers_len as i32);
    if state::send_request_impl(request, extra, optional, optional_len, optional_len, 0) == FALSE {
        return FALSE;
    }
    state::receive_response_impl(request)
}

pub unsafe extern "win64" fn HttpQueryInfoW(
    request: usize,
    info_level: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
    index: *mut u32,
) -> BOOL {
    let custom_name = if (info_level & 0xFFFF) == WINHTTP_QUERY_CUSTOM && !buffer.is_null() {
        strings::read_wide_z_opt(buffer as LPCWSTR, DEFAULT_MAX_STR_LEN)
    } else {
        None
    };
    state::query_headers_impl(request, info_level, custom_name, buffer, buffer_len, index, false)
}

pub unsafe extern "win64" fn HttpQueryInfoA(
    request: usize,
    info_level: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
    index: *mut u32,
) -> BOOL {
    let custom_name = if (info_level & 0xFFFF) == WINHTTP_QUERY_CUSTOM && !buffer.is_null() {
        strings::read_ansi_z_opt(buffer as LPCSTR, DEFAULT_MAX_STR_LEN)
    } else {
        None
    };
    state::query_headers_impl(request, info_level, custom_name, buffer, buffer_len, index, true)
}

pub unsafe extern "win64" fn InternetOpenUrlW(
    session: usize,
    url: LPCWSTR,
    headers: LPCWSTR,
    headers_len: u32,
    flags: u32,
    context: usize,
) -> usize {
    let Some(url_str) = strings::read_wide_z_opt(url, DEFAULT_MAX_STR_LEN) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    let hdrs = strings::read_wide_opt_len(headers, headers_len as i32);
    state::internet_open_url_impl(session, &url_str, hdrs, flags, context)
}

pub unsafe extern "win64" fn InternetOpenUrlA(
    session: usize,
    url: LPCSTR,
    headers: LPCSTR,
    headers_len: u32,
    flags: u32,
    context: usize,
) -> usize {
    let Some(url_str) = strings::read_ansi_z_opt(url, DEFAULT_MAX_STR_LEN) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    let hdrs = strings::read_ansi_opt_len(headers, headers_len as i32);
    state::internet_open_url_impl(session, &url_str, hdrs, flags, context)
}

pub unsafe extern "win64" fn InternetReadFile(
    request: usize,
    buffer: *mut u8,
    bytes_to_read: u32,
    bytes_read: *mut u32,
) -> BOOL {
    WinHttpReadData(request, buffer, bytes_to_read, bytes_read)
}

pub unsafe extern "win64" fn InternetWriteFile(
    request: usize,
    buffer: *const u8,
    bytes_to_write: u32,
    bytes_written: *mut u32,
) -> BOOL {
    WinHttpWriteData(request, buffer, bytes_to_write, bytes_written)
}

pub unsafe extern "win64" fn InternetQueryDataAvailable(
    request: usize,
    bytes_available: *mut u32,
    _flags: u32,
    _context: usize,
) -> BOOL {
    WinHttpQueryDataAvailable(request, bytes_available)
}

pub unsafe extern "win64" fn InternetCloseHandle(h_internet: usize) -> BOOL {
    WinHttpCloseHandle(h_internet)
}

pub unsafe extern "win64" fn InternetSetOptionW(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
) -> BOOL {
    state::set_option_impl(
        h_internet,
        state::map_wininet_option(option),
        buffer,
        buffer_len,
        false,
    )
}

pub unsafe extern "win64" fn InternetSetOptionA(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
) -> BOOL {
    state::set_option_impl(
        h_internet,
        state::map_wininet_option(option),
        buffer,
        buffer_len,
        true,
    )
}

pub unsafe extern "win64" fn InternetQueryOptionW(
    h_internet: usize,
    option: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
) -> BOOL {
    state::query_option_impl(
        h_internet,
        state::map_wininet_option(option),
        buffer,
        buffer_len,
        false,
    )
}

pub unsafe extern "win64" fn InternetQueryOptionA(
    h_internet: usize,
    option: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
) -> BOOL {
    state::query_option_impl(
        h_internet,
        state::map_wininet_option(option),
        buffer,
        buffer_len,
        true,
    )
}

pub unsafe extern "win64" fn InternetCrackUrlW(
    url: LPCWSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSW,
) -> BOOL {
    url::crack_url_w_impl(url, url_len, flags, components, true)
}

pub unsafe extern "win64" fn InternetCrackUrlA(
    url: LPCSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSA,
) -> BOOL {
    url::crack_url_a_impl(url, url_len, flags, components)
}

#[cfg(test)]
mod tests;
