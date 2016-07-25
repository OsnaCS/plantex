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
mod world;
mod world_manager;
mod event_manager;
pub mod util;

pub use camera::Camera;
pub use config::Config;
pub use event_manager::*;
pub use game_context::GameContext;
pub use renderer::Renderer;
pub use world_manager::WorldManager;

use game::Game;

pub fn start_game(config: Config) -> Result<(), ()> {
    let game = try!(Game::new(config));
    game.run()
}
