// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "migration-node-validation",
    about = "Validates data conformity after movement migration."
)]
pub struct Command {
    #[clap(long = "movement", help = "The path to the movement database.")]
    movement_db: PathBuf,
    #[clap(
        long = "movement-aptos",
        help = "The path to the movement Aptos database."
    )]
    movement_aptos_db: PathBuf,
}

impl Command {
    pub async fn run(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    Command::command().debug_assert()
}
