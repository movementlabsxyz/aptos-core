// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use validation_tool::ValidationTool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    ValidationTool::parse().run().await
}
