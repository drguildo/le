use clap::{App, Arg};

mod le;

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
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

    let match_on: le::LineEndingType = match matches.value_of("type").unwrap() {
        "lf" => le::LineEndingType::LF,
        "crlf" => le::LineEndingType::CRLF,
        _ => le::LineEndingType::MIXED,
    };

    let print_line_numbers: bool = matches.is_present("line_numbers");

    for path in matches.values_of("paths").unwrap() {
        process_path(path, &match_on, print_line_numbers);
    }
}

fn process_path(path: &str, match_on: &le::LineEndingType, print_line_numbers: bool) {
    for entry in walkdir::WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let path_display = entry.path().display();
                    match std::fs::read(entry.path()) {
                        Ok(file_bytes) => {
                            let stats = le::count_line_endings(&file_bytes);

                            match match_on {
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
                                        if print_line_numbers {
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
