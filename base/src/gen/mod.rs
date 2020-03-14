//! Functionality about procedurally generating content.
//!

extern crate fnv;

pub mod plant;
pub mod world;

pub use self::plant::PlantGenerator;
pub use self::world::WorldGenerator;

use self::fnv::FnvHasher;
use rand::{SeedableRng, XorShiftRng};
use std::hash::{Hash, Hasher};

pub type Random = XorShiftRng;

/// Creates a seeded RNG for use in world gen.
///
/// This function takes 3 seed parameters which are hashed and mixed together.
///
/// # Parameters
///
/// * `world_seed`: The constant world seed as set in the config
/// * `feat_seed`: Feature-specific constant seed
/// * `loc_seed`: Location-seed, for example, X/Y coordinates of a feature
pub fn seeded_rng<T: Hash, U: Hash>(world_seed: u64, feat_seed: T, loc_seed: U) -> Random {
    // Hash everything, even `world_seed`, since XorShift really doesn't like seeds
    // with many 0s in it
    let mut fnv = FnvHasher::default();
    world_seed.hash(&mut fnv);
    feat_seed.hash(&mut fnv);
    loc_seed.hash(&mut fnv);
    let rng_seed = fnv.finish();
    let seed0 = (rng_seed >> 32) as u32;
    let seed1 = rng_seed as u32;

    XorShiftRng::from_seed([seed0, seed1, seed0, seed1])
}
