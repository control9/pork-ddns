use std::net::Ipv4Addr;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

const AUTHENTICATE_ENDPOINT: &str = "/api/json/v3/ping";

#[derive(Serialize)]
struct AuthenticateRequest<'a> {
    #[serde(rename = "secretapikey")] // Why, porkbun, WHY? :(
    secret_api_key: &'a str,
    #[serde(rename = "apikey")]
    api_key: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticateResponse {
    status: String,
    your_ip: Ipv4Addr,
}

pub fn authenticate<'a>(
    client: &reqwest::blocking::Client,
    host: &str,
    secret_api_key: &str,
    api_key: &str,
) -> Result<Ipv4Addr, &'a str> {
    log::info!("Using authenticate call to get current external IP address");
    let req: &AuthenticateRequest = &AuthenticateRequest { secret_api_key, api_key };
    let addr = format!("{}{}", host, AUTHENTICATE_ENDPOINT);
    log::debug!("Address is {}", addr);
    let resp = client.post(addr)
        .json(req)
        .send();
    log::info!("Authenticate result was {:?}", resp );
    if let Err(error) = &resp {
        log::error!("Error while making request to authenticate: {}", error)
    }
    let result = resp.and_then(|r| r.json::<AuthenticateResponse>())
        .and_then(|ar| Ok(ar.your_ip));
    result.map_err(|_| "Error while calling authenticate")
}

#[cfg(test)]
use httpmock::prelude::*;

#[cfg(test)]
use serde_json::{json, Value};

#[test]
fn test_happy_path() {
    simple_logger::init_with_env().unwrap();

    let secret_api_key = "my_secret_api_key";
    let api_key = "my_api_key";
    let ip = "192.0.2.42";

    let server = MockServer::start();
    let host : &str = &format!("http://{}", server.address());

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(AUTHENTICATE_ENDPOINT)
            .json_body(json!({
                "secretapikey": secret_api_key,
                "apikey": api_key
            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                  "status": "SUCCESS",
                  "yourIp": ip
            }));
    });

    let result = authenticate( &reqwest::blocking::Client::new(), host, secret_api_key, api_key);

    mock.assert();
    let expected_ip = Ipv4Addr::from_str(ip).unwrap();
    assert_eq!(result, Ok(expected_ip))
}