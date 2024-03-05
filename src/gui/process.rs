use std::ops::RangeInclusive;

use eframe::{egui::{Ui, Grid, ScrollArea}, epaint::Vec2};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};
use sysinfo::Process;

use crate::comp::process::Processes;

use super::Nyx;

impl Nyx {
    pub fn grid_process_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("Process").striped(true).min_col_width((self.settings.display_size.x / 1.0) - 50.0).num_columns(1).show(ui, |ui: &mut Ui| {
                ui.label(format!("Total processes: {}", self.process_data.amount_processes.lock().unwrap().front().unwrap()));
                ui.end_row();
                self.draw_process_usage(ui);
            }).response
        });    
    }

    fn draw_process_usage(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(self.process_data.amount_processes.lock().unwrap().iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x as f64).width(1.0)).collect()
            ).color(self.settings.process_data_colour);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Processes".to_string()};
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.0}", val.x, val.y)};

            let processdata_plot = Plot::new("total_amount_processes")
                .show_axes(false)
                .y_axis_width(3)
                .custom_x_axes(vec![AxisHints::default().label("Time").formatter(x_fmt).max_digits(4)])
                .custom_y_axes(vec![AxisHints::default().label("Usage").formatter(y_fmt).max_digits(4)])
                .label_formatter(label_fmt) 
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(4000.0)
                .include_x(60)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(chart);
                });
            if processdata_plot.response.clicked(){
                self.process_clicked();
            }
        });
    }

    fn process_clicked(&mut self) {
            self.clear_screen();
            self.show_process_page = true;
    }

    pub fn process_page(&mut self, ui: &mut Ui) {
        Grid::new("proc grid column names").show(ui, |ui: &mut Ui| {
            let col_names = vec!["Name", "PID", "Memory", "Status", "Runtime", "Parent PID"];
            for name in col_names {
                ui.label(name);
            }
            ui.end_row();
        });
        if ui.button("Update").clicked() {
            self.processes = Processes::new();
        }
        ScrollArea::new([false, true]).show(ui, |ui: &mut Ui| {
            Grid::new("Process grid").striped(true).show(ui, |ui: &mut Ui| {
                let data_store = self.processes.processes.lock();
                if data_store.is_ok() {
                    let ok_store = data_store.unwrap();
                    for proc in ok_store.iter() {
                        ui.label(format!("{}", proc.name));
                        ui.label(format!("{}", proc.pid));
                        ui.label(format!("{}", proc.mem));
                        ui.label(format!("{}", proc.status));
                        ui.label(format!("{}", proc.runtime));
                        if proc.parent_pid.is_some() {
                            ui.label(format!("{}", proc.parent_pid.unwrap()));
                        } else {
                            ui.label("");
                        }
                        ui.end_row();
                    }
                }
            });
        });
    }

    pub fn new_process_page(&mut self, ui: &mut Ui) {
        if ui.button("Update").clicked() {
            self.processes = Processes::new();
        }
        ScrollArea::new([false, true]).show(ui, |ui: &mut Ui| {
            let data_store = self.processes.processes.lock();
            if data_store.is_ok() {
                let ok_store = data_store.unwrap();
                for proc in ok_store.iter() {
                    ui.add(|ui: &mut Ui| {
                        ui.horizontal(|ui: &mut Ui| {
                            ui.label(format!("{}", proc.name));
                            ui.separator();
                            ui.label(format!("PID: {}", proc.pid));
                            ui.separator();
                            ui.label(format!("Memory usage: {}", proc.mem));
                            ui.separator();
                            ui.label(format!("Status: {}", proc.status));
                            ui.separator();
                            ui.label(format!("Runtime: {}", proc.runtime));
                            ui.separator();
                            if proc.parent_pid.is_some() {
                                ui.label(format!("Parent PID: {}", proc.parent_pid.unwrap()));
                            } else {
                                ui.label("");
                            }
                            ui.separator();
                            ui.menu_button("Actions", |ui: &mut Ui| {
                                if ui.button("Kill").clicked() {
                                    let pid = proc.pid;
                                    let aa = 0;
                                }
                                if ui.button("Stop").clicked() {
                                }
                            });
                        }).response
                        
                    });
                }
            }

        });
    }
}
