use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct OxideHomeServerConfig {
}

#[derive(Serialize, Deserialize)]
struct OxideExternalNpmRepositoryConfig {
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct OxideExternalMavenRepositoryConfig {
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct OxideExternalRepositoriesConfig {
    npm: Vec<OxideExternalNpmRepositoryConfig>,
    maven: Vec<OxideExternalMavenRepositoryConfig>,
}

#[derive(Serialize, Deserialize)]
struct OxideDaemonConfig {
    home_servers: Vec<OxideHomeServerConfig>,
    external_repositories: OxideExternalRepositoriesConfig,
}

impl OxideDaemonConfig {

}