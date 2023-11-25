use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use oxide_project::OxideProject;

#[derive(Serialize, Deserialize)]
pub struct ProjectNpmConfig {
    pub enable: bool,
}

fn get_path_relative_to(project: &OxideProject) -> PathBuf {
    project.directory.clone().join(".oxide").join("npm.toml")
}

impl ProjectNpmConfig {
    fn load(project: &OxideProject) -> Self {
        let path = get_path_relative_to(project);
        let content = fs::read_to_string(path).expect("Failed to load project npm config");
        toml::from_str(content.as_str()).unwrap()
    }
}