// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::replay::replay_unseen_tx::ReplayUnseenTx;
use crate::types::api::MovementAptosRestClient;
use crate::types::storage::MovementAptosStorage;
use aptos_types::transaction::Version;
use clap::Parser;
use std::path::PathBuf;

mod replay_unseen_tx;

#[derive(Parser)]
#[clap(
    name = "migration-replay-transactions",
    about = "Replay unseen transactions"
)]
pub struct Command {
    #[clap(value_parser)]
    #[clap(long = "url", help = "The url of the Movement Aptos REST endpoint.")]
    pub movement_aptos_rest_api_url: String,
    #[clap(long = "db", help = "The path to the movement Aptos database.")]
    pub movement_aptos_db: PathBuf,
    #[clap(long = "start", help = "The ledger start version")]
    pub start_version: Version,
    #[clap(
        long = "batch-size",
        help = "The size of transaction batches send to the API",
        default_value_t = 1000
    )]
    pub batch_size: usize,
}

impl Command {
    pub async fn run(self) -> anyhow::Result<()> {
        let movement_aptos_rest_client =
            MovementAptosRestClient::new(&self.movement_aptos_rest_api_url)?;
        let movement_aptos_storage = MovementAptosStorage::open(&self.movement_aptos_db)?;

        ReplayUnseenTx::satisfies(
            &movement_aptos_rest_client,
            &movement_aptos_storage,
            self.start_version,
            self.batch_size,
        )
        .await?;

        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    crate::checks::node::Command::command().debug_assert()
}
