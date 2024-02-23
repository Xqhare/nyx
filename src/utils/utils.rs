use std::thread;

use chrono::{Utc, Duration, SecondsFormat};
use chrono_tz::Tz;
use rand::{prelude::SliceRandom, thread_rng};
use sysinfo::{System, RefreshKind, CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, MemoryRefreshKind, Disks, Networks, Components};

use procfs::{diskstats, process::Process, DiskStat};
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn get_process_amount() -> usize {
    let mut processes = System::new();
    processes.refresh_processes();
    return processes.processes().len();
}

pub fn get_temperature_data() -> Vec<Vec<(String, String, f32, f32)>> {
    let mut components = Components::new_with_refreshed_list();
    let mut out: Vec<(String, String, f32, f32)> = Default::default();
    for comp in &mut components {
        comp.refresh();
        let tmp = comp.label();
        let if_panics = (tmp, "");
        let (name, sensor) = tmp.split_once(" ").unwrap_or(if_panics);
        let current_temperature: f32 = comp.temperature();
        // I herby define 80 degrees Celsius as the critical lower bound!
        let critical_temperature: f32 = comp.critical().unwrap_or(80.0);
        out.push((name.to_string(), sensor.to_string(), current_temperature, critical_temperature));
    }
    let mut prev_name: String = Default::default();
    let mut clean_out: Vec<Vec<(String, String, f32, f32)>> = Default::default();
    for entry in out {
        if prev_name.is_empty() {
            clean_out.push(vec![entry.clone()]);
            prev_name = entry.0;
        } else {
            if prev_name == entry.0 {
                // Last should always return something, as the last inserted thing has the name of
                // this.
                let mut this_comp = clean_out.pop().unwrap().to_vec();
                this_comp.push(entry);
                clean_out.push(this_comp.to_vec());
            } else {
                clean_out.push(vec![entry.clone()]);
                prev_name = entry.0;
            }
        }
        
    }
    return clean_out;
}

pub fn get_temperature_update_data() -> Vec<Vec<(String, f32)>> {
    let components = Components::new_with_refreshed_list();
    let mut out: Vec<(String, String, f32)> = Default::default();
    for comp in &components {
        let tmp = comp.label();
        let if_panics = ("", tmp);
        let (name, sensor) = tmp.split_once(" ").unwrap_or(if_panics);
        let current_temperature: f32 = comp.temperature();
        out.push((name.to_string(), sensor.to_string(), current_temperature));
    }
    let mut prev_name: String = Default::default();
    let mut clean_out: Vec<Vec<(String, f32)>> = Default::default();
    for entry in out {
        if prev_name.is_empty() {
            clean_out.push(vec![(entry.1, entry.2)]);
            prev_name = entry.0;
        } else {
            if prev_name == entry.0 {
                // Last should always return something, as the last inserted thing has the name of
                // this.
                let mut this_comp = clean_out.pop().unwrap().to_vec();
                this_comp.push((entry.1, entry.2));
                clean_out.push(this_comp.to_vec());
            } else {
                clean_out.push(vec![(entry.1, entry.2)]);
                prev_name = entry.0;
            }
        }
        
    }
    return clean_out;
}

pub fn get_network_data() -> Vec<(String, String, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)> {
    let mut out: Vec<(String, String, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)> = Default::default();

    let networks = Networks::new_with_refreshed_list();
    for network in networks.iter() {
        let name = network.0.to_string();
        let data = network.1;
        let mac_addr = data.mac_address().to_string();
        let incoming = data.received();
        let total_incoming = data.total_received();
        let errors_incoming = data.errors_on_received();
        let total_errors_incoming = data.total_errors_on_received();
        let packets_incoming = data.packets_received();
        let total_packets_incoming = data.total_packets_received();
        let outgoing = data.transmitted();
        let total_outgoing = data.total_transmitted();
        let errors_outgoing = data.errors_on_transmitted();
        let total_errors_outgoing = data.total_errors_on_transmitted();
        let packets_outgoing = data.packets_transmitted();
        let total_packets_outgoing = data.total_packets_transmitted();
        out.push((name, mac_addr, incoming, total_incoming, errors_incoming, total_errors_incoming, packets_incoming, total_packets_incoming, outgoing, total_outgoing, errors_outgoing, total_errors_outgoing, packets_outgoing, total_packets_outgoing));
    }

    return out;
    
}

// You copied that function without understanding why it does what it does, and as a result your
// code IS GARBAGE. AGAIN. - Linus Torvalds
//
/// Returns a Vec containing all data needed to construct and update all disks detected.
/// Returned values are in order of struct definition. The stat values are dlivered as is from the
/// kernel, and need more processing before using them.
pub fn get_disk_data() -> Vec<(String, String, String, bool, String, u64, u64, u64, u64, u64)> {
    let me = Process::myself().unwrap();
    let mounts = me.mountinfo().unwrap();

    // Get a list of all disks that we have IO stat info on
    let disk_stats: HashMap<(i32, i32), DiskStat> =
        HashMap::from_iter(diskstats().unwrap().into_iter().map(|i| ((i.major, i.minor), i)));

    let mut out: Vec<(String, String, String, bool, String, u64, u64, u64, u64, u64)> = Default::default();

    let disks = Disks::new_with_refreshed_list();
    let mut tmp_disk_vec: Vec<(&str, &str, String, bool, &str, u64, u64, u64)> = Default::default();
    for disk in &disks {
        let name = disk.name().to_str().unwrap_or("");
        let filesystem = disk.file_system().to_str().unwrap_or("");
        let disk_type = disk.kind().to_string();
        let removable: bool = disk.is_removable();
        let mount = disk.mount_point().to_str().unwrap_or("");
        let used_bytes: u64 = disk.total_space() - disk.available_space();
        let free_bytes: u64 = disk.available_space();
        let total_bytes: u64 = disk.total_space();
        tmp_disk_vec.push((name, filesystem, disk_type, removable, mount, used_bytes, free_bytes, total_bytes));
    }

    for mount in mounts {
        // parse the majmin string (something like "0:3") into an (i32, i32) tuple
        let (maj, min): (i32, i32) = {
            let mut s = mount.majmin.split(':');
            (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
        };

        if let Some(stat) = disk_stats.get(&(maj, min)) {
            if !stat.name.contains("loop") {
                for disk in &tmp_disk_vec {
                    if disk.0.contains(&stat.name) {
                        let name = disk.0.to_string();
                        let filesystem = disk.1.to_string();
                        let disk_type = disk.2.to_string();
                        let removeable = disk.3;
                        let mounted_on = disk.4.to_string();
                        let used_bytes = disk.5;
                        let free_bytes = disk.6;
                        let total_bytes = disk.7;
                        let stat_reads = stat.reads;
                        let stat_writes = stat.writes;
                        out.push((name, filesystem, disk_type, removeable, mounted_on, used_bytes, free_bytes, total_bytes, stat_reads as u64, stat_writes));
                    }
                }
            }
        }
    }
    return out;
}

/// Returns a vector with a touple for each disk. The touple contains the name, used, free- bytes,
/// abs reads and abs writes.
pub fn get_disk_update_data() -> Vec<(String, u64, u64, u64, u64)>{
    let mut out: Vec<(String, u64, u64, u64, u64)> = Default::default();

    let disks = Disks::new_with_refreshed_list();
    let mut tmp_disk_vec: Vec<(String, u64, u64)> = Default::default();
    for disk in &disks {
        let name = disk.name().to_str().unwrap_or("").to_string();
        let used_bytes: u64 = disk.total_space() - disk.available_space();
        let free_bytes: u64 = disk.available_space();
        tmp_disk_vec.push((name, used_bytes, free_bytes));
    }

    let me = Process::myself().unwrap();
    let mounts = me.mountinfo().unwrap();

    // Get a list of all disks that we have IO stat info on
    let disk_stats: HashMap<(i32, i32), DiskStat> =
        HashMap::from_iter(diskstats().unwrap().into_iter().map(|i| ((i.major, i.minor), i)));

    for mount in mounts {
        // parse the majmin string (something like "0:3") into an (i32, i32) tuple
        let (maj, min): (i32, i32) = {
            let mut s = mount.majmin.split(':');
            (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
        };

        if let Some(stat) = disk_stats.get(&(maj, min)) {
            if !stat.name.contains("loop") {
                for disk in &tmp_disk_vec {
                    if disk.0.contains(&stat.name) {
                        let name = disk.0.to_string();
                        let stat_reads = stat.reads;
                        let stat_writes = stat.writes;
                        let used_bytes = disk.1;
                        let free_bytes = disk.2;
                        out.push((name, used_bytes, free_bytes, stat_reads as u64, stat_writes));
                    }
                }
            }
        }
    }
    return out;
}

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
        sum / (tmp_store.len() as f64 / 2.0)
    };
    return (tmp_store, avg);
}

/// Returns a touple containing two touples;
/// The first contains the current memory usage and swap usage in percent in that order,
/// The second contains the total memory and swap in bytes in that order.
pub fn get_ram_data() -> ((f64, f64), (u64, u64), u64, u64) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));
    sys.refresh_memory();
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_memory();
    // Sysinfo's doc only advices a double update for cpu data.
    let (mem, total_mem, used_mem, swap, total_swap, used_swap) = {
        let total_mem = sys.total_memory();
        let (mem, used_mem) = {
            let used_mem = sys.used_memory();
            let expr = format!("({} / ({} / 100))", used_mem, total_mem);
            let t = mexprp::eval::<f64>(&expr).unwrap().to_vec();
            (t[0], used_mem)
        };
        let total_swap = sys.total_swap();
        let (swap, used_swap) = {
            let used_swap = sys.used_swap();
            let expr = format!("({} / ({} / 100))", used_swap, total_swap);
            let t = mexprp::eval::<f64>(&expr).unwrap().to_vec();
            (t[0], used_swap)
        };
        (mem, total_mem, used_mem, swap, total_swap, used_swap)
    };
    return ((mem, swap), (total_mem, total_swap), used_mem, used_swap);
}

/// Computes the cpu core amount. Should the request for cores fail or be bigger than 255, 1 is returned.
pub fn get_system_data() ->  (u8, Option<usize>, Option<String>, Option<String>, Option<String>, Option<String>) {
    let sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything().with_frequency()));
    let cpu_phy_core = sys.physical_core_count();
    let sys_name = System::name();
    let kernel_ver = System::kernel_version();
    let os_ver = System::long_os_version();
    let host_name = System::host_name();
    let cpu_core_total = TryInto::<u8>::try_into(sys.cpus().len());

    if cpu_core_total.is_ok() {
        return (cpu_core_total.unwrap(), cpu_phy_core, sys_name, kernel_ver, os_ver, host_name);
    } else {
        return (1, cpu_phy_core, sys_name, kernel_ver, os_ver, host_name);
    }
}

