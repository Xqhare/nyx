use std::ops::RangeInclusive;

use eframe::{egui::{Ui, Grid}, epaint::Vec2};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};
use super::Nyx;

impl Nyx {

    pub fn grid_ram_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("RAM").striped(true).min_col_width((self.settings.display_size.x / 2.0) - 50.0).num_columns(2).show(ui, |ui: &mut Ui| {
                ui.add(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        ui.label("Memory:");
                        ui.spacing();
                        ui.separator();
                        ui.spacing();
                        let available = format!("Total: {} bytes / Used: {} bytes", self.ram_data.total_mem.lock().unwrap(), self.ram_data.mem_used.lock().unwrap());
                        ui.label(&available);
                        ui.spacing();
                        ui.separator();
                        ui.spacing();
                        let usage = format!("Usage: {:.5}%", self.ram_data.memory.lock().unwrap().front().unwrap());
                        ui.label(&usage);
                    }).response
                });
                ui.add(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        ui.label("Swap:");
                        ui.spacing();
                        ui.separator();
                        ui.spacing();
                        let available = format!("Total: {} bytes / Used: {} bytes", self.ram_data.total_swap.lock().unwrap(), self.ram_data.swap_used.lock().unwrap());
                        ui.label(&available);
                        ui.spacing();
                        ui.separator();
                        ui.spacing();
                        let usage = format!("Usage: {:.5}%", self.ram_data.swap.lock().unwrap().front().unwrap());
                        ui.label(&usage);
                    }).response
                });
                ui.end_row();
                self.draw_ram_usage(ui, "ram");
                self.draw_ram_usage(ui, "swap");
            }).response
        });
    }

    fn draw_ram_usage(&mut self, ui: &mut Ui, mem_swap: &str) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let data = match mem_swap {
                "ram" => self.ram_data.memory.clone(),
                _ => self.ram_data.swap.clone(),
            };
            let chart = BarChart::new(data.lock().unwrap().iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, (*x) as f64).width(1.0)).collect()
            ).color(self.settings.ram_colour);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.2}%", val.x, val.y)};

            let ram_plot = Plot::new(format!("{mem_swap} Usage").as_str())
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


}
