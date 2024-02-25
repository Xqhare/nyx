
use crate::utils::utils::time_now_rfc3339_with_timezone;

use super::Nyx;

use eframe::egui::{Ui, ScrollArea};

impl Nyx {

    pub fn draw_landing_page(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().vscroll(true).show(ui, |ui: &mut Ui| {
            self.grid_cpu_landing_page(ui);
            ui.separator();
            self.grid_process_landing_page(ui);
            ui.separator();
            ui.heading("Disks");
            self.grid_disks_landing_page(ui);
            /* Ref F1 
            ui.separator();
            ui.heading("GPU");
            self.grid_gpu_landing_page(ui); */
            ui.separator();
            ui.heading("RAM");
            self.grid_ram_landing_page(ui);
            ui.separator();
            ui.heading("Networks");
            self.gird_networks_landing_page(ui);
            ui.separator();
            ui.heading("Temperatures");
            self.gird_temperature_landing_page(ui);
        });
    }
    
    pub fn draw_main_menu(&mut self, ui: &mut Ui) {
        
        ui.horizontal(|ui: &mut Ui| {
            ui.menu_button("Nyx", |ui: &mut Ui| {
                if ui.button("Settings").clicked() {
                    self.clear_screen();
                    self.show_settings_page = true;
                }
                if ui.button("Help").clicked() {
                    println!("Help click");
                    self.clear_screen();
                    self.show_help = true;
                }
                if ui.button("About").clicked() {
                    println!("About click");
                    self.clear_screen();
                    self.show_about_page = true;
                }
            });
            if ui.button("Cpu").clicked() {
                self.clear_screen();
                self.show_cpu_page = true;
            }
            if ui.button("PROCESSES").clicked() {
                self.clear_screen();
                self.show_process_page = true;
            }
            /* Ref F1
            if ui.button("GPU").clicked() {
                println!("GPU");
                self.clear_screen();
                self.show_gpu_page = true;
            } */
            if ui.button("RAM").clicked() {
                println!("RAM");
                self.clear_screen();
                self.show_ram_page = true;
            }
            if ui.button("DISC").clicked() {
                println!("DISC");
                self.clear_screen();
                self.show_disk_page = true;
            }
            if ui.button("NETWORKS").clicked() {
                println!("NETWORKS");
                self.clear_screen();
                self.show_network_page = true;
            }
            if ui.button("TEMPERATURE").clicked() {
                println!("TEMPERATURE");
                self.clear_screen();
                self.show_temperature_page = true;
            }
            if ui.button("Eris").clicked() {
                println!("Eris click");
                self.clear_screen();
                self.show_eris_page = true;
            }
            if !self.show_landing_page {
                if ui.button("Back to main page").clicked() {
                    self.reset_to_landing_page();
                }
            }
            ui.spacing();
            ui.separator();
            ui.spacing();
            if self.settings.display_time_ribbon {
                ui.label(format!("{}", time_now_rfc3339_with_timezone(chrono::SecondsFormat::Secs, self.settings.timezone)));
            }

            
        });
    }
    
    pub fn clear_screen(&mut self) {
        self.show_landing_page = false;
        self.show_help = false;
        self.show_cpu_page = false;
        self.show_ram_page = false;
        self.show_gpu_page = false;
        self.show_disk_page = false;
        self.show_network_page = false;
        self.show_temperature_page = false;
        self.show_settings_page = false;
        self.show_about_page = false;
        self.show_eris_page = false;
        self.show_advanced_settings_page = false;
        self.show_process_page = false;
    }
    
    fn reset_to_landing_page(&mut self) {
        self.clear_screen();
        self.show_landing_page = true;
    }

}

