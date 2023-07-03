use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

use crate::render::RenderId;
use crate::template::TemplateId;

use crate::carbone::Result;

// #[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    pub success: bool,
    #[serde(default)]
    pub data: Option<HashMap<String, String>>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

///
/// On succes (when uploading a template or render data) the Carbone Service delivers two
/// responses which can contain a template_id or a render_id:
///
/// {
///     "success": true,
///         "data": {
///             "templateId": "2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad"
///     }
/// }
///
/// {
///     "success": true,
///         "data": {
///             "renderId": "MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt"
///     }
/// }
///
/// On Failure the Carbone Service responds with the following json:
///
/// {
///     "success": false,
///     "error": "<error message>"
//  }
///
///  or
///
/// {
///     "success": false,
///     "error": "Invalid or undefined TemplateId or RenderId in the URL",
///     "code": "w115"
/// }
///
///
impl ResponseBody {
    pub fn new(
        success: bool,
        data: Option<HashMap<String, String>>,
        error: Option<String>,
        code: Option<String>,
    ) -> Self {
        Self {
            success,
            data,
            error,
            code,
        }
    }

    pub fn get_template_id(&self) -> Result<TemplateId> {
        let render_id = self.get_id_from_data("templateId".to_string());
        TemplateId::new(render_id)
    }

    pub fn get_render_id(&self) -> Result<RenderId> {
        let render_id = self.get_id_from_data("renderId".to_string());
        RenderId::new(render_id)
    }

    pub fn get_error_message(&self) -> String {
        match self.error.clone() {
            Some(error_msg) => error_msg,
            None => "".to_string(),
        }
    }

    pub fn get_error_code(&self) -> String {
        match self.code.clone() {
            Some(error_code) => error_code,
            None => "".to_string(),
        }
    }

    fn get_id_from_data(&self, k: String) -> String {
        match self.data.clone() {
            Some(values) => {
                if let Some(value) = values.get(k.as_str()) {
                    value.clone()
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        }
    }
}
