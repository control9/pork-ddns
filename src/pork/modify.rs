use crate::options::Options;
use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};



#[derive(Serialize)]
struct ModifyRequest<'a> {
    #[serde(rename = "secretapikey")] // Why, porkbun, WHY? :(
    secret_api_key: &'a str,
    #[serde(rename = "apikey")]
    api_key: &'a str,
    content: &'a Ipv4Addr,
    ttl: &'a Option<u64>
}

#[derive(Deserialize)]
struct ModifyResponse {
    status: String
}

pub fn modify<'a>(
    client: &reqwest::blocking::Client,
    Options{secret_api_key, api_key, domain, subdomain, ttl, ..} : &Options,
    ip_address: &Ipv4Addr,
) -> Result<(), &'a str> {

    let url = format!("https://porkbun.com/api/json/v3/dns/editByNameType/{domain}/A/{sub}", sub = &subdomain.as_deref().unwrap_or(""));
    let req = ModifyRequest{secret_api_key, api_key, content: ip_address, ttl};

    log::info!("Using modify call to set {sub}{domain} to {ip_address}", sub = subdomain.as_ref().map(|x| x.to_owned() + ".").unwrap_or("".to_string()));

    let resp = client.post(url)
        .json(&req)
        .send();
    log::info!("Response: {:?}", resp);
    log::info!("Response body: {:?}", resp.expect("aaa").text());
    Ok(())
}