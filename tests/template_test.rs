use std::collections::HashMap;
use httpmock::prelude::*;

use carbone_sdk_rs::carbone_response::CarboneSDKResponse;
use carbone_sdk_rs::errors::CarboneSdkError;
use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::types::ApiJsonToken;
use carbone_sdk_rs::template::Template;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    const TOKEN_TEST: &str = "test_32u1i3ui1212334395349dsaowe912384ads89de8e93hj123iowa21085dsaowe91843784p213894dsa912384ads89de8e93hj123iowa210309dhsudausdasda72q37q783hy3243829434gdgadghdsaowe912384ads89de8e93hj1owa21023113i12u32i1321io39534985dsaowe9123843784p213894309dhsudausdasda72q37q783h43784p213894309dhsuda4gdgadghdsaow2384ads89de8e93hj123iowa21023113i12u32i1321io39534985dsa";

    fn create_config_for_mock_server(server: Option<&MockServer>) -> Result<Config, CarboneSdkError> {

        let port = match server {
            Some(s) => s.port(),
            None => 8080
        };

        let config = Config::new(
            format!("{}{}", "http://127.0.0.1:", port), // port changes each run when used with the MockServer
            4,
            2
        )?;
        Ok(config)
    }

    fn create_api_token() -> Result<ApiJsonToken> {
        let api_token = ApiJsonToken::new(TOKEN_TEST.to_string())?;
        Ok(api_token)
    }

    #[test]
    fn test_generate_template_id_odt_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let file_name = "tests/template.test.odt".to_string();
        let template_id = template.generate_id(&file_name, "")?;

        let expected_template_id = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string();
        assert_eq!(expected_template_id, template_id);
        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_2_payload_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let file_name = "tests/template.test.odt".to_string();
        let template_id = template.generate_id(&file_name, "ThisIsAPayload")?;

        let expected_template_id = "7de8d1d8676abb32291ea5119cb1f78fe37fdfdc75332fcdae28f1e30d064ac0".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_3_payload_2() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let file_name = "tests/template.test.odt".to_string();
        let template_id = template.generate_id(&file_name, "8B5PmafbjdRqHuksjHNw83mvPiGj7WTE")?;

        let expected_template_id = "a62eb407a5d5765ddf974636de8ab47bda7915cebd61197d7a2bb42ae70ffcd6".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())
    }


    #[test]
    fn test_generate_template_id_html_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let file_name = "tests/template.test.html".to_string();
        let template_id = template.generate_id(&file_name, "")?;

        let expected_template_id = "75256dd5c260cdf039ae807d3a007e78791e2d8963ea1aa6aff87ba03074df7f".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())

    }  

    #[test]
    fn test_generate_template_id_html_2_payload_1() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let file_name = "tests/template.test.html".to_string();
        let payload = "This is a long payload with different characters 1 *5 &*9 %$ 3%&@9 @(( 3992288282 29299 9299929";
        let template_id = template.generate_id(&file_name, payload)?;

        let expected_template_id = "70799b421cc9cf75d9112273a8e054c141d484eb8d5988bd006fac83e3990707".to_string();
        assert_eq!(expected_template_id, template_id);
        
        Ok(())

    }  

    #[test]
    fn test_generate_template_id_error() -> Result<(), CarboneSdkError> {

        let config: Config = Default::default();
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let file_name = "".to_string();
        let payload = "";
       
        let error = match template.generate_id(&file_name, payload) {
            Ok(template_id) => template_id,
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingTemplateFileName.to_string(); 

        assert_eq!(expected_error.to_string(), error);

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
        let m = server.mock(|when, then| {
            when.method("POST")
                .path("/template");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&body);
        });

        let config = create_config_for_mock_server(Some(&server))?;
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let template_file = String::from("template.odt");
        let template_id = template.upload(&template_file, "".to_string())?;
        
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
    
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let template_file = String::from("template.odt");
        let template_id = template.upload(&template_file, "salt1234".to_string())?;

        // Assert
        m.assert();
        assert_eq!(template_id_expected,template_id);

        Ok(())
    }

    #[test]
    fn template_file_name() -> Result<(), CarboneSdkError> {

        let config = create_config_for_mock_server(None)?;

        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let template_file = String::from("");

        let result = match template.upload(&template_file, "".to_string()) {
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
        
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let template_file = String::from("/wrong/path/to/template.odt");

        let result = match template.upload(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::FileNotFound("/wrong/path/to/template.odt".to_string()); 
        
        assert_eq!(expected_error.to_string(), result);

        Ok(())
    }

    #[test]
    fn test_add_template_error_with_directory() -> Result<(), CarboneSdkError> {

        let config = create_config_for_mock_server(None)?;
        
        let api_token = create_api_token()?;
        let template = Template::new(config, api_token);

        let template_file = String::from("tests");

        let result = match template.upload(&template_file, "".to_string()) {
            Ok(_) => panic!("the function doesn't return an error"),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::IsADirectory("tests".to_string()); 
        
        assert_eq!(expected_error.to_string(), result);

        Ok(())
    }

}