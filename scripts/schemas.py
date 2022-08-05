argSchema = "{name}: {type}"

methodBlueprintSchema = '''
{name}: {returnType}
"""
{docstring}
"""
'''

methodSchema = '''
{name}(
    """
    {docstring}
    """
{args}
): {returnType}
'''

moduleSchema = '''#import * from "./types.graphql"

type Module {{
    """
    CoinGecko API V3
    """
{methods}
}}'''


libSchema = '''pub mod wrap;

pub use wrap::*;
use polywrap_wasm_rs::JSON;
use crate::imported::http_module;

{functions}
'''

urlParamSchema = '''HttpUrlParam {{ key: "{key}".to_string(), value: "{value}".to_string() }}'''

funcSchema = '''pub fn {name}(args: {args}) -> {returnType} {{
    let url_params: Option<Vec<HttpUrlParam>> = Some(vec![
{params}
    ]);
    let http_response: HttpResponse = match HttpModule::get(&http_module::ArgsGet {{
        url: "https://api.coingecko.com/api/v3{path}".to_string(),
        request: Some(HttpRequest {{
            headers: None,
            url_params: url_params,
            response_type: HttpResponseType::TEXT,
            body: None,
        }}),
    }}) {{
        Ok(Some(v)) => v,
        Ok(None) => panic!("Did not receive HTTP response"),
        Err(e) => panic!("{{}}", e),
    }};

    // handle json rpc error
    if http_response.status == 400 || http_response.status == 404 || http_response.status == 500 {{
        let response_body: String = match http_response.body {{
            Some(v) => v,
            None => "An unknown error occurred!".to_string()
        }};

        panic!("Error {{}}: {{}}", http_response.status, response_body)
    }}

    // handle json rpc success
    if http_response.status >= 200 && http_response.status <= 299 {{
        return match http_response.body {{
            Some(v) => JSON::from_str::<{returnType}>(&v).unwrap(),
            None => panic!("Missing response with successful HTTP status {{}}", http_response.status)
        }};
    }}

    panic!("Unexpected HTTP response with status: {{}}", http_response.status);
}}'''
