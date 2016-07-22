//! This crate contains functionality for the client only. This is mainly
//! graphics and input handling.
//!

extern crate base;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;

pub mod render;
pub mod event_manager;
mod camera;
mod config;
mod game;
mod ghost;
mod world;

pub use config::Config;
pub use camera::Camera;

use game::Game;

pub fn start_game(config: Config) -> Result<(), ()> {
    let game = try!(Game::new(config));
    game.run()
}
