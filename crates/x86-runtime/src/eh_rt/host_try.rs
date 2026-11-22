//! Host-side `catch (...)` for MSVC C++ exceptions: [`catch_cxx`] runs a closure (guest or
//! lifted code) so that a C++ exception leaving it is caught at the closure's boundary, after
//! the cleanup (destructors) of every frame between the throw and the boundary ran, exactly
//! as a `catch (...)` in a native caller would see it.
//!
//! The boundary is an [`EhHostFrame`] with a landing. Its handler (dispatch phase) records
//! the exception, then calls `RtlUnwindEx` with the record as TargetFrame; the unwind runs
//! the newer frames' handlers and ends in the landing, which jumps to the anchor the closure
//! was entered under. The per-thread state of the call ([`Snapshot`]) is then restored.
//!
//! Frames on the path without a record (a Rust API calling guest code, an opaque emulator
//! entry) stop the dispatch before the boundary, as Windows stops at a frame without unwind
//! info; the exception then takes the unhandled path (`win32_rt::unwind::CXX_THROW_HOOK`).
//!
//! Used by the dll_lift difftest child for both runs (guest and lifted), so a function under
//! test that throws is compared by what it threw and what its unwinding left behind.
use super::anchor::{self, Anchor, Snapshot};
use win32_rt::unwind::{
    Context, DispatcherContext, EhHostFrame, EhLink, ExceptionRecord, CXX_EXCEPTION, EXCEPTION_EXIT_UNWIND,
    EXCEPTION_UNWINDING,
};
use core::mem::MaybeUninit;

/// A C++ exception caught by [`catch_cxx`].
#[derive(Clone, Copy)]
pub struct Caught {
    /// The exception record as the boundary saw it (dispatch phase).
    pub rec: ExceptionRecord,
    /// What the caller's `inspect` function returned for it, in the dispatch phase (the
    /// thrower's frames, and the exception object in them, still intact).
    pub note: [u64; 2],
}

/// State of one [`catch_cxx`] call. `repr(C)` with the frame record first: the handler finds
/// it from EstablisherFrame (= the record's address).
#[repr(C)]
struct HostTry<'a> {
    frame: EhHostFrame,
    anchor: MaybeUninit<Anchor>,
    snap: MaybeUninit<Snapshot>,
    /// Dispatch phase, C++ exception: `Some(note)` claims it (unwind to the boundary), `None`
    /// lets it pass on to the frames above.
    claim: &'a mut dyn FnMut(&ExceptionRecord) -> Option<[u64; 2]>,
    /// Exception code the boundary considers (`CXX_EXCEPTION` for [`catch_cxx_if`]).
    code: u32,
    caught: Option<Caught>,
}

/// Run `f`; `Err` if an MSVC C++ exception left it (see the module doc). `inspect` runs in
/// the dispatch phase, before any unwinding, and its result is kept in [`Caught::note`].
/// Exceptions of other codes pass through the boundary unhandled.
#[inline(never)]
pub unsafe fn catch_cxx<R, F: FnOnce() -> R>(inspect: unsafe fn(&ExceptionRecord) -> [u64; 2], f: F) -> Result<R, Caught> {
    unsafe { catch_cxx_if(&mut |rec| Some(inspect(rec)), f) }
}

/// [`catch_cxx`] for the C++ exceptions `claim` accepts: it runs in the dispatch phase (the
/// thrower's frames intact) and returns `Some(note)` to catch the exception here, or `None` to
/// let it pass to the frames above the boundary, as an MSVC `catch (T)` that does not match.
/// `exe::try_call` on aarch64 (catch lists of decorated type names).
#[inline(never)]
pub unsafe fn catch_cxx_if<R, F: FnOnce() -> R>(claim: &mut dyn FnMut(&ExceptionRecord) -> Option<[u64; 2]>, f: F) -> Result<R, Caught> {
    unsafe { catch_code_if(CXX_EXCEPTION, claim, f) }
}

/// [`catch_cxx_if`] for exceptions of `code` (any code a raise or a hardware fault dispatches,
/// e.g. `STATUS_STACK_OVERFLOW` from the lifted stack account): `claim` sees each one in the
/// dispatch phase; `Some(note)` catches it at this boundary.
#[inline(never)]
pub unsafe fn catch_code_if<R, F: FnOnce() -> R>(
    code: u32,
    claim: &mut dyn FnMut(&ExceptionRecord) -> Option<[u64; 2]>,
    f: F,
) -> Result<R, Caught> {
    super::chain::ensure_hooks();
    let mut st = HostTry {
        frame: EhHostFrame::new(handler, core::ptr::null_mut()).with_land(land),
        anchor: MaybeUninit::uninit(),
        snap: MaybeUninit::uninit(),
        claim,
        code,
        caught: None,
    };
    let sp: *mut HostTry = &mut st;
    unsafe {
        let _g = (*sp).frame.push();
        // After the push: a landing restores this record as the innermost one, and the guard
        // above pops it as usual.
        (*sp).snap.write(Snapshot::take());
        let mut landed = 0;
        let r = anchor::enter_with((*sp).anchor.as_mut_ptr(), f, &mut landed);
        match r {
            Some(v) => Ok(v),
            None => {
                (*sp).snap.assume_init_ref().restore();
                Err(core::ptr::read_volatile(&(*sp).caught).expect("catch_cxx: landed without a caught exception"))
            }
        }
    }
}

/// The boundary's language handler. Dispatch phase, exception of the boundary's code the claim accepts: record
/// it and unwind to the boundary (does not return). Anything else: continue the search.
unsafe extern "win64" fn handler(rec: *mut ExceptionRecord, frame: u64, ctx: *mut Context, dc: *mut DispatcherContext) -> u32 {
    let _ = dc;
    unsafe {
        let st = frame as *mut HostTry<'static>;
        if (*rec).flags & (EXCEPTION_UNWINDING | EXCEPTION_EXIT_UNWIND) != 0 || (*rec).code != (*st).code {
            return 1; // ExceptionContinueSearch
        }
        let Some(note) = ((*st).claim)(&*rec) else { return 1 };
        (*st).caught = Some(Caught { rec: *rec, note });
        #[cfg(target_arch = "x86_64")]
        {
            type RtlUnwindExFn = unsafe extern "win64" fn(u64, u64, *mut ExceptionRecord, u64, *mut Context, *mut core::ffi::c_void);
            let unwind: RtlUnwindExFn = core::mem::transmute(win32_rt::unwind::RtlUnwindEx as *const ());
            unwind(frame, 0, rec, 0, ctx, core::ptr::null_mut());
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = win32_rt::unwind::RtlUnwindEx(frame, 0, rec as u64, 0, ctx as u64, 0);
        }
        log::error!(target: "win32_rt::seh", "catch_cxx: RtlUnwindEx to the boundary returned");
        std::process::abort()
    }
}

/// The boundary's landing (end of the unwind to it): back to [`catch_cxx`]'s anchor.
unsafe fn land(r: *mut EhLink, _ctx: &Context) -> ! {
    unsafe { anchor::jump((*(r as *mut HostTry<'static>)).anchor.as_ptr(), 1) }
}

// `handler` / `land` cast the record address to `HostTry`.
const _: () = assert!(core::mem::offset_of!(HostTry<'static>, frame) == 0);
