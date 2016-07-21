use base::world::ChunkProvider as WorldProvider;
use base::world::World;
use event_manager::{EventManager, EventResponse};
use glium::backend::glutin_backend::GlutinFacade;
use glium::{self, DisplayBuild, glutin};
use render::Renderer;
use super::Config;
use world::WorldView;
use Camera;

/// Main game function: contains the mai render loop and owns all important
/// components. This function should remain rather small, all heavy lifting
/// should be done in other functions.
pub fn run(config: &Config, _: &WorldProvider) -> Result<(), ()> {
    // Create OpenGL context, the renderer and the event manager
    let context = try!(create_context(config));
    let renderer = Renderer::new(context.clone());
    let event_manager = EventManager::new(context.clone());
    let world = World::dummy();
    let world_view = WorldView::from_world(&world, &context);
    let camera = Camera {};

    loop {
        try!(renderer.render(&world_view, &camera));

        let event_resp = event_manager.poll_events();
        if event_resp == EventResponse::Quit {
            break;
        }
    }

    Ok(())
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

            if let Some(mem) = context.get_free_video_memory() {
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
