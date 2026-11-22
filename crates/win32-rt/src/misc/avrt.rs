//! avrt.dll: MMCSS (Multimedia Class Scheduler Service) on Mach scheduling.
//! Imported by xaudio2_9redist.dll (its mixing thread joins "Pro Audio"/
//! "Audio"). Audio tasks become Mach time-constraint threads (10 ms period —
//! the WASAPI shared-mode period XAudio2 processes at — 5 ms computation,
//! 10 ms constraint; same scheme as `audioclient::promote_render_thread`);
//! the other stock MMCSS tasks get a precedence boost. Reverting restores
//! the standard policy and the thread's Win32 priority. Only plain mach calls
//! (`thread::sched`), so this is safe on Win32 threads.
//! `AW2_AUDIO_RT=0` turns audio tasks into a plain boost too.

use crate::strings::*;
use crate::thread::sched;
use crate::types::*;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_INVALID_TASK_NAME: u32 = 1550;

const RT_PERIOD_NS: u64 = 10_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// Mach time-constraint thread.
    Realtime,
    /// Precedence importance of this Win32 priority (+ AvSetMmThreadPriority).
    Boost(i32),
}

/// Stock task names (HKLM\...\Multimedia\SystemProfile\Tasks); others fail
/// with ERROR_INVALID_TASK_NAME like an unregistered task on Windows.
pub fn task_class(name: &str) -> Option<Class> {
    match name.trim().to_ascii_lowercase().as_str() {
        "audio" | "pro audio" => Some(Class::Realtime),
        "capture" | "distribution" | "games" | "playback" => Some(Class::Boost(2)), // HIGHEST
        "window manager" => Some(Class::Boost(1)),                                 // ABOVE_NORMAL
        _ => None,
    }
}

struct Reg {
    handle: usize,
    port: u32,
    tid: usize,
    task: String,
    class: Class,
}

static REGS: Mutex<Vec<Reg>> = Mutex::new(Vec::new());
static NEXT_HANDLE: AtomicUsize = AtomicUsize::new(0);
static NEXT_TASK_INDEX: AtomicU32 = AtomicU32::new(1);

fn rt_disabled() -> bool {
    std::env::var("AW2_AUDIO_RT").is_ok_and(|v| v == "0")
}

fn current_win_priority() -> i32 {
    crate::thread::current_thread_object().map(|t| t.priority.load(Ordering::Relaxed)).unwrap_or(0)
}

fn apply(port: u32, class: Class, avrt_prio: i32) -> Result<(), i32> {
    match class {
        Class::Realtime => {
            let tc = sched::time_constraint_for(RT_PERIOD_NS, RT_PERIOD_NS / 2, RT_PERIOD_NS);
            sched::set_time_constraint(port, &tc)
        }
        Class::Boost(p) => sched::set_importance(port, sched::importance_for((p + avrt_prio).clamp(-15, 15))),
    }
}

unsafe fn set_task(task: &str, index: *mut u32) -> HANDLE {
    let Some(mut class) = task_class(task) else {
        log::warn!(target: "audio", "AvSetMmThreadCharacteristics({task:?}): unknown MMCSS task -> ERROR_INVALID_TASK_NAME");
        crate::teb::set_last_error(ERROR_INVALID_TASK_NAME);
        return std::ptr::null_mut();
    };
    if class == Class::Realtime && rt_disabled() {
        class = Class::Boost(2);
    }
    if !index.is_null() && *index == 0 {
        *index = NEXT_TASK_INDEX.fetch_add(1, Ordering::Relaxed);
    }
    let port = sched::current_port();
    let tid = crate::thread::current_thread_object().map(|t| t.tid.load(Ordering::Relaxed)).unwrap_or(0);
    match apply(port, class, 0) {
        Ok(()) => log::info!(target: "audio", "AvSetMmThreadCharacteristics({task:?}) on tid 0x{tid:x} -> {class:?}{}", if class == Class::Realtime { " (Mach time-constraint 10/5/10 ms)" } else { "" }),
        // Still succeed: the thread runs, just without the boost.
        Err(kr) => log::warn!(target: "audio", "AvSetMmThreadCharacteristics({task:?}) on tid 0x{tid:x}: thread_policy_set failed (kern_return {kr})"),
    }
    let handle = 0x4176_7274_0000usize + 4 * (NEXT_HANDLE.fetch_add(1, Ordering::Relaxed) + 1);
    REGS.lock().unwrap().push(Reg { handle, port, tid, task: task.to_string(), class });
    handle as HANDLE
}

pub unsafe extern "win64" fn AvSetMmThreadCharacteristicsA(task: LPCSTR, index: *mut u32) -> HANDLE {
    if task.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    set_task(&from_ansi(task), index)
}

pub unsafe extern "win64" fn AvSetMmThreadCharacteristicsW(task: LPCWSTR, index: *mut u32) -> HANDLE {
    if task.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    set_task(&from_wide(task), index)
}

/// Joins the first task (we have no per-task capacity limits to arbitrate).
pub unsafe extern "win64" fn AvSetMmMaxThreadCharacteristicsA(first: LPCSTR, _second: LPCSTR, index: *mut u32) -> HANDLE {
    AvSetMmThreadCharacteristicsA(first, index)
}

pub unsafe extern "win64" fn AvSetMmMaxThreadCharacteristicsW(first: LPCWSTR, _second: LPCWSTR, index: *mut u32) -> HANDLE {
    AvSetMmThreadCharacteristicsW(first, index)
}

pub unsafe extern "win64" fn AvRevertMmThreadCharacteristics(h: HANDLE) -> BOOL {
    let reg = {
        let mut g = REGS.lock().unwrap();
        g.iter().position(|r| r.handle == h as usize).map(|i| g.remove(i))
    };
    let Some(r) = reg else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if r.class == Class::Realtime {
        if let Err(kr) = sched::set_standard_policy(r.port) {
            log::warn!(target: "audio", "AvRevertMmThreadCharacteristics: standard policy failed (kern_return {kr})");
        }
    }
    let _ = sched::set_importance(r.port, sched::importance_for(current_win_priority()));
    log::debug!(target: "audio", "AvRevertMmThreadCharacteristics({:?}, tid 0x{:x})", r.task, r.tid);
    TRUE
}

/// AVRT_PRIORITY_VERYLOW -2 .. AVRT_PRIORITY_CRITICAL 2: shifts a boosted
/// task's importance; realtime tasks keep their time constraint.
pub unsafe extern "win64" fn AvSetMmThreadPriority(h: HANDLE, prio: i32) -> BOOL {
    if !(-2..=2).contains(&prio) {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let g = REGS.lock().unwrap();
    let Some(r) = g.iter().find(|r| r.handle == h as usize) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if let Class::Boost(_) = r.class {
        let _ = apply(r.port, r.class, prio);
    }
    log::debug!(target: "audio", "AvSetMmThreadPriority({:?}, {prio}) on tid 0x{:x}", r.task, r.tid);
    TRUE
}

/// Windows' default SystemResponsiveness (percent of CPU kept for non-MMCSS work).
pub unsafe extern "win64" fn AvQuerySystemResponsiveness(h: HANDLE, out: *mut u32) -> BOOL {
    if out.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if !REGS.lock().unwrap().iter().any(|r| r.handle == h as usize) {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    }
    *out = 20;
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_names() {
        assert_eq!(task_class("Pro Audio"), Some(Class::Realtime));
        assert_eq!(task_class("audio"), Some(Class::Realtime));
        assert_eq!(task_class("Games"), Some(Class::Boost(2)));
        assert_eq!(task_class("Nope"), None);
    }

    #[test]
    fn pro_audio_promotes_and_reverts() {
        std::thread::spawn(|| unsafe {
            let port = sched::current_port();
            let mut idx = 0u32;
            let name = b"Pro Audio\0";
            let h = AvSetMmThreadCharacteristicsA(name.as_ptr() as LPCSTR, &mut idx);
            assert!(!h.is_null());
            assert_ne!(idx, 0);
            if !rt_disabled() {
                let tc = sched::time_constraint(port).expect("realtime after AvSetMmThreadCharacteristics");
                assert_eq!(tc.preemptible, 1);
            }
            assert_eq!(AvSetMmThreadPriority(h, 1), TRUE);
            assert_eq!(AvSetMmThreadPriority(h, 7), FALSE);
            let mut resp = 0u32;
            assert_eq!(AvQuerySystemResponsiveness(h, &mut resp), TRUE);
            assert_eq!(resp, 20);
            assert_eq!(AvRevertMmThreadCharacteristics(h), TRUE);
            assert!(sched::time_constraint(port).is_none(), "still realtime after revert");
            assert_eq!(AvRevertMmThreadCharacteristics(h), FALSE);
            let bad = b"Nope\0";
            assert!(AvSetMmThreadCharacteristicsA(bad.as_ptr() as LPCSTR, &mut idx).is_null());
        })
        .join()
        .unwrap();
    }
}
