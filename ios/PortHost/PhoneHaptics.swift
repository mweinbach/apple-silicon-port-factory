import CoreHaptics
import UIKit

/// Bridges Windows game rumble (`XInputSetState` left low-frequency + right high-frequency
/// motors, 0..65535) directly to the iPhone's built-in Taptic Engine via `CoreHaptics`.
///
/// - Uses `engine.playsHapticsOnly = true` so CoreHaptics never interrupts game audio
///   (`RemoteIO` / `AVAudioSession`).
/// - Maps XInput's dual-motor spectrum onto the Taptic Engine's continuous analog actuator:
///   - `intensity = min(1.0, hypot(left, right))`
///   - `sharpness = (0.12 * left + 0.92 * right) / (left + right)`
///     (left bass motor feels deep/thuddy ~0.12; right crisp motor feels sharp/buzzy ~0.92).
/// - Uses a 30-second looping `CHHapticAdvancedPatternPlayer` (avoiding the built-in
///   Taptic Engine's 30s single-event duration limit) updated via zero-latency dynamic
///   parameters on a dedicated `.userInteractive` serial queue so Windows game threads
///   never stall.
/// - Automatically stops on app backgrounding / interruptions and gracefully no-ops on
///   hardware without a Taptic Engine (iPad / iOS Simulator).
final class PhoneHapticsBridge: @unchecked Sendable {
    static let shared = PhoneHapticsBridge()

    let supportsHardwareHaptics: Bool = CHHapticEngine.capabilitiesForHardware().supportsHaptics

    private let queue = DispatchQueue(label: "com.portfactory.phone-haptics", qos: .userInteractive)
    private var engine: CHHapticEngine?
    private var player: CHHapticAdvancedPatternPlayer?
    private var engineRunning = false
    private var playerPlaying = false
    private var appActive = true
    private var enabledFlag = true
    private var lastLeft: UInt16 = 0
    private var lastRight: UInt16 = 0
    private var observers: [NSObjectProtocol] = []
    private var started = false

    private init() {}

    var isEnabled: Bool {
        queue.sync { enabledFlag && supportsHardwareHaptics }
    }

    /// Install lifecycle observers and register the C-ABI rumble callback with `win32-rt`
    /// when hardware haptics are supported (or when enabled).
    func start() {
        queue.sync {
            guard !started else { return }
            started = true
            if supportsHardwareHaptics {
                installLifecycleObservers()
                registerCallbackLocked(active: enabledFlag)
                NSLog("PortHost: CoreHaptics phone rumble passthrough enabled (Taptic Engine ready)")
            } else {
                NSLog("PortHost: CoreHaptics hardware Taptic Engine unavailable on this device")
            }
        }
    }

    /// Enable or disable game-to-phone haptics passthrough at runtime (e.g. from HUD toggle).
    @discardableResult
    func setEnabled(_ enabled: Bool) -> Bool {
        queue.sync {
            guard supportsHardwareHaptics else {
                enabledFlag = false
                return false
            }
            enabledFlag = enabled
            registerCallbackLocked(active: enabled)
            if !enabled {
                stopPlayerLocked()
            }
            return enabledFlag
        }
    }

    /// Stop active haptics and unregister callback cleanly on scene disconnect / termination.
    func stop() {
        queue.sync {
            registerCallbackLocked(active: false)
            stopPlayerLocked()
            engine?.stop(completionHandler: nil)
            engine = nil
            player = nil
            engineRunning = false
            started = false
            for obs in observers {
                NotificationCenter.default.removeObserver(obs)
            }
            observers.removeAll()
        }
    }

    /// Invoked from the C-ABI `PortIosRumbleCallback` on any Windows worker thread.
    func setRumble(left: UInt16, right: UInt16) {
        queue.async { [self] in
            lastLeft = left
            lastRight = right
            applyRumbleLocked(left: left, right: right)
        }
    }

    // MARK: - Private (must run on `queue`)

    private func registerCallbackLocked(active: Bool) {
        if active && supportsHardwareHaptics {
            port_ios_set_rumble_callback { left, right in
                PhoneHapticsBridge.shared.setRumble(left: left, right: right)
            }
        } else {
            port_ios_set_rumble_callback(nil)
        }
    }

    private func installLifecycleObservers() {
        let nc = NotificationCenter.default
        observers.append(nc.addObserver(
            forName: UIApplication.willResignActiveNotification,
            object: nil,
            queue: nil
        ) { [weak self] _ in
            guard let self else { return }
            self.queue.async {
                self.appActive = false
                self.stopPlayerLocked()
            }
        })
        observers.append(nc.addObserver(
            forName: UIApplication.didBecomeActiveNotification,
            object: nil,
            queue: nil
        ) { [weak self] _ in
            guard let self else { return }
            self.queue.async {
                self.appActive = true
                if self.enabledFlag && (self.lastLeft > 0 || self.lastRight > 0) {
                    self.applyRumbleLocked(left: self.lastLeft, right: self.lastRight)
                }
            }
        })
    }

    private func ensureEngineAndPlayerLocked() -> CHHapticAdvancedPatternPlayer? {
        guard supportsHardwareHaptics else { return nil }
        if engine == nil {
            do {
                let eng = try CHHapticEngine()
                eng.playsHapticsOnly = true
                eng.isAutoShutdownEnabled = true
                eng.stoppedHandler = { [weak self] _ in
                    guard let self else { return }
                    self.queue.async {
                        self.engineRunning = false
                        self.playerPlaying = false
                    }
                }
                eng.resetHandler = { [weak self] in
                    guard let self else { return }
                    self.queue.async {
                        self.engineRunning = false
                        self.playerPlaying = false
                        self.player = nil
                        if self.enabledFlag && self.appActive && (self.lastLeft > 0 || self.lastRight > 0) {
                            self.applyRumbleLocked(left: self.lastLeft, right: self.lastRight)
                        }
                    }
                }
                engine = eng
            } catch {
                NSLog("PortHost: failed to create CHHapticEngine: %@", error.localizedDescription)
                return nil
            }
        }
        guard let eng = engine else { return nil }

        if !engineRunning {
            do {
                try eng.start()
                engineRunning = true
            } catch {
                NSLog("PortHost: failed to start CHHapticEngine: %@", error.localizedDescription)
                return nil
            }
        }

        if player == nil {
            do {
                // Base intensity = 1.0 (multiplied by hapticIntensityControl in [0, 1]).
                // Base sharpness = 0.0 (offset additively by hapticSharpnessControl in [0, 1]).
                let intensityParam = CHHapticEventParameter(parameterID: .hapticIntensity, value: 1.0)
                let sharpnessParam = CHHapticEventParameter(parameterID: .hapticSharpness, value: 0.0)
                let event = CHHapticEvent(
                    eventType: .hapticContinuous,
                    parameters: [intensityParam, sharpnessParam],
                    relativeTime: 0,
                    duration: 30.0
                )
                let pattern = try CHHapticPattern(events: [event], parameters: [])
                let adv = try eng.makeAdvancedPlayer(with: pattern)
                adv.loopEnabled = true
                player = adv
                playerPlaying = false
            } catch {
                NSLog("PortHost: failed to build continuous haptic player: %@", error.localizedDescription)
                return nil
            }
        }
        return player
    }

    private func stopPlayerLocked() {
        guard playerPlaying, let p = player else { return }
        do {
            try p.stop(atTime: CHHapticTimeImmediate)
        } catch {
            // Engine may already have been stopped/reset by iOS audio interruption.
        }
        playerPlaying = false
    }

    private func applyRumbleLocked(left: UInt16, right: UInt16) {
        guard enabledFlag && appActive && supportsHardwareHaptics else {
            stopPlayerLocked()
            return
        }
        let l = Float(left) / 65535.0
        let r = Float(right) / 65535.0
        let intensity = min(1.0, hypot(l, r))
        if intensity < 0.01 {
            stopPlayerLocked()
            return
        }
        let sharpness = min(1.0, max(0.0, (0.12 * l + 0.92 * r) / max(l + r, 0.0001)))

        guard let p = ensureEngineAndPlayerLocked() else { return }
        do {
            let dynIntensity = CHHapticDynamicParameter(
                parameterID: .hapticIntensityControl,
                value: intensity,
                relativeTime: 0
            )
            let dynSharpness = CHHapticDynamicParameter(
                parameterID: .hapticSharpnessControl,
                value: sharpness,
                relativeTime: 0
            )
            if !playerPlaying {
                try p.start(atTime: CHHapticTimeImmediate)
                playerPlaying = true
            }
            try p.sendParameters([dynIntensity, dynSharpness], atTime: CHHapticTimeImmediate)
        } catch {
            // Engine was likely interrupted; force clean re-creation on next rumble update.
            playerPlaying = false
            engineRunning = false
            player = nil
        }
    }
}
