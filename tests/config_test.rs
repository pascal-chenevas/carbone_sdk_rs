use carbone_rs::carbone_sdk::config::Config;
use carbone_rs::carbone_sdk::errors::CarboneSdkError;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use carbone_rs::carbone_sdk::carbone::CARBONE_API_URL;

    #[test]
    fn test_default() {

        let config: Config = Default::default();

        assert_eq!(config.api_timeout, 60);
        assert_eq!(config.api_url, CARBONE_API_URL.to_string());
        assert_eq!(config.api_token.is_empty(), true);
        assert_eq!(config.api_version, "4".to_string());
    }

    #[test]
    fn test_from_str() -> Result<(), CarboneSdkError> {

        let config = Config::from_str(r#"{
            "apiTimeout": 4,
            "apiUrl": "http://127.0.0.1",
            "apiToken": "test_abcd",
            "apiVersion" : "2"
        }"#)?;

        let expected = Config {
            api_timeout: 4,
            api_url: "http://127.0.0.1".to_string(),
            api_token: "test_abcd".to_string(),
            api_version: "2".to_string(),
        };

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