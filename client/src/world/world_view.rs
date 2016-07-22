use base::world::{self, World};
use base::math::*;
use glium::backend::Facade;
use glium::Surface;
use Camera;

pub use world::chunk_view::ChunkView;
pub use world::plant_view::PlantView;

/// Graphical representation of the `base::World`.
pub struct WorldView {
    chunks: Vec<ChunkView>,
}

impl WorldView {
    pub fn from_world<F: Facade>(world: &World, facade: &F) -> Self {
        let mut pillars_pos = Vec::new();
        for q in 0..world::CHUNK_SIZE {
            for r in 0..world::CHUNK_SIZE {
                let pos = AxialPoint::new(q.into(), r.into()).to_real();
                pillars_pos.push(pos);
            }
        }

        let mut chunks = Vec::new();
        for chunkkey in world.chunks.keys() {
            // FIX: 1 is HEX_OUTER_RADIUS, but thats a float
            chunks.push(ChunkView::from_chunk(world.chunks.get(chunkkey).unwrap(),
                                              AxialPoint::new(chunkkey.0.q *
                                                              (1 * world::CHUNK_SIZE as i32),
                                                              chunkkey.0.r *
                                                              (1 * world::CHUNK_SIZE as i32)),
                                              facade));
        }

        WorldView {
            chunks: chunks,
        }
    }

    pub fn draw<S: Surface>(&self, surface: &mut S, camera: &Camera) {
        for chunkview in &self.chunks {
            chunkview.draw(surface, camera);
        }
    }
}
