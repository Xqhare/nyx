use std::ops::RangeInclusive;

use eframe::{egui::{ScrollArea, Ui, Grid}, epaint::Vec2};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};

use super::Nyx;

impl Nyx {

    pub fn grid_cpu_landing_page(&mut self, ui: &mut Ui) {
        ScrollArea::new([true, false])
            .hscroll(true)
            .show(ui, |ui: &mut Ui| {
            Grid::new("landing page cpu").striped(true).num_columns(self.cpu_data.num_cores as usize + 1).show(ui, |ui: &mut Ui| {
                for n in 1..=self.cpu_data.num_cores {
                    ui.label(format!("CPU - Core {n}").as_str());
                }
                ui.add(|ui: &mut Ui| {
                        ui.horizontal(|ui: &mut Ui| {
                            ui.label("Average CPU load");
                            ui.spacing();
                            ui.separator();
                            ui.spacing();
                            let label = format!("{:.5}%", self.cpu_data.avg_load.lock().unwrap().front().unwrap());
                            ui.label(&label);
                        }).response
                });
                ui.end_row();
                for core in 1..=self.cpu_data.num_cores {
                    self.draw_cpu_core(ui, core, "cpu core");
                }
                self.draw_cpu_core(ui, 0, "avg");
                ui.end_row();
            });
        });
    }

    pub fn mini_cpu(&mut self, ui: &mut Ui) {
        let (data, name, colour) = {
                (self.cpu_data.avg_load.clone(), "avg load".to_string(), self.settings.cpu_avg_colour)
            };
            let chart = BarChart::new(data.lock().unwrap().iter()
                .enumerate()
                .map(|x| (*x.1, x.0))
                .map(|(x, y)| Bar::new(y as f64, x).width(1.0))
                .collect()
            )
            .color(colour);
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
            let (data, name, colour) = match avg_core {
                // I really dislike the cloning of the needed for data readout from appstate here: Ref E1
                "avg" => (self.cpu_data.avg_load.clone(), "avg load".to_string(), self.settings.cpu_avg_colour),
                _ => (self.cpu_data.core_data.lock().unwrap()[index].clone(), format!("CPU {core_nr}"), self.settings.cpu_colour),
            };
            // Locks need an unwrap or similar, keep that in mind!
            let chart = BarChart::new(data.lock().unwrap().iter()
                .enumerate()
                .map(|x| (*x.1, x.0))
                .map(|(x, y)| Bar::new(y as f64, x).width(1.0))
                .collect()
            )
            .color(colour);
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
        self.clear_screen();
        self.show_cpu_page = true;
    }

    pub fn cpu_page(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .vscroll(true)
            .show(ui, |ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.vertical(|ui: &mut Ui| {
                        ui.heading("System Info");
                        ui.horizontal(|ui: &mut Ui| {
                            if self.cpu_data.phy_cores.is_some() {
                                ui.label(format!("Number of physical cores: {}", self.cpu_data.phy_cores.unwrap()));
                                ui.separator();
                            }
                            ui.label(format!("Number of total cores: {}", self.cpu_data.num_cores));
                            
                        });
                        ui.horizontal(|ui: &mut Ui| {
                            if self.cpu_data.sys_name.is_some() {
                                ui.label(format!("System name: {}", self.cpu_data.sys_name.clone().unwrap()));
                            }
                            if self.cpu_data.host_name.is_some() {
                                ui.separator();
                                ui.label(format!("Host name: {}", self.cpu_data.host_name.clone().unwrap()));
                            }
                        });
                        ui.horizontal(|ui: &mut Ui| {
                            if self.cpu_data.os_ver.is_some() {
                                ui.label(format!("OS version: {}", self.cpu_data.os_ver.clone().unwrap()));
                            }
                            if self.cpu_data.kernel_ver.is_some() {
                                ui.separator();
                                ui.label(format!("Kernel version: {}", self.cpu_data.kernel_ver.clone().unwrap()));
                            }
                        });
                    });
                    ui.vertical(|ui: &mut Ui| {
                        ui.heading("CPU Info");
                        ui.horizontal(|ui: &mut Ui| {
                            ui.label(format!("CPU Name: {}", self.cpu_data.cpu_name));
                            ui.label(format!("CPU Brand: {}", self.cpu_data.cpu_brand));
                            ui.label(format!("CPU Vendor: {}", self.cpu_data.cpu_vendor));
                        });
                        ui.label(format!("CPU Frequency: {}", self.cpu_data.cpu_frequency));
                    });
                });
                
                ui.separator();
                Grid::new("page cpu").striped(true).num_columns(1).show(ui, |ui: &mut Ui| {
                    ui.add(|ui: &mut Ui| {
                        ui.horizontal(|ui: &mut Ui| {
                            ui.label("Average CPU load");
                            ui.spacing();
                            ui.separator();
                            ui.spacing();
                            let label = format!("{:.5}%", self.cpu_data.avg_load.lock().unwrap().front().unwrap());
                            ui.label(&label);
                        }).response
                    });
                    ui.end_row();
                    self.draw_cpu_core(ui, 0, "avg");
                    ui.end_row();
                    for n in 1..=self.cpu_data.num_cores {
                        ui.label(format!("CPU - Core {n} | Load: {:.5}%", self.cpu_data.core_data.lock().unwrap().get(n as usize - 1).unwrap().lock().unwrap().front().unwrap()).as_str());
                        ui.end_row();
                        self.draw_cpu_core(ui, n, "cpu core");
                        ui.end_row();
                    }
                
                });
            });
    }
}

