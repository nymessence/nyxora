#!/bin/bash

# Nyxora Wallet Setup Script
# This script creates a wallet in ~/.nyxora directory and sets up storage for node/blockchain/nft data

set -e  # Exit on any error

echo "Nyxora Quantum-Hybrid Cryptocurrency - Wallet Setup Script"
echo "=========================================================="

# Define the nyxora directory
NYXORA_DIR="$HOME/.nyxora"

echo "Step 1: Creating ~/.nyxora directory structure..."
# Create the nyxora directory and subdirectories
mkdir -p "$NYXORA_DIR"
mkdir -p "$NYXORA_DIR/data"        # For blockchain data
mkdir -p "$NYXORA_DIR/wallets"     # For wallet files
mkdir -p "$NYXORA_DIR/nfts"        # For NFT data
mkdir -p "$NYXORA_DIR/keys"        # For encryption keys
mkdir -p "$NYXORA_DIR/logs"        # For local logs if needed

echo "Step 2: Generating new wallet..."
# Generate a new wallet in the wallets directory
WALLET_FILE="$NYXORA_DIR/wallets/main_wallet.json"

# Run the installed nyxora-wallet binary to generate a wallet
/opt/nyxora/nyxora-wallet generate

# Move the generated wallet to the proper location
if [ -f "wallet.json" ]; then
    mv wallet.json "$WALLET_FILE"
    echo "✓ Wallet created: $WALLET_FILE"
else
    echo "✗ Error: Wallet file was not created"
    exit 1
fi

echo "Step 3: Setting up wallet permissions..."
# Set appropriate permissions for wallet files
chmod 600 "$WALLET_FILE"

echo "Step 4: Displaying wallet information..."
# Show the wallet address
WALLET_ADDR=$(/opt/nyxora/nyxora-wallet address --file "$WALLET_FILE")
echo "✓ Wallet Address: $WALLET_ADDR"

echo "Step 5: Creating configuration file..."
# Create a local configuration file
CONFIG_FILE="$NYXORA_DIR/config.json"
cat > "$CONFIG_FILE" << EOF
{
  "wallet_file": "$WALLET_FILE",
  "data_dir": "$NYXORA_DIR/data",
  "nft_dir": "$NYXORA_DIR/nfts",
  "keys_dir": "$NYXORA_DIR/keys",
  "logs_dir": "$NYXORA_DIR/logs",
  "node_api_url": "http://localhost:33333",
  "default_stake_amount": 1000,
  "quantum_enabled": true
}
EOF

echo "Step 6: Setting up encryption keys directory..."
# Generate a basic encryption key for wallet protection (for demonstration)
KEY_FILE="$NYXORA_DIR/keys/main.key"
if [ ! -f "$KEY_FILE" ]; then
    openssl rand -hex 32 > "$KEY_FILE"
    chmod 600 "$KEY_FILE"
    echo "✓ Encryption key created: $KEY_FILE"
fi

echo ""
echo "Wallet setup completed successfully!"
echo ""
echo "Directory structure created in: $NYXORA_DIR"
echo ""
echo "Wallet information:"
echo "  Address: $WALLET_ADDR"
echo "  File: $WALLET_FILE"
echo "  Configuration: $CONFIG_FILE"
echo ""
echo "Directory structure:"
echo "  $NYXORA_DIR/"
echo "  ├── wallets/          # Wallet files"
echo "  ├── data/             # Blockchain data"
echo "  ├── nfts/             # NFT data"
echo "  ├── keys/             # Encryption keys"
echo "  ├── logs/             # Local logs"
echo "  └── config.json       # Configuration file"
echo ""
echo "To check your wallet balance:"
echo "  /opt/nyxora/nyxora-wallet balance --file $WALLET_FILE"
echo ""
echo "To send tokens:"
echo "  /opt/nyxora/nyxora-wallet send <recipient_address> <amount> --file $WALLET_FILE"
echo ""
echo "To stake tokens:"
echo "  /opt/nyxora/nyxora-wallet stake <amount> --file $WALLET_FILE"