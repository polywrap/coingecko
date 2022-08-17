pub mod wrap;

pub use wrap::*;
use polywrap_wasm_rs::{JSON,BigNumber};
use crate::imported::http_module;

pub fn ping(_args: ArgsPing) -> Ping {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/ping".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Ping>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


// pub fn simple_price(args: ArgsSimplePrice) -> SimplePrice {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "ids".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "vs_currencies".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_market_cap".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_24hr_vol".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_24hr_change".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_last_updated_at".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
//         request: Some(HttpRequest {
//             headers: None,
//             url_params: url_params,
//             response_type: HttpResponseType::TEXT,
//             body: None,
//         }),
//     }) {
//         Ok(Some(v)) => v,
//         Ok(None) => panic!("Did not receive HTTP response"),
//         Err(e) => panic!("{}", e),
//     };

//     // handle json rpc error
//     if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
//         let response_body: String = match http_response.body {
//             Some(v) => v,
//             None => "An unknown error occurred!".to_string()
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<SimplePrice>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn simple_token_price(args: ArgsSimpleTokenPrice) -> SimplePrice {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "contract_addresses".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "vs_currencies".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_market_cap".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_24hr_vol".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_24hr_change".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_last_updated_at".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/simple/token_price/args.id".to_string(),
//         request: Some(HttpRequest {
//             headers: None,
//             url_params: url_params,
//             response_type: HttpResponseType::TEXT,
//             body: None,
//         }),
//     }) {
//         Ok(Some(v)) => v,
//         Ok(None) => panic!("Did not receive HTTP response"),
//         Err(e) => panic!("{}", e),
//     };

//     // handle json rpc error
//     if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
//         let response_body: String = match http_response.body {
//             Some(v) => v,
//             None => "An unknown error occurred!".to_string()
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<SimplePrice>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn simpleSupportedVsCurrencies(args: ArgsSimpleSupportedVsCurrencies) -> SimpleSupportedVsCurrencies {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/simple/supported_vs_currencies".to_string(),
//         request: Some(HttpRequest {
//             headers: None,
//             url_params: url_params,
//             response_type: HttpResponseType::TEXT,
//             body: None,
//         }),
//     }) {
//         Ok(Some(v)) => v,
//         Ok(None) => panic!("Did not receive HTTP response"),
//         Err(e) => panic!("{}", e),
//     };

//     // handle json rpc error
//     if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
//         let response_body: String = match http_response.body {
//             Some(v) => v,
//             None => "An unknown error occurred!".to_string()
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<SimpleSupportedVsCurrencies>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


pub fn coins_list(args: ArgsCoinsList) -> Vec<CoinListItem> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "include_platform".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<CoinListItem>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_markets(args: ArgsCoinsMarkets) -> Vec<CoinMarketItem> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "ids".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "category".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "sparkline".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "price_change_percentage".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/markets".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<CoinMarketItem>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coin(args: ArgsCoin) -> CoinsItem {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "localization".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "tickers".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "market_data".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "community_data".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "developer_data".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "sparkline".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<CoinsItem>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_tickers(args: ArgsCoinsTickers) -> Tickers {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "exchange_ids".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_exchange_logo".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "depth".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/tickers".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Tickers>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_history(args: ArgsCoinsHistory) -> History {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "date".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "localization".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/history".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<History>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_market_chart(args: ArgsCoinsMarketChart) -> MarketChart {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "interval".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/market_chart".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<MarketChart>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_market_chart_range(args: ArgsCoinsMarketChartRange) -> MarketChart {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "from".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "to".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/market_chart/range".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<MarketChart>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_contract(args: ArgsCoinsContract) -> Contract {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "contract_address".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/contract/args.contract_address".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Contract>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_contract_market_chart(args: ArgsCoinsContractMarketChart) -> MarketChart {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "contract_address".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/contract/args.contract_address/market_chart/".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<MarketChart>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_contract_market_chart_range(args: ArgsCoinsContractMarketChartRange) -> Contract {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "contract_address".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "from".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "to".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/contract/args.contract_address/market_chart/range".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Contract>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_ohlc(args: ArgsCoinsOhlc) -> Vec<Vec<BigNumber>> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/args.id/ohlc".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Vec<BigNumber>>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn asset_platforms(args: ArgsAssetPlatforms) -> Vec<AssetPlatform> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/asset_platforms".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<AssetPlatform>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_categories_list(args: ArgsCoinsCategoriesList) -> Vec<CategoryId> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/categories/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<CategoryId>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn coins_categories(args: ArgsCoinsCategories) -> Vec<Category> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/categories".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Category>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn exchanges(args: ArgsExchanges) -> Vec<Exchange> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/exchanges".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Exchange>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn exchanges_list(args: ArgsExchangesList) -> Vec<ExchangeId> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/exchanges/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<ExchangeId>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn exchange(args: ArgsExchange) -> Exchange {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/exchanges/args.id".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Exchange>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn exchanges_tickers(args: ArgsExchangesTickers) -> Tickers {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "coin_ids".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_exchange_logo".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "depth".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/exchanges/args.id/tickers".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Tickers>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn indexes(args: ArgsIndexes) -> Vec<Index> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/indexes".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Index>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn market_indexes(args: ArgsMarketIndexes) -> MarketIndex {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "market_id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/indexes/args.market_id/args.id".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<MarketIndex>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn indexes_list(args: ArgsIndexesList) -> Vec<IndexId> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/indexes/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<IndexId>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn derivatives(args: ArgsDerivatives) -> Vec<Derivative> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "include_tickers".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/derivatives".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Derivative>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn derivatives_exchanges(args: ArgsDerivativesExchanges) -> Vec<Derivative> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/derivatives/exchanges".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Derivative>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn derivatives_exchange(args: ArgsDerivativesExchange) -> Derivative {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_tickers".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/derivatives/exchanges/args.id".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Derivative>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn derivatives_exchanges_list(args: ArgsDerivativesExchangesList) -> Vec<DerivativeExchangeId> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/derivatives/exchanges/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<DerivativeExchangeId>>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


// pub fn exchangesVolumeChart(args: ArgsExchangesVolumeChart) -> ExchangesVolumeChart {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/exchanges/args.id/volume_chart".to_string(),
//         request: Some(HttpRequest {
//             headers: None,
//             url_params: url_params,
//             response_type: HttpResponseType::TEXT,
//             body: None,
//         }),
//     }) {
//         Ok(Some(v)) => v,
//         Ok(None) => panic!("Did not receive HTTP response"),
//         Err(e) => panic!("{}", e),
//     };

//     // handle json rpc error
//     if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
//         let response_body: String = match http_response.body {
//             Some(v) => v,
//             None => "An unknown error occurred!".to_string()
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<ExchangesVolumeChart>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


pub fn exchange_rates(args: ArgsExchangeRates) -> ExchangeRates {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/exchange_rates".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<ExchangeRates>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


// pub fn search(args: ArgsSearch) -> Search {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "query".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/search".to_string(),
//         request: Some(HttpRequest {
//             headers: None,
//             url_params: url_params,
//             response_type: HttpResponseType::TEXT,
//             body: None,
//         }),
//     }) {
//         Ok(Some(v)) => v,
//         Ok(None) => panic!("Did not receive HTTP response"),
//         Err(e) => panic!("{}", e),
//     };

//     // handle json rpc error
//     if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
//         let response_body: String = match http_response.body {
//             Some(v) => v,
//             None => "An unknown error occurred!".to_string()
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Search>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


pub fn search_trending(args: ArgsSearchTrending) -> SearchTrending {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/search/trending".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<SearchTrending>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn global(args: ArgsGlobal) -> Global {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/global".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Global>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn global_decentralized_finance_defi(args: ArgsGlobalDecentralizedFinanceDefi) -> GlobalDefi {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/global/decentralized_finance_defi".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<GlobalDefi>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn companies_public_treasury(args: ArgsCompaniesPublicTreasury) -> CompaniesPublicTreasury {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "coin_id".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/companies/public_treasury/args.coin_id".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
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
            None => "An unknown error occurred!".to_string()
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<CompaniesPublicTreasury>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}
