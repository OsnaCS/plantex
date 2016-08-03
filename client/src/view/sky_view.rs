use std::rc::Rc;
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use GameContext;
use Camera;
use glium::draw_parameters::DepthTest;
use util::ToArr;
use base::math::*;
use noise::{PermutationTable, open_simplex2};
use rand::Rand;
use base::gen::seeded_rng;
use glium::texture::Texture2d;



pub struct SkyView {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program,
    sun_position: Point3f,
    star_map: Texture2d,
}

impl SkyView {
    pub fn new(context: Rc<GameContext>) -> Self {
        const SKYDOME_SIZE: f32 = 500.0;
        let raw_vertex_buffer = vec![
            // a: part of xy-plane
            Vertex { i_position: [0.0, -SKYDOME_SIZE, 0.0], i_unit_coords: [0.0, -1.0, 0.0]},
            // b: part of xy-plane
            Vertex { i_position: [SKYDOME_SIZE, 0.0, 0.0], i_unit_coords: [1.0, 0.0, 0.0]},
            // c: part of xy-plane
            Vertex { i_position: [0.0, SKYDOME_SIZE, 0.0], i_unit_coords: [0.0, 1.0, 0.0]},
            // d: part of xy-plane
            Vertex { i_position: [-SKYDOME_SIZE, 0.0, 0.0], i_unit_coords: [-1.0, 0.0, 0.0]},
            // e: peak of lower hemisphere
            Vertex { i_position: [0.0, 0.0, -SKYDOME_SIZE], i_unit_coords: [0.0, 0.0, -1.0]},
            // f: peak of upper hemisphere
            Vertex { i_position: [0.0, 0.0, SKYDOME_SIZE], i_unit_coords: [0.0, 0.0, 1.0]},
        ];

        let vbuf = VertexBuffer::new(context.get_facade(), &raw_vertex_buffer).unwrap();

        // Indices
        // Index-Buffer corresponds to the planes of the octaeder
        // [a, b, e] [b, c, e] [c, d, e] [d, a, e] [b, a, f] [a, d, f] [d, c, f] [c, b,
        // f]
        let raw_index_buffer = [0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4, 1, 0, 5, 0, 3, 5, 3, 2, 5, 2,
                                1, 5]; //TrianglesList
        let ibuf = IndexBuffer::new(context.get_facade(),
                                    PrimitiveType::TrianglesList,
                                    &raw_index_buffer)
            .unwrap();

        let seed = 54342354434;
        let mut star_rng = seeded_rng(seed, 0, ());
        let star_table = PermutationTable::rand(&mut star_rng);


        // values between 0 and 0.5
        const TEX_SIZE: usize = 2048;
        let mut v = vec![Vec::new(); TEX_SIZE];

        for i in 0..TEX_SIZE {
            for j in 0..TEX_SIZE * 2 {
                v[i].push((open_simplex2::<f32>(&star_table,
                                                &[(i as f32) * 0.1, (j as f32) * 0.1]) +
                           0.6) / 2.0);
            }
        }

        SkyView {
            vertex_buffer: vbuf,
            index_buffer: ibuf,
            program: context.load_program("skydome").unwrap(),
            sun_position: Point3f::new(0.0, 0.0, -1000.0),
            star_map: Texture2d::new(context.get_facade(), v).expect("Could not load stars"),

        }
    }

    pub fn draw_skydome<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {

        let pos = Point3::new(0.0, 0.0, 0.0);

        // skydome-octaeder position is set to camera position
        let view_matrix =
            Matrix4::look_at(pos, pos + camera.get_look_at_vector(), Vector3::unit_z());

        let uniforms = uniform! {
            u_proj_matrix: camera.proj_matrix().to_arr(),
            u_view_matrix: view_matrix.to_arr(),
            u_sun_pos: self.sun_position.to_arr(),
            u_star_map: &self.star_map,
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

    pub fn update(&mut self, pos: Point3f) {
        self.sun_position = pos;
    }
}

/// Vertex type used to render chunks (or hex pillars).
#[derive(Debug, Copy, Clone)]
struct Vertex {
    pub i_position: [f32; 3],
    pub i_unit_coords: [f32; 3],
}

implement_vertex!(Vertex, i_position, i_unit_coords);
