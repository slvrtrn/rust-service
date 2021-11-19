use std::fs::File;
use std::io::Read;

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub kafka_bootstrap_server: String,
    pub kafka_inbox_topic: String,
    pub kafka_inbox_group_id: String,
    pub kafka_inbox_num_workers: u8,
    pub kafka_client_id: String,
    pub http_port: u16,
    pub rust_env: String,
}

#[allow(dead_code)]
impl Config {
    pub fn is_production(&self) -> bool {
        self.rust_env == "production"
    }
    pub fn is_development(&self) -> bool {
        !self.is_production()
    }
}

fn load_config() -> std::io::Result<Config> {
    let env = envy::from_env::<Config>();
    match env {
        // if we could load the config using the existing env variables - use that
        Ok(config) => Ok(config),
        // otherwise, try to load the .env file
        Err(_) => {
            // simulate https://www.npmjs.com/package/dotenv behavior
            let mut file = File::open(".env")?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            for line in content.lines() {
                let pair = line.split('=').collect::<Vec<&str>>();
                let (key, value) = match &pair[..] {
                    &[key, value] => (key, value),
                    _ => panic!("Expected env variable pairs, got {}", content),
                };
                std::env::set_var(key, value);
            }
            match envy::from_env::<Config>() {
                Ok(config) => Ok(config),
                Err(e) => panic!("Failed to read the config from env: {}", e),
            }
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = load_config().unwrap();
}
