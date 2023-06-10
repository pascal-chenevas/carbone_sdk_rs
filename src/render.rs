use crate::errors::CarboneSdkError;
use crate::template::TemplateId;

use crate::config::Config;
use crate::types::ApiJsonToken;

use crate::carbone::Result;
use crate::carbone_response::CarboneSDKResponse;

use reqwest::header::HeaderValue;

pub struct Render {
    config: Config,
    api_token: ApiJsonToken,
}

impl Render {

    pub fn new(config: Config, api_token: ApiJsonToken) -> Self {
        Self {
            config,
            api_token
        }
    }

    pub fn from_file(
        &self,
        file_path: String,
        json_data: &str,
        payload: &str,
    ) -> Result<()> {
       panic!("function not implemented");
    }

    pub fn render_report(
        &self,
        template_id: TemplateId,
        render_options: String,
    ) -> Result<String> {
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