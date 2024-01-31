use std::ops::RangeInclusive;

use eframe::{egui::{ScrollArea, Ui, Grid}, epaint::{Color32, Vec2}};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};

use super::Nyx;

impl Nyx {

    pub fn grid_cpu_landing_page(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .hscroll(true)
            .show(ui, |ui: &mut Ui| {
            Grid::new("landing page cpu").striped(true).num_columns(self.num_cores as usize + 1).show(ui, |ui: &mut Ui| {
                for n in 1..=self.num_cores {
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

}

