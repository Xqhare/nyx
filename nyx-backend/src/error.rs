
pub type NyxResult<T> = std::result::Result<T, NyxError>;

#[derive(Debug)]
pub enum NyxError {
    Gathering(GatheringError),
    StdIO(std::io::Error),
    Generic(String), // Useful for debugging
}

#[derive(Debug)]
pub enum GatheringError {
    Df(String),
    Docker(String),
    
}

impl From<std::io::Error> for NyxError {
    fn from(e: std::io::Error) -> Self {
        NyxError::StdIO(e)
    }
}
