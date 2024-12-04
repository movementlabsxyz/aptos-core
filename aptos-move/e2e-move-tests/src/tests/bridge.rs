use bcs;
use std::collections::BTreeMap;
use std::str::FromStr;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::transaction_argument::TransactionArgument;
use move_core_types::value::MoveValue;
use crate::{assert_abort, assert_success, MoveHarness};
use crate::tests::common;
use tiny_keccak::{Hasher, Keccak};


pub static BRIDGE_SCRIPTS: Lazy<BTreeMap<String, Vec<u8>>> = Lazy::new(build_scripts);
fn build_scripts() -> BTreeMap<String, Vec<u8>> {
    let package_folder = "bridge.data";
    let package_names = vec![
        "update_operator",
        "update_initiator_time_lock",
        "update_counterparty_time_lock",
        "mint_burn_caps",
        "mint_burn_caps_native",
        "atomic_bridge_feature",
        "native_bridge_feature"
    ];
    common::build_scripts(package_folder, package_names)
}

fn keccak256(to_be_hashed: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak::v256();
    hasher.update(to_be_hashed);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output.into()
}

#[test]
// The bridge can only be initialised by @aptos_framework
// The operator can only be updated by @aptos_framework
fn test_bridge_operator() {
    let mut harness = MoveHarness::new();
    let core_resources =
        harness.new_account_at(AccountAddress::from_hex_literal("0xA550C18").unwrap());

    let new_operator = harness.new_account_at(AccountAddress::from_hex_literal("0xCAFE").unwrap());
    let update_operator_script_code = BRIDGE_SCRIPTS
        .get("update_operator")
        .expect("bridge script should be built");

    let txn = harness.create_script(
        &core_resources,
        update_operator_script_code.clone(),
        vec![],
        vec![TransactionArgument::Address(*new_operator.address())]
    );

    assert_success!(harness.run(txn));
    let bytes = harness.execute_view_function(str::parse("0x1::atomic_bridge_configuration::bridge_operator").unwrap(),
                                  vec![], vec![]).values
        .unwrap()
        .pop()
        .unwrap();

    let bridge_operator = bcs::from_bytes::<AccountAddress>(bytes.as_slice()).unwrap();
    assert_eq!(*new_operator.address(), bridge_operator);

    let false_operator = harness.new_account_at(AccountAddress::from_hex_literal("0xFA").unwrap());
    let txn = harness.create_script(
        &false_operator,
        update_operator_script_code.clone(),
        vec![],
        vec![TransactionArgument::Address(*false_operator.address())]
    );

    assert_abort!(harness.run(txn), _);

    // Just confirm its the same as before till work out the above
    let bytes = harness.execute_view_function(str::parse("0x1::atomic_bridge_configuration::bridge_operator").unwrap(),
                                              vec![], vec![]).values
        .unwrap()
        .pop()
        .unwrap();

    let bridge_operator = bcs::from_bytes::<AccountAddress>(bytes.as_slice()).unwrap();
    assert_eq!(*new_operator.address(), bridge_operator);
}

#[cfg(test)]
fn run_mint_burn_caps(harness: &mut MoveHarness) {
    let core_resources = harness.new_account_at(AccountAddress::from_hex_literal("0xA550C18").unwrap());
    let mint_burn_caps_code = BRIDGE_SCRIPTS
        .get("mint_burn_caps")
        .expect("mint_burn_caps script should be built");

    let txn = harness.create_script(
        &core_resources,
        mint_burn_caps_code.clone(),
        vec![],
        vec![]
    );

    assert_success!(harness.run(txn));
}

#[cfg(test)]
fn run_mint_burn_caps_native(harness: &mut MoveHarness) {
    let core_resources = harness.new_account_at(AccountAddress::from_hex_literal("0xA550C18").unwrap());
    let mint_burn_caps_code_native = BRIDGE_SCRIPTS
        .get("mint_burn_caps_native")
        .expect("mint_burn_caps_native script should be built");

    let txn = harness.create_script(
        &core_resources,
        mint_burn_caps_code_native.clone(),
        vec![],
        vec![]
    );

    assert_success!(harness.run(txn));
}

#[cfg(test)]
fn atomic_bridge_feature(harness: &mut MoveHarness) {
    let core_resources = harness.new_account_at(AccountAddress::from_hex_literal("0xA550C18").unwrap());
    let atomic_bridge_feature_code = BRIDGE_SCRIPTS
        .get("atomic_bridge_feature")
        .expect("atomic_bridge_feature script should be built");

    let txn = harness.create_script(
        &core_resources,
        atomic_bridge_feature_code.clone(),
        vec![],
        vec![]
    );

    assert_success!(harness.run(txn));
}

#[cfg(test)]
fn native_bridge_feature(harness: &mut MoveHarness) {
    let core_resources = harness.new_account_at(AccountAddress::from_hex_literal("0xA550C18").unwrap());
    let native_bridge_feature_code = BRIDGE_SCRIPTS
        .get("native_bridge_feature")
        .expect("native_bridge_feature script should be built");

    let txn = harness.create_script(
        &core_resources,
        native_bridge_feature_code.clone(),
        vec![],
        vec![]
    );

    assert_success!(harness.run(txn));
}

#[test]
// The relayer has received a message from the source chain of a successful lock
// `lock_bridge_transfer_assets` is called with a timelock
// Wait for the timelock
// `complete_bridge_transfer` to mint the tokens on the destination chain
fn test_atomic_bridge_counterparty() {
    let mut harness = MoveHarness::new();

    atomic_bridge_feature(&mut harness);
    run_mint_burn_caps(&mut harness);

    let bridge_operator = harness.aptos_framework_account();
    let initiator = b"32Be343B94f860124dC4fEe278FDCBD38C102D88".to_vec();
    let pre_image = b"my secret";
    let time_lock = 1;
    let amount = 42;
    let recipient = harness.new_account_at(AccountAddress::from_hex_literal("0xCAFE").unwrap());
    let bridge_transfer_id = keccak256(b"bridge_transfer_id");
    let hash_lock = keccak256(pre_image);

    let original_balance = harness.read_aptos_balance(recipient.address());

    assert_success!(harness.run_entry_function(&bridge_operator,
                               str::parse("0x1::atomic_bridge_counterparty::lock_bridge_transfer_assets").unwrap(),
                               vec![],
                               vec![
                                   MoveValue::vector_u8(initiator).simple_serialize().unwrap(),
                                   MoveValue::vector_u8(bridge_transfer_id.clone()).simple_serialize().unwrap(),
                                   MoveValue::vector_u8(hash_lock).simple_serialize().unwrap(),
                                    MoveValue::Address(*recipient.address()).simple_serialize().unwrap(),
                                    MoveValue::U64(amount).simple_serialize().unwrap(),
                               ],));

    harness.fast_forward(time_lock + 1);

    assert_success!(harness.run_entry_function(&bridge_operator,
                               str::parse("0x1::atomic_bridge_counterparty::complete_bridge_transfer").unwrap(),
                               vec![],
                               vec![
                                   MoveValue::vector_u8(bridge_transfer_id.clone()).simple_serialize().unwrap(),
                                   MoveValue::vector_u8(pre_image.to_vec()).simple_serialize().unwrap(),
                               ],));
    let new_balance = harness.read_aptos_balance(recipient.address());
    assert_eq!(original_balance + amount, new_balance);
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BridgeTransferInitiatedEvent {
    bridge_transfer_id: Vec<u8>,
    initiator: AccountAddress,
    recipient: Vec<u8>,
    amount: u64,
    hash_lock: Vec<u8>,
    time_lock: u64,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct NativeBridgeTransferInitiatedEvent {
    bridge_transfer_id: Vec<u8>,
    initiator: AccountAddress,
    recipient: Vec<u8>,
    amount: u64,
    nonce: u64,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct NativeBridgeTransferCompletedEvent {
    bridge_transfer_id: Vec<u8>,
    initiator: Vec<u8>,
    recipient: AccountAddress,
    amount: u64,
    nonce: u64,
}

fn normalize_to_32_bytes(value: Vec<u8>) -> Vec<u8> {
    let mut meaningful = Vec::new();
    let mut i = 0;

    // Remove trailing zeroes
    while i < value.len() {
        if value[i] != 0 {
            meaningful.push(value[i]);
        }
        i += 1;
    }

    let mut result = Vec::with_capacity(32);
    let padding_length = 32 - meaningful.len();

    // Pad with zeros on the left
    for _ in 0..padding_length {
        result.push(0);
    }

    // Append the meaningful bytes
    result.extend_from_slice(&meaningful);

    result
}

#[test]
// A bridge is initiated with said amount to recipient on the destination chain
// A relayer confirms that the initiate bridge transfer is successful and validates the details
fn test_native_bridge_initiate() {
    let mut harness = MoveHarness::new();

    native_bridge_feature(&mut harness);
    run_mint_burn_caps_native(&mut harness);

    let initiator = harness.new_account_at(AccountAddress::from_hex_literal("0x726563697069656e740000000000000000000000000000000000000000000000").unwrap());
    let recipient = b"5B38Da6a701c568545dCfcB03FcB875f56beddC4".to_vec();
    let amount = 100; // 0.1

    let original_balance = harness.read_aptos_balance(initiator.address());
    let gas_used = harness.evaluate_entry_function_gas(&initiator,
                                str::parse("0x1::native_bridge::initiate_bridge_transfer").unwrap(),
                                vec![],
                                vec![
                                    MoveValue::vector_u8(recipient.clone()).simple_serialize().unwrap(),
                                    MoveValue::U64(amount).simple_serialize().unwrap(),
                                ],);

    let gas_used = gas_used * harness.default_gas_unit_price;
    let new_balance = harness.read_aptos_balance(initiator.address());
    assert_eq!(original_balance - amount - gas_used, new_balance);

    let events = harness.get_events();
    let bridge_transfer_initiated_event_tag = TypeTag::from_str("0x1::native_bridge::BridgeTransferInitiatedEvent").unwrap();
    let bridge_transfer_initiated_event = events.iter().find(|element| element.type_tag() == &bridge_transfer_initiated_event_tag).unwrap();
    let bridge_transfer_initiated_event = bcs::from_bytes::<NativeBridgeTransferInitiatedEvent>(bridge_transfer_initiated_event.event_data()).unwrap();

    let bridge_transfer_id = bridge_transfer_initiated_event.bridge_transfer_id;
    let initiator = bridge_transfer_initiated_event.initiator;
    let recipient = bridge_transfer_initiated_event.recipient;
    let amount = bridge_transfer_initiated_event.amount;
    let nonce = bridge_transfer_initiated_event.nonce;

    let mut combined_bytes = Vec::new();

    // Append serialized values to `combined_bytes` in the same way as in the Move function
    // Serialize initiator as BCS bytes
    combined_bytes.extend(bcs::to_bytes(&initiator).expect("Failed to serialize initiator"));

    // Convert recipient from hex to bytes
    let recipient_bytes = hex::decode(String::from_utf8(recipient).expect("Invalid UTF-8 recipient"))
    .expect("Failed to decode recipient hex");
    combined_bytes.extend(recipient_bytes);

    // Pad amount and nonce to 32 bytes
    let amount_bytes = normalize_to_32_bytes(bcs::to_bytes(&amount).expect("Failed to serialize amount"));
    let nonce_bytes = normalize_to_32_bytes(bcs::to_bytes(&nonce).expect("Failed to serialize nonce"));
    combined_bytes.extend(amount_bytes);
    combined_bytes.extend(nonce_bytes);

    // Compute keccak256 hash using tiny-keccak
    let mut hasher = Keccak::v256();
    hasher.update(&combined_bytes);

    let mut hash = [0u8; 32]; // Keccak256 outputs 32 bytes
    hasher.finalize(&mut hash);

    // Compare the computed hash to `bridge_transfer_id`
    assert!(bridge_transfer_id == hash.to_vec());
}

#[test]
// A bridge is initiated with said amount to recipient on the destination chain
// A relayer confirms that the initiate bridge transfer is successful and validates the details
fn test_native_bridge_complete() {
    let mut harness = MoveHarness::new();

    native_bridge_feature(&mut harness);
    run_mint_burn_caps_native(&mut harness);

    let relayer = harness.new_account_at(AccountAddress::from_hex_literal("0x1").unwrap());

    let initiator = b"5B38Da6a701c568545dCfcB03FcB875f56beddC4".to_vec();
    let recipient = harness.new_account_at(AccountAddress::from_hex_literal("0x726563697069656e740000000000000000000000000000000000000000000000").unwrap());
    let amount = 100; // 0.1
    let nonce = 1;

    let mut combined_bytes = Vec::new();

    // Append serialized values to `combined_bytes`
    
    // Convert recipient from hex to bytes
    let initiator_bytes = hex::decode(String::from_utf8(initiator.clone()).expect("Invalid UTF-8 recipient"))
    .expect("Failed to decode recipient hex");
    combined_bytes.extend(initiator_bytes);
    combined_bytes.extend(bcs::to_bytes(&recipient.address()).expect("Failed to serialize recipient"));
    combined_bytes.extend(normalize_to_32_bytes(bcs::to_bytes(&amount).expect("Failed to serialize amount")));
    combined_bytes.extend(normalize_to_32_bytes(bcs::to_bytes(&nonce).expect("Failed to serialize nonce")));
    // Compute keccak256 hash using tiny-keccak
    let mut hasher = Keccak::v256();
    hasher.update(&combined_bytes);

    let mut hash = [0u8; 32]; // Keccak256 outputs 32 bytes
    hasher.finalize(&mut hash);

    // Compare the computed hash to `bridge_transfer_id`
    let original_balance = harness.read_aptos_balance(relayer.address());
    let gas_used = harness.evaluate_entry_function_gas(&relayer,
                                str::parse("0x1::native_bridge::complete_bridge_transfer").unwrap(),
                                vec![],
                                vec![
                                    MoveValue::vector_u8(hash.to_vec()).simple_serialize().unwrap(),
                                    MoveValue::vector_u8(initiator.clone()).simple_serialize().unwrap(),
                                    MoveValue::Address(*recipient.address()).simple_serialize().unwrap(),
                                    MoveValue::U64(amount).simple_serialize().unwrap(),
                                    MoveValue::U64(nonce).simple_serialize().unwrap(),
                                ],);

    let gas_used = gas_used * harness.default_gas_unit_price;
    let new_balance = harness.read_aptos_balance(relayer.address());
    assert_eq!(original_balance - gas_used, new_balance);

    let events = harness.get_events();
    let bridge_transfer_completed_event_tag = TypeTag::from_str("0x1::native_bridge::BridgeTransferCompletedEvent").unwrap();
    let bridge_transfer_completed_event = events.iter().find(|element| element.type_tag() == &bridge_transfer_completed_event_tag).unwrap();
    let bridge_transfer_completed_event = bcs::from_bytes::<NativeBridgeTransferCompletedEvent>(bridge_transfer_completed_event.event_data()).unwrap();
 
    let bridge_transfer_id = bridge_transfer_completed_event.bridge_transfer_id;
 
    assert_eq!(bridge_transfer_id, hash.to_vec());

}

#[test]
// A bridge is initiated with said amount to recipient on the destination chain
// A relayer confirms that the amount was minted on the destination chain
fn test_atomic_bridge_initiator() {
    let mut harness = MoveHarness::new();

    atomic_bridge_feature(&mut harness);
    run_mint_burn_caps(&mut harness);

    let bridge_operator = harness.aptos_framework_account();

    let recipient = b"32Be343B94f860124dC4fEe278FDCBD38C102D88".to_vec();
    let initiator = harness.new_account_at(AccountAddress::from_hex_literal("0xCAFE").unwrap());
    let pre_image = b"my secret";
    let amount = 1_000_000; // 0.1
    let hash_lock = keccak256(pre_image);

    let original_balance = harness.read_aptos_balance(initiator.address());
    let gas_used = harness.evaluate_entry_function_gas(&initiator,
                                str::parse("0x1::atomic_bridge_initiator::initiate_bridge_transfer").unwrap(),
                                vec![],
                                vec![
                                    MoveValue::vector_u8(recipient.clone()).simple_serialize().unwrap(),
                                    MoveValue::vector_u8(hash_lock.clone()).simple_serialize().unwrap(),
                                    MoveValue::U64(amount).simple_serialize().unwrap(),
                                ],);

    let gas_used = gas_used * harness.default_gas_unit_price;
    let new_balance = harness.read_aptos_balance(initiator.address());
    assert_eq!(original_balance - amount - gas_used, new_balance);

    let events = harness.get_events();
    let bridge_transfer_initiated_event_tag = TypeTag::from_str("0x1::atomic_bridge_initiator::BridgeTransferInitiatedEvent").unwrap();
    let bridge_transfer_initiated_event = events.iter().find(|element| element.type_tag() == &bridge_transfer_initiated_event_tag).unwrap();
    let bridge_transfer_initiated_event = bcs::from_bytes::<BridgeTransferInitiatedEvent>(bridge_transfer_initiated_event.event_data()).unwrap();
    let bridge_transfer_id = bridge_transfer_initiated_event.bridge_transfer_id;

    assert_success!(harness.run_entry_function(&bridge_operator,
                               str::parse("0x1::atomic_bridge_initiator::complete_bridge_transfer").unwrap(),
                               vec![],
                               vec![
                                   MoveValue::vector_u8(bridge_transfer_id.clone()).simple_serialize().unwrap(),
                                   MoveValue::vector_u8(pre_image.to_vec()).simple_serialize().unwrap(),
                               ],));
}

#[test]
// Update the initiator time lock duration
fn test_update_initiator_time_lock() {
    let mut harness = MoveHarness::new();
    let core_resources =
        harness.new_account_at(AccountAddress::from_hex_literal("0xA550C18").unwrap());

    let update_initiator_time_lock_code = BRIDGE_SCRIPTS
        .get("update_initiator_time_lock")
        .expect("bridge script should be built");

    let new_time_lock = 42;
    let txn = harness.create_script(
        &core_resources,
        update_initiator_time_lock_code.clone(),
        vec![],
        vec![TransactionArgument::U64(new_time_lock)]
    );

    assert_success!(harness.run(txn));

    let res = harness.execute_view_function(
                                    str::parse("0x1::atomic_bridge_configuration::initiator_timelock_duration").unwrap(),
                                    vec![],
                                    vec![]);
    let bcs = res.values.unwrap().pop().unwrap();
    let res = bcs::from_bytes::<u64>(&bcs).unwrap();
    assert_eq!(res, new_time_lock);

    let imposter = harness.new_account_at(AccountAddress::from_hex_literal("0xFA").unwrap());
    let txn = harness.create_script(
        &imposter,
        update_initiator_time_lock_code.clone(),
        vec![],
        vec![TransactionArgument::U64(new_time_lock)]
    );

    assert_abort!(harness.run(txn), _);
}

#[test]
// Update the initiator time lock duration
fn test_update_counterparty_time_lock() {
    let mut harness = MoveHarness::new();
    let core_resources =
        harness.new_account_at(AccountAddress::from_hex_literal("0xA550C18").unwrap());

    let update_counterparty_time_lock_code = BRIDGE_SCRIPTS
        .get("update_counterparty_time_lock")
        .expect("bridge script should be built");

    let new_time_lock = 42;
    let txn = harness.create_script(
        &core_resources,
        update_counterparty_time_lock_code.clone(),
        vec![],
        vec![TransactionArgument::U64(new_time_lock)]
    );

    assert_success!(harness.run(txn));

    let res = harness.execute_view_function(
        str::parse("0x1::atomic_bridge_configuration::counterparty_timelock_duration").unwrap(),
        vec![],
        vec![]);
    let bcs = res.values.unwrap().pop().unwrap();
    let res = bcs::from_bytes::<u64>(&bcs).unwrap();
    assert_eq!(res, new_time_lock);

    let imposter = harness.new_account_at(AccountAddress::from_hex_literal("0xFA").unwrap());
    let txn = harness.create_script(
        &imposter,
        update_counterparty_time_lock_code.clone(),
        vec![],
        vec![TransactionArgument::U64(new_time_lock)]
    );

    assert_abort!(harness.run(txn), _);
}

