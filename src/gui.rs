use std::ops::RangeInclusive;

use chrono::{Duration, SecondsFormat};
use eframe::{*, epaint::{Vec2, Color32}, egui::{CentralPanel, Ui, ScrollArea, Grid}};
use egui_plot::{BarChart, Bar, Plot, AxisHints, PlotPoint};

use crate::utils;

use crate::{APPNAME, APPVERSION, APPAUTHORS, comp::{network::Network, disc::Disk, cpu::CpuData}};

const DATAUPDATEINTERVAL: i64 = 1000;


struct Nyx {
    // AppData
    next_data_update: String,
    // Data
    test_data: Vec<f64>,
    num_cores: u8,
    networks: Vec<Network>,
    disks: Vec<Disk>,
    cpu_data: CpuData,

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
}

impl Default for Nyx {

    fn default() -> Self {
        let test_data: Vec<f64> = vec![10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.3, 1.0, 2.2, 4.3, 2.6, 3.8, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.3, 1.0, 2.2, 4.3, 2.6, 3.8, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54,];
        let num_cores: u8 = 12;
        let networks = vec![Network::new("First".to_string()), Network::new("Second".to_string())];
        let disks = vec![Disk::new("One".to_string()), Disk::new("Two".to_string())];
        // TODO Put display_size into settings
        let display_size: Vec2 = Vec2 { x: 1200.0, y: 900.0 };
        let next_data_update = utils::next_update_time(Duration::milliseconds(DATAUPDATEINTERVAL));
        let cpu_data = CpuData::new();
        Nyx { 
            test_data, num_cores,  display_size, networks, disks, next_data_update, cpu_data,
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
        ctx.request_repaint_after(std::time::Duration::from_millis(DATAUPDATEINTERVAL as u64));
        CentralPanel::default()
            .show(ctx, |ui: &mut Ui| {
                // Time has come for Data update
                if utils::time_now_rfc3339zulu(SecondsFormat::Secs) >= self.next_data_update {
                    self.next_data_update = utils::next_update_time(Duration::milliseconds(DATAUPDATEINTERVAL));
                    self.cpu_data.update();
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
                    ui.label("about");
                }
                if self.show_eris_page {
                    ui.label("eris");
                }
            });
    }
}

impl Nyx {

    fn reset_to_landing_page(&mut self) {
        self.clear_screen();
        self.show_landing_page = true;
    }
    
    fn clear_screen(&mut self) {
        self.show_landing_page = false;
        self.show_help = false;
        self.show_cpu_page = false;
        self.show_ram_page = false;
        self.show_gpu_page = false;
        self.show_disk_page = false;
        self.show_network_page = false;
        self.show_temperature_page = false;
        self.show_settings_page = false;
        self.show_about_page = false;
        self.show_eris_page = false;
    }

    fn draw_main_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.menu_button("Nyx", |ui: &mut Ui| {
                if ui.button("Settings").clicked() {
                    println!("Settings click");
                    self.clear_screen();
                    self.show_settings_page = true;
                }
                if ui.button("Help").clicked() {
                    println!("Help click");
                    self.clear_screen();
                    self.show_help = true;
                }
                if ui.button("About").clicked() {
                    println!("About click");
                    self.clear_screen();
                    self.show_about_page = true;
                }
            });
            if ui.button("CPU").clicked() {
                println!("CPU");
                self.clear_screen();
                self.show_cpu_page = true;
            }
            if ui.button("GPU").clicked() {
                println!("GPU");
                self.clear_screen();
                self.show_gpu_page = true;
            }
            if ui.button("RAM").clicked() {
                println!("RAM");
                self.clear_screen();
                self.show_ram_page = true;
            }
            if ui.button("DISC").clicked() {
                println!("DISC");
                self.clear_screen();
                self.show_disk_page = true;
            }
            if ui.button("NETWORKS").clicked() {
                println!("NETWORKS");
                self.clear_screen();
                self.show_network_page = true;
            }
            if ui.button("TEMPERATURE").clicked() {
                println!("TEMPERATURE");
                self.clear_screen();
                self.show_temperature_page = true;
            }
            if ui.button("Eris").clicked() {
                println!("Eris click");
                self.clear_screen();
                self.show_eris_page = true;
            }
            if !self.show_landing_page {
                if ui.button("Back to main page").clicked() {
                    self.reset_to_landing_page();
                }
            }
            ui.spacing();
            ui.separator();
            ui.spacing();
            ui.heading(APPNAME);
            ui.label(" v. ");
            ui.label(APPVERSION);
            ui.label(" by ");
            ui.label(APPAUTHORS);
        });
    }

    fn draw_landing_page(&mut self, ui: &mut Ui) {
        ui.heading("CPU");
        self.grid_cpu_landing_page(ui);
        ui.separator();
        ui.heading("GPU");
        self.grid_gpu_landing_page();
        ui.separator();
        ui.heading("RAM");
        self.grid_ram_landing_page(ui);
        ui.separator();
        ui.heading("Disks");
        self.grid_discs_landing_page(ui);
        ui.separator();
        ui.heading("Networks");
        self.gird_networks_landing_page(ui);
    }

    fn grid_cpu_landing_page(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .hscroll(true)
            .show(ui, |ui: &mut Ui| {
            Grid::new("landing page cpu").striped(true).num_columns(self.num_cores as usize + 1).show(ui, |ui: &mut Ui| {
                for n in 1..=self.num_cores {
                    ui.label(format!("CPU - Core {n}").as_str());
                }
                ui.label("Average CPU load");
                ui.end_row();
                for core in 1..=self.num_cores {
                    self.draw_cpu_core(ui, core, "cpu core");
                }
                self.draw_cpu_core(ui, 0, "avg");
                ui.end_row();
            });
        });
    }

    /// `core_nr` can be set to any number if avg load is drawn.
    /// `avg_core` is only matched for "avg", if it is not avg, it is a core
    fn draw_cpu_core(&mut self, ui: &mut Ui, core_nr: u8, avg_core: &str) {
        // This horizontal puts the Plots nicely close together, without it you need two monitors.
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let index = {
                let index = core_nr.checked_sub(1);
                if index.is_some() {
                    index.unwrap() as usize
                } else {
                    0 as usize
                }
            };
            let (data, name) = match avg_core {
                // I really dislike the cloning of the needed for data readout from appstate here: Ref F1
                "avg" => (self.cpu_data.avg_load.clone(), "avg load".to_string()),
                _ => (self.cpu_data.core_data.lock().unwrap()[index].clone(), format!("CPU {core_nr}")),
            };
            // Locks need an unwrap or similar, keep that in mind!
            let chart = BarChart::new(data.lock().unwrap().iter()
                .enumerate()
                .map(|x| (*x.1, x.0))
                .map(|(x, y)| Bar::new(y as f64, x).width(1.0))
                .collect()
            )
            .color(Color32::GOLD);
            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.2}%", val.x, val.y)};

            let cpu_plot = Plot::new(name)
                .show_axes(false)
                .y_axis_width(3)
                .custom_x_axes(vec![AxisHints::default().label("Time").formatter(x_fmt).max_digits(4)])
                .custom_y_axes(vec![AxisHints::default().label("Usage").formatter(y_fmt).max_digits(4)])
                .label_formatter(label_fmt)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(100.0)
                .include_x(60)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            if cpu_plot.response.clicked(){
                self.cpu_clicked();
            }
        });
    }

    fn cpu_clicked(&mut self) {
        println!("CPU MENU CLICKED");
        self.clear_screen();
        self.show_cpu_page = true;
    }

    fn grid_ram_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("RAM").striped(true).min_col_width((self.display_size.x / 1.0) - 50.0).num_columns(1).show(ui, |ui: &mut Ui| {
                ui.label("Swap:");
                ui.end_row();
                self.draw_ram_usage(ui, "swap");
                ui.end_row();
                ui.label("Memory:");
                ui.end_row();
                self.draw_ram_usage(ui, "ram");
            }).response
        });
    }

    fn draw_ram_usage(&mut self, ui: &mut Ui, mem_swap: &str) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let data = match mem_swap {
                "ram" => &self.test_data,
                _ => &self.test_data,
            };
            let chart = BarChart::new(data.iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x).width(1.0)).collect()
            ).color(Color32::GOLD);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.2}%", val.x, val.y)};

            let ram_plot = Plot::new(format!("{mem_swap} Usage").as_str())
                .show_axes(false)
                .custom_x_axes(vec![AxisHints::default().label("Time").formatter(x_fmt).max_digits(4)])
                .custom_y_axes(vec![AxisHints::default().label("Usage").formatter(y_fmt).max_digits(4)])
                .label_formatter(label_fmt)                
                .y_axis_width(3)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(100.0)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            if ram_plot.response.clicked(){
                self.ram_clicked();
            }
        });
    }

    fn ram_clicked(&mut self) {
        println!("RAM MENU CLICKED");
        self.clear_screen();
        self.show_ram_page = true;
    }

    fn grid_discs_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("Disks").striped(true).min_col_width((self.display_size.x / 1.0) - 50.0).num_columns(1).show(ui, |ui: &mut Ui| {
                for disk in self.disks.clone() {
                    ui.label(disk.name.clone());
                    ui.end_row();
                    self.draw_disk_usage(ui, disk.clone());
                    ui.end_row();
                }
            }).response
        });    
    }

    fn draw_disk_usage(&mut self, ui: &mut Ui, disk: Disk) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(disk.data.iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x).width(1.0)).collect()
            ).color(Color32::GOLD);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.2}%", val.x, val.y)};


            let disk_plot = Plot::new(format!("{} Usage", disk.name).as_str())
                .show_axes(false)
                .y_axis_width(3)
                .custom_x_axes(vec![AxisHints::default().label("Time").formatter(x_fmt).max_digits(4)])
                .custom_y_axes(vec![AxisHints::default().label("Usage").formatter(y_fmt).max_digits(4)])
                .label_formatter(label_fmt) 
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(100.0)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            if disk_plot.response.clicked(){
                self.disk_clicked();
            }
        });
    }

    fn disk_clicked(&mut self) {
            println!("DISK MENU CLICKED");
            self.clear_screen();
            self.show_disk_page = true;
    }

    fn gird_networks_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("Networks").striped(true).min_col_width((self.display_size.x / 1.0) - 50.0).num_columns(1).show(ui, |ui: &mut Ui| {
                for network in self.networks.clone() {
                    ui.label(network.name.clone());
                    ui.end_row();
                    self.draw_network_usage(ui, network.clone());
                    ui.end_row();
                }
            }).response
        });
    }

    fn draw_network_usage(&mut self, ui: &mut Ui, network: Network) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(network.data.iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x).width(1.0)).collect()
            ).color(Color32::GOLD);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.2}%", val.x, val.y)};

            let network_plot = Plot::new(format!("{} Usage", network.name).as_str())
                .show_axes(false)
                .y_axis_width(3)
                .custom_x_axes(vec![AxisHints::default().label("Time").formatter(x_fmt).max_digits(4)])
                .custom_y_axes(vec![AxisHints::default().label("Usage").formatter(y_fmt).max_digits(4)])
                .label_formatter(label_fmt) 
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(100.0)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            if network_plot.response.clicked(){
                self.network_clicked();
            }
        });
    }

    fn network_clicked(&mut self) {
            println!("NETWORK MENU CLICKED");
            self.clear_screen();
            self.show_network_page = true;
    }

    fn grid_gpu_landing_page(&self) {
        // WIP
    }
}

// This will take in startup config later!
pub fn start_nyx() {
    let app_name = "Nyx";
    let size: Vec2 = Vec2 { x: 1165.0, y: 1000.0 };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(app_name, native_options, Box::new(|_cc| { Box::<Nyx>::default()})).expect("E 01");
}

