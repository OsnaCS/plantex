use world::WorldView;
use glium::Surface;
use super::{Camera, GameContext};

pub struct Renderer {
    context: GameContext,
}

impl Renderer {
    pub fn new(context: GameContext) -> Self {
        Renderer { context: context }
    }

    /// Is called once every main loop iteration
    pub fn render(&self, world_view: &WorldView, camera: &Camera) -> Result<(), ()> {
        let mut target = self.context.get_facade().draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        world_view.draw(&mut target, camera);

        target.finish().unwrap();

        Ok(())
    }
}
