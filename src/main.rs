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
    let config_path = dirs::config_dir().unwrap();
    let setting_path = config_path.join("nyxconfig.json");
    let icon_path = pic_path.join("logo.jpeg");
    let settings = Settings::load(setting_path);
    let test = utils::utils::get_process_data_new();
    if test.is_some() {
        for thing in test.unwrap() {
            println!("{:?}", thing);
        }
    }
    if settings.is_ok() {
        start_nyx(load_icon(icon_path), settings.unwrap());
    } else {
        start_nyx(load_icon(icon_path), Settings::default());
    }
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
