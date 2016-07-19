use base::world::{ChunkIndex, World};
use base::math::*;
use glium;

mod chunk;

pub use self::chunk::ChunkView;

/// Graphical representation of the `base::World`.
pub struct WorldView {
    chunk: ChunkView,
}

impl WorldView {
    pub fn from_world<F>(world: &World, facade: &F) -> Self
        where F: glium::backend::Facade
    {
        WorldView {
            chunk: ChunkView::from_chunk(&world.chunks[&ChunkIndex(AxialPoint::new(0, 0))],
                                         AxialPoint::new(0, 0),
                                         facade),
        }
    }


    pub fn draw<S>(&self, surface: &mut S)
        where S: glium::Surface
    {
        self.chunk.draw(surface);
    }
}
