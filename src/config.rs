use std::fmt;

use crate::errors::CarboneSdkError;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

use crate::carbone::CARBONE_API_URL;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    api_token: String,
    api_url: String,
    api_timeout: u8,
    api_version: String,
}

pub type Result<Config> = std::result::Result<Config, CarboneSdkError>;

impl Config {

    pub fn new(api_token: String, api_url: String, api_timeout: u8, api_version: String) -> Result<Self> {

       Self::assert_api_token(&api_token)?;
       Self::assert_api_url(&api_url)?;
       Self::assert_api_version(&api_version)?;

       Ok(
        Self {
                api_token,
                api_url,
                api_timeout,
                api_version
            }
        )
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let file = fs::read_to_string(path).or(Err(CarboneSdkError::FileNotFound(
            "from_file()".to_string(),
            path.to_string(),
        )));

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

        if config.api_token.is_empty() {
            return Err(CarboneSdkError::MissingApiToken);
        }

        Ok(config)
    }


    pub fn get_api_token(&self) -> &String {
        &self.api_token
    }

    pub fn set_api_token(&mut self, api_token: String) -> Result<()> {
        Self::assert_api_token(&api_token)?;
        self.api_token = api_token;
        Ok(())
    }

    pub fn get_api_url(&self) -> &String {
        &self.api_url
    } 
    
    pub fn set_api_url(&mut self, api_url: String) -> Result<()> {
        Self::assert_api_url(&api_url)?;
        self.api_url = api_url;
        Ok(())
    }

    pub fn get_api_timeout(&self) -> &u8 {
        &self.api_timeout
    } 

    pub fn set_api_timeout(&mut self, api_timeout: u8) {
        self.api_timeout = api_timeout;
    }
    
    pub fn get_api_version(&self) -> &String {
        &self.api_version
    }

    pub fn set_api_version(&mut self, api_version: String) -> Result<()> {
        Self::assert_api_version(&api_version)?;
        self.api_version = api_version;
        Ok(())
    }

    fn assert_api_token(field: &String) -> Result<()>{
        if field.is_empty() {
            return Err(CarboneSdkError::MissingApiToken);
        }
        Ok(())
    }

    fn assert_api_url(field: &String) -> Result<()>{
        if field.is_empty() {
            return Err(CarboneSdkError::MissingApiUrl);
        }
        Ok(())
    }

    fn assert_api_version(field: &String) -> Result<()>{
        if field.is_empty() {
            return Err(CarboneSdkError::MissingApiVersion);
        }
        Ok(())
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
        write!(f, "Config {{ api_token: {}, api_url: {} , api_timeout: {}, api_version: {} }}", self.api_token, self.api_url, self.api_timeout, self.api_version)
    }
}
