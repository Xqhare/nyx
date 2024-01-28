use std::{collections::VecDeque, sync::{Arc, Mutex}};


use crate::utils;

#[derive(Clone)]
pub struct CpuData {
    pub core_data: Arc<Mutex<Vec<Arc<Mutex<VecDeque<f64>>>>>>,
    pub avg_load: Arc<Mutex<VecDeque<f64>>>,
}

impl CpuData {
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
        let avg_load: Arc<Mutex<VecDeque<f64>>> = {
            let queue: Arc<Mutex<VecDeque<f64>>> = Arc::new(Mutex::new(VecDeque::from(vec![tmp.1])));
            queue
        };
        CpuData { core_data, avg_load }
    }

    pub fn update(&mut self) {
        let new_data = utils::get_cpu_data();
        let data_store = self.avg_load.lock();
        if data_store.is_ok() {
            let mut ok_store = data_store.unwrap();
            // If the collection is at 60 drop the last one and insert on in the front.
            if ok_store.len() == 60 {
                // avg_load
                let _ = ok_store.pop_back();
                ok_store.push_front(new_data.1);
                // core load
                for core in self.core_data.lock() {
                    for entry in new_data.0.iter().enumerate() {
                        let index = entry.0;
                        let data = entry.1;
                        let _ = core[index].lock().unwrap().pop_back();
                        core[index].lock().unwrap().push_front(*data);
                    }
                }
            // Until the collection has reached 60 elements, insert one in the front.
            } else if ok_store.len() < 60 {
                ok_store.push_front(new_data.1);
                for core in self.core_data.lock() {
                    for entry in new_data.0.iter().enumerate() {
                        let index = entry.0;
                        let data = entry.1;
                        core[index].lock().unwrap().push_front(*data);
                    }
                }

            // Failsave if collection ever grows bejond 60 elements.
            } else {
                ok_store.truncate(59);
                ok_store.push_front(new_data.1);
                for core in self.core_data.lock() {
                    for entry in new_data.0.iter().enumerate() {
                        let index = entry.0;
                        let data = entry.1;
                        core[index].lock().unwrap().truncate(59);
                        core[index].lock().unwrap().push_front(*data);
                    }
                }
            }
        } else {
            // Couldn't get a lock on data! lets abort the operation.
            return;
        }
    }

    
}
