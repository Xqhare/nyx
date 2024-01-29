use std::{collections::VecDeque, sync::{Arc, Mutex}};

use crate::utils;

#[derive(Clone)]
/// Holds core usage data and average system load data for the last 60 update increments
pub struct CpuData {
    /// Holds core usage data for the last 60 update increments
    pub core_data: Arc<Mutex<Vec<Arc<Mutex<VecDeque<f64>>>>>>,
    /// Holds average system load data for the last 60 update increments
    pub avg_load: Arc<Mutex<VecDeque<f64>>>,
}

impl CpuData {

    /// Creates a new instance of `CpuData` filled with one datapoint for each cpu and average load
    pub fn new() -> Self {
        let tmp = utils::get_cpu_data();
        let core_data: Arc<Mutex<Vec<Arc<Mutex<VecDeque<f64>>>>>>  = {
            let mut queue: Vec<Arc<Mutex<VecDeque<f64>>>> = Default::default();
            for data in tmp.0 {
                let mut core: VecDeque<f64> = Default::default();
                core.push_front(data);
                queue.push(Arc::new(Mutex::new(core)));
            };
            Arc::new(Mutex::new(queue))
        };
        let avg_load: Arc<Mutex<VecDeque<f64>>> = Arc::new(Mutex::new(VecDeque::from(vec![tmp.1])));
        CpuData { core_data, avg_load }
    }

    /// Updates the exsisting instance of `CpuData` with one datapoint for each cpu and average load
    pub fn update(&mut self) {
        let new_data = utils::get_cpu_data();
        let data_store = self.avg_load.lock();
        if data_store.is_ok() {
            let mut ok_store = data_store.unwrap();
            // If lock on core data is bad don't do anything, Ref F3
            // If the collection is at 60 drop the last one and insert on in the front.
            if ok_store.len() == 60 {
                // avg_load
                let _ = ok_store.pop_back();
                ok_store.push_front(new_data.1);
                // core load
                let core = self.core_data.lock();
                if core.is_ok() {
                    let un_core = core.unwrap();
                    for entry in new_data.0.iter().enumerate() {
                        let index = entry.0;
                        let data = entry.1;
                        let _ = un_core[index].lock().unwrap().pop_back();
                        un_core[index].lock().unwrap().push_front(*data);
                    }
                }
            // Until the collection has reached 60 elements, insert one in the front.
            } else if ok_store.len() < 60 {
                ok_store.push_front(new_data.1);
                let core = self.core_data.lock();
                if core.is_ok() {
                    let un_core = core.unwrap();
                    for entry in new_data.0.iter().enumerate() {
                        let index = entry.0;
                        let data = entry.1;
                        un_core[index].lock().unwrap().push_front(*data);
                    }
                }
            // Failsave if collection ever grows bejond 60 elements.
            } else {
                ok_store.truncate(59);
                ok_store.push_front(new_data.1);
                let core = self.core_data.lock();
                if core.is_ok() {
                    let un_core = core.unwrap();
                    for entry in new_data.0.iter().enumerate() {
                        let index = entry.0;
                        let data = entry.1;
                        un_core[index].lock().unwrap().truncate(59);
                        un_core[index].lock().unwrap().push_front(*data);
                    }
                }
            }
        } else {
            // Couldn't get a lock on data! lets abort the operation. Ref F3
            return;
        }
    }
    
}
