# MIGA - IPFS Content Fetcher

MIGA is a command-line tool that allows you to fetch content from the IPFS (InterPlanetary File System) network using libp2p. It connects to the IPFS network, searches for content based on a CID (Content Identifier), and retrieves the data.

## Features

- Connect to the IPFS network using libp2p
- Fetch content using a CID
- Bootstrap with well-known IPFS nodes
- Verbose logging option for debugging

## Requirements

- Rust 1.70 or later
- Cargo package manager

## Installation

### From Source

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/MIGA.git
   cd MIGA
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The executable will be available at `target/release/MIGA`

## Usage

Basic usage:

```
MIGA --cid <CONTENT_ID>
```

### Command Line Arguments

- `-c, --cid <CID>`: The Content Identifier (CID) of the content to fetch from IPFS (required)
- `-o, --output <FILE>`: Path to save the fetched content (optional)
- `-v, --verbose`: Enable verbose output for debugging
- `-h, --help`: Display help information
- `-V, --version`: Display version information

### Examples

1. Fetch content with a specific CID:
   ```
   MIGA --cid QmZ4tDuvesekSs4qM5ZBKpXiZGun7S2CYtEZRB3DYXkjGx
   ```

2. Fetch content and save to a file:
   ```
   MIGA --cid QmZ4tDuvesekSs4qM5ZBKpXiZGun7S2CYtEZRB3DYXkjGx --output my_file.txt
   ```

3. Fetch with verbose logging:
   ```
   MIGA --cid QmZ4tDuvesekSs4qM5ZBKpXiZGun7S2CYtEZRB3DYXkjGx --verbose
   ```

### Example Scripts

The project includes example scripts in the `examples` directory to help you get started:

- **Windows**: Run `examples\fetch_example.bat` to fetch a sample IPFS content
- **Linux/macOS**: Run `examples/fetch_example.sh` to fetch a sample IPFS content

These scripts:
1. Set the appropriate log level
2. Build the project if needed
3. Fetch a well-known IPFS content (the IPFS welcome page)
4. Display the results

## Environment Variables

- `RUST_LOG`: Controls the logging level. Set to `info`, `debug`, or `trace` for different verbosity levels.

Example:
```
RUST_LOG=debug MIGA --cid QmZ4tDuvesekSs4qM5ZBKpXiZGun7S2CYtEZRB3DYXkjGx
```

## How It Works

MIGA uses the libp2p library to connect to the IPFS network. When you provide a CID, the tool:

1. Creates a new peer identity
2. Connects to bootstrap nodes in the IPFS network
3. Uses the Kademlia DHT (Distributed Hash Table) to find the content
4. Retrieves the content from peers that have it
5. Displays or saves the content based on your options

## Current Limitations

- The tool currently only prints debug information about the retrieved content
- File saving functionality is not fully implemented yet
- Limited error handling for network issues

## License

This project is licensed under the MIT License - see the LICENSE file for details.
