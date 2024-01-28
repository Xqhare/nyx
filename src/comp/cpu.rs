use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};

use sysinfo::{System, RefreshKind, CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL};

#[derive(Clone)]
pub struct CpuData {
    pub core_data: Arc<Vec<Arc<Mutex<VecDeque<f64>>>>>,
    pub avg_load: Arc<Mutex<VecDeque<f64>>>,
}

impl CpuData {
    // core_data and avg_load is the same data:
    // The data input here is a dummy! In prod this would create the first data set of the cpu.
    pub fn new(data: Vec<f64>, num_cores: u8) -> Self {
        let core_data = {
            let mut out: Vec<Arc<Mutex<VecDeque<f64>>>> = Default::default();
            for _n in 1..=num_cores {
                let queue = Mutex::from(VecDeque::from(data.clone()));
                out.push(queue.into());
            }
            out
        };
        let avg_load = Mutex::from(VecDeque::from(data.clone()));
        CpuData { core_data: core_data.into(), avg_load: avg_load.into() }
    }

    pub fn update(&mut self) {
        let new_data = self.get_cpu_data();
        let data_store = self.avg_load.lock();
        if data_store.is_ok() {
            let mut ok_store = data_store.unwrap();
            // If the collection is at 60 drop the last one and insert on in the front.
            if ok_store.len() == 60 {
                let _ = ok_store.pop_back();
                ok_store.push_front(new_data.1);
            // Until the collection has reached 60 elements, insert one in the front.
            } else if ok_store.len() < 60 {
                ok_store.push_front(new_data.1);

            // Failsave if collection ever grows bejond 60 elements.
            } else {
                ok_store.truncate(59);
                ok_store.push_front(new_data.1);
            }
        } else {
        }
    }

    fn get_cpu_data(&self) -> (Vec<f64>, f64) {
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
}
