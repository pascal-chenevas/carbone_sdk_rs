pub const CARBONE_API_URL: &str = "https://api.carbone.io";
pub const CARBONE_API_VERSION: &str = "4";

use anyhow::{anyhow, Result};

use validator::Validate;

use crate::errors::CarboneError;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

use crate::types::ApiVersion;

#[derive(Debug, Clone, Deserialize, Validate, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[validate(url)]
    pub api_url: String,
    pub api_timeout: u64,
    pub api_version: ApiVersion,
}

impl Config {
    /// Create a new Configuraiton.
    ///
    /// This function will create new Config.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::errors::CarboneError;
    /// use carbone_sdk_rs::types::ApiVersion;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///     let api_version: ApiVersion = ApiVersion::new("4".to_string())?;
    ///     let config = Config::new(
    ///        "http://127.0.0.1:57780".to_string(),
    ///        4,
    ///        api_version)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(api_url: String, api_timeout: u64, api_version: ApiVersion) -> Result<Self> {
        let config = Self {
            api_url,
            api_timeout,
            api_version,
        };

        config.validate()?;
        Ok(config)
    }

    /// Load a Configuraiton from a file.
    ///
    /// This function will create new Config struct with,
    /// the values from the file.
    ///
    /// # Example
    ///
    /// ```no_run
    ///
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///     let config = Config::from_file("tests/config.test.json")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file(path: &str) -> Result<Self> {
        let file_content =
            fs::read_to_string(path).or(Err(CarboneError::FileNotFound(path.to_string())))?;
        let config: Self = Self::from_str(file_content.as_str())?;
        config.validate()?;
        Ok(config)
    }
}

/// Load a Default Configuraiton.
///
/// This function will create new Config struct the with,
/// the default values.
///
/// # Example
///
/// ```no_run
///
/// use carbone_sdk_rs::config::Config;
/// use carbone_sdk_rs::errors::CarboneError;
///
/// fn main() -> Result<(), CarboneError> {
///
///    let config: Config = Default::default();
///    
///     assert_eq!(config.api_url, "https://api.carbone.io".to_string());
///
///     Ok(())
/// }
/// ```
impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: CARBONE_API_URL.to_string(),
            api_timeout: 60,
            api_version: ApiVersion::new(CARBONE_API_VERSION.to_string()).unwrap(),
        }
    }
}

/// Load a Configuraiton from a str.
///
/// This function will create new Config struct with,
/// the values from the str given.
///
/// # Example
///
/// ```no_run
///
/// use std::str::FromStr;
/// use carbone_sdk_rs::config::Config;
/// use carbone_sdk_rs::errors::CarboneError;
///
/// fn main() -> Result<(), CarboneError> {
///     
///     let config = Config::from_str(r#"{
///         "apiUrl": "http://127.0.0.1",
///         "apiTimeout": 4,
///         "apiVersion" : "4"
///     }"#)?;
///
///     Ok(())
/// }
/// ```
impl FromStr for Config {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match serde_json::from_str(s) {
            Ok(config) => Ok(config),
            Err(e) => Err(anyhow!(format!(
                "CarboneSDK FromStr JsonParseError: {}",
                e.to_string()
            ))),
        }
    }
}
