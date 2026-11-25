//! Long-mode user XSAVE components (Intel SDM vol. 1, chapters 10 and 13).
//! Only the selected fields may be touched: a caller can place an unused component on a guard page.
use super::cpuid::{XCR0, YMM_OFFSET};
use super::regfile_ext::{is_valid_guest_range, x87_restore, x87_snapshot, X87State};
use super::{apply_guest_mxcsr_hw, Cpu};
use iced_x86::{Instruction, Mnemonic::*};

const X87: u64 = 1;
const SSE: u64 = 2;
const YMM: u64 = 4;
const COMPACT: u64 = 1 << 63;
const MXCSR_MASK: u32 = 0xffff;
const DEFAULT_MXCSR: u32 = 0x1f80;

#[derive(Debug, PartialEq)]
pub(super) struct Fault {
    address: u64,
    write: bool,
}

impl Fault {
    pub(super) fn gp() -> Self {
        // Windows reports a #GP(0) as an access violation with no linear fault address.
        Self { address: u64::MAX, write: false }
    }

    pub(super) unsafe fn dispatch(self, cpu: &mut Cpu, ip: u64) {
        let sp = cpu.jx.ctx_sp as *mut u64;
        if sp.is_null() {
            // The single-step oracle consumes pending faults and rolls back its own snapshot.
            super::FAULT_PENDING.with(|p| p.set(Some((self.address, self.write))));
            return;
        }
        use win32_rt::unwind;
        let mut rec = unwind::guest_exception_record(
            unwind::STATUS_ACCESS_VIOLATION,
            ip,
            &[self.write as u64, self.address],
        );
        let mut ctx = unwind::guest_context(&cpu.gpr, &cpu.xmm, ip, cpu.rflags());
        ctx.mx_csr = cpu.jx.mxcsr;
        ctx.flt_save[24..28].copy_from_slice(&cpu.jx.mxcsr.to_le_bytes());
        let saved_sp = *sp;
        *sp = cpu.gpr[4].wrapping_sub(256) & !15;
        let anchor = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp);
        {
            let _host = super::cpu::HostFpcrGuard::enter(&raw mut cpu.jx.mxcsr);
            if !unwind::dispatch_veh_guest(&mut rec, &mut ctx) {
                unwind::emu_unhandled_exception(&mut rec, &mut ctx, true);
            }
        }
        crate::lstack::anchor_leave(crate::callctx::ctx(), anchor);
        *sp = saved_sp;
        cpu.gpr = ctx.gpr;
        cpu.last_bits.fill(64);
        for i in 0..16 {
            cpu.xmm[i] = u128::from_le_bytes(*ctx.xmm_mut(i));
        }
        cpu.rip = ctx.rip;
        cpu.set_rflags(ctx.eflags);
        cpu.jx.mxcsr = ctx.mx_csr & MXCSR_MASK;
        apply_guest_mxcsr_hw(cpu.jx.mxcsr);
    }
}

struct Area(u64);

impl Area {
    fn read<const N: usize>(&self, off: usize) -> Result<[u8; N], Fault> {
        let addr = self.0.wrapping_add(off as u64);
        let mut buf = [0; N];
        if is_valid_guest_range(addr, N) && win32_rt::crash::safe_read(addr, &mut buf) {
            Ok(buf)
        } else {
            Err(Fault { address: addr, write: false })
        }
    }

    fn write(&self, off: usize, bytes: &[u8]) -> Result<(), Fault> {
        unsafe extern "C" {
            static mach_task_self_: u32;
            fn mach_vm_write(task: u32, addr: u64, data: usize, count: u32) -> i32;
        }
        let addr = self.0.wrapping_add(off as u64);
        if is_valid_guest_range(addr, bytes.len())
            && unsafe { mach_vm_write(mach_task_self_, addr, bytes.as_ptr() as usize, bytes.len() as u32) }
                == 0
        {
            Ok(())
        } else {
            Err(Fault { address: addr, write: true })
        }
    }
}

impl X87State {
    fn save(&self, area: &Area, wide: bool) -> Result<(), Fault> {
        let mut env = [0; 24];
        env[0..2].copy_from_slice(&self.cw.to_le_bytes());
        env[2..4].copy_from_slice(&self.status_word().to_le_bytes());
        env[4] = self.valid_mask; // physical register order
        env[6..8].copy_from_slice(&self.fop.to_le_bytes());
        let pointer_mask = if wide { u64::MAX } else { u32::MAX.into() };
        env[8..16].copy_from_slice(&(self.fip & pointer_mask).to_le_bytes());
        env[16..24].copy_from_slice(&(self.fdp & pointer_mask).to_le_bytes());
        area.write(0, &env)?;
        let mut st = [0; 128];
        for (i, slot) in st.chunks_exact_mut(16).enumerate() {
            slot[..10].copy_from_slice(&self.regs[self.phys(i as u8)]); // logical ST(i) order
        }
        area.write(32, &st)
    }

    fn load(area: &Area, wide: bool) -> Result<Self, Fault> {
        let env: [u8; 24] = area.read(0)?;
        let st: [u8; 128] = area.read(32)?;
        let mut state = Self::new();
        state.cw = control_word(u16::from_le_bytes(env[0..2].try_into().unwrap()));
        state.sw = u16::from_le_bytes(env[2..4].try_into().unwrap());
        state.top = ((state.sw >> 11) & 7) as u8;
        state.valid_mask = env[4];
        state.fop = u16::from_le_bytes(env[6..8].try_into().unwrap()) & 0x7ff;
        let pointer_mask = if wide { u64::MAX } else { u32::MAX.into() };
        state.fip = u64::from_le_bytes(env[8..16].try_into().unwrap()) & pointer_mask;
        state.fdp = u64::from_le_bytes(env[16..24].try_into().unwrap()) & pointer_mask;
        for (i, slot) in st.chunks_exact(16).enumerate() {
            let phys = state.phys(i as u8);
            state.regs[phys].copy_from_slice(&slot[..10]);
        }
        Ok(state)
    }
}

// FCW bit 6 reads as one. Bits 15:13 and 7 are reserved.
pub(super) fn control_word(value: u16) -> u16 {
    (value & 0x1f3f) | 0x40
}

impl Cpu {
    pub(super) unsafe fn exec_xstate(&mut self, ins: &Instruction) -> Result<(), Fault> {
        let mn = ins.mnemonic();
        if mn == Xgetbv {
            if self.gpr[1] as u32 != 0 {
                return Err(Fault::gp());
            }
            self.gpr[0] = XCR0.into();
            self.gpr[2] = 0;
            self.last_bits[0] = 64;
            self.last_bits[2] = 64;
            return Ok(());
        }

        let area = Area(self.eff_addr(ins));
        let legacy = matches!(mn, Fxsave | Fxsave64 | Fxrstor | Fxrstor64);
        let wide = matches!(mn, Fxsave64 | Fxrstor64 | Xsave64 | Xsaveopt64 | Xsavec64 | Xrstor64);
        let mask = if legacy { X87 | SSE } else { self.gpr[0] & u64::from(XCR0) };
        if area.0 & (if legacy { 15 } else { 63 }) != 0 {
            return Err(Fault::gp());
        }
        let restore = matches!(mn, Fxrstor | Fxrstor64 | Xrstor | Xrstor64);
        if !is_valid_guest_range(area.0, 1) {
            return Err(Fault { address: area.0, write: !restore });
        }
        if restore {
            return self.restore_state(&area, mask, legacy, wide);
        }
        // We always track the three components as in-use and modified. The SDM permits
        // XSAVEOPT / XSAVEC to store every selected component, including initial values.
        let compact = matches!(mn, Xsavec | Xsavec64);
        if mask & X87 != 0 {
            x87_snapshot().save(&area, wide)?;
        }
        if mask & SSE != 0 || (!compact && mask & YMM != 0) {
            let mx = u64::from(self.jx.mxcsr) | (u64::from(MXCSR_MASK) << 32);
            area.write(24, &mx.to_le_bytes())?;
        }
        if mask & SSE != 0 {
            area.write(160, self.xmm.map(u128::to_le_bytes).as_flattened())?;
        }
        if mask & YMM != 0 {
            area.write(YMM_OFFSET, self.ymm_hi.map(u128::to_le_bytes).as_flattened())?;
        }
        if !legacy {
            let bv = if compact {
                area.write(520, &(COMPACT | mask).to_le_bytes())?;
                mask
            } else {
                u64::from_le_bytes(area.read(512)?) | mask
            };
            area.write(512, &bv.to_le_bytes())?;
        }
        Ok(())
    }

    fn restore_state(&mut self, area: &Area, mask: u64, legacy: bool, wide: bool) -> Result<(), Fault> {
        let (bv, compact) = if legacy {
            (mask, false)
        } else {
            let bv = u64::from_le_bytes(area.read(512)?);
            let comp = u64::from_le_bytes(area.read(520)?);
            let compact = comp & COMPACT != 0;
            let valid = if compact {
                comp & !(COMPACT | u64::from(XCR0)) == 0
                    && bv & !(comp & !COMPACT) == 0
                    && area.read::<48>(528)? == [0; 48]
            } else {
                comp == 0 && bv & !u64::from(XCR0) == 0 && area.read::<8>(528)? == [0; 8]
            };
            if !valid {
                return Err(Fault::gp());
            }
            (bv, compact)
        };

        // Validate and stage every requested component before changing live guest state.
        let mxcsr = if compact && mask & SSE != 0 && bv & SSE == 0 {
            Some(DEFAULT_MXCSR)
        } else if mask & SSE != 0 || (!compact && mask & YMM != 0) {
            let value = u32::from_le_bytes(area.read(24)?);
            if value & !MXCSR_MASK != 0 {
                return Err(Fault::gp());
            }
            Some(value)
        } else {
            None
        };
        let x87 = if mask & bv & X87 != 0 { X87State::load(area, wide)? } else { X87State::new() };
        let vector = |bit, offset| -> Result<[u128; 16], Fault> {
            if mask & bv & bit == 0 {
                return Ok([0; 16]);
            }
            let bytes: [u8; 256] = area.read(offset)?;
            Ok(std::array::from_fn(|i| u128::from_le_bytes(bytes[16 * i..16 * i + 16].try_into().unwrap())))
        };
        let xmm = vector(SSE, 160)?;
        let ymm = vector(YMM, YMM_OFFSET)?;
        if mask & X87 != 0 {
            x87_restore(x87);
        }
        if mask & SSE != 0 {
            self.xmm = xmm;
        }
        if mask & YMM != 0 {
            for (i, v) in ymm.into_iter().enumerate() {
                self.set_ymm_hi(i, v);
            }
        }
        if let Some(value) = mxcsr {
            self.jx.mxcsr = value;
            apply_guest_mxcsr_hw(value);
        }
        Ok(())
    }
}
