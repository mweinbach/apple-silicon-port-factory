//! `mmdevapi!ActivateAudioInterfaceAsync` (ordinal 17): the WinRT-era way to
//! get an `IAudioClient`. `xaudio2_9redist.dll` (loaded by Bink's
//! `BinkOpenXAudio2`) delay-loads it by ordinal and calls it with the
//! `DEVINTERFACE_AUDIO_RENDER` string, `IID_IAudioClient3`, a `VT_BLOB`
//! activation parameter and a completion handler whose `ActivateCompleted`
//! sets an event it then waits on (xaudio2_9redist.dll+0x16e36).
//!
//! As on Windows, activation completes asynchronously: a short-lived Win32
//! worker thread (TEB, guest-callable) resolves the endpoint, activates the
//! interface through the regular `IMMDevice::Activate` and calls
//! `IActivateAudioInterfaceCompletionHandler::ActivateCompleted(op)`.
//! Activation errors are reported through `GetActivateResult`; the call
//! itself only fails for bad arguments.
//!
//! Device paths: the `DEVINTERFACE_AUDIO_RENDER` GUID string = the default
//! render endpoint (we do not re-route the stream when the default changes; a
//! device change invalidates it like any other client), an endpoint id from
//! `IMMDevice::GetId`, or a `\\?\SWD#MMDEVAPI#<id>#{...}` interface path.
//! Capture and process-loopback paths fail with `E_NOTFOUND` / `E_NOTIMPL`.

use super::com::*;
use super::mmdevapi::{self, Endpoint};
use crate::types::*;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

pub const IID_IActivateAudioInterfaceAsyncOperation: GUID =
    GUID::new(0x72A22D78, 0xCDE4, 0x431D, [0xB8, 0xCC, 0x84, 0x3A, 0x71, 0x19, 0x9B, 0x6D]);
pub const IID_IAgileObject: GUID = GUID::new(0x94EA2B94, 0xE9CC, 0x49E0, [0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90]);
pub const DEVINTERFACE_AUDIO_RENDER: GUID = GUID::new(0xE6327CAD, 0xDCEC, 0x4949, [0xAE, 0x8A, 0x99, 0x1E, 0x97, 0x6A, 0x79, 0xD2]);
pub const DEVINTERFACE_AUDIO_CAPTURE: GUID = GUID::new(0x2EEF81BE, 0x33FA, 0x4800, [0x96, 0x70, 0x1C, 0xD4, 0x74, 0x97, 0x2C, 0x3F]);

/// `E_ILLEGAL_METHOD_CALL`: `GetActivateResult` before completion.
const E_ILLEGAL_METHOD_CALL: HRESULT = 0x8000_000Eu32 as i32;

#[repr(C)]
pub struct OpVtbl {
    qi: unsafe extern "win64" fn(*mut OpObj, *const GUID, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut OpObj) -> u32,
    release: unsafe extern "win64" fn(*mut OpObj) -> u32,
    get_activate_result: unsafe extern "win64" fn(*mut OpObj, *mut HRESULT, *mut *mut c_void) -> HRESULT,
}

/// Completed activation: (result, activated interface or null).
#[derive(Default)]
pub struct OpState {
    done: Mutex<Option<(HRESULT, usize)>>,
}

impl Drop for OpState {
    fn drop(&mut self) {
        if let Some((_, p)) = self.done.get_mut().unwrap().take() {
            if p != 0 {
                unsafe { com_call_u32(p as *mut c_void, 2) };
            }
        }
    }
}

pub type OpObj = ComObj<OpVtbl, OpState>;

/// Call IUnknown::AddRef (slot 1) / Release (slot 2) on a native host COM object.
unsafe fn com_call_u32(obj: *mut c_void, slot: usize) -> u32 {
    let vtbl = *(obj as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vtbl.add(slot));
    f(obj)
}

/// Call IUnknown::AddRef (slot 1) / Release (slot 2) on a guest COM object.
unsafe fn guest_com_call_u32(obj: *mut c_void, slot: usize) -> u32 {
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    let method = std::ptr::read_unaligned(vtbl.add(slot));
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

mod op {
    use super::*;
    crate::com_iunknown!(OpObj, [IID_IActivateAudioInterfaceAsyncOperation, IID_IAgileObject]);

    pub unsafe extern "win64" fn get_activate_result(this: *mut OpObj, hr_out: *mut HRESULT, iface_out: *mut *mut c_void) -> HRESULT {
        if hr_out.is_null() || iface_out.is_null() {
            return E_POINTER;
        }
        *iface_out = std::ptr::null_mut();
        let g = OpObj::inner(this).done.lock().unwrap();
        let Some((hr, p)) = *g else { return E_ILLEGAL_METHOD_CALL };
        *hr_out = hr;
        if hr >= 0 && p != 0 {
            com_call_u32(p as *mut c_void, 1);
            *iface_out = p as *mut c_void;
        }
        S_OK
    }
}

static OP_VTBL: OpVtbl = OpVtbl { qi: op::qi, add_ref: op::add_ref, release: op::release, get_activate_result: op::get_activate_result };

/// What a device interface path names.
#[derive(Debug, PartialEq, Eq)]
pub enum Target {
    DefaultRender,
    Capture,
    ProcessLoopback,
    /// Endpoint id (lowercase), as returned by `IMMDevice::GetId`.
    Id(String),
}

pub fn parse_path(path: &str) -> Target {
    let p = path.trim().to_ascii_lowercase();
    let render = format!("{DEVINTERFACE_AUDIO_RENDER:?}").to_ascii_lowercase();
    let capture = format!("{DEVINTERFACE_AUDIO_CAPTURE:?}").to_ascii_lowercase();
    if p == render {
        return Target::DefaultRender;
    }
    if p == capture {
        return Target::Capture;
    }
    if p.starts_with("vad\\process_loopback") {
        return Target::ProcessLoopback;
    }
    // \\?\SWD#MMDEVAPI#{0.0.0.00000000}.{guid}#{e6327cad-...}
    if let Some(i) = p.find("#mmdevapi#") {
        let rest = &p[i + "#mmdevapi#".len()..];
        let id = rest.split('#').next().unwrap_or(rest);
        if rest.ends_with(&capture) {
            return Target::Capture;
        }
        return Target::Id(id.to_string());
    }
    Target::Id(p)
}

fn resolve(t: &Target) -> Result<Arc<Endpoint>, HRESULT> {
    match t {
        Target::DefaultRender => mmdevapi::refresh().into_iter().next().ok_or(E_NOTFOUND),
        Target::Capture => mmdevapi::refresh_capture().into_iter().next().ok_or(E_NOTFOUND),
        Target::ProcessLoopback => Err(E_NOTIMPL),
        Target::Id(id) => mmdevapi::all_endpoints()
            .into_iter()
            .chain(mmdevapi::refresh())
            .chain(mmdevapi::refresh_capture())
            .find(|e| &e.id == id)
            .ok_or(E_NOTFOUND),
    }
}

/// Resolve + `IMMDevice::Activate(riid, CLSCTX_ALL)`; returns the interface.
unsafe fn activate(t: &Target, iid: &GUID) -> (HRESULT, usize) {
    let ep = match resolve(t) {
        Ok(ep) => ep,
        Err(hr) => return (hr, 0),
    };
    let dev = mmdevapi::new_device(ep) as *mut c_void;
    // IMMDevice::Activate is vtable slot 3.
    let vtbl = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const GUID, u32, *const u8, *mut *mut c_void) -> HRESULT = std::mem::transmute(*vtbl.add(3));
    let mut out: *mut c_void = std::ptr::null_mut();
    let hr = f(dev, iid, 0x17, std::ptr::null(), &mut out);
    com_call_u32(dev, 2);
    (hr, if hr >= 0 { out as usize } else { 0 })
}

/// Worker body: activate, publish the result, call the handler, drop refs.
unsafe fn complete(target: Target, iid: GUID, op: usize, handler: usize) -> u32 {
    let (hr, iface) = activate(&target, &iid);
    log::info!(target: "audio", "ActivateAudioInterfaceAsync({target:?}, {iid:?}) -> {}", super::hr(hr));
    *OpObj::inner(op as *mut OpObj).done.lock().unwrap() = Some((hr, iface));
    // IActivateAudioInterfaceCompletionHandler::ActivateCompleted is slot 3.
    let h = handler as *mut c_void;
    let vtbl = std::ptr::read_unaligned(h as *const *const usize);
    let method = std::ptr::read_unaligned(vtbl.add(3));
    let r = crate::call_guest8(method, h as u64, op as u64, 0, 0, 0, 0, 0, 0) as HRESULT;
    log::debug!(target: "audio", "ActivateCompleted -> {}", super::hr(r));
    guest_com_call_u32(h, 2);
    OpObj::release(op as *mut OpObj);
    0
}

/// `ActivateAudioInterfaceAsync(path, riid, activationParams, handler, &op)`.
pub unsafe fn activate_audio_interface_async(path: *const u16, riid: *const GUID, params: *const u8, handler: *mut c_void, op_out: *mut *mut c_void) -> HRESULT {
    if op_out.is_null() {
        return E_POINTER;
    }
    *op_out = std::ptr::null_mut();
    if path.is_null() || handler.is_null() {
        return E_INVALIDARG;
    }
    let Some(iid) = GUID::read(riid) else { return E_INVALIDARG };
    let path_s = crate::strings::from_wide(path);
    let target = parse_path(&path_s);
    if !params.is_null() {
        // PROPVARIANT: vt u16 @0; VT_BLOB (0x41) = { cbSize u32 @8, pBlobData @16 }.
        let vt = std::ptr::read_unaligned(params as *const u16);
        let size = if vt == 0x41 { std::ptr::read_unaligned(params.add(8) as *const u32) } else { 0 };
        log::debug!(target: "audio", "ActivateAudioInterfaceAsync: activation params vt {vt:#x} ({size} byte blob) ignored");
    }
    log::info!(target: "audio", "ActivateAudioInterfaceAsync({path_s:?} = {target:?}, {iid:?})");

    let op = OpObj::create(&OP_VTBL, OpState::default());
    OpObj::add_ref(op); // worker's reference
    guest_com_call_u32(handler, 1);
    *op_out = op as *mut c_void;

    let (op_u, h_u) = (op as usize, handler as usize);
    let body = Box::new(move || unsafe { complete(target, iid, op_u, h_u) });
    match crate::thread::spawn(body, 256 * 1024, false, false) {
        Ok(t) => {
            *t.name.lock().unwrap() = "ActivateAudioInterfaceAsync worker".into();
        }
        Err(e) => {
            // No thread: undo and fail, the caller sees a clean error.
            log::error!(target: "audio", "ActivateAudioInterfaceAsync: cannot spawn the worker thread: {e}");
            guest_com_call_u32(handler, 2);
            OpObj::release(op);
            OpObj::release(op);
            *op_out = std::ptr::null_mut();
            return E_OUTOFMEMORY;
        }
    }
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_interface_paths() {
        assert_eq!(parse_path("{E6327CAD-DCEC-4949-AE8A-991E976A79D2}"), Target::DefaultRender);
        assert_eq!(parse_path("{e6327cad-dcec-4949-ae8a-991e976a79d2}"), Target::DefaultRender);
        assert_eq!(parse_path("{2EEF81BE-33FA-4800-9670-1CD474972C3F}"), Target::Capture);
        assert_eq!(parse_path("VAD\\Process_Loopback"), Target::ProcessLoopback);
        assert_eq!(
            parse_path("\\\\?\\SWD#MMDEVAPI#{0.0.0.00000000}.{EA996003-A8C2-5194-9835-49D9411E74A1}#{e6327cad-dcec-4949-ae8a-991e976a79d2}"),
            Target::Id("{0.0.0.00000000}.{ea996003-a8c2-5194-9835-49d9411e74a1}".into())
        );
        assert_eq!(parse_path("{0.0.0.00000000}.{EA996003-A8C2-5194-9835-49D9411E74A1}"), Target::Id("{0.0.0.00000000}.{ea996003-a8c2-5194-9835-49d9411e74a1}".into()));
    }
}
