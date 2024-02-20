
use eframe::egui::{Ui, Grid};

use super::Nyx;

impl Nyx {
    pub fn draw_settings_page(&mut self, ui: &mut Ui) {
        Grid::new("Coloursettingsgrid").striped(true).num_columns(2).show(ui, |ui: &mut Ui| {
            ui.heading("Colours:");
            ui.end_row();
            ui.label("CPU usage colour:");
            ui.color_edit_button_srgba(&mut self.settings.cpu_colour);
            ui.end_row();
            ui.label("RAM usage colour:");
            ui.color_edit_button_srgba(&mut self.settings.ram_colour);
            ui.end_row();
            ui.label("Network traffic colour:");
            ui.color_edit_button_srgba(&mut self.settings.network_colour);
            ui.end_row();
            ui.label("Network traffic error colour:");
            ui.color_edit_button_srgba(&mut self.settings.network_error_colour);
            ui.end_row();
            ui.label("Disk write colour:");
            ui.color_edit_button_srgba(&mut self.settings.disk_write_colour);
            ui.end_row();
            ui.label("Disk read colour:");
            ui.color_edit_button_srgba(&mut self.settings.disk_read_colour);
            ui.end_row();
            ui.label("Temperature colour:");
            ui.color_edit_button_srgba(&mut self.settings.temperature_colour);
        });
        
    }
}
