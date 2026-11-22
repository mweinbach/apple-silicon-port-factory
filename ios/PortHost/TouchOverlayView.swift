import GameController
import QuartzCore
import UIKit

/// User-selectable touch overlay presentation mode.
enum TouchOverlayMode: Int, CaseIterable {
    /// Ergonomic floating-stick + swipe-camera XInput overlay (auto-hides if software keyboard opens).
    case gamepad = 0
    /// Direct touch-as-mouse mode (hides virtual gamepad; single tap = left click, 2-finger = right click / scroll).
    case touchMouse = 1
    /// Minimal corner pill only for unobstructed fullscreen viewing (physical controller / hardware keyboard).
    case hidden = 2

    var shortLabel: String {
        switch self {
        case .gamepad: return "Pad"
        case .touchMouse: return "Touch"
        case .hidden: return "Hide"
        }
    }

    var symbolName: String {
        switch self {
        case .gamepad: return "gamecontroller.fill"
        case .touchMouse: return "hand.tap.fill"
        case .hidden: return "eye.slash"
        }
    }

    func next() -> TouchOverlayMode {
        TouchOverlayMode(rawValue: (rawValue + 1) % TouchOverlayMode.allCases.count) ?? .gamepad
    }
}

/// Right-side camera control style.
private enum CameraLookStyle: Int {
    /// Direct touch-drag trackpad anywhere on the right side (and while holding RT/RB) mapped smoothly to XInput R-Stick.
    case swipe = 0
    /// Floating analog thumbstick on the right half (classic hold-to-turn feel).
    case stick = 1
}

/// Persistent user preferences for on-screen touch controls (`UserDefaults`).
private struct TouchControlPrefs {
    private static let kLookStyle = "port.touch.lookStyle"
    private static let kLookSens = "port.touch.lookSensitivity"
    private static let kOpacity = "port.touch.opacity"
    private static let kAutoSprint = "port.touch.autoSprint"
    private static let kLeftTriggerToggle = "port.touch.ltToggle"
    private static let kInvertY = "port.touch.invertY"

    var lookStyle: CameraLookStyle = .swipe
    var lookSensitivity: Float = 1.15
    var opacity: Float = 0.82
    var autoSprint: Bool = true
    var leftTriggerToggle: Bool = false
    var invertY: Bool = false

    static func load() -> TouchControlPrefs {
        let d = UserDefaults.standard
        var p = TouchControlPrefs()
        if d.object(forKey: kLookStyle) != nil {
            p.lookStyle = CameraLookStyle(rawValue: d.integer(forKey: kLookStyle)) ?? .swipe
        }
        if d.object(forKey: kLookSens) != nil {
            p.lookSensitivity = max(0.4, min(2.4, d.float(forKey: kLookSens)))
        }
        if d.object(forKey: kOpacity) != nil {
            p.opacity = max(0.25, min(1.0, d.float(forKey: kOpacity)))
        }
        if d.object(forKey: kAutoSprint) != nil {
            p.autoSprint = d.bool(forKey: kAutoSprint)
        }
        if d.object(forKey: kLeftTriggerToggle) != nil {
            p.leftTriggerToggle = d.bool(forKey: kLeftTriggerToggle)
        }
        if d.object(forKey: kInvertY) != nil {
            p.invertY = d.bool(forKey: kInvertY)
        }
        return p
    }

    func save() {
        let d = UserDefaults.standard
        d.set(lookStyle.rawValue, forKey: Self.kLookStyle)
        d.set(lookSensitivity, forKey: Self.kLookSens)
        d.set(opacity, forKey: Self.kOpacity)
        d.set(autoSprint, forKey: Self.kAutoSprint)
        d.set(leftTriggerToggle, forKey: Self.kLeftTriggerToggle)
        d.set(invertY, forKey: Self.kInvertY)
    }
}

// MARK: - Glassmorphic Button (Supports Drag-to-Aim + Optional Toggle Latch)

/// Multi-touch glassmorphic push button supporting optional drag-through camera aiming
/// (e.g. hold & drag on `RT`/`RB` to aim while shooting) and optional toggle latching (`LT`).
private final class OverlayButton: UIView {
    private let bgLayer = CALayer()
    private let label = UILabel()
    private let iconView = UIImageView()
    private let lockDot = UIView()
    private let accentColor: UIColor
    private let isCircular: Bool
    private let hitExpansion: CGFloat
    private let haptic: UIImpactFeedbackGenerator

    private var activeTouch: UITouch?
    private var touchStartPoint: CGPoint = .zero
    private var lastGlobalPoint: CGPoint = .zero
    private var isDraggingAim = false
    private(set) var isLatched = false

    /// When true, dragging while holding this button forwards camera deltas to `onAimDragDelta`.
    var supportsDragAim: Bool = false
    /// When true, tapping toggles latched ON/OFF state (e.g. Aim Toggle on `LT`).
    var toggleMode: Bool = false {
        didSet {
            if !toggleMode && isLatched {
                isLatched = false
                updateVisual(pressed: activeTouch != nil, animated: true)
                onPressChanged?(activeTouch != nil)
            }
        }
    }

    var onPressChanged: ((Bool) -> Void)?
    var onAimDragDelta: ((CGFloat, CGFloat) -> Void)?
    var onAimDragEnded: (() -> Void)?
    var onUserInteraction: (() -> Void)?

    init(
        title: String?,
        symbol: String? = nil,
        accent: UIColor = .white,
        circular: Bool = true,
        fontSize: CGFloat = 15,
        hapticStyle: UIImpactFeedbackGenerator.FeedbackStyle = .light,
        hitExpansion: CGFloat = 6,
        id: String
    ) {
        self.accentColor = accent
        self.isCircular = circular
        self.hitExpansion = hitExpansion
        self.haptic = UIImpactFeedbackGenerator(style: hapticStyle)
        super.init(frame: .zero)

        isMultipleTouchEnabled = false
        isExclusiveTouch = false
        isAccessibilityElement = true
        accessibilityLabel = title ?? id
        accessibilityIdentifier = id
        accessibilityTraits = .button

        layer.shadowColor = UIColor.black.cgColor
        layer.shadowOpacity = 0.42
        layer.shadowRadius = 6
        layer.shadowOffset = CGSize(width: 0, height: 2)

        bgLayer.backgroundColor = UIColor(white: 0.09, alpha: 0.46).cgColor
        bgLayer.borderWidth = 1.0
        bgLayer.borderColor = accent.withAlphaComponent(0.45).cgColor
        layer.addSublayer(bgLayer)

        if let title {
            label.text = title
            label.textColor = accent.withAlphaComponent(0.94)
            label.font = .systemFont(ofSize: fontSize, weight: .semibold)
            label.textAlignment = .center
            label.isUserInteractionEnabled = false
            addSubview(label)
        }

        if let symbol {
            let cfg = UIImage.SymbolConfiguration(pointSize: fontSize, weight: .semibold)
            iconView.image = UIImage(systemName: symbol, withConfiguration: cfg)
            iconView.tintColor = accent.withAlphaComponent(0.92)
            iconView.contentMode = .center
            iconView.isUserInteractionEnabled = false
            addSubview(iconView)
        }

        lockDot.backgroundColor = UIColor.systemTeal
        lockDot.isHidden = true
        lockDot.isUserInteractionEnabled = false
        addSubview(lockDot)
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    override func point(inside point: CGPoint, with event: UIEvent?) -> Bool {
        bounds.insetBy(dx: -hitExpansion, dy: -hitExpansion).contains(point)
    }

    override func layoutSubviews() {
        super.layoutSubviews()
        bgLayer.frame = bounds
        let radius = isCircular ? min(bounds.width, bounds.height) * 0.5 : min(bounds.height * 0.48, 16)
        bgLayer.cornerRadius = radius
        label.frame = bounds
        iconView.frame = bounds
        let dotSize: CGFloat = 5
        lockDot.frame = CGRect(x: bounds.width - dotSize - 6, y: 5, width: dotSize, height: dotSize)
        lockDot.layer.cornerRadius = dotSize * 0.5
    }

    private func updateVisual(pressed: Bool, animated: Bool) {
        let active = pressed || isLatched
        lockDot.isHidden = !isLatched
        let changes = {
            self.bgLayer.backgroundColor = active
                ? self.accentColor.withAlphaComponent(self.isLatched && !pressed ? 0.34 : 0.42).cgColor
                : UIColor(white: 0.09, alpha: 0.46).cgColor
            self.bgLayer.borderColor = self.accentColor.withAlphaComponent(active ? 0.95 : 0.45).cgColor
            self.bgLayer.borderWidth = active ? 1.5 : 1.0
            self.transform = pressed ? CGAffineTransform(scaleX: 0.91, y: 0.91) : .identity
        }
        if animated {
            UIView.animate(withDuration: 0.07, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState], animations: changes)
        } else {
            changes()
        }
    }

    func forceRelease() {
        let wasActive = activeTouch != nil || isLatched
        if isDraggingAim {
            isDraggingAim = false
            onAimDragEnded?()
        }
        activeTouch = nil
        isLatched = false
        guard wasActive else { return }
        updateVisual(pressed: false, animated: true)
        onPressChanged?(false)
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard activeTouch == nil, let t = touches.first else { return }
        activeTouch = t
        touchStartPoint = t.location(in: self)
        lastGlobalPoint = t.location(in: superview)
        isDraggingAim = false
        onUserInteraction?()
        haptic.prepare()
        haptic.impactOccurred(intensity: 0.65)

        if toggleMode {
            isLatched.toggle()
            updateVisual(pressed: true, animated: true)
            onPressChanged?(isLatched)
        } else {
            updateVisual(pressed: true, animated: true)
            onPressChanged?(true)
        }
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard supportsDragAim, let active = activeTouch, touches.contains(active) else { return }
        let cur = active.location(in: superview)
        let dx = cur.x - lastGlobalPoint.x
        let dy = cur.y - lastGlobalPoint.y
        lastGlobalPoint = cur
        if !isDraggingAim {
            let startCur = active.location(in: self)
            if hypot(startCur.x - touchStartPoint.x, startCur.y - touchStartPoint.y) >= 3.5 {
                isDraggingAim = true
            }
        }
        if isDraggingAim {
            onAimDragDelta?(dx, dy)
        }
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        activeTouch = nil
        if isDraggingAim {
            isDraggingAim = false
            onAimDragEnded?()
        }
        if toggleMode {
            updateVisual(pressed: false, animated: true)
        } else {
            updateVisual(pressed: false, animated: true)
            onPressChanged?(false)
        }
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        activeTouch = nil
        if isDraggingAim {
            isDraggingAim = false
            onAimDragEnded?()
        }
        if !toggleMode {
            updateVisual(pressed: false, animated: true)
            onPressChanged?(false)
        } else {
            updateVisual(pressed: false, animated: true)
        }
    }
}

// MARK: - Curved Ergonomic ABXY Cluster (With Smooth Thumb-Roll)

/// Console-style curved ABXY cluster that supports smooth thumb-rolling between buttons
/// without lifting (e.g. rolling `A -> B` or `X -> Y`), plus generous hit-slop and multi-touch.
private final class ABXYClusterView: UIView {
    struct FaceItem {
        let bit: UInt16
        let title: String
        let accent: UIColor
        let id: String
        let view: UIView
        let label: UILabel
        var center: CGPoint = .zero
    }

    private var items: [FaceItem] = []
    private var touchToIndex: [ObjectIdentifier: Int] = [:]
    private var pressedMask: UInt16 = 0
    private let haptic = UIImpactFeedbackGenerator(style: .light)
    private var buttonSize: CGFloat = 50

    var onMaskChanged: ((UInt16) -> Void)?
    var onUserInteraction: (() -> Void)?

    override init(frame: CGRect) {
        super.init(frame: frame)
        isMultipleTouchEnabled = true
        isExclusiveTouch = false
        backgroundColor = .clear

        // Refined, softly muted console accents (less harsh than raw neon primaries).
        let specs: [(UInt16, String, UIColor, String)] = [
            (UInt16(PORT_IOS_BTN_A), "A", UIColor(red: 0.36, green: 0.86, blue: 0.56, alpha: 1.0), "overlay.btn.A"),
            (UInt16(PORT_IOS_BTN_B), "B", UIColor(red: 0.96, green: 0.42, blue: 0.42, alpha: 1.0), "overlay.btn.B"),
            (UInt16(PORT_IOS_BTN_X), "X", UIColor(red: 0.40, green: 0.72, blue: 0.98, alpha: 1.0), "overlay.btn.X"),
            (UInt16(PORT_IOS_BTN_Y), "Y", UIColor(red: 0.98, green: 0.80, blue: 0.36, alpha: 1.0), "overlay.btn.Y"),
        ]

        for (bit, title, accent, id) in specs {
            let v = UIView()
            v.isUserInteractionEnabled = false
            v.backgroundColor = UIColor(white: 0.09, alpha: 0.46)
            v.layer.borderWidth = 1.0
            v.layer.borderColor = accent.withAlphaComponent(0.48).cgColor
            v.layer.shadowColor = UIColor.black.cgColor
            v.layer.shadowOpacity = 0.42
            v.layer.shadowRadius = 6
            v.layer.shadowOffset = CGSize(width: 0, height: 2)
            addSubview(v)

            let lbl = UILabel()
            lbl.text = title
            lbl.textColor = accent.withAlphaComponent(0.95)
            lbl.font = .systemFont(ofSize: 17, weight: .bold)
            lbl.textAlignment = .center
            lbl.isUserInteractionEnabled = false
            v.addSubview(lbl)

            items.append(FaceItem(bit: bit, title: title, accent: accent, id: id, view: v, label: lbl))
        }
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    override func layoutSubviews() {
        super.layoutSubviews()
        let side = min(bounds.width, bounds.height)
        buttonSize = max(44, min(56, side * 0.335))
        let step = buttonSize * 0.94
        let cx = bounds.midX
        let cy = bounds.midY
        // Slight ergonomic clockwise rotation (~9 deg) aligns the diamond with natural right-thumb sweep.
        let angle: CGFloat = 0.16
        let cosA = cos(angle)
        let sinA = sin(angle)
        func rotated(_ dx: CGFloat, _ dy: CGFloat) -> CGPoint {
            CGPoint(x: cx + dx * cosA - dy * sinA, y: cy + dx * sinA + dy * cosA)
        }
        // Order matches specs: 0=A (bottom), 1=B (right), 2=X (left), 3=Y (top)
        let offsets: [(CGFloat, CGFloat)] = [
            (0, step),
            (step, 0),
            (-step, 0),
            (0, -step),
        ]
        for i in 0..<items.count {
            let c = rotated(offsets[i].0, offsets[i].1)
            items[i].center = c
            items[i].view.bounds = CGRect(x: 0, y: 0, width: buttonSize, height: buttonSize)
            items[i].view.center = c
            items[i].view.layer.cornerRadius = buttonSize * 0.5
            items[i].label.frame = items[i].view.bounds
        }
    }

    override func point(inside point: CGPoint, with event: UIEvent?) -> Bool {
        nearestIndex(to: point, maxDist: buttonSize * 0.68) != nil
    }

    private func nearestIndex(to pt: CGPoint, maxDist: CGFloat) -> Int? {
        var bestIdx: Int?
        var bestD = maxDist
        for i in 0..<items.count {
            let d = hypot(pt.x - items[i].center.x, pt.y - items[i].center.y)
            if d <= bestD {
                bestD = d
                bestIdx = i
            }
        }
        return bestIdx
    }

    private func syncMask(hapticOnNew: Bool) {
        var next: UInt16 = 0
        for idx in touchToIndex.values {
            next |= items[idx].bit
        }
        let added = next & ~pressedMask
        if hapticOnNew && added != 0 {
            haptic.impactOccurred(intensity: 0.62)
        }
        if next != pressedMask {
            let changed = next ^ pressedMask
            pressedMask = next
            for item in items where (changed & item.bit) != 0 {
                let isDown = (pressedMask & item.bit) != 0
                UIView.animate(withDuration: 0.06, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
                    item.view.backgroundColor = isDown
                        ? item.accent.withAlphaComponent(0.42)
                        : UIColor(white: 0.09, alpha: 0.46)
                    item.view.layer.borderColor = item.accent.withAlphaComponent(isDown ? 0.98 : 0.48).cgColor
                    item.view.layer.borderWidth = isDown ? 1.5 : 1.0
                    item.view.transform = isDown ? CGAffineTransform(scaleX: 0.90, y: 0.90) : .identity
                }
            }
            onMaskChanged?(pressedMask)
        }
    }

    func forceRelease() {
        guard !touchToIndex.isEmpty || pressedMask != 0 else { return }
        touchToIndex.removeAll()
        syncMask(hapticOnNew: false)
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        onUserInteraction?()
        haptic.prepare()
        for t in touches {
            let pt = t.location(in: self)
            if let idx = nearestIndex(to: pt, maxDist: buttonSize * 0.72) {
                touchToIndex[ObjectIdentifier(t)] = idx
            }
        }
        syncMask(hapticOnNew: true)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        for t in touches {
            let key = ObjectIdentifier(t)
            guard touchToIndex[key] != nil else { continue }
            let pt = t.location(in: self)
            // Slightly tighter roll threshold prevents accidental wobble while still enabling crisp A->B / X->Y roll.
            if let idx = nearestIndex(to: pt, maxDist: buttonSize * 0.85) {
                touchToIndex[key] = idx
            } else {
                touchToIndex.removeValue(forKey: key)
            }
        }
        syncMask(hapticOnNew: true)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        for t in touches {
            touchToIndex.removeValue(forKey: ObjectIdentifier(t))
        }
        syncMask(hapticOnNew: false)
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        touchesEnded(touches, with: event)
    }
}

// MARK: - Sculpted Cross D-Pad (With Per-Direction Haptics)

/// Sleek sculpted cross-shaped 4-way D-pad with diagonal support and crisp directional haptics.
private final class OverlayDPad: UIView {
    private let crossLayer = CAShapeLayer()
    private let upIcon = UIImageView()
    private let downIcon = UIImageView()
    private let leftIcon = UIImageView()
    private let rightIcon = UIImageView()
    private var activeTouch: UITouch?
    private var lastMask: UInt16 = 0
    private let haptic = UIImpactFeedbackGenerator(style: .light)

    var onMaskChanged: ((UInt16) -> Void)?
    var onUserInteraction: (() -> Void)?

    override init(frame: CGRect) {
        super.init(frame: frame)
        isMultipleTouchEnabled = false
        isExclusiveTouch = false
        isAccessibilityElement = true
        accessibilityLabel = "D-Pad"
        accessibilityIdentifier = "overlay.dpad"

        crossLayer.fillColor = UIColor(white: 0.09, alpha: 0.46).cgColor
        crossLayer.strokeColor = UIColor(white: 1.0, alpha: 0.34).cgColor
        crossLayer.lineWidth = 1.0
        crossLayer.shadowColor = UIColor.black.cgColor
        crossLayer.shadowOpacity = 0.40
        crossLayer.shadowRadius = 5
        crossLayer.shadowOffset = CGSize(width: 0, height: 2)
        layer.addSublayer(crossLayer)

        let symCfg = UIImage.SymbolConfiguration(pointSize: 10, weight: .bold)
        for (iv, sym) in [
            (upIcon, "chevron.up"),
            (downIcon, "chevron.down"),
            (leftIcon, "chevron.left"),
            (rightIcon, "chevron.right"),
        ] {
            iv.image = UIImage(systemName: sym, withConfiguration: symCfg)
            iv.tintColor = UIColor(white: 1.0, alpha: 0.72)
            iv.contentMode = .center
            iv.isUserInteractionEnabled = false
            addSubview(iv)
        }
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    override func point(inside point: CGPoint, with event: UIEvent?) -> Bool {
        bounds.insetBy(dx: -8, dy: -8).contains(point)
    }

    override func layoutSubviews() {
        super.layoutSubviews()
        let w = bounds.width
        let h = bounds.height
        let armW = w * 0.36
        let armH = h * 0.36
        let r: CGFloat = 7

        let path = UIBezierPath()
        let vRect = CGRect(x: (w - armW) * 0.5, y: 0, width: armW, height: h)
        let hRect = CGRect(x: 0, y: (h - armH) * 0.5, width: w, height: armH)
        path.append(UIBezierPath(roundedRect: vRect, cornerRadius: r))
        path.append(UIBezierPath(roundedRect: hRect, cornerRadius: r))
        crossLayer.path = path.cgPath

        upIcon.frame = CGRect(x: vRect.minX, y: 2, width: armW, height: h * 0.32)
        downIcon.frame = CGRect(x: vRect.minX, y: h * 0.66, width: armW, height: h * 0.32)
        leftIcon.frame = CGRect(x: 2, y: hRect.minY, width: w * 0.32, height: armH)
        rightIcon.frame = CGRect(x: w * 0.66, y: hRect.minY, width: w * 0.32, height: armH)
    }

    func forceRelease() {
        guard activeTouch != nil || lastMask != 0 else { return }
        activeTouch = nil
        applyMask(0, hapticFeedback: false)
    }

    private func applyMask(_ mask: UInt16, hapticFeedback: Bool) {
        guard mask != lastMask else { return }
        if hapticFeedback && mask != 0 {
            haptic.impactOccurred(intensity: 0.52)
        }
        lastMask = mask
        let hi = UIColor.systemTeal
        let lo = UIColor(white: 1.0, alpha: 0.72)
        upIcon.tintColor = (mask & UInt16(PORT_IOS_BTN_DPAD_UP)) != 0 ? hi : lo
        downIcon.tintColor = (mask & UInt16(PORT_IOS_BTN_DPAD_DOWN)) != 0 ? hi : lo
        leftIcon.tintColor = (mask & UInt16(PORT_IOS_BTN_DPAD_LEFT)) != 0 ? hi : lo
        rightIcon.tintColor = (mask & UInt16(PORT_IOS_BTN_DPAD_RIGHT)) != 0 ? hi : lo
        crossLayer.strokeColor = (mask != 0 ? UIColor.systemTeal.withAlphaComponent(0.80) : UIColor(white: 1.0, alpha: 0.34)).cgColor
        onMaskChanged?(mask)
    }

    private func maskFor(_ touch: UITouch) -> UInt16 {
        let pt = touch.location(in: self)
        let nx = (pt.x - bounds.midX) / max(1, bounds.width * 0.5)
        let ny = (pt.y - bounds.midY) / max(1, bounds.height * 0.5)
        var m: UInt16 = 0
        if ny < -0.26 { m |= UInt16(PORT_IOS_BTN_DPAD_UP) }
        if ny > 0.26 { m |= UInt16(PORT_IOS_BTN_DPAD_DOWN) }
        if nx < -0.26 { m |= UInt16(PORT_IOS_BTN_DPAD_LEFT) }
        if nx > 0.26 { m |= UInt16(PORT_IOS_BTN_DPAD_RIGHT) }
        return m
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard activeTouch == nil, let t = touches.first else { return }
        activeTouch = t
        onUserInteraction?()
        haptic.prepare()
        applyMask(maskFor(t), hapticFeedback: true)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        applyMask(maskFor(active), hapticFeedback: true)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        activeTouch = nil
        applyMask(0, hapticFeedback: false)
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        touchesEnded(touches, with: event)
    }
}

// MARK: - Floating Analog Joystick Zone (With Smooth Curve, Follow-Leash & Auto-Sprint)

/// Full-zone floating analog thumbstick: touch down ANYWHERE in the zone and the ring
/// centers under your thumb (zero initial jump). Supports smooth low-end response curve,
/// gentle follow-leash, and optional outer-forward Auto-Sprint (`L3`).
private final class FloatingStickZone: UIView {
    private let ring = UIView()
    private let knob = UIView()
    private let sprintBadge = UILabel()
    private let hapticLight = UIImpactFeedbackGenerator(style: .light)
    private let hapticSprint = UIImpactFeedbackGenerator(style: .medium)

    private var activeTouch: UITouch?
    private var anchorCenter: CGPoint = .zero
    private var sprintEngaged = false

    let ringDiameter: CGFloat
    let knobDiameter: CGFloat
    var restingCenter: CGPoint = .zero {
        didSet {
            if activeTouch == nil {
                anchorCenter = restingCenter
                ring.center = restingCenter
                knob.center = restingCenter
                updateSprintBadgePosition()
            }
        }
    }
    var autoSprintEnabled: Bool = false {
        didSet {
            if !autoSprintEnabled && sprintEngaged {
                sprintEngaged = false
                updateSprintVisual()
                onAutoSprintChanged?(false)
            }
        }
    }
    var showsRestingGhost: Bool = true {
        didSet {
            if activeTouch == nil {
                ring.alpha = showsRestingGhost ? 0.52 : 0.0
                knob.alpha = showsRestingGhost ? 0.52 : 0.0
            }
        }
    }

    var onAxesChanged: ((Float, Float) -> Void)?
    var onAutoSprintChanged: ((Bool) -> Void)?
    var onUserInteraction: (() -> Void)?

    init(ringDiameter: CGFloat = 114, id: String, label: String) {
        self.ringDiameter = ringDiameter
        self.knobDiameter = ringDiameter * 0.42
        super.init(frame: .zero)
        backgroundColor = .clear
        isMultipleTouchEnabled = false
        isExclusiveTouch = false
        accessibilityIdentifier = id
        accessibilityLabel = label

        ring.bounds = CGRect(x: 0, y: 0, width: ringDiameter, height: ringDiameter)
        ring.layer.cornerRadius = ringDiameter * 0.5
        ring.backgroundColor = UIColor(white: 0.08, alpha: 0.30)
        ring.layer.borderWidth = 1.25
        ring.layer.borderColor = UIColor(white: 1.0, alpha: 0.32).cgColor
        ring.isUserInteractionEnabled = false
        addSubview(ring)

        knob.bounds = CGRect(x: 0, y: 0, width: knobDiameter, height: knobDiameter)
        knob.layer.cornerRadius = knobDiameter * 0.5
        knob.backgroundColor = UIColor(white: 0.95, alpha: 0.34)
        knob.layer.borderWidth = 1.25
        knob.layer.borderColor = UIColor(white: 1.0, alpha: 0.78).cgColor
        knob.layer.shadowColor = UIColor.black.cgColor
        knob.layer.shadowOpacity = 0.35
        knob.layer.shadowRadius = 5
        knob.layer.shadowOffset = CGSize(width: 0, height: 2)
        knob.isUserInteractionEnabled = false
        addSubview(knob)

        sprintBadge.text = "» SPRINT"
        sprintBadge.font = .systemFont(ofSize: 9, weight: .bold)
        sprintBadge.textColor = UIColor.systemTeal
        sprintBadge.textAlignment = .center
        sprintBadge.alpha = 0
        sprintBadge.isUserInteractionEnabled = false
        addSubview(sprintBadge)
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    private func updateSprintBadgePosition() {
        sprintBadge.frame = CGRect(
            x: anchorCenter.x - 36,
            y: anchorCenter.y - ringDiameter * 0.5 - 18,
            width: 72,
            height: 14
        )
    }

    private func updateSprintVisual() {
        UIView.animate(withDuration: 0.10, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
            self.sprintBadge.alpha = self.sprintEngaged ? 0.95 : 0.0
            self.ring.layer.borderColor = self.sprintEngaged
                ? UIColor.systemTeal.withAlphaComponent(0.95).cgColor
                : (self.activeTouch != nil
                    ? UIColor.white.withAlphaComponent(0.62).cgColor
                    : UIColor(white: 1.0, alpha: 0.32).cgColor)
            self.knob.backgroundColor = self.sprintEngaged
                ? UIColor.systemTeal.withAlphaComponent(0.55)
                : (self.activeTouch != nil
                    ? UIColor.white.withAlphaComponent(0.48)
                    : UIColor(white: 0.95, alpha: 0.34))
        }
    }

    private func clampedAnchor(_ pt: CGPoint) -> CGPoint {
        let pad = ringDiameter * 0.45
        let x = max(pad, min(max(pad, bounds.width - pad), pt.x))
        let y = max(pad, min(max(pad, bounds.height - pad), pt.y))
        return CGPoint(x: x, y: y)
    }

    func forceRelease() {
        guard activeTouch != nil || sprintEngaged else { return }
        activeTouch = nil
        resetVisualAndOutput()
    }

    private func resetVisualAndOutput() {
        let wasSprinting = sprintEngaged
        sprintEngaged = false
        UIView.animate(withDuration: 0.14, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
            self.anchorCenter = self.restingCenter
            self.ring.center = self.restingCenter
            self.knob.center = self.restingCenter
            self.ring.alpha = self.showsRestingGhost ? 0.52 : 0.0
            self.knob.alpha = self.showsRestingGhost ? 0.52 : 0.0
            self.ring.layer.borderColor = UIColor(white: 1.0, alpha: 0.32).cgColor
            self.knob.backgroundColor = UIColor(white: 0.95, alpha: 0.34)
            self.sprintBadge.alpha = 0
        }
        updateSprintBadgePosition()
        if wasSprinting {
            onAutoSprintChanged?(false)
        }
        onAxesChanged?(0, 0)
    }

    private func updateStick(for touch: UITouch) {
        let pt = touch.location(in: self)
        let maxR = max(1, (ringDiameter - knobDiameter) * 0.5 + 8)
        var dx = pt.x - anchorCenter.x
        var dy = pt.y - anchorCenter.y
        var dist = hypot(dx, dy)

        // Gentle follow-leash: dragging past maxR + 20 pt smoothly pulls the ring so reversing direction stays instant.
        let leash = maxR + 20
        if dist > leash {
            let excess = dist - leash
            let pullX = (dx / dist) * excess
            let pullY = (dy / dist) * excess
            anchorCenter = clampedAnchor(CGPoint(x: anchorCenter.x + pullX, y: anchorCenter.y + pullY))
            ring.center = anchorCenter
            updateSprintBadgePosition()
            dx = pt.x - anchorCenter.x
            dy = pt.y - anchorCenter.y
            dist = hypot(dx, dy)
        }

        let clampedR = min(dist, maxR)
        let rawMag = clampedR / maxR
        let dirX: CGFloat = dist > 0.001 ? dx / dist : 0
        let dirY: CGFloat = dist > 0.001 ? dy / dist : 0
        knob.center = CGPoint(x: anchorCenter.x + dirX * clampedR, y: anchorCenter.y + dirY * clampedR)

        // Precision response curve: tiny deadzone (0.07) + gentle exponent (1.30) for smooth slow-walk control.
        let deadzone: CGFloat = 0.07
        let shapedMag: CGFloat
        if rawMag <= deadzone {
            shapedMag = 0
        } else {
            let norm = (rawMag - deadzone) / (1.0 - deadzone)
            shapedMag = pow(min(1.0, norm), 1.30)
        }

        // XInput Y axis is +Y up; UIKit is +Y down.
        let outX = Float(dirX * shapedMag)
        let outY = Float(-dirY * shapedMag)
        onAxesChanged?(outX, outY)

        if autoSprintEnabled {
            // Engage auto-sprint when pushing strongly forward past the outer ring edge.
            let pushingForward = outY > 0.68 && abs(outX) < 0.72
            let wantSprint = sprintEngaged
                ? (outY > 0.55 && rawMag > 0.78)
                : (pushingForward && dist >= maxR + 6)
            if wantSprint != sprintEngaged {
                sprintEngaged = wantSprint
                if sprintEngaged {
                    hapticSprint.impactOccurred(intensity: 0.75)
                }
                updateSprintVisual()
                onAutoSprintChanged?(sprintEngaged)
            }
        }
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard activeTouch == nil, let t = touches.first else { return }
        activeTouch = t
        onUserInteraction?()
        hapticLight.prepare()
        hapticSprint.prepare()
        hapticLight.impactOccurred(intensity: 0.35)

        anchorCenter = clampedAnchor(t.location(in: self))
        ring.center = anchorCenter
        knob.center = anchorCenter
        updateSprintBadgePosition()

        UIView.animate(withDuration: 0.08, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
            self.ring.alpha = 1.0
            self.knob.alpha = 1.0
            self.ring.layer.borderColor = UIColor.white.withAlphaComponent(0.62).cgColor
            self.knob.backgroundColor = UIColor.white.withAlphaComponent(0.48)
        }
        onAxesChanged?(0, 0)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        updateStick(for: active)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        activeTouch = nil
        resetVisualAndOutput()
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        touchesEnded(touches, with: event)
    }
}

// MARK: - Direct Touch-Drag Camera Trackpad Zone (60/120 Hz Smooth Right-Stick Mapper)

/// Weak target proxy preventing a retain cycle between `CADisplayLink` and `CameraLookZone`.
private final class DisplayLinkProxy: NSObject {
    weak var target: CameraLookZone?
    init(_ target: CameraLookZone) { self.target = target }
    @objc func tick(_ link: CADisplayLink) { target?.onDisplayTick(link) }
}

/// Full-right-side touch-drag camera look surface that translates native iOS touch velocity
/// into buttery, zero-drift XInput right-stick axes (and accepts drag-to-aim deltas from RT/RB).
private final class CameraLookZone: UIView {
    private let thumbHalo = UIView()
    private var activeTouch: UITouch?
    private var lastPoint: CGPoint = .zero
    private var displayLink: CADisplayLink?

    // Accumulated UIKit-point deltas since last display tick (fed by direct touch OR button drag-aim).
    private var accumDX: CGFloat = 0
    private var accumDY: CGFloat = 0
    private var lastDeltaTime: CFTimeInterval = 0
    private var smoothVX: CGFloat = 0
    private var smoothVY: CGFloat = 0
    private var lastOutX: Float = 0
    private var lastOutY: Float = 0
    private var externalDragActive = false

    var sensitivity: Float = 1.15
    var invertY: Bool = false

    var onAxesChanged: ((Float, Float) -> Void)?
    var onUserInteraction: (() -> Void)?

    override init(frame: CGRect) {
        super.init(frame: frame)
        backgroundColor = .clear
        isMultipleTouchEnabled = false
        isExclusiveTouch = false
        accessibilityIdentifier = "overlay.look.pad"
        accessibilityLabel = "Camera Look Pad"

        thumbHalo.bounds = CGRect(x: 0, y: 0, width: 46, height: 46)
        thumbHalo.layer.cornerRadius = 23
        thumbHalo.backgroundColor = UIColor.white.withAlphaComponent(0.10)
        thumbHalo.layer.borderWidth = 1.0
        thumbHalo.layer.borderColor = UIColor.white.withAlphaComponent(0.28).cgColor
        thumbHalo.alpha = 0
        thumbHalo.isUserInteractionEnabled = false
        addSubview(thumbHalo)
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    deinit {
        stopDisplayLink()
    }

    override func willMove(toWindow newWindow: UIWindow?) {
        super.willMove(toWindow: newWindow)
        if newWindow == nil {
            forceRelease()
            stopDisplayLink()
        }
    }

    private func ensureDisplayLinkRunning() {
        if displayLink == nil {
            let link = CADisplayLink(target: DisplayLinkProxy(self), selector: #selector(DisplayLinkProxy.tick(_:)))
            link.preferredFrameRateRange = CAFrameRateRange(minimum: 60, maximum: 120, preferred: 120)
            link.add(to: .main, forMode: .common)
            displayLink = link
        }
        displayLink?.isPaused = false
    }

    private func stopDisplayLink() {
        displayLink?.invalidate()
        displayLink = nil
    }

    /// Accepts drag-to-aim deltas forwarded while holding `RT` or `RB`.
    func injectExternalDelta(dx: CGFloat, dy: CGFloat) {
        externalDragActive = true
        accumDX += dx
        accumDY += dy
        lastDeltaTime = CACurrentMediaTime()
        ensureDisplayLinkRunning()
    }

    func endExternalDrag() {
        guard externalDragActive else { return }
        externalDragActive = false
        if activeTouch == nil {
            stopAndZero()
        }
    }

    func forceRelease() {
        activeTouch = nil
        externalDragActive = false
        UIView.animate(withDuration: 0.12, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
            self.thumbHalo.alpha = 0
        }
        stopAndZero()
    }

    private func stopAndZero() {
        accumDX = 0
        accumDY = 0
        smoothVX = 0
        smoothVY = 0
        displayLink?.isPaused = true
        if lastOutX != 0 || lastOutY != 0 {
            lastOutX = 0
            lastOutY = 0
            onAxesChanged?(0, 0)
        }
    }

    fileprivate func onDisplayTick(_ link: CADisplayLink) {
        let now = CACurrentMediaTime()
        let dt = max(1.0 / 120.0, min(1.0 / 24.0, link.targetTimestamp - link.timestamp))
        let dx = accumDX
        let dy = accumDY
        accumDX = 0
        accumDY = 0

        let idleSinceDelta = now - lastDeltaTime
        if dx != 0 || dy != 0 {
            let instVX = dx / CGFloat(dt)
            let instVY = dy / CGFloat(dt)
            // Responsive low-latency blend (0.68 new / 0.32 previous) smooths 60 Hz touch jitter without lag.
            smoothVX = smoothVX * 0.32 + instVX * 0.68
            smoothVY = smoothVY * 0.32 + instVY * 0.68
        } else if idleSinceDelta > 0.018 {
            // Fast clean stop when finger pauses (zero floaty drift).
            smoothVX *= 0.42
            smoothVY *= 0.42
            if hypot(smoothVX, smoothVY) < 8.0 {
                smoothVX = 0
                smoothVY = 0
            }
        }

        // Convert UIKit pt/s velocity into normalized XInput stick deflection using a dual-slope
        // precision + flick response curve (fine micro-aim at low speed, full 1.0 turn on quick swipes).
        let speed = hypot(smoothVX, smoothVY)
        let outX: Float
        let outY: Float
        if speed < 5.0 {
            outX = 0
            outY = 0
        } else {
            let sens = CGFloat(max(0.4, min(2.4, sensitivity)))
            let refSpeed: CGFloat = 540.0 / sens
            let norm = min(1.0, speed / refSpeed)
            // Slight precision curve (0.85 exponent boosts light thumb movements past game deadzones smoothly).
            let mag = min(1.0, pow(norm, 0.82))
            let nx = (smoothVX / speed) * mag
            let ny = (smoothVY / speed) * mag
            outX = Float(nx)
            // XInput right-stick +Y tilts camera UP; UIKit drag UP is negative dy.
            outY = Float(invertY ? ny : -ny)
        }

        if abs(outX - lastOutX) > 0.002 || abs(outY - lastOutY) > 0.002 || (outX == 0 && lastOutX != 0) || (outY == 0 && lastOutY != 0) {
            lastOutX = outX
            lastOutY = outY
            onAxesChanged?(outX, outY)
        }

        if activeTouch == nil && !externalDragActive && outX == 0 && outY == 0 {
            displayLink?.isPaused = true
        }
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard activeTouch == nil, let t = touches.first else { return }
        activeTouch = t
        lastPoint = t.location(in: self)
        lastDeltaTime = CACurrentMediaTime()
        accumDX = 0
        accumDY = 0
        onUserInteraction?()

        thumbHalo.center = lastPoint
        UIView.animate(withDuration: 0.08, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
            self.thumbHalo.alpha = 1.0
        }
        ensureDisplayLinkRunning()
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        let coalesced = event?.coalescedTouches(for: active) ?? [active]
        for ct in coalesced {
            let pt = ct.location(in: self)
            accumDX += pt.x - lastPoint.x
            accumDY += pt.y - lastPoint.y
            lastPoint = pt
        }
        lastDeltaTime = CACurrentMediaTime()
        thumbHalo.center = lastPoint
        ensureDisplayLinkRunning()
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let active = activeTouch, touches.contains(active) else { return }
        activeTouch = nil
        UIView.animate(withDuration: 0.14, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
            self.thumbHalo.alpha = 0
        }
        if !externalDragActive {
            stopAndZero()
        }
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        touchesEnded(touches, with: event)
    }
}

// MARK: - Compact Translucent Controls Settings Popover

/// Compact frosted-glass settings drawer toggled from the HUD pill so players can tune
/// Swipe vs Stick look, sensitivity, opacity, auto-sprint, LT aim toggle, and haptics live.
private final class ControlsSettingsCard: UIView {
    var onPrefsChanged: ((TouchControlPrefs) -> Void)?
    var onHapticsToggle: ((Bool) -> Void)?
    var onClose: (() -> Void)?

    private var prefs: TouchControlPrefs
    private let stack = UIStackView()
    private let lookSeg = UISegmentedControl(items: ["Swipe Look", "Right Stick"])
    private let sensSlider = UISlider()
    private let sensVal = UILabel()
    private let opacitySlider = UISlider()
    private let opacityVal = UILabel()
    private let autoSprintSwitch = UISwitch()
    private let ltToggleSwitch = UISwitch()
    private let invertYSwitch = UISwitch()
    private let hapticsSwitch = UISwitch()

    init(prefs: TouchControlPrefs, hapticsOn: Bool) {
        self.prefs = prefs
        super.init(frame: .zero)

        backgroundColor = UIColor(white: 0.08, alpha: 0.88)
        layer.cornerRadius = 16
        layer.borderWidth = 1.0
        layer.borderColor = UIColor(white: 1.0, alpha: 0.22).cgColor
        layer.shadowColor = UIColor.black.cgColor
        layer.shadowOpacity = 0.55
        layer.shadowRadius = 16
        layer.shadowOffset = CGSize(width: 0, height: 6)

        stack.axis = .vertical
        stack.spacing = 8
        stack.layoutMargins = UIEdgeInsets(top: 12, left: 14, bottom: 12, right: 14)
        stack.isLayoutMarginsRelativeArrangement = true
        addSubview(stack)

        // Header
        let header = UIStackView()
        header.axis = .horizontal
        header.alignment = .center
        let titleLbl = UILabel()
        titleLbl.text = "Touch Controls"
        titleLbl.font = .systemFont(ofSize: 13, weight: .bold)
        titleLbl.textColor = .white
        let resetBtn = UIButton(type: .system)
        resetBtn.setTitle("Reset", for: .normal)
        resetBtn.titleLabel?.font = .systemFont(ofSize: 12, weight: .semibold)
        resetBtn.tintColor = .systemTeal
        resetBtn.addTarget(self, action: #selector(tapReset), for: .touchUpInside)
        let doneBtn = UIButton(type: .system)
        doneBtn.setTitle("Done", for: .normal)
        doneBtn.titleLabel?.font = .systemFont(ofSize: 12, weight: .bold)
        doneBtn.tintColor = .white
        doneBtn.addTarget(self, action: #selector(tapDone), for: .touchUpInside)
        header.addArrangedSubview(titleLbl)
        header.addArrangedSubview(UIView())
        header.addArrangedSubview(resetBtn)
        header.setCustomSpacing(12, after: resetBtn)
        header.addArrangedSubview(doneBtn)
        stack.addArrangedSubview(header)

        // Look style segmented control
        lookSeg.selectedSegmentIndex = prefs.lookStyle.rawValue
        lookSeg.backgroundColor = UIColor(white: 1.0, alpha: 0.12)
        lookSeg.selectedSegmentTintColor = UIColor.systemTeal.withAlphaComponent(0.55)
        lookSeg.setTitleTextAttributes([.foregroundColor: UIColor.white, .font: UIFont.systemFont(ofSize: 11, weight: .semibold)], for: .normal)
        lookSeg.addTarget(self, action: #selector(changedLookStyle), for: .valueChanged)
        stack.addArrangedSubview(lookSeg)

        // Sliders
        sensSlider.minimumValue = 0.4
        sensSlider.maximumValue = 2.4
        sensSlider.minimumTrackTintColor = .systemTeal
        sensSlider.addTarget(self, action: #selector(changedSens), for: .valueChanged)
        stack.addArrangedSubview(makeSliderRow(title: "Look Speed", slider: sensSlider, valLabel: sensVal))

        opacitySlider.minimumValue = 0.25
        opacitySlider.maximumValue = 1.0
        opacitySlider.minimumTrackTintColor = .systemTeal
        opacitySlider.addTarget(self, action: #selector(changedOpacity), for: .valueChanged)
        stack.addArrangedSubview(makeSliderRow(title: "Opacity", slider: opacitySlider, valLabel: opacityVal))

        // Switches in compact 2-column grid for landscape friendliness
        autoSprintSwitch.addTarget(self, action: #selector(changedAutoSprint), for: .valueChanged)
        ltToggleSwitch.addTarget(self, action: #selector(changedLtToggle), for: .valueChanged)
        invertYSwitch.addTarget(self, action: #selector(changedInvertY), for: .valueChanged)
        hapticsSwitch.addTarget(self, action: #selector(changedHaptics), for: .valueChanged)
        hapticsSwitch.isOn = hapticsOn

        let row1 = UIStackView(arrangedSubviews: [
            makeSwitchItem(title: "Auto-Sprint", sw: autoSprintSwitch),
            makeSwitchItem(title: "LT Aim Toggle", sw: ltToggleSwitch),
        ])
        row1.axis = .horizontal
        row1.distribution = .fillEqually
        row1.spacing = 10
        stack.addArrangedSubview(row1)

        let row2 = UIStackView(arrangedSubviews: [
            makeSwitchItem(title: "Invert Look Y", sw: invertYSwitch),
            makeSwitchItem(title: "Rumble", sw: hapticsSwitch),
        ])
        row2.axis = .horizontal
        row2.distribution = .fillEqually
        row2.spacing = 10
        if !PhoneHapticsBridge.shared.supportsHardwareHaptics {
            row2.arrangedSubviews[1].alpha = 0.35
            hapticsSwitch.isEnabled = false
        }
        stack.addArrangedSubview(row2)

        syncWidgets()
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    override func layoutSubviews() {
        super.layoutSubviews()
        stack.frame = bounds
    }

    func updateHapticsState(_ on: Bool) {
        hapticsSwitch.isOn = on
    }

    private func makeSliderRow(title: String, slider: UISlider, valLabel: UILabel) -> UIView {
        let row = UIStackView()
        row.axis = .horizontal
        row.alignment = .center
        row.spacing = 8
        let lbl = UILabel()
        lbl.text = title
        lbl.font = .systemFont(ofSize: 11, weight: .medium)
        lbl.textColor = UIColor(white: 0.88, alpha: 1.0)
        lbl.widthAnchor.constraint(equalToConstant: 70).isActive = true

        valLabel.font = .monospacedDigitSystemFont(ofSize: 11, weight: .semibold)
        valLabel.textColor = .systemTeal
        valLabel.textAlignment = .right
        valLabel.widthAnchor.constraint(equalToConstant: 36).isActive = true

        row.addArrangedSubview(lbl)
        row.addArrangedSubview(slider)
        row.addArrangedSubview(valLabel)
        return row
    }

    private func makeSwitchItem(title: String, sw: UISwitch) -> UIView {
        sw.onTintColor = .systemTeal
        sw.transform = CGAffineTransform(scaleX: 0.72, y: 0.72)
        let row = UIStackView()
        row.axis = .horizontal
        row.alignment = .center
        row.spacing = 4
        let lbl = UILabel()
        lbl.text = title
        lbl.font = .systemFont(ofSize: 11, weight: .medium)
        lbl.textColor = UIColor(white: 0.88, alpha: 1.0)
        lbl.adjustsFontSizeToFitWidth = true
        lbl.minimumScaleFactor = 0.8
        row.addArrangedSubview(lbl)
        row.addArrangedSubview(sw)
        return row
    }

    private func syncWidgets() {
        lookSeg.selectedSegmentIndex = prefs.lookStyle.rawValue
        sensSlider.value = prefs.lookSensitivity
        sensVal.text = String(format: "%.1fx", prefs.lookSensitivity)
        opacitySlider.value = prefs.opacity
        opacityVal.text = "\(Int((prefs.opacity * 100).rounded()))%"
        autoSprintSwitch.isOn = prefs.autoSprint
        ltToggleSwitch.isOn = prefs.leftTriggerToggle
        invertYSwitch.isOn = prefs.invertY
    }

    private func commit() {
        prefs.save()
        syncWidgets()
        onPrefsChanged?(prefs)
    }

    @objc private func changedLookStyle() {
        prefs.lookStyle = CameraLookStyle(rawValue: lookSeg.selectedSegmentIndex) ?? .swipe
        commit()
    }
    @objc private func changedSens() {
        prefs.lookSensitivity = sensSlider.value
        commit()
    }
    @objc private func changedOpacity() {
        prefs.opacity = opacitySlider.value
        commit()
    }
    @objc private func changedAutoSprint() {
        prefs.autoSprint = autoSprintSwitch.isOn
        commit()
    }
    @objc private func changedLtToggle() {
        prefs.leftTriggerToggle = ltToggleSwitch.isOn
        commit()
    }
    @objc private func changedInvertY() {
        prefs.invertY = invertYSwitch.isOn
        commit()
    }
    @objc private func changedHaptics() {
        onHapticsToggle?(hapticsSwitch.isOn)
    }
    @objc private func tapReset() {
        prefs = TouchControlPrefs()
        commit()
    }
    @objc private func tapDone() {
        onClose?()
    }
}

// MARK: - Main Touch Overlay Host

/// Fullscreen pass-through touch-overlay host rendering ergonomic console-grade multi-touch
/// XInput controls (floating movement joystick with outer auto-sprint, direct swipe-to-look
/// camera pad with drag-to-aim triggers, curved thumb-roll ABXY cluster, sculpted D-pad)
/// plus a compact auto-dimming top HUD pill and live settings popover.
final class TouchOverlayView: UIView {
    var onKeyboardToggle: (() -> Void)?
    var onModeChanged: ((TouchOverlayMode) -> Void)?
    var onHapticsToggle: (() -> Bool)?

    private(set) var mode: TouchOverlayMode = .gamepad
    private var prefs = TouchControlPrefs.load()
    private var userLockedMode = false
    private var keyboardStatus = NativeKeyboardStatus()
    private var physicalControllerConnected = false
    private var hapticsEnabled = PhoneHapticsBridge.shared.supportsHardwareHaptics
    private var settingsOpen = false
    private var idleDimWorkItem: DispatchWorkItem?

    // Live XInput virtual state pushed to `port_ios_gamepad_state`.
    private var buttonBits: UInt16 = 0
    private var faceBits: UInt16 = 0
    private var dpadBits: UInt16 = 0
    private var autoSprintActive = false
    private var leftTrigger: UInt8 = 0
    private var rightTrigger: UInt8 = 0
    private var lx: Int16 = 0
    private var ly: Int16 = 0
    private var rx: Int16 = 0
    private var ry: Int16 = 0

    // Containers & HUD
    private let controlsContainer = UIView()
    private let hudBar = UIStackView()
    private let modeButton = UIButton(type: .system)
    private let settingsButton = UIButton(type: .system)
    private let keyboardButton = UIButton(type: .system)
    private lazy var settingsCard = ControlsSettingsCard(prefs: prefs, hapticsOn: hapticsEnabled)

    // Ergonomic full-zone touch surfaces (sit underneath buttons so empty-canvas touches work cleanly)
    private let moveZone = FloatingStickZone(ringDiameter: 112, id: "overlay.stick.left", label: "Left Stick")
    private let lookSwipeZone = CameraLookZone()
    private let rightStickZone = FloatingStickZone(ringDiameter: 108, id: "overlay.stick.right", label: "Right Stick")

    // Tactile button clusters
    private let abxy = ABXYClusterView()
    private let dpad = OverlayDPad()
    private let btnL2 = OverlayButton(
        title: "LT",
        accent: UIColor(red: 0.45, green: 0.85, blue: 0.96, alpha: 1.0),
        circular: false,
        fontSize: 13,
        hapticStyle: .medium,
        hitExpansion: 8,
        id: "overlay.btn.LT"
    )
    private let btnL1 = OverlayButton(
        title: "LB",
        accent: UIColor(white: 0.92, alpha: 1.0),
        circular: false,
        fontSize: 12,
        hitExpansion: 7,
        id: "overlay.btn.LB"
    )
    private let btnR2 = OverlayButton(
        title: "RT",
        accent: UIColor(red: 1.0, green: 0.62, blue: 0.36, alpha: 1.0),
        circular: true,
        fontSize: 14,
        hapticStyle: .medium,
        hitExpansion: 10,
        id: "overlay.btn.RT"
    )
    private let btnR1 = OverlayButton(
        title: "RB",
        accent: UIColor(white: 0.92, alpha: 1.0),
        circular: false,
        fontSize: 12,
        hitExpansion: 8,
        id: "overlay.btn.RB"
    )
    private let btnL3 = OverlayButton(
        title: "L3",
        accent: UIColor(white: 0.78, alpha: 1.0),
        circular: true,
        fontSize: 11,
        hitExpansion: 6,
        id: "overlay.btn.L3"
    )
    private let btnR3 = OverlayButton(
        title: "R3",
        accent: UIColor(white: 0.78, alpha: 1.0),
        circular: true,
        fontSize: 11,
        hitExpansion: 6,
        id: "overlay.btn.R3"
    )
    private let btnBack = OverlayButton(
        title: nil,
        symbol: "rectangle.on.rectangle",
        accent: UIColor(white: 0.82, alpha: 1.0),
        circular: false,
        fontSize: 12,
        hitExpansion: 8,
        id: "overlay.btn.VIEW"
    )
    private let btnStart = OverlayButton(
        title: nil,
        symbol: "line.3.horizontal",
        accent: UIColor(white: 0.82, alpha: 1.0),
        circular: false,
        fontSize: 13,
        hitExpansion: 8,
        id: "overlay.btn.MENU"
    )

    private var allButtons: [OverlayButton] {
        [btnL1, btnL2, btnR1, btnR2, btnL3, btnR3, btnBack, btnStart]
    }
    private var observers: [NSObjectProtocol] = []

    override init(frame: CGRect) {
        super.init(frame: frame)
        backgroundColor = .clear
        isMultipleTouchEnabled = true
        setupControls()
        setupHudBar()
        setupSettingsCard()
        observeSystemEvents()
        applyPrefs()
        refreshVisibility(animated: false)
        pushGamepadState()
        scheduleIdleDim()
    }

    required init?(coder: NSCoder) { fatalError("init(coder:) has not been implemented") }

    deinit {
        idleDimWorkItem?.cancel()
        for obs in observers {
            NotificationCenter.default.removeObserver(obs)
        }
        port_ios_gamepad_state(false, 0, 0, 0, 0, 0, 0, 0)
    }

    /// Pass touches through to `MetalSurfaceView` when controls are hidden or in `.touchMouse` mode;
    /// in `.gamepad` mode, `moveZone` and `lookSwipeZone`/`rightStickZone` claim empty-canvas touches
    /// so stray misses never trigger accidental Windows mouse clicks.
    override func hitTest(_ point: CGPoint, with event: UIEvent?) -> UIView? {
        if settingsOpen {
            let pCard = convert(point, to: settingsCard)
            if settingsCard.bounds.contains(pCard) {
                return settingsCard.hitTest(pCard, with: event)
            }
            let pHud = convert(point, to: hudBar)
            if hudBar.bounds.contains(pHud) {
                return hudBar.hitTest(pHud, with: event)
            }
            // Tapping outside the open settings card dismisses it cleanly.
            setSettingsOpen(false)
            return self
        }
        let hit = super.hitTest(point, with: event)
        if hit === self || hit === controlsContainer {
            return nil
        }
        return hit
    }

    func updateKeyboardStatus(_ status: NativeKeyboardStatus) {
        keyboardStatus = status
        refreshHudLabels()
        if status.softwareVisible {
            setSettingsOpen(false)
            releaseAllVirtualInputs()
        }
        refreshVisibility(animated: true)
    }

    // MARK: - Setup

    private func setupControls() {
        controlsContainer.backgroundColor = .clear
        controlsContainer.isMultipleTouchEnabled = true
        addSubview(controlsContainer)

        // Add full-zone surfaces FIRST so buttons sit cleanly on top in Z-order.
        controlsContainer.addSubview(moveZone)
        controlsContainer.addSubview(lookSwipeZone)
        controlsContainer.addSubview(rightStickZone)
        controlsContainer.addSubview(dpad)
        controlsContainer.addSubview(abxy)
        for b in allButtons {
            controlsContainer.addSubview(b)
        }

        // Enable drag-to-aim right on RT & RB so you can aim while shooting / boosting flashlight!
        btnR2.supportsDragAim = true
        btnR1.supportsDragAim = true
        let forwardAimDelta: (CGFloat, CGFloat) -> Void = { [weak self] dx, dy in
            guard let self, self.prefs.lookStyle == .swipe else { return }
            self.lookSwipeZone.injectExternalDelta(dx: dx, dy: dy)
        }
        let forwardAimEnd: () -> Void = { [weak self] in
            self?.lookSwipeZone.endExternalDrag()
        }
        btnR2.onAimDragDelta = forwardAimDelta
        btnR2.onAimDragEnded = forwardAimEnd
        btnR1.onAimDragDelta = forwardAimDelta
        btnR1.onAimDragEnded = forwardAimEnd

        let wake: () -> Void = { [weak self] in self?.noteActiveTouch() }
        for b in allButtons { b.onUserInteraction = wake }
        abxy.onUserInteraction = wake
        dpad.onUserInteraction = wake
        moveZone.onUserInteraction = wake
        lookSwipeZone.onUserInteraction = wake
        rightStickZone.onUserInteraction = wake

        bindButton(btnL1, bit: UInt16(PORT_IOS_BTN_LEFT_SHOULDER))
        bindButton(btnR1, bit: UInt16(PORT_IOS_BTN_RIGHT_SHOULDER))
        bindButton(btnL3, bit: UInt16(PORT_IOS_BTN_LEFT_THUMB))
        bindButton(btnR3, bit: UInt16(PORT_IOS_BTN_RIGHT_THUMB))
        bindButton(btnBack, bit: UInt16(PORT_IOS_BTN_BACK))
        bindButton(btnStart, bit: UInt16(PORT_IOS_BTN_START))

        btnL2.onPressChanged = { [weak self] down in
            self?.leftTrigger = down ? 255 : 0
            self?.pushGamepadState()
        }
        btnR2.onPressChanged = { [weak self] down in
            self?.rightTrigger = down ? 255 : 0
            self?.pushGamepadState()
        }
        abxy.onMaskChanged = { [weak self] mask in
            self?.faceBits = mask
            self?.pushGamepadState()
        }
        dpad.onMaskChanged = { [weak self] mask in
            self?.dpadBits = mask
            self?.pushGamepadState()
        }
        moveZone.onAxesChanged = { [weak self] x, y in
            self?.lx = Self.axisToI16(x)
            self?.ly = Self.axisToI16(y)
            self?.pushGamepadState()
        }
        moveZone.onAutoSprintChanged = { [weak self] sprinting in
            self?.autoSprintActive = sprinting
            self?.pushGamepadState()
        }
        lookSwipeZone.onAxesChanged = { [weak self] x, y in
            self?.rx = Self.axisToI16(x)
            self?.ry = Self.axisToI16(y)
            self?.pushGamepadState()
        }
        rightStickZone.onAxesChanged = { [weak self] x, y in
            guard let self else { return }
            let adjustedY = self.prefs.invertY ? -y : y
            self.rx = Self.axisToI16(x)
            self.ry = Self.axisToI16(adjustedY)
            self.pushGamepadState()
        }
    }

    private func bindButton(_ button: OverlayButton, bit: UInt16) {
        button.onPressChanged = { [weak self] down in
            guard let self else { return }
            if down {
                self.buttonBits |= bit
            } else {
                self.buttonBits &= ~bit
            }
            self.pushGamepadState()
        }
    }

    private func setupHudBar() {
        hudBar.axis = .horizontal
        hudBar.alignment = .center
        hudBar.spacing = 4
        hudBar.layoutMargins = UIEdgeInsets(top: 3, left: 6, bottom: 3, right: 6)
        hudBar.isLayoutMarginsRelativeArrangement = true
        hudBar.backgroundColor = UIColor(white: 0.08, alpha: 0.58)
        hudBar.layer.cornerRadius = 15
        hudBar.layer.borderWidth = 0.75
        hudBar.layer.borderColor = UIColor(white: 1.0, alpha: 0.20).cgColor
        addSubview(hudBar)

        modeButton.addTarget(self, action: #selector(cycleMode), for: .touchUpInside)
        settingsButton.addTarget(self, action: #selector(toggleSettings), for: .touchUpInside)
        keyboardButton.addTarget(self, action: #selector(tapKeyboard), for: .touchUpInside)

        hudBar.addArrangedSubview(modeButton)
        hudBar.addArrangedSubview(settingsButton)
        hudBar.addArrangedSubview(keyboardButton)
        refreshHudLabels()
    }

    private func setupSettingsCard() {
        settingsCard.alpha = 0
        settingsCard.isHidden = true
        settingsCard.onPrefsChanged = { [weak self] newPrefs in
            guard let self else { return }
            self.prefs = newPrefs
            self.applyPrefs()
            self.setNeedsLayout()
        }
        settingsCard.onHapticsToggle = { [weak self] wantOn in
            guard let self else { return }
            if wantOn != self.hapticsEnabled, let onHapticsToggle = self.onHapticsToggle {
                self.hapticsEnabled = onHapticsToggle()
                self.settingsCard.updateHapticsState(self.hapticsEnabled)
            }
        }
        settingsCard.onClose = { [weak self] in
            self?.setSettingsOpen(false)
        }
        addSubview(settingsCard)
    }

    private func stylePillButton(_ b: UIButton, title: String?, symbol: String?, id: String, label: String, highlighted: Bool) {
        var cfg = UIButton.Configuration.plain()
        cfg.contentInsets = NSDirectionalEdgeInsets(top: 4, leading: title != nil ? 8 : 7, bottom: 4, trailing: title != nil ? 8 : 7)
        cfg.baseForegroundColor = .white
        cfg.background.backgroundColor = highlighted
            ? UIColor.systemTeal.withAlphaComponent(0.42)
            : UIColor(white: 1.0, alpha: 0.11)
        cfg.background.cornerRadius = 11
        if let title {
            var attr = AttributedString(title)
            attr.font = .systemFont(ofSize: 11, weight: .semibold)
            cfg.attributedTitle = attr
        }
        if let symbol {
            let symCfg = UIImage.SymbolConfiguration(pointSize: 11, weight: .semibold)
            cfg.image = UIImage(systemName: symbol, withConfiguration: symCfg)
            cfg.imagePadding = title != nil ? 4 : 0
        }
        b.configuration = cfg
        b.accessibilityIdentifier = id
        b.accessibilityLabel = label
    }

    // MARK: - Preferences, Idle Auto-Dim & Lifecycle

    private func applyPrefs() {
        moveZone.autoSprintEnabled = prefs.autoSprint
        btnL2.toggleMode = prefs.leftTriggerToggle
        lookSwipeZone.sensitivity = prefs.lookSensitivity
        lookSwipeZone.invertY = prefs.invertY

        let isSwipe = (prefs.lookStyle == .swipe)
        lookSwipeZone.isHidden = !isSwipe
        rightStickZone.isHidden = isSwipe
        rightStickZone.showsRestingGhost = !isSwipe
        if isSwipe {
            rightStickZone.forceRelease()
        } else {
            lookSwipeZone.forceRelease()
        }
        refreshVisibility(animated: true)
    }

    private func noteActiveTouch() {
        idleDimWorkItem?.cancel()
        if mode == .gamepad && !keyboardStatus.softwareVisible {
            let targetAlpha = CGFloat(prefs.opacity)
            if abs(controlsContainer.alpha - targetAlpha) > 0.02 {
                UIView.animate(withDuration: 0.12, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
                    self.controlsContainer.alpha = targetAlpha
                }
            }
        }
        scheduleIdleDim()
    }

    private func scheduleIdleDim() {
        idleDimWorkItem?.cancel()
        let item = DispatchWorkItem { [weak self] in
            guard let self, !self.settingsOpen else { return }
            UIView.animate(withDuration: 0.45, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState]) {
                self.hudBar.alpha = (self.mode == .hidden && !self.keyboardStatus.softwareVisible) ? 0.34 : 0.46
                if self.mode == .gamepad && !self.keyboardStatus.softwareVisible {
                    self.controlsContainer.alpha = CGFloat(self.prefs.opacity) * 0.78
                }
            }
        }
        idleDimWorkItem = item
        DispatchQueue.main.asyncAfter(deadline: .now() + 3.5, execute: item)
    }

    private func setSettingsOpen(_ open: Bool) {
        guard settingsOpen != open else { return }
        settingsOpen = open
        if open {
            releaseAllVirtualInputs()
            settingsCard.isHidden = false
            hudBar.alpha = 0.96
            idleDimWorkItem?.cancel()
        } else {
            scheduleIdleDim()
        }
        refreshHudLabels()
        UIView.animate(
            withDuration: 0.18,
            delay: 0,
            options: [.allowUserInteraction, .beginFromCurrentState],
            animations: {
                self.settingsCard.alpha = open ? 1.0 : 0.0
                self.settingsCard.transform = open ? .identity : CGAffineTransform(scaleX: 0.95, y: 0.95)
            },
            completion: { _ in
                if !self.settingsOpen {
                    self.settingsCard.isHidden = true
                }
            }
        )
    }

    private func observeSystemEvents() {
        let nc = NotificationCenter.default
        let updatePad = { [weak self] in
            guard let self else { return }
            let hasRealPad = GCController.controllers().contains { ctrl in
                guard ctrl.extendedGamepad != nil, !ctrl.isSnapshot else { return false }
                let vendor = ctrl.vendorName ?? ""
                return !vendor.isEmpty && vendor != "Gamepad"
            }
            self.physicalControllerConnected = hasRealPad
            if !self.userLockedMode {
                let nextMode: TouchOverlayMode = hasRealPad ? .hidden : .gamepad
                if self.mode != nextMode {
                    self.mode = nextMode
                    self.onModeChanged?(nextMode)
                }
            }
            self.refreshHudLabels()
            self.refreshVisibility(animated: true)
            self.pushGamepadState()
        }
        observers.append(nc.addObserver(forName: .GCControllerDidConnect, object: nil, queue: .main) { _ in updatePad() })
        observers.append(nc.addObserver(forName: .GCControllerDidDisconnect, object: nil, queue: .main) { _ in updatePad() })
        observers.append(nc.addObserver(forName: UIApplication.willResignActiveNotification, object: nil, queue: .main) { [weak self] _ in
            self?.releaseAllVirtualInputs()
        })
        updatePad()
    }

    @objc private func cycleMode() {
        userLockedMode = true
        setSettingsOpen(false)
        mode = mode.next()
        if mode != .gamepad {
            releaseAllVirtualInputs()
        }
        hudBar.alpha = 0.95
        refreshHudLabels()
        refreshVisibility(animated: true)
        pushGamepadState()
        onModeChanged?(mode)
        scheduleIdleDim()
    }

    @objc private func toggleSettings() {
        setSettingsOpen(!settingsOpen)
    }

    @objc private func tapKeyboard() {
        setSettingsOpen(false)
        hudBar.alpha = 0.95
        onKeyboardToggle?()
        scheduleIdleDim()
    }

    private func refreshHudLabels() {
        stylePillButton(
            modeButton,
            title: mode == .hidden ? nil : mode.shortLabel,
            symbol: mode.symbolName,
            id: "overlay.mode",
            label: "Overlay Mode",
            highlighted: false
        )
        stylePillButton(
            settingsButton,
            title: nil,
            symbol: "slider.horizontal.3",
            id: "overlay.settings",
            label: "Touch Controls Settings",
            highlighted: settingsOpen
        )
        let kbTitle: String? = keyboardStatus.softwareVisible ? "Hide" : (keyboardStatus.hardwareConnected ? "HW" : nil)
        stylePillButton(
            keyboardButton,
            title: kbTitle,
            symbol: "keyboard",
            id: "overlay.keyboard",
            label: "Toggle Keyboard",
            highlighted: keyboardStatus.softwareVisible || keyboardStatus.hardwareConnected
        )

        let compact = (mode == .hidden && !keyboardStatus.softwareVisible)
        settingsButton.isHidden = compact || mode != .gamepad
        keyboardButton.isHidden = compact
        setNeedsLayout()
    }

    private func refreshVisibility(animated: Bool) {
        let showControls = (mode == .gamepad) && !keyboardStatus.softwareVisible
        let targetControlsAlpha = showControls ? CGFloat(prefs.opacity) : 0.0
        let targetHudAlpha: CGFloat = settingsOpen ? 0.96 : ((mode == .hidden && !keyboardStatus.softwareVisible) ? 0.42 : 0.85)
        let apply = {
            self.controlsContainer.alpha = targetControlsAlpha
            self.controlsContainer.isUserInteractionEnabled = showControls
            self.hudBar.alpha = targetHudAlpha
        }
        if animated {
            UIView.animate(withDuration: 0.18, delay: 0, options: [.allowUserInteraction, .beginFromCurrentState], animations: apply)
        } else {
            apply()
        }
    }

    private func releaseAllVirtualInputs() {
        for b in allButtons { b.forceRelease() }
        abxy.forceRelease()
        dpad.forceRelease()
        moveZone.forceRelease()
        lookSwipeZone.forceRelease()
        rightStickZone.forceRelease()
        buttonBits = 0
        faceBits = 0
        dpadBits = 0
        autoSprintActive = false
        leftTrigger = 0
        rightTrigger = 0
        lx = 0; ly = 0; rx = 0; ry = 0
        pushGamepadState()
    }

    private func pushGamepadState() {
        let virtualActive = (mode == .gamepad) && !keyboardStatus.softwareVisible
        let sprintBit: UInt16 = autoSprintActive ? UInt16(PORT_IOS_BTN_LEFT_THUMB) : 0
        let combinedButtons = buttonBits | faceBits | dpadBits | sprintBit
        port_ios_gamepad_state(
            virtualActive,
            combinedButtons,
            leftTrigger,
            rightTrigger,
            lx,
            ly,
            rx,
            ry
        )
    }

    private static func axisToI16(_ v: Float) -> Int16 {
        let c = max(-1.0, min(1.0, v))
        if c < 0 {
            return Int16(max(-32768.0, (c * 32768.0).rounded()))
        }
        return Int16(min(32767.0, (c * 32767.0).rounded()))
    }

    // MARK: - Responsive Ergonomic Layout (Landscape & Portrait)

    override func layoutSubviews() {
        super.layoutSubviews()
        controlsContainer.frame = bounds

        let safe = safeAreaInsets
        let w = bounds.width
        let h = bounds.height
        let isLandscape = w > h

        // Compact top-center HUD pill sitting safely below Dynamic Island / status notch.
        let hudSize = hudBar.systemLayoutSizeFitting(UIView.layoutFittingCompressedSize)
        hudBar.frame = CGRect(
            x: (w - hudSize.width) * 0.5,
            y: max(safe.top + 6, 8),
            width: hudSize.width,
            height: max(hudSize.height, 30)
        )

        // Floating settings card directly below HUD pill.
        let cardW = min(312, w - 32)
        let cardH: CGFloat = 228
        settingsCard.frame = CGRect(
            x: (w - cardW) * 0.5,
            y: hudBar.frame.maxY + 8,
            width: cardW,
            height: cardH
        )

        let leftMargin = max(safe.left, 16) + 10
        let rightMargin = max(safe.right, 16) + 10
        let bottomMargin = max(safe.bottom, 12) + 10
        let bottomY = h - bottomMargin

        if isLandscape {
            // Full-half touch zones beneath the buttons so dropping a thumb anywhere feels natural.
            let zoneTop = hudBar.frame.maxY + 4
            let splitX = w * 0.46
            moveZone.frame = CGRect(x: 0, y: zoneTop, width: splitX, height: h - zoneTop)
            lookSwipeZone.frame = CGRect(x: splitX, y: zoneTop, width: w - splitX, height: h - zoneTop)
            rightStickZone.frame = lookSwipeZone.frame

            // Left-thumb resting anchor (lower-outer-left ergonomic sweet spot).
            let leftRestGlobal = CGPoint(x: leftMargin + 72, y: bottomY - 72)
            moveZone.restingCenter = convert(leftRestGlobal, to: moveZone)

            // Sculpted D-pad tucked cleanly to the inner-bottom-left (reachable without crowding stick).
            let dpadSize: CGFloat = 86
            dpad.frame = CGRect(x: leftMargin + 152, y: bottomY - dpadSize - 4, width: dpadSize, height: dpadSize)

            // Ergonomic Left Shoulder/Trigger arc: LT prominent for aim, LB neatly beside it.
            let triggerW: CGFloat = 68
            let triggerH: CGFloat = 38
            let shoulderY = max(zoneTop + 18, h * 0.24)
            btnL2.frame = CGRect(x: leftMargin, y: shoulderY, width: triggerW, height: triggerH)
            btnL1.frame = CGRect(x: btnL2.frame.maxX + 10, y: shoulderY + 2, width: 54, height: 34)
            btnL3.frame = CGRect(x: leftMargin + 2, y: btnL2.frame.maxY + 14, width: 34, height: 34)

            // Right-thumb curved ABXY cluster in the natural outer-lower-right arc.
            let abxySize: CGFloat = 158
            abxy.frame = CGRect(
                x: w - rightMargin - abxySize + 4,
                y: bottomY - abxySize - 6,
                width: abxySize,
                height: abxySize
            )

            // Prominent circular RT (Fire / Primary) perched along the upper-inner right thumb arc
            // (supports hold-and-drag camera aiming right on the button!).
            let rtSize: CGFloat = 60
            btnR2.frame = CGRect(
                x: abxy.frame.minX - rtSize * 0.62,
                y: abxy.frame.minY - rtSize * 0.36,
                width: rtSize,
                height: rtSize
            )
            // RB (Flashlight boost / shoulder) cleanly beside RT along the upper-right arc (also drag-to-aim!).
            btnR1.frame = CGRect(
                x: w - rightMargin - 64,
                y: shoulderY + 2,
                width: 64,
                height: 36
            )
            btnR3.frame = CGRect(
                x: btnR1.frame.minX - 44,
                y: shoulderY + 3,
                width: 34,
                height: 34
            )

            // Classic right-stick resting anchor (used only when user selects "Right Stick" look mode).
            let rightRestGlobal = CGPoint(x: abxy.frame.minX - 64, y: bottomY - 64)
            rightStickZone.restingCenter = convert(rightRestGlobal, to: rightStickZone)

            // Compact View & Menu pills centered cleanly along the bottom safe edge.
            let pillW: CGFloat = 44
            let pillH: CGFloat = 26
            btnBack.frame = CGRect(x: w * 0.5 - pillW - 12, y: bottomY - pillH + 2, width: pillW, height: pillH)
            btnStart.frame = CGRect(x: w * 0.5 + 12, y: bottomY - pillH + 2, width: pillW, height: pillH)
        } else {
            // Portrait responsive layout: lower half hosts touch controls, upper half stays clean.
            let zoneTop = h * 0.44
            let splitX = w * 0.5
            moveZone.frame = CGRect(x: 0, y: zoneTop, width: splitX, height: h - zoneTop)
            lookSwipeZone.frame = CGRect(x: splitX, y: zoneTop, width: w - splitX, height: h - zoneTop)
            rightStickZone.frame = lookSwipeZone.frame

            let pillW: CGFloat = 46
            let pillH: CGFloat = 26
            btnBack.frame = CGRect(x: w * 0.5 - pillW - 14, y: bottomY - pillH, width: pillW, height: pillH)
            btnStart.frame = CGRect(x: w * 0.5 + 14, y: bottomY - pillH, width: pillW, height: pillH)

            let stickY = btnBack.frame.minY - 70
            moveZone.restingCenter = convert(CGPoint(x: leftMargin + 62, y: stickY), to: moveZone)
            rightStickZone.restingCenter = convert(CGPoint(x: w - rightMargin - 62, y: stickY), to: rightStickZone)

            let dpadSize: CGFloat = 82
            dpad.frame = CGRect(x: leftMargin, y: stickY - 66 - dpadSize, width: dpadSize, height: dpadSize)

            let abxySize: CGFloat = 144
            abxy.frame = CGRect(x: w - rightMargin - abxySize, y: stickY - 56 - abxySize, width: abxySize, height: abxySize)

            let shoulderY = hudBar.frame.maxY + 14
            btnL2.frame = CGRect(x: leftMargin, y: shoulderY, width: 62, height: 36)
            btnL1.frame = CGRect(x: btnL2.frame.maxX + 8, y: shoulderY + 1, width: 52, height: 34)
            btnR2.frame = CGRect(x: w - rightMargin - 58, y: shoulderY, width: 58, height: 58)
            btnR1.frame = CGRect(x: btnR2.frame.minX - 60, y: shoulderY + 2, width: 52, height: 34)
            btnL3.frame = CGRect(x: leftMargin + 126, y: stickY + 20, width: 32, height: 32)
            btnR3.frame = CGRect(x: w - rightMargin - 158, y: stickY + 20, width: 32, height: 32)
        }
    }
}
