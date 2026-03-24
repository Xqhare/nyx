use std::{
    thread,
    time::{Duration, Instant},
};

use athena::{Object, XffValue};
use hermes::Hermes;
use nyx_backend::{
    error::NyxResult,
    gathering::{
        df_gatherer, docker_gatherer, free_gatherer, lasa_gatherer, ps_gatherer, shamash_gatherer,
        uptime_gatherer,
    },
};

use crate::GATHER_SERVER;

const TARGET_UPS: u64 = 5;

/// All setup code that can panic or fail
pub fn setup_gathering_server() {
    std::thread::spawn(move || {
        let mut con = Hermes::new(GATHER_SERVER).expect("Failed to create Hermes Server");
        con.set_garbage_collection(true);
        let mut running = true;
        let ups_duration = Duration::from_nanos(1_000_000_000 / TARGET_UPS);
        while running {
            let last_run = Instant::now();
            if con.is_request_ready() {
                let request = con.await_request();
                match request {
                    Ok(request) => {
                        if request.is_null() {
                            running = false;
                        }
                    }
                    Err(err) => {
                        let err = XffValue::from(format!("Failed to get request: {err:?}"));
                        if let Err(err) = con.put_error(err) {
                            panic!("{err:?}")
                        }
                    }
                }
            }

            match gather() {
                Ok(value) => {
                    let mut value = value.into_object().expect("Failed to convert to object");
                    value.insert(
                        "time",
                        XffValue::from(last_run.elapsed().as_micros().to_string()),
                    );
                    if let Err(err) = con.respond(value.into()) {
                        panic!("{err:?}")
                    }
                }
                Err(err) => {
                    let err_val = XffValue::from(format!("Failed to gather: {err:?}"));
                    panic!("Gathering thread panicked: {err_val:?}");
                }
            }

            let elapsed = last_run.elapsed();
            if elapsed < ups_duration {
                thread::sleep(ups_duration.checked_sub(elapsed).unwrap());
            }
        }
    });
}

fn gather() -> NyxResult<XffValue> {
    let df_gathered = df_gatherer()?;
    let docker_gathered = match docker_gatherer() {
        Ok(docker_gathered) => docker_gathered,
        Err(err) => XffValue::from(format!("Failed to gather docker: {err}")),
    };
    let free_gathered = free_gatherer()?;
    let ps_gathered = ps_gatherer()?;
    let uptime_gathered = uptime_gatherer()?;
    let shamash_gathered = match shamash_gatherer() {
        Ok(shamash_gathered) => {
            if shamash_gathered.is_null() {
                XffValue::from("Shamash not installed!")
            } else {
                shamash_gathered
            }
        }
        Err(_) => XffValue::from("Shamash not installed!"),
    };
    let lasa_gathered = match lasa_gatherer() {
        Ok(lasa_gathered) => lasa_gathered,
        Err(_) => XffValue::from("Lasa not installed:"),
    };

    let mut obj = Object::new();
    obj.insert("df", df_gathered);
    obj.insert("docker", docker_gathered);
    obj.insert("free", free_gathered);
    obj.insert("ps", ps_gathered);
    obj.insert("uptime", uptime_gathered);
    obj.insert("shamash", shamash_gathered);
    obj.insert("lasa", lasa_gathered);
    Ok(obj.into())
}
