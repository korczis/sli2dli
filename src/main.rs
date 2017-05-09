#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate csv;
extern crate serde_json;
extern crate sli2dli;

use clap::{App, Arg};
use sli2dli::*;
use std::env;
use std::fs::*;
use std::io::*;

use self::types::*;

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
            .required(false)
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

    let manifest = match opts.manifest.as_ref() {
        Some(path) => {
            let br = BufReader::new(File::open(path).unwrap());
            let manifest: Manifest = serde_json::from_reader(br).unwrap();
            manifest
        },
        _ => Manifest {
            manifest: None,
        }
    };

    debug!("{:?}", manifest);

    for file in files {
        debug!("Processing file {:?}", &file);
        let mut processor = Processor::new();
        processor.process(&file, &manifest, &opts);
    }

    debug!("Finished!");
}
