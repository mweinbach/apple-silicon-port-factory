//! Native `lz4.dll` block decompressor bridge.

#![allow(non_snake_case, unused_variables)]

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub extern "win64" fn LZ4_versionNumber() -> i32 {
    10904 // 1.9.4
}

pub extern "win64" fn LZ4_compressBound(input_size: i32) -> i32 {
    input_size + (input_size / 255) + 16
}

pub extern "win64" fn LZ4_compress_default(
    src: *const u8,
    dst: *mut u8,
    src_size: i32,
    dst_capacity: i32,
) -> i32 {
    // Emit a valid uncompressed LZ4 literal-only block so any compliant LZ4 decoder reads it back.
    if src.is_null() || dst.is_null() || src_size <= 0 || dst_capacity <= 0 {
        return 0;
    }
    let len = src_size as usize;
    let extra_len_bytes = if len >= 15 { (len - 15) / 255 + 1 } else { 0 };
    let needed = 1 + extra_len_bytes + len;
    if needed > dst_capacity as usize {
        return 0;
    }
    unsafe {
        let mut out = dst;
        if len < 15 {
            *out = (len as u8) << 4;
            out = out.add(1);
        } else {
            *out = 0xF0;
            out = out.add(1);
            let mut rem = len - 15;
            while rem >= 255 {
                *out = 255;
                out = out.add(1);
                rem -= 255;
            }
            *out = rem as u8;
            out = out.add(1);
        }
        std::ptr::copy_nonoverlapping(src, out, len);
    }
    needed as i32
}

pub extern "win64" fn LZ4_decompress_safe(
    src: *const u8,
    dst: *mut u8,
    compressed_size: i32,
    dst_capacity: i32,
) -> i32 {
    if src.is_null() || dst.is_null() || compressed_size < 0 || dst_capacity < 0 {
        return -1;
    }
    if compressed_size == 0 {
        return 0;
    }
    let s_slice = unsafe { std::slice::from_raw_parts(src, compressed_size as usize) };
    let d_slice = unsafe { std::slice::from_raw_parts_mut(dst, dst_capacity as usize) };
    match decode_lz4_block(s_slice, d_slice) {
        Ok(written) => written as i32,
        Err(()) => -1,
    }
}

fn decode_lz4_block(src: &[u8], dst: &mut [u8]) -> Result<usize, ()> {
    let mut sp = 0usize;
    let mut dp = 0usize;
    while sp < src.len() {
        let token = src[sp];
        sp += 1;
        let mut lit_len = (token >> 4) as usize;
        if lit_len == 15 {
            loop {
                let b = *src.get(sp).ok_or(())?;
                sp += 1;
                lit_len += b as usize;
                if b != 255 {
                    break;
                }
            }
        }
        if sp + lit_len > src.len() || dp + lit_len > dst.len() {
            return Err(());
        }
        dst[dp..dp + lit_len].copy_from_slice(&src[sp..sp + lit_len]);
        sp += lit_len;
        dp += lit_len;
        if sp == src.len() {
            break;
        }
        let lo = *src.get(sp).ok_or(())? as usize;
        let hi = *src.get(sp + 1).ok_or(())? as usize;
        sp += 2;
        let offset = lo | (hi << 8);
        if offset == 0 || offset > dp {
            return Err(());
        }
        let mut match_len = ((token & 0x0F) as usize) + 4;
        if (token & 0x0F) == 15 {
            loop {
                let b = *src.get(sp).ok_or(())?;
                sp += 1;
                match_len += b as usize;
                if b != 255 {
                    break;
                }
            }
        }
        if dp + match_len > dst.len() {
            return Err(());
        }
        for _ in 0..match_len {
            let b = dst[dp - offset];
            dst[dp] = b;
            dp += 1;
        }
    }
    Ok(dp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lz4_compress_and_decompress_roundtrip() {
        let input = b"Hello LZ4 native block roundtrip across 40 bytes!";
        let mut comp = vec![0u8; LZ4_compressBound(input.len() as i32) as usize];
        let c_len = LZ4_compress_default(
            input.as_ptr(),
            comp.as_mut_ptr(),
            input.len() as i32,
            comp.len() as i32,
        );
        assert!(c_len > 0);
        let mut out = vec![0u8; input.len()];
        let d_len = LZ4_decompress_safe(comp.as_ptr(), out.as_mut_ptr(), c_len, out.len() as i32);
        assert_eq!(d_len, input.len() as i32);
        assert_eq!(&out, input);
    }
}
