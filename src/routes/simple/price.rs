pub fn simple_price(args: ArgsSimplePrice) -> SimplePrice {
    let mut url_params: Vec<HttpUrlParam> = vec![
        HttpUrlParam {
            key: "ids".to_string(),
            value: "args.name".to_string(),
        },
        HttpUrlParam {
            key: "vs_currencies".to_string(),
            value: "args.name".to_string(),
        },
    ];
    if args.include_market_cap.is_some() {
        url_params.push(HttpUrlParam {
            key: "include_market_cap".to_string(),
            value: args.include_market_cap.unwrap(),
        })
    }
    if args.include_24hr_vol.is_some() {
        url_params.push(HttpUrlParam {
            key: "include_24hr_vol".to_string(),
            value: args.include_24hr_vol.unwrap(),
        })
    }
    if args.include_24hr_change.is_some() {
        url_params.push(HttpUrlParam {
            key: "include_24hr_change".to_string(),
            value: args.include_24hr_change.unwrap(),
        })
    }
    if args.include_last_updated_at.is_some() {
        url_params.push(HttpUrlParam {
            key: "include_last_updated_at".to_string(),
            value: args.include_last_updated_at.unwrap(),
        })
    }
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    }) {
        Ok(Some(v)) => v,
        Ok(None) => panic!("Did not receive HTTP response"),
        Err(e) => panic!("{}", e),
    };

    // handle json rpc error
    if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
        let response_body: String = match http_response.body {
            Some(v) => v,
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<SimplePrice>(&v).unwrap(),
            None => panic!(
                "Missing response with successful HTTP status {}",
                http_response.status
            ),
        };
    }

    panic!(
        "Unexpected HTTP response with status: {}",
        http_response.status
    );
}