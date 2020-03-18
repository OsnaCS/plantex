use super::Config;
use glium::program;
use glium::Display;
use glium::Program;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

#[derive(Clone)]
pub struct GameContext {
    facade: Display,
    config: Config, // TODO: we might want to wrap it into `Rc` (performance)
}

impl GameContext {
    pub fn new(facade: Display, config: Config) -> Self {
        GameContext {
            facade: facade,
            config: config,
        }
    }

    pub fn get_facade(&self) -> &Display {
        &self.facade
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Loads vertex and fragment shader automatically to prevent recompiling
    /// the application
    /// everytime a shader is changed.
    pub fn load_program(&self, shader: &str) -> Result<Program, Box<dyn Error>> {
        fn load_if_present(path: &str) -> Result<String, io::Error> {
            let mut f = File::open(path)?;
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;
            Ok(buf)
        }

        let mut vert = File::open(&format!("client/shader/{}.vert", shader))?;
        let mut frag = File::open(&format!("client/shader/{}.frag", shader))?;

        let mut vert_buf = String::new();
        let mut frag_buf = String::new();
        vert.read_to_string(&mut vert_buf)?;
        frag.read_to_string(&mut frag_buf)?;

        let (tcs, tes);
        if self.config.tessellation {
            tcs = load_if_present(&format!("client/shader/{}.tcs", shader)).ok();
            tes = load_if_present(&format!("client/shader/{}.tes", shader)).ok();
        } else {
            // Don't even try to load the tessellation shaders if tessellation is off
            tcs = None;
            tes = None;
        }

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
        Ok(prog?)
    }

    // TODO: `load_post_processing_program` which loads a default vertex shader
}
