use clap::ArgMatches;

pub const DEFAULT_BULK_SIZE: usize = 100;
pub const DEFAULT_CHANNEL_SIZE: usize = 100;
pub const DEFAULT_DELIMITER: u8 = b',';

use super::channel::Channel;
use super::csv::OptionsCsv;

#[derive(Debug, Clone)]
pub struct Options {
    pub channel: Channel,
    pub csv: OptionsCsv,
    pub manifest: Option<String>,
    pub cache: bool,
    pub sync_io: bool,
    pub bulk_size: usize
}

impl<'a> From<&'a ArgMatches<'a>> for Options {
    fn from(matches: &ArgMatches) -> Options {
        debug!("Parsing options");
        Options {
            channel: Channel {
                size: matches.value_of("channel-size")
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap_or(DEFAULT_CHANNEL_SIZE),
            },
            csv: OptionsCsv {
                delimiter: match matches.value_of("delimiter") {
                    Some(val) => val.to_string().bytes().nth(0).unwrap_or(DEFAULT_DELIMITER),
                    _ => DEFAULT_DELIMITER
                },
                has_header: matches.is_present("has-header"),
                flexible: matches.is_present("flexible"),
            },
            manifest: match matches.value_of("manifest") {
                Some(val) => Some(val.to_string()),
                _ => None
            },
            cache: matches.is_present("cache"),
            sync_io: matches.is_present("sync-io"),
            bulk_size: matches.value_of("bulk-size")
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap_or(DEFAULT_BULK_SIZE),
        }
    }
}
