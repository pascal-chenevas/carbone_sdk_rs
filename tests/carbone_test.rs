use std::collections::HashMap;

use httpmock::prelude::*;

use carbone_rs::carbone_sdk::carbone_response::CarboneSDKResponse;
use carbone_rs::carbone_sdk::carbone::CarboneSDK;
use carbone_rs::carbone_sdk::errors::CarboneSdkError;
use carbone_rs::carbone_sdk::config::Config;

#[test]
fn simple_test() -> Result<(), CarboneSdkError>{

    let mut data = HashMap::new();
    data.insert("templateId".to_string(), "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string());

    let body = CarboneSDKResponse{
        success: true,
        data: Some(data),
        error: None
    };

    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let m = server.mock(|when, then| {
        when.method("POST")
            .path("/template");
        then.status(200)
            .header("content-type", "application/json")
            .json_body_obj(&body);
    });

    let config = Config{
        api_token: "test_q".to_string(),
        api_url: format!("{}{}", "http://127.0.0.1:", server.port()), // port change each run
        api_timeout: 4,
        api_version: "2".to_string(),
    };
   
    let carbone_sdk = CarboneSDK::new(config);

    let template_file = String::from("template.odt");
    let template_id = carbone_sdk.add_template(&template_file, "".to_string())?;

    // Assert
    m.assert();
    assert_eq!(template_id, "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114");

    Ok(())
}
