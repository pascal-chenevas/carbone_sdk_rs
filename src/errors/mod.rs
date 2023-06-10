use thiserror::Error;
use validator::ValidationErrors;
#[derive(Error, Debug)]
pub enum CarboneSdkError {
    #[error("Carbone SDK error: {0:?}")]
    Error(String),
    #[error("CarboneSDK: error: {0:?} can not be empty")]
    EmptyString(String),
    #[error("CarboneSDK: \"API URL\" is missing")]
    MissingApiUrl,
    #[error("CarboneSDK: \"API VERSION\" is missing")]
    MissingApiVersion,
    #[error("CarboneSDK: \"template_id\" is missing")]
    MissingTemplateId,
    #[error("CarboneSDK: \"template_file_name\" is missing")]
    MissingTemplateFileName,
    #[error("CarboneSDK: \"render_id\" is missing")]
    MissingRenderId,
    #[error("CarboneSDK: \"render_options\" is missing")]
    MissingRenderOptions,
    #[error("Carbone SDK error: argument is missing: {0:?}")]
    MissingArgument(String),
    #[error("Carbone SDK error: file {0:?} not found")]
    FileNotFound(String),
    #[error("Carbone SDK {0:?} is a directory")]
    IsADirectory(String),
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

impl From<anyhow::Error> for CarboneSdkError {
    fn from(err: anyhow::Error) -> Self {
        CarboneSdkError::Error(err.to_string())
    }
}
