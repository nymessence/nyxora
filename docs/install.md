# Nyxora Installation Guide

Welcome to the Nyxora quantum-hybrid cryptocurrency. This guide will walk you through installing and setting up a Nyxora node on your system.

## Prerequisites

- Rust 1.80+ (for building the node and wallet)
- Python 3.8+ (for quantum simulations)
- Git
- Docker (optional, for containerized deployment)

## Installation Steps

### 1. Clone the Repository

```bash
git clone https://github.com/nyxora/nyxora.git
cd nyxora
```

### 2. Install Rust Dependencies

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build the wallet and node
cd nyxora
cargo build --release
```

### 3. Install Python Dependencies for Quantum Layer

```bash
pip install qiskit numpy
```

### 4. Build Using Docker (Optional)

```bash
cd docker
docker build -t nyxora-node .
```

Or using docker-compose:

```bash
docker-compose up -d
```

## Running a Local Testnet

### 1. Start the Validator Node

```bash
cargo run --bin nyxora-node -- --validator --port 8080
```

### 2. Generate a Wallet

```bash
cargo run --bin nyxora-wallet -- generate
```

### 3. Check Your Balance

```bash
cargo run --bin nyxora-wallet -- balance
```

## Systemd Service Installation (Linux)

To run Nyxora as a system service:

1. Copy the service file:
   ```bash
   sudo cp services/nyxora-node.service /etc/systemd/system/
   ```

2. Reload systemd:
   ```bash
   sudo systemctl daemon-reload
   ```

3. Enable and start the service:
   ```bash
   sudo systemctl enable nyxora-node
   sudo systemctl start nyxora-node
   ```

## Configuration

The node can be configured using a `config.json` file. Example:

```json
{
  "address": "Q123456789012345678901234567890123456789",
  "stake_amount": 10000,
  "is_validator": true,
  "quantum_enabled": true,
  "peers": ["127.0.0.1:8081", "127.0.0.1:8082"]
}
```

## Troubleshooting

- If you encounter build errors, ensure you have the latest Rust toolchain installed
- For quantum simulation issues, verify Qiskit is properly installed
- Check that ports 8080+ are available for the node to use