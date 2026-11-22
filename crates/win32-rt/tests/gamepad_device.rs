//! Real-device gamepad test (runs on the process main thread, so AppKit
//! and GameController work as they do in the loader).
//!
//! ```sh
//! AW2_TEST_GAMEPAD=1 cargo test -p win32-rt --target x86_64-apple-darwin --test gamepad_device
//! ```
//!
//! With a controller connected (Xbox / DualSense / MFi), this:
//! 1. waits for XInput user 0 to report connected,
//! 2. pulses rumble on the left motor, then the right,
//! 3. prints every state change for `AW2_TEST_GAMEPAD_SECS` (default 10)
//!    seconds. Press buttons and move the sticks during that time.
//!
//! It fails if no pad connects within that time. Without the env var it
//! prints "skipped" and passes (so `cargo hybrid-test` stays hermetic).

use win32_rt::misc::xinput1_4 as xi;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

static DONE: AtomicU32 = AtomicU32::new(u32::MAX);

fn main() {
    if std::env::var("AW2_TEST_GAMEPAD").as_deref() != Ok("1") {
        println!("gamepad_device: skipped (set AW2_TEST_GAMEPAD=1 with a controller connected)");
        return;
    }
    let secs: u64 = std::env::var("AW2_TEST_GAMEPAD_SECS").ok().and_then(|s| s.parse().ok()).unwrap_or(10);
    assert!(win32_rt::macos::init_main_thread(), "AppKit init failed");
    std::thread::spawn(move || {
        let code = unsafe { exercise(Duration::from_secs(secs)) };
        DONE.store(code, Ordering::Release);
    });
    let code = win32_rt::macos::run_main_loop(&|| match DONE.load(Ordering::Acquire) {
        u32::MAX => None,
        c => Some(c),
    });
    std::process::exit(code as i32);
}

unsafe fn exercise(total: Duration) -> u32 {
    let start = Instant::now();
    let mut st = xi::State::default();
    let mut connected = None;
    while start.elapsed() < total {
        for u in 0..4 {
            if xi::XInputGetState(u, &mut st) == 0 {
                connected = Some(u);
                break;
            }
        }
        if connected.is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let Some(user) = connected else {
        println!("gamepad_device: FAIL: no controller connected within {total:?}");
        return 1;
    };
    let mut caps = xi::Capabilities::default();
    xi::XInputGetCapabilities(user, 0, &mut caps);
    let mut bat = xi::BatteryInformation::default();
    xi::XInputGetBatteryInformation(user, 0, &mut bat);
    println!(
        "gamepad_device: user {user} connected: caps type {} sub {} flags {:#x} vibration {:?}; battery type {} level {}",
        caps.ty, caps.sub_type, caps.flags, caps.vibration, bat.ty, bat.level
    );
    for (name, v) in [("left", xi::Vibration { left: 0xC000, right: 0 }), ("right", xi::Vibration { left: 0, right: 0xC000 })] {
        let mut v = v;
        println!("gamepad_device: rumble {name} motor (0.5 s)");
        assert_eq!(xi::XInputSetState(user, &mut v), 0);
        std::thread::sleep(Duration::from_millis(500));
        let mut off = xi::Vibration::default();
        xi::XInputSetState(user, &mut off);
        std::thread::sleep(Duration::from_millis(200));
    }
    println!("gamepad_device: press buttons / move sticks; printing changes until {total:?}");
    let mut last = 0u32;
    let mut changes = 0;
    while start.elapsed() < total {
        if xi::XInputGetState(user, &mut st) != 0 {
            println!("gamepad_device: user {user} disconnected");
            break;
        }
        if st.packet != last {
            last = st.packet;
            changes += 1;
            let g = st.gamepad;
            println!(
                "  packet {:>6} buttons {:#06x} LT {:>3} RT {:>3} L ({:>6},{:>6}) R ({:>6},{:>6})",
                st.packet, g.buttons, g.left_trigger, g.right_trigger, g.thumb_lx, g.thumb_ly, g.thumb_rx, g.thumb_ry
            );
        }
        std::thread::sleep(Duration::from_millis(8));
    }
    println!("gamepad_device: PASS ({changes} state changes observed)");
    0
}
