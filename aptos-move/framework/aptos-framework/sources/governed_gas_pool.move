module aptos_framework::governed_gas_pool {
    friend aptos_framework::transaction_validation;

    use std::error;
    use aptos_framework::account::SignerCapability;
    use aptos_framework::coin::Coin;

    // All entry points abort with this code to indicate deprecation.
    const E_DEPRECATED: u64 = 1;

    // Kept for type compatibility.
    struct GovernedGasPool has key {
        signer_capability: SignerCapability,
    }

    // ---- Deprecated API (inline aborts; no global storage access) ----

    #[deprecated]
    inline fun primary_fungible_store_address(_account: address): address {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    fun create_resource_account_seed(_delegation_pool_creation_seed: vector<u8>): vector<u8> {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    public fun initialize(_aptos_framework: &signer, _delegation_pool_creation_seed: vector<u8>) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    fun init_module(_aptos_framework: &signer) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    fun governed_gas_signer(): signer {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[view]
    #[deprecated]
    public fun governed_gas_pool_address(): address {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    public fun fund<CoinType>(_aptos_framework: &signer, _account: address, _amount: u64) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    fun deposit<CoinType>(_coin: Coin<CoinType>) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    fun deposit_from<CoinType>(_account: address, _amount: u64) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    fun deposit_from_fungible_store(_account: address, _amount: u64) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    public fun deposit_gas_fee(_gas_payer: address, _gas_fee: u64) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[deprecated]
    public(friend) fun deposit_gas_fee_v2(_gas_payer: address, _gas_fee: u64) {
        abort error::invalid_argument(E_DEPRECATED)
    }

    #[view]
    #[deprecated]
    public fun get_balance<CoinType>(): u64 {
        abort error::invalid_argument(E_DEPRECATED)
    }
}

