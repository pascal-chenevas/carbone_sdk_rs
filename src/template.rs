use std::fs;
use std::path::Path;
use std::str;

use std::fs::Metadata;

use std::ops::Deref;

use sha2::{Digest, Sha256};

use serde::{Deserialize, Serialize};

use crate::errors::CarboneError;
use crate::types::*;

use crate::types::Result;

#[derive(Debug, Clone)]
pub struct TemplateFile {
    path: String,
    pub content: Option<Vec<u8>>,
    pub metadata: Metadata,
}

impl TemplateFile {
    pub fn new(path: String, content: Option<Vec<u8>>) -> Result<Self> {
        if Path::new(path.as_str()).is_dir() {
            return Err(CarboneError::IsADirectory(path));
        }

        if !Path::new(path.as_str()).is_file() {
            return Err(CarboneError::TemplateFileNotFound(path));
        }

        let metadata = fs::metadata(path.as_str())?;

        Ok(Self {
            path,
            content,
            metadata,
        })
    }

    pub fn generate_id(&self, payload: Option<&str>) -> Result<TemplateId> {
        let file_content = match self.content.to_owned() {
            Some(c) => c,
            None => fs::read(self.path_as_str())?,
        };

        let mut sha256 = Sha256::new();

        let payload =  payload.unwrap_or("");

        sha256.update(payload);
        sha256.update(file_content);

        // convert [u8] to String
        let result: String = format!("{:X}", sha256.finalize());

        TemplateId::new(result.to_lowercase())
    }

    pub fn path_as_str(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TemplateId(Id);

impl TemplateId {
    /// Create a new template_id.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::template::TemplateId;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    ///
    ///     assert_eq!(template_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new<T: Into<String>>(id: T) -> Result<Self> {
        let id = Id::new(id, "template_id")?;
        Ok(TemplateId(id))
    }
}

impl Deref for TemplateId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for TemplateId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
