use std::collections::HashMap;
use httpmock::prelude::*;

use carbone_sdk_rs::carbone_response::CarboneSDKResponse;
use carbone_sdk_rs::carbone::CarboneSDK;
use carbone_sdk_rs::errors::CarboneSdkError;
use carbone_sdk_rs::config::Config;

#[cfg(test)]
mod tests {

    use super::*;

    fn create_default_config() -> Result<Config, CarboneSdkError> {
        let mut config: Config = Default::default();
        config.set_api_token("test_token".to_string())?;
        Ok(config)
    }

    fn create_config_for_mock_server(server: Option<&MockServer>) -> Result<Config, CarboneSdkError> {

        let port = match server {
            Some(s) => s.port(),
            None => 8080
        };

        let config = Config::new(
            "test_token".to_string(),
            format!("{}{}", "http://127.0.0.1:", port), // port changes each run when used with the MockServer
            4,
            "2".to_string()
        )?;
        Ok(config)
    }


    #[test]
    fn test_generate_template_id_odt_1() -> Result<(), CarboneSdkError> {

        let config = create_default_config()?;
        let cabone_sdk = CarboneSDK::new(&config)?;

        let file_name = "tests/template.test.odt".to_string();
        let template_id = cabone_sdk.generate_template_id(&file_name, "")?;

        let expected_template_id = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string();
        assert_eq!(expected_template_id, template_id);
        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_2_payload_1() -> Result<(), CarboneSdkError> {

        let config = create_default_config()?;
        let cabone_sdk = CarboneSDK::new(&config)?;

        let file_name = "tests/template.test.odt".to_string();
        let template_id = cabone_sdk.generate_template_id(&file_name, "ThisIsAPayload")?;

        let expected_template_id = "7de8d1d8676abb32291ea5119cb1f78fe37fdfdc75332fcdae28f1e30d064ac0".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_3_payload_2() -> Result<(), CarboneSdkError> {

        let config = create_default_config()?;
        let cabone_sdk = CarboneSDK::new(&config)?;

        let file_name = "tests/template.test.odt".to_string();
        let template_id = cabone_sdk.generate_template_id(&file_name, "8B5PmafbjdRqHuksjHNw83mvPiGj7WTE")?;

        let expected_template_id = "a62eb407a5d5765ddf974636de8ab47bda7915cebd61197d7a2bb42ae70ffcd6".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())
    }


    #[test]
    fn test_generate_template_id_html_1() -> Result<(), CarboneSdkError> {

        let config = create_default_config()?;
        let cabone_sdk = CarboneSDK::new(&config)?;

        let file_name = "tests/template.test.html".to_string();
        let template_id = cabone_sdk.generate_template_id(&file_name, "")?;

        let expected_template_id = "75256dd5c260cdf039ae807d3a007e78791e2d8963ea1aa6aff87ba03074df7f".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())

    }  

    #[test]
    fn test_generate_template_id_html_2_payload_1() -> Result<(), CarboneSdkError> {

        let config = create_default_config()?;
        let cabone_sdk = CarboneSDK::new(&config)?;

        let file_name = "tests/template.test.html".to_string();
        let payload = "This is a long payload with different characters 1 *5 &*9 %$ 3%&@9 @(( 3992288282 29299 9299929";
        let template_id = cabone_sdk.generate_template_id(&file_name, payload)?;

        let expected_template_id = "70799b421cc9cf75d9112273a8e054c141d484eb8d5988bd006fac83e3990707".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())

    }  

    #[test]
    fn test_generate_template_id_error() -> Result<(), CarboneSdkError> {

        let config = create_default_config()?;
        let cabone_sdk = CarboneSDK::new(&config)?;

        let file_name = "".to_string();
        let payload = "";
       
        let error = match cabone_sdk.generate_template_id(&file_name, payload) {
            Ok(template_id) => template_id,
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingTemplateFileName.to_string(); 

        assert_eq!(expected_error.to_string(), error);

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

        let config = create_config_for_mock_server(Some(&server))?;
    
        let carbone_sdk = CarboneSDK::new(&config)?;

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

        let config = create_config_for_mock_server(Some(&server))?;
    
        let carbone_sdk = CarboneSDK::new(&config)?;

        let template_file = String::from("template.odt");
        let template_id = carbone_sdk.add_template(&template_file, "salt1234".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn template_file_name() -> Result<(), CarboneSdkError> {

        let config = create_config_for_mock_server(None)?;
        let carbone_sdk = CarboneSDK::new(&config)?;

        let template_file = String::from("");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(template_id) => template_id,
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingTemplateFileName.to_string();    
       
        assert_eq!(expected_error.to_string(), result);

        Ok(())
    }

    #[test]
    fn test_add_template_error_with_a_non_existing_file() -> Result<(), CarboneSdkError> {

        let config = create_config_for_mock_server(None)?;
        let carbone_sdk = CarboneSDK::new(&config)?;

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

        let config = create_config_for_mock_server(None)?;
        let carbone_sdk = CarboneSDK::new(&config)?;

        let template_file = String::from("tests");

        let result = match carbone_sdk.add_template(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::IsADirectory("tests".to_string()); 
        
        assert_eq!(expected_error.to_string(), result);

        Ok(())
    }

    #[test]
    fn test_get_report_error_missing_render_id() -> Result<(), CarboneSdkError> {
        
        let config = create_config_for_mock_server(None)?;
        let carbone_sdk = CarboneSDK::new(&config)?;

        let error = match carbone_sdk.get_report(&"".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string()
        };

        let expected_error = CarboneSdkError::MissingRenderId.to_string(); 
        
        assert_eq!(expected_error.to_string(), error);

        Ok(())
    }

}