// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::smoke_test_environment::SwarmBuilder;
use movement::{common::types::GasOptions, test::CliTestFramework};
use aptos_cached_packages::aptos_stdlib;
use aptos_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    poseidon_bn254::keyless::fr_to_bytes_le,
    SigningKey, Uniform,
};
use aptos_forge::{AptosPublicInfo, LocalSwarm, NodeExt, Swarm, SwarmExt};
use aptos_logger::{debug, info};
use aptos_rest_client::Client;
use aptos_types::{
    jwks::{
        jwk::{JWKMoveStruct, JWK},
        rsa::RSA_JWK,
        secure_test_rsa_jwk, AllProvidersJWKs, PatchedJWKs, ProviderJWKs,
    },
    keyless::{
        get_public_inputs_hash,
        test_utils::{
            self, get_groth16_sig_and_pk_for_upgraded_vk, get_sample_esk,
            get_sample_groth16_sig_and_pk, get_sample_groth16_sig_and_pk_no_extra_field,
            get_sample_iss, get_sample_jwk, get_sample_openid_sig_and_pk, get_upgraded_vk,
        },
        Configuration, EphemeralCertificate, Groth16ProofAndStatement, Groth16VerificationKey,
        KeylessPublicKey, KeylessSignature, TransactionAndProof, DEVNET_VERIFICATION_KEY,
        KEYLESS_ACCOUNT_MODULE_NAME,
    },
    transaction::{
        authenticator::{
            AccountAuthenticator, AnyPublicKey, AnySignature, AuthenticationKey,
            EphemeralSignature, TransactionAuthenticator,
        },
        SignedTransaction,
    },
};
use move_core_types::account_address::AccountAddress;
use rand::thread_rng;
use serde::de::DeserializeOwned;
use std::{fmt::Debug, time::Duration};
// TODO(keyless): Test the override aud_val path

#[tokio::test]
async fn test_keyless_oidc_txn_verifies() {
    let (_, _, swarm, signed_txn) = get_transaction(get_sample_openid_sig_and_pk).await;

    info!("Submit OpenID transaction");
    let result = swarm
        .aptos_public_info()
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if let Err(e) = result {
        panic!("Error with OpenID TXN verification: {:?}", e)
    }
}

#[tokio::test]
async fn test_keyless_rotate_vk() {
    let (tw_sk, config, jwk, swarm, mut cli, root_idx) = setup_local_net().await;
    let mut info = swarm.aptos_public_info();

    let (old_sig, old_pk) = get_sample_groth16_sig_and_pk();
    let signed_txn = sign_transaction(
        &mut info,
        old_sig.clone(),
        old_pk.clone(),
        &jwk,
        &config,
        Some(&tw_sk),
        1,
    )
    .await;

    info!("Submitting keyless Groth16 transaction w.r.t. to initial VK; should succeed");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if let Err(e) = result {
        panic!("Keyless Groth16 TXN with old proof for old VK should have succeeded verification: {:?}", e)
    }

    let (new_sig, new_pk) = get_groth16_sig_and_pk_for_upgraded_vk();
    let signed_txn = sign_transaction(
        &mut info,
        new_sig.clone(),
        new_pk.clone(),
        &jwk,
        &config,
        Some(&tw_sk),
        2,
    )
    .await;
    info!("Submitting keyless Groth16 transaction w.r.t. to upgraded VK; should fail");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if result.is_ok() {
        panic!("Keyless Groth16 TXN with new proof for old VK should have failed verification")
    }

    info!("Rotating VK");
    let vk = get_upgraded_vk().into();
    rotate_vk_by_governance(&mut cli, &mut info, &vk, root_idx).await;

    let signed_txn =
        sign_transaction(&mut info, old_sig, old_pk, &jwk, &config, Some(&tw_sk), 2).await;

    info!("Submitting keyless Groth16 transaction w.r.t. to old VK; should fail");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if result.is_ok() {
        panic!("Keyless Groth16 TXN with old proof for old VK should have failed verification")
    }

    let signed_txn =
        sign_transaction(&mut info, new_sig, new_pk, &jwk, &config, Some(&tw_sk), 2).await;
    info!("Submitting keyless Groth16 transaction w.r.t. to upgraded VK; should succeed");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if let Err(e) = result {
        panic!("Keyless Groth16 TXN with new proof for new VK should have succeeded verification: {:?}", e)
    }
}

#[tokio::test]
async fn test_keyless_secure_test_jwk_initialized_at_genesis() {
    let (swarm, _cli, _faucet) = SwarmBuilder::new_local(1)
        .with_aptos()
        .build_with_cli(0)
        .await;
    let client = swarm.validators().next().unwrap().rest_client();
    swarm
        .wait_for_all_nodes_to_catchup_to_epoch(2, Duration::from_secs(60))
        .await
        .expect("Epoch 2 taking too long to come!");

    let patched_jwks = get_latest_jwkset(&client).await;
    debug!("patched_jwks={:?}", patched_jwks);
    let iss = get_sample_iss();
    let expected_providers_jwks = AllProvidersJWKs {
        entries: vec![ProviderJWKs {
            issuer: iss.into_bytes(),
            version: 0,
            jwks: vec![secure_test_rsa_jwk().into()],
        }],
    };
    assert_eq!(expected_providers_jwks, patched_jwks.jwks);
}

#[tokio::test]
async fn test_keyless_oidc_txn_with_bad_jwt_sig() {
    let (tw_sk, config, jwk, swarm, _, _) = setup_local_net().await;
    let (mut sig, pk) = get_sample_openid_sig_and_pk();

    match &mut sig.cert {
        EphemeralCertificate::ZeroKnowledgeSig(_) => panic!("Internal inconsistency"),
        EphemeralCertificate::OpenIdSig(openid_sig) => {
            openid_sig.jwt_sig = vec![0u8; 16] // Mauling the signature
        },
    }

    let mut info = swarm.aptos_public_info();
    let signed_txn = sign_transaction(&mut info, sig, pk, &jwk, &config, Some(&tw_sk), 1).await;

    info!("Submit OpenID transaction with bad JWT signature");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if result.is_ok() {
        panic!("OpenID TXN with bad JWT signature should have failed verification")
    }
}

#[tokio::test]
async fn test_keyless_oidc_txn_with_expired_epk() {
    let (tw_sk, config, jwk, swarm, _, _) = setup_local_net().await;
    let (mut sig, pk) = get_sample_openid_sig_and_pk();

    sig.exp_date_secs = 1; // This should fail the verification since the expiration date is way in the past

    let mut info = swarm.aptos_public_info();
    let signed_txn = sign_transaction(&mut info, sig, pk, &jwk, &config, Some(&tw_sk), 1).await;

    info!("Submit OpenID transaction with expired EPK");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if result.is_ok() {
        panic!("OpenID TXN with expired EPK should have failed verification")
    }
}

#[tokio::test]
async fn test_keyless_groth16_verifies() {
    let (_, _, swarm, signed_txn) = get_transaction(get_sample_groth16_sig_and_pk).await;

    info!("Submit keyless Groth16 transaction");
    let result = swarm
        .aptos_public_info()
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if let Err(e) = result {
        panic!("Error with keyless Groth16 TXN verification: {:?}", e)
    }
}

#[tokio::test]
async fn test_keyless_no_extra_field_groth16_verifies() {
    let (_, _, swarm, signed_txn) =
        get_transaction(get_sample_groth16_sig_and_pk_no_extra_field).await;

    info!("Submit keyless Groth16 transaction");
    let result = swarm
        .aptos_public_info()
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if let Err(e) = result {
        panic!("Error with keyless Groth16 TXN verification: {:?}", e)
    }
}

#[tokio::test]
async fn test_keyless_no_training_wheels_groth16_verifies() {
    let (_tw_sk, config, jwk, swarm, mut cli, root_idx) = setup_local_net().await;
    let (sig, pk) = get_sample_groth16_sig_and_pk();

    let mut info = swarm.aptos_public_info();

    remove_training_wheels(&mut cli, &mut info, root_idx).await;

    let signed_txn =
        sign_transaction(&mut info, sig.clone(), pk.clone(), &jwk, &config, None, 1).await;

    info!("Submit keyless Groth16 transaction");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if let Err(e) = result {
        panic!("Error with keyless Groth16 TXN verification: {:?}", e)
    }
}

#[tokio::test]
async fn test_keyless_groth16_with_mauled_proof() {
    let (tw_sk, config, jwk, swarm, _, _) = setup_local_net().await;
    let (sig, pk) = get_sample_groth16_sig_and_pk();

    let mut info = swarm.aptos_public_info();
    let signed_txn = sign_transaction(&mut info, sig, pk, &jwk, &config, Some(&tw_sk), 1).await;
    let signed_txn = maul_groth16_zkp_signature(signed_txn);

    info!("Submit keyless Groth16 transaction");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if result.is_ok() {
        panic!("Keyless Groth16 TXN with mauled proof should have failed verification")
    }
}

#[tokio::test]
async fn test_keyless_groth16_with_bad_tw_signature() {
    let (_tw_sk, config, jwk, swarm, _, _) = setup_local_net().await;
    let (sig, pk) = get_sample_groth16_sig_and_pk();

    let mut info = swarm.aptos_public_info();

    // using the sample ESK rather than the TW SK to get a bad training wheels signature
    let signed_txn = sign_transaction(
        &mut info,
        sig,
        pk,
        &jwk,
        &config,
        Some(&get_sample_esk()),
        1,
    )
    .await;

    info!("Submit keyless Groth16 transaction");
    let result = info
        .client()
        .submit_without_serializing_response(&signed_txn)
        .await;

    if result.is_ok() {
        panic!(
            "Keyless Groth16 TXN with bad training wheels signature should have failed verification"
        )
    }
}

async fn sign_transaction<'a>(
    info: &mut AptosPublicInfo,
    mut sig: KeylessSignature,
    pk: KeylessPublicKey,
    jwk: &RSA_JWK,
    config: &Configuration,
    tw_sk: Option<&Ed25519PrivateKey>,
    seqno: usize,
) -> SignedTransaction {
    let any_pk = AnyPublicKey::keyless(pk.clone());
    let addr = AuthenticationKey::any_key(any_pk.clone()).account_address();

    // If the account does not exist, create it.
    if info.account_exists(addr).await.is_err() {
        info!(
            "{} account does not exist. Creating...",
            addr.to_hex_literal()
        );
        info.create_user_account_with_any_key(&any_pk)
            .await
            .unwrap();
        info.mint(addr, 10_000_000_000).await.unwrap();
    }

    // TODO: No idea why, but these calls do not actually reflect the updated balance after a successful TXN.
    info!(
        "{} balance before TXN: {}",
        addr.to_hex_literal(),
        info.get_balance(addr).await.unwrap()
    );
    // TODO: No idea why, but these calls do not actually reflect the updated sequence number after a successful TXN.
    info!(
        "{} sequence number before TXN: {}",
        addr.to_hex_literal(),
        info.get_account_sequence_number(addr).await.unwrap()
    );

    let recipient = info
        .create_and_fund_user_account(20_000_000_000)
        .await
        .unwrap();

    let raw_txn = info
        .transaction_factory()
        .payload(aptos_stdlib::aptos_coin_transfer(
            recipient.address(),
            1_000_000,
        ))
        .sender(addr)
        .sequence_number(seqno as u64)
        .build();

    let esk = get_sample_esk();

    let public_inputs_hash: Option<[u8; 32]> =
        if let EphemeralCertificate::ZeroKnowledgeSig(_) = &sig.cert {
            // This will only calculate the hash if it's needed, avoiding unnecessary computation.
            Some(fr_to_bytes_le(
                &get_public_inputs_hash(&sig, &pk, jwk, config).unwrap(),
            ))
        } else {
            None
        };

    let mut txn_and_zkp = TransactionAndProof {
        message: raw_txn.clone(),
        proof: None,
    };

    // Compute the training wheels signature if not present
    match &mut sig.cert {
        EphemeralCertificate::ZeroKnowledgeSig(proof) => {
            let proof_and_statement = Groth16ProofAndStatement {
                proof: proof.proof.into(),
                public_inputs_hash: public_inputs_hash.unwrap(),
            };

            if let Some(tw_sk) = tw_sk {
                proof.training_wheels_signature = Some(EphemeralSignature::ed25519(
                    tw_sk.sign(&proof_and_statement).unwrap(),
                ));
            }

            txn_and_zkp.proof = Some(proof.proof);
        },
        EphemeralCertificate::OpenIdSig(_) => {},
    }

    sig.ephemeral_signature = EphemeralSignature::ed25519(esk.sign(&txn_and_zkp).unwrap());

    SignedTransaction::new_keyless(raw_txn, pk, sig)
}

fn maul_groth16_zkp_signature(txn: SignedTransaction) -> SignedTransaction {
    // extract the keyless PK and signature
    let (pk, sig) = match txn.authenticator() {
        TransactionAuthenticator::SingleSender {
            sender: AccountAuthenticator::SingleKey { authenticator },
        } => match (authenticator.public_key(), authenticator.signature()) {
            (AnyPublicKey::Keyless { public_key }, AnySignature::Keyless { signature }) => {
                (public_key.clone(), signature.clone())
            },
            _ => panic!("Expected keyless authenticator"),
        },
        _ => panic!("Expected keyless authenticator"),
    };

    // disassemble the txn
    let raw_txn = txn.into_raw_transaction();

    test_utils::maul_raw_groth16_txn(pk, sig, raw_txn)
}

async fn get_transaction(
    get_pk_and_sig_func: fn() -> (KeylessSignature, KeylessPublicKey),
) -> (
    KeylessSignature,
    KeylessPublicKey,
    LocalSwarm,
    SignedTransaction,
) {
    let (tw_sk, config, jwk, swarm, _, _) = setup_local_net().await;

    let (sig, pk) = get_pk_and_sig_func();

    let mut info = swarm.aptos_public_info();
    let signed_txn = sign_transaction(
        &mut info,
        sig.clone(),
        pk.clone(),
        &jwk,
        &config,
        Some(&tw_sk),
        1,
    )
    .await;

    (sig, pk, swarm, signed_txn)
}

async fn setup_local_net() -> (
    Ed25519PrivateKey,
    Configuration,
    RSA_JWK,
    LocalSwarm,
    CliTestFramework,
    usize,
) {
    let (mut swarm, mut cli, _faucet) = SwarmBuilder::new_local(1)
        .with_aptos()
        .build_with_cli(0)
        .await;

    let (tw_sk, config, jwk, root_idx) =
        spawn_network_and_execute_gov_proposals(&mut swarm, &mut cli).await;
    (tw_sk, config, jwk, swarm, cli, root_idx)
}

async fn remove_training_wheels<'a>(
    cli: &mut CliTestFramework,
    info: &mut AptosPublicInfo,
    root_idx: usize,
) {
    let script = format!(
        r#"
script {{
use aptos_framework::{};
use aptos_framework::aptos_governance;
use std::option;
fun main(core_resources: &signer) {{
    let framework_signer = aptos_governance::get_signer_testnet_only(core_resources, @0x1);
    {}::update_training_wheels_for_next_epoch(&framework_signer, option::none());
    aptos_governance::force_end_epoch(&framework_signer);
}}
}}
"#,
        KEYLESS_ACCOUNT_MODULE_NAME, KEYLESS_ACCOUNT_MODULE_NAME
    );
    let gas_options = GasOptions {
        gas_unit_price: Some(100),
        max_gas: Some(2000000),
        expiration_secs: 60,
    };
    let txn_summary = cli
        .run_script_with_gas_options(root_idx, &script, Some(gas_options))
        .await
        .unwrap();
    debug!("txn_summary={:?}", txn_summary);

    // Increment sequence number as we ran a governance proposal
    info.root_account().increment_sequence_number();

    print_account_resource::<Configuration>(
        info.client(),
        AccountAddress::ONE,
        KEYLESS_ACCOUNT_MODULE_NAME,
        "Configuration",
        "Keyless configuration after",
    )
    .await;
}

async fn spawn_network_and_execute_gov_proposals(
    swarm: &mut LocalSwarm,
    cli: &mut CliTestFramework,
) -> (Ed25519PrivateKey, Configuration, RSA_JWK, usize) {
    let client = swarm.validators().next().unwrap().rest_client();
    let root_idx = cli.add_account_with_address_to_cli(
        swarm.root_key(),
        swarm.chain_info().root_account().address(),
    );
    swarm
        .wait_for_all_nodes_to_catchup_to_epoch(2, Duration::from_secs(60))
        .await
        .expect("Epoch 2 taking too long to come!");

    let vk = print_account_resource::<Groth16VerificationKey>(
        &client,
        AccountAddress::ONE,
        KEYLESS_ACCOUNT_MODULE_NAME,
        "Groth16VerificationKey",
        "Groth16 VK",
    )
    .await;

    assert_eq!(
        vk,
        Groth16VerificationKey::from(DEVNET_VERIFICATION_KEY.clone())
    );

    let old_config = print_account_resource::<Configuration>(
        &client,
        AccountAddress::ONE,
        KEYLESS_ACCOUNT_MODULE_NAME,
        "Configuration",
        "Keyless configuration before",
    )
    .await;

    let iss = get_sample_iss();
    let jwk = get_sample_jwk();

    let training_wheels_sk = Ed25519PrivateKey::generate(&mut thread_rng());
    let training_wheels_pk = Ed25519PublicKey::from(&training_wheels_sk);

    info!("Insert JWK and update keyless configuration.");
    let max_exp_horizon_secs = Configuration::new_for_testing().max_exp_horizon_secs;
    let script = format!(
        r#"
script {{
use aptos_framework::jwks;
use aptos_framework::{};
use aptos_framework::aptos_governance;
use std::string::utf8;
use std::option;
fun main(core_resources: &signer) {{
    let framework_signer = aptos_governance::get_signer_testnet_only(core_resources, @0000000000000000000000000000000000000000000000000000000000000001);
    let jwk_0 = jwks::new_rsa_jwk(
        utf8(b"{}"),
        utf8(b"{}"),
        utf8(b"{}"),
        utf8(b"{}")
    );
    let patches = vector[
        jwks::new_patch_remove_all(),
        jwks::new_patch_upsert_jwk(b"{}", jwk_0),
    ];
    jwks::set_patches(&framework_signer, patches);

    {}::update_max_exp_horizon_for_next_epoch(&framework_signer, {});
    {}::update_training_wheels_for_next_epoch(&framework_signer, option::some(x"{}"));
    aptos_governance::force_end_epoch(&framework_signer);
}}
}}
"#,
        KEYLESS_ACCOUNT_MODULE_NAME,
        jwk.kid,
        jwk.alg,
        jwk.e,
        jwk.n,
        iss,
        KEYLESS_ACCOUNT_MODULE_NAME,
        max_exp_horizon_secs,
        KEYLESS_ACCOUNT_MODULE_NAME,
        hex::encode(training_wheels_pk.to_bytes())
    );

    let gas_options = GasOptions {
        gas_unit_price: Some(100),
        max_gas: Some(2000000),
        expiration_secs: 60,
    };
    let txn_summary = cli
        .run_script_with_gas_options(root_idx, &script, Some(gas_options))
        .await
        .unwrap();
    debug!("txn_summary={:?}", txn_summary);

    info!("Use resource API to check the patch result.");
    let patched_jwks = get_latest_jwkset(&client).await;
    debug!("patched_jwks={:?}", patched_jwks);

    let expected_providers_jwks = AllProvidersJWKs {
        entries: vec![ProviderJWKs {
            issuer: iss.into_bytes(),
            version: 0,
            jwks: vec![JWKMoveStruct::from(JWK::RSA(jwk.clone()))],
        }],
    };
    assert_eq!(expected_providers_jwks, patched_jwks.jwks);

    let new_config = print_account_resource::<Configuration>(
        &client,
        AccountAddress::ONE,
        KEYLESS_ACCOUNT_MODULE_NAME,
        "Configuration",
        "Keyless configuration after",
    )
    .await;

    assert_ne!(old_config, new_config);
    assert_eq!(new_config.max_exp_horizon_secs, max_exp_horizon_secs);

    let mut info = swarm.aptos_public_info();

    // Increment sequence number since we patched a JWK
    info.root_account().increment_sequence_number();

    (training_wheels_sk, new_config, jwk, root_idx)
}

async fn get_latest_jwkset(rest_client: &Client) -> PatchedJWKs {
    let maybe_response = rest_client
        .get_account_resource_bcs::<PatchedJWKs>(AccountAddress::ONE, "0x1::jwks::PatchedJWKs")
        .await;
    let response = maybe_response.unwrap();
    response.into_inner()
}

async fn rotate_vk_by_governance<'a>(
    cli: &mut CliTestFramework,
    info: &mut AptosPublicInfo,
    vk: &Groth16VerificationKey,
    root_idx: usize,
) {
    let script = format!(
        r#"
script {{
    use aptos_framework::{};
    use aptos_framework::aptos_governance;
    fun main(core_resources: &signer) {{
        let framework_signer = aptos_governance::get_signer_testnet_only(core_resources, @0x1);
        let vk = {}::new_groth16_verification_key(x"{}", x"{}", x"{}", x"{}", vector[x"{}", x"{}"]);
        {}::set_groth16_verification_key_for_next_epoch(&framework_signer, vk);
        aptos_governance::force_end_epoch(&framework_signer);
    }}
}}
"#,
        KEYLESS_ACCOUNT_MODULE_NAME,
        KEYLESS_ACCOUNT_MODULE_NAME,
        hex::encode(&vk.alpha_g1),
        hex::encode(&vk.beta_g2),
        hex::encode(&vk.gamma_g2),
        hex::encode(&vk.delta_g2),
        hex::encode(&vk.gamma_abc_g1[0]),
        hex::encode(&vk.gamma_abc_g1[1]),
        KEYLESS_ACCOUNT_MODULE_NAME
    );
    debug!("Move script for changing VK follows below:\n{:?}", script);

    print_account_resource::<Groth16VerificationKey>(
        info.client(),
        AccountAddress::ONE,
        KEYLESS_ACCOUNT_MODULE_NAME,
        "Groth16VerificationKey",
        "Keyless Groth16 VK before",
    )
    .await;

    let gas_options = GasOptions {
        gas_unit_price: Some(100),
        max_gas: Some(2000000),
        expiration_secs: 60,
    };
    let txn_summary = cli
        .run_script_with_gas_options(root_idx, &script, Some(gas_options))
        .await
        .unwrap();
    debug!("txn_summary={:?}", txn_summary);

    // Increment sequence number as we ran a governance proposal
    info.root_account().increment_sequence_number();

    print_account_resource::<Groth16VerificationKey>(
        info.client(),
        AccountAddress::ONE,
        KEYLESS_ACCOUNT_MODULE_NAME,
        "Groth16VerificationKey",
        "Keyless Groth16 VK after",
    )
    .await;
}

async fn print_account_resource<T: DeserializeOwned + Debug>(
    client: &Client,
    address: AccountAddress,
    module_name: &str,
    resource_name: &str,
    message: &str,
) -> T {
    let type_tag = format!(
        "{}::{}::{}",
        address.to_hex_literal(),
        module_name,
        resource_name
    );
    let maybe_response = client
        .get_account_resource_bcs::<T>(AccountAddress::ONE, type_tag.as_str())
        .await;

    let rsrc = maybe_response.unwrap().into_inner();
    println!("{}: {:?}", message, &rsrc);

    rsrc
}
