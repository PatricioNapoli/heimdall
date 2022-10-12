use std::{process};
use log::{info, error};
use std::time::Duration;
use std::sync::{Arc};

use dashmap::DashMap;

use redis::Commands;

use serde::Deserialize;

use tokio::task;

use super::config;

#[derive(Deserialize)]
pub struct Route {
    #[serde(rename = "gema.service")]
    #[serde(default)]
    pub service: String,

    #[serde(rename = "gema.proto")]
    #[serde(default)]
    pub protocol: String,

    #[serde(rename = "gema.port")]
    #[serde(default)]
    pub port: String,

    #[serde(rename = "gema.auth")]
    #[serde(default)]
    pub auth: String,

    #[serde(rename = "gema.access_level")]
    #[serde(default)]
    pub access_level: String,

    #[serde(rename = "gema.domain")]
    #[serde(default)]
    pub domain: String,

    #[serde(rename = "gema.subdomain")]
    #[serde(default)]
    pub subdomain: String,

    #[serde(rename = "gema.path")]
    #[serde(default)]
    pub path: String,

    #[serde(rename = "gema.cors")]
    #[serde(default)]
    pub cors: String
}

pub fn watch(_cache: &Arc<DashMap<String, Route>>, _redis: &Arc<redis::Client>) {
    let mut cache = _cache.clone();
    let mut redis = _redis.clone();

    let insert = |mut svc: String, cache: &Arc<dashmap::DashMap<String, Route>>| {
        match simd_json::serde::from_str::<Route>(&mut svc) {
            Ok(svc_parsed) => {
                info!("Accepting service: {}", svc_parsed.service);
                &cache.insert(svc_parsed.domain.clone(), svc_parsed);
            },
            Err(err) => error!("Failed parsing service: {}, {}", err, svc)
        }
    };

    task::spawn(async move {
        let mut con = (&redis).get_connection().expect("Error getting Redis connection.");
        let svcs: Vec<String> = con.keys("service:*").expect("Failed to fetch Redis service keys.");

        info!("Fetching services...");

        if svcs.len() == 1 {
            let svc: String = con.get(svcs).expect("Failed to get Redis service values.");
            insert(svc, &cache);
        } else {
            let svcs_vals: Vec<String> = con.get(svcs).expect("Failed to multiget Redis service values.");
            for svc in svcs_vals {
                insert(svc, &cache);
            }
        }
    });

    cache = _cache.clone();
    redis = _redis.clone();
    
    task::spawn(async move {
        let mut con = (&redis).get_connection().expect("Error getting Redis connection.");

        let mut pubsub = con.as_pubsub();
        pubsub.subscribe("events:service").unwrap();

        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();

            insert(payload, &cache);
        }
    });
}

pub fn new(config: &config::Config) -> redis::Client {
    info!("Connecting to Redis...");
    
    let redis: redis::Client;
    match redis::Client::open(format!("redis://{}/", config.heimdall_redis_host)) {
        Ok(client) => {
            redis = client;

            match redis.get_connection_with_timeout(Duration::new(5, 0)) {
                Ok(_) => info!("Connected!"),
                Err(error) => {
                    error!("Redis error: {}", error);
                    process::exit(1);
                }
            }
        }
        Err(_) => {
            error!("Error while connecting to redis.. exiting.");
            process::exit(1);
        }
    }
    redis
}
