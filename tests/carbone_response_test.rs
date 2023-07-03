#[cfg(test)]
mod tests {

    use carbone_sdk_rs::carbone_response::ResponseBody;
    use carbone_sdk_rs::errors::CarboneError;
    use carbone_sdk_rs::render::RenderId;
    use carbone_sdk_rs::template::TemplateId;
    use serde_json;

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
            \"error\": \"an error message\",
        }}
        "
        );

        let deserialized: ResponseBody = serde_json::from_str(&resp_boyd).unwrap();

        assert_eq!(deserialized.get_error_code().is_empty(), true);
    }
}
