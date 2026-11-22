# Windowing & Input (`crates/win32-rt`)

Win32 windowing, messaging, and input are split cleanly across two layers in `crates/win32-rt`:
- `src/user32/` — the Win32 model (window classes, message queues, focus, cursors, raw input,
  monitors, `DefWindowProc`).
- `src/macos/` — the native macOS backend (AppKit `NSWindow` + `CAMetalLayer`, `NSEvent` /
  `GCMouse` input, `GameController` XInput + CoreHaptics rumble, `IOHIDManager` raw HID).
- `src/misc/{xinput1_4,hid,setupapi}.rs` — gamepad and HID Win32 DLL exports.

---

## 1. Threading & the Deadlock Rule

**Cocoa owns the process main thread. Win32 threads own their own message queues.**

1. Before starting the guest's first Windows thread, `port-loader` calls
   `win32_rt::macos::init_main_thread()`. This sets up `NSApplication` with a regular activation
   policy, a Hide/Quit menu (using `AppConfig::app_name`), and the app delegate. The host main
   thread then runs `macos::run_main_loop()`, pumping `nextEventMatchingMask` with a 50 ms timeout
   and watching for process exit.
2. **Guest code never runs on the macOS main thread.** Window procedures (`WndProc`) execute only
   on the Win32 thread that created the window, inside `GetMessage`, `PeekMessage`, `SendMessage`,
   or `DispatchMessage` — just like Windows.
3. **Win32 → AppKit:**
   - `window_create` is the **only** `dispatch_sync` to the main queue (needed so the `NSWindow`
     exists before `CreateWindowEx` returns).
   - Every other UI call — destroy, set frame, style mask, show/hide, minimize, title, activate,
     cursor, warp — is a FIFO `dispatch_async` capturing only plain values.
   - Both helpers wrap their closure in `catch_unwind` and an autorelease pool.
4. **AppKit → Win32:**
   - AppKit delegates and views never call guest code directly.
   - Keyboard, mouse, wheel, and raw-mouse events turn into queued input messages on the owning
     Win32 thread's queue.
   - Window-state transitions (user resize/move, key/resign-key, miniaturize, live resize, backing
     or occlusion change, close button, Cmd-Q) enqueue *internal sent items* that run on the owner
     thread during its next queue poll and dispatch the normal `WM_*` sequence
     (`WM_WINDOWPOSCHANGING/CHANGED`, `WM_SIZE`, `WM_ACTIVATE[APP]`, `WM_CLOSE`, …).
5. **The deadlock rule:** The macOS main thread reads shared window metadata (for instance, the
   client origin when converting mouse coordinates). **Never hold a window lock on a Win32 thread
   while waiting on the main thread.** That is why everything except `window_create` is async.
6. **Lazy queues:** A thread's Win32 message queue is created lazily on its first `user32` call.
   Queue state sits under the kernel wait lock so `MsgWaitForMultipleObjects[Ex]` integrates cleanly
   with regular kernel handles.

Set `PORT_HEADLESS=1` (or `AW2_HEADLESS=1`) to disable AppKit completely: no `NSApplication`, a
synthetic 1920×1080 monitor, and pure in-memory Win32 windows. Unit tests always run headless off
the main thread.

---

## 2. Coordinates, Retina & Swapchain Attachment

- One Win32 pixel equals one backing pixel of the main display (`S = backingScaleFactor`, `2.0` on
  Retina). Set `AW2_RETINA=0` to force `S = 1`.
- Screen coordinates use top-left of `NSScreen.screens[0]` as `(0, 0)`, with `+y` pointing down.
- Reported DPI is **96** everywhere (`AW2_DPI=<n>` overrides it), so a Retina panel looks like a
  100% display at its native pixel resolution.
- **Known limitation:** secondary displays with a different scale factor currently inherit the main
  display's `S`.
- Each `NSWindow` hosts a `CAMetalLayer` with `contentsScale = S` and drawable size matching the
  Win32 client pixel size. D3D12, D3D11, and Vulkan (`VK_KHR_win32_surface`) all attach to the
  window through `macos::metal_layer_for_hwnd(hwnd)`.

### Window styles & closing

- `WS_POPUP` and frameless styles create borderless `NSWindow`s; overlapped styles get standard
  title bars (title-bar height comes from `frameRectForContentRect:styleMask:`). `WS_THICKFRAME`
  enables resize; `WS_MINIMIZEBOX` enables minimize.
- When a key borderless window covers an entire monitor, the menu bar and Dock auto-hide via
  `NSApplicationPresentationOptions`.
- The red close button and `Cmd-Q` translate into `WM_CLOSE` (`windowShouldClose` returns `NO` and
  `applicationShouldTerminate` returns Cancel), letting the game save state and exit cleanly.

---

## 3. Keyboard, Mouse & Raw Input

- **Keyboard:** `keyDown`, `keyUp`, and `flagsChanged` stay out of AppKit (except `Cmd` shortcuts).
  macOS keycodes map to Win32 VK and scan codes in `user32/keymap.rs` (Option → Alt, Caps Lock
  toggled, left/right modifiers preserved). Generates `WM_KEYDOWN`/`UP`, `WM_SYSKEY*`, `WM_CHAR`
  via `TranslateMessage`, raw keyboard `WM_INPUT`, and `GetKeyState`/`GetAsyncKeyState`.
  `ToUnicode` currently supports the US layout only.
- **Mouse & wheel:** Produces `WM_MOUSEMOVE`, button events, and `WM_MOUSEWHEEL`/`WM_MOUSEHWHEEL`
  (smooth scroll deltas accumulate into 120-unit notches). Mouse coalescing is disabled and the
  local event suppression interval is set to 0.
- **Raw mouse (`RIM_TYPEMOUSE`, `MOUSE_MOVE_RELATIVE`):** Controlled by `AW2_RAW_MOUSE`:
  - `gc` — `GCMouse` `mouseMovedHandler` (unaccelerated hardware deltas).
  - `nsevent` — `NSEvent` `deltaX`/`deltaY` (accelerated).
  - `auto` *(default)* — prefers `GCMouse` whenever it is active, falling back to `NSEvent` if
    `GCMouse` has been silent for > 500 ms (covers absolute pointers like tablets or KVMs).
- **Relative (FPS) mouse mode:** Activated automatically when a foreground window hides the cursor
  (`ShowCursor` count < 0 or `SetCursor(NULL)`) while raw mouse input is registered. Disassociates
  the cursor (`CGAssociateMouseAndMouseCursorPosition(0)`), warps to window center, and streams
  deltas only.
- **Cursor shapes & `ClipCursor`:**
  - Unloaded system cursor handles count as their system shape rather than hidden (preventing
    `IDC_ARROW` from falsely triggering relative mode).
  - `DefWindowProc(WM_SETCURSOR)` applies the class cursor only when non-NULL (Wine semantics).
  - `ClipCursor` clamps and warps the pointer back inside the clip rect when not in relative mode.
- **Registration:** `RegisterRawInputDevices`, `GetRegisteredRawInputDevices`, and
  `GetRawInputData` are implemented (`RIDEV_NOLEGACY` is accepted while legacy messages still flow;
  `GetRawInputBuffer` is not implemented yet).

---

## 4. Gamepads: XInput, DirectInput & DualSense HID

### XInput (`macos/gamepad.rs`, `misc/xinput1_4.rs`)
- All XInput DLL names (`xinput1_4`, `xinput1_3`, `xinput9_1_0`) alias to the builtin
  `xinput1_4.dll`.
- `GameController.framework` runs on the main thread and assigns controllers with `extendedGamepad`
  to XInput slots 0–3 in connection order. Sticks scale to `±32767`, triggers to `0..255`, and
  Menu/Options/Home map to `START`/`BACK`/`GUIDE` (`GUIDE` via `XInputGetStateEx`).
- **Rumble:** `XInputSetState` drives CoreHaptics engines created via
  `GCDeviceHaptics.createEngineWithLocality:` (left/right handles at sharpness 0.1 / 0.9, or one
  shared Default engine), updating intensity live and recreating engines after errors.
- **Exports:** `XInputGetState`, `XInputGetStateEx` (ordinal 100), `XInputSetState`,
  `XInputGetCapabilities`, `XInputEnable`, `XInputGetBatteryInformation`. (`XInputGetKeystroke`
  returns `ERROR_EMPTY`.)
- **DirectInput:** `DirectInput8Create` / `CoCreateInstance(CLSID_DirectInput8)` returns a minimal
  `IDirectInput8W` that enumerates zero devices (`DIERR_DEVICENOTREG` on `CreateDevice`/`FindDevice`).

### DualSense / DS4 Raw HID (`macos/hidmgr.rs`, `misc/hid.rs`, `misc/setupapi.rs`)
- Uses `IOHIDManager` across two native threads: `hid` runs a `CFRunLoop` for arrival, removal, and
  input reports; `hid-io` performs blocking `IOHIDDeviceGetReport` / `SetReport` (5 s timeout →
  `ERROR_SEM_TIMEOUT`).
- Exposes game controllers (Generic Desktop usages 4, 5, 8) opened shared. Whenever a Sony pad
  (VID `054C`: DS4, DualSense, DualSense Edge) is exposed over HID, `macos/gamepad.rs` hides it
  from XInput so games don't see duplicate controllers.
- Parses the device's actual HID report descriptor (`misc/hid/parse.rs`) to produce real Windows
  preparsed data and report lengths (e.g. DualSense USB `[64, 48, 64]`, BT `[78, 547, 64]`), and
  exposes Windows-style device paths (`\\?\hid#vid_054c&pid_0ce6#...`) via `SetupDi*`.
- Supports real overlapped I/O (`ReadFile` returning `ERROR_IO_PENDING`, async `WriteFile` for
  rumble/adaptive triggers/lightbar, `GetOverlappedResult[Ex]`, `CancelIo`/`CancelIoEx`, and
  `STATUS_DEVICE_NOT_CONNECTED` on unplug).
- **Known limitation:** DualSense USB audio-haptics endpoint pairing via `SPDRP_BASE_CONTAINERID`
  into `mmdevapi` is not wired yet.

---

## 5. Related Builtins & Known Stubs

- **Implemented:** `user32` (classes, windows, messages, timers, `DefWindowProc`, input, cursors,
  monitors, metrics, DPI awareness, `MessageBox` → `NSAlert`), `gdi32` (DCs, fonts, stock objects,
  `GetDeviceCaps`, approximate `GetTextExtentPoint32`), `dwmapi` (`DwmGet/SetWindowAttribute`,
  `DwmIsCompositionEnabled`, `DwmFlush`).
- **Stubbed (logged under `stub`):** clipboard payload transfer (open/close/empty work),
  `DialogBoxParam` (`-1`), hot keys, device notifications, `SPI_GETNONCLIENTMETRICS`, CCD display
  config (`ERROR_NOT_SUPPORTED`), `ReplyMessage`, `LoadIcon` placeholder, resource cursors,
  `GetRawInputBuffer`, `HidD_SetNumInputBuffers` (keeps fixed 32-report ring).
- **Why builtin DLLs synthesize real PE export directories:** Middleware such as Streamline
  (`sl.interposer.dll`) loads `dxgi.dll` and walks its PE export table directly instead of calling
  `GetProcAddress`. `port-loader` therefore synthesizes valid PE headers and export address tables
  for builtin DLLs.

---

## 6. Environment Variables & Tests

| Variable | Effect |
|---|---|
| `PORT_HEADLESS=1` / `AW2_HEADLESS=1` | Disable AppKit; synthetic 1920×1080 display |
| `AW2_RETINA=0` | Force 1:1 Win32 pixels to macOS points (`S = 1`) |
| `AW2_DPI=<n>` | Override reported DPI (default `96`) |
| `AW2_RAW_MOUSE=auto\|gc\|nsevent` | Raw mouse delta source (default `auto`) |
| `AW2_GAMEPAD=0` | Disable `GameController` XInput backend |
| `AW2_HID=0` | Disable `IOHIDManager` backend (Sony pads fall back to XInput) |
| `AW2_SCREENSHOT=<file.png>` | Capture window (+ `<stem>_screen.png`) after `AW2_SCREENSHOT_DELAY_MS` (default `3000`) |
| `AW2_EXIT_HOLD=<s>` | Keep windows alive for `s` seconds on fatal fault / trap |
| `AW2_TEST_GAMEPAD=1` | Enable live physical controller test (`AW2_TEST_GAMEPAD_SECS`, default `10`) |

Run tests:
```sh
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- user32 macos misc::hid misc::setupapi misc::xinput1_4 --test-threads=1
AW2_TEST_GAMEPAD=1 cargo test -p win32-rt --test gamepad_device --target aarch64-apple-darwin
```
