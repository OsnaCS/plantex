use glium::Program;
use GameContext;
use std::rc::Rc;

pub struct PlantRenderer {
    program: Program,
}

impl PlantRenderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        PlantRenderer {
            program: match context.load_program("plant_dummy") {
                Ok(prog) => prog,
                Err(_) => panic!("failed to compile program"),
            },
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
