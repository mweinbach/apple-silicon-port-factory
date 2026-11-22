//! Import-forwarder aliasing for the small runtime DLLs the game ships (msvcp140,
//! vcruntime140, vcruntime140_1, v8_libbase, v8_libplatform).
//!
//! Many of their exports are nothing but `jmp qword ptr [rip+IAT]`, reached directly or
//! through incremental-link (`E9 rel32`) thunks: v8_libbase `Mutex::Lock` ->
//! `AcquireSRWLockExclusive`, msvcp140 `_Thrd_id` -> `GetCurrentThreadId`, ... Executing
//! one is exactly a transfer to the bound import with the caller's registers and stack.
//! Once the image's imports are bound ([`install`], called by the loader right after
//! binding), every VA of such a chain is registered in the native VA table with the
//! host function its IAT slot resolved to, so callers (guest or native, via IAT, vtable or
//! intra-DLL call) skip the guest hops. Only slots bound to host code qualify, plus the
//! loader's guest `GetCurrentThreadId` leaf stub (same result as the host function).
//! Exports bound to other guest code keep running as is. `AW2_DLL_FWD=0` disables.
//!
//! Difftest (`dll_fwd`): every alias is re-derived from the image bytes, and each alias
//! class with a host-observable effect (getters, SRW/critical-section lock pairs, TLS,
//! condition-variable wakes) is run through the original guest `jmp [IAT]` (emulator) and
//! through the alias, comparing results and memory.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use super::DllDiff;
use crate::exe;

/// Modules whose forwarder exports are aliased (lower-case loader names).
const MODULES: &[&str] = &["msvcp140.dll", "vcruntime140.dll", "vcruntime140_1.dll", "v8_libbase.dll", "v8_libplatform.dll"];

/// Loader guest leaf stub for `GetCurrentThreadId` imports of CRT DLLs (`aw2-loader`
/// `guest_leaf_import_stub`): `mov eax, dword ptr gs:[0x48]; ret` (TEB ClientId.UniqueThread).
const TID_STUB: [u8; 9] = [0x65, 0x8B, 0x04, 0x25, 0x48, 0x00, 0x00, 0x00, 0xC3];

/// One aliased export.
#[derive(Clone, Debug)]
struct Alias {
    module: &'static str,
    base: usize,
    size: usize,
    name: String,
    export_rva: usize,
    /// RVAs of the chain (`E9` hops, then the `jmp [IAT]`), all registered.
    hops: Vec<usize>,
    slot_rva: usize,
    slot_val: u64,
    target: usize,
}

static ALIASES: Mutex<Vec<Alias>> = Mutex::new(Vec::new());

fn enabled() -> bool {
    !std::env::var("AW2_DLL_FWD").is_ok_and(|v| v == "0")
}

#[inline]
unsafe fn r8(a: usize) -> u8 {
    core::ptr::read_unaligned(a as *const u8)
}
#[inline]
unsafe fn r16(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}
#[inline]
unsafe fn r32(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}
#[inline]
unsafe fn ri32(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}
#[inline]
unsafe fn r64(a: usize) -> u64 {
    core::ptr::read_unaligned(a as *const u64)
}

/// Data directory `idx` of the PE32+ image at `[base, base + size)` as `(rva, size)`.
unsafe fn data_dir(base: usize, size: usize, idx: usize) -> Option<(usize, usize)> {
    if size < 0x400 || r16(base) != 0x5A4D {
        return None;
    }
    let nt = r32(base + 0x3C) as usize;
    if nt + 24 + 112 + 8 * 16 > size || r32(base + nt) != 0x0000_4550 || r16(base + nt + 24) != 0x20B {
        return None;
    }
    if idx >= r32(base + nt + 24 + 108) as usize {
        return None;
    }
    let e = base + nt + 24 + 112 + 8 * idx;
    let (rva, len) = (r32(e) as usize, r32(e + 4) as usize);
    (rva != 0 && rva.checked_add(len).is_some_and(|end| end <= size)).then_some((rva, len))
}

/// RVAs of every import address table slot (bound by now).
unsafe fn import_slots(base: usize, size: usize) -> HashSet<usize> {
    let mut out = HashSet::new();
    let Some((rva, len)) = data_dir(base, size, 1) else { return out };
    let mut d = rva;
    while d + 20 <= rva + len.max(20) && d + 20 <= size {
        let (oft, name, ft) = (r32(base + d), r32(base + d + 12), r32(base + d + 16) as usize);
        if oft == 0 && name == 0 && ft == 0 {
            break;
        }
        let mut s = ft;
        while ft != 0 && s + 8 <= size && r64(base + s) != 0 {
            out.insert(s);
            s += 8;
        }
        d += 20;
    }
    out
}

/// `(function rva, name)` of every non-forwarded export (unnamed ones as `#ordinal`).
unsafe fn exports(base: usize, size: usize) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let Some((rva, len)) = data_dir(base, size, 0) else { return out };
    let e = base + rva;
    let (obase, nfun, nnames) = (r32(e + 16), r32(e + 20) as usize, r32(e + 24) as usize);
    let (afun, anames, aord) = (r32(e + 28) as usize, r32(e + 32) as usize, r32(e + 36) as usize);
    if afun + 4 * nfun > size || anames + 4 * nnames > size || aord + 2 * nnames > size {
        return out;
    }
    let mut names: HashMap<usize, String> = HashMap::new();
    for i in 0..nnames {
        let n = r32(base + anames + 4 * i) as usize;
        let o = r16(base + aord + 2 * i) as usize;
        if n < size {
            let s = std::ffi::CStr::from_ptr((base + n) as *const std::ffi::c_char).to_string_lossy().into_owned();
            names.insert(o, s);
        }
    }
    for i in 0..nfun {
        let f = r32(base + afun + 4 * i) as usize;
        if f == 0 || f >= size || (rva..rva + len).contains(&f) {
            continue;
        }
        out.push((f, names.remove(&i).unwrap_or_else(|| format!("#{}", obase as usize + i))));
    }
    out
}

/// Follow `E9` hops from `rva` to a `jmp [rip+disp32]` (`FF 25` / `48 FF 25`): returns the
/// chain's RVAs and the slot RVA.
unsafe fn forwarder_chain(base: usize, size: usize, mut rva: usize) -> Option<(Vec<usize>, usize)> {
    let mut hops = Vec::new();
    for _ in 0..4 {
        if rva + 7 > size {
            return None;
        }
        let p = base + rva;
        if r8(p) == 0xE9 {
            hops.push(rva);
            let t = rva as i64 + 5 + ri32(p + 1) as i64;
            if t < 0 || t as usize >= size {
                return None;
            }
            rva = t as usize;
            continue;
        }
        let (len, disp_at) = if r16(p) == 0x25FF {
            (6, p + 2)
        } else if r8(p) == 0x48 && r16(p + 1) == 0x25FF {
            (7, p + 3)
        } else {
            return None;
        };
        hops.push(rva);
        let s = rva as i64 + len + ri32(disp_at) as i64;
        return (s >= 0 && (s as usize) + 8 <= size).then_some((hops, s as usize));
    }
    None
}

/// Host function a bound import value stands for: host code (or a VA registered native),
/// or the loader's guest `GetCurrentThreadId` stub.
unsafe fn host_target(v: u64) -> Option<usize> {
    if let Some(h) = crate::emu::resolve_host_callable(v) {
        return Some(h);
    }
    if (0x1_0000..1u64 << 47).contains(&v) && core::ptr::read_unaligned(v as usize as *const [u8; 9]) == TID_STUB {
        return Some(win32_rt::kernel32::threads::GetCurrentThreadId as *const () as usize);
    }
    None
}

/// The aliases of module `name` mapped at `[base, base + size)` (imports bound).
unsafe fn scan(name: &'static str, base: usize, size: usize) -> (Vec<Alias>, usize) {
    let slots = import_slots(base, size);
    let mut out = Vec::new();
    let mut guest = 0;
    for (rva, ename) in exports(base, size) {
        let Some((hops, slot_rva)) = forwarder_chain(base, size, rva) else { continue };
        if !slots.contains(&slot_rva) {
            continue;
        }
        let slot_val = r64(base + slot_rva);
        let Some(target) = host_target(slot_val) else {
            guest += 1;
            continue;
        };
        out.push(Alias { module: name, base, size, name: ename, export_rva: rva, hops, slot_rva, slot_val, target });
    }
    (out, guest)
}

/// Register the forwarder aliases of DLL `name` (lower-case) mapped at `[base, base + size)`
/// once its imports are bound. Returns the number of aliased exports.
pub unsafe fn install(name: &str, base: usize, size: usize) -> usize {
    let Some(&module) = MODULES.iter().find(|m| **m == name) else { return 0 };
    if !enabled() {
        return 0;
    }
    let (aliases, guest) = scan(module, base, size);
    let mut regs: HashMap<u64, usize> = HashMap::new();
    let mut conflicts = 0;
    for a in &aliases {
        for &r in &a.hops {
            let va = (base + r) as u64;
            match exe::lookup_va_only(va) {
                None => {
                    regs.insert(va, a.target);
                }
                Some(h) if h == a.target => {}
                Some(_) => conflicts += 1,
            }
        }
    }
    let nregs = regs.len();
    exe::register_native_vas(regs);
    log::info!(
        "dll_native: {name}: {} import-forwarder export(s) aliased to their bound targets ({nregs} VAs; {guest} bound to guest code kept; {conflicts} hop(s) already native elsewhere)",
        aliases.len()
    );
    let n = aliases.len();
    if let Ok(mut v) = ALIASES.lock() {
        v.extend(aliases);
    }
    n
}

// ---------------------------------------------------------------------------------------------
// Difftest
// ---------------------------------------------------------------------------------------------

/// One `dll_fwd` difftest over every alias (empty if nothing was aliased).
pub(crate) fn difftests() -> Vec<DllDiff> {
    let v = ALIASES.lock().map(|v| v.clone()).unwrap_or_default();
    let Some(a) = v.first() else { return Vec::new() };
    let last = a.base + *a.hops.last().unwrap();
    vec![DllDiff { name: "dll_fwd::import_forwarders", addr: last, new: a.target, diff: diff_forwarders }]
}

type F0 = unsafe extern "win64" fn() -> u64;
type F1 = unsafe extern "win64" fn(u64) -> u64;
type F2 = unsafe extern "win64" fn(u64, u64) -> u64;

/// `orig` = the harness's tagged address of the first alias; the tag is the bit it added.
pub unsafe fn diff_forwarders(orig: usize, _new: usize) -> Result<u32, String> {
    use win32_rt::{kernel32::process as kp, kernel32::threads as kt, sync};
    let tag = orig & (1usize << 46);
    let aliases = ALIASES.lock().map(|v| v.clone()).unwrap_or_default();
    let fp = |f: *const ()| f as usize;
    let (acq_x, rel_x) = (fp(sync::AcquireSRWLockExclusive as *const ()), fp(sync::ReleaseSRWLockExclusive as *const ()));
    let (acq_s, rel_s) = (fp(sync::AcquireSRWLockShared as *const ()), fp(sync::ReleaseSRWLockShared as *const ()));
    let (enter, leave) = (fp(sync::EnterCriticalSection as *const ()), fp(sync::LeaveCriticalSection as *const ()));
    let (wake1, wake_all) = (fp(sync::WakeConditionVariable as *const ()), fp(sync::WakeAllConditionVariable as *const ()));
    let (tid, pid, last_err) =
        (fp(kt::GetCurrentThreadId as *const ()), fp(kp::GetCurrentProcessId as *const ()), fp(kp::GetLastError as *const ()));
    let (tls_get, tls_set) = (fp(kt::TlsGetValue as *const ()), fp(kt::TlsSetValue as *const ()));
    let mut cases = 0u32;
    for a in &aliases {
        let what = format!("{}!{} (+0x{:x})", a.module, a.name, a.export_rva);
        // 1. Structure: the chain still decodes to the same slot, the slot still holds the
        //    bound value, it still maps to the target, and every hop dispatches to it.
        let base = a.base;
        match forwarder_chain(base, a.size, a.export_rva) {
            Some((hops, slot)) if hops == a.hops && slot == a.slot_rva => {}
            other => return Err(format!("{what}: chain changed: {other:x?} vs {:x?} slot 0x{:x}", a.hops, a.slot_rva)),
        }
        let v = r64(base + a.slot_rva);
        if v != a.slot_val || host_target(v) != Some(a.target) {
            return Err(format!("{what}: slot 0x{:x} now 0x{v:x} (bound 0x{:x}) -> {:x?}, alias 0x{:x}", a.slot_rva, a.slot_val, host_target(v), a.target));
        }
        for &r in &a.hops {
            if exe::lookup_va_only((base + r) as u64) != Some(a.target) {
                return Err(format!("{what}: hop +0x{r:x} not dispatched to the alias"));
            }
        }
        cases += 1;
        // 2. Behaviour: the guest `jmp [IAT]` (emulated) vs the alias, where observable.
        let jmp = base + *a.hops.last().unwrap();
        let o0: F0 = core::mem::transmute(jmp | tag);
        let o1: F1 = core::mem::transmute(jmp | tag);
        let o2: F2 = core::mem::transmute(jmp | tag);
        let n0: F0 = core::mem::transmute(a.target);
        let n1: F1 = core::mem::transmute(a.target);
        let n2: F2 = core::mem::transmute(a.target);
        let t = a.target;
        if t == tid || t == pid {
            let (x, y) = (o0() as u32, n0() as u32);
            if x != y {
                return Err(format!("{what}: guest {x:#x} != alias {y:#x}"));
            }
        } else if t == last_err {
            kp::SetLastError(0x0BAD_C0DE);
            let x = o0() as u32;
            kp::SetLastError(0x0BAD_C0DE);
            let y = n0() as u32;
            if x != y {
                return Err(format!("{what}: guest {x:#x} != alias {y:#x}"));
            }
        } else if [acq_x, rel_x, acq_s, rel_s].contains(&t) {
            // Lock/unlock through the guest export, undo with the host function, then the
            // same through the alias: identical SRW words after each step.
            let w = Box::into_raw(Box::new(0u64));
            let p = w as u64;
            let (pre, post): (usize, usize) = match t {
                x if x == acq_x => (0, rel_x),
                x if x == rel_x => (acq_x, 0),
                x if x == acq_s => (0, rel_s),
                _ => (acq_s, 0),
            };
            let run = |f: F1| -> u64 {
                if pre != 0 {
                    (core::mem::transmute::<usize, F1>(pre))(p);
                }
                f(p);
                let s = w.read_volatile();
                if post != 0 {
                    (core::mem::transmute::<usize, F1>(post))(p);
                }
                s
            };
            let (x, y) = (run(o1), run(n1));
            let fin = *Box::from_raw(w);
            if x != y || fin != 0 {
                return Err(format!("{what}: SRW word guest {x:#x} alias {y:#x} final {fin:#x}"));
            }
        } else if t == enter || t == leave {
            let cs = Box::into_raw(Box::new([0u64; 8]));
            let p = cs as u64;
            sync::InitializeCriticalSection(p as *mut _);
            let run = |f: F1| -> [u64; 8] {
                if t == leave {
                    sync::EnterCriticalSection(p as *mut _);
                }
                f(p);
                let s = cs.read_volatile();
                if t == enter {
                    sync::LeaveCriticalSection(p as *mut _);
                }
                s
            };
            let (x, y) = (run(o1), run(n1));
            sync::DeleteCriticalSection(p as *mut _);
            drop(Box::from_raw(cs));
            if x != y {
                return Err(format!("{what}: CRITICAL_SECTION guest {x:x?} alias {y:x?}"));
            }
        } else if t == wake1 || t == wake_all {
            let cv = Box::into_raw(Box::new(0u64));
            let p = cv as u64;
            o1(p);
            let x = cv.read_volatile();
            n1(p);
            let y = cv.read_volatile().wrapping_sub(x);
            drop(Box::from_raw(cv));
            if x != y {
                return Err(format!("{what}: CONDITION_VARIABLE word delta guest {x:#x} alias {y:#x}"));
            }
        } else if t == tls_get || t == tls_set {
            let i = kt::TlsAlloc();
            if i == u32::MAX {
                return Err(format!("{what}: TlsAlloc failed"));
            }
            let (x, y) = if t == tls_get {
                kt::TlsSetValue(i, 0x1234_5678 as *mut _);
                (o1(i as u64), n1(i as u64))
            } else {
                let rx = o2(i as u64, 0x55);
                let gx = kt::TlsGetValue(i) as u64;
                let ry = n2(i as u64, 0x66);
                let gy = kt::TlsGetValue(i) as u64;
                ((rx as u32 as u64) << 32 | (gx ^ 0x55), (ry as u32 as u64) << 32 | (gy ^ 0x66))
            };
            kt::TlsFree(i);
            if x != y {
                return Err(format!("{what}: guest {x:#x} != alias {y:#x}"));
            }
        } else {
            continue;
        }
        cases += 1;
    }
    Ok(cases)
}
