use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectNpmConfig {
    pub package_file: String,
}

/// An oxide project configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub subprojects: Option<Vec<String>>,
    pub npm: Option<ProjectNpmConfig>,
}

impl ProjectConfig {
    pub fn new(location: PathBuf) -> Self {
        let data = fs::read_to_string(location).expect("Err: Could not read oxide.toml!");
        let config: ProjectConfig = toml::from_str(&*data).unwrap();
        return config;
    }
}