use chrono_tz::Tz;
use eframe::epaint::{Color32, Vec2};


// Ref UIb2
#[derive(Debug)]
pub struct Settings {
    pub main_colour: Color32,
    pub cpu_colour: Color32,
    pub ram_colour: Color32,
    pub network_colour: Color32,
    pub network_error_colour: Color32,
    pub disk_write_colour: Color32,
    pub disk_read_colour: Color32,
    pub temperature_colour: Color32,
    pub timezone: Tz,
    pub dark_theme: bool,
    pub data_update_interval: i64,
    pub display_size: Vec2,
    pub display_time_ribbon: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { 
            main_colour: Color32::GOLD, 
            cpu_colour: Color32::GOLD,
            ram_colour: Color32::GOLD,
            network_colour: Color32::GOLD,
            network_error_colour: Color32::RED,
            disk_write_colour: Color32::GOLD,
            disk_read_colour: Color32::GREEN,
            temperature_colour: Color32::GOLD,
            timezone: chrono_tz::GMT,
            dark_theme: true, 
            data_update_interval: 1000,
            display_size: Vec2 { x: 1200.0, y: 1000.0 }, 
            display_time_ribbon: true 
        }
    }
}

impl Settings {
    fn new(main_colour: Color32, cpu_colour: Color32, ram_colour: Color32, network_colour: Color32, network_error_colour: Color32, disk_write_colour: Color32, disk_read_colour: Color32, temperature_colour: Color32, timezone: Tz, dark_theme: bool, data_update_interval: i64, display_size: Vec2, display_time_ribbon: bool) -> Self {
        Settings { main_colour, cpu_colour, ram_colour, network_colour, network_error_colour, disk_write_colour, temperature_colour, disk_read_colour, timezone, dark_theme, data_update_interval, display_size, display_time_ribbon }
    }
}
