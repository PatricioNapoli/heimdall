use log::error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub environment: String,
  pub heimdall_hq: String,
  pub heimdall_redis_host: String,
  pub heimdall_secret: String
}

impl Config {
  pub fn new() -> Config {
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => {
            error!("Failed when reading environment config values: {}", error);
            std::process::exit(1);
        }
     }
  }
}
