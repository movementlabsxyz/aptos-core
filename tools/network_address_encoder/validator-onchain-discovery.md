# Validator On-Chain Discovery Playbook

## What is On-Chain Discovery?

Instead of hardcoding validator network addresses in configuration files, validators store their network endpoints on-chain in `ValidatorConfig`. New validators can discover peers by reading the current `ValidatorSet` from the blockchain.

**Benefits**: Automatic peer discovery, no manual configuration updates when validators join/leave.

## Typical Flow

1. **Generate node identities** and register validator in ValidatorSet (with empty/placeholder addresses)
2. **Start the node** - it will discover and connect to existing validators via on-chain discovery
3. **Once stable**, update your actual network addresses on-chain for other validators to discover you

## Prerequisites

- Validator account should have some APT to submit transactions
- Network identity keys generated (validator-identity.yaml with x25519 public key)
- Validator registered in ValidatorSet (addresses can be empty initially)

## Step-by-Step Setup

### 1. Start Validator Node

Start your validator with on-chain discovery enabled:

```bash
./aptos-node -f validator.yaml
```

Configuration should have:
```yaml
# validator.yaml
validator_network:
  discovery_method: onchain  # Will discover other validators from ValidatorSet
  mutual_authentication: true
  identity:
    type: from_file
    path: validator-identity.yaml
  listen_address: /ip4/0.0.0.0/tcp/6180
```

Your node will automatically discover and connect to existing validators.

### 2. Encode Network Addresses (Once Stable)

Use the network address encoder tool to convert multiaddr strings to BCS hex:

```bash
cd tools/network_address_encoder

# Encode validator network address (handshake version 1)
VALIDATOR_ENCODED=$(cargo run -- "/dns/validator.example.com/tcp/6180/noise-ik/a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890/handshake/1")
echo "Validator encoded: $VALIDATOR_ENCODED"

# Encode fullnode address (handshake version 0)  
FULLNODE_ENCODED=$(cargo run -- "/dns/fullnode.example.com/tcp/6182/noise-ik/b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890a1/handshake/0")
echo "Fullnode encoded: $FULLNODE_ENCODED"
```

### 3. Update ValidatorConfig On-Chain

Execute the Move function to update network addresses:

```bash
./aptos move run \
  --function-id 0x1::stake::update_network_and_fullnode_addresses \
  --args address:0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
         hex:$VALIDATOR_ENCODED \
         hex:$FULLNODE_ENCODED \
  --private-key <OPERATOR_PRIVATE_KEY> \
  --url https://testnet.example.com/v1 \
  --assume-yes
```

### 4. Verify the Update

Check that the addresses were updated correctly:

```bash
# Query ValidatorConfig
curl -s "https://testnet.example.com/v1/accounts/0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef/resource/0x1::stake::ValidatorConfig" | jq '.data | {network_addresses, fullnode_addresses}'

# Decode the hex back to verify format
cargo run -- "$VALIDATOR_ENCODED"
cargo run -- "$FULLNODE_ENCODED"
```

## Result

After updating your addresses on-chain, other validators can now discover and connect to your validator automatically. Your validator will continue discovering other validators as they join or update their addresses.

## Troubleshooting

**Permission Denied**: Ensure operator has `StakeManagementPermission`
**Invalid Address Format**: Use the encoder tool to verify BCS encoding
**Discovery Fails**: Check that `ValidatorConfig.network_addresses` is not empty (`0x`)
**Connection Issues**: Verify firewall allows traffic on configured ports

## Example Validator

**Address**: `0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef`
**Status**: Example addresses for demonstration
**Operator**: Same as validator address (self-operated)

## References

- Network address encoder tool: `README.md`
- Move function: `0x1::stake::update_network_and_fullnode_addresses`
- Testnet endpoint: https://testnet.example.com/v1