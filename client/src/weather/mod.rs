extern crate rand;

use glium::{self, DrawParameters, Program, VertexBuffer};
use glium::draw_parameters::DepthTest;
use super::camera::Camera;
use util::ToArr;
use base::math::*;
use GameContext;
use std::rc::Rc;

const BOX_SIZE: f32 = 20.0;

#[derive(Copy, Clone)]
pub struct Vertex {
    point: [f32; 3],
}
implement_vertex!(Vertex, point);

#[derive(Debug, Copy, Clone)]
pub enum Form {
    #[allow(dead_code)]
    None = 0,
    #[allow(dead_code)]
    Rain = 1,
    #[allow(dead_code)]
    Snow = 2,
    #[allow(dead_code)]
    Pollen = 3,
}

#[derive(Copy, Clone)]
pub struct Particle {
    vertices: [Vertex; 4],
    position: Point3<f32>,
    velocity: f32,
    trans_x: f32,
    trans_y: f32,
}

#[derive(Copy, Clone)]
pub struct Instance {
    position: [f32; 3],
}
implement_vertex!(Instance, position);

impl Particle {
    pub fn new(cam_pos: Point3<f32>) -> Particle {
        let mut radius: f32 = 0.0;
        let mut spawn = false;

        // loop provides better distribution of particles, by rolling a dice
        // which equalizes the chance for every radius inside the circle
        while !spawn {
            radius = rand::random::<f32>();

            let dice = rand::random::<f32>();

            spawn = dice < radius;
        }

        radius = radius * BOX_SIZE;

        let angle = Rad::new(rand::random::<f32>() * 6.28);

        let mut velocity = rand::random::<f32>();
        if velocity < 0.25 {
            velocity = velocity + 0.25;
        }
        if velocity > 0.75 {
            velocity = velocity - 0.25;
        }

        Particle {
            vertices: [Vertex { point: [1.0, 1.0, 0.0] },
                       Vertex { point: [-1.0, 1.0, 0.0] },
                       Vertex { point: [1.0, -1.0, 0.0] },
                       Vertex { point: [-1.0, -1.0, 0.0] }],
            position: Point3::new(cam_pos.x + radius * Rad::sin(angle),
                                  cam_pos.y + radius * Rad::cos(angle),
                                  cam_pos.z + BOX_SIZE),
            velocity: velocity,
            trans_x: rand::random::<f32>() * 6.28,
            trans_y: rand::random::<f32>() * 6.28,
        }
    }
}


pub struct Weather {
    particles: Vec<Particle>,
    program: Program,
    indices: glium::index::NoIndices,
    context: Rc<GameContext>,
    camera: Camera,
    particle_buf: VertexBuffer<Instance>,
    form: Form,
}

impl Weather {
    pub fn new(context: Rc<GameContext>) -> Weather {
        let vec: Vec<Particle> = Vec::new();
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        let weather_program = context.load_program("weather").unwrap();
        let camera = Camera::default();
        let sections = glium::VertexBuffer::new(context.get_facade(), &[]).unwrap();
        Weather {
            particles: vec,
            program: weather_program,
            indices: index_buffer,
            context: context,
            camera: camera,
            particle_buf: sections,
            form: Form::None,
        }
    }


    pub fn draw<S: glium::Surface>(&mut self, surface: &mut S, camera: &Camera) {
        if self.form as u8 == 0 {
            return;
        }




        let params = DrawParameters {
            depth: glium::Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            ..Default::default()
        };



        let buffer = vec![Vertex { point: [1.0, 1.0, 0.0] },
                          Vertex { point: [-1.0, 1.0, 0.0] },
                          Vertex { point: [1.0, -1.0, 0.0] },
                          Vertex { point: [-1.0, -1.0, 0.0] }];
        let vertex_buffer = glium::VertexBuffer::new(self.context.get_facade(), &buffer).unwrap();
        let mut scaling = Matrix4::from_cols(Vector4f::new(1.0, 0.0, 0.0, 0.0),
                                             Vector4f::new(0.0, 1.0, 0.0, 0.0),
                                             Vector4f::new(0.0, 0.0, 1.0, 0.0),
                                             Vector4f::new(0.0, 0.0, 0.0, 1.0));
        let view = camera.view_matrix();
        let mut color = Vector4f::new(0.0, 0.0, 0.0, 0.0);
        match self.form {
            Form::Snow => {
                color = Vector4f::new(0.7, 0.7, 0.7, 1.0);

                scaling = Matrix4::from_cols(Vector4f::new(0.05, 0.0, 0.0, 0.0),
                                             Vector4f::new(0.0, 0.05, 0.0, 0.0),
                                             Vector4f::new(0.0, 0.0, 0.05, 0.0),
                                             Vector4f::new(0.0, 0.0, 0.0, 1.0));
            }
            Form::Rain => {
                color = Vector4f::new(0.35, 0.43, 0.47, 1.0);



                scaling = Matrix4::from_cols(Vector4f::new(0.01, 0.0, 0.0, 0.0),
                                             Vector4f::new(0.0, 0.05, 0.0, 0.0),
                                             Vector4f::new(0.0, 0.0, 0.5, 0.0),
                                             Vector4f::new(0.0, 0.0, 0.0, 1.0));
            }
            _ => (),
        };


        let uniforms = uniform!{
                form: self.form as i32,
                view_matrix: view.to_arr(),
                proj_matrix: camera.proj_matrix().to_arr(),
                scaling_matrix: scaling.to_arr(),
                color: color.to_arr(),
            };


        surface.draw((&vertex_buffer, self.particle_buf.per_instance().unwrap()),
                  &self.indices,
                  &self.program,
                  &uniforms,
                  &params)
            .unwrap();



    }


    pub fn update(&mut self, camera: &Camera) {
        if self.form as u8 == 0 {
            return;
        }
        self.camera = *camera;
        if self.particles.len() < 2000 {
            let count = 2000 - self.particles.len();
            for _ in 0..count {
                self.particles.push(Particle::new(camera.position));
            }
            let mut tmp = Vec::new();
            for particle in self.particles.iter() {
                tmp.push(Instance {
                    position: [particle.position.x, particle.position.y, particle.position.z],
                })
            }
            let vertex_buffer = glium::VertexBuffer::new(self.context.get_facade(), &tmp).unwrap();
            self.particle_buf = vertex_buffer;
        }

        let mut mapping = self.particle_buf.map();
        for (particle, instance) in &mut self.particles.iter_mut().zip(mapping.iter_mut()) {
            match self.form {
                Form::Snow => {
                    instance.position[2] = instance.position[2] - (particle.velocity * 0.1);
                    instance.position[0] += particle.trans_x.sin() * 0.05;
                    instance.position[1] += particle.trans_y.cos() * 0.05;
                    particle.trans_y += 3.14 * 0.005 * (rand::random::<f32>() - 0.5);
                    particle.trans_x += 3.14 * 0.005 * (rand::random::<f32>() - 0.5);
                }

                Form::Rain => {
                    instance.position[2] = instance.position[2] - ((particle.velocity + 0.5) * 1.0)
                }
                _ => (),

            }


            let pix_vec = Point2::new(instance.position[0], instance.position[1]);
            let cam_vec = Point2::new(camera.position.x, camera.position.y);
            let tmp = pix_vec - cam_vec;
            let len = ((tmp.x * tmp.x) + (tmp.y * tmp.y)).sqrt();

            if len > BOX_SIZE || len < -BOX_SIZE {
                instance.position[0] = instance.position[0] - (2.0 * tmp.x);
                instance.position[1] = instance.position[1] - (2.0 * tmp.y);
            }
            if instance.position[2] < self.camera.position.z - BOX_SIZE {
                instance.position[2] += 2.0 * BOX_SIZE;
            }
            if instance.position[2] > self.camera.position.z + BOX_SIZE {
                instance.position[2] -= 2.0 * BOX_SIZE;
            }
        }
    }
}
