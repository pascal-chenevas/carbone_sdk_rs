use serde::{Deserialize, Serialize};

use crate::errors::CarboneError;

pub type Result<T> = std::result::Result<T, CarboneError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiJsonToken(String);

impl ApiJsonToken {
    pub fn new(s: String) -> Result<Self> {
        if s.len() >= 300 {
            Ok(ApiJsonToken(s))
        } else {
            Err(CarboneError::Error("wrong token length".to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ApiVersion(String);

impl ApiVersion {
    pub fn new(s: String) -> Result<Self> {
        if !s.is_empty() {
            Ok(ApiVersion(s))
        } else {
            Err(CarboneError::Error("wrong token length".to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Id(String);

impl Id {
    pub fn new<T: Into<String>>(id: T, type_name: &str) -> Result<Self> {
        let id = id.into();
        if !id.is_empty() {
            Ok(Id(id))
        } else {
            Err(CarboneError::EmptyString(type_name.to_string()))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonData {
    render_options: String,
}

impl JsonData {
    /// Create a new render_options.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::types::JsonData;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///
    ///  let render_options_value = r#"
    ///        "data" : {
    ///            "firstname" : "John",
    ///            "lastname" : "Wick"
    ///        },
    ///        "convertTo" : "odt"
    ///    "#;
    ///
    ///    let render_options = JsonData::new(render_options_value.to_string())?;
    ///
    ///    assert_eq!(render_options.as_str(), render_options_value);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(s: String) -> Result<Self> {
        if s.is_empty() {
            return Err(CarboneError::EmptyString("json_data".to_string()));
        }
        Ok(Self { render_options: s })
    }

    pub fn as_str(&self) -> &str {
        &self.render_options
    }
}
