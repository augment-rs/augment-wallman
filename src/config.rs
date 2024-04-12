use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::DisplayType;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub display_type: DisplayType,
}

/// Configuration
impl Config {
    const DEFAULT: Self = Config {
        display_type: DisplayType::Crop,
    };

    pub fn new() -> Config {
        // Path to config file.
        let config_path = xdg::BaseDirectories::new()
            .unwrap()
            .get_config_file("augment/wallman.toml");

        // Initialize config file.
        if !config_path.exists() {
            xdg::BaseDirectories::new()
                .unwrap()
                .create_config_directory("augment")
                .unwrap();

            let mut config_file = File::create(&config_path).unwrap();
            config_file
                .write_all(toml::to_string(&Self::DEFAULT).unwrap().as_bytes())
                .unwrap();
        }

        // Read config file.
        let mut config_file = File::open(&config_path).unwrap();
        let config: Config = toml::from_str(&{
            let mut buf = String::new();
            config_file.read_to_string(&mut buf).unwrap();
            buf
        })
        .unwrap();

        config
    }
}
