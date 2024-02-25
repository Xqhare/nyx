use std::ops::RangeInclusive;

use eframe::{egui::{Ui, Grid}, epaint::Vec2};
use egui_plot::{BarChart, Bar, PlotPoint, Plot, AxisHints};

use crate::comp::network::Network;

use super::Nyx;

impl Nyx {
    pub fn gird_networks_landing_page(&mut self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("Networks").striped(true).min_col_width((self.settings.display_size.x / 2.0) - 50.0).num_columns(2).show(ui, |ui: &mut Ui| {
                let networks = self.networks.networks.clone();
                for network in networks.lock().unwrap().iter() {
                    ui.label(format!("{} | Outgoing: {}kb/s", network.name, network.outgoing.lock().unwrap().front().unwrap()));
                    ui.label(format!("Incoming: {}kb/s", network.incoming.lock().unwrap().front().unwrap()));
                    ui.end_row();
                    self.draw_network_usage(ui, network.clone(), "out");
                    self.draw_network_usage(ui, network.clone(), "inc");
                    ui.end_row();
                }
            }).response
        });
    }

    fn draw_network_usage(&mut self, ui: &mut Ui, network: Network, inc_vs_out: &str) {
        let (data, error) = {
            match inc_vs_out {
                "inc" => {
                    let data = network.incoming.lock();
                    let error = network.errors_incoming.lock();
                    (data, error)
                },
                _ => {
                    let data = network.outgoing.lock();
                    let error = network.errors_outgoing.lock();
                    (data, error)
                },
            }
        };
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(data.unwrap().iter().enumerate().map(|x| {
            (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, mexprp::eval::<f64>(format!("{x} / 1024").as_str()).unwrap().unwrap_single()).width(1.0)).collect()
            ).color(self.settings.network_colour);

            let err_chart = BarChart::new(error.unwrap().iter().enumerate().map(|x| {
            (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, mexprp::eval::<f64>(format!("{x} / 1024").as_str()).unwrap().unwrap_single()).width(0.5)).collect()
            ).color(self.settings.network_error_colour);

            let x_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Time".to_string()};
            let y_fmt = |_x, _digits, _range: &RangeInclusive<f64>| {"Usage".to_string()};
            // the :.2 in the {} means that the supplied values are cut of 2 digits after the . seperator
            let label_fmt = |_s: &str, val: &PlotPoint| {format!("{:.2}s\n{:.0}Kb", val.x, val.y)};

            let network_plot = Plot::new(format!("{} {}", network.name.clone(), inc_vs_out).as_str())
                .show_axes(false)
                .y_axis_width(3)
                .custom_x_axes(vec![AxisHints::default().label("Time").formatter(x_fmt).max_digits(4)])
                .custom_y_axes(vec![AxisHints::default().label("Usage").formatter(y_fmt).max_digits(4)])
                .label_formatter(label_fmt) 
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .include_y(13500.0)
                .include_x(60.0)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(chart);
                    plot_ui.bar_chart(err_chart);
                }
                );
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


}
