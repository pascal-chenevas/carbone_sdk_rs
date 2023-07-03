use std::fs;

use httpmock::prelude::*;
use serde_json::json;

use carbone_sdk_rs::carbone::Carbone;
use carbone_sdk_rs::errors::CarboneError;
use carbone_sdk_rs::render::*;
use carbone_sdk_rs::carbone_response::ResponseBody  ;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use carbone_sdk_rs::template::{Template, TemplateFile};

    use super::*;

    #[test]
    fn test_get_report() -> Result<(), CarboneError> {
        
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = &helper.create_config_for_mock_server(Some(&server))?;
        let api_token = &helper.create_api_token()?;

        let carbone = Carbone::new(&config, api_token)?;

        let render_id_value = "844318fe97904fb0897d4b0a47fbe9bbd1ce5c9624ae694545cbc1877f581d86.pdf";
        let render_id = &RenderId::new(render_id_value.to_string())?;

        let rendered_file_content = fs::read("tests/data/report.pdf")?;

        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(200)
                .body(rendered_file_content.clone());
        });

        let report_content = carbone.get_report(render_id)?;
    
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

        let carbone = Carbone::new(&config, api_token)?;

        let render_id_value = "unknown_render_id.pdf";
        let render_id = &RenderId::new(render_id_value.to_string())?;

        let error_msg = "Invalid or undefined TemplateId or RenderId in the URL".to_string();

        let body = ResponseBody
        {
            success: false,
            data: None,
            error: Some(error_msg.clone()),
            code: Some("w115".to_string())
        };

        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", render_id.as_str()));
            then.status(400)
                .header("content-type", "application/json; charset=utf-8")
                .json_body_obj(&body);
        });

        let expected_error = CarboneError::BadRequest(error_msg);
        let result = carbone.get_report(render_id);
    
        mock_server.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }
    #[test]
    fn test_generate_report_with_template_id() -> Result<(), CarboneError> {
        
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let config = &helper.create_config_for_mock_server(Some(&server))?;
        let api_token = &helper.create_api_token()?;

        let carbone = Carbone::new(&config, api_token)?;

        let template = Template::new(&config, &api_token);

        let report_data = fs::read_to_string("tests/data/report_data.json")?;

        let template_file = TemplateFile::new("tests/data/template.odt".to_string())?;
        let template_id = template.generate_id(&template_file, "")?;

        let render_options = RenderOptions::new(report_data)?;

        let render_id_value = "MTAuMjAuMjEuNDAgICAgBY4OM11wQg11ekv6_R0n0wcmVwb3J0.pdf".to_string(); 
        let _render_id = &RenderId::new(&render_id_value)?;

        let file_path = "tests/data/report.pdf";

        let expected_content  = fs::read(file_path)?;

        let mock_render_response = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200)
                .json_body(json!({
                    "success": true,
                    "data": {
                        "renderId": &render_id_value,
                        "inputFileExtension": "odt"
                    }
                }));
        });
       

        let mock_get_report_response = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/render/{}", &render_id_value));
            then.status(200)
                .body(&expected_content);
        });

        let result = carbone.generate_report_with_template_id(template_id, render_options)?;

        mock_render_response.assert();
        mock_get_report_response.assert();

        assert_eq!(result, expected_content);
       
        Ok(())
    }

}