// This file is also embedded in standalone generated Rust AOT crates.
/// Timestamp used when comparing guest executions with a frozen clock.
pub const GUEST_TSC: u64 = 0x1000;
#[cfg(any(target_arch = "aarch64", test))]
pub const TSC_NOMINAL_HZ: u64 = 1_000_000_000;
#[cfg(any(target_arch = "aarch64", test))]
pub const TWO_THIRDS_Q64: u64 = 0xAAAA_AAAA_AAAA_AAAB;

#[cfg(any(target_arch = "aarch64", test))]
#[inline]
pub const fn is_24mhz_to_ns(frequency: u64) -> bool {
    frequency == 24_000_000 && TSC_NOMINAL_HZ == 1_000_000_000
}

/// Convert an architectural counter to guest nanoseconds, wrapping at 64 bits.
#[cfg(any(target_arch = "aarch64", test))]
#[inline]
pub fn scale_counter_to_tsc(counter: u64, frequency: u64) -> u64 {
    match frequency {
        // Preserve the raw value if the host has not initialized its frequency register.
        0 | TSC_NOMINAL_HZ => counter,
        // 125/3 = 41 + 2/3. Rounding the Q64 multiplier up adds less than 1/3
        // for any u64 input, so the high product is exactly floor(2*counter/3).
        hz if is_24mhz_to_ns(hz) => {
            let fraction = (u128::from(counter) * u128::from(TWO_THIRDS_Q64)) >> 64;
            counter.wrapping_mul(41).wrapping_add(fraction as u64)
        }
        _ => (counter as u128 * TSC_NOMINAL_HZ as u128 / frequency as u128) as u64,
    }
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn counter_frequency() -> u64 {
    let frequency: u64;
    unsafe {
        core::arch::asm!("mrs {}, cntfrq_el0", out(reg) frequency, options(pure, nomem, nostack, preserves_flags));
    }
    frequency
}

#[cfg(target_arch = "aarch64")]
#[inline]
fn counter_ns() -> u64 {
    let count: u64;
    unsafe {
        core::arch::asm!("mrs {}, cntvct_el0", out(reg) count, options(nostack, preserves_flags));
    }
    scale_counter_to_tsc(count, counter_frequency())
}

#[cfg(all(not(target_arch = "aarch64"), target_vendor = "apple"))]
#[inline]
fn counter_ns() -> u64 {
    // The same uptime epoch is available to separately compiled modules under Rosetta.
    const CLOCK_UPTIME_RAW: u32 = 8;
    unsafe extern "C" {
        fn clock_gettime_nsec_np(clock_id: u32) -> u64;
    }
    unsafe { clock_gettime_nsec_np(CLOCK_UPTIME_RAW) }
}

#[cfg(all(not(target_arch = "aarch64"), not(target_vendor = "apple")))]
fn counter_ns() -> u64 {
    static EPOCH: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    EPOCH
        .get_or_init(std::time::Instant::now)
        .elapsed()
        .as_nanos() as u64
}

#[inline]
pub fn read_tsc(ordered: bool, frozen: bool) -> u64 {
    if ordered {
        // RDTSCP waits for older loads to complete before the counter read. The asm
        // also orders compiler memory accesses. This applies in verifier mode too.
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("dsb ishld", "isb", options(nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "aarch64"))]
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    }
    if frozen {
        GUEST_TSC
    } else {
        counter_ns()
    }
}
