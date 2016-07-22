//! Procedurally generating the game world.
//!

use world::{CHUNK_SIZE, Chunk, ChunkIndex, ChunkProvider, HeightType, HexPillar};
use rand::*;
use math::*;

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

    // P HAS TO BE PILLARINDEX!!!!!!! (not Point)
    fn calc_height(p: Point2f, poss: [Point2f; 4], grads: [Vector2f; 4]) -> HeightType {
        let mut weight_vals = [0.0; 4];

        for i in 0..4 {
            weight_vals[i] = (poss[i] - p).dot(grads[i]);
        }

        // This wont compile ... why?
        // HeightType(0(avg_weight(p, poss, weight_vals) + 20) * 10)
        HeightType(0)
    }

    // smothing function, maybe later
    // fn ease_function(x: f32, x0: f32) {
    //     (3(x - x0)).pow(2) - 2(x - x0).pow(3);
    // }

    fn avg_weight(p: Point2f, poss: [Point2f; 4], weight_vals: [f32; 4]) -> f32 {
        let mut sum = 0.0;

        for i in 0..4 {
            sum += weight_vals[i] * (poss[i] - p).magnitude() / (2 as f32).sqrt();
        }
        sum
    }

    fn gen_gradient(&self, p: Point2i, rng: &mut Isaac64Rng) -> Vector2f {
        rng.reseed(&[self.seed, (p.x + i32::max_value()) as u64, (p.y + i32::max_value()) as u64]);

        Vector2::new(rng.next_f32(), rng.next_f32()).normalize()
    }
}

impl ChunkProvider for WorldGenerator {
    fn load_chunk(&self, index: ChunkIndex) -> Option<Chunk> {
        // that was for sin-hills
        let mut pillars = Vec::new();
        let q = index.0.q * CHUNK_SIZE as i32;
        let r = index.0.r * CHUNK_SIZE as i32;
        let mut height;
        for i in q..q + CHUNK_SIZE as i32 {
            for j in r..r + CHUNK_SIZE as i32 {
                // * 10.0
                height = (((i as f32) * 0.25).sin() * ((j as f32) * 0.25).sin() * 10.0 +
                          25.0) as u16;
                pillars.push(HexPillar::from_height(HeightType(height)));
            }
        }
        Some(Chunk::from_pillars(pillars))

        // TODO: generate chunk with perlin noise

    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // for the moment returns true
        true
    }
}
