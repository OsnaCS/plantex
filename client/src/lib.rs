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
mod control_switcher;
pub mod daytime;
mod event_manager;
mod frustum;
mod game;
mod game_context;
mod ghost;
mod player;
mod renderer;
pub mod util;
pub mod view;
mod weather;
mod world;
mod world_manager;

pub use camera::Camera;
pub use config::Config;
pub use daytime::*;
pub use event_manager::*;
pub use frustum::Frustum;
pub use frustum::SimpleCull;
pub use frustum::LOCATION;
pub use game_context::GameContext;
pub use renderer::Renderer;
pub use world_manager::WorldManager;

use game::Game;
use std::error::Error;
use std::net::SocketAddr;

pub fn start_game(config: Config, server: SocketAddr) -> Result<(), Box<dyn Error>> {
    let game = Game::new(config, server)?;
    game.run()
}
