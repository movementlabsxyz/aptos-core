use anyhow::{Context, Result};
use aptos_types::network_address::NetworkAddress;
use clap::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "network_address_encoder")]
#[command(
    about = "Bidirectional encoder/decoder for Aptos NetworkAddress - auto-detects input format"
)]
struct Args {
    /// Input: either multiaddr string (/dns/...) or BCS hex string (01450402...)
    input: String,

    /// Output format: hex (default) or json
    #[arg(short, long, default_value = "hex")]
    format: String,
}

fn encode_multiaddr_to_bcs(multiaddr_input: &str) -> Result<String> {
    let mut addresses = Vec::new();

    // Split by comma and parse each address
    for addr_str in multiaddr_input.split(',') {
        let addr_str = addr_str.trim();
        if !addr_str.is_empty() {
            let network_address = NetworkAddress::from_str(addr_str)?;
            addresses.push(network_address);
        }
    }

    let encoded_bytes = bcs::to_bytes(&addresses)?;
    Ok(format!("0x{}", hex::encode(&encoded_bytes)))
}

fn decode_bcs_to_multiaddr(bcs_hex: &str) -> Result<String> {
    let hex_str = bcs_hex.strip_prefix("0x").unwrap_or(bcs_hex);
    let bcs_bytes = hex::decode(hex_str)?;

    if let Ok(addresses) = bcs::from_bytes::<Vec<NetworkAddress>>(&bcs_bytes) {
        let addr_strings: Vec<String> = addresses.iter().map(|addr| addr.to_string()).collect();
        return Ok(addr_strings.join(", "));
    }

    let network_address = bcs::from_bytes::<NetworkAddress>(&bcs_bytes)?;
    Ok(network_address.to_string())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Auto-detect input format and process accordingly
    if args.input.starts_with('/') {
        // Input is multiaddr string - encode to BCS
        let bcs_hex = encode_multiaddr_to_bcs(&args.input)?;
        println!("{}", bcs_hex);
    } else {
        // Input is BCS hex string - decode to multiaddr
        let multiaddr = decode_bcs_to_multiaddr(&args.input)?;
        println!("{}", multiaddr);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_address_encoding() {
        let addr_str = "/dns/validator.movement-l1/tcp/6180/noise-ik/77cc2fbc9935dd1630bf4e8143de17a4cd5e6df9a4abf69cc5c2f50bbc6e0763/handshake/1";

        let network_address = NetworkAddress::from_str(addr_str).unwrap();
        println!("Parsed: {}", network_address);

        // Test single address encoding
        let single_encoded = bcs::to_bytes(&network_address).unwrap();
        println!("Single encoded: {}", hex::encode(&single_encoded));

        // Test vector encoding (what Move function expects)
        let vector_encoded = bcs::to_bytes(&vec![network_address]).unwrap();
        println!("Vector encoded: {}", hex::encode(&vector_encoded));
    }

    #[test]
    fn test_fullnode_address_encoding() {
        let addr_str = "/dns/testnet.movementnetwork.xyz/tcp/6182/noise-ik/9967ebf40ac8c2ccb38709488952da1826176584ea3067b63b1695362ecb3d1f/handshake/0";

        let network_address = NetworkAddress::from_str(addr_str).unwrap();
        let vector_encoded = bcs::to_bytes(&vec![network_address]).unwrap();
        println!("Fullnode vector encoded: {}", hex::encode(&vector_encoded));
    }
}
