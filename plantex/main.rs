extern crate base;
extern crate client;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate server;

use log::LogLevelFilter;
use std::io::{self, Write};

fn main() {
    // Initialize logger (by default error, warning and info logs are shown)
    env_logger::LogBuilder::new()
        .filter(None, LogLevelFilter::Info)
        .parse(&std::env::var("RUST_LOG").unwrap_or_else(|_| "".into()))
        .init()
        .expect("logger initialization failed");

    info!("Launching local server");
    let addr = server::start_local_server();

    info!("~~~~~~~~~~ Plantex started ~~~~~~~~~~");

    let conf = match client::Config::load_config() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let res = client::start_game(conf, addr);

    // Check if any error occured
    if res.is_err() {
        // Maybe the user disabled all logs, so we mention that the logs
        // contain information about the error.
        writeln!(
            io::stderr(),
            "An error occured! Check logs for more information!"
        )
        .expect("write to stderr failed");
        std::process::exit(1);
    }
}
