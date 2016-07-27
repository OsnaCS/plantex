use glium::Program;
use GameContext;
use std::rc::Rc;

pub struct PlantRenderer {
    program: Program,
}

impl PlantRenderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        PlantRenderer { program: context.load_program("plant_dummy").unwrap() }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
