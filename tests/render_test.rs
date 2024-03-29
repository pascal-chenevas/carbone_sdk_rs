use carbone_sdk_rs::errors::CarboneError;
use carbone_sdk_rs::render::*;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_render_id() -> Result<(), CarboneError> {
        let render_id_value = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114";
        let render_id = RenderId::new(render_id_value.to_string())?;

        assert_eq!(render_id.as_str(), render_id_value);

        Ok(())
    }

    #[test]
    fn test_render_id_empty_value_given() -> Result<(), CarboneError> {
        let render_id_value = "";
        let result = RenderId::new(render_id_value.to_string());

        let exepected_error = CarboneError::EmptyString("render_id".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), exepected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_render_id_as_ref() -> Result<(), CarboneError> {
        let id_value = "1";
        let id = RenderId::new(id_value.to_string())?;

        assert_eq!(id.as_ref(), id_value);

        Ok(())
    }
}
