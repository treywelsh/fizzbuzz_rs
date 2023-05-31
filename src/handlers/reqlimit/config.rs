use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};
//use toml::de::Error;

#[derive(Deserialize, Debug)]
pub struct IPCache {
    #[serde(deserialize_with = "validate_cache_capacity")]
    pub cache_capacity: usize,
    #[serde(deserialize_with = "validate_bucket")]
    pub limiter: Bucket,
}

#[derive(Deserialize, Debug)]
pub struct Bucket {
    pub capacity: u64,
    pub quantum: u64,
    pub rate: u64,
}

fn validate_cache_capacity<'de, D>(d: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let value = usize::deserialize(d)?;

    if value < 1 {
        return Err(Error::invalid_value(
            Unexpected::Unsigned(value as u64),
            &"a value at least 1",
        ));
    }

    Ok(value)
}

fn validate_bucket<'de, D>(d: D) -> Result<Bucket, D::Error>
where
    D: Deserializer<'de>,
{
    let cfg = Bucket::deserialize(d)?;

    if cfg.capacity < 1 {
        return Err(Error::invalid_value(
            Unexpected::Unsigned(cfg.capacity),
            &"a value at least 1 for bucket capacity",
        ));
    }

    if cfg.quantum < 1 {
        return Err(Error::invalid_value(
            Unexpected::Unsigned(cfg.capacity),
            &"a value at least 1 for quantum",
        ));
    }

    if cfg.rate < 1 {
        return Err(Error::invalid_value(
            Unexpected::Unsigned(cfg.capacity),
            &"a value at least 1 for rate",
        ));
    }

    if cfg.rate < cfg.quantum {
        return Err(Error::invalid_value(
            Unexpected::Other("quantum is lesser than rate"),
            &"rate has to be greater or equal than quantum",
        ));
    }

    Ok(cfg)
}
