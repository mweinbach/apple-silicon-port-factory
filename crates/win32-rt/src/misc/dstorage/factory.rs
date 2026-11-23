//! `IDStorageFactory` singleton COM implementation.
//!
//! Vtable order (`dstorage.h`):
//!   0: QueryInterface, 1: AddRef, 2: Release,
//!   3: CreateQueue, 4: OpenFile, 5: CreateStatusArray,
//!   6: SetDebugFlags, 7: SetStagingBufferSize.

use std::ffi::c_void;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

use super::abi::*;
use super::queue::{custom_decomp_singleton, new_file_obj, new_queue_obj, new_status_array_obj};
use crate::com::{
    com_add_ref, read_guid, register_vtable_once, write_null_out, ComObj, Guid, IID_IUnknown,
};
use crate::strings::{rd, read_ansi_bytes_z, read_wide_z, wr};

pub(super) static DEBUG_FLAGS: AtomicU32 = AtomicU32::new(0);
pub(super) static STAGING_BUFFER_SIZE: AtomicU32 = AtomicU32::new(32 * 1024 * 1024);

struct FactoryObj;

unsafe extern "win64" fn ds_factory_qi(
    this: *mut ComObj<FactoryObj>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IDSTORAGE_FACTORY {
        ComObj::add_ref(this);
        wr(ppv, this as *mut c_void);
        S_OK
    } else if CUSTOM_DECOMP_IIDS.contains(&iid) {
        wr(ppv, custom_decomp_singleton() as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn ds_factory_release(this: *mut ComObj<FactoryObj>) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    // Process-wide singleton: keep a floor refcount of 1 so the static factory stays valid.
    let mut cur = obj.refs.load(Ordering::Acquire);
    while cur > 1 {
        match obj.refs.compare_exchange_weak(
            cur,
            cur - 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return cur - 1,
            Err(v) => cur = v,
        }
    }
    1
}

unsafe extern "win64" fn ds_factory_create_queue(
    _this: *mut c_void,
    desc_ptr: *const DSTORAGE_QUEUE_DESC,
    riid: usize,
    ppv: *mut usize,
) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    wr(ppv, 0);
    if desc_ptr.is_null() {
        return E_INVALIDARG;
    }
    if !iid_matches_or_null(riid, QUEUE_IIDS) {
        return E_NOINTERFACE;
    }

    let desc = rd(desc_ptr);
    if desc.source_type != DSTORAGE_REQUEST_SOURCE_FILE as u64
        && desc.source_type != DSTORAGE_REQUEST_SOURCE_MEMORY as u64
    {
        return E_INVALIDARG;
    }
    if desc.capacity == 0 || desc.capacity > DSTORAGE_MAX_QUEUE_CAPACITY {
        return E_DSTORAGE_INVALID_QUEUE_CAPACITY;
    }

    let name_buf = if desc.name.is_null() {
        None
    } else {
        let mut bytes = read_ansi_bytes_z(desc.name, 255);
        bytes.push(0);
        Some(bytes.into_boxed_slice())
    };
    wr(ppv, new_queue_obj(desc, name_buf));
    S_OK
}

unsafe extern "win64" fn ds_factory_open_file(
    _this: *mut c_void,
    path_w: *const u16,
    riid: usize,
    ppv: *mut usize,
) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    wr(ppv, 0);
    if path_w.is_null() {
        return E_INVALIDARG;
    }
    if !iid_matches_or_null(riid, FILE_IIDS) {
        return E_NOINTERFACE;
    }

    let guest_path = read_wide_z(path_w, 4096);
    if guest_path.is_empty() {
        return E_INVALIDARG;
    }
    let host_path = match crate::path::to_host(&guest_path) {
        Some(p) if p.exists() => p,
        other => {
            let direct = PathBuf::from(&guest_path);
            if direct.exists() {
                direct
            } else {
                other.unwrap_or(direct)
            }
        }
    };
    let file = match File::open(&host_path) {
        Ok(f) => f,
        Err(e) => return io_error_to_hresult(&e),
    };

    wr(ppv, new_file_obj(guest_path, file));
    S_OK
}

unsafe extern "win64" fn ds_factory_create_status_array(
    _this: *mut c_void,
    capacity: u32,
    _name: *const u8,
    riid: usize,
    ppv: *mut usize,
) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    wr(ppv, 0);
    if capacity == 0 {
        return E_INVALIDARG;
    }
    if !iid_matches_or_null(riid, STATUS_ARRAY_IIDS) {
        return E_NOINTERFACE;
    }

    wr(ppv, new_status_array_obj(capacity as usize));
    S_OK
}

unsafe extern "win64" fn ds_factory_set_debug_flags(_this: *mut c_void, flags: u32) {
    DEBUG_FLAGS.store(flags, Ordering::Relaxed);
}

unsafe extern "win64" fn ds_factory_set_staging_buffer_size(_this: *mut c_void, size: u32) -> i32 {
    STAGING_BUFFER_SIZE.store(size, Ordering::Relaxed);
    S_OK
}

fn factory_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            ds_factory_qi as *const () as usize,
            com_add_ref::<FactoryObj> as *const () as usize,
            ds_factory_release as *const () as usize,
            ds_factory_create_queue as *const () as usize,
            ds_factory_open_file as *const () as usize,
            ds_factory_create_status_array as *const () as usize,
            ds_factory_set_debug_flags as *const () as usize,
            ds_factory_set_staging_buffer_size as *const () as usize,
        ]
    })
}

pub(super) fn factory_singleton() -> usize {
    static FACTORY: OnceLock<usize> = OnceLock::new();
    let ptr = *FACTORY.get_or_init(|| ComObj::new(factory_vtable(), FactoryObj) as usize);
    unsafe {
        ComObj::add_ref(ptr as *mut ComObj<FactoryObj>);
    }
    ptr
}
