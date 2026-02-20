use std::time::Instant;

use athena::XffValue;
use hermes::Hermes;
use nyx_tui::draw_state;
use talos::Talos;

use crate::gathering::setup_gathering_server;

mod gathering;

const GATHER_SERVER: &str = ".nxy_data/gathering";
const GATHER_INTERVAL: u128 = 750;

fn main() {
    eprintln!("Checkpoint 1: Starting main");
    setup_gathering_server();

    let mut run = true;
    let con = Hermes::new(GATHER_SERVER).expect("Failed to create Hermes Server");
    let mut state: XffValue = XffValue::Null;
    eprintln!("Checkpoint 2: Building Talos");
    let mut talos = Talos::builder().build().expect("Failed to create Talos");
    eprintln!("Checkpoint 3: Entering loop");
    let mut first_iter = true;
    let mut last_run = Instant::now();
    // Not 0 because of the first iteration - would div by 0 otherwise
    let mut gui_run_dur = 1;
    while run {
        last_run = Instant::now();
        if first_iter {
            let response = con.await_response();
            match response {
                Ok(response) => {
                    state = response;
                }
                Err(err) => {
                    drop(talos);
                    eprintln!(
                        "Fatal: Failed to get initial response from gathering server: {:?}",
                        err
                    );
                    std::process::exit(1);
                }
            }
            first_iter = false;
        }
        if con.is_response_ready() {
            let response = con.await_response();
            match response {
                Ok(response) => {
                    state = response;
                }
                Err(err) => {
                    drop(talos);
                    eprintln!(
                        "Fatal: Lost connection or error from gathering server: {:?}",
                        err
                    );
                    std::process::exit(1);
                }
            }
        }
        if let Some(XffValue::Null) = draw_state(gui_run_dur.to_string(), state.clone(), &mut talos) {
            run = false;
        }
        gui_run_dur = last_run.elapsed().as_micros();
    }
    let _ = con.request(XffValue::Null);
}
