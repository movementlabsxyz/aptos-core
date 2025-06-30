use anyhow::{anyhow, Result};
use aptos_crypto::x25519;
use aptos_types::account_address::from_identity_public_key;
use clap::Parser;
use l1_migration::extract_genesis_and_waypoint;
use std::path::PathBuf;

/// L1 Migration Tool - Extract genesis and waypoint from database
#[derive(Parser)]
#[command(name = "l1-migration")]
#[command(about = "adhoc command for l1 migration")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Generate waypoint and genesis files from database
    GenerateWaypointGenesis {
        /// Path to the database directory
        db_path: PathBuf,
        /// Destination directory for extracted files
        destination_path: PathBuf,
    },
    /// Calculate peer ID from network private key
    CalculatePeerId {
        /// Network private key in hex format (with or without 0x prefix)
        private_key: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::GenerateWaypointGenesis { db_path, destination_path } => {
            // Validate that the database path exists
            if !db_path.exists() {
                eprintln!("Error: Database path '{}' does not exist", db_path.display());
                std::process::exit(1);
            }

            // Create destination directory if it doesn't exist
            if !destination_path.exists() {
                std::fs::create_dir_all(&destination_path)?;
            }

            // Call the extraction function from the module
            let db_path_str = db_path.to_string_lossy();
            let destination_path_str = destination_path.to_string_lossy();
            
            extract_genesis_and_waypoint(&db_path_str, &destination_path_str)
        },
        Commands::CalculatePeerId { private_key } => {
            calculate_peer_id_from_private_key(&private_key)
        }
    }
}

fn calculate_peer_id_from_private_key(private_key_hex: &str) -> Result<()> {
    // Remove 0x prefix if present
    let private_key_hex = private_key_hex.strip_prefix("0x").unwrap_or(private_key_hex);
    
    // Convert hex string to bytes
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| anyhow!("Invalid hex format: {}", e))?;
    
    // Validate key length (x25519 private key should be 32 bytes)
    if private_key_bytes.len() != 32 {
        return Err(anyhow!("Invalid private key length: expected 32 bytes, got {}", private_key_bytes.len()));
    }
    
    // Create x25519 private key
    let private_key = x25519::PrivateKey::try_from(private_key_bytes.as_slice())
        .map_err(|e| anyhow!("Invalid private key: {}", e))?;
    
    // Get the public key
    let public_key = private_key.public_key();
    
    // Calculate peer ID using Aptos function
    let peer_id = from_identity_public_key(public_key);
    
    // Display results
    println!("Network Private Key: 0x{}", private_key_hex);
    println!("Network Public Key:  0x{}", hex::encode(public_key.as_slice()));
    println!("Peer ID:             0x{}", peer_id.to_hex());
    
    // Show the calculation details
    let pubkey_slice = public_key.as_slice();
    let last_16_bytes = &pubkey_slice[32 - 16..];
    println!("\nCalculation Details:");
    println!("- Public key (32 bytes): 0x{}", hex::encode(pubkey_slice));
    println!("- Last 16 bytes:         0x{}", hex::encode(last_16_bytes));
    println!("- Peer ID (last 16):     0x{}", peer_id.to_hex());
    
    // Verify the calculation manually
    let manual_peer_id_bytes = &pubkey_slice[16..];
    println!("- Manual calculation:    0x{}", hex::encode(manual_peer_id_bytes));
    
    Ok(())
}
