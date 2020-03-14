use base::math::*;
use glium::draw_parameters::DepthTest;
use glium::index::PrimitiveType;
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use std::rc::Rc;
use util::ToArr;
use Camera;
use GameContext;

pub struct Sun {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program,
    position: Point3f,
}

const SUN_SIZE: f32 = 15.0;

impl Sun {
    pub fn new(context: Rc<GameContext>) -> Self {
        let raw_vertex_buffer = vec![
            Vertex {
                i_position: [SUN_SIZE, SUN_SIZE, 0.0],
                i_unit_coords: [1.0, 1.0, 0.0],
            },
            Vertex {
                i_position: [-SUN_SIZE, -SUN_SIZE, 0.0],
                i_unit_coords: [-1.0, -1.0, 0.0],
            },
            Vertex {
                i_position: [-SUN_SIZE, SUN_SIZE, 0.0],
                i_unit_coords: [-1.0, 1.0, 0.0],
            },
            Vertex {
                i_position: [SUN_SIZE, -SUN_SIZE, 0.0],
                i_unit_coords: [1.0, -1.0, 0.0],
            },
        ];

        let vbuf = VertexBuffer::new(context.get_facade(), &raw_vertex_buffer).unwrap();

        let raw_index_buffer = [2, 1, 0, 3]; //TrianglesStrip

        let ibuf = IndexBuffer::new(
            context.get_facade(),
            PrimitiveType::TriangleStrip,
            &raw_index_buffer,
        )
        .unwrap();

        Sun {
            vertex_buffer: vbuf,
            index_buffer: ibuf,
            program: context.load_program("sun").unwrap(),
            position: Point3f::new(0.0, 0.0, 0.0),
        }
    }

    pub fn draw_sun<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let sun_pos = self.position.to_vec() + camera.position.to_vec();
        let uniforms = uniform! {
            u_proj_matrix: camera.proj_matrix().to_arr(),
            u_view_matrix: camera.view_matrix().to_arr(),
            u_model: Matrix4::from_translation(sun_pos).to_arr(),
            sun_pos: sun_pos.normalize().to_arr() ,
        };
        let params = DrawParameters {
            depth: glium::Depth {
                write: false,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            ..Default::default()
        };

        surface
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &params,
            )
            .unwrap();
    }

    pub fn update(&mut self, pos: Point3f) {
        self.position = pos;
    }

    pub fn position(&self) -> Point3f {
        self.position
    }
}
#[derive(Debug, Copy, Clone)]
struct Vertex {
    pub i_position: [f32; 3],
    pub i_unit_coords: [f32; 3],
}

implement_vertex!(Vertex, i_position, i_unit_coords);
