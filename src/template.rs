use std::str;
use std::fs;
use std::path::Path;

use bytes::Bytes;

use reqwest::blocking::multipart;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;

use sha2::{Digest, Sha256};

use crate::types::ApiJsonToken;
use crate::config::Config;
use crate::errors::CarboneSdkError;
use crate::carbone_response::CarboneSDKResponse;

use crate::carbone::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateId {
    id: String,
}

impl TemplateId {
    /// Create a new template_id.
    /// 
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    /// 
    /// use carbone_sdk_rs::template::TemplateId;
    /// use carbone_sdk_rs::errors::CarboneSdkError;
    ///
    /// fn main() -> Result<(), CarboneSdkError> {
    ///    
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    /// 
    ///     assert_eq!(template_id.as_str().is_empty(), false);
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn new(s: String) -> Result<Self> {
        if s.is_empty() {
            return Err(CarboneSdkError::EmptyString("template_id".to_string()));
        }
        let template_id = Self {id: s};
        Ok(template_id)  
    }
  
    pub fn as_str(&self) -> &str { &self.id }

  }

pub struct Template<'a> {
    config: &'a Config,
    api_token: &'a ApiJsonToken,
}

impl <'a>Template<'a> {

    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Self {
        Self {
            config,
            api_token
        }
    }

    pub fn generate_id(
        &self,
        template_file_name: &String,
        payload: &str,
    ) -> Result<String> {
        if template_file_name.is_empty() {
            return Err(CarboneSdkError::MissingTemplateFileName);
        }

        let file_content = fs::read(template_file_name)?;

        let mut sha256 = Sha256::new();

        sha256.update(payload);
        sha256.update(file_content);

        // convert [u8] to String
        let result: String = format!("{:X}", sha256.finalize());

        Ok(result.to_lowercase())
    }

    /// Upload a template to the Carbone Service.
    /// 
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    /// 
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::Template;
    /// use carbone_sdk_rs::errors::CarboneSdkError;
    ///
    /// fn main() -> Result<(), CarboneSdkError> {
    ///    
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    /// 
    ///     let config = &Config::new("http://127.0.0.1".to_string(), 4, 2)?;
    /// 
    ///     let api_token = &ApiJsonToken::new(token)?;
    /// 
    ///     let template_file = String::from("template.odt");
    /// 
    ///     let template = Template::new(config, api_token);
    ///     let template_id = template.upload(&template_file, "".to_string())?;
    /// 
    ///     assert_eq!(template_id.is_empty(), false);
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn upload(
        &self,
        template_file_name: &String,
        salt: String,
    ) -> Result<String> {
        if template_file_name.is_empty() {
            return Err(CarboneSdkError::MissingTemplateFileName);
        }

        if Path::new(template_file_name.as_str()).is_dir() {
            return Err(CarboneSdkError::IsADirectory(template_file_name.to_string()));
        }

        if !Path::new(template_file_name.as_str()).is_file() {
            return Err(CarboneSdkError::FileNotFound(template_file_name.to_string()));
        }

        let form = multipart::Form::new()
            .text("", salt)
            .file("template", template_file_name)?;

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template", self.config.api_url);

        // TODO move new client to new() method
        let response = client
            .post(url)
            .multipart(form)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(self.api_token.as_str())
            .send();

        match response {
            Ok(response) => {
                let json = response.json::<CarboneSDKResponse>()?;
                let template_id = json.get_template_id();
                let error_msg = json.get_error_message();

                if json.success {
                    Ok(template_id)
                } else {
                    Err(CarboneSdkError::ResponseError(error_msg))
                }
            }
            Err(e) => Err(CarboneSdkError::RequestError(e)),
        }
    }

    // Download a template from the Carbone Service.
    /// 
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    /// 
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::{Template, TemplateId};
    /// use carbone_sdk_rs::errors::CarboneSdkError;
    ///
    /// fn main() -> Result<(), CarboneSdkError> {
    ///    
    ///     let token = match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    /// 
    ///     let config = &Config::new("http://127.0.0.1".to_string(), 4, 2)?;
    /// 
    ///     let api_token = &ApiJsonToken::new(token)?;
    /// 
    ///     let template_file = String::from("template.odt");
    /// 
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    ///     let template = Template::new(config, api_token);
    ///     
    ///     let template_content = template.download(template_id)?;
    /// 
    ///     assert_eq!(template_content.is_empty(), false);
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn download(&self, template_id: TemplateId) -> Result<Bytes> {
       
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        // TODO move new client to new() method
        let response_result = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(self.api_token.as_str())
            .send();

        if response_result.is_err() {
            return Err(CarboneSdkError::RequestError(response_result.unwrap_err()));
        } else {

            let response = response_result.unwrap();
            if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
                if content_type == "application/json" {
                    let json = response.json::<CarboneSDKResponse>()?;
                    let error_msg = json.get_error_message();
                    Err(CarboneSdkError::ResponseError(error_msg))
                } else {
                    match content_type.to_str() {
                        Ok(v) =>  Err(CarboneSdkError::Error(format!("Content-Type `{}` not supported", v))),
                        Err(e) => Err(CarboneSdkError::Error(e.to_string())),
                    }
                }
            } else {
                Ok(response.bytes()?)
            }
        }
    }

    // Delete a template from the Carbone Service.
    /// 
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    /// 
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::{Template, TemplateId};
    /// use carbone_sdk_rs::errors::CarboneSdkError;
    ///
    /// fn main() -> Result<(), CarboneSdkError> {
    ///    
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    /// 
    ///     let config = &Config::new("http://127.0.0.1".to_string(), 4, 2)?;
    /// 
    ///     let api_token = &ApiJsonToken::new(token)?;
    /// 
    ///     let template_file = String::from("template.odt");
    /// 
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    ///     let template = Template::new(config, api_token);
    /// 
    ///     let is_deleted = template.delete(template_id)?;
    /// 
    ///     assert_eq!(is_deleted, true);
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn delete(&self, template_id: TemplateId) -> Result<bool> {
       
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        // TODO move new client to new() method
        let response = client
            .delete(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(self.api_token.as_str())
            .send();

        match response {
            Ok(response) => {
                let json = response.json::<CarboneSDKResponse>()?;
                let error_msg = json.get_error_message();

                if json.success {
                    Ok(true)
                } else {
                    Err(CarboneSdkError::ResponseError(error_msg))
                }
            }
            Err(e) => Err(CarboneSdkError::RequestError(e)),
        }
    }
}