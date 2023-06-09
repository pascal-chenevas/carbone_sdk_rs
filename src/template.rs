use std::str;
use std::fs;
use std::path::Path;

use bytes::Bytes;

use reqwest::blocking::multipart;
use reqwest::header::HeaderValue;

use sha2::{Digest, Sha256};

use crate::types::ApiJsonToken;
use crate::config::Config;
use crate::errors::CarboneSdkError;
use crate::carbone_response::CarboneSDKResponse;

use crate::carbone::Result;

pub struct Template {
    config: Config,
    api_token: ApiJsonToken,
}

impl Template {

    pub fn new(config: Config, api_token: ApiJsonToken) -> Self {
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
    ///             Err(e) => "".to_string()
    ///     };
    /// 
    ///     let config = Config::new("http://127.0.0.1".to_string(), 4, 2)?;
    /// 
    ///     let api_token = ApiJsonToken::new(token)?;
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
    /// use carbone_sdk_rs::template::Template;
    /// use carbone_sdk_rs::errors::CarboneSdkError;
    ///
    /// fn main() -> Result<(), CarboneSdkError> {
    ///    
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => "".to_string()
    ///     };
    /// 
    ///     let config = Config::new("http://127.0.0.1".to_string(), 4, 2)?;
    /// 
    ///     let api_token = ApiJsonToken::new(token)?;
    /// 
    ///     let template_file = String::from("template.odt");
    /// 
    ///     let template = Template::new(config, api_token);
    ///     let template_id = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string();
    ///     let template_content = template.download(&template_id)?;
    /// 
    ///     assert_eq!(template_content.is_empty(), false);
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn download(&self, template_id: &String) -> Result<Bytes> {
        if template_id.is_empty() {
            return Err(CarboneSdkError::MissingTemplateId);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id);

        // TODO move new client to new() method
        let response = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(self.api_token.as_str())
            .send();

        match response {
            Ok(response) => Ok(response.bytes()?),
            Err(e) => Err(CarboneSdkError::ResponseError(e.to_string())),
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
    /// use carbone_sdk_rs::template::Template;
    /// use carbone_sdk_rs::errors::CarboneSdkError;
    ///
    /// fn main() -> Result<(), CarboneSdkError> {
    ///    
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => "".to_string()
    ///     };
    /// 
    ///     let config = Config::new("http://127.0.0.1".to_string(), 4, 2)?;
    /// 
    ///     let api_token = ApiJsonToken::new(token)?;
    /// 
    ///     let template_file = String::from("template.odt");
    /// 
    ///     let template = Template::new(config, api_token);
    ///     let template_id = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string();
    ///     let is_deleted = template.delete(&template_id)?;
    /// 
    ///     assert_eq!(is_deleted, true);
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn delete(&self, template_id: &String) -> Result<bool> {
        if template_id.is_empty() {
            return Err(CarboneSdkError::MissingTemplateId);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id);

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