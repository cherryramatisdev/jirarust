#[derive(thiserror::Error, Debug)]
pub enum Error {
    IoError(#[from] std::io::Error),
    SerdeError(#[from] serde_json::Error),
    // Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::SerdeError(e) => write!(f, "{}", e),
            // Error::Other(s) => write!(f, "{}", s),
        }
    }
}
