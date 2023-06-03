use carbone_rs::carbone_sdk::config::Config;
use carbone_rs::carbone_sdk::errors::CarboneSdkError;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use carbone_rs::carbone_sdk::carbone::CARBONE_API_URL;

    #[test]
    fn test_api_token_not_given() {

        let error = match Config::new("".to_string(), "http://localhost".to_string(), 2 as u8, "2".to_string()) {
            Ok(config) => config.to_string(),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingApiToken.to_string();

        assert_eq!(expected_error, error);
    }

    #[test]
    fn test_api_url_not_given() {

        let error = match Config::new("test_token".to_string(), "".to_string(), 2 as u8, "2".to_string()) {
            Ok(config) => config.to_string(),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingApiUrl.to_string();

        assert_eq!(expected_error, error);
    }

    #[test]
    fn test_api_version_not_given() {

        let error = match Config::new("test_token".to_string(), "http://localhost".to_string(), 2 as u8, "".to_string()) {
            Ok(config) => config.to_string(),
            Err(e) => e.to_string(),
        };

        let expected_error = CarboneSdkError::MissingApiVersion.to_string();

        assert_eq!(expected_error, error);
    }

    #[test]
    fn test_default() {

        let config: Config = Default::default();

        let timeout: u8 = 60;
        let api_url = CARBONE_API_URL.to_string();
        let api_version = "4".to_string();

        assert_eq!(config.get_api_timeout(), &timeout);
        assert_eq!(config.get_api_url(), &api_url);
        assert_eq!(config.get_api_token().is_empty(), true);
        assert_eq!(config.get_api_version(), &api_version);
    }

    #[test]
    fn test_from_str() -> Result<(), CarboneSdkError> {

        let config = Config::from_str(r#"{
            "apiTimeout": 4,
            "apiUrl": "http://127.0.0.1",
            "apiToken": "test_abcd",
            "apiVersion" : "2"
        }"#)?;

        let expected = Config::new(
            "test_abcd".to_string(),
            "http://127.0.0.1".to_string(), 
            4 as u8,
            "2".to_string())?;

        assert_eq!(expected, config);

        Ok(())
    }

    #[test]
    fn test_from_str_bad_format_given() {

        let error = match Config::from_str(r#"{
            "apiTimeout": 4,
            "apiUrl": "http://127.0.0.1",
            "apToken": "test_abcd",
            "apiVersion" : "2"
        }"#) {
            Ok(_) => panic!("the convertion is ok"),
            Err(e) => e.to_string()
        };

        let expected_error = CarboneSdkError::ParseError("from_str".to_string(), "missing field `apiToken` at line 6 column 9".to_string()); 

        assert_eq!(expected_error.to_string(), error);

    }
}