//! DirectML COM objects (`IDMLDevice1`, `IDMLOperator`, `IDMLCompiledOperator`,
//! `IDMLOperatorInitializer`, `IDMLBindingTable`, `IDMLCommandRecorder`) built
//! directly on [`crate::com`] primitives.

use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

use super::abi::*;
use super::graph::{
    parse_graph_plan, run_graph, run_initialize, run_single_op, BindingSlot, BindingTableState,
    BufferBinding, CompiledOpState, ExecutionPlan,
};
use super::ops::{elem_byte_size, OpSpec};
use crate::com::{
    com_add_ref, com_call_qi, com_call_slot, com_from_iface, com_release, qi_helper, read_guid,
    register_vtable_once, write_null_out, ComObj, ComPtr, Guid, PrivateData,
};
use crate::d3d12::com::DXGI_ERROR_UNSUPPORTED;
use crate::strings::{rd, wr};
use crate::types::{BOOL, E_FAIL, E_INVALIDARG, E_POINTER, FALSE, HRESULT, S_OK, TRUE};

// ============================================================================
// Shared DirectML COM Object Mixins (IDMLObject + IDMLDeviceChild via crate::com)
// ============================================================================

trait HasPrivateData {
    fn private_data(&self) -> &Mutex<PrivateData>;
}

struct DmlDeviceState {
    private: Mutex<PrivateData>,
    d3d12_device: Option<ComPtr>,
    flags: u32,
}

impl HasPrivateData for DmlDeviceState {
    fn private_data(&self) -> &Mutex<PrivateData> {
        &self.private
    }
}

struct DmlChild<T> {
    private: Mutex<PrivateData>,
    device: Option<ComPtr>,
    inner: T,
}

impl<T> DmlChild<T> {
    unsafe fn new(device_ptr: *mut c_void, inner: T) -> Self {
        Self {
            private: Mutex::new(PrivateData::new()),
            device: ComPtr::retain(device_ptr),
            inner,
        }
    }
}

impl<T> HasPrivateData for DmlChild<T> {
    fn private_data(&self) -> &Mutex<PrivateData> {
        &self.private
    }
}

struct DmlOperatorData {
    spec: Arc<OpSpec>,
}

struct DmlCompiledData {
    compiled: Arc<CompiledOpState>,
}

struct DmlInitializerData {
    ops: Mutex<Vec<Arc<CompiledOpState>>>,
}

struct DmlBindingTableData {
    state: Mutex<BindingTableState>,
}

struct DmlCommandRecorderData;

type DmlDeviceObj = ComObj<DmlDeviceState>;
type DmlOperatorObj = ComObj<DmlChild<DmlOperatorData>>;
type DmlCompiledOperatorObj = ComObj<DmlChild<DmlCompiledData>>;
type DmlInitializerObj = ComObj<DmlChild<DmlInitializerData>>;
type DmlBindingTableObj = ComObj<DmlChild<DmlBindingTableData>>;
type DmlCommandRecorderObj = ComObj<DmlChild<DmlCommandRecorderData>>;

// IDMLObject slots 3..6: GetPrivateData, SetPrivateData, SetPrivateDataInterface, SetName
unsafe extern "win64" fn dml_get_private_data<T: HasPrivateData>(
    this: *mut ComObj<T>,
    g: *const Guid,
    size: *mut u32,
    data: *mut c_void,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    ComObj::inner(this)
        .private_data()
        .lock()
        .unwrap()
        .get_private_data(g, size, data)
}

unsafe extern "win64" fn dml_set_private_data<T: HasPrivateData>(
    this: *mut ComObj<T>,
    g: *const Guid,
    size: u32,
    data: *const c_void,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    ComObj::inner(this)
        .private_data()
        .lock()
        .unwrap()
        .set_private_data(g, size, data)
}

unsafe extern "win64" fn dml_set_private_data_interface<T: HasPrivateData>(
    this: *mut ComObj<T>,
    g: *const Guid,
    unk: *mut c_void,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    ComObj::inner(this)
        .private_data()
        .lock()
        .unwrap()
        .set_private_data_interface(g, unk)
}

unsafe extern "win64" fn dml_set_name<T: HasPrivateData>(
    this: *mut ComObj<T>,
    name: *const u16,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    ComObj::inner(this)
        .private_data()
        .lock()
        .unwrap()
        .set_name(name)
}

// IDMLDeviceChild slot 7: GetDevice
unsafe extern "win64" fn dml_child_get_device<T>(
    this: *mut ComObj<DmlChild<T>>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
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
    let Some(dev) = ComObj::inner(this).device.as_ref() else {
        return E_FAIL;
    };
    com_call_qi(dev.as_ptr(), &iid, ppv)
}

// ============================================================================
// IDMLOperator (8 slots)
// ============================================================================

const OP_IIDS: &[Guid] = &[IID_IDMLObject, IID_IDMLDeviceChild, IID_IDMLOperator];

unsafe extern "win64" fn op_qi(
    this: *mut DmlOperatorObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, OP_IIDS)
}

fn operator_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            op_qi as *const () as usize,
            com_add_ref::<DmlChild<DmlOperatorData>> as *const () as usize,
            com_release::<DmlChild<DmlOperatorData>> as *const () as usize,
            dml_get_private_data::<DmlChild<DmlOperatorData>> as *const () as usize,
            dml_set_private_data::<DmlChild<DmlOperatorData>> as *const () as usize,
            dml_set_private_data_interface::<DmlChild<DmlOperatorData>> as *const () as usize,
            dml_set_name::<DmlChild<DmlOperatorData>> as *const () as usize,
            dml_child_get_device::<DmlOperatorData> as *const () as usize,
        ]
    })
}

unsafe fn operator_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlOperatorObj> {
    com_from_iface(p, operator_vtable())
}

// ============================================================================
// IDMLCompiledOperator (9 slots)
// ============================================================================

const COMPILED_IIDS: &[Guid] = &[
    IID_IDMLObject,
    IID_IDMLDeviceChild,
    IID_IDMLPageable,
    IID_IDMLDispatchable,
    IID_IDMLCompiledOperator,
];

unsafe extern "win64" fn compiled_qi(
    this: *mut DmlCompiledOperatorObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, COMPILED_IIDS)
}

unsafe extern "win64" fn compiled_get_binding_props(
    this: *mut DmlCompiledOperatorObj,
    ret: *mut DML_BINDING_PROPERTIES,
) -> *mut DML_BINDING_PROPERTIES {
    if !ret.is_null() {
        let props = if this.is_null() {
            DML_BINDING_PROPERTIES::default()
        } else {
            ComObj::inner(this).inner.compiled.binding_props
        };
        wr(ret, props);
    }
    ret
}

fn compiled_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 9]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            compiled_qi as *const () as usize,
            com_add_ref::<DmlChild<DmlCompiledData>> as *const () as usize,
            com_release::<DmlChild<DmlCompiledData>> as *const () as usize,
            dml_get_private_data::<DmlChild<DmlCompiledData>> as *const () as usize,
            dml_set_private_data::<DmlChild<DmlCompiledData>> as *const () as usize,
            dml_set_private_data_interface::<DmlChild<DmlCompiledData>> as *const () as usize,
            dml_set_name::<DmlChild<DmlCompiledData>> as *const () as usize,
            dml_child_get_device::<DmlCompiledData> as *const () as usize,
            compiled_get_binding_props as *const () as usize,
        ]
    })
}

unsafe fn compiled_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlCompiledOperatorObj> {
    com_from_iface(p, compiled_vtable())
}

// ============================================================================
// IDMLOperatorInitializer (10 slots)
// ============================================================================

const INIT_IIDS: &[Guid] = &[
    IID_IDMLObject,
    IID_IDMLDeviceChild,
    IID_IDMLPageable,
    IID_IDMLDispatchable,
    IID_IDMLOperatorInitializer,
];

unsafe extern "win64" fn init_qi(
    this: *mut DmlInitializerObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, INIT_IIDS)
}

unsafe extern "win64" fn init_get_binding_props(
    this: *mut DmlInitializerObj,
    ret: *mut DML_BINDING_PROPERTIES,
) -> *mut DML_BINDING_PROPERTIES {
    if !ret.is_null() {
        let props = if this.is_null() {
            DML_BINDING_PROPERTIES::default()
        } else {
            let ops = ComObj::inner(this).inner.ops.lock().unwrap();
            let req = ops
                .iter()
                .map(|o| o.binding_props.RequiredDescriptorCount)
                .sum::<u32>()
                .max(1);
            DML_BINDING_PROPERTIES {
                RequiredDescriptorCount: req,
                _pad: 0,
                TemporaryResourceSize: 0,
                PersistentResourceSize: 0,
            }
        };
        wr(ret, props);
    }
    ret
}

unsafe extern "win64" fn init_reset(
    this: *mut DmlInitializerObj,
    op_count: u32,
    ops_ptr: *const *mut c_void,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    if op_count > 0 && ops_ptr.is_null() {
        return E_INVALIDARG;
    }
    let mut new_ops = Vec::with_capacity(op_count as usize);
    for i in 0..op_count as usize {
        let p = rd(ops_ptr.add(i));
        let Some(co) = compiled_from_ptr(p) else {
            return E_INVALIDARG;
        };
        new_ops.push(co.inner.inner.compiled.clone());
    }
    *ComObj::inner(this).inner.ops.lock().unwrap() = new_ops;
    S_OK
}

fn initializer_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 10]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            init_qi as *const () as usize,
            com_add_ref::<DmlChild<DmlInitializerData>> as *const () as usize,
            com_release::<DmlChild<DmlInitializerData>> as *const () as usize,
            dml_get_private_data::<DmlChild<DmlInitializerData>> as *const () as usize,
            dml_set_private_data::<DmlChild<DmlInitializerData>> as *const () as usize,
            dml_set_private_data_interface::<DmlChild<DmlInitializerData>> as *const () as usize,
            dml_set_name::<DmlChild<DmlInitializerData>> as *const () as usize,
            dml_child_get_device::<DmlInitializerData> as *const () as usize,
            init_get_binding_props as *const () as usize,
            init_reset as *const () as usize,
        ]
    })
}

unsafe fn initializer_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlInitializerObj> {
    com_from_iface(p, initializer_vtable())
}

unsafe fn is_valid_dispatchable(p: *mut c_void) -> bool {
    compiled_from_ptr(p).is_some() || initializer_from_ptr(p).is_some()
}

// ============================================================================
// IDMLBindingTable (13 slots: 0..7 DeviceChild, 8..12 Bind*/Reset)
// ============================================================================

const BT_IIDS: &[Guid] = &[IID_IDMLObject, IID_IDMLDeviceChild, IID_IDMLBindingTable];

unsafe fn parse_binding_desc(p: *const DML_BINDING_DESC) -> BindingSlot {
    if p.is_null() {
        return BindingSlot::None;
    }
    let bd = rd(p);
    match bd.Type {
        DML_BINDING_TYPE_BUFFER if !bd.Desc.is_null() => {
            let bb = rd(bd.Desc as *const DML_BUFFER_BINDING);
            BindingSlot::Buffer(BufferBinding {
                resource_ptr: bb.Buffer as usize,
                offset: bb.Offset,
                size: bb.SizeInBytes,
            })
        }
        DML_BINDING_TYPE_BUFFER_ARRAY if !bd.Desc.is_null() => {
            let ba = rd(bd.Desc as *const DML_BUFFER_ARRAY_BINDING);
            let mut list = Vec::with_capacity(ba.BindingCount as usize);
            if !ba.Bindings.is_null() {
                for i in 0..ba.BindingCount as usize {
                    let bb = rd(ba.Bindings.add(i));
                    list.push(BufferBinding {
                        resource_ptr: bb.Buffer as usize,
                        offset: bb.Offset,
                        size: bb.SizeInBytes,
                    });
                }
            }
            BindingSlot::BufferArray(list)
        }
        _ => BindingSlot::None,
    }
}

unsafe extern "win64" fn bt_qi(
    this: *mut DmlBindingTableObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, BT_IIDS)
}

unsafe extern "win64" fn bt_bind_inputs(
    this: *mut DmlBindingTableObj,
    count: u32,
    bindings: *const DML_BINDING_DESC,
) {
    if this.is_null() {
        return;
    }
    let mut list = Vec::with_capacity(count as usize);
    if !bindings.is_null() {
        for i in 0..count as usize {
            list.push(parse_binding_desc(bindings.add(i)));
        }
    }
    ComObj::inner(this).inner.state.lock().unwrap().inputs = list;
}

unsafe extern "win64" fn bt_bind_outputs(
    this: *mut DmlBindingTableObj,
    count: u32,
    bindings: *const DML_BINDING_DESC,
) {
    if this.is_null() {
        return;
    }
    let mut list = Vec::with_capacity(count as usize);
    if !bindings.is_null() {
        for i in 0..count as usize {
            list.push(parse_binding_desc(bindings.add(i)));
        }
    }
    ComObj::inner(this).inner.state.lock().unwrap().outputs = list;
}

unsafe extern "win64" fn bt_bind_temp(
    this: *mut DmlBindingTableObj,
    binding: *const DML_BINDING_DESC,
) {
    if this.is_null() {
        return;
    }
    ComObj::inner(this).inner.state.lock().unwrap().temp = parse_binding_desc(binding);
}

unsafe extern "win64" fn bt_bind_persistent(
    this: *mut DmlBindingTableObj,
    binding: *const DML_BINDING_DESC,
) {
    if this.is_null() {
        return;
    }
    ComObj::inner(this).inner.state.lock().unwrap().persistent = parse_binding_desc(binding);
}

unsafe extern "win64" fn bt_reset(
    this: *mut DmlBindingTableObj,
    desc: *const DML_BINDING_TABLE_DESC,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).inner.state.lock().unwrap();
    st.inputs.clear();
    st.outputs.clear();
    st.temp = BindingSlot::None;
    st.persistent = BindingSlot::None;
    if desc.is_null() {
        st.target_ptr = 0;
        st.cpu_handle = 0;
        st.gpu_handle = 0;
        st.size_in_descriptors = 0;
        return S_OK;
    }
    let d = rd(desc);
    if !d.Dispatchable.is_null() {
        if !is_valid_dispatchable(d.Dispatchable) {
            return E_INVALIDARG;
        }
        st.target_ptr = d.Dispatchable as usize;
    } else {
        st.target_ptr = 0;
    }
    st.cpu_handle = d.CPUDescriptorHandle;
    st.gpu_handle = d.GPUDescriptorHandle;
    st.size_in_descriptors = d.SizeInDescriptors;
    S_OK
}

fn binding_table_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 13]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            bt_qi as *const () as usize,
            com_add_ref::<DmlChild<DmlBindingTableData>> as *const () as usize,
            com_release::<DmlChild<DmlBindingTableData>> as *const () as usize,
            dml_get_private_data::<DmlChild<DmlBindingTableData>> as *const () as usize,
            dml_set_private_data::<DmlChild<DmlBindingTableData>> as *const () as usize,
            dml_set_private_data_interface::<DmlChild<DmlBindingTableData>> as *const () as usize,
            dml_set_name::<DmlChild<DmlBindingTableData>> as *const () as usize,
            dml_child_get_device::<DmlBindingTableData> as *const () as usize,
            bt_bind_inputs as *const () as usize,
            bt_bind_outputs as *const () as usize,
            bt_bind_temp as *const () as usize,
            bt_bind_persistent as *const () as usize,
            bt_reset as *const () as usize,
        ]
    })
}

unsafe fn binding_table_from_ptr<'a>(p: *mut c_void) -> Option<&'a DmlBindingTableObj> {
    com_from_iface(p, binding_table_vtable())
}

// ============================================================================
// IDMLCommandRecorder (9 slots: 0..7 DeviceChild, 8 RecordDispatch)
// ============================================================================

const RECORDER_IIDS: &[Guid] = &[IID_IDMLObject, IID_IDMLDeviceChild, IID_IDMLCommandRecorder];

unsafe extern "win64" fn recorder_qi(
    this: *mut DmlCommandRecorderObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, RECORDER_IIDS)
}

unsafe extern "win64" fn recorder_record_dispatch(
    this: *mut DmlCommandRecorderObj,
    _command_list: *mut c_void,
    dispatchable: *mut c_void,
    bindings: *mut c_void,
) {
    if this.is_null() || dispatchable.is_null() || bindings.is_null() {
        return;
    }
    let Some(bt_obj) = binding_table_from_ptr(bindings) else {
        log::warn!(target: "d3d", "IDMLCommandRecorder::RecordDispatch: foreign binding table {bindings:?}");
        return;
    };
    let bt_guard = bt_obj.inner.inner.state.lock().unwrap();
    log::trace!(
        target: "d3d",
        "IDMLCommandRecorder::RecordDispatch(disp={dispatchable:?}, target={:#x}, descs={}, cpu={:#x}, gpu={:#x}, temp={})",
        bt_guard.target_ptr,
        bt_guard.size_in_descriptors,
        bt_guard.cpu_handle,
        bt_guard.gpu_handle,
        bt_guard.temp.first_buffer().is_some(),
    );
    if let Some(init) = initializer_from_ptr(dispatchable) {
        let ops = init.inner.inner.ops.lock().unwrap().clone();
        run_initialize(&ops, &bt_guard);
    } else if let Some(co) = compiled_from_ptr(dispatchable) {
        let compiled = &co.inner.inner.compiled;
        match &compiled.plan {
            ExecutionPlan::Single(spec) => run_single_op(compiled, spec, &bt_guard),
            ExecutionPlan::Graph(graph) => run_graph(graph, &bt_guard),
        }
    } else {
        log::warn!(target: "d3d", "IDMLCommandRecorder::RecordDispatch: unknown dispatchable {dispatchable:?}");
    }
}

fn recorder_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 9]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            recorder_qi as *const () as usize,
            com_add_ref::<DmlChild<DmlCommandRecorderData>> as *const () as usize,
            com_release::<DmlChild<DmlCommandRecorderData>> as *const () as usize,
            dml_get_private_data::<DmlChild<DmlCommandRecorderData>> as *const () as usize,
            dml_set_private_data::<DmlChild<DmlCommandRecorderData>> as *const () as usize,
            dml_set_private_data_interface::<DmlChild<DmlCommandRecorderData>> as *const () as usize,
            dml_set_name::<DmlChild<DmlCommandRecorderData>> as *const () as usize,
            dml_child_get_device::<DmlCommandRecorderData> as *const () as usize,
            recorder_record_dispatch as *const () as usize,
        ]
    })
}

// ============================================================================
// IDMLDevice / IDMLDevice1 (18 slots)
// ============================================================================

const DEVICE_IIDS: &[Guid] = &[IID_IDMLObject, IID_IDMLDevice, IID_IDMLDevice1];

unsafe extern "win64" fn dev_qi(
    this: *mut DmlDeviceObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, DEVICE_IIDS)
}

// Slot 7: CheckFeatureSupport
unsafe extern "win64" fn dev_check_feature_support(
    _this: *mut DmlDeviceObj,
    feature: u32,
    query_size: u32,
    query_data: *const c_void,
    data_size: u32,
    feature_data: *mut c_void,
) -> HRESULT {
    if query_data.is_null() || feature_data.is_null() {
        return E_INVALIDARG;
    }
    match feature {
        DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT => {
            if query_size < 4 || data_size < 4 {
                return E_INVALIDARG;
            }
            let dt = rd(query_data as *const u32);
            let supported = elem_byte_size(dt).is_some();
            wr(feature_data as *mut BOOL, if supported { TRUE } else { FALSE });
            S_OK
        }
        DML_FEATURE_FEATURE_LEVELS => {
            if query_size < std::mem::size_of::<DML_FEATURE_QUERY_FEATURE_LEVELS>() as u32
                || data_size < 4
            {
                return E_INVALIDARG;
            }
            let q = rd(query_data as *const DML_FEATURE_QUERY_FEATURE_LEVELS);
            if q.RequestedFeatureLevelCount == 0 || q.RequestedFeatureLevels.is_null() {
                return E_INVALIDARG;
            }
            let mut max_fl = 0u32;
            for i in 0..q.RequestedFeatureLevelCount as usize {
                let fl = rd(q.RequestedFeatureLevels.add(i));
                if (DML_FEATURE_LEVEL_1_0..=MAX_SUPPORTED_FEATURE_LEVEL).contains(&fl) {
                    max_fl = max_fl.max(fl);
                }
            }
            if max_fl == 0 {
                wr(feature_data as *mut u32, 0);
                return DXGI_ERROR_UNSUPPORTED;
            }
            wr(feature_data as *mut u32, max_fl);
            S_OK
        }
        _ => E_INVALIDARG,
    }
}

// Slot 8: CreateOperator
unsafe extern "win64" fn dev_create_operator(
    this: *mut DmlDeviceObj,
    desc: *const DML_OPERATOR_DESC,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let spec = match OpSpec::parse(desc) {
        Ok(s) => s,
        Err(hr) => return hr,
    };
    if !this.is_null() && (ComObj::inner(this).flags & DML_CREATE_DEVICE_FLAG_DEBUG) != 0 {
        log::debug!(target: "d3d", "IDMLDevice::CreateOperator({spec:?})");
    }
    let obj = ComObj::new(
        operator_vtable(),
        DmlChild::new(this as *mut c_void, DmlOperatorData { spec: Arc::new(spec) }),
    );
    let hr = com_call_qi(obj as *mut c_void, &iid, ppv);
    ComObj::release(obj);
    hr
}

// Slot 9: CompileOperator
unsafe extern "win64" fn dev_compile_operator(
    this: *mut DmlDeviceObj,
    op_ptr: *mut c_void,
    _flags: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let Some(op) = operator_from_ptr(op_ptr) else {
        return E_INVALIDARG;
    };
    let spec = &op.inner.inner.spec;
    let (persistent_inputs, persistent_size) = spec.persistent_layout();
    let req_descs = (spec.input_descs().len() + spec.output_descs().len()).max(1) as u32;
    let state = Arc::new(CompiledOpState {
        plan: ExecutionPlan::Single((**spec).clone()),
        binding_props: DML_BINDING_PROPERTIES {
            RequiredDescriptorCount: req_descs,
            _pad: 0,
            TemporaryResourceSize: 0,
            PersistentResourceSize: persistent_size,
        },
        persistent_inputs,
    });
    let obj = ComObj::new(
        compiled_vtable(),
        DmlChild::new(this as *mut c_void, DmlCompiledData { compiled: state }),
    );
    let hr = com_call_qi(obj as *mut c_void, &iid, ppv);
    ComObj::release(obj);
    hr
}

// Slot 10: CreateOperatorInitializer
unsafe extern "win64" fn dev_create_operator_initializer(
    this: *mut DmlDeviceObj,
    op_count: u32,
    ops_ptr: *const *mut c_void,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if op_count > 0 && ops_ptr.is_null() {
        return E_INVALIDARG;
    }
    let mut ops = Vec::with_capacity(op_count as usize);
    for i in 0..op_count as usize {
        let p = rd(ops_ptr.add(i));
        let Some(co) = compiled_from_ptr(p) else {
            return E_INVALIDARG;
        };
        ops.push(co.inner.inner.compiled.clone());
    }
    let obj = ComObj::new(
        initializer_vtable(),
        DmlChild::new(this as *mut c_void, DmlInitializerData { ops: Mutex::new(ops) }),
    );
    let hr = com_call_qi(obj as *mut c_void, &iid, ppv);
    ComObj::release(obj);
    hr
}

// Slot 11: CreateCommandRecorder
unsafe extern "win64" fn dev_create_command_recorder(
    this: *mut DmlDeviceObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = ComObj::new(
        recorder_vtable(),
        DmlChild::new(this as *mut c_void, DmlCommandRecorderData),
    );
    let hr = com_call_qi(obj as *mut c_void, &iid, ppv);
    ComObj::release(obj);
    hr
}

// Slot 12: CreateBindingTable
unsafe extern "win64" fn dev_create_binding_table(
    this: *mut DmlDeviceObj,
    desc: *const DML_BINDING_TABLE_DESC,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let obj = ComObj::new(
        binding_table_vtable(),
        DmlChild::new(
            this as *mut c_void,
            DmlBindingTableData {
                state: Mutex::new(BindingTableState {
                    target_ptr: 0,
                    cpu_handle: 0,
                    gpu_handle: 0,
                    size_in_descriptors: 0,
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    temp: BindingSlot::None,
                    persistent: BindingSlot::None,
                }),
            },
        ),
    );
    let rhr = bt_reset(obj, desc);
    if rhr < 0 {
        ComObj::release(obj);
        return rhr;
    }
    let hr = com_call_qi(obj as *mut c_void, &iid, ppv);
    ComObj::release(obj);
    hr
}

// Slot 13: Evict
unsafe extern "win64" fn dev_evict(
    _this: *mut DmlDeviceObj,
    count: u32,
    objects: *const *mut c_void,
) -> HRESULT {
    if count > 0 && objects.is_null() {
        return E_INVALIDARG;
    }
    S_OK
}

// Slot 14: MakeResident
unsafe extern "win64" fn dev_make_resident(
    _this: *mut DmlDeviceObj,
    count: u32,
    objects: *const *mut c_void,
) -> HRESULT {
    if count > 0 && objects.is_null() {
        return E_INVALIDARG;
    }
    S_OK
}

// Slot 15: GetDeviceRemovedReason
unsafe extern "win64" fn dev_get_device_removed_reason(this: *mut DmlDeviceObj) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(d3d) = ComObj::inner(this).d3d12_device.as_ref() else {
        return S_OK;
    };
    // ID3D12Device::GetDeviceRemovedReason is vtable slot 37.
    com_call_slot(d3d.as_ptr(), 37, 0, 0, 0)
}

// Slot 16: GetParentDevice
unsafe extern "win64" fn dev_get_parent_device(
    this: *mut DmlDeviceObj,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(d3d) = ComObj::inner(this).d3d12_device.as_ref() else {
        return E_FAIL;
    };
    com_call_qi(d3d.as_ptr(), &iid, ppv)
}

// Slot 17 (IDMLDevice1): CompileGraph
unsafe extern "win64" fn dev_compile_graph(
    this: *mut DmlDeviceObj,
    desc_ptr: *const DML_GRAPH_DESC,
    _flags: u32,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    let (graph_plan, req_descs) = match parse_graph_plan(desc_ptr, |op_ptr| {
        operator_from_ptr(op_ptr).map(|op| (*op.inner.inner.spec).clone())
    }) {
        Ok(res) => res,
        Err(hr) => return hr,
    };
    let compiled = Arc::new(CompiledOpState {
        plan: ExecutionPlan::Graph(graph_plan),
        binding_props: DML_BINDING_PROPERTIES {
            RequiredDescriptorCount: req_descs,
            _pad: 0,
            TemporaryResourceSize: 0,
            PersistentResourceSize: 0,
        },
        persistent_inputs: Vec::new(),
    });
    let obj = ComObj::new(
        compiled_vtable(),
        DmlChild::new(this as *mut c_void, DmlCompiledData { compiled }),
    );
    let hr = com_call_qi(obj as *mut c_void, &iid, ppv);
    ComObj::release(obj);
    hr
}

fn device_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 18]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            dev_qi as *const () as usize,
            com_add_ref::<DmlDeviceState> as *const () as usize,
            com_release::<DmlDeviceState> as *const () as usize,
            dml_get_private_data::<DmlDeviceState> as *const () as usize,
            dml_set_private_data::<DmlDeviceState> as *const () as usize,
            dml_set_private_data_interface::<DmlDeviceState> as *const () as usize,
            dml_set_name::<DmlDeviceState> as *const () as usize,
            dev_check_feature_support as *const () as usize,
            dev_create_operator as *const () as usize,
            dev_compile_operator as *const () as usize,
            dev_create_operator_initializer as *const () as usize,
            dev_create_command_recorder as *const () as usize,
            dev_create_binding_table as *const () as usize,
            dev_evict as *const () as usize,
            dev_make_resident as *const () as usize,
            dev_get_device_removed_reason as *const () as usize,
            dev_get_parent_device as *const () as usize,
            dev_compile_graph as *const () as usize,
        ]
    })
}

/// Construct a new `IDMLDevice1` object wrapping `d3d12_device` and query `iid` into `ppv`.
pub(super) unsafe fn create_dml_device(
    d3d12_device: *mut c_void,
    flags: u32,
    iid: &Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let obj = ComObj::new(
        device_vtable(),
        DmlDeviceState {
            private: Mutex::new(PrivateData::new()),
            d3d12_device: ComPtr::retain(d3d12_device),
            flags,
        },
    );
    let hr = com_call_qi(obj as *mut c_void, iid, ppv);
    ComObj::release(obj);
    hr
}
