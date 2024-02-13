
use std::sync::Arc;

use chrono::{Duration, SecondsFormat};
use chrono_tz::Tz;
use eframe::{epaint::Vec2, egui::{CentralPanel, Ui, IconData, Context}, run_native, NativeOptions, App, Frame};


use crate::{utils::{self, settings::Settings}, comp::{ram::RamData, disk::Disks, network::Networks, cpu::CpuData, temperature::Temperatures}};

const DATAUPDATEINTERVAL: i64 = 1000;

struct Nyx {
    // AppData
    next_data_update: String,
    // Data
    num_cores: u8,
    networks: Networks,
    disks: Disks,
    cpu_data: CpuData,
    ram_data: RamData,
    temperatures: Temperatures,

    // Drawing booleans
    show_landing_page: bool,
    show_help: bool,
    show_cpu_page: bool,
    show_ram_page: bool,
    show_gpu_page: bool,
    show_disk_page: bool,
    show_network_page: bool,
    show_temperature_page: bool,
    show_settings_page: bool,
    show_about_page: bool,
    show_eris_page: bool,
    
    // Settings
    display_size: Vec2,
    timezone: Tz,
    settings: Settings,
}

impl Default for Nyx {

    fn default() -> Self {
        let num_cores: u8 = utils::utils::get_cpu_core_amount();
        let networks = Networks::new();
        let disks = Disks::new();
        // TODO Put display_size into settings
        let display_size: Vec2 = Vec2 { x: 1200.0, y: 900.0 };
        let next_data_update = utils::utils::next_update_time(Duration::milliseconds(DATAUPDATEINTERVAL));
        let cpu_data = CpuData::new();
        let ram_data = RamData::new();
        let temperatures = Temperatures::new();
        let timezone = chrono_tz::Europe::Berlin;
        let settings = Settings::default();
        Nyx { 
            num_cores,  display_size, networks, disks, next_data_update, cpu_data, ram_data, timezone, temperatures, settings,
            // default true
            show_landing_page: true,
            // default false
            show_cpu_page: false, show_ram_page: false, show_help: false, show_gpu_page: false, show_disk_page: false, show_temperature_page: false, show_network_page: false,
            show_settings_page: false, show_about_page: false, show_eris_page: false,
        }
    }

}

impl Nyx {
    fn new(settings: Settings) -> Self {
        let num_cores: u8 = utils::utils::get_cpu_core_amount();
        let networks = Networks::new();
        let disks = Disks::new();
        // TODO Put display_size into settings
        let display_size: Vec2 = Vec2 { x: 1200.0, y: 900.0 };
        let next_data_update = utils::utils::next_update_time(Duration::milliseconds(DATAUPDATEINTERVAL));
        let cpu_data = CpuData::new();
        let ram_data = RamData::new();
        let temperatures = Temperatures::new();
        let timezone = chrono_tz::Europe::Berlin;
        let settings = settings;
        Nyx { 
            num_cores,  display_size, networks, disks, next_data_update, cpu_data, ram_data, timezone, temperatures, settings,
            // default true
            show_landing_page: true,
            // default false
            show_cpu_page: false, show_ram_page: false, show_help: false, show_gpu_page: false, show_disk_page: false, show_temperature_page: false, show_network_page: false,
            show_settings_page: false, show_about_page: false, show_eris_page: false,
        }
    }
}

impl App for Nyx {

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // This makes sure that Nyx is run continiously with a maximum wait time in millisecounds.
        ctx.request_repaint_after(std::time::Duration::from_millis(DATAUPDATEINTERVAL as u64 / 100));
        CentralPanel::default()
            .show(ctx, |ui: &mut Ui| {
                // Time has come for Data update
                if utils::utils::time_now_rfc3339zulu(SecondsFormat::Secs) >= self.next_data_update {
                    self.next_data_update = utils::utils::next_update_time(Duration::milliseconds(DATAUPDATEINTERVAL));
                    self.cpu_data.update();
                    self.ram_data.update();
                    self.disks.update();
                    self.networks.update();
                    self.temperatures.update();
                }
                self.draw_main_menu(ui);
                ui.separator();
                if self.show_help {
                    ui.label("help");
                }
                if self.show_landing_page {
                    self.draw_landing_page(ui);
                }
                if self.show_cpu_page {
                    ui.label("cpu");
                }
                if self.show_ram_page {
                    ui.label("ram");
                }
                
                if self.show_gpu_page {
                    ui.label("gpu");
                }
                if self.show_disk_page {
                    ui.label("disk");
                }
                if self.show_network_page {
                    ui.label("network");
                }
                if self.show_temperature_page {
                    ui.label("temperature");
                }
                if self.show_settings_page {
                    self.draw_settings_page(ui);
                }
                if self.show_about_page {
                    self.draw_about_page(ui);
                }
                if self.show_eris_page {
                    ui.label("eris");
                }
            });
    }
}

// All components of the gui split up into their respective parts
mod cpu;
mod gui;
mod gpu;
mod ram;
mod disk;
mod network;
mod temperature;
mod eris;
mod settings;
mod help;
mod about;

// This will take in startup config later!
pub fn start_nyx(icon: IconData, settings: Settings) {
    let app_name = "Nyx";
    let size: Vec2 = settings.display_size;
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    native_options.viewport.icon = Option::from(Arc::from(icon));
    run_native(app_name, native_options, Box::new(|_cc| { Box::new(Nyx::new(settings)) })).expect("E 01");
}

