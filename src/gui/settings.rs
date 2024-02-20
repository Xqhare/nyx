
use eframe::{egui::{Ui, Grid, ComboBox}, Theme, epaint::Vec2};

use super::Nyx;

impl Nyx {
    pub fn draw_settings_page(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.vertical(|ui: &mut Ui| {
                ui.heading("Date & Time:");
                ComboBox::from_label("Timezone").selected_text(format!("{}", self.settings.timezone)).show_ui(ui, |ui: &mut Ui| {
                    for tz in chrono_tz::TZ_VARIANTS {
                        ui.selectable_value(&mut self.settings.timezone, tz, format!("{}", tz));
                    }
                });
                if ui.radio(self.settings.display_time_ribbon, "Display date & time in ribbon").clicked() {
                    if self.settings.display_time_ribbon {
                        self.settings.display_time_ribbon = false;
                    } else {
                        self.settings.display_time_ribbon = true;
                    }
                }
            });
            ui.vertical(|ui: &mut Ui| {
                ui.heading("Display:");
                ui.label("Window size:");
                ui.horizontal(|ui: &mut Ui| {
                    if ui.text_edit_singleline(&mut self.settings.set_size_x).lost_focus() {
                        self.set_size();
                    }
                    ui.label("x");
                    if ui.text_edit_singleline(&mut self.settings.set_size_y).lost_focus() {
                        self.set_size();
                    }
                });
                ComboBox::from_label("Theme").selected_text(format!("{:?}", self.settings.dark_theme)).show_ui(ui, |ui: &mut Ui| {
                    ui.selectable_value(&mut self.settings.dark_theme, Theme::Light, "Light");
                    ui.selectable_value(&mut self.settings.dark_theme, Theme::Dark, "Dark");
                });
                ui.label("Applies only after restart.");
            });
            ui.vertical(|ui: &mut Ui| {
                ui.heading("Misc:");
                ui.label("Update-Interval in milliseconds:");
                ui.horizontal(|ui: &mut Ui| {
                    if ui.text_edit_singleline(&mut self.settings.set_interval).lost_focus() {
                        let new_i = self.settings.set_interval.parse::<i64>();
                        if new_i.is_ok() {
                            self.settings.data_update_interval = new_i.unwrap();
                        } else {
                            self.settings.set_interval = Default::default();
                        }
                    }
                });
                
            });
        });
        ui.spacing();
        ui.spacing();
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
        ui.spacing();
        ui.separator();
        ui.spacing();
        if ui.button("Save Settings").clicked() {
            // This can go wrong, I should handle this somehow. TODO!
            let _ = self.settings.save(self.settings.save_location.clone());
            ui.label("Saved!");
        }
    }

    fn set_size(&mut self) {
        let x = self.settings.set_size_x.parse();
        let y = self.settings.set_size_y.parse();
        if x.is_ok() && y.is_ok() {
            let new_size = Vec2 { x: x.unwrap(), y: y.unwrap()};
            self.settings.display_size = new_size;
        } else {
            self.settings.set_size_x = Default::default();
            self.settings.set_size_y = Default::default();
        }
    }
}
