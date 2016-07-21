use std::collections::HashMap;
use super::{Chunk, ChunkIndex, HexPillar, PillarIndex};
use world::provider::ChunkProvider;
use math::*;

/// Represents a whole game world consisting of multiple `Chunk`s.
///
/// Chunks are parallelograms (roughly) that are placed next to each other
/// in the world.
pub struct World {
    // TODO: make it private after we can access it immutable via a method! (see #7)
    pub chunks: HashMap<ChunkIndex, Chunk>,
}

impl World {
    /// Creates an empty world without any chunks.
    pub fn empty() -> Self {
        World { chunks: HashMap::new() }
    }

    /// Returns a dummy world with one dummy chunk for early testing.
    /// FIXME: remove
    pub fn dummy() -> Self {
        let mut chunks = HashMap::new();
        chunks.insert(ChunkIndex(AxialPoint::new(0, 0)), Chunk::dummy());
        World { chunks: chunks }
    }

    pub fn add_chunk(&mut self, index: ChunkIndex, provider: &ChunkProvider) {
        match provider.load_chunk(index) {
            Some(x) => {
                self.chunks.insert(index, x);
                info!("Chunk {:?} loaded", index);
            }
            None => error!("Chunk {:?} not loadable!", index),
        }
    }

    /// Returns the hex pillar at the given world position, iff the
    /// corresponding chunk is loaded.
    pub fn pillar_at(&self, pos: PillarIndex) -> Option<&HexPillar> {
        // TODO: use `/` operator once it's implemented
        // let chunk_pos = pos / (super::CHUNK_SIZE as i32);
        let chunk_pos = AxialPoint::new(pos.0.q / (super::CHUNK_SIZE as i32),
                                        pos.0.r / (super::CHUNK_SIZE as i32));

        let out = self.chunks.get(&ChunkIndex(chunk_pos)).map(|chunk| {
            // TODO: use `%` operator once it's implemented
            // let inner_pos = pos % (super::CHUNK_SIZE as i32);
            let inner_pos = AxialPoint::new(pos.0.q % (super::CHUNK_SIZE as i32),
                                            pos.0.r % (super::CHUNK_SIZE as i32));
            &chunk[inner_pos]
        });

        if out.is_none() {
            debug!("chunk {:?} is not loaded (position request {:?})",
                   chunk_pos,
                   pos);
        }

        out
    }

    /// Returns the chunk in which the given pillar exists.
    pub fn chunk_from_pillar(&self, pos: PillarIndex) -> Option<&Chunk> {
        let tmp = AxialPoint::new(pos.0.q / (super::CHUNK_SIZE as i32),
                                  pos.0.r / (super::CHUNK_SIZE as i32));
        let chunk_pos = ChunkIndex(tmp);
        self.chunk_at(chunk_pos)
    }

    /// Returns the requested chunk.
    pub fn chunk_at(&self, pos: ChunkIndex) -> Option<&Chunk> {
        let out = self.chunks.get(&pos);

        if out.is_none() {
            debug!("chunk {:?} is not loaded", pos);
        }

        out
    }
}
