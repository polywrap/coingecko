use polywrap_wasm_rs::{Map, JSON};

use crate::imported::http_module;
use crate::wrap::*;

pub fn simple_supported_vs_currencies(args: ArgsSimpleSupportedVsCurrencies) -> Vec<String> {
    let url_params: Option<Map<String, String>> = Some(Map::new());

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
