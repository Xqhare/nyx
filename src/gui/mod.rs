
use std::sync::Arc;

use chrono::{Duration, SecondsFormat};
use eframe::{epaint::{Vec2, Pos2}, egui::{CentralPanel, Ui, IconData, Context, Window}, run_native, NativeOptions, App, Frame};


use crate::{utils::{self, settings::Settings}, comp::{ram::RamData, disk::Disks, network::Networks, cpu::CpuData, temperature::Temperatures, process::{ProcessData, Processes}}};

struct Nyx {
    // AppData
    next_data_update: String,
    next_process_update: String,
    // Data
    networks: Networks,
    disks: Disks,
    cpu_data: CpuData,
    ram_data: RamData,
    temperatures: Temperatures,
    process_data: ProcessData,
    processes: Processes,

    // Drawing booleans
    show_landing_page: bool,
    show_help: bool,
    show_cpu_page: bool,
    show_process_page: bool,
    show_ram_page: bool,
    show_gpu_page: bool,
    show_disk_page: bool,
    show_network_page: bool,
    show_temperature_page: bool,
    show_settings_page: bool,
    show_advanced_settings_page: bool,
    show_about_page: bool,
    show_eris_page: bool,
    show_minimal_view: bool,
    show_success_msg: bool,
    show_error_msg: bool,
    
    // Settings
    settings: Settings,
}

impl Default for Nyx {

    fn default() -> Self {
        let networks = Networks::new();
        let disks = Disks::new();
        // TODO Put display_size into settings
        let next_data_update = utils::utils::next_update_time(Duration::try_milliseconds(1000).unwrap());
        let next_process_update = utils::utils::next_update_time(Duration::try_milliseconds(15000).unwrap());
        let cpu_data = CpuData::new();
        let ram_data = RamData::new();
        let temperatures = Temperatures::new();
        let settings = Settings::default();
        let process_data = ProcessData::new();
        let processes = Processes::new();
        Nyx { 
            networks, disks, next_data_update, cpu_data, ram_data, temperatures, settings, process_data, processes, next_process_update,
            // default true
            show_landing_page: true,
            // default false
            show_cpu_page: false, show_ram_page: false, show_help: false, show_gpu_page: false, show_disk_page: false, show_temperature_page: false, show_network_page: false,
            show_settings_page: false, show_about_page: false, show_eris_page: false, show_advanced_settings_page: false, show_process_page: false, show_minimal_view: false, show_error_msg: false, show_success_msg: false,
        }
    }

}

impl Nyx {
    fn new(settings: Settings) -> Self {
        let networks = Networks::new();
        let disks = Disks::new();
        // Unwrapping should be fine, it just checks if the duration is larger than an i64, and I
        // don't do that.
        let next_data_update = utils::utils::next_update_time(Duration::try_milliseconds(settings.data_update_interval).unwrap());
        let next_process_update = utils::utils::next_update_time(Duration::try_milliseconds(settings.data_update_interval * 15).unwrap());
        let cpu_data = CpuData::new();
        let ram_data = RamData::new();
        let temperatures = Temperatures::new();
        let process_data = ProcessData::new();
        let processes = Processes::new();
        let settings = settings;
        Nyx { 
            networks, disks, next_data_update, cpu_data, ram_data, temperatures, settings, process_data, processes, next_process_update,
            // default true
            show_landing_page: true,
            // default false
            show_cpu_page: false, show_ram_page: false, show_help: false, show_gpu_page: false, show_disk_page: false, show_temperature_page: false, show_network_page: false,
            show_settings_page: false, show_about_page: false, show_eris_page: false, show_advanced_settings_page: false, show_process_page: false, show_minimal_view: false, show_error_msg: false, show_success_msg: false,
        }
    }
}

impl App for Nyx {

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // This makes sure that Nyx is run continiously with a maximum wait time in millisecounds.
        // This is also the root cause for the unresponsiveness. I know now that eframe has an
        // async mode, I don't want to rewirte this to use it though.
        ctx.request_repaint_after(std::time::Duration::from_millis(self.settings.data_update_interval as u64 / 100));
        CentralPanel::default()
            .show(ctx, |ui: &mut Ui| {
                // I am once again asking for another data update. - Thanks Bernie!
                let time_now = utils::utils::time_now_rfc3339zulu(SecondsFormat::Secs);
                if time_now >= self.next_data_update {
                    self.next_data_update = utils::utils::next_update_time(Duration::try_milliseconds(self.settings.data_update_interval).unwrap());
                    if !self.show_process_page {
                        self.cpu_data.update();
                        self.ram_data.update();
                        self.disks.update();
                        self.networks.update();
                        self.temperatures.update();
                        self.process_data.update();
                    }
                    
                }
                /* if time_now >= self.next_process_update {
                    // Only update the process list when it is shown to the user
                    if self.show_process_page {
                        self.processes = Processes::new();
                    }
                } */
                self.draw_main_menu(ui);
                ui.separator();
                if self.show_help {
                    self.help_page(ui);
                }
                if self.show_landing_page {
                    self.draw_landing_page(ui);
                }
                if self.show_cpu_page {
                    self.cpu_page(ui);
                }
                if self.show_ram_page {
                    self.ram_page(ui);
                }
                // Ref F1
                /* if self.show_gpu_page {
                    ui.label("gpu");
                } */
                if self.show_disk_page {
                    self.disk_page(ui);
                }
                if self.show_network_page {
                    self.network_page(ui);
                }
                if self.show_temperature_page {
                    self.temperature_page(ui);
                }
                if self.show_settings_page {
                    self.draw_settings_page(ui);
                }
                if self.show_process_page {
                    self.new_process_page(ui);
                }
                if self.show_about_page {
                    self.draw_about_page(ui);
                }
                if self.show_eris_page {
                    ui.label("eris");
                }
                if self.show_minimal_view {
                    self.mini_viewer_page(ui);
                }
                if self.show_success_msg {
                Window::new("Success").collapsible(false).resizable(false).default_pos(Pos2::new(self.settings.display_size.x / 2.0, self.settings.display_size.y / 2.0)).open(&mut self.show_success_msg).show(ctx, |ui: &mut Ui| {
                        ui.label("Task done successfully");
                    });
                }
                if self.show_error_msg {
                    Window::new("Error").collapsible(true).resizable(false).open(&mut self.show_error_msg).show(ctx, |ui: &mut Ui| {
                            ui.label("An error has occured. Consider restarting Nyx");
                        });
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
mod process;
mod mini_viewer;

pub fn start_nyx(icon: IconData, settings: Settings) {
    let app_name = "Nyx";
    let size: Vec2 = settings.display_size;
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    native_options.viewport.icon = Option::from(Arc::from(icon));
    native_options.default_theme = settings.dark_theme;
    run_native(app_name, native_options, Box::new(|_cc| { Box::new(Nyx::new(settings)) })).expect("E 01");
}

