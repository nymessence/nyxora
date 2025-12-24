// chain/consensus/pos.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha3::{Sha3_256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub last_block_proposed: u64,
    pub uptime: f64, // Percentage of blocks validated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeTransaction {
    pub from: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: String,
    pub transactions: Vec<String>, // Simplified - in real implementation these would be transaction structs
    pub proposer: String,
    pub hash: String,
}

pub struct PoSConsensus {
    pub validators: HashMap<String, Validator>,
    pub total_stake: u64,
    pub current_block: u64,
}

impl PoSConsensus {
    pub fn new() -> Self {
        PoSConsensus {
            validators: HashMap::new(),
            total_stake: 0,
            current_block: 0,
        }
    }

    pub fn register_validator(&mut self, address: String, initial_stake: u64) {
        self.validators.insert(
            address.clone(),
            Validator {
                address,
                stake: initial_stake,
                last_block_proposed: 0,
                uptime: 100.0,
            }
        );
        self.total_stake += initial_stake;
    }

    pub fn stake(&mut self, address: &str, amount: u64) -> bool {
        if let Some(validator) = self.validators.get_mut(address) {
            validator.stake += amount;
            self.total_stake += amount;
            true
        } else {
            false
        }
    }

    pub fn unstake(&mut self, address: &str, amount: u64) -> bool {
        if let Some(validator) = self.validators.get_mut(address) {
            if validator.stake >= amount {
                validator.stake -= amount;
                self.total_stake -= amount;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn select_proposer(&self) -> Option<String> {
        if self.validators.is_empty() {
            return None;
        }

        // Simple weighted random selection based on stake
        let mut rng = rand::thread_rng();
        let total_stake = self.total_stake as f64;
        
        if total_stake == 0.0 {
            return None;
        }
        
        let random_value = (rand::random::<f64>() * total_stake) as u64;
        
        let mut cumulative_stake = 0;
        for (address, validator) in &self.validators {
            cumulative_stake += validator.stake;
            if cumulative_stake >= random_value {
                return Some(address.clone());
            }
        }
        
        // Fallback to first validator
        self.validators.keys().next().cloned()
    }

    pub fn propose_block(&mut self, proposer: &str, transactions: Vec<String>) -> Option<Block> {
        if !self.validators.contains_key(proposer) {
            return None;
        }

        let index = self.current_block + 1;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create a simple hash of the block data
        let mut hasher = Sha3_256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        if let Some(prev_validator) = self.validators.values().next() {
            hasher.update(&prev_validator.address);
        }
        for tx in &transactions {
            hasher.update(tx);
        }
        let hash = format!("{:x}", hasher.finalize());

        let block = Block {
            index,
            timestamp,
            prev_hash: if self.current_block == 0 { "0".to_string() } else { 
                // In a real implementation, we would have the previous block hash
                format!("prev_hash_{}", self.current_block) 
            },
            transactions,
            proposer: proposer.to_string(),
            hash,
        };

        // Update validator's last proposed block
        if let Some(validator) = self.validators.get_mut(proposer) {
            validator.last_block_proposed = index;
        }

        self.current_block = index;
        Some(block)
    }

    pub fn calculate_rewards(&mut self, block: &Block) {
        // Simple reward calculation
        if let Some(validator) = self.validators.get_mut(&block.proposer) {
            // Add reward for proposing a block
            validator.stake += 10; // Fixed reward for simplicity
            self.total_stake += 10;
        }
    }
}