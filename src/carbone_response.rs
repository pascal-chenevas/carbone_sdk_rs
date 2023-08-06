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