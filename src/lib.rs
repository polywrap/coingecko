pub mod wrap;

pub use wrap::*;
use polywrap_wasm_rs::JSON;
use crate::imported::http_module;

pub fn ping(args: ArgsPing) -> Ping {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

    ]);
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


pub fn simple_price(args: ArgsSimplePrice) -> SimplePrice {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "ids".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "vs_currencies".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_market_cap".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_24hr_vol".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_24hr_change".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_last_updated_at".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
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
            Some(v) => JSON::from_str::<SimplePrice>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


pub fn simple_token_price(args: ArgsSimpleTokenPrice) -> SimplePrice {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "contract_addresses".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "vs_currencies".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_market_cap".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_24hr_vol".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_24hr_change".to_string(), value: "args.name".to_string() },
        HttpUrlParam { key: "include_last_updated_at".to_string(), value: "args.name".to_string() }
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/simple/token_price/args.id".to_string(),
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
            Some(v) => JSON::from_str::<SimplePrice>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {}", http_response.status)
        };
    }

    panic!("Unexpected HTTP response with status: {}", http_response.status);
}


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


// pub fn coinsList(args: ArgsCoinsList) -> CoinsList {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "include_platform".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/list".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsList>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsMarkets(args: ArgsCoinsMarkets) -> CoinsMarkets {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "ids".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "category".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "sparkline".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "price_change_percentage".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/markets".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsMarkets>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coins(args: ArgsCoins) -> Coins {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "localization".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "tickers".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "market_data".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "community_data".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "developer_data".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "sparkline".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id".to_string(),
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
//             Some(v) => JSON::from_str::<Coins>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsTickers(args: ArgsCoinsTickers) -> CoinsTickers {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "exchange_ids".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_exchange_logo".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "depth".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/tickers".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsTickers>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsHistory(args: ArgsCoinsHistory) -> CoinsHistory {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "date".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "localization".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/history".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsHistory>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsMarketChart(args: ArgsCoinsMarketChart) -> CoinsMarketChart {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "interval".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/market_chart".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsMarketChart>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsMarketChartRange(args: ArgsCoinsMarketChartRange) -> CoinsMarketChartRange {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "from".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "to".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/market_chart/range".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsMarketChartRange>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsContract(args: ArgsCoinsContract) -> CoinsContract {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "contract_address".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/contract/args.contract_address".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsContract>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsContractMarketChart(args: ArgsCoinsContractMarketChart) -> CoinsContractMarketChart {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "contract_address".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/contract/args.contract_address/market_chart/".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsContractMarketChart>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsContractMarketChartRange(args: ArgsCoinsContractMarketChartRange) -> CoinsContractMarketChartRange {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "contract_address".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "from".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "to".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/contract/args.contract_address/market_chart/range".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsContractMarketChartRange>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsOhlc(args: ArgsCoinsOhlc) -> CoinsOhlc {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "vs_currency".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/ohlc".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsOhlc>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn assetPlatforms(args: ArgsAssetPlatforms) -> AssetPlatforms {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/asset_platforms".to_string(),
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
//             Some(v) => JSON::from_str::<AssetPlatforms>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsCategoriesList(args: ArgsCoinsCategoriesList) -> CoinsCategoriesList {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/categories/list".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsCategoriesList>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn coinsCategories(args: ArgsCoinsCategories) -> CoinsCategories {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/categories".to_string(),
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
//             Some(v) => JSON::from_str::<CoinsCategories>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn exchanges(args: ArgsExchanges) -> Exchanges {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/exchanges".to_string(),
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
//             Some(v) => JSON::from_str::<Exchanges>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn exchangesList(args: ArgsExchangesList) -> ExchangesList {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/exchanges/list".to_string(),
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
//             Some(v) => JSON::from_str::<ExchangesList>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn exchanges(args: ArgsExchanges) -> Exchanges {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/exchanges/args.id".to_string(),
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
//             Some(v) => JSON::from_str::<Exchanges>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn exchangesTickers(args: ArgsExchangesTickers) -> ExchangesTickers {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "coin_ids".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_exchange_logo".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "depth".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/exchanges/args.id/tickers".to_string(),
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
//             Some(v) => JSON::from_str::<ExchangesTickers>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn indexes(args: ArgsIndexes) -> Indexes {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/indexes".to_string(),
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
//             Some(v) => JSON::from_str::<Indexes>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn indexes(args: ArgsIndexes) -> Indexes {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "market_id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/indexes/args.market_id/args.id".to_string(),
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
//             Some(v) => JSON::from_str::<Indexes>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn indexesList(args: ArgsIndexesList) -> IndexesList {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/indexes/list".to_string(),
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
//             Some(v) => JSON::from_str::<IndexesList>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn derivatives(args: ArgsDerivatives) -> Derivatives {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "include_tickers".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/derivatives".to_string(),
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
//             Some(v) => JSON::from_str::<Derivatives>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn derivativesExchanges(args: ArgsDerivativesExchanges) -> DerivativesExchanges {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "order".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "per_page".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "page".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/derivatives/exchanges".to_string(),
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
//             Some(v) => JSON::from_str::<DerivativesExchanges>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn derivativesExchanges(args: ArgsDerivativesExchanges) -> DerivativesExchanges {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
//         HttpUrlParam { key: "include_tickers".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/derivatives/exchanges/args.id".to_string(),
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
//             Some(v) => JSON::from_str::<DerivativesExchanges>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn derivativesExchangesList(args: ArgsDerivativesExchangesList) -> DerivativesExchangesList {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/derivatives/exchanges/list".to_string(),
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
//             Some(v) => JSON::from_str::<DerivativesExchangesList>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


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


// pub fn exchangeRates(args: ArgsExchangeRates) -> ExchangeRates {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/exchange_rates".to_string(),
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
//             Some(v) => JSON::from_str::<ExchangeRates>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


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


// pub fn searchTrending(args: ArgsSearchTrending) -> SearchTrending {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/search/trending".to_string(),
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
//             Some(v) => JSON::from_str::<SearchTrending>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn global(args: ArgsGlobal) -> Global {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/global".to_string(),
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
//             Some(v) => JSON::from_str::<Global>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn globalDecentralizedFinanceDefi(args: ArgsGlobalDecentralizedFinanceDefi) -> GlobalDecentralizedFinanceDefi {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![

//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/global/decentralized_finance_defi".to_string(),
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
//             Some(v) => JSON::from_str::<GlobalDecentralizedFinanceDefi>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }


// pub fn companiesPublicTreasury(args: ArgsCompaniesPublicTreasury) -> CompaniesPublicTreasury {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "coin_id".to_string(), value: "args.name".to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/companies/public_treasury/args.coin_id".to_string(),
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
//             Some(v) => JSON::from_str::<CompaniesPublicTreasury>(&v).unwrap(),
//             None => panic!("Missing response with successful HTTP status {}", http_response.status)
//         };
//     }

//     panic!("Unexpected HTTP response with status: {}", http_response.status);
// }
