use glium::Program;
use glium::backend::Facade;
use GameContext;

pub struct PlantRenderer {
    program: Program,
}

impl PlantRenderer {
    pub fn new<F: Facade>(facade: &F) -> Self {
        PlantRenderer { program: GameContext::shader_func("plant_dummy", facade) }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
