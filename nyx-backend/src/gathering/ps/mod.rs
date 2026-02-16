
use std::process::Stdio;

use athena::{Object, XffValue};

use crate::error::{GatheringError, NyxError, NyxResult};

pub fn gather() -> NyxResult<XffValue> {
    parse_ps(&run_ps()?)
}

fn parse_ps(input: &str) -> NyxResult<XffValue> {
    let input = input.trim();
    let lines = input.lines().collect::<Vec<&str>>();
    if lines.is_empty() {
        return Err(NyxError::Gathering(GatheringError::Free("ps output is empty".to_string())));
    }
    let headers = parse_headers(&lines[0])?;

    let mut out = Object::new();
    for proc in lines.iter().skip(1) {
        if proc.trim().is_empty() {
            continue;
        }
        let parsed_proc = parse_process(proc, &headers)?;
        #[allow(clippy::unwrap_used)] // All valid as constructed above
        let proc_pid = parsed_proc.into_object().unwrap().get("PID").unwrap().into_string().unwrap();
        out.insert(proc_pid, parsed_proc);
    }
    Ok(out.into())
}

fn parse_process(input: &str, headers: &[String; 5]) -> NyxResult<XffValue> {
    let values = input.split_whitespace().collect::<Vec<&str>>();

    println!("{:?}", values);
    if values.len() < 5 {
        return Err(NyxError::Gathering(GatheringError::Free(format!("Invalid free output. Expected at least 5 columns, got {}.", values.len()))));
    }

    let mut out = Object::new();

    out.insert(headers[0].clone(), values[0]);
    out.insert(headers[1].clone(), values[1]);
    out.insert(headers[2].clone(), values[2]);
    out.insert(headers[3].clone(), values[3]);
    if values.len() == 5 {
        out.insert(headers[4].clone(), values[4]);
    } else {
        out.insert(headers[4].clone(), values[5..].join(" "));
    }

    Ok(out.into())
}

fn parse_headers(input: &str) -> NyxResult<[String; 5]> {
    let split = input.split_whitespace().collect::<Vec<&str>>();
    if split.len() == 5 {
        Ok([split[0].to_string(), split[1].to_string(), split[2].to_string(), split[3].to_string(), split[4].to_string()])
    } else {
        Err(NyxError::Gathering(GatheringError::Free(format!("Invalid free output. Expected 5 header columns, got {}.", split.len()))))
    }
}

fn run_ps() -> NyxResult<String> {
    let ps = std::process::Command::new("ps").args(["-eo", "user,pid,%mem,%cpu,comm", "--sort=-%cpu"]).stdout(Stdio::piped()).spawn()?;
    let head = std::process::Command::new("head").args(["-n", "16"]).stdin(ps.stdout.unwrap()).stdout(Stdio::piped()).spawn()?;

    let output = head.wait_with_output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_simulated() {
        let path = std::env::current_dir().unwrap().join("./src/gathering/ps/ps.data");
        let parsed = parse_ps(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert!(parsed.is_object());
        let obj = parsed.into_object().unwrap();
        assert!(obj.len() == 15);
        let val = obj.get("1273").unwrap();
        let obj = val.into_object().unwrap();
        assert!(obj.len() == 5);
        assert_eq!(obj.get("USER").unwrap().into_string().unwrap(), "101");
        assert_eq!(obj.get("PID").unwrap().into_string().unwrap(), "1273");
        assert_eq!(obj.get("%MEM").unwrap().into_string().unwrap(), "1.6");
        assert_eq!(obj.get("%CPU").unwrap().into_string().unwrap(), "0.5");
        assert_eq!(obj.get("COMMAND").unwrap().into_string().unwrap(), "urbackupsrv");
    }

    #[test]
    fn parse_real() {
        let gatered = gather().unwrap();
        assert!(gatered.is_object());
        let obj = gatered.into_object().unwrap();
        assert!(obj.len() == 15);
    }
}

