use std::collections::VecDeque;

use chrono::{Utc, Duration, SecondsFormat};
use eframe::{*, epaint::{Vec2, Color32}, egui::{CentralPanel, Ui, ScrollArea, Grid}};
use egui_plot::{BarChart, Bar, Plot};

use crate::time_now_rfc3339zulu;

use crate::{APPNAME, APPVERSION, APPAUTHORS, comp::{network::Network, disc::Disk, cpu::CpuData}};

const DATAUPDATEINTERVAL: i64 = 1;


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
    show_temperature_page: bool,
    
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
        let next_data_update = next_update_time();
        let cpu_data = CpuData::new(test_data.clone(), num_cores);
        Nyx { 
            test_data, num_cores,  display_size, networks, disks, next_data_update, cpu_data,
            // default true
            show_landing_page: true,
            // default false
            show_cpu_page: false, show_ram_page: false, show_help: false, show_gpu_page: false, show_disk_page: false, show_temperature_page: false,
        }
    }

}

impl App for Nyx {

    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default()
            .show(ctx, |ui: &mut Ui| {
                if time_now_rfc3339zulu(SecondsFormat::Secs) >= self.next_data_update {
                    self.next_data_update = next_update_time();
                }
                self.draw_main_menu(ui);
                if self.show_help {
                }
                ui.separator();
                if self.show_landing_page {
                    self.draw_landing_page(ui);
                }
                if self.show_cpu_page {
                }
                if self.show_ram_page {
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
        self.show_temperature_page = false;
    }

    fn draw_main_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.menu_button("Nyx", |ui: &mut Ui| {
                if ui.button("Settings").clicked() {
                    println!("Settings click");
                }
                if ui.button("Help").clicked() {
                    println!("Help click");
                }
                if ui.button("About").clicked() {
                    println!("About click");
                }
                if ui.button("Eris").clicked() {
                    println!("Eris click");
                }
            });
            if ui.button("CPU").clicked() {
                println!("CPU");
            }
            if ui.button("GPU").clicked() {
                println!("GPU");
            }
            if ui.button("RAM").clicked() {
                println!("RAM");
            }
            if ui.button("DISC").clicked() {
                println!("DISC");
            }
            if ui.button("NETWORKS").clicked() {
                println!("NETWORKS");
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
                let labels = {
                    let mut out: Vec<String> = Default::default();
                    for n in 1..=self.num_cores {
                        let label = format!("CPU - Core {n}");
                        out.push(label);
                    }
                    out
                };
                for core in labels {
                    ui.label(core.as_str());
                }
                // the two for loops above can be integrated later!    
                ui.label("Average CPU load");
                ui.end_row();
                // I really dislike the cloning of the needed for data readout from appstate here, so I think a index based approach may be adviseable
                // Done so, clone just moved lower into the function stack. VecDeque seems to be
                // the culprit.
                // After consulting with google and bard, I have come to the conclusion that a
                // singly linked list seemst to be the best move for a copyable list perfect
                // for my needs.
                    // f64 can be copyied, I checked
                    // singly linked lists seem to not be a part of the std library -> Something to implement myself, nice!
                    // a doublely linked list is, using it, however, seems boring.
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
                "avg" => (self.cpu_data.avg_load.clone(), "avg load".to_string()),
                _ => (self.cpu_data.core_data[index].clone(), format!("CPU {core_nr}")),
            };
            let chart = BarChart::new(data.iter()
                .enumerate()
                .map(|x| (x.1, x.0 as f64))
                .map(|(x, y)| Bar::new(y, *x).width(1.0))
                .collect()
            )
            .color(Color32::GOLD);
            let cpu_plot = Plot::new(name)
                .show_axes(false)
                .y_axis_width(3)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(100.0)
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
            Grid::new("RAM").striped(true).min_col_width((self.display_size.x / 2.0) - 50.0).num_columns(2).show(ui, |ui: &mut Ui| {
                ui.label("Swap:");
                ui.label("Memory:");
                ui.end_row();
                self.draw_ram_usage(ui, "swap");
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

            let ram_plot = Plot::new(format!("{mem_swap} Usage").as_str())
                .show_axes(false)
                .y_axis_width(3)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(100.0)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            // If code below is changed, change it to the same in `draw_cpu_avg_load`
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
            Grid::new("Disks").striped(true).min_col_width((self.display_size.x / 2.0) - 50.0).num_columns(self.disks.len()).show(ui, |ui: &mut Ui| {
                for disk in &self.disks {
                    ui.label(disk.name.clone());
                }
                ui.end_row();
                for desk in self.disks.clone() {
                    self.draw_disk_usage(ui, desk.clone());
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

            let ram_plot = Plot::new(format!("{} Usage", disk.name).as_str())
                .show_axes(false)
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

    fn gird_networks_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("Networks").striped(true).min_col_width((self.display_size.x / 2.0) - 50.0).num_columns(self.networks.len()).show(ui, |ui: &mut Ui| {
                for network in &self.networks {
                    ui.label(network.name.clone());
                }
                ui.end_row();
                for network in self.networks.clone() {
                    self.draw_network_usage(ui, network.clone());
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

            let ram_plot = Plot::new(format!("{} Usage", network.name).as_str())
                .show_axes(false)
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

    fn grid_gpu_landing_page(&self) {
        // WIP
    }
}

fn next_update_time() -> String {
    let now = Utc::now().checked_add_signed(Duration::seconds(DATAUPDATEINTERVAL));
    if now.is_some() {
        return now.unwrap().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    } else {
        return Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    }
}

// This will take in startup config later!
pub fn start_nyx() {
    let app_name = "Nyx";
    let size: Vec2 = Vec2 { x: 1200.0, y: 900.0 };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(app_name, native_options, Box::new(|_cc| { Box::<Nyx>::default()})).expect("E 01");
}

