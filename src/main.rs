use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::{App, Arg};

use walkdir::WalkDir;

#[derive(Debug)]
struct LineEndingStats {
    cr: u32,
    lf: u32,
    crlf: u32,
}

impl LineEndingStats {
    fn is_mixed(&self) -> bool {
        let mut num_types = 0;
        if self.cr > 0 {
            num_types += 1;
        }
        if self.lf > 0 {
            num_types += 1;
        }
        if self.crlf > 0 {
            num_types += 1;
        }
        return num_types > 1;
    }
}

fn main() {
    let matches = App::new("le")
        .version("0.1.0")
        .author("Simon Morgan <sjm@sjm.io>")
        .about("A utility for checking file line-endings")
        .arg(
            Arg::with_name("PATHS")
                .help("The paths to process")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    for path in matches.values_of("PATHS").unwrap() {
        process_path(path);
    }
}

fn process_path(path: &str) {
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let stats = count_line_endings(entry.path());
                    match stats {
                        Ok(stats) => {
                            if stats.is_mixed() {
                                println!(
                                    "{} has mixed line endings: {:?}",
                                    entry.path().display(),
                                    stats
                                );
                            }
                        }
                        Err(err) => eprintln!("{}", err),
                    }
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn count_line_endings(file_path: &Path) -> Result<LineEndingStats, std::io::Error> {
    const LINE_FEED: u8 = 0x0A;
    const CARRIAGE_RETURN: u8 = 0x0D;

    let mut stats = LineEndingStats {
        lf: 0,
        cr: 0,
        crlf: 0,
    };
    let mut cur: u8;
    let mut prev: u8 = 0;

    let f: File = File::open(file_path)?;

    for byte in f.bytes() {
        cur = byte?;

        if cur == LINE_FEED {
            if prev == CARRIAGE_RETURN {
                stats.crlf += 1;
                stats.cr -= 1;
            } else {
                stats.lf += 1;
            }
        } else if cur == CARRIAGE_RETURN {
            stats.cr += 1;
        }

        prev = cur;
    }

    Ok(stats)
}
