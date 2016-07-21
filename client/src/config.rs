extern crate clap;
extern crate toml;
extern crate regex;

use base::math::*;
use self::clap::{App, Arg, ArgMatches};
use self::regex::Regex;
use std::error::Error as StdError;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use self::toml::Parser;

pub struct Config {
    pub resolution: Dimension2u,
    pub window_mode: WindowMode,
    pub window_title: String,
    pub vsync: bool,
    pub seed: u64, /* sichtweite
                    * Vsync
                    * kantenglättung
                    * steuerung
                    * frames
                    * seed
                    * Chunkweite */
}
impl Config {
    /// creates a new Config in three steps:
    /// 1. loads default config
    /// 2. Overrides from toml config file
    /// 3. Overrides from command line
    pub fn load_config() -> Result<Config, Box<StdError>> {
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
            .arg(Arg::with_name("Seed")
                .help("'Takes a specified seed to generate map'")
                .takes_value(true)
                .long("seed"))
            .arg(Arg::with_name("File")
                .help("Takes config file")
                .takes_value(true)
                .long("config-file"))
            .get_matches();

        let conf = Config::default();

        let t_conf = match config_toml(conf, &matches) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        let conf_final = match config_command(t_conf, &matches) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        Ok(conf_final)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: Dimension2::new(800, 600),
            window_mode: WindowMode::Windowed,
            window_title: format!("Plantex {}", env!("CARGO_PKG_VERSION")),
            vsync: false,
            seed: 42,
        }
    }
}

fn config_toml(mut default_config: Config, matches: &ArgMatches) -> Result<Config, Box<StdError>> {
    let mut name = "config.toml";
    if let Some(file) = matches.value_of("File") {
        let file_reg = Regex::new(".*\\.toml").unwrap();

        if file_reg.is_match(file) && Path::new(file).exists() {
            name = file;
        } else {
            return Err("invalid File in command line".into());
        }
    }
    if Path::new(name).exists() {
        let mut f = try!(File::open(name));
        let mut s = String::new();

        try!(f.read_to_string(&mut s));
        let toml = s;

        let value = match toml::Parser::new(&toml).parse() {
            Some(n) => n,
            None => return Err("config file is corrupted".into()),
        };
        println!("{:?}", value);
    }


    Ok(default_config)
}


fn config_command(mut toml_config: Config, matches: &ArgMatches) -> Result<Config, Box<StdError>> {


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

    if let Some(sync) = matches.value_of("Vsync") {
        match sync {
            "on" => toml_config.vsync = true,
            "off" => toml_config.vsync = false,
            _ => return Err("Vsync can only be set on or off on command line".into()),
        }
    }

    if let Some(seed) = matches.value_of("Seed") {
        match seed.parse::<u64>() {
            Ok(n) => toml_config.seed = n,
            _ => return Err("Seed from command line is invalid".into()),
        }
    }
    Ok(toml_config)
}
pub enum WindowMode {
    Windowed,
    // FullScreenWindow, // TODO: maybe add this
    FullScreen,
}
