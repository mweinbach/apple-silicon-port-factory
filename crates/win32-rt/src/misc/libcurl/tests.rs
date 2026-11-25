use super::easy::{
    CURLE_COULDNT_CONNECT, CURLE_OK, CURLE_TOO_MANY_REDIRECTS, CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
    CURLINFO_CONTENT_TYPE, CURLINFO_EFFECTIVE_URL, CURLINFO_PRIVATE, CURLINFO_REDIRECT_COUNT,
    CURLINFO_RESPONSE_CODE, CURLINFO_SIZE_DOWNLOAD_T, CURLINFO_TOTAL_TIME, CURLOPT_COPYPOSTFIELDS,
    CURLOPT_CUSTOMREQUEST, CURLOPT_ERRORBUFFER, CURLOPT_FOLLOWLOCATION, CURLOPT_HEADERDATA,
    CURLOPT_HEADERFUNCTION, CURLOPT_HTTPHEADER, CURLOPT_MAXREDIRS, CURLOPT_NOBODY,
    CURLOPT_POSTFIELDS, CURLOPT_POSTFIELDSIZE, CURLOPT_PRIVATE, CURLOPT_READDATA,
    CURLOPT_READFUNCTION, CURLOPT_URL, CURLOPT_USERAGENT, CURLOPT_WRITEDATA, CURLOPT_WRITEFUNCTION,
    CURL_ERROR_SIZE, CURL_VERSION_SSL,
};
use super::multi::{CURLM_ADDED_ALREADY, CURLM_OK, CURLMSG_DONE};
use super::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

unsafe extern "win64" fn vec_write_cb(
    ptr: *const u8,
    size: usize,
    nmemb: usize,
    userdata: usize,
) -> usize {
    let total = size.saturating_mul(nmemb);
    if total > 0 && !ptr.is_null() && userdata != 0 {
        let dst = &mut *(userdata as *mut Vec<u8>);
        dst.extend_from_slice(std::slice::from_raw_parts(ptr, total));
    }
    total
}

unsafe extern "win64" fn vec_header_cb(
    ptr: *const u8,
    size: usize,
    nmemb: usize,
    userdata: usize,
) -> usize {
    let total = size.saturating_mul(nmemb);
    if total > 0 && !ptr.is_null() && userdata != 0 {
        let dst = &mut *(userdata as *mut Vec<String>);
        let line = String::from_utf8_lossy(std::slice::from_raw_parts(ptr, total)).into_owned();
        dst.push(line);
    }
    total
}

struct ReadSource {
    data: Vec<u8>,
    pos: usize,
}

unsafe extern "win64" fn slice_read_cb(
    buf: *mut u8,
    size: usize,
    nitems: usize,
    userdata: usize,
) -> usize {
    let cap = size.saturating_mul(nitems);
    if cap == 0 || buf.is_null() || userdata == 0 {
        return 0;
    }
    let src = &mut *(userdata as *mut ReadSource);
    let rem = src.data.len().saturating_sub(src.pos);
    let n = rem.min(cap).min(4); // Intentionally stream in small chunks
    if n > 0 {
        std::ptr::copy_nonoverlapping(src.data.as_ptr().add(src.pos), buf, n);
        src.pos += n;
    }
    n
}

fn read_http_request(stream: &mut TcpStream) -> (String, Vec<u8>) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let mut buf = Vec::new();
    let mut tmp = [0u8; 1024];
    let header_end = loop {
        let n = stream.read(&mut tmp).unwrap_or(0);
        if n == 0 {
            break buf.len();
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break pos + 4;
        }
    };
    let head = String::from_utf8_lossy(&buf[..header_end]).into_owned();
    let content_len = head
        .lines()
        .find_map(|l| {
            let (k, v) = l.split_once(':')?;
            if k.trim().eq_ignore_ascii_case("content-length") {
                v.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    while buf.len() < header_end + content_len {
        let n = stream.read(&mut tmp).unwrap_or(0);
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
    }
    let body = buf[header_end..buf.len().min(header_end + content_len)].to_vec();
    (head, body)
}

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, val: &str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, val);
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => std::env::set_var(self.key, v),
            None => std::env::remove_var(self.key),
        }
    }
}

#[test]
fn easy_get_headers_and_getinfo() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        let (mut sock, _) = listener.accept().unwrap();
        let (req_head, _) = read_http_request(&mut sock);
        assert!(req_head.starts_with("GET /hello?x=1 HTTP/1.1\r\n"));
        assert!(req_head.contains("User-Agent: TestAgent/1.0\r\n"));
        let resp = concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Type: text/plain; charset=utf-8\r\n",
            "Content-Length: 13\r\n",
            "X-Reply: ok\r\n",
            "\r\n",
            "Hello, World!"
        );
        sock.write_all(resp.as_bytes()).unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        assert_ne!(easy, 0);

        let url = format!("http://{addr}/hello?x=1\0");
        let ua = b"TestAgent/1.0\0";
        let mut body: Vec<u8> = Vec::new();
        let mut headers: Vec<String> = Vec::new();
        let priv_cookie: usize = 0x1234_5678_9ABC_DEF0;

        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_USERAGENT, ua.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEDATA,
                (&mut body as *mut Vec<u8>) as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_HEADERFUNCTION,
                vec_header_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_HEADERDATA,
                (&mut headers as *mut Vec<String>) as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_PRIVATE, priv_cookie),
            CURLE_OK
        );

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(body, b"Hello, World!");
        assert!(headers.iter().any(|h| h.starts_with("HTTP/1.1 200 OK")));
        assert!(headers.iter().any(|h| h.starts_with("X-Reply: ok")));

        let mut code: i32 = 0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_RESPONSE_CODE,
                &mut code as *mut i32 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(code, 200);

        let mut cl: i64 = 0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
                &mut cl as *mut i64 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(cl, 13);

        let mut dl: i64 = 0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_SIZE_DOWNLOAD_T,
                &mut dl as *mut i64 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(dl, 13);

        let mut total_time: f64 = -1.0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_TOTAL_TIME,
                &mut total_time as *mut f64 as *mut c_void
            ),
            CURLE_OK
        );
        assert!(total_time >= 0.0);

        let mut eff_url: *const u8 = std::ptr::null();
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_EFFECTIVE_URL,
                &mut eff_url as *mut *const u8 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(
            crate::strings::from_ansi(eff_url),
            format!("http://{addr}/hello?x=1")
        );

        let mut ct: *const u8 = std::ptr::null();
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_CONTENT_TYPE,
                &mut ct as *mut *const u8 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(crate::strings::from_ansi(ct), "text/plain; charset=utf-8");

        let mut got_priv: usize = 0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_PRIVATE,
                &mut got_priv as *mut usize as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(got_priv, priv_cookie);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_post_with_slist_headers_and_copypostfields() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        let (mut sock, _) = listener.accept().unwrap();
        let (req_head, req_body) = read_http_request(&mut sock);
        assert!(req_head.starts_with("POST /submit HTTP/1.1\r\n"));
        assert!(req_head.contains("Content-Type: application/json\r\n"));
        assert!(req_head.contains("X-Token: secret-42\r\n"));
        assert!(!req_head.to_ascii_lowercase().contains("accept:"));
        assert_eq!(req_body, br#"{"ok":true}"#);

        let resp = "HTTP/1.1 201 Created\r\nContent-Length: 7\r\n\r\ncreated";
        sock.write_all(resp.as_bytes()).unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let url = format!("http://{addr}/submit\0");
        let mut payload = br#"{"ok":true}"#.to_vec();

        let mut slist = curl_slist_append(
            std::ptr::null_mut(),
            b"Content-Type: application/json\0".as_ptr(),
        );
        slist = curl_slist_append(slist, b"X-Token: secret-42\0".as_ptr());
        slist = curl_slist_append(slist, b"Accept:\0".as_ptr()); // suppress default Accept

        let mut body: Vec<u8> = Vec::new();
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_HTTPHEADER, slist as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_POSTFIELDSIZE, payload.len()),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_COPYPOSTFIELDS, payload.as_ptr() as usize),
            CURLE_OK
        );
        // Mutate original buffer to prove COPYPOSTFIELDS copied it eagerly.
        payload.fill(b'X');

        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEDATA,
                (&mut body as *mut Vec<u8>) as usize
            ),
            CURLE_OK
        );

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(body, b"created");

        let mut code: i32 = 0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_RESPONSE_CODE,
                &mut code as *mut i32 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(code, 201);

        curl_slist_free_all(slist);
        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_chunked_transfer_decoding() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        let (mut sock, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut sock);
        let resp = concat!(
            "HTTP/1.1 200 OK\r\n",
            "Transfer-Encoding: chunked\r\n",
            "Content-Type: application/octet-stream\r\n",
            "\r\n",
            "5;ext=1\r\n",
            "Hello\r\n",
            "2\r\n",
            ", \r\n",
            "8\r\n",
            "chunked!\r\n",
            "0\r\n",
            "X-Trailer: done\r\n",
            "\r\n"
        );
        sock.write_all(resp.as_bytes()).unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let url = format!("http://{addr}/chunked\0");
        let mut body: Vec<u8> = Vec::new();
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEDATA,
                (&mut body as *mut Vec<u8>) as usize
            ),
            CURLE_OK
        );

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(body, b"Hello, chunked!");

        let mut cl: i64 = 0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
                &mut cl as *mut i64 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(cl, -1);

        let mut dl: i64 = 0;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_SIZE_DOWNLOAD_T,
                &mut dl as *mut i64 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(dl, 15);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_follow_302_redirect_and_maxredirs() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        // Connection 1: 302 redirect from /start -> /final
        let (mut s1, _) = listener.accept().unwrap();
        let (h1, _) = read_http_request(&mut s1);
        assert!(h1.starts_with("POST /start HTTP/1.1\r\n"));
        s1.write_all(b"HTTP/1.1 302 Found\r\nLocation: /final\r\nContent-Length: 0\r\n\r\n")
            .unwrap();
        drop(s1);

        // Connection 2: redirected GET /final
        let (mut s2, _) = listener.accept().unwrap();
        let (h2, b2) = read_http_request(&mut s2);
        assert!(h2.starts_with("GET /final HTTP/1.1\r\n"));
        assert!(b2.is_empty());
        s2.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nredirected")
            .unwrap();
        drop(s2);

        // Connection 3: test MAXREDIRS = 0
        let (mut s3, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s3);
        s3.write_all(b"HTTP/1.1 302 Found\r\nLocation: /final\r\nContent-Length: 0\r\n\r\n")
            .unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let url = format!("http://{addr}/start\0");
        let mut body: Vec<u8> = Vec::new();
        let post_data = b"a=1\0";

        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_POSTFIELDS, post_data.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_setopt(easy, CURLOPT_FOLLOWLOCATION, 1), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_MAXREDIRS, 5), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEDATA,
                (&mut body as *mut Vec<u8>) as usize
            ),
            CURLE_OK
        );

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(body, b"redirected");

        let mut code: i32 = 0;
        curl_easy_getinfo(
            easy,
            CURLINFO_RESPONSE_CODE,
            &mut code as *mut i32 as *mut c_void,
        );
        assert_eq!(code, 200);

        let mut redirs: i32 = 0;
        curl_easy_getinfo(
            easy,
            CURLINFO_REDIRECT_COUNT,
            &mut redirs as *mut i32 as *mut c_void,
        );
        assert_eq!(redirs, 1);

        let mut eff_url: *const u8 = std::ptr::null();
        curl_easy_getinfo(
            easy,
            CURLINFO_EFFECTIVE_URL,
            &mut eff_url as *mut *const u8 as *mut c_void,
        );
        assert_eq!(
            crate::strings::from_ansi(eff_url),
            format!("http://{addr}/final")
        );

        // Now test MAXREDIRS = 0 -> CURLE_TOO_MANY_REDIRECTS
        assert_eq!(curl_easy_setopt(easy, CURLOPT_MAXREDIRS, 0), CURLE_OK);
        assert_eq!(curl_easy_perform(easy), CURLE_TOO_MANY_REDIRECTS);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_put_with_readfunction_and_head() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        let (mut s1, _) = listener.accept().unwrap();
        let (h1, b1) = read_http_request(&mut s1);
        assert!(h1.starts_with("PUT /upload HTTP/1.1\r\n"));
        assert_eq!(b1, b"streamed-body");
        s1.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok")
            .unwrap();
        drop(s1);

        let (mut s2, _) = listener.accept().unwrap();
        let (h2, _) = read_http_request(&mut s2);
        assert!(h2.starts_with("HEAD /check HTTP/1.1\r\n"));
        s2.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 999\r\n\r\n")
            .unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let url = format!("http://{addr}/upload\0");
        let mut src = ReadSource {
            data: b"streamed-body".to_vec(),
            pos: 0,
        };
        let mut body: Vec<u8> = Vec::new();

        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_CUSTOMREQUEST, b"PUT\0".as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_READFUNCTION,
                slice_read_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_READDATA,
                (&mut src as *mut ReadSource) as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEDATA,
                (&mut body as *mut Vec<u8>) as usize
            ),
            CURLE_OK
        );
        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(body, b"ok");

        // Reset and do HEAD via CURLOPT_NOBODY
        curl_easy_reset(easy);
        body.clear();
        let url2 = format!("http://{addr}/check\0");
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_URL, url2.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_setopt(easy, CURLOPT_NOBODY, 1), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEDATA,
                (&mut body as *mut Vec<u8>) as usize
            ),
            CURLE_OK
        );
        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert!(body.is_empty());

        let mut cl: i64 = 0;
        curl_easy_getinfo(
            easy,
            CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
            &mut cl as *mut i64 as *mut c_void,
        );
        assert_eq!(cl, 999);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn multi_concurrent_perform_poll_and_info_read() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        for _ in 0..2 {
            let (mut sock, _) = listener.accept().unwrap();
            std::thread::spawn(move || {
                let (req_head, _) = read_http_request(&mut sock);
                let reply_body = if req_head.starts_with("GET /one ") {
                    "alpha"
                } else if req_head.starts_with("GET /two ") {
                    "beta-two"
                } else {
                    "unknown"
                };
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    reply_body.len(),
                    reply_body
                );
                let _ = sock.write_all(resp.as_bytes());
            });
        }
    });

    unsafe {
        let multi = curl_multi_init();
        assert_ne!(multi, 0);

        let e1 = curl_easy_init();
        let e2 = curl_easy_init();
        let url1 = format!("http://{addr}/one\0");
        let url2 = format!("http://{addr}/two\0");
        let mut b1: Vec<u8> = Vec::new();
        let mut b2: Vec<u8> = Vec::new();

        assert_eq!(
            curl_easy_setopt(e1, CURLOPT_URL, url1.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                e1,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(e1, CURLOPT_WRITEDATA, (&mut b1 as *mut Vec<u8>) as usize),
            CURLE_OK
        );

        assert_eq!(
            curl_easy_setopt(e2, CURLOPT_URL, url2.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                e2,
                CURLOPT_WRITEFUNCTION,
                vec_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(e2, CURLOPT_WRITEDATA, (&mut b2 as *mut Vec<u8>) as usize),
            CURLE_OK
        );

        assert_eq!(curl_multi_add_handle(multi, e1), CURLM_OK);
        assert_eq!(curl_multi_add_handle(multi, e2), CURLM_OK);
        // Adding the same handle twice must return CURLM_ADDED_ALREADY.
        assert_eq!(curl_multi_add_handle(multi, e1), CURLM_ADDED_ALREADY);

        let mut running: i32 = 1;
        let mut iters = 0;
        while running > 0 {
            assert_eq!(curl_multi_perform(multi, &mut running), CURLM_OK);
            if running > 0 {
                let mut numfds: i32 = 0;
                assert_eq!(
                    curl_multi_poll(multi, 0, 0, 2000, &mut numfds),
                    CURLM_OK
                );
            }
            iters += 1;
            assert!(iters < 200, "multi transfer did not complete");
        }

        let mut completed: Vec<(usize, i32)> = Vec::new();
        loop {
            let mut left: i32 = 0;
            let msg_ptr = curl_multi_info_read(multi, &mut left);
            if msg_ptr == 0 {
                assert_eq!(left, 0);
                break;
            }
            let msg = std::ptr::read_unaligned(msg_ptr as *const CurlMsg);
            assert_eq!(msg.msg, CURLMSG_DONE);
            completed.push((msg.easy_handle, msg.data as i32));
        }
        assert_eq!(completed.len(), 2);
        assert!(completed.contains(&(e1, CURLE_OK)));
        assert!(completed.contains(&(e2, CURLE_OK)));
        assert_eq!(b1, b"alpha");
        assert_eq!(b2, b"beta-two");

        assert_eq!(curl_multi_remove_handle(multi, e1), CURLM_OK);
        assert_eq!(curl_multi_remove_handle(multi, e2), CURLM_OK);
        curl_easy_cleanup(e1);
        curl_easy_cleanup(e2);
        assert_eq!(curl_multi_cleanup(multi), CURLM_OK);
    }

    server.join().unwrap();
}

#[test]
fn offline_mode_duphandle_escape_and_version() {
    unsafe {
        // PORT_OFFLINE=1 forces CURLE_COULDNT_CONNECT without touching the network.
        {
            let _guard = EnvGuard::set("PORT_OFFLINE", "1");
            let easy = curl_easy_init();
            let mut errbuf = [0u8; CURL_ERROR_SIZE];
            assert_eq!(
                curl_easy_setopt(
                    easy,
                    CURLOPT_URL,
                    b"http://127.0.0.1:1/test\0".as_ptr() as usize
                ),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_ERRORBUFFER, errbuf.as_mut_ptr() as usize),
                CURLE_OK
            );
            assert_eq!(curl_easy_perform(easy), CURLE_COULDNT_CONNECT);
            let err_str = crate::strings::from_ansi(errbuf.as_ptr());
            assert!(err_str.contains("PORT_OFFLINE"));

            // duphandle copies options cleanly
            let dup = curl_easy_duphandle(easy);
            assert_ne!(dup, 0);
            assert_ne!(dup, easy);
            assert_eq!(curl_easy_perform(dup), CURLE_COULDNT_CONNECT);
            curl_easy_cleanup(dup);
            curl_easy_cleanup(easy);
        }

        // Escape / unescape roundtrip + curl_free
        let raw = b"hello world/a+b=c\0";
        let esc = curl_easy_escape(0, raw.as_ptr(), 0);
        assert!(!esc.is_null());
        assert_eq!(
            crate::strings::from_ansi(esc),
            "hello%20world%2Fa%2Bb%3Dc"
        );

        let mut out_len: i32 = 0;
        let unesc = curl_easy_unescape(0, esc, 0, &mut out_len);
        assert!(!unesc.is_null());
        assert_eq!(out_len, 17);
        assert_eq!(crate::strings::from_ansi(unesc), "hello world/a+b=c");
        curl_free(esc as *mut c_void);
        curl_free(unesc as *mut c_void);

        // Version & version_info sanity
        let ver = crate::strings::from_ansi(curl_version());
        assert!(ver.contains("libcurl/"));
        assert!(ver.contains("rustls"));
        let vi = curl_version_info(10);
        assert!(!vi.is_null());
        let features = std::ptr::read_unaligned(std::ptr::addr_of!((*vi).features));
        let ver_ptr = std::ptr::read_unaligned(std::ptr::addr_of!((*vi).version));
        assert_ne!(features & CURL_VERSION_SSL, 0);
        assert_eq!(crate::strings::from_ansi(ver_ptr), "8.7.1");
    }
}

#[test]
fn easy_extended_setopt_and_getinfo_behaviors() {
    use super::easy::{
        CURLE_FILESIZE_EXCEEDED, CURLE_NOT_BUILT_IN, CURLE_UNKNOWN_OPTION,
        CURLE_UNSUPPORTED_PROTOCOL, CURLAUTH_BASIC, CURLAUTH_BEARER, CURLINFO_ACTIVESOCKET,
        CURLINFO_APPCONNECT_TIME_T, CURLINFO_CONDITION_UNMET, CURLINFO_EFFECTIVE_METHOD,
        CURLINFO_FILETIME_T, CURLINFO_HTTPAUTH_AVAIL, CURLINFO_LOCAL_IP, CURLINFO_LOCAL_PORT,
        CURLINFO_NAMELOOKUP_TIME_T, CURLINFO_PRETRANSFER_TIME_T, CURLINFO_PRIMARY_PORT,
        CURLINFO_PROTOCOL, CURLINFO_RETRY_AFTER, CURLINFO_SCHEME, CURLINFO_STARTTRANSFER_TIME_T,
        CURLPROTO_HTTP, CURLPROTO_HTTPS, CURLOPT_IPRESOLVE, CURLOPT_MAXFILESIZE,
        CURLOPT_NOPROGRESS, CURLOPT_PROTOCOLS, CURLOPT_REQUEST_TARGET, CURLOPT_RESOLVE,
        CURLOPT_RESUME_FROM, CURLOPT_SSH_AUTH_TYPES, CURLOPT_TCP_KEEPALIVE,
        CURLOPT_XFERINFODATA, CURLOPT_XFERINFOFUNCTION, CURLOPT_XOAUTH2_BEARER,
        CURL_IPRESOLVE_V4,
    };

    unsafe extern "win64" fn progress_cb(
        userdata: usize,
        _dltotal: i64,
        dlnow: i64,
        _ultotal: i64,
        _ulnow: i64,
    ) -> i32 {
        if userdata != 0 {
            let max_seen = &mut *(userdata as *mut i64);
            *max_seen = (*max_seen).max(dlnow);
        }
        0
    }

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        // Request 1: verify RESOLVE, XOAUTH2_BEARER, RESUME_FROM, REQUEST_TARGET, and extended CURLINFO
        let (mut s1, _) = listener.accept().unwrap();
        let (h1, _) = read_http_request(&mut s1);
        assert!(h1.starts_with("GET /override-target HTTP/1.1\r\n"));
        assert!(h1.contains("Authorization: Bearer tok-99\r\n"));
        assert!(h1.contains("Range: bytes=4-\r\n"));
        let resp1 = concat!(
            "HTTP/1.1 206 Partial Content\r\n",
            "Content-Range: bytes 4-13/14\r\n",
            "WWW-Authenticate: Basic realm=\"x\", Bearer\r\n",
            "Retry-After: 42\r\n",
            "Content-Length: 10\r\n",
            "\r\n",
            "0123456789"
        );
        s1.write_all(resp1.as_bytes()).unwrap();
        drop(s1);

        // Request 2: MAXFILESIZE exceeded via Content-Length
        let (mut s2, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s2);
        s2.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 500\r\n\r\n")
            .unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let resolve_entry = format!("override.local:{}:127.0.0.1\0", addr.port());
        let slist = curl_slist_append(std::ptr::null_mut(), resolve_entry.as_ptr());
        let url = format!("http://override.local:{}/ignored\0", addr.port());
        let mut body = Vec::<u8>::new();
        let mut max_progress = -1i64;

        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_RESOLVE, slist as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_IPRESOLVE, CURL_IPRESOLVE_V4 as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_TCP_KEEPALIVE, 1), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_XOAUTH2_BEARER, b"tok-99\0".as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_RESUME_FROM, 4), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_REQUEST_TARGET, b"/override-target\0".as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_NOPROGRESS, 0), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_XFERINFOFUNCTION, progress_cb as *const () as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_XFERINFODATA, (&mut max_progress as *mut i64) as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize), CURLE_OK);

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(body, b"0123456789");
        assert_eq!(max_progress, 10);

        // Verify extended CURLINFO_* fields
        let mut primary_port = 0i32;
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_PRIMARY_PORT, &mut primary_port as *mut i32 as *mut c_void), CURLE_OK);
        assert_eq!(primary_port, addr.port() as i32);

        let mut local_port = 0i32;
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_LOCAL_PORT, &mut local_port as *mut i32 as *mut c_void), CURLE_OK);
        assert!(local_port > 0);

        let mut local_ip: *const u8 = std::ptr::null();
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_LOCAL_IP, &mut local_ip as *mut *const u8 as *mut c_void), CURLE_OK);
        assert_eq!(crate::strings::from_ansi(local_ip), "127.0.0.1");

        let mut scheme: *const u8 = std::ptr::null();
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_SCHEME, &mut scheme as *mut *const u8 as *mut c_void), CURLE_OK);
        assert_eq!(crate::strings::from_ansi(scheme), "HTTP");

        let mut method: *const u8 = std::ptr::null();
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_EFFECTIVE_METHOD, &mut method as *mut *const u8 as *mut c_void), CURLE_OK);
        assert_eq!(crate::strings::from_ansi(method), "GET");

        let mut proto = 0i32;
        // Verify against the raw upstream ABI value (CURLINFO_LONG + 48 = 0x20_0030).
        assert_eq!(CURLINFO_PROTOCOL, 0x20_0030);
        assert_eq!(curl_easy_getinfo(easy, 0x20_0030, &mut proto as *mut i32 as *mut c_void), CURLE_OK);
        assert_eq!(proto, CURLPROTO_HTTP as i32);

        let mut auth_avail = 0i32;
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_HTTPAUTH_AVAIL, &mut auth_avail as *mut i32 as *mut c_void), CURLE_OK);
        assert_eq!(auth_avail, CURLAUTH_BASIC | CURLAUTH_BEARER);

        let mut retry_after = 0i64;
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_RETRY_AFTER, &mut retry_after as *mut i64 as *mut c_void), CURLE_OK);
        assert_eq!(retry_after, 42);

        let mut cond_unmet = 99i32;
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_CONDITION_UNMET, &mut cond_unmet as *mut i32 as *mut c_void), CURLE_OK);
        assert_eq!(cond_unmet, 0);

        let mut filetime = 0i64;
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_FILETIME_T, &mut filetime as *mut i64 as *mut c_void), CURLE_OK);
        assert_eq!(filetime, -1);

        let mut active_sock = 0usize;
        assert_eq!(curl_easy_getinfo(easy, CURLINFO_ACTIVESOCKET, &mut active_sock as *mut usize as *mut c_void), CURLE_OK);
        assert_eq!(active_sock, usize::MAX);

        for info in [
            CURLINFO_NAMELOOKUP_TIME_T,
            CURLINFO_PRETRANSFER_TIME_T,
            CURLINFO_STARTTRANSFER_TIME_T,
            CURLINFO_APPCONNECT_TIME_T,
        ] {
            let mut t_us = -1i64;
            assert_eq!(curl_easy_getinfo(easy, info, &mut t_us as *mut i64 as *mut c_void), CURLE_OK);
            assert!(t_us >= 0);
        }

        // MAXFILESIZE enforcement
        assert_eq!(curl_easy_setopt(easy, CURLOPT_MAXFILESIZE, 100), CURLE_OK);
        assert_eq!(curl_easy_perform(easy), CURLE_FILESIZE_EXCEEDED);

        // CURLOPT_PROTOCOLS restriction (allow HTTPS only -> HTTP URL rejected)
        assert_eq!(curl_easy_setopt(easy, CURLOPT_PROTOCOLS, CURLPROTO_HTTPS as usize), CURLE_OK);
        assert_eq!(curl_easy_perform(easy), CURLE_UNSUPPORTED_PROTOCOL);

        // Built-in disabled protocol option -> CURLE_NOT_BUILT_IN; unknown option/info -> CURLE_UNKNOWN_OPTION
        assert_eq!(curl_easy_setopt(easy, CURLOPT_SSH_AUTH_TYPES, 1), CURLE_NOT_BUILT_IN);
        assert_eq!(curl_easy_setopt(easy, 999_999, 0), CURLE_UNKNOWN_OPTION);
        let mut dummy = 0i64;
        assert_eq!(curl_easy_getinfo(easy, 0x20_9999, &mut dummy as *mut i64 as *mut c_void), CURLE_UNKNOWN_OPTION);

        curl_slist_free_all(slist);
        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_httpauth_mask_rejects_unsupported_and_gates_credentials() {
    use super::easy::{
        CURLE_NOT_BUILT_IN, CURLAUTH_BASIC, CURLAUTH_BEARER, CURLAUTH_DIGEST, CURLAUTH_NEGOTIATE,
        CURLAUTH_NTLM, CURLOPT_HTTPAUTH, CURLOPT_USERPWD, CURLOPT_XOAUTH2_BEARER,
    };

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        // 1. CURLAUTH_NONE: must NOT send Authorization even when USERPWD is set
        let (mut s1, _) = listener.accept().unwrap();
        let (h1, _) = read_http_request(&mut s1);
        assert!(
            !h1.to_ascii_lowercase().contains("authorization:"),
            "CURLAUTH_NONE must not send Authorization header: {h1}"
        );
        s1.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok").unwrap();
        drop(s1);

        // 2. CURLAUTH_BEARER only (no XOAUTH2_BEARER, USERPWD set): must NOT leak Basic
        let (mut s2, _) = listener.accept().unwrap();
        let (h2, _) = read_http_request(&mut s2);
        assert!(
            !h2.to_ascii_lowercase().contains("authorization:"),
            "CURLAUTH_BEARER without bearer token must not fall back to Basic: {h2}"
        );
        s2.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok").unwrap();
        drop(s2);

        // 3. CURLAUTH_BEARER with XOAUTH2_BEARER: sends Bearer (not Basic)
        let (mut s3, _) = listener.accept().unwrap();
        let (h3, _) = read_http_request(&mut s3);
        assert!(h3.contains("Authorization: Bearer secret-tok\r\n"));
        assert!(!h3.contains("Authorization: Basic"));
        s3.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok").unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let url = format!("http://{addr}/auth\0");
        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_USERPWD, b"alice:pw\0".as_ptr() as usize), CURLE_OK);

        // Unsupported Digest/NTLM/Negotiate-only modes must fail honestly with CURLE_NOT_BUILT_IN.
        assert_eq!(curl_easy_setopt(easy, CURLOPT_HTTPAUTH, CURLAUTH_DIGEST as usize), CURLE_NOT_BUILT_IN);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_HTTPAUTH, CURLAUTH_NTLM as usize), CURLE_NOT_BUILT_IN);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_HTTPAUTH, CURLAUTH_NEGOTIATE as usize), CURLE_NOT_BUILT_IN);
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_HTTPAUTH, (CURLAUTH_DIGEST | CURLAUTH_NTLM) as usize),
            CURLE_NOT_BUILT_IN
        );

        // Supported modes accepted.
        assert_eq!(curl_easy_setopt(easy, CURLOPT_HTTPAUTH, CURLAUTH_BASIC as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_HTTPAUTH, 0 /* CURLAUTH_NONE */), CURLE_OK);
        assert_eq!(curl_easy_perform(easy), CURLE_OK);

        // Bearer-only without token must not send Basic from USERPWD.
        assert_eq!(curl_easy_setopt(easy, CURLOPT_HTTPAUTH, CURLAUTH_BEARER as usize), CURLE_OK);
        assert_eq!(curl_easy_perform(easy), CURLE_OK);

        // Bearer-only with token sends Bearer.
        assert_eq!(curl_easy_setopt(easy, CURLOPT_XOAUTH2_BEARER, b"secret-tok\0".as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_perform(easy), CURLE_OK);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_redirect_enforces_overall_protocols_allowlist() {
    use super::easy::{
        CURLE_UNSUPPORTED_PROTOCOL, CURLPROTO_HTTP, CURLPROTO_HTTPS, CURLOPT_PROTOCOLS,
        CURLOPT_REDIR_PROTOCOLS,
    };

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        // 1. Redirect from http -> https (must be blocked when CURLOPT_PROTOCOLS = HTTP only,
        // even though default CURLOPT_REDIR_PROTOCOLS allows HTTPS).
        let (mut s1, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s1);
        let resp1 = format!(
            "HTTP/1.1 302 Found\r\nLocation: https://{addr}/tls-only\r\nContent-Length: 0\r\n\r\n"
        );
        s1.write_all(resp1.as_bytes()).unwrap();
        drop(s1);

        // 2. Redirect from http -> http (must be blocked when CURLOPT_REDIR_PROTOCOLS = HTTPS only,
        // even though CURLOPT_PROTOCOLS allows HTTP).
        let (mut s2, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s2);
        let resp2 = format!(
            "HTTP/1.1 302 Found\r\nLocation: http://{addr}/plain\r\nContent-Length: 0\r\n\r\n"
        );
        s2.write_all(resp2.as_bytes()).unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let url = format!("http://{addr}/start\0");
        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_FOLLOWLOCATION, 1), CURLE_OK);

        // Restrict overall protocols to HTTP only; default redir_protocols includes HTTPS.
        assert_eq!(curl_easy_setopt(easy, CURLOPT_PROTOCOLS, CURLPROTO_HTTP as usize), CURLE_OK);
        assert_eq!(curl_easy_perform(easy), CURLE_UNSUPPORTED_PROTOCOL);

        // Allow both HTTP and HTTPS overall, but restrict redirect protocols to HTTPS only.
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_PROTOCOLS, (CURLPROTO_HTTP | CURLPROTO_HTTPS) as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_REDIR_PROTOCOLS, CURLPROTO_HTTPS as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_perform(easy), CURLE_UNSUPPORTED_PROTOCOL);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_resume_from_validates_206_and_content_range() {
    use super::easy::{CURLE_RANGE_ERROR, CURLOPT_RESUME_FROM};

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        // 1. Server ignores Range and replies 200 OK -> must fail with CURLE_RANGE_ERROR and 0 body bytes.
        let (mut s1, _) = listener.accept().unwrap();
        let (h1, _) = read_http_request(&mut s1);
        assert!(h1.contains("Range: bytes=5-\r\n"), "expected Range header: {h1}");
        s1.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n0123456789")
            .unwrap();
        drop(s1);

        // 2. Server replies 206 with mismatched Content-Range start (bytes 0-9/10) -> CURLE_RANGE_ERROR.
        let (mut s2, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s2);
        s2.write_all(
            b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 0-9/10\r\nContent-Length: 10\r\n\r\n0123456789",
        )
        .unwrap();
        drop(s2);

        // 3. Server replies 206 with matching Content-Range: bytes 5-9/10 -> CURLE_OK + tail bytes.
        let (mut s3, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s3);
        s3.write_all(
            b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 5-9/10\r\nContent-Length: 5\r\n\r\n56789",
        )
        .unwrap();
    });

    unsafe {
        let easy = curl_easy_init();
        let url = format!("http://{addr}/file.bin\0");
        let mut sink = Box::new(Vec::<u8>::new());
        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_RESUME_FROM, 5), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_WRITEDATA, &mut *sink as *mut Vec<u8> as usize),
            CURLE_OK
        );

        assert_eq!(curl_easy_perform(easy), CURLE_RANGE_ERROR);
        assert!(sink.is_empty(), "200 OK on resume must not deliver body bytes");

        assert_eq!(curl_easy_perform(easy), CURLE_RANGE_ERROR);
        assert!(sink.is_empty(), "mismatched Content-Range must not deliver body bytes");

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(sink.as_slice(), b"56789");

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_resume_from_rejects_http_upload_and_post_before_io() {
    use super::easy::{
        CURLE_BAD_FUNCTION_ARGUMENT, CURLOPT_POST, CURLOPT_POSTFIELDS, CURLOPT_READDATA,
        CURLOPT_READFUNCTION, CURLOPT_RESUME_FROM, CURLOPT_UPLOAD,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    unsafe extern "win64" fn counting_read_cb(
        _ptr: *mut u8,
        _size: usize,
        _nmemb: usize,
        userdata: *mut c_void,
    ) -> usize {
        let counter = &*(userdata as *const AtomicUsize);
        counter.fetch_add(1, Ordering::SeqCst);
        0
    }

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let addr = listener.local_addr().unwrap();

    unsafe {
        crate::register_native_va(
            counting_read_cb as *const () as u64,
            counting_read_cb as *const () as usize,
        );
        let url = format!("http://{addr}/upload\0");
        let read_calls = AtomicUsize::new(0);

        // 1. CURLOPT_UPLOAD + positive CURLOPT_RESUME_FROM -> CURLE_BAD_FUNCTION_ARGUMENT
        //    before invoking READFUNCTION or connecting to the server.
        let easy_put = curl_easy_init();
        assert_eq!(curl_easy_setopt(easy_put, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy_put, CURLOPT_UPLOAD, 1), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy_put, CURLOPT_RESUME_FROM, 4), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy_put, CURLOPT_READFUNCTION, counting_read_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy_put, CURLOPT_READDATA, &read_calls as *const AtomicUsize as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_perform(easy_put), CURLE_BAD_FUNCTION_ARGUMENT);
        assert_eq!(
            read_calls.load(Ordering::SeqCst),
            0,
            "READFUNCTION must not be invoked when upload resume is rejected"
        );
        assert!(
            matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock),
            "rejected resumed PUT must not connect to the server"
        );
        curl_easy_cleanup(easy_put);

        // 2. CURLOPT_POST / CURLOPT_POSTFIELDS + positive CURLOPT_RESUME_FROM ->
        //    CURLE_BAD_FUNCTION_ARGUMENT before connecting to the server.
        let easy_post = curl_easy_init();
        assert_eq!(curl_easy_setopt(easy_post, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy_post, CURLOPT_POST, 1), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy_post, CURLOPT_POSTFIELDS, b"payload=1\0".as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_setopt(easy_post, CURLOPT_RESUME_FROM, 2), CURLE_OK);
        assert_eq!(curl_easy_perform(easy_post), CURLE_BAD_FUNCTION_ARGUMENT);
        assert!(
            matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock),
            "rejected resumed POST must not connect to the server"
        );
        curl_easy_cleanup(easy_post);
    }
}

#[test]
fn multi_delivers_xferinfo_progress_and_honors_abort() {
    use super::easy::{
        CURLE_ABORTED_BY_CALLBACK, CURLOPT_NOPROGRESS, CURLOPT_XFERINFODATA,
        CURLOPT_XFERINFOFUNCTION,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    unsafe extern "win64" fn progress_abort_cb(
        clientp: *mut c_void,
        _dltotal: i64,
        _dlnow: i64,
        _ultotal: i64,
        _ulnow: i64,
    ) -> i32 {
        let counter = &*(clientp as *const AtomicUsize);
        counter.fetch_add(1, Ordering::Relaxed);
        1 // non-zero aborts the transfer
    }

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s);
        let body = vec![b'x'; 4096];
        let hdr = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
        let _ = s.write_all(hdr.as_bytes());
        let _ = s.write_all(&body);
    });

    unsafe {
        crate::register_native_va(
            progress_abort_cb as *const () as u64,
            progress_abort_cb as *const () as usize,
        );

        let multi = curl_multi_init();
        let easy = curl_easy_init();
        let url = format!("http://{addr}/progress\0");
        let calls = Box::new(AtomicUsize::new(0));
        let mut sink = Box::new(Vec::<u8>::new());

        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_NOPROGRESS, 0), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_XFERINFOFUNCTION,
                progress_abort_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_XFERINFODATA, &*calls as *const AtomicUsize as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_WRITEDATA, &mut *sink as *mut Vec<u8> as usize),
            CURLE_OK
        );
        assert_eq!(curl_multi_add_handle(multi, easy), CURLM_OK);

        let mut running = 1i32;
        for _ in 0..200 {
            assert_eq!(curl_multi_perform(multi, &mut running), CURLM_OK);
            if running == 0 {
                break;
            }
            let mut numfds = 0i32;
            assert_eq!(curl_multi_poll(multi, 0, 0, 50, &mut numfds), CURLM_OK);
        }
        assert_eq!(running, 0);
        assert!(calls.load(Ordering::Relaxed) >= 1, "multi must invoke XFERINFOFUNCTION");

        let mut msgs_left = 0i32;
        let msg_ptr = curl_multi_info_read(multi, &mut msgs_left);
        assert_ne!(msg_ptr, 0);
        let msg = std::ptr::read_unaligned(msg_ptr as *const CurlMsg);
        assert_eq!(msg.easy_handle, easy);
        assert_eq!(msg.data as u32 as i32, CURLE_ABORTED_BY_CALLBACK);

        assert_eq!(curl_multi_remove_handle(multi, easy), CURLM_OK);
        curl_easy_cleanup(easy);
        assert_eq!(curl_multi_cleanup(multi), CURLM_OK);
    }

    server.join().unwrap();
}

#[test]
fn easy_write_callback_caps_chunks_at_curl_max_write_size() {
    use super::easy::CURLOPT_BUFFERSIZE;
    use super::http::CURL_MAX_WRITE_SIZE;

    struct ChunkTracker {
        body: Vec<u8>,
        max_chunk: usize,
        chunks: usize,
    }
    unsafe extern "win64" fn tracking_write_cb(
        ptr: *const u8,
        size: usize,
        nmemb: usize,
        userdata: *mut c_void,
    ) -> usize {
        let total = size.saturating_mul(nmemb);
        let tracker = &mut *(userdata as *mut ChunkTracker);
        tracker.max_chunk = tracker.max_chunk.max(total);
        tracker.chunks += 1;
        if total > 0 && !ptr.is_null() {
            tracker
                .body
                .extend_from_slice(std::slice::from_raw_parts(ptr, total));
        }
        total
    }

    let payload: Vec<u8> = (0..40 * 1024).map(|i| (i % 251) as u8).collect();
    let expected = payload.clone();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s);
        let hdr = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            payload.len()
        );
        s.write_all(hdr.as_bytes()).unwrap();
        s.write_all(&payload).unwrap();
    });

    unsafe {
        crate::register_native_va(
            tracking_write_cb as *const () as u64,
            tracking_write_cb as *const () as usize,
        );
        let easy = curl_easy_init();
        let url = format!("http://{addr}/big\0");
        let mut tracker = ChunkTracker {
            body: Vec::new(),
            max_chunk: 0,
            chunks: 0,
        };
        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        // Set a 64 KiB receive buffer (4x CURL_MAX_WRITE_SIZE); write callback chunks must still be <= 16 KiB.
        assert_eq!(curl_easy_setopt(easy, CURLOPT_BUFFERSIZE, 64 * 1024), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEFUNCTION,
                tracking_write_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_WRITEDATA,
                (&mut tracker as *mut ChunkTracker) as usize
            ),
            CURLE_OK
        );

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        assert_eq!(tracker.body, expected);
        assert!(
            tracker.max_chunk <= CURL_MAX_WRITE_SIZE,
            "write callback chunk {} exceeded CURL_MAX_WRITE_SIZE ({})",
            tracker.max_chunk,
            CURL_MAX_WRITE_SIZE
        );
        assert!(tracker.chunks >= 3, "40 KiB body must split into >= 3 chunks");

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_idle_network_io_invokes_progress_callback_and_aborts() {
    use super::easy::{
        CURLE_ABORTED_BY_CALLBACK, CURLOPT_NOPROGRESS, CURLOPT_XFERINFODATA,
        CURLOPT_XFERINFOFUNCTION,
    };
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    unsafe extern "win64" fn abort_after_two_calls(
        clientp: *mut c_void,
        _dltotal: i64,
        _dlnow: i64,
        _ultotal: i64,
        _ulnow: i64,
    ) -> i32 {
        let count = &*(clientp as *const AtomicUsize);
        let prev = count.fetch_add(1, Ordering::Relaxed);
        if prev >= 1 { 1 } else { 0 }
    }

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let done = Arc::new(AtomicBool::new(false));
    let done_srv = Arc::clone(&done);

    let server = std::thread::spawn(move || {
        // 1. Stall before sending response headers until client aborts.
        let (mut s1, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s1);
        while !done_srv.load(Ordering::Acquire) {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        drop(s1);

        // 2. Send headers (Content-Length: 1024) and then stall body until client aborts.
        let (mut s2, _) = listener.accept().unwrap();
        let _ = read_http_request(&mut s2);
        let _ = s2.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1024\r\n\r\n");
        let _ = s2.flush();
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_secs(3) {
            if s2.write(&[]).is_err() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    });

    unsafe {
        crate::register_native_va(
            abort_after_two_calls as *const () as u64,
            abort_after_two_calls as *const () as usize,
        );

        let easy = curl_easy_init();
        let url = format!("http://{addr}/stall\0");
        let calls = AtomicUsize::new(0);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy, CURLOPT_NOPROGRESS, 0), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_XFERINFOFUNCTION,
                abort_after_two_calls as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_XFERINFODATA,
                (&calls as *const AtomicUsize) as usize
            ),
            CURLE_OK
        );

        // 1. Stalled while waiting for headers -> idle progress fires and aborts with CURLE_ABORTED_BY_CALLBACK.
        assert_eq!(curl_easy_perform(easy), CURLE_ABORTED_BY_CALLBACK);
        assert!(calls.load(Ordering::Relaxed) >= 2);
        done.store(true, Ordering::Release);

        // 2. Stalled after headers during body read -> idle progress fires and aborts with CURLE_ABORTED_BY_CALLBACK.
        calls.store(0, Ordering::Relaxed);
        assert_eq!(curl_easy_perform(easy), CURLE_ABORTED_BY_CALLBACK);
        assert!(calls.load(Ordering::Relaxed) >= 2);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_gzip_progress_reports_transferred_bytes_not_decompressed() {
    use super::easy::{
        CURLOPT_ACCEPT_ENCODING, CURLOPT_NOPROGRESS, CURLOPT_XFERINFODATA,
        CURLOPT_XFERINFOFUNCTION,
    };
    use crate::misc::zlib1::ZlibStream;

    struct ProgressLog {
        samples: Vec<(i64, i64)>,
    }

    unsafe extern "win64" fn record_progress_cb(
        clientp: *mut c_void,
        dltotal: i64,
        dlnow: i64,
        _ultotal: i64,
        _ulnow: i64,
    ) -> i32 {
        let log = &mut *(clientp as *mut ProgressLog);
        log.samples.push((dltotal, dlnow));
        0
    }

    // 32 KiB highly compressible payload compresses to ~80 bytes of gzip.
    let plain = vec![b'Z'; 32 * 1024];
    let gzipped = ZlibStream::deflate_all(&plain, 6, 15 + 16).expect("gzip compress");
    assert!(
        gzipped.len() < 1024,
        "expected strong gzip compression, got {} bytes",
        gzipped.len()
    );
    let wire_len = gzipped.len() as i64;
    let gzipped_srv = gzipped.clone();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        let (mut sock, _) = listener.accept().unwrap();
        let (req_head, _) = read_http_request(&mut sock);
        assert!(
            req_head.to_ascii_lowercase().contains("accept-encoding:"),
            "expected Accept-Encoding header: {req_head}"
        );
        let hdr = format!(
            "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n",
            gzipped_srv.len()
        );
        sock.write_all(hdr.as_bytes()).unwrap();
        // Send compressed payload in two parts (body + final trailer) to exercise streaming progress.
        let split = gzipped_srv.len().saturating_sub(8);
        sock.write_all(&gzipped_srv[..split]).unwrap();
        sock.flush().unwrap();
        std::thread::sleep(Duration::from_millis(20));
        sock.write_all(&gzipped_srv[split..]).unwrap();
        sock.flush().unwrap();
    });

    unsafe {
        crate::register_native_va(
            record_progress_cb as *const () as u64,
            record_progress_cb as *const () as usize,
        );
        let easy = curl_easy_init();
        let url = format!("http://{addr}/compressed\0");
        let mut body: Vec<u8> = Vec::new();
        let mut plog = ProgressLog { samples: Vec::new() };

        assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_ACCEPT_ENCODING, b"\0".as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_setopt(easy, CURLOPT_NOPROGRESS, 0), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_XFERINFOFUNCTION,
                record_progress_cb as *const () as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(
                easy,
                CURLOPT_XFERINFODATA,
                (&mut plog as *mut ProgressLog) as usize
            ),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize),
            CURLE_OK
        );

        assert_eq!(curl_easy_perform(easy), CURLE_OK);
        // Write callback still receives the full 32 KiB decompressed body.
        assert_eq!(body.len(), plain.len());
        assert_eq!(body, plain);

        // Every progress callback must report transferred wire bytes (dlnow <= dltotal == wire_len).
        assert!(!plog.samples.is_empty());
        let mut prev_dlnow = 0i64;
        for &(dltotal, dlnow) in &plog.samples {
            assert_eq!(dltotal, wire_len, "dltotal must equal compressed Content-Length");
            assert!(
                dlnow >= prev_dlnow && dlnow <= dltotal,
                "dlnow ({dlnow}) must be monotonic and <= dltotal ({dltotal})"
            );
            prev_dlnow = dlnow;
        }
        assert_eq!(
            plog.samples.last().copied(),
            Some((wire_len, wire_len)),
            "final progress sample must reach dlnow == dltotal == wire_len"
        );

        let mut size_dl = 0i64;
        assert_eq!(
            curl_easy_getinfo(
                easy,
                CURLINFO_SIZE_DOWNLOAD_T,
                &mut size_dl as *mut i64 as *mut c_void
            ),
            CURLE_OK
        );
        assert_eq!(size_dl, wire_len);

        curl_easy_cleanup(easy);
    }

    server.join().unwrap();
}

#[test]
fn easy_gzip_large_buffer_streams_bounded_and_enforces_max_filesize() {
    use super::easy::{
        CURLE_FILESIZE_EXCEEDED, CURLE_WRITE_ERROR, CURLOPT_ACCEPT_ENCODING, CURLOPT_BUFFERSIZE,
        CURLOPT_MAXFILESIZE,
    };
    use super::http::CURL_MAX_WRITE_SIZE;
    use crate::misc::zlib1::ZlibStream;

    struct BoundedSink {
        total_bytes: usize,
        max_chunk: usize,
        abort_after_chunks: usize,
        chunks_seen: usize,
    }

    unsafe extern "win64" fn bounded_write_cb(
        _ptr: *const u8,
        size: usize,
        nmemb: usize,
        userdata: *mut c_void,
    ) -> usize {
        let total = size.saturating_mul(nmemb);
        let st = &mut *(userdata as *mut BoundedSink);
        st.chunks_seen += 1;
        st.max_chunk = st.max_chunk.max(total);
        st.total_bytes += total;
        if st.abort_after_chunks > 0 && st.chunks_seen >= st.abort_after_chunks {
            return 0; // Signal abort to verify streaming inflate stops immediately.
        }
        total
    }

    // 4 MiB highly compressible payload compresses to ~4 KiB (>1000x ratio).
    let plain = vec![b'A'; 4 * 1024 * 1024];
    let gzipped = ZlibStream::deflate_all(&plain, 6, 15 + 16).expect("gzip compress");
    assert!(
        gzipped.len() < 16 * 1024,
        "expected tiny gzip payload, got {} bytes",
        gzipped.len()
    );
    let gz_srv = gzipped.clone();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = std::thread::spawn(move || {
        for _ in 0..2 {
            let (mut sock, _) = listener.accept().unwrap();
            let _ = read_http_request(&mut sock);
            let hdr = format!(
                "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n",
                gz_srv.len()
            );
            let _ = sock.write_all(hdr.as_bytes());
            let _ = sock.write_all(&gz_srv);
            let _ = sock.flush();
        }
    });

    unsafe {
        crate::register_native_va(
            bounded_write_cb as *const () as u64,
            bounded_write_cb as *const () as usize,
        );
        let url = format!("http://{addr}/bomb.gz\0");
        let enc = b"gzip\0";

        // 1. With CURLOPT_BUFFERSIZE = 512 KiB and CURLOPT_MAXFILESIZE = 64 KiB,
        //    decoded output must stop before exceeding 64 KiB even though wire size is ~4 KiB.
        let easy1 = curl_easy_init();
        let mut st1 = BoundedSink {
            total_bytes: 0,
            max_chunk: 0,
            abort_after_chunks: 0,
            chunks_seen: 0,
        };
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_ACCEPT_ENCODING, enc.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_BUFFERSIZE, 512 * 1024), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_MAXFILESIZE, 64 * 1024), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy1, CURLOPT_WRITEFUNCTION, bounded_write_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy1, CURLOPT_WRITEDATA, &mut st1 as *mut BoundedSink as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_perform(easy1), CURLE_FILESIZE_EXCEEDED);
        assert!(
            st1.total_bytes <= 64 * 1024,
            "decoded output exceeded CURLOPT_MAXFILESIZE: {} bytes",
            st1.total_bytes
        );
        assert!(
            st1.max_chunk <= CURL_MAX_WRITE_SIZE,
            "decoded chunk exceeded CURL_MAX_WRITE_SIZE: {}",
            st1.max_chunk
        );
        curl_easy_cleanup(easy1);

        // 2. Without MAXFILESIZE, aborting on the 2nd 16 KiB write callback must halt
        //    streaming inflation immediately without decoding the rest of the 4 MiB payload.
        let easy2 = curl_easy_init();
        let mut st2 = BoundedSink {
            total_bytes: 0,
            max_chunk: 0,
            abort_after_chunks: 2,
            chunks_seen: 0,
        };
        assert_eq!(curl_easy_setopt(easy2, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy2, CURLOPT_ACCEPT_ENCODING, enc.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy2, CURLOPT_BUFFERSIZE, 512 * 1024), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy2, CURLOPT_WRITEFUNCTION, bounded_write_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy2, CURLOPT_WRITEDATA, &mut st2 as *mut BoundedSink as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_perform(easy2), CURLE_WRITE_ERROR);
        assert_eq!(st2.chunks_seen, 2);
        assert!(st2.total_bytes <= 2 * CURL_MAX_WRITE_SIZE);
        curl_easy_cleanup(easy2);
    }

    server.join().unwrap();
}

#[test]
fn easy_upload_dispatches_progress_and_aborts_on_stalled_receiver() {
    use super::easy::{
        CURLE_ABORTED_BY_CALLBACK, CURLOPT_COPYPOSTFIELDS, CURLOPT_NOPROGRESS, CURLOPT_POST,
        CURLOPT_POSTFIELDSIZE, CURLOPT_XFERINFODATA, CURLOPT_XFERINFOFUNCTION,
    };
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    struct UploadProg {
        samples: Vec<(i64, i64)>,
        abort_when_stalled: bool,
        stalled_repeats: usize,
    }

    unsafe extern "win64" fn upload_prog_cb(
        clientp: *mut c_void,
        _dltotal: i64,
        _dlnow: i64,
        ultotal: i64,
        ulnow: i64,
    ) -> i32 {
        let st = &mut *(clientp as *mut UploadProg);
        let prev = st.samples.last().copied();
        st.samples.push((ultotal, ulnow));
        if st.abort_when_stalled {
            if prev == Some((ultotal, ulnow)) && ulnow < ultotal {
                st.stalled_repeats += 1;
                if st.stalled_repeats >= 1 {
                    return 1; // Abort stalled upload!
                }
            }
        }
        0
    }

    // Part 1: Successful 256 KiB POST dispatches initial (total, 0), intermediate (0 < ulnow < total),
    // and final (total, total) progress samples monotonically.
    let body1 = vec![b'P'; 256 * 1024];
    let listener1 = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr1 = listener1.local_addr().unwrap();
    let srv1 = std::thread::spawn(move || {
        let (mut sock, _) = listener1.accept().unwrap();
        let (_h, b) = read_http_request(&mut sock);
        assert_eq!(b.len(), 256 * 1024);
        let _ = sock.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok");
    });

    // Part 2: Stalled receiver (never reads) with a 4 MiB POST and NO CURLOPT_TIMEOUT must still
    // invoke XFERINFOFUNCTION periodically on write timeouts and abort promptly.
    let body2 = vec![b'S'; 4 * 1024 * 1024];
    let listener2 = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr2 = listener2.local_addr().unwrap();
    let release_srv2 = Arc::new(AtomicBool::new(false));
    let release_srv2_bg = release_srv2.clone();
    let srv2 = std::thread::spawn(move || {
        let (_sock, _) = listener2.accept().unwrap();
        // Deliberately do not read from `_sock` so the client's send buffer fills and stalls.
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while !release_srv2_bg.load(Ordering::Acquire) && std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
    });

    unsafe {
        crate::register_native_va(
            upload_prog_cb as *const () as u64,
            upload_prog_cb as *const () as usize,
        );

        let easy1 = curl_easy_init();
        let url1 = format!("http://{addr1}/upload\0");
        let mut prog1 = UploadProg {
            samples: Vec::new(),
            abort_when_stalled: false,
            stalled_repeats: 0,
        };
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_URL, url1.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_POST, 1), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_POSTFIELDSIZE, body1.len()), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy1, CURLOPT_COPYPOSTFIELDS, body1.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_setopt(easy1, CURLOPT_NOPROGRESS, 0), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy1, CURLOPT_XFERINFOFUNCTION, upload_prog_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy1, CURLOPT_XFERINFODATA, &mut prog1 as *mut UploadProg as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_perform(easy1), CURLE_OK);
        let want_total = body1.len() as i64;
        assert_eq!(prog1.samples.first().copied(), Some((want_total, 0)));
        assert!(
            prog1.samples.iter().any(|&(t, n)| t == want_total && n > 0 && n < want_total),
            "expected intermediate upload progress samples, got {:?}",
            prog1.samples
        );
        assert_eq!(prog1.samples.last().copied(), Some((want_total, want_total)));
        curl_easy_cleanup(easy1);

        let easy2 = curl_easy_init();
        let url2 = format!("http://{addr2}/stalled\0");
        let mut prog2 = UploadProg {
            samples: Vec::new(),
            abort_when_stalled: true,
            stalled_repeats: 0,
        };
        assert_eq!(curl_easy_setopt(easy2, CURLOPT_URL, url2.as_ptr() as usize), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy2, CURLOPT_POST, 1), CURLE_OK);
        assert_eq!(curl_easy_setopt(easy2, CURLOPT_POSTFIELDSIZE, body2.len()), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy2, CURLOPT_COPYPOSTFIELDS, body2.as_ptr() as usize),
            CURLE_OK
        );
        assert_eq!(curl_easy_setopt(easy2, CURLOPT_NOPROGRESS, 0), CURLE_OK);
        assert_eq!(
            curl_easy_setopt(easy2, CURLOPT_XFERINFOFUNCTION, upload_prog_cb as *const () as usize),
            CURLE_OK
        );
        assert_eq!(
            curl_easy_setopt(easy2, CURLOPT_XFERINFODATA, &mut prog2 as *mut UploadProg as usize),
            CURLE_OK
        );
        let t0 = std::time::Instant::now();
        assert_eq!(curl_easy_perform(easy2), CURLE_ABORTED_BY_CALLBACK);
        assert!(
            t0.elapsed() < Duration::from_secs(3),
            "stalled upload must abort promptly via periodic progress callbacks, took {:?}",
            t0.elapsed()
        );
        assert!(prog2.stalled_repeats >= 1);
        curl_easy_cleanup(easy2);
    }

    release_srv2.store(true, Ordering::Release);
    srv1.join().unwrap();
    srv2.join().unwrap();
}
