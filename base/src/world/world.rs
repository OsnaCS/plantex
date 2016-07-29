use std::collections::HashMap;
use super::{Chunk, ChunkIndex, HexPillar, PillarIndex};
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

    /// Inserts the given chunk into the world and replaces the chunk that
    /// might have been at the given position before.
    pub fn replace_chunk(&mut self, index: ChunkIndex, chunk: Chunk) {
        // TODO: we might want to return the replaced chunk...
        self.chunks.insert(index, chunk);
        debug!("inserted chunk at position {:?}", index);
    }

    /// Inserts the given chunk at the given position, if there wasn't a chunk
    /// at that position before. In the latter case the given chunk is returned.
    pub fn add_chunk(&mut self, index: ChunkIndex, chunk: Chunk) -> Result<(), Chunk> {
        if self.chunks.contains_key(&index) {
            Err(chunk)
        } else {
            self.replace_chunk(index, chunk);
            Ok(())
        }
    }

    /// Returns the hex pillar at the given world position, iff the
    /// corresponding chunk is loaded.
    pub fn pillar_at(&self, pos: PillarIndex) -> Option<&HexPillar> {
        let chunk_size = super::CHUNK_SIZE as i32;
        let mut new_pos = pos;
        // TODO: use `/` operator once it's implemented
        // let chunk_pos = pos / (super::CHUNK_SIZE as i32);
        if new_pos.0.q < 0 {
            new_pos.0.q -= 15;
        }
        if new_pos.0.r < 0 {
            new_pos.0.r -= 15;
        }
        let chunk_pos = new_pos.0 / chunk_size;
        let out = self.chunks.get(&ChunkIndex(chunk_pos)).map(|chunk| {
            let mut inner_pos = pos.0 % chunk_size;
            if inner_pos.q < 0 {
                inner_pos.q += chunk_size;
            }
            if inner_pos.r < 0 {
                inner_pos.r += chunk_size;
            }
            if chunk_pos.q != chunk_pos.r {
                ::std::mem::swap(&mut inner_pos.r, &mut inner_pos.q);
            }
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
