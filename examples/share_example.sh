#!/bin/bash
# Example script to demonstrate using MIGA to fetch and share content on the IPFS network

# Set the log level to info for better visibility
export RUST_LOG=info

# Build the project (if not already built)
echo "Building MIGA..."
cargo build --release

# Example CID for the IPFS welcome page
# This is a well-known CID that should be available on the IPFS network
CID="QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG"

echo "Fetching and sharing content with CID: $CID"
echo "This may take a while as MIGA connects to the IPFS network..."

# Run MIGA with the example CID, IPFS sharing enabled, and a description
./target/release/MIGA --cid $CID --verbose --share --description "IPFS Welcome Page"

# Note: The script will not complete until you press Ctrl+C to stop the IPFS node
# Other IPFS nodes can access the content using the CID while this node is running
