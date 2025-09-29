
<a id="0x1_native_bridge"></a>

# Module `0x1::native_bridge`



-  [Struct `BridgeConfigRelayerUpdated`](#0x1_native_bridge_BridgeConfigRelayerUpdated)
-  [Struct `BridgeFeeChangedEvent`](#0x1_native_bridge_BridgeFeeChangedEvent)
-  [Struct `BridgeInsuranceBudgetDividerChangedEvent`](#0x1_native_bridge_BridgeInsuranceBudgetDividerChangedEvent)
-  [Struct `BridgeInsuranceFundChangedEvent`](#0x1_native_bridge_BridgeInsuranceFundChangedEvent)
-  [Struct `BridgeTransferInitiatedEvent`](#0x1_native_bridge_BridgeTransferInitiatedEvent)
-  [Struct `BridgeTransferCompletedEvent`](#0x1_native_bridge_BridgeTransferCompletedEvent)
-  [Resource `BridgeEvents`](#0x1_native_bridge_BridgeEvents)
-  [Resource `AptosCoinBurnCapability`](#0x1_native_bridge_AptosCoinBurnCapability)
-  [Resource `AptosCoinMintCapability`](#0x1_native_bridge_AptosCoinMintCapability)
-  [Resource `AptosFABurnCapabilities`](#0x1_native_bridge_AptosFABurnCapabilities)
-  [Resource `AptosFAMintCapabilities`](#0x1_native_bridge_AptosFAMintCapabilities)
-  [Resource `Nonce`](#0x1_native_bridge_Nonce)
-  [Resource `OutboundRateLimitBudget`](#0x1_native_bridge_OutboundRateLimitBudget)
-  [Resource `InboundRateLimitBudget`](#0x1_native_bridge_InboundRateLimitBudget)
-  [Resource `SmartTableWrapper`](#0x1_native_bridge_SmartTableWrapper)
-  [Struct `OutboundTransfer`](#0x1_native_bridge_OutboundTransfer)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_native_bridge_initialize)
-  [Function `normalize_u64_to_32_bytes`](#0x1_native_bridge_normalize_u64_to_32_bytes)
-  [Function `is_inbound_nonce_set`](#0x1_native_bridge_is_inbound_nonce_set)
-  [Function `create_details`](#0x1_native_bridge_create_details)
-  [Function `add`](#0x1_native_bridge_add)
-  [Function `set_bridge_transfer_id_to_inbound_nonce`](#0x1_native_bridge_set_bridge_transfer_id_to_inbound_nonce)
-  [Function `assert_valid_bridge_transfer_id`](#0x1_native_bridge_assert_valid_bridge_transfer_id)
-  [Function `bridge_transfer_id`](#0x1_native_bridge_bridge_transfer_id)
-  [Function `bridge_relayer`](#0x1_native_bridge_bridge_relayer)
-  [Function `insurance_fund`](#0x1_native_bridge_insurance_fund)
-  [Function `insurance_budget_divider`](#0x1_native_bridge_insurance_budget_divider)
-  [Function `bridge_fee`](#0x1_native_bridge_bridge_fee)
-  [Function `get_bridge_transfer_details_from_nonce`](#0x1_native_bridge_get_bridge_transfer_details_from_nonce)
-  [Function `get_inbound_nonce_from_bridge_transfer_id`](#0x1_native_bridge_get_inbound_nonce_from_bridge_transfer_id)
-  [Function `increment_and_get_nonce`](#0x1_native_bridge_increment_and_get_nonce)
-  [Function `store_aptos_coin_burn_cap`](#0x1_native_bridge_store_aptos_coin_burn_cap)
-  [Function `store_aptos_coin_mint_cap`](#0x1_native_bridge_store_aptos_coin_mint_cap)
-  [Function `mint_to`](#0x1_native_bridge_mint_to)
-  [Function `mint`](#0x1_native_bridge_mint)
-  [Function `mint_internal`](#0x1_native_bridge_mint_internal)
-  [Function `burn_from`](#0x1_native_bridge_burn_from)
-  [Function `burn`](#0x1_native_bridge_burn)
-  [Function `burn_internal`](#0x1_native_bridge_burn_internal)
-  [Function `initiate_bridge_transfer`](#0x1_native_bridge_initiate_bridge_transfer)
-  [Function `complete_bridge_transfer`](#0x1_native_bridge_complete_bridge_transfer)
-  [Function `charge_bridge_fee`](#0x1_native_bridge_charge_bridge_fee)
-  [Function `update_bridge_relayer`](#0x1_native_bridge_update_bridge_relayer)
-  [Function `update_bridge_fee`](#0x1_native_bridge_update_bridge_fee)
-  [Function `update_insurance_fund`](#0x1_native_bridge_update_insurance_fund)
-  [Function `update_insurance_budget_divider`](#0x1_native_bridge_update_insurance_budget_divider)
-  [Function `assert_is_caller_relayer`](#0x1_native_bridge_assert_is_caller_relayer)
-  [Function `assert_outbound_rate_limit_budget_not_exceeded`](#0x1_native_bridge_assert_outbound_rate_limit_budget_not_exceeded)
-  [Function `assert_inbound_rate_limit_budget_not_exceeded`](#0x1_native_bridge_assert_inbound_rate_limit_budget_not_exceeded)
-  [Function `test_normalize_u64_to_32_bytes_helper`](#0x1_native_bridge_test_normalize_u64_to_32_bytes_helper)


<pre><code><b>use</b> <a href="aptos_coin.md#0x1_aptos_coin">0x1::aptos_coin</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="../../aptos-stdlib/doc/smart_table.md#0x1_smart_table">0x1::smart_table</a>;
</code></pre>



<a id="0x1_native_bridge_BridgeConfigRelayerUpdated"></a>

## Struct `BridgeConfigRelayerUpdated`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="native_bridge.md#0x1_native_bridge_BridgeConfigRelayerUpdated">BridgeConfigRelayerUpdated</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>old_relayer: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_relayer: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_BridgeFeeChangedEvent"></a>

## Struct `BridgeFeeChangedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="native_bridge.md#0x1_native_bridge_BridgeFeeChangedEvent">BridgeFeeChangedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>old_bridge_fee: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_bridge_fee: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_BridgeInsuranceBudgetDividerChangedEvent"></a>

## Struct `BridgeInsuranceBudgetDividerChangedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="native_bridge.md#0x1_native_bridge_BridgeInsuranceBudgetDividerChangedEvent">BridgeInsuranceBudgetDividerChangedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>old_insurance_budget_divider: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>new_insurance_budget_divider: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_BridgeInsuranceFundChangedEvent"></a>

## Struct `BridgeInsuranceFundChangedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="native_bridge.md#0x1_native_bridge_BridgeInsuranceFundChangedEvent">BridgeInsuranceFundChangedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>old_insurance_fund: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_insurance_fund: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_BridgeTransferInitiatedEvent"></a>

## Struct `BridgeTransferInitiatedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="native_bridge.md#0x1_native_bridge_BridgeTransferInitiatedEvent">BridgeTransferInitiatedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>nonce: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_BridgeTransferCompletedEvent"></a>

## Struct `BridgeTransferCompletedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="native_bridge.md#0x1_native_bridge_BridgeTransferCompletedEvent">BridgeTransferCompletedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>nonce: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_BridgeEvents"></a>

## Resource `BridgeEvents`

Event handles container.


<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_BridgeEvents">BridgeEvents</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_transfer_initiated_events: <a href="event.md#0x1_event_EventHandle">event::EventHandle</a>&lt;<a href="native_bridge.md#0x1_native_bridge_BridgeTransferInitiatedEvent">native_bridge::BridgeTransferInitiatedEvent</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>bridge_transfer_completed_events: <a href="event.md#0x1_event_EventHandle">event::EventHandle</a>&lt;<a href="native_bridge.md#0x1_native_bridge_BridgeTransferCompletedEvent">native_bridge::BridgeTransferCompletedEvent</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_AptosCoinBurnCapability"></a>

## Resource `AptosCoinBurnCapability`



<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_AptosCoinBurnCapability">AptosCoinBurnCapability</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a>&lt;<a href="aptos_coin.md#0x1_aptos_coin_AptosCoin">aptos_coin::AptosCoin</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_AptosCoinMintCapability"></a>

## Resource `AptosCoinMintCapability`



<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_AptosCoinMintCapability">AptosCoinMintCapability</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>mint_cap: <a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a>&lt;<a href="aptos_coin.md#0x1_aptos_coin_AptosCoin">aptos_coin::AptosCoin</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_AptosFABurnCapabilities"></a>

## Resource `AptosFABurnCapabilities`



<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_AptosFABurnCapabilities">AptosFABurnCapabilities</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>burn_ref: <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_AptosFAMintCapabilities"></a>

## Resource `AptosFAMintCapabilities`



<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_AptosFAMintCapabilities">AptosFAMintCapabilities</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>burn_ref: <a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_Nonce"></a>

## Resource `Nonce`

Monotonic nonce for transfers.


<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_Nonce">Nonce</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_OutboundRateLimitBudget"></a>

## Resource `OutboundRateLimitBudget`



<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_OutboundRateLimitBudget">OutboundRateLimitBudget</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>day: <a href="../../aptos-stdlib/doc/smart_table.md#0x1_smart_table_SmartTable">smart_table::SmartTable</a>&lt;u64, u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_InboundRateLimitBudget"></a>

## Resource `InboundRateLimitBudget`



<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_InboundRateLimitBudget">InboundRateLimitBudget</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>day: <a href="../../aptos-stdlib/doc/smart_table.md#0x1_smart_table_SmartTable">smart_table::SmartTable</a>&lt;u64, u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_SmartTableWrapper"></a>

## Resource `SmartTableWrapper`



<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_SmartTableWrapper">SmartTableWrapper</a>&lt;K, V&gt; <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: <a href="../../aptos-stdlib/doc/smart_table.md#0x1_smart_table_SmartTable">smart_table::SmartTable</a>&lt;K, V&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_native_bridge_OutboundTransfer"></a>

## Struct `OutboundTransfer`

Outbound transfer details (external recipient address as bytes).


<pre><code><b>struct</b> <a href="native_bridge.md#0x1_native_bridge_OutboundTransfer">OutboundTransfer</a> <b>has</b> <b>copy</b>, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_native_bridge_EEVENT_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EEVENT_NOT_FOUND">EEVENT_NOT_FOUND</a>: u64 = 3;
</code></pre>



<a id="0x1_native_bridge_EID_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EID_NOT_FOUND">EID_NOT_FOUND</a>: u64 = 10;
</code></pre>



<a id="0x1_native_bridge_EINCORRECT_NONCE"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EINCORRECT_NONCE">EINCORRECT_NONCE</a>: u64 = 9;
</code></pre>



<a id="0x1_native_bridge_EINVALID_AMOUNT"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EINVALID_AMOUNT">EINVALID_AMOUNT</a>: u64 = 5;
</code></pre>



<a id="0x1_native_bridge_EINVALID_BRIDGE_RELAYER"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EINVALID_BRIDGE_RELAYER">EINVALID_BRIDGE_RELAYER</a>: u64 = 11;
</code></pre>



<a id="0x1_native_bridge_EINVALID_BRIDGE_TRANSFER_ID"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EINVALID_BRIDGE_TRANSFER_ID">EINVALID_BRIDGE_TRANSFER_ID</a>: u64 = 2;
</code></pre>



<a id="0x1_native_bridge_EINVALID_NONCE"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EINVALID_NONCE">EINVALID_NONCE</a>: u64 = 4;
</code></pre>



<a id="0x1_native_bridge_EINVALID_VALUE"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EINVALID_VALUE">EINVALID_VALUE</a>: u64 = 3;
</code></pre>



<a id="0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>: u64 = 8;
</code></pre>



<a id="0x1_native_bridge_ENONCE_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_ENONCE_NOT_FOUND">ENONCE_NOT_FOUND</a>: u64 = 6;
</code></pre>



<a id="0x1_native_bridge_ERATE_LIMIT_EXCEEDED"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_ERATE_LIMIT_EXCEEDED">ERATE_LIMIT_EXCEEDED</a>: u64 = 4;
</code></pre>



<a id="0x1_native_bridge_ESAME_FEE"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_ESAME_FEE">ESAME_FEE</a>: u64 = 2;
</code></pre>



<a id="0x1_native_bridge_ETRANSFER_ALREADY_PROCESSED"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_ETRANSFER_ALREADY_PROCESSED">ETRANSFER_ALREADY_PROCESSED</a>: u64 = 1;
</code></pre>



<a id="0x1_native_bridge_EZERO_AMOUNT"></a>



<pre><code><b>const</b> <a href="native_bridge.md#0x1_native_bridge_EZERO_AMOUNT">EZERO_AMOUNT</a>: u64 = 7;
</code></pre>



<a id="0x1_native_bridge_initialize"></a>

## Function `initialize`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_initialize">initialize</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_initialize">initialize</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_normalize_u64_to_32_bytes"></a>

## Function `normalize_u64_to_32_bytes`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_normalize_u64_to_32_bytes">normalize_u64_to_32_bytes</a>(_value: &u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_normalize_u64_to_32_bytes">normalize_u64_to_32_bytes</a>(_value: &u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_is_inbound_nonce_set"></a>

## Function `is_inbound_nonce_set`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_is_inbound_nonce_set">is_inbound_nonce_set</a>(_bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_is_inbound_nonce_set">is_inbound_nonce_set</a>(_bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_create_details"></a>

## Function `create_details`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_create_details">create_details</a>(_initiator: <b>address</b>, _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _amount: u64, _nonce: u64): <a href="native_bridge.md#0x1_native_bridge_OutboundTransfer">native_bridge::OutboundTransfer</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_create_details">create_details</a>(
    _initiator: <b>address</b>,
    _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _amount: u64,
    _nonce: u64
): <a href="native_bridge.md#0x1_native_bridge_OutboundTransfer">OutboundTransfer</a> {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_add"></a>

## Function `add`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_add">add</a>(_nonce: u64, _details: <a href="native_bridge.md#0x1_native_bridge_OutboundTransfer">native_bridge::OutboundTransfer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_add">add</a>(_nonce: u64, _details: <a href="native_bridge.md#0x1_native_bridge_OutboundTransfer">OutboundTransfer</a>) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_set_bridge_transfer_id_to_inbound_nonce"></a>

## Function `set_bridge_transfer_id_to_inbound_nonce`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_set_bridge_transfer_id_to_inbound_nonce">set_bridge_transfer_id_to_inbound_nonce</a>(_bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _inbound_nonce: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_set_bridge_transfer_id_to_inbound_nonce">set_bridge_transfer_id_to_inbound_nonce</a>(
    _bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _inbound_nonce: u64
) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_assert_valid_bridge_transfer_id"></a>

## Function `assert_valid_bridge_transfer_id`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_valid_bridge_transfer_id">assert_valid_bridge_transfer_id</a>(_bridge_transfer_id: &<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_valid_bridge_transfer_id">assert_valid_bridge_transfer_id</a>(_bridge_transfer_id: &<a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_bridge_transfer_id"></a>

## Function `bridge_transfer_id`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_bridge_transfer_id">bridge_transfer_id</a>(_initiator: <b>address</b>, _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _amount: u64, _nonce: u64): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_bridge_transfer_id">bridge_transfer_id</a>(
    _initiator: <b>address</b>,
    _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _amount: u64,
    _nonce: u64
): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_bridge_relayer"></a>

## Function `bridge_relayer`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_bridge_relayer">bridge_relayer</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_bridge_relayer">bridge_relayer</a>(): <b>address</b> {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_insurance_fund"></a>

## Function `insurance_fund`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_insurance_fund">insurance_fund</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_insurance_fund">insurance_fund</a>(): <b>address</b> {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_insurance_budget_divider"></a>

## Function `insurance_budget_divider`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_insurance_budget_divider">insurance_budget_divider</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_insurance_budget_divider">insurance_budget_divider</a>(): u64 {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_bridge_fee"></a>

## Function `bridge_fee`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_bridge_fee">bridge_fee</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_bridge_fee">bridge_fee</a>(): u64 {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_get_bridge_transfer_details_from_nonce"></a>

## Function `get_bridge_transfer_details_from_nonce`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_get_bridge_transfer_details_from_nonce">get_bridge_transfer_details_from_nonce</a>(_nonce: u64): <a href="native_bridge.md#0x1_native_bridge_OutboundTransfer">native_bridge::OutboundTransfer</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_get_bridge_transfer_details_from_nonce">get_bridge_transfer_details_from_nonce</a>(_nonce: u64): <a href="native_bridge.md#0x1_native_bridge_OutboundTransfer">OutboundTransfer</a> {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_get_inbound_nonce_from_bridge_transfer_id"></a>

## Function `get_inbound_nonce_from_bridge_transfer_id`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_get_inbound_nonce_from_bridge_transfer_id">get_inbound_nonce_from_bridge_transfer_id</a>(_bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_get_inbound_nonce_from_bridge_transfer_id">get_inbound_nonce_from_bridge_transfer_id</a>(_bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u64 {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_increment_and_get_nonce"></a>

## Function `increment_and_get_nonce`



<pre><code>#[deprecated]
<b>fun</b> <a href="native_bridge.md#0x1_native_bridge_increment_and_get_nonce">increment_and_get_nonce</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="native_bridge.md#0x1_native_bridge_increment_and_get_nonce">increment_and_get_nonce</a>(): u64 {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_store_aptos_coin_burn_cap"></a>

## Function `store_aptos_coin_burn_cap`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_store_aptos_coin_burn_cap">store_aptos_coin_burn_cap</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a>&lt;<a href="aptos_coin.md#0x1_aptos_coin_AptosCoin">aptos_coin::AptosCoin</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_store_aptos_coin_burn_cap">store_aptos_coin_burn_cap</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _burn_cap: BurnCapability&lt;AptosCoin&gt;) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_store_aptos_coin_mint_cap"></a>

## Function `store_aptos_coin_mint_cap`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_store_aptos_coin_mint_cap">store_aptos_coin_mint_cap</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _mint_cap: <a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a>&lt;<a href="aptos_coin.md#0x1_aptos_coin_AptosCoin">aptos_coin::AptosCoin</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_store_aptos_coin_mint_cap">store_aptos_coin_mint_cap</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _mint_cap: MintCapability&lt;AptosCoin&gt;) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_mint_to"></a>

## Function `mint_to`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_mint_to">mint_to</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _recipient: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_mint_to">mint_to</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _recipient: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_mint"></a>

## Function `mint`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_mint">mint</a>(_recipient: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_mint">mint</a>(_recipient: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_mint_internal"></a>

## Function `mint_internal`



<pre><code>#[deprecated]
<b>fun</b> <a href="native_bridge.md#0x1_native_bridge_mint_internal">mint_internal</a>(_recipient: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="native_bridge.md#0x1_native_bridge_mint_internal">mint_internal</a>(_recipient: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_burn_from"></a>

## Function `burn_from`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_burn_from">burn_from</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _from: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_burn_from">burn_from</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _from: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_burn"></a>

## Function `burn`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_burn">burn</a>(_from: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_burn">burn</a>(_from: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_burn_internal"></a>

## Function `burn_internal`



<pre><code>#[deprecated]
<b>fun</b> <a href="native_bridge.md#0x1_native_bridge_burn_internal">burn_internal</a>(_from: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="native_bridge.md#0x1_native_bridge_burn_internal">burn_internal</a>(_from: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_initiate_bridge_transfer"></a>

## Function `initiate_bridge_transfer`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_initiate_bridge_transfer">initiate_bridge_transfer</a>(_initiator: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_initiate_bridge_transfer">initiate_bridge_transfer</a>(
    _initiator: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _amount: u64
) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_complete_bridge_transfer"></a>

## Function `complete_bridge_transfer`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_complete_bridge_transfer">complete_bridge_transfer</a>(_caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _initiator: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _recipient: <b>address</b>, _amount: u64, _nonce: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_complete_bridge_transfer">complete_bridge_transfer</a>(
    _caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    _bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _initiator: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _recipient: <b>address</b>,
    _amount: u64,
    _nonce: u64
) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_charge_bridge_fee"></a>

## Function `charge_bridge_fee`



<pre><code>#[deprecated]
<b>fun</b> <a href="native_bridge.md#0x1_native_bridge_charge_bridge_fee">charge_bridge_fee</a>(_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="native_bridge.md#0x1_native_bridge_charge_bridge_fee">charge_bridge_fee</a>(_amount: u64): u64 {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_update_bridge_relayer"></a>

## Function `update_bridge_relayer`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_bridge_relayer">update_bridge_relayer</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _new_relayer: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_bridge_relayer">update_bridge_relayer</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _new_relayer: <b>address</b>) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_update_bridge_fee"></a>

## Function `update_bridge_fee`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_bridge_fee">update_bridge_fee</a>(_relayer: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _new_bridge_fee: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_bridge_fee">update_bridge_fee</a>(_relayer: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _new_bridge_fee: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_update_insurance_fund"></a>

## Function `update_insurance_fund`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_insurance_fund">update_insurance_fund</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _new_insurance_fund: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_insurance_fund">update_insurance_fund</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _new_insurance_fund: <b>address</b>) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_update_insurance_budget_divider"></a>

## Function `update_insurance_budget_divider`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_insurance_budget_divider">update_insurance_budget_divider</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _new_insurance_budget_divider: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_update_insurance_budget_divider">update_insurance_budget_divider</a>(
    _aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    _new_insurance_budget_divider: u64
) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_assert_is_caller_relayer"></a>

## Function `assert_is_caller_relayer`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_is_caller_relayer">assert_is_caller_relayer</a>(_caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_is_caller_relayer">assert_is_caller_relayer</a>(_caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_assert_outbound_rate_limit_budget_not_exceeded"></a>

## Function `assert_outbound_rate_limit_budget_not_exceeded`



<pre><code>#[deprecated]
<b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_outbound_rate_limit_budget_not_exceeded">assert_outbound_rate_limit_budget_not_exceeded</a>(_amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_outbound_rate_limit_budget_not_exceeded">assert_outbound_rate_limit_budget_not_exceeded</a>(_amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_assert_inbound_rate_limit_budget_not_exceeded"></a>

## Function `assert_inbound_rate_limit_budget_not_exceeded`



<pre><code>#[deprecated]
<b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_inbound_rate_limit_budget_not_exceeded">assert_inbound_rate_limit_budget_not_exceeded</a>(_amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="native_bridge.md#0x1_native_bridge_assert_inbound_rate_limit_budget_not_exceeded">assert_inbound_rate_limit_budget_not_exceeded</a>(_amount: u64) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>

<a id="0x1_native_bridge_test_normalize_u64_to_32_bytes_helper"></a>

## Function `test_normalize_u64_to_32_bytes_helper`



<pre><code>#[deprecated]
<b>fun</b> <a href="native_bridge.md#0x1_native_bridge_test_normalize_u64_to_32_bytes_helper">test_normalize_u64_to_32_bytes_helper</a>(_x: u64, _expected: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="native_bridge.md#0x1_native_bridge_test_normalize_u64_to_32_bytes_helper">test_normalize_u64_to_32_bytes_helper</a>(_x: u64, _expected: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>abort</b> <a href="native_bridge.md#0x1_native_bridge_ENATIVE_BRIDGE_NOT_ENABLED">ENATIVE_BRIDGE_NOT_ENABLED</a>
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
