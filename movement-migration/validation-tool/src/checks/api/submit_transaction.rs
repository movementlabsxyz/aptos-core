// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::types::api::MovementAptosRestClient;
use anyhow::Context;
use aptos_rest_client::aptos_api_types::TransactionOnChainData;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::{
    account_address::AccountAddress,
    chain_id::ChainId,
    transaction::{EntryFunction, SignedTransaction, TransactionPayload},
    LocalAccount,
};
use clap::Parser;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, TypeTag};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

#[derive(Parser)]
#[clap(
    name = "submit-transaction",
    about = "Transfers funds from one account to another account"
)]
pub struct SubmitTransaction {
    #[clap(value_parser)]
    #[clap(long = "movement-url", help = "The url of the Movement REST endpoint.")]
    pub movement_rest_api_url: String,
    #[clap(value_parser)]
    #[clap(long = "aptos-url", help = "The url of the Aptos REST endpoint.")]
    pub aptos_rest_api_url: String,
    #[clap(value_parser)]
    #[clap(long = "receiver", help = "The receiver account address.")]
    pub receiver_address: String,
    #[clap(value_parser)]
    #[clap(long = "amount", help = "The amount of tokens to be sent.")]
    pub send_amount: u64,
}

impl SubmitTransaction {
    pub async fn run(self) -> anyhow::Result<()> {
        let private_key = std::env::var("MOVE_ACCOUNT_PRIVATE_KEY")
            .context("MOVE_ACCOUNT_PRIVATE_KEY variable is not set")?;
        let local_account = LocalAccount::from_private_key(&private_key, 0)?;
        let remote_account =
            AccountAddress::from_hex(self.receiver_address.trim_start_matches("0x"))?;

        let rest_client_movement = MovementAptosRestClient::new(&self.movement_rest_api_url)?;
        let rest_client_aptos = MovementAptosRestClient::new(&self.aptos_rest_api_url)?;
        let chain_id = check_chain_id(&rest_client_movement, &rest_client_aptos).await?;
        check_sequence_number(&rest_client_movement, &rest_client_aptos, &local_account).await?;
        check_balance(
            &rest_client_movement,
            &rest_client_aptos,
            local_account.address(),
            self.send_amount,
        )
        .await?;
        let transaction =
            create_transaction(&local_account, remote_account, self.send_amount, chain_id)?;
        // let json = serde_json::to_string_pretty(&transaction)?;
        // info!("Transaction created:\n{}", json);
        submit_transaction(&rest_client_movement, &rest_client_aptos, &transaction).await?;
        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    SubmitTransaction::command().debug_assert()
}

async fn check_chain_id(
    rest_client_movement: &MovementAptosRestClient,
    rest_client_aptos: &MovementAptosRestClient,
) -> anyhow::Result<ChainId> {
    let chain_id_movement = rest_client_movement
        .get_index_bcs()
        .await?
        .into_inner()
        .chain_id;
    let chain_id_aptos = rest_client_aptos
        .get_index_bcs()
        .await?
        .into_inner()
        .chain_id;

    if chain_id_aptos == chain_id_movement {
        info!("Chain-Id: {}", chain_id_movement);
        Ok(ChainId::new(chain_id_movement))
    } else {
        Err(anyhow::anyhow!(
            "Chain-Id mismatch. Movement chain-id: {}. Aptos chain-id: {}.",
            chain_id_movement,
            chain_id_aptos
        ))
    }
}

async fn check_sequence_number(
    rest_client_movement: &MovementAptosRestClient,
    rest_client_aptos: &MovementAptosRestClient,
    local_account: &LocalAccount,
) -> anyhow::Result<()> {
    let sequence_number_movement = rest_client_movement
        .get_account_bcs(local_account.address())
        .await
        .context(format!(
            "Can't get the Movement account for the address {}",
            local_account.address()
        ))?
        .into_inner()
        .sequence_number();
    let sequence_number_aptos = rest_client_aptos
        .get_account_bcs(local_account.address())
        .await
        .context(format!(
            "Can't get the Aptos account for the address {}",
            local_account.address()
        ))?
        .into_inner()
        .sequence_number();

    if sequence_number_movement == sequence_number_aptos {
        info!("Account sequence number: {}", sequence_number_movement);
        local_account.set_sequence_number(sequence_number_movement);
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Sequence number mismatch. Movement sequence number: {}. Aptos sequence number: {}.",
            sequence_number_movement,
            sequence_number_aptos
        ))
    }
}

async fn check_balance(
    rest_client_movement: &MovementAptosRestClient,
    rest_client_aptos: &MovementAptosRestClient,
    account: AccountAddress,
    amount: u64,
) -> anyhow::Result<()> {
    let balance_movement = rest_client_movement
        .view_apt_account_balance(account)
        .await
        .context(format!(
            "Can't get the Movement balance for address {}",
            account
        ))?
        .into_inner();
    let balance_aptos = rest_client_aptos
        .view_apt_account_balance(account)
        .await
        .context(format!(
            "Can't get the Aptos balance for address {}",
            account
        ))?
        .into_inner();

    if balance_movement == balance_aptos {
        info!("Account address: {}", account);
        info!("Account balance: {}", balance_movement);

        if amount <= balance_movement {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "The account balance is less than the amount to transfer"
            ))
        }
    } else {
        Err(anyhow::anyhow!(
            "Balance mismatch. Movement account balance: {}. Aptos account balance: {}.",
            balance_movement,
            balance_aptos
        ))
    }
}

fn create_transaction(
    from_account: &LocalAccount,
    to_account: AccountAddress,
    amount: u64,
    chain_id: ChainId,
) -> anyhow::Result<SignedTransaction> {
    info!(
        "Sending {} Octas from {} to {}",
        amount,
        from_account.address(),
        to_account
    );
    let coin_type = "0x1::aptos_coin::AptosCoin";
    let max_gas_amount = 5_000;
    let gas_unit_price = 100;
    let transaction_builder = TransactionBuilder::new(
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("coin")?),
            Identifier::new("transfer")?,
            vec![TypeTag::from_str(coin_type)?],
            vec![bcs::to_bytes(&to_account)?, bcs::to_bytes(&amount)?],
        )),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 60,
        chain_id,
    )
    .sender(from_account.address())
    .sequence_number(from_account.sequence_number())
    .max_gas_amount(max_gas_amount)
    .gas_unit_price(gas_unit_price);

    Ok(from_account.sign_with_transaction_builder(transaction_builder))
}

async fn submit_transaction(
    rest_client_movement: &MovementAptosRestClient,
    rest_client_aptos: &MovementAptosRestClient,
    transaction: &SignedTransaction,
) -> anyhow::Result<()> {
    // Send the transaction to the Aptos node first.
    // If the submission fails, we can fix the problem and reset the DB.
    let tx_on_chain_data_aptos = rest_client_aptos
        .submit_and_wait_bcs(transaction)
        .await
        .context("Failed to submit the transaction to Aptos")?
        .into_inner();

    log_tx_on_chain_data(&tx_on_chain_data_aptos, "Aptos")?;

    let tx_on_chain_data_movement = rest_client_movement
        .submit_and_wait_bcs(transaction)
        .await
        .context("Failed to submit the transaction to Movement")?
        .into_inner();

    log_tx_on_chain_data(&tx_on_chain_data_movement, "Movement")?;

    if tx_on_chain_data_movement == tx_on_chain_data_aptos {
        info!("Transaction on-chain data on Aptos equals the data on Movement");
    } else {
        warn!("Transaction on-chain data on Aptos is different than the data on Movement");
    }

    Ok(())
}

fn log_tx_on_chain_data(data: &TransactionOnChainData, name: &str) -> anyhow::Result<()> {
    let bytes = bcs::to_bytes(data)?;
    let str = hex::encode(&bytes);

    info!("Transaction on-chain data ({}):\n{}", name, str);
    info!(
        "Transaction info ({}):\n{}",
        name,
        serde_json::to_string_pretty(&data.info)?
    );

    Ok(())
}
