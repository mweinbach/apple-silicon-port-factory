use super::schannel::{
    SecBuffer, SecBufferDesc, SecHandle, SecPkgContextApplicationProtocol,
    SecPkgContextCipherInfo, SecPkgContextConnectionInfo, SecPkgContextIssuerListInfoEx,
    SecPkgContextNamesA, SecPkgContextNamesW, SecPkgContextSizes, SecPkgContextStreamSizes,
    SecPkgCredSupportedProtocols, ISC_REQ_ALLOCATE_MEMORY, ISC_REQ_CONFIDENTIALITY,
    ISC_REQ_REPLAY_DETECT, ISC_REQ_SEQUENCE_DETECT, ISC_REQ_STREAM, SCHANNEL_CRED_VERSION,
    SCHANNEL_SHUTDOWN, SCH_CREDENTIALS_VERSION, SCH_CRED_MANUAL_CRED_VALIDATION,
    SECBUFFER_APPLICATION_PROTOCOLS, SECBUFFER_DATA, SECBUFFER_EMPTY, SECBUFFER_EXTRA,
    SECBUFFER_MISSING, SECBUFFER_STREAM_HEADER, SECBUFFER_STREAM_TRAILER, SECBUFFER_TOKEN,
    SECPKGCONTEXT_CIPHERINFO_V1, SECPKG_ATTR_APPLICATION_PROTOCOL, SECPKG_ATTR_CIPHER_INFO,
    SECPKG_ATTR_CONNECTION_INFO, SECPKG_ATTR_ISSUER_LIST_EX, SECPKG_ATTR_NAMES,
    SECPKG_ATTR_REMOTE_CERT_CONTEXT, SECPKG_ATTR_SIZES, SECPKG_ATTR_STREAM_SIZES,
    SECPKG_CRED_ATTR_NAMES, SECPKG_CRED_ATTR_SUPPORTED_PROTOCOLS, SECPKG_CRED_OUTBOUND,
    SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION, SEC_E_BUFFER_TOO_SMALL, SEC_E_INCOMPLETE_MESSAGE,
    SEC_E_OK, SEC_E_UNTRUSTED_ROOT, SEC_I_CONTEXT_EXPIRED, SEC_I_CONTINUE_NEEDED,
    SP_PROT_TLS1_2_CLIENT, SP_PROT_TLS1_3_CLIENT, STREAM_HEADER_SIZE, STREAM_MAX_MESSAGE,
    STREAM_TRAILER_SIZE,
};
use super::*;
use crate::misc::crypt32;
use crate::misc::wintrust::CertContext;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{ServerConfig, ServerConnection};
use std::io::{Read, Write};
use std::sync::Arc;

// Deterministic Ed25519 Test CA + Leaf cert (CN=test.example.com, SAN=DNS:test.example.com, IP:127.0.0.1,
// valid 2020-01-01 to 2095-01-01) for offline loopback Schannel <-> rustls::ServerConnection tests.
const TEST_CA_DER: &[u8] = &[
    0x30, 0x82, 0x01, 0x2F, 0x30, 0x81, 0xE2, 0xA0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x02, 0x10, 0x00,
    0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70, 0x30, 0x1E, 0x31, 0x1C, 0x30, 0x1A, 0x06, 0x03, 0x55,
    0x04, 0x03, 0x0C, 0x13, 0x50, 0x6F, 0x72, 0x74, 0x46, 0x61, 0x63, 0x74, 0x6F, 0x72, 0x79, 0x20,
    0x54, 0x65, 0x73, 0x74, 0x20, 0x43, 0x41, 0x30, 0x20, 0x17, 0x0D, 0x32, 0x30, 0x30, 0x31, 0x30,
    0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x18, 0x0F, 0x32, 0x30, 0x39, 0x35, 0x30, 0x31,
    0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x30, 0x1E, 0x31, 0x1C, 0x30, 0x1A, 0x06,
    0x03, 0x55, 0x04, 0x03, 0x0C, 0x13, 0x50, 0x6F, 0x72, 0x74, 0x46, 0x61, 0x63, 0x74, 0x6F, 0x72,
    0x79, 0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x43, 0x41, 0x30, 0x2A, 0x30, 0x05, 0x06, 0x03, 0x2B,
    0x65, 0x70, 0x03, 0x21, 0x00, 0xC9, 0xDF, 0xC5, 0xF9, 0x98, 0x74, 0x8D, 0xAC, 0x1F, 0xD1, 0x1A,
    0x1E, 0xEB, 0x8B, 0x3D, 0x7C, 0xB7, 0x3E, 0x2D, 0x94, 0xCC, 0x7B, 0x36, 0xD6, 0xC5, 0x3F, 0x5E,
    0xD3, 0x6D, 0x62, 0x5B, 0x03, 0xA3, 0x42, 0x30, 0x40, 0x30, 0x0F, 0x06, 0x03, 0x55, 0x1D, 0x13,
    0x01, 0x01, 0xFF, 0x04, 0x05, 0x30, 0x03, 0x01, 0x01, 0xFF, 0x30, 0x0E, 0x06, 0x03, 0x55, 0x1D,
    0x0F, 0x01, 0x01, 0xFF, 0x04, 0x04, 0x03, 0x02, 0x01, 0x06, 0x30, 0x1D, 0x06, 0x03, 0x55, 0x1D,
    0x0E, 0x04, 0x16, 0x04, 0x14, 0xDE, 0x3C, 0xDD, 0x00, 0xB6, 0xC3, 0xE0, 0x57, 0x91, 0xA7, 0xBA,
    0x51, 0x58, 0xFA, 0xD7, 0xEC, 0x4B, 0x22, 0x85, 0x9C, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70,
    0x03, 0x41, 0x00, 0x30, 0x82, 0x52, 0xB8, 0x65, 0x95, 0x78, 0x8F, 0x16, 0xCC, 0x90, 0x9B, 0x53,
    0xAD, 0x3F, 0x06, 0xA0, 0xB8, 0xD3, 0xC3, 0xF1, 0x88, 0xBC, 0xAB, 0xC1, 0xE1, 0x7B, 0xC3, 0x49,
    0x45, 0xBD, 0x18, 0x87, 0x68, 0x69, 0x53, 0x74, 0x9A, 0xB1, 0xA1, 0xF3, 0x3E, 0xBC, 0x1E, 0x29,
    0xFA, 0x49, 0x6D, 0x5E, 0xCD, 0x19, 0x92, 0xB6, 0x4E, 0xE5, 0xEE, 0x68, 0x9C, 0xB4, 0x5F, 0x9E,
    0x0F, 0x5B, 0x04,
];

const TEST_LEAF_DER: &[u8] = &[
    0x30, 0x82, 0x01, 0x82, 0x30, 0x82, 0x01, 0x34, 0xA0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x02, 0x10,
    0x01, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70, 0x30, 0x1E, 0x31, 0x1C, 0x30, 0x1A, 0x06, 0x03,
    0x55, 0x04, 0x03, 0x0C, 0x13, 0x50, 0x6F, 0x72, 0x74, 0x46, 0x61, 0x63, 0x74, 0x6F, 0x72, 0x79,
    0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x43, 0x41, 0x30, 0x20, 0x17, 0x0D, 0x32, 0x30, 0x30, 0x31,
    0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x18, 0x0F, 0x32, 0x30, 0x39, 0x35, 0x30,
    0x31, 0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x30, 0x1B, 0x31, 0x19, 0x30, 0x17,
    0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x10, 0x74, 0x65, 0x73, 0x74, 0x2E, 0x65, 0x78, 0x61, 0x6D,
    0x70, 0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x30, 0x2A, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70,
    0x03, 0x21, 0x00, 0x64, 0x12, 0x69, 0x0A, 0x76, 0x96, 0x67, 0x9F, 0x8D, 0x7E, 0x53, 0xCE, 0x59,
    0xA0, 0x95, 0x62, 0x95, 0xED, 0xFE, 0xBF, 0xBA, 0xD1, 0xB8, 0xCE, 0x98, 0x10, 0x3C, 0xE7, 0xFE,
    0x3D, 0x15, 0x04, 0xA3, 0x81, 0x96, 0x30, 0x81, 0x93, 0x30, 0x09, 0x06, 0x03, 0x55, 0x1D, 0x13,
    0x04, 0x02, 0x30, 0x00, 0x30, 0x0E, 0x06, 0x03, 0x55, 0x1D, 0x0F, 0x01, 0x01, 0xFF, 0x04, 0x04,
    0x03, 0x02, 0x07, 0x80, 0x30, 0x13, 0x06, 0x03, 0x55, 0x1D, 0x25, 0x04, 0x0C, 0x30, 0x0A, 0x06,
    0x08, 0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x01, 0x30, 0x21, 0x06, 0x03, 0x55, 0x1D, 0x11,
    0x04, 0x1A, 0x30, 0x18, 0x82, 0x10, 0x74, 0x65, 0x73, 0x74, 0x2E, 0x65, 0x78, 0x61, 0x6D, 0x70,
    0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x87, 0x04, 0x7F, 0x00, 0x00, 0x01, 0x30, 0x1D, 0x06, 0x03,
    0x55, 0x1D, 0x0E, 0x04, 0x16, 0x04, 0x14, 0x13, 0xAF, 0xD3, 0x84, 0xCA, 0xC2, 0x1E, 0xC9, 0x60,
    0x11, 0xE3, 0x10, 0x7F, 0x67, 0x39, 0x27, 0xB5, 0xCF, 0x48, 0x10, 0x30, 0x1F, 0x06, 0x03, 0x55,
    0x1D, 0x23, 0x04, 0x18, 0x30, 0x16, 0x80, 0x14, 0xDE, 0x3C, 0xDD, 0x00, 0xB6, 0xC3, 0xE0, 0x57,
    0x91, 0xA7, 0xBA, 0x51, 0x58, 0xFA, 0xD7, 0xEC, 0x4B, 0x22, 0x85, 0x9C, 0x30, 0x05, 0x06, 0x03,
    0x2B, 0x65, 0x70, 0x03, 0x41, 0x00, 0x6E, 0x85, 0x65, 0xC4, 0x03, 0x91, 0xBC, 0x68, 0x8A, 0x98,
    0xFD, 0xC1, 0xC0, 0x3F, 0x33, 0xA2, 0xEF, 0x34, 0x3F, 0x39, 0xB7, 0x15, 0x6B, 0xFF, 0xD3, 0xEB,
    0xC9, 0xB4, 0xE1, 0x7F, 0xD7, 0x3D, 0x21, 0xC1, 0x27, 0x5C, 0xE9, 0xE2, 0xD5, 0x0C, 0xED, 0xAF,
    0x45, 0x39, 0x22, 0x06, 0x8C, 0x32, 0x4C, 0x68, 0xFD, 0xFB, 0xF5, 0x62, 0xA9, 0x6F, 0x73, 0x38,
    0xBA, 0x01, 0x47, 0x14, 0x74, 0x03,
];

const TEST_LEAF_KEY_PKCS8_DER: &[u8] = &[
    0x30, 0x2E, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70, 0x04, 0x22, 0x04, 0x20,
    0xA6, 0xD8, 0x4E, 0xD4, 0x73, 0x80, 0x73, 0x52, 0xB6, 0x85, 0x95, 0x39, 0xA7, 0x4E, 0x3C, 0x72,
    0x3B, 0x17, 0xDC, 0xB7, 0x50, 0x23, 0xA5, 0x51, 0x8A, 0x09, 0x95, 0x99, 0xEB, 0xE0, 0x91, 0x9A,
];

fn make_test_server(
    versions: &[&'static rustls::SupportedProtocolVersion],
    alpn: Vec<Vec<u8>>,
) -> ServerConnection {
    let chain = vec![CertificateDer::from(TEST_LEAF_DER.to_vec())];
    let key = PrivateKeyDer::Pkcs8(TEST_LEAF_KEY_PKCS8_DER.to_vec().into());
    let mut cfg = ServerConfig::builder_with_provider(crate::net_tls::crypto_provider())
        .with_protocol_versions(versions)
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(chain, key)
        .unwrap();
    cfg.alpn_protocols = alpn;
    ServerConnection::new(Arc::new(cfg)).unwrap()
}

fn server_step(srv: &mut ServerConnection, incoming: &[u8]) -> Vec<u8> {
    let mut slice = incoming;
    while !slice.is_empty() {
        let n = srv.read_tls(&mut slice).unwrap();
        if n == 0 {
            break;
        }
        srv.process_new_packets().unwrap();
    }
    let mut out = Vec::new();
    while srv.wants_write() {
        if srv.write_tls(&mut out).unwrap() == 0 {
            break;
        }
    }
    out
}

unsafe fn make_ca_memory_store() -> *mut c_void {
    let store = crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
    assert!(!store.is_null());
    assert_eq!(
        crypt32::CertAddEncodedCertificateToStore(
            store,
            1,
            TEST_CA_DER.as_ptr(),
            TEST_CA_DER.len() as u32,
            4,
            std::ptr::null_mut(),
        ),
        1
    );
    store
}

#[test]
fn init_security_interface_returns_valid_function_tables() {
    unsafe {
        let ta = InitSecurityInterfaceA() as *const SecurityFunctionTable;
        let tw = InitSecurityInterfaceW() as *const SecurityFunctionTable;
        assert!(!ta.is_null());
        assert!(!tw.is_null());
        assert_eq!((*ta).dw_version, SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION);
        assert_eq!((*tw).dw_version, SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION);
        for slot in [1, 2, 3, 5, 7, 8, 9, 10, 15, 16, 24, 25] {
            assert_ne!((*ta).slots[slot], 0, "A slot {slot} empty");
            assert_ne!((*tw).slots[slot], 0, "W slot {slot} empty");
        }
    }
}

#[test]
fn schannel_full_tls13_handshake_alpn_data_cert_chain_spki_and_shutdown() {
    unsafe {
        let ca_store = make_ca_memory_store();

        // Build SCHANNEL_CRED (v4) with SCH_CRED_MANUAL_CRED_VALIDATION + hRootStore = ca_store
        let mut cred_cfg = [0u8; 0x50];
        std::ptr::write_unaligned(cred_cfg.as_mut_ptr() as *mut u32, SCHANNEL_CRED_VERSION);
        std::ptr::write_unaligned(
            cred_cfg.as_mut_ptr().add(0x10) as *mut *mut c_void,
            ca_store,
        );
        std::ptr::write_unaligned(
            cred_cfg.as_mut_ptr().add(0x48) as *mut u32,
            SCH_CRED_MANUAL_CRED_VALIDATION,
        );

        let pkg = b"Microsoft Unified Security Protocol Provider\0";
        let mut cred = SecHandle::default();
        let mut expiry = 0i64;
        assert_eq!(
            AcquireCredentialsHandleA(
                std::ptr::null(),
                pkg.as_ptr(),
                SECPKG_CRED_OUTBOUND,
                std::ptr::null_mut(),
                cred_cfg.as_mut_ptr() as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cred as *mut SecHandle as *mut c_void,
                &mut expiry,
            ),
            SEC_E_OK
        );

        // Prepare ALPN input buffer: ["h2", "http/1.1"]
        let mut alpn_payload = Vec::new();
        let proto_list: &[u8] = &[2, b'h', b'2', 8, b'h', b't', b't', b'p', b'/', b'1', b'.', b'1'];
        let lists_size = (4 + 2 + proto_list.len()) as u32;
        alpn_payload.extend_from_slice(&lists_size.to_le_bytes());
        alpn_payload.extend_from_slice(&2u32.to_le_bytes()); // ALPN ext
        alpn_payload.extend_from_slice(&(proto_list.len() as u16).to_le_bytes());
        alpn_payload.extend_from_slice(proto_list);

        let mut alpn_in = SecBuffer {
            cb_buffer: alpn_payload.len() as u32,
            buffer_type: SECBUFFER_APPLICATION_PROTOCOLS,
            pv_buffer: alpn_payload.as_mut_ptr() as *mut c_void,
        };
        let mut alpn_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut alpn_in,
        };

        let mut out_buf = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        let mut out_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut out_buf,
        };

        let target = b"test.example.com\0";
        let mut ctx = SecHandle::default();
        let mut attrs = 0u32;
        let req = ISC_REQ_SEQUENCE_DETECT
            | ISC_REQ_REPLAY_DETECT
            | ISC_REQ_CONFIDENTIALITY
            | ISC_REQ_ALLOCATE_MEMORY
            | ISC_REQ_STREAM;

        let st = InitializeSecurityContextA(
            &mut cred as *mut SecHandle as *mut c_void,
            std::ptr::null_mut(),
            target.as_ptr(),
            req,
            0,
            0,
            &mut alpn_desc as *mut SecBufferDesc as *mut c_void,
            0,
            &mut ctx as *mut SecHandle as *mut c_void,
            &mut out_desc as *mut SecBufferDesc as *mut c_void,
            &mut attrs,
            &mut expiry,
        );
        assert_eq!(st, SEC_I_CONTINUE_NEEDED);
        assert!(!out_buf.pv_buffer.is_null() && out_buf.cb_buffer > 0);

        let mut srv = make_test_server(&[&rustls::version::TLS13], vec![b"h2".to_vec()]);
        let ch = std::slice::from_raw_parts(
            out_buf.pv_buffer as *const u8,
            out_buf.cb_buffer as usize,
        )
        .to_vec();
        assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);

        // Server responds to ClientHello; intentionally feed fragmented first to exercise SEC_E_INCOMPLETE_MESSAGE!
        let srv_flight = server_step(&mut srv, &ch);
        assert!(srv_flight.len() > 20);

        let mut partial = srv_flight[..7].to_vec();
        let mut in_bufs = [
            SecBuffer {
                cb_buffer: partial.len() as u32,
                buffer_type: SECBUFFER_TOKEN,
                pv_buffer: partial.as_mut_ptr() as *mut c_void,
            },
            SecBuffer {
                cb_buffer: 0,
                buffer_type: SECBUFFER_EMPTY,
                pv_buffer: std::ptr::null_mut(),
            },
        ];
        let mut in_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 2,
            p_buffers: in_bufs.as_mut_ptr(),
        };
        out_buf = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        let st_inc = InitializeSecurityContextA(
            &mut cred as *mut SecHandle as *mut c_void,
            &mut ctx as *mut SecHandle as *mut c_void,
            target.as_ptr(),
            req,
            0,
            0,
            &mut in_desc as *mut SecBufferDesc as *mut c_void,
            0,
            &mut ctx as *mut SecHandle as *mut c_void,
            &mut out_desc as *mut SecBufferDesc as *mut c_void,
            &mut attrs,
            &mut expiry,
        );
        assert_eq!(st_inc, SEC_E_INCOMPLETE_MESSAGE);
        assert_eq!(in_bufs[1].buffer_type, SECBUFFER_MISSING);
        assert!(in_bufs[1].cb_buffer > 0);

        // Feed full server flight + 5 trailing dummy bytes to verify SECBUFFER_EXTRA preservation
        let mut full_in = srv_flight.clone();
        full_in.extend_from_slice(&[0x17, 0x03, 0x03, 0x00, 0x01]);
        in_bufs[0] = SecBuffer {
            cb_buffer: full_in.len() as u32,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: full_in.as_mut_ptr() as *mut c_void,
        };
        in_bufs[1] = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_EMPTY,
            pv_buffer: std::ptr::null_mut(),
        };
        let st2 = InitializeSecurityContextA(
            &mut cred as *mut SecHandle as *mut c_void,
            &mut ctx as *mut SecHandle as *mut c_void,
            target.as_ptr(),
            req,
            0,
            0,
            &mut in_desc as *mut SecBufferDesc as *mut c_void,
            0,
            &mut ctx as *mut SecHandle as *mut c_void,
            &mut out_desc as *mut SecBufferDesc as *mut c_void,
            &mut attrs,
            &mut expiry,
        );
        assert_eq!(st2, SEC_E_OK);
        assert_eq!(in_bufs[1].buffer_type, SECBUFFER_EXTRA);
        assert_eq!(in_bufs[1].cb_buffer, 5);

        // Send Client Finished to server
        let cfin = std::slice::from_raw_parts(
            out_buf.pv_buffer as *const u8,
            out_buf.cb_buffer as usize,
        )
        .to_vec();
        assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);
        let post_hs = server_step(&mut srv, &cfin);
        assert!(!srv.is_handshaking());

        // QueryContextAttributes: STREAM_SIZES, CONNECTION_INFO, APPLICATION_PROTOCOL, REMOTE_CERT_CONTEXT
        let mut sizes = std::mem::zeroed::<SecPkgContextStreamSizes>();
        assert_eq!(
            QueryContextAttributesA(
                &mut ctx as *mut SecHandle as *mut c_void,
                SECPKG_ATTR_STREAM_SIZES,
                &mut sizes as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert_eq!(sizes.cb_header, STREAM_HEADER_SIZE);
        assert_eq!(sizes.cb_trailer, STREAM_TRAILER_SIZE);

        let mut cinfo = std::mem::zeroed::<SecPkgContextConnectionInfo>();
        assert_eq!(
            QueryContextAttributesA(
                &mut ctx as *mut SecHandle as *mut c_void,
                SECPKG_ATTR_CONNECTION_INFO,
                &mut cinfo as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert_eq!(cinfo.dw_protocol, SP_PROT_TLS1_3_CLIENT);

        let mut alpn_out = std::mem::zeroed::<SecPkgContextApplicationProtocol>();
        assert_eq!(
            QueryContextAttributesA(
                &mut ctx as *mut SecHandle as *mut c_void,
                SECPKG_ATTR_APPLICATION_PROTOCOL,
                &mut alpn_out as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert_eq!(alpn_out.proto_nego_status, 1);
        assert_eq!(
            &alpn_out.protocol_id[..alpn_out.protocol_id_size as usize],
            b"h2"
        );

        // Peer cert context + CertGetCertificateChain + CertVerifyCertificateChainPolicy + CryptEncodeObjectEx(SPKI)
        let mut cert_ptr: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            QueryContextAttributesA(
                &mut ctx as *mut SecHandle as *mut c_void,
                SECPKG_ATTR_REMOTE_CERT_CONTEXT,
                &mut cert_ptr as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert!(!cert_ptr.is_null());

        let mut chain_ptr: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crypt32::CertGetCertificateChain(
                std::ptr::null_mut(),
                cert_ptr,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                &mut chain_ptr,
            ),
            1
        );
        assert!(!chain_ptr.is_null());

        // Verify SSL policy passes for matching hostname "test.example.com" (CA was in hRootStore)
        let host_ok: Vec<u16> = "test.example.com\0".encode_utf16().collect();
        #[repr(C)]
        struct SslPara {
            cb: u32,
            auth: u32,
            checks: u32,
            host: *const u16,
        }
        #[repr(C)]
        struct PolPara {
            cb: u32,
            flags: u32,
            extra: *const c_void,
        }
        #[repr(C)]
        struct PolStatus {
            cb: u32,
            err: u32,
            ci: i32,
            ei: i32,
            extra: *mut c_void,
        }

        let ssl_ok = SslPara {
            cb: 24,
            auth: 2,
            checks: 0,
            host: host_ok.as_ptr(),
        };
        let para_ok = PolPara {
            cb: 16,
            flags: 0,
            extra: &ssl_ok as *const _ as *const c_void,
        };
        let mut status_ok = PolStatus {
            cb: 24,
            err: 99,
            ci: 0,
            ei: 0,
            extra: std::ptr::null_mut(),
        };
        assert_eq!(
            crypt32::CertVerifyCertificateChainPolicy(
                4 as *const u8,
                chain_ptr,
                &para_ok as *const _ as *const c_void,
                &mut status_ok as *mut _ as *mut c_void,
            ),
            1
        );
        assert_eq!(status_ok.err, 0);

        // Verify SSL policy FAILS with CERT_E_CN_NO_MATCH for wrong hostname
        let host_bad: Vec<u16> = "evil.example.com\0".encode_utf16().collect();
        let ssl_bad = SslPara {
            cb: 24,
            auth: 2,
            checks: 0,
            host: host_bad.as_ptr(),
        };
        let para_bad = PolPara {
            cb: 16,
            flags: 0,
            extra: &ssl_bad as *const _ as *const c_void,
        };
        let mut status_bad = PolStatus {
            cb: 24,
            err: 0,
            ci: 0,
            ei: 0,
            extra: std::ptr::null_mut(),
        };
        assert_eq!(
            crypt32::CertVerifyCertificateChainPolicy(
                4 as *const u8,
                chain_ptr,
                &para_bad as *const _ as *const c_void,
                &mut status_bad as *mut _ as *mut c_void,
            ),
            1
        );
        assert_eq!(status_bad.err, crypt32::CERT_E_CN_NO_MATCH);

        // CryptEncodeObjectEx(X509_PUBLIC_KEY_INFO) matches leaf cert SPKI DER
        let cc = std::ptr::read_unaligned(cert_ptr as *const CertContext);
        let spki_ptr = &(*cc.p_cert_info).subject_public_key_info as *const _ as *const c_void;
        let mut spki_len = 0u32;
        assert_eq!(
            crypt32::CryptEncodeObjectEx(
                1,
                8 as *const u8,
                spki_ptr,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut spki_len
            ),
            1
        );
        assert_eq!(spki_len, 44); // Ed25519 SPKI is exactly 44 bytes DER
        let mut spki_der = vec![0u8; spki_len as usize];
        assert_eq!(
            crypt32::CryptEncodeObjectEx(
                1,
                8 as *const u8,
                spki_ptr,
                0,
                std::ptr::null(),
                spki_der.as_mut_ptr() as *mut c_void,
                &mut spki_len
            ),
            1
        );
        assert!(TEST_LEAF_DER
            .windows(spki_der.len())
            .any(|w| w == spki_der.as_slice()));

        crypt32::CertFreeCertificateChain(chain_ptr);
        assert_eq!(crypt32::CertFreeCertificateContext(cert_ptr), 1);

        // EncryptMessage -> Server decrypts exact application request
        let msg = b"GET /health HTTP/1.1\r\nHost: test.example.com\r\n\r\n";
        let mut wire = vec![0u8; sizes.cb_header as usize + msg.len() + sizes.cb_trailer as usize];
        let hdr_len = sizes.cb_header as usize;
        wire[hdr_len..hdr_len + msg.len()].copy_from_slice(msg);
        let mut enc_bufs = [
            SecBuffer {
                cb_buffer: sizes.cb_header,
                buffer_type: SECBUFFER_STREAM_HEADER,
                pv_buffer: wire.as_mut_ptr() as *mut c_void,
            },
            SecBuffer {
                cb_buffer: msg.len() as u32,
                buffer_type: SECBUFFER_DATA,
                pv_buffer: wire.as_mut_ptr().add(hdr_len) as *mut c_void,
            },
            SecBuffer {
                cb_buffer: sizes.cb_trailer,
                buffer_type: SECBUFFER_STREAM_TRAILER,
                pv_buffer: wire.as_mut_ptr().add(hdr_len + msg.len()) as *mut c_void,
            },
            SecBuffer {
                cb_buffer: 0,
                buffer_type: SECBUFFER_EMPTY,
                pv_buffer: std::ptr::null_mut(),
            },
        ];
        let mut enc_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 4,
            p_buffers: enc_bufs.as_mut_ptr(),
        };
        assert_eq!(
            EncryptMessage(
                &mut ctx as *mut SecHandle as *mut c_void,
                0,
                &mut enc_desc as *mut _ as *mut c_void,
                0
            ),
            SEC_E_OK
        );
        let total_enc = (enc_bufs[0].cb_buffer + enc_bufs[1].cb_buffer + enc_bufs[2].cb_buffer) as usize;
        let _ = server_step(&mut srv, &wire[..total_enc]);
        let mut srv_plain = vec![0u8; msg.len()];
        srv.reader().read_exact(&mut srv_plain).unwrap();
        assert_eq!(&srv_plain, msg);

        // Server sends response (+ TLS 1.3 post-handshake NewSessionTicket in post_hs!) + close_notify
        let resp = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
        srv.writer().write_all(resp).unwrap();
        srv.writer().flush().unwrap();
        srv.send_close_notify();
        let mut srv_wire = post_hs;
        while srv.wants_write() {
            if srv.write_tls(&mut srv_wire).unwrap() == 0 {
                break;
            }
        }

        // DecryptMessage handles post-handshake NewSessionTicket, extracts exact resp, then SEC_I_CONTEXT_EXPIRED
        let mut recv_buf = srv_wire;
        let mut got_plain = Vec::new();
        let mut saw_expired = false;
        for _ in 0..8 {
            let mut dec_bufs = [
                SecBuffer {
                    cb_buffer: recv_buf.len() as u32,
                    buffer_type: SECBUFFER_DATA,
                    pv_buffer: recv_buf.as_mut_ptr() as *mut c_void,
                },
                SecBuffer {
                    cb_buffer: 0,
                    buffer_type: SECBUFFER_EMPTY,
                    pv_buffer: std::ptr::null_mut(),
                },
                SecBuffer {
                    cb_buffer: 0,
                    buffer_type: SECBUFFER_EMPTY,
                    pv_buffer: std::ptr::null_mut(),
                },
                SecBuffer {
                    cb_buffer: 0,
                    buffer_type: SECBUFFER_EMPTY,
                    pv_buffer: std::ptr::null_mut(),
                },
            ];
            let mut dec_desc = SecBufferDesc {
                ul_version: 0,
                c_buffers: 4,
                p_buffers: dec_bufs.as_mut_ptr(),
            };
            let ds = DecryptMessage(
                &mut ctx as *mut SecHandle as *mut c_void,
                &mut dec_desc as *mut _ as *mut c_void,
                0,
                std::ptr::null_mut(),
            );
            if ds == SEC_I_CONTEXT_EXPIRED {
                saw_expired = true;
                break;
            }
            assert_eq!(ds, SEC_E_OK);
            for b in &dec_bufs {
                if b.buffer_type == SECBUFFER_DATA && b.cb_buffer > 0 {
                    got_plain.extend_from_slice(std::slice::from_raw_parts(
                        b.pv_buffer as *const u8,
                        b.cb_buffer as usize,
                    ));
                }
            }
            let next = dec_bufs
                .iter()
                .find(|b| b.buffer_type == SECBUFFER_EXTRA && b.cb_buffer > 0)
                .map(|b| {
                    std::slice::from_raw_parts(b.pv_buffer as *const u8, b.cb_buffer as usize)
                        .to_vec()
                })
                .unwrap_or_default();
            recv_buf = next;
            if recv_buf.is_empty() {
                break;
            }
        }
        assert_eq!(&got_plain, resp);
        assert!(saw_expired);

        // Client shutdown via ApplyControlToken(SCHANNEL_SHUTDOWN) + InitializeSecurityContextA
        let mut shut_tok = SCHANNEL_SHUTDOWN;
        let mut ctl_buf = SecBuffer {
            cb_buffer: 4,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: &mut shut_tok as *mut u32 as *mut c_void,
        };
        let mut ctl_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut ctl_buf,
        };
        assert_eq!(
            ApplyControlToken(
                &mut ctx as *mut SecHandle as *mut c_void,
                &mut ctl_desc as *mut _ as *mut c_void
            ),
            SEC_E_OK
        );
        out_buf = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        assert_eq!(
            InitializeSecurityContextA(
                &mut cred as *mut SecHandle as *mut c_void,
                &mut ctx as *mut SecHandle as *mut c_void,
                std::ptr::null(),
                req,
                0,
                0,
                std::ptr::null_mut(),
                0,
                &mut ctx as *mut SecHandle as *mut c_void,
                &mut out_desc as *mut SecBufferDesc as *mut c_void,
                &mut attrs,
                std::ptr::null_mut(),
            ),
            SEC_E_OK
        );
        assert!(!out_buf.pv_buffer.is_null() && out_buf.cb_buffer > 0);
        assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);

        assert_eq!(
            DeleteSecurityContext(&mut ctx as *mut SecHandle as *mut c_void),
            SEC_E_OK
        );
        assert_eq!(
            FreeCredentialsHandle(&mut cred as *mut SecHandle as *mut c_void),
            SEC_E_OK
        );
        assert_eq!(crypt32::CertCloseStore(ca_store, 0), 1);
    }
}

#[test]
fn auto_validation_rejects_untrusted_root_honestly() {
    unsafe {
        // Acquire default Schannel creds WITHOUT manual validation and WITHOUT test CA -> must reject!
        let pkg = b"Schannel\0";
        let mut cred = SecHandle::default();
        assert_eq!(
            AcquireCredentialsHandleA(
                std::ptr::null(),
                pkg.as_ptr(),
                SECPKG_CRED_OUTBOUND,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cred as *mut SecHandle as *mut c_void,
                std::ptr::null_mut(),
            ),
            SEC_E_OK
        );
        let mut out_buf = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        let mut out_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut out_buf,
        };
        let mut ctx = SecHandle::default();
        let target = b"test.example.com\0";
        let req = ISC_REQ_ALLOCATE_MEMORY | ISC_REQ_STREAM;

        assert_eq!(
            InitializeSecurityContextA(
                &mut cred as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                target.as_ptr(),
                req,
                0,
                0,
                std::ptr::null_mut(),
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut out_desc as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            SEC_I_CONTINUE_NEEDED
        );
        let ch = std::slice::from_raw_parts(
            out_buf.pv_buffer as *const u8,
            out_buf.cb_buffer as usize,
        )
        .to_vec();
        assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);

        let mut srv = make_test_server(&[&rustls::version::TLS12], vec![]);
        let mut srv_flight = server_step(&mut srv, &ch);
        let mut in_buf = SecBuffer {
            cb_buffer: srv_flight.len() as u32,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: srv_flight.as_mut_ptr() as *mut c_void,
        };
        let mut in_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut in_buf,
        };
        let mut out_buf2 = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        let mut out_desc2 = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut out_buf2,
        };
        let res = InitializeSecurityContextA(
            &mut cred as *mut _ as *mut c_void,
            &mut ctx as *mut _ as *mut c_void,
            target.as_ptr(),
            req,
            0,
            0,
            &mut in_desc as *mut _ as *mut c_void,
            0,
            &mut ctx as *mut _ as *mut c_void,
            &mut out_desc2 as *mut _ as *mut c_void,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(res, SEC_E_UNTRUSTED_ROOT);
        assert_eq!(
            DeleteSecurityContext(&mut ctx as *mut _ as *mut c_void),
            SEC_E_OK
        );
        assert_eq!(
            FreeCredentialsHandle(&mut cred as *mut _ as *mut c_void),
            SEC_E_OK
        );
    }
}

#[test]
fn sch_credentials_v5_tls12_and_custom_chain_engine_exclusive_root() {
    unsafe {
        // Build SCH_CREDENTIALS (v5) disabling TLS 1.3 via TLS_PARAMETERS.grbitDisabledProtocols
        // and enabling manual cert validation (verifying via CertCreateCertificateChainEngine!).
        let mut tls_params = [0u8; 0x28];
        std::ptr::write_unaligned(
            tls_params.as_mut_ptr().add(0x10) as *mut u32,
            SP_PROT_TLS1_3_CLIENT,
        );

        let mut cred_v5 = [0u8; 0x48];
        std::ptr::write_unaligned(cred_v5.as_mut_ptr() as *mut u32, SCH_CREDENTIALS_VERSION);
        std::ptr::write_unaligned(
            cred_v5.as_mut_ptr().add(0x34) as *mut u32,
            SCH_CRED_MANUAL_CRED_VALIDATION,
        );
        std::ptr::write_unaligned(cred_v5.as_mut_ptr().add(0x38) as *mut u32, 1u32);
        std::ptr::write_unaligned(
            cred_v5.as_mut_ptr().add(0x40) as *mut *const u8,
            tls_params.as_ptr(),
        );

        let pkg_w: Vec<u16> = "Microsoft Unified Security Protocol Provider\0"
            .encode_utf16()
            .collect();
        let mut cred = SecHandle::default();
        assert_eq!(
            AcquireCredentialsHandleW(
                std::ptr::null(),
                pkg_w.as_ptr(),
                SECPKG_CRED_OUTBOUND,
                std::ptr::null_mut(),
                cred_v5.as_mut_ptr() as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cred as *mut _ as *mut c_void,
                std::ptr::null_mut(),
            ),
            SEC_E_OK
        );

        let target_w: Vec<u16> = "127.0.0.1\0".encode_utf16().collect();
        let mut ctx = SecHandle::default();
        let mut srv = make_test_server(&[&rustls::version::TLS12], vec![]);
        let req = ISC_REQ_ALLOCATE_MEMORY | ISC_REQ_STREAM;

        let mut out_buf = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        let mut out_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut out_buf,
        };
        assert_eq!(
            InitializeSecurityContextW(
                &mut cred as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                target_w.as_ptr(),
                req,
                0,
                0,
                std::ptr::null_mut(),
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut out_desc as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            SEC_I_CONTINUE_NEEDED
        );

        // Drive TLS 1.2 2-RTT handshake to completion
        let mut to_server = std::slice::from_raw_parts(
            out_buf.pv_buffer as *const u8,
            out_buf.cb_buffer as usize,
        )
        .to_vec();
        assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);
        let mut done = false;
        for _ in 0..6 {
            let mut from_server = server_step(&mut srv, &to_server);
            if from_server.is_empty() && !srv.is_handshaking() {
                break;
            }
            let mut in_buf = SecBuffer {
                cb_buffer: from_server.len() as u32,
                buffer_type: SECBUFFER_TOKEN,
                pv_buffer: from_server.as_mut_ptr() as *mut c_void,
            };
            let mut in_desc = SecBufferDesc {
                ul_version: 0,
                c_buffers: 1,
                p_buffers: &mut in_buf,
            };
            let mut ob = SecBuffer {
                cb_buffer: 0,
                buffer_type: SECBUFFER_TOKEN,
                pv_buffer: std::ptr::null_mut(),
            };
            let mut od = SecBufferDesc {
                ul_version: 0,
                c_buffers: 1,
                p_buffers: &mut ob,
            };
            let st = InitializeSecurityContextW(
                &mut cred as *mut _ as *mut c_void,
                &mut ctx as *mut _ as *mut c_void,
                target_w.as_ptr(),
                req,
                0,
                0,
                &mut in_desc as *mut _ as *mut c_void,
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut od as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            to_server = if !ob.pv_buffer.is_null() && ob.cb_buffer > 0 {
                let v = std::slice::from_raw_parts(ob.pv_buffer as *const u8, ob.cb_buffer as usize)
                    .to_vec();
                assert_eq!(FreeContextBuffer(ob.pv_buffer), SEC_E_OK);
                v
            } else {
                Vec::new()
            };
            if st == SEC_E_OK {
                done = true;
                break;
            }
            assert_eq!(st, SEC_I_CONTINUE_NEEDED);
        }
        assert!(done, "TLS 1.2 handshake did not complete");

        let mut cinfo = std::mem::zeroed::<SecPkgContextConnectionInfo>();
        assert_eq!(
            QueryContextAttributesW(
                &mut ctx as *mut _ as *mut c_void,
                SECPKG_ATTR_CONNECTION_INFO,
                &mut cinfo as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert_eq!(cinfo.dw_protocol, SP_PROT_TLS1_2_CLIENT);

        // Verify peer cert via custom CertCreateCertificateChainEngine(hExclusiveRoot = ca_store)
        let mut cert_ptr: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            QueryContextAttributesW(
                &mut ctx as *mut _ as *mut c_void,
                SECPKG_ATTR_REMOTE_CERT_CONTEXT,
                &mut cert_ptr as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        let ca_store = make_ca_memory_store();
        let mut eng_cfg = [0u8; 0x58];
        std::ptr::write_unaligned(eng_cfg.as_mut_ptr() as *mut u32, 0x58u32);
        std::ptr::write_unaligned(
            eng_cfg.as_mut_ptr().add(0x40) as *mut *mut c_void,
            ca_store,
        );
        let mut engine: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crypt32::CertCreateCertificateChainEngine(
                eng_cfg.as_ptr() as *const c_void,
                &mut engine
            ),
            1
        );

        let mut chain_ptr: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crypt32::CertGetCertificateChain(
                engine,
                cert_ptr,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                &mut chain_ptr,
            ),
            1
        );

        #[repr(C)]
        struct SslPara {
            cb: u32,
            auth: u32,
            checks: u32,
            host: *const u16,
        }
        #[repr(C)]
        struct PolPara {
            cb: u32,
            flags: u32,
            extra: *const c_void,
        }
        #[repr(C)]
        struct PolStatus {
            cb: u32,
            err: u32,
            ci: i32,
            ei: i32,
            extra: *mut c_void,
        }
        let ssl = SslPara {
            cb: 24,
            auth: 2,
            checks: 0,
            host: target_w.as_ptr(),
        };
        let para = PolPara {
            cb: 16,
            flags: 0,
            extra: &ssl as *const _ as *const c_void,
        };
        let mut status = PolStatus {
            cb: 24,
            err: 99,
            ci: 0,
            ei: 0,
            extra: std::ptr::null_mut(),
        };
        assert_eq!(
            crypt32::CertVerifyCertificateChainPolicy(
                4 as *const u8,
                chain_ptr,
                &para as *const _ as *const c_void,
                &mut status as *mut _ as *mut c_void,
            ),
            1
        );
        assert_eq!(status.err, 0);

        crypt32::CertFreeCertificateChain(chain_ptr);
        crypt32::CertFreeCertificateChainEngine(engine);
        assert_eq!(crypt32::CertFreeCertificateContext(cert_ptr), 1);
        assert_eq!(crypt32::CertCloseStore(ca_store, 0), 1);
        assert_eq!(
            DeleteSecurityContext(&mut ctx as *mut _ as *mut c_void),
            SEC_E_OK
        );
        assert_eq!(
            FreeCredentialsHandle(&mut cred as *mut _ as *mut c_void),
            SEC_E_OK
        );
    }
}

#[test]
fn query_context_cipher_info_names_creds_and_shutdown_close_notify() {
    assert_eq!(std::mem::size_of::<SecPkgContextCipherInfo>(), 0x2A8);
    assert_eq!(
        std::mem::offset_of!(SecPkgContextCipherInfo, sz_cipher_suite),
        0x10
    );
    assert_eq!(
        std::mem::offset_of!(SecPkgContextCipherInfo, sz_cipher),
        0x90
    );
    assert_eq!(
        std::mem::offset_of!(SecPkgContextCipherInfo, dw_cipher_len),
        0x110
    );
    assert_eq!(
        std::mem::offset_of!(SecPkgContextCipherInfo, sz_hash),
        0x118
    );
    assert_eq!(
        std::mem::offset_of!(SecPkgContextCipherInfo, sz_exchange),
        0x19C
    );
    assert_eq!(
        std::mem::offset_of!(SecPkgContextCipherInfo, sz_certificate),
        0x224
    );

    unsafe {
        let ca_store = make_ca_memory_store();
        let mut cred_cfg = [0u8; 0x50];
        std::ptr::write_unaligned(cred_cfg.as_mut_ptr() as *mut u32, SCHANNEL_CRED_VERSION);
        std::ptr::write_unaligned(
            cred_cfg.as_mut_ptr().add(0x10) as *mut *mut c_void,
            ca_store,
        );

        let principal = b"client@test.example.com\0";
        let pkg = b"Schannel\0";
        let mut cred = SecHandle::default();
        assert_eq!(
            AcquireCredentialsHandleA(
                principal.as_ptr(),
                pkg.as_ptr(),
                SECPKG_CRED_OUTBOUND,
                std::ptr::null_mut(),
                cred_cfg.as_mut_ptr() as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cred as *mut _ as *mut c_void,
                std::ptr::null_mut(),
            ),
            SEC_E_OK
        );

        // QueryCredentialsAttributesA & W (SECPKG_CRED_ATTR_NAMES + SUPPORTED_PROTOCOLS)
        let mut cred_names_a = SecPkgContextNamesA {
            s_user_name: std::ptr::null_mut(),
        };
        assert_eq!(
            QueryCredentialsAttributesA(
                &mut cred as *mut _ as *mut c_void,
                SECPKG_CRED_ATTR_NAMES,
                &mut cred_names_a as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert!(!cred_names_a.s_user_name.is_null());
        assert_eq!(
            crate::strings::from_ansi(cred_names_a.s_user_name),
            "client@test.example.com"
        );
        assert_eq!(
            FreeContextBuffer(cred_names_a.s_user_name as *mut c_void),
            SEC_E_OK
        );

        let mut cred_names_w = SecPkgContextNamesW {
            s_user_name: std::ptr::null_mut(),
        };
        assert_eq!(
            QueryCredentialsAttributesW(
                &mut cred as *mut _ as *mut c_void,
                SECPKG_CRED_ATTR_NAMES,
                &mut cred_names_w as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert!(!cred_names_w.s_user_name.is_null());
        assert_eq!(
            crate::strings::from_wide(cred_names_w.s_user_name),
            "client@test.example.com"
        );
        assert_eq!(
            FreeContextBuffer(cred_names_w.s_user_name as *mut c_void),
            SEC_E_OK
        );

        let mut sup_prot = SecPkgCredSupportedProtocols { grbit_protocol: 0 };
        assert_eq!(
            QueryCredentialsAttributesA(
                &mut cred as *mut _ as *mut c_void,
                SECPKG_CRED_ATTR_SUPPORTED_PROTOCOLS,
                &mut sup_prot as *mut _ as *mut c_void,
            ),
            SEC_E_OK
        );
        assert_eq!(
            sup_prot.grbit_protocol,
            SP_PROT_TLS1_2_CLIENT | SP_PROT_TLS1_3_CLIENT
        );

        // Complete a TLS 1.3 handshake
        let target = b"test.example.com\0";
        let mut ctx = SecHandle::default();
        let req = ISC_REQ_ALLOCATE_MEMORY | ISC_REQ_STREAM;
        let mut ob = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        let mut od = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut ob,
        };
        assert_eq!(
            InitializeSecurityContextA(
                &mut cred as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                target.as_ptr(),
                req,
                0,
                0,
                std::ptr::null_mut(),
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut od as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            SEC_I_CONTINUE_NEEDED
        );
        let ch = std::slice::from_raw_parts(ob.pv_buffer as *const u8, ob.cb_buffer as usize)
            .to_vec();
        assert_eq!(FreeContextBuffer(ob.pv_buffer), SEC_E_OK);

        let mut srv = make_test_server(&[&rustls::version::TLS13], vec![]);
        let mut sf = server_step(&mut srv, &ch);
        let mut ib = SecBuffer {
            cb_buffer: sf.len() as u32,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: sf.as_mut_ptr() as *mut c_void,
        };
        let mut id = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut ib,
        };
        ob = SecBuffer {
            cb_buffer: 0,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: std::ptr::null_mut(),
        };
        assert_eq!(
            InitializeSecurityContextA(
                &mut cred as *mut _ as *mut c_void,
                &mut ctx as *mut _ as *mut c_void,
                target.as_ptr(),
                req,
                0,
                0,
                &mut id as *mut _ as *mut c_void,
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut od as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            SEC_E_OK
        );
        let cfin = std::slice::from_raw_parts(ob.pv_buffer as *const u8, ob.cb_buffer as usize)
            .to_vec();
        assert_eq!(FreeContextBuffer(ob.pv_buffer), SEC_E_OK);
        let _ = server_step(&mut srv, &cfin);
        assert!(!srv.is_handshaking());

        // QueryContextAttributes: SIZES, STREAM_SIZES, CONNECTION_INFO, CIPHER_INFO, ISSUER_LIST_EX, NAMES (A & W)
        let mut ctx_sizes = std::mem::zeroed::<SecPkgContextSizes>();
        assert_eq!(
            QueryContextAttributesA(
                &mut ctx as *mut _ as *mut c_void,
                SECPKG_ATTR_SIZES,
                &mut ctx_sizes as *mut _ as *mut c_void
            ),
            SEC_E_OK
        );
        assert_eq!(ctx_sizes.cb_max_token, STREAM_MAX_MESSAGE);

        let mut ci = std::mem::zeroed::<SecPkgContextCipherInfo>();
        assert_eq!(
            QueryContextAttributesW(
                &mut ctx as *mut _ as *mut c_void,
                SECPKG_ATTR_CIPHER_INFO,
                &mut ci as *mut _ as *mut c_void
            ),
            SEC_E_OK
        );
        assert_eq!(ci.dw_version, SECPKGCONTEXT_CIPHERINFO_V1);
        assert_eq!(ci.dw_protocol, SP_PROT_TLS1_3_CLIENT);
        assert!(matches!(ci.dw_cipher_suite, 0x1301 | 0x1302 | 0x1303));
        let suite_str = crate::strings::from_wide(ci.sz_cipher_suite.as_ptr());
        assert!(suite_str.starts_with("TLS_"), "unexpected suite {suite_str}");
        assert!(ci.dw_cipher_len == 128 || ci.dw_cipher_len == 256);
        assert!(ci.dw_hash_len == 256 || ci.dw_hash_len == 384);

        let mut issuers = SecPkgContextIssuerListInfoEx {
            a_issuers: 1 as *mut c_void,
            c_issuers: 99,
        };
        assert_eq!(
            QueryContextAttributesA(
                &mut ctx as *mut _ as *mut c_void,
                SECPKG_ATTR_ISSUER_LIST_EX,
                &mut issuers as *mut _ as *mut c_void
            ),
            SEC_E_OK
        );
        assert_eq!(issuers.c_issuers, 0);
        assert!(issuers.a_issuers.is_null());

        let mut names_a = SecPkgContextNamesA {
            s_user_name: std::ptr::null_mut(),
        };
        assert_eq!(
            QueryContextAttributesA(
                &mut ctx as *mut _ as *mut c_void,
                SECPKG_ATTR_NAMES,
                &mut names_a as *mut _ as *mut c_void
            ),
            SEC_E_OK
        );
        assert_eq!(
            crate::strings::from_ansi(names_a.s_user_name),
            "test.example.com"
        );
        assert_eq!(
            FreeContextBuffer(names_a.s_user_name as *mut c_void),
            SEC_E_OK
        );

        let mut names_w = SecPkgContextNamesW {
            s_user_name: std::ptr::null_mut(),
        };
        assert_eq!(
            QueryContextAttributesW(
                &mut ctx as *mut _ as *mut c_void,
                SECPKG_ATTR_NAMES,
                &mut names_w as *mut _ as *mut c_void
            ),
            SEC_E_OK
        );
        assert_eq!(
            crate::strings::from_wide(names_w.s_user_name),
            "test.example.com"
        );
        assert_eq!(
            FreeContextBuffer(names_w.s_user_name as *mut c_void),
            SEC_E_OK
        );

        // ApplyControlToken(SCHANNEL_SHUTDOWN) -> retry after BUFFER_TOO_SMALL -> server observes genuine close_notify EOF!
        let mut shut_tok = SCHANNEL_SHUTDOWN;
        let mut ctl_buf = SecBuffer {
            cb_buffer: 4,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: &mut shut_tok as *mut u32 as *mut c_void,
        };
        let mut ctl_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut ctl_buf,
        };
        assert_eq!(
            ApplyControlToken(
                &mut ctx as *mut _ as *mut c_void,
                &mut ctl_desc as *mut _ as *mut c_void
            ),
            SEC_E_OK
        );

        // First try with tiny caller-supplied buffer -> SEC_E_BUFFER_TOO_SMALL, preserving close_notify bytes
        let mut tiny = [0u8; 2];
        let mut tiny_buf = SecBuffer {
            cb_buffer: 2,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: tiny.as_mut_ptr() as *mut c_void,
        };
        let mut tiny_desc = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut tiny_buf,
        };
        assert_eq!(
            InitializeSecurityContextA(
                &mut cred as *mut _ as *mut c_void,
                &mut ctx as *mut _ as *mut c_void,
                std::ptr::null(),
                ISC_REQ_STREAM,
                0,
                0,
                std::ptr::null_mut(),
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut tiny_desc as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            SEC_E_BUFFER_TOO_SMALL
        );

        // Retry with adequate buffer -> emits close_notify TLS record; feed it to server and verify clean EOF!
        let mut alert_buf = [0u8; 128];
        let mut ab = SecBuffer {
            cb_buffer: alert_buf.len() as u32,
            buffer_type: SECBUFFER_TOKEN,
            pv_buffer: alert_buf.as_mut_ptr() as *mut c_void,
        };
        let mut ad = SecBufferDesc {
            ul_version: 0,
            c_buffers: 1,
            p_buffers: &mut ab,
        };
        assert_eq!(
            InitializeSecurityContextA(
                &mut cred as *mut _ as *mut c_void,
                &mut ctx as *mut _ as *mut c_void,
                std::ptr::null(),
                ISC_REQ_STREAM,
                0,
                0,
                std::ptr::null_mut(),
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut ad as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            SEC_E_OK
        );
        assert!(ab.cb_buffer > 5);
        let mut wire = &alert_buf[..ab.cb_buffer as usize];
        srv.read_tls(&mut wire).unwrap();
        let io_st = srv.process_new_packets().unwrap();
        assert!(
            io_st.peer_has_closed(),
            "server did not receive TLS close_notify"
        );

        assert_eq!(
            DeleteSecurityContext(&mut ctx as *mut _ as *mut c_void),
            SEC_E_OK
        );
        assert_eq!(
            FreeCredentialsHandle(&mut cred as *mut _ as *mut c_void),
            SEC_E_OK
        );
        assert_eq!(crypt32::CertCloseStore(ca_store, 0), 1);
    }
}
