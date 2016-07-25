//! Procedurally generating the game world.
//!

// extern crate noise;

use world::{CHUNK_SIZE, Chunk, ChunkIndex, ChunkProvider, HeightType, HexPillar};
use world::{GroundMaterial, PillarSection, Prop, PropType};
use rand::Rand;
use gen::PlantGenerator;
use noise::{PermutationTable, open_simplex2};
use gen::seeded_rng;

// pub const LANDSCAPE_FLAT: f32 = 0.01;
pub const LANDSCAPE_MODERATE: f32 = 0.05;
// pub const LANDSCAPE_HILLY: f32 = 0.1125;


/// Main type to generate the game world. Implements the `ChunkProvider` trait
/// (TODO, see #8).
pub struct WorldGenerator {
    seed: u64,
    terrain_table: PermutationTable,
}

impl WorldGenerator {
    /// Creates the generator with the given seed.
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = seeded_rng(seed, 0, ());

        WorldGenerator {
            seed: seed,
            terrain_table: PermutationTable::rand(&mut rng),
        }
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

                let noise = open_simplex2::<f32>(&self.terrain_table,
                                                 &[(i as f32) * LANDSCAPE_MODERATE,
                                                   (j as f32) * LANDSCAPE_MODERATE]);
                let height = 50.0 + noise * 50.0;
                println!("{}", height);
                let ground_section = PillarSection::new(GroundMaterial::Dirt,
                                                        HeightType::from_units(0),
                                                        HeightType::from_units(height as u16));
                let mut props = Vec::new();

                // Place a test plant every few blocks
                const TREE_SPACING: i32 = 8;
                if i % TREE_SPACING == 0 && j % TREE_SPACING == 0 {
                    let mut rng = super::seeded_rng(self.seed, "TREE", (i, j));
                    let gen = PlantGenerator::rand(&mut rng);

                    props.push(Prop {
                        baseline: HeightType::from_units(height as u16),
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
