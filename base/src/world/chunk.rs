use super::{CHUNK_SIZE, ChunkIndexComponent, HexPillar};
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
        let chunk_size: ChunkIndexComponent = CHUNK_SIZE.into();
        assert!(pos.q >= 0 && pos.q < chunk_size && pos.r >= 0 && pos.r < chunk_size,
                "axial position to index `Chunk` are out of bounds: {:?}",
                pos);

        &self.pillars[(pos.r as usize) * (CHUNK_SIZE as usize) + (pos.q as usize)]
    }
}
