use std::{collections::VecDeque, sync::{Arc, Mutex}};

use crate::utils;

#[derive(Clone)]
/// Holds core usage data and average system load data for the last 60 update increments
pub struct RamData {
    /// Holds average system load data for the last 60 update increments
    pub memory: Arc<Mutex<VecDeque<f64>>>,
    pub swap: Arc<Mutex<VecDeque<f64>>>,
    pub total_mem: Arc<Mutex<u64>>,
    pub total_swap: Arc<Mutex<u64>>,
}

impl RamData {

    pub fn new() -> Self {
        let tmp = utils::get_ram_data();
        let memory = Arc::new(Mutex::new(VecDeque::from(vec![tmp.0.0])));
        let swap = Arc::new(Mutex::new(VecDeque::from(vec![tmp.0.1])));
        let total_mem = Arc::new(Mutex::new(tmp.1.0));
        let total_swap = Arc::new(Mutex::new(tmp.1.1));
        RamData { memory, swap , total_mem, total_swap}
    }

    pub fn update(&mut self) {
        // Not all of the returned data has to be used, only the first touple!
        // Ram totals don't usally change during operation!
        let new_data = utils::get_ram_data();
        let mem_store = self.memory.lock();
        // The if statements are nested for atomicity in statechanges. So only if both
        // locks are successfull any data manipulation is done. Ref F3
        if mem_store.is_ok() {
            let mut ok_mem_store = mem_store.unwrap();
            let swap_store = self.swap.lock();
            if swap_store.is_ok() {
                let mut ok_swap_store = swap_store.unwrap();
                // Starting with memory
                if ok_mem_store.len() == 60 {
                    let _ = ok_mem_store.pop_back();
                    ok_mem_store.push_front(new_data.0.0);
                } else if ok_mem_store.len() < 60 {
                    ok_mem_store.push_front(new_data.0.0);
                } else if  ok_mem_store.len() > 60 {
                    ok_mem_store.truncate(59);
                    ok_mem_store.push_front(new_data.0.0);
                }
                // Then swap
                if ok_swap_store.len() == 60 {
                    let _ = ok_swap_store.pop_back();
                    ok_swap_store.push_front(new_data.0.1);
                } else if ok_swap_store.len() < 60 {
                    ok_swap_store.push_front(new_data.0.1);
                } else if  ok_swap_store.len() > 60 {
                    ok_swap_store.truncate(59);
                    ok_swap_store.push_front(new_data.0.1);
                }
            } else {
                // Ref F3
                return;
            }
        } else {
            // Ref F3
            return;
        }
    }

}

