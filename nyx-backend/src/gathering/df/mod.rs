
mod reader;
mod parser;

/// Runs the data gathering process for `df`
///
/// It runs `df -h` and then parses the output
///
/// Returned are all devices with their corresponding size, used, available, used % and mount point
pub fn gather() {
    todo!();
}
