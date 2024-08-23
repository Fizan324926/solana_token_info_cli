use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use crate::models::Config;

/// Parses command-line arguments and handles various options and flags.
///
/// # Returns
/// Returns a `Config` struct containing the parsed options and flags.
///
/// # Errors
/// Exits the program with a status code of 1 if there is an error reading the token file.
pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut tokens = Vec::new();
    let mut proxy = None;
    let mut verbosity = false;
    let mut output = None;
    let mut token_file = None;
    let mut num_threads = 4;  // Default value for number of threads

    // Create an iterator to process arguments
    let mut iter = args.iter().skip(1); // Skip the program name

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-t" | "--token" => {
                // Collect tokens until another flag is encountered
                while let Some(token) = iter.next() {
                    if token.starts_with('-') {
                        iter.next_back(); // Put back the argument if it's a flag
                        break;
                    }
                    tokens.push(token.clone());
                }
            }
            "-tf" | "--token-file" => {
                if let Some(file) = iter.next() {
                    token_file = Some(PathBuf::from(file));
                }
            }
            "-v" | "--verbosity" => {
                verbosity = true;
            }
            "-p" | "--proxy" => {
                if let Some(url) = iter.next() {
                    proxy = Some(url.clone());
                }
            }
            "-o" | "--output" => {
                if let Some(file) = iter.next() {
                    output = Some(file.clone());
                }
            }
            "-n" | "--threads" => {
                if let Some(threads) = iter.next() {
                    // Parse the number of threads, default to 4 if parsing fails
                    num_threads = threads.parse().unwrap_or(4);
                }
            }
            "-h" | "--help" => {
                Config::print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                Config::print_help();
                std::process::exit(1);
            }
        }
    }

    // Handle token file if provided
    if let Some(file) = token_file {
        match fs::read_to_string(file) {
            Ok(content) => tokens.extend(content.lines().map(String::from)),
            Err(err) => {
                eprintln!("Error reading token file: {}", err);
                std::process::exit(1);
            }
        }
    }

    // If no tokens were provided via flags or file, prompt for one
    if tokens.is_empty() {
        print!("Enter Solana token address: ");
        io::stdout().flush().unwrap();
        let mut token = String::new();
        io::stdin().read_line(&mut token).unwrap();
        tokens.push(token.trim().to_string());
    }

    Config {
        tokens,
        proxy,
        verbosity,
        output,
        num_threads,
    }
}
