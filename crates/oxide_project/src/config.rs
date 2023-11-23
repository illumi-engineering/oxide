use std::{fs, io};
use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
// pub struct ProjectNpmConfig {
//     pub package_file: String,
// }

/// An oxide project configuration
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProjectConfig {
    name: String,
    subprojects: Option<Vec<String>>,
    targets: HashMap<String, Box<dyn >>
}

impl ProjectConfig {
    pub fn new(location: PathBuf) -> Self {
        let data = fs::read_to_string(location.clone())
            .expect(&*format!("[oxide] err: could not read config at {:?}", location.clone()));

        let config: ProjectConfig = toml::from_str(&*data)
            .expect("[oxide] err: invalid project configuration");

        return config;
    }

    pub fn write(self, location: PathBuf) -> io::Result<()> {
        let toml_val = toml::to_string_pretty(&self).unwrap();
        return fs::write(location, toml_val);
    }
}