extern crate music_player_rs;
extern crate simplelog;
extern crate clap;

use simplelog::{Level, LevelFilter, WriteLogger, Config};
use std::fs::File;
use clap::{Arg, App};
use music_player_rs::music_manager::{music_database::MusicDatabase};

fn main() {
    let matches = App::new("music player")
                    .version("0.1")
                    .author("Adrián G. <adrian.garcia04@ciencias.unam.mx>")
                    .about("A music player written in Rust")
                    .arg(Arg::with_name("log output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Log file")
                        .takes_value(true))
                    .arg(Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Verbosity level"))
                    .get_matches();

    let log_file = matches.value_of("output").unwrap_or("music_player.log");

    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Warn,
        3 | _ => LevelFilter::max(),
    };

    let config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Error),
        location: Some(Level::Trace),
        time_format: Some("%r"),
    };

    let archivo_log = File::create(log_file).unwrap();
    WriteLogger::init(log_level, config, archivo_log).unwrap();

    let mut music_database = MusicDatabase::new();
    music_database.connect().unwrap();
    match music_database.mine() {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    }
}
