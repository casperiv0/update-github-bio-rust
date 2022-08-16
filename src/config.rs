use serde_derive::Deserialize;
use std::fs::File;
use std::io::Error;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github_api_token: String,
    pub wakatime_api_token: String,
}

pub fn load_config() -> Result<Config, Error> {
    let file_path = Path::new("./config.json");
    let file = File::open(file_path).expect("config file not found");
    let config = serde_json::from_reader(file).expect("unable to read config.json");

    Ok(config)
}
