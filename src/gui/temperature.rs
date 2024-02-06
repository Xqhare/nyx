use std::ops::RangeInclusive;

use eframe::{egui::{Ui, Grid}, epaint::{Color32, Vec2}};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};


use crate::comp::temperature::Temperature;

use super::Nyx;

impl Nyx {
    pub fn gird_temperature_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            let max_len = self.temperatures.components.lock().unwrap().iter().map(|v| v.iter().len()).max().unwrap();
            Grid::new("Temperatures").striped(true).min_col_width((self.display_size.x / max_len as f32) - 50.0).num_columns(max_len).show(ui, |ui: &mut Ui| {
                let temperatures = self.temperatures.components.clone();
                for temperature in temperatures.lock().unwrap().iter() {
                    for comp in temperature {
                        let name = format!("{} {}",comp.name,comp.sensor);
                        ui.label(name);
                    }
                    ui.end_row();
                    for comp in temperature {
                        self.draw_temperature_usage(ui, comp.clone());
                    }
                    ui.end_row();
                }
            }).response
        });
    }

    fn draw_temperature_usage(&mut self, ui: &mut Ui, temperature: Temperature) {
        let data = temperature.temperature.lock();
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(data.unwrap().iter().enumerate().map(|x| {
            (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x as f64).width(1.0)).collect()
            ).color(Color32::GOLD);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.0}\u{00B0}C", val.x, val.y)};

            let temperature_plot = Plot::new(format!("{} {}", temperature.name.clone(), temperature.sensor.clone()).as_str())
                .show_axes(false)
                .y_axis_width(3)
                .custom_x_axes(vec![AxisHints::default().label("Time").formatter(x_fmt).max_digits(4)])
                .custom_y_axes(vec![AxisHints::default().label("Usage").formatter(y_fmt).max_digits(4)])
                .label_formatter(label_fmt) 
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(mexprp::eval::<f64>(format!("{} + 25", temperature.critical_temperature).as_str()).unwrap().unwrap_single())
                .include_x(60.0)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(chart);
                }
                );
            if temperature_plot.response.clicked(){
                self.temperature_clicked();
            }
        });
    }

    fn temperature_clicked(&mut self) {
            println!("TEMPERATURE MENU CLICKED");
            self.clear_screen();
            self.show_temperature_page = true;
    }


}
