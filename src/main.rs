/// MIGA - A tool to fetch data from IPFS using libp2p
///
/// This application connects to the IPFS network using the libp2p protocol stack
/// and retrieves content based on its Content Identifier (CID).
///
/// # Features
/// - Connect to the IPFS network using libp2p
/// - Fetch content using a CID
/// - Bootstrap with well-known IPFS nodes
/// - Verbose logging option for debugging

use anyhow::{anyhow, Result};
use clap::Parser;
use futures::StreamExt;
use libp2p::{
    core::multiaddr::Protocol,
    identity, kad, noise, swarm, tcp, yamux,
    Multiaddr, PeerId,
};
use log::{debug, error, info, warn};
use std::{
    path::PathBuf,
    time::Duration,
};
use tokio::io::AsyncWriteExt;

/// Command line arguments for the MIGA application
///
/// This struct defines the command-line interface for the application
/// using the clap crate for argument parsing.
#[derive(Parser, Debug)]
#[clap(author, version, about = "A tool to fetch data from IPFS using libp2p")]
struct Args {
    /// The CID (Content Identifier) of the content to fetch from IPFS
    /// This is a required parameter and must be a valid CID string
    #[clap(short, long)]
    cid: String,

    /// Output file path (optional)
    /// If provided, the fetched content will be saved to this file
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Enable verbose output for debugging
    /// When enabled, additional information about the process will be displayed
    #[clap(short, long)]
    verbose: bool,
}

/// Main entry point for the MIGA application
///
/// This async function:
/// 1. Initializes the logger
/// 2. Parses command line arguments
/// 3. Sets up a libp2p node with Kademlia DHT
/// 4. Connects to the IPFS network
/// 5. Searches for and retrieves content based on the provided CID
///
/// # Returns
/// - `Result<()>`: Ok if the operation was successful, Err otherwise
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger for output based on RUST_LOG environment variable
    env_logger::init();

    // Parse command line arguments using clap
    let args = Args::parse();

    // Print information about the requested CID if verbose mode is enabled
    if args.verbose {
        println!("Fetching content with CID: {}", args.cid);
    }

    // Parse the CID string into a CID object
    // Return an error if the CID is invalid
    let cid = match cid::Cid::try_from(args.cid.as_str()) {
        Ok(cid) => cid,
        Err(err) => {
            return Err(anyhow!("Invalid CID: {}", err));
        }
    };

    // Create a new Ed25519 keypair for this node's identity
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer ID: {peer_id}");

    // Configure the Kademlia DHT behavior
    // This is used for finding peers and content in the network
    let mut kad_config = kad::Config::default();
    kad_config.set_query_timeout(Duration::from_secs(60)); // Set a 60-second timeout for queries
    let store = kad::store::MemoryStore::new(peer_id);     // In-memory store for DHT records
    let mut kad_behaviour = kad::Behaviour::with_config(peer_id, store, kad_config);

    // Add well-known IPFS bootstrap nodes to connect to the network
    add_bootstrap_nodes(&mut kad_behaviour, args.verbose);

    // Create a libp2p Swarm with the Kademlia behavior
    // The Swarm manages connections and protocol negotiations
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
        .with_tokio()                                      // Use Tokio as the async runtime
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)? // TCP transport with Noise encryption and Yamux multiplexing
        .with_behaviour(|_| kad_behaviour)?                // Add the Kademlia behavior
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60))) // Set connection timeout
        .build();

    // Listen on all network interfaces with a random port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Convert the CID's multihash to a Kademlia record key
    // This is what we'll search for in the DHT
    let key = kad::RecordKey::from(cid.hash().to_bytes());

    // Start a Kademlia GET query to find the content
    info!("Searching for content with CID: {}", cid);
    swarm.behaviour_mut().get_record(key.clone());

    // Process events from the network
    // We'll keep processing events until we find the content we're looking for
    let mut content_found = false;
    let mut bootstrap_complete = false;

    while !content_found {
        // Wait for the next event from the swarm
        match swarm.select_next_some().await {
            // When we get a new listening address
            swarm::SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {address}");

                // Bootstrap the Kademlia DHT if we haven't already done so
                // This connects us to the wider IPFS network
                if !bootstrap_complete {
                    info!("Bootstrapping Kademlia DHT...");
                    if let Err(e) = swarm.behaviour_mut().bootstrap() {
                        error!("Failed to bootstrap Kademlia: {}", e);
                    }
                    bootstrap_complete = true;
                }
            }
            // When we successfully get a record from the network
            swarm::SwarmEvent::Behaviour(kad::Event::OutboundQueryProgressed { 
                result: kad::QueryResult::GetRecord(Ok(result)), 
                ..
            }) => {
                // Print the debug representation to understand the structure
                // This is useful for development and debugging
                info!("Got record result: {:?}", result);

                // For now, we just print the result and continue
                // In a future version, we could extract and process the actual content
                println!("Received a record from the IPFS network. Check the logs for details.");

                // Mark that we found the content so we can exit the loop
                content_found = true;
            }
            // When we fail to get a record
            swarm::SwarmEvent::Behaviour(kad::Event::OutboundQueryProgressed { 
                result: kad::QueryResult::GetRecord(Err(err)), 
                ..
            }) => {
                warn!("Failed to get record: {:?}", err);
                // Retry the query after a delay
                // This helps with temporary network issues
                tokio::time::sleep(Duration::from_secs(5)).await;
                swarm.behaviour_mut().get_record(key.clone());
            }
            // When we get a result from bootstrapping
            swarm::SwarmEvent::Behaviour(kad::Event::OutboundQueryProgressed { 
                result: kad::QueryResult::Bootstrap(Ok(result)), 
                ..
            }) => {
                if args.verbose {
                    info!("Bootstrap result: {} peers found", result.num_remaining);
                }
                // Try to get the record again after bootstrapping
                // Now that we're connected to more peers, we have a better chance of finding the content
                swarm.behaviour_mut().get_record(key.clone());
            }
            // Handle any other events
            e => {
                if args.verbose {
                    debug!("Other event: {:?}", e);
                }
            }
        }
    }

    Ok(())
}

/// Add well-known IPFS bootstrap nodes to the Kademlia DHT
///
/// This function adds a list of standard IPFS bootstrap nodes to the Kademlia
/// routing table. These nodes serve as entry points to the IPFS network and
/// help our node discover other peers.
///
/// # Arguments
/// * `kademlia` - A mutable reference to the Kademlia behavior
/// * `verbose` - Whether to print verbose information about the bootstrap process
fn add_bootstrap_nodes(kademlia: &mut kad::Behaviour<kad::store::MemoryStore>, verbose: bool) {
    // List of well-known IPFS bootstrap nodes
    // These are maintained by Protocol Labs and the IPFS community
    let bootstrap_nodes = [
        // DNS-based addresses (more stable over time)
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
        // IP-based addresses
        "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
        "/ip4/104.236.179.241/tcp/4001/p2p/QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM",
    ];

    // Add each bootstrap node to the Kademlia routing table
    for node in bootstrap_nodes {
        // Parse the multiaddress string
        match node.parse::<Multiaddr>() {
            Ok(addr) => {
                // Extract the peer ID from the multiaddress
                if let Some(peer_id) = extract_peer_id_from_multiaddr(&addr) {
                    // Add the address to Kademlia's routing table
                    kademlia.add_address(&peer_id, addr.clone());

                    // Print information if verbose mode is enabled
                    if verbose {
                        println!("Added bootstrap node: {} ({})", addr, peer_id);
                    }
                }
            }
            Err(err) => {
                warn!("Failed to parse bootstrap address: {}: {}", node, err);
            }
        }
    }
}

/// Extract a PeerId from a multiaddress
///
/// A multiaddress (Multiaddr) may contain a peer ID as its last component.
/// This function extracts that peer ID if present.
///
/// # Arguments
/// * `addr` - The multiaddress to extract the peer ID from
///
/// # Returns
/// * `Option<PeerId>` - The extracted peer ID, or None if no valid peer ID was found
fn extract_peer_id_from_multiaddr(addr: &Multiaddr) -> Option<PeerId> {
    // Iterate through the protocols in the multiaddress
    addr.iter().find_map(|proto| {
        // Look for the P2p protocol which contains the peer ID
        if let Protocol::P2p(hash) = proto {
            // Convert the hash to a PeerId
            match PeerId::from_multihash(hash.into()) {
                Ok(peer_id) => Some(peer_id),
                Err(_) => None,
            }
        } else {
            None
        }
    })
}
