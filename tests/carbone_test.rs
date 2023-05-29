use std::collections::HashMap;
use httpmock::prelude::*;

use carbone_rs::carbone_sdk::carbone_response::CarboneSDKResponse;
use carbone_rs::carbone_sdk::carbone::CarboneSDK;
use carbone_rs::carbone_sdk::errors::CarboneSdkError;
use carbone_rs::carbone_sdk::config::Config;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_sdk_error_missing_token() -> Result<(), CarboneSdkError> {

        let config = Default::default();
    
        let error = match CarboneSDK::new(config) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingApiToken("CarboneSDK::new()".to_string()); 
        
        assert_eq!(expected_error.to_string(), error);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_1() -> Result<(), CarboneSdkError> {

        let mut config: Config = Default::default();
        config.api_token = "test_a".to_string();
        let cabone_sdk = CarboneSDK::new(config)?;

        let file_name = "tests/template.test.odt".to_string();
        let template_id = cabone_sdk.generate_template_id(&file_name, "")?;

        let expected_template_id = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string();
        assert_eq!(expected_template_id, template_id);
        Ok(())

    }

    #[test]
    fn test_add_template() -> Result<(), CarboneSdkError> {

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
        let m = server.mock(|when, then| {
            when.method("POST")
                .path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: format!("{}{}", "http://127.0.0.1:", server.port()), // port change each run
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config)?;

        let template_file = String::from("template.odt");
        let template_id = carbone_sdk.add_template(&template_file, "".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn test_add_template_with_payload() -> Result<(), CarboneSdkError> {

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

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: format!("{}{}", "http://127.0.0.1:", server.port()), // port change each run
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config)?;

        let template_file = String::from("template.odt");
        let template_id = carbone_sdk.add_template(&template_file, "salt1234".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn template_file_name() -> Result<(), CarboneSdkError> {

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: "http://127.0.0.1".to_string(),
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config)?;

        let template_file = String::from("");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingArgument("add_template".to_string(), "template_file_name".to_string());    
       
        assert_eq!(expected_error.to_string(), result);

        Ok(())
    }

    #[test]
    fn test_add_template_error_with_a_non_existing_file() -> Result<(), CarboneSdkError> {

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: "http://127.0.0.1".to_string(),
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config)?;

        let template_file = String::from("/wrong/path/to/template.odt");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::FileNotFound("add_template".to_string(), "/wrong/path/to/template.odt".to_string()); 
        
        assert_eq!(expected_error.to_string(), result);

        Ok(())
    }

    #[test]
    fn test_add_template_error_with_directory() -> Result<(), CarboneSdkError> {

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: "http://127.0.0.1".to_string(),
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config)?;

        let template_file = String::from("tests");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::IsADirectory("add_template".to_string(), "tests".to_string()); 
        
        assert_eq!(expected_error.to_string(), result);

        Ok(())
    }
}