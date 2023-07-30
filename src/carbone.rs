use bytes::Bytes;

use std::path::Path;
use std::time::Duration;

use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::multipart;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::StatusCode;

use crate::carbone_response::ResponseBody;
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
    /// use carbone_sdk_rs::carbone::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::TemplateId;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CarboneError> {
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
    ///     let is_deleted = carbone.delete_template(template_id).await.unwrap();
    ///
    ///     assert_eq!(is_deleted, true);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_template(&self, template_id: TemplateId) -> Result<bool> {
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        let response = self.http_client.delete(url).send().await?;

        let response_body = response.text().await?;

        let json: ResponseBody = match serde_json::from_str(response_body.as_str()) {
            Ok(s) => s,
            Err(e) => return Err(CarboneError::Error(e.to_string())),
        };

        if json.success {
            Ok(true)
        } else {
            let error_msg = json.get_error_message();
            Err(CarboneError::Error(error_msg))
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
    /// use carbone_sdk_rs::carbone::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::TemplateId;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CarboneError> {
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
    ///     let template_content = carbone.download_template(template_id).await.unwrap();
    ///
    ///     assert_eq!(template_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_template(&self, template_id: TemplateId) -> Result<Bytes> {
        let url = format!("{}/template/{}", self.config.api_url, template_id.as_str());

        let response = self.http_client.get(url).send().await?;

        if response.status() == StatusCode::OK {
            Ok(response.bytes().await?)
        } else {
            let response_body = response.text().await?;
            let json: ResponseBody = match serde_json::from_str(response_body.as_str()) {
                Ok(s) => s,
                Err(e) => return Err(CarboneError::Error(e.to_string())),
            };
            let error_msg = json.get_error_message();
            Err(CarboneError::Error(error_msg))
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
    /// use carbone_sdk_rs::carbone::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::{TemplateFile,TemplateId};
    ///
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CarboneError> {
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
    ///     let template_file = &TemplateFile::new("/path/to/template.odf".to_string(), None)?;
    ///     let report_content = carbone.generate_report_with_file(&template_file, render_options, None).await.unwrap();
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn generate_report_with_file(
        &self,
        template_file: &TemplateFile,
        render_options: RenderOptions,
        payload: Option<&str>,
    ) -> Result<Bytes> {
        let template_id = template_file.generate_id(payload)?;
        let render_id = self.render_data(template_id, render_options).await?;
        let report_content = self.get_report(&render_id).await?;

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
    /// use carbone_sdk_rs::carbone::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CarboneError> {
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
    ///     let report_content = carbone.get_report(render_id).await.unwrap();
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_report(&self, render_id: &RenderId) -> Result<Bytes> {
        let url = format!("{}/render/{}", self.config.api_url, render_id.as_str());

        let response = self.http_client.get(url).send().await?;

        if response.status() == StatusCode::OK {
            Ok(response.bytes().await?)
        } else {
            let response_body = response.text().await?;
            let json: ResponseBody = match serde_json::from_str(response_body.as_str()) {
                Ok(s) => s,
                Err(e) => return Err(CarboneError::Error(e.to_string())),
            };
            let error_msg = json.get_error_message();
            Err(CarboneError::Error(error_msg))
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
    /// #[tokio::main]
    /// async fn main() -> Result<(), CarboneError> {
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
    ///     let report_content = carbone.generate_report_with_template_id(template_id, render_options).await.unwrap();
    ///
    ///     assert_eq!(report_content.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn generate_report_with_template_id(
        &self,
        template_id: TemplateId,
        render_options: RenderOptions,
    ) -> Result<Bytes> {
        let render_id = self.render_data(template_id, render_options).await?;
        let report_content = self.get_report(&render_id).await?;

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
    /// use carbone_sdk_rs::carbone::Carbone;
    /// use carbone_sdk_rs::render::RenderOptions;
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::template::TemplateId;
    /// use carbone_sdk_rs::errors::CarboneError;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CarboneError> {
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
    ///     let render_id = carbone.render_data(template_id, render_options).await.unwrap();
    ///
    ///     assert_eq!(render_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn render_data(
        &self,
        template_id: TemplateId,
        render_options: RenderOptions,
    ) -> Result<RenderId> {
        let url = format!("{}/render/{}", self.config.api_url, template_id.as_str());

        let response = self
            .http_client
            .post(url)
            .header("Content-Type", "application/json")
            .body(render_options.as_str().to_owned())
            .send()
            .await?;

        let response_body = response.text().await?;

        let json: ResponseBody = match serde_json::from_str(response_body.as_str()) {
            Ok(s) => s,
            Err(e) => return Err(CarboneError::Error(e.to_string())),
        };

        if json.success {
            let render_id = json.get_render_id()?;
            Ok(render_id)
        } else {
            let error_msg = json.get_error_message();
            Err(CarboneError::Error(error_msg))
        }
    }

    /// Upload a template to the Carbone Service.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    /// use std::fs;
    /// 
    /// use carbone_sdk_rs::config::Config;
    /// use carbone_sdk_rs::carbone::Carbone;
    /// use carbone_sdk_rs::types::ApiJsonToken;
    /// use carbone_sdk_rs::template::TemplateFile;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CarboneError> {
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
    ///     let file_name = "template.odt";
    ///     let file_path = format!("tests/data/{}", file_name);
    ///     let filte_content = fs::read(file_path)?;
    ///
    ///     let carbone = Carbone::new(&config, &api_token)?;
    ///     let template_id = carbone.upload_template(file_name, filte_content, None).await.unwrap();
    ///
    ///     assert_eq!(template_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn upload_template(
        &self,
        file_name: &str,
        file_content: Vec<u8>,
        salt: Option<&str>,
    ) -> Result<TemplateId> {

        let salt = match salt {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };

        let file_path = Path::new(file_name);

        let file_name = file_path
            .file_name()
            .map(|filename| filename.to_string_lossy().into_owned());

        let file_name = match file_name {
            Some(s) => s,
            None => return Err(CarboneError::Error("Failed to fetch file name".to_string())),
        };

        let ext = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        let mime = mime_guess::from_ext(ext).first_or_octet_stream();

        let part = multipart::Part::bytes(file_content)
            .file_name(file_name)
            .mime_str(mime.as_ref())?;

        let form: multipart::Form = multipart::Form::new().text("", salt).part("template", part);

        let url = format!("{}/template", self.config.api_url);

        let response = self.http_client.post(url).multipart(form).send().await?;

        let response_body = response.text().await?;

        let json: ResponseBody = match serde_json::from_str(response_body.as_str()) {
            Ok(s) => s,
            Err(e) => return Err(CarboneError::Error(e.to_string())),
        };

        if json.success {
            let template_id = json.get_template_id()?;
            Ok(template_id)
        } else {
            let error_msg = json.get_error_message();
            Err(CarboneError::Error(error_msg))
        }
    }
}
