use std::net::{TcpListener, TcpStream};
use std::io;
use std::thread;
use std::sync::mpsc::{Receiver, TryRecvError, channel};
use std::time::Duration;

/// A player connected to the server.
struct Player {
    conn: TcpStream,
}

pub struct Server {
    /// Receives newly connected player connections from the TCP listener
    /// thread.
    connections: Receiver<TcpStream>,
    /// Currently connected players.
    players: Vec<Player>,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        let (sender, recv) = channel();

        thread::Builder::new()
            .name("TCP listener".to_string())
            .spawn(move || {
                for conn in listener.incoming() {
                    match conn {
                        Ok(conn) => {
                            match sender.send(conn) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("tcp listener exiting: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("failed to accept client connection: {}", e);
                        }
                    }
                }
            })
            .unwrap();

        Server {
            connections: recv,
            players: Vec::new(),
        }
    }

    /// Runs the server's main loop.
    pub fn run(mut self) -> io::Result<()> {
        loop {
            loop {
                match self.connections.try_recv() {
                    Ok(stream) => {
                        let id = self.players.len();
                        self.players.push(Player { conn: stream });
                        self.handle_new_player(id);
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        info!("tcp listener thread exited, killing server");
                        return Ok(());
                    }
                }
            }

            // Sleep for 1/60th of a second
            // FIXME Put in a proper rate limit
            thread::sleep(Duration::new(0, 1000000 / 60));

            // self.world_manager.update_world(self.player.get_camera().position);
        }
    }

    /// Calls a closure with a mutable reference to the given player.
    ///
    /// If the closure returns `Err`, the player will be disconnected.
    fn with_player<F>(&mut self, id: usize, f: F)
        where F: FnOnce(&mut Player) -> io::Result<()>
    {
        match f(&mut self.players[id]) {
            Ok(()) => return,
            Err(e) => {
                error!("lost connection to player #{} - {}", id, e);
            }
        }

        self.players.remove(id);
    }

    fn handle_new_player(&mut self, id: usize) {
        self.with_player(id, |player| {
            info!("client connected from {}", player.conn.peer_addr()?);
            Ok(())
        })
    }
}
