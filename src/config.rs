use std::{
    env,
    fs,
};

use log::error;
use secrecy::{
    ExposeSecret,
    Secret,
};
use serde::Deserialize;
use toml::de::from_str;

const CONFIG_PATH_ENV: &str = "CONFIG_PATH";

#[derive(Debug, Deserialize)]
pub struct SecretString(Secret<String>);

impl SecretString {
    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl Default for SecretString {
    fn default() -> Self {
        SecretString(Secret::new("".to_string()))
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub authorized_user_ids: Vec<u64>,
    pub telegram_bot_token: SecretString,
    pub check_interval_secs: u64,
}

pub fn read_config() -> Config {
    env::var(CONFIG_PATH_ENV)
        .map_err(|_| format!("{CONFIG_PATH_ENV} environment variable not set"))
        .and_then(|config_path| fs::read_to_string(config_path).map_err(|e| e.to_string()))
        .and_then(|content| from_str(&content).map_err(|e| e.to_string()))
        .unwrap_or_else(|err| {
            error!("failed to read config: {err}");
            std::process::exit(1);
        })
}
