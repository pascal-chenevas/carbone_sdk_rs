use std::str;

use bytes::Bytes;

use reqwest::header::HeaderValue;

use validator::Validate;

use crate::carbone_response::CarboneSDKResponse;
use crate::config::Config;
use crate::errors::*;
use crate::types::ApiJsonToken;
use crate::render::Render;
use crate::template::Template;
use crate::render::RenderId;

pub type Result<T> = std::result::Result<T, CarboneSdkError>;

#[derive(Debug, Validate, PartialEq, Eq)]
pub struct CarboneSDK<'a>{
    config: &'a Config,
    api_token: &'a ApiJsonToken,
    template: Template<'a>,
    render: Render<'a>,
}

impl <'a>CarboneSDK<'a> {
    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken) -> Result<Self> {
        
        let template: Template<'a> = Template::new(config, api_token);
        let render: Render<'a> = Render::new(config, api_token);

        Ok(Self { config: config, api_token: api_token, template: template, render: render })
    }

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
            Ok(response) => Ok(response.bytes()?),
            Err(e) => Err(CarboneSdkError::ResponseError(e.to_string())),
        }
    }


}