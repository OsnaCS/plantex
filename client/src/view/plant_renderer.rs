use glium::Program;
use std::rc::Rc;
use GameContext;

pub struct PlantRenderer {
    program: Program,
    shadow_program: Program,
    context: Rc<GameContext>,
}

impl PlantRenderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        let program = if context.get_config().tessellation {
            context.load_program("plants")
        } else {
            context.load_program("plants_notess")
        };

        PlantRenderer {
            program: program.unwrap(),
            shadow_program: context.load_program("plant_shadow").unwrap(),
            context: context,
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn shadow_program(&self) -> &Program {
        &self.shadow_program
    }

    pub fn context(&self) -> &Rc<GameContext> {
        &self.context
    }
}
