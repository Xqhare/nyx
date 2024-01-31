use std::{collections::VecDeque, sync::{Arc, Mutex}};

use crate::utils;

#[derive(Clone)]
/// Holds core usage data and average system load data for the last 60 update increments
pub struct RamData {
    /// Holds average system load data for the last 60 update increments
    pub memory: Arc<Mutex<VecDeque<f64>>>,
    pub swap: Arc<Mutex<VecDeque<f64>>>,
    pub total_mem: Arc<Mutex<u64>>,
    pub mem_used: Arc<Mutex<u64>>,
    pub total_swap: Arc<Mutex<u64>>,
    pub swap_used: Arc<Mutex<u64>>,
}

impl RamData {

    pub fn new() -> Self {
        let tmp = utils::get_ram_data();
        let memory = Arc::new(Mutex::new(VecDeque::from(vec![tmp.0.0])));
        let swap = Arc::new(Mutex::new(VecDeque::from(vec![tmp.0.1])));
        let total_mem = Arc::new(Mutex::new(tmp.1.0));
        let total_swap = Arc::new(Mutex::new(tmp.1.1));
        let mem_used = Arc::new(Mutex::new(tmp.2));
        let swap_used = Arc::new(Mutex::new(tmp.3));
        RamData { memory, swap, mem_used, total_mem, total_swap, swap_used}
    }

    pub fn update(&mut self) {
        // Not all of the returned data has to be used, only the first touple!
        // Ram totals don't usally change during operation!
        let new_data = utils::get_ram_data();
        let mem_store = self.memory.lock();
        let mem_used_store = self.mem_used.lock();
        // The if statements are nested for atomicity in statechanges. So only if both
        // locks are successfull any data manipulation is done. Ref F3
        if mem_store.is_ok() && mem_used_store.is_ok() {
            let mut ok_mem_store = mem_store.unwrap();
            let mut ok_mem_used_store = mem_used_store.unwrap();
            let swap_store = self.swap.lock();
            let swap_used_store = self.swap_used.lock();
            if swap_store.is_ok() && swap_used_store.is_ok() {
                let mut ok_swap_store = swap_store.unwrap();
                let mut ok_swap_used_store = swap_used_store.unwrap();
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
                *ok_mem_used_store = new_data.2;
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
                *ok_swap_used_store = new_data.3;
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

