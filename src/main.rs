use gui::start_nyx;

mod gui;
mod comp;
mod utils;

const APPNAME: &str = env!("CARGO_PKG_NAME");
const APPVERSION: &str = env!("CARGO_PKG_VERSION");
const APPAUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    start_nyx();
}

