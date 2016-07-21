extern crate clap;
extern crate toml;
extern crate regex;

use base::math::*;
use self::clap::{App, Arg};
use self::regex::Regex;

pub struct Config {
    pub resolution: Dimension2u,
    pub window_mode: WindowMode,
    pub window_title: String,
    //sichtweite
    //Vsync
    //kantenglättung
    //steuerung
    //frames
    //seed
    //Chunkweite
}
impl Config{
    pub fn load_config() -> Config{
        let mut conf = Config::default();
        command_config(conf)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: Dimension2::new(1280, 720),
            window_mode: WindowMode::Windowed,
            window_title: format!("Plantex {}", env!("CARGO_PKG_VERSION")),
        }
    }
}


fn command_config(mut toml_config: Config) -> Config{
    let matches = App::new("Plantex")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Game about Plants!")

        .arg(Arg::with_name("Resolution")
            .help("(e.g. =1280x720) 'Sets Resolution to new value'")
            .takes_value(true)
            .long("resolution")
        )
        .get_matches();

    if let Some(ref res) = matches.value_of("Resolution"){
        let reg_res = Regex::new(r"^[1-9]\d{1,4}x[1-9]\d{1,4}").unwrap();
        if reg_res.is_match(res){
            let mut split_res = res.split("x");
            let vec_res = split_res.collect::<Vec<&str>>();
            let res_x = vec_res[0].parse::<u32>().unwrap();
            let res_y = vec_res[1].parse::<u32>().unwrap();
            toml_config.resolution = Dimension2::new(res_x, res_y);
        }else{}
    }

    toml_config
}
pub enum WindowMode {
    Windowed,
    // FullScreenWindow, // TODO: maybe add this
    FullScreen,
}
