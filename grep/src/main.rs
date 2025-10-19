use colored::Colorize;
use regex::{Regex, RegexBuilder};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    // Skip the binary name so options can be provided before or after the pattern.
    let args: Vec<String> = env::args().skip(1).collect();
    match Config::parse(args) {
        Ok(ParseOutcome::HelpPrinted) => {}
        Ok(ParseOutcome::Run(config)) => {
            if let Err(error) = run(&config) {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }
        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }
}

fn run(config: &Config) -> io::Result<()> {
    let targets = collect_targets(&config.inputs, config.recursive);
    for path in targets {
        process_file(&path, config)?;
    }
    Ok(())
}

fn collect_targets(inputs: &[String], recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for input in inputs {
        let path = PathBuf::from(input);
        if path.is_dir() {
            if recursive {
                // Walk nested directories when -r is present, queuing every file for scanning.
                for entry in WalkDir::new(&path).into_iter().filter_map(Result::ok) {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        files.push(entry_path.to_path_buf());
                    }
                }
            }
        } else if path.is_file() {
            files.push(path);
        } else {
            // Keep the original path even if it does not exist; processing will raise an error.
            files.push(path);
        }
    }

    files
}

fn process_file(path: &Path, config: &Config) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let is_match = config.matcher.is_match(&line);
        let should_print = if config.invert_match {
            !is_match
        } else {
            is_match
        };

        if should_print {
            let display_line = if config.colored && is_match && !config.invert_match {
                highlight_line(&line, &config.matcher)
            } else {
                line.clone()
            };

            if let Some(prefix) = build_prefix(path, index + 1, config) {
                println!("{}: {}", prefix, display_line);
            } else {
                println!("{}", display_line);
            }
        }
    }

    Ok(())
}

fn highlight_line(line: &str, matcher: &Regex) -> String {
    // Replace each match with a colored version so only the pattern stands out.
    matcher
        .replace_all(line, |caps: &regex::Captures| caps[0].red().to_string())
        .to_string()
}

fn build_prefix(path: &Path, line_number: usize, config: &Config) -> Option<String> {
    let mut parts = Vec::new();

    if config.show_filenames {
        parts.push(path.to_string_lossy().into_owned());
    }

    if config.show_line_numbers {
        parts.push(line_number.to_string());
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(":"))
    }
}

fn print_usage() {
    println!("Usage: grep [OPTIONS] <pattern> <files...>");
    println!();
    println!("Options:");
    println!("-i                Case-insensitive search");
    println!("-n                Print line numbers");
    println!("-v                Invert match (exclude lines that match the pattern)");
    println!("-r                Recursive directory search");
    println!("-f                Print filenames");
    println!("-c                Enable colored output");
    println!("-h, --help        Show help information");
}

struct Config {
    inputs: Vec<String>,
    show_line_numbers: bool,
    invert_match: bool,
    recursive: bool,
    show_filenames: bool,
    colored: bool,
    matcher: Regex,
}

enum ParseOutcome {
    HelpPrinted,
    Run(Config),
}

impl Config {
    fn parse(args: Vec<String>) -> Result<ParseOutcome, String> {
        if args.is_empty() {
            return Err("Missing arguments. Use -h for help.".to_string());
        }

        let mut case_insensitive = false;
        let mut show_line_numbers = false;
        let mut invert_match = false;
        let mut recursive = false;
        let mut show_filenames = false;
        let mut colored = false;
        let mut pattern: Option<String> = None;
        let mut inputs: Vec<String> = Vec::new();
        let mut options_done = false;

        for arg in args {
            if !options_done {
                match arg.as_str() {
                    "-h" | "--help" => {
                        print_usage();
                        return Ok(ParseOutcome::HelpPrinted);
                    }
                    "-i" => {
                        case_insensitive = true;
                        continue;
                    }
                    "-n" => {
                        show_line_numbers = true;
                        continue;
                    }
                    "-v" => {
                        invert_match = true;
                        continue;
                    }
                    "-r" => {
                        recursive = true;
                        continue;
                    }
                    "-f" => {
                        show_filenames = true;
                        continue;
                    }
                    "-c" => {
                        colored = true;
                        continue;
                    }
                    "--" => {
                        options_done = true;
                        continue;
                    }
                    _ => {}
                }
            }

            if pattern.is_none() {
                pattern = Some(arg);
            } else {
                inputs.push(arg);
            }
        }

        let pattern = pattern.ok_or_else(|| "Missing search pattern.".to_string())?;

        if inputs.is_empty() {
            return Err("Missing input files.".to_string());
        }

        // Escape the literal pattern so flags behave the same regardless of special characters.
        let matcher = RegexBuilder::new(&regex::escape(&pattern))
            .case_insensitive(case_insensitive)
            .build()
            .map_err(|err| err.to_string())?;

        Ok(ParseOutcome::Run(Config {
            inputs,
            show_line_numbers,
            invert_match,
            recursive,
            show_filenames,
            colored,
            matcher,
        }))
    }
}
