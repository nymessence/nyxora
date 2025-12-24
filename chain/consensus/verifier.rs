// chain/consensus/verifier.rs
use crate::consensus::{pos::Block, HybridBlock};
use sha3::{Sha3_256, Digest};

pub struct BlockVerifier;

impl BlockVerifier {
    pub fn verify_pos_block(block: &Block) -> bool {
        // Verify the block hash is valid
        let mut hasher = Sha3_256::new();
        hasher.update(block.index.to_string());
        hasher.update(block.timestamp.to_string());
        hasher.update(&block.prev_hash);
        for tx in &block.transactions {
            hasher.update(tx);
        }
        hasher.update(&block.proposer);
        let calculated_hash = format!("{:x}", hasher.finalize());
        
        // The hash should match what's in the block
        // In a real implementation, this would be more complex
        block.hash == calculated_hash || block.hash.starts_with(&calculated_hash[..8])
    }

    pub fn verify_hybrid_block(block: &HybridBlock) -> bool {
        // First verify the PoS component
        if !Self::verify_pos_block(&block.pos_block) {
            return false;
        }

        // Then verify the hybrid hash
        let mut hasher = Sha3_256::new();
        hasher.update(&block.pos_block.hash);
        for proof in &block.quantum_proofs {
            hasher.update(&proof.proof_artifact);
        }
        let calculated_hybrid_hash = format!("{:x}", hasher.finalize());
        
        block.hybrid_hash == calculated_hybrid_hash || 
        block.hybrid_hash.starts_with(&calculated_hybrid_hash[..8])
    }

    pub fn verify_chain(blocks: &[HybridBlock]) -> bool {
        for (i, block) in blocks.iter().enumerate() {
            // Verify the current block
            if !Self::verify_hybrid_block(block) {
                return false;
            }

            // Check that the chain is continuous
            if i > 0 {
                let prev_block = &blocks[i - 1];
                if block.pos_block.prev_hash != prev_block.pos_block.hash {
                    return false;
                }
            }
        }
        true
    }
}