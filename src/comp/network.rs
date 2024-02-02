use std::{rc::Rc, sync::Mutex, collections::VecDeque};

use crate::utils;

// Ref UIc3: 
#[derive(Clone)]
pub struct Network {
    pub name: Rc<String>,
    pub mac_addr: Rc<String>,
    pub incoming: Rc<Mutex<VecDeque<u64>>>,
    pub total_incoming: Rc<Mutex<u64>>,
    pub errors_incoming: Rc<Mutex<VecDeque<u64>>>,
    pub total_errors_incoming: Rc<Mutex<u64>>,
    pub packets_incoming: Rc<Mutex<VecDeque<u64>>>,
    pub total_packets_incoming: Rc<Mutex<u64>>,
    pub outgoing: Rc<Mutex<VecDeque<u64>>>,
    pub total_outgoing: Rc<Mutex<u64>>,
    pub errors_outgoing: Rc<Mutex<VecDeque<u64>>>,
    pub total_errors_outgoing: Rc<Mutex<u64>>,
    pub packets_outgoing: Rc<Mutex<VecDeque<u64>>>,
    pub total_packets_outgoing: Rc<Mutex<u64>>,
}


#[derive(Clone)]
pub struct Networks {
    pub networks: Rc<Mutex<Vec<Network>>>,
}

impl Network {
    pub fn new(name: String, mac_addr: String, incoming: u64, total_incoming: u64, errors_incoming: u64, total_errors_incoming: u64, packets_incoming: u64, total_packets_incoming: u64, outgoing: u64, total_outgoing: u64, errors_outgoing: u64, total_errors_outgoing: u64, packets_outgoing: u64, total_packets_outgoing: u64) -> Network {
        
        Network { name: Rc::new(name), mac_addr: Rc::new(mac_addr), incoming: Rc::new(Mutex::new(VecDeque::from(vec![incoming]))), total_incoming: Rc::new(Mutex::new(total_incoming)), errors_incoming: Rc::new(Mutex::new(VecDeque::from(vec![errors_incoming]))), total_errors_incoming: Rc::new(Mutex::new(total_errors_incoming)), packets_incoming: Rc::new(Mutex::new(VecDeque::from(vec![packets_incoming]))), total_packets_incoming: Rc::new(Mutex::new(total_packets_incoming)), outgoing: Rc::new(Mutex::new(VecDeque::from(vec![outgoing]))), total_outgoing: Rc::new(Mutex::new(total_outgoing)), errors_outgoing: Rc::new(Mutex::new(VecDeque::from(vec![errors_outgoing]))), total_errors_outgoing: Rc::new(Mutex::new(total_errors_outgoing)), packets_outgoing: Rc::new(Mutex::new(VecDeque::from(vec![packets_outgoing]))), total_packets_outgoing: Rc::new(Mutex::new(total_packets_outgoing)) }
    }
}

impl Networks {
    pub fn new() -> Self {
        let data = utils::get_network_data();
        let mut networks: Vec<Network> = Default::default();
        for network_data in data {
            let network = Network::new(network_data.0, network_data.1, network_data.2, network_data.3, network_data.4, network_data.5, network_data.6, network_data.7, network_data.8, network_data.9, network_data.10, network_data.11, network_data.12, network_data.13);
            networks.push(network);
        }
        Networks { networks: Rc::new(Mutex::new(networks)) }
    }

    pub fn update(&mut self) {
        let new_data = utils::get_network_data();
        let data_store = self.networks.lock();
        if data_store.is_ok() {
            let ok_data_store = data_store.unwrap();
            for network in ok_data_store.iter() {
                for data in &new_data {
                    if network.name.contains(&data.0) {
                        let mut incoming = network.incoming.lock();
                        let total_incoming = network.total_incoming.lock();
                        let mut errors_incoming = network.errors_incoming.lock();
                        let total_errors_incoming = network.total_errors_incoming.lock();
                        let mut packets_incoming = network.packets_incoming.lock();
                        let total_packets_incoming = network.total_packets_incoming.lock();
                        let mut outgoing = network.outgoing.lock();
                        let total_outgoing = network.total_outgoing.lock();
                        let mut errors_outgoing = network.errors_outgoing.lock();
                        let total_errors_outgoing = network.total_errors_outgoing.lock();
                        let mut packets_outgoing = network.packets_outgoing.lock();
                        let total_packets_outgoing = network.total_packets_outgoing.lock();
                        if incoming.is_ok() && total_incoming.is_ok() && errors_incoming.is_ok() && total_errors_incoming.is_ok() && packets_incoming.is_ok() && total_packets_incoming.is_ok() && outgoing.is_ok() && total_outgoing.is_ok() && errors_outgoing.is_ok() && total_errors_outgoing.is_ok() && packets_outgoing.is_ok() && total_packets_outgoing.is_ok() {
                            let inc_calc = data.3 - **total_incoming.as_ref().unwrap();
                            let err_inc_calc = data.5 - **total_errors_incoming.as_ref().unwrap();
                            let pack_inc_calc = data.7 - **total_packets_incoming.as_ref().unwrap();
                            
                            let out_calc = data.9 - **total_outgoing.as_ref().unwrap();
                            let err_out_calc = data.11 - **total_errors_outgoing.as_ref().unwrap();
                            let pack_out_calc = data.13 - **total_packets_outgoing.as_ref().unwrap();
                            
                            if incoming.as_ref().unwrap().len() == 60 {
                                let _ = incoming.as_mut().unwrap().pop_back();
                                let _ = errors_incoming.as_mut().unwrap().pop_back();
                                let _ = packets_incoming.as_mut().unwrap().pop_back();
                                
                                let _ = outgoing.as_mut().unwrap().pop_back();
                                let _ = errors_outgoing.as_mut().unwrap().pop_back();
                                let _ = packets_outgoing.as_mut().unwrap().pop_back();

                                incoming.as_mut().unwrap().push_front(inc_calc);
                                errors_incoming.as_mut().unwrap().push_front(err_inc_calc);
                                packets_incoming.as_mut().unwrap().push_front(pack_inc_calc);

                                outgoing.as_mut().unwrap().push_front(out_calc);
                                errors_outgoing.as_mut().unwrap().push_front(err_out_calc);
                                packets_outgoing.as_mut().unwrap().push_front(pack_out_calc);

                                *total_incoming.unwrap() = data.3;
                                *total_errors_incoming.unwrap() = data.5;
                                *total_packets_incoming.unwrap() = data.7;
                                
                                *total_outgoing.unwrap() = data.9;
                                *total_errors_outgoing.unwrap() = data.11;
                                *total_packets_outgoing.unwrap() = data.13;

                            } else if incoming.as_ref().unwrap().len() < 60 {
                                incoming.as_mut().unwrap().push_front(inc_calc);
                                errors_incoming.as_mut().unwrap().push_front(err_inc_calc);
                                packets_incoming.as_mut().unwrap().push_front(pack_inc_calc);

                                outgoing.as_mut().unwrap().push_front(out_calc);
                                errors_outgoing.as_mut().unwrap().push_front(err_out_calc);
                                packets_outgoing.as_mut().unwrap().push_front(pack_out_calc);

                                *total_incoming.unwrap() = data.3;
                                *total_errors_incoming.unwrap() = data.5;
                                *total_packets_incoming.unwrap() = data.7;
                                
                                *total_outgoing.unwrap() = data.9;
                                *total_errors_outgoing.unwrap() = data.11;
                                *total_packets_outgoing.unwrap() = data.13;
                            } else {

                                incoming.as_mut().unwrap().truncate(59);
                                errors_incoming.as_mut().unwrap().truncate(59);
                                packets_incoming.as_mut().unwrap().truncate(59);
                                
                                outgoing.as_mut().unwrap().truncate(59);
                                errors_outgoing.as_mut().unwrap().truncate(59);
                                packets_outgoing.as_mut().unwrap().truncate(59);

                                incoming.as_mut().unwrap().push_front(inc_calc);
                                errors_incoming.as_mut().unwrap().push_front(err_inc_calc);
                                packets_incoming.as_mut().unwrap().push_front(pack_inc_calc);

                                outgoing.as_mut().unwrap().push_front(out_calc);
                                errors_outgoing.as_mut().unwrap().push_front(err_out_calc);
                                packets_outgoing.as_mut().unwrap().push_front(pack_out_calc);

                                *total_incoming.unwrap() = data.3;
                                *total_errors_incoming.unwrap() = data.5;
                                *total_packets_incoming.unwrap() = data.7;
                                
                                *total_outgoing.unwrap() = data.9;
                                *total_errors_outgoing.unwrap() = data.11;
                                *total_packets_outgoing.unwrap() = data.13;
                            }
                        }
                    }
                }
            }
        }
        
    }
}
