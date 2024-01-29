use std::{collections::VecDeque, sync::{Arc, Mutex}};

use crate::utils;

#[derive(Clone)]
/// Holds core usage data and average system load data for the last 60 update increments
pub struct RamData {
    /// Holds average system load data for the last 60 update increments
    pub memory: Arc<Mutex<VecDeque<f64>>>,
    pub swap: Arc<Mutex<VecDeque<f64>>>,
}

impl RamData {
    pub fn new() -> Self {
        let memory: Arc<Mutex<VecDeque<f64>>> = Default::default();
        let swap: Arc<Mutex<VecDeque<f64>>> = Default::default();
        RamData { memory, swap }
    }
}
