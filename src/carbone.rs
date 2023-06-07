
use std::str;
use std::fs;

use bytes::Bytes;

use reqwest::header::HeaderValue;

use sha2::{Digest, Sha256};

use validator::Validate;

use crate::carbone_response::CarboneSDKResponse;
use crate::config::Config;
use crate::errors::*;

pub type Result<T> = std::result::Result<T, CarboneSdkError>;

#[derive(Debug, Validate, PartialEq, Eq)]
pub struct CarboneSDK<'a>{
    pub config: &'a Config,
    #[validate(length(min = 357))]
    api_token: String,
}

impl <'a>CarboneSDK<'a> {
    pub fn new(config: &'a Config, api_token: String) -> Result<Self> {
        Ok(Self { config: config, api_token: api_token })
    }

    pub fn render_report(
        &self,
        template_id: &String,
        render_options: String,
    ) -> Result<String> {
        if template_id.is_empty() {
            return Err(CarboneSdkError::MissingTemplateId);
        }
        if render_options.is_empty() {
            return Err(CarboneSdkError::MissingRenderOptions);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, template_id);

        // TODO move new client to new() method
        let response = client
            .post(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_token)
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

    // TODO return also name of the report from headers
    pub fn get_report(&self, render_id: &String) -> Result<Bytes> {
        if render_id.is_empty() {
            return Err(CarboneSdkError::MissingRenderId);
        }

        let client = reqwest::blocking::Client::new();
        let url = format!("{}/render/{}", self.config.api_url, render_id);

        // TODO move new client to new() method
        let response = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .bearer_auth(&self.api_token)
            .send();

        match response {
            Ok(response) => Ok(response.bytes()?),
            Err(e) => Err(CarboneSdkError::ResponseError(e.to_string())),
        }
    }

    pub fn generate_template_id(
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

    pub fn render(
        &self,
        file_or_template_id: &str,
        json_data: &str,
        payload: &str,
    ) -> Result<()> {
       panic!("function not implemented");
    }

    pub fn get_report_name_from_header(&self) -> String {
        "get_report_name_from_header() to be implemented".to_string()
    }

    pub fn get_status(&self) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/status", self.config.api_url);

        // TODO move new client to new() method
        let response = client
            .get(url)
            .header(
                "carbone-version",
                HeaderValue::from_str(&self.config.api_version.to_string()).unwrap(),
            )
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_token)
            .send();

        match response {
            Ok(response) => Ok(response.text()?),
            Err(e) => Err(CarboneSdkError::ResponseError(e.to_string())),
        }
    }

}