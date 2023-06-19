use std::fs;

use httpmock::prelude::*;

use carbone_sdk_rs::carbone::Carbone;
use carbone_sdk_rs::errors::CarboneSdkError;
use carbone_sdk_rs::render::*;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_report() -> Result<(), CarboneSdkError> {
        
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

}