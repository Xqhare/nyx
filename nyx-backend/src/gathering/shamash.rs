use std::path::PathBuf;

use areia::BaseDirs;
use athena::XffValue;

use crate::error::{GatheringError, NyxError, NyxResult};

pub fn gather() -> NyxResult<XffValue> {
    let base_path = make_shamash_path()?;
    if make_diagnosing_path(&base_path).exists() {
        return Ok(XffValue::from("Diagnosing"));
    }
    if make_complete_network_outage_path(&base_path).exists() {
        return Ok(XffValue::from("Ongoing Complete Network Outage"));
    }
    if make_isp_outage_path(&base_path).exists() {
        return Ok(XffValue::from("Ongoing ISP Outage"));
    }
    if make_local_outage_path(&base_path).exists() {
        return Ok(XffValue::from("Ongoing Local Outage"));
    }
    Ok(XffValue::from("Online"))
}

fn make_shamash_path() -> NyxResult<PathBuf> {
    let home = match BaseDirs::new() {
        Ok(dirs) => dirs.home_dir().clone(),
        Err(err) => {
            return Err(NyxError::Gathering(GatheringError::Shamash(err.to_string())));
        }
    };
    let base_path = home.join("docker/shamash/shamash-logs/");
    if !base_path.exists() {
        return Err(NyxError::Gathering(GatheringError::Shamash(
            "Shamash path does not exist".to_string(),
        )));
    }
    Ok(base_path)
}

fn make_complete_network_outage_path(base_path: &PathBuf) -> PathBuf {
    base_path.join("complete_network_outage_ongoing")
}

fn make_diagnosing_path(base_path: &PathBuf) -> PathBuf {
    base_path.join("diagnosing")
}

fn make_isp_outage_path(base_path: &PathBuf) -> PathBuf {
    base_path.join("isp_outage_ongoing")
}

fn make_local_outage_path(base_path: &PathBuf) -> PathBuf {
    base_path.join("local_outage_ongoing")
}
