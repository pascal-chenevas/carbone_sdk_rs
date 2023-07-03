#[cfg(test)]
mod tests {

    use carbone_sdk_rs::carbone_response::ResponseBody;
    use carbone_sdk_rs::errors::CarboneError;
    use carbone_sdk_rs::render::RenderId;
    use carbone_sdk_rs::template::TemplateId;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_response_succeed() -> Result<(), CarboneError> {
       
        let expected_template_id = TemplateId::new(
            "2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad".to_string(),
        )?;
        let data = HashMap::from([("templateId".to_string(), expected_template_id.as_str().to_string())]);
        let carbone_resp = ResponseBody::new(true, Some(data), None, None);

    
        let resp_boyd = format!(
            "
        {{
            \"success\": true,
            \"data\": {{
                      \"templateId\": \"{}\"
            }}
        }}
        ",
            expected_template_id.as_str()
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();
        assert_eq!(deserialized, carbone_resp);
        
        Ok(())

    }

    #[test]
    fn test_deserialize_response_failed() -> Result<(), CarboneError> {
       
        let error_msg = "an error message".to_string();
        let error_code = "W45".to_string();
        let carbone_resp = ResponseBody::new(false, None, Some(error_msg.clone()), Some(error_code.clone()));

    
        let resp_boyd = format!(
            "
            {{
                \"success\": false,
                \"error\": \"{}\",
                \"code\" : \"{}\"
            }}
        ",
        &error_msg.as_str(),
        &error_code.as_str()
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();
        assert_eq!(deserialized, carbone_resp);
        
        Ok(())

    }

    #[test]
    fn test_get_template_id() -> Result<(), CarboneError> {
        let expected_template_id = TemplateId::new(
            "2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad".to_string(),
        )?;
        let resp_boyd = format!(
            "
        {{
            \"success\": true,
            \"data\": {{
                      \"templateId\": \"{}\"
            }}
        }}
        ",
            expected_template_id.as_str()
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(
            deserialized.get_template_id().unwrap(),
            expected_template_id
        );

        Ok(())
    }

    #[test]
    fn test_get_template_id_failed() {
        let resp_boyd = format!(
            "
        {{
            \"success\": true,
            \"data\": {{
                      \"a_key\": \"123\"
            }}
        }}
        "
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();
        let result = deserialized.get_template_id();

        let expected_error = CarboneError::EmptyString("template_id".to_string());

        assert!(result.is_err());

        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    #[test]
    fn test_get_render_id() -> Result<(), CarboneError> {
        let expected_render_id =
            RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string())?;
        let resp_boyd = format!(
            "
        {{
            \"success\": true,
            \"data\": {{
                      \"renderId\": \"{}\"
            }}
        }}
        ",
            expected_render_id.as_str()
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(deserialized.get_render_id().unwrap(), expected_render_id);

        Ok(())
    }

    #[test]
    fn test_get_render_id_failed() {
        let resp_boyd = format!(
            "
        {{
            \"success\": true,
            \"data\": {{
                      \"a_key\": \"123\"
            }}
        }}
        "
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();
        let result = deserialized.get_render_id();

        let expected_error = CarboneError::EmptyString("render_id".to_string());

        assert!(result.is_err());

        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    #[test]
    fn test_get_template_id_missing_data_as_key_in_json() {
        let resp_boyd = format!(
            "
        {{
            \"success\": true
        }}
        "
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();
        let result = deserialized.get_render_id();

        let expected_error = CarboneError::EmptyString("render_id".to_string());

        assert!(result.is_err());

        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    #[test]
    fn test_get_error_code() {
        let expected_error_code = "w115";
        let resp_boyd = format!(
            "
        {{
            \"success\": false,
            \"error\": \"Invalid or undefined TemplateId or RenderId in the URL\",
            \"code\" : \"{}\"
        }}
        ",
            expected_error_code
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(deserialized.get_error_code(), expected_error_code);
    }

    #[test]
    fn test_get_error_code_is_empty() {
        let resp_boyd = format!(
            "
        {{
            \"success\": false,
            \"error\": \"an error message\"
        }}
        "
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(deserialized.get_error_code().is_empty(), true);
    }
}
