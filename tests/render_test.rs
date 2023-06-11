use httpmock::prelude::*;

use carbone_sdk_rs::render::*;
use carbone_sdk_rs::errors::CarboneSdkError;

use serde_json::json;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use carbone_sdk_rs::template::*;

    #[test]
    fn test_render_options() -> Result<(), CarboneSdkError> {

        let render_options_value = r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#;

        let render_options = RenderOptions::new(render_options_value.to_string())?;

        assert_eq!(render_options.as_str(), render_options_value);

        Ok(())
    }

    #[test]
    fn test_render_options_value_not_given() -> Result<(), CarboneSdkError> {

        let render_options = "";
        let result = RenderOptions::new(render_options.to_string());

        let exepected_error = CarboneSdkError::EmptyString("render_options".to_string());
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), exepected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_render_report_with_file() -> Result<(), CarboneSdkError> {

        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let template_file_name = "tests/data/template.test.odt".to_string();

        let config = helper.create_config_for_mock_server(Some(&server))?;
        let api_token = helper.create_api_token()?;

        let template: Template = Template::new(&config, &api_token);
        let generated_template_id = template.generate_id(&template_file_name, "")?;
        let template_id = TemplateId::new(generated_template_id)?;

        let expected_render_id = "MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200)
                .json_body(json!({
                    "success": true,
                    "data": {
                        "renderId": expected_render_id.clone(),
                        "inputFileExtension": "odt"
                    }
                }));
        });

        let render = Render::new(&config, &api_token);
        
        let render_options = String::from(r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#);
        let render_options = RenderOptions::new(render_options)?;

        let render_id = render.render_report_with_file(template_file_name, render_options, "")?;

        mock_server.assert();
        assert_eq!(render_id, expected_render_id);

        Ok(())
    }

    #[test]
    fn test_render_report_with_template_id() -> Result<(), CarboneSdkError> {

        let helper = Helper::new();

        let template_id = TemplateId::new("foiejwoi21e093ru3209jf2093j".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let expected_render_id = "MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string();
        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200)
                .json_body(json!({
                    "success": true,
                    "data": {
                        "renderId": expected_render_id.clone(),
                        "inputFileExtension": "odt"
                    }
                }));
        });

        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let render = Render::new(&config, &api_token);
        
        let render_options = String::from(r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#);

        let render_options = RenderOptions::new(render_options)?;
        let render_id = render.render_report_with_template_id(template_id, render_options)?;

        mock_server.assert();
        assert_eq!(render_id, expected_render_id);

        Ok(())
    }
    
}