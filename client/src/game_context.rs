use glium::backend::glutin_backend::GlutinFacade;
use glium::backend::Facade;
use glium::Program;
use super::Config;
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
pub struct GameContext {
    facade: GlutinFacade,
    config: Config, // TODO: we might want to wrap it into `Rc` (performance)
}

impl GameContext {
    pub fn new(facade: GlutinFacade, config: Config) -> Self {
        GameContext {
            facade: facade,
            config: config,
        }
    }

    pub fn get_facade(&self) -> &GlutinFacade {
        &self.facade
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Function to load shader files more easily
    pub fn shader_func<F: Facade>(shader: &str, facade: &F) -> Program {
        if (Path::new(format!("shader/{}.vert", shader).as_str())).exists() &&
           (Path::new(format!("shader/{}.frag", shader).as_str())).exists() {

            let mut vert = match File::open((format!("shader/{}.vert", shader)).as_str()) {
                Err(_) => panic!("Could not open {}.vert!", shader),
                Ok(file) => file,
            };
            let mut frag = match File::open((format!("shader/{}.frag", shader)).as_str()) {
                Err(_) => panic!("Could not open {}.frag!", shader),
                Ok(file) => file,
            };

            let mut vert_buf = String::new();
            let mut frag_buf = String::new();

            match vert.read_to_string(&mut vert_buf) {
                Err(_) => panic!("Could not read {}.vert!", shader),
                Ok(_) => (),
            }
            match frag.read_to_string(&mut frag_buf) {
                Err(_) => panic!("Could not read the {}.frag!", shader),
                Ok(_) => (),
            }

            Program::from_source(facade, vert_buf.as_str(), frag_buf.as_str(), None).unwrap()
        } else {
            panic!("No such shader file!")
        }

    }
}
