use base::world::{self, Chunk};
use base::math::*;
use glium;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    vertices: glium::VertexBuffer<Vertex>,
    program: glium::Program,
    pillars_positions: Vec<Point2f>,
}

impl ChunkView {
    /// Creates the graphical representation of given chunk at the given chunk
    /// offset
    pub fn from_chunk<F>(_chunk: &Chunk, offset: AxialPoint, facade: &F) -> Self
        where F: glium::backend::Facade
    {
        let buffer = vec![Vertex {
                              position: [0.0, 0.0, 0.0],
                              color: [1.0, 0.0, 0.0],
                          },
                          Vertex {
                              position: [1.0, 0.0, 0.0],
                              color: [0.0, 1.0, 0.0],
                          },
                          Vertex {
                              position: [0.0, 1.0, 0.0],
                              color: [0.0, 0.0, 1.0],
                          }];
        let vbuf = glium::VertexBuffer::new(facade, &buffer).unwrap();
        let ibuf = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let prog = glium::Program::from_source(facade,
                                               include_str!("chunk_std.vert"),
                                               include_str!("chunk_std.frag"),
                                               None)
            .unwrap();

        let mut positions = Vec::new();
        for q in 0..world::CHUNK_SIZE {
            for r in 0..world::CHUNK_SIZE {
                let pos = offset.to_real() + AxialVector::new(q.into(), r.into()).to_real();
                positions.push(pos);
            }
        }

        ChunkView {
            vertices: vbuf,
            program: prog,
            pillars_positions: positions,
        }
    }

    pub fn draw<S>(&self, surface: &mut S)
        where S: glium::Surface
    {

        let ibuf = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        for pillar_pos in &self.pillars_positions {
            surface.draw(&self.vertices,
                      &ibuf,
                      &self.program,
                      &uniform!{},
                      &Default::default())
                .unwrap();

        }

    }
}


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);
