use glium::Program;
use GameContext;
use std::rc::Rc;

pub struct PlantRenderer {
    program: Program,
    shadow_program: Program,
}

impl PlantRenderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        PlantRenderer {
            program: context.load_program("plants").unwrap(),
            shadow_program: context.load_program("plant_shadow").unwrap(),
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn shadow_program(&self) -> &Program {
        &self.shadow_program
    }
}
