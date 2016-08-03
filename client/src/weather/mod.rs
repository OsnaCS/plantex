extern crate rand;

use glium::{self, DrawParameters, LinearBlendingFactor, Program, VertexBuffer};
use glium::draw_parameters::{BlendingFunction, DepthTest};
use super::camera::Camera;
use util::ToArr;
use base::math::*;
use base::gen::world::biome;
use base::world::PillarIndex;
use GameContext;
use std::rc::Rc;
use std::f32::consts::PI;
use std::f32;

const BOX_SIZE: f32 = 20.0;

#[derive(Copy, Clone)]
pub struct Vertex {
    point: [f32; 3],
}
implement_vertex!(Vertex, point);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Form {
    Rain = 1,
    Snow = 2,
    Pollen = 3,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Strength {
    None = 0,
    Weak = 1,
    Medium = 2,
    Heavy = 3,
}

#[derive(Copy, Clone)]
pub struct Particle {
    position: Point3<f32>,
    velocity: f32,
    trans_x: f32,
    trans_y: f32,
    trans_z: f32,
    lifetime: f32,
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

        // random value for downfall velocity
        let mut velocity = rand::random::<f32>();
        if velocity < 0.25 {
            velocity = velocity + 0.25;
        }
        if velocity > 0.75 {
            velocity = velocity - 0.25;
        }

        Particle {
            position: Point3::new(cam_pos.x + radius * Rad::sin(angle),
                                  cam_pos.y + radius * Rad::cos(angle),
                                  cam_pos.z + (BOX_SIZE * rand::random::<f32>() + 0.5)),
            velocity: velocity,
            trans_x: rand::random::<f32>() * PI * 2.0,
            trans_y: rand::random::<f32>() * PI * 2.0,
            trans_z: rand::random::<f32>() * PI * 2.0,
            lifetime: (rand::random::<f32>() + 0.5) * 100.0,
        }
    }

    /// returns true iff particle is inside a cave
    fn in_cave(&self, world_manager: &super::WorldManager) -> bool {
        let mut height = 0.0;
        let mut above = 0.0;

        let world = world_manager.get_world();
        let relevant_pos = Point2f::new(self.position.x, self.position.y);
        let pillar_index = PillarIndex(AxialPoint::from_real(relevant_pos));

        let vec_len = world.pillar_at(pillar_index)
            .map(|pillar| pillar.sections().len())
            .unwrap_or(0);

        let pillar_vec = world.pillar_at(pillar_index).map(|pillar| pillar.sections());

        if pillar_vec.is_some() {
            let new_pillar_vec = pillar_vec.unwrap();

            if vec_len == 1 {
                height = new_pillar_vec[0].top.to_real();
                above = f32::INFINITY;
            } else {
                for i in 0..vec_len {
                    if i != vec_len - 1 {
                        if new_pillar_vec[i].top.to_real() < self.position.z &&
                           self.position.z < new_pillar_vec[i + 1].bottom.to_real() {
                            height = new_pillar_vec[i].top.to_real();
                            above = new_pillar_vec[i + 1].bottom.to_real();
                            break;
                        } else {
                            continue;
                        }
                    } else {
                        height = new_pillar_vec[i].top.to_real();
                        above = f32::INFINITY;
                        break;
                    }
                }
            }
        }

        !(self.position.z > height && above == f32::INFINITY)
    }
}


pub struct Weather {
    particles: Vec<Particle>,
    program: Program,
    indices: glium::index::NoIndices,
    context: Rc<GameContext>,
    camera: Camera,
    actual_buf: VertexBuffer<Instance>,
    vertex_buffer: VertexBuffer<Vertex>,
    form: Form,
    strength: Strength,
    wind: Vector2f,
    wind_speed: f32,
    delta_time: f32,
    weather_time: f32,
    last_biome: biome::Biome,
    change: bool,
}

impl Weather {
    pub fn new(context: Rc<GameContext>) -> Weather {
        let vec: Vec<Particle> = Vec::new();
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        let weather_program = context.load_program("weather").unwrap();
        let camera = Camera::new(context.get_config().resolution.aspect_ratio());
        let sections = glium::VertexBuffer::new(context.get_facade(), &[]).unwrap();

        let buffer = vec![Vertex { point: [1.0, 1.0, 0.0] },
                          Vertex { point: [-1.0, 1.0, 0.0] },
                          Vertex { point: [1.0, -1.0, 0.0] },
                          Vertex { point: [-1.0, -1.0, 0.0] }];
        let vertex_buffer = glium::VertexBuffer::new(context.get_facade(), &buffer).unwrap();

        Weather {
            particles: vec,
            program: weather_program,
            indices: index_buffer,
            context: context,
            camera: camera,
            actual_buf: sections,
            vertex_buffer: vertex_buffer,
            form: Form::Rain,
            strength: Strength::None,
            wind: Vector2f::new(rand::random::<f32>(), rand::random::<f32>()),
            wind_speed: (rand::random::<f32>() + 0.2) * 5.0,
            delta_time: 0.0,
            weather_time: 0.0,
            last_biome: biome::Biome::GrassLand,
            change: false,
        }
    }

    /// Draws particles on the screen
    pub fn draw<S: glium::Surface>(&mut self, surface: &mut S, camera: &Camera) {
        if self.strength == Strength::None && self.particles.len() == 0 {
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

        let scaling: Matrix4<f32>;
        let view = camera.view_matrix();
        let color: Vector4f;

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
        };

        let uniforms = uniform!{
                form: self.form as i32,
                view_matrix: view.to_arr(),
                proj_matrix: camera.proj_matrix().to_arr(),
                scaling_matrix: scaling.to_arr(),
                color: color.to_arr(),
            };

        surface.draw((&self.vertex_buffer, self.actual_buf.per_instance().unwrap()),
                  &self.indices,
                  &self.program,
                  &uniforms,
                  &params)
            .unwrap();
    }

    /// updates particles in terms of lifetime and position dependend on their
    /// form and deletes
    /// particles that are in the back of the player or in caves. Also
    /// responsible for setting the
    /// apropriate weather.
    pub fn update(&mut self, camera: &Camera, delta: f32, world_manager: &super::WorldManager) {
        // Toggle downfall
        let world = world_manager.get_world();
        let relevant_pos = Point2f::new(camera.position.x, camera.position.y);
        let pillar_index = PillarIndex(AxialPoint::from_real(relevant_pos));
        let pillar_vec = world.pillar_at(pillar_index);

        if pillar_vec.is_some() && !self.change {
            let biome = pillar_vec.unwrap().biome();
            self.weather_time += delta;

            if *biome != self.last_biome && self.weather_time > 5.0 || self.weather_time >= 120.0 {
                let chance = rand::random::<f32>() * 100.0;
                match *biome {
                    biome::Biome::GrassLand => {
                        if (self.form == Form::Pollen || self.form == Form::Snow) &&
                           self.particles.len() > 0 {
                            self.change = true;
                            return;
                        }
                        self.form = Form::Rain;
                        self.last_biome = biome::Biome::GrassLand;
                        match chance {
                            0.0...5.0 => self.strength = Strength::Weak,
                            5.0...8.0 => self.strength = Strength::Medium,
                            8.0...10.0 => self.strength = Strength::Heavy,
                            _ => self.strength = Strength::None,
                        }
                    }

                    biome::Biome::Desert => {
                        if (self.form == Form::Pollen || self.form == Form::Snow) &&
                           self.particles.len() > 0 {
                            self.change = true;
                            return;
                        }
                        self.form = Form::Rain;
                        self.last_biome = biome::Biome::Desert;
                        match chance {
                            0.0...1.0 => self.strength = Strength::Weak,
                            1.0...2.0 => self.strength = Strength::Medium,
                            2.0...4.0 => self.strength = Strength::Heavy,
                            _ => self.strength = Strength::None,
                        }
                    }

                    biome::Biome::Snow => {
                        if (self.form == Form::Pollen || self.form == Form::Rain) &&
                           self.particles.len() > 0 {
                            self.change = true;
                            return;
                        }
                        self.form = Form::Snow;
                        self.last_biome = biome::Biome::Snow;
                        match chance {
                            0.0...20.0 => self.strength = Strength::Weak,
                            20.0...50.0 => self.strength = Strength::Medium,
                            50.0...75.0 => self.strength = Strength::Heavy,
                            _ => self.strength = Strength::None,
                        }
                    }

                    biome::Biome::Forest => {
                        if (self.form == Form::Rain || self.form == Form::Snow) &&
                           self.particles.len() > 0 {
                            self.change = true;
                            return;
                        }
                        self.form = Form::Pollen;
                        self.last_biome = biome::Biome::Forest;
                        match chance {
                            0.0...15.0 => self.strength = Strength::Weak,
                            15.0...35.0 => self.strength = Strength::Medium,
                            35.0...65.0 => self.strength = Strength::Heavy,
                            _ => self.strength = Strength::None,
                        }
                    }

                    biome::Biome::RainForest => {
                        if (self.form == Form::Pollen || self.form == Form::Snow) &&
                           self.particles.len() > 0 {
                            self.change = true;
                            return;
                        }
                        self.form = Form::Rain;
                        self.last_biome = biome::Biome::RainForest;
                        match chance {
                            0.0...21.0 => self.strength = Strength::Weak,
                            21.0...51.0 => self.strength = Strength::Medium,
                            51.0...81.0 => self.strength = Strength::Heavy,
                            _ => self.strength = Strength::None,
                        }
                    }

                    biome::Biome::Savanna => {
                        if (self.form == Form::Rain || self.form == Form::Snow) &&
                           self.particles.len() > 0 {
                            self.change = true;
                            return;
                        }
                        self.form = Form::Pollen;
                        self.last_biome = biome::Biome::Savanna;
                        match chance {
                            0.0...5.0 => self.strength = Strength::Weak,
                            5.0...7.0 => self.strength = Strength::Medium,
                            7.0...9.0 => self.strength = Strength::Heavy,
                            _ => self.strength = Strength::None,
                        }
                    }

                    biome::Biome::Stone => {
                        if (self.form == Form::Pollen || self.form == Form::Snow) &&
                           self.particles.len() > 0 {
                            self.change = true;
                            return;
                        }
                        self.form = Form::Rain;
                        self.last_biome = biome::Biome::Stone;
                        match chance {
                            0.0...7.0 => self.strength = Strength::Weak,
                            7.0...12.0 => self.strength = Strength::Medium,
                            12.0...17.0 => self.strength = Strength::Heavy,
                            _ => self.strength = Strength::None,

                        }
                    }

                    biome::Biome::Debug => (),
                }
                self.weather_time = 0.0;
            }
        }

        // stops method if no weather is set
        if self.strength == Strength::None && self.particles.len() == 0 {
            self.change = false;
            return;
        }

        // wind settings
        self.delta_time += delta;

        if self.delta_time >= 90.0 {
            self.wind.x += (rand::random::<f32>() * 0.4) - 0.2;
            self.wind.y += (rand::random::<f32>() * 0.4) - 0.2;
            self.wind_speed = (rand::random::<f32>() + 0.2) * 5.0;
            self.delta_time = 0.0;
        }

        // setting amount of particles depending on form
        self.camera = *camera;

        let max_count = match self.form {
            Form::Pollen => 40 * self.strength as usize,
            _ => 750 * self.strength as usize,
        };

        if self.particles.len() < max_count && !self.change {
            let count = max_count / 10;
            for _ in 0..count {
                self.particles.push(Particle::new(camera.position));
            }
        }

        // deletes dead particles
        self.particles.retain(|&p| p.lifetime > 0.0);

        // instancing and position updating for particles
        let mut tmp_instances = Vec::new();

        for particle in &mut self.particles.iter_mut() {
            // if weather changes, kill particles much faster
            if self.change {
                particle.lifetime -= 25.0 * delta;
            } else {
                particle.lifetime -= 1.0 * delta;
            }

            match self.form {

                Form::Snow => {
                    particle.position.z -= particle.velocity * 3.0 * delta;
                    particle.position.x += (particle.trans_x.sin() * 0.05 +
                                            (self.wind.x * self.wind_speed) * delta) *
                                           0.5;
                    particle.position.y += (particle.trans_y.cos() * 0.05 +
                                            (self.wind.y * self.wind_speed) * delta) *
                                           0.5;
                    particle.trans_y += PI * 0.005 * (rand::random::<f32>() - 0.5);
                    particle.trans_x += PI * 0.005 * (rand::random::<f32>() - 0.5);
                }

                Form::Rain => {
                    particle.position.x += ((self.wind.x * self.wind_speed) * delta) * 2.0;
                    particle.position.y += ((self.wind.y * self.wind_speed) * delta) * 2.0;
                    particle.position.z = particle.position.z -
                                          ((particle.velocity + 0.5) * 50.0 * delta)
                }

                Form::Pollen => {
                    particle.position.x +=
                        (particle.trans_x.sin() * delta * (self.wind.x * self.wind_speed)) * 2.0;
                    particle.position.y +=
                        (particle.trans_y.cos() * delta * (self.wind.y * self.wind_speed)) * 2.0;
                    particle.position.z += (particle.trans_z.sin() - 0.5) * 0.5 * delta;

                    particle.trans_y += PI * 0.005 * (rand::random::<f32>() - 0.5);
                    particle.trans_x += PI * 0.005 * (rand::random::<f32>() - 0.5);
                    particle.trans_z += PI * 0.005 * (rand::random::<f32>() - 0.3);
                }

            }

            // moving particles which fell out of box to the other direction of it
            let pix_vec = Point2::new(particle.position.x, particle.position[1]);
            let cam_vec = Point2::new(camera.position.x, camera.position.y);
            let tmp = pix_vec - cam_vec;
            let len = ((tmp.x * tmp.x) + (tmp.y * tmp.y)).sqrt();

            let size = match self.form {
                Form::Pollen => BOX_SIZE / 5.0,
                _ => 1.0,
            };

            if len > BOX_SIZE - size || len < -BOX_SIZE - size {
                particle.position.x = particle.position.x - (1.95 * tmp.x);
                particle.position.y = particle.position.y - (1.95 * tmp.y);
            }

            if particle.position.z < self.camera.position.z - BOX_SIZE / size {
                particle.position.z += 1.9 * (BOX_SIZE / size);
            }

            if particle.position.z > self.camera.position.z + BOX_SIZE / size {
                particle.position.z -= 1.9 * (BOX_SIZE / size);
            }

            // prevents drawing the particle if in cave or behind the player
            if !particle.in_cave(&world_manager) &&
               ((particle.position - camera.position).dot(camera.get_look_at_vector())) > 0.0 {
                tmp_instances.push(Instance {
                    position: [particle.position.x, particle.position.y, particle.position.z],
                })
            }
        }
        // create vertex buffer from left particles
        let vertex_buffer = glium::VertexBuffer::new(self.context.get_facade(), &tmp_instances)
            .unwrap();
        self.actual_buf = vertex_buffer;

        // if all particles are deleted allow new weather
        if self.particles.len() == 0 {
            self.change = false;
        }
    }
}
