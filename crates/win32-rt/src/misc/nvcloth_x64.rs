//! `NvCloth_x64.dll` — Native NVIDIA NvCloth 1.1.6 CPU factory, fabric, cloth, and solver.
//!
//! Implements the 3 C exports (`InitializeNvCloth`, `NvClothCreateFactoryCPU`,
//! `NvClothDestroyFactory`), exact MSVC x64 C++ vtables for `nv::cloth::SwFactory` (14 slots),
//! `nv::cloth::SwFabric` (12 slots), `nv::cloth::SwCloth` (114 slots), and `nv::cloth::SwSolver`
//! (18 slots), and the full `SwSolverKernel` / `SwCollision` / `SwSelfCollision` /
//! `SwInterCollision` physics pipeline.

#![allow(unused_unsafe)]

use std::cell::UnsafeCell;
use std::sync::Once;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

static LOGGED_INIT: AtomicBool = AtomicBool::new(false);
static REGISTER_VTABLES: Once = Once::new();

const EPS: f32 = f32::EPSILON;
const SAFE_LOG2_MIN: f32 = -128.0;
/// `(1.0 - 0.2)^2 - 1.0`: particle trajectory must penetrate > 20% radius to trigger CCD.
const SKELETON_WIDTH: f32 = -0.36;

// ---------------------------------------------------------------------------
// Math, Quaternion, Matrix, MovingAverage & Spatial Radix Sort Primitives
// ---------------------------------------------------------------------------

#[inline]
fn safe_log2(x: f32) -> f32 {
    if x > 0.0 {
        x.log2()
    } else {
        SAFE_LOG2_MIN
    }
}

#[inline]
fn safe_exp2(x: f32) -> f32 {
    if x <= SAFE_LOG2_MIN {
        0.0
    } else {
        x.exp2()
    }
}

#[inline]
fn safe_log2_v3(v: [f32; 3]) -> [f32; 3] {
    [safe_log2(v[0]), safe_log2(v[1]), safe_log2(v[2])]
}

#[inline]
fn safe_exp2_v3(v: [f32; 3]) -> [f32; 3] {
    [safe_exp2(v[0]), safe_exp2(v[1]), safe_exp2(v[2])]
}

#[inline]
fn eff_stiffness(log_k: f32, stiff_exp: f32) -> f32 {
    let val = 1.0 - (stiff_exp * log_k).exp2();
    if val.is_finite() { val } else { 0.0 }
}

#[inline]
fn v3_add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn v3_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn v3_mul(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
}

#[inline]
fn v3_scale(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
fn v3_dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn v3_cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn v3_min(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])]
}

#[inline]
fn v3_max(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
}

#[inline]
fn v3_lerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

#[inline]
fn v4_lerp(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
        a[3] + (b[3] - a[3]) * t,
    ]
}

#[inline]
fn v4_xyz(v: [f32; 4]) -> [f32; 3] {
    [v[0], v[1], v[2]]
}

#[inline]
fn v4_dist_sq(a: [f32; 4], b: [f32; 4]) -> f32 {
    let d0 = a[0] - b[0];
    let d1 = a[1] - b[1];
    let d2 = a[2] - b[2];
    let d3 = a[3] - b[3];
    d0 * d0 + d1 * d1 + d2 * d2 + d3 * d3
}

#[inline]
fn longest_axis(e: [f32; 3]) -> usize {
    if e[0] > e[1] {
        if e[0] > e[2] { 0 } else { 2 }
    } else if e[1] > e[2] {
        1
    } else {
        2
    }
}

#[inline]
fn quat_conj(q: [f32; 4]) -> [f32; 4] {
    [-q[0], -q[1], -q[2], q[3]]
}

#[inline]
fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
        a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
        a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
}

#[inline]
fn quat_normalize_or_identity(q: [f32; 4]) -> [f32; 4] {
    let sq = q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3];
    if sq > EPS && sq.is_finite() {
        let inv = 1.0 / sq.sqrt();
        [q[0] * inv, q[1] * inv, q[2] * inv, q[3] * inv]
    } else {
        [0.0, 0.0, 0.0, 1.0]
    }
}

/// Half-angle quaternion logarithm matching `IterationState.h:60-66`.
#[inline]
fn quat_log_half(q: [f32; 4]) -> [f32; 3] {
    let theta = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
    let mut s = if theta > EPS {
        theta.clamp(-1.0, 1.0).asin() / theta
    } else {
        1.0
    };
    if q[3] < 0.0 {
        s = -s;
    }
    [q[0] * s, q[1] * s, q[2] * s]
}

/// Half-angle quaternion exponential matching `IterationState.h:68-73`.
#[inline]
fn quat_exp_half(v: [f32; 3]) -> [f32; 4] {
    let theta = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let s = if theta > EPS { theta.sin() / theta } else { 1.0 };
    [v[0] * s, v[1] * s, v[2] * s, theta.cos()]
}

/// Column-major 3x3 rotation matrix from unit quaternion (`m[col][row]`).
#[inline]
fn mat3_from_quat(q: [f32; 4]) -> [[f32; 3]; 3] {
    let [x, y, z, w] = quat_normalize_or_identity(q);
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let yy = y * y2;
    let zz = z * z2;
    let xy = x * y2;
    let xz = x * z2;
    let yz = y * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;
    [
        [1.0 - (yy + zz), xy + wz, xz - wy],
        [xy - wz, 1.0 - (xx + zz), yz + wx],
        [xz + wy, yz - wx, 1.0 - (xx + yy)],
    ]
}

#[inline]
fn mat3_mul_vec(m: [[f32; 3]; 3], v: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[1][0] * v[1] + m[2][0] * v[2],
        m[0][1] * v[0] + m[1][1] * v[1] + m[2][1] * v[2],
        m[0][2] * v[0] + m[1][2] * v[1] + m[2][2] * v[2],
    ]
}

/// Skew-symmetric cross-product matrix (`star(v) * x = v x x`), column-major `m[col][row]`.
#[inline]
fn star(v: [f32; 3]) -> [[f32; 3]; 3] {
    [
        [0.0, v[2], -v[1]],
        [-v[2], 0.0, v[0]],
        [v[1], -v[0], 0.0],
    ]
}

#[inline]
fn quat_rotate(q: [f32; 4], v: [f32; 3]) -> [f32; 3] {
    let qn = quat_normalize_or_identity(q);
    let u = [qn[0], qn[1], qn[2]];
    let t = v3_scale(v3_cross(u, v), 2.0);
    v3_add(v, v3_add(v3_scale(t, qn[3]), v3_cross(u, t)))
}

#[derive(Clone, Copy)]
struct Pose {
    p: [f32; 3],
    q: [f32; 4],
}

impl Pose {
    const IDENTITY: Self = Self {
        p: [0.0, 0.0, 0.0],
        q: [0.0, 0.0, 0.0, 1.0],
    };

    #[inline]
    fn inverse(self) -> Self {
        let q_inv = quat_conj(quat_normalize_or_identity(self.q));
        let p_inv = v3_scale(quat_rotate(q_inv, self.p), -1.0);
        Self { p: p_inv, q: q_inv }
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            p: v3_add(quat_rotate(self.q, rhs.p), self.p),
            q: quat_mul(self.q, rhs.q),
        }
    }

    #[inline]
    fn transform_point(self, v: [f32; 3]) -> [f32; 3] {
        v3_add(quat_rotate(self.q, v), self.p)
    }
}

#[inline]
fn transform_aabb(pose: Pose, center: [f32; 3], half_ext: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    let m = mat3_from_quat(pose.q);
    let wc = v3_add(mat3_mul_vec(m, center), pose.p);
    let mut we = [0.0f32; 3];
    for r in 0..3 {
        we[r] = (m[0][r] * half_ext[0]).abs()
            + (m[1][r] * half_ext[1]).abs()
            + (m[2][r] * half_ext[2]).abs();
    }
    (v3_sub(wc, we), v3_add(wc, we))
}

/// Trapezoidal-weighted moving average ring buffer matching `MovingAverage.h`.
#[derive(Clone)]
struct MovingAverage {
    data: Vec<f32>,
    begin: usize,
    count: usize,
}

impl MovingAverage {
    fn new(cap: usize) -> Self {
        Self {
            data: vec![0.0; cap],
            begin: 0,
            count: 0,
        }
    }

    #[inline]
    fn empty(&self) -> bool {
        self.count == 0
    }

    #[inline]
    fn size(&self) -> u32 {
        self.data.len() as u32
    }

    fn reset(&mut self) {
        self.begin = 0;
        self.count = 0;
    }

    fn resize(&mut self, n: usize) {
        if n == 0 {
            self.data.clear();
            self.begin = 0;
            self.count = 0;
            return;
        }
        let old_size = self.data.len();
        if old_size == 0 {
            self.data = vec![0.0; n];
            self.begin = 0;
            self.count = 0;
            return;
        }
        let cut_off_front = self.count.saturating_sub(n);
        let keep = self.count - cut_off_front;
        let mut new_data = vec![0.0f32; n];
        let mut idx = (self.begin + cut_off_front) % old_size;
        for dst in new_data.iter_mut().take(keep) {
            *dst = self.data[idx];
            idx = (idx + 1) % old_size;
        }
        self.data = new_data;
        self.count = keep;
        self.begin = 0;
    }

    fn push(&mut self, n: u32, value: f32) {
        let cap = self.data.len();
        if cap == 0 || n == 0 {
            return;
        }
        let n = (n as usize).min(cap);
        let start = (self.begin + self.count) % cap;
        let end = start + n;
        let end1 = end.min(cap);
        let end2 = end.saturating_sub(end1);
        for slot in &mut self.data[start..end1] {
            *slot = value;
        }
        for slot in &mut self.data[0..end2] {
            *slot = value;
        }
        let new_count = (self.count + n).min(cap);
        self.begin = (self.begin + n - (new_count - self.count)) % cap;
        self.count = new_count;
    }

    fn average(&self) -> f32 {
        let cap = self.data.len();
        if self.count == 0 || cap == 0 {
            return 0.0;
        }
        let ramp = (self.count / 8).max(1);
        let mut sum = 0.0f32;
        let mut total_weight = 0usize;
        for c in 0..self.count {
            let idx = (self.begin + c) % cap;
            let w = (c + 1).min(ramp).min((self.count - c).min(ramp));
            sum += self.data[idx] * (w as f32);
            total_weight += w;
        }
        if total_weight > 0 {
            sum / (total_weight as f32)
        } else {
            0.0
        }
    }
}

/// 4-pass 8-bit LSB radix sort returning element indices sorted by ascending 32-bit key.
fn radix_sort_indices(keys: &[u32], sorted_indices: &mut Vec<u32>, scratch_indices: &mut Vec<u32>) {
    let n = keys.len();
    sorted_indices.resize(n, 0);
    scratch_indices.resize(n, 0);
    if n == 0 {
        return;
    }
    let mut hist = [[0u32; 256]; 4];
    for &k in keys {
        hist[0][(k & 0xff) as usize] += 1;
        hist[1][((k >> 8) & 0xff) as usize] += 1;
        hist[2][((k >> 16) & 0xff) as usize] += 1;
        hist[3][(k >> 24) as usize] += 1;
    }
    let mut sums = [0u32; 4];
    for b in 0..256 {
        for p in 0..4 {
            let c = hist[p][b];
            hist[p][b] = sums[p];
            sums[p] += c;
        }
    }
    for (i, &k) in keys.iter().enumerate() {
        let b = (k & 0xff) as usize;
        let dst = hist[0][b] as usize;
        hist[0][b] += 1;
        scratch_indices[dst] = i as u32;
    }
    for &idx in scratch_indices.iter() {
        let b = ((keys[idx as usize] >> 8) & 0xff) as usize;
        let dst = hist[1][b] as usize;
        hist[1][b] += 1;
        sorted_indices[dst] = idx;
    }
    for &idx in sorted_indices.iter() {
        let b = ((keys[idx as usize] >> 16) & 0xff) as usize;
        let dst = hist[2][b] as usize;
        hist[2][b] += 1;
        scratch_indices[dst] = idx;
    }
    for &idx in scratch_indices.iter() {
        let b = (keys[idx as usize] >> 24) as usize;
        let dst = hist[3][b] as usize;
        hist[3][b] += 1;
        sorted_indices[dst] = idx;
    }
}

// ---------------------------------------------------------------------------
// MSVC x64 ABI Value Structs & Guest Memory Helpers
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PxRange<T> {
    pub begin: *mut T,
    pub end: *mut T,
}

/// MSVC x64 `nv::cloth::MappedRange<physx::PxVec4>` (`0x38` bytes).
#[repr(C)]
pub struct MappedRangePxVec4 {
    pub begin: *mut [f32; 4],
    pub end: *mut [f32; 4],
    pub cloth: *const NvCloth,
    pub lock_fn: usize,
    pub lock_adj: i32,
    pub lock_pad: i32,
    pub unlock_fn: usize,
    pub unlock_adj: i32,
    pub unlock_pad: i32,
}

const _: () = assert!(std::mem::size_of::<MappedRangePxVec4>() == 0x38);

/// MSVC x64 `nv::cloth::PhaseConfig` (`0x14` = 20 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PhaseConfig {
    pub stiffness: f32,
    pub stiffness_multiplier: f32,
    pub compression_limit: f32,
    pub stretch_limit: f32,
    pub phase_index: u16,
    pub padding: u16,
}

const _: () = assert!(std::mem::size_of::<PhaseConfig>() == 20);

fn range_len<T>(range: &PxRange<T>) -> usize {
    if range.begin.is_null() || range.end.is_null() || range.end <= range.begin {
        return 0;
    }
    let elem_size = std::mem::size_of::<T>();
    if elem_size == 0 {
        return 0;
    }
    ((range.end as usize) - (range.begin as usize)) / elem_size
}

unsafe fn read_range_vec<T: Copy>(range_ptr: *const PxRange<T>) -> Vec<T> {
    if range_ptr.is_null() {
        return Vec::new();
    }
    let range = unsafe { range_ptr.read_unaligned() };
    let count = range_len(&range);
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        out.push(unsafe { range.begin.add(i).read_unaligned() });
    }
    out
}

unsafe fn write_range_slice<T: Copy>(range_ptr: *const PxRange<T>, src: &[T]) {
    if range_ptr.is_null() || src.is_empty() {
        return;
    }
    let range = unsafe { range_ptr.read_unaligned() };
    let cap = range_len(&range);
    let n = cap.min(src.len());
    for (i, &v) in src.iter().take(n).enumerate() {
        unsafe { range.begin.add(i).write_unaligned(v) };
    }
}

// ---------------------------------------------------------------------------
// Core NvCloth Objects (Factory, Fabric, Cloth, Solver)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct NvClothFactory {
    pub vtbl: *const [crate::Ptr; 14],
}

/// `nv::cloth::Fabric` layout: `vptr` at `+0x00`, `mRefCount: AtomicI32` at `+0x08`
/// (manipulated inline via atomic increment/decrement by callers).
#[repr(C)]
pub struct NvClothFabric {
    pub vtbl: *const [crate::Ptr; 12],
    pub ref_count: AtomicI32,
    pub num_particles: u32,
    pub factory: *mut NvClothFactory,
    pub phases: Vec<u32>,
    /// Prefixed with 0: `sets[0] == 0`; set `s` spans constraints `sets[s]..sets[s + 1]`.
    pub sets: Vec<u32>,
    pub restvalues: Vec<f32>,
    /// Empty if omitted; otherwise `len() == restvalues.len()`, pre-encoded in `log2(1 - k)` space.
    pub stiffness_values: Vec<f32>,
    /// Length `2 * restvalues.len()`.
    pub indices: Vec<u32>,
    /// Stranded SoA tethers `(anchor_idx, length)`.
    pub tethers: Vec<(u32, f32)>,
    pub tether_length_scale: f32,
    /// Length `3 * num_triangles` (used by `applyWind`).
    pub triangles: Vec<u32>,
}

const _: () = assert!(std::mem::offset_of!(NvClothFabric, ref_count) == 0x08);

#[derive(Clone, Default)]
struct ConstraintsPair {
    start: Vec<[f32; 4]>,
    target: Vec<[f32; 4]>,
}

impl ConstraintsPair {
    fn push(&mut self, n: usize) -> (*mut [f32; 4], *mut [f32; 4]) {
        if n == 0 {
            return (std::ptr::null_mut(), std::ptr::null_mut());
        }
        if self.start.is_empty() {
            self.start.resize(n, [0.0; 4]);
            let ptr = self.start.as_mut_ptr();
            (ptr, unsafe { ptr.add(n) })
        } else {
            self.target.resize(n, [0.0; 4]);
            let ptr = self.target.as_mut_ptr();
            (ptr, unsafe { ptr.add(n) })
        }
    }

    fn pop(&mut self) {
        if !self.target.is_empty() {
            std::mem::swap(&mut self.start, &mut self.target);
            self.target.clear();
        }
    }

    fn clear(&mut self) {
        self.start.clear();
        self.target.clear();
    }
}

#[repr(C)]
pub struct NvCloth {
    pub vtbl: *const [crate::Ptr; 114],
    pub factory: *mut NvClothFactory,
    pub fabric: *mut NvClothFabric,
    pub num_particles: u32,
    pub bbox_center: [f32; 3],
    pub bbox_scale: [f32; 3],
    pub gravity: [f32; 3],
    log_damping: [f32; 3],
    linear_log_drag: [f32; 3],
    angular_log_drag: [f32; 3],
    linear_inertia: [f32; 3],
    angular_inertia: [f32; 3],
    centrifugal_inertia: [f32; 3],
    solver_frequency: f32,
    stiffness_frequency: f32,
    target_motion: Pose,
    current_motion: Pose,
    linear_velocity: [f32; 3],
    angular_velocity: [f32; 3],
    ignore_velocity_discontinuity_next_frame: bool,
    prev_iter_dt: f32,
    iter_dt_avg: MovingAverage,
    wind: [f32; 3],
    drag_log_coefficient: f32,
    lift_log_coefficient: f32,
    fluid_density: f32,
    sleep_test_interval: u32,
    sleep_after_count: u32,
    sleep_threshold: f32,
    sleep_pass_counter: u32,
    sleep_test_counter: u32,
    pub current_particles: Vec<[f32; 4]>,
    pub previous_particles: Vec<[f32; 4]>,
    phase_configs: Vec<PhaseConfig>,
    tether_constraint_log_stiffness: f32,
    tether_constraint_scale: f32,
    motion_constraints: ConstraintsPair,
    motion_constraint_scale: f32,
    motion_constraint_bias: f32,
    motion_constraint_log_stiffness: f32,
    separation_constraints: ConstraintsPair,
    particle_accelerations: Vec<[f32; 4]>,
    capsule_indices: Vec<[u32; 2]>,
    start_collision_spheres: Vec<[f32; 4]>,
    target_collision_spheres: Vec<[f32; 4]>,
    convex_masks: Vec<u32>,
    start_collision_planes: Vec<[f32; 4]>,
    target_collision_planes: Vec<[f32; 4]>,
    start_collision_triangles: Vec<[f32; 3]>,
    target_collision_triangles: Vec<[f32; 3]>,
    enable_continuous_collision: bool,
    collision_mass_scale: f32,
    friction: f32,
    virtual_particle_indices: Vec<[u32; 4]>,
    /// Stores `[wx, wy, wz, 1.0 / (wx^2 + wy^2 + wz^2)]`.
    virtual_particle_weights: Vec<[f32; 4]>,
    self_collision_distance: f32,
    self_collision_log_stiffness: f32,
    self_collision_indices: Vec<u32>,
    rest_positions: Vec<[f32; 4]>,
    pub user_data: *mut u8,
}

impl NvCloth {
    #[inline]
    fn wake_up(&mut self) {
        self.sleep_pass_counter = 0;
    }

    #[inline]
    fn is_sleeping(&self) -> bool {
        self.sleep_pass_counter >= self.sleep_after_count
    }

    fn set_particle_bounds(&mut self, lower: [f32; 3], upper: [f32; 3]) {
        for i in 0..3 {
            let c = (upper[i] + lower[i]) * 0.5;
            let e = (upper[i] - lower[i]) * 0.5;
            self.bbox_center[i] = if c.is_finite() { c } else { 0.0 };
            self.bbox_scale[i] = if e.is_finite() { e.max(0.0) } else { 0.0 };
        }
    }
}

#[derive(Clone, Copy, Default)]
struct ConeData {
    center: [f32; 3],
    /// Cone radius at center; `0.0` marks a degenerate cone (one sphere encloses the other).
    radius: f32,
    axis: [f32; 3],
    slope: f32,
    sqr_cos: f32,
    half_len: f32,
    first_idx: usize,
    second_idx: usize,
    first_mask: u32,
    second_mask: u32,
    both_mask: u32,
}

#[derive(Clone, Copy)]
struct TriangleData {
    p0: [f32; 3],
    e0: [f32; 3],
    e1: [f32; 3],
    normal: [f32; 3],
    e0_sq: f32,
    e1_sq: f32,
    e0_e1: f32,
    inv_det: f32,
    inv_denom: f32,
    inv_e0_sq: f32,
    inv_e1_sq: f32,
}

/// Reusable per-cloth scratch buffers so concurrent `simulateChunk(idx)` calls never allocate or share state.
#[derive(Default)]
struct SolverScratch {
    cur_spheres: Vec<[f32; 4]>,
    prev_spheres: Vec<[f32; 4]>,
    cur_cones: Vec<ConeData>,
    prev_cones: Vec<ConeData>,
    planes: Vec<[f32; 4]>,
    plane_dists: Vec<f32>,
    triangles: Vec<TriangleData>,
    sc_keys: Vec<u32>,
    sc_sorted_indices: Vec<u32>,
    sc_scratch_indices: Vec<u32>,
    sc_sorted_keys: Vec<u32>,
    sc_mapped_indices: Vec<u32>,
}

struct SimulatedCloth {
    cloth: *mut NvCloth,
    inv_num_iterations: f32,
    scratch: SolverScratch,
}

impl SimulatedCloth {
    fn new(cloth: *mut NvCloth) -> Self {
        Self {
            cloth,
            inv_num_iterations: 0.0,
            scratch: SolverScratch::default(),
        }
    }
}

#[derive(Default)]
struct InterCollisionScratch {
    world_bounds: Vec<([f32; 3], [f32; 3])>,
    sorted_cloths: Vec<usize>,
    overlap_matrix: Vec<bool>,
    overlap_boxes: Vec<([f32; 3], [f32; 3])>,
    cand_cloth: Vec<usize>,
    cand_particle: Vec<usize>,
    keys: Vec<u32>,
    sorted_indices: Vec<u32>,
    scratch_indices: Vec<u32>,
    sorted_keys: Vec<u32>,
}

#[repr(C)]
pub struct NvClothSolver {
    pub vtbl: *const [crate::Ptr; 18],
    pub cloths: Vec<*mut NvCloth>,
    /// Each element is disjointly owned by one `simulateChunk(idx)` worker thread via `UnsafeCell`.
    simulated_cloths: Vec<UnsafeCell<SimulatedCloth>>,
    current_dt: f32,
    pub inter_collision_distance: f32,
    pub inter_collision_stiffness: f32,
    pub inter_collision_iterations: u32,
    inter_collision_filter: usize,
    inter_scratch: InterCollisionScratch,
}

// ---------------------------------------------------------------------------
// IterationStateFactory & IterationState (`IterationState.h`)
// ---------------------------------------------------------------------------

struct IterationStateFactory {
    num_iterations: u32,
    inv_num_iterations: f32,
    iter_dt: f32,
    iter_dt_ratio: f32,
    iter_dt_average: f32,
    current_rotation: [f32; 4],
    prev_linear_velocity: [f32; 3],
    prev_angular_velocity: [f32; 3],
}

impl IterationStateFactory {
    fn new(cloth: &mut NvCloth, frame_dt: f32) -> Self {
        let freq = if cloth.solver_frequency.is_finite() && cloth.solver_frequency > 0.0 {
            cloth.solver_frequency
        } else {
            0.0
        };
        let num_iterations = (frame_dt * freq + 0.5).clamp(1.0, 10_000.0) as u32;
        let inv_num_iterations = 1.0 / (num_iterations as f32);
        let iter_dt = frame_dt * inv_num_iterations;
        let iter_dt_ratio = if cloth.prev_iter_dt > 0.0 && cloth.prev_iter_dt.is_finite() {
            iter_dt / cloth.prev_iter_dt
        } else {
            1.0
        };
        let iter_dt_average = if cloth.iter_dt_avg.empty() {
            iter_dt
        } else {
            cloth.iter_dt_avg.average()
        };

        let current_rotation = cloth.current_motion.q;
        let prev_linear_velocity = cloth.linear_velocity;
        let prev_angular_velocity = cloth.angular_velocity;

        if !cloth.ignore_velocity_discontinuity_next_frame {
            let inv_frame_dt = 1.0 / frame_dt;
            cloth.linear_velocity =
                v3_scale(v3_sub(cloth.target_motion.p, cloth.current_motion.p), inv_frame_dt);
            let dq = quat_mul(cloth.target_motion.q, quat_conj(cloth.current_motion.q));
            cloth.angular_velocity = v3_scale(quat_log_half(dq), inv_frame_dt);
        }
        cloth.ignore_velocity_discontinuity_next_frame = false;

        cloth.prev_iter_dt = iter_dt;
        cloth.iter_dt_avg.push(num_iterations, iter_dt);
        cloth.current_motion = cloth.target_motion;

        Self {
            num_iterations,
            inv_num_iterations,
            iter_dt,
            iter_dt_ratio,
            iter_dt_average,
            current_rotation,
            prev_linear_velocity,
            prev_angular_velocity,
        }
    }

    fn create(&self, cloth: &NvCloth) -> IterationState {
        let damp_exp = cloth.stiffness_frequency * self.iter_dt;
        let translation = v3_scale(cloth.linear_velocity, self.iter_dt);
        let gravity = v3_scale(cloth.gravity, self.iter_dt_average * self.iter_dt_average);

        let damp_scale = [
            (cloth.log_damping[0] * damp_exp).exp2(),
            (cloth.log_damping[1] * damp_exp).exp2(),
            (cloth.log_damping[2] * damp_exp).exp2(),
        ];
        let first_damp_scale = v3_scale(damp_scale, self.iter_dt_ratio);

        let linear_drag = [
            (1.0 - (cloth.linear_log_drag[0] * damp_exp).exp2()) * translation[0],
            (1.0 - (cloth.linear_log_drag[1] * damp_exp).exp2()) * translation[1],
            (1.0 - (cloth.linear_log_drag[2] * damp_exp).exp2()) * translation[2],
        ];
        let linear_inertia = v3_mul(
            cloth.linear_inertia,
            v3_scale(v3_sub(self.prev_linear_velocity, cloth.linear_velocity), self.iter_dt),
        );

        let linear_alpha = ((self.num_iterations as f32) + 1.0) * 0.5 * self.inv_num_iterations;
        let cur_linear_inertia = v3_scale(linear_inertia, linear_alpha);

        let inv_rot = mat3_from_quat(quat_conj(self.current_rotation));

        let bias = v3_sub(gravity, linear_drag);
        let cur_bias = mat3_mul_vec(inv_rot, v3_add(cur_linear_inertia, bias));
        let prev_bias = mat3_mul_vec(inv_rot, v3_sub(linear_inertia, cur_linear_inertia));
        let wind = mat3_mul_vec(
            inv_rot,
            v3_sub(translation, v3_scale(cloth.wind, self.iter_dt)),
        );

        let is_turning = v3_dot(self.prev_angular_velocity, self.prev_angular_velocity)
            + v3_dot(cloth.angular_velocity, cloth.angular_velocity)
            > 0.0;

        let mut rotation_matrix = [[0.0f32; 3]; 3];
        let mut prev_matrix = [[0.0f32; 3]; 3];
        let mut cur_matrix = [[0.0f32; 3]; 3];
        let scale = first_damp_scale;

        if is_turning {
            let cur_ang = mat3_mul_vec(inv_rot, cloth.angular_velocity);
            let prev_ang = mat3_mul_vec(inv_rot, self.prev_angular_velocity);

            let cur_inv_angle = v3_scale(cur_ang, -self.iter_dt);
            let prev_inv_angle = v3_scale(prev_ang, -self.iter_dt);

            let cur_inv_rot = quat_exp_half(cur_inv_angle);
            let prev_inv_rot = quat_exp_half(prev_inv_angle);

            let cur_mat = mat3_from_quat(cur_inv_rot);
            let prev_mat = mat3_from_quat(quat_mul(prev_inv_rot, cur_inv_rot));
            rotation_matrix = cur_mat;

            let ang_drag = [
                1.0 - (cloth.angular_log_drag[0] * damp_exp).exp2(),
                1.0 - (cloth.angular_log_drag[1] * damp_exp).exp2(),
                1.0 - (cloth.angular_log_drag[2] * damp_exp).exp2(),
            ];
            let ang_acc = v3_sub(cur_ang, prev_ang);

            let eps = f32::MIN_POSITIVE.sqrt();
            let vel_sq = v3_dot(cur_ang, cur_ang) + eps;
            let drag_v = v3_mul(cur_ang, ang_drag);
            let drag_sq = v3_dot(drag_v, drag_v) + eps;
            let cent_v = v3_mul(cur_ang, cloth.centrifugal_inertia);
            let cent_sq = v3_dot(cent_v, cent_v) + eps;
            let acc_sq = v3_dot(ang_acc, ang_acc) + eps;
            let inert_v = v3_mul(ang_acc, cloth.angular_inertia);
            let inert_sq = v3_dot(inert_v, inert_v) + eps;

            let drag_scale = (drag_sq / vel_sq).sqrt();
            let inertia_scale = self.inv_num_iterations * (inert_sq / acc_sq).sqrt();
            let cent_alpha = (2.0 * (self.num_iterations as f32) - 1.0) * self.inv_num_iterations;
            let cent_scale = cent_alpha * (cent_sq / vel_sq).sqrt() - inertia_scale;

            let cent_vel = v3_scale(v3_add(prev_inv_angle, cur_inv_angle), 0.5);
            let cent_sqr_len = v3_dot(cent_vel, cent_vel) * cent_scale;
            let coriolis_mat = star(v3_scale(cent_vel, cent_scale));

            for j in 0..3 {
                let cent_j = -cent_vel[j] * cent_scale;
                let damp = first_damp_scale[j];
                for i in 0..3 {
                    let cor = coriolis_mat[j][i];
                    let cent = cent_vel[i] * cent_j;
                    prev_matrix[j][i] = cent - cor
                        + cur_mat[j][i] * (inertia_scale - damp)
                        - prev_mat[j][i] * inertia_scale;
                    cur_matrix[j][i] =
                        cent + cor + cur_mat[j][i] * (inertia_scale + damp + drag_scale);
                }
                cur_matrix[j][j] += cent_sqr_len - inertia_scale - drag_scale;
                prev_matrix[j][j] += cent_sqr_len;
            }
        }

        let damp_scale_update = v3_sub(damp_scale, first_damp_scale);

        IterationState {
            rotation_matrix,
            cur_bias,
            prev_bias,
            wind,
            prev_matrix,
            cur_matrix,
            scale,
            damp_scale_update,
            remaining_iterations: self.num_iterations,
            inv_num_iterations: self.inv_num_iterations,
            iter_dt: self.iter_dt,
            is_turning,
        }
    }
}

struct IterationState {
    rotation_matrix: [[f32; 3]; 3],
    cur_bias: [f32; 3],
    prev_bias: [f32; 3],
    wind: [f32; 3],
    prev_matrix: [[f32; 3]; 3],
    cur_matrix: [[f32; 3]; 3],
    scale: [f32; 3],
    damp_scale_update: [f32; 3],
    remaining_iterations: u32,
    inv_num_iterations: f32,
    iter_dt: f32,
    is_turning: bool,
}

impl IterationState {
    #[inline]
    fn prev_alpha(&self) -> f32 {
        1.0 - (self.remaining_iterations as f32) * self.inv_num_iterations
    }

    #[inline]
    fn cur_alpha(&self) -> f32 {
        self.prev_alpha() + self.inv_num_iterations
    }

    fn update(&mut self) {
        if self.is_turning {
            self.cur_bias = mat3_mul_vec(self.rotation_matrix, self.cur_bias);
            self.prev_bias = mat3_mul_vec(self.rotation_matrix, self.prev_bias);
            self.wind = mat3_mul_vec(self.rotation_matrix, self.wind);
            for col in 0..3 {
                for row in 0..3 {
                    let delta = self.rotation_matrix[col][row] * self.damp_scale_update[row];
                    self.prev_matrix[col][row] -= delta;
                    self.cur_matrix[col][row] += delta;
                }
            }
        } else {
            self.scale = v3_add(self.scale, self.damp_scale_update);
        }
        self.damp_scale_update = [0.0; 3];
        self.remaining_iterations = self.remaining_iterations.saturating_sub(1);
    }
}

// ---------------------------------------------------------------------------
// SwCollision Helpers (`SwCollision.cpp`)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct ImpulseAccum {
    delta: [f32; 3],
    vel: [f32; 3],
    count: u32,
}

impl ImpulseAccum {
    #[inline]
    fn add(&mut self, dir: [f32; 3], scale: f32) {
        self.delta[0] += dir[0] * scale;
        self.delta[1] += dir[1] * scale;
        self.delta[2] += dir[2] * scale;
        self.count += 1;
    }

    #[inline]
    fn subtract(&mut self, dir: [f32; 3], scale: f32) {
        self.delta[0] -= dir[0] * scale;
        self.delta[1] -= dir[1] * scale;
        self.delta[2] -= dir[2] * scale;
        self.count += 1;
    }

    #[inline]
    fn add_vel(&mut self, v: [f32; 3]) {
        self.vel[0] += v[0];
        self.vel[1] += v[1];
        self.vel[2] += v[2];
    }
}

#[inline]
fn calc_friction_impulse(
    accum_delta: [f32; 3],
    accum_vel: [f32; 3],
    cur_pos: [f32; 3],
    prev_pos: [f32; 3],
    inv_n: f32,
    friction: f32,
) -> [f32; 3] {
    let d_sq = v3_dot(accum_delta, accum_delta);
    let rcp_d = 1.0 / (d_sq + EPS).sqrt();
    let n = v3_scale(accum_delta, rcp_d);
    let rv = v3_sub(v3_sub(cur_pos, prev_pos), v3_scale(accum_vel, inv_n));
    let rvt = v3_sub(rv, v3_scale(n, v3_dot(rv, n)));
    let rcp_vt = 1.0 / (v3_dot(rvt, rvt) + EPS).sqrt();
    let j = (-friction * d_sq * rcp_d * rcp_vt).max(-1.0);
    let imp = v3_scale(rvt, j);
    if imp[0].is_finite() && imp[1].is_finite() && imp[2].is_finite() {
        imp
    } else {
        [0.0; 3]
    }
}

fn generate_spheres(dst: &mut Vec<[f32; 4]>, start: &[[f32; 4]], target: &[[f32; 4]], alpha: f32, use_target: bool) {
    dst.clear();
    let n = start.len();
    dst.reserve(n);
    if target.is_empty() {
        for &s in start {
            dst.push([s[0], s[1], s[2], s[3].max(0.0)]);
        }
    } else if use_target {
        for i in 0..n {
            let s = target.get(i).copied().unwrap_or(start[i]);
            dst.push([s[0], s[1], s[2], s[3].max(0.0)]);
        }
    } else {
        for i in 0..n {
            let a = start[i];
            let b = target.get(i).copied().unwrap_or(a);
            let m = v4_lerp(a, b, alpha);
            dst.push([m[0], m[1], m[2], m[3].max(0.0)]);
        }
    }
}

fn generate_cones(dst: &mut Vec<ConeData>, spheres: &[[f32; 4]], capsules: &[[u32; 2]]) {
    dst.clear();
    dst.reserve(capsules.len());
    let num_spheres = spheres.len();
    for &[s0_u32, s1_u32] in capsules {
        let s0 = s0_u32 as usize;
        let s1 = s1_u32 as usize;
        if s0 >= num_spheres || s1 >= num_spheres || s0 >= 32 || s1 >= 32 {
            dst.push(ConeData::default());
            continue;
        }
        let first = spheres[s0];
        let second = spheres[s1];
        let center = v3_scale(v3_add(v4_xyz(second), v4_xyz(first)), 0.5);
        let half_axis = v3_scale(v3_sub(v4_xyz(second), v4_xyz(first)), 0.5);
        let half_dr = (second[3] - first[3]) * 0.5;
        let sqr_axis = v3_dot(half_axis, half_axis);
        let sqr_cone = sqr_axis - half_dr * half_dr;
        let first_mask = 1u32 << s0;
        let second_mask = 1u32 << s1;
        let both_mask = first_mask | second_mask;
        if !(sqr_cone > 0.0
            && sqr_axis > EPS
            && sqr_cone.is_finite()
            && sqr_axis.is_finite()
            && first[3].is_finite()
            && second[3].is_finite())
        {
            dst.push(ConeData {
                center,
                radius: 0.0,
                axis: [0.0; 3],
                slope: 0.0,
                sqr_cos: 1.0,
                half_len: 0.0,
                first_idx: s0,
                second_idx: s1,
                first_mask,
                second_mask,
                both_mask,
            });
            continue;
        }
        let inv_axis = 1.0 / sqr_axis.sqrt();
        let inv_cone = 1.0 / sqr_cone.sqrt();
        let half_len = sqr_axis * inv_axis;
        let axis = v3_scale(half_axis, inv_axis);
        let slope = half_dr * inv_cone;
        let radius = (half_dr + first[3]) * inv_cone * half_len;
        let sin_a = half_dr * inv_axis;
        let sqr_cos = 1.0 - sin_a * sin_a;
        dst.push(ConeData {
            center,
            radius,
            axis,
            slope,
            sqr_cos,
            half_len,
            first_idx: s0,
            second_idx: s1,
            first_mask,
            second_mask,
            both_mask,
        });
    }
}

fn collide_discrete_cones_and_spheres(
    pos: [f32; 3],
    cur_spheres: &[[f32; 4]],
    prev_spheres: &[[f32; 4]],
    cur_cones: &[ConeData],
    friction_enabled: bool,
    accum: &mut ImpulseAccum,
) {
    let n_sph = cur_spheres.len().min(32);
    let mut sphere_mask: u32 = if n_sph == 32 {
        u32::MAX
    } else {
        (1u32 << n_sph) - 1
    };

    for cone in cur_cones {
        if !(cone.radius > 0.0) {
            continue;
        }
        let delta = v3_sub(pos, cone.center);
        let d_axis = v3_dot(delta, cone.axis);
        let r = (d_axis * cone.slope + cone.radius).max(0.0);
        let sqr_dist = v3_dot(delta, delta) - d_axis * d_axis;
        if !(r * r > sqr_dist) {
            sphere_mask &= !cone.both_mask;
            continue;
        }
        let sqr_dist = sqr_dist.max(EPS);
        let inv_dist = 1.0 / sqr_dist.sqrt();
        let base = d_axis + cone.slope * sqr_dist * inv_dist;
        let left = base < -cone.half_len;
        let right = base > cone.half_len;
        if !left {
            sphere_mask &= !cone.first_mask;
        }
        if !right {
            sphere_mask &= !cone.second_mask;
        }
        if base >= -cone.half_len && base <= cone.half_len {
            let n_dir = v3_sub(delta, v3_scale(cone.axis, base));
            let scale = (r * inv_dist - 1.0) * cone.sqr_cos;
            accum.add(n_dir, scale);
            if friction_enabled && cone.first_idx < prev_spheres.len() && cone.second_idx < prev_spheres.len() {
                let p0 = v4_xyz(prev_spheres[cone.first_idx]);
                let p1 = v4_xyz(prev_spheres[cone.second_idx]);
                if p0[0].is_finite() && p0[1].is_finite() && p0[2].is_finite()
                    && p1[0].is_finite() && p1[1].is_finite() && p1[2].is_finite()
                {
                    let v0 = v3_sub(v4_xyz(cur_spheres[cone.first_idx]), p0);
                    let v1 = v3_sub(v4_xyz(cur_spheres[cone.second_idx]), p1);
                    let t = d_axis * 0.5 + 0.5;
                    accum.add_vel(v3_add(v0, v3_scale(v3_sub(v1, v0), t)));
                }
            }
        }
    }

    let mut mask = sphere_mask;
    while mask != 0 {
        let s = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        let sph = cur_spheres[s];
        let delta = v3_sub(pos, v4_xyz(sph));
        let sqr_dist = EPS + v3_dot(delta, delta);
        let neg_scale = 1.0 - sph[3] / sqr_dist.sqrt();
        if neg_scale < 0.0 {
            accum.subtract(delta, neg_scale);
            if friction_enabled && s < prev_spheres.len() {
                let prev_c = v4_xyz(prev_spheres[s]);
                if prev_c[0].is_finite() && prev_c[1].is_finite() && prev_c[2].is_finite() {
                    accum.add_vel(v3_sub(v4_xyz(sph), prev_c));
                }
            }
        }
    }
}

fn collide_continuous_cones_and_spheres(
    prev_pos: [f32; 3],
    cur_pos: &mut [f32; 3],
    cur_spheres: &[[f32; 4]],
    prev_spheres: &[[f32; 4]],
    cur_cones: &[ConeData],
    prev_cones: &[ConeData],
    friction_enabled: bool,
    accum: &mut ImpulseAccum,
) {
    let n_sph = cur_spheres.len().min(32);
    let mut sphere_mask: u32 = if n_sph == 32 {
        u32::MAX
    } else {
        (1u32 << n_sph) - 1
    };

    for (ci, cur_cone) in cur_cones.iter().enumerate() {
        if !(cur_cone.radius > 0.0) {
            continue;
        }
        let prev_cone = prev_cones
            .get(ci)
            .copied()
            .filter(|c| c.radius > 0.0)
            .unwrap_or(*cur_cone);

        let prev = v3_sub(prev_pos, prev_cone.center);
        let prev_tuv = v3_cross(prev, prev_cone.axis);
        let prev_dot = v3_dot(prev, prev_cone.axis);
        let prev_r = (prev_dot * prev_cone.slope + prev_cone.radius).max(0.0);

        let mut cur = v3_sub(*cur_pos, cur_cone.center);
        let cur_tuv = v3_cross(cur, cur_cone.axis);
        let mut cur_dot = v3_dot(cur, cur_cone.axis);
        let mut cur_r = (cur_dot * cur_cone.slope + cur_cone.radius).max(0.0);
        let mut cur_sqr_dist = EPS + v3_dot(cur_tuv, cur_tuv);

        let d_pp = v3_dot(prev_tuv, prev_tuv) - prev_r * prev_r;
        let d_pc = v3_dot(prev_tuv, cur_tuv) - prev_r * cur_r;
        let d_cc = cur_sqr_dist - cur_r * cur_r;

        let disc = d_pc * d_pc - d_cc * d_pp;
        let half_b = d_pc - d_pp;
        let minus_a = d_pc - d_cc + half_b;

        let mut ccd_hit = false;
        if disc >= 0.0 && minus_a < -EPS {
            let sqrt_d = disc.sqrt();
            let toi = (half_b + sqrt_d).min(0.0) / minus_a;
            let r_min = prev_r + half_b * minus_a * (cur_r - prev_r);
            if toi < 1.0 && half_b < sqrt_d && disc > minus_a * r_min * r_min * SKELETON_WIDTH {
                ccd_hit = true;
                let mut delta = v3_sub(prev, cur);
                let pos_toi = v3_sub(prev, v3_scale(delta, toi));
                let cur_scaled_axis = v3_scale(cur_cone.axis, cur_cone.half_len);
                let prev_scaled_axis = v3_scale(prev_cone.axis, prev_cone.half_len);
                let delta_scaled_axis = v3_sub(cur_scaled_axis, prev_scaled_axis);
                let mut one_minus_toi = 1.0 - toi;
                let axis_toi = v3_sub(cur_scaled_axis, v3_scale(delta_scaled_axis, one_minus_toi));
                let slope_toi = prev_cone.slope * one_minus_toi + cur_cone.slope * toi;
                let sqr_hl = v3_dot(axis_toi, axis_toi);
                if sqr_hl > EPS {
                    let inv_hl = 1.0 / sqr_hl.sqrt();
                    let dot_toi = v3_dot(pos_toi, axis_toi) * inv_hl;
                    let sqr_dist_toi = v3_dot(pos_toi, pos_toi) - dot_toi * dot_toi;
                    let inv_dist_toi = if sqr_dist_toi > 0.0 {
                        1.0 / sqr_dist_toi.sqrt()
                    } else {
                        0.0
                    };
                    let base_toi = dot_toi + slope_toi * sqr_dist_toi * inv_dist_toi;
                    let scale_toi = base_toi * inv_hl;
                    if scale_toi.abs() < 1.0 {
                        delta = v3_add(delta, v3_scale(delta_scaled_axis, scale_toi));
                        let minus_k = if one_minus_toi > EPS {
                            sqrt_d / (minus_a * one_minus_toi)
                        } else {
                            0.0
                        };
                        one_minus_toi /= 1.0 - minus_k;
                        cur = v3_add(cur, v3_scale(delta, one_minus_toi));
                        cur_dot = v3_dot(cur, cur_cone.axis);
                        cur_r = (cur_dot * cur_cone.slope + cur_cone.radius).max(0.0);
                        cur_sqr_dist = v3_dot(cur, cur) - cur_dot * cur_dot;
                        *cur_pos = v3_add(cur_cone.center, cur);
                    }
                }
            }
        }

        let contact = cur_r * cur_r > cur_sqr_dist;
        if !ccd_hit && !contact {
            sphere_mask &= !cur_cone.both_mask;
        }
        if !contact {
            continue;
        }
        let inv_dist = if cur_sqr_dist > 0.0 {
            1.0 / cur_sqr_dist.sqrt()
        } else {
            0.0
        };
        let base = cur_dot + cur_cone.slope * cur_sqr_dist * inv_dist;
        let left = base < -cur_cone.half_len;
        let right = base > cur_cone.half_len;
        if ccd_hit {
            if !left {
                sphere_mask &= !cur_cone.first_mask;
            }
            if !right {
                sphere_mask &= !cur_cone.second_mask;
            }
        }
        if base >= -cur_cone.half_len && base <= cur_cone.half_len {
            let n_dir = v3_sub(cur, v3_scale(cur_cone.axis, base));
            let scale = (cur_r * inv_dist - 1.0) * cur_cone.sqr_cos;
            accum.add(n_dir, scale);
            if friction_enabled
                && cur_cone.first_idx < prev_spheres.len()
                && cur_cone.second_idx < prev_spheres.len()
            {
                let p0 = v4_xyz(prev_spheres[cur_cone.first_idx]);
                let p1 = v4_xyz(prev_spheres[cur_cone.second_idx]);
                if p0[0].is_finite() && p0[1].is_finite() && p0[2].is_finite()
                    && p1[0].is_finite() && p1[1].is_finite() && p1[2].is_finite()
                {
                    let v0 = v3_sub(v4_xyz(cur_spheres[cur_cone.first_idx]), p0);
                    let v1 = v3_sub(v4_xyz(cur_spheres[cur_cone.second_idx]), p1);
                    let t = cur_dot * 0.5 + 0.5;
                    accum.add_vel(v3_add(v0, v3_scale(v3_sub(v1, v0), t)));
                }
            }
        }
    }

    let mut mask = sphere_mask;
    while mask != 0 {
        let s = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        let cur_sph = cur_spheres[s];
        let prev_sph = prev_spheres
            .get(s)
            .copied()
            .filter(|p| p[0].is_finite() && p[1].is_finite() && p[2].is_finite() && p[3].is_finite())
            .unwrap_or(cur_sph);

        let prev = v3_sub(prev_pos, v4_xyz(prev_sph));
        let mut cur = v3_sub(*cur_pos, v4_xyz(cur_sph));
        let r0 = prev_sph[3];
        let r1 = cur_sph[3];
        let mut sqr_dist = EPS + v3_dot(cur, cur);

        let d_pp = v3_dot(prev, prev) - r0 * r0;
        let d_pc = v3_dot(prev, cur) - r0 * r1;
        let d_cc = sqr_dist - r1 * r1;

        let disc = d_pc * d_pc - d_cc * d_pp;
        let half_b = d_pc - d_pp;
        let minus_a = d_pc - d_cc + half_b;

        if disc >= 0.0 && minus_a < -EPS {
            let sqrt_d = disc.sqrt();
            let toi = (half_b + sqrt_d).min(0.0) / minus_a;
            let r_min = r0 + half_b * minus_a * (r1 - r0);
            if toi < 1.0 && half_b < sqrt_d && disc > minus_a * r_min * r_min * SKELETON_WIDTH {
                let mut one_minus_toi = 1.0 - toi;
                let minus_k = if one_minus_toi > EPS {
                    sqrt_d / (minus_a * one_minus_toi)
                } else {
                    0.0
                };
                one_minus_toi /= 1.0 - minus_k;
                cur = v3_add(cur, v3_scale(v3_sub(prev, cur), one_minus_toi));
                *cur_pos = v3_add(v4_xyz(cur_sph), cur);
                sqr_dist = EPS + v3_dot(cur, cur);
            }
        }

        let neg_scale = 1.0 - r1 / sqr_dist.sqrt();
        if neg_scale < 0.0 {
            accum.subtract(cur, neg_scale);
            if friction_enabled {
                accum.add_vel(v3_sub(v4_xyz(cur_sph), v4_xyz(prev_sph)));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SwSolverKernel Simulation Pipeline (`SwSolverKernel.cpp`)
// ---------------------------------------------------------------------------

impl SimulatedCloth {
    fn simulate_and_destroy(&mut self, frame_dt: f32) {
        if self.cloth.is_null() {
            return;
        }
        let cloth = unsafe { &mut *self.cloth };
        if frame_dt > 0.0 && frame_dt.is_finite() && !cloth.current_particles.is_empty() {
            self.simulate_inner(cloth, frame_dt);
        }
        // Destroy(): pop target constraints and swap target -> start collision shapes.
        cloth.motion_constraints.pop();
        cloth.separation_constraints.pop();
        if !cloth.target_collision_spheres.is_empty() {
            std::mem::swap(&mut cloth.start_collision_spheres, &mut cloth.target_collision_spheres);
            cloth.target_collision_spheres.clear();
        }
        if !cloth.target_collision_planes.is_empty() {
            std::mem::swap(&mut cloth.start_collision_planes, &mut cloth.target_collision_planes);
            cloth.target_collision_planes.clear();
        }
        if !cloth.target_collision_triangles.is_empty() {
            std::mem::swap(
                &mut cloth.start_collision_triangles,
                &mut cloth.target_collision_triangles,
            );
            cloth.target_collision_triangles.clear();
        }
    }

    fn simulate_inner(&mut self, cloth: &mut NvCloth, frame_dt: f32) {
        let factory = IterationStateFactory::new(cloth, frame_dt);
        self.inv_num_iterations = factory.inv_num_iterations;
        let mut state = factory.create(cloth);

        let fabric = if cloth.fabric.is_null() {
            None
        } else {
            Some(unsafe { &*cloth.fabric })
        };

        let stiff_exp = cloth.stiffness_frequency * cloth.prev_iter_dt;
        let eff_tether_stiff = eff_stiffness(cloth.tether_constraint_log_stiffness, stiff_exp);
        let eff_tether_scale = cloth.tether_constraint_scale
            * fabric.map_or(1.0, |f| f.tether_length_scale);
        let eff_drag = eff_stiffness(cloth.drag_log_coefficient, stiff_exp);
        let eff_lift = eff_stiffness(cloth.lift_log_coefficient, stiff_exp);
        let half_density = cloth.fluid_density * 0.5;
        let eff_motion_stiff = eff_stiffness(cloth.motion_constraint_log_stiffness, stiff_exp);
        let eff_self_stiff = eff_stiffness(cloth.self_collision_log_stiffness, stiff_exp);

        let mut cur_bounds = (
            v3_sub(cloth.bbox_center, cloth.bbox_scale),
            v3_add(cloth.bbox_center, cloth.bbox_scale),
        );
        let mut prev_bounds = ([0.0f32; 3], [0.0f32; 3]);
        let mut sleep_test_counter = cloth.sleep_test_counter;
        let mut sleep_pass_counter = cloth.sleep_pass_counter;

        let need_prev_shapes = cloth.enable_continuous_collision || cloth.friction > 0.0;
        if need_prev_shapes && !cloth.start_collision_spheres.is_empty() {
            generate_spheres(
                &mut self.scratch.prev_spheres,
                &cloth.start_collision_spheres,
                &[],
                0.0,
                false,
            );
            generate_cones(
                &mut self.scratch.prev_cones,
                &self.scratch.prev_spheres,
                &cloth.capsule_indices,
            );
        }

        while state.remaining_iterations > 0 {
            // 1. integrateParticles
            step_integrate_particles(cloth, &state);
            // 2. applyWind
            if let Some(fab) = fabric {
                step_apply_wind(cloth, fab, &state, eff_drag, eff_lift, half_density);
            }
            // 3. constrainMotion
            step_constrain_motion(cloth, &state, eff_motion_stiff);
            // 4. constrainTether
            if let Some(fab) = fabric {
                step_constrain_tether(cloth, fab, eff_tether_stiff, eff_tether_scale);
            }
            // 5. solveFabric
            if let Some(fab) = fabric {
                step_solve_fabric(cloth, fab, stiff_exp);
            }
            // 6. constrainSeparation
            step_constrain_separation(cloth, &state);
            // 7. collideParticles (convexes -> triangles -> computeBounds -> spheres/cones/virtuals)
            step_collide_particles(
                cloth,
                &state,
                &mut cur_bounds,
                &mut prev_bounds,
                &mut self.scratch,
                need_prev_shapes,
            );
            // 8. selfCollideParticles
            step_self_collide(cloth, cur_bounds, eff_self_stiff, &mut self.scratch);
            // 9. updateSleepState
            step_update_sleep_state(
                cloth,
                state.iter_dt,
                &mut sleep_test_counter,
                &mut sleep_pass_counter,
            );
            // 10. state.update()
            state.update();
        }

        // Reconcile bounds & sleep counters back to SwCloth.
        cloth.set_particle_bounds(cur_bounds.0, cur_bounds.1);
        cloth.sleep_test_counter = sleep_test_counter;
        cloth.sleep_pass_counter = sleep_pass_counter;
    }
}

fn step_integrate_particles(cloth: &mut NvCloth, state: &IterationState) {
    let n = cloth.current_particles.len().min(cloth.previous_particles.len());
    let has_accel = cloth.particle_accelerations.len() >= n && !cloth.particle_accelerations.is_empty();
    let sqr_dt = state.iter_dt * state.iter_dt;

    if !state.is_turning {
        let scale = state.scale;
        let cur_bias = state.cur_bias;
        let prev_bias = state.prev_bias;
        for i in 0..n {
            let mut cur = cloth.current_particles[i];
            let prev = cloth.previous_particles[i];
            if cur[3] <= 0.0 {
                cur[3] = prev[3];
            }
            let finite = prev[3] > 0.0;
            if finite {
                let accel = if has_accel {
                    let pa = cloth.particle_accelerations[i];
                    [
                        cur_bias[0] + pa[0] * sqr_dt,
                        cur_bias[1] + pa[1] * sqr_dt,
                        cur_bias[2] + pa[2] * sqr_dt,
                    ]
                } else {
                    cur_bias
                };
                let dx = (cur[0] - prev[0]) * scale[0] + accel[0];
                let dy = (cur[1] - prev[1]) * scale[1] + accel[1];
                let dz = (cur[2] - prev[2]) * scale[2] + accel[2];
                cloth.current_particles[i] = [cur[0] + dx, cur[1] + dy, cur[2] + dz, cur[3]];
                cloth.previous_particles[i] = [
                    cur[0] + prev_bias[0],
                    cur[1] + prev_bias[1],
                    cur[2] + prev_bias[2],
                    prev[3],
                ];
            } else {
                cloth.current_particles[i] = cur;
                cloth.previous_particles[i] = [cur[0], cur[1], cur[2], prev[3]];
            }
        }
    } else {
        let cur_m = state.cur_matrix;
        let prev_m = state.prev_matrix;
        let cur_bias = state.cur_bias;
        let prev_bias = state.prev_bias;
        for i in 0..n {
            let mut cur = cloth.current_particles[i];
            let prev = cloth.previous_particles[i];
            if cur[3] <= 0.0 {
                cur[3] = prev[3];
            }
            let finite = prev[3] > 0.0;
            if finite {
                let accel = if has_accel {
                    let pa = cloth.particle_accelerations[i];
                    [
                        cur_bias[0] + pa[0] * sqr_dt,
                        cur_bias[1] + pa[1] * sqr_dt,
                        cur_bias[2] + pa[2] * sqr_dt,
                    ]
                } else {
                    cur_bias
                };
                let c_xyz = v4_xyz(cur);
                let p_xyz = v4_xyz(prev);
                let delta = v3_add(
                    mat3_mul_vec(cur_m, c_xyz),
                    v3_add(mat3_mul_vec(prev_m, p_xyz), accel),
                );
                cloth.current_particles[i] = [
                    cur[0] + delta[0],
                    cur[1] + delta[1],
                    cur[2] + delta[2],
                    cur[3],
                ];
                cloth.previous_particles[i] = [
                    cur[0] + prev_bias[0],
                    cur[1] + prev_bias[1],
                    cur[2] + prev_bias[2],
                    prev[3],
                ];
            } else {
                cloth.current_particles[i] = cur;
                cloth.previous_particles[i] = [cur[0], cur[1], cur[2], prev[3]];
            }
        }
    }
}

fn step_apply_wind(
    cloth: &mut NvCloth,
    fabric: &NvClothFabric,
    state: &IterationState,
    eff_drag: f32,
    eff_lift: f32,
    half_density: f32,
) {
    if (eff_drag == 0.0 && eff_lift == 0.0) || fabric.triangles.len() < 3 || state.iter_dt <= EPS {
        return;
    }
    let n = cloth.current_particles.len().min(cloth.previous_particles.len());
    let inv_iter_dt = 1.0 / state.iter_dt;
    let one_third = 1.0f32 / 3.0;

    for tri in fabric.triangles.chunks_exact(3) {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= n || i1 >= n || i2 >= n {
            continue;
        }
        let c0 = cloth.current_particles[i0];
        let c1 = cloth.current_particles[i1];
        let c2 = cloth.current_particles[i2];
        let p0 = cloth.previous_particles[i0];
        let p1 = cloth.previous_particles[i1];
        let p2 = cloth.previous_particles[i2];

        let current = [
            (c0[0] + c1[0] + c2[0]) * one_third,
            (c0[1] + c1[1] + c2[1]) * one_third,
            (c0[2] + c1[2] + c2[2]) * one_third,
        ];
        let previous = [
            (p0[0] + p1[0] + p2[0]) * one_third,
            (p0[1] + p1[1] + p2[1]) * one_third,
            (p0[2] + p1[2] + p2[2]) * one_third,
        ];

        let mut delta = v3_add(v3_sub(current, previous), state.wind);
        if state.is_turning {
            delta = v3_add(current, mat3_mul_vec(state.rotation_matrix, v3_sub(delta, current)));
        }

        let normal_raw = v3_cross(
            v3_sub(v4_xyz(c2), v4_xyz(c0)),
            v3_sub(v4_xyz(c1), v4_xyz(c0)),
        );
        let double_area = v3_dot(normal_raw, normal_raw).sqrt();
        let inv_sqr_scale = v3_dot(delta, delta);
        if !(inv_sqr_scale >= EPS && double_area > EPS) {
            continue;
        }

        let normal = v3_scale(normal_raw, 1.0 / double_area);
        let delta_len = inv_sqr_scale.sqrt();
        let scale = 1.0 / delta_len;
        let cos_theta = v3_dot(normal, delta) * scale;
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();

        let lift_dir = v3_cross(v3_cross(delta, normal), v3_scale(delta, scale));
        let speed = delta_len * inv_iter_dt;
        let lift = v3_scale(lift_dir, eff_lift * cos_theta * sin_theta * speed);
        let drag = v3_scale(delta, eff_drag * cos_theta.abs() * speed);
        let impulse = v3_scale(v3_add(drag, lift), half_density * double_area);

        if impulse[0].is_finite() && impulse[1].is_finite() && impulse[2].is_finite() {
            for &(idx, w) in &[(i0, c0[3]), (i1, c1[3]), (i2, c2[3])] {
                cloth.current_particles[idx][0] -= impulse[0] * w;
                cloth.current_particles[idx][1] -= impulse[1] * w;
                cloth.current_particles[idx][2] -= impulse[2] * w;
            }
        }
    }
}

fn step_constrain_motion(cloth: &mut NvCloth, state: &IterationState, eff_motion_stiff: f32) {
    if cloth.motion_constraints.start.is_empty() {
        return;
    }
    let n = cloth
        .current_particles
        .len()
        .min(cloth.motion_constraints.start.len());
    let has_target = cloth.motion_constraints.target.len() >= n && !cloth.motion_constraints.target.is_empty();
    let use_target = state.remaining_iterations == 1;
    let alpha = state.cur_alpha();
    let scale = cloth.motion_constraint_scale;
    let bias = cloth.motion_constraint_bias;

    for i in 0..n {
        let sph = if !has_target {
            cloth.motion_constraints.start[i]
        } else if use_target {
            cloth.motion_constraints.target[i]
        } else {
            v4_lerp(
                cloth.motion_constraints.start[i],
                cloth.motion_constraints.target[i],
                alpha,
            )
        };
        let cur = &mut cloth.current_particles[i];
        if !sph[0].is_finite() || !sph[1].is_finite() || !sph[2].is_finite() {
            // Match retail SIMD `constrainMotion` (`0.0 * (NaN - cur) = NaN`): when an
            // upstream bone produces a non-finite motion constraint, propagate NaN to
            // only this particle rather than dropping it unconstrained under gravity
            // (which stretches triangles across the screen).
            cur[0] = sph[0];
            cur[1] = sph[1];
            cur[2] = sph[2];
            continue;
        }
        if !cur[0].is_finite() || !cur[1].is_finite() || !cur[2].is_finite() {
            // Recover cleanly if a previously non-finite bone constraint becomes finite again.
            cur[0] = sph[0];
            cur[1] = sph[1];
            cur[2] = sph[2];
            if let Some(prev) = cloth.previous_particles.get_mut(i) {
                prev[0] = sph[0];
                prev[1] = sph[1];
                prev[2] = sph[2];
            }
            continue;
        }
        let dx = sph[0] - cur[0];
        let dy = sph[1] - cur[1];
        let dz = sph[2] - cur[2];
        let sqr_len = EPS + dx * dx + dy * dy + dz * dz;
        let radius = (sph[3] * scale + bias).max(0.0);
        let slack = 1.0 - radius / sqr_len.sqrt();
        if slack > 0.0 {
            if radius <= 0.0 {
                cur[3] = 0.0;
            }
            let f = slack * eff_motion_stiff;
            cur[0] += dx * f;
            cur[1] += dy * f;
            cur[2] += dz * f;
        }
    }
}

fn step_constrain_tether(
    cloth: &mut NvCloth,
    fabric: &NvClothFabric,
    eff_tether_stiff: f32,
    eff_tether_scale: f32,
) {
    if eff_tether_stiff == 0.0 || fabric.tethers.is_empty() {
        return;
    }
    let n = cloth.current_particles.len();
    if n == 0 {
        return;
    }
    let strands = fabric.tethers.len() / n;
    if strands == 0 {
        return;
    }
    let stiff = eff_tether_stiff / (strands as f32);

    for p in 0..n {
        let pos = v4_xyz(cloth.current_particles[p]);
        let mut offset = [0.0f32; 3];
        for s in 0..strands {
            let (anchor_u32, len) = fabric.tethers[p + s * n];
            let anchor = anchor_u32 as usize;
            if anchor >= n {
                continue;
            }
            let a_pos = v4_xyz(cloth.current_particles[anchor]);
            let delta = v3_sub(a_pos, pos);
            let sqr_len = EPS + v3_dot(delta, delta);
            let slack = 1.0 - (len * eff_tether_scale) / sqr_len.sqrt();
            if slack > 0.0 {
                offset[0] += delta[0] * slack;
                offset[1] += delta[1] * slack;
                offset[2] += delta[2] * slack;
            }
        }
        cloth.current_particles[p][0] = pos[0] + offset[0] * stiff;
        cloth.current_particles[p][1] = pos[1] + offset[1] * stiff;
        cloth.current_particles[p][2] = pos[2] + offset[2] * stiff;
    }
}

fn step_solve_fabric(cloth: &mut NvCloth, fabric: &NvClothFabric, stiff_exp: f32) {
    if cloth.phase_configs.is_empty() || fabric.sets.len() < 2 {
        return;
    }
    let n = cloth.current_particles.len();
    let has_per_constraint_stiff = fabric.stiffness_values.len() >= fabric.restvalues.len()
        && !fabric.stiffness_values.is_empty();

    for cfg in &cloth.phase_configs {
        let p_idx = cfg.phase_index as usize;
        if p_idx >= fabric.phases.len() {
            continue;
        }
        let set_idx = fabric.phases[p_idx] as usize;
        if set_idx + 1 >= fabric.sets.len() {
            continue;
        }
        let c_start = fabric.sets[set_idx] as usize;
        let c_end = (fabric.sets[set_idx + 1] as usize)
            .min(fabric.restvalues.len())
            .min(fabric.indices.len() / 2);
        if c_start >= c_end {
            continue;
        }

        let eff_stiff = eff_stiffness(cfg.stiffness, stiff_exp);
        let eff_mult = eff_stiffness(cfg.stiffness_multiplier, stiff_exp);
        let comp_lim = cfg.compression_limit;
        let stretch_lim = cfg.stretch_limit;
        let use_mult = !(eff_mult == 0.0 && comp_lim == 0.0 && stretch_lim == 0.0);

        for c in c_start..c_end {
            let r = fabric.restvalues[c];
            if !(r > EPS && r.is_finite()) {
                continue;
            }
            let i = fabric.indices[2 * c] as usize;
            let j = fabric.indices[2 * c + 1] as usize;
            if i >= n || j >= n {
                continue;
            }
            let pi = cloth.current_particles[i];
            let pj = cloth.current_particles[j];
            let hx = pj[0] - pi[0];
            let hy = pj[1] - pi[1];
            let hz = pj[2] - pi[2];
            let wi = pi[3];
            let wj = pj[3];
            let st = if has_per_constraint_stiff {
                eff_stiffness(fabric.stiffness_values[c], stiff_exp)
            } else {
                eff_stiff
            };
            let e2 = EPS + hx * hx + hy * hy + hz * hz;
            let mut er = 1.0 - r / e2.sqrt();
            if use_mult {
                er -= eff_mult * er.clamp(comp_lim, stretch_lim);
            }
            let ex = er * st / (EPS + wi + wj);
            if !ex.is_finite() {
                continue;
            }
            let cx = hx * ex;
            let cy = hy * ex;
            let cz = hz * ex;
            cloth.current_particles[i][0] += cx * wi;
            cloth.current_particles[i][1] += cy * wi;
            cloth.current_particles[i][2] += cz * wi;
            cloth.current_particles[j][0] -= cx * wj;
            cloth.current_particles[j][1] -= cy * wj;
            cloth.current_particles[j][2] -= cz * wj;
        }
    }
}

fn step_constrain_separation(cloth: &mut NvCloth, state: &IterationState) {
    if cloth.separation_constraints.start.is_empty() {
        return;
    }
    let n = cloth
        .current_particles
        .len()
        .min(cloth.separation_constraints.start.len());
    let has_target = cloth.separation_constraints.target.len() >= n
        && !cloth.separation_constraints.target.is_empty();
    let use_target = state.remaining_iterations == 1;
    let alpha = state.cur_alpha();

    for i in 0..n {
        let sph = if !has_target {
            cloth.separation_constraints.start[i]
        } else if use_target {
            cloth.separation_constraints.target[i]
        } else {
            v4_lerp(
                cloth.separation_constraints.start[i],
                cloth.separation_constraints.target[i],
                alpha,
            )
        };
        let cur = &mut cloth.current_particles[i];
        let dx = sph[0] - cur[0];
        let dy = sph[1] - cur[1];
        let dz = sph[2] - cur[2];
        let sqr_len = EPS + dx * dx + dy * dy + dz * dz;
        let slack = 1.0 - sph[3] / sqr_len.sqrt();
        if slack < 0.0 {
            cur[0] += dx * slack;
            cur[1] += dy * slack;
            cur[2] += dz * slack;
        }
    }
}

fn step_collide_convexes(cloth: &mut NvCloth, state: &IterationState, scratch: &mut SolverScratch) {
    if cloth.convex_masks.is_empty() || cloth.start_collision_planes.is_empty() {
        return;
    }
    let num_planes = cloth.start_collision_planes.len().min(32);
    let use_target = state.remaining_iterations == 1;
    let alpha = state.cur_alpha();

    scratch.planes.clear();
    if cloth.target_collision_planes.is_empty() {
        scratch
            .planes
            .extend_from_slice(&cloth.start_collision_planes[..num_planes]);
    } else if use_target {
        for i in 0..num_planes {
            let p = cloth
                .target_collision_planes
                .get(i)
                .copied()
                .unwrap_or(cloth.start_collision_planes[i]);
            scratch.planes.push(p);
        }
    } else {
        for i in 0..num_planes {
            let a = cloth.start_collision_planes[i];
            let b = cloth.target_collision_planes.get(i).copied().unwrap_or(a);
            scratch.planes.push(v4_lerp(a, b, alpha));
        }
    }
    scratch.plane_dists.resize(num_planes, 0.0);

    let friction_enabled = cloth.friction > 0.0;
    let friction = cloth.friction;
    let n = cloth.current_particles.len().min(cloth.previous_particles.len());
    let valid_planes_mask: u32 = if num_planes == 32 {
        u32::MAX
    } else {
        (1u32 << num_planes) - 1
    };

    for i in 0..n {
        let cur = v4_xyz(cloth.current_particles[i]);
        let mut inside_mask: u32 = 0;
        for p in 0..num_planes {
            let pl = scratch.planes[p];
            let d = pl[0] * cur[0] + pl[1] * cur[1] + pl[2] * cur[2] + pl[3];
            scratch.plane_dists[p] = d;
            if d < 0.0 {
                inside_mask |= 1u32 << p;
            }
        }
        if inside_mask == 0 {
            continue;
        }
        let mut accum = ImpulseAccum::default();
        for &raw_mask in &cloth.convex_masks {
            let mask = raw_mask & valid_planes_mask;
            if mask == 0 || (inside_mask & mask) != mask {
                continue;
            }
            let mut m = mask;
            let first_p = m.trailing_zeros() as usize;
            m &= m - 1;
            let mut best_p = first_p;
            let mut best_d = scratch.plane_dists[first_p];
            while m != 0 {
                let p = m.trailing_zeros() as usize;
                m &= m - 1;
                let d = scratch.plane_dists[p];
                if d > best_d {
                    best_d = d;
                    best_p = p;
                }
            }
            let pl = scratch.planes[best_p];
            accum.subtract(v4_xyz(pl), best_d);
        }
        if accum.count > 0 {
            let inv_n = 1.0 / (accum.count as f32);
            if friction_enabled {
                let prev = v4_xyz(cloth.previous_particles[i]);
                let imp = calc_friction_impulse(accum.delta, [0.0; 3], cur, prev, inv_n, friction);
                cloth.previous_particles[i][0] -= imp[0];
                cloth.previous_particles[i][1] -= imp[1];
                cloth.previous_particles[i][2] -= imp[2];
            }
            cloth.current_particles[i][0] += accum.delta[0] * inv_n;
            cloth.current_particles[i][1] += accum.delta[1] * inv_n;
            cloth.current_particles[i][2] += accum.delta[2] * inv_n;
        }
    }
}

fn step_collide_triangles(
    cloth: &mut NvCloth,
    state: &IterationState,
    scratch: &mut SolverScratch,
) {
    let num_tris = cloth.start_collision_triangles.len() / 3;
    if num_tris == 0 {
        return;
    }
    let use_target = state.remaining_iterations == 1;
    let alpha = state.cur_alpha();
    let has_target = cloth.target_collision_triangles.len() >= num_tris * 3;

    scratch.triangles.clear();
    scratch.triangles.reserve(num_tris);
    for t in 0..num_tris {
        let base = t * 3;
        let vert = |k: usize| -> [f32; 3] {
            if !has_target {
                cloth.start_collision_triangles[base + k]
            } else if use_target {
                cloth.target_collision_triangles[base + k]
            } else {
                v3_lerp(
                    cloth.start_collision_triangles[base + k],
                    cloth.target_collision_triangles[base + k],
                    alpha,
                )
            }
        };
        let p0 = vert(0);
        let p1 = vert(1);
        let p2 = vert(2);
        let e0 = v3_sub(p1, p0);
        let e1 = v3_sub(p2, p0);
        let n_raw = v3_cross(e0, e1);
        let n_sq = v3_dot(n_raw, n_raw);
        let e0_sq = v3_dot(e0, e0);
        let e1_sq = v3_dot(e1, e1);
        let e0_e1 = v3_dot(e0, e1);
        let det = e0_sq * e1_sq - e0_e1 * e0_e1;
        let denom = e0_sq + e1_sq - 2.0 * e0_e1;
        if !(n_sq > EPS * EPS
            && det.abs() > EPS * EPS
            && e0_sq > EPS * EPS
            && e1_sq > EPS * EPS
            && denom > EPS * EPS
            && n_sq.is_finite()
            && det.is_finite())
        {
            continue;
        }
        let normal = v3_scale(n_raw, 1.0 / n_sq.sqrt());
        scratch.triangles.push(TriangleData {
            p0,
            e0,
            e1,
            normal,
            e0_sq,
            e1_sq,
            e0_e1,
            inv_det: 1.0 / det,
            inv_denom: 1.0 / denom,
            inv_e0_sq: 1.0 / e0_sq,
            inv_e1_sq: 1.0 / e1_sq,
        });
    }

    if scratch.triangles.is_empty() {
        return;
    }

    for p in &mut cloth.current_particles {
        let pos = v4_xyz(*p);
        let mut min_sqr_len = f32::MAX;
        let mut best_n = [0.0f32; 3];
        let mut best_dn = 0.0f32;

        for tri in &scratch.triangles {
            let d = v3_sub(pos, tri.p0);
            let d0 = v3_dot(d, tri.e0);
            let d1 = v3_dot(d, tri.e1);
            let dn = v3_dot(d, tri.normal);

            let s_raw = tri.e1_sq * d0 - tri.e0_e1 * d1;
            let t_raw = tri.e0_sq * d1 - tri.e0_e1 * d0;

            let mut s = if t_raw > 0.0 {
                s_raw * tri.inv_det
            } else {
                d0 * tri.inv_e0_sq
            };
            let mut t = if s_raw > 0.0 {
                t_raw * tri.inv_det
            } else {
                d1 * tri.inv_e1_sq
            };
            if s + t > 1.0 {
                s = (tri.e1_sq - tri.e0_e1 + d0 - d1) * tri.inv_denom;
            }
            s = s.clamp(0.0, 1.0);
            t = t.clamp(0.0, 1.0 - s);

            let off = v3_sub(v3_sub(d, v3_scale(tri.e0, s)), v3_scale(tri.e1, t));
            let mut sqr_len = v3_dot(off, off);
            if dn < 0.0 {
                sqr_len += sqr_len * 1e-4;
            }
            if sqr_len < min_sqr_len {
                min_sqr_len = sqr_len;
                best_n = tri.normal;
                best_dn = dn;
            }
        }

        if best_dn < 0.0 {
            p[0] -= best_n[0] * best_dn;
            p[1] -= best_n[1] * best_dn;
            p[2] -= best_n[2] * best_dn;
        }
    }
}

fn step_collide_particles(
    cloth: &mut NvCloth,
    state: &IterationState,
    cur_bounds: &mut ([f32; 3], [f32; 3]),
    prev_bounds: &mut ([f32; 3], [f32; 3]),
    scratch: &mut SolverScratch,
    need_prev_shapes: bool,
) {
    // 1. Convexes & Triangles
    step_collide_convexes(cloth, state, scratch);
    step_collide_triangles(cloth, state, scratch);

    // 2. computeBounds() — computes particle AABB AND restores cur[i].w = prev[i].w where cur[i].w > 0
    let n = cloth.current_particles.len().min(cloth.previous_particles.len());
    let mut lower = [f32::MAX; 3];
    let mut upper = [-f32::MAX; 3];
    for i in 0..n {
        let p = &mut cloth.current_particles[i];
        lower = v3_min(lower, v4_xyz(*p));
        upper = v3_max(upper, v4_xyz(*p));
        if p[3] > 0.0 {
            p[3] = cloth.previous_particles[i][3];
        }
    }
    if n == 0 {
        lower = [0.0; 3];
        upper = [0.0; 3];
    }
    *prev_bounds = *cur_bounds;
    *cur_bounds = (lower, upper);

    // 3. Spheres, Tapered Capsules (Cones) & Virtual Particles
    if cloth.start_collision_spheres.is_empty() {
        return;
    }

    let last_iter = state.remaining_iterations == 1;
    let alpha = state.cur_alpha();
    generate_spheres(
        &mut scratch.cur_spheres,
        &cloth.start_collision_spheres,
        &cloth.target_collision_spheres,
        alpha,
        last_iter,
    );
    generate_cones(
        &mut scratch.cur_cones,
        &scratch.cur_spheres,
        &cloth.capsule_indices,
    );

    // Broadphase AABB overlap between all spheres and particles
    let mut sph_lo = [f32::MAX; 3];
    let mut sph_hi = [-f32::MAX; 3];
    for &s in &scratch.cur_spheres {
        let r = [s[3]; 3];
        sph_lo = v3_min(sph_lo, v3_sub(v4_xyz(s), r));
        sph_hi = v3_max(sph_hi, v3_add(v4_xyz(s), r));
    }
    let mut part_lo = cur_bounds.0;
    let mut part_hi = cur_bounds.1;
    if cloth.enable_continuous_collision {
        for &s in &scratch.prev_spheres {
            let r = [s[3]; 3];
            sph_lo = v3_min(sph_lo, v3_sub(v4_xyz(s), r));
            sph_hi = v3_max(sph_hi, v3_add(v4_xyz(s), r));
        }
        part_lo = v3_min(part_lo, prev_bounds.0);
        part_hi = v3_max(part_hi, prev_bounds.1);
    }

    let overlap_lo = v3_max(sph_lo, part_lo);
    let overlap_hi = v3_min(sph_hi, part_hi);
    let overlaps = overlap_hi[0] >= overlap_lo[0]
        && overlap_hi[1] >= overlap_lo[1]
        && overlap_hi[2] >= overlap_lo[2];

    if overlaps {
        let friction_enabled = cloth.friction > 0.0;
        let friction = cloth.friction;
        let mass_scale_enabled = cloth.collision_mass_scale > 0.0;
        let mass_scale = cloth.collision_mass_scale;

        if cloth.enable_continuous_collision {
            for i in 0..n {
                let prev_p = v4_xyz(cloth.previous_particles[i]);
                let mut cur_p = v4_xyz(cloth.current_particles[i]);
                let mut accum = ImpulseAccum::default();
                collide_continuous_cones_and_spheres(
                    prev_p,
                    &mut cur_p,
                    &scratch.cur_spheres,
                    &scratch.prev_spheres,
                    &scratch.cur_cones,
                    &scratch.prev_cones,
                    friction_enabled,
                    &mut accum,
                );
                if accum.count == 0 {
                    continue;
                }
                let inv_n = 1.0 / (accum.count as f32);
                if friction_enabled {
                    let imp = calc_friction_impulse(
                        accum.delta,
                        accum.vel,
                        cur_p,
                        prev_p,
                        inv_n,
                        friction,
                    );
                    cloth.previous_particles[i][0] -= imp[0];
                    cloth.previous_particles[i][1] -= imp[1];
                    cloth.previous_particles[i][2] -= imp[2];
                }
                let disp = v3_scale(accum.delta, inv_n);
                if mass_scale_enabled {
                    let d_sq = v3_dot(disp, disp);
                    cloth.current_particles[i][3] *= 1.0 / (1.0 + mass_scale * d_sq);
                }
                cloth.current_particles[i][0] = cur_p[0] + disp[0];
                cloth.current_particles[i][1] = cur_p[1] + disp[1];
                cloth.current_particles[i][2] = cur_p[2] + disp[2];
            }
        } else {
            for i in 0..n {
                let cur_p = v4_xyz(cloth.current_particles[i]);
                let mut accum = ImpulseAccum::default();
                collide_discrete_cones_and_spheres(
                    cur_p,
                    &scratch.cur_spheres,
                    &scratch.prev_spheres,
                    &scratch.cur_cones,
                    friction_enabled,
                    &mut accum,
                );
                if accum.count == 0 {
                    continue;
                }
                let inv_n = 1.0 / (accum.count as f32);
                if friction_enabled {
                    let prev_p = v4_xyz(cloth.previous_particles[i]);
                    let imp = calc_friction_impulse(
                        accum.delta,
                        accum.vel,
                        cur_p,
                        prev_p,
                        inv_n,
                        friction,
                    );
                    cloth.previous_particles[i][0] -= imp[0];
                    cloth.previous_particles[i][1] -= imp[1];
                    cloth.previous_particles[i][2] -= imp[2];
                }
                let disp = v3_scale(accum.delta, inv_n);
                if mass_scale_enabled {
                    let d_sq = v3_dot(disp, disp);
                    cloth.current_particles[i][3] *= 1.0 / (1.0 + mass_scale * d_sq);
                }
                cloth.current_particles[i][0] = cur_p[0] + disp[0];
                cloth.current_particles[i][1] = cur_p[1] + disp[1];
                cloth.current_particles[i][2] = cur_p[2] + disp[2];
            }
        }

        // Virtual particles (always discrete, even when CCD is enabled).
        if !cloth.virtual_particle_indices.is_empty() && !cloth.virtual_particle_weights.is_empty()
        {
            let num_weights = cloth.virtual_particle_weights.len();
            for &[i0_u32, i1_u32, i2_u32, wi_u32] in &cloth.virtual_particle_indices {
                let i0 = i0_u32 as usize;
                let i1 = i1_u32 as usize;
                let i2 = i2_u32 as usize;
                let wi = wi_u32 as usize;
                if i0 >= n || i1 >= n || i2 >= n || wi >= num_weights {
                    continue;
                }
                let w = cloth.virtual_particle_weights[wi];
                let c0 = cloth.current_particles[i0];
                let c1 = cloth.current_particles[i1];
                let c2 = cloth.current_particles[i2];
                let vp = [
                    w[0] * c0[0] + w[1] * c1[0] + w[2] * c2[0],
                    w[0] * c0[1] + w[1] * c1[1] + w[2] * c2[1],
                    w[0] * c0[2] + w[1] * c1[2] + w[2] * c2[2],
                ];
                let mut accum = ImpulseAccum::default();
                collide_discrete_cones_and_spheres(
                    vp,
                    &scratch.cur_spheres,
                    &scratch.prev_spheres,
                    &scratch.cur_cones,
                    friction_enabled,
                    &mut accum,
                );
                if accum.count == 0 {
                    continue;
                }
                let inv_n = 1.0 / (accum.count as f32);
                let disp = v3_scale(accum.delta, inv_n);
                let rw = [w[0] * w[3], w[1] * w[3], w[2] * w[3]];
                let idxs = [i0, i1, i2];

                if friction_enabled {
                    let q0 = cloth.previous_particles[i0];
                    let q1 = cloth.previous_particles[i1];
                    let q2 = cloth.previous_particles[i2];
                    let prev_vp = [
                        w[0] * q0[0] + w[1] * q1[0] + w[2] * q2[0],
                        w[0] * q0[1] + w[1] * q1[1] + w[2] * q2[1],
                        w[0] * q0[2] + w[1] * q1[2] + w[2] * q2[2],
                    ];
                    let imp = calc_friction_impulse(
                        accum.delta,
                        accum.vel,
                        vp,
                        prev_vp,
                        inv_n,
                        friction,
                    );
                    for k in 0..3 {
                        let idx = idxs[k];
                        if cloth.current_particles[idx][3] > 0.0 {
                            cloth.previous_particles[idx][0] -= rw[k] * imp[0];
                            cloth.previous_particles[idx][1] -= rw[k] * imp[1];
                            cloth.previous_particles[idx][2] -= rw[k] * imp[2];
                        }
                    }
                }

                if mass_scale_enabled {
                    let ws = 1.0 / (1.0 + mass_scale * v3_dot(disp, disp)) - 1.0;
                    for k in 0..3 {
                        cloth.current_particles[idxs[k]][3] *= 1.0 + ws * w[k];
                    }
                }

                for k in 0..3 {
                    let idx = idxs[k];
                    if cloth.current_particles[idx][3] > 0.0 {
                        cloth.current_particles[idx][0] += rw[k] * disp[0];
                        cloth.current_particles[idx][1] += rw[k] * disp[1];
                        cloth.current_particles[idx][2] += rw[k] * disp[2];
                    }
                }
            }
        }
    }

    if need_prev_shapes {
        std::mem::swap(&mut scratch.cur_spheres, &mut scratch.prev_spheres);
        std::mem::swap(&mut scratch.cur_cones, &mut scratch.prev_cones);
    }
}

// ---------------------------------------------------------------------------
// SwSelfCollision (`SwSelfCollision.cpp`)
// ---------------------------------------------------------------------------

const KEY_OFFSETS: [u32; 5] = [0x0000_0000, 0x0001_0000, 0x00ff_0000, 0x0100_0000, 0x0101_0000];

fn step_self_collide(
    cloth: &mut NvCloth,
    cur_bounds: ([f32; 3], [f32; 3]),
    eff_self_stiff: f32,
    scratch: &mut SolverScratch,
) {
    let d = cloth.self_collision_distance;
    if !(d > 0.0 && eff_self_stiff > 0.0 && d.is_finite() && eff_self_stiff.is_finite()) {
        return;
    }
    let num_particles = cloth.current_particles.len();
    let use_subset = !cloth.self_collision_indices.is_empty();
    let num_active = if use_subset {
        cloth.self_collision_indices.len()
    } else {
        num_particles
    };
    if num_active < 2 {
        return;
    }

    let lower = cur_bounds.0;
    let edge = v3_max(v3_sub(cur_bounds.1, lower), [EPS; 3]);
    let sweep_axis = longest_axis(edge);
    let h0 = (sweep_axis + 1) % 3;
    let h1 = (sweep_axis + 2) % 3;

    let mut cell = [0.0f32; 3];
    cell[h0] = d.max(edge[h0] / 253.0);
    cell[h1] = d.max(edge[h1] / 253.0);
    cell[sweep_axis] = edge[sweep_axis] / 65533.0;

    let grid_scale = [1.0 / cell[0], 1.0 / cell[1], 1.0 / cell[2]];
    let grid_bias = [
        -lower[0] * grid_scale[0] + 1.0,
        -lower[1] * grid_scale[1] + 1.0,
        -lower[2] * grid_scale[2] + 1.0,
    ];
    let mut grid_max = [254.0f32; 3];
    grid_max[sweep_axis] = 65534.0;

    scratch.sc_keys.clear();
    scratch.sc_keys.reserve(num_active);
    scratch.sc_mapped_indices.clear();
    scratch.sc_mapped_indices.reserve(num_active);

    for i in 0..num_active {
        let p_idx = if use_subset {
            cloth.self_collision_indices[i] as usize
        } else {
            i
        };
        if p_idx >= num_particles {
            continue;
        }
        let pos = cloth.current_particles[p_idx];
        let mut c = [0u32; 3];
        for a in 0..3 {
            let kf = pos[a] * grid_scale[a] + grid_bias[a];
            let clamped = if kf.is_finite() {
                kf.clamp(1.0, grid_max[a])
            } else {
                1.0
            };
            c[a] = clamped.floor() as u32;
        }
        let key = c[sweep_axis] | (c[h0] << 16) | (c[h1] << 24);
        scratch.sc_keys.push(key);
        scratch.sc_mapped_indices.push(p_idx as u32);
    }

    let count = scratch.sc_keys.len();
    if count < 2 {
        return;
    }

    radix_sort_indices(
        &scratch.sc_keys,
        &mut scratch.sc_sorted_indices,
        &mut scratch.sc_scratch_indices,
    );

    scratch.sc_sorted_keys.clear();
    scratch.sc_sorted_keys.reserve(count + 1);
    for &si in &scratch.sc_sorted_indices {
        scratch.sc_sorted_keys.push(scratch.sc_keys[si as usize]);
    }
    scratch.sc_sorted_keys.push(u32::MAX);

    let bucket_dist = (2.0 + (grid_scale[sweep_axis] * d).floor().clamp(0.0, 65535.0)) as u32;
    let d_sq = d * d;
    let use_rest = cloth.rest_positions.len() == num_particles;

    let mut k_first = [0usize; 5];
    let mut k_last = [0usize; 5];

    for i in 0..count {
        let p0 = scratch.sc_mapped_indices[scratch.sc_sorted_indices[i] as usize] as usize;
        let mut pos0 = cloth.current_particles[p0];
        let rest0 = if use_rest {
            cloth.rest_positions[p0]
        } else {
            [0.0; 4]
        };

        let key = scratch.sc_sorted_keys[i];
        let first_key = key - bucket_dist.min(key & 0xffff);
        let last_key = key.saturating_add(bucket_dist).min(key | 0xffff);

        while scratch.sc_sorted_keys[k_last[0]] < last_key {
            k_last[0] += 1;
        }

        // Same cell: j in (i + 1) .. k_last[0]
        for j in (i + 1)..k_last[0] {
            let p1 = scratch.sc_mapped_indices[scratch.sc_sorted_indices[j] as usize] as usize;
            if p1 == p0 {
                continue;
            }
            resolve_self_pair(
                &mut pos0,
                &mut cloth.current_particles[p1],
                rest0,
                if use_rest { cloth.rest_positions[p1] } else { [0.0; 4] },
                use_rest,
                d,
                d_sq,
                eff_self_stiff,
            );
        }

        // Neighbor cells k in 1..5
        for k in 1..5 {
            let fk = first_key.saturating_add(KEY_OFFSETS[k]);
            let lk = last_key.saturating_add(KEY_OFFSETS[k]);
            while scratch.sc_sorted_keys[k_first[k]] < fk {
                k_first[k] += 1;
            }
            while scratch.sc_sorted_keys[k_last[k]] < lk {
                k_last[k] += 1;
            }
            for j in k_first[k]..k_last[k] {
                let p1 = scratch.sc_mapped_indices[scratch.sc_sorted_indices[j] as usize] as usize;
                if p1 == p0 {
                    continue;
                }
                resolve_self_pair(
                    &mut pos0,
                    &mut cloth.current_particles[p1],
                    rest0,
                    if use_rest { cloth.rest_positions[p1] } else { [0.0; 4] },
                    use_rest,
                    d,
                    d_sq,
                    eff_self_stiff,
                );
            }
        }

        cloth.current_particles[p0] = pos0;
    }
}

#[inline]
fn resolve_self_pair(
    pos0: &mut [f32; 4],
    pos1: &mut [f32; 4],
    rest0: [f32; 4],
    rest1: [f32; 4],
    use_rest: bool,
    d: f32,
    d_sq: f32,
    eff_stiff: f32,
) {
    let dx = pos1[0] - pos0[0];
    let dy = pos1[1] - pos0[1];
    let dz = pos1[2] - pos0[2];
    let dist_sq = dx * dx + dy * dy + dz * dz;
    if !(dist_sq <= d_sq) {
        return;
    }
    if use_rest {
        let rdx = rest1[0] - rest0[0];
        let rdy = rest1[1] - rest0[1];
        let rdz = rest1[2] - rest0[2];
        if rdx * rdx + rdy * rdy + rdz * rdz < d_sq {
            return;
        }
    }
    let w0 = pos0[3];
    let w1 = pos1[3];
    let ratio = d / (dist_sq + EPS).sqrt();
    let scale = eff_stiff / (EPS + w0 + w1);
    let f = (1.0 - ratio) * scale;
    let cx = dx * f;
    let cy = dy * f;
    let cz = dz * f;
    pos0[0] += cx * w0;
    pos0[1] += cy * w0;
    pos0[2] += cz * w0;
    pos1[0] -= cx * w1;
    pos1[1] -= cy * w1;
    pos1[2] -= cz * w1;
}

// ---------------------------------------------------------------------------
// Sleep State Machine (`SwSolverKernel.cpp:782-803`)
// ---------------------------------------------------------------------------

fn step_update_sleep_state(
    cloth: &NvCloth,
    iter_dt: f32,
    sleep_test_counter: &mut u32,
    sleep_pass_counter: &mut u32,
) {
    let dt_ms = ((iter_dt * 1000.0).clamp(0.0, u32::MAX as f32) as u32).max(1);
    *sleep_test_counter = sleep_test_counter.wrapping_add(dt_ms);
    if *sleep_test_counter >= cloth.sleep_test_interval {
        let n = cloth.current_particles.len().min(cloth.previous_particles.len());
        let mut max_delta = [0.0f32; 3];
        for i in 0..n {
            for a in 0..3 {
                max_delta[a] =
                    max_delta[a].max((cloth.current_particles[i][a] - cloth.previous_particles[i][a]).abs());
            }
        }
        *sleep_pass_counter = sleep_pass_counter.saturating_add(1);
        let thresh = cloth.sleep_threshold * iter_dt;
        if max_delta[0] >= thresh || max_delta[1] >= thresh || max_delta[2] >= thresh {
            *sleep_pass_counter = 0;
        }
        *sleep_test_counter -= cloth.sleep_test_interval;
    }
}

// ---------------------------------------------------------------------------
// SwInterCollision (`SwInterCollision.cpp`, invoked in `endSimulation`)
// ---------------------------------------------------------------------------

#[inline]
fn resolve_inter_pair(
    j: usize,
    cloth0_idx: usize,
    pos0: &mut [f32; 4],
    imp0: &mut [f32; 4],
    num_cloths: usize,
    d: f32,
    d_sq: f32,
    stiff: f32,
    simulated_cloths: &[UnsafeCell<SimulatedCloth>],
    scratch: &InterCollisionScratch,
) {
    let ci1 = scratch.sorted_indices[j] as usize;
    let cloth1_idx = scratch.cand_cloth[ci1];
    if !scratch.overlap_matrix[cloth0_idx * num_cloths + cloth1_idx] {
        return;
    }
    let p1_idx = scratch.cand_particle[ci1];
    let c1_ptr = unsafe { (*simulated_cloths[cloth1_idx].get()).cloth };
    let c1 = unsafe { &mut *c1_ptr };
    let pos1 = &mut c1.current_particles[p1_idx];
    let imp1 = &mut c1.previous_particles[p1_idx];

    let dx = pos1[0] - pos0[0];
    let dy = pos1[1] - pos0[1];
    let dz = pos1[2] - pos0[2];
    let dist_sq = dx * dx + dy * dy + dz * dz;
    if !(dist_sq <= d_sq) {
        return;
    }
    let w0 = pos0[3];
    let w1 = pos1[3];
    let ratio = d / (dist_sq + EPS).sqrt();
    let scale = stiff / (EPS + w0 + w1);
    let f = (1.0 - ratio) * scale;
    let cx = dx * f;
    let cy = dy * f;
    let cz = dz * f;

    pos0[0] += cx * w0;
    pos0[1] += cy * w0;
    pos0[2] += cz * w0;
    pos1[0] -= cx * w1;
    pos1[1] -= cy * w1;
    pos1[2] -= cz * w1;

    imp0[0] += cx * w0;
    imp0[1] += cy * w0;
    imp0[2] += cz * w0;
    imp1[0] -= cx * w1;
    imp1[1] -= cy * w1;
    imp1[2] -= cz * w1;
}

impl NvClothSolver {
    fn sort_simulated_cloths(&mut self) {
        self.simulated_cloths.sort_by(|a, b| {
            let ca = unsafe { (*a.get()).cloth };
            let cb = unsafe { (*b.get()).cloth };
            let na = if ca.is_null() {
                0
            } else {
                unsafe { (*ca).current_particles.len() }
            };
            let nb = if cb.is_null() {
                0
            } else {
                unsafe { (*cb).current_particles.len() }
            };
            nb.cmp(&na)
        });
    }

    fn inter_collision(&mut self) {
        let d = self.inter_collision_distance;
        let stiff = self.inter_collision_stiffness;
        let nb_iters = self.inter_collision_iterations;
        let filter = self.inter_collision_filter;
        let Self {
            simulated_cloths,
            inter_scratch: scratch,
            ..
        } = self;
        let num_cloths = simulated_cloths.len();
        if nb_iters == 0 || !(d > 0.0 && d.is_finite()) || filter == 0 || num_cloths < 2 {
            return;
        }

        let d_sq = d * d;

        for _ in 0..nb_iters {
            // 1. Broadphase: transform each cloth's inflated local AABB to world space
            scratch.world_bounds.clear();
            scratch.sorted_cloths.clear();
            let mut union_lo = [f32::MAX; 3];
            let mut union_hi = [-f32::MAX; 3];

            for i in 0..num_cloths {
                let c_ptr = unsafe { (*simulated_cloths[i].get()).cloth };
                let cloth = unsafe { &*c_ptr };
                let ext = v3_add(cloth.bbox_scale, [d; 3]);
                let wb = transform_aabb(cloth.target_motion, cloth.bbox_center, ext);
                union_lo = v3_min(union_lo, wb.0);
                union_hi = v3_max(union_hi, wb.1);
                scratch.world_bounds.push(wb);
                scratch.sorted_cloths.push(i);
            }

            let cloth_sweep_axis = longest_axis(v3_sub(union_hi, union_lo));
            scratch.sorted_cloths.sort_by(|&a, &b| {
                scratch.world_bounds[a].0[cloth_sweep_axis]
                    .total_cmp(&scratch.world_bounds[b].0[cloth_sweep_axis])
            });

            scratch.overlap_matrix.clear();
            scratch.overlap_matrix.resize(num_cloths * num_cloths, false);
            scratch.cand_cloth.clear();
            scratch.cand_particle.clear();
            let mut world_lo = [f32::MAX; 3];
            let mut world_hi = [-f32::MAX; 3];

            for si in 0..num_cloths {
                let idx_a = scratch.sorted_cloths[si];
                let sim_a = unsafe { &*simulated_cloths[idx_a].get() };
                let cloth_a_ptr = sim_a.cloth;
                let cloth_a = unsafe { &*cloth_a_ptr };
                let a_pose = cloth_a.target_motion;
                let a_inv = a_pose.inverse();
                let a_user = cloth_a.user_data;
                let a_lo = v3_sub(cloth_a.bbox_center, v3_add(cloth_a.bbox_scale, [d; 3]));
                let a_hi = v3_add(cloth_a.bbox_center, v3_add(cloth_a.bbox_scale, [d; 3]));
                let axis_min = scratch.world_bounds[idx_a].0[cloth_sweep_axis];
                let axis_max = scratch.world_bounds[idx_a].1[cloth_sweep_axis];

                let mut start_j = 0usize;
                while start_j < num_cloths
                    && scratch.world_bounds[scratch.sorted_cloths[start_j]].1[cloth_sweep_axis]
                        < axis_min
                {
                    start_j += 1;
                }

                scratch.overlap_boxes.clear();
                for sj in start_j..num_cloths {
                    if si == sj {
                        continue;
                    }
                    let idx_b = scratch.sorted_cloths[sj];
                    if scratch.world_bounds[idx_b].0[cloth_sweep_axis] > axis_max {
                        break;
                    }
                    let cloth_b_ptr = unsafe { (*simulated_cloths[idx_b].get()).cloth };
                    let cloth_b = unsafe { &*cloth_b_ptr };
                    let pass = unsafe {
                        crate::call_guest8(
                            filter,
                            a_user as u64,
                            cloth_b.user_data as u64,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ) as u8
                    } != 0;
                    if !pass {
                        continue;
                    }
                    scratch.overlap_matrix[idx_a * num_cloths + idx_b] = true;

                    let b_in_a_pose = a_inv.mul(cloth_b.target_motion);
                    let b_ext = v3_add(cloth_b.bbox_scale, [d; 3]);
                    let (b_lo, b_hi) = transform_aabb(b_in_a_pose, cloth_b.bbox_center, b_ext);
                    let ib_lo = v3_max(a_lo, b_lo);
                    let ib_hi = v3_min(a_hi, b_hi);
                    if ib_lo[0] <= ib_hi[0] && ib_lo[1] <= ib_hi[1] && ib_lo[2] <= ib_hi[2] {
                        scratch.overlap_boxes.push((ib_lo, ib_hi));
                    }
                }

                if scratch.overlap_boxes.is_empty() {
                    continue;
                }

                // Cull cloth_a particles against overlap_boxes & transform candidates to world space
                let cloth_a_mut = unsafe { &mut *cloth_a_ptr };
                let imp_scale = sim_a.inv_num_iterations;
                let imp_inv_scale = if imp_scale > EPS { 1.0 / imp_scale } else { 1.0 };
                let n_particles = cloth_a_mut
                    .current_particles
                    .len()
                    .min(cloth_a_mut.previous_particles.len());
                let use_subset = !cloth_a_mut.self_collision_indices.is_empty();
                let active_count = if use_subset {
                    cloth_a_mut.self_collision_indices.len()
                } else {
                    n_particles
                };

                for k in 0..active_count {
                    let p_idx = if use_subset {
                        cloth_a_mut.self_collision_indices[k] as usize
                    } else {
                        k
                    };
                    if p_idx >= n_particles {
                        continue;
                    }
                    let cur = cloth_a_mut.current_particles[p_idx];
                    let inside_any = scratch.overlap_boxes.iter().any(|&(lo, hi)| {
                        cur[0] >= lo[0]
                            && cur[1] >= lo[1]
                            && cur[2] >= lo[2]
                            && cur[0] <= hi[0]
                            && cur[1] <= hi[1]
                            && cur[2] <= hi[2]
                    });
                    if !inside_any {
                        continue;
                    }

                    let prev = cloth_a_mut.previous_particles[p_idx];
                    let cur_w = a_pose.transform_point(v4_xyz(cur));
                    let vel_local = v3_scale(v3_sub(v4_xyz(cur), v4_xyz(prev)), imp_inv_scale);
                    let vel_w = quat_rotate(a_pose.q, vel_local);

                    cloth_a_mut.current_particles[p_idx] = [cur_w[0], cur_w[1], cur_w[2], cur[3]];
                    cloth_a_mut.previous_particles[p_idx] = [vel_w[0], vel_w[1], vel_w[2], prev[3]];

                    world_lo = v3_min(world_lo, cur_w);
                    world_hi = v3_max(world_hi, cur_w);
                    scratch.cand_cloth.push(idx_a);
                    scratch.cand_particle.push(p_idx);
                }
            }

            let num_cands = scratch.cand_cloth.len();
            if num_cands >= 2 {
                // 2. Narrowphase: radix-sort spatial grid + 5-offset sweep
                let edge = v3_max(v3_sub(world_hi, world_lo), [EPS; 3]);
                let sweep_axis = longest_axis(edge);
                let h0 = (sweep_axis + 1) % 3;
                let h1 = (sweep_axis + 2) % 3;

                let mut cell = [0.0f32; 3];
                cell[h0] = d.max(edge[h0] / 253.0);
                cell[h1] = d.max(edge[h1] / 253.0);
                cell[sweep_axis] = edge[sweep_axis] / 65533.0;

                let grid_scale = [1.0 / cell[0], 1.0 / cell[1], 1.0 / cell[2]];
                let grid_bias = [
                    -world_lo[0] * grid_scale[0] + 1.0,
                    -world_lo[1] * grid_scale[1] + 1.0,
                    -world_lo[2] * grid_scale[2] + 1.0,
                ];
                let mut grid_max = [254.0f32; 3];
                grid_max[sweep_axis] = 65534.0;

                scratch.keys.clear();
                scratch.keys.reserve(num_cands);
                for i in 0..num_cands {
                    let c_ptr = unsafe { (*simulated_cloths[scratch.cand_cloth[i]].get()).cloth };
                    let pos = unsafe { (&(*c_ptr).current_particles)[scratch.cand_particle[i]] };
                    let mut c = [0u32; 3];
                    for a in 0..3 {
                        let kf = pos[a] * grid_scale[a] + grid_bias[a];
                        let clamped = if kf.is_finite() {
                            kf.clamp(1.0, grid_max[a])
                        } else {
                            1.0
                        };
                        c[a] = clamped.floor() as u32;
                    }
                    scratch.keys.push(c[sweep_axis] | (c[h0] << 16) | (c[h1] << 24));
                }

                radix_sort_indices(
                    &scratch.keys,
                    &mut scratch.sorted_indices,
                    &mut scratch.scratch_indices,
                );

                scratch.sorted_keys.clear();
                scratch.sorted_keys.reserve(num_cands + 1);
                for &si in &scratch.sorted_indices {
                    scratch.sorted_keys.push(scratch.keys[si as usize]);
                }
                scratch.sorted_keys.push(u32::MAX);

                let bucket_dist =
                    (2.0 + (grid_scale[sweep_axis] * d).floor().clamp(0.0, 65535.0)) as u32;
                let mut k_first = [0usize; 5];
                let mut k_last = [0usize; 5];

                for i in 0..num_cands {
                    let ci0 = scratch.sorted_indices[i] as usize;
                    let cloth0_idx = scratch.cand_cloth[ci0];
                    let p0_idx = scratch.cand_particle[ci0];
                    let c0_ptr = unsafe { (*simulated_cloths[cloth0_idx].get()).cloth };
                    let mut pos0 = unsafe { (&(*c0_ptr).current_particles)[p0_idx] };
                    let mut imp0 = unsafe { (&(*c0_ptr).previous_particles)[p0_idx] };

                    let key = scratch.sorted_keys[i];
                    let first_key = key - bucket_dist.min(key & 0xffff);
                    let last_key = key.saturating_add(bucket_dist).min(key | 0xffff);

                    while scratch.sorted_keys[k_last[0]] < last_key {
                        k_last[0] += 1;
                    }

                    for j in (i + 1)..k_last[0] {
                        resolve_inter_pair(
                            j,
                            cloth0_idx,
                            &mut pos0,
                            &mut imp0,
                            num_cloths,
                            d,
                            d_sq,
                            stiff,
                            simulated_cloths,
                            scratch,
                        );
                    }
                    for k in 1..5 {
                        let fk = first_key.saturating_add(KEY_OFFSETS[k]);
                        let lk = last_key.saturating_add(KEY_OFFSETS[k]);
                        while scratch.sorted_keys[k_first[k]] < fk {
                            k_first[k] += 1;
                        }
                        while scratch.sorted_keys[k_last[k]] < lk {
                            k_last[k] += 1;
                        }
                        for j in k_first[k]..k_last[k] {
                            resolve_inter_pair(
                                j,
                                cloth0_idx,
                                &mut pos0,
                                &mut imp0,
                                num_cloths,
                                d,
                                d_sq,
                                stiff,
                                simulated_cloths,
                                scratch,
                            );
                        }
                    }

                    unsafe {
                        let c0 = &mut *c0_ptr;
                        c0.current_particles[p0_idx] = pos0;
                        c0.previous_particles[p0_idx] = imp0;
                    }
                }
            }

            // 3. Post-transform all candidate particles back to local space
            for i in 0..num_cands {
                let c_idx = scratch.cand_cloth[i];
                let p_idx = scratch.cand_particle[i];
                let sim = unsafe { &*simulated_cloths[c_idx].get() };
                let cloth = unsafe { &mut *sim.cloth };
                let a_inv = cloth.target_motion.inverse();
                let imp_scale = sim.inv_num_iterations;

                let cur_w = cloth.current_particles[p_idx];
                let imp_w = cloth.previous_particles[p_idx];
                let local_cur = a_inv.transform_point(v4_xyz(cur_w));
                let local_vel = quat_rotate(a_inv.q, v3_scale(v4_xyz(imp_w), imp_scale));
                let local_prev = v3_sub(local_cur, local_vel);

                cloth.current_particles[p_idx] = [local_cur[0], local_cur[1], local_cur[2], cur_w[3]];
                cloth.previous_particles[p_idx] = [
                    local_prev[0],
                    local_prev[1],
                    local_prev[2],
                    imp_w[3].max(0.0),
                ];
            }
        }
    }
}

// ---------------------------------------------------------------------------
// MSVC x64 C++ Vtables (Exact Verified Slot Counts: 14, 12, 18, 114)
// ---------------------------------------------------------------------------

static FACTORY_VTBL: [crate::Ptr; 14] = [
    crate::Ptr(factory_dtor as *const ()),                         // 0 (+0x00) ~Factory
    crate::Ptr(factory_get_platform as *const ()),                 // 1 (+0x08) getPlatform
    crate::Ptr(factory_create_fabric as *const ()),                // 2 (+0x10) createFabric
    crate::Ptr(factory_create_cloth as *const ()),                 // 3 (+0x18) createCloth
    crate::Ptr(factory_create_solver as *const ()),                // 4 (+0x20) createSolver
    crate::Ptr(factory_clone as *const ()),                        // 5 (+0x28) clone
    crate::Ptr(factory_extract_fabric_data as *const ()),          // 6 (+0x30) extractFabricData
    crate::Ptr(factory_extract_collision_data as *const ()),       // 7 (+0x38) extractCollisionData
    crate::Ptr(factory_extract_motion_constraints as *const ()),   // 8 (+0x40) extractMotionConstraints
    crate::Ptr(factory_extract_separation_constraints as *const ()), // 9 (+0x48) extractSeparationConstraints
    crate::Ptr(factory_extract_particle_accelerations as *const ()), // 10 (+0x50) extractParticleAccelerations
    crate::Ptr(factory_extract_virtual_particles as *const ()),    // 11 (+0x58) extractVirtualParticles
    crate::Ptr(factory_extract_self_collision_indices as *const ()), // 12 (+0x60) extractSelfCollisionIndices
    crate::Ptr(factory_extract_rest_positions as *const ()),       // 13 (+0x68) extractRestPositions
];

static FABRIC_VTBL: [crate::Ptr; 12] = [
    crate::Ptr(fabric_dtor as *const ()),                              // 0 (+0x00) ~Fabric
    crate::Ptr(fabric_get_factory as *const ()),                       // 1 (+0x08) getFactory
    crate::Ptr(fabric_get_num_phases as *const ()),                    // 2 (+0x10) getNumPhases
    crate::Ptr(fabric_get_num_restvalues as *const ()),                // 3 (+0x18) getNumRestvalues
    crate::Ptr(fabric_get_num_stiffness_values as *const ()),          // 4 (+0x20) getNumStiffnessValues
    crate::Ptr(fabric_get_num_sets as *const ()),                      // 5 (+0x28) getNumSets
    crate::Ptr(fabric_get_num_indices as *const ()),                   // 6 (+0x30) getNumIndices
    crate::Ptr(fabric_get_num_particles as *const ()),                 // 7 (+0x38) getNumParticles
    crate::Ptr(fabric_get_num_tethers as *const ()),                   // 8 (+0x40) getNumTethers
    crate::Ptr(fabric_get_num_triangles as *const ()),                 // 9 (+0x48) getNumTriangles
    crate::Ptr(crate::bridge_entry!(fabric_scale_restvalues)),         // 10 (+0x50) scaleRestvalues
    crate::Ptr(crate::bridge_entry!(fabric_scale_tether_lengths)),     // 11 (+0x58) scaleTetherLengths
];

static SOLVER_VTBL: [crate::Ptr; 18] = [
    crate::Ptr(solver_dtor as *const ()),                                   // 0 (+0x00) ~Solver
    crate::Ptr(solver_add_cloth as *const ()),                              // 1 (+0x08) addCloth
    crate::Ptr(solver_add_cloths as *const ()),                             // 2 (+0x10) addCloths
    crate::Ptr(solver_remove_cloth as *const ()),                           // 3 (+0x18) removeCloth
    crate::Ptr(solver_get_num_cloths as *const ()),                         // 4 (+0x20) getNumCloths
    crate::Ptr(solver_get_cloth_list as *const ()),                         // 5 (+0x28) getClothList
    crate::Ptr(crate::bridge_entry!(solver_begin_simulation)),              // 6 (+0x30) beginSimulation
    crate::Ptr(solver_simulate_chunk as *const ()),                         // 7 (+0x38) simulateChunk
    crate::Ptr(solver_end_simulation as *const ()),                         // 8 (+0x40) endSimulation
    crate::Ptr(solver_get_simulation_chunk_count as *const ()),             // 9 (+0x48) getSimulationChunkCount
    crate::Ptr(crate::bridge_entry!(solver_set_inter_collision_distance)),  // 10 (+0x50) setInterCollisionDistance
    crate::Ptr(solver_get_inter_collision_distance as *const ()),           // 11 (+0x58) getInterCollisionDistance
    crate::Ptr(crate::bridge_entry!(solver_set_inter_collision_stiffness)), // 12 (+0x60) setInterCollisionStiffness
    crate::Ptr(solver_get_inter_collision_stiffness as *const ()),          // 13 (+0x68) getInterCollisionStiffness
    crate::Ptr(solver_set_inter_collision_nb_iterations as *const ()),      // 14 (+0x70) setInterCollisionNbIterations
    crate::Ptr(solver_get_inter_collision_nb_iterations as *const ()),      // 15 (+0x78) getInterCollisionNbIterations
    crate::Ptr(solver_set_inter_collision_filter as *const ()),             // 16 (+0x80) setInterCollisionFilter
    crate::Ptr(solver_has_error as *const ()),                              // 17 (+0x88) hasError
];

static CLOTH_VTBL: [crate::Ptr; 114] = [
    crate::Ptr(cloth_dtor as *const ()),                                   // 0 (+0x000) ~Cloth
    crate::Ptr(cloth_clone as *const ()),                                  // 1 (+0x008) clone
    crate::Ptr(cloth_get_fabric as *const ()),                             // 2 (+0x010) getFabric
    crate::Ptr(cloth_get_factory as *const ()),                            // 3 (+0x018) getFactory
    crate::Ptr(cloth_get_num_particles as *const ()),                      // 4 (+0x020) getNumParticles
    crate::Ptr(cloth_lock_particles as *const ()),                         // 5 (+0x028) lockParticles
    crate::Ptr(cloth_unlock_particles as *const ()),                       // 6 (+0x030) unlockParticles
    crate::Ptr(cloth_get_current_particles as *const ()),                  // 7 (+0x038) getCurrentParticles const
    crate::Ptr(cloth_get_current_particles as *const ()),                  // 8 (+0x040) getCurrentParticles
    crate::Ptr(cloth_get_previous_particles as *const ()),                 // 9 (+0x048) getPreviousParticles const
    crate::Ptr(cloth_get_previous_particles as *const ()),                 // 10 (+0x050) getPreviousParticles
    crate::Ptr(cloth_get_gpu_particles as *const ()),                      // 11 (+0x058) getGpuParticles
    crate::Ptr(cloth_set_translation as *const ()),                        // 12 (+0x060) setTranslation
    crate::Ptr(cloth_set_rotation as *const ()),                           // 13 (+0x068) setRotation
    crate::Ptr(cloth_get_translation as *const ()),                        // 14 (+0x070) getTranslation
    crate::Ptr(cloth_get_rotation as *const ()),                           // 15 (+0x078) getRotation
    crate::Ptr(cloth_clear_inertia as *const ()),                          // 16 (+0x080) clearInertia
    crate::Ptr(cloth_teleport as *const ()),                               // 17 (+0x088) teleport
    crate::Ptr(cloth_teleport_to_location as *const ()),                   // 18 (+0x090) teleportToLocation
    crate::Ptr(cloth_ignore_velocity_discontinuity as *const ()),          // 19 (+0x098) ignoreVelocityDiscontinuity
    crate::Ptr(cloth_get_previous_iteration_dt as *const ()),              // 20 (+0x0a0) getPreviousIterationDt
    crate::Ptr(cloth_set_gravity as *const ()),                            // 21 (+0x0a8) setGravity
    crate::Ptr(cloth_get_gravity as *const ()),                            // 22 (+0x0b0) getGravity
    crate::Ptr(cloth_set_damping as *const ()),                            // 23 (+0x0b8) setDamping
    crate::Ptr(cloth_get_damping as *const ()),                            // 24 (+0x0c0) getDamping
    crate::Ptr(cloth_set_linear_drag as *const ()),                        // 25 (+0x0c8) setLinearDrag
    crate::Ptr(cloth_get_linear_drag as *const ()),                        // 26 (+0x0d0) getLinearDrag
    crate::Ptr(cloth_set_angular_drag as *const ()),                       // 27 (+0x0d8) setAngularDrag
    crate::Ptr(cloth_get_angular_drag as *const ()),                       // 28 (+0x0e0) getAngularDrag
    crate::Ptr(cloth_set_linear_inertia as *const ()),                     // 29 (+0x0e8) setLinearInertia
    crate::Ptr(cloth_get_linear_inertia as *const ()),                     // 30 (+0x0f0) getLinearInertia
    crate::Ptr(cloth_set_angular_inertia as *const ()),                    // 31 (+0x0f8) setAngularInertia
    crate::Ptr(cloth_get_angular_inertia as *const ()),                    // 32 (+0x100) getAngularInertia
    crate::Ptr(cloth_set_centrifugal_inertia as *const ()),                // 33 (+0x108) setCentrifugalInertia
    crate::Ptr(cloth_get_centrifugal_inertia as *const ()),                // 34 (+0x110) getCentrifugalInertia
    crate::Ptr(crate::bridge_entry!(cloth_set_solver_frequency)),          // 35 (+0x118) setSolverFrequency
    crate::Ptr(cloth_get_solver_frequency as *const ()),                   // 36 (+0x120) getSolverFrequency
    crate::Ptr(crate::bridge_entry!(cloth_set_stiffness_frequency)),       // 37 (+0x128) setStiffnessFrequency
    crate::Ptr(cloth_get_stiffness_frequency as *const ()),                // 38 (+0x130) getStiffnessFrequency
    crate::Ptr(cloth_set_acceleration_filter_width as *const ()),          // 39 (+0x138) setAcceleationFilterWidth
    crate::Ptr(cloth_get_acceleration_filter_width as *const ()),          // 40 (+0x140) getAccelerationFilterWidth
    crate::Ptr(cloth_set_phase_config as *const ()),                       // 41 (+0x148) setPhaseConfig
    crate::Ptr(cloth_set_spheres_start_target as *const ()),               // 42 (+0x150) setSpheres(start, target)
    crate::Ptr(cloth_set_spheres_range as *const ()),                      // 43 (+0x158) setSpheres(spheres, first, last)
    crate::Ptr(cloth_get_num_spheres as *const ()),                        // 44 (+0x160) getNumSpheres
    crate::Ptr(cloth_set_capsules as *const ()),                           // 45 (+0x168) setCapsules
    crate::Ptr(cloth_get_num_capsules as *const ()),                       // 46 (+0x170) getNumCapsules
    crate::Ptr(cloth_set_planes_start_target as *const ()),                // 47 (+0x178) setPlanes(start, target)
    crate::Ptr(cloth_set_planes_range as *const ()),                       // 48 (+0x180) setPlanes(planes, first, last)
    crate::Ptr(cloth_get_num_planes as *const ()),                         // 49 (+0x188) getNumPlanes
    crate::Ptr(cloth_set_convexes as *const ()),                           // 50 (+0x190) setConvexes
    crate::Ptr(cloth_get_num_convexes as *const ()),                       // 51 (+0x198) getNumConvexes
    crate::Ptr(cloth_set_triangles_start_target as *const ()),             // 52 (+0x1a0) setTriangles(start, target, first)
    crate::Ptr(cloth_set_triangles_range as *const ()),                    // 53 (+0x1a8) setTriangles(tris, first, last)
    crate::Ptr(cloth_get_num_triangles as *const ()),                      // 54 (+0x1b0) getNumTriangles
    crate::Ptr(cloth_is_continuous_collision_enabled as *const ()),        // 55 (+0x1b8) isContinuousCollisionEnabled
    crate::Ptr(cloth_enable_continuous_collision as *const ()),            // 56 (+0x1c0) enableContinuousCollision
    crate::Ptr(cloth_get_collision_mass_scale as *const ()),               // 57 (+0x1c8) getCollisionMassScale
    crate::Ptr(crate::bridge_entry!(cloth_set_collision_mass_scale)),      // 58 (+0x1d0) setCollisionMassScale
    crate::Ptr(crate::bridge_entry!(cloth_set_friction)),                  // 59 (+0x1d8) setFriction
    crate::Ptr(cloth_get_friction as *const ()),                           // 60 (+0x1e0) getFriction
    crate::Ptr(cloth_set_virtual_particles as *const ()),                  // 61 (+0x1e8) setVirtualParticles
    crate::Ptr(cloth_get_num_virtual_particles as *const ()),              // 62 (+0x1f0) getNumVirtualParticles
    crate::Ptr(cloth_get_num_virtual_particle_weights as *const ()),       // 63 (+0x1f8) getNumVirtualParticleWeights
    crate::Ptr(crate::bridge_entry!(cloth_set_tether_constraint_scale)),   // 64 (+0x200) setTetherConstraintScale
    crate::Ptr(cloth_get_tether_constraint_scale as *const ()),            // 65 (+0x208) getTetherConstraintScale
    crate::Ptr(crate::bridge_entry!(cloth_set_tether_constraint_stiffness)), // 66 (+0x210) setTetherConstraintStiffness
    crate::Ptr(cloth_get_tether_constraint_stiffness as *const ()),        // 67 (+0x218) getTetherConstraintStiffness
    crate::Ptr(cloth_get_motion_constraints as *const ()),                 // 68 (+0x220) getMotionConstraints
    crate::Ptr(cloth_clear_motion_constraints as *const ()),               // 69 (+0x228) clearMotionConstraints
    crate::Ptr(cloth_get_num_motion_constraints as *const ()),             // 70 (+0x230) getNumMotionConstraints
    crate::Ptr(crate::bridge_entry!(cloth_set_motion_constraint_scale_bias)), // 71 (+0x238) setMotionConstraintScaleBias
    crate::Ptr(cloth_get_motion_constraint_scale as *const ()),            // 72 (+0x240) getMotionConstraintScale
    crate::Ptr(cloth_get_motion_constraint_bias as *const ()),             // 73 (+0x248) getMotionConstraintBias
    crate::Ptr(crate::bridge_entry!(cloth_set_motion_constraint_stiffness)), // 74 (+0x250) setMotionConstraintStiffness
    crate::Ptr(cloth_get_motion_constraint_stiffness as *const ()),        // 75 (+0x258) getMotionConstraintStiffness
    crate::Ptr(cloth_get_separation_constraints as *const ()),             // 76 (+0x260) getSeparationConstraints
    crate::Ptr(cloth_clear_separation_constraints as *const ()),           // 77 (+0x268) clearSeparationConstraints
    crate::Ptr(cloth_get_num_separation_constraints as *const ()),         // 78 (+0x270) getNumSeparationConstraints
    crate::Ptr(cloth_clear_interpolation as *const ()),                    // 79 (+0x278) clearInterpolation
    crate::Ptr(cloth_get_particle_accelerations as *const ()),             // 80 (+0x280) getParticleAccelerations
    crate::Ptr(cloth_clear_particle_accelerations as *const ()),           // 81 (+0x288) clearParticleAccelerations
    crate::Ptr(cloth_get_num_particle_accelerations as *const ()),         // 82 (+0x290) getNumParticleAccelerations
    crate::Ptr(cloth_set_wind_velocity as *const ()),                      // 83 (+0x298) setWindVelocity
    crate::Ptr(cloth_get_wind_velocity as *const ()),                      // 84 (+0x2a0) getWindVelocity
    crate::Ptr(crate::bridge_entry!(cloth_set_drag_coefficient)),          // 85 (+0x2a8) setDragCoefficient
    crate::Ptr(cloth_get_drag_coefficient as *const ()),                   // 86 (+0x2b0) getDragCoefficient
    crate::Ptr(crate::bridge_entry!(cloth_set_lift_coefficient)),          // 87 (+0x2b8) setLiftCoefficient
    crate::Ptr(cloth_get_lift_coefficient as *const ()),                   // 88 (+0x2c0) getLiftCoefficient
    crate::Ptr(crate::bridge_entry!(cloth_set_fluid_density)),             // 89 (+0x2c8) setFluidDensity
    crate::Ptr(cloth_get_fluid_density as *const ()),                      // 90 (+0x2d0) getFluidDensity
    crate::Ptr(crate::bridge_entry!(cloth_set_self_collision_distance)),   // 91 (+0x2d8) setSelfCollisionDistance
    crate::Ptr(cloth_get_self_collision_distance as *const ()),            // 92 (+0x2e0) getSelfCollisionDistance
    crate::Ptr(crate::bridge_entry!(cloth_set_self_collision_stiffness)),  // 93 (+0x2e8) setSelfCollisionStiffness
    crate::Ptr(cloth_get_self_collision_stiffness as *const ()),           // 94 (+0x2f0) getSelfCollisionStiffness
    crate::Ptr(cloth_set_self_collision_indices as *const ()),             // 95 (+0x2f8) setSelfCollisionIndices
    crate::Ptr(cloth_get_num_self_collision_indices as *const ()),         // 96 (+0x300) getNumSelfCollisionIndices
    crate::Ptr(cloth_set_rest_positions as *const ()),                     // 97 (+0x308) setRestPositions
    crate::Ptr(cloth_get_num_rest_positions as *const ()),                 // 98 (+0x310) getNumRestPositions
    crate::Ptr(cloth_get_bbox_center as *const ()),                        // 99 (+0x318) getBoundingBoxCenter
    crate::Ptr(cloth_get_bbox_scale as *const ()),                         // 100 (+0x320) getBoundingBoxScale
    crate::Ptr(crate::bridge_entry!(cloth_set_sleep_threshold)),           // 101 (+0x328) setSleepThreshold
    crate::Ptr(cloth_get_sleep_threshold as *const ()),                    // 102 (+0x330) getSleepThreshold
    crate::Ptr(cloth_set_sleep_test_interval as *const ()),                // 103 (+0x338) setSleepTestInterval
    crate::Ptr(cloth_get_sleep_test_interval as *const ()),                // 104 (+0x340) getSleepTestInterval
    crate::Ptr(cloth_set_sleep_after_count as *const ()),                  // 105 (+0x348) setSleepAfterCount
    crate::Ptr(cloth_get_sleep_after_count as *const ()),                  // 106 (+0x350) getSleepAfterCount
    crate::Ptr(cloth_get_sleep_pass_count as *const ()),                   // 107 (+0x358) getSleepPassCount
    crate::Ptr(cloth_is_asleep as *const ()),                              // 108 (+0x360) isAsleep
    crate::Ptr(cloth_put_to_sleep as *const ()),                           // 109 (+0x368) putToSleep
    crate::Ptr(cloth_wake_up as *const ()),                                // 110 (+0x370) wakeUp
    crate::Ptr(cloth_set_user_data as *const ()),                          // 111 (+0x378) setUserData
    crate::Ptr(cloth_get_user_data as *const ()),                          // 112 (+0x380) getUserData
    crate::Ptr(cloth_is_sleeping as *const ()),                            // 113 (+0x388) ClothImpl::isSleeping
];

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        for p in FACTORY_VTBL
            .iter()
            .chain(FABRIC_VTBL.iter())
            .chain(SOLVER_VTBL.iter())
            .chain(CLOTH_VTBL.iter())
        {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
    });
}

// ---------------------------------------------------------------------------
// SwFactory Vtable Implementations (Slots 0..13)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn factory_dtor(
    this: *mut NvClothFactory,
    flags: u32,
) -> *mut NvClothFactory {
    if !this.is_null() && (flags & 1) != 0 {
        unsafe { drop(Box::from_raw(this)) };
    }
    this
}

unsafe extern "win64" fn factory_get_platform(_this: *const NvClothFactory) -> u32 {
    0 // Platform::CPU
}

unsafe extern "win64" fn factory_create_fabric(
    this: *mut NvClothFactory,
    num_particles: u32,
    phase_indices: *const PxRange<u32>,
    sets: *const PxRange<u32>,
    restvalues: *const PxRange<f32>,
    stiffness_values: *const PxRange<f32>,
    indices: *const PxRange<u32>,
    anchors: *const PxRange<u32>,
    tether_lengths: *const PxRange<f32>,
    triangles: *const PxRange<u32>,
) -> *mut NvClothFabric {
    ensure_vtables_registered();
    let phases = unsafe { read_range_vec(phase_indices) };
    let raw_sets = unsafe { read_range_vec(sets) };
    let mut rest = unsafe { read_range_vec(restvalues) };
    let mut stiff = unsafe { read_range_vec(stiffness_values) };
    let mut idx = unsafe { read_range_vec(indices) };
    let anc = unsafe { read_range_vec(anchors) };
    let tlen = unsafe { read_range_vec(tether_lengths) };
    let tris = unsafe { read_range_vec(triangles) };

    let num_constraints = rest.len().min(idx.len() / 2);
    rest.truncate(num_constraints);
    idx.truncate(num_constraints * 2);
    if !stiff.is_empty() {
        stiff.resize(num_constraints, 0.0);
    }

    let mut prefixed_sets = Vec::with_capacity(raw_sets.len() + 1);
    prefixed_sets.push(0u32);
    let mut prev_end = 0u32;
    for s in raw_sets {
        let clamped = (s as usize).min(num_constraints) as u32;
        let mono = clamped.max(prev_end);
        prefixed_sets.push(mono);
        prev_end = mono;
    }

    let num_tethers = anc.len().min(tlen.len());
    let mut tethers = Vec::with_capacity(num_tethers);
    for i in 0..num_tethers {
        tethers.push((anc[i], tlen[i]));
    }

    Box::into_raw(Box::new(NvClothFabric {
        vtbl: &FABRIC_VTBL,
        ref_count: AtomicI32::new(1),
        num_particles,
        factory: this,
        phases,
        sets: prefixed_sets,
        restvalues: rest,
        stiffness_values: stiff,
        indices: idx,
        tethers,
        tether_length_scale: 1.0,
        triangles: tris,
    }))
}

unsafe extern "win64" fn factory_create_cloth(
    this: *mut NvClothFactory,
    particles_range: *const PxRange<[f32; 4]>,
    fabric: *mut NvClothFabric,
) -> *mut NvCloth {
    ensure_vtables_registered();
    let particles = unsafe { read_range_vec(particles_range) };
    if !fabric.is_null() {
        unsafe { (*fabric).ref_count.fetch_add(1, Ordering::AcqRel) };
    }

    let (bbox_center, bbox_scale) = if particles.is_empty() {
        ([0.0; 3], [0.0; 3])
    } else {
        let mut lo = [f32::MAX; 3];
        let mut hi = [-f32::MAX; 3];
        for &p in &particles {
            lo = v3_min(lo, v4_xyz(p));
            hi = v3_max(hi, v4_xyz(p));
        }
        (
            v3_scale(v3_add(hi, lo), 0.5),
            v3_scale(v3_sub(hi, lo), 0.5),
        )
    };

    let num_particles = particles.len() as u32;
    let previous_particles = particles.clone();
    Box::into_raw(Box::new(NvCloth {
        vtbl: &CLOTH_VTBL,
        factory: this,
        fabric,
        num_particles,
        bbox_center,
        bbox_scale,
        gravity: [0.0; 3],
        log_damping: [0.0; 3],
        linear_log_drag: [0.0; 3],
        angular_log_drag: [0.0; 3],
        linear_inertia: [1.0; 3],
        angular_inertia: [1.0; 3],
        centrifugal_inertia: [1.0; 3],
        solver_frequency: 300.0,
        stiffness_frequency: 10.0,
        target_motion: Pose::IDENTITY,
        current_motion: Pose::IDENTITY,
        linear_velocity: [0.0; 3],
        angular_velocity: [0.0; 3],
        ignore_velocity_discontinuity_next_frame: false,
        prev_iter_dt: 0.0,
        iter_dt_avg: MovingAverage::new(30),
        wind: [0.0; 3],
        drag_log_coefficient: 0.0,
        lift_log_coefficient: 0.0,
        fluid_density: 1.0,
        sleep_test_interval: u32::MAX,
        sleep_after_count: u32::MAX,
        sleep_threshold: 0.0,
        sleep_pass_counter: 0,
        sleep_test_counter: 0,
        current_particles: particles,
        previous_particles,
        phase_configs: Vec::new(),
        tether_constraint_log_stiffness: SAFE_LOG2_MIN,
        tether_constraint_scale: 1.0,
        motion_constraints: ConstraintsPair::default(),
        motion_constraint_scale: 1.0,
        motion_constraint_bias: 0.0,
        motion_constraint_log_stiffness: SAFE_LOG2_MIN,
        separation_constraints: ConstraintsPair::default(),
        particle_accelerations: Vec::new(),
        capsule_indices: Vec::new(),
        start_collision_spheres: Vec::new(),
        target_collision_spheres: Vec::new(),
        convex_masks: Vec::new(),
        start_collision_planes: Vec::new(),
        target_collision_planes: Vec::new(),
        start_collision_triangles: Vec::new(),
        target_collision_triangles: Vec::new(),
        enable_continuous_collision: false,
        collision_mass_scale: 0.0,
        friction: 0.0,
        virtual_particle_indices: Vec::new(),
        virtual_particle_weights: Vec::new(),
        self_collision_distance: 0.0,
        self_collision_log_stiffness: SAFE_LOG2_MIN,
        self_collision_indices: Vec::new(),
        rest_positions: Vec::new(),
        user_data: std::ptr::null_mut(),
    }))
}

unsafe extern "win64" fn factory_create_solver(_this: *mut NvClothFactory) -> *mut NvClothSolver {
    ensure_vtables_registered();
    Box::into_raw(Box::new(NvClothSolver {
        vtbl: &SOLVER_VTBL,
        cloths: Vec::new(),
        simulated_cloths: Vec::new(),
        current_dt: 0.0,
        inter_collision_distance: 0.0,
        inter_collision_stiffness: 1.0,
        inter_collision_iterations: 1,
        inter_collision_filter: 0,
        inter_scratch: InterCollisionScratch::default(),
    }))
}

unsafe extern "win64" fn factory_clone(
    this: *mut NvClothFactory,
    cloth: *const NvCloth,
) -> *mut NvCloth {
    if cloth.is_null() {
        return std::ptr::null_mut();
    }
    ensure_vtables_registered();
    let src = unsafe { &*cloth };
    if !src.fabric.is_null() {
        unsafe { (*src.fabric).ref_count.fetch_add(1, Ordering::AcqRel) };
    }
    Box::into_raw(Box::new(NvCloth {
        vtbl: &CLOTH_VTBL,
        factory: this,
        fabric: src.fabric,
        num_particles: src.num_particles,
        bbox_center: src.bbox_center,
        bbox_scale: src.bbox_scale,
        gravity: src.gravity,
        log_damping: src.log_damping,
        linear_log_drag: src.linear_log_drag,
        angular_log_drag: src.angular_log_drag,
        linear_inertia: src.linear_inertia,
        angular_inertia: src.angular_inertia,
        centrifugal_inertia: src.centrifugal_inertia,
        solver_frequency: src.solver_frequency,
        stiffness_frequency: src.stiffness_frequency,
        target_motion: src.target_motion,
        current_motion: src.current_motion,
        linear_velocity: src.linear_velocity,
        angular_velocity: src.angular_velocity,
        ignore_velocity_discontinuity_next_frame: src.ignore_velocity_discontinuity_next_frame,
        prev_iter_dt: src.prev_iter_dt,
        iter_dt_avg: src.iter_dt_avg.clone(),
        wind: src.wind,
        drag_log_coefficient: src.drag_log_coefficient,
        lift_log_coefficient: src.lift_log_coefficient,
        fluid_density: src.fluid_density,
        sleep_test_interval: src.sleep_test_interval,
        sleep_after_count: src.sleep_after_count,
        sleep_threshold: src.sleep_threshold,
        sleep_pass_counter: src.sleep_pass_counter,
        sleep_test_counter: src.sleep_test_counter,
        current_particles: src.current_particles.clone(),
        previous_particles: src.previous_particles.clone(),
        phase_configs: src.phase_configs.clone(),
        tether_constraint_log_stiffness: src.tether_constraint_log_stiffness,
        tether_constraint_scale: src.tether_constraint_scale,
        motion_constraints: src.motion_constraints.clone(),
        motion_constraint_scale: src.motion_constraint_scale,
        motion_constraint_bias: src.motion_constraint_bias,
        motion_constraint_log_stiffness: src.motion_constraint_log_stiffness,
        separation_constraints: src.separation_constraints.clone(),
        particle_accelerations: src.particle_accelerations.clone(),
        capsule_indices: src.capsule_indices.clone(),
        start_collision_spheres: src.start_collision_spheres.clone(),
        target_collision_spheres: src.target_collision_spheres.clone(),
        convex_masks: src.convex_masks.clone(),
        start_collision_planes: src.start_collision_planes.clone(),
        target_collision_planes: src.target_collision_planes.clone(),
        start_collision_triangles: src.start_collision_triangles.clone(),
        target_collision_triangles: src.target_collision_triangles.clone(),
        enable_continuous_collision: src.enable_continuous_collision,
        collision_mass_scale: src.collision_mass_scale,
        friction: src.friction,
        virtual_particle_indices: src.virtual_particle_indices.clone(),
        virtual_particle_weights: src.virtual_particle_weights.clone(),
        self_collision_distance: src.self_collision_distance,
        self_collision_log_stiffness: src.self_collision_log_stiffness,
        self_collision_indices: src.self_collision_indices.clone(),
        rest_positions: src.rest_positions.clone(),
        user_data: src.user_data,
    }))
}

unsafe extern "win64" fn factory_extract_fabric_data(
    _this: *const NvClothFactory,
    fabric: *const NvClothFabric,
    phase_indices: *const PxRange<u32>,
    sets: *const PxRange<u32>,
    restvalues: *const PxRange<f32>,
    stiffness_values: *const PxRange<f32>,
    indices: *const PxRange<u32>,
    anchors: *const PxRange<u32>,
    tether_lengths: *const PxRange<f32>,
    triangles: *const PxRange<u32>,
) {
    if fabric.is_null() {
        return;
    }
    let fab = unsafe { &*fabric };
    unsafe {
        write_range_slice(phase_indices, &fab.phases);
        if fab.sets.len() > 1 {
            write_range_slice(sets, &fab.sets[1..]);
        }
        write_range_slice(restvalues, &fab.restvalues);
        write_range_slice(stiffness_values, &fab.stiffness_values);
        write_range_slice(indices, &fab.indices);

        if !anchors.is_null() && !fab.tethers.is_empty() {
            let range = anchors.read_unaligned();
            let n = range_len(&range).min(fab.tethers.len());
            for i in 0..n {
                range.begin.add(i).write_unaligned(fab.tethers[i].0);
            }
        }
        if !tether_lengths.is_null() && !fab.tethers.is_empty() {
            let range = tether_lengths.read_unaligned();
            let n = range_len(&range).min(fab.tethers.len());
            for i in 0..n {
                range
                    .begin
                    .add(i)
                    .write_unaligned(fab.tethers[i].1 * fab.tether_length_scale);
            }
        }
        write_range_slice(triangles, &fab.triangles);
    }
}

unsafe extern "win64" fn factory_extract_collision_data(
    _this: *const NvClothFactory,
    cloth: *const NvCloth,
    spheres: *const PxRange<[f32; 4]>,
    capsules: *const PxRange<u32>,
    planes: *const PxRange<[f32; 4]>,
    convexes: *const PxRange<u32>,
    triangles: *const PxRange<[f32; 3]>,
) {
    if cloth.is_null() {
        return;
    }
    let c = unsafe { &*cloth };
    unsafe {
        write_range_slice(spheres, &c.start_collision_spheres);
        if !capsules.is_null() && !c.capsule_indices.is_empty() {
            let range = capsules.read_unaligned();
            let cap = range_len(&range);
            let total = c.capsule_indices.len() * 2;
            let n = cap.min(total);
            for i in 0..n {
                let val = c.capsule_indices[i / 2][i % 2];
                range.begin.add(i).write_unaligned(val);
            }
        }
        write_range_slice(planes, &c.start_collision_planes);
        write_range_slice(convexes, &c.convex_masks);
        write_range_slice(triangles, &c.start_collision_triangles);
    }
}

unsafe extern "win64" fn factory_extract_motion_constraints(
    _this: *const NvClothFactory,
    cloth: *const NvCloth,
    dest: *const PxRange<[f32; 4]>,
) {
    if cloth.is_null() {
        return;
    }
    let c = unsafe { &*cloth };
    let src = if !c.motion_constraints.target.is_empty() {
        &c.motion_constraints.target
    } else {
        &c.motion_constraints.start
    };
    unsafe { write_range_slice(dest, src) };
}

unsafe extern "win64" fn factory_extract_separation_constraints(
    _this: *const NvClothFactory,
    cloth: *const NvCloth,
    dest: *const PxRange<[f32; 4]>,
) {
    if cloth.is_null() {
        return;
    }
    let c = unsafe { &*cloth };
    let src = if !c.separation_constraints.target.is_empty() {
        &c.separation_constraints.target
    } else {
        &c.separation_constraints.start
    };
    unsafe { write_range_slice(dest, src) };
}

unsafe extern "win64" fn factory_extract_particle_accelerations(
    _this: *const NvClothFactory,
    cloth: *const NvCloth,
    dest: *const PxRange<[f32; 4]>,
) {
    if cloth.is_null() {
        return;
    }
    let c = unsafe { &*cloth };
    unsafe { write_range_slice(dest, &c.particle_accelerations) };
}

unsafe extern "win64" fn factory_extract_virtual_particles(
    _this: *const NvClothFactory,
    cloth: *const NvCloth,
    dest_indices: *const PxRange<[u32; 4]>,
    dest_weights: *const PxRange<[f32; 3]>,
) {
    if cloth.is_null() {
        return;
    }
    let c = unsafe { &*cloth };
    unsafe {
        write_range_slice(dest_indices, &c.virtual_particle_indices);
        if !dest_weights.is_null() && !c.virtual_particle_weights.is_empty() {
            let range = dest_weights.read_unaligned();
            let n = range_len(&range).min(c.virtual_particle_weights.len());
            for i in 0..n {
                let w = c.virtual_particle_weights[i];
                range.begin.add(i).write_unaligned([w[0], w[1], w[2]]);
            }
        }
    }
}

unsafe extern "win64" fn factory_extract_self_collision_indices(
    _this: *const NvClothFactory,
    cloth: *const NvCloth,
    dest_indices: *const PxRange<u32>,
) {
    if cloth.is_null() {
        return;
    }
    let c = unsafe { &*cloth };
    unsafe { write_range_slice(dest_indices, &c.self_collision_indices) };
}

unsafe extern "win64" fn factory_extract_rest_positions(
    _this: *const NvClothFactory,
    cloth: *const NvCloth,
    dest_rest: *const PxRange<[f32; 4]>,
) {
    if cloth.is_null() {
        return;
    }
    let c = unsafe { &*cloth };
    unsafe { write_range_slice(dest_rest, &c.rest_positions) };
}

// ---------------------------------------------------------------------------
// SwFabric Vtable Implementations (Slots 0..11)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn fabric_dtor(this: *mut NvClothFabric, flags: u32) -> *mut NvClothFabric {
    if !this.is_null() && (flags & 1) != 0 {
        unsafe { drop(Box::from_raw(this)) };
    }
    this
}

unsafe extern "win64" fn fabric_get_factory(this: *const NvClothFabric) -> *mut NvClothFactory {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (*this).factory }
    }
}

unsafe extern "win64" fn fabric_get_num_phases(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).phases.len() as u32 }
    }
}

unsafe extern "win64" fn fabric_get_num_restvalues(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).restvalues.len() as u32 }
    }
}

unsafe extern "win64" fn fabric_get_num_stiffness_values(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).stiffness_values.len() as u32 }
    }
}

unsafe extern "win64" fn fabric_get_num_sets(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).sets.len().saturating_sub(1) as u32 }
    }
}

unsafe extern "win64" fn fabric_get_num_indices(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).indices.len() as u32 }
    }
}

unsafe extern "win64" fn fabric_get_num_particles(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).num_particles }
    }
}

unsafe extern "win64" fn fabric_get_num_tethers(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).tethers.len() as u32 }
    }
}

unsafe extern "win64" fn fabric_get_num_triangles(this: *const NvClothFabric) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { ((*this).triangles.len() / 3) as u32 }
    }
}

unsafe extern "win64" fn fabric_scale_restvalues(this: *mut NvClothFabric, scale: f32) {
    if !this.is_null() {
        let fab = unsafe { &mut *this };
        for r in &mut fab.restvalues {
            *r *= scale;
        }
    }
}
crate::win64_bridge!(fn fabric_scale_restvalues(*mut NvClothFabric, f32) -> ());

unsafe extern "win64" fn fabric_scale_tether_lengths(this: *mut NvClothFabric, scale: f32) {
    if !this.is_null() {
        unsafe { (*this).tether_length_scale *= scale };
    }
}
crate::win64_bridge!(fn fabric_scale_tether_lengths(*mut NvClothFabric, f32) -> ());

// ---------------------------------------------------------------------------
// SwSolver Vtable Implementations (Slots 0..17)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn solver_dtor(this: *mut NvClothSolver, flags: u32) -> *mut NvClothSolver {
    if !this.is_null() && (flags & 1) != 0 {
        unsafe { drop(Box::from_raw(this)) };
    }
    this
}

unsafe extern "win64" fn solver_add_cloth(this: *mut NvClothSolver, cloth: *mut NvCloth) {
    if !this.is_null() && !cloth.is_null() {
        let solver = unsafe { &mut *this };
        if !solver.cloths.contains(&cloth) {
            solver.cloths.push(cloth);
            solver
                .simulated_cloths
                .push(UnsafeCell::new(SimulatedCloth::new(cloth)));
            solver.sort_simulated_cloths();
        }
    }
}

unsafe extern "win64" fn solver_add_cloths(
    this: *mut NvClothSolver,
    cloths: *const PxRange<*mut NvCloth>,
) {
    if !this.is_null() && !cloths.is_null() {
        let list = unsafe { read_range_vec(cloths) };
        let solver = unsafe { &mut *this };
        let mut added = false;
        for c in list {
            if !c.is_null() && !solver.cloths.contains(&c) {
                solver.cloths.push(c);
                solver
                    .simulated_cloths
                    .push(UnsafeCell::new(SimulatedCloth::new(c)));
                added = true;
            }
        }
        if added {
            solver.sort_simulated_cloths();
        }
    }
}

unsafe extern "win64" fn solver_remove_cloth(this: *mut NvClothSolver, cloth: *mut NvCloth) {
    if !this.is_null() {
        let solver = unsafe { &mut *this };
        if let Some(pos) = solver
            .simulated_cloths
            .iter()
            .position(|sc| unsafe { (*sc.get()).cloth == cloth })
        {
            solver.simulated_cloths.swap_remove(pos);
            solver.sort_simulated_cloths();
        }
        if let Some(pos) = solver.cloths.iter().position(|&c| c == cloth) {
            solver.cloths.swap_remove(pos);
        }
    }
}

unsafe extern "win64" fn solver_get_num_cloths(this: *const NvClothSolver) -> i32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).cloths.len() as i32 }
    }
}

unsafe extern "win64" fn solver_get_cloth_list(this: *const NvClothSolver) -> *const *mut NvCloth {
    if this.is_null() {
        std::ptr::null()
    } else {
        let solver = unsafe { &*this };
        if solver.cloths.is_empty() {
            std::ptr::null()
        } else {
            solver.cloths.as_ptr()
        }
    }
}

unsafe extern "win64" fn solver_begin_simulation(this: *mut NvClothSolver, dt: f32) -> u8 {
    if this.is_null() {
        return 0;
    }
    let solver = unsafe { &mut *this };
    if solver.simulated_cloths.is_empty() {
        return 0;
    }
    solver.current_dt = if dt.is_finite() && dt > 0.0 { dt } else { 0.0 };
    1
}
crate::win64_bridge!(fn solver_begin_simulation(*mut NvClothSolver, f32) -> u8);

unsafe extern "win64" fn solver_simulate_chunk(this: *mut NvClothSolver, idx: i32) {
    if this.is_null() || idx < 0 {
        return;
    }
    // Shared &NvClothSolver reference: safe for concurrent workers on distinct `idx`.
    let solver = unsafe { &*this };
    let i = idx as usize;
    if i >= solver.simulated_cloths.len() {
        return;
    }
    let dt = solver.current_dt;
    let sim = unsafe { &mut *solver.simulated_cloths[i].get() };
    sim.simulate_and_destroy(dt);
}

unsafe extern "win64" fn solver_end_simulation(this: *mut NvClothSolver) {
    if !this.is_null() {
        let solver = unsafe { &mut *this };
        solver.inter_collision();
    }
}

unsafe extern "win64" fn solver_get_simulation_chunk_count(this: *const NvClothSolver) -> i32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).simulated_cloths.len() as i32 }
    }
}

unsafe extern "win64" fn solver_set_inter_collision_distance(this: *mut NvClothSolver, d: f32) {
    if !this.is_null() {
        unsafe { (*this).inter_collision_distance = d };
    }
}
crate::win64_bridge!(fn solver_set_inter_collision_distance(*mut NvClothSolver, f32) -> ());

unsafe extern "win64" fn solver_get_inter_collision_distance(this: *const NvClothSolver) -> f32 {
    if this.is_null() {
        0.0
    } else {
        unsafe { (*this).inter_collision_distance }
    }
}

unsafe extern "win64" fn solver_set_inter_collision_stiffness(this: *mut NvClothSolver, s: f32) {
    if !this.is_null() {
        unsafe { (*this).inter_collision_stiffness = s };
    }
}
crate::win64_bridge!(fn solver_set_inter_collision_stiffness(*mut NvClothSolver, f32) -> ());

unsafe extern "win64" fn solver_get_inter_collision_stiffness(this: *const NvClothSolver) -> f32 {
    if this.is_null() {
        1.0
    } else {
        unsafe { (*this).inter_collision_stiffness }
    }
}

unsafe extern "win64" fn solver_set_inter_collision_nb_iterations(
    this: *mut NvClothSolver,
    n: u32,
) {
    if !this.is_null() {
        unsafe { (*this).inter_collision_iterations = n };
    }
}

unsafe extern "win64" fn solver_get_inter_collision_nb_iterations(
    this: *const NvClothSolver,
) -> u32 {
    if this.is_null() {
        1
    } else {
        unsafe { (*this).inter_collision_iterations }
    }
}

unsafe extern "win64" fn solver_set_inter_collision_filter(
    this: *mut NvClothSolver,
    filter: usize,
) {
    if !this.is_null() {
        unsafe { (*this).inter_collision_filter = filter };
    }
}

unsafe extern "win64" fn solver_has_error(_this: *const NvClothSolver) -> u8 {
    0
}

// ---------------------------------------------------------------------------
// SwCloth Vtable Implementations (Slots 0..113)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn cloth_dtor(this: *mut NvCloth, flags: u32) -> *mut NvCloth {
    if !this.is_null() {
        let fabric = unsafe { (*this).fabric };
        if !fabric.is_null() {
            let prev = unsafe { (*fabric).ref_count.fetch_sub(1, Ordering::AcqRel) };
            if prev == 1 {
                unsafe { drop(Box::from_raw(fabric)) };
            }
        }
        if (flags & 1) != 0 {
            unsafe { drop(Box::from_raw(this)) };
        }
    }
    this
}

unsafe extern "win64" fn cloth_clone(
    this: *const NvCloth,
    factory: *mut NvClothFactory,
) -> *mut NvCloth {
    unsafe { factory_clone(factory, this) }
}

unsafe extern "win64" fn cloth_get_fabric(this: *const NvCloth) -> *mut NvClothFabric {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (*this).fabric }
    }
}

unsafe extern "win64" fn cloth_get_factory(this: *const NvCloth) -> *mut NvClothFactory {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (*this).factory }
    }
}

unsafe extern "win64" fn cloth_get_num_particles(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).num_particles }
    }
}

unsafe extern "win64" fn cloth_lock_particles(_this: *const NvCloth) {}

unsafe extern "win64" fn cloth_unlock_particles(_this: *const NvCloth) {}

unsafe extern "win64" fn cloth_get_current_particles(
    this: *mut NvCloth,
    sret_out: *mut MappedRangePxVec4,
) -> *mut MappedRangePxVec4 {
    if !sret_out.is_null() {
        let (begin, end) = if this.is_null() {
            (std::ptr::null_mut(), std::ptr::null_mut())
        } else {
            let cloth = unsafe { &mut *this };
            if cloth.current_particles.is_empty() {
                (std::ptr::null_mut(), std::ptr::null_mut())
            } else {
                let ptr = cloth.current_particles.as_mut_ptr();
                let end_ptr = unsafe { ptr.add(cloth.current_particles.len()) };
                (ptr, end_ptr)
            }
        };
        unsafe {
            sret_out.write_unaligned(MappedRangePxVec4 {
                begin,
                end,
                cloth: this,
                lock_fn: cloth_lock_particles as *const () as usize,
                lock_adj: 0,
                lock_pad: 0,
                unlock_fn: cloth_unlock_particles as *const () as usize,
                unlock_adj: 0,
                unlock_pad: 0,
            });
        }
    }
    sret_out
}

unsafe extern "win64" fn cloth_get_previous_particles(
    this: *mut NvCloth,
    sret_out: *mut MappedRangePxVec4,
) -> *mut MappedRangePxVec4 {
    if !sret_out.is_null() {
        let (begin, end) = if this.is_null() {
            (std::ptr::null_mut(), std::ptr::null_mut())
        } else {
            let cloth = unsafe { &mut *this };
            if cloth.previous_particles.is_empty() {
                (std::ptr::null_mut(), std::ptr::null_mut())
            } else {
                let ptr = cloth.previous_particles.as_mut_ptr();
                let end_ptr = unsafe { ptr.add(cloth.previous_particles.len()) };
                (ptr, end_ptr)
            }
        };
        unsafe {
            sret_out.write_unaligned(MappedRangePxVec4 {
                begin,
                end,
                cloth: this,
                lock_fn: cloth_lock_particles as *const () as usize,
                lock_adj: 0,
                lock_pad: 0,
                unlock_fn: cloth_unlock_particles as *const () as usize,
                unlock_adj: 0,
                unlock_pad: 0,
            });
        }
    }
    sret_out
}

unsafe extern "win64" fn cloth_get_gpu_particles(
    _this: *mut NvCloth,
    sret_out: *mut [usize; 3],
) -> *mut [usize; 3] {
    if !sret_out.is_null() {
        unsafe { sret_out.write_unaligned([0, 0, 0]) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_translation(this: *mut NvCloth, trans: *const [f32; 3]) {
    if !this.is_null() && !trans.is_null() {
        let t = unsafe { trans.read_unaligned() };
        let cloth = unsafe { &mut *this };
        if t != cloth.target_motion.p {
            cloth.target_motion.p = t;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_set_rotation(this: *mut NvCloth, rot: *const [f32; 4]) {
    if !this.is_null() && !rot.is_null() {
        let q = unsafe { rot.read_unaligned() };
        let cloth = unsafe { &mut *this };
        if v4_dist_sq(q, cloth.target_motion.q) != 0.0 {
            cloth.target_motion.q = q;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_translation(this: *const NvCloth) -> *const [f32; 3] {
    if this.is_null() {
        static ZERO: [f32; 3] = [0.0, 0.0, 0.0];
        &ZERO
    } else {
        unsafe { &(*this).target_motion.p }
    }
}

unsafe extern "win64" fn cloth_get_rotation(this: *const NvCloth) -> *const [f32; 4] {
    if this.is_null() {
        static IDENT: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        &IDENT
    } else {
        unsafe { &(*this).target_motion.q }
    }
}

unsafe extern "win64" fn cloth_clear_inertia(this: *mut NvCloth) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        cloth.current_motion = cloth.target_motion;
        cloth.linear_velocity = [0.0; 3];
        cloth.angular_velocity = [0.0; 3];
        cloth.wake_up();
    }
}

unsafe extern "win64" fn cloth_teleport(this: *mut NvCloth, delta: *const [f32; 3]) {
    if !this.is_null() && !delta.is_null() {
        let d = unsafe { delta.read_unaligned() };
        let cloth = unsafe { &mut *this };
        cloth.current_motion.p = v3_add(cloth.current_motion.p, d);
        cloth.target_motion.p = v3_add(cloth.target_motion.p, d);
    }
}

unsafe extern "win64" fn cloth_teleport_to_location(
    this: *mut NvCloth,
    trans: *const [f32; 3],
    rot: *const [f32; 4],
) {
    if !this.is_null() && !trans.is_null() && !rot.is_null() {
        let p = unsafe { trans.read_unaligned() };
        let q = unsafe { rot.read_unaligned() };
        let cloth = unsafe { &mut *this };
        cloth.current_motion = Pose { p, q };
        cloth.target_motion = Pose { p, q };
    }
}

unsafe extern "win64" fn cloth_ignore_velocity_discontinuity(this: *mut NvCloth) {
    if !this.is_null() {
        unsafe { (*this).ignore_velocity_discontinuity_next_frame = true };
    }
}

unsafe extern "win64" fn cloth_get_previous_iteration_dt(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        unsafe { (*this).prev_iter_dt }
    }
}

unsafe extern "win64" fn cloth_set_gravity(this: *mut NvCloth, v: *const [f32; 3]) {
    if !this.is_null() && !v.is_null() {
        let g = unsafe { v.read_unaligned() };
        let cloth = unsafe { &mut *this };
        if g != cloth.gravity {
            cloth.gravity = g;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_gravity(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [0.0; 3]
        } else {
            unsafe { (*this).gravity }
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_damping(this: *mut NvCloth, v: *const [f32; 3]) {
    if !this.is_null() && !v.is_null() {
        let d = unsafe { v.read_unaligned() };
        let val = safe_log2_v3(v3_sub([1.0; 3], d));
        let cloth = unsafe { &mut *this };
        if val != cloth.log_damping {
            cloth.log_damping = val;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_damping(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [0.0; 3]
        } else {
            v3_sub([1.0; 3], safe_exp2_v3(unsafe { (*this).log_damping }))
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_linear_drag(this: *mut NvCloth, v: *const [f32; 3]) {
    if !this.is_null() && !v.is_null() {
        let d = unsafe { v.read_unaligned() };
        let val = safe_log2_v3(v3_sub([1.0; 3], d));
        let cloth = unsafe { &mut *this };
        if val != cloth.linear_log_drag {
            cloth.linear_log_drag = val;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_linear_drag(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [0.0; 3]
        } else {
            v3_sub([1.0; 3], safe_exp2_v3(unsafe { (*this).linear_log_drag }))
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_angular_drag(this: *mut NvCloth, v: *const [f32; 3]) {
    if !this.is_null() && !v.is_null() {
        let d = unsafe { v.read_unaligned() };
        let val = safe_log2_v3(v3_sub([1.0; 3], d));
        let cloth = unsafe { &mut *this };
        if val != cloth.angular_log_drag {
            cloth.angular_log_drag = val;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_angular_drag(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [0.0; 3]
        } else {
            v3_sub([1.0; 3], safe_exp2_v3(unsafe { (*this).angular_log_drag }))
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_linear_inertia(this: *mut NvCloth, v: *const [f32; 3]) {
    if !this.is_null() && !v.is_null() {
        let val = unsafe { v.read_unaligned() };
        let cloth = unsafe { &mut *this };
        if val != cloth.linear_inertia {
            cloth.linear_inertia = val;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_linear_inertia(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [1.0; 3]
        } else {
            unsafe { (*this).linear_inertia }
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_angular_inertia(this: *mut NvCloth, v: *const [f32; 3]) {
    if !this.is_null() && !v.is_null() {
        let val = unsafe { v.read_unaligned() };
        let cloth = unsafe { &mut *this };
        if val != cloth.angular_inertia {
            cloth.angular_inertia = val;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_angular_inertia(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [1.0; 3]
        } else {
            unsafe { (*this).angular_inertia }
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_centrifugal_inertia(this: *mut NvCloth, v: *const [f32; 3]) {
    if !this.is_null() && !v.is_null() {
        let val = unsafe { v.read_unaligned() };
        let cloth = unsafe { &mut *this };
        if val != cloth.centrifugal_inertia {
            cloth.centrifugal_inertia = val;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_centrifugal_inertia(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [1.0; 3]
        } else {
            unsafe { (*this).centrifugal_inertia }
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_solver_frequency(this: *mut NvCloth, freq: f32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if freq != cloth.solver_frequency {
            cloth.solver_frequency = freq;
            cloth.iter_dt_avg.reset();
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_solver_frequency(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_solver_frequency(this: *const NvCloth) -> f32 {
    if this.is_null() {
        300.0
    } else {
        unsafe { (*this).solver_frequency }
    }
}

unsafe extern "win64" fn cloth_set_stiffness_frequency(this: *mut NvCloth, freq: f32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if freq != cloth.stiffness_frequency {
            cloth.stiffness_frequency = freq;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_stiffness_frequency(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_stiffness_frequency(this: *const NvCloth) -> f32 {
    if this.is_null() {
        10.0
    } else {
        unsafe { (*this).stiffness_frequency }
    }
}

unsafe extern "win64" fn cloth_set_acceleration_filter_width(this: *mut NvCloth, n: u32) {
    if !this.is_null() {
        unsafe { (*this).iter_dt_avg.resize(n as usize) };
    }
}

unsafe extern "win64" fn cloth_get_acceleration_filter_width(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).iter_dt_avg.size() }
    }
}

unsafe extern "win64" fn cloth_set_phase_config(
    this: *mut NvCloth,
    configs: *const PxRange<PhaseConfig>,
) {
    if this.is_null() {
        return;
    }
    let raw = unsafe { read_range_vec(configs) };
    let cloth = unsafe { &mut *this };
    cloth.phase_configs.clear();
    for c in raw {
        if c.stiffness > 0.0 {
            let comp = if c.compression_limit > 0.0 {
                1.0 - 1.0 / c.compression_limit
            } else {
                0.0
            };
            let stretch = if c.stretch_limit > 0.0 {
                1.0 - 1.0 / c.stretch_limit
            } else {
                0.0
            };
            cloth.phase_configs.push(PhaseConfig {
                stiffness: safe_log2(1.0 - c.stiffness),
                stiffness_multiplier: safe_log2(c.stiffness_multiplier),
                compression_limit: comp,
                stretch_limit: stretch,
                phase_index: c.phase_index,
                padding: c.padding,
            });
        }
    }
    cloth.wake_up();
}

/// Shared `[first..last)` splice helper preserving start->target interpolation semantics
/// matching `ClothImpl.h:536-619` / `711-809` / `903-1007`.
fn splice_interpolated_shapes<T: Copy>(
    start: &mut Vec<T>,
    target: &mut Vec<T>,
    items: &[T],
    first: usize,
    last: usize,
    zero: T,
) -> (usize, usize, isize) {
    let old_size = start.len();
    let first = first.min(old_size);
    let last = last.clamp(first, old_size);
    let count = items.len();
    let new_size = old_size - (last - first) + count;
    let delta = (new_size as isize) - (old_size as isize);

    if old_size == 0 && new_size == 0 {
        return (first, last, 0);
    }
    if old_size == 0 {
        start.extend_from_slice(items);
        target.clear();
        return (first, last, delta);
    }

    if target.len() < old_size {
        let cur_len = target.len();
        target.extend_from_slice(&start[cur_len..old_size]);
    }
    let max_size = old_size.max(new_size);
    start.resize(max_size, zero);
    target.resize(max_size, zero);

    if delta != 0 {
        let dst = (last as isize + delta) as usize;
        start.copy_within(last..old_size, dst);
        target.copy_within(last..old_size, dst);
        if delta > 0 {
            let add_end = (last as isize + delta) as usize;
            for i in last..add_end {
                start[i] = items[i - first];
            }
        }
        start.truncate(new_size);
        target.truncate(new_size);
    }

    target[first..first + count].copy_from_slice(items);
    (first, last, delta)
}

unsafe extern "win64" fn cloth_set_spheres_start_target(
    this: *mut NvCloth,
    start_range: *const PxRange<[f32; 4]>,
    target_range: *const PxRange<[f32; 4]>,
) {
    if this.is_null() {
        return;
    }
    let start = unsafe { read_range_vec(start_range) };
    let target = unsafe { read_range_vec(target_range) };
    let ns = start.len().min(32);
    let nt = target.len().min(32);
    let cloth = unsafe { &mut *this };
    cloth.start_collision_spheres.clear();
    cloth.start_collision_spheres.extend_from_slice(&start[..ns]);
    cloth.target_collision_spheres.clear();
    cloth.target_collision_spheres.extend_from_slice(&target[..nt]);
    cloth.wake_up();
}

#[inline]
fn boundary_nan_diag_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("PORT_NAN_BOUNDARY").is_ok_and(|v| v.trim() != "0" && !v.trim().is_empty()))
}

#[cold]
#[inline(never)]
fn log_boundary_nan_spheres(where_name: &str, cloth_n: usize, ptr: usize, items: &[[f32; 4]]) {
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let mut bad = 0usize;
    let mut first_idx = 0usize;
    let mut first_val = [0.0f32; 4];
    for (i, s) in items.iter().enumerate() {
        if !s[0].is_finite() || !s[1].is_finite() || !s[2].is_finite() || !s[3].is_finite() {
            if bad == 0 {
                first_idx = i;
                first_val = *s;
            }
            bad += 1;
        }
    }
    if bad > 0 && COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 64 {
        let first_addr = ptr.wrapping_add(first_idx * 16);
        let detail = format!(
            "n_particles={cloth_n} ptr={ptr:#x} len={} bad={bad} first_idx={first_idx} val={first_val:?}",
            items.len()
        );
        log::warn!("[diag-boundary] nvcloth::{where_name} {detail} first_addr={first_addr:#x}");
        crate::report_boundary_nan(&format!("nvcloth::{where_name}"), first_addr, 16, &detail);
    }
}

unsafe extern "win64" fn cloth_set_spheres_range(
    this: *mut NvCloth,
    spheres_range: *const PxRange<[f32; 4]>,
    first: u32,
    last: u32,
) {
    if this.is_null() {
        return;
    }
    let raw = unsafe { read_range_vec(spheres_range) };
    let cloth = unsafe { &mut *this };
    if boundary_nan_diag_enabled() && !raw.is_empty() {
        let ptr = if spheres_range.is_null() {
            0
        } else {
            unsafe { core::ptr::read_unaligned(spheres_range).begin as usize }
        };
        log_boundary_nan_spheres("set_spheres_range", cloth.current_particles.len(), ptr, &raw);
    }
    let old_size = cloth.start_collision_spheres.len();
    let f = (first as usize).min(old_size);
    let l = (last as usize).clamp(f, old_size);
    let max_add = 32usize.saturating_sub(old_size - (l - f));
    let items = &raw[..raw.len().min(max_add)];

    let (_f, l, delta) = splice_interpolated_shapes(
        &mut cloth.start_collision_spheres,
        &mut cloth.target_collision_spheres,
        items,
        f,
        l,
        [0.0; 4],
    );

    if old_size > 0 && delta != 0 {
        let threshold = (l as isize + delta.min(0)) as u32;
        let mut i = 0;
        while i < cloth.capsule_indices.len() {
            let mut pair = cloth.capsule_indices[i];
            let mut removed = false;
            for idx in &mut pair {
                if *idx >= threshold {
                    let updated = (*idx as isize) + delta;
                    if updated < threshold as isize {
                        removed = true;
                    } else {
                        *idx = updated as u32;
                    }
                }
            }
            if removed {
                cloth.capsule_indices.swap_remove(i);
            } else {
                cloth.capsule_indices[i] = pair;
                i += 1;
            }
        }
    }
    cloth.wake_up();
}

unsafe extern "win64" fn cloth_get_num_spheres(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).start_collision_spheres.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_capsules(
    this: *mut NvCloth,
    capsules_range: *const PxRange<u32>,
    first: u32,
    last: u32,
) {
    if this.is_null() {
        return;
    }
    let raw = unsafe { read_range_vec(capsules_range) };
    let pairs_len = raw.len() / 2;
    let cloth = unsafe { &mut *this };
    let old_size = cloth.capsule_indices.len();
    let f = (first as usize).min(old_size);
    let l = (last as usize).clamp(f, old_size);
    let max_add = 32usize.saturating_sub(old_size - (l - f));
    let count = pairs_len.min(max_add);

    let pairs = (0..count).map(|i| [raw[2 * i], raw[2 * i + 1]]);
    cloth.capsule_indices.splice(f..l, pairs);
    cloth.wake_up();
}

unsafe extern "win64" fn cloth_get_num_capsules(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).capsule_indices.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_planes_start_target(
    this: *mut NvCloth,
    start_range: *const PxRange<[f32; 4]>,
    target_range: *const PxRange<[f32; 4]>,
) {
    if this.is_null() {
        return;
    }
    let start = unsafe { read_range_vec(start_range) };
    let target = unsafe { read_range_vec(target_range) };
    let ns = start.len().min(32);
    let nt = target.len().min(32);
    let cloth = unsafe { &mut *this };
    cloth.start_collision_planes.clear();
    cloth.start_collision_planes.extend_from_slice(&start[..ns]);
    cloth.target_collision_planes.clear();
    cloth.target_collision_planes.extend_from_slice(&target[..nt]);
    cloth.wake_up();
}

unsafe extern "win64" fn cloth_set_planes_range(
    this: *mut NvCloth,
    planes_range: *const PxRange<[f32; 4]>,
    first: u32,
    last: u32,
) {
    if this.is_null() {
        return;
    }
    let raw = unsafe { read_range_vec(planes_range) };
    let cloth = unsafe { &mut *this };
    let old_size = cloth.start_collision_planes.len();
    let f = (first as usize).min(old_size);
    let l = (last as usize).clamp(f, old_size);
    let max_add = 32usize.saturating_sub(old_size - (l - f));
    let items = &raw[..raw.len().min(max_add)];

    let (_f, l, delta) = splice_interpolated_shapes(
        &mut cloth.start_collision_planes,
        &mut cloth.target_collision_planes,
        items,
        f,
        l,
        [0.0; 4],
    );

    if old_size > 0 && delta != 0 {
        let shift_pos = (l as isize + delta.min(0)) as u32;
        let mask = if shift_pos >= 32 {
            u32::MAX
        } else {
            (1u32 << shift_pos).wrapping_sub(1)
        };
        let mut i = 0;
        while i < cloth.convex_masks.len() {
            let old_m = cloth.convex_masks[i];
            let mut convex = old_m & mask;
            if delta < 0 {
                let sh = (-delta) as u32;
                if sh < 32 {
                    convex |= (old_m >> sh) & !mask;
                }
            } else {
                let sh = delta as u32;
                if sh < 32 {
                    convex |= (old_m & !mask) << sh;
                }
            }
            if convex != 0 {
                cloth.convex_masks[i] = convex;
                i += 1;
            } else {
                cloth.convex_masks.swap_remove(i);
            }
        }
    }
    cloth.wake_up();
}

unsafe extern "win64" fn cloth_get_num_planes(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).start_collision_planes.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_convexes(
    this: *mut NvCloth,
    masks_range: *const PxRange<u32>,
    first: u32,
    last: u32,
) {
    if this.is_null() {
        return;
    }
    let raw = unsafe { read_range_vec(masks_range) };
    let cloth = unsafe { &mut *this };
    let old_size = cloth.convex_masks.len();
    let f = (first as usize).min(old_size);
    let l = (last as usize).clamp(f, old_size);
    let max_add = 32usize.saturating_sub(old_size - (l - f));
    let count = raw.len().min(max_add);
    cloth.convex_masks.splice(f..l, raw[..count].iter().copied());
    cloth.wake_up();
}

unsafe extern "win64" fn cloth_get_num_convexes(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).convex_masks.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_triangles_start_target(
    this: *mut NvCloth,
    start_range: *const PxRange<[f32; 3]>,
    target_range: *const PxRange<[f32; 3]>,
    first: u32,
) {
    if this.is_null() {
        return;
    }
    let start = unsafe { read_range_vec(start_range) };
    let target = unsafe { read_range_vec(target_range) };
    let start_count = (start.len() / 3) * 3;
    let target_count = (target.len() / 3) * 3;
    let cloth = unsafe { &mut *this };
    let first_v = (first as usize).saturating_mul(3).min(cloth.start_collision_triangles.len());
    if target_count > 0 && first_v > 0 && cloth.target_collision_triangles.len() < first_v {
        let cur_len = cloth.target_collision_triangles.len();
        cloth
            .target_collision_triangles
            .extend_from_slice(&cloth.start_collision_triangles[cur_len..first_v]);
    }
    cloth.start_collision_triangles.truncate(first_v);
    cloth.start_collision_triangles.extend_from_slice(&start[..start_count]);
    if target_count > 0 {
        cloth.target_collision_triangles.truncate(first_v);
        cloth.target_collision_triangles.extend_from_slice(&target[..target_count]);
    } else if first_v == 0 {
        cloth.target_collision_triangles.clear();
    } else {
        cloth.target_collision_triangles.truncate(first_v);
    }
    cloth.wake_up();
}

unsafe extern "win64" fn cloth_set_triangles_range(
    this: *mut NvCloth,
    tris_range: *const PxRange<[f32; 3]>,
    first: u32,
    last: u32,
) {
    if this.is_null() {
        return;
    }
    let raw = unsafe { read_range_vec(tris_range) };
    let count = (raw.len() / 3) * 3;
    let cloth = unsafe { &mut *this };
    let old_size = cloth.start_collision_triangles.len();
    let f = (first as usize).saturating_mul(3).min(old_size);
    let l = (last as usize).saturating_mul(3).clamp(f, old_size);

    splice_interpolated_shapes(
        &mut cloth.start_collision_triangles,
        &mut cloth.target_collision_triangles,
        &raw[..count],
        f,
        l,
        [0.0; 3],
    );
    cloth.wake_up();
}

unsafe extern "win64" fn cloth_get_num_triangles(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { ((*this).start_collision_triangles.len() / 3) as u32 }
    }
}

unsafe extern "win64" fn cloth_is_continuous_collision_enabled(this: *const NvCloth) -> u8 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).enable_continuous_collision as u8 }
    }
}

unsafe extern "win64" fn cloth_enable_continuous_collision(this: *mut NvCloth, enable: u8) {
    if !this.is_null() {
        let b = enable != 0;
        let cloth = unsafe { &mut *this };
        if b != cloth.enable_continuous_collision {
            cloth.enable_continuous_collision = b;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_collision_mass_scale(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        unsafe { (*this).collision_mass_scale }
    }
}

unsafe extern "win64" fn cloth_set_collision_mass_scale(this: *mut NvCloth, scale: f32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if scale != cloth.collision_mass_scale {
            cloth.collision_mass_scale = scale;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_collision_mass_scale(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_set_friction(this: *mut NvCloth, friction: f32) {
    if !this.is_null() {
        unsafe { (*this).friction = friction };
    }
}
crate::win64_bridge!(fn cloth_set_friction(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_friction(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        unsafe { (*this).friction }
    }
}

unsafe extern "win64" fn cloth_set_virtual_particles(
    this: *mut NvCloth,
    indices_range: *const PxRange<[u32; 4]>,
    weights_range: *const PxRange<[f32; 3]>,
) {
    if this.is_null() {
        return;
    }
    let indices = unsafe { read_range_vec(indices_range) };
    let weights_raw = unsafe { read_range_vec(weights_range) };
    let cloth = unsafe { &mut *this };
    cloth.virtual_particle_indices = indices;
    cloth.virtual_particle_weights = weights_raw
        .into_iter()
        .map(|w| {
            let sq = w[0] * w[0] + w[1] * w[1] + w[2] * w[2];
            let inv_sq = if sq > EPS { 1.0 / sq } else { 0.0 };
            [w[0], w[1], w[2], inv_sq]
        })
        .collect();
}

unsafe extern "win64" fn cloth_get_num_virtual_particles(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).virtual_particle_indices.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_get_num_virtual_particle_weights(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).virtual_particle_weights.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_tether_constraint_scale(this: *mut NvCloth, scale: f32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if scale != cloth.tether_constraint_scale {
            cloth.tether_constraint_scale = scale;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_tether_constraint_scale(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_tether_constraint_scale(this: *const NvCloth) -> f32 {
    if this.is_null() {
        1.0
    } else {
        unsafe { (*this).tether_constraint_scale }
    }
}

unsafe extern "win64" fn cloth_set_tether_constraint_stiffness(
    this: *mut NvCloth,
    stiffness: f32,
) {
    if !this.is_null() {
        let val = safe_log2(1.0 - stiffness);
        let cloth = unsafe { &mut *this };
        if val != cloth.tether_constraint_log_stiffness {
            cloth.tether_constraint_log_stiffness = val;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_tether_constraint_stiffness(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_tether_constraint_stiffness(this: *const NvCloth) -> f32 {
    if this.is_null() {
        1.0
    } else {
        1.0 - safe_exp2(unsafe { (*this).tether_constraint_log_stiffness })
    }
}

unsafe extern "win64" fn cloth_get_motion_constraints(
    this: *mut NvCloth,
    sret_out: *mut PxRange<[f32; 4]>,
) -> *mut PxRange<[f32; 4]> {
    if !sret_out.is_null() {
        let (begin, end) = if this.is_null() {
            (std::ptr::null_mut(), std::ptr::null_mut())
        } else {
            let cloth = unsafe { &mut *this };
            cloth.wake_up();
            let n = cloth.current_particles.len();
            cloth.motion_constraints.push(n)
        };
        unsafe { sret_out.write_unaligned(PxRange { begin, end }) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_clear_motion_constraints(this: *mut NvCloth) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        cloth.motion_constraints.clear();
        cloth.wake_up();
    }
}

unsafe extern "win64" fn cloth_get_num_motion_constraints(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).motion_constraints.start.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_motion_constraint_scale_bias(
    this: *mut NvCloth,
    scale: f32,
    bias: f32,
) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if scale != cloth.motion_constraint_scale || bias != cloth.motion_constraint_bias {
            cloth.motion_constraint_scale = scale;
            cloth.motion_constraint_bias = bias;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_motion_constraint_scale_bias(*mut NvCloth, f32, f32) -> ());

unsafe extern "win64" fn cloth_get_motion_constraint_scale(this: *const NvCloth) -> f32 {
    if this.is_null() {
        1.0
    } else {
        unsafe { (*this).motion_constraint_scale }
    }
}

unsafe extern "win64" fn cloth_get_motion_constraint_bias(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        unsafe { (*this).motion_constraint_bias }
    }
}

unsafe extern "win64" fn cloth_set_motion_constraint_stiffness(
    this: *mut NvCloth,
    stiffness: f32,
) {
    if !this.is_null() {
        let val = safe_log2(1.0 - stiffness);
        let cloth = unsafe { &mut *this };
        if val != cloth.motion_constraint_log_stiffness {
            cloth.motion_constraint_log_stiffness = val;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_motion_constraint_stiffness(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_motion_constraint_stiffness(this: *const NvCloth) -> f32 {
    if this.is_null() {
        1.0
    } else {
        1.0 - safe_exp2(unsafe { (*this).motion_constraint_log_stiffness })
    }
}

unsafe extern "win64" fn cloth_get_separation_constraints(
    this: *mut NvCloth,
    sret_out: *mut PxRange<[f32; 4]>,
) -> *mut PxRange<[f32; 4]> {
    if !sret_out.is_null() {
        let (begin, end) = if this.is_null() {
            (std::ptr::null_mut(), std::ptr::null_mut())
        } else {
            let cloth = unsafe { &mut *this };
            cloth.wake_up();
            let n = cloth.current_particles.len();
            cloth.separation_constraints.push(n)
        };
        unsafe { sret_out.write_unaligned(PxRange { begin, end }) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_clear_separation_constraints(this: *mut NvCloth) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        cloth.separation_constraints.clear();
        cloth.wake_up();
    }
}

unsafe extern "win64" fn cloth_get_num_separation_constraints(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).separation_constraints.start.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_clear_interpolation(this: *mut NvCloth) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if !cloth.target_collision_spheres.is_empty() {
            std::mem::swap(
                &mut cloth.start_collision_spheres,
                &mut cloth.target_collision_spheres,
            );
            cloth.target_collision_spheres.clear();
        }
        cloth.motion_constraints.pop();
        cloth.separation_constraints.pop();
        cloth.wake_up();
    }
}

unsafe extern "win64" fn cloth_get_particle_accelerations(
    this: *mut NvCloth,
    sret_out: *mut PxRange<[f32; 4]>,
) -> *mut PxRange<[f32; 4]> {
    if !sret_out.is_null() {
        let (begin, end) = if this.is_null() {
            (std::ptr::null_mut(), std::ptr::null_mut())
        } else {
            let cloth = unsafe { &mut *this };
            let n = cloth.current_particles.len();
            if cloth.particle_accelerations.is_empty() && n > 0 {
                cloth.particle_accelerations.resize(n, [0.0; 4]);
            }
            cloth.wake_up();
            if cloth.particle_accelerations.is_empty() {
                (std::ptr::null_mut(), std::ptr::null_mut())
            } else {
                let ptr = cloth.particle_accelerations.as_mut_ptr();
                (ptr, unsafe { ptr.add(cloth.particle_accelerations.len()) })
            }
        };
        unsafe { sret_out.write_unaligned(PxRange { begin, end }) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_clear_particle_accelerations(this: *mut NvCloth) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        cloth.particle_accelerations.clear();
        cloth.wake_up();
    }
}

unsafe extern "win64" fn cloth_get_num_particle_accelerations(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).particle_accelerations.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_wind_velocity(this: *mut NvCloth, wind: *const [f32; 3]) {
    if !this.is_null() && !wind.is_null() {
        let w = unsafe { wind.read_unaligned() };
        let cloth = unsafe { &mut *this };
        if w != cloth.wind {
            cloth.wind = w;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_wind_velocity(
    this: *const NvCloth,
    sret_out: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret_out.is_null() {
        let v = if this.is_null() {
            [0.0; 3]
        } else {
            unsafe { (*this).wind }
        };
        unsafe { sret_out.write_unaligned(v) };
    }
    sret_out
}

unsafe extern "win64" fn cloth_set_drag_coefficient(this: *mut NvCloth, coeff: f32) {
    if !this.is_null() {
        let val = safe_log2(1.0 - coeff);
        let cloth = unsafe { &mut *this };
        if val != cloth.drag_log_coefficient {
            cloth.drag_log_coefficient = val;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_drag_coefficient(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_drag_coefficient(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        1.0 - safe_exp2(unsafe { (*this).drag_log_coefficient })
    }
}

unsafe extern "win64" fn cloth_set_lift_coefficient(this: *mut NvCloth, coeff: f32) {
    if !this.is_null() {
        let val = safe_log2(1.0 - coeff);
        let cloth = unsafe { &mut *this };
        if val != cloth.lift_log_coefficient {
            cloth.lift_log_coefficient = val;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_lift_coefficient(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_lift_coefficient(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        1.0 - safe_exp2(unsafe { (*this).lift_log_coefficient })
    }
}

unsafe extern "win64" fn cloth_set_fluid_density(this: *mut NvCloth, density: f32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if density != cloth.fluid_density {
            cloth.fluid_density = density;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_fluid_density(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_fluid_density(this: *const NvCloth) -> f32 {
    if this.is_null() {
        1.0
    } else {
        unsafe { (*this).fluid_density }
    }
}

unsafe extern "win64" fn cloth_set_self_collision_distance(this: *mut NvCloth, distance: f32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if distance != cloth.self_collision_distance {
            cloth.self_collision_distance = distance;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_self_collision_distance(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_self_collision_distance(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        unsafe { (*this).self_collision_distance }
    }
}

unsafe extern "win64" fn cloth_set_self_collision_stiffness(this: *mut NvCloth, stiffness: f32) {
    if !this.is_null() {
        let val = safe_log2(1.0 - stiffness);
        let cloth = unsafe { &mut *this };
        if val != cloth.self_collision_log_stiffness {
            cloth.self_collision_log_stiffness = val;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_self_collision_stiffness(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_self_collision_stiffness(this: *const NvCloth) -> f32 {
    if this.is_null() {
        1.0
    } else {
        1.0 - safe_exp2(unsafe { (*this).self_collision_log_stiffness })
    }
}

unsafe extern "win64" fn cloth_set_self_collision_indices(
    this: *mut NvCloth,
    indices: *const PxRange<u32>,
) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        cloth.self_collision_indices = unsafe { read_range_vec(indices) };
        cloth.wake_up();
    }
}

unsafe extern "win64" fn cloth_get_num_self_collision_indices(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).self_collision_indices.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_set_rest_positions(
    this: *mut NvCloth,
    rest: *const PxRange<[f32; 4]>,
) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        cloth.rest_positions = unsafe { read_range_vec(rest) };
        cloth.wake_up();
    }
}

unsafe extern "win64" fn cloth_get_num_rest_positions(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).rest_positions.len() as u32 }
    }
}

unsafe extern "win64" fn cloth_get_bbox_center(this: *const NvCloth) -> *const [f32; 3] {
    if this.is_null() {
        static ZERO: [f32; 3] = [0.0, 0.0, 0.0];
        &ZERO
    } else {
        unsafe { &(*this).bbox_center }
    }
}

unsafe extern "win64" fn cloth_get_bbox_scale(this: *const NvCloth) -> *const [f32; 3] {
    if this.is_null() {
        static ZERO: [f32; 3] = [0.0, 0.0, 0.0];
        &ZERO
    } else {
        unsafe { &(*this).bbox_scale }
    }
}

unsafe extern "win64" fn cloth_set_sleep_threshold(this: *mut NvCloth, threshold: f32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if threshold != cloth.sleep_threshold {
            cloth.sleep_threshold = threshold;
            cloth.wake_up();
        }
    }
}
crate::win64_bridge!(fn cloth_set_sleep_threshold(*mut NvCloth, f32) -> ());

unsafe extern "win64" fn cloth_get_sleep_threshold(this: *const NvCloth) -> f32 {
    if this.is_null() {
        0.0
    } else {
        unsafe { (*this).sleep_threshold }
    }
}

unsafe extern "win64" fn cloth_set_sleep_test_interval(this: *mut NvCloth, interval: u32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if interval != cloth.sleep_test_interval {
            cloth.sleep_test_interval = interval;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_sleep_test_interval(this: *const NvCloth) -> u32 {
    if this.is_null() {
        u32::MAX
    } else {
        unsafe { (*this).sleep_test_interval }
    }
}

unsafe extern "win64" fn cloth_set_sleep_after_count(this: *mut NvCloth, count: u32) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        if count != cloth.sleep_after_count {
            cloth.sleep_after_count = count;
            cloth.wake_up();
        }
    }
}

unsafe extern "win64" fn cloth_get_sleep_after_count(this: *const NvCloth) -> u32 {
    if this.is_null() {
        u32::MAX
    } else {
        unsafe { (*this).sleep_after_count }
    }
}

unsafe extern "win64" fn cloth_get_sleep_pass_count(this: *const NvCloth) -> u32 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).sleep_pass_counter }
    }
}

unsafe extern "win64" fn cloth_is_asleep(this: *const NvCloth) -> u8 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).is_sleeping() as u8 }
    }
}

unsafe extern "win64" fn cloth_put_to_sleep(this: *mut NvCloth) {
    if !this.is_null() {
        let cloth = unsafe { &mut *this };
        cloth.sleep_pass_counter = cloth.sleep_after_count;
    }
}

unsafe extern "win64" fn cloth_wake_up(this: *mut NvCloth) {
    if !this.is_null() {
        unsafe { (*this).wake_up() };
    }
}

unsafe extern "win64" fn cloth_set_user_data(this: *mut NvCloth, data: *mut u8) {
    if !this.is_null() {
        unsafe { (*this).user_data = data };
    }
}

unsafe extern "win64" fn cloth_get_user_data(this: *const NvCloth) -> *mut u8 {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (*this).user_data }
    }
}

unsafe extern "win64" fn cloth_is_sleeping(this: *const NvCloth) -> u8 {
    if this.is_null() {
        0
    } else {
        unsafe { (*this).is_sleeping() as u8 }
    }
}

// ---------------------------------------------------------------------------
// Exported DLL C Entry Points (auto-exported by build.rs)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn InitializeNvCloth(
    _allocator: *mut u8,
    _error_callback: *mut u8,
    _assert_handler: *mut u8,
    _profiler_callback: *mut u8,
    _auto_dll_id_check: i32,
) {
    ensure_vtables_registered();
    if !LOGGED_INIT.swap(true, Ordering::Relaxed) {
        log::info!("NvCloth_x64 (builtin): InitializeNvCloth (native CPU solver)");
    }
}

pub unsafe extern "win64" fn NvClothCreateFactoryCPU() -> *mut NvClothFactory {
    ensure_vtables_registered();
    Box::into_raw(Box::new(NvClothFactory {
        vtbl: &FACTORY_VTBL,
    }))
}

pub unsafe extern "win64" fn NvClothDestroyFactory(factory: *mut NvClothFactory) {
    if !factory.is_null() {
        unsafe {
            factory_dtor(factory, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_range<T>(slice: &mut [T]) -> PxRange<T> {
        let begin = slice.as_mut_ptr();
        let end = unsafe { begin.add(slice.len()) };
        PxRange { begin, end }
    }

    fn empty_range<T>() -> PxRange<T> {
        PxRange {
            begin: std::ptr::null_mut(),
            end: std::ptr::null_mut(),
        }
    }

    unsafe fn step_solver(solver: *mut NvClothSolver, dt: f32) {
        unsafe {
            if solver_begin_simulation(solver, dt) != 0 {
                let chunks = solver_get_simulation_chunk_count(solver);
                for i in 0..chunks {
                    solver_simulate_chunk(solver, i);
                }
                solver_end_simulation(solver);
            }
        }
    }

    unsafe fn destroy_fabric(fabric: *mut NvClothFabric) {
        if !fabric.is_null() {
            let prev = unsafe { (*fabric).ref_count.fetch_sub(1, Ordering::AcqRel) };
            if prev == 1 {
                unsafe { fabric_dtor(fabric, 1) };
            }
        }
    }

    #[test]
    fn abi_vtables_layouts_and_defaults() {
        assert_eq!(std::mem::size_of::<MappedRangePxVec4>(), 0x38);
        assert_eq!(std::mem::size_of::<PhaseConfig>(), 20);
        assert_eq!(std::mem::offset_of!(NvClothFabric, ref_count), 0x08);
        assert_eq!(FACTORY_VTBL.len(), 14);
        assert_eq!(FABRIC_VTBL.len(), 12);
        assert_eq!(SOLVER_VTBL.len(), 18);
        assert_eq!(CLOTH_VTBL.len(), 114);

        unsafe {
            InitializeNvCloth(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                2,
            );
            let factory = NvClothCreateFactoryCPU();
            assert!(!factory.is_null());
            assert_eq!(factory_get_platform(factory), 0);

            let mut phases = [0u32];
            let mut sets = [2u32];
            let mut rest = [0.5f32, 0.5];
            let mut stiff = [safe_log2(0.5), safe_log2(0.25)];
            let mut indices = [0u32, 1, 1, 2];
            let mut anchors = [0u32, 0, 0];
            let mut tlengths = [0.0f32, 0.5, 1.0];
            let mut tris = [0u32, 1, 2];

            let r_phases = mk_range(&mut phases);
            let r_sets = mk_range(&mut sets);
            let r_rest = mk_range(&mut rest);
            let r_stiff = mk_range(&mut stiff);
            let r_indices = mk_range(&mut indices);
            let r_anchors = mk_range(&mut anchors);
            let r_tlen = mk_range(&mut tlengths);
            let r_tris = mk_range(&mut tris);

            let fabric = factory_create_fabric(
                factory,
                3,
                &r_phases,
                &r_sets,
                &r_rest,
                &r_stiff,
                &r_indices,
                &r_anchors,
                &r_tlen,
                &r_tris,
            );
            assert!(!fabric.is_null());
            assert_eq!((*fabric).ref_count.load(Ordering::Acquire), 1);
            assert_eq!(fabric_get_factory(fabric), factory);
            assert_eq!(fabric_get_num_phases(fabric), 1);
            assert_eq!(fabric_get_num_restvalues(fabric), 2);
            assert_eq!(fabric_get_num_stiffness_values(fabric), 2);
            assert_eq!(fabric_get_num_sets(fabric), 1);
            assert_eq!(fabric_get_num_indices(fabric), 4);
            assert_eq!(fabric_get_num_particles(fabric), 3);
            assert_eq!(fabric_get_num_tethers(fabric), 3);
            assert_eq!(fabric_get_num_triangles(fabric), 1);

            fabric_scale_restvalues(fabric, 2.0);
            fabric_scale_tether_lengths(fabric, 1.5);
            assert!(((&(*fabric).restvalues)[0] - 1.0).abs() < 1e-6);
            assert!(((*fabric).tether_length_scale - 1.5).abs() < 1e-6);

            let mut init_p = [
                [0.0f32, 0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 1.0],
                [2.0, 0.0, 0.0, 1.0],
            ];
            let r_p = mk_range(&mut init_p);
            let cloth = factory_create_cloth(factory, &r_p, fabric);
            assert!(!cloth.is_null());
            assert_eq!((*fabric).ref_count.load(Ordering::Acquire), 2);
            assert_eq!(cloth_get_fabric(cloth), fabric);
            assert_eq!(cloth_get_factory(cloth), factory);
            assert_eq!(cloth_get_num_particles(cloth), 3);

            // Defaults check
            assert_eq!(cloth_get_solver_frequency(cloth), 300.0);
            assert_eq!(cloth_get_stiffness_frequency(cloth), 10.0);
            assert_eq!(cloth_get_acceleration_filter_width(cloth), 30);
            assert_eq!(cloth_get_fluid_density(cloth), 1.0);
            assert_eq!(cloth_get_tether_constraint_scale(cloth), 1.0);
            assert!((cloth_get_tether_constraint_stiffness(cloth) - 1.0).abs() < 1e-6);
            assert!((cloth_get_motion_constraint_stiffness(cloth) - 1.0).abs() < 1e-6);
            assert!((cloth_get_self_collision_stiffness(cloth) - 1.0).abs() < 1e-6);
            assert_eq!(cloth_get_sleep_test_interval(cloth), u32::MAX);
            assert_eq!(cloth_get_sleep_after_count(cloth), u32::MAX);
            assert_eq!(cloth_is_asleep(cloth), 0);
            assert_eq!(cloth_is_sleeping(cloth), 0);

            let mut v3 = [0.0f32; 3];
            assert_eq!(cloth_get_linear_inertia(cloth, &mut v3), &mut v3 as *mut _);
            assert_eq!(v3, [1.0, 1.0, 1.0]);
            cloth_get_angular_inertia(cloth, &mut v3);
            assert_eq!(v3, [1.0, 1.0, 1.0]);
            cloth_get_centrifugal_inertia(cloth, &mut v3);
            assert_eq!(v3, [1.0, 1.0, 1.0]);

            // MappedRange + GpuParticles + clone
            let mut mapped = std::mem::MaybeUninit::<MappedRangePxVec4>::uninit();
            let out = cloth_get_current_particles(cloth, mapped.as_mut_ptr());
            assert_eq!(out, mapped.as_mut_ptr());
            let mapped = mapped.assume_init();
            assert_eq!(mapped.end.offset_from(mapped.begin), 3);
            assert_eq!((*mapped.begin.add(1))[0], 1.0);
            assert_eq!(mapped.cloth, cloth);
            assert_ne!(mapped.unlock_fn, 0);

            let mut gpu = [1usize; 3];
            cloth_get_gpu_particles(cloth, &mut gpu);
            assert_eq!(gpu, [0, 0, 0]);

            let cloned = cloth_clone(cloth, factory);
            assert!(!cloned.is_null());
            assert_eq!((*fabric).ref_count.load(Ordering::Acquire), 3);

            cloth_dtor(cloned, 1);
            cloth_dtor(cloth, 1);
            assert_eq!((*fabric).ref_count.load(Ordering::Acquire), 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn gravity_pinned_strand_and_distance_constraints() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let n = 5usize;
            let seg = 0.25f32;
            let mut phases = [0u32];
            let mut sets = [4u32];
            let mut rest = [seg; 4];
            let mut indices = [0u32, 1, 1, 2, 2, 3, 3, 4];

            let r_phases = mk_range(&mut phases);
            let r_sets = mk_range(&mut sets);
            let r_rest = mk_range(&mut rest);
            let r_empty_f: PxRange<f32> = empty_range();
            let r_indices = mk_range(&mut indices);
            let r_empty_u: PxRange<u32> = empty_range();

            let fabric = factory_create_fabric(
                factory,
                n as u32,
                &r_phases,
                &r_sets,
                &r_rest,
                &r_empty_f,
                &r_indices,
                &r_empty_u,
                &r_empty_f,
                &r_empty_u,
            );

            let mut particles = [
                [0.0f32, 0.0, 0.0, 0.0], // pinned root
                [seg, 0.0, 0.0, 1.0],
                [2.0 * seg, 0.0, 0.0, 1.0],
                [3.0 * seg, 0.0, 0.0, 1.0],
                [4.0 * seg, 0.0, 0.0, 1.0],
            ];
            let r_p = mk_range(&mut particles);
            let cloth = factory_create_cloth(factory, &r_p, fabric);

            let mut cfgs = [PhaseConfig {
                stiffness: 1.0,
                stiffness_multiplier: 1.0,
                compression_limit: 1.0,
                stretch_limit: 1.0,
                phase_index: 0,
                padding: 0,
            }];
            let r_cfg = mk_range(&mut cfgs);
            cloth_set_phase_config(cloth, &r_cfg);
            let grav = [0.0f32, -9.81, 0.0];
            cloth_set_gravity(cloth, &grav);
            let damp = [0.2f32, 0.2, 0.2];
            cloth_set_damping(cloth, &damp);

            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);
            for _ in 0..60 {
                step_solver(solver, 1.0 / 60.0);
            }

            let cur = &(*cloth).current_particles;
            // Root particle remains pinned at origin
            assert_eq!(v4_xyz(cur[0]), [0.0, 0.0, 0.0]);
            // Free strand particles hang downward
            for i in 1..n {
                assert!(cur[i][1] < -0.05, "particle {i} did not fall: {:?}", cur[i]);
            }
            // Edge lengths hold close to rest length
            for e in 0..4 {
                let d = v3_sub(v4_xyz(cur[e + 1]), v4_xyz(cur[e]));
                let len = v3_dot(d, d).sqrt();
                assert!(
                    (len - seg).abs() < 0.04,
                    "edge {e} length {len} deviated from {seg}"
                );
            }
            // Bounding box reflects hanging strand
            let scale = *cloth_get_bbox_scale(cloth);
            assert!(scale[1] > 0.1);

            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn motion_and_separation_constraints() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let r_empty_u: PxRange<u32> = empty_range();
            let r_empty_f: PxRange<f32> = empty_range();
            let fabric = factory_create_fabric(
                factory,
                2,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_f,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_u,
            );
            let mut particles = [
                [0.0f32, 0.0, 0.0, 1.0],
                [0.1, 0.0, 0.0, 1.0],
            ];
            let r_p = mk_range(&mut particles);
            let cloth = factory_create_cloth(factory, &r_p, fabric);
            let grav = [0.0f32, -20.0, 0.0];
            cloth_set_gravity(cloth, &grav);
            cloth_set_motion_constraint_stiffness(cloth, 1.0);
            cloth_set_motion_constraint_scale_bias(cloth, 1.0, 0.0);

            // Push start motion constraints: p0 pinned with radius 0, p1 wide radius
            let mut mr = empty_range::<[f32; 4]>();
            cloth_get_motion_constraints(cloth, &mut mr);
            (*mr.begin.add(0)) = [0.0, 0.0, 0.0, 0.0]; // radius 0 -> pins cur[0].w = 0
            (*mr.begin.add(1)) = [0.0, 0.0, 0.0, 100.0];

            // Push target motion constraints (moves p0 anchor to x=1.0 across substeps)
            let mut mr_t = empty_range::<[f32; 4]>();
            cloth_get_motion_constraints(cloth, &mut mr_t);
            (*mr_t.begin.add(0)) = [1.0, 0.0, 0.0, 0.0];
            (*mr_t.begin.add(1)) = [0.0, 0.0, 0.0, 100.0];

            // Separation constraint pushing p1 outside sphere at origin radius 2.0
            let mut sr = empty_range::<[f32; 4]>();
            cloth_get_separation_constraints(cloth, &mut sr);
            (*sr.begin.add(0)) = [100.0, 0.0, 0.0, 0.1];
            (*sr.begin.add(1)) = [0.0, 0.0, 0.0, 2.0];

            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);
            step_solver(solver, 1.0 / 60.0);

            let cur = &(*cloth).current_particles;
            let prev = &(*cloth).previous_particles;
            // p0 snapped close to target [1, 0, 0] and has cur.w == 0 while prev.w == 1
            assert!((cur[0][0] - 1.0).abs() < 0.05, "p0 did not track motion target: {:?}", cur[0]);
            assert_eq!(cur[0][3], 0.0, "radius <= 0 must pin cur.w to 0");
            assert_eq!(prev[0][3], 1.0, "prev.w must preserve original invMass");

            // p1 pushed outside separation radius 2.0
            let d1 = v3_dot(v4_xyz(cur[1]), v4_xyz(cur[1])).sqrt();
            assert!(d1 >= 1.99, "p1 not separated to r>=2.0: d={d1}");

            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn tether_constraints_prevent_overstretch() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let r_empty_u: PxRange<u32> = empty_range();
            let r_empty_f: PxRange<f32> = empty_range();
            let mut anchors = [0u32, 0];
            let mut tlen = [0.0f32, 0.5];
            let r_anc = mk_range(&mut anchors);
            let r_tlen = mk_range(&mut tlen);

            let fabric = factory_create_fabric(
                factory,
                2,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_f,
                &r_empty_u,
                &r_anc,
                &r_tlen,
                &r_empty_u,
            );
            let mut particles = [
                [0.0f32, 0.0, 0.0, 0.0],
                [0.0, -0.5, 0.0, 1.0],
            ];
            let r_p = mk_range(&mut particles);
            let cloth = factory_create_cloth(factory, &r_p, fabric);
            let grav = [0.0f32, -50.0, 0.0];
            cloth_set_gravity(cloth, &grav);
            cloth_set_tether_constraint_stiffness(cloth, 1.0);
            cloth_set_tether_constraint_scale(cloth, 1.0);

            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);
            for _ in 0..30 {
                step_solver(solver, 1.0 / 60.0);
            }

            let cur = &(*cloth).current_particles;
            let dist = v3_dot(v4_xyz(cur[1]), v4_xyz(cur[1])).sqrt();
            assert!(dist < 0.6, "tether failed to hold particle: dist={dist}");

            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn sphere_capsule_collision_friction_ccd_and_virtual_particles() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let r_empty_u: PxRange<u32> = empty_range();
            let r_empty_f: PxRange<f32> = empty_range();
            let fabric = factory_create_fabric(
                factory,
                4,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_f,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_u,
            );

            // p0: tests tapered capsule wall; p1..p3: triangle with virtual particle at center
            let mut particles = [
                [0.0f32, 0.5, 0.0, 1.0],
                [-1.0, 0.5, 0.0, 1.0],
                [1.0, 0.5, 0.0, 1.0],
                [0.0, 0.5, 1.0, 1.0],
            ];
            let r_p = mk_range(&mut particles);
            let cloth = factory_create_cloth(factory, &r_p, fabric);
            let grav = [0.0f32, -9.81, 0.0];
            cloth_set_gravity(cloth, &grav);
            cloth_set_friction(cloth, 0.5);
            cloth_set_collision_mass_scale(cloth, 1.0);

            // Tapered capsule along X from (-2, 0, 0) r=0.6 to (2, 0, 0) r=0.8
            let mut spheres = [[-2.0f32, 0.0, 0.0, 0.6], [2.0, 0.0, 0.0, 0.8]];
            let r_sph = mk_range(&mut spheres);
            cloth_set_spheres_range(cloth, &r_sph, 0, 0);
            let mut caps = [0u32, 1];
            let r_cap = mk_range(&mut caps);
            cloth_set_capsules(cloth, &r_cap, 0, 0);

            // Virtual particle at centroid of p1, p2, p3
            let mut vp_idx = [[1u32, 2, 3, 0]];
            let mut vp_w = [[1.0f32 / 3.0, 1.0 / 3.0, 1.0 / 3.0]];
            let r_vpi = mk_range(&mut vp_idx);
            let r_vpw = mk_range(&mut vp_w);
            cloth_set_virtual_particles(cloth, &r_vpi, &r_vpw);
            assert_eq!(cloth_get_num_virtual_particles(cloth), 1);
            assert_eq!(cloth_get_num_virtual_particle_weights(cloth), 1);

            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);
            for _ in 0..10 {
                step_solver(solver, 1.0 / 60.0);
            }

            // p0 was at x=0, y=0.5 inside capsule of center radius ~0.7 -> pushed up above ~0.68
            let p0 = (&(*cloth).current_particles)[0];
            assert!(p0[1] >= 0.68, "capsule failed to push out p0: {p0:?}");

            // Dedicated virtual-particle test (zero gravity, triangle vertices outside sphere r=0.6
            // at origin, midpoint virtual particle at (0, 0.3, 0) inside sphere -> pushes vertices +Y)
            let mut vp_particles = [
                [-2.0f32, 0.3, 0.0, 1.0],
                [2.0, 0.3, 0.0, 1.0],
                [0.0, 0.3, -2.0, 1.0],
                [0.0, 0.3, 2.0, 1.0],
            ];
            let r_vp_p = mk_range(&mut vp_particles);
            let cloth_vp = factory_create_cloth(factory, &r_vp_p, fabric);
            let mut vp_sph = [[0.0f32, 0.0, 0.0, 0.6]];
            let r_vp_sph = mk_range(&mut vp_sph);
            cloth_set_spheres_range(cloth_vp, &r_vp_sph, 0, 0);
            let mut vp_i = [[0u32, 1, 2, 0]];
            let mut vp_w2 = [[0.5f32, 0.5, 0.0]]; // midpoint of (-2, 0.3, 0) and (2, 0.3, 0) = (0, 0.3, 0)
            let r_vpi2 = mk_range(&mut vp_i);
            let r_vpw2 = mk_range(&mut vp_w2);
            cloth_set_virtual_particles(cloth_vp, &r_vpi2, &r_vpw2);
            solver_add_cloth(solver, cloth_vp);
            step_solver(solver, 1.0 / 60.0);
            let vp_cur = &(*cloth_vp).current_particles;
            assert!(
                vp_cur[0][1] > 0.55 && vp_cur[1][1] > 0.55,
                "virtual particle failed to push triangle vertices: {:?}",
                vp_cur
            );

            // CCD test: fast particle sweeping [0, 1, 0] -> [0, -1, 0] straight through sphere at origin (r=0.5)
            let cloth_ccd = factory_create_cloth(factory, &r_p, fabric);
            cloth_set_solver_frequency(cloth_ccd, 60.0); // 1 iteration per step
            cloth_enable_continuous_collision(cloth_ccd, 1);
            assert_eq!(cloth_is_continuous_collision_enabled(cloth_ccd), 1);
            (&mut (*cloth_ccd).previous_particles)[0] = [0.0, 3.0, 0.0, 1.0];
            (&mut (*cloth_ccd).current_particles)[0] = [0.0, 1.0, 0.0, 1.0];
            let mut ccd_sph = [[0.0f32, 0.0, 0.0, 0.5]];
            let r_csph = mk_range(&mut ccd_sph);
            cloth_set_spheres_range(cloth_ccd, &r_csph, 0, 0);
            solver_add_cloth(solver, cloth_ccd);
            step_solver(solver, 1.0 / 60.0);
            // Integration moves [0, 1, 0] -> [0, -1, 0]; CCD intercepts on +Y side of sphere at y=0.5
            let ccd_y = (&(*cloth_ccd).current_particles)[0][1];
            assert!(ccd_y >= 0.45, "CCD did not intercept tunneling particle: y={ccd_y}");

            solver_remove_cloth(solver, cloth_ccd);
            solver_remove_cloth(solver, cloth_vp);
            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth_ccd, 1);
            cloth_dtor(cloth_vp, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn convex_plane_and_triangle_collision() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let r_empty_u: PxRange<u32> = empty_range();
            let r_empty_f: PxRange<f32> = empty_range();
            let fabric = factory_create_fabric(
                factory,
                1,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_f,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_u,
            );
            let mut p_convex = [[0.0f32, 0.2, 0.0, 1.0]];
            let mut p_tri = [[5.0f32, 0.2, 0.0, 1.0]];
            let r_pc = mk_range(&mut p_convex);
            let r_pt = mk_range(&mut p_tri);
            let cloth_convex = factory_create_cloth(factory, &r_pc, fabric);
            let cloth_tri = factory_create_cloth(factory, &r_pt, fabric);
            let grav = [0.0f32, -9.81, 0.0];
            cloth_set_gravity(cloth_convex, &grav);
            cloth_set_friction(cloth_convex, 0.3);
            cloth_set_gravity(cloth_tri, &grav);

            // Ground plane y >= 0 as convex 0 (normal +Y, d = 0)
            let mut planes = [[0.0f32, 1.0, 0.0, 0.0]];
            let r_pl = mk_range(&mut planes);
            cloth_set_planes_range(cloth_convex, &r_pl, 0, 0);
            let mut masks = [1u32];
            let r_m = mk_range(&mut masks);
            cloth_set_convexes(cloth_convex, &r_m, 0, 0);

            // Collision triangle under cloth_tri at y = 0.1 with +Y normal
            let mut tris = [
                [4.0f32, 0.1, -1.0],
                [4.0, 0.1, 1.0],
                [6.0, 0.1, 0.0],
            ];
            let r_t = mk_range(&mut tris);
            cloth_set_triangles_range(cloth_tri, &r_t, 0, 0);
            assert_eq!(cloth_get_num_triangles(cloth_tri), 1);

            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth_convex);
            solver_add_cloth(solver, cloth_tri);
            for _ in 0..30 {
                step_solver(solver, 1.0 / 60.0);
            }

            let y_convex = (&(*cloth_convex).current_particles)[0][1];
            let y_tri = (&(*cloth_tri).current_particles)[0][1];
            assert!(y_convex >= -1e-4 && y_convex < 0.05, "convex plane failed to hold p0 at y=0: {y_convex}");
            assert!(y_tri >= 0.09 && y_tri < 0.15, "collision triangle failed to hold p1 at y=0.1: {y_tri}");

            solver_remove_cloth(solver, cloth_tri);
            solver_remove_cloth(solver, cloth_convex);
            solver_dtor(solver, 1);
            cloth_dtor(cloth_tri, 1);
            cloth_dtor(cloth_convex, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn local_space_translation_turning_inertia_and_teleport() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let r_empty_u: PxRange<u32> = empty_range();
            let r_empty_f: PxRange<f32> = empty_range();
            let fabric = factory_create_fabric(
                factory,
                1,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_f,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_u,
            );
            let mut particles = [[1.0f32, 0.0, 0.0, 1.0]];
            let r_p = mk_range(&mut particles);
            let cloth = factory_create_cloth(factory, &r_p, fabric);
            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);

            // Linear acceleration along +X causes local particle to lag toward -X
            let t1 = [1.0f32, 0.0, 0.0];
            cloth_set_translation(cloth, &t1);
            step_solver(solver, 1.0 / 60.0);
            let x_after_accel = (&(*cloth).current_particles)[0][0];
            assert!(x_after_accel < 0.9, "linear inertia should lag local x: {x_after_accel}");

            // clearInertia + teleportToLocation moves frame without introducing velocity
            cloth_clear_inertia(cloth);
            let t2 = [10.0f32, 0.0, 0.0];
            let q_id = [0.0f32, 0.0, 0.0, 1.0];
            cloth_teleport_to_location(cloth, &t2, &q_id);
            (&mut (*cloth).current_particles)[0] = [1.0, 0.0, 0.0, 1.0];
            (&mut (*cloth).previous_particles)[0] = [1.0, 0.0, 0.0, 1.0];
            step_solver(solver, 1.0 / 60.0);
            assert!(((&(*cloth).current_particles)[0][0] - 1.0).abs() < 1e-5);

            // Turning frame around +Z engages turning integration matrix path
            let half_angle = 0.25f32;
            let q_rot = [0.0f32, 0.0, half_angle.sin(), half_angle.cos()];
            cloth_set_rotation(cloth, &q_rot);
            step_solver(solver, 1.0 / 60.0);
            let p_turn = (&(*cloth).current_particles)[0];
            assert!(p_turn[0].is_finite() && p_turn[1].is_finite() && p_turn[2].is_finite());
            assert!(p_turn[1].abs() > 1e-3, "turning inertia should deflect y: {p_turn:?}");

            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn wind_drag_and_lift_on_fabric_triangles() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let r_empty_u: PxRange<u32> = empty_range();
            let r_empty_f: PxRange<f32> = empty_range();
            let mut tris = [0u32, 1, 2, 1, 3, 2];
            let r_tris = mk_range(&mut tris);

            let fabric = factory_create_fabric(
                factory,
                4,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_f,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_tris,
            );
            let mut particles = [
                [0.0f32, 1.0, 0.0, 0.0], // top-left pinned
                [1.0, 1.0, 0.0, 0.0],    // top-right pinned
                [0.0, 0.0, 0.0, 1.0],    // bottom-left free
                [1.0, 0.0, 0.0, 1.0],    // bottom-right free
            ];
            let r_p = mk_range(&mut particles);
            let cloth = factory_create_cloth(factory, &r_p, fabric);
            let wind = [0.0f32, 0.0, 10.0];
            cloth_set_wind_velocity(cloth, &wind);
            cloth_set_drag_coefficient(cloth, 0.5);
            cloth_set_lift_coefficient(cloth, 0.25);
            cloth_set_fluid_density(cloth, 1.2);

            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);
            for _ in 0..10 {
                step_solver(solver, 1.0 / 60.0);
            }

            let cur = &(*cloth).current_particles;
            assert_eq!(cur[0][2], 0.0);
            assert_eq!(cur[1][2], 0.0);
            assert!(cur[2][2] > 0.05, "wind failed to push bottom-left: {:?}", cur[2]);
            assert!(cur[3][2] > 0.05, "wind failed to push bottom-right: {:?}", cur[3]);

            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn self_collision_and_rest_positions() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let r_empty_u: PxRange<u32> = empty_range();
            let r_empty_f: PxRange<f32> = empty_range();
            let fabric = factory_create_fabric(
                factory,
                5,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_f,
                &r_empty_u,
                &r_empty_u,
                &r_empty_f,
                &r_empty_u,
            );
            // (p0, p1) are close at rest (0.1 < 0.5) -> ignored by rest filter.
            // (p2, p3) are far at rest (2.0 >= 0.5) and placed on z=10 so p2's Verlet rebound
            // along -X does not travel across into (p0, p1), currently 0.1 apart -> pushed apart.
            // p4 is excluded from self_collision_indices -> stays untouched.
            let mut particles = [
                [0.0f32, 0.0, 0.0, 1.0],
                [0.0, 0.1, 0.0, 1.0],
                [2.0, 0.0, 10.0, 1.0],
                [2.1, 0.0, 10.0, 1.0],
                [2.05, 0.0, 10.0, 1.0],
            ];
            let r_p = mk_range(&mut particles);
            let cloth = factory_create_cloth(factory, &r_p, fabric);
            let mut rest = [
                [0.0f32, 0.0, 0.0, 0.0],
                [0.0, 0.1, 0.0, 0.0],
                [2.0, 0.0, 10.0, 0.0],
                [4.0, 0.0, 10.0, 0.0],
                [6.0, 0.0, 10.0, 0.0],
            ];
            let r_rest = mk_range(&mut rest);
            cloth_set_rest_positions(cloth, &r_rest);
            let mut sc_idx = [0u32, 1, 2, 3];
            let r_sci = mk_range(&mut sc_idx);
            cloth_set_self_collision_indices(cloth, &r_sci);
            cloth_set_self_collision_distance(cloth, 0.5);
            cloth_set_self_collision_stiffness(cloth, 1.0);

            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);
            for _ in 0..5 {
                step_solver(solver, 1.0 / 60.0);
            }

            let cur = &(*cloth).current_particles;
            let d23 = v3_sub(v4_xyz(cur[3]), v4_xyz(cur[2]));
            let len23 = v3_dot(d23, d23).sqrt();
            assert!(len23 > 0.45, "p2 and p3 should be pushed apart: len={len23}");
            assert!((cur[0][1] - 0.0).abs() < 1e-5 && (cur[1][1] - 0.1).abs() < 1e-5,
                "p0/p1 should respect rest-position filter: {:?} {:?}", cur[0], cur[1]);
            assert!((cur[4][0] - 2.05).abs() < 1e-5,
                "p4 excluded from selfCollisionIndices should not move: {:?}", cur[4]);

            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    unsafe extern "win64" fn allow_all_inter_filter(_a: *mut u8, _b: *mut u8) -> u8 {
        1
    }

    #[test]
    fn multi_cloth_inter_collision_sleep_and_extract_roundtrip() {
        #[derive(Clone, Copy)]
        struct SendSolver(*mut NvClothSolver);
        unsafe impl Send for SendSolver {}
        unsafe impl Sync for SendSolver {}

        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let mut phases = [0u32];
            let mut sets = [1u32];
            let mut rest = [1.0f32];
            let mut stiff = [safe_log2(0.5)];
            let mut indices = [0u32, 1];
            let mut anchors = [0u32, 0];
            let mut tlen = [0.0f32, 1.0];
            let mut tris = [0u32, 1, 0];

            let r_ph = mk_range(&mut phases);
            let r_st = mk_range(&mut sets);
            let r_rs = mk_range(&mut rest);
            let r_sf = mk_range(&mut stiff);
            let r_ix = mk_range(&mut indices);
            let r_an = mk_range(&mut anchors);
            let r_tl = mk_range(&mut tlen);
            let r_tr = mk_range(&mut tris);

            let fabric = factory_create_fabric(
                factory, 2, &r_ph, &r_st, &r_rs, &r_sf, &r_ix, &r_an, &r_tl, &r_tr,
            );

            let mut p_a = [[0.0f32, 0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0]];
            let mut p_b = [[0.1f32, 0.0, 0.0, 1.0], [1.1, 0.0, 0.0, 1.0]];
            let r_pa = mk_range(&mut p_a);
            let r_pb = mk_range(&mut p_b);
            let cloth_a = factory_create_cloth(factory, &r_pa, fabric);
            let cloth_b = factory_create_cloth(factory, &r_pb, fabric);

            cloth_set_user_data(cloth_a, 1usize as *mut u8);
            cloth_set_user_data(cloth_b, 2usize as *mut u8);
            assert_eq!(cloth_get_user_data(cloth_a), 1usize as *mut u8);

            let solver = factory_create_solver(factory);
            let mut cloth_list = [cloth_a, cloth_b];
            let r_cl = mk_range(&mut cloth_list);
            solver_add_cloths(solver, &r_cl);
            assert_eq!(solver_get_num_cloths(solver), 2);
            assert_eq!(solver_get_simulation_chunk_count(solver), 2);
            assert_eq!(solver_has_error(solver), 0);

            solver_set_inter_collision_distance(solver, 0.5);
            solver_set_inter_collision_stiffness(solver, 1.0);
            solver_set_inter_collision_nb_iterations(solver, 2);
            solver_set_inter_collision_filter(solver, allow_all_inter_filter as *const () as usize);

            // Concurrent simulateChunk across threads + endSimulation inter-collision
            assert_eq!(solver_begin_simulation(solver, 1.0 / 60.0), 1);
            let shared = SendSolver(solver);
            std::thread::scope(|s| {
                let s0 = shared;
                let s1 = shared;
                s.spawn(move || {
                    let s0 = s0;
                    unsafe { solver_simulate_chunk(s0.0, 0) }
                });
                s.spawn(move || {
                    let s1 = s1;
                    unsafe { solver_simulate_chunk(s1.0, 1) }
                });
            });
            solver_end_simulation(solver);

            let dist_ab = ((&(*cloth_b).current_particles)[0][0] - (&(*cloth_a).current_particles)[0][0]).abs();
            assert!(dist_ab > 0.4, "inter-collision should separate cloths: dist={dist_ab}");

            // Sleep state machine test
            cloth_set_sleep_threshold(cloth_a, 1.0);
            cloth_set_sleep_test_interval(cloth_a, 1);
            cloth_set_sleep_after_count(cloth_a, 2);
            assert_eq!(cloth_is_asleep(cloth_a), 0);
            cloth_put_to_sleep(cloth_a);
            assert_eq!(cloth_is_asleep(cloth_a), 1);
            assert_eq!(cloth_is_sleeping(cloth_a), 1);
            cloth_wake_up(cloth_a);
            assert_eq!(cloth_is_asleep(cloth_a), 0);

            // Extract* factory round-trip checks
            let mut out_ph = [0u32; 1];
            let mut out_st = [0u32; 1];
            let mut out_rs = [0.0f32; 1];
            let mut out_sf = [0.0f32; 1];
            let mut out_ix = [0u32; 2];
            let mut out_an = [0u32; 2];
            let mut out_tl = [0.0f32; 2];
            let mut out_tr = [0u32; 3];
            let (rp, rst, rrs, rsf, rix, ran, rtl, rtr) = (
                mk_range(&mut out_ph),
                mk_range(&mut out_st),
                mk_range(&mut out_rs),
                mk_range(&mut out_sf),
                mk_range(&mut out_ix),
                mk_range(&mut out_an),
                mk_range(&mut out_tl),
                mk_range(&mut out_tr),
            );
            factory_extract_fabric_data(factory, fabric, &rp, &rst, &rrs, &rsf, &rix, &ran, &rtl, &rtr);
            assert_eq!(out_ph, [0]);
            assert_eq!(out_st, [1]);
            assert_eq!(out_rs, [1.0]);
            assert_eq!(out_ix, [0, 1]);
            assert_eq!(out_tl, [0.0, 1.0]);
            assert_eq!(out_tr, [0, 1, 0]);

            let mut sph = [[1.0f32, 2.0, 3.0, 0.5], [4.0, 5.0, 6.0, 0.5]];
            let r_s = mk_range(&mut sph);
            cloth_set_spheres_range(cloth_a, &r_s, 0, 0);
            let mut cap = [0u32, 1];
            let r_c = mk_range(&mut cap);
            cloth_set_capsules(cloth_a, &r_c, 0, 0);
            let mut pl = [[0.0f32, 1.0, 0.0, 0.0]];
            let r_pl = mk_range(&mut pl);
            cloth_set_planes_range(cloth_a, &r_pl, 0, 0);
            let mut cx = [1u32];
            let r_cx = mk_range(&mut cx);
            cloth_set_convexes(cloth_a, &r_cx, 0, 0);
            let mut tr = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
            let r_ct = mk_range(&mut tr);
            cloth_set_triangles_range(cloth_a, &r_ct, 0, 0);

            let mut ex_s = [[0.0f32; 4]; 2];
            let mut ex_c = [0u32; 2];
            let mut ex_p = [[0.0f32; 4]; 1];
            let mut ex_x = [0u32; 1];
            let mut ex_t = [[0.0f32; 3]; 3];
            let (rs_o, rc_o, rp_o, rx_o, rt_o) = (
                mk_range(&mut ex_s),
                mk_range(&mut ex_c),
                mk_range(&mut ex_p),
                mk_range(&mut ex_x),
                mk_range(&mut ex_t),
            );
            factory_extract_collision_data(factory, cloth_a, &rs_o, &rc_o, &rp_o, &rx_o, &rt_o);
            assert_eq!(ex_s[0], [1.0, 2.0, 3.0, 0.5]);
            assert_eq!(ex_c, [0, 1]);
            assert_eq!(ex_p[0], [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(ex_x, [1]);
            assert_eq!(ex_t[1], [1.0, 0.0, 0.0]);

            // Motion, separation, accelerations, virtual particles, self-collision, rest positions
            let mut mr = empty_range::<[f32; 4]>();
            cloth_get_motion_constraints(cloth_a, &mut mr);
            (*mr.begin) = [7.0, 8.0, 9.0, 1.0];
            let mut ex_mc = [[0.0f32; 4]; 2];
            let r_ex_mc = mk_range(&mut ex_mc);
            factory_extract_motion_constraints(factory, cloth_a, &r_ex_mc);
            assert_eq!(ex_mc[0], [7.0, 8.0, 9.0, 1.0]);

            let mut sr = empty_range::<[f32; 4]>();
            cloth_get_separation_constraints(cloth_a, &mut sr);
            (*sr.begin) = [2.0, 3.0, 4.0, 0.5];
            let mut ex_sc = [[0.0f32; 4]; 2];
            let r_ex_sc = mk_range(&mut ex_sc);
            factory_extract_separation_constraints(factory, cloth_a, &r_ex_sc);
            assert_eq!(ex_sc[0], [2.0, 3.0, 4.0, 0.5]);

            let mut ar = empty_range::<[f32; 4]>();
            cloth_get_particle_accelerations(cloth_a, &mut ar);
            (*ar.begin) = [0.0, -5.0, 0.0, 0.0];
            let mut ex_ac = [[0.0f32; 4]; 2];
            let r_ex_ac = mk_range(&mut ex_ac);
            factory_extract_particle_accelerations(factory, cloth_a, &r_ex_ac);
            assert_eq!(ex_ac[0], [0.0, -5.0, 0.0, 0.0]);

            let mut vpi = [[0u32, 1, 0, 0]];
            let mut vpw = [[0.5f32, 0.5, 0.0]];
            let (r_vpi, r_vpw) = (mk_range(&mut vpi), mk_range(&mut vpw));
            cloth_set_virtual_particles(cloth_a, &r_vpi, &r_vpw);
            let mut ex_vpi = [[0u32; 4]; 1];
            let mut ex_vpw = [[0.0f32; 3]; 1];
            let (r_evpi, r_evpw) = (mk_range(&mut ex_vpi), mk_range(&mut ex_vpw));
            factory_extract_virtual_particles(factory, cloth_a, &r_evpi, &r_evpw);
            assert_eq!(ex_vpi[0], [0, 1, 0, 0]);
            assert_eq!(ex_vpw[0], [0.5, 0.5, 0.0]);

            let mut sci = [1u32, 0];
            let r_sci = mk_range(&mut sci);
            cloth_set_self_collision_indices(cloth_a, &r_sci);
            let mut ex_sci = [0u32; 2];
            let r_ex_sci = mk_range(&mut ex_sci);
            factory_extract_self_collision_indices(factory, cloth_a, &r_ex_sci);
            assert_eq!(ex_sci, [1, 0]);

            let mut rp_in = [[1.0f32, 0.0, 0.0, 0.0], [2.0, 0.0, 0.0, 0.0]];
            let r_rpin = mk_range(&mut rp_in);
            cloth_set_rest_positions(cloth_a, &r_rpin);
            let mut ex_rp = [[0.0f32; 4]; 2];
            let r_ex_rp = mk_range(&mut ex_rp);
            factory_extract_rest_positions(factory, cloth_a, &r_ex_rp);
            assert_eq!(ex_rp[1], [2.0, 0.0, 0.0, 0.0]);

            solver_remove_cloth(solver, cloth_a);
            solver_remove_cloth(solver, cloth_b);
            solver_dtor(solver, 1);
            cloth_dtor(cloth_a, 1);
            cloth_dtor(cloth_b, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn set_start_target_with_empty_target_preserves_start() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let mut phases = [0u32];
            let mut sets = [0u32];
            let mut rest: [f32; 0] = [];
            let mut stiff: [f32; 0] = [];
            let mut idx: [u32; 0] = [];
            let mut anc: [u32; 0] = [];
            let mut tlen: [f32; 0] = [];
            let mut tris: [u32; 0] = [];
            let (rp, rs, rr, rsf, ri, ra, rt, rtr) = (
                mk_range(&mut phases),
                mk_range(&mut sets),
                mk_range(&mut rest),
                mk_range(&mut stiff),
                mk_range(&mut idx),
                mk_range(&mut anc),
                mk_range(&mut tlen),
                mk_range(&mut tris),
            );
            let fabric = factory_create_fabric(factory, 1, &rp, &rs, &rr, &rsf, &ri, &ra, &rt, &rtr);
            let mut parts = [[0.0f32, 1.0, 0.0, 1.0]];
            let r_parts = mk_range(&mut parts);
            let cloth = factory_create_cloth(factory, &r_parts, fabric);

            let mut sph = [[0.0f32, 0.0, 0.0, 0.5], [1.0, 0.0, 0.0, 0.5]];
            let r_sph = mk_range(&mut sph);
            let r_empty_v4 = empty_range::<[f32; 4]>();
            cloth_set_spheres_start_target(cloth, &r_sph, &r_empty_v4);
            assert_eq!(cloth_get_num_spheres(cloth), 2);
            assert!((&*cloth).target_collision_spheres.is_empty());

            let mut planes = [[0.0f32, 1.0, 0.0, 0.0]];
            let r_planes = mk_range(&mut planes);
            cloth_set_planes_start_target(cloth, &r_planes, &r_empty_v4);
            assert_eq!(cloth_get_num_planes(cloth), 1);
            assert!((&*cloth).target_collision_planes.is_empty());

            let mut ctris = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
            let r_ctris = mk_range(&mut ctris);
            let r_empty_v3 = empty_range::<[f32; 3]>();
            cloth_set_triangles_start_target(cloth, &r_ctris, &r_empty_v3, 0);
            assert_eq!(cloth_get_num_triangles(cloth), 1);
            assert!((&*cloth).target_collision_triangles.is_empty());

            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }

    #[test]
    fn nan_collision_capsule_and_recovery_keep_particles_finite() {
        for ccd in [0u8, 1u8] {
            unsafe {
                let factory = NvClothCreateFactoryCPU();
                let mut phases = [0u32];
                let mut sets = [1u32];
                let mut rest = [0.5f32];
                let mut stiff: [f32; 0] = [];
                let mut idx = [0u32, 1];
                let mut anc: [u32; 0] = [];
                let mut tlen: [f32; 0] = [];
                let mut tris: [u32; 0] = [];
                let (rp, rs, rr, rsf, ri, ra, rt, rtr) = (
                    mk_range(&mut phases),
                    mk_range(&mut sets),
                    mk_range(&mut rest),
                    mk_range(&mut stiff),
                    mk_range(&mut idx),
                    mk_range(&mut anc),
                    mk_range(&mut tlen),
                    mk_range(&mut tris),
                );
                let fabric = factory_create_fabric(factory, 2, &rp, &rs, &rr, &rsf, &ri, &ra, &rt, &rtr);
                let mut parts = [[0.0f32, 0.5, 0.0, 2.0], [0.5, 0.5, 0.0, 2.0]];
                let r_parts = mk_range(&mut parts);
                let cloth = factory_create_cloth(factory, &r_parts, fabric);
                let solver = factory_create_solver(factory);
                solver_add_cloth(solver, cloth);

                let g = [0.0f32, -9.8, 0.0];
                cloth_set_gravity(cloth, &g);
                cloth_set_friction(cloth, 0.5);
                cloth_set_collision_mass_scale(cloth, 25.0);
                cloth_enable_continuous_collision(cloth, ccd);

                // Seed valid capsule + sphere, then transition sphere 0..2 to NaN (simulating an upstream NaN bone)
                let mut sph_valid = [
                    [0.0f32, 0.0, 0.0, 0.25],
                    [0.0, 0.4, 0.0, 0.25],
                    [0.5, 0.4, 0.0, 0.2],
                ];
                let r_v = mk_range(&mut sph_valid);
                cloth_set_spheres_range(cloth, &r_v, 0, 0);
                let mut cap = [0u32, 1];
                let r_cap = mk_range(&mut cap);
                cloth_set_capsules(cloth, &r_cap, 0, 0);

                // Frame 1: target capsule becomes NaN
                let mut sph_nan = [
                    [f32::NAN, f32::NAN, f32::NAN, 0.25],
                    [f32::NAN, f32::NAN, f32::NAN, 0.25],
                    [0.5, 0.4, 0.0, 0.2],
                ];
                let r_nan = mk_range(&mut sph_nan);
                cloth_set_spheres_range(cloth, &r_nan, 0, 3);
                solver_begin_simulation(solver, 1.0 / 30.0);
                solver_simulate_chunk(solver, 0);
                solver_end_simulation(solver);

                for (i, p) in (&*cloth).current_particles.iter().enumerate() {
                    assert!(
                        p.iter().all(|v| v.is_finite()),
                        "ccd={ccd} frame1 particle {i} became non-finite: {p:?}"
                    );
                }

                // Frame 2: start is NaN, target recovers to finite overlapping particle 0
                let mut sph_rec = [
                    [0.0f32, 0.35, 0.0, 0.25],
                    [0.0, 0.65, 0.0, 0.25],
                    [0.5, 0.4, 0.0, 0.2],
                ];
                let r_rec = mk_range(&mut sph_rec);
                cloth_set_spheres_range(cloth, &r_rec, 0, 3);
                solver_begin_simulation(solver, 1.0 / 30.0);
                solver_simulate_chunk(solver, 0);
                solver_end_simulation(solver);

                for (i, p) in (&*cloth).current_particles.iter().enumerate() {
                    assert!(
                        p.iter().all(|v| v.is_finite()),
                        "ccd={ccd} frame2 recovery particle {i} became non-finite: {p:?}"
                    );
                }
                for (i, p) in (&*cloth).previous_particles.iter().enumerate() {
                    assert!(
                        p.iter().all(|v| v.is_finite()),
                        "ccd={ccd} frame2 recovery prev particle {i} became non-finite: {p:?}"
                    );
                }

                solver_remove_cloth(solver, cloth);
                solver_dtor(solver, 1);
                cloth_dtor(cloth, 1);
                destroy_fabric(fabric);
                NvClothDestroyFactory(factory);
            }
        }
    }

    #[test]
    fn nan_motion_constraint_isolates_particle_and_recovers() {
        unsafe {
            let factory = NvClothCreateFactoryCPU();
            let mut phases = [0u32];
            let mut sets = [1u32];
            let mut rest = [1.0f32];
            let mut stiff: [f32; 0] = [];
            let mut idx = [0u32, 1];
            let mut anc: [u32; 0] = [];
            let mut tlen: [f32; 0] = [];
            let mut tris: [u32; 0] = [];
            let (rp, rs, rr, rsf, ri, ra, rt, rtr) = (
                mk_range(&mut phases),
                mk_range(&mut sets),
                mk_range(&mut rest),
                mk_range(&mut stiff),
                mk_range(&mut idx),
                mk_range(&mut anc),
                mk_range(&mut tlen),
                mk_range(&mut tris),
            );
            let fabric = factory_create_fabric(factory, 2, &rp, &rs, &rr, &rsf, &ri, &ra, &rt, &rtr);
            let mut parts = [[0.0f32, 1.0, 0.0, 1.0], [1.0, 1.0, 0.0, 1.0]];
            let r_parts = mk_range(&mut parts);
            let cloth = factory_create_cloth(factory, &r_parts, fabric);
            let solver = factory_create_solver(factory);
            solver_add_cloth(solver, cloth);

            let mut cfgs = [PhaseConfig {
                stiffness: 1.0,
                stiffness_multiplier: 1.0,
                compression_limit: 1.0,
                stretch_limit: 1.0,
                phase_index: 0,
                padding: 0,
            }];
            let r_cfgs = mk_range(&mut cfgs);
            cloth_set_phase_config(cloth, &r_cfgs);
            let g = [0.0f32, -9.8, 0.0];
            cloth_set_gravity(cloth, &g);

            // Particle 0 receives a NaN motion constraint; particle 1 remains finite and connected via fabric.
            let mut mr = empty_range::<[f32; 4]>();
            cloth_get_motion_constraints(cloth, &mut mr);
            (*mr.begin) = [f32::NAN, f32::NAN, f32::NAN, 0.05];
            (*mr.begin.add(1)) = [1.0, 1.0, 0.0, 0.05];

            solver_begin_simulation(solver, 1.0 / 30.0);
            solver_simulate_chunk(solver, 0);
            solver_end_simulation(solver);

            {
                let cur = &(&*cloth).current_particles;
                assert!(cur[0][0].is_nan());
                assert!(
                    cur[1].iter().all(|v| v.is_finite()),
                    "finite neighbor must not be poisoned by NaN particle: {:?}",
                    cur[1]
                );
            }

            // When particle 0's motion constraint recovers to finite, particle 0 snaps back cleanly.
            let mut mr2 = empty_range::<[f32; 4]>();
            cloth_get_motion_constraints(cloth, &mut mr2);
            (*mr2.begin) = [0.0, 1.0, 0.0, 0.05];
            (*mr2.begin.add(1)) = [1.0, 1.0, 0.0, 0.05];

            solver_begin_simulation(solver, 1.0 / 30.0);
            solver_simulate_chunk(solver, 0);
            solver_end_simulation(solver);

            {
                let cur = &(&*cloth).current_particles;
                let prev = &(&*cloth).previous_particles;
                assert!(
                    cur[0].iter().all(|v| v.is_finite()),
                    "particle 0 should recover cleanly when constraint becomes finite: {:?}",
                    cur[0]
                );
                assert!(
                    prev[0].iter().all(|v| v.is_finite()),
                    "particle 0 previous position should recover cleanly: {:?}",
                    prev[0]
                );
            }

            solver_remove_cloth(solver, cloth);
            solver_dtor(solver, 1);
            cloth_dtor(cloth, 1);
            destroy_fabric(fabric);
            NvClothDestroyFactory(factory);
        }
    }
}
