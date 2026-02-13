use std::time::Instant;

use hermes::Hermes;
use athena::{XffValue, Object};
use nyx_backend::{error::NyxResult, gathering::{df_gatherer, docker_gatherer, free_gatherer, ps_gatherer, shamash_gatherer, uptime_gatherer}};

fn main() {
    // Space for future startup code
    

}

/// All setup code that can panic or fail
fn setup_gathering_server() {
    std::thread::spawn(move || {
        let con = Hermes::new(".nxy_data/gathering").expect("Failed to create Hermes Server");
        let mut running = true;
        let mut last_run = Instant::now();
        while running {
            if con.is_request_ready() {
                let request = con.await_request();
                match request {
                    Ok(request) => {
                        if request.is_null() {
                            running = false;
                        }
                    },
                    Err(err) => {
                        let err = XffValue::from(format!("Failed to get request: {:?}", err));
                        if let Err(err) = con.put_error(err) {panic!("{:?}", err)};
                    }
                }
            }

            if !last_run.elapsed().as_millis() > 750 {
                continue
            }
            last_run = Instant::now();

            match gather() {
                Ok(value) => {
                    let mut value = value.into_object().expect("Failed to convert to object");
                    value.insert("time", last_run.elapsed().as_millis());
                    if let Err(err) = con.respond(value.into()) {panic!("{:?}", err)};
                },
                Err(err) => {
                    let err = XffValue::from(format!("Failed to gather: {:?}", err));
                    if let Err(err) = con.put_error(err) {panic!("{:?}", err)};
                }
            }

        }
    });
}

fn gather() -> NyxResult<XffValue> {

    let df_gathered = df_gatherer()?;
    let docker_gathered = docker_gatherer()?;
    let free_gathered = free_gatherer()?;
    let ps_gathered = ps_gatherer()?;
    let uptime_gathered = uptime_gatherer()?;
    let shamash_gathered = shamash_gatherer()?;

    let mut obj = Object::new();
    obj.insert("df", df_gathered);
    obj.insert("docker", docker_gathered);
    obj.insert("free", free_gathered);
    obj.insert("ps", ps_gathered);
    obj.insert("uptime", uptime_gathered);
    obj.insert("shamash", shamash_gathered);
    Ok(obj.into())
}
