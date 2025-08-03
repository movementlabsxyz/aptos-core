// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::error::ValidationError;
use crate::types::api::MovementAptosRestClient;
use crate::types::storage::MovementAptosStorage;
use aptos_transaction_emitter_lib::emitter::transaction_executor::RestApiReliableTransactionSubmitter;
use aptos_transaction_generator_lib::ReliableTransactionSubmitter;
use aptos_types::transaction::{Transaction, Version};
use itertools::Itertools;
use std::time::Duration;
use tracing::{error, info};

pub struct ReplayUnseenTx;

impl ReplayUnseenTx {
    pub async fn satisfies(
        movement_aptos_rest_client: &MovementAptosRestClient,
        movement_aptos_storage: &MovementAptosStorage,
        start_version: Version,
        batch_size: usize,
    ) -> Result<(), ValidationError> {
        info!(
            "replaying unseen transaction starting from ledger version {}",
            start_version
        );
        let rest_clients = vec![movement_aptos_rest_client.clone().into_inner()];
        let submitter =
            RestApiReliableTransactionSubmitter::new(rest_clients, 3, Duration::from_secs(1));

        let tx_iter = movement_aptos_storage
            .get_transaction_iterator(start_version + 1, 10_000)
            .map_err(|e| ValidationError::Internal(e.into()))?
            .filter_map(|txn| match txn {
                Ok(Transaction::UserTransaction(signed_txn)) => Some(Ok(signed_txn)),
                Ok(_) => None, // Skip non-user transactions
                Err(e) => Some(Err(ValidationError::Internal(e.into()))),
            })
            .chunks(batch_size);

        let mut batch_counter = 0u64;
        for tx_chunk in &tx_iter {
            batch_counter += 1;
            info!("processing unseen transaction batch {}", batch_counter);
            let chunk = tx_chunk.collect::<Vec<_>>();
            let signed_txn = chunk.into_iter().collect::<Result<Vec<_>, _>>()?;
            let counters = submitter.create_counter_state();

            if let Err(e) = submitter
                .execute_transactions_with_counter(&signed_txn, &counters)
                .await
            {
                error!("encountered error executing batch {}: {}", batch_counter, e);
            } else {
                info!("replayed {} unseen transactions", counters.show_simple());
            }
        }

        Ok(())
    }
}
