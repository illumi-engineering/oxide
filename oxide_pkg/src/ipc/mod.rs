use std::net::{Ipv4Addr, SocketAddrV4};

mod utils;
pub mod protocol;
pub mod response;
pub mod request;

pub const OXIDE_LOCAL_COMMUNICATION_ADDRESS: SocketAddrV4 =
    SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 42068);
pub const OXIDE_REPO_COMMUNICATION_ADDRESS: SocketAddrV4 =
    SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 42069);
