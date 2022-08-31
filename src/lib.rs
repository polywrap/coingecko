pub mod routes;
pub mod utils;
pub mod wrap;

use crate::imported::http_module;
use polywrap_wasm_rs::{BigNumber, Map, JSON};

pub use utils::*;
pub use wrap::*;

/***********************************************************************
*                                 PING                                 *
***********************************************************************/

pub fn ping(_: ArgsPing) -> Ping {
    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/ping".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Ping>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

/***********************************************************************
*                                SIMPLE                                *
***********************************************************************/

pub fn simple_price(args: ArgsSimplePrice) -> Map<String, Map<String, Option<BigNumber>>> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("ids", args.ids);
    url_params.add("vs_currencies", args.vs_currencies);
    url_params.add(
        "include_market_cap",
        bool_to_string(args.include_market_cap),
    );
    url_params.add("include_24hr_vol", bool_to_string(args.include_24hr_vol));
    url_params.add(
        "include_24hr_change",
        bool_to_string(args.include_24hr_change),
    );
    url_params.add(
        "include_last_updated_at",
        bool_to_string(args.include_last_updated_at),
    );

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/simple/price".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Map<String, Map<String, Option<BigNumber>>>>(&response_body)
            .unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn simple_token_price(
    args: ArgsSimpleTokenPrice,
) -> Map<String, Map<String, Option<BigNumber>>> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("contract_addresses", args.contract_addresses);
    url_params.add("vs_currencies", args.vs_currencies);
    url_params.add(
        "include_market_cap",
        bool_to_string(args.include_market_cap),
    );
    url_params.add("include_24hr_vol", bool_to_string(args.include_24hr_vol));
    url_params.add(
        "include_24hr_change",
        bool_to_string(args.include_24hr_change),
    );
    url_params.add(
        "include_last_updated_at",
        bool_to_string(args.include_last_updated_at),
    );

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/simple/token_price/{}",
            args.id
        )
    };

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Map<String, Map<String, Option<BigNumber>>>>(&response_body)
            .unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn simple_supported_vs_currencies(_: ArgsSimpleSupportedVsCurrencies) -> Vec<String> {
    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/simple/supported_vs_currencies".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Vec<String>>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

/***********************************************************************
*                                COINS                                 *
***********************************************************************/

pub fn coins_list(args: ArgsCoinsList) -> Vec<CoinListItem> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("include_platform", bool_to_string(args.include_platform));

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/list".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Vec<CoinListItem>>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn coins_markets(args: ArgsCoinsMarkets) -> Vec<CoinMarketItem> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.insert("vs_currency".to_string(), args.vs_currency);
    url_params.add("ids", args.ids);
    url_params.add("category", args.category);
    url_params.add("order", args.order);
    url_params.add("per_page", int_to_string(args.per_page));
    url_params.add("page", int_to_string(args.page));
    url_params.add("sparkline", bool_to_string(args.sparkline));
    url_params.add("price_change_percentage", args.price_change_percentage);

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: "https://api.coingecko.com/api/v3/coins/markets".to_string(),
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Vec<CoinMarketItem>>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn coin(args: ArgsCoin) -> CoinsItem {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("localization", args.localization);
    url_params.add("tickers", bool_to_string(args.tickers));
    url_params.add("market_data", bool_to_string(args.market_data));
    url_params.add("community_data", bool_to_string(args.community_data));
    url_params.add("developer_data", bool_to_string(args.developer_data));
    url_params.add("sparkline", bool_to_string(args.sparkline));

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}",
            args.id.as_str()
        )
    };

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<CoinsItem>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn coin_tickers(args: ArgsCoinTickers) -> Tickers {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("exchange_ids", args.exchange_ids);
    url_params.add(
        "include_exchange_logo",
        bool_to_string(args.include_exchange_logo),
    );
    url_params.add("page", int_to_string(args.page));
    url_params.add("order", args.order);
    url_params.add("depth", args.depth);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/tickers",
            args.id.as_str()
        )
    };

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Tickers>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn coin_history(args: ArgsCoinHistory) -> History {
    let mut url_params: Map<String, String> = Map::new();
    url_params.insert("data".to_string(), args.date);
    url_params.add("localization", args.localization);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/history",
            args.id.as_str()
        )
    };

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<History>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn coin_market_chart(args: ArgsCoinMarketChart) -> MarketChart {
    let mut url_params: Map<String, String> = Map::new();
    url_params.insert("vs_currency".to_string(), args.vs_currency);
    url_params.insert("days".to_string(), args.days);
    url_params.add("interval", args.interval);


    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart",
            args.id.as_str()
        )
    };

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<MarketChart>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn coin_market_chart_range(args: ArgsCoinMarketChartRange) -> MarketChart {
    let mut url_params: Map<String, String> = Map::new();
    url_params.insert("vs_currency".to_string(), args.vs_currency);
    url_params.insert("from".to_string(), args.from);
    url_params.insert("to".to_string(), args.to);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart/range",
            args.id.as_str()
        )
    };

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<MarketChart>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

pub fn coin_ohlc(args: ArgsCoinOhlc) -> Vec<Vec<BigNumber>> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.insert("vs_currency".to_string(), args.vs_currency);
    url_params.insert("days".to_string(), args.days);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/ohlc",
            args.id.as_str()
        )
    };

    let http_response: HttpResponse = HttpModule::get(&http_module::ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: Some(url_params),
            response_type: HttpResponseType::TEXT,
            body: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<Vec<Vec<BigNumber>>>(&response_body).unwrap();
    }
    unsafe {
        panic!(
            "Unexpected HTTP response: {} with status: {}",
            response_body, http_response.status
        );
    }
}

/***********************************************************************
*                               CONTRACT                               *
***********************************************************************/

// pub fn coins_contract(args: ArgsCoinsContract) -> Contract {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "id".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "contract_address".to_string(),
//             value: "args.name".to_string(),
//         },
//     ]);
//     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
//         url: "https://api.coingecko.com/api/v3/coins/args.id/contract/args.contract_address"
//             .to_string(),
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Contract>(&v).unwrap(),
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

// pub fn coins_contract_market_chart(args: ArgsCoinsContractMarketChart) -> MarketChart {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "id".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "contract_address".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "vs_currency".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "days".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<MarketChart>(&v).unwrap(),
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

// pub fn coins_contract_market_chart_range(args: ArgsCoinsContractMarketChartRange) -> Contract {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "id".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "contract_address".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "vs_currency".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "from".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "to".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Contract>(&v).unwrap(),
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

// pub fn asset_platforms(args: ArgsAssetPlatforms) -> Vec<AssetPlatform> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<AssetPlatform>>(&v).unwrap(),
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

// pub fn coins_categories_list(args: ArgsCoinsCategoriesList) -> Vec<CategoryId> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<CategoryId>>(&v).unwrap(),
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

// pub fn coins_categories(args: ArgsCoinsCategories) -> Vec<Category> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![HttpUrlParam {
//         key: "order".to_string(),
//         value: "args.name".to_string(),
//     }]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<Category>>(&v).unwrap(),
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

// pub fn exchanges(args: ArgsExchanges) -> Vec<Exchange> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "per_page".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "page".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<Exchange>>(&v).unwrap(),
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

// pub fn exchanges_list(args: ArgsExchangesList) -> Vec<ExchangeId> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<ExchangeId>>(&v).unwrap(),
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

// pub fn exchange(args: ArgsExchange) -> Exchange {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![HttpUrlParam {
//         key: "id".to_string(),
//         value: "args.name".to_string(),
//     }]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Exchange>(&v).unwrap(),
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

// pub fn exchanges_tickers(args: ArgsExchangesTickers) -> Tickers {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "id".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "coin_ids".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "include_exchange_logo".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "page".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "depth".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "order".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Tickers>(&v).unwrap(),
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

// pub fn indexes(args: ArgsIndexes) -> Vec<Index> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "per_page".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "page".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<Index>>(&v).unwrap(),
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

// pub fn market_indexes(args: ArgsMarketIndexes) -> MarketIndex {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "market_id".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "id".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<MarketIndex>(&v).unwrap(),
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

// pub fn indexes_list(args: ArgsIndexesList) -> Vec<IndexId> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<IndexId>>(&v).unwrap(),
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

// pub fn derivatives(args: ArgsDerivatives) -> Vec<Derivative> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![HttpUrlParam {
//         key: "include_tickers".to_string(),
//         value: "args.name".to_string(),
//     }]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<Derivative>>(&v).unwrap(),
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

// pub fn derivatives_exchanges(args: ArgsDerivativesExchanges) -> Vec<Derivative> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "order".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "per_page".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "page".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<Derivative>>(&v).unwrap(),
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

// pub fn derivatives_exchange(args: ArgsDerivativesExchange) -> Derivative {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
//         HttpUrlParam {
//             key: "id".to_string(),
//             value: "args.name".to_string(),
//         },
//         HttpUrlParam {
//             key: "include_tickers".to_string(),
//             value: "args.name".to_string(),
//         },
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Derivative>(&v).unwrap(),
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

// pub fn derivatives_exchanges_list(args: ArgsDerivativesExchangesList) -> Vec<DerivativeExchangeId> {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Vec<DerivativeExchangeId>>(&v).unwrap(),
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

// // pub fn exchangesVolumeChart(args: ArgsExchangesVolumeChart) -> ExchangesVolumeChart {
// //     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
// //         HttpUrlParam { key: "id".to_string(), value: "args.name".to_string() },
// //         HttpUrlParam { key: "days".to_string(), value: "args.name".to_string() }
// //     ]);
// //     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
// //         url: "https://api.coingecko.com/api/v3/exchanges/args.id/volume_chart".to_string(),
// //         request: Some(HttpRequest {
// //             headers: None,
// //             url_params: url_params,
// //             response_type: HttpResponseType::TEXT,
// //             body: None,
// //         }),
// //     }) {
// //         Ok(Some(v)) => v,
// //         Ok(None) => panic!("Did not receive HTTP response"),
// //         Err(e) => panic!("{}", e),
// //     };

// //     // handle json rpc error
// //     if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
// //         let response_body: String = match http_response.body {
// //             Some(v) => v,
// //             None => "An unknown error occurred!".to_string()
// //         };

// //         panic!("Error {}: {}", http_response.status, response_body)
// //     }

// //     // handle json rpc success
// //     if http_response.status >= 200 && http_response.status <= 299 {
// //         return match http_response.body {
// //             Some(v) => JSON::from_str::<ExchangesVolumeChart>(&v).unwrap(),
// //             None => panic!("Missing response with successful HTTP status {}", http_response.status)
// //         };
// //     }

// //     panic!("Unexpected HTTP response with status: {}", http_response.status);
// // }

// pub fn exchange_rates(args: ArgsExchangeRates) -> ExchangeRates {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<ExchangeRates>(&v).unwrap(),
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

// // pub fn search(args: ArgsSearch) -> Search {
// //     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
// //         HttpUrlParam { key: "query".to_string(), value: "args.name".to_string() }
// //     ]);
// //     let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {
// //         url: "https://api.coingecko.com/api/v3/search".to_string(),
// //         request: Some(HttpRequest {
// //             headers: None,
// //             url_params: url_params,
// //             response_type: HttpResponseType::TEXT,
// //             body: None,
// //         }),
// //     }) {
// //         Ok(Some(v)) => v,
// //         Ok(None) => panic!("Did not receive HTTP response"),
// //         Err(e) => panic!("{}", e),
// //     };

// //     // handle json rpc error
// //     if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {
// //         let response_body: String = match http_response.body {
// //             Some(v) => v,
// //             None => "An unknown error occurred!".to_string()
// //         };

// //         panic!("Error {}: {}", http_response.status, response_body)
// //     }

// //     // handle json rpc success
// //     if http_response.status >= 200 && http_response.status <= 299 {
// //         return match http_response.body {
// //             Some(v) => JSON::from_str::<Search>(&v).unwrap(),
// //             None => panic!("Missing response with successful HTTP status {}", http_response.status)
// //         };
// //     }

// //     panic!("Unexpected HTTP response with status: {}", http_response.status);
// // }

// pub fn search_trending(args: ArgsSearchTrending) -> SearchTrending {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<SearchTrending>(&v).unwrap(),
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

// pub fn global(args: ArgsGlobal) -> Global {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<Global>(&v).unwrap(),
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

// pub fn global_decentralized_finance_defi(args: ArgsGlobalDecentralizedFinanceDefi) -> GlobalDefi {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<GlobalDefi>(&v).unwrap(),
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

// pub fn companies_public_treasury(args: ArgsCompaniesPublicTreasury) -> CompaniesPublicTreasury {
//     let url_params: Option<Vec<HttpUrlParam>> = Some(vec![HttpUrlParam {
//         key: "coin_id".to_string(),
//         value: "args.name".to_string(),
//     }]);
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
//             None => "An unknown error occurred!".to_string(),
//         };

//         panic!("Error {}: {}", http_response.status, response_body)
//     }

//     // handle json rpc success
//     if http_response.status >= 200 && http_response.status <= 299 {
//         return match http_response.body {
//             Some(v) => JSON::from_str::<CompaniesPublicTreasury>(&v).unwrap(),
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
