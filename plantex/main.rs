extern crate client;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate server;

use std::io::{self, Write};
use log::LogLevelFilter;

fn main() {
    // Initialize logger (by default error, warning and info logs are shown)
    env_logger::LogBuilder::new()
        .filter(None, LogLevelFilter::Info)
        .parse(&std::env::var("RUST_LOG").unwrap_or("".into()))
        .init()
        .expect("logger initialization failed");


    info!("~~~~~~~~~~ Plantex started ~~~~~~~~~~");
    let res = client::start_game(client::Config::default());

    // Check if any error occured
    if res.is_err() {
        // Maybe the user disabled all logs, so we mention that the logs
        // contain information about the error.
        writeln!(io::stderr(),
                 "An error occured! Check logs for more information!")
            .expect("write to stderr failed");
        std::process::exit(1);
    }
}
