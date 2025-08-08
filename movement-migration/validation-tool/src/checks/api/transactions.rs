// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::types::api::MovementAptosRestClient;
use clap::Parser;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tracing::info;

#[derive(Parser)]
#[clap(
    name = "transactions",
    about = "Gets a list of transactions from a validator node and stores them into a file"
)]
pub struct GetTransactions {
    #[clap(value_parser)]
    #[clap(long = "url", help = "The url of the Movement Aptos REST endpoint.")]
    pub rest_api_url: String,
    #[clap(long = "start", help = "The start ledger version")]
    pub start: u64,
    #[clap(long = "out", help = "Output path file name")]
    pub output_path: PathBuf,
}

impl GetTransactions {
    pub async fn run(self) -> anyhow::Result<()> {
        let rest_client = MovementAptosRestClient::new(&self.rest_api_url)?;
        get_transactions(&rest_client, self.start, self.output_path).await?;
        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    GetTransactions::command().debug_assert()
}

async fn get_transactions(
    rest_client: &MovementAptosRestClient,
    start: u64,
    output_path: impl AsRef<Path>,
) -> Result<(), anyhow::Error> {
    let response = rest_client.get_index_bcs().await?;
    let latest_ledger_version: u64 = response.into_inner().ledger_version.into();
    let mut current_ledger_version = start;
    let mut transaction_count = 0usize;
    let file = File::create(output_path).await?;
    let mut writer = BufWriter::new(file);

    info!("Latest ledger version is {}", latest_ledger_version);

    while current_ledger_version < latest_ledger_version {
        info!(
            "Getting transactions from version {}",
            current_ledger_version
        );
        let txs = rest_client
            .get_transactions_bcs(Some(current_ledger_version), Some(100))
            .await?
            .into_inner();

        if txs.is_empty() {
            info!("Transactions not found");
            break;
        }

        let mut user_transactions = 0usize;

        for tx in txs.iter() {
            if tx.transaction.try_as_signed_user_txn().is_some() {
                let bytes = bcs::to_bytes(tx)?;
                let str = format!("{}\n", hex::encode(&bytes));
                writer.write_all(str.as_bytes()).await?;
                user_transactions += 1;
            }
            current_ledger_version = tx.version;
        }

        transaction_count += user_transactions;
        info!(
            "Node returned {} transactions ({} signed user transactions)",
            txs.len(),
            user_transactions
        );
    }

    info!("Total transaction count is {}", transaction_count);

    writer.flush().await?;
    Ok(())
}
