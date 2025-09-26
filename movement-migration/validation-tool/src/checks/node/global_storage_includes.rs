// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::types::storage::Storage;
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
use clap::Parser;
use move_core_types::{account_address::AccountAddress, language_storage::StructTag};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[clap(
    name = "compare-database",
    about = "Validates data conformity after movement migration."
)]
pub struct CompareDbCmd {
    #[clap(long = "movement", help = "The path to the movement database.")]
    pub movement_db: PathBuf,
    #[clap(
        long = "movement-aptos",
        help = "The path to the movement Aptos database."
    )]
    pub movement_aptos_db: PathBuf,
}

impl CompareDbCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        let movement_storage = MovementStorage::open(&self.movement_db)?;
        let movement_aptos_storage = MovementAptosStorage::open(&self.movement_aptos_db)?;

        GlobalStorageIncludes::satisfies(&movement_storage, &movement_aptos_storage)?;

        Ok(())
    }
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    CompareDbCmd::command().debug_assert()
}

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

impl FailedComparison {
    fn to_int(&self) -> u32 {
        match self {
            FailedComparison::MissingStateValue(_) => 4,
            FailedComparison::NotMissingStateValue(_) => 3,
            FailedComparison::RawStateDiverge { .. } => 2,
            FailedComparison::AccountDiverge { .. } => 1,
            FailedComparison::BalanceDiverge { .. } => 0,
        }
    }
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
                FailedComparison::AccountDiverge {
                    address,
                    movement_account,
                    ..
                },
                FailedComparison::AccountDiverge {
                    address: other_address,
                    movement_account: other_movement_account,
                    ..
                },
            ) => address == other_address && movement_account == other_movement_account,
            (
                FailedComparison::BalanceDiverge {
                    address,
                    movement_balance,
                    ..
                },
                FailedComparison::BalanceDiverge {
                    address: other_address,
                    movement_balance: other_movement_balance,
                    ..
                },
            ) => address == other_address && movement_balance == other_movement_balance,
            (val1, val2) => val1.to_int() == val2.to_int(),
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
            ) => address.cmp(other_address),
            (
                FailedComparison::BalanceDiverge {
                    address,
                    movement_balance,
                    ..
                },
                FailedComparison::BalanceDiverge {
                    address: other_address,
                    movement_balance: other_movement_balance,
                    ..
                },
            ) => {
                if address == other_address {
                    movement_balance.cmp(other_movement_balance)
                } else {
                    address.cmp(other_address)
                }
            },
            (val1, val2) => val1.to_int().cmp(&val2.to_int()),
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
            FailedComparison::MissingStateValue(state_key) =>
                write!(f,
                    "Movement Aptos is missing a value for {:?}",
                    state_key
                )
                   ,
            FailedComparison::NotMissingStateValue(state_key) =>
                write!(f,
                    "Movement Aptos is unexpectedly not missing a value for {:?}",
                    state_key
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
        aptos_storage: &MovementAptosStorage,
        aptos_version: u64,
    ) -> Result<BTreeSet<FailedComparison>, ValidationError> {
        let mut res1 =
            Self::compare_db_one_way(movement_storage, mvt_version, aptos_storage, aptos_version)?;
        let mut res2 =
            Self::compare_db_one_way(aptos_storage, aptos_version, movement_storage, mvt_version)?;

        res1.append(&mut res2);
        Ok(res1)
    }

    pub fn compare_db_one_way(
        base_storage: &Storage,
        base_version: u64,
        other_storage: &Storage,
        other_version: u64,
    ) -> Result<BTreeSet<FailedComparison>, ValidationError> {
        info!("checking global state keys and values");
        debug!("movement_ledger_version: {:?}", base_version);
        debug!("aptos_ledger_version: {:?}", other_version);

        // get the latest state view from the movement storage
        let base_state_view = base_storage
            .state_view_at_version(Some(base_version))
            .map_err(|e| ValidationError::Internal(e.into()))?;

        // get the latest state view from the maptos storage
        let other_state_view = other_storage
            .state_view_at_version(Some(other_version))
            .map_err(|e| ValidationError::Internal(e.into()))?;

        // the movement state view is the domain, so the maptos state view is the codomain
        let base_global_state_keys_iterator = base_storage.global_state_keys_from_version(None);
        let base_global_state_keys = base_global_state_keys_iterator
            .iter()
            .map_err(|e| ValidationError::Internal(e.into()))?;

        let mut count = 0;
        let account = StructTag::from_str("0x1::account::Account").unwrap();
        let coin = StructTag::from_str("0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>").unwrap();

        let mut failed_list = BTreeSet::new();

        for new_state_key in base_global_state_keys {
            debug!("processing new_state_key {}: {:?}", count, new_state_key);

            let new_state_key = new_state_key.map_err(|e| ValidationError::Internal(e.into()))?;

            let base_value = base_state_view
                .get_state_value_bytes(&new_state_key)
                .map_err(|e| ValidationError::Internal(e.into()))?;

            // info!(
            //     "############################ processing movement_state_key {}: {:?} \n {:?}",
            //     count, new_state_key, base_value
            // );

            match base_value {
                Some(base_value) => {
                    let other_state_value = match other_state_view
                        .get_state_value_bytes(&new_state_key)
                        .map_err(|e| ValidationError::Internal(e.into()))?
                    {
                        Some(val) => val,
                        None => {
                            failed_list.insert(FailedComparison::MissingStateValue(new_state_key));
                            break;
                        },
                    };

                    if let StateKeyInner::AccessPath(p) = new_state_key.inner() {
                        match p.get_path() {
                            Path::Resource(tag) if tag == account => {
                                if let Some(fail) = Self::compare_accounts(
                                    p.address,
                                    base_value,
                                    other_state_value,
                                )? {
                                    failed_list.insert(fail);
                                }
                            },
                            Path::Resource(tag) if tag == coin => {
                                if let Some(fail) = Self::compare_balances(
                                    p.address,
                                    base_value,
                                    other_state_value,
                                )? {
                                    failed_list.insert(fail);
                                }
                            },
                            _ => {
                                if let Some(fail) = Self::compare_raw_state(
                                    new_state_key,
                                    base_value,
                                    other_state_value,
                                ) {
                                    failed_list.insert(fail);
                                }
                            },
                        }
                    } else {
                        if let Some(fail) =
                            Self::compare_raw_state(new_state_key, base_value, other_state_value)
                        {
                            failed_list.insert(fail);
                        };
                    }
                },
                None => {
                    debug!("Value from a previous version has been removed at the latest ledger version");

                    match other_state_view
                        .get_state_value(&new_state_key)
                        .map_err(|e| ValidationError::Internal(e.into()))?
                    {
                        Some(_) => {
                            failed_list
                                .insert(FailedComparison::NotMissingStateValue(new_state_key));
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
        debug!("compare_raw_state");
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
            "compare_accounts movement account at 0x{}: {:?}, aptos:{:?}",
            address.short_str_lossless(),
            movement_account,
            movement_aptos_account
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
            "compare_balances movement balance at 0x{}: {} coins vs {}",
            address.short_str_lossless(),
            movement_balance,
            movement_aptos_balance
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
