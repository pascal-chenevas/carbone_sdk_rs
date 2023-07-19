use bytes::Bytes;

use reqwest::header::HeaderValue;
use reqwest::StatusCode;
use reqwest::blocking::multipart;

use crate::carbone_response::ResponseBody;
use crate::config::Config;
use crate::errors::*;
use crate::render::*;
use crate::template::*;
use crate::types::ApiJsonToken;

use crate::types::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carbone<'a> {
    config: &'a Config,
    api_token: &'a ApiJsonToken,
}

impl<'a> Carbone<'a> {
    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Result<Self> {
        Ok(Self {
            config,
            api_token
        })
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
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::TemplateId;
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
    ///
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///     let is_deleted = carbone.delete_template(template_id)?;
    ///
    ///     assert_eq!(is_deleted, true);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn delete_template(&self, template_id: TemplateId) -> Result<bool> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

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
                    Err(CarboneError::Error(error_msg))
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
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::TemplateId;
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
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///     
    ///     let template_content = carbone.download_template(template_id)?;
    ///
    ///     assert_eq!(template_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn download_template(&self, template_id: TemplateId) -> Result<Bytes> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

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
                    Err(CarboneError::Error(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
    }

    /// Generate a report.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::render::*;
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::{TemplateFile,TemplateId};
    ///
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
    ///     let api_token = &ApiJsonToken::new(token)?;
    ///
    ///     let carbone = Carbone::new(&config, api_token)?;
    ///
    ///     let render_options_value = String::from(r#"
    ///         "data" : {
    ///             "firstname" : "John",
    ///             "lastname" : "Wick"
    ///         },
    ///         "convertTo" : "odt"
    ///     "#);
    ///
    ///     let render_options = RenderOptions::new(render_options_value)?;
    ///
    ///     let template_file = &TemplateFile::new("/path/to/template.odf".to_string())?;
    ///     let report_content = carbone.generate_report_with_file(&template_file, render_options, "")?;
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn generate_report_with_file(
        &self,
        template_file: &TemplateFile,
        render_options: RenderOptions,
        payload: &str,
    ) -> Result<Bytes> {
        let render_id =
            self.render_report_with_file(template_file, render_options, payload)?;
        let report_content = self.get_report(&render_id)?;

        Ok(report_content)
    }

    /// Create a new render_options.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::render::RenderId;
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
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
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///
    ///     let render_id = &RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.pdf".to_string())?;
    ///     let report_content = carbone.get_report(render_id)?;
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_report(&self, render_id: &RenderId) -> Result<Bytes> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, render_id.as_str());

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
                    Err(CarboneError::Error(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
    }

    /// Render data with a given template file.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::render::RenderOptions;
    /// use carbone_sdk_rs::template::TemplateFile;
    /// use carbone_sdk_rs::errors::CarboneError;
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    ///
    ///     let config: Config = Default::default();
    ///     let api_token = ApiJsonToken::new(token)?;
    ///
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///    
    ///     let render_options_value = String::from(r#"
    ///         "data" : {
    ///             "firstname" : "John",
    ///             "lastname" : "Wick"
    ///         },
    ///         "convertTo" : "odt"
    ///     "#);
    ///
    ///     let render_options = RenderOptions::new(render_options_value)?;
    ///
    ///     let template_file = &TemplateFile::new("/path/to/template.odf".to_string())?;
    ///     let render_id = carbone.render_report_with_file(template_file, render_options, "")?;
    ///
    ///     assert_eq!(render_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn render_report_with_file(
        &self,
        template_file: &TemplateFile,
        render_options: RenderOptions,
        payload: &str,
    ) -> Result<RenderId> {

        let template_id = template_file.generate_id(payload)?;
        let render_id = self.render_report_with_template_id(template_id, render_options)?;

        Ok(render_id)
    }

    /// Generate a report with a template_id given.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::render::*;
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::TemplateId;
    ///
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
    ///     let api_token = &ApiJsonToken::new(token)?;
    ///
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///
    ///     let render_options_value = String::from(r#"
    ///         "data" : {
    ///             "firstname" : "John",
    ///             "lastname" : "Wick"
    ///         },
    ///         "convertTo" : "odt"
    ///     "#);
    ///
    ///     let render_options = RenderOptions::new(render_options_value)?;
    ///     let report_content = carbone.generate_report_with_template_id(template_id, render_options)?;
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn generate_report_with_template_id(
        &self,
        template_id: TemplateId,
        render_options: RenderOptions,
    ) -> Result<Bytes> {
        let render_id = self.render_report_with_template_id(template_id, render_options)?;
        let report_content = self.get_report(&render_id)?;

        Ok(report_content)
    }

    /// Render data with a given template_id.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::render::RenderOptions;
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::template::TemplateId;
    /// use carbone_sdk_rs::errors::CarboneError;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///
    ///     let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    ///
    ///     let config: Config = Default::default();
    ///     let api_token = ApiJsonToken::new(token)?;
    ///
    ///     let template_id = TemplateId::new("foiejwoi21e093ru3209jf2093j".to_string())?;
    ///
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///    
    ///     let render_options_value = String::from(r#"
    ///         "data" : {
    ///             "firstname" : "John",
    ///             "lastname" : "Wick"
    ///         },
    ///         "convertTo" : "odt"
    ///     "#);
    ///
    ///     let render_options = RenderOptions::new(render_options_value)?;
    ///
    ///     let render_id = carbone.render_report_with_template_id(template_id, render_options)?;
    ///
    ///     assert_eq!(render_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn render_report_with_template_id(
        &self,
        template_id: TemplateId,
        render_options: RenderOptions,
    ) -> Result<RenderId> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, template_id.as_str());

        let response = client
            .post(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .header("Content-Type", "application/json")
            .bearer_auth(self.api_token.as_str())
            .body(render_options.as_str().to_owned())
            .send();

        match response {
            Ok(response) => {
                let json = response.json::<ResponseBody>()?;
                let error_msg = json.get_error_message();

                if json.success {
                    let render_id = json.get_render_id()?;
                    Ok(render_id)
                } else {
                    Err(CarboneError::Error(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
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
    /// use carbone_sdk_rs::blocking::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::TemplateFile;
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
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///     let template_id = carbone.upload_template(&template_file, "".to_string())?;
    ///
    ///     assert_eq!(template_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn upload_template(&self, template_file: &TemplateFile, salt: String) -> Result<TemplateId> {
        let form = multipart::Form::new()
            .text("", salt)
            .file("template", template_file.path_as_str())?;

        let client = reqwest::blocking::Client::new();

        let url = format!("{}/template", self.config.api_url);

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
                    Err(CarboneError::Error(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
    }
}
