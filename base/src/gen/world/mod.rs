//! Procedurally generating the game world.
//!

use world::{Chunk, ChunkIndex, Provider};

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

impl Provider for WorldGenerator {
    /// Returns requested `Chunk`.
    fn load_chunk(&self, _: ChunkIndex) -> Option<Chunk> {
        // for the moment returns dummy chunk
        Some(Chunk::dummy())
    }

    /// Returns wether `Chunk` load is possible
    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // for the moment returns true
        true
    }
}
