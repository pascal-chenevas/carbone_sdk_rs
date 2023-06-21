use std::fs;

use httpmock::prelude::*;

use carbone_sdk_rs::carbone::Carbone;
use carbone_sdk_rs::errors::CarboneError;
use carbone_sdk_rs::render::*;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_report() -> Result<(), CarboneError> {
        
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = &helper.create_config_for_mock_server(Some(&server))?;
        let api_token = &helper.create_api_token()?;

        let carbone_sdk = Carbone::new(&config, api_token)?;

        let render_id_value = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114.pdf";
        let render_id = &RenderId::new(render_id_value.to_string())?;

        let rendered_file_content = fs::read("tests/data/report.pdf")?;

        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(200)
                .body(rendered_file_content.clone());
        });

        let report_content = carbone_sdk.get_report(render_id)?;
    
        mock_server.assert();
        assert_eq!(report_content, rendered_file_content.to_vec());

        Ok(())
    }

    #[test]
    fn test_get_report_unknown_render_id_given() -> Result<(), CarboneError> {
        
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = &helper.create_config_for_mock_server(Some(&server))?;
        let api_token = &helper.create_api_token()?;

        let carbone_sdk = Carbone::new(&config, api_token)?;

        let render_id_value = "unknown_render_id.pdf";
        let render_id = &RenderId::new(render_id_value.to_string())?;

        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(404);
        });

        let expected_error = CarboneError::RenderIdNotFound(render_id_value.to_string());
        let result = carbone_sdk.get_report(render_id);
    
        mock_server.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

}