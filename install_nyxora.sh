#!/bin/bash

# Nyxora Installation Script
# This script installs the Nyxora quantum-hybrid cryptocurrency node as a system service
# Run this script from within the nyxora repository directory after cloning

set -e  # Exit on any error

echo "Nyxora Quantum-Hybrid Cryptocurrency - Installation Script"
echo "=========================================================="

# Check if running as root (should not run as root directly)
if [[ $EUID -eq 0 ]]; then
   echo "This script should NOT be run as root directly."
   echo "Run as a regular user, and the script will use sudo for system operations."
   echo "For example: ./install_nyxora.sh"
   exit 1
fi

# Change to the directory where the script is located
cd "$(dirname "$0")"

# Check for required tools
command -v rustc >/dev/null 2>&1 || { echo "Rust is required but not installed. Please install Rust first. Aborting."; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "Cargo is required but not installed. Please install Cargo first. Aborting."; exit 1; }

# Check if Python is available
command -v python3 >/dev/null 2>&1 || { echo "Python3 is required but not installed. Aborting."; exit 1; }

# Define installation directory
INSTALL_DIR="/opt/nyxora"
SVC_FILE="/etc/systemd/system/nyxora-node.service"
CONFIG_DIR="/etc/nyxora"

echo "Step 1: Building Nyxora binaries..."
# Build the binaries from the current repository
# The binaries are already in the current directory structure
cargo build --release

echo "Step 2: Installing Python dependencies for quantum layer..."
# Install Python dependencies - using --break-system-packages for non-package-manager installations
pip3 install --break-system-packages qiskit numpy || {
    echo "Failed to install Python dependencies with --break-system-packages"
    echo "Trying to install with user flag..."
    pip3 install --user qiskit numpy || {
        echo "Also failed to install with --user flag"
        echo "Please install qiskit and numpy manually before running this script"
        exit 1
    }
}

echo "Step 3: Creating nyxora system user..."
# Create user for nyxora if it doesn't exist (requires sudo)
if ! sudo -n id "nyxora" &>/dev/null; then
    echo "Creating nyxora user..."
    sudo useradd -r -s /bin/false nyxora
fi

echo "Step 4: Creating directories..."
# Create directories (requires sudo)
sudo mkdir -p "$INSTALL_DIR"
sudo mkdir -p "$CONFIG_DIR"
sudo mkdir -p "$INSTALL_DIR/data"
sudo mkdir -p "$INSTALL_DIR/logs"

echo "Step 5: Installing binaries..."
# Copy binaries to installation directory (requires sudo)
sudo cp target/release/nyxora-node "$INSTALL_DIR/"
sudo cp target/release/nyxora-wallet "$INSTALL_DIR/"

# Make binaries executable (requires sudo)
sudo chmod +x "$INSTALL_DIR/nyxora-node"
sudo chmod +x "$INSTALL_DIR/nyxora-wallet"

echo "Step 6: Installing service file..."
# Copy service file (requires sudo)
sudo cp services/nyxora-node.service "$SVC_FILE"

# Create default configuration (requires sudo)
if [ ! -f "$CONFIG_DIR/config.json" ]; then
    echo "Creating default configuration..."
    sudo tee "$CONFIG_DIR/config.json" > /dev/null <<EOF
{
  "address": "",
  "stake_amount": 10000,
  "is_validator": false,
  "quantum_enabled": true,
  "peers": ["mainnet-peer1.nyxora.io:33333", "mainnet-peer2.nyxora.io:33333"]
}
EOF
    sudo chown nyxora:nyxora "$CONFIG_DIR/config.json"
fi

echo "Step 7: Setting permissions..."
# Set proper permissions (requires sudo)
sudo chown -R nyxora:nyxora "$INSTALL_DIR"
sudo chown -R nyxora:nyxora "$CONFIG_DIR"

echo "Step 8: Reloading systemd..."
# Reload systemd (requires sudo)
sudo systemctl daemon-reload

echo "Step 9: Enabling and starting Nyxora service..."
# Enable and start the service (requires sudo)
sudo systemctl enable nyxora-node
sudo systemctl start nyxora-node

# Verify the service is running (requires sudo)
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
echo "Installation completed successfully!"
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