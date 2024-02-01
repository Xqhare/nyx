
use chrono::{Duration, SecondsFormat};
use chrono_tz::Tz;
use eframe::{*, epaint::Vec2, egui::{CentralPanel, Ui}};

use crate::{utils, comp::{ram::RamData, disk::Disks}};

use crate::comp::{network::Network, cpu::CpuData};

const DATAUPDATEINTERVAL: i64 = 1000;

struct Nyx {
    // AppData
    next_data_update: String,
    // Data
    test_data: Vec<f64>,
    num_cores: u8,
    networks: Vec<Network>,
    disks: Disks,
    cpu_data: CpuData,
    ram_data: RamData,

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
}

impl Default for Nyx {

    fn default() -> Self {
        let test_data: Vec<f64> = vec![10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.3, 1.0, 2.2, 4.3, 2.6, 3.8, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.3, 1.0, 2.2, 4.3, 2.6, 3.8, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54,];
        let num_cores: u8 = utils::get_cpu_core_amount();
        let networks = vec![Network::new("First".to_string()), Network::new("Second".to_string())];
        let disks = Disks::new();
        // TODO Put display_size into settings
        let display_size: Vec2 = Vec2 { x: 1200.0, y: 900.0 };
        let next_data_update = utils::next_update_time(Duration::milliseconds(DATAUPDATEINTERVAL));
        let cpu_data = CpuData::new();
        let ram_data = RamData::new();
        let timezone = chrono_tz::Europe::Berlin;
        Nyx { 
            test_data, num_cores,  display_size, networks, disks, next_data_update, cpu_data, ram_data, timezone,
            // default true
            show_landing_page: true,
            // default false
            show_cpu_page: false, show_ram_page: false, show_help: false, show_gpu_page: false, show_disk_page: false, show_temperature_page: false, show_network_page: false,
            show_settings_page: false, show_about_page: false, show_eris_page: false,
        }
    }

}

impl App for Nyx {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // This makes sure that Nyx is run continiously with a maximum wait time in millisecounds.
        ctx.request_repaint_after(std::time::Duration::from_millis(DATAUPDATEINTERVAL as u64 / 100));
        CentralPanel::default()
            .show(ctx, |ui: &mut Ui| {
                // Time has come for Data update
                if utils::time_now_rfc3339zulu(SecondsFormat::Secs) >= self.next_data_update {
                    self.next_data_update = utils::next_update_time(Duration::milliseconds(DATAUPDATEINTERVAL));
                    self.cpu_data.update();
                    self.ram_data.update();
                    self.disks.update();
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
                    ui.label("settings");
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
pub fn start_nyx() {
    let app_name = "Nyx";
    let size: Vec2 = Vec2 { x: 1165.0, y: 1000.0 };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(app_name, native_options, Box::new(|_cc| { Box::<Nyx>::default()})).expect("E 01");
}

