use std::sync::Arc;
use redis::Commands;
use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};

use super::utils;

pub struct Session {
    redis: Arc<redis::Client>
}

impl Session {
    pub fn new(redis: &Arc<redis::Client>) -> Session {
        Session {
            redis: redis.clone()
        }
    }

    pub fn start(&self, user_id: String) -> String {
        let mut con = self.redis.get_connection().expect("Error getting Redis connection.");

        let rng = thread_rng();
        let token: String = rng.sample_iter(Alphanumeric).take(32).collect();

        // 72h session
        con.set_ex::<String, &str, usize>(format!("heim:session:{}", token), &user_id, 60 * 60 * 72).unwrap();

        format!("token_id={}; expires={}", token, utils::long_expiry_time("%a, %d %b %Y %H:%M:%S 'GMT'"))
    }

    pub fn exists(&self, session_id: String) -> Option<String> {
        if session_id.len() != 32 {
            return None;
        }

        let mut con = self.redis.get_connection().expect("Error getting Redis connection.");
        con.get(format!("heim:session:{}", session_id)).unwrap()
    }
}
