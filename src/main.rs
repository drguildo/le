use clap::{App, Arg, ArgMatches};

mod le;

struct Configuration {
    glob_pattern: Option<glob::Pattern>,
    match_on: le::LineEndingType,
    print_line_numbers: bool,
}

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("glob_pattern")
                .help("The glob pattern a file must match to be processed")
                .short("g")
                .long("glob")
                .takes_value(true))
        .arg(
            Arg::with_name("line_numbers")
                .help("If the file contains mixed line endings, print which lines contain which line endings.")
                .short("l")
                .long("line-numbers")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("type")
                .help("The type of line endings to search for")
                .short("t")
                .long("type")
                .takes_value(true)
                .possible_values(["crlf", "lf", "mixed"].as_ref())
                .default_value("mixed"),
        )
        .arg(
            Arg::with_name("paths")
                .help("The paths to process")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let config = get_configuration(&matches);

    for path in matches.values_of("paths").unwrap() {
        process_path(path, &config);
    }
}

fn process_path(path: &str, config: &Configuration) {
    for entry in walkdir::WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    if let Some(glob_pattern) = config.glob_pattern.clone() {
                        if !glob_pattern.matches_path(entry.path()) {
                            continue;
                        }
                    }

                    let path_display = entry.path().display();
                    match std::fs::read(entry.path()) {
                        Ok(file_bytes) => {
                            let stats = le::count_line_endings(&file_bytes);

                            match config.match_on {
                                le::LineEndingType::LF => {
                                    if stats.is_lf() {
                                        println!("{} has LF line endings", path_display);
                                    }
                                }
                                le::LineEndingType::CRLF => {
                                    if stats.is_crlf() {
                                        println!("{} has CRLF line endings", path_display);
                                    }
                                }
                                le::LineEndingType::MIXED => {
                                    if stats.is_mixed() {
                                        println!("{} has mixed line endings", path_display);
                                        if config.print_line_numbers {
                                            println!("LF: {:?}", stats.lf);
                                            println!("CRLF: {:?}", stats.crlf);
                                        }
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

fn get_configuration(matches: &ArgMatches) -> Configuration {
    let mut config = Configuration {
        glob_pattern: None,
        match_on: le::LineEndingType::MIXED,
        print_line_numbers: false,
    };

    config.match_on = match matches.value_of("type").unwrap() {
        "lf" => le::LineEndingType::LF,
        "crlf" => le::LineEndingType::CRLF,
        _ => le::LineEndingType::MIXED,
    };

    config.print_line_numbers = matches.is_present("line_numbers");

    if let Some(glob_string) = matches.value_of("glob_pattern") {
        config.glob_pattern =
            Some(glob::Pattern::new(glob_string).expect("Failed to read glob pattern"));
    }

    config
}
