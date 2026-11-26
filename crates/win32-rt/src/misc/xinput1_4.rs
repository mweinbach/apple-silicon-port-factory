//! xinput1_4.dll (also `xinput1_3.dll` / `xinput9_1_0.dll`, see
//! `crate::canonical_dll`): XInput on GameController.framework.
//!
//! The game imports `XInputGetState` (ordinal 2) statically. Wwise's
//! `AkXInputRumbleSink` loads `XInput9_1_0.dll` / `XInput1_4.dll` /
//! `XInput1_3.dll` and resolves `XInputGetCapabilities` / `XInputSetState`
//! with GetProcAddress.
//!
//! Model: four user slots. The macOS backend (`crate::macos::gamepad`, main
//! thread) assigns GameController devices to slots and pushes state here
//! ([`backend_connect`] / [`backend_update`] / [`backend_disconnect`]).
//! XInput calls only read this table. They never touch Apple frameworks
//! from a Windows thread. Rumble goes the other way through
//! `crate::macos::gamepad::set_rumble` (queued to the main thread).
//!
//! Values follow XInput, not GameController:
//! * thumbsticks are -32768..32767 with +Y up,
//! * triggers are 0..255,
//! * no deadzone is applied (games apply their own),
//! * `dwPacketNumber` changes only when the state changes.

use std::ffi::c_void;
use std::sync::atomic::{fence, AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, Once};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (2, "XInputGetState"),
    (3, "XInputSetState"),
    (4, "XInputGetCapabilities"),
    (5, "XInputEnable"),
    (7, "XInputGetBatteryInformation"),
    (8, "XInputGetKeystroke"),
    (100, "XInputGetStateEx"),
];

const ERROR_SUCCESS: u32 = 0;
const ERROR_BAD_ARGUMENTS: u32 = 160;
const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;
const ERROR_EMPTY: u32 = 4306;

pub const XUSER_MAX_COUNT: usize = 4;

pub mod btn {
    pub const DPAD_UP: u16 = 0x0001;
    pub const DPAD_DOWN: u16 = 0x0002;
    pub const DPAD_LEFT: u16 = 0x0004;
    pub const DPAD_RIGHT: u16 = 0x0008;
    pub const START: u16 = 0x0010;
    pub const BACK: u16 = 0x0020;
    pub const LEFT_THUMB: u16 = 0x0040;
    pub const RIGHT_THUMB: u16 = 0x0080;
    pub const LEFT_SHOULDER: u16 = 0x0100;
    pub const RIGHT_SHOULDER: u16 = 0x0200;
    /// Only reported by `XInputGetStateEx` (ordinal 100).
    pub const GUIDE: u16 = 0x0400;
    pub const A: u16 = 0x1000;
    pub const B: u16 = 0x2000;
    pub const X: u16 = 0x4000;
    pub const Y: u16 = 0x8000;
}

/// `XINPUT_GAMEPAD` (12 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gamepad {
    pub buttons: u16,
    pub left_trigger: u8,
    pub right_trigger: u8,
    pub thumb_lx: i16,
    pub thumb_ly: i16,
    pub thumb_rx: i16,
    pub thumb_ry: i16,
}
const _: () = assert!(std::mem::size_of::<Gamepad>() == 12);

/// `XINPUT_STATE` (16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct State {
    pub packet: u32,
    pub gamepad: Gamepad,
}
const _: () = assert!(std::mem::size_of::<State>() == 16);

/// `XINPUT_VIBRATION`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vibration {
    pub left: u16,
    pub right: u16,
}

/// `XINPUT_CAPABILITIES` (20 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Capabilities {
    pub ty: u8,
    pub sub_type: u8,
    pub flags: u16,
    pub gamepad: Gamepad,
    pub vibration: Vibration,
}
const _: () = assert!(std::mem::size_of::<Capabilities>() == 20);

/// `XINPUT_BATTERY_INFORMATION`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BatteryInformation {
    pub ty: u8,
    pub level: u8,
}

/// Static facts about a connected device (from the backend).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceInfo {
    pub name: String,
    pub wireless: bool,
    pub rumble: bool,
    /// 0..1, `None` when the device reports no battery.
    pub battery: Option<f32>,
}

#[derive(Clone, Default)]
struct Slot {
    connected: bool,
    info: DeviceInfo,
    pad: Gamepad,
    packet: u32,
    vibration: (u16, u16),
}

#[derive(Clone, Default)]
struct VirtualPadState {
    connected: bool,
    pad: Gamepad,
    packet: u32,
    vibration: (u16, u16),
}

static SLOTS: Mutex<[Option<Slot>; XUSER_MAX_COUNT]> = Mutex::new([None, None, None, None]);
static VIRTUAL_PAD: Mutex<VirtualPadState> = Mutex::new(VirtualPadState {
    connected: false,
    pad: Gamepad {
        buttons: 0,
        left_trigger: 0,
        right_trigger: 0,
        thumb_lx: 0,
        thumb_ly: 0,
        thumb_rx: 0,
        thumb_ry: 0,
    },
    packet: 0,
    vibration: (0, 0),
});
static ENABLED: AtomicBool = AtomicBool::new(true);
static START_ONCE: Once = Once::new();

/// Bit position of `btn::GUIDE` (`0x0400`) inside packed `XInputFastSlot::w0`
/// (`packet: u32` in bits `0..32`, `buttons: u16` in bits `32..48`).
const GUIDE_W0_BIT: u64 = (btn::GUIDE as u64) << 32;

/// Lock-free seqlock snapshot for one XInput user slot, read directly by both
/// Rust `XInputGetState`/`XInputGetStateEx` and inline ARM64 JIT/AOT fast thunks.
#[repr(C, align(32))]
pub struct XInputFastSlot {
    /// Seqlock counter: odd while a writer is updating, even when quiescent.
    pub seq: AtomicU32,
    /// `0` = disconnected (`ERROR_DEVICE_NOT_CONNECTED`), `1` = connected (`ERROR_SUCCESS`).
    pub connected: AtomicU32,
    /// Low 8 bytes of `XINPUT_STATE` (with `btn::GUIDE` intact and `ENABLED` applied):
    /// `packet (0..32) | buttons (32..48) | left_trigger (48..56) | right_trigger (56..64)`.
    pub w0: AtomicU64,
    /// High 8 bytes of `XINPUT_STATE`:
    /// `thumb_lx (0..16) | thumb_ly (16..32) | thumb_rx (32..48) | thumb_ry (48..64)`.
    pub w1: AtomicU64,
    _pad: u64,
}
const _: () = assert!(std::mem::size_of::<XInputFastSlot>() == 32);
const _: () = assert!(std::mem::offset_of!(XInputFastSlot, seq) == 0);
const _: () = assert!(std::mem::offset_of!(XInputFastSlot, connected) == 4);
const _: () = assert!(std::mem::offset_of!(XInputFastSlot, w0) == 8);
const _: () = assert!(std::mem::offset_of!(XInputFastSlot, w1) == 16);

/// Process-wide lock-free XInput state table for inline ARM64 fast thunks.
#[repr(C, align(32))]
pub struct XInputFastTable {
    /// `0` = gamepad backend not yet started (must take slow path to run `ensure_started`),
    /// `1` = backend started and fast-path reads are valid.
    pub ready: AtomicU64,
    _pad: [u64; 3],
    pub slots: [XInputFastSlot; XUSER_MAX_COUNT],
}
const _: () = assert!(std::mem::offset_of!(XInputFastTable, ready) == 0);
const _: () = assert!(std::mem::offset_of!(XInputFastTable, slots) == 32);

impl XInputFastSlot {
    const fn new() -> Self {
        Self {
            seq: AtomicU32::new(0),
            connected: AtomicU32::new(0),
            w0: AtomicU64::new(0),
            w1: AtomicU64::new(0),
            _pad: 0,
        }
    }
}

static FAST_TABLE: XInputFastTable = XInputFastTable {
    ready: AtomicU64::new(0),
    _pad: [0; 3],
    slots: [
        XInputFastSlot::new(),
        XInputFastSlot::new(),
        XInputFastSlot::new(),
        XInputFastSlot::new(),
    ],
};

/// Address of the process-wide [`XInputFastTable`] for inlined ARM64 `XInputGetState` /
/// `XInputGetStateEx` thunks.
pub fn xinput_fast_table_ptr() -> usize {
    &FAST_TABLE as *const XInputFastTable as usize
}

#[inline]
fn pack_state_words(packet: u32, pad: Gamepad) -> (u64, u64) {
    let w0 = (packet as u64)
        | ((pad.buttons as u64) << 32)
        | ((pad.left_trigger as u64) << 48)
        | ((pad.right_trigger as u64) << 56);
    let w1 = ((pad.thumb_lx as u16) as u64)
        | (((pad.thumb_ly as u16) as u64) << 16)
        | (((pad.thumb_rx as u16) as u64) << 32)
        | (((pad.thumb_ry as u16) as u64) << 48);
    (w0, w1)
}

/// Recompute and publish `FAST_TABLE.slots[user]` under the `SLOTS` mutex (lock order:
/// `SLOTS` -> `VIRTUAL_PAD`, matching `slot_state` and `connected`).
fn publish_fast_slot(user: usize) {
    if user >= XUSER_MAX_COUNT {
        return;
    }
    let s = SLOTS.lock().unwrap();
    let phys = s[user].as_ref().filter(|sl| sl.connected).map(|sl| (sl.pad, sl.packet));
    let effective = if user == 0 {
        let v = VIRTUAL_PAD.lock().unwrap();
        match (phys, v.connected) {
            (Some((p, pkt)), true) => {
                let merged = if v.pad != Gamepad::default() { merge_gamepad(p, v.pad) } else { p };
                Some((merged, pkt.wrapping_add(v.packet)))
            }
            (Some(st), false) => Some(st),
            (None, true) => Some((v.pad, v.packet)),
            (None, false) => None,
        }
    } else {
        phys
    };
    let (conn, w0, w1) = match effective {
        Some((raw_pad, pkt)) => {
            let pad = if ENABLED.load(Ordering::Acquire) { raw_pad } else { Gamepad::default() };
            let (w0, w1) = pack_state_words(pkt, pad);
            (1u32, w0, w1)
        }
        None => (0u32, 0u64, 0u64),
    };
    let dst = &FAST_TABLE.slots[user];
    let seq = dst.seq.load(Ordering::Relaxed);
    dst.seq.store(seq.wrapping_add(1), Ordering::Relaxed);
    fence(Ordering::AcqRel);
    dst.connected.store(conn, Ordering::Relaxed);
    dst.w0.store(w0, Ordering::Relaxed);
    dst.w1.store(w1, Ordering::Relaxed);
    dst.seq.store(seq.wrapping_add(2), Ordering::Release);
    drop(s);
}

fn with_slot<R>(user: usize, f: impl FnOnce(&mut Slot) -> R) -> R {
    let mut s = SLOTS.lock().unwrap();
    f(s[user].get_or_insert_with(Slot::default))
}

// ---------------------------------------------------------------------------
// Backend interface (main thread or tests)
// ---------------------------------------------------------------------------

pub fn backend_connect(user: usize, info: DeviceInfo) {
    log::info!("xinput: user {user} connected: {} (wireless={}, rumble={})", info.name, info.wireless, info.rumble);
    let phys_rumble = info.rumble;
    with_slot(user, |s| {
        *s = Slot { connected: true, info, packet: s.packet.wrapping_add(1), ..Default::default() };
    });
    // If a physical controller with its own rumble takes over slot 0, stop any active phone vibration.
    if user == 0 && phys_rumble {
        let stop_phone = {
            let mut v = VIRTUAL_PAD.lock().unwrap();
            let had = v.vibration != (0, 0);
            v.vibration = (0, 0);
            had
        };
        if stop_phone {
            crate::macos::uikit::dispatch_host_rumble(0, 0);
        }
    }
    publish_fast_slot(user);
    crate::user32::misc::notify_device_change(true);
}

pub fn backend_disconnect(user: usize) {
    log::info!("xinput: user {user} disconnected");
    let was_connected = with_slot(user, |s| {
        let was = s.connected;
        s.connected = false;
        s.pad = Gamepad::default();
        s.vibration = (0, 0);
        was
    });
    publish_fast_slot(user);
    if was_connected {
        crate::user32::misc::notify_device_change(false);
    }
}

/// New input state. `dwPacketNumber` only changes when the state did.
pub fn backend_update(user: usize, pad: Gamepad) {
    let changed = with_slot(user, |s| {
        if s.connected && s.pad != pad {
            s.pad = pad;
            s.packet = s.packet.wrapping_add(1);
            true
        } else {
            false
        }
    });
    if changed {
        publish_fast_slot(user);
    }
}

pub fn backend_battery(user: usize, level: Option<f32>) {
    with_slot(user, |s| s.info.battery = level);
}

/// Read back for the backend, DirectInput 8 bridge, and tests.
pub fn slot_state(user: usize) -> Option<(Gamepad, u32)> {
    let phys = {
        let s = SLOTS.lock().unwrap();
        s[user].as_ref().filter(|s| s.connected).map(|s| (s.pad, s.packet))
    };
    if user == 0 {
        let v = VIRTUAL_PAD.lock().unwrap();
        return match (phys, v.connected) {
            (Some((p, pkt)), true) => {
                let merged = if v.pad != Gamepad::default() { merge_gamepad(p, v.pad) } else { p };
                Some((merged, pkt.wrapping_add(v.packet)))
            }
            (Some(st), false) => Some(st),
            (None, true) => Some((v.pad, v.packet)),
            (None, false) => None,
        };
    }
    phys
}

/// Update the virtual touch-overlay gamepad (used by the iOS host overlay).
/// Exposed on XInput user slot 0 when no physical controller occupies slot 0,
/// and seamlessly merged onto slot 0 whenever virtual touch controls are active.
pub fn set_virtual_gamepad(connected: bool, pad: Gamepad) {
    let (conn_changed, stop_phone) = {
        let mut v = VIRTUAL_PAD.lock().unwrap();
        let conn_changed = v.connected != connected;
        let changed = conn_changed || (connected && v.pad != pad);
        if !v.connected && connected {
            log::info!("xinput: virtual touch-overlay gamepad connected (slot 0 fallback/overlay)");
        } else if v.connected && !connected {
            log::info!("xinput: virtual touch-overlay gamepad disconnected");
        }
        v.connected = connected;
        v.pad = if connected { pad } else { Gamepad::default() };
        if changed {
            v.packet = v.packet.wrapping_add(1);
        }
        let stop_phone = if !connected && v.vibration != (0, 0) {
            v.vibration = (0, 0);
            true
        } else {
            false
        };
        (conn_changed, stop_phone)
    };
    publish_fast_slot(0);
    if stop_phone {
        crate::macos::uikit::dispatch_host_rumble(0, 0);
    }
    if conn_changed {
        crate::user32::misc::notify_device_change(connected);
    }
}

/// Read back the current virtual touch-overlay gamepad state (diagnostics & tests).
pub fn virtual_gamepad_state() -> Option<(Gamepad, u32)> {
    let v = VIRTUAL_PAD.lock().unwrap();
    v.connected.then_some((v.pad, v.packet))
}

fn merge_gamepad(base: Gamepad, overlay: Gamepad) -> Gamepad {
    let pick_axis = |a: i16, b: i16| if b != 0 { b } else { a };
    Gamepad {
        buttons: base.buttons | overlay.buttons,
        left_trigger: base.left_trigger.max(overlay.left_trigger),
        right_trigger: base.right_trigger.max(overlay.right_trigger),
        thumb_lx: pick_axis(base.thumb_lx, overlay.thumb_lx),
        thumb_ly: pick_axis(base.thumb_ly, overlay.thumb_ly),
        thumb_rx: pick_axis(base.thumb_rx, overlay.thumb_rx),
        thumb_ry: pick_axis(base.thumb_ry, overlay.thumb_ry),
    }
}

// ---------------------------------------------------------------------------
// GameController -> XInput value conversion (pure, unit-tested)
// ---------------------------------------------------------------------------

/// GameController axis (-1..1, +Y up) -> XInput thumb value.
pub fn axis(v: f32) -> i16 {
    let v = if v.is_finite() { v.clamp(-1.0, 1.0) } else { 0.0 };
    if v < 0.0 {
        (v * 32768.0).round().max(-32768.0) as i16
    } else {
        (v * 32767.0).round() as i16
    }
}

/// GameController analog button (0..1) -> XInput trigger value.
pub fn trigger(v: f32) -> u8 {
    let v = if v.is_finite() { v.clamp(0.0, 1.0) } else { 0.0 };
    (v * 255.0).round() as u8
}

/// Snapshot of an extended gamepad, in GameController terms.
#[derive(Clone, Copy, Debug, Default)]
pub struct GcPad {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub left_shoulder: bool,
    pub right_shoulder: bool,
    pub left_thumb: bool,
    pub right_thumb: bool,
    /// `buttonMenu` (Xbox ☰ / DualSense Options).
    pub menu: bool,
    /// `buttonOptions` (Xbox ⧉ View / DualSense Create).
    pub options: bool,
    pub home: bool,
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    pub left_trigger: f32,
    pub right_trigger: f32,
    pub lx: f32,
    pub ly: f32,
    pub rx: f32,
    pub ry: f32,
}

impl GcPad {
    /// Map to XInput positionally, by the face-button positions of an Xbox
    /// pad: DualSense Cross is A, Circle is B, Square is X and Triangle is Y.
    pub fn to_xinput(&self) -> Gamepad {
        let mut b = 0u16;
        for (on, bit) in [
            (self.dpad_up, btn::DPAD_UP),
            (self.dpad_down, btn::DPAD_DOWN),
            (self.dpad_left, btn::DPAD_LEFT),
            (self.dpad_right, btn::DPAD_RIGHT),
            (self.menu, btn::START),
            (self.options, btn::BACK),
            (self.left_thumb, btn::LEFT_THUMB),
            (self.right_thumb, btn::RIGHT_THUMB),
            (self.left_shoulder, btn::LEFT_SHOULDER),
            (self.right_shoulder, btn::RIGHT_SHOULDER),
            (self.home, btn::GUIDE),
            (self.a, btn::A),
            (self.b, btn::B),
            (self.x, btn::X),
            (self.y, btn::Y),
        ] {
            if on {
                b |= bit;
            }
        }
        Gamepad {
            buttons: b,
            left_trigger: trigger(self.left_trigger),
            right_trigger: trigger(self.right_trigger),
            thumb_lx: axis(self.lx),
            thumb_ly: axis(self.ly),
            thumb_rx: axis(self.rx),
            thumb_ry: axis(self.ry),
        }
    }
}

// ---------------------------------------------------------------------------
// Exports
// ---------------------------------------------------------------------------

fn ensure_started() {
    if FAST_TABLE.ready.load(Ordering::Acquire) != 0 {
        return;
    }
    START_ONCE.call_once(|| {
        crate::macos::gamepad::start();
        FAST_TABLE.ready.store(1, Ordering::Release);
    });
}

/// Validate `user` and return the slot if connected.
fn connected(user: u32) -> Result<Slot, u32> {
    if user as usize >= XUSER_MAX_COUNT {
        return Err(ERROR_BAD_ARGUMENTS);
    }
    ensure_started();
    let phys = {
        let s = SLOTS.lock().unwrap();
        s[user as usize].as_ref().filter(|sl| sl.connected).cloned()
    };
    if user == 0 {
        let host_rumble = crate::macos::uikit::has_rumble_callback();
        let v = VIRTUAL_PAD.lock().unwrap().clone();
        match (phys, v.connected) {
            (Some(mut sl), true) => {
                if v.pad != Gamepad::default() {
                    sl.pad = merge_gamepad(sl.pad, v.pad);
                }
                sl.packet = sl.packet.wrapping_add(v.packet);
                if !sl.info.rumble && host_rumble {
                    sl.info.rumble = true;
                    sl.vibration = v.vibration;
                }
                return Ok(sl);
            }
            (Some(mut sl), false) => {
                if !sl.info.rumble && host_rumble {
                    sl.info.rumble = true;
                    sl.vibration = v.vibration;
                }
                return Ok(sl);
            }
            (None, true) => {
                return Ok(Slot {
                    connected: true,
                    info: DeviceInfo {
                        name: "iOS Touch Overlay".into(),
                        wireless: false,
                        rumble: host_rumble,
                        battery: None,
                    },
                    pad: v.pad,
                    packet: v.packet,
                    vibration: v.vibration,
                });
            }
            (None, false) => return Err(ERROR_DEVICE_NOT_CONNECTED),
        }
    }
    phys.ok_or(ERROR_DEVICE_NOT_CONNECTED)
}

#[inline(always)]
unsafe fn get_state(user: u32, out: *mut State, guide: bool) -> u32 {
    if out.is_null() || user as usize >= XUSER_MAX_COUNT {
        return ERROR_BAD_ARGUMENTS;
    }
    if FAST_TABLE.ready.load(Ordering::Acquire) == 0 {
        ensure_started();
    }
    let slot = &FAST_TABLE.slots[user as usize];
    loop {
        let seq1 = slot.seq.load(Ordering::Acquire);
        if seq1 & 1 != 0 {
            std::hint::spin_loop();
            continue;
        }
        let conn = slot.connected.load(Ordering::Relaxed);
        let mut w0 = slot.w0.load(Ordering::Relaxed);
        let w1 = slot.w1.load(Ordering::Relaxed);
        fence(Ordering::Acquire);
        if slot.seq.load(Ordering::Relaxed) != seq1 {
            std::hint::spin_loop();
            continue;
        }
        if conn == 0 {
            return ERROR_DEVICE_NOT_CONNECTED;
        }
        if !guide {
            w0 &= !GUIDE_W0_BIT;
        }
        let dst = out as *mut u8;
        std::ptr::write_unaligned(dst as *mut u64, w0);
        std::ptr::write_unaligned(dst.add(8) as *mut u64, w1);
        return ERROR_SUCCESS;
    }
}

#[inline(never)]
pub unsafe extern "win64" fn XInputGetState(user: u32, s: *mut State) -> u32 {
    get_state(user, s, false)
}

#[inline(never)]
pub unsafe extern "win64" fn XInputGetStateEx(user: u32, s: *mut State) -> u32 {
    get_state(user, s, true)
}

pub unsafe extern "win64" fn XInputSetState(user: u32, v: *mut Vibration) -> u32 {
    if v.is_null() {
        return ERROR_BAD_ARGUMENTS;
    }
    let s = match connected(user) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let v = v.read_unaligned();
    let want = if ENABLED.load(Ordering::Acquire) { (v.left, v.right) } else { (0, 0) };
    let phys_rumble = SLOTS.lock().unwrap()[user as usize]
        .as_ref()
        .is_some_and(|sl| sl.connected && sl.info.rumble);
    if phys_rumble {
        if want != s.vibration {
            with_slot(user as usize, |sl| sl.vibration = want);
            crate::macos::gamepad::set_rumble(user as usize, want.0, want.1);
        }
    } else if user == 0 && crate::macos::uikit::has_rumble_callback() {
        let changed = {
            let mut vp = VIRTUAL_PAD.lock().unwrap();
            let c = vp.vibration != want;
            vp.vibration = want;
            c
        };
        if changed {
            crate::macos::uikit::dispatch_host_rumble(want.0, want.1);
        }
    } else if want != s.vibration {
        with_slot(user as usize, |sl| sl.vibration = want);
    }
    ERROR_SUCCESS
}

const XINPUT_DEVTYPE_GAMEPAD: u8 = 1;
const XINPUT_DEVSUBTYPE_GAMEPAD: u8 = 1;
const XINPUT_CAPS_FFB_SUPPORTED: u16 = 0x0001;
const XINPUT_CAPS_WIRELESS: u16 = 0x0002;
const XINPUT_FLAG_GAMEPAD: u32 = 1;

pub unsafe extern "win64" fn XInputGetCapabilities(user: u32, flags: u32, c: *mut Capabilities) -> u32 {
    if c.is_null() || flags & !XINPUT_FLAG_GAMEPAD != 0 {
        return ERROR_BAD_ARGUMENTS;
    }
    let s = match connected(user) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut caps = Capabilities {
        ty: XINPUT_DEVTYPE_GAMEPAD,
        sub_type: XINPUT_DEVSUBTYPE_GAMEPAD,
        flags: 0,
        // Every control present at full resolution (as the Xbox driver does).
        gamepad: Gamepad {
            buttons: 0xF3FF,
            left_trigger: 0xFF,
            right_trigger: 0xFF,
            thumb_lx: -64,
            thumb_ly: -64,
            thumb_rx: -64,
            thumb_ry: -64,
        },
        vibration: Vibration::default(),
    };
    if s.info.wireless {
        caps.flags |= XINPUT_CAPS_WIRELESS;
    }
    if s.info.rumble {
        caps.flags |= XINPUT_CAPS_FFB_SUPPORTED;
        caps.vibration = Vibration { left: 0xFFFF, right: 0xFFFF };
    }
    c.write_unaligned(caps);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn XInputEnable(enable: i32) {
    ENABLED.store(enable != 0, Ordering::Release);
    for u in 0..XUSER_MAX_COUNT {
        publish_fast_slot(u);
    }
    if enable == 0 {
        // Windows stops vibration while disabled.
        for u in 0..XUSER_MAX_COUNT {
            let had = SLOTS.lock().unwrap()[u].as_ref().is_some_and(|s| s.connected && s.info.rumble && s.vibration != (0, 0));
            if had {
                with_slot(u, |s| s.vibration = (0, 0));
                crate::macos::gamepad::set_rumble(u, 0, 0);
            }
        }
        let had_phone = {
            let mut vp = VIRTUAL_PAD.lock().unwrap();
            let had = vp.vibration != (0, 0);
            vp.vibration = (0, 0);
            had
        };
        if had_phone {
            crate::macos::uikit::dispatch_host_rumble(0, 0);
        }
    }
}

const BATTERY_DEVTYPE_GAMEPAD: u8 = 0;
const BATTERY_TYPE_WIRED: u8 = 1;
const BATTERY_TYPE_NIMH: u8 = 3;
const BATTERY_TYPE_UNKNOWN: u8 = 0xFF;

pub fn battery_level(v: f32) -> u8 {
    match v {
        v if v < 0.05 => 0, // EMPTY
        v if v < 0.40 => 1, // LOW
        v if v < 0.70 => 2, // MEDIUM
        _ => 3,             // FULL
    }
}

pub unsafe extern "win64" fn XInputGetBatteryInformation(user: u32, dev_type: u8, b: *mut BatteryInformation) -> u32 {
    if b.is_null() {
        return ERROR_BAD_ARGUMENTS;
    }
    let s = match connected(user) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let info = if dev_type != BATTERY_DEVTYPE_GAMEPAD {
        BatteryInformation { ty: 0, level: 0 } // BATTERY_TYPE_DISCONNECTED (no headset)
    } else {
        match s.info.battery {
            Some(l) => BatteryInformation { ty: BATTERY_TYPE_NIMH, level: battery_level(l) },
            None if !s.info.wireless => BatteryInformation { ty: BATTERY_TYPE_WIRED, level: 3 },
            None => BatteryInformation { ty: BATTERY_TYPE_UNKNOWN, level: 3 },
        }
    };
    b.write_unaligned(info);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn XInputGetKeystroke(user: u32, _r: u32, _k: *mut c_void) -> u32 {
    if user as usize >= XUSER_MAX_COUNT && user != 0xFF {
        return ERROR_BAD_ARGUMENTS;
    }
    ensure_started();
    let any_phys = SLOTS.lock().unwrap().iter().enumerate().any(|(i, s)| (user == 0xFF || i == user as usize) && s.as_ref().is_some_and(|s| s.connected));
    let any_virt = (user == 0xFF || user == 0) && VIRTUAL_PAD.lock().unwrap().connected;
    if !any_phys && !any_virt {
        return ERROR_DEVICE_NOT_CONNECTED;
    }
    stub!("XInputGetKeystroke: keystroke events not generated (ERROR_EMPTY)");
    ERROR_EMPTY
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Slots are global: run these one at a time.
    static SERIAL: Mutex<()> = Mutex::new(());

    #[test]
    fn conversions() {
        assert_eq!(axis(0.0), 0);
        assert_eq!(axis(1.0), 32767);
        assert_eq!(axis(-1.0), -32768);
        assert_eq!(axis(0.5), 16384);
        assert_eq!(axis(2.0), 32767);
        assert_eq!(axis(f32::NAN), 0);
        assert_eq!(trigger(0.0), 0);
        assert_eq!(trigger(1.0), 255);
        assert_eq!(trigger(0.5), 128);
        assert_eq!(trigger(-0.1), 0);
        let p = GcPad { a: true, y: true, menu: true, options: true, home: true, dpad_left: true, left_shoulder: true, right_thumb: true, ly: 1.0, rx: -1.0, right_trigger: 1.0, ..Default::default() };
        let x = p.to_xinput();
        assert_eq!(x.buttons, btn::A | btn::Y | btn::START | btn::BACK | btn::GUIDE | btn::DPAD_LEFT | btn::LEFT_SHOULDER | btn::RIGHT_THUMB);
        assert_eq!((x.thumb_ly, x.thumb_rx, x.right_trigger, x.left_trigger), (32767, -32768, 255, 0));
        assert_eq!(battery_level(0.02), 0);
        assert_eq!(battery_level(0.5), 2);
        assert_eq!(battery_level(1.0), 3);
    }

    #[test]
    fn api_contract() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let mut st = State::default();
            assert_eq!(XInputGetState(4, &mut st), ERROR_BAD_ARGUMENTS);
            assert_eq!(XInputGetState(3, &mut st), ERROR_DEVICE_NOT_CONNECTED);
            backend_connect(3, DeviceInfo { name: "test".into(), wireless: true, rumble: true, battery: Some(0.3) });
            assert_eq!(XInputGetState(3, &mut st), ERROR_SUCCESS);
            let p0 = st.packet;
            assert_eq!(st.gamepad, Gamepad::default());
            // Same state: packet unchanged.
            backend_update(3, Gamepad::default());
            XInputGetState(3, &mut st);
            assert_eq!(st.packet, p0);
            let pad = GcPad { a: true, home: true, lx: -0.5, left_trigger: 0.25, ..Default::default() }.to_xinput();
            backend_update(3, pad);
            XInputGetState(3, &mut st);
            assert_ne!(st.packet, p0);
            assert_eq!(st.gamepad.buttons, btn::A, "guide hidden from XInputGetState");
            assert_eq!((st.gamepad.thumb_lx, st.gamepad.left_trigger), (-16384, 64));
            XInputGetStateEx(3, &mut st);
            assert_eq!(st.gamepad.buttons, btn::A | btn::GUIDE);
            // Disabled: neutral state, still connected.
            XInputEnable(0);
            XInputGetState(3, &mut st);
            assert_eq!(st.gamepad, Gamepad::default());
            XInputEnable(1);
            let mut caps = Capabilities::default();
            assert_eq!(XInputGetCapabilities(3, 0, &mut caps), ERROR_SUCCESS);
            assert_eq!((caps.ty, caps.sub_type), (1, 1));
            assert_eq!(caps.flags, XINPUT_CAPS_WIRELESS | XINPUT_CAPS_FFB_SUPPORTED);
            assert_eq!(caps.vibration.left, 0xFFFF);
            assert_eq!(XInputGetCapabilities(3, 2, &mut caps), ERROR_BAD_ARGUMENTS);
            let mut v = Vibration { left: 1000, right: 2000 };
            assert_eq!(XInputSetState(3, &mut v), ERROR_SUCCESS);
            let mut bi = BatteryInformation::default();
            assert_eq!(XInputGetBatteryInformation(3, 0, &mut bi), ERROR_SUCCESS);
            assert_eq!((bi.ty, bi.level), (BATTERY_TYPE_NIMH, 1)); // 0.3 -> LOW
            backend_disconnect(3);
            assert_eq!(XInputGetState(3, &mut st), ERROR_DEVICE_NOT_CONNECTED);
            assert_eq!(XInputSetState(3, &mut v), ERROR_DEVICE_NOT_CONNECTED);
        }
    }

    #[test]
    fn virtual_touch_gamepad_and_phone_haptics_passthrough() {
        use std::sync::Mutex as StdMutex;
        static LOG: StdMutex<Vec<(u16, u16)>> = StdMutex::new(Vec::new());
        extern "C" fn capture_rumble(l: u16, r: u16) {
            LOG.lock().unwrap().push((l, r));
        }

        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        LOG.lock().unwrap().clear();
        backend_disconnect(0);
        set_virtual_gamepad(false, Gamepad::default());
        crate::macos::uikit::set_rumble_callback(None);

        unsafe {
            let mut st = State::default();
            assert_eq!(XInputGetState(0, &mut st), ERROR_DEVICE_NOT_CONNECTED);

            // Connect virtual touch overlay + register phone Taptic Engine callback.
            crate::macos::uikit::set_rumble_callback(Some(capture_rumble));
            let pad = Gamepad {
                buttons: btn::A | btn::RIGHT_SHOULDER,
                left_trigger: 100,
                right_trigger: 255,
                thumb_lx: 12000,
                thumb_ly: -24000,
                thumb_rx: 0,
                thumb_ry: 0,
            };
            set_virtual_gamepad(true, pad);
            assert_eq!(XInputGetState(0, &mut st), ERROR_SUCCESS);
            assert_eq!(st.gamepad, pad);
            assert_eq!(slot_state(0).map(|(p, _)| p), Some(pad));

            // Capabilities honestly advertise force feedback when phone callback is registered.
            let mut caps = Capabilities::default();
            assert_eq!(XInputGetCapabilities(0, 0, &mut caps), ERROR_SUCCESS);
            assert_ne!(caps.flags & XINPUT_CAPS_FFB_SUPPORTED, 0);
            assert_eq!((caps.vibration.left, caps.vibration.right), (0xFFFF, 0xFFFF));

            // XInputSetState forwards motor speeds to the phone haptics callback.
            let mut v = Vibration { left: 40000, right: 20000 };
            assert_eq!(XInputSetState(0, &mut v), ERROR_SUCCESS);
            // Duplicate identical call is coalesced.
            assert_eq!(XInputSetState(0, &mut v), ERROR_SUCCESS);
            assert_eq!(*LOG.lock().unwrap(), vec![(40000, 20000)]);

            // XInputEnable(0) stops active phone rumble immediately.
            XInputEnable(0);
            assert_eq!(*LOG.lock().unwrap(), vec![(40000, 20000), (0, 0)]);
            XInputEnable(1);

            // Non-rumble physical controller on slot 0 (e.g. Backbone/MFi) still falls back to phone haptics!
            backend_connect(0, DeviceInfo { name: "MFi Pad".into(), wireless: false, rumble: false, battery: None });
            let mut v2 = Vibration { left: 50000, right: 10000 };
            assert_eq!(XInputSetState(0, &mut v2), ERROR_SUCCESS);
            assert_eq!(*LOG.lock().unwrap(), vec![(40000, 20000), (0, 0), (50000, 10000)]);

            // Connecting a rumble-capable physical controller stops phone haptics and takes ownership.
            backend_connect(0, DeviceInfo { name: "DualSense".into(), wireless: true, rumble: true, battery: Some(0.9) });
            assert_eq!(*LOG.lock().unwrap(), vec![(40000, 20000), (0, 0), (50000, 10000), (0, 0)]);
            let mut v3 = Vibration { left: 60000, right: 60000 };
            assert_eq!(XInputSetState(0, &mut v3), ERROR_SUCCESS);
            assert_eq!(LOG.lock().unwrap().len(), 4, "physical rumble pad must not drive phone haptics");

            // Clean up global state.
            backend_disconnect(0);
            set_virtual_gamepad(false, Gamepad::default());
            crate::macos::uikit::set_rumble_callback(None);
        }
    }

    #[test]
    fn fast_table_concurrent_updates_and_seqlock_reads() {
        use std::sync::Arc;

        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        backend_disconnect(2);
        unsafe { XInputEnable(1) };

        // Null pointer and out-of-range user index return ERROR_BAD_ARGUMENTS.
        unsafe {
            let mut sentinel = State {
                packet: 0xDEAD_BEEF,
                gamepad: Gamepad {
                    buttons: 0x5A5A,
                    left_trigger: 11,
                    right_trigger: 22,
                    thumb_lx: 333,
                    thumb_ly: -444,
                    thumb_rx: 555,
                    thumb_ry: -666,
                },
            };
            assert_eq!(XInputGetState(0, std::ptr::null_mut()), ERROR_BAD_ARGUMENTS);
            assert_eq!(XInputGetStateEx(4, &mut sentinel), ERROR_BAD_ARGUMENTS);
            // Disconnected slot must return ERROR_DEVICE_NOT_CONNECTED and leave output untouched.
            assert_eq!(XInputGetState(2, &mut sentinel), ERROR_DEVICE_NOT_CONNECTED);
            assert_eq!(sentinel.packet, 0xDEAD_BEEF);
            assert_eq!(sentinel.gamepad.buttons, 0x5A5A);

            // Unaligned output buffer write test.
            backend_connect(2, DeviceInfo { name: "seqlock".into(), wireless: false, rumble: false, battery: None });
            let pad0 = Gamepad {
                buttons: btn::A | btn::GUIDE,
                left_trigger: 77,
                right_trigger: 88,
                thumb_lx: -12345,
                thumb_ly: 23456,
                thumb_rx: -32768,
                thumb_ry: 32767,
            };
            backend_update(2, pad0);
            let mut raw = [0xAAu8; 24];
            let unaligned = raw.as_mut_ptr().add(3) as *mut State;
            assert_eq!(XInputGetState(2, unaligned), ERROR_SUCCESS);
            let st_non_ex = std::ptr::read_unaligned(unaligned);
            assert_eq!(st_non_ex.gamepad.buttons, btn::A);
            assert_eq!((st_non_ex.gamepad.left_trigger, st_non_ex.gamepad.right_trigger), (77, 88));
            assert_eq!((st_non_ex.gamepad.thumb_lx, st_non_ex.gamepad.thumb_ly), (-12345, 23456));
            assert_eq!((st_non_ex.gamepad.thumb_rx, st_non_ex.gamepad.thumb_ry), (-32768, 32767));
            assert_eq!(XInputGetStateEx(2, unaligned), ERROR_SUCCESS);
            let st_ex = std::ptr::read_unaligned(unaligned);
            assert_eq!(st_ex.gamepad.buttons, btn::A | btn::GUIDE);
            assert_eq!(st_ex.packet, st_non_ex.packet);
        }

        // Seed slot 2 with a snapshot satisfying the reader invariant before starting readers.
        backend_update(
            2,
            Gamepad {
                buttons: btn::A | btn::GUIDE,
                left_trigger: 0,
                right_trigger: 0xFF,
                thumb_lx: 0,
                thumb_ly: 0,
                thumb_rx: 0,
                thumb_ry: 0,
            },
        );
        // Concurrent writer hammering slot 2 while 4 reader threads verify zero torn snapshots.
        let stop = Arc::new(AtomicBool::new(false));
        let mut readers = Vec::new();
        for tid in 0..4usize {
            let stop_r = Arc::clone(&stop);
            readers.push(std::thread::spawn(move || {
                let mut st = State::default();
                let mut last_pkt = 0u32;
                let mut samples = 0u64;
                while !stop_r.load(Ordering::Acquire) {
                    let rc = unsafe {
                        if tid % 2 == 0 {
                            XInputGetState(2, &mut st)
                        } else {
                            XInputGetStateEx(2, &mut st)
                        }
                    };
                    assert_eq!(rc, ERROR_SUCCESS);
                    // Invariant established by writer: thumb_lx == thumb_ly == -thumb_rx == -thumb_ry
                    // and left_trigger == (thumb_lx as u8), right_trigger == left_trigger ^ 0xFF.
                    let v = st.gamepad.thumb_lx;
                    assert_eq!(st.gamepad.thumb_ly, v, "torn snapshot: lx={v} ly={}", st.gamepad.thumb_ly);
                    assert_eq!(st.gamepad.thumb_rx, -v, "torn snapshot: lx={v} rx={}", st.gamepad.thumb_rx);
                    assert_eq!(st.gamepad.thumb_ry, -v, "torn snapshot: lx={v} ry={}", st.gamepad.thumb_ry);
                    let lt = v as u8;
                    assert_eq!(st.gamepad.left_trigger, lt, "torn trigger: lx={v} lt={}", st.gamepad.left_trigger);
                    assert_eq!(st.gamepad.right_trigger, lt ^ 0xFF, "torn trigger: lx={v} rt={}", st.gamepad.right_trigger);
                    if tid % 2 == 0 {
                        assert_eq!(st.gamepad.buttons & btn::GUIDE, 0, "XInputGetState leaked GUIDE bit");
                    }
                    if last_pkt != 0 {
                        assert!(
                            st.packet.wrapping_sub(last_pkt) < 0x8000_0000,
                            "packet moved backwards: {last_pkt} -> {}",
                            st.packet
                        );
                    }
                    last_pkt = st.packet;
                    samples += 1;
                }
                assert!(samples > 0);
            }));
        }
        for i in 1i16..=4000 {
            let lt = i as u8;
            backend_update(
                2,
                Gamepad {
                    buttons: btn::A | btn::GUIDE,
                    left_trigger: lt,
                    right_trigger: lt ^ 0xFF,
                    thumb_lx: i,
                    thumb_ly: i,
                    thumb_rx: -i,
                    thumb_ry: -i,
                },
            );
        }
        stop.store(true, Ordering::Release);
        for r in readers {
            r.join().unwrap();
        }
        backend_disconnect(2);
    }
}
