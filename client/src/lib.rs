//! This crate contains functionality for the client only. This is mainly
//! graphics and input handling.
//!

extern crate base;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;

mod camera;
mod config;
mod game;
mod game_context;
mod ghost;
mod renderer;
mod weather;
mod world;
mod world_manager;
mod event_manager;
pub mod view;
pub mod util;
pub mod daytime;
mod player;

pub use daytime::*;
pub use camera::Camera;
pub use config::Config;
pub use event_manager::*;
pub use game_context::GameContext;
pub use renderer::Renderer;
pub use world_manager::WorldManager;

use game::Game;
use std::net::SocketAddr;
use std::error::Error;

pub fn start_game(config: Config, server: SocketAddr) -> Result<(), Box<Error>> {
    let game = try!(Game::new(config, server));
    game.run()
}
