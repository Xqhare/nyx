use std::{sync::Mutex, rc::Rc, collections::VecDeque};


use crate::utils::{self, utils::get_process_data};

#[derive(Clone)]
/// Holds amount of processes for the last 60 update increments
pub struct ProcessData {
    /// Holds amount of processes for the last 60 update increments
    pub amount_processes: Rc<Mutex<VecDeque<usize>>>,
}

impl ProcessData {
    pub fn new() -> Self {
        let data_in = utils::utils::get_process_amount();
        let out = VecDeque::from(vec![data_in]);
        ProcessData { amount_processes: Rc::new(Mutex::new(out))}
    }

    pub fn update(&mut self) {
        let new_data = utils::utils::get_process_amount();
        let data_store = self.amount_processes.lock();
        if data_store.is_ok() {
            let mut ok_store = data_store.unwrap();
            if ok_store.len() == 60 {
                let _ = ok_store.pop_back();
                ok_store.push_front(new_data);
            } else if ok_store.len() < 60 {
                ok_store.push_front(new_data);
            } else {
                ok_store.truncate(59);
                ok_store.push_front(new_data);
            }
        }
    }
}

/// Represents a single process, holding the pertinant data for it, e.g. pid, parent pid, etc
pub struct NyxProcess {
    pub pid: u32,
    pub name: String,
    pub mem: u64,
    pub parent_pid: Option<u32>,
    pub status: String,
    pub runtime: u64,
}

impl NyxProcess {
    pub fn new(pid: u32, name: String, mem: u64, parent_pid: Option<u32>, status: String, runtime: u64, ) -> Self {
        NyxProcess { pid, name, mem, parent_pid, status, runtime}
    }
}

/// Holds all current processes
pub struct Processes {
    pub processes: Rc<Mutex<Vec<NyxProcess>>>,
}

impl Processes {
    pub fn new() -> Self {
        let data = get_process_data();
        let mut out: Vec<NyxProcess> = Default::default();
        for entry in data {
            let proc = NyxProcess::new(entry.0, entry.1, entry.2, entry.3, entry.4, entry.5);
            out.push(proc);
        }
        Processes { processes: Rc::new(Mutex::new(out)) }
    }
}
