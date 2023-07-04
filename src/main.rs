use clap::{value_parser, Arg, ArgMatches, Command};

mod le;

struct Configuration {
    glob_pattern: Option<glob::Pattern>,
    match_on: le::LineEndingType,
    print_line_numbers: bool,
}

fn main() {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("glob_pattern")
                .help("The glob pattern a file must match to be processed")
                .short('g')
                .long("glob")
                .value_parser(value_parser!(String)))
        .arg(
            Arg::new("line_numbers")
                .help("If the file contains mixed line endings, print which lines contain which line endings.")
                .short('l')
                .long("line-numbers")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("type")
                .help("The type of line endings to search for")
                .short('t')
                .long("type")
                .value_parser(["crlf", "lf", "mixed"])
                .default_value("mixed"),
        )
        .arg(
            Arg::new("paths")
                .help("The paths to process")
                .required(true)
                .action(clap::ArgAction::Append),
        )
        .get_matches();

    let config = get_configuration(&matches);

    for path in matches.get_many::<String>("paths").unwrap() {
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
                                le::LineEndingType::Lf => {
                                    if stats.is_lf() {
                                        println!("{} has LF line endings", path_display);
                                    }
                                }
                                le::LineEndingType::Crlf => {
                                    if stats.is_crlf() {
                                        println!("{} has CRLF line endings", path_display);
                                    }
                                }
                                le::LineEndingType::Mixed => {
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
        match_on: le::LineEndingType::Mixed,
        print_line_numbers: false,
    };

    config.match_on = match matches.get_one::<String>("type").unwrap().as_str() {
        "lf" => le::LineEndingType::Lf,
        "crlf" => le::LineEndingType::Crlf,
        _ => le::LineEndingType::Mixed,
    };

    config.print_line_numbers = matches.get_flag("line_numbers");

    if let Some(glob_string) = matches.get_one::<String>("glob_pattern") {
        config.glob_pattern =
            Some(glob::Pattern::new(glob_string).expect("Failed to read glob pattern"));
    }

    config
}
