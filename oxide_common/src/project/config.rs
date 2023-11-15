use std::fs;

use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Serialize, Deserialize)]
struct OxideNpmConfig {
    package_file: String,
}

/// An oxide project configuration
#[derive(Serialize, Deserialize)]
struct OxideConfig {
    npm: Option<OxideNpmConfig>
}

impl OxideConfig {
    fn new(location: String) -> Self {
        let data = fs::read_to_string(location).expect("Err: Could not read oxide.toml!");
        let config: OxideConfig = toml::from_str(&*data).unwrap();
        return config;
    }


}