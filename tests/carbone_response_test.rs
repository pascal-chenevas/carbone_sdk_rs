#[cfg(test)]
mod tests {

    use carbone_sdk_rs::carbone_response::CarboneSDKResponse;
    use std::collections::HashMap;

    #[test]
    fn test_get_template_id()  {
        
        let expected_template_id = "2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad".to_string();
        let data = HashMap::from([("templateId".to_string(), expected_template_id.clone())]);
        let carbone_resp = CarboneSDKResponse::new(true, Some(data), None);
        
        let template_id = carbone_resp.get_template_id();

        assert_eq!(template_id, expected_template_id);
    }

    #[test]
    fn test_get_render_id()  {
        
        let expected_render_id = "MTAuMjAuMjEuMTAgICAg01E98H4R7PMC2H6XSE5Z6J8XYQ.odt".to_string();
        let data = HashMap::from([("renderId".to_string(), expected_render_id.clone())]);
        let carbone_resp = CarboneSDKResponse::new(true, Some(data), None);
        
        let template_id = carbone_resp.get_render_id();

        assert_eq!(template_id, expected_render_id);
    }

}