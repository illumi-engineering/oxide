use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideHomeServerConfig {
    alias: String,
    rpc_uri: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideExternalNpmRepositoryConfig {
    uri: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideExternalMavenRepositoryConfig {
    uri: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideExternalRepositoriesConfig {
    npm: Vec<OxideExternalNpmRepositoryConfig>,
    maven: Vec<OxideExternalMavenRepositoryConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideLocalRepositoriesConfig {
    npm: Option<bool>,
    maven: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideDaemonConfig {
    home_servers: Vec<OxideHomeServerConfig>,
    external_repositories: OxideExternalRepositoriesConfig,
    local_repositories: OxideLocalRepositoriesConfig,
}

impl OxideDaemonConfig {
    pub fn load(location: PathBuf) -> Self {
        let config_content = fs::read_to_string(location.clone())
            .expect(&*format!("[oxided] err: unable to open config at {:?}", location.clone()));
        serde_json::from_str(&*config_content).expect("[oxided] err: invalid configuration file.")
    }
}