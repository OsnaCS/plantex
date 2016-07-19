use std::collections::HashMap;
use super::{Chunk, HexPillar};
use math::*;

/// Represents a whole game world consisting of multiple `Chunk`s.
///
/// Chunks are parallelograms (roughly) that are placed next to each other
/// in the world.
pub struct World {
    chunks: HashMap<AxialPoint, Chunk>,
}

impl World {
    /// Creates an empty world without any chunks.
    pub fn empty() -> Self {
        World { chunks: HashMap::new() }
    }

    /// Returns the hex pillar at the given world position, iff the
    /// corresponding chunk is loaded.
    pub fn pillar_at(&self, pos: AxialPoint) -> Option<&HexPillar> {
        // TODO: use `/` operator once it's implemented
        // let chunk_pos = pos / (super::CHUNK_SIZE as i32);
        let chunk_pos = AxialPoint::new(pos.q / (super::CHUNK_SIZE as i32),
                                        pos.r / (super::CHUNK_SIZE as i32));

        let out = self.chunks.get(&chunk_pos).map(|chunk| {
            // TODO: use `%` operator once it's implemented
            // let inner_pos = pos % (super::CHUNK_SIZE as i32);
            let inner_pos = AxialPoint::new(pos.q % (super::CHUNK_SIZE as i32),
                                            pos.r % (super::CHUNK_SIZE as i32));
            &chunk[inner_pos]
        });

        if out.is_none() {
            debug!("chunk {:?} is not loaded (position request {:?})",
                   chunk_pos,
                   pos);
        }

        out
    }
}
