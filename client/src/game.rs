use base::world::ChunkProvider;
use ghost::Ghost;
use event_manager::{CloseHandler, EventManager, EventResponse};
use glium::backend::glutin_backend::GlutinFacade;
use glium::{self, DisplayBuild, glutin};
use super::Renderer;
use super::{Config, GameContext, WorldManager};
use base::gen::WorldGenerator;
use std::time::{Duration, Instant};
use std::rc::Rc;
use std::net::{SocketAddr, TcpStream};
use std::error::Error;
use view::SkyView;
use base::world::World;
use camera::Camera;
use base::world::PillarSection;
use base::world;
use base::math::*;
use base::world::PillarIndex;
use base::world::HexPillar;
use std::f32::consts;

pub struct Point {
    x: f32,
    y: f32,
}

pub struct Hexagon {
    points: Vec<Point>,
    x: f32,
    y: f32,
    max: f32,
}

impl Hexagon {
    pub fn new(pos_x: f32, pos_y: f32) -> Hexagon {
        let max;
        if pos_x.abs() > pos_y.abs() {
            max = pos_x.abs();
        } else {
            max = pos_y.abs();
        }
        let mut points = Vec::new();
        for i in 0..6 {
            let (x, y) = Hexagon::hex_corner(world::HEX_OUTER_RADIUS, i);
            points.push(Point {
                x: x + pos_x,
                y: y + pos_y,
            });
        }
        Hexagon {
            points: points,
            x: pos_x,
            y: pos_y,
            max: max,
        }
    }

    fn vec_in_hex(&self, direction: Vector3f) -> bool {
        let mut side = 0.0f32;
        for point in &self.points {
            let mut tmp_side = Hexagon::get_side(direction, point);
            if tmp_side > 0.0 {
                tmp_side = 1.0;
            }
            if tmp_side < 0.0 {
                tmp_side = -1.0;
            }
            if side != 0.0 && tmp_side != 0.0 && side != tmp_side {
                return true;
            }
            side = tmp_side;
        }
        false
    }
    fn get_side(p2: Vector3f, point: &Point) -> f32 {
        p2.x * point.y - p2.y * point.x
    }
    fn hex_corner(size: f32, i: i32) -> (f32, f32) {
        let angle_deg = 60.0 * (i as f32) + 30.0;
        let angle_rad = (consts::PI / 180.0) * angle_deg;

        (size * angle_rad.cos(), size * angle_rad.sin())
    }
}

pub struct HexGrid2D {
    hexagons: Vec<Hexagon>,
}

impl HexGrid2D {
    pub fn new(radius: i32) -> (HexGrid2D) {
        let mut hexagons = Vec::new();
        for i in -radius..radius {
            for j in radius..radius * 2 {
                hexagons.push(Hexagon::new(i as f32, j as f32 / 2.0));
                hexagons.push(Hexagon::new(i as f32, -j as f32 / 2.0));
            }
        }
        HexGrid2D { hexagons: hexagons }
    }
    pub fn get_hex_with_intersect(&self, vec: Vector3f) -> (Vec<&Hexagon>) {
        let mut crossing = Vec::new();
        for hexagon in &self.hexagons {
            if hexagon.vec_in_hex(vec) {
                crossing.push(hexagon);
            }
        }
        crossing.sort_by_key(|hex| (hex.x.abs() + hex.y.abs()) as u32);
        crossing
    }
}

pub struct Game {
    renderer: Renderer,
    event_manager: EventManager,
    world_manager: WorldManager,
    player: Ghost,
    #[allow(dead_code)]
    server: TcpStream,
    sky_view: SkyView,
}

impl Game {
    pub fn new(config: Config, server: SocketAddr) -> Result<Self, Box<Error>> {
        info!("connecting to {}", server);
        let server = try!(TcpStream::connect(server));
        let facade = try!(create_context(&config));
        let context = Rc::new(GameContext::new(facade, config.clone()));

        Ok(Game {
            renderer: Renderer::new(context.clone()),
            event_manager: EventManager::new(context.get_facade().clone()),
            world_manager: WorldManager::new(create_chunk_provider(context.get_config()),
                                             context.clone()),
            player: Ghost::new(context.clone()),
            server: server,
            sky_view: SkyView::new(context.clone()),
        })
    }

    /// Main game function: contains the main render loop and owns all important
    /// components. This function should remain rather small, all heavy lifting
    /// should be done in other functions.
    pub fn run(mut self) -> Result<(), Box<Error>> {
        let mut frames = 0;
        let mut next_fps_measure = Instant::now() + Duration::from_secs(1);
        let mut time_prev = Instant::now();
        let hexgrid2d = HexGrid2D::new(4);
        loop {
            self.world_manager.update_world(self.player.get_camera().position);

            let time_now = Instant::now();
            let duration_delta = time_now.duration_since(time_prev);
            let delta = ((duration_delta.subsec_nanos() / 1_000) as f32) / 1_000_000.0 +
                        duration_delta.as_secs() as f32;

            time_prev = Instant::now();

            // println!("{:?}",
            // get_pillarsection_looking_at(&self.world_manager.get_world(),
            //                           self.player.get_camera()));
            println!("{:?}",
                     get_pillarsectionpos_looking_at(&self.world_manager.get_world(),
                                                     self.player.get_camera(),
                                                     &hexgrid2d));

            try!(self.renderer.render(&*self.world_manager.get_view(),
                                      &self.player.get_camera(),
                                      &self.sky_view));
            let event_resp = self.event_manager
                .poll_events(vec![&mut CloseHandler, &mut self.player]);
            if event_resp == EventResponse::Quit {
                break;
            }
            self.player.update(delta);

            frames += 1;
            if next_fps_measure < Instant::now() {
                info!("{} FPS", frames);
                next_fps_measure = Instant::now() + Duration::from_secs(1);
                frames = 0;
            }
        }

        Ok(())
    }
}

fn get_pillarsectionpos_looking_at(world: &World,
                                   cam: Camera,
                                   hexgrid: &HexGrid2D)
                                   -> Option<Vector3f> {
    let look_vec = cam.get_look_at_vector().normalize();
    let hexagons = hexgrid.get_hex_with_intersect(look_vec);
    for hexagon in hexagons {
        let real_pos = Point2f::new(cam.position.x +
                                    (look_vec[0] * world::HEX_INNER_RADIUS * hexagon.x as f32),
                                    cam.position.y +
                                    (look_vec[1] * world::HEX_INNER_RADIUS * hexagon.y as f32));
        let mut pillar_index = PillarIndex(AxialPoint::from_real(real_pos));
        if pillar_index.0.q < 0 {
            pillar_index.0.q *= -1;
        }
        if pillar_index.0.r < 0 {
            pillar_index.0.r *= -1;
        }
        let pillar_at_position = world.pillar_at(pillar_index);
        match pillar_at_position {
            Some(n) => {
                match get_pillar_section_at_position(n,
                                                     cam.position.z +
                                                     (look_vec[2] * world::PILLAR_STEP_HEIGHT *
                                                      hexagon.max as f32)) {
                    Some(_) => {
                        println!("{}-{}-{}",
                                 pillar_index.0.q,
                                 pillar_index.0.r,
                                 cam.position.z +
                                 (look_vec[2] * world::PILLAR_STEP_HEIGHT * hexagon.max as f32));
                        return Some(Vector3f {
                            x: cam.position.x +
                               (look_vec[0] * world::HEX_INNER_RADIUS * hexagon.x as f32),
                            y: cam.position.y +
                               (look_vec[1] * world::HEX_INNER_RADIUS * hexagon.y as f32),
                            z: cam.position.z +
                               (look_vec[2] * world::PILLAR_STEP_HEIGHT * hexagon.max as f32),
                        });
                    }
                    None => {}
                }
            }
            _ => return None,
        }
    }
    None
}

fn get_pillar_section_at_position(pillar: &HexPillar, pos_z: f32) -> Option<&PillarSection> {
    for section in pillar.sections() {
        if (section.top.0 as f32) > pos_z && (section.bottom.0 as f32) < pos_z {
            println!("{:?} - {:?} : {:?} => {:?}",
                     section.top.0,
                     section.bottom.0,
                     pos_z,
                     section);
            return Some(section);
        }
    }
    None
}

fn create_chunk_provider(config: &Config) -> Box<ChunkProvider> {
    Box::new(WorldGenerator::with_seed(config.seed))
}

/// Creates the OpenGL context and prints useful information about the
/// success or failure of said action.
fn create_context(config: &Config) -> Result<GlutinFacade, Box<Error>> {
    // Create glium context
    // TODO: handle fullscreen
    // TODO: OpenGL version/profile
    // TODO: vsync
    let context = glutin::WindowBuilder::new()
        .with_dimensions(config.resolution.width, config.resolution.height)
        .with_title(config.window_title.clone())
        .with_depth_buffer(24)
        .build_glium();

    match context {
        Err(e) => {
            // TODO: print more user friendly output
            error!("OpenGL context creation failed! Detailed error:");
            error!("{}", e);

            Err(e.into())
        }
        Ok(context) => {
            // Print some information about the acquired OpenGL context
            info!("OpenGL context was successfully built");

            let glium::Version(api, major, minor) = *context.get_opengl_version();
            info!("Version of context: {} {}.{}",
                  if api == glium::Api::Gl { "OpenGL" } else { "OpenGL ES" },
                  major,
                  minor);

            let glium::Version(api, major, minor) = context.get_supported_glsl_version();
            info!("Supported GLSL version: {} {}.{}",
                  if api == glium::Api::Gl { "GLSL" } else { "GLSL ES" },
                  major,
                  minor);

            if let Some(mem) = context.get_free_video_memory().map(|mem| mem as u64) {
                let (mem, unit) = match () {
                    _ if mem < (1 << 12) => (mem, "B"),
                    _ if mem < (1 << 22) => (mem >> 10, "KB"),
                    _ if mem < (1 << 32) => (mem >> 20, "MB"),
                    _ => (mem >> 30, "GB"),
                };
                info!("Free GPU memory (estimated): {}{}", mem, unit);
            }

            Ok(context)
        }
    }
}
