use std::sync::Arc;

use argonautica::{Hasher, Verifier};

use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};

use super::config;

pub struct SecureHash {
    hash: String,
    salt: String
}

pub fn new_hash(value: &str, config: &Arc<config::Config>) -> SecureHash {
    let rng = thread_rng();

    let salt: String = rng.sample_iter(Alphanumeric).take(12).collect();

    let hash = Hasher::default()
        .with_password(value)
        .with_secret_key(&config.heimdall_secret)
        .with_salt(&salt)
        .hash()
        .unwrap();
    
    SecureHash {
        hash,
        salt: salt.to_string()
    }
}

pub fn verify_hash(value: &str, hash: &str, config: &Arc<config::Config>) -> bool {
    Verifier::default()
        .with_hash(hash)
        .with_password(value)
        .with_secret_key(&config.heimdall_secret)
        .verify()
        .unwrap()
}
