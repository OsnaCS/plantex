//! Procedurally generating the game world.
//!

/// Main type to generate the game world. Implements the `WorldProvider` trait
/// (TODO, see #8).
pub struct WorldGenerator {
    seed: u64,
}

impl WorldGenerator {
    /// Creates the generator with the given seed.
    pub fn with_seed(seed: u64) -> Self {
        WorldGenerator { seed: seed }
    }

    /// Returns the seed of this world generator.
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
