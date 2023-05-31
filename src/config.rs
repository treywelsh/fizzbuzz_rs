use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};
use std::fs;

use crate::errors::Errors;
use crate::handlers::reqlimit::config::{Bucket, IPCache};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    #[serde(deserialize_with = "validate_max_conn")]
    pub max_conn: Option<usize>,
    pub requests: Option<IPCache>,
    pub tls: Option<Tls>,
}

#[derive(Deserialize, Debug)]
pub struct Tls {
    pub cert: String,
    pub key: String,
}

fn validate_max_conn<'de, D>(d: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<usize>::deserialize(d)?;

    if let Some(value) = opt {
        if value < 2 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(value as u64),
                &"a value at least 2",
            ));
        }
    }

    Ok(opt)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_owned(),
            port: 8080,
            max_conn: Some(1000),
            requests: Some(IPCache {
                cache_capacity: 500,
                limiter: Bucket {
                    capacity: 10,
                    quantum: 1,
                    rate: 10,
                },
            }),
            tls: None,
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

    Ok(config)
}
