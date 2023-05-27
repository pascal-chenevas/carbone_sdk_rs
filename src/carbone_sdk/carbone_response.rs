use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::str;
// #[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct CarboneSDKResponse {
    pub success: bool,
    #[serde(default)]
    //#[serde(with = "date_serde")
    pub data: Option<HashMap<String, String>>,
    #[serde(default)]
    pub error: Option<String>,
}

impl CarboneSDKResponse {
    /**
     * TODO refactoring duplicate code get_template_id/get_render_id
     */
    pub fn get_template_id(&self) -> String {
        match self.data.clone() {
            Some(values) => {
                if let Some(template_id) = values.get("templateId") {
                    template_id.clone()
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        }
    }

    pub fn get_render_id(&self) -> String {
        match self.data.clone() {
            Some(values) => {
                if let Some(render_id) = values.get("renderId") {
                    render_id.clone()
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        }
    }

    pub fn get_error_message(&self) -> String {
        match self.error.clone() {
            Some(error_msg) => error_msg,
            None => "".to_string(),
        }
    }
}
