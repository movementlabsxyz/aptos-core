
<a id="0x1_governed_gas_pool"></a>

# Module `0x1::governed_gas_pool`



-  [Resource `GovernedGasPool`](#0x1_governed_gas_pool_GovernedGasPool)
-  [Constants](#@Constants_0)
-  [Function `primary_fungible_store_address`](#0x1_governed_gas_pool_primary_fungible_store_address)
-  [Function `create_resource_account_seed`](#0x1_governed_gas_pool_create_resource_account_seed)
-  [Function `initialize`](#0x1_governed_gas_pool_initialize)
-  [Function `init_module`](#0x1_governed_gas_pool_init_module)
-  [Function `governed_gas_signer`](#0x1_governed_gas_pool_governed_gas_signer)
-  [Function `governed_gas_pool_address`](#0x1_governed_gas_pool_governed_gas_pool_address)
-  [Function `fund`](#0x1_governed_gas_pool_fund)
-  [Function `deposit`](#0x1_governed_gas_pool_deposit)
-  [Function `deposit_from`](#0x1_governed_gas_pool_deposit_from)
-  [Function `deposit_from_fungible_store`](#0x1_governed_gas_pool_deposit_from_fungible_store)
-  [Function `deposit_gas_fee`](#0x1_governed_gas_pool_deposit_gas_fee)
-  [Function `deposit_gas_fee_v2`](#0x1_governed_gas_pool_deposit_gas_fee_v2)
-  [Function `get_balance`](#0x1_governed_gas_pool_get_balance)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error">0x1::error</a>;
</code></pre>



<a id="0x1_governed_gas_pool_GovernedGasPool"></a>

## Resource `GovernedGasPool`



<pre><code><b>struct</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_GovernedGasPool">GovernedGasPool</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>signer_capability: <a href="account.md#0x1_account_SignerCapability">account::SignerCapability</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_governed_gas_pool_E_DEPRECATED"></a>



<pre><code><b>const</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>: u64 = 1;
</code></pre>



<a id="0x1_governed_gas_pool_primary_fungible_store_address"></a>

## Function `primary_fungible_store_address`



<pre><code>#[deprecated]
<b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_primary_fungible_store_address">primary_fungible_store_address</a>(_account: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_primary_fungible_store_address">primary_fungible_store_address</a>(_account: <b>address</b>): <b>address</b> {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_create_resource_account_seed"></a>

## Function `create_resource_account_seed`



<pre><code>#[deprecated]
<b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_create_resource_account_seed">create_resource_account_seed</a>(_delegation_pool_creation_seed: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_create_resource_account_seed">create_resource_account_seed</a>(_delegation_pool_creation_seed: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_initialize"></a>

## Function `initialize`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_initialize">initialize</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _delegation_pool_creation_seed: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_initialize">initialize</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _delegation_pool_creation_seed: <a href="../../aptos-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_init_module"></a>

## Function `init_module`



<pre><code>#[deprecated]
<b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_init_module">init_module</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_init_module">init_module</a>(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_governed_gas_signer"></a>

## Function `governed_gas_signer`



<pre><code>#[deprecated]
<b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_governed_gas_signer">governed_gas_signer</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_governed_gas_signer">governed_gas_signer</a>(): <a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a> {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_governed_gas_pool_address"></a>

## Function `governed_gas_pool_address`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_governed_gas_pool_address">governed_gas_pool_address</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_governed_gas_pool_address">governed_gas_pool_address</a>(): <b>address</b> {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_fund"></a>

## Function `fund`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_fund">fund</a>&lt;CoinType&gt;(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _account: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_fund">fund</a>&lt;CoinType&gt;(_aptos_framework: &<a href="../../aptos-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, _account: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_deposit"></a>

## Function `deposit`



<pre><code>#[deprecated]
<b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit">deposit</a>&lt;CoinType&gt;(_coin: <a href="coin.md#0x1_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit">deposit</a>&lt;CoinType&gt;(_coin: Coin&lt;CoinType&gt;) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_deposit_from"></a>

## Function `deposit_from`



<pre><code>#[deprecated]
<b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_from">deposit_from</a>&lt;CoinType&gt;(_account: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_from">deposit_from</a>&lt;CoinType&gt;(_account: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_deposit_from_fungible_store"></a>

## Function `deposit_from_fungible_store`



<pre><code>#[deprecated]
<b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_from_fungible_store">deposit_from_fungible_store</a>(_account: <b>address</b>, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_from_fungible_store">deposit_from_fungible_store</a>(_account: <b>address</b>, _amount: u64) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_deposit_gas_fee"></a>

## Function `deposit_gas_fee`



<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_gas_fee">deposit_gas_fee</a>(_gas_payer: <b>address</b>, _gas_fee: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_gas_fee">deposit_gas_fee</a>(_gas_payer: <b>address</b>, _gas_fee: u64) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_deposit_gas_fee_v2"></a>

## Function `deposit_gas_fee_v2`



<pre><code>#[deprecated]
<b>public</b>(<b>friend</b>) <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_gas_fee_v2">deposit_gas_fee_v2</a>(_gas_payer: <b>address</b>, _gas_fee: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_deposit_gas_fee_v2">deposit_gas_fee_v2</a>(_gas_payer: <b>address</b>, _gas_fee: u64) {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>

<a id="0x1_governed_gas_pool_get_balance"></a>

## Function `get_balance`



<pre><code>#[view]
#[deprecated]
<b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_get_balance">get_balance</a>&lt;CoinType&gt;(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="governed_gas_pool.md#0x1_governed_gas_pool_get_balance">get_balance</a>&lt;CoinType&gt;(): u64 {
    <b>abort</b> <a href="../../aptos-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="governed_gas_pool.md#0x1_governed_gas_pool_E_DEPRECATED">E_DEPRECATED</a>)
}
</code></pre>



</details>


[move-book]: https://aptos.dev/move/book/SUMMARY
