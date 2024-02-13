use std::path::PathBuf;

use eframe::egui::IconData;
use gui::start_nyx;
use utils::settings::Settings;

mod gui;
mod comp;
mod utils;

const APPNAME: &str = env!("CARGO_PKG_NAME");
const APPVERSION: &str = env!("CARGO_PKG_VERSION");
const APPAUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let pic_path = dirs::picture_dir().unwrap();
    let icon_path = pic_path.join("logo.jpeg");
    let settings = Settings::default();
    start_nyx(load_icon(icon_path), settings);
}

fn load_icon(path: PathBuf)-> IconData {
    let (icon_rgba, icon_width, icon_hight) = {
        let patht = path.as_path();
        let image = image::open(patht).expect("Failed to open path!").into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    IconData { rgba: icon_rgba, width: icon_width, height: icon_hight }
}
