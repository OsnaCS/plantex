//! Procedurally generating the game world.
//!

use world::{CHUNK_SIZE, Chunk, ChunkIndex, ChunkProvider, HeightType, HexPillar};

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
    fn load_chunk(&self, index: ChunkIndex) -> Option<Chunk> {
        let mut pillars = Vec::new();
        let q = index.0.q * CHUNK_SIZE as i32;
        let r = index.0.r * CHUNK_SIZE as i32;
        let mut height;
        for i in q..q + CHUNK_SIZE as i32 {
            for j in r..r + CHUNK_SIZE as i32 {
                height = ((i as f32).sin() * 25.0 + (j as f32).sin() * 25.0 + 100.0) as u16;
                pillars.push(HexPillar::from_height(HeightType(height)));
            }
        }

        Some(Chunk::from_pillars(pillars))
    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // for the moment returns true
        true
    }
}
