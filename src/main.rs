#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate csv;
extern crate sli2dli;
extern crate time;

use clap::{App, Arg};
use sli2dli::*;
use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use time::PreciseTime;

use self::manifest::*;
use self::processor::*;
use self::types::*;
use self::types::formatter::human_format;

const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author(AUTHOR)
        .about("Disk Usage Information")
        .arg(Arg::with_name("verbose")
            .help("Verbose mode")
            .short("v")
            .long("verbose")
            .multiple(true))
        .arg(Arg::with_name("cache")
            .help("Cache results")
            .short("c")
            .long("cache"))
        .arg(Arg::with_name("delimiter")
            .help("Delimiter")
            .short("d")
            .long("delimiter")
            .default_value(","))
        .arg(Arg::with_name("has-header")
            .help("CSV has header row")
            .long("has-header"))
        .arg(Arg::with_name("manifest")
            .help("Path to manifest file")
            .takes_value(true)
            .short("m")
            .long("manifest")
            .required(true))
        .arg(Arg::with_name("FILE")
            .help("Files to process")
            .index(1)
            .required(true)
            .multiple(true))
        .get_matches();

    let opts = Options::from(&matches);

    match matches.occurrences_of("verbose") {
        0 => {}
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        _ => env::set_var("RUST_LOG", "debug"),
    }

    env_logger::init().unwrap();

    debug!("Raw options are {:?}", &matches);
    debug!("Parsed options are {:?}", &opts);
    debug!("Escape character is {:?}", String::from_utf8(vec!(opts.delimiter)).unwrap());

    let files: Vec<_> = match matches.values_of("FILE") {
        Some(dirs) => {
            dirs.map(|d| {
                d.to_string()
            })
                .collect()
        }
        _ => vec![String::from(".")],
    };

    let manifest: Manifest = match opts.manifest.as_ref() {
        Some(path) => {
            Manifest::from_file(path)
        }
        _ => Manifest {
            manifest: None,
        }
    };

    debug!("{:?}", manifest);

    for file in &files {
        let metadata = Box::new(fs::metadata(&file).unwrap()) as Box<MetadataExt>;
        let size = metadata.size();

        let start = PreciseTime::now();

        debug!("Processing file {:?}", &file);
        let mut processor = Processor::new();
        processor.process(&file, &manifest, &opts);

        let diff = start.to(PreciseTime::now());
        let elapsed_secs = diff.num_nanoseconds().unwrap() as f64 * 1e-9;

        let human_size = human_format(size as f32);
        let human_speed = human_format(size as f32 / elapsed_secs as f32);
        debug!("Stats - size: {:.2}{}B, time: {:.2}s, speed: {:.2}{}Bps", human_size.0, human_size.1, elapsed_secs, human_speed.0, human_speed.1 );
    }

    debug!("Finished!");
}
