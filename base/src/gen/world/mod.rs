//! Procedurally generating the game world.
//!

use world::{Chunk, ChunkIndex, ChunkProvider};

/// Main type to generate the game world. Implements the `ChunkProvider` trait
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

impl ChunkProvider for WorldGenerator {
    fn load_chunk(&self, _: ChunkIndex) -> Option<Chunk> {
        // for the moment returns dummy chunk
        Some(Chunk::dummy())
    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // for the moment returns true
        true
    }
}
