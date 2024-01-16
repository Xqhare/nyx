
use eframe::{*, epaint::{Vec2, Color32}, egui::{CentralPanel, Ui, ScrollArea, Grid}};
use egui_plot::{BarChart, Bar, Plot};

use crate::{APPNAME, APPVERSION, APPAUTHORS};

struct Nyx {
    test_data: Vec<f64>,
    num_cores: u8,
    show_help: bool,
    display_size: Vec2,
}

impl Default for Nyx {

    fn default() -> Self {
        let test_data = vec![10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.3, 1.0, 2.2, 4.3, 2.6, 3.8, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.3, 1.0, 2.2, 4.3, 2.6, 3.8, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54,];
        let num_cores: u8 = 12;
        let show_help = false;
        // TODO Put display_size into settings
        let display_size: Vec2 = Vec2 { x: 1200.0, y: 900.0 };
        Nyx { test_data, num_cores, show_help, display_size }
    }

}

impl App for Nyx {

    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default()
            .show(ctx, |ui: &mut Ui| {
                self.draw_main_menu(ui);
                ui.separator();
                self.draw_landing_page(ui);
                
            });
    }

}

impl Nyx {
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
        self.grid_cpu_landing_page(ui);
        self.grid_gpu_landing_page();
        self.grid_ram_landing_page(ui);
        self.grid_discs_landing_page();
        self.gird_networks_landing_page();
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
                ui.label("Average CPU load");
                ui.end_row();
                for core in 1..=self.num_cores {
                    self.draw_cpu_core(ui, core);
                }
                self.draw_cpu_avg_load(ui);
                ui.end_row();
            });
        });
    }

    fn draw_cpu_avg_load(&mut self, ui: &mut Ui) {
        ui.horizontal_centered(|ui: &mut Ui| {
            let mut chart = BarChart::new(self.test_data.iter()
                .enumerate()
                .map(|x| (x.1, x.0 as f64))
                .map(|(x, y)| Bar::new(y, *x).width(1.0))
                .collect()
                )
                .color(Color32::GOLD);
            chart = chart.vertical();
            chart = chart.name("Avg Cpu load");
            let cpu_avg = Plot::new("CPU Avg load")
                .y_axis_width(3)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .show_axes(false)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            // If code below is changed, change it to the same in `draw_cpu_avg_load`
            if cpu_avg.response.clicked(){
                self.cpu_clicked();
            }
        });
    }

    fn draw_cpu_core(&self, ui: &mut Ui, start: u8) {
        // This horizontal puts the Plots nicely close together, without it you need two monitors.
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(self.test_data.iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x).width(1.0)).collect()
            ).color(Color32::GOLD);

            let cpu_plot = Plot::new(format!("CPU {start}").as_str())
                .show_axes(false)
                .y_axis_width(3)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            // If code below is changed, change it to the same in `draw_cpu_avg_load`
            if cpu_plot.response.clicked(){
                self.cpu_clicked();
            }
        });
    }

    fn cpu_clicked(&self) {
        println!("CPU MENU CLICKED")
    }

    fn grid_ram_landing_page(&self, ui: &mut Ui) {
        ui.add(|ui: &mut Ui| {
            Grid::new("RAM").striped(true).min_col_width((self.display_size.x / 2.0) - 50.0).num_columns(2).show(ui, |ui: &mut Ui| {
                ui.label("Swap:");
                ui.label("Memory:");
                ui.end_row();
                self.draw_swap_usage(ui);
                self.draw_ram_usage(ui);
            }).response
        });
    }

    fn draw_swap_usage(&self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(self.test_data.iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x).width(1.0)).collect()
            ).color(Color32::GOLD);

            let swap_plot = Plot::new("Swap Usage")
                .show_axes(false)
                .y_axis_width(3)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            // If code below is changed, change it to the same in `draw_cpu_avg_load`
            if swap_plot.response.clicked(){
                println!("Swap CLICKED")
            }
        });
    }

    fn draw_ram_usage(&self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui: &mut Ui| {
            let chart = BarChart::new(self.test_data.iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x).width(1.0)).collect()
            ).color(Color32::GOLD);

            let ram_plot = Plot::new("Memory Usage")
                .show_axes(false)
                .y_axis_width(3)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .set_margin_fraction(Vec2 { x: 0.0, y: 0.0 })
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
            // If code below is changed, change it to the same in `draw_cpu_avg_load`
            if ram_plot.response.clicked(){
                println!("RAM CLICKED")
            }
        });
    }

    fn grid_discs_landing_page(&self) {
        // WIP
    }

    fn gird_networks_landing_page(&self) {
        // WIP
    }

    fn grid_gpu_landing_page(&self) {
        // WIP
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

