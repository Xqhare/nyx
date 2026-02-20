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
    while run {
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
        if let Some(XffValue::Null) = draw_state(state.clone(), &mut talos) {
            run = false;
        }
        //std::thread::sleep(std::time::Duration::from_millis(16));
    }
    let _ = con.request(XffValue::Null);
}
