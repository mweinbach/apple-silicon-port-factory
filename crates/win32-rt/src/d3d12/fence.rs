//! ID3D12Fence1 on `MTLSharedEvent`.

use super::com::*;
use super::tr::tr;
use super::failed;
use super::vtables::*;
use crate::types::*;
use d3d12_metal::metal::fence::Fence as MtlFence;
use std::sync::OnceLock;

pub struct Fence {
    pub f: MtlFence,
    pub flags: u32,
}

static CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for Fence {
    fn class() -> &'static Class {
        class_once(&CLASS, || Class {
            name: "ID3D12Fence1",
            vtbl: build_named::<Fence>(
                "ID3D12Fence1",
                ID3D12Fence1,
                &[
                    m!("GetCompletedValue" => get_completed_value),
                    m!("SetEventOnCompletion" => set_event_on_completion),
                    m!("Signal" => signal),
                    m!("GetCreationFlags" => get_creation_flags),
                ],
            ),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12Fence, IID_ID3D12Fence1],
        })
    }
}

pub type PF = *mut Obj<Fence>;

pub unsafe fn from_ptr<'a>(p: *mut std::ffi::c_void) -> Option<&'a Fence> {
    Obj::<Fence>::from_iface(p).map(|o| &(*o).inner)
}

unsafe extern "win64" fn get_completed_value(this: PF) -> u64 {
    Obj::get(this).f.completed()
}

/// NULL event = block until reached (documented D3D12 behavior).
unsafe extern "win64" fn set_event_on_completion(this: PF, value: u64, event: HANDLE) -> HRESULT {
    let f = &Obj::get(this).f;
    if event.is_null() {
        let _scope = crate::hitch::Scope::enter(crate::hitch::ActKind::FenceWait, value);
        f.wait_cpu(value, u64::MAX);
        return S_OK;
    }
    let Some(ev) = crate::handles::get_as::<crate::handles::Event>(event) else {
        return failed!(E_INVALIDARG, "ID3D12Fence::SetEventOnCompletion({value}, {event:?}): not an event handle");
    };
    // Event::set touches no TEB state, so it may run on the listener's
    // dispatch thread.
    f.on_completion(value, Box::new(move || ev.set()));
    S_OK
}

unsafe extern "win64" fn signal(this: PF, value: u64) -> HRESULT {
    tr!(Call::FenceSignal { fence: this as u64, value });
    Obj::get(this).f.signal_cpu(value);
    S_OK
}

unsafe extern "win64" fn get_creation_flags(this: PF) -> u32 {
    Obj::get(this).flags
}

pub(crate) fn plain_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        vec![
            get_completed_value as *const () as usize,
            set_event_on_completion as *const () as usize,
            signal as *const () as usize,
            get_creation_flags as *const () as usize,
        ]
        .into_boxed_slice()
    })
}
