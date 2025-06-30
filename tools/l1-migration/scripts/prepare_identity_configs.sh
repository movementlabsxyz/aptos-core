#!/bin/bash

# Script to generate identity files for validator and validator fullnode
# and update configuration files with the correct keys and peer IDs

set -e

# Configuration DATA_DIR is path to your DB folder
DATA_DIR=$1
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="$SCRIPT_DIR/../local-node-configs"

echo "Generating identity files for local Aptos network..."
echo "Data directory: $DATA_DIR"
echo "Config directory: $CONFIG_DIR"

# Create data directory if it doesn't exist
mkdir -p "$DATA_DIR"

# Generate validator identity using aptos genesis generate-keys
echo "Generating validator identity..."
cargo run -p aptos -- genesis generate-keys --output-dir "$DATA_DIR"

# Check if files were generated
if [ ! -f "$DATA_DIR/validator-identity.yaml" ]; then
    echo "✗ Failed to generate validator identity"
    exit 1
fi

if [ ! -f "$DATA_DIR/validator-full-node-identity.yaml" ]; then
    echo "✗ Failed to generate validator fullnode identity"
    exit 1
fi

if [ ! -f "$DATA_DIR/private-keys.yaml" ]; then
    echo "✗ Failed to generate private keys"
    exit 1
fi

echo "✓ Identity files generated successfully"

# Extract the VFN network private key from validator-full-node-identity.yaml
echo "Extracting VFN network identity..."
VFN_PRIVATE_KEY=$(grep "network_private_key:" "$DATA_DIR/validator-full-node-identity.yaml" | cut -d'"' -f2)

if [ -z "$VFN_PRIVATE_KEY" ]; then
    echo "✗ Failed to extract VFN private key"
    exit 1
fi

echo "VFN Private Key: $VFN_PRIVATE_KEY"

# Generate public key from private key
echo "Generating public key from VFN private key..."
TEMP_PUB_FILE="/tmp/vfn_public_key.yaml"
cargo run -p aptos -- key extract-public-key --private-key "$VFN_PRIVATE_KEY" --output-file "$TEMP_PUB_FILE"

if [ ! -f "$TEMP_PUB_FILE.pub" ]; then
    echo "✗ Failed to generate public key"
    exit 1
fi

VFN_PUBLIC_KEY=$(cat "$TEMP_PUB_FILE.pub")
echo "VFN Public Key: $VFN_PUBLIC_KEY"

# Calculate peer_id from public key (last 16 bytes of 32-byte public key)
VFN_PEER_ID_SUFFIX=$(echo "$VFN_PUBLIC_KEY" | tail -c 33 | head -c 32 | tr '[:upper:]' '[:lower:]')
VFN_PEER_ID="00000000000000000000000000000000$VFN_PEER_ID_SUFFIX"
echo "VFN Peer ID: $VFN_PEER_ID"

# Clean up temp file
rm -f "$TEMP_PUB_FILE" "$TEMP_PUB_FILE.pub"

# Function to update YAML files with correct identity configuration and data directories
update_config_file() {
    local config_file="$1"
    local config_type="$2"
    
    echo "Updating $config_file..."
    
    if [ ! -f "$config_file" ]; then
        echo "✗ Config file not found: $config_file"
        return 1
    fi
    
    # Update common data directory paths for all config types
    echo "  Updating data directory paths...$DATA_DIR"
    
    # Update base data_dir
    if [ "$config_type" = "validator_node" ]; then
        yq -y -i '.base.data_dir = "'"$DATA_DIR"'"' "$config_file"
    elif [ "$config_type" = "validator_fullnode" ]; then
        yq -y -i '.base.data_dir = "'"$DATA_DIR"'"' "$config_file"
    elif [ "$config_type" = "fullnode" ]; then
        yq -y -i '.base.data_dir = "'"$DATA_DIR"'"' "$config_file"
    fi
    
    # Update waypoint file path
    yq -y -i '.base.waypoint.from_file = "'"$DATA_DIR"'/waypoint.txt"' "$config_file"
    
    # Update genesis file location
    yq -y -i '.execution.genesis_file_location = "'"$DATA_DIR"'/genesis.blob"' "$config_file"
    
    if [ "$config_type" = "validator_node" ]; then
        # Update validator-specific paths
        echo "  Updating validator-specific paths..."
        
        # Update consensus safety rules paths
        yq -y -i '.consensus.safety_rules.backend.path = "'"$DATA_DIR"'/validator/secure-data.json"' "$config_file"
        yq -y -i '.consensus.safety_rules.initial_safety_rules_config.from_file.waypoint.from_file = "'"$DATA_DIR"'/waypoint.txt"' "$config_file"
        yq -y -i '.consensus.safety_rules.initial_safety_rules_config.from_file.identity_blob_path = "'"$DATA_DIR"'/validator-identity.yaml"' "$config_file"
        
        # Update validator network identity path
        yq -y -i '.validator_network.identity.path = "'"$DATA_DIR"'/validator-identity.yaml"' "$config_file"
        
        # Update VFN network identity configuration
        echo "  Updating VFN identity configuration..."
        yq -y -i '.full_node_networks[0].identity.key = "'"$VFN_PRIVATE_KEY"'"' "$config_file"
        yq -y -i '.full_node_networks[0].identity.peer_id = "'"$VFN_PEER_ID"'"' "$config_file"
        echo "✓ Updated VFN identity in $config_file"
        
    elif [ "$config_type" = "validator_fullnode" ]; then
        # Update validator fullnode-specific paths
        echo "  Updating validator fullnode-specific paths..."
        
        # Update identity file paths for both VFN and public networks
        yq -y -i '.full_node_networks[0].identity.path = "'"$DATA_DIR"'/validator-full-node-identity.yaml"' "$config_file"
        yq -y -i '.full_node_networks[1].identity.path = "'"$DATA_DIR"'/validator-full-node-identity.yaml"' "$config_file"
        
        # Update seeds section
        echo "  Updating seeds configuration..."
        # Clear existing seeds and add the new seed with the correct peer_id
        yq -y -i 'del(.full_node_networks[0].seeds)' "$config_file"
        yq -y -i '.full_node_networks[0].seeds."'"$VFN_PEER_ID"'".addresses[0] = "/ip4/127.0.0.1/tcp/6181"' "$config_file"
        yq -y -i '.full_node_networks[0].seeds."'"$VFN_PEER_ID"'".role = "Validator"' "$config_file"
        echo "✓ Updated seeds in $config_file with peer_id: $VFN_PEER_ID"
        
    elif [ "$config_type" = "fullnode" ]; then
        # For regular fullnode, no additional identity paths to update
        echo "  Fullnode configuration updated with data directory paths"
    fi
    
    echo "✓ Successfully updated $config_file"
}

# Update configuration files
echo ""
echo "Updating configuration files with generated identities..."

# Update validator_node.yaml
if [ -f "$CONFIG_DIR/validator_node.yaml" ]; then
    update_config_file "$CONFIG_DIR/validator_node.yaml" "validator_node"
fi

# Update validator_full_node.yaml
if [ -f "$CONFIG_DIR/validator_full_node.yaml" ]; then
    update_config_file "$CONFIG_DIR/validator_full_node.yaml" "validator_fullnode"
fi

# Update full_node.yaml
if [ -f "$CONFIG_DIR/full_node.yaml" ]; then
    update_config_file "$CONFIG_DIR/full_node.yaml" "fullnode"
fi

echo ""
echo "✓ Identity generation and configuration update completed successfully!"
echo ""
echo "Generated files:"
echo "  - $DATA_DIR/validator-identity.yaml"
echo "  - $DATA_DIR/validator-full-node-identity.yaml"
echo "  - $DATA_DIR/private-keys.yaml"
echo ""
echo "Updated configuration files:"
echo "  - $CONFIG_DIR/validator_node.yaml (if exists)"
echo "  - $CONFIG_DIR/validator_full_node.yaml (if exists)"
echo "  - $CONFIG_DIR/full_node.yaml (if exists)"
echo ""
echo "VFN Network Identity:"
echo "  Private Key: $VFN_PRIVATE_KEY"
echo "  Public Key:  $VFN_PUBLIC_KEY"
echo "  Peer ID:     $VFN_PEER_ID"
echo ""
echo "⚠️  IMPORTANT: Keep private keys secure and do not share them!"
echo "⚠️  Backup files created with .backup extension"
