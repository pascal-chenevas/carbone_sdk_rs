use crate::errors::CarboneSdkError;
use crate::template::TemplateId;

use crate::config::Config;
use crate::types::ApiJsonToken;

use crate::carbone::Result;
use crate::carbone_response::CarboneSDKResponse;

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
    /// use carbone_sdk_rs::errors::CarboneSdkError;
    ///
    /// fn main() -> Result<(), CarboneSdkError> {
    ///    
    ///     let render_options_value = r#"
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

        if s.len() == 0 {
            return Err(CarboneSdkError::EmptyString("render_options".to_string()));
        }
        Ok(Self{render_options: s})  
    }
  
    pub fn as_str(&self) -> &str { &self.render_options }
}

pub struct Render<'a> {
    config: &'a Config,
    api_token: &'a ApiJsonToken,
}

impl <'a>Render<'a> {

    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Self {
        Self {
            config,
            api_token
        }
    }

    pub fn render_report_with_file(
        &self,
        file_name: String,
        render_options: RenderOptions,
        payload: &str
    ) -> Result<String> {

        let template: Template = Template::new(self.config, self.api_token);
        let generated_template_id = template.generate_id(&file_name, payload)?;
        let template_id = TemplateId::new(generated_template_id)?;

        let render_id = self.render_data(template_id, render_options)?;
        
        Ok(render_id)
    }

    pub fn render_report_with_template_id(
        &self,
        template_id: TemplateId,
        render_options: RenderOptions,
    ) -> Result<String> {
        Ok(self.render_data(template_id, render_options)?)
    }

    fn render_data(&self, template_id: TemplateId, render_options: RenderOptions) -> Result<String> {

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
                let json = response.json::<CarboneSDKResponse>()?;
                let render_id = json.get_render_id();
                let error_msg = json.get_error_message();

                if json.success {
                    Ok(render_id)
                } else {
                    Err(CarboneSdkError::ResponseError(error_msg))
                }
            }
            Err(e) => Err(CarboneSdkError::RequestError(e)),
        }
    }
}