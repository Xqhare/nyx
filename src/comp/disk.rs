use std::{sync::Mutex, rc::Rc, collections::VecDeque};

use crate::utils::{get_disk_data, get_disk_update_data};

// Ref UIc4: Disks need name, filesystem, type, removable, mounted on, used bytes, free bytes
#[derive(Clone, Debug)]
pub struct Disk {
    pub name: Rc<String>,
    pub filesystem: Rc<String>,
    pub disk_type: Rc<String>,
    pub removable: Rc<bool>,
    pub mountpoint: Rc<String>,
    pub used_bytes: Rc<Mutex<u64>>,
    pub free_bytes: Rc<Mutex<u64>>,
    pub total_bytes: Rc<u64>,
    pub stat_reads: Rc<Mutex<VecDeque<u64>>>,
    pub stat_writes: Rc<Mutex<VecDeque<u64>>>,
    pub last_abs_reads: Rc<Mutex<u64>>,
    pub last_abs_writes: Rc<Mutex<u64>>,
}

#[derive(Debug, Clone)]
pub struct Disks {
    pub disks: Rc<Mutex<Vec<Disk>>>,
}

impl Disk {
    pub fn new(name: String, filesystem: String, disk_type: String, removeable: bool, mountpoint: String, used_bytes: u64, free_bytes: u64, total_bytes: u64, stat_reads: u64, stat_writes: u64, last_abs_reads: u64, last_abs_writes: u64) -> Disk {
        
        Disk { name: Rc::new(name), filesystem: Rc::new(filesystem), disk_type: Rc::new(disk_type), removable: Rc::new(removeable), mountpoint: Rc::new(mountpoint), used_bytes: Rc::new(Mutex::new(used_bytes)), free_bytes: Rc::new(Mutex::new(free_bytes)), total_bytes: Rc::new(total_bytes), stat_reads: Rc::new(Mutex::new(VecDeque::from(vec![stat_reads]))), stat_writes: Rc::new(Mutex::new(VecDeque::from(vec![stat_writes]))), last_abs_reads: Rc::new(Mutex::new(last_abs_reads)), last_abs_writes: Rc::new(Mutex::new(last_abs_writes)) }
    }
}

impl Disks {
    pub fn new() -> Self {
        let data = get_disk_data();
        let mut out: Vec<Disk> = Default::default();
        for disk in data {
            let name = disk.0;
            let filesystem = disk.1;
            let disk_type = disk.2;
            let removeable = disk.3;
            let mountpoint = disk.4;
            let used_bytes = disk.5;
            let free_bytes = disk.6;
            let total_bytes = disk.7;
            let last_abs_reads = disk.8;
            let last_abs_writes = disk.9;
            let stat_reads: u64 = 0;
            let stat_writes: u64 = 0;
            let new_disk = Disk::new(name, filesystem, disk_type, removeable, mountpoint, used_bytes, free_bytes, total_bytes, stat_reads, stat_writes, last_abs_reads, last_abs_writes);
            out.push(new_disk);
        }
        Disks { disks: Rc::new(Mutex::new(out)) }
    }

    pub fn update(&mut self) {
        let new_data = get_disk_update_data();
        let data_store = self.disks.lock();
        // There has to be a better way
        if data_store.is_ok() {
            let ok_data_store = data_store.unwrap();
            for disk_data in new_data {
                for disk in ok_data_store.iter() {
                    if disk.name.contains(&disk_data.0) {
                        let last_abs_reads = disk.last_abs_reads.lock();
                        let last_abs_writes = disk.last_abs_writes.lock();
                        let mut stat_store_reads = disk.stat_reads.lock();
                        let mut stat_store_writes = disk.stat_writes.lock();
                        let now_abs_reads = disk_data.3;
                        let now_abs_writes = disk_data.4;
                        if last_abs_reads.is_ok() && last_abs_writes.is_ok() && stat_store_reads.is_ok() && stat_store_writes.is_ok() {
                            let stat_reads = now_abs_reads - **last_abs_reads.as_ref().unwrap();
                            let stat_writes = now_abs_writes - **last_abs_writes.as_ref().unwrap();
                            *last_abs_reads.unwrap() = now_abs_reads;
                            *last_abs_writes.unwrap() = now_abs_writes;
                            if stat_store_reads.as_ref().unwrap().len() == 60 {
                                let _ = stat_store_reads.as_mut().unwrap().pop_back();
                                let _ = stat_store_writes.as_mut().unwrap().pop_back();
                                stat_store_reads.unwrap().push_front(stat_reads);
                                stat_store_writes.unwrap().push_front(stat_writes);
                            } else if stat_store_reads.as_ref().unwrap().len() < 60 {
                                stat_store_reads.unwrap().push_front(stat_reads);
                                stat_store_writes.unwrap().push_front(stat_writes);
                            } else {
                                stat_store_reads.as_mut().unwrap().truncate(59);
                                stat_store_writes.as_mut().unwrap().truncate(59);
                                stat_store_reads.as_mut().unwrap().push_front(stat_reads);
                                stat_store_writes.as_mut().unwrap().push_front(stat_writes);
                            }
                        }
                        
                    }
                }
            }
            
        }
    }
}
