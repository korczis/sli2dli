use clap::ArgMatches;

#[derive(Debug, Clone)]
pub struct Options {
    pub delimiter: u8,
    pub has_header: bool,
    pub manifest: Option<String>,
}

impl<'a> From<&'a ArgMatches<'a>> for Options {
    fn from(matches: &ArgMatches) -> Options {
        debug!("Parsing options");
        Options {
            delimiter: match matches.value_of("delimiter") {
                Some(val) => val.to_string().bytes().nth(0).unwrap_or(b','),
                _ => b','
            },
            has_header: matches.is_present("has-header"),
            manifest: match matches.value_of("manifest") {
                Some(val) => Some(val.to_string()),
                _ => None
            }
        }
    }
}
