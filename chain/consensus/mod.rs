// chain/consensus/mod.rs
pub mod pos;
pub mod poq;
pub mod verifier;

use serde::{Deserialize, Serialize};
use pos::{PoSConsensus, Block};
use poq::{PoQConsensus, QuantumProof};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridBlock {
    pub pos_block: Block,
    pub quantum_proofs: Vec<QuantumProof>,
    pub hybrid_hash: String,
}

pub struct HybridConsensus {
    pub pos: PoSConsensus,
    pub poq: PoQConsensus,
}

impl HybridConsensus {
    pub fn new() -> Self {
        HybridConsensus {
            pos: PoSConsensus::new(),
            poq: PoQConsensus::new(),
        }
    }

    pub fn register_validator(&mut self, address: String, initial_stake: u64) {
        self.pos.register_validator(address, initial_stake);
    }

    pub fn generate_quantum_challenge(&mut self, qubit_count: usize) -> poq::PoQChallenge {
        self.poq.generate_challenge(qubit_count)
    }

    pub fn submit_quantum_proof(&mut self, proof: QuantumProof) -> Result<(), String> {
        self.poq.submit_proof(proof)
    }

    pub fn propose_hybrid_block(&mut self, proposer: &str, transactions: Vec<String>) -> Option<HybridBlock> {
        // First propose a PoS block
        if let Some(pos_block) = self.pos.propose_block(proposer, transactions) {
            // Get recent quantum proofs to include in the hybrid block
            let recent_proofs = self.get_recent_quantum_proofs();
            
            // Calculate a hybrid hash that includes both PoS and PoQ elements
            let mut hasher = sha3::Sha3_256::new();
            hasher.update(&pos_block.hash);
            for proof in &recent_proofs {
                hasher.update(&proof.proof_artifact);
            }
            let hybrid_hash = format!("{:x}", hasher.finalize());

            let hybrid_block = HybridBlock {
                pos_block,
                quantum_proofs: recent_proofs,
                hybrid_hash,
            };

            // Calculate rewards for both PoS and PoQ contributions
            self.pos.calculate_rewards(&hybrid_block.pos_block);

            Some(hybrid_block)
        } else {
            None
        }
    }

    fn get_recent_quantum_proofs(&self) -> Vec<QuantumProof> {
        // In a real implementation, this would return proofs from a recent time window
        // For now, we'll return the last 5 proofs
        self.poq.proofs.iter()
            .rev()
            .take(5)
            .cloned()
            .collect()
    }
}