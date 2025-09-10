module aptos_framework::governed_gas_pool {
    /// NOTE: This module is deprecated. All functions are marked #[deprecated]
    /// and abort with E_DEPRECATED to preserve ABI compatibility while disabling behavior.

    friend aptos_framework::transaction_validation;

    use std::error;
    use std::vector;
    use aptos_framework::account::{Self, SignerCapability, create_signer_with_capability};
    use aptos_framework::system_addresses::{Self};
    use aptos_framework::fungible_asset::{Self};
    use aptos_framework::object::{Self};
    use aptos_framework::aptos_coin::AptosCoin;
    use aptos_framework::coin::{Self, Coin};
    use std::features;
    use aptos_framework::signer;
    use aptos_framework::aptos_account::Self;

    /// All entry points abort with this code.
    const E_DEPRECATED: u64 = 1;

    const MODULE_SALT: vector<u8> = b"aptos_framework::governed_gas_pool";

    /// The Governed Gas Pool (kept only for type compatibility).
    struct GovernedGasPool has key {
        signer_capability: SignerCapability,
    }

    /// Helper that every function calls to abort consistently.
    inline fun fail_deprecated<T>(): T {
        abort error::invalid_argument(E_DEPRECATED)
    }

    /// Address of APT Primary Fungible Store
    #[deprecated]
    inline fun primary_fungible_store_address(_account: address): address {
        fail_deprecated()
    }

    /// Create the seed to derive the resource account address.
    #[deprecated]
    fun create_resource_account_seed(
        _delegation_pool_creation_seed: vector<u8>,
    ): vector<u8> {
        fail_deprecated()
    }

    /// Initializes the governed gas pool (deprecated).
    #[deprecated]
    public fun initialize(
        _aptos_framework: &signer,
        _delegation_pool_creation_seed: vector<u8>,
    ) {
        fail_deprecated()
    }

    /// Initialize the governed gas pool as a module (deprecated).
    #[deprecated]
    fun init_module(_aptos_framework: &signer) {
        fail_deprecated()
    }

    /// Borrows the signer of the governed gas pool (deprecated).
    #[deprecated]
    fun governed_gas_signer(): signer acquires GovernedGasPool {
        fail_deprecated()
    }

    #[view]
    /// Gets the address of the governed gas pool (deprecated).
    #[deprecated]
    public fun governed_gas_pool_address(): address acquires GovernedGasPool {
        fail_deprecated()
    }

    /// Funds the destination account with a given amount of coin (deprecated).
    #[deprecated]
    public fun fund<CoinType>(
        _aptos_framework: &signer,
        _account: address,
        _amount: u64
    ) acquires GovernedGasPool {
        fail_deprecated()
    }

    /// Deposits some coin into the governed gas pool (deprecated).
    #[deprecated]
    fun deposit<CoinType>(_coin: Coin<CoinType>) acquires GovernedGasPool {
        fail_deprecated()
    }

    /// Deposits some coin from an account to the governed gas pool (deprecated).
    #[deprecated]
    fun deposit_from<CoinType>(_account: address, _amount: u64) acquires GovernedGasPool {
        fail_deprecated()
    }

    /// Deposits some FA from the fungible store (deprecated).
    #[deprecated]
    fun deposit_from_fungible_store(_account: address, _amount: u64) acquires GovernedGasPool {
        fail_deprecated()
    }

    /// Deposits gas fees into the governed gas pool (deprecated).
    #[deprecated]
    public fun deposit_gas_fee(_gas_payer: address, _gas_fee: u64) acquires GovernedGasPool {
        fail_deprecated()
    }

    /// Deposits gas fees into the governed gas pool (deprecated).
    #[deprecated]
    public(friend) fun deposit_gas_fee_v2(
        _gas_payer: address,
        _gas_fee: u64
    ) acquires GovernedGasPool {
        fail_deprecated()
    }

    #[view]
    /// Gets the balance of a specified coin type in the governed gas pool (deprecated).
    #[deprecated]
    public fun get_balance<CoinType>(): u64 acquires GovernedGasPool {
        fail_deprecated()
    }
}

