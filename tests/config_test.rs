use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::errors::CarboneSdkError;

#[cfg(test)]
mod tests {

    use super::*;
    use carbone_sdk_rs::config::CARBONE_API_URL;
    use carbone_sdk_rs::config::CARBONE_API_VERSION;
    use std::str::FromStr;

    #[test]
    fn test_api_url_not_given() {

        let result = Config::new("".to_string(), 6, 2);

        let expected_error = "api_url: Validation error: url [{\"value\": String(\"\")}]".to_string();

        assert!(result.is_err());
        assert_eq!(expected_error, result.unwrap_err().to_string());
    }

    #[test]
    fn test_default() {

        let config: Config = Default::default();

        let timeout: u8 = 60;
        let api_url = CARBONE_API_URL.to_string();
        let api_version = CARBONE_API_VERSION;

        assert_eq!(config.api_timeout, timeout);
        assert_eq!(config.api_url, api_url);
        assert_eq!(config.api_version, api_version);
    }

    #[test]
    fn test_from_str() -> Result<(), CarboneSdkError> {

        let config = Config::from_str(r#"{
            "apiTimeout": 4,
            "apiUrl": "http://127.0.0.1",
            "apiVersion" : 2
        }"#)?;

        let expected = Config::new(
            "http://127.0.0.1".to_string(), 
            4,
            2)?;

        assert_eq!(expected, config);

        Ok(())
    }

    #[test]
    fn test_from_str_bad_format_given() {

        let result = Config::from_str(r#"{
            "apiTimeout": 4,
            "apiUr" "http://127.0.0.1",
            "apiVersion" : 2
        }"#);
        
        let expected_error = "CarboneSDK FromStr JsonParseError: expected `:` at line 3 column 21".to_string(); 

        assert!(result.is_err());
        assert_eq!(expected_error, result.unwrap_err().to_string());

    }

    #[test]
    fn test_from_file() -> Result<(), CarboneSdkError> {

        let config = Config::from_file("tests/config.test.json")?;

        let expected = Config::new(
            "http://127.0.0.1:57780".to_string(), 
            4,
            2)?;

        assert_eq!(expected, config);

        Ok(())
    }

    #[test]
    fn test_from_file_wrong_path_given() -> Result<(), CarboneSdkError> {

        let result = Config::from_file("tests/bad/path/config.test.json") ;

        let expected_error = "Carbone SDK error: file \"tests/bad/path/config.test.json\" not found".to_string(); 

        assert!(result.is_err());
        assert_eq!(expected_error, result.unwrap_err().to_string());

        Ok(())
    }


}