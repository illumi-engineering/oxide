use std::fs;
use std::net::SocketAddrV4;
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
    label: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideExternalMavenRepositoryConfig {
    uri: String,
    label: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideExternalRepositoriesConfig {
    pub npm: Vec<OxideExternalNpmRepositoryConfig>,
    pub maven: Vec<OxideExternalMavenRepositoryConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideLocalRepositoriesConfig {
    pub npm: Option<bool>,
    pub maven: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OxideDaemonConfig {
    pub bind_addr: SocketAddrV4,
    pub home_servers: Vec<OxideHomeServerConfig>,
    pub external_repositories: OxideExternalRepositoriesConfig,
    pub local_repositories: OxideLocalRepositoriesConfig,
}

impl OxideDaemonConfig {
    pub fn load(location: PathBuf) -> Self {
        let config_content = fs::read_to_string(location.clone())
            .expect(&*format!("[oxided] err: unable to open config at {:?}", location.clone()));
        serde_json::from_str(&*config_content).expect("[oxided] err: invalid configuration file.")
    }
}