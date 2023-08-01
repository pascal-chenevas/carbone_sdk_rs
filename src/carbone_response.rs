use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use std::str;

use crate::render::RenderId;
use crate::template::TemplateId;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct APIResponseData {
    #[serde(default)]
    pub template_id: Option<TemplateId>,
    #[serde(default)]
    pub render_id: Option<RenderId>,
    #[serde(default)]
    pub template_file_extension: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct APIResponse {
    pub success: bool,
    pub data: Option<APIResponseData>,
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
impl APIResponse {
    pub fn new(
        success: bool,
        data: Option<APIResponseData>,
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
}
