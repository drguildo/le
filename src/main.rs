use std::path::Path;

enum LineEndingType {
    CR,
    LF,
    CRLF,
    MIXED,
}

#[derive(Debug)]
struct LineEndingStats {
    cr: u32,
    lf: u32,
    crlf: u32,
}

impl LineEndingStats {
    fn is_cr(&self) -> bool {
        self.cr > 0 && self.lf == 0 && self.crlf == 0
    }

    fn is_lf(&self) -> bool {
        self.cr == 0 && self.lf > 0 && self.crlf == 0
    }

    fn is_crlf(&self) -> bool {
        self.cr == 0 && self.lf == 0 && self.crlf > 0
    }

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
    use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("TYPE")
                .help("The type of line endings to search for")
                .long("type")
                .takes_value(true)
                .possible_values(["crlf", "lf", "mixed"].as_ref())
                .default_value("mixed"),
        )
        .arg(
            Arg::with_name("PATHS")
                .help("The paths to process")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let match_on: LineEndingType = match matches.value_of("TYPE").unwrap() {
        "cr" => LineEndingType::CR,
        "lf" => LineEndingType::LF,
        "crlf" => LineEndingType::CRLF,
        _ => LineEndingType::MIXED,
    };
    for path in matches.values_of("PATHS").unwrap() {
        process_path(path, &match_on);
    }
}

fn process_path(path: &str, match_on: &LineEndingType) {
    for entry in walkdir::WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let stats = count_line_endings(entry.path());
                    match stats {
                        Ok(stats) => {
                            let path_display = entry.path().display();
                            match match_on {
                                LineEndingType::CR => {
                                    if stats.is_cr() {
                                        println!("{} has cr line endings", path_display)
                                    }
                                }
                                LineEndingType::LF => {
                                    if stats.is_lf() {
                                        println!("{} has lf line endings", path_display)
                                    }
                                }
                                LineEndingType::CRLF => {
                                    if stats.is_crlf() {
                                        println!("{} has crlf line endings", path_display)
                                    }
                                }
                                LineEndingType::MIXED => {
                                    if stats.is_mixed() {
                                        println!("{} has mixed line endings", path_display)
                                    }
                                }
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
    use std::io::Read;

    const LINE_FEED: u8 = 0x0A;
    const CARRIAGE_RETURN: u8 = 0x0D;

    let mut stats = LineEndingStats {
        lf: 0,
        cr: 0,
        crlf: 0,
    };
    let mut cur: u8;
    let mut prev: u8 = 0;

    let f: std::fs::File = std::fs::File::open(file_path)?;

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
