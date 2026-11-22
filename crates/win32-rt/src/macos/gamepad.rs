//! GameController.framework backend: XInput pads, GCMouse raw deltas and
//! CoreHaptics rumble. Main thread only.
//!
//! Windows threads never call into GameController (see the threading rules
//! in [`super`]):
//! * [`start`] and [`set_rumble`] are queued to the main thread.
//! * State flows back through `crate::misc::xinput1_4::backend_*` (a mutex
//!   table) and `crate::user32::input::native_raw_mouse`.
//!
//! * **Controllers.** Controllers that have an `extendedGamepad` profile
//!   get XInput user slots 0..3, in connection order, and keep their slot
//!   until they disconnect. `playerIndex` is set so the pad's LED shows the
//!   slot.
//! * **Input state.** State is pushed from the profile's
//!   `valueChangedHandler`, so there is no polling latency. As a safety net
//!   [`tick`] also re-reads it every main-loop iteration (at least every
//!   50 ms) and picks up connects and disconnects.
//! * **Background input.** `GCController.shouldMonitorBackgroundEvents` is
//!   turned on to match XInput, which reports input while the game is in
//!   the background.
//! * **Rumble.** Each motor gets one `CHHapticEngine` from
//!   `GCDeviceHaptics`: the left (low-frequency) motor uses the left handle
//!   and the right (high-frequency) motor uses the right handle. If the
//!   handles aren't separately addressable, both motors share the default
//!   locality. Each engine runs one infinite continuous event whose
//!   strength follows the XInput motor speed through the
//!   `HapticIntensityControl` dynamic parameter.
//!
//! * **Sony pads.** While `misc::hid` exposes a Sony controller (the
//!   game's own DualSense/DS4 path), DualSense/DualShock controllers are not
//!   reported through XInput, so the game sees each pad once.
//!
//! `AW2_GAMEPAD=0` turns the backend off (no controllers reported).

use crate::misc::xinput1_4 as xi;
use std::sync::atomic::{AtomicBool, Ordering};

static STARTED: AtomicBool = AtomicBool::new(false);

/// Start monitoring controllers and mice (idempotent, any thread). No-op
/// while headless.
pub fn start() {
    if std::env::var("AW2_GAMEPAD").is_ok_and(|v| v == "0") {
        log::info!("gamepad: disabled by AW2_GAMEPAD=0");
        return;
    }
    if !super::active() {
        log::info!("gamepad: no main loop (headless); no controllers");
        return;
    }
    super::run_async(|| {
        if STARTED.swap(true, Ordering::AcqRel) {
            return;
        }
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        imp::start();
    });
}

/// Set rumble motor speeds (XInput 0..65535) for a user slot (any thread).
pub fn set_rumble(user: usize, left: u16, right: u16) {
    super::run_async(move || {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        imp::rumble(user, left, right);
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        let _ = (user, left, right);
    });
}

/// Main-loop / display-link hook (main thread): connects/disconnects and state refresh.
pub fn tick() {
    if STARTED.load(Ordering::Acquire) {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        imp::tick();
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) mod imp {
    use super::xi;
    use block2::RcBlock;
    use objc2::rc::Retained;
    use objc2::runtime::ProtocolObject;
    use objc2::{msg_send, sel, AnyThread};
    use objc2_core_haptics::{
        CHHapticDynamicParameter, CHHapticDynamicParameterIDHapticIntensityControl, CHHapticEngine, CHHapticEvent,
        CHHapticEventParameter, CHHapticEventParameterIDHapticIntensity, CHHapticEventParameterIDHapticSharpness,
        CHHapticEventTypeHapticContinuous, CHHapticPattern, CHHapticPatternPlayer,
    };
    use objc2_foundation::{NSArray, NSObjectProtocol, NSString};
    use objc2_game_controller::{
        GCController, GCControllerElement, GCDevice, GCDeviceBatteryState, GCDeviceHaptics, GCExtendedGamepad,
        GCHapticDurationInfinite, GCHapticsLocality, GCHapticsLocalityDefault, GCHapticsLocalityLeftHandle,
        GCHapticsLocalityRightHandle, GCMouse, GCMouseInput,
    };
    use std::cell::RefCell;
    use std::ptr::NonNull;

    struct Motor {
        // Kept alive while the player runs.
        _engine: Retained<CHHapticEngine>,
        player: Retained<ProtocolObject<dyn CHHapticPatternPlayer>>,
        playing: bool,
        level: f32,
    }

    struct Pad {
        ctrl: Retained<GCController>,
        user: usize,
        /// Left/right motors; `shared` when both map to one engine.
        motors: Vec<Motor>,
        shared: bool,
        haptics_tried: bool,
        _handler: RcBlock<dyn Fn(NonNull<GCExtendedGamepad>, NonNull<GCControllerElement>)>,
    }

    #[derive(Default)]
    struct State {
        pads: Vec<Pad>,
        mice: Vec<(Retained<GCMouse>, RcBlock<dyn Fn(NonNull<GCMouseInput>, f32, f32)>)>,
    }

    thread_local! {
        static STATE: RefCell<State> = RefCell::new(State::default());
    }

    pub(super) fn start() {
        unsafe { GCController::setShouldMonitorBackgroundEvents(true) };
        log::info!("gamepad: GameController monitoring started");
        tick();
    }

    /// Read an extended gamepad profile into XInput terms.
    pub fn read_pad(gp: &GCExtendedGamepad) -> xi::GcPad {
        unsafe {
            let pressed = |b: Option<Retained<objc2_game_controller::GCControllerButtonInput>>| b.is_some_and(|b| b.isPressed());
            let dpad = gp.dpad();
            let (ls, rs) = (gp.leftThumbstick(), gp.rightThumbstick());
            xi::GcPad {
                a: gp.buttonA().isPressed(),
                b: gp.buttonB().isPressed(),
                x: gp.buttonX().isPressed(),
                y: gp.buttonY().isPressed(),
                left_shoulder: gp.leftShoulder().isPressed(),
                right_shoulder: gp.rightShoulder().isPressed(),
                left_thumb: pressed(gp.leftThumbstickButton()),
                right_thumb: pressed(gp.rightThumbstickButton()),
                menu: gp.buttonMenu().isPressed(),
                options: pressed(gp.buttonOptions()),
                home: pressed(gp.buttonHome()),
                dpad_up: dpad.up().isPressed(),
                dpad_down: dpad.down().isPressed(),
                dpad_left: dpad.left().isPressed(),
                dpad_right: dpad.right().isPressed(),
                left_trigger: gp.leftTrigger().value(),
                right_trigger: gp.rightTrigger().value(),
                lx: ls.xAxis().value(),
                ly: ls.yAxis().value(),
                rx: rs.xAxis().value(),
                ry: rs.yAxis().value(),
            }
        }
    }

    fn device_info(c: &GCController) -> xi::DeviceInfo {
        unsafe {
            let name = c.vendorName().map(|s| s.to_string()).unwrap_or_else(|| c.productCategory().to_string());
            let battery = c.battery();
            // GameController does not expose the transport; a pad running on
            // its battery is wireless.
            let wireless = battery.as_ref().is_some_and(|b| b.batteryState() == GCDeviceBatteryState::Discharging);
            xi::DeviceInfo {
                name,
                wireless,
                rumble: c.haptics().is_some(),
                battery: battery.map(|b| b.batteryLevel()),
            }
        }
    }

    /// Sony pads exposed through hid.dll are driven by the game's own pad
    /// library (haptics, adaptive triggers, lightbar); XInput must not
    /// report them a second time.
    fn hid_owned(c: &GCController) -> bool {
        if !crate::misc::hid::exposes_sony_pad() {
            return false;
        }
        let cat = unsafe { c.productCategory() }.to_string();
        cat.contains("DualSense") || cat.contains("DualShock")
    }

    pub(super) fn tick() {
        STATE.with(|st| {
            let mut st = st.borrow_mut();
            let ctrls = unsafe { GCController::controllers() };
            let present: Vec<Retained<GCController>> =
                ctrls.iter().filter(|c| unsafe { c.extendedGamepad() }.is_some() && !hid_owned(c)).collect();
            // Disconnects (or pads taken over by the HID path).
            st.pads.retain(|p| {
                let alive = present.iter().any(|c| Retained::as_ptr(c) == Retained::as_ptr(&p.ctrl));
                if !alive {
                    if let Some(gp) = unsafe { p.ctrl.extendedGamepad() } {
                        unsafe { gp.setValueChangedHandler(std::ptr::null_mut()) };
                    }
                    xi::backend_disconnect(p.user);
                }
                alive
            });
            // Connects.
            for c in present {
                if st.pads.iter().any(|p| Retained::as_ptr(&p.ctrl) == Retained::as_ptr(&c)) {
                    continue;
                }
                let Some(user) = (0..xi::XUSER_MAX_COUNT).find(|u| !st.pads.iter().any(|p| p.user == *u)) else {
                    continue; // more than four pads
                };
                let Some(gp) = (unsafe { c.extendedGamepad() }) else { continue };
                let handler = RcBlock::new(move |gp: NonNull<GCExtendedGamepad>, _e: NonNull<GCControllerElement>| {
                    xi::backend_update(user, read_pad(unsafe { gp.as_ref() }).to_xinput());
                });
                unsafe {
                    c.setPlayerIndex(objc2_game_controller::GCControllerPlayerIndex(user as isize));
                    gp.setValueChangedHandler(RcBlock::as_ptr(&handler));
                }
                xi::backend_connect(user, device_info(&c));
                xi::backend_update(user, read_pad(&gp).to_xinput());
                st.pads.push(Pad { ctrl: c, user, motors: Vec::new(), shared: false, haptics_tried: false, _handler: handler });
            }
            // Safety-net refresh (handlers normally keep this current).
            for p in &st.pads {
                if let Some(gp) = unsafe { p.ctrl.extendedGamepad() } {
                    xi::backend_update(p.user, read_pad(&gp).to_xinput());
                }
            }
            // Mice (raw deltas for WM_INPUT).
            let mice = unsafe { GCMouse::mice() };
            st.mice.retain(|(m, _)| mice.iter().any(|x| Retained::as_ptr(&x) == Retained::as_ptr(m)));
            for m in mice.iter() {
                if st.mice.iter().any(|(x, _)| Retained::as_ptr(x) == Retained::as_ptr(&m)) {
                    continue;
                }
                let Some(input) = (unsafe { m.mouseInput() }) else { continue };
                // GameController reports +Y up; raw input is +Y down.
                let h = RcBlock::new(|_i: NonNull<GCMouseInput>, dx: f32, dy: f32| {
                    crate::user32::input::native_raw_mouse(dx as f64, -(dy as f64));
                });
                unsafe { input.setMouseMovedHandler(RcBlock::as_ptr(&h)) };
                log::info!("gamepad: GCMouse attached ({}); raw mouse deltas come from it", unsafe { m.vendorName() }.map(|s| s.to_string()).unwrap_or_default());
                st.mice.push((m, h));
            }
        });
    }

    fn make_motor(h: &GCDeviceHaptics, loc: &GCHapticsLocality, sharpness: f32) -> Result<Motor, String> {
        unsafe {
            // createEngineWithLocality: exists on macOS 11+ but the bindings
            // only declare it for iOS/tvOS; send it directly.
            if !h.respondsToSelector(sel!(createEngineWithLocality:)) {
                return Err("createEngineWithLocality: unavailable".into());
            }
            let engine: Option<Retained<CHHapticEngine>> = msg_send![h, createEngineWithLocality: loc];
            let engine = engine.ok_or_else(|| format!("no engine for locality {loc}"))?;
            engine.startAndReturnError().map_err(|e| format!("engine start: {e:?}"))?;
            let params = NSArray::from_retained_slice(&[
                CHHapticEventParameter::initWithParameterID_value(CHHapticEventParameter::alloc(), CHHapticEventParameterIDHapticIntensity, 1.0),
                CHHapticEventParameter::initWithParameterID_value(CHHapticEventParameter::alloc(), CHHapticEventParameterIDHapticSharpness, sharpness),
            ]);
            let ev = CHHapticEvent::initWithEventType_parameters_relativeTime_duration(
                CHHapticEvent::alloc(),
                CHHapticEventTypeHapticContinuous,
                &params,
                0.0,
                GCHapticDurationInfinite as f64,
            );
            let pattern = CHHapticPattern::initWithEvents_parameters_error(CHHapticPattern::alloc(), &NSArray::from_retained_slice(&[ev]), &NSArray::new())
                .map_err(|e| format!("pattern: {e:?}"))?;
            let player = engine.createPlayerWithPattern_error(&pattern).map_err(|e| format!("player: {e:?}"))?;
            Ok(Motor { _engine: engine, player, playing: false, level: 0.0 })
        }
    }

    fn set_motor(m: &mut Motor, level: f32) -> Result<(), String> {
        if level == m.level {
            return Ok(());
        }
        unsafe {
            if level > 0.0 {
                if !m.playing {
                    m.player.startAtTime_error(0.0).map_err(|e| format!("start: {e:?}"))?;
                    m.playing = true;
                }
                let p = CHHapticDynamicParameter::initWithParameterID_value_relativeTime(
                    CHHapticDynamicParameter::alloc(),
                    CHHapticDynamicParameterIDHapticIntensityControl,
                    level,
                    0.0,
                );
                m.player.sendParameters_atTime_error(&NSArray::from_retained_slice(&[p]), 0.0).map_err(|e| format!("params: {e:?}"))?;
            } else if m.playing {
                m.player.stopAtTime_error(0.0).map_err(|e| format!("stop: {e:?}"))?;
                m.playing = false;
            }
        }
        m.level = level;
        Ok(())
    }

    pub(super) fn rumble(user: usize, left: u16, right: u16) {
        STATE.with(|st| {
            let mut st = st.borrow_mut();
            let Some(p) = st.pads.iter_mut().find(|p| p.user == user) else { return };
            if !p.haptics_tried {
                p.haptics_tried = true;
                let Some(h) = (unsafe { p.ctrl.haptics() }) else { return };
                let locs = unsafe { h.supportedLocalities() };
                let has = |l: &NSString| locs.containsObject(l);
                let (lh, rh) = unsafe { (GCHapticsLocalityLeftHandle, GCHapticsLocalityRightHandle) };
                let res = if has(lh) && has(rh) {
                    // Low-frequency (left) motor: dull; high-frequency (right): sharp.
                    make_motor(&h, lh, 0.1).and_then(|l| make_motor(&h, rh, 0.9).map(|r| vec![l, r]))
                } else {
                    p.shared = true;
                    make_motor(&h, unsafe { GCHapticsLocalityDefault }, 0.5).map(|m| vec![m])
                };
                match res {
                    Ok(m) => {
                        log::info!("gamepad: user {user} rumble via CoreHaptics ({} engine(s))", m.len());
                        p.motors = m;
                    }
                    Err(e) => log::warn!("gamepad: user {user} rumble unavailable: {e}"),
                }
            }
            let (l, r) = (left as f32 / 65535.0, right as f32 / 65535.0);
            let levels: Vec<f32> = if p.shared { vec![l.max(r)] } else { vec![l, r] };
            let mut failed = None;
            for (m, lv) in p.motors.iter_mut().zip(levels) {
                if let Err(e) = set_motor(m, lv) {
                    failed = Some(e);
                }
            }
            if let Some(e) = failed {
                log::warn!("gamepad: user {user} rumble failed ({e}); engines will be recreated");
                p.motors.clear();
                p.haptics_tried = false;
            }
        });
    }
}

/// Synthesized GameController state: a virtual extended gamepad
/// (`+[GCController controllerWithExtendedGamepad]`) whose elements are set
/// directly, read through the same `read_pad` the live handler uses.
#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::imp::read_pad;
    use super::xi::{self, btn};
    use objc2_game_controller::GCController;

    #[test]
    fn virtual_extended_gamepad_to_xinput() {
        unsafe {
            let c = GCController::controllerWithExtendedGamepad();
            let gp = c.extendedGamepad().expect("extended profile");
            assert_eq!(read_pad(&gp).to_xinput(), xi::Gamepad::default());
            gp.buttonA().setValue(1.0);
            gp.buttonY().setValue(1.0);
            gp.rightShoulder().setValue(1.0);
            gp.buttonMenu().setValue(1.0);
            gp.dpad().setValueForXAxis_yAxis(0.0, 1.0);
            gp.leftTrigger().setValue(0.5);
            gp.rightTrigger().setValue(1.0);
            gp.leftThumbstick().xAxis().setValue(-1.0);
            gp.leftThumbstick().yAxis().setValue(0.5);
            gp.rightThumbstick().yAxis().setValue(-0.25);
            if let Some(b) = gp.leftThumbstickButton() {
                b.setValue(1.0);
            }
            let x = read_pad(&gp).to_xinput();
            let mut want = btn::A | btn::Y | btn::RIGHT_SHOULDER | btn::START | btn::DPAD_UP;
            if gp.leftThumbstickButton().is_some() {
                want |= btn::LEFT_THUMB;
            }
            assert_eq!(x.buttons, want);
            assert_eq!((x.left_trigger, x.right_trigger), (128, 255));
            assert_eq!((x.thumb_lx, x.thumb_ly, x.thumb_rx, x.thumb_ry), (-32768, 16384, 0, -8192));
        }
    }
}
