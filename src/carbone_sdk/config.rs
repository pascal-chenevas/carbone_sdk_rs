use crate::carbone_sdk::errors::CarboneSdkError;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_token: String,
    pub api_url: String,
    pub api_timeout: i32,
    pub api_version: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, CarboneSdkError> {
        let file = fs::read_to_string(path).or(Err(CarboneSdkError::FileNotFound(
            "from_file()".to_string(),
            path.to_string(),
        )));

        let file_content = match file {
            Ok(content) => content,
            Err(e) => {
                return Err(CarboneSdkError::Error(
                    "from_file()".to_string(),
                    e.to_string(),
                ));
            }
        };
        //println!("{:#?}", file_content);
        // Read the JSON contents of the file as an instance of `Config`.
        let config: Self = match serde_json::from_str(file_content.as_str()) {
            Ok(config) => config,
            Err(e) => {
                return Err(CarboneSdkError::Error(
                    "from_file()".to_string(),
                    e.to_string(),
                ));
            }
        };
        Ok(config)
    }
}
