
use crate::errors::CarboneError;

pub type Result<T> = std::result::Result<T, CarboneError>;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiJsonToken{
    api_token: String,
}

impl ApiJsonToken {
    pub fn new(s: String) -> Result<Self> {
        if s.len() >= 300 {
            Ok(Self{api_token: s})
        } else {
            Err(CarboneError::Error("wrong token length".to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.api_token
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
