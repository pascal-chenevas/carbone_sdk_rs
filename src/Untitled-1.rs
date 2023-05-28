{
                Ok(resp) => {
                    if resp.status().is_success() {
                         resp.text();
                    } else {
                        Err(CarboneSdkError::MissingArgument("()".to_string(), "()".to_string()));
                    }
                },
                Err(_) => Err(CarboneSdkError::MissingArgument("()".to_string(), "()".to_string())),
            };
            Ok(response_txt)

            let data = r#"{
      "id": 42,
      "date": 1492012745,
      "company": {
        "name": "myCompany",
        "address": "here",
        "city": "Notfar",
        "postalCode": 123456
      },
      "customer": {
        "name": "myCustomer",
        "address": "there",
        "city": "Faraway",
        "postalCode": 654321
      },
      "products": [
        {
          "name": "product 1",
          "priceUnit": 0.1,
          "quantity": 10,
          "priceTotal": 1
        },
        {
          "name": "product 2",
          "priceUnit": 0.2,
          "quantity": 20,
          "priceTotal": 4
        },
        {
          "name": "product 3",
          "priceUnit": 0.3,
          "quantity": 30,
          "priceTotal": 9
        }
      ],
      "total": 14
    }"#;

2436447a0d5954de2ad9cd28376f9e743a8fe732b829a1d37b60f51539dad7ad
81700022b5cab8efc79f276b69d17251b03ffcdab61c026b75f783b55e3953cb
