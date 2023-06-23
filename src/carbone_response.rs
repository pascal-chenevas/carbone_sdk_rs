use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::str;

use crate::template::TemplateId;
use crate::render::RenderId;

use crate::carbone::Result;

// #[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct CarboneSDKResponse {
    pub success: bool,
    #[serde(default)]
    pub data: Option<HashMap<String, String>>,
    #[serde(default)]
    pub error: Option<String>,
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
impl CarboneSDKResponse {

    pub fn new(success: bool, data: Option<HashMap<String, String>>, error: Option<String>) -> Self {
        Self { success, data, error }
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
