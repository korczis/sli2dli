extern crate csv;

use serde_json;
use std::collections::HashSet;
use std::fs::{self, File};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use super::super::manifest::Manifest;
use super::super::options::Options;

enum MessageType {
    Row,
    Bulk,
    EndOfStream
}

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
//            let (tx, rx) = if opts.sync_io {
//                mpsc::sync_channel(100)
//            } else {
//                mpsc::channel()
//            };

            // let (tx, rx) = mpsc::channel();
            let (tx, rx) = mpsc::sync_channel(100);

            let rows_count = Arc::new(Mutex::new(0));
            let rows_count_clone = rows_count.clone();
            let thread_handle = thread::spawn(move || {
                loop {
                    match rx.recv() {
                        Ok(data) => {
                            let (mt, _data) = data;
                            match mt {
                                MessageType::Bulk => {
//                                    if let Some(data) = _data {
//                                        let mut data_rows_count = rows_count_clone.lock().unwrap();
//                                        *data_rows_count += data.len();
//                                    };

                                    let mut data_rows_count = rows_count_clone.lock().unwrap();
                                    *data_rows_count += 1;
                                },
                                MessageType::EndOfStream => break,
                                MessageType::Row => {
                                    let mut data_rows_count = rows_count_clone.lock().unwrap();
                                    *data_rows_count += 1;
                                },
                            }
                        }
                        _ => break
                    };
                }
            });

            let mut rdr = rdr.delimiter(opts.csv.delimiter)
                .has_headers(opts.csv.has_header)
                .flexible(true);

            let headers = if opts.csv.has_header {
                rdr.headers().unwrap()
            } else {
                let nums = 0..rdr.headers().unwrap_or(vec!()).len();
                nums.map(|i| i.to_string()).collect()
            };

            debug!("Header is {:?}", headers);

            for _ in &headers {
                self.sets.push(HashSet::new());
            }


            let mut rows = Vec::new();
            for row in rdr.records() {
                rows.push(row);

                if rows.len() == opts.bulk_size {
                    let _ = tx.send((MessageType::Bulk, Some(rows))).unwrap();
                    rows = Vec::new();
                }
            }

            if rows.len() > 0 {
                let _ = tx.send((MessageType::Bulk, Some(rows))).unwrap();
                rows = Vec::new();
            }

            let _ = tx.send((MessageType::EndOfStream, None)).unwrap();
            let _ = thread_handle.join();

            debug!("Number of rows - {:?}", *rows_count.lock().unwrap());

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
