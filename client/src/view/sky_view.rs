use glium::backend::Facade;
use std::rc::Rc;
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use GameContext;
use Camera;
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use util::ToArr;
use base::math::*;

pub struct SkyView {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program,
    context: Rc<GameContext>,
}

impl SkyView {
    pub fn new(context: Rc<GameContext>) -> Self {

        const SKYDOME_SIZE: f32 = 1000.0;
        let raw_vertex_buffer = vec![
            Vertex { position: [0.0, -SKYDOME_SIZE, 0.0], color: [0.7, 0.0, 0.0]},
            Vertex { position: [SKYDOME_SIZE, 0.0, 0.0], color: [0.5, 0.2, 0.0]},
            Vertex { position: [0.0, SKYDOME_SIZE, 0.0], color: [0.0, 0.0, 0.7]},
            Vertex { position: [-SKYDOME_SIZE, 0.0, 0.0], color: [0.0, 0.0, 0.7]},
            Vertex { position: [0.0, 0.0, -SKYDOME_SIZE], color: [0.0, 0.0, 1.0]},
            Vertex { position: [0.0, 0.0, SKYDOME_SIZE], color: [0.0, 0.0, 1.0]},
        ];

        //         let raw_vertex_buffer = vec![
        //     Vertex { position: [0.0, -60.0, 60.0], color: [1.0, 0.0, 0.0]},
        //     Vertex { position: [60.0, 0.0, 60.0], color: [0.0, 1.0, 0.0]},
        //     Vertex { position: [0.0, 60.0, 60.0], color: [0.0, 0.0, 1.0]},
        //     Vertex { position: [-60.0, 0.0, 60.0], color: [0.0, 0.0, 1.0]},
        //     Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.0, 1.0]},
        //     Vertex { position: [0.0, 0.0, 120.0], color: [0.0, 0.0, 1.0]},
        // ];

        let vbuf = VertexBuffer::new(context.get_facade(), &raw_vertex_buffer).unwrap();

        // Indices
        let raw_index_buffer = [0, 1, 4, 1, 2, 4, 2,3,4,3,0,4,1,0,5,0,3,5,3,2,5,2,1,5]; //TrianglesList
        // let raw_index_buffer = [0, 1, 4, 2, -1, 2, 4, 3, 0, -1, 3, 2, 5, 1, -1, 3, 5, 0, 1]; // would need primitive restart
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
            context: context,
        }
    }

    pub fn draw_skydome<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {

        let pos = Point3::new(0.0, 0.0, 0.0);
        let mut view_matrix =
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
}

implement_vertex!(Vertex, position, color);
