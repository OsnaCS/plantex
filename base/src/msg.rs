//! Messages for client-server-communication.

use math::{Point3f, Vector3f};

/// A message from the server to a client.
#[derive(RustcEncodable, RustcDecodable)]
pub enum ClientCommand {
    /// Register a player currently playing on the same server.
    ///
    /// Sent when a player joins the server or when this client just joined the
    /// server (it gets a list of all players currently on the server).
    RegisterPlayer {
        id: u32,
    },
}

/// A message from a client to the server.
#[derive(RustcEncodable, RustcDecodable)]
pub enum ServerCommand {
    UpdatePose {
        position: Point3f,
        orientation: Vector3f,
    },
}
