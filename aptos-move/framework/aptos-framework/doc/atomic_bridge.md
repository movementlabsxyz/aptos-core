
<a id="0x1_atomic_bridge_initiator"></a>

# Module `0x1::atomic_bridge_initiator`



-  [Struct `BridgeTransferInitiatedEvent`](#0x1_atomic_bridge_initiator_BridgeTransferInitiatedEvent)
-  [Struct `BridgeTransferCompletedEvent`](#0x1_atomic_bridge_initiator_BridgeTransferCompletedEvent)
-  [Struct `BridgeTransferRefundedEvent`](#0x1_atomic_bridge_initiator_BridgeTransferRefundedEvent)
-  [Resource `BridgeInitiatorEvents`](#0x1_atomic_bridge_initiator_BridgeInitiatorEvents)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_atomic_bridge_initiator_initialize)
-  [Function `initiate_bridge_transfer`](#0x1_atomic_bridge_initiator_initiate_bridge_transfer)
-  [Function `complete_bridge_transfer`](#0x1_atomic_bridge_initiator_complete_bridge_transfer)
-  [Function `refund_bridge_transfer`](#0x1_atomic_bridge_initiator_refund_bridge_transfer)


<pre><code><b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
</code></pre>



<a id="0x1_atomic_bridge_initiator_BridgeTransferInitiatedEvent"></a>

## Struct `BridgeTransferInitiatedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_BridgeTransferInitiatedEvent">BridgeTransferInitiatedEvent</a> <b>has</b> drop, store
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
<code>hash_lock: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>time_lock: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_atomic_bridge_initiator_BridgeTransferCompletedEvent"></a>

## Struct `BridgeTransferCompletedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_BridgeTransferCompletedEvent">BridgeTransferCompletedEvent</a> <b>has</b> drop, store
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
<code>pre_image: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_atomic_bridge_initiator_BridgeTransferRefundedEvent"></a>

## Struct `BridgeTransferRefundedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_BridgeTransferRefundedEvent">BridgeTransferRefundedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_atomic_bridge_initiator_BridgeInitiatorEvents"></a>

## Resource `BridgeInitiatorEvents`



<pre><code><b>struct</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_BridgeInitiatorEvents">BridgeInitiatorEvents</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_transfer_initiated_events: <a href="event.md#0x1_event_EventHandle">event::EventHandle</a>&lt;<a href="atomic_bridge.md#0x1_atomic_bridge_initiator_BridgeTransferInitiatedEvent">atomic_bridge_initiator::BridgeTransferInitiatedEvent</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>bridge_transfer_completed_events: <a href="event.md#0x1_event_EventHandle">event::EventHandle</a>&lt;<a href="atomic_bridge.md#0x1_atomic_bridge_initiator_BridgeTransferCompletedEvent">atomic_bridge_initiator::BridgeTransferCompletedEvent</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>bridge_transfer_refunded_events: <a href="event.md#0x1_event_EventHandle">event::EventHandle</a>&lt;<a href="atomic_bridge.md#0x1_atomic_bridge_initiator_BridgeTransferRefundedEvent">atomic_bridge_initiator::BridgeTransferRefundedEvent</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_atomic_bridge_initiator_EATOMIC_BRIDGE_DISABLED"></a>



<pre><code><b>const</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_EATOMIC_BRIDGE_DISABLED">EATOMIC_BRIDGE_DISABLED</a>: u64 = 198461;
</code></pre>



<a id="0x1_atomic_bridge_initiator_initialize"></a>

## Function `initialize`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_initialize">initialize</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_initialize">initialize</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>abort</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_EATOMIC_BRIDGE_DISABLED">EATOMIC_BRIDGE_DISABLED</a>
}
</code></pre>



</details>

<a id="0x1_atomic_bridge_initiator_initiate_bridge_transfer"></a>

## Function `initiate_bridge_transfer`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_initiate_bridge_transfer">initiate_bridge_transfer</a>(_initiator: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _hash_lock: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_initiate_bridge_transfer">initiate_bridge_transfer</a>(
    _initiator: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    _recipient: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _hash_lock: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _amount: u64
) {
    <b>abort</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_EATOMIC_BRIDGE_DISABLED">EATOMIC_BRIDGE_DISABLED</a>
}
</code></pre>



</details>

<a id="0x1_atomic_bridge_initiator_complete_bridge_transfer"></a>

## Function `complete_bridge_transfer`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_complete_bridge_transfer">complete_bridge_transfer</a>(_caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _pre_image: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_complete_bridge_transfer">complete_bridge_transfer</a> (
    _caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    _bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _pre_image: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
   <b>abort</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_EATOMIC_BRIDGE_DISABLED">EATOMIC_BRIDGE_DISABLED</a>
}
</code></pre>



</details>

<a id="0x1_atomic_bridge_initiator_refund_bridge_transfer"></a>

## Function `refund_bridge_transfer`



<pre><code>#[deprecated]
<b>public</b> entry <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_refund_bridge_transfer">refund_bridge_transfer</a>(_caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_refund_bridge_transfer">refund_bridge_transfer</a> (
    _caller: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    _bridge_transfer_id: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
   <b>abort</b> <a href="atomic_bridge.md#0x1_atomic_bridge_initiator_EATOMIC_BRIDGE_DISABLED">EATOMIC_BRIDGE_DISABLED</a>
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
