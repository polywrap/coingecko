pub mod wrap;

use crate::imported::http_module;
use polywrap_wasm_rs::{BigNumber, Map, JSON};
pub use wrap::*;

pub fn ping(_args: ArgsPing) -> Ping {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/ping".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Ping>(&v).unwrap(),
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

// pub fn simple_price(args: ArgsSimplePrice) -> SimplePrice {
//     let mut url_params: Vec<HttpUrlParam> = vec![
//         HttpUrlParam {
//             key: "ids".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "vs_currencies".to_string(),
//             value: "args.name".to_string(),
//         },
//     ];
//     if args.include_market_cap.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_market_cap".to_string(),
//             value: args.include_market_cap.unwrap(),
//         })
//     }
//     if args.include_24hr_vol.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_24hr_vol".to_string(),
//             value: args.include_24hr_vol.unwrap(),
//         })
//     }
//     if args.include_24hr_change.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_24hr_change".to_string(),
//             value: args.include_24hr_change.unwrap(),
//         })
//     }
//     if args.include_last_updated_at.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_last_updated_at".to_string(),
//             value: args.include_last_updated_at.unwrap(),
//         })
//     }
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
//         request: Some(HttpRequest {
//             headers: None,
//             url_params: Some(url_params),
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<SimplePrice>(&v).unwrap(),
//             None => panic!(
//                 "Missing response with successful HTTP status {}",
//                 http_response.status
//             ),
//         };
//     }

//     panic!(
//         "Unexpected HTTP response with status: {}",
//         http_response.status
//     );
// }

// pub fn simple_token_price(args: ArgsSimpleTokenPrice) -> Map<String, SimplePrice> {
//     let mut url_params: Vec<HttpUrlParam> = vec![
//         HttpUrlParam {
//             key: "contract_addresses".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "vs_currencies".to_string(),
//             value: "args.name".to_string(),
//         },
//     ];
//     if args.include_market_cap.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_market_cap".to_string(),
//             value: args.include_market_cap.unwrap(),
//         })
//     }
//     if args.include_24hr_vol.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_24hr_vol".to_string(),
//             value: args.include_24hr_vol.unwrap(),
//         })
//     }
//     if args.include_24hr_change.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_24hr_change".to_string(),
//             value: args.include_24hr_change.unwrap(),
//         })
//     }
//     if args.include_last_updated_at.is_some() {
//         url_params.push(HttpUrlParam {
//             key: "include_last_updated_at".to_string(),
//             value: args.include_last_updated_at.unwrap(),
//         })
//     }
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: format!(
//             "https://api.coingecko.com/api/v3/simple/token_price/{}",
//             args.id
//         ),
//         request: Some(HttpRequest {
//             headers: None,
//             url_params: Some(url_params),
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Map<String, SimplePrice>>(&v).unwrap(),
//             None => panic!(
//                 "Missing response with successful HTTP status {}",
//                 http_response.status
//             ),
//         };
//     }

//     panic!(
//         "Unexpected HTTP response with status: {}",
//         http_response.status
//     );
// }

pub fn simple_supported_vs_currencies(args: ArgsSimpleSupportedVsCurrencies) -> Vec<String> {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/simple/supported_vs_currencies".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Vec<String>>(&v).unwrap(),
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

pub fn coins_list(args: ArgsCoinsList) -> Vec<CoinListItem> {
    let mut url_params: Option<Vec<HttpUrlParam>> = None;

    if args.include_platform.is_some() {
        url_params = Some(vec![HttpUrlParam {
            key: "include_platform".to_string(),
            value: args.include_platform.unwrap().to_string(),
        }])
    }

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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<CoinListItem>>(&v).unwrap(),
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

pub fn coins_markets(args: ArgsCoinsMarkets) -> Vec<CoinMarketItem> {
    let mut url_params: Vec<HttpUrlParam> = vec![HttpUrlParam {
        key: "vs_currency".to_string(),
        value: args.vs_currency,
    }];

    if args.ids.is_some() {
        url_params.push(HttpUrlParam {
            key: "ids".to_string(),
            value: args.ids.unwrap(),
        })
    }
    if args.category.is_some() {
        url_params.push(HttpUrlParam {
            key: "category".to_string(),
            value: args.category.unwrap(),
        })
    }
    if args.order.is_some() {
        url_params.push(HttpUrlParam {
            key: "order".to_string(),
            value: args.order.unwrap(),
        })
    }
    if args.per_page.is_some() {
        url_params.push(HttpUrlParam {
            key: "per_page".to_string(),
            value: args.per_page.unwrap().to_string(),
        })
    }
    if args.page.is_some() {
        url_params.push(HttpUrlParam {
            key: "page".to_string(),
            value: args.page.unwrap().to_string(),
        })
    }
    if args.sparkline.is_some() {
        url_params.push(HttpUrlParam {
            key: "sparkline".to_string(),
            value: args.sparkline.unwrap().to_string(),
        })
    }
    if args.price_change_percentage.is_some() {
        url_params.push(HttpUrlParam {
            key: "price_change_percentage".to_string(),
            value: args.price_change_percentage.unwrap(),
        })
    }

    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/markets".to_string(),
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
            Some(v) => JSON::from_str::<Vec<CoinMarketItem>>(&v).unwrap(),
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

pub fn coin(args: ArgsCoin) -> CoinsItem {
    let mut url_params: Vec<HttpUrlParam> = vec![];

    if args.localization.is_some() {
        url_params.push(HttpUrlParam {
            key: "localization".to_string(),
            value: args.localization.unwrap(),
        })
    }
    if args.tickers.is_some() {
        url_params.push(HttpUrlParam {
            key: "tickers".to_string(),
            value: args.tickers.unwrap().to_string(),
        })
    }
    if args.market_data.is_some() {
        url_params.push(HttpUrlParam {
            key: "market_data".to_string(),
            value: args.market_data.unwrap().to_string(),
        })
    }
    if args.community_data.is_some() {
        url_params.push(HttpUrlParam {
            key: "community_data".to_string(),
            value: args.community_data.unwrap().to_string(),
        })
    }
    if args.developer_data.is_some() {
        url_params.push(HttpUrlParam {
            key: "developer_data".to_string(),
            value: args.developer_data.unwrap().to_string(),
        })
    }
    if args.sparkline.is_some() {
        url_params.push(HttpUrlParam {
            key: "sparkline".to_string(),
            value: args.sparkline.unwrap().to_string(),
        })
    }
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!("https://api.coingecko.com/api/v3/coins/{}", args.id),
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
            Some(v) => JSON::from_str::<CoinsItem>(&v).unwrap(),
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

pub fn coins_tickers(args: ArgsCoinsTickers) -> Tickers {
    let mut url_params: Vec<HttpUrlParam> = vec![];

    if args.exchange_ids.is_some() {
        url_params.push(HttpUrlParam {
            key: "exchange_ids".to_string(),
            value: args.exchange_ids.unwrap(),
        })
    }
    if args.include_exchange_logo.is_some() {
        url_params.push(HttpUrlParam {
            key: "include_exchange_logo".to_string(),
            value: args.include_exchange_logo.unwrap(),
        })
    }
    if args.page.is_some() {
        url_params.push(HttpUrlParam {
            key: "page".to_string(),
            value: args.page.unwrap().to_string(),
        })
    }
    if args.order.is_some() {
        url_params.push(HttpUrlParam {
            key: "order".to_string(),
            value: args.order.unwrap(),
        })
    }
    if args.depth.is_some() {
        url_params.push(HttpUrlParam {
            key: "depth".to_string(),
            value: args.depth.unwrap(),
        })
    }

    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!("https://api.coingecko.com/api/v3/coins/{}/tickers", args.id),
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
            Some(v) => JSON::from_str::<Tickers>(&v).unwrap(),
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

pub fn coins_history(args: ArgsCoinsHistory) -> History {
    let mut url_params: Vec<HttpUrlParam> = vec![HttpUrlParam {
        key: "date".to_string(),
        value: args.date.to_string(),
    }];

    if args.localization.is_some() {
        url_params.push(HttpUrlParam {
            key: "localization".to_string(),
            value: args.localization.unwrap(),
        })
    }

    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!("https://api.coingecko.com/api/v3/coins/{}/history", args.id),
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

    println!("{:#?}", http_response.body);

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<History>(&v).unwrap(),
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

pub fn coins_market_chart(args: ArgsCoinsMarketChart) -> MarketChart {
    let mut url_params: Vec<HttpUrlParam> = vec![
        HttpUrlParam {
            key: "vs_currency".to_string(),
            value: args.vs_currency,
        },
        HttpUrlParam {
            key: "days".to_string(),
            value: args.days,
        },
    ];

    if args.interval.is_some() {
        url_params.push(HttpUrlParam {
            key: "interval".to_string(),
            value: args.interval.unwrap(),
        })
    }

    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart",
            args.id
        ),
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
            Some(v) => JSON::from_str::<MarketChart>(&v).unwrap(),
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

pub fn coins_market_chart_range(args: ArgsCoinsMarketChartRange) -> MarketChart {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam {
            key: "vs_currency".to_string(),
            value: args.vs_currency,
        },
        HttpUrlParam {
            key: "from".to_string(),
            value: args.from,
        },
        HttpUrlParam {
            key: "to".to_string(),
            value: args.to,
        },
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart/range",
            args.id
        ),
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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<MarketChart>(&v).unwrap(),
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

pub fn coins_contract(args: ArgsCoinsContract) -> Contract {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}",
            args.id, args.contract_address
        ),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Contract>(&v).unwrap(),
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

pub fn coins_contract_market_chart(args: ArgsCoinsContractMarketChart) -> MarketChart {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam {
            key: "vs_currency".to_string(),
            value: args.vs_currency,
        },
        HttpUrlParam {
            key: "days".to_string(),
            value: args.days,
        },
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}/market_chart/",
            args.id, args.contract_address
        ),
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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<MarketChart>(&v).unwrap(),
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

pub fn coins_contract_market_chart_range(args: ArgsCoinsContractMarketChartRange) -> Contract {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam {
            key: "vs_currency".to_string(),
            value: args.vs_currency,
        },
        HttpUrlParam {
            key: "from".to_string(),
            value: args.from,
        },
        HttpUrlParam {
            key: "to".to_string(),
            value: args.to,
        },
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}/market_chart/range",
            args.id, args.contract_address
        ),
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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Contract>(&v).unwrap(),
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

pub fn coins_ohlc(args: ArgsCoinsOhlc) -> Vec<Vec<BigNumber>> {
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
        HttpUrlParam {
            key: "vs_currency".to_string(),
            value: args.vs_currency,
        },
        HttpUrlParam {
            key: "days".to_string(),
            value: args.days,
        },
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!("https://api.coingecko.com/api/v3/coins/{}/ohlc", args.id),
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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Vec<BigNumber>>>(&v).unwrap(),
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

pub fn asset_platforms(args: ArgsAssetPlatforms) -> Vec<AssetPlatform> {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/asset_platforms".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Vec<AssetPlatform>>(&v).unwrap(),
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

pub fn coins_categories_list(args: ArgsCoinsCategoriesList) -> Vec<CategoryId> {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/categories/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Vec<CategoryId>>(&v).unwrap(),
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

pub fn coins_categories(args: ArgsCoinsCategories) -> Vec<Category> {
    let mut url_params: Option<Vec<HttpUrlParam>> = None;

    if args.order.is_some() {
        url_params = Some(vec![HttpUrlParam {
            key: "order".to_string(),
            value: args.order.unwrap(),
        }])
    }

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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Category>>(&v).unwrap(),
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

pub fn exchanges(args: ArgsExchanges) -> Vec<Exchange> {
    let mut url_params: Vec<HttpUrlParam> = vec![];

    if args.per_page.is_some() {
        url_params.push(HttpUrlParam {
            key: "per_page".to_string(),
            value: args.per_page.unwrap().to_string(),
        })
    }
    if args.page.is_some() {
        url_params.push(HttpUrlParam {
            key: "page".to_string(),
            value: args.page.unwrap(),
        })
    }
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/exchanges".to_string(),
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
            Some(v) => JSON::from_str::<Vec<Exchange>>(&v).unwrap(),
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

pub fn exchanges_list(args: ArgsExchangesList) -> Vec<ExchangeId> {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/exchanges/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Vec<ExchangeId>>(&v).unwrap(),
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

pub fn exchange(args: ArgsExchange) -> Exchange {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!("https://api.coingecko.com/api/v3/exchanges/{}", args.id),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Exchange>(&v).unwrap(),
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

pub fn exchanges_tickers(args: ArgsExchangesTickers) -> Tickers {
    let mut url_params: Vec<HttpUrlParam> = vec![];
    if args.coin_ids.is_some() {
        url_params.push(HttpUrlParam {
            key: "coin_ids".to_string(),
            value: args.coin_ids.unwrap(),
        })
    }
    if args.include_exchange_logo.is_some() {
        url_params.push(HttpUrlParam {
            key: "include_exchange_logo".to_string(),
            value: args.include_exchange_logo.unwrap(),
        })
    }
    if args.page.is_some() {
        url_params.push(HttpUrlParam {
            key: "page".to_string(),
            value: args.page.unwrap().to_string(),
        })
    }
    if args.depth.is_some() {
        url_params.push(HttpUrlParam {
            key: "depth".to_string(),
            value: args.depth.unwrap(),
        })
    }
    if args.order.is_some() {
        url_params.push(HttpUrlParam {
            key: "order".to_string(),
            value: args.order.unwrap(),
        })
    }
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/exchanges/{}/tickers",
            args.id
        ),
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
            Some(v) => JSON::from_str::<Tickers>(&v).unwrap(),
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

pub fn indexes(args: ArgsIndexes) -> Vec<Index> {
    let mut url_params: Vec<HttpUrlParam> = vec![];
    if args.per_page.is_some() {
        url_params.push(HttpUrlParam {
            key: "per_page".to_string(),
            value: args.per_page.unwrap().to_string(),
        })
    }
    if args.page.is_some() {
        url_params.push(HttpUrlParam {
            key: "page".to_string(),
            value: args.page.unwrap().to_string(),
        })
    }
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/indexes".to_string(),
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
            Some(v) => JSON::from_str::<Vec<Index>>(&v).unwrap(),
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

pub fn market_indexes(args: ArgsMarketIndexes) -> MarketIndex {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/indexes/{}/{}",
            args.market_id, args.id
        ),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<MarketIndex>(&v).unwrap(),
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

pub fn indexes_list(args: ArgsIndexesList) -> Vec<IndexId> {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/indexes/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Vec<IndexId>>(&v).unwrap(),
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

pub fn derivatives(args: ArgsDerivatives) -> Vec<Derivative> {
    let mut url_params: Option<Vec<HttpUrlParam>> = None;
    if args.include_tickers.is_some() {
        url_params = Some(vec![HttpUrlParam {
            key: "include_tickers".to_string(),
            value: args.include_tickers.unwrap(),
        }])
    };
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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<Vec<Derivative>>(&v).unwrap(),
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

pub fn derivatives_exchanges(args: ArgsDerivativesExchanges) -> Vec<DerivativeExchange> {
    let mut url_params: Vec<HttpUrlParam> = vec![];
    if args.order.is_some() {
        url_params.push(HttpUrlParam {
            key: "order".to_string(),
            value: args.order.unwrap(),
        })
    }
    if args.per_page.is_some() {
        url_params.push(HttpUrlParam {
            key: "per_page".to_string(),
            value: args.per_page.unwrap().to_string(),
        })
    }
    if args.page.is_some() {
        url_params.push(HttpUrlParam {
            key: "page".to_string(),
            value: args.page.unwrap().to_string(),
        })
    }
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/derivatives/exchanges".to_string(),
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
            Some(v) => JSON::from_str::<Vec<DerivativeExchange>>(&v).unwrap(),
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

pub fn derivatives_exchange(args: ArgsDerivativesExchange) -> DerivativeExchange {
    let mut url_params: Option<Vec<HttpUrlParam>> = None;
    if args.include_tickers.is_some() {
        url_params = Some(vec![HttpUrlParam {
            key: "include_tickers".to_string(),
            value: args.include_tickers.unwrap(),
        }])
    }
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/derivatives/exchanges/{}",
            args.id
        ),
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
            None => "An unknown error occurred!".to_string(),
        };

        panic!("Error {}: {}", http_response.status, response_body)
    }

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return match http_response.body {
            Some(v) => JSON::from_str::<DerivativeExchange>(&v).unwrap(),
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

pub fn derivatives_exchanges_list(args: ArgsDerivativesExchangesList) -> Vec<DerivativeExchangeId> {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/derivatives/exchanges/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Vec<DerivativeExchangeId>>(&v).unwrap(),
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

// pub fn exchanges_volume_chart(args: ArgsExchangesVolumeChart) -> ExchangesVolumeChart {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "days".to_string(), value: args.days.to_string() }
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: format!("https://api.coingecko.com/api/v3/exchanges/{}/volume_chart", args.id),
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

// pub fn exchange_rates(args: ArgsExchangeRates) -> ExchangeRates {
//    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//        url: "https://api.coingecko.com/api/v3/exchange_rates".to_string(),
//        request: Some(HttpRequest {
//            headers: None,
//            url_params: None,
//            response_type: HttpResponseType::TEXT,
//            body: None,
//        }),
//    }) {
//        Ok(Some(v)) => v,
//        Ok(None) => panic!("Did not receive HTTP response"),
//        Err(e) => panic!("{}", e),
//    };

//    // handle json rpc error
//    if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
//        let response_body: String = match http_response.body {
//            Some(v) => v,
//            None => "An unknown error occurred!".to_string()
//        };

//        panic!("Error {}: {}", http_response.status, response_body)
//    }

//    // handle json rpc success
//    if http_response.status >= 200 && http_response.status <= 299 {
//        return match http_response.body {
//            Some(v) => JSON::from_str::<ExchangeRates>(&v).unwrap(),
//            None => panic!("Missing response with successful HTTP status {}", http_response.status)
//        };
//    }

//    panic!("Unexpected HTTP response with status: {}", http_response.status);

// pub fn search(args: ArgsSearch) -> Search {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam { key: "query".to_string(), value: args.query }
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
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/search/trending".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<SearchTrending>(&v).unwrap(),
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

pub fn global(args: ArgsGlobal) -> Global {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/global".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<Global>(&v).unwrap(),
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

pub fn global_decentralized_finance_defi(args: ArgsGlobalDecentralizedFinanceDefi) -> GlobalDefi {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/global/decentralized_finance_defi".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<GlobalDefi>(&v).unwrap(),
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

pub fn companies_public_treasury(args: ArgsCompaniesPublicTreasury) -> CompaniesPublicTreasury {
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
        url: format!(
            "https://api.coingecko.com/api/v3/companies/public_treasury/{}",
            args.coin_id
        ),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
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
            Some(v) => JSON::from_str::<CompaniesPublicTreasury>(&v).unwrap(),
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
