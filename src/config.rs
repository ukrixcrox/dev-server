use serde::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub projectfolder_path: String,
    pub ip_address:String,
    pub port: u16,
}

pub fn parse_config() -> ServerConfig{
    let current_dir_string: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();

    let toml_data = std::fs::read_to_string(current_dir_string + "/serverconfig.toml")
                                                .expect("Error: failed to read TOML");

    let value: ServerConfig = toml::from_str(&toml_data).unwrap();

    value
}
