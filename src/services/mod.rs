use std::sync::Arc;

use super::config;
use super::cache;
use super::router;
use super::session;

pub struct Services {
    pub config: Arc<config::Config>,
    pub redis: Arc<redis::Client>,
    pub data: Arc<dashmap::DashMap<String, cache::Route>>,
    pub router: Arc<router::Router>,
    pub session: Arc<session::Session>,
}

pub fn new(routes: Vec<router::Route>) -> Services {
    let config = Arc::new(config::Config::new());
    let redis = Arc::new(cache::new(&config));
    let data = Arc::new(dashmap::DashMap::new());
    let router = Arc::new(router::Router::new(routes));
    let session = Arc::new(session::Session::new(&redis));
    cache::watch(&data, &redis);

    Services {
        config,
        redis,
        data,
        router,
        session,
    }
}
