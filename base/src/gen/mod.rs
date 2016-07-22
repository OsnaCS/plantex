//! Functionality about procedurally generating content.
//!

extern crate fnv;

mod plant;
mod world;

pub use self::world::WorldGenerator;
pub use self::plant::PlantGenerator;

use rand::{SeedableRng, XorShiftRng};
use self::fnv::FnvHasher;
use std::hash::{Hash, Hasher};

type Random = XorShiftRng;

/// Creates a seeded RNG for use in world gen.
///
/// This function takes 3 seed parameters which are hashed and mixed together.
///
/// # Parameters
///
/// * `world_seed`: The constant world seed as set in the config
/// * `feat_seed`: Feature-specific constant seed
/// * `loc_seed`: Location-seed, for example, X/Y coordinates of a feature
fn seeded_rng<T: Hash, U: Hash>(world_seed: u64, feat_seed: T, loc_seed: U) -> Random {
    // Hash everything, even `world_seed`, since XorShift really doesn't like seeds
    // with many 0s in it
    let mut fnv = FnvHasher::default();
    world_seed.hash(&mut fnv);
    feat_seed.hash(&mut fnv);
    loc_seed.hash(&mut fnv);
    let rng_seed = fnv.finish();
    let seed0 = (rng_seed >> 32) as u32;
    let seed1 = rng_seed as u32;
    // Apply our patented cryptoperturbation mask to magically extend the 64-bit
    // hash to 128 bits used by xorshift:
    // (To be honest, I just didn't want to pass the same 2 word twice)
    let seed2 = seed0 ^ 0xdeadbeef;
    let seed3 = seed1 ^ 0xcafebabe;

    XorShiftRng::from_seed([seed0, seed1, seed2, seed3])
}
