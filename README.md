# Solana Token Metadata Fetcher

This Rust-based command-line tool fetches metadata for Solana tokens using the Solana Explorer API. It supports various options like verbosity, proxy usage, and multithreading, allowing efficient and customizable metadata retrieval.

## Features
- Fetch metadata for multiple Solana tokens.
- Support for proxies to route requests.
- Configurable verbosity for detailed output.
- Option to read token addresses from a file.
- Multithreaded processing for faster performance.

![Example Image](demo\demo.png)

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Ensure Rust and Cargo are installed on your system)

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/Fizan324926/solana_token_info_cli.git
    cd solana_token_info_cli
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
./target/release/solana_token_info_cli [OPTIONS]

```

## Command-Line Options

- `-t`, `--token <TOKEN>`: Provide one or more Solana token addresses directly via the command line.
- `-tf`, `--token-file <FILE>`: Specify a file containing a list of Solana token addresses (one per line).
- `-p`, `--proxy <URL>`: Route requests through the specified proxy URL.
- `-v`, `--verbosity`: Enable verbose output for more detailed logs.
- `-o`, `--output <FILE>`: Save the output to the specified file.
- `-n`, `--threads <NUMBER>`: Set the number of threads to use (default is 4).
- `-h`, `--help`: Display the help message and exit.

## Examples

### Fetch metadata for a single token:
```sh
./target/release/solana-token-fetcher -t YourTokenAddressHere
```

### Fetch metadata for multiple tokens:
```sh
./target/release/solana-token-fetcher -t Token1 Token2 Token3
```
### Use a proxy server:
```sh
./target/release/solana-token-fetcher -t YourTokenAddressHere -p http://yourproxy.com:8080
```
### Read tokens from a file:
```sh
./target/release/solana-token-fetcher -tf tokens.txt
```
### Enable verbose output:
```sh
./target/release/solana-token-fetcher -t YourTokenAddressHere -v
```
### Specify the number of threads:
```sh
./target/release/solana-token-fetcher -t YourTokenAddressHere -n 8
```
