use std::ops::Deref;

use crate::errors::CarboneError;
use crate::template::{TemplateFile, TemplateId};

use crate::config::Config;
use crate::types::*;

use crate::carbone::Result;
use crate::carbone_response::ResponseBody;

use reqwest::header::HeaderValue;

use crate::template::Template;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOptions {
    render_options: String,
}

impl RenderOptions {
    /// Create a new render_options.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::render::RenderOptions;
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
    ///    let render_options = RenderOptions::new(render_options_value.to_string())?;
    ///
    ///    assert_eq!(render_options.as_str(), render_options_value);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(s: String) -> Result<Self> {
        if s.is_empty() {
            return Err(CarboneError::EmptyString("render_options".to_string()));
        }
        Ok(Self { render_options: s })
    }

    pub fn as_str(&self) -> &str {
        &self.render_options
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderId(Id);

impl RenderId {
    /// Create a new render_id struct.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::render::RenderId;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let render_id = RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string())?;
    ///
    ///     assert_eq!(render_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new<T: Into<String>>(id: T) -> Result<Self> {
        let id = Id::new(id, "render_id")?;
        Ok(RenderId(id))
    }
}

impl Deref for RenderId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for RenderId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Render<'a> {
    config: &'a Config,
    api_token: &'a ApiJsonToken,
}

impl<'a> Render<'a> {
    /// Create a new render stuct.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::render::Render;
    /// use carbone_sdk_rs::errors::CarboneError;
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///
    ///    let token =  match env::var("CARBONE_TOKEN") {
    ///             Ok(v) => v,
    ///             Err(e) => panic!("{}", e.to_string())
    ///     };
    ///     let config: Config = Default::default();
    ///     let api_token = ApiJsonToken::new(token)?;
    ///    
    ///     let render = Render::new(&config, &api_token);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Self {
        Self { config, api_token }
    }

    /// Render data with a given template file.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::render::{Render, RenderOptions};
    /// use carbone_sdk_rs::template::{Template, TemplateFile};
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
    ///     let render = Render::new(&config, &api_token);
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
    ///     let render_id = render.render_report_with_file(template_file, render_options, "")?;
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
        let template: Template = Template::new(self.config, self.api_token);
        let template_id = template.generate_id(template_file, payload)?;

        let render_id = self.render_data(template_id, render_options)?;

        Ok(render_id)
    }

    /// Render data with a given template_id.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::render::{Render, RenderOptions};
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
    ///     let render = Render::new(&config, &api_token);
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
    ///     let render_id = render.render_report_with_template_id(template_id, render_options)?;
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
        self.render_data(template_id, render_options)
    }

    fn render_data(
        &self,
        template_id: TemplateId,
        render_options: RenderOptions,
    ) -> Result<RenderId> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, template_id.as_str());

        // TODO move new client to new() method
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
                    Err(CarboneError::BadRequest(error_msg))
                }
            }
            Err(e) => Err(CarboneError::RequestError(e)),
        }
    }
}
