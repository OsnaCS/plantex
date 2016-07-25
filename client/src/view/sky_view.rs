// use glium::backend::Facade;
use std::rc::Rc;
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use GameContext;
use Camera;
use glium::draw_parameters::{DepthTest};
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
            Vertex { position: [0.0, -SKYDOME_SIZE, 0.0], color: [0.7, 0.0, 0.0], theta: consts::PI/2.0, phi: 45.0f32.to_radians()}, //a
            Vertex { position: [SKYDOME_SIZE, 0.0, 0.0], color: [0.5, 0.2, 0.0], theta: consts::PI/2.0, phi: 135.0f32.to_radians()}, //b
            Vertex { position: [0.0, SKYDOME_SIZE, 0.0], color: [0.0, 0.0, 0.7], theta: consts::PI/2.0, phi: 225.0f32.to_radians()}, //c
            Vertex { position: [-SKYDOME_SIZE, 0.0, 0.0], color: [0.0, 0.0, 0.7], theta: consts::PI/2.0, phi: 315.0f32.to_radians()}, //d
            Vertex { position: [0.0, 0.0, -SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: consts::PI, phi: 45.0f32.to_radians()}, //e for ab
            Vertex { position: [0.0, 0.0, -SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: consts::PI, phi: 135.0f32.to_radians()}, //e for bc
            Vertex { position: [0.0, 0.0, -SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: consts::PI, phi: 225.0f32.to_radians()}, //e for cd
            Vertex { position: [0.0, 0.0, -SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: consts::PI, phi: 315.0f32.to_radians()}, //e for da
            Vertex { position: [0.0, 0.0, SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: 0.0, phi: 45.0f32.to_radians()}, //f for ab
            Vertex { position: [0.0, 0.0, SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: 0.0, phi: 135.0f32.to_radians()}, //f for bc
            Vertex { position: [0.0, 0.0, SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: 0.0, phi: 225.0f32.to_radians()}, //f for cd
            Vertex { position: [0.0, 0.0, SKYDOME_SIZE], color: [0.0, 0.0, 1.0], theta: 0.0, phi: 315.0f32.to_radians()}, //f for da
        ];

        let vbuf = VertexBuffer::new(context.get_facade(), &raw_vertex_buffer).unwrap();

        // Indices
        let raw_index_buffer = [0, 1, 4, 1, 2, 5, 2,3,6,3,0,7,1,0,8,0,3,9,3,2,10,2,1,11]; //TrianglesList
        // let raw_index_buffer = [0, 1, 4, 1, 2, 4, 2,3,4,3,0,4,1,0,8,0,3,8,3,2,8,2,1,8]; //TrianglesList //with all the same e's and f's for testing
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
    pub color: [f32; 3],
    pub theta: f32,
    pub phi: f32,
}

implement_vertex!(Vertex, position, color, theta, phi);
