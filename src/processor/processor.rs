extern crate csv;

use serde_json;
use std::collections::HashSet;
use std::fs::{self, File};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use super::super::manifest::Manifest;
use super::super::types::options::Options;

pub struct Processor {
    pub sets: Vec<HashSet<String>>,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            sets: Vec::new()
        }
    }

    pub fn process(&mut self, path: &String, _manifest: &Manifest, opts: &Options) {
        self.sets = Vec::new();

        if let Ok(rdr) = csv::Reader::from_file(&path) {
            let (tx, rx) = mpsc::sync_channel(100);

            thread::spawn(move || {
                loop {
                    let _data = rx.recv();
                }
            });

            let mut rdr = rdr.delimiter(opts.delimiter)
                .has_headers(opts.has_header)
                .flexible(true);

            let headers = if opts.has_header {
                rdr.headers().unwrap()
            } else {
                let nums = 0..rdr.headers().unwrap_or(vec!()).len();
                nums.map(|i| i.to_string()).collect()
            };

            debug!("Header is {:?}", headers);

            for _ in &headers {
                self.sets.push(HashSet::new());
            }

            let mut rows_count = 0;
            let mut rows = Vec::new();
            for row in rdr.records() {
                rows.push(row);

                rows_count += 1;

                if rows.len() == 50 {
                    let _ = tx.send(rows).unwrap();
                    rows = Vec::new();
                }
            }

            debug!("Number of rows - {}", &rows_count);

            if rows.len() > 0 {
                let _ = tx.send(rows).unwrap();
            }

            let p = match opts.cache {
                true => {
                    let p = format!("{}.s2d", &path);
                    debug!("Creating directory {:?}", &p);
                    let _ = fs::create_dir_all(&p);
                    Some(p)
                },
                false => {
                    None
                }
            };

            let mut i = 0;
            for set in &self.sets {
                println!("{} - {}", headers[i], set.len());

                if let &Some(ref p) = &p {
                    let filename = format!("{}.json", &headers[i]);
                    let file = File::create(Path::new(&p).join(filename)).unwrap();
                    let _ = serde_json::to_writer_pretty(&file, &set).unwrap();
                }

                i += 1;
            }
        }
    }
}
