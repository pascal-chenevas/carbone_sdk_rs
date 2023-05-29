use std::collections::HashMap;

use httpmock::prelude::*;

use carbone_rs::carbone_sdk::carbone_response::CarboneSDKResponse;
use carbone_rs::carbone_sdk::carbone::CarboneSDK;
use carbone_rs::carbone_sdk::errors::CarboneSdkError;
use carbone_rs::carbone_sdk::config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use carbone_rs::carbone_sdk::carbone::CARBONE_API_URL;

    #[test]
    fn test_default() {

        let carbone_sdk: CarboneSDK = Default::default();

        assert_eq!(carbone_sdk.config.api_timeout, 60);
        assert_eq!(carbone_sdk.config.api_url, CARBONE_API_URL.to_string());
        assert_eq!(carbone_sdk.config.api_token.is_empty(), true);
        assert_eq!(carbone_sdk.config.api_version, "4".to_string());
    }

    #[test]
    fn test_add_template() -> Result<(), CarboneSdkError>{

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
    
        let carbone_sdk = CarboneSDK::new(config);

        let template_file = String::from("template.odt");
        let template_id = carbone_sdk.add_template(&template_file, "".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn test_add_template_with_payload() -> Result<(), CarboneSdkError>{

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
    
        let carbone_sdk = CarboneSDK::new(config);

        let template_file = String::from("template.odt");
        let template_id = carbone_sdk.add_template(&template_file, "salt1234".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn template_file_name() {

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: "".to_string(),
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config);

        let template_file = String::from("");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingArgument("add_template".to_string(), "template_file_name".to_string());    
       
        assert_eq!(expected_error.to_string(), result);
    }

    #[test]
    fn test_add_template_error_with_a_non_existing_file() {

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: "".to_string(),
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config);

        let template_file = String::from("/wrong/path/to/template.odt");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::FileNotFound("add_template".to_string(), "/wrong/path/to/template.odt".to_string()); 
        
        assert_eq!(expected_error.to_string(), result);
    }

    #[test]
    fn test_add_template_error_with_directory() {

        let config = Config{
            api_token: "test_q".to_string(),
            api_url: "".to_string(),
            api_timeout: 4,
            api_version: "2".to_string(),
        };
    
        let carbone_sdk = CarboneSDK::new(config);

        let template_file = String::from("tests");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::IsADirectory("add_template".to_string(), "tests".to_string()); 
        
        assert_eq!(expected_error.to_string(), result);
    }
}