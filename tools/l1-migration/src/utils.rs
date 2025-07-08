use anyhow::Result;
use aptos_config::config::{PeerSet, RocksdbConfigs, StorageDirPaths, NO_OP_STORAGE_PRUNER_CONFIG};
use aptos_db::AptosDB;
use aptos_storage_interface::DbReader;
use aptos_types::{transaction::Transaction, waypoint::Waypoint, PeerId};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs;
use aptos_types::account_address::from_identity_public_key;
use aptos_crypto::{x25519, ValidCryptoMaterialStringExt};
use aptos_config::config::{Peer, PeerRole};
use aptos_types::network_address::NetworkAddress;
use std::str::FromStr;
use serde_yaml;

/// Extract genesis transaction and waypoint from an Aptos database
pub fn extract_genesis_and_waypoint(db_path: &str, output_dir: &str) -> Result<()> {
    println!("Opening database at: {}", db_path);
    
    // Create storage directory paths
    let storage_dir_paths = StorageDirPaths::from_path(Path::new(db_path));
    
    // Open the database with correct API
    let db = AptosDB::open(
        storage_dir_paths,
        true,  // readonly
        NO_OP_STORAGE_PRUNER_CONFIG, // pruner_config
        RocksdbConfigs::default(),
        false, // enable_indexer
        1,     // buffered_state_target_items
        10000, // max_num_nodes_per_lru_cache_shard
        None,  // internal_indexer_db
    )?;

    println!("Database opened successfully");

    // Get the latest version to understand the database state
    let latest_version = db.get_synced_version()?;
    println!("Latest synced version: {:?}", latest_version);

    if latest_version.is_none() {
        return Err(anyhow::anyhow!("Database has no synced version"));
    }

    let latest_ver = latest_version.unwrap();
    
    // Extract genesis transaction
    extract_genesis_transaction(&db, latest_ver, output_dir)?;
    
    // Extract waypoint
    extract_waypoint(&db, output_dir)?;

    println!("✓ Genesis extraction completed successfully!");
    println!("  - genesis.blob: Contains the BCS-serialized genesis transaction");
    println!("  - waypoint.txt: Contains the initial waypoint for bootstrapping");
    
    Ok(())
}

/// Extract the genesis transaction from the database
fn extract_genesis_transaction(db: &AptosDB, latest_ver: u64, output_dir: &str) -> Result<()> {
    println!("Extracting genesis transaction (version 0)...");
    let genesis_txn_with_proof = db.get_transaction_by_version(0, latest_ver, false)?;
    let genesis_transaction = genesis_txn_with_proof.transaction;

    // Serialize the genesis transaction using BCS
    let genesis_bytes = bcs::to_bytes(&genesis_transaction)?;
    
    // Write genesis.blob
    let genesis_path = format!("{}/genesis.blob", output_dir);
    fs::write(&genesis_path, &genesis_bytes)?;
    println!("Genesis transaction written to: {}", genesis_path);
    println!("Genesis blob size: {} bytes", genesis_bytes.len());

    // Print information about the genesis transaction
    print_genesis_transaction_info(&genesis_transaction);

    Ok(())
}

/// Extract the waypoint from the database using proper waypoint conversion
fn extract_waypoint(db: &AptosDB, output_dir: &str) -> Result<()> {
    // Get the ledger info to extract waypoint
    let ledger_info_with_sigs = db.get_latest_ledger_info()?;
    let ledger_info = ledger_info_with_sigs.ledger_info();
    
    // Generate waypoint using the proper converter
    let waypoint = Waypoint::new_any(ledger_info);
    
    // Write waypoint.txt
    let waypoint_path = format!("{}/waypoint.txt", output_dir);
    fs::write(&waypoint_path, waypoint.to_string())?;
    println!("Waypoint written to: {}", waypoint_path);
    println!("Waypoint: {}", waypoint);

    Ok(())
}

/// Print detailed information about the genesis transaction
fn print_genesis_transaction_info(genesis_transaction: &Transaction) {
    match genesis_transaction {
        Transaction::GenesisTransaction(genesis_payload) => {
            println!("✓ Found GenesisTransaction (WriteSet transaction)");
            // Access the payload correctly
            match genesis_payload {
                aptos_types::transaction::WriteSetPayload::Direct(change_set) => {
                    println!("  Direct WriteSet payload");
                    println!("  Change set size: {} bytes", bcs::to_bytes(change_set).unwrap_or_default().len());
                },
                aptos_types::transaction::WriteSetPayload::Script { .. } => {
                    println!("  Script-based WriteSet");
                },
            }
        },
        Transaction::BlockMetadata(_) => {
            println!("⚠ Transaction 0 is BlockMetadata (unexpected for genesis)");
        },
        Transaction::BlockMetadataExt(_) => {
            println!("⚠ Transaction 0 is BlockMetadataExt (unexpected for genesis)");
        },
        Transaction::BlockEpilogue(_) => {
            println!("⚠ Transaction 0 is BlockEpilogue (unexpected for genesis)");
        },
        Transaction::UserTransaction(_) => {
            println!("⚠ Transaction 0 is UserTransaction (unexpected for genesis)");
        },
        Transaction::StateCheckpoint(_) => {
            println!("⚠ Transaction 0 is StateCheckpoint (unexpected for genesis)");
        },
        Transaction::ValidatorTransaction(_) => {
            println!("⚠ Transaction 0 is ValidatorTransaction (unexpected for genesis)");
        },
    }
}

/// Generate a peerSet yaml for peer discover
pub fn generate_peer_set() {
    let mut pset: PeerSet = HashMap::new();
    // Get the public key
    let public_key = x25519::PublicKey::from_encoded_string("0xac456ebc028d030b3fcf5e97ca3e7bf2286a2f3bff70306ae2ceac4935931d6f").unwrap();
    // Calculate peer ID using Aptos function
    let peer_id = from_identity_public_key(public_key);
    let network_address = format!("/ip4/192.168.0.174/tcp/6180/noise-ik/{}/handshake/0", peer_id.to_hex_literal());
    let addr = NetworkAddress::from_str(network_address.as_str()).unwrap();
    let peer = Peer::new(vec![addr], HashSet::new() , PeerRole::Validator);
    pset.insert(peer_id, peer);
    fs::write("./peer_set.yaml", serde_yaml::to_string(&pset).unwrap()).unwrap();
}
