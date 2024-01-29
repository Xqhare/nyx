use std::thread;

use chrono::{Utc, Duration, SecondsFormat};
use rand::{prelude::SliceRandom, thread_rng};
use sysinfo::{System, RefreshKind, CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL};

pub fn time_now_rfc3339zulu(seconds_format: SecondsFormat) -> String {
    Utc::now().to_rfc3339_opts(seconds_format, true)
}

#[allow(dead_code)]
fn ran_small_num() -> u8 { 
    let lib = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let mut rng = thread_rng(); 
    let ran_num = lib.choose(&mut rng).unwrap(); 
    return *ran_num; 
}

pub fn next_update_time(interval: Duration) -> String {
    let now = Utc::now().checked_add_signed(interval);
    if now.is_some() {
        return now.unwrap().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    } else {
        return Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    }
}

pub fn get_cpu_data() -> (Vec<f64>, f64) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything().without_frequency()));
    sys.refresh_cpu_usage();
    // because of some funky shit sysinfo does internally, it is adviced to update twice for consistent data.
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    // Real work
    let mut tmp_store: Vec<f64> = Default::default();
    for cpu in sys.cpus() {
        tmp_store.push(cpu.cpu_usage() as f64);
    }
    // reason for own ang_load calc: Ref D1
    let avg = {
        let sum = {
            let mut out: f64 = 0.0;
            for n in &tmp_store {
                out = out + n;
            }
            out
        };
        sum / tmp_store.len() as f64
    };
    return (tmp_store, avg);
}

pub fn get_cpu_core_amount() -> u8 {
    let sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything().without_frequency()));
    let phy_cores = sys.physical_core_count();
    if phy_cores.is_some() {
        let out = phy_cores.unwrap().checked_mul(2);
        if out.is_none() {
            return 255;
        } else {
            return out.unwrap() as u8;
        }
    } else {
        return 1;
    }
}
