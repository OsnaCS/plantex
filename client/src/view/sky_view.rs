use std::rc::Rc;
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use GameContext;
use Camera;
use glium::draw_parameters::DepthTest;
use util::ToArr;
use base::math::*;
use std::f32::consts;

pub struct SkyView {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program,
}

impl SkyView {
    pub fn new(context: Rc<GameContext>) -> Self {

        const SKYDOME_SIZE: f32 = 1000.0;
        let raw_vertex_buffer = vec![
            Vertex { position: [0.0, -SKYDOME_SIZE, 0.0], xyz: [0.0, -1.0, 0.0]}, //a
            Vertex { position: [SKYDOME_SIZE, 0.0, 0.0], xyz: [1.0, 0.0, 0.0]}, //b
            Vertex { position: [0.0, SKYDOME_SIZE, 0.0], xyz: [0.0, 1.0, 0.0]}, //c
            Vertex { position: [-SKYDOME_SIZE, 0.0, 0.0], xyz: [-1.0, 0.0, 0.0]}, //d
            Vertex { position: [0.0, 0.0, -SKYDOME_SIZE], xyz: [0.0, 0.0, -1.0]}, //e
            Vertex { position: [0.0, 0.0, SKYDOME_SIZE], xyz: [0.0, 0.0, 1.0]}, //f

        ];

        let vbuf = VertexBuffer::new(context.get_facade(), &raw_vertex_buffer).unwrap();

        // Indices
        let raw_index_buffer = [0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4, 1, 0, 5, 0, 3, 5, 3, 2, 5, 2,
                                1, 5]; //TrianglesList
        let ibuf = IndexBuffer::new(context.get_facade(),
                                    PrimitiveType::TrianglesList,
                                    &raw_index_buffer)
            .unwrap();



        let prog = Program::from_source(context.get_facade(),
                                        include_str!("skydome.vert"),
                                        include_str!("skydome.frag"),
                                        None)
            .unwrap();


        SkyView {
            vertex_buffer: vbuf,
            index_buffer: ibuf,
            program: prog,
        }
    }

    pub fn draw_skydome<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {

        let pos = Point3::new(0.0, 0.0, 0.0);
        let view_matrix =
            Matrix4::look_at(pos, pos + camera.get_look_at_vector(), Vector3::unit_z());

        let uniforms = uniform! {
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: view_matrix.to_arr(),
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
}


/// Vertex type used to render chunks (or hex pillars).
#[derive(Debug, Copy, Clone)]
struct Vertex {
    pub position: [f32; 3],
    pub xyz: [f32; 3],
}

implement_vertex!(Vertex, position, xyz);
