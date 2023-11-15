use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OxideHomeServerConfig {
    alias: String,
    rpc_uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct OxideExternalNpmRepositoryConfig {
    uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct OxideExternalMavenRepositoryConfig {
    uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct OxideExternalRepositoriesConfig {
    npm: Vec<OxideExternalNpmRepositoryConfig>,
    maven: Vec<OxideExternalMavenRepositoryConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct OxideDaemonConfig {
    home_servers: Vec<OxideHomeServerConfig>,
    external_repositories: OxideExternalRepositoriesConfig,
}

impl OxideDaemonConfig {
    pub fn load(location: PathBuf) -> Self {
        let config_content = fs::read_to_string(location.clone())
            .expect(&*format!("[oxided] err: unable to open config at {:?}", location.clone()));
        return serde_json::from_str(&*config_content).expect("[oxided] err: invalid configuration file.");
    }
}