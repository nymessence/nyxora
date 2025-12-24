# Nyxora Wallet Guide

This guide explains how to use the Nyxora CLI wallet to manage your NYX tokens, participate in staking, and interact with the quantum-hybrid blockchain.

## Overview

The Nyxora CLI wallet provides secure access to your NYX tokens with quantum-enhanced security features. All wallet addresses start with 'Q' and use advanced cryptographic techniques for transaction signing.

## Installation

### Prerequisites

- Rust 1.80+ installed
- Completed installation steps in install.md

### Building the Wallet

```bash
cd nyxora
cargo build --release
```

The wallet binary will be available at `target/release/nyxora-wallet`.

## Wallet Commands

### Generate a New Wallet

```bash
./target/release/nyxora-wallet generate
```

This creates a new wallet file (`wallet.json`) with a unique address starting with 'Q'.

### Check Your Address

```bash
./target/release/nyxora-wallet address
```

### Check Your Balance

```bash
./target/release/nyxora-wallet balance
```

### Send Tokens

```bash
./target/release/nyxora-wallet send <recipient_address> <amount>
```

Example:
```bash
./target/release/nyxora-wallet send Qrecipient123456789012345678901234567890 100.0
```

### Stake Tokens

```bash
./target/release/nyxora-wallet stake <amount>
```

Example:
```bash
./target/release/nyxora-wallet stake 1000.0
```

### Sign a Message

```bash
./target/release/nyxora-wallet sign "message to sign"
```

## Wallet File Management

By default, the wallet uses `wallet.json` in the current directory. You can specify a different file:

```bash
./target/release/nyxora-wallet balance --file mywallet.json
```

## Security Best Practices

- Store your wallet file securely with appropriate file permissions
- Backup your wallet file regularly
- Never share your wallet file with others
- Use strong system security to protect your wallet

## Wallet File Format

The wallet file is a JSON document containing:

```json
{
  "private_key": "hex_encoded_private_key",
  "public_key": "hex_encoded_public_key",
  "address": "Qaddress_string"
}
```

## Troubleshooting

- If you get "Wallet file does not exist" error, generate a wallet first
- Ensure proper file permissions on wallet files
- Verify you're in the correct directory when running wallet commands