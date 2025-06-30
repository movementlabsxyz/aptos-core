// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

mod matching_feature_flags;

use crate::checks::api::matching_feature_flags::GlobalFeatureCheck;
use crate::types::api::{MovementAptosRestClient, MovementRestClient};
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
        let movement_rest_client = MovementRestClient::new(&self.movement_rest_api_url)?;
        let movement_aptos_rest_client =
            MovementAptosRestClient::new(&self.movement_aptos_rest_api_url)?;

        GlobalFeatureCheck::satisfies(&movement_rest_client, &movement_aptos_rest_client).await?;

        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    Command::command().debug_assert()
}
