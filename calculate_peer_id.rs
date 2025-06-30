use aptos_crypto::x25519;
use aptos_types::account_address::{from_identity_public_key, AccountAddress};

fn main() {
    // Your VFN private key
    let private_key_hex = "d0a1a7058f7bf2bd4af5b6257442d6dcf790a1cca89115d7983dd4a395dc3a7e";
    
    // Convert hex string to bytes
    let private_key_bytes = hex::decode(private_key_hex).expect("Invalid hex");
    
    // Create x25519 private key
    let private_key = x25519::PrivateKey::from_bytes(&private_key_bytes).expect("Invalid private key");
    
    // Get the public key
    let public_key = private_key.public_key();
    
    // Calculate peer ID using Aptos function
    let peer_id = from_identity_public_key(public_key);
    
    println!("Private Key: 0x{}", private_key_hex);
    println!("Public Key: 0x{}", hex::encode(public_key.as_slice()));
    println!("Peer ID: 0x{}", peer_id.to_hex());
    
    // Manual calculation to verify
    let pubkey_slice = public_key.as_slice();
    let last_16_bytes = &pubkey_slice[32 - 16..];
    println!("Last 16 bytes of public key: 0x{}", hex::encode(last_16_bytes));
    
    // Verify they match
    let manual_peer_id = AccountAddress::new(last_16_bytes.try_into().unwrap());
    println!("Manual calculation: 0x{}", manual_peer_id.to_hex());
    
    assert_eq!(peer_id, manual_peer_id);
    println!("✅ Calculations match!");
}
