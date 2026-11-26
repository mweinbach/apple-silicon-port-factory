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

use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::{fence, AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, Once};
use std::time::{Duration, Instant};

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

/// `XINPUT_KEYSTROKE` (8 bytes, `Xinput.h`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Keystroke {
    pub virtual_key: u16,
    pub unicode: u16,
    pub flags: u16,
    pub user_index: u8,
    pub hid_code: u8,
}
const _: () = assert!(std::mem::size_of::<Keystroke>() == 8);

pub const XINPUT_KEYSTROKE_KEYDOWN: u16 = 0x0001;
pub const XINPUT_KEYSTROKE_KEYUP: u16 = 0x0002;
pub const XINPUT_KEYSTROKE_REPEAT: u16 = 0x0004;

/// Standard Windows XInput keystroke repeat timings (initial delay + repeat interval).
const KEYSTROKE_REPEAT_INITIAL: Duration = Duration::from_millis(400);
const KEYSTROKE_REPEAT_INTERVAL: Duration = Duration::from_millis(100);

pub mod vk_pad {
    pub const A: u16 = 0x5800;
    pub const B: u16 = 0x5801;
    pub const X: u16 = 0x5802;
    pub const Y: u16 = 0x5803;
    pub const RSHOULDER: u16 = 0x5804;
    pub const LSHOULDER: u16 = 0x5805;
    pub const LTRIGGER: u16 = 0x5806;
    pub const RTRIGGER: u16 = 0x5807;
    pub const DPAD_UP: u16 = 0x5810;
    pub const DPAD_DOWN: u16 = 0x5811;
    pub const DPAD_LEFT: u16 = 0x5812;
    pub const DPAD_RIGHT: u16 = 0x5813;
    pub const START: u16 = 0x5814;
    pub const BACK: u16 = 0x5815;
    pub const LTHUMB_PRESS: u16 = 0x5816;
    pub const RTHUMB_PRESS: u16 = 0x5817;
    pub const LTHUMB_UP: u16 = 0x5820;
    pub const LTHUMB_DOWN: u16 = 0x5821;
    pub const LTHUMB_RIGHT: u16 = 0x5822;
    pub const LTHUMB_LEFT: u16 = 0x5823;
    pub const LTHUMB_UPLEFT: u16 = 0x5824;
    pub const LTHUMB_UPRIGHT: u16 = 0x5825;
    pub const LTHUMB_DOWNRIGHT: u16 = 0x5826;
    pub const LTHUMB_DOWNLEFT: u16 = 0x5827;
    pub const RTHUMB_UP: u16 = 0x5830;
    pub const RTHUMB_DOWN: u16 = 0x5831;
    pub const RTHUMB_RIGHT: u16 = 0x5832;
    pub const RTHUMB_LEFT: u16 = 0x5833;
    pub const RTHUMB_UPLEFT: u16 = 0x5834;
    pub const RTHUMB_UPRIGHT: u16 = 0x5835;
    pub const RTHUMB_DOWNRIGHT: u16 = 0x5836;
    pub const RTHUMB_DOWNLEFT: u16 = 0x5837;
}

const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: u8 = 30;
const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: i32 = 7849;
const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: i32 = 8689;
const KEYSTROKE_QUEUE_CAP: usize = 64;

fn thumb_vk(x: i16, y: i16, deadzone: i32, base: u16) -> Option<u16> {
    let xi = x as i32;
    let yi = y as i32;
    let pos_x = xi > deadzone;
    let neg_x = xi < -deadzone;
    let pos_y = yi > deadzone;
    let neg_y = yi < -deadzone;
    match (pos_y, neg_y, pos_x, neg_x) {
        (true, false, false, false) => Some(base),     // UP
        (false, true, false, false) => Some(base + 1), // DOWN
        (false, false, true, false) => Some(base + 2), // RIGHT
        (false, false, false, true) => Some(base + 3), // LEFT
        (true, false, false, true) => Some(base + 4),  // UPLEFT
        (true, false, true, false) => Some(base + 5),  // UPRIGHT
        (false, true, true, false) => Some(base + 6),  // DOWNRIGHT
        (false, true, false, true) => Some(base + 7),  // DOWNLEFT
        _ => None,
    }
}

fn push_keystroke(q: &mut VecDeque<Keystroke>, ks: Keystroke) {
    if q.len() >= KEYSTROKE_QUEUE_CAP {
        q.pop_front();
    }
    q.push_back(ks);
}

fn record_key_transition(
    user: u8,
    vk: u16,
    down: bool,
    now: Instant,
    q: &mut VecDeque<Keystroke>,
    held: &mut Vec<(u16, Instant)>,
) {
    push_keystroke(
        q,
        Keystroke {
            virtual_key: vk,
            unicode: 0,
            flags: if down { XINPUT_KEYSTROKE_KEYDOWN } else { XINPUT_KEYSTROKE_KEYUP },
            user_index: user,
            hid_code: 0,
        },
    );
    if down {
        let next_due = now + KEYSTROKE_REPEAT_INITIAL;
        if let Some((_, due)) = held.iter_mut().find(|(k, _)| *k == vk) {
            *due = next_due;
        } else {
            held.push((vk, next_due));
        }
    } else {
        held.retain(|(k, _)| *k != vk);
    }
}

fn emit_due_repeats(
    user: u8,
    now: Instant,
    q: &mut VecDeque<Keystroke>,
    held: &mut [(u16, Instant)],
) {
    for (vk, next_due) in held.iter_mut() {
        if now >= *next_due {
            push_keystroke(
                q,
                Keystroke {
                    virtual_key: *vk,
                    unicode: 0,
                    flags: XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT,
                    user_index: user,
                    hid_code: 0,
                },
            );
            *next_due = now + KEYSTROKE_REPEAT_INTERVAL;
        }
    }
}

fn diff_gamepad_keystrokes(
    user: u8,
    prev: Gamepad,
    next: Gamepad,
    now: Instant,
    q: &mut VecDeque<Keystroke>,
    held: &mut Vec<(u16, Instant)>,
) {
    const BUTTON_MAP: &[(u16, u16)] = &[
        (btn::A, vk_pad::A),
        (btn::B, vk_pad::B),
        (btn::X, vk_pad::X),
        (btn::Y, vk_pad::Y),
        (btn::RIGHT_SHOULDER, vk_pad::RSHOULDER),
        (btn::LEFT_SHOULDER, vk_pad::LSHOULDER),
        (btn::DPAD_UP, vk_pad::DPAD_UP),
        (btn::DPAD_DOWN, vk_pad::DPAD_DOWN),
        (btn::DPAD_LEFT, vk_pad::DPAD_LEFT),
        (btn::DPAD_RIGHT, vk_pad::DPAD_RIGHT),
        (btn::START, vk_pad::START),
        (btn::BACK, vk_pad::BACK),
        (btn::LEFT_THUMB, vk_pad::LTHUMB_PRESS),
        (btn::RIGHT_THUMB, vk_pad::RTHUMB_PRESS),
    ];
    for &(mask, vk) in BUTTON_MAP {
        let was = (prev.buttons & mask) != 0;
        let is_down = (next.buttons & mask) != 0;
        if was != is_down {
            record_key_transition(user, vk, is_down, now, q, held);
        }
    }
    for (was_t, now_t, vk) in [
        (prev.left_trigger >= XINPUT_GAMEPAD_TRIGGER_THRESHOLD, next.left_trigger >= XINPUT_GAMEPAD_TRIGGER_THRESHOLD, vk_pad::LTRIGGER),
        (prev.right_trigger >= XINPUT_GAMEPAD_TRIGGER_THRESHOLD, next.right_trigger >= XINPUT_GAMEPAD_TRIGGER_THRESHOLD, vk_pad::RTRIGGER),
    ] {
        if was_t != now_t {
            record_key_transition(user, vk, now_t, now, q, held);
        }
    }
    for (prev_dir, next_dir) in [
        (
            thumb_vk(prev.thumb_lx, prev.thumb_ly, XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE, vk_pad::LTHUMB_UP),
            thumb_vk(next.thumb_lx, next.thumb_ly, XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE, vk_pad::LTHUMB_UP),
        ),
        (
            thumb_vk(prev.thumb_rx, prev.thumb_ry, XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE, vk_pad::RTHUMB_UP),
            thumb_vk(next.thumb_rx, next.thumb_ry, XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE, vk_pad::RTHUMB_UP),
        ),
    ] {
        if prev_dir != next_dir {
            if let Some(old_vk) = prev_dir {
                record_key_transition(user, old_vk, false, now, q, held);
            }
            if let Some(new_vk) = next_dir {
                record_key_transition(user, new_vk, true, now, q, held);
            }
        }
    }
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
    keystrokes: VecDeque<Keystroke>,
    held_vks: Vec<(u16, Instant)>,
}

#[derive(Clone, Default)]
struct VirtualPadState {
    connected: bool,
    pad: Gamepad,
    packet: u32,
    vibration: (u16, u16),
    /// Authoritative merged slot-0 connection/pad state used to generate slot-0 keystrokes
    /// and repeats without duplicating or prematurely releasing overlapped physical+virtual inputs.
    merged_connected: bool,
    merged_pad: Gamepad,
    keystrokes: VecDeque<Keystroke>,
    held_vks: Vec<(u16, Instant)>,
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
    merged_connected: false,
    merged_pad: Gamepad {
        buttons: 0,
        left_trigger: 0,
        right_trigger: 0,
        thumb_lx: 0,
        thumb_ly: 0,
        thumb_rx: 0,
        thumb_ry: 0,
    },
    keystrokes: VecDeque::new(),
    held_vks: Vec::new(),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot0Transition {
    None,
    Connected(Gamepad),
    Disconnected,
    StateChanged(Gamepad),
}

fn emit_slot0_transition(transition: Slot0Transition) {
    match transition {
        Slot0Transition::None => {}
        Slot0Transition::Connected(initial_pad) => {
            crate::misc::gameinput::on_gamepad_connected(0, &initial_pad);
        }
        Slot0Transition::Disconnected => {
            crate::misc::gameinput::on_gamepad_disconnected(0);
        }
        Slot0Transition::StateChanged(pad) => {
            crate::misc::gameinput::on_gamepad_state_changed(0, &pad);
        }
    }
}

/// Recompute the merged slot-0 controller state from physical `slots[0]` and `virt`,
/// generating keystroke transitions and repeat tracking exclusively from the merged state,
/// and return the resulting GameInput transition to emit after releasing locks.
/// Caller MUST hold `SLOTS` first, then `VIRTUAL_PAD` (global lock ordering).
fn sync_slot0_keystrokes(
    slots: &[Option<Slot>; XUSER_MAX_COUNT],
    virt: &mut VirtualPadState,
    now: Instant,
) -> Slot0Transition {
    let prev_connected = virt.merged_connected;
    let prev_pad = virt.merged_pad;
    let phys = slots[0].as_ref().filter(|s| s.connected).map(|s| s.pad);
    let merged = match (phys, virt.connected) {
        (Some(p), true) => Some(if virt.pad != Gamepad::default() { merge_gamepad(p, virt.pad) } else { p }),
        (Some(p), false) => Some(p),
        (None, true) => Some(virt.pad),
        (None, false) => None,
    };
    match merged {
        Some(next_pad) => {
            let prev = if prev_connected { prev_pad } else { Gamepad::default() };
            let changed = !prev_connected || prev != next_pad;
            virt.merged_connected = true;
            virt.merged_pad = next_pad;
            if changed {
                let VirtualPadState { keystrokes, held_vks, .. } = virt;
                diff_gamepad_keystrokes(0, prev, next_pad, now, keystrokes, held_vks);
            }
            if !prev_connected {
                Slot0Transition::Connected(next_pad)
            } else if prev_pad != next_pad {
                Slot0Transition::StateChanged(next_pad)
            } else {
                Slot0Transition::None
            }
        }
        None => {
            virt.merged_connected = false;
            virt.merged_pad = Gamepad::default();
            virt.keystrokes.clear();
            virt.held_vks.clear();
            if prev_connected {
                Slot0Transition::Disconnected
            } else {
                Slot0Transition::None
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Backend interface (main thread or tests)
// ---------------------------------------------------------------------------

pub fn backend_connect(user: usize, info: DeviceInfo) {
    log::info!("xinput: user {user} connected: {} (wireless={}, rumble={})", info.name, info.wireless, info.rumble);
    let phys_rumble = info.rumble;
    let (stop_phone, transition) = if user == 0 {
        let now = Instant::now();
        let mut slots = SLOTS.lock().unwrap();
        let slot = slots[0].get_or_insert_with(Slot::default);
        *slot = Slot { connected: true, info, packet: slot.packet.wrapping_add(1), ..Default::default() };
        let mut v = VIRTUAL_PAD.lock().unwrap();
        let had = if phys_rumble {
            let h = v.vibration != (0, 0);
            v.vibration = (0, 0);
            h
        } else {
            false
        };
        let transition = sync_slot0_keystrokes(&slots, &mut v, now);
        (had, transition)
    } else {
        let transition = with_slot(user, |s| {
            let was_connected = s.connected;
            let prev_pad = s.pad;
            *s = Slot { connected: true, info, packet: s.packet.wrapping_add(1), ..Default::default() };
            if !was_connected {
                Slot0Transition::Connected(Gamepad::default())
            } else if prev_pad != Gamepad::default() {
                Slot0Transition::StateChanged(Gamepad::default())
            } else {
                Slot0Transition::None
            }
        });
        (false, transition)
    };
    if stop_phone {
        crate::macos::uikit::dispatch_host_rumble(0, 0);
    }
    if user == 0 {
        emit_slot0_transition(transition);
    } else {
        match transition {
            Slot0Transition::Connected(initial_pad) => {
                crate::misc::gameinput::on_gamepad_connected(user, &initial_pad);
            }
            Slot0Transition::StateChanged(pad) => {
                crate::misc::gameinput::on_gamepad_state_changed(user, &pad);
            }
            Slot0Transition::Disconnected | Slot0Transition::None => {}
        }
    }
    publish_fast_slot(user);
    crate::user32::misc::notify_device_change(true);
}

pub fn backend_disconnect(user: usize) {
    log::info!("xinput: user {user} disconnected");
    let (was_connected, transition) = if user == 0 {
        let now = Instant::now();
        let mut slots = SLOTS.lock().unwrap();
        let slot = slots[0].get_or_insert_with(Slot::default);
        let was = slot.connected;
        slot.connected = false;
        slot.pad = Gamepad::default();
        slot.vibration = (0, 0);
        slot.keystrokes.clear();
        slot.held_vks.clear();
        let mut v = VIRTUAL_PAD.lock().unwrap();
        let transition = sync_slot0_keystrokes(&slots, &mut v, now);
        (was, transition)
    } else {
        with_slot(user, |s| {
            let was = s.connected;
            s.connected = false;
            s.pad = Gamepad::default();
            s.vibration = (0, 0);
            s.keystrokes.clear();
            s.held_vks.clear();
            (was, Slot0Transition::Disconnected)
        })
    };
    publish_fast_slot(user);
    if user == 0 {
        emit_slot0_transition(transition);
    } else {
        crate::misc::gameinput::on_gamepad_disconnected(user);
    }
    if was_connected {
        crate::user32::misc::notify_device_change(false);
    }
}

/// New input state. `dwPacketNumber` only changes when the state did.
pub fn backend_update(user: usize, pad: Gamepad) {
    let now = Instant::now();
    if user == 0 {
        let (changed, transition) = {
            let mut slots = SLOTS.lock().unwrap();
            let slot = slots[0].get_or_insert_with(Slot::default);
            if slot.connected && slot.pad != pad {
                slot.pad = pad;
                slot.packet = slot.packet.wrapping_add(1);
                let mut v = VIRTUAL_PAD.lock().unwrap();
                (true, sync_slot0_keystrokes(&slots, &mut v, now))
            } else {
                (false, Slot0Transition::None)
            }
        };
        if changed {
            publish_fast_slot(0);
        }
        emit_slot0_transition(transition);
    } else {
        let updated_pad = with_slot(user, |s| {
            if s.connected && s.pad != pad {
                let prev = s.pad;
                s.pad = pad;
                s.packet = s.packet.wrapping_add(1);
                diff_gamepad_keystrokes(user as u8, prev, pad, now, &mut s.keystrokes, &mut s.held_vks);
                Some(pad)
            } else {
                None
            }
        });
        if let Some(effective) = updated_pad {
            publish_fast_slot(user);
            crate::misc::gameinput::on_gamepad_state_changed(user, &effective);
        }
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
    let (conn_changed, stop_phone, transition) = {
        let now = Instant::now();
        let slots = SLOTS.lock().unwrap();
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
        let transition = if changed {
            v.packet = v.packet.wrapping_add(1);
            sync_slot0_keystrokes(&slots, &mut v, now)
        } else {
            Slot0Transition::None
        };
        let stop_phone = if !connected && v.vibration != (0, 0) {
            v.vibration = (0, 0);
            true
        } else {
            false
        };
        (conn_changed, stop_phone, transition)
    };
    publish_fast_slot(0);
    if stop_phone {
        crate::macos::uikit::dispatch_host_rumble(0, 0);
    }
    emit_slot0_transition(transition);
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
                    keystrokes: VecDeque::new(),
                    held_vks: Vec::new(),
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

pub unsafe extern "win64" fn XInputGetKeystroke(user: u32, _r: u32, k: *mut c_void) -> u32 {
    if k.is_null() || (user as usize >= XUSER_MAX_COUNT && user != 0xFF) {
        return ERROR_BAD_ARGUMENTS;
    }
    ensure_started();
    let mut slots = SLOTS.lock().unwrap();
    let mut virt = VIRTUAL_PAD.lock().unwrap();
    let any_phys = slots
        .iter()
        .enumerate()
        .any(|(i, s)| (user == 0xFF || i == user as usize) && s.as_ref().is_some_and(|s| s.connected));
    let any_virt = (user == 0xFF || user == 0) && virt.connected;
    if !any_phys && !any_virt {
        return ERROR_DEVICE_NOT_CONNECTED;
    }
    if !ENABLED.load(Ordering::Acquire) {
        return ERROR_EMPTY;
    }
    let now = Instant::now();
    let range = if user == 0xFF { 0..XUSER_MAX_COUNT } else { (user as usize)..(user as usize + 1) };
    for idx in range {
        if idx == 0 {
            if virt.merged_connected {
                if virt.keystrokes.is_empty() && !virt.held_vks.is_empty() {
                    let VirtualPadState { keystrokes, held_vks, .. } = &mut *virt;
                    emit_due_repeats(0, now, keystrokes, held_vks);
                }
                if let Some(ks) = virt.keystrokes.pop_front() {
                    (k as *mut Keystroke).write_unaligned(ks);
                    return ERROR_SUCCESS;
                }
            }
        } else if let Some(slot) = slots[idx].as_mut().filter(|s| s.connected) {
            if slot.keystrokes.is_empty() && !slot.held_vks.is_empty() {
                emit_due_repeats(idx as u8, now, &mut slot.keystrokes, &mut slot.held_vks);
            }
            if let Some(ks) = slot.keystrokes.pop_front() {
                (k as *mut Keystroke).write_unaligned(ks);
                return ERROR_SUCCESS;
            }
        }
    }
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

            // XInputGetKeystroke returns queued button, trigger, and thumbstick events.
            let mut ks = Keystroke::default();
            // Drain events queued by earlier update(s) on slot 3: A + LTRIGGER + LTHUMB_LEFT
            let mut got_vks = Vec::new();
            while XInputGetKeystroke(3, 0, &mut ks as *mut _ as *mut c_void) == ERROR_SUCCESS {
                assert_eq!(ks.user_index, 3);
                got_vks.push((ks.virtual_key, ks.flags));
            }
            assert!(got_vks.contains(&(vk_pad::A, XINPUT_KEYSTROKE_KEYDOWN)));
            assert!(got_vks.contains(&(vk_pad::LTRIGGER, XINPUT_KEYSTROKE_KEYDOWN)));
            assert!(got_vks.contains(&(vk_pad::LTHUMB_LEFT, XINPUT_KEYSTROKE_KEYDOWN)));
            assert_eq!(XInputGetKeystroke(3, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            // Release all -> KEYUP events delivered via XUSER_INDEX_ANY (0xFF).
            backend_update(3, Gamepad::default());
            let mut up_vks = Vec::new();
            while XInputGetKeystroke(0xFF, 0, &mut ks as *mut _ as *mut c_void) == ERROR_SUCCESS {
                assert_eq!(ks.user_index, 3);
                up_vks.push((ks.virtual_key, ks.flags));
            }
            assert!(up_vks.contains(&(vk_pad::A, XINPUT_KEYSTROKE_KEYUP)));
            assert!(up_vks.contains(&(vk_pad::LTRIGGER, XINPUT_KEYSTROKE_KEYUP)));
            assert!(up_vks.contains(&(vk_pad::LTHUMB_LEFT, XINPUT_KEYSTROKE_KEYUP)));

            backend_disconnect(3);
            assert_eq!(XInputGetKeystroke(3, 0, &mut ks as *mut _ as *mut c_void), ERROR_DEVICE_NOT_CONNECTED);
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

    #[test]
    fn keystroke_repeats_for_held_inputs_on_physical_and_virtual_slots() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        backend_disconnect(2);
        backend_disconnect(0);
        set_virtual_gamepad(false, Gamepad::default());
        unsafe { XInputEnable(1) };

        let backdate_phys = |slot_idx: usize, delta: Duration| {
            let mut s = SLOTS.lock().unwrap();
            if let Some(slot) = s[slot_idx].as_mut() {
                for (_, due) in &mut slot.held_vks {
                    *due = due.checked_sub(delta).unwrap_or(*due);
                }
            }
        };
        let backdate_virt = |delta: Duration| {
            let mut v = VIRTUAL_PAD.lock().unwrap();
            for (_, due) in &mut v.held_vks {
                *due = due.checked_sub(delta).unwrap_or(*due);
            }
        };

        unsafe {
            // 1. Physical controller on slot 2: hold DPAD_DOWN + left thumbstick up.
            backend_connect(2, DeviceInfo { name: "repeat-test".into(), wireless: false, rumble: false, battery: None });
            let held_pad = Gamepad {
                buttons: btn::DPAD_DOWN,
                thumb_ly: 24_000,
                ..Default::default()
            };
            backend_update(2, held_pad);

            let mut ks = Keystroke::default();
            let mut initial = Vec::new();
            while XInputGetKeystroke(2, 0, &mut ks as *mut _ as *mut c_void) == ERROR_SUCCESS {
                initial.push((ks.virtual_key, ks.flags, ks.user_index));
            }
            assert_eq!(initial.len(), 2);
            assert!(initial.contains(&(vk_pad::DPAD_DOWN, XINPUT_KEYSTROKE_KEYDOWN, 2)));
            assert!(initial.contains(&(vk_pad::LTHUMB_UP, XINPUT_KEYSTROKE_KEYDOWN, 2)));
            // Before initial repeat delay expires, polling returns ERROR_EMPTY.
            assert_eq!(XInputGetKeystroke(2, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            // Simulate holding past initial repeat delay (400 ms) without any new backend update.
            backdate_phys(2, KEYSTROKE_REPEAT_INITIAL + Duration::from_millis(10));
            let mut repeats1 = Vec::new();
            while XInputGetKeystroke(2, 0, &mut ks as *mut _ as *mut c_void) == ERROR_SUCCESS {
                repeats1.push((ks.virtual_key, ks.flags, ks.user_index));
            }
            assert_eq!(repeats1.len(), 2);
            assert!(repeats1.contains(&(vk_pad::DPAD_DOWN, XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT, 2)));
            assert!(repeats1.contains(&(vk_pad::LTHUMB_UP, XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT, 2)));
            // Immediately after repeat emission, next poll is empty until repeat interval elapses.
            assert_eq!(XInputGetKeystroke(2, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            // Simulate repeat interval (100 ms) elapsing -> fires again via XUSER_INDEX_ANY (0xFF).
            backdate_phys(2, KEYSTROKE_REPEAT_INTERVAL + Duration::from_millis(10));
            let mut repeats2 = Vec::new();
            while XInputGetKeystroke(0xFF, 0, &mut ks as *mut _ as *mut c_void) == ERROR_SUCCESS {
                repeats2.push((ks.virtual_key, ks.flags, ks.user_index));
            }
            assert!(repeats2.contains(&(vk_pad::DPAD_DOWN, XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT, 2)));
            assert!(repeats2.contains(&(vk_pad::LTHUMB_UP, XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT, 2)));

            // Release DPAD_DOWN while keeping LTHUMB_UP held -> only LTHUMB_UP continues repeating.
            backend_update(2, Gamepad { thumb_ly: 24_000, ..Default::default() });
            assert_eq!(XInputGetKeystroke(2, 0, &mut ks as *mut _ as *mut c_void), ERROR_SUCCESS);
            assert_eq!((ks.virtual_key, ks.flags), (vk_pad::DPAD_DOWN, XINPUT_KEYSTROKE_KEYUP));

            backdate_phys(2, KEYSTROKE_REPEAT_INTERVAL + Duration::from_millis(10));
            let mut repeats3 = Vec::new();
            while XInputGetKeystroke(2, 0, &mut ks as *mut _ as *mut c_void) == ERROR_SUCCESS {
                repeats3.push((ks.virtual_key, ks.flags));
            }
            assert_eq!(repeats3, vec![(vk_pad::LTHUMB_UP, XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT)]);

            backend_disconnect(2);

            // 2. Virtual touch gamepad on slot 0: held button repeats and stops on disconnect.
            set_virtual_gamepad(true, Gamepad { buttons: btn::A, ..Default::default() });
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_SUCCESS);
            assert_eq!((ks.virtual_key, ks.flags, ks.user_index), (vk_pad::A, XINPUT_KEYSTROKE_KEYDOWN, 0));
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            backdate_virt(KEYSTROKE_REPEAT_INITIAL + Duration::from_millis(10));
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_SUCCESS);
            assert_eq!(
                (ks.virtual_key, ks.flags, ks.user_index),
                (vk_pad::A, XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT, 0)
            );

            set_virtual_gamepad(false, Gamepad::default());
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_DEVICE_NOT_CONNECTED);

            // 3. Merged slot-0 physical + virtual overlay: pressing A on both and releasing only
            //    the physical button must NOT produce duplicate KEYDOWN or premature KEYUP.
            backend_connect(0, DeviceInfo { name: "phys0".into(), wireless: false, rumble: false, battery: None });
            set_virtual_gamepad(true, Gamepad::default());
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            // Press A physically -> single KEYDOWN(A).
            backend_update(0, Gamepad { buttons: btn::A, ..Default::default() });
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_SUCCESS);
            assert_eq!((ks.virtual_key, ks.flags, ks.user_index), (vk_pad::A, XINPUT_KEYSTROKE_KEYDOWN, 0));
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            // Press A on touch overlay while physical A is held -> NO duplicate KEYDOWN.
            set_virtual_gamepad(true, Gamepad { buttons: btn::A, ..Default::default() });
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            // Release physical A while touch overlay still holds A -> NO premature KEYUP, and repeats continue.
            backend_update(0, Gamepad::default());
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);
            backdate_virt(KEYSTROKE_REPEAT_INITIAL + Duration::from_millis(10));
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_SUCCESS);
            assert_eq!(
                (ks.virtual_key, ks.flags, ks.user_index),
                (vk_pad::A, XINPUT_KEYSTROKE_KEYDOWN | XINPUT_KEYSTROKE_REPEAT, 0)
            );

            // Release touch overlay A -> single KEYUP(A).
            set_virtual_gamepad(true, Gamepad::default());
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_SUCCESS);
            assert_eq!((ks.virtual_key, ks.flags, ks.user_index), (vk_pad::A, XINPUT_KEYSTROKE_KEYUP, 0));
            assert_eq!(XInputGetKeystroke(0, 0, &mut ks as *mut _ as *mut c_void), ERROR_EMPTY);

            backend_disconnect(0);
            set_virtual_gamepad(false, Gamepad::default());
        }
    }
}
