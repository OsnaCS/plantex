use glium::backend::glutin_backend::GlutinFacade;
use glium::program;
use glium::Program;
use super::Config;
use std::fs::File;
use std::io::{self, Read};
use std::error::Error;

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

    /// Loads vertex and fragment shader automatically to prevent recompiling
    /// the application
    /// everytime a shader is changed.
    pub fn load_program(&self, shader: &str) -> Result<Program, Box<Error>> {
        fn load_if_present(path: &str) -> Result<String, io::Error> {
            let mut f = try!(File::open(path));
            let mut buf = String::new();
            try!(f.read_to_string(&mut buf));
            Ok(buf)
        }

        let mut vert = try!(File::open(&format!("client/shader/{}.vert", shader)));
        let mut frag = try!(File::open(&format!("client/shader/{}.frag", shader)));

        let mut vert_buf = String::new();
        let mut frag_buf = String::new();
        try!(vert.read_to_string(&mut vert_buf));
        try!(frag.read_to_string(&mut frag_buf));

        let tcs = load_if_present(&format!("client/shader/{}.tcs", shader)).ok();
        let tes = load_if_present(&format!("client/shader/{}.tes", shader)).ok();

        let source = program::SourceCode {
            vertex_shader: &vert_buf,
            tessellation_control_shader: tcs.as_ref().map(|s| s.as_str()),
            tessellation_evaluation_shader: tes.as_ref().map(|s| s.as_str()),
            geometry_shader: None,
            fragment_shader: &frag_buf,
        };

        let prog = Program::new(&self.facade, source);

        if let Err(ref e) = prog {
            warn!("failed to compile program '{}':\n{}", shader, e);
        }
        Ok(try!(prog))
    }
}
