//! Procedurally generating the game world.
//!

use world::{CHUNK_SIZE, Chunk, ChunkIndex, ChunkProvider, HeightType, HexPillar};
use world::{GroundMaterial, PillarSection, Prop, PropType};
use rand::{Rand, SeedableRng, XorShiftRng};
use gen::PlantGenerator;

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

        for i in q..q + CHUNK_SIZE as i32 {
            for j in r..r + CHUNK_SIZE as i32 {
                let height = (((i as f32) * 0.25).sin() * 10.0 + ((j as f32) * 0.25).sin() * 10.0 +
                              100.0) as u16;

                let ground_section =
                    PillarSection::new(GroundMaterial::Dirt, HeightType(0), HeightType(height));
                let mut props = Vec::new();

                // Place random plants in some spots
                if i % 8 == 0 && j % 8 == 0 {
                    let mut rng = XorShiftRng::from_seed([(self.seed >> 32) as u32,
                                                          self.seed as u32,
                                                          i as u32,
                                                          j as u32]);
                    let gen = PlantGenerator::rand(&mut rng);

                    props.push(Prop {
                        baseline: HeightType(height),
                        prop: PropType::Plant(gen.generate(&mut rng)),
                    });
                }

                pillars.push(HexPillar::new(vec![ground_section], props));
            }
        }

        Some(Chunk::from_pillars(pillars))
    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // for the moment returns true
        true
    }
}
