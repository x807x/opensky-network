/// This error type wraps other crate's errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Server request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Server returned HTTP error code: {0}")]
    Http(reqwest::StatusCode),

    #[error("Unable to parse response as string: {0}")]
    InvalidString(#[from] std::string::FromUtf8Error),

    #[error("Unable to parse response as Json: {0}")]
    InvalidJson(#[from] serde_json::error::Error),

    #[error("Unable to parse response as Json: {0}")]
    InvalidCsv(#[from] csv::Error),

    #[error("Unable to write file: {0}")]
    IO(#[from] std::io::Error),

    #[error("Unable to parse system time: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
}
