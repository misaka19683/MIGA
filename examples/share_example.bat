@echo off
REM Example script to demonstrate using MIGA to fetch and share content on the IPFS network

REM Set the log level to info for better visibility
set RUST_LOG=info

REM Build the project (if not already built)
echo Building MIGA...
cargo build --release

REM Example CID for the IPFS welcome page
REM This is a well-known CID that should be available on the IPFS network
set CID=QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG

echo Fetching and sharing content with CID: %CID%
echo This may take a while as MIGA connects to the IPFS network...

REM Run MIGA with the example CID, IPFS sharing enabled, and a description
.\target\release\MIGA.exe --cid %CID% --verbose --share --description "IPFS Welcome Page"

REM Note: The script will not complete until you press Ctrl+C to stop the IPFS node
REM Other IPFS nodes can access the content using the CID while this node is running
