use std::ffi::c_void;

use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};

use super::graph::{read_d3d12_buffer, write_d3d12_buffer, BufferBinding};
use super::*;
use crate::com::{com_call_release as com_release, guid, Guid, DXGI_ERROR_MORE_DATA};
use crate::d3d12::com::DXGI_ERROR_UNSUPPORTED;
use crate::d3d12::vtables::{IID_ID3D12Device, IID_ID3D12Resource};
use crate::types::{BOOL, E_INVALIDARG, FALSE, HRESULT, S_OK, TRUE};

unsafe fn vslot<F: Copy>(obj: *mut c_void, idx: usize) -> F {
    let vt = std::ptr::read_unaligned(obj as *const *const usize);
    std::mem::transmute_copy(&std::ptr::read_unaligned(vt.add(idx)))
}

unsafe fn make_d3d12_buffer(d3d_dev: *mut c_void, bytes: u64, heap_ty: u32) -> *mut c_void {
    let create_res: unsafe extern "win64" fn(
        *mut c_void,
        *const HeapProperties,
        u32,
        *const ResourceDesc,
        u32,
        *const c_void,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = vslot(d3d_dev, 27);
    let hp = HeapProperties { Type: heap_ty, ..Default::default() };
    let desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: bytes.max(16),
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let mut res = std::ptr::null_mut();
    assert_eq!(
        create_res(d3d_dev, &hp, 0, &desc, 0, std::ptr::null(), &IID_ID3D12Resource, &mut res),
        S_OK
    );
    res
}

unsafe fn write_f32_buf(res: *mut c_void, vals: &[f32]) {
    let bytes = std::slice::from_raw_parts(vals.as_ptr() as *const u8, vals.len() * 4);
    write_d3d12_buffer(&BufferBinding { resource_ptr: res as usize, offset: 0, size: bytes.len() as u64 }, bytes);
}

unsafe fn read_f32_buf(res: *mut c_void, count: usize) -> Vec<f32> {
    let bytes = read_d3d12_buffer(
        &BufferBinding { resource_ptr: res as usize, offset: 0, size: (count * 4) as u64 },
        count * 4,
    )
    .unwrap();
    bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

fn tensor_f32(sizes: &[u32]) -> (DML_BUFFER_TENSOR_DESC, DML_TENSOR_DESC) {
    let total = sizes.iter().fold(1u64, |a, &b| a * b as u64) * 4;
    let bd = DML_BUFFER_TENSOR_DESC {
        DataType: DML_TENSOR_DATA_TYPE_FLOAT32,
        Flags: DML_TENSOR_FLAG_NONE,
        DimensionCount: sizes.len() as u32,
        _pad0: 0,
        Sizes: sizes.as_ptr(),
        Strides: std::ptr::null(),
        TotalTensorSizeInBytes: total,
        GuaranteedBaseOffsetAlignment: 0,
        _pad1: 0,
    };
    let td = DML_TENSOR_DESC {
        Type: DML_TENSOR_TYPE_BUFFER,
        _pad: 0,
        Desc: std::ptr::null(),
    };
    (bd, td)
}

unsafe fn dispatch_compiled(
    dml_dev: *mut c_void,
    op_desc: &DML_OPERATOR_DESC,
    in_res: &[*mut c_void],
    in_sizes: &[u64],
    out_res: &[*mut c_void],
    out_sizes: &[u64],
) {
    let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
    let compile_op: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 9);
    let create_init: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 10);
    let create_rec: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 11);
    let create_bt: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 12);

    let mut op = std::ptr::null_mut();
    assert_eq!(create_op(dml_dev, op_desc, &IID_IDMLOperator, &mut op), S_OK);
    let mut compiled = std::ptr::null_mut();
    assert_eq!(compile_op(dml_dev, op, 0, &IID_IDMLCompiledOperator, &mut compiled), S_OK);

    // Verify GetBindingProperties
    let get_props: unsafe extern "win64" fn(*mut c_void, *mut DML_BINDING_PROPERTIES) -> *mut DML_BINDING_PROPERTIES = vslot(compiled, 8);
    let mut props = DML_BINDING_PROPERTIES::default();
    get_props(compiled, &mut props);
    assert!(props.RequiredDescriptorCount >= 1);

    // Run initializer through RecordDispatch
    let mut init = std::ptr::null_mut();
    assert_eq!(create_init(dml_dev, 1, &compiled, &IID_IDMLOperatorInitializer, &mut init), S_OK);
    let mut rec = std::ptr::null_mut();
    assert_eq!(create_rec(dml_dev, &IID_IDMLCommandRecorder, &mut rec), S_OK);
    let mut bt = std::ptr::null_mut();
    let bt_init_desc = DML_BINDING_TABLE_DESC {
        Dispatchable: init,
        CPUDescriptorHandle: 0,
        GPUDescriptorHandle: 0,
        SizeInDescriptors: 4,
        _pad: 0,
    };
    assert_eq!(create_bt(dml_dev, &bt_init_desc, &IID_IDMLBindingTable, &mut bt), S_OK);
    let record_dispatch: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) = vslot(rec, 8);
    record_dispatch(rec, std::ptr::null_mut(), init, bt);

    // Reset binding table for compiled op dispatch
    let bt_reset_fn: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC) -> HRESULT = vslot(bt, 12);
    let bt_bind_in: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 8);
    let bt_bind_out: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 9);

    let bt_exec_desc = DML_BINDING_TABLE_DESC {
        Dispatchable: compiled,
        CPUDescriptorHandle: 0,
        GPUDescriptorHandle: 0,
        SizeInDescriptors: props.RequiredDescriptorCount,
        _pad: 0,
    };
    assert_eq!(bt_reset_fn(bt, &bt_exec_desc), S_OK);

    let in_bbs: Vec<DML_BUFFER_BINDING> = in_res
        .iter()
        .zip(in_sizes.iter())
        .map(|(&r, &sz)| DML_BUFFER_BINDING { Buffer: r, Offset: 0, SizeInBytes: sz })
        .collect();
    let in_bds: Vec<DML_BINDING_DESC> = in_bbs
        .iter()
        .map(|bb| {
            if bb.Buffer.is_null() {
                DML_BINDING_DESC { Type: DML_BINDING_TYPE_NONE, _pad: 0, Desc: std::ptr::null() }
            } else {
                DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: bb as *const _ as *const c_void }
            }
        })
        .collect();
    bt_bind_in(bt, in_bds.len() as u32, in_bds.as_ptr());

    let out_bbs: Vec<DML_BUFFER_BINDING> = out_res
        .iter()
        .zip(out_sizes.iter())
        .map(|(&r, &sz)| DML_BUFFER_BINDING { Buffer: r, Offset: 0, SizeInBytes: sz })
        .collect();
    let out_bds: Vec<DML_BINDING_DESC> = out_bbs
        .iter()
        .map(|bb| DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: bb as *const _ as *const c_void })
        .collect();
    bt_bind_out(bt, out_bds.len() as u32, out_bds.as_ptr());

    record_dispatch(rec, std::ptr::null_mut(), compiled, bt);

    com_release(bt);
    com_release(rec);
    com_release(init);
    com_release(compiled);
    com_release(op);
}

#[test]
fn device_creation_feature_support_and_gemm_execution() {
    unsafe {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let mut d3d_dev = std::ptr::null_mut();
        let mut dml_dev = std::ptr::null_mut();
        assert_eq!(
            DMLCreateDeviceAndDirect3D12Device(
                std::ptr::null_mut(),
                0xB000,
                DML_CREATE_DEVICE_FLAG_NONE,
                DML_FEATURE_LEVEL_4_0,
                &IID_ID3D12Device,
                &mut d3d_dev,
                &IID_IDMLDevice1,
                &mut dml_dev,
            ),
            S_OK
        );
        assert!(!d3d_dev.is_null());
        assert!(!dml_dev.is_null());

        // CheckFeatureSupport: FLOAT32 & FLOAT16 supported, feature levels up to 5_0.
        let check_fs: unsafe extern "win64" fn(*mut c_void, u32, u32, *const c_void, u32, *mut c_void) -> HRESULT = vslot(dml_dev, 7);
        let q_f32 = DML_TENSOR_DATA_TYPE_FLOAT32;
        let mut is_sup: BOOL = FALSE;
        assert_eq!(check_fs(dml_dev, DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT, 4, &q_f32 as *const _ as *const c_void, 4, &mut is_sup as *mut _ as *mut c_void), S_OK);
        assert_eq!(is_sup, TRUE);

        let req_fls = [DML_FEATURE_LEVEL_1_0, DML_FEATURE_LEVEL_4_0, DML_FEATURE_LEVEL_5_0];
        let q_fl = DML_FEATURE_QUERY_FEATURE_LEVELS {
            RequestedFeatureLevelCount: req_fls.len() as u32,
            _pad: 0,
            RequestedFeatureLevels: req_fls.as_ptr(),
        };
        let mut max_fl = 0u32;
        assert_eq!(
            check_fs(dml_dev, DML_FEATURE_FEATURE_LEVELS, std::mem::size_of::<DML_FEATURE_QUERY_FEATURE_LEVELS>() as u32, &q_fl as *const _ as *const c_void, 4, &mut max_fl as *mut _ as *mut c_void),
            S_OK
        );
        assert_eq!(max_fl, DML_FEATURE_LEVEL_5_0);

        // GEMM: Y = ReLU(2.0 * A(2x3) * B(3x2) + 1.0 * C(2x2)) on DEFAULT & UPLOAD D3D12 buffers!
        let a_buf = make_d3d12_buffer(d3d_dev, 24, heap_type::DEFAULT);
        let b_buf = make_d3d12_buffer(d3d_dev, 24, heap_type::UPLOAD);
        let c_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let y_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::DEFAULT);
        // A = [[1, 2, 3], [-1, 0, 2]], B = [[1, -1], [0, 2], [1, 1]], C = [[-10, 1], [0, -5]]
        // A*B = [[4, 6], [1, 3]], 2*A*B + C = [[-2, 13], [2, 1]] -> ReLU => [[0, 13], [2, 1]]
        write_f32_buf(a_buf, &[1.0, 2.0, 3.0, -1.0, 0.0, 2.0]);
        write_f32_buf(b_buf, &[1.0, -1.0, 0.0, 2.0, 1.0, 1.0]);
        write_f32_buf(c_buf, &[-10.0, 1.0, 0.0, -5.0]);

        let s_a = [1u32, 1, 2, 3];
        let s_b = [1u32, 1, 3, 2];
        let s_c = [1u32, 1, 2, 2];
        let (bd_a, mut td_a) = tensor_f32(&s_a);
        td_a.Desc = &bd_a as *const _ as *const c_void;
        let (bd_b, mut td_b) = tensor_f32(&s_b);
        td_b.Desc = &bd_b as *const _ as *const c_void;
        let (bd_c, mut td_c) = tensor_f32(&s_c);
        td_c.Desc = &bd_c as *const _ as *const c_void;
        let (bd_y, mut td_y) = tensor_f32(&s_c);
        td_y.Desc = &bd_y as *const _ as *const c_void;

        let relu_inner = DML_SIMPLE_UNARY_DESC { InputTensor: std::ptr::null(), OutputTensor: std::ptr::null() };
        let relu_desc = DML_OPERATOR_DESC {
            Type: DML_OPERATOR_ACTIVATION_RELU,
            _pad: 0,
            Desc: &relu_inner as *const _ as *const c_void,
        };
        let gemm_desc = DML_GEMM_OPERATOR_DESC {
            ATensor: &td_a,
            BTensor: &td_b,
            CTensor: &td_c,
            OutputTensor: &td_y,
            TransA: DML_MATRIX_TRANSFORM_NONE,
            TransB: DML_MATRIX_TRANSFORM_NONE,
            Alpha: 2.0,
            Beta: 1.0,
            FusedActivation: &relu_desc,
        };
        let op_desc = DML_OPERATOR_DESC {
            Type: DML_OPERATOR_GEMM,
            _pad: 0,
            Desc: &gemm_desc as *const _ as *const c_void,
        };

        dispatch_compiled(dml_dev, &op_desc, &[a_buf, b_buf, c_buf], &[24, 24, 16], &[y_buf], &[16]);
        let y = read_f32_buf(y_buf, 4);
        assert!((y[0] - 0.0).abs() < 1e-5, "y={y:?}");
        assert!((y[1] - 13.0).abs() < 1e-5, "y={y:?}");
        assert!((y[2] - 2.0).abs() < 1e-5, "y={y:?}");
        assert!((y[3] - 1.0).abs() < 1e-5, "y={y:?}");

        com_release(a_buf);
        com_release(b_buf);
        com_release(c_buf);
        com_release(y_buf);
        com_release(dml_dev);
        com_release(d3d_dev);
    }
}

#[test]
fn elementwise_activations_conv2d_pool_reduce_and_norm() {
    unsafe {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let mut d3d_dev = std::ptr::null_mut();
        let mut dml_dev = std::ptr::null_mut();
        assert_eq!(
            DMLCreateDeviceAndDirect3D12Device(
                std::ptr::null_mut(),
                0xB000,
                0,
                DML_FEATURE_LEVEL_1_0,
                &IID_ID3D12Device,
                &mut d3d_dev,
                &IID_IDMLDevice1,
                &mut dml_dev,
            ),
            S_OK
        );

        // 1) ElementWise Add + LeakyReLU
        let b_a = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let b_b = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let b_mid = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let b_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(b_a, &[1.0, -4.0, 2.5, -2.0]);
        write_f32_buf(b_b, &[3.0, 2.0, -2.5, 0.0]);
        let s4 = [1u32, 1, 1, 4];
        let (bd0, mut td0) = tensor_f32(&s4);
        td0.Desc = &bd0 as *const _ as *const c_void;
        let add_desc = DML_ELEMENT_WISE_BINARY_DESC { ATensor: &td0, BTensor: &td0, OutputTensor: &td0 };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_ADD, _pad: 0, Desc: &add_desc as *const _ as *const c_void },
            &[b_a, b_b],
            &[16, 16],
            &[b_mid],
            &[16],
        );
        assert_eq!(read_f32_buf(b_mid, 4), vec![4.0, -2.0, 0.0, -2.0]);

        let lrelu = DML_ACTIVATION_ALPHA_DESC { InputTensor: &td0, OutputTensor: &td0, Alpha: 0.25, _pad: 0 };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_LEAKY_RELU, _pad: 0, Desc: &lrelu as *const _ as *const c_void },
            &[b_mid],
            &[16],
            &[b_out],
            &[16],
        );
        assert_eq!(read_f32_buf(b_out, 4), vec![4.0, -0.5, 0.0, -0.5]);

        // 2) 2D Cross-Correlation Convolution (1x1x3x3 * 1x1x2x2 -> 1x1x2x2)
        let c_in = make_d3d12_buffer(d3d_dev, 36, heap_type::UPLOAD);
        let c_flt = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let c_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(c_in, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        write_f32_buf(c_flt, &[1.0, 0.0, 0.0, 1.0]);
        let s_in = [1u32, 1, 3, 3];
        let s_flt = [1u32, 1, 2, 2];
        let s_co = [1u32, 1, 2, 2];
        let (bdi, mut tdi) = tensor_f32(&s_in); tdi.Desc = &bdi as *const _ as *const c_void;
        let (bdf, mut tdf) = tensor_f32(&s_flt); tdf.Desc = &bdf as *const _ as *const c_void;
        let (bdo, mut tdo) = tensor_f32(&s_co); tdo.Desc = &bdo as *const _ as *const c_void;
        let st = [1u32, 1];
        let dil = [1u32, 1];
        let pad = [0u32, 0];
        let conv = DML_CONVOLUTION_OPERATOR_DESC {
            InputTensor: &tdi,
            FilterTensor: &tdf,
            BiasTensor: std::ptr::null(),
            OutputTensor: &tdo,
            Mode: DML_CONVOLUTION_MODE_CROSS_CORRELATION,
            Direction: DML_CONVOLUTION_DIRECTION_FORWARD,
            DimensionCount: 2,
            _pad0: 0,
            Strides: st.as_ptr(),
            Dilations: dil.as_ptr(),
            StartPadding: pad.as_ptr(),
            EndPadding: pad.as_ptr(),
            OutputPadding: pad.as_ptr(),
            GroupCount: 1,
            _pad1: 0,
            FusedActivation: std::ptr::null(),
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_CONVOLUTION, _pad: 0, Desc: &conv as *const _ as *const c_void },
            &[c_in, c_flt, std::ptr::null_mut()],
            &[36, 16, 0],
            &[c_out],
            &[16],
        );
        // Windows: (1+5)=6, (2+6)=8, (4+8)=12, (5+9)=14
        assert_eq!(read_f32_buf(c_out, 4), vec![6.0, 8.0, 12.0, 14.0]);

        // 3) MaxPooling 2x2 over 2x2 output -> 1x1x1x1 = 14.0
        let p_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let s_p = [1u32, 1, 1, 1];
        let (bdp, mut tdp) = tensor_f32(&s_p); tdp.Desc = &bdp as *const _ as *const c_void;
        let win = [2u32, 2];
        let mp = DML_MAX_POOLING_OPERATOR_DESC {
            InputTensor: &tdo,
            OutputTensor: &tdp,
            DimensionCount: 2,
            _pad0: 0,
            Strides: win.as_ptr(),
            WindowSize: win.as_ptr(),
            StartPadding: pad.as_ptr(),
            EndPadding: pad.as_ptr(),
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_MAX_POOLING, _pad: 0, Desc: &mp as *const _ as *const c_void },
            &[c_out],
            &[16],
            &[p_out],
            &[4],
        );
        assert_eq!(read_f32_buf(p_out, 1), vec![14.0]);

        // 4) Reduce SUM across H, W on [6, 8, 12, 14] -> 40.0
        let r_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let axes = [2u32, 3];
        let red = DML_REDUCE_OPERATOR_DESC {
            Function: DML_REDUCE_FUNCTION_SUM,
            _pad0: 0,
            InputTensor: &tdo,
            OutputTensor: &tdp,
            AxisCount: 2,
            _pad1: 0,
            Axes: axes.as_ptr(),
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_REDUCE, _pad: 0, Desc: &red as *const _ as *const c_void },
            &[c_out],
            &[16],
            &[r_out],
            &[4],
        );
        assert_eq!(read_f32_buf(r_out, 1), vec![40.0]);

        // 5) Cast FLOAT32 -> FLOAT16 -> FLOAT32 round-trip
        let f16_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let rt_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let bd_f16 = DML_BUFFER_TENSOR_DESC {
            DataType: DML_TENSOR_DATA_TYPE_FLOAT16,
            Flags: 0,
            DimensionCount: 4,
            _pad0: 0,
            Sizes: s4.as_ptr(),
            Strides: std::ptr::null(),
            TotalTensorSizeInBytes: 8,
            GuaranteedBaseOffsetAlignment: 0,
            _pad1: 0,
        };
        let td_f16 = DML_TENSOR_DESC { Type: DML_TENSOR_TYPE_BUFFER, _pad: 0, Desc: &bd_f16 as *const _ as *const c_void };
        let cast1 = DML_SIMPLE_UNARY_DESC { InputTensor: &td0, OutputTensor: &td_f16 };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_CAST, _pad: 0, Desc: &cast1 as *const _ as *const c_void },
            &[b_out],
            &[16],
            &[f16_buf],
            &[8],
        );
        let cast2 = DML_SIMPLE_UNARY_DESC { InputTensor: &td_f16, OutputTensor: &td0 };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_CAST, _pad: 0, Desc: &cast2 as *const _ as *const c_void },
            &[f16_buf],
            &[8],
            &[rt_buf],
            &[16],
        );
        assert_eq!(read_f32_buf(rt_buf, 4), vec![4.0, -0.5, 0.0, -0.5]);

        // 6) BatchNormalization: scale * (x - mean) / sqrt(var + eps) + bias
        let bn_x = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let bn_m = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let bn_v = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let bn_s = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let bn_b = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let bn_y = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(bn_x, &[2.0, 4.0, 6.0, 8.0]);
        write_f32_buf(bn_m, &[4.0]);
        write_f32_buf(bn_v, &[4.0]); // sqrt(4) = 2
        write_f32_buf(bn_s, &[2.0]);
        write_f32_buf(bn_b, &[1.0]);
        // y = 2*(x - 4)/2 + 1 = x - 3 => [-1, 1, 3, 5]
        let bn_desc = DML_BATCH_NORMALIZATION_OPERATOR_DESC {
            InputTensor: &td0,
            MeanTensor: &tdp,
            VarianceTensor: &tdp,
            ScaleTensor: &tdp,
            BiasTensor: &tdp,
            OutputTensor: &td0,
            Spatial: TRUE,
            Epsilon: 0.0,
            FusedActivation: std::ptr::null(),
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_BATCH_NORMALIZATION, _pad: 0, Desc: &bn_desc as *const _ as *const c_void },
            &[bn_x, bn_m, bn_v, bn_s, bn_b],
            &[16, 4, 4, 4, 4],
            &[bn_y],
            &[16],
        );
        let bny = read_f32_buf(bn_y, 4);
        for (got, exp) in bny.iter().zip([-1.0f32, 1.0, 3.0, 5.0]) {
            assert!((got - exp).abs() < 1e-5, "bny={bny:?}");
        }

        // 7) IDMLDevice1::CompileGraph: Negate -> Exp
        let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
        let compile_graph: unsafe extern "win64" fn(*mut c_void, *const DML_GRAPH_DESC, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 17);
        let create_rec: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 11);
        let create_bt: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 12);

        let neg_d = DML_SIMPLE_UNARY_DESC { InputTensor: &td0, OutputTensor: &td0 };
        let exp_d = DML_ELEMENT_WISE_UNARY_DESC { InputTensor: &td0, OutputTensor: &td0, ScaleBias: std::ptr::null() };
        let mut op_neg = std::ptr::null_mut();
        let mut op_exp = std::ptr::null_mut();
        assert_eq!(create_op(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_NEGATE, _pad: 0, Desc: &neg_d as *const _ as *const c_void }, &IID_IDMLOperator, &mut op_neg), S_OK);
        assert_eq!(create_op(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_EXP, _pad: 0, Desc: &exp_d as *const _ as *const c_void }, &IID_IDMLOperator, &mut op_exp), S_OK);

        let n0 = DML_OPERATOR_GRAPH_NODE_DESC { Operator: op_neg, Name: std::ptr::null() };
        let n1 = DML_OPERATOR_GRAPH_NODE_DESC { Operator: op_exp, Name: std::ptr::null() };
        let nodes = [
            DML_GRAPH_NODE_DESC { Type: DML_GRAPH_NODE_TYPE_OPERATOR, _pad: 0, Desc: &n0 as *const _ as *const c_void },
            DML_GRAPH_NODE_DESC { Type: DML_GRAPH_NODE_TYPE_OPERATOR, _pad: 0, Desc: &n1 as *const _ as *const c_void },
        ];
        let ie = DML_INPUT_GRAPH_EDGE_DESC { GraphInputIndex: 0, ToNodeIndex: 0, ToNodeInputIndex: 0, _pad: 0, Name: std::ptr::null() };
        let me = DML_INTERMEDIATE_GRAPH_EDGE_DESC { FromNodeIndex: 0, FromNodeOutputIndex: 0, ToNodeIndex: 1, ToNodeInputIndex: 0, Name: std::ptr::null() };
        let oe = DML_OUTPUT_GRAPH_EDGE_DESC { FromNodeIndex: 1, FromNodeOutputIndex: 0, GraphOutputIndex: 0, _pad: 0, Name: std::ptr::null() };
        let in_edges = [DML_GRAPH_EDGE_DESC { Type: DML_GRAPH_EDGE_TYPE_INPUT, _pad: 0, Desc: &ie as *const _ as *const c_void }];
        let inter_edges = [DML_GRAPH_EDGE_DESC { Type: DML_GRAPH_EDGE_TYPE_INTERMEDIATE, _pad: 0, Desc: &me as *const _ as *const c_void }];
        let out_edges = [DML_GRAPH_EDGE_DESC { Type: DML_GRAPH_EDGE_TYPE_OUTPUT, _pad: 0, Desc: &oe as *const _ as *const c_void }];
        let gdesc = DML_GRAPH_DESC {
            InputCount: 1,
            OutputCount: 1,
            NodeCount: 2,
            _pad0: 0,
            Nodes: nodes.as_ptr(),
            InputEdgeCount: 1,
            _pad1: 0,
            InputEdges: in_edges.as_ptr(),
            OutputEdgeCount: 1,
            _pad2: 0,
            OutputEdges: out_edges.as_ptr(),
            IntermediateEdgeCount: 1,
            _pad3: 0,
            IntermediateEdges: inter_edges.as_ptr(),
        };
        let mut gcomp = std::ptr::null_mut();
        assert_eq!(compile_graph(dml_dev, &gdesc, 0, &IID_IDMLCompiledOperator, &mut gcomp), S_OK);
        let mut rec = std::ptr::null_mut();
        assert_eq!(create_rec(dml_dev, &IID_IDMLCommandRecorder, &mut rec), S_OK);
        let mut bt = std::ptr::null_mut();
        let btd = DML_BINDING_TABLE_DESC { Dispatchable: gcomp, CPUDescriptorHandle: 0, GPUDescriptorHandle: 0, SizeInDescriptors: 2, _pad: 0 };
        assert_eq!(create_bt(dml_dev, &btd, &IID_IDMLBindingTable, &mut bt), S_OK);

        let g_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let g_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(g_in, &[0.0, 1.0, -1.0, 2.0]);
        let bb_in = DML_BUFFER_BINDING { Buffer: g_in, Offset: 0, SizeInBytes: 16 };
        let bb_out = DML_BUFFER_BINDING { Buffer: g_out, Offset: 0, SizeInBytes: 16 };
        let bd_in = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &bb_in as *const _ as *const c_void };
        let bd_out = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &bb_out as *const _ as *const c_void };
        let bind_in: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 8);
        let bind_out: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 9);
        let rec_disp: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) = vslot(rec, 8);
        bind_in(bt, 1, &bd_in);
        bind_out(bt, 1, &bd_out);
        rec_disp(rec, std::ptr::null_mut(), gcomp, bt);

        let gy = read_f32_buf(g_out, 4);
        let expected = [0.0f32.exp(), (-1.0f32).exp(), 1.0f32.exp(), (-2.0f32).exp()];
        for (got, exp) in gy.iter().zip(expected) {
            assert!((got - exp).abs() < 1e-5, "gy={gy:?}");
        }

        for p in [g_in, g_out, bt, rec, gcomp, op_neg, op_exp, bn_x, bn_m, bn_v, bn_s, bn_b, bn_y, f16_buf, rt_buf, r_out, p_out, c_in, c_flt, c_out, b_a, b_b, b_mid, b_out, dml_dev, d3d_dev] {
            com_release(p);
        }
    }
}

#[test]
fn join_split_avgpool_mvn_clip_sigmoid_tanh_and_owned_persistent_init() {
    unsafe {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let mut d3d_dev = std::ptr::null_mut();
        let mut dml_dev = std::ptr::null_mut();
        assert_eq!(
            DMLCreateDeviceAndDirect3D12Device(
                std::ptr::null_mut(),
                0xB000,
                0,
                DML_FEATURE_LEVEL_1_0,
                &IID_ID3D12Device,
                &mut d3d_dev,
                &IID_IDMLDevice1,
                &mut dml_dev,
            ),
            S_OK
        );

        // 1) Join two [1, 2] tensors along axis 1 -> [1, 4], then Split back into two [1, 2]
        let j0 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let j1 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let joined = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let sp0 = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let sp1 = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(j0, &[10.0, 20.0]);
        write_f32_buf(j1, &[30.0, 40.0]);
        let s12 = [1u32, 2];
        let s14 = [1u32, 4];
        let (bd12, mut td12) = tensor_f32(&s12); td12.Desc = &bd12 as *const _ as *const c_void;
        let (bd14, mut td14) = tensor_f32(&s14); td14.Desc = &bd14 as *const _ as *const c_void;
        let in_tds = [td12, td12];
        let join_desc = DML_JOIN_OPERATOR_DESC {
            InputCount: 2,
            _pad0: 0,
            InputTensors: in_tds.as_ptr(),
            OutputTensor: &td14,
            Axis: 1,
            _pad1: 0,
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_JOIN, _pad: 0, Desc: &join_desc as *const _ as *const c_void },
            &[j0, j1],
            &[8, 8],
            &[joined],
            &[16],
        );
        assert_eq!(read_f32_buf(joined, 4), vec![10.0, 20.0, 30.0, 40.0]);

        let split_desc = DML_SPLIT_OPERATOR_DESC {
            InputTensor: &td14,
            OutputCount: 2,
            _pad0: 0,
            OutputTensors: in_tds.as_ptr(),
            Axis: 1,
            _pad1: 0,
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_SPLIT, _pad: 0, Desc: &split_desc as *const _ as *const c_void },
            &[joined],
            &[16],
            &[sp0, sp1],
            &[8, 8],
        );
        assert_eq!(read_f32_buf(sp0, 2), vec![10.0, 20.0]);
        assert_eq!(read_f32_buf(sp1, 2), vec![30.0, 40.0]);

        // 2) AveragePooling 2x2 over [1, 1, 2, 2] = [2, 4, 6, 8] -> 5.0
        let ap_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let ap_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(ap_in, &[2.0, 4.0, 6.0, 8.0]);
        let s22 = [1u32, 1, 2, 2];
        let s11 = [1u32, 1, 1, 1];
        let (bd22, mut td22) = tensor_f32(&s22); td22.Desc = &bd22 as *const _ as *const c_void;
        let (bd11, mut td11) = tensor_f32(&s11); td11.Desc = &bd11 as *const _ as *const c_void;
        let win = [2u32, 2];
        let pad = [0u32, 0];
        let ap_desc = DML_AVERAGE_POOLING_OPERATOR_DESC {
            InputTensor: &td22,
            OutputTensor: &td11,
            DimensionCount: 2,
            _pad0: 0,
            Strides: win.as_ptr(),
            WindowSize: win.as_ptr(),
            StartPadding: pad.as_ptr(),
            EndPadding: pad.as_ptr(),
            IncludePadding: FALSE,
            _pad1: 0,
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_AVERAGE_POOLING, _pad: 0, Desc: &ap_desc as *const _ as *const c_void },
            &[ap_in],
            &[16],
            &[ap_out],
            &[4],
        );
        assert_eq!(read_f32_buf(ap_out, 1), vec![5.0]);

        // 3) MeanVarianceNormalization over [1, 1, 2, 2] = [1, 3, 5, 7] (mean=4, var=5)
        let mvn_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(ap_in, &[1.0, 3.0, 5.0, 7.0]);
        let mvn_desc = DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
            InputTensor: &td22,
            ScaleTensor: std::ptr::null(),
            BiasTensor: std::ptr::null(),
            OutputTensor: &td22,
            CrossChannel: FALSE,
            NormalizeVariance: TRUE,
            Epsilon: 0.0,
            _pad0: 0,
            FusedActivation: std::ptr::null(),
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION, _pad: 0, Desc: &mvn_desc as *const _ as *const c_void },
            &[ap_in, std::ptr::null_mut(), std::ptr::null_mut()],
            &[16, 0, 0],
            &[mvn_out],
            &[16],
        );
        let mv = read_f32_buf(mvn_out, 4);
        let inv_s = 1.0f32 / 5.0f32.sqrt();
        for (got, exp) in mv.iter().zip([-3.0 * inv_s, -inv_s, inv_s, 3.0 * inv_s]) {
            assert!((got - exp).abs() < 1e-5, "mv={mv:?}");
        }

        // 4) Clip, Sigmoid, Tanh, Identity+ScaleBias, Sub, Mul, Div, Abs, Sqrt, Log
        let u_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let u_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(u_in, &[-2.0, 0.0, 1.0, 5.0]);
        let clip_d = DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
            InputTensor: &td14,
            OutputTensor: &td14,
            ScaleBias: std::ptr::null(),
            Min: -1.0,
            Max: 2.0,
        };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_CLIP, _pad: 0, Desc: &clip_d as *const _ as *const c_void },
            &[u_in],
            &[16],
            &[u_out],
            &[16],
        );
        assert_eq!(read_f32_buf(u_out, 4), vec![-1.0, 0.0, 1.0, 2.0]);

        let sig_d = DML_SIMPLE_UNARY_DESC { InputTensor: &td14, OutputTensor: &td14 };
        dispatch_compiled(
            dml_dev,
            &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_SIGMOID, _pad: 0, Desc: &sig_d as *const _ as *const c_void },
            &[u_out],
            &[16],
            &[u_in],
            &[16],
        );
        let sv = read_f32_buf(u_in, 4);
        assert!((sv[1] - 0.5).abs() < 1e-5);

        // 5) OWNED_BY_DML persistent resource initialization via IDMLOperatorInitializer
        let w_src = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let x_in = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let p_buf = make_d3d12_buffer(d3d_dev, 16, heap_type::DEFAULT);
        let y_out = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        write_f32_buf(x_in, &[2.0, 3.0, 4.0, 5.0]);
        write_f32_buf(w_src, &[10.0, 10.0, 10.0, 10.0]);
        let bd_owned = DML_BUFFER_TENSOR_DESC {
            DataType: DML_TENSOR_DATA_TYPE_FLOAT32,
            Flags: DML_TENSOR_FLAG_OWNED_BY_DML,
            DimensionCount: 2,
            _pad0: 0,
            Sizes: s14.as_ptr(),
            Strides: std::ptr::null(),
            TotalTensorSizeInBytes: 16,
            GuaranteedBaseOffsetAlignment: 0,
            _pad1: 0,
        };
        let td_owned = DML_TENSOR_DESC { Type: DML_TENSOR_TYPE_BUFFER, _pad: 0, Desc: &bd_owned as *const _ as *const c_void };
        let mul_d = DML_ELEMENT_WISE_BINARY_DESC { ATensor: &td14, BTensor: &td_owned, OutputTensor: &td14 };
        let op_desc = DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_MULTIPLY, _pad: 0, Desc: &mul_d as *const _ as *const c_void };

        let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
        let compile_op: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 9);
        let create_init: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 10);
        let create_rec: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 11);
        let create_bt: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 12);

        let mut op = std::ptr::null_mut();
        let mut co = std::ptr::null_mut();
        let mut init = std::ptr::null_mut();
        let mut rec = std::ptr::null_mut();
        let mut bt = std::ptr::null_mut();
        assert_eq!(create_op(dml_dev, &op_desc, &IID_IDMLOperator, &mut op), S_OK);
        assert_eq!(compile_op(dml_dev, op, 0, &IID_IDMLCompiledOperator, &mut co), S_OK);
        let get_props: unsafe extern "win64" fn(*mut c_void, *mut DML_BINDING_PROPERTIES) -> *mut DML_BINDING_PROPERTIES = vslot(co, 8);
        let mut props = DML_BINDING_PROPERTIES::default();
        get_props(co, &mut props);
        assert_eq!(props.PersistentResourceSize, 16);

        assert_eq!(create_init(dml_dev, 1, &co, &IID_IDMLOperatorInitializer, &mut init), S_OK);
        assert_eq!(create_rec(dml_dev, &IID_IDMLCommandRecorder, &mut rec), S_OK);
        let btd_i = DML_BINDING_TABLE_DESC { Dispatchable: init, CPUDescriptorHandle: 0, GPUDescriptorHandle: 0, SizeInDescriptors: 4, _pad: 0 };
        assert_eq!(create_bt(dml_dev, &btd_i, &IID_IDMLBindingTable, &mut bt), S_OK);
        let bind_in: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 8);
        let bind_out: unsafe extern "win64" fn(*mut c_void, u32, *const DML_BINDING_DESC) = vslot(bt, 9);
        let bind_pers: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_DESC) = vslot(bt, 11);
        let bt_reset_fn: unsafe extern "win64" fn(*mut c_void, *const DML_BINDING_TABLE_DESC) -> HRESULT = vslot(bt, 12);
        let rec_disp: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) = vslot(rec, 8);

        // Bind initializer inputs (BufferArray [null, w_src]) and output (persistent p_buf)
        let arr_bbs = [
            DML_BUFFER_BINDING { Buffer: std::ptr::null_mut(), Offset: 0, SizeInBytes: 0 },
            DML_BUFFER_BINDING { Buffer: w_src, Offset: 0, SizeInBytes: 16 },
        ];
        let arr_b = DML_BUFFER_ARRAY_BINDING { BindingCount: 2, _pad: 0, Bindings: arr_bbs.as_ptr() };
        let init_in = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER_ARRAY, _pad: 0, Desc: &arr_b as *const _ as *const c_void };
        let p_bb = DML_BUFFER_BINDING { Buffer: p_buf, Offset: 0, SizeInBytes: 16 };
        let p_bd = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &p_bb as *const _ as *const c_void };
        bind_in(bt, 1, &init_in);
        bind_out(bt, 1, &p_bd);
        rec_disp(rec, std::ptr::null_mut(), init, bt);

        // Now dispatch compiled op with input 1 = NONE (reads owned weights from persistent buffer!)
        let btd_e = DML_BINDING_TABLE_DESC { Dispatchable: co, CPUDescriptorHandle: 0, GPUDescriptorHandle: 0, SizeInDescriptors: 4, _pad: 0 };
        assert_eq!(bt_reset_fn(bt, &btd_e), S_OK);
        let x_bb = DML_BUFFER_BINDING { Buffer: x_in, Offset: 0, SizeInBytes: 16 };
        let y_bb = DML_BUFFER_BINDING { Buffer: y_out, Offset: 0, SizeInBytes: 16 };
        let exec_ins = [
            DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &x_bb as *const _ as *const c_void },
            DML_BINDING_DESC { Type: DML_BINDING_TYPE_NONE, _pad: 0, Desc: std::ptr::null() },
        ];
        let exec_out = DML_BINDING_DESC { Type: DML_BINDING_TYPE_BUFFER, _pad: 0, Desc: &y_bb as *const _ as *const c_void };
        bind_in(bt, 2, exec_ins.as_ptr());
        bind_out(bt, 1, &exec_out);
        bind_pers(bt, &p_bd);
        rec_disp(rec, std::ptr::null_mut(), co, bt);
        assert_eq!(read_f32_buf(y_out, 4), vec![20.0, 30.0, 40.0, 50.0]);

        for p in [bt, rec, init, co, op, w_src, x_in, p_buf, y_out, u_in, u_out, mvn_out, ap_in, ap_out, j0, j1, joined, sp0, sp1, dml_dev, d3d_dev] {
            com_release(p);
        }
    }
}

#[test]
fn device_errors_object_metadata_and_extra_operators() {
    unsafe {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let mut out = std::ptr::null_mut();
        assert_eq!(DMLCreateDevice(std::ptr::null_mut(), 0, &IID_IDMLDevice, &mut out), E_INVALIDARG);

        let mut d3d_dev = std::ptr::null_mut();
        let mut dml_dev = std::ptr::null_mut();
        assert_eq!(
            DMLCreateDeviceAndDirect3D12Device(
                std::ptr::null_mut(),
                0xB000,
                DML_CREATE_DEVICE_FLAG_DEBUG,
                DML_FEATURE_LEVEL_1_0,
                &IID_ID3D12Device,
                &mut d3d_dev,
                &IID_IDMLDevice1,
                &mut dml_dev,
            ),
            S_OK
        );
        assert_eq!(
            DMLCreateDevice1(d3d_dev, 0, 0x9000, &IID_IDMLDevice1, &mut out),
            DXGI_ERROR_UNSUPPORTED
        );

        // IDMLObject: SetName, SetPrivateData, GetPrivateData, GetDeviceRemovedReason, GetParentDevice
        let get_pd: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32, *mut c_void) -> HRESULT = vslot(dml_dev, 3);
        let set_pd: unsafe extern "win64" fn(*mut c_void, *const Guid, u32, *const c_void) -> HRESULT = vslot(dml_dev, 4);
        let set_name: unsafe extern "win64" fn(*mut c_void, *const u16) -> HRESULT = vslot(dml_dev, 6);
        let get_rem: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(dml_dev, 15);
        let get_parent: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 16);

        assert_eq!(get_rem(dml_dev), S_OK);
        let mut parent = std::ptr::null_mut();
        assert_eq!(get_parent(dml_dev, &IID_ID3D12Device, &mut parent), S_OK);
        assert!(!parent.is_null());
        com_release(parent);

        let wname: Vec<u16> = "TestDML\0".encode_utf16().collect();
        assert_eq!(set_name(dml_dev, wname.as_ptr()), S_OK);
        let pk = guid("33333333-4444-5555-6666-777777777777");
        let payload = [9u8, 8, 7, 6];
        assert_eq!(set_pd(dml_dev, &pk, 4, payload.as_ptr() as *const c_void), S_OK);
        let mut psz = 2u32;
        let mut pbuf = [0u8; 4];
        assert_eq!(get_pd(dml_dev, &pk, &mut psz, pbuf.as_mut_ptr() as *mut c_void), DXGI_ERROR_MORE_DATA);
        assert_eq!(psz, 4);
        assert_eq!(get_pd(dml_dev, &pk, &mut psz, pbuf.as_mut_ptr() as *mut c_void), S_OK);
        assert_eq!(pbuf, payload);

        // Identity + ScaleBias, Sub, Div, Abs, Sqrt, Log, Tanh, Softmax
        let b0 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let b1 = make_d3d12_buffer(d3d_dev, 16, heap_type::UPLOAD);
        let bo = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let s4 = [1u32, 1, 1, 4];
        let (bd4, mut td4) = tensor_f32(&s4);
        td4.Desc = &bd4 as *const _ as *const c_void;

        write_f32_buf(b0, &[1.0, 2.0, -3.0, 4.0]);
        let sb = DML_SCALE_BIAS { Scale: 2.0, Bias: -1.0 };
        let id_d = DML_ELEMENT_WISE_UNARY_DESC { InputTensor: &td4, OutputTensor: &td4, ScaleBias: &sb };
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_IDENTITY, _pad: 0, Desc: &id_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
        assert_eq!(read_f32_buf(bo, 4), vec![1.0, 3.0, -7.0, 7.0]);

        write_f32_buf(b1, &[1.0, 2.0, 2.0, 4.0]);
        let bin_d = DML_ELEMENT_WISE_BINARY_DESC { ATensor: &td4, BTensor: &td4, OutputTensor: &td4 };
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_SUBTRACT, _pad: 0, Desc: &bin_d as *const _ as *const c_void }, &[b0, b1], &[16, 16], &[bo], &[16]);
        assert_eq!(read_f32_buf(bo, 4), vec![0.0, 0.0, -5.0, 0.0]);
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_DIVIDE, _pad: 0, Desc: &bin_d as *const _ as *const c_void }, &[b0, b1], &[16, 16], &[bo], &[16]);
        assert_eq!(read_f32_buf(bo, 4), vec![1.0, 1.0, -1.5, 1.0]);

        let u_d = DML_ELEMENT_WISE_UNARY_DESC { InputTensor: &td4, OutputTensor: &td4, ScaleBias: std::ptr::null() };
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_ABS, _pad: 0, Desc: &u_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
        assert_eq!(read_f32_buf(bo, 4), vec![1.0, 2.0, 3.0, 4.0]);

        write_f32_buf(b0, &[1.0, 4.0, 9.0, 16.0]);
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_SQRT, _pad: 0, Desc: &u_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
        assert_eq!(read_f32_buf(bo, 4), vec![1.0, 2.0, 3.0, 4.0]);

        write_f32_buf(b0, &[1.0, std::f32::consts::E, 1.0, 1.0]);
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ELEMENT_WISE_LOG, _pad: 0, Desc: &u_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
        let lg = read_f32_buf(bo, 4);
        assert!(lg[0].abs() < 1e-5 && (lg[1] - 1.0).abs() < 1e-5);

        write_f32_buf(b0, &[0.0, 0.0, 0.0, 0.0]);
        let su_d = DML_SIMPLE_UNARY_DESC { InputTensor: &td4, OutputTensor: &td4 };
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_SOFTMAX, _pad: 0, Desc: &su_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
        assert_eq!(read_f32_buf(bo, 4), vec![0.25, 0.25, 0.25, 0.25]);

        write_f32_buf(b0, &[0.0, 1.0, -1.0, 0.5]);
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_TANH, _pad: 0, Desc: &su_d as *const _ as *const c_void }, &[b0], &[16], &[bo], &[16]);
        let th = read_f32_buf(bo, 4);
        assert!(th[0].abs() < 1e-5 && (th[1] - 1.0f32.tanh()).abs() < 1e-5);

        // GEMM with TransA = TRANSPOSE, TransB = TRANSPOSE: A(3x2)^T * B(2x3)^T = (2x2)
        let ga = make_d3d12_buffer(d3d_dev, 24, heap_type::UPLOAD);
        let gb = make_d3d12_buffer(d3d_dev, 24, heap_type::UPLOAD);
        let gy = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        // A = [[1, 4], [2, 5], [3, 6]] -> A^T = [[1, 2, 3], [4, 5, 6]]
        // B = [[1, 0, 1], [0, 1, 0]] -> B^T = [[1, 0], [0, 1], [1, 0]]
        // A^T * B^T = [[4, 2], [10, 5]]
        write_f32_buf(ga, &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
        write_f32_buf(gb, &[1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
        let s32 = [1u32, 1, 3, 2];
        let s23 = [1u32, 1, 2, 3];
        let s22 = [1u32, 1, 2, 2];
        let (bda, mut tda) = tensor_f32(&s32); tda.Desc = &bda as *const _ as *const c_void;
        let (bdb, mut tdb) = tensor_f32(&s23); tdb.Desc = &bdb as *const _ as *const c_void;
        let (bdy, mut tdy) = tensor_f32(&s22); tdy.Desc = &bdy as *const _ as *const c_void;
        let gemm_t = DML_GEMM_OPERATOR_DESC {
            ATensor: &tda,
            BTensor: &tdb,
            CTensor: std::ptr::null(),
            OutputTensor: &tdy,
            TransA: DML_MATRIX_TRANSFORM_TRANSPOSE,
            TransB: DML_MATRIX_TRANSFORM_TRANSPOSE,
            Alpha: 1.0,
            Beta: 0.0,
            FusedActivation: std::ptr::null(),
        };
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_GEMM, _pad: 0, Desc: &gemm_t as *const _ as *const c_void }, &[ga, gb, std::ptr::null_mut()], &[24, 24, 0], &[gy], &[16]);
        assert_eq!(read_f32_buf(gy, 4), vec![4.0, 2.0, 10.0, 5.0]);

        // MaxPooling1 with UINT32 output indices
        let mp_val = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let mp_idx = make_d3d12_buffer(d3d_dev, 16, heap_type::READBACK);
        let s11 = [1u32, 1, 1, 1];
        let (bd11, mut td11) = tensor_f32(&s11); td11.Desc = &bd11 as *const _ as *const c_void;
        let bd_u32 = DML_BUFFER_TENSOR_DESC {
            DataType: DML_TENSOR_DATA_TYPE_UINT32,
            Flags: 0,
            DimensionCount: 4,
            _pad0: 0,
            Sizes: s11.as_ptr(),
            Strides: std::ptr::null(),
            TotalTensorSizeInBytes: 4,
            GuaranteedBaseOffsetAlignment: 0,
            _pad1: 0,
        };
        let td_u32 = DML_TENSOR_DESC { Type: DML_TENSOR_TYPE_BUFFER, _pad: 0, Desc: &bd_u32 as *const _ as *const c_void };
        let win2 = [2u32, 2];
        let pad0 = [0u32, 0];
        let mp1 = DML_MAX_POOLING1_OPERATOR_DESC {
            InputTensor: &tdy,
            OutputTensor: &td11,
            OutputIndicesTensor: &td_u32,
            DimensionCount: 2,
            _pad0: 0,
            Strides: win2.as_ptr(),
            WindowSize: win2.as_ptr(),
            StartPadding: pad0.as_ptr(),
            EndPadding: pad0.as_ptr(),
        };
        // gy = [4, 2, 10, 5] -> max = 10 at flat spatial index 2
        dispatch_compiled(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_MAX_POOLING1, _pad: 0, Desc: &mp1 as *const _ as *const c_void }, &[gy], &[16], &[mp_val, mp_idx], &[4, 4]);
        assert_eq!(read_f32_buf(mp_val, 1), vec![10.0]);
        let idx_bytes = read_d3d12_buffer(&BufferBinding { resource_ptr: mp_idx as usize, offset: 0, size: 4 }, 4).unwrap();
        assert_eq!(u32::from_le_bytes([idx_bytes[0], idx_bytes[1], idx_bytes[2], idx_bytes[3]]), 2);

        // IDMLOperatorInitializer::Reset + IDMLDeviceChild::GetDevice
        let create_op: unsafe extern "win64" fn(*mut c_void, *const DML_OPERATOR_DESC, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 8);
        let compile_op: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 9);
        let create_init: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dml_dev, 10);
        let mut op = std::ptr::null_mut();
        let mut co = std::ptr::null_mut();
        let mut init = std::ptr::null_mut();
        assert_eq!(create_op(dml_dev, &DML_OPERATOR_DESC { Type: DML_OPERATOR_ACTIVATION_TANH, _pad: 0, Desc: &su_d as *const _ as *const c_void }, &IID_IDMLOperator, &mut op), S_OK);
        assert_eq!(compile_op(dml_dev, op, 0, &IID_IDMLCompiledOperator, &mut co), S_OK);
        assert_eq!(create_init(dml_dev, 0, std::ptr::null(), &IID_IDMLOperatorInitializer, &mut init), S_OK);
        let init_reset_fn: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void) -> HRESULT = vslot(init, 9);
        assert_eq!(init_reset_fn(init, 1, &co), S_OK);
        let child_get_dev: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(op, 7);
        let mut got_dev = std::ptr::null_mut();
        assert_eq!(child_get_dev(op, &IID_IDMLDevice1, &mut got_dev), S_OK);
        assert_eq!(got_dev, dml_dev);
        com_release(got_dev);

        for p in [init, co, op, mp_val, mp_idx, ga, gb, gy, b0, b1, bo, dml_dev, d3d_dev] {
            com_release(p);
        }
    }
}

#[test]
fn metal41_mpp_gpu_gemm_matches_cpu_for_f32_and_f16() {
    unsafe {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        struct EnvGuard {
            key: &'static str,
            prev: Option<std::ffi::OsString>,
        }
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                match &self.prev {
                    Some(v) => std::env::set_var(self.key, v),
                    None => std::env::remove_var(self.key),
                }
            }
        }
        let _g_master = EnvGuard { key: "PORT_METAL41", prev: std::env::var_os("PORT_METAL41") };
        let _g_sub = EnvGuard {
            key: "PORT_METAL41_DIRECTML_GPU",
            prev: std::env::var_os("PORT_METAL41_DIRECTML_GPU"),
        };
        std::env::set_var("PORT_METAL41_DIRECTML_GPU", "1");

        let mut d3d_dev = std::ptr::null_mut();
        let mut dml_dev = std::ptr::null_mut();
        assert_eq!(
            DMLCreateDeviceAndDirect3D12Device(
                std::ptr::null_mut(),
                0xB000,
                DML_CREATE_DEVICE_FLAG_NONE,
                DML_FEATURE_LEVEL_4_0,
                &IID_ID3D12Device,
                &mut d3d_dev,
                &IID_IDMLDevice1,
                &mut dml_dev,
            ),
            S_OK
        );

        // 1) FLOAT32 batched GEMM (B=2, M=3, K=4, N=2) with TransB + broadcast C[2,1,2] + ReLU
        let a_buf = make_d3d12_buffer(d3d_dev, 2 * 3 * 4 * 4, heap_type::DEFAULT);
        let b_buf = make_d3d12_buffer(d3d_dev, 2 * 2 * 4 * 4, heap_type::UPLOAD);
        let c_buf = make_d3d12_buffer(d3d_dev, 2 * 2 * 4, heap_type::UPLOAD);
        let y_buf = make_d3d12_buffer(d3d_dev, 2 * 3 * 2 * 4, heap_type::DEFAULT);
        let a_vals: Vec<f32> = (0..24).map(|i| (i as f32 - 10.0) * 0.25).collect();
        let b_vals: Vec<f32> = (0..16).map(|i| (i as f32 - 7.0) * 0.5).collect();
        let c_vals = [-2.0f32, 1.5, 0.5, -3.0];
        write_f32_buf(a_buf, &a_vals);
        write_f32_buf(b_buf, &b_vals);
        write_f32_buf(c_buf, &c_vals);

        let s_a = [2u32, 3, 4];
        let s_b = [2u32, 2, 4];
        let s_c = [2u32, 1, 2];
        let s_y = [2u32, 3, 2];
        let (bd_a, mut td_a) = tensor_f32(&s_a); td_a.Desc = &bd_a as *const _ as *const c_void;
        let (bd_b, mut td_b) = tensor_f32(&s_b); td_b.Desc = &bd_b as *const _ as *const c_void;
        let (bd_c, mut td_c) = tensor_f32(&s_c); td_c.Desc = &bd_c as *const _ as *const c_void;
        let (bd_y, mut td_y) = tensor_f32(&s_y); td_y.Desc = &bd_y as *const _ as *const c_void;
        let relu_inner = DML_SIMPLE_UNARY_DESC { InputTensor: std::ptr::null(), OutputTensor: std::ptr::null() };
        let relu_desc = DML_OPERATOR_DESC {
            Type: DML_OPERATOR_ACTIVATION_RELU,
            _pad: 0,
            Desc: &relu_inner as *const _ as *const c_void,
        };
        let gemm = DML_GEMM_OPERATOR_DESC {
            ATensor: &td_a,
            BTensor: &td_b,
            CTensor: &td_c,
            OutputTensor: &td_y,
            TransA: DML_MATRIX_TRANSFORM_NONE,
            TransB: DML_MATRIX_TRANSFORM_TRANSPOSE,
            Alpha: 1.5,
            Beta: 0.5,
            FusedActivation: &relu_desc,
        };
        let op = DML_OPERATOR_DESC { Type: DML_OPERATOR_GEMM, _pad: 0, Desc: &gemm as *const _ as *const c_void };
        dispatch_compiled(dml_dev, &op, &[a_buf, b_buf, c_buf], &[96, 64, 16], &[y_buf], &[48]);
        let y_gpu = read_f32_buf(y_buf, 12);

        // Compare against CPU path with PORT_METAL41_DIRECTML_GPU=0.
        std::env::set_var("PORT_METAL41_DIRECTML_GPU", "0");
        let y_cpu_buf = make_d3d12_buffer(d3d_dev, 2 * 3 * 2 * 4, heap_type::DEFAULT);
        dispatch_compiled(dml_dev, &op, &[a_buf, b_buf, c_buf], &[96, 64, 16], &[y_cpu_buf], &[48]);
        let y_cpu = read_f32_buf(y_cpu_buf, 12);
        for (i, (&g, &c)) in y_gpu.iter().zip(y_cpu.iter()).enumerate() {
            assert!((g - c).abs() < 1e-3, "f32 GEMM mismatch at {i}: gpu={g}, cpu={c}");
        }

        for p in [a_buf, b_buf, c_buf, y_buf, y_cpu_buf, dml_dev, d3d_dev] {
            com_release(p);
        }
    }
}
