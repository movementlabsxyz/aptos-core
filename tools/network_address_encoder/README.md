# Network Address Encoder

Converts between human-readable multiaddr strings and BCS-encoded hex format required by Aptos Move functions.

## What it does

- **Encodes** multiaddr strings → BCS hex for Move function parameters
- **Decodes** BCS hex → human-readable multiaddr strings  
- **Auto-detects** input format (starts with `/` = multiaddr, otherwise = hex)

## Usage

```bash
# Encode multiaddr to BCS hex
cargo run -- "/dns/validator.example.com/tcp/6180/noise-ik/a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890/handshake/1"
# → 0x013f04021576616c696461746f722e6578616d706c652e636f6d0524180720a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef12345678900801

# Decode BCS hex to multiaddr
cargo run -- "0x013f04021576616c696461746f722e6578616d706c652e636f6d0524180720a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef12345678900801"
# → /dns/validator.example.com/tcp/6180/noise-ik/0xa1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890/handshake/1

# Multiple addresses (comma-separated)
cargo run -- "/dns/host1.example.com/tcp/6180/noise-ik/a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890/handshake/1,/dns/host2.example.com/tcp/6182/noise-ik/b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890a1/handshake/0"
# → 0x023b040211686f7374312e6578616d706c652e636f6d0524180720a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef123456789008013b040211686f7374322e6578616d706c652e636f6d0526180720b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890a10800
```

## Multiaddr Format

- **DNS**: `/dns/hostname/tcp/port/noise-ik/<64-hex-char-x25519-pubkey>/handshake/<version>`
- **IP**: `/ip4/1.2.3.4/tcp/port/noise-ik/<64-hex-char-x25519-pubkey>/handshake/<version>`
- **Handshake version**: `1` for validator networks, `0` for fullnode networks

## Move Function Integration

Use the hex output directly in Move function calls:

```bash
# Get encoded address
ENCODED=$(cargo run -- "/dns/validator.example.com/tcp/6180/noise-ik/a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890/handshake/1")

# Use in Move function
aptos move run --function-id 0x1::stake::update_network_and_fullnode_addresses \
  --args address:0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
         hex:$ENCODED \
         hex:$FULLNODE_ENCODED
```

## Technical Details

The tool uses `aptos_types::network_address::NetworkAddress` for parsing and `bcs` for encoding into `vector<u8>` format that Move functions expect.