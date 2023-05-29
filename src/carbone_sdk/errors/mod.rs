use thiserror::Error;

#[derive(Error, Debug)]
pub enum CarboneSdkError {
    #[error("Carbone SDK {0:?} error: {1:?}")]
    Error(String, String),
    #[error("CarboneSDK: \"API access token\" is missing")]
    MissingApiToken(String),
    #[error("Carbone SDK {0:?} error: argument is missing: {1:?}")]
    MissingArgument(String, String),
    #[error("Carbone SDK {0:?} error: file {1:?} not found")]
    FileNotFound(String, String),
    #[error("Carbone SDK {0:?} {1:?} is a directory")]
    IsADirectory(String, String),
    #[error("Carbone SDK IoError {0:?}")]
    IoError(std::io::Error),
    #[error("Carbone SDK RequestError {0:?}")]
    RequestError(reqwest::Error),
    #[error("Carbone SDK ResponseError {0:?}")]
    ResponseError(String),
    #[error("Carbone SDK RequestBodyNotWellFormedJsonError")]
    RequestBodyNotWellFormedJsonError,
    #[error("Carbone SDK {0:?} ParseError {1:?}")]
    ParseError(String, String)
}

impl From<std::io::Error> for CarboneSdkError {
    fn from(err: std::io::Error) -> Self {
        CarboneSdkError::IoError(err)
    }
}

impl From<reqwest::Error> for CarboneSdkError {
    fn from(err: reqwest::Error) -> Self {
        CarboneSdkError::RequestError(err)
    }
}