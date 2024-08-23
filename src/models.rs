use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub address: Option<String>,
    pub supply: Option<u64>,
    pub website: Option<String>,
    pub num_dns_records: Option<usize>, 
    pub dns_records: Option<Vec<String>>,
    pub update_authority: Option<String>, // Add this field
}


/// Struct to hold configuration options.
#[derive(Debug)]
pub struct Config {
    pub tokens: Vec<String>,
    pub proxy: Option<String>,
    pub verbosity: bool,
    pub output: Option<String>,
    pub num_threads: usize,  // New field for number of threads
}

impl Config {
    /// Prints help information.
    /// Displays usage instructions and available options.
    pub fn print_help() {
        println!("Usage: your_app_name [OPTIONS]");
        println!();
        println!("Options:");
        println!("  -t, --token <TOKENS>      Specify one or more tokens");
        println!("  -tf, --token-file <FILE>  Specify a file containing tokens");
        println!("  -v, --verbosity           Enable verbosity");
        println!("  -p, --proxy <URL>         Specify a proxy URL");
        println!("  -o, --output <FILE>       Specify an output file");
        println!("  -n, --threads <NUMBER>    Specify the number of threads (default: 4)");
        println!("  -h, --help                Display this help message");
    }
}
