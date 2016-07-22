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
    chunk: ChunkView,
    plant_view: PlantView,
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

        WorldView {
            chunk: ChunkView::from_chunk(&world.chunks[&ChunkIndex(AxialPoint::new(0, 0))],
                                         AxialPoint::new(0, 0),
                                         facade),
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
        self.chunk.draw(surface, camera);
        self.plant_view.draw(surface, camera);
    }
}
