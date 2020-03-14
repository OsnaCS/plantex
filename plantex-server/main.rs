extern crate server;
#[macro_use]
extern crate log;
extern crate env_logger;

use log::LogLevelFilter;
use std::net::TcpListener;

fn main() {
    // Initialize logger (by default error, warning and info logs are shown)
    env_logger::LogBuilder::new()
        .filter(None, LogLevelFilter::Info)
        .parse(&std::env::var("RUST_LOG").unwrap_or("".into()))
        .init()
        .expect("logger initialization failed");

    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    server::start_server(listener).unwrap();
}
