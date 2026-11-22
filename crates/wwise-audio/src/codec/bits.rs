//! LSB-first bit packing as used by Vorbis (and vgmstream's `bitstream_lsb.h`).

pub struct BitReader<'a> {
    buf: &'a [u8],
    bit: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, bit: 0 }
    }

    /// Reads `n` (0..=32) bits, least significant first. `None` past the end.
    pub fn get(&mut self, n: u32) -> Option<u32> {
        if n == 0 {
            return Some(0);
        }
        if self.bit + n as usize > self.buf.len() * 8 {
            return None;
        }
        let mut v: u64 = 0;
        for i in 0..n as usize {
            let b = self.bit + i;
            let bit = (self.buf[b / 8] >> (b % 8)) & 1;
            v |= (bit as u64) << i;
        }
        self.bit += n as usize;
        Some(v as u32)
    }

    pub fn pos(&self) -> usize {
        self.bit
    }
}

#[derive(Default)]
pub struct BitWriter {
    buf: Vec<u8>,
    bit: usize,
}

impl BitWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, n: u32, v: u32) {
        for i in 0..n as usize {
            if self.bit % 8 == 0 {
                self.buf.push(0);
            }
            if (v >> i) & 1 != 0 {
                let last = self.buf.len() - 1;
                self.buf[last] |= 1 << (self.bit % 8);
            }
            self.bit += 1;
        }
    }

    pub fn bit_len(&self) -> usize {
        self.bit
    }

    /// Pads to a byte boundary with zero bits and returns the bytes.
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let mut w = BitWriter::new();
        w.put(3, 5);
        w.put(10, 0x2AB);
        w.put(32, 0xDEADBEEF);
        w.put(1, 1);
        let b = w.finish();
        let mut r = BitReader::new(&b);
        assert_eq!(r.get(3), Some(5));
        assert_eq!(r.get(10), Some(0x2AB));
        assert_eq!(r.get(32), Some(0xDEADBEEF));
        assert_eq!(r.get(1), Some(1));
    }
}
