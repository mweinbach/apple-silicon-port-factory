import GameController
import UIKit

/// Snapshot of detected native iOS hardware and software keyboard state.
struct NativeKeyboardStatus: Equatable {
    var hardwareConnected: Bool = false
    var softwareVisible: Bool = false
    var softwareHeightPt: CGFloat = 0
    var softwareHeightPx: UInt32 = 0
}

/// Complete USB HID Usage Page 0x07 (`UIKeyboardHIDUsage` / `GCKeyCode`) -> macOS `kVK_*`
/// positional virtual keycode mapping matching `win32-rt/src/user32/keymap.rs`.
enum HidKeyMapper {
    private static let table: [Int: UInt16] = [
        // Letters A..Z (HID 0x04..0x1D)
        0x04: 0x00, // A
        0x05: 0x0B, // B
        0x06: 0x08, // C
        0x07: 0x02, // D
        0x08: 0x0E, // E
        0x09: 0x03, // F
        0x0A: 0x05, // G
        0x0B: 0x04, // H
        0x0C: 0x22, // I
        0x0D: 0x26, // J
        0x0E: 0x28, // K
        0x0F: 0x25, // L
        0x10: 0x2E, // M
        0x11: 0x2D, // N
        0x12: 0x1F, // O
        0x13: 0x23, // P
        0x14: 0x0C, // Q
        0x15: 0x0F, // R
        0x16: 0x01, // S
        0x17: 0x11, // T
        0x18: 0x20, // U
        0x19: 0x09, // V
        0x1A: 0x0D, // W
        0x1B: 0x07, // X
        0x1C: 0x10, // Y
        0x1D: 0x06, // Z

        // Top-row digits 1..9, 0 (HID 0x1E..0x27)
        0x1E: 0x12, // 1
        0x1F: 0x13, // 2
        0x20: 0x14, // 3
        0x21: 0x15, // 4
        0x22: 0x17, // 5
        0x23: 0x16, // 6
        0x24: 0x1A, // 7
        0x25: 0x1C, // 8
        0x26: 0x19, // 9
        0x27: 0x1D, // 0

        // Control & punctuation keys (HID 0x28..0x39)
        0x28: 0x24, // Return / Enter
        0x29: 0x35, // Escape
        0x2A: 0x33, // Backspace / Delete
        0x2B: 0x30, // Tab
        0x2C: 0x31, // Space
        0x2D: 0x1B, // -
        0x2E: 0x18, // =
        0x2F: 0x21, // [
        0x30: 0x1E, // ]
        0x31: 0x2A, // \
        0x32: 0x2A, // Non-US #/~ -> \
        0x33: 0x29, // ;
        0x34: 0x27, // '
        0x35: 0x32, // `
        0x36: 0x2B, // ,
        0x37: 0x2F, // .
        0x38: 0x2C, // /
        0x39: 0x39, // Caps Lock

        // Function keys F1..F12 (HID 0x3A..0x45)
        0x3A: 0x7A, // F1
        0x3B: 0x78, // F2
        0x3C: 0x63, // F3
        0x3D: 0x76, // F4
        0x3E: 0x60, // F5
        0x3F: 0x61, // F6
        0x40: 0x62, // F7
        0x41: 0x64, // F8
        0x42: 0x65, // F9
        0x43: 0x6D, // F10
        0x44: 0x67, // F11
        0x45: 0x6F, // F12

        // Navigation & arrows (HID 0x49..0x52)
        0x49: 0x72, // Insert / Help
        0x4A: 0x73, // Home
        0x4B: 0x74, // Page Up
        0x4C: 0x75, // Forward Delete
        0x4D: 0x77, // End
        0x4E: 0x79, // Page Down
        0x4F: 0x7C, // Right Arrow
        0x50: 0x7B, // Left Arrow
        0x51: 0x7D, // Down Arrow
        0x52: 0x7E, // Up Arrow

        // Keypad (HID 0x53..0x63)
        0x53: 0x47, // Num Lock / Clear
        0x54: 0x4B, // Keypad /
        0x55: 0x43, // Keypad *
        0x56: 0x4E, // Keypad -
        0x57: 0x45, // Keypad +
        0x58: 0x4C, // Keypad Enter
        0x59: 0x53, // Keypad 1
        0x5A: 0x54, // Keypad 2
        0x5B: 0x55, // Keypad 3
        0x5C: 0x56, // Keypad 4
        0x5D: 0x57, // Keypad 5
        0x5E: 0x58, // Keypad 6
        0x5F: 0x59, // Keypad 7
        0x60: 0x5B, // Keypad 8
        0x61: 0x5C, // Keypad 9
        0x62: 0x52, // Keypad 0
        0x63: 0x41, // Keypad .
        0x64: 0x0A, // ISO non-US backslash
        0x65: 0x6E, // Application / Menu

        // Modifiers (HID 0xE0..0xE6)
        0xE0: 0x3B, // Left Control
        0xE1: 0x38, // Left Shift
        0xE2: 0x3A, // Left Alt / Option
        0xE4: 0x3E, // Right Control
        0xE5: 0x3C, // Right Shift
        0xE6: 0x3D, // Right Alt / Option
    ]

    static func hidToMac(_ usage: Int) -> UInt16? {
        table[usage]
    }

    static func usUtf16(forMac mac: UInt16, shift: Bool) -> UInt16 {
        switch mac {
        case 0x00: return shift ? 0x41 : 0x61 // A
        case 0x0B: return shift ? 0x42 : 0x62 // B
        case 0x08: return shift ? 0x43 : 0x63 // C
        case 0x02: return shift ? 0x44 : 0x64 // D
        case 0x0E: return shift ? 0x45 : 0x65 // E
        case 0x03: return shift ? 0x46 : 0x66 // F
        case 0x05: return shift ? 0x47 : 0x67 // G
        case 0x04: return shift ? 0x48 : 0x68 // H
        case 0x22: return shift ? 0x49 : 0x69 // I
        case 0x26: return shift ? 0x4A : 0x6A // J
        case 0x28: return shift ? 0x4B : 0x6B // K
        case 0x25: return shift ? 0x4C : 0x6C // L
        case 0x2E: return shift ? 0x4D : 0x6D // M
        case 0x2D: return shift ? 0x4E : 0x6E // N
        case 0x1F: return shift ? 0x4F : 0x6F // O
        case 0x23: return shift ? 0x50 : 0x70 // P
        case 0x0C: return shift ? 0x51 : 0x71 // Q
        case 0x0F: return shift ? 0x52 : 0x72 // R
        case 0x01: return shift ? 0x53 : 0x73 // S
        case 0x11: return shift ? 0x54 : 0x74 // T
        case 0x20: return shift ? 0x55 : 0x75 // U
        case 0x09: return shift ? 0x56 : 0x76 // V
        case 0x0D: return shift ? 0x57 : 0x77 // W
        case 0x07: return shift ? 0x58 : 0x78 // X
        case 0x10: return shift ? 0x59 : 0x79 // Y
        case 0x06: return shift ? 0x5A : 0x7A // Z
        case 0x12: return shift ? 0x21 : 0x31 // 1 / !
        case 0x13: return shift ? 0x40 : 0x32 // 2 / @
        case 0x14: return shift ? 0x23 : 0x33 // 3 / #
        case 0x15: return shift ? 0x24 : 0x34 // 4 / $
        case 0x17: return shift ? 0x25 : 0x35 // 5 / %
        case 0x16: return shift ? 0x5E : 0x36 // 6 / ^
        case 0x1A: return shift ? 0x26 : 0x37 // 7 / &
        case 0x1C: return shift ? 0x2A : 0x38 // 8 / *
        case 0x19: return shift ? 0x28 : 0x39 // 9 / (
        case 0x1D: return shift ? 0x29 : 0x30 // 0 / )
        case 0x24, 0x4C: return 0x0D          // Return
        case 0x30: return 0x09                // Tab
        case 0x31: return 0x20                // Space
        case 0x33: return 0x08                // Backspace
        case 0x35: return 0x1B                // Escape
        case 0x1B: return shift ? 0x5F : 0x2D // - / _
        case 0x18: return shift ? 0x2B : 0x3D // = / +
        case 0x21: return shift ? 0x7B : 0x5B // [ / {
        case 0x1E: return shift ? 0x7D : 0x5D // ] / }
        case 0x2A: return shift ? 0x7C : 0x5C // \ / |
        case 0x29: return shift ? 0x3A : 0x3B // ; / :
        case 0x27: return shift ? 0x22 : 0x27 // ' / "
        case 0x32: return shift ? 0x7E : 0x60 // ` / ~
        case 0x2B: return shift ? 0x3C : 0x2C // , / <
        case 0x2F: return shift ? 0x3E : 0x2E // . / >
        case 0x2C: return shift ? 0x3F : 0x2F // / / ?
        default: return 0
        }
    }
}

/// Hidden `UIKeyInput` responder that summons the native iOS software keyboard on demand
/// and provides a compact PC shortcut accessory bar (`Esc`, `Tab`, `Ctrl`, `Alt`, `Done`).
final class NativeSoftKeyboardProxy: UIView, UIKeyInput {
    var onInsertText: ((String) -> Void)?
    var onDeleteBackward: (() -> Void)?
    var onSpecialKey: ((UInt16, Bool) -> Void)?
    var onDismissRequested: (() -> Void)?

    private var ctrlLatched = false
    private var altLatched = false
    private weak var ctrlButton: UIBarButtonItem?
    private weak var altButton: UIBarButtonItem?

    private lazy var accessoryBar: UIToolbar = {
        let bar = UIToolbar(frame: CGRect(x: 0, y: 0, width: 320, height: 42))
        bar.barStyle = .black
        bar.isTranslucent = true
        bar.tintColor = .white

        let esc = UIBarButtonItem(title: "Esc", style: .plain, target: self, action: #selector(tapEsc))
        let tab = UIBarButtonItem(title: "Tab", style: .plain, target: self, action: #selector(tapTab))
        let ctrl = UIBarButtonItem(title: "Ctrl", style: .plain, target: self, action: #selector(toggleCtrl))
        let alt = UIBarButtonItem(title: "Alt", style: .plain, target: self, action: #selector(toggleAlt))
        let flex = UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil)
        let done = UIBarButtonItem(barButtonSystemItem: .done, target: self, action: #selector(tapDone))
        self.ctrlButton = ctrl
        self.altButton = alt
        bar.items = [esc, tab, ctrl, alt, flex, done]
        return bar
    }()

    override var inputAccessoryView: UIView? { accessoryBar }
    override var canBecomeFirstResponder: Bool { true }

    var hasText: Bool { true }
    var autocorrectionType: UITextAutocorrectionType { get { .no } set {} }
    var autocapitalizationType: UITextAutocapitalizationType { get { .none } set {} }
    var spellCheckingType: UITextSpellCheckingType { get { .no } set {} }
    var smartQuotesType: UITextSmartQuotesType { get { .no } set {} }
    var smartDashesType: UITextSmartDashesType { get { .no } set {} }
    var smartInsertDeleteType: UITextSmartInsertDeleteType { get { .no } set {} }
    var keyboardAppearance: UIKeyboardAppearance { get { .dark } set {} }

    func insertText(_ text: String) {
        onInsertText?(text)
        releaseLatchedModifiers()
    }

    func deleteBackward() {
        onDeleteBackward?()
        releaseLatchedModifiers()
    }

    private func releaseLatchedModifiers() {
        if ctrlLatched {
            ctrlLatched = false
            ctrlButton?.tintColor = .white
            onSpecialKey?(0x3B, false) // Left Control up
        }
        if altLatched {
            altLatched = false
            altButton?.tintColor = .white
            onSpecialKey?(0x3A, false) // Left Alt up
        }
    }

    @objc private func tapEsc() {
        onSpecialKey?(0x35, true)
        onSpecialKey?(0x35, false)
    }

    @objc private func tapTab() {
        onSpecialKey?(0x30, true)
        onSpecialKey?(0x30, false)
    }

    @objc private func toggleCtrl() {
        ctrlLatched.toggle()
        ctrlButton?.tintColor = ctrlLatched ? .systemTeal : .white
        onSpecialKey?(0x3B, ctrlLatched)
    }

    @objc private func toggleAlt() {
        altLatched.toggle()
        altButton?.tintColor = altLatched ? .systemTeal : .white
        onSpecialKey?(0x3A, altLatched)
    }

    @objc private func tapDone() {
        releaseLatchedModifiers()
        resignFirstResponder()
        onDismissRequested?()
    }
}

/// Unified iOS native hardware (`GCKeyboard` + `UIPress`) and software (`UIKeyInput` +
/// `UIResponder.keyboardWill*`) keyboard detector and Win32 input bridge.
final class NativeKeyboardBridge {
    private weak var hostView: UIView?
    let softProxy = NativeSoftKeyboardProxy(frame: CGRect(x: 0, y: 0, width: 1, height: 1))

    private(set) var status = NativeKeyboardStatus()
    var onStatusChanged: ((NativeKeyboardStatus) -> Void)?

    private var gcKeyboardActive = false
    private var sawPressHardwareKeyboard = false
    private var shiftHeld = false
    private var observers: [NSObjectProtocol] = []

    init(hostView: UIView) {
        self.hostView = hostView
        softProxy.isHidden = true
        hostView.addSubview(softProxy)

        softProxy.onInsertText = { [weak self] text in
            guard let self else { return }
            // When a GCKeyboard hardware keyboard is active and the on-screen software keyboard
            // is NOT visible, hardware keys are already delivered directly via GCKeyboard.
            if self.gcKeyboardActive && !self.status.softwareVisible {
                return
            }
            text.withCString { cstr in
                port_ios_text_input(cstr)
            }
        }
        softProxy.onDeleteBackward = { [weak self] in
            guard let self else { return }
            if self.gcKeyboardActive && !self.status.softwareVisible {
                return
            }
            port_ios_key_event(0x33, true, false, 0x08)
            port_ios_key_event(0x33, false, false, 0)
        }
        softProxy.onSpecialKey = { mac, down in
            let ch = down ? HidKeyMapper.usUtf16(forMac: mac, shift: false) : 0
            port_ios_key_event(mac, down, false, ch)
        }
        softProxy.onDismissRequested = { [weak self] in
            self?.refreshStatus(softwareVisible: false, softwareHeightPt: 0)
        }

        installObservers()
        attachCoalescedGCKeyboard()
    }

    deinit {
        for obs in observers {
            NotificationCenter.default.removeObserver(obs)
        }
        GCKeyboard.coalesced?.keyboardInput?.keyChangedHandler = nil
    }

    var isSoftKeyboardActive: Bool {
        softProxy.isFirstResponder && status.softwareVisible
    }

    func toggleSoftKeyboard() {
        if softProxy.isFirstResponder {
            softProxy.resignFirstResponder()
        } else {
            softProxy.becomeFirstResponder()
        }
    }

    func showSoftKeyboard() {
        if !softProxy.isFirstResponder {
            softProxy.becomeFirstResponder()
        }
    }

    func hideSoftKeyboard() {
        if softProxy.isFirstResponder {
            softProxy.resignFirstResponder()
        }
    }

    // MARK: - UIPress Hardware Keyboard Fallback (deduplicated against GCKeyboard)

    func handlePressesBegan(_ presses: Set<UIPress>) -> Bool {
        if gcKeyboardActive {
            return false
        }
        var handled = false
        for press in presses {
            guard let key = press.key else { continue }
            if !sawPressHardwareKeyboard {
                sawPressHardwareKeyboard = true
                refreshStatus()
            }
            if let mac = HidKeyMapper.hidToMac(key.keyCode.rawValue) {
                updateShiftFlag(mac: mac, down: true)
                let utf16 = key.characters.utf16.first ?? HidKeyMapper.usUtf16(forMac: mac, shift: shiftHeld)
                port_ios_key_event(mac, true, false, utf16)
                handled = true
            }
        }
        return handled
    }

    func handlePressesEnded(_ presses: Set<UIPress>) -> Bool {
        if gcKeyboardActive {
            return false
        }
        var handled = false
        for press in presses {
            guard let key = press.key else { continue }
            if let mac = HidKeyMapper.hidToMac(key.keyCode.rawValue) {
                updateShiftFlag(mac: mac, down: false)
                port_ios_key_event(mac, false, false, 0)
                handled = true
            }
        }
        return handled
    }

    // MARK: - GCKeyboard & UIKeyboard Notifications

    private func installObservers() {
        let nc = NotificationCenter.default

        observers.append(nc.addObserver(
            forName: .GCKeyboardDidConnect,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.attachCoalescedGCKeyboard()
        })

        observers.append(nc.addObserver(
            forName: .GCKeyboardDidDisconnect,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            guard let self else { return }
            self.gcKeyboardActive = GCKeyboard.coalesced != nil
            if !self.gcKeyboardActive {
                self.sawPressHardwareKeyboard = false
            }
            self.refreshStatus()
        })

        observers.append(nc.addObserver(
            forName: UIResponder.keyboardWillShowNotification,
            object: nil,
            queue: .main
        ) { [weak self] note in
            self?.handleKeyboardFrameNotification(note, hiding: false)
        })

        observers.append(nc.addObserver(
            forName: UIResponder.keyboardWillChangeFrameNotification,
            object: nil,
            queue: .main
        ) { [weak self] note in
            self?.handleKeyboardFrameNotification(note, hiding: false)
        })

        observers.append(nc.addObserver(
            forName: UIResponder.keyboardWillHideNotification,
            object: nil,
            queue: .main
        ) { [weak self] note in
            self?.handleKeyboardFrameNotification(note, hiding: true)
        })
    }

    private func attachCoalescedGCKeyboard() {
        guard let kb = GCKeyboard.coalesced, let input = kb.keyboardInput else {
            gcKeyboardActive = false
            refreshStatus()
            return
        }
        gcKeyboardActive = true
        input.keyChangedHandler = { [weak self] _, _, keyCode, pressed in
            guard let self else { return }
            guard let mac = HidKeyMapper.hidToMac(keyCode.rawValue) else { return }
            self.updateShiftFlag(mac: mac, down: pressed)
            let utf16 = pressed ? HidKeyMapper.usUtf16(forMac: mac, shift: self.shiftHeld) : 0
            port_ios_key_event(mac, pressed, false, utf16)
        }
        refreshStatus()
    }

    private func handleKeyboardFrameNotification(_ note: Notification, hiding: Bool) {
        guard let hostView else { return }
        if hiding {
            refreshStatus(softwareVisible: false, softwareHeightPt: 0)
            return
        }
        guard let frameVal = note.userInfo?[UIResponder.keyboardFrameEndUserInfoKey] as? NSValue else {
            return
        }
        let screenRect = frameVal.cgRectValue
        let localRect = hostView.convert(screenRect, from: nil)
        let overlapPt = max(0, hostView.bounds.maxY - localRect.minY)

        // iOS shows a slim (< 100 pt) shortcut/input-accessory bar when a native hardware
        // keyboard is attached, and a full (>= 100 pt) on-screen software keyboard otherwise.
        if overlapPt >= 100.0 {
            refreshStatus(softwareVisible: true, softwareHeightPt: overlapPt)
        } else if overlapPt > 0.0 {
            sawPressHardwareKeyboard = true
            refreshStatus(softwareVisible: false, softwareHeightPt: 0)
        } else {
            refreshStatus(softwareVisible: false, softwareHeightPt: 0)
        }
    }

    private func updateShiftFlag(mac: UInt16, down: Bool) {
        if mac == 0x38 || mac == 0x3C {
            shiftHeld = down
        }
    }

    private func refreshStatus(softwareVisible: Bool? = nil, softwareHeightPt: CGFloat? = nil) {
        let hw = gcKeyboardActive || sawPressHardwareKeyboard
        let sw = softwareVisible ?? status.softwareVisible
        let hPt = softwareHeightPt ?? (sw ? status.softwareHeightPt : 0)
        let scale = hostView?.window?.windowScene?.screen.nativeScale
            ?? hostView?.traitCollection.displayScale
            ?? 1.0
        let hPx = sw ? UInt32((hPt * max(scale, 1.0)).rounded()) : 0
        let next = NativeKeyboardStatus(
            hardwareConnected: hw,
            softwareVisible: sw,
            softwareHeightPt: hPt,
            softwareHeightPx: hPx
        )
        let changed = next != status
        status = next
        port_ios_keyboard_state(next.hardwareConnected, next.softwareVisible, next.softwareHeightPx)
        if changed {
            onStatusChanged?(next)
        }
    }
}
