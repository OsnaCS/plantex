use base::world::{self, Chunk, ChunkIndex, World};
use base::math::*;
use glium::backend::Facade;
use glium;
use Camera;
use std::collections::HashMap;
use std::rc::Rc;
use view::PlantRenderer;

pub use world::chunk_view::ChunkView;
pub use view::PlantView;

/// Graphical representation of the `base::World`.
pub struct WorldView {
    chunks: HashMap<ChunkIndex, ChunkView>,
    plant_renderer: Rc<PlantRenderer>,
}

impl WorldView {
    pub fn from_world<F: Facade>(world: &World, facade: &F) -> Self {
        let plant_renderer = Rc::new(PlantRenderer::new(facade));

        let mut chunks = HashMap::new();
        for chunkkey in world.chunks.keys() {
            // FIX: 1 is HEX_OUTER_RADIUS, but thats a float
            chunks.insert(*chunkkey,
                          ChunkView::from_chunk(world.chunks.get(chunkkey).unwrap(),
                                                AxialPoint::new(chunkkey.0.q *
                                                                (1 * world::CHUNK_SIZE as i32),
                                                                chunkkey.0.r *
                                                                (1 * world::CHUNK_SIZE as i32)),
                                                plant_renderer.clone(),
                                                facade));
        }

        WorldView {
            chunks: chunks,
            plant_renderer: plant_renderer,
        }
    }

    pub fn refresh_chunk<F: Facade>(&mut self, chunk_pos: ChunkIndex, chunk: &Chunk, facade: &F) {
        self.chunks.insert(chunk_pos,
                           ChunkView::from_chunk(chunk,
                                                 AxialPoint::new(chunk_pos.0.q *
                                                                 (1 * world::CHUNK_SIZE as i32),
                                                                 chunk_pos.0.r *
                                                                 (1 * world::CHUNK_SIZE as i32)),
                                                 self.plant_renderer.clone(),
                                                 facade));
    }

    pub fn remove_chunk(&mut self, chunk_pos: ChunkIndex) {
        self.chunks.remove(&chunk_pos);
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        for chunkview in self.chunks.values() {
            chunkview.draw(surface, camera);
        }
    }
}
