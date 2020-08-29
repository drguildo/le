enum LineEndingType {
    LF,
    CRLF,
    MIXED,
}

struct LineEndingStats {
    lf: u32,
    crlf: u32,
}

impl LineEndingStats {
    fn is_lf(&self) -> bool {
        self.lf > 0 && self.crlf == 0
    }

    fn is_crlf(&self) -> bool {
        self.lf == 0 && self.crlf > 0
    }

    fn is_mixed(&self) -> bool {
        self.lf > 0 && self.crlf > 0
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
                    let path_display = entry.path().display();
                    match std::fs::read(entry.path()) {
                        Ok(file_bytes) => {
                            let stats = count_line_endings(file_bytes);
                            match match_on {
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
                        Err(err) => {
                            eprint!("failed to read {}: {}", path_display, err);
                        }
                    }
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn count_line_endings(bytes: Vec<u8>) -> LineEndingStats {
    const LINE_FEED: u8 = 0x0A;
    const CARRIAGE_RETURN: u8 = 0x0D;

    let mut stats = LineEndingStats { lf: 0, crlf: 0 };
    let mut prev: u8 = 0;

    for byte in bytes.into_iter() {
        if byte == LINE_FEED {
            if prev == CARRIAGE_RETURN {
                stats.crlf += 1;
            } else {
                stats.lf += 1;
            }
        }

        prev = byte;
    }

    stats
}
