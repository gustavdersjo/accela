use serde::Deserialize;
use config::{Config, File, FileFormat};

const DEFAULT_CONFIG_TOML: &str = include_str!("resources/config.toml");
const CONFIG_PATH: &str = "./config/config.toml";

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub name: String,
    pub addr: String,
    pub port: u16,
    pub boot: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log
}

impl Settings {
    pub fn new() -> Settings {
        let config = Config::builder()
            .add_source(File::from_str(DEFAULT_CONFIG_TOML, FileFormat::Toml))
            .add_source(File::with_name(&CONFIG_PATH))
            .build().unwrap();
        let settings: Settings = config.try_deserialize().unwrap();
        return settings
    }
}
