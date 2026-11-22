//! `dxil-metald`: out-of-process Metal Shader Converter worker (native arm64).
//!
//! Spawned by [`dxil_metal::helper::HelperClient`] (the D3D12 layer in the x86_64 loader, or
//! `dxil-metal prewarm` with `AW2_SHADER_BACKEND=helper`); speaks the framed protocol on
//! stdin/stdout and exits at EOF. Not meant to be run by hand.

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--version") {
        match dxil_metal::ffi::api() {
            Ok(api) => println!("{} protocol v{} ({})", dxil_metal::helper::HELPER_NAME, dxil_metal::helper::PROTOCOL_VERSION, api.fingerprint),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        return;
    }
    dxil_metal::helper::install_daemon_lifecycle_guards();
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();
    if let Err(e) = dxil_metal::helper::serve(stdin, stdout) {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            return;
        }
        eprintln!("{}: {e}", dxil_metal::helper::HELPER_NAME);
        std::process::exit(2);
    }
}
