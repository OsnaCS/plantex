use std::rc::Rc;
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use GameContext;
use Camera;
use glium::draw_parameters::DepthTest;
use util::ToArr;
use base::math::*;


pub struct Sun {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program,
    position: Vector3f,
}

const SUN_SIZE: f32 = 35.0;

impl Sun {
    pub fn new(context: Rc<GameContext>) -> Self {
        let raw_vertex_buffer = vec![
            Vertex { i_position: [SUN_SIZE, SUN_SIZE, 0.0], i_unit_coords: [1.0, 1.0, 0.0]},
            Vertex { i_position: [-SUN_SIZE, -SUN_SIZE, 0.0], i_unit_coords: [-1.0, -1.0, 0.0]},
            Vertex { i_position: [-SUN_SIZE, SUN_SIZE, 0.0], i_unit_coords: [-1.0, 1.0, 0.0]},
            Vertex { i_position: [SUN_SIZE, -SUN_SIZE, 0.0], i_unit_coords: [1.0, -1.0, 0.0]},
];

        let vbuf = VertexBuffer::new(context.get_facade(), &raw_vertex_buffer).unwrap();

        let raw_index_buffer = [2, 1, 0, 3]; //TrianglesStrip

        let ibuf = IndexBuffer::new(context.get_facade(),
                                    PrimitiveType::TriangleStrip,
                                    &raw_index_buffer)
            .unwrap();

        Sun {
            vertex_buffer: vbuf,
            index_buffer: ibuf,
            program: context.load_program("sun").unwrap(),
            position: Vector3f::new(0.0, 0.0, 0.0),
        }
    }

    pub fn draw_sun<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {

        let uniforms = uniform! {
            u_proj_matrix: camera.proj_matrix().to_arr(),
            u_view_matrix: camera.view_matrix().to_arr(),
            u_model: Matrix4::from_translation(camera.position.to_vec() + self.position).to_arr(),
        };
        let params = DrawParameters {
            depth: glium::Depth {
                write: false,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            ..Default::default()
        };

        surface.draw(&self.vertex_buffer,
                  &self.index_buffer,
                  &self.program,
                  &uniforms,
                  &params)
            .unwrap();
    }

    pub fn update(&mut self, pos: Vector3f) {
        self.position = pos;
    }
}
#[derive(Debug, Copy, Clone)]
struct Vertex {
    pub i_position: [f32; 3],
    pub i_unit_coords: [f32; 3],
}

implement_vertex!(Vertex, i_position, i_unit_coords);
