use base::world::{self, Chunk};
use base::math::*;
use glium;
use Camera;
use render::ToArr;
use std::collections::VecDeque;
use std::f32::consts;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    vertices: glium::VertexBuffer<Vertex>,
    program: glium::Program,
    pillars_positions: Vec<Point2f>,
    index_buffer: glium::index::IndexBuffer<u32>,
}
/// Calculates one Point-coordinates of a Hexagon
fn hex_corner(size: f32, i: i32) -> (f32, f32) {
    let angle_deg = 60.0 * (i as f32) + 30.0;
    let angle_rad = (consts::PI / 180.0) * angle_deg;

    (size * angle_rad.cos(), size * angle_rad.sin())
}

impl ChunkView {
    /// Creates the graphical representation of given chunk at the given chunk
    /// offset
    pub fn from_chunk<F>(_chunk: &Chunk, offset: AxialPoint, facade: &F) -> Self
        where F: glium::backend::Facade
    {

        // Create one hexagon for this chunk
        let mut hexagon_vertex_buffer = VecDeque::new();
        for i in 0..6 {
            let (x, y) = hex_corner(world::HEX_OUTER_RADIUS, i);

            hexagon_vertex_buffer.push_front(Vertex {
                position: [x, y, world::PILLAR_STEP_HEIGHT],
                color: [0.15 * i as f32, 0.0, 0.0],
            });

            hexagon_vertex_buffer.push_back(Vertex {
                position: [x, y, -1.0 * world::PILLAR_STEP_HEIGHT],
                color: [1.0 - 0.15 * i as f32, 0.15 * i as f32, 0.0],
            });

        }

        // convert to vector
        let mut final_buffer = Vec::new();
        for element in hexagon_vertex_buffer {
            final_buffer.push(element);
        }


        let vbuf = glium::VertexBuffer::new(facade, &final_buffer).unwrap();
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

        // Indecies
        let raw_index_buffer = [5, 0, 1, 2, 5, 1, 4, 5, 2, 3, 4, 2 /* TOP */, 6, 7, 8, 8, 9,
                                6, 9, 11, 6, 9, 10, 11 /* BOTTOM */, 6, 5, 4, 7, 6, 4, 6, 0,
                                5, 11, 0, 6, 10, 1, 0, 11, 10, 0, 9, 2, 1, 10, 9, 1, 8, 3, 2, 9,
                                8, 2, 7, 4, 3, 8, 7, 3 /* Body */];


        let ibuf = glium::index::IndexBuffer::new(facade,
                                                  glium::index::PrimitiveType::TrianglesList,
                                                  &raw_index_buffer)
            .unwrap();

        ChunkView {
            vertices: vbuf,
            program: prog,
            pillars_positions: positions,
            index_buffer: ibuf,
        }
    }

    pub fn draw<S>(&self, surface: &mut S, camera: &Camera)
        where S: glium::Surface
    {

        for pillar_pos in &self.pillars_positions {
            let uniforms = uniform!{
              scale_matrix:[[1.0,0.0,0.0,0.0]
                      ,[0.0,1.0,0.0,0.0],
                      //FIXME 5.0 heightvalue
                      [0.0,0.0,1.0,0.0],
                      [0.0,0.0,0.0,1.0f32]],
                offset: [pillar_pos.x, pillar_pos.y],
                proj_matrix: camera.proj_matrix().to_arr(),
                view_matrix: camera.view_matrix().to_arr(),
            };

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    write: true,
                    test: glium::draw_parameters::DepthTest::IfLess,
                    ..Default::default()
                },
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
                ..Default::default()
            };

            surface.draw(&self.vertices,
                      &self.index_buffer,
                      &self.program,
                      &uniforms,
                      &params)
                .unwrap();

        }

    }
}

#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);
