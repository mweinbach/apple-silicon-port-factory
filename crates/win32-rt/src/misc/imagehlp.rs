//! imagehlp.dll: PE Authenticode `IMAGE_DIRECTORY_ENTRY_SECURITY` certificate
//! enumeration and reading (`ImageEnumerateCertificates`,
//! `ImageGetCertificateHeader`, `ImageGetCertificateData`).

use crate::types::*;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const CERT_SECTION_TYPE_ANY: u16 = 0x00FF;

#[derive(Clone, Copy)]
struct PeCertEntry {
    file_offset: usize,
    dw_length: u32,
    w_revision: u16,
    w_cert_type: u16,
}

unsafe fn pread_exact_at(fd: i32, off: u64, dst: &mut [u8]) -> bool {
    let mut done = 0usize;
    while done < dst.len() {
        let n = libc::pread(
            fd,
            dst.as_mut_ptr().add(done).cast(),
            dst.len() - done,
            (off + done as u64) as libc::off_t,
        );
        if n <= 0 {
            if n < 0 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            return false;
        }
        done += n as usize;
    }
    true
}

unsafe fn parse_pe_certificates(h: HANDLE) -> Result<(i32, Vec<PeCertEntry>), u32> {
    let Some(f) = crate::kernel32::file::get_file(h) else {
        return Err(err::ERROR_INVALID_HANDLE);
    };
    let fd = f.fd();
    let mut st: libc::stat = std::mem::zeroed();
    if libc::fstat(fd, &mut st) != 0 || st.st_size < 0 {
        return Err(err::ERROR_INVALID_HANDLE);
    }
    let file_len = st.st_size as usize;
    if file_len < 0x40 {
        return Ok((fd, Vec::new()));
    }
    let mut dos = [0u8; 0x40];
    if !pread_exact_at(fd, 0, &mut dos) || &dos[0..2] != b"MZ" {
        return Ok((fd, Vec::new()));
    }
    let pe_off = u32::from_le_bytes(dos[0x3C..0x40].try_into().unwrap()) as usize;
    if pe_off.saturating_add(24 + 120) > file_len {
        return Ok((fd, Vec::new()));
    }
    let mut hdr = [0u8; 24 + 160];
    let hdr_len = hdr.len().min(file_len - pe_off);
    if !pread_exact_at(fd, pe_off as u64, &mut hdr[..hdr_len]) || &hdr[0..4] != b"PE\0\0" {
        return Ok((fd, Vec::new()));
    }
    let opt = 24usize;
    let magic = u16::from_le_bytes(hdr[opt..opt + 2].try_into().unwrap());
    let (num_rva_off, dd_off) = match magic {
        0x10B => (opt + 92, opt + 96),
        0x20B => (opt + 108, opt + 112),
        _ => return Ok((fd, Vec::new())),
    };
    if dd_off + 5 * 8 > hdr_len {
        return Ok((fd, Vec::new()));
    }
    let num_rva = u32::from_le_bytes(hdr[num_rva_off..num_rva_off + 4].try_into().unwrap());
    if num_rva <= 4 {
        return Ok((fd, Vec::new()));
    }
    let sec_dir_off = dd_off + 4 * 8;
    let sec_off = u32::from_le_bytes(hdr[sec_dir_off..sec_dir_off + 4].try_into().unwrap()) as usize;
    let sec_sz = u32::from_le_bytes(hdr[sec_dir_off + 4..sec_dir_off + 8].try_into().unwrap()) as usize;
    if sec_off == 0 || sec_sz < 8 || sec_off.saturating_add(sec_sz) > file_len {
        return Ok((fd, Vec::new()));
    }
    let sec_end = sec_off + sec_sz;
    let mut cur = sec_off;
    let mut out = Vec::new();
    while cur + 8 <= sec_end && out.len() < 128 {
        let mut chdr = [0u8; 8];
        if !pread_exact_at(fd, cur as u64, &mut chdr) {
            break;
        }
        let dw_length = u32::from_le_bytes(chdr[0..4].try_into().unwrap());
        let w_revision = u16::from_le_bytes(chdr[4..6].try_into().unwrap());
        let w_cert_type = u16::from_le_bytes(chdr[6..8].try_into().unwrap());
        if (dw_length as usize) < 8 || cur.saturating_add(dw_length as usize) > sec_end {
            break;
        }
        out.push(PeCertEntry {
            file_offset: cur,
            dw_length,
            w_revision,
            w_cert_type,
        });
        let aligned = ((dw_length as usize) + 7) & !7;
        cur = match cur.checked_add(aligned) {
            Some(next) if next > cur => next,
            _ => break,
        };
    }
    Ok((fd, out))
}

pub unsafe extern "win64" fn ImageEnumerateCertificates(
    file_handle: HANDLE,
    type_filter: u16,
    cert_count: *mut u32,
    indices: *mut u32,
    index_count: u32,
) -> BOOL {
    if cert_count.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (_fd, certs) = match parse_pe_certificates(file_handle) {
        Ok(v) => v,
        Err(e) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    };
    let matching: Vec<u32> = certs
        .iter()
        .enumerate()
        .filter(|(_, c)| type_filter == CERT_SECTION_TYPE_ANY || c.w_cert_type == type_filter)
        .map(|(i, _)| i as u32)
        .collect();
    std::ptr::write_unaligned(cert_count, matching.len() as u32);
    if !indices.is_null() {
        if (index_count as usize) < matching.len() {
            crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        for (slot, &idx) in matching.iter().enumerate() {
            std::ptr::write_unaligned(indices.add(slot), idx);
        }
    }
    TRUE
}

pub unsafe extern "win64" fn ImageGetCertificateHeader(
    file_handle: HANDLE,
    cert_index: u32,
    cert_header: *mut u8,
) -> BOOL {
    if cert_header.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (_fd, certs) = match parse_pe_certificates(file_handle) {
        Ok(v) => v,
        Err(e) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    };
    let Some(ent) = certs.get(cert_index as usize) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    std::ptr::write_unaligned(cert_header as *mut u32, ent.dw_length);
    std::ptr::write_unaligned(cert_header.add(4) as *mut u16, ent.w_revision);
    std::ptr::write_unaligned(cert_header.add(6) as *mut u16, ent.w_cert_type);
    TRUE
}

pub unsafe extern "win64" fn ImageGetCertificateData(
    file_handle: HANDLE,
    cert_index: u32,
    certificate: *mut u8,
    required_length: *mut u32,
) -> BOOL {
    if required_length.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (fd, certs) = match parse_pe_certificates(file_handle) {
        Ok(v) => v,
        Err(e) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    };
    let Some(ent) = certs.get(cert_index as usize) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let avail = std::ptr::read_unaligned(required_length);
    std::ptr::write_unaligned(required_length, ent.dw_length);
    if certificate.is_null() || avail < ent.dw_length {
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    let mut buf = vec![0u8; ent.dw_length as usize];
    if !pread_exact_at(fd, ent.file_offset as u64, &mut buf) {
        crate::teb::set_last_error(30); // ERROR_READ_FAULT
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(buf.as_ptr(), certificate, buf.len());
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel32::file::CreateFileW;
    use crate::kernel32::threads::CloseHandle;
    use crate::strings::to_wide_z;

    const GENERIC_READ: u32 = 0x8000_0000;
    const OPEN_EXISTING: u32 = 3;

    #[test]
    fn imagehlp_reads_pe_security_directory_certificates() {
        let dir = std::env::temp_dir().join(format!("win32_rt_imagehlp_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Build a minimal PE32+ image with one WIN_CERTIFICATE entry (type 2 = PKCS#7).
        let payload = b"FAKE_PKCS7_PAYLOAD_1234";
        let cert_len = 8 + payload.len();
        let sec_off = 0x200usize;
        let mut pe = vec![0u8; sec_off + ((cert_len + 7) & !7)];
        pe[0..2].copy_from_slice(b"MZ");
        pe[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        pe[0x80..0x84].copy_from_slice(b"PE\0\0");
        let opt = 0x80 + 24;
        pe[opt..opt + 2].copy_from_slice(&0x20Bu16.to_le_bytes()); // PE32+
        pe[opt + 108..opt + 112].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
        let sec_dir = opt + 112 + 4 * 8;
        pe[sec_dir..sec_dir + 4].copy_from_slice(&(sec_off as u32).to_le_bytes());
        pe[sec_dir + 4..sec_dir + 8].copy_from_slice(&(((cert_len + 7) & !7) as u32).to_le_bytes());
        pe[sec_off..sec_off + 4].copy_from_slice(&(cert_len as u32).to_le_bytes());
        pe[sec_off + 4..sec_off + 6].copy_from_slice(&0x0200u16.to_le_bytes()); // WIN_CERT_REVISION_2_0
        pe[sec_off + 6..sec_off + 8].copy_from_slice(&0x0002u16.to_le_bytes()); // WIN_CERT_TYPE_PKCS_SIGNED_DATA
        pe[sec_off + 8..sec_off + cert_len].copy_from_slice(payload);

        let pe_path = dir.join("signed.exe");
        std::fs::write(&pe_path, &pe).unwrap();
        crate::path::add_mount("J:\\", &dir);

        unsafe {
            let wp = to_wide_z("J:\\signed.exe");
            let h = CreateFileW(wp.as_ptr(), GENERIC_READ, 1, std::ptr::null_mut(), OPEN_EXISTING, 0, std::ptr::null_mut());
            assert_ne!(h, INVALID_HANDLE_VALUE);

            let mut count = 0u32;
            let mut indices = [99u32; 4];
            assert_eq!(ImageEnumerateCertificates(h, CERT_SECTION_TYPE_ANY, &mut count, indices.as_mut_ptr(), 4), TRUE);
            assert_eq!(count, 1);
            assert_eq!(indices[0], 0);

            let mut hdr = [0u8; 8];
            assert_eq!(ImageGetCertificateHeader(h, 0, hdr.as_mut_ptr()), TRUE);
            assert_eq!(u32::from_le_bytes(hdr[0..4].try_into().unwrap()) as usize, cert_len);
            assert_eq!(u16::from_le_bytes(hdr[4..6].try_into().unwrap()), 0x0200);
            assert_eq!(u16::from_le_bytes(hdr[6..8].try_into().unwrap()), 0x0002);

            let mut req = 0u32;
            assert_eq!(ImageGetCertificateData(h, 0, std::ptr::null_mut(), &mut req), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INSUFFICIENT_BUFFER);
            assert_eq!(req as usize, cert_len);

            let mut data = vec![0u8; req as usize];
            assert_eq!(ImageGetCertificateData(h, 0, data.as_mut_ptr(), &mut req), TRUE);
            assert_eq!(&data[8..], payload);

            CloseHandle(h);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
