// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "migration-api-validation",
    about = "Validates api conformity after movement migration."
)]
pub struct Command {
    #[clap(long = "movement", help = "The url of the Movement REST endpoint.")]
    movement_rest_api_url: String,
    #[clap(value_parser)]
    #[clap(
        long = "movement-aptos",
        help = "The url of the Movement Aptos REST endpoint."
    )]
    movement_aptos_rest_api_url: String,
}

impl Command {
    pub async fn run(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    crate::checks::node::Command::command().debug_assert()
}
