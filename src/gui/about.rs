
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
            ui.spacing();
            ui.separator();
            ui.spacing();
            ui.heading("Dependencies and Licenses");
            ui.spacing();
            ui.spacing();
            ui.heading("chrono");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/chrono");
            ui.hyperlink_to("License", "https://github.com/chronotype/chrono/blob/main/LICENSE.txt");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("chrono-tz");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/chrono-tz");
            ui.hyperlink_to("License", "https://github.com/chronotype/chrono-tz/blob/main/LICENSE");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("eframe");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/eframe");
            ui.hyperlink_to("License", "https://github.com/emilk/egui/blob/master/LICENSE-MIT");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("sysinfo");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/sysinfo");
            ui.hyperlink_to("License", "https://github.com/GuillaumeGomez/sysinfo/blob/master/LICENSE");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("dirs");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/dirs");
            ui.hyperlink_to("License", "https://github.com/dirs-dev/dirs-rs/blob/main/LICENSE-MIT");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("mexprp");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/mexprp");
            ui.hyperlink_to("License", "https://github.com/InterpidPig/mexprp/blob/master/LICENSE");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("procfs");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/procfs");
            ui.hyperlink_to("License", "https://github.com/eminence/procfs/blob/master/LICENSE-MIT");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("rand");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/rand");
            ui.hyperlink_to("License", "https://github.com/rust-random/rand/blob/master/LICENSE-MIT");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("image");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/image");
            ui.hyperlink_to("License", "https://github.com/image-rs/image/blob/master/LICENSE-MIT");
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.spacing();
            ui.heading("json");
            ui.hyperlink_to("Crates.io", "https://crates.io/crates/json");
            ui.hyperlink_to("License", "https://github.com/maciejhirsz/json-rust/blob/master/LICENSE-MIT");
        });
        // Grid doesn't work because egui hasn't implemented centering grids
        // jet. Ref: https://github.com/emilk/egui/issues/2247
    }
}
