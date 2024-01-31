
use super::Nyx;

use crate::{APPNAME, APPVERSION, APPAUTHORS};

use eframe::{egui::{Ui, Grid, Layout}, emath::Align};

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
            ui.spacing();
            ui.separator();
            ui.spacing();
            ui.heading("Dependencies and Licenses");
            
        });
        // This should work, but doesn't because egui hasn't implemented centering grids
        // jet. Ref: https://github.com/emilk/egui/issues/2247
        ui.with_layout(Layout::top_down(Align::Center).with_main_align(Align::Center).with_cross_align(Align::Center), |ui: &mut Ui| {
            Grid::new("License").striped(true).show(ui, |ui: &mut Ui| {
                ui.hyperlink_to("chrono", "https://crates.io/crates/chrono");
                ui.hyperlink_to("License", "https://github.com/chronotype/chrono/blob/main/LICENSE.txt");
                ui.end_row();
                ui.hyperlink_to("chrono-tz", "https://crates.io/crates/chrono-tz");
                ui.hyperlink_to("License", "https://github.com/chronotype/chrono-tz/blob/main/LICENSE");
                ui.end_row();
                ui.hyperlink_to("eframe", "https://crates.io/crates/eframe");
                ui.hyperlink_to("License", "https://github.com/emilk/egui/blob/master/LICENSE-MIT");
                ui.end_row();
                ui.hyperlink_to("sysinfo", "https://crates.io/crates/sysinfo");
                ui.hyperlink_to("License", "https://github.com/GuillaumeGomez/sysinfo/blob/master/LICENSE");
                ui.end_row();
                ui.hyperlink_to("libdrm_amdgpu_sys", "https://crates.io/crates/libdrm_amdgpu_sys");
                ui.hyperlink_to("License", "https://github.com/Umio-Yasuno/libdrm-amdgpu-sys-rs/blob/main/LICENSE");
                ui.end_row();
                ui.hyperlink_to("dirs", "https://crates.io/crates/dirs");
                ui.hyperlink_to("License", "https://github.com/dirs-dev/dirs-rs/blob/main/LICENSE-MIT");
                ui.end_row();
                ui.hyperlink_to("mexprp", "https://crates.io/crates/mexprp");
                ui.hyperlink_to("License", "https://github.com/InterpidPig/mexprp/blob/master/LICENSE");
                ui.end_row();
                ui.hyperlink_to("procfs", "https://crates.io/crates/procfs");
                ui.hyperlink_to("License", "https://github.com/eminence/procfs/blob/master/LICENSE-MIT");
                ui.end_row();
                ui.hyperlink_to("rand", "https://crates.io/crates/rand");
                ui.hyperlink_to("License", "https://github.com/rust-random/rand/blob/master/LICENSE-MIT");
                ui.end_row();
            });
        });
    }
}
