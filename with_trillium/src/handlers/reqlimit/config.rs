use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IPCache {
    pub cache_capacity: usize,
    pub limiter: Bucket,
}

#[derive(Deserialize, Debug)]
pub struct Bucket {
    pub capacity: u64,
    pub quantum: u64,
    pub rate: u64,
}
