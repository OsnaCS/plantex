use glium::Program;
use glium::backend::Facade;

pub struct PlantRenderer {
    program: Program,
}

impl PlantRenderer {
    pub fn new<F: Facade>(facade: &F) -> Self {
        let prog = Program::from_source(facade,
                                        include_str!("plant_dummy.vert"),
                                        include_str!("plant_dummy.frag"),
                                        None)
            .unwrap();

        PlantRenderer { program: prog }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
