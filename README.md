[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/pascal-chenevas/carbone_sdk_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/pascal-chenevas/carbone_sdk_rs/actions/workflows/rust.yml)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

# carbone_sdk_rs

carbone_sdk_rs is a Library that supplies functionalities to communicate with the [Carbone API](https://carbone.io/api-reference.html).

# State of development

This Library is in the early stage of active development and doesn't reach the status of a stable release.
Therefore it can not be used in a production environment.

# Process to render a new report

```mermaid
sequenceDiagram
    Client->>Carbone API: send a template file to /template
    Carbone API-->>Client: send a template_id 
    Client->>Carbone API: send json data to be rendered to /render/{template_id}
    Carbone API-->>Client: send a render_id
    Carbone API-->>Client: get the rendered report from /render/{render_id}
```

# Installation

TODO

# Render a new report

### Using an existing Template

```rust
use std::env;
 
use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::render::*;
use carbone_sdk_rs::carbone::Carbone;
use carbone_sdk_rs::types::ApiJsonToken;
use carbone_sdk_rs::template::TemplateId;
 
use carbone_sdk_rs::errors::CarboneError;

fn main() -> Result<(), CarboneError> {
    
     let token =  match env::var("CARBONE_TOKEN") {
             Ok(v) => v,
             Err(e) => panic!("{}", e.to_string())
     };
 
    let config: Config = Default::default();
 
    let api_token = ApiJsonToken::new(token)?;

    let render_options_value = String::from(r#"
         "data" : {
             "firstname" : "John",
             "lastname" : "Wick"
        },
        "convertTo" : "odt"
    "#);
 
    let render_options = RenderOptions::new(render_options_value)?;

    let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;

    let carbone_sdk = Carbone::new(&config, &api_token)?;
    
    let _report_content = carbone_sdk.generate_report_with_template_id(template_id, render_options)?;

    Ok(())
}
```

# References

[Carbone.io](https://carbone.io) a report generator.

[Carbone CLI App](https://github.com/pascal-chenevas/carbone_cli_rs) a simple CLI App to create reports.

## Useful links

- [How to build a template file](https://carbone.io/documentation.html#building-a-template)
