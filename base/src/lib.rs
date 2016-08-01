//! Functionality used by both the plantex server and the plantex client.
//!
//! - the `math` module reexports everything from the `cgmath` crate and
//!   defines a few own type
//! - the world module is all about saving and managing the game world

pub extern crate rand;
extern crate num_traits;
pub extern crate noise;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub mod gen;
pub mod math;
pub mod prop;
pub mod world;
pub mod msg;
