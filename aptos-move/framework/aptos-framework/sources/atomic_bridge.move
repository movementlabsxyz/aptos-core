module aptos_framework::atomic_bridge_initiator {
    const EATOMIC_BRIDGE_DISABLED: u64 = 0x3073d;

    use aptos_framework::event::EventHandle;

    #[event]
    struct BridgeTransferInitiatedEvent has store, drop {
        bridge_transfer_id: vector<u8>,
        initiator: address,
        recipient: vector<u8>,
        amount: u64,
        hash_lock: vector<u8>,
        time_lock: u64,
    }

    #[event]
    struct BridgeTransferCompletedEvent has store, drop {
        bridge_transfer_id: vector<u8>,
        pre_image: vector<u8>,
    }

    #[event]
    struct BridgeTransferRefundedEvent has store, drop {
        bridge_transfer_id: vector<u8>,
    }

    struct BridgeInitiatorEvents has key, store {
        bridge_transfer_initiated_events: EventHandle<BridgeTransferInitiatedEvent>,
        bridge_transfer_completed_events: EventHandle<BridgeTransferCompletedEvent>,
        bridge_transfer_refunded_events: EventHandle<BridgeTransferRefundedEvent>,
    }

    #[deprecated]
    public fun initialize(_aptos_framework: &signer) {
        abort EATOMIC_BRIDGE_DISABLED
    }

    /// Initiate a bridge transfer (external recipient address as bytes).
    #[deprecated]
    public entry fun initiate_bridge_transfer(
        _initiator: &signer,
        _recipient: vector<u8>,
        _hash_lock: vector<u8>,
        _amount: u64
    ) {
        abort EATOMIC_BRIDGE_DISABLED
    }

    /// Operator completes the transfer.
    #[deprecated]
    public entry fun complete_bridge_transfer (
        _caller: &signer,
        _bridge_transfer_id: vector<u8>,
        _pre_image: vector<u8>,
    ) {
       abort EATOMIC_BRIDGE_DISABLED
    }

    /// Refund after timelock.
    #[deprecated]
    public entry fun refund_bridge_transfer (
        _caller: &signer,
        _bridge_transfer_id: vector<u8>,
    ) {
       abort EATOMIC_BRIDGE_DISABLED
    }
}

