use glium::{self, DrawParameters, Frame, Program, Surface};
use std::rc::Rc;
use super::GameContext;
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use super::camera::Camera;
use std::cell::Ref;

#[derive(Copy, Clone)]
pub struct Particle {
    position: [f32; 2],
}

implement_vertex!(Particle, position);

pub struct Weather {
    particles: glium::VertexBuffer<Particle>,
    program: Program,
    indices: glium::index::NoIndices,
    facade: GlutinFacade,
}

impl Weather {
    pub fn new(facade: GlutinFacade) -> Weather {
        let particle1 = Particle { position: [1.0, 1.0] };
        let particle2 = Particle { position: [-1.0, 1.0] };
        let particle3 = Particle { position: [1.0, -1.0] };
        let particle4 = Particle { position: [-1.0, -1.0] };
        let buffer = vec![particle1, particle2, particle3, particle4];
        let vertex_buffer = glium::VertexBuffer::new(&facade, &buffer).unwrap();
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        let weather_program = glium::Program::from_source(&facade,
                                                          include_str!("weather.vert"),
                                                          include_str!("weather.frag"),
                                                          None)
            .unwrap();
        Weather {
            particles: vertex_buffer,
            program: weather_program,
            indices: index_buffer,
            facade: facade,
        }
    }



    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let params = DrawParameters {
            depth: glium::Depth {
                write: true,
                test: DepthTest::IfLess,

                ..Default::default()
            },
            point_size: Some(35.0),
            // backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        surface.draw(&self.particles,
                  &self.indices,
                  &self.program,
                  &glium::uniforms::EmptyUniforms,
                  &params)
            .unwrap();
    }

    pub fn update(&self) {}
}
