use reqwest::{Client, Proxy};
use std::error::Error;
use crate::models::Config;
use crate::helpers::{fetch_token_metadata, print_token_metadata}; // Import functions from the helper file

/// Processes a list of tokens by fetching and printing their metadata.
///
/// # Arguments
/// * `config` - A reference to the `Config` struct containing configuration options.
///
/// # Returns
/// A `Result` indicating success or an error if the process fails.
///
/// # Description
/// This function creates a new HTTP client, retrieves token metadata for each token in the `config`,
/// and prints the metadata. It handles errors for each token individually and prints appropriate messages.
/// Verbosity and proxy settings from the `config` are used to configure the output and HTTP client.
pub async fn process_tokens(config: &Config) -> Result<(), Box<dyn Error>> {
    // Create a new HTTP client with proxy if provided
    let client = {
        let mut builder = Client::builder();

        if let Some(proxy_url) = &config.proxy {
            let proxy = Proxy::http(proxy_url)?;
            builder = builder.proxy(proxy);
        }

        builder.build()?
    };


    // Iterate over the list of tokens
    for token in &config.tokens {
      
        // Fetch metadata for the token and handle errors
        match fetch_token_metadata(&client, token, config.verbosity,config.proxy.clone()).await {
            Ok(metadata) => {
                if config.verbosity {
                    println!("Success: Retrieved token metadata:");
                }
                print_token_metadata(&metadata);
            }
            Err(e) => {
                eprintln!("Error fetching metadata for token {}: {}", token, e);
            }
        }
    }

    Ok(())
}
