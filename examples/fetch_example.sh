#!/bin/bash
# Example script to demonstrate using MIGA to fetch content from IPFS

# Set the log level to info for better visibility
export RUST_LOG=info

# Build the project (if not already built)
echo "Building MIGA..."
cargo build --release

# Example CID for the IPFS welcome page
# This is a well-known CID that should be available on the IPFS network
CID="QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG"

echo "Fetching content with CID: $CID"
echo "This may take a while as MIGA connects to the IPFS network..."

# Run MIGA with the example CID and verbose output
./target/release/MIGA --cid $CID --verbose

echo "Example completed!"