
use httpmock::prelude::*;

use carbone_sdk_rs::carbone::CarboneSDK;
use carbone_sdk_rs::errors::CarboneSdkError;
use carbone_sdk_rs::config::Config;

#[cfg(test)]
mod tests {

    use super::*;

    const TOKEN_TEST: &str = "test_32u1i3ui121233439534985dsaowe9123843784p213894309dhsudausdasda72q37q783hy3243829434gdgadghads89de8e93hj123iowa21023113i12u32i1321io";

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

    #[test]
    fn test_get_report_error_missing_render_id() -> Result<(), CarboneSdkError> {
        
        let config = create_config_for_mock_server(None)?;
        let carbone_sdk = CarboneSDK::new(&config, TOKEN_TEST.to_string())?;

        let result = carbone_sdk.get_report(&"".to_string());

        let is_err = result.is_err();
        let error = result.unwrap_err().to_string();

        let expected_error = CarboneSdkError::MissingRenderId.to_string(); 
        
        assert!(is_err);
        assert_eq!(expected_error.to_string(), error);

        Ok(())
    }

}