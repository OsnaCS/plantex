use base::world::ChunkProvider;
use ghost::Ghost;
use event_manager::{CloseHandler, EventManager, EventResponse};
use glium::backend::glutin_backend::GlutinFacade;
use glium::{self, DisplayBuild, glutin};
use super::Renderer;
use super::{Config, GameContext, WorldManager};
use config::WindowMode;
use base::gen::WorldGenerator;
use std::time::{Duration, Instant};
use std::rc::Rc;
use std::net::{SocketAddr, TcpStream};
use std::error::Error;
use view::{SkyView, Sun};
use super::DayTime;
use super::weather::Weather;
use player::Player;
use control_switcher::ControlSwitcher;

pub struct Game {
    renderer: Renderer,
    event_manager: EventManager,
    world_manager: WorldManager,
    #[allow(dead_code)]
    server: TcpStream,
    sun: Sun,
    sky_view: SkyView,
    daytime: DayTime,
    weather: Weather,
    control_switcher: ControlSwitcher,
}

impl Game {
    pub fn new(config: Config, server: SocketAddr) -> Result<Self, Box<Error>> {
        info!("connecting to {}", server);
        let server = try!(TcpStream::connect(server));
        let facade = try!(create_context(&config));
        let context = Rc::new(GameContext::new(facade, config.clone()));
        let world_weather = Weather::new(context.clone());
        let world_manager = WorldManager::new(create_chunk_provider(context.get_config()),
                                              context.clone());

        Ok(Game {
            renderer: Renderer::new(context.clone()),
            event_manager: EventManager::new(context.get_facade().clone()),
            world_manager: world_manager.clone(),
            server: server,
            sun: Sun::new(context.clone()),
            sky_view: SkyView::new(context.clone()),
            daytime: DayTime::default(),
            weather: world_weather,
            control_switcher: ControlSwitcher::new(Player::new(context.clone(), world_manager),
                                                   Ghost::new(context.clone())),
        })
    }

    /// Main game function: contains the main render loop and owns all important
    /// components. This function should remain rather small, all heavy lifting
    /// should be done in other functions.
    pub fn run(mut self) -> Result<(), Box<Error>> {
        let mut frames = 0;
        let mut next_fps_measure = Instant::now() + Duration::from_secs(1);
        let mut time_prev = Instant::now();

        loop {
            self.world_manager.update_world(self.control_switcher.get_camera().position);


            let time_now = Instant::now();
            let duration_delta = time_now.duration_since(time_prev);
            // delta in seconds
            let delta = ((duration_delta.subsec_nanos() / 1_000) as f32) / 1_000_000.0 +
                        duration_delta.as_secs() as f32;

            self.weather.update(&self.control_switcher.get_camera(),
                                delta,
                                &self.world_manager);
            self.world_manager.update_world(self.control_switcher.get_camera().position);

            time_prev = Instant::now();

            self.daytime.update(delta);
            self.sky_view.update(self.daytime.get_sun_position());
            self.sun.update(self.daytime.get_sun_position());

            try!(self.renderer.render(&*self.world_manager.get_view(),
                                      &self.control_switcher.get_camera(),
                                      &self.sun,
                                      &mut self.weather,
                                      &self.sky_view));

            let event_resp = self.event_manager
                .poll_events(vec![&mut CloseHandler,
                                  &mut self.control_switcher,
                                  &mut self.daytime]);
            if event_resp == EventResponse::Quit {
                break;
            }

            self.control_switcher.update(delta);

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


fn create_chunk_provider(config: &Config) -> Box<ChunkProvider> {
    Box::new(WorldGenerator::with_seed(config.seed))
}

/// Creates the OpenGL context and prints useful information about the
/// success or failure of said action.
fn create_context(config: &Config) -> Result<GlutinFacade, Box<Error>> {

    // initialize window builder
    let mut window_builder = glutin::WindowBuilder::new();

    // check for window mode and set params
    match config.window_mode {
        WindowMode::Windowed => (),
        // TODO: if we add a fullscreen window mode
        // FullScreenWindow => (),
        WindowMode::FullScreen => {
            window_builder = window_builder.with_fullscreen(glutin::get_primary_monitor());
            window_builder = window_builder.with_decorations(false);
        }
    }

    // check for vsync
    if config.vsync {
        window_builder = window_builder.with_vsync();
    }

    // set title, resolution & create glium context
    window_builder = window_builder.with_title(config.window_title.clone());
    window_builder =
        window_builder.with_dimensions(config.resolution.width, config.resolution.height);
    let context = window_builder.with_depth_buffer(24).build_glium();

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
