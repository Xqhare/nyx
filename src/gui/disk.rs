use std::ops::RangeInclusive;

use eframe::{egui::{Ui, Grid}, epaint::Vec2};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};
use crate::comp::disk::Disk;

use super::Nyx;

impl Nyx {
    pub fn grid_disks_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("Disks").striped(true).min_col_width((self.display_size.x / 1.0) - 50.0).num_columns(1).show(ui, |ui: &mut Ui| {
                let disks = self.disks.disks.lock().unwrap().clone();
                for disk in disks {
                    ui.label(disk.name.to_string());
                    ui.end_row();
                    self.draw_disk_usage(ui, disk.clone());
                    ui.end_row();
                };
            }).response
        });    
    }

    fn draw_disk_usage(&mut self, ui: &mut Ui, disk: Disk) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(disk.stat_reads.lock().unwrap().iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x as f64).width(1.0)).collect()
            ).color(self.settings.disk_read_colour);

            let chart2 = BarChart::new(disk.stat_writes.lock().unwrap().iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x as f64).width(0.5)).collect()
            ).color(self.settings.disk_write_colour);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.0}", val.x, val.y)};


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
                .include_y(1000.0)
                .include_x(60)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(chart);
                    plot_ui.bar_chart(chart2);
                });
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


}
