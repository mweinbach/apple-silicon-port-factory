//! Unit tests for `msctf.dll` TSF COM objects and exports.

use super::abi::*;
use super::context::{ITfCompartmentVtbl, ITfContextVtbl, ITfDocumentMgrVtbl};
use super::profiles::{
    CategoryMgrObj, DisplayAttrInfoObj, DisplayAttrMgrObj, EnumLangProfilesObj,
    IEnumTfLanguageProfilesVtbl, ITfCategoryMgrVtbl, ITfDisplayAttributeInfoVtbl,
    ITfDisplayAttributeMgrVtbl, ITfInputProcessorProfileMgrVtbl, ITfInputProcessorProfilesVtbl,
    ITfLangBarMgrVtbl, LangBarMgrObj,
};
use super::range::{ITfPropertyVtbl, ITfRangeVtbl, PropertyObj, RangeObj};
use super::thread_mgr::{
    ITfKeystrokeMgrVtbl, ITfThreadMgr2Vtbl, ITfThreadMgrExVtbl, ITfUIElementMgrVtbl,
};
use super::{
    SetInputScope, SetInputScopes, TF_CreateCategoryMgr, TF_CreateDisplayAttributeMgr,
    TF_CreateInputProcessorProfiles, TF_CreateLangBarMgr, TF_CreateThreadMgr, TF_GetThreadMgr,
    TF_InvalidAssemblyListCacheIfExist,
};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};

const IID_ICLASSFACTORY: Guid =
    Guid::new(0x00000001, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);

// Mock ITfUIElement + ITfUIElementSink + ITfThreadMgrEventSink + ITfCompartmentEventSink
#[repr(C)]
struct MockSinkVtbl {
    qi: unsafe extern "win64" fn(*mut MockSink, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut MockSink) -> u32,
    release: unsafe extern "win64" fn(*mut MockSink) -> u32,
    slot3: unsafe extern "win64" fn(*mut MockSink, u64, u64) -> HRESULT,
    slot4: unsafe extern "win64" fn(*mut MockSink, u64, u64) -> HRESULT,
    slot5: unsafe extern "win64" fn(*mut MockSink, u64, u64) -> HRESULT,
    slot6: unsafe extern "win64" fn(*mut MockSink, u64) -> HRESULT,
    slot7: unsafe extern "win64" fn(*mut MockSink, u64) -> HRESULT,
}

#[repr(C)]
struct MockSink {
    vtbl: *const MockSinkVtbl,
    accept_iid: Guid,
    refs: AtomicU32,
    slot3_calls: AtomicU32,
    slot4_calls: AtomicU32,
    slot5_calls: AtomicU32,
    slot6_calls: AtomicU32,
    slot7_calls: AtomicU32,
    suppress_ui: bool,
}

unsafe extern "win64" fn mock_sink_qi(
    this: *mut MockSink,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let obj = &*this;
    let Some(g) = read_guid_from_bytes(riid) else {
        return E_INVALIDARG;
    };
    if g == IID_IUNKNOWN || g == obj.accept_iid {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn mock_sink_add_ref(this: *mut MockSink) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn mock_sink_release(this: *mut MockSink) -> u32 {
    let obj = &*this;
    obj.refs.fetch_sub(1, Ordering::AcqRel) - 1
}
unsafe extern "win64" fn mock_sink_slot3(this: *mut MockSink, _a1: u64, a2: u64) -> HRESULT {
    let obj = &*this;
    obj.slot3_calls.fetch_add(1, Ordering::AcqRel);
    if obj.suppress_ui && a2 != 0 {
        std::ptr::write_unaligned(a2 as *mut BOOL, FALSE);
    }
    S_OK
}
unsafe extern "win64" fn mock_sink_slot4(this: *mut MockSink, _a1: u64, _a2: u64) -> HRESULT {
    let obj = &*this;
    obj.slot4_calls.fetch_add(1, Ordering::AcqRel);
    S_OK
}
unsafe extern "win64" fn mock_sink_slot5(this: *mut MockSink, _a1: u64, _a2: u64) -> HRESULT {
    let obj = &*this;
    obj.slot5_calls.fetch_add(1, Ordering::AcqRel);
    S_OK
}
unsafe extern "win64" fn mock_sink_slot6(this: *mut MockSink, _a1: u64) -> HRESULT {
    let obj = &*this;
    obj.slot6_calls.fetch_add(1, Ordering::AcqRel);
    S_OK
}
unsafe extern "win64" fn mock_sink_slot7(this: *mut MockSink, _a1: u64) -> HRESULT {
    let obj = &*this;
    obj.slot7_calls.fetch_add(1, Ordering::AcqRel);
    S_OK
}

static MOCK_SINK_VTBL: MockSinkVtbl = MockSinkVtbl {
    qi: mock_sink_qi,
    add_ref: mock_sink_add_ref,
    release: mock_sink_release,
    slot3: mock_sink_slot3,
    slot4: mock_sink_slot4,
    slot5: mock_sink_slot5,
    slot6: mock_sink_slot6,
    slot7: mock_sink_slot7,
};

fn make_mock_sink(iid: Guid, suppress_ui: bool) -> MockSink {
    MockSink {
        vtbl: &MOCK_SINK_VTBL,
        accept_iid: iid,
        refs: AtomicU32::new(1),
        slot3_calls: AtomicU32::new(0),
        slot4_calls: AtomicU32::new(0),
        slot5_calls: AtomicU32::new(0),
        slot6_calls: AtomicU32::new(0),
        slot7_calls: AtomicU32::new(0),
        suppress_ui,
    }
}

// Mock ITfEditSession
#[repr(C)]
struct MockEditSessionVtbl {
    qi: unsafe extern "win64" fn(*mut MockEditSession, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut MockEditSession) -> u32,
    release: unsafe extern "win64" fn(*mut MockEditSession) -> u32,
    do_edit_session: unsafe extern "win64" fn(*mut MockEditSession, TfEditCookie) -> HRESULT,
}
#[repr(C)]
struct MockEditSession {
    vtbl: *const MockEditSessionVtbl,
    refs: AtomicU32,
    ctx: *mut c_void,
    tid: TfClientId,
    saw_cookie: AtomicU32,
    saw_in_write: AtomicU32,
}
unsafe extern "win64" fn mes_qi(
    this: *mut MockEditSession,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if read_guid_from_bytes(riid) == Some(IID_IUNKNOWN) {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}
unsafe extern "win64" fn mes_add_ref(this: *mut MockEditSession) -> u32 {
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}
unsafe extern "win64" fn mes_release(this: *mut MockEditSession) -> u32 {
    let obj = &*this;
    obj.refs.fetch_sub(1, Ordering::AcqRel) - 1
}
unsafe extern "win64" fn mes_do_edit(this: *mut MockEditSession, ec: TfEditCookie) -> HRESULT {
    let obj = &*this;
    obj.saw_cookie.store(ec, Ordering::Release);
    let ctx_vt = &*(*(obj.ctx as *const *const ITfContextVtbl));
    let mut in_write: BOOL = FALSE;
    assert_eq!((ctx_vt.in_write_session)(obj.ctx, obj.tid, &mut in_write), S_OK);
    obj.saw_in_write.store(in_write as u32, Ordering::Release);
    S_OK
}
static MOCK_EDIT_SESSION_VTBL: MockEditSessionVtbl = MockEditSessionVtbl {
    qi: mes_qi,
    add_ref: mes_add_ref,
    release: mes_release,
    do_edit_session: mes_do_edit,
};

#[test]
fn com_creation_co_get_class_object_and_thread_mgr_lifecycle() {
    // Run on a dedicated thread so TF_GetThreadMgr starts strictly empty.
    std::thread::spawn(|| unsafe {
        // Before creation, TF_GetThreadMgr returns S_OK with NULL pointer.
        let mut got: *mut c_void = 0x1234usize as *mut c_void;
        assert_eq!(TF_GetThreadMgr(&mut got), S_OK);
        assert!(got.is_null());

        // CoCreateInstance(CLSID_TF_ThreadMgr, IID_ITfThreadMgr)
        let mut tm: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::misc::ole32::CoCreateInstance(
                guid_as_ptr(&CLSID_TF_THREADMGR),
                std::ptr::null_mut(),
                1,
                guid_as_ptr(&IID_ITFTHREADMGR),
                &mut tm,
            ),
            S_OK
        );
        assert!(!tm.is_null());

        // TF_GetThreadMgr now returns the exact same per-thread manager.
        let mut tm_get: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_GetThreadMgr(&mut tm_get), S_OK);
        assert_eq!(tm_get, tm);
        com_call_release(tm_get);

        // TF_CreateThreadMgr also returns the synchronized per-thread manager.
        let mut tm_create: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateThreadMgr(&mut tm_create), S_OK);
        assert_eq!(tm_create, tm);
        com_call_release(tm_create);

        // CoCreateInstance with IID_ITfThreadMgrEx and IID_ITfThreadMgr2
        let mut tm_ex: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::misc::ole32::CoCreateInstance(
                guid_as_ptr(&CLSID_TF_THREADMGR),
                std::ptr::null_mut(),
                1,
                guid_as_ptr(&IID_ITFTHREADMGREX),
                &mut tm_ex,
            ),
            S_OK
        );
        assert_eq!(tm_ex, tm);

        let mut tm2: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi_bytes(tm, guid_as_ptr(&IID_ITFTHREADMGR2), &mut tm2), S_OK);
        assert!(!tm2.is_null());
        assert_ne!(tm2, tm); // distinct vtable offset!
        // COM identity rule: QI(IUnknown) from ITfThreadMgr2 returns canonical offset-0 pointer.
        let mut unk: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi_bytes(tm2, guid_as_ptr(&IID_IUNKNOWN), &mut unk), S_OK);
        assert_eq!(unk, tm);
        com_call_release(unk);

        // ActivateEx / GetActiveFlags / Activate / Deactivate + ITfThreadMgr2 keystroke suspend
        let tm_vt = &*(*(tm_ex as *const *const ITfThreadMgrExVtbl));
        let tm2_vt = &*(*(tm2 as *const *const ITfThreadMgr2Vtbl));

        let mut active_flags = 0xDEADu32;
        assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
        assert_eq!(active_flags, 0);

        let mut cid: TfClientId = 0;
        assert_eq!((tm_vt.activate_ex)(tm_ex, &mut cid, 0x0004), S_OK);
        assert_ne!(cid, 0);
        assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
        assert_eq!(active_flags, 0x0004 | TF_TMF_ACTIVATED);

        // Nested Activate through ITfThreadMgr2 shares exact same thread state
        let mut cid2: TfClientId = 0;
        assert_eq!((tm2_vt.activate)(tm2, &mut cid2), S_OK);
        assert_eq!(cid2, cid);

        let mut is_focus: BOOL = FALSE;
        assert_eq!((tm2_vt.is_thread_focus)(tm2, &mut is_focus), S_OK);
        assert_eq!(is_focus, TRUE);

        assert_eq!((tm2_vt.suspend_keystroke_handling)(tm2), S_OK);
        assert_eq!((tm2_vt.resume_keystroke_handling)(tm2), S_OK);
        assert_eq!((tm2_vt.resume_keystroke_handling)(tm2), E_UNEXPECTED);

        assert_eq!((tm_vt.deactivate)(tm_ex), S_OK);
        assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
        assert_ne!(active_flags, 0); // still active once
        assert_eq!((tm_vt.deactivate)(tm_ex), S_OK);
        assert_eq!((tm_vt.get_active_flags)(tm_ex, &mut active_flags), S_OK);
        assert_eq!(active_flags, 0);
        assert_eq!((tm_vt.deactivate)(tm_ex), E_UNEXPECTED);

        // CoGetClassObject returns real IClassFactory for CLSID_TF_ThreadMgr
        let mut cf: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::misc::ole32::CoGetClassObject(
                guid_as_ptr(&CLSID_TF_THREADMGR),
                1,
                std::ptr::null_mut(),
                guid_as_ptr(&IID_ICLASSFACTORY),
                &mut cf,
            ),
            S_OK
        );
        assert!(!cf.is_null());
        let mut from_cf: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            com_call_slot(
                cf,
                3, // IClassFactory::CreateInstance
                0,
                guid_as_ptr(&IID_ITFTHREADMGR) as u64,
                (&mut from_cf as *mut *mut c_void) as u64,
            ),
            S_OK
        );
        assert_eq!(from_cf, tm);
        com_call_release(from_cf);
        com_call_release(cf);

        // Release all refs to ThreadMgr; TF_GetThreadMgr must return NULL again.
        com_call_release(tm2);
        com_call_release(tm_ex);
        assert_eq!(com_call_release(tm), 0);

        let mut after_drop: *mut c_void = 0x9999usize as *mut c_void;
        assert_eq!(TF_GetThreadMgr(&mut after_drop), S_OK);
        assert!(after_drop.is_null());
    })
    .join()
    .unwrap();
}

#[test]
fn document_mgr_context_stack_focus_and_edit_session() {
    std::thread::spawn(|| unsafe {
        let mut tm: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateThreadMgr(&mut tm), S_OK);
        let tm_vt = &*(*(tm as *const *const ITfThreadMgrExVtbl));

        let mut cid: TfClientId = 0;
        assert_eq!((tm_vt.activate)(tm, &mut cid), S_OK);

        // Advise ITfThreadMgrEventSink to observe OnInitDocumentMgr, OnSetFocus, OnPushContext, OnPopContext
        let mut src: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi_bytes(tm, guid_as_ptr(&IID_ITFSOURCE), &mut src), S_OK);
        let src_vt = &*(*(src as *const *const ITfSourceVtbl));
        let mut tm_sink = make_mock_sink(IID_ITFTHREADMGREVENTSINK, false);
        let mut sink_cookie = 0u32;
        assert_eq!(
            (src_vt.advise_sink)(
                src,
                guid_as_ptr(&IID_ITFTHREADMGREVENTSINK),
                (&mut tm_sink as *mut MockSink) as *mut c_void,
                &mut sink_cookie,
            ),
            S_OK
        );
        assert_eq!(tm_sink.refs.load(Ordering::Acquire), 2);

        // Create two DocumentMgrs
        let mut dm1: *mut c_void = std::ptr::null_mut();
        let mut dm2: *mut c_void = std::ptr::null_mut();
        assert_eq!((tm_vt.create_document_mgr)(tm, &mut dm1), S_OK);
        assert_eq!((tm_vt.create_document_mgr)(tm, &mut dm2), S_OK);
        assert_eq!(tm_sink.slot3_calls.load(Ordering::Acquire), 2); // OnInitDocumentMgr x2

        // CreateContext / Push / GetTop / GetBase / EnumContexts / Pop
        let dm_vt = &*(*(dm1 as *const *const ITfDocumentMgrVtbl));
        let (mut ctx1, mut ec1) = (std::ptr::null_mut(), 0u32);
        let (mut ctx2, mut ec2) = (std::ptr::null_mut(), 0u32);
        assert_eq!(
            (dm_vt.create_context)(dm1, cid, 0, std::ptr::null_mut(), &mut ctx1, &mut ec1),
            S_OK
        );
        assert_eq!(
            (dm_vt.create_context)(dm1, cid, 0, std::ptr::null_mut(), &mut ctx2, &mut ec2),
            S_OK
        );
        assert_ne!(ec1, TF_INVALID_EDIT_COOKIE);
        assert_ne!(ec2, TF_INVALID_EDIT_COOKIE);

        assert_eq!((dm_vt.push)(dm1, ctx1), S_OK);
        assert_eq!((dm_vt.push)(dm1, ctx2), S_OK);
        assert_eq!(tm_sink.slot6_calls.load(Ordering::Acquire), 2); // OnPushContext x2

        let mut top: *mut c_void = std::ptr::null_mut();
        let mut base: *mut c_void = std::ptr::null_mut();
        assert_eq!((dm_vt.get_top)(dm1, &mut top), S_OK);
        assert_eq!((dm_vt.get_base)(dm1, &mut base), S_OK);
        assert_eq!(top, ctx2);
        assert_eq!(base, ctx1);
        com_call_release(top);
        com_call_release(base);

        // ITfContext::GetDocumentMgr & RequestEditSession + ITfProperty roundtrip
        let ctx_vt = &*(*(ctx1 as *const *const ITfContextVtbl));
        let mut owner_dm: *mut c_void = std::ptr::null_mut();
        assert_eq!((ctx_vt.get_document_mgr)(ctx1, &mut owner_dm), S_OK);
        assert_eq!(owner_dm, dm1);
        com_call_release(owner_dm);

        let mut es = MockEditSession {
            vtbl: &MOCK_EDIT_SESSION_VTBL,
            refs: AtomicU32::new(1),
            ctx: ctx1,
            tid: cid,
            saw_cookie: AtomicU32::new(0),
            saw_in_write: AtomicU32::new(0),
        };
        let mut hr_sess = E_FAIL;
        assert_eq!(
            (ctx_vt.request_edit_session)(
                ctx1,
                cid,
                (&mut es as *mut MockEditSession) as *mut c_void,
                TF_ES_SYNC | TF_ES_READWRITE,
                &mut hr_sess,
            ),
            S_OK
        );
        assert_eq!(hr_sess, S_OK);
        assert_ne!(es.saw_cookie.load(Ordering::Acquire), 0);
        assert_eq!(es.saw_in_write.load(Ordering::Acquire), TRUE as u32);
        assert_eq!(es.refs.load(Ordering::Acquire), 1);

        // Pop top context -> ctx1 becomes top; Pop(TF_POPF_ALL) empties stack
        assert_eq!((dm_vt.pop)(dm1, 0), S_OK);
        assert_eq!((dm_vt.get_top)(dm1, &mut top), S_OK);
        assert_eq!(top, ctx1);
        com_call_release(top);
        assert_eq!((dm_vt.pop)(dm1, TF_POPF_ALL), S_OK);
        assert_eq!((dm_vt.get_top)(dm1, &mut top), S_OK);
        assert!(top.is_null());
        assert_eq!(tm_sink.slot7_calls.load(Ordering::Acquire), 2); // OnPopContext x2

        // SetFocus / GetFocus / AssociateFocus
        assert_eq!((tm_vt.set_focus)(tm, dm1), S_OK);
        let mut focused: *mut c_void = std::ptr::null_mut();
        assert_eq!((tm_vt.get_focus)(tm, &mut focused), S_OK);
        assert_eq!(focused, dm1);
        com_call_release(focused);
        assert_eq!(tm_sink.slot5_calls.load(Ordering::Acquire), 1); // OnSetFocus

        let fake_hwnd = 0x000A_BCDEusize as HWND;
        let mut prev_assoc: *mut c_void = 0x1usize as *mut c_void;
        assert_eq!((tm_vt.associate_focus)(tm, fake_hwnd, dm1, &mut prev_assoc), S_OK);
        assert!(prev_assoc.is_null());

        assert_eq!((tm_vt.associate_focus)(tm, fake_hwnd, dm2, &mut prev_assoc), S_OK);
        assert_eq!(prev_assoc, dm1);
        com_call_release(prev_assoc);

        assert_eq!(
            (tm_vt.associate_focus)(tm, fake_hwnd, std::ptr::null_mut(), &mut prev_assoc),
            S_OK
        );
        assert_eq!(prev_assoc, dm2);
        com_call_release(prev_assoc);

        // UnadviseSink releases sink ref
        assert_eq!((src_vt.unadvise_sink)(src, sink_cookie), S_OK);
        assert_eq!(tm_sink.refs.load(Ordering::Acquire), 1);
        assert_eq!((src_vt.unadvise_sink)(src, sink_cookie), CONNECT_E_NOCONNECTION);

        // Cleanup
        com_call_release(ctx2);
        com_call_release(ctx1);
        assert_eq!((tm_vt.deactivate)(tm), S_OK);
        com_call_release(dm2);
        com_call_release(dm1);
        com_call_release(src);
        assert_eq!(com_call_release(tm), 0);
    })
    .join()
    .unwrap();
}

#[test]
fn ui_element_mgr_compartments_and_profiles_category_display_attrs() {
    std::thread::spawn(|| unsafe {
        let mut tm: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateThreadMgr(&mut tm), S_OK);

        // ITfUIElementMgr + ITfSource(ITfUIElementSink)
        let mut src: *mut c_void = std::ptr::null_mut();
        let mut ui_mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi_bytes(tm, guid_as_ptr(&IID_ITFSOURCE), &mut src), S_OK);
        assert_eq!(com_call_qi_bytes(tm, guid_as_ptr(&IID_ITFUIELEMENTMGR), &mut ui_mgr), S_OK);
        assert_ne!(src, tm);
        assert_ne!(ui_mgr, tm);
        assert_ne!(src, ui_mgr);

        let src_vt = &*(*(src as *const *const ITfSourceVtbl));
        let ui_vt = &*(*(ui_mgr as *const *const ITfUIElementMgrVtbl));

        let mut ui_sink = make_mock_sink(IID_ITFUIELEMENTSINK, true);
        let mut ui_cookie = 0u32;
        assert_eq!(
            (src_vt.advise_sink)(
                src,
                guid_as_ptr(&IID_ITFUIELEMENTSINK),
                (&mut ui_sink as *mut MockSink) as *mut c_void,
                &mut ui_cookie,
            ),
            S_OK
        );

        let mut elem = make_mock_sink(IID_IUNKNOWN, false);
        let elem_ptr = (&mut elem as *mut MockSink) as *mut c_void;
        let mut show: BOOL = TRUE;
        let mut elem_id = 0u32;
        assert_eq!((ui_vt.begin_ui_element)(ui_mgr, elem_ptr, &mut show, &mut elem_id), S_OK);
        assert_ne!(elem_id, 0);
        assert_eq!(show, FALSE); // sink suppressed default UI!
        assert_eq!(ui_sink.slot3_calls.load(Ordering::Acquire), 1);

        assert_eq!((ui_vt.update_ui_element)(ui_mgr, elem_id), S_OK);
        assert_eq!(ui_sink.slot4_calls.load(Ordering::Acquire), 1);

        let mut fetched_elem: *mut c_void = std::ptr::null_mut();
        assert_eq!((ui_vt.get_ui_element)(ui_mgr, elem_id, &mut fetched_elem), S_OK);
        assert_eq!(fetched_elem, elem_ptr);
        com_call_release(fetched_elem);

        let mut ui_enum: *mut c_void = std::ptr::null_mut();
        assert_eq!((ui_vt.enum_ui_elements)(ui_mgr, &mut ui_enum), S_OK);
        let e_vt = &*(*(ui_enum as *const *const IEnumTfComVtbl));
        let mut out_elem: *mut c_void = std::ptr::null_mut();
        let mut fetched = 0u32;
        assert_eq!((e_vt.next)(ui_enum as *mut EnumTfComObj, 1, &mut out_elem, &mut fetched), S_OK);
        assert_eq!(fetched, 1);
        assert_eq!(out_elem, elem_ptr);
        com_call_release(out_elem);
        com_call_release(ui_enum);

        assert_eq!((ui_vt.end_ui_element)(ui_mgr, elem_id), S_OK);
        assert_eq!(ui_sink.slot5_calls.load(Ordering::Acquire), 1);
        assert_eq!(elem.refs.load(Ordering::Acquire), 1);
        assert_eq!((src_vt.unadvise_sink)(src, ui_cookie), S_OK);
        assert_eq!(ui_sink.refs.load(Ordering::Acquire), 1);

        // ITfCompartmentMgr + ITfCompartment + ITfCompartmentEventSink + IEnumGUID
        let mut comp_mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi_bytes(tm, guid_as_ptr(&IID_ITFCOMPARTMENTMGR), &mut comp_mgr), S_OK);
        let cm_vt = &*(*(comp_mgr as *const *const ITfCompartmentMgrVtbl));

        let test_comp_guid = Guid::new(0x11223344, 0x5566, 0x7788, [1, 2, 3, 4, 5, 6, 7, 8]);
        let mut comp: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            (cm_vt.get_compartment)(comp_mgr, guid_as_ptr(&test_comp_guid), &mut comp),
            S_OK
        );
        let comp_vt = &*(*(comp as *const *const ITfCompartmentVtbl));

        let mut comp_src: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi_bytes(comp, guid_as_ptr(&IID_ITFSOURCE), &mut comp_src), S_OK);
        let csrc_vt = &*(*(comp_src as *const *const ITfSourceVtbl));
        let mut comp_sink = make_mock_sink(IID_ITFCOMPARTMENTEVENTSINK, false);
        let mut comp_cookie = 0u32;
        assert_eq!(
            (csrc_vt.advise_sink)(
                comp_src,
                guid_as_ptr(&IID_ITFCOMPARTMENTEVENTSINK),
                (&mut comp_sink as *mut MockSink) as *mut c_void,
                &mut comp_cookie,
            ),
            S_OK
        );

        // Empty compartment returns S_FALSE
        let mut var_out = [0xFFu8; 24];
        assert_eq!((comp_vt.get_value)(comp, var_out.as_mut_ptr()), S_FALSE);
        assert_eq!(u16::from_le_bytes([var_out[0], var_out[1]]), VT_EMPTY);

        // Set VT_I4 = 42
        let mut var_in = [0u8; 24];
        std::ptr::write_unaligned(var_in.as_mut_ptr() as *mut u16, 3u16); // VT_I4
        std::ptr::write_unaligned(var_in.as_mut_ptr().add(8) as *mut i32, 42);
        assert_eq!((comp_vt.set_value)(comp, 1, var_in.as_ptr()), S_OK);
        assert_eq!(comp_sink.slot3_calls.load(Ordering::Acquire), 1);

        assert_eq!((comp_vt.get_value)(comp, var_out.as_mut_ptr()), S_OK);
        assert_eq!(std::ptr::read_unaligned(var_out.as_ptr() as *const u16), 3u16);
        assert_eq!(std::ptr::read_unaligned(var_out.as_ptr().add(8) as *const i32), 42);

        // EnumCompartments (IEnumGUID: Next at slot 3!)
        let mut comp_enum: *mut c_void = std::ptr::null_mut();
        assert_eq!((cm_vt.enum_compartments)(comp_mgr, &mut comp_enum), S_OK);
        let eg_vt = &*(*(comp_enum as *const *const IEnumGUIDVtbl));
        let mut enum_g = [0u8; 16];
        let mut eg_fetched = 0u32;
        assert_eq!(
            (eg_vt.next)(comp_enum as *mut EnumGuidObj, 1, enum_g.as_mut_ptr(), &mut eg_fetched),
            S_OK
        );
        assert_eq!(eg_fetched, 1);
        assert_eq!(enum_g, test_comp_guid.to_bytes());
        com_call_release(comp_enum);

        assert_eq!((cm_vt.clear_compartment)(comp_mgr, 1, guid_as_ptr(&test_comp_guid)), S_OK);
        assert_eq!(comp_sink.slot3_calls.load(Ordering::Acquire), 2);
        assert_eq!((comp_vt.get_value)(comp, var_out.as_mut_ptr()), S_FALSE);
        assert_eq!((csrc_vt.unadvise_sink)(comp_src, comp_cookie), S_OK);
        com_call_release(comp_src);
        com_call_release(comp);

        // ITfInputProcessorProfiles (en-US active profile, GetCurrentLanguage, GetLanguageList, GetDefaultLanguageProfile)
        let mut profiles: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::misc::ole32::CoCreateInstance(
                guid_as_ptr(&CLSID_TF_INPUTPROCESSORPROFILES),
                std::ptr::null_mut(),
                1,
                guid_as_ptr(&IID_ITFINPUTPROCESSORPROFILES),
                &mut profiles,
            ),
            S_OK
        );
        let p_vt = &*(*(profiles as *const *const ITfInputProcessorProfilesVtbl));
        let mut cur_lang = 0u16;
        assert_eq!((p_vt.get_current_language)(profiles, &mut cur_lang), S_OK);
        assert_eq!(cur_lang, LANGID_EN_US);

        let mut langs_ptr: *mut u16 = std::ptr::null_mut();
        let mut langs_cnt = 0u32;
        assert_eq!((p_vt.get_language_list)(profiles, &mut langs_ptr, &mut langs_cnt), S_OK);
        assert!(langs_cnt >= 1);
        assert_eq!(std::ptr::read_unaligned(langs_ptr), LANGID_EN_US);
        crate::misc::ole32::CoTaskMemFree(langs_ptr as *mut c_void);

        let mut def_clsid = [0u8; 16];
        let mut def_guid = [0u8; 16];
        assert_eq!(
            (p_vt.get_default_language_profile)(
                profiles,
                LANGID_EN_US,
                guid_as_ptr(&GUID_TFCAT_TIP_KEYBOARD),
                def_clsid.as_mut_ptr(),
                def_guid.as_mut_ptr(),
            ),
            S_OK
        );
        assert_eq!(def_clsid, CLSID_EN_US_KEYBOARD_TIP.to_bytes());
        assert_eq!(def_guid, GUID_EN_US_KEYBOARD_PROFILE.to_bytes());

        // ITfInputProcessorProfileMgr via QI
        let mut prof_mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            com_call_qi_bytes(profiles, guid_as_ptr(&IID_ITFINPUTPROCESSORPROFILEMGR), &mut prof_mgr),
            S_OK
        );
        let pm_vt = &*(*(prof_mgr as *const *const ITfInputProcessorProfileMgrVtbl));
        let mut ipp_rec = TF_INPUTPROCESSORPROFILE::default();
        assert_eq!(
            (pm_vt.get_active_profile)(
                prof_mgr,
                guid_as_ptr(&GUID_TFCAT_TIP_KEYBOARD),
                (&mut ipp_rec as *mut TF_INPUTPROCESSORPROFILE) as *mut u8,
            ),
            S_OK
        );
        assert_eq!(ipp_rec.langid, LANGID_EN_US);
        com_call_release(prof_mgr);
        com_call_release(profiles);

        // ITfCategoryMgr GUID-atom roundtrip
        let mut cat_mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateCategoryMgr(&mut cat_mgr), S_OK);
        let cat_vt = &*(*(cat_mgr as *const *const ITfCategoryMgrVtbl));
        let mut atom: TfGuidAtom = 0;
        assert_eq!(
            (cat_vt.register_guid)(
                cat_mgr as *mut CategoryMgrObj,
                guid_as_ptr(&test_comp_guid),
                &mut atom,
            ),
            S_OK
        );
        assert_ne!(atom, TF_INVALID_GUIDATOM);
        let mut got_g = [0u8; 16];
        assert_eq!((cat_vt.get_guid)(cat_mgr as *mut CategoryMgrObj, atom, got_g.as_mut_ptr()), S_OK);
        assert_eq!(got_g, test_comp_guid.to_bytes());
        let mut eq: BOOL = FALSE;
        assert_eq!(
            (cat_vt.is_equal_tf_guid_atom)(
                cat_mgr as *mut CategoryMgrObj,
                atom,
                guid_as_ptr(&test_comp_guid),
                &mut eq,
            ),
            S_OK
        );
        assert_eq!(eq, TRUE);
        com_call_release(cat_mgr);

        // ITfDisplayAttributeMgr enumerator & info lookup
        let mut da_mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateDisplayAttributeMgr(&mut da_mgr), S_OK);
        let da_vt = &*(*(da_mgr as *const *const ITfDisplayAttributeMgrVtbl));
        let mut da_enum: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            (da_vt.enum_display_attribute_info)(da_mgr as *mut DisplayAttrMgrObj, &mut da_enum),
            S_OK
        );
        let dae_vt = &*(*(da_enum as *const *const IEnumTfComVtbl));
        let mut da_info: *mut c_void = std::ptr::null_mut();
        let mut da_fetched = 0u32;
        assert_eq!(
            (dae_vt.next)(da_enum as *mut EnumTfComObj, 1, &mut da_info, &mut da_fetched),
            S_OK
        );
        assert_eq!(da_fetched, 1);
        assert!(!da_info.is_null());
        let dai_vt = &*(*(da_info as *const *const ITfDisplayAttributeInfoVtbl));
        let mut attr = TF_DISPLAYATTRIBUTE::default();
        assert_eq!(
            (dai_vt.get_attribute_info)(
                da_info as *mut DisplayAttrInfoObj,
                (&mut attr as *mut TF_DISPLAYATTRIBUTE) as *mut u8,
            ),
            S_OK
        );
        assert_eq!(attr.ls_style, TF_LS_DOT);
        com_call_release(da_info);
        com_call_release(da_enum);
        com_call_release(da_mgr);

        // ITfLangBarMgr + SetInputScope / SetInputScopes / TF_InvalidAssemblyListCacheIfExist
        let mut lbm: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateLangBarMgr(&mut lbm), S_OK);
        let lbm_vt = &*(*(lbm as *const *const ITfLangBarMgrVtbl));
        assert_eq!((lbm_vt.show_floating)(lbm as *mut LangBarMgrObj, 0x0001), S_OK);
        let mut fl = 0u32;
        assert_eq!((lbm_vt.get_show_floating_status)(lbm as *mut LangBarMgrObj, &mut fl), S_OK);
        assert_eq!(fl, 0x0001);
        com_call_release(lbm);

        let hwnd = 0x0005_0001usize as HWND;
        assert_eq!(SetInputScope(hwnd, 57), S_OK);
        let scopes = [1i32, 57i32];
        assert_eq!(
            SetInputScopes(
                hwnd,
                scopes.as_ptr(),
                2,
                std::ptr::null(),
                0,
                std::ptr::null(),
                std::ptr::null(),
            ),
            S_OK
        );
        assert_eq!(SetInputScope(hwnd, 0), S_OK);
        assert_eq!(TF_InvalidAssemblyListCacheIfExist(), S_OK);

        com_call_release(comp_mgr);
        com_call_release(ui_mgr);
        com_call_release(src);
        assert_eq!(com_call_release(tm), 0);
    })
    .join()
    .unwrap();
}

#[test]
fn keystroke_mgr_enumerators_properties_and_exports_lookup() {
    // Verify all 9 msctf.dll exports resolve through win32-rt's DLL table.
    for sym in [
        "TF_CreateThreadMgr",
        "TF_GetThreadMgr",
        "TF_CreateInputProcessorProfiles",
        "TF_CreateDisplayAttributeMgr",
        "TF_CreateCategoryMgr",
        "TF_CreateLangBarMgr",
        "TF_InvalidAssemblyListCacheIfExist",
        "SetInputScope",
        "SetInputScopes",
    ] {
        assert!(
            crate::lookup("msctf.dll", sym).is_some(),
            "msctf.dll!{sym} missing from builtin export table"
        );
    }

    std::thread::spawn(|| unsafe {
        // CoCreateInstance covers all 5 TSF CLSIDs and enforces aggregation/interface checks.
        for (clsid, iid) in [
            (CLSID_TF_THREADMGR, IID_ITFTHREADMGR),
            (CLSID_TF_INPUTPROCESSORPROFILES, IID_ITFINPUTPROCESSORPROFILES),
            (CLSID_TF_CATEGORYMGR, IID_ITFCATEGORYMGR),
            (CLSID_TF_DISPLAYATTRIBUTEMGR, IID_ITFDISPLAYATTRIBUTEMGR),
            (CLSID_TF_LANGBARMGR, IID_ITFLANGBARMGR),
        ] {
            let mut obj: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::ole32::CoCreateInstance(
                    guid_as_ptr(&clsid),
                    std::ptr::null_mut(),
                    1,
                    guid_as_ptr(&iid),
                    &mut obj,
                ),
                S_OK
            );
            assert!(!obj.is_null());
            com_call_release(obj);

            // Aggregation must fail cleanly with CLASS_E_NOAGGREGATION.
            assert_eq!(
                crate::misc::ole32::CoCreateInstance(
                    guid_as_ptr(&clsid),
                    0x1usize as *mut c_void,
                    1,
                    guid_as_ptr(&iid),
                    &mut obj,
                ),
                CLASS_E_NOAGGREGATION
            );
        }

        // ITfKeystrokeMgr preserved keys, key event sink, and key down/up queries
        let mut tm: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateThreadMgr(&mut tm), S_OK);
        let tm_vt = &*(*(tm as *const *const ITfThreadMgrExVtbl));
        let mut cid: TfClientId = 0;
        assert_eq!((tm_vt.activate)(tm, &mut cid), S_OK);

        let mut ks: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi_bytes(tm, guid_as_ptr(&IID_ITFKEYSTROKEMGR), &mut ks), S_OK);
        let ks_vt = &*(*(ks as *const *const ITfKeystrokeMgrVtbl));

        let mut eaten: BOOL = TRUE;
        assert_eq!((ks_vt.test_key_down)(ks, 0x41, 0, &mut eaten), S_OK);
        assert_eq!(eaten, FALSE);
        assert_eq!((ks_vt.key_down)(ks, 0x41, 0, &mut eaten), S_OK);
        assert_eq!(eaten, FALSE);

        let pk_guid = Guid::new(0xAABBCCDD, 0x1122, 0x3344, [5, 6, 7, 8, 9, 10, 11, 12]);
        let prekey = TF_PRESERVEDKEY { u_vkey: 0x20, u_modifiers: 0x0002 }; // Ctrl+Space
        let desc_w = crate::strings::to_wide("Toggle IME");
        assert_eq!(
            (ks_vt.preserve_key)(
                ks,
                cid,
                guid_as_ptr(&pk_guid),
                (&prekey as *const TF_PRESERVEDKEY) as *const u8,
                desc_w.as_ptr(),
                desc_w.len() as u32,
            ),
            S_OK
        );
        let mut registered: BOOL = FALSE;
        assert_eq!(
            (ks_vt.is_preserved_key)(
                ks,
                guid_as_ptr(&pk_guid),
                (&prekey as *const TF_PRESERVEDKEY) as *const u8,
                &mut registered,
            ),
            S_OK
        );
        assert_eq!(registered, TRUE);

        let mut bstr_desc: *mut u16 = std::ptr::null_mut();
        assert_eq!(
            (ks_vt.get_preserved_key_description)(ks, guid_as_ptr(&pk_guid), &mut bstr_desc),
            S_OK
        );
        assert_eq!(crate::strings::from_wide(bstr_desc), "Toggle IME");
        crate::misc::oleaut32::SysFreeString(bstr_desc);

        assert_eq!(
            (ks_vt.unpreserve_key)(
                ks,
                guid_as_ptr(&pk_guid),
                (&prekey as *const TF_PRESERVEDKEY) as *const u8,
            ),
            S_OK
        );
        com_call_release(ks);

        // ITfDocumentMgr::EnumContexts + ITfContext::GetProperty + ITfRange + GetGlobalCompartment
        let mut dm: *mut c_void = std::ptr::null_mut();
        assert_eq!((tm_vt.create_document_mgr)(tm, &mut dm), S_OK);
        let dm_vt = &*(*(dm as *const *const ITfDocumentMgrVtbl));
        let (mut ctx, mut ec) = (std::ptr::null_mut(), 0u32);
        assert_eq!(
            (dm_vt.create_context)(dm, cid, 0, std::ptr::null_mut(), &mut ctx, &mut ec),
            S_OK
        );
        assert_eq!((dm_vt.push)(dm, ctx), S_OK);

        let mut enum_ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!((dm_vt.enum_contexts)(dm, &mut enum_ctx), S_OK);
        let ec_vt = &*(*(enum_ctx as *const *const IEnumTfComVtbl));
        let mut got_ctx: *mut c_void = std::ptr::null_mut();
        let mut fetched = 0u32;
        assert_eq!((ec_vt.next)(enum_ctx as *mut EnumTfComObj, 1, &mut got_ctx, &mut fetched), S_OK);
        assert_eq!(fetched, 1);
        assert_eq!(got_ctx, ctx);
        com_call_release(got_ctx);
        com_call_release(enum_ctx);

        let ctx_vt = &*(*(ctx as *const *const ITfContextVtbl));
        let mut prop: *mut c_void = std::ptr::null_mut();
        assert_eq!((ctx_vt.get_property)(ctx, guid_as_ptr(&pk_guid), &mut prop), S_OK);
        let prop_vt = &*(*(prop as *const *const ITfPropertyVtbl));
        let mut var_in = [0u8; 24];
        std::ptr::write_unaligned(var_in.as_mut_ptr() as *mut u16, 3u16); // VT_I4
        std::ptr::write_unaligned(var_in.as_mut_ptr().add(8) as *mut i32, 777);
        assert_eq!(
            (prop_vt.set_value)(prop as *mut PropertyObj, ec, std::ptr::null_mut(), var_in.as_ptr()),
            S_OK
        );
        let mut var_out = [0u8; 24];
        assert_eq!(
            (prop_vt.get_value)(
                prop as *mut PropertyObj,
                ec,
                std::ptr::null_mut(),
                var_out.as_mut_ptr(),
            ),
            S_OK
        );
        assert_eq!(std::ptr::read_unaligned(var_out.as_ptr().add(8) as *const i32), 777);
        com_call_release(prop);

        // ITfContext::TrackProperties: track two properties (one set to 777, one empty)
        // and verify ITfReadOnlyProperty::GetValue returns VT_UNKNOWN -> IEnumTfPropertyValue.
        let prop_guid2 = Guid::new(0xDEADBEEF, 0xCAFE, 0xBABE, [9, 8, 7, 6, 5, 4, 3, 2]);
        let prop_ptrs = [guid_as_ptr(&pk_guid), guid_as_ptr(&prop_guid2)];
        let mut tracked: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            (ctx_vt.track_properties)(ctx, prop_ptrs.as_ptr(), 2, std::ptr::null(), 0, &mut tracked),
            S_OK
        );
        assert!(!tracked.is_null());
        let tp_vt = &*(*(tracked as *const *const super::range::ITfReadOnlyPropertyVtbl));
        let mut tguid = [0xFFu8; 16];
        assert_eq!((tp_vt.get_type)(tracked as *mut super::range::TrackedPropertyObj, tguid.as_mut_ptr()), S_OK);
        assert_eq!(tguid, GUID_NULL.to_bytes());

        let mut enum_ranges: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            (tp_vt.enum_ranges)(tracked as *mut super::range::TrackedPropertyObj, ec, &mut enum_ranges, std::ptr::null_mut()),
            S_OK
        );
        assert!(!enum_ranges.is_null());
        com_call_release(enum_ranges);

        let mut tracked_var = [0u8; 24];
        assert_eq!(
            (tp_vt.get_value)(
                tracked as *mut super::range::TrackedPropertyObj,
                ec,
                std::ptr::null_mut(),
                tracked_var.as_mut_ptr()
            ),
            S_OK
        );
        assert_eq!(std::ptr::read_unaligned(tracked_var.as_ptr() as *const u16), VT_UNKNOWN);
        let pv_enum = std::ptr::read_unaligned(tracked_var.as_ptr().add(8) as *const usize) as *mut c_void;
        assert!(!pv_enum.is_null());
        let pv_vt = &*(*(pv_enum as *const *const super::range::IEnumTfPropertyValueVtbl));
        let mut vals = [TF_PROPERTYVAL { guid_id: GUID_NULL, var_value: [0u8; 24] }; 2];
        let mut pv_fetched = 0u32;
        assert_eq!(
            (pv_vt.next)(pv_enum as *mut super::range::EnumTfPropValObj, 2, vals.as_mut_ptr(), &mut pv_fetched),
            S_OK
        );
        assert_eq!(pv_fetched, 2);
        assert_eq!(vals[0].guid_id, pk_guid);
        assert_eq!(std::ptr::read_unaligned(vals[0].var_value.as_ptr() as *const u16), 3u16); // VT_I4
        assert_eq!(std::ptr::read_unaligned(vals[0].var_value.as_ptr().add(8) as *const i32), 777);
        assert_eq!(vals[1].guid_id, prop_guid2);
        assert_eq!(std::ptr::read_unaligned(vals[1].var_value.as_ptr() as *const u16), VT_EMPTY);
        assert_eq!((pv_vt.reset)(pv_enum as *mut super::range::EnumTfPropValObj), S_OK);
        assert_eq!((pv_vt.skip)(pv_enum as *mut super::range::EnumTfPropValObj, 1), S_OK);
        let mut one_val = TF_PROPERTYVAL { guid_id: GUID_NULL, var_value: [0u8; 24] };
        assert_eq!(
            (pv_vt.next)(pv_enum as *mut super::range::EnumTfPropValObj, 1, &mut one_val, &mut pv_fetched),
            S_OK
        );
        assert_eq!(pv_fetched, 1);
        assert_eq!(one_val.guid_id, prop_guid2);
        com_call_release(pv_enum);
        com_call_release(tracked);

        // TrackProperties + IEnumTfPropertyValue::Clone must not leak intermediate
        // owning VT_UNKNOWN refcounts when cloning/reading property values.
        #[repr(C)]
        struct RefTrackedUnk {
            vtbl: *const [usize; 3],
            refs: std::sync::atomic::AtomicU32,
        }
        unsafe extern "win64" fn rtu_qi(_t: *mut RefTrackedUnk, _r: *const u8, p: *mut *mut c_void) -> HRESULT {
            if !p.is_null() { p.write_unaligned(std::ptr::null_mut()); }
            E_NOINTERFACE
        }
        unsafe extern "win64" fn rtu_addref(t: *mut RefTrackedUnk) -> u32 {
            (*t).refs.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
        }
        unsafe extern "win64" fn rtu_release(t: *mut RefTrackedUnk) -> u32 {
            (*t).refs.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) - 1
        }
        let rtu_vtbl: [usize; 3] = [
            rtu_qi as *const () as usize,
            rtu_addref as *const () as usize,
            rtu_release as *const () as usize,
        ];
        crate::com::register_vtable_ptrs(&rtu_vtbl);
        let mut unk_obj = RefTrackedUnk {
            vtbl: &rtu_vtbl,
            refs: std::sync::atomic::AtomicU32::new(1),
        };
        let unk_guid = Guid::new(0x11223344, 0x5566, 0x7788, [1, 2, 3, 4, 5, 6, 7, 8]);
        let mut unk_prop: *mut c_void = std::ptr::null_mut();
        assert_eq!((ctx_vt.get_property)(ctx, guid_as_ptr(&unk_guid), &mut unk_prop), S_OK);
        let unk_prop_vt = &*(*(unk_prop as *const *const ITfPropertyVtbl));
        let mut unk_var = [0u8; 24];
        std::ptr::write_unaligned(unk_var.as_mut_ptr() as *mut u16, VT_UNKNOWN);
        std::ptr::write_unaligned(
            unk_var.as_mut_ptr().add(8) as *mut usize,
            &mut unk_obj as *mut RefTrackedUnk as usize,
        );
        assert_eq!(
            (unk_prop_vt.set_value)(unk_prop as *mut PropertyObj, ec, std::ptr::null_mut(), unk_var.as_ptr()),
            S_OK
        );
        assert_eq!(unk_obj.refs.load(std::sync::atomic::Ordering::SeqCst), 2); // caller + property
        let unk_guids = [guid_as_ptr(&unk_guid)];
        let mut tracked_u: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            (ctx_vt.track_properties)(ctx, unk_guids.as_ptr(), 1, std::ptr::null(), 0, &mut tracked_u),
            S_OK
        );
        let tpu_vt = &*(*(tracked_u as *const *const super::range::ITfReadOnlyPropertyVtbl));
        let mut tv_u = [0u8; 24];
        assert_eq!(
            (tpu_vt.get_value)(tracked_u as *mut super::range::TrackedPropertyObj, ec, std::ptr::null_mut(), tv_u.as_mut_ptr()),
            S_OK
        );
        assert_eq!(unk_obj.refs.load(std::sync::atomic::Ordering::SeqCst), 3); // +1 for enumerator (no double-copy leak!)
        let en1 = std::ptr::read_unaligned(tv_u.as_ptr().add(8) as *const usize) as *mut c_void;
        let en1_vt = &*(*(en1 as *const *const super::range::IEnumTfPropertyValueVtbl));
        let mut en2: *mut c_void = std::ptr::null_mut();
        assert_eq!((en1_vt.clone)(en1 as *mut super::range::EnumTfPropValObj, &mut en2), S_OK);
        assert_eq!(unk_obj.refs.load(std::sync::atomic::Ordering::SeqCst), 4); // +1 for cloned enumerator (no double-copy leak!)
        com_call_release(en2);
        assert_eq!(unk_obj.refs.load(std::sync::atomic::Ordering::SeqCst), 3);
        com_call_release(en1);
        assert_eq!(unk_obj.refs.load(std::sync::atomic::Ordering::SeqCst), 2);
        com_call_release(tracked_u);
        assert_eq!((unk_prop_vt.clear)(unk_prop as *mut PropertyObj, ec, std::ptr::null_mut()), S_OK);
        com_call_release(unk_prop);
        assert_eq!(unk_obj.refs.load(std::sync::atomic::Ordering::SeqCst), 1);

        // Invalid TrackProperties arguments return E_INVALIDARG.
        assert_eq!(
            (ctx_vt.track_properties)(ctx, std::ptr::null(), 0, std::ptr::null(), 0, &mut tracked),
            E_INVALIDARG
        );
        assert!(tracked.is_null());

        let mut start_range: *mut c_void = std::ptr::null_mut();
        assert_eq!((ctx_vt.get_start)(ctx, ec, &mut start_range), S_OK);
        let r_vt = &*(*(start_range as *const *const ITfRangeVtbl));
        let mut is_empty: BOOL = FALSE;
        assert_eq!((r_vt.is_empty)(start_range as *mut RangeObj, ec, &mut is_empty), S_OK);
        assert_eq!(is_empty, TRUE);
        com_call_release(start_range);

        let mut gcomp_mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!((tm_vt.get_global_compartment)(tm, &mut gcomp_mgr), S_OK);
        assert!(!gcomp_mgr.is_null());
        com_call_release(gcomp_mgr);

        // ITfInputProcessorProfiles::EnumLanguageProfiles
        let mut profiles: *mut c_void = std::ptr::null_mut();
        assert_eq!(TF_CreateInputProcessorProfiles(&mut profiles), S_OK);
        let p_vt = &*(*(profiles as *const *const ITfInputProcessorProfilesVtbl));
        let mut lp_enum: *mut c_void = std::ptr::null_mut();
        assert_eq!((p_vt.enum_language_profiles)(profiles, LANGID_EN_US, &mut lp_enum), S_OK);
        let elp_vt = &*(*(lp_enum as *const *const IEnumTfLanguageProfilesVtbl));
        let mut lp = TF_LANGUAGEPROFILE::default();
        let mut lp_fetched = 0u32;
        assert_eq!(
            (elp_vt.next)(
                lp_enum as *mut EnumLangProfilesObj,
                1,
                (&mut lp as *mut TF_LANGUAGEPROFILE) as *mut u8,
                &mut lp_fetched,
            ),
            S_OK
        );
        assert_eq!(lp_fetched, 1);
        assert_eq!(lp.langid, LANGID_EN_US);
        assert_eq!(lp.f_active, TRUE);
        com_call_release(lp_enum);
        com_call_release(profiles);

        assert_eq!((dm_vt.pop)(dm, TF_POPF_ALL), S_OK);
        com_call_release(ctx);
        com_call_release(dm);
        assert_eq!((tm_vt.deactivate)(tm), S_OK);
        assert_eq!(com_call_release(tm), 0);
    })
    .join()
    .unwrap();
}
