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
    hexagon_vertex_buffer: Vec<Vertex>,
    index_buffer: glium::index::IndexBuffer<u32>,
}

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
        let mut hexagon_vertex_buffer = VecDeque::new();

        for i in 0..6 {
            let raw_modell = hex_corner(world::HEX_OUTER_RADIUS, i);

            hexagon_vertex_buffer.push_front(Vertex {
                position: [raw_modell.0, raw_modell.1, world::PILLAR_STEP_HEIGHT / 2.0],
                color: [1.0, 0.0, 0.0],
            });

            hexagon_vertex_buffer.push_back(Vertex {
                position: [raw_modell.0, raw_modell.1, -1.0 * (world::PILLAR_STEP_HEIGHT / 2.0)],
                color: [0.0, 1.0, 0.0],
            });

        }


        let mut final_buffer = Vec::new();
        for element in hexagon_vertex_buffer {
            final_buffer.push(element);
        }

        // println!("{:#?}", final_buffer);

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

        let mut raw_index_buffer = [1, 0, 5, 1, 5, 2, 2, 5, 4, 2, 4, 3 /* TOP */, 6, 7, 8, 8,
                                    9, 6, 9, 11, 6, 9, 10, 11 /* BOTTOM */, 4, 5, 6, 4, 6, 7,
                                    5, 0, 6, 6, 0, 11, 0, 1, 10, 0, 10, 11, 1, 2, 9, 1, 9, 10, 2,
                                    3, 8, 2, 8, 9, 3, 4, 7, 3, 7, 8];


        let ibuf = glium::index::IndexBuffer::new(facade,
                                                  glium::index::PrimitiveType::TrianglesList,
                                                  &raw_index_buffer)
            .unwrap();

        ChunkView {
            vertices: vbuf,
            program: prog,
            pillars_positions: positions,
            hexagon_vertex_buffer: final_buffer,
            index_buffer: ibuf,
        }
    }

    fn get_hexagon_model(&self) -> Vec<Vertex> {
        self.hexagon_vertex_buffer.clone()
    }

    pub fn draw<S>(&self, surface: &mut S, camera: &Camera)
        where S: glium::Surface
    {

        for pillar_pos in &self.pillars_positions {
            let uniforms = uniform!{
                offset: [pillar_pos.x, pillar_pos.y],
                proj_matrix: camera.proj_matrix().to_arr(),
                view_matrix: camera.view_matrix().to_arr(),
            };

            let params = glium::DrawParameters {
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
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
