import UIKit

/// Minimal fullscreen UIKit host controller embedding `MetalSurfaceView`,
/// multi-touch XInput `TouchOverlayView`, native hardware/software `NativeKeyboardBridge`,
/// and launching the universal `port-loader` runtime on a background thread.
final class PortViewController: UIViewController {
    private let metalView = MetalSurfaceView(frame: .zero)
    private let touchOverlay = TouchOverlayView(frame: .zero)
    private var keyboardBridge: NativeKeyboardBridge?
    private var launched = false

    override func loadView() {
        let root = UIView(frame: .zero)
        root.backgroundColor = .black
        root.isMultipleTouchEnabled = true

        metalView.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        root.addSubview(metalView)

        touchOverlay.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        root.addSubview(touchOverlay)

        view = root
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        PhoneHapticsBridge.shared.start()

        let kb = NativeKeyboardBridge(hostView: view)
        kb.onStatusChanged = { [weak self] status in
            self?.touchOverlay.updateKeyboardStatus(status)
        }
        touchOverlay.onKeyboardToggle = { [weak kb] in
            kb?.toggleSoftKeyboard()
        }
        touchOverlay.onModeChanged = { [weak self] mode in
            self?.metalView.touchAsMouseEnabled = (mode == .touchMouse)
        }
        metalView.touchAsMouseEnabled = (touchOverlay.mode == .touchMouse)
        touchOverlay.onHapticsToggle = {
            PhoneHapticsBridge.shared.setEnabled(!PhoneHapticsBridge.shared.isEnabled)
        }
        touchOverlay.updateKeyboardStatus(kb.status)
        keyboardBridge = kb
    }

    override var canBecomeFirstResponder: Bool { true }
    override var prefersStatusBarHidden: Bool { true }
    override var prefersHomeIndicatorAutoHidden: Bool { true }
    override var preferredScreenEdgesDeferringSystemGestures: UIRectEdge { .all }
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask { .landscape }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        metalView.frame = view.bounds
        touchOverlay.frame = view.bounds
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        UIApplication.shared.isIdleTimerDisabled = true
        becomeFirstResponder()
        guard !launched else { return }
        launched = true

        // Pass NULL to auto-discover `<App>.app/game.toml`, `<App>.app/Game`,
        // `<App>.app/ShaderCache`, and `~/Documents/Translator`.
        let rc = port_ios_start_background(nil)
        if rc != 0 {
            NSLog("PortHost: port_ios_start_background returned %d", rc)
        }
    }

    override func pressesBegan(_ presses: Set<UIPress>, with event: UIPressesEvent?) {
        if keyboardBridge?.handlePressesBegan(presses) == true {
            return
        }
        super.pressesBegan(presses, with: event)
    }

    override func pressesEnded(_ presses: Set<UIPress>, with event: UIPressesEvent?) {
        if keyboardBridge?.handlePressesEnded(presses) == true {
            return
        }
        super.pressesEnded(presses, with: event)
    }

    override func pressesCancelled(_ presses: Set<UIPress>, with event: UIPressesEvent?) {
        if keyboardBridge?.handlePressesEnded(presses) == true {
            return
        }
        super.pressesCancelled(presses, with: event)
    }
}

/// Modern `UIWindowSceneDelegate` (required on iOS 26/27+ SDKs).
final class PortSceneDelegate: UIResponder, UIWindowSceneDelegate {
    var window: UIWindow?

    func scene(
        _ scene: UIScene,
        willConnectTo session: UISceneSession,
        options connectionOptions: UIScene.ConnectionOptions
    ) {
        guard let windowScene = scene as? UIWindowScene else { return }
        let win = UIWindow(windowScene: windowScene)
        win.rootViewController = PortViewController()
        win.makeKeyAndVisible()
        self.window = win
    }

    func sceneDidDisconnect(_ scene: UIScene) {
        PhoneHapticsBridge.shared.stop()
        port_ios_detach_surface()
    }
}

@main
final class PortAppDelegate: UIResponder, UIApplicationDelegate {
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        application.isIdleTimerDisabled = true
        return true
    }

    func application(
        _ application: UIApplication,
        configurationForConnecting connectingSceneSession: UISceneSession,
        options: UIScene.ConnectionOptions
    ) -> UISceneConfiguration {
        let config = UISceneConfiguration(
            name: "Default Configuration",
            sessionRole: connectingSceneSession.role
        )
        config.delegateClass = PortSceneDelegate.self
        return config
    }

    func applicationDidReceiveMemoryWarning(_ application: UIApplication) {
        port_ios_memory_warning()
    }

    func applicationWillTerminate(_ application: UIApplication) {
        PhoneHapticsBridge.shared.stop()
        port_ios_request_exit(0)
    }
}
