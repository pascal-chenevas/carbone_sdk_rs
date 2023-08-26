use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::types::*;

use crate::types::Result;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RenderId(Id);

impl RenderId {
    /// Create a new render_id struct.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::env;
    ///
    /// use carbone_sdk_rs::render::RenderId;
    /// use carbone_sdk_rs::errors::CarboneError;
    ///
    /// fn main() -> Result<(), CarboneError> {
    ///    
    ///     let render_id = RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string())?;
    ///
    ///     assert_eq!(render_id.as_str().is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new<T: Into<String>>(id: T) -> Result<Self> {
        let id = Id::new(id, "render_id")?;
        Ok(RenderId(id))
    }
}

impl Deref for RenderId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for RenderId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
