# Nyxora Setup Scripts

This directory contains shell scripts to help set up and run the Nyxora quantum-hybrid cryptocurrency.

## Scripts

### 1. setup_nyxora.sh
This script will:
- Clone or update the Nyxora repository
- Build the binaries (nyxora-node and nyxora-wallet)
- Install Python dependencies for the quantum layer
- Install the binaries to `/opt/nyxora/`
- Set up the systemd service for the Nyxora node
- Enable and start the service

### 2. setup_wallet.sh
This script will:
- Create the `~/.nyxora` directory structure
- Generate a new wallet file in `~/.nyxora/wallets/`
- Set up directories for data, NFTs, keys, and logs
- Create a configuration file with default settings
- Generate an encryption key for wallet protection

## Usage

### To set up the complete system:

1. Run the setup script to compile and install the system service:
```bash
./setup_nyxora.sh
```

2. After the system is set up, run the wallet script to create your wallet:
```bash
./setup_wallet.sh
```

### Directory Structure

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

### Service Management

The Nyxora node runs as a systemd service. You can manage it with:

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

### Wallet Usage

After setup, you can use the wallet with:

```bash
# Check balance
/opt/nyxora/nyxora-wallet balance --file ~/.nyxora/wallets/main_wallet.json

# Send tokens
/opt/nyxora/nyxora-wallet send <recipient_address> <amount> --file ~/.nyxora/wallets/main_wallet.json

# Stake tokens
/opt/nyxora/nyxora-wallet stake <amount> --file ~/.nyxora/wallets/main_wallet.json
```