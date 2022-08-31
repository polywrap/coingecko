use polywrap_wasm_rs::{Map, JSON};

use crate::wrap::*;
use crate::imported::http_module;

pub fn ping(_args: ArgsPing) -> Ping {
  let url_params: Option<Map<String, String>> = Some(Map::new());

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
