use crate::carbone_sdk::errors::CarboneSdkError;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

use crate::carbone_sdk::carbone::CARBONE_API_URL;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_token: String,
    pub api_url: String,
    pub api_timeout: i32,
    pub api_version: String,
}

pub type Result<Config> = std::result::Result<Config, CarboneSdkError>;

impl Config {

    pub fn from_file(path: &str) -> Result<Self> {
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

        if config.api_token.is_empty() {
            return Err(CarboneSdkError::MissingApiToken("from_file()".to_string()));
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self { 
       Self{
            api_url: CARBONE_API_URL.to_string(),
            api_timeout: 60,
            api_token: "".to_string(),
            api_version: "4".to_string(),
        }
    }
}

impl FromStr for Config {

    type Err = CarboneSdkError;

    fn from_str(s: &str) -> Result<Self> {

        match serde_json::from_str(&s) {
            Ok(config) => Ok(config),
            Err(e) => {
                return Err(CarboneSdkError::ParseError(
                    "from_str".to_string(),
                    e.to_string(),
                ));
            }
        }
    }
}
