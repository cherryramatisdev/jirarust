#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ERROR: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("ERROR: `{0}`")]
    SerdeError(#[from] serde_json::Error),
    #[error("ERROR: `{0}`")]
    MinreqError(#[from] minreq::Error),
    #[error("ERROR: `{0}`")]
    Other(String)
}
