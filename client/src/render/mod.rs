use world::WorldView;
use glium::Surface;
use glium::backend::glutin_backend::GlutinFacade;
use Camera;

mod to_arr;

pub use self::to_arr::ToArr;

pub struct Renderer {
    context: GlutinFacade,
}

impl Renderer {
    pub fn new(context: GlutinFacade) -> Self {
        Renderer { context: context }
    }

    /// Is called once every main loop iteration
    pub fn render(&self, world_view: &WorldView, camera: &Camera) -> Result<(), ()> {
        let mut target = self.context.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        world_view.draw(&mut target, camera);

        target.finish().unwrap();

        Ok(())
    }
}
