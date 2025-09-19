// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::node::balances::CompareBalances;
use crate::checks::node::global_storage_includes::CompareStateView;
use clap::Subcommand;

mod balances;
mod global_storage_includes;

#[derive(Subcommand, Debug)]
#[clap(rename_all = "kebab-case", about = "Node database verification tool")]
pub enum NodeValidation {
    CompareDb(CompareStateView),
    CompareBalances(CompareBalances),
}

impl NodeValidation {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            NodeValidation::CompareDb(cmd) => cmd.run().await,
            NodeValidation::CompareBalances(cmd) => cmd.run().await,
        }
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    NodeValidation::command().debug_assert()
}
