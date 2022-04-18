use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CmdConfig {
    pub cmd: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightConfig {
    pub inc: CmdConfig,
    pub dec: CmdConfig,
    pub set: CmdConfig,
    pub get: CmdConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundConfig {
    pub inc: CmdConfig,
    pub dec: CmdConfig,
    pub set: CmdConfig,
    pub mute: CmdConfig,
    pub get: CmdConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub start: CmdConfig,
    pub stop: CmdConfig,
    pub status: CmdConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub light: LightConfig,
    pub sound: SoundConfig,
    pub services: Vec<ServiceConfig>,
}

pub fn get_config() -> Config {
    let config_home = std::env::var("XDG_CONFIG_HOME").unwrap();
    let config_file = format!("{}/tostadora/tostadora.yml", config_home);

    let contents =
        fs::read_to_string(config_file).expect("Something went wrong reading the config file");

    serde_yaml::from_str(&contents).expect("config file should be deserialized correctly")
}
