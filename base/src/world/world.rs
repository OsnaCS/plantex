

use std::collections::HashMap;
use std::ops;
use super::{Chunk, HexPillar};
use math::*;

pub struct World {
    chunks: HashMap<AxialPoint, Chunk>,
}

impl World {
    pub fn empty() -> Self {
        World { chunks: HashMap::new() }
    }
}

impl ops::Index<AxialPoint> for World {
    type Output = HexPillar;

    fn index(&self, pos: AxialPoint) -> &Self::Output {
        // TODO: use `/` operator once it's implemented
        // let chunk_pos = pos / (super::CHUNK_SIZE as i32);
        let chunk_pos = AxialPoint::new(pos.q / (super::CHUNK_SIZE as i32),
                                        pos.r / (super::CHUNK_SIZE as i32));

        match self.chunks.get(&chunk_pos) {
            None => {
                panic!("chunk {:?} is not loaded (position request {:?})",
                       chunk_pos,
                       pos)
            }
            Some(chunk) => {
                // TODO: use `%` operator once it's implemented
                // let inner_pos = pos % (super::CHUNK_SIZE as i32);
                let inner_pos = AxialPoint::new(pos.q % (super::CHUNK_SIZE as i32),
                                                pos.r % (super::CHUNK_SIZE as i32));
                &chunk[inner_pos]
            }
        }
    }
}
