use std::thread;

use chrono::{Utc, Duration, SecondsFormat};
use chrono_tz::Tz;
use rand::{prelude::SliceRandom, thread_rng};
use sysinfo::{System, RefreshKind, CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, MemoryRefreshKind};

/// Returns the current time with the supplied `seconds_format`
pub fn time_now_rfc3339zulu(seconds_format: SecondsFormat) -> String {
    Utc::now().to_rfc3339_opts(seconds_format, true)
}

pub fn time_now_rfc3339_with_timezone(seconds_format: SecondsFormat, timezone: Tz) -> String {
    Utc::now().with_timezone(&timezone).to_rfc3339_opts(seconds_format, false)
}

#[allow(dead_code)]
fn ran_small_num() -> u8 { 
    let lib = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let mut rng = thread_rng(); 
    let ran_num = lib.choose(&mut rng).unwrap(); 
    return *ran_num; 
}

/// Computes the timestamp of current time + Duration
pub fn next_update_time(interval: Duration) -> String {
    let now = Utc::now().checked_add_signed(interval);
    if now.is_some() {
        return now.unwrap().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    } else {
        return Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    }
}

/// Reads the cpu usage and returns it for each core, as well as the avg system load.
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

/// Returns a touple containing two touples;
/// The first contains the current memory usage and swap usage in percent in that order,
/// The second contains the total memory and swap in bytes in that order.
pub fn get_ram_data() -> ((f64, f64), (u64, u64), u64) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));
    sys.refresh_memory();
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_memory();
    // Sysinfo's doc only advices a double update for cpu data.
    let (mem, total_mem, used_mem, swap, total_swap) = {
        let total_mem = sys.total_memory();
        let (mem, used_mem) = {
            let used_mem = sys.used_memory();
            let expr = format!("({} / ({} / 100))", used_mem, total_mem);
            let t = mexprp::eval::<f64>(&expr).unwrap().to_vec();
            (t[0], used_mem)
        };
        let total_swap = sys.total_swap();
        let swap = {
            let used_swap = sys.used_swap();
            let expr = format!("({} / ({} / 100))", used_swap, total_swap);
            let t = mexprp::eval::<f64>(&expr).unwrap().to_vec();
            t[0]
        };
        (mem, total_mem, used_mem, swap, total_swap)
    };
    return ((mem, swap), (total_mem, total_swap), used_mem);
}

/// Computes the cpu core amount. Should the request for cores fail or be bigger than 255, 1 is returned.
pub fn get_cpu_core_amount() -> u8 {
    let sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything().without_frequency()));
    if TryInto::<u8>::try_into(sys.cpus().len()).is_ok() {
        return TryInto::<u8>::try_into(sys.cpus().len()).unwrap();
    } else {
        return 1;
    }
}
