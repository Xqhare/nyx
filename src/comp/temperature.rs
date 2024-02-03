use std::{rc::Rc, sync::Mutex, collections::VecDeque};


use crate::utils;

#[derive(Clone)]
pub struct Temperature {
    pub name: Rc<String>,
    pub sensor: Rc<String>,
    pub temperature: Rc<Mutex<VecDeque<f32>>>,
    pub critical_temperature: Rc<f32>,
}

#[derive(Clone)]
pub struct Temperatures {
    pub components: Rc<Mutex<Vec<Temperature>>>,
}

impl Temperature {
    fn new(name: String, sensor: String, temperature: f32, critical_temperature: f32) -> Self {
        Temperature { name: Rc::new(name), sensor: Rc::new(sensor), temperature: Rc::new(Mutex::new(VecDeque::from(vec![temperature]))), critical_temperature: Rc::new(critical_temperature) }
    }
}

impl Temperatures {
    pub fn new() -> Self {
        let new_data = utils::get_temperature_data();
        let mut out: Vec<Temperature> = Default::default();
        for data in new_data {
            out.push(Temperature::new(data.0, data.1, data.2, data.3));
        }
        Temperatures { components: Rc::from(Mutex::from(out)) }
    }
    pub fn update(&mut self) {
        let new_data = utils::get_temperature_update_data();
        let data_store = self.components.lock();
        if data_store.is_ok() {
            let ok_data_store = data_store.unwrap();
            // Now I belive that after locking the store, all other locks I have done to be
            // superflous... lets see how this goes!
            // -> It doesn't! Back to locking!
            for comp in ok_data_store.iter() {
                for data in &new_data {
                    if comp.sensor.contains(&data.0) {
                        let temperature_data = comp.temperature.lock();
                        if temperature_data.is_ok() {
                            let mut ok_temperature_data = temperature_data.unwrap();
                            if ok_temperature_data.len() == 60 {
                                let _ = ok_temperature_data.pop_back();
                                ok_temperature_data.push_front(data.1);
                            } else if ok_temperature_data.len() < 60 {
                                ok_temperature_data.push_front(data.1);
                            } else {
                                ok_temperature_data.truncate(59);
                                ok_temperature_data.push_front(data.1);
                            }
                        }
                    }
                }
            }
        }
    }
}
