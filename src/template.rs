use std::fs;
use std::path::Path;
use std::str;

use std::fs::Metadata;

use std::ops::Deref;

use bytes::Bytes;

use reqwest::blocking::multipart;
use reqwest::header::HeaderValue;
use reqwest::StatusCode;

use sha2::{Digest, Sha256};

use crate::carbone_response::ResponseBody;
use crate::config::Config;
use crate::errors::CarboneError;
use crate::types::*;

use crate::carbone::Result;

#[derive(Debug, Clone)]
pub struct TemplateFile {
    path: String,
    pub metadata: Metadata,
}

impl TemplateFile {
    pub fn new(path: String) -> Result<Self> {
        if Path::new(path.as_str()).is_dir() {
            return Err(CarboneError::IsADirectory(path));
        }

        if !Path::new(path.as_str()).is_file() {
            return Err(CarboneError::TemplateFileNotFound(path));
        }

        let metadata = fs::metadata(path.as_str())?;

        Ok(Self { path, metadata })
    }

    pub fn path_as_str(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateId(Id);

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
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    ///
    ///     assert_eq!(template_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new<T: Into<String>>(id: T) -> Result<Self> {
        let id = Id::new(id, "template_id")?;
        Ok(TemplateId(id))
    }
}

impl Deref for TemplateId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for TemplateId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template<'a> {
    config: &'a Config,
    api_token: &'a ApiJsonToken,
}

impl<'a> Template<'a> {
    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Self {
        Self { config, api_token }
    }

    pub fn generate_id(&self, template_file: &TemplateFile, payload: &str) -> Result<TemplateId> {
        let file_content = fs::read(template_file.path_as_str())?;

        let mut sha256 = Sha256::new();

        sha256.update(payload);
        sha256.update(file_content);

        // convert [u8] to String
        let result: String = format!("{:X}", sha256.finalize());

        TemplateId::new(result.to_lowercase())
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
    /// use carbone_sdk_rs::template::{Template, TemplateFile};
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    ///
    ///     let config: Config = Default::default();
    ///
    ///     let api_token = ApiJsonToken::new(token)?;
    ///
    ///     let template_file = TemplateFile::new("template.odt".to_string())?;
    ///
    ///     let template = Template::new(&config, &api_token);
    ///     let template_id = template.upload(&template_file, "".to_string())?;
    ///
    ///     assert_eq!(template_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn upload(&self, template_file: &TemplateFile, salt: String) -> Result<TemplateId> {
        let form = multipart::Form::new()
            .text("", salt)
            .file("template", template_file.path_as_str())?;

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
                let json = response.json::<ResponseBody>()?;
                let error_msg = json.get_error_message();

                if json.success {
                    let template_id = json.get_template_id()?;
                    Ok(template_id)
                } else {
                    Err(CarboneError::BadRequest(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
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
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let token = match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    ///
    ///     let config: Config = Default::default();
    ///
    ///     let api_token = ApiJsonToken::new(token)?;
    ///
    ///     let template_file = String::from("template.odt");
    ///
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    ///     let template = Template::new(&config, &api_token);
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
        let response = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(self.api_token.as_str())
            .send();

        match response {
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    Ok(r.bytes()?)
                } else {
                    let json = r.json::<ResponseBody>()?;
                    let error_msg = json.get_error_message();
                    Err(CarboneError::BadRequest(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
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
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    ///
    ///     let config: Config = Default::default();
    ///
    ///     let api_token = ApiJsonToken::new(token)?;
    ///
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    ///     let template = Template::new(&config, &api_token);
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
                let json = response.json::<ResponseBody>()?;
                let error_msg = json.get_error_message();

                if json.success {
                    Ok(true)
                } else {
                    Err(CarboneError::BadRequest(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
    }
}
