use std::collections::VecDeque;

pub struct CpuData {
    pub core_data: Vec<VecDeque<f64>>,
    pub avg_load: VecDeque<f64>,
}

impl CpuData {
    pub fn new(data: Vec<f64>) -> Self {
        let core_data = {
            let mut out: Vec<VecDeque<f64>> = Default::default();
            for _n in 1..=12 {
                let queue = VecDeque::from(data.clone());
                out.push(queue);
            }
            out
        };
        let avg_load = VecDeque::from(data);
        CpuData { core_data, avg_load }
    }
}
