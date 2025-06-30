// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::checks::error::ValidationError;
use crate::types::storage::{MovementAptosStorage, MovementStorage};
use aptos_types::state_store::TStateView;
use tracing::debug;

pub struct GlobalStorageIncludes;

impl GlobalStorageIncludes {
    pub fn satisfies(
        movement_storage: &MovementStorage,
        movement_aptos_storage: &MovementAptosStorage,
    ) -> Result<(), ValidationError> {
        // get the latest ledger version from the movement storage
        let movement_ledger_version = movement_storage
            .latest_ledger_version()
            .map_err(|e| ValidationError::Internal(e.into()))?;

        debug!("movement_ledger_version: {:?}", movement_ledger_version);

        // get the latest state view from the movement storage
        let movement_state_view = movement_storage
            .state_view_at_version(Some(movement_ledger_version))
            .map_err(|e| ValidationError::Internal(e.into()))?;

        // get the latest state view from the maptos storage
        let maptos_state_view = movement_aptos_storage
            .state_view_at_version(Some(movement_ledger_version))
            .map_err(|e| ValidationError::Internal(e.into()))?;

        // the movement state view is the domain, so the maptos state view is the codomain
        let movement_global_state_keys_iterator =
            movement_storage.global_state_keys_from_version(None);
        let movement_global_state_keys = movement_global_state_keys_iterator
            .iter()
            .map_err(|e| ValidationError::Internal(e.into()))?;

        let mut count = 0;
        for movement_state_key in movement_global_state_keys {
            debug!(
                "processing movement_state_key {}: {:?}",
                count, movement_state_key
            );

            let movement_state_key =
                movement_state_key.map_err(|e| ValidationError::Internal(e.into()))?;

            let movement_value = movement_state_view
                .get_state_value_bytes(&movement_state_key)
                .map_err(|e| ValidationError::Internal(e.into()))?;

            match movement_value {
                Some(movement_value) => {
                    let maptos_state_value = maptos_state_view
                        .get_state_value_bytes(&movement_state_key)
                        .map_err(|e| ValidationError::Internal(e.into()))?
                        .ok_or(ValidationError::Unsatisfied(
                            format!(
                                "Movement Aptos is missing a value for {:?}",
                                movement_state_key
                            )
                            .into(),
                        ))?;

                    if movement_value != maptos_state_value {
                        return Err(ValidationError::Unsatisfied(
                            format!(
                                "Movement state value for {:?} is {:?}, while Movement Aptos state value is {:?}",
                                movement_state_key,
                                movement_value,
                                maptos_state_value
                            )
                                .into(),
                        ));
                    }
                },
                None => {
                    debug!("Value from a previous version has been removed at the latest ledger version");

                    match maptos_state_view
                        .get_state_value(&movement_state_key)
                        .map_err(|e| ValidationError::Internal(e.into()))?
                    {
                        Some(_) => {
                            return Err(ValidationError::Unsatisfied(
                                format!(
                                    "Movement Aptos is unexpectedly not missing a value for {:?}",
                                    movement_state_key
                                )
                                .into(),
                            ));
                        },
                        None => {},
                    }
                },
            }
            count += 1;
        }

        Ok(())
    }
}
