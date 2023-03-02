use std::{net::IpAddr, num::NonZeroUsize, sync::Mutex};

use lru::LruCache;

use ratelimit::Ratelimiter;
use trillium::{Conn, Handler, Status};

use self::config::Bucket;

pub mod config;

// keeps an LRU set
pub struct Limiter {
    bucket_conf: Bucket,
    addrs_set: Mutex<LruCache<IpAddr, Ratelimiter>>,
}

impl Limiter {
    pub fn new(config: config::IPCache) -> Self {
        Limiter {
            bucket_conf: config.limiter,
            addrs_set: Mutex::new(LruCache::new(
                NonZeroUsize::new(config.cache_capacity).unwrap(),
            )),
        }
    }
}

#[trillium::async_trait]
impl Handler for Limiter {
    async fn run(&self, conn: Conn) -> Conn {
        if let Some(ip) = conn.peer_ip() {
            let mut got_token = false;

            let mut addrs = self.addrs_set.lock().unwrap();

            if let Some(limiter) = addrs.get(&ip) {
                log::debug!("got IP: {:?}", ip);

                if limiter.try_wait().is_ok() {
                    got_token = true;
                }
            } else {
                let limiter = Ratelimiter::new(
                    self.bucket_conf.capacity,
                    self.bucket_conf.quantum,
                    self.bucket_conf.rate,
                );
                if limiter.try_wait().is_ok() {
                    got_token = true;
                }
                log::debug!("add ip: {:?}", ip);
                addrs.put(ip, limiter);
            }

            if !got_token {
                conn.with_status(Status::TooManyRequests).halt()
            } else {
                conn
            }
        } else {
            log::info!("limiter can't get the IP of this conn: {:?}", conn);
            conn
        }
    }
}
