use std::{path::PathBuf, io::{Error, Read, self}, fs::File};

use chrono_tz::Tz;
use eframe::{epaint::{Color32, Vec2}, Theme};
use json::{JsonValue, parse};


// Ref UIb2
#[derive(Debug)]
pub struct Settings {
    pub main_colour: Color32,
    pub cpu_colour: Color32,
    pub ram_colour: Color32,
    pub network_colour: Color32,
    pub network_error_colour: Color32,
    pub disk_write_colour: Color32,
    pub disk_read_colour: Color32,
    pub temperature_colour: Color32,
    pub process_data_colour: Color32,
    pub timezone: Tz,
    pub dark_theme: Theme,
    pub data_update_interval: i64,
    pub set_interval: String,
    pub display_size: Vec2,
    pub set_size_x: String,
    pub set_size_y: String,
    pub display_time_ribbon: bool,
    pub save_location: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        let config_path = dirs::config_dir().unwrap();
        let setting_path = config_path.join("nyxconfig.json");
        // No touchy the numbers! I SAID NO TOUCHY!!!
        Settings { 
            main_colour: Color32::GOLD, 
            cpu_colour: Color32::GOLD,
            ram_colour: Color32::GOLD,
            network_colour: Color32::GOLD,
            network_error_colour: Color32::RED,
            disk_write_colour: Color32::GOLD,
            disk_read_colour: Color32::GREEN,
            temperature_colour: Color32::GOLD,
            process_data_colour: Color32::GOLD,
            timezone: chrono_tz::GMT,
            dark_theme: Theme::Dark, 
            data_update_interval: 1000,
            display_size: Vec2 { x: 1200.0, y: 1000.0 },
            set_size_x: format!("1200.0"),
            set_size_y: format!("1000.0"),
            set_interval: format!("1000"),
            display_time_ribbon: true,
            save_location: setting_path,
        }
    }
}

impl Settings {
    pub fn new(main_colour: Color32, cpu_colour: Color32, ram_colour: Color32, network_colour: Color32, network_error_colour: Color32, disk_write_colour: Color32, disk_read_colour: Color32, temperature_colour: Color32, process_data_colour: Color32, timezone: Tz, dark_theme: Theme, data_update_interval: i64, display_size: Vec2, display_time_ribbon: bool, save_location: PathBuf) -> Self {
        Settings { main_colour, cpu_colour, ram_colour, network_colour, network_error_colour, disk_write_colour, temperature_colour, disk_read_colour, process_data_colour, timezone, dark_theme, data_update_interval, display_size, display_time_ribbon, set_size_x: format!("{}", display_size.x), set_size_y: format!("{}", display_size.y), set_interval: format!("{}", data_update_interval), save_location, }
    }

    pub fn load(path: PathBuf) -> Result<Self, Error> {
        let mut input = File::open(path.clone())?;
        let mut buffer: String = Default::default();
        let _ = input.read_to_string(&mut buffer);
        let json_val: JsonValue = parse(&buffer).expect("UNABLE TO PARSE JSON!");
        let mut main_colour: Color32 = Default::default();
        let mut cpu_colour: Color32 = Default::default();
        let mut ram_colour: Color32 = Default::default();
        let mut network_colour: Color32 = Default::default();
        let mut network_error_colour: Color32 = Default::default();
        let mut disk_write_colour: Color32 = Default::default();
        let mut disk_read_colour: Color32 = Default::default();
        let mut temperature_colour: Color32 = Default::default();
        let mut process_data_colour: Color32 = Default::default();

        let mut timezone: Tz = Default::default();
        let mut dark_theme: Theme = Theme::Dark;
        let mut data_update_interval: i64 = Default::default();
        let mut display_size: Vec2 = Default::default();
        let mut display_time_ribbon: bool = Default::default();
        for t in json_val.entries() {
            match t.0 {
                "main_colour" => main_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "cpu_colour" => cpu_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "ram_colour" => ram_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "network_colour" => network_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "network_error_colour" => network_error_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "disk_write_colour" => disk_write_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "disk_read_colour" => disk_read_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "temperature_colour" => temperature_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),
                "process_data_colour" => process_data_colour = Color32::from_rgba_premultiplied(t.1.clone().array_remove(0).as_u8().unwrap(), t.1.clone().array_remove(1).as_u8().unwrap(), t.1.clone().array_remove(2).as_u8().unwrap(), t.1.clone().array_remove(3).as_u8().unwrap()),

                "timezone" => {
                    let tz: Tz = t.1.clone().take_string().unwrap().parse().unwrap();
                    timezone = tz;
                },
                "dark_theme" => match t.1.clone().take_string().unwrap().as_str() {
                    "Light" => dark_theme = Theme::Light,
                    _ => dark_theme = Theme::Dark,
                },
                "data_update_interval" => data_update_interval = t.1.as_i64().unwrap(),
                "display_size" => display_size = Vec2 { x: t.1.clone().array_remove(0).as_f32().unwrap(), y: t.1.clone().array_remove(1).as_f32().unwrap()},
                "display_time_ribbon" => display_time_ribbon = t.1.as_bool().unwrap(),
                _ => println!("heyo!"),
            }
        }
        let out = Settings::new(main_colour, cpu_colour, ram_colour, network_colour, network_error_colour, disk_write_colour, disk_read_colour, temperature_colour, process_data_colour, timezone, dark_theme, data_update_interval, display_size, display_time_ribbon, path);
        return Ok(out);
    }

    pub fn save(&self, path: PathBuf) -> Result<(), Error> {
        let mut final_json = JsonValue::new_object();

        let mut tmcb = JsonValue::new_array();
        let _ = tmcb.push(self.main_colour.r());
        let _ = tmcb.push(self.main_colour.g());
        let _ = tmcb.push(self.main_colour.b());
        let _ = tmcb.push(self.main_colour.a());
        let mc = final_json.insert("main_colour", tmcb);

        let mut ccb = JsonValue::new_array();
        let _ = ccb.push(self.cpu_colour.r());
        let _ = ccb.push(self.cpu_colour.g());
        let _ = ccb.push(self.cpu_colour.b());
        let _ = ccb.push(self.cpu_colour.a());
        let cc = final_json.insert("cpu_colour", ccb);

        let mut rcb = JsonValue::new_array();
        let _ = rcb.push(self.ram_colour.r());
        let _ = rcb.push(self.ram_colour.g());
        let _ = rcb.push(self.ram_colour.b());
        let _ = rcb.push(self.ram_colour.a());
        let rc = final_json.insert("ram_colour", rcb);

        let mut ncb = JsonValue::new_array();
        let _ = ncb.push(self.network_colour.r());
        let _ = ncb.push(self.network_colour.g());
        let _ = ncb.push(self.network_colour.b());
        let _ = ncb.push(self.network_colour.a());
        let nc = final_json.insert("network_colour", ncb);

        let mut necb = JsonValue::new_array();
        let _ = necb.push(self.network_error_colour.r());
        let _ = necb.push(self.network_error_colour.g());
        let _ = necb.push(self.network_error_colour.b());
        let _ = necb.push(self.network_error_colour.a());
        let nec = final_json.insert("network_error_colour", necb);

        let mut dwcb = JsonValue::new_array();
        let _ = dwcb.push(self.disk_write_colour.r());
        let _ = dwcb.push(self.disk_write_colour.g());
        let _ = dwcb.push(self.disk_write_colour.b());
        let _ = dwcb.push(self.disk_write_colour.a());
        let dwc = final_json.insert("disk_write_colour", dwcb);

        let mut drcb = JsonValue::new_array();
        let _ = drcb.push(self.disk_read_colour.r());
        let _ = drcb.push(self.disk_read_colour.g());
        let _ = drcb.push(self.disk_read_colour.b());
        let _ = drcb.push(self.disk_read_colour.a());
        let drc = final_json.insert("disk_read_colour", drcb);

        let mut tcb = JsonValue::new_array();
        let _ = tcb.push(self.temperature_colour.r());
        let _ = tcb.push(self.temperature_colour.g());
        let _ = tcb.push(self.temperature_colour.b());
        let _ = tcb.push(self.temperature_colour.a());
        let tc = final_json.insert("temperature_colour", tcb);

        let mut pdb = JsonValue::new_array();
        let _ = pdb.push(self.temperature_colour.r());
        let _ = pdb.push(self.temperature_colour.g());
        let _ = pdb.push(self.temperature_colour.b());
        let _ = pdb.push(self.temperature_colour.a());
        let pb = final_json.insert("process_data_colour", pdb);
        
        if mc.is_ok() && cc.is_ok() && rc.is_ok() && nc.is_ok() && nec.is_ok() && dwc.is_ok() && drc.is_ok() && tc.is_ok() && pb.is_ok() {
            let tz = final_json.insert("timezone", format!("{}", self.timezone));
            let dt = final_json.insert("dark_theme", format!("{:?}", self.dark_theme));
            let dui = final_json.insert("data_update_interval", self.data_update_interval);

            let mut dsb = JsonValue::new_array();
            let _ = dsb.push(self.display_size.x);
            let _ = dsb.push(self.display_size.y);
            let ds = final_json.insert("display_size", dsb);

            let dtr = final_json.insert("display_time_ribbon", self.display_time_ribbon);
            if tz.is_ok() && dt.is_ok() && dui.is_ok() && ds.is_ok() && dtr.is_ok() {
                let mut file = File::create(path)?;
                let fin_write = final_json.write_pretty(&mut file, 2);
                return fin_write;
            } else {
                return Err(Error::from(io::ErrorKind::InvalidData));
            }
        } else {
            return Err(Error::from(io::ErrorKind::InvalidData));
        }

    }
}
