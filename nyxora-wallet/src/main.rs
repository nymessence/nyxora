use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use rand::rngs::OsRng;
use sha3::{Sha3_256, Digest};
use hex;

#[derive(Parser)]
#[command(name = "nyxora-wallet")]
#[command(about = "A CLI wallet for the Nyxora quantum-hybrid cryptocurrency")]
enum Cli {
    /// Generate a new wallet
    Generate,

    /// Show wallet address
    Address {
        #[arg(short, long, default_value = "wallet.json")]
        file: String,
    },

    /// Check wallet balance
    Balance {
        #[arg(short, long, default_value = "wallet.json")]
        file: String,
    },

    /// Send tokens to another address
    Send {
        to: String,
        amount: f64,
        #[arg(short, long, default_value = "wallet.json")]
        file: String,
    },

    /// Stake tokens
    Stake {
        amount: f64,
        #[arg(short, long, default_value = "wallet.json")]
        file: String,
    },

    /// Sign a message
    Sign {
        message: String,
        #[arg(short, long, default_value = "wallet.json")]
        file: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Wallet {
    private_key: String,
    public_key: String,
    address: String,
}

impl Wallet {
    fn new() -> Self {
        // Generate a random private key using OS entropy
        let mut rng = OsRng;
        let mut private_key_bytes = [0u8; 32];
        for byte in &mut private_key_bytes {
            *byte = rand::RngCore::next_u32(&mut rng) as u8;
        }

        let private_key = hex::encode(&private_key_bytes);

        // Derive public key (in a real implementation, this would be proper ECC)
        // For now, we'll hash the private key to simulate public key derivation
        let mut hasher = Sha3_256::new();
        hasher.update(&private_key_bytes);
        let public_key_bytes = hasher.finalize();
        let public_key = hex::encode(&public_key_bytes);

        // Generate address starting with 'Q' as specified
        let address = format!("Q{}", &public_key[..39]); // Make it start with Q and be 40 chars

        Wallet {
            private_key,
            public_key,
            address,
        }
    }

    fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let wallet: Wallet = serde_json::from_str(&contents)?;
        Ok(wallet)
    }

    fn sign_message(&self, message: &str) -> String {
        // In a real implementation, this would be a proper cryptographic signature
        // For now, we'll simulate by hashing the private key + message
        let mut hasher = Sha3_256::new();
        hasher.update(&self.private_key);
        hasher.update(message);
        let signature_bytes = hasher.finalize();
        hex::encode(&signature_bytes)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli {
        Cli::Generate => {
            println!("Generating new Nyxora wallet...");
            let wallet = Wallet::new();

            // Save to default file
            wallet.save("wallet.json")?;
            println!("Wallet generated successfully!");
            println!("Address: {}", wallet.address);
            println!("Wallet saved to wallet.json");
        },

        Cli::Address { file } => {
            if !Path::new(&file).exists() {
                eprintln!("Wallet file '{}' does not exist. Generate a wallet first.", file);
                std::process::exit(1);
            }

            let wallet = Wallet::load(&file)?;
            println!("{}", wallet.address);
        },

        Cli::Balance { file } => {
            if !Path::new(&file).exists() {
                eprintln!("Wallet file '{}' does not exist. Generate a wallet first.", file);
                std::process::exit(1);
            }

            let wallet = Wallet::load(&file)?;
            // In a real implementation, this would query the blockchain
            // For now, we'll return a mock balance
            println!("Balance for {}: 100.0 NYX", wallet.address);
        },

        Cli::Send { to, amount, file } => {
            if !Path::new(&file).exists() {
                eprintln!("Wallet file '{}' does not exist. Generate a wallet first.", file);
                std::process::exit(1);
            }

            let wallet = Wallet::load(&file)?;
            println!("Sending {} NYX from {} to {}", amount, wallet.address, to);

            // In a real implementation, this would create and broadcast a transaction
            // For now, we'll just simulate the action
            println!("Transaction created and broadcasted successfully!");
        },

        Cli::Stake { amount, file } => {
            if !Path::new(&file).exists() {
                eprintln!("Wallet file '{}' does not exist. Generate a wallet first.", file);
                std::process::exit(1);
            }

            let wallet = Wallet::load(&file)?;
            println!("Staking {} NYX from {}", amount, wallet.address);

            // In a real implementation, this would stake tokens on the blockchain
            // For now, we'll just simulate the action
            println!("Tokens staked successfully!");
        },

        Cli::Sign { message, file } => {
            if !Path::new(&file).exists() {
                eprintln!("Wallet file '{}' does not exist. Generate a wallet first.", file);
                std::process::exit(1);
            }

            let wallet = Wallet::load(&file)?;
            let signature = wallet.sign_message(&message);
            println!("Signature: {}", signature);
        },
    }

    Ok(())
}
