use glium::backend::glutin_backend::GlutinFacade;
use super::Config;

#[derive(Clone)]
pub struct GameContext {
    facade: GlutinFacade,
    config: Config,
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
}
