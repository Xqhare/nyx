use chrono::{Utc, Duration, SecondsFormat};
use rand::{prelude::SliceRandom, thread_rng};

pub fn time_now_rfc3339zulu(seconds_format: SecondsFormat) -> String {
    Utc::now().to_rfc3339_opts(seconds_format, true)
}

#[allow(dead_code)]
fn ran_small_num() -> u8 { 
    let lib = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let mut rng = thread_rng(); 
    let ran_num = lib.choose(&mut rng).unwrap(); 
    return *ran_num; 
}

pub fn next_update_time(interval: Duration) -> String {
    let now = Utc::now().checked_add_signed(interval);
    if now.is_some() {
        return now.unwrap().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    } else {
        return Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    }
}
