use std::fs;

use httpmock::prelude::*;
use std::collections::HashMap;

use carbone_sdk_rs::carbone_response::ResponseBody;
use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::errors::CarboneError;
use carbone_sdk_rs::template::*;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use carbone_sdk_rs::template::TemplateId;

    #[test]
    fn test_template_file() -> Result<(), CarboneError> {
        let template_file_path = "tests/data/template.test.odt";
        let template_file = TemplateFile::new(template_file_path.to_string())?;

        assert_eq!(template_file.path_as_str(), template_file_path);

        Ok(())
    }

    #[test]
    fn test_template_file_directory_given() -> Result<(), CarboneError> {
        let template_file_path = "tests/data/";
        let result = TemplateFile::new(template_file_path.to_string());

        let expected_error = CarboneError::IsADirectory(template_file_path.to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_template_file_not_exists_given() -> Result<(), CarboneError> {
        let template_file_path = "tests/data/unknown_template.test.docx";
        let result = TemplateFile::new(template_file_path.to_string());

        let expected_error = CarboneError::TemplateFileNotFound(template_file_path.to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_downaload() -> Result<(), CarboneError> {
        let template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        let template_file_content = fs::read("tests/data/template.test.odt")?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200).body(template_file_content.clone());
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(&config, &api_token);

        let template_content = template.download(template_id)?;

        mock_server.assert();

        assert_eq!(template_file_content, template_content.to_vec());

        Ok(())
    }

    #[test]
    fn test_downaload_unknown_template_id_given() -> Result<(), CarboneError> {
        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let error_msg = "Invalid or undefined TemplateId or RenderId in the URL".to_string();

        let body = ResponseBody {
            success: false,
            data: None,
            error: Some(error_msg.clone()),
            code: Some("w115".to_string()),
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(400)
                .header("content-type", "application/json; charset=utf-8")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(config, api_token);

        let result = template.download(template_id);

        let expected_error = CarboneError::BadRequest(error_msg);

        mock_server.assert();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_1() -> Result<(), CarboneError> {
        let config: Config = Default::default();

        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let template_file_path = "tests/data/template.test.odt".to_string();
        let template_file = TemplateFile::new(template_file_path.to_string())?;
        let template_id = template.generate_id(&template_file, "")?;

        let expected_template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_2_payload_1() -> Result<(), CarboneError> {
        let config: Config = Default::default();

        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "tests/data/template.test.odt".to_string();
        let template_file = TemplateFile::new(file_name.to_string())?;
        let template_id = template.generate_id(&template_file, "ThisIsAPayload")?;

        let expected_template_id = TemplateId::new(
            "7de8d1d8676abb32291ea5119cb1f78fe37fdfdc75332fcdae28f1e30d064ac0".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_3_payload_2() -> Result<(), CarboneError> {
        let config: Config = Default::default();

        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "tests/data/template.test.odt".to_string();
        let template_file = TemplateFile::new(file_name.to_string())?;
        let template_id =
            template.generate_id(&template_file, "8B5PmafbjdRqHuksjHNw83mvPiGj7WTE")?;

        let expected_template_id = TemplateId::new(
            "a62eb407a5d5765ddf974636de8ab47bda7915cebd61197d7a2bb42ae70ffcd6".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_html_1() -> Result<(), CarboneError> {
        let config: Config = Default::default();

        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let template_file = TemplateFile::new("tests/data/template.test.html".to_string())?;
        let template_id = template.generate_id(&template_file, "")?;

        let expected_template_id = TemplateId::new(
            "75256dd5c260cdf039ae807d3a007e78791e2d8963ea1aa6aff87ba03074df7f".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_html_2_payload_1() -> Result<(), CarboneError> {
        let config: Config = Default::default();

        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let template_file = TemplateFile::new("tests/data/template.test.html".to_string())?;
        let payload = "This is a long payload with different characters 1 *5 &*9 %$ 3%&@9 @(( 3992288282 29299 9299929";
        let template_id = template.generate_id(&template_file, payload)?;

        let expected_template_id = TemplateId::new(
            "70799b421cc9cf75d9112273a8e054c141d484eb8d5988bd006fac83e3990707".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_upload_template() -> Result<(), CarboneError> {
        let mut data = HashMap::new();
        let template_id_expected = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;
        data.insert(
            "templateId".to_string(),
            template_id_expected.as_str().to_string(),
        );

        let body = ResponseBody {
            success: true,
            data: Some(data),
            error: None,
            code: None,
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server: httpmock::Mock = server.mock(|when, then| {
            when.method("POST").path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(config, api_token);
        let template_file = TemplateFile::new("tests/data/template.odt".to_string())?;
        let template_id = template.upload(&template_file, "".to_string())?;

        // Assert
        mock_server.assert();
        assert_eq!(template_id, template_id_expected);

        Ok(())
    }

    #[test]
    fn test_upload_template_with_payload() -> Result<(), CarboneError> {
        let mut data = HashMap::new();
        let template_id_expected = TemplateId::new(
            "cb03f7676ef0fbe5d7824a64676166ac2c7c789d9e6da5b7c0c46794911ee7a7".to_string(),
        )?;
        data.insert(
            "templateId".to_string(),
            template_id_expected.as_str().to_string(),
        );

        let body = ResponseBody {
            success: true,
            data: Some(data),
            error: None,
            code: None,
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let m = server.mock(|when, then| {
            when.method("POST").path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(config, api_token);
        let template_file = TemplateFile::new("tests/data/template.odt".to_string())?;
        let template_id = template.upload(&template_file, "salt1234".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id, template_id_expected);

        Ok(())
    }

    #[test]
    fn test_delete() -> Result<(), CarboneError> {
        let template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let body = ResponseBody {
            success: true,
            data: None,
            error: None,
            code: None,
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("DELETE")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200).json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(config, api_token);

        let is_deleted = template.delete(template_id)?;

        mock_server.assert();

        assert_eq!(is_deleted, true);

        Ok(())
    }

    #[test]
    fn test_delete_unknown_template_id_given() -> Result<(), CarboneError> {
        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let error_msg = "Invalid or undefined TemplateId or RenderId in the URL".to_string();

        let body = ResponseBody {
            success: false,
            data: None,
            error: Some(error_msg.clone()),
            code: None,
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("DELETE")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(400)
                .header("content-type", "application/json; charset=utf-8")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(config, api_token);

        let result = template.delete(template_id);

        let expected_error = CarboneError::BadRequest(error_msg);

        mock_server.assert();

        assert!(result.is_err());
        assert_eq!(expected_error.to_string(), result.unwrap_err().to_string());

        Ok(())
    }
}
