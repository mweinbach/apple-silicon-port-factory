//! Little-endian cursor used by the bank and .wem parsers.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub offset: usize,
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {:#x}", self.msg, self.offset)
    }
}

impl std::error::Error for ParseError {}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone)]
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
    /// Absolute offset of `buf[0]` within the enclosing file (for error messages).
    base: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0, base: 0 }
    }

    pub fn with_base(buf: &'a [u8], base: usize) -> Self {
        Self { buf, pos: 0, base }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn abs_pos(&self) -> usize {
        self.base + self.pos
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    pub fn err<T>(&self, msg: impl Into<String>) -> Result<T> {
        Err(ParseError { offset: self.abs_pos(), msg: msg.into() })
    }

    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.remaining() < n {
            return self.err(format!("unexpected end of data (need {} bytes, have {})", n, self.remaining()));
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    pub fn skip(&mut self, n: usize) -> Result<()> {
        self.bytes(n).map(|_| ())
    }

    /// Sub-reader over the next `n` bytes (advances this reader past them).
    pub fn sub(&mut self, n: usize) -> Result<Reader<'a>> {
        let base = self.abs_pos();
        let b = self.bytes(n)?;
        Ok(Reader::with_base(b, base))
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes(1)?[0])
    }
    pub fn i8(&mut self) -> Result<i8> {
        Ok(self.u8()? as i8)
    }
    pub fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.bytes(2)?.try_into().unwrap()))
    }
    pub fn i16(&mut self) -> Result<i16> {
        Ok(self.u16()? as i16)
    }
    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into().unwrap()))
    }
    pub fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }
    pub fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.bytes(4)?.try_into().unwrap()))
    }
    pub fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.bytes(8)?.try_into().unwrap()))
    }
    pub fn fourcc(&mut self) -> Result<[u8; 4]> {
        Ok(self.bytes(4)?.try_into().unwrap())
    }
    pub fn peek_u32(&self) -> Result<u32> {
        self.clone().u32()
    }

    /// Wwise 7-bit variable-length integer, most significant group first
    /// (wwiser `wmodel.py` TYPE_VAR; AK `AK::ReadVariableSizeInt`-style, used from v123+).
    pub fn var(&mut self) -> Result<u32> {
        let mut cur = self.u8()?;
        let mut value = (cur & 0x7F) as u32;
        let mut n = 0;
        while cur & 0x80 != 0 {
            if n >= 4 {
                return self.err("variable-size int too long");
            }
            cur = self.u8()?;
            value = (value << 7) | (cur & 0x7F) as u32;
            n += 1;
        }
        Ok(value)
    }

    /// NUL-terminated string (bank v137+ `AK::ReadBankStringUtf8`).
    pub fn stz(&mut self) -> Result<String> {
        let rest = &self.buf[self.pos..];
        let Some(n) = rest.iter().position(|&b| b == 0) else {
            return self.err("unterminated string");
        };
        let s = String::from_utf8_lossy(&rest[..n]).into_owned();
        self.pos += n + 1;
        Ok(s)
    }

    pub fn string(&mut self, n: usize) -> Result<String> {
        Ok(String::from_utf8_lossy(self.bytes(n)?).into_owned())
    }
}
