# Design advice: M2 (Win32 windowing on AppKit) and A1 (WASAPI on CoreAudio)

Status: advisory memo, 2026-10-05. Facts tagged **[V]** were checked against the binary
(`tools/ghidra_decompile.sh`, `tools/fndb.sh`, GUIDs read from the exe). **[A]** marks
assumptions or recall of external projects (Wine, Apple docs) that you should confirm before
relying on them.

---

## 0. What the game actually does (read this first)

### Windowing [V]
- **The window does not belong to WinMain's thread.** `app::SystemEventQueueWindows` starts a
  dedicated **`MessagePumpThread`** (`vfunc_1` @ `0x1418687b0`):
  ```c
  PeekMessageW(&msg, 0, 0, 0, PM_NOREMOVE);   // creates the queue
  drain_user_functions();                     // FUN_1418685f0
  while (GetMessageW(&msg, 0, 0, 0)) {
      if (msg.message != 0x8000) { TranslateMessage(&msg); DispatchMessageW(&msg); }
      drain_user_functions();                 // SRW-locked ring of functors
  }
  ```
  Other threads push functors (`UserFunctionImpl<...>`) into that ring and wake the pump with
  `PostThreadMessageA(pumpTid, 0x8000 /*WM_APP*/, 0, 0)` (`0x14186a310`). The return value is
  ignored except for `GetLastError()`.
- **`CreateWindowExW` runs on the pump thread** as a functor
  (`UserFunctionImpl<HWND, WindowWin32::WindowWin32::lambda_1>` → `0x141864fe0`). The creating
  thread blocks on the result. `lpClassName` is an **ATOM** (`(LPCWSTR)(u16)atom`) from
  `RegisterClassW(L"AppWindowClass")` (`0x141867bb0`, which runs on the WinMain thread). `lpParam`
  is the `WindowWin32*`.
- `ShowCursor`, `refreshCursor`, `DropTarget` and `~WindowWin32` are also marshalled to the pump.
  **`SetWindowPos`/`ShowWindow`/`GetWindowLongW` are called directly from game threads**
  (`WindowWin32::vfunc_2/3/5/10`: `SetWindowPos(h, 0, x, y, 0, 0, SWP_NOSIZE|SWP_NOZORDER)`,
  `ShowWindow(h, SW_RESTORE)`). Those are cross-thread operations on a window owned by the pump.
- WndProc (`0x141869700`) handles: `WM_MOUSEMOVE`, `WM_SETFOCUS`/`WM_KILLFOCUS`,
  `WM_ACTIVATEAPP`, key, char and syskey messages (`VK_F12` on keyup, Alt+`VK_F4`), all mouse
  button and wheel messages (`0x201..0x20e`). Everything else goes to
  `SystemEventQueueWindowsDesktop::vfunc_7` (`0x141866d40`), then to `DefWindowProcW`. That
  function handles:
  - `WM_CREATE` (reads `((CREATESTRUCTW*)lParam)->lpCreateParams`) and `WM_DESTROY`.
  - `WM_SIZE` (ignores `SIZE_MINIMIZED`), `WM_PAINT` (just `BeginPaint` + `EndPaint`), `WM_CLOSE`.
  - `WM_SETCURSOR` (only when `LOWORD(lParam)==HTCLIENT`) and `WM_WINDOWPOSCHANGING`, which uses
    `MonitorFromWindow` and `AdjustWindowRectEx` to clamp.
  - `WM_DISPLAYCHANGE`, `WM_SYSCOMMAND` (blocks `SC_SCREENSAVE`/`SC_MONITORPOWER`, handles
    `SC_KEYMENU`, `SC_MINIMIZE`, `SC_RESTORE`), `WM_DEVICECHANGE` and `WM_CAPTURECHANGED`.
  - `WM_ENTERSIZEMOVE`/`WM_EXITSIZEMOVE`, and `WM_DPICHANGED` (scale = `HIWORD(wParam)/96.0`).
  - **`WM_INPUT`**: `GetRawInputData(lParam, RID_INPUT, buf, &cb /*=0x40*/, 0x18)` into a fixed
    64-byte stack buffer, timestamped with QPC (`0x141866590`).
- Raw input registration [V]: mouse `(1,2)` and keyboard `(1,6)` with `dwFlags=0` and
  `hwndTarget=gameHwnd`. Unregistration uses `RIDEV_REMOVE` with `hwndTarget=0`. There are no
  `RIDEV_NOLEGACY` or `RIDEV_INPUTSINK` flags, so legacy `WM_MOUSEMOVE`/`WM_KEY*` still have to
  arrive. The keyboard path saves and restores sticky, toggle and filter keys with
  `SystemParametersInfoA(SPI_SETSTICKYKEYS/…)`. Return TRUE and do nothing.
- **There is no `ClipCursor` import.** `SetCursorPos` is used for "place cursor at normalized
  client position" (`0x1420cbec0`: `GetClientRect` → `ClientToScreen` → `SetCursorPos`) and
  by `WindowWin32::vfunc_14`.
- **The `ShowCursor` return value controls loops**: `while (ShowCursor(TRUE) < 0) {...}`
  (`0x1423f1670`) and its inverse (`0x1423f1620`). Mouse coordinates are normalized by
  `GetClientRect` (`Desktop::vfunc_9`).
- `0x1423f1910` is the **ErrorDialog**: `RegisterClassExW(L"ErrorDialog")`, child `EDIT`/`STATIC`
  controls, its own `GetMessageA` loop, `SendMessageA(WM_SETFONT…)`. It is not on the happy path.
  Map it to an `NSAlert` later. For now, unknown classes fail loudly.
- user32 imports (75) include `QueryDisplayConfig`, `DisplayConfigGetDeviceInfo` (HDR / SDR
  white-level probing), `GetDpiForWindow`, `RegisterDeviceNotificationW`, `GetGUIThreadInfo`,
  `CreateCursor` and `ToUnicode`. dwmapi: `DwmSetWindowAttribute`. gdi32: 5 font/stock-object
  calls. The current stop is `LoadCursorA` @ `0x141867dd4`, inside the class-registration
  function.

### Wwise sink (AkSinkCoreAudio.cpp, Wwise 2022.1.4) [V]
Init chain: `0x141652ce0` → `0x1416538d0` → format pick `0x141653fc0`.
1. `CoCreateInstance(CLSID_MMDeviceEnumerator, …, 0x17, IID_IMMDeviceEnumerator)`, then
   `RegisterEndpointNotificationCallback(self)` (`0x1415ecaf0`).
2. `IMMDevice::OpenPropertyStore(STGM_READ)` → `GetValue(PKEY_AudioEngine_DeviceFormat
   {f19f064d-082c-4e27-bc73-6882a1bb8e4c},0)`. **If `vt == VT_EMPTY` the sink init fails**
   (`goto` the error exit with `AK_Fail`). Wwise reads `nChannels` from `pBlobData+2` (the blob
   pointer is at PROPVARIANT+0x10), so this has to be a `VT_BLOB` holding a WAVEFORMATEX(TENSIBLE).
3. ISpatialAudioClient path (`0x141653340`): only tried for a specific device type with a
   setting enabled. On failure it falls back to plain WASAPI.
4. `Activate(IID_IAudioClient {1cb9ad4c…}, CLSCTX_ALL)`. Failure returns `AK_Fail`.
5. `Activate(IID_IAudioSessionManager2 {77aa99a0…})` → `GetAudioSessionControl(NULL,0,&c)` →
   QI `IAudioSessionControl2` → `SetDuckingPreference(TRUE)`. **Optional**: NULL results are
   tolerated.
6. `GetMixFormat` must return exactly 0 and a non-NULL pointer. Wwise copies the format and frees
   it with `CoTaskMemFree`. It **forces the copy to EXTENSIBLE + `KSDATAFORMAT_SUBTYPE_IEEE_FLOAT`,
   32-bit**. If the wanted channel count differs, it calls `IsFormatSupported(SHARED, fmt,
   &closest)` and accepts 0, or 1 (S_FALSE) with `closest` set.
7. QI `IID_IAudioClient2 {726778cd…}`. If that succeeds:
   `IsOffloadCapable(category, &b)` then **`SetClientProperties({12, b, category})` — if this
   returns <0 the sink fails**. Category is 6 (GameEffects), 7 or 8 depending on the device kind.
8. `GetDevicePeriod(&def, &min)`: requires success and `def >= 1`.
9. Buffer duration: `frame = 512*1e7/48000` (106 667 hns). Start at `2*def` and step by `def`
   until it is ≥ `frame`, then add `frame`. Clamp to `N*frame` (N = a sink setting) and add
   `1.2*def` when a sink flag is clear. With `def = 100000` this comes to about 30–43 ms
   (≈1472–2048 frames).
10. `Initialize(SHARED, flags, dur, 0, fmt, NULL)`. Flags are `EVENTCALLBACK` (0x40000), or
    `0x88040000` (`AUTOCONVERTPCM|SRC_DEFAULT_QUALITY|EVENTCALLBACK`) **when the mix rate ≠ 48000**
    (in that case `fmt` is rewritten to 48 kHz first). **Any result other than exactly 0 fails**
    (in the non-autoconvert branch), so return `S_OK`, never an `AUDCLNT_S_*` value. If the
    autoconvert Initialize fails, Wwise retries at the device rate and resamples itself
    (`FUN_141643200`).
11. `SetEventHandle(globalEvent)` must return 0. `GetBufferSize(&frames)` must return 0.
12. `GetService(IID_IAudioRenderClient {f294acfc…})` → **`GetBuffer(bufferFrames)` then
    `ReleaseBuffer(bufferFrames, AUDCLNT_BUFFERFLAGS_SILENT)`**, which prefills the whole buffer
    before `Start`.
13. `Start()` must be ≥ 0. Rendering then happens on Wwise's own audio thread, which waits on the
    event and writes 512-frame chunks while `bufferFrames - padding >= 512`.

---

## TRACK 1 — Win32 windowing on AppKit

### 1.1 Threading: use option (a), "Cocoa owns the main thread, Win32 owns its queues"

**Option (b) is wrong for this game.** The window-owning thread is the game's own
`MessagePumpThread`, not WinMain. Moving WinMain to the main thread therefore does nothing for
window ownership. It does cost you three things:
- The main thread blocks for seconds during boot (asset and shader loads). macOS shows a
  beachball and marks the app "not responding".
- The main thread's stack is fixed at 8 MB instead of the PE `stack_reserve`.
- AppKit and the game's SEH and TEB would share one stack.

**Recommendation: option (a).** This is also how Wine's macOS driver is built [A, from
winemac.drv]. The real main thread runs Cocoa (`macdrv_start_cocoa_app`) and Win32 runs on
secondary threads. Every Win32 thread that owns windows has an event queue that the Cocoa thread
posts into. Win32 threads drain it inside their message waits and turn the events into
synchronous wndproc calls. Win32 → Cocoa calls use `OnMainThread` (a synchronous perform on a
custom common-modes run-loop source). The few cases where Cocoa needs a synchronous answer
("queries", such as min/max during live resize) use timeouts.

```
 main thread (host, no TEB)                      Win32 threads (TEB, guest code)
 ─────────────────────────────                    ─────────────────────────────────
 NSApplication run loop                           MessagePumpThread: GetMessageW()
  ├─ sendEvent: / delegates                         waits on ITS ThreadQueue (Mutex+Condvar)
  │    translate NSEvent ──post(HostEvent)──────►   HostEvent → wndproc (as *sent* msg)
  │    (never blocks on a Win32 thread)             posted msgs → returned to caller
  ├─ main_sync(|| ...) blocks ◄──────────────────  CreateWindowExW / SetWindowPos / ShowCursor
  │    run AppKit mutation, return                  (Win32 side updates its own state first)
  └─ GCMouse handlers on a private serial queue ─► post WM_INPUT directly (bypass main)
```

Rules that make this deadlock-free:
1. **The main thread never waits on a Win32 thread.** No `SendMessage` from Cocoa. Window
   frame changes, focus, close requests, backing-scale changes and occlusion become `HostEvent`s
   posted to the owner thread's queue. The owner converts them into `WM_WINDOWPOSCHANGING/
   CHANGED`, `WM_SIZE`, `WM_ACTIVATEAPP`, `WM_SETFOCUS` and so on, and calls the wndproc
   directly. These events are *not* returned from `GetMessage`, exactly like Windows sent
   messages.
2. **Win32 threads may block on main** with `main_sync`. Implement it with
   `CFRunLoopPerformBlock(mainRL, kCFRunLoopCommonModes, …)` plus `CFRunLoopWakeUp`, or a custom
   source. Don't use `dispatch_sync(main_queue)` blindly, because you need it serviced in
   modal and event-tracking modes too [A: the main GCD queue is serviced in common modes, but a
   custom source gives you control]. This is safe because of rule 1.
3. **Win32 is the source of truth for Win32-visible state.** `SetWindowPos` updates the Rust-side
   window rect immediately, sends the WM_ messages to the owner, then pushes the NSWindow change
   to main asynchronously. `GetClientRect` never asks AppKit. When AppKit later reports the frame
   it actually applied (for example, constrained by the screen), it posts a `FrameChanged` event
   carrying a sequence number. The owner thread drops stale ones (winemac does the same) to avoid
   resize feedback loops.
4. **Cross-thread `SendMessage`, and the implicit sends inside `SetWindowPos`/`ShowWindow`/
   `DestroyWindow`/`SetFocus` from non-owner threads**, enqueue a `SentMsg{reply: Arc<Slot>}` on
   the owner's queue and block. While blocked, the sender keeps servicing *its own* incoming
   sent messages (Windows semantics; prevents A↔B deadlock). The owner services sent messages
   inside `GetMessage`, `PeekMessage` and its own `SendMessage` waits. `WaitForSingleObject` does
   **not** service them (also Windows semantics). `SWP_ASYNCWINDOWPOS` becomes a post.
5. **No AppKit, CoreGraphics or CoreFoundation calls from Win32 threads.** Under our TEB scheme,
   TSD slots 11 and 12 hold the TEB TLS array and PEB. A framework that internally calls
   `localtime()`/`ttyname()`, which use those keys, would silently corrupt the guest's TLS
   pointer. AppKit is also main-thread-only. Route everything through `main_sync`/`main_async`
   or dedicated host threads. (This rule also applies to Track 2.)
6. **Callbacks into guest code only run on threads with a TEB** (Win32 threads). Never call a
   wndproc or COM callback from main, from GCD queues or from CoreAudio threads.

Loader changes:
- `real_main` currently spawns the initial Win32 thread and polls `exit_code()` every 50 ms.
  Replace the poll with
  `NSApplication.sharedApplication; setActivationPolicy(.regular); finishLaunching; run`.
  Thread exit becomes a `main_async(terminate)`.
- Make `exit_process` from any thread safe while AppKit runs. Flush logs and then `_exit`,
  rather than running `exit()` atexit handlers concurrently with AppKit.
- Embed an Info.plist into the loader with
  `-sectcreate __TEXT __info_plist` containing `CFBundleIdentifier`, `NSHighResolutionCapable=YES`
  and `LSApplicationCategoryType=public.app-category.games` (Game Mode only triggers for
  that category). Without a plist and a regular activation policy, an unbundled binary
  launched from Terminal never becomes key, so it gets no keyboard and the window stays behind
  the terminal.
- Call `activateIgnoringOtherApps` once at the first `ShowWindow`.

**Rosetta notes.** AppKit, GameController and Metal run fine in a translated x86_64 process, and
the msgSend overhead is irrelevant at UI rates. The real risks are the TSD-slot aliasing above,
and code that runs both host and guest on one thread. Keep the pump thread purely guest: it
waits on a Rust condvar, not on a CFRunLoop.

**Input latency.** One extra hop (main → pump condvar) costs microseconds. To keep mouselook
fast, raw mouse deltas skip the main thread (§1.2).

### 1.2 Message semantics that are easy to get wrong

**Queue model (per Win32 thread):**
- Each queue holds: `sent: VecDeque<SentMsg>`, `posted: VecDeque<MSG>`,
  `input: VecDeque<MSG>` (hardware), `quit: Option<i32>`, per-window invalid flags, timers and
  `wake: Condvar`.
- Create queues **eagerly for every Win32 thread**. On real Windows, `PostThreadMessage` to a
  thread without a queue fails with `ERROR_INVALID_THREAD_ID` (1444). The pump calls
  `PeekMessage` first, so it would be fine either way, but eager creation removes a startup race.
  Record it as a known deviation.
- **Retrieval order in `GetMessage`/`PeekMessage`:**
  1. Process *all* sent messages; they are never returned.
  2. Posted messages.
  3. Input messages.
  4. `WM_QUIT`, once the posted queue is empty.
  5. `WM_PAINT`, synthesized when the window has an invalid region.
  6. `WM_TIMER`.

  `PeekMessage` **always** returns `WM_QUIT` regardless of the filter. `GetMessage` returns 0
  for `WM_QUIT` and -1 on error. Filtering: `hwnd=NULL` means any window of the thread plus
  thread messages, `hwnd=-1` means thread messages only, and the min/max range filters
  `message`. `PM_NOREMOVE`, `PM_REMOVE` and `PM_NOYIELD` must work. `PM_QS_*` flags can be
  ignored at first.
- **`WM_PAINT`: `BeginPaint` must validate the region**, and so must
  `DefWindowProc(WM_PAINT)`. Otherwise `GetMessage` returns `WM_PAINT` forever and the pump
  never sleeps, which starves the `0x8000` functor wakeups. Invalidate on show, resize and
  un-minimize only. Keep `WM_ERASEBKGND` and `WM_NCPAINT` minimal.
- **`CreateWindowExW` order:**
  1. `WM_GETMINMAXINFO`, `WM_NCCREATE` (returning FALSE aborts creation), `WM_NCCALCSIZE`.
  2. **`WM_CREATE` with `lParam = &CREATESTRUCTW`**, where `lpCreateParams` is the first field.
     The game reads it. Returning -1 aborts.
  3. Only if `WS_VISIBLE` is set: `WM_SHOWWINDOW`, `WM_WINDOWPOSCHANGING`, activation
     (`WM_ACTIVATEAPP`, `WM_NCACTIVATE`, `WM_ACTIVATE`, `WM_SETFOCUS`), `WM_WINDOWPOSCHANGED`,
     `WM_SIZE`, `WM_MOVE`.

  All of these are synchronous, on the creating (pump) thread, before `CreateWindowExW`
  returns. Messages before `WM_CREATE` hit a wndproc that doesn't know the HWND yet; that has to
  be harmless (it falls through to `DefWindowProcW`). **`lpClassName` may be an ATOM (≤ 0xFFFF):
  don't dereference it.** The HWND must be valid inside `WM_NCCREATE`.
- **`WM_SIZE` and `WM_MOVE` come from `DefWindowProcW(WM_WINDOWPOSCHANGED)`**, not from
  `SetWindowPos` directly (Wine does it this way). The game's `WM_WINDOWPOSCHANGING` handler
  clamps the `WINDOWPOS` in place, so read the struct back after the call.
- **`SetWindowLong(GWL_STYLE)`** changes nothing visible until
  `SetWindowPos(SWP_FRAMECHANGED)`. `GetWindowLongW(GWL_STYLE)` is used for
  maximized/child tests. Keep style and exstyle exact. `SetWindowLongPtrW(GWLP_WNDPROC)` +
  `CallWindowProcW` (subclassing at `0x14152d3e0`) must return the previous proc and chain.
- **`DispatchMessageW`** on a thread message (hwnd NULL) is a no-op. On a window message it calls
  the class or subclass proc with the message's hwnd.
- **`TranslateMessage`** posts `WM_CHAR`/`WM_SYSCHAR` for `WM_KEYDOWN`/`WM_SYSKEYDOWN`. Don't
  reimplement layouts: attach the `NSEvent.characters` of the originating keyDown to the queued
  `MSG` in a side table keyed by a serial in `MSG.time`/an internal field, and emit those
  UTF-16 units. Handle dead keys and IME later with `interpretKeyEvents`.
- **`WM_KEYDOWN` lParam bits:** repeat count (0–15), scancode (16–23), extended (24), alt
  context (29), **previous state (30, games use it to detect auto-repeat)** and transition (31).
  Alt combos and F10 arrive as `WM_SYSKEYDOWN`. **Don't map Cmd to `VK_LWIN`**: let Cmd+Tab, Cmd+Q
  and Cmd+H reach macOS. Map `applicationShouldTerminate` → `NSTerminateCancel` + post `WM_CLOSE`
  to the game window (it handles `WM_CLOSE` itself). `windowShouldClose:` returns NO and posts
  `WM_CLOSE`. Only Win32 `DestroyWindow` closes the NSWindow.
- **Key state:** `GetKeyState` and `GetKeyboardState` should reflect the state as of the last
  message *retrieved* by the thread, and `GetAsyncKeyState` the live state. A single global
  state is fine for M2, but update it at retrieval rather than at NSEvent arrival if you see
  ordering bugs. `MapVirtualKeyA` and `ToUnicode` need a mac-keycode ↔ scancode ↔ VK table (US
  layout first). Generate it once from the Windows Set-1 scancodes; the same table feeds
  `RAWKEYBOARD.MakeCode`.
- **Activation:** `windowDidBecomeKey`/`ResignKey` and `applicationDidBecome`/`ResignActive`
  post host events that become `WM_ACTIVATEAPP`, `WM_NCACTIVATE`, `WM_ACTIVATE` and
  `WM_SETFOCUS`/`WM_KILLFOCUS`. **On deactivate, leave relative-mouse mode and restore the cursor**
  so that Cmd+Tab out works even if the game doesn't react.

**Raw input (mouselook):**
- **Source.** Use `GCMouse.current.mouseInput.mouseMovedHandler` (GameController, macOS 11+)
  [A]: it delivers unaccelerated HID deltas like Windows raw input. Set
  `GCMouse.handlerQueue` to a **private serial queue**, not main, so look input doesn't wait
  behind AppKit work. The fallback is `NSEvent` `deltaX`/`deltaY` with
  `NSEvent.mouseCoalescingEnabled = NO`. Those deltas are accelerated and display-coalesced
  otherwise. Buttons and wheel can come from NSEvent.
- **Delivery.** If a thread registered usage `(1,2)` with `hwndTarget` set and dwFlags 0, post
  `WM_INPUT(wParam=RIM_INPUT, lParam=HRAWINPUT)` to the target's owner queue **only while the
  target is foreground** (no `RIDEV_INPUTSINK`). `HRAWINPUT` is a handle into a per-thread ring of
  `RAWINPUT` records, e.g. 4096 entries with a handle of `0xR000_0000 | seq`. Stale handles make
  `GetRawInputData` fail with -1. On queue overflow, drop the oldest record; don't grow
  unboundedly.
- **`GetRawInputData` exact behavior:**
  - `cbSizeHeader` must equal 24, otherwise -1.
  - `pData == NULL` → write the required size and return 0.
  - `*pcbSize < needed` → `ERROR_INSUFFICIENT_BUFFER` and -1.
  - Otherwise copy and return the bytes copied.

  Sizes: x64 `RAWINPUTHEADER` = 24 bytes; `RAWINPUT` for a mouse = 48, keyboard = 40. The game
  passes 64. `RAWMOUSE`: `usFlags = MOUSE_MOVE_RELATIVE (0)`, `lLastX`/`lLastY` as integer
  counts. Accumulate the fractional GC deltas and emit the integer part. Buttons go in
  `usButtonFlags`, wheel in `usButtonData` (signed, 120 per notch, `RI_MOUSE_WHEEL` 0x400).
  `hDevice` is a fake non-null handle.
- **Keep legacy messages flowing.** Without `RIDEV_NOLEGACY` the game still expects
  `WM_MOUSEMOVE`/`WM_KEY*` alongside `WM_INPUT`.

**Cursor:**
- **`ShowCursor`:** keep a real counter (per thread is the Windows semantics; the game funnels
  all calls through the pump anyway) and return the new value. Hide when it goes below 0 and
  show at ≥ 0. Call `NSCursor hide`/`unhide` **only on transitions**, on main: AppKit counts
  these too, and an unbalanced pair leaves the cursor stuck.
- **`SetCursor`:** remember the HCURSOR per window and apply it on main
  (`NSCursor set` / cursor rects). `LoadCursorA(NULL, IDC_*)` returns stable fake handles mapped
  to `NSCursor` system cursors. `CreateCursor(AND/XOR bits)` builds an `NSImage` cursor; an
  all-transparent one is fine. `LoadIconA(hInst, "MAINICON")` can return a fake handle.
- **Relative mode:** enter it when the cursor count is below 0, the game window is key, and raw
  mouse is registered:
  - `CGAssociateMouseAndMouseCursorPosition(false)` and hide the cursor. Deltas keep flowing via
    GCMouse.
  - `SetCursorPos` updates a *virtual* cursor position (returned by `GetCursorPos`) and posts a
    synthetic `WM_MOUSEMOVE`, as Windows does. **Don't warp physically** while in relative mode.
  - Outside relative mode, `SetCursorPos` is
    `CGWarpMouseCursorPosition` (on main, coordinates converted to global points), followed once
    at startup by `CGSetLocalEventsSuppressionInterval(0)`. Otherwise every warp causes about
    250 ms of dropped mouse events.
  - Ignore NSEvent mouseMoved events that predate the warp; Wine tracks warp records for this
    [A].
  - Avoid CGEventTaps for cursor clipping: they need Input Monitoring or Accessibility
    permission.
- **Capture:** `SetCapture`/`ReleaseCapture` track the capture HWND. While captured, deliver
  `WM_MOUSEMOVE` with client coordinates outside the window (sign-extended shorts). Send
  `WM_CAPTURECHANGED` to the old capture window.

**Coordinates, DPI and Retina (pick one model and apply it everywhere):**
- **Win32 coordinates are backing pixels.** Window, client, monitor rects,
  `GetSystemMetrics(SM_CXSCREEN)`, mouse lParams, `GetCursorPos`/`SetCursorPos`, DXGI output
  `DesktopCoordinates` and mode lists all use pixels. The game is a renderer: it sizes the swap
  chain from `GetClientRect`. Point-sized rects would render at a quarter of the pixel count on
  Retina, and the game already has its own render-scale, DLSS and FSR. Conversion:
  `px = pt * backingScaleFactor`. Flip Y against the **primary** screen's height
  (Cocoa's origin is bottom-left of the primary screen; Win32's is top-left, y down).
- **Report 96 DPI** (`GetDpiForWindow`, `GetDpiForMonitor`, and `LOGPIXELSX` if asked): "a
  Retina panel is a 100%-scaled monitor of its pixel size". That keeps every number consistent
  for a per-monitor-aware app, and the only `WM_DPICHANGED` consumer just rescales by `dpi/96`.
  Add `AW2_DPI=` to experiment. Wine's RetinaMode is the precedent for pixel coordinates [A].
  Accept `SetProcessDpiAwarenessContext` and `SetThreadDpiAwarenessContext` if they show up via
  `GetProcAddress`.
- **Existing bug:** `user32::GetSystemMetrics` uses `CGDisplayPixelsWide`, which returns the
  *mode's point size* on HiDPI modes (e.g. 1512×982 on a 14" MBP). Use
  `CGDisplayModeGetPixelWidth/Height(CGDisplayCopyDisplayMode(d))`, or `NSScreen.frame *
  backingScaleFactor` from main, cached and refreshed on screen-change notifications.
- **Monitors:** one `HMONITOR` per `NSScreen` (stable fake handles keyed by `CGDirectDisplayID`),
  with `MONITORINFOEXA.szDevice = "\\\\.\\DISPLAY1"…`. **DXGI must report the same `HMONITOR`
  and `DeviceName`** in `DXGI_OUTPUT_DESC`; engines correlate the two.
  `QueryDisplayConfig`/`DisplayConfigGetDeviceInfo` can honestly return `ERROR_NOT_SUPPORTED`
  for M2. Watch whether the game then disables HDR or bails. Later, expose one path with
  advanced-color info from `NSScreen.maximumPotentialExtendedDynamicRangeColorComponentValue`.
- **Non-client area:** `AdjustWindowRectEx` must agree with the NSWindow you create:
  - `WS_POPUP` (borderless): no frame, window rect = client rect.
  - `WS_CAPTION`: add the macOS title-bar height in pixels (`frameRectForContentRect` delta ×
    scale) on top and 0 elsewhere.
  - Only make the NSWindow resizable when `WS_THICKFRAME` is set. For M2 answer live-resize
    constraints from the cached `WM_GETMINMAXINFO`, and don't query the game synchronously.
- **Fullscreen:**
  - A monitor-sized `WS_POPUP` window means borderless fullscreen. Use a borderless NSWindow at
    `NSScreen.frame`, set `presentationOptions = autoHideMenuBar|autoHideDock` while it is key,
    and use the normal window level.
  - **Never use `toggleFullScreen:`.** It uses Spaces, animates asynchronously, and produces a
    burst of `WM_SIZE` messages with intermediate sizes.
  - On notched MacBooks, decide between covering the full frame and starting below
    `safeAreaInsets.top`. Default to below the notch and add an env override.
  - DXGI "exclusive" fullscreen (`SetFullscreenState(TRUE)`) is the same borderless path. Never
    change the display mode; scale through the layer.

### 1.3 HWND → NSView → CAMetalLayer for the swap chain

- Each top-level HWND gets one `NSWindow` whose `contentView` is an `AW2View`. Its
  `makeBackingLayer` returns a `CAMetalLayer`, with `wantsLayer = YES`, `contentsScale =
  backingScaleFactor` (update it in `viewDidChangeBackingProperties`) and
  `layerContentsRedrawPolicy = NSViewLayerContentsRedrawDuringViewResize`/`Never`. Creation
  happens on main inside the `CreateWindowExW` `main_sync`.
- Shared, lock-free window info that DXGI reads from any thread:
  ```rust
  pub struct WindowShared {
      pub hwnd: HWND,
      pub owner_tid: u32,
      pub layer: *mut CAMetalLayer,     // CFRetain'ed once; released when the last user drops
      pub client_px: AtomicU64,         // (w<<32)|h, Win32-side truth
      pub scale_x1000: AtomicU32,
      pub occluded: AtomicBool,         // from windowDidChangeOcclusionState / miniaturize
      pub generation: AtomicU32,        // bumps on resize/screen change
  }
  pub fn window_shared(hwnd: HWND) -> Option<Arc<WindowShared>>;
  ```
  The swap chain holds `Arc<WindowShared>` plus its own CFRetain on the layer, so a
  `DestroyWindow` during a present can't free it. Afterwards it presents to a detached layer,
  which is harmless.
- **Present from the render thread**: `layer.nextDrawable` → blit → `presentDrawable` →
  `commit`. This is supported off-main; UE's Metal RHI and MoltenVK do it [A].
  - The window side owns geometry (frame and contentsScale on main). The swap chain owns
    `drawableSize`, `pixelFormat`, `colorspace`, `displaySyncEnabled` and `maximumDrawableCount`.
    It sets them at creation and in `ResizeBuffers`, wrapped in
    `CATransaction begin/setDisableActions(YES)/commit` so off-main changes commit promptly.
  - `ResizeBuffers(0,0)` reads `client_px`, never AppKit.
- **Back buffers are swap-chain-owned `MTLTexture`s**, not drawable textures. D3D12 back buffers
  are persistent resources with fixed indices (`GetCurrentBackBufferIndex`), while drawables are
  transient. Present blits or converts into the drawable texture. That needs
  `framebufferOnly = NO`, or use a render pass.
  - CAMetalLayer supports BGRA8 (+sRGB), RGBA16Float, RGB10A2/BGR10A2 and BGRA10_XR, **not
    RGBA8**. `DXGI_FORMAT_R8G8B8A8_UNORM` needs a swizzle in the blit.
  - HDR: `RGBA16Float`, `wantsExtendedDynamicRangeContent`, and an extended-linear or PQ
    colorspace.
- **`SyncInterval` → `displaySyncEnabled`.** Set `allowsNextDrawableTimeout = YES`. A `nil`
  drawable, or an occluded/minimized window, makes Present return `DXGI_STATUS_OCCLUDED`
  (0x087A0001) without blocking. Otherwise the render thread can stall up to 1 s per frame while
  the window is hidden.
- During a live resize, drawable size and bounds may disagree for a frame. Keep
  `contentsGravity = resize` and `presentsWithTransaction = NO`.

### Track 1 — top 10 pitfalls
1. **Treating the WinMain thread as the window thread.** The window lives on the game's
   `MessagePumpThread`, and game threads call `SetWindowPos`/`ShowWindow` on it cross-thread.
2. **The main thread blocking on a Win32 thread** (a synchronous WM_SIZE/WM_CLOSE from a Cocoa
   delegate) while that thread is in `main_sync`. This is a guaranteed deadlock. Cocoa → Win32
   is post-only.
3. **Calling AppKit, CG or CF from Win32 threads.** That breaks main-thread affinity, and any
   framework path that touches libc's `localtime`/`ttyname` TSD keys corrupts TSD slots 11/12
   (our TEB TLS array and PEB).
4. **`BeginPaint` not validating**, which produces an endless `WM_PAINT` busy loop and starves the
   pump's 0x8000 functors.
5. **Dereferencing an ATOM `lpClassName`**, or not passing `CREATESTRUCTW*` in `WM_CREATE`
   lParam. The game reads `lpCreateParams` there.
6. **A stub `ShowCursor` returning a constant.** The game loops `while (ShowCursor(TRUE) < 0)`.
   Unbalanced `NSCursor hide`/`unhide` leaves the cursor stuck.
7. **Mixed units**: points somewhere, pixels elsewhere (current `GetSystemMetrics`). Mouse is
   normalized by `GetClientRect`, so mixing units breaks aiming and UI hit-testing. Remember the
   Y flip.
8. **Physical warps in relative mode, or warps without zeroing the local-event suppression
   interval** (≈250 ms of dead mouse). Not releasing relative mode on deactivate traps the user.
9. **`GetRawInputData` contract errors**: wrong header size check, writing more than `*pcbSize`
   (the game passes 64), no size query with NULL, or `HRAWINPUT` handles that are never retired.
10. **`toggleFullScreen:`, or delivering intermediate AppKit sizes as `WM_SIZE`.** Also: no
    Info.plist or activation policy, so the window never becomes key and gets no keyboard.

---

## TRACK 2 — WASAPI on CoreAudio

### 2.1 Design

**Use AUHAL (`kAudioUnitSubType_DefaultOutput`) with a render callback.**
- Compared with `AudioDeviceIOProc`, AUHAL gives sample-rate and format conversion (our fixed
  48 kHz float interleaved → whatever the device runs: 44.1 kHz, 24 kHz AirPods with the mic
  active, multi-stream interfaces) and channel mapping. DefaultOutput also follows the system
  default device on its own. A raw IOProc makes you handle stream layouts, SRC and device
  switching yourself, with no benefit at about 30 ms of latency.
- AVAudioEngine adds Obj-C object graphs and latency with no gain here.
- This matches the plan in `docs/AUDIO.md` §7.11.

```
Wwise audio thread (Win32, TEB)        bridge thread (Win32-spawned, TEB)    CoreAudio IO thread (host, RT)
───────────────────────────────        ──────────────────────────────────    ──────────────────────────────
WaitForSingleObject(evt)  ◄──SetEvent── loop { semaphore_timedwait(sem,20ms)   render_cb(n):
GetCurrentPadding = w - r                  if running: event.set() }            read ≤ n frames from ring
GetBuffer(k)  → scratch                                                         zero-fill shortfall
ReleaseBuffer(k,f) → copy into ring,                                            r += consumed (atomic)
   tap.write(), w += k (Release)                                                semaphore_signal(sem)  ← RT-safe
```

Details:
- **The ring *is* the WASAPI buffer.** Capacity = `bufferFrames` (the `GetBufferSize` result).
  `padding = w - r`, read with acquire ordering. `GetBuffer(k)` fails with
  `AUDCLNT_E_BUFFER_TOO_LARGE` if `k > bufferFrames - padding`, and with
  `AUDCLNT_E_OUT_OF_ORDER` if a buffer is already outstanding. It returns a pointer to a linear
  **scratch** buffer of `bufferFrames` frames.
- `ReleaseBuffer(k, flags)`: `k > requested` gives `AUDCLNT_E_INVALID_SIZE`; `k == 0` is allowed.
  Copy the scratch into the ring with wrap handling, **writing zeros if
  `AUDCLNT_BUFFERFLAGS_SILENT`** (the content is undefined in that case; Wwise's prefill relies on
  this). Then the tap writes, then `w += k`. Copying makes ring wrap trivial and is the natural
  tap point; the cost is noise. A mirrored ring via `vm_remap` would remove the copy, but you
  don't need it.
- **The IO callback never signals the Win32 event directly.** `SetEvent` goes through
  `handles::WAIT_LOCK`, a *global* `std::sync::Mutex`, and `notify_all`. Taking it on the
  real-time thread risks priority inversion behind any game thread. Under Rosetta, a host IO
  thread has no TEB, so any `set_last_error` path writes into libc TSD. Instead the IO callback
  does a mach `semaphore_signal` (RT-safe), and a bridge thread spawned with
  `aw2_win32::thread::spawn` (so it has a TEB) sets the event through the internal Rust event
  API. The 20 ms timed wait keeps events flowing if the device stalls. Wine likewise signals
  from a period timer, not the AU callback [A].
- **Never run guest code on CoreAudio threads.** No Wwise render callback pulled from the IO
  proc, and no `IMMNotificationClient` callbacks from HAL property listeners. If you add device
  notifications, run them on a dedicated Win32-spawned "mmdevapi" thread.
- **`GetDevicePeriod`: report the Windows values, default 100000 and minimum 30000 (10 ms /
  3 ms).** Wwise's buffer arithmetic was tuned on these (§0, step 9). Don't report the CoreAudio
  IO size.
- **Buffer size and the IO slice:**
  - `GetBufferSize` may legally return more than requested. Return
    `max(ceil(dur*48000/1e7), 2*maxSlice + 512)`.
  - Set `kAudioUnitProperty_MaximumFramesPerSlice = 4096`. CoreAudio raises the slice to 4096
    when the display sleeps or the system is under power pressure. A 1472-frame ring would then
    underrun on every callback.
  - Try to set `kAudioDevicePropertyBufferFrameSize` to 512 (it is per-process-minimum, so this
    is harmless) and read back the actual value.
  - Log requested vs granted sizes.
- **`GetMixFormat`: return WAVEFORMATEXTENSIBLE, 48000 Hz, 32-bit float, 2 channels,
  `KSAUDIO_SPEAKER_STEREO` (0x3), `cbSize = 22`, `wValidBitsPerSample = 32`**, allocated with
  `CoTaskMemAlloc`. Always 48 kHz, whatever the device rate: AUHAL resamples. This keeps Wwise on
  its happy path: no `AUTOCONVERTPCM` and no internal SRC (`FUN_141643200`).
  `PKEY_AudioEngine_DeviceFormat` (`VT_BLOB`) must describe the **same** format.
  - Multichannel later: channels from `kAudioDevicePropertyPreferredChannelLayout`, capped at 8,
    with standard masks (6 → 0x60F/0x3F, 8 → 0x63F).
  - Still accept `AUTOCONVERTPCM|SRC_DEFAULT_QUALITY` in `Initialize` and treat it as a no-op
    when the client format is 48 kHz float. Wwise sends it if your mix rate is ever ≠ 48000.
- **`IsFormatSupported(SHARED)`:**
  - Exact match → `S_OK`, `*closest = NULL`.
  - Same rate and float with a different channel count → `S_FALSE` + a `CoTaskMemAlloc`'d mix
    format.
  - Otherwise `AUDCLNT_E_UNSUPPORTED_FORMAT`.
  - `EXCLUSIVE` → `AUDCLNT_E_UNSUPPORTED_FORMAT`.
- **Clock semantics:**
  - Before `Start`, the prefilled frames sit in the ring and `padding == bufferFrames`.
  - `Stop` pauses consumption and keeps the padding. `Reset` (only when stopped) sets
    `r = w = 0`.
  - `IAudioClock` (if requested): position = frames consumed `r`. In shared mode, `GetFrequency`
    returns *bytes per second* (Wine returns `rate * blockAlign` [A]).
  - Expose `IAudioClock` only if Wwise asks for it (log `GetService` IIDs); otherwise return
    `E_NOINTERFACE`.
  - `GetStreamLatency`: return `def` (100000).
- **Device change:** with DefaultOutput and a fixed 48 kHz stereo client format, a default-device
  switch is transparent, so A1 sends **no** notifications.
  `RegisterEndpointNotificationCallback` must return `S_OK` and AddRef/store the pointer; release
  it on unregister.
  - If the device dies (`kAudioDevicePropertyDeviceIsAlive` false / AU errors), make subsequent
    `GetCurrentPadding`/`GetBuffer` return `AUDCLNT_E_DEVICE_INVALIDATED` (0x88890004). Wwise
    then rebuilds the sink.
  - Later, wire `OnDefaultDeviceChanged(eRender, eConsole, id)` via the mmdevapi thread.
- **Thread priority:** Wwise sets audio thread priorities through `SetThreadPriority`, which is
  currently only recorded. Under shader-compile storms on Rosetta, map
  `THREAD_PRIORITY_TIME_CRITICAL`/`HIGHEST` to `QOS_CLASS_USER_INTERACTIVE` (via
  `pthread_set_qos_class_self_np` on that thread at the next wait or on creation) so the 30 ms
  buffer doesn't underrun.
- **All CoreAudio API calls** (AU setup, property listeners, start/stop) run on one host
  "coreaudio control" thread or on main, never on Win32 threads (TSD-slot rule, Track 1 §1.1
  rule 5). Win32-side COM methods post to it and wait. `Start`/`Stop` are rare, so the latency
  doesn't matter.

### 2.2 COM ABI pitfalls in Rust

- **Vtables include IUnknown and exact inheritance order.** Each is a `#[repr(C)]` struct of
  `unsafe extern "win64" fn` pointers; the object's first field is `*const Vtbl`. Offsets checked
  against the decompile:
  - `IMMDeviceEnumerator`: QI, AddRef, Release, `EnumAudioEndpoints` (0x18),
    `GetDefaultAudioEndpoint` (0x20), `GetDevice` (0x28),
    `RegisterEndpointNotificationCallback` (0x30), `Unregister…` (0x38).
  - `IMMDevice`: …, `Activate` (0x18), `OpenPropertyStore` (0x20), `GetId` (0x28),
    `GetState` (0x30).
  - `IPropertyStore`: …, `GetCount`, `GetAt`, `GetValue` (0x28), `SetValue`, `Commit`.
  - `IAudioClient`: …, `Initialize` (0x18), `GetBufferSize` (0x20), `GetStreamLatency`,
    `GetCurrentPadding` (0x30), `IsFormatSupported` (0x38), `GetMixFormat` (0x40),
    `GetDevicePeriod` (0x48), `Start` (0x50), `Stop`, `Reset`, `SetEventHandle` (0x68),
    `GetService` (0x70).
  - `IAudioClient2` appends `IsOffloadCapable` (0x78), `SetClientProperties` (0x80) and
    `GetBufferSizeLimits` (0x88).
  - `IAudioRenderClient`: …, `GetBuffer` (0x18), `ReleaseBuffer` (0x20).
  - `IAudioSessionControl2::SetDuckingPreference` is at 0x80.
  - Add `const` offset asserts (`offset_of!`) for every slot.
- **Multiple interfaces on one object** (e.g. `IMMDevice` + `IMMEndpoint`) need a second vtable
  pointer field and `this` adjustment in its thunks.
- **QueryInterface:**
  - Answer `IID_IUnknown` with the canonical pointer (identity rule) and AddRef what you return.
  - On failure set `*ppv = NULL` and return `E_NOINTERFACE` (0x80004002). A NULL `ppv` gets
    `E_POINTER` (0x80004003).
  - Implement `IAudioClient2`, and make `SetClientProperties` return `S_OK` (Wwise fails the
    sink on <0). `IsOffloadCapable` writes FALSE and returns `S_OK`.
  - Answer `IAudioClient3` with `E_NOINTERFACE`. Answering it would bring in the
    shared-engine-period API.
- **Refcounts:** `AtomicU32`; `AddRef`/`Release` return the **new count as `u32`**, not an
  HRESULT. Releases come from Wwise's audio thread and from its init thread. The final Release
  must stop the AU (on the control thread) and join the bridge thread **before** freeing the
  ring. `Release` on a Win32 thread is fine.
- **HRESULTs Wwise checks with `== 0`:** `Initialize`, `SetEventHandle`, `GetBufferSize`,
  `GetMixFormat`, `GetService` and the property-store reads. Return exactly `S_OK`, never
  `S_FALSE` or `AUDCLNT_S_*`. Error codes:

  | Code | Value |
  |---|---|
  | `AUDCLNT_E_NOT_INITIALIZED` | 0x88890001 |
  | `ALREADY_INITIALIZED` | 0x88890002 |
  | `DEVICE_INVALIDATED` | 0x88890004 |
  | `NOT_STOPPED` | 0x88890005 (Start twice, Reset while running) |
  | `BUFFER_TOO_LARGE` | 0x88890006 |
  | `OUT_OF_ORDER` | 0x88890007 |
  | `UNSUPPORTED_FORMAT` | 0x88890008 |
  | `INVALID_SIZE` | 0x88890009 |
  | `EVENTHANDLE_NOT_EXPECTED` | 0x88890011 |
  | `EVENTHANDLE_NOT_SET` | 0x88890014 (Start with EVENTCALLBACK but no handle) |
  | `AUDCLNT_S_BUFFER_EMPTY` | 0x08890001 |

  Double-check these against `third_party/` or the Windows SDK headers before hardcoding them.
- **`ISpatialAudioClient`** (`{BBF8E066-AAAA-49BE-9A4D-FD2A858EA27F}`) and
  **`IAudioSessionManager2`** via `IMMDevice::Activate` → `E_NOINTERFACE` with `*ppv = NULL`.
  Wwise falls back or tolerates it [V]. Log each unknown IID with `stub!`.
- **PROPVARIANT is 24 bytes:** `vt: u16`, 3×`u16` reserved, union at +8.
  - `VT_LPWSTR` (31): pointer at +8.
  - **`VT_BLOB` (65): `cbSize: u32` at +8, `pBlobData` at +0x10** (Wwise reads +0x10).
  - All payloads are `CoTaskMemAlloc`'d by the callee and freed by the caller via
    `PropVariantClear`. **`ole32::PropVariantClear` currently leaks `VT_BLOB`** (`stub!` arm),
    and treats 0..=25 as plain even though `VT_BSTR`/`VT_UNKNOWN`/`VT_DISPATCH` aren't. Fix the
    BLOB arm: free `*(pv+0x10)`, not +8.
  - Properties to serve:
    - `PKEY_AudioEngine_DeviceFormat` (BLOB, mandatory).
    - `PKEY_Device_FriendlyName` `{a45c254e-df1c-4efd-8020-67d146a850e0},14` (LPWSTR).
    - `PKEY_AudioEndpoint_FormFactor` `{1da5d803-d492-4edd-8c23-e0c0ffee7f0e},0` (UI4:
      Speakers=1 / Headphones=3).
    - `PKEY_AudioEndpoint_GUID` (LPWSTR).
  - Anything else: `S_OK` with `VT_EMPTY`. That is what Windows returns for a missing key.
- **WAVEFORMATEX is packed** (`pshpack1`): 18 bytes, with `cbSize` at +16.
  WAVEFORMATEXTENSIBLE is 40 bytes. Use `#[repr(C, packed)]`, size asserts and
  `read_unaligned`. The `SubFormat` GUIDs are `KSDATAFORMAT_SUBTYPE_IEEE_FLOAT`
  `{00000003-0000-0010-8000-00aa00389b71}` and `_PCM` `{00000001-…}`.
- **`GetId`** returns a `CoTaskMemAlloc`'d UTF-16 string that the caller frees with
  `CoTaskMemFree`. Make it **deterministic from the CoreAudio device UID**, e.g.
  `{0.0.0.00000000}.{<uuid5(UID)>}`. Wwise hashes device IDs into `AkDeviceID`, and some code
  persists them. `GetState` → `DEVICE_STATE_ACTIVE` (1). `EnumAudioEndpoints(eRender,
  DEVICE_STATE_ACTIVE)` returns an `IMMDeviceCollection` (`GetCount`, `Item`).
  `GetDefaultAudioEndpoint(eCapture, …)` → `E_NOTFOUND` (0x80070490).
- **`CoCreateInstance`** needs a class table: `CLSID_MMDeviceEnumerator`
  `{BCDE0395-E52F-467C-8E3D-C4579291692E}` / `IID_IMMDeviceEnumerator`
  `{A95664D2-9614-4F35-A746-DE8DB63617E6}`. A non-null `pUnkOuter` returns
  `CLASS_E_NOAGGREGATION`.
- **No panics across `extern "win64"`** (`catch_unwind` → `E_FAIL`, or abort with a log). Rust
  code reached from the IO thread must not log through anything that touches the TEB.

### 2.3 Validation: our tap vs `AK::SoundEngine::StartOutputCapture`

- **What the two points see.** Wwise captures the System device's final mix, after the master
  bus and its effects, inside the output manager, just before handing the buffer to the sink.
  The sink then only reformats: deinterleaved → interleaved, Wwise channel order → WAVEFORMAT
  order, float. With **48 kHz, float, stereo** there is no SRC (thanks to the 48 kHz mix
  format), no channel reorder and no dither, so **bit-exact equality is achievable**.
  - The Windows audio engine's limiter and the endpoint volume sit *after* the render client,
    and so do CoreAudio's device volume and clipping. None of them affect our tap.
  - Multichannel breaks naive comparison: Wwise's internal order puts LFE last, WAVEFORMAT puts
    it 4th. Compare per named channel.
- **Check the capture file's `fmt ` chunk first.** If the original writes 16-bit PCM [A: older
  Wwise versions did, I'm not sure about 2022.1], "sample-identical" can only mean "identical
  after applying AK's float→int16 conversion to the tap". Get the clamp, scale (32767 vs 32768)
  and round-vs-truncate behaviour from the decompiled writer, and expect no dither. If it is
  float32, demand bit equality.
- **Alignment.** The capture starts at the audio frame (512-sample boundary) where the
  `StartOutputCapture` command is processed. The tap runs from `Start`, beginning with
  `bufferFrames` of prefilled silence. Write a tap sidecar of
  `(seq, frames, flags, w_before, QPC)` per `ReleaseBuffer`. Align by finding the first 512-frame
  block of the capture in the tap (exact search over non-silent blocks), then compare the
  overlap and report the max |Δ| and residual dBFS.
- **Volume:** if Remedy's master volume is a bus volume or RTPC it sits before the capture
  point and both sides match. Test at 100% and 50%, and expect identical ratios. If
  `SetOutputVolume`/device volume is applied in the sink, you will see a constant gain
  difference, which tells you it is post-capture.
- **The tap writes WAV float32 at 48 kHz, interleaved, plus the sidecar**, to `out/audio/tap/`
  (gitignored). Keep the tap's write path off the IO thread: it runs in `ReleaseBuffer` on
  Wwise's thread, appending to a buffered file. Don't do an fsync per call.

### Track 2 — top 10 pitfalls
1. **Missing or `VT_EMPTY` `PKEY_AudioEngine_DeviceFormat`.** The Wwise sink init fails outright.
   It has to be a `VT_BLOB` with the blob pointer at PROPVARIANT+0x10, and `PropVariantClear`
   must free it (it currently leaks).
2. **Returning `S_FALSE` or `AUDCLNT_S_*` where Wwise checks `== 0`** (Initialize,
   SetEventHandle, GetBufferSize, GetMixFormat, GetService). Exposing `IAudioClient2` with a
   failing `SetClientProperties` also fails the sink.
3. **Signalling the Win32 event from the CoreAudio IO thread.** That takes a global mutex
   (`handles::WAIT_LOCK`): priority inversion, and a TEB-less thread touching TEB paths. Use
   mach semaphore → bridge thread.
4. **Ignoring `AUDCLNT_BUFFERFLAGS_SILENT`.** Garbage gets played during the prefill and in any
   later silent buffer.
5. **A ring smaller than the AU's max slice** (4096 when the display sleeps), which underruns
   repeatedly. Inflate `GetBufferSize` and set `MaximumFramesPerSlice`.
6. **A mix format that isn't 48 kHz.** That triggers `AUTOCONVERTPCM` (which you then must accept)
   or Wwise's internal SRC, and kills bit-exact validation. Keep `GetMixFormat` and DeviceFormat
   consistent (48 kHz, float, EXTENSIBLE, correct mask).
7. **COM vtable or slot mistakes**: missing inherited IUnknown slots, `IAudioClient2` slots not
   appended after `GetService`, `AddRef`/`Release` returning HRESULT, non-atomic refcounts, a QI
   that doesn't null `*ppv` on failure.
8. **Packing and allocator mismatches**: WAVEFORMATEX not packed (18/40 bytes), `GetMixFormat`,
   `GetId` or PROPVARIANT payloads not from `CoTaskMemAlloc`. Wwise frees them with
   `CoTaskMemFree`.
9. **Calling guest code (Wwise or IMMNotificationClient) from CoreAudio or HAL listener threads,
   or calling CoreAudio APIs from Win32 threads.** That breaks the TEB / TSD-slot isolation.
10. **Validating against `StartOutputCapture` without aligning** to the command's frame boundary
    or checking the capture's sample format and channel order. You get false "not identical"
    results, or worse, false passes on silence. Require non-silent overlap.
