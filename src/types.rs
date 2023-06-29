use validator::Validate;
use anyhow::Result as AnyHowResult;

use crate::errors::CarboneError;
use crate::carbone::Result;

#[derive(Debug, Clone, Validate, PartialEq, Eq)]
pub struct ApiJsonToken {
    #[validate(length(min = 357))]
    api_token: String,
}

impl ApiJsonToken {
    pub fn new(s: String) -> AnyHowResult<Self> {

        let api_token = Self {api_token: s};
        api_token.validate()?;
        Ok(api_token)  
    }
  
    pub fn as_str(&self) -> &str { &self.api_token }

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
    pub fn as_str(&self) -> &str { &self.0 }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
