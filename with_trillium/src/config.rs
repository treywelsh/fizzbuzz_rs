use serde::Deserialize;
use std::fs;
use toml::Value;

use crate::handlers::reqlimit::config::IPCache;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub max_conn: Option<usize>,
    pub requests: Option<IPCache>,
}

pub fn read(path: &str) -> Config {
    let data = fs::read_to_string(path).unwrap();
    let config = toml::from_str(&data).unwrap();
    log::debug!("configuration: {:?}", config);

    config
}
