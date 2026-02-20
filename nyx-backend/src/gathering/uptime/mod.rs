use athena::{Object, XffValue};

use crate::error::{GatheringError, NyxError, NyxResult};

pub fn gather() -> NyxResult<XffValue> {
    parse_uptime(&run_uptime()?)
}

fn parse_uptime(input: &str) -> NyxResult<XffValue> {
    let input = input.trim();
    let split_comma = input.split(',').collect::<Vec<&str>>();
    if split_comma.len() < 2 {
        return Err(NyxError::Gathering(GatheringError::Uptime(format!(
            "Invalid uptime output. Expected at least 2 comma-separated parts, got {}. Input: {}",
            split_comma.len(),
            input
        ))));
    }

    // First part contains current time and start of uptime
    let first_part = split_comma[0].split_whitespace().collect::<Vec<&str>>();
    if first_part.len() < 2 {
        return Err(NyxError::Gathering(GatheringError::Uptime(format!(
            "Invalid uptime output. First part too short: {:?}",
            first_part
        ))));
    }
    let time = first_part[0];

    // Uptime can span multiple comma-separated parts (e.g., "up 6 days, 10:45")
    // We'll collect everything until we hit the part with "user(s)"
    let mut up_parts = vec![first_part[1..].join(" ")];
    let mut user_index = 1;
    while user_index < split_comma.len() && !split_comma[user_index].contains("user") {
        up_parts.push(split_comma[user_index].trim().to_string());
        user_index += 1;
    }
    let up = up_parts.join(", ");

    let load_avg = {
        let mut index = 0;
        for entry in split_comma.iter() {
            if entry.trim().starts_with("load average:") {
                break;
            }
            index += 1;
        }
        if index + 2 < split_comma.len() {
            let one_min = split_comma[index]
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .to_string();
            let five_min = split_comma[index + 1].trim().to_string();
            let fifteen_min = split_comma[index + 2].trim().to_string();
            vec![one_min, five_min, fifteen_min]
        } else {
            // Fallback if load average is not where we expect
            vec!["0.00".to_string(), "0.00".to_string(), "0.00".to_string()]
        }
    };

    let mut out = Object::new();
    out.insert("time".to_string(), time);
    out.insert("up".to_string(), up);
    out.insert("load_avg".to_string(), load_avg);
    Ok(out.into())
}

fn run_uptime() -> NyxResult<String> {
    let output = std::process::Command::new("uptime")
        .env("LC_ALL", "C")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use athena::XffValue;

    #[test]
    fn parse_simulated() {
        let path = std::env::current_dir()
            .unwrap()
            .join("./src/gathering/uptime/uptime.data");
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
