// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::node::global_storage_includes::GlobalStorageIncludes;
use crate::types::storage::{MovementAptosStorage, MovementStorage};
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;

#[derive(Parser, Debug)]
#[clap(
    name = "compare-states",
    about = "Compares balances for each transaction at specific ledger versions"
)]
pub struct CompareStates {
    #[clap(long = "movement-db", help = "Path to the Movement database.")]
    pub movement_db: PathBuf,
    #[clap(long = "aptos-db", help = "Path to the Aptos database.")]
    pub movement_aptos_db: PathBuf,
    #[arg(help = "First hash,version,version tuple")]
    first: String,
    #[arg(help = "Second hash,version,version tuple")]
    second: String,
}

impl CompareStates {
    pub async fn run(&self) -> anyhow::Result<()> {
        let movement_storage = MovementStorage::open(&self.movement_db)?;
        let aptos_storage = MovementAptosStorage::open(&self.movement_aptos_db)?;

        compare_states(&movement_storage, &aptos_storage, &self.first, &self.second).await?;

        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    CompareStates::command().debug_assert()
}

async fn compare_states(
    movement_storage: &MovementStorage,
    aptos_storage: &MovementAptosStorage,
    first: &str,
    second: &str,
) -> anyhow::Result<()> {
    let (hash1, aptos_version1, movement_version1) = parse_line(first)?;
    let (hash2, aptos_version2, movement_version2) = parse_line(second)?;

    info!(
        "Comparing post transaction {}: Movement version: {}, Aptos version: {}",
        hash1, movement_version1, aptos_version1
    );

    let result1 = GlobalStorageIncludes::compare_db(
        movement_storage,
        movement_version1,
        aptos_storage,
        aptos_version1,
    )?;

    info!(
        "Comparing post transaction {}: Movement version: {}, Aptos version: {}",
        hash2, movement_version2, aptos_version2
    );

    let result2 = GlobalStorageIncludes::compare_db(
        movement_storage,
        movement_version2,
        aptos_storage,
        aptos_version2,
    )?;

    let diff = result2
        .into_iter()
        .filter(|c| result1.contains(c))
        .collect::<Vec<_>>();

    for comparison in diff {
        info!("{}", comparison);
    }

    Ok(())
}

// async fn compare_balances(
//     movement_storage: &MovementStorage,
//     aptos_storage: &MovementAptosStorage,
//     path: &PathBuf,
// ) -> anyhow::Result<()> {
//     use tokio::fs::File;
//     use tokio::io::{AsyncBufReadExt, BufReader};
//     let file = File::open(path).await?;
//     let reader = BufReader::new(file);
//     let mut lines = reader.lines();
//
//     while let Some(line) = lines.next_line().await? {
//         let (hash, aptos_version, movement_version) = parse_line(&line)?;
//         info!(
//             "Processing transaction {}: Aptos version {}, Movement version {}",
//             hash, aptos_version, movement_version
//         );
//         let diff = GlobalStorageIncludes::compare_db(
//             movement_storage,
//             movement_version,
//             aptos_storage,
//             aptos_version,
//         )?;
//
//         for comparison in diff {}
//     }
//
//     Ok(())
// }

fn parse_line(line: &str) -> anyhow::Result<(&str, u64, u64)> {
    let parts = line.split(',').collect::<Vec<_>>();
    let parts: [&str; 3] = parts.try_into().map_err(|v: Vec<&str>| {
        anyhow::anyhow!(
            "Expected 3 parts extracted from the line. Found {}",
            v.len()
        )
    })?;
    let [hash, aptos_version, movement_version] = parts;
    let aptos_version = u64::from_str(aptos_version)?;
    let movement_version = u64::from_str(movement_version)?;
    Ok((hash, aptos_version, movement_version))
}
