use athena::{Object, XffValue};

use crate::error::{GatheringError, NyxError, NyxResult};

pub fn gather() -> NyxResult<XffValue> {
    parse_free(&run_free()?)
}

fn parse_free(input: &str) -> NyxResult<XffValue> {
    let input = input.trim();
    let lines = input.lines().collect::<Vec<&str>>();
    if lines.len() != 3 {
        return Err(NyxError::Gathering(GatheringError::Free(format!(
            "Invalid free output. Expected 3 lines, got {}.",
            lines.len()
        ))));
    }
    let headers = parse_headers(lines[0])?;
    let mem = parse_mem(lines[1], &headers)?;
    let swap = parse_swap(lines[2], &headers)?;

    let mut out = Object::new();
    out.insert("mem".to_string(), mem);
    out.insert("swap".to_string(), swap);

    Ok(out.into())
}

fn parse_swap(input: &str, headers: &[String; 6]) -> NyxResult<XffValue> {
    let values = input.split_whitespace().collect::<Vec<&str>>();

    if values.len() != 4 {
        return Err(NyxError::Gathering(GatheringError::Free(format!(
            "Invalid free output. Expected 4 Swap columns, got {}.",
            values.len()
        ))));
    }

    if values[0] != "Swap:" {
        return Err(NyxError::Gathering(GatheringError::Free(format!(
            "Invalid free output. Expected Swap:, got {}.",
            values[0]
        ))));
    }

    let mut out = Object::new();

    out.insert(headers[0].clone().trim(), values[1].trim());
    out.insert(headers[1].clone().trim(), values[2].trim());
    out.insert(headers[2].clone().trim(), values[3].trim());

    Ok(out.into())
}

fn parse_mem(input: &str, headers: &[String; 6]) -> NyxResult<XffValue> {
    let values = input.split_whitespace().collect::<Vec<&str>>();

    if values.len() != 7 {
        return Err(NyxError::Gathering(GatheringError::Free(format!(
            "Invalid free output. Expected 7 Mem columns, got {}.",
            values.len()
        ))));
    }

    if values[0] != "Mem:" {
        return Err(NyxError::Gathering(GatheringError::Free(format!(
            "Invalid free output. Expected Mem:, got {}.",
            values[0]
        ))));
    }

    let mut out = Object::new();

    out.insert(headers[0].clone().trim(), values[1].trim());
    out.insert(headers[1].clone().trim(), values[2].trim());
    out.insert(headers[2].clone().trim(), values[3].trim());
    out.insert(headers[3].clone().trim(), values[4].trim());
    out.insert(headers[4].clone().trim(), values[5].trim());
    out.insert(headers[5].clone().trim(), values[6].trim());

    Ok(out.into())
}

fn parse_headers(input: &str) -> NyxResult<[String; 6]> {
    let split = input.split_whitespace().collect::<Vec<&str>>();
    if split.len() == 6 {
        Ok([
            split[0].to_string(),
            split[1].to_string(),
            split[2].to_string(),
            split[3].to_string(),
            split[4].to_string(),
            split[5].to_string(),
        ])
    } else {
        Err(NyxError::Gathering(GatheringError::Free(format!(
            "Invalid free output. Expected 6 header columns, got {}.",
            split.len()
        ))))
    }
}

fn run_free() -> NyxResult<String> {
    let output = std::process::Command::new("free").arg("-h").output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_simulated() {
        let path = std::env::current_dir()
            .unwrap()
            .join("./src/gathering/free/free.data");
        let parsed = super::parse_free(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert!(parsed.is_object());
        let obj = parsed.into_object().unwrap();
        assert!(obj.len() == 2);
        assert!(obj.contains_key("mem"));
        let val = obj.get("mem").unwrap();
        assert!(val.is_object());
        let obj = val.into_object().unwrap();
        assert!(obj.len() == 6);
        assert_eq!(obj.get("total").unwrap().into_string().unwrap(), "31Gi");
        assert_eq!(obj.get("used").unwrap().into_string().unwrap(), "2.1Gi");
        assert_eq!(obj.get("free").unwrap().into_string().unwrap(), "834Mi");
        assert_eq!(obj.get("shared").unwrap().into_string().unwrap(), "1.5Mi");
        assert_eq!(
            obj.get("buff/cache").unwrap().into_string().unwrap(),
            "29Gi"
        );
        assert_eq!(obj.get("available").unwrap().into_string().unwrap(), "29Gi");
    }

    #[test]
    fn parse_real() {
        let gatered = super::gather().unwrap();
        assert!(gatered.is_object());
        let obj = gatered.into_object().unwrap();
        assert!(obj.len() == 2);
    }
}
