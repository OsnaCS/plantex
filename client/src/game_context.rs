use glium::backend::glutin_backend::GlutinFacade;
use glium::Program;
use glium::program;
use super::Config;
use std::fs::File;
use std::io::Read;
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

        let mut vert = try!(File::open(&format!("client/shader/{}.vert", shader)));
        let mut frag = try!(File::open(&format!("client/shader/{}.frag", shader)));
        let geo_file = File::open(&format!("client/shader/{}.geo", shader));

        let mut vert_buf = String::new();
        let mut frag_buf = String::new();
        try!(vert.read_to_string(&mut vert_buf));
        try!(frag.read_to_string(&mut frag_buf));

        let prog;
        match geo_file {
            Ok(mut n) => {
                let mut geo_buf = String::new();
                let mut tcs_file = try!(File::open(&format!("client/shader/{}.tcs", shader)));
                let mut tes_file = try!(File::open(&format!("client/shader/{}.tes", shader)));
                let mut tcs_buf = String::new();
                let mut tes_buf = String::new();
                try!(tcs_file.read_to_string(&mut tcs_buf));
                try!(tes_file.read_to_string(&mut tes_buf));

                try!(n.read_to_string(&mut geo_buf));
                // prog = Program::from_source(&self.facade, &vert_buf, &frag_buf,
                // Some(&geo_buf));
                let source = program::SourceCode {
                    vertex_shader: &vert_buf,
                    tessellation_control_shader: Some(&tcs_buf),
                    tessellation_evaluation_shader: Some(&tes_buf),
                    geometry_shader: None,
                    fragment_shader: &frag_buf,
                };

                // prog = Program::from_source(&self.facade, &vert_buf, &frag_buf, None);
                prog = Program::new(&self.facade, source);
            }
            Err(_) => prog = Program::from_source(&self.facade, &vert_buf, &frag_buf, None),
        };
        if let Err(ref e) = prog {
            warn!("failed to compile program '{}':\n{}", shader, e);
        }
        Ok(try!(prog))
    }
}
