use anyhow::{anyhow, Result};
use aptos_crypto::x25519;
use aptos_types::account_address::from_identity_public_key;
use clap::Parser;
use l1_migration::{extract_genesis_and_waypoint, generate_peer_set};
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
    /// Generate peer set YAML file for peer discovery
    GeneratePeerSet,
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
        Commands::GeneratePeerSet => {
            generate_peer_set();
            println!("✓ Peer set YAML file generated successfully at ./peer_set.yaml");
            Ok(())
        }
    }
}
