# Nyxora: Quantum-Hybrid Cryptocurrency

Nyxora is a pioneering quantum-hybrid cryptocurrency that combines traditional Proof-of-Stake (PoS) consensus with innovative Proof-of-Quantum (PoQ) mechanisms. This unique architecture leverages quantum computational advantages to enhance blockchain security, randomness, and scalability.

## Features

- **Hybrid Consensus**: Combines Proof-of-Stake (PoS) and Proof-of-Quantum (PoQ) mechanisms
- **Quantum Circuits**: Hexagonal quantum circuits with alternating Hadamard and CNOT gates
- **CLI Wallet**: Secure wallet with quantum-enhanced security
- **Validator Node**: Full node implementation with staking capabilities
- **Smart Contracts**: Quantum-enhanced contracts with randomness and NFT functionality
- **Installable Service**: Systemd service for production deployment

## Architecture

### Core Components

1. **Blockchain Core**: Hybrid PoS/PoQ consensus engine
2. **Quantum Layer**: Hexagonal quantum circuit generator and verifier
3. **Wallet**: CLI interface for managing NYX tokens
4. **Validator Node**: Full node with staking and quantum proof capabilities
5. **Smart Contracts**: Quantum-enhanced contract execution environment

### Quantum Design

The quantum layer uses hexagonal quantum circuits with:
- Primary qubits arranged in hexagonal patterns
- Alternating layers of Hadamard gates for superposition
- CNOT gates connecting adjacent qubits in the hexagonal lattice
- Measurement operations to generate quantum proof artifacts

## Installation

### Prerequisites

- Rust 1.80+ (for building the node and wallet)
- Python 3.8+ (for quantum simulations)
- Git
- Docker (optional, for containerized deployment)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/nyxora/nyxora.git
cd nyxora

# Build the project
cd nyxora
cargo build --release

# Install Python dependencies for quantum layer
pip install qiskit numpy

# Generate a wallet
./target/release/nyxora-wallet generate

# Start a validator node
./target/release/nyxora-node --validator --port 8080
```

### Docker Deployment

```bash
# Build the Docker image
cd docker
docker build -t nyxora-node .

# Or use docker-compose
docker-compose up -d
```

## Usage

### Wallet Commands

```bash
# Generate a new wallet
./target/release/nyxora-wallet generate

# Check your address
./target/release/nyxora-wallet address

# Check your balance
./target/release/nyxora-wallet balance

# Send tokens
./target/release/nyxora-wallet send <recipient_address> <amount>

# Stake tokens
./target/release/nyxora-wallet stake <amount>

# Sign a message
./target/release/nyxora-wallet sign "message to sign"
```

### Node Commands

```bash
# Start as a validator node
./target/release/nyxora-node --validator --port 8080

# Start as a regular node
./target/release/nyxora-node --port 8080

# Use custom configuration
./target/release/nyxora-node --config config.json --port 8080
```

## Tokenomics

- Maximum supply: 1 billion NYX tokens
- Emission period: 500 years
- Halving schedule: Every 25 years
- Hybrid reward system: PoS and PoQ rewards

## Quantum Proof System

The Proof-of-Quantum (PoQ) system:
- Accepts quantum proof submissions from validators
- Implements linear difficulty scaling with qubit count
- Supports both real quantum hardware and simulator proofs
- Provides quantum-enhanced randomness to the network

## Smart Contracts

Nyxora supports quantum-enhanced smart contracts including:
- Quantum Randomness Contract: Requests quantum circuit execution and verifies proofs
- Quantum NFT Contract: Mints NFTs with embedded quantum proof hashes

## Documentation

- [Installation Guide](docs/install.md)
- [Validator Guide](docs/validator.md)
- [Wallet Guide](docs/wallet.md)
- [Smart Contracts Guide](docs/contracts.md)
- [Architecture Overview](docs/architecture.md)
- [Whitepaper](whitepaper/nyxora_whitepaper.md)

## License

This project is licensed under the MIT License - see the LICENSE file for details.