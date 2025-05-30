# MIGA - IPFS Content Fetcher

MIGA is a command-line tool that allows you to fetch content from the IPFS (InterPlanetary File System) network using libp2p. It connects to the IPFS network, searches for content based on a CID (Content Identifier), and retrieves the data.

## Features

- Connect to the IPFS network using libp2p
- Fetch content using a CID
- Bootstrap with well-known IPFS nodes
- Verbose logging option for debugging
- IPFS network sharing for making content available to other IPFS nodes

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
- `--share`: Enable IPFS network sharing for making content available to other IPFS nodes
- `--port <PORT>`: Port to listen for IPFS connections (default: 4001)
- `--description <TEXT>`: Description of the content being shared (stored with content metadata)
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

   4. Fetch content and share it on the IPFS network:
   ```
   MIGA --cid QmZ4tDuvesekSs4qM5ZBKpXiZGun7S2CYtEZRB3DYXkjGx --share --description "IPFS Documentation"
   ```

   5. Fetch content and share it on a specific port:
   ```
   MIGA --cid QmZ4tDuvesekSs4qM5ZBKpXiZGun7S2CYtEZRB3DYXkjGx --share --port 5001
   ```

### Example Scripts

The project includes example scripts in the `examples` directory to help you get started:

- **Windows**: 
  - Run `examples\fetch_example.bat` to fetch a sample IPFS content
  - Run `examples\share_example.bat` to fetch and share content on the IPFS network
- **Linux/macOS**: 
  - Run `examples/fetch_example.sh` to fetch a sample IPFS content
  - Run `examples/share_example.sh` to fetch and share content on the IPFS network

These scripts:
1. Set the appropriate log level
2. Build the project if needed
3. Fetch a well-known IPFS content (the IPFS welcome page)
4. Display the results
5. For share examples, make the content available on the IPFS network for other nodes to access

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

When sharing is enabled, MIGA also:

1. Makes the content available on the IPFS network using the Kademlia DHT
2. Listens for incoming connections from other IPFS nodes
3. Provides the content to other nodes that request it using the CID
4. Displays your node's multiaddress that other nodes can use to connect directly

## Current Limitations

- Limited error handling for network issues
- No persistence for shared content (content is only available while the program is running)
- Limited NAT traversal capabilities (may require port forwarding for full connectivity)
- No content verification or integrity checking beyond what's provided by CIDs
- No bandwidth or resource usage limits

## License

This project is licensed under the MIT License - see the LICENSE file for details.
