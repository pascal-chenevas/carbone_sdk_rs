use crate::errors::CarboneSdkError;
use crate::template::TemplateId;

use crate::config::Config;
use crate::types::ApiJsonToken;

use crate::carbone::Result;
use crate::carbone_response::CarboneSDKResponse;

use reqwest::header::HeaderValue;

use crate::template::Template;

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
        render_options: String,
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
        render_options: String,
    ) -> Result<String> {
        Ok(self.render_data(template_id, render_options)?)
    }

    fn render_data(&self, template_id: TemplateId, render_options: String) -> Result<String> {
        if render_options.is_empty() {
            return Err(CarboneSdkError::MissingRenderOptions);
        }
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
            .body(render_options)
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