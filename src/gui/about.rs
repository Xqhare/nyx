
use super::Nyx;

use crate::{APPNAME, APPVERSION, APPAUTHORS};

use eframe::egui::Ui;

impl Nyx {
    
    pub fn draw_about_page(&self, ui: &mut Ui) {
        ui.spacing();
        ui.vertical_centered_justified(|ui: &mut Ui| {
            ui.heading(APPNAME);
            ui.label(" v. ");
            ui.label(APPVERSION);
            ui.label(" by ");
            ui.label(APPAUTHORS);
            ui.spacing();
            ui.hyperlink_to("Nyx on github", "https://github.com/Xqhare/nyx");
        });
    }
}
