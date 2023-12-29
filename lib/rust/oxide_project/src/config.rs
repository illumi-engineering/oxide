use std::{fs, io};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// An oxide project configuration
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub subprojects: Option<Vec<String>>,
}

impl ProjectConfig {
    pub fn new(name: String) -> Self {
        ProjectConfig {
            name,
            subprojects: None,
        }
    }

    pub fn read_from(location: PathBuf) -> Self {
        let data = fs::read_to_string(location.clone())
            .expect(&*format!("[oxide] err: could not read config at {:?}", location.clone()));

        let config: ProjectConfig = toml::from_str(&*data)
            .expect("[oxide] err: invalid project configuration");

        return config;
    }

    pub fn write_to(self, location: PathBuf) -> io::Result<()> {
        let toml_val = toml::to_string_pretty(&self).unwrap();
        return fs::write(location, toml_val);
    }
}