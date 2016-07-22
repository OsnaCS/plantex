use base::world::{self, ChunkIndex, World};
use base::math::*;
use glium;
use Camera;

mod chunk;
mod plant_view;

pub use self::chunk::ChunkView;
pub use self::plant_view::PlantView;

/// Graphical representation of the `base::World`.
pub struct WorldView {
    plant_view: PlantView,
    chunks: Vec<ChunkView>,
}

impl WorldView {
    pub fn from_world<F>(world: &World, facade: &F) -> Self
        where F: glium::backend::Facade
    {
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
            plant_view: PlantView::from_dummy_plant(&world.chunks[&ChunkIndex(AxialPoint::new(0,
                                                                                              0))]
                                                        .pillars(),
                                                    pillars_pos,
                                                    facade),
        }


    }


    pub fn draw<S>(&self, surface: &mut S, camera: &Camera)
        where S: glium::Surface
    {
        for chunkview in &self.chunks {
            chunkview.draw(surface, camera);
        }
        self.plant_view.draw(surface, camera);
    }
}
