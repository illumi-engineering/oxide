use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericRepositoryConfig {
    pub url: String,
    pub label: String,
}