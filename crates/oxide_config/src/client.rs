use twelf::config;

#[config]
pub struct ClientConfig {
    /// Connection port of the local node daemon
    daemon_port: i32,
}