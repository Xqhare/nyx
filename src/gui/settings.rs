
use eframe::{egui::Ui, epaint::Color32};

use super::Nyx;

impl Nyx {
    pub fn draw_settings_page(&self, ui: &mut Ui) {
        ui.heading("Colours:");
        let mut tmp: Color32 = Color32::GOLD;
        ui.color_edit_button_srgba(&mut tmp);
        println!("{:?}", tmp)
    }
}
