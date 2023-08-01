#[cfg(test)]
mod tests {

    use carbone_sdk_rs::carbone_response::{APIResponse, APIResponseData};
    use carbone_sdk_rs::errors::CarboneError;
    use carbone_sdk_rs::render::RenderId;
    use carbone_sdk_rs::template::TemplateId;
    use serde_json;

    #[test]
    fn test_deserialize_response_succeed() -> Result<(), CarboneError> {
        let expected_template_id = TemplateId::new(
            "2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad".to_string(),
        )?;

        let resp_data = APIResponseData {
            template_id: Some(expected_template_id.clone()),
            render_id: None,
            template_file_extension: None,
        };
        let carbone_resp = APIResponse::new(true, Some(resp_data), None, None);

        let resp_body = format!(
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

        let deserialized: APIResponse = serde_json::from_str(&resp_body).unwrap();
        assert_eq!(deserialized, carbone_resp);

        Ok(())
    }

    #[test]
    fn test_serialize_response_succeed() -> Result<(), CarboneError> {
        let expected_template_id = TemplateId::new(
            "2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad".to_string(),
        )?;

        let resp_data = APIResponseData {
            template_id: Some(expected_template_id.clone()),
            render_id: None,
            template_file_extension: None,
        };
        let carbone_resp = APIResponse::new(true, Some(resp_data), None, None);

        let resp_body = format!(
            "{{\"success\":true,\"data\":{{\"templateId\":\"{}\"}}}}",
            expected_template_id.as_str()
        );

        let serialized = serde_json::to_string(&carbone_resp).unwrap();
        assert_eq!(resp_body, serialized);

        Ok(())
    }

    #[test]
    fn test_deserialize_response_failed() -> Result<(), CarboneError> {
        let error_msg = "an error message".to_string();
        let error_code = "W45".to_string();
        let carbone_resp = APIResponse::new(
            false,
            None,
            Some(error_msg.clone()),
            Some(error_code.clone()),
        );

        let resp_body = format!(
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

        let deserialized: APIResponse = serde_json::from_str(&resp_body).unwrap();
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

        let deserialized: APIResponse = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(
            deserialized.data.unwrap().template_id.unwrap(),
            expected_template_id
        );

        Ok(())
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

        let deserialized: APIResponse = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(
            deserialized.data.unwrap().render_id.unwrap(),
            expected_render_id
        );

        Ok(())
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

        let deserialized: APIResponse = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(deserialized.get_error_code(), expected_error_code);
    }

    #[test]
    fn test_get_error_code_is_empty() {
        let resp_body = format!(
            "
        {{
            \"success\": false,
            \"error\": \"an error message\"
        }}
        "
        );

        let deserialized: APIResponse = serde_json::from_str(&resp_body).unwrap();

        assert_eq!(deserialized.get_error_code().is_empty(), true);
    }

    #[test]
    fn test_response_body_without_error_code() {
        let succeed = false;
        let error_msg = "an error message";

        let resp_body = format!(
            "
        {{
            \"success\": {},
            \"error\": \"{}\"
        }}
        ",
            succeed, error_msg
        );

        let carbone_resp = APIResponse::new(succeed, None, Some(error_msg.to_string()), None);

        let deserialized: APIResponse = serde_json::from_str(&resp_body).unwrap();

        assert_eq!(carbone_resp, deserialized);
    }
}
