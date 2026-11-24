//! Tests for the A64 emitter and JIT cache.

mod opcode_encodings;

use super::cache::{jslot, MAX_LSE2_REGIONS, NOJIT};
use super::*;
use crate::emu::exec_mem::{
    bind_static_aot_module, jit_writable, register_static_aot_desc, register_static_aot_module,
    static_aot_gen, test_reset_static_lookups, test_static_lookups,
    unregister_static_aot_for_test, StaticAotEntry, StaticAotModuleDesc,
    STATIC_AOT_DESC_MAGIC, STATIC_AOT_DESC_VERSION, STATIC_AOT_FLAG_HOST_CALLABLE,
};
use crate::emu::BlockCache;

#[test]
fn static_only_negative_cache_and_rebind_invalidation() {
    let entries_v1 = [
        StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
        StaticAotEntry { rva: 0x2000, code_offset: 0x200, flags: STATIC_AOT_FLAG_HOST_CALLABLE },
    ];
    register_static_aot_module("jit_neg_test.dll", 0x7_0000_0000, &entries_v1);
    assert_eq!(bind_static_aot_module("jit_neg_test.dll", 0x3_0000_0000, 0x1_0000), 2);

    let mut bc = Box::new(BlockCache::new());
    let mut jc = JitCache::new();
    jc.test_set_static_only(true);
    assert!(jc.meta.is_empty(), "static_only must skip the 4 MiB meta slab");
    let m_static = jc.mode_tag();
    assert_ne!(m_static, 0);

    let hit_ip = 0x3_0000_1000u64;
    let host_ip = 0x3_0000_2000u64;
    let miss_ip = 0x3_0000_3000u64;

    unsafe {
        test_reset_static_lookups();
        // Positive hit populates index; repeat lookups do not hit registry again.
        assert_eq!(jc.lookup(hit_ip, &mut *bc), Some(0x7_0000_0100));
        assert_eq!(test_static_lookups(), 1);
        assert_eq!(jc.lookup(hit_ip, &mut *bc), Some(0x7_0000_0100));
        assert_eq!(test_static_lookups(), 1);
        assert!(!jc.is_interp_sticky(hit_ip));

        // Host-callable entry caches code != 0 with fast == 0.
        assert_eq!(jc.lookup(host_ip, &mut *bc), Some(0x7_0000_0200));
        assert_eq!(test_static_lookups(), 2);
        let h_ent = jc.index[jslot(host_ip)];
        assert_eq!(h_ent.ip, host_ip);
        assert_eq!(h_ent.code, 0x7_0000_0200);
        assert_eq!(h_ent.fast, 0);

        // Miss populates a NOJIT negative entry; repeated lookups MUST NOT re-query registry.
        assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
        assert_eq!(test_static_lookups(), 3);
        let neg = jc.index[jslot(miss_ip)];
        assert_eq!(neg.ip, miss_ip);
        assert_eq!(neg.code, 0);
        assert_eq!(neg.fast, 0);
        assert_eq!(neg.hits, NOJIT);
        assert!(jc.is_interp_sticky(miss_ip));

        for _ in 0..16 {
            assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
        }
        assert_eq!(
            test_static_lookups(),
            3,
            "negative cache must eliminate repeated lookup_static_aot calls"
        );

        // Rebinding a new module containing `miss_ip` bumps STATIC_AOT_GEN, invalidates
        // `is_interp_sticky`, flushes the negative cache, and resolves the newly bound block.
        let g_before = static_aot_gen();
        let entries_v2 = [
            StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
            StaticAotEntry { rva: 0x3000, code_offset: 0x300, flags: 0 },
        ];
        register_static_aot_module("jit_neg_test.dll", 0x7_1000_0000, &entries_v2);
        assert_eq!(bind_static_aot_module("jit_neg_test.dll", 0x3_0000_0000, 0x1_0000), 2);
        assert_ne!(static_aot_gen(), g_before);
        assert!(!jc.is_interp_sticky(miss_ip));

        assert_eq!(jc.lookup(miss_ip, &mut *bc), Some(0x7_1000_0300));
        assert_eq!(test_static_lookups(), 4);
        assert!(!jc.is_interp_sticky(miss_ip));

        // Unbinding bumps generation and flushes cached entries back to None.
        unregister_static_aot_for_test("jit_neg_test.dll");
        assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
        assert_eq!(test_static_lookups(), 5);
        assert!(jc.is_interp_sticky(miss_ip));
    }
}

#[test]
fn no_jit_transitions_to_static_only_on_late_bind_and_skips_meta() {
    let mut bc = Box::new(BlockCache::new());
    let mut jc = JitCache::new();
    // Simulate `!jit_writable()` with no static AOT module bound yet.
    jc.no_jit = true;
    jc.static_aot_gen = static_aot_gen();
    let mode_before = jc.mode_tag();

    let ip = 0x3_8000_1000u64;
    assert!(jc.is_interp_sticky(ip));
    unsafe {
        test_reset_static_lookups();
        // Steady-state no_jit without static modules must not touch static registry or init().
        for _ in 0..8 {
            assert_eq!(jc.lookup(ip, &mut *bc), None);
        }
        assert_eq!(test_static_lookups(), 0);
        assert!(jc.mem.is_null());
        assert!(!jc.static_only);
    }

    // Bind a static module with a signed enter/lookup_rip stub descriptor.
    let name = std::ffi::CString::new("late_bind.dll").unwrap();
    let entries = [StaticAotEntry { rva: 0x1000, code_offset: 0x800, flags: 0 }];
    let desc = StaticAotModuleDesc {
        magic: STATIC_AOT_DESC_MAGIC,
        version: STATIC_AOT_DESC_VERSION,
        knobs: 0,
        image_base: 0x3_8000_0000,
        image_size: 0x1_0000,
        image_hash: [1, 2],
        code_base: 0x8_0000_0000,
        code_len: 0x4000,
        enter_off: 0x40,
        lookup_rip_off: 0x100,
        flags: 0,
        entry_count: 1,
        _pad: 0,
        entries: entries.as_ptr(),
        module_name: name.as_ptr(),
    };
    assert!(unsafe { register_static_aot_desc(&desc) });
    assert_eq!(bind_static_aot_module("late_bind.dll", 0x3_8000_0000, 0x1_0000), 1);

    // Generation bumped: `is_interp_sticky` drops immediately, and `lookup` transitions
    // `JitCache` into `static_only` without allocating `self.meta`.
    assert!(!jc.is_interp_sticky(ip));
    unsafe {
        assert_eq!(jc.lookup(ip, &mut *bc), Some(0x8_0000_0800));
        assert!(jc.static_only);
        assert!(jc.meta.is_empty());
        assert_eq!(jc.enter, 0x8_0000_0040);
        assert_eq!(jc.lookup_rip, 0x8_0000_0100);
        assert_ne!(jc.mode_tag(), mode_before);
    }

    // Unbinding drops back out of `static_only` cleanly.
    unregister_static_aot_for_test("late_bind.dll");
    unsafe {
        assert_eq!(jc.lookup(ip, &mut *bc), None);
        assert!(!jc.static_only);
        assert_eq!(jc.enter, 0);
        assert!(jc.is_interp_sticky(ip));
    }
}

#[test]
fn lse2_regions_recycle_and_jit_cache_drop_reclaims_slots() {
    // 1. Direct register/unregister cycle well past MAX_LSE2_REGIONS (128).
    let mut first_batch = Vec::new();
    for i in 0..8usize {
        let base = 0x5_0000_0000usize + i * 0x10_0000;
        let slot = register_lse2_region(base, base + 0x8_0000, base + 0x8_0000, base + 0xC_0000)
            .expect("initial LSE2 slot available");
        first_batch.push(slot);
    }
    for &slot in &first_batch {
        unregister_lse2_region(slot);
    }
    // Repeatedly register and unregister more times than MAX_LSE2_REGIONS; every iteration must succeed.
    for i in 0..(MAX_LSE2_REGIONS + 64) {
        let base = 0x5_1000_0000usize + (i & 0xFF) * 0x10_0000;
        let slot = register_lse2_region(base, base + 0x8_0000, base + 0x8_0000, base + 0xC_0000)
            .expect("LSE2 slot must be recycled after unregister");
        unregister_lse2_region(slot);
    }
    // Re-claiming 8 slots must reuse slots within `0..MAX_LSE2_REGIONS`.
    let mut second_batch = Vec::new();
    for i in 0..8usize {
        let base = 0x5_2000_0000usize + i * 0x10_0000;
        let slot = register_lse2_region(base, base + 0x8_0000, base + 0x8_0000, base + 0xC_0000)
            .expect("recycled batch slot available");
        assert!(slot < MAX_LSE2_REGIONS);
        second_batch.push(slot);
    }
    for &slot in &second_batch {
        unregister_lse2_region(slot);
    }

    // 2. Creating, warming, and dropping > MAX_LSE2_REGIONS JitCache instances must reclaim
    //    both MAP_JIT buffers and LSE2_REGIONS slots on every Drop.
    if jit_writable() {
        for iter in 0..(MAX_LSE2_REGIONS + 16) {
            let mut jc = JitCache::new();
            unsafe { jc.warm() };
            assert!(!jc.mem.is_null(), "iter {iter}: MAP_JIT allocation failed (leak?)");
            assert!(
                jc.lse2_slot.is_some(),
                "iter {iter}: LSE2_REGIONS slot exhausted (Drop failed to unregister)"
            );
            drop(jc);
        }
    }
}
