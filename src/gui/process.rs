use std::ops::RangeInclusive;

use eframe::{egui::{Ui, Grid}, epaint::Vec2};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};

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
                .include_y(5000.0)
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
            println!("DISK MENU CLICKED");
            self.clear_screen();
            self.show_cpu_page = true;
    }
}
