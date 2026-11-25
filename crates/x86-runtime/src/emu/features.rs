//! Optional ARM instructions used by the translator. A missing Darwin sysctl is a negative probe.

#[derive(Clone, Copy, Default)]
pub(super) struct HostFeatures(u8);

impl HostFeatures {
    pub const AES: u8 = 1 << 0;
    pub const PMULL: u8 = 1 << 1;
    pub const SHA1: u8 = 1 << 2;
    pub const SHA256: u8 = 1 << 3;
    pub const CRC32: u8 = 1 << 4;
    pub const LSE2: u8 = 1 << 5;
    pub const LRCPC2: u8 = 1 << 6;
    pub const AFP: u8 = 1 << 7;

    #[inline]
    pub fn has(self, features: u8) -> bool {
        self.0 & features == features
    }

    // Keep these bits recoverable from the disk cache when embedding a static Mach-O descriptor.
    #[cfg(target_arch = "aarch64")]
    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn can_run(self, knobs: u64) -> bool {
        self.has((knobs >> 19) as u8)
    }
}

#[inline]
pub(super) fn host() -> HostFeatures {
    #[cfg(test)]
    if let Some(features) = TEST_HOST.with(|v| v.get()) {
        return features;
    }
    static HOST: std::sync::OnceLock<HostFeatures> = std::sync::OnceLock::new();
    *HOST.get_or_init(detect)
}

#[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
fn detect() -> HostFeatures {
    let mut bits = 0;
    for (flag, name) in [
        (HostFeatures::AES, c"hw.optional.arm.FEAT_AES"),
        (HostFeatures::PMULL, c"hw.optional.arm.FEAT_PMULL"),
        (HostFeatures::SHA1, c"hw.optional.arm.FEAT_SHA1"),
        (HostFeatures::SHA256, c"hw.optional.arm.FEAT_SHA256"),
        (HostFeatures::CRC32, c"hw.optional.arm.FEAT_CRC32"),
        (HostFeatures::LSE2, c"hw.optional.arm.FEAT_LSE2"),
        (HostFeatures::LRCPC2, c"hw.optional.arm.FEAT_LRCPC2"),
        (HostFeatures::AFP, c"hw.optional.arm.FEAT_AFP"),
    ] {
        if super::tsc::darwin_feature(name) {
            bits |= flag;
        }
    }
    HostFeatures(bits)
}

#[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
fn detect() -> HostFeatures {
    HostFeatures::default()
}

#[cfg(test)]
thread_local! {
    static TEST_HOST: std::cell::Cell<Option<HostFeatures>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(super) fn without_features<R>(mask: u8, run: impl FnOnce() -> R) -> R {
    let reduced = HostFeatures(host().0 & !mask);
    struct Restore(Option<HostFeatures>);
    impl Drop for Restore {
        fn drop(&mut self) {
            TEST_HOST.with(|slot| slot.set(self.0));
        }
    }
    let _restore = Restore(TEST_HOST.with(|slot| slot.replace(Some(reduced))));
    run()
}
