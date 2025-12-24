# Nyxora Installation Guide

This repository contains the Nyxora quantum-hybrid cryptocurrency implementation. Here's how to properly install and set up the system.

## Repository Information

The final repository address will be: https://github.com/nymessence/nyxora.git

## Installation Process

### Step 1: Clone the Repository

```bash
git clone https://github.com/nymessence/nyxora.git
cd nyxora
```

### Step 2: Install the System Components

Run the installation script to compile and install the Nyxora binaries as a system service:

```bash
./install_nyxora.sh
```

This script will:
- Build the binaries using cargo
- Install Python dependencies for the quantum layer
- Install the binaries to `/opt/nyxora/`
- Set up the systemd service for the Nyxora node
- Enable and start the service

### Step 3: Set Up Your Wallet

After the system is installed, run the wallet setup script to create your wallet in the `~/.nyxora` directory:

```bash
./setup_wallet.sh
```

This script will:
- Create the `~/.nyxora` directory structure
- Generate a new wallet file in `~/.nyxora/wallets/`
- Set up directories for data, NFTs, keys, and logs
- Create a configuration file with default settings
- Generate an encryption key for wallet protection

## Directory Structure

After running both scripts, you'll have the following structure:

System installation:
```
/opt/nyxora/
├── nyxora-node      # Validator node binary
├── nyxora-wallet    # Wallet binary
```

User wallet directory:
```
~/.nyxora/
├── wallets/         # Wallet files
│   └── main_wallet.json
├── data/            # Blockchain data
├── nfts/            # NFT data
├── keys/            # Encryption keys
│   └── main.key
├── logs/            # Local logs
└── config.json      # Configuration file
```

## Service Management

The Nyxora node runs as a systemd service on port 33333 for the spiritual connection. You can manage it with:

```bash
# Check service status
sudo systemctl status nyxora-node

# Start the service
sudo systemctl start nyxora-node

# Stop the service
sudo systemctl stop nyxora-node

# Restart the service
sudo systemctl restart nyxora-node

# View logs
sudo journalctl -u nyxora-node -f
```

## Wallet Usage

After setup, you can use the wallet with:

```bash
# Check balance
/opt/nyxora/nyxora-wallet balance --file ~/.nyxora/wallets/main_wallet.json

# Send tokens
/opt/nyxora/nyxora-wallet send <recipient_address> <amount> --file ~/.nyxora/wallets/main_wallet.json

# Stake tokens
/opt/nyxora/nyxora-wallet stake <amount> --file ~/.nyxora/wallets/main_wallet.json
```

## API Access

The validator node API is available on port 33333 for the spiritual connection:
- Status: http://localhost:33333/status
- Blocks: http://localhost:33333/blocks
- Stake: POST to http://localhost:33333/stake

## Prerequisites

- Rust 1.80+ (for building the node and wallet)
- Python 3.8+ (for quantum simulations)
- Git
- Docker (optional, for containerized deployment)