import QuartzCore
import UIKit

/// Fullscreen `CAMetalLayer`-backed `UIView` that binds directly to `libport_loader.a`
/// and forwards resize, orientation, and touch events to `win32_rt::platform::uikit`.
final class MetalSurfaceView: UIView {
    override class var layerClass: AnyClass {
        CAMetalLayer.self
    }

    var metalLayer: CAMetalLayer {
        layer as! CAMetalLayer
    }

    private var surfaceAttached = false
    private var lastReportedWidthPx: UInt32 = 0
    private var lastReportedHeightPx: UInt32 = 0
    private var lastReportedScale: Double = 0
    private var displayLink: CADisplayLink?
    var touchAsMouseEnabled: Bool = true
    private var singleTouchDown = false
    private var singleTouchLastPx: CGPoint = .zero
    private var twoFingerActive = false
    private var twoFingerDidScroll = false
    private var twoFingerRightClickFired = false
    private var twoFingerStartCentroid: CGPoint = .zero
    private var twoFingerLastCentroid: CGPoint = .zero
    private var twoFingerAccumX: CGFloat = 0
    private var twoFingerAccumY: CGFloat = 0

    override init(frame: CGRect) {
        super.init(frame: frame)
        commonInit()
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
        commonInit()
    }

    private func commonInit() {
        backgroundColor = .black
        isMultipleTouchEnabled = true
        metalLayer.isOpaque = true
        metalLayer.contentsGravity = .resizeAspect
    }

    override func didMoveToWindow() {
        super.didMoveToWindow()
        if window != nil {
            attachOrResizeSurface()
            startDisplayLink()
        } else {
            stopDisplayLink()
            if surfaceAttached {
                port_ios_detach_surface()
                surfaceAttached = false
            }
        }
    }

    override func layoutSubviews() {
        super.layoutSubviews()
        attachOrResizeSurface()
    }

    private func attachOrResizeSurface() {
        guard bounds.width > 0, bounds.height > 0 else { return }
        let scale = Double(window?.windowScene?.screen.nativeScale ?? traitCollection.displayScale)
        let effectiveScale = scale > 0 ? scale : 1.0
        contentScaleFactor = CGFloat(effectiveScale)
        metalLayer.contentsScale = CGFloat(effectiveScale)

        let widthPx = max(1, UInt32((bounds.width * CGFloat(effectiveScale)).rounded()))
        let heightPx = max(1, UInt32((bounds.height * CGFloat(effectiveScale)).rounded()))

        if !surfaceAttached {
            metalLayer.drawableSize = CGSize(width: CGFloat(widthPx), height: CGFloat(heightPx))
            let layerPtr = Unmanaged.passUnretained(metalLayer).toOpaque()
            port_ios_attach_surface(layerPtr, widthPx, heightPx, effectiveScale)
            surfaceAttached = true
            lastReportedWidthPx = widthPx
            lastReportedHeightPx = heightPx
            lastReportedScale = effectiveScale
        } else if widthPx != lastReportedWidthPx
            || heightPx != lastReportedHeightPx
            || abs(effectiveScale - lastReportedScale) > 0.001
        {
            // Do not overwrite `metalLayer.drawableSize` directly here: `d3d12-metal`'s
            // swapchain owns `drawableSize` (at the normalized logical display resolution).
            port_ios_resize_surface(widthPx, heightPx, effectiveScale)
            lastReportedWidthPx = widthPx
            lastReportedHeightPx = heightPx
            lastReportedScale = effectiveScale
        }
    }

    private func startDisplayLink() {
        guard displayLink == nil else { return }
        let link = CADisplayLink(target: self, selector: #selector(onTick))
        link.preferredFrameRateRange = CAFrameRateRange(minimum: 30, maximum: 120, preferred: 60)
        link.add(to: .main, forMode: .common)
        displayLink = link
    }

    private func stopDisplayLink() {
        displayLink?.invalidate()
        displayLink = nil
    }

    @objc private func onTick() {
        port_ios_tick()
    }

    private func centroid(of touches: Set<UITouch>) -> CGPoint {
        var sx: CGFloat = 0
        var sy: CGFloat = 0
        var count: CGFloat = 0
        for t in touches {
            let p = t.location(in: self)
            sx += p.x
            sy += p.y
            count += 1
        }
        return count > 0 ? CGPoint(x: sx / count, y: sy / count) : .zero
    }

    private func forward(_ touches: Set<UITouch>, phase: UInt32, event: UIEvent?) {
        guard touchAsMouseEnabled else { return }
        let allTouches = event?.touches(for: self) ?? touches
        let activeTouches = allTouches.filter {
            $0.phase != .ended && $0.phase != .cancelled
        }
        let scale = Float(contentScaleFactor)

        if !twoFingerActive && allTouches.count >= 2 {
            // Transition cleanly from any in-flight 1-finger press into 2-finger mode
            // (2-finger drag = WM_MOUSEWHEEL scroll; 2-finger tap = right-click).
            if singleTouchDown {
                port_ios_touch_event(
                    UInt32(PORT_IOS_TOUCH_CANCELLED),
                    Float(singleTouchLastPx.x),
                    Float(singleTouchLastPx.y)
                )
                singleTouchDown = false
            }
            let c = centroid(of: activeTouches.isEmpty ? allTouches : activeTouches)
            twoFingerActive = true
            twoFingerDidScroll = false
            twoFingerRightClickFired = false
            twoFingerStartCentroid = c
            twoFingerLastCentroid = c
            twoFingerAccumX = 0
            twoFingerAccumY = 0
            return
        }

        if twoFingerActive {
            let endedOrCancelled =
                phase == UInt32(PORT_IOS_TOUCH_ENDED) || phase == UInt32(PORT_IOS_TOUCH_CANCELLED)
            if phase == UInt32(PORT_IOS_TOUCH_MOVED) && activeTouches.count >= 2 {
                let c = centroid(of: activeTouches)
                let distFromStart = hypot(
                    c.x - twoFingerStartCentroid.x,
                    c.y - twoFingerStartCentroid.y
                )
                if !twoFingerDidScroll && distFromStart >= 6.0 {
                    twoFingerDidScroll = true
                }
                if twoFingerDidScroll {
                    let dx = c.x - twoFingerLastCentroid.x
                    let dy = c.y - twoFingerLastCentroid.y
                    // Natural iOS scroll: dragging fingers up (dy < 0) scrolls content down
                    // (negative WM_MOUSEWHEEL delta); dragging left (dx < 0) scrolls right (+HWHEEL).
                    twoFingerAccumY += dy
                    twoFingerAccumX -= dx
                    let ptsPerDetent: CGFloat = 14.0
                    let notchesY = Int32((twoFingerAccumY / ptsPerDetent).rounded(.towardZero))
                    let notchesX = Int32((twoFingerAccumX / ptsPerDetent).rounded(.towardZero))
                    if notchesY != 0 || notchesX != 0 {
                        twoFingerAccumY -= CGFloat(notchesY) * ptsPerDetent
                        twoFingerAccumX -= CGFloat(notchesX) * ptsPerDetent
                        port_ios_scroll_wheel(
                            Float(c.x) * scale,
                            Float(c.y) * scale,
                            notchesY * 120,
                            notchesX * 120
                        )
                    }
                }
                twoFingerLastCentroid = c
            } else if endedOrCancelled {
                if !twoFingerDidScroll && !twoFingerRightClickFired
                    && phase == UInt32(PORT_IOS_TOUCH_ENDED)
                {
                    let c = twoFingerStartCentroid
                    let px = Float(c.x) * scale
                    let py = Float(c.y) * scale
                    port_ios_mouse_button(UInt32(PORT_IOS_MOUSE_RIGHT), true, px, py)
                    port_ios_mouse_button(UInt32(PORT_IOS_MOUSE_RIGHT), false, px, py)
                    twoFingerRightClickFired = true
                }
                let remaining = activeTouches.subtracting(touches)
                if remaining.isEmpty {
                    twoFingerActive = false
                    twoFingerDidScroll = false
                    twoFingerRightClickFired = false
                    twoFingerAccumX = 0
                    twoFingerAccumY = 0
                }
            }
            return
        }

        guard let touch = touches.first else { return }
        let pt = touch.location(in: self)
        let px = Float(pt.x) * scale
        let py = Float(pt.y) * scale
        if phase == UInt32(PORT_IOS_TOUCH_BEGAN) {
            singleTouchDown = true
            singleTouchLastPx = CGPoint(x: CGFloat(px), y: CGFloat(py))
        } else if phase == UInt32(PORT_IOS_TOUCH_MOVED) {
            singleTouchLastPx = CGPoint(x: CGFloat(px), y: CGFloat(py))
        } else {
            singleTouchDown = false
        }
        port_ios_touch_event(phase, px, py)
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        forward(touches, phase: UInt32(PORT_IOS_TOUCH_BEGAN), event: event)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        forward(touches, phase: UInt32(PORT_IOS_TOUCH_MOVED), event: event)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        forward(touches, phase: UInt32(PORT_IOS_TOUCH_ENDED), event: event)
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        forward(touches, phase: UInt32(PORT_IOS_TOUCH_CANCELLED), event: event)
    }
}
