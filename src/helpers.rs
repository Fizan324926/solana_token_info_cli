use reqwest::{Client, Proxy};
use std::env;
use std::error::Error;
use serde_json::Value;
use crate::models::TokenMetadata;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;


/// Prints the metadata of a token.
///
/// # Arguments
/// * metadata - A reference to the TokenMetadata to print.
///
/// # Description
/// This function prints various details of the token, including its name, symbol, address, supply, website, and update authority.
/// It also prints DNS records if available.
pub fn print_token_metadata(metadata: &TokenMetadata) {
    println!("\nToken Metadata:");
    println!("  Name: {}", metadata.name.as_deref().unwrap_or("Unknown"));
    println!("  Symbol: {}", metadata.symbol.as_deref().unwrap_or("Unknown"));
    println!("  Address: {}", metadata.address.as_deref().unwrap_or("Unknown"));
    println!("  Supply: {}", metadata.supply.unwrap_or(0));
    println!("  Website: {}", metadata.website.as_deref().unwrap_or("Not available"));
    println!("  Update Authority: {}", metadata.update_authority.as_deref().unwrap_or("Not available"));

    match &metadata.num_dns_records {
        Some(count) => {
            println!("  Number of DNS Records: {}", count);
            if let Some(records) = &metadata.dns_records {
                println!("  DNS Records:");
                for record in records {
                    println!("    {}", record);
                }
            }
        }
        None => println!("  Number of DNS Records: Not available"),
    }

    println!("--------------------------"); // Adds an extra line for readability
}




/// Fetches metadata for a Solana token.
///
/// # Arguments
/// * `client` - A reference to the `reqwest::Client`.
/// * `token` - A reference to the token string.
/// * `verbosity` - A boolean to control detailed logging.
/// * `proxy` - An optional proxy URL to configure the client.
///
/// # Returns
/// A `Result` containing `TokenMetadata` if successful or an error if it fails.
pub async fn fetch_token_metadata(
    client: &Client,
    token: &str,
    verbosity: bool,
    proxy: Option<String>
) -> Result<TokenMetadata, Box<dyn Error>> {
    // Configure the HTTP client to use the proxy if provided
    let client = match proxy {
        Some(proxy_url) => Client::builder()
            .proxy(Proxy::all(proxy_url)?)
            .build()?,
        None => Client::new(),
    };

    // Get the API URL from the environment variable or use the default
    let api_url = env::var("API_URL").unwrap_or_else(|_| "https://explorer-api.mainnet-beta.solana.com/".to_string());

    if verbosity {
        println!("API URL: {}", api_url);
        println!("Token: {}", token);
       
    }

    let payload = serde_json::json!({
        "id": token,
        "jsonrpc": "2.0",
        "method": "getAsset",
        "params": {
            "id": token
        }
    });

    // Get a random user agent
    let user_agent = generate_random_user_agent();

    let response = client.post(&api_url)
        .header("accept", "*/*")
        .header("accept-encoding", "gzip, deflate, br, zstd")
        .header("accept-language", "en-US,en;q=0.9")
        .header("content-type", "application/json")
        .header("origin", "https://explorer.solana.com")
        .header("referer", "https://explorer.solana.com/")
        .header("sec-ch-ua", "\"Not)A;Brand\";v=\"99\", \"Google Chrome\";v=\"127\", \"Chromium\";v=\"127\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"Windows\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-site")
        .header("user-agent", user_agent)
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if verbosity {
        println!("Response Status: {}", status);
        println!("Response Text: {}", response_text);
    }

    if !status.is_success() {
        return Err(format!("Request failed with status: {}. Response: {}", status, response_text).into());
    }

    let json_response: Value = serde_json::from_str(&response_text)?;
    let metadata = json_response["result"]["content"]["metadata"].as_object();
    let supply_info = json_response["result"]["supply"].as_object();
    let links = json_response["result"]["content"]["links"].as_object();
    let authorities = json_response["result"]["authorities"].as_array();

    let website = links.and_then(|l| l.get("external_url").and_then(|v| v.as_str().map(String::from)));
    let update_authority = authorities.and_then(|a| a.get(0)?.get("address").and_then(|v| v.as_str().map(String::from)));

    let (num_dns_records, dns_records) = if let Some(ref website) = website {
        match fetch_dns_records(&extract_domain(website).unwrap_or_default()).await {
            Ok((count, records)) => (Some(count), Some(records)),
            Err(e) => {
                eprintln!("Error fetching DNS records: {}", e);
                (None, None)
            }
        }
    } else {
        (None, None)
    };

    let token_metadata = TokenMetadata {
        name: metadata.and_then(|m| m.get("name").and_then(|v| v.as_str().map(String::from))),
        symbol: metadata.and_then(|m| m.get("symbol").and_then(|v| v.as_str().map(String::from))),
        address: Some(token.to_string()),
        supply: supply_info.and_then(|s| s.get("print_current_supply").and_then(|v| v.as_u64())),
        website,
        num_dns_records,
        dns_records,
        update_authority,
    };

    Ok(token_metadata)
}

/// Extracts the domain from a URL.
///
/// # Arguments
/// * `url` - The URL string.
///
/// # Returns
/// A `Result` containing the domain if successful or an error if it fails.
fn extract_domain(url: &str) -> Result<String, Box<dyn Error>> {
    // Find the start of the domain by locating the "://" or "http(s)://" part
    let url = if let Some(pos) = url.find("://") {
        &url[(pos + 3)..]
    } else {
        url
    };

    // Find the end of the domain, which could be marked by the next '/' or the end of the string
    let end_pos = url.find('/').unwrap_or(url.len());

    // Extract the domain
    let domain = &url[..end_pos];

    // Return the domain as a String
    if domain.is_empty() {
        Err("Invalid URL: Domain not found".into())
    } else {
        Ok(domain.to_string())
    }
}

/// Fetches DNS records for a given domain.
///
/// # Arguments
/// * `domain` - The domain string.
///
/// # Returns
/// A `Result` containing the number of DNS records and a vector of record strings if successful or an error if it fails.
async fn fetch_dns_records(domain: &str) -> Result<(usize, Vec<String>), Box<dyn Error>> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    let response = resolver.lookup_ip(domain).await
        .map_err(|e| format!("Failed to lookup IP for domain: {}", e))?;

    let num_records = response.iter().count();
    let records: Vec<String> = response.iter().map(|ip| ip.to_string()).collect();

    Ok((num_records, records))
}

/// Generates a random user agent string.
///
/// # Returns
/// A random user agent string.
fn generate_random_user_agent() -> String {
    // List of user agents
    let user_agents = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:91.0) Gecko/20100101 Firefox/91.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:93.0) Gecko/20100101 Firefox/93.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:92.0) Gecko/20100101 Firefox/92.0",
    ];

    // Choose a random user agent
    let random_index = rand::random::<usize>() % user_agents.len();
    user_agents[random_index].to_string()
}
