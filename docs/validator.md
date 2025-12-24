# Nyxora Validator Guide

This guide explains how to set up and operate a Nyxora validator node, participate in the hybrid PoS/PoQ consensus, and earn rewards.

## Overview

Nyxora uses a hybrid consensus mechanism combining:
- **Proof-of-Stake (PoS)**: Validators stake NYX tokens to participate in block validation
- **Proof-of-Quantum (PoQ)**: Validators solve quantum computational challenges to earn additional rewards

## Setting Up a Validator

### 1. System Requirements

- 4+ CPU cores
- 8GB+ RAM
- 100GB+ storage
- Stable internet connection
- Optional: Quantum computing access (simulator works for now)

### 2. Stake Requirements

- Minimum stake: 10,000 NYX
- Recommended: 100,000+ NYX for competitive rewards

### 3. Node Configuration

Create a `config.json` file:

```json
{
  "address": "Q<your_wallet_address>",
  "stake_amount": 100000,
  "is_validator": true,
  "quantum_enabled": true,
  "peers": ["mainnet-peer1.nyxora.io:8080", "mainnet-peer2.nyxora.io:8080"]
}
```

### 4. Start the Validator Node

```bash
cargo run --bin nyxora-node --validator --config config.json --port 8080
```

## Participating in Consensus

### Proof-of-Stake (PoS)

- Your chance of being selected to propose a block is proportional to your stake
- Maintain your node's uptime to earn consistent rewards
- Slashing penalties apply for malicious behavior

### Proof-of-Quantum (PoQ)

- The network periodically issues quantum challenges
- Challenges require generating quantum proofs using hexagonal Hadamard-CNOT circuits
- Difficulty scales linearly with qubit count
- Submit quantum proofs to earn additional rewards

## Quantum Challenges

### Solving Challenges

When a quantum challenge is issued:

1. Generate a quantum proof using the hexagonal circuit generator
2. Submit the proof to the network
3. Earn rewards based on difficulty level

Example quantum proof generation:

```python
from quantum.circuits.hex_hadamard_cnot import generate_quantum_proof

# Generate a proof with 20 qubits
proof = generate_quantum_proof(qubit_count=20, difficulty_level=2)
```

### Quantum Hardware Integration

For advanced users with quantum hardware access:
- Implement the quantum circuit generation on actual quantum computers
- Submit proofs generated from real quantum computations
- Potentially earn higher rewards for quantum-verified computations

## Rewards and Staking

### PoS Rewards

- Base reward for block validation
- Proportional to your stake weight in the network
- Distributed with each validated block

### PoQ Rewards

- Bonus rewards for quantum proof submission
- Higher rewards for higher difficulty challenges
- Rewards distributed when proofs are verified and accepted

### Unstaking

- Minimum lockup period: 21 days
- Submit unstake transaction via wallet
- Funds become available after lockup period

## Monitoring Your Validator

The validator node exposes a status endpoint:

```bash
curl http://localhost:8080/status
```

Expected response:
```json
{
  "config": {
    "address": "Q...",
    "stake_amount": 100000,
    "is_validator": true
  },
  "block_height": 12345,
  "peers": 15,
  "status": "running"
}
```

## Troubleshooting

- Ensure your wallet has sufficient balance for transaction fees
- Verify your node is properly connected to the network
- Monitor system resources during quantum computations
- Keep your validator node updated with the latest releases