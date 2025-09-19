// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    checks::error::ValidationError,
    types::storage::{MovementAptosStorage, MovementStorage},
};
use aptos_types::{
    access_path::Path,
    account_config::{AccountResource, CoinStoreResourceUntyped},
    state_store::{
        state_key::{inner::StateKeyInner, StateKey},
        TStateView,
    },
};
use bytes::Bytes;
use move_core_types::{account_address::AccountAddress, language_storage::StructTag};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use tracing::{debug, info};

#[derive(Debug)]
pub enum FailedComparison {
    MissingStateValue(StateKey),
    NotMissingStateValue(StateKey),
    RawStateDiverge {
        state_key: StateKey,
        movement_value: Bytes,
        maptos_state_value: Bytes,
    },
    AccountDiverge {
        address: AccountAddress,
        movement_account: AccountResource,
        movement_aptos_account: AccountResource,
    },
    BalanceDiverge {
        address: AccountAddress,
        movement_balance: u64,
        movement_aptos_balance: u64,
    },
}

impl PartialEq for FailedComparison {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                FailedComparison::MissingStateValue(state_key),
                FailedComparison::MissingStateValue(other_state_key),
            )
            | (
                FailedComparison::NotMissingStateValue(state_key),
                FailedComparison::NotMissingStateValue(other_state_key),
            )
            | (
                FailedComparison::RawStateDiverge { state_key, .. },
                FailedComparison::RawStateDiverge {
                    state_key: other_state_key,
                    ..
                },
            ) => state_key == other_state_key,
            (
                FailedComparison::AccountDiverge { address, .. },
                FailedComparison::AccountDiverge {
                    address: other_address,
                    ..
                },
            )
            | (
                FailedComparison::BalanceDiverge { address, .. },
                FailedComparison::BalanceDiverge {
                    address: other_address,
                    ..
                },
            ) => address == other_address,
            _ => false,
        }
    }
}

impl Eq for FailedComparison {}

impl Ord for FailedComparison {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                FailedComparison::MissingStateValue(state_key),
                FailedComparison::MissingStateValue(other_state_key),
            )
            | (
                FailedComparison::NotMissingStateValue(state_key),
                FailedComparison::NotMissingStateValue(other_state_key),
            )
            | (
                FailedComparison::RawStateDiverge { state_key, .. },
                FailedComparison::RawStateDiverge {
                    state_key: other_state_key,
                    ..
                },
            ) => state_key.cmp(other_state_key),
            (
                FailedComparison::AccountDiverge { address, .. },
                FailedComparison::AccountDiverge {
                    address: other_address,
                    ..
                },
            )
            | (
                FailedComparison::BalanceDiverge { address, .. },
                FailedComparison::BalanceDiverge {
                    address: other_address,
                    ..
                },
            ) => address.cmp(other_address),
            (FailedComparison::MissingStateValue(_), _) => Ordering::Less,
            (FailedComparison::NotMissingStateValue(_), _) => Ordering::Less,
            (FailedComparison::RawStateDiverge { .. }, _) => Ordering::Less,
            (FailedComparison::AccountDiverge { .. }, _) => Ordering::Less,
            (FailedComparison::BalanceDiverge { .. }, _) => Ordering::Less,
        }
    }
}

impl PartialOrd for FailedComparison {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for FailedComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FailedComparison::MissingStateValue(movement_state_key) =>
                write!(f,
                    "Movement Aptos is missing a value for {:?}",
                    movement_state_key
                )
                   ,
            FailedComparison::NotMissingStateValue(movement_state_key) =>
                write!(f,
                    "Movement Aptos is unexpectedly not missing a value for {:?}",
                    movement_state_key
                ),
            FailedComparison::RawStateDiverge {
                state_key,
                movement_value,
                maptos_state_value,
            } =>
                write!(f,
                    "Movement state value for {:?} is {:?}, while Movement Aptos state value is {:?}",
                    state_key,
                    movement_value,
                    maptos_state_value
                ),
            FailedComparison::AccountDiverge {
                address,
                movement_account,
                movement_aptos_account,
            } =>
                write!(f,
                    "Movement account for {:?} is {:?}, while Movement Aptos account is {:?}",
                    address.to_standard_string(),
                    movement_account,
                    movement_aptos_account
                ),
            FailedComparison::BalanceDiverge {
                address,
                movement_balance,
                movement_aptos_balance,
            } =>
                write!(f,
                    "Movement balance for 0x{} is {} coin(s), while Movement Aptos balance is {} coin(s)",
                    address.short_str_lossless(),
                    movement_balance,
                    movement_aptos_balance
                ),
        }
    }
}

impl From<FailedComparison> for ValidationError {
    fn from(fail: FailedComparison) -> Self {
        ValidationError::Unsatisfied(fail.to_string().into())
    }
}

/// This check iterates over all global state keys starting at ledger version 0.
/// For each state key it fetches the state view for the latest ledger version,
/// from the old Movment database and the new Aptos database. The state view bytes
/// from both databases need to match. If the state key has no value in the latest
/// ledger version of the old Movement database then it should also have no value
/// in the new Aptos database.
/// Account Resources and Coin Stores are deserialized from BSC before comparison.
/// In case of Coin Stores, only the balances are compared.
pub struct GlobalStorageIncludes;

impl GlobalStorageIncludes {
    pub fn compare_db(
        movement_storage: &MovementStorage,
        mvt_version: u64,
        movement_aptos_storage: &MovementAptosStorage,
        aptos_version: u64,
    ) -> Result<BTreeSet<FailedComparison>, ValidationError> {
        info!("checking global state keys and values");
        debug!("movement_ledger_version: {:?}", mvt_version);
        debug!("aptos_ledger_version: {:?}", aptos_version);

        // get the latest state view from the movement storage
        let movement_state_view = movement_storage
            .state_view_at_version(Some(mvt_version))
            .map_err(|e| ValidationError::Internal(e.into()))?;

        // get the latest state view from the maptos storage
        let maptos_state_view = movement_aptos_storage
            .state_view_at_version(Some(aptos_version))
            .map_err(|e| ValidationError::Internal(e.into()))?;

        // the movement state view is the domain, so the maptos state view is the codomain
        let movement_global_state_keys_iterator =
            movement_storage.global_state_keys_from_version(None);
        let movement_global_state_keys = movement_global_state_keys_iterator
            .iter()
            .map_err(|e| ValidationError::Internal(e.into()))?;

        let mut count = 0;
        let account = StructTag::from_str("0x1::account::Account").unwrap();
        let coin = StructTag::from_str("0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>").unwrap();

        let mut failed_list = BTreeSet::new();

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
                    let maptos_state_value = match maptos_state_view
                        .get_state_value_bytes(&movement_state_key)
                        .map_err(|e| ValidationError::Internal(e.into()))?
                    {
                        Some(val) => val,
                        None => {
                            failed_list
                                .insert(FailedComparison::MissingStateValue(movement_state_key));
                            break;
                        },
                    };

                    if let StateKeyInner::AccessPath(p) = movement_state_key.inner() {
                        match p.get_path() {
                            Path::Resource(tag) if tag == account => {
                                if let Some(fail) = Self::compare_accounts(
                                    p.address,
                                    movement_value,
                                    maptos_state_value,
                                )? {
                                    failed_list.insert(fail);
                                }
                            },
                            Path::Resource(tag) if tag == coin => {
                                if let Some(fail) = Self::compare_balances(
                                    p.address,
                                    movement_value,
                                    maptos_state_value,
                                )? {
                                    failed_list.insert(fail);
                                }
                            },
                            _ => {
                                if let Some(fail) = Self::compare_raw_state(
                                    movement_state_key,
                                    movement_value,
                                    maptos_state_value,
                                ) {
                                    failed_list.insert(fail);
                                }
                            },
                        }
                    } else {
                        if let Some(fail) = Self::compare_raw_state(
                            movement_state_key,
                            movement_value,
                            maptos_state_value,
                        ) {
                            failed_list.insert(fail);
                        };
                    }
                },
                None => {
                    debug!("Value from a previous version has been removed at the latest ledger version");

                    match maptos_state_view
                        .get_state_value(&movement_state_key)
                        .map_err(|e| ValidationError::Internal(e.into()))?
                    {
                        Some(_) => {
                            failed_list
                                .insert(FailedComparison::NotMissingStateValue(movement_state_key));
                            break;
                        },
                        None => {},
                    }
                },
            }
            count += 1;
        }

        Ok(failed_list)
    }

    pub fn satisfies(
        movement_storage: &MovementStorage,
        movement_aptos_storage: &MovementAptosStorage,
    ) -> Result<(), ValidationError> {
        // get the latest ledger version from the movement storage
        let movement_ledger_version = movement_storage
            .latest_ledger_version()
            .map_err(|e| ValidationError::Internal(e.into()))?;

        let aptos_ledger_version = movement_aptos_storage
            .latest_ledger_version()
            .map_err(|e| ValidationError::Internal(e.into()))?;

        let mut failed_list = GlobalStorageIncludes::compare_db(
            movement_storage,
            movement_ledger_version,
            movement_aptos_storage,
            aptos_ledger_version,
        )?;

        match failed_list.pop_first() {
            None => Ok(()),
            Some(val) => Err(val.into()),
        }
    }

    fn compare_raw_state(
        state_key: StateKey,
        movement_value: Bytes,
        maptos_state_value: Bytes,
    ) -> Option<FailedComparison> {
        if movement_value != maptos_state_value {
            Some(FailedComparison::RawStateDiverge {
                state_key,
                movement_value,
                maptos_state_value,
            })
        } else {
            None
        }
    }

    fn compare_accounts(
        address: AccountAddress,
        movement_value: Bytes,
        maptos_state_value: Bytes,
    ) -> Result<Option<FailedComparison>, ValidationError> {
        let movement_account = bcs::from_bytes::<AccountResource>(&movement_value)
            .map_err(|e| ValidationError::Internal(e.into()))?;
        let movement_aptos_account = bcs::from_bytes::<AccountResource>(&maptos_state_value)
            .map_err(|e| ValidationError::Internal(e.into()))?;

        debug!(
            "movement account at 0x{}: {:?}",
            address.short_str_lossless(),
            movement_account
        );

        if movement_account != movement_aptos_account {
            Ok(Some(FailedComparison::AccountDiverge {
                address,
                movement_account,
                movement_aptos_account,
            }))
        } else {
            Ok(None)
        }
    }

    fn compare_balances(
        address: AccountAddress,
        movement_value: Bytes,
        maptos_state_value: Bytes,
    ) -> Result<Option<FailedComparison>, ValidationError> {
        let movement_balance = bcs::from_bytes::<CoinStoreResourceUntyped>(&movement_value)
            .map_err(|e| ValidationError::Internal(e.into()))?
            .coin();
        let movement_aptos_balance =
            bcs::from_bytes::<CoinStoreResourceUntyped>(&maptos_state_value)
                .map_err(|e| ValidationError::Internal(e.into()))?
                .coin();

        debug!(
            "movement balance at 0x{}: {} coins",
            address.short_str_lossless(),
            movement_balance
        );

        if movement_balance != movement_aptos_balance {
            Ok(Some(FailedComparison::BalanceDiverge {
                address,
                movement_balance,
                movement_aptos_balance,
            }))
        } else {
            Ok(None)
        }
    }
}
