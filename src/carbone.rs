use bytes::Bytes;

use reqwest::header::HeaderValue;
use reqwest::StatusCode;

use crate::carbone_response::ResponseBody;
use crate::config::Config;
use crate::errors::*;
use crate::render::*;
use crate::template::*;
use crate::types::ApiJsonToken;

pub type Result<T> = std::result::Result<T, CarboneError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carbone<'a> {
    config: &'a Config,
    api_token: &'a ApiJsonToken,
    template: Template<'a>,
    render: Render<'a>,
}

impl<'a> Carbone<'a> {
    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Result<Self> {
        let template: Template<'a> = Template::new(config, api_token);
        let render: Render<'a> = Render::new(config, api_token);

        Ok(Self {
            config,
            api_token,
            template,
            render,
        })
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
    /// use carbone_sdk_rs::render::{Render, RenderId};
    /// use carbone_sdk_rs::carbone::Carbone;
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
    ///     let api_token = &ApiJsonToken::new(token)?;
    ///
    ///     let carbone_sdk = Carbone::new(&config, api_token)?;
    ///
    ///     let render_id = &RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.pdf".to_string())?;
    ///     let report_content = carbone_sdk.get_report(render_id)?;
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_report(&self, render_id: &RenderId) -> Result<Bytes> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, render_id.as_str());

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
    /// use carbone_sdk_rs::carbone::Carbone;
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
    ///     let carbone_sdk = Carbone::new(&config, api_token)?;
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
    ///     let report_content = carbone_sdk.generate_report_with_template_id(template_id, render_options)?;
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
        let render_id = self
            .render
            .render_report_with_template_id(template_id, render_options)?;
        let report_content = self.get_report(&render_id)?;

        Ok(report_content)
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
    /// use carbone_sdk_rs::carbone::Carbone;
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
    ///     let carbone_sdk = Carbone::new(&config, api_token)?;
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
    ///     let report_content = carbone_sdk.generate_report_with_file(&template_file, render_options, "")?;
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
            self.render
                .render_report_with_file(template_file, render_options, payload)?;
        let report_content = self.get_report(&render_id)?;

        Ok(report_content)
    }

    pub fn template(&self) -> &Template {
        &self.template
    }

    pub fn render(&self) -> &Render {
        &self.render
    }
}
