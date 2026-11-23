use super::state::{
    find_header, ERROR_INSUFFICIENT_BUFFER, ERROR_WINHTTP_AUTODETECTION_FAILED,
    ERROR_WINHTTP_CANNOT_CONNECT, ERROR_WINHTTP_HEADER_NOT_FOUND,
    ERROR_WINHTTP_UNRECOGNIZED_SCHEME, WINHTTP_ACCESS_TYPE_NO_PROXY, WINHTTP_ADDREQ_FLAG_ADD,
    WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA, WINHTTP_ADDREQ_FLAG_REPLACE,
    WINHTTP_HANDLE_TYPE_REQUEST, WINHTTP_OPTION_HANDLE_TYPE, WINHTTP_OPTION_REDIRECT_POLICY,
    WINHTTP_OPTION_REDIRECT_POLICY_NEVER, WINHTTP_OPTION_URL, WINHTTP_QUERY_CUSTOM,
    WINHTTP_QUERY_FLAG_NUMBER, WINHTTP_QUERY_RAW_HEADERS, WINHTTP_QUERY_RAW_HEADERS_CRLF,
    WINHTTP_QUERY_STATUS_CODE, WINHTTP_QUERY_STATUS_TEXT, WINHTTP_SCHEME_HTTP,
    WINHTTP_SCHEME_HTTPS,
};
use super::*;
use crate::net_tls::{find_header_end, parse_content_length};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{LazyLock, RwLock};
use std::time::Duration;

/// Coordinates tests that read vs temporarily mutate `PORT_OFFLINE`.
static OFFLINE_ENV_LOCK: LazyLock<RwLock<()>> = LazyLock::new(|| RwLock::new(()));

struct OfflineGuard {
    prev: Option<String>,
}

impl OfflineGuard {
    fn set(val: &str) -> Self {
        let prev = std::env::var("PORT_OFFLINE").ok();
        std::env::set_var("PORT_OFFLINE", val);
        Self { prev }
    }
}

impl Drop for OfflineGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => std::env::set_var("PORT_OFFLINE", v),
            None => std::env::remove_var("PORT_OFFLINE"),
        }
    }
}

fn wide_z(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn read_http_request(stream: &mut TcpStream) -> (String, Vec<(String, String)>, Vec<u8>) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let mut data = Vec::new();
    let mut tmp = [0u8; 4096];
    let header_end = loop {
        if let Some((end, sep)) = find_header_end(&data) {
            break (end, sep);
        }
        let n = stream.read(&mut tmp).expect("read request headers");
        assert!(n > 0, "unexpected EOF before HTTP request headers");
        data.extend_from_slice(&tmp[..n]);
    };
    let header_text = String::from_utf8_lossy(&data[..header_end.0]).into_owned();
    let mut lines = header_text.split('\n');
    let req_line = lines.next().unwrap_or("").trim_end_matches('\r').to_string();
    let mut headers = Vec::new();
    for l in lines {
        let l = l.trim_end_matches('\r');
        if let Some((k, v)) = l.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    let content_len = parse_content_length(&headers).unwrap_or(0);
    let mut body = data[header_end.0 + header_end.1..].to_vec();
    while body.len() < content_len {
        let n = stream.read(&mut tmp).expect("read request body");
        assert!(n > 0, "unexpected EOF reading request body");
        body.extend_from_slice(&tmp[..n]);
    }
    body.truncate(content_len);
    (req_line, headers, body)
}

unsafe fn query_header_wide(req: usize, info_level: u32, custom: Option<&str>) -> String {
    let custom_w = custom.map(wide_z);
    let name_ptr = custom_w
        .as_ref()
        .map_or(std::ptr::null(), |v| v.as_ptr());
    let mut needed: u32 = 0;
    let ok = WinHttpQueryHeaders(
        req,
        info_level,
        name_ptr,
        std::ptr::null_mut(),
        &mut needed,
        std::ptr::null_mut(),
    );
    assert_eq!(ok, FALSE);
    assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
    assert!(needed >= 2);
    let mut buf = vec![0u8; needed as usize];
    let mut len = needed;
    let ok2 = WinHttpQueryHeaders(
        req,
        info_level,
        name_ptr,
        buf.as_mut_ptr(),
        &mut len,
        std::ptr::null_mut(),
    );
    assert_eq!(ok2, TRUE);
    let units: Vec<u16> = buf[..len as usize]
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    String::from_utf16_lossy(&units)
}

unsafe fn read_all_body(req: usize, chunk_size: u32) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut avail: u32 = 0;
        assert_eq!(WinHttpQueryDataAvailable(req, &mut avail), TRUE);
        if avail == 0 {
            let mut tmp = [0u8; 16];
            let mut got: u32 = 99;
            assert_eq!(WinHttpReadData(req, tmp.as_mut_ptr(), 16, &mut got), TRUE);
            assert_eq!(got, 0);
            break;
        }
        let to_read = avail.min(chunk_size);
        let mut buf = vec![0u8; to_read as usize];
        let mut got: u32 = 0;
        assert_eq!(WinHttpReadData(req, buf.as_mut_ptr(), to_read, &mut got), TRUE);
        assert!(got > 0);
        out.extend_from_slice(&buf[..got as usize]);
    }
    out
}

#[test]
fn http_get_content_length_headers_and_options() {
    let _guard = OFFLINE_ENV_LOCK.read().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let body_payload = b"Hello from WinHTTP HTTP/1.1!";
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let (req_line, headers, body) = read_http_request(&mut stream);
        assert_eq!(req_line, "GET /api/status?v=2 HTTP/1.1");
        assert!(body.is_empty());
        assert_eq!(find_header(&headers, "User-Agent"), Some("WinHttpTest/1.0"));
        assert_eq!(find_header(&headers, "X-Test"), Some("updated"));
        assert_eq!(find_header(&headers, "X-List"), Some("alpha, beta"));
        assert_eq!(find_header(&headers, "X-Extra"), Some("send-hdr"));
        let resp = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/plain; charset=utf-8\r\n\
             Content-Length: {}\r\n\
             X-Server-Tag: native-rust\r\n\
             Connection: close\r\n\r\n",
            body_payload.len()
        );
        stream.write_all(resp.as_bytes()).unwrap();
        stream.write_all(body_payload).unwrap();
    });

    unsafe {
        let ua = wide_z("WinHttpTest/1.0");
        let session = WinHttpOpen(
            ua.as_ptr(),
            WINHTTP_ACCESS_TYPE_NO_PROXY,
            std::ptr::null(),
            std::ptr::null(),
            0,
        );
        assert_ne!(session, 0);
        assert_eq!(WinHttpSetTimeouts(session, 2000, 5000, 5000, 5000), TRUE);

        let host = wide_z("127.0.0.1");
        let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
        assert_ne!(conn, 0);

        let verb = wide_z("GET");
        let path = wide_z("/api/status?v=2");
        let req = WinHttpOpenRequest(
            conn,
            verb.as_ptr(),
            path.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            0,
        );
        assert_ne!(req, 0);

        let h1 = wide_z("X-Test: initial\r\nX-List: alpha");
        assert_eq!(
            WinHttpAddRequestHeaders(req, h1.as_ptr(), u32::MAX, WINHTTP_ADDREQ_FLAG_ADD),
            TRUE
        );
        let h2 = wide_z("X-Test: updated");
        assert_eq!(
            WinHttpAddRequestHeaders(req, h2.as_ptr(), u32::MAX, WINHTTP_ADDREQ_FLAG_REPLACE),
            TRUE
        );
        let h3 = wide_z("X-List: beta");
        assert_eq!(
            WinHttpAddRequestHeaders(
                req,
                h3.as_ptr(),
                u32::MAX,
                WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA,
            ),
            TRUE
        );

        let extra = wide_z("X-Extra: send-hdr");
        assert_eq!(
            WinHttpSendRequest(req, extra.as_ptr(), u32::MAX, std::ptr::null(), 0, 0, 0x1234),
            TRUE
        );
        assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), TRUE);

        // Status code as DWORD
        let mut status_num: u32 = 0;
        let mut status_len: u32 = 4;
        assert_eq!(
            WinHttpQueryHeaders(
                req,
                WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                std::ptr::null(),
                &mut status_num as *mut u32 as *mut u8,
                &mut status_len,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        assert_eq!(status_num, 200);

        // Content-Length as DWORD
        let mut cl_num: u32 = 0;
        let mut cl_len: u32 = 4;
        assert_eq!(
            WinHttpQueryHeaders(
                req,
                5 | WINHTTP_QUERY_FLAG_NUMBER,
                std::ptr::null(),
                &mut cl_num as *mut u32 as *mut u8,
                &mut cl_len,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        assert_eq!(cl_num as usize, body_payload.len());

        // String queries (all exercise ERROR_INSUFFICIENT_BUFFER probe first)
        assert_eq!(query_header_wide(req, WINHTTP_QUERY_STATUS_CODE, None), "200");
        assert_eq!(query_header_wide(req, WINHTTP_QUERY_STATUS_TEXT, None), "OK");
        assert_eq!(
            query_header_wide(req, 1, None),
            "text/plain; charset=utf-8"
        );
        assert_eq!(
            query_header_wide(req, WINHTTP_QUERY_CUSTOM, Some("X-Server-Tag")),
            "native-rust"
        );
        let raw_crlf = query_header_wide(req, WINHTTP_QUERY_RAW_HEADERS_CRLF, None);
        assert!(raw_crlf.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(raw_crlf.contains("X-Server-Tag: native-rust\r\n"));
        assert!(raw_crlf.ends_with("\r\n\r\n"));

        let raw_nul = query_header_wide(req, WINHTTP_QUERY_RAW_HEADERS, None);
        assert!(raw_nul.starts_with("HTTP/1.1 200 OK\0"));
        assert!(raw_nul.contains("X-Server-Tag: native-rust\0"));

        // Query options
        let mut handle_ty: u32 = 0;
        let mut opt_len: u32 = 4;
        assert_eq!(
            WinHttpQueryOption(
                req,
                WINHTTP_OPTION_HANDLE_TYPE,
                &mut handle_ty as *mut u32 as *mut u8,
                &mut opt_len,
            ),
            TRUE
        );
        assert_eq!(handle_ty, WINHTTP_HANDLE_TYPE_REQUEST);

        let mut url_bytes: u32 = 0;
        assert_eq!(
            WinHttpQueryOption(req, WINHTTP_OPTION_URL, std::ptr::null_mut(), &mut url_bytes),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
        let mut url_buf = vec![0u8; url_bytes as usize];
        assert_eq!(
            WinHttpQueryOption(req, WINHTTP_OPTION_URL, url_buf.as_mut_ptr(), &mut url_bytes),
            TRUE
        );
        let url_units: Vec<u16> = url_buf[..url_bytes as usize]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        assert_eq!(
            String::from_utf16_lossy(&url_units),
            format!("http://127.0.0.1:{port}/api/status?v=2")
        );

        // Stream body in small 7-byte slices
        let received = read_all_body(req, 7);
        assert_eq!(received, body_payload);

        assert_eq!(WinHttpCloseHandle(req), TRUE);
        assert_eq!(WinHttpCloseHandle(conn), TRUE);
        assert_eq!(WinHttpCloseHandle(session), TRUE);
        assert_eq!(WinHttpCloseHandle(session), FALSE);
    }

    server.join().unwrap();
}

#[test]
fn http_chunked_transfer_decoding() {
    let _guard = OFFLINE_ENV_LOCK.read().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut stream);
        let resp = concat!(
            "HTTP/1.1 200 OK\r\n",
            "Transfer-Encoding: chunked\r\n",
            "Content-Type: text/plain\r\n",
            "\r\n",
            "5;ext=1\r\nHello\r\n",
            "2\r\n, \r\n",
            "6\r\nWorld!\r\n",
            "0\r\n",
            "X-Trailer: ignored\r\n",
            "\r\n",
        );
        stream.write_all(resp.as_bytes()).unwrap();
    });

    unsafe {
        let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
        let host = wide_z("127.0.0.1");
        let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
        let path = wide_z("/chunked");
        let req = WinHttpOpenRequest(conn, std::ptr::null(), path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
        assert_eq!(WinHttpSendRequest(req, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
        assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), TRUE);

        // Content-Length header was not sent on chunked response.
        let mut cl: u32 = 0;
        let mut cl_len: u32 = 4;
        assert_eq!(
            WinHttpQueryHeaders(req, 5 | WINHTTP_QUERY_FLAG_NUMBER, std::ptr::null(), &mut cl as *mut u32 as *mut u8, &mut cl_len, std::ptr::null_mut()),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_HEADER_NOT_FOUND);

        let body = read_all_body(req, 64);
        assert_eq!(body, b"Hello, World!");

        WinHttpCloseHandle(req);
        WinHttpCloseHandle(conn);
        WinHttpCloseHandle(session);
    }

    server.join().unwrap();
}

#[test]
fn http_redirect_302_follow_and_disable() {
    let _guard = OFFLINE_ENV_LOCK.read().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = std::thread::spawn(move || {
        // Serve 3 connections: 1st redirects -> 2nd final; 3rd redirects (client disables redirects).
        for _ in 0..3 {
            let (mut stream, _) = listener.accept().unwrap();
            let (req_line, _, _) = read_http_request(&mut stream);
            if req_line.starts_with("GET /start ") {
                let resp = "HTTP/1.1 302 Found\r\nLocation: /final?done=1\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                stream.write_all(resp.as_bytes()).unwrap();
            } else if req_line.starts_with("GET /final?done=1 ") {
                let body = b"redirected-ok";
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                stream.write_all(resp.as_bytes()).unwrap();
                stream.write_all(body).unwrap();
            } else {
                panic!("unexpected request line: {req_line}");
            }
        }
    });

    unsafe {
        let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
        let host = wide_z("127.0.0.1");
        let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
        let start_path = wide_z("/start");

        // Case 1: automatic redirect enabled (default)
        let req1 = WinHttpOpenRequest(conn, std::ptr::null(), start_path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
        assert_eq!(WinHttpSendRequest(req1, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
        assert_eq!(WinHttpReceiveResponse(req1, std::ptr::null_mut()), TRUE);
        assert_eq!(query_header_wide(req1, WINHTTP_QUERY_STATUS_CODE, None), "200");
        assert_eq!(read_all_body(req1, 64), b"redirected-ok");
        let mut ulen = 0u32;
        WinHttpQueryOption(req1, WINHTTP_OPTION_URL, std::ptr::null_mut(), &mut ulen);
        let mut ubuf = vec![0u8; ulen as usize];
        assert_eq!(WinHttpQueryOption(req1, WINHTTP_OPTION_URL, ubuf.as_mut_ptr(), &mut ulen), TRUE);
        let u16s: Vec<u16> = ubuf[..ulen as usize].chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        assert!(String::from_utf16_lossy(&u16s).ends_with("/final?done=1"));
        WinHttpCloseHandle(req1);

        // Case 2: redirects disabled via WINHTTP_OPTION_REDIRECT_POLICY_NEVER
        let req2 = WinHttpOpenRequest(conn, std::ptr::null(), start_path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
        let policy = WINHTTP_OPTION_REDIRECT_POLICY_NEVER;
        assert_eq!(
            WinHttpSetOption(req2, WINHTTP_OPTION_REDIRECT_POLICY, &policy as *const u32 as *const u8, 4),
            TRUE
        );
        assert_eq!(WinHttpSendRequest(req2, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
        assert_eq!(WinHttpReceiveResponse(req2, std::ptr::null_mut()), TRUE);
        assert_eq!(query_header_wide(req2, WINHTTP_QUERY_STATUS_CODE, None), "302");
        assert_eq!(query_header_wide(req2, 33, None), "/final?done=1");
        WinHttpCloseHandle(req2);

        WinHttpCloseHandle(conn);
        WinHttpCloseHandle(session);
    }

    server.join().unwrap();
}

#[test]
fn http_post_body_echo_with_write_data() {
    let _guard = OFFLINE_ENV_LOCK.read().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let (req_line, headers, body) = read_http_request(&mut stream);
        assert_eq!(req_line, "POST /echo HTTP/1.1");
        assert_eq!(find_header(&headers, "Content-Type"), Some("application/octet-stream"));
        let resp = format!(
            "HTTP/1.1 200 OK\r\nX-Method: POST\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(resp.as_bytes()).unwrap();
        stream.write_all(&body).unwrap();
    });

    unsafe {
        let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
        let host = wide_z("127.0.0.1");
        let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
        let verb = wide_z("POST");
        let path = wide_z("/echo");
        let req = WinHttpOpenRequest(conn, verb.as_ptr(), path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);

        let ct = wide_z("Content-Type: application/octet-stream");
        let part1 = b"first-part:";
        let part2 = b"second-streamed-part";
        let total = (part1.len() + part2.len()) as u32;
        assert_eq!(
            WinHttpSendRequest(
                req,
                ct.as_ptr(),
                u32::MAX,
                part1.as_ptr(),
                part1.len() as u32,
                total,
                0,
            ),
            TRUE
        );
        let mut written: u32 = 0;
        assert_eq!(
            WinHttpWriteData(req, part2.as_ptr(), part2.len() as u32, &mut written),
            TRUE
        );
        assert_eq!(written as usize, part2.len());

        assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), TRUE);
        assert_eq!(query_header_wide(req, WINHTTP_QUERY_CUSTOM, Some("X-Method")), "POST");
        let echoed = read_all_body(req, 128);
        assert_eq!(echoed, b"first-part:second-streamed-part");

        WinHttpCloseHandle(req);
        WinHttpCloseHandle(conn);
        WinHttpCloseHandle(session);
    }

    server.join().unwrap();
}

#[test]
fn crack_and_create_url_modes() {
    unsafe {
        let url = wide_z("https://user:pass@api.example.com:8443/v1/items?page=2&sort=asc");
        let mut scheme = vec![0u16; 16];
        let mut host = vec![0u16; 64];
        let mut user = vec![0u16; 32];
        let mut pass = vec![0u16; 32];
        let mut path = vec![0u16; 64];
        let mut extra = vec![0u16; 64];

        let mut comps = URL_COMPONENTSW {
            dw_struct_size: std::mem::size_of::<URL_COMPONENTSW>() as u32,
            lpsz_scheme: scheme.as_mut_ptr(),
            dw_scheme_length: scheme.len() as u32,
            n_scheme: 0,
            lpsz_host_name: host.as_mut_ptr(),
            dw_host_name_length: host.len() as u32,
            n_port: 0,
            lpsz_user_name: user.as_mut_ptr(),
            dw_user_name_length: user.len() as u32,
            lpsz_password: pass.as_mut_ptr(),
            dw_password_length: pass.len() as u32,
            lpsz_url_path: path.as_mut_ptr(),
            dw_url_path_length: path.len() as u32,
            lpsz_extra_info: extra.as_mut_ptr(),
            dw_extra_info_length: extra.len() as u32,
        };
        assert_eq!(WinHttpCrackUrl(url.as_ptr(), 0, 0, &mut comps), TRUE);
        assert_eq!(comps.n_scheme, WINHTTP_SCHEME_HTTPS);
        assert_eq!(comps.n_port, 8443);
        assert_eq!(String::from_utf16_lossy(&scheme[..comps.dw_scheme_length as usize]), "https");
        assert_eq!(String::from_utf16_lossy(&host[..comps.dw_host_name_length as usize]), "api.example.com");
        assert_eq!(String::from_utf16_lossy(&user[..comps.dw_user_name_length as usize]), "user");
        assert_eq!(String::from_utf16_lossy(&pass[..comps.dw_password_length as usize]), "pass");
        assert_eq!(String::from_utf16_lossy(&path[..comps.dw_url_path_length as usize]), "/v1/items");
        assert_eq!(String::from_utf16_lossy(&extra[..comps.dw_extra_info_length as usize]), "?page=2&sort=asc");

        // Roundtrip through WinHttpCreateUrl (buffer probe + write)
        let mut out_len: u32 = 0;
        assert_eq!(WinHttpCreateUrl(&comps, 0, std::ptr::null_mut(), &mut out_len), FALSE);
        assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
        let mut out_buf = vec![0u16; out_len as usize];
        assert_eq!(WinHttpCreateUrl(&comps, 0, out_buf.as_mut_ptr(), &mut out_len), TRUE);
        assert_eq!(
            String::from_utf16_lossy(&out_buf[..out_len as usize]),
            "https://user:pass@api.example.com:8443/v1/items?page=2&sort=asc"
        );

        // In-place pointer mode with ExtraInfo omitted -> UrlPath includes query string
        let url2 = wide_z("http://example.com/a/b?k=v");
        let mut ip = URL_COMPONENTSW {
            dw_struct_size: std::mem::size_of::<URL_COMPONENTSW>() as u32,
            lpsz_scheme: std::ptr::null_mut(),
            dw_scheme_length: 0,
            n_scheme: 0,
            lpsz_host_name: std::ptr::null_mut(),
            dw_host_name_length: u32::MAX,
            n_port: 0,
            lpsz_user_name: std::ptr::null_mut(),
            dw_user_name_length: 0,
            lpsz_password: std::ptr::null_mut(),
            dw_password_length: 0,
            lpsz_url_path: std::ptr::null_mut(),
            dw_url_path_length: u32::MAX,
            lpsz_extra_info: std::ptr::null_mut(),
            dw_extra_info_length: 0,
        };
        assert_eq!(WinHttpCrackUrl(url2.as_ptr(), 0, 0, &mut ip), TRUE);
        assert_eq!(ip.n_scheme, WINHTTP_SCHEME_HTTP);
        assert_eq!(ip.n_port, 80);
        let host_slice = std::slice::from_raw_parts(ip.lpsz_host_name, ip.dw_host_name_length as usize);
        let path_slice = std::slice::from_raw_parts(ip.lpsz_url_path, ip.dw_url_path_length as usize);
        assert_eq!(String::from_utf16_lossy(host_slice), "example.com");
        assert_eq!(String::from_utf16_lossy(path_slice), "/a/b?k=v");

        // Unrecognized scheme
        let bad = wide_z("ftp://example.com/file");
        assert_eq!(WinHttpCrackUrl(bad.as_ptr(), 0, 0, &mut ip), FALSE);
        assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_UNRECOGNIZED_SCHEME);
    }
}

#[test]
fn offline_mode_and_connection_refused_errors() {
    // 1) Connection refused on closed loopback port
    {
        let _guard = OFFLINE_ENV_LOCK.read().unwrap();
        let temp_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let closed_port = temp_listener.local_addr().unwrap().port();
        drop(temp_listener);

        unsafe {
            let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
            let host = wide_z("127.0.0.1");
            let conn = WinHttpConnect(session, host.as_ptr(), closed_port, 0);
            let req = WinHttpOpenRequest(conn, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
            assert_eq!(WinHttpSendRequest(req, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
            assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_CANNOT_CONNECT);
            WinHttpCloseHandle(req);
            WinHttpCloseHandle(conn);
            WinHttpCloseHandle(session);
        }
    }

    // 2) PORT_OFFLINE=1 blocks requests honestly with ERROR_WINHTTP_CANNOT_CONNECT
    {
        let _write_guard = OFFLINE_ENV_LOCK.write().unwrap();
        let _offline = OfflineGuard::set("1");

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let live_port = listener.local_addr().unwrap().port();

        unsafe {
            let mut alive_flags: u32 = 0xDEAD;
            assert_eq!(IsNetworkAlive(&mut alive_flags), FALSE);
            assert_eq!(alive_flags, 0);

            let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
            let host = wide_z("127.0.0.1");
            let conn = WinHttpConnect(session, host.as_ptr(), live_port, 0);
            let req = WinHttpOpenRequest(conn, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
            assert_eq!(WinHttpSendRequest(req, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
            assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_CANNOT_CONNECT);
            WinHttpCloseHandle(req);
            WinHttpCloseHandle(conn);
            WinHttpCloseHandle(session);
        }
        drop(listener);
    }
}

#[test]
fn proxy_queries_and_wininet_roundtrip() {
    let _guard = OFFLINE_ENV_LOCK.read().unwrap();
    unsafe {
        let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
        let mut def_proxy = WINHTTP_PROXY_INFO {
            access_type: 99,
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        };
        assert_eq!(WinHttpGetDefaultProxyConfiguration(&mut def_proxy), TRUE);
        assert_eq!(def_proxy.access_type, WINHTTP_ACCESS_TYPE_NO_PROXY);

        let mut ie_cfg = WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
            auto_detect: TRUE,
            auto_config_url: std::ptr::null_mut(),
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        };
        assert_eq!(WinHttpGetIEProxyConfigForCurrentUser(&mut ie_cfg), TRUE);
        assert_eq!(ie_cfg.auto_detect, FALSE);

        let url = wide_z("https://example.com/");
        let opts = WINHTTP_AUTOPROXY_OPTIONS {
            dw_flags: 1,
            dw_auto_detect_flags: 3,
            lpsz_auto_config_url: std::ptr::null(),
            lpv_reserved: std::ptr::null_mut(),
            dw_reserved: 0,
            f_auto_logon_if_challenged: FALSE,
        };
        assert_eq!(WinHttpGetProxyForUrl(session, url.as_ptr(), &opts, &mut def_proxy), FALSE);
        assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_AUTODETECTION_FAILED);
        WinHttpCloseHandle(session);
    }

    // WinINet InternetOpenUrlW roundtrip
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let (req_line, _, _) = read_http_request(&mut stream);
        assert_eq!(req_line, "GET /inet HTTP/1.1");
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 7\r\nConnection: close\r\n\r\nwininet")
            .unwrap();
    });

    unsafe {
        let inet = InternetOpenA(b"WinINetTest\0".as_ptr(), 1, std::ptr::null(), std::ptr::null(), 0);
        assert_ne!(inet, 0);
        let url = wide_z(&format!("http://127.0.0.1:{port}/inet"));
        let h = InternetOpenUrlW(inet, url.as_ptr(), std::ptr::null(), 0, 0, 0);
        assert_ne!(h, 0);
        let mut status: u32 = 0;
        let mut slen: u32 = 4;
        assert_eq!(
            HttpQueryInfoA(
                h,
                WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                &mut status as *mut u32 as *mut u8,
                &mut slen,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        assert_eq!(status, 200);
        let mut buf = [0u8; 32];
        let mut got: u32 = 0;
        assert_eq!(InternetReadFile(h, buf.as_mut_ptr(), 32, &mut got), TRUE);
        assert_eq!(&buf[..got as usize], b"wininet");
        assert_eq!(InternetCloseHandle(h), TRUE);
        assert_eq!(InternetCloseHandle(inet), TRUE);
    }

    server.join().unwrap();
}
