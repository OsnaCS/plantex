use world::WorldView;
use glium::Surface;
use super::{Camera, GameContext};
use std::rc::Rc;
use super::weather::Weather;

pub struct Renderer {
    context: Rc<GameContext>,
}

impl Renderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        Renderer { context: context }
    }

    /// Is called once every main loop iteration
    pub fn render(&self,
                  weather: &Weather,
                  world_view: &WorldView,
                  camera: &Camera)
                  -> Result<(), ()> {
        let mut target = self.context.get_facade().draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        world_view.draw(&mut target, &camera);
        weather.draw(&mut target, &camera);

        target.finish().unwrap();

        Ok(())
    }
}
