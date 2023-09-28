 use crate::imported::http_module::*;
use crate::imported::http_request::*;
use crate::imported::http_response::*;
use crate::imported::http_response_type::*;
use polywrap_wasm_rs::{Map, JSON};
use serde::de::DeserializeOwned;

pub fn call_api<T: DeserializeOwned>(url: String, url_params: Option<Map<String, String>>) -> Result<T, String> {
    let http_response: HttpResponse = HttpModule::get(&ArgsGet {
        url: url,
        request: Some(HttpRequest {
            headers: None,
            url_params: url_params,
            response_type: HttpResponseType::TEXT,
            body: None,
            form_data: None,
            timeout: None,
        }),
    })
    .expect("Received an error as HTTP Response")
    .expect("Received an empty HTTP Response");

    let response_body: String = http_response
        .body
        .expect("Received an empty body as HTTP Response");

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {
        return JSON::from_str::<T>(&response_body).map_err(|e| e.to_string());
    }
    Err(format!(
        "Unexpected HTTP response: {} with status: {}",
        response_body, http_response.status
    ))
}
