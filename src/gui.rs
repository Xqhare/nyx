
use eframe::{*, epaint::Vec2, egui::{CentralPanel, Ui}};

pub struct Nyx {
}

impl Default for Nyx {
    fn default() -> Self {
        Nyx {  }
    }
}

impl App for Nyx {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Hello World!");
        });
    }
}

// This will take in startup config later!
pub fn start_nyx() {
    let app_name = "Nyx";
    let size: Vec2 = Vec2 { x: 800.0, y: 400.0 };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(app_name, native_options, Box::new(|_cc| { Box::<Nyx>::default()})).expect("E 01");
}
