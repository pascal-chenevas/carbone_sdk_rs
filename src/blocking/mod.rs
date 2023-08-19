use bytes::Bytes;

use std::time::Duration;

use reqwest::blocking::multipart;
use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::StatusCode;

use crate::carbone_response::APIResponse;
use crate::config::Config;
use crate::errors::*;
use crate::render::*;
use crate::template::*;
use crate::types::ApiJsonToken;

use crate::types::Result;

#[derive(Debug, Clone)]
pub struct Carbone<'a> {
    config: &'a Config,
    http_client: Client,
}

impl<'a> Carbone<'a> {
    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "carbone-version",
            HeaderValue::from_str(config.api_version.as_str()).unwrap(),
        );

        let bearer = format!("Bearer {}", api_token.as_str());

        let mut auth_value = header::HeaderValue::from_str(bearer.as_str()).unwrap();
        auth_value.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth_value);

        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.api_timeout))
            .build()?;

        Ok(Self {
            config,
            http_client,
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
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        let response = self.http_client.delete(url).send();

        match response {
            Ok(response) => {
                let json = response.json::<APIResponse>()?;

                if json.success {
                    Ok(true)
                } else {
                    Err(CarboneError::Error(json.error.unwrap()))
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
    ///     let template_content = carbone.download_template(&template_id)?;
    ///
    ///     assert_eq!(template_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn download_template(&self, template_id: &TemplateId) -> Result<Bytes> {
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        let response = self.http_client.get(url).send();

        match response {
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    Ok(r.bytes()?)
                } else {
                    let json = r.json::<APIResponse>()?;
                    Err(CarboneError::Error(json.error.unwrap()))
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
    ///     let json_data_value = String::from(r#"
    ///         "data" : {
    ///             "firstname" : "John",
    ///             "lastname" : "Wick"
    ///         },
    ///         "convertTo" : "odt"
    ///     "#);
    ///
    ///     let json_data = JsonData::new(json_data_value)?;
    ///
    ///     let template_file = &TemplateFile::new("/path/to/template.odf".to_string(), None)?;
    ///     let report_content = carbone.generate_report_with_file(&template_file, json_data, None)?;
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn generate_report_with_file(
        &self,
        template_file: &TemplateFile,
        json_data: JsonData,
        payload: Option<&str>,
    ) -> Result<Bytes> {
        
        let template_id_generated = template_file.generate_id(payload)?;

        let result = self.download_template(&template_id_generated);

        let template_id = if result.is_err() {
            self.upload_template(&template_file, None)?
        } else {
            template_id_generated
        };

        let render_id = self.render_data(template_id, json_data)?;
        let report_content = self.get_report(&render_id)?;

        Ok(report_content)
    }

    /// Get a new report.
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
        let url = format!("{}/render/{}", self.config.api_url, render_id.as_str());

        let response = self.http_client.get(url).send();

        match response {
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    Ok(r.bytes()?)
                } else {
                    let json = r.json::<APIResponse>()?;
                    Err(CarboneError::Error(json.error.unwrap()))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
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
    ///     let json_data_value = String::from(r#"
    ///         "data" : {
    ///             "firstname" : "John",
    ///             "lastname" : "Wick"
    ///         },
    ///         "convertTo" : "odt"
    ///     "#);
    ///
    ///     let json_data = JsonData::new(json_data_value)?;
    ///     let report_content = carbone.generate_report_with_template_id(template_id, json_data)?;
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn generate_report_with_template_id(
        &self,
        template_id: TemplateId,
        json_data: JsonData,
    ) -> Result<Bytes> {
        let render_id = self.render_data(template_id, json_data)?;
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
    /// use carbone_sdk_rs::render::JsonData;
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
    ///     let json_data_value = String::from(r#"
    ///         "data" : {
    ///             "firstname" : "John",
    ///             "lastname" : "Wick"
    ///         },
    ///         "convertTo" : "odt"
    ///     "#);
    ///
    ///     let json_data = JsonData::new(json_data_value)?;
    ///
    ///     let render_id = carbone.render_data(template_id, json_data)?;
    ///
    ///     assert_eq!(render_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn render_data(&self, template_id: TemplateId, json_data: JsonData) -> Result<RenderId> {
        let url = format!("{}/render/{}", self.config.api_url, template_id.as_str());

        let response = self
            .http_client
            .post(url)
            .header("Content-Type", "application/json")
            .body(json_data.as_str().to_owned())
            .send();

        match response {
            Ok(response) => {
                let json = response.json::<APIResponse>()?;

                if json.success {
                    Ok(json.data.unwrap().render_id.unwrap())
                } else {
                    Err(CarboneError::Error(json.error.unwrap()))
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
    ///     let template_file = TemplateFile::new("template.odt".to_string(), None)?;
    ///
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///     let template_id = carbone.upload_template(&template_file, None)?;
    ///
    ///     assert_eq!(template_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn upload_template(
        &self,
        template_file: &TemplateFile,
        salt: Option<&str>,
    ) -> Result<TemplateId> {
        let salt = match salt {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };

        let form = multipart::Form::new()
            .text("", salt)
            .file("template", template_file.path_as_str())?;

        let url = format!("{}/template", self.config.api_url);

        let response = self.http_client.post(url).multipart(form).send();

        match response {
            Ok(response) => {
                let json = response.json::<APIResponse>()?;

                if json.success {
                    Ok(json.data.unwrap().template_id.unwrap())
                } else {
                    Err(CarboneError::Error(json.error.unwrap()))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
    }
}
