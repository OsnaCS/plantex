use base::world::{ChunkIndex, ChunkProvider, World};
use ghost::Ghost;
use event_manager::{CloseHandler, EventManager, EventResponse};
use glium::backend::glutin_backend::GlutinFacade;
use glium::{self, DisplayBuild, glutin};
use render::Renderer;
use super::Config;
use world::WorldView;
use base::math::*;
use base::gen::WorldGenerator;
use std::time::{Instant, Duration};

pub struct Game {
    #[allow(dead_code)]
    provider: Box<ChunkProvider>, // Needed when we dynamically load chunks
    renderer: Renderer,
    event_manager: EventManager,
    world_view: WorldView,
    #[allow(dead_code)]
    world: World, // Needed for physics and updated as chunk updates arrive
    player: Ghost,
}

impl Game {
    pub fn new(config: Config) -> Result<Self, ()> {
        let context = try!(create_context(&config));
        let mut world = World::empty();
        let provider = create_chunk_provider(&config);
        pregenerate_world(&mut world, &*provider);

        Ok(Game {
            provider: provider,
            renderer: Renderer::new(context.clone()),
            event_manager: EventManager::new(context.clone()),
            world_view: WorldView::from_world(&world, &context),
            world: world,
            player: Ghost::new(context.clone()),
        })
    }

    /// Main game function: contains the main render loop and owns all important
    /// components. This function should remain rather small, all heavy lifting
    /// should be done in other functions.
    pub fn run(mut self) -> Result<(), ()> {
        let mut frames = 0;
        let mut next_fps_measure = Instant::now() + Duration::from_secs(1);
        loop {
            try!(self.renderer.render(&self.world_view, &self.player.get_camera()));
            let event_resp = self.event_manager
                .poll_events(vec![&mut CloseHandler, &mut self.player]);
            if event_resp == EventResponse::Quit {
                break;
            }
            self.player.update();

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

fn pregenerate_world(world: &mut World, provider: &ChunkProvider) {
    // FIXME temporary worldgen invoker, replace with dynamic gen
    for i in 0..5 {
        for j in 0..5 {
            world.add_chunk(ChunkIndex(AxialPoint::new(i, j)), provider).unwrap();
        }
    }
}

/// Creates the OpenGL context and prints useful information about the
/// success or failure of said action.
fn create_context(config: &Config) -> Result<GlutinFacade, ()> {
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

            Err(())
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
