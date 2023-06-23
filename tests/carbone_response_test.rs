#[cfg(test)]
mod tests {

    use carbone_sdk_rs::errors::CarboneError;
    use carbone_sdk_rs::carbone_response::CarboneSDKResponse;
    use carbone_sdk_rs::render::RenderId;
    use carbone_sdk_rs::template::TemplateId;
    use std::collections::HashMap;

    #[test]
    fn test_get_template_id() -> Result<(), CarboneError> {
        
        let expected_template_id = TemplateId::new("2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad".to_string())?;
        let data = HashMap::from([("templateId".to_string(), expected_template_id.as_str().to_string())]);
        let carbone_resp = CarboneSDKResponse::new(true, Some(data), None);
        
        let template_id = carbone_resp.get_template_id()?;

         assert_eq!(template_id, expected_template_id);

         Ok(())
    }

    #[test]
    fn test_get_render_id() -> Result<(), CarboneError> {
        
        let expected_render_id = RenderId::new("MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string())?;
        let data = HashMap::from([("renderId".to_string(), expected_render_id.as_str().to_string())]);
        let carbone_resp = CarboneSDKResponse::new(true, Some(data), None);
        
        let render_id = carbone_resp.get_render_id()?;

        assert_eq!(render_id, expected_render_id);

        Ok(())
    }

}