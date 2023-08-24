use std::net::Ipv4Addr;
use crate::options::Options;
use serde::{Serialize, Deserialize};

/// IPv4 specific endpoint, feel free to use api.porkbun.com if IPv6 address is desired.

#[cfg(not(test))]
const AUTHENTICATE_HOST: &str = "https://api-ipv4.porkbun.com";

#[cfg(test)]
const AUTHENTICATE_HOST: &str = "https://api-ipv4.porkbun.com";

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
    Options{secret_api_key, api_key,..} : &Options
) -> Result<Ipv4Addr, &'a str> {

    log::info!("Using authenticate call to get current external IP address");
    let req : &AuthenticateRequest = &AuthenticateRequest{secret_api_key, api_key};
    let resp = client.post(AUTHENTICATE_ENDPOINT)
        .json(req)
        .send();
    log::info!("Authenticate result was {:?}", resp );
    if let Err(error) = &resp {
        log::error!("Error while making request to authenticate: {}", error)
    }
    let result = resp.and_then( |r|  r.json::<AuthenticateResponse>())
        .and_then(|ar| Ok(ar.your_ip));
    result.map_err(|_| "Error while calling authenticate")
}