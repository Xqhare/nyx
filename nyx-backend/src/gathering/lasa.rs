use std::path::PathBuf;

use areia::BaseDirs;
use athena::XffValue;
use nabu::serde::read;

use crate::error::{GatheringError, NyxError, NyxResult};

pub fn gather() -> NyxResult<XffValue> {
    let path = make_lasa_path()?;
    if path.exists() {
        if let Ok(read) = read(path) {
            return Ok(read);
        }
    }
    Err(NyxError::Gathering(GatheringError::Lasa(
        "File does not exist".to_string(),
    )))
}

fn make_lasa_path() -> NyxResult<PathBuf> {
    let home = match BaseDirs::new() {
        Ok(dirs) => dirs.home_dir().clone(),
        Err(err) => return Err(NyxError::Gathering(GatheringError::Lasa(err.to_string()))),
    };
    Ok(home.join("lasa_system_uptime.xff"))
}
