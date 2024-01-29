
use super::Nyx;
use std::ops::RangeInclusive;

use eframe::{epaint::{Vec2, Color32}, egui::{Ui, Grid}};
use egui_plot::{BarChart, Bar, Plot, AxisHints, PlotPoint};

use crate::{APPNAME, APPVERSION, APPAUTHORS, comp::{network::Network, disc::Disk}};

impl Nyx {

    fn reset_to_landing_page(&mut self) {
        self.clear_screen();
        self.show_landing_page = true;
    }
    
    pub fn clear_screen(&mut self) {
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

    pub fn draw_main_menu(&mut self, ui: &mut Ui) {
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

    pub fn draw_landing_page(&mut self, ui: &mut Ui) {
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
