use std::fs;
use std::io;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub password: String
}

impl Config {
    pub fn load_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}