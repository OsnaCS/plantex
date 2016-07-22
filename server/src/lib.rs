#[macro_use]
extern crate log;

mod server;

use server::Server;
use std::net::{TcpListener, SocketAddr};
use std::thread;
use std::io;

/// Starts a Plantex server listening for connections on the given `TcpListener`.
pub fn start_server(listener: TcpListener) -> io::Result<()> {
    info!("starting server on {}", try!(listener.local_addr()));

    let server = Server::new(listener);
    server.run()
}

/// Starts a server in a different thread, returns the address to connect to.
pub fn start_local_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    thread::Builder::new().name("Plantex Local Server".to_string()).spawn(move || {
        start_server(listener).unwrap();
    }).unwrap();

    addr
}
