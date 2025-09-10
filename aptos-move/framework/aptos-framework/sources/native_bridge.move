module aptos_framework::native_bridge {
    use aptos_std::smart_table::SmartTable;
    use aptos_framework::event::EventHandle;
    use aptos_framework::aptos_coin::AptosCoin;
    use aptos_framework::coin::{BurnCapability, MintCapability};
    use aptos_framework::fungible_asset::{BurnRef, MintRef};

    const ETRANSFER_ALREADY_PROCESSED: u64 = 1;
    const EINVALID_BRIDGE_TRANSFER_ID: u64 = 2;
    const EEVENT_NOT_FOUND: u64 = 3;
    const EINVALID_NONCE: u64 = 4;
    const EINVALID_AMOUNT: u64 = 5;
    const ENONCE_NOT_FOUND: u64 = 6;
    const EZERO_AMOUNT: u64 = 7;
    const ENATIVE_BRIDGE_NOT_ENABLED: u64 = 8;
    const EINCORRECT_NONCE: u64 = 9;
    const EID_NOT_FOUND: u64 = 10;
    const EINVALID_BRIDGE_RELAYER: u64 = 11;
    const ESAME_FEE: u64 = 0x2;
    const EINVALID_VALUE: u64 = 0x3;
    const ERATE_LIMIT_EXCEEDED: u64 = 0x4;

    friend aptos_framework::genesis;

    #[event]
    struct BridgeConfigRelayerUpdated has store, drop {
        old_relayer: address,
        new_relayer: address,
    }

    #[event]
    struct BridgeFeeChangedEvent has store, drop {
        old_bridge_fee: u64,
        new_bridge_fee: u64,
    }

    #[event]
    struct BridgeInsuranceBudgetDividerChangedEvent has store, drop {
        old_insurance_budget_divider: u64,
        new_insurance_budget_divider: u64,
    }

    #[event]
    struct BridgeInsuranceFundChangedEvent has store, drop {
        old_insurance_fund: address,
        new_insurance_fund: address,
    }

    #[event]
    struct BridgeTransferInitiatedEvent has store, drop {
        bridge_transfer_id: vector<u8>,
        initiator: address,
        recipient: vector<u8>, // external destination address bytes
        amount: u64,
        nonce: u64,
    }

    #[event]
    struct BridgeTransferCompletedEvent has store, drop {
        bridge_transfer_id: vector<u8>,
        initiator: vector<u8>, // external source address bytes
        recipient: address,
        amount: u64,
        nonce: u64,
    }

    /// Event handles container.
    struct BridgeEvents has key, store {
        bridge_transfer_initiated_events: EventHandle<BridgeTransferInitiatedEvent>,
        bridge_transfer_completed_events: EventHandle<BridgeTransferCompletedEvent>,
    }

    struct AptosCoinBurnCapability has key { burn_cap: BurnCapability<AptosCoin> }
    struct AptosCoinMintCapability has key { mint_cap: MintCapability<AptosCoin> }
    struct AptosFABurnCapabilities has key { burn_ref: BurnRef }
    struct AptosFAMintCapabilities has key { burn_ref: MintRef }

    /// Monotonic nonce for transfers.
    struct Nonce has key { value: u64 }

    struct OutboundRateLimitBudget has key, store { day: SmartTable<u64, u64> }
    struct InboundRateLimitBudget has key, store { day: SmartTable<u64, u64> }

    struct SmartTableWrapper<K, V> has key, store { inner: SmartTable<K, V> }

    /// Outbound transfer details (external recipient address as bytes).
    struct OutboundTransfer has store, copy {
        bridge_transfer_id: vector<u8>,
        initiator: address,
        recipient: vector<u8>,
        amount: u64,
    }

    // -------------------------
    // Deprecated API (inline aborts)
    // -------------------------

    #[deprecated]
    public fun initialize(_aptos_framework: &signer) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun normalize_u64_to_32_bytes(_value: &u64): vector<u8> {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun is_inbound_nonce_set(_bridge_transfer_id: vector<u8>): bool {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun create_details(
        _initiator: address,
        _recipient: vector<u8>,
        _amount: u64,
        _nonce: u64
    ): OutboundTransfer {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun add(_nonce: u64, _details: OutboundTransfer) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun set_bridge_transfer_id_to_inbound_nonce(
        _bridge_transfer_id: vector<u8>,
        _inbound_nonce: u64
    ) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun assert_valid_bridge_transfer_id(_bridge_transfer_id: &vector<u8>) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun bridge_transfer_id(
        _initiator: address,
        _recipient: vector<u8>,
        _amount: u64,
        _nonce: u64
    ): vector<u8> {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[view]
    #[deprecated]
    public fun bridge_relayer(): address {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[view]
    #[deprecated]
    public fun insurance_fund(): address {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[view]
    #[deprecated]
    public fun insurance_budget_divider(): u64 {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[view]
    #[deprecated]
    public fun bridge_fee(): u64 {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[view]
    #[deprecated]
    public fun get_bridge_transfer_details_from_nonce(_nonce: u64): OutboundTransfer {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[view]
    #[deprecated]
    public fun get_inbound_nonce_from_bridge_transfer_id(_bridge_transfer_id: vector<u8>): u64 {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    fun increment_and_get_nonce(): u64 {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[test_only]
    #[deprecated]
    public fun initialize_for_test(_aptos_framework: &signer) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public fun store_aptos_coin_burn_cap(_aptos_framework: &signer, _burn_cap: BurnCapability<AptosCoin>) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public fun store_aptos_coin_mint_cap(_aptos_framework: &signer, _mint_cap: MintCapability<AptosCoin>) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public fun mint_to(_aptos_framework: &signer, _recipient: address, _amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun mint(_recipient: address, _amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    fun mint_internal(_recipient: address, _amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public fun burn_from(_aptos_framework: &signer, _from: address, _amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun burn(_from: address, _amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    fun burn_internal(_from: address, _amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public entry fun initiate_bridge_transfer(
        _initiator: &signer,
        _recipient: vector<u8>,
        _amount: u64
    ) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public entry fun complete_bridge_transfer(
        _caller: &signer,
        _bridge_transfer_id: vector<u8>,
        _initiator: vector<u8>,
        _recipient: address,
        _amount: u64,
        _nonce: u64
    ) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    fun charge_bridge_fee(_amount: u64): u64 {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public fun update_bridge_relayer(_aptos_framework: &signer, _new_relayer: address) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public entry fun update_bridge_fee(_relayer: &signer, _new_bridge_fee: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public entry fun update_insurance_fund(_aptos_framework: &signer, _new_insurance_fund: address) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public entry fun update_insurance_budget_divider(
        _aptos_framework: &signer,
        _new_insurance_budget_divider: u64
    ) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    public(friend) fun assert_is_caller_relayer(_caller: &signer) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    fun assert_outbound_rate_limit_budget_not_exceeded(_amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    fun assert_inbound_rate_limit_budget_not_exceeded(_amount: u64) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }

    #[deprecated]
    fun test_normalize_u64_to_32_bytes_helper(_x: u64, _expected: vector<u8>) {
        abort ENATIVE_BRIDGE_NOT_ENABLED
    }
}

