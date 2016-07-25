use base::world::{self, World, Chunk};
use base::math::*;
use glium::backend::Facade;
use glium;
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

        // Iterating through the Chunk by using the closure function in `Chunk`
        Chunk::with_pillars(|axial| {
            pillars_pos.push(axial.to_real());
        });

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

        WorldView { chunks: chunks }
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        for chunkview in &self.chunks {
            chunkview.draw(surface, camera);
        }
    }
}
