//! `wwise-audio` CLI: bank-info / decode / wem-info / inventory / compare / validate / render / plan / list-2d / batch / nulltest.
mod cli;
mod inventory;
mod render;
mod validate;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
