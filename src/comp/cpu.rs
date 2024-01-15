use chrono::{self, Utc, SecondsFormat};

struct CPU {
    // Reasoning for u8 in design doc. Ref C1.
    core_number: u8,
    core_load: f64,
    time: String,
}

impl CPU {
    fn new(core_number: u8, core_load: f64) -> CPU {
        // Reasoning for time format in design doc. Ref D1.
        let time = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        CPU { core_number, core_load, time }
    }
}
