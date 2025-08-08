// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::api::active_feature_flags::GlobalFeatureCheck;
use crate::checks::api::transactions::GetTransactions;
use clap::Subcommand;

mod active_feature_flags;
mod transactions;

#[derive(Subcommand)]
#[clap(
    name = "migration-api-tool",
    about = "Validates api conformity after movement migration"
)]
pub enum ApiTool {
    ActiveFeatures(GlobalFeatureCheck),
    Transactions(GetTransactions),
}

impl ApiTool {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            ApiTool::ActiveFeatures(tool) => tool.run().await,
            ApiTool::Transactions(tool) => tool.run().await,
        }
    }
}
