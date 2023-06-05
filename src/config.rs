use std::fmt;

use validator::Validate;

use crate::errors::CarboneSdkError;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

use crate::carbone::CARBONE_API_URL;

#[derive(Debug, Clone, Deserialize, Validate, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[validate(url)]
    pub api_url: String,
    pub api_timeout: u8,
    pub api_version: u32,
}

pub type Result<Config> = std::result::Result<Config, CarboneSdkError>;

impl Config {

    pub fn new(api_url: String, api_timeout: u8, api_version: u32) -> Result<Self> {

       Ok(
        Self {
                api_url,
                api_timeout,
                api_version
            }
        )
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let file = fs::read_to_string(path).or(Err(CarboneSdkError::FileNotFound(path.to_string())));

        let file_content = match file {
            Ok(content) => content,
            Err(e) => {
                return Err(CarboneSdkError::Error(e.to_string()));
            }
        };
        //println!("{:#?}", file_content);
        // Read the JSON contents of the file as an instance of `Config`.
        let config: Self = match serde_json::from_str(file_content.as_str()) {
            Ok(config) => config,
            Err(e) => {
                return Err(CarboneSdkError::Error(e.to_string()));
            }
        };

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self { 
       Self{
            api_url: CARBONE_API_URL.to_string(),
            api_timeout: 60,
            api_version: 4,
        }
    }
}

impl FromStr for Config {

    type Err = CarboneSdkError;

    fn from_str(s: &str) -> Result<Self> {

        match serde_json::from_str(s) {
            Ok(config) => Ok(config),
            Err(e) => {
                Err(CarboneSdkError::ParseError(
                    "from_str".to_string(),
                    e.to_string(),
                ))
            }
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config {{ api_url: {} , api_timeout: {}, api_version: {} }}", self.api_url, self.api_timeout, self.api_version)
    }
}
