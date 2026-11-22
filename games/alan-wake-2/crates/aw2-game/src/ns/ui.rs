//! `ui::*` — Northlight / Alan Wake 2 UI components, page resources, and renderer helpers.

pub mod allocator;

mod difftest {
    #[inline]
    pub fn eq<T: PartialEq + core::fmt::Debug>(
        ctx: &str,
        orig: T,
        new: T,
    ) -> Result<(), String> {
        if orig != new {
            return Err(format!("mismatch: orig={orig:?} new={new:?} for {ctx}"));
        }
        Ok(())
    }
}

/// `ui::UIRenderer::vfunc_1` (VA `0x1424f0ba0`, 15 bytes).
///
/// ```text
/// 1424f0ba0: mov r8, qword ptr [rcx + 0x60]
/// 1424f0ba4: mov rax, qword ptr [rdx]
/// 1424f0ba7: mov qword ptr [r8 + 0xb48], rax
/// 1424f0bae: ret
/// ```
///
/// Copies the 64-bit handle/value at `*src` (`rdx`) into `[*(this + 0x60) + 0xb48]`.
pub unsafe extern "win64" fn ui_renderer_vfunc_1(this: *mut u8, src: *const u64) {
    unsafe {
        let ctx = this.add(0x60).cast::<*mut u8>().read_unaligned();
        let val = src.read_unaligned();
        ctx.add(0xb48).cast::<u64>().write_unaligned(val);
    }
}

pub unsafe fn diff_ui_renderer_vfunc_1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u64);
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for (seed, val) in [
        (0u8, 0x1122_3344_5566_7788u64),
        (0xff, 0u64),
        (0xaa, u64::MAX),
        (0x55, 0xdead_beef_cafe_babeu64),
    ] {
        let mut ctx_a = vec![seed; 0xc00];
        let mut ctx_b = vec![seed; 0xc00];
        let mut obj_a = [seed; 0x80];
        let mut obj_b = [seed; 0x80];
        unsafe {
            obj_a
                .as_mut_ptr()
                .add(0x60)
                .cast::<*mut u8>()
                .write_unaligned(ctx_a.as_mut_ptr());
            obj_b
                .as_mut_ptr()
                .add(0x60)
                .cast::<*mut u8>()
                .write_unaligned(ctx_b.as_mut_ptr());
            orig(obj_a.as_mut_ptr(), &val);
            new(obj_b.as_mut_ptr(), &val);
        }
        difftest::eq(&format!("seed={seed:#x} val={val:#x}"), &ctx_a[..], &ctx_b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::TextureAtlasMeta::ClassTypeInfoImplementation::vfunc_12` (VA `0x1424f3780`, 14 bytes).
///
/// ```text
/// 1424f3780: xor eax, eax
/// 1424f3782: mov qword ptr [rdx + 0x10], rax
/// 1424f3786: mov qword ptr [rdx + 0x18], rax
/// 1424f378a: mov rax, rdx
/// 1424f378d: ret
/// ```
///
/// In-place default constructor for `ui::TextureAtlasMeta`: zeroes `[dst + 0x10]` and
/// `[dst + 0x18]` and returns `dst`.
pub unsafe extern "win64" fn texture_atlas_meta_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.add(0x10).cast::<u64>().write_unaligned(0);
        dst.add(0x18).cast::<u64>().write_unaligned(0);
        dst
    }
}

pub unsafe fn diff_texture_atlas_meta_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x30];
        let mut b = [seed; 0x30];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::TextureCoordinates::ClassTypeInfoImplementation::vfunc_12` (VA `0x1424f4450`, 28 bytes).
///
/// ```text
/// 1424f4450: and dword ptr [rdx], 0xff000020
/// 1424f4456: mov rax, rdx
/// 1424f4459: or dword ptr [rdx], 0x20
/// 1424f445c: mov byte ptr [rdx + 0x3], 0x20
/// 1424f4460: mov dword ptr [rdx + 0x4], 0x0
/// 1424f4467: mov byte ptr [rdx + 0x8], 0x0
/// 1424f446b: ret
/// ```
///
/// In-place default constructor for `ui::TextureCoordinates`: initializes its leading
/// `r::InplaceString<32>` header (`0x2000_0020`), length (`0`), and null terminator (`0`),
/// and returns `dst`.
pub unsafe extern "win64" fn texture_coordinates_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x2000_0020);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst
    }
}

pub unsafe fn diff_texture_coordinates_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x12, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x30];
        let mut b = [seed; 0x30];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageResource::Header::ClassTypeInfoImplementation::vfunc_12` (VA `0x1425004d0`, 32 bytes).
///
/// ```text
/// 1425004d0: and dword ptr [rdx], 0xff000020
/// 1425004d6: xor eax, eax
/// 1425004d8: or dword ptr [rdx], 0x20
/// 1425004db: mov dword ptr [rdx + 0x4], eax
/// 1425004de: mov byte ptr [rdx + 0x8], al
/// 1425004e1: mov byte ptr [rdx + 0x3], 0x20
/// 1425004e5: mov qword ptr [rdx + 0x108], rax
/// 1425004ec: mov rax, rdx
/// 1425004ef: ret
/// ```
///
/// In-place default constructor for `ui::PageResource::Header`.
pub unsafe extern "win64" fn page_resource_header_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x2000_0020);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x108).cast::<u64>().write_unaligned(0);
        dst
    }
}

pub unsafe fn diff_page_resource_header_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x120];
        let mut b = [seed; 0x120];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageResource::EventDefinition::ClassTypeInfoImplementation::vfunc_12` (VA `0x1424fb4c0`, 33 bytes)
/// and `ui::PageResource::TransformArgumentDefinition::ClassTypeInfoImplementation::vfunc_12` (VA `0x1424fc880`, 33 bytes).
///
/// ```text
/// 1424fb4c0: and dword ptr [rdx], 0xff000004
/// 1424fb4c6: xor eax, eax
/// 1424fb4c8: or dword ptr [rdx], 0x4
/// 1424fb4cb: mov dword ptr [rdx + 0x4], eax
/// 1424fb4ce: mov byte ptr [rdx + 0x8], al
/// 1424fb4d1: mov byte ptr [rdx + 0x3], 0x4
/// 1424fb4d5: mov qword ptr [rdx + 0x28], rax
/// 1424fb4d9: mov qword ptr [rdx + 0x30], rax
/// 1424fb4dd: mov rax, rdx
/// 1424fb4e0: ret
/// ```
pub unsafe extern "win64" fn page_resource_event_def_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x28).cast::<u64>().write_unaligned(0);
        dst.add(0x30).cast::<u64>().write_unaligned(0);
        dst
    }
}

pub unsafe fn diff_page_resource_event_def_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x40];
        let mut b = [seed; 0x40];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::vfunc_12`
/// (VA `0x1424fcfa0`, 37 bytes).
///
/// ```text
/// 1424fcfa0: xor eax, eax
/// 1424fcfa2: mov qword ptr [rdx], rax
/// 1424fcfa5: mov qword ptr [rdx + 0x8], rax
/// 1424fcfa9: mov dword ptr [rdx + 0x10], eax
/// 1424fcfac: and dword ptr [rdx + 0x14], 0xff000008
/// 1424fcfb3: or dword ptr [rdx + 0x14], 0x8
/// 1424fcfb7: mov dword ptr [rdx + 0x18], eax
/// 1424fcfba: mov byte ptr [rdx + 0x1c], al
/// 1424fcfbd: mov rax, rdx
/// 1424fcfc0: mov byte ptr [rdx + 0x17], 0x8
/// 1424fcfc4: ret
/// ```
pub unsafe extern "win64" fn page_resource_transform_arg_val_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u64>().write_unaligned(0);
        dst.add(0x8).cast::<u64>().write_unaligned(0);
        dst.add(0x10).cast::<u32>().write_unaligned(0);
        dst.add(0x14).cast::<u32>().write_unaligned(0x0800_0008);
        dst.add(0x18).cast::<u32>().write_unaligned(0);
        *dst.add(0x1c) = 0;
        dst
    }
}

pub unsafe fn diff_page_resource_transform_arg_val_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x30];
        let mut b = [seed; 0x30];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::TextureAtlas::ClassTypeInfoImplementation::vfunc_12` (VA `0x1424f3da0`, 39 bytes).
///
/// ```text
/// 1424f3da0: and dword ptr [rdx], 0xff000020
/// 1424f3da6: xor eax, eax
/// 1424f3da8: or dword ptr [rdx], 0x20
/// 1424f3dab: mov dword ptr [rdx + 0x4], eax
/// 1424f3dae: mov byte ptr [rdx + 0x8], al
/// 1424f3db1: mov byte ptr [rdx + 0x3], 0x20
/// 1424f3db5: mov qword ptr [rdx + 0x110], rax
/// 1424f3dbc: mov qword ptr [rdx + 0x118], rax
/// 1424f3dc3: mov rax, rdx
/// 1424f3dc6: ret
/// ```
pub unsafe extern "win64" fn texture_atlas_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x2000_0020);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x110).cast::<u64>().write_unaligned(0);
        dst.add(0x118).cast::<u64>().write_unaligned(0);
        dst
    }
}

pub unsafe fn diff_texture_atlas_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x130];
        let mut b = [seed; 0x130];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfunc_12` (VA `0x1424feda0`, 39 bytes).
///
/// ```text
/// 1424feda0: and dword ptr [rdx], 0xff000004
/// 1424feda6: xor eax, eax
/// 1424feda8: or dword ptr [rdx], 0x4
/// 1424fedab: mov dword ptr [rdx + 0x4], eax
/// 1424fedae: mov byte ptr [rdx + 0x8], al
/// 1424fedb1: mov byte ptr [rdx + 0x3], 0x4
/// 1424fedb5: mov qword ptr [rdx + 0x28], rax
/// 1424fedb9: mov qword ptr [rdx + 0x30], rax
/// 1424fedbd: mov dword ptr [rdx + 0x38], eax
/// 1424fedc0: mov byte ptr [rdx + 0x3c], al
/// 1424fedc3: mov rax, rdx
/// 1424fedc6: ret
/// ```
pub unsafe extern "win64" fn page_curve_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x28).cast::<u64>().write_unaligned(0);
        dst.add(0x30).cast::<u64>().write_unaligned(0);
        dst.add(0x38).cast::<u32>().write_unaligned(0);
        *dst.add(0x3c) = 0;
        dst
    }
}

pub unsafe fn diff_page_curve_typeinfo_vfunc_12(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x50];
        let mut b = [seed; 0x50];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageContent::vfunc_11` (VA `0x141410630`, 54 bytes).
///
/// ```text
/// 141410630: mov edx, dword ptr [r8]
/// 141410633: test edx, edx
/// 141410635: je 0x14141063d
/// 141410637: sub edx, 0x1
/// 14141063a: xor eax, eax
/// 14141063c: ret
/// 14141063d: mov rcx, qword ptr [rcx + 0x10]
/// 141410641: mov eax, dword ptr [r8 + 0x8]
/// 141410645: cmp dword ptr [rcx + 0x108], eax
/// 14141064b: jne 0x14141063a
/// 14141064d: mov eax, dword ptr [r8 + 0xc]
/// 141410651: cmp dword ptr [rcx + 0x10c], eax
/// 141410657: jne 0x14141063a
/// 141410659: cmp dword ptr [rsp + 0x28], 0x2
/// 14141065e: jne 0x14141063a
/// 141410660: mov eax, 0x1
/// 141410665: ret
/// ```
///
/// Checks whether `req` (`r8`) matches the page content's resource IDs at `[*(this + 0x10) + 0x108..0x110]`
/// when `*req == 0` and the 5th parameter `mode` (`[rsp + 0x28]`) equals `2`.
pub unsafe extern "win64" fn page_content_vfunc_11(
    this: *const u8,
    _arg2: u64,
    req: *const u32,
    _arg4: u64,
    mode: u32,
) -> u32 {
    unsafe {
        if req.read_unaligned() != 0 {
            return 0;
        }
        let inner = this.add(0x10).cast::<*const u8>().read_unaligned();
        let req_lo = req.add(2).read_unaligned();
        if inner.add(0x108).cast::<u32>().read_unaligned() != req_lo {
            return 0;
        }
        let req_hi = req.add(3).read_unaligned();
        if inner.add(0x10c).cast::<u32>().read_unaligned() != req_hi {
            return 0;
        }
        u32::from(mode == 2)
    }
}

pub unsafe fn diff_page_content_vfunc_11(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, u64, *const u32, u64, u32) -> u32;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    let mut inner = [0u8; 0x120];
    unsafe {
        inner.as_mut_ptr().add(0x108).cast::<u32>().write_unaligned(0x1111_2222);
        inner.as_mut_ptr().add(0x10c).cast::<u32>().write_unaligned(0x3333_4444);
    }
    let mut obj = [0u8; 0x20];
    unsafe {
        obj.as_mut_ptr()
            .add(0x10)
            .cast::<*const u8>()
            .write_unaligned(inner.as_ptr());
    }

    for kind in [0u32, 1, 2] {
        for r_lo in [0x1111_2222u32, 0x9999_9999] {
            for r_hi in [0x3333_4444u32, 0x8888_8888] {
                for mode in [0u32, 1, 2, 3] {
                    let req = [kind, 0, r_lo, r_hi];
                    let ra = unsafe { orig(obj.as_ptr(), 0, req.as_ptr(), 0, mode) };
                    let rb = unsafe { new(obj.as_ptr(), 0, req.as_ptr(), 0, mode) };
                    difftest::eq(
                        &format!("kind={kind} lo={r_lo:#x} hi={r_hi:#x} mode={mode}"),
                        ra,
                        rb,
                    )?;
                    cases += 1;
                }
            }
        }
    }
    Ok(cases)
}

/// `ui::move_construct_element_40` (inferred, VA `0x141408490`, 91 bytes).
///
/// ```text
/// 141408490: mov qword ptr [rsp + 0x8], rbx
/// 141408495: push rdi
/// 141408496: sub rsp, 0x20
/// 14140849a: mov rax, qword ptr [r8]
/// 14140849d: mov qword ptr [rdx], rax
/// 1414084a0: mov rax, qword ptr [r8 + 0x8]
/// 1414084a4: mov qword ptr [rdx + 0x8], rax
/// 1414084a8: movzx eax, byte ptr [r8 + 0x10]
/// 1414084ad: mov byte ptr [rdx + 0x10], al
/// 1414084b0: movzx eax, byte ptr [r8 + 0x11]
/// 1414084b5: mov byte ptr [rdx + 0x11], al
/// 1414084b8: xor edi, edi
/// 1414084ba: mov qword ptr [rdx + 0x18], rdi
/// 1414084be: mov qword ptr [rdx + 0x20], rdi
/// 1414084c2: mov rax, qword ptr [r8 + 0x18]
/// 1414084c6: mov qword ptr [rdx + 0x18], rax
/// 1414084ca: mov eax, dword ptr [r8 + 0x20]
/// 1414084ce: mov dword ptr [rdx + 0x20], eax
/// 1414084d1: mov eax, dword ptr [r8 + 0x24]
/// 1414084d5: mov dword ptr [rdx + 0x24], eax
/// 1414084d8: mov qword ptr [r8 + 0x18], rdi
/// 1414084dc: mov qword ptr [r8 + 0x20], rdi
/// 1414084e0: mov rbx, qword ptr [rsp + 0x30]
/// 1414084e5: add rsp, 0x20
/// 1414084e9: pop rdi
/// 1414084ea: ret
/// ```
///
/// Move-constructs a 40-byte UI struct from `src` (`r8`) into `dst` (`rdx`), zeroing the moved-from
/// buffer/vector fields at `src + 0x18..0x28`.
pub unsafe extern "win64" fn move_construct_element_40(
    _this: *mut u8,
    dst: *mut u8,
    src: *mut u8,
) {
    unsafe {
        let w0 = src.cast::<u64>().read_unaligned();
        dst.cast::<u64>().write_unaligned(w0);
        let w1 = src.add(0x8).cast::<u64>().read_unaligned();
        dst.add(0x8).cast::<u64>().write_unaligned(w1);
        *dst.add(0x10) = *src.add(0x10);
        *dst.add(0x11) = *src.add(0x11);
        let ptr18 = src.add(0x18).cast::<u64>().read_unaligned();
        dst.add(0x18).cast::<u64>().write_unaligned(ptr18);
        let d20 = src.add(0x20).cast::<u32>().read_unaligned();
        dst.add(0x20).cast::<u32>().write_unaligned(d20);
        let d24 = src.add(0x24).cast::<u32>().read_unaligned();
        dst.add(0x24).cast::<u32>().write_unaligned(d24);
        src.add(0x18).cast::<u64>().write_unaligned(0);
        src.add(0x20).cast::<u64>().write_unaligned(0);
    }
}

pub unsafe fn diff_move_construct_element_40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, *mut u8);
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for (dst_seed, src_seed) in [(0u8, 0xffu8), (0x55, 0xaa), (0xaa, 0x12), (0xff, 0x00)] {
        let mut dst_a = [dst_seed; 0x30];
        let mut dst_b = [dst_seed; 0x30];
        let mut src_a = [src_seed; 0x30];
        for (i, b) in src_a.iter_mut().enumerate() {
            *b ^= (i as u8).wrapping_mul(7);
        }
        let mut src_b = src_a;
        unsafe {
            orig(core::ptr::null_mut(), dst_a.as_mut_ptr(), src_a.as_mut_ptr());
            new(core::ptr::null_mut(), dst_b.as_mut_ptr(), src_b.as_mut_ptr());
        }
        difftest::eq(
            &format!("dst dst_seed={dst_seed:#x} src_seed={src_seed:#x}"),
            &dst_a[..],
            &dst_b[..],
        )?;
        difftest::eq(
            &format!("src dst_seed={dst_seed:#x} src_seed={src_seed:#x}"),
            &src_a[..],
            &src_b[..],
        )?;
        cases += 1;
    }
    Ok(cases)
}

/// Helper: lower-bound lookup of `key` (`u32` at `node + 0x20`) in an MSVC `std::_Tree` whose
/// header node pointer `_Myhead` is stored at `**[owner + 0x60] + 0x20`.
/// Returns `cand + 0x28` if an exact `key` match is found, or null otherwise.
#[inline]
unsafe fn tree_find_u32_payload(owner: *const u8, key: u32) -> *mut u8 {
    unsafe {
        let mgr = owner.add(0x60).cast::<*const u8>().read_unaligned();
        let head = mgr.add(0x20).cast::<*mut u8>().read_unaligned();
        let mut cand = head;
        let mut cur = head.add(0x8).cast::<*mut u8>().read_unaligned();
        while *cur.add(0x19) == 0 {
            let cur_key = cur.add(0x20).cast::<u32>().read_unaligned();
            if cur_key >= key {
                cand = cur;
                cur = cur.cast::<*mut u8>().read_unaligned();
            } else {
                cur = cur.add(0x10).cast::<*mut u8>().read_unaligned();
            }
        }
        if *cand.add(0x19) != 0 || key < cand.add(0x20).cast::<u32>().read_unaligned() {
            cand = head;
        }
        if cand == head {
            core::ptr::null_mut()
        } else {
            cand.add(0x28)
        }
    }
}

/// `ui::set_node_flag_by_id` (inferred, VA `0x1424f6730`, 92 bytes).
///
/// Looks up the `u32` key `[cmd + 0xc]` in the red-black tree at `[*(cmd + 0) + 0x60] + 0x20`.
/// If found, writes `[cmd + 0x8]` (`u8`) to `[node + 0x38]` (`payload + 0x10`).
pub unsafe extern "win64" fn set_node_flag_by_id(cmd: *const u8) {
    unsafe {
        let owner = cmd.cast::<*const u8>().read_unaligned();
        let flag = *cmd.add(0x8);
        let key = cmd.add(0xc).cast::<u32>().read_unaligned();
        let payload = tree_find_u32_payload(owner, key);
        if !payload.is_null() {
            *payload.add(0x10) = flag;
        }
    }
}

/// `ui::set_node_pair_by_id` (inferred, VA `0x1424f6810`, 99 bytes).
///
/// Looks up the `u32` key `[cmd + 0x10]` in the red-black tree at `[*(cmd + 0) + 0x60] + 0x20`.
/// If found, writes `[cmd + 0x8]` (`u32`) and `[cmd + 0xc]` (`u32`) to `payload + 0x14` and
/// `payload + 0x18` (`node + 0x3c` and `node + 0x40`).
pub unsafe extern "win64" fn set_node_pair_by_id(cmd: *const u8) {
    unsafe {
        let owner = cmd.cast::<*const u8>().read_unaligned();
        let v0 = cmd.add(0x8).cast::<u32>().read_unaligned();
        let v1 = cmd.add(0xc).cast::<u32>().read_unaligned();
        let key = cmd.add(0x10).cast::<u32>().read_unaligned();
        let payload = tree_find_u32_payload(owner, key);
        if !payload.is_null() {
            payload.add(0x14).cast::<u32>().write_unaligned(v0);
            payload.add(0x18).cast::<u32>().write_unaligned(v1);
        }
    }
}

/// Build a 4-node test `_Tree` (head + root(20) + left(10) + right(30)) inside a flat buffer.
/// Node stride is `0x50` bytes:
/// - `0x00..0x50`: head (`_Isnil = 1` at `+0x19`, root at `+0x08`)
/// - `0x50..0xa0`: root (`key = 20`)
/// - `0xa0..0xf0`: left (`key = 10`)
/// - `0xf0..0x140`: right (`key = 30`)
unsafe fn init_test_tree(nodes: &mut [u8; 0x140]) {
    nodes.fill(0x55);
    let base = nodes.as_mut_ptr();
    unsafe {
        let head = base;
        let root = base.add(0x50);
        let left = base.add(0xa0);
        let right = base.add(0xf0);

        // head: _Isnil = 1, parent/root = root
        *head.add(0x19) = 1;
        head.cast::<*mut u8>().write_unaligned(left);
        head.add(0x08).cast::<*mut u8>().write_unaligned(root);
        head.add(0x10).cast::<*mut u8>().write_unaligned(right);

        // root (key = 20)
        *root.add(0x19) = 0;
        root.cast::<*mut u8>().write_unaligned(left);
        root.add(0x08).cast::<*mut u8>().write_unaligned(head);
        root.add(0x10).cast::<*mut u8>().write_unaligned(right);
        root.add(0x20).cast::<u32>().write_unaligned(20);

        // left (key = 10)
        *left.add(0x19) = 0;
        left.cast::<*mut u8>().write_unaligned(head);
        left.add(0x08).cast::<*mut u8>().write_unaligned(root);
        left.add(0x10).cast::<*mut u8>().write_unaligned(head);
        left.add(0x20).cast::<u32>().write_unaligned(10);

        // right (key = 30)
        *right.add(0x19) = 0;
        right.cast::<*mut u8>().write_unaligned(head);
        right.add(0x08).cast::<*mut u8>().write_unaligned(root);
        right.add(0x10).cast::<*mut u8>().write_unaligned(head);
        right.add(0x20).cast::<u32>().write_unaligned(30);
    }
}

pub unsafe fn diff_set_node_flag_by_id(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8);
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for key in [5u32, 10, 15, 20, 25, 30, 35] {
        for flag in [0u8, 1, 0x7f, 0xff] {
            let mut nodes_a = [0u8; 0x140];
            let mut nodes_b = [0u8; 0x140];
            unsafe {
                init_test_tree(&mut nodes_a);
                init_test_tree(&mut nodes_b);
            }
            let mut mgr_a = [0u8; 0x30];
            let mut mgr_b = [0u8; 0x30];
            let mut owner_a = [0u8; 0x70];
            let mut owner_b = [0u8; 0x70];
            let mut cmd_a = [0u8; 0x20];
            let mut cmd_b = [0u8; 0x20];
            unsafe {
                mgr_a.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_a.as_mut_ptr());
                mgr_b.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_b.as_mut_ptr());
                owner_a.as_mut_ptr().add(0x60).cast::<*mut u8>().write_unaligned(mgr_a.as_mut_ptr());
                owner_b.as_mut_ptr().add(0x60).cast::<*mut u8>().write_unaligned(mgr_b.as_mut_ptr());
                cmd_a.as_mut_ptr().cast::<*mut u8>().write_unaligned(owner_a.as_mut_ptr());
                *cmd_a.as_mut_ptr().add(0x8) = flag;
                cmd_a.as_mut_ptr().add(0xc).cast::<u32>().write_unaligned(key);
                cmd_b.as_mut_ptr().cast::<*mut u8>().write_unaligned(owner_b.as_mut_ptr());
                *cmd_b.as_mut_ptr().add(0x8) = flag;
                cmd_b.as_mut_ptr().add(0xc).cast::<u32>().write_unaligned(key);

                orig(cmd_a.as_ptr());
                new(cmd_b.as_ptr());
            }
            // Compare payload bytes of each node (`+0x28..+0x50`)
            for (ni, off) in [(0, 0usize), (1, 0x50), (2, 0xa0), (3, 0xf0)] {
                difftest::eq(
                    &format!("key={key} flag={flag:#x} node={ni}"),
                    &nodes_a[off + 0x28..off + 0x50],
                    &nodes_b[off + 0x28..off + 0x50],
                )?;
            }
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_set_node_pair_by_id(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8);
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for key in [5u32, 10, 15, 20, 25, 30, 35] {
        for (v0, v1) in [(0u32, 0u32), (0x1234_5678, 0x9abc_def0), (u32::MAX, 42)] {
            let mut nodes_a = [0u8; 0x140];
            let mut nodes_b = [0u8; 0x140];
            unsafe {
                init_test_tree(&mut nodes_a);
                init_test_tree(&mut nodes_b);
            }
            let mut mgr_a = [0u8; 0x30];
            let mut mgr_b = [0u8; 0x30];
            let mut owner_a = [0u8; 0x70];
            let mut owner_b = [0u8; 0x70];
            let mut cmd_a = [0u8; 0x20];
            let mut cmd_b = [0u8; 0x20];
            unsafe {
                mgr_a.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_a.as_mut_ptr());
                mgr_b.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_b.as_mut_ptr());
                owner_a.as_mut_ptr().add(0x60).cast::<*mut u8>().write_unaligned(mgr_a.as_mut_ptr());
                owner_b.as_mut_ptr().add(0x60).cast::<*mut u8>().write_unaligned(mgr_b.as_mut_ptr());
                cmd_a.as_mut_ptr().cast::<*mut u8>().write_unaligned(owner_a.as_mut_ptr());
                cmd_a.as_mut_ptr().add(0x8).cast::<u32>().write_unaligned(v0);
                cmd_a.as_mut_ptr().add(0xc).cast::<u32>().write_unaligned(v1);
                cmd_a.as_mut_ptr().add(0x10).cast::<u32>().write_unaligned(key);
                cmd_b.as_mut_ptr().cast::<*mut u8>().write_unaligned(owner_b.as_mut_ptr());
                cmd_b.as_mut_ptr().add(0x8).cast::<u32>().write_unaligned(v0);
                cmd_b.as_mut_ptr().add(0xc).cast::<u32>().write_unaligned(v1);
                cmd_b.as_mut_ptr().add(0x10).cast::<u32>().write_unaligned(key);

                orig(cmd_a.as_ptr());
                new(cmd_b.as_ptr());
            }
            for (ni, off) in [(0, 0usize), (1, 0x50), (2, 0xa0), (3, 0xf0)] {
                difftest::eq(
                    &format!("key={key} v0={v0:#x} v1={v1:#x} node={ni}"),
                    &nodes_a[off + 0x28..off + 0x50],
                    &nodes_b[off + 0x28..off + 0x50],
                )?;
            }
            cases += 1;
        }
    }
    Ok(cases)
}

/// `ui::BreakIterator::vfunc_5` (VA `0x14145e010`, 95 bytes).
///
/// Binary-searches the sorted `i32` break-offset array `[this + 0x10]` of length
/// `count = *(u32*)(this + 0x18)` for the lower bound of `pos` (`edx`).
/// Returns the element immediately preceding the lower-bound position (`*(lb - 1)`), or `-1`
/// (`0xffff_ffff`) if `[this + 0x8] == 0`, `count == 0`, `lb == begin`, or `lb == end`.
pub unsafe extern "win64" fn break_iterator_vfunc_5(this: *const u8, pos: i32) -> i32 {
    unsafe {
        if this.add(0x8).cast::<u64>().read_unaligned() == 0 {
            return -1;
        }
        let count = this.add(0x18).cast::<u32>().read_unaligned() as usize;
        if count == 0 {
            return -1;
        }
        let begin = this.add(0x10).cast::<*const i32>().read_unaligned();
        let end = begin.add(count);
        let mut cur = begin;
        let mut rem = count as isize;
        while rem > 0 {
            let half = (rem as usize) >> 1;
            let mid = cur.add(half);
            if mid.read_unaligned() < pos {
                rem += -1 - (half as isize);
                cur = mid.add(1);
            } else {
                rem = half as isize;
            }
        }
        if cur == begin || cur == end {
            -1
        } else {
            cur.sub(1).read_unaligned()
        }
    }
}

pub unsafe fn diff_break_iterator_vfunc_5(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, i32) -> i32;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    let arr: [i32; 6] = [0, 5, 12, 20, 35, 50];
    for has_text in [0u64, 1u64] {
        for len in [0u32, 1, 2, 6] {
            for q in [-5i32, 0, 1, 5, 6, 12, 19, 20, 21, 35, 50, 51, 100] {
                let mut obj = [0u8; 0x28];
                unsafe {
                    obj.as_mut_ptr().add(0x8).cast::<u64>().write_unaligned(has_text);
                    obj.as_mut_ptr()
                        .add(0x10)
                        .cast::<*const i32>()
                        .write_unaligned(arr.as_ptr());
                    obj.as_mut_ptr().add(0x18).cast::<u32>().write_unaligned(len);
                }
                let ra = unsafe { orig(obj.as_ptr(), q) };
                let rb = unsafe { new(obj.as_ptr(), q) };
                difftest::eq(
                    &format!("has_text={has_text} len={len} q={q}"),
                    ra,
                    rb,
                )?;
                cases += 1;
            }
        }
    }
    Ok(cases)
}

/// `ui::mask_low_nibble_flags` (inferred, VA `0x1424efd20`, 51 bytes).
///
/// Preserves the low 4 flag bits (`0x1 | 0x2 | 0x4 | 0x8`) of `flags` (`ecx`), returning `flags & 0xf`.
pub unsafe extern "win64" fn mask_low_nibble_flags(flags: u32) -> u32 {
    if flags == 0xf {
        flags
    } else {
        flags & 0xf
    }
}

pub unsafe fn diff_mask_low_nibble_flags(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u32;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for v in 0..32u32 {
        for hi in [0u32, 0x10, 0x100, 0xffff_fff0] {
            let x = v | hi;
            let ra = unsafe { orig(x) };
            let rb = unsafe { new(x) };
            difftest::eq(&format!("x={x:#x}"), ra, rb)?;
            cases += 1;
        }
    }
    Ok(cases)
}

/// Helper: lower-bound lookup of `key` (`u32` at `node + 0x20`) in an MSVC `std::_Tree` whose
/// header node pointer `_Myhead` is stored at `[mgr + 0x20]`.
/// Returns `cand + 0x28` if an exact `key` match is found, or null otherwise.
#[inline]
unsafe fn mgr_tree_find_u32_payload(mgr: *const u8, key: u32) -> *mut u8 {
    unsafe {
        let head = mgr.add(0x20).cast::<*mut u8>().read_unaligned();
        let mut cand = head;
        let mut cur = head.add(0x8).cast::<*mut u8>().read_unaligned();
        while *cur.add(0x19) == 0 {
            let cur_key = cur.add(0x20).cast::<u32>().read_unaligned();
            if cur_key >= key {
                cand = cur;
                cur = cur.cast::<*mut u8>().read_unaligned();
            } else {
                cur = cur.add(0x10).cast::<*mut u8>().read_unaligned();
            }
        }
        if *cand.add(0x19) != 0 || key < cand.add(0x20).cast::<u32>().read_unaligned() {
            cand = head;
        }
        if cand == head {
            core::ptr::null_mut()
        } else {
            cand.add(0x28)
        }
    }
}

/// `ui::find_node_payload_by_id` (inferred, VA `0x1424f1360`, 70 bytes).
///
/// Looks up `key` (`edx`) in the red-black tree at `[mgr + 0x20]` (`rcx`) and returns a pointer
/// to the node's value payload (`node + 0x28`), or null if not found.
pub unsafe extern "win64" fn find_node_payload_by_id(mgr: *const u8, key: u32) -> *mut u8 {
    unsafe { mgr_tree_find_u32_payload(mgr, key) }
}

pub unsafe fn diff_find_node_payload_by_id(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, u32) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    let mut nodes = [0u8; 0x140];
    unsafe { init_test_tree(&mut nodes) };
    let mut mgr = [0u8; 0x30];
    unsafe {
        mgr.as_mut_ptr()
            .add(0x20)
            .cast::<*mut u8>()
            .write_unaligned(nodes.as_mut_ptr());
    }
    for key in [0u32, 5, 10, 15, 20, 25, 30, 35, u32::MAX] {
        let ra = unsafe { orig(mgr.as_ptr(), key) } as usize;
        let rb = unsafe { new(mgr.as_ptr(), key) } as usize;
        difftest::eq(&format!("key={key}"), ra, rb)?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::mgr_set_node_flag_by_id` (inferred, VA `0x1424f2550`, 77 bytes).
///
/// Looks up `key` (`edx`) in the red-black tree at `[mgr + 0x20]` (`rcx`) and, if found,
/// writes `flag` (`r8b`) to `payload + 0x10` (`node + 0x38`).
pub unsafe extern "win64" fn mgr_set_node_flag_by_id(mgr: *const u8, key: u32, flag: u8) {
    unsafe {
        let payload = mgr_tree_find_u32_payload(mgr, key);
        if !payload.is_null() {
            *payload.add(0x10) = flag;
        }
    }
}

pub unsafe fn diff_mgr_set_node_flag_by_id(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, u32, u8);
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for key in [5u32, 10, 15, 20, 25, 30, 35] {
        for flag in [0u8, 1, 0x7f, 0xff] {
            let mut nodes_a = [0u8; 0x140];
            let mut nodes_b = [0u8; 0x140];
            unsafe {
                init_test_tree(&mut nodes_a);
                init_test_tree(&mut nodes_b);
            }
            let mut mgr_a = [0u8; 0x30];
            let mut mgr_b = [0u8; 0x30];
            unsafe {
                mgr_a.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_a.as_mut_ptr());
                mgr_b.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_b.as_mut_ptr());
                orig(mgr_a.as_ptr(), key, flag);
                new(mgr_b.as_ptr(), key, flag);
            }
            for (ni, off) in [(0, 0usize), (1, 0x50), (2, 0xa0), (3, 0xf0)] {
                difftest::eq(
                    &format!("key={key} flag={flag:#x} node={ni}"),
                    &nodes_a[off + 0x28..off + 0x50],
                    &nodes_b[off + 0x28..off + 0x50],
                )?;
            }
            cases += 1;
        }
    }
    Ok(cases)
}

/// `ui::mgr_set_node_pair_by_id` (inferred, VA `0x1424f25b0`, 81 bytes).
///
/// Looks up `key` (`edx`) in the red-black tree at `[mgr + 0x20]` (`rcx`) and, if found,
/// writes `v0` (`r8d`) and `v1` (`r9d`) to `payload + 0x14` and `payload + 0x18`.
pub unsafe extern "win64" fn mgr_set_node_pair_by_id(
    mgr: *const u8,
    key: u32,
    v0: u32,
    v1: u32,
) {
    unsafe {
        let payload = mgr_tree_find_u32_payload(mgr, key);
        if !payload.is_null() {
            payload.add(0x14).cast::<u32>().write_unaligned(v0);
            payload.add(0x18).cast::<u32>().write_unaligned(v1);
        }
    }
}

pub unsafe fn diff_mgr_set_node_pair_by_id(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, u32, u32, u32);
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for key in [5u32, 10, 15, 20, 25, 30, 35] {
        for (v0, v1) in [(0u32, 0u32), (0x1234_5678, 0x9abc_def0), (u32::MAX, 42)] {
            let mut nodes_a = [0u8; 0x140];
            let mut nodes_b = [0u8; 0x140];
            unsafe {
                init_test_tree(&mut nodes_a);
                init_test_tree(&mut nodes_b);
            }
            let mut mgr_a = [0u8; 0x30];
            let mut mgr_b = [0u8; 0x30];
            unsafe {
                mgr_a.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_a.as_mut_ptr());
                mgr_b.as_mut_ptr().add(0x20).cast::<*mut u8>().write_unaligned(nodes_b.as_mut_ptr());
                orig(mgr_a.as_ptr(), key, v0, v1);
                new(mgr_b.as_ptr(), key, v0, v1);
            }
            for (ni, off) in [(0, 0usize), (1, 0x50), (2, 0xa0), (3, 0xf0)] {
                difftest::eq(
                    &format!("key={key} v0={v0:#x} v1={v1:#x} node={ni}"),
                    &nodes_a[off + 0x28..off + 0x50],
                    &nodes_b[off + 0x28..off + 0x50],
                )?;
            }
            cases += 1;
        }
    }
    Ok(cases)
}

/// `ui::init_work_cmd_40` (inferred, VA `0x1424f6be0`, 65 bytes).
///
/// Initializes a 40-byte UI job/command packet `this` (`rcx`) from `owner` (`rdx`) and six
/// indirect arguments (`r8`, `r9`, `[rsp+0x28]`, `[rsp+0x30]`, `[rsp+0x38]`, `[rsp+0x40]`),
/// returning `this`.
pub unsafe extern "win64" fn init_work_cmd_40(
    this: *mut u8,
    owner: u64,
    p_d8: *const u32,
    p_dc: *const u32,
    p_d10: *const u32,
    p_q18: *const u64,
    p_d20: *const u32,
    p_d24: *const u32,
) -> *mut u8 {
    unsafe {
        this.cast::<u64>().write_unaligned(owner);
        this.add(0x8).cast::<u32>().write_unaligned(p_d8.read_unaligned());
        this.add(0xc).cast::<u32>().write_unaligned(p_dc.read_unaligned());
        this.add(0x10).cast::<u32>().write_unaligned(p_d10.read_unaligned());
        this.add(0x18).cast::<u64>().write_unaligned(p_q18.read_unaligned());
        this.add(0x20).cast::<u32>().write_unaligned(p_d20.read_unaligned());
        this.add(0x24).cast::<u32>().write_unaligned(p_d24.read_unaligned());
        this
    }
}

pub unsafe fn diff_init_work_cmd_40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        *mut u8,
        u64,
        *const u32,
        *const u32,
        *const u32,
        *const u64,
        *const u32,
        *const u32,
    ) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x30];
        let mut b = [seed; 0x30];
        let owner = 0x1122_3344_5566_7788u64 ^ (seed as u64);
        let d8 = 0x1111_0001u32.wrapping_add(seed as u32);
        let dc = 0x2222_0002u32.wrapping_add(seed as u32);
        let d10 = 0x3333_0003u32.wrapping_add(seed as u32);
        let q18 = 0x99aa_bbcc_ddee_ff00u64 ^ (seed as u64);
        let d20 = 0x4444_0004u32.wrapping_add(seed as u32);
        let d24 = 0x5555_0005u32.wrapping_add(seed as u32);

        let ra = unsafe { orig(a.as_mut_ptr(), owner, &d8, &dc, &d10, &q18, &d20, &d24) };
        let rb = unsafe { new(b.as_mut_ptr(), owner, &d8, &dc, &d10, &q18, &d20, &d24) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageResource::NavigationContextDefinition::ClassTypeInfoImplementation::vfunc_12`
/// (VA `0x1424fbb80`, 53 bytes).
pub unsafe extern "win64" fn page_resource_nav_context_def_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x28).cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x2c).cast::<u32>().write_unaligned(0);
        *dst.add(0x30) = 0;
        dst.add(0x58).cast::<u32>().write_unaligned(0);
        *dst.add(0x5c) = 1;
        dst
    }
}

pub unsafe fn diff_page_resource_nav_context_def_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x70];
        let mut b = [seed; 0x70];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::vfunc_12`
/// (VA `0x1424fd6d0`, 120 bytes) and
/// `ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::vfunc_12`
/// (VA `0x1424fe5f0`, 120 bytes).
pub unsafe extern "win64" fn page_resource_fact_binding_def_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x28).cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x2c).cast::<u32>().write_unaligned(0);
        *dst.add(0x30) = 0;
        dst.add(0x50).cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x54).cast::<u32>().write_unaligned(0);
        *dst.add(0x58) = 0;
        dst.add(0x78).cast::<u64>().write_unaligned(0);
        dst.add(0x80).cast::<u64>().write_unaligned(0);
        dst.add(0x88).cast::<u32>().write_unaligned(0);
        dst.add(0x8c).cast::<u32>().write_unaligned(0x0800_0008);
        dst.add(0x90).cast::<u32>().write_unaligned(0);
        *dst.add(0x94) = 0;
        dst
    }
}

pub unsafe fn diff_page_resource_fact_binding_def_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0xb0];
        let mut b = [seed; 0xb0];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::vfunc_12`
/// (VA `0x1424fde70`, 78 bytes).
pub unsafe extern "win64" fn page_resource_fact_binding_list_def_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x28).cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x2c).cast::<u32>().write_unaligned(0);
        *dst.add(0x30) = 0;
        dst.add(0x50).cast::<u64>().write_unaligned(0);
        dst.add(0x58).cast::<u64>().write_unaligned(0);
        dst.add(0x60).cast::<u32>().write_unaligned(0);
        dst.add(0x64).cast::<u32>().write_unaligned(0x0800_0008);
        dst.add(0x68).cast::<u32>().write_unaligned(0);
        *dst.add(0x6c) = 0;
        dst
    }
}

pub unsafe fn diff_page_resource_fact_binding_list_def_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x80];
        let mut b = [seed; 0x80];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementation::vfunc_12`
/// (VA `0x1424ff4e0`, 60 bytes).
pub unsafe extern "win64" fn page_metadata_resource_group_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        dst.cast::<u32>().write_unaligned(0x0400_0004);
        dst.add(0x4).cast::<u32>().write_unaligned(0);
        *dst.add(0x8) = 0;
        dst.add(0x28).cast::<u32>().write_unaligned(0);
        dst.add(0x30).cast::<u64>().write_unaligned(0);
        dst.add(0x38).cast::<u64>().write_unaligned(0);
        dst.add(0x40).cast::<u64>().write_unaligned(0);
        dst.add(0x48).cast::<u64>().write_unaligned(0);
        dst.add(0x50).cast::<u64>().write_unaligned(0);
        dst.add(0x58).cast::<u64>().write_unaligned(0);
        dst.add(0x60).cast::<u64>().write_unaligned(0);
        dst.add(0x68).cast::<u64>().write_unaligned(0);
        dst
    }
}

pub unsafe fn diff_page_metadata_resource_group_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x80];
        let mut b = [seed; 0x80];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}

/// `ui::PageMetadata::ClassTypeInfoImplementation::vfunc_12`
/// (VA `0x1424ffdc0`, 82 bytes).
///
/// In-place default constructor for `ui::PageMetadata`: installs the vftable pointer (`0x1434178e0`
/// resolved relative to the mapped executable base) and initializes metadata fields.
pub unsafe extern "win64" fn page_metadata_typeinfo_vfunc_12(
    _this: *const u8,
    dst: *mut u8,
) -> *mut u8 {
    unsafe {
        let vft = crate::exe::ptr::<u8>(0x1_4341_78e0) as u64;
        *dst.add(0x8) = 1;
        dst.cast::<u64>().write_unaligned(vft);
        dst.add(0x10).cast::<u64>().write_unaligned(0);
        dst.add(0x18).cast::<u64>().write_unaligned(0);
        dst.add(0x20).cast::<u64>().write_unaligned(0);
        dst.add(0x28).cast::<u64>().write_unaligned(0);
        dst.add(0x38).cast::<u64>().write_unaligned(0);
        dst.add(0x40).cast::<u64>().write_unaligned(0);
        dst.add(0x48).cast::<u64>().write_unaligned(0);
        dst.add(0x50).cast::<u64>().write_unaligned(0);
        *dst.add(0x60) = 0;
        dst.add(0x58).cast::<u16>().write_unaligned(1);
        dst.add(0x5c).cast::<u32>().write_unaligned(0xbb8);
        dst.add(0x64).cast::<u32>().write_unaligned(0x8);
        dst.add(0x68).cast::<u32>().write_unaligned(0x40);
        dst
    }
}

pub unsafe fn diff_page_metadata_typeinfo_vfunc_12(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> *mut u8;
    let orig: F = unsafe { core::mem::transmute(orig) };
    let new: F = unsafe { core::mem::transmute(new) };

    let mut cases = 0u32;
    for seed in [0u8, 0x55, 0xaa, 0xff] {
        let mut a = [seed; 0x80];
        let mut b = [seed; 0x80];
        let ra = unsafe { orig(core::ptr::null(), a.as_mut_ptr()) };
        let rb = unsafe { new(core::ptr::null(), b.as_mut_ptr()) };
        difftest::eq(&format!("ret seed={seed:#x}"), ra == a.as_mut_ptr(), rb == b.as_mut_ptr())?;
        difftest::eq(&format!("buf seed={seed:#x}"), &a[..], &b[..])?;
        cases += 1;
    }
    Ok(cases)
}
