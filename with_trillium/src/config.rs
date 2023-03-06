use serde::Deserialize;
use std::fs;

use crate::errors::Errors;
use crate::handlers::reqlimit::config::{Bucket, IPCache};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub max_conn: Option<usize>,
    pub requests: Option<IPCache>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            ip: "127.0.0.1".to_owned(),
            port: 8080,
            max_conn: Some(1000),
            requests: Some(IPCache {
                cache_capacity: 500,
                limiter: Bucket {
                    capacity: 10,
                    quantum: 10,
                    rate: 1,
                },
            }),
        }
    }
}

pub fn read(path: &str) -> Result<Config, Errors> {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            return Err(e.into());
        }
    };
    let config = match toml::from_str(&data) {
        Ok(config) => config,
        Err(e) => {
            return Err(e.into());
        }
    };
    log::debug!("configuration: {:?}", config);

    Ok(config)
}
