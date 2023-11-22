use serde::{Deserialize, Serialize};
use twelf::config;

#[config]
#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    /// Connection port of the local node daemon
    daemon_port: i32,
}