
use crate::{APPNAME, APPVERSION, APPAUTHORS};

use eframe::{egui::{Ui, style::Spacing}, epaint::{Rect, Pos2}};

use super::Nyx;

impl Nyx {
    pub fn help_page(&mut self, ui: &mut Ui) {
        // God please forgive me for this mess, but you made me fight with the layout again!
        let _ = ui.put(Rect::from_two_pos(
            Pos2 { x: self.settings.display_size.x / 2.0 - 45.0, 
                y: Spacing::default().menu_margin.top * 6.0 }, 
            Pos2 { x: self.settings.display_size.x / 2.0 + 45.0, 
                y: Spacing::default().menu_margin.bottom * 7.0}),
            |ui: &mut Ui|{
                ui.horizontal(|ui: &mut Ui| {
                    ui.heading(APPNAME);
                    ui.label(" v. ");
                    ui.label(APPVERSION);
                }).response
            });
        ui.vertical_centered(|ui: &mut Ui| {
            ui.label(" by ");
            ui.label(APPAUTHORS);
            ui.hyperlink_to("Contribute on github!", "https://github.com/Xqhare/nyx");
        });
        ui.separator();
        ui.heading("Using Nyx");
        ui.label("Nyx is split into one main page, also called the landing page, as well as subpages for the different components, temperature, the minimal view, settings, about page and Eris.");
        ui.label("You can get to these subpages by clicking on the corresponding chart on the landing page, or by using the main menu at the top-left of the window.");

        ui.separator();

        ui.heading("Settings");
        ui.label("The settings can be reached by clicking the 'Nyx' menu button in the top-left corner of the window. Here you can change the size the window is supposed to launch with, set you Timezone, change the update interval (not recommended to go lower than 1000ms), as well as the colours of the charts.");
        ui.label("Don't forget to save your changes before leaving the page! You can always reset or delete the settings you have and return to the default.");

        ui.separator();
        
        ui.heading("Components");
        ui.label("Under the second menu, 'Components', you find a list of all the different components Nyx provides monitoring for. You can either click these buttons or the corresponding chart on the main page");
    }

}
