use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BackendProvider {
    RabbitMQ,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub app_name: String,
    pub backend: BackendConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BackendConfig {
    pub provider: BackendProvider,
    pub connection_string: String,
}

impl Config {
    pub fn load_config(config_path: &String) -> Config {
        let file = File::open(config_path).unwrap();
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).unwrap();
        config
    }
}
