
use athena::{Object, XffValue};

use crate::error::{GatheringError, NyxError, NyxResult};

pub fn gather() -> NyxResult<XffValue> {
    parse_uptime(&run_uptime()?)
}

fn parse_uptime(input: &str) -> NyxResult<XffValue> {
    let input = input.trim();
    let split_comma = input.split(',').collect::<Vec<&str>>();
    if split_comma.len() < 5 {
        return Err(NyxError::Gathering(GatheringError::Uptime(format!("Invalid uptime output. Expected at least 5 columns, got {}.", split_comma.len()))));
    }
    let split_first = split_comma[0].split_whitespace().collect::<Vec<&str>>();
    if split_first.len() < 2 {
        return Err(NyxError::Gathering(GatheringError::Uptime(format!("Invalid uptime output. Expected at least 2 columns, got {}.", split_first.len()))));
    }
    let time = split_first[0];
    let up = split_first[1..].join(" ");
    let up2 = if split_comma.len() > 5 { format!("{} {}", up, split_comma[1])} else { up };

    let load_avg = {
        let mut index = 0;
        for entry in split_comma.iter() {
            if entry.trim().starts_with("load average:") {
                break;
            }
            index += 1;
        }
        if index + 2 < split_comma.len() {
            let one_min = split_comma[index].split_once(':').unwrap().1.trim().to_string();
            let five_min = split_comma[index + 1].trim().to_string();
            let fifteen_min = split_comma[index + 2].trim().to_string();
            vec![one_min, five_min, fifteen_min]
        } else {
            return Err(NyxError::Gathering(GatheringError::Uptime(format!("Invalid uptime output. Expected at least 5 columns, got {}.", split_comma.len()))));
        }
    };

    let mut out = Object::new();
    out.insert("time".to_string(), time);
    out.insert("up".to_string(), up2);
    out.insert("load_avg".to_string(), load_avg);
    Ok(out.into())
}

fn run_uptime() -> NyxResult<String> {
    let output = std::process::Command::new("uptime").env("LC_ALL", "C").output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use athena::XffValue;

    #[test]
    fn parse_simulated() {
        let path = std::env::current_dir().unwrap().join("./src/gathering/uptime/uptime.data");
        let parsed = super::parse_uptime(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert!(parsed.is_object());
        let obj = parsed.into_object().unwrap();
        assert!(obj.len() == 3);
        assert!(obj.contains_key("time"));
        assert!(obj.contains_key("up"));
        assert!(obj.contains_key("load_avg"));
        let val = obj.get("time").unwrap();
        assert!(val.is_string());
        let val = obj.get("up").unwrap();
        assert!(val.is_string());
        let val = obj.get("load_avg").unwrap();
        assert!(val.is_array());
        let val = val.into_array().unwrap();
        assert!(val.len() == 3);
        assert!(val.contains(&XffValue::String("0.00".to_string())));
    }

    #[test]
    fn parse_real() {
        let parsed = super::parse_uptime(&super::run_uptime().unwrap()).unwrap();
        assert!(parsed.is_object());
        let obj = parsed.into_object().unwrap();
        assert!(obj.len() == 3);
        assert!(obj.contains_key("time"));
        assert!(obj.contains_key("up"));
        assert!(obj.contains_key("load_avg"));
        let val = obj.get("time").unwrap();
        assert!(val.is_string());
        let val = obj.get("up").unwrap();
        assert!(val.is_string());
        let val = obj.get("load_avg").unwrap();
        assert!(val.is_array());
        let val = val.into_array().unwrap();
        assert!(val.len() == 3);
    }
}
