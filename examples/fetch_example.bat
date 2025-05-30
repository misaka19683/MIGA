@echo off
REM Example script to demonstrate using MIGA to fetch content from IPFS

REM Set the log level to info for better visibility
set RUST_LOG=info

REM Build the project (if not already built)
echo Building MIGA...
cargo build --release

REM Example CID for the IPFS welcome page
REM This is a well-known CID that should be available on the IPFS network
set CID=QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG

echo Fetching content with CID: %CID%
echo This may take a while as MIGA connects to the IPFS network...

REM Run MIGA with the example CID and verbose output
.\target\release\MIGA.exe --cid %CID% --verbose

echo Example completed!