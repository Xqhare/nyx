use athena::{Object, XffValue};

use crate::error::{GatheringError, NyxError, NyxResult};

pub fn gather() -> NyxResult<XffValue> {
    parse_docker_ps(&run_docker_ps()?)
}

fn parse_docker_ps(input: &str) -> NyxResult<XffValue> {
    let mut out = Object::new();

    let lines = input.lines().collect::<Vec<&str>>();
    let headers = make_headers();

    for line in lines.iter() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        if split.is_empty() {
            continue;
        }
        if split.len() != 4 {
            return Err(NyxError::Gathering(GatheringError::Docker(format!(
                "Invalid docker ps output. Expected 4 columns, got {}.",
                split.len()
            ))));
        }
        let mut tmp = Object::new();
        tmp.insert(headers[0].clone(), split[0]);
        tmp.insert(headers[1].clone(), split[1]);
        tmp.insert(headers[2].clone(), split[2]);
        tmp.insert(headers[3].clone(), split[3]);
        out.insert(split[1], tmp);
    }
    Ok(out.into())
}

fn make_headers() -> [String; 4] {
    [
        "ID".to_string(),
        "Names".to_string(),
        "State".to_string(),
        "Image".to_string(),
    ]
}

// For future reference, just run this command
// docker ps --format "table {{.ID}}\t{{.Names}}\t{{.State}}\t{{.Image}}"
fn run_docker_ps() -> NyxResult<String> {
    let output = std::process::Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("{{.ID}}\t{{.Names}}\t{{.State}}\t{{.Image}}")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_simulated() {
        let path = std::env::current_dir()
            .unwrap()
            .join("./src/gathering/docker/docker_ps_output.data");
        let parsed = super::parse_docker_ps(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert!(parsed.is_object());
        let obj = parsed.into_object().unwrap();
        assert!(obj.len() == 5);
        assert!(obj.contains_key("Shamash"));
        let val = obj.get("Shamash").unwrap();
        assert!(val.is_object());
        let obj = val.into_object().unwrap();
        assert!(obj.len() == 4);
        assert_eq!(
            obj.get("ID").unwrap().into_string().unwrap(),
            "bf240f90dcb3"
        );
        assert_eq!(obj.get("Names").unwrap().into_string().unwrap(), "Shamash");
        assert_eq!(obj.get("State").unwrap().into_string().unwrap(), "running");
        assert_eq!(
            obj.get("Image").unwrap().into_string().unwrap(),
            "shamash-shamash"
        );
    }
}
