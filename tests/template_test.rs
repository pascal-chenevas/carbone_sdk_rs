use std::fs;

use std::collections::HashMap;
use httpmock::prelude::*;

use carbone_sdk_rs::carbone_response::CarboneSDKResponse;
use carbone_sdk_rs::errors::CarboneSdkError;
use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::template::Template;

mod helper;

use helper::Helper;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use carbone_sdk_rs::template::TemplateId;

    #[test]
    fn test_template_id() -> Result<(), CarboneSdkError> {

        let template_id_value = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114";
        let template_id = TemplateId::new(template_id_value.to_string())?;

        assert_eq!(template_id.as_str(), template_id_value);

        Ok(())
    }

    #[test]
    fn test_template_id_empty_value_given() -> Result<(), CarboneSdkError> {

        let template_id_value = "";
        let result = TemplateId::new(template_id_value.to_string());

        let exepected_error = CarboneSdkError::EmptyString("template_id".to_string());
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), exepected_error.to_string());

        Ok(())
    }


    #[test]
    fn test_downaload() -> Result<(), CarboneSdkError> {

        let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;

        let template_file_content = fs::read("tests/data/template.test.odt")?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200)
                .body(template_file_content.clone());
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
    fn test_downaload_unknown_template_id_given() -> Result<(), CarboneSdkError> {

        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        let error_msg = "Error: Cannot remove template, does it exist ?".to_string();

        let body = CarboneSDKResponse{
            success: false,
            data: None,
            error: Some(error_msg.clone()),
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("GET")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(config, api_token);

        let result = template.download(template_id);

        let expected_error = CarboneSdkError::ResponseError(error_msg);

        mock_server.assert();

        assert!(result.is_err());
        assert_eq!(expected_error.to_string(), result.unwrap_err().to_string());

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "tests/data/template.test.odt".to_string();
        let template_id = template.generate_id(&file_name, "")?;

        let expected_template_id = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string();
        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_2_payload_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "tests/data/template.test.odt".to_string();
        let template_id = template.generate_id(&file_name, "ThisIsAPayload")?;

        let expected_template_id = "7de8d1d8676abb32291ea5119cb1f78fe37fdfdc75332fcdae28f1e30d064ac0".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_3_payload_2() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "tests/data/template.test.odt".to_string();
        let template_id = template.generate_id(&file_name, "8B5PmafbjdRqHuksjHNw83mvPiGj7WTE")?;

        let expected_template_id = "a62eb407a5d5765ddf974636de8ab47bda7915cebd61197d7a2bb42ae70ffcd6".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())
    }


    #[test]
    fn test_generate_template_id_html_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "tests/data/template.test.html".to_string();
        let template_id = template.generate_id(&file_name, "")?;

        let expected_template_id = "75256dd5c260cdf039ae807d3a007e78791e2d8963ea1aa6aff87ba03074df7f".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())

    }  

    #[test]
    fn test_generate_template_id_html_2_payload_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "tests/data/template.test.html".to_string();
        let payload = "This is a long payload with different characters 1 *5 &*9 %$ 3%&@9 @(( 3992288282 29299 9299929";
        let template_id = template.generate_id(&file_name, payload)?;

        let expected_template_id = "70799b421cc9cf75d9112273a8e054c141d484eb8d5988bd006fac83e3990707".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())

    }  

    #[test]
    fn test_generate_template_id_error() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let helper = Helper::new();
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(&config, api_token);

        let file_name = "".to_string();
        let payload = "";
       
        let result = template.generate_id(&file_name, payload);

        let expected_error = CarboneSdkError::MissingTemplateFileName.to_string(); 

        assert!(result.is_err());
        assert_eq!(expected_error.to_string(), result.unwrap_err().to_string());

        Ok(())

    }  

    #[test]
    fn test_upload_template() -> Result<(), CarboneSdkError> {

        let mut data = HashMap::new();
        let template_id_expected = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string();
        data.insert("templateId".to_string(), template_id_expected.clone());

        let body = CarboneSDKResponse{
            success: true,
            data: Some(data),
            error: None
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock_server: httpmock::Mock = server.mock(|when, then| {
            when.method("POST")
                .path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;
    
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(config, api_token);

        let template_file = String::from("tests/data/template.odt");
        let template_id = template.upload(&template_file, "".to_string())?;
        
        // Assert
        mock_server.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn test_upload_template_with_payload() -> Result<(), CarboneSdkError> {

        let mut data = HashMap::new();
        let template_id_expected = "cb03f7676ef0fbe5d7824a64676166ac2c7c789d9e6da5b7c0c46794911ee7a7".to_string();
        data.insert("templateId".to_string(), template_id_expected.clone());

        let body = CarboneSDKResponse{
            success: true,
            data: Some(data),
            error: None
        };

        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let m = server.mock(|when, then| {
            when.method("POST")
                .path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;
    
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(config, api_token);

        let template_file = String::from("tests/data/template.odt");
        let template_id = template.upload(&template_file, "salt1234".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn test_upload_template_template_file_path_not_given() -> Result<(), CarboneSdkError> {

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(None)?;

        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(config, api_token);

        let template_file = String::from("");

        let error = template.upload(&template_file, "".to_string());
       
        assert!(error.is_err());
        assert_eq!(CarboneSdkError::MissingTemplateFileName.to_string(), error.unwrap_err().to_string());

        Ok(())
    }

    #[test]
    fn test_upload_template_error_with_a_non_existing_file() -> Result<(), CarboneSdkError> {

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(None)?;
        
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(config, api_token);

        let template_file = String::from("/wrong/path/to/template.odt");

        let result = template.upload(&template_file, "".to_string());

        let expected_error = CarboneSdkError::FileNotFound("/wrong/path/to/template.odt".to_string()); 
        
        assert!(result.is_err());
        assert_eq!(expected_error.to_string(), result.unwrap_err().to_string());

        Ok(())
    }

    #[test]
    fn test_upload_template_error_with_directory() -> Result<(), CarboneSdkError> {

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(None)?;
        
        let api_token = &helper.create_api_token()?;
        let template: Template = Template::new(config, api_token);

        let template_file = String::from("tests");

        let result = template.upload(&template_file, "".to_string());

        let expected_error = CarboneSdkError::IsADirectory("tests".to_string()); 
        
        assert!(result.is_err());
        assert_eq!(expected_error.to_string(), result.unwrap_err().to_string());

        Ok(())
    }

    #[test]
    fn test_delete() -> Result<(), CarboneSdkError> {

        let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let body = CarboneSDKResponse{
            success: true,
            data: None,
            error: None,
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("DELETE")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200)
                .json_body_obj(&body);
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
    fn test_delete_unknown_template_id_given() -> Result<(), CarboneSdkError> {

        let template_id = TemplateId::new("unknown_template_id".to_string())?;

        // Start a lightweight mock server.
        let server = MockServer::start();

        let error_msg = "ResponseError: Cannot remove template, does it exist ?".to_string();

        let body = CarboneSDKResponse{
            success: false,
            data: None,
            error: Some(error_msg.clone()),
        };

        // Create a mock on the server.
        let mock_server = server.mock(|when, then| {
            when.method("DELETE")
                .path(format!("/template/{}", template_id.as_str()));
            then.status(200)
                .json_body_obj(&body);
        });

        let helper = Helper::new();
        let config = &helper.create_config_for_mock_server(Some(&server))?;

        let api_token = &helper.create_api_token()?;

        let template: Template = Template::new(config, api_token);

        let result = template.delete(template_id);

        let expected_error = CarboneSdkError::ResponseError(error_msg);

        mock_server.assert();

        assert!(result.is_err());
        assert_eq!(expected_error.to_string(), result.unwrap_err().to_string());

        Ok(())
    }

}