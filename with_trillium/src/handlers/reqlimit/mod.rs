use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    num::NonZeroUsize,
    sync::Mutex,
    time::Duration,
};

use lru::LruCache;

use ratelimit::Ratelimiter;
use trillium::{Conn, Handler, Status};

// keeps an LRU set
pub struct Limiter {
    addrs_set: Mutex<LruCache<IpAddr, Ratelimiter>>,
}

impl Limiter {
    pub fn new() -> Self {
        Limiter {
            addrs_set: Mutex::new(LruCache::new(NonZeroUsize::new(2).unwrap())),
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
                let limiter = Ratelimiter::new(1, 1, 1);
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
