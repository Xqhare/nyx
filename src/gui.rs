
use eframe::{*, epaint::{Vec2, Color32}, egui::{CentralPanel, Ui, ScrollArea, Grid,}};
use egui_plot::{BarChart, Bar, Plot};

pub struct Nyx {
    test_data: Vec<f64>,
}

impl Default for Nyx {
    fn default() -> Self {
        let test_data = vec![10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.4, 56.0, 15.4, 68.7, 91.25, 41.2, 56.47, 41.54, 10.3, 1.0, 2.2, 4.3, 2.6, 3.8];
        Nyx { test_data }
    }
}

impl App for Nyx {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().hscroll(true).show(ui, |ui: &mut Ui| {
                self.draw_main_menu(ui);
            });
        });
    }
}

impl Nyx {
    fn draw_main_menu(&self, ui: &mut Ui) {
        Grid::new("Main Menu").striped(true).show(ui, |ui: &mut Ui| {
            for core in 1..=12 {
                self.draw_main_menu_cpu(ui, 1);
            }
            ui.end_row();

            //self.draw_main_menu_cpu(ui, 14);
        });
    }
    fn draw_main_menu_cpu(&self, ui: &mut Ui, start: usize) {
        // This horizontal puts the Plots nicely close together, without it you need two monitors.
        ui.horizontal(|ui: &mut Ui| {
            let mut chart = BarChart::new(self.test_data.iter().enumerate().map(|x| {
                (x.1, x.0 as f64)
            }).map(|(x, y)| Bar::new(y, *x).width(1.0)).collect()
            ).color(Color32::GOLD);
            chart = chart.vertical();

            Plot::new("CPU").clamp_grid(false).y_axis_width(3).allow_zoom(false).allow_drag(false).allow_scroll(false).allow_boxed_zoom(false).show(ui, |plot_ui| plot_ui.bar_chart(chart));
        });
        
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
