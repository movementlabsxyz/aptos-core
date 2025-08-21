// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::api::active_feature_flags::GlobalFeatureCheck;
use crate::checks::api::cmp_transactions::CompareTransactions;
use crate::checks::api::submit_transaction::SubmitTransaction;
use crate::checks::api::transactions::GetTransactions;
use clap::Subcommand;

mod active_feature_flags;
mod cmp_transactions;
mod submit_transaction;
mod transactions;

#[derive(Subcommand)]
#[clap(
    name = "migration-api-tool",
    about = "Validates api conformity after movement migration"
)]
pub enum ApiTool {
    ActiveFeatures(GlobalFeatureCheck),
    Transactions(GetTransactions),
    CompareTransactions(CompareTransactions),
    SubmitTransaction(SubmitTransaction),
}

impl ApiTool {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            ApiTool::ActiveFeatures(tool) => tool.run().await,
            ApiTool::Transactions(tool) => tool.run().await,
            ApiTool::CompareTransactions(tool) => tool.run().await,
            ApiTool::SubmitTransaction(tool) => tool.run().await,
        }
    }
}
