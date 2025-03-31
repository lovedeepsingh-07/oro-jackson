// modules
pub mod cli;
pub mod content;
pub mod error;
pub mod handlers;
pub mod server;
pub mod templates;
pub mod utils;

// constants
pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
pub const PORT: u16 = 8080;
