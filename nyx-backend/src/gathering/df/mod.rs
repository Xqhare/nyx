use athena::{Object, XffValue};

use crate::error::{GatheringError, NyxError, NyxResult};

/// Runs the data gathering process for `df`
///
/// It runs `df -h` and then parses the output
///
/// Returned are all devices with their corresponding size, used, available, used % and mount point
pub fn gather() -> NyxResult<XffValue> {
    parse_df(&run_df()?)
}

fn parse_df(input: &str) -> NyxResult<XffValue> {
    let mut out = Object::new();
    for (index, line) in input.lines().enumerate() {
        if index == 0 {
            // Sanity checking
            let headers = line.split_whitespace().collect::<Vec<&str>>();
            parse_head(&headers)?;
        } else {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            if split.is_empty() {
                continue;
            }
            if split.len() < 6 {
                return Err(NyxError::Gathering(GatheringError::Df(format!(
                    "Invalid df output. Expected at least 6 columns, got {}. Line: {}",
                    split.len(),
                    line
                ))));
            }
            // Ignore all non devices
            if !split[0].starts_with("/dev") {
                continue;
            }
            let mut tmp = Object::new();
            tmp.insert("Filesystem", split[0]);
            tmp.insert("Size", split[1]);
            tmp.insert("Used", split[2]);
            tmp.insert("Avail", split[3]);
            tmp.insert("Use%", split[4]);
            tmp.insert("Mounted on", split[5..].join(" "));
            // Lets hope the filesystem name is unique
            out.insert(split[0], tmp);
        }
    }
    Ok(out.into())
}

fn parse_head(headers: &[&str]) -> NyxResult<()> {
    if headers.len() != 7 {
        return Err(NyxError::Gathering(GatheringError::Df(format!(
            "Invalid df output. Expected Failed to gather7 headers, got {}.",
            headers.len()
        ))));
    }
    for (n, header) in headers.iter().enumerate() {
        match n {
            0 => {
                if *header != "Filesystem" {
                    return Err(NyxError::Gathering(GatheringError::Df(format!(
                        "Expected 'Filesystem' header, got: {}",
                        header
                    ))));
                }
            }
            1 => {
                if *header != "Size" {
                    return Err(NyxError::Gathering(GatheringError::Df(format!(
                        "Expected 'Size' header, got: {}",
                        header
                    ))));
                }
            }
            2 => {
                if *header != "Used" {
                    return Err(NyxError::Gathering(GatheringError::Df(format!(
                        "Expected 'Used' header, got: {}",
                        header
                    ))));
                }
            }
            3 => {
                if *header != "Avail" {
                    return Err(NyxError::Gathering(GatheringError::Df(format!(
                        "Expected 'Available' header, got: {}",
                        header
                    ))));
                }
            }
            4 => {
                if *header != "Use%" {
                    return Err(NyxError::Gathering(GatheringError::Df(format!(
                        "Expected 'Use%' header, got: {}",
                        header
                    ))));
                }
            }
            5 => {
                if *header != "Mounted" {
                    return Err(NyxError::Gathering(GatheringError::Df(format!(
                        "Expected 'Mounted on' header, got: {}",
                        header
                    ))));
                }
            }
            6 => {
                if *header != "on" {
                    return Err(NyxError::Gathering(GatheringError::Df(format!(
                        "Expected 'Mounted on' header, got: {}",
                        header
                    ))));
                }
            }
            _ => {
                return Err(NyxError::Gathering(GatheringError::Df(format!(
                    "Unexpected header: {}",
                    header
                ))));
            }
        }
    }

    Ok(())
}

fn run_df() -> NyxResult<String> {
    let output = std::process::Command::new("df").arg("-h").output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_simulated() -> NyxResult<()> {
        let path = std::env::current_dir()?.join("./src/gathering/df/df.data");
        let parsed = parse_df(&std::fs::read_to_string(path)?)?;
        assert!(parsed.is_object());
        let obj = parsed.into_object().unwrap();
        assert!(obj.len() == 2);
        assert!(obj.contains_key("/dev/sda1"));
        let val = obj.get("/dev/sda1").unwrap();
        assert!(val.is_object());
        let obj = val.into_object().unwrap();
        assert!(obj.len() == 6);
        assert_eq!(
            obj.get("Filesystem").unwrap().into_string().unwrap(),
            "/dev/sda1"
        );
        assert_eq!(obj.get("Size").unwrap().into_string().unwrap(), "1.8T");
        assert_eq!(obj.get("Used").unwrap().into_string().unwrap(), "361G");
        assert_eq!(obj.get("Avail").unwrap().into_string().unwrap(), "1.4T");
        assert_eq!(obj.get("Use%").unwrap().into_string().unwrap(), "21%");
        assert_eq!(
            obj.get("Mounted on").unwrap().into_string().unwrap(),
            "/home/master/backup-usb-disk"
        );
        Ok(())
    }

    #[test]
    fn parse_real() -> NyxResult<()> {
        let gatered = gather()?;
        assert!(gatered.is_object());
        let obj = gatered.into_object().unwrap();
        assert!(!obj.is_empty());
        Ok(())
    }
}
