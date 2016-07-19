use super::{CHUNK_SIZE, ChunkIndex, HexPillar};
use std::ops;
use math::*;


pub struct Chunk {
    pillars: Vec<HexPillar>,
}

impl Chunk {
    // neighbors
}

impl ops::Index<AxialPoint> for Chunk {
    type Output = HexPillar;

    fn index(&self, pos: AxialPoint) -> &Self::Output {
        assert!(pos.q >= 0 && pos.q < (CHUNK_SIZE as ChunkIndex) && pos.r >= 0 &&
                pos.r < (CHUNK_SIZE as ChunkIndex),
                "axial position to index `Chunk` are out of bounds: {:?}",
                pos);

        &self.pillars[(pos.r as usize) * (CHUNK_SIZE as usize) + (pos.q as usize)]
    }
}
