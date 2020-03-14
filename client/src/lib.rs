//! This crate contains functionality for the client only. This is mainly
//! graphics and input handling.
//!

#![allow(illegal_floating_point_literal_pattern)]

extern crate base;
extern crate rand;
#[macro_use]
extern crate glium;
extern crate noise;
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
mod frustum;
pub mod view;
pub mod util;
pub mod daytime;
mod player;
mod control_switcher;

pub use daytime::*;
pub use camera::Camera;
pub use config::Config;
pub use event_manager::*;
pub use game_context::GameContext;
pub use renderer::Renderer;
pub use world_manager::WorldManager;
pub use frustum::Frustum;
pub use frustum::LOCATION;
pub use frustum::SimpleCull;

use game::Game;
use std::net::SocketAddr;
use std::error::Error;

pub fn start_game(config: Config, server: SocketAddr) -> Result<(), Box<dyn Error>> {
    let game = Game::new(config, server)?;
    game.run()
}
