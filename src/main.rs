mod options;
mod pork;

use clap::Parser;
use options::Options;
use serde::{Serialize, Deserialize};


fn main() {
    let opts = Options::parse();
    simple_logger::init_with_env().unwrap();
    let client = reqwest::blocking::Client::new();
    log::warn!("App started!");

    let auth = pork::authenticate::authenticate(&client, &opts);
    if let Ok(ip) = auth {
        log::info!("Current ip address is: {:?}", ip);
        pork::modify::modify(&client, &opts, &ip);
    };
}



