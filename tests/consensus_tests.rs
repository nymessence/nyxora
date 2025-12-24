// tests/consensus_tests.rs
#[cfg(test)]
mod pos_tests {
    use nyxora_node::chain::consensus::pos::{PoSConsensus, Validator};

    #[test]
    fn test_validator_registration() {
        let mut pos = PoSConsensus::new();
        let address = "Qvalidator123".to_string();
        let stake = 1000;
        
        pos.register_validator(address.clone(), stake);
        
        assert!(pos.validators.contains_key(&address));
        let validator = pos.validators.get(&address).unwrap();
        assert_eq!(validator.stake, stake);
        assert_eq!(pos.total_stake, stake);
    }

    #[test]
    fn test_staking() {
        let mut pos = PoSConsensus::new();
        let address = "Qvalidator123".to_string();
        let initial_stake = 1000;
        let additional_stake = 500;
        
        pos.register_validator(address.clone(), initial_stake);
        let success = pos.stake(&address, additional_stake);
        
        assert!(success);
        let validator = pos.validators.get(&address).unwrap();
        assert_eq!(validator.stake, initial_stake + additional_stake);
        assert_eq!(pos.total_stake, initial_stake + additional_stake);
    }

    #[test]
    fn test_unstaking() {
        let mut pos = PoSConsensus::new();
        let address = "Qvalidator123".to_string();
        let initial_stake = 1000;
        let unstake_amount = 300;
        
        pos.register_validator(address.clone(), initial_stake);
        let success = pos.unstake(&address, unstake_amount);
        
        assert!(success);
        let validator = pos.validators.get(&address).unwrap();
        assert_eq!(validator.stake, initial_stake - unstake_amount);
        assert_eq!(pos.total_stake, initial_stake - unstake_amount);
    }

    #[test]
    fn test_unstaking_insufficient_funds() {
        let mut pos = PoSConsensus::new();
        let address = "Qvalidator123".to_string();
        let initial_stake = 100;
        let unstake_amount = 300;
        
        pos.register_validator(address.clone(), initial_stake);
        let success = pos.unstake(&address, unstake_amount);
        
        assert!(!success);
        let validator = pos.validators.get(&address).unwrap();
        assert_eq!(validator.stake, initial_stake);
        assert_eq!(pos.total_stake, initial_stake);
    }
}

#[cfg(test)]
mod poq_tests {
    use nyxora_node::chain::consensus::poq::{PoQConsensus, QuantumProof};

    #[test]
    fn test_challenge_generation() {
        let mut poq = PoQConsensus::new();
        let qubit_count = 10;
        
        let challenge = poq.generate_challenge(qubit_count);
        
        assert_eq!(challenge.qubit_count, qubit_count);
        assert!(challenge.reward > 0);
        assert!(poq.challenges.contains_key(&challenge.challenge_id));
    }

    #[test]
    fn test_proof_submission() {
        let mut poq = PoQConsensus::new();
        let qubit_count = 10;
        
        // Generate a challenge
        let challenge = poq.generate_challenge(qubit_count);
        let challenge_id = challenge.challenge_id.clone();
        
        // Create a valid proof
        let proof = QuantumProof {
            circuit_descriptor: challenge_id,
            measurement_results: vec![0, 1, 1, 0],
            proof_artifact: "valid_proof_hash".to_string(),
            qubit_count,
            validator_address: "Qvalidator123".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let result = poq.submit_proof(proof);
        
        assert!(result.is_ok());
        assert!(!poq.challenges.contains_key(&challenge_id));
    }

    #[test]
    fn test_invalid_proof_submission() {
        let mut poq = PoQConsensus::new();
        let qubit_count = 10;
        
        // Generate a challenge
        let challenge = poq.generate_challenge(qubit_count);
        let challenge_id = challenge.challenge_id.clone();
        
        // Create an invalid proof (empty proof artifact)
        let proof = QuantumProof {
            circuit_descriptor: challenge_id,
            measurement_results: vec![0, 1, 1, 0],
            proof_artifact: "".to_string(),  // Invalid: empty proof artifact
            qubit_count,
            validator_address: "Qvalidator123".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let result = poq.submit_proof(proof);
        
        assert!(result.is_err());
        assert!(poq.challenges.contains_key(&challenge_id));
    }
}

#[cfg(test)]
mod hybrid_consensus_tests {
    use nyxora_node::chain::consensus::{HybridConsensus, pos::Block};

    #[test]
    fn test_hybrid_block_proposal() {
        let mut hybrid = HybridConsensus::new();
        let address = "Qvalidator123".to_string();
        let stake = 1000;
        
        hybrid.register_validator(address.clone(), stake);
        
        let transactions = vec!["tx1".to_string(), "tx2".to_string()];
        let result = hybrid.propose_hybrid_block(&address, transactions);
        
        assert!(result.is_some());
        let block = result.unwrap();
        assert_eq!(block.pos_block.proposer, address);
        assert_eq!(block.pos_block.transactions.len(), 2);
    }

    #[test]
    fn test_validator_scoring() {
        let mut hybrid = HybridConsensus::new();
        let address = "Qvalidator123".to_string();
        let stake = 1000;
        
        hybrid.register_validator(address.clone(), stake);
        
        // Submit a quantum proof to increase the validator's score
        let qubit_count = 10;
        let challenge = hybrid.generate_quantum_challenge(qubit_count);
        let challenge_id = challenge.challenge_id.clone();
        
        let proof = nyxora_node::chain::consensus::poq::QuantumProof {
            circuit_descriptor: challenge_id,
            measurement_results: vec![0, 1, 1, 0],
            proof_artifact: "valid_proof_hash".to_string(),
            qubit_count,
            validator_address: address.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let result = hybrid.submit_quantum_proof(proof);
        assert!(result.is_ok());
        
        // Check that the validator's score increased
        let score = hybrid.poq.get_validator_score(&address);
        assert!(score > 0);
    }
}