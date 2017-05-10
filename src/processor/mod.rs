extern crate csv;

use std::collections::HashSet;

use super::manifest::Manifest;
use super::types::options::Options;

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

            for row in rdr.records() {
                let row = row.unwrap();
                for (i, val) in row.iter().enumerate() {
                    self.sets[i].insert(val.clone());
                }
            }

            let mut i = 0;
            for set in &self.sets {
                println!("{} - {}", headers[i], set.len());
                i += 1;
            }
        }
    }
}
