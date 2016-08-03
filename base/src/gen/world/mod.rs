//! Procedurally generating the game world.
pub mod biome;

use world::{Chunk, ChunkIndex, ChunkProvider, HeightType, HexPillar};
use world::{CHUNK_SIZE, GroundMaterial, PILLAR_STEP_HEIGHT, PillarSection, Prop, PropType};
use rand::{Rand, Rng};
use gen::{PlantGenerator, seeded_rng};
use noise::{PermutationTable, open_simplex2, open_simplex3};
use gen::world::biome::Biome;
use prop::plant::Plant;
use gen::plant::tree::PlantType;

/// Land "fill noise" scaling in x, y, and z direction.
const LAND_NOISE_SCALE: (f32, f32, f32) = (0.03, 0.03, 0.05);

/// Main type to generate the game world. Implements the `ChunkProvider` trait
/// (TODO, see #8).
pub struct WorldGenerator {
    seed: u64,
    terrain_table: PermutationTable,
    plant_table: PermutationTable,
    temperature_table: PermutationTable,
    humidity_table: PermutationTable,
}

impl WorldGenerator {
    /// Creates the generator with the given seed.
    pub fn with_seed(seed: u64) -> Self {
        let mut terrain_rng = seeded_rng(seed, 0, ());
        let mut plant_rng = seeded_rng(seed, 1, ());
        let mut temperature_rng = seeded_rng(seed, 2, ());
        let mut humidity_rng = seeded_rng(seed, 3, ());

        WorldGenerator {
            seed: seed,
            terrain_table: PermutationTable::rand(&mut terrain_rng),
            plant_table: PermutationTable::rand(&mut plant_rng),
            temperature_table: PermutationTable::rand(&mut temperature_rng),
            humidity_table: PermutationTable::rand(&mut humidity_rng),
        }
    }

    /// Returns the seed of this world generator.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// trying aproximate the
    /// steepvalues       10   20   60  200
    /// for temperature  0.0  0.2  0.4  1.0
    fn steepness_from_temperature(temperature: f32) -> f32 {
        120.0 * temperature * temperature + 72.0 * temperature + 3.5
    }
}

impl ChunkProvider for WorldGenerator {
    fn load_chunk(&self, index: ChunkIndex) -> Option<Chunk> {
        const WORLDGEN_HEIGHT: usize = 256;
        // Create a 3D-Array of booleans indicating which pillar sections to fill
        // (Map height is unlimited in theory, but we'll limit worldgen to 64 height
        // units)
        let mut fill = [[[false; WORLDGEN_HEIGHT]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

        Some(Chunk::with_pillars(index, |pos| {

            let real_pos = pos.to_real();
            let x = real_pos.x;
            let y = real_pos.y;
            // Pillar pos relative to first pillar
            let rel_pos = pos - index.0 * CHUNK_SIZE as i32;

            // noises
            let mut temperature_noise = (open_simplex2::<f32>(&self.temperature_table,
                                                              &[(x as f32) * 0.0015,
                                                                (y as f32) * 0.0015]) +
                                         0.6) / 2.0;
            temperature_noise += 0.035 *
                                 open_simplex2::<f32>(&self.temperature_table,
                                                      &[(x as f32) * 0.15, (y as f32) * 0.15]);


            let mut humidity_noise = (open_simplex2::<f32>(&self.humidity_table,
                                                           &[(x as f32) * 0.0015,
                                                             (y as f32) * 0.0015]) +
                                      0.6) / 2.0;
            humidity_noise += 0.035 *
                              open_simplex2::<f32>(&self.humidity_table,
                                                   &[(x as f32) * 0.15, (y as f32) * 0.15]);


            let current_biome = Biome::from_climate(temperature_noise, humidity_noise);

            for i in 0..WORLDGEN_HEIGHT {
                if i == 0 {
                    fill[rel_pos.q as usize][rel_pos.r as usize][i as usize] = true;
                    continue;
                }

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
                /// Threshold at half value (max. steepness, avg. terrain
                /// height)
                const THRESH_MID: f32 = 0.5;

                // "Steepness" of the sigmoid function.
                let thresh_steepness: f32 =
                    WorldGenerator::steepness_from_temperature(temperature_noise);

                let sig_thresh = 1.0 /
                                 (1.0 + f32::exp(-thresh_steepness * (height_pct - THRESH_MID)));

                let threshold = (sig_thresh + MIN_THRESH) / (1.0 + MIN_THRESH);

                fill[rel_pos.q as usize][rel_pos.r as usize][i as usize] = fill_noise > threshold;
            }

            let column = &fill[rel_pos.q as usize][rel_pos.r as usize];

            // Create sections for all connected `true`s in the array
            let mut sections = Vec::new();
            let mut low = 0;
            let mut height = None;
            for i in 0..WORLDGEN_HEIGHT {
                let material = current_biome.material();

                match (height, column[i]) {

                    (Some(h), true) => {
                        // Next one's still solid, increase section height
                        height = Some(h + 1);
                    }
                    (Some(h), false) => {
                        // Create a section of height `h` and start over
                        sections.push(PillarSection::new(material,
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

            // plants
            let plant_noise = open_simplex2::<f32>(&self.plant_table,
                                                   &[(x as f32) * 0.25, (y as f32) * 0.25]);

            if plant_noise > current_biome.plant_threshold() {
                let mut rng = super::seeded_rng(self.seed, "TREE", (pos.q, pos.r));

                let tmp = current_biome.plant_distribution();
                let plant_type = rng.choose(tmp).unwrap();

                let type_index = match *plant_type {
                    PlantType::RegularTree => 0,
                    PlantType::Shrub => 1,
                    PlantType::Cactus => 2,
                    PlantType::JungleTree => 3,
                    PlantType::ClumpOfGrass => 4,
                    PlantType::Conifer => 5,
                    PlantType::OakTree => 6,
                    PlantType::Flower => 7,
                };

                let plant_instance = rng.gen_range(0, 5);
                let plant_index = 8 * plant_instance + type_index;


                //     // put the tree at the highest position
                let height = match sections.last() {
                    Some(section) => section.top,
                    None => HeightType::from_units(0),
                };

                props.push(Prop {
                    baseline: height,
                    // for now, you can here set which plants should be placed
                    // all over the world
                    prop: PropType::Plant(PlantGenerator::new(PlantType::RegularTree)
                        .generate(&mut rng)),
                    plant_index: plant_index as usize,
                });

            }

            HexPillar::new(sections,
                           props,
                           Biome::from_climate(temperature_noise, humidity_noise))
        }))
    }

    fn get_plant_list(&self) -> Vec<Plant> {
        let mut rng = super::seeded_rng(self.seed, "TREE", 42);

        let mut vec = Vec::new();
        for _ in 0..5 {
            vec.push(PlantGenerator::new(PlantType::RegularTree).generate(&mut rng));
            vec.push(PlantGenerator::new(PlantType::Shrub).generate(&mut rng));
            vec.push(PlantGenerator::new(PlantType::Cactus).generate(&mut rng));
            vec.push(PlantGenerator::new(PlantType::JungleTree).generate(&mut rng));
            vec.push(PlantGenerator::new(PlantType::ClumpOfGrass).generate(&mut rng));
            vec.push(PlantGenerator::new(PlantType::Conifer).generate(&mut rng));
            vec.push(PlantGenerator::new(PlantType::OakTree).generate(&mut rng));
            vec.push(PlantGenerator::new(PlantType::Flower).generate(&mut rng));
        }
        vec
    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // All chunks can be generated from nothing
        true
    }
}
