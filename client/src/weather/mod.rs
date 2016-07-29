extern crate rand;

use glium::{self, DrawParameters, LinearBlendingFactor, Program, VertexBuffer};
use glium::draw_parameters::{BlendingFunction, DepthTest};
use super::camera::Camera;
use util::ToArr;
use base::math::*;
use base::world::PillarIndex;
use GameContext;
use std::rc::Rc;
use std::f32::consts::PI;

const BOX_SIZE: f32 = 20.0;

#[derive(Copy, Clone)]
pub struct Vertex {
    point: [f32; 3],
}
implement_vertex!(Vertex, point);

#[derive(Debug, Copy, Clone)]
pub enum Form {
    None = 0,
    Rain = 1,
    Snow = 2,
    Pollen = 3,
}

#[derive(Debug, Copy, Clone)]
pub enum Strength {
    Weak = 1,
    Medium = 2,
    Heavy = 3,
}

#[derive(Copy, Clone)]
pub struct Particle {
    vertices: [Vertex; 4],
    position: Point3<f32>,
    velocity: f32,
    trans_x: f32,
    trans_y: f32,
    trans_z: f32,
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

        let angle = Rad::new(rand::random::<f32>() * PI * 2.0);

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
            trans_x: rand::random::<f32>() * PI * 2.0,
            trans_y: rand::random::<f32>() * PI * 2.0,
            trans_z: rand::random::<f32>() * PI * 2.0,
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
    actual_buf: VertexBuffer<Instance>,
    form: Form,
    strength: Strength,
}

impl Weather {
    pub fn new(context: Rc<GameContext>) -> Weather {
        let vec: Vec<Particle> = Vec::new();
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        let weather_program = context.load_program("weather").unwrap();
        let camera = Camera::new(context.get_config().resolution.aspect_ratio());
        let sections = glium::VertexBuffer::new(context.get_facade(), &[]).unwrap();
        let sections2 = glium::VertexBuffer::new(context.get_facade(), &[]).unwrap();
        Weather {
            particles: vec,
            program: weather_program,
            indices: index_buffer,
            context: context,
            camera: camera,
            particle_buf: sections,
            actual_buf: sections2,
            form: Form::None,
            strength: Strength::Medium,
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
            blend: glium::Blend {
                color: BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                },
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
            Form::Pollen => {
                color = Vector4f::new(0.5, 0.5, 0.5, 1.0);

                scaling = Matrix4::from_cols(Vector4f::new(0.05, 0.0, 0.0, 0.0),
                                             Vector4f::new(0.0, 0.05, 0.0, 0.0),
                                             Vector4f::new(0.0, 0.0, 0.05, 0.0),
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


        surface.draw((&vertex_buffer, self.actual_buf.per_instance().unwrap()),
                  &self.indices,
                  &self.program,
                  &uniforms,
                  &params)
            .unwrap();



    }


    pub fn update(&mut self, camera: &Camera, delta: f32, world_manager: &super::WorldManager) {
        if self.form as u8 == 0 {
            return;
        }
        self.camera = *camera;
        let max_count: usize;
        match self.form {
            Form::Pollen => max_count = 25 * self.strength as usize,
            _ => max_count = 750 * self.strength as usize,
        }
        if self.particles.len() < max_count {
            let count = max_count / 10;
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
        let mut tmp2 = Vec::new();
        for (particle, instance) in &mut self.particles.iter_mut().zip(mapping.iter_mut()) {
            match self.form {
                Form::Snow => {
                    instance.position[2] = instance.position[2] - (particle.velocity * 3.0 * delta);
                    instance.position[0] += particle.trans_x.sin() * 0.05;
                    instance.position[1] += particle.trans_y.cos() * 0.05;
                    particle.trans_y += PI * 0.005 * (rand::random::<f32>() - 0.5);
                    particle.trans_x += PI * 0.005 * (rand::random::<f32>() - 0.5);
                }

                Form::Rain => {
                    instance.position[2] = instance.position[2] -
                                           ((particle.velocity + 0.5) * 50.0 * delta)
                }
                Form::Pollen => {
                    instance.position[0] += particle.trans_x.sin() * 0.025;
                    instance.position[1] += particle.trans_y.cos() * 0.025;
                    instance.position[2] += (particle.trans_z.sin() - 0.7) * 0.5 * delta;
                    particle.trans_y += PI * 0.005 * (rand::random::<f32>() - 0.5);
                    particle.trans_x += PI * 0.005 * (rand::random::<f32>() - 0.5);
                    particle.trans_z += PI * 0.005 * (rand::random::<f32>() - 0.1);
                }
                _ => (),

            }


            let pix_vec = Point2::new(instance.position[0], instance.position[1]);
            let cam_vec = Point2::new(camera.position.x, camera.position.y);
            let tmp = pix_vec - cam_vec;
            let len = ((tmp.x * tmp.x) + (tmp.y * tmp.y)).sqrt();


            let size = match self.form {
                Form::Pollen => BOX_SIZE / 4.0,
                _ => 1.0,
            };

            if len > BOX_SIZE - size || len < -BOX_SIZE - size {
                instance.position[0] = instance.position[0] - (1.95 * tmp.x);
                instance.position[1] = instance.position[1] - (1.95 * tmp.y);
            }
            if instance.position[2] < self.camera.position.z - BOX_SIZE / size {
                instance.position[2] += 1.9 * (BOX_SIZE / size);
            }
            if instance.position[2] > self.camera.position.z + BOX_SIZE / size {
                instance.position[2] -= 1.9 * (BOX_SIZE / size);
            }
            let tmp_point = Point3::new(instance.position[0],
                                        instance.position[1],
                                        instance.position[2]);
            let mut height = 0.0;

            let world = world_manager.get_world();
            let relevant_pos = Point2f::new(instance.position[0], instance.position[1]);
            let pillar_index = PillarIndex(AxialPoint::from_real(relevant_pos));
            let vec_len = world.pillar_at(pillar_index)
                .map(|pillar| pillar.sections().len())
                .unwrap_or(0);
            let pillar_vec = world.pillar_at(pillar_index).map(|pillar| pillar.sections());
            if pillar_vec.is_some() {
                let new_pillar_vec = pillar_vec.unwrap();

                if vec_len == 1 {
                    height = new_pillar_vec[0].top.to_real();
                } else {
                    for i in 0..vec_len {
                        if i != vec_len - 1 {
                            if new_pillar_vec[i].top.to_real() < instance.position[2] &&
                               instance.position[2] < new_pillar_vec[i + 1].bottom.to_real() {
                                height = new_pillar_vec[i].top.to_real();
                                break;
                            } else {
                                continue;
                            }
                        } else {
                            height = new_pillar_vec[i].top.to_real();
                            break;
                        }
                    }
                }
            }
            // let vec = world.pillar_at(pillar_index);
            if ((tmp_point - camera.position).dot(camera.get_look_at_vector())) > 0.0 &&
               instance.position[2] > height {
                tmp2.push(Instance {
                    position: [instance.position[0], instance.position[1], instance.position[2]],
                })
            }
        }
        let vertex_buffer = glium::VertexBuffer::new(self.context.get_facade(), &tmp2).unwrap();

        self.actual_buf = vertex_buffer;
    }

    pub fn set_weather(&mut self, form: Form, strength: Strength) {
        self.form = form;
        self.strength = strength;
    }
}
