extern crate csv;

use std::fs;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use super::super::manifest::Manifest;
use super::super::options::Options;

enum MessageType {
    Bulk,
    EndOfStream
}

type CsvRow = Vec<String>;
type CsvRows = Vec<CsvRow>;

type TransposeMessage = (MessageType, Option<CsvRows>);
type TransposeMessageSender = mpsc::SyncSender<TransposeMessage>;
type TransposeMessageReceiver = mpsc::Receiver<TransposeMessage>;

pub struct Processor {}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }

    fn create_transpose_thread(headers: &CsvRow, rx: TransposeMessageReceiver) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut stats: Vec<Vec<String>> = Vec::new();
            let _ = stats.reserve(headers.len());
            for _ in headers {
                stats.push(Vec::new());
            }

            let mut rows_count = 0;

            loop {
                // TODO: Extract loop handler in method
                match rx.recv() {
                    Ok(data) => {
                        // TODO: Wrap message handler in method
                        let (mt, maybe_data) = data;
                        match mt {
                            // TODO: Wrap this in method ..
                            MessageType::Bulk => {
                                if let Some(raw_data) = maybe_data {
                                    let data: Vec<_> = raw_data;

                                    let mut stats = stats.clone();

                                    for row in &data {
                                        let mut i: usize = 0;
                                        for item in row {
                                            let str: &String = item;
                                            stats[i].push(str.clone());
                                            i += 1;
                                        }
                                    }

                                    rows_count += data.len();
                                };
                            }
                            // TODO: Wrap this in method ..
                            MessageType::EndOfStream => {
                                debug!("Number of rows - {:?}", rows_count);
                                break
                            }
                        }
                    }
                    _ => break
                };
            }
        })
    }

    fn get_header(rdr: &mut csv::Reader<fs::File>, opts: &Options) -> CsvRow {
        if opts.csv.has_header {
            rdr.headers().unwrap()
        } else {
            let nums = 0..rdr.headers().unwrap_or(vec!()).len();
            nums.map(|i| i.to_string()).collect()
        }
    }

    fn process_rows(rdr: &mut csv::Reader<fs::File>, tx: TransposeMessageSender, opts: &Options) {
        // TODO: Wrap CSV Parsing in method - begin
        let mut rows = Vec::new();
        for row in rdr.records() {
            rows.push(row.unwrap());

            if rows.len() == opts.bulk_size {
                let _ = tx.send((MessageType::Bulk, Some(rows))).unwrap();
                rows = Vec::new();
            }
        }

        if rows.len() > 0 {
            let _ = tx.send((MessageType::Bulk, Some(rows))).unwrap();
        }

        let _ = tx.send((MessageType::EndOfStream, None)).unwrap();
    }

    pub fn process(&mut self, path: &String, _manifest: &Manifest, opts: &Options) {
        if let Ok(rdr) = csv::Reader::from_file(&path) {
            // TODO: Wrap reader construction (rdr <- reader)_
            let mut rdr = rdr.delimiter(opts.csv.delimiter)
                .has_headers(opts.csv.has_header)
                .flexible(opts.csv.flexible);

            let headers = Processor::get_header(&mut rdr, opts);

            debug!("Header is {:?}", headers);

            // TODO: Get sync_channel size from CLI opts
            let (tx, rx) = mpsc::sync_channel(100);

            // TODO: Method for creating worker handle
            let thread_handle = Processor::create_transpose_thread(&headers, rx);

            Processor::process_rows(&mut rdr, tx, opts);

            let _ = thread_handle.join();

            //            let p = match opts.cache {
            //                true => {
            //                    let p = format!("{}.s2d", &path);
            //                    debug!("Creating directory {:?}", &p);
            //                    let _ = fs::create_dir_all(&p);
            //                    Some(p)
            //                },
            //                false => {
            //                    None
            //                }
            //            };

            //            let mut i = 0;
            //            for set in &self.sets {
            //                println!("{} - {}", headers[i], set.len());
            //
            //                if let &Some(ref p) = &p {
            //                    let filename = format!("{}.json", &headers[i]);
            //
        }
    }
}
