//! Procedurally generating the game world.

use math::*;
use world::{Chunk, ChunkIndex, ChunkProvider, HeightType, HexPillar};
use world::{CHUNK_SIZE, GroundMaterial, PILLAR_STEP_HEIGHT, PillarSection, Prop, PropType};
use rand::{Rand, Rng};
use gen::PlantGenerator;
use noise::{PermutationTable, open_simplex2, open_simplex3};
use gen::seeded_rng;

// pub const LANDSCAPE_FLAT: f32 = 0.01;
// pub const LANDSCAPE_MODERATE: f32 = 0.05;
// pub const LANDSCAPE_HILLY: f32 = 0.1125;


/// Land "fill noise" scaling in x, y, and z direction.
const LAND_NOISE_SCALE: (f32, f32, f32) = (0.03, 0.03, 0.05);

/// Main type to generate the game world. Implements the `ChunkProvider` trait
/// (TODO, see #8).
pub struct WorldGenerator {
    seed: u64,
    terrain_table: PermutationTable,
    plant_table: PermutationTable,
}

impl WorldGenerator {
    /// Creates the generator with the given seed.
    pub fn with_seed(seed: u64) -> Self {
        let mut terrain_rng = seeded_rng(seed, 0, ());
        let mut plant_rng = seeded_rng(seed, 1, ());

        WorldGenerator {
            seed: seed,
            terrain_table: PermutationTable::rand(&mut terrain_rng),
            plant_table: PermutationTable::rand(&mut plant_rng),
        }
    }

    /// Returns the seed of this world generator.
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl ChunkProvider for WorldGenerator {
    fn load_chunk(&self, index: ChunkIndex) -> Option<Chunk> {
        const WORLDGEN_HEIGHT: usize = 256;
        // Create a 3D-Array of booleans indicating which pillar sections to fill
        // (Map height is unlimited in theory, but we'll limit worldgen to 64 height
        // units)
        let mut fill = [[[false; WORLDGEN_HEIGHT]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

        // Chunk start position in absolute axial coords
        let chunk_start = index.0 * CHUNK_SIZE as i32;
        for q in 0..CHUNK_SIZE {
            for r in 0..CHUNK_SIZE {
                for i in 0..WORLDGEN_HEIGHT {
                    if i == 0 {
                        fill[q as usize][r as usize][i as usize] = true;
                        continue;
                    }

                    let real_pos = (chunk_start + AxialVector::new(q as i32, r as i32)).to_real();
                    let x = real_pos.x;
                    let y = real_pos.y;
                    let z = i as f32 * PILLAR_STEP_HEIGHT;
                    let fill_noise = open_simplex3::<f32>(&self.terrain_table,
                                                          &[x * LAND_NOISE_SCALE.0,
                                                            y * LAND_NOISE_SCALE.1,
                                                            z * LAND_NOISE_SCALE.2]);

                    // The noise is (theoretically) in the range -1..1
                    // Map the noise to a range of 0..1
                    let fill_noise = (fill_noise + 1.0) / 2.0;

                    // Calculate threshold to fill this "block". The lower the threshold, the more
                    // likely this voxel is filled, so it should increase with height.
                    let height_pct = i as f32 / WORLDGEN_HEIGHT as f32;

                    // The threshold is calculated using a sigmoid function. These are the
                    // parameters used:

                    /// Minimum threshold to prevent threshold to reach 0,
                    /// needed to have any caves at all
                    const MIN_THRESH: f32 = 0.6;
                    /// "Steepness" of the sigmoid function.
                    const THRESH_STEEPNESS: f32 = 30.0;
                    /// Threshold at half value (max. steepness, avg. terrain
                    /// height)
                    const THRESH_MID: f32 = 0.5;

                    let sig_thresh =
                        1.0 / (1.0 + f32::exp(-THRESH_STEEPNESS * (height_pct - THRESH_MID)));

                    let threshold = (sig_thresh + MIN_THRESH) / (1.0 + MIN_THRESH);

                    fill[q as usize][r as usize][i as usize] = fill_noise > threshold;
                }
            }
        }

        Some(Chunk::with_pillars(index, |pos| {
            let real_pos = pos.to_real();
            let x = real_pos.x;
            let y = real_pos.y;

            // Pillar pos relative to first pillar
            let rel_pos = pos - index.0 * CHUNK_SIZE as i32;
            let column = &fill[rel_pos.q as usize][rel_pos.r as usize];

            // Create sections for all connected `true`s in the array
            let mut sections = Vec::new();
            let mut low = 0;
            let mut height = None;
            for i in 0..WORLDGEN_HEIGHT {
                match (height, column[i]) {
                    (Some(h), true) => {
                        // Next one's still solid, increase section height
                        height = Some(h + 1);
                    }
                    (Some(h), false) => {
                        // Create a section of height `h` and start over
                        sections.push(PillarSection::new(GroundMaterial::Grass,
                                                         HeightType::from_units(low),
                                                         HeightType::from_units(low + h)));
                        height = None;
                    }
                    (None, true) => {
                        low = i as u16;
                        height = Some(1);
                    }
                    (None, false) => {}
                };
            }

            if let Some(h) = height {
                // Create the topmost pillar
                sections.push(PillarSection::new(GroundMaterial::Dirt,
                                                 HeightType::from_units(low),
                                                 HeightType::from_units(low + h)));
            }

            let mut props = Vec::new();

            let plant_noise = open_simplex2::<f32>(&self.plant_table,
                                                   &[(x as f32) * 0.08, (y as f32) * 0.08]);

            // Place a test plant every few blocks
            if plant_noise > 0.4 {
                let mut rng = super::seeded_rng(self.seed, "TREE", (pos.q, pos.r));
                if rng.next_f32() < plant_noise {
                    let gen = PlantGenerator::rand(&mut rng);

                    // put the tree at the highest position
                    let height = match sections.last() {
                        Some(section) => section.top,
                        None => HeightType::from_units(0),
                    };

                    props.push(Prop {
                        baseline: height,
                        prop: PropType::Plant(gen.generate(&mut rng)),
                    });
                }
            }

            HexPillar::new(sections, props)
        }))
    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // All chunks can be generated from nothing
        true
    }
}
