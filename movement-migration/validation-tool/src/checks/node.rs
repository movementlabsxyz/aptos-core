// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::node::global_storage_includes::CompareStateView;
use crate::checks::node::state_diff::CompareStates;
use clap::Subcommand;

mod global_storage_includes;
mod state_diff;

#[derive(Subcommand, Debug)]
#[clap(rename_all = "kebab-case", about = "Node database verification tool")]
pub enum NodeValidation {
    CompareDb(CompareStateView),
    FindDiff(CompareStates),
}

impl NodeValidation {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            NodeValidation::CompareDb(cmd) => cmd.run().await,
            NodeValidation::FindDiff(cmd) => cmd.run().await,
        }
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    NodeValidation::command().debug_assert()
}
