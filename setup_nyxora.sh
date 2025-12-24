#!/bin/bash

# Nyxora Setup and Compilation Script
# This script builds and installs the Nyxora quantum-hybrid cryptocurrency node as a system service

set -e  # Exit on any error

echo "Nyxora Quantum-Hybrid Cryptocurrency - Setup and Compilation Script"
echo "====================================================================="

# Check if running as root (required for system service installation)
if [[ $EUID -eq 0 ]]; then
   echo "This script should NOT be run as root" 
   exit 1
fi

# Check for required tools
command -v git >/dev/null 2>&1 || { echo "git is required but not installed. Aborting."; exit 1; }
command -v rustc >/dev/null 2>&1 || { echo "Rust is required but not installed. Please install Rust first. Aborting."; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "Cargo is required but not installed. Please install Cargo first. Aborting."; exit 1; }

# Check if Python is available
command -v python3 >/dev/null 2>&1 || { echo "Python3 is required but not installed. Aborting."; exit 1; }

# Define installation directory
INSTALL_DIR="/opt/nyxora"
SVC_FILE="/etc/systemd/system/nyxora-node.service"
CONFIG_DIR="/etc/nyxora"

echo "Step 1: Cloning or updating Nyxora repository..."
# Clone or update the repository
REPO_DIR="$HOME/nyxora-source"
if [ -d "$REPO_DIR" ]; then
    echo "Updating existing repository..."
    cd "$REPO_DIR"
    git pull
else
    echo "Cloning repository..."
    git clone https://github.com/nyxora/nyxora.git "$REPO_DIR"
    cd "$REPO_DIR"
fi

echo "Step 2: Building Nyxora binaries..."
# Build the binaries
cd "$REPO_DIR/nyxora"
cargo build --release

echo "Step 3: Installing Python dependencies for quantum layer..."
# Install Python dependencies
pip3 install qiskit numpy

echo "Step 4: Creating nyxora system user..."
# Create user for nyxora if it doesn't exist
if ! id "nyxora" &>/dev/null; then
    echo "Creating nyxora user..."
    sudo useradd -r -s /bin/false nyxora
fi

echo "Step 5: Creating directories..."
# Create directories
sudo mkdir -p "$INSTALL_DIR"
sudo mkdir -p "$CONFIG_DIR"
sudo mkdir -p "$INSTALL_DIR/data"
sudo mkdir -p "$INSTALL_DIR/logs"

echo "Step 6: Installing binaries..."
# Copy binaries to installation directory
sudo cp "$REPO_DIR/nyxora/target/release/nyxora-node" "$INSTALL_DIR/"
sudo cp "$REPO_DIR/nyxora/target/release/nyxora-wallet" "$INSTALL_DIR/"

# Make binaries executable
sudo chmod +x "$INSTALL_DIR/nyxora-node"
sudo chmod +x "$INSTALL_DIR/nyxora-wallet"

echo "Step 7: Installing service file..."
# Copy service file
sudo cp "$REPO_DIR/nyxora/services/nyxora-node.service" "$SVC_FILE"

# Create default configuration
if [ ! -f "$CONFIG_DIR/config.json" ]; then
    echo "Creating default configuration..."
    sudo tee "$CONFIG_DIR/config.json" > /dev/null <<EOF
{
  "address": "",
  "stake_amount": 10000,
  "is_validator": false,
  "quantum_enabled": true,
  "peers": ["mainnet-peer1.nyxora.io:8080", "mainnet-peer2.nyxora.io:8080"]
}
EOF
    sudo chown nyxora:nyxora "$CONFIG_DIR/config.json"
fi

echo "Step 8: Setting permissions..."
# Set proper permissions
sudo chown -R nyxora:nyxora "$INSTALL_DIR"
sudo chown -R nyxora:nyxora "$CONFIG_DIR"

echo "Step 9: Reloading systemd..."
# Reload systemd
sudo systemctl daemon-reload

echo "Step 10: Enabling and starting Nyxora service..."
# Enable and start the service
sudo systemctl enable nyxora-node
sudo systemctl start nyxora-node

# Verify the service is running
if sudo systemctl is-active --quiet nyxora-node; then
    echo "✓ Nyxora node service is now running!"
    echo "Status:"
    sudo systemctl status nyxora-node --no-pager -l
else
    echo "⚠ Warning: Nyxora service failed to start"
    echo "Check logs with: sudo journalctl -u nyxora-node -f"
    exit 1
fi

echo ""
echo "Setup completed successfully!"
echo ""
echo "To manage the service:"
echo "  Start: sudo systemctl start nyxora-node"
echo "  Stop: sudo systemctl stop nyxora-node"
echo "  Restart: sudo systemctl restart nyxora-node"
echo "  Status: sudo systemctl status nyxora-node"
echo "  Logs: sudo journalctl -u nyxora-node -f"
echo ""
echo "Binaries installed to: $INSTALL_DIR"
echo ""
echo "Next, run the wallet setup script to create your wallet in ~/.nyxora/"