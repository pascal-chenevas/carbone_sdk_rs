#[cfg(test)]
mod tests {

    use  std::matches;
    use carbone_sdk_rs::types::ApiJsonToken;

    use anyhow::Result;

    #[test]
    fn test_api_token_as_str() -> Result<()> {

        let api_token_value = "test_32u1i3ui1212334395349dsaowe912384ads89de8e93hj123iowa21085dsaowe91843784p213894dsa912384ads89de8e93hj123iowa210309dhsudausdasda72q37q783hy3243829434gdgadghdsaowe912384ads89de8e93hj1owa21023113i12u32i1321io39534985dsaowe9123843784p213894309dhsudausdasda72q37q783h43784p213894309dhsuda4gdgadghdsaow2384ads89de8e93hj123iowa21023113i12u32i1321io39534985dsa";
        let api_token = ApiJsonToken::new(api_token_value.to_string())?;

        assert_eq!(api_token_value, api_token.as_str());

        Ok(())
    }

    #[test]
    fn test_api_token_short_token_given() -> Result<()> {

        let api_token_value = "test_";

        let result = ApiJsonToken::new(api_token_value.to_string());
        let is_err = result.is_err();
        let error = result.unwrap_err().to_string();

        let _expected_error = "api_token: Validation error: length [{\"min\": Number(357), \"value\": String(\"test_\")}]";

        assert!(is_err);
        assert!(matches!(error, _expected_error));

        Ok(())
    }
}