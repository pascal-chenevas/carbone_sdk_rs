use httpmock::prelude::*;

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::errors::CarboneError;
use carbone_sdk_rs::render::*;

use serde_json::json;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use carbone_sdk_rs::template::*;

    #[test]
    fn test_render_options() -> Result<(), CarboneError> {
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
    fn test_render_options_value_not_given() -> Result<(), CarboneError> {
        let render_options = "";
        let result = RenderOptions::new(render_options.to_string());

        let exepected_error = CarboneError::EmptyString("render_options".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), exepected_error.to_string());

        Ok(())
    }

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
    fn test_render_report_with_file() -> Result<(), CarboneError> {
        // Start a lightweight mock server.
        let server = MockServer::start();

        let helper = Helper::new();

        let template_file = &TemplateFile::new("tests/data/template.test.odt".to_string())?;

        let config = helper.create_config_for_mock_server(Some(&server))?;
        let api_token = helper.create_api_token()?;

        let template: Template = Template::new(&config, &api_token);
        let template_id = template.generate_id(&template_file, "")?;

        let expected_render_id =
            RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string())?;

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "renderId": expected_render_id.as_str(),
                    "inputFileExtension": "odt"
                }
            }));
        });

        let render = Render::new(&config, &api_token);

        let render_options = String::from(
            r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#,
        );
        let render_options = RenderOptions::new(render_options)?;

        let render_id = render.render_report_with_file(template_file, render_options, "")?;

        mock_server.assert();
        assert_eq!(render_id, expected_render_id);

        Ok(())
    }

    #[test]
    fn test_render_report_with_template_id() -> Result<(), CarboneError> {
        let helper = Helper::new();

        let template_id = TemplateId::new("foiejwoi21e093ru3209jf2093j".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let expected_render_id =
            RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string())?;
        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(200).json_body(json!({
                "success": true,
                "data": {
                    "renderId": expected_render_id.as_str(),
                    "inputFileExtension": "odt"
                }
            }));
        });

        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let render = Render::new(&config, &api_token);

        let render_options = String::from(
            r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#,
        );

        let render_options = RenderOptions::new(render_options)?;
        let render_id = render.render_report_with_template_id(template_id, render_options)?;

        mock_server.assert();
        assert_eq!(render_id, expected_render_id);

        Ok(())
    }

    #[test]
    fn test_render_report_with_template_id_unknown_template_id_given() -> Result<(), CarboneError> {
        let helper = Helper::new();

        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("POST")
                .path(format!("/render/{}", template_id.as_str()));
            then.status(400).json_body(json!({
                "success": false,
                "error": "Invalid or undefined TemplateId or RenderId in the URL",
                "code": "w115"
            }));
        });

        let config = helper.create_config_for_mock_server(Some(&server))?;

        let api_token = helper.create_api_token()?;

        let render = Render::new(&config, &api_token);

        let render_options = String::from(
            r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#,
        );

        let render_options = RenderOptions::new(render_options)?;
        let result = render.render_report_with_template_id(template_id, render_options);

        let expected_error = CarboneError::Error(
            "Invalid or undefined TemplateId or RenderId in the URL".to_string(),
        );

        mock_server.assert();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_render_report_with_template_id_failed() -> Result<(), CarboneError> {

        let helper = Helper::new();

        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        let config = Config::new("http://bad_url".to_string(), 1, 4)?;
        let api_token = helper.create_api_token()?;

        let render = Render::new(&config, &api_token);

        let render_options = String::from(
            r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#,
        );

        let render_options = RenderOptions::new(render_options)?;
        let result = render.render_report_with_template_id(template_id, render_options);

        assert!(result.is_err());

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
