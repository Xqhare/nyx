use athena::XffValue;
use hermes::Hermes;
use nyx_tui::draw_state;
use talos::Talos;

use crate::gathering::setup_gathering_server;

mod gathering;

const GATHER_SERVER: &str = ".nxy_data/gathering";
const GATHER_INTERVAL: u128 = 750;

fn main() {
    // Space for future startup code
    
    setup_gathering_server();

    let mut run = true;
    let mut con = Hermes::new(GATHER_SERVER).expect("Failed to create Hermes Server");
    let mut state: XffValue = XffValue::Null;
    let mut talos = Talos::builder().build().expect("Failed to create Talos");
    let mut first_iter = true;
    while run {
        if first_iter {
            let response = con.await_response();
            match response {
                Ok(response) => {
                    state = response;
                },
                Err(err) => {
                    drop(talos);
                    eprintln!("Fatal: Failed to get initial response from gathering server: {:?}", err);
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
                },
                Err(err) => {
                    drop(talos);
                    eprintln!("Fatal: Lost connection or error from gathering server: {:?}", err);
                    std::process::exit(1);
                }
            }
        }
        match draw_state(state.clone(), &mut talos) {
            Some(state) => {
                if state == XffValue::Null {
                    run = false;
                } 
            }
            None => {}
        }
    }
}

