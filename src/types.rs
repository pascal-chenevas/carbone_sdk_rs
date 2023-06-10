use validator::Validate;
use anyhow::Result;

#[derive(Debug, Clone, Validate, PartialEq, Eq)]
pub struct ApiJsonToken {
    #[validate(length(min = 357))]
    api_token: String,
}

impl ApiJsonToken {
    pub fn new(s: String) -> Result<Self> {

        let api_token = Self {api_token: s};
        api_token.validate()?;
        Ok(api_token)  
    }
  
    pub fn as_str(&self) -> &str { &self.api_token }

  }