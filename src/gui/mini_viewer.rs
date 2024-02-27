
use eframe::egui::{Ui, ScrollArea, Grid};

use super::Nyx;

impl Nyx {
    pub fn mini_viewer_page(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().vscroll(true).show(ui, |ui: &mut Ui| {
            Grid::new("mini view").striped(false).num_columns(1).show(ui, |ui: &mut Ui| {
                ui.label("CPU");
                ui.end_row();
                self.mini_cpu(ui);
                ui.end_row();
                ui.label("RAM");
                ui.end_row();
                self.draw_ram_usage(ui, "ram");
                ui.end_row();
                let network_data = self.networks.networks.lock();
                let disk_data = self.disks.disks.lock();
                let temperature_data = self.temperatures.components.lock();
                if network_data.is_ok() && disk_data.is_ok() && temperature_data.is_ok() {
                    let ok_network_data = network_data.unwrap().clone();
                    let ok_disk_data = disk_data.unwrap().clone();
                    let ok_temperature_data = temperature_data.unwrap().clone();
                    for network in ok_network_data.iter() {
                        ui.label(format!("Network {} | Packets In and Out", network.name));
                        ui.end_row();
                        self.draw_network_usage(ui, network.clone(), "packets");
                        ui.end_row();
                    }
                    for disk in ok_disk_data.iter() {
                        ui.label(format!("Disk {}", disk.name));
                        ui.end_row();
                        self.draw_disk_usage(ui, disk.clone());
                        ui.end_row();                    
                    }
                    for component in ok_temperature_data.iter() {
                        let mut tmp: String = Default::default();
                        for sensor in component.iter() {
                            if sensor.name.to_string() != tmp {
                                ui.label(format!("{} {}", sensor.name, sensor.sensor));
                                ui.end_row();
                                self.draw_temperature_usage(ui, sensor.clone());
                                ui.end_row();
                                tmp = sensor.name.clone().to_string();
                            }
                            
                        }
                    }
                }
            });
        });
    }
}
