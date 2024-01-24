use chrono::{SecondsFormat, Utc};
use rand::{prelude::SliceRandom, thread_rng};
use gui::start_nyx;

mod gui;
mod comp;

const APPNAME: &str = env!("CARGO_PKG_NAME");
const APPVERSION: &str = env!("CARGO_PKG_VERSION");
const APPAUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    start_nyx();
}

pub fn time_now_rfc3339zulu(seconds_format: SecondsFormat) -> String {
    Utc::now().to_rfc3339_opts(seconds_format, true)
}

fn ran_small_num() -> u8 { 
    let lib = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let mut rng = thread_rng(); 
    let ran_num = lib.choose(&mut rng).unwrap(); 
    return *ran_num; 
}

