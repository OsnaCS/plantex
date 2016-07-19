extern crate base;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;

pub mod render;
pub mod event_manager;
mod config;
mod game;

pub use config::Config;


pub fn start_game(config: Config, world_provider: &base::world::Provider) -> Result<(), ()> {
    game::run(&config, world_provider)
}
