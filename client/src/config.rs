extern crate clap;
extern crate toml;
extern crate regex;

use base::math::*;
use self::clap::{App, Arg, Error as ClapError, ErrorKind as ClapErrorKind};
use self::regex::Regex;
use std::io;
use std::error::Error as StdError;

pub struct Config {
    pub resolution: Dimension2u,
    pub window_mode: WindowMode,
    pub window_title: String,
    pub vsync: bool, /* sichtweite
                      * Vsync
                      * kantenglättung
                      * steuerung
                      * frames
                      * seed
                      * Chunkweite */
}
impl Config {
    pub fn load_config() -> Result<Config, Box<StdError>> {
        let conf = Config::default();
        command_config(conf)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: Dimension2::new(800, 600),
            window_mode: WindowMode::Windowed,
            window_title: format!("Plantex {}", env!("CARGO_PKG_VERSION")),
            vsync: false,
        }
    }
}


fn command_config(mut toml_config: Config) -> Result<Config, Box<StdError>> {
    let matches = App::new("Plantex")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Game about Plants!")
        .arg(Arg::with_name("Resolution")
            .help("(e.g. =1280x720) 'Sets Resolution to new value'")
            .takes_value(true)
            .long("resolution"))
        .arg(Arg::with_name("WindowMode")
            .help("[Windowed, FullScreen] 'Sets WindowMode'")
            .takes_value(true)
            .long("windowmode"))
        .arg(Arg::with_name("Vsync")
            .help("[on/off]")
            .takes_value(true)
            .long("vsync"))
        .get_matches();

    if let Some(res) = matches.value_of("Resolution") {
        let reg_res = Regex::new(r"^([1-9]\d{1,4})x([1-9]\d{1,4})").unwrap();

        if reg_res.is_match(res) {
            for cap in reg_res.captures_iter(res) {
                let res_x = cap.at(1).unwrap().parse::<u32>().unwrap();
                let res_y = cap.at(2).unwrap().parse::<u32>().unwrap();
                toml_config.resolution = Dimension2::new(res_x, res_y);
            }
        } else {
            return Err("invalid resolution in command line argument".into());
        }
    }


    if let Some(mode) = matches.value_of("WindowMode") {
        match mode {
            "Windowed" => toml_config.window_mode = WindowMode::Windowed,
            "FullScreen" => toml_config.window_mode = WindowMode::FullScreen,
            _ => return Err("invalid Window Mode in command line argument".into()),
        }
    }
    Ok(toml_config)
}
pub enum WindowMode {
    Windowed,
    // FullScreenWindow, // TODO: maybe add this
    FullScreen,
}
