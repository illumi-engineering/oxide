use std::{fs, io};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProjectNpmConfig {
    pub package_file: String,
}

/// An oxide project configuration
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub subprojects: Option<Vec<String>>,
    pub npm: Option<ProjectNpmConfig>,
}

impl ProjectConfig {
    pub fn new(location: PathBuf) -> Self {
        let data = fs::read_to_string(location.clone())
            .expect(&*format!("[oxide_common] err: could not read config at {:?}", location.clone()));

        let config: ProjectConfig = toml::from_str(&*data)
            .expect("[oxide_common] err: invalid project configuration");

        return config;
    }

    pub fn write(self, location: PathBuf) -> io::Result<()> {
        let toml_val = toml::to_string_pretty(&self).unwrap();
        return fs::write(location, toml_val);
    }
}