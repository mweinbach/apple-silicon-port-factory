//! Compact 16-byte CIL evaluation-stack values, managed addresses, direct
//! unaligned memory load/store primitives, and numeric/comparison helpers.

use super::super::metadata::SigType;
use super::heap::MONO_OBJECT_HEADER_SIZE;

/// Managed interior or frame-slot address (for `ldloca`/`ldarga`/`ldsflda`/`ldelema`/`byref`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManagedAddr {
    Raw(usize),
    Local { frame: u32, idx: u32 },
    Arg { frame: u32, idx: u32 },
    StaticField(u32),
}

/// Compact 16-byte CIL runtime value on the contiguous evaluation stack.
#[derive(Clone, Copy, Debug)]
pub enum CilVal {
    I32(i32),
    I64(i64),
    NativeInt(isize),
    F64(f64),
    Obj(usize),
    RawPtr(usize),
    LocalPtr { frame: u32, idx: u32 },
    ArgPtr { frame: u32, idx: u32 },
    StaticFieldPtr(u32),
    ValueType { class_id: u32, ptr: usize },
}

const _: () = assert!(std::mem::size_of::<CilVal>() == 16);

impl CilVal {
    #[inline]
    pub fn as_managed_addr(self) -> Option<ManagedAddr> {
        match self {
            Self::RawPtr(p) => Some(ManagedAddr::Raw(p)),
            Self::LocalPtr { frame, idx } => Some(ManagedAddr::Local { frame, idx }),
            Self::ArgPtr { frame, idx } => Some(ManagedAddr::Arg { frame, idx }),
            Self::StaticFieldPtr(fid) => Some(ManagedAddr::StaticField(fid)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_managed_ptr(self) -> bool {
        matches!(
            self,
            Self::RawPtr(_)
                | Self::LocalPtr { .. }
                | Self::ArgPtr { .. }
                | Self::StaticFieldPtr(_)
        )
    }

    #[inline]
    pub fn as_i32(self) -> i32 {
        match self {
            CilVal::I32(v) => v,
            CilVal::I64(v) => v as i32,
            CilVal::NativeInt(v) => v as i32,
            CilVal::F64(v) => v as i32,
            CilVal::Obj(p) => p as i32,
            CilVal::RawPtr(p) => p as i32,
            _ => 0,
        }
    }

    #[inline]
    pub fn as_i64(self) -> i64 {
        match self {
            CilVal::I32(v) => v as i64,
            CilVal::I64(v) => v,
            CilVal::NativeInt(v) => v as i64,
            CilVal::F64(v) => v as i64,
            CilVal::Obj(p) => p as i64,
            CilVal::RawPtr(p) => p as i64,
            _ => 0,
        }
    }

    #[inline]
    pub fn as_u64_bits(self) -> u64 {
        match self {
            CilVal::I32(v) => v as u32 as u64,
            CilVal::I64(v) => v as u64,
            CilVal::NativeInt(v) => v as usize as u64,
            CilVal::F64(v) => v.to_bits(),
            CilVal::Obj(p) => p as u64,
            CilVal::RawPtr(p) => p as u64,
            CilVal::ValueType { ptr, .. } => ptr as u64,
            CilVal::LocalPtr { .. } | CilVal::ArgPtr { .. } | CilVal::StaticFieldPtr(_) => 0,
        }
    }

    #[inline]
    pub fn as_isize(self) -> isize {
        match self {
            CilVal::I32(v) => v as isize,
            CilVal::I64(v) => v as isize,
            CilVal::NativeInt(v) => v,
            CilVal::Obj(p) => p as isize,
            CilVal::RawPtr(p) => p as isize,
            CilVal::F64(v) => v as isize,
            _ => 0,
        }
    }

    #[inline]
    pub fn as_f64(self) -> f64 {
        match self {
            CilVal::F64(v) => v,
            CilVal::I32(v) => v as f64,
            CilVal::I64(v) => v as f64,
            CilVal::NativeInt(v) => v as f64,
            _ => 0.0,
        }
    }

    #[inline]
    pub fn as_obj(self) -> usize {
        match self {
            CilVal::Obj(p) => p,
            CilVal::NativeInt(p) => p as usize,
            CilVal::I64(p) => p as usize,
            CilVal::I32(p) => p as u32 as usize,
            _ => 0,
        }
    }

    #[inline]
    pub fn is_truthy(self) -> bool {
        match self {
            CilVal::I32(v) => v != 0,
            CilVal::I64(v) => v != 0,
            CilVal::NativeInt(v) => v != 0,
            CilVal::Obj(p) => p != 0,
            CilVal::F64(v) => v != 0.0,
            CilVal::RawPtr(p) => p != 0,
            CilVal::LocalPtr { .. } | CilVal::ArgPtr { .. } | CilVal::StaticFieldPtr(_) => true,
            CilVal::ValueType { ptr, .. } => ptr != 0,
        }
    }
}

/// Compact copyable storage kind avoiding `SigType::clone()` on field/element/indirect ops.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StorageKind {
    Bool,
    I1,
    U1,
    I2,
    U2,
    I4,
    U4,
    I8,
    U8,
    R4,
    R8,
    NativeInt,
    Ref,
}

impl StorageKind {
    #[inline]
    pub fn from_sig(sig: &SigType) -> Self {
        match sig {
            SigType::Boolean => Self::Bool,
            SigType::I1 => Self::I1,
            SigType::U1 => Self::U1,
            SigType::I2 => Self::I2,
            SigType::Char | SigType::U2 => Self::U2,
            SigType::I4 => Self::I4,
            SigType::U4 => Self::U4,
            SigType::I8 => Self::I8,
            SigType::U8 => Self::U8,
            SigType::R4 => Self::R4,
            SigType::R8 => Self::R8,
            SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
                Self::NativeInt
            }
            _ => Self::Ref,
        }
    }

    #[inline]
    pub fn default_val(self) -> CilVal {
        match self {
            Self::Bool | Self::I1 | Self::U1 | Self::I2 | Self::U2 | Self::I4 | Self::U4 => {
                CilVal::I32(0)
            }
            Self::I8 | Self::U8 => CilVal::I64(0),
            Self::R4 | Self::R8 => CilVal::F64(0.0),
            Self::NativeInt => CilVal::NativeInt(0),
            Self::Ref => CilVal::Obj(0),
        }
    }
}

#[inline]
pub fn default_val_for_sig(sig: &SigType) -> CilVal {
    StorageKind::from_sig(sig).default_val()
}

/// Stack-allocated serializer (zero heap allocation) for primitive/reference boxing.
#[inline]
pub(crate) fn write_sig_to_stack_buf(sig: &SigType, val: CilVal) -> ([u8; 8], usize) {
    let mut buf = [0u8; 8];
    let len = match sig {
        SigType::Boolean => {
            buf[0] = u8::from(val.is_truthy());
            1
        }
        SigType::I1 | SigType::U1 => {
            buf[0] = val.as_i32() as u8;
            1
        }
        SigType::Char | SigType::I2 | SigType::U2 => {
            buf[..2].copy_from_slice(&(val.as_i32() as u16).to_le_bytes());
            2
        }
        SigType::I4 | SigType::U4 => {
            buf[..4].copy_from_slice(&val.as_i32().to_le_bytes());
            4
        }
        SigType::I8 | SigType::U8 => {
            buf.copy_from_slice(&val.as_i64().to_le_bytes());
            8
        }
        SigType::R4 => {
            buf[..4].copy_from_slice(&(val.as_f64() as f32).to_le_bytes());
            4
        }
        SigType::R8 => {
            buf.copy_from_slice(&val.as_f64().to_le_bytes());
            8
        }
        SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
            let b = (val.as_isize() as usize).to_le_bytes();
            buf[..b.len()].copy_from_slice(&b);
            b.len()
        }
        _ => {
            let b = val.as_obj().to_le_bytes();
            buf[..b.len()].copy_from_slice(&b);
            b.len()
        }
    };
    (buf, len)
}

#[inline]
pub(crate) fn read_kind_from_ptr(kind: StorageKind, ptr: *const u8) -> CilVal {
    unsafe {
        match kind {
            StorageKind::Bool | StorageKind::U1 => CilVal::I32(std::ptr::read_unaligned(ptr) as i32),
            StorageKind::I1 => CilVal::I32(std::ptr::read_unaligned(ptr as *const i8) as i32),
            StorageKind::U2 => CilVal::I32(std::ptr::read_unaligned(ptr as *const u16) as i32),
            StorageKind::I2 => CilVal::I32(std::ptr::read_unaligned(ptr as *const i16) as i32),
            StorageKind::I4 | StorageKind::U4 => {
                CilVal::I32(std::ptr::read_unaligned(ptr as *const i32))
            }
            StorageKind::I8 | StorageKind::U8 => {
                CilVal::I64(std::ptr::read_unaligned(ptr as *const i64))
            }
            StorageKind::R4 => CilVal::F64(std::ptr::read_unaligned(ptr as *const f32) as f64),
            StorageKind::R8 => CilVal::F64(std::ptr::read_unaligned(ptr as *const f64)),
            StorageKind::NativeInt => {
                CilVal::NativeInt(std::ptr::read_unaligned(ptr as *const isize))
            }
            StorageKind::Ref => CilVal::Obj(std::ptr::read_unaligned(ptr as *const usize)),
        }
    }
}

#[inline]
pub(crate) fn write_kind_to_ptr(kind: StorageKind, ptr: *mut u8, val: CilVal) {
    unsafe {
        match kind {
            StorageKind::Bool => std::ptr::write_unaligned(ptr, u8::from(val.is_truthy())),
            StorageKind::I1 | StorageKind::U1 => {
                std::ptr::write_unaligned(ptr, val.as_i32() as u8)
            }
            StorageKind::I2 | StorageKind::U2 => {
                std::ptr::write_unaligned(ptr as *mut u16, val.as_i32() as u16)
            }
            StorageKind::I4 | StorageKind::U4 => {
                std::ptr::write_unaligned(ptr as *mut i32, val.as_i32())
            }
            StorageKind::I8 | StorageKind::U8 => {
                std::ptr::write_unaligned(ptr as *mut i64, val.as_i64())
            }
            StorageKind::R4 => std::ptr::write_unaligned(ptr as *mut f32, val.as_f64() as f32),
            StorageKind::R8 => std::ptr::write_unaligned(ptr as *mut f64, val.as_f64()),
            StorageKind::NativeInt => {
                std::ptr::write_unaligned(ptr as *mut usize, val.as_isize() as usize)
            }
            StorageKind::Ref => std::ptr::write_unaligned(ptr as *mut usize, val.as_obj()),
        }
    }
}

#[inline]
pub fn read_sig_from_ptr(sig: &SigType, ptr: *const u8) -> CilVal {
    read_kind_from_ptr(StorageKind::from_sig(sig), ptr)
}

#[inline]
pub fn write_sig_to_ptr(sig: &SigType, ptr: *mut u8, val: CilVal) {
    write_kind_to_ptr(StorageKind::from_sig(sig), ptr, val)
}

#[inline]
pub(crate) fn field_base_ptr(target: CilVal) -> Option<usize> {
    match target {
        CilVal::Obj(0) => None,
        CilVal::Obj(p) => Some(p + MONO_OBJECT_HEADER_SIZE),
        CilVal::RawPtr(p) if p != 0 => Some(p),
        CilVal::ValueType { ptr, .. } if ptr != 0 => Some(ptr),
        CilVal::NativeInt(p) if p != 0 => Some(p as usize),
        _ => None,
    }
}

#[inline]
pub(crate) fn raw_ptr_from_val(v: CilVal) -> Option<usize> {
    match v {
        CilVal::RawPtr(p) if p != 0 => Some(p),
        CilVal::NativeInt(p) if p != 0 => Some(p as usize),
        CilVal::I64(p) if p != 0 => Some(p as usize),
        CilVal::Obj(p) if p != 0 => Some(p),
        CilVal::ValueType { ptr, .. } if ptr != 0 => Some(ptr),
        _ => None,
    }
}

/// Inline stack-allocated argument buffer (zero heap allocation for <= 8 args).
pub(crate) struct SmallArgs {
    inline: [CilVal; 8],
    spill: Vec<CilVal>,
    len: usize,
}

impl SmallArgs {
    #[inline]
    pub fn pop_from(val_stack: &mut Vec<CilVal>, argc: usize) -> Self {
        let start = val_stack.len().saturating_sub(argc);
        let avail = val_stack.len() - start;
        let pad = argc - avail;
        if argc <= 8 {
            let mut inline = [CilVal::I32(0); 8];
            inline[pad..argc].copy_from_slice(&val_stack[start..]);
            val_stack.truncate(start);
            Self {
                inline,
                spill: Vec::new(),
                len: argc,
            }
        } else {
            let mut spill = vec![CilVal::I32(0); argc];
            spill[pad..argc].copy_from_slice(&val_stack[start..]);
            val_stack.truncate(start);
            Self {
                inline: [CilVal::I32(0); 8],
                spill,
                len: argc,
            }
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[CilVal] {
        if self.len <= 8 {
            &self.inline[..self.len]
        } else {
            &self.spill
        }
    }
}

// ---------------------------------------------------------------------------
// Arithmetic, Comparison, Conversion Helpers
// ---------------------------------------------------------------------------

#[inline]
pub(crate) fn arith_bin(a: CilVal, b: CilVal, op: u8) -> CilVal {
    match (a, b) {
        (CilVal::F64(_), _) | (_, CilVal::F64(_)) => {
            let (x, y) = (a.as_f64(), b.as_f64());
            CilVal::F64(match op {
                0 => x + y,
                1 => x - y,
                _ => x * y,
            })
        }
        (CilVal::I64(_), _) | (_, CilVal::I64(_)) => {
            let (x, y) = (a.as_i64(), b.as_i64());
            CilVal::I64(match op {
                0 => x.wrapping_add(y),
                1 => x.wrapping_sub(y),
                _ => x.wrapping_mul(y),
            })
        }
        (CilVal::NativeInt(_), _) | (_, CilVal::NativeInt(_)) => {
            let (x, y) = (a.as_isize(), b.as_isize());
            CilVal::NativeInt(match op {
                0 => x.wrapping_add(y),
                1 => x.wrapping_sub(y),
                _ => x.wrapping_mul(y),
            })
        }
        _ if a.is_managed_ptr() || b.is_managed_ptr() => {
            let (x, y) = (a.as_isize(), b.as_isize());
            CilVal::NativeInt(match op {
                0 => x.wrapping_add(y),
                1 => x.wrapping_sub(y),
                _ => x.wrapping_mul(y),
            })
        }
        _ => {
            let (x, y) = (a.as_i32(), b.as_i32());
            CilVal::I32(match op {
                0 => x.wrapping_add(y),
                1 => x.wrapping_sub(y),
                _ => x.wrapping_mul(y),
            })
        }
    }
}

#[inline]
pub(crate) fn arith_div(a: CilVal, b: CilVal, unsigned: bool) -> Result<CilVal, bool> {
    if matches!(a, CilVal::F64(_)) || matches!(b, CilVal::F64(_)) {
        return Ok(CilVal::F64(a.as_f64() / b.as_f64()));
    }
    if matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_)) {
        let (x, y) = (a.as_i64(), b.as_i64());
        if y == 0 {
            return Err(true);
        }
        if unsigned {
            return Ok(CilVal::I64(((x as u64) / (y as u64)) as i64));
        }
        if x == i64::MIN && y == -1 {
            return Err(false);
        }
        return Ok(CilVal::I64(x / y));
    }
    let (x, y) = (a.as_i32(), b.as_i32());
    if y == 0 {
        return Err(true);
    }
    if unsigned {
        return Ok(CilVal::I32(((x as u32) / (y as u32)) as i32));
    }
    if x == i32::MIN && y == -1 {
        return Err(false);
    }
    Ok(CilVal::I32(x / y))
}

#[inline]
pub(crate) fn arith_rem(a: CilVal, b: CilVal, unsigned: bool) -> Result<CilVal, bool> {
    if matches!(a, CilVal::F64(_)) || matches!(b, CilVal::F64(_)) {
        return Ok(CilVal::F64(a.as_f64() % b.as_f64()));
    }
    if matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_)) {
        let (x, y) = (a.as_i64(), b.as_i64());
        if y == 0 {
            return Err(true);
        }
        if unsigned {
            return Ok(CilVal::I64(((x as u64) % (y as u64)) as i64));
        }
        if x == i64::MIN && y == -1 {
            return Err(false);
        }
        return Ok(CilVal::I64(x % y));
    }
    let (x, y) = (a.as_i32(), b.as_i32());
    if y == 0 {
        return Err(true);
    }
    if unsigned {
        return Ok(CilVal::I32(((x as u32) % (y as u32)) as i32));
    }
    if x == i32::MIN && y == -1 {
        return Err(false);
    }
    Ok(CilVal::I32(x % y))
}

#[inline]
pub(crate) fn arith_checked(op: u8, a: CilVal, b: CilVal) -> Result<CilVal, ()> {
    let is64 = matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_));
    if is64 {
        let (x, y) = (a.as_i64(), b.as_i64());
        let r = match op {
            0xD6 => x.checked_add(y),
            0xD7 => (x as u64).checked_add(y as u64).map(|v| v as i64),
            0xD8 => x.checked_mul(y),
            0xD9 => (x as u64).checked_mul(y as u64).map(|v| v as i64),
            0xDA => x.checked_sub(y),
            0xDB => (x as u64).checked_sub(y as u64).map(|v| v as i64),
            _ => None,
        };
        r.map(CilVal::I64).ok_or(())
    } else {
        let (x, y) = (a.as_i32(), b.as_i32());
        let r = match op {
            0xD6 => x.checked_add(y),
            0xD7 => (x as u32).checked_add(y as u32).map(|v| v as i32),
            0xD8 => x.checked_mul(y),
            0xD9 => (x as u32).checked_mul(y as u32).map(|v| v as i32),
            0xDA => x.checked_sub(y),
            0xDB => (x as u32).checked_sub(y as u32).map(|v| v as i32),
            _ => None,
        };
        r.map(CilVal::I32).ok_or(())
    }
}

#[inline]
pub(crate) fn bitwise_bin(a: CilVal, b: CilVal, op: u8) -> CilVal {
    if matches!(a, CilVal::I64(_)) || matches!(b, CilVal::I64(_)) {
        let (x, y) = (a.as_i64(), b.as_i64());
        CilVal::I64(match op {
            0 => x & y,
            1 => x | y,
            _ => x ^ y,
        })
    } else if matches!(a, CilVal::NativeInt(_)) || matches!(b, CilVal::NativeInt(_)) {
        let (x, y) = (a.as_isize(), b.as_isize());
        CilVal::NativeInt(match op {
            0 => x & y,
            1 => x | y,
            _ => x ^ y,
        })
    } else {
        let (x, y) = (a.as_i32(), b.as_i32());
        CilVal::I32(match op {
            0 => x & y,
            1 => x | y,
            _ => x ^ y,
        })
    }
}

#[inline]
pub(crate) fn shift_bin(a: CilVal, b: CilVal, op: u8) -> CilVal {
    let sh = b.as_i32() as u32;
    match a {
        CilVal::I64(x) => CilVal::I64(match op {
            0 => x.wrapping_shl(sh & 63),
            1 => x.wrapping_shr(sh & 63),
            _ => (x as u64).wrapping_shr(sh & 63) as i64,
        }),
        CilVal::NativeInt(x) => CilVal::NativeInt(match op {
            0 => x.wrapping_shl(sh & 63),
            1 => x.wrapping_shr(sh & 63),
            _ => (x as usize).wrapping_shr(sh & 63) as isize,
        }),
        _ => {
            let x = a.as_i32();
            CilVal::I32(match op {
                0 => x.wrapping_shl(sh & 31),
                1 => x.wrapping_shr(sh & 31),
                _ => (x as u32).wrapping_shr(sh & 31) as i32,
            })
        }
    }
}

#[inline]
pub(crate) fn eval_branch_cmp(kind: u8, a: CilVal, b: CilVal) -> bool {
    if matches!(a, CilVal::F64(_)) || matches!(b, CilVal::F64(_)) {
        let (x, y) = (a.as_f64(), b.as_f64());
        return match kind {
            0 => x == y,    // beq
            1 => x >= y,    // bge
            2 => x > y,     // bgt
            3 => x <= y,    // ble
            4 => x < y,     // blt
            5 => !(x == y), // bne.un
            6 => !(x < y),  // bge.un
            7 => !(x <= y), // bgt.un
            8 => !(x > y),  // ble.un
            _ => !(x >= y), // blt.un
        };
    }
    let wide = matches!(
        (a, b),
        (CilVal::I64(_), _)
            | (_, CilVal::I64(_))
            | (CilVal::NativeInt(_), _)
            | (_, CilVal::NativeInt(_))
            | (CilVal::Obj(_), _)
            | (_, CilVal::Obj(_))
    );
    if wide {
        let (sx, sy) = (a.as_i64(), b.as_i64());
        let (ux, uy) = (a.as_u64_bits(), b.as_u64_bits());
        match kind {
            0 => ux == uy,
            1 => sx >= sy,
            2 => sx > sy,
            3 => sx <= sy,
            4 => sx < sy,
            5 => ux != uy,
            6 => ux >= uy,
            7 => ux > uy,
            8 => ux <= uy,
            _ => ux < uy,
        }
    } else {
        let (sx, sy) = (a.as_i32(), b.as_i32());
        let (ux, uy) = (sx as u32, sy as u32);
        match kind {
            0 => sx == sy,
            1 => sx >= sy,
            2 => sx > sy,
            3 => sx <= sy,
            4 => sx < sy,
            5 => sx != sy,
            6 => ux >= uy,
            7 => ux > uy,
            8 => ux <= uy,
            _ => ux < uy,
        }
    }
}

#[inline]
pub(crate) fn eval_cmp_op(op2: u8, a: CilVal, b: CilVal) -> bool {
    match op2 {
        0x01 => eval_branch_cmp(0, a, b), // ceq
        0x02 => eval_branch_cmp(2, a, b), // cgt
        0x03 => eval_branch_cmp(7, a, b), // cgt.un
        0x04 => eval_branch_cmp(4, a, b), // clt
        0x05 => eval_branch_cmp(9, a, b), // clt.un
        _ => false,
    }
}

#[inline]
pub(crate) fn conv_checked(op: u8, v: CilVal) -> Result<CilVal, ()> {
    let unsigned_in = (0x82..=0x8A).contains(&op);
    if unsigned_in {
        let u = match v {
            CilVal::I32(x) => x as u32 as u64,
            CilVal::I64(x) => x as u64,
            CilVal::NativeInt(x) => x as usize as u64,
            CilVal::F64(x) if x >= 0.0 && x.is_finite() => x as u64,
            _ => return Err(()),
        };
        match op {
            0x82 => i8::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x83 => i16::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x84 => i32::try_from(u).map(CilVal::I32).map_err(|_| ()),
            0x85 => i64::try_from(u).map(CilVal::I64).map_err(|_| ()),
            0x86 => u8::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x87 => u16::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x88 => u32::try_from(u).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0x89 => Ok(CilVal::I64(u as i64)),
            0x8A => isize::try_from(u).map(CilVal::NativeInt).map_err(|_| ()),
            _ => Err(()),
        }
    } else {
        let s: i64 = match v {
            CilVal::I32(x) => x as i64,
            CilVal::I64(x) => x,
            CilVal::NativeInt(x) => x as i64,
            CilVal::F64(x) if x.is_finite() && x >= (i64::MIN as f64) && x <= (i64::MAX as f64) => {
                x as i64
            }
            _ => return Err(()),
        };
        match op {
            0xB3 => i8::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB4 => u8::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB5 => i16::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB6 => u16::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB7 => i32::try_from(s).map(CilVal::I32).map_err(|_| ()),
            0xB8 => u32::try_from(s).map(|x| CilVal::I32(x as i32)).map_err(|_| ()),
            0xB9 => Ok(CilVal::I64(s)),
            0xBA => u64::try_from(s).map(|x| CilVal::I64(x as i64)).map_err(|_| ()),
            0xD4 => isize::try_from(s).map(CilVal::NativeInt).map_err(|_| ()),
            0xD5 => usize::try_from(s).map(|x| CilVal::NativeInt(x as isize)).map_err(|_| ()),
            _ => Err(()),
        }
    }
}
