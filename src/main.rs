enum LineEndingType {
    LF,
    CRLF,
    MIXED,
}

#[derive(Debug)]
struct LineEndingStats {
    lf: Vec<usize>,
    crlf: Vec<usize>,
}

impl LineEndingStats {
    fn is_lf(&self) -> bool {
        self.lf.len() > 0 && self.crlf.len() == 0
    }

    fn is_crlf(&self) -> bool {
        self.lf.len() == 0 && self.crlf.len() > 0
    }

    fn is_mixed(&self) -> bool {
        self.lf.len() > 0 && self.crlf.len() > 0
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
                .short("t")
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
                            let stats = count_line_endings(&file_bytes);
                            println!("{:?}", stats);

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

fn count_line_endings(bytes: &[u8]) -> LineEndingStats {
    const LINE_FEED: u8 = 0x0A;
    const CARRIAGE_RETURN: u8 = 0x0D;

    let mut stats = LineEndingStats {
        lf: vec![],
        crlf: vec![],
    };
    let mut prev: u8 = 0;
    let mut line_number: usize = 1;

    for byte in bytes.into_iter() {
        if *byte == LINE_FEED {
            if prev == CARRIAGE_RETURN {
                stats.crlf.push(line_number);
            } else {
                stats.lf.push(line_number);
            }
            line_number += 1;
        }

        prev = *byte;
    }

    stats
}
