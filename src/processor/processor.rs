extern crate csv;

use std::fs;
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

#[derive(Default)]
pub struct Processor {}

impl Processor {
    fn create_transpose_thread(headers: &CsvRow, rx: TransposeMessageReceiver) -> JoinHandle<()> {
        let stats: Vec<_> = headers.iter().map(|_| Vec::new()).collect();

        thread::spawn(move || {
            let mut rows_count = 0;

            while let Ok(data) = rx.recv() {
                // TODO: Wrap message handler in method
                let (mt, maybe_data) = data;
                match mt {
                    // TODO: Wrap this in method ..
                    MessageType::Bulk => {
                        if let Some(raw_data) = maybe_data {
                            let data: Vec<_> = raw_data;

                            let mut stats = stats.clone();

                            for row in &data {
                                for (i, item) in row.iter().enumerate() {
                                    let str: &String = item;
                                    stats[i].push(str.clone());
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
        let mut rows = Vec::new();
        for row in rdr.records() {
            rows.push(row.unwrap());

            if rows.len() == opts.bulk_size {
                tx.send((MessageType::Bulk, Some(rows))).unwrap();
                rows = Vec::new();
            }
        }

        if !rows.is_empty() {
            tx.send((MessageType::Bulk, Some(rows))).unwrap();
        }

        tx.send((MessageType::EndOfStream, None)).unwrap();
    }

    pub fn process(&mut self, path: &str, _manifest: &Manifest, opts: &Options) {
        if let Ok(rdr) = csv::Reader::from_file(&path) {
            // TODO: Wrap reader construction (rdr <- reader)_
            let mut rdr = rdr.delimiter(opts.csv.delimiter)
                .has_headers(opts.csv.has_header)
                .flexible(opts.csv.flexible);

            let headers = Processor::get_header(&mut rdr, opts);

            debug!("Header is {:?}", headers);

            let channel_size = opts.channel.size;

            debug!("Transpose thread channel size is {}", &channel_size);
            debug!("Transpose thread bulk size is {}", &opts.bulk_size);

            let (tx, rx) = mpsc::sync_channel(channel_size);

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
