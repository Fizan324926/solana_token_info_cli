#![allow(warnings)]
mod args;
mod models;
mod token_processor;
mod helpers;

use models::Config;
use tokio;

#[tokio::main]
async fn main() {
    // Parse command-line arguments to get the configuration.
    let config = args::parse_args();

    // Display basic configuration details.
    println!(
        "Configuration: Tokens found: {}, Proxy: {:?}, Threads: {}",
        config.tokens.len(),
        config.proxy,
        config.num_threads
    );

    // Print verbosity details if enabled.
    if config.verbosity {
        println!("Verbosity is enabled.");
        // Additional verbosity details can be added here if needed.
    }

    // Process tokens asynchronously.
    if let Err(e) = token_processor::process_tokens(&config).await {
        eprintln!("Error processing tokens: {}", e);
        std::process::exit(1);
    }
}
