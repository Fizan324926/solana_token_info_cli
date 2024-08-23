# solana_token_info_cli
# Solana Token Metadata Fetcher

This Rust-based command-line tool fetches metadata for Solana tokens using the Solana Explorer API. It supports various options like verbosity, proxy usage, and multithreading, allowing efficient and customizable metadata retrieval.

## Features
- Fetch metadata for multiple Solana tokens.
- Support for proxies to route requests.
- Configurable verbosity for detailed output.
- Option to read token addresses from a file.
- Multithreaded processing for faster performance.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Ensure Rust and Cargo are installed on your system)

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/solana-token-fetcher.git
    cd solana-token-fetcher
    ```

2. **Build the project**:
    ```sh
    cargo build --release
    ```

   This will compile the project and produce an executable in the `target/release` directory.

## Usage

### Basic Command

After building, you can run the tool with various command-line options:

```sh
./target/release/solana-token-fetcher [OPTIONS]

```


