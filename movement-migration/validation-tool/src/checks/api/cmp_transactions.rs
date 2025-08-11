// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::types::api::MovementAptosRestClient;
use aptos_rest_client::aptos_api_types::{TransactionData, TransactionOnChainData};
use clap::Parser;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{error, info};

#[derive(Parser)]
#[clap(
    name = "compare-transactions",
    about = "Compares transactions with the same transactions from a remote validator node"
)]
pub struct CompareTransactions {
    #[clap(value_parser)]
    #[clap(long = "url", help = "The url of the Movement Aptos REST endpoint.")]
    pub rest_api_url: String,
    #[clap(long = "in", help = "Input path file name")]
    pub path: PathBuf,
}

impl CompareTransactions {
    pub async fn run(self) -> anyhow::Result<()> {
        let rest_client = MovementAptosRestClient::new(&self.rest_api_url)?;
        compare_transactions(&rest_client, self.path).await?;
        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    CompareTransactions::command().debug_assert()
}

async fn compare_transactions(
    rest_client: &MovementAptosRestClient,
    path: PathBuf,
) -> anyhow::Result<()> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut error = false;

    while let Some(line) = lines.next_line().await? {
        let bytes = hex::decode(line.trim_end())?;
        let tx_data_local = bcs::from_bytes::<'_, TransactionOnChainData>(&bytes)?;
        let hash = tx_data_local.info.transaction_hash();
        if let Ok(response) = rest_client.get_transaction_by_hash_bcs(hash).await {
            if let TransactionData::OnChain(tx_data_remote) = response.into_inner() {
                if tx_data_local == tx_data_remote {
                    info!("Checked transaction with hash {}", hash);
                } else {
                    error!("Remote transaction with hash {} mismatch", hash);
                    error = true;
                }
            } else {
                error!("Remote transaction with hash {} is pending", hash);
                error = true;
            };
        } else {
            error!("Remote transaction with hash {} not found", hash);
            error = true;
        }
    }

    if error {
        Err(anyhow::Error::msg("Validation failed"))
    } else {
        Ok(())
    }
}
